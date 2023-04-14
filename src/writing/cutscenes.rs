use super::*;

pub fn write(cutscenes: Vec<Check>, app: &crate::Rando, pak: &unpak::Pak) -> Result<(), Error> {
    for Check { context, drop, .. } in cutscenes {
        let Context::Cutscene(cutscene) = context else {
            return Err(Error::Assumption);
        };
        create_hook(
            app,
            &pak,
            |_| {
                Ok(open_from_bytes(
                    include_bytes!("../blueprints/hook.uasset"),
                    include_bytes!("../blueprints/hook.uexp"),
                )?)
            },
            &drop,
            cutscene,
            69,
        )?
    }
    Ok(())
}
