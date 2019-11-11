mod config;
mod entity;
mod map;
mod map_data;
mod position;
mod world;
mod world_renderer;

use std::time::Instant;

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

fn create_world() -> World {
    let map_data = map_data::load_tiled_map("./assets/map/test.json").unwrap();
    let mut world = World::new(map_data);

    world.create_data("grass", CLASS_PLANT)
        .set_lifetime(40, 600)
        .set_altitude(4, 6)
        .set_height(1)
        .set_spawn(4, 10, 600, &PATTERN_CLOSE)
        .set_sprite(28)
    ;
    
    world.create_data("bush", CLASS_PLANT)
        .set_lifetime(60, 70)
        .set_altitude(4, 6)
        .set_height(2)
        .set_spawn(14, 20, 30, &PATTERN_CIRCLE)
        .set_sprite(24)
    ;

    world.create_data("tree", CLASS_PLANT)
        .set_lifetime(80, 1400)
        .set_altitude(4, 6)
        .set_height(3)
        .set_spawn(100, 50, 70, &PATTERN_CIRCLE)
        .set_sprite(23)
    ;

    world.create_data("scorpid", CLASS_CREATURE)
        .set_lifetime(8, 14)
        .set_altitude(4, 6)
        .set_height(2)
        .set_spawn(12, 4, 14, &PATTERN_CLOSE)
        .set_sprite(44)
    ;

    world
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        graphics::set_scaling(ctx, ScreenScaling::ShowAll);

        let world = create_world();

        let font = Font::new(ctx, "./assets/font/04b03.ttf")?;

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

fn get_neighbour_height(map: &Map, entity: &Entity, delta: Position) -> u8 {
    if let Some(tile) = map.get(entity.pos + delta) {
        if tile.entity.is_some() {
            return tile.entity_height;
        }
    }

    0
}

fn tick(world: &mut World) {
    let begin_time = Instant::now();

    let mut kill_list: Vec<EntityId> = Vec::new();
    let mut add_list: Vec<Entity> = Vec::new();

    for (entity_id, entity) in &mut world.entities {
        //println!("tick entity {:?}", entity);

        let data = &world.data_store[entity.data_id];

        // age tick
        entity.age += 1;
        if entity.age >= entity.lifetime {
            // die!
            kill_list.push(entity_id);
            world.map.get_mut(entity.pos).unwrap().entity = None;
            continue;
        }

        // energy tick
        match data.class { // FIXME use component instead!
            CLASS_PLANT => {
                let mut sun: i32 = 3;
                if get_neighbour_height(&world.map, entity, position::LEFT) >= entity.height { sun -= 1; }
                if get_neighbour_height(&world.map, entity, position::RIGHT) >= entity.height  { sun -= 1; }
                if get_neighbour_height(&world.map, entity, position::UP) >= entity.height { sun -= 1; }
                if get_neighbour_height(&world.map, entity, position::DOWN) >= entity.height { sun -= 1; }

                let energy = entity.energy as i32 + sun;
                if energy <= 0 {
                    // die!
                    kill_list.push(entity_id);
                    world.map.get_mut(entity.pos).unwrap().entity = None;
                    continue;
                }
                entity.energy = energy as u16;
            },
            CLASS_CREATURE => {
                entity.energy += 1; // TODO
            },
            _ => panic!("unknown entity class {}", data.class),
        }
        entity.energy += 1;

        // spawn tick
        if entity.age >= data.spawn_age_min && entity.age <= data.spawn_age_max && entity.energy >= data.spawn_cost {
            let pattern = data.spawn_pattern.unwrap();
            let offset = pattern[rand::random::<usize>() % pattern.len()];

            if let Some(tile) = world.map.get_mut(entity.pos + offset) {
                if tile.height >= data.altitude_min && tile.height <= data.altitude_max &&
                    (tile.is_free() || tile.entity_height < entity.height) {
                    
                    if let Some(entity_id) = tile.entity {
                        kill_list.push(entity_id);
                        tile.entity = None;
                    }
    
                    let spawned_entity = Entity::new(data, tile.pos);
                    tile.booked = true;
                    add_list.push(spawned_entity);

                    entity.energy -= data.spawn_cost;
                }
            }
        }
    };

    for entity_id in kill_list {
        world.entities.remove(entity_id);
    }

    for entity in add_list {
        let tile = world.map.get_mut(entity.pos).unwrap();
        tile.entity_height = entity.height; // FIXME
        let entity_id = world.entities.insert(entity);
        tile.entity = Some(entity_id);
        tile.booked = false;
    }

    println!("{}", begin_time.elapsed().as_micros() as f32 / 1000.0);
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
            spawn_entity(&mut self.world, 2, mouse_pos);
        }


        if input::is_key_pressed(ctx, Key::Num1) {
            spawn_entity(&mut self.world, 0, mouse_pos);
        }

        if input::is_key_pressed(ctx, Key::Num2) {
            spawn_entity(&mut self.world, 1, mouse_pos);
        }

        if input::is_key_pressed(ctx, Key::Num3) {
            spawn_entity(&mut self.world, 3, mouse_pos);
        }

        if input::is_key_pressed(ctx, Key::Tab) {
            self.show_cursor = !self.show_cursor;
        }

        if input::is_key_pressed(ctx, Key::R) {
            self.world = create_world();
            self.turn = 0;
            self.running = false;
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

fn main() -> tetra::Result {
    ContextBuilder::new("evo", SCREEN_WIDTH, SCREEN_HEIGHT)
        .tick_rate(30.0)
        .resizable(true)
        .fullscreen(false)
        .quit_on_escape(true)
        .build()?
        .run_with(GameState::new)
}
