use super::*;

pub fn write(
    checks: Vec<Check>,
    shop_emotes: Vec<(Shop, usize)>,
    app: &crate::Rando,
    pak: &unpak::Pak,
) -> Result<(), Error> {
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
    for Check { context, drop, .. } in checks {
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
