#![allow(dead_code)]
mod drops;
pub use drops::*;

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

enum Drop {
    Item(Items),
    Weapon(Weapons),
    Tunic(Tunics),
    Spirit(Spirits),
    Life,
    SpiritSlot,
    Ability(Abilities),
    Emote(Emotes),
}

struct Check {
    context: Context,
    drop: Drop,
    requires: &'static [&'static [Drop]],
}

struct Location {
    path: &'static str,
    connects_to: &'static [Location],
    requires: &'static [&'static [Drop]],
    checks: &'static [Check],
}

const DATA: Location = Location {
    path: "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_KeepSouth",
    connects_to: &[
        Location {
            path: "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_Exterior",
            connects_to: &[],
            requires: &[],
            checks: &[Check {
                context: Context::Overworld("A01_FireKeep_EmoteStatue_Levitation"),
                drop: Drop::Emote(Emotes::Levitation),
                requires: &[],
            }],
        },
        Location {
            path: "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_KeepEast",
            connects_to: &[Location {
                path: "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_EastWing",
                connects_to: &[],
                requires: &[],
                checks: &[
                    Check {
                        context: Context::Overworld("Chest_A02_Keep_Key_01"),
                        drop: Drop::Item(Items::OldKey),
                        requires: &[],
                    },
                    Check {
                        context: Context::Overworld("Chest_A01_Keep_Shield"),
                        drop: Drop::Ability(Abilities::Block),
                        requires: &[],
                    },
                ],
            }],
            requires: &[],
            checks: &[
                Check {
                    context: Context::Overworld("Chest_A01_TempleGardens_Ability_SpinAttack2"),
                    drop: Drop::Item(Items::SapphireOre),
                    requires: &[],
                },
                Check {
                    context: Context::Overworld("Chest_A02_Keep_Loot_02"),
                    drop: Drop::Item(Items::SapphireOre),
                    requires: &[],
                },
            ],
        },
        Location {
            path: "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
            connects_to: &[Location {
                path: "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_KeepWest",
                connects_to: &[Location {
                    path: "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_MemorialMain",
                    connects_to: &[
                        // into arcane tunnels
                    ],
                    requires: &[],
                    checks: &[
                        Check {
                            context: Context::Overworld("Chest_A02_GameIntro"),
                            drop: Drop::Item(Items::EmeraldOre),
                            requires: &[],
                        },
                        Check {
                            context: Context::Overworld("Chest_A02_Sword_DiamondWings"),
                            drop: Drop::Weapon(Weapons::DiamondWings),
                            requires: &[],
                        },
                        Check {
                            context: Context::Overworld("Dance_Platform_Photo_Chest"),
                            drop: Drop::Item(Items::Mandoline),
                            requires: &[&[Drop::Emote(Emotes::Photo)]],
                        },
                    ],
                }],
                requires: &[],
                checks: &[Check {
                    context: Context::Overworld("Chest_A02_Keep_Loot_01"),
                    drop: Drop::Item(Items::SapphireOre),
                    requires: &[],
                }],
            }],
            requires: &[&[Drop::Item(Items::OldKey)]],
            checks: &[
                Check {
                    context: Context::Overworld("A01_FireKeep_EmoteStatue_Techno"),
                    drop: Drop::Emote(Emotes::Techno),
                    requires: &[],
                },
                Check {
                    context: Context::Overworld("Spirit_A02_RiverSpirit"),
                    drop: Drop::Spirit(Spirits::RiverSpirit),
                    requires: &[
                        &[Drop::Ability(Abilities::Dash)],
                        &[Drop::Ability(Abilities::WallRun)],
                        &[
                            Drop::Ability(Abilities::DoubleJump),
                            Drop::Ability(Abilities::SpinAttack),
                        ],
                    ],
                },
            ],
        },
    ],
    requires: &[],
    checks: &[],
};

/*
i think i'll try to refactor to this format
hashbrown::HashMap{
    (
        "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_KeepSouth",
        Location {
            unlocks: &[
                "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_Exterior",
                "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_KeepEast",
                "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
            ],
            requires: &[],
            checks: &[],
        }
    )
}
this decreases nesting, allows multiple maps to connect to other ones and would be more efficient for the algorithm i have in mind
*/

fn randomise(app: &super::Rando) {
    todo!(
        r"
    put the fire keep data in the new format
    tidy everything into their own files
    "
    );
}
