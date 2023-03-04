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

#[derive(Clone, Copy, Debug, strum::AsRefStr)]
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

impl PartialEq for Drop {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Item(l0, _), Self::Item(r0, _)) => l0 == r0,
            (Self::Weapon(l0), Self::Weapon(r0)) => l0 == r0,
            (Self::Tunic(l0), Self::Tunic(r0)) => l0 == r0,
            (Self::Spirit(l0), Self::Spirit(r0)) => l0 == r0,
            (Self::Ability(l0), Self::Ability(r0)) => l0 == r0,
            (Self::Emote(l0), Self::Emote(r0)) => l0 == r0,
            (Self::Ore(l0), Self::Ore(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
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
    locks: Option<&'static [Lock]>,
}

struct Location {
    unlocks: &'static [&'static str],
    locks: Option<&'static [Lock]>,
}

#[derive(Debug)]
enum Lock {
    Location(&'static str),
    Movement(&'static [Move]),
    Item(Items),
    Emote(Emotes),
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Move {
    extra_height: u8,
    horizontal: u8,
    walljump: bool,
}

impl Move {
    const fn walljump(extra_height: u8, horizontal: u8) -> Self {
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
