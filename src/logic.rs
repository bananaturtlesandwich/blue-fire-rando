#![allow(dead_code)]
mod drops;
pub use drops::*;
mod seeding;
pub use seeding::randomise;
mod writing;
pub use writing::write;

#[derive(Debug, Clone, strum::EnumIter, strum::AsRefStr)]
pub enum Shop {
    #[strum(serialize = "ShopA")]
    Mork = 7,
    #[strum(serialize = "ShopC")]
    SpiritHunter = 9,
    #[strum(serialize = "ShopD")]
    Ari = 10,
    #[strum(serialize = "ShopE")]
    Poti = 11,
    #[strum(serialize = "ShopF")]
    Poi = 12,
    #[strum(serialize = "ShopH")]
    Nilo = 19,
}

#[derive(Debug)]
pub enum Context {
    Shop(Shop, usize),
    Cutscene(&'static str),
    Overworld(&'static str),
    Starting,
}

#[derive(PartialEq, Clone, Debug, strum::AsRefStr)]
pub enum Drop {
    #[strum(serialize = "0")]
    Item(Items, i32),
    #[strum(serialize = "1")]
    Weapon(Weapons),
    #[strum(serialize = "2")]
    Tunic(Tunics),
    #[strum(serialize = "3")]
    Spirit(Spirits),
    #[strum(serialize = "6")]
    Ability(Abilities),
    #[strum(serialize = "7")]
    Emote(Emotes),
    Ore(i32),
    Duck,
}

impl Drop {
    fn as_u8(&self) -> u8 {
        match self {
            Drop::Item(_, _) => 0,
            Drop::Weapon(_) => 1,
            Drop::Tunic(_) => 2,
            Drop::Spirit(_) => 3,
            Drop::Ability(_) => 6,
            Drop::Emote(_) => 7,
            Drop::Ore(_) => 8,
            Drop::Duck => 9,
        }
    }

    fn inner_as_u8(&self) -> u8 {
        match self {
            Drop::Item(inner, _) => inner.clone() as u8,
            Drop::Weapon(inner) => inner.clone() as u8,
            Drop::Tunic(inner) => inner.clone() as u8,
            Drop::Spirit(inner) => inner.clone() as u8,
            Drop::Ability(inner) => inner.clone() as u8,
            Drop::Emote(inner) => inner.clone() as u8,
            Drop::Ore(inner) => inner.clone() as u8,
            Drop::Duck => 80,
        }
    }
}

#[derive(Debug)]
pub struct Check {
    location: &'static str,
    context: Context,
    drop: Drop,
    requirements: Option<&'static [&'static [Drop]]>,
}

struct Location {
    unlocks: &'static [&'static str],
    requirements: Option<&'static [&'static [Drop]]>,
}

const CHECKS: [Check; 17] = [
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepSouth",
        context: Context::Starting,
        drop: Drop::Emote(Emotes::Hello2),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepSouth",
        context: Context::Starting,
        drop: Drop::Ability(Abilities::Dash),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_Exterior",
        context: Context::Overworld("A01_FireKeep_EmoteStatue_Levitation"),
        drop: Drop::Emote(Emotes::Levitation),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_Exterior",
        context: Context::Cutscene(
            "/Game/BlueFire/NPC/Onops/MUSIC_Onops/Onop_Musicians/NPC_Onop_IO_Bitoven",
        ),
        drop: Drop::Ore(500),
        requirements: Some(&[&[Drop::Item(Items::ComposerLetter, 1)]]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro",
        context: Context::Overworld("Duck"),
        drop: Drop::Duck,
        requirements: Some(&[
            &[
                Drop::Ability(Abilities::DoubleJump),
                Drop::Ability(Abilities::SpinAttack),
                Drop::Spirit(Spirits::HolyCentry),
            ],
            &[
                Drop::Ability(Abilities::Dash),
                Drop::Ability(Abilities::SpinAttack),
                Drop::Spirit(Spirits::PossesedBook),
            ],
        ]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        context: Context::Overworld("Chest_A01_TempleGardens_Ability_SpinAttack2"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        context: Context::Overworld("Chest_A02_Keep_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        context: Context::Overworld("Pickup"),
        drop: Drop::Ore(250),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_EastWing",
        context: Context::Overworld("Chest_A02_Keep_Key_01"),
        drop: Drop::Item(Items::OldKey, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_EastWing",
        context: Context::Overworld("Chest_A01_Keep_Shield"),
        drop: Drop::Ability(Abilities::Block),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
        context: Context::Overworld("A01_FireKeep_EmoteStatue_Techno"),
        drop: Drop::Emote(Emotes::Techno),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
        context: Context::Overworld("Spirit_A02_RiverSpirit"),
        drop: Drop::Spirit(Spirits::RiverSpirit),
        requirements: Some(&[
            &[Drop::Ability(Abilities::Dash)],
            &[Drop::Ability(Abilities::WallRun)],
            &[
                Drop::Ability(Abilities::DoubleJump),
                Drop::Ability(Abilities::SpinAttack),
            ],
        ]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepWest",
        context: Context::Overworld("Chest_A02_Keep_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("Chest_A02_GameIntro"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("Chest_A02_Sword_DiamondWings"),
        drop: Drop::Weapon(Weapons::DiamondWings),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("A02_FireKeep_EmoteStatue_Celebration"),
        drop: Drop::Emote(Emotes::Celebration),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("Dance_Platform_Photo_Chest"),
        drop: Drop::Item(Items::Mandoline, 1),
        requirements: Some(&[&[Drop::Emote(Emotes::Photo)]]),
    },
];
