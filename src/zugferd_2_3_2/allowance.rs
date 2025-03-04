#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Allowance {
    /// Bonus for works ahead of schedule
    BonusForWorksAheadSchedule,
    /// Other bonus
    OtherBonus,
    /// Manufacturer’s consumer discount
    ManufacturerSConsumerDiscount,
    /// Due to military status
    DueToMilitaryStatus,
    /// Due to work accident
    DueToWorkAccident,
    /// Special agreement
    SpecialAgreement,
    /// Production error discount
    ProductionErrorDiscount,
    /// New outlet discount
    NewOutletDiscount,
    /// Sample discount
    SampleDiscount,
    /// End-of-range discount
    EndRangeDiscount,
    /// Incoterm discount
    IncotermDiscount,
    /// Point of sales threshold allowance
    PointSalesThresholdAllowance,
    /// Material surcharge/deduction
    MaterialSurchargeDeduction,
    /// Discount
    Discount,
    /// Special rebate
    SpecialRebate,
    /// Fixed long term
    FixedLongTerm,
    /// Temporary
    Temporary,
    /// Standard
    Standard,
    /// Yearly turnover
    YearlyTurnover,
}

impl crate::Code for Allowance {
    fn code(&self) -> &str {
        match self {
            Allowance::BonusForWorksAheadSchedule => "41",
            Allowance::OtherBonus => "42",
            Allowance::ManufacturerSConsumerDiscount => "60",
            Allowance::DueToMilitaryStatus => "62",
            Allowance::DueToWorkAccident => "63",
            Allowance::SpecialAgreement => "64",
            Allowance::ProductionErrorDiscount => "65",
            Allowance::NewOutletDiscount => "66",
            Allowance::SampleDiscount => "67",
            Allowance::EndRangeDiscount => "68",
            Allowance::IncotermDiscount => "70",
            Allowance::PointSalesThresholdAllowance => "71",
            Allowance::MaterialSurchargeDeduction => "88",
            Allowance::Discount => "95",
            Allowance::SpecialRebate => "100",
            Allowance::FixedLongTerm => "102",
            Allowance::Temporary => "103",
            Allowance::Standard => "104",
            Allowance::YearlyTurnover => "105",
        }
    }
}

impl crate::Description for Allowance {
    fn description(&self) -> &str {
        match self {
            Allowance::BonusForWorksAheadSchedule => "Bonus for works ahead of schedule",
            Allowance::OtherBonus => "Other bonus",
            Allowance::ManufacturerSConsumerDiscount => "Manufacturer’s consumer discount",
            Allowance::DueToMilitaryStatus => "Due to military status",
            Allowance::DueToWorkAccident => "Due to work accident",
            Allowance::SpecialAgreement => "Special agreement",
            Allowance::ProductionErrorDiscount => "Production error discount",
            Allowance::NewOutletDiscount => "New outlet discount",
            Allowance::SampleDiscount => "Sample discount",
            Allowance::EndRangeDiscount => "End-of-range discount",
            Allowance::IncotermDiscount => "Incoterm discount",
            Allowance::PointSalesThresholdAllowance => "Point of sales threshold allowance",
            Allowance::MaterialSurchargeDeduction => "Material surcharge/deduction",
            Allowance::Discount => "Discount",
            Allowance::SpecialRebate => "Special rebate",
            Allowance::FixedLongTerm => "Fixed long term",
            Allowance::Temporary => "Temporary",
            Allowance::Standard => "Standard",
            Allowance::YearlyTurnover => "Yearly turnover",
        }
    }
}

impl crate::FromCode for Allowance {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "41" => Some(Allowance::BonusForWorksAheadSchedule),
            "42" => Some(Allowance::OtherBonus),
            "60" => Some(Allowance::ManufacturerSConsumerDiscount),
            "62" => Some(Allowance::DueToMilitaryStatus),
            "63" => Some(Allowance::DueToWorkAccident),
            "64" => Some(Allowance::SpecialAgreement),
            "65" => Some(Allowance::ProductionErrorDiscount),
            "66" => Some(Allowance::NewOutletDiscount),
            "67" => Some(Allowance::SampleDiscount),
            "68" => Some(Allowance::EndRangeDiscount),
            "70" => Some(Allowance::IncotermDiscount),
            "71" => Some(Allowance::PointSalesThresholdAllowance),
            "88" => Some(Allowance::MaterialSurchargeDeduction),
            "95" => Some(Allowance::Discount),
            "100" => Some(Allowance::SpecialRebate),
            "102" => Some(Allowance::FixedLongTerm),
            "103" => Some(Allowance::Temporary),
            "104" => Some(Allowance::Standard),
            "105" => Some(Allowance::YearlyTurnover),
            _ => None,
        }
    }
}
