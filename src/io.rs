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
    loop {
        match asset.write_data(
            &mut File::create(&path)?,
            File::create(path.as_ref().with_extension("uexp"))
                .ok()
                .as_mut(),
        ) {
            Ok(_) => break Ok(()),
            Err(e) if e.to_string().starts_with("name reference for ") => {
                asset.add_fname(
                    e.to_string()
                        .trim_start_matches("name reference for ")
                        .trim_end_matches(" not found"),
                );
            }
            e => break e,
        }
    }
}
