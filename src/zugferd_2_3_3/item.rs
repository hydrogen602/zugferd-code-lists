#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Item {
    /// Product version number
    ProductVersionNumber,
    /// Assembly
    Assembly,
    /// HIBC (Health Industry Bar Code)
    HibcHealthIndustryBarCode,
    /// Cold roll number
    ColdRollNumber,
    /// Hot roll number
    HotRollNumber,
    /// Slab number
    SlabNumber,
    /// Software revision number
    SoftwareRevisionNumber,
    /// UPC (Universal Product Code) Consumer package code (1-5-5)
    UpcUniversalProductCodeConsumerPackageCode155,
    /// UPC (Universal Product Code) Consumer package code (1-5-5-
    UpcUniversalProductCodeConsumerPackageCode155_Dup,
    /// Sample number
    SampleNumber,
    /// Pack number
    PackNumber,
    /// UPC (Universal Product Code) Shipping container code (1-2-
    UpcUniversalProductCodeShippingContainerCode12,
    /// UPC (Universal Product Code)/EAN (European article number)
    UpcUniversalProductCodeEanEuropeanArticleNumber,
    /// UPC (Universal Product Code) suffix
    UpcUniversalProductCodeSuffix,
    /// State label code
    StateLabelCode,
    /// Heat number
    HeatNumber,
    /// Coupon number
    CouponNumber,
    /// Resource number
    ResourceNumber,
    /// Work task number
    WorkTaskNumber,
    /// Price look up number
    PriceLookUpNumber,
    /// NSN (North Atlantic Treaty Organization Stock Number)
    NsnNorthAtlanticTreatyOrganizationStockNumber,
    /// Refined product code
    RefinedProductCode,
    /// Exhibit
    Exhibit,
    /// End item
    EndItem,
    /// Federal supply classification
    FederalSupplyClassification,
    /// Engineering data list
    EngineeringDataList,
    /// Milestone event number
    MilestoneEventNumber,
    /// Lot number
    LotNumber,
    /// National drug code 4-4-2 format
    NationalDrugCode442Format,
    /// National drug code 5-3-2 format
    NationalDrugCode532Format,
    /// National drug code 5-4-1 format
    NationalDrugCode541Format,
    /// National drug code 5-4-2 format
    NationalDrugCode542Format,
    /// National drug code
    NationalDrugCode,
    /// Part number
    PartNumber,
    /// Local Stock Number (LSN)
    LocalStockNumberLsn,
    /// Next higher assembly number
    NextHigherAssemblyNumber,
    /// Data category
    DataCategory,
    /// Control number
    ControlNumber,
    /// Special material identification code
    SpecialMaterialIdentificationCode,
    /// Locally assigned control number
    LocallyAssignedControlNumber,
    /// Buyer's colour
    BuyersColour,
    /// Buyer's part number
    BuyersPartNumber,
    /// Variable measure product code
    VariableMeasureProductCode,
    /// Financial phase
    FinancialPhase,
    /// Contract breakdown
    ContractBreakdown,
    /// Technical phase
    TechnicalPhase,
    /// Dye lot number
    DyeLotNumber,
    /// Daily statement of activities
    DailyStatementActivities,
    /// Periodical statement of activities within a bilaterally
    PeriodicalStatementActivitiesWithinABilaterally,
    /// Calendar week statement of activities
    CalendarWeekStatementActivities,
    /// Calendar month statement of activities
    CalendarMonthStatementActivities,
    /// Original equipment number
    OriginalEquipmentNumber,
    /// Industry commodity code
    IndustryCommodityCode,
    /// Commodity grouping
    CommodityGrouping,
    /// Colour number
    ColourNumber,
    /// Contract number
    ContractNumber,
    /// Customs article number
    CustomsArticleNumber,
    /// Drawing revision number
    DrawingRevisionNumber,
    /// Drawing
    Drawing,
    /// Engineering change level
    EngineeringChangeLevel,
    /// Material code
    MaterialCode,
    /// EMDN (European Medical Device Nomenclature)
    EmdnEuropeanMedicalDeviceNomenclature,
    /// International Article Numbering Association (EAN)
    InternationalArticleNumberingAssociationEan,
    /// Fish species
    FishSpecies,
    /// Buyer's internal product group code
    BuyersInternalProductGroupCode,
    /// Global model number
    GlobalModelNumber,
    /// National product group code
    NationalProductGroupCode,
    /// General specification number
    GeneralSpecificationNumber,
    /// Harmonised system
    HarmonisedSystem,
    /// ISBN (International Standard Book Number)
    IsbnInternationalStandardBookNumber,
    /// Buyer's item number
    BuyersItemNumber,
    /// ISSN (International Standard Serial Number)
    IssnInternationalStandardSerialNumber,
    /// Buyer's style number
    BuyersStyleNumber,
    /// Buyer's size code
    BuyersSizeCode,
    /// Machine number
    MachineNumber,
    /// Manufacturer's (producer's) article number
    ManufacturersProducersArticleNumber,
    /// Model number
    ModelNumber,
    /// Product/service identification number
    ProductServiceIdentificationNumber,
    /// Batch number
    BatchNumber,
    /// Customer order number
    CustomerOrderNumber,
    /// Part number description
    PartNumberDescription,
    /// Purchaser's order line number
    PurchasersOrderLineNumber,
    /// Purchase order number
    PurchaseOrderNumber,
    /// Phytosanitary Passport identifier
    PhytosanitaryPassportIdentifier,
    /// Promotional variant number
    PromotionalVariantNumber,
    /// Buyer's qualifier for size
    BuyersQualifierForSize,
    /// Returnable container number
    ReturnableContainerNumber,
    /// Release number
    ReleaseNumber,
    /// Run number
    RunNumber,
    /// Record keeping of model year
    RecordKeepingModelYear,
    /// Supplier's article number
    SuppliersArticleNumber,
    /// Standard group of products (mixed assortment)
    StandardGroupProductsMixedAssortment,
    /// SKU (Stock keeping unit)
    SkuStockKeepingUnit,
    /// Serial number
    SerialNumber,
    /// RSK number
    RskNumber,
    /// IFLS (Institut Francais du Libre Service) 5 digit product
    IflsInstitutFrancaisDuLibreService5DigitProduct,
    /// IFLS (Institut Francais du Libre Service) 9 digit product
    IflsInstitutFrancaisDuLibreService9DigitProduct,
    /// GS1 Global Trade Item Number
    Gs1GlobalTradeItemNumber,
    /// EDIS (Energy Data Identification System)
    EdisEnergyDataIdentificationSystem,
    /// Slaughter number
    SlaughterNumber,
    /// Official animal number
    OfficialAnimalNumber,
    /// Harmonized tariff schedule
    HarmonizedTariffSchedule,
    /// Supplier's supplier article number
    SuppliersSupplierArticleNumber,
    /// 46 Level DOT Code
    _46LevelDotCode,
    /// Airline Tariff 6D
    AirlineTariff6d,
    /// Title 49 Code of Federal Regulations
    Title49CodeFederalRegulations,
    /// International Civil Aviation Administration code
    InternationalCivilAviationAdministrationCode,
    /// Hazardous Materials ID DOT
    HazardousMaterialsIdDot,
    /// Endorsement
    Endorsement,
    /// Air Force Regulation 71-4
    AirForceRegulation714,
    /// Breed
    Breed,
    /// Chemical Abstract Service (CAS) registry number
    ChemicalAbstractServiceCasRegistryNumber,
    /// Engine model designation
    EngineModelDesignation,
    /// Institutional Meat Purchase Specifications (IMPS) Number
    InstitutionalMeatPurchaseSpecificationsImpsNumber,
    /// Price Look-Up code (PLU)
    PriceLookUpCodePlu,
    /// International Maritime Organization (IMO) Code
    InternationalMaritimeOrganizationImoCode,
    /// Bureau of Explosives 600-A (rail)
    BureauExplosives600ARail,
    /// United Nations Dangerous Goods List
    UnitedNationsDangerousGoodsList,
    /// International Code of Botanical Nomenclature (ICBN)
    InternationalCodeBotanicalNomenclatureIcbn,
    /// International Code of Zoological Nomenclature (ICZN)
    InternationalCodeZoologicalNomenclatureIczn,
    /// International Code of Nomenclature for Cultivated Plants
    InternationalCodeNomenclatureForCultivatedPlants,
    /// Distributor’s article identifier
    DistributorSArticleIdentifier,
    /// Norwegian Classification system ENVA
    NorwegianClassificationSystemEnva,
    /// Supplier assigned classification
    SupplierAssignedClassification,
    /// Mexican classification system AMECE
    MexicanClassificationSystemAmece,
    /// German classification system CCG
    GermanClassificationSystemCcg,
    /// Finnish classification system EANFIN
    FinnishClassificationSystemEanfin,
    /// Canadian classification system ICC
    CanadianClassificationSystemIcc,
    /// French classification system IFLS5
    FrenchClassificationSystemIfls5,
    /// Style number
    StyleNumber,
    /// Dutch classification system CBL
    DutchClassificationSystemCbl,
    /// Japanese classification system JICFS
    JapaneseClassificationSystemJicfs,
    /// European Union dairy subsidy eligibility classification
    EuropeanUnionDairySubsidyEligibilityClassification,
    /// GS1 Spain classification system
    Gs1SpainClassificationSystem,
    /// GS1 Poland classification system
    Gs1PolandClassificationSystem,
    /// Federal Agency on Technical Regulating and Metrology of the
    FederalAgencyOnTechnicalRegulatingAndMetrology,
    /// Efficient Consumer Response (ECR) Austria classification
    EfficientConsumerResponseEcrAustriaClassification,
    /// GS1 Italy classification system
    Gs1ItalyClassificationSystem,
    /// CPV (Common Procurement Vocabulary)
    CpvCommonProcurementVocabulary,
    /// IFDA (International Foodservice Distributors Association)
    IfdaInternationalFoodserviceDistributorsAssociation,
    /// AHFS (American Hospital Formulary Service) pharmacologic -
    AhfsAmericanHospitalFormularyServicePharmacologic,
    /// ATC (Anatomical Therapeutic Chemical) classification system
    AtcAnatomicalTherapeuticChemicalClassificationSystem,
    /// CLADIMED (Classification des Dispositifs Médicaux)
    CladimedClassificationDesDispositifsMédicaux,
    /// CMDR (Canadian Medical Device Regulations) classification
    CmdrCanadianMedicalDeviceRegulationsClassification,
    /// CNDM (Classificazione Nazionale dei Dispositivi Medici)
    CndmClassificazioneNazionaleDeiDispositiviMedici,
    /// UK DM&D (Dictionary of Medicines & Devices) standard coding
    UkDmDDictionaryMedicinesDevicesStandardCoding,
    /// eCl@ss
    EclSs,
    /// EDMA (European Diagnostic Manufacturers Association)
    EdmaEuropeanDiagnosticManufacturersAssociation,
    /// EGAR (European Generic Article Register)
    EgarEuropeanGenericArticleRegister,
    /// GMDN (Global Medical Devices Nomenclature)
    GmdnGlobalMedicalDevicesNomenclature,
    /// GPI (Generic Product Identifier)
    GpiGenericProductIdentifier,
    /// HCPCS (Healthcare Common Procedure Coding System)
    HcpcsHealthcareCommonProcedureCodingSystem,
    /// ICPS (International Classification for Patient Safety)
    IcpsInternationalClassificationForPatientSafety,
    /// MedDRA (Medical Dictionary for Regulatory Activities)
    MeddraMedicalDictionaryForRegulatoryActivities,
    /// Medical Columbus
    MedicalColumbus,
    /// NAPCS (North American Product Classification System)
    NapcsNorthAmericanProductClassificationSystem,
    /// NHS (National Health Services) eClass
    NhsNationalHealthServicesEclass,
    /// US FDA (Food and Drug Administration) Product Code
    UsFdaFoodAndDrugAdministrationProductCode,
    /// SNOMED CT (Systematized Nomenclature of Medicine-Clinical
    SnomedCtSystematizedNomenclatureMedicineClinical,
    /// UMDNS (Universal Medical Device Nomenclature System)
    UmdnsUniversalMedicalDeviceNomenclatureSystem,
    /// GS1 Global Returnable Asset Identifier, non-serialised
    Gs1GlobalReturnableAssetIdentifierNonSerialised,
    /// IMEI
    Imei,
    /// Waste Type (EMSA)
    WasteTypeEmsa,
    /// Ship's store classification type
    ShipsStoreClassificationType,
    /// Emergency fire code
    EmergencyFireCode,
    /// Emergency spillage code
    EmergencySpillageCode,
    /// IMDG packing group
    ImdgPackingGroup,
    /// MARPOL Code IBC
    MarpolCodeIbc,
    /// IMDG subsidiary risk class
    ImdgSubsidiaryRiskClass,
    /// Transport group number
    TransportGroupNumber,
    /// Taxonomic Serial Number
    TaxonomicSerialNumber,
    /// IMDG main hazard class
    ImdgMainHazardClass,
    /// EU Combined Nomenclature
    EuCombinedNomenclature,
    /// Therapeutic classification number
    TherapeuticClassificationNumber,
    /// European Waste Catalogue
    EuropeanWasteCatalogue,
    /// Price grouping code
    PriceGroupingCode,
    /// UNSPSC
    Unspsc,
    /// EU RoHS Directive
    EuRohsDirective,
    /// Ultimate customer's article number
    UltimateCustomersArticleNumber,
    /// UPC (Universal product code)
    UpcUniversalProductCode,
    /// Vendor item number
    VendorItemNumber,
    /// Vendor's (seller's) part number
    VendorsSellersPartNumber,
    /// Vendor's supplemental item number
    VendorsSupplementalItemNumber,
    /// Vendor specification number
    VendorSpecificationNumber,
    /// Mutually defined
    MutuallyDefined,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for Item {
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
    }
}

impl crate::Code for Item {
    fn code(self) -> &'static str {
        match self {
            Item::ProductVersionNumber => "AA",
            Item::Assembly => "AB",
            Item::HibcHealthIndustryBarCode => "AC",
            Item::ColdRollNumber => "AD",
            Item::HotRollNumber => "AE",
            Item::SlabNumber => "AF",
            Item::SoftwareRevisionNumber => "AG",
            Item::UpcUniversalProductCodeConsumerPackageCode155 => "AH",
            Item::UpcUniversalProductCodeConsumerPackageCode155_Dup => "AI",
            Item::SampleNumber => "AJ",
            Item::PackNumber => "AK",
            Item::UpcUniversalProductCodeShippingContainerCode12 => "AL",
            Item::UpcUniversalProductCodeEanEuropeanArticleNumber => "AM",
            Item::UpcUniversalProductCodeSuffix => "AN",
            Item::StateLabelCode => "AO",
            Item::HeatNumber => "AP",
            Item::CouponNumber => "AQ",
            Item::ResourceNumber => "AR",
            Item::WorkTaskNumber => "AS",
            Item::PriceLookUpNumber => "AT",
            Item::NsnNorthAtlanticTreatyOrganizationStockNumber => "AU",
            Item::RefinedProductCode => "AV",
            Item::Exhibit => "AW",
            Item::EndItem => "AX",
            Item::FederalSupplyClassification => "AY",
            Item::EngineeringDataList => "AZ",
            Item::MilestoneEventNumber => "BA",
            Item::LotNumber => "BB",
            Item::NationalDrugCode442Format => "BC",
            Item::NationalDrugCode532Format => "BD",
            Item::NationalDrugCode541Format => "BE",
            Item::NationalDrugCode542Format => "BF",
            Item::NationalDrugCode => "BG",
            Item::PartNumber => "BH",
            Item::LocalStockNumberLsn => "BI",
            Item::NextHigherAssemblyNumber => "BJ",
            Item::DataCategory => "BK",
            Item::ControlNumber => "BL",
            Item::SpecialMaterialIdentificationCode => "BM",
            Item::LocallyAssignedControlNumber => "BN",
            Item::BuyersColour => "BO",
            Item::BuyersPartNumber => "BP",
            Item::VariableMeasureProductCode => "BQ",
            Item::FinancialPhase => "BR",
            Item::ContractBreakdown => "BS",
            Item::TechnicalPhase => "BT",
            Item::DyeLotNumber => "BU",
            Item::DailyStatementActivities => "BV",
            Item::PeriodicalStatementActivitiesWithinABilaterally => "BW",
            Item::CalendarWeekStatementActivities => "BX",
            Item::CalendarMonthStatementActivities => "BY",
            Item::OriginalEquipmentNumber => "BZ",
            Item::IndustryCommodityCode => "CC",
            Item::CommodityGrouping => "CG",
            Item::ColourNumber => "CL",
            Item::ContractNumber => "CR",
            Item::CustomsArticleNumber => "CV",
            Item::DrawingRevisionNumber => "DR",
            Item::Drawing => "DW",
            Item::EngineeringChangeLevel => "EC",
            Item::MaterialCode => "EF",
            Item::EmdnEuropeanMedicalDeviceNomenclature => "EMD",
            Item::InternationalArticleNumberingAssociationEan => "EN",
            Item::FishSpecies => "FS",
            Item::BuyersInternalProductGroupCode => "GB",
            Item::GlobalModelNumber => "GMN",
            Item::NationalProductGroupCode => "GN",
            Item::GeneralSpecificationNumber => "GS",
            Item::HarmonisedSystem => "HS",
            Item::IsbnInternationalStandardBookNumber => "IB",
            Item::BuyersItemNumber => "IN",
            Item::IssnInternationalStandardSerialNumber => "IS",
            Item::BuyersStyleNumber => "IT",
            Item::BuyersSizeCode => "IZ",
            Item::MachineNumber => "MA",
            Item::ManufacturersProducersArticleNumber => "MF",
            Item::ModelNumber => "MN",
            Item::ProductServiceIdentificationNumber => "MP",
            Item::BatchNumber => "NB",
            Item::CustomerOrderNumber => "ON",
            Item::PartNumberDescription => "PD",
            Item::PurchasersOrderLineNumber => "PL",
            Item::PurchaseOrderNumber => "PO",
            Item::PhytosanitaryPassportIdentifier => "PPI",
            Item::PromotionalVariantNumber => "PV",
            Item::BuyersQualifierForSize => "QS",
            Item::ReturnableContainerNumber => "RC",
            Item::ReleaseNumber => "RN",
            Item::RunNumber => "RU",
            Item::RecordKeepingModelYear => "RY",
            Item::SuppliersArticleNumber => "SA",
            Item::StandardGroupProductsMixedAssortment => "SG",
            Item::SkuStockKeepingUnit => "SK",
            Item::SerialNumber => "SN",
            Item::RskNumber => "SRS",
            Item::IflsInstitutFrancaisDuLibreService5DigitProduct => "SRT",
            Item::IflsInstitutFrancaisDuLibreService9DigitProduct => "SRU",
            Item::Gs1GlobalTradeItemNumber => "SRV",
            Item::EdisEnergyDataIdentificationSystem => "SRW",
            Item::SlaughterNumber => "SRX",
            Item::OfficialAnimalNumber => "SRY",
            Item::HarmonizedTariffSchedule => "SRZ",
            Item::SuppliersSupplierArticleNumber => "SS",
            Item::_46LevelDotCode => "SSA",
            Item::AirlineTariff6d => "SSB",
            Item::Title49CodeFederalRegulations => "SSC",
            Item::InternationalCivilAviationAdministrationCode => "SSD",
            Item::HazardousMaterialsIdDot => "SSE",
            Item::Endorsement => "SSF",
            Item::AirForceRegulation714 => "SSG",
            Item::Breed => "SSH",
            Item::ChemicalAbstractServiceCasRegistryNumber => "SSI",
            Item::EngineModelDesignation => "SSJ",
            Item::InstitutionalMeatPurchaseSpecificationsImpsNumber => "SSK",
            Item::PriceLookUpCodePlu => "SSL",
            Item::InternationalMaritimeOrganizationImoCode => "SSM",
            Item::BureauExplosives600ARail => "SSN",
            Item::UnitedNationsDangerousGoodsList => "SSO",
            Item::InternationalCodeBotanicalNomenclatureIcbn => "SSP",
            Item::InternationalCodeZoologicalNomenclatureIczn => "SSQ",
            Item::InternationalCodeNomenclatureForCultivatedPlants => "SSR",
            Item::DistributorSArticleIdentifier => "SSS",
            Item::NorwegianClassificationSystemEnva => "SST",
            Item::SupplierAssignedClassification => "SSU",
            Item::MexicanClassificationSystemAmece => "SSV",
            Item::GermanClassificationSystemCcg => "SSW",
            Item::FinnishClassificationSystemEanfin => "SSX",
            Item::CanadianClassificationSystemIcc => "SSY",
            Item::FrenchClassificationSystemIfls5 => "SSZ",
            Item::StyleNumber => "ST",
            Item::DutchClassificationSystemCbl => "STA",
            Item::JapaneseClassificationSystemJicfs => "STB",
            Item::EuropeanUnionDairySubsidyEligibilityClassification => "STC",
            Item::Gs1SpainClassificationSystem => "STD",
            Item::Gs1PolandClassificationSystem => "STE",
            Item::FederalAgencyOnTechnicalRegulatingAndMetrology => "STF",
            Item::EfficientConsumerResponseEcrAustriaClassification => "STG",
            Item::Gs1ItalyClassificationSystem => "STH",
            Item::CpvCommonProcurementVocabulary => "STI",
            Item::IfdaInternationalFoodserviceDistributorsAssociation => "STJ",
            Item::AhfsAmericanHospitalFormularyServicePharmacologic => "STK",
            Item::AtcAnatomicalTherapeuticChemicalClassificationSystem => "STL",
            Item::CladimedClassificationDesDispositifsMédicaux => "STM",
            Item::CmdrCanadianMedicalDeviceRegulationsClassification => "STN",
            Item::CndmClassificazioneNazionaleDeiDispositiviMedici => "STO",
            Item::UkDmDDictionaryMedicinesDevicesStandardCoding => "STP",
            Item::EclSs => "STQ",
            Item::EdmaEuropeanDiagnosticManufacturersAssociation => "STR",
            Item::EgarEuropeanGenericArticleRegister => "STS",
            Item::GmdnGlobalMedicalDevicesNomenclature => "STT",
            Item::GpiGenericProductIdentifier => "STU",
            Item::HcpcsHealthcareCommonProcedureCodingSystem => "STV",
            Item::IcpsInternationalClassificationForPatientSafety => "STW",
            Item::MeddraMedicalDictionaryForRegulatoryActivities => "STX",
            Item::MedicalColumbus => "STY",
            Item::NapcsNorthAmericanProductClassificationSystem => "STZ",
            Item::NhsNationalHealthServicesEclass => "SUA",
            Item::UsFdaFoodAndDrugAdministrationProductCode => "SUB",
            Item::SnomedCtSystematizedNomenclatureMedicineClinical => "SUC",
            Item::UmdnsUniversalMedicalDeviceNomenclatureSystem => "SUD",
            Item::Gs1GlobalReturnableAssetIdentifierNonSerialised => "SUE",
            Item::Imei => "SUF",
            Item::WasteTypeEmsa => "SUG",
            Item::ShipsStoreClassificationType => "SUH",
            Item::EmergencyFireCode => "SUI",
            Item::EmergencySpillageCode => "SUJ",
            Item::ImdgPackingGroup => "SUK",
            Item::MarpolCodeIbc => "SUL",
            Item::ImdgSubsidiaryRiskClass => "SUM",
            Item::TransportGroupNumber => "TG",
            Item::TaxonomicSerialNumber => "TSN",
            Item::ImdgMainHazardClass => "TSO",
            Item::EuCombinedNomenclature => "TSP",
            Item::TherapeuticClassificationNumber => "TSQ",
            Item::EuropeanWasteCatalogue => "TSR",
            Item::PriceGroupingCode => "TSS",
            Item::Unspsc => "TST",
            Item::EuRohsDirective => "TSU",
            Item::UltimateCustomersArticleNumber => "UA",
            Item::UpcUniversalProductCode => "UP",
            Item::VendorItemNumber => "VN",
            Item::VendorsSellersPartNumber => "VP",
            Item::VendorsSupplementalItemNumber => "VS",
            Item::VendorSpecificationNumber => "VX",
            Item::MutuallyDefined => "ZZZ",
        }
    }
}

impl crate::Description for Item {
    fn description(self) -> &'static str {
        match self {
            Item::ProductVersionNumber => "Product version number",
            Item::Assembly => "Assembly",
            Item::HibcHealthIndustryBarCode => "HIBC (Health Industry Bar Code)",
            Item::ColdRollNumber => "Cold roll number",
            Item::HotRollNumber => "Hot roll number",
            Item::SlabNumber => "Slab number",
            Item::SoftwareRevisionNumber => "Software revision number",
            Item::UpcUniversalProductCodeConsumerPackageCode155 => {
                "UPC (Universal Product Code) Consumer package code (1-5-5)"
            }
            Item::UpcUniversalProductCodeConsumerPackageCode155_Dup => {
                "UPC (Universal Product Code) Consumer package code (1-5-5-"
            }
            Item::SampleNumber => "Sample number",
            Item::PackNumber => "Pack number",
            Item::UpcUniversalProductCodeShippingContainerCode12 => {
                "UPC (Universal Product Code) Shipping container code (1-2-"
            }
            Item::UpcUniversalProductCodeEanEuropeanArticleNumber => {
                "UPC (Universal Product Code)/EAN (European article number)"
            }
            Item::UpcUniversalProductCodeSuffix => "UPC (Universal Product Code) suffix",
            Item::StateLabelCode => "State label code",
            Item::HeatNumber => "Heat number",
            Item::CouponNumber => "Coupon number",
            Item::ResourceNumber => "Resource number",
            Item::WorkTaskNumber => "Work task number",
            Item::PriceLookUpNumber => "Price look up number",
            Item::NsnNorthAtlanticTreatyOrganizationStockNumber => {
                "NSN (North Atlantic Treaty Organization Stock Number)"
            }
            Item::RefinedProductCode => "Refined product code",
            Item::Exhibit => "Exhibit",
            Item::EndItem => "End item",
            Item::FederalSupplyClassification => "Federal supply classification",
            Item::EngineeringDataList => "Engineering data list",
            Item::MilestoneEventNumber => "Milestone event number",
            Item::LotNumber => "Lot number",
            Item::NationalDrugCode442Format => "National drug code 4-4-2 format",
            Item::NationalDrugCode532Format => "National drug code 5-3-2 format",
            Item::NationalDrugCode541Format => "National drug code 5-4-1 format",
            Item::NationalDrugCode542Format => "National drug code 5-4-2 format",
            Item::NationalDrugCode => "National drug code",
            Item::PartNumber => "Part number",
            Item::LocalStockNumberLsn => "Local Stock Number (LSN)",
            Item::NextHigherAssemblyNumber => "Next higher assembly number",
            Item::DataCategory => "Data category",
            Item::ControlNumber => "Control number",
            Item::SpecialMaterialIdentificationCode => "Special material identification code",
            Item::LocallyAssignedControlNumber => "Locally assigned control number",
            Item::BuyersColour => "Buyer's colour",
            Item::BuyersPartNumber => "Buyer's part number",
            Item::VariableMeasureProductCode => "Variable measure product code",
            Item::FinancialPhase => "Financial phase",
            Item::ContractBreakdown => "Contract breakdown",
            Item::TechnicalPhase => "Technical phase",
            Item::DyeLotNumber => "Dye lot number",
            Item::DailyStatementActivities => "Daily statement of activities",
            Item::PeriodicalStatementActivitiesWithinABilaterally => {
                "Periodical statement of activities within a bilaterally"
            }
            Item::CalendarWeekStatementActivities => "Calendar week statement of activities",
            Item::CalendarMonthStatementActivities => "Calendar month statement of activities",
            Item::OriginalEquipmentNumber => "Original equipment number",
            Item::IndustryCommodityCode => "Industry commodity code",
            Item::CommodityGrouping => "Commodity grouping",
            Item::ColourNumber => "Colour number",
            Item::ContractNumber => "Contract number",
            Item::CustomsArticleNumber => "Customs article number",
            Item::DrawingRevisionNumber => "Drawing revision number",
            Item::Drawing => "Drawing",
            Item::EngineeringChangeLevel => "Engineering change level",
            Item::MaterialCode => "Material code",
            Item::EmdnEuropeanMedicalDeviceNomenclature => {
                "EMDN (European Medical Device Nomenclature)"
            }
            Item::InternationalArticleNumberingAssociationEan => {
                "International Article Numbering Association (EAN)"
            }
            Item::FishSpecies => "Fish species",
            Item::BuyersInternalProductGroupCode => "Buyer's internal product group code",
            Item::GlobalModelNumber => "Global model number",
            Item::NationalProductGroupCode => "National product group code",
            Item::GeneralSpecificationNumber => "General specification number",
            Item::HarmonisedSystem => "Harmonised system",
            Item::IsbnInternationalStandardBookNumber => {
                "ISBN (International Standard Book Number)"
            }
            Item::BuyersItemNumber => "Buyer's item number",
            Item::IssnInternationalStandardSerialNumber => {
                "ISSN (International Standard Serial Number)"
            }
            Item::BuyersStyleNumber => "Buyer's style number",
            Item::BuyersSizeCode => "Buyer's size code",
            Item::MachineNumber => "Machine number",
            Item::ManufacturersProducersArticleNumber => {
                "Manufacturer's (producer's) article number"
            }
            Item::ModelNumber => "Model number",
            Item::ProductServiceIdentificationNumber => "Product/service identification number",
            Item::BatchNumber => "Batch number",
            Item::CustomerOrderNumber => "Customer order number",
            Item::PartNumberDescription => "Part number description",
            Item::PurchasersOrderLineNumber => "Purchaser's order line number",
            Item::PurchaseOrderNumber => "Purchase order number",
            Item::PhytosanitaryPassportIdentifier => "Phytosanitary Passport identifier",
            Item::PromotionalVariantNumber => "Promotional variant number",
            Item::BuyersQualifierForSize => "Buyer's qualifier for size",
            Item::ReturnableContainerNumber => "Returnable container number",
            Item::ReleaseNumber => "Release number",
            Item::RunNumber => "Run number",
            Item::RecordKeepingModelYear => "Record keeping of model year",
            Item::SuppliersArticleNumber => "Supplier's article number",
            Item::StandardGroupProductsMixedAssortment => {
                "Standard group of products (mixed assortment)"
            }
            Item::SkuStockKeepingUnit => "SKU (Stock keeping unit)",
            Item::SerialNumber => "Serial number",
            Item::RskNumber => "RSK number",
            Item::IflsInstitutFrancaisDuLibreService5DigitProduct => {
                "IFLS (Institut Francais du Libre Service) 5 digit product"
            }
            Item::IflsInstitutFrancaisDuLibreService9DigitProduct => {
                "IFLS (Institut Francais du Libre Service) 9 digit product"
            }
            Item::Gs1GlobalTradeItemNumber => "GS1 Global Trade Item Number",
            Item::EdisEnergyDataIdentificationSystem => "EDIS (Energy Data Identification System)",
            Item::SlaughterNumber => "Slaughter number",
            Item::OfficialAnimalNumber => "Official animal number",
            Item::HarmonizedTariffSchedule => "Harmonized tariff schedule",
            Item::SuppliersSupplierArticleNumber => "Supplier's supplier article number",
            Item::_46LevelDotCode => "46 Level DOT Code",
            Item::AirlineTariff6d => "Airline Tariff 6D",
            Item::Title49CodeFederalRegulations => "Title 49 Code of Federal Regulations",
            Item::InternationalCivilAviationAdministrationCode => {
                "International Civil Aviation Administration code"
            }
            Item::HazardousMaterialsIdDot => "Hazardous Materials ID DOT",
            Item::Endorsement => "Endorsement",
            Item::AirForceRegulation714 => "Air Force Regulation 71-4",
            Item::Breed => "Breed",
            Item::ChemicalAbstractServiceCasRegistryNumber => {
                "Chemical Abstract Service (CAS) registry number"
            }
            Item::EngineModelDesignation => "Engine model designation",
            Item::InstitutionalMeatPurchaseSpecificationsImpsNumber => {
                "Institutional Meat Purchase Specifications (IMPS) Number"
            }
            Item::PriceLookUpCodePlu => "Price Look-Up code (PLU)",
            Item::InternationalMaritimeOrganizationImoCode => {
                "International Maritime Organization (IMO) Code"
            }
            Item::BureauExplosives600ARail => "Bureau of Explosives 600-A (rail)",
            Item::UnitedNationsDangerousGoodsList => "United Nations Dangerous Goods List",
            Item::InternationalCodeBotanicalNomenclatureIcbn => {
                "International Code of Botanical Nomenclature (ICBN)"
            }
            Item::InternationalCodeZoologicalNomenclatureIczn => {
                "International Code of Zoological Nomenclature (ICZN)"
            }
            Item::InternationalCodeNomenclatureForCultivatedPlants => {
                "International Code of Nomenclature for Cultivated Plants"
            }
            Item::DistributorSArticleIdentifier => "Distributor’s article identifier",
            Item::NorwegianClassificationSystemEnva => "Norwegian Classification system ENVA",
            Item::SupplierAssignedClassification => "Supplier assigned classification",
            Item::MexicanClassificationSystemAmece => "Mexican classification system AMECE",
            Item::GermanClassificationSystemCcg => "German classification system CCG",
            Item::FinnishClassificationSystemEanfin => "Finnish classification system EANFIN",
            Item::CanadianClassificationSystemIcc => "Canadian classification system ICC",
            Item::FrenchClassificationSystemIfls5 => "French classification system IFLS5",
            Item::StyleNumber => "Style number",
            Item::DutchClassificationSystemCbl => "Dutch classification system CBL",
            Item::JapaneseClassificationSystemJicfs => "Japanese classification system JICFS",
            Item::EuropeanUnionDairySubsidyEligibilityClassification => {
                "European Union dairy subsidy eligibility classification"
            }
            Item::Gs1SpainClassificationSystem => "GS1 Spain classification system",
            Item::Gs1PolandClassificationSystem => "GS1 Poland classification system",
            Item::FederalAgencyOnTechnicalRegulatingAndMetrology => {
                "Federal Agency on Technical Regulating and Metrology of the"
            }
            Item::EfficientConsumerResponseEcrAustriaClassification => {
                "Efficient Consumer Response (ECR) Austria classification"
            }
            Item::Gs1ItalyClassificationSystem => "GS1 Italy classification system",
            Item::CpvCommonProcurementVocabulary => "CPV (Common Procurement Vocabulary)",
            Item::IfdaInternationalFoodserviceDistributorsAssociation => {
                "IFDA (International Foodservice Distributors Association)"
            }
            Item::AhfsAmericanHospitalFormularyServicePharmacologic => {
                "AHFS (American Hospital Formulary Service) pharmacologic -"
            }
            Item::AtcAnatomicalTherapeuticChemicalClassificationSystem => {
                "ATC (Anatomical Therapeutic Chemical) classification system"
            }
            Item::CladimedClassificationDesDispositifsMédicaux => {
                "CLADIMED (Classification des Dispositifs Médicaux)"
            }
            Item::CmdrCanadianMedicalDeviceRegulationsClassification => {
                "CMDR (Canadian Medical Device Regulations) classification"
            }
            Item::CndmClassificazioneNazionaleDeiDispositiviMedici => {
                "CNDM (Classificazione Nazionale dei Dispositivi Medici)"
            }
            Item::UkDmDDictionaryMedicinesDevicesStandardCoding => {
                "UK DM&D (Dictionary of Medicines & Devices) standard coding"
            }
            Item::EclSs => "eCl@ss",
            Item::EdmaEuropeanDiagnosticManufacturersAssociation => {
                "EDMA (European Diagnostic Manufacturers Association)"
            }
            Item::EgarEuropeanGenericArticleRegister => "EGAR (European Generic Article Register)",
            Item::GmdnGlobalMedicalDevicesNomenclature => {
                "GMDN (Global Medical Devices Nomenclature)"
            }
            Item::GpiGenericProductIdentifier => "GPI (Generic Product Identifier)",
            Item::HcpcsHealthcareCommonProcedureCodingSystem => {
                "HCPCS (Healthcare Common Procedure Coding System)"
            }
            Item::IcpsInternationalClassificationForPatientSafety => {
                "ICPS (International Classification for Patient Safety)"
            }
            Item::MeddraMedicalDictionaryForRegulatoryActivities => {
                "MedDRA (Medical Dictionary for Regulatory Activities)"
            }
            Item::MedicalColumbus => "Medical Columbus",
            Item::NapcsNorthAmericanProductClassificationSystem => {
                "NAPCS (North American Product Classification System)"
            }
            Item::NhsNationalHealthServicesEclass => "NHS (National Health Services) eClass",
            Item::UsFdaFoodAndDrugAdministrationProductCode => {
                "US FDA (Food and Drug Administration) Product Code"
            }
            Item::SnomedCtSystematizedNomenclatureMedicineClinical => {
                "SNOMED CT (Systematized Nomenclature of Medicine-Clinical"
            }
            Item::UmdnsUniversalMedicalDeviceNomenclatureSystem => {
                "UMDNS (Universal Medical Device Nomenclature System)"
            }
            Item::Gs1GlobalReturnableAssetIdentifierNonSerialised => {
                "GS1 Global Returnable Asset Identifier, non-serialised"
            }
            Item::Imei => "IMEI",
            Item::WasteTypeEmsa => "Waste Type (EMSA)",
            Item::ShipsStoreClassificationType => "Ship's store classification type",
            Item::EmergencyFireCode => "Emergency fire code",
            Item::EmergencySpillageCode => "Emergency spillage code",
            Item::ImdgPackingGroup => "IMDG packing group",
            Item::MarpolCodeIbc => "MARPOL Code IBC",
            Item::ImdgSubsidiaryRiskClass => "IMDG subsidiary risk class",
            Item::TransportGroupNumber => "Transport group number",
            Item::TaxonomicSerialNumber => "Taxonomic Serial Number",
            Item::ImdgMainHazardClass => "IMDG main hazard class",
            Item::EuCombinedNomenclature => "EU Combined Nomenclature",
            Item::TherapeuticClassificationNumber => "Therapeutic classification number",
            Item::EuropeanWasteCatalogue => "European Waste Catalogue",
            Item::PriceGroupingCode => "Price grouping code",
            Item::Unspsc => "UNSPSC",
            Item::EuRohsDirective => "EU RoHS Directive",
            Item::UltimateCustomersArticleNumber => "Ultimate customer's article number",
            Item::UpcUniversalProductCode => "UPC (Universal product code)",
            Item::VendorItemNumber => "Vendor item number",
            Item::VendorsSellersPartNumber => "Vendor's (seller's) part number",
            Item::VendorsSupplementalItemNumber => "Vendor's supplemental item number",
            Item::VendorSpecificationNumber => "Vendor specification number",
            Item::MutuallyDefined => "Mutually defined",
        }
    }
}

impl crate::FromCode for Item {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "AA" => Some(Item::ProductVersionNumber),
            "AB" => Some(Item::Assembly),
            "AC" => Some(Item::HibcHealthIndustryBarCode),
            "AD" => Some(Item::ColdRollNumber),
            "AE" => Some(Item::HotRollNumber),
            "AF" => Some(Item::SlabNumber),
            "AG" => Some(Item::SoftwareRevisionNumber),
            "AH" => Some(Item::UpcUniversalProductCodeConsumerPackageCode155),
            "AI" => Some(Item::UpcUniversalProductCodeConsumerPackageCode155_Dup),
            "AJ" => Some(Item::SampleNumber),
            "AK" => Some(Item::PackNumber),
            "AL" => Some(Item::UpcUniversalProductCodeShippingContainerCode12),
            "AM" => Some(Item::UpcUniversalProductCodeEanEuropeanArticleNumber),
            "AN" => Some(Item::UpcUniversalProductCodeSuffix),
            "AO" => Some(Item::StateLabelCode),
            "AP" => Some(Item::HeatNumber),
            "AQ" => Some(Item::CouponNumber),
            "AR" => Some(Item::ResourceNumber),
            "AS" => Some(Item::WorkTaskNumber),
            "AT" => Some(Item::PriceLookUpNumber),
            "AU" => Some(Item::NsnNorthAtlanticTreatyOrganizationStockNumber),
            "AV" => Some(Item::RefinedProductCode),
            "AW" => Some(Item::Exhibit),
            "AX" => Some(Item::EndItem),
            "AY" => Some(Item::FederalSupplyClassification),
            "AZ" => Some(Item::EngineeringDataList),
            "BA" => Some(Item::MilestoneEventNumber),
            "BB" => Some(Item::LotNumber),
            "BC" => Some(Item::NationalDrugCode442Format),
            "BD" => Some(Item::NationalDrugCode532Format),
            "BE" => Some(Item::NationalDrugCode541Format),
            "BF" => Some(Item::NationalDrugCode542Format),
            "BG" => Some(Item::NationalDrugCode),
            "BH" => Some(Item::PartNumber),
            "BI" => Some(Item::LocalStockNumberLsn),
            "BJ" => Some(Item::NextHigherAssemblyNumber),
            "BK" => Some(Item::DataCategory),
            "BL" => Some(Item::ControlNumber),
            "BM" => Some(Item::SpecialMaterialIdentificationCode),
            "BN" => Some(Item::LocallyAssignedControlNumber),
            "BO" => Some(Item::BuyersColour),
            "BP" => Some(Item::BuyersPartNumber),
            "BQ" => Some(Item::VariableMeasureProductCode),
            "BR" => Some(Item::FinancialPhase),
            "BS" => Some(Item::ContractBreakdown),
            "BT" => Some(Item::TechnicalPhase),
            "BU" => Some(Item::DyeLotNumber),
            "BV" => Some(Item::DailyStatementActivities),
            "BW" => Some(Item::PeriodicalStatementActivitiesWithinABilaterally),
            "BX" => Some(Item::CalendarWeekStatementActivities),
            "BY" => Some(Item::CalendarMonthStatementActivities),
            "BZ" => Some(Item::OriginalEquipmentNumber),
            "CC" => Some(Item::IndustryCommodityCode),
            "CG" => Some(Item::CommodityGrouping),
            "CL" => Some(Item::ColourNumber),
            "CR" => Some(Item::ContractNumber),
            "CV" => Some(Item::CustomsArticleNumber),
            "DR" => Some(Item::DrawingRevisionNumber),
            "DW" => Some(Item::Drawing),
            "EC" => Some(Item::EngineeringChangeLevel),
            "EF" => Some(Item::MaterialCode),
            "EMD" => Some(Item::EmdnEuropeanMedicalDeviceNomenclature),
            "EN" => Some(Item::InternationalArticleNumberingAssociationEan),
            "FS" => Some(Item::FishSpecies),
            "GB" => Some(Item::BuyersInternalProductGroupCode),
            "GMN" => Some(Item::GlobalModelNumber),
            "GN" => Some(Item::NationalProductGroupCode),
            "GS" => Some(Item::GeneralSpecificationNumber),
            "HS" => Some(Item::HarmonisedSystem),
            "IB" => Some(Item::IsbnInternationalStandardBookNumber),
            "IN" => Some(Item::BuyersItemNumber),
            "IS" => Some(Item::IssnInternationalStandardSerialNumber),
            "IT" => Some(Item::BuyersStyleNumber),
            "IZ" => Some(Item::BuyersSizeCode),
            "MA" => Some(Item::MachineNumber),
            "MF" => Some(Item::ManufacturersProducersArticleNumber),
            "MN" => Some(Item::ModelNumber),
            "MP" => Some(Item::ProductServiceIdentificationNumber),
            "NB" => Some(Item::BatchNumber),
            "ON" => Some(Item::CustomerOrderNumber),
            "PD" => Some(Item::PartNumberDescription),
            "PL" => Some(Item::PurchasersOrderLineNumber),
            "PO" => Some(Item::PurchaseOrderNumber),
            "PPI" => Some(Item::PhytosanitaryPassportIdentifier),
            "PV" => Some(Item::PromotionalVariantNumber),
            "QS" => Some(Item::BuyersQualifierForSize),
            "RC" => Some(Item::ReturnableContainerNumber),
            "RN" => Some(Item::ReleaseNumber),
            "RU" => Some(Item::RunNumber),
            "RY" => Some(Item::RecordKeepingModelYear),
            "SA" => Some(Item::SuppliersArticleNumber),
            "SG" => Some(Item::StandardGroupProductsMixedAssortment),
            "SK" => Some(Item::SkuStockKeepingUnit),
            "SN" => Some(Item::SerialNumber),
            "SRS" => Some(Item::RskNumber),
            "SRT" => Some(Item::IflsInstitutFrancaisDuLibreService5DigitProduct),
            "SRU" => Some(Item::IflsInstitutFrancaisDuLibreService9DigitProduct),
            "SRV" => Some(Item::Gs1GlobalTradeItemNumber),
            "SRW" => Some(Item::EdisEnergyDataIdentificationSystem),
            "SRX" => Some(Item::SlaughterNumber),
            "SRY" => Some(Item::OfficialAnimalNumber),
            "SRZ" => Some(Item::HarmonizedTariffSchedule),
            "SS" => Some(Item::SuppliersSupplierArticleNumber),
            "SSA" => Some(Item::_46LevelDotCode),
            "SSB" => Some(Item::AirlineTariff6d),
            "SSC" => Some(Item::Title49CodeFederalRegulations),
            "SSD" => Some(Item::InternationalCivilAviationAdministrationCode),
            "SSE" => Some(Item::HazardousMaterialsIdDot),
            "SSF" => Some(Item::Endorsement),
            "SSG" => Some(Item::AirForceRegulation714),
            "SSH" => Some(Item::Breed),
            "SSI" => Some(Item::ChemicalAbstractServiceCasRegistryNumber),
            "SSJ" => Some(Item::EngineModelDesignation),
            "SSK" => Some(Item::InstitutionalMeatPurchaseSpecificationsImpsNumber),
            "SSL" => Some(Item::PriceLookUpCodePlu),
            "SSM" => Some(Item::InternationalMaritimeOrganizationImoCode),
            "SSN" => Some(Item::BureauExplosives600ARail),
            "SSO" => Some(Item::UnitedNationsDangerousGoodsList),
            "SSP" => Some(Item::InternationalCodeBotanicalNomenclatureIcbn),
            "SSQ" => Some(Item::InternationalCodeZoologicalNomenclatureIczn),
            "SSR" => Some(Item::InternationalCodeNomenclatureForCultivatedPlants),
            "SSS" => Some(Item::DistributorSArticleIdentifier),
            "SST" => Some(Item::NorwegianClassificationSystemEnva),
            "SSU" => Some(Item::SupplierAssignedClassification),
            "SSV" => Some(Item::MexicanClassificationSystemAmece),
            "SSW" => Some(Item::GermanClassificationSystemCcg),
            "SSX" => Some(Item::FinnishClassificationSystemEanfin),
            "SSY" => Some(Item::CanadianClassificationSystemIcc),
            "SSZ" => Some(Item::FrenchClassificationSystemIfls5),
            "ST" => Some(Item::StyleNumber),
            "STA" => Some(Item::DutchClassificationSystemCbl),
            "STB" => Some(Item::JapaneseClassificationSystemJicfs),
            "STC" => Some(Item::EuropeanUnionDairySubsidyEligibilityClassification),
            "STD" => Some(Item::Gs1SpainClassificationSystem),
            "STE" => Some(Item::Gs1PolandClassificationSystem),
            "STF" => Some(Item::FederalAgencyOnTechnicalRegulatingAndMetrology),
            "STG" => Some(Item::EfficientConsumerResponseEcrAustriaClassification),
            "STH" => Some(Item::Gs1ItalyClassificationSystem),
            "STI" => Some(Item::CpvCommonProcurementVocabulary),
            "STJ" => Some(Item::IfdaInternationalFoodserviceDistributorsAssociation),
            "STK" => Some(Item::AhfsAmericanHospitalFormularyServicePharmacologic),
            "STL" => Some(Item::AtcAnatomicalTherapeuticChemicalClassificationSystem),
            "STM" => Some(Item::CladimedClassificationDesDispositifsMédicaux),
            "STN" => Some(Item::CmdrCanadianMedicalDeviceRegulationsClassification),
            "STO" => Some(Item::CndmClassificazioneNazionaleDeiDispositiviMedici),
            "STP" => Some(Item::UkDmDDictionaryMedicinesDevicesStandardCoding),
            "STQ" => Some(Item::EclSs),
            "STR" => Some(Item::EdmaEuropeanDiagnosticManufacturersAssociation),
            "STS" => Some(Item::EgarEuropeanGenericArticleRegister),
            "STT" => Some(Item::GmdnGlobalMedicalDevicesNomenclature),
            "STU" => Some(Item::GpiGenericProductIdentifier),
            "STV" => Some(Item::HcpcsHealthcareCommonProcedureCodingSystem),
            "STW" => Some(Item::IcpsInternationalClassificationForPatientSafety),
            "STX" => Some(Item::MeddraMedicalDictionaryForRegulatoryActivities),
            "STY" => Some(Item::MedicalColumbus),
            "STZ" => Some(Item::NapcsNorthAmericanProductClassificationSystem),
            "SUA" => Some(Item::NhsNationalHealthServicesEclass),
            "SUB" => Some(Item::UsFdaFoodAndDrugAdministrationProductCode),
            "SUC" => Some(Item::SnomedCtSystematizedNomenclatureMedicineClinical),
            "SUD" => Some(Item::UmdnsUniversalMedicalDeviceNomenclatureSystem),
            "SUE" => Some(Item::Gs1GlobalReturnableAssetIdentifierNonSerialised),
            "SUF" => Some(Item::Imei),
            "SUG" => Some(Item::WasteTypeEmsa),
            "SUH" => Some(Item::ShipsStoreClassificationType),
            "SUI" => Some(Item::EmergencyFireCode),
            "SUJ" => Some(Item::EmergencySpillageCode),
            "SUK" => Some(Item::ImdgPackingGroup),
            "SUL" => Some(Item::MarpolCodeIbc),
            "SUM" => Some(Item::ImdgSubsidiaryRiskClass),
            "TG" => Some(Item::TransportGroupNumber),
            "TSN" => Some(Item::TaxonomicSerialNumber),
            "TSO" => Some(Item::ImdgMainHazardClass),
            "TSP" => Some(Item::EuCombinedNomenclature),
            "TSQ" => Some(Item::TherapeuticClassificationNumber),
            "TSR" => Some(Item::EuropeanWasteCatalogue),
            "TSS" => Some(Item::PriceGroupingCode),
            "TST" => Some(Item::Unspsc),
            "TSU" => Some(Item::EuRohsDirective),
            "UA" => Some(Item::UltimateCustomersArticleNumber),
            "UP" => Some(Item::UpcUniversalProductCode),
            "VN" => Some(Item::VendorItemNumber),
            "VP" => Some(Item::VendorsSellersPartNumber),
            "VS" => Some(Item::VendorsSupplementalItemNumber),
            "VX" => Some(Item::VendorSpecificationNumber),
            "ZZZ" => Some(Item::MutuallyDefined),
            _ => None,
        }
    }
}

// Start: (Version) TryFrom Item to crate::zugferd_2_3_2::Item
impl std::convert::TryFrom<Item> for crate::zugferd_2_3_2::Item {
    type Error = ErrFromItemToCrateZugferd232Item;
    fn try_from(value: Item) -> Result<Self, Self::Error> {
        match value {
            Item::ProductVersionNumber => Ok(crate::zugferd_2_3_2::Item::ProductVersionNumber),
            Item::Assembly => Ok(crate::zugferd_2_3_2::Item::Assembly),
            Item::HibcHealthIndustryBarCode => {
                Ok(crate::zugferd_2_3_2::Item::HibcHealthIndustryBarCode)
            }
            Item::ColdRollNumber => Ok(crate::zugferd_2_3_2::Item::ColdRollNumber),
            Item::HotRollNumber => Ok(crate::zugferd_2_3_2::Item::HotRollNumber),
            Item::SlabNumber => Ok(crate::zugferd_2_3_2::Item::SlabNumber),
            Item::SoftwareRevisionNumber => Ok(crate::zugferd_2_3_2::Item::SoftwareRevisionNumber),
            Item::UpcUniversalProductCodeConsumerPackageCode155 => {
                Ok(crate::zugferd_2_3_2::Item::UpcUniversalProductCodeConsumerPackageCode155)
            }
            Item::UpcUniversalProductCodeConsumerPackageCode155_Dup => {
                Ok(crate::zugferd_2_3_2::Item::UpcUniversalProductCodeConsumerPackageCode155_Dup)
            }
            Item::SampleNumber => Ok(crate::zugferd_2_3_2::Item::SampleNumber),
            Item::PackNumber => Ok(crate::zugferd_2_3_2::Item::PackNumber),
            Item::UpcUniversalProductCodeShippingContainerCode12 => {
                Ok(crate::zugferd_2_3_2::Item::UpcUniversalProductCodeShippingContainerCode12)
            }
            Item::UpcUniversalProductCodeEanEuropeanArticleNumber => {
                Ok(crate::zugferd_2_3_2::Item::UpcUniversalProductCodeEanEuropeanArticleNumber)
            }
            Item::UpcUniversalProductCodeSuffix => {
                Ok(crate::zugferd_2_3_2::Item::UpcUniversalProductCodeSuffix)
            }
            Item::StateLabelCode => Ok(crate::zugferd_2_3_2::Item::StateLabelCode),
            Item::HeatNumber => Ok(crate::zugferd_2_3_2::Item::HeatNumber),
            Item::CouponNumber => Ok(crate::zugferd_2_3_2::Item::CouponNumber),
            Item::ResourceNumber => Ok(crate::zugferd_2_3_2::Item::ResourceNumber),
            Item::WorkTaskNumber => Ok(crate::zugferd_2_3_2::Item::WorkTaskNumber),
            Item::PriceLookUpNumber => Ok(crate::zugferd_2_3_2::Item::PriceLookUpNumber),
            Item::NsnNorthAtlanticTreatyOrganizationStockNumber => {
                Ok(crate::zugferd_2_3_2::Item::NsnNorthAtlanticTreatyOrganizationStockNumber)
            }
            Item::RefinedProductCode => Ok(crate::zugferd_2_3_2::Item::RefinedProductCode),
            Item::Exhibit => Ok(crate::zugferd_2_3_2::Item::Exhibit),
            Item::EndItem => Ok(crate::zugferd_2_3_2::Item::EndItem),
            Item::FederalSupplyClassification => {
                Ok(crate::zugferd_2_3_2::Item::FederalSupplyClassification)
            }
            Item::EngineeringDataList => Ok(crate::zugferd_2_3_2::Item::EngineeringDataList),
            Item::MilestoneEventNumber => Ok(crate::zugferd_2_3_2::Item::MilestoneEventNumber),
            Item::LotNumber => Ok(crate::zugferd_2_3_2::Item::LotNumber),
            Item::NationalDrugCode442Format => {
                Ok(crate::zugferd_2_3_2::Item::NationalDrugCode442Format)
            }
            Item::NationalDrugCode532Format => {
                Ok(crate::zugferd_2_3_2::Item::NationalDrugCode532Format)
            }
            Item::NationalDrugCode541Format => {
                Ok(crate::zugferd_2_3_2::Item::NationalDrugCode541Format)
            }
            Item::NationalDrugCode542Format => {
                Ok(crate::zugferd_2_3_2::Item::NationalDrugCode542Format)
            }
            Item::NationalDrugCode => Ok(crate::zugferd_2_3_2::Item::NationalDrugCode),
            Item::PartNumber => Ok(crate::zugferd_2_3_2::Item::PartNumber),
            Item::LocalStockNumberLsn => Ok(crate::zugferd_2_3_2::Item::LocalStockNumberLsn),
            Item::NextHigherAssemblyNumber => {
                Ok(crate::zugferd_2_3_2::Item::NextHigherAssemblyNumber)
            }
            Item::DataCategory => Ok(crate::zugferd_2_3_2::Item::DataCategory),
            Item::ControlNumber => Ok(crate::zugferd_2_3_2::Item::ControlNumber),
            Item::SpecialMaterialIdentificationCode => {
                Ok(crate::zugferd_2_3_2::Item::SpecialMaterialIdentificationCode)
            }
            Item::LocallyAssignedControlNumber => {
                Ok(crate::zugferd_2_3_2::Item::LocallyAssignedControlNumber)
            }
            Item::BuyersColour => Ok(crate::zugferd_2_3_2::Item::BuyersColour),
            Item::BuyersPartNumber => Ok(crate::zugferd_2_3_2::Item::BuyersPartNumber),
            Item::VariableMeasureProductCode => {
                Ok(crate::zugferd_2_3_2::Item::VariableMeasureProductCode)
            }
            Item::FinancialPhase => Ok(crate::zugferd_2_3_2::Item::FinancialPhase),
            Item::ContractBreakdown => Ok(crate::zugferd_2_3_2::Item::ContractBreakdown),
            Item::TechnicalPhase => Ok(crate::zugferd_2_3_2::Item::TechnicalPhase),
            Item::DyeLotNumber => Ok(crate::zugferd_2_3_2::Item::DyeLotNumber),
            Item::DailyStatementActivities => {
                Ok(crate::zugferd_2_3_2::Item::DailyStatementActivities)
            }
            Item::PeriodicalStatementActivitiesWithinABilaterally => {
                Ok(crate::zugferd_2_3_2::Item::PeriodicalStatementActivitiesWithinABilaterally)
            }
            Item::CalendarWeekStatementActivities => {
                Ok(crate::zugferd_2_3_2::Item::CalendarWeekStatementActivities)
            }
            Item::CalendarMonthStatementActivities => {
                Ok(crate::zugferd_2_3_2::Item::CalendarMonthStatementActivities)
            }
            Item::OriginalEquipmentNumber => {
                Ok(crate::zugferd_2_3_2::Item::OriginalEquipmentNumber)
            }
            Item::IndustryCommodityCode => Ok(crate::zugferd_2_3_2::Item::IndustryCommodityCode),
            Item::CommodityGrouping => Ok(crate::zugferd_2_3_2::Item::CommodityGrouping),
            Item::ColourNumber => Ok(crate::zugferd_2_3_2::Item::ColourNumber),
            Item::ContractNumber => Ok(crate::zugferd_2_3_2::Item::ContractNumber),
            Item::CustomsArticleNumber => Ok(crate::zugferd_2_3_2::Item::CustomsArticleNumber),
            Item::DrawingRevisionNumber => Ok(crate::zugferd_2_3_2::Item::DrawingRevisionNumber),
            Item::Drawing => Ok(crate::zugferd_2_3_2::Item::Drawing),
            Item::EngineeringChangeLevel => Ok(crate::zugferd_2_3_2::Item::EngineeringChangeLevel),
            Item::MaterialCode => Ok(crate::zugferd_2_3_2::Item::MaterialCode),
            Item::EmdnEuropeanMedicalDeviceNomenclature => {
                Ok(crate::zugferd_2_3_2::Item::EmdnEuropeanMedicalDeviceNomenclature)
            }
            Item::InternationalArticleNumberingAssociationEan => {
                Ok(crate::zugferd_2_3_2::Item::InternationalArticleNumberingAssociationEan)
            }
            Item::FishSpecies => Ok(crate::zugferd_2_3_2::Item::FishSpecies),
            Item::BuyersInternalProductGroupCode => {
                Ok(crate::zugferd_2_3_2::Item::BuyersInternalProductGroupCode)
            }
            Item::GlobalModelNumber => Ok(crate::zugferd_2_3_2::Item::GlobalModelNumber),
            Item::NationalProductGroupCode => {
                Ok(crate::zugferd_2_3_2::Item::NationalProductGroupCode)
            }
            Item::GeneralSpecificationNumber => {
                Ok(crate::zugferd_2_3_2::Item::GeneralSpecificationNumber)
            }
            Item::HarmonisedSystem => Ok(crate::zugferd_2_3_2::Item::HarmonisedSystem),
            Item::IsbnInternationalStandardBookNumber => {
                Ok(crate::zugferd_2_3_2::Item::IsbnInternationalStandardBookNumber)
            }
            Item::BuyersItemNumber => Ok(crate::zugferd_2_3_2::Item::BuyersItemNumber),
            Item::IssnInternationalStandardSerialNumber => {
                Ok(crate::zugferd_2_3_2::Item::IssnInternationalStandardSerialNumber)
            }
            Item::BuyersStyleNumber => Ok(crate::zugferd_2_3_2::Item::BuyersStyleNumber),
            Item::BuyersSizeCode => Ok(crate::zugferd_2_3_2::Item::BuyersSizeCode),
            Item::MachineNumber => Ok(crate::zugferd_2_3_2::Item::MachineNumber),
            Item::ManufacturersProducersArticleNumber => {
                Ok(crate::zugferd_2_3_2::Item::ManufacturersProducersArticleNumber)
            }
            Item::ModelNumber => Ok(crate::zugferd_2_3_2::Item::ModelNumber),
            Item::ProductServiceIdentificationNumber => {
                Ok(crate::zugferd_2_3_2::Item::ProductServiceIdentificationNumber)
            }
            Item::BatchNumber => Ok(crate::zugferd_2_3_2::Item::BatchNumber),
            Item::CustomerOrderNumber => Ok(crate::zugferd_2_3_2::Item::CustomerOrderNumber),
            Item::PartNumberDescription => Ok(crate::zugferd_2_3_2::Item::PartNumberDescription),
            Item::PurchasersOrderLineNumber => {
                Ok(crate::zugferd_2_3_2::Item::PurchasersOrderLineNumber)
            }
            Item::PurchaseOrderNumber => Ok(crate::zugferd_2_3_2::Item::PurchaseOrderNumber),
            Item::PromotionalVariantNumber => {
                Ok(crate::zugferd_2_3_2::Item::PromotionalVariantNumber)
            }
            Item::BuyersQualifierForSize => Ok(crate::zugferd_2_3_2::Item::BuyersQualifierForSize),
            Item::ReturnableContainerNumber => {
                Ok(crate::zugferd_2_3_2::Item::ReturnableContainerNumber)
            }
            Item::ReleaseNumber => Ok(crate::zugferd_2_3_2::Item::ReleaseNumber),
            Item::RunNumber => Ok(crate::zugferd_2_3_2::Item::RunNumber),
            Item::RecordKeepingModelYear => Ok(crate::zugferd_2_3_2::Item::RecordKeepingModelYear),
            Item::SuppliersArticleNumber => Ok(crate::zugferd_2_3_2::Item::SuppliersArticleNumber),
            Item::StandardGroupProductsMixedAssortment => {
                Ok(crate::zugferd_2_3_2::Item::StandardGroupProductsMixedAssortment)
            }
            Item::SkuStockKeepingUnit => Ok(crate::zugferd_2_3_2::Item::SkuStockKeepingUnit),
            Item::SerialNumber => Ok(crate::zugferd_2_3_2::Item::SerialNumber),
            Item::RskNumber => Ok(crate::zugferd_2_3_2::Item::RskNumber),
            Item::IflsInstitutFrancaisDuLibreService5DigitProduct => {
                Ok(crate::zugferd_2_3_2::Item::IflsInstitutFrancaisDuLibreService5DigitProduct)
            }
            Item::IflsInstitutFrancaisDuLibreService9DigitProduct => {
                Ok(crate::zugferd_2_3_2::Item::IflsInstitutFrancaisDuLibreService9DigitProduct)
            }
            Item::Gs1GlobalTradeItemNumber => {
                Ok(crate::zugferd_2_3_2::Item::Gs1GlobalTradeItemNumber)
            }
            Item::EdisEnergyDataIdentificationSystem => {
                Ok(crate::zugferd_2_3_2::Item::EdisEnergyDataIdentificationSystem)
            }
            Item::SlaughterNumber => Ok(crate::zugferd_2_3_2::Item::SlaughterNumber),
            Item::OfficialAnimalNumber => Ok(crate::zugferd_2_3_2::Item::OfficialAnimalNumber),
            Item::HarmonizedTariffSchedule => {
                Ok(crate::zugferd_2_3_2::Item::HarmonizedTariffSchedule)
            }
            Item::SuppliersSupplierArticleNumber => {
                Ok(crate::zugferd_2_3_2::Item::SuppliersSupplierArticleNumber)
            }
            Item::_46LevelDotCode => Ok(crate::zugferd_2_3_2::Item::_46LevelDotCode),
            Item::AirlineTariff6d => Ok(crate::zugferd_2_3_2::Item::AirlineTariff6d),
            Item::Title49CodeFederalRegulations => {
                Ok(crate::zugferd_2_3_2::Item::Title49CodeFederalRegulations)
            }
            Item::InternationalCivilAviationAdministrationCode => {
                Ok(crate::zugferd_2_3_2::Item::InternationalCivilAviationAdministrationCode)
            }
            Item::HazardousMaterialsIdDot => {
                Ok(crate::zugferd_2_3_2::Item::HazardousMaterialsIdDot)
            }
            Item::Endorsement => Ok(crate::zugferd_2_3_2::Item::Endorsement),
            Item::AirForceRegulation714 => Ok(crate::zugferd_2_3_2::Item::AirForceRegulation714),
            Item::Breed => Ok(crate::zugferd_2_3_2::Item::Breed),
            Item::ChemicalAbstractServiceCasRegistryNumber => {
                Ok(crate::zugferd_2_3_2::Item::ChemicalAbstractServiceCasRegistryNumber)
            }
            Item::EngineModelDesignation => Ok(crate::zugferd_2_3_2::Item::EngineModelDesignation),
            Item::InstitutionalMeatPurchaseSpecificationsImpsNumber => {
                Ok(crate::zugferd_2_3_2::Item::InstitutionalMeatPurchaseSpecificationsImpsNumber)
            }
            Item::PriceLookUpCodePlu => Ok(crate::zugferd_2_3_2::Item::PriceLookUpCodePlu),
            Item::InternationalMaritimeOrganizationImoCode => {
                Ok(crate::zugferd_2_3_2::Item::InternationalMaritimeOrganizationImoCode)
            }
            Item::BureauExplosives600ARail => {
                Ok(crate::zugferd_2_3_2::Item::BureauExplosives600ARail)
            }
            Item::UnitedNationsDangerousGoodsList => {
                Ok(crate::zugferd_2_3_2::Item::UnitedNationsDangerousGoodsList)
            }
            Item::InternationalCodeBotanicalNomenclatureIcbn => {
                Ok(crate::zugferd_2_3_2::Item::InternationalCodeBotanicalNomenclatureIcbn)
            }
            Item::InternationalCodeZoologicalNomenclatureIczn => {
                Ok(crate::zugferd_2_3_2::Item::InternationalCodeZoologicalNomenclatureIczn)
            }
            Item::InternationalCodeNomenclatureForCultivatedPlants => {
                Ok(crate::zugferd_2_3_2::Item::InternationalCodeNomenclatureForCultivatedPlants)
            }
            Item::DistributorSArticleIdentifier => {
                Ok(crate::zugferd_2_3_2::Item::DistributorSArticleIdentifier)
            }
            Item::NorwegianClassificationSystemEnva => {
                Ok(crate::zugferd_2_3_2::Item::NorwegianClassificationSystemEnva)
            }
            Item::SupplierAssignedClassification => {
                Ok(crate::zugferd_2_3_2::Item::SupplierAssignedClassification)
            }
            Item::MexicanClassificationSystemAmece => {
                Ok(crate::zugferd_2_3_2::Item::MexicanClassificationSystemAmece)
            }
            Item::GermanClassificationSystemCcg => {
                Ok(crate::zugferd_2_3_2::Item::GermanClassificationSystemCcg)
            }
            Item::FinnishClassificationSystemEanfin => {
                Ok(crate::zugferd_2_3_2::Item::FinnishClassificationSystemEanfin)
            }
            Item::CanadianClassificationSystemIcc => {
                Ok(crate::zugferd_2_3_2::Item::CanadianClassificationSystemIcc)
            }
            Item::FrenchClassificationSystemIfls5 => {
                Ok(crate::zugferd_2_3_2::Item::FrenchClassificationSystemIfls5)
            }
            Item::StyleNumber => Ok(crate::zugferd_2_3_2::Item::StyleNumber),
            Item::DutchClassificationSystemCbl => {
                Ok(crate::zugferd_2_3_2::Item::DutchClassificationSystemCbl)
            }
            Item::JapaneseClassificationSystemJicfs => {
                Ok(crate::zugferd_2_3_2::Item::JapaneseClassificationSystemJicfs)
            }
            Item::EuropeanUnionDairySubsidyEligibilityClassification => {
                Ok(crate::zugferd_2_3_2::Item::EuropeanUnionDairySubsidyEligibilityClassification)
            }
            Item::Gs1SpainClassificationSystem => {
                Ok(crate::zugferd_2_3_2::Item::Gs1SpainClassificationSystem)
            }
            Item::Gs1PolandClassificationSystem => {
                Ok(crate::zugferd_2_3_2::Item::Gs1PolandClassificationSystem)
            }
            Item::FederalAgencyOnTechnicalRegulatingAndMetrology => {
                Ok(crate::zugferd_2_3_2::Item::FederalAgencyOnTechnicalRegulatingAndMetrology)
            }
            Item::EfficientConsumerResponseEcrAustriaClassification => {
                Ok(crate::zugferd_2_3_2::Item::EfficientConsumerResponseEcrAustriaClassification)
            }
            Item::Gs1ItalyClassificationSystem => {
                Ok(crate::zugferd_2_3_2::Item::Gs1ItalyClassificationSystem)
            }
            Item::CpvCommonProcurementVocabulary => {
                Ok(crate::zugferd_2_3_2::Item::CpvCommonProcurementVocabulary)
            }
            Item::IfdaInternationalFoodserviceDistributorsAssociation => {
                Ok(crate::zugferd_2_3_2::Item::IfdaInternationalFoodserviceDistributorsAssociation)
            }
            Item::AhfsAmericanHospitalFormularyServicePharmacologic => {
                Ok(crate::zugferd_2_3_2::Item::AhfsAmericanHospitalFormularyServicePharmacologic)
            }
            Item::AtcAnatomicalTherapeuticChemicalClassificationSystem => Ok(
                crate::zugferd_2_3_2::Item::AtcAnatomicalTherapeuticChemicalClassificationSystem,
            ),
            Item::CladimedClassificationDesDispositifsMédicaux => {
                Ok(crate::zugferd_2_3_2::Item::CladimedClassificationDesDispositifsMédicaux)
            }
            Item::CmdrCanadianMedicalDeviceRegulationsClassification => {
                Ok(crate::zugferd_2_3_2::Item::CmdrCanadianMedicalDeviceRegulationsClassification)
            }
            Item::CndmClassificazioneNazionaleDeiDispositiviMedici => {
                Ok(crate::zugferd_2_3_2::Item::CndmClassificazioneNazionaleDeiDispositiviMedici)
            }
            Item::UkDmDDictionaryMedicinesDevicesStandardCoding => {
                Ok(crate::zugferd_2_3_2::Item::UkDmDDictionaryMedicinesDevicesStandardCoding)
            }
            Item::EclSs => Ok(crate::zugferd_2_3_2::Item::EclSs),
            Item::EdmaEuropeanDiagnosticManufacturersAssociation => {
                Ok(crate::zugferd_2_3_2::Item::EdmaEuropeanDiagnosticManufacturersAssociation)
            }
            Item::EgarEuropeanGenericArticleRegister => {
                Ok(crate::zugferd_2_3_2::Item::EgarEuropeanGenericArticleRegister)
            }
            Item::GmdnGlobalMedicalDevicesNomenclature => {
                Ok(crate::zugferd_2_3_2::Item::GmdnGlobalMedicalDevicesNomenclature)
            }
            Item::GpiGenericProductIdentifier => {
                Ok(crate::zugferd_2_3_2::Item::GpiGenericProductIdentifier)
            }
            Item::HcpcsHealthcareCommonProcedureCodingSystem => {
                Ok(crate::zugferd_2_3_2::Item::HcpcsHealthcareCommonProcedureCodingSystem)
            }
            Item::IcpsInternationalClassificationForPatientSafety => {
                Ok(crate::zugferd_2_3_2::Item::IcpsInternationalClassificationForPatientSafety)
            }
            Item::MeddraMedicalDictionaryForRegulatoryActivities => {
                Ok(crate::zugferd_2_3_2::Item::MeddraMedicalDictionaryForRegulatoryActivities)
            }
            Item::MedicalColumbus => Ok(crate::zugferd_2_3_2::Item::MedicalColumbus),
            Item::NapcsNorthAmericanProductClassificationSystem => {
                Ok(crate::zugferd_2_3_2::Item::NapcsNorthAmericanProductClassificationSystem)
            }
            Item::NhsNationalHealthServicesEclass => {
                Ok(crate::zugferd_2_3_2::Item::NhsNationalHealthServicesEclass)
            }
            Item::UsFdaFoodAndDrugAdministrationProductCode => {
                Ok(crate::zugferd_2_3_2::Item::UsFdaFoodAndDrugAdministrationProductCode)
            }
            Item::SnomedCtSystematizedNomenclatureMedicineClinical => {
                Ok(crate::zugferd_2_3_2::Item::SnomedCtSystematizedNomenclatureMedicineClinical)
            }
            Item::UmdnsUniversalMedicalDeviceNomenclatureSystem => {
                Ok(crate::zugferd_2_3_2::Item::UmdnsUniversalMedicalDeviceNomenclatureSystem)
            }
            Item::Gs1GlobalReturnableAssetIdentifierNonSerialised => {
                Ok(crate::zugferd_2_3_2::Item::Gs1GlobalReturnableAssetIdentifierNonSerialised)
            }
            Item::Imei => Ok(crate::zugferd_2_3_2::Item::Imei),
            Item::WasteTypeEmsa => Ok(crate::zugferd_2_3_2::Item::WasteTypeEmsa),
            Item::ShipsStoreClassificationType => {
                Ok(crate::zugferd_2_3_2::Item::ShipsStoreClassificationType)
            }
            Item::EmergencyFireCode => Ok(crate::zugferd_2_3_2::Item::EmergencyFireCode),
            Item::EmergencySpillageCode => Ok(crate::zugferd_2_3_2::Item::EmergencySpillageCode),
            Item::ImdgPackingGroup => Ok(crate::zugferd_2_3_2::Item::ImdgPackingGroup),
            Item::MarpolCodeIbc => Ok(crate::zugferd_2_3_2::Item::MarpolCodeIbc),
            Item::ImdgSubsidiaryRiskClass => {
                Ok(crate::zugferd_2_3_2::Item::ImdgSubsidiaryRiskClass)
            }
            Item::TransportGroupNumber => Ok(crate::zugferd_2_3_2::Item::TransportGroupNumber),
            Item::TaxonomicSerialNumber => Ok(crate::zugferd_2_3_2::Item::TaxonomicSerialNumber),
            Item::ImdgMainHazardClass => Ok(crate::zugferd_2_3_2::Item::ImdgMainHazardClass),
            Item::EuCombinedNomenclature => Ok(crate::zugferd_2_3_2::Item::EuCombinedNomenclature),
            Item::TherapeuticClassificationNumber => {
                Ok(crate::zugferd_2_3_2::Item::TherapeuticClassificationNumber)
            }
            Item::EuropeanWasteCatalogue => Ok(crate::zugferd_2_3_2::Item::EuropeanWasteCatalogue),
            Item::PriceGroupingCode => Ok(crate::zugferd_2_3_2::Item::PriceGroupingCode),
            Item::Unspsc => Ok(crate::zugferd_2_3_2::Item::Unspsc),
            Item::EuRohsDirective => Ok(crate::zugferd_2_3_2::Item::EuRohsDirective),
            Item::UltimateCustomersArticleNumber => {
                Ok(crate::zugferd_2_3_2::Item::UltimateCustomersArticleNumber)
            }
            Item::UpcUniversalProductCode => {
                Ok(crate::zugferd_2_3_2::Item::UpcUniversalProductCode)
            }
            Item::VendorItemNumber => Ok(crate::zugferd_2_3_2::Item::VendorItemNumber),
            Item::VendorsSellersPartNumber => {
                Ok(crate::zugferd_2_3_2::Item::VendorsSellersPartNumber)
            }
            Item::VendorsSupplementalItemNumber => {
                Ok(crate::zugferd_2_3_2::Item::VendorsSupplementalItemNumber)
            }
            Item::VendorSpecificationNumber => {
                Ok(crate::zugferd_2_3_2::Item::VendorSpecificationNumber)
            }
            Item::MutuallyDefined => Ok(crate::zugferd_2_3_2::Item::MutuallyDefined),
            Item::PhytosanitaryPassportIdentifier => {
                Err(ErrFromItemToCrateZugferd232Item::PhytosanitaryPassportIdentifier)
            }
        }
    }
}

/// All the variants of Item that are not matched to any variant of crate::zugferd_2_3_2::Item
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrFromItemToCrateZugferd232Item {
    PhytosanitaryPassportIdentifier,
}

impl std::fmt::Display for ErrFromItemToCrateZugferd232Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrFromItemToCrateZugferd232Item::PhytosanitaryPassportIdentifier => write!(f, "PhytosanitaryPassportIdentifier has no corresponding value in crate::zugferd_2_3_2::Item"),
        }
    }
}

impl std::error::Error for ErrFromItemToCrateZugferd232Item {}

impl std::convert::TryFrom<crate::zugferd_2_3_2::Item> for Item {
    type Error = std::convert::Infallible;
    fn try_from(value: crate::zugferd_2_3_2::Item) -> Result<Item, Self::Error> {
        match value {
            crate::zugferd_2_3_2::Item::ProductVersionNumber => Ok(Item::ProductVersionNumber),
            crate::zugferd_2_3_2::Item::Assembly => Ok(Item::Assembly),
            crate::zugferd_2_3_2::Item::HibcHealthIndustryBarCode => {
                Ok(Item::HibcHealthIndustryBarCode)
            }
            crate::zugferd_2_3_2::Item::ColdRollNumber => Ok(Item::ColdRollNumber),
            crate::zugferd_2_3_2::Item::HotRollNumber => Ok(Item::HotRollNumber),
            crate::zugferd_2_3_2::Item::SlabNumber => Ok(Item::SlabNumber),
            crate::zugferd_2_3_2::Item::SoftwareRevisionNumber => Ok(Item::SoftwareRevisionNumber),
            crate::zugferd_2_3_2::Item::UpcUniversalProductCodeConsumerPackageCode155 => {
                Ok(Item::UpcUniversalProductCodeConsumerPackageCode155)
            }
            crate::zugferd_2_3_2::Item::UpcUniversalProductCodeConsumerPackageCode155_Dup => {
                Ok(Item::UpcUniversalProductCodeConsumerPackageCode155_Dup)
            }
            crate::zugferd_2_3_2::Item::SampleNumber => Ok(Item::SampleNumber),
            crate::zugferd_2_3_2::Item::PackNumber => Ok(Item::PackNumber),
            crate::zugferd_2_3_2::Item::UpcUniversalProductCodeShippingContainerCode12 => {
                Ok(Item::UpcUniversalProductCodeShippingContainerCode12)
            }
            crate::zugferd_2_3_2::Item::UpcUniversalProductCodeEanEuropeanArticleNumber => {
                Ok(Item::UpcUniversalProductCodeEanEuropeanArticleNumber)
            }
            crate::zugferd_2_3_2::Item::UpcUniversalProductCodeSuffix => {
                Ok(Item::UpcUniversalProductCodeSuffix)
            }
            crate::zugferd_2_3_2::Item::StateLabelCode => Ok(Item::StateLabelCode),
            crate::zugferd_2_3_2::Item::HeatNumber => Ok(Item::HeatNumber),
            crate::zugferd_2_3_2::Item::CouponNumber => Ok(Item::CouponNumber),
            crate::zugferd_2_3_2::Item::ResourceNumber => Ok(Item::ResourceNumber),
            crate::zugferd_2_3_2::Item::WorkTaskNumber => Ok(Item::WorkTaskNumber),
            crate::zugferd_2_3_2::Item::PriceLookUpNumber => Ok(Item::PriceLookUpNumber),
            crate::zugferd_2_3_2::Item::NsnNorthAtlanticTreatyOrganizationStockNumber => {
                Ok(Item::NsnNorthAtlanticTreatyOrganizationStockNumber)
            }
            crate::zugferd_2_3_2::Item::RefinedProductCode => Ok(Item::RefinedProductCode),
            crate::zugferd_2_3_2::Item::Exhibit => Ok(Item::Exhibit),
            crate::zugferd_2_3_2::Item::EndItem => Ok(Item::EndItem),
            crate::zugferd_2_3_2::Item::FederalSupplyClassification => {
                Ok(Item::FederalSupplyClassification)
            }
            crate::zugferd_2_3_2::Item::EngineeringDataList => Ok(Item::EngineeringDataList),
            crate::zugferd_2_3_2::Item::MilestoneEventNumber => Ok(Item::MilestoneEventNumber),
            crate::zugferd_2_3_2::Item::LotNumber => Ok(Item::LotNumber),
            crate::zugferd_2_3_2::Item::NationalDrugCode442Format => {
                Ok(Item::NationalDrugCode442Format)
            }
            crate::zugferd_2_3_2::Item::NationalDrugCode532Format => {
                Ok(Item::NationalDrugCode532Format)
            }
            crate::zugferd_2_3_2::Item::NationalDrugCode541Format => {
                Ok(Item::NationalDrugCode541Format)
            }
            crate::zugferd_2_3_2::Item::NationalDrugCode542Format => {
                Ok(Item::NationalDrugCode542Format)
            }
            crate::zugferd_2_3_2::Item::NationalDrugCode => Ok(Item::NationalDrugCode),
            crate::zugferd_2_3_2::Item::PartNumber => Ok(Item::PartNumber),
            crate::zugferd_2_3_2::Item::LocalStockNumberLsn => Ok(Item::LocalStockNumberLsn),
            crate::zugferd_2_3_2::Item::NextHigherAssemblyNumber => {
                Ok(Item::NextHigherAssemblyNumber)
            }
            crate::zugferd_2_3_2::Item::DataCategory => Ok(Item::DataCategory),
            crate::zugferd_2_3_2::Item::ControlNumber => Ok(Item::ControlNumber),
            crate::zugferd_2_3_2::Item::SpecialMaterialIdentificationCode => {
                Ok(Item::SpecialMaterialIdentificationCode)
            }
            crate::zugferd_2_3_2::Item::LocallyAssignedControlNumber => {
                Ok(Item::LocallyAssignedControlNumber)
            }
            crate::zugferd_2_3_2::Item::BuyersColour => Ok(Item::BuyersColour),
            crate::zugferd_2_3_2::Item::BuyersPartNumber => Ok(Item::BuyersPartNumber),
            crate::zugferd_2_3_2::Item::VariableMeasureProductCode => {
                Ok(Item::VariableMeasureProductCode)
            }
            crate::zugferd_2_3_2::Item::FinancialPhase => Ok(Item::FinancialPhase),
            crate::zugferd_2_3_2::Item::ContractBreakdown => Ok(Item::ContractBreakdown),
            crate::zugferd_2_3_2::Item::TechnicalPhase => Ok(Item::TechnicalPhase),
            crate::zugferd_2_3_2::Item::DyeLotNumber => Ok(Item::DyeLotNumber),
            crate::zugferd_2_3_2::Item::DailyStatementActivities => {
                Ok(Item::DailyStatementActivities)
            }
            crate::zugferd_2_3_2::Item::PeriodicalStatementActivitiesWithinABilaterally => {
                Ok(Item::PeriodicalStatementActivitiesWithinABilaterally)
            }
            crate::zugferd_2_3_2::Item::CalendarWeekStatementActivities => {
                Ok(Item::CalendarWeekStatementActivities)
            }
            crate::zugferd_2_3_2::Item::CalendarMonthStatementActivities => {
                Ok(Item::CalendarMonthStatementActivities)
            }
            crate::zugferd_2_3_2::Item::OriginalEquipmentNumber => {
                Ok(Item::OriginalEquipmentNumber)
            }
            crate::zugferd_2_3_2::Item::IndustryCommodityCode => Ok(Item::IndustryCommodityCode),
            crate::zugferd_2_3_2::Item::CommodityGrouping => Ok(Item::CommodityGrouping),
            crate::zugferd_2_3_2::Item::ColourNumber => Ok(Item::ColourNumber),
            crate::zugferd_2_3_2::Item::ContractNumber => Ok(Item::ContractNumber),
            crate::zugferd_2_3_2::Item::CustomsArticleNumber => Ok(Item::CustomsArticleNumber),
            crate::zugferd_2_3_2::Item::DrawingRevisionNumber => Ok(Item::DrawingRevisionNumber),
            crate::zugferd_2_3_2::Item::Drawing => Ok(Item::Drawing),
            crate::zugferd_2_3_2::Item::EngineeringChangeLevel => Ok(Item::EngineeringChangeLevel),
            crate::zugferd_2_3_2::Item::MaterialCode => Ok(Item::MaterialCode),
            crate::zugferd_2_3_2::Item::EmdnEuropeanMedicalDeviceNomenclature => {
                Ok(Item::EmdnEuropeanMedicalDeviceNomenclature)
            }
            crate::zugferd_2_3_2::Item::InternationalArticleNumberingAssociationEan => {
                Ok(Item::InternationalArticleNumberingAssociationEan)
            }
            crate::zugferd_2_3_2::Item::FishSpecies => Ok(Item::FishSpecies),
            crate::zugferd_2_3_2::Item::BuyersInternalProductGroupCode => {
                Ok(Item::BuyersInternalProductGroupCode)
            }
            crate::zugferd_2_3_2::Item::GlobalModelNumber => Ok(Item::GlobalModelNumber),
            crate::zugferd_2_3_2::Item::NationalProductGroupCode => {
                Ok(Item::NationalProductGroupCode)
            }
            crate::zugferd_2_3_2::Item::GeneralSpecificationNumber => {
                Ok(Item::GeneralSpecificationNumber)
            }
            crate::zugferd_2_3_2::Item::HarmonisedSystem => Ok(Item::HarmonisedSystem),
            crate::zugferd_2_3_2::Item::IsbnInternationalStandardBookNumber => {
                Ok(Item::IsbnInternationalStandardBookNumber)
            }
            crate::zugferd_2_3_2::Item::BuyersItemNumber => Ok(Item::BuyersItemNumber),
            crate::zugferd_2_3_2::Item::IssnInternationalStandardSerialNumber => {
                Ok(Item::IssnInternationalStandardSerialNumber)
            }
            crate::zugferd_2_3_2::Item::BuyersStyleNumber => Ok(Item::BuyersStyleNumber),
            crate::zugferd_2_3_2::Item::BuyersSizeCode => Ok(Item::BuyersSizeCode),
            crate::zugferd_2_3_2::Item::MachineNumber => Ok(Item::MachineNumber),
            crate::zugferd_2_3_2::Item::ManufacturersProducersArticleNumber => {
                Ok(Item::ManufacturersProducersArticleNumber)
            }
            crate::zugferd_2_3_2::Item::ModelNumber => Ok(Item::ModelNumber),
            crate::zugferd_2_3_2::Item::ProductServiceIdentificationNumber => {
                Ok(Item::ProductServiceIdentificationNumber)
            }
            crate::zugferd_2_3_2::Item::BatchNumber => Ok(Item::BatchNumber),
            crate::zugferd_2_3_2::Item::CustomerOrderNumber => Ok(Item::CustomerOrderNumber),
            crate::zugferd_2_3_2::Item::PartNumberDescription => Ok(Item::PartNumberDescription),
            crate::zugferd_2_3_2::Item::PurchasersOrderLineNumber => {
                Ok(Item::PurchasersOrderLineNumber)
            }
            crate::zugferd_2_3_2::Item::PurchaseOrderNumber => Ok(Item::PurchaseOrderNumber),
            crate::zugferd_2_3_2::Item::PromotionalVariantNumber => {
                Ok(Item::PromotionalVariantNumber)
            }
            crate::zugferd_2_3_2::Item::BuyersQualifierForSize => Ok(Item::BuyersQualifierForSize),
            crate::zugferd_2_3_2::Item::ReturnableContainerNumber => {
                Ok(Item::ReturnableContainerNumber)
            }
            crate::zugferd_2_3_2::Item::ReleaseNumber => Ok(Item::ReleaseNumber),
            crate::zugferd_2_3_2::Item::RunNumber => Ok(Item::RunNumber),
            crate::zugferd_2_3_2::Item::RecordKeepingModelYear => Ok(Item::RecordKeepingModelYear),
            crate::zugferd_2_3_2::Item::SuppliersArticleNumber => Ok(Item::SuppliersArticleNumber),
            crate::zugferd_2_3_2::Item::StandardGroupProductsMixedAssortment => {
                Ok(Item::StandardGroupProductsMixedAssortment)
            }
            crate::zugferd_2_3_2::Item::SkuStockKeepingUnit => Ok(Item::SkuStockKeepingUnit),
            crate::zugferd_2_3_2::Item::SerialNumber => Ok(Item::SerialNumber),
            crate::zugferd_2_3_2::Item::RskNumber => Ok(Item::RskNumber),
            crate::zugferd_2_3_2::Item::IflsInstitutFrancaisDuLibreService5DigitProduct => {
                Ok(Item::IflsInstitutFrancaisDuLibreService5DigitProduct)
            }
            crate::zugferd_2_3_2::Item::IflsInstitutFrancaisDuLibreService9DigitProduct => {
                Ok(Item::IflsInstitutFrancaisDuLibreService9DigitProduct)
            }
            crate::zugferd_2_3_2::Item::Gs1GlobalTradeItemNumber => {
                Ok(Item::Gs1GlobalTradeItemNumber)
            }
            crate::zugferd_2_3_2::Item::EdisEnergyDataIdentificationSystem => {
                Ok(Item::EdisEnergyDataIdentificationSystem)
            }
            crate::zugferd_2_3_2::Item::SlaughterNumber => Ok(Item::SlaughterNumber),
            crate::zugferd_2_3_2::Item::OfficialAnimalNumber => Ok(Item::OfficialAnimalNumber),
            crate::zugferd_2_3_2::Item::HarmonizedTariffSchedule => {
                Ok(Item::HarmonizedTariffSchedule)
            }
            crate::zugferd_2_3_2::Item::SuppliersSupplierArticleNumber => {
                Ok(Item::SuppliersSupplierArticleNumber)
            }
            crate::zugferd_2_3_2::Item::_46LevelDotCode => Ok(Item::_46LevelDotCode),
            crate::zugferd_2_3_2::Item::AirlineTariff6d => Ok(Item::AirlineTariff6d),
            crate::zugferd_2_3_2::Item::Title49CodeFederalRegulations => {
                Ok(Item::Title49CodeFederalRegulations)
            }
            crate::zugferd_2_3_2::Item::InternationalCivilAviationAdministrationCode => {
                Ok(Item::InternationalCivilAviationAdministrationCode)
            }
            crate::zugferd_2_3_2::Item::HazardousMaterialsIdDot => {
                Ok(Item::HazardousMaterialsIdDot)
            }
            crate::zugferd_2_3_2::Item::Endorsement => Ok(Item::Endorsement),
            crate::zugferd_2_3_2::Item::AirForceRegulation714 => Ok(Item::AirForceRegulation714),
            crate::zugferd_2_3_2::Item::Breed => Ok(Item::Breed),
            crate::zugferd_2_3_2::Item::ChemicalAbstractServiceCasRegistryNumber => {
                Ok(Item::ChemicalAbstractServiceCasRegistryNumber)
            }
            crate::zugferd_2_3_2::Item::EngineModelDesignation => Ok(Item::EngineModelDesignation),
            crate::zugferd_2_3_2::Item::InstitutionalMeatPurchaseSpecificationsImpsNumber => {
                Ok(Item::InstitutionalMeatPurchaseSpecificationsImpsNumber)
            }
            crate::zugferd_2_3_2::Item::PriceLookUpCodePlu => Ok(Item::PriceLookUpCodePlu),
            crate::zugferd_2_3_2::Item::InternationalMaritimeOrganizationImoCode => {
                Ok(Item::InternationalMaritimeOrganizationImoCode)
            }
            crate::zugferd_2_3_2::Item::BureauExplosives600ARail => {
                Ok(Item::BureauExplosives600ARail)
            }
            crate::zugferd_2_3_2::Item::UnitedNationsDangerousGoodsList => {
                Ok(Item::UnitedNationsDangerousGoodsList)
            }
            crate::zugferd_2_3_2::Item::InternationalCodeBotanicalNomenclatureIcbn => {
                Ok(Item::InternationalCodeBotanicalNomenclatureIcbn)
            }
            crate::zugferd_2_3_2::Item::InternationalCodeZoologicalNomenclatureIczn => {
                Ok(Item::InternationalCodeZoologicalNomenclatureIczn)
            }
            crate::zugferd_2_3_2::Item::InternationalCodeNomenclatureForCultivatedPlants => {
                Ok(Item::InternationalCodeNomenclatureForCultivatedPlants)
            }
            crate::zugferd_2_3_2::Item::DistributorSArticleIdentifier => {
                Ok(Item::DistributorSArticleIdentifier)
            }
            crate::zugferd_2_3_2::Item::NorwegianClassificationSystemEnva => {
                Ok(Item::NorwegianClassificationSystemEnva)
            }
            crate::zugferd_2_3_2::Item::SupplierAssignedClassification => {
                Ok(Item::SupplierAssignedClassification)
            }
            crate::zugferd_2_3_2::Item::MexicanClassificationSystemAmece => {
                Ok(Item::MexicanClassificationSystemAmece)
            }
            crate::zugferd_2_3_2::Item::GermanClassificationSystemCcg => {
                Ok(Item::GermanClassificationSystemCcg)
            }
            crate::zugferd_2_3_2::Item::FinnishClassificationSystemEanfin => {
                Ok(Item::FinnishClassificationSystemEanfin)
            }
            crate::zugferd_2_3_2::Item::CanadianClassificationSystemIcc => {
                Ok(Item::CanadianClassificationSystemIcc)
            }
            crate::zugferd_2_3_2::Item::FrenchClassificationSystemIfls5 => {
                Ok(Item::FrenchClassificationSystemIfls5)
            }
            crate::zugferd_2_3_2::Item::StyleNumber => Ok(Item::StyleNumber),
            crate::zugferd_2_3_2::Item::DutchClassificationSystemCbl => {
                Ok(Item::DutchClassificationSystemCbl)
            }
            crate::zugferd_2_3_2::Item::JapaneseClassificationSystemJicfs => {
                Ok(Item::JapaneseClassificationSystemJicfs)
            }
            crate::zugferd_2_3_2::Item::EuropeanUnionDairySubsidyEligibilityClassification => {
                Ok(Item::EuropeanUnionDairySubsidyEligibilityClassification)
            }
            crate::zugferd_2_3_2::Item::Gs1SpainClassificationSystem => {
                Ok(Item::Gs1SpainClassificationSystem)
            }
            crate::zugferd_2_3_2::Item::Gs1PolandClassificationSystem => {
                Ok(Item::Gs1PolandClassificationSystem)
            }
            crate::zugferd_2_3_2::Item::FederalAgencyOnTechnicalRegulatingAndMetrology => {
                Ok(Item::FederalAgencyOnTechnicalRegulatingAndMetrology)
            }
            crate::zugferd_2_3_2::Item::EfficientConsumerResponseEcrAustriaClassification => {
                Ok(Item::EfficientConsumerResponseEcrAustriaClassification)
            }
            crate::zugferd_2_3_2::Item::Gs1ItalyClassificationSystem => {
                Ok(Item::Gs1ItalyClassificationSystem)
            }
            crate::zugferd_2_3_2::Item::CpvCommonProcurementVocabulary => {
                Ok(Item::CpvCommonProcurementVocabulary)
            }
            crate::zugferd_2_3_2::Item::IfdaInternationalFoodserviceDistributorsAssociation => {
                Ok(Item::IfdaInternationalFoodserviceDistributorsAssociation)
            }
            crate::zugferd_2_3_2::Item::AhfsAmericanHospitalFormularyServicePharmacologic => {
                Ok(Item::AhfsAmericanHospitalFormularyServicePharmacologic)
            }
            crate::zugferd_2_3_2::Item::AtcAnatomicalTherapeuticChemicalClassificationSystem => {
                Ok(Item::AtcAnatomicalTherapeuticChemicalClassificationSystem)
            }
            crate::zugferd_2_3_2::Item::CladimedClassificationDesDispositifsMédicaux => {
                Ok(Item::CladimedClassificationDesDispositifsMédicaux)
            }
            crate::zugferd_2_3_2::Item::CmdrCanadianMedicalDeviceRegulationsClassification => {
                Ok(Item::CmdrCanadianMedicalDeviceRegulationsClassification)
            }
            crate::zugferd_2_3_2::Item::CndmClassificazioneNazionaleDeiDispositiviMedici => {
                Ok(Item::CndmClassificazioneNazionaleDeiDispositiviMedici)
            }
            crate::zugferd_2_3_2::Item::UkDmDDictionaryMedicinesDevicesStandardCoding => {
                Ok(Item::UkDmDDictionaryMedicinesDevicesStandardCoding)
            }
            crate::zugferd_2_3_2::Item::EclSs => Ok(Item::EclSs),
            crate::zugferd_2_3_2::Item::EdmaEuropeanDiagnosticManufacturersAssociation => {
                Ok(Item::EdmaEuropeanDiagnosticManufacturersAssociation)
            }
            crate::zugferd_2_3_2::Item::EgarEuropeanGenericArticleRegister => {
                Ok(Item::EgarEuropeanGenericArticleRegister)
            }
            crate::zugferd_2_3_2::Item::GmdnGlobalMedicalDevicesNomenclature => {
                Ok(Item::GmdnGlobalMedicalDevicesNomenclature)
            }
            crate::zugferd_2_3_2::Item::GpiGenericProductIdentifier => {
                Ok(Item::GpiGenericProductIdentifier)
            }
            crate::zugferd_2_3_2::Item::HcpcsHealthcareCommonProcedureCodingSystem => {
                Ok(Item::HcpcsHealthcareCommonProcedureCodingSystem)
            }
            crate::zugferd_2_3_2::Item::IcpsInternationalClassificationForPatientSafety => {
                Ok(Item::IcpsInternationalClassificationForPatientSafety)
            }
            crate::zugferd_2_3_2::Item::MeddraMedicalDictionaryForRegulatoryActivities => {
                Ok(Item::MeddraMedicalDictionaryForRegulatoryActivities)
            }
            crate::zugferd_2_3_2::Item::MedicalColumbus => Ok(Item::MedicalColumbus),
            crate::zugferd_2_3_2::Item::NapcsNorthAmericanProductClassificationSystem => {
                Ok(Item::NapcsNorthAmericanProductClassificationSystem)
            }
            crate::zugferd_2_3_2::Item::NhsNationalHealthServicesEclass => {
                Ok(Item::NhsNationalHealthServicesEclass)
            }
            crate::zugferd_2_3_2::Item::UsFdaFoodAndDrugAdministrationProductCode => {
                Ok(Item::UsFdaFoodAndDrugAdministrationProductCode)
            }
            crate::zugferd_2_3_2::Item::SnomedCtSystematizedNomenclatureMedicineClinical => {
                Ok(Item::SnomedCtSystematizedNomenclatureMedicineClinical)
            }
            crate::zugferd_2_3_2::Item::UmdnsUniversalMedicalDeviceNomenclatureSystem => {
                Ok(Item::UmdnsUniversalMedicalDeviceNomenclatureSystem)
            }
            crate::zugferd_2_3_2::Item::Gs1GlobalReturnableAssetIdentifierNonSerialised => {
                Ok(Item::Gs1GlobalReturnableAssetIdentifierNonSerialised)
            }
            crate::zugferd_2_3_2::Item::Imei => Ok(Item::Imei),
            crate::zugferd_2_3_2::Item::WasteTypeEmsa => Ok(Item::WasteTypeEmsa),
            crate::zugferd_2_3_2::Item::ShipsStoreClassificationType => {
                Ok(Item::ShipsStoreClassificationType)
            }
            crate::zugferd_2_3_2::Item::EmergencyFireCode => Ok(Item::EmergencyFireCode),
            crate::zugferd_2_3_2::Item::EmergencySpillageCode => Ok(Item::EmergencySpillageCode),
            crate::zugferd_2_3_2::Item::ImdgPackingGroup => Ok(Item::ImdgPackingGroup),
            crate::zugferd_2_3_2::Item::MarpolCodeIbc => Ok(Item::MarpolCodeIbc),
            crate::zugferd_2_3_2::Item::ImdgSubsidiaryRiskClass => {
                Ok(Item::ImdgSubsidiaryRiskClass)
            }
            crate::zugferd_2_3_2::Item::TransportGroupNumber => Ok(Item::TransportGroupNumber),
            crate::zugferd_2_3_2::Item::TaxonomicSerialNumber => Ok(Item::TaxonomicSerialNumber),
            crate::zugferd_2_3_2::Item::ImdgMainHazardClass => Ok(Item::ImdgMainHazardClass),
            crate::zugferd_2_3_2::Item::EuCombinedNomenclature => Ok(Item::EuCombinedNomenclature),
            crate::zugferd_2_3_2::Item::TherapeuticClassificationNumber => {
                Ok(Item::TherapeuticClassificationNumber)
            }
            crate::zugferd_2_3_2::Item::EuropeanWasteCatalogue => Ok(Item::EuropeanWasteCatalogue),
            crate::zugferd_2_3_2::Item::PriceGroupingCode => Ok(Item::PriceGroupingCode),
            crate::zugferd_2_3_2::Item::Unspsc => Ok(Item::Unspsc),
            crate::zugferd_2_3_2::Item::EuRohsDirective => Ok(Item::EuRohsDirective),
            crate::zugferd_2_3_2::Item::UltimateCustomersArticleNumber => {
                Ok(Item::UltimateCustomersArticleNumber)
            }
            crate::zugferd_2_3_2::Item::UpcUniversalProductCode => {
                Ok(Item::UpcUniversalProductCode)
            }
            crate::zugferd_2_3_2::Item::VendorItemNumber => Ok(Item::VendorItemNumber),
            crate::zugferd_2_3_2::Item::VendorsSellersPartNumber => {
                Ok(Item::VendorsSellersPartNumber)
            }
            crate::zugferd_2_3_2::Item::VendorsSupplementalItemNumber => {
                Ok(Item::VendorsSupplementalItemNumber)
            }
            crate::zugferd_2_3_2::Item::VendorSpecificationNumber => {
                Ok(Item::VendorSpecificationNumber)
            }
            crate::zugferd_2_3_2::Item::MutuallyDefined => Ok(Item::MutuallyDefined),
        }
    }
}
// End: (Version) TryFrom crate::zugferd_2_3_2::Item to Item
