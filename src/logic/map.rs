use std::fs::File;
use unreal_asset::{
    exports::*, properties::Property, reader::asset_trait::AssetTrait, types::*, *,
};

/// delete an actor from a map
pub fn delete(index: usize, map: &mut Asset<std::fs::File>) {
    let val = PackageIndex::new(index as i32 + 1);
    if let Some(level) = map
        .exports
        .iter_mut()
        .find_map(|ex| cast!(Export, LevelExport, ex))
    {
        level
            .actors
            .remove(level.actors.iter().position(|i| i == &val).unwrap());
        let pos = level
            .get_base_export()
            .create_before_serialization_dependencies
            .iter()
            .position(|i| i == &val)
            .unwrap();
        level
            .get_base_export_mut()
            .create_before_serialization_dependencies
            .remove(pos);
    }
}

pub fn transplant(index: usize, recipient: &mut Asset<File>, donor: &Asset<File>) {
    let mut children = get_actor_exports(index, donor, recipient.exports.len());

    // make sure the actor has a unique object name
    give_unique_name(
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
        on_import_refs(child, |index| {
            if let Some(import) = donor.get_import(*index) {
                index.index = match recipient.find_import_no_index(
                    &import.class_package,
                    &import.class_name,
                    &import.object_name,
                ) {
                    Some(existing) => existing,
                    None => {
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
                Some(existing) => existing,
                None => {
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
    recipient.imports.append(&mut imports);
}

/// gets all exports related to the given actor
fn get_actor_exports(index: usize, asset: &Asset<File>, offset: usize) -> Vec<Export> {
    // get references to all the actor's children
    let mut child_indexes: Vec<PackageIndex> = asset.exports[index]
        .get_base_export()
        .create_before_serialization_dependencies
        .iter()
        .filter(|dep| dep.is_export())
        // dw PackageIndex is just a wrapper around i32 which is cloned by default anyway
        .cloned()
        .collect();
    // add the top-level actor reference
    child_indexes.insert(0, PackageIndex::new(index as i32 + 1));

    // get all the exports from those indexes
    let mut children: Vec<Export> = child_indexes
        .iter()
        .filter_map(|index| asset.get_export(*index))
        // i'm pretty sure i have to clone here so i can modify then insert data
        .cloned()
        .collect();

    let package_offset = (offset + 1) as i32;
    // update export references to what they will be once added
    for (i, child_index) in child_indexes.into_iter().enumerate() {
        for child in children.iter_mut() {
            on_export_refs(child, |index| {
                if index == &child_index {
                    index.index = package_offset + i as i32;
                }
            });
        }
    }
    children
}

/// creates and assigns a unique name
fn give_unique_name(orig: &mut FName, asset: &mut Asset<File>) {
    // for the cases where the number is unnecessary
    if asset.search_name_reference(&orig.content).is_none() {
        *orig = asset.add_fname(&orig.content);
        return;
    }
    let mut name = orig.content.clone();
    let mut id: u16 = match name.rfind(|ch: char| ch.to_digit(10).is_none()) {
        Some(index) if index != name.len() - 1 => {
            name.drain(index + 1..).collect::<String>().parse().unwrap()
        }
        _ => 1,
    };
    while asset
        .search_name_reference(&format!("{}{}", &name, id))
        .is_some()
    {
        id += 1;
    }
    *orig = asset.add_fname(&(name + &id.to_string()))
}

/// on all possible export references
fn on_export_refs(export: &mut Export, mut func: impl FnMut(&mut PackageIndex)) {
    if let Some(norm) = export.get_normal_export_mut() {
        for prop in norm.properties.iter_mut() {
            update_props(prop, &mut func);
        }
    }
    let export = export.get_base_export_mut();
    export
        .create_before_create_dependencies
        .iter_mut()
        .for_each(&mut func);
    export
        .create_before_serialization_dependencies
        .iter_mut()
        .for_each(&mut func);
    export
        .serialization_before_create_dependencies
        .iter_mut()
        .for_each(&mut func);
    func(&mut export.outer_index);
}

/// on all of an export's possible references to imports
fn on_import_refs(export: &mut Export, mut func: impl FnMut(&mut PackageIndex)) {
    if let Some(norm) = export.get_normal_export_mut() {
        for prop in norm.properties.iter_mut() {
            update_props(prop, &mut func);
        }
    }
    let export = export.get_base_export_mut();
    func(&mut export.class_index);
    func(&mut export.template_index);
    // not serialization_before_serialization because only the first few map exports have those
    export
        .serialization_before_create_dependencies
        .iter_mut()
        .for_each(&mut func);
    export
        .create_before_serialization_dependencies
        .iter_mut()
        .for_each(&mut func);
}

/// on any possible references stashed away in properties
fn update_props(prop: &mut Property, func: &mut impl FnMut(&mut PackageIndex)) {
    match prop {
        Property::ObjectProperty(obj) => {
            func(&mut obj.value);
        }
        Property::ArrayProperty(arr) => {
            for entry in arr.value.iter_mut() {
                update_props(entry, func);
            }
        }
        Property::MapProperty(map) => {
            for val in map.value.values_mut() {
                update_props(val, func);
            }
        }
        Property::SetProperty(set) => {
            for entry in set.value.value.iter_mut() {
                update_props(entry, func);
            }
            for entry in set.removed_items.value.iter_mut() {
                update_props(entry, func);
            }
        }
        Property::DelegateProperty(del) => func(&mut del.value.object),
        Property::MulticastDelegateProperty(del) => {
            for delegate in del.value.iter_mut() {
                func(&mut delegate.object)
            }
        }
        Property::MulticastSparseDelegateProperty(del) => {
            for delegate in del.value.iter_mut() {
                func(&mut delegate.object)
            }
        }
        Property::MulticastInlineDelegateProperty(del) => {
            for delegate in del.value.iter_mut() {
                func(&mut delegate.object)
            }
        }
        Property::StructProperty(struc) => {
            for entry in struc.value.iter_mut() {
                update_props(entry, func);
            }
        }
        _ => (),
    }
}
