use anchor_lang::error_code;

#[error_code]
pub enum GameErrorCode {
    #[msg("The Game must be in the required game state")]
    InvalidGameState,
    #[msg("Player has already joined the game")]
    PlayerAlreadyJoined,
    #[msg("The game is full")]
    MaxPlayersReached,
    #[msg("The specified player is not actively in the game")]
    PlayerNotInGame,
    #[msg("The last player(host) Can not leave the game. Consider ending the game instead")]
    LastPlayerLeaving,
    #[msg("Bet vault account must be provided only if bet amount is provided")]
    BetNotFound,
    #[msg("Deposit vault acount must be provided only if deposit amount is provided")]
    DepositNotFound,
    #[msg("5 to 10 players are needed to play")]
    MinimumPlayersNotReached,
    #[msg("The nominated chancellor is ineligible")]
    IneligibleChancellorNominated,
    #[msg("Eliminated Player can not participate in game")]
    EiminatedPlayer,
    #[msg("Number of votes can not exceed player count")]
    MaxVotesReached,
    #[msg("Each Player can vote once per nomination")]
    PlayerAlreadyVoted,
    #[msg("Wait for turn duration to finish")]
    TurnNotFinished,
    #[msg("Turn Duration needs to be atleast 60 seconds")]
    MinimumTurnDurationNotReached,
    #[msg("Turn has already finished")]
    TurnFinished,
    #[msg("The president enacting policy needs LegistlativePresident State")]
    PresidentPolicyError,
    #[msg("The Chancellor enacting policy needs LegistlativeChancellor State and PolicyCard")]
    ChancellorPolicyError,
    #[msg("the player needs to be a chancellor or president")]
    PlayerNotInGovernment,
    #[msg("Player is not the current president")]
    PresidentRoleRequired,
    #[msg("The previous president index was ot found in game_data")]
    PrevPresidentNotFound,
    #[msg("The previous chancellor index was ot found in game_data")]
    PrevChancellorNotFound,
    #[msg("Player needs to be the chancellor")]
    ChancellorRoleNeeded,
    #[msg("The starting player count could not be found in game data")]
    StartPlayerCountNotFound,
    #[msg("Could not find the turn started at timestamp")]
    TurnStartTimeNotFound,
    #[msg("Nomination account is needed to check for inactive voters for chancellor nomination game state")]
    NominationAccountNotFound,
}
