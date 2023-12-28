use crate::prelude::*;

// #[system] here is an annotation that grants player_input/0 a procedural macro
// named system.  This macros transforms our function name into
// player_input_system and wraps it with all of the extra code Legion requires
// to construct a system in the ECS sense.
// write_component requests writable
// access to a component type. In this case, it lets us write to the contents of
// the Point component.  read_component is exactly what it says on the tin. It
// gives us read-only access to a component type. I'm not certain, but I suspect
// this is related to Rust's notion of ownership, lifetimes, memory management,
// etc. Read-only access is prevented while a component is being written to? Or
// while it's owned by a write-access process? I'm not sure...
// This function also has a "Subworld", which is similar to a World but can only
// see the components requested in this function. I think that means the
// components requested by the macro annotations. The #[resource] requests
// access to types stored in Legion's resource handler. It is also a procedural
// macro. Listing map: &Map lets us access the map via a read-only reference.
// Finally, &mut Camera gives us a mutable reference ot the camear, which lets
// us change the contents of the Camera struct. The Camera is a global resource,
// so it will be updated with new values. Note that this makes sense, logically,
// because only one player would see any given camera at any given time. It's
// unlikely that down the line we would have multiple players fighting for
// control of the camera.

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        if delta.x != 0 || delta.y != 0 {
            let mut players = <&mut Point>::query().filter(component::<Player>());
            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}

// Notice that, on the line beginning with `let mut players...` we use a query.
// Queries return references, which are mutable if we use &mut on the query. If
// we request more than one component type, the query returns only those
// entities having ALL of those types.  The .filter() statement is necessary
// because our query will return a mutable reference to every single entity
// having a Point component. To get around this, we use
// .filter(component::<Player>()) to only match entities also having a Player
// tag.
// The query does not become an iterator until we call .iter or .iter_mut on it.
// It's still a query, so adding filters before the iterator call limits the
// types included in the query. Query filters can require that a component
// exists, but can't refer to its content. If we need to filter on the
// component's content, say, on a struct field, then we can use the iterator's
// filter() function instead
