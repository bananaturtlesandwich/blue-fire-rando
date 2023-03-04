use super::*;

pub const CHECKS: [Check; 103] = [
    // Fire Keep
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepSouth",
        context: Context::Starting,
        drop: Drop::Emote(Emotes::Hello2),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepSouth",
        context: Context::Starting,
        drop: Drop::Ability(Abilities::Dash),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_Exterior",
        context: Context::Overworld("A01_FireKeep_EmoteStatue_Levitation"),
        drop: Drop::Emote(Emotes::Levitation),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_Exterior",
        context: Context::Cutscene(
            "/Game/BlueFire/NPC/Onops/MUSIC_Onops/Onop_Musicians/NPC_Onop_IO_Bitoven",
        ),
        drop: Drop::Ore(500),
        locks: Some(&[
            Lock::Location("A06_RustCity"),
            Lock::Item(Items::ComposerLetter),
        ]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro",
        context: Context::Overworld("Duck"),
        drop: Drop::Duck,
        locks: Some(&[Lock::Movement(&[Move::no_walljump(5, 0)])]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        context: Context::Overworld("Chest_A01_TempleGardens_Ability_SpinAttack2"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        context: Context::Overworld("Chest_A02_Keep_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepEast",
        context: Context::Overworld("Pickup"),
        drop: Drop::Ore(250),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_EastWing",
        context: Context::Overworld("Chest_A02_Keep_Key_01"),
        drop: Drop::Item(Items::OldKey, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_EastWing",
        context: Context::Overworld("Chest_A01_Keep_Shield"),
        drop: Drop::Ability(Abilities::Block),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
        context: Context::Overworld("A01_FireKeep_EmoteStatue_Techno"),
        drop: Drop::Emote(Emotes::Techno),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
        context: Context::Overworld("Spirit_A02_RiverSpirit"),
        drop: Drop::Spirit(Spirits::RiverSpirit),
        locks: Some(&[Lock::Movement(&[
            Move::no_walljump(0, 2),
            Move::no_walljump(5, 0),
            Move::walljump(3, 0),
        ])]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_KeepWest",
        context: Context::Overworld("Chest_A02_Keep_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("Chest_A02_GameIntro"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("Chest_A02_Sword_DiamondWings"),
        drop: Drop::Weapon(Weapons::DiamondWings),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("A02_FireKeep_EmoteStatue_Celebration"),
        drop: Drop::Emote(Emotes::Celebration),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
        context: Context::Overworld("Dance_Platform_Photo_Chest"),
        drop: Drop::Item(Items::Mandoline, 1),
        locks: Some(&[Lock::Emote(Emotes::Photo)]),
    },
    // Arcane Tunnels
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Pickup3"),
        drop: Drop::Ore(250),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("A02_Arcane_EmoteStatue_Windmill"),
        drop: Drop::Emote(Emotes::Windmill),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 2),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Dance_Platform_KungFu_Chest"),
        drop: Drop::Item(Items::Boot, 1),
        locks: Some(&[Lock::Emote(Emotes::KungFu)]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess",
        context: Context::Overworld("A02_Arcane_EmoteStatue_HatKid"),
        drop: Drop::Emote(Emotes::HatKid),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Sword_Bloodstorm"),
        drop: Drop::Weapon(Weapons::BloodstormBlades),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Spirit_A02_ToxicRat"),
        drop: Drop::Spirit(Spirits::ToxicRat),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_SouthArcane_Key_01"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Dance_Platform_Celebration_Chest"),
        drop: Drop::Item(Items::Rice, 1),
        locks: Some(&[Lock::Emote(Emotes::Celebration)]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_SouthArcane_Loot_01"),
        drop: Drop::Item(Items::RubyOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_SouthArcane_Key_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Pickup4"),
        drop: Drop::Ore(250),
        locks: Some(&[Lock::Movement(&[Move::walljump(0, 4)])]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Loot_04"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: Some(&[Lock::Movement(&[Move::walljump(0, 4)])]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A01_Arcane_Spell"),
        drop: Drop::Ability(Abilities::Spell),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_SRF"),
        drop: Drop::Ore(200),
        locks: None,
    },
    // noooo why and how are they named the same? also uassetgui fails to open this map
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_SRF"),
        drop: Drop::Ore(300),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 0),
        drop: Drop::Spirit(Spirits::StormCentry),
        locks: Some(&[Lock::Item(Items::SmallPouch)]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 1),
        drop: Drop::Spirit(Spirits::BloodPhantom),
        locks: Some(&[Lock::Item(Items::SmallPouch)]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 2),
        drop: Drop::Spirit(Spirits::FrozenSoul),
        locks: Some(&[Lock::Item(Items::SmallPouch)]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 3),
        drop: Drop::Spirit(Spirits::ShadowGru),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_05"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_04"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_Arcane",
        context: Context::Overworld("Duck"),
        drop: Drop::Duck,
        locks: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_03"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        locks: Some(&[Lock::Movement(&[Move::no_walljump(0, 1)])]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_Arcane_SR_Loot"),
        drop: Drop::Ore(400),
        locks: Some(&[Lock::Movement(&[Move::no_walljump(0, 1)])]),
    },
    // AND THESE ONES ARE NAMED FINE???
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_Arcane_SR_Loot2"),
        drop: Drop::Ore(400),
        locks: Some(&[Lock::Movement(&[Move::no_walljump(0, 1)])]),
    },
    // Crossroads
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup49"),
        drop: Drop::Ore(50),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup47"),
        drop: Drop::Ore(50),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Check_A01_CrossRoads_Loot"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: Some(&[
            Lock::Movement(&[Move::no_walljump(1, 0)]),
            Lock::Movement(&[Move::no_walljump(0, 2)]),
            Lock::Movement(&[Move::walljump(0, 0)]),
        ]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Chest_A01_Well_SpinAttack"),
        drop: Drop::Ability(Abilities::SpinAttack),
        locks: Some(&[
            Lock::Movement(&[Move::no_walljump(1, 0)]),
            Lock::Movement(&[Move::no_walljump(0, 2)]),
            Lock::Movement(&[Move::walljump(0, 0)]),
        ]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_Well",
        context: Context::Overworld("Pickup57"),
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
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup14"),
        drop: Drop::Ore(50),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup15"),
        drop: Drop::Ore(50),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup11"),
        drop: Drop::Ore(50),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup12"),
        drop: Drop::Ore(50),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup56"),
        drop: Drop::Ore(50),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Chest_A01_CrossRoads_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Pickup50"),
        drop: Drop::Ore(50),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Chest_A01_CrossRoads_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: Some(&[Lock::Movement(&[Move::no_walljump(0, 1)])]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Dance_Platform_Techno_Chest"),
        drop: Drop::Tunic(Tunics::Galaxy),
        locks: Some(&[
            Lock::Emote(Emotes::Techno),
            Lock::Movement(&[Move::no_walljump(0, 1)]),
        ]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CrossRoads",
        context: Context::Overworld("Chest_A01_CrossRoads_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    // Stoneheart City
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_SandRelic"),
        drop: Drop::Item(Items::SandRelic, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup51"),
        drop: Drop::Ore(100),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_A01_Stoneheart_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 2),
        locks: Some(&[Lock::Movement(&[
            Move::walljump(2, 0),
            Move::no_walljump(4, 0),
        ])]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_A01_City_SRL3"),
        drop: Drop::Ore(400),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 0),
        drop: Drop::Tunic(Tunics::Orange),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 1),
        drop: Drop::Tunic(Tunics::Aqua),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 2),
        drop: Drop::Tunic(Tunics::LightBlue),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 3),
        drop: Drop::Tunic(Tunics::Lila),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 4),
        drop: Drop::Tunic(Tunics::Rainbow),
        locks: Some(&[Lock::Item(Items::SmallPouch)]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 5),
        drop: Drop::Tunic(Tunics::Empty),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 6),
        drop: Drop::Tunic(Tunics::Violet),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 7),
        drop: Drop::Tunic(Tunics::Grey),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 8),
        drop: Drop::Tunic(Tunics::Green),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 9),
        drop: Drop::Tunic(Tunics::Yellow),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Ari, 10),
        drop: Drop::Tunic(Tunics::Red),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup27"),
        drop: Drop::Ore(200),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_A01_City_SRL2"),
        drop: Drop::Ore(300),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Cutscene("/Game/BlueFire/NPC/Orip/BP_Orip_Saw/NPC_Orip_Stoneheart"),
        drop: Drop::Item(Items::OddRock, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_A01_City_SRL"),
        drop: Drop::Ore(150),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_45"),
        drop: Drop::Ore(150),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_A01_Stoneheart_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_46"),
        drop: Drop::Ore(100),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_54"),
        drop: Drop::Ore(100),
        locks: None,
    },
    // the cutest chest in existence
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_A01_Stoneheart_Loot"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 0),
        drop: Drop::Spirit(Spirits::PossesedBook),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 1),
        drop: Drop::Spirit(Spirits::GoldenLust),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 2),
        drop: Drop::Spirit(Spirits::LifeSteal),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 3),
        drop: Drop::Item(Items::LargePouch, 1),
        locks: Some(&[Lock::Item(Items::SmallPouch)]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Shop(Shop::Mork, 4),
        drop: Drop::Item(Items::RareSnow, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book3"),
        drop: Drop::Item(Items::Book, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book"),
        drop: Drop::Item(Items::Book, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Dance_Platform_Triceps_Chest"),
        drop: Drop::Item(Items::IceCrystal, 1),
        locks: Some(&[Lock::Emote(Emotes::Triceps)]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Spirit_A01_FarasGrace"),
        drop: Drop::Spirit(Spirits::FarasGrace),
        locks: Some(&[Lock::Item(Items::Necklace)]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_29"),
        drop: Drop::Ore(150),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_29"),
        drop: Drop::Ore(100),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book4"),
        drop: Drop::Item(Items::Book, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book5"),
        drop: Drop::Item(Items::Book, 1),
        locks: Some(&[Lock::Movement(&[
            Move::walljump(0, 0),
            Move::no_walljump(1, 0),
        ])]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Chest_A01_Stoneheart_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        locks: Some(&[Lock::Movement(&[
            Move::walljump(0, 0),
            Move::no_walljump(1, 0),
        ])]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Overworld("Pickup_Book2"),
        drop: Drop::Item(Items::Book, 1),
        locks: None,
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Cutscene("/Game/BlueFire/NPC/Merchant/NPC_Merchant"),
        drop: Drop::Tunic(Tunics::MerchantsRobe),
        locks: Some(&[Lock::Item(Items::Book)]),
    },
    Check {
        location: "A01_StoneHeartCity/A01_CliffPath",
        context: Context::Cutscene(
            "/Game/BlueFire/NPC/Onops/MUSIC_Onops/Onop_Musicians/NPC_Onop_IO_Wolfgang",
        ),
        drop: Drop::Ore(500),
        locks: Some(&[
            Lock::Location("A06_RustCity"),
            Lock::Item(Items::ComposerLetter),
        ]),
    },
];
