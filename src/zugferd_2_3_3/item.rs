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
