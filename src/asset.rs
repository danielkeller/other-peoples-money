/// Identifies a loan made to a tile on the board
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Customer(pub usize);

/// Identifies a player
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Player(pub usize);

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Collateral {
    Loan { customer: Customer },
    Bond { player: Player, count: u32 },
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Repo {
    pub count: u32,
    pub interest: u32,
}

impl Repo {
    pub fn payment(&self) -> u32 {
        self.count * self.interest
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum CbCollateral {
    Loan { customer: Customer },
    Bond { player: Player, count: u32 },
    Repo { player: Player, collateral: Collateral },
}
