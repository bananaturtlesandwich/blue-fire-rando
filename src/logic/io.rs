use std::fs::File;
use unreal_asset::{
    exports::{ExportBaseTrait, ExportNormalTrait},
    properties::Property,
};

pub fn open(
    file: impl AsRef<std::path::Path>,
) -> Result<unreal_asset::Asset<File>, unreal_asset::error::Error> {
    let mut asset = unreal_asset::Asset::new(
        File::open(&file)?,
        File::open(file.as_ref().with_extension("uexp")).ok(),
    );
    asset.set_engine_version(unreal_asset::engine_version::EngineVersion::VER_UE4_25);
    asset.parse_data()?;
    Ok(asset)
}

pub fn save<R: std::io::Seek + std::io::Read>(
    asset: &mut unreal_asset::Asset<R>,
    path: impl AsRef<std::path::Path>,
) -> Result<(), unreal_asset::error::Error> {
    update_names(asset);
    asset.write_data(
        &mut File::create(&path)?,
        File::create(path.as_ref().with_extension("uexp"))
            .ok()
            .as_mut(),
    )
}

/// so i don't have to deal with borrow checker when editing name properties
fn update_names<R: std::io::Seek + std::io::Read>(asset: &mut unreal_asset::Asset<R>) {
    for import in asset.imports.clone().iter() {
        asset.add_fname(&import.class_package.content);
        asset.add_fname(&import.class_name.content);
        asset.add_fname(&import.object_name.content);
    }
    for export in asset.exports.clone().iter() {
        asset.add_fname(&export.get_base_export().object_name.content);
        // resolve the rest of the name references
        if let Some(norm) = export.get_normal_export() {
            for prop in norm.properties.iter() {
                update_prop_name(prop, asset, false);
            }
        }
    }
}

fn update_prop_name<R: std::io::Seek + std::io::Read>(
    prop: &Property,
    asset: &mut unreal_asset::Asset<R>,
    is_array: bool,
) {
    use unreal_asset::properties::PropertyDataTrait;
    use unreal_asset::types::ToFName;
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
                update_prop_name(prop, asset, false);
            }
        }
        Property::ArrayProperty(prop) => {
            for prop in prop.value.iter() {
                update_prop_name(prop, asset, true);
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
                update_prop_name(prop, asset, true);
            }
            for prop in prop.removed_items.value.iter() {
                update_prop_name(prop, asset, true);
            }
        }
        Property::MapProperty(prop) => {
            for (_, key, value) in prop.value.iter() {
                update_prop_name(key, asset, false);
                update_prop_name(value, asset, false);
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
                update_prop_name(prop, asset, false);
            }
            asset.add_fname(&prop.variable_name.content);
        }
        Property::NiagaraVariableWithOffsetProperty(prop) => {
            for prop in prop.niagara_variable.struct_property.value.iter() {
                update_prop_name(prop, asset, false);
            }
            asset.add_fname(&prop.niagara_variable.variable_name.content);
        }
        _ => (),
    }
}
