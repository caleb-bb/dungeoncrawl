#![warn(clippy::pedantic)]

//These import the modules into this file to be accessed in our prelude definition
mod camera;
mod components;
mod map;
mod map_builder;
mod systems;

// These import all the stuff out of the modules to be used
// Any module that imports the prelude has access to all of this
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

// note that we inject the map into the resources using resources.insert. This is why resources must be declared as mutable
// we also inject the camera, because it, too, is a resource
impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
        // TODO - Render Draw Buffer
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_font("dungeonfont.png", 32, 32)
        .with_resource_path("resources/")
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
