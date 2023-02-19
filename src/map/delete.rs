use unreal_asset::{exports::*, types::*, *};

/// delete an actor from a map
pub fn delete<C: std::io::Read + std::io::Seek>(index: usize, map: &mut Asset<C>) {
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
