#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Enum5305 {
    StandardRate,
    ZeroRatedGoods,
    ExemptFromTax,
    VatReverseCharge,
    VatExemptForEeaIntraCommunitySupplyGoodsAndServices,
    FreeExportItemTaxNotCharged,
    ServiceOutsideScopeTax,
    CanaryIslandsGeneralIndirectTax,
    TaxForProductionServicesAndImportationInCeutaAndMelilla,
}

impl crate::Code for Enum5305 {
    fn code(&self) -> &str {
        match self {
            Enum5305::StandardRate => "S",
            Enum5305::ZeroRatedGoods => "Z",
            Enum5305::ExemptFromTax => "E",
            Enum5305::VatReverseCharge => "AE",
            Enum5305::VatExemptForEeaIntraCommunitySupplyGoodsAndServices => "K",
            Enum5305::FreeExportItemTaxNotCharged => "G",
            Enum5305::ServiceOutsideScopeTax => "O",
            Enum5305::CanaryIslandsGeneralIndirectTax => "L",
            Enum5305::TaxForProductionServicesAndImportationInCeutaAndMelilla => "M",
        }
    }
}

impl crate::Description for Enum5305 {
    fn description(&self) -> &str {
        match self {
            Enum5305::StandardRate => "Standard rate",
            Enum5305::ZeroRatedGoods => "Zero rated goods",
            Enum5305::ExemptFromTax => "Exempt from tax",
            Enum5305::VatReverseCharge => "VAT Reverse charge",
            Enum5305::VatExemptForEeaIntraCommunitySupplyGoodsAndServices => {
                "VAT exempt for EEA intra-community supply of goods and services"
            }
            Enum5305::FreeExportItemTaxNotCharged => "Free export item, tax not charged",
            Enum5305::ServiceOutsideScopeTax => "Service outside scope of tax",
            Enum5305::CanaryIslandsGeneralIndirectTax => "Canary Islands general indirect tax",
            Enum5305::TaxForProductionServicesAndImportationInCeutaAndMelilla => {
                "Tax for production, services and importation in Ceuta and Melilla"
            }
        }
    }
}

impl crate::FromCode for Enum5305 {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "S" => Some(Enum5305::StandardRate),
            "Z" => Some(Enum5305::ZeroRatedGoods),
            "E" => Some(Enum5305::ExemptFromTax),
            "AE" => Some(Enum5305::VatReverseCharge),
            "K" => Some(Enum5305::VatExemptForEeaIntraCommunitySupplyGoodsAndServices),
            "G" => Some(Enum5305::FreeExportItemTaxNotCharged),
            "O" => Some(Enum5305::ServiceOutsideScopeTax),
            "L" => Some(Enum5305::CanaryIslandsGeneralIndirectTax),
            "M" => Some(Enum5305::TaxForProductionServicesAndImportationInCeutaAndMelilla),
            _ => None,
        }
    }
}
