use unreal_asset::{exports::*, properties::*, types::*, *};

mod delete;
pub use delete::delete;
mod transplant;
pub use transplant::transplant;
mod transform;
pub use transform::*;

/// gets all exports related to the given actor
fn get_actor_exports<C: std::io::Seek + std::io::Read>(
    index: usize,
    asset: &Asset<C>,
    offset: usize,
) -> Vec<Export> {
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
fn give_unique_name<C: std::io::Seek + std::io::Read>(orig: &mut FName, asset: &mut Asset<C>) {
    // for the cases where the number is unnecessary
    if asset.search_name_reference(&orig.content).is_none() {
        *orig = asset.add_fname(&orig.content);
        return;
    }
    let name = orig.content.as_str();
    let mut counter: u16 = match name.rfind(|ch: char| ch.to_digit(10).is_none()) {
        Some(index) if index != name.len() - 1 => name[index + 1..].parse().unwrap(),
        _ => 1,
    };
    while asset
        .search_name_reference(&format!("{name}{counter}"))
        .is_some()
    {
        counter += 1;
    }
    *orig = asset.add_fname(&format!("{name}{counter}"))
}

/// on all possible export references
fn on_export_refs(export: &mut Export, mut func: impl FnMut(&mut PackageIndex)) {
    if let Some(norm) = export.get_normal_export_mut() {
        for prop in norm.properties.iter_mut() {
            on_props(prop, &mut func);
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
            on_props(prop, &mut func);
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
fn on_props(prop: &mut Property, func: &mut impl FnMut(&mut PackageIndex)) {
    match prop {
        Property::ObjectProperty(obj) => {
            func(&mut obj.value);
        }
        Property::ArrayProperty(arr) => {
            for entry in arr.value.iter_mut() {
                on_props(entry, func);
            }
        }
        Property::MapProperty(map) => {
            for val in map.value.values_mut() {
                on_props(val, func);
            }
        }
        Property::SetProperty(set) => {
            for entry in set.value.value.iter_mut() {
                on_props(entry, func);
            }
            for entry in set.removed_items.value.iter_mut() {
                on_props(entry, func);
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
                on_props(entry, func);
            }
        }
        _ => (),
    }
}

fn add_prop_names<C: std::io::Read + std::io::Seek>(
    prop: &Property,
    asset: &mut Asset<C>,
    is_array: bool,
) {
    asset.add_fname(&prop.to_fname().content);
    // the name of properties in arrays is their index
    if !is_array {
        asset.add_fname(&prop.get_name().content);
    }
    match prop {
        Property::ByteProperty(prop) => {
            if let Some(en) = &prop.enum_type {
                asset.add_fname(&en.content);
            }
            if let unreal_asset::properties::int_property::BytePropertyValue::FName(name) =
                &prop.value
            {
                asset.add_fname(&name.content);
            }
        }
        Property::NameProperty(prop) => {
            asset.add_fname(&prop.value.content);
        }
        Property::TextProperty(prop) => {
            if let Some(id) = prop.table_id.as_ref() {
                asset.add_fname(&id.content);
            }
        }
        Property::SoftObjectProperty(prop) => {
            asset.add_fname(&prop.value.asset_path_name.content);
        }
        Property::SoftAssetPathProperty(prop) => {
            if let Some(path) = prop.asset_path_name.as_ref() {
                asset.add_fname(&path.content);
            }
        }
        Property::SoftObjectPathProperty(prop) => {
            if let Some(path) = prop.asset_path_name.as_ref() {
                asset.add_fname(&path.content);
            }
        }
        Property::SoftClassPathProperty(prop) => {
            if let Some(path) = prop.asset_path_name.as_ref() {
                asset.add_fname(&path.content);
            }
        }
        Property::DelegateProperty(del) => {
            asset.add_fname(&del.value.delegate.content);
        }
        Property::MulticastDelegateProperty(del) => {
            for delegate in del.value.iter() {
                asset.add_fname(&delegate.delegate.content);
            }
        }
        Property::MulticastSparseDelegateProperty(del) => {
            for delegate in del.value.iter() {
                asset.add_fname(&delegate.delegate.content);
            }
        }
        Property::MulticastInlineDelegateProperty(del) => {
            for delegate in del.value.iter() {
                asset.add_fname(&delegate.delegate.content);
            }
        }
        Property::SmartNameProperty(prop) => {
            asset.add_fname(&prop.display_name.content);
        }
        Property::StructProperty(prop) => {
            if let Some(typ) = prop.struct_type.as_ref() {
                asset.add_fname(&typ.content);
            }
            for prop in prop.value.iter() {
                add_prop_names(prop, asset, false);
            }
        }
        Property::ArrayProperty(prop) => {
            for prop in prop.value.iter() {
                add_prop_names(prop, asset, true);
            }
        }
        Property::EnumProperty(prop) => {
            asset.add_fname(&prop.value.content);
            if let Some(typ) = prop.enum_type.as_ref() {
                asset.add_fname(&typ.content);
            }
        }
        Property::UnknownProperty(prop) => {
            asset.add_fname(&prop.serialized_type.content);
        }
        Property::SetProperty(prop) => {
            for prop in prop.value.value.iter() {
                add_prop_names(prop, asset, true);
            }
            for prop in prop.removed_items.value.iter() {
                add_prop_names(prop, asset, true);
            }
        }
        Property::MapProperty(prop) => {
            for (_, key, value) in prop.value.iter() {
                add_prop_names(key, asset, false);
                add_prop_names(value, asset, false);
            }
        }
        Property::MaterialAttributesInputProperty(prop) => {
            asset.add_fname(&prop.material_expression.input_name.content);
            asset.add_fname(&prop.material_expression.expression_name.content);
        }
        Property::ExpressionInputProperty(prop) => {
            asset.add_fname(&prop.material_expression.input_name.content);
            asset.add_fname(&prop.material_expression.expression_name.content);
        }
        Property::ColorMaterialInputProperty(prop) => {
            asset.add_fname(&prop.material_expression.input_name.content);
            asset.add_fname(&prop.material_expression.expression_name.content);
        }
        Property::ScalarMaterialInputProperty(prop) => {
            asset.add_fname(&prop.material_expression.input_name.content);
            asset.add_fname(&prop.material_expression.expression_name.content);
        }
        Property::ShadingModelMaterialInputProperty(prop) => {
            asset.add_fname(&prop.material_expression.input_name.content);
            asset.add_fname(&prop.material_expression.expression_name.content);
        }
        Property::VectorMaterialInputProperty(prop) => {
            asset.add_fname(&prop.material_expression.input_name.content);
            asset.add_fname(&prop.material_expression.expression_name.content);
        }
        Property::Vector2MaterialInputProperty(prop) => {
            asset.add_fname(&prop.material_expression.input_name.content);
            asset.add_fname(&prop.material_expression.expression_name.content);
        }
        Property::StringAssetReferenceProperty(prop) => {
            if let Some(path) = &prop.asset_path_name {
                asset.add_fname(&path.content);
            }
        }
        Property::GameplayTagContainerProperty(prop) => {
            for name in prop.value.iter() {
                asset.add_fname(&name.content);
            }
        }
        Property::UniqueNetIdProperty(net) => {
            if let Some(id) = &net.value {
                asset.add_fname(&id.ty.content);
            }
        }
        Property::NiagaraVariableProperty(prop) => {
            for prop in prop.struct_property.value.iter() {
                add_prop_names(prop, asset, false);
            }
            asset.add_fname(&prop.variable_name.content);
        }
        Property::NiagaraVariableWithOffsetProperty(prop) => {
            for prop in prop.niagara_variable.struct_property.value.iter() {
                add_prop_names(prop, asset, false);
            }
            asset.add_fname(&prop.niagara_variable.variable_name.content);
        }
        _ => (),
    }
}
