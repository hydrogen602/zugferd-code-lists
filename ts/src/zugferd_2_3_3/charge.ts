
export enum Charge {
    /**
    * Advertising
    */
    Advertising = "AA",
    /**
    * Telecommunication
    */
    Telecommunication = "AAA",
    /**
    * Technical modification
    */
    TechnicalModification = "AAC",
    /**
    * Job-order production
    */
    JobOrderProduction = "AAD",
    /**
    * Outlays
    */
    Outlays = "AAE",
    /**
    * Off-premises
    */
    OffPremises = "AAF",
    /**
    * Additional processing
    */
    AdditionalProcessing = "AAH",
    /**
    * Attesting
    */
    Attesting = "AAI",
    /**
    * Acceptance
    */
    Acceptance = "AAS",
    /**
    * Rush delivery
    */
    RushDelivery = "AAT",
    /**
    * Special construction
    */
    SpecialConstruction = "AAV",
    /**
    * Airport facilities
    */
    AirportFacilities = "AAY",
    /**
    * Concession
    */
    Concession = "AAZ",
    /**
    * Compulsory storage
    */
    CompulsoryStorage = "ABA",
    /**
    * Fuel removal
    */
    FuelRemoval = "ABB",
    /**
    * Into plane
    */
    IntoPlane = "ABC",
    /**
    * Overtime
    */
    Overtime = "ABD",
    /**
    * Tooling
    */
    Tooling = "ABF",
    /**
    * Miscellaneous
    */
    Miscellaneous = "ABK",
    /**
    * Additional packaging
    */
    AdditionalPackaging = "ABL",
    /**
    * Dunnage
    */
    Dunnage = "ABN",
    /**
    * Containerisation
    */
    Containerisation = "ABR",
    /**
    * Carton packing
    */
    CartonPacking = "ABS",
    /**
    * Hessian wrapped
    */
    HessianWrapped = "ABT",
    /**
    * Polyethylene wrap packing
    */
    PolyethyleneWrapPacking = "ABU",
    /**
    * Miscellaneous treatment
    */
    MiscellaneousTreatment = "ACF",
    /**
    * Enamelling treatment
    */
    EnamellingTreatment = "ACG",
    /**
    * Heat treatment
    */
    HeatTreatment = "ACH",
    /**
    * Plating treatment
    */
    PlatingTreatment = "ACI",
    /**
    * Painting
    */
    Painting = "ACJ",
    /**
    * Polishing
    */
    Polishing = "ACK",
    /**
    * Priming
    */
    Priming = "ACL",
    /**
    * Preservation treatment
    */
    PreservationTreatment = "ACM",
    /**
    * Fitting
    */
    Fitting = "ACS",
    /**
    * Consolidation
    */
    Consolidation = "ADC",
    /**
    * Bill of lading
    */
    BillLading = "ADE",
    /**
    * Airbag
    */
    Airbag = "ADJ",
    /**
    * Transfer
    */
    Transfer = "ADK",
    /**
    * Slipsheet
    */
    Slipsheet = "ADL",
    /**
    * Binding
    */
    Binding = "ADM",
    /**
    * Repair or replacement of broken returnable package
    */
    RepairOrReplacementBrokenReturnablePackage = "ADN",
    /**
    * Efficient logistics
    */
    EfficientLogistics = "ADO",
    /**
    * Merchandising
    */
    Merchandising = "ADP",
    /**
    * Product mix
    */
    ProductMix = "ADQ",
    /**
    * Other services
    */
    OtherServices = "ADR",
    /**
    * Pick-up
    */
    PickUp = "ADT",
    /**
    * Chronic illness
    */
    ChronicIllness = "ADW",
    /**
    * New product introduction
    */
    NewProductIntroduction = "ADY",
    /**
    * Direct delivery
    */
    DirectDelivery = "ADZ",
    /**
    * Diversion
    */
    Diversion = "AEA",
    /**
    * Disconnect
    */
    Disconnect = "AEB",
    /**
    * Distribution
    */
    Distribution = "AEC",
    /**
    * Handling of hazardous cargo
    */
    HandlingHazardousCargo = "AED",
    /**
    * Rents and leases
    */
    RentsAndLeases = "AEF",
    /**
    * Location differential
    */
    LocationDifferential = "AEH",
    /**
    * Aircraft refueling
    */
    AircraftRefueling = "AEI",
    /**
    * Fuel shipped into storage
    */
    FuelShippedIntoStorage = "AEJ",
    /**
    * Cash on delivery
    */
    CashOnDelivery = "AEK",
    /**
    * Small order processing service
    */
    SmallOrderProcessingService = "AEL",
    /**
    * Clerical or administrative services
    */
    ClericalOrAdministrativeServices = "AEM",
    /**
    * Guarantee
    */
    Guarantee = "AEN",
    /**
    * Collection and recycling
    */
    CollectionAndRecycling = "AEO",
    /**
    * Copyright fee collection
    */
    CopyrightFeeCollection = "AEP",
    /**
    * Veterinary inspection service
    */
    VeterinaryInspectionService = "AES",
    /**
    * Pensioner service
    */
    PensionerService = "AET",
    /**
    * Medicine free pass holder
    */
    MedicineFreePassHolder = "AEU",
    /**
    * Environmental protection service
    */
    EnvironmentalProtectionService = "AEV",
    /**
    * Environmental clean-up service
    */
    EnvironmentalCleanUpService = "AEW",
    /**
    * National cheque processing service outside account area
    */
    NationalChequeProcessingServiceOutsideAccountArea = "AEX",
    /**
    * National payment service outside account area
    */
    NationalPaymentServiceOutsideAccountArea = "AEY",
    /**
    * National payment service within account area
    */
    NationalPaymentServiceWithinAccountArea = "AEZ",
    /**
    * Adjustments
    */
    Adjustments = "AJ",
    /**
    * Authentication
    */
    Authentication = "AU",
    /**
    * Cataloguing
    */
    Cataloguing = "CA",
    /**
    * Cartage
    */
    Cartage = "CAB",
    /**
    * Certification
    */
    Certification = "CAD",
    /**
    * Certificate of conformance
    */
    CertificateConformance = "CAE",
    /**
    * Certificate of origin
    */
    CertificateOrigin = "CAF",
    /**
    * Cutting
    */
    Cutting = "CAI",
    /**
    * Consular service
    */
    ConsularService = "CAJ",
    /**
    * Customer collection
    */
    CustomerCollection = "CAK",
    /**
    * Payroll payment service
    */
    PayrollPaymentService = "CAL",
    /**
    * Cash transportation
    */
    CashTransportation = "CAM",
    /**
    * Home banking service
    */
    HomeBankingService = "CAN",
    /**
    * Bilateral agreement service
    */
    BilateralAgreementService = "CAO",
    /**
    * Insurance brokerage service
    */
    InsuranceBrokerageService = "CAP",
    /**
    * Cheque generation
    */
    ChequeGeneration = "CAQ",
    /**
    * Preferential merchandising location
    */
    PreferentialMerchandisingLocation = "CAR",
    /**
    * Crane
    */
    Crane = "CAS",
    /**
    * Special colour service
    */
    SpecialColourService = "CAT",
    /**
    * Sorting
    */
    Sorting = "CAU",
    /**
    * Battery collection and recycling
    */
    BatteryCollectionAndRecycling = "CAV",
    /**
    * Product take back fee
    */
    ProductTakeBackFee = "CAW",
    /**
    * Quality control released
    */
    QualityControlReleased = "CAX",
    /**
    * Quality control held
    */
    QualityControlHeld = "CAY",
    /**
    * Quality control embargo
    */
    QualityControlEmbargo = "CAZ",
    /**
    * Car loading
    */
    CarLoading = "CD",
    /**
    * Cleaning
    */
    Cleaning = "CG",
    /**
    * Cigarette stamping
    */
    CigaretteStamping = "CS",
    /**
    * Count and recount
    */
    CountAndRecount = "CT",
    /**
    * Layout/design
    */
    LayoutDesign = "DAB",
    /**
    * Assortment allowance
    */
    AssortmentAllowance = "DAC",
    /**
    * Driver assigned unloading
    */
    DriverAssignedUnloading = "DAD",
    /**
    * Debtor bound
    */
    DebtorBound = "DAF",
    /**
    * Dealer allowance
    */
    DealerAllowance = "DAG",
    /**
    * Allowance transferable to the consumer
    */
    AllowanceTransferableToConsumer = "DAH",
    /**
    * Growth of business
    */
    GrowthBusiness = "DAI",
    /**
    * Introduction allowance
    */
    IntroductionAllowance = "DAJ",
    /**
    * Multi-buy promotion
    */
    MultiBuyPromotion = "DAK",
    /**
    * Partnership
    */
    Partnership = "DAL",
    /**
    * Return handling
    */
    ReturnHandling = "DAM",
    /**
    * Minimum order not fulfilled charge
    */
    MinimumOrderNotFulfilledCharge = "DAN",
    /**
    * Point of sales threshold allowance
    */
    PointSalesThresholdAllowance = "DAO",
    /**
    * Wholesaling discount
    */
    WholesalingDiscount = "DAP",
    /**
    * Documentary credits transfer commission
    */
    DocumentaryCreditsTransferCommission = "DAQ",
    /**
    * Delivery
    */
    Delivery = "DL",
    /**
    * Engraving
    */
    Engraving = "EG",
    /**
    * Expediting
    */
    Expediting = "EP",
    /**
    * Exchange rate guarantee
    */
    ExchangeRateGuarantee = "ER",
    /**
    * Fabrication
    */
    Fabrication = "FAA",
    /**
    * Freight equalization
    */
    FreightEqualization = "FAB",
    /**
    * Freight extraordinary handling
    */
    FreightExtraordinaryHandling = "FAC",
    /**
    * Freight service
    */
    FreightService = "FC",
    /**
    * Filling/handling
    */
    FillingHandling = "FH",
    /**
    * Financing
    */
    Financing = "FI",
    /**
    * Grinding
    */
    Grinding = "GAA",
    /**
    * Hose
    */
    Hose = "HAA",
    /**
    * Handling
    */
    Handling = "HD",
    /**
    * Hoisting and hauling
    */
    HoistingAndHauling = "HH",
    /**
    * Installation
    */
    Installation = "IAA",
    /**
    * Installation and warranty
    */
    InstallationAndWarranty = "IAB",
    /**
    * Inside delivery
    */
    InsideDelivery = "ID",
    /**
    * Inspection
    */
    Inspection = "IF",
    /**
    * Installation and training
    */
    InstallationAndTraining = "IR",
    /**
    * Invoicing
    */
    Invoicing = "IS",
    /**
    * Koshering
    */
    Koshering = "KO",
    /**
    * Carrier count
    */
    CarrierCount = "L1",
    /**
    * Labelling
    */
    Labelling = "LA",
    /**
    * Labour
    */
    Labour = "LAA",
    /**
    * Repair and return
    */
    RepairAndReturn = "LAB",
    /**
    * Legalisation
    */
    Legalisation = "LF",
    /**
    * Mounting
    */
    Mounting = "MAE",
    /**
    * Mail invoice
    */
    MailInvoice = "MI",
    /**
    * Mail invoice to each location
    */
    MailInvoiceToEachLocation = "ML",
    /**
    * Non-returnable containers
    */
    NonReturnableContainers = "NAA",
    /**
    * Outside cable connectors
    */
    OutsideCableConnectors = "OA",
    /**
    * Invoice with shipment
    */
    InvoiceWithShipment = "PA",
    /**
    * Phosphatizing (steel treatment)
    */
    PhosphatizingSteelTreatment = "PAA",
    /**
    * Packing
    */
    Packing = "PC",
    /**
    * Palletizing
    */
    Palletizing = "PL",
    /**
    * Price variation
    */
    PriceVariation = "PRV",
    /**
    * Repacking
    */
    Repacking = "RAB",
    /**
    * Repair
    */
    Repair = "RAC",
    /**
    * Returnable container
    */
    ReturnableContainer = "RAD",
    /**
    * Restocking
    */
    Restocking = "RAF",
    /**
    * Re-delivery
    */
    ReDelivery = "RE",
    /**
    * Refurbishing
    */
    Refurbishing = "RF",
    /**
    * Rail wagon hire
    */
    RailWagonHire = "RH",
    /**
    * Loading
    */
    Loading = "RV",
    /**
    * Salvaging
    */
    Salvaging = "SA",
    /**
    * Shipping and handling
    */
    ShippingAndHandling = "SAA",
    /**
    * Special packaging
    */
    SpecialPackaging = "SAD",
    /**
    * Stamping
    */
    Stamping = "SAE",
    /**
    * Consignee unload
    */
    ConsigneeUnload = "SAI",
    /**
    * Shrink-wrap
    */
    ShrinkWrap = "SG",
    /**
    * Special handling
    */
    SpecialHandling = "SH",
    /**
    * Special finish
    */
    SpecialFinish = "SM",
    /**
    * Set-up
    */
    SetUp = "SU",
    /**
    * Tank renting
    */
    TankRenting = "TAB",
    /**
    * Testing
    */
    Testing = "TAC",
    /**
    * Transportation - third party billing
    */
    TransportationThirdPartyBilling = "TT",
    /**
    * Transportation by vendor
    */
    TransportationByVendor = "TV",
    /**
    * Drop yard
    */
    DropYard = "V1",
    /**
    * Drop dock
    */
    DropDock = "V2",
    /**
    * Warehousing
    */
    Warehousing = "WH",
    /**
    * Combine all same day shipment
    */
    CombineAllSameDayShipment = "XAA",
    /**
    * Split pick-up
    */
    SplitPickUp = "YY",
    /**
    * Mutually defined
    */
    MutuallyDefined = "ZZZ",
}

export function description(value: Charge): string {
    switch (value) {
        case Charge.Advertising: return "Advertising";
        case Charge.Telecommunication: return "Telecommunication";
        case Charge.TechnicalModification: return "Technical modification";
        case Charge.JobOrderProduction: return "Job-order production";
        case Charge.Outlays: return "Outlays";
        case Charge.OffPremises: return "Off-premises";
        case Charge.AdditionalProcessing: return "Additional processing";
        case Charge.Attesting: return "Attesting";
        case Charge.Acceptance: return "Acceptance";
        case Charge.RushDelivery: return "Rush delivery";
        case Charge.SpecialConstruction: return "Special construction";
        case Charge.AirportFacilities: return "Airport facilities";
        case Charge.Concession: return "Concession";
        case Charge.CompulsoryStorage: return "Compulsory storage";
        case Charge.FuelRemoval: return "Fuel removal";
        case Charge.IntoPlane: return "Into plane";
        case Charge.Overtime: return "Overtime";
        case Charge.Tooling: return "Tooling";
        case Charge.Miscellaneous: return "Miscellaneous";
        case Charge.AdditionalPackaging: return "Additional packaging";
        case Charge.Dunnage: return "Dunnage";
        case Charge.Containerisation: return "Containerisation";
        case Charge.CartonPacking: return "Carton packing";
        case Charge.HessianWrapped: return "Hessian wrapped";
        case Charge.PolyethyleneWrapPacking: return "Polyethylene wrap packing";
        case Charge.MiscellaneousTreatment: return "Miscellaneous treatment";
        case Charge.EnamellingTreatment: return "Enamelling treatment";
        case Charge.HeatTreatment: return "Heat treatment";
        case Charge.PlatingTreatment: return "Plating treatment";
        case Charge.Painting: return "Painting";
        case Charge.Polishing: return "Polishing";
        case Charge.Priming: return "Priming";
        case Charge.PreservationTreatment: return "Preservation treatment";
        case Charge.Fitting: return "Fitting";
        case Charge.Consolidation: return "Consolidation";
        case Charge.BillLading: return "Bill of lading";
        case Charge.Airbag: return "Airbag";
        case Charge.Transfer: return "Transfer";
        case Charge.Slipsheet: return "Slipsheet";
        case Charge.Binding: return "Binding";
        case Charge.RepairOrReplacementBrokenReturnablePackage: return "Repair or replacement of broken returnable package";
        case Charge.EfficientLogistics: return "Efficient logistics";
        case Charge.Merchandising: return "Merchandising";
        case Charge.ProductMix: return "Product mix";
        case Charge.OtherServices: return "Other services";
        case Charge.PickUp: return "Pick-up";
        case Charge.ChronicIllness: return "Chronic illness";
        case Charge.NewProductIntroduction: return "New product introduction";
        case Charge.DirectDelivery: return "Direct delivery";
        case Charge.Diversion: return "Diversion";
        case Charge.Disconnect: return "Disconnect";
        case Charge.Distribution: return "Distribution";
        case Charge.HandlingHazardousCargo: return "Handling of hazardous cargo";
        case Charge.RentsAndLeases: return "Rents and leases";
        case Charge.LocationDifferential: return "Location differential";
        case Charge.AircraftRefueling: return "Aircraft refueling";
        case Charge.FuelShippedIntoStorage: return "Fuel shipped into storage";
        case Charge.CashOnDelivery: return "Cash on delivery";
        case Charge.SmallOrderProcessingService: return "Small order processing service";
        case Charge.ClericalOrAdministrativeServices: return "Clerical or administrative services";
        case Charge.Guarantee: return "Guarantee";
        case Charge.CollectionAndRecycling: return "Collection and recycling";
        case Charge.CopyrightFeeCollection: return "Copyright fee collection";
        case Charge.VeterinaryInspectionService: return "Veterinary inspection service";
        case Charge.PensionerService: return "Pensioner service";
        case Charge.MedicineFreePassHolder: return "Medicine free pass holder";
        case Charge.EnvironmentalProtectionService: return "Environmental protection service";
        case Charge.EnvironmentalCleanUpService: return "Environmental clean-up service";
        case Charge.NationalChequeProcessingServiceOutsideAccountArea: return "National cheque processing service outside account area";
        case Charge.NationalPaymentServiceOutsideAccountArea: return "National payment service outside account area";
        case Charge.NationalPaymentServiceWithinAccountArea: return "National payment service within account area";
        case Charge.Adjustments: return "Adjustments";
        case Charge.Authentication: return "Authentication";
        case Charge.Cataloguing: return "Cataloguing";
        case Charge.Cartage: return "Cartage";
        case Charge.Certification: return "Certification";
        case Charge.CertificateConformance: return "Certificate of conformance";
        case Charge.CertificateOrigin: return "Certificate of origin";
        case Charge.Cutting: return "Cutting";
        case Charge.ConsularService: return "Consular service";
        case Charge.CustomerCollection: return "Customer collection";
        case Charge.PayrollPaymentService: return "Payroll payment service";
        case Charge.CashTransportation: return "Cash transportation";
        case Charge.HomeBankingService: return "Home banking service";
        case Charge.BilateralAgreementService: return "Bilateral agreement service";
        case Charge.InsuranceBrokerageService: return "Insurance brokerage service";
        case Charge.ChequeGeneration: return "Cheque generation";
        case Charge.PreferentialMerchandisingLocation: return "Preferential merchandising location";
        case Charge.Crane: return "Crane";
        case Charge.SpecialColourService: return "Special colour service";
        case Charge.Sorting: return "Sorting";
        case Charge.BatteryCollectionAndRecycling: return "Battery collection and recycling";
        case Charge.ProductTakeBackFee: return "Product take back fee";
        case Charge.QualityControlReleased: return "Quality control released";
        case Charge.QualityControlHeld: return "Quality control held";
        case Charge.QualityControlEmbargo: return "Quality control embargo";
        case Charge.CarLoading: return "Car loading";
        case Charge.Cleaning: return "Cleaning";
        case Charge.CigaretteStamping: return "Cigarette stamping";
        case Charge.CountAndRecount: return "Count and recount";
        case Charge.LayoutDesign: return "Layout/design";
        case Charge.AssortmentAllowance: return "Assortment allowance";
        case Charge.DriverAssignedUnloading: return "Driver assigned unloading";
        case Charge.DebtorBound: return "Debtor bound";
        case Charge.DealerAllowance: return "Dealer allowance";
        case Charge.AllowanceTransferableToConsumer: return "Allowance transferable to the consumer";
        case Charge.GrowthBusiness: return "Growth of business";
        case Charge.IntroductionAllowance: return "Introduction allowance";
        case Charge.MultiBuyPromotion: return "Multi-buy promotion";
        case Charge.Partnership: return "Partnership";
        case Charge.ReturnHandling: return "Return handling";
        case Charge.MinimumOrderNotFulfilledCharge: return "Minimum order not fulfilled charge";
        case Charge.PointSalesThresholdAllowance: return "Point of sales threshold allowance";
        case Charge.WholesalingDiscount: return "Wholesaling discount";
        case Charge.DocumentaryCreditsTransferCommission: return "Documentary credits transfer commission";
        case Charge.Delivery: return "Delivery";
        case Charge.Engraving: return "Engraving";
        case Charge.Expediting: return "Expediting";
        case Charge.ExchangeRateGuarantee: return "Exchange rate guarantee";
        case Charge.Fabrication: return "Fabrication";
        case Charge.FreightEqualization: return "Freight equalization";
        case Charge.FreightExtraordinaryHandling: return "Freight extraordinary handling";
        case Charge.FreightService: return "Freight service";
        case Charge.FillingHandling: return "Filling/handling";
        case Charge.Financing: return "Financing";
        case Charge.Grinding: return "Grinding";
        case Charge.Hose: return "Hose";
        case Charge.Handling: return "Handling";
        case Charge.HoistingAndHauling: return "Hoisting and hauling";
        case Charge.Installation: return "Installation";
        case Charge.InstallationAndWarranty: return "Installation and warranty";
        case Charge.InsideDelivery: return "Inside delivery";
        case Charge.Inspection: return "Inspection";
        case Charge.InstallationAndTraining: return "Installation and training";
        case Charge.Invoicing: return "Invoicing";
        case Charge.Koshering: return "Koshering";
        case Charge.CarrierCount: return "Carrier count";
        case Charge.Labelling: return "Labelling";
        case Charge.Labour: return "Labour";
        case Charge.RepairAndReturn: return "Repair and return";
        case Charge.Legalisation: return "Legalisation";
        case Charge.Mounting: return "Mounting";
        case Charge.MailInvoice: return "Mail invoice";
        case Charge.MailInvoiceToEachLocation: return "Mail invoice to each location";
        case Charge.NonReturnableContainers: return "Non-returnable containers";
        case Charge.OutsideCableConnectors: return "Outside cable connectors";
        case Charge.InvoiceWithShipment: return "Invoice with shipment";
        case Charge.PhosphatizingSteelTreatment: return "Phosphatizing (steel treatment)";
        case Charge.Packing: return "Packing";
        case Charge.Palletizing: return "Palletizing";
        case Charge.PriceVariation: return "Price variation";
        case Charge.Repacking: return "Repacking";
        case Charge.Repair: return "Repair";
        case Charge.ReturnableContainer: return "Returnable container";
        case Charge.Restocking: return "Restocking";
        case Charge.ReDelivery: return "Re-delivery";
        case Charge.Refurbishing: return "Refurbishing";
        case Charge.RailWagonHire: return "Rail wagon hire";
        case Charge.Loading: return "Loading";
        case Charge.Salvaging: return "Salvaging";
        case Charge.ShippingAndHandling: return "Shipping and handling";
        case Charge.SpecialPackaging: return "Special packaging";
        case Charge.Stamping: return "Stamping";
        case Charge.ConsigneeUnload: return "Consignee unload";
        case Charge.ShrinkWrap: return "Shrink-wrap";
        case Charge.SpecialHandling: return "Special handling";
        case Charge.SpecialFinish: return "Special finish";
        case Charge.SetUp: return "Set-up";
        case Charge.TankRenting: return "Tank renting";
        case Charge.Testing: return "Testing";
        case Charge.TransportationThirdPartyBilling: return "Transportation - third party billing";
        case Charge.TransportationByVendor: return "Transportation by vendor";
        case Charge.DropYard: return "Drop yard";
        case Charge.DropDock: return "Drop dock";
        case Charge.Warehousing: return "Warehousing";
        case Charge.CombineAllSameDayShipment: return "Combine all same day shipment";
        case Charge.SplitPickUp: return "Split pick-up";
        case Charge.MutuallyDefined: return "Mutually defined";
    }
}
