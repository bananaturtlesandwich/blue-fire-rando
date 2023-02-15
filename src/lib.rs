mod logic;

#[derive(Default)]
pub struct Rando {
    item: bool,
    weapons: bool,
    tunics: bool,
    spirits: bool,
    abilities: bool,
    emotes: bool,
    treasure: bool,
    starting: bool,
    ore: bool,
    ducks: bool,
    surprise: bool,
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
                ui[1].checkbox(&mut self.starting, "Dash -===(    - _ o)");
                ui[1].checkbox(&mut self.ore, "Ore  (    $ o $)");
                ui[1].checkbox(&mut self.ducks, "Ducks <(⭕ ◑ ө ◑ ⭕)>");
                ui[1].checkbox(&mut self.surprise, "Surprise... >:p");
                ui[1].horizontal(|ui| ui.label("share rando_p.pak for races"));
            });
            if ui
                .button(egui::RichText::new("start rando").strong().size(70.0))
                .with_new_rect(ui.max_rect())
                .clicked()
            {
                logic::randomise(&self)
            }
        });
    }
}
