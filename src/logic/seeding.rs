use super::*;
use strum::{EnumCount, IntoEnumIterator};

const NOTENOUGH: &str =
    "you haven't picked enough checks for anything to be random - include more checks in the pool";

fn update(
    locks: &[Lock],
    locations: &[Locations],
    possible: &mut Vec<Drop>,
    checks: &mut Vec<Check>,
    overworld: &mut std::collections::HashMap<Locations, Vec<Check>>,
    cutscenes: &mut Vec<Check>,
    savegames: &mut Vec<Check>,
    cases: &mut Vec<Check>,
) -> bool {
    let both = || {
        possible[0..checks.len()]
            .iter()
            .chain(overworld.values().flatten().map(|check| &check.drop))
            .chain(cutscenes.iter().map(|check| &check.drop))
            .chain(savegames.iter().map(|check| &check.drop))
            .chain(cases.iter().map(|check| &check.drop))
    };
    // see if there's any requirements met and what they are
    if !locks.iter().all(|lock| match lock {
        Lock::Location(loc) => locations.contains(loc),
        Lock::Movement(movement) => {
            let mut current = crate::no_walljump!(0, 0);
            for drop in both() {
                match drop {
                    Drop::Ability(Abilities::DoubleJump) | Drop::Ability(Abilities::SpinAttack) => {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Spirit(Spirits::HolyCentry)
                        if both().any(|drop| drop == &Drop::Ability(Abilities::DoubleJump)) =>
                    {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Spirit(Spirits::PossesedBook)
                        if both().any(|drop| drop == &Drop::Ability(Abilities::SpinAttack))
                            && both().any(|drop| drop == &Drop::Ability(Abilities::Dash)) =>
                    {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Spirit(Spirits::MoiTheDreadful)
                        if both().any(|drop| drop == &Drop::Ability(Abilities::SpinAttack)) =>
                    {
                        current.extra_height += 1;
                        current.horizontal += 1;
                    }
                    Drop::Ability(Abilities::Sprint) | Drop::Ability(Abilities::Spell) => {
                        current.horizontal += 1
                    }
                    Drop::Spirit(Spirits::FlyingOnop) | Drop::Ability(Abilities::Dash) => {
                        current.horizontal += 2
                    }
                    Drop::Spirit(Spirits::StormCentry)
                        if both().any(|drop| drop == &Drop::Ability(Abilities::Dash)) =>
                    {
                        current.horizontal += 2
                    }
                    Drop::Ability(Abilities::WallRun) => current.walljump = true,
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
        Lock::Mork => {
            both().fold(0, |acc, drop| {
                if drop == &Drop::Item(Items::Book, 1) {
                    acc + 1
                } else {
                    acc
                }
            }) == 5
        }
        Lock::SpiritHunter => {
            both().fold(0, |acc, drop| {
                if matches!(drop, Drop::Spirit(_)) {
                    acc + 1
                } else {
                    acc
                }
            }) >= 10
        }
        Lock::EvolairTunic => both().any(|drop| drop == &Drop::Tunic(Tunics::SteamWorkerTunic)),
        Lock::IronJustice => both().any(|drop| drop == &Drop::Weapon(Weapons::IronJustice)),
    }) {
        return false;
    }
    for lock in locks {
        // freeze any progression items where they are
        while let Some(i) = match lock {
            Lock::Location(_) => None,
            Lock::Movement(_) => possible[0..checks.len()].iter().position(|drop| {
                matches!(
                    drop,
                    Drop::Spirit(Spirits::PossesedBook)
                        | Drop::Spirit(Spirits::MoiTheDreadful)
                        | Drop::Spirit(Spirits::HolyCentry)
                        | Drop::Ability(Abilities::DoubleJump)
                        | Drop::Ability(Abilities::SpinAttack)
                        | Drop::Ability(Abilities::Sprint)
                        | Drop::Ability(Abilities::Spell)
                        | Drop::Ability(Abilities::Dash)
                        | Drop::Ability(Abilities::WallRun)
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
            Lock::Mork => possible[0..checks.len()]
                .iter()
                .position(|drop| drop == &Drop::Item(Items::Book, 1)),
            Lock::SpiritHunter => possible[0..checks.len()]
                .iter()
                .position(|drop| matches!(drop, Drop::Spirit(_))),
            Lock::EvolairTunic => possible[0..checks.len()]
                .iter()
                .position(|drop| drop == &Drop::Tunic(Tunics::SteamWorkerTunic)),
            Lock::IronJustice => possible[0..checks.len()]
                .iter()
                .position(|drop| drop == &Drop::Weapon(Weapons::IronJustice)),
        } {
            let mut check = checks.remove(i);
            check.drop = possible.remove(i);
            match check.context {
                Context::Shop(_, _, _) | Context::Starting => savegames.push(check),
                Context::Overworld(_) => match overworld.get_mut(&check.location) {
                    Some(checks) => checks.push(check),
                    None => {
                        overworld.insert(check.location.clone(), vec![check]);
                    }
                },
                Context::Cutscene(_) => cutscenes.push(check),
                Context::Specific(_, _) => cases.push(check),
            }
        }
    }
    true
}

pub fn randomise(app: &crate::Rando) -> Result<(), String> {
    let in_pool = |drop: &Drop| match drop {
        Drop::Item(item, _) => match item.is_treasure() {
            true => app.treasure,
            false => app.item,
        },
        Drop::Weapon(_) => app.weapons,
        Drop::Tunic(_) => app.tunics,
        Drop::Spirit(_) => app.spirits,
        Drop::Ability(ability) => match ability == &Abilities::Dash {
            true => app.dash,
            false => app.abilities,
        },
        Drop::Emote(_) => app.emotes,
        Drop::Ore(_) => app.ore,
        Drop::Duck => app.ducks,
    };
    let (mut pool, mut unrandomised): (Vec<Check>, Vec<Check>) =
        CHECKS.into_iter().partition(|check| in_pool(&check.drop));
    if pool.len() <= 1 {
        return Err(NOTENOUGH.to_string());
    }
    let mut possible: Vec<Drop> = pool.iter().map(|check| check.drop).collect();
    let mut checks: Vec<Check> = Vec::with_capacity(pool.len());
    let mut overworld = std::collections::HashMap::with_capacity(Locations::COUNT);
    let mut cutscenes = Vec::with_capacity(checks.len());
    let mut savegames = Vec::with_capacity(checks.len());
    let mut cases = Vec::with_capacity(checks.len());
    let mut locations = Vec::with_capacity(Locations::COUNT);
    let mut rng = rand::thread_rng();
    while locations.len() != Locations::COUNT && !pool.is_empty() {
        // shuffle the possible drops
        use rand::seq::SliceRandom;
        possible.shuffle(&mut rng);
        checks.shuffle(&mut rng);
        // update accessible locations
        for loc in Locations::iter() {
            if !locations.contains(&loc)
                && loc.locks().iter().any(|locks| {
                    update(
                        locks,
                        &locations,
                        &mut possible,
                        &mut checks,
                        &mut overworld,
                        &mut cutscenes,
                        &mut savegames,
                        &mut cases,
                    )
                })
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
                    &mut overworld,
                    &mut cutscenes,
                    &mut savegames,
                    &mut cases,
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
                    &mut overworld,
                    &mut cutscenes,
                    &mut savegames,
                    &mut cases,
                )
            {
                match unrandomised[i].context {
                    Context::Shop(_, _, _) | Context::Starting => {
                        savegames.push(unrandomised.remove(i))
                    }
                    Context::Overworld(_) => match overworld.get_mut(&unrandomised[i].location) {
                        Some(checks) => checks.push(unrandomised.remove(i)),
                        None => {
                            overworld.insert(
                                unrandomised[i].location.clone(),
                                vec![unrandomised.remove(i)],
                            );
                        }
                    },
                    Context::Cutscene(_) => cutscenes.push(unrandomised.remove(i)),
                    Context::Specific(_, _) => cases.push(unrandomised.remove(i)),
                }
            }
        }
    }
    for (check, drop) in checks.iter_mut().zip(possible.into_iter()) {
        check.drop = drop
    }
    for check in checks {
        match check.context {
            Context::Shop(_, _, _) | Context::Starting => savegames.push(check),
            Context::Overworld(_) => match overworld.get_mut(&check.location) {
                Some(checks) => checks.push(check),
                None => {
                    overworld.insert(check.location.clone(), vec![check]);
                }
            },
            Context::Cutscene(_) => cutscenes.push(check),
            Context::Specific(_, _) => cases.push(check),
        }
    }
    overworld = overworld
        .into_iter()
        .map(|(key, value)| {
            (
                key,
                value
                    .into_iter()
                    .filter(|check| in_pool(&check.drop))
                    .collect(),
            )
        })
        .collect();
    if overworld.is_empty() {
        return Err(NOTENOUGH.to_string());
    }
    std::fs::write("spoiler_log.txt", format!("{overworld:#?}")).unwrap_or_default();
    write(overworld, savegames, cutscenes, cases, app).map_err(|e| e.to_string())
}
