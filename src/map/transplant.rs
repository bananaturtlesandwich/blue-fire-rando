use unreal_asset::{exports::*, reader::asset_trait::AssetTrait, types::*, *};

pub fn transplant<C: std::io::Seek + std::io::Read, D: std::io::Seek + std::io::Read>(
    index: usize,
    recipient: &mut Asset<C>,
    donor: &Asset<D>,
) {
    let mut children = super::get_actor_exports(index, donor, recipient.exports.len());

    // make sure the actor has a unique object name
    super::give_unique_name(
        &mut children[0].get_base_export_mut().object_name,
        recipient,
    );

    let actor_ref = PackageIndex::new(recipient.exports.len() as i32 + 1);
    // add the actor to persistent level
    if let Some((pos, level)) = recipient
        .exports
        .iter_mut()
        // least awkward way to get position and reference
        .enumerate()
        .find_map(|(i, ex)| cast!(Export, LevelExport, ex).map(|level| (i, level)))
    {
        // update actor's level reference
        let level_ref = PackageIndex::new(pos as i32 + 1);
        children[0].get_base_export_mut().outer_index = level_ref;
        children[0]
            .get_base_export_mut()
            .create_before_create_dependencies[0] = level_ref;
        // add actor to level data
        level.actors.push(actor_ref);
        level
            .get_base_export_mut()
            .create_before_serialization_dependencies
            .push(actor_ref);
    }

    // resolve all import references from exports
    let import_offset = recipient.imports.len() as i32;
    let mut imports = Vec::new();
    for child in children.iter_mut() {
        super::on_import_refs(child, |index| {
            if let Some(import) = donor.get_import(*index) {
                index.index = match recipient.find_import_no_index(
                    &import.class_package,
                    &import.class_name,
                    &import.object_name,
                ) {
                    Some(existing)
                        if donor.get_import(import.outer_index).map(|import| {
                            (
                                &import.class_package.content,
                                &import.class_name.content,
                                &import.object_name.content,
                            )
                        }) == recipient.get_import(PackageIndex::new(existing)).map(
                            |import| {
                                (
                                    &import.class_package.content,
                                    &import.class_name.content,
                                    &import.object_name.content,
                                )
                            },
                        ) =>
                    {
                        existing
                    }
                    _ => {
                        -import_offset
                            - match imports.iter().position(|imp: &Import| {
                                imp.class_package.content == import.class_package.content
                                    && imp.class_name.content == import.class_name.content
                                    && imp.object_name.content == import.object_name.content
                            }) {
                                Some(existing) => existing + 1,
                                None => {
                                    imports.push(import.clone());
                                    // this actually pads perfectly so no need for + 1
                                    imports.len()
                                }
                            } as i32
                    }
                }
            }
        })
    }
    // add export names
    for export in children.iter() {
        recipient.add_fname(&export.get_base_export().object_name.content);
        if let Some(norm) = export.get_normal_export() {
            for prop in norm.properties.iter() {
                super::add_prop_names(prop, recipient, false)
            }
        }
    }
    // finally add the exports
    recipient.exports.append(&mut children);

    // resolve all import references from exports
    let mut i = 0;
    // use a while loop because the vector is expanding while the operation occurs & imports.len() updates every loop
    while i < imports.len() {
        if let Some(parent) = donor.get_import(imports[i].outer_index) {
            imports[i].outer_index.index = match recipient.find_import_no_index(
                &parent.class_package,
                &parent.class_name,
                &parent.object_name,
            ) {
                Some(existing)
                    if donor.get_import(parent.outer_index).map(|import| {
                        (
                            &import.class_package.content,
                            &import.class_name.content,
                            &import.object_name.content,
                        )
                    }) == recipient
                        .get_import(PackageIndex::new(existing))
                        .map(|import| {
                            (
                                &import.class_package.content,
                                &import.class_name.content,
                                &import.object_name.content,
                            )
                        }) =>
                {
                    existing
                }
                _ => {
                    -import_offset
                        - match imports.iter().position(|import: &Import| {
                            import.class_package.content == parent.class_package.content
                                && import.class_name.content == parent.class_name.content
                                && import.object_name.content == parent.object_name.content
                        }) {
                            Some(existing) => existing + 1,
                            None => {
                                imports.push(parent.clone());
                                // this actually pads perfectly so no need for + 1
                                imports.len()
                            }
                        } as i32
                }
            }
        }
        i += 1;
    }
    // add import names
    for import in imports.iter() {
        recipient.add_fname(&import.class_package.content);
        recipient.add_fname(&import.class_name.content);
        recipient.add_fname(&import.object_name.content);
    }
    recipient.imports.append(&mut imports);
}
