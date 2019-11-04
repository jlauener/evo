use std::fmt;
use std::fs;
use std::path::Path;

use tetra::{Context};
use tetra::graphics::{self, DrawParams, Rectangle, Texture, Vec2};

use crate::config::*;

#[derive(Copy, Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    tid: i32,
    pub screen_position: Vec2,
}

impl Tile {
    pub fn solid(&self) -> bool {
        // FIXME
        self.tid == 3 || self.tid == 10 || self.tid == 11 || (self.tid >= 12 && self.tid <= 15) || self.tid >= 20
    }

    pub fn get_name(&self) -> String {
        // FIXME
        if self.tid < 4 {
            return String::from("mud");
        } else if self.tid < 8 {
            return String::from("grass");
        } else if self.tid < 12 {
            return String::from("rock");
        } else if self.tid < 16  {
            return String::from("water");
        } else if self.tid < 20 {
            return String::from("plant");
        } else if self.tid < 24 {
            return String::from("tree");
        } else if self.tid < 28 {
            return String::from("bush");
        }

        String::from("unknown")
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Tile x={} y={} tid={}", self.x, self.y, self.tid)
    }
}

pub struct Map {
    tileset: Texture,
    data: [Tile; MAP_WIDTH * MAP_HEIGHT],
}

impl Map {
    pub fn new<P: AsRef<Path>>(ctx: &mut Context, map_path: P) -> tetra::Result<Map> {
        println!("loading map...");

        let content = fs::read_to_string(map_path)?;
        let parsed = json::parse(content.as_str()).unwrap();
        let raw_data = &parsed["layers"][0]["data"];

        let tile = Tile {
            x: 0,
            y: 0,
            tid: 0,
            screen_position: Vec2::new(0.0, 0.0),
        };
        let mut data: [Tile; MAP_WIDTH * MAP_HEIGHT] = [tile; MAP_WIDTH * MAP_HEIGHT];

        for iy in 0..MAP_HEIGHT {
            for ix in 0..MAP_WIDTH {
                let index = iy * MAP_WIDTH + ix;
                let tile = &mut data[index];
                tile.x = ix as i32;
                tile.y = iy as i32;
                tile.tid = raw_data[index].as_i32().unwrap() - 1;
                tile.screen_position.x = (ix as i32 * TILE_SIZE) as f32;
                tile.screen_position.y = (iy as i32 * TILE_SIZE + UI_HEIGHT) as f32;
            }
        }

        let tileset = Texture::new(ctx, "./assets/gfx/tiles.png")?;

        Ok(Map {
            tileset,
            data,
        })
    }

    pub fn draw(&self, ctx: &mut Context) {
        let texture_width = self.tileset.width() / TILE_SIZE;

        for iy in 0..MAP_HEIGHT {
            for ix in 0..MAP_WIDTH {
                let tile = &self.data[iy * MAP_WIDTH + ix];
                let tileset_x = (tile.tid % texture_width) * TILE_SIZE;
                let tileset_y = (tile.tid / texture_width) * TILE_SIZE;

                graphics::draw(
                    ctx,
                    &self.tileset,
                    DrawParams::new()
                        .position(tile.screen_position)
                        .clip(Rectangle::new(tileset_x as f32, tileset_y as f32, TILE_SIZE as f32, TILE_SIZE as f32))
                );
            }
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&Tile>
    {
        if x < 0 || x >= MAP_WIDTH as i32 || y < 0 || y >= MAP_HEIGHT as i32 {
            return None;
        }

        Some(&self.data[y as usize * MAP_WIDTH + x as usize])
    }
}