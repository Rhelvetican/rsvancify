use anyhow::Result;

use crate::{app::App, tui::init};

mod app;
mod error;
mod tui;

fn main() -> Result<()> {
    let mut tui = init()?;
    let app = App::default().run(&mut tui);
    tui::restore()?;
    app
}
