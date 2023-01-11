use std::cell::RefCell;

use geometry::{Point, TwoDimDirection, Vec2};
use tetra::graphics::mesh::{Mesh, ShapeStyle};
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::Context;

use crate::assets::{Assets, Tileset};
use crate::colors::Colors;
use crate::game::bodies::Freshness;
use crate::game::map::item::ItemView;
use crate::game::map::terrain::TerrainView;
use crate::game::races::Gender;
use crate::game::{Avatar, Soul, World};

// TODO: refactor this to small functions

pub fn draw(
    ctx: &mut Context,
    world: &RefCell<World>,
    assets: &Assets,
    window_size: (i32, i32),
    shift_of_view: Point,
    cursors: Vec<(Point, Color)>,
) {
    let world = world.borrow();
    tetra::graphics::clear(ctx, Colors::BLACK);
    let width = window_size.0 as f32;
    let height = window_size.1 as f32;
    let scale = world.game_view.zoom.as_scale();
    let zoom = world.game_view.zoom.as_view();
    let tile_size = assets.tileset.tile_size as f32 * zoom;
    let window_size_in_tiles = (
        (width / tile_size).ceil() as i32,
        (height / tile_size).ceil() as i32,
    );
    let center = Vec2::new(
        width / 2.0 - tile_size / 2.0,
        height / 2.0 - tile_size / 2.0,
    );
    let center_tile = world.player().pos + shift_of_view;
    let left_top = center_tile + (-window_size_in_tiles.0 / 2, -window_size_in_tiles.1 / 2);
    let right_bottom = center_tile + (window_size_in_tiles.0 / 2, window_size_in_tiles.1 / 2);
    world.map().load_tiles_between(left_top, right_bottom);
    for (pos, tile) in world.map().tiles_between(left_top, right_bottom) {
        if !world.is_visible(pos) {
            continue; // TODO: TileView struct for remembering unseen tiles
        }
        let dx = pos.x - center_tile.x;
        let dy = pos.y - center_tile.y;
        let this_tile_size = Tileset::get_size(tile.terrain.looks_like());
        let asset_tile_size = assets.tileset.tile_size as f32;
        let x_correction = -(this_tile_size.x - asset_tile_size) / 2.0 * zoom;
        let y_correction = -(this_tile_size.y - asset_tile_size) * zoom;
        let params = DrawParams::new()
            .position(Vec2::new(
                (center.x + dx as f32 * tile_size + x_correction).round(),
                (center.y + dy as f32 * tile_size + y_correction).round(),
            ))
            .scale(scale);
        assets
            .tileset
            .draw_region(ctx, tile.terrain.looks_like(), params.clone());
        if let Some(item) = tile.top_item() {
            assets
                .tileset
                .draw_region(ctx, item.looks_like(), params.clone());
            if tile.items.len() > 1 {
                assets.tileset.draw_region(ctx, "highlight", params);
            }
        }
        // TODO: multitile units
        let position = Vec2::new(
            center.x + dx as f32 * tile_size,
            center.y + dy as f32 * tile_size,
        );
        for i in tile.units.iter().copied() {
            draw_unit(
                ctx,
                &assets.tileset,
                position,
                zoom,
                true,
                world.get_unit(i),
            );
        }
    }
    // if world.player().action.is_some() {
    //     self.draw_action_loader(ctx, center);
    // } else {
    //     self.action_text = None;
    // }
    let cursor_mesh = Mesh::rectangle(
        ctx,
        ShapeStyle::Stroke(1.0),
        Rectangle::new(
            0.0,
            0.0,
            assets.tileset.tile_size as f32,
            assets.tileset.tile_size as f32,
        ),
    )
    .unwrap();
    for (delta, color) in cursors {
        let delta = delta * tile_size;
        cursor_mesh.draw(
            ctx,
            DrawParams::new()
                .position(center + delta)
                .scale(scale)
                .color(color.with_alpha(0.7)),
        );
    }
}

pub fn draw_unit(
    ctx: &mut Context,
    tileset: &Tileset,
    mut position: Vec2,
    zoom: f32,
    rotate: bool,
    avatar: &Avatar,
) {
    let scale = if !rotate || matches!(avatar.vision, TwoDimDirection::East) {
        Vec2::new(zoom, zoom)
    } else {
        position.x += 10.0 * zoom;
        Vec2::new(-zoom, zoom)
    };
    match &avatar.soul {
        Soul::Zombie(person, ..) => {
            let freshness = avatar
                .body
                .parts
                .get(&Point::new(0, 0))
                .map_or(Freshness::Rotten, |i| i.data.freshness);
            let (name, color) = match freshness {
                Freshness::Fresh => (
                    if person.appearance.age > 15 {
                        "raw_zombie"
                    } else {
                        "raw_zombie_child"
                    },
                    person.appearance.skin_tone.into(),
                ),
                Freshness::Rotten => (
                    if person.appearance.age > 15 {
                        "zombie"
                    } else {
                        "zombie_child"
                    },
                    Colors::WHITE,
                ),
                Freshness::Skeletal => (
                    if person.appearance.age > 15 {
                        "skeleton"
                    } else {
                        "skeleton_child"
                    },
                    Colors::WARM_IVORY,
                ),
            };
            tileset.draw_region(
                ctx,
                name,
                DrawParams::new()
                    .position(position)
                    .scale(scale)
                    .color(color),
            );
        }
        Soul::Player(person) => {
            // TODO: draw wear
            tileset.draw_region(
                ctx,
                match person.mind.gender {
                    Gender::Female => "female",
                    Gender::Male => "male",
                    Gender::Custom(_) => "queer",
                },
                DrawParams::new()
                    .position(position)
                    .scale(scale)
                    .color(person.appearance.skin_tone.into()),
            );
        }
    }
    if let Some(item) = avatar.wield.get(0) {
        let offset = if !rotate || matches!(avatar.vision, TwoDimDirection::East) {
            Vec2::new(15.0 * zoom, 10.0 * zoom)
        } else {
            Vec2::new(-15.0 * zoom, 10.0 * zoom)
        };
        tileset.draw_region(
            ctx,
            item.looks_like(),
            DrawParams::new()
                .position(position + offset)
                .scale(scale * -1.0),
        );
    }
}
