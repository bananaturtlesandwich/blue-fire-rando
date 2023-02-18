use unreal_asset::{
    exports::*,
    properties::{struct_property::StructProperty, vector_property::VectorProperty, *},
    types::{vector::Vector, FName},
    *,
};

const DEFAULT: Vector<f32> = Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub fn get_location(index: usize, asset: &Asset<std::fs::File>) -> Vector<f32> {
    let Some(transform) = get_transform_index(index, asset) else {
        return DEFAULT
    };
    asset.exports[transform]
        .get_normal_export()
        .and_then(|norm| {
            norm.properties.iter().rev().find_map(|prop| {
                if let Property::StructProperty(struc) = prop {
                    if &struc.name.content == "RelativeLocation" {
                        if let Property::VectorProperty(vec) = &struc.value[0] {
                            return Some(Vector {
                                x: vec.value.x.0,
                                y: vec.value.y.0,
                                z: vec.value.z.0,
                            });
                        }
                    }
                }
                None
            })
        })
        .unwrap_or(DEFAULT)
}

pub fn set_location(index: usize, asset: &mut Asset<std::fs::File>, new: Vector<f32>) {
    let Some(transform) = get_transform_index(index, asset) else {
        return
    };
    let Some(norm) = asset.exports[transform].get_normal_export_mut() else {
        return
    };
    match norm
        .properties
        .iter_mut()
        .find(|prop| prop.get_name().content == "RelativeLocation")
    {
        Some(scale) => {
            if let Property::StructProperty(struc) = scale {
                if let Property::VectorProperty(vec) = &mut struc.value[0] {
                    vec.value.x.0 = new.x;
                    vec.value.y.0 = new.y;
                    vec.value.z.0 = new.z;
                }
            }
        }
        None => norm
            .properties
            .push(Property::StructProperty(StructProperty {
                name: FName::from_slice("RelativeLocation"),
                struct_type: Some(FName::from_slice("Vector")),
                struct_guid: None,
                property_guid: None,
                duplication_index: 0,
                serialize_none: true,
                value: vec![Property::VectorProperty(VectorProperty {
                    name: FName::from_slice("RelativeLocation"),
                    property_guid: None,
                    duplication_index: 0,
                    value: Vector::new(new.x.into(), new.y.into(), new.z.into()),
                })],
            })),
    }
}

fn get_transform_index(index: usize, asset: &Asset<std::fs::File>) -> Option<usize> {
    asset.exports[index].get_normal_export().and_then(|norm| {
        // normally these are further back so reversed should be a bit faster
        norm.properties.iter().rev().find_map(|prop| {
            match prop.get_name().content.as_str() {
                // of course this wouldn't be able to be detected if all transforms were left default
                "RelativeLocation" | "RelativeRotation" | "RelativeScale3D" => Some(index),
                "RootComponent" => {
                    if let Property::ObjectProperty(obj) = prop {
                        if obj.value.is_export() {
                            return Some(obj.value.index as usize - 1);
                        }
                    }
                    None
                }
                _ => None,
            }
        })
    })
}
