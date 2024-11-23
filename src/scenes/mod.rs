mod game;
mod menu;

pub use game::create_new_game;
pub use menu::{MainMenu, SettingsMenu};

pub enum Scene {
    None,
    MainMenu(MainMenu),
    SettingsMenu(SettingsMenu),
}
