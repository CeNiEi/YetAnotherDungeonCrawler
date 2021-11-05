use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect::<Vec<(Entity, Entity, Entity)>>();

    victims.iter().for_each(|(message, attacker, victim)| {
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 0;
            println!("{}, {}", health.current + 1, health.current);
            if health.current < 1 {
                commands.remove(*victim);
            }
        }
        if let Ok(health) = ecs.entry_mut(*attacker).unwrap().get_component::<Health>() {
            if health.current < 1 {
                commands.remove(*attacker);
            }
        }
        commands.remove(*message);
    });
}

#[system]
#[write_component(Health)]
#[read_component(Ranged)]
#[read_component(RangedSprite)]
pub fn auto_reduce_health(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let elements = <(Entity, &RangedSprite)>::query()
        .filter(component::<Ranged>())
        .iter(ecs)
        .filter(|(_, ranged_sprite)| ranged_sprite.mode == RangedSpriteMode::Landed)
        .map(|(entity, _)| *entity)
        .collect::<Vec<Entity>>();

    elements.iter().for_each(|entity| {
        if let Ok(health) = ecs
            .entry_mut(*entity)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 {
                commands.remove(*entity)
            }
        }
    });
}
