use unreal_asset::{exports::ExportNormalTrait, properties::Property};

use super::*;

pub enum Error {
    UnrealAsset(unreal_asset::error::Error),
    Unpak(unpak::Error),
    Assumption,
}

impl From<unreal_asset::error::Error> for Error {
    fn from(value: unreal_asset::error::Error) -> Self {
        Self::UnrealAsset(value)
    }
}

impl From<unpak::Error> for Error {
    fn from(value: unpak::Error) -> Self {
        Self::Unpak(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnrealAsset(e) => f.write_str(&e.to_string()),
            Error::Unpak(e) => f.write_str(&e.to_string()),
            Error::Assumption => {
                f.write_str("data was as expected - you may be on an older version of the game")
            }
        }
    }
}

pub const MOD: &str = "rando_p/Blue Fire/Content";

const SAVEGAME: &str = "/Game/BlueFire/Player/Logic/FrameWork/BlueFireSaveGame";

const PREFIX: &str = "/Game/BlueFire/Maps/World/";

pub fn write(checks: Vec<Check>, app: &mut crate::Rando) -> Result<(), Error> {
    let pak = unpak::Pak::new_from_path(
        dbg!(app.pak.join("Blue Fire-WindowsNoEditor.pak")),
        unpak::Version::FrozenIndex,
        None,
    )?;
    for check in checks {
        match check.context {
            Context::Shop(shopkeep) => {
                let loc = app.pak.join(SAVEGAME.replacen("/Game", MOD, 1));
                let mut savegame = if !loc.exists() {
                    std::fs::create_dir_all(loc.parent().expect("is a file")).unwrap_or_default();
                    pak.read_from_path_to_file(
                        &format!("{SAVEGAME}.uasset"),
                        &app.pak,
                        loc.with_extension("uasset"),
                    )?;
                    pak.read_from_path_to_file(
                        &format!("{SAVEGAME}.uexp"),
                        &app.pak,
                        loc.with_extension("uexp"),
                    )?;
                    let mut savegame = open(&loc)?;
                    let Some(default) = savegame.exports[1].get_normal_export_mut() else {
                        return Err(Error::Assumption)
                    };
                    use strum::IntoEnumIterator;
                    for shop in Shop::iter() {
                        let Property::ArrayProperty(shop) = &mut default.properties[shop as usize] else {
                            return Err(Error::Assumption);
                        };
                        shop.value.clear();
                    }
                    savegame
                } else {
                    open(&loc).map_err(|e| unpak::Error::Other(e.to_string()))?
                };
                let Some(Property::ArrayProperty(shop)) = savegame.exports[1]
                    .get_normal_export_mut()
                    .map(|norm| &mut norm.properties[shopkeep.clone() as usize]) else {
                        return Err(Error::Assumption);
                    };
                use unreal_asset::types::FName;
                shop.value.push(Property::StructProperty(
                    unreal_asset::properties::struct_property::StructProperty {
                        name: FName::from_slice(shopkeep.as_ref()),
                        struct_type: Some(FName::from_slice("Inventory")),
                        struct_guid: None,
                        property_guid: None,
                        duplication_index: 0,
                        serialize_none: true,
                        value: check.drop.get_shop_entry(),
                    },
                ));
                save(&mut savegame, loc)?;
            }
            Context::Cutscene(file) => {
                let loc = app.pak.join(file.replacen("/Game", MOD, 1));
                std::fs::create_dir_all(loc.parent().expect("is a file")).unwrap_or_default();
                pak.read_from_path_to_file(
                    &format!("{file}.uasset"),
                    &app.pak,
                    loc.with_extension("uasset"),
                )?;
                pak.read_from_path_to_file(
                    &format!("{file}.uexp"),
                    &app.pak,
                    loc.with_extension("uexp"),
                )?;
                let mut hook = unreal_asset::Asset::new(
                    std::io::Cursor::new(include_bytes!("../blueprints/Hook.uasset").as_slice()),
                    Some(std::io::Cursor::new(
                        include_bytes!("../blueprints/Hook.uexp").as_slice(),
                    )),
                );
                hook.set_engine_version(unreal_asset::engine_version::EngineVersion::VER_UE4_25);
                hook.parse_data()?;
                let new_name = file.split('/').last().unwrap_or_default();
                // edit Hook name refs to name of cutscene and save to there
                save(&mut hook, format!("{MOD}/BlueFire/Libraries/{new_name}"))?;
                let mut cutscene = open(&loc)?;
                // edit UniversalFunction name refs to name of cutscene
                save(&mut cutscene, &loc)?;
                todo!("make PR for an editable name map")
            }
            Context::Overworld(actor) => todo!(),
        }
    }
    Ok(())
}

impl Drop {
    pub fn get_shop_entry(&self) -> Vec<unreal_asset::properties::Property> {
        use int_property::*;
        use unreal_asset::properties::*;
        use unreal_asset::types::FName;
        [
            Property::ByteProperty(ByteProperty {
                name: FName::from_slice("Item_3_54327288464702F41977D48660F8979E"),
                property_guid: None,
                duplication_index: 0,
                enum_type: Some(FName::from_slice("Items")),
                value: BytePropertyValue::FName(FName::new(
                    if let Drop::Item(item, _) = self {
                        format!("Items::NewEnumerator{}", item.as_ref())
                    } else {
                        "Items::NewEnumerator0".to_string()
                    },
                    0,
                )),
            }),
            Property::IntProperty(IntProperty {
                name: FName::from_slice("Amount_6_185C591747EF40A592FB63886FDB4281"),
                property_guid: None,
                duplication_index: 0,
                value: if let Drop::Item(_, amount) = self {
                    *amount
                } else {
                    1
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
                value: if let Drop::Item(_, amount) = self {
                    *amount
                } else {
                    1
                },
            }),
            Property::ByteProperty(ByteProperty {
                name: FName::from_slice("Type_17_9B84CFD04716464F71190CB4CECE0F49"),
                property_guid: None,
                duplication_index: 0,
                enum_type: Some(FName::from_slice("InventoryItemType")),
                value: BytePropertyValue::FName(FName::new(
                    format!("InventoryItemType::NewEnumerator{}", self.as_ref()),
                    0,
                )),
            }),
            Property::ByteProperty(ByteProperty {
                name: FName::from_slice("Tunic_23_B7D465CA4DCF57F409450789A6DB8590"),
                property_guid: None,
                duplication_index: 0,
                enum_type: Some(FName::from_slice("Tunics")),
                value: BytePropertyValue::FName(FName::new(
                    if let Drop::Tunic(tunic) = self {
                        format!("Tunics::NewEnumerator{}", tunic.as_ref())
                    } else {
                        "Tunics::NewEnumerator0".to_string()
                    },
                    0,
                )),
            }),
            Property::ByteProperty(ByteProperty {
                name: FName::from_slice("Weapon_22_F3B61F384438EE8A8193F385AE45F88A"),
                property_guid: None,
                duplication_index: 0,
                enum_type: Some(FName::from_slice("Weapons")),
                value: BytePropertyValue::FName(FName::new(
                    if let Drop::Weapon(weapon) = self {
                        format!("Weapons::NewEnumerator{}", weapon.as_ref())
                    } else {
                        "Weapons::NewEnumerator0".to_string()
                    },
                    0,
                )),
            }),
            Property::ByteProperty(ByteProperty {
                name: FName::from_slice("Spirit_21_55691F2E4B399DB3F381209D33BBE30B"),
                property_guid: None,
                duplication_index: 0,
                enum_type: Some(FName::from_slice("Spirits")),
                value: BytePropertyValue::FName(FName::new(
                    if let Drop::Spirit(spirit) = self {
                        format!("Spirits::NewEnumerator{}", spirit.as_ref())
                    } else {
                        "Spirits::NewEnumerator0".to_string()
                    },
                    0,
                )),
            }),
            Property::IntProperty(IntProperty {
                name: FName::from_slice("Price_26_80A37F3645AE8292A9F311B86094C095"),
                property_guid: None,
                duplication_index: 0,
                value: 500,
            }),
            Property::ByteProperty(ByteProperty {
                name: FName::from_slice("Ability_29_EBF42DD143E9F82EC9303082A50329F0"),
                property_guid: None,
                duplication_index: 0,
                enum_type: Some(FName::from_slice("Abilities")),
                value: BytePropertyValue::FName(FName::new(
                    if let Drop::Ability(ability) = self {
                        format!("Abilities::NewEnumerator{}", ability.as_ref())
                    } else {
                        "Abilities::NewEnumerator0".to_string()
                    },
                    0,
                )),
            }),
        ]
        .to_vec()
    }
}
