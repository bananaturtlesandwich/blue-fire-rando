use super::*;

pub const LOCATIONS: [Location; 33] = [
    // Fire Keep
    Location {
        map: "A02_ArcaneTunnels/A02_GameIntro_KeepSouth",
        locks: &[&[]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_GameIntro_Exterior",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_GameIntro_KeepSouth")]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_GameIntro",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_GameIntro_Exterior")]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_GameIntro_KeepSouth")]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_GameIntro_EastWing",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_GameIntro_KeepEast")]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
        locks: &[&[
            Lock::Location("A02_ArcaneTunnels/A02_GameIntro_KeepSouth"),
            Lock::Item(Items::OldKey),
        ]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_GameIntro_KeepWest",
        locks: &[&[Lock::Location(
            "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
        )]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_GameIntro_KeepWest")]],
    },
    // Arcane Tunnels
    Location {
        map: "A02_ArcaneTunnels/A02_NorthArcane",
        locks: &[&[Lock::Location(
            "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        )]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_SouthArcane",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_NorthArcane")]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_EastArcane",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_SouthArcane")]],
    },
    Location {
        map: "A02_ArcaneTunnels/A02_Arcane",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_EastArcane")]],
    },
    // Crossroads
    Location {
        map: "A01_StoneHeartCity/A01_CrossRoads",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_EastArcane")]],
    },
    Location {
        map: "A01_StoneHeartCity/A01_Well",
        locks: &[&[
            Lock::Location("A01_StoneHeartCity/A01_CrossRoads"),
            Lock::Movement(&[Move::no_walljump(0, 1)]),
        ]],
    },
    // Stoneheart City
    Location {
        map: "A01_StoneHeartCity/A01_CliffPath",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_EastArcane")]],
    },
    // Forest Temple
    Location {
        map: "A01_StoneHeartCity/A01_AbilityShrine_WaterLevels",
        locks: &[&[Lock::Location("A01_StoneHeartCity/A01_CliffPath")]],
    },
    Location {
        map: "A01_StoneHeartCity/A01_AbilityShrine_AmbushZone",
        locks: &[&[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_WaterLevels"),
            Lock::Item(Items::OldKey),
        ]],
    },
    Location {
        map: "A01_StoneHeartCity/A01_AbilityShrine_CenterTree",
        locks: &[&[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_WaterLevels"),
            Lock::Movement(&[Move::walljump(0, 1), Move::no_walljump(2, 0)]),
        ]],
    },
    Location {
        map: "A01_StoneHeartCity/A01_AbilityShrine",
        locks: &[&[Lock::Location(
            "A01_StoneHeartCity/A01_AbilityShrine_CenterTree",
        )]],
    },
    Location {
        map: "A01_StoneHeartCity/A01_AbilityShrine_BossRoom",
        locks: &[&[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_CenterTree"),
            Lock::Item(Items::KeyHolyMaster),
        ]],
    },
    // Temple Gardens
    Location {
        map: "A01_StoneHeartCity/A01_TempleGardens",
        locks: &[&[
            Lock::Location("A01_StoneHeartCity/A01_CliffPath"),
            Lock::Movement(&[Move::no_walljump(1, 0)]),
        ]],
    },
    // Abandoned Path
    Location {
        map: "A01_StoneHeartCity/A01_Graveyard",
        locks: &[&[
            Lock::Location("A01_StoneHeartCity/A01_CliffPath"),
            Lock::Item(Items::KeyGraveyardKey),
            Lock::Movement(&[Move::walljump(0, 1), Move::no_walljump(0, 3)]),
        ]],
    },
    // Uthas Temple
    Location {
        map: "A02_ArcaneTunnels/A01_SmallShrine_Intro",
        locks: &[&[
            Lock::Location("A01_StoneHeartCity/A01_Graveyard"),
            Lock::Movement(&[Move::no_walljump(0, 2)]),
            Lock::Item(Items::KeyUthasTemple),
        ]],
    },
    Location {
        map: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        locks: &[&[
            Lock::Location("A02_ArcaneTunnels/A01_SmallShrine_Intro"),
            Lock::Item(Items::OldKey),
        ]],
    },
    Location {
        map: "A02_ArcaneTunnels/A01_SmallShrine_SouthEast",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A01_SmallShrine_Main")]],
    },
    Location {
        map: "A02_ArcaneTunnels/A01_SmallShrine_SouthWest",
        locks: &[&[
            Lock::Location("A02_ArcaneTunnels/A01_SmallShrine_Main"),
            Lock::Movement(&[Move::walljump(0, 1), Move::no_walljump(0, 3)]),
            Lock::Item(Items::OldKey),
        ]],
    },
    Location {
        map: "A02_ArcaneTunnels/A01_SmallShrine_BottomPassage",
        locks: &[&[
            Lock::Location("A02_ArcaneTunnels/A01_SmallShrine_Main"),
            Lock::Movement(&[Move::no_walljump(1, 4), Move::walljump(1, 3)]),
            Lock::Item(Items::OldKey),
        ]],
    },
    Location {
        map: "A02_ArcaneTunnels/A01_SmallShrine_EndPath",
        locks: &[
            &[
                Lock::Location("A02_ArcaneTunnels/A01_SmallShrine_Main"),
                Lock::Movement(&[Move::walljump(0, 1), Move::no_walljump(0, 3)]),
                Lock::Item(Items::OldKey),
            ],
            &[
                Lock::Location("A02_ArcaneTunnels/A01_SmallShrine_Main"),
                Lock::Movement(&[Move::walljump(0, 1), Move::no_walljump(4, 3)]),
            ],
        ],
    },
    // Temple of Gods
    Location {
        map: "A10_PenumbraTemple/A10_Entrance",
        locks: &[&[
            Lock::Location("A01_StoneHeartCity/A01_TempleGardens"),
            Lock::Item(Items::KeyGodMaster),
        ]],
    },
    // Firefall River
    Location {
        map: "A06_IronCaves/A06_Firefall_A",
        locks: &[&[Lock::Location("A10_PenumbraTemple/A10_Entrance")]],
    },
    Location {
        map: "A06_IronCaves/A06_Firefall_B",
        locks: &[&[
            Lock::Location("A06_IronCaves/A06_Firefall_A"),
            Lock::Movement(&[Move::no_walljump(0, 3)]),
        ]],
    },
    Location {
        map: "A06_IronCaves/A06_LakeMolva",
        locks: &[&[
            Lock::Location("A06_IronCaves/A06_Firefall_B"),
            Lock::Movement(&[Move::no_walljump(0, 1)]),
        ]],
    },
    // Waterways
    Location {
        map: "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_SouthArcane")]],
    },
];
