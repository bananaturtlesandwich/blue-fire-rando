use super::*;

pub const LOCATIONS: [Location; 21] = [
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
    // Waterways
    Location {
        map: "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess",
        locks: &[&[Lock::Location("A02_ArcaneTunnels/A02_SouthArcane")]],
    },
];
