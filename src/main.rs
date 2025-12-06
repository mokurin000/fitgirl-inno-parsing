use std::{error::Error, fs::OpenOptions};

fn main() -> Result<(), Box<dyn Send + Sync + Error>> {
    let inno = inno::Inno::new(
        OpenOptions::new()
            .read(true)
            .create(false)
            .open("payload/setup.exe")?,
    )?;

    let icon = inno
        .icons()
        .iter()
        .filter(|icon| {
            icon.name()
                .is_some_and(|name| name.starts_with("{commondesktop}"))
        })
        .filter_map(|icon| icon.file())
        .next();
    let run = inno
        .run_entries()
        .iter()
        .filter(|entry| {
            entry
                .description()
                .is_some_and(|desc| desc.starts_with("{cm:LaunchProgram,"))
        })
        .filter_map(|entry| entry.name())
        .next();
    println!("Icon: {icon:?}\nRun: {run:?}",);

    Ok(())
}
