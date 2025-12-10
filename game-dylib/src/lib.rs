//! Wrapper for hot-reloadable plugin.
use prisoner_dilemma_simulator::{fyrox::plugin::Plugin, Game};

#[no_mangle]
pub fn fyrox_plugin() -> Box<dyn Plugin> {
    Box::new(Game::default())
}
