
export enum Enum5305 {
    /**
    * Standard rate
    */
    StandardRate = "S",
    /**
    * Zero rated goods
    */
    ZeroRatedGoods = "Z",
    /**
    * Exempt from tax
    */
    ExemptFromTax = "E",
    /**
    * VAT Reverse charge
    *
    * VAT reverse charge
    */
    VatReverseCharge = "AE",
    /**
    * VAT exempt for EEA intra-community supply of goods and services
    *
    * VAT exempt for intra community supply of goods
    */
    VatExemptForEeaIntraCommunitySupplyGoodsAndServices = "K",
    /**
    * Free export item, tax not charged
    */
    FreeExportItemTaxNotCharged = "G",
    /**
    * Service outside scope of tax
    *
    * Services outside scope of tax
    */
    ServiceOutsideScopeTax = "O",
    /**
    * Canary Islands general indirect tax
    *
    * Canary Islands General Indirect Tax
    */
    CanaryIslandsGeneralIndirectTax = "L",
    /**
    * Tax for production, services and importation in Ceuta and Melilla
    *
    * Liable for IPSI
    */
    TaxForProductionServicesAndImportationInCeutaAndMelilla = "M",
}

export function description(value: Enum5305): string {
    switch (value) {
        case Enum5305.StandardRate: return "Standard rate";
        case Enum5305.ZeroRatedGoods: return "Zero rated goods";
        case Enum5305.ExemptFromTax: return "Exempt from tax";
        case Enum5305.VatReverseCharge: return "VAT Reverse charge";
        case Enum5305.VatExemptForEeaIntraCommunitySupplyGoodsAndServices: return "VAT exempt for EEA intra-community supply of goods and services";
        case Enum5305.FreeExportItemTaxNotCharged: return "Free export item, tax not charged";
        case Enum5305.ServiceOutsideScopeTax: return "Service outside scope of tax";
        case Enum5305.CanaryIslandsGeneralIndirectTax: return "Canary Islands general indirect tax";
        case Enum5305.TaxForProductionServicesAndImportationInCeutaAndMelilla: return "Tax for production, services and importation in Ceuta and Melilla";
    }
}
