/*
Logic is iteratively evaluated:
loop {
    Possible drops list is shuffled
    Available checks are populated using the shuffled drop list
    If more locations/checks are unlocked then the check(s) that added them are marked as progression
    All the items not marked as progression are purged from the list
}
repeats until all locations and checks are now accessible :)
*/
use super::*;

const BEGINNING: &'static str = "A02_ArcaneTunnels/A02_GameIntro_KeepSouth";

pub fn randomise(app: &crate::Rando) {
    let (mut pool, mut unrandomised): (Vec<Check>, Vec<Check>) =
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
    let mut possible: Vec<Drop> = pool.iter().map(|check| check.drop.clone()).collect();
    let mut checks: Vec<Check> = Vec::with_capacity(pool.len());
    let mut progression: Vec<Check> = Vec::with_capacity(pool.len());
    let mut locations = vec![BEGINNING];
    let mut rng = rand::thread_rng();
    while locations.len() != LOCATIONS.len() {
        // shuffle the possible drops
        use rand::seq::SliceRandom;
        possible.shuffle(&mut rng);
        // update accessible locations
        for i in 0..locations.len() {
            for loc in LOCATIONS[locations[i]].unlocks {
                if locations.contains(loc) {
                    continue;
                }
                // is there any drops currently unlocking a location?
                if let Some(req) = LOCATIONS[loc].requirements {
                    // see if there's any requirements met and what they are
                    let Some(fulfilled) = req.iter().find(|req| {
                        req.iter().fold(true, |acc, req| {
                            // don't need to check progression because that's already verified
                            acc && possible[0..checks.len()].contains(req)
                        })
                    }) else {continue};
                    for req in fulfilled.into_iter() {
                        // move all the progression items
                        let Some(i) = possible.iter().position(|drop| drop == req) else {continue};
                        let mut check = checks.remove(i);
                        check.drop = possible.remove(i);
                        progression.push(check);
                    }
                }
                locations.push(loc);
            }
        }
        // update accessible editable checks
        for i in (0..pool.len()).rev() {
            if locations.contains(&unrandomised[i].location) {
                if let Some(req) = unrandomised[i].requirements {
                    let Some(fulfilled) = req.iter().find(|req| {
                        req.iter().fold(true, |acc, req| {
                            // don't need to check progression because that's already verified
                            acc && possible[0..checks.len()].contains(req)
                        })
                    }) else {continue};
                    for req in fulfilled.into_iter() {
                        // move all the progression items
                        let Some(i) = possible.iter().position(|drop| drop == req) else {continue};
                        let mut check = checks.remove(i);
                        check.drop = possible.remove(i);
                        progression.push(check);
                    }
                }
                checks.push(pool.remove(i));
            }
        }
        // update progression with unrandomised
        for i in (0..unrandomised.len()).rev() {
            if locations.contains(&unrandomised[i].location) {
                if let Some(req) = unrandomised[i].requirements {
                    let Some(fulfilled) = req.iter().find(|req| {
                    req.iter().fold(true, |acc, req| {
                        // don't need to check progression because that's already verified
                        acc && possible[0..checks.len()].contains(req)
                    })
                }) else {continue};
                    for req in fulfilled.into_iter() {
                        // move all the progression items
                        let Some(i) = possible.iter().position(|drop| drop == req) else {continue};
                        let mut check = checks.remove(i);
                        check.drop = possible.remove(i);
                        progression.push(check);
                    }
                }
                checks.push(unrandomised.remove(i));
            }
        }
    }
    for (check, drop) in checks.iter_mut().zip(possible.into_iter()) {
        check.drop = drop
    }
    progression.append(&mut checks);
}
