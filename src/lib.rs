use crate::app::App;

pub mod app;
pub mod config;
pub mod cli;
pub mod logging;

pub fn run() -> anyhow::Result<()>{
    let cli = cli::parse();
    config::initialize_config(&cli)?;
    logging::initialize_logging(cli.log_to_stdio)?;

    if cli.print_config {
        println!("{}", toml::to_string_pretty(config::get())?);
    }

    println!("hi there this is a testyh thingiweoruiwo34ioiw3u4owi3u4o");
    println!("config: {:?}", config::get());
    let mut app = App::new()?;
    app.run()
}

