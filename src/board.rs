pub const BOARD_SIZE: usize = 22;

// Board spaces are numbered 1..=BOARD_SIZE

pub const PRICES: [u32; BOARD_SIZE as usize + 1] = [
    0, 30, 30, 40, 40, 50, 60, 70, 70, 80, 90, 110, 120, 130, 150, 170, 190,
    200, 200, 250, 250, 300, 300,
];

pub const MIN_FUNDS: [u32; BOARD_SIZE as usize + 1] = [
    0, 30, 50, 40, 160, 100, 150, 280, 80, 400, 180, 410, 500, 140, 630, 240,
    480, 600, 250, 250, 1300, 800, 340,
];

pub const LOAN_PAYMENT: [u32; BOARD_SIZE as usize + 1] = [
    0, 50, 50, 60, 80, 90, 90, 120, 120, 190, 170, 220, 260, 240, 360, 340,
    450, 550, 480, 500, 1020, 850, 1000,
];

pub const LOAN_TERM: [u32; BOARD_SIZE as usize + 1] = [
    0, 5, 5, 6, 6, 6, 7, 7, 7, 8, 8, 8, 9, 9, 9, 10, 10, 10, 12, 12, 12, 14, 14,
];

pub const BOND_COUPON: u32 = 5;
pub const CB_REPO_RATE: u32 = 2;
