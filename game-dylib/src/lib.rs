//! Wrapper for hot-reloadable plugin.
use ChoRPS_7DRL26::{fyrox::plugin::Plugin, Game};

#[no_mangle]
pub fn fyrox_plugin() -> Box<dyn Plugin> {
    Box::new(Game::default())
}
