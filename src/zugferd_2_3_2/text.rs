#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Text {
    /// Goods item description
    GoodsItemDescription,
    /// Payment term
    PaymentTerm,
    /// Dangerous goods additional information
    DangerousGoodsAdditionalInformation,
    /// Dangerous goods technical name
    DangerousGoodsTechnicalName,
    /// Acknowledgement description
    AcknowledgementDescription,
    /// Rate additional information
    RateAdditionalInformation,
    /// Party instructions
    PartyInstructions,
    /// General information
    GeneralInformation,
    /// Additional conditions of sale/purchase
    AdditionalConditionsSalePurchase,
    /// Price conditions
    PriceConditions,
    /// Goods dimensions in characters
    GoodsDimensionsInCharacters,
    /// Equipment re-usage restrictions
    EquipmentReUsageRestrictions,
    /// Handling restriction
    HandlingRestriction,
    /// Error description (free text)
    ErrorDescriptionFreeText,
    /// Response (free text)
    ResponseFreeText,
    /// Package content's description
    PackageContentsDescription,
    /// Terms of delivery
    TermsDelivery,
    /// Bill of lading remarks
    BillLadingRemarks,
    /// Mode of settlement information
    ModeSettlementInformation,
    /// Consignment invoice information
    ConsignmentInvoiceInformation,
    /// Clearance invoice information
    ClearanceInvoiceInformation,
    /// Letter of credit information
    LetterCreditInformation,
    /// License information
    LicenseInformation,
    /// Certification statements
    CertificationStatements,
    /// Additional export information
    AdditionalExportInformation,
    /// Tariff statements
    TariffStatements,
    /// Medical history
    MedicalHistory,
    /// Conditions of sale or purchase
    ConditionsSaleOrPurchase,
    /// Contract document type
    ContractDocumentType,
    /// Additional terms and/or conditions (documentary credit)
    AdditionalTermsAndOrConditionsDocumentaryCredit,
    /// Instructions or information about standby documentary
    InstructionsOrInformationAboutStandbyDocumentary,
    /// Instructions or information about partial shipment(s)
    InstructionsOrInformationAboutPartialShipmentS,
    /// Instructions or information about transhipment(s)
    InstructionsOrInformationAboutTranshipmentS,
    /// Additional handling instructions documentary credit
    AdditionalHandlingInstructionsDocumentaryCredit,
    /// Domestic routing information
    DomesticRoutingInformation,
    /// Chargeable category of equipment
    ChargeableCategoryEquipment,
    /// Government information
    GovernmentInformation,
    /// Onward routing information
    OnwardRoutingInformation,
    /// Accounting information
    AccountingInformation,
    /// Discrepancy information
    DiscrepancyInformation,
    /// Confirmation instructions
    ConfirmationInstructions,
    /// Method of issuance
    MethodIssuance,
    /// Documents delivery instructions
    DocumentsDeliveryInstructions,
    /// Additional conditions
    AdditionalConditions,
    /// Information/instructions about additional amounts covered
    InformationInstructionsAboutAdditionalAmountsCovered,
    /// Deferred payment termed additional
    DeferredPaymentTermedAdditional,
    /// Acceptance terms additional
    AcceptanceTermsAdditional,
    /// Negotiation terms additional
    NegotiationTermsAdditional,
    /// Document name and documentary requirements
    DocumentNameAndDocumentaryRequirements,
    /// Instructions/information about revolving documentary credit
    InstructionsInformationAboutRevolvingDocumentaryCredit,
    /// Documentary requirements
    DocumentaryRequirements,
    /// Additional information
    AdditionalInformation,
    /// Factor assignment clause
    FactorAssignmentClause,
    /// Reason
    Reason,
    /// Dispute
    Dispute,
    /// Additional attribute information
    AdditionalAttributeInformation,
    /// Absence declaration
    AbsenceDeclaration,
    /// Aggregation statement
    AggregationStatement,
    /// Compilation statement
    CompilationStatement,
    /// Definitional exception
    DefinitionalException,
    /// Privacy statement
    PrivacyStatement,
    /// Quality statement
    QualityStatement,
    /// Statistical description
    StatisticalDescription,
    /// Statistical definition
    StatisticalDefinition,
    /// Statistical name
    StatisticalName,
    /// Statistical title
    StatisticalTitle,
    /// Off-dimension information
    OffDimensionInformation,
    /// Unexpected stops information
    UnexpectedStopsInformation,
    /// Principles
    Principles,
    /// Terms and definition
    TermsAndDefinition,
    /// Segment name
    SegmentName,
    /// Simple data element name
    SimpleDataElementName,
    /// Scope
    Scope,
    /// Message type name
    MessageTypeName,
    /// Introduction
    Introduction,
    /// Glossary
    Glossary,
    /// Functional definition
    FunctionalDefinition,
    /// Examples
    Examples,
    /// Cover page
    CoverPage,
    /// Dependency (syntax) notes
    DependencySyntaxNotes,
    /// Code value name
    CodeValueName,
    /// Code list name
    CodeListName,
    /// Clarification of usage
    ClarificationUsage,
    /// Composite data element name
    CompositeDataElementName,
    /// Field of application
    FieldApplication,
    /// Type of assets and liabilities
    TypeAssetsAndLiabilities,
    /// Promotion information
    PromotionInformation,
    /// Meter condition
    MeterCondition,
    /// Meter reading information
    MeterReadingInformation,
    /// Type of transaction reason
    TypeTransactionReason,
    /// Type of survey question
    TypeSurveyQuestion,
    /// Carrier's agent counter information
    CarriersAgentCounterInformation,
    /// Description of work item on equipment
    DescriptionWorkItemOnEquipment,
    /// Message definition
    MessageDefinition,
    /// Booked item information
    BookedItemInformation,
    /// Source of document
    SourceDocument,
    /// Note
    Note,
    /// Fixed part of segment clarification text
    FixedPartSegmentClarificationText,
    /// Characteristics of goods
    CharacteristicsGoods,
    /// Additional discharge instructions
    AdditionalDischargeInstructions,
    /// Container stripping instructions
    ContainerStrippingInstructions,
    /// CSC (Container Safety Convention) plate information
    CscContainerSafetyConventionPlateInformation,
    /// Cargo remarks
    CargoRemarks,
    /// Temperature control instructions
    TemperatureControlInstructions,
    /// Text refers to expected data
    TextRefersToExpectedData,
    /// Text refers to received data
    TextRefersToReceivedData,
    /// Section clarification text
    SectionClarificationText,
    /// Information to the beneficiary
    InformationToBeneficiary,
    /// Information to the applicant
    InformationToApplicant,
    /// Instructions to the beneficiary
    InstructionsToBeneficiary,
    /// Instructions to the applicant
    InstructionsToApplicant,
    /// Controlled atmosphere
    ControlledAtmosphere,
    /// Take off annotation
    TakeOffAnnotation,
    /// Price variation narrative
    PriceVariationNarrative,
    /// Documentary credit amendment instructions
    DocumentaryCreditAmendmentInstructions,
    /// Standard method narrative
    StandardMethodNarrative,
    /// Project narrative
    ProjectNarrative,
    /// Radioactive goods, additional information
    RadioactiveGoodsAdditionalInformation,
    /// Bank-to-bank information
    BankToBankInformation,
    /// Reimbursement instructions
    ReimbursementInstructions,
    /// Reason for amending a message
    ReasonForAmendingAMessage,
    /// Instructions to the paying and/or accepting and/or
    InstructionsToPayingAndOrAcceptingAndOr,
    /// Interest instructions
    InterestInstructions,
    /// Agent commission
    AgentCommission,
    /// Remitting bank instructions
    RemittingBankInstructions,
    /// Instructions to the collecting bank
    InstructionsToCollectingBank,
    /// Collection amount instructions
    CollectionAmountInstructions,
    /// Internal auditing information
    InternalAuditingInformation,
    /// Constraint
    Constraint,
    /// Comment
    Comment,
    /// Semantic note
    SemanticNote,
    /// Help text
    HelpText,
    /// Legend
    Legend,
    /// Batch code structure
    BatchCodeStructure,
    /// Product application
    ProductApplication,
    /// Customer complaint
    CustomerComplaint,
    /// Probable cause of fault
    ProbableCauseFault,
    /// Defect description
    DefectDescription,
    /// Repair description
    RepairDescription,
    /// Review comments
    ReviewComments,
    /// Title
    Title,
    /// Description of amount
    DescriptionAmount,
    /// Responsibilities
    Responsibilities,
    /// Supplier
    Supplier,
    /// Purchase region
    PurchaseRegion,
    /// Affiliation
    Affiliation,
    /// Borrower
    Borrower,
    /// Line of business
    LineBusiness,
    /// Financial institution
    FinancialInstitution,
    /// Business founder
    BusinessFounder,
    /// Business history
    BusinessHistory,
    /// Banking arrangements
    BankingArrangements,
    /// Business origin
    BusinessOrigin,
    /// Brand names' description
    BrandNamesDescription,
    /// Business financing details
    BusinessFinancingDetails,
    /// Competition
    Competition,
    /// Construction process details
    ConstructionProcessDetails,
    /// Construction specialty
    ConstructionSpecialty,
    /// Contract information
    ContractInformation,
    /// Corporate filing
    CorporateFiling,
    /// Customer information
    CustomerInformation,
    /// Copyright notice
    CopyrightNotice,
    /// Contingent debt
    ContingentDebt,
    /// Conviction details
    ConvictionDetails,
    /// Equipment
    Equipment,
    /// Workforce description
    WorkforceDescription,
    /// Exemption
    Exemption,
    /// Future plans
    FuturePlans,
    /// Interviewee conversation information
    IntervieweeConversationInformation,
    /// Intangible asset
    IntangibleAsset,
    /// Inventory
    Inventory,
    /// Investment
    Investment,
    /// Intercompany relations information
    IntercompanyRelationsInformation,
    /// Joint venture
    JointVenture,
    /// Loan
    Loan,
    /// Long term debt
    LongTermDebt,
    /// Location
    Location,
    /// Current legal structure
    CurrentLegalStructure,
    /// Marital contract
    MaritalContract,
    /// Marketing activities
    MarketingActivities,
    /// Merger
    Merger,
    /// Marketable securities
    MarketableSecurities,
    /// Business debt
    BusinessDebt,
    /// Original legal structure
    OriginalLegalStructure,
    /// Employee sharing arrangements
    EmployeeSharingArrangements,
    /// Organization details
    OrganizationDetails,
    /// Public record details
    PublicRecordDetails,
    /// Price range
    PriceRange,
    /// Qualifications
    Qualifications,
    /// Registered activity
    RegisteredActivity,
    /// Criminal sentence
    CriminalSentence,
    /// Sales method
    SalesMethod,
    /// Educational institution information
    EducationalInstitutionInformation,
    /// Status details
    StatusDetails,
    /// Sales
    Sales,
    /// Spouse information
    SpouseInformation,
    /// Educational degree information
    EducationalDegreeInformation,
    /// Shareholding information
    ShareholdingInformation,
    /// Sales territory
    SalesTerritory,
    /// Accountant's comments
    AccountantsComments,
    /// Exemption law location
    ExemptionLawLocation,
    /// Share classifications
    ShareClassifications,
    /// Forecast
    Forecast,
    /// Event location
    EventLocation,
    /// Facility occupancy
    FacilityOccupancy,
    /// Import and export details
    ImportAndExportDetails,
    /// Additional facility information
    AdditionalFacilityInformation,
    /// Inventory value
    InventoryValue,
    /// Education
    Education,
    /// Event
    Event,
    /// Agent
    Agent,
    /// Domestically agreed financial statement details
    DomesticallyAgreedFinancialStatementDetails,
    /// Other current asset description
    OtherCurrentAssetDescription,
    /// Other current liability description
    OtherCurrentLiabilityDescription,
    /// Former business activity
    FormerBusinessActivity,
    /// Trade name use
    TradeNameUse,
    /// Signing authority
    SigningAuthority,
    /// Guarantee
    Guarantee,
    /// Holding company operation
    HoldingCompanyOperation,
    /// Consignment routing
    ConsignmentRouting,
    /// Letter of protest
    LetterProtest,
    /// Question
    Question,
    /// Party information
    PartyInformation,
    /// Area boundaries description
    AreaBoundariesDescription,
    /// Advertisement information
    AdvertisementInformation,
    /// Financial statement details
    FinancialStatementDetails,
    /// Access instructions
    AccessInstructions,
    /// Liquidity
    Liquidity,
    /// Credit line
    CreditLine,
    /// Warranty terms
    WarrantyTerms,
    /// Division description
    DivisionDescription,
    /// Reporting instruction
    ReportingInstruction,
    /// Examination result
    ExaminationResult,
    /// Laboratory result
    LaboratoryResult,
    /// Allowance/charge information
    AllowanceChargeInformation,
    /// X-ray result
    XRayResult,
    /// Pathology result
    PathologyResult,
    /// Intervention description
    InterventionDescription,
    /// Summary of admittance
    SummaryAdmittance,
    /// Medical treatment course detail
    MedicalTreatmentCourseDetail,
    /// Prognosis
    Prognosis,
    /// Instruction to patient
    InstructionToPatient,
    /// Instruction to physician
    InstructionToPhysician,
    /// All documents
    AllDocuments,
    /// Medicine treatment
    MedicineTreatment,
    /// Medicine dosage and administration
    MedicineDosageAndAdministration,
    /// Availability of patient
    AvailabilityPatient,
    /// Reason for service request
    ReasonForServiceRequest,
    /// Purpose of service
    PurposeService,
    /// Arrival conditions
    ArrivalConditions,
    /// Service requester's comment
    ServiceRequestersComment,
    /// Authentication
    Authentication,
    /// Requested location description
    RequestedLocationDescription,
    /// Medicine administration condition
    MedicineAdministrationCondition,
    /// Patient information
    PatientInformation,
    /// Precautionary measure
    PrecautionaryMeasure,
    /// Service characteristic
    ServiceCharacteristic,
    /// Planned event comment
    PlannedEventComment,
    /// Expected delay comment
    ExpectedDelayComment,
    /// Transport requirements comment
    TransportRequirementsComment,
    /// Temporary approval condition
    TemporaryApprovalCondition,
    /// Customs Valuation Information
    CustomsValuationInformation,
    /// Value Added Tax (VAT) margin scheme
    ValueAddedTaxVatMarginScheme,
    /// Maritime Declaration of Health
    MaritimeDeclarationHealth,
    /// Passenger baggage information
    PassengerBaggageInformation,
    /// Maritime Declaration of Health
    MaritimeDeclarationHealth_Dup,
    /// Additional product information address
    AdditionalProductInformationAddress,
    /// Information to be printed on despatch advice
    InformationToBePrintedOnDespatchAdvice,
    /// Missing goods remarks
    MissingGoodsRemarks,
    /// Non-acceptance information
    NonAcceptanceInformation,
    /// Returns information
    ReturnsInformation,
    /// Sub-line item information
    SubLineItemInformation,
    /// Test information
    TestInformation,
    /// External link
    ExternalLink,
    /// VAT exemption reason
    VatExemptionReason,
    /// Processing Instructions
    ProcessingInstructions,
    /// Relay Instructions
    RelayInstructions,
    /// SIMA applicable
    SimaApplicable,
    /// Appeals program code
    AppealsProgramCode,
    /// SIMA subject
    SimaSubject,
    /// Surtax applicable
    SurtaxApplicable,
    /// SIMA security bond
    SimaSecurityBond,
    /// Surtax subject
    SurtaxSubject,
    /// Safeguard applicable
    SafeguardApplicable,
    /// Safeguard applicable
    SafeguardApplicable_Dup,
    /// Safeguard subject
    SafeguardSubject,
    /// Transport contract document clause
    TransportContractDocumentClause,
    /// Instruction to prepare the patient
    InstructionToPreparePatient,
    /// Medicine treatment comment
    MedicineTreatmentComment,
    /// Examination result comment
    ExaminationResultComment,
    /// Service request comment
    ServiceRequestComment,
    /// Prescription reason
    PrescriptionReason,
    /// Prescription comment
    PrescriptionComment,
    /// Clinical investigation comment
    ClinicalInvestigationComment,
    /// Medicinal specification comment
    MedicinalSpecificationComment,
    /// Economic contribution comment
    EconomicContributionComment,
    /// Status of a plan
    StatusAPlan,
    /// Random sample test information
    RandomSampleTestInformation,
    /// Period of time
    PeriodTime,
    /// Legislation
    Legislation,
    /// Security measures requested
    SecurityMeasuresRequested,
    /// Transport contract document remark
    TransportContractDocumentRemark,
    /// Previous port of call security information
    PreviousPortCallSecurityInformation,
    /// Security information
    SecurityInformation,
    /// Waste information
    WasteInformation,
    /// B2C marketing information, short description
    B2cMarketingInformationShortDescription,
    /// B2B marketing information, long description
    B2bMarketingInformationLongDescription,
    /// B2C marketing information, long description
    B2cMarketingInformationLongDescription,
    /// Product ingredients
    ProductIngredients,
    /// Location short name
    LocationShortName,
    /// Packaging material information
    PackagingMaterialInformation,
    /// Filler material information
    FillerMaterialInformation,
    /// Ship-to-ship activity information
    ShipToShipActivityInformation,
    /// Package material description
    PackageMaterialDescription,
    /// Consumer level package marking
    ConsumerLevelPackageMarking,
    /// SIMA measure in force
    SimaMeasureInForce,
    /// Pre-CARM
    PreCarm,
    /// SIMA measure type
    SimaMeasureType,
    /// Customs clearance instructions
    CustomsClearanceInstructions,
    /// Sub Type Code
    SubTypeCode,
    /// SIMA information
    SimaInformation,
    /// Time limit end
    TimeLimitEnd,
    /// Time limit start
    TimeLimitStart,
    /// Warehouse time limit
    WarehouseTimeLimit,
    /// Value for duty information
    ValueForDutyInformation,
    /// Customs clearance instructions export
    CustomsClearanceInstructionsExport,
    /// Change information
    ChangeInformation,
    /// Customs clearance instruction import
    CustomsClearanceInstructionImport,
    /// Clearance place requested
    ClearancePlaceRequested,
    /// Loading remarks
    LoadingRemarks,
    /// Order information
    OrderInformation,
    /// Customer remarks
    CustomerRemarks,
    /// Customs declaration information
    CustomsDeclarationInformation,
    /// Damage remarks
    DamageRemarks,
    /// Document issuer declaration
    DocumentIssuerDeclaration,
    /// Delivery information
    DeliveryInformation,
    /// Delivery instructions
    DeliveryInstructions,
    /// Documentation instructions
    DocumentationInstructions,
    /// Duty declaration
    DutyDeclaration,
    /// Effective used routing
    EffectiveUsedRouting,
    /// First block to be printed on the transport contract
    FirstBlockToBePrintedOnTransportContract,
    /// Government bill of lading information
    GovernmentBillLadingInformation,
    /// Entire transaction set
    EntireTransactionSet,
    /// Further information concerning GGVS par. 7
    FurtherInformationConcerningGgvsPar7,
    /// Consignment handling instruction
    ConsignmentHandlingInstruction,
    /// Hazard information
    HazardInformation,
    /// Consignment information for consignee
    ConsignmentInformationForConsignee,
    /// Insurance instructions
    InsuranceInstructions,
    /// Invoice mailing instructions
    InvoiceMailingInstructions,
    /// Commercial invoice item description
    CommercialInvoiceItemDescription,
    /// Insurance information
    InsuranceInformation,
    /// Invoice instruction
    InvoiceInstruction,
    /// Information for railway purpose
    InformationForRailwayPurpose,
    /// Inland transport details
    InlandTransportDetails,
    /// Testing instructions
    TestingInstructions,
    /// Location Alias
    LocationAlias,
    /// Line item
    LineItem,
    /// Loading instruction
    LoadingInstruction,
    /// Miscellaneous charge order
    MiscellaneousChargeOrder,
    /// Maritime Declaration of Health
    MaritimeDeclarationHealth_Dup_Dup,
    /// Additional marks/numbers information
    AdditionalMarksNumbersInformation,
    /// Order instruction
    OrderInstruction,
    /// Other service information
    OtherServiceInformation,
    /// Packing/marking information
    PackingMarkingInformation,
    /// Payment instructions information
    PaymentInstructionsInformation,
    /// Payables information
    PayablesInformation,
    /// Packaging information
    PackagingInformation,
    /// Packaging terms information
    PackagingTermsInformation,
    /// Payment detail/remittance information
    PaymentDetailRemittanceInformation,
    /// Payment information
    PaymentInformation,
    /// Product information
    ProductInformation,
    /// Price calculation formula
    PriceCalculationFormula,
    /// Priority information
    PriorityInformation,
    /// Purchasing information
    PurchasingInformation,
    /// Quarantine instructions
    QuarantineInstructions,
    /// Quality demands/requirements
    QualityDemandsRequirements,
    /// Quotation instruction/information
    QuotationInstructionInformation,
    /// Risk and handling information
    RiskAndHandlingInformation,
    /// Regulatory information
    RegulatoryInformation,
    /// Return to origin information
    ReturnToOriginInformation,
    /// Receivables
    Receivables,
    /// Consignment route
    ConsignmentRoute,
    /// Safety information
    SafetyInformation,
    /// Consignment documentary instruction
    ConsignmentDocumentaryInstruction,
    /// Special instructions
    SpecialInstructions,
    /// Ship line requested
    ShipLineRequested,
    /// Special permission for transport, generally
    SpecialPermissionForTransportGenerally,
    /// Special permission concerning the goods to be transported
    SpecialPermissionConcerningGoodsToBeTransported,
    /// Special handling
    SpecialHandling,
    /// Special permission concerning package
    SpecialPermissionConcerningPackage,
    /// Special permission concerning transport means
    SpecialPermissionConcerningTransportMeans,
    /// Subsidiary risk number (IATA/DGR)
    SubsidiaryRiskNumberIataDgr,
    /// Special service request
    SpecialServiceRequest,
    /// Supplier remarks
    SupplierRemarks,
    /// Consignment tariff
    ConsignmentTariff,
    /// Consignment transport
    ConsignmentTransport,
    /// Transportation information
    TransportationInformation,
    /// Requested tariff
    RequestedTariff,
    /// Tax declaration
    TaxDeclaration,
    /// Warehouse instruction/information
    WarehouseInstructionInformation,
    /// Mutually defined
    MutuallyDefined,
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for Text {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s).ok_or(())
    }
}

impl crate::Code for Text {
    fn code(self) -> &'static str {
        match self {
            Text::GoodsItemDescription => "AAA",
            Text::PaymentTerm => "AAB",
            Text::DangerousGoodsAdditionalInformation => "AAC",
            Text::DangerousGoodsTechnicalName => "AAD",
            Text::AcknowledgementDescription => "AAE",
            Text::RateAdditionalInformation => "AAF",
            Text::PartyInstructions => "AAG",
            Text::GeneralInformation => "AAI",
            Text::AdditionalConditionsSalePurchase => "AAJ",
            Text::PriceConditions => "AAK",
            Text::GoodsDimensionsInCharacters => "AAL",
            Text::EquipmentReUsageRestrictions => "AAM",
            Text::HandlingRestriction => "AAN",
            Text::ErrorDescriptionFreeText => "AAO",
            Text::ResponseFreeText => "AAP",
            Text::PackageContentsDescription => "AAQ",
            Text::TermsDelivery => "AAR",
            Text::BillLadingRemarks => "AAS",
            Text::ModeSettlementInformation => "AAT",
            Text::ConsignmentInvoiceInformation => "AAU",
            Text::ClearanceInvoiceInformation => "AAV",
            Text::LetterCreditInformation => "AAW",
            Text::LicenseInformation => "AAX",
            Text::CertificationStatements => "AAY",
            Text::AdditionalExportInformation => "AAZ",
            Text::TariffStatements => "ABA",
            Text::MedicalHistory => "ABB",
            Text::ConditionsSaleOrPurchase => "ABC",
            Text::ContractDocumentType => "ABD",
            Text::AdditionalTermsAndOrConditionsDocumentaryCredit => "ABE",
            Text::InstructionsOrInformationAboutStandbyDocumentary => "ABF",
            Text::InstructionsOrInformationAboutPartialShipmentS => "ABG",
            Text::InstructionsOrInformationAboutTranshipmentS => "ABH",
            Text::AdditionalHandlingInstructionsDocumentaryCredit => "ABI",
            Text::DomesticRoutingInformation => "ABJ",
            Text::ChargeableCategoryEquipment => "ABK",
            Text::GovernmentInformation => "ABL",
            Text::OnwardRoutingInformation => "ABM",
            Text::AccountingInformation => "ABN",
            Text::DiscrepancyInformation => "ABO",
            Text::ConfirmationInstructions => "ABP",
            Text::MethodIssuance => "ABQ",
            Text::DocumentsDeliveryInstructions => "ABR",
            Text::AdditionalConditions => "ABS",
            Text::InformationInstructionsAboutAdditionalAmountsCovered => "ABT",
            Text::DeferredPaymentTermedAdditional => "ABU",
            Text::AcceptanceTermsAdditional => "ABV",
            Text::NegotiationTermsAdditional => "ABW",
            Text::DocumentNameAndDocumentaryRequirements => "ABX",
            Text::InstructionsInformationAboutRevolvingDocumentaryCredit => "ABZ",
            Text::DocumentaryRequirements => "ACA",
            Text::AdditionalInformation => "ACB",
            Text::FactorAssignmentClause => "ACC",
            Text::Reason => "ACD",
            Text::Dispute => "ACE",
            Text::AdditionalAttributeInformation => "ACF",
            Text::AbsenceDeclaration => "ACG",
            Text::AggregationStatement => "ACH",
            Text::CompilationStatement => "ACI",
            Text::DefinitionalException => "ACJ",
            Text::PrivacyStatement => "ACK",
            Text::QualityStatement => "ACL",
            Text::StatisticalDescription => "ACM",
            Text::StatisticalDefinition => "ACN",
            Text::StatisticalName => "ACO",
            Text::StatisticalTitle => "ACP",
            Text::OffDimensionInformation => "ACQ",
            Text::UnexpectedStopsInformation => "ACR",
            Text::Principles => "ACS",
            Text::TermsAndDefinition => "ACT",
            Text::SegmentName => "ACU",
            Text::SimpleDataElementName => "ACV",
            Text::Scope => "ACW",
            Text::MessageTypeName => "ACX",
            Text::Introduction => "ACY",
            Text::Glossary => "ACZ",
            Text::FunctionalDefinition => "ADA",
            Text::Examples => "ADB",
            Text::CoverPage => "ADC",
            Text::DependencySyntaxNotes => "ADD",
            Text::CodeValueName => "ADE",
            Text::CodeListName => "ADF",
            Text::ClarificationUsage => "ADG",
            Text::CompositeDataElementName => "ADH",
            Text::FieldApplication => "ADI",
            Text::TypeAssetsAndLiabilities => "ADJ",
            Text::PromotionInformation => "ADK",
            Text::MeterCondition => "ADL",
            Text::MeterReadingInformation => "ADM",
            Text::TypeTransactionReason => "ADN",
            Text::TypeSurveyQuestion => "ADO",
            Text::CarriersAgentCounterInformation => "ADP",
            Text::DescriptionWorkItemOnEquipment => "ADQ",
            Text::MessageDefinition => "ADR",
            Text::BookedItemInformation => "ADS",
            Text::SourceDocument => "ADT",
            Text::Note => "ADU",
            Text::FixedPartSegmentClarificationText => "ADV",
            Text::CharacteristicsGoods => "ADW",
            Text::AdditionalDischargeInstructions => "ADX",
            Text::ContainerStrippingInstructions => "ADY",
            Text::CscContainerSafetyConventionPlateInformation => "ADZ",
            Text::CargoRemarks => "AEA",
            Text::TemperatureControlInstructions => "AEB",
            Text::TextRefersToExpectedData => "AEC",
            Text::TextRefersToReceivedData => "AED",
            Text::SectionClarificationText => "AEE",
            Text::InformationToBeneficiary => "AEF",
            Text::InformationToApplicant => "AEG",
            Text::InstructionsToBeneficiary => "AEH",
            Text::InstructionsToApplicant => "AEI",
            Text::ControlledAtmosphere => "AEJ",
            Text::TakeOffAnnotation => "AEK",
            Text::PriceVariationNarrative => "AEL",
            Text::DocumentaryCreditAmendmentInstructions => "AEM",
            Text::StandardMethodNarrative => "AEN",
            Text::ProjectNarrative => "AEO",
            Text::RadioactiveGoodsAdditionalInformation => "AEP",
            Text::BankToBankInformation => "AEQ",
            Text::ReimbursementInstructions => "AER",
            Text::ReasonForAmendingAMessage => "AES",
            Text::InstructionsToPayingAndOrAcceptingAndOr => "AET",
            Text::InterestInstructions => "AEU",
            Text::AgentCommission => "AEV",
            Text::RemittingBankInstructions => "AEW",
            Text::InstructionsToCollectingBank => "AEX",
            Text::CollectionAmountInstructions => "AEY",
            Text::InternalAuditingInformation => "AEZ",
            Text::Constraint => "AFA",
            Text::Comment => "AFB",
            Text::SemanticNote => "AFC",
            Text::HelpText => "AFD",
            Text::Legend => "AFE",
            Text::BatchCodeStructure => "AFF",
            Text::ProductApplication => "AFG",
            Text::CustomerComplaint => "AFH",
            Text::ProbableCauseFault => "AFI",
            Text::DefectDescription => "AFJ",
            Text::RepairDescription => "AFK",
            Text::ReviewComments => "AFL",
            Text::Title => "AFM",
            Text::DescriptionAmount => "AFN",
            Text::Responsibilities => "AFO",
            Text::Supplier => "AFP",
            Text::PurchaseRegion => "AFQ",
            Text::Affiliation => "AFR",
            Text::Borrower => "AFS",
            Text::LineBusiness => "AFT",
            Text::FinancialInstitution => "AFU",
            Text::BusinessFounder => "AFV",
            Text::BusinessHistory => "AFW",
            Text::BankingArrangements => "AFX",
            Text::BusinessOrigin => "AFY",
            Text::BrandNamesDescription => "AFZ",
            Text::BusinessFinancingDetails => "AGA",
            Text::Competition => "AGB",
            Text::ConstructionProcessDetails => "AGC",
            Text::ConstructionSpecialty => "AGD",
            Text::ContractInformation => "AGE",
            Text::CorporateFiling => "AGF",
            Text::CustomerInformation => "AGG",
            Text::CopyrightNotice => "AGH",
            Text::ContingentDebt => "AGI",
            Text::ConvictionDetails => "AGJ",
            Text::Equipment => "AGK",
            Text::WorkforceDescription => "AGL",
            Text::Exemption => "AGM",
            Text::FuturePlans => "AGN",
            Text::IntervieweeConversationInformation => "AGO",
            Text::IntangibleAsset => "AGP",
            Text::Inventory => "AGQ",
            Text::Investment => "AGR",
            Text::IntercompanyRelationsInformation => "AGS",
            Text::JointVenture => "AGT",
            Text::Loan => "AGU",
            Text::LongTermDebt => "AGV",
            Text::Location => "AGW",
            Text::CurrentLegalStructure => "AGX",
            Text::MaritalContract => "AGY",
            Text::MarketingActivities => "AGZ",
            Text::Merger => "AHA",
            Text::MarketableSecurities => "AHB",
            Text::BusinessDebt => "AHC",
            Text::OriginalLegalStructure => "AHD",
            Text::EmployeeSharingArrangements => "AHE",
            Text::OrganizationDetails => "AHF",
            Text::PublicRecordDetails => "AHG",
            Text::PriceRange => "AHH",
            Text::Qualifications => "AHI",
            Text::RegisteredActivity => "AHJ",
            Text::CriminalSentence => "AHK",
            Text::SalesMethod => "AHL",
            Text::EducationalInstitutionInformation => "AHM",
            Text::StatusDetails => "AHN",
            Text::Sales => "AHO",
            Text::SpouseInformation => "AHP",
            Text::EducationalDegreeInformation => "AHQ",
            Text::ShareholdingInformation => "AHR",
            Text::SalesTerritory => "AHS",
            Text::AccountantsComments => "AHT",
            Text::ExemptionLawLocation => "AHU",
            Text::ShareClassifications => "AHV",
            Text::Forecast => "AHW",
            Text::EventLocation => "AHX",
            Text::FacilityOccupancy => "AHY",
            Text::ImportAndExportDetails => "AHZ",
            Text::AdditionalFacilityInformation => "AIA",
            Text::InventoryValue => "AIB",
            Text::Education => "AIC",
            Text::Event => "AID",
            Text::Agent => "AIE",
            Text::DomesticallyAgreedFinancialStatementDetails => "AIF",
            Text::OtherCurrentAssetDescription => "AIG",
            Text::OtherCurrentLiabilityDescription => "AIH",
            Text::FormerBusinessActivity => "AII",
            Text::TradeNameUse => "AIJ",
            Text::SigningAuthority => "AIK",
            Text::Guarantee => "AIL",
            Text::HoldingCompanyOperation => "AIM",
            Text::ConsignmentRouting => "AIN",
            Text::LetterProtest => "AIO",
            Text::Question => "AIP",
            Text::PartyInformation => "AIQ",
            Text::AreaBoundariesDescription => "AIR",
            Text::AdvertisementInformation => "AIS",
            Text::FinancialStatementDetails => "AIT",
            Text::AccessInstructions => "AIU",
            Text::Liquidity => "AIV",
            Text::CreditLine => "AIW",
            Text::WarrantyTerms => "AIX",
            Text::DivisionDescription => "AIY",
            Text::ReportingInstruction => "AIZ",
            Text::ExaminationResult => "AJA",
            Text::LaboratoryResult => "AJB",
            Text::AllowanceChargeInformation => "ALC",
            Text::XRayResult => "ALD",
            Text::PathologyResult => "ALE",
            Text::InterventionDescription => "ALF",
            Text::SummaryAdmittance => "ALG",
            Text::MedicalTreatmentCourseDetail => "ALH",
            Text::Prognosis => "ALI",
            Text::InstructionToPatient => "ALJ",
            Text::InstructionToPhysician => "ALK",
            Text::AllDocuments => "ALL",
            Text::MedicineTreatment => "ALM",
            Text::MedicineDosageAndAdministration => "ALN",
            Text::AvailabilityPatient => "ALO",
            Text::ReasonForServiceRequest => "ALP",
            Text::PurposeService => "ALQ",
            Text::ArrivalConditions => "ARR",
            Text::ServiceRequestersComment => "ARS",
            Text::Authentication => "AUT",
            Text::RequestedLocationDescription => "AUU",
            Text::MedicineAdministrationCondition => "AUV",
            Text::PatientInformation => "AUW",
            Text::PrecautionaryMeasure => "AUX",
            Text::ServiceCharacteristic => "AUY",
            Text::PlannedEventComment => "AUZ",
            Text::ExpectedDelayComment => "AVA",
            Text::TransportRequirementsComment => "AVB",
            Text::TemporaryApprovalCondition => "AVC",
            Text::CustomsValuationInformation => "AVD",
            Text::ValueAddedTaxVatMarginScheme => "AVE",
            Text::MaritimeDeclarationHealth => "AVF",
            Text::PassengerBaggageInformation => "BAG",
            Text::MaritimeDeclarationHealth_Dup => "BAH",
            Text::AdditionalProductInformationAddress => "BAI",
            Text::InformationToBePrintedOnDespatchAdvice => "BAJ",
            Text::MissingGoodsRemarks => "BAK",
            Text::NonAcceptanceInformation => "BAL",
            Text::ReturnsInformation => "BAM",
            Text::SubLineItemInformation => "BAN",
            Text::TestInformation => "BAO",
            Text::ExternalLink => "BAP",
            Text::VatExemptionReason => "BAQ",
            Text::ProcessingInstructions => "BAR",
            Text::RelayInstructions => "BAS",
            Text::SimaApplicable => "BAT",
            Text::AppealsProgramCode => "BAU",
            Text::SimaSubject => "BAV",
            Text::SurtaxApplicable => "BAW",
            Text::SimaSecurityBond => "BAX",
            Text::SurtaxSubject => "BAY",
            Text::SafeguardApplicable => "BAZ",
            Text::SafeguardApplicable_Dup => "BBA",
            Text::SafeguardSubject => "BBB",
            Text::TransportContractDocumentClause => "BLC",
            Text::InstructionToPreparePatient => "BLD",
            Text::MedicineTreatmentComment => "BLE",
            Text::ExaminationResultComment => "BLF",
            Text::ServiceRequestComment => "BLG",
            Text::PrescriptionReason => "BLH",
            Text::PrescriptionComment => "BLI",
            Text::ClinicalInvestigationComment => "BLJ",
            Text::MedicinalSpecificationComment => "BLK",
            Text::EconomicContributionComment => "BLL",
            Text::StatusAPlan => "BLM",
            Text::RandomSampleTestInformation => "BLN",
            Text::PeriodTime => "BLO",
            Text::Legislation => "BLP",
            Text::SecurityMeasuresRequested => "BLQ",
            Text::TransportContractDocumentRemark => "BLR",
            Text::PreviousPortCallSecurityInformation => "BLS",
            Text::SecurityInformation => "BLT",
            Text::WasteInformation => "BLU",
            Text::B2cMarketingInformationShortDescription => "BLV",
            Text::B2bMarketingInformationLongDescription => "BLW",
            Text::B2cMarketingInformationLongDescription => "BLX",
            Text::ProductIngredients => "BLY",
            Text::LocationShortName => "BLZ",
            Text::PackagingMaterialInformation => "BMA",
            Text::FillerMaterialInformation => "BMB",
            Text::ShipToShipActivityInformation => "BMC",
            Text::PackageMaterialDescription => "BMD",
            Text::ConsumerLevelPackageMarking => "BME",
            Text::SimaMeasureInForce => "BMF",
            Text::PreCarm => "BMG",
            Text::SimaMeasureType => "BMH",
            Text::CustomsClearanceInstructions => "CCI",
            Text::SubTypeCode => "CCJ",
            Text::SimaInformation => "CCK",
            Text::TimeLimitEnd => "CCL",
            Text::TimeLimitStart => "CCM",
            Text::WarehouseTimeLimit => "CCN",
            Text::ValueForDutyInformation => "CCO",
            Text::CustomsClearanceInstructionsExport => "CEX",
            Text::ChangeInformation => "CHG",
            Text::CustomsClearanceInstructionImport => "CIP",
            Text::ClearancePlaceRequested => "CLP",
            Text::LoadingRemarks => "CLR",
            Text::OrderInformation => "COI",
            Text::CustomerRemarks => "CUR",
            Text::CustomsDeclarationInformation => "CUS",
            Text::DamageRemarks => "DAR",
            Text::DocumentIssuerDeclaration => "DCL",
            Text::DeliveryInformation => "DEL",
            Text::DeliveryInstructions => "DIN",
            Text::DocumentationInstructions => "DOC",
            Text::DutyDeclaration => "DUT",
            Text::EffectiveUsedRouting => "EUR",
            Text::FirstBlockToBePrintedOnTransportContract => "FBC",
            Text::GovernmentBillLadingInformation => "GBL",
            Text::EntireTransactionSet => "GEN",
            Text::FurtherInformationConcerningGgvsPar7 => "GS7",
            Text::ConsignmentHandlingInstruction => "HAN",
            Text::HazardInformation => "HAZ",
            Text::ConsignmentInformationForConsignee => "ICN",
            Text::InsuranceInstructions => "IIN",
            Text::InvoiceMailingInstructions => "IMI",
            Text::CommercialInvoiceItemDescription => "IND",
            Text::InsuranceInformation => "INS",
            Text::InvoiceInstruction => "INV",
            Text::InformationForRailwayPurpose => "IRP",
            Text::InlandTransportDetails => "ITR",
            Text::TestingInstructions => "ITS",
            Text::LocationAlias => "LAN",
            Text::LineItem => "LIN",
            Text::LoadingInstruction => "LOI",
            Text::MiscellaneousChargeOrder => "MCO",
            Text::MaritimeDeclarationHealth_Dup_Dup => "MDH",
            Text::AdditionalMarksNumbersInformation => "MKS",
            Text::OrderInstruction => "ORI",
            Text::OtherServiceInformation => "OSI",
            Text::PackingMarkingInformation => "PAC",
            Text::PaymentInstructionsInformation => "PAI",
            Text::PayablesInformation => "PAY",
            Text::PackagingInformation => "PKG",
            Text::PackagingTermsInformation => "PKT",
            Text::PaymentDetailRemittanceInformation => "PMD",
            Text::PaymentInformation => "PMT",
            Text::ProductInformation => "PRD",
            Text::PriceCalculationFormula => "PRF",
            Text::PriorityInformation => "PRI",
            Text::PurchasingInformation => "PUR",
            Text::QuarantineInstructions => "QIN",
            Text::QualityDemandsRequirements => "QQD",
            Text::QuotationInstructionInformation => "QUT",
            Text::RiskAndHandlingInformation => "RAH",
            Text::RegulatoryInformation => "REG",
            Text::ReturnToOriginInformation => "RET",
            Text::Receivables => "REV",
            Text::ConsignmentRoute => "RQR",
            Text::SafetyInformation => "SAF",
            Text::ConsignmentDocumentaryInstruction => "SIC",
            Text::SpecialInstructions => "SIN",
            Text::ShipLineRequested => "SLR",
            Text::SpecialPermissionForTransportGenerally => "SPA",
            Text::SpecialPermissionConcerningGoodsToBeTransported => "SPG",
            Text::SpecialHandling => "SPH",
            Text::SpecialPermissionConcerningPackage => "SPP",
            Text::SpecialPermissionConcerningTransportMeans => "SPT",
            Text::SubsidiaryRiskNumberIataDgr => "SRN",
            Text::SpecialServiceRequest => "SSR",
            Text::SupplierRemarks => "SUR",
            Text::ConsignmentTariff => "TCA",
            Text::ConsignmentTransport => "TDT",
            Text::TransportationInformation => "TRA",
            Text::RequestedTariff => "TRR",
            Text::TaxDeclaration => "TXD",
            Text::WarehouseInstructionInformation => "WHI",
            Text::MutuallyDefined => "ZZZ",
        }
    }
}

impl crate::Description for Text {
    fn description(self) -> &'static str {
        match self {
            Text::GoodsItemDescription => "Goods item description",
            Text::PaymentTerm => "Payment term",
            Text::DangerousGoodsAdditionalInformation => "Dangerous goods additional information",
            Text::DangerousGoodsTechnicalName => "Dangerous goods technical name",
            Text::AcknowledgementDescription => "Acknowledgement description",
            Text::RateAdditionalInformation => "Rate additional information",
            Text::PartyInstructions => "Party instructions",
            Text::GeneralInformation => "General information",
            Text::AdditionalConditionsSalePurchase => "Additional conditions of sale/purchase",
            Text::PriceConditions => "Price conditions",
            Text::GoodsDimensionsInCharacters => "Goods dimensions in characters",
            Text::EquipmentReUsageRestrictions => "Equipment re-usage restrictions",
            Text::HandlingRestriction => "Handling restriction",
            Text::ErrorDescriptionFreeText => "Error description (free text)",
            Text::ResponseFreeText => "Response (free text)",
            Text::PackageContentsDescription => "Package content's description",
            Text::TermsDelivery => "Terms of delivery",
            Text::BillLadingRemarks => "Bill of lading remarks",
            Text::ModeSettlementInformation => "Mode of settlement information",
            Text::ConsignmentInvoiceInformation => "Consignment invoice information",
            Text::ClearanceInvoiceInformation => "Clearance invoice information",
            Text::LetterCreditInformation => "Letter of credit information",
            Text::LicenseInformation => "License information",
            Text::CertificationStatements => "Certification statements",
            Text::AdditionalExportInformation => "Additional export information",
            Text::TariffStatements => "Tariff statements",
            Text::MedicalHistory => "Medical history",
            Text::ConditionsSaleOrPurchase => "Conditions of sale or purchase",
            Text::ContractDocumentType => "Contract document type",
            Text::AdditionalTermsAndOrConditionsDocumentaryCredit => {
                "Additional terms and/or conditions (documentary credit)"
            }
            Text::InstructionsOrInformationAboutStandbyDocumentary => {
                "Instructions or information about standby documentary"
            }
            Text::InstructionsOrInformationAboutPartialShipmentS => {
                "Instructions or information about partial shipment(s)"
            }
            Text::InstructionsOrInformationAboutTranshipmentS => {
                "Instructions or information about transhipment(s)"
            }
            Text::AdditionalHandlingInstructionsDocumentaryCredit => {
                "Additional handling instructions documentary credit"
            }
            Text::DomesticRoutingInformation => "Domestic routing information",
            Text::ChargeableCategoryEquipment => "Chargeable category of equipment",
            Text::GovernmentInformation => "Government information",
            Text::OnwardRoutingInformation => "Onward routing information",
            Text::AccountingInformation => "Accounting information",
            Text::DiscrepancyInformation => "Discrepancy information",
            Text::ConfirmationInstructions => "Confirmation instructions",
            Text::MethodIssuance => "Method of issuance",
            Text::DocumentsDeliveryInstructions => "Documents delivery instructions",
            Text::AdditionalConditions => "Additional conditions",
            Text::InformationInstructionsAboutAdditionalAmountsCovered => {
                "Information/instructions about additional amounts covered"
            }
            Text::DeferredPaymentTermedAdditional => "Deferred payment termed additional",
            Text::AcceptanceTermsAdditional => "Acceptance terms additional",
            Text::NegotiationTermsAdditional => "Negotiation terms additional",
            Text::DocumentNameAndDocumentaryRequirements => {
                "Document name and documentary requirements"
            }
            Text::InstructionsInformationAboutRevolvingDocumentaryCredit => {
                "Instructions/information about revolving documentary credit"
            }
            Text::DocumentaryRequirements => "Documentary requirements",
            Text::AdditionalInformation => "Additional information",
            Text::FactorAssignmentClause => "Factor assignment clause",
            Text::Reason => "Reason",
            Text::Dispute => "Dispute",
            Text::AdditionalAttributeInformation => "Additional attribute information",
            Text::AbsenceDeclaration => "Absence declaration",
            Text::AggregationStatement => "Aggregation statement",
            Text::CompilationStatement => "Compilation statement",
            Text::DefinitionalException => "Definitional exception",
            Text::PrivacyStatement => "Privacy statement",
            Text::QualityStatement => "Quality statement",
            Text::StatisticalDescription => "Statistical description",
            Text::StatisticalDefinition => "Statistical definition",
            Text::StatisticalName => "Statistical name",
            Text::StatisticalTitle => "Statistical title",
            Text::OffDimensionInformation => "Off-dimension information",
            Text::UnexpectedStopsInformation => "Unexpected stops information",
            Text::Principles => "Principles",
            Text::TermsAndDefinition => "Terms and definition",
            Text::SegmentName => "Segment name",
            Text::SimpleDataElementName => "Simple data element name",
            Text::Scope => "Scope",
            Text::MessageTypeName => "Message type name",
            Text::Introduction => "Introduction",
            Text::Glossary => "Glossary",
            Text::FunctionalDefinition => "Functional definition",
            Text::Examples => "Examples",
            Text::CoverPage => "Cover page",
            Text::DependencySyntaxNotes => "Dependency (syntax) notes",
            Text::CodeValueName => "Code value name",
            Text::CodeListName => "Code list name",
            Text::ClarificationUsage => "Clarification of usage",
            Text::CompositeDataElementName => "Composite data element name",
            Text::FieldApplication => "Field of application",
            Text::TypeAssetsAndLiabilities => "Type of assets and liabilities",
            Text::PromotionInformation => "Promotion information",
            Text::MeterCondition => "Meter condition",
            Text::MeterReadingInformation => "Meter reading information",
            Text::TypeTransactionReason => "Type of transaction reason",
            Text::TypeSurveyQuestion => "Type of survey question",
            Text::CarriersAgentCounterInformation => "Carrier's agent counter information",
            Text::DescriptionWorkItemOnEquipment => "Description of work item on equipment",
            Text::MessageDefinition => "Message definition",
            Text::BookedItemInformation => "Booked item information",
            Text::SourceDocument => "Source of document",
            Text::Note => "Note",
            Text::FixedPartSegmentClarificationText => "Fixed part of segment clarification text",
            Text::CharacteristicsGoods => "Characteristics of goods",
            Text::AdditionalDischargeInstructions => "Additional discharge instructions",
            Text::ContainerStrippingInstructions => "Container stripping instructions",
            Text::CscContainerSafetyConventionPlateInformation => {
                "CSC (Container Safety Convention) plate information"
            }
            Text::CargoRemarks => "Cargo remarks",
            Text::TemperatureControlInstructions => "Temperature control instructions",
            Text::TextRefersToExpectedData => "Text refers to expected data",
            Text::TextRefersToReceivedData => "Text refers to received data",
            Text::SectionClarificationText => "Section clarification text",
            Text::InformationToBeneficiary => "Information to the beneficiary",
            Text::InformationToApplicant => "Information to the applicant",
            Text::InstructionsToBeneficiary => "Instructions to the beneficiary",
            Text::InstructionsToApplicant => "Instructions to the applicant",
            Text::ControlledAtmosphere => "Controlled atmosphere",
            Text::TakeOffAnnotation => "Take off annotation",
            Text::PriceVariationNarrative => "Price variation narrative",
            Text::DocumentaryCreditAmendmentInstructions => {
                "Documentary credit amendment instructions"
            }
            Text::StandardMethodNarrative => "Standard method narrative",
            Text::ProjectNarrative => "Project narrative",
            Text::RadioactiveGoodsAdditionalInformation => {
                "Radioactive goods, additional information"
            }
            Text::BankToBankInformation => "Bank-to-bank information",
            Text::ReimbursementInstructions => "Reimbursement instructions",
            Text::ReasonForAmendingAMessage => "Reason for amending a message",
            Text::InstructionsToPayingAndOrAcceptingAndOr => {
                "Instructions to the paying and/or accepting and/or"
            }
            Text::InterestInstructions => "Interest instructions",
            Text::AgentCommission => "Agent commission",
            Text::RemittingBankInstructions => "Remitting bank instructions",
            Text::InstructionsToCollectingBank => "Instructions to the collecting bank",
            Text::CollectionAmountInstructions => "Collection amount instructions",
            Text::InternalAuditingInformation => "Internal auditing information",
            Text::Constraint => "Constraint",
            Text::Comment => "Comment",
            Text::SemanticNote => "Semantic note",
            Text::HelpText => "Help text",
            Text::Legend => "Legend",
            Text::BatchCodeStructure => "Batch code structure",
            Text::ProductApplication => "Product application",
            Text::CustomerComplaint => "Customer complaint",
            Text::ProbableCauseFault => "Probable cause of fault",
            Text::DefectDescription => "Defect description",
            Text::RepairDescription => "Repair description",
            Text::ReviewComments => "Review comments",
            Text::Title => "Title",
            Text::DescriptionAmount => "Description of amount",
            Text::Responsibilities => "Responsibilities",
            Text::Supplier => "Supplier",
            Text::PurchaseRegion => "Purchase region",
            Text::Affiliation => "Affiliation",
            Text::Borrower => "Borrower",
            Text::LineBusiness => "Line of business",
            Text::FinancialInstitution => "Financial institution",
            Text::BusinessFounder => "Business founder",
            Text::BusinessHistory => "Business history",
            Text::BankingArrangements => "Banking arrangements",
            Text::BusinessOrigin => "Business origin",
            Text::BrandNamesDescription => "Brand names' description",
            Text::BusinessFinancingDetails => "Business financing details",
            Text::Competition => "Competition",
            Text::ConstructionProcessDetails => "Construction process details",
            Text::ConstructionSpecialty => "Construction specialty",
            Text::ContractInformation => "Contract information",
            Text::CorporateFiling => "Corporate filing",
            Text::CustomerInformation => "Customer information",
            Text::CopyrightNotice => "Copyright notice",
            Text::ContingentDebt => "Contingent debt",
            Text::ConvictionDetails => "Conviction details",
            Text::Equipment => "Equipment",
            Text::WorkforceDescription => "Workforce description",
            Text::Exemption => "Exemption",
            Text::FuturePlans => "Future plans",
            Text::IntervieweeConversationInformation => "Interviewee conversation information",
            Text::IntangibleAsset => "Intangible asset",
            Text::Inventory => "Inventory",
            Text::Investment => "Investment",
            Text::IntercompanyRelationsInformation => "Intercompany relations information",
            Text::JointVenture => "Joint venture",
            Text::Loan => "Loan",
            Text::LongTermDebt => "Long term debt",
            Text::Location => "Location",
            Text::CurrentLegalStructure => "Current legal structure",
            Text::MaritalContract => "Marital contract",
            Text::MarketingActivities => "Marketing activities",
            Text::Merger => "Merger",
            Text::MarketableSecurities => "Marketable securities",
            Text::BusinessDebt => "Business debt",
            Text::OriginalLegalStructure => "Original legal structure",
            Text::EmployeeSharingArrangements => "Employee sharing arrangements",
            Text::OrganizationDetails => "Organization details",
            Text::PublicRecordDetails => "Public record details",
            Text::PriceRange => "Price range",
            Text::Qualifications => "Qualifications",
            Text::RegisteredActivity => "Registered activity",
            Text::CriminalSentence => "Criminal sentence",
            Text::SalesMethod => "Sales method",
            Text::EducationalInstitutionInformation => "Educational institution information",
            Text::StatusDetails => "Status details",
            Text::Sales => "Sales",
            Text::SpouseInformation => "Spouse information",
            Text::EducationalDegreeInformation => "Educational degree information",
            Text::ShareholdingInformation => "Shareholding information",
            Text::SalesTerritory => "Sales territory",
            Text::AccountantsComments => "Accountant's comments",
            Text::ExemptionLawLocation => "Exemption law location",
            Text::ShareClassifications => "Share classifications",
            Text::Forecast => "Forecast",
            Text::EventLocation => "Event location",
            Text::FacilityOccupancy => "Facility occupancy",
            Text::ImportAndExportDetails => "Import and export details",
            Text::AdditionalFacilityInformation => "Additional facility information",
            Text::InventoryValue => "Inventory value",
            Text::Education => "Education",
            Text::Event => "Event",
            Text::Agent => "Agent",
            Text::DomesticallyAgreedFinancialStatementDetails => {
                "Domestically agreed financial statement details"
            }
            Text::OtherCurrentAssetDescription => "Other current asset description",
            Text::OtherCurrentLiabilityDescription => "Other current liability description",
            Text::FormerBusinessActivity => "Former business activity",
            Text::TradeNameUse => "Trade name use",
            Text::SigningAuthority => "Signing authority",
            Text::Guarantee => "Guarantee",
            Text::HoldingCompanyOperation => "Holding company operation",
            Text::ConsignmentRouting => "Consignment routing",
            Text::LetterProtest => "Letter of protest",
            Text::Question => "Question",
            Text::PartyInformation => "Party information",
            Text::AreaBoundariesDescription => "Area boundaries description",
            Text::AdvertisementInformation => "Advertisement information",
            Text::FinancialStatementDetails => "Financial statement details",
            Text::AccessInstructions => "Access instructions",
            Text::Liquidity => "Liquidity",
            Text::CreditLine => "Credit line",
            Text::WarrantyTerms => "Warranty terms",
            Text::DivisionDescription => "Division description",
            Text::ReportingInstruction => "Reporting instruction",
            Text::ExaminationResult => "Examination result",
            Text::LaboratoryResult => "Laboratory result",
            Text::AllowanceChargeInformation => "Allowance/charge information",
            Text::XRayResult => "X-ray result",
            Text::PathologyResult => "Pathology result",
            Text::InterventionDescription => "Intervention description",
            Text::SummaryAdmittance => "Summary of admittance",
            Text::MedicalTreatmentCourseDetail => "Medical treatment course detail",
            Text::Prognosis => "Prognosis",
            Text::InstructionToPatient => "Instruction to patient",
            Text::InstructionToPhysician => "Instruction to physician",
            Text::AllDocuments => "All documents",
            Text::MedicineTreatment => "Medicine treatment",
            Text::MedicineDosageAndAdministration => "Medicine dosage and administration",
            Text::AvailabilityPatient => "Availability of patient",
            Text::ReasonForServiceRequest => "Reason for service request",
            Text::PurposeService => "Purpose of service",
            Text::ArrivalConditions => "Arrival conditions",
            Text::ServiceRequestersComment => "Service requester's comment",
            Text::Authentication => "Authentication",
            Text::RequestedLocationDescription => "Requested location description",
            Text::MedicineAdministrationCondition => "Medicine administration condition",
            Text::PatientInformation => "Patient information",
            Text::PrecautionaryMeasure => "Precautionary measure",
            Text::ServiceCharacteristic => "Service characteristic",
            Text::PlannedEventComment => "Planned event comment",
            Text::ExpectedDelayComment => "Expected delay comment",
            Text::TransportRequirementsComment => "Transport requirements comment",
            Text::TemporaryApprovalCondition => "Temporary approval condition",
            Text::CustomsValuationInformation => "Customs Valuation Information",
            Text::ValueAddedTaxVatMarginScheme => "Value Added Tax (VAT) margin scheme",
            Text::MaritimeDeclarationHealth => "Maritime Declaration of Health",
            Text::PassengerBaggageInformation => "Passenger baggage information",
            Text::MaritimeDeclarationHealth_Dup => "Maritime Declaration of Health",
            Text::AdditionalProductInformationAddress => "Additional product information address",
            Text::InformationToBePrintedOnDespatchAdvice => {
                "Information to be printed on despatch advice"
            }
            Text::MissingGoodsRemarks => "Missing goods remarks",
            Text::NonAcceptanceInformation => "Non-acceptance information",
            Text::ReturnsInformation => "Returns information",
            Text::SubLineItemInformation => "Sub-line item information",
            Text::TestInformation => "Test information",
            Text::ExternalLink => "External link",
            Text::VatExemptionReason => "VAT exemption reason",
            Text::ProcessingInstructions => "Processing Instructions",
            Text::RelayInstructions => "Relay Instructions",
            Text::SimaApplicable => "SIMA applicable",
            Text::AppealsProgramCode => "Appeals program code",
            Text::SimaSubject => "SIMA subject",
            Text::SurtaxApplicable => "Surtax applicable",
            Text::SimaSecurityBond => "SIMA security bond",
            Text::SurtaxSubject => "Surtax subject",
            Text::SafeguardApplicable => "Safeguard applicable",
            Text::SafeguardApplicable_Dup => "Safeguard applicable",
            Text::SafeguardSubject => "Safeguard subject",
            Text::TransportContractDocumentClause => "Transport contract document clause",
            Text::InstructionToPreparePatient => "Instruction to prepare the patient",
            Text::MedicineTreatmentComment => "Medicine treatment comment",
            Text::ExaminationResultComment => "Examination result comment",
            Text::ServiceRequestComment => "Service request comment",
            Text::PrescriptionReason => "Prescription reason",
            Text::PrescriptionComment => "Prescription comment",
            Text::ClinicalInvestigationComment => "Clinical investigation comment",
            Text::MedicinalSpecificationComment => "Medicinal specification comment",
            Text::EconomicContributionComment => "Economic contribution comment",
            Text::StatusAPlan => "Status of a plan",
            Text::RandomSampleTestInformation => "Random sample test information",
            Text::PeriodTime => "Period of time",
            Text::Legislation => "Legislation",
            Text::SecurityMeasuresRequested => "Security measures requested",
            Text::TransportContractDocumentRemark => "Transport contract document remark",
            Text::PreviousPortCallSecurityInformation => {
                "Previous port of call security information"
            }
            Text::SecurityInformation => "Security information",
            Text::WasteInformation => "Waste information",
            Text::B2cMarketingInformationShortDescription => {
                "B2C marketing information, short description"
            }
            Text::B2bMarketingInformationLongDescription => {
                "B2B marketing information, long description"
            }
            Text::B2cMarketingInformationLongDescription => {
                "B2C marketing information, long description"
            }
            Text::ProductIngredients => "Product ingredients",
            Text::LocationShortName => "Location short name",
            Text::PackagingMaterialInformation => "Packaging material information",
            Text::FillerMaterialInformation => "Filler material information",
            Text::ShipToShipActivityInformation => "Ship-to-ship activity information",
            Text::PackageMaterialDescription => "Package material description",
            Text::ConsumerLevelPackageMarking => "Consumer level package marking",
            Text::SimaMeasureInForce => "SIMA measure in force",
            Text::PreCarm => "Pre-CARM",
            Text::SimaMeasureType => "SIMA measure type",
            Text::CustomsClearanceInstructions => "Customs clearance instructions",
            Text::SubTypeCode => "Sub Type Code",
            Text::SimaInformation => "SIMA information",
            Text::TimeLimitEnd => "Time limit end",
            Text::TimeLimitStart => "Time limit start",
            Text::WarehouseTimeLimit => "Warehouse time limit",
            Text::ValueForDutyInformation => "Value for duty information",
            Text::CustomsClearanceInstructionsExport => "Customs clearance instructions export",
            Text::ChangeInformation => "Change information",
            Text::CustomsClearanceInstructionImport => "Customs clearance instruction import",
            Text::ClearancePlaceRequested => "Clearance place requested",
            Text::LoadingRemarks => "Loading remarks",
            Text::OrderInformation => "Order information",
            Text::CustomerRemarks => "Customer remarks",
            Text::CustomsDeclarationInformation => "Customs declaration information",
            Text::DamageRemarks => "Damage remarks",
            Text::DocumentIssuerDeclaration => "Document issuer declaration",
            Text::DeliveryInformation => "Delivery information",
            Text::DeliveryInstructions => "Delivery instructions",
            Text::DocumentationInstructions => "Documentation instructions",
            Text::DutyDeclaration => "Duty declaration",
            Text::EffectiveUsedRouting => "Effective used routing",
            Text::FirstBlockToBePrintedOnTransportContract => {
                "First block to be printed on the transport contract"
            }
            Text::GovernmentBillLadingInformation => "Government bill of lading information",
            Text::EntireTransactionSet => "Entire transaction set",
            Text::FurtherInformationConcerningGgvsPar7 => {
                "Further information concerning GGVS par. 7"
            }
            Text::ConsignmentHandlingInstruction => "Consignment handling instruction",
            Text::HazardInformation => "Hazard information",
            Text::ConsignmentInformationForConsignee => "Consignment information for consignee",
            Text::InsuranceInstructions => "Insurance instructions",
            Text::InvoiceMailingInstructions => "Invoice mailing instructions",
            Text::CommercialInvoiceItemDescription => "Commercial invoice item description",
            Text::InsuranceInformation => "Insurance information",
            Text::InvoiceInstruction => "Invoice instruction",
            Text::InformationForRailwayPurpose => "Information for railway purpose",
            Text::InlandTransportDetails => "Inland transport details",
            Text::TestingInstructions => "Testing instructions",
            Text::LocationAlias => "Location Alias",
            Text::LineItem => "Line item",
            Text::LoadingInstruction => "Loading instruction",
            Text::MiscellaneousChargeOrder => "Miscellaneous charge order",
            Text::MaritimeDeclarationHealth_Dup_Dup => "Maritime Declaration of Health",
            Text::AdditionalMarksNumbersInformation => "Additional marks/numbers information",
            Text::OrderInstruction => "Order instruction",
            Text::OtherServiceInformation => "Other service information",
            Text::PackingMarkingInformation => "Packing/marking information",
            Text::PaymentInstructionsInformation => "Payment instructions information",
            Text::PayablesInformation => "Payables information",
            Text::PackagingInformation => "Packaging information",
            Text::PackagingTermsInformation => "Packaging terms information",
            Text::PaymentDetailRemittanceInformation => "Payment detail/remittance information",
            Text::PaymentInformation => "Payment information",
            Text::ProductInformation => "Product information",
            Text::PriceCalculationFormula => "Price calculation formula",
            Text::PriorityInformation => "Priority information",
            Text::PurchasingInformation => "Purchasing information",
            Text::QuarantineInstructions => "Quarantine instructions",
            Text::QualityDemandsRequirements => "Quality demands/requirements",
            Text::QuotationInstructionInformation => "Quotation instruction/information",
            Text::RiskAndHandlingInformation => "Risk and handling information",
            Text::RegulatoryInformation => "Regulatory information",
            Text::ReturnToOriginInformation => "Return to origin information",
            Text::Receivables => "Receivables",
            Text::ConsignmentRoute => "Consignment route",
            Text::SafetyInformation => "Safety information",
            Text::ConsignmentDocumentaryInstruction => "Consignment documentary instruction",
            Text::SpecialInstructions => "Special instructions",
            Text::ShipLineRequested => "Ship line requested",
            Text::SpecialPermissionForTransportGenerally => {
                "Special permission for transport, generally"
            }
            Text::SpecialPermissionConcerningGoodsToBeTransported => {
                "Special permission concerning the goods to be transported"
            }
            Text::SpecialHandling => "Special handling",
            Text::SpecialPermissionConcerningPackage => "Special permission concerning package",
            Text::SpecialPermissionConcerningTransportMeans => {
                "Special permission concerning transport means"
            }
            Text::SubsidiaryRiskNumberIataDgr => "Subsidiary risk number (IATA/DGR)",
            Text::SpecialServiceRequest => "Special service request",
            Text::SupplierRemarks => "Supplier remarks",
            Text::ConsignmentTariff => "Consignment tariff",
            Text::ConsignmentTransport => "Consignment transport",
            Text::TransportationInformation => "Transportation information",
            Text::RequestedTariff => "Requested tariff",
            Text::TaxDeclaration => "Tax declaration",
            Text::WarehouseInstructionInformation => "Warehouse instruction/information",
            Text::MutuallyDefined => "Mutually defined",
        }
    }
}

impl crate::FromCode for Text {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "AAA" => Some(Text::GoodsItemDescription),
            "AAB" => Some(Text::PaymentTerm),
            "AAC" => Some(Text::DangerousGoodsAdditionalInformation),
            "AAD" => Some(Text::DangerousGoodsTechnicalName),
            "AAE" => Some(Text::AcknowledgementDescription),
            "AAF" => Some(Text::RateAdditionalInformation),
            "AAG" => Some(Text::PartyInstructions),
            "AAI" => Some(Text::GeneralInformation),
            "AAJ" => Some(Text::AdditionalConditionsSalePurchase),
            "AAK" => Some(Text::PriceConditions),
            "AAL" => Some(Text::GoodsDimensionsInCharacters),
            "AAM" => Some(Text::EquipmentReUsageRestrictions),
            "AAN" => Some(Text::HandlingRestriction),
            "AAO" => Some(Text::ErrorDescriptionFreeText),
            "AAP" => Some(Text::ResponseFreeText),
            "AAQ" => Some(Text::PackageContentsDescription),
            "AAR" => Some(Text::TermsDelivery),
            "AAS" => Some(Text::BillLadingRemarks),
            "AAT" => Some(Text::ModeSettlementInformation),
            "AAU" => Some(Text::ConsignmentInvoiceInformation),
            "AAV" => Some(Text::ClearanceInvoiceInformation),
            "AAW" => Some(Text::LetterCreditInformation),
            "AAX" => Some(Text::LicenseInformation),
            "AAY" => Some(Text::CertificationStatements),
            "AAZ" => Some(Text::AdditionalExportInformation),
            "ABA" => Some(Text::TariffStatements),
            "ABB" => Some(Text::MedicalHistory),
            "ABC" => Some(Text::ConditionsSaleOrPurchase),
            "ABD" => Some(Text::ContractDocumentType),
            "ABE" => Some(Text::AdditionalTermsAndOrConditionsDocumentaryCredit),
            "ABF" => Some(Text::InstructionsOrInformationAboutStandbyDocumentary),
            "ABG" => Some(Text::InstructionsOrInformationAboutPartialShipmentS),
            "ABH" => Some(Text::InstructionsOrInformationAboutTranshipmentS),
            "ABI" => Some(Text::AdditionalHandlingInstructionsDocumentaryCredit),
            "ABJ" => Some(Text::DomesticRoutingInformation),
            "ABK" => Some(Text::ChargeableCategoryEquipment),
            "ABL" => Some(Text::GovernmentInformation),
            "ABM" => Some(Text::OnwardRoutingInformation),
            "ABN" => Some(Text::AccountingInformation),
            "ABO" => Some(Text::DiscrepancyInformation),
            "ABP" => Some(Text::ConfirmationInstructions),
            "ABQ" => Some(Text::MethodIssuance),
            "ABR" => Some(Text::DocumentsDeliveryInstructions),
            "ABS" => Some(Text::AdditionalConditions),
            "ABT" => Some(Text::InformationInstructionsAboutAdditionalAmountsCovered),
            "ABU" => Some(Text::DeferredPaymentTermedAdditional),
            "ABV" => Some(Text::AcceptanceTermsAdditional),
            "ABW" => Some(Text::NegotiationTermsAdditional),
            "ABX" => Some(Text::DocumentNameAndDocumentaryRequirements),
            "ABZ" => Some(Text::InstructionsInformationAboutRevolvingDocumentaryCredit),
            "ACA" => Some(Text::DocumentaryRequirements),
            "ACB" => Some(Text::AdditionalInformation),
            "ACC" => Some(Text::FactorAssignmentClause),
            "ACD" => Some(Text::Reason),
            "ACE" => Some(Text::Dispute),
            "ACF" => Some(Text::AdditionalAttributeInformation),
            "ACG" => Some(Text::AbsenceDeclaration),
            "ACH" => Some(Text::AggregationStatement),
            "ACI" => Some(Text::CompilationStatement),
            "ACJ" => Some(Text::DefinitionalException),
            "ACK" => Some(Text::PrivacyStatement),
            "ACL" => Some(Text::QualityStatement),
            "ACM" => Some(Text::StatisticalDescription),
            "ACN" => Some(Text::StatisticalDefinition),
            "ACO" => Some(Text::StatisticalName),
            "ACP" => Some(Text::StatisticalTitle),
            "ACQ" => Some(Text::OffDimensionInformation),
            "ACR" => Some(Text::UnexpectedStopsInformation),
            "ACS" => Some(Text::Principles),
            "ACT" => Some(Text::TermsAndDefinition),
            "ACU" => Some(Text::SegmentName),
            "ACV" => Some(Text::SimpleDataElementName),
            "ACW" => Some(Text::Scope),
            "ACX" => Some(Text::MessageTypeName),
            "ACY" => Some(Text::Introduction),
            "ACZ" => Some(Text::Glossary),
            "ADA" => Some(Text::FunctionalDefinition),
            "ADB" => Some(Text::Examples),
            "ADC" => Some(Text::CoverPage),
            "ADD" => Some(Text::DependencySyntaxNotes),
            "ADE" => Some(Text::CodeValueName),
            "ADF" => Some(Text::CodeListName),
            "ADG" => Some(Text::ClarificationUsage),
            "ADH" => Some(Text::CompositeDataElementName),
            "ADI" => Some(Text::FieldApplication),
            "ADJ" => Some(Text::TypeAssetsAndLiabilities),
            "ADK" => Some(Text::PromotionInformation),
            "ADL" => Some(Text::MeterCondition),
            "ADM" => Some(Text::MeterReadingInformation),
            "ADN" => Some(Text::TypeTransactionReason),
            "ADO" => Some(Text::TypeSurveyQuestion),
            "ADP" => Some(Text::CarriersAgentCounterInformation),
            "ADQ" => Some(Text::DescriptionWorkItemOnEquipment),
            "ADR" => Some(Text::MessageDefinition),
            "ADS" => Some(Text::BookedItemInformation),
            "ADT" => Some(Text::SourceDocument),
            "ADU" => Some(Text::Note),
            "ADV" => Some(Text::FixedPartSegmentClarificationText),
            "ADW" => Some(Text::CharacteristicsGoods),
            "ADX" => Some(Text::AdditionalDischargeInstructions),
            "ADY" => Some(Text::ContainerStrippingInstructions),
            "ADZ" => Some(Text::CscContainerSafetyConventionPlateInformation),
            "AEA" => Some(Text::CargoRemarks),
            "AEB" => Some(Text::TemperatureControlInstructions),
            "AEC" => Some(Text::TextRefersToExpectedData),
            "AED" => Some(Text::TextRefersToReceivedData),
            "AEE" => Some(Text::SectionClarificationText),
            "AEF" => Some(Text::InformationToBeneficiary),
            "AEG" => Some(Text::InformationToApplicant),
            "AEH" => Some(Text::InstructionsToBeneficiary),
            "AEI" => Some(Text::InstructionsToApplicant),
            "AEJ" => Some(Text::ControlledAtmosphere),
            "AEK" => Some(Text::TakeOffAnnotation),
            "AEL" => Some(Text::PriceVariationNarrative),
            "AEM" => Some(Text::DocumentaryCreditAmendmentInstructions),
            "AEN" => Some(Text::StandardMethodNarrative),
            "AEO" => Some(Text::ProjectNarrative),
            "AEP" => Some(Text::RadioactiveGoodsAdditionalInformation),
            "AEQ" => Some(Text::BankToBankInformation),
            "AER" => Some(Text::ReimbursementInstructions),
            "AES" => Some(Text::ReasonForAmendingAMessage),
            "AET" => Some(Text::InstructionsToPayingAndOrAcceptingAndOr),
            "AEU" => Some(Text::InterestInstructions),
            "AEV" => Some(Text::AgentCommission),
            "AEW" => Some(Text::RemittingBankInstructions),
            "AEX" => Some(Text::InstructionsToCollectingBank),
            "AEY" => Some(Text::CollectionAmountInstructions),
            "AEZ" => Some(Text::InternalAuditingInformation),
            "AFA" => Some(Text::Constraint),
            "AFB" => Some(Text::Comment),
            "AFC" => Some(Text::SemanticNote),
            "AFD" => Some(Text::HelpText),
            "AFE" => Some(Text::Legend),
            "AFF" => Some(Text::BatchCodeStructure),
            "AFG" => Some(Text::ProductApplication),
            "AFH" => Some(Text::CustomerComplaint),
            "AFI" => Some(Text::ProbableCauseFault),
            "AFJ" => Some(Text::DefectDescription),
            "AFK" => Some(Text::RepairDescription),
            "AFL" => Some(Text::ReviewComments),
            "AFM" => Some(Text::Title),
            "AFN" => Some(Text::DescriptionAmount),
            "AFO" => Some(Text::Responsibilities),
            "AFP" => Some(Text::Supplier),
            "AFQ" => Some(Text::PurchaseRegion),
            "AFR" => Some(Text::Affiliation),
            "AFS" => Some(Text::Borrower),
            "AFT" => Some(Text::LineBusiness),
            "AFU" => Some(Text::FinancialInstitution),
            "AFV" => Some(Text::BusinessFounder),
            "AFW" => Some(Text::BusinessHistory),
            "AFX" => Some(Text::BankingArrangements),
            "AFY" => Some(Text::BusinessOrigin),
            "AFZ" => Some(Text::BrandNamesDescription),
            "AGA" => Some(Text::BusinessFinancingDetails),
            "AGB" => Some(Text::Competition),
            "AGC" => Some(Text::ConstructionProcessDetails),
            "AGD" => Some(Text::ConstructionSpecialty),
            "AGE" => Some(Text::ContractInformation),
            "AGF" => Some(Text::CorporateFiling),
            "AGG" => Some(Text::CustomerInformation),
            "AGH" => Some(Text::CopyrightNotice),
            "AGI" => Some(Text::ContingentDebt),
            "AGJ" => Some(Text::ConvictionDetails),
            "AGK" => Some(Text::Equipment),
            "AGL" => Some(Text::WorkforceDescription),
            "AGM" => Some(Text::Exemption),
            "AGN" => Some(Text::FuturePlans),
            "AGO" => Some(Text::IntervieweeConversationInformation),
            "AGP" => Some(Text::IntangibleAsset),
            "AGQ" => Some(Text::Inventory),
            "AGR" => Some(Text::Investment),
            "AGS" => Some(Text::IntercompanyRelationsInformation),
            "AGT" => Some(Text::JointVenture),
            "AGU" => Some(Text::Loan),
            "AGV" => Some(Text::LongTermDebt),
            "AGW" => Some(Text::Location),
            "AGX" => Some(Text::CurrentLegalStructure),
            "AGY" => Some(Text::MaritalContract),
            "AGZ" => Some(Text::MarketingActivities),
            "AHA" => Some(Text::Merger),
            "AHB" => Some(Text::MarketableSecurities),
            "AHC" => Some(Text::BusinessDebt),
            "AHD" => Some(Text::OriginalLegalStructure),
            "AHE" => Some(Text::EmployeeSharingArrangements),
            "AHF" => Some(Text::OrganizationDetails),
            "AHG" => Some(Text::PublicRecordDetails),
            "AHH" => Some(Text::PriceRange),
            "AHI" => Some(Text::Qualifications),
            "AHJ" => Some(Text::RegisteredActivity),
            "AHK" => Some(Text::CriminalSentence),
            "AHL" => Some(Text::SalesMethod),
            "AHM" => Some(Text::EducationalInstitutionInformation),
            "AHN" => Some(Text::StatusDetails),
            "AHO" => Some(Text::Sales),
            "AHP" => Some(Text::SpouseInformation),
            "AHQ" => Some(Text::EducationalDegreeInformation),
            "AHR" => Some(Text::ShareholdingInformation),
            "AHS" => Some(Text::SalesTerritory),
            "AHT" => Some(Text::AccountantsComments),
            "AHU" => Some(Text::ExemptionLawLocation),
            "AHV" => Some(Text::ShareClassifications),
            "AHW" => Some(Text::Forecast),
            "AHX" => Some(Text::EventLocation),
            "AHY" => Some(Text::FacilityOccupancy),
            "AHZ" => Some(Text::ImportAndExportDetails),
            "AIA" => Some(Text::AdditionalFacilityInformation),
            "AIB" => Some(Text::InventoryValue),
            "AIC" => Some(Text::Education),
            "AID" => Some(Text::Event),
            "AIE" => Some(Text::Agent),
            "AIF" => Some(Text::DomesticallyAgreedFinancialStatementDetails),
            "AIG" => Some(Text::OtherCurrentAssetDescription),
            "AIH" => Some(Text::OtherCurrentLiabilityDescription),
            "AII" => Some(Text::FormerBusinessActivity),
            "AIJ" => Some(Text::TradeNameUse),
            "AIK" => Some(Text::SigningAuthority),
            "AIL" => Some(Text::Guarantee),
            "AIM" => Some(Text::HoldingCompanyOperation),
            "AIN" => Some(Text::ConsignmentRouting),
            "AIO" => Some(Text::LetterProtest),
            "AIP" => Some(Text::Question),
            "AIQ" => Some(Text::PartyInformation),
            "AIR" => Some(Text::AreaBoundariesDescription),
            "AIS" => Some(Text::AdvertisementInformation),
            "AIT" => Some(Text::FinancialStatementDetails),
            "AIU" => Some(Text::AccessInstructions),
            "AIV" => Some(Text::Liquidity),
            "AIW" => Some(Text::CreditLine),
            "AIX" => Some(Text::WarrantyTerms),
            "AIY" => Some(Text::DivisionDescription),
            "AIZ" => Some(Text::ReportingInstruction),
            "AJA" => Some(Text::ExaminationResult),
            "AJB" => Some(Text::LaboratoryResult),
            "ALC" => Some(Text::AllowanceChargeInformation),
            "ALD" => Some(Text::XRayResult),
            "ALE" => Some(Text::PathologyResult),
            "ALF" => Some(Text::InterventionDescription),
            "ALG" => Some(Text::SummaryAdmittance),
            "ALH" => Some(Text::MedicalTreatmentCourseDetail),
            "ALI" => Some(Text::Prognosis),
            "ALJ" => Some(Text::InstructionToPatient),
            "ALK" => Some(Text::InstructionToPhysician),
            "ALL" => Some(Text::AllDocuments),
            "ALM" => Some(Text::MedicineTreatment),
            "ALN" => Some(Text::MedicineDosageAndAdministration),
            "ALO" => Some(Text::AvailabilityPatient),
            "ALP" => Some(Text::ReasonForServiceRequest),
            "ALQ" => Some(Text::PurposeService),
            "ARR" => Some(Text::ArrivalConditions),
            "ARS" => Some(Text::ServiceRequestersComment),
            "AUT" => Some(Text::Authentication),
            "AUU" => Some(Text::RequestedLocationDescription),
            "AUV" => Some(Text::MedicineAdministrationCondition),
            "AUW" => Some(Text::PatientInformation),
            "AUX" => Some(Text::PrecautionaryMeasure),
            "AUY" => Some(Text::ServiceCharacteristic),
            "AUZ" => Some(Text::PlannedEventComment),
            "AVA" => Some(Text::ExpectedDelayComment),
            "AVB" => Some(Text::TransportRequirementsComment),
            "AVC" => Some(Text::TemporaryApprovalCondition),
            "AVD" => Some(Text::CustomsValuationInformation),
            "AVE" => Some(Text::ValueAddedTaxVatMarginScheme),
            "AVF" => Some(Text::MaritimeDeclarationHealth),
            "BAG" => Some(Text::PassengerBaggageInformation),
            "BAH" => Some(Text::MaritimeDeclarationHealth_Dup),
            "BAI" => Some(Text::AdditionalProductInformationAddress),
            "BAJ" => Some(Text::InformationToBePrintedOnDespatchAdvice),
            "BAK" => Some(Text::MissingGoodsRemarks),
            "BAL" => Some(Text::NonAcceptanceInformation),
            "BAM" => Some(Text::ReturnsInformation),
            "BAN" => Some(Text::SubLineItemInformation),
            "BAO" => Some(Text::TestInformation),
            "BAP" => Some(Text::ExternalLink),
            "BAQ" => Some(Text::VatExemptionReason),
            "BAR" => Some(Text::ProcessingInstructions),
            "BAS" => Some(Text::RelayInstructions),
            "BAT" => Some(Text::SimaApplicable),
            "BAU" => Some(Text::AppealsProgramCode),
            "BAV" => Some(Text::SimaSubject),
            "BAW" => Some(Text::SurtaxApplicable),
            "BAX" => Some(Text::SimaSecurityBond),
            "BAY" => Some(Text::SurtaxSubject),
            "BAZ" => Some(Text::SafeguardApplicable),
            "BBA" => Some(Text::SafeguardApplicable_Dup),
            "BBB" => Some(Text::SafeguardSubject),
            "BLC" => Some(Text::TransportContractDocumentClause),
            "BLD" => Some(Text::InstructionToPreparePatient),
            "BLE" => Some(Text::MedicineTreatmentComment),
            "BLF" => Some(Text::ExaminationResultComment),
            "BLG" => Some(Text::ServiceRequestComment),
            "BLH" => Some(Text::PrescriptionReason),
            "BLI" => Some(Text::PrescriptionComment),
            "BLJ" => Some(Text::ClinicalInvestigationComment),
            "BLK" => Some(Text::MedicinalSpecificationComment),
            "BLL" => Some(Text::EconomicContributionComment),
            "BLM" => Some(Text::StatusAPlan),
            "BLN" => Some(Text::RandomSampleTestInformation),
            "BLO" => Some(Text::PeriodTime),
            "BLP" => Some(Text::Legislation),
            "BLQ" => Some(Text::SecurityMeasuresRequested),
            "BLR" => Some(Text::TransportContractDocumentRemark),
            "BLS" => Some(Text::PreviousPortCallSecurityInformation),
            "BLT" => Some(Text::SecurityInformation),
            "BLU" => Some(Text::WasteInformation),
            "BLV" => Some(Text::B2cMarketingInformationShortDescription),
            "BLW" => Some(Text::B2bMarketingInformationLongDescription),
            "BLX" => Some(Text::B2cMarketingInformationLongDescription),
            "BLY" => Some(Text::ProductIngredients),
            "BLZ" => Some(Text::LocationShortName),
            "BMA" => Some(Text::PackagingMaterialInformation),
            "BMB" => Some(Text::FillerMaterialInformation),
            "BMC" => Some(Text::ShipToShipActivityInformation),
            "BMD" => Some(Text::PackageMaterialDescription),
            "BME" => Some(Text::ConsumerLevelPackageMarking),
            "BMF" => Some(Text::SimaMeasureInForce),
            "BMG" => Some(Text::PreCarm),
            "BMH" => Some(Text::SimaMeasureType),
            "CCI" => Some(Text::CustomsClearanceInstructions),
            "CCJ" => Some(Text::SubTypeCode),
            "CCK" => Some(Text::SimaInformation),
            "CCL" => Some(Text::TimeLimitEnd),
            "CCM" => Some(Text::TimeLimitStart),
            "CCN" => Some(Text::WarehouseTimeLimit),
            "CCO" => Some(Text::ValueForDutyInformation),
            "CEX" => Some(Text::CustomsClearanceInstructionsExport),
            "CHG" => Some(Text::ChangeInformation),
            "CIP" => Some(Text::CustomsClearanceInstructionImport),
            "CLP" => Some(Text::ClearancePlaceRequested),
            "CLR" => Some(Text::LoadingRemarks),
            "COI" => Some(Text::OrderInformation),
            "CUR" => Some(Text::CustomerRemarks),
            "CUS" => Some(Text::CustomsDeclarationInformation),
            "DAR" => Some(Text::DamageRemarks),
            "DCL" => Some(Text::DocumentIssuerDeclaration),
            "DEL" => Some(Text::DeliveryInformation),
            "DIN" => Some(Text::DeliveryInstructions),
            "DOC" => Some(Text::DocumentationInstructions),
            "DUT" => Some(Text::DutyDeclaration),
            "EUR" => Some(Text::EffectiveUsedRouting),
            "FBC" => Some(Text::FirstBlockToBePrintedOnTransportContract),
            "GBL" => Some(Text::GovernmentBillLadingInformation),
            "GEN" => Some(Text::EntireTransactionSet),
            "GS7" => Some(Text::FurtherInformationConcerningGgvsPar7),
            "HAN" => Some(Text::ConsignmentHandlingInstruction),
            "HAZ" => Some(Text::HazardInformation),
            "ICN" => Some(Text::ConsignmentInformationForConsignee),
            "IIN" => Some(Text::InsuranceInstructions),
            "IMI" => Some(Text::InvoiceMailingInstructions),
            "IND" => Some(Text::CommercialInvoiceItemDescription),
            "INS" => Some(Text::InsuranceInformation),
            "INV" => Some(Text::InvoiceInstruction),
            "IRP" => Some(Text::InformationForRailwayPurpose),
            "ITR" => Some(Text::InlandTransportDetails),
            "ITS" => Some(Text::TestingInstructions),
            "LAN" => Some(Text::LocationAlias),
            "LIN" => Some(Text::LineItem),
            "LOI" => Some(Text::LoadingInstruction),
            "MCO" => Some(Text::MiscellaneousChargeOrder),
            "MDH" => Some(Text::MaritimeDeclarationHealth_Dup_Dup),
            "MKS" => Some(Text::AdditionalMarksNumbersInformation),
            "ORI" => Some(Text::OrderInstruction),
            "OSI" => Some(Text::OtherServiceInformation),
            "PAC" => Some(Text::PackingMarkingInformation),
            "PAI" => Some(Text::PaymentInstructionsInformation),
            "PAY" => Some(Text::PayablesInformation),
            "PKG" => Some(Text::PackagingInformation),
            "PKT" => Some(Text::PackagingTermsInformation),
            "PMD" => Some(Text::PaymentDetailRemittanceInformation),
            "PMT" => Some(Text::PaymentInformation),
            "PRD" => Some(Text::ProductInformation),
            "PRF" => Some(Text::PriceCalculationFormula),
            "PRI" => Some(Text::PriorityInformation),
            "PUR" => Some(Text::PurchasingInformation),
            "QIN" => Some(Text::QuarantineInstructions),
            "QQD" => Some(Text::QualityDemandsRequirements),
            "QUT" => Some(Text::QuotationInstructionInformation),
            "RAH" => Some(Text::RiskAndHandlingInformation),
            "REG" => Some(Text::RegulatoryInformation),
            "RET" => Some(Text::ReturnToOriginInformation),
            "REV" => Some(Text::Receivables),
            "RQR" => Some(Text::ConsignmentRoute),
            "SAF" => Some(Text::SafetyInformation),
            "SIC" => Some(Text::ConsignmentDocumentaryInstruction),
            "SIN" => Some(Text::SpecialInstructions),
            "SLR" => Some(Text::ShipLineRequested),
            "SPA" => Some(Text::SpecialPermissionForTransportGenerally),
            "SPG" => Some(Text::SpecialPermissionConcerningGoodsToBeTransported),
            "SPH" => Some(Text::SpecialHandling),
            "SPP" => Some(Text::SpecialPermissionConcerningPackage),
            "SPT" => Some(Text::SpecialPermissionConcerningTransportMeans),
            "SRN" => Some(Text::SubsidiaryRiskNumberIataDgr),
            "SSR" => Some(Text::SpecialServiceRequest),
            "SUR" => Some(Text::SupplierRemarks),
            "TCA" => Some(Text::ConsignmentTariff),
            "TDT" => Some(Text::ConsignmentTransport),
            "TRA" => Some(Text::TransportationInformation),
            "TRR" => Some(Text::RequestedTariff),
            "TXD" => Some(Text::TaxDeclaration),
            "WHI" => Some(Text::WarehouseInstructionInformation),
            "ZZZ" => Some(Text::MutuallyDefined),
            _ => None,
        }
    }
}
