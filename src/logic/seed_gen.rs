/*
Logic is iteratively evaluated:
loop {
    Possible drops list is shuffled
    Available checks are populated using the shuffled drop list
    If more locations/checks are unlocked then the check(s) that added them are marked as progression
    All the items not marked as progression are purged from the list
}
repeats until all locations are now accessible :)
After that we'll have a fully populated data set
The generated seed data can then be written appropriately

how to do this in code :/

definitions:
drop - the thing that you get which is randomised
check - a place where you can get a drop
location - a place where you can find checks

variables:
possible_drops = list of possible drops that gets shuffled each iteration
available_checks = list of available checks that get populated by possible_drops
available_locations = list of locations that can be queried for availability and then removed
progression = list of checks removed from checks that unlock more locations or checks
in the end...populate available_checks and append to progression then you have data that can be written
*/
use super::*;

const BEGINNING: &'static str = "A02_ArcaneTunnels/A02_GameIntro_KeepSouth";

pub fn randomise(app: &crate::Rando) {
    let (mut pool, unrandomised): (Vec<Check>, Vec<Check>) =
        CHECKS.into_iter().partition(|check| match &check.drop {
            Drop::Item(item, _) => match item.is_treasure() {
                true => app.treasure,
                false => app.item,
            },
            Drop::Weapon(_) => app.weapons,
            Drop::Tunic(_) => app.tunics,
            Drop::Spirit(_) => app.spirits,
            Drop::Ability(_) => app.abilities,
            Drop::Emote(_) => app.emotes,
            Drop::Ore(_) => app.ore,
            Drop::Duck => app.ducks,
        });
    let mut possible_drops: Vec<Drop> = pool.iter().map(|check| check.drop).collect();
    let mut rng = rand::thread_rng();
    loop {
        use rand::seq::SliceRandom;
        possible_drops.shuffle(&mut rng);
        // populate available checks
        for (i, check) in pool.iter_mut().enumerate() {
            check.drop = possible_drops[i];
        }
    }
}
