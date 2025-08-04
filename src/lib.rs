pub mod config;
pub mod cli;

pub fn run() -> anyhow::Result<()>{
    let cli = cli::parse();
    config::initialize_config(&cli)?;
    if cli.print_config {
        println!("{}", toml::to_string_pretty(config::get())?);
    }
    println!("hi there this is a testyh thingiweoruiwo34ioiw3u4owi3u4o");
    println!("config: {:?}", config::get());
    Ok(())
}