use super::*;

pub fn write(cutscenes: Vec<Check>, app: &crate::Rando, pak: &unpak::Pak) -> Result<(), Error> {
    std::thread::scope(|thread| {
        for Check { context, drop, .. } in cutscenes {
            thread.spawn(move || -> Result<(), Error> {
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
                )?;
                Ok(())
            });
        }
    });
    Ok(())
}
