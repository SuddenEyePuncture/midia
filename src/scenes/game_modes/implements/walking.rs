use std::time::Instant;

use geometry::Direction;
use tetra::{
    input::{Key, KeyModifier},
    Context,
};

use crate::{
    colors::Colors,
    game::actions::implements::{Drop, Skip, Walk},
    input,
    settings::Settings,
};

use super::super::{
    super::{implements::GameScene, Scene, SomeTransitions, Transition},
    implements::{Digging, Dropping, Examining, Observing, Reading, Wielding},
    GameModeImpl,
};

pub struct Walking {
    last_walk: Instant,
}

impl Walking {
    pub fn new() -> Self {
        Self {
            last_walk: Instant::now(),
        }
    }
}

impl Default for Walking {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Walking {
    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        if input::is_mouse_scrolled_down(ctx)
            || input::is_key_with_mod_pressed(ctx, (Key::Z, KeyModifier::Shift))
        {
            game.world.borrow_mut().game_view.zoom.dec();
            None
        } else if input::is_mouse_scrolled_up(ctx) || input::is_key_with_mod_pressed(ctx, Key::Z) {
            game.world.borrow_mut().game_view.zoom.inc();
            None
        } else if input::is_key_pressed(ctx, Key::Escape) {
            Some(vec![Transition::Push(Scene::GameMenu)])
        } else if input::is_key_with_mod_pressed(ctx, Key::E) {
            game.push_mode(Examining::new().into());
            None
        } else if input::is_key_with_mod_pressed(ctx, Key::D) {
            game.try_start_action(
                Drop {
                    item_id: 0,
                    dir: Direction::Here,
                }
                .into(),
            );
            None
        } else if input::is_key_with_mod_pressed(ctx, (Key::D, KeyModifier::Shift)) {
            game.push_mode(Dropping::new().into());
            None
        } else if input::is_key_with_mod_pressed(ctx, Key::W) {
            game.push_mode(Wielding::new().into());
            None
        } else if input::is_key_with_mod_pressed(ctx, (Key::C, KeyModifier::Shift)) {
            game.log.clear();
            None
        } else if input::is_key_with_mod_pressed(ctx, Key::G) {
            game.push_mode(Digging::new().into());
            None
        } else if input::is_key_with_mod_pressed(ctx, Key::X) {
            game.push_mode(Observing::new().into());
            None
        } else if input::is_key_with_mod_pressed(ctx, (Key::R, KeyModifier::Shift)) {
            game.push_mode(Reading::new().into());
            None
            // } else if input::is_key_with_mod_pressed(ctx, (Key::Num2, KeyModifier::Shift)) {
            //     Some(vec![Transition::Push(Scene::BodyView(0))])
        } else if input::is_key_with_mod_pressed(ctx, Key::I) {
            // TODO: inventory game scene
            let items: Vec<String> = game
                .world
                .borrow()
                .player()
                .wear
                .iter()
                .map(|i| i.name().to_string())
                .collect();
            game.log.log(
                format!("You wear: {}", items.join(", ")),
                Colors::WHITE_SMOKE,
            );
            None
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let now = Instant::now();
            if now.duration_since(self.last_walk).subsec_millis()
                > Settings::instance().input.repeat_interval
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_walk = now;
                if dir.is_here() {
                    game.try_start_action(Skip {}.into());
                } else {
                    game.try_rotate_player(dir);
                    game.try_start_action(Walk { dir }.into());
                }
            }
            None
        } else {
            None
        }
    }
}
