use eframe::egui;

mod io;
mod logic;
mod map;
mod writing;

pub struct Rando {
    font: egui::FontDefinitions,
    notifs: egui_modal::Modal,
    pak: std::path::PathBuf,
    pak_str: String,
    autoupdate: bool,
    items: bool,
    gems: bool,
    keys: bool,
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

#[cfg(not(debug_assertions))]
const EXE: &str = "blue-fire-rando-drm-free.exe";

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

        let mut font = egui::FontDefinitions::default();
        font.font_data.insert(
            "cinzel".to_string(),
            egui::FontData::from_static(include_bytes!("Cinzel-Regular.ttf")),
        );
        font.families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "cinzel".to_string());

        let notifs = egui_modal::Modal::new(&ctx.egui_ctx, "dialog");
        let autoupdate = get_bool("autoupdate");

        #[cfg(not(debug_assertions))]
        if autoupdate {
            std::thread::spawn(update);
        }
        #[cfg(not(debug_assertions))]
        if std::fs::remove_file(format!("{EXE}.old")).is_ok() {
            notifs.open_dialog(
                Some("success"),
                Some(format!(
                    "successfully updated to {}",
                    env!("CARGO_PKG_VERSION")
                )),
                Some(egui_modal::Icon::Success),
            );
        }

        let pak = match ctx.storage.and_then(|storage| storage.get_string("pak")) {
            Some(path) => path.into(),
            None => loop {
                if let Some(pak) = ask_game_path() {
                    break pak;
                }
            },
        };
        let pak_str = get_pak_str(&pak);

        Self {
            font,
            notifs,
            pak,
            pak_str,
            autoupdate,
            items: get_bool("items"),
            gems: get_bool("gems"),
            keys: get_bool("keys"),
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

fn ask_game_path() -> Option<std::path::PathBuf> {
    let path = rfd::FileDialog::new()
        .set_title(
            "Select where you have Blue Fire installed (e.g C:/Amazon Games/Library/Blue Fire)",
        )
        .pick_folder()?;
    (path.ends_with("Blue Fire") && path.join("PROA34/Content/Paks").exists())
        .then(|| path.join("PROA34\\Content\\Paks"))
}

fn get_pak_str(pak: &std::path::PathBuf) -> String {
    let mut pak_str = pak.to_str().unwrap_or_default().to_string();
    pak_str.truncate(pak_str.len() - 13);
    pak_str = "...".to_string() + &pak_str[(pak_str.len() - 50).clamp(0, 1000)..];
    pak_str
}

#[cfg(not(debug_assertions))]
fn update() {
    let api = autoupdater::apis::github::GithubApi::new("bananaturtlesandwich", "blue-fire-rando")
        .current_version(env!("CARGO_PKG_VERSION"));
    if let Ok(Some(asset)) = api.get_newer(None::<autoupdater::Sort>) {
        use autoupdater::apis::DownloadApiTrait;
        if api
            .download(
                &asset
                    .assets
                    .into_iter()
                    .find(|asset| asset.name == EXE)
                    .unwrap(),
                None::<autoupdater::Download>,
            )
            .is_ok()
        {
            std::process::Command::new(EXE).spawn().unwrap();
            std::process::exit(0);
        }
    }
}

macro_rules! notify {
    ($self:expr, $result: expr, $message: literal) => {
        match $result {
            Ok(..) => $self.notifs.open_dialog(
                Some("success"),
                Some($message),
                Some(egui_modal::Icon::Success),
            ),
            Err(e) => $self
                .notifs
                .open_dialog(Some(":/"), Some(e), Some(egui_modal::Icon::Error)),
        }
    };
}

impl eframe::App for Rando {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        ctx.set_fonts(self.font.clone());
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    egui::RichText::new("Blue Fire Rando")
                        .underline()
                        .size(40.0),
                );
                ui.label(egui::RichText::new("by spuds :p").italics().size(15.0));
            });
            ui.horizontal(|ui|{
                ui.checkbox(&mut self.autoupdate, "autoupdate");
                ui.label(&self.pak_str);
                if ui.button("...").clicked(){
                    if let Some(pak) = ask_game_path(){
                        self.pak_str = get_pak_str(&pak);
                        self.pak = pak
                    } else {
                        self.notifs.open_dialog(
                            Some(":/"),
                            Some("that isn't a valid blue fire install location"),
                            Some(egui_modal::Icon::Warning)
                        )
                    }
                }
            });
            ui.columns(2, |ui| {
                ui[0].heading(egui::RichText::new("Pool options").underline());
                ui[0].checkbox(&mut self.abilities, "Abilities e.g Nuos claw");
                ui[0].checkbox(&mut self.spirits, "Spirits e.g Aerial rat");
                ui[0].checkbox(&mut self.weapons, "Weapons e.g Dual blades");
                ui[0].checkbox(&mut self.emotes, "Emotes e.g Smug dance");
                ui[0].checkbox(&mut self.tunics, "Tunics e.g Bunny suit");
                ui[0].checkbox(&mut self.items, "Items e.g Rose");
                ui[0].checkbox(&mut self.gems, "Gems e.g Void ore");
                ui[0].checkbox(&mut self.keys, "Keys e.g Old key");

                ui[1].heading(egui::RichText::new("Extra options").underline());
                ui[1].checkbox(&mut self.treasure, "Treasures e.g Seagull soup");
                ui[1].checkbox(&mut self.dash, "Dash -=====(    - _ o)");
                ui[1].checkbox(&mut self.ore, "Ore  (>    $ o $)>");
                ui[1].checkbox(&mut self.ducks, "Ducks <(⭕ ◑ ө ◑ ⭕)>");
                let size = ui[1].fonts(|fonts| {
                    fonts.glyph_width(&egui::TextStyle::Body.resolve(ui[1].style()), ' ')
                });
                ui[1].horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = size;
                    ui.label("chat about the rando on");
                    ui.hyperlink_to("discord", "https://discord.gg/bluefire");
                });
                ui[1].horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = size;
                    ui.label("share");
                    if ui.link("rando_p.pak").clicked() {
                        notify!(
                            self,
                            std::process::Command::new(
                                #[cfg(target_os = "windows")]
                                "explorer",
                                #[cfg(target_os = "macos")]
                                "open",
                                #[cfg(target_os = "linux")]
                                "xdg-open",
                            )
                            .arg(&self.pak)
                            .spawn(),
                            "share and put it in the same folder"
                        )
                    }
                    ui.label("to race!")
                });
                ui[1].label("installing should take about 15 seconds because a lot of data is procedurally written to disk")
            });
            ui.vertical_centered_justified(|ui| {
                if ui.button("uninstall seed").clicked() {
                    notify!(
                        self,
                        std::fs::remove_file(self.pak.join("rando_p.pak")),
                        "randomness has been removed from the game"
                    )
                }
                if ui.button("launch blue fire").clicked() {
                    notify!(
                        self,
                        std::process::Command::new(
                            self.pak
                                .join("../../Binaries/Win64/PROA34-Win64-Shipping.exe")
                        )
                        .spawn(),
                        "game found and launched successfully"
                    )
                }
                if ui
                    .button(
                        egui::RichText::new("generate and install seed")
                            .size(33.0),
                    )
                    .clicked()
                {
                    std::fs::remove_dir_all(self.pak.join("rando_p")).unwrap_or_default();
                    notify!(
                        self,
                        logic::randomise(self),
                        "seed has been generated, written and installed"
                    );
                    std::fs::remove_dir_all(self.pak.join("rando_p")).unwrap_or_default();
                }
            });
            self.notifs.show_dialog();
        });
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string("pak", self.pak.to_str().unwrap_or_default().to_string());
        storage.set_string("autoupdate", self.autoupdate.to_string());
        storage.set_string("items", self.items.to_string());
        storage.set_string("gems", self.gems.to_string());
        storage.set_string("keys", self.keys.to_string());
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
