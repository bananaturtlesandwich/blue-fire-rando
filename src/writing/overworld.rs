use super::*;

pub fn write(
    checks: std::collections::HashMap<Locations, Vec<Check>>,
    app: &crate::Rando,
    pak: &unpak::Pak,
) -> Result<(), Error> {
    // reference so it isn't moved
    let used = &std::sync::Arc::new(std::sync::Mutex::new(Vec::with_capacity(checks.len())));
    std::thread::scope(|thread| -> Result<(), Error> {
        let mut threads = Vec::new();
        for (location, checks) in checks {
            threads.push(thread.spawn(move || -> Result<(), Error> {
                let (mut map, loc) = extract(app, pak, &format!("{PREFIX}{location}.umap"))?;
                for Check { context, drop, .. } in checks {
                    match context {
                        Context::Shop(shop, index, ..) => {
                            let insert = map.exports.len();
                            transplant(
                                match drop {
                                    Drop::Ability(..) => 36,
                                    Drop::Emote(..) => 20,
                                    _ => unimplemented!(),
                                },
                                &mut map,
                                &open_from_bytes(
                                    include_bytes!("../blueprints/collectibles.umap").as_slice(),
                                    include_bytes!("../blueprints/collectibles.uexp").as_slice(),
                                )?,
                            );
                            let mut pos = shop.location();
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
                                    Drop::Ability(..) => 11,
                                    Drop::Emote(..) => 6,
                                    _ => unimplemented!(),
                                }]
                            )
                            .ok_or(Error::Assumption)?
                            .value = Some(format!("{}{index}", shop.as_ref()));
                        }
                        Context::Overworld(name) => {
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
                                // unfortunately i can't share this between threads
                                let donor = open_from_bytes(
                                    include_bytes!("../blueprints/collectibles.umap"),
                                    include_bytes!("../blueprints/collectibles.uexp"),
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
                                        Locations::ForestDucks if name == "Duck" => {
                                            (0.0, 0.0, 800.0)
                                        }
                                        Locations::AbandonedPath if name == "Duck" => {
                                            (0.0, 0.0, 300.0)
                                        }
                                        Locations::Stoneheart if name == "Duck2" => {
                                            (0.0, -100.0, 0.0)
                                        }
                                        Locations::FirefallDucks | Locations::Sirion => {
                                            (0.0, 0.0, 100.0)
                                        }
                                        Locations::WaterwayDucks => (800.0, 0.0, 100.0),
                                        Locations::Queen => (500.0, -500.0, 0.0),
                                        _ => (0.0, 0.0, 0.0),
                                    },
                                );
                                let mut name = name.to_string();
                                // create unique id to prevent multiple checks being registered as collected
                                let mut counter: u16 = match name
                                    .rfind(|ch: char| ch.to_digit(10).is_none())
                                {
                                    Some(index) if index != name.len() - 1 => {
                                        name.drain(index + 1..).collect::<String>().parse().unwrap()
                                    }
                                    _ => 1,
                                };
                                while used.lock().unwrap().contains(&format!("{name}{counter}")) {
                                    counter += 1;
                                }
                                used.lock().unwrap().push(format!("{name}{counter}"));
                                let norm = &mut map.exports[insert]
                                    .get_normal_export_mut()
                                    .ok_or(Error::Assumption)?;
                                match norm.properties.iter_mut().find_map(|prop| {
                                    cast!(Property, StrProperty, prop)
                                        .filter(|id| id.name.content == "ID")
                                }) {
                                    Some(id) => id.value = Some(format!("{name}{counter}")),
                                    None => norm.properties.push(Property::StrProperty(
                                        str_property::StrProperty {
                                            name: FName::from_slice("ID"),
                                            property_guid: None,
                                            duplication_index: 0,
                                            value: Some(format!("{name}{counter}")),
                                        },
                                    )),
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
                                        Some(key_item) => key_item.value = item.key_item(),
                                        None if item.key_item() => chest.properties.push(
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
                                        cast!(Property, IntProperty, prop).filter(|amount| {
                                            amount.name.content == "Souls/LifeAmount"
                                        })
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
                        }
                        _ => (),
                    }
                }
                map.rebuild_name_map()?;
                save(&mut map, &loc)?;
                Ok(())
            }));
        }
        for thread in threads {
            thread.join().unwrap()?;
        }
        Ok(())
    })?;
    Ok(())
}
