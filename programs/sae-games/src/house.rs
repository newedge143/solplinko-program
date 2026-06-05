use super::*;

#[account]
#[derive(Default)]
pub struct House {
  pub bump: u8,
  pub authority: Pubkey,
  pub paused: bool,
  pub volume: u64,
  pub today_volume: u64,
  pub min_bet: u64,
  pub max_bet: u64,
  pub min_balls_count: u8,
  pub max_balls_count: u8,
}

impl House {
  pub const LEN: usize = 8  // DISCRIMINATOR
    + 1                     // BUMP
    + 32                    // AUTHORITY PUBKEY
    + 1                     // PAUSED
    + 8                     // VOLUME
    + 8                     // TODAY VOLUME
    + 8                     // MIN BET
    + 8                     // MAX BET
    + 1                     // MIN BALLS COUNT
    + 1;                    // MAX BALLS COUNT
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct InitHouseSettings {
  pub min_bet: u64,
  pub max_bet: u64,
  pub min_balls_count: u8,
  pub max_balls_count: u8,
  pub volume: Option<u64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct HouseSettings {
  pub paused: Option<bool>,
  pub min_bet: Option<u64>,
  pub max_bet: Option<u64>,
  pub min_balls_count: Option<u8>,
  pub max_balls_count: Option<u8>,
}

#[derive(Accounts)]
#[instruction(
  bump: u8,
  settings: InitHouseSettings,
)]
pub struct InitHouse<'info> {
  #[account(
    init,
    seeds = [
      b"house".as_ref(),
      authority.key().as_ref(),
    ],
    payer = authority,
    space = House::LEN,
    bump,
  )]
  pub house: Account<'info, House>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(
  bump: u8,
  settings: InitHouseSettings,
)]
pub struct InitTokenHouse<'info> {
  #[account(
    init,
    payer = authority,
    associated_token::mint = mint,
    associated_token::authority = house,
  )]
  pub house_ata: Account<'info, TokenAccount>,

  #[account(
    init,
    seeds = [
      b"house".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    payer = authority,
    space = House::LEN,
    bump,
  )]
  pub house: Account<'info, House>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub mint: Account<'info, Mint>,

  pub rent: Sysvar<'info, Rent>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  amount: u64,
)]
pub struct DepositHouse<'info> {
  #[account(
    mut,
    seeds = [
      b"house".as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = house.bump == _bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Account<'info, House>,

  #[account(mut)]
  pub source: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  amount: u64,
)]
pub struct DepositTokenHouse<'info> {
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = house,
  )]
  pub house_ata: Account<'info, TokenAccount>,

  #[account(
    seeds = [
      b"house".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = house.bump == _bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Account<'info, House>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = source,
  )]
  pub source_ata: Account<'info, TokenAccount>,

  #[account(mut)]
  pub source: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub mint: Account<'info, Mint>,

  pub rent: Sysvar<'info, Rent>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  amount: u64,
)]
pub struct WithdrawHouse<'info> {
  #[account(
    mut,
    seeds = [
      b"house".as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = house.bump == _bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Account<'info, House>,

  /// CHECK: authority account
  #[account(mut)]
  pub destination: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  amount: u64,
)]
pub struct WithdrawTokenHouse<'info> {
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = house,
  )]
  pub house_ata: Account<'info, TokenAccount>,

  #[account(
    seeds = [
      b"house".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = house.bump == _bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Account<'info, House>,

  #[account(
    init_if_needed,
    payer = authority,
    associated_token::mint = mint,
    associated_token::authority = destination,
  )]
  pub destination_ata: Account<'info, TokenAccount>,

  /// CHECK: destination account
  pub destination: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub mint: Account<'info, Mint>,

  pub rent: Sysvar<'info, Rent>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  settings: HouseSettings,
)]
pub struct UpdateHouse<'info> {
  #[account(
    mut,
    seeds = [
      b"house".as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = house.bump == _bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Account<'info, House>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  settings: HouseSettings,
)]
pub struct UpdateTokenHouse<'info> {
  #[account(
    mut,
    seeds = [
      b"house".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = house.bump == _bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Account<'info, House>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub mint: Account<'info, Mint>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(
  _fee_bump: u8,
  _house_bump: u8,
)]
pub struct ResetDailyStats<'info> {
  #[account(
    mut,
    seeds = [
      b"fee".as_ref(),
      authority.key().as_ref(),
    ],
    bump = _fee_bump,
    constraint = fee.bump == _fee_bump,
    constraint = fee.authority == authority.key(),
  )]
  pub fee: Account<'info, Fee>,

  #[account(
    mut,
    seeds = [
      b"house".as_ref(),
      authority.key().as_ref(),
    ],
    bump = _house_bump,
    constraint = house.bump == _house_bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Account<'info, House>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(
  _fee_bump: u8,
  _house_bump: u8,
)]
pub struct ResetTokenDailyStats<'info> {
  #[account(
    mut,
    seeds = [
      b"fee".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _fee_bump,
    constraint = fee.bump == _fee_bump,
    constraint = fee.authority == authority.key(),
  )]
  pub fee: Account<'info, Fee>,

  #[account(
    mut,
    seeds = [
      b"house".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _house_bump,
    constraint = house.bump == _house_bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Account<'info, House>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub mint: Account<'info, Mint>,

  pub system_program: Program<'info, System>
}
