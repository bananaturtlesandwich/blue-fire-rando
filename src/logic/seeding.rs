use super::*;
use strum::{EnumCount, IntoEnumIterator};

fn update(
    locks: &[Lock],
    locations: &[Location],
    possible: &mut Vec<Drop>,
    checks: &mut Vec<Check>,
    data: &mut Data,
) -> bool {
    let both = || {
        possible[0..checks.len()]
            .iter()
            .chain(data.overworld.values().flatten().map(|check| &check.drop))
            .chain(data.cutscenes.iter().map(|check| &check.drop))
            .chain(data.savegames.iter().map(|check| &check.drop))
            .chain(data.cases.iter().map(|check| &check.drop))
    };
    // see if there's any requirements met and what they are
    if !locks.iter().all(|lock| match lock {
        Lock::Location(loc) => locations.contains(loc),
        Lock::Movement(movement) => {
            let mut current = crate::no_walljump!(0, 0);
            for drop in both() {
                match drop {
                    Drop::Ability(Ability::DoubleJump) | Drop::Ability(Ability::SpinAttack) => {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Spirit(Spirit::HolyCentry)
                        if both().any(|drop| drop == &Drop::Ability(Ability::DoubleJump)) =>
                    {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Spirit(Spirit::PossesedBook)
                        if both().any(|drop| drop == &Drop::Ability(Ability::SpinAttack))
                            && both().any(|drop| drop == &Drop::Ability(Ability::Dash)) =>
                    {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Spirit(Spirit::MoiTheDreadful)
                        if both().any(|drop| drop == &Drop::Ability(Ability::SpinAttack)) =>
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
                        if both().any(|drop| drop == &Drop::Ability(Ability::Dash)) =>
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
            both().any(|prog| prog == &drop)
        }
        Lock::Emote(emote) => {
            let emote = Drop::Emote(*emote);
            both().any(|drop| drop == &emote)
        }
        Lock::Money(amount) => {
            if amount < &3000 {
                true
            } else if amount < &6000 {
                both().any(|drop| matches!(drop, Drop::Item(Item::SmallPouch, ..)))
            } else if amount < &10000 {
                both().any(|drop| {
                    matches!(
                        drop,
                        Drop::Item(Item::SmallPouch, ..) | Drop::Item(Item::LargePouch, ..)
                    )
                })
            } else {
                both().any(|drop| {
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
            both().fold(0, |acc, drop| {
                if drop == &Drop::Item(Item::Book, 1) {
                    acc + 1
                } else {
                    acc
                }
            }) == 5
        }
        Lock::SpiritHunter => {
            both().fold(0, |acc, drop| {
                if matches!(drop, Drop::Spirit(..)) {
                    acc + 1
                } else {
                    acc
                }
            }) >= 10
        }
        Lock::EvolairTunic => both().any(|drop| drop == &Drop::Tunic(Tunic::SteamWorkerTunic)),
        Lock::IronJustice => both().any(|drop| drop == &Drop::Weapon(Weapon::IronJustice)),
    }) {
        return false;
    }
    for lock in locks {
        // freeze any progression items where they are
        while let Some(i) = match lock {
            Lock::Location(..) => None,
            Lock::Movement(..) => possible[0..checks.len()].iter().position(|drop| {
                matches!(
                    drop,
                    Drop::Spirit(Spirit::PossesedBook)
                        | Drop::Spirit(Spirit::MoiTheDreadful)
                        | Drop::Spirit(Spirit::HolyCentry)
                        | Drop::Ability(Ability::DoubleJump)
                        | Drop::Ability(Ability::SpinAttack)
                        | Drop::Ability(Ability::Sprint)
                        | Drop::Ability(Ability::Spell)
                        | Drop::Ability(Ability::Dash)
                        | Drop::Ability(Ability::WallRun)
                )
            }),
            Lock::Item(item) => {
                let item = Drop::Item(*item, 1);
                possible[0..checks.len()]
                    .iter()
                    .position(|drop| drop == &item)
            }
            Lock::Emote(emote) => {
                let emote = Drop::Emote(*emote);
                possible[0..checks.len()]
                    .iter()
                    .position(|drop| drop == &emote)
            }
            Lock::Money(amount) => {
                if amount < &3000 {
                    None
                } else if amount < &6000 {
                    possible[0..checks.len()]
                        .iter()
                        .position(|drop| matches!(drop, Drop::Item(Item::SmallPouch, ..)))
                } else if amount < &10000 {
                    possible[0..checks.len()].iter().position(|drop| {
                        matches!(
                            drop,
                            Drop::Item(Item::SmallPouch, ..) | Drop::Item(Item::LargePouch, ..)
                        )
                    })
                } else {
                    possible[0..checks.len()].iter().position(|drop| {
                        matches!(
                            drop,
                            Drop::Item(Item::SmallPouch, ..)
                                | Drop::Item(Item::LargePouch, ..)
                                | Drop::Item(Item::ExtraLargePouch, ..)
                        )
                    })
                }
            }
            Lock::Mork => possible[0..checks.len()]
                .iter()
                .position(|drop| drop == &Drop::Item(Item::Book, 1)),
            Lock::SpiritHunter => possible[0..checks.len()]
                .iter()
                .position(|drop| matches!(drop, Drop::Spirit(..))),
            Lock::EvolairTunic => possible[0..checks.len()]
                .iter()
                .position(|drop| drop == &Drop::Tunic(Tunic::SteamWorkerTunic)),
            Lock::IronJustice => possible[0..checks.len()]
                .iter()
                .position(|drop| drop == &Drop::Weapon(Weapon::IronJustice)),
        } {
            let mut check = checks.remove(i);
            check.drop = possible.remove(i);
            push(check, data);
        }
    }
    true
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
    let (mut pool, mut unrandomised): (Vec<Check>, Vec<Check>) =
        CHECKS.into_iter().partition(in_pool);
    if pool.len() <= 1 {
        return Err("you haven't picked enough checks for anything to be random - include more checks in the pool".to_string());
    }
    let mut possible: Vec<Drop> = pool.iter().map(|check| check.drop).collect();
    let mut checks: Vec<Check> = Vec::with_capacity(pool.len());
    let mut data = Data {
        overworld: std::collections::HashMap::with_capacity(Location::COUNT),
        cutscenes: Vec::with_capacity(checks.len()),
        savegames: Vec::with_capacity(checks.len()),
        cases: Vec::with_capacity(checks.len()),
        shop_emotes: Vec::with_capacity(checks.len()),
    };
    let mut locations = Vec::with_capacity(Location::COUNT);
    let mut rng = rand::rng();
    while locations.len() != Location::COUNT && !pool.is_empty() {
        // shuffle the possible drops
        use rand::seq::SliceRandom;
        possible.shuffle(&mut rng);
        checks.shuffle(&mut rng);
        // update accessible locations
        for loc in Location::iter() {
            if !locations.contains(&loc)
                && loc
                    .locks()
                    .iter()
                    .any(|locks| update(locks, &locations, &mut possible, &mut checks, &mut data))
            {
                locations.push(loc);
            }
        }
        // update accessible editable checks
        for i in (0..pool.len()).rev() {
            if locations.contains(&pool[i].location)
                && update(
                    pool[i].locks,
                    &locations,
                    &mut possible,
                    &mut checks,
                    &mut data,
                )
            {
                checks.push(pool.remove(i));
            }
        }
        // update progression with unrandomised
        for i in (0..unrandomised.len()).rev() {
            if locations.contains(&unrandomised[i].location)
                && update(
                    unrandomised[i].locks,
                    &locations,
                    &mut possible,
                    &mut checks,
                    &mut data,
                )
            {
                push(unrandomised.remove(i), &mut data);
            }
        }
    }
    for (check, drop) in checks.iter_mut().zip(possible.into_iter()) {
        check.drop = drop
    }
    for check in checks {
        push(check, &mut data)
    }
    data.overworld = data
        .overworld
        .into_iter()
        .map(|(key, value)| (key, value.into_iter().filter(in_pool).collect()))
        .collect();
    data.savegames = data.savegames.into_iter().filter(in_pool).collect();
    data.cutscenes = data.cutscenes.into_iter().filter(in_pool).collect();
    data.cases = data.cases.into_iter().filter(in_pool).collect();
    // sort descending so removing in order doesn't mess up indexes
    data.shop_emotes
        .sort_unstable_by_key(|(_, i)| std::cmp::Reverse(*i));
    std::fs::write(
        "spoiler_log.txt",
        format!(
            "{:#?}\n{:#?}\n{:#?}\n{:#?}",
            data.overworld.values().flatten().collect::<Vec<_>>(),
            data.savegames,
            data.cutscenes,
            data.cases
        ),
    )
    .unwrap_or_default();
    crate::writing::write(data, app).map_err(|e| e.to_string())
}
