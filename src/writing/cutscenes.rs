use super::*;

pub fn write(
    cutscenes: Vec<Check>,
    app: &crate::Rando,
    pak: &repak::PakReader,
    mod_pak: &Mod,
) -> Result<(), Error> {
    std::thread::scope(|thread| -> Result<(), Error> {
        let mut threads = Vec::with_capacity(cutscenes.len());
        for Check { context, drop, .. } in cutscenes {
            threads.push(thread.spawn(move || {
                let Context::Cutscene(cutscene) = context else {
                    return Err(Error::Assumption);
                };
                create_hook(
                    app,
                    pak,
                    mod_pak,
                    &mut open_slice(
                        include_bytes!("../blueprints/hook.uasset"),
                        include_bytes!("../blueprints/hook.uexp"),
                    )?,
                    &drop,
                    cutscene,
                    69,
                )?;
                Ok(())
            }));
        }
        for thread in threads {
            thread.join()??;
        }
        Ok(())
    })?;
    Ok(())
}
