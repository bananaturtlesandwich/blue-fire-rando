use super::*;

pub fn write(
    cases: Vec<Check>,
    app: &crate::Rando,
    pak: &repak::PakReader,
    mod_pak: &Mod,
) -> Result<(), Error> {
    if cases.is_empty() {
        return Ok(());
    }
    let mut angels = open_slice(
        include_bytes!("../blueprints/angel_hook.uasset"),
        include_bytes!("../blueprints/angel_hook.uexp"),
    )?;
    let mut bremur = open_slice(
        include_bytes!("../blueprints/bremur_hook.uasset"),
        include_bytes!("../blueprints/bremur_hook.uexp"),
    )?;
    let mut paulale = open_slice(
        include_bytes!("../blueprints/paulale_hook.uasset"),
        include_bytes!("../blueprints/paulale_hook.uexp"),
    )?;
    let mut player = open_slice(
        include_bytes!("../blueprints/player_hook.uasset"),
        include_bytes!("../blueprints/player_hook.uexp"),
    )?;
    for Check { context, drop, .. } in cases {
        let Context::Specific(case, index) = context else {
            return Err(Error::Assumption)?;
        };
        create_hook(
            app,
            pak,
            mod_pak,
            match case {
                Case::Bremur => &mut bremur,
                Case::Paulale => &mut paulale,
                Case::Angels => &mut angels,
                Case::AllVoids => &mut player,
            },
            &drop,
            case.as_ref(),
            index,
        )?
    }
    Ok(())
}
