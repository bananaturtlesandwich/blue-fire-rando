use unreal_asset::{exports::*, types::*, *};

/// adds an actor to a map where the actor is already present
pub fn duplicate(index: usize, asset: &mut Asset<std::fs::File>) {
    let len = asset.exports.len();
    let mut children = super::get_actor_exports(index, asset, len);

    // make sure the actor has a unique object name
    super::give_unique_name(&mut children[0].get_base_export_mut().object_name, asset);

    let actor_ref = PackageIndex::new(len as i32 + 1);
    // add the actor to persistent level
    if let Some(level) = asset
        .exports
        .iter_mut()
        .find_map(|ex| cast!(Export, LevelExport, ex))
    {
        level.actors.push(actor_ref);
        level
            .get_base_export_mut()
            .create_before_serialization_dependencies
            .push(actor_ref);
    }

    // actually add the exports ;p
    asset.exports.append(&mut children);
}
