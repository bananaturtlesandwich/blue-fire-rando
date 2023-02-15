/*
Logic is iteratively evaluated:
Available checks are populated at random
If more locations/checks are unlocked then the check(s) that added them are marked as progression
All the items not marked as progression are purged from the list

This loop repeats until all locations are now accessible :)
After that we'll have a fully populated data set
The generated seed data can then be written appropriately
*/
use super::*;

const BEGINNING: &'static str = "A02_ArcaneTunnels/A02_GameIntro_KeepSouth";

pub fn randomise(app: &crate::Rando) {
    let available = vec![BEGINNING];
    // also going to use the closure for requirements
    let in_pool = |drop: &Drop| match &drop {
        Drop::Item(item, _) => match item.is_treasure() {
            true => app.treasure,
            false => app.item,
        },
        Drop::Weapon(_) => app.weapons,
        Drop::Tunic(_) => app.tunics,
        Drop::Spirit(_) => app.spirits,
        Drop::Life => app.lives,
        Drop::Ability(_) => app.abilities,
        Drop::Emote(_) => app.emotes,
        Drop::Ore(_) => app.ore,
        Drop::Duck => app.ducks,
    };
    let checks: Vec<_> = CHECKS
        .into_iter()
        .filter(|check| in_pool(&check.drop))
        .collect();
}
