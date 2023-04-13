use super::*;
use crate::{io::*, map::*};
use unreal_asset::{exports::*, properties::*, reader::asset_trait::AssetTrait, types::FName, *};

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
    Ok((
        {
            if !loc.exists() {
                std::fs::create_dir_all(loc.parent().expect("is a file"))?;
                pak.read_to_file(path, &loc)?;
                pak.read_to_file(
                    &path.replace(".uasset", ".uexp").replace(".umap", ".uexp"),
                    loc.with_extension("uexp"),
                )?;
            }
            open(&loc)?
        },
        loc,
    ))
}

fn get_savegame(
    app: &crate::Rando,
    pak: &unpak::Pak,
) -> Result<(Asset<std::fs::File>, std::path::PathBuf), Error> {
    let initialised = app.pak.join(MOD).join(SAVEGAME).exists();
    let (mut savegame, loc) = extract(app, pak, SAVEGAME)?;
    if !initialised {
        let default = savegame.exports[1]
            .get_normal_export_mut()
            .ok_or(Error::Assumption)?;
        if app.dash {
            cast!(Property, StructProperty, &mut default.properties[2])
                .and_then(|inventory| cast!(Property, BoolProperty, &mut inventory.value[1]))
                .ok_or(Error::Assumption)?
                .value = false;
        }
        if app.emotes {
            cast!(Property, ArrayProperty, &mut default.properties[15])
                .ok_or(Error::Assumption)?
                .value
                .clear()
        }
    }
    Ok((savegame, loc))
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

pub fn write(checks: Vec<Check>, app: &crate::Rando) -> Result<(), Error> {
    let mut used = Vec::with_capacity(checks.len());
    let pak = unpak::Pak::new(
        app.pak.join("Blue Fire-WindowsNoEditor.pak"),
        unpak::Version::FrozenIndex,
    )?;
    let (mut bullshit, loc) = extract(
        app,
        &pak,
        "Blue Fire/Content/BlueFire/Maps/World/A02_ArcaneTunnels/A02_EastArcane.umap",
    )?;
    bullshit.exports[440]
        .get_base_export_mut()
        .object_name
        .content = "Pickup_A02_SRF2".to_string();
    save(&mut bullshit, &loc)?;
    let mut shop_emotes: Vec<_> = checks
        .iter()
        .filter_map(|check| {
            if let Context::Shop(keep, i, _) = check.context {
                if matches!(check.drop, Drop::Emote(_) | Drop::Ability(_)) {
                    return Some((keep, i));
                }
            }
            None
        })
        .collect();
    // sort descending
    shop_emotes.sort_unstable_by_key(|(_, i)| std::cmp::Reverse(*i));
    for Check {
        location,
        context,
        drop,
        ..
    } in checks
    {
        match context {
            Context::Shop(shopkeep, index, price) => {
                let (mut savegame, loc) = get_savegame(app, &pak)?;
                savegame.exports[1]
                    .get_normal_export_mut()
                    .and_then(|norm| {
                        cast!(
                            Property,
                            ArrayProperty,
                            &mut norm.properties[shopkeep as usize]
                        )
                    })
                    .ok_or(Error::Assumption)?
                    .value[index] = Property::StructProperty(
                    unreal_asset::properties::struct_property::StructProperty {
                        name: FName::from_slice(shopkeep.as_ref()),
                        struct_type: Some(FName::from_slice("Inventory")),
                        struct_guid: None,
                        property_guid: None,
                        duplication_index: 0,
                        serialize_none: true,
                        value: drop.as_shop_entry(price),
                    },
                );
                if matches!(drop, Drop::Emote(_) | Drop::Ability(_)) {
                    let (mut map, loc) = extract(app, &pak, &format!("{PREFIX}{location}.umap"))?;
                    let insert = map.exports.len();
                    transplant(
                        match drop {
                            Drop::Ability(_) => 36,
                            Drop::Emote(_) => 20,
                            _ => unimplemented!(),
                        },
                        &mut map,
                        &open_from_bytes(
                            include_bytes!("../blueprints/collectibles.umap").as_slice(),
                            include_bytes!("../blueprints/collectibles.uexp").as_slice(),
                        )?,
                    );
                    let mut pos = shopkeep.location();
                    let (x, y) = (9.0 * index as f32).to_radians().sin_cos();
                    pos.x -= 1000.0 * x;
                    pos.y -= 1000.0 * y;
                    set_location(insert, &mut map, pos, (0.0, 0.0, 0.0));
                    let norm = map.exports[insert]
                        .get_normal_export_mut()
                        .ok_or(Error::Assumption)?;
                    if let Drop::Emote(emote) = drop {
                        use int_property::BytePropertyValue;
                        cast!(
                            BytePropertyValue,
                            FName,
                            &mut cast!(Property, ByteProperty, &mut norm.properties[2])
                                .ok_or(Error::Assumption)?
                                .value
                        )
                        .ok_or(Error::Assumption)?
                        .content = format!("E_Emotes::NewEnumerator{}", emote.as_ref());
                    }
                    if let Drop::Ability(ability) = drop {
                        set_byte("Ability", "Abilities", ability.as_ref(), norm)?;
                        set_byte("Type", "InventoryItemType", drop.as_ref(), norm)?;
                    }
                    cast!(
                        Property,
                        StrProperty,
                        &mut norm.properties[match drop {
                            Drop::Ability(_) => 11,
                            Drop::Emote(_) => 6,
                            _ => unimplemented!(),
                        }]
                    )
                    .ok_or(Error::Assumption)?
                    .value = Some(format!("{}{index}", shopkeep.as_ref()));
                    save(&mut map, loc)?;
                }
                save(&mut savegame, loc)?;
            }
            // sapphire ore turns to house keys?????
            Context::Cutscene(cutscene) => {
                let loc = app
                    .pak
                    .join(MOD)
                    .join("Blue Fire/Content/BlueFire/Libraries");
                std::fs::create_dir_all(&loc)?;
                let mut hook = open_from_bytes(
                    include_bytes!("../blueprints/hook.uasset").as_slice(),
                    include_bytes!("../blueprints/hook.uexp").as_slice(),
                )?;
                // edit the item given by the kismet bytecode in the hook
                let exports::Export::FunctionExport(
                    exports::function_export::FunctionExport{
                        struct_export: struct_export::StructExport{
                            script_bytecode:Some(bytecode),
                            ..
                        },
                        ..
                    }
                ) = &mut hook.exports[69] else {
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
                let (
                    KismetExpression::ExByteConst(item_type),
                    KismetExpression::ExByteConst(index),
                    KismetExpression::ExIntConst(amount),
                ) = (
                    item_type.expression.as_mut(),
                    index.expression.as_mut(),
                    amount.expression.as_mut(),
                ) else {
                    return Err(Error::Assumption)
                };
                item_type.value = drop.as_u8();
                index.value = drop.inner_as_u8();
                amount.value = match &drop {
                    Drop::Item(_, amount) => *amount,
                    Drop::Ore(amount) => *amount,
                    _ => 1,
                };
                key_item.assignment_expression = Box::new(match &drop {
                    Drop::Item(item, _) if item.is_key_item() => {
                        KismetExpression::ExTrue(ExTrue::default())
                    }
                    _ => KismetExpression::ExFalse(ExFalse::default()),
                });
                let new_name = format!("{}_Hook", cutscene.split('/').last().unwrap_or_default());
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
                save(&mut hook, loc.join(&new_name).with_extension("uasset"))?;
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
            }
            Context::Overworld(name) => {
                let (mut map, loc) = extract(app, &pak, &format!("{PREFIX}{location}.umap"))?;
                let mut i = map
                    .exports
                    .iter()
                    .position(|ex| ex.get_base_export().object_name.content == name)
                    .ok_or(Error::Assumption)?;
                let class = map
                    .get_import(map.exports[i].get_base_export().class_index)
                    .map(|import| import.object_name.content.to_owned())
                    .unwrap_or_default();
                let is_chest = || {
                    matches!(
                        class.as_str(),
                        "Chest_Master_C" | "Chest_Master_Child_C" | "Chest_Dance_C"
                    )
                };
                let mut replace = |actor: usize| -> Result<(), Error> {
                    let donor = open_from_bytes(
                        include_bytes!("../blueprints/collectibles.umap").as_slice(),
                        include_bytes!("../blueprints/collectibles.uexp").as_slice(),
                    )?;
                    delete(i, &mut map);
                    let insert = map.exports.len();
                    transplant(actor, &mut map, &donor);
                    let loc = get_location(i, &map);
                    set_location(
                        insert,
                        &mut map,
                        loc,
                        // some of the ducks are impossible to physically reach
                        match location {
                            Locations::ArcaneDucks => (0.0, 150.0, 0.0),
                            Locations::ForestDucks if name == "Duck" => (0.0, 0.0, 800.0),
                            Locations::AbandonedPath if name == "Duck" => (0.0, 0.0, 300.0),
                            Locations::Stoneheart if name == "Duck2" => (0.0, -100.0, 0.0),
                            Locations::FirefallDucks | Locations::Sirion => (0.0, 0.0, 100.0),
                            Locations::WaterwayDucks => (500.0, 0.0, 100.0),
                            _ => (0.0, 0.0, 0.0),
                        },
                    );
                    // create unique id to prevent multiple checks being registered as collected
                    let mut counter: u16 = match name.rfind(|ch: char| ch.to_digit(10).is_none()) {
                        Some(index) if index != name.len() - 1 => {
                            name[index + 1..].parse().unwrap()
                        }
                        _ => 1,
                    };
                    while used.contains(&format!("{name}{counter}")) {
                        counter += 1;
                    }
                    used.push(format!("{name}{counter}"));
                    let norm = &mut map.exports[insert]
                        .get_normal_export_mut()
                        .ok_or(Error::Assumption)?;
                    match norm.properties.iter_mut().find_map(|prop| {
                        cast!(Property, StrProperty, prop).filter(|id| id.name.content == "ID")
                    }) {
                        Some(id) => id.value = Some(format!("{name}{counter}")),
                        None => {
                            norm.properties
                                .push(Property::StrProperty(str_property::StrProperty {
                                    name: FName::from_slice("ID"),
                                    property_guid: None,
                                    duplication_index: 0,
                                    value: Some(format!("{name}{counter}")),
                                }))
                        }
                    }
                    i = insert;
                    Ok(())
                };
                match &drop {
                    Drop::Item(item, amount) => {
                        if !is_chest() {
                            replace(36)?;
                        }
                        let chest = map.exports[i]
                            .get_normal_export_mut()
                            .ok_or(Error::Assumption)?;
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Item", "Items", item.as_ref(), chest)?;
                        match chest.properties.iter_mut().find_map(|prop| {
                            cast!(Property, BoolProperty, prop)
                                .filter(|bool| bool.name.content == "KeyItem")
                        }) {
                            Some(key_item) => key_item.value = item.is_key_item(),
                            None if item.is_key_item() => chest.properties.push(
                                Property::BoolProperty(int_property::BoolProperty {
                                    name: FName::from_slice("KeyItem"),
                                    property_guid: None,
                                    duplication_index: 0,
                                    value: true,
                                }),
                            ),
                            _ => (),
                        }
                        match chest.properties.iter_mut().find_map(|prop| {
                            cast!(Property, IntProperty, prop)
                                .filter(|amount| amount.name.content == "Amount")
                        }) {
                            Some(num) => num.value = *amount,
                            None => chest.properties.push(Property::IntProperty(
                                int_property::IntProperty {
                                    name: FName::from_slice("Amount"),
                                    property_guid: None,
                                    duplication_index: 0,
                                    value: *amount,
                                },
                            )),
                        }
                    }
                    Drop::Weapon(weapon) => {
                        if !is_chest() {
                            replace(36)?;
                        }
                        let chest = map.exports[i]
                            .get_normal_export_mut()
                            .ok_or(Error::Assumption)?;
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Weapon", "Weapons", weapon.as_ref(), chest)?;
                    }
                    Drop::Tunic(tunic) => {
                        if !is_chest() {
                            replace(36)?;
                        }
                        let chest = map.exports[i]
                            .get_normal_export_mut()
                            .ok_or(Error::Assumption)?;
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Tunic", "Tunics", tunic.as_ref(), chest)?;
                    }
                    Drop::Spirit(spirit) if is_chest() => {
                        let chest = map.exports[i]
                            .get_normal_export_mut()
                            .ok_or(Error::Assumption)?;
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Amulet", "Spirits", spirit.as_ref(), chest)?;
                    }
                    Drop::Spirit(spirit) => {
                        if class != "Spirit_C" {
                            replace(26)?;
                        }
                        let spirit_bp = map.exports[i]
                            .get_normal_export_mut()
                            .ok_or(Error::Assumption)?;
                        set_byte("Amulet", "Spirits", spirit.as_ref(), spirit_bp)?;
                    }
                    Drop::Ability(ability) => {
                        if !is_chest() {
                            replace(36)?;
                        }
                        let chest = map.exports[i]
                            .get_normal_export_mut()
                            .ok_or(Error::Assumption)?;
                        set_byte("Type", "InventoryItemType", drop.as_ref(), chest)?;
                        set_byte("Ability", "Abilities", ability.as_ref(), chest)?;
                    }
                    Drop::Emote(emote) => {
                        if class != "EmoteStatue_BP_C" {
                            replace(20)?;
                        }
                        let statue = map.exports[i]
                            .get_normal_export_mut()
                            .ok_or(Error::Assumption)?;
                        set_byte("Emote", "E_Emotes", emote.as_ref(), statue)?;
                    }
                    Drop::Ore(amount) => {
                        if class != "Pickup_C" {
                            replace(5)?;
                        }
                        let pickup = map.exports[i]
                            .get_normal_export_mut()
                            .ok_or(Error::Assumption)?;
                        set_byte("Type", "PickUpList", "5", pickup)?;
                        match pickup.properties.iter_mut().find_map(|prop| {
                            cast!(Property, IntProperty, prop)
                                .filter(|amount| amount.name.content == "Souls/LifeAmount")
                        }) {
                            Some(num) => num.value = *amount,
                            None => pickup.properties.push(Property::IntProperty(
                                int_property::IntProperty {
                                    name: FName::from_slice("Souls/LifeAmount"),
                                    property_guid: None,
                                    duplication_index: 0,
                                    value: *amount,
                                },
                            )),
                        }
                    }
                    Drop::Duck => replace(18)?,
                }
                save(&mut map, &loc)?;
            }
            Context::Starting => {
                fn add_item(savegame: &mut Asset<std::fs::File>, drop: Drop) -> Result<(), Error> {
                    savegame.exports[1]
                        .get_normal_export_mut()
                        .and_then(|default| {
                            cast!(Property, StructProperty, &mut default.properties[3])
                        })
                        .and_then(|stats| {
                            cast!(
                                Property,
                                ArrayProperty,
                                &mut stats.value[match drop {
                                    Drop::Item(item, _) if item.is_key_item() => 7,
                                    _ => 6,
                                }]
                            )
                        })
                        .ok_or(Error::Assumption)?
                        .value
                        .push(unreal_asset::properties::Property::StructProperty(
                            unreal_asset::properties::struct_property::StructProperty {
                                name: FName::from_slice(match drop {
                                    Drop::Item(item, _) if item.is_key_item() => {
                                        "PassiveInventory_48_636C916F4A37F051CF9B14A1402B4C94"
                                    }
                                    _ => "Inventory_23_288399C5416269F828550FB7376E7942",
                                }),
                                struct_type: Some(FName::from_slice("Inventory")),
                                struct_guid: None,
                                property_guid: None,
                                duplication_index: 0,
                                serialize_none: true,
                                value: drop.as_shop_entry(0),
                            },
                        ));
                    Ok(())
                }
                let (mut savegame, loc) = get_savegame(app, &pak)?;
                match &drop {
                    Drop::Ability(ability) => {
                        add_item(&mut savegame, Drop::Item(ability.as_item(), 1))?;
                        savegame.exports[1]
                            .get_normal_export_mut()
                            .and_then(|default| {
                                cast!(Property, StructProperty, &mut default.properties[2])
                            })
                            .and_then(|abilities| {
                                cast!(
                                    Property,
                                    BoolProperty,
                                    &mut abilities.value[ability.savegame_index()]
                                )
                            })
                            .ok_or(Error::Assumption)?
                            .value = true;
                    }
                    Drop::Emote(emote) => {
                        let emotes = savegame.exports[1]
                            .get_normal_export_mut()
                            .and_then(|default| {
                                cast!(Property, ArrayProperty, &mut default.properties[15])
                            })
                            .ok_or(Error::Assumption)?;
                        emotes.value.push(byte_property(
                            &emotes.value.len().to_string(),
                            "E_Emotes",
                            emote.as_ref(),
                        ))
                    }
                    Drop::Ore(amount) => {
                        savegame.exports[1]
                            .get_normal_export_mut()
                            .and_then(|default| {
                                cast!(Property, StructProperty, &mut default.properties[3])
                            })
                            .and_then(|stats| cast!(Property, IntProperty, &mut stats.value[0]))
                            .ok_or(Error::Assumption)?
                            .value += *amount;
                    }
                    Drop::Duck => add_item(&mut savegame, Drop::Item(Items::Duck, 1))?,
                    _ => add_item(&mut savegame, drop)?,
                }
                save(&mut savegame, loc)?;
            }
        }
    }
    // clear out emote shop items
    let (mut savegame, loc) = get_savegame(app, &pak)?;
    let default = savegame.exports[1]
        .get_normal_export_mut()
        .ok_or(Error::Assumption)?;
    for (shopkeep, i) in shop_emotes {
        cast!(
            Property,
            ArrayProperty,
            &mut default.properties[shopkeep as usize]
        )
        .ok_or(Error::Assumption)?
        .value
        .remove(i);
    }
    save(&mut savegame, loc)?;
    // change the logo so people know it worked
    let logo_path = app
        .pak
        .join(MOD)
        .join("Blue Fire/Content/BlueFire/HUD/Menu/Blue-Fire-Logo.uasset");
    std::fs::create_dir_all(logo_path.parent().expect("is a file"))?;
    std::fs::write(&logo_path, include_bytes!("../blueprints/logo.uasset"))?;
    std::fs::write(
        logo_path.with_extension("uexp"),
        include_bytes!("../blueprints/logo.uexp"),
    )?;
    // package the mod in the most scuffed way possible
    std::fs::write("UnrealPak.exe", include_bytes!("../UnrealPak.exe"))?;
    std::fs::write("pak.bat", include_str!("../pak.bat"))?;
    // for some reason calling with rust doesn't work so a batch file will do
    std::process::Command::new("./pak.bat")
        .arg(app.pak.join("rando_p"))
        .output()?;
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
