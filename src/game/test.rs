use super::*;

#[test]
fn first_roll() {
    let state = GameState::new(4);
    assert!(state.players[0].position >= 2);
    assert!(state.players[0].position <= 12);
}
