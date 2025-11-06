export enum Item {
  /**
   * Product version number
   */
  ProductVersionNumber = "AA",
  /**
   * Assembly
   */
  Assembly = "AB",
  /**
   * HIBC (Health Industry Bar Code)
   */
  HibcHealthIndustryBarCode = "AC",
  /**
   * Cold roll number
   */
  ColdRollNumber = "AD",
  /**
   * Hot roll number
   */
  HotRollNumber = "AE",
  /**
   * Slab number
   */
  SlabNumber = "AF",
  /**
   * Software revision number
   */
  SoftwareRevisionNumber = "AG",
  /**
   * UPC (Universal Product Code) Consumer package code (1-5-5)
   */
  UpcUniversalProductCodeConsumerPackageCode155 = "AH",
  /**
   * UPC (Universal Product Code) Consumer package code (1-5-5-
   */
  UpcUniversalProductCodeConsumerPackageCode155_Dup = "AI",
  /**
   * Sample number
   */
  SampleNumber = "AJ",
  /**
   * Pack number
   */
  PackNumber = "AK",
  /**
   * UPC (Universal Product Code) Shipping container code (1-2-
   */
  UpcUniversalProductCodeShippingContainerCode12 = "AL",
  /**
   * UPC (Universal Product Code)/EAN (European article number)
   */
  UpcUniversalProductCodeEanEuropeanArticleNumber = "AM",
  /**
   * UPC (Universal Product Code) suffix
   */
  UpcUniversalProductCodeSuffix = "AN",
  /**
   * State label code
   */
  StateLabelCode = "AO",
  /**
   * Heat number
   */
  HeatNumber = "AP",
  /**
   * Coupon number
   */
  CouponNumber = "AQ",
  /**
   * Resource number
   */
  ResourceNumber = "AR",
  /**
   * Work task number
   */
  WorkTaskNumber = "AS",
  /**
   * Price look up number
   */
  PriceLookUpNumber = "AT",
  /**
   * NSN (North Atlantic Treaty Organization Stock Number)
   */
  NsnNorthAtlanticTreatyOrganizationStockNumber = "AU",
  /**
   * Refined product code
   */
  RefinedProductCode = "AV",
  /**
   * Exhibit
   */
  Exhibit = "AW",
  /**
   * End item
   */
  EndItem = "AX",
  /**
   * Federal supply classification
   */
  FederalSupplyClassification = "AY",
  /**
   * Engineering data list
   */
  EngineeringDataList = "AZ",
  /**
   * Milestone event number
   */
  MilestoneEventNumber = "BA",
  /**
   * Lot number
   */
  LotNumber = "BB",
  /**
   * National drug code 4-4-2 format
   */
  NationalDrugCode442Format = "BC",
  /**
   * National drug code 5-3-2 format
   */
  NationalDrugCode532Format = "BD",
  /**
   * National drug code 5-4-1 format
   */
  NationalDrugCode541Format = "BE",
  /**
   * National drug code 5-4-2 format
   */
  NationalDrugCode542Format = "BF",
  /**
   * National drug code
   */
  NationalDrugCode = "BG",
  /**
   * Part number
   */
  PartNumber = "BH",
  /**
   * Local Stock Number (LSN)
   */
  LocalStockNumberLsn = "BI",
  /**
   * Next higher assembly number
   */
  NextHigherAssemblyNumber = "BJ",
  /**
   * Data category
   */
  DataCategory = "BK",
  /**
   * Control number
   */
  ControlNumber = "BL",
  /**
   * Special material identification code
   */
  SpecialMaterialIdentificationCode = "BM",
  /**
   * Locally assigned control number
   */
  LocallyAssignedControlNumber = "BN",
  /**
   * Buyer's colour
   */
  BuyersColour = "BO",
  /**
   * Buyer's part number
   */
  BuyersPartNumber = "BP",
  /**
   * Variable measure product code
   */
  VariableMeasureProductCode = "BQ",
  /**
   * Financial phase
   */
  FinancialPhase = "BR",
  /**
   * Contract breakdown
   */
  ContractBreakdown = "BS",
  /**
   * Technical phase
   */
  TechnicalPhase = "BT",
  /**
   * Dye lot number
   */
  DyeLotNumber = "BU",
  /**
   * Daily statement of activities
   */
  DailyStatementActivities = "BV",
  /**
   * Periodical statement of activities within a bilaterally
   */
  PeriodicalStatementActivitiesWithinABilaterally = "BW",
  /**
   * Calendar week statement of activities
   */
  CalendarWeekStatementActivities = "BX",
  /**
   * Calendar month statement of activities
   */
  CalendarMonthStatementActivities = "BY",
  /**
   * Original equipment number
   */
  OriginalEquipmentNumber = "BZ",
  /**
   * Industry commodity code
   */
  IndustryCommodityCode = "CC",
  /**
   * Commodity grouping
   */
  CommodityGrouping = "CG",
  /**
   * Colour number
   */
  ColourNumber = "CL",
  /**
   * Contract number
   */
  ContractNumber = "CR",
  /**
   * Customs article number
   */
  CustomsArticleNumber = "CV",
  /**
   * Drawing revision number
   */
  DrawingRevisionNumber = "DR",
  /**
   * Drawing
   */
  Drawing = "DW",
  /**
   * Engineering change level
   */
  EngineeringChangeLevel = "EC",
  /**
   * Material code
   */
  MaterialCode = "EF",
  /**
   * EMDN (European Medical Device Nomenclature)
   */
  EmdnEuropeanMedicalDeviceNomenclature = "EMD",
  /**
   * International Article Numbering Association (EAN)
   */
  InternationalArticleNumberingAssociationEan = "EN",
  /**
   * Fish species
   */
  FishSpecies = "FS",
  /**
   * Buyer's internal product group code
   */
  BuyersInternalProductGroupCode = "GB",
  /**
   * Global model number
   */
  GlobalModelNumber = "GMN",
  /**
   * National product group code
   */
  NationalProductGroupCode = "GN",
  /**
   * General specification number
   */
  GeneralSpecificationNumber = "GS",
  /**
   * Harmonised system
   */
  HarmonisedSystem = "HS",
  /**
   * ISBN (International Standard Book Number)
   */
  IsbnInternationalStandardBookNumber = "IB",
  /**
   * Buyer's item number
   */
  BuyersItemNumber = "IN",
  /**
   * ISSN (International Standard Serial Number)
   */
  IssnInternationalStandardSerialNumber = "IS",
  /**
   * Buyer's style number
   */
  BuyersStyleNumber = "IT",
  /**
   * Buyer's size code
   */
  BuyersSizeCode = "IZ",
  /**
   * Machine number
   */
  MachineNumber = "MA",
  /**
   * Manufacturer's (producer's) article number
   */
  ManufacturersProducersArticleNumber = "MF",
  /**
   * Model number
   */
  ModelNumber = "MN",
  /**
   * Product/service identification number
   */
  ProductServiceIdentificationNumber = "MP",
  /**
   * Batch number
   */
  BatchNumber = "NB",
  /**
   * Customer order number
   */
  CustomerOrderNumber = "ON",
  /**
   * Part number description
   */
  PartNumberDescription = "PD",
  /**
   * Purchaser's order line number
   */
  PurchasersOrderLineNumber = "PL",
  /**
   * Purchase order number
   */
  PurchaseOrderNumber = "PO",
  /**
   * Promotional variant number
   */
  PromotionalVariantNumber = "PV",
  /**
   * Buyer's qualifier for size
   */
  BuyersQualifierForSize = "QS",
  /**
   * Returnable container number
   */
  ReturnableContainerNumber = "RC",
  /**
   * Release number
   */
  ReleaseNumber = "RN",
  /**
   * Run number
   */
  RunNumber = "RU",
  /**
   * Record keeping of model year
   */
  RecordKeepingModelYear = "RY",
  /**
   * Supplier's article number
   */
  SuppliersArticleNumber = "SA",
  /**
   * Standard group of products (mixed assortment)
   */
  StandardGroupProductsMixedAssortment = "SG",
  /**
   * SKU (Stock keeping unit)
   */
  SkuStockKeepingUnit = "SK",
  /**
   * Serial number
   */
  SerialNumber = "SN",
  /**
   * RSK number
   */
  RskNumber = "SRS",
  /**
   * IFLS (Institut Francais du Libre Service) 5 digit product
   */
  IflsInstitutFrancaisDuLibreService5DigitProduct = "SRT",
  /**
   * IFLS (Institut Francais du Libre Service) 9 digit product
   */
  IflsInstitutFrancaisDuLibreService9DigitProduct = "SRU",
  /**
   * GS1 Global Trade Item Number
   */
  Gs1GlobalTradeItemNumber = "SRV",
  /**
   * EDIS (Energy Data Identification System)
   */
  EdisEnergyDataIdentificationSystem = "SRW",
  /**
   * Slaughter number
   */
  SlaughterNumber = "SRX",
  /**
   * Official animal number
   */
  OfficialAnimalNumber = "SRY",
  /**
   * Harmonized tariff schedule
   */
  HarmonizedTariffSchedule = "SRZ",
  /**
   * Supplier's supplier article number
   */
  SuppliersSupplierArticleNumber = "SS",
  /**
   * 46 Level DOT Code
   */
  _46LevelDotCode = "SSA",
  /**
   * Airline Tariff 6D
   */
  AirlineTariff6d = "SSB",
  /**
   * Title 49 Code of Federal Regulations
   */
  Title49CodeFederalRegulations = "SSC",
  /**
   * International Civil Aviation Administration code
   */
  InternationalCivilAviationAdministrationCode = "SSD",
  /**
   * Hazardous Materials ID DOT
   */
  HazardousMaterialsIdDot = "SSE",
  /**
   * Endorsement
   */
  Endorsement = "SSF",
  /**
   * Air Force Regulation 71-4
   */
  AirForceRegulation714 = "SSG",
  /**
   * Breed
   */
  Breed = "SSH",
  /**
   * Chemical Abstract Service (CAS) registry number
   */
  ChemicalAbstractServiceCasRegistryNumber = "SSI",
  /**
   * Engine model designation
   */
  EngineModelDesignation = "SSJ",
  /**
   * Institutional Meat Purchase Specifications (IMPS) Number
   */
  InstitutionalMeatPurchaseSpecificationsImpsNumber = "SSK",
  /**
   * Price Look-Up code (PLU)
   */
  PriceLookUpCodePlu = "SSL",
  /**
   * International Maritime Organization (IMO) Code
   */
  InternationalMaritimeOrganizationImoCode = "SSM",
  /**
   * Bureau of Explosives 600-A (rail)
   */
  BureauExplosives600ARail = "SSN",
  /**
   * United Nations Dangerous Goods List
   */
  UnitedNationsDangerousGoodsList = "SSO",
  /**
   * International Code of Botanical Nomenclature (ICBN)
   */
  InternationalCodeBotanicalNomenclatureIcbn = "SSP",
  /**
   * International Code of Zoological Nomenclature (ICZN)
   */
  InternationalCodeZoologicalNomenclatureIczn = "SSQ",
  /**
   * International Code of Nomenclature for Cultivated Plants
   */
  InternationalCodeNomenclatureForCultivatedPlants = "SSR",
  /**
   * Distributor’s article identifier
   */
  DistributorSArticleIdentifier = "SSS",
  /**
   * Norwegian Classification system ENVA
   */
  NorwegianClassificationSystemEnva = "SST",
  /**
   * Supplier assigned classification
   */
  SupplierAssignedClassification = "SSU",
  /**
   * Mexican classification system AMECE
   */
  MexicanClassificationSystemAmece = "SSV",
  /**
   * German classification system CCG
   */
  GermanClassificationSystemCcg = "SSW",
  /**
   * Finnish classification system EANFIN
   */
  FinnishClassificationSystemEanfin = "SSX",
  /**
   * Canadian classification system ICC
   */
  CanadianClassificationSystemIcc = "SSY",
  /**
   * French classification system IFLS5
   */
  FrenchClassificationSystemIfls5 = "SSZ",
  /**
   * Style number
   */
  StyleNumber = "ST",
  /**
   * Dutch classification system CBL
   */
  DutchClassificationSystemCbl = "STA",
  /**
   * Japanese classification system JICFS
   */
  JapaneseClassificationSystemJicfs = "STB",
  /**
   * European Union dairy subsidy eligibility classification
   */
  EuropeanUnionDairySubsidyEligibilityClassification = "STC",
  /**
   * GS1 Spain classification system
   */
  Gs1SpainClassificationSystem = "STD",
  /**
   * GS1 Poland classification system
   */
  Gs1PolandClassificationSystem = "STE",
  /**
   * Federal Agency on Technical Regulating and Metrology of the
   */
  FederalAgencyOnTechnicalRegulatingAndMetrology = "STF",
  /**
   * Efficient Consumer Response (ECR) Austria classification
   */
  EfficientConsumerResponseEcrAustriaClassification = "STG",
  /**
   * GS1 Italy classification system
   */
  Gs1ItalyClassificationSystem = "STH",
  /**
   * CPV (Common Procurement Vocabulary)
   */
  CpvCommonProcurementVocabulary = "STI",
  /**
   * IFDA (International Foodservice Distributors Association)
   */
  IfdaInternationalFoodserviceDistributorsAssociation = "STJ",
  /**
   * AHFS (American Hospital Formulary Service) pharmacologic -
   */
  AhfsAmericanHospitalFormularyServicePharmacologic = "STK",
  /**
   * ATC (Anatomical Therapeutic Chemical) classification system
   */
  AtcAnatomicalTherapeuticChemicalClassificationSystem = "STL",
  /**
   * CLADIMED (Classification des Dispositifs Médicaux)
   */
  CladimedClassificationDesDispositifsMédicaux = "STM",
  /**
   * CMDR (Canadian Medical Device Regulations) classification
   */
  CmdrCanadianMedicalDeviceRegulationsClassification = "STN",
  /**
   * CNDM (Classificazione Nazionale dei Dispositivi Medici)
   */
  CndmClassificazioneNazionaleDeiDispositiviMedici = "STO",
  /**
   * UK DM&D (Dictionary of Medicines & Devices) standard coding
   */
  UkDmDDictionaryMedicinesDevicesStandardCoding = "STP",
  /**
   * eCl@ss
   */
  EclSs = "STQ",
  /**
   * EDMA (European Diagnostic Manufacturers Association)
   */
  EdmaEuropeanDiagnosticManufacturersAssociation = "STR",
  /**
   * EGAR (European Generic Article Register)
   */
  EgarEuropeanGenericArticleRegister = "STS",
  /**
   * GMDN (Global Medical Devices Nomenclature)
   */
  GmdnGlobalMedicalDevicesNomenclature = "STT",
  /**
   * GPI (Generic Product Identifier)
   */
  GpiGenericProductIdentifier = "STU",
  /**
   * HCPCS (Healthcare Common Procedure Coding System)
   */
  HcpcsHealthcareCommonProcedureCodingSystem = "STV",
  /**
   * ICPS (International Classification for Patient Safety)
   */
  IcpsInternationalClassificationForPatientSafety = "STW",
  /**
   * MedDRA (Medical Dictionary for Regulatory Activities)
   */
  MeddraMedicalDictionaryForRegulatoryActivities = "STX",
  /**
   * Medical Columbus
   */
  MedicalColumbus = "STY",
  /**
   * NAPCS (North American Product Classification System)
   */
  NapcsNorthAmericanProductClassificationSystem = "STZ",
  /**
   * NHS (National Health Services) eClass
   */
  NhsNationalHealthServicesEclass = "SUA",
  /**
   * US FDA (Food and Drug Administration) Product Code
   */
  UsFdaFoodAndDrugAdministrationProductCode = "SUB",
  /**
   * SNOMED CT (Systematized Nomenclature of Medicine-Clinical
   */
  SnomedCtSystematizedNomenclatureMedicineClinical = "SUC",
  /**
   * UMDNS (Universal Medical Device Nomenclature System)
   */
  UmdnsUniversalMedicalDeviceNomenclatureSystem = "SUD",
  /**
   * GS1 Global Returnable Asset Identifier, non-serialised
   */
  Gs1GlobalReturnableAssetIdentifierNonSerialised = "SUE",
  /**
   * IMEI
   */
  Imei = "SUF",
  /**
   * Waste Type (EMSA)
   */
  WasteTypeEmsa = "SUG",
  /**
   * Ship's store classification type
   */
  ShipsStoreClassificationType = "SUH",
  /**
   * Emergency fire code
   */
  EmergencyFireCode = "SUI",
  /**
   * Emergency spillage code
   */
  EmergencySpillageCode = "SUJ",
  /**
   * IMDG packing group
   */
  ImdgPackingGroup = "SUK",
  /**
   * MARPOL Code IBC
   */
  MarpolCodeIbc = "SUL",
  /**
   * IMDG subsidiary risk class
   */
  ImdgSubsidiaryRiskClass = "SUM",
  /**
   * Transport group number
   */
  TransportGroupNumber = "TG",
  /**
   * Taxonomic Serial Number
   */
  TaxonomicSerialNumber = "TSN",
  /**
   * IMDG main hazard class
   */
  ImdgMainHazardClass = "TSO",
  /**
   * EU Combined Nomenclature
   */
  EuCombinedNomenclature = "TSP",
  /**
   * Therapeutic classification number
   */
  TherapeuticClassificationNumber = "TSQ",
  /**
   * European Waste Catalogue
   */
  EuropeanWasteCatalogue = "TSR",
  /**
   * Price grouping code
   */
  PriceGroupingCode = "TSS",
  /**
   * UNSPSC
   */
  Unspsc = "TST",
  /**
   * EU RoHS Directive
   */
  EuRohsDirective = "TSU",
  /**
   * Ultimate customer's article number
   */
  UltimateCustomersArticleNumber = "UA",
  /**
   * UPC (Universal product code)
   */
  UpcUniversalProductCode = "UP",
  /**
   * Vendor item number
   */
  VendorItemNumber = "VN",
  /**
   * Vendor's (seller's) part number
   */
  VendorsSellersPartNumber = "VP",
  /**
   * Vendor's supplemental item number
   */
  VendorsSupplementalItemNumber = "VS",
  /**
   * Vendor specification number
   */
  VendorSpecificationNumber = "VX",
  /**
   * Mutually defined
   */
  MutuallyDefined = "ZZZ",
}

export function description(value: Item): string {
  switch (value) {
    case Item.ProductVersionNumber:
      return "Product version number";
    case Item.Assembly:
      return "Assembly";
    case Item.HibcHealthIndustryBarCode:
      return "HIBC (Health Industry Bar Code)";
    case Item.ColdRollNumber:
      return "Cold roll number";
    case Item.HotRollNumber:
      return "Hot roll number";
    case Item.SlabNumber:
      return "Slab number";
    case Item.SoftwareRevisionNumber:
      return "Software revision number";
    case Item.UpcUniversalProductCodeConsumerPackageCode155:
      return "UPC (Universal Product Code) Consumer package code (1-5-5)";
    case Item.UpcUniversalProductCodeConsumerPackageCode155_Dup:
      return "UPC (Universal Product Code) Consumer package code (1-5-5-";
    case Item.SampleNumber:
      return "Sample number";
    case Item.PackNumber:
      return "Pack number";
    case Item.UpcUniversalProductCodeShippingContainerCode12:
      return "UPC (Universal Product Code) Shipping container code (1-2-";
    case Item.UpcUniversalProductCodeEanEuropeanArticleNumber:
      return "UPC (Universal Product Code)/EAN (European article number)";
    case Item.UpcUniversalProductCodeSuffix:
      return "UPC (Universal Product Code) suffix";
    case Item.StateLabelCode:
      return "State label code";
    case Item.HeatNumber:
      return "Heat number";
    case Item.CouponNumber:
      return "Coupon number";
    case Item.ResourceNumber:
      return "Resource number";
    case Item.WorkTaskNumber:
      return "Work task number";
    case Item.PriceLookUpNumber:
      return "Price look up number";
    case Item.NsnNorthAtlanticTreatyOrganizationStockNumber:
      return "NSN (North Atlantic Treaty Organization Stock Number)";
    case Item.RefinedProductCode:
      return "Refined product code";
    case Item.Exhibit:
      return "Exhibit";
    case Item.EndItem:
      return "End item";
    case Item.FederalSupplyClassification:
      return "Federal supply classification";
    case Item.EngineeringDataList:
      return "Engineering data list";
    case Item.MilestoneEventNumber:
      return "Milestone event number";
    case Item.LotNumber:
      return "Lot number";
    case Item.NationalDrugCode442Format:
      return "National drug code 4-4-2 format";
    case Item.NationalDrugCode532Format:
      return "National drug code 5-3-2 format";
    case Item.NationalDrugCode541Format:
      return "National drug code 5-4-1 format";
    case Item.NationalDrugCode542Format:
      return "National drug code 5-4-2 format";
    case Item.NationalDrugCode:
      return "National drug code";
    case Item.PartNumber:
      return "Part number";
    case Item.LocalStockNumberLsn:
      return "Local Stock Number (LSN)";
    case Item.NextHigherAssemblyNumber:
      return "Next higher assembly number";
    case Item.DataCategory:
      return "Data category";
    case Item.ControlNumber:
      return "Control number";
    case Item.SpecialMaterialIdentificationCode:
      return "Special material identification code";
    case Item.LocallyAssignedControlNumber:
      return "Locally assigned control number";
    case Item.BuyersColour:
      return "Buyer's colour";
    case Item.BuyersPartNumber:
      return "Buyer's part number";
    case Item.VariableMeasureProductCode:
      return "Variable measure product code";
    case Item.FinancialPhase:
      return "Financial phase";
    case Item.ContractBreakdown:
      return "Contract breakdown";
    case Item.TechnicalPhase:
      return "Technical phase";
    case Item.DyeLotNumber:
      return "Dye lot number";
    case Item.DailyStatementActivities:
      return "Daily statement of activities";
    case Item.PeriodicalStatementActivitiesWithinABilaterally:
      return "Periodical statement of activities within a bilaterally";
    case Item.CalendarWeekStatementActivities:
      return "Calendar week statement of activities";
    case Item.CalendarMonthStatementActivities:
      return "Calendar month statement of activities";
    case Item.OriginalEquipmentNumber:
      return "Original equipment number";
    case Item.IndustryCommodityCode:
      return "Industry commodity code";
    case Item.CommodityGrouping:
      return "Commodity grouping";
    case Item.ColourNumber:
      return "Colour number";
    case Item.ContractNumber:
      return "Contract number";
    case Item.CustomsArticleNumber:
      return "Customs article number";
    case Item.DrawingRevisionNumber:
      return "Drawing revision number";
    case Item.Drawing:
      return "Drawing";
    case Item.EngineeringChangeLevel:
      return "Engineering change level";
    case Item.MaterialCode:
      return "Material code";
    case Item.EmdnEuropeanMedicalDeviceNomenclature:
      return "EMDN (European Medical Device Nomenclature)";
    case Item.InternationalArticleNumberingAssociationEan:
      return "International Article Numbering Association (EAN)";
    case Item.FishSpecies:
      return "Fish species";
    case Item.BuyersInternalProductGroupCode:
      return "Buyer's internal product group code";
    case Item.GlobalModelNumber:
      return "Global model number";
    case Item.NationalProductGroupCode:
      return "National product group code";
    case Item.GeneralSpecificationNumber:
      return "General specification number";
    case Item.HarmonisedSystem:
      return "Harmonised system";
    case Item.IsbnInternationalStandardBookNumber:
      return "ISBN (International Standard Book Number)";
    case Item.BuyersItemNumber:
      return "Buyer's item number";
    case Item.IssnInternationalStandardSerialNumber:
      return "ISSN (International Standard Serial Number)";
    case Item.BuyersStyleNumber:
      return "Buyer's style number";
    case Item.BuyersSizeCode:
      return "Buyer's size code";
    case Item.MachineNumber:
      return "Machine number";
    case Item.ManufacturersProducersArticleNumber:
      return "Manufacturer's (producer's) article number";
    case Item.ModelNumber:
      return "Model number";
    case Item.ProductServiceIdentificationNumber:
      return "Product/service identification number";
    case Item.BatchNumber:
      return "Batch number";
    case Item.CustomerOrderNumber:
      return "Customer order number";
    case Item.PartNumberDescription:
      return "Part number description";
    case Item.PurchasersOrderLineNumber:
      return "Purchaser's order line number";
    case Item.PurchaseOrderNumber:
      return "Purchase order number";
    case Item.PromotionalVariantNumber:
      return "Promotional variant number";
    case Item.BuyersQualifierForSize:
      return "Buyer's qualifier for size";
    case Item.ReturnableContainerNumber:
      return "Returnable container number";
    case Item.ReleaseNumber:
      return "Release number";
    case Item.RunNumber:
      return "Run number";
    case Item.RecordKeepingModelYear:
      return "Record keeping of model year";
    case Item.SuppliersArticleNumber:
      return "Supplier's article number";
    case Item.StandardGroupProductsMixedAssortment:
      return "Standard group of products (mixed assortment)";
    case Item.SkuStockKeepingUnit:
      return "SKU (Stock keeping unit)";
    case Item.SerialNumber:
      return "Serial number";
    case Item.RskNumber:
      return "RSK number";
    case Item.IflsInstitutFrancaisDuLibreService5DigitProduct:
      return "IFLS (Institut Francais du Libre Service) 5 digit product";
    case Item.IflsInstitutFrancaisDuLibreService9DigitProduct:
      return "IFLS (Institut Francais du Libre Service) 9 digit product";
    case Item.Gs1GlobalTradeItemNumber:
      return "GS1 Global Trade Item Number";
    case Item.EdisEnergyDataIdentificationSystem:
      return "EDIS (Energy Data Identification System)";
    case Item.SlaughterNumber:
      return "Slaughter number";
    case Item.OfficialAnimalNumber:
      return "Official animal number";
    case Item.HarmonizedTariffSchedule:
      return "Harmonized tariff schedule";
    case Item.SuppliersSupplierArticleNumber:
      return "Supplier's supplier article number";
    case Item._46LevelDotCode:
      return "46 Level DOT Code";
    case Item.AirlineTariff6d:
      return "Airline Tariff 6D";
    case Item.Title49CodeFederalRegulations:
      return "Title 49 Code of Federal Regulations";
    case Item.InternationalCivilAviationAdministrationCode:
      return "International Civil Aviation Administration code";
    case Item.HazardousMaterialsIdDot:
      return "Hazardous Materials ID DOT";
    case Item.Endorsement:
      return "Endorsement";
    case Item.AirForceRegulation714:
      return "Air Force Regulation 71-4";
    case Item.Breed:
      return "Breed";
    case Item.ChemicalAbstractServiceCasRegistryNumber:
      return "Chemical Abstract Service (CAS) registry number";
    case Item.EngineModelDesignation:
      return "Engine model designation";
    case Item.InstitutionalMeatPurchaseSpecificationsImpsNumber:
      return "Institutional Meat Purchase Specifications (IMPS) Number";
    case Item.PriceLookUpCodePlu:
      return "Price Look-Up code (PLU)";
    case Item.InternationalMaritimeOrganizationImoCode:
      return "International Maritime Organization (IMO) Code";
    case Item.BureauExplosives600ARail:
      return "Bureau of Explosives 600-A (rail)";
    case Item.UnitedNationsDangerousGoodsList:
      return "United Nations Dangerous Goods List";
    case Item.InternationalCodeBotanicalNomenclatureIcbn:
      return "International Code of Botanical Nomenclature (ICBN)";
    case Item.InternationalCodeZoologicalNomenclatureIczn:
      return "International Code of Zoological Nomenclature (ICZN)";
    case Item.InternationalCodeNomenclatureForCultivatedPlants:
      return "International Code of Nomenclature for Cultivated Plants";
    case Item.DistributorSArticleIdentifier:
      return "Distributor’s article identifier";
    case Item.NorwegianClassificationSystemEnva:
      return "Norwegian Classification system ENVA";
    case Item.SupplierAssignedClassification:
      return "Supplier assigned classification";
    case Item.MexicanClassificationSystemAmece:
      return "Mexican classification system AMECE";
    case Item.GermanClassificationSystemCcg:
      return "German classification system CCG";
    case Item.FinnishClassificationSystemEanfin:
      return "Finnish classification system EANFIN";
    case Item.CanadianClassificationSystemIcc:
      return "Canadian classification system ICC";
    case Item.FrenchClassificationSystemIfls5:
      return "French classification system IFLS5";
    case Item.StyleNumber:
      return "Style number";
    case Item.DutchClassificationSystemCbl:
      return "Dutch classification system CBL";
    case Item.JapaneseClassificationSystemJicfs:
      return "Japanese classification system JICFS";
    case Item.EuropeanUnionDairySubsidyEligibilityClassification:
      return "European Union dairy subsidy eligibility classification";
    case Item.Gs1SpainClassificationSystem:
      return "GS1 Spain classification system";
    case Item.Gs1PolandClassificationSystem:
      return "GS1 Poland classification system";
    case Item.FederalAgencyOnTechnicalRegulatingAndMetrology:
      return "Federal Agency on Technical Regulating and Metrology of the";
    case Item.EfficientConsumerResponseEcrAustriaClassification:
      return "Efficient Consumer Response (ECR) Austria classification";
    case Item.Gs1ItalyClassificationSystem:
      return "GS1 Italy classification system";
    case Item.CpvCommonProcurementVocabulary:
      return "CPV (Common Procurement Vocabulary)";
    case Item.IfdaInternationalFoodserviceDistributorsAssociation:
      return "IFDA (International Foodservice Distributors Association)";
    case Item.AhfsAmericanHospitalFormularyServicePharmacologic:
      return "AHFS (American Hospital Formulary Service) pharmacologic -";
    case Item.AtcAnatomicalTherapeuticChemicalClassificationSystem:
      return "ATC (Anatomical Therapeutic Chemical) classification system";
    case Item.CladimedClassificationDesDispositifsMédicaux:
      return "CLADIMED (Classification des Dispositifs Médicaux)";
    case Item.CmdrCanadianMedicalDeviceRegulationsClassification:
      return "CMDR (Canadian Medical Device Regulations) classification";
    case Item.CndmClassificazioneNazionaleDeiDispositiviMedici:
      return "CNDM (Classificazione Nazionale dei Dispositivi Medici)";
    case Item.UkDmDDictionaryMedicinesDevicesStandardCoding:
      return "UK DM&D (Dictionary of Medicines & Devices) standard coding";
    case Item.EclSs:
      return "eCl@ss";
    case Item.EdmaEuropeanDiagnosticManufacturersAssociation:
      return "EDMA (European Diagnostic Manufacturers Association)";
    case Item.EgarEuropeanGenericArticleRegister:
      return "EGAR (European Generic Article Register)";
    case Item.GmdnGlobalMedicalDevicesNomenclature:
      return "GMDN (Global Medical Devices Nomenclature)";
    case Item.GpiGenericProductIdentifier:
      return "GPI (Generic Product Identifier)";
    case Item.HcpcsHealthcareCommonProcedureCodingSystem:
      return "HCPCS (Healthcare Common Procedure Coding System)";
    case Item.IcpsInternationalClassificationForPatientSafety:
      return "ICPS (International Classification for Patient Safety)";
    case Item.MeddraMedicalDictionaryForRegulatoryActivities:
      return "MedDRA (Medical Dictionary for Regulatory Activities)";
    case Item.MedicalColumbus:
      return "Medical Columbus";
    case Item.NapcsNorthAmericanProductClassificationSystem:
      return "NAPCS (North American Product Classification System)";
    case Item.NhsNationalHealthServicesEclass:
      return "NHS (National Health Services) eClass";
    case Item.UsFdaFoodAndDrugAdministrationProductCode:
      return "US FDA (Food and Drug Administration) Product Code";
    case Item.SnomedCtSystematizedNomenclatureMedicineClinical:
      return "SNOMED CT (Systematized Nomenclature of Medicine-Clinical";
    case Item.UmdnsUniversalMedicalDeviceNomenclatureSystem:
      return "UMDNS (Universal Medical Device Nomenclature System)";
    case Item.Gs1GlobalReturnableAssetIdentifierNonSerialised:
      return "GS1 Global Returnable Asset Identifier, non-serialised";
    case Item.Imei:
      return "IMEI";
    case Item.WasteTypeEmsa:
      return "Waste Type (EMSA)";
    case Item.ShipsStoreClassificationType:
      return "Ship's store classification type";
    case Item.EmergencyFireCode:
      return "Emergency fire code";
    case Item.EmergencySpillageCode:
      return "Emergency spillage code";
    case Item.ImdgPackingGroup:
      return "IMDG packing group";
    case Item.MarpolCodeIbc:
      return "MARPOL Code IBC";
    case Item.ImdgSubsidiaryRiskClass:
      return "IMDG subsidiary risk class";
    case Item.TransportGroupNumber:
      return "Transport group number";
    case Item.TaxonomicSerialNumber:
      return "Taxonomic Serial Number";
    case Item.ImdgMainHazardClass:
      return "IMDG main hazard class";
    case Item.EuCombinedNomenclature:
      return "EU Combined Nomenclature";
    case Item.TherapeuticClassificationNumber:
      return "Therapeutic classification number";
    case Item.EuropeanWasteCatalogue:
      return "European Waste Catalogue";
    case Item.PriceGroupingCode:
      return "Price grouping code";
    case Item.Unspsc:
      return "UNSPSC";
    case Item.EuRohsDirective:
      return "EU RoHS Directive";
    case Item.UltimateCustomersArticleNumber:
      return "Ultimate customer's article number";
    case Item.UpcUniversalProductCode:
      return "UPC (Universal product code)";
    case Item.VendorItemNumber:
      return "Vendor item number";
    case Item.VendorsSellersPartNumber:
      return "Vendor's (seller's) part number";
    case Item.VendorsSupplementalItemNumber:
      return "Vendor's supplemental item number";
    case Item.VendorSpecificationNumber:
      return "Vendor specification number";
    case Item.MutuallyDefined:
      return "Mutually defined";
  }
}
