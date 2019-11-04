mod config;
mod map;

use std::cmp;

use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key, MouseButton};
use tetra::graphics::{self, Color, DrawParams, Font, Rectangle, ScreenScaling, Text, Texture, Vec2};
use tetra::window::{self};

use rand::random;

use config::*;
use map::{Map};

struct Creature {
    x: i32,
    y: i32,
    sprite: i32,
    sheet: Texture,
    flip: bool,
}

impl Creature {
    fn new(x: i32, y: i32, sprite: i32, sheet: Texture) -> tetra::Result<Creature> {
        Ok(Creature {
            x,
            y,
            sprite,
            sheet,
            flip: random::<bool>(),
        })
    }

    fn tick(&mut self, map: &Map) {
        let mut candidates: Vec<(i32,i32)> = Vec::new();
        if self.can_move_by(map, 1, 0) { candidates.push((1, 0)); }
        if self.can_move_by(map, -1, 0) { candidates.push((-1, 0)); }
        if self.can_move_by(map, 0, 1) { candidates.push((0, 1)); }
        if self.can_move_by(map, 0, -1) { candidates.push((0, -1)); }

        if candidates.len() > 0 {
            let index = random::<usize>() % candidates.len();
            let delta = candidates[index];
            self.x += delta.0;
            self.y += delta.1;

            if delta.0 == -1 { self.flip = true; }
            else if delta.0 == 1 { self.flip = false; }
        }
    }

    fn can_move_by(&self, map: &Map, x: i32, y: i32) -> bool {
        if let Some(tile) = map.get_tile(self.x + x, self.y + y) {
            return !tile.solid();
        }

        false
    }

    fn draw(&self, ctx: &mut Context) {
        let texture_width = self.sheet.width() / TILE_SIZE;

        let sheet_x = (self.sprite % texture_width) * TILE_SIZE;
        let sheet_y = (self.sprite / texture_width) * TILE_SIZE;

        let position = Vec2::new(
            (self.x * TILE_SIZE) as f32,
            (self.y * TILE_SIZE + UI_HEIGHT) as f32);
    
        let scale =
            if self.flip { Vec2::new(-1.0, 1.0) }
            else { Vec2::new(1.0, 1.0) };

        graphics::draw(
            ctx,
            &self.sheet,
            DrawParams::new()
                .position(position)
                .scale(scale)
                .clip(Rectangle::new(sheet_x as f32, sheet_y as f32, TILE_SIZE as f32, TILE_SIZE as f32))
        );
    }
}

struct GameState {
    map: Map,
    creatures_sheet: Texture,
    creatures: Vec<Creature>,
    info_label: Text,
    cursor_position: Option<Vec2>,
    cursor_label: Text,
    cursor: Texture,
    running: bool,
    tick_counter: i32,
    turn: u32,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        graphics::set_scaling(ctx, ScreenScaling::ShowAll);

        let font = Font::new(ctx, "./assets/font/04b03.ttf")?;

        Ok(GameState {
            map: Map::new(ctx, "./assets/map/test.json")?,
            creatures_sheet: Texture::new(ctx, "./assets/gfx/entities.png")?,
            creatures: Vec::new(),
            info_label: Text::new("", font, 8.0),
            cursor_position : None,
            cursor_label: Text::new("", font, 8.0),
            cursor: Texture::new(ctx, "./assets/gfx/cursor.png")?,
            tick_counter: 1,
            running: false,
            turn: 0,
        })
    }

    fn spawn_creature(&mut self, x: i32, y: i32, id: i32) -> tetra::Result {
        self.creatures.push(Creature::new(x, y, id, self.creatures_sheet.clone())?);
        Ok(())
    }

    fn tick(&mut self) {
        for creature in &mut self.creatures {
            creature.tick(&self.map);
        }
        self.turn += 1;
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let tile_x = input::get_mouse_x(ctx) as i32 / TILE_SIZE;
        // let tile_y = cmp::max(0, (input::get_mouse_y(ctx) as i32 - UI_HEIGHT) / TILE_SIZE);
        let tile_y = (input::get_mouse_y(ctx) as i32 - UI_HEIGHT) / TILE_SIZE; // TEMP: allow to hide cursor

        if let Some(tile) = self.map.get_tile(tile_x, tile_y)  {
            self.cursor_label.set_content(format!("{} ({},{})", tile.get_name(), tile.x, tile.y));
            self.cursor_position = Some(tile.screen_position + Vec2::new(-1.0, -1.0));
        }
        else {
            self.cursor_label.set_content("");
            self.cursor_position = None;
        }

        if input::is_key_pressed(ctx, Key::Num1) { self.spawn_creature(tile_x, tile_y, 4)?; }
        if input::is_key_pressed(ctx, Key::Num2) { self.spawn_creature(tile_x, tile_y, 18)?; }
        if input::is_key_pressed(ctx, Key::Num3) { self.spawn_creature(tile_x, tile_y, 17)?; }
        if input::is_key_pressed(ctx, Key::Num4) { self.spawn_creature(tile_x, tile_y, 21)?; }

        if input::is_key_pressed(ctx, Key::Right) {
            self.tick();    
        }

        if input::is_key_pressed(ctx, Key::Space) {
            self.running = !self.running;
        }

        if self.running {
            self.tick_counter -= 1;
            if  self.tick_counter == 0 {
                self.tick();
                self.tick_counter = TICK_INTERVAL;
            }
        }

        if input::is_key_down(ctx, Key::LAlt) && input::is_key_pressed(ctx, Key::Return) {
            window::toggle_fullscreen(ctx)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        self.map.draw(ctx);

        for creature in &self.creatures {
            creature.draw(ctx);
        }

        if let Some(cursor_position) = self.cursor_position {
            graphics::draw(ctx, &self.cursor, cursor_position);
        }

        if let Some(bounds) = self.cursor_label.get_bounds(ctx) {
            let position = Vec2::new(
                graphics::get_internal_width(ctx) as f32 - bounds.width - 2.0,
                1.0
            );
            graphics::draw(ctx, &self.cursor_label, position);
        }

        self.info_label.set_content(format!("turn: {} creatures: {}", self.turn, self.creatures.len()));
        graphics::draw(ctx, &self.info_label, Vec2::new(2.0, 1.0));
        
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("EVO", SCREEN_WIDTH, SCREEN_HEIGHT)
        .resizable(true)
        .fullscreen(false)
        .quit_on_escape(true)
        .build()?
        .run_with(GameState::new)
}