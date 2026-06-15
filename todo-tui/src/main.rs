// Todo Ratatui application
//

use color_eyre::eyre::Result;

use crate::app::App;

mod app;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}
