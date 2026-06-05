use super::*;

#[error_code]
pub enum CustomErrors {
  #[msg("An account's balance was too small to complete the instruction")]
  InsufficientFunds,

  #[msg("The provided min bet should be greater than or equal to 10000000")]
  TooSmallMinBet,

  #[msg("The provided min bet should be greater than or equal to 1")]
  TooSmallMinBetToken,

  #[msg("The provided min balls count should be greater than or equal to 1")]
  TooSmallMinBallsCount,

  #[msg("DAO already verified")]
  DaoAlreadyVerified,

  #[msg("DAO not verified")]
  DaoNotVerified,

  #[msg("Changing dao are same")]
  DaoAreSame,

  #[msg("DAO already removed")]
  DaoAlreadyRemoved,

  #[msg("Referral already verified")]
  ReferralAlreadyVerified,

  #[msg("Referral not verified")]
  ReferralNotVerified,

  #[msg("Invite owner and registerer are same")]
  InviteOwnerAndRegistererAreSame,

  #[msg("Player profile has invite")]
  ProfileHasInvite,

  #[msg("Player profile hasn't invite")]
  ProfileHasNotInvite,

  #[msg("Game is paused")]
  GameIsPaused,

  #[msg("The provided rows should be between 8 and 16 (both inclusive)")]
  InvalidRows,

  #[msg("The provided risk should be 0 (Low), 1 (Medium) or 2 (High)")]
  InvalidRisk,

  #[msg("The provided bet amount per ball isn't correct")]
  InvalidBetAmount,

  #[msg("The provided balls count isn't correct")]
  InvalidBallsCount,

  #[msg("House balance is lower than bet amount")]
  InsufficientFundsInHouse,

  #[msg("The provided plinko settings isn't correct")]
  InvalidInitPlinkoSettings,

  #[msg("Game is already revealed")]
  GameIsRevealed,

  #[msg("Game is not revealed")]
  GameIsNotRevealed,

  #[msg("Game is already claimed")]
  GameIsClaimed,

  #[msg("Referral amount is 0")]
  ReferralAmountIsZero,

  #[msg("The provided rows should be between 0 and 100 (both inclusive)")]
  InvalidPercentage,
}
