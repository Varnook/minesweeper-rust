use fltk::{app, prelude::*, window::Window};
mod errors;
mod game;
use crate::errors::{MinesweeperError, Result};

fn main() -> Result<()> {
    let application = app::App::default();
    let mut w = Window::new(100, 100, 400, 300, "Minesweeper");
    w.end();
    w.show();
    match application.run() {
        Ok(_) => Ok(()),
        Err(e) => Err(MinesweeperError::FltkErr(e)),
    }
}
