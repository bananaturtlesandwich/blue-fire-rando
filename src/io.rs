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

pub fn save<R: std::io::Seek + std::io::Read>(
    asset: &mut Asset<R>,
    path: impl AsRef<Path>,
) -> Result<(), Error> {
    loop {
        match asset.write_data(
            &mut File::create(&path)?,
            Some(&mut File::create(path.as_ref().with_extension("uexp"))?),
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
