#![allow(dead_code)]
mod drops;
pub use drops::*;
mod seed_gen;
pub use seed_gen::randomise;

enum Shop {
    Mork = 7,
    SpiritHunter = 9,
    Ari = 10,
    Poti = 11,
    Poi = 12,
    Nilo = 19,
}

enum Context {
    Shop(Shop),
    Cutscene(&'static str),
    Overworld(&'static str),
}

#[derive(strum::AsRefStr)]
enum Drop {
    Item(Items),
    Weapon(Weapons),
    Tunic(Tunics),
    Spirit(Spirits),
    Life,
    SpiritSlot,
    Ability(Abilities),
    Emote(Emotes),
}

struct Check {
    context: Context,
    drop: Drop,
    requires: &'static [&'static [Drop]],
}

struct Location {
    unlocks: &'static [&'static str],
    requires: &'static [&'static [Drop]],
    checks: &'static [Check],
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

const PREFIX: &'static str = "/Game/BlueFire/Maps/World/";

lazy_static::lazy_static! {
    static ref DATA: hashbrown::HashMap<&'static str, Location> = hashmap![
        "A02_ArcaneTunnels/A02_GameIntro_KeepSouth" => Location {
            unlocks: &[
                "A02_ArcaneTunnels/A02_GameIntro_Exterior",
                "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
                "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
            ],
            requires: &[],
            checks: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_Exterior" => Location {
            unlocks: &[],
            requires: &[],
            checks: &[
                Check {
                    context: Context::Overworld("A01_FireKeep_EmoteStatue_Levitation"),
                    drop: Drop::Emote(Emotes::Levitation),
                    requires: &[],
                }
            ]
        },
        "A02_ArcaneTunnels/A02_GameIntro_KeepEast" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_EastWing"],
            requires: &[],
            checks: &[
                Check {
                    context: Context::Overworld("Chest_A01_TempleGardens_Ability_SpinAttack2"),
                    drop: Drop::Item(Items::SapphireOre),
                    requires: &[],
                },
                Check {
                    context: Context::Overworld("Chest_A02_Keep_Loot_02"),
                    drop: Drop::Item(Items::SapphireOre),
                    requires: &[],
                },
            ]
        },
        "A02_ArcaneTunnels/A02_GameIntro_EastWing" => Location {
            unlocks: &[],
            requires: &[],
            checks: &[
                Check {
                    context: Context::Overworld("Chest_A02_Keep_Key_01"),
                    drop: Drop::Item(Items::OldKey),
                    requires: &[],
                },
                Check {
                    context: Context::Overworld("Chest_A01_Keep_Shield"),
                    drop: Drop::Ability(Abilities::Block),
                    requires: &[],
                },
            ]
        },
        "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_KeepWest"],
            requires: &[&[Drop::Item(Items::OldKey)]],
            checks: &[
                Check {
                    context: Context::Overworld("A01_FireKeep_EmoteStatue_Techno"),
                    drop: Drop::Emote(Emotes::Techno),
                    requires: &[],
                },
                Check {
                    context: Context::Overworld("Spirit_A02_RiverSpirit"),
                    drop: Drop::Spirit(Spirits::RiverSpirit),
                    requires: &[
                        &[Drop::Ability(Abilities::Dash)],
                        &[Drop::Ability(Abilities::WallRun)],
                        &[
                            Drop::Ability(Abilities::DoubleJump),
                            Drop::Ability(Abilities::SpinAttack),
                        ],
                    ],
                },
            ]
        },
        "A02_ArcaneTunnels/A02_GameIntro_KeepWest" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_MemorialMain"],
            requires: &[],
            checks: &[
                Check {
                    context: Context::Overworld("Chest_A02_Keep_Loot_01"),
                    drop: Drop::Item(Items::SapphireOre),
                    requires: &[],
                }
            ]
        },
        "A02_ArcaneTunnels/A02_GameIntro_MemorialMain" => Location {
            unlocks: &[/* into arcane tunnels */],
            requires: &[],
            checks: &[
                Check {
                    context: Context::Overworld("Chest_A02_GameIntro"),
                    drop: Drop::Item(Items::EmeraldOre),
                    requires: &[],
                },
                Check {
                    context: Context::Overworld("Chest_A02_Sword_DiamondWings"),
                    drop: Drop::Weapon(Weapons::DiamondWings),
                    requires: &[],
                },
                Check {
                    context: Context::Overworld("Dance_Platform_Photo_Chest"),
                    drop: Drop::Item(Items::Mandoline),
                    requires: &[&[Drop::Emote(Emotes::Photo)]],
                },
            ]
        }
    ];
}
