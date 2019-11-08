mod config;
mod entity;
mod map;
mod map_data;
mod position;
mod world;
mod world_renderer;

use tetra::{Context, ContextBuilder, State};
use tetra::input::{self, Key, MouseButton};
use tetra::graphics::{self, Color, Font, ScreenScaling, Text, Texture, Vec2};
use tetra::window::{self};

use config::*;
use entity::*;
use map::*;
use position::*;
use world::*;
use world_renderer::*;

struct GameState {
    world: World,
    world_renderer: WorldRenderer,
    info_label: Text,
    cursor_position: Option<Vec2>,
    cursor_label: Text,
    cursor: Texture,
    show_cursor: bool,
    running: bool,
    tick_counter: i32,
    tick_interval: i32,
    turn: u32,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        graphics::set_scaling(ctx, ScreenScaling::ShowAll);

        let font = Font::new(ctx, "./assets/font/04b03.ttf")?;

        let map_data = load_tiled_map("./assets/map/test.json").unwrap();
        let mut world = World::new(map_data.get_width(), map_data.get_height());

        for iy in 0..map_data.get_height() {
            for ix in 0..map_data.get_width() {
                let position = Position::new(ix as i16, iy as i16);
                world.map.get_mut(position).unwrap().height = map_data.get_tile(ix, iy);
            }
        }

        world.create_data("tree")
            .set_lifetime(80, 100)
            .set_height(4, 6)
            .set_spawn(32, 30, 70)
            .set_sprite(20)
        ;

        world.create_data("scorpid")
            .set_lifetime(10, 14)
            .set_height(4, 7)
            .set_spawn(4, 3, 6)
            .set_sprite(44)
        ;

        Ok(GameState {
            world,
            world_renderer: WorldRenderer::new(ctx)?,
            info_label: Text::new("", font, 8.0),
            cursor_position : None,
            cursor_label: Text::new("", font, 8.0),
            cursor: Texture::new(ctx, "./assets/gfx/cursor.png")?,
            show_cursor: true,
            tick_counter: 1,
            tick_interval: 1,
            running: false,
            turn: 0,
        })
    }
}

fn spawn_entity(world: &mut World, data_id: EntityDataId, pos: Position) {
    if let Some(tile) = world.map.get_mut(pos) {
        if tile.entity == None {
            let data = &world.data_store[data_id];
            let entity = Entity::new(data, pos);
            let entity_id = world.entities.insert(entity);
            tile.entity = Some(entity_id);
        }
    }
}

fn has_neighbour(map: &Map, entity: &Entity, delta: Position) -> bool {
    if let Some(tile) = map.get(entity.pos + delta) {
        return tile.entity.is_some();
    }

    true
}

fn tick(world: &mut World) {
    let mut kill_list: Vec<EntityId> = Vec::new();
    let mut add_list: Vec<Entity> = Vec::new();

    for (entity_id, entity) in &mut world.entities {
        //println!("tick entity {:?}", entity);

        let data = &world.data_store[entity.data_id];

        // age tick
        entity.age += 1;
        if entity.age >= entity.lifetime {
            kill_list.push(entity_id);
            world.map.get_mut(entity.pos).unwrap().entity = None;
            continue;
        }

        // energy tick
        let mut neighbour_count = 0;
        if has_neighbour(&world.map, entity, position::LEFT) { neighbour_count += 1; }
        if has_neighbour(&world.map, entity, position::RIGHT) { neighbour_count += 1; }
        if has_neighbour(&world.map, entity, position::UP) { neighbour_count += 1; }
        if has_neighbour(&world.map, entity, position::DOWN) { neighbour_count += 1; }
        if has_neighbour(&world.map, entity, position::LEFT_UP) { neighbour_count += 1; }
        if has_neighbour(&world.map, entity, position::RIGHT_UP) { neighbour_count += 1; }
        if has_neighbour(&world.map, entity, position::LEFT_DOWN) { neighbour_count += 1; }
        if has_neighbour(&world.map, entity, position::RIGHT_DOWN) { neighbour_count += 1; }

        if neighbour_count <= 5 {
            entity.energy += 6 - neighbour_count;
        }

        // spawn tick
        if entity.age >= data.spawn_age_min && entity.age <= data.spawn_age_max && entity.energy >= data.spawn_cost {
            let dir = match rand::random::<u32>() % 12 {
                0 => position::LEFT,
                1 => position::RIGHT,
                2 => position::UP,
                3 => position::DOWN,
                4 => position::LEFT_UP,
                5 => position::RIGHT_UP,
                6 => position::LEFT_DOWN,
                7 => position::RIGHT_DOWN,
                8 => position::LEFT + position::LEFT,
                9 => position::RIGHT + position::RIGHT,
                10 => position::UP + position::UP,
                _ => position::DOWN + position::DOWN,
            };

            if let Some(tile) = world.map.get_mut(entity.pos + dir) {
                if tile.is_free() && tile.height >= data.height_min && tile.height <= data.height_max {
                    let spawned_entity = Entity::new(data, tile.pos);
                    tile.booked = true;
                    add_list.push(spawned_entity);

                    entity.energy -= data.spawn_cost;
                }
            }
        }
    };

    for entity_id in kill_list {
        if world.entities.remove(entity_id).is_none() {
            panic!("failed to remove entity {:?}", entity_id);
        }
    }

    for entity in add_list {
        let tile = world.map.get_mut(entity.pos).unwrap();
        let entity_id = world.entities.insert(entity);
        tile.entity = Some(entity_id);
        tile.booked = false;
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let tile_x = input::get_mouse_x(ctx) as i16 / TILE_SIZE;
        let tile_y = std::cmp::max(0, (input::get_mouse_y(ctx) as i16 - UI_HEIGHT) / TILE_SIZE);
        let mouse_pos = Position::new(tile_x, tile_y);

        if let Some(tile) = self.world.map.get(mouse_pos)  {
            if let Some(entity_id) = tile.entity {
                if let Some(entity) = self.world.entities.get(entity_id) {
                    let entity_data = self.world.data_store.get(entity.data_id).unwrap();
                    self.cursor_label.set_content(format!("{} - height {} - {} (age: {} energy: {})",
                        mouse_pos,
                        tile.height,
                        entity_data.name,
                        entity.age,
                        entity.energy,
                    ));
                }
            }
            else {
                self.cursor_label.set_content(format!("{} - height {} - empty",
                    mouse_pos,
                    tile.height
                ));
            }
            self.cursor_position = Some(
                WorldRenderer::get_screen_position(mouse_pos) + Vec2::new(-1.0, -1.0));
        }
        else {
            self.cursor_label.set_content("");
            self.cursor_position = None;
        }

        if input::is_mouse_button_pressed(ctx, MouseButton::Left) {
            spawn_entity(&mut self.world, 0, mouse_pos);
        }

        if input::is_key_pressed(ctx, Key::Num1) {
            spawn_entity(&mut self.world, 1, mouse_pos);
        }

        if input::is_key_pressed(ctx, Key::Tab) {
            self.show_cursor = !self.show_cursor;
        }

        if input::is_key_pressed(ctx, Key::Right) {
            tick(&mut self.world);
            self.turn += 1;
        }

        if input::is_key_pressed(ctx, Key::Space) {
            self.running = !self.running;
        }

        if self.running {
            self.tick_counter -= 1;
            if  self.tick_counter == 0 {
                tick(&mut self.world);
                self.turn += 1;
                self.tick_counter = self.tick_interval;
            }
        }

        if input::is_key_down(ctx, Key::LAlt) && input::is_key_pressed(ctx, Key::Return) {
            window::toggle_fullscreen(ctx)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
        self.world_renderer.draw(ctx, &self.world);

        if self.show_cursor {
            if let Some(cursor_position) = self.cursor_position {
                graphics::draw(ctx, &self.cursor, cursor_position);
            }

            graphics::draw(ctx, &self.cursor_label, Vec2::new(2.0, 1.0));
        }

        self.info_label.set_content(format!("entities {} year {}", self.world.entities.len(), self.turn));
        if let Some(bounds) = self.info_label.get_bounds(ctx) {
            let position = Vec2::new(
                SCREEN_WIDTH as f32 - bounds.width - 2.0,
                1.0
            );
            graphics::draw(ctx, &self.info_label, position);
        }
        
        Ok(())
    }
}

pub fn load_tiled_map<P: AsRef<std::path::Path>>(map_path: P) -> tetra::Result<map_data::MapData> {
    let content = std::fs::read_to_string(map_path)?;
    let parsed = json::parse(content.as_str()).unwrap();

    let width = parsed["width"].as_usize().unwrap();
    let height = parsed["height"].as_usize().unwrap();
    let raw_data = &parsed["layers"][0]["data"];

    let mut data = map_data::MapData::new(width, height);

    for ix in 0..width {
        for iy in 0..height {
            let tid = raw_data[iy * width + ix].as_u8().unwrap() - 1;
            data.set_tile(ix, iy, tid);
        }
    }

    Ok(data)
}

fn main() -> tetra::Result {
    ContextBuilder::new("evo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .tick_rate(30.0)
        .resizable(true)
        .fullscreen(false)
        .quit_on_escape(true)
        .build()?
        .run_with(GameState::new)
}
