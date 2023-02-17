use unreal_asset::{exports::*, properties::*, reader::asset_trait::AssetTrait, types::FName, *};

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
                f.write_str("data was not as expected - you may have an older version of the game")
            }
        }
    }
}

pub const MOD: &str = "rando_p/Blue Fire/Content";

const SAVEGAME: &str = "/Game/BlueFire/Player/Logic/FrameWork/BlueFireSaveGame";

const PREFIX: &str = "/Game/BlueFire/Maps/World/";

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

pub fn write(checks: Vec<Check>, app: &mut crate::Rando) -> Result<(), Error> {
    let pak_path = app.pak.join("Blue Fire-WindowsNoEditor.pak");
    let pak = unpak::Pak::new_from_path(&pak_path, unpak::Version::FrozenIndex, None)?;
    for Check {
        location,
        context,
        drop,
        ..
    } in checks
    {
        match context {
            Context::Shop(shopkeep) => {
                let loc = app
                    .pak
                    .join(SAVEGAME.replacen("/Game", MOD, 1))
                    .with_extension("uasset");
                let mut savegame = if !loc.exists() {
                    std::fs::create_dir_all(loc.parent().expect("is a file")).unwrap_or_default();
                    pak.read_from_path_to_file(&format!("{SAVEGAME}.uasset"), &pak_path, &loc)?;
                    pak.read_from_path_to_file(
                        &format!("{SAVEGAME}.uexp"),
                        &pak_path,
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
                    open(&loc)?
                };
                let Some(Property::ArrayProperty(shop)) = savegame.exports[1]
                    .get_normal_export_mut()
                    .map(|norm| &mut norm.properties[shopkeep.clone() as usize]) else {
                        return Err(Error::Assumption);
                    };
                shop.value.push(Property::StructProperty(
                    unreal_asset::properties::struct_property::StructProperty {
                        name: FName::from_slice(shopkeep.as_ref()),
                        struct_type: Some(FName::from_slice("Inventory")),
                        struct_guid: None,
                        property_guid: None,
                        duplication_index: 0,
                        serialize_none: true,
                        value: drop.get_shop_entry(),
                    },
                ));
                save(&mut savegame, loc)?;
            }
            Context::Cutscene(cutscene) => {
                std::fs::create_dir_all(app.pak.join(MOD).join("BlueFire/Libraries"))
                    .unwrap_or_default();
                let mut hook = unreal_asset::Asset::new(
                    std::io::Cursor::new(include_bytes!("../blueprints/hook.uasset").as_slice()),
                    Some(std::io::Cursor::new(
                        include_bytes!("../blueprints/hook.uexp").as_slice(),
                    )),
                );
                hook.set_engine_version(unreal_asset::engine_version::EngineVersion::VER_UE4_25);
                hook.parse_data()?;
                let new_name = cutscene.split('/').last().unwrap_or_default();
                // edit hook name refs to this new name and save to there
                save(&mut hook, format!("{MOD}/BlueFire/Libraries/{new_name}"))?;
                let loc = app.pak.join(cutscene.replacen("/Game", MOD, 1));
                std::fs::create_dir_all(loc.parent().expect("is a file")).unwrap_or_default();
                pak.read_from_path_to_file(
                    &format!("{cutscene}.uasset"),
                    &pak_path,
                    loc.with_extension("uasset"),
                )?;
                pak.read_from_path_to_file(
                    &format!("{cutscene}.uexp"),
                    &pak_path,
                    loc.with_extension("uexp"),
                )?;
                let mut cutscene = open(&loc)?;
                // edit UniversalFunction name refs to name of cutscene
                save(&mut cutscene, &loc)?;
                todo!("make PR for an editable name map")
            }
            Context::Overworld(actor_name) => {
                let loc = app
                    .pak
                    .join(format!("{PREFIX}{location}").replacen("/Game", MOD, 1))
                    .with_extension("umap");
                if !loc.exists() {
                    std::fs::create_dir_all(loc.parent().expect("is a file")).unwrap_or_default();
                    pak.read_from_path_to_file(
                        &format!("{PREFIX}{location}.umap"),
                        &pak_path,
                        &loc,
                    )?;
                    pak.read_from_path_to_file(
                        &format!("{PREFIX}{location}.uexp"),
                        &pak_path,
                        loc.with_extension("uexp"),
                    )?;
                }
                let mut map = open(&loc)?;
                let Some(i) = map.exports.iter().position(|ex| ex.get_base_export().object_name.content == actor_name) else {
                    return Err(Error::Assumption)
                };
                let class = map
                    .get_import(map.exports[i].get_base_export().class_index)
                    .map(|import| import.object_name.content.as_str())
                    .unwrap_or_default();
                let is_chest = matches!(
                    class,
                    "Chest_Master_C" | "Chest_Master_Child_C" | "Chest_Dance_C"
                );
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
                            let int_property::BytePropertyValue::FName(name) = &mut byte.value else {
                                return Err(Error::Assumption);
                            };
                            name.content = format!("{}::NewEnumerator{}", enum_type, val)
                        }
                        None => export.properties.push(byte_property(name, enum_type, val)),
                    }
                    Ok(())
                }
                #[allow(unused_variables)]
                match &drop {
                    Drop::Item(item, amount) if is_chest => {
                        let Some(chest) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Item", "Items", item.as_ref(), chest)?;
                        match chest.properties.iter_mut().find_map(|prop| {
                            cast!(Property, BoolProperty, prop)
                                .filter(|bool| bool.name.content == "KeyItem")
                        }) {
                            Some(key_item) => key_item.value = item.is_key_item(),
                            None => {
                                if item.is_key_item() {
                                    chest.properties.push(Property::BoolProperty(
                                        int_property::BoolProperty {
                                            name: FName::from_slice("KeyItem"),
                                            property_guid: None,
                                            duplication_index: 0,
                                            value: true,
                                        },
                                    ))
                                }
                            }
                        }
                        match chest.properties.iter_mut().find_map(|prop| {
                            cast!(Property, IntProperty, prop)
                                .filter(|amount| amount.name.content == "Amount")
                        }) {
                            Some(num) => num.value = *amount as i32,
                            None => chest.properties.push(Property::IntProperty(
                                int_property::IntProperty {
                                    name: FName::from_slice("Amount"),
                                    property_guid: None,
                                    duplication_index: 0,
                                    value: *amount as i32,
                                },
                            )),
                        }
                    }
                    Drop::Item(item, amount) if class == "Pickup_C" => {
                        let Some(pickup) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Type", "PickUpList", "1", pickup)?;
                        set_byte("Item", "Items", item.as_ref(), pickup)?;
                        todo!("duplicate the pickup on the amount")
                    }
                    Drop::Item(item, amount) => todo!(),
                    Drop::Weapon(weapon) if is_chest => {
                        let Some(chest) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Weapon", "Weapons", weapon.as_ref(), chest)?;
                    }
                    Drop::Weapon(weapon) => todo!(),
                    Drop::Tunic(tunic) if is_chest => {
                        let Some(chest) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Tunic", "Tunics", tunic.as_ref(), chest)?;
                    }
                    Drop::Tunic(tunic) => todo!(),
                    Drop::Spirit(spirit) if is_chest => {
                        let Some(chest) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Spirit", "Spirits", spirit.as_ref(), chest)?;
                    }
                    Drop::Spirit(spirit) if class == "Spirit_C" => {
                        let Some(spirit_bp) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Spirit", "Spirits", spirit.as_ref(), spirit_bp)?;
                    }
                    Drop::Spirit(spirit) => todo!(),
                    Drop::Ability(ability) if is_chest => {
                        let Some(chest) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Ability", "Abilities", ability.as_ref(), chest)?;
                    }
                    Drop::Ability(ability) => todo!(),
                    Drop::Emote(emote) if class == "EmoteStatue_BP_C" => {
                        let Some(statue) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Emote", "E_Emotes", emote.as_ref(), statue)?;
                    }
                    Drop::Emote(emote) => todo!(),
                    Drop::Ore(amount) if class == "Pickup_C" => {
                        let Some(pickup) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Type", "PickUpList", "0", pickup)?;
                        match pickup.properties.iter_mut().find_map(|prop| {
                            cast!(Property, IntProperty, prop)
                                .filter(|amount| amount.name.content == "Souls/LifeAmount")
                        }) {
                            Some(num) => num.value = *amount as i32,
                            None => pickup.properties.push(Property::IntProperty(
                                int_property::IntProperty {
                                    name: FName::from_slice("Souls/LifeAmount"),
                                    property_guid: None,
                                    duplication_index: 0,
                                    value: *amount as i32,
                                },
                            )),
                        }
                    }
                    Drop::Ore(amount) => todo!(),
                    Drop::Duck if is_chest => {
                        let Some(chest) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Item", "Items", Items::Duck.as_ref(), chest)?;
                        match chest.properties.iter_mut().find_map(|prop| {
                            cast!(Property, BoolProperty, prop)
                                .filter(|bool| bool.name.content == "KeyItem")
                        }) {
                            Some(key_item) => key_item.value = true,
                            None => chest.properties.push(Property::BoolProperty(
                                int_property::BoolProperty {
                                    name: FName::from_slice("KeyItem"),
                                    property_guid: None,
                                    duplication_index: 0,
                                    value: true,
                                },
                            )),
                        }
                        match chest.properties.iter_mut().find_map(|prop| {
                            cast!(Property, IntProperty, prop)
                                .filter(|amount| amount.name.content == "Amount")
                        }) {
                            Some(num) => num.value = 1,
                            None => chest.properties.push(Property::IntProperty(
                                int_property::IntProperty {
                                    name: FName::from_slice("Amount"),
                                    property_guid: None,
                                    duplication_index: 0,
                                    value: 1,
                                },
                            )),
                        }
                    }
                    Drop::Duck if class == "Pickup_C" => {
                        let Some(pickup) = map.exports[i].get_normal_export_mut() else {
                            return Err(Error::Assumption)
                        };
                        set_byte("Type", "PickUpList", "1", pickup)?;
                        set_byte("Item", "Items", Items::Duck.as_ref(), pickup)?;
                    }
                    Drop::Duck => todo!(),
                }
                // find the actor and delete/replace it using the reference in the collectables map to reflect the drop
                save(&mut map, &loc)?;
            }
        }
    }
    Ok(())
}

impl Drop {
    pub fn get_shop_entry(&self) -> Vec<unreal_asset::properties::Property> {
        use int_property::*;
        [
            byte_property(
                "Item_3_54327288464702F41977D48660F8979E",
                "Items",
                if let Drop::Item(item, _) = self {
                    item.as_ref()
                } else {
                    "25"
                },
            ),
            Property::IntProperty(IntProperty {
                name: FName::from_slice("Amount_6_185C591747EF40A592FB63886FDB4281"),
                property_guid: None,
                duplication_index: 0,
                value: if let Drop::Item(_, amount) = self {
                    *amount as i32
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
                    *amount as i32
                } else {
                    1
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
                value: 500,
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
