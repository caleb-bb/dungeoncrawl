use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]

// this grabs all the entities that currently want to attack. Any entity that
// wants to attack will already have chosen its victim (see player_input for an
// example), so the victims can be found as a field in the attacking entity's
// WantsToAttack component, which is a struct
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect();
    // Notice that we create the collection from the iterator first, and only
    // then do we modify the stuff inside in a separate loop.

    victims.iter().for_each(|(message, victim)| {
        // this if let creates a mutable health variable only if the victim
        // actually has sa health component. entry_mut comes from Legion and
        // returns an option that gives us access to the entity and its
        // components. Meanwhile, get_component_mut gives us a mutable reference
        // to the component after we unwrap it.
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            println!("Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 {
                //if your HP < 1, you're dead. YOU LOSE. GOOD DAY SIR.
                commands.remove(*victim);
            }
        }
        // We remove the message from the command buffer because we've already executed it.
        commands.remove(*message);
    });
}

// Note that collect/0 takes the result of map and turns it into a collection.
// In the let statement, we specified that we wanted a vector of tuples
// containing Entity, Entity. Because we specified the type, collect knows how
// to gather up the result of map.
