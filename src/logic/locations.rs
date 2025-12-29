use super::*;
use crate::{no_walljump, walljump};

#[derive(Debug, PartialEq, Eq, Hash, Clone, strum::Display, strum::EnumIter, strum::EnumCount)]
pub enum Location {
    // Fire Keep
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_KeepSouth")]
    Lab,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_Exterior")]
    Bitoven,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_DLC_VoidMaster")]
    KeepDucks,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_KeepEast")]
    KeepVessel,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_EastWing")]
    Shield,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom")]
    FirstVoid,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_KeepWest")]
    Crates,
    #[strum(serialize = "A02_ArcaneTunnels/A02_GameIntro_MemorialMain")]
    Memorial,
    // Arcane Tunnels
    #[strum(serialize = "A02_ArcaneTunnels/A02_NorthArcane")]
    ArcaneNorth,
    #[strum(serialize = "A02_ArcaneTunnels/A02_SouthArcane")]
    ArcaneSouth,
    #[strum(serialize = "A02_ArcaneTunnels/A02_EastArcane")]
    ArcaneSpiritHunter,
    #[strum(serialize = "A02_ArcaneTunnels/A02_Arcane")]
    ArcaneDucks,
    // Crossroads
    #[strum(serialize = "A01_StoneHeartCity/A01_CrossRoads")]
    Crossroads,
    #[strum(serialize = "A01_StoneHeartCity/A01_Well")]
    Well,
    // Stoneheart City
    #[strum(serialize = "A01_StoneHeartCity/A01_CliffPath")]
    Stoneheart,
    // Forest Temple
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine_WaterLevels")]
    WaterLevels,
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine_AmbushZone")]
    NuosClaw,
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine_CenterTree")]
    Tree,
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine")]
    ForestDucks,
    #[strum(serialize = "A01_StoneHeartCity/A01_AbilityShrine_BossRoom")]
    Gruh,
    // Temple Gardens
    #[strum(serialize = "A01_StoneHeartCity/A01_TempleGardens")]
    TempleGardens,
    // Abandoned Path
    #[strum(serialize = "A01_StoneHeartCity/A01_Graveyard")]
    AbandonedPath,
    #[strum(serialize = "A01_StoneHeartCity/A01_GraveyardShrine")]
    Beira,
    // Uthas Temple
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_Intro")]
    UthasStart,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_Main")]
    UthasBracelet,
    #[strum(serialize = "A01_StoneHeartCity/A01_SmallShrine")]
    UthasDucks,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_SouthEast")]
    UthasPuzzle,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_SouthWest")]
    UthasCombat,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_BottomPassage")]
    UthasPlatforming,
    #[strum(serialize = "A02_ArcaneTunnels/A01_SmallShrine_EndPath")]
    UthasEnd,
    // Temple of the gods
    #[strum(serialize = "A10_PenumbraTemple/A10_Entrance")]
    SanctuaryStone,
    #[strum(serialize = "A10_PenumbraTemple/A10_GodessChamber")]
    Queen,
    // Firefall River
    #[strum(serialize = "A06_IronCaves/A06_FireFall_A")]
    FirefallSpiritHunter,
    #[strum(serialize = "A06_IronCaves/A06_FireFall_B")]
    Bunny,
    #[strum(serialize = "A06_IronCaves/A06_LakeMolva")]
    LakeMolva,
    #[strum(serialize = "A06_IronCaves/A06_FireFallRiver")]
    FirefallDucks,
    // Steam House
    #[strum(serialize = "A06_IronCaves/A06_SteamHouse_Core")]
    SteamHouse,
    #[strum(serialize = "A06_IronCaves/A06_SteamHouse")]
    SteamHouseDucks,
    #[strum(serialize = "A06_IronCaves/A06_SteamHouse_Corridor")]
    SteamHousePlatforming,
    #[strum(serialize = "A06_IronCaves/A06_Sirion")]
    Sirion,
    // Rust Village
    #[strum(serialize = "A06_IronCaves/A06_RustCity")]
    RustVillage,
    // Waterway
    #[strum(serialize = "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess")]
    Waterway,
    #[strum(serialize = "A02_ArcaneTunnels/A02_CentralWaterWay")]
    WaterwayDucks,
    #[strum(serialize = "A02_ArcaneTunnels/A02_BossRoom")]
    Samael,
    #[strum(serialize = "Voids/16_VoidPaul/Void_Paul")]
    PathOfVictory,
    #[strum(serialize = "Voids/03_VoidEasy/Void_Easy")]
    ArigosChallenge,
    #[strum(serialize = "Voids/02_Void04_V2/Void_04_V2")]
    BorisasFate,
    #[strum(serialize = "Voids/15_VoidFlauta/Void_Flauta")]
    JuliansSong,
    #[strum(serialize = "Voids/17_VoidSanti/Void_Santi")]
    Alchemist,
    #[strum(serialize = "don't look into it - after all it stares back")]
    TheVoid,
}

impl Location {
    pub const fn locks(&self) -> &[&[Lock]] {
        match self {
            Location::Lab => &[&[]],
            Location::Bitoven => &[&[Lock::Location(Location::Lab)]],
            Location::KeepDucks => &[&[Lock::Location(Location::Bitoven)]],
            Location::KeepVessel => &[&[Lock::Location(Location::Lab)]],
            Location::Shield => &[&[Lock::Location(Location::KeepVessel)]],
            Location::FirstVoid => &[&[Lock::Location(Location::Lab), Lock::Item(Item::OldKey)]],
            Location::Crates => &[&[Lock::Location(Location::FirstVoid)]],
            Location::Memorial => &[&[Lock::Location(Location::Crates)]],
            Location::ArcaneNorth => &[&[Lock::Location(Location::Memorial)]],
            Location::ArcaneSouth => &[&[Lock::Location(Location::ArcaneNorth)]],
            Location::ArcaneSpiritHunter => &[&[Lock::Location(Location::ArcaneSouth)]],
            Location::ArcaneDucks => &[&[Lock::Location(Location::ArcaneSpiritHunter)]],
            Location::Crossroads => &[&[Lock::Location(Location::ArcaneSpiritHunter)]],
            Location::Well => &[&[
                Lock::Location(Location::Crossroads),
                Lock::Movement(&[no_walljump!(0, 1)]),
            ]],
            Location::Stoneheart => &[&[Lock::Location(Location::ArcaneSpiritHunter)]],
            Location::WaterLevels => &[&[Lock::Location(Location::Stoneheart)]],
            Location::NuosClaw => &[&[
                Lock::Location(Location::WaterLevels),
                Lock::Item(Item::OldKey),
            ]],
            Location::Tree => &[&[
                Lock::Location(Location::WaterLevels),
                Lock::Movement(&[walljump!(0, 1), no_walljump!(2, 0)]),
            ]],
            Location::ForestDucks => &[&[Lock::Location(Location::Tree)]],
            Location::Gruh => &[&[
                Lock::Location(Location::Tree),
                Lock::Item(Item::KeyHolyMaster),
            ]],
            Location::TempleGardens => &[&[
                Lock::Item(Item::OldKey),
                Lock::Item(Item::KeyHolyMaster),
                Lock::Location(Location::UthasEnd),
                Lock::Movement(&[no_walljump!(1, 0)]),
            ]],
            Location::AbandonedPath => &[&[
                Lock::Location(Location::Stoneheart),
                Lock::Item(Item::KeyGraveyardKey),
                Lock::Movement(&[walljump!(0, 1), no_walljump!(0, 3)]),
            ]],
            Location::Beira => &[&[
                Lock::Location(Location::AbandonedPath),
                Lock::Location(Location::LakeMolva),
                Lock::Location(Location::TempleGardens),
                Lock::Item(Item::SanctuaryStone),
                Lock::Item(Item::BeiraVessel),
                // needs walljump for temple gardens blocked stairway soul
                // also requires climbing tower
                Lock::Movement(&[walljump!(3, 3)]),
            ]],
            Location::UthasStart => &[&[
                Lock::Location(Location::AbandonedPath),
                Lock::Movement(&[no_walljump!(0, 2)]),
                Lock::Item(Item::KeyUthasTemple),
            ]],
            Location::UthasBracelet => &[&[
                Lock::Location(Location::UthasStart),
                Lock::Item(Item::OldKey),
            ]],
            Location::UthasDucks => &[&[Lock::Location(Self::UthasBracelet)]],
            Location::UthasPuzzle => &[&[Lock::Location(Self::UthasBracelet)]],
            Location::UthasCombat => &[&[
                Lock::Location(Location::UthasBracelet),
                Lock::Movement(&[walljump!(0, 1), no_walljump!(0, 3)]),
                Lock::Item(Item::OldKey),
            ]],
            Location::UthasPlatforming => &[&[
                Lock::Location(Location::UthasBracelet),
                Lock::Movement(&[no_walljump!(1, 4), walljump!(1, 3)]),
                Lock::Item(Item::OldKey),
            ]],
            Location::UthasEnd => &[
                &[
                    Lock::Location(Location::UthasBracelet),
                    Lock::Movement(&[walljump!(0, 1), no_walljump!(0, 3)]),
                    Lock::Item(Item::OldKey),
                ],
                &[
                    Lock::Location(Location::UthasBracelet),
                    Lock::Movement(&[walljump!(0, 1), no_walljump!(4, 3)]),
                ],
            ],
            Location::SanctuaryStone => &[&[
                Lock::Location(Location::TempleGardens),
                Lock::Item(Item::KeyGodMaster),
            ]],
            Location::Queen => &[&[
                Lock::Location(Location::SanctuaryStone),
                Lock::Location(Location::Samael),
                Lock::Location(Location::Sirion),
                Lock::Location(Location::Beira),
            ]],
            Location::FirefallSpiritHunter => &[&[Lock::Location(Location::SanctuaryStone)]],
            Location::Bunny => &[&[
                Lock::Location(Self::FirefallSpiritHunter),
                Lock::Movement(&[no_walljump!(0, 3)]),
            ]],
            Location::LakeMolva => &[&[
                Lock::Location(Location::Bunny),
                Lock::Movement(&[no_walljump!(0, 1)]),
            ]],
            Location::FirefallDucks => &[&[Lock::Location(Location::LakeMolva)]],
            Location::SteamHouse => &[&[
                Lock::Location(Location::LakeMolva),
                Lock::Movement(&[no_walljump!(1, 2)]),
            ]],
            Location::SteamHouseDucks => &[&[Lock::Location(Location::SteamHouse)]],
            Location::SteamHousePlatforming => &[&[
                Lock::Location(Location::SteamHouse),
                Lock::Item(Item::KeySteam),
                Lock::Movement(&[no_walljump!(1, 4), walljump!(1, 3)]),
            ]],
            Location::Sirion => &[&[
                Lock::Location(Location::SteamHouse),
                Lock::Item(Item::KeyFireMaster),
                Lock::Movement(&[walljump!(0, 4), no_walljump!(0, 5)]),
                Lock::Item(Item::SanctuaryStone),
            ]],
            Location::RustVillage => &[&[
                Lock::Location(Location::SteamHouse),
                Lock::Location(Location::SteamHousePlatforming),
                Lock::Movement(&[no_walljump!(1, 2)]),
                Lock::IronJustice,
            ]],
            Location::Waterway => &[&[Lock::Location(Location::ArcaneSouth)]],
            Location::WaterwayDucks => &[&[Lock::Location(Location::Waterway)]],
            Location::Samael => &[&[
                Lock::Location(Location::ArcaneSouth),
                Lock::Location(Location::LakeMolva),
                // movement requirements for oliver's diary area
                Lock::Movement(&[walljump!(0, 4), no_walljump!(0, 8)]),
                Lock::Item(Item::SanctuaryStone),
            ]],
            Location::PathOfVictory => &[&[Lock::Location(Location::ArcaneSouth)]],
            Location::ArigosChallenge => &[&[Lock::Location(Location::Tree)]],
            Location::BorisasFate => &[&[
                Lock::Location(Location::Stoneheart),
                Lock::Movement(&[walljump!(0, 2), no_walljump!(0, 4)]),
            ]],
            Location::JuliansSong => &[&[Lock::Location(Location::TempleGardens)]],
            Location::Alchemist => &[&[Lock::Location(Location::LakeMolva)]],
            Location::TheVoid => &[&[
                Lock::Location(Location::PathOfVictory),
                Lock::Location(Location::ArigosChallenge),
                Lock::Location(Location::BorisasFate),
                Lock::Location(Location::JuliansSong),
                Lock::Location(Location::Alchemist),
                Lock::Movement(&[walljump!(1, 3)]),
            ]],
        }
    }
}
