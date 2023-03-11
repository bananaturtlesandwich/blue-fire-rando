use super::*;

const BEGINNING: &str = "A02_ArcaneTunnels/A02_GameIntro_KeepSouth";

const NOTENOUGH: &str =
    "you haven't picked enough checks for anything to be random - include more checks in the pool";

fn update(
    locks: &[Lock],
    locations: &[&str],
    possible: &mut Vec<Drop>,
    checks: &mut Vec<Check>,
    progression: &mut Vec<Check>,
) -> bool {
    let both = || {
        possible[0..checks.len()]
            .iter()
            .chain(progression.iter().map(|check| &check.drop))
    };
    // see if there's any requirements met and what they are
    if !locks.iter().all(|lock| match lock {
        Lock::Location(loc) => locations.contains(loc),
        Lock::Movement(movement) => {
            let mut current = Move::no_walljump(0, 0);
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
            possible[0..checks.len()].contains(&drop)
                || item.is_key_item() && progression.iter().any(|check| &check.drop == &drop)
        }
        Lock::Emote(emote) => {
            let emote = Drop::Emote(*emote);
            both().any(|drop| drop == &emote)
        }
    }) {
        return false;
    }
    for lock in locks.iter() {
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
        } {
            let mut check = checks.remove(i);
            check.drop = possible.remove(i);
            progression.push(check);
        }
    }
    true
}

pub fn randomise(app: &crate::Rando) -> Result<(), String> {
    let in_pool = |check: &Check| match &check.drop {
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
        CHECKS.into_iter().partition(in_pool);
    if pool.len() <= 1 {
        return Err(NOTENOUGH.to_string());
    }
    let mut possible: Vec<Drop> = pool.iter().map(|check| check.drop).collect();
    let mut checks: Vec<Check> = Vec::with_capacity(pool.len());
    let mut progression: Vec<Check> = Vec::with_capacity(pool.len());
    let mut locations = Vec::with_capacity(LOCATIONS.len());
    locations.push(BEGINNING);
    let mut rng = rand::thread_rng();
    while locations.len() != LOCATIONS.len() {
        // shuffle the possible drops
        use rand::seq::SliceRandom;
        possible.shuffle(&mut rng);
        checks.shuffle(&mut rng);
        // update accessible locations
        for i in 0..locations.len() {
            for loc in LOCATIONS[locations[i]].unlocks {
                if !locations.contains(loc)
                    && update(
                        &LOCATIONS[loc].locks,
                        &locations,
                        &mut possible,
                        &mut checks,
                        &mut progression,
                    )
                {
                    locations.push(loc);
                }
            }
        }
        // update accessible editable checks
        for i in (0..pool.len()).rev() {
            if locations.contains(&pool[i].location)
                && update(
                    &pool[i].locks,
                    &locations,
                    &mut possible,
                    &mut checks,
                    &mut progression,
                )
            {
                checks.push(pool.remove(i));
            }
        }
        // update progression with unrandomised
        for i in (0..unrandomised.len()).rev() {
            if locations.contains(&unrandomised[i].location)
                && update(
                    &unrandomised[i].locks,
                    &locations,
                    &mut possible,
                    &mut checks,
                    &mut progression,
                )
            {
                progression.push(unrandomised.remove(i));
            }
        }
    }
    for (check, drop) in checks.iter_mut().zip(possible.into_iter()) {
        check.drop = drop
    }
    progression.append(&mut checks);
    progression = progression.into_iter().filter(in_pool).collect();
    if progression.is_empty() {
        return Err(NOTENOUGH.to_string());
    }
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
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_Exterior" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro"],
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro" => Location {
            unlocks: &[],
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_KeepEast" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_EastWing"],
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_EastWing" => Location {
            unlocks: &[],
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_KeepWest"],
            locks: &[Lock::Item(Items::OldKey)],
        },
        "A02_ArcaneTunnels/A02_GameIntro_KeepWest" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_MemorialMain"],
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_MemorialMain" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_NorthArcane"],
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_NorthArcane" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_SouthArcane"],
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_SouthArcane" => Location {
            unlocks: &[
                "A02_ArcaneTunnels/A02_EastArcane",
                "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess"
            ],
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_EastArcane" => Location {
            unlocks: &[
                "A02_ArcaneTunnels/A02_Arcane",
                "A01_StoneHeartCity/A01_CrossRoads"
            ],
            locks: &[],
        },
        "A02_ArcaneTunnels/A02_Arcane" => Location {
            unlocks: &[],
            locks: &[],
        },
        "A01_StoneHeartCity/A01_CrossRoads" => Location {
            unlocks: &["A01_StoneHeartCity/A01_Well","A01_StoneHeartCity/A01_CliffPath"],
            locks: &[],
        },
        "A01_StoneHeartCity/A01_Well" => Location {
            unlocks: &[],
            locks: &[Lock::Movement(&[Move{
                extra_height: 0,
                horizontal: 1,
                walljump: false
            }])],
        },
        "A01_StoneHeartCity/A01_CliffPath" => Location {
            unlocks: &["A01_StoneHeartCity/A01_AbilityShrine_WaterLevels"],
            locks: &[],
        },
        "A01_StoneHeartCity/A01_AbilityShrine_WaterLevels" => Location {
            unlocks: &["A01_StoneHeartCity/A01_AbilityShrine_AmbushZone", "A01_StoneHeartCity/A01_AbilityShrine_CenterTree"],
            locks: &[]
        },
        "A01_StoneHeartCity/A01_AbilityShrine_AmbushZone" => Location {
            unlocks: &[],
            locks: &[Lock::Item(Items::OldKey)]
        },
        "A01_StoneHeartCity/A01_AbilityShrine_CenterTree" => Location {
            unlocks: &["A01_StoneHeartCity/A01_AbilityShrine", "A01_StoneHeartCity/A01_AbilityShrine_BossRoom"],
            locks: &[Lock::Movement(&[
                Move {
                    extra_height: 0,
                    horizontal: 1,
                    walljump: true
                },
                Move {
                    extra_height: 2,
                    horizontal: 0,
                    walljump: false
                }
            ])]
        },
        "A01_StoneHeartCity/A01_AbilityShrine" => Location {
            unlocks: &[],
            locks: &[]
        },
        "A01_StoneHeartCity/A01_AbilityShrine_BossRoom" => Location {
            unlocks: &[],
            locks: &[Lock::Item(Items::KeyHolyMaster)]
        },
        "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess" => Location {
            unlocks: &[],
            locks: &[],
        }
    ];
}
