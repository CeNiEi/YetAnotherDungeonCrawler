use crate::prelude::*;

#[system]
#[read_component(WantsToChangeRangedSpriteMode)]
#[write_component(Ranged)]
#[write_component(RangedSprite)]
pub fn change_ranged_sprite_mode(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut messages = <(Entity, &WantsToChangeRangedSpriteMode)>::query();

    let entities = messages
        .iter(ecs)
        .map(|(entity, message)| (*entity, message.entity, message.mode))
        .collect::<Vec<(Entity, Entity, RangedSpriteMode)>>();

    entities
        .iter()
        .for_each(|(message_entity, changing_entity, new_mode)| {
            if let Ok(ranged_sprite) = ecs
                .entry_mut(*changing_entity)
                .unwrap()
                .get_component_mut::<RangedSprite>()
            {
                ranged_sprite.mode = *new_mode;
            }
            commands.remove(*message_entity);
        });
}
