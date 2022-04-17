use crate::prelude::*;
#[system]
#[read_component(ActivateItem)]
#[read_component(Healer)]
#[write_component(Health)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut healing_to_apply = Vec::<Entity>::new();
    <(Entity, &ActivateItem)
        >::query().iter(ecs).for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if item.get_component::<Healer>().is_ok() {
                    healing_to_apply.push(activate.used_by);
                }
            }
            commands.remove(activate.item);
            commands.remove(*entity);
        });
    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(*heal) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = health.max;
            }
        }
    }
}
