mod app;
mod dmenu;
mod models;

use app::App;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, enable_raw_mode},
};
use models::MenuConfig;

use anyhow::{Result, bail};
use clap::{Args, Parser};
use ratatui::{Terminal, prelude::CrosstermBackend};

#[derive(Args, Debug)]
#[group(required = false, multiple = true)]
struct DmenuArgs {
    #[arg(long, action, requires = "title")]
    dmenu: bool,
    #[arg(short, long)]
    title: Option<String>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct ConfigMode {
    #[arg(short, long)]
    config_file: Option<String>,
}

#[derive(Parser, Debug)]
struct ProgramArgs {
    #[command(flatten)]
    dmenu_mode: DmenuArgs,

    #[command(flatten)]
    config_mode: ConfigMode,
}

fn try_main(args: ProgramArgs) -> Result<()> {
    let mut config: MenuConfig = Default::default();

    if args.dmenu_mode.dmenu {
        // Want to read from stdin
        let lines = dmenu::read_input_lines()?;

        config.items = dmenu::items_from_str(&lines);
        config.title = models::Title {
            name: args.dmenu_mode.title.unwrap_or("dmenu".to_string()),
        };
    } else if let Some(config_path) = args.config_mode.config_file {
        let contents = std::fs::read_to_string(config_path)?;
        config = toml::from_str(&contents)?;
    } else {
        bail!("need --demnu mode or --config-file mode")
    }

    let mut app = App {
        title: config.title.name,
        items: config.items,
        dmenu: args.dmenu_mode.dmenu,
        ..Default::default()
    };

    // Rendering to stderr for dmenu!
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;
    app.run(&mut terminal)?;
    Ok(())
}

fn main() {
    let args = ProgramArgs::parse();
    if let Err(err) = try_main(args) {
        eprintln!("runtime error: {:#?}", err);
    }
}
