use anchor_lang::constant;

#[constant]
pub const NUM_FASCIST_POLICIES: u8 = 11;
#[constant]
pub const NUM_LIBERAL_POLICIES: u8 = 6;
#[constant]
pub const LIBERAL_VICTORY_POLICIES: u8 = 5;
#[constant]
pub const FASCIST_VICTORY_POLICIES: u8 = 6;

#[constant]
pub const MIN_PLAYERS: u8 = 5;
#[constant]
pub const MAX_PLAYERS: u8 = 10;
#[constant]
pub const MINI_TURN_DURATION: i64 = 100;

#[constant]
pub const MAX_FAILED_ELECTIONS: u8 = 3;
#[constant]
pub const PRESIDENT_DRAW_SIZE: u8 = 3;
#[constant]
pub const CHANCELLOR_DRAW_SIZE: u8 = 2;
