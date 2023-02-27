#![allow(dead_code)]
mod drops;
pub use drops::*;
mod seeding;
pub use seeding::randomise;
mod writing;
pub use writing::write;
mod checks;
pub use checks::CHECKS;

#[derive(Debug, Clone, Copy, strum::EnumIter, strum::AsRefStr)]
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

#[derive(PartialEq, Clone, Copy, Debug, strum::AsRefStr)]
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
    #[strum(serialize = "0")]
    Ore(i32),
    #[strum(serialize = "0")]
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
            Drop::Item(inner, _) => *inner as u8,
            Drop::Weapon(inner) => *inner as u8,
            Drop::Tunic(inner) => *inner as u8,
            Drop::Spirit(inner) => *inner as u8,
            Drop::Ability(inner) => *inner as u8,
            Drop::Emote(inner) => *inner as u8,
            Drop::Ore(inner) => *inner as u8,
            Drop::Duck => 80,
        }
    }
}

#[derive(Debug)]
pub struct Check {
    location: &'static str,
    context: Context,
    drop: Drop,
    requirements: Option<&'static [Requirement]>,
}

struct Location {
    unlocks: &'static [&'static str],
    requirements: Option<&'static [Requirement]>,
}

#[derive(Debug)]
enum Requirement {
    Location(&'static str),
    Movement(&'static [Move]),
    Item(Items),
    Emote(Emotes),
}

#[derive(Debug)]
struct Move {
    extra_height: u8,
    horizontal: u8,
    walljump: bool,
}

impl Move {
    const fn with_walljump(extra_height: u8, horizontal: u8) -> Self {
        Self {
            extra_height,
            horizontal,
            walljump: true,
        }
    }
    const fn no_walljump(extra_height: u8, horizontal: u8) -> Self {
        Self {
            extra_height,
            horizontal,
            walljump: false,
        }
    }
}
