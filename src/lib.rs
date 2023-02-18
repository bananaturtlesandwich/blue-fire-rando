mod logic;
mod actor;

pub struct Rando {
    dialog: egui_modal::Modal,
    pak: std::path::PathBuf,
    item: bool,
    weapons: bool,
    tunics: bool,
    spirits: bool,
    abilities: bool,
    emotes: bool,
    treasure: bool,
    dash: bool,
    ore: bool,
    ducks: bool,
}

impl Rando {
    pub fn new(ctx: &eframe::CreationContext) -> Self {
        let get_bool = |key: &str| -> bool {
            ctx.storage
                .map(|storage| {
                    storage
                        .get_string(key)
                        .unwrap_or_default()
                        .parse()
                        .unwrap_or_default()
                })
                .unwrap_or_default()
        };
        Self {
            dialog: egui_modal::Modal::new(&ctx.egui_ctx, "dialog"),
            pak: match ctx.storage.and_then(|storage| storage.get_string("pak")){
                Some(path) => path.into(),
                None => loop {
                    let Some(path) = rfd::FileDialog::new().set_title("Please select where you have Blue Fire installed").pick_folder() else {
                        continue
                    };
                    if !path.ends_with("Blue Fire") || path.ends_with("Blue Fire/Blue Fire"){
                        continue;
                    }
                    break path;
                }
                .join("Blue Fire/Content/Paks")
            },
            item: get_bool("item"),
            weapons: get_bool("weapons"),
            tunics: get_bool("tunics"),
            spirits: get_bool("spirits"),
            abilities: get_bool("abilities"),
            emotes: get_bool("emotes"),
            treasure: get_bool("treasure"),
            dash: get_bool("dash"),
            ore: get_bool("ore"),
            ducks: get_bool("ducks"),
        }
    }
}

impl eframe::App for Rando {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        use eframe::egui;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    egui::RichText::new("Blue Fire Rando")
                        .underline()
                        .size(40.0),
                );
                ui.label(egui::RichText::new("by spuds :p").italics().size(15.0));
            });
            ui.columns(2, |ui| {
                ui[0].heading(egui::RichText::new("Pool options").underline());
                ui[0].checkbox(&mut self.abilities, "Abilities e.g Nuos claw");
                ui[0].checkbox(&mut self.spirits, "Spirits e.g Aerial rat");
                ui[0].checkbox(&mut self.weapons, "Weapons e.g Dual blades");
                ui[0].checkbox(&mut self.emotes, "Emotes e.g Smug dance");
                ui[0].checkbox(&mut self.tunics, "Tunics e.g Bunny suit");
                ui[0].checkbox(&mut self.item, "Items e.g Ruby/Rose");

                ui[1].heading(egui::RichText::new("Extra options").underline());
                ui[1].checkbox(&mut self.treasure, "Treasures e.g Seagull soup");
                // since dash is an ability i might have to think more on how to do this - plus context is funky too
                ui[1].checkbox(&mut self.dash, "Dash -===(    - _ o)");
                ui[1].checkbox(&mut self.ore, "Ore  (    $ o $)");
                // ducks are typically located on the master map - maybe make it so room it's locked in unlocks master map
                ui[1].code("Ducks <(⭕ ◑ ө ◑ ⭕)>");
                ui[1].label("atm all the app does is make a spoiler log for fire keep");
                ui[1].label("share rando_p.pak for races");
            });
            if ui
                .button(egui::RichText::new("start rando").strong().size(70.0))
                .with_new_rect(ui.max_rect())
                .clicked()
            {
                match logic::randomise(self) {
                    Ok(()) => self.dialog.open_dialog(
                        Some("success"),
                        Some("the seed has been generated, written and installed - enjoy and have fun!"),
                        Some(egui_modal::Icon::Success),
                    ),
                    Err(e) => self.dialog.open_dialog(
                        Some("whoopsie"),
                        Some(e),
                        Some(egui_modal::Icon::Warning),
                    ),
                }
            }
            self.dialog.show_dialog();
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string("pak", self.pak.to_str().unwrap_or_default().to_string());
        storage.set_string("item", self.item.to_string());
        storage.set_string("weapons", self.weapons.to_string());
        storage.set_string("tunics", self.tunics.to_string());
        storage.set_string("spirits", self.spirits.to_string());
        storage.set_string("abilities", self.abilities.to_string());
        storage.set_string("emotes", self.emotes.to_string());
        storage.set_string("treasure", self.treasure.to_string());
        storage.set_string("dash", self.dash.to_string());
        storage.set_string("ore", self.ore.to_string());
        storage.set_string("ducks", self.ducks.to_string());
    }
}
