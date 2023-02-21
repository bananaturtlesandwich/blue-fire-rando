use super::*;

pub const CHECKS: [Check; 46] = [
    // Fire Keep
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
    // Arcane Tunnels
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Pickup3"),
        drop: Drop::Ore(250),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("A02_Arcane_EmoteStatue_Windmill"),
        drop: Drop::Emote(Emotes::Windmill),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 2),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Loot_03"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Dance_Platform_KungFu_Chest"),
        drop: Drop::Item(Items::Boot, 1),
        requirements: Some(&[&[Drop::Emote(Emotes::KungFu)]]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_CentralWaterWay_CenterAccess",
        context: Context::Overworld("A02_Arcane_EmoteStatue_HatKid"),
        drop: Drop::Emote(Emotes::HatKid),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Sword_Bloodstorm"),
        drop: Drop::Weapon(Weapons::BloodstormBlades),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Spirit_A02_ToxicRat"),
        drop: Drop::Spirit(Spirits::ToxicRat),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_SouthArcane_Key_01"),
        drop: Drop::Item(Items::RubyOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Dance_Platform_Celebration_Chest"),
        drop: Drop::Item(Items::Rice, 1),
        requirements: Some(&[&[Drop::Emote(Emotes::Celebration)]]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_SouthArcane_Loot_01"),
        drop: Drop::Item(Items::RubyOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_SouthArcane",
        context: Context::Overworld("Chest_A02_SouthArcane_Key_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Pickup4"),
        drop: Drop::Ore(250),
        requirements: Some(&[
            &[
                Drop::Ability(Abilities::WallRun),
                Drop::Ability(Abilities::DoubleJump),
                Drop::Ability(Abilities::Dash),
            ],
            &[
                Drop::Ability(Abilities::WallRun),
                Drop::Ability(Abilities::SpinAttack),
                Drop::Ability(Abilities::Dash),
            ],
            &[
                Drop::Ability(Abilities::WallRun),
                Drop::Ability(Abilities::Spell),
                Drop::Ability(Abilities::Dash),
            ],
        ]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_NorthArcane",
        context: Context::Overworld("Chest_A02_NorthArcane_Loot_04"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: Some(&[
            &[
                Drop::Ability(Abilities::WallRun),
                Drop::Ability(Abilities::DoubleJump),
                Drop::Ability(Abilities::Dash),
            ],
            &[
                Drop::Ability(Abilities::WallRun),
                Drop::Ability(Abilities::SpinAttack),
                Drop::Ability(Abilities::Dash),
            ],
            &[
                Drop::Ability(Abilities::WallRun),
                Drop::Ability(Abilities::Spell),
                Drop::Ability(Abilities::Dash),
            ],
        ]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A01_Arcane_Spell"),
        drop: Drop::Ability(Abilities::Spell),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_01"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_02"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_SRF"),
        drop: Drop::Ore(200),
        requirements: None,
    },
    // noooo why and how are they named the same? also uassetgui fails to open this map
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_SRF"),
        drop: Drop::Ore(300),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 0),
        drop: Drop::Spirit(Spirits::StormCentry),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 1),
        drop: Drop::Spirit(Spirits::BloodPhantom),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 2),
        drop: Drop::Spirit(Spirits::FrozenSoul),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Shop(Shop::SpiritHunter, 3),
        drop: Drop::Spirit(Spirits::ShadowGru),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_05"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_04"),
        drop: Drop::Item(Items::SapphireOre, 1),
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_Arcane",
        context: Context::Overworld("Duck"),
        drop: Drop::Duck,
        requirements: None,
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Chest_A02_EastArcane_Loot_03"),
        drop: Drop::Item(Items::EmeraldOre, 1),
        requirements: Some(&[
            &[Drop::Ability(Abilities::Dash)],
            &[Drop::Ability(Abilities::DoubleJump)],
            &[Drop::Ability(Abilities::SpinAttack)],
        ]),
    },
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_Arcane_SR_Loot"),
        drop: Drop::Ore(400),
        requirements: Some(&[
            &[Drop::Ability(Abilities::Dash)],
            &[Drop::Ability(Abilities::DoubleJump)],
            &[Drop::Ability(Abilities::SpinAttack)],
        ]),
    },
    // AND THESE ONES ARE NAMED FINE???
    Check {
        location: "A02_ArcaneTunnels/A02_EastArcane",
        context: Context::Overworld("Pickup_A02_Arcane_SR_Loot2"),
        drop: Drop::Ore(400),
        requirements: Some(&[
            &[Drop::Ability(Abilities::Dash)],
            &[Drop::Ability(Abilities::DoubleJump)],
            &[Drop::Ability(Abilities::SpinAttack)],
        ]),
    },
];
