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

const BEGINNING: &str = "A02_ArcaneTunnels/A02_GameIntro_KeepSouth";

pub fn randomise(app: &mut crate::Rando) -> Result<(), String> {
    let in_pool = |check: &Check| match &check.drop {
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
    };
    let (mut pool, mut unrandomised): (Vec<Check>, Vec<Check>) =
        CHECKS.into_iter().partition(in_pool);
    if pool.len() <= 1 {
        return Err("you haven't picked enough checks for anything to be random - include more checks in the pool".to_string());
    }
    let mut possible: Vec<Drop> = pool.iter().map(|check| check.drop.clone()).collect();
    let mut checks: Vec<Check> = Vec::with_capacity(pool.len());
    let mut progression: Vec<Check> = Vec::with_capacity(pool.len());
    let mut locations = Vec::with_capacity(LOCATIONS.len());
    locations.push(BEGINNING);
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
                        req.iter().all(|req| possible[0..checks.len()].contains(req) || progression.iter().any(|check| &check.drop==req))
                    }) else {continue};
                    for req in fulfilled.iter() {
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
            if !locations.contains(&pool[i].location) {
                continue;
            }
            if let Some(req) = pool[i].requirements {
                let Some(fulfilled) = req.iter().find(|req| {
                    req.iter().all(|req| possible[0..checks.len()].contains(req) || progression.iter().any(|check| &check.drop==req))
                }) else {continue};
                for req in fulfilled.iter() {
                    // move all the progression items
                    let Some(i) = possible.iter().position(|drop| drop == req) else {continue};
                    let mut check = checks.remove(i);
                    check.drop = possible.remove(i);
                    progression.push(check);
                }
            }
            checks.push(pool.remove(i));
        }
        // update progression with unrandomised
        for i in (0..unrandomised.len()).rev() {
            if locations.contains(&unrandomised[i].location) {
                continue;
            }
            if let Some(req) = unrandomised[i].requirements {
                let Some(fulfilled) = req.iter().find(|req| {
                    req.iter().all(|req| possible[0..checks.len()].contains(req) || progression.iter().any(|check| &check.drop==req))
                }) else {continue};
                for req in fulfilled.iter() {
                    // move all the progression items
                    let Some(i) = possible.iter().position(|drop| drop == req) else {continue};
                    let mut check = checks.remove(i);
                    check.drop = possible.remove(i);
                    progression.push(check);
                }
            }
            progression.push(unrandomised.remove(i));
        }
    }
    for (check, drop) in checks.iter_mut().zip(possible.into_iter()) {
        check.drop = drop
    }
    progression.append(&mut checks);
    progression = progression.into_iter().filter(in_pool).collect();
    std::fs::write("spoiler_log.txt", format!("{progression:#?}")).unwrap_or_default();
    write(progression, app).map_err(|e| e.to_string())
}

macro_rules! hashmap {
    [$($key:literal => $value:expr), *] => ({
        let mut map = hashbrown::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    });
}

lazy_static::lazy_static! {
    static ref LOCATIONS: hashbrown::HashMap<&'static str, Location> = hashmap![
        "A02_ArcaneTunnels/A02_GameIntro_KeepSouth" => Location {
            unlocks: &[
                "A02_ArcaneTunnels/A02_GameIntro_Exterior",
                "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
                "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
            ],
            requirements: None,
        },
        "A02_ArcaneTunnels/A02_GameIntro_Exterior" => Location {
            unlocks: &[],
            requirements: None,
        },
        "A02_ArcaneTunnels/A02_GameIntro_KeepEast" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_EastWing"],
            requirements: None,
        },
        "A02_ArcaneTunnels/A02_GameIntro_EastWing" => Location {
            unlocks: &[],
            requirements: None,
        },
        "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_KeepWest"],
            requirements: Some(&[&[Drop::Item(Items::OldKey, 1)]]),
        },
        "A02_ArcaneTunnels/A02_GameIntro_KeepWest" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_MemorialMain"],
            requirements: None,
        },
        "A02_ArcaneTunnels/A02_GameIntro_MemorialMain" => Location {
            unlocks: &[/* into arcane tunnels */],
            requirements: None,
        }
    ];
}
