#[derive(Copy, Clone, PartialEq)]

pub enum TurnState {
    AwaitingInput, 
    PlayerTurn,
    MonsterTurn,
    GameOver, 
    Victory
}