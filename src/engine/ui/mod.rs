mod fps_text;

use bevy::prelude::*;

use self::fps_text::FpsTextPlugin;

#[derive(Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FpsTextPlugin);
    }
}
