use super::logic::*;
use crate::{io::*, map::*};
use unreal_asset::{exports::*, properties::*, reader::asset_trait::AssetTrait, types::FName, *};

mod cutscenes;
mod overworld;
mod savegames;
mod specific;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unreal_asset: {0}")]
    UnrealAsset(#[from] unreal_asset::error::Error),
    #[error("unpak: {0}")]
    Unpak(#[from] unpak::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("data was not as expected - you may have an older version of the game")]
    Assumption,
}

pub const MOD: &str = "rando_p";

const SAVEGAME: &str = "Blue Fire/Content/BlueFire/Player/Logic/FrameWork/BlueFireSaveGame.uasset";

const PREFIX: &str = "Blue Fire/Content/BlueFire/Maps/World/";

fn extract(
    app: &crate::Rando,
    pak: &unpak::Pak,
    path: &str,
) -> Result<(Asset<std::fs::File>, std::path::PathBuf), Error> {
    let loc = app.pak.join(MOD).join(path);
    if path != "Blue Fire/Content/BlueFire/Maps/World/A02_ArcaneTunnels/A02_EastArcane.umap" {
        std::fs::create_dir_all(loc.parent().expect("is a file"))?;
        pak.read_to_file(path, &loc)?;
        pak.read_to_file(
            &path.replace(".uasset", ".uexp").replace(".umap", ".uexp"),
            loc.with_extension("uexp"),
        )?;
    }
    Ok((open(&loc)?, loc))
}

fn byte_property(name: &str, enum_type: &str, val: &str) -> Property {
    Property::ByteProperty(int_property::ByteProperty {
        name: FName::from_slice(name),
        property_guid: None,
        duplication_index: 0,
        enum_type: Some(FName::from_slice(enum_type)),
        value: int_property::BytePropertyValue::FName(FName::new(
            format!("{}::NewEnumerator{}", enum_type, val),
            0,
        )),
    })
}

fn set_byte(
    name: &str,
    enum_type: &str,
    val: &str,
    export: &mut normal_export::NormalExport,
) -> Result<(), Error> {
    match export.properties.iter_mut().find_map(|prop| {
        cast!(Property, ByteProperty, prop).filter(|byte| byte.name.content == name)
    }) {
        Some(byte) => {
            use int_property::BytePropertyValue;
            cast!(BytePropertyValue, FName, &mut byte.value)
                .ok_or(Error::Assumption)?
                .content = format!("{}::NewEnumerator{}", enum_type, val)
        }
        None => export.properties.push(byte_property(name, enum_type, val)),
    }
    Ok(())
}

pub fn write(
    checks: std::collections::HashMap<Locations, Vec<Check>>,
    savegames: Vec<Check>,
    cutscenes: Vec<Check>,
    cases: Vec<Check>,
    app: &crate::Rando,
) -> Result<(), Error> {
    let pak = unpak::Pak::new(
        app.pak.join("Blue Fire-WindowsNoEditor.pak"),
        unpak::Version::FrozenIndex,
    )?;
    // correct the shenanigans in spirit hunter
    let loc = app
        .pak
        .join(MOD)
        .join("Blue Fire/Content/BlueFire/Maps/World/A02_ArcaneTunnels/A02_EastArcane.umap");
    std::fs::create_dir_all(loc.parent().expect("is a file"))?;
    pak.read_to_file(
        "Blue Fire/Content/BlueFire/Maps/World/A02_ArcaneTunnels/A02_EastArcane.umap",
        &loc,
    )?;
    pak.read_to_file(
        "Blue Fire/Content/BlueFire/Maps/World/A02_ArcaneTunnels/A02_EastArcane.uexp",
        loc.with_extension("uexp"),
    )?;
    let mut spirit_hunter = open(&loc)?;
    spirit_hunter.exports[440]
        .get_base_export_mut()
        .object_name
        .content = "Pickup_A02_SRF2".to_string();
    save(&mut spirit_hunter, &loc)?;
    std::thread::scope(|thread| {
        thread.spawn(|| -> Result<(), Error> { overworld::write(checks, &app, &pak) });
        thread.spawn(|| -> Result<(), Error> { cutscenes::write(cutscenes, &app, &pak) });
        thread.spawn(|| -> Result<(), Error> { savegames::write(savegames, &app, &pak) });
        thread.spawn(|| -> Result<(), Error> { specific::write(cases, &app, &pak) });
    });
    // change the logo so people know it worked
    let logo = app
        .pak
        .join(MOD)
        .join("Blue Fire/Content/BlueFire/HUD/Menu/Blue-Fire-Logo.uasset");
    std::fs::create_dir_all(logo.parent().expect("is a file"))?;
    std::fs::write(&logo, include_bytes!("blueprints/logo.uasset"))?;
    std::fs::write(
        logo.with_extension("uexp"),
        include_bytes!("blueprints/logo.uexp"),
    )?;
    // package the mod in the most scuffed way possible
    std::fs::write("UnrealPak.exe", include_bytes!("UnrealPak.exe"))?;
    std::fs::write("pak.bat", include_str!("pak.bat"))?;
    // for some reason calling with rust doesn't work so a batch file will do
    std::process::Command::new("./pak.bat")
        .arg(app.pak.join("rando_p"))
        .output()?;
    Ok(())
}

fn create_hook<C: std::io::Read + std::io::Seek>(
    app: &crate::Rando,
    pak: &unpak::Pak,
    get_hook: impl Fn(&std::path::PathBuf) -> Result<Asset<C>, Error>,
    drop: &Drop,
    cutscene: &str,
    index: usize,
) -> Result<(), Error> {
    let mut loc = app
        .pak
        .join(MOD)
        .join("Blue Fire/Content/BlueFire/Libraries");
    std::fs::create_dir_all(&loc)?;
    let new_name = format!("{}_Hook", cutscene.split('/').last().unwrap_or_default());
    loc = loc.join(&new_name).with_extension("uasset");
    let mut hook = get_hook(&loc)?;
    // edit the item given by the kismet bytecode in the hook
    let exports::Export::FunctionExport(
                    exports::function_export::FunctionExport{
                        struct_export: struct_export::StructExport{
                            script_bytecode:Some(bytecode),
                            ..
                        },
                        ..
                    }
                ) = &mut hook.exports[index] else {
                    return Err(Error::Assumption)
                };
    use unreal_asset::kismet::*;
    let [
            KismetExpression::ExLet(item_type),
            KismetExpression::ExLet(index),
            KismetExpression::ExLet(amount),
            KismetExpression::ExLetBool(key_item)
        ] = &mut bytecode[0..4] else {
            return Err(Error::Assumption)
        };
    let [
            KismetExpression::ExByteConst(item_type),
            KismetExpression::ExByteConst(index),
            KismetExpression::ExIntConst(amount),
        ] = [
            item_type.expression.as_mut(),
            index.expression.as_mut(),
            amount.expression.as_mut(),
        ] else {
            return Err(Error::Assumption)
        };
    item_type.value = drop.as_u8();
    index.value = drop.inner_as_u8();
    amount.value = match &drop {
        Drop::Item(_, amount) => *amount,
        Drop::Ore(amount) => *amount,
        _ => 1,
    };
    *key_item.assignment_expression = match &drop {
        Drop::Item(item, _) if item.is_key_item() => KismetExpression::ExTrue(ExTrue::default()),
        _ => KismetExpression::ExFalse(ExFalse::default()),
    };
    let self_refs: Vec<usize> = hook
        .get_name_map_index_list()
        .iter()
        .enumerate()
        .filter_map(|(i, name)| name.contains("hook").then_some(i))
        .collect();
    for i in self_refs {
        let name = hook.get_name_reference_mut(i as i32);
        *name = name.replace("hook", &new_name);
    }
    save(&mut hook, loc)?;
    let loc = app.pak.join(MOD).join(cutscene).with_extension("uasset");
    std::fs::create_dir_all(loc.parent().expect("is a file"))?;
    pak.read_to_file(&format!("{cutscene}.uasset"), &loc)?;
    pak.read_to_file(&format!("{cutscene}.uexp"), loc.with_extension("uexp"))?;
    let mut cutscene = open(&loc)?;
    let universal_refs: Vec<usize> = cutscene
        .get_name_map_index_list()
        .iter()
        .enumerate()
        .filter_map(|(i, name)| name.contains("UniversalFunctions").then_some(i))
        .collect();
    for i in universal_refs {
        let name = cutscene.get_name_reference_mut(i as i32);
        *name = name.replace("UniversalFunctions", &new_name);
    }
    save(&mut cutscene, &loc)?;
    Ok(())
}

impl Drop {
    pub fn as_shop_entry(&self, price: i32) -> Vec<unreal_asset::properties::Property> {
        use int_property::*;
        [
            byte_property(
                "Item_3_54327288464702F41977D48660F8979E",
                "Items",
                match self {
                    Drop::Item(item, _) => item.as_ref(),
                    Drop::Ore(_) => Items::KinbankDebitCard.as_ref(),
                    Drop::Duck => Items::Duck.as_ref(),
                    _ => "25",
                },
            ),
            Property::IntProperty(IntProperty {
                name: FName::from_slice("Amount_6_185C591747EF40A592FB63886FDB4281"),
                property_guid: None,
                duplication_index: 0,
                value: match self {
                    Drop::Item(_, amount) => *amount,
                    Drop::Emote(_) => 0,
                    _ => 1,
                },
            }),
            Property::BoolProperty(BoolProperty {
                name: FName::from_slice("Resets_8_E303F5DF4270CCEE83F05F974F3661C9"),
                property_guid: None,
                duplication_index: 0,
                value: false,
            }),
            Property::IntProperty(IntProperty {
                name: FName::from_slice("OriginalAmount_11_58C3C17D426D49A439C0EE85D7E9B6EC"),
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
            ),
            byte_property(
                "Tunic_23_B7D465CA4DCF57F409450789A6DB8590",
                "Tunics",
                if let Drop::Tunic(tunic) = self {
                    tunic.as_ref()
                } else {
                    "0"
                },
            ),
            byte_property(
                "Weapon_22_F3B61F384438EE8A8193F385AE45F88A",
                "Weapons",
                if let Drop::Weapon(weapon) = self {
                    weapon.as_ref()
                } else {
                    "0"
                },
            ),
            byte_property(
                "Spirit_21_55691F2E4B399DB3F381209D33BBE30B",
                "Spirits",
                if let Drop::Spirit(spirit) = self {
                    spirit.as_ref()
                } else {
                    "0"
                },
            ),
            Property::IntProperty(IntProperty {
                name: FName::from_slice("Price_26_80A37F3645AE8292A9F311B86094C095"),
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
