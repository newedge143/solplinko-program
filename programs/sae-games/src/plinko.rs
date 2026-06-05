use super::*;

#[account]
#[derive(Default)]
pub struct Plinko {
  pub bump: u8,
  pub id: u64,
  pub authority: Pubkey,
  pub player: Pubkey,
  pub row: Option<u8>,
  pub risk: Option<u8>,
  pub degen_mode: bool,
  pub balls_count: u8,
  pub bet_amount: u64,
  pub payout: u64,
  pub timestamp: i64,
  pub revealed: bool,
  pub paid_out: bool,
  pub dao: Option<Pubkey>,
  pub referral: Option<Pubkey>,
}

impl Plinko {
  pub const LEN: usize = 8  // DISCRIMINATOR
    + 1                     // BUMP
    + 8                     // ID
    + 32                    // AUTHORITY PUBKEY
    + 32                    // PLAYER PUBKEY
    + 2                     // ROW
    + 2                     // RISK
    + 1                     // DEGEN MODE
    + 1                     // BALLS COUNT
    + 8                     // BET AMOUNT
    + 8                     // PAYOUT
    + 8                     // TIMESTAMP
    + 1                     // REVEALED
    + 1                     // PAID OUT
    + 1 + 32                // DAO PUBKEY
    + 1 + 32;               // REFERRAL PUBKEY
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Debug, Clone)]
pub struct InitPlinkoSettings {
  pub row: Option<u8>,
  pub risk: Option<u8>,
  pub degen_mode: bool,
  pub balls_count: u8,
  pub bet_amount: u64,
}

#[derive(Accounts)]
#[instruction(
  bump: u8,
  _house_bump: u8,
  _profile_bump: u8,
  settings: InitPlinkoSettings,
)]
pub struct InitPlinko<'info> {
  #[account(
    init,
    seeds = [
      b"plinko".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
      &profile.next_game_id.to_le_bytes(),
    ],
    payer = player,
    space = Plinko::LEN,
    bump,
  )]
  pub plinko: Account<'info, Plinko>,

  #[account(
    seeds = [
      b"house".as_ref(),
      authority.key().as_ref(),
    ],
    bump = _house_bump,
    constraint = house.bump == _house_bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Account<'info, House>,

  #[account(
    mut,
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _profile_bump,
    constraint = profile.bump == _profile_bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Account<'info, Profile>,

  #[account(mut)]
  pub player: Signer<'info>,

  /// CHECK: authority account
  #[account(mut)]
  pub authority: AccountInfo<'info>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  bump: u8,
  _house_bump: u8,
  _profile_bump: u8,
  settings: InitPlinkoSettings,
)]
pub struct InitTokenPlinko<'info> {
  #[account(
    init_if_needed,
    payer = player,
    associated_token::mint = mint,
    associated_token::authority = plinko,
  )]
  pub plinko_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    init,
    seeds = [
      b"plinko".as_ref(),
      player.key().as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
      &profile.next_game_id.to_le_bytes(),
    ],
    payer = player,
    space = Plinko::LEN,
    bump,
  )]
  pub plinko: Box<Account<'info, Plinko>>,

  #[account(
    associated_token::mint = mint,
    associated_token::authority = house,
  )]
  pub house_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    seeds = [
      b"house".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _house_bump,
    constraint = house.bump == _house_bump,
    constraint = house.authority == authority.key(),
  )]
  pub house: Box<Account<'info, House>>,

  #[account(
    mut,
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _profile_bump,
    constraint = profile.bump == _profile_bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Box<Account<'info, Profile>>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = player,
  )]
  pub player_ata: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub player: Signer<'info>,

  /// CHECK: authority account
  #[account(mut)]
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
  _fee_bump: u8,
  _house_bump: u8,
  _profile_bump: u8,
  _id: u64,
  payout: u64,
)]
pub struct RevealPlinko<'info> {
  #[account(
    mut,
    seeds = [
      b"plinko".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
      &_id.to_le_bytes(),
    ],
    bump = _bump,
    constraint = plinko.bump == _bump,
    constraint = plinko.id == _id,
    constraint = plinko.player == player.key(),
    constraint = plinko.authority == authority.key(),
  )]
  pub plinko: Account<'info, Plinko>,

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

  #[account(
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _profile_bump,
    constraint = profile.bump == _profile_bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Account<'info, Profile>,

  /// CHECK: player account
  pub player: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  _fee_bump: u8,
  _house_bump: u8,
  _profile_bump: u8,
  id: u64,
  payout: u64,
)]
pub struct RevealTokenPlinko<'info> {
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = plinko,
  )]
  pub plinko_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    seeds = [
      b"plinko".as_ref(),
      player.key().as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
      &id.to_le_bytes(),
    ],
    bump = _bump,
    constraint = plinko.bump == _bump,
    constraint = plinko.id == id,
    constraint = plinko.player == player.key(),
    constraint = plinko.authority == authority.key(),
  )]
  pub plinko: Box<Account<'info, Plinko>>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = fee,
  )]
  pub fee_ata: Box<Account<'info, TokenAccount>>,

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
  pub fee: Box<Account<'info, Fee>>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = house,
  )]
  pub house_ata: Box<Account<'info, TokenAccount>>,

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
  pub house: Box<Account<'info, House>>,

  #[account(
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _profile_bump,
    constraint = profile.bump == _profile_bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Box<Account<'info, Profile>>,

  /// CHECK: player account
  pub player: AccountInfo<'info>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = authority,
  )]
  pub authority_ata: Box<Account<'info, TokenAccount>>,

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
  _fee_bump: u8,
  _house_bump: u8,
  _profile_bump: u8,
  _referral_bump: u8,
  _id: u64,
  payout: u64,
)]
pub struct RevealPlinkoWithReferral<'info> {
  #[account(
    mut,
    seeds = [
      b"plinko".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
      &_id.to_le_bytes(),
    ],
    bump = _bump,
    constraint = plinko.bump == _bump,
    constraint = plinko.id == _id,
    constraint = plinko.player == player.key(),
    constraint = plinko.authority == authority.key(),
  )]
  pub plinko: Box<Account<'info, Plinko>>,

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
  pub fee: Box<Account<'info, Fee>>,

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
  pub house: Box<Account<'info, House>>,

  #[account(
    mut,
    seeds = [
      b"referral".as_ref(),
      referral_owner.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.authority == authority.key(),
    constraint = referral.owner == referral_owner.key(),
  )]
  pub referral: Box<Account<'info, Referral>>,

  /// CHECK: referral owner account
  pub referral_owner: AccountInfo<'info>,

  #[account(
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _profile_bump,
    constraint = profile.bump == _profile_bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
    constraint = profile.invitation == Some(referral.key()),
  )]
  pub profile: Box<Account<'info, Profile>>,

  /// CHECK: player account
  pub player: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  _fee_bump: u8,
  _house_bump: u8,
  _profile_bump: u8,
  _referral_bump: u8,
  id: u64,
  payout: u64,
)]
pub struct  RevealTokenPlinkoWithReferral<'info> {
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = plinko,
  )]
  pub plinko_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    seeds = [
      b"plinko".as_ref(),
      player.key().as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
      &id.to_le_bytes(),
    ],
    bump = _bump,
    constraint = plinko.bump == _bump,
    constraint = plinko.id == id,
    constraint = plinko.player == player.key(),
    constraint = plinko.authority == authority.key(),
  )]
  pub plinko: Box<Account<'info, Plinko>>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = fee,
  )]
  pub fee_ata: Box<Account<'info, TokenAccount>>,

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
  pub fee: Box<Account<'info, Fee>>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = house,
  )]
  pub house_ata: Box<Account<'info, TokenAccount>>,

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
  pub house: Box<Account<'info, House>>,

  #[account(
    init_if_needed,
    payer = authority,
    associated_token::mint = mint,
    associated_token::authority = referral,
  )]
  pub referral_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    seeds = [
      b"referral".as_ref(),
      referral_owner.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.authority == authority.key(),
    constraint = referral.owner == referral_owner.key(),
  )]
  pub referral: Box<Account<'info, Referral>>,

  /// CHECK: referral owner account
  pub referral_owner: AccountInfo<'info>,

  #[account(
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _profile_bump,
    constraint = profile.bump == _profile_bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
    constraint = profile.invitation == Some(referral.key()),
  )]
  pub profile: Box<Account<'info, Profile>>,

  /// CHECK: player account
  pub player: AccountInfo<'info>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = authority,
  )]
  pub authority_ata: Box<Account<'info, TokenAccount>>,

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
  _id: u64,
)]
pub struct ClaimPlinko<'info> {
  #[account(
    mut,
    seeds = [
      b"plinko".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
      &_id.to_le_bytes(),
    ],
    bump = _bump,
    constraint = plinko.bump == _bump,
    constraint = plinko.id == _id,
    constraint = plinko.player == player.key(),
    constraint = plinko.authority == authority.key(),
  )]
  pub plinko: Account<'info, Plinko>,

  #[account(mut)]
  pub player: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  id: u64,
)]
pub struct ClaimTokenPlinko<'info> {
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = plinko,
  )]
  pub plinko_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    mut,
    seeds = [
      b"plinko".as_ref(),
      player.key().as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
      &id.to_le_bytes(),
    ],
    bump = _bump,
    constraint = plinko.bump == _bump,
    constraint = plinko.id == id,
    constraint = plinko.player == player.key(),
    constraint = plinko.authority == authority.key(),
  )]
  pub plinko: Account<'info, Plinko>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = player,
  )]
  pub player_ata: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub player: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub mint: Account<'info, Mint>,

  pub rent: Sysvar<'info, Rent>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}
