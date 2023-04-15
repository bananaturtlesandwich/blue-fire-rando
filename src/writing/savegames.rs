use super::*;

pub fn write(checks: Vec<Check>, app: &crate::Rando, pak: &unpak::Pak) -> Result<(), Error> {
    let (mut savegame, savegame_loc) = extract(app, pak, SAVEGAME)?;
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
    let mut shop_emotes: Vec<_> = checks
        .iter()
        .filter_map(|check| {
            if let Context::Shop(keep, i, ..) = check.context {
                if matches!(check.drop, Drop::Emote(..) | Drop::Ability(..)) {
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
            Context::Shop(shop, index, price) => {
                savegame.exports[1]
                    .get_normal_export_mut()
                    .and_then(|norm| {
                        cast!(Property, ArrayProperty, &mut norm.properties[shop as usize])
                    })
                    .ok_or(Error::Assumption)?
                    .value[index] = Property::StructProperty(
                    unreal_asset::properties::struct_property::StructProperty {
                        name: FName::from_slice(shop.as_ref()),
                        struct_type: Some(FName::from_slice("Inventory")),
                        struct_guid: None,
                        property_guid: None,
                        duplication_index: 0,
                        serialize_none: true,
                        value: drop.as_shop_entry(price),
                    },
                );
                if matches!(drop, Drop::Emote(..) | Drop::Ability(..)) {
                    let (mut map, loc) = extract(app, &pak, &format!("{PREFIX}{location}.umap"))?;
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
                    save(&mut map, loc)?;
                }
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
                                    Drop::Item(item, ..) if item.key_item() => 7,
                                    _ => 6,
                                }]
                            )
                        })
                        .ok_or(Error::Assumption)?
                        .value
                        .push(unreal_asset::properties::Property::StructProperty(
                            unreal_asset::properties::struct_property::StructProperty {
                                name: FName::from_slice(match drop {
                                    Drop::Item(item, ..) if item.key_item() => {
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
            }
            _ => (),
        }
    }
    // clear out external shop items
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
    save(&mut savegame, savegame_loc)?;
    Ok(())
}
