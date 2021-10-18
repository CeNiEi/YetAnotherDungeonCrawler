use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect::<Vec<(Entity, Entity, Entity)>>();

    victims.iter().for_each(|(message, attacker, _)| {
        commands.remove(*attacker);
        commands.remove(*message);
    });
}
