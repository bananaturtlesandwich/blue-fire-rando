use std::{fs::File, io::Cursor, path::Path};
use unreal_asset::{engine_version::EngineVersion::VER_UE4_25, error::Error, Asset};

pub fn open(file: impl AsRef<Path>) -> Result<Asset<File>, Error> {
    Ok(Asset::new(
        File::open(&file)?,
        File::open(file.as_ref().with_extension("uexp")).ok(),
        VER_UE4_25,
    )?)
}

pub fn open_from_bytes<'chain>(
    asset: &'chain [u8],
    bulk: &'chain [u8],
) -> Result<Asset<Cursor<&'chain [u8]>>, Error> {
    Ok(Asset::new(
        Cursor::new(asset),
        Some(Cursor::new(bulk)),
        VER_UE4_25,
    )?)
}

pub fn save<C: std::io::Read + std::io::Seek>(
    asset: &mut Asset<C>,
    path: impl AsRef<Path>,
) -> Result<(), Error> {
    register_names(asset);
    asset.write_data(
        &mut File::create(&path)?,
        Some(&mut File::create(path.as_ref().with_extension("uexp"))?),
    )
}

fn register<C: std::io::Read + std::io::Seek>(
    name: &String,
    asset: &Asset<C>,
    unregistered: &mut Vec<String>,
) {
    if asset.search_name_reference(name).is_none() {
        unregistered.push(name.clone());
    }
}

fn register_names<C: std::io::Read + std::io::Seek>(asset: &mut Asset<C>) {
    use unreal_asset::exports::{ExportBaseTrait, ExportNormalTrait};
    let mut unregistered = Vec::new();
    for import in asset.imports.iter() {
        register(&import.class_package.content, &asset, &mut unregistered);
        register(&import.class_name.content, &asset, &mut unregistered);
        register(&import.object_name.content, &asset, &mut unregistered);
    }
    for export in asset.exports.iter() {
        register(
            &export.get_base_export().object_name.content,
            &asset,
            &mut unregistered,
        );
        // resolve the rest of the name references
        if let Some(norm) = export.get_normal_export() {
            for prop in norm.properties.iter() {
                resolve_prop_name(prop, &asset, false, &mut unregistered);
            }
        }
    }
    // remove duplicate names
    unregistered.sort_unstable();
    unregistered.dedup();
    for name in unregistered {
        asset.add_name_reference(name, true);
    }
}

fn resolve_prop_name<C: std::io::Read + std::io::Seek>(
    prop: &unreal_asset::properties::Property,
    asset: &Asset<C>,
    is_array: bool,
    unregistered: &mut Vec<String>,
) {
    use unreal_asset::{
        properties::{Property, PropertyDataTrait},
        types::ToFName,
    };
    register(&prop.to_fname().content, asset, unregistered);
    // the name of properties in arrays is their index
    if !is_array {
        register(&prop.get_name().content, asset, unregistered);
    }
    match prop {
        Property::ByteProperty(prop) => {
            if let Some(en) = &prop.enum_type {
                register(&en.content, asset, unregistered);
            }
            if let unreal_asset::properties::int_property::BytePropertyValue::FName(name) =
                &prop.value
            {
                register(&name.content, asset, unregistered);
            }
        }
        Property::NameProperty(prop) => {
            register(&prop.value.content, asset, unregistered);
        }
        Property::TextProperty(prop) => {
            if let Some(id) = prop.table_id.as_ref() {
                register(&id.content, asset, unregistered);
            }
        }
        Property::SoftObjectProperty(prop) => {
            register(&prop.value.asset_path_name.content, asset, unregistered);
        }
        Property::SoftAssetPathProperty(prop) => {
            if let Some(path) = prop.asset_path_name.as_ref() {
                register(&path.content, asset, unregistered);
            }
        }
        Property::SoftObjectPathProperty(prop) => {
            if let Some(path) = prop.asset_path_name.as_ref() {
                register(&path.content, asset, unregistered);
            }
        }
        Property::SoftClassPathProperty(prop) => {
            if let Some(path) = prop.asset_path_name.as_ref() {
                register(&path.content, asset, unregistered);
            }
        }
        Property::DelegateProperty(del) => {
            register(&del.value.delegate.content, asset, unregistered);
        }
        Property::MulticastDelegateProperty(del) => {
            for delegate in del.value.iter() {
                register(&delegate.delegate.content, asset, unregistered);
            }
        }
        Property::MulticastSparseDelegateProperty(del) => {
            for delegate in del.value.iter() {
                register(&delegate.delegate.content, asset, unregistered);
            }
        }
        Property::MulticastInlineDelegateProperty(del) => {
            for delegate in del.value.iter() {
                register(&delegate.delegate.content, asset, unregistered);
            }
        }
        Property::SmartNameProperty(prop) => {
            register(&prop.display_name.content, asset, unregistered);
        }
        Property::StructProperty(prop) => {
            if let Some(typ) = prop.struct_type.as_ref() {
                register(&typ.content, asset, unregistered);
            }
            for prop in prop.value.iter() {
                resolve_prop_name(prop, asset, false, unregistered);
            }
        }
        Property::ArrayProperty(prop) => {
            for prop in prop.value.iter() {
                resolve_prop_name(prop, asset, true, unregistered);
            }
        }
        Property::EnumProperty(prop) => {
            register(&prop.value.content, asset, unregistered);
            if let Some(typ) = prop.enum_type.as_ref() {
                register(&typ.content, asset, unregistered);
            }
        }
        Property::UnknownProperty(prop) => {
            register(&prop.serialized_type.content, asset, unregistered);
        }
        Property::SetProperty(prop) => {
            for prop in prop.value.value.iter() {
                resolve_prop_name(prop, asset, true, unregistered);
            }
            for prop in prop.removed_items.value.iter() {
                resolve_prop_name(prop, asset, true, unregistered);
            }
        }
        Property::MapProperty(prop) => {
            for (_, key, value) in prop.value.iter() {
                resolve_prop_name(key, asset, false, unregistered);
                resolve_prop_name(value, asset, false, unregistered);
            }
        }
        Property::MaterialAttributesInputProperty(prop) => {
            register(
                &prop.material_expression.input_name.content,
                asset,
                unregistered,
            );
            register(
                &prop.material_expression.expression_name.content,
                asset,
                unregistered,
            );
        }
        Property::ExpressionInputProperty(prop) => {
            register(
                &prop.material_expression.input_name.content,
                asset,
                unregistered,
            );
            register(
                &prop.material_expression.expression_name.content,
                asset,
                unregistered,
            );
        }
        Property::ColorMaterialInputProperty(prop) => {
            register(
                &prop.material_expression.input_name.content,
                asset,
                unregistered,
            );
            register(
                &prop.material_expression.expression_name.content,
                asset,
                unregistered,
            );
        }
        Property::ScalarMaterialInputProperty(prop) => {
            register(
                &prop.material_expression.input_name.content,
                asset,
                unregistered,
            );
            register(
                &prop.material_expression.expression_name.content,
                asset,
                unregistered,
            );
        }
        Property::ShadingModelMaterialInputProperty(prop) => {
            register(
                &prop.material_expression.input_name.content,
                asset,
                unregistered,
            );
            register(
                &prop.material_expression.expression_name.content,
                asset,
                unregistered,
            );
        }
        Property::VectorMaterialInputProperty(prop) => {
            register(
                &prop.material_expression.input_name.content,
                asset,
                unregistered,
            );
            register(
                &prop.material_expression.expression_name.content,
                asset,
                unregistered,
            );
        }
        Property::Vector2MaterialInputProperty(prop) => {
            register(
                &prop.material_expression.input_name.content,
                asset,
                unregistered,
            );
            register(
                &prop.material_expression.expression_name.content,
                asset,
                unregistered,
            );
        }
        Property::StringAssetReferenceProperty(prop) => {
            if let Some(path) = &prop.asset_path_name {
                register(&path.content, asset, unregistered);
            }
        }
        Property::GameplayTagContainerProperty(prop) => {
            for name in prop.value.iter() {
                register(&name.content, asset, unregistered);
            }
        }
        Property::UniqueNetIdProperty(net) => {
            if let Some(id) = &net.value {
                register(&id.ty.content, asset, unregistered);
            }
        }
        Property::NiagaraVariableProperty(prop) => {
            for prop in prop.struct_property.value.iter() {
                resolve_prop_name(prop, asset, false, unregistered);
            }
            register(&prop.variable_name.content, asset, unregistered);
        }
        Property::NiagaraVariableWithOffsetProperty(prop) => {
            for prop in prop.niagara_variable.struct_property.value.iter() {
                resolve_prop_name(prop, asset, false, unregistered);
            }
            register(
                &prop.niagara_variable.variable_name.content,
                asset,
                unregistered,
            );
        }
        _ => (),
    }
}
