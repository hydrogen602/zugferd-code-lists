#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VATCAT {
    /// Petroleum tax
    ///
    /// A tax levied on the volume of petroleum being transacted.
    PetroleumTax,
    /// Provisional countervailing duty cash
    ///
    /// Countervailing duty paid in cash prior to a formal finding of subsidization by Customs.
    ProvisionalCountervailingDutyCash,
    /// Provisional countervailing duty bond
    ///
    /// Countervailing duty paid by posting a bond during an investigation period prior to a formal decision on subsidization by Customs.
    ProvisionalCountervailingDutyBond,
    /// Tobacco tax
    ///
    /// A tax levied on tobacco products.
    TobaccoTax,
    /// Energy fee
    ///
    /// General fee or tax for the use of energy.
    EnergyFee,
    /// Coffee tax
    ///
    /// A tax levied specifically on coffee products.
    CoffeeTax,
    /// Harmonised sales tax, Canadian
    ///
    /// A harmonized sales tax consisting of a goods and service tax, a Canadian provincial sales tax and, as applicable, a Quebec sales tax which is recoverable.
    HarmonisedSalesTaxCanadian,
    /// Quebec sales tax
    ///
    /// A sales tax charged within the Canadian province of Quebec which is recoverable.
    QuebecSalesTax,
    /// Canadian provincial sales tax
    ///
    /// A sales tax charged within Canadian provinces which is non-recoverable.
    CanadianProvincialSalesTax,
    /// Tax on replacement part
    ///
    /// A tax levied on a replacement part, where the original part is returned.
    TaxOnReplacementPart,
    /// Mineral oil tax
    ///
    /// Tax that is levied specifically on products containing mineral oil.
    MineralOilTax,
    /// Special tax
    ///
    /// To indicate a special type of tax.
    SpecialTax,
    /// Insurance tax
    ///
    /// A tax levied specifically on insurances.
    InsuranceTax,
    /// Provincial Cannabis Tax
    ///
    /// A tax levied on Cannabis products
    ProvincialCannabisTax,
    /// Outstanding duty interest
    ///
    /// Fee levied due to outstanding duties to be paid
    OutstandingDutyInterest,
    /// Anti-dumping duty
    ///
    /// Duty applied to goods ruled to have been dumped in an import market at a price lower than that in the exporter's domestic market.
    AntiDumpingDuty,
    /// Stamp duty (Imposta di Bollo)
    ///
    /// Tax required in Italy, which may be fixed or graduated in various circumstances (e.g. VAT exempt documents or bank receipts).
    StampDutyImpostaDiBollo,
    /// Agricultural levy
    ///
    /// Levy imposed on agricultural products where there is a difference between the selling price between trading countries.
    AgriculturalLevy,
    /// Car tax
    ///
    /// A tax that is levied on the value of the automobile.
    CarTax,
    /// Paper consortium tax (Italy)
    ///
    /// Italian Paper consortium tax.
    PaperConsortiumTaxItaly,
    /// Commodity specific tax
    ///
    /// Tax related to a specified commodity, e.g. illuminants, salts.
    CommoditySpecificTax,
    /// Customs duty
    ///
    /// Duties laid down in the Customs tariff, to which goods are liable on entering or leaving the Customs territory (CCC).
    CustomsDuty,
    /// Countervailing duty
    ///
    /// A duty on imported goods applied for compensate for subsidies granted to those goods in the exporting country.
    CountervailingDuty,
    /// Environmental tax
    ///
    /// Tax assessed for funding or assuring environmental protection or clean-up.
    EnvironmentalTax,
    /// Excise duty
    ///
    /// Customs or fiscal authorities code to identify a specific or ad valorem levy on a specific commodity, applied either domestically or at time of importation.
    ExciseDuty,
    /// Agricultural export rebate
    ///
    /// Monetary rebate given to the seller in certain circumstances when agricultural products are exported.
    AgriculturalExportRebate,
    /// Federal excise tax
    ///
    /// Tax levied by the federal government on the manufacture of specific items.
    FederalExciseTax,
    /// Free
    ///
    /// No tax levied.
    Free,
    /// General construction tax
    ///
    /// General tax for construction.
    GeneralConstructionTax,
    /// Goods and services tax
    ///
    /// Tax levied on the final consumption of goods and services throughout the production and distribution chain.
    GoodsAndServicesTax,
    /// Illuminants tax
    ///
    /// Tax of illuminants.
    IlluminantsTax,
    /// Import tax
    ///
    /// Tax assessed on imports.
    ImportTax,
    /// Individual tax
    ///
    /// A tax levied based on an individual's ability to pay.
    IndividualTax,
    /// Business license fee
    ///
    /// Government assessed charge for permit to do business.
    BusinessLicenseFee,
    /// Local construction tax
    ///
    /// Local tax for construction.
    LocalConstructionTax,
    /// Light dues payable
    ///
    /// Fee levied on a vessel to pay for port navigation lights.
    LightDuesPayable,
    /// Local sales tax
    ///
    /// Assessment charges on sale of goods or services by city, borough country or other taxing authorities below state or provincial level.
    LocalSalesTax,
    /// Lust tax
    ///
    /// Tax imposed for clean-up of leaky underground storage tanks.
    LustTax,
    /// Monetary compensatory amount
    ///
    /// Levy on Common Agricultural Policy (European Union) goods used to compensate for fluctuating currencies between member states.
    MonetaryCompensatoryAmount,
    /// Miscellaneous cash deposit
    ///
    /// Duty paid and held on deposit, by Customs, during an investigation period prior to a final decision being made on any aspect related to imported goods (except valuation) by Customs.
    MiscellaneousCashDeposit,
    /// Other taxes
    ///
    /// Unspecified, miscellaneous tax charges.
    OtherTaxes,
    /// Provisional duty bond
    ///
    /// Anti-dumping duty paid by posting a bond during an investigation period prior to a formal decision on dumping by Customs.
    ProvisionalDutyBond,
    /// Provisional duty cash
    ///
    /// Anti-dumping duty paid in cash prior to a formal finding of dumping by Customs.
    ProvisionalDutyCash,
    /// Preference duty
    ///
    /// Duties laid down in the Customs tariff, to which goods are liable on entering or leaving the Customs territory falling under a preferential regime such as Generalised System of Preferences (GSP).
    PreferenceDuty,
    /// Special construction tax
    ///
    /// Special tax for construction.
    SpecialConstructionTax,
    /// Shifted social securities
    ///
    /// Social securities share of the invoice amount to be paid directly to the social securities collector.
    ShiftedSocialSecurities,
    /// State/provincial sales tax
    ///
    /// All applicable sale taxes by authorities at the state or provincial level, below national level.
    StateProvincialSalesTax,
    /// Suspended duty
    ///
    /// Duty suspended or deferred from payment.
    SuspendedDuty,
    /// Surtax
    ///
    /// A tax or duty applied on and in addition to existing duties and taxes.
    Surtax,
    /// Shifted wage tax
    ///
    /// Wage tax share of the invoice amount to be paid directly to the tax collector(s office).
    ShiftedWageTax,
    /// Alcohol mark tax
    ///
    /// A tax levied based on the type of alcohol being obtained.
    AlcoholMarkTax,
    /// Total
    ///
    /// The summary amount of all taxes.
    Total,
    /// Turnover tax
    ///
    /// Tax levied on the total sales/turnover of a corporation.
    TurnoverTax,
    /// Tonnage taxes
    ///
    /// Tax levied based on the vessel's net tonnage.
    TonnageTaxes,
    /// Valuation deposit
    ///
    /// Duty paid and held on deposit, by Customs, during an investigation period prior to a formal decision on valuation of the goods being made.
    ValuationDeposit,
    /// Value added tax
    ///
    /// A tax on domestic or imported goods applied to the value added at each stage in the production/distribution cycle.
    ValueAddedTax,
}

impl std::fmt::Display for VATCAT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for VATCAT {
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
    }
}

impl crate::Code for VATCAT {
    fn code(self) -> &'static str {
        match self {
            VATCAT::PetroleumTax => "AAA",
            VATCAT::ProvisionalCountervailingDutyCash => "AAB",
            VATCAT::ProvisionalCountervailingDutyBond => "AAC",
            VATCAT::TobaccoTax => "AAD",
            VATCAT::EnergyFee => "AAE",
            VATCAT::CoffeeTax => "AAF",
            VATCAT::HarmonisedSalesTaxCanadian => "AAG",
            VATCAT::QuebecSalesTax => "AAH",
            VATCAT::CanadianProvincialSalesTax => "AAI",
            VATCAT::TaxOnReplacementPart => "AAJ",
            VATCAT::MineralOilTax => "AAK",
            VATCAT::SpecialTax => "AAL",
            VATCAT::InsuranceTax => "AAM",
            VATCAT::ProvincialCannabisTax => "AAO",
            VATCAT::OutstandingDutyInterest => "AAP",
            VATCAT::AntiDumpingDuty => "ADD",
            VATCAT::StampDutyImpostaDiBollo => "BOL",
            VATCAT::AgriculturalLevy => "CAP",
            VATCAT::CarTax => "CAR",
            VATCAT::PaperConsortiumTaxItaly => "COC",
            VATCAT::CommoditySpecificTax => "CST",
            VATCAT::CustomsDuty => "CUD",
            VATCAT::CountervailingDuty => "CVD",
            VATCAT::EnvironmentalTax => "ENV",
            VATCAT::ExciseDuty => "EXC",
            VATCAT::AgriculturalExportRebate => "EXP",
            VATCAT::FederalExciseTax => "FET",
            VATCAT::Free => "FRE",
            VATCAT::GeneralConstructionTax => "GCN",
            VATCAT::GoodsAndServicesTax => "GST",
            VATCAT::IlluminantsTax => "ILL",
            VATCAT::ImportTax => "IMP",
            VATCAT::IndividualTax => "IND",
            VATCAT::BusinessLicenseFee => "LAC",
            VATCAT::LocalConstructionTax => "LCN",
            VATCAT::LightDuesPayable => "LDP",
            VATCAT::LocalSalesTax => "LOC",
            VATCAT::LustTax => "LST",
            VATCAT::MonetaryCompensatoryAmount => "MCA",
            VATCAT::MiscellaneousCashDeposit => "MCD",
            VATCAT::OtherTaxes => "OTH",
            VATCAT::ProvisionalDutyBond => "PDB",
            VATCAT::ProvisionalDutyCash => "PDC",
            VATCAT::PreferenceDuty => "PRF",
            VATCAT::SpecialConstructionTax => "SCN",
            VATCAT::ShiftedSocialSecurities => "SSS",
            VATCAT::StateProvincialSalesTax => "STT",
            VATCAT::SuspendedDuty => "SUP",
            VATCAT::Surtax => "SUR",
            VATCAT::ShiftedWageTax => "SWT",
            VATCAT::AlcoholMarkTax => "TAC",
            VATCAT::Total => "TOT",
            VATCAT::TurnoverTax => "TOX",
            VATCAT::TonnageTaxes => "TTA",
            VATCAT::ValuationDeposit => "VAD",
            VATCAT::ValueAddedTax => "VAT",
        }
    }
}

impl crate::Description for VATCAT {
    fn description(self) -> &'static str {
        match self {
            VATCAT::PetroleumTax => "Petroleum tax",
            VATCAT::ProvisionalCountervailingDutyCash => "Provisional countervailing duty cash",
            VATCAT::ProvisionalCountervailingDutyBond => "Provisional countervailing duty bond",
            VATCAT::TobaccoTax => "Tobacco tax",
            VATCAT::EnergyFee => "Energy fee",
            VATCAT::CoffeeTax => "Coffee tax",
            VATCAT::HarmonisedSalesTaxCanadian => "Harmonised sales tax, Canadian",
            VATCAT::QuebecSalesTax => "Quebec sales tax",
            VATCAT::CanadianProvincialSalesTax => "Canadian provincial sales tax",
            VATCAT::TaxOnReplacementPart => "Tax on replacement part",
            VATCAT::MineralOilTax => "Mineral oil tax",
            VATCAT::SpecialTax => "Special tax",
            VATCAT::InsuranceTax => "Insurance tax",
            VATCAT::ProvincialCannabisTax => "Provincial Cannabis Tax",
            VATCAT::OutstandingDutyInterest => "Outstanding duty interest",
            VATCAT::AntiDumpingDuty => "Anti-dumping duty",
            VATCAT::StampDutyImpostaDiBollo => "Stamp duty (Imposta di Bollo)",
            VATCAT::AgriculturalLevy => "Agricultural levy",
            VATCAT::CarTax => "Car tax",
            VATCAT::PaperConsortiumTaxItaly => "Paper consortium tax (Italy)",
            VATCAT::CommoditySpecificTax => "Commodity specific tax",
            VATCAT::CustomsDuty => "Customs duty",
            VATCAT::CountervailingDuty => "Countervailing duty",
            VATCAT::EnvironmentalTax => "Environmental tax",
            VATCAT::ExciseDuty => "Excise duty",
            VATCAT::AgriculturalExportRebate => "Agricultural export rebate",
            VATCAT::FederalExciseTax => "Federal excise tax",
            VATCAT::Free => "Free",
            VATCAT::GeneralConstructionTax => "General construction tax",
            VATCAT::GoodsAndServicesTax => "Goods and services tax",
            VATCAT::IlluminantsTax => "Illuminants tax",
            VATCAT::ImportTax => "Import tax",
            VATCAT::IndividualTax => "Individual tax",
            VATCAT::BusinessLicenseFee => "Business license fee",
            VATCAT::LocalConstructionTax => "Local construction tax",
            VATCAT::LightDuesPayable => "Light dues payable",
            VATCAT::LocalSalesTax => "Local sales tax",
            VATCAT::LustTax => "Lust tax",
            VATCAT::MonetaryCompensatoryAmount => "Monetary compensatory amount",
            VATCAT::MiscellaneousCashDeposit => "Miscellaneous cash deposit",
            VATCAT::OtherTaxes => "Other taxes",
            VATCAT::ProvisionalDutyBond => "Provisional duty bond",
            VATCAT::ProvisionalDutyCash => "Provisional duty cash",
            VATCAT::PreferenceDuty => "Preference duty",
            VATCAT::SpecialConstructionTax => "Special construction tax",
            VATCAT::ShiftedSocialSecurities => "Shifted social securities",
            VATCAT::StateProvincialSalesTax => "State/provincial sales tax",
            VATCAT::SuspendedDuty => "Suspended duty",
            VATCAT::Surtax => "Surtax",
            VATCAT::ShiftedWageTax => "Shifted wage tax",
            VATCAT::AlcoholMarkTax => "Alcohol mark tax",
            VATCAT::Total => "Total",
            VATCAT::TurnoverTax => "Turnover tax",
            VATCAT::TonnageTaxes => "Tonnage taxes",
            VATCAT::ValuationDeposit => "Valuation deposit",
            VATCAT::ValueAddedTax => "Value added tax",
        }
    }
}

impl crate::FromCode for VATCAT {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "AAA" => Some(VATCAT::PetroleumTax),
            "AAB" => Some(VATCAT::ProvisionalCountervailingDutyCash),
            "AAC" => Some(VATCAT::ProvisionalCountervailingDutyBond),
            "AAD" => Some(VATCAT::TobaccoTax),
            "AAE" => Some(VATCAT::EnergyFee),
            "AAF" => Some(VATCAT::CoffeeTax),
            "AAG" => Some(VATCAT::HarmonisedSalesTaxCanadian),
            "AAH" => Some(VATCAT::QuebecSalesTax),
            "AAI" => Some(VATCAT::CanadianProvincialSalesTax),
            "AAJ" => Some(VATCAT::TaxOnReplacementPart),
            "AAK" => Some(VATCAT::MineralOilTax),
            "AAL" => Some(VATCAT::SpecialTax),
            "AAM" => Some(VATCAT::InsuranceTax),
            "AAO" => Some(VATCAT::ProvincialCannabisTax),
            "AAP" => Some(VATCAT::OutstandingDutyInterest),
            "ADD" => Some(VATCAT::AntiDumpingDuty),
            "BOL" => Some(VATCAT::StampDutyImpostaDiBollo),
            "CAP" => Some(VATCAT::AgriculturalLevy),
            "CAR" => Some(VATCAT::CarTax),
            "COC" => Some(VATCAT::PaperConsortiumTaxItaly),
            "CST" => Some(VATCAT::CommoditySpecificTax),
            "CUD" => Some(VATCAT::CustomsDuty),
            "CVD" => Some(VATCAT::CountervailingDuty),
            "ENV" => Some(VATCAT::EnvironmentalTax),
            "EXC" => Some(VATCAT::ExciseDuty),
            "EXP" => Some(VATCAT::AgriculturalExportRebate),
            "FET" => Some(VATCAT::FederalExciseTax),
            "FRE" => Some(VATCAT::Free),
            "GCN" => Some(VATCAT::GeneralConstructionTax),
            "GST" => Some(VATCAT::GoodsAndServicesTax),
            "ILL" => Some(VATCAT::IlluminantsTax),
            "IMP" => Some(VATCAT::ImportTax),
            "IND" => Some(VATCAT::IndividualTax),
            "LAC" => Some(VATCAT::BusinessLicenseFee),
            "LCN" => Some(VATCAT::LocalConstructionTax),
            "LDP" => Some(VATCAT::LightDuesPayable),
            "LOC" => Some(VATCAT::LocalSalesTax),
            "LST" => Some(VATCAT::LustTax),
            "MCA" => Some(VATCAT::MonetaryCompensatoryAmount),
            "MCD" => Some(VATCAT::MiscellaneousCashDeposit),
            "OTH" => Some(VATCAT::OtherTaxes),
            "PDB" => Some(VATCAT::ProvisionalDutyBond),
            "PDC" => Some(VATCAT::ProvisionalDutyCash),
            "PRF" => Some(VATCAT::PreferenceDuty),
            "SCN" => Some(VATCAT::SpecialConstructionTax),
            "SSS" => Some(VATCAT::ShiftedSocialSecurities),
            "STT" => Some(VATCAT::StateProvincialSalesTax),
            "SUP" => Some(VATCAT::SuspendedDuty),
            "SUR" => Some(VATCAT::Surtax),
            "SWT" => Some(VATCAT::ShiftedWageTax),
            "TAC" => Some(VATCAT::AlcoholMarkTax),
            "TOT" => Some(VATCAT::Total),
            "TOX" => Some(VATCAT::TurnoverTax),
            "TTA" => Some(VATCAT::TonnageTaxes),
            "VAD" => Some(VATCAT::ValuationDeposit),
            "VAT" => Some(VATCAT::ValueAddedTax),
            _ => None,
        }
    }
}
