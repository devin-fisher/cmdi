use crate::app::App;
use crate::config::{Cli, load};
use crate::opencli::v0_1::V0_1;
use crate::screens::builder_screen::model::BuilderScreen;
use clap::Parser;
use directories::ProjectDirs;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::LeaveAlternateScreen;
use std::io::stdout;
use std::panic::{set_hook, take_hook};

#[macro_use]
extern crate rust_i18n;

i18n!(
    "locales",
    fallback = "en"
);

mod app;
mod builder;
mod config;
pub mod event;
mod opencli;
mod screens;
mod theme;
mod util;

use log::info;
use log4rs;
use log4rs::Config;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;

const DEMO_YAML: &str = include_str!("../src/opencli/demo-kubectl.yaml");

fn main() -> color_eyre::Result<()> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
        .build("/tmp/cmdi.log")
        .unwrap();

    let config = Config::builder()
        .appender(
            Appender::builder().build(
                "logfile",
                Box::new(logfile),
            ),
        )
        .build(
            Root::builder()
                .appenders(["logfile"])
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();

    info!("Starting UP");

    color_eyre::install()?;

    let directories = ProjectDirs::from(
        "", "", "cmdi",
    )
    .expect("Failed to get project directories");
    let cli = Cli::parse();
    let _settings = load(
        &cli,
        directories,
    )?;

    info!(
        "Build for {}",
        cli.cmd
    );

    let parsed: V0_1 = serde_yml::from_str(DEMO_YAML)?;

    // TODO trap SIGTERM see signal-hook
    // Add panic hook
    let original_hook = take_hook();
    set_hook(
        Box::new(
            move |panic_info| {
                ratatui::restore();
                original_hook(panic_info);
            },
        ),
    );

    let terminal = ratatui::init();
    let result = App::new(BuilderScreen::demo(parsed))
        .run(
        terminal,
    );

    execute!(
        stdout(),
        LeaveAlternateScreen
    )?;
    ratatui::restore();

    match result {
        Ok(result_str) => println!(
            "{}",
            result_str
        ),
        Err(msg) => println!(
            "Exiting - {}",
            msg
        ),
    }

    Ok(())
}
