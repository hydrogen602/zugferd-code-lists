
export enum Allowance {
    /**
    * Bonus for works ahead of schedule
    */
    BonusForWorksAheadSchedule = "41",
    /**
    * Other bonus
    */
    OtherBonus = "42",
    /**
    * Manufacturer’s consumer discount
    */
    ManufacturerSConsumerDiscount = "60",
    /**
    * Due to military status
    */
    DueToMilitaryStatus = "62",
    /**
    * Due to work accident
    */
    DueToWorkAccident = "63",
    /**
    * Special agreement
    */
    SpecialAgreement = "64",
    /**
    * Production error discount
    */
    ProductionErrorDiscount = "65",
    /**
    * New outlet discount
    */
    NewOutletDiscount = "66",
    /**
    * Sample discount
    */
    SampleDiscount = "67",
    /**
    * End-of-range discount
    */
    EndRangeDiscount = "68",
    /**
    * Incoterm discount
    */
    IncotermDiscount = "70",
    /**
    * Point of sales threshold allowance
    */
    PointSalesThresholdAllowance = "71",
    /**
    * Material surcharge/deduction
    */
    MaterialSurchargeDeduction = "88",
    /**
    * Discount
    */
    Discount = "95",
    /**
    * Special rebate
    */
    SpecialRebate = "100",
    /**
    * Fixed long term
    */
    FixedLongTerm = "102",
    /**
    * Temporary
    */
    Temporary = "103",
    /**
    * Standard
    */
    Standard = "104",
    /**
    * Yearly turnover
    */
    YearlyTurnover = "105",
}

export function description(value: Allowance): string {
    switch (value) {
        case Allowance.BonusForWorksAheadSchedule: return "Bonus for works ahead of schedule";
        case Allowance.OtherBonus: return "Other bonus";
        case Allowance.ManufacturerSConsumerDiscount: return "Manufacturer’s consumer discount";
        case Allowance.DueToMilitaryStatus: return "Due to military status";
        case Allowance.DueToWorkAccident: return "Due to work accident";
        case Allowance.SpecialAgreement: return "Special agreement";
        case Allowance.ProductionErrorDiscount: return "Production error discount";
        case Allowance.NewOutletDiscount: return "New outlet discount";
        case Allowance.SampleDiscount: return "Sample discount";
        case Allowance.EndRangeDiscount: return "End-of-range discount";
        case Allowance.IncotermDiscount: return "Incoterm discount";
        case Allowance.PointSalesThresholdAllowance: return "Point of sales threshold allowance";
        case Allowance.MaterialSurchargeDeduction: return "Material surcharge/deduction";
        case Allowance.Discount: return "Discount";
        case Allowance.SpecialRebate: return "Special rebate";
        case Allowance.FixedLongTerm: return "Fixed long term";
        case Allowance.Temporary: return "Temporary";
        case Allowance.Standard: return "Standard";
        case Allowance.YearlyTurnover: return "Yearly turnover";
    }
}
