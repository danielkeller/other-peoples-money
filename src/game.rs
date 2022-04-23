use rand::{thread_rng, Rng};

use crate::asset::*;
use crate::balance_sheet::*;
use crate::board::*;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TurnState {
    /// Need to pay interest
    PassingGo,
    /// Need to make customer payment
    ToPay,
    /// Business defaulted, decide what to do
    // LoanDefaulted,
    /// Can buy and sell etc
    Done,
    // Bankruptcy
}

#[derive(Debug, Clone)]
struct PlayerState {
    position: usize,
    points: u32,
}

#[derive(Debug, Clone)]
struct GameState {
    players: Box<[PlayerState]>,
    assets: Box<[BalanceSheet]>,
    turn: Player,
    state: TurnState,
}

fn dice_roll() -> usize {
    let mut rng = thread_rng();
    rng.gen_range(1..=6) + rng.gen_range(1..=6)
}

impl GameState {
    pub fn new(n_players: usize) -> GameState {
        let init_player = PlayerState { position: 0, points: 0 };
        let mut state = GameState {
            players: vec![init_player; n_players].into_boxed_slice(),
            assets: vec![BalanceSheet::new(); n_players].into_boxed_slice(),
            turn: Player(0),
            state: TurnState::Done,
        };
        state.players[0].position = dice_roll();
        state
    }

    /// How much reserves are required to pay and exit the current state?
    pub fn required_reserves(&self) -> u32 {
        match self.state {
            TurnState::ToPay => {
                self.current_player_assets().customer_payment_amount(Customer(
                    self.current_player().position,
                ))
            }
            TurnState::PassingGo => {
                self.current_player_assets().interest_amount()
            }
            _ => 0,
        }
    }

    /// Panics if state is exited incorrectly
    pub fn exit_state(&mut self) {
        match self.state {
            TurnState::PassingGo => self.interest_payment(),
            TurnState::ToPay => self.customer_payment(),
            // TurnState::LoanDefaulted => todo!(),
            TurnState::Done => self.roll(),
        }
    }

    fn roll(&mut self) {
        self.turn.0 += 1;
        self.turn.0 %= self.players.len();

        let current_pos = &mut self.current_player_mut().position;
        let mut end_pos = *current_pos + dice_roll();
        if end_pos > BOARD_SIZE {
            end_pos -= BOARD_SIZE;
            *current_pos = end_pos;
            self.state = TurnState::PassingGo;
        } else {
            *current_pos = end_pos;
            if self.find_lender(Customer(end_pos)).is_some() {
                self.state = TurnState::ToPay;
            } else {
                self.state = TurnState::Done;
            }
        }
    }

    fn customer_payment(&mut self) {
        let customer = Customer(self.current_player().position);
        let lender = self.find_lender(customer).unwrap();
        let amount =
            self.current_player_assets_mut().customer_payment(customer);
        let assets = &mut self.assets[lender.0];
        assets.reserves += amount;
        if amount >= LOAN_PAYMENT[customer.0] {
            assets.customer_interest(customer, amount);
            self.state = TurnState::Done;
        } else {
            *assets.deposits.get_mut(&customer).unwrap() += amount;
            *assets.loans.get_mut(&customer).unwrap() += 1;
            self.state = TurnState::Done;
            // self.state = TurnState::LoanDefaulted;
        }
    }

    fn interest_payment(&mut self) {
        BalanceSheet::pay_interest(&mut self.assets, self.turn)
    }

    fn current_player(&self) -> &PlayerState {
        &self.players[self.turn.0]
    }

    fn current_player_mut(&mut self) -> &mut PlayerState {
        &mut self.players[self.turn.0]
    }

    fn current_player_assets(&self) -> &BalanceSheet {
        &self.assets[self.turn.0]
    }

    fn current_player_assets_mut(&mut self) -> &mut BalanceSheet {
        &mut self.assets[self.turn.0]
    }

    fn find_lender(&self, c: Customer) -> Option<Player> {
        self.assets.iter().position(|assets| assets.has_customer(c)).map(Player)
    }
}
