use super::*;

pub const CHECKS: [Check; 227] = [
    // Fire Keep
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepSouth",
        context: Context::Starting,
        drop: Drop::Emote(Emotes::Hello2),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepSouth",
        context: Context::Starting,
        drop: Drop::Ability(Abilities::Dash),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_Exterior",
        context: Context::Overworld("A01_FireKeep_EmoteStatue_Levitation"),
        drop: Drop::Emote(Emotes::Levitation),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_Exterior",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/MUSIC_Onops/Onop_Musicians/NPC_Onop_IO_Bitoven",
        ),
        drop: Drop::Ore(500),
        locks: &[
            Lock::Location("A06_IronCaves/A06_RustCity"),
            Lock::Item(Items::ComposerLetter),
        ],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro",
        context: Context::Overworld("Duck"),
        drop: Drop::Duck,
        locks: &[Lock::Movement(&[Move::no_walljump(4, 0)])],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        context: Context::Overworld("Chest_A01_TempleGardens_Ability_SpinAttack2"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        context: Context::Overworld("Chest_A02_Keep_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        context: Context::Overworld("Pickup"),
        drop: Drop::Ore(250),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_EastWing",
        context: Context::Overworld("Chest_A02_Keep_Key_01"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_EastWing",
        context: Context::Overworld("Chest_A01_Keep_Shield"),
        drop: Drop::Ability(Abilities::Block),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
        context: Context::Overworld("A01_FireKeep_EmoteStatue_Techno"),
        drop: Drop::Emote(Emotes::Techno),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
        context: Context::Overworld("Spirit_A02_RiverSpirit"),
        drop: Drop::Spirit(Spirits::RiverSpirit),
        locks: &[Lock::Movement(&[
            Move::no_walljump(0, 2),
            Move::no_walljump(2, 0),
            Move::walljump(0, 0),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepWest",
        context: Context::Overworld("Chest_A02_Keep_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("Chest_A02_GameIntro"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("Chest_A02_Sword_DiamondWings"),
        drop: Drop::Weapon(Weapons::DiamondWings),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("A02_FireKeep_EmoteStatue_Celebration"),
        drop: Drop::Emote(Emotes::Celebration),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("Dance_Platform_Photo_Chest"),
        drop: Drop::Item(Items::Mandoline, 1),
        locks: &[Lock::Emote(Emotes::Photo)],
    },
    // Arcane Tunnels
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Pickup3"),
        drop: Drop::Ore(250),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("A02_Arcane_EmoteStatue_Windmill"),
        drop: Drop::Emote(Emotes::Windmill),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 2),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Dance_Platform_KungFu_Chest"),
        drop: Drop::Item(Items::Boot, 1),
        locks: &[Lock::Emote(Emotes::KungFu)],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess",
        context: Context::Overworld("A02_Arcane_EmoteStatue_HatKid"),
        drop: Drop::Emote(Emotes::HatKid),
        locks: &[],
    },
    // to hit the lever you can just drop down
    Check {
        location: "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess",
        context: Context::Overworld("Pickup"),
        drop: Drop::Ore(400),
        locks: &[Lock::Location("A06_IronCaves/A06_LakeMolva")],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess",
        context: Context::Overworld("Chest_A02_Tunic_PureShadow"),
        drop: Drop::Tunic(Tunics::PureShadow),
        locks: &[Lock::Location("A02_ArcaneTunnels/A02_BossRoom")],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Sword_Bloodstorm"),
        drop: Drop::Weapon(Weapons::BloodstormBlades),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Spirit_A02_ToxicRat"),
        drop: Drop::Spirit(Spirits::ToxicRat),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_SouthArcane_Key_01"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Dance_Platform_Celebration_Chest"),
        drop: Drop::Item(Items::Rice, 1),
        locks: &[Lock::Emote(Emotes::Celebration)],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_SouthArcane_Loot_01"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_SouthArcane_Key_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Pickup4"),
        drop: Drop::Ore(250),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 4),
            Move::no_walljump(0, 8),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Loot_04"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 4),
            Move::no_walljump(0, 8),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A01_Arcane_Spell"),
        drop: Drop::Ability(Abilities::Spell),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_SRF"),
        drop: Drop::Ore(300),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_SRF2"),
        drop: Drop::Ore(200),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 0, 5500),
        drop: Drop::Spirit(Spirits::StormCentry),
        locks: &[Lock::Item(Items::SmallPouch)],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 1, 3500),
        drop: Drop::Spirit(Spirits::BloodPhantom),
        locks: &[Lock::Item(Items::SmallPouch)],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 2, 4500),
        drop: Drop::Spirit(Spirits::FrozenSoul),
        locks: &[Lock::Item(Items::SmallPouch)],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 3, 2500),
        drop: Drop::Spirit(Spirits::ShadowGru),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_05"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_04"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_Arcane",
        context: Context::Overworld("Duck"),
        drop: Drop::Duck,
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_03"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_Arcane_SR_Loot"),
        drop: Drop::Ore(400),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    // AND THESE ONES ARE NAMED FINE???
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_Arcane_SR_Loot2"),
        drop: Drop::Ore(400),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    // Crossroads
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup49"),
        drop: Drop::Ore(50),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup47"),
        drop: Drop::Ore(50),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Chest_A01_CrossRoads_Loot"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[
            Lock::Movement(&[Move::no_walljump(1, 0)]),
            Lock::Movement(&[Move::no_walljump(0, 2)]),
            Lock::Movement(&[Move::walljump(0, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Chest_A01_Well_SpinAttack"),
        drop: Drop::Ability(Abilities::SpinAttack),
        locks: &[
            Lock::Movement(&[Move::no_walljump(1, 0)]),
            Lock::Movement(&[Move::no_walljump(0, 2)]),
            Lock::Movement(&[Move::walljump(0, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Pickup57"),
        drop: Drop::Ore(500),
        locks: &[
            Lock::Movement(&[Move::no_walljump(1, 0)]),
            Lock::Movement(&[Move::no_walljump(0, 2)]),
            Lock::Movement(&[Move::walljump(0, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Pickup60"),
        drop: Drop::Ore(500),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 2)])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Pickup5"),
        drop: Drop::Ore(500),
        locks: &[
            Lock::Movement(&[Move::no_walljump(2, 2)]),
            Lock::Movement(&[Move::walljump(1, 2)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Pickup60"),
        drop: Drop::Ore(500),
        locks: Some(&[
            Lock::Movement(&[Move::no_walljump(1, 0)]),
            Lock::Movement(&[Move::no_walljump(0, 2)]),
            Lock::Movement(&[Move::walljump(0, 0)]),
        ]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Pickup5"),
        drop: Drop::Ore(500),
        locks: Some(&[
            Lock::Movement(&[Move::no_walljump(1, 0)]),
            Lock::Movement(&[Move::no_walljump(0, 2)]),
            Lock::Movement(&[Move::walljump(0, 0)]),
        ]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup7"),
        drop: Drop::Ore(50),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup14"),
        drop: Drop::Ore(50),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup15"),
        drop: Drop::Ore(50),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup11"),
        drop: Drop::Ore(50),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup12"),
        drop: Drop::Ore(50),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup56"),
        drop: Drop::Ore(50),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Chest_A01_CrossRoads_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup50"),
        drop: Drop::Ore(50),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Chest_A01_CrossRoads_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Dance_Platform_Techno_Chest"),
        drop: Drop::Tunic(Tunics::Galaxy),
        locks: &[
            Lock::Emote(Emotes::Techno),
            Lock::Movement(&[Move::no_walljump(0, 1)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Chest_A01_CrossRoads_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    // Stoneheart City
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_SandRelic"),
        drop: Drop::Item(Items::SandRelic, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup51"),
        drop: Drop::Ore(100),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_A01_Stoneheart_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 2),
        locks: &[Lock::Movement(&[
            Move::walljump(2, 0),
            Move::no_walljump(4, 0),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_A01_City_SRL3"),
        drop: Drop::Ore(400),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 0, 1000),
        drop: Drop::Tunic(Tunics::Orange),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 1, 1000),
        drop: Drop::Tunic(Tunics::Aqua),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 2, 1000),
        drop: Drop::Tunic(Tunics::Royal),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 3, 1000),
        drop: Drop::Tunic(Tunics::Lila),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 4, 5000),
        drop: Drop::Tunic(Tunics::Rainbow),
        locks: &[Lock::Item(Items::SmallPouch)],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 5, 1000),
        drop: Drop::Tunic(Tunics::LightBlue),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 6, 1000),
        drop: Drop::Tunic(Tunics::Violet),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 7, 1000),
        drop: Drop::Tunic(Tunics::Grey),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 8, 1000),
        drop: Drop::Tunic(Tunics::Green),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 9, 1000),
        drop: Drop::Tunic(Tunics::Yellow),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 10, 1000),
        drop: Drop::Tunic(Tunics::Red),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup27"),
        drop: Drop::Ore(200),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_A01_City_SRL2"),
        drop: Drop::Ore(300),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Orip/BP_Orip_Saw/NPC_Orip_Stoneheart",
        ),
        drop: Drop::Item(Items::OddRock, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_A01_City_SRL"),
        drop: Drop::Ore(150),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup45"),
        drop: Drop::Ore(150),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_A01_Stoneheart_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup46"),
        drop: Drop::Ore(100),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup54"),
        drop: Drop::Ore(100),
        locks: &[],
    },
    // the cutest chest in existence
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_A01_Stoneheart_Loot"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 0, 2800),
        drop: Drop::Spirit(Spirits::PossesedBook),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 1, 2500),
        drop: Drop::Spirit(Spirits::GoldenLust),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 2, 2200),
        drop: Drop::Spirit(Spirits::LifeSteal),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 3, 4000),
        drop: Drop::Item(Items::LargePouch, 1),
        locks: &[Lock::Item(Items::SmallPouch)],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 4, 1200),
        drop: Drop::Item(Items::RareSnow, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book3"),
        drop: Drop::Item(Items::Book, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book"),
        drop: Drop::Item(Items::Book, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Dance_Platform_Triceps_Chest"),
        drop: Drop::Item(Items::IceCrystal, 1),
        locks: &[Lock::Emote(Emotes::Triceps)],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Spirit_A01_FarasGrace"),
        drop: Drop::Spirit(Spirits::FarasGrace),
        locks: &[Lock::Item(Items::Necklace)],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup29"),
        drop: Drop::Ore(150),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup34"),
        drop: Drop::Ore(100),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book4"),
        drop: Drop::Item(Items::Book, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book5"),
        drop: Drop::Item(Items::Book, 1),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 0),
            Move::no_walljump(1, 0),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_A01_Stoneheart_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 0),
            Move::no_walljump(1, 0),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book2"),
        drop: Drop::Item(Items::Book, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Cutscene("Blue Fire/Content/BlueFire/NPC/Merchant/NPC_Merchant"),
        drop: Drop::Tunic(Tunics::MerchantsRobe),
        locks: &[
            Lock::Item(Items::Book),
            Lock::Item(Items::Book),
            Lock::Item(Items::Book),
            Lock::Item(Items::Book),
            Lock::Item(Items::Book),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/MUSIC_Onops/Onop_Musicians/NPC_Onop_IO_Wolfgang",
        ),
        drop: Drop::Ore(500),
        locks: &[
            Lock::Location("A06_RustCity"),
            Lock::Item(Items::ComposerLetter),
        ],
    },
    // Forest Temple
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_WaterLevels",
        context: Context::Overworld("Chest_A01_Nuos_Loot_05"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_WaterLevels",
        context: Context::Overworld("Chest_A01_Nuos_Loot_02"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_WaterLevels",
        context: Context::Overworld("Chest_A01_Nuos_Key"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_AmbushZone",
        context: Context::Overworld("Chest_A01_Nuos_Loot_01"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_AmbushZone",
        context: Context::Overworld("Chest_A01_Nuos_Key_01"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_AmbushZone",
        context: Context::Overworld("Chest_A01_Nuos_Key_03"),
        drop: Drop::Item(Items::KeyHolyMaster, 1),
        locks: &[Lock::Item(Items::OldKey)],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_AmbushZone",
        context: Context::Overworld("Chest_A01_Nuos_Ability_WallRun"),
        drop: Drop::Ability(Abilities::WallRun),
        locks: &[Lock::Item(Items::KeyHolyMaster)],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_WaterLevels",
        context: Context::Overworld("Spirit_A01_ForestGuardian"),
        drop: Drop::Spirit(Spirits::ForestGuardian),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 0),
            Move::no_walljump(5, 0),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_WaterLevels",
        context: Context::Overworld("Chest_A01_Nuos_Loot"),
        drop: Drop::Item(Items::EmeraldOre, 3),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 4),
            Move::no_walljump(2, 10),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_CenterTree",
        context: Context::Overworld("Chest_A01_Nuos_Loot_06"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_CenterTree",
        context: Context::Overworld("Chest_A01_TempleGardens_Sword_SilverBlades"),
        drop: Drop::Weapon(Weapons::SilverBlades),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_CenterTree",
        context: Context::Overworld("A01_Nuos_EmoteStatue_Wave"),
        drop: Drop::Emote(Emotes::Wave),
        locks: &[Lock::Movement(&[
            Move::no_walljump(2, 1),
            Move::walljump(0, 1),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_CenterTree",
        context: Context::Overworld("Chest_A01_Nuos_Loot_04"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[Lock::Movement(&[
            Move::no_walljump(2, 1),
            Move::walljump(0, 1),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_CenterTree",
        context: Context::Overworld("Chest_A01_Nuos_Key_02"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: &[Lock::Movement(&[
            Move::no_walljump(2, 1),
            Move::walljump(0, 1),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine",
        context: Context::Overworld("Dance_Platform_Hello2_Chest"),
        drop: Drop::Item(Items::Apple, 1),
        locks: &[Lock::Movement(&[
            Move::no_walljump(2, 1),
            Move::walljump(0, 1),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine",
        context: Context::Overworld("Duck"),
        drop: Drop::Duck,
        locks: &[Lock::Movement(&[
            Move::no_walljump(2, 1),
            Move::walljump(0, 1),
        ])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine",
        context: Context::Overworld("Chest_A01_Nuos_MasterKey"),
        drop: Drop::Item(Items::KeyHolyMaster, 1),
        locks: &[Lock::Item(Items::OldKey)],
    },
    Check {
        location: "A01_StoneHeartCity/A01_AbilityShrine_BossRoom",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/Cinematics/NuosTempleEnd/Nuos_Temple_End_Controller",
        ),
        drop: Drop::Item(Items::KeyUthasTemple, 1),
        locks: &[],
    },
    // Tavern
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Cutscene("Blue Fire/Content/BlueFire/NPC/Bremur/NPC_Bremur"),
        drop: Drop::Item(Items::KeyGraveyardKey, 1),
        locks: &[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_BossRoom"),
            Lock::Movement(&[Move::walljump(0, 0), Move::no_walljump(1, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_Master_A01_StoneheartCity_GraveyardKey"),
        drop: Drop::Item(Items::DeadRat, 1),
        locks: &[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_BossRoom"),
            Lock::Movement(&[Move::walljump(0, 0), Move::no_walljump(1, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_Master_A01_StoneheartCity_GraveyardKey2"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: &[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_BossRoom"),
            Lock::Movement(&[Move::walljump(0, 0), Move::no_walljump(1, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_A01_Stoneheart_Tunic_MerchantsRobe"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_BossRoom"),
            Lock::Movement(&[Move::walljump(0, 0), Move::no_walljump(1, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/Onop_Thief/NPC_Onop_Thief",
        ),
        drop: Drop::Item(Items::SmallPouch, 1),
        locks: &[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_BossRoom"),
            Lock::Movement(&[Move::walljump(0, 0), Move::no_walljump(1, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Poi, 0, 150),
        drop: Drop::Item(Items::FireEssence, 1),
        locks: &[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_BossRoom"),
            Lock::Movement(&[Move::walljump(0, 0), Move::no_walljump(1, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Poi, 1, 1500),
        drop: Drop::Item(Items::FireEssence, 2),
        locks: &[
            Lock::Location("A01_StoneHeartCity/A01_AbilityShrine_BossRoom"),
            Lock::Movement(&[Move::walljump(0, 0), Move::no_walljump(1, 0)]),
        ],
    },
    // Temple Gardens
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Pickup2"),
        drop: Drop::Ore(100),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/InteractiveObjects/Collectibles/BloodStone/BloodStone_BP",
        ),
        drop: Drop::Item(Items::FireEssenceSlot, 2),
        locks: &[Lock::EvolairTunic],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_Loot_04"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_Loot_06"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_Loot_07"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_Loot_08"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_RareSnow"),
        drop: Drop::Item(Items::DeadRat, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Cutscene("Blue Fire/Content/BlueFire/NPC/Nilo/NPC_Nilo"),
        drop: Drop::Tunic(Tunics::ForestTunic),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Shop(Shop::Nilo, 0, 1500),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Shop(Shop::Nilo, 1, 800),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Shop(Shop::Nilo, 2, 500),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Dance_Platform_Wave_Chest"),
        drop: Drop::Item(Items::RottenApple, 1),
        locks: &[
            Lock::Movement(&[Move::no_walljump(0, 4)]),
            Lock::Emote(Emotes::Wave),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_Sword_Shanks"),
        drop: Drop::Weapon(Weapons::SteelShanks),
        locks: &[Lock::Movement(&[Move::walljump(2, 1)])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Dance_Platform_Applause_Tunic_BananaKing"),
        drop: Drop::Tunic(Tunics::BananaKing),
        locks: &[
            Lock::Movement(&[Move::walljump(2, 1)]),
            Lock::Emote(Emotes::Applause),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Spirit_A01_HolyCentry"),
        drop: Drop::Spirit(Spirits::HolyCentry),
        locks: &[Lock::Movement(&[Move::walljump(2, 2)])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_TempleGardens",
        context: Context::Overworld("Chest_A01_TempleGardens_Loot_05"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[Lock::Movement(&[Move::walljump(2, 2)])],
    },
    // Abandoned Path
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Chest_A01_Graveyard_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 3)])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("A01_Graveyard_EmoteStatue_Aggressive"),
        drop: Drop::Emote(Emotes::Aggressive),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 2)])],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Pickup_Necklace"),
        drop: Drop::Item(Items::Necklace, 1),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 2)])],
    },
    // Uthas Temple
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Intro",
        context: Context::Overworld("Spirit_A01_LoveFlower"),
        drop: Drop::Spirit(Spirits::LoveFlower),
        locks: &[Lock::Movement(&[Move::walljump(1, 0)])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Intro",
        context: Context::Overworld("Chest_A01_Uthas_Loot_03"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Intro",
        context: Context::Overworld("Chest_A01_Uthas_Key_04"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Intro",
        context: Context::Overworld("Chest_A01_Uthas_Loot_02"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: &[Lock::Location("A02_ArcaneTunnels/A01_SmallShrine_Main")],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        context: Context::Overworld("Chest_A01_Uthas_Key_02"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_SouthEast",
        context: Context::Overworld("Chest_A01_Uthas_Key_01"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: &[Lock::Movement(&[
            Move::no_walljump(1, 0),
            Move::walljump(0, 1),
            Move::no_walljump(0, 3),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        context: Context::Overworld("Chest_A01_Uthas_Loot_04"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 1),
            Move::no_walljump(0, 3),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        context: Context::Overworld("Chest_A01_Uthas_Loot"),
        drop: Drop::Weapon(Weapons::PeaceKeepers),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 1),
            Move::no_walljump(0, 3),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        context: Context::Overworld("A01_Uthas_EmoteStatue_Photo"),
        drop: Drop::Emote(Emotes::Photo),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 1),
            Move::no_walljump(0, 3),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        context: Context::Overworld("Chest_A01_Uthas_Loot_01"),
        drop: Drop::Ability(Abilities::DoubleJump),
        locks: &[
            Lock::Movement(&[Move::walljump(0, 1), Move::no_walljump(0, 3)]),
            Lock::Item(Items::KeyHolyMaster),
        ],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_SouthWest",
        context: Context::Overworld("Chest_A01_Uthas_MasterKey"),
        drop: Drop::Item(Items::KeyHolyMaster, 1),
        locks: &[],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        context: Context::Overworld("A01_Uthas_EmoteStatue_Party"),
        drop: Drop::Emote(Emotes::Party),
        locks: &[Lock::Movement(&[Move::no_walljump(1, 1)])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        context: Context::Overworld("Chest_A01_Uthas_Loot_05"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: &[Lock::Movement(&[Move::no_walljump(1, 1)])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_BottomPassage",
        context: Context::Overworld("Chest_A01_Uthas_Key_03"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: &[Lock::Movement(&[
            Move::no_walljump(1, 4),
            Move::walljump(1, 2),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        context: Context::Overworld("Chest_A01_Uthas_Loot_06"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 1),
            Move::no_walljump(0, 3),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_Main",
        context: Context::Overworld("Chest_A01_Uthas_MasterKey2"),
        drop: Drop::Item(Items::KeyHolyMaster, 1),
        locks: &[
            Lock::Location("A02_ArcaneTunnels/A01_SmallShrine_EndPath"),
            Lock::Movement(&[Move::no_walljump(0, 2)]),
        ],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_EndPath",
        context: Context::Overworld("Chest_A01_Uthas_Key_05"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: &[Lock::Movement(&[
            Move::no_walljump(1, 3),
            Move::walljump(1, 0),
        ])],
    },
    Check {
        location: "A02_ArcaneTunnels/A01_SmallShrine_EndPath",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/Cinematics/UthasTempleEnd/Uthas_Temple_End_Controller",
        ),
        drop: Drop::Item(Items::KeyGodMaster, 1),
        locks: &[Lock::Item(Items::OldKey), Lock::Item(Items::KeyHolyMaster)],
    },
    Check {
        location: "A10_PenumbraTemple/A10_Entrance",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/Cinematics/InsideTemple/InsideTemple_Controller",
        ),
        drop: Drop::Item(Items::SanctuaryStone, 1),
        locks: &[],
    },
    // Tower
    // for most of these you can climb the tower and drop down
    // i can tell if you don't have a lot of movement this'll be tedious...
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("A01_Graveyard_EmoteStatue_No"),
        drop: Drop::Emote(Emotes::No),
        locks: &[Lock::Location("A10_PenumbraTemple/A10_Entrance")],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Pickup_Rose2"),
        drop: Drop::Item(Items::Rose, 1),
        locks: &[
            Lock::Location("A10_PenumbraTemple/A10_Entrance"),
            Lock::Movement(&[Move::walljump(0, 2), Move::no_walljump(0, 3)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Pickup_BremurPicture"),
        drop: Drop::Item(Items::BremurPicture, 1),
        locks: &[
            Lock::Location("A10_PenumbraTemple/A10_Entrance"),
            Lock::Movement(&[Move::walljump(0, 1), Move::no_walljump(0, 2)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Chest_A01_Graveyard_IceDestroyers"),
        drop: Drop::Weapon(Weapons::IceDestroyers),
        locks: &[
            Lock::Location("A10_PenumbraTemple/A10_Entrance"),
            Lock::Movement(&[Move::walljump(1, 1), Move::no_walljump(4, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Chest_A02_Tunic_HolyAttire"),
        drop: Drop::Tunic(Tunics::OnopCoat),
        locks: &[
            Lock::Location("A10_PenumbraTemple/A10_Entrance"),
            Lock::Movement(&[Move::walljump(1, 1), Move::no_walljump(4, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Pickup_Rose"),
        drop: Drop::Item(Items::Rose, 1),
        locks: &[
            Lock::Location("A10_PenumbraTemple/A10_Entrance"),
            Lock::Movement(&[Move::walljump(1, 1), Move::no_walljump(4, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Chest_A01_Graveyard_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[
            Lock::Location("A10_PenumbraTemple/A10_Entrance"),
            Lock::Movement(&[Move::walljump(1, 1), Move::no_walljump(4, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Chest_A01_Graveyard_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[
            Lock::Location("A10_PenumbraTemple/A10_Entrance"),
            Lock::Movement(&[Move::walljump(1, 1), Move::no_walljump(4, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Overworld("Dance_Platform_Levitation_Chest"),
        drop: Drop::Item(Items::SeagulSoup, 1),
        locks: &[
            Lock::Location("A10_PenumbraTemple/A10_Entrance"),
            Lock::Emote(Emotes::Levitation),
            Lock::Movement(&[Move::walljump(1, 1), Move::no_walljump(4, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/MUSIC_Onops/Onop_Musicians/NPC_Onop_IO_Bech",
        ),
        drop: Drop::Ore(500),
        locks: &[
            Lock::Location("A06_IronCaves/A06_RustCity"),
            Lock::Item(Items::ComposerLetter),
            Lock::Movement(&[Move::walljump(1, 1), Move::no_walljump(4, 0)]),
        ],
    },
    Check {
        location: "A01_StoneHeartCity/A01_Graveyard",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/Cinematics/VesselVon/VesselVon_Controller",
        ),
        drop: Drop::Item(Items::BeiraVessel, 1),
        locks: &[
            Lock::Location("A10_PenumbraTemple/A10_Entrance"),
            Lock::Movement(&[Move::walljump(1, 1), Move::no_walljump(4, 0)]),
        ],
    },
    // Firefall River
    Check {
        location: "A06_IronCaves/A06_FireFall_A",
        context: Context::Overworld("Dance_Platform_HatKid_Chest"),
        drop: Drop::Weapon(Weapons::EmberTwins),
        locks: &[
            Lock::Emote(Emotes::HatKid),
            Lock::Movement(&[Move::no_walljump(1, 4)]),
        ],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_A",
        context: Context::Overworld("Pickup_A06_SRL4"),
        drop: Drop::Ore(250),
        locks: &[Lock::Movement(&[Move::no_walljump(1, 4)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_A",
        context: Context::Overworld("Pickup_A06_SRL5"),
        drop: Drop::Ore(200),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_A",
        context: Context::Overworld("Chest_A06_River_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 0),
            Move::no_walljump(0, 1),
        ])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_A",
        context: Context::Overworld("Chest_A06_River_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 3)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_A",
        context: Context::Overworld("Pickup_A06_SRL2"),
        drop: Drop::Ore(200),
        locks: &[Lock::Movement(&[Move::no_walljump(1, 3)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_A",
        context: Context::Overworld("Pickup_A06_SRL"),
        drop: Drop::Ore(300),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 3)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_A",
        context: Context::Overworld("Pickup_A06_SRL3"),
        drop: Drop::Ore(300),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 3)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_A",
        context: Context::Overworld("Pickup8"),
        drop: Drop::Ore(200),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 3)])],
    },
    // i can't find pickups 11, 12 and 6
    Check {
        location: "A06_IronCaves/A06_FireFall_B",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/Onop_Speedo/NPC_Onop_Speedo",
        ),
        drop: Drop::Ability(Abilities::Sprint),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_B",
        context: Context::Overworld("Pickup9"),
        drop: Drop::Ore(150),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_B",
        context: Context::Overworld("Dance_Platform_Windmill_Chest"),
        drop: Drop::Item(Items::RareCheese, 1),
        locks: &[
            Lock::Emote(Emotes::Windmill),
            Lock::Movement(&[Move::no_walljump(1, 4)]),
        ],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_B",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/Onop_Onari/NPC_Master_Onari",
        ),
        drop: Drop::Tunic(Tunics::BunnySuit),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_B",
        context: Context::Overworld("Chest_A06_River_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_B",
        context: Context::Overworld("Pickup10"),
        drop: Drop::Ore(150),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_B",
        context: Context::Overworld("A06_Firefall_EmoteStatue_KungFu"),
        drop: Drop::Emote(Emotes::KungFu),
        locks: &[Lock::Movement(&[Move::no_walljump(0, 1)])],
    },
    Check {
        location: "A06_IronCaves/A06_FireFall_B",
        context: Context::Cutscene("Blue Fire/Content/BlueFire/NPC/Mira_Mia/NPC_Mia_Firefall"),
        drop: Drop::Ore(5000),
        locks: &[Lock::Location("A06_IronCaves/A06_RustCity")],
    },
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Overworld("Pickup58"),
        drop: Drop::Ore(150),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Overworld("Dance_Platform_Party_Chest_Spirit_HammerKing"),
        drop: Drop::Spirit(Spirits::HammerKing),
        locks: &[
            Lock::Emote(Emotes::Party),
            Lock::Movement(&[Move::no_walljump(1, 0)]),
        ],
    },
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Overworld("Spirit_A06_FlyingOnop"),
        drop: Drop::Spirit(Spirits::FlyingOnop),
        locks: &[Lock::Location("A06_IronCaves/A06_RustCity")],
    },
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Overworld("Pickup2"),
        drop: Drop::Ore(250),
        locks: &[],
    },
    // myurder
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Cutscene("Blue Fire/Content/BlueFire/NPC/Onops/Onop_Codi/NPC_Onop_Codi"),
        drop: Drop::Item(Items::HouseKey, 1),
        locks: &[Lock::Location("A06_IronCaves/A06_RustCity")],
    },
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/MUSIC_Onops/Onop_Musicians/NPC_Onop_IO_Viveldi",
        ),
        drop: Drop::Ore(500),
        locks: &[
            Lock::Location("A06_IronCaves/A06_RustCity"),
            Lock::Item(Items::ComposerLetter),
        ],
    },
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Overworld("Chest_A06_Lake_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Overworld("A06_Firefall_EmoteStatue_Triceps"),
        drop: Drop::Emote(Emotes::Triceps),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Overworld("Chest_A06_Lake_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_LakeMolva",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/NPC_Onop_SectMember_Tunic",
        ),
        drop: Drop::Tunic(Tunics::SectMember),
        locks: &[Lock::Movement(&[
            Move::walljump(0, 1),
            Move::no_walljump(2, 0),
        ])],
    },
    Check {
        location: "A06_IronCaves/A06_SteamHouse_Core",
        context: Context::Cutscene("Blue Fire/Content/BlueFire/NPC/Mira_Mia/NPC_Mira"),
        drop: Drop::Item(Items::KeySteam, 1),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_SteamHouse_Core",
        context: Context::Overworld("Chest_A06_SteamHouse_Tunic_SteamSuit"),
        drop: Drop::Tunic(Tunics::SteamWorkerTunic),
        locks: &[Lock::Movement(&[Move::no_walljump(1, 1)])],
    },
    Check {
        location: "A06_IronCaves/A06_SteamHouse_Corridor",
        context: Context::Cutscene("Blue Fire/Content/BlueFire/NPC/Mira_Mia/NPC_Mia"),
        drop: Drop::Weapon(Weapons::IronJustice),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/Cinematics/RustCity_Von/RustCityVon_Controller",
        ),
        drop: Drop::Item(Items::KeyFireMaster, 1),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Overworld("Pickup21"),
        drop: Drop::Ore(400),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Overworld("A06_RustCity_EmoteStatue_Applause"),
        drop: Drop::Emote(Emotes::Applause),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Cutscene("Blue Fire/Content/BlueFire/NPC/Onops/Onop_Mon/NPC_Onop_Mon"),
        drop: Drop::Item(Items::HouseContract, 1),
        locks: &[Lock::Item(Items::HouseKey)],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Overworld("Dance_Platform_No_Chest"),
        drop: Drop::Tunic(Tunics::Pumpkin),
        locks: &[Lock::Emote(Emotes::No)],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/MUSIC_Onops/NPC_Onop_Compositor",
        ),
        drop: Drop::Item(Items::ComposerLetter, 4),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/MUSIC_Onops/NPC_Onop_Compositor_Ready",
        ),
        drop: Drop::Tunic(Tunics::PerformerCostume),
        // just Bech's requirements since everyone else is accessible
        locks: &[Lock::Movement(&[
            Move::walljump(1, 1),
            Move::no_walljump(4, 0),
        ])],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Cutscene(
            "Blue Fire/Content/BlueFire/NPC/Onops/Onop_Barri/NPC_Master_BarriStage2",
        ),
        drop: Drop::Weapon(Weapons::KinaDefenders),
        locks: &[Lock::Item(Items::Rose)],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Cutscene("Blue Fire/Content/BlueFire/NPC/Onops/Onop_Nuno/NPC_Onop_Nuno"),
        drop: Drop::Ore(3000),
        locks: &[Lock::Item(Items::RareSnow)],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Shop(Shop::Poti, 0, 3000),
        drop: Drop::Spirit(Spirits::OnopSiblings),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Shop(Shop::Poti, 1, 2200),
        drop: Drop::Spirit(Spirits::MoiTheDreadful),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Shop(Shop::Poti, 2, 12500),
        drop: Drop::Weapon(Weapons::ShadowCasters),
        locks: &[],
    },
    Check {
        location: "A06_IronCaves/A06_RustCity",
        context: Context::Shop(Shop::Poti, 3, 8000),
        drop: Drop::Item(Items::ExtraLargePouch, 1),
        locks: &[],
    },
];
