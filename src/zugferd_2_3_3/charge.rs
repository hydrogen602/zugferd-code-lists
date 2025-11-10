#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Charge {
    /// Advertising
    Advertising,
    /// Telecommunication
    Telecommunication,
    /// Technical modification
    TechnicalModification,
    /// Job-order production
    JobOrderProduction,
    /// Outlays
    Outlays,
    /// Off-premises
    OffPremises,
    /// Additional processing
    AdditionalProcessing,
    /// Attesting
    Attesting,
    /// Acceptance
    Acceptance,
    /// Rush delivery
    RushDelivery,
    /// Special construction
    SpecialConstruction,
    /// Airport facilities
    AirportFacilities,
    /// Concession
    Concession,
    /// Compulsory storage
    CompulsoryStorage,
    /// Fuel removal
    FuelRemoval,
    /// Into plane
    IntoPlane,
    /// Overtime
    Overtime,
    /// Tooling
    Tooling,
    /// Miscellaneous
    Miscellaneous,
    /// Additional packaging
    AdditionalPackaging,
    /// Dunnage
    Dunnage,
    /// Containerisation
    Containerisation,
    /// Carton packing
    CartonPacking,
    /// Hessian wrapped
    HessianWrapped,
    /// Polyethylene wrap packing
    PolyethyleneWrapPacking,
    /// Miscellaneous treatment
    MiscellaneousTreatment,
    /// Enamelling treatment
    EnamellingTreatment,
    /// Heat treatment
    HeatTreatment,
    /// Plating treatment
    PlatingTreatment,
    /// Painting
    Painting,
    /// Polishing
    Polishing,
    /// Priming
    Priming,
    /// Preservation treatment
    PreservationTreatment,
    /// Fitting
    Fitting,
    /// Consolidation
    Consolidation,
    /// Bill of lading
    BillLading,
    /// Airbag
    Airbag,
    /// Transfer
    Transfer,
    /// Slipsheet
    Slipsheet,
    /// Binding
    Binding,
    /// Repair or replacement of broken returnable package
    RepairOrReplacementBrokenReturnablePackage,
    /// Efficient logistics
    EfficientLogistics,
    /// Merchandising
    Merchandising,
    /// Product mix
    ProductMix,
    /// Other services
    OtherServices,
    /// Pick-up
    PickUp,
    /// Chronic illness
    ChronicIllness,
    /// New product introduction
    NewProductIntroduction,
    /// Direct delivery
    DirectDelivery,
    /// Diversion
    Diversion,
    /// Disconnect
    Disconnect,
    /// Distribution
    Distribution,
    /// Handling of hazardous cargo
    HandlingHazardousCargo,
    /// Rents and leases
    RentsAndLeases,
    /// Location differential
    LocationDifferential,
    /// Aircraft refueling
    AircraftRefueling,
    /// Fuel shipped into storage
    FuelShippedIntoStorage,
    /// Cash on delivery
    CashOnDelivery,
    /// Small order processing service
    SmallOrderProcessingService,
    /// Clerical or administrative services
    ClericalOrAdministrativeServices,
    /// Guarantee
    Guarantee,
    /// Collection and recycling
    CollectionAndRecycling,
    /// Copyright fee collection
    CopyrightFeeCollection,
    /// Veterinary inspection service
    VeterinaryInspectionService,
    /// Pensioner service
    PensionerService,
    /// Medicine free pass holder
    MedicineFreePassHolder,
    /// Environmental protection service
    EnvironmentalProtectionService,
    /// Environmental clean-up service
    EnvironmentalCleanUpService,
    /// National cheque processing service outside account area
    NationalChequeProcessingServiceOutsideAccountArea,
    /// National payment service outside account area
    NationalPaymentServiceOutsideAccountArea,
    /// National payment service within account area
    NationalPaymentServiceWithinAccountArea,
    /// Adjustments
    Adjustments,
    /// Authentication
    Authentication,
    /// Cataloguing
    Cataloguing,
    /// Cartage
    Cartage,
    /// Certification
    Certification,
    /// Certificate of conformance
    CertificateConformance,
    /// Certificate of origin
    CertificateOrigin,
    /// Cutting
    Cutting,
    /// Consular service
    ConsularService,
    /// Customer collection
    CustomerCollection,
    /// Payroll payment service
    PayrollPaymentService,
    /// Cash transportation
    CashTransportation,
    /// Home banking service
    HomeBankingService,
    /// Bilateral agreement service
    BilateralAgreementService,
    /// Insurance brokerage service
    InsuranceBrokerageService,
    /// Cheque generation
    ChequeGeneration,
    /// Preferential merchandising location
    PreferentialMerchandisingLocation,
    /// Crane
    Crane,
    /// Special colour service
    SpecialColourService,
    /// Sorting
    Sorting,
    /// Battery collection and recycling
    BatteryCollectionAndRecycling,
    /// Product take back fee
    ProductTakeBackFee,
    /// Quality control released
    QualityControlReleased,
    /// Quality control held
    QualityControlHeld,
    /// Quality control embargo
    QualityControlEmbargo,
    /// Car loading
    CarLoading,
    /// Cleaning
    Cleaning,
    /// Cigarette stamping
    CigaretteStamping,
    /// Count and recount
    CountAndRecount,
    /// Layout/design
    LayoutDesign,
    /// Assortment allowance
    AssortmentAllowance,
    /// Driver assigned unloading
    DriverAssignedUnloading,
    /// Debtor bound
    DebtorBound,
    /// Dealer allowance
    DealerAllowance,
    /// Allowance transferable to the consumer
    AllowanceTransferableToConsumer,
    /// Growth of business
    GrowthBusiness,
    /// Introduction allowance
    IntroductionAllowance,
    /// Multi-buy promotion
    MultiBuyPromotion,
    /// Partnership
    Partnership,
    /// Return handling
    ReturnHandling,
    /// Minimum order not fulfilled charge
    MinimumOrderNotFulfilledCharge,
    /// Point of sales threshold allowance
    PointSalesThresholdAllowance,
    /// Wholesaling discount
    WholesalingDiscount,
    /// Documentary credits transfer commission
    DocumentaryCreditsTransferCommission,
    /// Delivery
    Delivery,
    /// Engraving
    Engraving,
    /// Expediting
    Expediting,
    /// Exchange rate guarantee
    ExchangeRateGuarantee,
    /// Fabrication
    Fabrication,
    /// Freight equalization
    FreightEqualization,
    /// Freight extraordinary handling
    FreightExtraordinaryHandling,
    /// Freight service
    FreightService,
    /// Filling/handling
    FillingHandling,
    /// Financing
    Financing,
    /// Grinding
    Grinding,
    /// Hose
    Hose,
    /// Handling
    Handling,
    /// Hoisting and hauling
    HoistingAndHauling,
    /// Installation
    Installation,
    /// Installation and warranty
    InstallationAndWarranty,
    /// Inside delivery
    InsideDelivery,
    /// Inspection
    Inspection,
    /// Installation and training
    InstallationAndTraining,
    /// Invoicing
    Invoicing,
    /// Koshering
    Koshering,
    /// Carrier count
    CarrierCount,
    /// Labelling
    Labelling,
    /// Labour
    Labour,
    /// Repair and return
    RepairAndReturn,
    /// Legalisation
    Legalisation,
    /// Mounting
    Mounting,
    /// Mail invoice
    MailInvoice,
    /// Mail invoice to each location
    MailInvoiceToEachLocation,
    /// Non-returnable containers
    NonReturnableContainers,
    /// Outside cable connectors
    OutsideCableConnectors,
    /// Invoice with shipment
    InvoiceWithShipment,
    /// Phosphatizing (steel treatment)
    PhosphatizingSteelTreatment,
    /// Packing
    Packing,
    /// Palletizing
    Palletizing,
    /// Price variation
    PriceVariation,
    /// Repacking
    Repacking,
    /// Repair
    Repair,
    /// Returnable container
    ReturnableContainer,
    /// Restocking
    Restocking,
    /// Re-delivery
    ReDelivery,
    /// Refurbishing
    Refurbishing,
    /// Rail wagon hire
    RailWagonHire,
    /// Loading
    Loading,
    /// Salvaging
    Salvaging,
    /// Shipping and handling
    ShippingAndHandling,
    /// Special packaging
    SpecialPackaging,
    /// Stamping
    Stamping,
    /// Consignee unload
    ConsigneeUnload,
    /// Shrink-wrap
    ShrinkWrap,
    /// Special handling
    SpecialHandling,
    /// Special finish
    SpecialFinish,
    /// Set-up
    SetUp,
    /// Tank renting
    TankRenting,
    /// Testing
    Testing,
    /// Transportation - third party billing
    TransportationThirdPartyBilling,
    /// Transportation by vendor
    TransportationByVendor,
    /// Drop yard
    DropYard,
    /// Drop dock
    DropDock,
    /// Warehousing
    Warehousing,
    /// Combine all same day shipment
    CombineAllSameDayShipment,
    /// Split pick-up
    SplitPickUp,
    /// Mutually defined
    MutuallyDefined,
}

impl crate::Code for Charge {
    fn code(self) -> &'static str {
        match self {
            Charge::Advertising => "AA",
            Charge::Telecommunication => "AAA",
            Charge::TechnicalModification => "AAC",
            Charge::JobOrderProduction => "AAD",
            Charge::Outlays => "AAE",
            Charge::OffPremises => "AAF",
            Charge::AdditionalProcessing => "AAH",
            Charge::Attesting => "AAI",
            Charge::Acceptance => "AAS",
            Charge::RushDelivery => "AAT",
            Charge::SpecialConstruction => "AAV",
            Charge::AirportFacilities => "AAY",
            Charge::Concession => "AAZ",
            Charge::CompulsoryStorage => "ABA",
            Charge::FuelRemoval => "ABB",
            Charge::IntoPlane => "ABC",
            Charge::Overtime => "ABD",
            Charge::Tooling => "ABF",
            Charge::Miscellaneous => "ABK",
            Charge::AdditionalPackaging => "ABL",
            Charge::Dunnage => "ABN",
            Charge::Containerisation => "ABR",
            Charge::CartonPacking => "ABS",
            Charge::HessianWrapped => "ABT",
            Charge::PolyethyleneWrapPacking => "ABU",
            Charge::MiscellaneousTreatment => "ACF",
            Charge::EnamellingTreatment => "ACG",
            Charge::HeatTreatment => "ACH",
            Charge::PlatingTreatment => "ACI",
            Charge::Painting => "ACJ",
            Charge::Polishing => "ACK",
            Charge::Priming => "ACL",
            Charge::PreservationTreatment => "ACM",
            Charge::Fitting => "ACS",
            Charge::Consolidation => "ADC",
            Charge::BillLading => "ADE",
            Charge::Airbag => "ADJ",
            Charge::Transfer => "ADK",
            Charge::Slipsheet => "ADL",
            Charge::Binding => "ADM",
            Charge::RepairOrReplacementBrokenReturnablePackage => "ADN",
            Charge::EfficientLogistics => "ADO",
            Charge::Merchandising => "ADP",
            Charge::ProductMix => "ADQ",
            Charge::OtherServices => "ADR",
            Charge::PickUp => "ADT",
            Charge::ChronicIllness => "ADW",
            Charge::NewProductIntroduction => "ADY",
            Charge::DirectDelivery => "ADZ",
            Charge::Diversion => "AEA",
            Charge::Disconnect => "AEB",
            Charge::Distribution => "AEC",
            Charge::HandlingHazardousCargo => "AED",
            Charge::RentsAndLeases => "AEF",
            Charge::LocationDifferential => "AEH",
            Charge::AircraftRefueling => "AEI",
            Charge::FuelShippedIntoStorage => "AEJ",
            Charge::CashOnDelivery => "AEK",
            Charge::SmallOrderProcessingService => "AEL",
            Charge::ClericalOrAdministrativeServices => "AEM",
            Charge::Guarantee => "AEN",
            Charge::CollectionAndRecycling => "AEO",
            Charge::CopyrightFeeCollection => "AEP",
            Charge::VeterinaryInspectionService => "AES",
            Charge::PensionerService => "AET",
            Charge::MedicineFreePassHolder => "AEU",
            Charge::EnvironmentalProtectionService => "AEV",
            Charge::EnvironmentalCleanUpService => "AEW",
            Charge::NationalChequeProcessingServiceOutsideAccountArea => "AEX",
            Charge::NationalPaymentServiceOutsideAccountArea => "AEY",
            Charge::NationalPaymentServiceWithinAccountArea => "AEZ",
            Charge::Adjustments => "AJ",
            Charge::Authentication => "AU",
            Charge::Cataloguing => "CA",
            Charge::Cartage => "CAB",
            Charge::Certification => "CAD",
            Charge::CertificateConformance => "CAE",
            Charge::CertificateOrigin => "CAF",
            Charge::Cutting => "CAI",
            Charge::ConsularService => "CAJ",
            Charge::CustomerCollection => "CAK",
            Charge::PayrollPaymentService => "CAL",
            Charge::CashTransportation => "CAM",
            Charge::HomeBankingService => "CAN",
            Charge::BilateralAgreementService => "CAO",
            Charge::InsuranceBrokerageService => "CAP",
            Charge::ChequeGeneration => "CAQ",
            Charge::PreferentialMerchandisingLocation => "CAR",
            Charge::Crane => "CAS",
            Charge::SpecialColourService => "CAT",
            Charge::Sorting => "CAU",
            Charge::BatteryCollectionAndRecycling => "CAV",
            Charge::ProductTakeBackFee => "CAW",
            Charge::QualityControlReleased => "CAX",
            Charge::QualityControlHeld => "CAY",
            Charge::QualityControlEmbargo => "CAZ",
            Charge::CarLoading => "CD",
            Charge::Cleaning => "CG",
            Charge::CigaretteStamping => "CS",
            Charge::CountAndRecount => "CT",
            Charge::LayoutDesign => "DAB",
            Charge::AssortmentAllowance => "DAC",
            Charge::DriverAssignedUnloading => "DAD",
            Charge::DebtorBound => "DAF",
            Charge::DealerAllowance => "DAG",
            Charge::AllowanceTransferableToConsumer => "DAH",
            Charge::GrowthBusiness => "DAI",
            Charge::IntroductionAllowance => "DAJ",
            Charge::MultiBuyPromotion => "DAK",
            Charge::Partnership => "DAL",
            Charge::ReturnHandling => "DAM",
            Charge::MinimumOrderNotFulfilledCharge => "DAN",
            Charge::PointSalesThresholdAllowance => "DAO",
            Charge::WholesalingDiscount => "DAP",
            Charge::DocumentaryCreditsTransferCommission => "DAQ",
            Charge::Delivery => "DL",
            Charge::Engraving => "EG",
            Charge::Expediting => "EP",
            Charge::ExchangeRateGuarantee => "ER",
            Charge::Fabrication => "FAA",
            Charge::FreightEqualization => "FAB",
            Charge::FreightExtraordinaryHandling => "FAC",
            Charge::FreightService => "FC",
            Charge::FillingHandling => "FH",
            Charge::Financing => "FI",
            Charge::Grinding => "GAA",
            Charge::Hose => "HAA",
            Charge::Handling => "HD",
            Charge::HoistingAndHauling => "HH",
            Charge::Installation => "IAA",
            Charge::InstallationAndWarranty => "IAB",
            Charge::InsideDelivery => "ID",
            Charge::Inspection => "IF",
            Charge::InstallationAndTraining => "IR",
            Charge::Invoicing => "IS",
            Charge::Koshering => "KO",
            Charge::CarrierCount => "L1",
            Charge::Labelling => "LA",
            Charge::Labour => "LAA",
            Charge::RepairAndReturn => "LAB",
            Charge::Legalisation => "LF",
            Charge::Mounting => "MAE",
            Charge::MailInvoice => "MI",
            Charge::MailInvoiceToEachLocation => "ML",
            Charge::NonReturnableContainers => "NAA",
            Charge::OutsideCableConnectors => "OA",
            Charge::InvoiceWithShipment => "PA",
            Charge::PhosphatizingSteelTreatment => "PAA",
            Charge::Packing => "PC",
            Charge::Palletizing => "PL",
            Charge::PriceVariation => "PRV",
            Charge::Repacking => "RAB",
            Charge::Repair => "RAC",
            Charge::ReturnableContainer => "RAD",
            Charge::Restocking => "RAF",
            Charge::ReDelivery => "RE",
            Charge::Refurbishing => "RF",
            Charge::RailWagonHire => "RH",
            Charge::Loading => "RV",
            Charge::Salvaging => "SA",
            Charge::ShippingAndHandling => "SAA",
            Charge::SpecialPackaging => "SAD",
            Charge::Stamping => "SAE",
            Charge::ConsigneeUnload => "SAI",
            Charge::ShrinkWrap => "SG",
            Charge::SpecialHandling => "SH",
            Charge::SpecialFinish => "SM",
            Charge::SetUp => "SU",
            Charge::TankRenting => "TAB",
            Charge::Testing => "TAC",
            Charge::TransportationThirdPartyBilling => "TT",
            Charge::TransportationByVendor => "TV",
            Charge::DropYard => "V1",
            Charge::DropDock => "V2",
            Charge::Warehousing => "WH",
            Charge::CombineAllSameDayShipment => "XAA",
            Charge::SplitPickUp => "YY",
            Charge::MutuallyDefined => "ZZZ",
        }
    }
}

impl crate::Description for Charge {
    fn description(self) -> &'static str {
        match self {
            Charge::Advertising => "Advertising",
            Charge::Telecommunication => "Telecommunication",
            Charge::TechnicalModification => "Technical modification",
            Charge::JobOrderProduction => "Job-order production",
            Charge::Outlays => "Outlays",
            Charge::OffPremises => "Off-premises",
            Charge::AdditionalProcessing => "Additional processing",
            Charge::Attesting => "Attesting",
            Charge::Acceptance => "Acceptance",
            Charge::RushDelivery => "Rush delivery",
            Charge::SpecialConstruction => "Special construction",
            Charge::AirportFacilities => "Airport facilities",
            Charge::Concession => "Concession",
            Charge::CompulsoryStorage => "Compulsory storage",
            Charge::FuelRemoval => "Fuel removal",
            Charge::IntoPlane => "Into plane",
            Charge::Overtime => "Overtime",
            Charge::Tooling => "Tooling",
            Charge::Miscellaneous => "Miscellaneous",
            Charge::AdditionalPackaging => "Additional packaging",
            Charge::Dunnage => "Dunnage",
            Charge::Containerisation => "Containerisation",
            Charge::CartonPacking => "Carton packing",
            Charge::HessianWrapped => "Hessian wrapped",
            Charge::PolyethyleneWrapPacking => "Polyethylene wrap packing",
            Charge::MiscellaneousTreatment => "Miscellaneous treatment",
            Charge::EnamellingTreatment => "Enamelling treatment",
            Charge::HeatTreatment => "Heat treatment",
            Charge::PlatingTreatment => "Plating treatment",
            Charge::Painting => "Painting",
            Charge::Polishing => "Polishing",
            Charge::Priming => "Priming",
            Charge::PreservationTreatment => "Preservation treatment",
            Charge::Fitting => "Fitting",
            Charge::Consolidation => "Consolidation",
            Charge::BillLading => "Bill of lading",
            Charge::Airbag => "Airbag",
            Charge::Transfer => "Transfer",
            Charge::Slipsheet => "Slipsheet",
            Charge::Binding => "Binding",
            Charge::RepairOrReplacementBrokenReturnablePackage => {
                "Repair or replacement of broken returnable package"
            }
            Charge::EfficientLogistics => "Efficient logistics",
            Charge::Merchandising => "Merchandising",
            Charge::ProductMix => "Product mix",
            Charge::OtherServices => "Other services",
            Charge::PickUp => "Pick-up",
            Charge::ChronicIllness => "Chronic illness",
            Charge::NewProductIntroduction => "New product introduction",
            Charge::DirectDelivery => "Direct delivery",
            Charge::Diversion => "Diversion",
            Charge::Disconnect => "Disconnect",
            Charge::Distribution => "Distribution",
            Charge::HandlingHazardousCargo => "Handling of hazardous cargo",
            Charge::RentsAndLeases => "Rents and leases",
            Charge::LocationDifferential => "Location differential",
            Charge::AircraftRefueling => "Aircraft refueling",
            Charge::FuelShippedIntoStorage => "Fuel shipped into storage",
            Charge::CashOnDelivery => "Cash on delivery",
            Charge::SmallOrderProcessingService => "Small order processing service",
            Charge::ClericalOrAdministrativeServices => "Clerical or administrative services",
            Charge::Guarantee => "Guarantee",
            Charge::CollectionAndRecycling => "Collection and recycling",
            Charge::CopyrightFeeCollection => "Copyright fee collection",
            Charge::VeterinaryInspectionService => "Veterinary inspection service",
            Charge::PensionerService => "Pensioner service",
            Charge::MedicineFreePassHolder => "Medicine free pass holder",
            Charge::EnvironmentalProtectionService => "Environmental protection service",
            Charge::EnvironmentalCleanUpService => "Environmental clean-up service",
            Charge::NationalChequeProcessingServiceOutsideAccountArea => {
                "National cheque processing service outside account area"
            }
            Charge::NationalPaymentServiceOutsideAccountArea => {
                "National payment service outside account area"
            }
            Charge::NationalPaymentServiceWithinAccountArea => {
                "National payment service within account area"
            }
            Charge::Adjustments => "Adjustments",
            Charge::Authentication => "Authentication",
            Charge::Cataloguing => "Cataloguing",
            Charge::Cartage => "Cartage",
            Charge::Certification => "Certification",
            Charge::CertificateConformance => "Certificate of conformance",
            Charge::CertificateOrigin => "Certificate of origin",
            Charge::Cutting => "Cutting",
            Charge::ConsularService => "Consular service",
            Charge::CustomerCollection => "Customer collection",
            Charge::PayrollPaymentService => "Payroll payment service",
            Charge::CashTransportation => "Cash transportation",
            Charge::HomeBankingService => "Home banking service",
            Charge::BilateralAgreementService => "Bilateral agreement service",
            Charge::InsuranceBrokerageService => "Insurance brokerage service",
            Charge::ChequeGeneration => "Cheque generation",
            Charge::PreferentialMerchandisingLocation => "Preferential merchandising location",
            Charge::Crane => "Crane",
            Charge::SpecialColourService => "Special colour service",
            Charge::Sorting => "Sorting",
            Charge::BatteryCollectionAndRecycling => "Battery collection and recycling",
            Charge::ProductTakeBackFee => "Product take back fee",
            Charge::QualityControlReleased => "Quality control released",
            Charge::QualityControlHeld => "Quality control held",
            Charge::QualityControlEmbargo => "Quality control embargo",
            Charge::CarLoading => "Car loading",
            Charge::Cleaning => "Cleaning",
            Charge::CigaretteStamping => "Cigarette stamping",
            Charge::CountAndRecount => "Count and recount",
            Charge::LayoutDesign => "Layout/design",
            Charge::AssortmentAllowance => "Assortment allowance",
            Charge::DriverAssignedUnloading => "Driver assigned unloading",
            Charge::DebtorBound => "Debtor bound",
            Charge::DealerAllowance => "Dealer allowance",
            Charge::AllowanceTransferableToConsumer => "Allowance transferable to the consumer",
            Charge::GrowthBusiness => "Growth of business",
            Charge::IntroductionAllowance => "Introduction allowance",
            Charge::MultiBuyPromotion => "Multi-buy promotion",
            Charge::Partnership => "Partnership",
            Charge::ReturnHandling => "Return handling",
            Charge::MinimumOrderNotFulfilledCharge => "Minimum order not fulfilled charge",
            Charge::PointSalesThresholdAllowance => "Point of sales threshold allowance",
            Charge::WholesalingDiscount => "Wholesaling discount",
            Charge::DocumentaryCreditsTransferCommission => {
                "Documentary credits transfer commission"
            }
            Charge::Delivery => "Delivery",
            Charge::Engraving => "Engraving",
            Charge::Expediting => "Expediting",
            Charge::ExchangeRateGuarantee => "Exchange rate guarantee",
            Charge::Fabrication => "Fabrication",
            Charge::FreightEqualization => "Freight equalization",
            Charge::FreightExtraordinaryHandling => "Freight extraordinary handling",
            Charge::FreightService => "Freight service",
            Charge::FillingHandling => "Filling/handling",
            Charge::Financing => "Financing",
            Charge::Grinding => "Grinding",
            Charge::Hose => "Hose",
            Charge::Handling => "Handling",
            Charge::HoistingAndHauling => "Hoisting and hauling",
            Charge::Installation => "Installation",
            Charge::InstallationAndWarranty => "Installation and warranty",
            Charge::InsideDelivery => "Inside delivery",
            Charge::Inspection => "Inspection",
            Charge::InstallationAndTraining => "Installation and training",
            Charge::Invoicing => "Invoicing",
            Charge::Koshering => "Koshering",
            Charge::CarrierCount => "Carrier count",
            Charge::Labelling => "Labelling",
            Charge::Labour => "Labour",
            Charge::RepairAndReturn => "Repair and return",
            Charge::Legalisation => "Legalisation",
            Charge::Mounting => "Mounting",
            Charge::MailInvoice => "Mail invoice",
            Charge::MailInvoiceToEachLocation => "Mail invoice to each location",
            Charge::NonReturnableContainers => "Non-returnable containers",
            Charge::OutsideCableConnectors => "Outside cable connectors",
            Charge::InvoiceWithShipment => "Invoice with shipment",
            Charge::PhosphatizingSteelTreatment => "Phosphatizing (steel treatment)",
            Charge::Packing => "Packing",
            Charge::Palletizing => "Palletizing",
            Charge::PriceVariation => "Price variation",
            Charge::Repacking => "Repacking",
            Charge::Repair => "Repair",
            Charge::ReturnableContainer => "Returnable container",
            Charge::Restocking => "Restocking",
            Charge::ReDelivery => "Re-delivery",
            Charge::Refurbishing => "Refurbishing",
            Charge::RailWagonHire => "Rail wagon hire",
            Charge::Loading => "Loading",
            Charge::Salvaging => "Salvaging",
            Charge::ShippingAndHandling => "Shipping and handling",
            Charge::SpecialPackaging => "Special packaging",
            Charge::Stamping => "Stamping",
            Charge::ConsigneeUnload => "Consignee unload",
            Charge::ShrinkWrap => "Shrink-wrap",
            Charge::SpecialHandling => "Special handling",
            Charge::SpecialFinish => "Special finish",
            Charge::SetUp => "Set-up",
            Charge::TankRenting => "Tank renting",
            Charge::Testing => "Testing",
            Charge::TransportationThirdPartyBilling => "Transportation - third party billing",
            Charge::TransportationByVendor => "Transportation by vendor",
            Charge::DropYard => "Drop yard",
            Charge::DropDock => "Drop dock",
            Charge::Warehousing => "Warehousing",
            Charge::CombineAllSameDayShipment => "Combine all same day shipment",
            Charge::SplitPickUp => "Split pick-up",
            Charge::MutuallyDefined => "Mutually defined",
        }
    }
}

impl crate::FromCode for Charge {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "AA" => Some(Charge::Advertising),
            "AAA" => Some(Charge::Telecommunication),
            "AAC" => Some(Charge::TechnicalModification),
            "AAD" => Some(Charge::JobOrderProduction),
            "AAE" => Some(Charge::Outlays),
            "AAF" => Some(Charge::OffPremises),
            "AAH" => Some(Charge::AdditionalProcessing),
            "AAI" => Some(Charge::Attesting),
            "AAS" => Some(Charge::Acceptance),
            "AAT" => Some(Charge::RushDelivery),
            "AAV" => Some(Charge::SpecialConstruction),
            "AAY" => Some(Charge::AirportFacilities),
            "AAZ" => Some(Charge::Concession),
            "ABA" => Some(Charge::CompulsoryStorage),
            "ABB" => Some(Charge::FuelRemoval),
            "ABC" => Some(Charge::IntoPlane),
            "ABD" => Some(Charge::Overtime),
            "ABF" => Some(Charge::Tooling),
            "ABK" => Some(Charge::Miscellaneous),
            "ABL" => Some(Charge::AdditionalPackaging),
            "ABN" => Some(Charge::Dunnage),
            "ABR" => Some(Charge::Containerisation),
            "ABS" => Some(Charge::CartonPacking),
            "ABT" => Some(Charge::HessianWrapped),
            "ABU" => Some(Charge::PolyethyleneWrapPacking),
            "ACF" => Some(Charge::MiscellaneousTreatment),
            "ACG" => Some(Charge::EnamellingTreatment),
            "ACH" => Some(Charge::HeatTreatment),
            "ACI" => Some(Charge::PlatingTreatment),
            "ACJ" => Some(Charge::Painting),
            "ACK" => Some(Charge::Polishing),
            "ACL" => Some(Charge::Priming),
            "ACM" => Some(Charge::PreservationTreatment),
            "ACS" => Some(Charge::Fitting),
            "ADC" => Some(Charge::Consolidation),
            "ADE" => Some(Charge::BillLading),
            "ADJ" => Some(Charge::Airbag),
            "ADK" => Some(Charge::Transfer),
            "ADL" => Some(Charge::Slipsheet),
            "ADM" => Some(Charge::Binding),
            "ADN" => Some(Charge::RepairOrReplacementBrokenReturnablePackage),
            "ADO" => Some(Charge::EfficientLogistics),
            "ADP" => Some(Charge::Merchandising),
            "ADQ" => Some(Charge::ProductMix),
            "ADR" => Some(Charge::OtherServices),
            "ADT" => Some(Charge::PickUp),
            "ADW" => Some(Charge::ChronicIllness),
            "ADY" => Some(Charge::NewProductIntroduction),
            "ADZ" => Some(Charge::DirectDelivery),
            "AEA" => Some(Charge::Diversion),
            "AEB" => Some(Charge::Disconnect),
            "AEC" => Some(Charge::Distribution),
            "AED" => Some(Charge::HandlingHazardousCargo),
            "AEF" => Some(Charge::RentsAndLeases),
            "AEH" => Some(Charge::LocationDifferential),
            "AEI" => Some(Charge::AircraftRefueling),
            "AEJ" => Some(Charge::FuelShippedIntoStorage),
            "AEK" => Some(Charge::CashOnDelivery),
            "AEL" => Some(Charge::SmallOrderProcessingService),
            "AEM" => Some(Charge::ClericalOrAdministrativeServices),
            "AEN" => Some(Charge::Guarantee),
            "AEO" => Some(Charge::CollectionAndRecycling),
            "AEP" => Some(Charge::CopyrightFeeCollection),
            "AES" => Some(Charge::VeterinaryInspectionService),
            "AET" => Some(Charge::PensionerService),
            "AEU" => Some(Charge::MedicineFreePassHolder),
            "AEV" => Some(Charge::EnvironmentalProtectionService),
            "AEW" => Some(Charge::EnvironmentalCleanUpService),
            "AEX" => Some(Charge::NationalChequeProcessingServiceOutsideAccountArea),
            "AEY" => Some(Charge::NationalPaymentServiceOutsideAccountArea),
            "AEZ" => Some(Charge::NationalPaymentServiceWithinAccountArea),
            "AJ" => Some(Charge::Adjustments),
            "AU" => Some(Charge::Authentication),
            "CA" => Some(Charge::Cataloguing),
            "CAB" => Some(Charge::Cartage),
            "CAD" => Some(Charge::Certification),
            "CAE" => Some(Charge::CertificateConformance),
            "CAF" => Some(Charge::CertificateOrigin),
            "CAI" => Some(Charge::Cutting),
            "CAJ" => Some(Charge::ConsularService),
            "CAK" => Some(Charge::CustomerCollection),
            "CAL" => Some(Charge::PayrollPaymentService),
            "CAM" => Some(Charge::CashTransportation),
            "CAN" => Some(Charge::HomeBankingService),
            "CAO" => Some(Charge::BilateralAgreementService),
            "CAP" => Some(Charge::InsuranceBrokerageService),
            "CAQ" => Some(Charge::ChequeGeneration),
            "CAR" => Some(Charge::PreferentialMerchandisingLocation),
            "CAS" => Some(Charge::Crane),
            "CAT" => Some(Charge::SpecialColourService),
            "CAU" => Some(Charge::Sorting),
            "CAV" => Some(Charge::BatteryCollectionAndRecycling),
            "CAW" => Some(Charge::ProductTakeBackFee),
            "CAX" => Some(Charge::QualityControlReleased),
            "CAY" => Some(Charge::QualityControlHeld),
            "CAZ" => Some(Charge::QualityControlEmbargo),
            "CD" => Some(Charge::CarLoading),
            "CG" => Some(Charge::Cleaning),
            "CS" => Some(Charge::CigaretteStamping),
            "CT" => Some(Charge::CountAndRecount),
            "DAB" => Some(Charge::LayoutDesign),
            "DAC" => Some(Charge::AssortmentAllowance),
            "DAD" => Some(Charge::DriverAssignedUnloading),
            "DAF" => Some(Charge::DebtorBound),
            "DAG" => Some(Charge::DealerAllowance),
            "DAH" => Some(Charge::AllowanceTransferableToConsumer),
            "DAI" => Some(Charge::GrowthBusiness),
            "DAJ" => Some(Charge::IntroductionAllowance),
            "DAK" => Some(Charge::MultiBuyPromotion),
            "DAL" => Some(Charge::Partnership),
            "DAM" => Some(Charge::ReturnHandling),
            "DAN" => Some(Charge::MinimumOrderNotFulfilledCharge),
            "DAO" => Some(Charge::PointSalesThresholdAllowance),
            "DAP" => Some(Charge::WholesalingDiscount),
            "DAQ" => Some(Charge::DocumentaryCreditsTransferCommission),
            "DL" => Some(Charge::Delivery),
            "EG" => Some(Charge::Engraving),
            "EP" => Some(Charge::Expediting),
            "ER" => Some(Charge::ExchangeRateGuarantee),
            "FAA" => Some(Charge::Fabrication),
            "FAB" => Some(Charge::FreightEqualization),
            "FAC" => Some(Charge::FreightExtraordinaryHandling),
            "FC" => Some(Charge::FreightService),
            "FH" => Some(Charge::FillingHandling),
            "FI" => Some(Charge::Financing),
            "GAA" => Some(Charge::Grinding),
            "HAA" => Some(Charge::Hose),
            "HD" => Some(Charge::Handling),
            "HH" => Some(Charge::HoistingAndHauling),
            "IAA" => Some(Charge::Installation),
            "IAB" => Some(Charge::InstallationAndWarranty),
            "ID" => Some(Charge::InsideDelivery),
            "IF" => Some(Charge::Inspection),
            "IR" => Some(Charge::InstallationAndTraining),
            "IS" => Some(Charge::Invoicing),
            "KO" => Some(Charge::Koshering),
            "L1" => Some(Charge::CarrierCount),
            "LA" => Some(Charge::Labelling),
            "LAA" => Some(Charge::Labour),
            "LAB" => Some(Charge::RepairAndReturn),
            "LF" => Some(Charge::Legalisation),
            "MAE" => Some(Charge::Mounting),
            "MI" => Some(Charge::MailInvoice),
            "ML" => Some(Charge::MailInvoiceToEachLocation),
            "NAA" => Some(Charge::NonReturnableContainers),
            "OA" => Some(Charge::OutsideCableConnectors),
            "PA" => Some(Charge::InvoiceWithShipment),
            "PAA" => Some(Charge::PhosphatizingSteelTreatment),
            "PC" => Some(Charge::Packing),
            "PL" => Some(Charge::Palletizing),
            "PRV" => Some(Charge::PriceVariation),
            "RAB" => Some(Charge::Repacking),
            "RAC" => Some(Charge::Repair),
            "RAD" => Some(Charge::ReturnableContainer),
            "RAF" => Some(Charge::Restocking),
            "RE" => Some(Charge::ReDelivery),
            "RF" => Some(Charge::Refurbishing),
            "RH" => Some(Charge::RailWagonHire),
            "RV" => Some(Charge::Loading),
            "SA" => Some(Charge::Salvaging),
            "SAA" => Some(Charge::ShippingAndHandling),
            "SAD" => Some(Charge::SpecialPackaging),
            "SAE" => Some(Charge::Stamping),
            "SAI" => Some(Charge::ConsigneeUnload),
            "SG" => Some(Charge::ShrinkWrap),
            "SH" => Some(Charge::SpecialHandling),
            "SM" => Some(Charge::SpecialFinish),
            "SU" => Some(Charge::SetUp),
            "TAB" => Some(Charge::TankRenting),
            "TAC" => Some(Charge::Testing),
            "TT" => Some(Charge::TransportationThirdPartyBilling),
            "TV" => Some(Charge::TransportationByVendor),
            "V1" => Some(Charge::DropYard),
            "V2" => Some(Charge::DropDock),
            "WH" => Some(Charge::Warehousing),
            "XAA" => Some(Charge::CombineAllSameDayShipment),
            "YY" => Some(Charge::SplitPickUp),
            "ZZZ" => Some(Charge::MutuallyDefined),
            _ => None,
        }
    }
}
