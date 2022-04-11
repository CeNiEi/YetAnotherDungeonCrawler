use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Key)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut player_stats = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut key = <&Point>::query().filter(component::<Key>());

    let key_pos = key.iter(ecs).nth(0).unwrap();


    let current_state = turn_state.clone();

    let mut new_state = match current_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state
    };

    player_stats.iter(ecs).for_each(|(hp, pos)| {
        if hp.current < 1 {
            new_state = TurnState::GameOver;
        }
        if pos == key_pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}