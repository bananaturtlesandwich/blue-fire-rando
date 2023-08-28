use super::logic::*;
use super::Mod;
use crate::{io::*, map::*};
use unreal_asset::{
    cast,
    containers::{NameMap, SharedResource},
    exports::*,
    properties::*,
    unversioned::Ancestry,
};

mod cutscenes;
mod overworld;
mod savegames;
mod specific;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unreal_asset: {0}")]
    UnrealAsset(#[from] unreal_asset::Error),
    #[error("repak: {0}")]
    Repak(#[from] repak::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("locked poisoned name vec")]
    VecPoison,
    #[error("locked poisoned writer")]
    WriterPoison,
    #[error("extracted poisoned writer")]
    InnerMutex(#[from] std::sync::PoisonError<repak::PakWriter<std::io::BufWriter<std::fs::File>>>),
    #[error("some threads are still using writer")]
    InnerArc,
    #[error("failed to strip prefix when writing file to pak")]
    Strip(#[from] std::path::StripPrefixError),
    #[error("thread failed to complete")]
    Thread,
    #[error("data was not as expected - you may have an older version of the game")]
    Assumption,
}

macro_rules! stub {
    ($type: ty, $variant: ident) => {
        impl From<$type> for Error {
            fn from(_: $type) -> Self {
                Self::$variant
            }
        }
    };
}

stub!(
    std::sync::Arc<std::sync::Mutex<repak::PakWriter<std::io::BufWriter<std::fs::File>>>>,
    InnerArc
);
stub!(
    std::sync::PoisonError<
        std::sync::MutexGuard<'_, repak::PakWriter<std::io::BufWriter<std::fs::File>>>,
    >,
    WriterPoison
);
stub!(
    std::sync::PoisonError<std::sync::MutexGuard<'_, Vec<String>>>,
    VecPoison
);
stub!(Box<dyn std::any::Any + Send + 'static>, Thread);

pub const MOD: &str = "PROA34/Content/BlueFire/";

const SAVEGAME: &str = "Player/Logic/FrameWork/BlueFireSaveGame.uasset";

const PREFIX: &str = "Maps/World/";

fn extract(
    app: &crate::Rando,
    pak: &repak::PakReader,
    path: &str,
) -> Result<super::Asset<Vec<u8>>, Error> {
    open(
        pak.get(
            &format!("Blue Fire/Content/BlueFire/{path}"),
            &mut app.pak()?,
        )?,
        pak.get(
            &format!(
                "Blue Fire/Content/BlueFire/{}",
                path.replace(".uasset", ".uexp").replace(".umap", ".uexp")
            ),
            &mut app.pak()?,
        )?,
    )
}

fn byte_property(
    name: &str,
    enum_ty: &str,
    val: &str,
    name_map: &mut SharedResource<NameMap>,
) -> Property {
    let name = name_map.get_mut().add_fname(name);
    let enum_type = Some(name_map.get_mut().add_fname(enum_ty));
    Property::ByteProperty(int_property::ByteProperty {
        name,
        ancestry: Ancestry {
            ancestry: Vec::new(),
        },
        property_guid: None,
        duplication_index: 0,
        enum_type,
        value: int_property::BytePropertyValue::FName(
            name_map
                .get_mut()
                .add_fname(&format!("{}::NewEnumerator{}", enum_ty, val)),
        ),
    })
}

fn set_byte(
    name: &str,
    enum_type: &str,
    val: &str,
    export: &mut normal_export::NormalExport,
    name_map: &mut SharedResource<NameMap>,
) -> Result<(), Error> {
    match export
        .properties
        .iter_mut()
        .find_map(|prop| cast!(Property, ByteProperty, prop).filter(|byte| byte.name == name))
    {
        Some(byte) => {
            use int_property::BytePropertyValue;
            *cast!(BytePropertyValue, FName, &mut byte.value).ok_or(Error::Assumption)? = name_map
                .get_mut()
                .add_fname(&format!("{}::NewEnumerator{}", enum_type, val))
        }
        None => export
            .properties
            .push(byte_property(name, enum_type, val, name_map)),
    }
    Ok(())
}

pub fn write(data: Data, app: &crate::Rando) -> Result<(), Error> {
    let mut sync = app.pak()?;
    let pak = repak::PakReader::new(&mut sync, repak::Version::V9)?;
    let mod_pak = std::sync::Arc::new(std::sync::Mutex::new(repak::PakWriter::new(
        std::io::BufWriter::new(std::fs::File::create(app.pak.join("rando_p.pak"))?),
        repak::Version::V9,
        "../../../".to_string(),
        None,
    )));
    std::thread::scope(|thread| -> Result<(), Error> {
        for thread in [
            thread.spawn(|| overworld::write(data.overworld, app, &pak, &mod_pak)),
            thread.spawn(|| cutscenes::write(data.cutscenes, app, &pak, &mod_pak)),
            thread
                .spawn(|| savegames::write(data.savegames, data.shop_emotes, app, &pak, &mod_pak)),
            thread.spawn(|| specific::write(data.cases, app, &pak, &mod_pak)),
        ] {
            thread.join()??
        }
        Ok(())
    })?;
    let mut mod_pak = Mod::try_unwrap(mod_pak)?.into_inner()?;
    // change the logo so people know it worked
    let logo = MOD.to_string() + "HUD/Menu/Blue-Fire-Logo.uasset";
    mod_pak.write_file(
        &logo,
        &mut std::io::Cursor::new(include_bytes!("blueprints/logo.uasset")),
    )?;
    mod_pak.write_file(
        &logo.replace(".uasset", ".uexp"),
        &mut std::io::Cursor::new(include_bytes!("blueprints/logo.uexp")),
    )?;
    mod_pak.write_index()?;
    Ok(())
}

fn create_hook<C: std::io::Read + std::io::Seek>(
    app: &crate::Rando,
    pak: &repak::PakReader,
    mod_pak: &Mod,
    hook: &mut unreal_asset::Asset<C>,
    drop: &Drop,
    cutscene: &str,
    index: usize,
) -> Result<(), Error> {
    let mut loc = MOD.to_string() + "Libraries";
    let new_name = format!("{}_Hook", cutscene.split('/').last().unwrap_or_default());
    loc = format!("{loc}/{new_name}.uasset");
    // edit the item given by the kismet bytecode in the hook
    let Export::FunctionExport(function_export::FunctionExport {
        struct_export:
            struct_export::StructExport {
                script_bytecode: Some(bytecode),
                ..
            },
        ..
    }) = &mut hook.asset_data.exports[index]
    else {
        return Err(Error::Assumption);
    };
    use unreal_asset::kismet::*;
    let [KismetExpression::ExLet(item_type), KismetExpression::ExLet(index), KismetExpression::ExLet(amount), KismetExpression::ExLetBool(key_item)] =
        &mut bytecode[0..4]
    else {
        return Err(Error::Assumption);
    };
    let [KismetExpression::ExByteConst(item_type), KismetExpression::ExByteConst(index), KismetExpression::ExIntConst(amount)] = [
        item_type.expression.as_mut(),
        index.expression.as_mut(),
        amount.expression.as_mut(),
    ] else {
        return Err(Error::Assumption);
    };
    item_type.value = drop.as_u8();
    index.value = drop.inner_as_u8();
    amount.value = match &drop {
        Drop::Item(_, amount) => *amount,
        Drop::Ore(amount) => *amount,
        _ => 1,
    };
    *key_item.assignment_expression = match &drop {
        Drop::Item(item, ..) if item.key_item() => KismetExpression::ExTrue(ExTrue::default()),
        _ => KismetExpression::ExFalse(ExFalse::default()),
    };
    let self_refs: Vec<usize> = hook
        .get_name_map()
        .get_ref()
        .get_name_map_index_list()
        .iter()
        .enumerate()
        .filter_map(|(i, name)| name.contains("hook").then_some(i))
        .collect();
    for i in self_refs {
        let mut map = hook.get_name_map();
        let mut map = map.get_mut();
        let name = map.get_name_reference_mut(i as i32);
        *name = name.replace("hook", &new_name);
    }
    save(hook, mod_pak, &loc)?;
    let loc = format!("{MOD}{cutscene}.uasset");
    let mut cutscene = open(
        pak.get(
            &format!("Blue Fire/Content/BlueFire/{cutscene}.uasset"),
            &mut app.pak()?,
        )?,
        pak.get(
            &format!("Blue Fire/Content/BlueFire/{cutscene}.uexp"),
            &mut app.pak()?,
        )?,
    )?;
    let universal_refs: Vec<usize> = cutscene
        .get_name_map()
        .get_ref()
        .get_name_map_index_list()
        .iter()
        .enumerate()
        .filter_map(|(i, name)| name.contains("UniversalFunctions").then_some(i))
        .collect();
    for i in universal_refs {
        let mut map = cutscene.get_name_map();
        let mut map = map.get_mut();
        let name = map.get_name_reference_mut(i as i32);
        *name = name.replace("UniversalFunctions", &new_name);
    }
    save(&mut cutscene, mod_pak, &loc)?;
    Ok(())
}

impl Drop {
    pub fn as_shop_entry(
        &self,
        price: i32,
        name_map: &mut SharedResource<NameMap>,
    ) -> Vec<Property> {
        let amount_name = name_map
            .get_mut()
            .add_fname("Amount_6_185C591747EF40A592FB63886FDB4281");
        let resets_name = name_map
            .get_mut()
            .add_fname("Resets_8_E303F5DF4270CCEE83F05F974F3661C9");
        let original_amounts_name = name_map
            .get_mut()
            .add_fname("OriginalAmount_11_58C3C17D426D49A439C0EE85D7E9B6EC");
        let price_name = name_map
            .get_mut()
            .add_fname("Price_26_80A37F3645AE8292A9F311B86094C095");
        use int_property::*;
        [
            byte_property(
                "Item_3_54327288464702F41977D48660F8979E",
                "Items",
                match self {
                    Drop::Item(item, ..) => item.as_ref(),
                    Drop::Ore(..) => Items::KinbankDebitCard.as_ref(),
                    Drop::Duck => Items::Duck.as_ref(),
                    _ => "25",
                },
                name_map,
            ),
            Property::IntProperty(IntProperty {
                name: amount_name,
                ancestry: Ancestry {
                    ancestry: Vec::new(),
                },
                property_guid: None,
                duplication_index: 0,
                value: match self {
                    Drop::Item(_, amount) => *amount,
                    Drop::Emote(..) => 0,
                    _ => 1,
                },
            }),
            Property::BoolProperty(BoolProperty {
                name: resets_name,
                ancestry: Ancestry {
                    ancestry: Vec::new(),
                },
                property_guid: None,
                duplication_index: 0,
                value: false,
            }),
            Property::IntProperty(IntProperty {
                name: original_amounts_name,
                ancestry: Ancestry {
                    ancestry: Vec::new(),
                },
                property_guid: None,
                duplication_index: 0,
                value: match self {
                    Drop::Item(_, amount) => *amount,
                    _ => 1,
                },
            }),
            byte_property(
                "Type_17_9B84CFD04716464F71190CB4CECE0F49",
                "InventoryItemType",
                self.as_ref(),
                name_map,
            ),
            byte_property(
                "Tunic_23_B7D465CA4DCF57F409450789A6DB8590",
                "Tunics",
                if let Drop::Tunic(tunic) = self {
                    tunic.as_ref()
                } else {
                    "0"
                },
                name_map,
            ),
            byte_property(
                "Weapon_22_F3B61F384438EE8A8193F385AE45F88A",
                "Weapons",
                if let Drop::Weapon(weapon) = self {
                    weapon.as_ref()
                } else {
                    "0"
                },
                name_map,
            ),
            byte_property(
                "Spirit_21_55691F2E4B399DB3F381209D33BBE30B",
                "Spirits",
                if let Drop::Spirit(spirit) = self {
                    spirit.as_ref()
                } else {
                    "0"
                },
                name_map,
            ),
            Property::IntProperty(IntProperty {
                name: price_name,
                ancestry: Ancestry {
                    ancestry: Vec::new(),
                },
                property_guid: None,
                duplication_index: 0,
                value: if let Drop::Ore(amount) = self {
                    -*amount
                } else {
                    price
                },
            }),
            byte_property(
                "Ability_29_EBF42DD143E9F82EC9303082A50329F0",
                "Abilities",
                if let Drop::Ability(ability) = self {
                    ability.as_ref()
                } else {
                    "0"
                },
                name_map,
            ),
        ]
        .to_vec()
    }
}

impl Abilities {
    pub fn as_item(&self) -> Items {
        match self {
            Abilities::DoubleJump => Items::DoubleJump,
            Abilities::Dash => Items::Dash,
            Abilities::Attack => todo!(),
            Abilities::DownSmash => Items::DownSmash,
            Abilities::WallRun => Items::WallRun,
            Abilities::Grind => todo!(),
            Abilities::Sprint => Items::Sprint,
            Abilities::Spell => Items::FireBall,
            Abilities::Block => Items::Shield,
            Abilities::SpinAttack => Items::SpinAttack,
        }
    }
    pub fn savegame_index(&self) -> usize {
        match self {
            Abilities::DoubleJump => 2,
            Abilities::Dash => 1,
            Abilities::Attack => 0,
            Abilities::DownSmash => 5,
            Abilities::WallRun => 3,
            Abilities::Grind => 7,
            Abilities::Sprint => 4,
            Abilities::Spell => 6,
            Abilities::Block => 8,
            Abilities::SpinAttack => 9,
        }
    }
}
