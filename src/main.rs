use emeta::*;

fn main() -> Res<()> {
    env_logger::init();

    let mut config = App::new()?;
    config.from_file().unwrap_or_else(|_| {});
    config.main()?;

    Ok(())
}
