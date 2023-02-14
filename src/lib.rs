mod logic;

#[derive(Default)]
pub struct Rando {
    item: bool,
    weapon: bool,
    tunic: bool,
    amulet: bool,
    ability: bool,
    emote: bool,
    treasure: bool,
    starting: bool,
    life: bool,
    slot: bool,
    duck: bool,
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
                ui[0].checkbox(&mut self.ability, "Abilities e.g Nuos claw");
                ui[0].checkbox(&mut self.amulet, "Spirits e.g Aerial rat");
                ui[0].checkbox(&mut self.weapon, "Weapons e.g Dual blades");
                ui[0].checkbox(&mut self.emote, "Emotes e.g Hat kid smug");
                ui[0].checkbox(&mut self.tunic, "Tunics e.g Bunny suit");
                ui[0].checkbox(&mut self.item, "Items e.g Rose");

                ui[1].heading(egui::RichText::new("Extra options").underline());
                ui[1].checkbox(&mut self.treasure, "Treasures e.g Seagull soup");
                ui[1].checkbox(&mut self.starting, "Dash -===(  -_o)");
                ui[1].checkbox(&mut self.life, "Lives 💗");
                ui[1].checkbox(&mut self.slot, "Spirit slots 🍩");
                ui[1].checkbox(&mut self.duck, "Ducks <(⭕ ◑ ө ◑ ⭕)>");
                ui[1].checkbox(&mut self.surprise, "Surprise... (   ¬u¬)");
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
