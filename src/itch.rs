use std::fmt;
use std::io::{Cursor, Read, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub fn u48_to_u64(num: &[u8;6]) -> u64 {
  ((num[0] as u64) << 40) +
  ((num[1] as u64) << 32) +
  ((num[2] as u64) << 24) +
  ((num[3] as u64) << 16) +
  ((num[4] as u64) << 8) +
  ((num[5] as u64) << 0)
}

// Enums
#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eSystemEvent(pub u8);
#[allow(non_upper_case_globals)]
impl eSystemEvent {
  pub const Start_of_Messages : u8 = 'O' as u8;
  pub const Start_of_System_hours : u8 = 'S' as u8;
  pub const Start_of_Market_hours : u8 = 'Q' as u8;
  pub const End_of_Market_hours : u8 = 'M' as u8;
  pub const End_of_System_hours : u8 = 'E' as u8;
  pub const End_of_Messages : u8 = 'C' as u8;
} // eSystemEvent

impl fmt::Display for eSystemEvent {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eSystemEvent::Start_of_Messages => "'O' (Start of Messages)".to_string(),
      eSystemEvent::Start_of_System_hours => "'S' (Start of System hours)".to_string(),
      eSystemEvent::Start_of_Market_hours => "'Q' (Start of Market hours)".to_string(),
      eSystemEvent::End_of_Market_hours => "'M' (End of Market hours)".to_string(),
      eSystemEvent::End_of_System_hours => "'E' (End of System hours)".to_string(),
      eSystemEvent::End_of_Messages => "'C' (End of Messages)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eMarketCategory(pub u8);
#[allow(non_upper_case_globals)]
impl eMarketCategory {
  pub const Nasdaq_Global_Select_MarketSM : u8 = 'Q' as u8;
  pub const Nasdaq_Global_MarketSM : u8 = 'G' as u8;
  pub const Nasdaq_Capital_Market : u8 = 'S' as u8;
  pub const New_York_Stock_Exchange : u8 = 'N' as u8;
  pub const NYSE_MKT : u8 = 'A' as u8;
  pub const NYSE_Arca : u8 = 'P' as u8;
  pub const BATS_Z_Exchange : u8 = 'Z' as u8;
  pub const Investors_Exchange_LLC : u8 = 'V' as u8;
  pub const Not_available : u8 = ' ' as u8;
} // eMarketCategory

impl fmt::Display for eMarketCategory {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eMarketCategory::Nasdaq_Global_Select_MarketSM => "'Q' (Nasdaq Global Select MarketSM)".to_string(),
      eMarketCategory::Nasdaq_Global_MarketSM => "'G' (Nasdaq Global MarketSM)".to_string(),
      eMarketCategory::Nasdaq_Capital_Market => "'S' (Nasdaq Capital Market)".to_string(),
      eMarketCategory::New_York_Stock_Exchange => "'N' (New York Stock Exchange)".to_string(),
      eMarketCategory::NYSE_MKT => "'A' (NYSE MKT)".to_string(),
      eMarketCategory::NYSE_Arca => "'P' (NYSE Arca)".to_string(),
      eMarketCategory::BATS_Z_Exchange => "'Z' (BATS Z Exchange)".to_string(),
      eMarketCategory::Investors_Exchange_LLC => "'V' (Investors Exchange LLC)".to_string(),
      eMarketCategory::Not_available => "' ' (Not available)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eFinancialStatusIndicator(pub u8);
#[allow(non_upper_case_globals)]
impl eFinancialStatusIndicator {
  pub const Deficient : u8 = 'D' as u8;
  pub const Delinquent : u8 = 'E' as u8;
  pub const Bankrupt : u8 = 'Q' as u8;
  pub const Suspended : u8 = 'S' as u8;
  pub const Deficient_and_Bankrupt : u8 = 'G' as u8;
  pub const Deficient_and_Delinquent : u8 = 'H' as u8;
  pub const Delinquent_and_Bankrupt : u8 = 'J' as u8;
  pub const Deficient_Delinquent_and_Bankrupt : u8 = 'K' as u8;
  pub const Creations_and_or_Redemptions_Suspended : u8 = 'C' as u8;
  pub const Normal : u8 = 'N' as u8;
  pub const Not_available : u8 = ' ' as u8;
} // eFinancialStatusIndicator

impl fmt::Display for eFinancialStatusIndicator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eFinancialStatusIndicator::Deficient => "'D' (Deficient)".to_string(),
      eFinancialStatusIndicator::Delinquent => "'E' (Delinquent)".to_string(),
      eFinancialStatusIndicator::Bankrupt => "'Q' (Bankrupt)".to_string(),
      eFinancialStatusIndicator::Suspended => "'S' (Suspended)".to_string(),
      eFinancialStatusIndicator::Deficient_and_Bankrupt => "'G' (Deficient and Bankrupt)".to_string(),
      eFinancialStatusIndicator::Deficient_and_Delinquent => "'H' (Deficient and Delinquent)".to_string(),
      eFinancialStatusIndicator::Delinquent_and_Bankrupt => "'J' (Delinquent and Bankrupt)".to_string(),
      eFinancialStatusIndicator::Deficient_Delinquent_and_Bankrupt => "'K' (Deficient Delinquent and Bankrupt)".to_string(),
      eFinancialStatusIndicator::Creations_and_or_Redemptions_Suspended => "'C' (Creations and or Redemptions Suspended)".to_string(),
      eFinancialStatusIndicator::Normal => "'N' (Normal)".to_string(),
      eFinancialStatusIndicator::Not_available => "' ' (Not available)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eRoundLotsOnly(pub u8);
#[allow(non_upper_case_globals)]
impl eRoundLotsOnly {
  pub const Round_Lots_Only : u8 = 'Y' as u8;
  pub const Accepts_Round_Lots : u8 = 'N' as u8;
} // eRoundLotsOnly

impl fmt::Display for eRoundLotsOnly {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eRoundLotsOnly::Round_Lots_Only => "'Y' (Round Lots Only)".to_string(),
      eRoundLotsOnly::Accepts_Round_Lots => "'N' (Accepts Round Lots)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eAuthenticity(pub u8);
#[allow(non_upper_case_globals)]
impl eAuthenticity {
  pub const Production : u8 = 'P' as u8;
  pub const Test : u8 = 'T' as u8;
} // eAuthenticity

impl fmt::Display for eAuthenticity {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eAuthenticity::Production => "'P' (Production)".to_string(),
      eAuthenticity::Test => "'T' (Test)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eShortSaleThresholdIndicator(pub u8);
#[allow(non_upper_case_globals)]
impl eShortSaleThresholdIndicator {
  pub const Restricted : u8 = 'Y' as u8;
  pub const Not_Restricted : u8 = 'N' as u8;
  pub const Not_available : u8 = ' ' as u8;
} // eShortSaleThresholdIndicator

impl fmt::Display for eShortSaleThresholdIndicator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eShortSaleThresholdIndicator::Restricted => "'Y' (Restricted)".to_string(),
      eShortSaleThresholdIndicator::Not_Restricted => "'N' (Not Restricted)".to_string(),
      eShortSaleThresholdIndicator::Not_available => "' ' (Not available)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eIPOFlag(pub u8);
#[allow(non_upper_case_globals)]
impl eIPOFlag {
  pub const New_IPO_Security : u8 = 'Y' as u8;
  pub const Not_A_New_IPO_Security : u8 = 'N' as u8;
  pub const Not_available : u8 = ' ' as u8;
} // eIPOFlag

impl fmt::Display for eIPOFlag {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eIPOFlag::New_IPO_Security => "'Y' (New IPO Security)".to_string(),
      eIPOFlag::Not_A_New_IPO_Security => "'N' (Not A New IPO Security)".to_string(),
      eIPOFlag::Not_available => "' ' (Not available)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eLULDReferencePriceTier(pub u8);
#[allow(non_upper_case_globals)]
impl eLULDReferencePriceTier {
  pub const Tier_1_NMS_Stocks_and_select_ETPs : u8 = '1' as u8;
  pub const Tier_2_NMSStocks : u8 = '2' as u8;
  pub const Not_available : u8 = ' ' as u8;
} // eLULDReferencePriceTier

impl fmt::Display for eLULDReferencePriceTier {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eLULDReferencePriceTier::Tier_1_NMS_Stocks_and_select_ETPs => "'1' (Tier 1 NMS Stocks and select ETPs)".to_string(),
      eLULDReferencePriceTier::Tier_2_NMSStocks => "'2' (Tier 2 NMSStocks)".to_string(),
      eLULDReferencePriceTier::Not_available => "' ' (Not available)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eETPFlag(pub u8);
#[allow(non_upper_case_globals)]
impl eETPFlag {
  pub const Instrument_is_an_ETP : u8 = 'Y' as u8;
  pub const Instrument_is_not_an_ETP : u8 = 'N' as u8;
  pub const Not_available : u8 = ' ' as u8;
} // eETPFlag

impl fmt::Display for eETPFlag {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eETPFlag::Instrument_is_an_ETP => "'Y' (Instrument is an ETP)".to_string(),
      eETPFlag::Instrument_is_not_an_ETP => "'N' (Instrument is not an ETP)".to_string(),
      eETPFlag::Not_available => "' ' (Not available)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eInverseIndicator(pub u8);
#[allow(non_upper_case_globals)]
impl eInverseIndicator {
  pub const ETP_is_an_Inverse_ETP : u8 = 'Y' as u8;
  pub const ETP_is_not_an_Inverse_ETP : u8 = 'N' as u8;
} // eInverseIndicator

impl fmt::Display for eInverseIndicator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eInverseIndicator::ETP_is_an_Inverse_ETP => "'Y' (ETP is an Inverse ETP)".to_string(),
      eInverseIndicator::ETP_is_not_an_Inverse_ETP => "'N' (ETP is not an Inverse ETP)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eTradingState(pub u8);
#[allow(non_upper_case_globals)]
impl eTradingState {
  pub const Halted : u8 = 'H' as u8;
  pub const Paused : u8 = 'P' as u8;
  pub const Quotation_only : u8 = 'Q' as u8;
  pub const Trading : u8 = 'T' as u8;
} // eTradingState

impl fmt::Display for eTradingState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eTradingState::Halted => "'H' (Halted)".to_string(),
      eTradingState::Paused => "'P' (Paused)".to_string(),
      eTradingState::Quotation_only => "'Q' (Quotation only)".to_string(),
      eTradingState::Trading => "'T' (Trading)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eRegSHOAction(pub u8);
#[allow(non_upper_case_globals)]
impl eRegSHOAction {
  pub const No_price_test_in_place : u8 = '0' as u8;
  pub const Reg_SHO_Short_Sale_Price_Test_Restriction_in_effect : u8 = '1' as u8;
  pub const Reg_SHO_Short_Sale_Price_Test_Restriction_remains_in_effect : u8 = '2' as u8;
} // eRegSHOAction

impl fmt::Display for eRegSHOAction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eRegSHOAction::No_price_test_in_place => "'0' (No price test in place)".to_string(),
      eRegSHOAction::Reg_SHO_Short_Sale_Price_Test_Restriction_in_effect => "'1' (Reg SHO Short Sale Price Test Restriction in effect)".to_string(),
      eRegSHOAction::Reg_SHO_Short_Sale_Price_Test_Restriction_remains_in_effect => "'2' (Reg SHO Short Sale Price Test Restriction remains in effect)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct ePrimaryMarketMaker(pub u8);
#[allow(non_upper_case_globals)]
impl ePrimaryMarketMaker {
  pub const primary_market_maker : u8 = 'Y' as u8;
  pub const non_primary_market_maker : u8 = 'N' as u8;
} // ePrimaryMarketMaker

impl fmt::Display for ePrimaryMarketMaker {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      ePrimaryMarketMaker::primary_market_maker => "'Y' (primary market maker)".to_string(),
      ePrimaryMarketMaker::non_primary_market_maker => "'N' (non primary market maker)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eMarketMakerMode(pub u8);
#[allow(non_upper_case_globals)]
impl eMarketMakerMode {
  pub const normal : u8 = 'N' as u8;
  pub const passive : u8 = 'P' as u8;
  pub const syndicate : u8 = 'S' as u8;
  pub const pre_syndicate : u8 = 'R' as u8;
  pub const penalty : u8 = 'L' as u8;
} // eMarketMakerMode

impl fmt::Display for eMarketMakerMode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eMarketMakerMode::normal => "'N' (normal)".to_string(),
      eMarketMakerMode::passive => "'P' (passive)".to_string(),
      eMarketMakerMode::syndicate => "'S' (syndicate)".to_string(),
      eMarketMakerMode::pre_syndicate => "'R' (pre syndicate)".to_string(),
      eMarketMakerMode::penalty => "'L' (penalty)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eMarketParticipantState(pub u8);
#[allow(non_upper_case_globals)]
impl eMarketParticipantState {
  pub const Active : u8 = 'A' as u8;
  pub const Excused_Withdrawn : u8 = 'E' as u8;
  pub const Withdrawn : u8 = 'W' as u8;
  pub const Suspended : u8 = 'S' as u8;
  pub const Deleted : u8 = 'D' as u8;
} // eMarketParticipantState

impl fmt::Display for eMarketParticipantState {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eMarketParticipantState::Active => "'A' (Active)".to_string(),
      eMarketParticipantState::Excused_Withdrawn => "'E' (Excused Withdrawn)".to_string(),
      eMarketParticipantState::Withdrawn => "'W' (Withdrawn)".to_string(),
      eMarketParticipantState::Suspended => "'S' (Suspended)".to_string(),
      eMarketParticipantState::Deleted => "'D' (Deleted)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eBreachedLevel(pub u8);
#[allow(non_upper_case_globals)]
impl eBreachedLevel {
  pub const Level_1 : u8 = '1' as u8;
  pub const Level_2 : u8 = '2' as u8;
  pub const Level_3 : u8 = '3' as u8;
} // eBreachedLevel

impl fmt::Display for eBreachedLevel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eBreachedLevel::Level_1 => "'1' (Level 1)".to_string(),
      eBreachedLevel::Level_2 => "'2' (Level 2)".to_string(),
      eBreachedLevel::Level_3 => "'3' (Level 3)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eIPOQuotationReleaseQualifier(pub u8);
#[allow(non_upper_case_globals)]
impl eIPOQuotationReleaseQualifier {
  pub const Anticipated_Quotation_Release_Time : u8 = 'A' as u8;
  pub const IPO_Release_Canceled_Postponed : u8 = 'C' as u8;
} // eIPOQuotationReleaseQualifier

impl fmt::Display for eIPOQuotationReleaseQualifier {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eIPOQuotationReleaseQualifier::Anticipated_Quotation_Release_Time => "'A' (Anticipated Quotation Release Time)".to_string(),
      eIPOQuotationReleaseQualifier::IPO_Release_Canceled_Postponed => "'C' (IPO Release Canceled Postponed)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eMarketCode(pub u8);
#[allow(non_upper_case_globals)]
impl eMarketCode {
  pub const Nasdaq : u8 = 'Q' as u8;
  pub const BX : u8 = 'B' as u8;
  pub const PSX : u8 = 'X' as u8;
} // eMarketCode

impl fmt::Display for eMarketCode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eMarketCode::Nasdaq => "'Q' (Nasdaq)".to_string(),
      eMarketCode::BX => "'B' (BX)".to_string(),
      eMarketCode::PSX => "'X' (PSX)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eOperationalHaltAction(pub u8);
#[allow(non_upper_case_globals)]
impl eOperationalHaltAction {
  pub const Halted : u8 = 'H' as u8;
  pub const Trading : u8 = 'T' as u8;
} // eOperationalHaltAction

impl fmt::Display for eOperationalHaltAction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eOperationalHaltAction::Halted => "'H' (Halted)".to_string(),
      eOperationalHaltAction::Trading => "'T' (Trading)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eBuySellIndicator(pub u8);
#[allow(non_upper_case_globals)]
impl eBuySellIndicator {
  pub const Buy_Order : u8 = 'B' as u8;
  pub const Sell_Order : u8 = 'S' as u8;
} // eBuySellIndicator

impl fmt::Display for eBuySellIndicator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eBuySellIndicator::Buy_Order => "'B' (Buy Order)".to_string(),
      eBuySellIndicator::Sell_Order => "'S' (Sell Order)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct ePrintable(pub u8);
#[allow(non_upper_case_globals)]
impl ePrintable {
  pub const Non_Printable : u8 = 'N' as u8;
  pub const Printable : u8 = 'Y' as u8;
} // ePrintable

impl fmt::Display for ePrintable {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      ePrintable::Non_Printable => "'N' (Non Printable)".to_string(),
      ePrintable::Printable => "'Y' (Printable)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eCrossType(pub u8);
#[allow(non_upper_case_globals)]
impl eCrossType {
  pub const Nasdaq_Opening_Cross : u8 = 'O' as u8;
  pub const Nasdaq_Closing_Cross : u8 = 'C' as u8;
  pub const Cross_for_IPO_and_halted_paused_securities : u8 = 'H' as u8;
  pub const Nasdaq_Cross_Network_Intraday_Cross_and_Post_Close_Cross : u8 = 'I' as u8;
} // eCrossType

impl fmt::Display for eCrossType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eCrossType::Nasdaq_Opening_Cross => "'O' (Nasdaq Opening Cross)".to_string(),
      eCrossType::Nasdaq_Closing_Cross => "'C' (Nasdaq Closing Cross)".to_string(),
      eCrossType::Cross_for_IPO_and_halted_paused_securities => "'H' (Cross for IPO and halted paused securities)".to_string(),
      eCrossType::Nasdaq_Cross_Network_Intraday_Cross_and_Post_Close_Cross => "'I' (Nasdaq Cross Network Intraday Cross and Post Close Cross)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eImbalanceDirection(pub u8);
#[allow(non_upper_case_globals)]
impl eImbalanceDirection {
  pub const buy_imbalance : u8 = 'B' as u8;
  pub const sell_imbalance : u8 = 'S' as u8;
  pub const no_imbalance : u8 = 'N' as u8;
  pub const Insufficient_orders_to_calculate : u8 = 'O' as u8;
} // eImbalanceDirection

impl fmt::Display for eImbalanceDirection {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eImbalanceDirection::buy_imbalance => "'B' (buy imbalance)".to_string(),
      eImbalanceDirection::sell_imbalance => "'S' (sell imbalance)".to_string(),
      eImbalanceDirection::no_imbalance => "'N' (no imbalance)".to_string(),
      eImbalanceDirection::Insufficient_orders_to_calculate => "'O' (Insufficient orders to calculate)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct ePriceVariationIndicator(pub u8);
#[allow(non_upper_case_globals)]
impl ePriceVariationIndicator {
  pub const less_than_1_pct : u8 = 'L' as u8;
  pub const deviation_1_to_2_pct : u8 = '1' as u8;
  pub const deviation_2_to_3_pct : u8 = '2' as u8;
  pub const deviation_3_to_4_pct : u8 = '3' as u8;
  pub const deviation_4_to_5_pct : u8 = '4' as u8;
  pub const deviation_5_to_6_pct : u8 = '5' as u8;
  pub const deviation_6_to_7_pct : u8 = '6' as u8;
  pub const deviation_7_to_8_pct : u8 = '7' as u8;
  pub const deviation_8_to_9_pct : u8 = '8' as u8;
  pub const deviation_9_to_10_pct : u8 = '9' as u8;
  pub const deviation_10_to_20_pct : u8 = 'A' as u8;
  pub const deviation_20_to_30_pct : u8 = 'B' as u8;
  pub const deviation_30_pct_or_greater : u8 = 'C' as u8;
  pub const Cannot_be_calculated : u8 = ' ' as u8;
} // ePriceVariationIndicator

impl fmt::Display for ePriceVariationIndicator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      ePriceVariationIndicator::less_than_1_pct => "'L' (less than 1 pct)".to_string(),
      ePriceVariationIndicator::deviation_1_to_2_pct => "'1' (deviation 1 to 2 pct)".to_string(),
      ePriceVariationIndicator::deviation_2_to_3_pct => "'2' (deviation 2 to 3 pct)".to_string(),
      ePriceVariationIndicator::deviation_3_to_4_pct => "'3' (deviation 3 to 4 pct)".to_string(),
      ePriceVariationIndicator::deviation_4_to_5_pct => "'4' (deviation 4 to 5 pct)".to_string(),
      ePriceVariationIndicator::deviation_5_to_6_pct => "'5' (deviation 5 to 6 pct)".to_string(),
      ePriceVariationIndicator::deviation_6_to_7_pct => "'6' (deviation 6 to 7 pct)".to_string(),
      ePriceVariationIndicator::deviation_7_to_8_pct => "'7' (deviation 7 to 8 pct)".to_string(),
      ePriceVariationIndicator::deviation_8_to_9_pct => "'8' (deviation 8 to 9 pct)".to_string(),
      ePriceVariationIndicator::deviation_9_to_10_pct => "'9' (deviation 9 to 10 pct)".to_string(),
      ePriceVariationIndicator::deviation_10_to_20_pct => "'A' (deviation 10 to 20 pct)".to_string(),
      ePriceVariationIndicator::deviation_20_to_30_pct => "'B' (deviation 20 to 30 pct)".to_string(),
      ePriceVariationIndicator::deviation_30_pct_or_greater => "'C' (deviation 30 pct or greater)".to_string(),
      ePriceVariationIndicator::Cannot_be_calculated => "' ' (Cannot be calculated)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eInterestFlag(pub u8);
#[allow(non_upper_case_globals)]
impl eInterestFlag {
  pub const RPI_orders_available_on_the_buy_side : u8 = 'B' as u8;
  pub const RPI_orders_available_on_the_sell_side : u8 = 'S' as u8;
  pub const RPI_orders_available_on_both_sides : u8 = 'A' as u8;
  pub const No_RPI_orders_available : u8 = 'N' as u8;
} // eInterestFlag

impl fmt::Display for eInterestFlag {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eInterestFlag::RPI_orders_available_on_the_buy_side => "'B' (RPI orders available on the buy side)".to_string(),
      eInterestFlag::RPI_orders_available_on_the_sell_side => "'S' (RPI orders available on the sell side)".to_string(),
      eInterestFlag::RPI_orders_available_on_both_sides => "'A' (RPI orders available on both sides)".to_string(),
      eInterestFlag::No_RPI_orders_available => "'N' (No RPI orders available)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[derive(PartialEq, Default)]
#[allow(non_camel_case_types)]
pub struct eIssueClassification(pub u8);
#[allow(non_upper_case_globals)]
impl eIssueClassification {
  pub const American_Depositary_Share : u8 = 'A' as u8;
  pub const Bond : u8 = 'B' as u8;
  pub const Common_Stock : u8 = 'C' as u8;
  pub const Depository_Receipt : u8 = 'F' as u8;
  pub const Rule_144A : u8 = 'I' as u8;
  pub const Limited_Partnership : u8 = 'L' as u8;
  pub const Notes : u8 = 'N' as u8;
  pub const Ordinary_Share : u8 = 'O' as u8;
  pub const Preferred_Stock : u8 = 'P' as u8;
  pub const Other_Securities : u8 = 'Q' as u8;
  pub const Right : u8 = 'R' as u8;
  pub const Shares_of_Beneficial_Interest : u8 = 'S' as u8;
  pub const Convertible_Debenture : u8 = 'T' as u8;
  pub const Unit : u8 = 'U' as u8;
  pub const Units_Benif_Int : u8 = 'V' as u8;
  pub const Warrant : u8 = 'W' as u8;
} // eIssueClassification

impl fmt::Display for eIssueClassification {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self.0 {
      eIssueClassification::American_Depositary_Share => "'A' (American Depositary Share)".to_string(),
      eIssueClassification::Bond => "'B' (Bond)".to_string(),
      eIssueClassification::Common_Stock => "'C' (Common Stock)".to_string(),
      eIssueClassification::Depository_Receipt => "'F' (Depository Receipt)".to_string(),
      eIssueClassification::Rule_144A => "'I' (Rule 144A)".to_string(),
      eIssueClassification::Limited_Partnership => "'L' (Limited Partnership)".to_string(),
      eIssueClassification::Notes => "'N' (Notes)".to_string(),
      eIssueClassification::Ordinary_Share => "'O' (Ordinary Share)".to_string(),
      eIssueClassification::Preferred_Stock => "'P' (Preferred Stock)".to_string(),
      eIssueClassification::Other_Securities => "'Q' (Other Securities)".to_string(),
      eIssueClassification::Right => "'R' (Right)".to_string(),
      eIssueClassification::Shares_of_Beneficial_Interest => "'S' (Shares of Beneficial Interest)".to_string(),
      eIssueClassification::Convertible_Debenture => "'T' (Convertible Debenture)".to_string(),
      eIssueClassification::Unit => "'U' (Unit)".to_string(),
      eIssueClassification::Units_Benif_Int => "'V' (Units Benif Int)".to_string(),
      eIssueClassification::Warrant => "'W' (Warrant)".to_string(),
      _ => "Unknown".to_string(),
    };
    write!(f, "{}", s)
  }
}

// Structs
#[derive(Default)]
pub struct PacketHeader {
  pub session: [u8;10],
  pub sequence_number: u64,
  pub message_count: u16,
} // PacketHeader
pub const PACKET_HEADER_SIZE : usize = 20;

impl PacketHeader {
  pub fn from_bytes(bytes: &[u8]) -> Option<(PacketHeader,usize)> {
    if bytes.len() < PACKET_HEADER_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(&mut obj.session[..10]).unwrap();
    obj.sequence_number = rdr.read_u64::<BigEndian>().unwrap();
    obj.message_count = rdr.read_u16::<BigEndian>().unwrap();
    Some((obj, PACKET_HEADER_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<PacketHeader> {
    let mut obj = Self::default();
    rdr.read_exact(&mut obj.session[..10])?;
    obj.sequence_number = rdr.read_u64::<BigEndian>()?;
    obj.message_count = rdr.read_u16::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for PacketHeader {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "PacketHeader(session:{},sequence_number:{},message_count:{})", String::from_utf8_lossy(&self.session[..]), self.sequence_number, self.message_count)
  }
}
#[derive(Default)]
pub struct MessageBlock {
  pub message_length: u16,
} // MessageBlock
pub const MESSAGE_BLOCK_SIZE : usize = 2;

impl MessageBlock {
  pub fn from_bytes(bytes: &[u8]) -> Option<(MessageBlock,usize)> {
    if bytes.len() < MESSAGE_BLOCK_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    obj.message_length = rdr.read_u16::<BigEndian>().unwrap();
    Some((obj, MESSAGE_BLOCK_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<MessageBlock> {
    let mut obj = Self::default();
    obj.message_length = rdr.read_u16::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for MessageBlock {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "MessageBlock(message_length:{})", self.message_length)
  }
}
#[derive(Default)]
pub struct RequestPacket {
  pub session: [u8;10],
  pub sequence_number: [u8;8],
  pub requested_message_count: u16,
} // RequestPacket
pub const REQUEST_PACKET_SIZE : usize = 20;

impl RequestPacket {
  pub fn from_bytes(bytes: &[u8]) -> Option<(RequestPacket,usize)> {
    if bytes.len() < REQUEST_PACKET_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(&mut obj.session[..10]).unwrap();
    rdr.read_exact(&mut obj.sequence_number[..8]).unwrap();
    obj.requested_message_count = rdr.read_u16::<BigEndian>().unwrap();
    Some((obj, REQUEST_PACKET_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<RequestPacket> {
    let mut obj = Self::default();
    rdr.read_exact(&mut obj.session[..10])?;
    rdr.read_exact(&mut obj.sequence_number[..8])?;
    obj.requested_message_count = rdr.read_u16::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for RequestPacket {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RequestPacket(session:{},sequence_number:{},requested_message_count:{})", String::from_utf8_lossy(&self.session[..]), String::from_utf8_lossy(&self.sequence_number[..]), self.requested_message_count)
  }
}
#[derive(Default)]
pub struct SystemEvent {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub event_code: eSystemEvent,
} // SystemEvent
pub const SYSTEM_EVENT_SIZE : usize = 12;

impl SystemEvent {
  pub const TYPE : u8 = 'S' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(SystemEvent,usize)> {
    if bytes.len() < SYSTEM_EVENT_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(std::slice::from_mut(&mut obj.event_code.0)).unwrap();
    Some((obj, SYSTEM_EVENT_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<SystemEvent> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(std::slice::from_mut(&mut obj.event_code.0))?;
    Ok(obj)
  }
}
impl fmt::Display for SystemEvent {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "SystemEvent(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},event_code:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.event_code)
  }
}
#[derive(Default)]
pub struct StockDirectory {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub stock: [u8;8],
  pub market_category: eMarketCategory,
  pub financial_status_indicator: eFinancialStatusIndicator,
  pub round_lot_size: u32,
  pub round_lots_only: eRoundLotsOnly,
  pub issue_classification: eIssueClassification,
  pub issue_sub_type: [u8;2],
  pub authenticity: eAuthenticity,
  pub short_sale_threshold_indicator: eShortSaleThresholdIndicator,
  pub ipo_flag: eIPOFlag,
  pub luld_reference_price_tier: eLULDReferencePriceTier,
  pub etp_flag: eETPFlag,
  pub etp_leverage_factor: u32,
  pub inverse_indicator: eInverseIndicator,
} // StockDirectory
pub const STOCK_DIRECTORY_SIZE : usize = 39;

impl StockDirectory {
  pub const TYPE : u8 = 'R' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(StockDirectory,usize)> {
    if bytes.len() < STOCK_DIRECTORY_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.market_category.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.financial_status_indicator.0)).unwrap();
    obj.round_lot_size = rdr.read_u32::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.round_lots_only.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.issue_classification.0)).unwrap();
    rdr.read_exact(&mut obj.issue_sub_type[..2]).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.authenticity.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.short_sale_threshold_indicator.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.ipo_flag.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.luld_reference_price_tier.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.etp_flag.0)).unwrap();
    obj.etp_leverage_factor = rdr.read_u32::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.inverse_indicator.0)).unwrap();
    Some((obj, STOCK_DIRECTORY_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<StockDirectory> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8])?;
    rdr.read_exact(std::slice::from_mut(&mut obj.market_category.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.financial_status_indicator.0))?;
    obj.round_lot_size = rdr.read_u32::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.round_lots_only.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.issue_classification.0))?;
    rdr.read_exact(&mut obj.issue_sub_type[..2])?;
    rdr.read_exact(std::slice::from_mut(&mut obj.authenticity.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.short_sale_threshold_indicator.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.ipo_flag.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.luld_reference_price_tier.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.etp_flag.0))?;
    obj.etp_leverage_factor = rdr.read_u32::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.inverse_indicator.0))?;
    Ok(obj)
  }
}
impl fmt::Display for StockDirectory {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "StockDirectory(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},stock:{},market_category:{},financial_status_indicator:{},round_lot_size:{},round_lots_only:{},issue_classification:{},issue_sub_type:{},authenticity:{},short_sale_threshold_indicator:{},ipo_flag:{},luld_reference_price_tier:{},etp_flag:{},etp_leverage_factor:{},inverse_indicator:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, String::from_utf8_lossy(&self.stock[..]), self.market_category, self.financial_status_indicator, self.round_lot_size, self.round_lots_only, self.issue_classification, String::from_utf8_lossy(&self.issue_sub_type[..]), self.authenticity, self.short_sale_threshold_indicator, self.ipo_flag, self.luld_reference_price_tier, self.etp_flag, self.etp_leverage_factor, self.inverse_indicator)
  }
}
#[derive(Default)]
pub struct StockTradingAction {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub stock: [u8;8],
  pub trading_state: eTradingState,
  pub reserved: u8,
  pub reason: [u8;4],
} // StockTradingAction
pub const STOCK_TRADING_ACTION_SIZE : usize = 25;

impl StockTradingAction {
  pub const TYPE : u8 = 'H' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(StockTradingAction,usize)> {
    if bytes.len() < STOCK_TRADING_ACTION_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.trading_state.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.reserved)).unwrap();
    rdr.read_exact(&mut obj.reason[..4]).unwrap();
    Some((obj, STOCK_TRADING_ACTION_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<StockTradingAction> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8])?;
    rdr.read_exact(std::slice::from_mut(&mut obj.trading_state.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.reserved))?;
    rdr.read_exact(&mut obj.reason[..4])?;
    Ok(obj)
  }
}
impl fmt::Display for StockTradingAction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "StockTradingAction(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},stock:{},trading_state:{},reserved:{},reason:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, String::from_utf8_lossy(&self.stock[..]), self.trading_state, self.reserved, String::from_utf8_lossy(&self.reason[..]))
  }
}
#[derive(Default)]
pub struct RegShoRestriction {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub stock: [u8;8],
  pub reg_sho_action: eRegSHOAction,
} // RegShoRestriction
pub const REG_SHO_RESTRICTION_SIZE : usize = 20;

impl RegShoRestriction {
  pub const TYPE : u8 = 'Y' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(RegShoRestriction,usize)> {
    if bytes.len() < REG_SHO_RESTRICTION_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.reg_sho_action.0)).unwrap();
    Some((obj, REG_SHO_RESTRICTION_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<RegShoRestriction> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8])?;
    rdr.read_exact(std::slice::from_mut(&mut obj.reg_sho_action.0))?;
    Ok(obj)
  }
}
impl fmt::Display for RegShoRestriction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RegShoRestriction(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},stock:{},reg_sho_action:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, String::from_utf8_lossy(&self.stock[..]), self.reg_sho_action)
  }
}
#[derive(Default)]
pub struct MarketParticipantPosition {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub mpid: [u8;4],
  pub stock: [u8;8],
  pub primary_market_maker: ePrimaryMarketMaker,
  pub market_maker_mode: eMarketMakerMode,
  pub market_participant_state: eMarketParticipantState,
} // MarketParticipantPosition
pub const MARKET_PARTICIPANT_POSITION_SIZE : usize = 26;

impl MarketParticipantPosition {
  pub const TYPE : u8 = 'L' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(MarketParticipantPosition,usize)> {
    if bytes.len() < MARKET_PARTICIPANT_POSITION_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.mpid[..4]).unwrap();
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.primary_market_maker.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.market_maker_mode.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.market_participant_state.0)).unwrap();
    Some((obj, MARKET_PARTICIPANT_POSITION_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<MarketParticipantPosition> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.mpid[..4])?;
    rdr.read_exact(&mut obj.stock[..8])?;
    rdr.read_exact(std::slice::from_mut(&mut obj.primary_market_maker.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.market_maker_mode.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.market_participant_state.0))?;
    Ok(obj)
  }
}
impl fmt::Display for MarketParticipantPosition {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "MarketParticipantPosition(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},mpid:{},stock:{},primary_market_maker:{},market_maker_mode:{},market_participant_state:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, String::from_utf8_lossy(&self.mpid[..]), String::from_utf8_lossy(&self.stock[..]), self.primary_market_maker, self.market_maker_mode, self.market_participant_state)
  }
}
#[derive(Default)]
pub struct MwcbDeclineLevel {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub level_1: u64,
  pub level_2: u64,
  pub level_3: u64,
} // MwcbDeclineLevel
pub const MWCB_DECLINE_LEVEL_SIZE : usize = 35;

impl MwcbDeclineLevel {
  pub const TYPE : u8 = 'V' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(MwcbDeclineLevel,usize)> {
    if bytes.len() < MWCB_DECLINE_LEVEL_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.level_1 = rdr.read_u64::<BigEndian>().unwrap();
    obj.level_2 = rdr.read_u64::<BigEndian>().unwrap();
    obj.level_3 = rdr.read_u64::<BigEndian>().unwrap();
    Some((obj, MWCB_DECLINE_LEVEL_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<MwcbDeclineLevel> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.level_1 = rdr.read_u64::<BigEndian>()?;
    obj.level_2 = rdr.read_u64::<BigEndian>()?;
    obj.level_3 = rdr.read_u64::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for MwcbDeclineLevel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "MwcbDeclineLevel(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},level_1:{},level_2:{},level_3:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.level_1, self.level_2, self.level_3)
  }
}
#[derive(Default)]
pub struct MwcbStatus {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub breached_level: eBreachedLevel,
} // MwcbStatus
pub const MWCB_STATUS_SIZE : usize = 12;

impl MwcbStatus {
  pub const TYPE : u8 = 'W' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(MwcbStatus,usize)> {
    if bytes.len() < MWCB_STATUS_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(std::slice::from_mut(&mut obj.breached_level.0)).unwrap();
    Some((obj, MWCB_STATUS_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<MwcbStatus> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(std::slice::from_mut(&mut obj.breached_level.0))?;
    Ok(obj)
  }
}
impl fmt::Display for MwcbStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "MwcbStatus(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},breached_level:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.breached_level)
  }
}
#[derive(Default)]
pub struct IpoQuotingPeriodUpdate {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub stock: [u8;8],
  pub ipo_quotation_release_time: u32,
  pub ipo_quotation_release_qualifier: eIPOQuotationReleaseQualifier,
  pub ipo_price: u32,
} // IpoQuotingPeriodUpdate
pub const IPO_QUOTING_PERIOD_UPDATE_SIZE : usize = 28;

impl IpoQuotingPeriodUpdate {
  pub const TYPE : u8 = 'K' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(IpoQuotingPeriodUpdate,usize)> {
    if bytes.len() < IPO_QUOTING_PERIOD_UPDATE_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    obj.ipo_quotation_release_time = rdr.read_u32::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.ipo_quotation_release_qualifier.0)).unwrap();
    obj.ipo_price = rdr.read_u32::<BigEndian>().unwrap();
    Some((obj, IPO_QUOTING_PERIOD_UPDATE_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<IpoQuotingPeriodUpdate> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8])?;
    obj.ipo_quotation_release_time = rdr.read_u32::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.ipo_quotation_release_qualifier.0))?;
    obj.ipo_price = rdr.read_u32::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for IpoQuotingPeriodUpdate {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "IpoQuotingPeriodUpdate(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},stock:{},ipo_quotation_release_time:{},ipo_quotation_release_qualifier:{},ipo_price:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, String::from_utf8_lossy(&self.stock[..]), self.ipo_quotation_release_time, self.ipo_quotation_release_qualifier, self.ipo_price)
  }
}
#[derive(Default)]
pub struct LuldAuctionCollar {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub stock: [u8;8],
  pub auction_collar_reference_price: u32,
  pub upper_auction_collar_price: u32,
  pub lower_auction_collar_price: u32,
  pub auction_collar_extension: u32,
} // LuldAuctionCollar
pub const LULD_AUCTION_COLLAR_SIZE : usize = 35;

impl LuldAuctionCollar {
  pub const TYPE : u8 = 'J' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(LuldAuctionCollar,usize)> {
    if bytes.len() < LULD_AUCTION_COLLAR_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    obj.auction_collar_reference_price = rdr.read_u32::<BigEndian>().unwrap();
    obj.upper_auction_collar_price = rdr.read_u32::<BigEndian>().unwrap();
    obj.lower_auction_collar_price = rdr.read_u32::<BigEndian>().unwrap();
    obj.auction_collar_extension = rdr.read_u32::<BigEndian>().unwrap();
    Some((obj, LULD_AUCTION_COLLAR_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<LuldAuctionCollar> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8])?;
    obj.auction_collar_reference_price = rdr.read_u32::<BigEndian>()?;
    obj.upper_auction_collar_price = rdr.read_u32::<BigEndian>()?;
    obj.lower_auction_collar_price = rdr.read_u32::<BigEndian>()?;
    obj.auction_collar_extension = rdr.read_u32::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for LuldAuctionCollar {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "LuldAuctionCollar(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},stock:{},auction_collar_reference_price:{},upper_auction_collar_price:{},lower_auction_collar_price:{},auction_collar_extension:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, String::from_utf8_lossy(&self.stock[..]), self.auction_collar_reference_price, self.upper_auction_collar_price, self.lower_auction_collar_price, self.auction_collar_extension)
  }
}
#[derive(Default)]
pub struct OperationalHalt {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub stock: [u8;8],
  pub market_code: eMarketCode,
  pub operational_halt_action: eOperationalHaltAction,
} // OperationalHalt
pub const OPERATIONAL_HALT_SIZE : usize = 21;

impl OperationalHalt {
  pub const TYPE : u8 = 'h' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(OperationalHalt,usize)> {
    if bytes.len() < OPERATIONAL_HALT_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.market_code.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.operational_halt_action.0)).unwrap();
    Some((obj, OPERATIONAL_HALT_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<OperationalHalt> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8])?;
    rdr.read_exact(std::slice::from_mut(&mut obj.market_code.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.operational_halt_action.0))?;
    Ok(obj)
  }
}
impl fmt::Display for OperationalHalt {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OperationalHalt(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},stock:{},market_code:{},operational_halt_action:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, String::from_utf8_lossy(&self.stock[..]), self.market_code, self.operational_halt_action)
  }
}
#[derive(Default)]
pub struct AddOrder {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub order_reference_number: u64,
  pub buy_sell_indicator: eBuySellIndicator,
  pub shares: u32,
  pub stock: [u8;8],
  pub price: u32,
} // AddOrder
pub const ADD_ORDER_SIZE : usize = 36;

impl AddOrder {
  pub const TYPE : u8 = 'A' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(AddOrder,usize)> {
    if bytes.len() < ADD_ORDER_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.buy_sell_indicator.0)).unwrap();
    obj.shares = rdr.read_u32::<BigEndian>().unwrap();
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    obj.price = rdr.read_u32::<BigEndian>().unwrap();
    Some((obj, ADD_ORDER_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<AddOrder> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.buy_sell_indicator.0))?;
    obj.shares = rdr.read_u32::<BigEndian>()?;
    rdr.read_exact(&mut obj.stock[..8])?;
    obj.price = rdr.read_u32::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for AddOrder {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AddOrder(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},order_reference_number:{},buy_sell_indicator:{},shares:{},stock:{},price:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.order_reference_number, self.buy_sell_indicator, self.shares, String::from_utf8_lossy(&self.stock[..]), self.price)
  }
}
#[derive(Default)]
pub struct AddOrderWithMpid {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub order_reference_number: u64,
  pub buy_sell_indicator: eBuySellIndicator,
  pub shares: u32,
  pub stock: [u8;8],
  pub price: u32,
  pub attribution: [u8;4],
} // AddOrderWithMpid
pub const ADD_ORDER_WITH_MPID_SIZE : usize = 40;

impl AddOrderWithMpid {
  pub const TYPE : u8 = 'F' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(AddOrderWithMpid,usize)> {
    if bytes.len() < ADD_ORDER_WITH_MPID_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.buy_sell_indicator.0)).unwrap();
    obj.shares = rdr.read_u32::<BigEndian>().unwrap();
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    obj.price = rdr.read_u32::<BigEndian>().unwrap();
    rdr.read_exact(&mut obj.attribution[..4]).unwrap();
    Some((obj, ADD_ORDER_WITH_MPID_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<AddOrderWithMpid> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.buy_sell_indicator.0))?;
    obj.shares = rdr.read_u32::<BigEndian>()?;
    rdr.read_exact(&mut obj.stock[..8])?;
    obj.price = rdr.read_u32::<BigEndian>()?;
    rdr.read_exact(&mut obj.attribution[..4])?;
    Ok(obj)
  }
}
impl fmt::Display for AddOrderWithMpid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "AddOrderWithMpid(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},order_reference_number:{},buy_sell_indicator:{},shares:{},stock:{},price:{},attribution:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.order_reference_number, self.buy_sell_indicator, self.shares, String::from_utf8_lossy(&self.stock[..]), self.price, String::from_utf8_lossy(&self.attribution[..]))
  }
}
#[derive(Default)]
pub struct OrderExecuted {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub order_reference_number: u64,
  pub executed_shares: u32,
  pub match_number: u64,
} // OrderExecuted
pub const ORDER_EXECUTED_SIZE : usize = 31;

impl OrderExecuted {
  pub const TYPE : u8 = 'E' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(OrderExecuted,usize)> {
    if bytes.len() < ORDER_EXECUTED_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>().unwrap();
    obj.executed_shares = rdr.read_u32::<BigEndian>().unwrap();
    obj.match_number = rdr.read_u64::<BigEndian>().unwrap();
    Some((obj, ORDER_EXECUTED_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<OrderExecuted> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>()?;
    obj.executed_shares = rdr.read_u32::<BigEndian>()?;
    obj.match_number = rdr.read_u64::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for OrderExecuted {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OrderExecuted(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},order_reference_number:{},executed_shares:{},match_number:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.order_reference_number, self.executed_shares, self.match_number)
  }
}
#[derive(Default)]
pub struct OrderExecutedWithPrice {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub order_reference_number: u64,
  pub executed_shares: u32,
  pub match_number: u64,
  pub printable: ePrintable,
  pub execution_price: u32,
} // OrderExecutedWithPrice
pub const ORDER_EXECUTED_WITH_PRICE_SIZE : usize = 36;

impl OrderExecutedWithPrice {
  pub const TYPE : u8 = 'C' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(OrderExecutedWithPrice,usize)> {
    if bytes.len() < ORDER_EXECUTED_WITH_PRICE_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>().unwrap();
    obj.executed_shares = rdr.read_u32::<BigEndian>().unwrap();
    obj.match_number = rdr.read_u64::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.printable.0)).unwrap();
    obj.execution_price = rdr.read_u32::<BigEndian>().unwrap();
    Some((obj, ORDER_EXECUTED_WITH_PRICE_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<OrderExecutedWithPrice> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>()?;
    obj.executed_shares = rdr.read_u32::<BigEndian>()?;
    obj.match_number = rdr.read_u64::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.printable.0))?;
    obj.execution_price = rdr.read_u32::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for OrderExecutedWithPrice {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OrderExecutedWithPrice(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},order_reference_number:{},executed_shares:{},match_number:{},printable:{},execution_price:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.order_reference_number, self.executed_shares, self.match_number, self.printable, self.execution_price)
  }
}
#[derive(Default)]
pub struct OrderCancel {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub order_reference_number: u64,
  pub cancelled_shares: u32,
} // OrderCancel
pub const ORDER_CANCEL_SIZE : usize = 23;

impl OrderCancel {
  pub const TYPE : u8 = 'X' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(OrderCancel,usize)> {
    if bytes.len() < ORDER_CANCEL_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>().unwrap();
    obj.cancelled_shares = rdr.read_u32::<BigEndian>().unwrap();
    Some((obj, ORDER_CANCEL_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<OrderCancel> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>()?;
    obj.cancelled_shares = rdr.read_u32::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for OrderCancel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OrderCancel(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},order_reference_number:{},cancelled_shares:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.order_reference_number, self.cancelled_shares)
  }
}
#[derive(Default)]
pub struct OrderDelete {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub order_reference_number: u64,
} // OrderDelete
pub const ORDER_DELETE_SIZE : usize = 19;

impl OrderDelete {
  pub const TYPE : u8 = 'D' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(OrderDelete,usize)> {
    if bytes.len() < ORDER_DELETE_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>().unwrap();
    Some((obj, ORDER_DELETE_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<OrderDelete> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for OrderDelete {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OrderDelete(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},order_reference_number:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.order_reference_number)
  }
}
#[derive(Default)]
pub struct OrderReplace {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub original_order_reference_number: u64,
  pub new_order_reference_number: u64,
  pub shares: u32,
  pub price: u32,
} // OrderReplace
pub const ORDER_REPLACE_SIZE : usize = 35;

impl OrderReplace {
  pub const TYPE : u8 = 'U' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(OrderReplace,usize)> {
    if bytes.len() < ORDER_REPLACE_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.original_order_reference_number = rdr.read_u64::<BigEndian>().unwrap();
    obj.new_order_reference_number = rdr.read_u64::<BigEndian>().unwrap();
    obj.shares = rdr.read_u32::<BigEndian>().unwrap();
    obj.price = rdr.read_u32::<BigEndian>().unwrap();
    Some((obj, ORDER_REPLACE_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<OrderReplace> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.original_order_reference_number = rdr.read_u64::<BigEndian>()?;
    obj.new_order_reference_number = rdr.read_u64::<BigEndian>()?;
    obj.shares = rdr.read_u32::<BigEndian>()?;
    obj.price = rdr.read_u32::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for OrderReplace {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "OrderReplace(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},original_order_reference_number:{},new_order_reference_number:{},shares:{},price:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.original_order_reference_number, self.new_order_reference_number, self.shares, self.price)
  }
}
#[derive(Default)]
pub struct Trade {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub order_reference_number: u64,
  pub buy_sell_indicator: eBuySellIndicator,
  pub shares: u32,
  pub stock: [u8;8],
  pub price: u32,
  pub match_number: u64,
} // Trade
pub const TRADE_SIZE : usize = 44;

impl Trade {
  pub const TYPE : u8 = 'P' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(Trade,usize)> {
    if bytes.len() < TRADE_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.buy_sell_indicator.0)).unwrap();
    obj.shares = rdr.read_u32::<BigEndian>().unwrap();
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    obj.price = rdr.read_u32::<BigEndian>().unwrap();
    obj.match_number = rdr.read_u64::<BigEndian>().unwrap();
    Some((obj, TRADE_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<Trade> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.order_reference_number = rdr.read_u64::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.buy_sell_indicator.0))?;
    obj.shares = rdr.read_u32::<BigEndian>()?;
    rdr.read_exact(&mut obj.stock[..8])?;
    obj.price = rdr.read_u32::<BigEndian>()?;
    obj.match_number = rdr.read_u64::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for Trade {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Trade(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},order_reference_number:{},buy_sell_indicator:{},shares:{},stock:{},price:{},match_number:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.order_reference_number, self.buy_sell_indicator, self.shares, String::from_utf8_lossy(&self.stock[..]), self.price, self.match_number)
  }
}
#[derive(Default)]
pub struct CrossTrade {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub shares: u64,
  pub stock: [u8;8],
  pub cross_price: u32,
  pub match_number: u64,
  pub cross_type: eCrossType,
} // CrossTrade
pub const CROSS_TRADE_SIZE : usize = 40;

impl CrossTrade {
  pub const TYPE : u8 = 'Q' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(CrossTrade,usize)> {
    if bytes.len() < CROSS_TRADE_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.shares = rdr.read_u64::<BigEndian>().unwrap();
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    obj.cross_price = rdr.read_u32::<BigEndian>().unwrap();
    obj.match_number = rdr.read_u64::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.cross_type.0)).unwrap();
    Some((obj, CROSS_TRADE_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<CrossTrade> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.shares = rdr.read_u64::<BigEndian>()?;
    rdr.read_exact(&mut obj.stock[..8])?;
    obj.cross_price = rdr.read_u32::<BigEndian>()?;
    obj.match_number = rdr.read_u64::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.cross_type.0))?;
    Ok(obj)
  }
}
impl fmt::Display for CrossTrade {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "CrossTrade(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},shares:{},stock:{},cross_price:{},match_number:{},cross_type:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.shares, String::from_utf8_lossy(&self.stock[..]), self.cross_price, self.match_number, self.cross_type)
  }
}
#[derive(Default)]
pub struct BrokenTrade {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub match_number: u64,
} // BrokenTrade
pub const BROKEN_TRADE_SIZE : usize = 19;

impl BrokenTrade {
  pub const TYPE : u8 = 'B' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(BrokenTrade,usize)> {
    if bytes.len() < BROKEN_TRADE_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.match_number = rdr.read_u64::<BigEndian>().unwrap();
    Some((obj, BROKEN_TRADE_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<BrokenTrade> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.match_number = rdr.read_u64::<BigEndian>()?;
    Ok(obj)
  }
}
impl fmt::Display for BrokenTrade {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "BrokenTrade(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},match_number:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.match_number)
  }
}
#[derive(Default)]
pub struct NetOrderImbalanceIndicator {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub paired_shares: u64,
  pub imbalance_shares: u64,
  pub imbalance_direction: eImbalanceDirection,
  pub stock: [u8;8],
  pub far_price: u32,
  pub near_price: u32,
  pub current_reference_price: u32,
  pub cross_type: eCrossType,
  pub price_variation_indicator: ePriceVariationIndicator,
} // NetOrderImbalanceIndicator
pub const NET_ORDER_IMBALANCE_INDICATOR_SIZE : usize = 50;

impl NetOrderImbalanceIndicator {
  pub const TYPE : u8 = 'I' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(NetOrderImbalanceIndicator,usize)> {
    if bytes.len() < NET_ORDER_IMBALANCE_INDICATOR_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    obj.paired_shares = rdr.read_u64::<BigEndian>().unwrap();
    obj.imbalance_shares = rdr.read_u64::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.imbalance_direction.0)).unwrap();
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    obj.far_price = rdr.read_u32::<BigEndian>().unwrap();
    obj.near_price = rdr.read_u32::<BigEndian>().unwrap();
    obj.current_reference_price = rdr.read_u32::<BigEndian>().unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.cross_type.0)).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.price_variation_indicator.0)).unwrap();
    Some((obj, NET_ORDER_IMBALANCE_INDICATOR_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<NetOrderImbalanceIndicator> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    obj.paired_shares = rdr.read_u64::<BigEndian>()?;
    obj.imbalance_shares = rdr.read_u64::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.imbalance_direction.0))?;
    rdr.read_exact(&mut obj.stock[..8])?;
    obj.far_price = rdr.read_u32::<BigEndian>()?;
    obj.near_price = rdr.read_u32::<BigEndian>()?;
    obj.current_reference_price = rdr.read_u32::<BigEndian>()?;
    rdr.read_exact(std::slice::from_mut(&mut obj.cross_type.0))?;
    rdr.read_exact(std::slice::from_mut(&mut obj.price_variation_indicator.0))?;
    Ok(obj)
  }
}
impl fmt::Display for NetOrderImbalanceIndicator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "NetOrderImbalanceIndicator(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},paired_shares:{},imbalance_shares:{},imbalance_direction:{},stock:{},far_price:{},near_price:{},current_reference_price:{},cross_type:{},price_variation_indicator:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, self.paired_shares, self.imbalance_shares, self.imbalance_direction, String::from_utf8_lossy(&self.stock[..]), self.far_price, self.near_price, self.current_reference_price, self.cross_type, self.price_variation_indicator)
  }
}
#[derive(Default)]
pub struct RetailPriceImprovementIndicator {
  pub message_type: u8,
  pub stock_locate: u16,
  pub tracking_number: u16,
  pub timestamp: u64,
  pub stock: [u8;8],
  pub interest_flag: eInterestFlag,
} // RetailPriceImprovementIndicator
pub const RETAIL_PRICE_IMPROVEMENT_INDICATOR_SIZE : usize = 20;

impl RetailPriceImprovementIndicator {
  pub const TYPE : u8 = 'N' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(RetailPriceImprovementIndicator,usize)> {
    if bytes.len() < RETAIL_PRICE_IMPROVEMENT_INDICATOR_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    obj.stock_locate = rdr.read_u16::<BigEndian>().unwrap();
    obj.tracking_number = rdr.read_u16::<BigEndian>().unwrap();
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6]).unwrap();
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8]).unwrap();
    rdr.read_exact(std::slice::from_mut(&mut obj.interest_flag.0)).unwrap();
    Some((obj, RETAIL_PRICE_IMPROVEMENT_INDICATOR_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<RetailPriceImprovementIndicator> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    obj.stock_locate = rdr.read_u16::<BigEndian>()?;
    obj.tracking_number = rdr.read_u16::<BigEndian>()?;
    let mut the_u48 = [0u8; 6];
    rdr.read_exact(&mut the_u48[..6])?;
    obj.timestamp = u48_to_u64(&the_u48);
    rdr.read_exact(&mut obj.stock[..8])?;
    rdr.read_exact(std::slice::from_mut(&mut obj.interest_flag.0))?;
    Ok(obj)
  }
}
impl fmt::Display for RetailPriceImprovementIndicator {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RetailPriceImprovementIndicator(message_type:{},stock_locate:{},tracking_number:{},timestamp:{},stock:{},interest_flag:{})", self.message_type, self.stock_locate, self.tracking_number, self.timestamp, String::from_utf8_lossy(&self.stock[..]), self.interest_flag)
  }
}
#[derive(Default)]
pub struct EndOfSnapshot {
  pub message_type: u8,
  pub sequence_number: [u8;20],
} // EndOfSnapshot
pub const END_OF_SNAPSHOT_SIZE : usize = 21;

impl EndOfSnapshot {
  pub const TYPE : u8 = 'G' as u8;
  pub fn from_bytes(bytes: &[u8]) -> Option<(EndOfSnapshot,usize)> {
    if bytes.len() < END_OF_SNAPSHOT_SIZE {
      return None;
    }
    let mut rdr = Cursor::new(bytes);
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type)).unwrap();
    rdr.read_exact(&mut obj.sequence_number[..20]).unwrap();
    Some((obj, END_OF_SNAPSHOT_SIZE))
  }
  pub fn from_cursor(rdr: &mut Cursor<&[u8]>) -> std::io::Result<EndOfSnapshot> {
    let mut obj = Self::default();
    rdr.read_exact(std::slice::from_mut(&mut obj.message_type))?;
    rdr.read_exact(&mut obj.sequence_number[..20])?;
    Ok(obj)
  }
}
impl fmt::Display for EndOfSnapshot {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "EndOfSnapshot(message_type:{},sequence_number:{})", self.message_type, String::from_utf8_lossy(&self.sequence_number[..]))
  }
}
pub trait ItchHandler {
  fn on_system_event(&mut self, _msg: SystemEvent) {}
  fn on_stock_directory(&mut self, _msg: StockDirectory) {}
  fn on_stock_trading_action(&mut self, _msg: StockTradingAction) {}
  fn on_reg_sho_restriction(&mut self, _msg: RegShoRestriction) {}
  fn on_market_participant_position(&mut self, _msg: MarketParticipantPosition) {}
  fn on_mwcb_decline_level(&mut self, _msg: MwcbDeclineLevel) {}
  fn on_mwcb_status(&mut self, _msg: MwcbStatus) {}
  fn on_ipo_quoting_period_update(&mut self, _msg: IpoQuotingPeriodUpdate) {}
  fn on_luld_auction_collar(&mut self, _msg: LuldAuctionCollar) {}
  fn on_operational_halt(&mut self, _msg: OperationalHalt) {}
  fn on_add_order(&mut self, _msg: AddOrder) {}
  fn on_add_order_with_mpid(&mut self, _msg: AddOrderWithMpid) {}
  fn on_order_executed(&mut self, _msg: OrderExecuted) {}
  fn on_order_executed_with_price(&mut self, _msg: OrderExecutedWithPrice) {}
  fn on_order_cancel(&mut self, _msg: OrderCancel) {}
  fn on_order_delete(&mut self, _msg: OrderDelete) {}
  fn on_order_replace(&mut self, _msg: OrderReplace) {}
  fn on_trade(&mut self, _msg: Trade) {}
  fn on_cross_trade(&mut self, _msg: CrossTrade) {}
  fn on_broken_trade(&mut self, _msg: BrokenTrade) {}
  fn on_net_order_imbalance_indicator(&mut self, _msg: NetOrderImbalanceIndicator) {}
  fn on_retail_price_improvement_indicator(&mut self, _msg: RetailPriceImprovementIndicator) {}
  fn on_end_of_snapshot(&mut self, _msg: EndOfSnapshot) {}
}

pub fn crack_message<T: ItchHandler>(msg: &[u8], handler: &mut T) {
  let tipe = msg[0];
  let mut rdr = Cursor::new(msg);
  match tipe {
    SystemEvent::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const SystemEvent) };
      if let Ok(msg) = SystemEvent::from_cursor(&mut rdr) {
        handler.on_system_event(msg);
      }
      else { unreachable!() }
    },
    StockDirectory::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const StockDirectory) };
      if let Ok(msg) = StockDirectory::from_cursor(&mut rdr) {
        handler.on_stock_directory(msg);
      }
      else { unreachable!() }
    },
    StockTradingAction::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const StockTradingAction) };
      if let Ok(msg) = StockTradingAction::from_cursor(&mut rdr) {
        handler.on_stock_trading_action(msg);
      }
      else { unreachable!() }
    },
    RegShoRestriction::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const RegShoRestriction) };
      if let Ok(msg) = RegShoRestriction::from_cursor(&mut rdr) {
        handler.on_reg_sho_restriction(msg);
      }
      else { unreachable!() }
    },
    MarketParticipantPosition::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const MarketParticipantPosition) };
      if let Ok(msg) = MarketParticipantPosition::from_cursor(&mut rdr) {
        handler.on_market_participant_position(msg);
      }
      else { unreachable!() }
    },
    MwcbDeclineLevel::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const MwcbDeclineLevel) };
      if let Ok(msg) = MwcbDeclineLevel::from_cursor(&mut rdr) {
        handler.on_mwcb_decline_level(msg);
      }
      else { unreachable!() }
    },
    MwcbStatus::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const MwcbStatus) };
      if let Ok(msg) = MwcbStatus::from_cursor(&mut rdr) {
        handler.on_mwcb_status(msg);
      }
      else { unreachable!() }
    },
    IpoQuotingPeriodUpdate::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const IpoQuotingPeriodUpdate) };
      if let Ok(msg) = IpoQuotingPeriodUpdate::from_cursor(&mut rdr) {
        handler.on_ipo_quoting_period_update(msg);
      }
      else { unreachable!() }
    },
    LuldAuctionCollar::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const LuldAuctionCollar) };
      if let Ok(msg) = LuldAuctionCollar::from_cursor(&mut rdr) {
        handler.on_luld_auction_collar(msg);
      }
      else { unreachable!() }
    },
    OperationalHalt::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const OperationalHalt) };
      if let Ok(msg) = OperationalHalt::from_cursor(&mut rdr) {
        handler.on_operational_halt(msg);
      }
      else { unreachable!() }
    },
    AddOrder::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const AddOrder) };
      if let Ok(msg) = AddOrder::from_cursor(&mut rdr) {
        handler.on_add_order(msg);
      }
      else { unreachable!() }
    },
    AddOrderWithMpid::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const AddOrderWithMpid) };
      if let Ok(msg) = AddOrderWithMpid::from_cursor(&mut rdr) {
        handler.on_add_order_with_mpid(msg);
      }
      else { unreachable!() }
    },
    OrderExecuted::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const OrderExecuted) };
      if let Ok(msg) = OrderExecuted::from_cursor(&mut rdr) {
        handler.on_order_executed(msg);
      }
      else { unreachable!() }
    },
    OrderExecutedWithPrice::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const OrderExecutedWithPrice) };
      if let Ok(msg) = OrderExecutedWithPrice::from_cursor(&mut rdr) {
        handler.on_order_executed_with_price(msg);
      }
      else { unreachable!() }
    },
    OrderCancel::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const OrderCancel) };
      if let Ok(msg) = OrderCancel::from_cursor(&mut rdr) {
        handler.on_order_cancel(msg);
      }
      else { unreachable!() }
    },
    OrderDelete::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const OrderDelete) };
      if let Ok(msg) = OrderDelete::from_cursor(&mut rdr) {
        handler.on_order_delete(msg);
      }
      else { unreachable!() }
    },
    OrderReplace::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const OrderReplace) };
      if let Ok(msg) = OrderReplace::from_cursor(&mut rdr) {
        handler.on_order_replace(msg);
      }
      else { unreachable!() }
    },
    Trade::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const Trade) };
      if let Ok(msg) = Trade::from_cursor(&mut rdr) {
        handler.on_trade(msg);
      }
      else { unreachable!() }
    },
    CrossTrade::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const CrossTrade) };
      if let Ok(msg) = CrossTrade::from_cursor(&mut rdr) {
        handler.on_cross_trade(msg);
      }
      else { unreachable!() }
    },
    BrokenTrade::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const BrokenTrade) };
      if let Ok(msg) = BrokenTrade::from_cursor(&mut rdr) {
        handler.on_broken_trade(msg);
      }
      else { unreachable!() }
    },
    NetOrderImbalanceIndicator::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const NetOrderImbalanceIndicator) };
      if let Ok(msg) = NetOrderImbalanceIndicator::from_cursor(&mut rdr) {
        handler.on_net_order_imbalance_indicator(msg);
      }
      else { unreachable!() }
    },
    RetailPriceImprovementIndicator::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const RetailPriceImprovementIndicator) };
      if let Ok(msg) = RetailPriceImprovementIndicator::from_cursor(&mut rdr) {
        handler.on_retail_price_improvement_indicator(msg);
      }
      else { unreachable!() }
    },
    EndOfSnapshot::TYPE => {
      //let msg = unsafe { std::ptr::read(msg[..].as_ptr() as *const EndOfSnapshot) };
      if let Ok(msg) = EndOfSnapshot::from_cursor(&mut rdr) {
        handler.on_end_of_snapshot(msg);
      }
      else { unreachable!() }
    },
    _ => { panic!("unknown type!") },
  }
}
pub fn write_system_event(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, event_code: eSystemEvent) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = SystemEvent::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(std::slice::from_ref(&event_code.0))?;
  assert_eq!(wrt.position() - start_pos, SYSTEM_EVENT_SIZE as u64);
  Ok(())
}

pub fn write_system_event_struct(wrt: &mut Cursor<&mut [u8]>, msg: SystemEvent) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = SystemEvent::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(std::slice::from_ref(&msg.event_code.0))?;
  assert_eq!(wrt.position() - start_pos, SYSTEM_EVENT_SIZE as u64);
  Ok(())
}
pub fn write_stock_directory(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, stock: [u8;8], market_category: eMarketCategory, financial_status_indicator: eFinancialStatusIndicator, round_lot_size: u32, round_lots_only: eRoundLotsOnly, issue_classification: eIssueClassification, issue_sub_type: [u8;2], authenticity: eAuthenticity, short_sale_threshold_indicator: eShortSaleThresholdIndicator, ipo_flag: eIPOFlag, luld_reference_price_tier: eLULDReferencePriceTier, etp_flag: eETPFlag, etp_leverage_factor: u32, inverse_indicator: eInverseIndicator) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = StockDirectory::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(&stock[..8])?;
  wrt.write_all(std::slice::from_ref(&market_category.0))?;
  wrt.write_all(std::slice::from_ref(&financial_status_indicator.0))?;
  wrt.write_u32::<BigEndian>(round_lot_size)?;
  wrt.write_all(std::slice::from_ref(&round_lots_only.0))?;
  wrt.write_all(std::slice::from_ref(&issue_classification.0))?;
  wrt.write_all(&issue_sub_type[..2])?;
  wrt.write_all(std::slice::from_ref(&authenticity.0))?;
  wrt.write_all(std::slice::from_ref(&short_sale_threshold_indicator.0))?;
  wrt.write_all(std::slice::from_ref(&ipo_flag.0))?;
  wrt.write_all(std::slice::from_ref(&luld_reference_price_tier.0))?;
  wrt.write_all(std::slice::from_ref(&etp_flag.0))?;
  wrt.write_u32::<BigEndian>(etp_leverage_factor)?;
  wrt.write_all(std::slice::from_ref(&inverse_indicator.0))?;
  assert_eq!(wrt.position() - start_pos, STOCK_DIRECTORY_SIZE as u64);
  Ok(())
}

pub fn write_stock_directory_struct(wrt: &mut Cursor<&mut [u8]>, msg: StockDirectory) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = StockDirectory::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_all(std::slice::from_ref(&msg.market_category.0))?;
  wrt.write_all(std::slice::from_ref(&msg.financial_status_indicator.0))?;
  wrt.write_u32::<BigEndian>(msg.round_lot_size)?;
  wrt.write_all(std::slice::from_ref(&msg.round_lots_only.0))?;
  wrt.write_all(std::slice::from_ref(&msg.issue_classification.0))?;
  wrt.write_all(&msg.issue_sub_type[..2])?;
  wrt.write_all(std::slice::from_ref(&msg.authenticity.0))?;
  wrt.write_all(std::slice::from_ref(&msg.short_sale_threshold_indicator.0))?;
  wrt.write_all(std::slice::from_ref(&msg.ipo_flag.0))?;
  wrt.write_all(std::slice::from_ref(&msg.luld_reference_price_tier.0))?;
  wrt.write_all(std::slice::from_ref(&msg.etp_flag.0))?;
  wrt.write_u32::<BigEndian>(msg.etp_leverage_factor)?;
  wrt.write_all(std::slice::from_ref(&msg.inverse_indicator.0))?;
  assert_eq!(wrt.position() - start_pos, STOCK_DIRECTORY_SIZE as u64);
  Ok(())
}
pub fn write_stock_trading_action(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, stock: [u8;8], trading_state: eTradingState, reserved: u8, reason: [u8;4]) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = StockTradingAction::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(&stock[..8])?;
  wrt.write_all(std::slice::from_ref(&trading_state.0))?;
  wrt.write_all(std::slice::from_ref(&reserved))?;
  wrt.write_all(&reason[..4])?;
  assert_eq!(wrt.position() - start_pos, STOCK_TRADING_ACTION_SIZE as u64);
  Ok(())
}

pub fn write_stock_trading_action_struct(wrt: &mut Cursor<&mut [u8]>, msg: StockTradingAction) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = StockTradingAction::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_all(std::slice::from_ref(&msg.trading_state.0))?;
  wrt.write_all(std::slice::from_ref(&msg.reserved))?;
  wrt.write_all(&msg.reason[..4])?;
  assert_eq!(wrt.position() - start_pos, STOCK_TRADING_ACTION_SIZE as u64);
  Ok(())
}
pub fn write_reg_sho_restriction(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, stock: [u8;8], reg_sho_action: eRegSHOAction) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = RegShoRestriction::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(&stock[..8])?;
  wrt.write_all(std::slice::from_ref(&reg_sho_action.0))?;
  assert_eq!(wrt.position() - start_pos, REG_SHO_RESTRICTION_SIZE as u64);
  Ok(())
}

pub fn write_reg_sho_restriction_struct(wrt: &mut Cursor<&mut [u8]>, msg: RegShoRestriction) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = RegShoRestriction::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_all(std::slice::from_ref(&msg.reg_sho_action.0))?;
  assert_eq!(wrt.position() - start_pos, REG_SHO_RESTRICTION_SIZE as u64);
  Ok(())
}
pub fn write_market_participant_position(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, mpid: [u8;4], stock: [u8;8], primary_market_maker: ePrimaryMarketMaker, market_maker_mode: eMarketMakerMode, market_participant_state: eMarketParticipantState) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = MarketParticipantPosition::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(&mpid[..4])?;
  wrt.write_all(&stock[..8])?;
  wrt.write_all(std::slice::from_ref(&primary_market_maker.0))?;
  wrt.write_all(std::slice::from_ref(&market_maker_mode.0))?;
  wrt.write_all(std::slice::from_ref(&market_participant_state.0))?;
  assert_eq!(wrt.position() - start_pos, MARKET_PARTICIPANT_POSITION_SIZE as u64);
  Ok(())
}

pub fn write_market_participant_position_struct(wrt: &mut Cursor<&mut [u8]>, msg: MarketParticipantPosition) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = MarketParticipantPosition::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(&msg.mpid[..4])?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_all(std::slice::from_ref(&msg.primary_market_maker.0))?;
  wrt.write_all(std::slice::from_ref(&msg.market_maker_mode.0))?;
  wrt.write_all(std::slice::from_ref(&msg.market_participant_state.0))?;
  assert_eq!(wrt.position() - start_pos, MARKET_PARTICIPANT_POSITION_SIZE as u64);
  Ok(())
}
pub fn write_mwcb_decline_level(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, level_1: u64, level_2: u64, level_3: u64) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = MwcbDeclineLevel::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(level_1)?;
  wrt.write_u64::<BigEndian>(level_2)?;
  wrt.write_u64::<BigEndian>(level_3)?;
  assert_eq!(wrt.position() - start_pos, MWCB_DECLINE_LEVEL_SIZE as u64);
  Ok(())
}

pub fn write_mwcb_decline_level_struct(wrt: &mut Cursor<&mut [u8]>, msg: MwcbDeclineLevel) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = MwcbDeclineLevel::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.level_1)?;
  wrt.write_u64::<BigEndian>(msg.level_2)?;
  wrt.write_u64::<BigEndian>(msg.level_3)?;
  assert_eq!(wrt.position() - start_pos, MWCB_DECLINE_LEVEL_SIZE as u64);
  Ok(())
}
pub fn write_mwcb_status(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, breached_level: eBreachedLevel) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = MwcbStatus::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(std::slice::from_ref(&breached_level.0))?;
  assert_eq!(wrt.position() - start_pos, MWCB_STATUS_SIZE as u64);
  Ok(())
}

pub fn write_mwcb_status_struct(wrt: &mut Cursor<&mut [u8]>, msg: MwcbStatus) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = MwcbStatus::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(std::slice::from_ref(&msg.breached_level.0))?;
  assert_eq!(wrt.position() - start_pos, MWCB_STATUS_SIZE as u64);
  Ok(())
}
pub fn write_ipo_quoting_period_update(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, stock: [u8;8], ipo_quotation_release_time: u32, ipo_quotation_release_qualifier: eIPOQuotationReleaseQualifier, ipo_price: u32) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = IpoQuotingPeriodUpdate::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(&stock[..8])?;
  wrt.write_u32::<BigEndian>(ipo_quotation_release_time)?;
  wrt.write_all(std::slice::from_ref(&ipo_quotation_release_qualifier.0))?;
  wrt.write_u32::<BigEndian>(ipo_price)?;
  assert_eq!(wrt.position() - start_pos, IPO_QUOTING_PERIOD_UPDATE_SIZE as u64);
  Ok(())
}

pub fn write_ipo_quoting_period_update_struct(wrt: &mut Cursor<&mut [u8]>, msg: IpoQuotingPeriodUpdate) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = IpoQuotingPeriodUpdate::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_u32::<BigEndian>(msg.ipo_quotation_release_time)?;
  wrt.write_all(std::slice::from_ref(&msg.ipo_quotation_release_qualifier.0))?;
  wrt.write_u32::<BigEndian>(msg.ipo_price)?;
  assert_eq!(wrt.position() - start_pos, IPO_QUOTING_PERIOD_UPDATE_SIZE as u64);
  Ok(())
}
pub fn write_luld_auction_collar(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, stock: [u8;8], auction_collar_reference_price: u32, upper_auction_collar_price: u32, lower_auction_collar_price: u32, auction_collar_extension: u32) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = LuldAuctionCollar::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(&stock[..8])?;
  wrt.write_u32::<BigEndian>(auction_collar_reference_price)?;
  wrt.write_u32::<BigEndian>(upper_auction_collar_price)?;
  wrt.write_u32::<BigEndian>(lower_auction_collar_price)?;
  wrt.write_u32::<BigEndian>(auction_collar_extension)?;
  assert_eq!(wrt.position() - start_pos, LULD_AUCTION_COLLAR_SIZE as u64);
  Ok(())
}

pub fn write_luld_auction_collar_struct(wrt: &mut Cursor<&mut [u8]>, msg: LuldAuctionCollar) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = LuldAuctionCollar::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_u32::<BigEndian>(msg.auction_collar_reference_price)?;
  wrt.write_u32::<BigEndian>(msg.upper_auction_collar_price)?;
  wrt.write_u32::<BigEndian>(msg.lower_auction_collar_price)?;
  wrt.write_u32::<BigEndian>(msg.auction_collar_extension)?;
  assert_eq!(wrt.position() - start_pos, LULD_AUCTION_COLLAR_SIZE as u64);
  Ok(())
}
pub fn write_operational_halt(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, stock: [u8;8], market_code: eMarketCode, operational_halt_action: eOperationalHaltAction) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OperationalHalt::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(&stock[..8])?;
  wrt.write_all(std::slice::from_ref(&market_code.0))?;
  wrt.write_all(std::slice::from_ref(&operational_halt_action.0))?;
  assert_eq!(wrt.position() - start_pos, OPERATIONAL_HALT_SIZE as u64);
  Ok(())
}

pub fn write_operational_halt_struct(wrt: &mut Cursor<&mut [u8]>, msg: OperationalHalt) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OperationalHalt::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_all(std::slice::from_ref(&msg.market_code.0))?;
  wrt.write_all(std::slice::from_ref(&msg.operational_halt_action.0))?;
  assert_eq!(wrt.position() - start_pos, OPERATIONAL_HALT_SIZE as u64);
  Ok(())
}
pub fn write_add_order(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, order_reference_number: u64, buy_sell_indicator: eBuySellIndicator, shares: u32, stock: [u8;8], price: u32) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = AddOrder::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(order_reference_number)?;
  wrt.write_all(std::slice::from_ref(&buy_sell_indicator.0))?;
  wrt.write_u32::<BigEndian>(shares)?;
  wrt.write_all(&stock[..8])?;
  wrt.write_u32::<BigEndian>(price)?;
  assert_eq!(wrt.position() - start_pos, ADD_ORDER_SIZE as u64);
  Ok(())
}

pub fn write_add_order_struct(wrt: &mut Cursor<&mut [u8]>, msg: AddOrder) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = AddOrder::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.order_reference_number)?;
  wrt.write_all(std::slice::from_ref(&msg.buy_sell_indicator.0))?;
  wrt.write_u32::<BigEndian>(msg.shares)?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_u32::<BigEndian>(msg.price)?;
  assert_eq!(wrt.position() - start_pos, ADD_ORDER_SIZE as u64);
  Ok(())
}
pub fn write_add_order_with_mpid(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, order_reference_number: u64, buy_sell_indicator: eBuySellIndicator, shares: u32, stock: [u8;8], price: u32, attribution: [u8;4]) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = AddOrderWithMpid::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(order_reference_number)?;
  wrt.write_all(std::slice::from_ref(&buy_sell_indicator.0))?;
  wrt.write_u32::<BigEndian>(shares)?;
  wrt.write_all(&stock[..8])?;
  wrt.write_u32::<BigEndian>(price)?;
  wrt.write_all(&attribution[..4])?;
  assert_eq!(wrt.position() - start_pos, ADD_ORDER_WITH_MPID_SIZE as u64);
  Ok(())
}

pub fn write_add_order_with_mpid_struct(wrt: &mut Cursor<&mut [u8]>, msg: AddOrderWithMpid) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = AddOrderWithMpid::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.order_reference_number)?;
  wrt.write_all(std::slice::from_ref(&msg.buy_sell_indicator.0))?;
  wrt.write_u32::<BigEndian>(msg.shares)?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_u32::<BigEndian>(msg.price)?;
  wrt.write_all(&msg.attribution[..4])?;
  assert_eq!(wrt.position() - start_pos, ADD_ORDER_WITH_MPID_SIZE as u64);
  Ok(())
}
pub fn write_order_executed(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, order_reference_number: u64, executed_shares: u32, match_number: u64) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderExecuted::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(order_reference_number)?;
  wrt.write_u32::<BigEndian>(executed_shares)?;
  wrt.write_u64::<BigEndian>(match_number)?;
  assert_eq!(wrt.position() - start_pos, ORDER_EXECUTED_SIZE as u64);
  Ok(())
}

pub fn write_order_executed_struct(wrt: &mut Cursor<&mut [u8]>, msg: OrderExecuted) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderExecuted::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.order_reference_number)?;
  wrt.write_u32::<BigEndian>(msg.executed_shares)?;
  wrt.write_u64::<BigEndian>(msg.match_number)?;
  assert_eq!(wrt.position() - start_pos, ORDER_EXECUTED_SIZE as u64);
  Ok(())
}
pub fn write_order_executed_with_price(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, order_reference_number: u64, executed_shares: u32, match_number: u64, printable: ePrintable, execution_price: u32) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderExecutedWithPrice::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(order_reference_number)?;
  wrt.write_u32::<BigEndian>(executed_shares)?;
  wrt.write_u64::<BigEndian>(match_number)?;
  wrt.write_all(std::slice::from_ref(&printable.0))?;
  wrt.write_u32::<BigEndian>(execution_price)?;
  assert_eq!(wrt.position() - start_pos, ORDER_EXECUTED_WITH_PRICE_SIZE as u64);
  Ok(())
}

pub fn write_order_executed_with_price_struct(wrt: &mut Cursor<&mut [u8]>, msg: OrderExecutedWithPrice) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderExecutedWithPrice::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.order_reference_number)?;
  wrt.write_u32::<BigEndian>(msg.executed_shares)?;
  wrt.write_u64::<BigEndian>(msg.match_number)?;
  wrt.write_all(std::slice::from_ref(&msg.printable.0))?;
  wrt.write_u32::<BigEndian>(msg.execution_price)?;
  assert_eq!(wrt.position() - start_pos, ORDER_EXECUTED_WITH_PRICE_SIZE as u64);
  Ok(())
}
pub fn write_order_cancel(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, order_reference_number: u64, cancelled_shares: u32) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderCancel::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(order_reference_number)?;
  wrt.write_u32::<BigEndian>(cancelled_shares)?;
  assert_eq!(wrt.position() - start_pos, ORDER_CANCEL_SIZE as u64);
  Ok(())
}

pub fn write_order_cancel_struct(wrt: &mut Cursor<&mut [u8]>, msg: OrderCancel) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderCancel::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.order_reference_number)?;
  wrt.write_u32::<BigEndian>(msg.cancelled_shares)?;
  assert_eq!(wrt.position() - start_pos, ORDER_CANCEL_SIZE as u64);
  Ok(())
}
pub fn write_order_delete(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, order_reference_number: u64) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderDelete::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(order_reference_number)?;
  assert_eq!(wrt.position() - start_pos, ORDER_DELETE_SIZE as u64);
  Ok(())
}

pub fn write_order_delete_struct(wrt: &mut Cursor<&mut [u8]>, msg: OrderDelete) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderDelete::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.order_reference_number)?;
  assert_eq!(wrt.position() - start_pos, ORDER_DELETE_SIZE as u64);
  Ok(())
}
pub fn write_order_replace(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, original_order_reference_number: u64, new_order_reference_number: u64, shares: u32, price: u32) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderReplace::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(original_order_reference_number)?;
  wrt.write_u64::<BigEndian>(new_order_reference_number)?;
  wrt.write_u32::<BigEndian>(shares)?;
  wrt.write_u32::<BigEndian>(price)?;
  assert_eq!(wrt.position() - start_pos, ORDER_REPLACE_SIZE as u64);
  Ok(())
}

pub fn write_order_replace_struct(wrt: &mut Cursor<&mut [u8]>, msg: OrderReplace) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = OrderReplace::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.original_order_reference_number)?;
  wrt.write_u64::<BigEndian>(msg.new_order_reference_number)?;
  wrt.write_u32::<BigEndian>(msg.shares)?;
  wrt.write_u32::<BigEndian>(msg.price)?;
  assert_eq!(wrt.position() - start_pos, ORDER_REPLACE_SIZE as u64);
  Ok(())
}
pub fn write_trade(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, order_reference_number: u64, buy_sell_indicator: eBuySellIndicator, shares: u32, stock: [u8;8], price: u32, match_number: u64) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = Trade::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(order_reference_number)?;
  wrt.write_all(std::slice::from_ref(&buy_sell_indicator.0))?;
  wrt.write_u32::<BigEndian>(shares)?;
  wrt.write_all(&stock[..8])?;
  wrt.write_u32::<BigEndian>(price)?;
  wrt.write_u64::<BigEndian>(match_number)?;
  assert_eq!(wrt.position() - start_pos, TRADE_SIZE as u64);
  Ok(())
}

pub fn write_trade_struct(wrt: &mut Cursor<&mut [u8]>, msg: Trade) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = Trade::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.order_reference_number)?;
  wrt.write_all(std::slice::from_ref(&msg.buy_sell_indicator.0))?;
  wrt.write_u32::<BigEndian>(msg.shares)?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_u32::<BigEndian>(msg.price)?;
  wrt.write_u64::<BigEndian>(msg.match_number)?;
  assert_eq!(wrt.position() - start_pos, TRADE_SIZE as u64);
  Ok(())
}
pub fn write_cross_trade(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, shares: u64, stock: [u8;8], cross_price: u32, match_number: u64, cross_type: eCrossType) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = CrossTrade::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(shares)?;
  wrt.write_all(&stock[..8])?;
  wrt.write_u32::<BigEndian>(cross_price)?;
  wrt.write_u64::<BigEndian>(match_number)?;
  wrt.write_all(std::slice::from_ref(&cross_type.0))?;
  assert_eq!(wrt.position() - start_pos, CROSS_TRADE_SIZE as u64);
  Ok(())
}

pub fn write_cross_trade_struct(wrt: &mut Cursor<&mut [u8]>, msg: CrossTrade) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = CrossTrade::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.shares)?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_u32::<BigEndian>(msg.cross_price)?;
  wrt.write_u64::<BigEndian>(msg.match_number)?;
  wrt.write_all(std::slice::from_ref(&msg.cross_type.0))?;
  assert_eq!(wrt.position() - start_pos, CROSS_TRADE_SIZE as u64);
  Ok(())
}
pub fn write_broken_trade(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, match_number: u64) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = BrokenTrade::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(match_number)?;
  assert_eq!(wrt.position() - start_pos, BROKEN_TRADE_SIZE as u64);
  Ok(())
}

pub fn write_broken_trade_struct(wrt: &mut Cursor<&mut [u8]>, msg: BrokenTrade) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = BrokenTrade::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.match_number)?;
  assert_eq!(wrt.position() - start_pos, BROKEN_TRADE_SIZE as u64);
  Ok(())
}
pub fn write_net_order_imbalance_indicator(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, paired_shares: u64, imbalance_shares: u64, imbalance_direction: eImbalanceDirection, stock: [u8;8], far_price: u32, near_price: u32, current_reference_price: u32, cross_type: eCrossType, price_variation_indicator: ePriceVariationIndicator) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = NetOrderImbalanceIndicator::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(paired_shares)?;
  wrt.write_u64::<BigEndian>(imbalance_shares)?;
  wrt.write_all(std::slice::from_ref(&imbalance_direction.0))?;
  wrt.write_all(&stock[..8])?;
  wrt.write_u32::<BigEndian>(far_price)?;
  wrt.write_u32::<BigEndian>(near_price)?;
  wrt.write_u32::<BigEndian>(current_reference_price)?;
  wrt.write_all(std::slice::from_ref(&cross_type.0))?;
  wrt.write_all(std::slice::from_ref(&price_variation_indicator.0))?;
  assert_eq!(wrt.position() - start_pos, NET_ORDER_IMBALANCE_INDICATOR_SIZE as u64);
  Ok(())
}

pub fn write_net_order_imbalance_indicator_struct(wrt: &mut Cursor<&mut [u8]>, msg: NetOrderImbalanceIndicator) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = NetOrderImbalanceIndicator::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_u64::<BigEndian>(msg.paired_shares)?;
  wrt.write_u64::<BigEndian>(msg.imbalance_shares)?;
  wrt.write_all(std::slice::from_ref(&msg.imbalance_direction.0))?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_u32::<BigEndian>(msg.far_price)?;
  wrt.write_u32::<BigEndian>(msg.near_price)?;
  wrt.write_u32::<BigEndian>(msg.current_reference_price)?;
  wrt.write_all(std::slice::from_ref(&msg.cross_type.0))?;
  wrt.write_all(std::slice::from_ref(&msg.price_variation_indicator.0))?;
  assert_eq!(wrt.position() - start_pos, NET_ORDER_IMBALANCE_INDICATOR_SIZE as u64);
  Ok(())
}
pub fn write_retail_price_improvement_indicator(wrt: &mut Cursor<&mut [u8]>, stock_locate: u16, tracking_number: u16, timestamp: u64, stock: [u8;8], interest_flag: eInterestFlag) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = RetailPriceImprovementIndicator::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(stock_locate)?;
  wrt.write_u16::<BigEndian>(tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(timestamp)[2..])?;
  wrt.write_all(&stock[..8])?;
  wrt.write_all(std::slice::from_ref(&interest_flag.0))?;
  assert_eq!(wrt.position() - start_pos, RETAIL_PRICE_IMPROVEMENT_INDICATOR_SIZE as u64);
  Ok(())
}

pub fn write_retail_price_improvement_indicator_struct(wrt: &mut Cursor<&mut [u8]>, msg: RetailPriceImprovementIndicator) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = RetailPriceImprovementIndicator::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_u16::<BigEndian>(msg.stock_locate)?;
  wrt.write_u16::<BigEndian>(msg.tracking_number)?;
  wrt.write_all(&u64::to_be_bytes(msg.timestamp)[2..])?;
  wrt.write_all(&msg.stock[..8])?;
  wrt.write_all(std::slice::from_ref(&msg.interest_flag.0))?;
  assert_eq!(wrt.position() - start_pos, RETAIL_PRICE_IMPROVEMENT_INDICATOR_SIZE as u64);
  Ok(())
}
pub fn write_end_of_snapshot(wrt: &mut Cursor<&mut [u8]>, sequence_number: [u8;20]) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = EndOfSnapshot::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_all(&sequence_number[..20])?;
  assert_eq!(wrt.position() - start_pos, END_OF_SNAPSHOT_SIZE as u64);
  Ok(())
}

pub fn write_end_of_snapshot_struct(wrt: &mut Cursor<&mut [u8]>, msg: EndOfSnapshot) -> std::io::Result<()> {
  let start_pos = wrt.position();
  let tipe = EndOfSnapshot::TYPE;
  wrt.write_all(std::slice::from_ref(&tipe))?;
  wrt.write_all(&msg.sequence_number[..20])?;
  assert_eq!(wrt.position() - start_pos, END_OF_SNAPSHOT_SIZE as u64);
  Ok(())
}

pub struct Dumper {}
impl ItchHandler for Dumper {
  fn on_system_event(&mut self, msg: SystemEvent) { println!("{}", msg); }
  fn on_stock_directory(&mut self, msg: StockDirectory) { println!("{}", msg); }
  fn on_stock_trading_action(&mut self, msg: StockTradingAction) { println!("{}", msg); }
  fn on_reg_sho_restriction(&mut self, msg: RegShoRestriction) { println!("{}", msg); }
  fn on_market_participant_position(&mut self, msg: MarketParticipantPosition) { println!("{}", msg); }
  fn on_mwcb_decline_level(&mut self, msg: MwcbDeclineLevel) { println!("{}", msg); }
  fn on_mwcb_status(&mut self, msg: MwcbStatus) { println!("{}", msg); }
  fn on_ipo_quoting_period_update(&mut self, msg: IpoQuotingPeriodUpdate) { println!("{}", msg); }
  fn on_luld_auction_collar(&mut self, msg: LuldAuctionCollar) { println!("{}", msg); }
  fn on_operational_halt(&mut self, msg: OperationalHalt) { println!("{}", msg); }
  fn on_add_order(&mut self, msg: AddOrder) { println!("{}", msg); }
  fn on_add_order_with_mpid(&mut self, msg: AddOrderWithMpid) { println!("{}", msg); }
  fn on_order_executed(&mut self, msg: OrderExecuted) { println!("{}", msg); }
  fn on_order_executed_with_price(&mut self, msg: OrderExecutedWithPrice) { println!("{}", msg); }
  fn on_order_cancel(&mut self, msg: OrderCancel) { println!("{}", msg); }
  fn on_order_delete(&mut self, msg: OrderDelete) { println!("{}", msg); }
  fn on_order_replace(&mut self, msg: OrderReplace) { println!("{}", msg); }
  fn on_trade(&mut self, msg: Trade) { println!("{}", msg); }
  fn on_cross_trade(&mut self, msg: CrossTrade) { println!("{}", msg); }
  fn on_broken_trade(&mut self, msg: BrokenTrade) { println!("{}", msg); }
  fn on_net_order_imbalance_indicator(&mut self, msg: NetOrderImbalanceIndicator) { println!("{}", msg); }
  fn on_retail_price_improvement_indicator(&mut self, msg: RetailPriceImprovementIndicator) { println!("{}", msg); }
  fn on_end_of_snapshot(&mut self, msg: EndOfSnapshot) { println!("{}", msg); }
}
