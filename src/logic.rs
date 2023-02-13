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
    Item,
    Weapon,
    Tunic,
    Spirit,
    Life,
    SpiritSlot,
    Ability,
    Emote,
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
                drop: Drop::Emote,
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
                        drop: Drop::Item,
                        requires: &[],
                    },
                    Check {
                        context: Context::Overworld("Chest_A01_Keep_Shield"),
                        drop: Drop::Item,
                        requires: &[],
                    },
                ],
            }],
            requires: &[],
            checks: &[
                Check {
                    context: Context::Overworld("Chest_A01_TempleGardens_Ability_SpinAttack2"),
                    drop: Drop::Item,
                    requires: &[],
                },
                Check {
                    context: Context::Overworld("Chest_A02_Keep_Loot_02"),
                    drop: Drop::Item,
                    requires: &[],
                },
            ],
        },
        Location {
            path: "/Game/BlueFire/Maps/World/A02_ArcaneTunnels/A02_GameIntro_FirstVoidRoom",
            connects_to: &[],
            requires: &[&[Drop::Item]],
            checks: &[
                Check {
                    context: Context::Overworld("A01_FireKeep_EmoteStatue_Techno"),
                    drop: Drop::Emote,
                    requires: &[],
                },
                Check {
                    context: Context::Overworld("Spirit_A02_RiverSpirit"),
                    drop: Drop::Spirit,
                    requires: &[&[Drop::Ability], &[Drop::Spirit]],
                },
            ],
        },
    ],
    requires: &[],
    checks: &[],
};

fn randomise(app: &super::Rando) {
    todo!(
        r"
    finish off fire keep to test the whole thing
    dump all enums and give them as fields to Drop
    tidy everything into their own files
    "
    );
}
