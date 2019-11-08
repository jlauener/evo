use tetra::{Context};
use tetra::graphics::{self, DrawParams, Rectangle, Texture, Vec2};

use crate::config::*;
use crate::position::Position;
use crate::world::World;

pub struct WorldRenderer {
    spritesheet: Texture,
}

impl WorldRenderer {
    pub fn new(ctx: &mut Context) -> tetra::Result<WorldRenderer> {
        Ok(WorldRenderer {
            spritesheet: Texture::new(ctx, "./assets/gfx/spritesheet.png")?,
        })
    }

    pub fn draw(&self, ctx: &mut Context, world: &World) {
        world.map.for_each(|tile| {
            if let Some(entity_id) = tile.entity {
                let entity = &world.entities[entity_id];
               
                let entity_data = world.data_store.get(entity.data_id).unwrap();
                // FIXME
                let mut sprite = entity_data.sprite;
                if entity.age > 12 {
                    sprite += 3;
                }
                else if entity.age > 8 {
                    sprite += 2;
                }
                else if entity.age > 4 {
                    sprite += 1;
                }
                self.draw_sprite(ctx, sprite, entity.pos, false);
            }
            else {
                self.draw_sprite(ctx, tile.height as u16, tile.pos, false);
            }
        });
    }

    pub fn get_screen_position(pos: Position) -> Vec2 {
        Vec2::new((pos.x * TILE_SIZE) as f32, (pos.y * TILE_SIZE + UI_HEIGHT) as f32)
    }

    fn draw_sprite(&self, ctx: &mut Context, sprite: u16, pos: Position, flip: bool) {
        let sheet_x = ((sprite as i16 % 8) * TILE_SIZE) as f32;
        let sheet_y = ((sprite as i16 / 8) * TILE_SIZE) as f32;

        graphics::draw(
            ctx,
            &self.spritesheet,
            DrawParams::new()
                .position(WorldRenderer::get_screen_position(pos))
                .scale(if flip { Vec2::new(-1.0, 1.0) } else { Vec2::new(1.0, 1.0) })
                .clip(Rectangle::new(sheet_x, sheet_y, TILE_SIZE as f32, TILE_SIZE as f32))
        );
    }
}