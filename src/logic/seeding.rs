use super::*;
use strum::{EnumCount, IntoEnumIterator};

fn accessible(locks: &[Lock], locations: &[Location], obtainable: &[Drop]) -> bool {
    locks.iter().all(|lock| match lock {
        Lock::Location(loc) => locations.contains(loc),
        Lock::Movement(movement) => {
            let mut current = crate::no_walljump!(0, 0);
            for drop in obtainable {
                match drop {
                    Drop::Ability(Ability::DoubleJump) | Drop::Ability(Ability::SpinAttack) => {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Spirit(Spirit::HolyCentry)
                        if obtainable
                            .iter()
                            .any(|drop| drop == &Drop::Ability(Ability::DoubleJump)) =>
                    {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Spirit(Spirit::PossesedBook)
                        if obtainable
                            .iter()
                            .any(|drop| drop == &Drop::Ability(Ability::SpinAttack))
                            && obtainable
                                .iter()
                                .any(|drop| drop == &Drop::Ability(Ability::Dash)) =>
                    {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Spirit(Spirit::MoiTheDreadful)
                        if obtainable
                            .iter()
                            .any(|drop| drop == &Drop::Ability(Ability::SpinAttack)) =>
                    {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Ability(Ability::Sprint) | Drop::Ability(Ability::Spell) => {
                        current.horizontal += 1
                    }
                    Drop::Spirit(Spirit::FlyingOnop) | Drop::Ability(Ability::Dash) => {
                        current.horizontal += 2
                    }
                    Drop::Spirit(Spirit::StormCentry)
                        if obtainable
                            .iter()
                            .any(|drop| drop == &Drop::Ability(Ability::Dash)) =>
                    {
                        current.horizontal += 2
                    }
                    Drop::Ability(Ability::WallRun) => current.walljump = true,
                    _ => (),
                }
            }
            movement.iter().any(|moves| &current >= moves)
        }
        Lock::Item(item) => {
            let drop = Drop::Item(*item, 1);
            obtainable.iter().any(|prog| prog == &drop)
        }
        Lock::Emote(emote) => {
            let emote = Drop::Emote(*emote);
            obtainable.iter().any(|drop| drop == &emote)
        }
        Lock::Money(amount) => {
            if amount < &3000 {
                true
            } else if amount < &6000 {
                obtainable
                    .iter()
                    .any(|drop| matches!(drop, Drop::Item(Item::SmallPouch, ..)))
            } else if amount < &10000 {
                obtainable.iter().any(|drop| {
                    matches!(
                        drop,
                        Drop::Item(Item::SmallPouch, ..) | Drop::Item(Item::LargePouch, ..)
                    )
                })
            } else {
                obtainable.iter().any(|drop| {
                    matches!(
                        drop,
                        Drop::Item(Item::SmallPouch, ..)
                            | Drop::Item(Item::LargePouch, ..)
                            | Drop::Item(Item::ExtraLargePouch, ..)
                    )
                })
            }
        }
        Lock::Mork => {
            obtainable.iter().fold(0, |acc, drop| {
                if drop == &Drop::Item(Item::Book, 1) {
                    acc + 1
                } else {
                    acc
                }
            }) == 5
        }
        Lock::SpiritHunter => {
            obtainable.iter().fold(0, |acc, drop| {
                if matches!(drop, Drop::Spirit(..)) {
                    acc + 1
                } else {
                    acc
                }
            }) >= 10
        }
        Lock::EvolairTunic => obtainable
            .iter()
            .any(|drop| drop == &Drop::Tunic(Tunic::SteamWorkerTunic)),
        Lock::IronJustice => obtainable
            .iter()
            .any(|drop| drop == &Drop::Weapon(Weapon::IronJustice)),
    })
}

fn possible(spawn: Location, checks: &[Check], app: &crate::Rando) -> Option<String> {
    let mut seeding = String::new();
    let mut checks = checks.to_vec();
    let mut locations: Vec<Location> = Vec::with_capacity(Location::COUNT);
    locations.push(spawn);
    let mut locations_len = 1;
    let mut obtainable = Vec::with_capacity(checks.len());
    let mut obtainable_len = 0;
    loop {
        for loc in Location::iter() {
            if !locations.contains(&loc)
                && loc
                    .locks()
                    .iter()
                    .any(|lock| accessible(&lock, &locations, &obtainable))
            {
                seeding.push_str(&format!("{loc:?}\n"));
                locations.push(loc)
            }
        }
        let slated: Vec<_> = checks
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(i, check)| {
                (locations.contains(&check.location)
                    && accessible(&check.locks, &locations, &obtainable))
                .then_some(i)
            })
            .collect();
        for i in slated {
            seeding.push_str(&format!("{:?}\n", checks[i]));
            obtainable.push(checks.remove(i).drop)
        }
        // if all locations accessible then possible
        if locations.len() == Location::COUNT {
            break Some(seeding);
        }
        // if no change in location or drop count then impossible
        if locations.len() == locations_len && obtainable.len() == obtainable_len {
            break None;
        }

        locations_len = locations.len();
        obtainable_len = obtainable.len();
        seeding.push('\n');
    }
}

fn push(check: Check, data: &mut Data) {
    match check.context {
        Context::Shop(shop, index, ..)
            if matches!(check.drop, Drop::Ability(..) | Drop::Emote(..)) =>
        {
            data.shop_emotes.push((shop, index));
            match data.overworld.get_mut(&check.location) {
                Some(checks) => checks.push(check),
                None => {
                    data.overworld.insert(check.location.clone(), vec![check]);
                }
            }
        }
        Context::Overworld(..) => match data.overworld.get_mut(&check.location) {
            Some(checks) => checks.push(check),
            None => {
                data.overworld.insert(check.location.clone(), vec![check]);
            }
        },
        Context::Shop(..) | Context::Starting => data.savegames.push(check),
        Context::Cutscene(..) => data.cutscenes.push(check),
        Context::Specific(..) => data.cases.push(check),
    }
}

pub fn randomise(app: &crate::Rando) -> Result<(), String> {
    let in_pool = |check: &Check| match &check.drop {
        Drop::Item(item, ..) => match item.gem() {
            true => app.gems,
            false => match item.treasure() {
                true => app.treasure,
                false => match item.key() {
                    true => app.keys,
                    false => app.items,
                },
            },
        },
        Drop::Weapon(..) => app.weapons,
        Drop::Tunic(..) => app.tunics,
        Drop::Spirit(..) => app.spirits,
        Drop::Ability(ability) => match ability == &Ability::Dash {
            true => app.dash,
            false => app.abilities,
        },
        Drop::Emote(..) => app.emotes,
        Drop::Ore(..) => app.ore,
        Drop::Duck => app.ducks,
    };
    let (pool, unrandomised): (Vec<_>, Vec<_>) = CHECKS.into_iter().partition(in_pool);
    if pool.len() <= 1 {
        return Err("you haven't picked enough checks for anything to be random - include more checks in the pool".to_string());
    }

    use rand::seq::SliceRandom;
    let mut rng = rand::rng();
    let (seeding, data) = loop {
        let mut checks = pool.clone();
        let mut drops: Vec<Drop> = checks.iter().map(|check| check.drop).collect();
        drops.shuffle(&mut rng);
        for (check, drop) in checks.iter_mut().zip(drops.into_iter()) {
            check.drop = drop
        }
        let mut full = checks.clone();
        full.extend_from_slice(&unrandomised);
        if let Some(seeding) = possible(Location::Lab, &full, app) {
            let mut data = Data {
                overworld: std::collections::HashMap::with_capacity(Location::COUNT),
                cutscenes: Vec::with_capacity(checks.len()),
                savegames: Vec::with_capacity(checks.len()),
                cases: Vec::with_capacity(checks.len()),
                shop_emotes: Vec::with_capacity(checks.len()),
            };
            // sort descending so removing in order doesn't mess up indexes
            data.shop_emotes
                .sort_unstable_by_key(|(_, i)| std::cmp::Reverse(*i));
            for check in checks {
                push(check, &mut data)
            }
            break (seeding, data);
        }
    };
    std::fs::write("seeding.log", seeding).map_err(|e| e.to_string())?;
    std::fs::write("spoiler.log", format!("{:?}", data)).map_err(|e| e.to_string())?;
    crate::writing::write(data, app).map_err(|e| e.to_string())
}
