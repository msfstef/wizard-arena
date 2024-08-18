pub mod fighting;
pub mod main_menu;
use crate::models::State;

pub trait GameScene {
    fn handle_input(&self, state: &mut State) -> ();
    fn render(&self, state: &State) -> ();
    fn render_ui(&self, state: &mut State) -> ();
}
