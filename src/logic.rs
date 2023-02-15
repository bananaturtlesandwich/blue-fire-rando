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

#[derive(PartialEq, strum::AsRefStr)]
enum Drop {
    Item(Items, u8),
    Weapon(Weapons),
    Tunic(Tunics),
    Spirit(Spirits),
    Life,
    Ability(Abilities),
    Emote(Emotes),
    Ore(u16),
    Duck,
}

struct Check {
    context: Context,
    drop: Drop,
    requirements: &'static [&'static [Drop]],
}

struct Location {
    unlocks: &'static [&'static str],
    requirements: &'static [&'static [Drop]],
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

const CHECKS: [Check; 12] = [
    // A02_ArcaneTunnels/A02_GameIntro_Exterior
    Check {
        context: Context::Overworld("A01_FireKeep_EmoteStatue_Levitation"),
        drop: Drop::Emote(Emotes::Levitation),
        requirements: &[],
    },
    Check {
        context: Context::Cutscene(
            "/Game/BlueFire/NPC/Onops/MUSIC_Onops/Onop_Musicians/NPC_Onop_IO_Bech",
        ),
        drop: Drop::Ore(500),
        requirements: &[&[Drop::Item(Items::ComposerLetter, 1)]],
    },
    // A02_ArcaneTunnels/A02_GameIntro_KeepEast
    Check {
        context: Context::Overworld("Chest_A01_TempleGardens_Ability_SpinAttack2"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: &[],
    },
    Check {
        context: Context::Overworld("Chest_A02_Keep_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: &[],
    },
    // A02_ArcaneTunnels/A02_GameIntro_EastWing
    Check {
        context: Context::Overworld("Chest_A02_Keep_Key_01"),
        drop: Drop::Item(Items::OldKey, 1),
        requirements: &[],
    },
    Check {
        context: Context::Overworld("Chest_A01_Keep_Shield"),
        drop: Drop::Ability(Abilities::Block),
        requirements: &[],
    },
    // A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom
    Check {
        context: Context::Overworld("A01_FireKeep_EmoteStatue_Techno"),
        drop: Drop::Emote(Emotes::Techno),
        requirements: &[],
    },
    Check {
        context: Context::Overworld("Spirit_A02_RiverSpirit"),
        drop: Drop::Spirit(Spirits::RiverSpirit),
        requirements: &[
            &[Drop::Ability(Abilities::Dash)],
            &[Drop::Ability(Abilities::WallRun)],
            &[
                Drop::Ability(Abilities::DoubleJump),
                Drop::Ability(Abilities::SpinAttack),
            ],
        ],
    },
    // A02_ArcaneTunnels/A02_GameIntro_KeepWest
    Check {
        context: Context::Overworld("Chest_A02_Keep_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: &[],
    },
    // A02_ArcaneTunnels/A02_GameIntro_MemorialMain
    Check {
        context: Context::Overworld("Chest_A02_GameIntro"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        requirements: &[],
    },
    Check {
        context: Context::Overworld("Chest_A02_Sword_DiamondWings"),
        drop: Drop::Weapon(Weapons::DiamondWings),
        requirements: &[],
    },
    Check {
        context: Context::Overworld("Dance_Platform_Photo_Chest"),
        drop: Drop::Item(Items::Mandoline, 1),
        requirements: &[&[Drop::Emote(Emotes::Photo)]],
    },
];

lazy_static::lazy_static! {
    static ref LOCATIONS: hashbrown::HashMap<&'static str, Location> = hashmap![
        "A02_ArcaneTunnels/A02_GameIntro_KeepSouth" => Location {
            unlocks: &[
                "A02_ArcaneTunnels/A02_GameIntro_Exterior",
                "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
                "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
            ],
            requirements: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_Exterior" => Location {
            unlocks: &[],
            requirements: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_KeepEast" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_EastWing"],
            requirements: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_EastWing" => Location {
            unlocks: &[],
            requirements: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_KeepWest"],
            requirements: &[&[Drop::Item(Items::OldKey, 1)]],
        },
        "A02_ArcaneTunnels/A02_GameIntro_KeepWest" => Location {
            unlocks: &["A02_ArcaneTunnels/A02_GameIntro_MemorialMain"],
            requirements: &[],
        },
        "A02_ArcaneTunnels/A02_GameIntro_MemorialMain" => Location {
            unlocks: &[/* into arcane tunnels */],
            requirements: &[],
        }
    ];
}
