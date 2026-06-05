pub mod errors;
pub mod utils;
pub mod fee;
pub mod house;
pub mod referral;
pub mod dao;
pub mod profile;
pub mod plinko;

use {
  anchor_lang::{
    prelude::*,
    solana_program::{
      system_instruction,
      entrypoint::{
        ProgramResult,
      },
      program_error::{
        ProgramError,
      },
      program::{
        invoke,
      },
    },
  },
  anchor_spl::{
    token::{
      TokenAccount,
      Mint,
      Token,
      transfer,
      Transfer,
      close_account,
      CloseAccount,
    },
    associated_token::{
      AssociatedToken,
    }
  },
  errors::*,
  utils::*,
  fee::*,
  house::*,
  referral::*,
  dao::*,
  profile::*,
  plinko::*,
};

declare_id!("PUBKEY");

#[program]
pub mod sae_games {
  use super::*;

  // FEE PDA INSTRUCTIONS

  pub fn init_fee(
    ctx: Context<InitFee>,
    bump: u8,
    volume: u64,
  ) -> Result<()> {
    let fee = &mut ctx.accounts.fee;
    let authority = &mut ctx.accounts.authority;

    fee.bump = bump;
    fee.volume = volume;
    fee.referral_volume = 0;
    fee.referral_today_volume = 0;
    fee.authority = *authority.to_account_info().key;

    Ok(())
  }

  pub fn init_token_fee(
    ctx: Context<InitTokenFee>,
    bump: u8,
    volume: u64,
  ) -> Result<()> {
    let fee = &mut ctx.accounts.fee;
    let authority = &mut ctx.accounts.authority;

    fee.bump = bump;
    fee.volume = volume;
    fee.referral_volume = 0;
    fee.referral_today_volume = 0;
    fee.authority = *authority.to_account_info().key;

    Ok(())
  }

  pub fn deposit_fee<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositFee<'info>>,
    _bump: u8,
    amount: u64,
  ) -> Result<()> {
    let source = &mut ctx.accounts.source;
    let fee = &mut ctx.accounts.fee;

    return transfer_invoke(
      &source.to_account_info(),
      &fee.to_account_info(),
      amount,
    );
  }

  pub fn deposit_token_fee<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositTokenFee<'info>>,
    _bump: u8,
    amount: u64,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let source = &mut ctx.accounts.source;

    let source_ata = &mut ctx.accounts.source_ata;
    let fee_ata = &mut ctx.accounts.fee_ata;

    return transfer_spl_invoke(
      &source_ata.to_account_info(),
      &fee_ata.to_account_info(),
      amount,
      &token_program.to_account_info(),
      &source.to_account_info(),
    );
  }

  pub fn withdraw_fee<'info>(
    ctx: Context<'_, '_, '_, 'info, WithdrawFee<'info>>,
    _bump: u8,
    amount: u64,
  ) -> Result<()> {
    let fee = &mut ctx.accounts.fee;
    let destination = &mut ctx.accounts.destination;

    return transfer_invoke_signed(
      &mut fee.to_account_info(),
      &mut destination.to_account_info(),
      amount,
    ).map_err(Into::into);
  }

  pub fn withdraw_token_fee<'info>(
    ctx: Context<'_, '_, '_, 'info, WithdrawTokenFee<'info>>,
    _bump: u8,
    amount: u64,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let fee = &mut ctx.accounts.fee;
    let authority = &mut ctx.accounts.authority;
    let mint = &mut ctx.accounts.mint;

    let fee_ata = &mut ctx.accounts.fee_ata;
    let destination_ata = &mut ctx.accounts.destination_ata;

    return transfer_spl_invoke_signed(
      &fee_ata.to_account_info(),
      &destination_ata.to_account_info(),
      amount,
      &token_program.to_account_info(),
      &fee.to_account_info(),
      &[&[
        b"fee".as_ref(),
        mint.key().as_ref(),
        authority.key().as_ref(),
        &[fee.bump],
      ]],
    );
  }

  // HOUSE PDA INSTRUCTIONS

  pub fn init_house(
    ctx: Context<InitHouse>,
    bump: u8,
    settings: InitHouseSettings,
  ) -> Result<()> {
    let house = &mut ctx.accounts.house;
    let authority = &mut ctx.accounts.authority;

    require!(settings.min_bet >= 10_000_000, CustomErrors::TooSmallMinBet);
    require!(settings.min_balls_count >= 1, CustomErrors::TooSmallMinBallsCount);

    match settings.volume {
      Some(volume) => house.volume = volume,
      None => house.volume = 0,
    }

    house.bump = bump;
    house.paused = false;
    house.today_volume = 0;
    house.min_bet = settings.min_bet;
    house.max_bet = settings.max_bet;
    house.min_balls_count = settings.min_balls_count;
    house.max_balls_count = settings.max_balls_count;
    house.authority = *authority.to_account_info().key;

    Ok(())
  }

  pub fn init_token_house(
    ctx: Context<InitTokenHouse>,
    bump: u8,
    settings: InitHouseSettings,
  ) -> Result<()> {
    let house = &mut ctx.accounts.house;
    let authority = &mut ctx.accounts.authority;

    require!(settings.min_bet >= 1, CustomErrors::TooSmallMinBetToken);
    require!(settings.min_balls_count >= 1, CustomErrors::TooSmallMinBallsCount);

    match settings.volume {
      Some(volume) => house.volume = volume,
      None => house.volume = 0,
    }

    house.bump = bump;
    house.paused = false;
    house.today_volume = 0;
    house.min_bet = settings.min_bet;
    house.max_bet = settings.max_bet;
    house.min_balls_count = settings.min_balls_count;
    house.max_balls_count = settings.max_balls_count;
    house.authority = *authority.to_account_info().key;

    Ok(())
  }

  pub fn deposit_house<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositHouse<'info>>,
    _bump: u8,
    amount: u64,
  ) -> Result<()> {
    let source = &mut ctx.accounts.source;
    let house = &mut ctx.accounts.house;

    return transfer_invoke(
      &source.to_account_info(),
      &house.to_account_info(),
      amount,
    );
  }

  pub fn deposit_token_house<'info>(
    ctx: Context<'_, '_, '_, 'info, DepositTokenHouse<'info>>,
    _bump: u8,
    amount: u64,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let source = &mut ctx.accounts.source;

    let source_ata = &mut ctx.accounts.source_ata;
    let house_ata = &mut ctx.accounts.house_ata;

    return transfer_spl_invoke(
      &source_ata.to_account_info(),
      &house_ata.to_account_info(),
      amount,
      &token_program.to_account_info(),
      &source.to_account_info(),
    );
  }

  pub fn withdraw_house<'info>(
    ctx: Context<'_, '_, '_, 'info, WithdrawHouse<'info>>,
    _bump: u8,
    amount: u64,
  ) -> Result<()> {
    let house = &mut ctx.accounts.house;
    let destination = &mut ctx.accounts.destination;

    return transfer_invoke_signed(
      &mut house.to_account_info(),
      &mut destination.to_account_info(),
      amount,
    ).map_err(Into::into);
  }

  pub fn withdraw_token_house<'info>(
    ctx: Context<'_, '_, '_, 'info, WithdrawTokenHouse<'info>>,
    _bump: u8,
    amount: u64,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let house = &mut ctx.accounts.house;
    let authority = &mut ctx.accounts.authority;
    let mint = &mut ctx.accounts.mint;

    let house_ata = &mut ctx.accounts.house_ata;
    let destination_ata = &mut ctx.accounts.destination_ata;

    return transfer_spl_invoke_signed(
      &house_ata.to_account_info(),
      &destination_ata.to_account_info(),
      amount,
      &token_program.to_account_info(),
      &house.to_account_info(),
      &[&[
        b"house".as_ref(),
        mint.key().as_ref(),
        authority.key().as_ref(),
        &[house.bump],
      ]],
    );
  }

  pub fn update_house(
    ctx: Context<UpdateHouse>,
    _bump: u8,
    settings: HouseSettings,
  ) -> Result<()> {
    let house = &mut ctx.accounts.house;

    match settings.paused {
      Some(paused) => house.paused = paused,
      None => {},
    }

    match settings.min_bet {
      Some(min_bet) => {
        require!(min_bet >= 10_000_000, CustomErrors::TooSmallMinBet);
        house.min_bet = min_bet;
      },
      None => {},
    }

    match settings.max_bet {
      Some(max_bet) => house.max_bet = max_bet,
      None => {},
    }

    match settings.min_balls_count {
      Some(min_balls_count) => {
        require!(min_balls_count >= 1, CustomErrors::TooSmallMinBallsCount);
        house.min_balls_count = min_balls_count;
      },
      None => {},
    }

    match settings.max_balls_count {
      Some(max_balls_count) => house.max_balls_count = max_balls_count,
      None => {},
    }

    Ok(())
  }

  pub fn update_token_house(
    ctx: Context<UpdateTokenHouse>,
    _bump: u8,
    settings: HouseSettings,
  ) -> Result<()> {
    let house = &mut ctx.accounts.house;

    match settings.paused {
      Some(paused) => house.paused = paused,
      None => {},
    }

    match settings.min_bet {
      Some(min_bet) => {
        require!(min_bet >= 1, CustomErrors::TooSmallMinBetToken);
        house.min_bet = min_bet;
      },
      None => {},
    }

    match settings.max_bet {
      Some(max_bet) => house.max_bet = max_bet,
      None => {},
    }

    match settings.min_balls_count {
      Some(min_balls_count) => {
        require!(min_balls_count >= 1, CustomErrors::TooSmallMinBallsCount);
        house.min_balls_count = min_balls_count;
      },
      None => {},
    }

    match settings.max_balls_count {
      Some(max_balls_count) => house.max_balls_count = max_balls_count,
      None => {},
    }

    Ok(())
  }

  // FEE AND HOUSE PDA INSTRUCTIONS

  pub fn reset_daily_stats(
    ctx: Context<ResetDailyStats>,
    _fee_bump: u8,
    _house_bump: u8,
  ) -> Result<()> {
    let fee = &mut ctx.accounts.fee;
    let house = &mut ctx.accounts.house;

    fee.referral_today_volume = 0;
    house.today_volume = 0;

    Ok(())
  }

  pub fn reset_token_daily_stats(
    ctx: Context<ResetTokenDailyStats>,
    _fee_bump: u8,
    _house_bump: u8,
  ) -> Result<()> {
    let fee = &mut ctx.accounts.fee;
    let house = &mut ctx.accounts.house;

    fee.referral_today_volume = 0;
    house.today_volume = 0;

    Ok(())
  }

  // DAO PDA INSTRUCTIONS

  pub fn create_dao(
    ctx: Context<CreateDao>,
    bump: u8,
    referral_bump: u8,
  ) -> Result<()> {
    let authority = &mut ctx.accounts.authority;
    let owner = &mut ctx.accounts.owner;
    let dao = &mut ctx.accounts.dao;
    let referral = &mut ctx.accounts.referral;
    let clock = Clock::get().unwrap();

    dao.bump = bump;
    dao.timestamp = clock.unix_timestamp;
    dao.owner = *owner.to_account_info().key;
    dao.authority = *authority.to_account_info().key;
    dao.verified = false;

    referral.bump = referral_bump;
    referral.verified = false;
    referral.percentage = 40;
    referral.owner = *dao.to_account_info().key;
    referral.authority = *authority.to_account_info().key;

    Ok(())
  }

  pub fn verify_dao(
    ctx: Context<DaoVerification>,
    _bump: u8,
    _referral_bump: u8,
  ) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let referral = &mut ctx.accounts.referral;
    let clock = Clock::get().unwrap();

    dao.verified = true;
    dao.timestamp = clock.unix_timestamp;

    referral.verified = true;

    Ok(())
  }

  pub fn unverify_dao(
    ctx: Context<DaoVerification>,
    _bump: u8,
    _referral_bump: u8,
  ) -> Result<()> {
    let dao = &mut ctx.accounts.dao;
    let referral = &mut ctx.accounts.referral;

    dao.verified = false;

    referral.verified = false;

    Ok(())
  }

  pub fn claim_dao_referral(
    ctx: Context<ClaimDaoReferral>,
    _bump: u8,
    _referral_bump: u8,
  ) -> Result<()> {
    let referral = &mut ctx.accounts.referral;
    let owner = &mut ctx.accounts.owner;
    let dao = &mut ctx.accounts.dao;
    let rent = &mut ctx.accounts.rent;

    let claim_lamports = referral.to_account_info().lamports();
    let rent_lamports = rent.minimum_balance(Referral::LEN);

    require!(dao.verified, CustomErrors::DaoNotVerified);
    require!(referral.verified, CustomErrors::ReferralNotVerified);
    require!(claim_lamports > 0, CustomErrors::ReferralAmountIsZero);

    return transfer_invoke_signed(
      &mut referral.to_account_info(),
      &mut owner.to_account_info(),
      claim_lamports - rent_lamports,
    ).map_err(Into::into);
  }

  pub fn claim_dao_token_referral(
    ctx: Context<ClaimDaoTokenReferral>,
    _bump: u8,
    _referral_bump: u8,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let referral_ata = &mut ctx.accounts.referral_ata;
    let referral = &mut ctx.accounts.referral;
    let owner_ata = &mut ctx.accounts.owner_ata;
    let authority = &mut ctx.accounts.authority;
    let dao = &mut ctx.accounts.dao;

    require!(dao.verified, CustomErrors::DaoNotVerified);
    require!(referral.verified, CustomErrors::ReferralNotVerified);
    require!(referral_ata.amount > 0, CustomErrors::ReferralAmountIsZero);

    return transfer_spl_invoke_signed(
      &referral_ata.to_account_info(),
      &owner_ata.to_account_info(),
      referral_ata.amount,
      &token_program.to_account_info(),
      &referral.to_account_info(),
      &[&[
        b"referral".as_ref(),
        dao.key().as_ref(),
        authority.key().as_ref(),
        &[referral.bump],
      ]],
    );
  }

  pub fn change_percentage_of_dao_referral(
    ctx: Context<ChangePercentageOfDaoReferral>,
    _bump: u8,
    _referral_bump: u8,
    percentage: u8,
  ) -> Result<()> {
    let referral = &mut ctx.accounts.referral;

    require!(percentage <= 100, CustomErrors::InvalidPercentage);

    referral.percentage = percentage;

    Ok(())
  }

  // PROFILE PDA INSTRUCTIONS

  pub fn register(
    ctx: Context<Register>,
    bump: u8,
    referral_bump: u8,
  ) -> Result<()> {
    let authority = &mut ctx.accounts.authority;
    let player = &mut ctx.accounts.player;
    let profile = &mut ctx.accounts.profile;
    let referral = &mut ctx.accounts.referral;
    let clock = Clock::get().unwrap();

    profile.bump = bump;
    profile.timestamp = clock.unix_timestamp;
    profile.player = *player.to_account_info().key;
    profile.authority = *authority.to_account_info().key;
    profile.dao = None;
    profile.invitation = None;
    profile.next_game_id = 1;

    referral.bump = referral_bump;
    referral.verified = true;
    referral.percentage = 20;
    referral.owner = *profile.to_account_info().key;
    referral.authority = *authority.to_account_info().key;

    Ok(())
  }

  pub fn register_with_referral(
    ctx: Context<RegisterWithReferral>,
    bump: u8,
    referral_bump: u8,
    _invitation_bump: u8,
  ) -> Result<()> {
    let authority = &mut ctx.accounts.authority;
    let player = &mut ctx.accounts.player;
    let profile = &mut ctx.accounts.profile;
    let referral = &mut ctx.accounts.referral;
    let invitation = &mut ctx.accounts.invitation;
    let clock = Clock::get().unwrap();

    require!(invitation.verified, CustomErrors::ReferralNotVerified);
    require!(invitation.owner != profile.key(), CustomErrors::InviteOwnerAndRegistererAreSame);

    profile.bump = bump;
    profile.timestamp = clock.unix_timestamp;
    profile.player = *player.to_account_info().key;
    profile.authority = *authority.to_account_info().key;
    profile.invitation = Some(*invitation.to_account_info().key);
    profile.dao = None;
    profile.next_game_id = 1;

    referral.bump = referral_bump;
    referral.verified = true;
    referral.percentage = 20;
    referral.owner = *profile.to_account_info().key;
    referral.authority = *authority.to_account_info().key;

    Ok(())
  }

  pub fn set_profile_dao(
    ctx: Context<SetProfileDao>,
    _bump: u8,
    _dao_bump: u8
  ) -> Result<()> {
    let profile = &mut ctx.accounts.profile;
    let dao = &mut ctx.accounts.dao;

    require!(profile.dao != Some(dao.key()), CustomErrors::DaoAreSame);
    require!(dao.verified, CustomErrors::DaoNotVerified);

    profile.dao = Some(*dao.to_account_info().key);

    Ok(())
  }

  pub fn remove_profile_dao(
    ctx: Context<RemoveProfileDao>,
    _bump: u8,
  ) -> Result<()> {
    let profile = &mut ctx.accounts.profile;

    require!(profile.dao != None, CustomErrors::DaoAlreadyRemoved);

    profile.dao = None;

    Ok(())
  }

  pub fn claim_profile_referral(
    ctx: Context<ClaimProfileReferral>,
    _bump: u8,
    _referral_bump: u8,
  ) -> Result<()> {
    let referral = &mut ctx.accounts.referral;
    let player = &mut ctx.accounts.player;
    let rent = &mut ctx.accounts.rent;

    let claim_lamports = referral.to_account_info().lamports();
    let rent_lamports = rent.minimum_balance(Referral::LEN);

    require!(referral.verified, CustomErrors::ReferralNotVerified);
    require!(claim_lamports > 0, CustomErrors::ReferralAmountIsZero);

    return transfer_invoke_signed(
      &mut referral.to_account_info(),
      &mut player.to_account_info(),
      claim_lamports - rent_lamports,
    ).map_err(Into::into);
  }

  pub fn claim_profile_token_referral(
    ctx: Context<ClaimProfileTokenReferral>,
    _bump: u8,
    _referral_bump: u8,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let referral_ata = &mut ctx.accounts.referral_ata;
    let referral = &mut ctx.accounts.referral;
    let player_ata = &mut ctx.accounts.player_ata;
    let authority = &mut ctx.accounts.authority;
    let profile = &mut ctx.accounts.profile;

    require!(referral.verified, CustomErrors::ReferralNotVerified);
    require!(referral_ata.amount > 0, CustomErrors::ReferralAmountIsZero);

    return transfer_spl_invoke_signed(
      &referral_ata.to_account_info(),
      &player_ata.to_account_info(),
      referral_ata.amount,
      &token_program.to_account_info(),
      &referral.to_account_info(),
      &[&[
        b"referral".as_ref(),
        profile.key().as_ref(),
        authority.key().as_ref(),
        &[referral.bump],
      ]],
    );
  }

  pub fn unverify_profile_referral(
    ctx: Context<UnverifyProfileReferral>,
    _bump: u8,
    _referral_bump: u8,
  ) -> Result<()> {
    let referral = &mut ctx.accounts.referral;

    referral.verified = false;

    Ok(())
  }

  pub fn change_percentage_of_profile_referral(
    ctx: Context<ChangePercentageOfProfileReferral>,
    _bump: u8,
    _referral_bump: u8,
    percentage: u8,
  ) -> Result<()> {
    let referral = &mut ctx.accounts.referral;

    require!(percentage <= 100, CustomErrors::InvalidPercentage);

    referral.percentage = percentage;

    Ok(())
  }

  // PLINKO PDA INSTRUCTIONS

  pub fn init_plinko(
    ctx: Context<InitPlinko>,
    bump: u8,
    _house_bump: u8,
    _profile_bump: u8,
    settings: InitPlinkoSettings,
  ) -> Result<()> {
    let authority = &mut ctx.accounts.authority;
    let house = &mut ctx.accounts.house;
    let plinko = &mut ctx.accounts.plinko;
    let player = &mut ctx.accounts.player;
    let profile = &mut ctx.accounts.profile;
    let rent = &mut ctx.accounts.rent;
    let clock = Clock::get().unwrap();

    let house_rent_lamports = rent.minimum_balance(House::LEN);
    let house_lamports = house.to_account_info().lamports() - house_rent_lamports;
    let bet_lamports = settings.bet_amount * u64::from(settings.balls_count);
    let fee_lamports = (bet_lamports * 35) / 1000;
    let transaction_fee: u64 = 10000;

    if settings.degen_mode {
      require!(
        settings.row.is_none() &&
        settings.risk.is_none(),
        CustomErrors::InvalidInitPlinkoSettings,
      );
    } else {
      require!(
        settings.row.is_some() &&
        settings.risk.is_some(),
        CustomErrors::InvalidInitPlinkoSettings,
      );
    }
    require!(!house.paused, CustomErrors::GameIsPaused);
    require!(
      settings.bet_amount >= house.min_bet &&
      settings.bet_amount <= house.max_bet,
      CustomErrors::InvalidBetAmount,
    );
    require!(
      settings.balls_count >= house.min_balls_count &&
      settings.balls_count <= house.max_balls_count,
      CustomErrors::InvalidBallsCount,
    );
    require!(
      house_lamports > bet_lamports,
      CustomErrors::InsufficientFundsInHouse,
    );

    match settings.row {
      Some(row) => {
        require!(row >= 8 && row <= 16, CustomErrors::InvalidRows);
        plinko.row = settings.row;
      },
      None => plinko.row = None,
    }
    match settings.risk {
      Some(risk) => {
        require!(risk == 0 || risk == 1 || risk == 2, CustomErrors::InvalidRisk);
        plinko.risk = settings.risk;
      },
      None => plinko.risk = None,
    }

    plinko.bump = bump;
    plinko.id = profile.next_game_id;
    plinko.degen_mode = settings.degen_mode;
    plinko.balls_count = settings.balls_count;
    plinko.bet_amount = settings.bet_amount;
    plinko.payout = 0;
    plinko.timestamp = clock.unix_timestamp;
    plinko.revealed = false;
    plinko.paid_out = false;
    plinko.dao = None;
    plinko.referral = None;
    plinko.authority = *authority.to_account_info().key;
    plinko.player = *player.to_account_info().key;

    profile.next_game_id += 1;

    let bet_result = transfer_invoke(
      &player.to_account_info(),
      &plinko.to_account_info(),
      bet_lamports + fee_lamports,
    );
    if bet_result.is_err() {
      return bet_result.map_err(Into::into);
    }

    return transfer_invoke(
      &player.to_account_info(),
      &authority.to_account_info(),
      transaction_fee,
    );
  }

  pub fn init_token_plinko(
    ctx: Context<InitTokenPlinko>,
    bump: u8,
    _house_bump: u8,
    _profile_bump: u8,
    settings: InitPlinkoSettings,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let authority = &mut ctx.accounts.authority;
    let house_ata = &mut ctx.accounts.house_ata;
    let house = &mut ctx.accounts.house;
    let plinko_ata = &mut ctx.accounts.plinko_ata;
    let plinko = &mut ctx.accounts.plinko;
    let player_ata = &mut ctx.accounts.player_ata;
    let player = &mut ctx.accounts.player;
    let profile = &mut ctx.accounts.profile;
    let clock = Clock::get().unwrap();

    let house_lamports = house_ata.amount;
    let bet_lamports = settings.bet_amount * u64::from(settings.balls_count);
    let fee_lamports = (bet_lamports * 35) / 1000;
    let transaction_fee: u64 = 10000;

    if settings.degen_mode {
      require!(
        settings.row.is_none() &&
        settings.risk.is_none(),
        CustomErrors::InvalidInitPlinkoSettings,
      );
    } else {
      require!(
        settings.row.is_some() &&
        settings.risk.is_some(),
        CustomErrors::InvalidInitPlinkoSettings,
      );
    }
    require!(!house.paused, CustomErrors::GameIsPaused);
    require!(
      settings.bet_amount >= house.min_bet &&
      settings.bet_amount <= house.max_bet,
      CustomErrors::InvalidBetAmount,
    );
    require!(
      settings.balls_count >= house.min_balls_count &&
      settings.balls_count <= house.max_balls_count,
      CustomErrors::InvalidBallsCount,
    );
    require!(
      house_lamports > bet_lamports,
      CustomErrors::InsufficientFundsInHouse,
    );

    match settings.row {
      Some(row) => {
        require!(row >= 8 && row <= 16, CustomErrors::InvalidRows);
        plinko.row = settings.row;
      },
      None => plinko.row = None,
    }
    match settings.risk {
      Some(risk) => {
        require!(risk == 0 || risk == 1 || risk == 2, CustomErrors::InvalidRisk);
        plinko.risk = settings.risk;
      },
      None => plinko.risk = None,
    }

    plinko.bump = bump;
    plinko.id = profile.next_game_id;
    plinko.degen_mode = settings.degen_mode;
    plinko.balls_count = settings.balls_count;
    plinko.bet_amount = settings.bet_amount;
    plinko.payout = 0;
    plinko.timestamp = clock.unix_timestamp;
    plinko.revealed = false;
    plinko.paid_out = false;
    plinko.dao = None;
    plinko.referral = None;
    plinko.authority = *authority.to_account_info().key;
    plinko.player = *player.to_account_info().key;
    profile.next_game_id += 1;

    let bet_result = transfer_spl_invoke(
      &player_ata.to_account_info(),
      &plinko_ata.to_account_info(),
      bet_lamports + fee_lamports,
      &token_program.to_account_info(),
      &player.to_account_info(),
    );
    if bet_result.is_err() {
      return bet_result.map_err(Into::into);
    }

    return transfer_invoke(
      &player.to_account_info(),
      &authority.to_account_info(),
      transaction_fee,
    );
  }

  pub fn reveal_plinko(
    ctx: Context<RevealPlinko>,
    _bump: u8,
    _fee_bump: u8,
    _house_bump: u8,
    _profile_bump: u8,
    _id: u64,
    payout: u64,
  ) -> Result<()> {
    let plinko = &mut ctx.accounts.plinko;
    let house = &mut ctx.accounts.house;
    let fee = &mut ctx.accounts.fee;
    let authority = &mut ctx.accounts.authority;
    let profile = &mut ctx.accounts.profile;
    let rent = &mut ctx.accounts.rent;

    require!(!plinko.revealed, CustomErrors::GameIsRevealed);
    require!(profile.invitation.is_none(), CustomErrors::ProfileHasInvite);

    let house_rent = rent.minimum_balance(House::LEN);
    let house_lamports = house.to_account_info().lamports() - house_rent;
    let bet_lamports = plinko.bet_amount * u64::from(plinko.balls_count);
    let holders_fee_lamports = (bet_lamports * 30) / 1000;
    let team_fee_lamports = (bet_lamports * 5) / 1000;
    let max_win_lamports: u64 = (house_lamports * 3) / 10;
    let computed_payout: u64 = payout.min(max_win_lamports);

    plinko.payout = computed_payout;
    plinko.dao = profile.dao;
    plinko.revealed = true;

    house.volume += bet_lamports;
    house.today_volume += bet_lamports;
    fee.volume += holders_fee_lamports;

    if bet_lamports > computed_payout {
      let payout_result = transfer_invoke_signed(
        &mut plinko.to_account_info(),
        &mut house.to_account_info(),
        bet_lamports - computed_payout,
      );

      if payout_result.is_err() {
        return payout_result.map_err(Into::into);
      }
    } else if bet_lamports < computed_payout {
      let payout_result = transfer_invoke_signed(
        &mut house.to_account_info(),
        &mut plinko.to_account_info(),
        computed_payout - bet_lamports,
      );

      if payout_result.is_err() {
        return payout_result.map_err(Into::into);
      }
    }

    let fee_result = transfer_invoke_signed(
      &mut plinko.to_account_info(),
      &mut fee.to_account_info(),
      holders_fee_lamports,
    );
    if fee_result.is_err() {
      return fee_result.map_err(Into::into);
    }

    return transfer_invoke_signed(
      &mut plinko.to_account_info(),
      &mut authority.to_account_info(),
      team_fee_lamports,
    ).map_err(Into::into);
  }

  pub fn reveal_token_plinko(
    ctx: Context<RevealTokenPlinko>,
    _bump: u8,
    _fee_bump: u8,
    _house_bump: u8,
    _profile_bump: u8,
    id: u64,
    payout: u64,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let plinko = &mut ctx.accounts.plinko;
    let plinko_ata = &mut ctx.accounts.plinko_ata;
    let house = &mut ctx.accounts.house;
    let house_ata = &mut ctx.accounts.house_ata;
    let fee = &mut ctx.accounts.fee;
    let fee_ata = &mut ctx.accounts.fee_ata;
    let authority = &mut ctx.accounts.authority;
    let authority_ata = &mut ctx.accounts.authority_ata;
    let profile = &mut ctx.accounts.profile;
    let player = &mut ctx.accounts.player;
    let mint = &mut ctx.accounts.mint;

    require!(!plinko.revealed, CustomErrors::GameIsRevealed);
    require!(profile.invitation.is_none(), CustomErrors::ProfileHasInvite);

    let bet_lamports = plinko.bet_amount * u64::from(plinko.balls_count);
    let holders_fee_lamports = (bet_lamports * 30) / 1000;
    let team_fee_lamports = (bet_lamports * 5) / 1000;
    let max_win_lamports: u64 = (house_ata.amount * 3) / 10;
    let computed_payout: u64 = payout.min(max_win_lamports);

    plinko.payout = computed_payout;
    plinko.dao = profile.dao;
    plinko.revealed = true;

    house.volume += bet_lamports;
    house.today_volume += bet_lamports;
    fee.volume += holders_fee_lamports;

    if bet_lamports > computed_payout {
      let payout_result = transfer_spl_invoke_signed(
        &plinko_ata.to_account_info(),
        &house_ata.to_account_info(),
        bet_lamports - computed_payout,
        &token_program.to_account_info(),
        &plinko.to_account_info(),
        &[&[
          b"plinko".as_ref(),
          player.key().as_ref(),
          mint.key().as_ref(),
          authority.key().as_ref(),
          &id.to_le_bytes(),
          &[plinko.bump],
        ]],
      );

      if payout_result.is_err() {
        return payout_result.map_err(Into::into);
      }
    } else if bet_lamports < computed_payout {
      let payout_result = transfer_spl_invoke_signed(
        &house_ata.to_account_info(),
        &plinko_ata.to_account_info(),
        computed_payout - bet_lamports,
        &token_program.to_account_info(),
        &house.to_account_info(),
        &[&[
          b"house".as_ref(),
          mint.key().as_ref(),
          authority.key().as_ref(),
          &[house.bump],
        ]],
      );

      if payout_result.is_err() {
        return payout_result.map_err(Into::into);
      }
    }

    let fee_result = transfer_spl_invoke_signed(
      &plinko_ata.to_account_info(),
      &fee_ata.to_account_info(),
      holders_fee_lamports,
      &token_program.to_account_info(),
      &plinko.to_account_info(),
      &[&[
        b"plinko".as_ref(),
        player.key().as_ref(),
        mint.key().as_ref(),
        authority.key().as_ref(),
        &id.to_le_bytes(),
        &[plinko.bump],
      ]],
    );
    if fee_result.is_err() {
      return fee_result.map_err(Into::into);
    }

    return transfer_spl_invoke_signed(
      &plinko_ata.to_account_info(),
      &authority_ata.to_account_info(),
      team_fee_lamports,
      &token_program.to_account_info(),
      &plinko.to_account_info(),
      &[&[
        b"plinko".as_ref(),
        player.key().as_ref(),
        mint.key().as_ref(),
        authority.key().as_ref(),
        &id.to_le_bytes(),
        &[plinko.bump],
      ]],
    );
  }

  pub fn reveal_plinko_with_referral(
    ctx: Context<RevealPlinkoWithReferral>,
    _bump: u8,
    _fee_bump: u8,
    _house_bump: u8,
    _profile_bump: u8,
    _referral_bump: u8,
    _id: u64,
    payout: u64,
  ) -> Result<()> {
    let plinko = &mut ctx.accounts.plinko;
    let house = &mut ctx.accounts.house;
    let fee = &mut ctx.accounts.fee;
    let authority = &mut ctx.accounts.authority;
    let profile = &mut ctx.accounts.profile;
    let referral = &mut ctx.accounts.referral;
    let rent = &mut ctx.accounts.rent;

    require!(!plinko.revealed, CustomErrors::GameIsRevealed);
    require!(profile.invitation.is_some(), CustomErrors::ProfileHasNotInvite);

    let house_rent = rent.minimum_balance(House::LEN);
    let house_lamports = house.to_account_info().lamports() - house_rent;
    let bet_lamports = plinko.bet_amount * u64::from(plinko.balls_count);
    let fee_lamports = (bet_lamports * 35) / 1000;
    let referral_lamports: u64;
    let sae_lamports: u64;

    if referral.verified {
      plinko.referral = profile.invitation;
      sae_lamports = (fee_lamports * (100 - u64::from(referral.percentage))) / 100;
      referral_lamports = (fee_lamports * u64::from(referral.percentage)) / 100;
    } else {
      sae_lamports = fee_lamports;
      referral_lamports = 0;
    }

    let holders_fee_lamports = (sae_lamports * 30) / 35;
    let team_fee_lamports = (sae_lamports * 5) / 35;
    let max_win_lamports: u64 = (house_lamports * 3) / 10;
    let computed_payout: u64 = payout.min(max_win_lamports);

    plinko.payout = computed_payout;
    plinko.dao = profile.dao;
    plinko.revealed = true;

    house.volume += bet_lamports;
    house.today_volume += bet_lamports;
    fee.volume += holders_fee_lamports;
    fee.referral_today_volume += referral_lamports;
    fee.referral_volume += referral_lamports;

    if bet_lamports > computed_payout {
      let payout_result = transfer_invoke_signed(
        &mut plinko.to_account_info(),
        &mut house.to_account_info(),
        bet_lamports - computed_payout,
      );

      if payout_result.is_err() {
        return payout_result.map_err(Into::into);
      }
    } else if bet_lamports < computed_payout {
      let payout_result = transfer_invoke_signed(
        &mut house.to_account_info(),
        &mut plinko.to_account_info(),
        computed_payout - bet_lamports,
      );

      if payout_result.is_err() {
        return payout_result.map_err(Into::into);
      }
    }

    let fee_result = transfer_invoke_signed(
      &mut plinko.to_account_info(),
      &mut fee.to_account_info(),
      holders_fee_lamports,
    );
    if fee_result.is_err() {
      return fee_result.map_err(Into::into);
    }

    if referral_lamports > 0 {
      let referral_result = transfer_invoke_signed(
        &mut plinko.to_account_info(),
        &mut referral.to_account_info(),
        referral_lamports,
      );
      if referral_result.is_err() {
        return referral_result.map_err(Into::into);
      }
    }

    return transfer_invoke_signed(
      &mut plinko.to_account_info(),
      &mut authority.to_account_info(),
      team_fee_lamports,
    ).map_err(Into::into);
  }

  pub fn reveal_token_plinko_with_referral(
    ctx: Context<RevealTokenPlinkoWithReferral>,
    _bump: u8,
    _fee_bump: u8,
    _house_bump: u8,
    _profile_bump: u8,
    _referral_bump: u8,
    id: u64,
    payout: u64,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let plinko = &mut ctx.accounts.plinko;
    let plinko_ata = &mut ctx.accounts.plinko_ata;
    let house = &mut ctx.accounts.house;
    let house_ata = &mut ctx.accounts.house_ata;
    let fee = &mut ctx.accounts.fee;
    let fee_ata = &mut ctx.accounts.fee_ata;
    let authority = &mut ctx.accounts.authority;
    let authority_ata = &mut ctx.accounts.authority_ata;
    let profile = &mut ctx.accounts.profile;
    let player = &mut ctx.accounts.player;
    let mint = &mut ctx.accounts.mint;
    let referral = &mut ctx.accounts.referral;
    let referral_ata = &mut ctx.accounts.referral_ata;

    require!(!plinko.revealed, CustomErrors::GameIsRevealed);
    require!(profile.invitation.is_some(), CustomErrors::ProfileHasNotInvite);

    let bet_lamports = plinko.bet_amount * u64::from(plinko.balls_count);
    let fee_lamports = (bet_lamports * 35) / 1000;
    let referral_lamports: u64;
    let sae_lamports: u64;

    if referral.verified {
      plinko.referral = profile.invitation;
      sae_lamports = (fee_lamports * (100 - u64::from(referral.percentage))) / 100;
      referral_lamports = (fee_lamports * u64::from(referral.percentage)) / 100;
    } else {
      referral_lamports = 0;
      sae_lamports = fee_lamports;
    }

    let holders_fee_lamports = (sae_lamports * 30) / 35;
    let team_fee_lamports = (sae_lamports * 5) / 35;
    let max_win_lamports: u64 = (house_ata.amount * 3) / 10;
    let computed_payout: u64 = payout.min(max_win_lamports);

    plinko.payout = computed_payout;
    plinko.dao = profile.dao;
    plinko.revealed = true;

    house.volume += bet_lamports;
    house.today_volume += bet_lamports;
    fee.volume += holders_fee_lamports;
    fee.referral_today_volume += referral_lamports;
    fee.referral_volume += referral_lamports;

    if bet_lamports > computed_payout {
      let payout_result = transfer_spl_invoke_signed(
        &plinko_ata.to_account_info(),
        &house_ata.to_account_info(),
        bet_lamports - computed_payout,
        &token_program.to_account_info(),
        &plinko.to_account_info(),
        &[&[
          b"plinko".as_ref(),
          player.key().as_ref(),
          mint.key().as_ref(),
          authority.key().as_ref(),
          &id.to_le_bytes(),
          &[plinko.bump],
        ]],
      );

      if payout_result.is_err() {
        return payout_result.map_err(Into::into);
      }
    } else if bet_lamports < computed_payout {
      let payout_result = transfer_spl_invoke_signed(
        &house_ata.to_account_info(),
        &plinko_ata.to_account_info(),
        computed_payout - bet_lamports,
        &token_program.to_account_info(),
        &house.to_account_info(),
        &[&[
          b"house".as_ref(),
          mint.key().as_ref(),
          authority.key().as_ref(),
          &[house.bump],
        ]],
      );

      if payout_result.is_err() {
        return payout_result.map_err(Into::into);
      }
    }

    let fee_result = transfer_spl_invoke_signed(
      &plinko_ata.to_account_info(),
      &fee_ata.to_account_info(),
      holders_fee_lamports,
      &token_program.to_account_info(),
      &plinko.to_account_info(),
      &[&[
        b"plinko".as_ref(),
        player.key().as_ref(),
        mint.key().as_ref(),
        authority.key().as_ref(),
        &id.to_le_bytes(),
        &[plinko.bump],
      ]],
    );
    if fee_result.is_err() {
      return fee_result.map_err(Into::into);
    }

    if referral_lamports > 0 {
      let referral_result = transfer_spl_invoke_signed(
        &plinko_ata.to_account_info(),
        &referral_ata.to_account_info(),
        referral_lamports,
        &token_program.to_account_info(),
        &plinko.to_account_info(),
        &[&[
          b"plinko".as_ref(),
          player.key().as_ref(),
          mint.key().as_ref(),
          authority.key().as_ref(),
          &id.to_le_bytes(),
          &[plinko.bump],
        ]],
      );
      if referral_result.is_err() {
        return referral_result.map_err(Into::into);
      }
    }

    return transfer_spl_invoke_signed(
      &plinko_ata.to_account_info(),
      &authority_ata.to_account_info(),
      team_fee_lamports,
      &token_program.to_account_info(),
      &plinko.to_account_info(),
      &[&[
        b"plinko".as_ref(),
        player.key().as_ref(),
        mint.key().as_ref(),
        authority.key().as_ref(),
        &id.to_le_bytes(),
        &[plinko.bump],
      ]],
    );
  }

  pub fn claim_plinko(
    ctx: Context<ClaimPlinko>,
    _bump: u8,
    _id: u64,
  ) -> Result<()> {
    let plinko = &mut ctx.accounts.plinko;
    let player = &mut ctx.accounts.player;
    let rent = &mut ctx.accounts.rent;

    require!(plinko.revealed, CustomErrors::GameIsNotRevealed);
    require!(!plinko.paid_out, CustomErrors::GameIsClaimed);

    let rent_lamports = rent.minimum_balance(Plinko::LEN);
    let plinko_lamports = plinko.to_account_info().lamports();

    plinko.paid_out = true;

    return transfer_invoke_signed(
      &mut plinko.to_account_info(),
      &mut player.to_account_info(),
      plinko_lamports - rent_lamports,
    ).map_err(Into::into);
  }

  pub fn claim_token_plinko(
    ctx: Context<ClaimTokenPlinko>,
    _bump: u8,
    id: u64,
  ) -> Result<()> {
    let token_program = &mut ctx.accounts.token_program;
    let plinko_ata = &mut ctx.accounts.plinko_ata;
    let plinko = &mut ctx.accounts.plinko;
    let player_ata = &mut ctx.accounts.player_ata;
    let player = &mut ctx.accounts.player;
    let authority = &mut ctx.accounts.authority;
    let mint = &mut ctx.accounts.mint;

    require!(plinko.revealed, CustomErrors::GameIsNotRevealed);
    require!(!plinko.paid_out, CustomErrors::GameIsClaimed);

    let payout_result = transfer_spl_invoke_signed(
      &plinko_ata.to_account_info(),
      &player_ata.to_account_info(),
      plinko_ata.amount,
      &token_program.to_account_info(),
      &plinko.to_account_info(),
      &[&[
        b"plinko".as_ref(),
        player.key().as_ref(),
        mint.key().as_ref(),
        authority.key().as_ref(),
        &id.to_le_bytes(),
        &[plinko.bump],
      ]],
    );
    if payout_result.is_err() {
      return payout_result.map_err(Into::into);
    }

    plinko.paid_out = true;

    return close_account(CpiContext::new_with_signer(
      token_program.to_account_info(),
      CloseAccount {
        account: plinko_ata.to_account_info(),
        destination: player.to_account_info(),
        authority: plinko.to_account_info(),
      },
      &[&[
        b"plinko".as_ref(),
        player.key().as_ref(),
        mint.key().as_ref(),
        authority.key().as_ref(),
        &id.to_le_bytes(),
        &[plinko.bump],
      ]],
    ));
  }
}
