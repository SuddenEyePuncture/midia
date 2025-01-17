use std::time::Instant;

use geometry::{Direction, Point, Rect, Vec2};
use tetra::{
    graphics::{
        mesh::{Mesh, ShapeStyle},
        Color, Rectangle,
    },
    input::{Key, KeyModifier},
    Context,
};

use crate::{
    colors::Colors,
    game::World,
    input,
    settings::Settings,
    ui::{Draw, JustMesh, Label, Position, Positionate, Stringify},
};

use super::super::{
    super::{implements::GameScene, SomeTransitions},
    GameModeImpl,
};

struct ObservingSprite {
    pub label: Label,
    pub mesh: JustMesh,
}

fn create_mesh(ctx: &mut Context, rect: Rect, position: Position) -> JustMesh {
    JustMesh::new(
        Mesh::rectangle(
            ctx,
            ShapeStyle::Fill,
            Rectangle::new(0.0, 0.0, rect.w, rect.h),
        )
        .unwrap(),
        Some(Colors::BLACK.with_alpha(0.7)),
        Vec2::new(rect.w, rect.h),
        position,
    )
}

pub struct Observing {
    last_shift: Instant,
    last_mouse_position: Vec2,
    mouse_moved: bool,
    last_zoom: u8,
    mouse_moved_pos: Point,
    sprite: Option<Box<ObservingSprite>>,
}

impl Observing {
    pub fn new() -> Self {
        Self {
            last_shift: Instant::now(),
            last_mouse_position: Vec2::zero(),
            mouse_moved: false,
            last_zoom: 0,
            mouse_moved_pos: Point::default(),
            sprite: None,
        }
    }

    fn update_mouse(&mut self, ctx: &mut Context, game: &mut GameScene) {
        let mouse = input::get_mouse_position(ctx);
        let zoom_view = game.world.borrow().game_view.zoom.as_view();
        let zoom = game.world.borrow().game_view.zoom.0;
        if mouse != self.last_mouse_position || zoom != self.last_zoom {
            self.last_mouse_position = mouse;
            self.last_zoom = zoom;
            if self.mouse_moved {
                let (w, h) = game.window_size;
                self.mouse_moved_pos = ((mouse - Vec2::new((w / 2) as f32, (h / 2) as f32))
                    / (game.assets.tileset.tile_size as f32 * zoom_view))
                    .into();
            }
            self.mouse_moved = true;
        }
    }

    fn update_sprite(&mut self, ctx: &mut Context, game: &mut GameScene) {
        let pos = game.world.borrow().player().pos + game.shift_of_view + self.mouse_moved_pos;
        let msg = game.world.borrow().this_is(pos, true);
        let tile_size = game.tile_size();
        let position = Vec2::from(self.mouse_moved_pos * tile_size);
        let position_shift = tile_size / 2.0 + 5.0;
        let position = match Direction::from_delta(self.mouse_moved_pos.x, self.mouse_moved_pos.y) {
            Direction::NorthWest | Direction::North | Direction::West | Direction::Here => {
                Position::at_center_by_left_top(
                    position.x + position_shift,
                    position.y + position_shift,
                )
            }
            Direction::East | Direction::NorthEast => Position::at_center_by_right_top(
                position.x - position_shift,
                position.y + position_shift,
            ),
            Direction::South | Direction::SouthWest => Position::at_center_by_left_bottom(
                position.x + position_shift,
                position.y - position_shift,
            ),
            Direction::SouthEast => Position::at_center_by_right_bottom(
                position.x - position_shift,
                position.y - position_shift,
            ),
        };
        let window_size = game.window_size;
        if let Some(sprite) = &mut self.sprite {
            sprite.label.set_value(msg);
            sprite.label.set_position(position);
            sprite.label.positionate(ctx, window_size);
            let rect = sprite.label.rect();
            sprite.mesh = create_mesh(ctx, rect, position);
            sprite.mesh.positionate(ctx, window_size);
        } else {
            let mut label = Label::new(
                msg,
                game.assets.fonts.default.clone(),
                Colors::WHITE_SMOKE,
                position,
            );
            label.positionate(ctx, window_size);
            let rect = label.rect();
            let mut mesh = create_mesh(ctx, rect, position);
            mesh.positionate(ctx, window_size);
            self.sprite = Some(Box::new(ObservingSprite { label, mesh }));
        }
    }
}

impl Default for Observing {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModeImpl for Observing {
    fn cursors(&self, _world: &World) -> Vec<(Point, Color)> {
        vec![(self.mouse_moved_pos, Colors::LIME)]
    }

    fn update(&mut self, ctx: &mut Context, game: &mut GameScene) -> SomeTransitions {
        self.update_mouse(ctx, game);
        let mut shifted = false;
        if input::is_key_pressed(ctx, Key::Escape) {
            game.shift_of_view = Point::default();
            game.modes.pop();
            return None;
        } else if input::is_mouse_scrolled_down(ctx) {
            game.world.borrow_mut().game_view.zoom.dec();
            shifted = true;
        } else if input::is_mouse_scrolled_up(ctx) {
            game.world.borrow_mut().game_view.zoom.inc();
            shifted = true;
        } else if let Some(dir) = input::get_direction_keys_down(ctx) {
            let now = Instant::now();
            if now.duration_since(self.last_shift).subsec_millis()
                > Settings::instance().input.repeat_interval
                || input::is_key_modifier_down(ctx, KeyModifier::Shift)
            {
                self.last_shift = now;
                game.shift_of_view += dir;
                shifted = true;
            }
        }
        if self.mouse_moved || shifted {
            self.update_sprite(ctx, game);
        }

        None
    }

    fn draw(&mut self, ctx: &mut Context, _game: &mut GameScene) {
        if let Some(sprite) = &mut self.sprite {
            sprite.mesh.draw(ctx);
            sprite.label.draw(ctx);
        }
    }
}
