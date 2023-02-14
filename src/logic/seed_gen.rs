const BEGINNING: &'static str = "A02_ArcaneTunnels/A02_GameIntro_KeepSouth";

pub fn randomise(app: &crate::Rando) {
    let possible_checks: Vec<&super::Check> = super::DATA
        .values()
        .map(|loc| loc.checks)
        .flatten()
        .collect();
}
