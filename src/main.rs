mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;

    pub const SCREEN_WIDTH: i32 = 146;
    pub const SCREEN_HEIGHT: i32 = 160;
    pub const DISPLAY_WIDTH: i32 = 40;
    pub const DISPLAY_HEIGHT: i32 = 25;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let map_builder = MapBuilder::new();

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(0_usize);
        resources.insert(TurnState::AwaitingInput);

        spawn_player(&mut ecs, map_builder.player_start);
        spawn_key(&mut ecs, map_builder.key_pos);
        map_builder
            .immovable_enemies
            .iter()
            .for_each(|pos| spawn_immovable_enemy(&mut ecs, *pos));

        map_builder
            .movable_enemies
            .iter()
            .for_each(|pos| spawn_movable_enemy(&mut ecs, *pos));

        map_builder
            .potions
            .iter()
            .for_each(|pos| spawn_healing_potion(&mut ecs, *pos));
 
        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let map_builder = MapBuilder::new();

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(0_usize);
        self.resources.insert(TurnState::AwaitingInput);

        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_key(&mut self.ecs, map_builder.key_pos);
        map_builder
            .immovable_enemies
            .iter()
            .for_each(|pos| spawn_immovable_enemy(&mut self.ecs, *pos));

        map_builder
            .movable_enemies
            .iter()
            .for_each(|pos| spawn_movable_enemy(&mut self.ecs, *pos));

        map_builder
            .potions
            .iter()
            .for_each(|pos| spawn_healing_potion(&mut self.ecs, *pos));
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            7,
            GREEN,
            BLACK,
            "Press 1 to \
        play again.",
        );
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended");
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
        self.resources.insert(ctx.key);
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => self.game_over(ctx),
            TurnState::Victory => self.victory(ctx)
        }
        render_draw_buffer(ctx).expect("Render Error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont_custom_1.png", 32, 32)
        .with_font("dungeonfont_custom_2.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        //floor - 0
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont_custom_1.png")
        //walls & entities - 1
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont_custom_1.png")
        //monsters - 2
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont_custom_2.png")
        //tooltip - 3
        .with_simple_console_no_bg(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
