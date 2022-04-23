use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use crate::asset::*;
use crate::board::*;

#[derive(Debug, Clone, Default)]
pub struct BalanceSheet {
    pub reserves: u32,
    pub deposits: HashMap<Customer, u32>,
    pub loans: HashMap<Customer, u32>,
    pub short_bonds: u32,
    pub bonds: HashMap<Player, u32>,
    pub short_repos: HashMap<Collateral, Repo>,
    pub repos: HashSet<(Player, Collateral)>,
    pub cb_repos: HashMap<CbCollateral, u32>,
}

impl BalanceSheet {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn has_customer(&self, c: Customer) -> bool {
        self.loans.contains_key(&c)
    }

    pub fn customer_payment_amount(&self, to: Customer) -> u32 {
        let mut amount = 0;
        for &size in self.deposits.values() {
            if size >= MIN_FUNDS[to.0] {
                amount += PRICES[to.0];
            }
        }
        amount
    }

    /// Panics if the player can't afford it.
    pub fn customer_payment(&mut self, to: Customer) -> u32 {
        let mut amount = 0;
        for size in self.deposits.values_mut() {
            if *size >= MIN_FUNDS[to.0] {
                *size -= PRICES[to.0];
                amount += PRICES[to.0];
            }
        }
        assert!(self.reserves <= amount);
        self.reserves -= amount;
        amount
    }

    /// Panics if the amount isn't enough or the loan is done
    pub fn customer_interest(&mut self, c: Customer, amount: u32) {
        assert!(amount >= LOAN_PAYMENT[c.0]);
        *self.deposits.get_mut(&c).unwrap() += amount - LOAN_PAYMENT[c.0];
        let mut loan = match self.loans.entry(c) {
            Entry::Occupied(e) => e,
            _ => panic!("No loan"),
        };
        *loan.get_mut() -= 1;
        if *loan.get() == 0 {
            loan.remove();
        }
    }

    pub fn interest_amount(&self) -> u32 {
        let repo_int: u32 = self.short_repos.values().map(Repo::payment).sum();
        let cb_repo_int = self.cb_repos.values().sum::<u32>() * CB_REPO_RATE;
        repo_int + cb_repo_int + self.short_bonds * BOND_COUPON
    }

    /// Panics if the player can't afford it.
    pub fn pay_interest(assets: &mut [Self], from: Player) {
        let amount = assets[from.0].interest_amount();
        assert!(assets[from.0].reserves <= amount);
        assets[from.0].reserves -= amount;
        for to in 0..assets.len() {
            if let Some(n) = assets[to].bonds.get(&from) {
                assets[to].reserves += BOND_COUPON * n;
            }
            for &(player, collateral) in &assets[to].repos {
                if player == from {
                    let repo = assets[from.0].short_repos.get(&collateral);
                    assets[to].reserves += repo.unwrap().payment();
                }
            }
        }
    }
}
