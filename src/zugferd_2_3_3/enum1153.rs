#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Enum1153 {
    /// Order acknowledgement document identifier
    OrderAcknowledgementDocumentIdentifier,
    /// Proforma invoice document identifier
    ProformaInvoiceDocumentIdentifier,
    /// Documentary credit identifier
    DocumentaryCreditIdentifier,
    /// Contract document addendum identifier
    ContractDocumentAddendumIdentifier,
    /// Goods declaration number
    GoodsDeclarationNumber,
    /// Debit card number
    DebitCardNumber,
    /// Offer number
    OfferNumber,
    /// Bank's batch interbank transaction reference number
    BanksBatchInterbankTransactionReferenceNumber,
    /// Bank's individual interbank transaction reference number
    BanksIndividualInterbankTransactionReferenceNumber,
    /// Delivery order number
    DeliveryOrderNumber,
    /// Despatch advice number
    DespatchAdviceNumber,
    /// Drawing number
    DrawingNumber,
    /// Waybill number
    WaybillNumber,
    /// Delivery schedule number
    DeliveryScheduleNumber,
    /// Consignment identifier, consignee assigned
    ConsignmentIdentifierConsigneeAssigned,
    /// Partial shipment identifier
    PartialShipmentIdentifier,
    /// Transport equipment identifier
    TransportEquipmentIdentifier,
    /// Municipality assigned business registry number
    MunicipalityAssignedBusinessRegistryNumber,
    /// Transport contract document identifier
    TransportContractDocumentIdentifier,
    /// Master label number
    MasterLabelNumber,
    /// Despatch note document identifier
    DespatchNoteDocumentIdentifier,
    /// Enquiry number
    EnquiryNumber,
    /// Docket number
    DocketNumber,
    /// Civil action number
    CivilActionNumber,
    /// Carrier's agent reference number
    CarriersAgentReferenceNumber,
    /// Standard Carrier Alpha Code (SCAC) number
    StandardCarrierAlphaCodeScacNumber,
    /// Customs valuation decision number
    CustomsValuationDecisionNumber,
    /// End use authorization number
    EndUseAuthorizationNumber,
    /// Anti-dumping case number
    AntiDumpingCaseNumber,
    /// Customs tariff number
    CustomsTariffNumber,
    /// Declarant's reference number
    DeclarantsReferenceNumber,
    /// Repair estimate number
    RepairEstimateNumber,
    /// Customs decision request number
    CustomsDecisionRequestNumber,
    /// Sub-house bill of lading number
    SubHouseBillLadingNumber,
    /// Tax payment identifier
    TaxPaymentIdentifier,
    /// Quota number
    QuotaNumber,
    /// Transit (onward carriage) guarantee (bond) number
    TransitOnwardCarriageGuaranteeBondNumber,
    /// Customs guarantee number
    CustomsGuaranteeNumber,
    /// Replacing part number
    ReplacingPartNumber,
    /// Seller's catalogue number
    SellersCatalogueNumber,
    /// Originator's reference
    OriginatorsReference,
    /// Declarant's Customs identity number
    DeclarantsCustomsIdentityNumber,
    /// Importer reference number
    ImporterReferenceNumber,
    /// Export clearance instruction reference number
    ExportClearanceInstructionReferenceNumber,
    /// Import clearance instruction reference number
    ImportClearanceInstructionReferenceNumber,
    /// Goods declaration document identifier, Customs
    GoodsDeclarationDocumentIdentifierCustoms,
    /// Article number
    ArticleNumber,
    /// Intra-plant routing
    IntraPlantRouting,
    /// Stock keeping unit number
    StockKeepingUnitNumber,
    /// Text Element Identifier deletion reference
    TextElementIdentifierDeletionReference,
    /// Allotment identification (Air)
    AllotmentIdentificationAir,
    /// Vehicle licence number
    VehicleLicenceNumber,
    ///  Air cargo transfer manifest
    AirCargoTransferManifest,
    /// Cargo acceptance order reference number
    CargoAcceptanceOrderReferenceNumber,
    /// US government agency number
    UsGovernmentAgencyNumber,
    /// Shipping unit identification
    ShippingUnitIdentification,
    /// Additional reference number
    AdditionalReferenceNumber,
    /// Related document number
    RelatedDocumentNumber,
    /// Addressee reference
    AddresseeReference,
    /// ATA carnet number
    AtaCarnetNumber,
    /// Packaging unit identification
    PackagingUnitIdentification,
    /// Outerpackaging unit identification
    OuterpackagingUnitIdentification,
    /// Customer material specification number
    CustomerMaterialSpecificationNumber,
    /// Bank reference
    BankReference,
    /// Principal reference number
    PrincipalReferenceNumber,
    /// Collection advice document identifier
    CollectionAdviceDocumentIdentifier,
    /// Iron charge number
    IronChargeNumber,
    /// Hot roll number
    HotRollNumber,
    /// Cold roll number
    ColdRollNumber,
    /// Railway wagon number
    RailwayWagonNumber,
    /// Unique claims reference number of the sender
    UniqueClaimsReferenceNumberSender,
    /// Loss/event number
    LossEventNumber,
    /// Estimate order reference number
    EstimateOrderReferenceNumber,
    /// Reference number to previous message
    ReferenceNumberToPreviousMessage,
    /// Banker's acceptance
    BankersAcceptance,
    /// Duty memo number
    DutyMemoNumber,
    /// Equipment transport charge number
    EquipmentTransportChargeNumber,
    /// Buyer's item number
    BuyersItemNumber,
    /// Matured certificate of deposit
    MaturedCertificateDeposit,
    /// Loan
    Loan,
    /// Analysis number/test number
    AnalysisNumberTestNumber,
    /// Account number
    AccountNumber,
    /// Treaty number
    TreatyNumber,
    /// Catastrophe number
    CatastropheNumber,
    /// Bureau signing (statement reference)
    BureauSigningStatementReference,
    /// Company / syndicate reference 1
    CompanySyndicateReference1,
    /// Company / syndicate reference 2
    CompanySyndicateReference2,
    /// Ordering customer consignment reference number
    OrderingCustomerConsignmentReferenceNumber,
    /// Shipowner's authorization number
    ShipownersAuthorizationNumber,
    /// Inland transport order number
    InlandTransportOrderNumber,
    /// Container work order reference number
    ContainerWorkOrderReferenceNumber,
    /// Statement number
    StatementNumber,
    /// Unique market reference
    UniqueMarketReference,
    /// Group accounting
    GroupAccounting,
    /// Broker reference 1
    BrokerReference1,
    /// Broker reference 2
    BrokerReference2,
    /// Lloyd's claims office reference
    LloydsClaimsOfficeReference,
    /// Secure delivery terms and conditions agreement reference
    SecureDeliveryTermsAndConditionsAgreementReference,
    /// Report number
    ReportNumber,
    /// Trader account number
    TraderAccountNumber,
    ///  Authorization for expense (AFE) number
    AuthorizationForExpenseAfeNumber,
    /// Government agency reference number
    GovernmentAgencyReferenceNumber,
    /// Assembly number
    AssemblyNumber,
    /// Symbol number
    SymbolNumber,
    /// Commodity number
    CommodityNumber,
    /// Eur 1 certificate number
    Eur1CertificateNumber,
    /// Customer process specification number
    CustomerProcessSpecificationNumber,
    /// Customer specification number
    CustomerSpecificationNumber,
    /// Applicable instructions or standards
    ApplicableInstructionsOrStandards,
    /// Registration number of previous Customs declaration
    RegistrationNumberPreviousCustomsDeclaration,
    /// Post-entry reference
    PostEntryReference,
    /// Payment order number
    PaymentOrderNumber,
    /// Delivery number (transport)
    DeliveryNumberTransport,
    /// Transport route
    TransportRoute,
    /// Customer's unit inventory number
    CustomersUnitInventoryNumber,
    /// Product reservation number
    ProductReservationNumber,
    /// Project number
    ProjectNumber,
    /// Drawing list number
    DrawingListNumber,
    /// Project specification number
    ProjectSpecificationNumber,
    /// Primary reference
    PrimaryReference,
    /// Request for cancellation number
    RequestForCancellationNumber,
    /// Supplier's control number
    SuppliersControlNumber,
    /// Shipping note number
    ShippingNoteNumber,
    /// Empty container bill number
    EmptyContainerBillNumber,
    /// Non-negotiable maritime transport document number
    NonNegotiableMaritimeTransportDocumentNumber,
    /// Substitute air waybill number
    SubstituteAirWaybillNumber,
    /// Despatch note (post parcels) number
    DespatchNotePostParcelsNumber,
    ///  Airlines flight identification number
    AirlinesFlightIdentificationNumber,
    /// Through bill of lading number
    ThroughBillLadingNumber,
    /// Cargo manifest number
    CargoManifestNumber,
    /// Bordereau number
    BordereauNumber,
    /// Customs item number
    CustomsItemNumber,
    /// Export Control Commodity number (ECCN)
    ExportControlCommodityNumberEccn,
    /// Marking/label reference
    MarkingLabelReference,
    /// Tariff number
    TariffNumber,
    /// Replenishment purchase order number
    ReplenishmentPurchaseOrderNumber,
    /// Immediate transportation no. for in bond movement
    ImmediateTransportationNoForInBondMovement,
    /// Transportation exportation no. for in bond movement
    TransportationExportationNoForInBondMovement,
    /// Immediate exportation no. for in bond movement
    ImmediateExportationNoForInBondMovement,
    /// Associated invoices
    AssociatedInvoices,
    /// Secondary Customs reference
    SecondaryCustomsReference,
    /// Account party's reference
    AccountPartysReference,
    /// Beneficiary's reference
    BeneficiarysReference,
    /// Second beneficiary's reference
    SecondBeneficiarysReference,
    /// Applicant's bank reference
    ApplicantsBankReference,
    /// Issuing bank's reference
    IssuingBanksReference,
    /// Beneficiary's bank reference
    BeneficiarysBankReference,
    /// Direct payment valuation number
    DirectPaymentValuationNumber,
    /// Direct payment valuation request number
    DirectPaymentValuationRequestNumber,
    /// Quantity valuation number
    QuantityValuationNumber,
    /// Quantity valuation request number
    QuantityValuationRequestNumber,
    /// Bill of quantities number
    BillQuantitiesNumber,
    /// Payment valuation number
    PaymentValuationNumber,
    /// Situation number
    SituationNumber,
    /// Agreement to pay number
    AgreementToPayNumber,
    /// Contract party reference number
    ContractPartyReferenceNumber,
    /// Account party's bank reference
    AccountPartysBankReference,
    /// Agent's bank reference
    AgentsBankReference,
    /// Agent's reference
    AgentsReference,
    /// Applicant's reference
    ApplicantsReference,
    /// Dispute number
    DisputeNumber,
    /// Credit rating agency's reference number
    CreditRatingAgencysReferenceNumber,
    /// Request number
    RequestNumber,
    /// Single transaction sequence number
    SingleTransactionSequenceNumber,
    /// Application reference number
    ApplicationReferenceNumber,
    /// Delivery verification certificate
    DeliveryVerificationCertificate,
    /// Number of temporary importation document
    NumberTemporaryImportationDocument,
    /// Reference number quoted on statement
    ReferenceNumberQuotedOnStatement,
    /// Sender's reference to the original message
    SendersReferenceToOriginalMessage,
    /// Company issued equipment ID
    CompanyIssuedEquipmentId,
    /// Domestic flight number
    DomesticFlightNumber,
    /// International flight number
    InternationalFlightNumber,
    /// Employer identification number of service bureau
    EmployerIdentificationNumberServiceBureau,
    /// Service group identification number
    ServiceGroupIdentificationNumber,
    /// Member number
    MemberNumber,
    /// Previous member number
    PreviousMemberNumber,
    /// Scheme/plan number
    SchemePlanNumber,
    /// Previous scheme/plan number
    PreviousSchemePlanNumber,
    /// Receiving party's member identification
    ReceivingPartysMemberIdentification,
    /// Payroll number
    PayrollNumber,
    /// Packaging specification number
    PackagingSpecificationNumber,
    /// Authority issued equipment identification
    AuthorityIssuedEquipmentIdentification,
    /// Training flight number
    TrainingFlightNumber,
    /// Fund code number
    FundCodeNumber,
    /// Signal code number
    SignalCodeNumber,
    /// Major force program number
    MajorForceProgramNumber,
    /// Nomination number
    NominationNumber,
    /// Laboratory registration number
    LaboratoryRegistrationNumber,
    /// Transport contract reference number
    TransportContractReferenceNumber,
    /// Payee's reference number
    PayeesReferenceNumber,
    /// Payer's reference number
    PayersReferenceNumber,
    /// Creditor's reference number
    CreditorsReferenceNumber,
    /// Debtor's reference number
    DebtorsReferenceNumber,
    /// Joint venture reference number
    JointVentureReferenceNumber,
    /// Chamber of Commerce registration number
    ChamberCommerceRegistrationNumber,
    /// Tax registration number
    TaxRegistrationNumber,
    /// Wool identification number
    WoolIdentificationNumber,
    /// Wool tax reference number
    WoolTaxReferenceNumber,
    /// Meat processing establishment registration number
    MeatProcessingEstablishmentRegistrationNumber,
    /// Quarantine/treatment status reference number
    QuarantineTreatmentStatusReferenceNumber,
    /// Request for quote number
    RequestForQuoteNumber,
    /// Manual processing authority number
    ManualProcessingAuthorityNumber,
    /// Rate note number
    RateNoteNumber,
    /// Freight Forwarder number
    FreightForwarderNumber,
    /// Customs release code
    CustomsReleaseCode,
    /// Compliance code number
    ComplianceCodeNumber,
    /// Department of transportation bond number
    DepartmentTransportationBondNumber,
    /// Export establishment number
    ExportEstablishmentNumber,
    /// Certificate of conformity
    CertificateConformity,
    /// Ministerial certificate of homologation
    MinisterialCertificateHomologation,
    /// Previous delivery instruction number
    PreviousDeliveryInstructionNumber,
    /// Passport number
    PassportNumber,
    /// Common transaction reference number
    CommonTransactionReferenceNumber,
    /// Bank's common transaction reference number
    BanksCommonTransactionReferenceNumber,
    /// Customer's individual transaction reference number
    CustomersIndividualTransactionReferenceNumber,
    /// Bank's individual transaction reference number
    BanksIndividualTransactionReferenceNumber,
    /// Customer's common transaction reference number
    CustomersCommonTransactionReferenceNumber,
    /// Individual transaction reference number
    IndividualTransactionReferenceNumber,
    /// Product sourcing agreement number
    ProductSourcingAgreementNumber,
    /// Customs transhipment number
    CustomsTranshipmentNumber,
    /// Customs preference inquiry number
    CustomsPreferenceInquiryNumber,
    /// Packing plant number
    PackingPlantNumber,
    /// Original certificate number
    OriginalCertificateNumber,
    /// Processing plant number
    ProcessingPlantNumber,
    /// Slaughter plant number
    SlaughterPlantNumber,
    /// Charge card account number
    ChargeCardAccountNumber,
    /// Event reference number
    EventReferenceNumber,
    /// Transport section reference number
    TransportSectionReferenceNumber,
    /// Referred product for mechanical analysis
    ReferredProductForMechanicalAnalysis,
    /// Referred product for chemical analysis
    ReferredProductForChemicalAnalysis,
    /// Consolidated invoice number
    ConsolidatedInvoiceNumber,
    /// Part reference indicator in a drawing
    PartReferenceIndicatorInADrawing,
    /// U.S. Code of Federal Regulations (CFR)
    USCodeFederalRegulationsCfr,
    /// Purchasing activity clause number
    PurchasingActivityClauseNumber,
    /// U.S. Defense Federal Acquisition Regulation Supplement
    USDefenseFederalAcquisitionRegulationSupplement,
    /// Agency clause number
    AgencyClauseNumber,
    /// Circular publication number
    CircularPublicationNumber,
    /// U.S. Federal Acquisition Regulation
    USFederalAcquisitionRegulation,
    /// U.S. General Services Administration Regulation
    USGeneralServicesAdministrationRegulation,
    /// U.S. Federal Information Resources Management Regulation
    USFederalInformationResourcesManagementRegulation,
    /// Paragraph
    Paragraph,
    /// Special instructions number
    SpecialInstructionsNumber,
    /// Site specific procedures, terms, and conditions number
    SiteSpecificProceduresTermsAndConditionsNumber,
    /// Master solicitation procedures, terms, and conditions
    MasterSolicitationProceduresTermsAndConditions,
    /// U.S. Department of Veterans Affairs Acquisition Regulation
    USDepartmentVeteransAffairsAcquisitionRegulation,
    /// Military Interdepartmental Purchase Request (MIPR) number
    MilitaryInterdepartmentalPurchaseRequestMiprNumber,
    /// Foreign military sales number
    ForeignMilitarySalesNumber,
    /// Defense priorities allocation system priority rating
    DefensePrioritiesAllocationSystemPriorityRating,
    /// Wage determination number
    WageDeterminationNumber,
    /// Agreement number
    AgreementNumber,
    /// Standard Industry Classification (SIC) number
    StandardIndustryClassificationSicNumber,
    /// End item number
    EndItemNumber,
    /// Federal supply schedule item number
    FederalSupplyScheduleItemNumber,
    /// Technical document number
    TechnicalDocumentNumber,
    /// Technical order number
    TechnicalOrderNumber,
    /// Suffix
    Suffix,
    /// Transportation account number
    TransportationAccountNumber,
    /// Container disposition order reference number
    ContainerDispositionOrderReferenceNumber,
    /// Container prefix
    ContainerPrefix,
    /// Transport equipment return reference
    TransportEquipmentReturnReference,
    /// Transport equipment survey reference
    TransportEquipmentSurveyReference,
    /// Transport equipment survey report number
    TransportEquipmentSurveyReportNumber,
    /// Transport equipment stuffing order
    TransportEquipmentStuffingOrder,
    /// Vehicle Identification Number (VIN)
    VehicleIdentificationNumberVin,
    /// Government bill of lading
    GovernmentBillLading,
    /// Ordering customer's second reference number
    OrderingCustomersSecondReferenceNumber,
    /// Direct debit reference
    DirectDebitReference,
    /// Meter reading at the beginning of the delivery
    MeterReadingAtBeginningDelivery,
    /// Meter reading at the end of delivery
    MeterReadingAtEndDelivery,
    /// Replenishment purchase order range start number
    ReplenishmentPurchaseOrderRangeStartNumber,
    /// Third bank's reference
    ThirdBanksReference,
    /// Action authorization number
    ActionAuthorizationNumber,
    /// Appropriation number
    AppropriationNumber,
    /// Product change authority number
    ProductChangeAuthorityNumber,
    /// General cargo consignment reference number
    GeneralCargoConsignmentReferenceNumber,
    /// Catalogue sequence number
    CatalogueSequenceNumber,
    /// Forwarding order number
    ForwardingOrderNumber,
    /// Transport equipment survey reference number
    TransportEquipmentSurveyReferenceNumber,
    /// Lease contract reference
    LeaseContractReference,
    /// Transport costs reference number
    TransportCostsReferenceNumber,
    /// Transport equipment stripping order
    TransportEquipmentStrippingOrder,
    /// Prior policy number
    PriorPolicyNumber,
    /// Policy number
    PolicyNumber,
    /// Procurement budget number
    ProcurementBudgetNumber,
    /// Domestic inventory management code
    DomesticInventoryManagementCode,
    /// Customer reference number assigned to previous balance of
    CustomerReferenceNumberAssignedToPreviousBalance,
    /// Previous credit advice reference number
    PreviousCreditAdviceReferenceNumber,
    /// Reporting form number
    ReportingFormNumber,
    /// Authorization number for exception to dangerous goods
    AuthorizationNumberForExceptionToDangerousGoods,
    /// Dangerous goods security number
    DangerousGoodsSecurityNumber,
    /// Dangerous goods transport licence number
    DangerousGoodsTransportLicenceNumber,
    /// Previous rental agreement number
    PreviousRentalAgreementNumber,
    /// Next rental agreement reason number
    NextRentalAgreementReasonNumber,
    /// Consignee's invoice number
    ConsigneesInvoiceNumber,
    /// Message batch number
    MessageBatchNumber,
    /// Previous delivery schedule number
    PreviousDeliveryScheduleNumber,
    /// Physical inventory recount reference number
    PhysicalInventoryRecountReferenceNumber,
    /// Receiving advice number
    ReceivingAdviceNumber,
    /// Returnable container reference number
    ReturnableContainerReferenceNumber,
    /// Returns notice number
    ReturnsNoticeNumber,
    /// Sales forecast number
    SalesForecastNumber,
    /// Sales report number
    SalesReportNumber,
    /// Previous tax control number
    PreviousTaxControlNumber,
    /// AGERD (Aerospace Ground Equipment Requirement Data) number
    AgerdAerospaceGroundEquipmentRequirementDataNumber,
    /// Registered capital reference
    RegisteredCapitalReference,
    /// Standard number of inspection document
    StandardNumberInspectionDocument,
    /// Model
    Model,
    /// Financial management reference
    FinancialManagementReference,
    /// NOTIfication for COLlection number (NOTICOL)
    NotificationForCollectionNumberNoticol,
    /// Previous request for metered reading reference number
    PreviousRequestForMeteredReadingReferenceNumber,
    /// Next rental agreement number
    NextRentalAgreementNumber,
    /// Reference number of a request for metered reading
    ReferenceNumberARequestForMeteredReading,
    /// Hastening number
    HasteningNumber,
    /// Repair data request number
    RepairDataRequestNumber,
    /// Consumption data request number
    ConsumptionDataRequestNumber,
    /// Profile number
    ProfileNumber,
    /// Case number
    CaseNumber,
    /// Government quality assurance and control level Number
    GovernmentQualityAssuranceAndControlLevelNumber,
    /// Payment plan reference
    PaymentPlanReference,
    /// Replaced meter unit number
    ReplacedMeterUnitNumber,
    /// Replenishment purchase order range end number
    ReplenishmentPurchaseOrderRangeEndNumber,
    /// Insurer assigned reference number
    InsurerAssignedReferenceNumber,
    /// Canadian excise entry number
    CanadianExciseEntryNumber,
    /// Premium rate table
    PremiumRateTable,
    /// Advise through bank's reference
    AdviseThroughBanksReference,
    /// US, Department of Transportation bond surety code
    UsDepartmentTransportationBondSuretyCode,
    /// US, Food and Drug Administration establishment indicator
    UsFoodAndDrugAdministrationEstablishmentIndicator,
    /// US, Federal Communications Commission (FCC) import
    UsFederalCommunicationsCommissionFccImport,
    /// Goods and Services Tax identification number
    GoodsAndServicesTaxIdentificationNumber,
    /// Integrated logistic support cross reference number
    IntegratedLogisticSupportCrossReferenceNumber,
    /// Department number
    DepartmentNumber,
    /// Buyer's catalogue number
    BuyersCatalogueNumber,
    /// Financial settlement party's reference number
    FinancialSettlementPartysReferenceNumber,
    /// Standard's version number
    StandardsVersionNumber,
    /// Pipeline number
    PipelineNumber,
    /// Account servicing bank's reference number
    AccountServicingBanksReferenceNumber,
    /// Completed units payment request reference
    CompletedUnitsPaymentRequestReference,
    /// Payment in advance request reference
    PaymentInAdvanceRequestReference,
    /// Parent file
    ParentFile,
    /// Sub file
    SubFile,
    /// CAD file layer convention
    CadFileLayerConvention,
    /// Technical regulation
    TechnicalRegulation,
    /// Plot file
    PlotFile,
    /// File conversion journal
    FileConversionJournal,
    /// Authorization number
    AuthorizationNumber,
    /// Reference number assigned by third party
    ReferenceNumberAssignedByThirdParty,
    /// Deposit reference number
    DepositReferenceNumber,
    /// Named bank's reference
    NamedBanksReference,
    /// Drawee's reference
    DraweesReference,
    /// Case of need party's reference
    CaseNeedPartysReference,
    /// Collecting bank's reference
    CollectingBanksReference,
    /// Remitting bank's reference
    RemittingBanksReference,
    /// Principal's bank reference
    PrincipalsBankReference,
    /// Presenting bank's reference
    PresentingBanksReference,
    /// Consignee's reference
    ConsigneesReference,
    /// Financial transaction reference number
    FinancialTransactionReferenceNumber,
    /// Credit reference number
    CreditReferenceNumber,
    /// Receiving bank's authorization number
    ReceivingBanksAuthorizationNumber,
    /// Clearing reference
    ClearingReference,
    /// Sending bank's reference number
    SendingBanksReferenceNumber,
    /// Documentary payment reference
    DocumentaryPaymentReference,
    /// Accounting file reference
    AccountingFileReference,
    /// Sender's file reference number
    SendersFileReferenceNumber,
    /// Receiver's file reference number
    ReceiversFileReferenceNumber,
    /// Source document internal reference
    SourceDocumentInternalReference,
    /// Principal's reference
    PrincipalsReference,
    /// Debit reference number
    DebitReferenceNumber,
    /// Calendar
    Calendar,
    /// Work shift
    WorkShift,
    /// Work breakdown structure
    WorkBreakdownStructure,
    /// Organisation breakdown structure
    OrganisationBreakdownStructure,
    /// Work task charge number
    WorkTaskChargeNumber,
    /// Functional work group
    FunctionalWorkGroup,
    /// Work team
    WorkTeam,
    /// Department
    Department,
    /// Statement of work
    StatementWork,
    /// Work package
    WorkPackage,
    /// Planning package
    PlanningPackage,
    /// Cost account
    CostAccount,
    /// Work order
    WorkOrder,
    /// Transportation Control Number (TCN)
    TransportationControlNumberTcn,
    /// Constraint notation
    ConstraintNotation,
    /// ETERMS reference
    EtermsReference,
    /// Implementation version number
    ImplementationVersionNumber,
    ///  Accounts receivable number
    AccountsReceivableNumber,
    /// Incorporated legal reference
    IncorporatedLegalReference,
    /// Payment instalment reference number
    PaymentInstalmentReferenceNumber,
    /// Equipment owner reference number
    EquipmentOwnerReferenceNumber,
    /// Cedent's claim number
    CedentsClaimNumber,
    /// Reinsurer's claim number
    ReinsurersClaimNumber,
    /// Price/sales catalogue response reference number
    PriceSalesCatalogueResponseReferenceNumber,
    /// General purpose message reference number
    GeneralPurposeMessageReferenceNumber,
    /// Invoicing data sheet reference number
    InvoicingDataSheetReferenceNumber,
    /// Inventory report reference number
    InventoryReportReferenceNumber,
    /// Ceiling formula reference number
    CeilingFormulaReferenceNumber,
    /// Price variation formula reference number
    PriceVariationFormulaReferenceNumber,
    /// Reference to account servicing bank's message
    ReferenceToAccountServicingBanksMessage,
    /// Party sequence number
    PartySequenceNumber,
    /// Purchaser's request reference
    PurchasersRequestReference,
    /// Contractor request reference
    ContractorRequestReference,
    /// Accident reference number
    AccidentReferenceNumber,
    /// Commercial account summary reference number
    CommercialAccountSummaryReferenceNumber,
    /// Contract breakdown reference
    ContractBreakdownReference,
    /// Contractor registration number
    ContractorRegistrationNumber,
    /// Applicable coefficient identification number
    ApplicableCoefficientIdentificationNumber,
    /// Special budget account number
    SpecialBudgetAccountNumber,
    /// Authorisation for repair reference
    AuthorisationForRepairReference,
    /// Manufacturer defined repair rates reference
    ManufacturerDefinedRepairRatesReference,
    /// Original submitter log number
    OriginalSubmitterLogNumber,
    /// Original submitter, parent Data Maintenance Request (DMR)
    OriginalSubmitterParentDataMaintenanceRequestDmr,
    /// Original submitter, child Data Maintenance Request (DMR)
    OriginalSubmitterChildDataMaintenanceRequestDmr,
    /// Entry point assessment log number
    EntryPointAssessmentLogNumber,
    /// Entry point assessment log number, parent DMR
    EntryPointAssessmentLogNumberParentDmr,
    /// Entry point assessment log number, child DMR
    EntryPointAssessmentLogNumberChildDmr,
    /// Data structure tag
    DataStructureTag,
    /// Central secretariat log number
    CentralSecretariatLogNumber,
    /// Central secretariat log number, parent Data Maintenance
    CentralSecretariatLogNumberParentDataMaintenance,
    /// Central secretariat log number, child Data Maintenance
    CentralSecretariatLogNumberChildDataMaintenance,
    /// International assessment log number
    InternationalAssessmentLogNumber,
    /// International assessment log number, parent Data
    InternationalAssessmentLogNumberParentData,
    /// International assessment log number, child Data Maintenance
    InternationalAssessmentLogNumberChildDataMaintenance,
    /// Status report number
    StatusReportNumber,
    /// Message design group number
    MessageDesignGroupNumber,
    /// US Customs Service (USCS) entry code
    UsCustomsServiceUscsEntryCode,
    /// Beginning job sequence number
    BeginningJobSequenceNumber,
    /// Sender's clause number
    SendersClauseNumber,
    /// Dun and Bradstreet Canada's 8 digit Standard Industrial
    DunAndBradstreetCanadas8DigitStandardIndustrial,
    /// Activite Principale Exercee (APE) identifier
    ActivitePrincipaleExerceeApeIdentifier,
    /// Dun and Bradstreet US 8 digit Standard Industrial
    DunAndBradstreetUs8DigitStandardIndustrial,
    /// Nomenclature Activity Classification Economy (NACE)
    NomenclatureActivityClassificationEconomyNace,
    /// Norme Activite Francaise (NAF) identifier
    NormeActiviteFrancaiseNafIdentifier,
    /// Registered contractor activity type
    RegisteredContractorActivityType,
    /// Statistic Bundes Amt (SBA) identifier
    StatisticBundesAmtSbaIdentifier,
    /// State or province assigned entity identification
    StateOrProvinceAssignedEntityIdentification,
    /// Institute of Security and Future Market Development (ISFMD)
    InstituteSecurityAndFutureMarketDevelopmentIsfmd,
    /// File identification number
    FileIdentificationNumber,
    /// Bankruptcy procedure number
    BankruptcyProcedureNumber,
    /// National government business identification number
    NationalGovernmentBusinessIdentificationNumber,
    /// Prior Data Universal Number System (DUNS) number
    PriorDataUniversalNumberSystemDunsNumber,
    /// Companies Registry Office (CRO) number
    CompaniesRegistryOfficeCroNumber,
    /// Costa Rican judicial number
    CostaRicanJudicialNumber,
    /// Numero de Identificacion Tributaria (NIT)
    NumeroDeIdentificacionTributariaNit,
    /// Patron number
    PatronNumber,
    /// Registro Informacion Fiscal (RIF) number
    RegistroInformacionFiscalRifNumber,
    /// Registro Unico de Contribuyente (RUC) number
    RegistroUnicoDeContribuyenteRucNumber,
    /// Tokyo SHOKO Research (TSR) business identifier
    TokyoShokoResearchTsrBusinessIdentifier,
    /// Personal identity card number
    PersonalIdentityCardNumber,
    /// Systeme Informatique pour le Repertoire des ENtreprises
    SystemeInformatiquePourLeRepertoireDesEntreprises,
    /// Systeme Informatique pour le Repertoire des ETablissements
    SystemeInformatiquePourLeRepertoireDesEtablissements,
    /// Publication issue number
    PublicationIssueNumber,
    /// Original filing number
    OriginalFilingNumber,
    /// Document page identifier
    DocumentPageIdentifier,
    /// Public filing registration number
    PublicFilingRegistrationNumber,
    /// Regiristo Federal de Contribuyentes
    RegiristoFederalDeContribuyentes,
    /// Social security number
    SocialSecurityNumber,
    /// Document volume number
    DocumentVolumeNumber,
    /// Book number
    BookNumber,
    /// Stock exchange company identifier
    StockExchangeCompanyIdentifier,
    /// Imputation account
    ImputationAccount,
    /// Financial phase reference
    FinancialPhaseReference,
    /// Technical phase reference
    TechnicalPhaseReference,
    /// Prior contractor registration number
    PriorContractorRegistrationNumber,
    /// Stock adjustment number
    StockAdjustmentNumber,
    /// Dispensation reference
    DispensationReference,
    /// Investment reference number
    InvestmentReferenceNumber,
    /// Assuming company
    AssumingCompany,
    /// Budget chapter
    BudgetChapter,
    /// Duty free products security number
    DutyFreeProductsSecurityNumber,
    /// Duty free products receipt authorisation number
    DutyFreeProductsReceiptAuthorisationNumber,
    /// Party information message reference
    PartyInformationMessageReference,
    /// Formal statement reference
    FormalStatementReference,
    /// Proof of delivery reference number
    ProofDeliveryReferenceNumber,
    /// Supplier's credit claim reference number
    SuppliersCreditClaimReferenceNumber,
    /// Picture of actual product
    PictureActualProduct,
    /// Picture of a generic product
    PictureAGenericProduct,
    /// Trading partner identification number
    TradingPartnerIdentificationNumber,
    /// Prior trading partner identification number
    PriorTradingPartnerIdentificationNumber,
    /// Password
    Password,
    /// Formal report number
    FormalReportNumber,
    /// Fund account number
    FundAccountNumber,
    /// Safe custody number
    SafeCustodyNumber,
    /// Master account number
    MasterAccountNumber,
    /// Group reference number
    GroupReferenceNumber,
    /// Accounting transmission number
    AccountingTransmissionNumber,
    /// Product data file number
    ProductDataFileNumber,
    /// Cadastro Geral do Contribuinte (CGC)
    CadastroGeralDoContribuinteCgc,
    /// Foreign resident identification number
    ForeignResidentIdentificationNumber,
    /// CD-ROM
    CdRom,
    /// Physical medium
    PhysicalMedium,
    /// Financial cancellation reference number
    FinancialCancellationReferenceNumber,
    /// Purchase for export Customs agreement number
    PurchaseForExportCustomsAgreementNumber,
    /// Judgment number
    JudgmentNumber,
    /// Secretariat number
    SecretariatNumber,
    /// Previous banking status message reference
    PreviousBankingStatusMessageReference,
    /// Last received banking status message reference
    LastReceivedBankingStatusMessageReference,
    /// Bank's documentary procedure reference
    BanksDocumentaryProcedureReference,
    /// Customer's documentary procedure reference
    CustomersDocumentaryProcedureReference,
    /// Safe deposit box number
    SafeDepositBoxNumber,
    /// Receiving Bankgiro number
    ReceivingBankgiroNumber,
    /// Sending Bankgiro number
    SendingBankgiroNumber,
    /// Bankgiro reference
    BankgiroReference,
    /// Guarantee number
    GuaranteeNumber,
    /// Collection instrument number
    CollectionInstrumentNumber,
    /// Converted Postgiro number
    ConvertedPostgiroNumber,
    /// Cost centre alignment number
    CostCentreAlignmentNumber,
    /// Kamer Van Koophandel (KVK) number
    KamerVanKoophandelKvkNumber,
    /// Institut Belgo-Luxembourgeois de Codification (IBLC) number
    InstitutBelgoLuxembourgeoisDeCodificationIblcNumber,
    /// External object reference
    ExternalObjectReference,
    /// Exceptional transport authorisation number
    ExceptionalTransportAuthorisationNumber,
    /// Clave Unica de Identificacion Tributaria (CUIT)
    ClaveUnicaDeIdentificacionTributariaCuit,
    /// Registro Unico Tributario (RUT)
    RegistroUnicoTributarioRut,
    /// Flat rack container bundle identification number
    FlatRackContainerBundleIdentificationNumber,
    /// Transport equipment acceptance order reference
    TransportEquipmentAcceptanceOrderReference,
    /// Transport equipment release order reference
    TransportEquipmentReleaseOrderReference,
    /// Ship's stay reference number
    ShipsStayReferenceNumber,
    ///  Authorization to meet competition number
    AuthorizationToMeetCompetitionNumber,
    /// Place of positioning reference
    PlacePositioningReference,
    /// Party reference
    PartyReference,
    /// Issued prescription identification
    IssuedPrescriptionIdentification,
    /// Collection reference
    CollectionReference,
    /// Travel service
    TravelService,
    /// Consignment stock contract
    ConsignmentStockContract,
    /// Importer's letter of credit reference
    ImportersLetterCreditReference,
    /// Performed prescription identification
    PerformedPrescriptionIdentification,
    /// Image reference
    ImageReference,
    /// Proposed purchase order reference number
    ProposedPurchaseOrderReferenceNumber,
    /// Application for financial support reference number
    ApplicationForFinancialSupportReferenceNumber,
    /// Manufacturing quality agreement number
    ManufacturingQualityAgreementNumber,
    /// Software editor reference
    SoftwareEditorReference,
    /// Software reference
    SoftwareReference,
    /// Software quality reference
    SoftwareQualityReference,
    /// Consolidated orders' reference
    ConsolidatedOrdersReference,
    /// Customs binding ruling number
    CustomsBindingRulingNumber,
    /// Customs non-binding ruling number
    CustomsNonBindingRulingNumber,
    /// Delivery route reference
    DeliveryRouteReference,
    /// Net area supplier reference
    NetAreaSupplierReference,
    /// Time series reference
    TimeSeriesReference,
    /// Connecting point to central grid
    ConnectingPointToCentralGrid,
    /// Marketing plan identification number (MPIN)
    MarketingPlanIdentificationNumberMpin,
    /// Entity reference number, previous
    EntityReferenceNumberPrevious,
    /// International Standard Industrial Classification (ISIC)
    InternationalStandardIndustrialClassificationIsic,
    /// Customs pre-approval ruling number
    CustomsPreApprovalRulingNumber,
    ///  Account payable number
    AccountPayableNumber,
    /// First financial institution's transaction reference
    FirstFinancialInstitutionsTransactionReference,
    /// Product characteristics directory
    ProductCharacteristicsDirectory,
    /// Supplier's customer reference number
    SuppliersCustomerReferenceNumber,
    /// Inventory report request number
    InventoryReportRequestNumber,
    /// Metering point
    MeteringPoint,
    /// Passenger reservation number
    PassengerReservationNumber,
    /// Slaughterhouse approval number
    SlaughterhouseApprovalNumber,
    /// Meat cutting plant approval number
    MeatCuttingPlantApprovalNumber,
    /// Customer travel service identifier
    CustomerTravelServiceIdentifier,
    /// Export control classification number
    ExportControlClassificationNumber,
    /// Broker reference 3
    BrokerReference3,
    /// Consignment information
    ConsignmentInformation,
    /// Goods item information
    GoodsItemInformation,
    /// Dangerous Goods information
    DangerousGoodsInformation,
    /// Pilotage services exemption number
    PilotageServicesExemptionNumber,
    /// Person registration number
    PersonRegistrationNumber,
    /// Place of packing approval number
    PlacePackingApprovalNumber,
    /// Original Mandate Reference
    OriginalMandateReference,
    /// Mandate Reference
    MandateReference,
    /// Reservation station indentifier
    ReservationStationIndentifier,
    /// Unique goods shipment identifier
    UniqueGoodsShipmentIdentifier,
    /// Framework Agreement Number
    FrameworkAgreementNumber,
    /// Hash value
    HashValue,
    /// Movement reference number
    MovementReferenceNumber,
    /// Economic Operators Registration and Identification Number
    EconomicOperatorsRegistrationAndIdentificationNumber,
    /// Local Reference Number
    LocalReferenceNumber,
    /// Rate code number
    RateCodeNumber,
    /// Air waybill number
    AirWaybillNumber,
    /// Documentary credit amendment number
    DocumentaryCreditAmendmentNumber,
    /// Advising bank's reference
    AdvisingBanksReference,
    /// Cost centre
    CostCentre,
    /// Work item quantity determination
    WorkItemQuantityDetermination,
    /// Internal data process number
    InternalDataProcessNumber,
    /// Category of work reference
    CategoryWorkReference,
    /// Policy form number
    PolicyFormNumber,
    /// Net area
    NetArea,
    /// Service provider
    ServiceProvider,
    /// Error position
    ErrorPosition,
    /// Service category reference
    ServiceCategoryReference,
    /// Connected location
    ConnectedLocation,
    /// Related party
    RelatedParty,
    /// Latest accounting entry record reference
    LatestAccountingEntryRecordReference,
    /// Accounting entry
    AccountingEntry,
    /// Document reference, original
    DocumentReferenceOriginal,
    /// Hygienic Certificate number, national
    HygienicCertificateNumberNational,
    /// Administrative Reference Code
    AdministrativeReferenceCode,
    /// Pick-up sheet number
    PickUpSheetNumber,
    /// Phone number
    PhoneNumber,
    /// Buyer's fund number
    BuyersFundNumber,
    /// Company trading account number
    CompanyTradingAccountNumber,
    /// Reserved goods identifier
    ReservedGoodsIdentifier,
    /// Handling and movement reference number
    HandlingAndMovementReferenceNumber,
    /// Instruction to despatch reference number
    InstructionToDespatchReferenceNumber,
    /// Instruction for returns number
    InstructionForReturnsNumber,
    /// Metered services consumption report number
    MeteredServicesConsumptionReportNumber,
    /// Order status enquiry number
    OrderStatusEnquiryNumber,
    /// Firm booking reference number
    FirmBookingReferenceNumber,
    /// Product inquiry number
    ProductInquiryNumber,
    /// Split delivery number
    SplitDeliveryNumber,
    /// Service relation number
    ServiceRelationNumber,
    /// Serial shipping container code
    SerialShippingContainerCode,
    /// Test specification number
    TestSpecificationNumber,
    /// Transport status report number
    TransportStatusReportNumber,
    /// Tooling contract number
    ToolingContractNumber,
    /// Formula reference number
    FormulaReferenceNumber,
    /// Pre-agreement number
    PreAgreementNumber,
    /// Product certification number
    ProductCertificationNumber,
    /// Consignment contract number
    ConsignmentContractNumber,
    /// Product specification reference number
    ProductSpecificationReferenceNumber,
    /// Payroll deduction advice reference
    PayrollDeductionAdviceReference,
    /// TRACES party identification
    TracesPartyIdentification,
    /// Block Stowage Reference
    BlockStowageReference,
    ///  Beginning meter reading actual
    BeginningMeterReadingActual,
    ///  Buyer's contract number
    BuyersContractNumber,
    ///  Bid number
    BidNumber,
    ///  Beginning meter reading estimated
    BeginningMeterReadingEstimated,
    ///  House bill of lading number
    HouseBillLadingNumber,
    ///  Bill of lading number
    BillLadingNumber,
    ///  Consignment identifier, carrier assigned
    ConsignmentIdentifierCarrierAssigned,
    ///  Blanket order number
    BlanketOrderNumber,
    ///  Broker or sales office number
    BrokerOrSalesOfficeNumber,
    ///  Batch number/lot number
    BatchNumberLotNumber,
    /// Battery and accumulator producer registration number
    BatteryAndAccumulatorProducerRegistrationNumber,
    ///  Blended with number
    BlendedWithNumber,
    /// IATA Cargo Agent CASS Address number
    IataCargoAgentCassAddressNumber,
    /// Matching of entries, balanced
    MatchingEntriesBalanced,
    /// Entry flagging
    EntryFlagging,
    /// Matching of entries, unbalanced
    MatchingEntriesUnbalanced,
    /// Document reference, internal
    DocumentReferenceInternal,
    /// European Value Added Tax identification
    EuropeanValueAddedTaxIdentification,
    /// Cost accounting document
    CostAccountingDocument,
    /// Grid operator's customer reference number
    GridOperatorsCustomerReferenceNumber,
    /// Ticket control number
    TicketControlNumber,
    /// Order shipment grouping reference
    OrderShipmentGroupingReference,
    ///  Credit note number
    CreditNoteNumber,
    /// Ceding company
    CedingCompany,
    /// Debit letter number
    DebitLetterNumber,
    /// Consignee's further order
    ConsigneesFurtherOrder,
    /// Animal farm licence number
    AnimalFarmLicenceNumber,
    /// Consignor's further order
    ConsignorsFurtherOrder,
    ///  Consignee's order number
    ConsigneesOrderNumber,
    ///  Customer catalogue number
    CustomerCatalogueNumber,
    ///  Cheque number
    ChequeNumber,
    /// Checking number
    CheckingNumber,
    ///  Credit memo number
    CreditMemoNumber,
    /// Road consignment note number
    RoadConsignmentNoteNumber,
    ///  Carrier's reference number
    CarriersReferenceNumber,
    /// Charges note document attachment indicator
    ChargesNoteDocumentAttachmentIndicator,
    /// Call off order number
    CallOffOrderNumber,
    ///  Condition of purchase document number
    ConditionPurchaseDocumentNumber,
    ///  Customer reference number
    CustomerReferenceNumber,
    /// Transport means journey identifier
    TransportMeansJourneyIdentifier,
    ///  Condition of sale document number
    ConditionSaleDocumentNumber,
    /// Team assignment number
    TeamAssignmentNumber,
    ///  Contract number
    ContractNumber,
    ///  Consignment identifier, consignor assigned
    ConsignmentIdentifierConsignorAssigned,
    ///  Container operators reference number
    ContainerOperatorsReferenceNumber,
    ///  Package number
    PackageNumber,
    ///  Cooperation contract number
    CooperationContractNumber,
    ///  Deferment approval number
    DefermentApprovalNumber,
    /// Debit account number
    DebitAccountNumber,
    ///  Buyer's debtor number
    BuyersDebtorNumber,
    ///  Distributor invoice number
    DistributorInvoiceNumber,
    ///  Debit note number
    DebitNoteNumber,
    ///  Document identifier
    DocumentIdentifier,
    ///  Delivery note number
    DeliveryNoteNumber,
    ///  Dock receipt number
    DockReceiptNumber,
    ///  Ending meter reading actual
    EndingMeterReadingActual,
    ///  Embargo permit number
    EmbargoPermitNumber,
    ///  Export declaration
    ExportDeclaration,
    ///  Ending meter reading estimated
    EndingMeterReadingEstimated,
    /// Electrical and electronic equipment producer registration
    ElectricalAndElectronicEquipmentProducerRegistration,
    ///  Employer's identification number
    EmployersIdentificationNumber,
    ///  Embargo number
    EmbargoNumber,
    ///  Equipment number
    EquipmentNumber,
    ///  Container/equipment receipt number
    ContainerEquipmentReceiptNumber,
    /// Exporter's reference number
    ExportersReferenceNumber,
    ///  Excess transportation number
    ExcessTransportationNumber,
    ///  Export permit identifier
    ExportPermitIdentifier,
    ///  Fiscal number
    FiscalNumber,
    ///  Consignment identifier, freight forwarder assigned
    ConsignmentIdentifierFreightForwarderAssigned,
    ///  File line identifier
    FileLineIdentifier,
    /// Flow reference number
    FlowReferenceNumber,
    ///  Freight bill number
    FreightBillNumber,
    ///  Foreign exchange
    ForeignExchange,
    ///  Final sequence number
    FinalSequenceNumber,
    ///  Free zone identifier
    FreeZoneIdentifier,
    ///  File version number
    FileVersionNumber,
    ///  Foreign exchange contract number
    ForeignExchangeContractNumber,
    ///  Standard's number
    StandardsNumber,
    ///  Government contract number
    GovernmentContractNumber,
    ///  Standard's code number
    StandardsCodeNumber,
    /// General declaration number
    GeneralDeclarationNumber,
    ///  Government reference number
    GovernmentReferenceNumber,
    ///  Harmonised system number
    HarmonisedSystemNumber,
    /// House waybill number
    HouseWaybillNumber,
    ///  Internal vendor number
    InternalVendorNumber,
    ///  In bond number
    InBondNumber,
    /// IATA cargo agent code number
    IataCargoAgentCodeNumber,
    /// Insurance certificate reference number
    InsuranceCertificateReferenceNumber,
    /// Insurance contract reference number
    InsuranceContractReferenceNumber,
    ///  Initial sample inspection report number
    InitialSampleInspectionReportNumber,
    ///  Internal order number
    InternalOrderNumber,
    /// Intermediary broker
    IntermediaryBroker,
    /// Interchange number new
    InterchangeNumberNew,
    /// Interchange number old
    InterchangeNumberOld,
    ///  Import permit identifier
    ImportPermitIdentifier,
    ///  Invoice number suffix
    InvoiceNumberSuffix,
    ///  Internal customer number
    InternalCustomerNumber,
    ///  Invoice document identifier
    InvoiceDocumentIdentifier,
    ///  Job number
    JobNumber,
    ///  Ending job sequence number
    EndingJobSequenceNumber,
    ///  Shipping label serial number
    ShippingLabelSerialNumber,
    /// Loading authorisation identifier
    LoadingAuthorisationIdentifier,
    /// Lower number in range
    LowerNumberInRange,
    ///  Lockbox
    Lockbox,
    ///  Letter of credit number
    LetterCreditNumber,
    ///  Document line identifier
    DocumentLineIdentifier,
    ///  Load planning number
    LoadPlanningNumber,
    /// Reservation office identifier
    ReservationOfficeIdentifier,
    ///  Bar coded label serial number
    BarCodedLabelSerialNumber,
    ///  Ship notice/manifest number
    ShipNoticeManifestNumber,
    ///  Master bill of lading number
    MasterBillLadingNumber,
    ///  Manufacturer's part number
    ManufacturersPartNumber,
    ///  Meter unit number
    MeterUnitNumber,
    ///  Manufacturing order number
    ManufacturingOrderNumber,
    ///  Message recipient
    MessageRecipient,
    /// Mailing reference number
    MailingReferenceNumber,
    ///  Message sender
    MessageSender,
    /// Manufacturer's material safety data sheet number
    ManufacturersMaterialSafetyDataSheetNumber,
    /// Master air waybill number
    MasterAirWaybillNumber,
    ///  North American hazardous goods classification number
    NorthAmericanHazardousGoodsClassificationNumber,
    ///  Nota Fiscal
    NotaFiscal,
    ///  Current invoice number
    CurrentInvoiceNumber,
    ///  Previous invoice number
    PreviousInvoiceNumber,
    ///  Order document identifier, buyer assigned
    OrderDocumentIdentifierBuyerAssigned,
    ///  Original purchase order
    OriginalPurchaseOrder,
    ///  General order number
    GeneralOrderNumber,
    ///  Payer's financial institution account number
    PayersFinancialInstitutionAccountNumber,
    ///  Production code
    ProductionCode,
    ///  Promotion deal number
    PromotionDealNumber,
    ///  Plant number
    PlantNumber,
    ///  Prime contractor contract number
    PrimeContractorContractNumber,
    ///  Price list version number
    PriceListVersionNumber,
    ///  Packing list number
    PackingListNumber,
    ///  Price list number
    PriceListNumber,
    /// Purchase order response number
    PurchaseOrderResponseNumber,
    ///  Purchase order change number
    PurchaseOrderChangeNumber,
    ///  Payment reference
    PaymentReference,
    ///  Price quote number
    PriceQuoteNumber,
    ///  Purchase order number suffix
    PurchaseOrderNumberSuffix,
    ///  Prior purchase order number
    PriorPurchaseOrderNumber,
    ///  Payee's financial institution account number
    PayeesFinancialInstitutionAccountNumber,
    ///  Remittance advice number
    RemittanceAdviceNumber,
    ///  Rail/road routing code
    RailRoadRoutingCode,
    /// Railway consignment note number
    RailwayConsignmentNoteNumber,
    ///  Release number
    ReleaseNumber,
    /// Consignment receipt identifier
    ConsignmentReceiptIdentifier,
    ///  Export reference number
    ExportReferenceNumber,
    ///  Payer's financial institution transit routing No.(ACH
    PayersFinancialInstitutionTransitRoutingNoAch,
    ///  Payee's financial institution transit routing No.
    PayeesFinancialInstitutionTransitRoutingNo,
    ///  Sales person number
    SalesPersonNumber,
    ///  Sales region number
    SalesRegionNumber,
    ///  Sales department number
    SalesDepartmentNumber,
    ///  Serial number
    SerialNumber,
    /// Allocated seat
    AllocatedSeat,
    ///  Ship from
    ShipFrom,
    ///  Previous highest schedule number
    PreviousHighestScheduleNumber,
    ///  SID (Shipper's identifying number for shipment)
    SidShippersIdentifyingNumberForShipment,
    ///  Sales office number
    SalesOfficeNumber,
    ///  Transport equipment seal identifier
    TransportEquipmentSealIdentifier,
    ///  Scan line
    ScanLine,
    ///  Equipment sequence number
    EquipmentSequenceNumber,
    /// Shipment reference number
    ShipmentReferenceNumber,
    ///  Sellers reference number
    SellersReferenceNumber,
    /// Station reference number
    StationReferenceNumber,
    ///  Swap order number
    SwapOrderNumber,
    ///  Specification number
    SpecificationNumber,
    ///  Trucker's bill of lading
    TruckersBillLading,
    /// Terminal operator's consignment reference
    TerminalOperatorsConsignmentReference,
    ///  Telex message number
    TelexMessageNumber,
    ///  Transfer number
    TransferNumber,
    ///  TIR carnet number
    TirCarnetNumber,
    /// Transport instruction number
    TransportInstructionNumber,
    ///  Tax exemption licence number
    TaxExemptionLicenceNumber,
    ///  Transaction reference number
    TransactionReferenceNumber,
    ///  Test report number
    TestReportNumber,
    /// Upper number of range
    UpperNumberRange,
    ///  Ultimate customer's reference number
    UltimateCustomersReferenceNumber,
    /// Unique consignment reference number
    UniqueConsignmentReferenceNumber,
    ///  United Nations Dangerous Goods identifier
    UnitedNationsDangerousGoodsIdentifier,
    ///  Ultimate customer's order number
    UltimateCustomersOrderNumber,
    /// Uniform Resource Identifier
    UniformResourceIdentifier,
    ///  VAT registration number
    VatRegistrationNumber,
    ///  Vendor contract number
    VendorContractNumber,
    /// Transport equipment gross mass verification reference
    TransportEquipmentGrossMassVerificationReference,
    ///  Vessel identifier
    VesselIdentifier,
    ///  Order number (vendor)
    OrderNumberVendor,
    /// Voyage number
    VoyageNumber,
    /// Transport equipment gross mass verification order reference
    TransportEquipmentGrossMassVerificationOrderReference,
    ///  Vendor product number
    VendorProductNumber,
    ///  Vendor ID number
    VendorIdNumber,
    ///  Vendor order number suffix
    VendorOrderNumberSuffix,
    ///  Motor vehicle identification number
    MotorVehicleIdentificationNumber,
    ///  Voucher number
    VoucherNumber,
    ///  Warehouse entry number
    WarehouseEntryNumber,
    ///  Weight agreement number
    WeightAgreementNumber,
    ///  Well number
    WellNumber,
    ///  Warehouse receipt number
    WarehouseReceiptNumber,
    ///  Warehouse storage location number
    WarehouseStorageLocationNumber,
    ///  Rail waybill number
    RailWaybillNumber,
    ///  Company/place registration number
    CompanyPlaceRegistrationNumber,
    ///  Cargo control number
    CargoControlNumber,
    ///  Previous cargo control number
    PreviousCargoControlNumber,
    /// Mutually defined reference number
    MutuallyDefinedReferenceNumber,
}

impl std::fmt::Display for Enum1153 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for Enum1153 {
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
    }
}

impl crate::Code for Enum1153 {
    fn code(self) -> &'static str {
        match self {
            Enum1153::OrderAcknowledgementDocumentIdentifier => "AAA",
            Enum1153::ProformaInvoiceDocumentIdentifier => "AAB",
            Enum1153::DocumentaryCreditIdentifier => "AAC",
            Enum1153::ContractDocumentAddendumIdentifier => "AAD",
            Enum1153::GoodsDeclarationNumber => "AAE",
            Enum1153::DebitCardNumber => "AAF",
            Enum1153::OfferNumber => "AAG",
            Enum1153::BanksBatchInterbankTransactionReferenceNumber => "AAH",
            Enum1153::BanksIndividualInterbankTransactionReferenceNumber => "AAI",
            Enum1153::DeliveryOrderNumber => "AAJ",
            Enum1153::DespatchAdviceNumber => "AAK",
            Enum1153::DrawingNumber => "AAL",
            Enum1153::WaybillNumber => "AAM",
            Enum1153::DeliveryScheduleNumber => "AAN",
            Enum1153::ConsignmentIdentifierConsigneeAssigned => "AAO",
            Enum1153::PartialShipmentIdentifier => "AAP",
            Enum1153::TransportEquipmentIdentifier => "AAQ",
            Enum1153::MunicipalityAssignedBusinessRegistryNumber => "AAR",
            Enum1153::TransportContractDocumentIdentifier => "AAS",
            Enum1153::MasterLabelNumber => "AAT",
            Enum1153::DespatchNoteDocumentIdentifier => "AAU",
            Enum1153::EnquiryNumber => "AAV",
            Enum1153::DocketNumber => "AAW",
            Enum1153::CivilActionNumber => "AAX",
            Enum1153::CarriersAgentReferenceNumber => "AAY",
            Enum1153::StandardCarrierAlphaCodeScacNumber => "AAZ",
            Enum1153::CustomsValuationDecisionNumber => "ABA",
            Enum1153::EndUseAuthorizationNumber => "ABB",
            Enum1153::AntiDumpingCaseNumber => "ABC",
            Enum1153::CustomsTariffNumber => "ABD",
            Enum1153::DeclarantsReferenceNumber => "ABE",
            Enum1153::RepairEstimateNumber => "ABF",
            Enum1153::CustomsDecisionRequestNumber => "ABG",
            Enum1153::SubHouseBillLadingNumber => "ABH",
            Enum1153::TaxPaymentIdentifier => "ABI",
            Enum1153::QuotaNumber => "ABJ",
            Enum1153::TransitOnwardCarriageGuaranteeBondNumber => "ABK",
            Enum1153::CustomsGuaranteeNumber => "ABL",
            Enum1153::ReplacingPartNumber => "ABM",
            Enum1153::SellersCatalogueNumber => "ABN",
            Enum1153::OriginatorsReference => "ABO",
            Enum1153::DeclarantsCustomsIdentityNumber => "ABP",
            Enum1153::ImporterReferenceNumber => "ABQ",
            Enum1153::ExportClearanceInstructionReferenceNumber => "ABR",
            Enum1153::ImportClearanceInstructionReferenceNumber => "ABS",
            Enum1153::GoodsDeclarationDocumentIdentifierCustoms => "ABT",
            Enum1153::ArticleNumber => "ABU",
            Enum1153::IntraPlantRouting => "ABV",
            Enum1153::StockKeepingUnitNumber => "ABW",
            Enum1153::TextElementIdentifierDeletionReference => "ABX",
            Enum1153::AllotmentIdentificationAir => "ABY",
            Enum1153::VehicleLicenceNumber => "ABZ",
            Enum1153::AirCargoTransferManifest => "AC",
            Enum1153::CargoAcceptanceOrderReferenceNumber => "ACA",
            Enum1153::UsGovernmentAgencyNumber => "ACB",
            Enum1153::ShippingUnitIdentification => "ACC",
            Enum1153::AdditionalReferenceNumber => "ACD",
            Enum1153::RelatedDocumentNumber => "ACE",
            Enum1153::AddresseeReference => "ACF",
            Enum1153::AtaCarnetNumber => "ACG",
            Enum1153::PackagingUnitIdentification => "ACH",
            Enum1153::OuterpackagingUnitIdentification => "ACI",
            Enum1153::CustomerMaterialSpecificationNumber => "ACJ",
            Enum1153::BankReference => "ACK",
            Enum1153::PrincipalReferenceNumber => "ACL",
            Enum1153::CollectionAdviceDocumentIdentifier => "ACN",
            Enum1153::IronChargeNumber => "ACO",
            Enum1153::HotRollNumber => "ACP",
            Enum1153::ColdRollNumber => "ACQ",
            Enum1153::RailwayWagonNumber => "ACR",
            Enum1153::UniqueClaimsReferenceNumberSender => "ACT",
            Enum1153::LossEventNumber => "ACU",
            Enum1153::EstimateOrderReferenceNumber => "ACV",
            Enum1153::ReferenceNumberToPreviousMessage => "ACW",
            Enum1153::BankersAcceptance => "ACX",
            Enum1153::DutyMemoNumber => "ACY",
            Enum1153::EquipmentTransportChargeNumber => "ACZ",
            Enum1153::BuyersItemNumber => "ADA",
            Enum1153::MaturedCertificateDeposit => "ADB",
            Enum1153::Loan => "ADC",
            Enum1153::AnalysisNumberTestNumber => "ADD",
            Enum1153::AccountNumber => "ADE",
            Enum1153::TreatyNumber => "ADF",
            Enum1153::CatastropheNumber => "ADG",
            Enum1153::BureauSigningStatementReference => "ADI",
            Enum1153::CompanySyndicateReference1 => "ADJ",
            Enum1153::CompanySyndicateReference2 => "ADK",
            Enum1153::OrderingCustomerConsignmentReferenceNumber => "ADL",
            Enum1153::ShipownersAuthorizationNumber => "ADM",
            Enum1153::InlandTransportOrderNumber => "ADN",
            Enum1153::ContainerWorkOrderReferenceNumber => "ADO",
            Enum1153::StatementNumber => "ADP",
            Enum1153::UniqueMarketReference => "ADQ",
            Enum1153::GroupAccounting => "ADT",
            Enum1153::BrokerReference1 => "ADU",
            Enum1153::BrokerReference2 => "ADV",
            Enum1153::LloydsClaimsOfficeReference => "ADW",
            Enum1153::SecureDeliveryTermsAndConditionsAgreementReference => "ADX",
            Enum1153::ReportNumber => "ADY",
            Enum1153::TraderAccountNumber => "ADZ",
            Enum1153::AuthorizationForExpenseAfeNumber => "AE",
            Enum1153::GovernmentAgencyReferenceNumber => "AEA",
            Enum1153::AssemblyNumber => "AEB",
            Enum1153::SymbolNumber => "AEC",
            Enum1153::CommodityNumber => "AED",
            Enum1153::Eur1CertificateNumber => "AEE",
            Enum1153::CustomerProcessSpecificationNumber => "AEF",
            Enum1153::CustomerSpecificationNumber => "AEG",
            Enum1153::ApplicableInstructionsOrStandards => "AEH",
            Enum1153::RegistrationNumberPreviousCustomsDeclaration => "AEI",
            Enum1153::PostEntryReference => "AEJ",
            Enum1153::PaymentOrderNumber => "AEK",
            Enum1153::DeliveryNumberTransport => "AEL",
            Enum1153::TransportRoute => "AEM",
            Enum1153::CustomersUnitInventoryNumber => "AEN",
            Enum1153::ProductReservationNumber => "AEO",
            Enum1153::ProjectNumber => "AEP",
            Enum1153::DrawingListNumber => "AEQ",
            Enum1153::ProjectSpecificationNumber => "AER",
            Enum1153::PrimaryReference => "AES",
            Enum1153::RequestForCancellationNumber => "AET",
            Enum1153::SuppliersControlNumber => "AEU",
            Enum1153::ShippingNoteNumber => "AEV",
            Enum1153::EmptyContainerBillNumber => "AEW",
            Enum1153::NonNegotiableMaritimeTransportDocumentNumber => "AEX",
            Enum1153::SubstituteAirWaybillNumber => "AEY",
            Enum1153::DespatchNotePostParcelsNumber => "AEZ",
            Enum1153::AirlinesFlightIdentificationNumber => "AF",
            Enum1153::ThroughBillLadingNumber => "AFA",
            Enum1153::CargoManifestNumber => "AFB",
            Enum1153::BordereauNumber => "AFC",
            Enum1153::CustomsItemNumber => "AFD",
            Enum1153::ExportControlCommodityNumberEccn => "AFE",
            Enum1153::MarkingLabelReference => "AFF",
            Enum1153::TariffNumber => "AFG",
            Enum1153::ReplenishmentPurchaseOrderNumber => "AFH",
            Enum1153::ImmediateTransportationNoForInBondMovement => "AFI",
            Enum1153::TransportationExportationNoForInBondMovement => "AFJ",
            Enum1153::ImmediateExportationNoForInBondMovement => "AFK",
            Enum1153::AssociatedInvoices => "AFL",
            Enum1153::SecondaryCustomsReference => "AFM",
            Enum1153::AccountPartysReference => "AFN",
            Enum1153::BeneficiarysReference => "AFO",
            Enum1153::SecondBeneficiarysReference => "AFP",
            Enum1153::ApplicantsBankReference => "AFQ",
            Enum1153::IssuingBanksReference => "AFR",
            Enum1153::BeneficiarysBankReference => "AFS",
            Enum1153::DirectPaymentValuationNumber => "AFT",
            Enum1153::DirectPaymentValuationRequestNumber => "AFU",
            Enum1153::QuantityValuationNumber => "AFV",
            Enum1153::QuantityValuationRequestNumber => "AFW",
            Enum1153::BillQuantitiesNumber => "AFX",
            Enum1153::PaymentValuationNumber => "AFY",
            Enum1153::SituationNumber => "AFZ",
            Enum1153::AgreementToPayNumber => "AGA",
            Enum1153::ContractPartyReferenceNumber => "AGB",
            Enum1153::AccountPartysBankReference => "AGC",
            Enum1153::AgentsBankReference => "AGD",
            Enum1153::AgentsReference => "AGE",
            Enum1153::ApplicantsReference => "AGF",
            Enum1153::DisputeNumber => "AGG",
            Enum1153::CreditRatingAgencysReferenceNumber => "AGH",
            Enum1153::RequestNumber => "AGI",
            Enum1153::SingleTransactionSequenceNumber => "AGJ",
            Enum1153::ApplicationReferenceNumber => "AGK",
            Enum1153::DeliveryVerificationCertificate => "AGL",
            Enum1153::NumberTemporaryImportationDocument => "AGM",
            Enum1153::ReferenceNumberQuotedOnStatement => "AGN",
            Enum1153::SendersReferenceToOriginalMessage => "AGO",
            Enum1153::CompanyIssuedEquipmentId => "AGP",
            Enum1153::DomesticFlightNumber => "AGQ",
            Enum1153::InternationalFlightNumber => "AGR",
            Enum1153::EmployerIdentificationNumberServiceBureau => "AGS",
            Enum1153::ServiceGroupIdentificationNumber => "AGT",
            Enum1153::MemberNumber => "AGU",
            Enum1153::PreviousMemberNumber => "AGV",
            Enum1153::SchemePlanNumber => "AGW",
            Enum1153::PreviousSchemePlanNumber => "AGX",
            Enum1153::ReceivingPartysMemberIdentification => "AGY",
            Enum1153::PayrollNumber => "AGZ",
            Enum1153::PackagingSpecificationNumber => "AHA",
            Enum1153::AuthorityIssuedEquipmentIdentification => "AHB",
            Enum1153::TrainingFlightNumber => "AHC",
            Enum1153::FundCodeNumber => "AHD",
            Enum1153::SignalCodeNumber => "AHE",
            Enum1153::MajorForceProgramNumber => "AHF",
            Enum1153::NominationNumber => "AHG",
            Enum1153::LaboratoryRegistrationNumber => "AHH",
            Enum1153::TransportContractReferenceNumber => "AHI",
            Enum1153::PayeesReferenceNumber => "AHJ",
            Enum1153::PayersReferenceNumber => "AHK",
            Enum1153::CreditorsReferenceNumber => "AHL",
            Enum1153::DebtorsReferenceNumber => "AHM",
            Enum1153::JointVentureReferenceNumber => "AHN",
            Enum1153::ChamberCommerceRegistrationNumber => "AHO",
            Enum1153::TaxRegistrationNumber => "AHP",
            Enum1153::WoolIdentificationNumber => "AHQ",
            Enum1153::WoolTaxReferenceNumber => "AHR",
            Enum1153::MeatProcessingEstablishmentRegistrationNumber => "AHS",
            Enum1153::QuarantineTreatmentStatusReferenceNumber => "AHT",
            Enum1153::RequestForQuoteNumber => "AHU",
            Enum1153::ManualProcessingAuthorityNumber => "AHV",
            Enum1153::RateNoteNumber => "AHX",
            Enum1153::FreightForwarderNumber => "AHY",
            Enum1153::CustomsReleaseCode => "AHZ",
            Enum1153::ComplianceCodeNumber => "AIA",
            Enum1153::DepartmentTransportationBondNumber => "AIB",
            Enum1153::ExportEstablishmentNumber => "AIC",
            Enum1153::CertificateConformity => "AID",
            Enum1153::MinisterialCertificateHomologation => "AIE",
            Enum1153::PreviousDeliveryInstructionNumber => "AIF",
            Enum1153::PassportNumber => "AIG",
            Enum1153::CommonTransactionReferenceNumber => "AIH",
            Enum1153::BanksCommonTransactionReferenceNumber => "AII",
            Enum1153::CustomersIndividualTransactionReferenceNumber => "AIJ",
            Enum1153::BanksIndividualTransactionReferenceNumber => "AIK",
            Enum1153::CustomersCommonTransactionReferenceNumber => "AIL",
            Enum1153::IndividualTransactionReferenceNumber => "AIM",
            Enum1153::ProductSourcingAgreementNumber => "AIN",
            Enum1153::CustomsTranshipmentNumber => "AIO",
            Enum1153::CustomsPreferenceInquiryNumber => "AIP",
            Enum1153::PackingPlantNumber => "AIQ",
            Enum1153::OriginalCertificateNumber => "AIR",
            Enum1153::ProcessingPlantNumber => "AIS",
            Enum1153::SlaughterPlantNumber => "AIT",
            Enum1153::ChargeCardAccountNumber => "AIU",
            Enum1153::EventReferenceNumber => "AIV",
            Enum1153::TransportSectionReferenceNumber => "AIW",
            Enum1153::ReferredProductForMechanicalAnalysis => "AIX",
            Enum1153::ReferredProductForChemicalAnalysis => "AIY",
            Enum1153::ConsolidatedInvoiceNumber => "AIZ",
            Enum1153::PartReferenceIndicatorInADrawing => "AJA",
            Enum1153::USCodeFederalRegulationsCfr => "AJB",
            Enum1153::PurchasingActivityClauseNumber => "AJC",
            Enum1153::USDefenseFederalAcquisitionRegulationSupplement => "AJD",
            Enum1153::AgencyClauseNumber => "AJE",
            Enum1153::CircularPublicationNumber => "AJF",
            Enum1153::USFederalAcquisitionRegulation => "AJG",
            Enum1153::USGeneralServicesAdministrationRegulation => "AJH",
            Enum1153::USFederalInformationResourcesManagementRegulation => "AJI",
            Enum1153::Paragraph => "AJJ",
            Enum1153::SpecialInstructionsNumber => "AJK",
            Enum1153::SiteSpecificProceduresTermsAndConditionsNumber => "AJL",
            Enum1153::MasterSolicitationProceduresTermsAndConditions => "AJM",
            Enum1153::USDepartmentVeteransAffairsAcquisitionRegulation => "AJN",
            Enum1153::MilitaryInterdepartmentalPurchaseRequestMiprNumber => "AJO",
            Enum1153::ForeignMilitarySalesNumber => "AJP",
            Enum1153::DefensePrioritiesAllocationSystemPriorityRating => "AJQ",
            Enum1153::WageDeterminationNumber => "AJR",
            Enum1153::AgreementNumber => "AJS",
            Enum1153::StandardIndustryClassificationSicNumber => "AJT",
            Enum1153::EndItemNumber => "AJU",
            Enum1153::FederalSupplyScheduleItemNumber => "AJV",
            Enum1153::TechnicalDocumentNumber => "AJW",
            Enum1153::TechnicalOrderNumber => "AJX",
            Enum1153::Suffix => "AJY",
            Enum1153::TransportationAccountNumber => "AJZ",
            Enum1153::ContainerDispositionOrderReferenceNumber => "AKA",
            Enum1153::ContainerPrefix => "AKB",
            Enum1153::TransportEquipmentReturnReference => "AKC",
            Enum1153::TransportEquipmentSurveyReference => "AKD",
            Enum1153::TransportEquipmentSurveyReportNumber => "AKE",
            Enum1153::TransportEquipmentStuffingOrder => "AKF",
            Enum1153::VehicleIdentificationNumberVin => "AKG",
            Enum1153::GovernmentBillLading => "AKH",
            Enum1153::OrderingCustomersSecondReferenceNumber => "AKI",
            Enum1153::DirectDebitReference => "AKJ",
            Enum1153::MeterReadingAtBeginningDelivery => "AKK",
            Enum1153::MeterReadingAtEndDelivery => "AKL",
            Enum1153::ReplenishmentPurchaseOrderRangeStartNumber => "AKM",
            Enum1153::ThirdBanksReference => "AKN",
            Enum1153::ActionAuthorizationNumber => "AKO",
            Enum1153::AppropriationNumber => "AKP",
            Enum1153::ProductChangeAuthorityNumber => "AKQ",
            Enum1153::GeneralCargoConsignmentReferenceNumber => "AKR",
            Enum1153::CatalogueSequenceNumber => "AKS",
            Enum1153::ForwardingOrderNumber => "AKT",
            Enum1153::TransportEquipmentSurveyReferenceNumber => "AKU",
            Enum1153::LeaseContractReference => "AKV",
            Enum1153::TransportCostsReferenceNumber => "AKW",
            Enum1153::TransportEquipmentStrippingOrder => "AKX",
            Enum1153::PriorPolicyNumber => "AKY",
            Enum1153::PolicyNumber => "AKZ",
            Enum1153::ProcurementBudgetNumber => "ALA",
            Enum1153::DomesticInventoryManagementCode => "ALB",
            Enum1153::CustomerReferenceNumberAssignedToPreviousBalance => "ALC",
            Enum1153::PreviousCreditAdviceReferenceNumber => "ALD",
            Enum1153::ReportingFormNumber => "ALE",
            Enum1153::AuthorizationNumberForExceptionToDangerousGoods => "ALF",
            Enum1153::DangerousGoodsSecurityNumber => "ALG",
            Enum1153::DangerousGoodsTransportLicenceNumber => "ALH",
            Enum1153::PreviousRentalAgreementNumber => "ALI",
            Enum1153::NextRentalAgreementReasonNumber => "ALJ",
            Enum1153::ConsigneesInvoiceNumber => "ALK",
            Enum1153::MessageBatchNumber => "ALL",
            Enum1153::PreviousDeliveryScheduleNumber => "ALM",
            Enum1153::PhysicalInventoryRecountReferenceNumber => "ALN",
            Enum1153::ReceivingAdviceNumber => "ALO",
            Enum1153::ReturnableContainerReferenceNumber => "ALP",
            Enum1153::ReturnsNoticeNumber => "ALQ",
            Enum1153::SalesForecastNumber => "ALR",
            Enum1153::SalesReportNumber => "ALS",
            Enum1153::PreviousTaxControlNumber => "ALT",
            Enum1153::AgerdAerospaceGroundEquipmentRequirementDataNumber => "ALU",
            Enum1153::RegisteredCapitalReference => "ALV",
            Enum1153::StandardNumberInspectionDocument => "ALW",
            Enum1153::Model => "ALX",
            Enum1153::FinancialManagementReference => "ALY",
            Enum1153::NotificationForCollectionNumberNoticol => "ALZ",
            Enum1153::PreviousRequestForMeteredReadingReferenceNumber => "AMA",
            Enum1153::NextRentalAgreementNumber => "AMB",
            Enum1153::ReferenceNumberARequestForMeteredReading => "AMC",
            Enum1153::HasteningNumber => "AMD",
            Enum1153::RepairDataRequestNumber => "AME",
            Enum1153::ConsumptionDataRequestNumber => "AMF",
            Enum1153::ProfileNumber => "AMG",
            Enum1153::CaseNumber => "AMH",
            Enum1153::GovernmentQualityAssuranceAndControlLevelNumber => "AMI",
            Enum1153::PaymentPlanReference => "AMJ",
            Enum1153::ReplacedMeterUnitNumber => "AMK",
            Enum1153::ReplenishmentPurchaseOrderRangeEndNumber => "AML",
            Enum1153::InsurerAssignedReferenceNumber => "AMM",
            Enum1153::CanadianExciseEntryNumber => "AMN",
            Enum1153::PremiumRateTable => "AMO",
            Enum1153::AdviseThroughBanksReference => "AMP",
            Enum1153::UsDepartmentTransportationBondSuretyCode => "AMQ",
            Enum1153::UsFoodAndDrugAdministrationEstablishmentIndicator => "AMR",
            Enum1153::UsFederalCommunicationsCommissionFccImport => "AMS",
            Enum1153::GoodsAndServicesTaxIdentificationNumber => "AMT",
            Enum1153::IntegratedLogisticSupportCrossReferenceNumber => "AMU",
            Enum1153::DepartmentNumber => "AMV",
            Enum1153::BuyersCatalogueNumber => "AMW",
            Enum1153::FinancialSettlementPartysReferenceNumber => "AMX",
            Enum1153::StandardsVersionNumber => "AMY",
            Enum1153::PipelineNumber => "AMZ",
            Enum1153::AccountServicingBanksReferenceNumber => "ANA",
            Enum1153::CompletedUnitsPaymentRequestReference => "ANB",
            Enum1153::PaymentInAdvanceRequestReference => "ANC",
            Enum1153::ParentFile => "AND",
            Enum1153::SubFile => "ANE",
            Enum1153::CadFileLayerConvention => "ANF",
            Enum1153::TechnicalRegulation => "ANG",
            Enum1153::PlotFile => "ANH",
            Enum1153::FileConversionJournal => "ANI",
            Enum1153::AuthorizationNumber => "ANJ",
            Enum1153::ReferenceNumberAssignedByThirdParty => "ANK",
            Enum1153::DepositReferenceNumber => "ANL",
            Enum1153::NamedBanksReference => "ANM",
            Enum1153::DraweesReference => "ANN",
            Enum1153::CaseNeedPartysReference => "ANO",
            Enum1153::CollectingBanksReference => "ANP",
            Enum1153::RemittingBanksReference => "ANQ",
            Enum1153::PrincipalsBankReference => "ANR",
            Enum1153::PresentingBanksReference => "ANS",
            Enum1153::ConsigneesReference => "ANT",
            Enum1153::FinancialTransactionReferenceNumber => "ANU",
            Enum1153::CreditReferenceNumber => "ANV",
            Enum1153::ReceivingBanksAuthorizationNumber => "ANW",
            Enum1153::ClearingReference => "ANX",
            Enum1153::SendingBanksReferenceNumber => "ANY",
            Enum1153::DocumentaryPaymentReference => "AOA",
            Enum1153::AccountingFileReference => "AOD",
            Enum1153::SendersFileReferenceNumber => "AOE",
            Enum1153::ReceiversFileReferenceNumber => "AOF",
            Enum1153::SourceDocumentInternalReference => "AOG",
            Enum1153::PrincipalsReference => "AOH",
            Enum1153::DebitReferenceNumber => "AOI",
            Enum1153::Calendar => "AOJ",
            Enum1153::WorkShift => "AOK",
            Enum1153::WorkBreakdownStructure => "AOL",
            Enum1153::OrganisationBreakdownStructure => "AOM",
            Enum1153::WorkTaskChargeNumber => "AON",
            Enum1153::FunctionalWorkGroup => "AOO",
            Enum1153::WorkTeam => "AOP",
            Enum1153::Department => "AOQ",
            Enum1153::StatementWork => "AOR",
            Enum1153::WorkPackage => "AOS",
            Enum1153::PlanningPackage => "AOT",
            Enum1153::CostAccount => "AOU",
            Enum1153::WorkOrder => "AOV",
            Enum1153::TransportationControlNumberTcn => "AOW",
            Enum1153::ConstraintNotation => "AOX",
            Enum1153::EtermsReference => "AOY",
            Enum1153::ImplementationVersionNumber => "AOZ",
            Enum1153::AccountsReceivableNumber => "AP",
            Enum1153::IncorporatedLegalReference => "APA",
            Enum1153::PaymentInstalmentReferenceNumber => "APB",
            Enum1153::EquipmentOwnerReferenceNumber => "APC",
            Enum1153::CedentsClaimNumber => "APD",
            Enum1153::ReinsurersClaimNumber => "APE",
            Enum1153::PriceSalesCatalogueResponseReferenceNumber => "APF",
            Enum1153::GeneralPurposeMessageReferenceNumber => "APG",
            Enum1153::InvoicingDataSheetReferenceNumber => "APH",
            Enum1153::InventoryReportReferenceNumber => "API",
            Enum1153::CeilingFormulaReferenceNumber => "APJ",
            Enum1153::PriceVariationFormulaReferenceNumber => "APK",
            Enum1153::ReferenceToAccountServicingBanksMessage => "APL",
            Enum1153::PartySequenceNumber => "APM",
            Enum1153::PurchasersRequestReference => "APN",
            Enum1153::ContractorRequestReference => "APO",
            Enum1153::AccidentReferenceNumber => "APP",
            Enum1153::CommercialAccountSummaryReferenceNumber => "APQ",
            Enum1153::ContractBreakdownReference => "APR",
            Enum1153::ContractorRegistrationNumber => "APS",
            Enum1153::ApplicableCoefficientIdentificationNumber => "APT",
            Enum1153::SpecialBudgetAccountNumber => "APU",
            Enum1153::AuthorisationForRepairReference => "APV",
            Enum1153::ManufacturerDefinedRepairRatesReference => "APW",
            Enum1153::OriginalSubmitterLogNumber => "APX",
            Enum1153::OriginalSubmitterParentDataMaintenanceRequestDmr => "APY",
            Enum1153::OriginalSubmitterChildDataMaintenanceRequestDmr => "APZ",
            Enum1153::EntryPointAssessmentLogNumber => "AQA",
            Enum1153::EntryPointAssessmentLogNumberParentDmr => "AQB",
            Enum1153::EntryPointAssessmentLogNumberChildDmr => "AQC",
            Enum1153::DataStructureTag => "AQD",
            Enum1153::CentralSecretariatLogNumber => "AQE",
            Enum1153::CentralSecretariatLogNumberParentDataMaintenance => "AQF",
            Enum1153::CentralSecretariatLogNumberChildDataMaintenance => "AQG",
            Enum1153::InternationalAssessmentLogNumber => "AQH",
            Enum1153::InternationalAssessmentLogNumberParentData => "AQI",
            Enum1153::InternationalAssessmentLogNumberChildDataMaintenance => "AQJ",
            Enum1153::StatusReportNumber => "AQK",
            Enum1153::MessageDesignGroupNumber => "AQL",
            Enum1153::UsCustomsServiceUscsEntryCode => "AQM",
            Enum1153::BeginningJobSequenceNumber => "AQN",
            Enum1153::SendersClauseNumber => "AQO",
            Enum1153::DunAndBradstreetCanadas8DigitStandardIndustrial => "AQP",
            Enum1153::ActivitePrincipaleExerceeApeIdentifier => "AQQ",
            Enum1153::DunAndBradstreetUs8DigitStandardIndustrial => "AQR",
            Enum1153::NomenclatureActivityClassificationEconomyNace => "AQS",
            Enum1153::NormeActiviteFrancaiseNafIdentifier => "AQT",
            Enum1153::RegisteredContractorActivityType => "AQU",
            Enum1153::StatisticBundesAmtSbaIdentifier => "AQV",
            Enum1153::StateOrProvinceAssignedEntityIdentification => "AQW",
            Enum1153::InstituteSecurityAndFutureMarketDevelopmentIsfmd => "AQX",
            Enum1153::FileIdentificationNumber => "AQY",
            Enum1153::BankruptcyProcedureNumber => "AQZ",
            Enum1153::NationalGovernmentBusinessIdentificationNumber => "ARA",
            Enum1153::PriorDataUniversalNumberSystemDunsNumber => "ARB",
            Enum1153::CompaniesRegistryOfficeCroNumber => "ARC",
            Enum1153::CostaRicanJudicialNumber => "ARD",
            Enum1153::NumeroDeIdentificacionTributariaNit => "ARE",
            Enum1153::PatronNumber => "ARF",
            Enum1153::RegistroInformacionFiscalRifNumber => "ARG",
            Enum1153::RegistroUnicoDeContribuyenteRucNumber => "ARH",
            Enum1153::TokyoShokoResearchTsrBusinessIdentifier => "ARI",
            Enum1153::PersonalIdentityCardNumber => "ARJ",
            Enum1153::SystemeInformatiquePourLeRepertoireDesEntreprises => "ARK",
            Enum1153::SystemeInformatiquePourLeRepertoireDesEtablissements => "ARL",
            Enum1153::PublicationIssueNumber => "ARM",
            Enum1153::OriginalFilingNumber => "ARN",
            Enum1153::DocumentPageIdentifier => "ARO",
            Enum1153::PublicFilingRegistrationNumber => "ARP",
            Enum1153::RegiristoFederalDeContribuyentes => "ARQ",
            Enum1153::SocialSecurityNumber => "ARR",
            Enum1153::DocumentVolumeNumber => "ARS",
            Enum1153::BookNumber => "ART",
            Enum1153::StockExchangeCompanyIdentifier => "ARU",
            Enum1153::ImputationAccount => "ARV",
            Enum1153::FinancialPhaseReference => "ARW",
            Enum1153::TechnicalPhaseReference => "ARX",
            Enum1153::PriorContractorRegistrationNumber => "ARY",
            Enum1153::StockAdjustmentNumber => "ARZ",
            Enum1153::DispensationReference => "ASA",
            Enum1153::InvestmentReferenceNumber => "ASB",
            Enum1153::AssumingCompany => "ASC",
            Enum1153::BudgetChapter => "ASD",
            Enum1153::DutyFreeProductsSecurityNumber => "ASE",
            Enum1153::DutyFreeProductsReceiptAuthorisationNumber => "ASF",
            Enum1153::PartyInformationMessageReference => "ASG",
            Enum1153::FormalStatementReference => "ASH",
            Enum1153::ProofDeliveryReferenceNumber => "ASI",
            Enum1153::SuppliersCreditClaimReferenceNumber => "ASJ",
            Enum1153::PictureActualProduct => "ASK",
            Enum1153::PictureAGenericProduct => "ASL",
            Enum1153::TradingPartnerIdentificationNumber => "ASM",
            Enum1153::PriorTradingPartnerIdentificationNumber => "ASN",
            Enum1153::Password => "ASO",
            Enum1153::FormalReportNumber => "ASP",
            Enum1153::FundAccountNumber => "ASQ",
            Enum1153::SafeCustodyNumber => "ASR",
            Enum1153::MasterAccountNumber => "ASS",
            Enum1153::GroupReferenceNumber => "AST",
            Enum1153::AccountingTransmissionNumber => "ASU",
            Enum1153::ProductDataFileNumber => "ASV",
            Enum1153::CadastroGeralDoContribuinteCgc => "ASW",
            Enum1153::ForeignResidentIdentificationNumber => "ASX",
            Enum1153::CdRom => "ASY",
            Enum1153::PhysicalMedium => "ASZ",
            Enum1153::FinancialCancellationReferenceNumber => "ATA",
            Enum1153::PurchaseForExportCustomsAgreementNumber => "ATB",
            Enum1153::JudgmentNumber => "ATC",
            Enum1153::SecretariatNumber => "ATD",
            Enum1153::PreviousBankingStatusMessageReference => "ATE",
            Enum1153::LastReceivedBankingStatusMessageReference => "ATF",
            Enum1153::BanksDocumentaryProcedureReference => "ATG",
            Enum1153::CustomersDocumentaryProcedureReference => "ATH",
            Enum1153::SafeDepositBoxNumber => "ATI",
            Enum1153::ReceivingBankgiroNumber => "ATJ",
            Enum1153::SendingBankgiroNumber => "ATK",
            Enum1153::BankgiroReference => "ATL",
            Enum1153::GuaranteeNumber => "ATM",
            Enum1153::CollectionInstrumentNumber => "ATN",
            Enum1153::ConvertedPostgiroNumber => "ATO",
            Enum1153::CostCentreAlignmentNumber => "ATP",
            Enum1153::KamerVanKoophandelKvkNumber => "ATQ",
            Enum1153::InstitutBelgoLuxembourgeoisDeCodificationIblcNumber => "ATR",
            Enum1153::ExternalObjectReference => "ATS",
            Enum1153::ExceptionalTransportAuthorisationNumber => "ATT",
            Enum1153::ClaveUnicaDeIdentificacionTributariaCuit => "ATU",
            Enum1153::RegistroUnicoTributarioRut => "ATV",
            Enum1153::FlatRackContainerBundleIdentificationNumber => "ATW",
            Enum1153::TransportEquipmentAcceptanceOrderReference => "ATX",
            Enum1153::TransportEquipmentReleaseOrderReference => "ATY",
            Enum1153::ShipsStayReferenceNumber => "ATZ",
            Enum1153::AuthorizationToMeetCompetitionNumber => "AU",
            Enum1153::PlacePositioningReference => "AUA",
            Enum1153::PartyReference => "AUB",
            Enum1153::IssuedPrescriptionIdentification => "AUC",
            Enum1153::CollectionReference => "AUD",
            Enum1153::TravelService => "AUE",
            Enum1153::ConsignmentStockContract => "AUF",
            Enum1153::ImportersLetterCreditReference => "AUG",
            Enum1153::PerformedPrescriptionIdentification => "AUH",
            Enum1153::ImageReference => "AUI",
            Enum1153::ProposedPurchaseOrderReferenceNumber => "AUJ",
            Enum1153::ApplicationForFinancialSupportReferenceNumber => "AUK",
            Enum1153::ManufacturingQualityAgreementNumber => "AUL",
            Enum1153::SoftwareEditorReference => "AUM",
            Enum1153::SoftwareReference => "AUN",
            Enum1153::SoftwareQualityReference => "AUO",
            Enum1153::ConsolidatedOrdersReference => "AUP",
            Enum1153::CustomsBindingRulingNumber => "AUQ",
            Enum1153::CustomsNonBindingRulingNumber => "AUR",
            Enum1153::DeliveryRouteReference => "AUS",
            Enum1153::NetAreaSupplierReference => "AUT",
            Enum1153::TimeSeriesReference => "AUU",
            Enum1153::ConnectingPointToCentralGrid => "AUV",
            Enum1153::MarketingPlanIdentificationNumberMpin => "AUW",
            Enum1153::EntityReferenceNumberPrevious => "AUX",
            Enum1153::InternationalStandardIndustrialClassificationIsic => "AUY",
            Enum1153::CustomsPreApprovalRulingNumber => "AUZ",
            Enum1153::AccountPayableNumber => "AV",
            Enum1153::FirstFinancialInstitutionsTransactionReference => "AVA",
            Enum1153::ProductCharacteristicsDirectory => "AVB",
            Enum1153::SuppliersCustomerReferenceNumber => "AVC",
            Enum1153::InventoryReportRequestNumber => "AVD",
            Enum1153::MeteringPoint => "AVE",
            Enum1153::PassengerReservationNumber => "AVF",
            Enum1153::SlaughterhouseApprovalNumber => "AVG",
            Enum1153::MeatCuttingPlantApprovalNumber => "AVH",
            Enum1153::CustomerTravelServiceIdentifier => "AVI",
            Enum1153::ExportControlClassificationNumber => "AVJ",
            Enum1153::BrokerReference3 => "AVK",
            Enum1153::ConsignmentInformation => "AVL",
            Enum1153::GoodsItemInformation => "AVM",
            Enum1153::DangerousGoodsInformation => "AVN",
            Enum1153::PilotageServicesExemptionNumber => "AVO",
            Enum1153::PersonRegistrationNumber => "AVP",
            Enum1153::PlacePackingApprovalNumber => "AVQ",
            Enum1153::OriginalMandateReference => "AVR",
            Enum1153::MandateReference => "AVS",
            Enum1153::ReservationStationIndentifier => "AVT",
            Enum1153::UniqueGoodsShipmentIdentifier => "AVU",
            Enum1153::FrameworkAgreementNumber => "AVV",
            Enum1153::HashValue => "AVW",
            Enum1153::MovementReferenceNumber => "AVX",
            Enum1153::EconomicOperatorsRegistrationAndIdentificationNumber => "AVY",
            Enum1153::LocalReferenceNumber => "AVZ",
            Enum1153::RateCodeNumber => "AWA",
            Enum1153::AirWaybillNumber => "AWB",
            Enum1153::DocumentaryCreditAmendmentNumber => "AWC",
            Enum1153::AdvisingBanksReference => "AWD",
            Enum1153::CostCentre => "AWE",
            Enum1153::WorkItemQuantityDetermination => "AWF",
            Enum1153::InternalDataProcessNumber => "AWG",
            Enum1153::CategoryWorkReference => "AWH",
            Enum1153::PolicyFormNumber => "AWI",
            Enum1153::NetArea => "AWJ",
            Enum1153::ServiceProvider => "AWK",
            Enum1153::ErrorPosition => "AWL",
            Enum1153::ServiceCategoryReference => "AWM",
            Enum1153::ConnectedLocation => "AWN",
            Enum1153::RelatedParty => "AWO",
            Enum1153::LatestAccountingEntryRecordReference => "AWP",
            Enum1153::AccountingEntry => "AWQ",
            Enum1153::DocumentReferenceOriginal => "AWR",
            Enum1153::HygienicCertificateNumberNational => "AWS",
            Enum1153::AdministrativeReferenceCode => "AWT",
            Enum1153::PickUpSheetNumber => "AWU",
            Enum1153::PhoneNumber => "AWV",
            Enum1153::BuyersFundNumber => "AWW",
            Enum1153::CompanyTradingAccountNumber => "AWX",
            Enum1153::ReservedGoodsIdentifier => "AWY",
            Enum1153::HandlingAndMovementReferenceNumber => "AWZ",
            Enum1153::InstructionToDespatchReferenceNumber => "AXA",
            Enum1153::InstructionForReturnsNumber => "AXB",
            Enum1153::MeteredServicesConsumptionReportNumber => "AXC",
            Enum1153::OrderStatusEnquiryNumber => "AXD",
            Enum1153::FirmBookingReferenceNumber => "AXE",
            Enum1153::ProductInquiryNumber => "AXF",
            Enum1153::SplitDeliveryNumber => "AXG",
            Enum1153::ServiceRelationNumber => "AXH",
            Enum1153::SerialShippingContainerCode => "AXI",
            Enum1153::TestSpecificationNumber => "AXJ",
            Enum1153::TransportStatusReportNumber => "AXK",
            Enum1153::ToolingContractNumber => "AXL",
            Enum1153::FormulaReferenceNumber => "AXM",
            Enum1153::PreAgreementNumber => "AXN",
            Enum1153::ProductCertificationNumber => "AXO",
            Enum1153::ConsignmentContractNumber => "AXP",
            Enum1153::ProductSpecificationReferenceNumber => "AXQ",
            Enum1153::PayrollDeductionAdviceReference => "AXR",
            Enum1153::TracesPartyIdentification => "AXS",
            Enum1153::BlockStowageReference => "AXU",
            Enum1153::BeginningMeterReadingActual => "BA",
            Enum1153::BuyersContractNumber => "BC",
            Enum1153::BidNumber => "BD",
            Enum1153::BeginningMeterReadingEstimated => "BE",
            Enum1153::HouseBillLadingNumber => "BH",
            Enum1153::BillLadingNumber => "BM",
            Enum1153::ConsignmentIdentifierCarrierAssigned => "BN",
            Enum1153::BlanketOrderNumber => "BO",
            Enum1153::BrokerOrSalesOfficeNumber => "BR",
            Enum1153::BatchNumberLotNumber => "BT",
            Enum1153::BatteryAndAccumulatorProducerRegistrationNumber => "BTP",
            Enum1153::BlendedWithNumber => "BW",
            Enum1153::IataCargoAgentCassAddressNumber => "CAS",
            Enum1153::MatchingEntriesBalanced => "CAT",
            Enum1153::EntryFlagging => "CAU",
            Enum1153::MatchingEntriesUnbalanced => "CAV",
            Enum1153::DocumentReferenceInternal => "CAW",
            Enum1153::EuropeanValueAddedTaxIdentification => "CAX",
            Enum1153::CostAccountingDocument => "CAY",
            Enum1153::GridOperatorsCustomerReferenceNumber => "CAZ",
            Enum1153::TicketControlNumber => "CBA",
            Enum1153::OrderShipmentGroupingReference => "CBB",
            Enum1153::CreditNoteNumber => "CD",
            Enum1153::CedingCompany => "CEC",
            Enum1153::DebitLetterNumber => "CED",
            Enum1153::ConsigneesFurtherOrder => "CFE",
            Enum1153::AnimalFarmLicenceNumber => "CFF",
            Enum1153::ConsignorsFurtherOrder => "CFO",
            Enum1153::ConsigneesOrderNumber => "CG",
            Enum1153::CustomerCatalogueNumber => "CH",
            Enum1153::ChequeNumber => "CK",
            Enum1153::CheckingNumber => "CKN",
            Enum1153::CreditMemoNumber => "CM",
            Enum1153::RoadConsignmentNoteNumber => "CMR",
            Enum1153::CarriersReferenceNumber => "CN",
            Enum1153::ChargesNoteDocumentAttachmentIndicator => "CNO",
            Enum1153::CallOffOrderNumber => "COF",
            Enum1153::ConditionPurchaseDocumentNumber => "CP",
            Enum1153::CustomerReferenceNumber => "CR",
            Enum1153::TransportMeansJourneyIdentifier => "CRN",
            Enum1153::ConditionSaleDocumentNumber => "CS",
            Enum1153::TeamAssignmentNumber => "CST",
            Enum1153::ContractNumber => "CT",
            Enum1153::ConsignmentIdentifierConsignorAssigned => "CU",
            Enum1153::ContainerOperatorsReferenceNumber => "CV",
            Enum1153::PackageNumber => "CW",
            Enum1153::CooperationContractNumber => "CZ",
            Enum1153::DefermentApprovalNumber => "DA",
            Enum1153::DebitAccountNumber => "DAN",
            Enum1153::BuyersDebtorNumber => "DB",
            Enum1153::DistributorInvoiceNumber => "DI",
            Enum1153::DebitNoteNumber => "DL",
            Enum1153::DocumentIdentifier => "DM",
            Enum1153::DeliveryNoteNumber => "DQ",
            Enum1153::DockReceiptNumber => "DR",
            Enum1153::EndingMeterReadingActual => "EA",
            Enum1153::EmbargoPermitNumber => "EB",
            Enum1153::ExportDeclaration => "ED",
            Enum1153::EndingMeterReadingEstimated => "EE",
            Enum1153::ElectricalAndElectronicEquipmentProducerRegistration => "EEP",
            Enum1153::EmployersIdentificationNumber => "EI",
            Enum1153::EmbargoNumber => "EN",
            Enum1153::EquipmentNumber => "EQ",
            Enum1153::ContainerEquipmentReceiptNumber => "ER",
            Enum1153::ExportersReferenceNumber => "ERN",
            Enum1153::ExcessTransportationNumber => "ET",
            Enum1153::ExportPermitIdentifier => "EX",
            Enum1153::FiscalNumber => "FC",
            Enum1153::ConsignmentIdentifierFreightForwarderAssigned => "FF",
            Enum1153::FileLineIdentifier => "FI",
            Enum1153::FlowReferenceNumber => "FLW",
            Enum1153::FreightBillNumber => "FN",
            Enum1153::ForeignExchange => "FO",
            Enum1153::FinalSequenceNumber => "FS",
            Enum1153::FreeZoneIdentifier => "FT",
            Enum1153::FileVersionNumber => "FV",
            Enum1153::ForeignExchangeContractNumber => "FX",
            Enum1153::StandardsNumber => "GA",
            Enum1153::GovernmentContractNumber => "GC",
            Enum1153::StandardsCodeNumber => "GD",
            Enum1153::GeneralDeclarationNumber => "GDN",
            Enum1153::GovernmentReferenceNumber => "GN",
            Enum1153::HarmonisedSystemNumber => "HS",
            Enum1153::HouseWaybillNumber => "HWB",
            Enum1153::InternalVendorNumber => "IA",
            Enum1153::InBondNumber => "IB",
            Enum1153::IataCargoAgentCodeNumber => "ICA",
            Enum1153::InsuranceCertificateReferenceNumber => "ICE",
            Enum1153::InsuranceContractReferenceNumber => "ICO",
            Enum1153::InitialSampleInspectionReportNumber => "II",
            Enum1153::InternalOrderNumber => "IL",
            Enum1153::IntermediaryBroker => "INB",
            Enum1153::InterchangeNumberNew => "INN",
            Enum1153::InterchangeNumberOld => "INO",
            Enum1153::ImportPermitIdentifier => "IP",
            Enum1153::InvoiceNumberSuffix => "IS",
            Enum1153::InternalCustomerNumber => "IT",
            Enum1153::InvoiceDocumentIdentifier => "IV",
            Enum1153::JobNumber => "JB",
            Enum1153::EndingJobSequenceNumber => "JE",
            Enum1153::ShippingLabelSerialNumber => "LA",
            Enum1153::LoadingAuthorisationIdentifier => "LAN",
            Enum1153::LowerNumberInRange => "LAR",
            Enum1153::Lockbox => "LB",
            Enum1153::LetterCreditNumber => "LC",
            Enum1153::DocumentLineIdentifier => "LI",
            Enum1153::LoadPlanningNumber => "LO",
            Enum1153::ReservationOfficeIdentifier => "LRC",
            Enum1153::BarCodedLabelSerialNumber => "LS",
            Enum1153::ShipNoticeManifestNumber => "MA",
            Enum1153::MasterBillLadingNumber => "MB",
            Enum1153::ManufacturersPartNumber => "MF",
            Enum1153::MeterUnitNumber => "MG",
            Enum1153::ManufacturingOrderNumber => "MH",
            Enum1153::MessageRecipient => "MR",
            Enum1153::MailingReferenceNumber => "MRN",
            Enum1153::MessageSender => "MS",
            Enum1153::ManufacturersMaterialSafetyDataSheetNumber => "MSS",
            Enum1153::MasterAirWaybillNumber => "MWB",
            Enum1153::NorthAmericanHazardousGoodsClassificationNumber => "NA",
            Enum1153::NotaFiscal => "NF",
            Enum1153::CurrentInvoiceNumber => "OH",
            Enum1153::PreviousInvoiceNumber => "OI",
            Enum1153::OrderDocumentIdentifierBuyerAssigned => "ON",
            Enum1153::OriginalPurchaseOrder => "OP",
            Enum1153::GeneralOrderNumber => "OR",
            Enum1153::PayersFinancialInstitutionAccountNumber => "PB",
            Enum1153::ProductionCode => "PC",
            Enum1153::PromotionDealNumber => "PD",
            Enum1153::PlantNumber => "PE",
            Enum1153::PrimeContractorContractNumber => "PF",
            Enum1153::PriceListVersionNumber => "PI",
            Enum1153::PackingListNumber => "PK",
            Enum1153::PriceListNumber => "PL",
            Enum1153::PurchaseOrderResponseNumber => "POR",
            Enum1153::PurchaseOrderChangeNumber => "PP",
            Enum1153::PaymentReference => "PQ",
            Enum1153::PriceQuoteNumber => "PR",
            Enum1153::PurchaseOrderNumberSuffix => "PS",
            Enum1153::PriorPurchaseOrderNumber => "PW",
            Enum1153::PayeesFinancialInstitutionAccountNumber => "PY",
            Enum1153::RemittanceAdviceNumber => "RA",
            Enum1153::RailRoadRoutingCode => "RC",
            Enum1153::RailwayConsignmentNoteNumber => "RCN",
            Enum1153::ReleaseNumber => "RE",
            Enum1153::ConsignmentReceiptIdentifier => "REN",
            Enum1153::ExportReferenceNumber => "RF",
            Enum1153::PayersFinancialInstitutionTransitRoutingNoAch => "RR",
            Enum1153::PayeesFinancialInstitutionTransitRoutingNo => "RT",
            Enum1153::SalesPersonNumber => "SA",
            Enum1153::SalesRegionNumber => "SB",
            Enum1153::SalesDepartmentNumber => "SD",
            Enum1153::SerialNumber => "SE",
            Enum1153::AllocatedSeat => "SEA",
            Enum1153::ShipFrom => "SF",
            Enum1153::PreviousHighestScheduleNumber => "SH",
            Enum1153::SidShippersIdentifyingNumberForShipment => "SI",
            Enum1153::SalesOfficeNumber => "SM",
            Enum1153::TransportEquipmentSealIdentifier => "SN",
            Enum1153::ScanLine => "SP",
            Enum1153::EquipmentSequenceNumber => "SQ",
            Enum1153::ShipmentReferenceNumber => "SRN",
            Enum1153::SellersReferenceNumber => "SS",
            Enum1153::StationReferenceNumber => "STA",
            Enum1153::SwapOrderNumber => "SW",
            Enum1153::SpecificationNumber => "SZ",
            Enum1153::TruckersBillLading => "TB",
            Enum1153::TerminalOperatorsConsignmentReference => "TCR",
            Enum1153::TelexMessageNumber => "TE",
            Enum1153::TransferNumber => "TF",
            Enum1153::TirCarnetNumber => "TI",
            Enum1153::TransportInstructionNumber => "TIN",
            Enum1153::TaxExemptionLicenceNumber => "TL",
            Enum1153::TransactionReferenceNumber => "TN",
            Enum1153::TestReportNumber => "TP",
            Enum1153::UpperNumberRange => "UAR",
            Enum1153::UltimateCustomersReferenceNumber => "UC",
            Enum1153::UniqueConsignmentReferenceNumber => "UCN",
            Enum1153::UnitedNationsDangerousGoodsIdentifier => "UN",
            Enum1153::UltimateCustomersOrderNumber => "UO",
            Enum1153::UniformResourceIdentifier => "URI",
            Enum1153::VatRegistrationNumber => "VA",
            Enum1153::VendorContractNumber => "VC",
            Enum1153::TransportEquipmentGrossMassVerificationReference => "VGR",
            Enum1153::VesselIdentifier => "VM",
            Enum1153::OrderNumberVendor => "VN",
            Enum1153::VoyageNumber => "VON",
            Enum1153::TransportEquipmentGrossMassVerificationOrderReference => "VOR",
            Enum1153::VendorProductNumber => "VP",
            Enum1153::VendorIdNumber => "VR",
            Enum1153::VendorOrderNumberSuffix => "VS",
            Enum1153::MotorVehicleIdentificationNumber => "VT",
            Enum1153::VoucherNumber => "VV",
            Enum1153::WarehouseEntryNumber => "WE",
            Enum1153::WeightAgreementNumber => "WM",
            Enum1153::WellNumber => "WN",
            Enum1153::WarehouseReceiptNumber => "WR",
            Enum1153::WarehouseStorageLocationNumber => "WS",
            Enum1153::RailWaybillNumber => "WY",
            Enum1153::CompanyPlaceRegistrationNumber => "XA",
            Enum1153::CargoControlNumber => "XC",
            Enum1153::PreviousCargoControlNumber => "XP",
            Enum1153::MutuallyDefinedReferenceNumber => "ZZZ",
        }
    }
}

impl crate::Description for Enum1153 {
    fn description(self) -> &'static str {
        match self {
            Enum1153::OrderAcknowledgementDocumentIdentifier => {
                "Order acknowledgement document identifier"
            }
            Enum1153::ProformaInvoiceDocumentIdentifier => "Proforma invoice document identifier",
            Enum1153::DocumentaryCreditIdentifier => "Documentary credit identifier",
            Enum1153::ContractDocumentAddendumIdentifier => "Contract document addendum identifier",
            Enum1153::GoodsDeclarationNumber => "Goods declaration number",
            Enum1153::DebitCardNumber => "Debit card number",
            Enum1153::OfferNumber => "Offer number",
            Enum1153::BanksBatchInterbankTransactionReferenceNumber => {
                "Bank's batch interbank transaction reference number"
            }
            Enum1153::BanksIndividualInterbankTransactionReferenceNumber => {
                "Bank's individual interbank transaction reference number"
            }
            Enum1153::DeliveryOrderNumber => "Delivery order number",
            Enum1153::DespatchAdviceNumber => "Despatch advice number",
            Enum1153::DrawingNumber => "Drawing number",
            Enum1153::WaybillNumber => "Waybill number",
            Enum1153::DeliveryScheduleNumber => "Delivery schedule number",
            Enum1153::ConsignmentIdentifierConsigneeAssigned => {
                "Consignment identifier, consignee assigned"
            }
            Enum1153::PartialShipmentIdentifier => "Partial shipment identifier",
            Enum1153::TransportEquipmentIdentifier => "Transport equipment identifier",
            Enum1153::MunicipalityAssignedBusinessRegistryNumber => {
                "Municipality assigned business registry number"
            }
            Enum1153::TransportContractDocumentIdentifier => {
                "Transport contract document identifier"
            }
            Enum1153::MasterLabelNumber => "Master label number",
            Enum1153::DespatchNoteDocumentIdentifier => "Despatch note document identifier",
            Enum1153::EnquiryNumber => "Enquiry number",
            Enum1153::DocketNumber => "Docket number",
            Enum1153::CivilActionNumber => "Civil action number",
            Enum1153::CarriersAgentReferenceNumber => "Carrier's agent reference number",
            Enum1153::StandardCarrierAlphaCodeScacNumber => {
                "Standard Carrier Alpha Code (SCAC) number"
            }
            Enum1153::CustomsValuationDecisionNumber => "Customs valuation decision number",
            Enum1153::EndUseAuthorizationNumber => "End use authorization number",
            Enum1153::AntiDumpingCaseNumber => "Anti-dumping case number",
            Enum1153::CustomsTariffNumber => "Customs tariff number",
            Enum1153::DeclarantsReferenceNumber => "Declarant's reference number",
            Enum1153::RepairEstimateNumber => "Repair estimate number",
            Enum1153::CustomsDecisionRequestNumber => "Customs decision request number",
            Enum1153::SubHouseBillLadingNumber => "Sub-house bill of lading number",
            Enum1153::TaxPaymentIdentifier => "Tax payment identifier",
            Enum1153::QuotaNumber => "Quota number",
            Enum1153::TransitOnwardCarriageGuaranteeBondNumber => {
                "Transit (onward carriage) guarantee (bond) number"
            }
            Enum1153::CustomsGuaranteeNumber => "Customs guarantee number",
            Enum1153::ReplacingPartNumber => "Replacing part number",
            Enum1153::SellersCatalogueNumber => "Seller's catalogue number",
            Enum1153::OriginatorsReference => "Originator's reference",
            Enum1153::DeclarantsCustomsIdentityNumber => "Declarant's Customs identity number",
            Enum1153::ImporterReferenceNumber => "Importer reference number",
            Enum1153::ExportClearanceInstructionReferenceNumber => {
                "Export clearance instruction reference number"
            }
            Enum1153::ImportClearanceInstructionReferenceNumber => {
                "Import clearance instruction reference number"
            }
            Enum1153::GoodsDeclarationDocumentIdentifierCustoms => {
                "Goods declaration document identifier, Customs"
            }
            Enum1153::ArticleNumber => "Article number",
            Enum1153::IntraPlantRouting => "Intra-plant routing",
            Enum1153::StockKeepingUnitNumber => "Stock keeping unit number",
            Enum1153::TextElementIdentifierDeletionReference => {
                "Text Element Identifier deletion reference"
            }
            Enum1153::AllotmentIdentificationAir => "Allotment identification (Air)",
            Enum1153::VehicleLicenceNumber => "Vehicle licence number",
            Enum1153::AirCargoTransferManifest => " Air cargo transfer manifest",
            Enum1153::CargoAcceptanceOrderReferenceNumber => {
                "Cargo acceptance order reference number"
            }
            Enum1153::UsGovernmentAgencyNumber => "US government agency number",
            Enum1153::ShippingUnitIdentification => "Shipping unit identification",
            Enum1153::AdditionalReferenceNumber => "Additional reference number",
            Enum1153::RelatedDocumentNumber => "Related document number",
            Enum1153::AddresseeReference => "Addressee reference",
            Enum1153::AtaCarnetNumber => "ATA carnet number",
            Enum1153::PackagingUnitIdentification => "Packaging unit identification",
            Enum1153::OuterpackagingUnitIdentification => "Outerpackaging unit identification",
            Enum1153::CustomerMaterialSpecificationNumber => {
                "Customer material specification number"
            }
            Enum1153::BankReference => "Bank reference",
            Enum1153::PrincipalReferenceNumber => "Principal reference number",
            Enum1153::CollectionAdviceDocumentIdentifier => "Collection advice document identifier",
            Enum1153::IronChargeNumber => "Iron charge number",
            Enum1153::HotRollNumber => "Hot roll number",
            Enum1153::ColdRollNumber => "Cold roll number",
            Enum1153::RailwayWagonNumber => "Railway wagon number",
            Enum1153::UniqueClaimsReferenceNumberSender => {
                "Unique claims reference number of the sender"
            }
            Enum1153::LossEventNumber => "Loss/event number",
            Enum1153::EstimateOrderReferenceNumber => "Estimate order reference number",
            Enum1153::ReferenceNumberToPreviousMessage => "Reference number to previous message",
            Enum1153::BankersAcceptance => "Banker's acceptance",
            Enum1153::DutyMemoNumber => "Duty memo number",
            Enum1153::EquipmentTransportChargeNumber => "Equipment transport charge number",
            Enum1153::BuyersItemNumber => "Buyer's item number",
            Enum1153::MaturedCertificateDeposit => "Matured certificate of deposit",
            Enum1153::Loan => "Loan",
            Enum1153::AnalysisNumberTestNumber => "Analysis number/test number",
            Enum1153::AccountNumber => "Account number",
            Enum1153::TreatyNumber => "Treaty number",
            Enum1153::CatastropheNumber => "Catastrophe number",
            Enum1153::BureauSigningStatementReference => "Bureau signing (statement reference)",
            Enum1153::CompanySyndicateReference1 => "Company / syndicate reference 1",
            Enum1153::CompanySyndicateReference2 => "Company / syndicate reference 2",
            Enum1153::OrderingCustomerConsignmentReferenceNumber => {
                "Ordering customer consignment reference number"
            }
            Enum1153::ShipownersAuthorizationNumber => "Shipowner's authorization number",
            Enum1153::InlandTransportOrderNumber => "Inland transport order number",
            Enum1153::ContainerWorkOrderReferenceNumber => "Container work order reference number",
            Enum1153::StatementNumber => "Statement number",
            Enum1153::UniqueMarketReference => "Unique market reference",
            Enum1153::GroupAccounting => "Group accounting",
            Enum1153::BrokerReference1 => "Broker reference 1",
            Enum1153::BrokerReference2 => "Broker reference 2",
            Enum1153::LloydsClaimsOfficeReference => "Lloyd's claims office reference",
            Enum1153::SecureDeliveryTermsAndConditionsAgreementReference => {
                "Secure delivery terms and conditions agreement reference"
            }
            Enum1153::ReportNumber => "Report number",
            Enum1153::TraderAccountNumber => "Trader account number",
            Enum1153::AuthorizationForExpenseAfeNumber => " Authorization for expense (AFE) number",
            Enum1153::GovernmentAgencyReferenceNumber => "Government agency reference number",
            Enum1153::AssemblyNumber => "Assembly number",
            Enum1153::SymbolNumber => "Symbol number",
            Enum1153::CommodityNumber => "Commodity number",
            Enum1153::Eur1CertificateNumber => "Eur 1 certificate number",
            Enum1153::CustomerProcessSpecificationNumber => "Customer process specification number",
            Enum1153::CustomerSpecificationNumber => "Customer specification number",
            Enum1153::ApplicableInstructionsOrStandards => "Applicable instructions or standards",
            Enum1153::RegistrationNumberPreviousCustomsDeclaration => {
                "Registration number of previous Customs declaration"
            }
            Enum1153::PostEntryReference => "Post-entry reference",
            Enum1153::PaymentOrderNumber => "Payment order number",
            Enum1153::DeliveryNumberTransport => "Delivery number (transport)",
            Enum1153::TransportRoute => "Transport route",
            Enum1153::CustomersUnitInventoryNumber => "Customer's unit inventory number",
            Enum1153::ProductReservationNumber => "Product reservation number",
            Enum1153::ProjectNumber => "Project number",
            Enum1153::DrawingListNumber => "Drawing list number",
            Enum1153::ProjectSpecificationNumber => "Project specification number",
            Enum1153::PrimaryReference => "Primary reference",
            Enum1153::RequestForCancellationNumber => "Request for cancellation number",
            Enum1153::SuppliersControlNumber => "Supplier's control number",
            Enum1153::ShippingNoteNumber => "Shipping note number",
            Enum1153::EmptyContainerBillNumber => "Empty container bill number",
            Enum1153::NonNegotiableMaritimeTransportDocumentNumber => {
                "Non-negotiable maritime transport document number"
            }
            Enum1153::SubstituteAirWaybillNumber => "Substitute air waybill number",
            Enum1153::DespatchNotePostParcelsNumber => "Despatch note (post parcels) number",
            Enum1153::AirlinesFlightIdentificationNumber => {
                " Airlines flight identification number"
            }
            Enum1153::ThroughBillLadingNumber => "Through bill of lading number",
            Enum1153::CargoManifestNumber => "Cargo manifest number",
            Enum1153::BordereauNumber => "Bordereau number",
            Enum1153::CustomsItemNumber => "Customs item number",
            Enum1153::ExportControlCommodityNumberEccn => "Export Control Commodity number (ECCN)",
            Enum1153::MarkingLabelReference => "Marking/label reference",
            Enum1153::TariffNumber => "Tariff number",
            Enum1153::ReplenishmentPurchaseOrderNumber => "Replenishment purchase order number",
            Enum1153::ImmediateTransportationNoForInBondMovement => {
                "Immediate transportation no. for in bond movement"
            }
            Enum1153::TransportationExportationNoForInBondMovement => {
                "Transportation exportation no. for in bond movement"
            }
            Enum1153::ImmediateExportationNoForInBondMovement => {
                "Immediate exportation no. for in bond movement"
            }
            Enum1153::AssociatedInvoices => "Associated invoices",
            Enum1153::SecondaryCustomsReference => "Secondary Customs reference",
            Enum1153::AccountPartysReference => "Account party's reference",
            Enum1153::BeneficiarysReference => "Beneficiary's reference",
            Enum1153::SecondBeneficiarysReference => "Second beneficiary's reference",
            Enum1153::ApplicantsBankReference => "Applicant's bank reference",
            Enum1153::IssuingBanksReference => "Issuing bank's reference",
            Enum1153::BeneficiarysBankReference => "Beneficiary's bank reference",
            Enum1153::DirectPaymentValuationNumber => "Direct payment valuation number",
            Enum1153::DirectPaymentValuationRequestNumber => {
                "Direct payment valuation request number"
            }
            Enum1153::QuantityValuationNumber => "Quantity valuation number",
            Enum1153::QuantityValuationRequestNumber => "Quantity valuation request number",
            Enum1153::BillQuantitiesNumber => "Bill of quantities number",
            Enum1153::PaymentValuationNumber => "Payment valuation number",
            Enum1153::SituationNumber => "Situation number",
            Enum1153::AgreementToPayNumber => "Agreement to pay number",
            Enum1153::ContractPartyReferenceNumber => "Contract party reference number",
            Enum1153::AccountPartysBankReference => "Account party's bank reference",
            Enum1153::AgentsBankReference => "Agent's bank reference",
            Enum1153::AgentsReference => "Agent's reference",
            Enum1153::ApplicantsReference => "Applicant's reference",
            Enum1153::DisputeNumber => "Dispute number",
            Enum1153::CreditRatingAgencysReferenceNumber => {
                "Credit rating agency's reference number"
            }
            Enum1153::RequestNumber => "Request number",
            Enum1153::SingleTransactionSequenceNumber => "Single transaction sequence number",
            Enum1153::ApplicationReferenceNumber => "Application reference number",
            Enum1153::DeliveryVerificationCertificate => "Delivery verification certificate",
            Enum1153::NumberTemporaryImportationDocument => {
                "Number of temporary importation document"
            }
            Enum1153::ReferenceNumberQuotedOnStatement => "Reference number quoted on statement",
            Enum1153::SendersReferenceToOriginalMessage => {
                "Sender's reference to the original message"
            }
            Enum1153::CompanyIssuedEquipmentId => "Company issued equipment ID",
            Enum1153::DomesticFlightNumber => "Domestic flight number",
            Enum1153::InternationalFlightNumber => "International flight number",
            Enum1153::EmployerIdentificationNumberServiceBureau => {
                "Employer identification number of service bureau"
            }
            Enum1153::ServiceGroupIdentificationNumber => "Service group identification number",
            Enum1153::MemberNumber => "Member number",
            Enum1153::PreviousMemberNumber => "Previous member number",
            Enum1153::SchemePlanNumber => "Scheme/plan number",
            Enum1153::PreviousSchemePlanNumber => "Previous scheme/plan number",
            Enum1153::ReceivingPartysMemberIdentification => {
                "Receiving party's member identification"
            }
            Enum1153::PayrollNumber => "Payroll number",
            Enum1153::PackagingSpecificationNumber => "Packaging specification number",
            Enum1153::AuthorityIssuedEquipmentIdentification => {
                "Authority issued equipment identification"
            }
            Enum1153::TrainingFlightNumber => "Training flight number",
            Enum1153::FundCodeNumber => "Fund code number",
            Enum1153::SignalCodeNumber => "Signal code number",
            Enum1153::MajorForceProgramNumber => "Major force program number",
            Enum1153::NominationNumber => "Nomination number",
            Enum1153::LaboratoryRegistrationNumber => "Laboratory registration number",
            Enum1153::TransportContractReferenceNumber => "Transport contract reference number",
            Enum1153::PayeesReferenceNumber => "Payee's reference number",
            Enum1153::PayersReferenceNumber => "Payer's reference number",
            Enum1153::CreditorsReferenceNumber => "Creditor's reference number",
            Enum1153::DebtorsReferenceNumber => "Debtor's reference number",
            Enum1153::JointVentureReferenceNumber => "Joint venture reference number",
            Enum1153::ChamberCommerceRegistrationNumber => {
                "Chamber of Commerce registration number"
            }
            Enum1153::TaxRegistrationNumber => "Tax registration number",
            Enum1153::WoolIdentificationNumber => "Wool identification number",
            Enum1153::WoolTaxReferenceNumber => "Wool tax reference number",
            Enum1153::MeatProcessingEstablishmentRegistrationNumber => {
                "Meat processing establishment registration number"
            }
            Enum1153::QuarantineTreatmentStatusReferenceNumber => {
                "Quarantine/treatment status reference number"
            }
            Enum1153::RequestForQuoteNumber => "Request for quote number",
            Enum1153::ManualProcessingAuthorityNumber => "Manual processing authority number",
            Enum1153::RateNoteNumber => "Rate note number",
            Enum1153::FreightForwarderNumber => "Freight Forwarder number",
            Enum1153::CustomsReleaseCode => "Customs release code",
            Enum1153::ComplianceCodeNumber => "Compliance code number",
            Enum1153::DepartmentTransportationBondNumber => {
                "Department of transportation bond number"
            }
            Enum1153::ExportEstablishmentNumber => "Export establishment number",
            Enum1153::CertificateConformity => "Certificate of conformity",
            Enum1153::MinisterialCertificateHomologation => {
                "Ministerial certificate of homologation"
            }
            Enum1153::PreviousDeliveryInstructionNumber => "Previous delivery instruction number",
            Enum1153::PassportNumber => "Passport number",
            Enum1153::CommonTransactionReferenceNumber => "Common transaction reference number",
            Enum1153::BanksCommonTransactionReferenceNumber => {
                "Bank's common transaction reference number"
            }
            Enum1153::CustomersIndividualTransactionReferenceNumber => {
                "Customer's individual transaction reference number"
            }
            Enum1153::BanksIndividualTransactionReferenceNumber => {
                "Bank's individual transaction reference number"
            }
            Enum1153::CustomersCommonTransactionReferenceNumber => {
                "Customer's common transaction reference number"
            }
            Enum1153::IndividualTransactionReferenceNumber => {
                "Individual transaction reference number"
            }
            Enum1153::ProductSourcingAgreementNumber => "Product sourcing agreement number",
            Enum1153::CustomsTranshipmentNumber => "Customs transhipment number",
            Enum1153::CustomsPreferenceInquiryNumber => "Customs preference inquiry number",
            Enum1153::PackingPlantNumber => "Packing plant number",
            Enum1153::OriginalCertificateNumber => "Original certificate number",
            Enum1153::ProcessingPlantNumber => "Processing plant number",
            Enum1153::SlaughterPlantNumber => "Slaughter plant number",
            Enum1153::ChargeCardAccountNumber => "Charge card account number",
            Enum1153::EventReferenceNumber => "Event reference number",
            Enum1153::TransportSectionReferenceNumber => "Transport section reference number",
            Enum1153::ReferredProductForMechanicalAnalysis => {
                "Referred product for mechanical analysis"
            }
            Enum1153::ReferredProductForChemicalAnalysis => {
                "Referred product for chemical analysis"
            }
            Enum1153::ConsolidatedInvoiceNumber => "Consolidated invoice number",
            Enum1153::PartReferenceIndicatorInADrawing => "Part reference indicator in a drawing",
            Enum1153::USCodeFederalRegulationsCfr => "U.S. Code of Federal Regulations (CFR)",
            Enum1153::PurchasingActivityClauseNumber => "Purchasing activity clause number",
            Enum1153::USDefenseFederalAcquisitionRegulationSupplement => {
                "U.S. Defense Federal Acquisition Regulation Supplement"
            }
            Enum1153::AgencyClauseNumber => "Agency clause number",
            Enum1153::CircularPublicationNumber => "Circular publication number",
            Enum1153::USFederalAcquisitionRegulation => "U.S. Federal Acquisition Regulation",
            Enum1153::USGeneralServicesAdministrationRegulation => {
                "U.S. General Services Administration Regulation"
            }
            Enum1153::USFederalInformationResourcesManagementRegulation => {
                "U.S. Federal Information Resources Management Regulation"
            }
            Enum1153::Paragraph => "Paragraph",
            Enum1153::SpecialInstructionsNumber => "Special instructions number",
            Enum1153::SiteSpecificProceduresTermsAndConditionsNumber => {
                "Site specific procedures, terms, and conditions number"
            }
            Enum1153::MasterSolicitationProceduresTermsAndConditions => {
                "Master solicitation procedures, terms, and conditions"
            }
            Enum1153::USDepartmentVeteransAffairsAcquisitionRegulation => {
                "U.S. Department of Veterans Affairs Acquisition Regulation"
            }
            Enum1153::MilitaryInterdepartmentalPurchaseRequestMiprNumber => {
                "Military Interdepartmental Purchase Request (MIPR) number"
            }
            Enum1153::ForeignMilitarySalesNumber => "Foreign military sales number",
            Enum1153::DefensePrioritiesAllocationSystemPriorityRating => {
                "Defense priorities allocation system priority rating"
            }
            Enum1153::WageDeterminationNumber => "Wage determination number",
            Enum1153::AgreementNumber => "Agreement number",
            Enum1153::StandardIndustryClassificationSicNumber => {
                "Standard Industry Classification (SIC) number"
            }
            Enum1153::EndItemNumber => "End item number",
            Enum1153::FederalSupplyScheduleItemNumber => "Federal supply schedule item number",
            Enum1153::TechnicalDocumentNumber => "Technical document number",
            Enum1153::TechnicalOrderNumber => "Technical order number",
            Enum1153::Suffix => "Suffix",
            Enum1153::TransportationAccountNumber => "Transportation account number",
            Enum1153::ContainerDispositionOrderReferenceNumber => {
                "Container disposition order reference number"
            }
            Enum1153::ContainerPrefix => "Container prefix",
            Enum1153::TransportEquipmentReturnReference => "Transport equipment return reference",
            Enum1153::TransportEquipmentSurveyReference => "Transport equipment survey reference",
            Enum1153::TransportEquipmentSurveyReportNumber => {
                "Transport equipment survey report number"
            }
            Enum1153::TransportEquipmentStuffingOrder => "Transport equipment stuffing order",
            Enum1153::VehicleIdentificationNumberVin => "Vehicle Identification Number (VIN)",
            Enum1153::GovernmentBillLading => "Government bill of lading",
            Enum1153::OrderingCustomersSecondReferenceNumber => {
                "Ordering customer's second reference number"
            }
            Enum1153::DirectDebitReference => "Direct debit reference",
            Enum1153::MeterReadingAtBeginningDelivery => {
                "Meter reading at the beginning of the delivery"
            }
            Enum1153::MeterReadingAtEndDelivery => "Meter reading at the end of delivery",
            Enum1153::ReplenishmentPurchaseOrderRangeStartNumber => {
                "Replenishment purchase order range start number"
            }
            Enum1153::ThirdBanksReference => "Third bank's reference",
            Enum1153::ActionAuthorizationNumber => "Action authorization number",
            Enum1153::AppropriationNumber => "Appropriation number",
            Enum1153::ProductChangeAuthorityNumber => "Product change authority number",
            Enum1153::GeneralCargoConsignmentReferenceNumber => {
                "General cargo consignment reference number"
            }
            Enum1153::CatalogueSequenceNumber => "Catalogue sequence number",
            Enum1153::ForwardingOrderNumber => "Forwarding order number",
            Enum1153::TransportEquipmentSurveyReferenceNumber => {
                "Transport equipment survey reference number"
            }
            Enum1153::LeaseContractReference => "Lease contract reference",
            Enum1153::TransportCostsReferenceNumber => "Transport costs reference number",
            Enum1153::TransportEquipmentStrippingOrder => "Transport equipment stripping order",
            Enum1153::PriorPolicyNumber => "Prior policy number",
            Enum1153::PolicyNumber => "Policy number",
            Enum1153::ProcurementBudgetNumber => "Procurement budget number",
            Enum1153::DomesticInventoryManagementCode => "Domestic inventory management code",
            Enum1153::CustomerReferenceNumberAssignedToPreviousBalance => {
                "Customer reference number assigned to previous balance of"
            }
            Enum1153::PreviousCreditAdviceReferenceNumber => {
                "Previous credit advice reference number"
            }
            Enum1153::ReportingFormNumber => "Reporting form number",
            Enum1153::AuthorizationNumberForExceptionToDangerousGoods => {
                "Authorization number for exception to dangerous goods"
            }
            Enum1153::DangerousGoodsSecurityNumber => "Dangerous goods security number",
            Enum1153::DangerousGoodsTransportLicenceNumber => {
                "Dangerous goods transport licence number"
            }
            Enum1153::PreviousRentalAgreementNumber => "Previous rental agreement number",
            Enum1153::NextRentalAgreementReasonNumber => "Next rental agreement reason number",
            Enum1153::ConsigneesInvoiceNumber => "Consignee's invoice number",
            Enum1153::MessageBatchNumber => "Message batch number",
            Enum1153::PreviousDeliveryScheduleNumber => "Previous delivery schedule number",
            Enum1153::PhysicalInventoryRecountReferenceNumber => {
                "Physical inventory recount reference number"
            }
            Enum1153::ReceivingAdviceNumber => "Receiving advice number",
            Enum1153::ReturnableContainerReferenceNumber => "Returnable container reference number",
            Enum1153::ReturnsNoticeNumber => "Returns notice number",
            Enum1153::SalesForecastNumber => "Sales forecast number",
            Enum1153::SalesReportNumber => "Sales report number",
            Enum1153::PreviousTaxControlNumber => "Previous tax control number",
            Enum1153::AgerdAerospaceGroundEquipmentRequirementDataNumber => {
                "AGERD (Aerospace Ground Equipment Requirement Data) number"
            }
            Enum1153::RegisteredCapitalReference => "Registered capital reference",
            Enum1153::StandardNumberInspectionDocument => "Standard number of inspection document",
            Enum1153::Model => "Model",
            Enum1153::FinancialManagementReference => "Financial management reference",
            Enum1153::NotificationForCollectionNumberNoticol => {
                "NOTIfication for COLlection number (NOTICOL)"
            }
            Enum1153::PreviousRequestForMeteredReadingReferenceNumber => {
                "Previous request for metered reading reference number"
            }
            Enum1153::NextRentalAgreementNumber => "Next rental agreement number",
            Enum1153::ReferenceNumberARequestForMeteredReading => {
                "Reference number of a request for metered reading"
            }
            Enum1153::HasteningNumber => "Hastening number",
            Enum1153::RepairDataRequestNumber => "Repair data request number",
            Enum1153::ConsumptionDataRequestNumber => "Consumption data request number",
            Enum1153::ProfileNumber => "Profile number",
            Enum1153::CaseNumber => "Case number",
            Enum1153::GovernmentQualityAssuranceAndControlLevelNumber => {
                "Government quality assurance and control level Number"
            }
            Enum1153::PaymentPlanReference => "Payment plan reference",
            Enum1153::ReplacedMeterUnitNumber => "Replaced meter unit number",
            Enum1153::ReplenishmentPurchaseOrderRangeEndNumber => {
                "Replenishment purchase order range end number"
            }
            Enum1153::InsurerAssignedReferenceNumber => "Insurer assigned reference number",
            Enum1153::CanadianExciseEntryNumber => "Canadian excise entry number",
            Enum1153::PremiumRateTable => "Premium rate table",
            Enum1153::AdviseThroughBanksReference => "Advise through bank's reference",
            Enum1153::UsDepartmentTransportationBondSuretyCode => {
                "US, Department of Transportation bond surety code"
            }
            Enum1153::UsFoodAndDrugAdministrationEstablishmentIndicator => {
                "US, Food and Drug Administration establishment indicator"
            }
            Enum1153::UsFederalCommunicationsCommissionFccImport => {
                "US, Federal Communications Commission (FCC) import"
            }
            Enum1153::GoodsAndServicesTaxIdentificationNumber => {
                "Goods and Services Tax identification number"
            }
            Enum1153::IntegratedLogisticSupportCrossReferenceNumber => {
                "Integrated logistic support cross reference number"
            }
            Enum1153::DepartmentNumber => "Department number",
            Enum1153::BuyersCatalogueNumber => "Buyer's catalogue number",
            Enum1153::FinancialSettlementPartysReferenceNumber => {
                "Financial settlement party's reference number"
            }
            Enum1153::StandardsVersionNumber => "Standard's version number",
            Enum1153::PipelineNumber => "Pipeline number",
            Enum1153::AccountServicingBanksReferenceNumber => {
                "Account servicing bank's reference number"
            }
            Enum1153::CompletedUnitsPaymentRequestReference => {
                "Completed units payment request reference"
            }
            Enum1153::PaymentInAdvanceRequestReference => "Payment in advance request reference",
            Enum1153::ParentFile => "Parent file",
            Enum1153::SubFile => "Sub file",
            Enum1153::CadFileLayerConvention => "CAD file layer convention",
            Enum1153::TechnicalRegulation => "Technical regulation",
            Enum1153::PlotFile => "Plot file",
            Enum1153::FileConversionJournal => "File conversion journal",
            Enum1153::AuthorizationNumber => "Authorization number",
            Enum1153::ReferenceNumberAssignedByThirdParty => {
                "Reference number assigned by third party"
            }
            Enum1153::DepositReferenceNumber => "Deposit reference number",
            Enum1153::NamedBanksReference => "Named bank's reference",
            Enum1153::DraweesReference => "Drawee's reference",
            Enum1153::CaseNeedPartysReference => "Case of need party's reference",
            Enum1153::CollectingBanksReference => "Collecting bank's reference",
            Enum1153::RemittingBanksReference => "Remitting bank's reference",
            Enum1153::PrincipalsBankReference => "Principal's bank reference",
            Enum1153::PresentingBanksReference => "Presenting bank's reference",
            Enum1153::ConsigneesReference => "Consignee's reference",
            Enum1153::FinancialTransactionReferenceNumber => {
                "Financial transaction reference number"
            }
            Enum1153::CreditReferenceNumber => "Credit reference number",
            Enum1153::ReceivingBanksAuthorizationNumber => "Receiving bank's authorization number",
            Enum1153::ClearingReference => "Clearing reference",
            Enum1153::SendingBanksReferenceNumber => "Sending bank's reference number",
            Enum1153::DocumentaryPaymentReference => "Documentary payment reference",
            Enum1153::AccountingFileReference => "Accounting file reference",
            Enum1153::SendersFileReferenceNumber => "Sender's file reference number",
            Enum1153::ReceiversFileReferenceNumber => "Receiver's file reference number",
            Enum1153::SourceDocumentInternalReference => "Source document internal reference",
            Enum1153::PrincipalsReference => "Principal's reference",
            Enum1153::DebitReferenceNumber => "Debit reference number",
            Enum1153::Calendar => "Calendar",
            Enum1153::WorkShift => "Work shift",
            Enum1153::WorkBreakdownStructure => "Work breakdown structure",
            Enum1153::OrganisationBreakdownStructure => "Organisation breakdown structure",
            Enum1153::WorkTaskChargeNumber => "Work task charge number",
            Enum1153::FunctionalWorkGroup => "Functional work group",
            Enum1153::WorkTeam => "Work team",
            Enum1153::Department => "Department",
            Enum1153::StatementWork => "Statement of work",
            Enum1153::WorkPackage => "Work package",
            Enum1153::PlanningPackage => "Planning package",
            Enum1153::CostAccount => "Cost account",
            Enum1153::WorkOrder => "Work order",
            Enum1153::TransportationControlNumberTcn => "Transportation Control Number (TCN)",
            Enum1153::ConstraintNotation => "Constraint notation",
            Enum1153::EtermsReference => "ETERMS reference",
            Enum1153::ImplementationVersionNumber => "Implementation version number",
            Enum1153::AccountsReceivableNumber => " Accounts receivable number",
            Enum1153::IncorporatedLegalReference => "Incorporated legal reference",
            Enum1153::PaymentInstalmentReferenceNumber => "Payment instalment reference number",
            Enum1153::EquipmentOwnerReferenceNumber => "Equipment owner reference number",
            Enum1153::CedentsClaimNumber => "Cedent's claim number",
            Enum1153::ReinsurersClaimNumber => "Reinsurer's claim number",
            Enum1153::PriceSalesCatalogueResponseReferenceNumber => {
                "Price/sales catalogue response reference number"
            }
            Enum1153::GeneralPurposeMessageReferenceNumber => {
                "General purpose message reference number"
            }
            Enum1153::InvoicingDataSheetReferenceNumber => "Invoicing data sheet reference number",
            Enum1153::InventoryReportReferenceNumber => "Inventory report reference number",
            Enum1153::CeilingFormulaReferenceNumber => "Ceiling formula reference number",
            Enum1153::PriceVariationFormulaReferenceNumber => {
                "Price variation formula reference number"
            }
            Enum1153::ReferenceToAccountServicingBanksMessage => {
                "Reference to account servicing bank's message"
            }
            Enum1153::PartySequenceNumber => "Party sequence number",
            Enum1153::PurchasersRequestReference => "Purchaser's request reference",
            Enum1153::ContractorRequestReference => "Contractor request reference",
            Enum1153::AccidentReferenceNumber => "Accident reference number",
            Enum1153::CommercialAccountSummaryReferenceNumber => {
                "Commercial account summary reference number"
            }
            Enum1153::ContractBreakdownReference => "Contract breakdown reference",
            Enum1153::ContractorRegistrationNumber => "Contractor registration number",
            Enum1153::ApplicableCoefficientIdentificationNumber => {
                "Applicable coefficient identification number"
            }
            Enum1153::SpecialBudgetAccountNumber => "Special budget account number",
            Enum1153::AuthorisationForRepairReference => "Authorisation for repair reference",
            Enum1153::ManufacturerDefinedRepairRatesReference => {
                "Manufacturer defined repair rates reference"
            }
            Enum1153::OriginalSubmitterLogNumber => "Original submitter log number",
            Enum1153::OriginalSubmitterParentDataMaintenanceRequestDmr => {
                "Original submitter, parent Data Maintenance Request (DMR)"
            }
            Enum1153::OriginalSubmitterChildDataMaintenanceRequestDmr => {
                "Original submitter, child Data Maintenance Request (DMR)"
            }
            Enum1153::EntryPointAssessmentLogNumber => "Entry point assessment log number",
            Enum1153::EntryPointAssessmentLogNumberParentDmr => {
                "Entry point assessment log number, parent DMR"
            }
            Enum1153::EntryPointAssessmentLogNumberChildDmr => {
                "Entry point assessment log number, child DMR"
            }
            Enum1153::DataStructureTag => "Data structure tag",
            Enum1153::CentralSecretariatLogNumber => "Central secretariat log number",
            Enum1153::CentralSecretariatLogNumberParentDataMaintenance => {
                "Central secretariat log number, parent Data Maintenance"
            }
            Enum1153::CentralSecretariatLogNumberChildDataMaintenance => {
                "Central secretariat log number, child Data Maintenance"
            }
            Enum1153::InternationalAssessmentLogNumber => "International assessment log number",
            Enum1153::InternationalAssessmentLogNumberParentData => {
                "International assessment log number, parent Data"
            }
            Enum1153::InternationalAssessmentLogNumberChildDataMaintenance => {
                "International assessment log number, child Data Maintenance"
            }
            Enum1153::StatusReportNumber => "Status report number",
            Enum1153::MessageDesignGroupNumber => "Message design group number",
            Enum1153::UsCustomsServiceUscsEntryCode => "US Customs Service (USCS) entry code",
            Enum1153::BeginningJobSequenceNumber => "Beginning job sequence number",
            Enum1153::SendersClauseNumber => "Sender's clause number",
            Enum1153::DunAndBradstreetCanadas8DigitStandardIndustrial => {
                "Dun and Bradstreet Canada's 8 digit Standard Industrial"
            }
            Enum1153::ActivitePrincipaleExerceeApeIdentifier => {
                "Activite Principale Exercee (APE) identifier"
            }
            Enum1153::DunAndBradstreetUs8DigitStandardIndustrial => {
                "Dun and Bradstreet US 8 digit Standard Industrial"
            }
            Enum1153::NomenclatureActivityClassificationEconomyNace => {
                "Nomenclature Activity Classification Economy (NACE)"
            }
            Enum1153::NormeActiviteFrancaiseNafIdentifier => {
                "Norme Activite Francaise (NAF) identifier"
            }
            Enum1153::RegisteredContractorActivityType => "Registered contractor activity type",
            Enum1153::StatisticBundesAmtSbaIdentifier => "Statistic Bundes Amt (SBA) identifier",
            Enum1153::StateOrProvinceAssignedEntityIdentification => {
                "State or province assigned entity identification"
            }
            Enum1153::InstituteSecurityAndFutureMarketDevelopmentIsfmd => {
                "Institute of Security and Future Market Development (ISFMD)"
            }
            Enum1153::FileIdentificationNumber => "File identification number",
            Enum1153::BankruptcyProcedureNumber => "Bankruptcy procedure number",
            Enum1153::NationalGovernmentBusinessIdentificationNumber => {
                "National government business identification number"
            }
            Enum1153::PriorDataUniversalNumberSystemDunsNumber => {
                "Prior Data Universal Number System (DUNS) number"
            }
            Enum1153::CompaniesRegistryOfficeCroNumber => "Companies Registry Office (CRO) number",
            Enum1153::CostaRicanJudicialNumber => "Costa Rican judicial number",
            Enum1153::NumeroDeIdentificacionTributariaNit => {
                "Numero de Identificacion Tributaria (NIT)"
            }
            Enum1153::PatronNumber => "Patron number",
            Enum1153::RegistroInformacionFiscalRifNumber => {
                "Registro Informacion Fiscal (RIF) number"
            }
            Enum1153::RegistroUnicoDeContribuyenteRucNumber => {
                "Registro Unico de Contribuyente (RUC) number"
            }
            Enum1153::TokyoShokoResearchTsrBusinessIdentifier => {
                "Tokyo SHOKO Research (TSR) business identifier"
            }
            Enum1153::PersonalIdentityCardNumber => "Personal identity card number",
            Enum1153::SystemeInformatiquePourLeRepertoireDesEntreprises => {
                "Systeme Informatique pour le Repertoire des ENtreprises"
            }
            Enum1153::SystemeInformatiquePourLeRepertoireDesEtablissements => {
                "Systeme Informatique pour le Repertoire des ETablissements"
            }
            Enum1153::PublicationIssueNumber => "Publication issue number",
            Enum1153::OriginalFilingNumber => "Original filing number",
            Enum1153::DocumentPageIdentifier => "Document page identifier",
            Enum1153::PublicFilingRegistrationNumber => "Public filing registration number",
            Enum1153::RegiristoFederalDeContribuyentes => "Regiristo Federal de Contribuyentes",
            Enum1153::SocialSecurityNumber => "Social security number",
            Enum1153::DocumentVolumeNumber => "Document volume number",
            Enum1153::BookNumber => "Book number",
            Enum1153::StockExchangeCompanyIdentifier => "Stock exchange company identifier",
            Enum1153::ImputationAccount => "Imputation account",
            Enum1153::FinancialPhaseReference => "Financial phase reference",
            Enum1153::TechnicalPhaseReference => "Technical phase reference",
            Enum1153::PriorContractorRegistrationNumber => "Prior contractor registration number",
            Enum1153::StockAdjustmentNumber => "Stock adjustment number",
            Enum1153::DispensationReference => "Dispensation reference",
            Enum1153::InvestmentReferenceNumber => "Investment reference number",
            Enum1153::AssumingCompany => "Assuming company",
            Enum1153::BudgetChapter => "Budget chapter",
            Enum1153::DutyFreeProductsSecurityNumber => "Duty free products security number",
            Enum1153::DutyFreeProductsReceiptAuthorisationNumber => {
                "Duty free products receipt authorisation number"
            }
            Enum1153::PartyInformationMessageReference => "Party information message reference",
            Enum1153::FormalStatementReference => "Formal statement reference",
            Enum1153::ProofDeliveryReferenceNumber => "Proof of delivery reference number",
            Enum1153::SuppliersCreditClaimReferenceNumber => {
                "Supplier's credit claim reference number"
            }
            Enum1153::PictureActualProduct => "Picture of actual product",
            Enum1153::PictureAGenericProduct => "Picture of a generic product",
            Enum1153::TradingPartnerIdentificationNumber => "Trading partner identification number",
            Enum1153::PriorTradingPartnerIdentificationNumber => {
                "Prior trading partner identification number"
            }
            Enum1153::Password => "Password",
            Enum1153::FormalReportNumber => "Formal report number",
            Enum1153::FundAccountNumber => "Fund account number",
            Enum1153::SafeCustodyNumber => "Safe custody number",
            Enum1153::MasterAccountNumber => "Master account number",
            Enum1153::GroupReferenceNumber => "Group reference number",
            Enum1153::AccountingTransmissionNumber => "Accounting transmission number",
            Enum1153::ProductDataFileNumber => "Product data file number",
            Enum1153::CadastroGeralDoContribuinteCgc => "Cadastro Geral do Contribuinte (CGC)",
            Enum1153::ForeignResidentIdentificationNumber => {
                "Foreign resident identification number"
            }
            Enum1153::CdRom => "CD-ROM",
            Enum1153::PhysicalMedium => "Physical medium",
            Enum1153::FinancialCancellationReferenceNumber => {
                "Financial cancellation reference number"
            }
            Enum1153::PurchaseForExportCustomsAgreementNumber => {
                "Purchase for export Customs agreement number"
            }
            Enum1153::JudgmentNumber => "Judgment number",
            Enum1153::SecretariatNumber => "Secretariat number",
            Enum1153::PreviousBankingStatusMessageReference => {
                "Previous banking status message reference"
            }
            Enum1153::LastReceivedBankingStatusMessageReference => {
                "Last received banking status message reference"
            }
            Enum1153::BanksDocumentaryProcedureReference => {
                "Bank's documentary procedure reference"
            }
            Enum1153::CustomersDocumentaryProcedureReference => {
                "Customer's documentary procedure reference"
            }
            Enum1153::SafeDepositBoxNumber => "Safe deposit box number",
            Enum1153::ReceivingBankgiroNumber => "Receiving Bankgiro number",
            Enum1153::SendingBankgiroNumber => "Sending Bankgiro number",
            Enum1153::BankgiroReference => "Bankgiro reference",
            Enum1153::GuaranteeNumber => "Guarantee number",
            Enum1153::CollectionInstrumentNumber => "Collection instrument number",
            Enum1153::ConvertedPostgiroNumber => "Converted Postgiro number",
            Enum1153::CostCentreAlignmentNumber => "Cost centre alignment number",
            Enum1153::KamerVanKoophandelKvkNumber => "Kamer Van Koophandel (KVK) number",
            Enum1153::InstitutBelgoLuxembourgeoisDeCodificationIblcNumber => {
                "Institut Belgo-Luxembourgeois de Codification (IBLC) number"
            }
            Enum1153::ExternalObjectReference => "External object reference",
            Enum1153::ExceptionalTransportAuthorisationNumber => {
                "Exceptional transport authorisation number"
            }
            Enum1153::ClaveUnicaDeIdentificacionTributariaCuit => {
                "Clave Unica de Identificacion Tributaria (CUIT)"
            }
            Enum1153::RegistroUnicoTributarioRut => "Registro Unico Tributario (RUT)",
            Enum1153::FlatRackContainerBundleIdentificationNumber => {
                "Flat rack container bundle identification number"
            }
            Enum1153::TransportEquipmentAcceptanceOrderReference => {
                "Transport equipment acceptance order reference"
            }
            Enum1153::TransportEquipmentReleaseOrderReference => {
                "Transport equipment release order reference"
            }
            Enum1153::ShipsStayReferenceNumber => "Ship's stay reference number",
            Enum1153::AuthorizationToMeetCompetitionNumber => {
                " Authorization to meet competition number"
            }
            Enum1153::PlacePositioningReference => "Place of positioning reference",
            Enum1153::PartyReference => "Party reference",
            Enum1153::IssuedPrescriptionIdentification => "Issued prescription identification",
            Enum1153::CollectionReference => "Collection reference",
            Enum1153::TravelService => "Travel service",
            Enum1153::ConsignmentStockContract => "Consignment stock contract",
            Enum1153::ImportersLetterCreditReference => "Importer's letter of credit reference",
            Enum1153::PerformedPrescriptionIdentification => {
                "Performed prescription identification"
            }
            Enum1153::ImageReference => "Image reference",
            Enum1153::ProposedPurchaseOrderReferenceNumber => {
                "Proposed purchase order reference number"
            }
            Enum1153::ApplicationForFinancialSupportReferenceNumber => {
                "Application for financial support reference number"
            }
            Enum1153::ManufacturingQualityAgreementNumber => {
                "Manufacturing quality agreement number"
            }
            Enum1153::SoftwareEditorReference => "Software editor reference",
            Enum1153::SoftwareReference => "Software reference",
            Enum1153::SoftwareQualityReference => "Software quality reference",
            Enum1153::ConsolidatedOrdersReference => "Consolidated orders' reference",
            Enum1153::CustomsBindingRulingNumber => "Customs binding ruling number",
            Enum1153::CustomsNonBindingRulingNumber => "Customs non-binding ruling number",
            Enum1153::DeliveryRouteReference => "Delivery route reference",
            Enum1153::NetAreaSupplierReference => "Net area supplier reference",
            Enum1153::TimeSeriesReference => "Time series reference",
            Enum1153::ConnectingPointToCentralGrid => "Connecting point to central grid",
            Enum1153::MarketingPlanIdentificationNumberMpin => {
                "Marketing plan identification number (MPIN)"
            }
            Enum1153::EntityReferenceNumberPrevious => "Entity reference number, previous",
            Enum1153::InternationalStandardIndustrialClassificationIsic => {
                "International Standard Industrial Classification (ISIC)"
            }
            Enum1153::CustomsPreApprovalRulingNumber => "Customs pre-approval ruling number",
            Enum1153::AccountPayableNumber => " Account payable number",
            Enum1153::FirstFinancialInstitutionsTransactionReference => {
                "First financial institution's transaction reference"
            }
            Enum1153::ProductCharacteristicsDirectory => "Product characteristics directory",
            Enum1153::SuppliersCustomerReferenceNumber => "Supplier's customer reference number",
            Enum1153::InventoryReportRequestNumber => "Inventory report request number",
            Enum1153::MeteringPoint => "Metering point",
            Enum1153::PassengerReservationNumber => "Passenger reservation number",
            Enum1153::SlaughterhouseApprovalNumber => "Slaughterhouse approval number",
            Enum1153::MeatCuttingPlantApprovalNumber => "Meat cutting plant approval number",
            Enum1153::CustomerTravelServiceIdentifier => "Customer travel service identifier",
            Enum1153::ExportControlClassificationNumber => "Export control classification number",
            Enum1153::BrokerReference3 => "Broker reference 3",
            Enum1153::ConsignmentInformation => "Consignment information",
            Enum1153::GoodsItemInformation => "Goods item information",
            Enum1153::DangerousGoodsInformation => "Dangerous Goods information",
            Enum1153::PilotageServicesExemptionNumber => "Pilotage services exemption number",
            Enum1153::PersonRegistrationNumber => "Person registration number",
            Enum1153::PlacePackingApprovalNumber => "Place of packing approval number",
            Enum1153::OriginalMandateReference => "Original Mandate Reference",
            Enum1153::MandateReference => "Mandate Reference",
            Enum1153::ReservationStationIndentifier => "Reservation station indentifier",
            Enum1153::UniqueGoodsShipmentIdentifier => "Unique goods shipment identifier",
            Enum1153::FrameworkAgreementNumber => "Framework Agreement Number",
            Enum1153::HashValue => "Hash value",
            Enum1153::MovementReferenceNumber => "Movement reference number",
            Enum1153::EconomicOperatorsRegistrationAndIdentificationNumber => {
                "Economic Operators Registration and Identification Number"
            }
            Enum1153::LocalReferenceNumber => "Local Reference Number",
            Enum1153::RateCodeNumber => "Rate code number",
            Enum1153::AirWaybillNumber => "Air waybill number",
            Enum1153::DocumentaryCreditAmendmentNumber => "Documentary credit amendment number",
            Enum1153::AdvisingBanksReference => "Advising bank's reference",
            Enum1153::CostCentre => "Cost centre",
            Enum1153::WorkItemQuantityDetermination => "Work item quantity determination",
            Enum1153::InternalDataProcessNumber => "Internal data process number",
            Enum1153::CategoryWorkReference => "Category of work reference",
            Enum1153::PolicyFormNumber => "Policy form number",
            Enum1153::NetArea => "Net area",
            Enum1153::ServiceProvider => "Service provider",
            Enum1153::ErrorPosition => "Error position",
            Enum1153::ServiceCategoryReference => "Service category reference",
            Enum1153::ConnectedLocation => "Connected location",
            Enum1153::RelatedParty => "Related party",
            Enum1153::LatestAccountingEntryRecordReference => {
                "Latest accounting entry record reference"
            }
            Enum1153::AccountingEntry => "Accounting entry",
            Enum1153::DocumentReferenceOriginal => "Document reference, original",
            Enum1153::HygienicCertificateNumberNational => "Hygienic Certificate number, national",
            Enum1153::AdministrativeReferenceCode => "Administrative Reference Code",
            Enum1153::PickUpSheetNumber => "Pick-up sheet number",
            Enum1153::PhoneNumber => "Phone number",
            Enum1153::BuyersFundNumber => "Buyer's fund number",
            Enum1153::CompanyTradingAccountNumber => "Company trading account number",
            Enum1153::ReservedGoodsIdentifier => "Reserved goods identifier",
            Enum1153::HandlingAndMovementReferenceNumber => {
                "Handling and movement reference number"
            }
            Enum1153::InstructionToDespatchReferenceNumber => {
                "Instruction to despatch reference number"
            }
            Enum1153::InstructionForReturnsNumber => "Instruction for returns number",
            Enum1153::MeteredServicesConsumptionReportNumber => {
                "Metered services consumption report number"
            }
            Enum1153::OrderStatusEnquiryNumber => "Order status enquiry number",
            Enum1153::FirmBookingReferenceNumber => "Firm booking reference number",
            Enum1153::ProductInquiryNumber => "Product inquiry number",
            Enum1153::SplitDeliveryNumber => "Split delivery number",
            Enum1153::ServiceRelationNumber => "Service relation number",
            Enum1153::SerialShippingContainerCode => "Serial shipping container code",
            Enum1153::TestSpecificationNumber => "Test specification number",
            Enum1153::TransportStatusReportNumber => "Transport status report number",
            Enum1153::ToolingContractNumber => "Tooling contract number",
            Enum1153::FormulaReferenceNumber => "Formula reference number",
            Enum1153::PreAgreementNumber => "Pre-agreement number",
            Enum1153::ProductCertificationNumber => "Product certification number",
            Enum1153::ConsignmentContractNumber => "Consignment contract number",
            Enum1153::ProductSpecificationReferenceNumber => {
                "Product specification reference number"
            }
            Enum1153::PayrollDeductionAdviceReference => "Payroll deduction advice reference",
            Enum1153::TracesPartyIdentification => "TRACES party identification",
            Enum1153::BlockStowageReference => "Block Stowage Reference",
            Enum1153::BeginningMeterReadingActual => " Beginning meter reading actual",
            Enum1153::BuyersContractNumber => " Buyer's contract number",
            Enum1153::BidNumber => " Bid number",
            Enum1153::BeginningMeterReadingEstimated => " Beginning meter reading estimated",
            Enum1153::HouseBillLadingNumber => " House bill of lading number",
            Enum1153::BillLadingNumber => " Bill of lading number",
            Enum1153::ConsignmentIdentifierCarrierAssigned => {
                " Consignment identifier, carrier assigned"
            }
            Enum1153::BlanketOrderNumber => " Blanket order number",
            Enum1153::BrokerOrSalesOfficeNumber => " Broker or sales office number",
            Enum1153::BatchNumberLotNumber => " Batch number/lot number",
            Enum1153::BatteryAndAccumulatorProducerRegistrationNumber => {
                "Battery and accumulator producer registration number"
            }
            Enum1153::BlendedWithNumber => " Blended with number",
            Enum1153::IataCargoAgentCassAddressNumber => "IATA Cargo Agent CASS Address number",
            Enum1153::MatchingEntriesBalanced => "Matching of entries, balanced",
            Enum1153::EntryFlagging => "Entry flagging",
            Enum1153::MatchingEntriesUnbalanced => "Matching of entries, unbalanced",
            Enum1153::DocumentReferenceInternal => "Document reference, internal",
            Enum1153::EuropeanValueAddedTaxIdentification => {
                "European Value Added Tax identification"
            }
            Enum1153::CostAccountingDocument => "Cost accounting document",
            Enum1153::GridOperatorsCustomerReferenceNumber => {
                "Grid operator's customer reference number"
            }
            Enum1153::TicketControlNumber => "Ticket control number",
            Enum1153::OrderShipmentGroupingReference => "Order shipment grouping reference",
            Enum1153::CreditNoteNumber => " Credit note number",
            Enum1153::CedingCompany => "Ceding company",
            Enum1153::DebitLetterNumber => "Debit letter number",
            Enum1153::ConsigneesFurtherOrder => "Consignee's further order",
            Enum1153::AnimalFarmLicenceNumber => "Animal farm licence number",
            Enum1153::ConsignorsFurtherOrder => "Consignor's further order",
            Enum1153::ConsigneesOrderNumber => " Consignee's order number",
            Enum1153::CustomerCatalogueNumber => " Customer catalogue number",
            Enum1153::ChequeNumber => " Cheque number",
            Enum1153::CheckingNumber => "Checking number",
            Enum1153::CreditMemoNumber => " Credit memo number",
            Enum1153::RoadConsignmentNoteNumber => "Road consignment note number",
            Enum1153::CarriersReferenceNumber => " Carrier's reference number",
            Enum1153::ChargesNoteDocumentAttachmentIndicator => {
                "Charges note document attachment indicator"
            }
            Enum1153::CallOffOrderNumber => "Call off order number",
            Enum1153::ConditionPurchaseDocumentNumber => " Condition of purchase document number",
            Enum1153::CustomerReferenceNumber => " Customer reference number",
            Enum1153::TransportMeansJourneyIdentifier => "Transport means journey identifier",
            Enum1153::ConditionSaleDocumentNumber => " Condition of sale document number",
            Enum1153::TeamAssignmentNumber => "Team assignment number",
            Enum1153::ContractNumber => " Contract number",
            Enum1153::ConsignmentIdentifierConsignorAssigned => {
                " Consignment identifier, consignor assigned"
            }
            Enum1153::ContainerOperatorsReferenceNumber => " Container operators reference number",
            Enum1153::PackageNumber => " Package number",
            Enum1153::CooperationContractNumber => " Cooperation contract number",
            Enum1153::DefermentApprovalNumber => " Deferment approval number",
            Enum1153::DebitAccountNumber => "Debit account number",
            Enum1153::BuyersDebtorNumber => " Buyer's debtor number",
            Enum1153::DistributorInvoiceNumber => " Distributor invoice number",
            Enum1153::DebitNoteNumber => " Debit note number",
            Enum1153::DocumentIdentifier => " Document identifier",
            Enum1153::DeliveryNoteNumber => " Delivery note number",
            Enum1153::DockReceiptNumber => " Dock receipt number",
            Enum1153::EndingMeterReadingActual => " Ending meter reading actual",
            Enum1153::EmbargoPermitNumber => " Embargo permit number",
            Enum1153::ExportDeclaration => " Export declaration",
            Enum1153::EndingMeterReadingEstimated => " Ending meter reading estimated",
            Enum1153::ElectricalAndElectronicEquipmentProducerRegistration => {
                "Electrical and electronic equipment producer registration"
            }
            Enum1153::EmployersIdentificationNumber => " Employer's identification number",
            Enum1153::EmbargoNumber => " Embargo number",
            Enum1153::EquipmentNumber => " Equipment number",
            Enum1153::ContainerEquipmentReceiptNumber => " Container/equipment receipt number",
            Enum1153::ExportersReferenceNumber => "Exporter's reference number",
            Enum1153::ExcessTransportationNumber => " Excess transportation number",
            Enum1153::ExportPermitIdentifier => " Export permit identifier",
            Enum1153::FiscalNumber => " Fiscal number",
            Enum1153::ConsignmentIdentifierFreightForwarderAssigned => {
                " Consignment identifier, freight forwarder assigned"
            }
            Enum1153::FileLineIdentifier => " File line identifier",
            Enum1153::FlowReferenceNumber => "Flow reference number",
            Enum1153::FreightBillNumber => " Freight bill number",
            Enum1153::ForeignExchange => " Foreign exchange",
            Enum1153::FinalSequenceNumber => " Final sequence number",
            Enum1153::FreeZoneIdentifier => " Free zone identifier",
            Enum1153::FileVersionNumber => " File version number",
            Enum1153::ForeignExchangeContractNumber => " Foreign exchange contract number",
            Enum1153::StandardsNumber => " Standard's number",
            Enum1153::GovernmentContractNumber => " Government contract number",
            Enum1153::StandardsCodeNumber => " Standard's code number",
            Enum1153::GeneralDeclarationNumber => "General declaration number",
            Enum1153::GovernmentReferenceNumber => " Government reference number",
            Enum1153::HarmonisedSystemNumber => " Harmonised system number",
            Enum1153::HouseWaybillNumber => "House waybill number",
            Enum1153::InternalVendorNumber => " Internal vendor number",
            Enum1153::InBondNumber => " In bond number",
            Enum1153::IataCargoAgentCodeNumber => "IATA cargo agent code number",
            Enum1153::InsuranceCertificateReferenceNumber => {
                "Insurance certificate reference number"
            }
            Enum1153::InsuranceContractReferenceNumber => "Insurance contract reference number",
            Enum1153::InitialSampleInspectionReportNumber => {
                " Initial sample inspection report number"
            }
            Enum1153::InternalOrderNumber => " Internal order number",
            Enum1153::IntermediaryBroker => "Intermediary broker",
            Enum1153::InterchangeNumberNew => "Interchange number new",
            Enum1153::InterchangeNumberOld => "Interchange number old",
            Enum1153::ImportPermitIdentifier => " Import permit identifier",
            Enum1153::InvoiceNumberSuffix => " Invoice number suffix",
            Enum1153::InternalCustomerNumber => " Internal customer number",
            Enum1153::InvoiceDocumentIdentifier => " Invoice document identifier",
            Enum1153::JobNumber => " Job number",
            Enum1153::EndingJobSequenceNumber => " Ending job sequence number",
            Enum1153::ShippingLabelSerialNumber => " Shipping label serial number",
            Enum1153::LoadingAuthorisationIdentifier => "Loading authorisation identifier",
            Enum1153::LowerNumberInRange => "Lower number in range",
            Enum1153::Lockbox => " Lockbox",
            Enum1153::LetterCreditNumber => " Letter of credit number",
            Enum1153::DocumentLineIdentifier => " Document line identifier",
            Enum1153::LoadPlanningNumber => " Load planning number",
            Enum1153::ReservationOfficeIdentifier => "Reservation office identifier",
            Enum1153::BarCodedLabelSerialNumber => " Bar coded label serial number",
            Enum1153::ShipNoticeManifestNumber => " Ship notice/manifest number",
            Enum1153::MasterBillLadingNumber => " Master bill of lading number",
            Enum1153::ManufacturersPartNumber => " Manufacturer's part number",
            Enum1153::MeterUnitNumber => " Meter unit number",
            Enum1153::ManufacturingOrderNumber => " Manufacturing order number",
            Enum1153::MessageRecipient => " Message recipient",
            Enum1153::MailingReferenceNumber => "Mailing reference number",
            Enum1153::MessageSender => " Message sender",
            Enum1153::ManufacturersMaterialSafetyDataSheetNumber => {
                "Manufacturer's material safety data sheet number"
            }
            Enum1153::MasterAirWaybillNumber => "Master air waybill number",
            Enum1153::NorthAmericanHazardousGoodsClassificationNumber => {
                " North American hazardous goods classification number"
            }
            Enum1153::NotaFiscal => " Nota Fiscal",
            Enum1153::CurrentInvoiceNumber => " Current invoice number",
            Enum1153::PreviousInvoiceNumber => " Previous invoice number",
            Enum1153::OrderDocumentIdentifierBuyerAssigned => {
                " Order document identifier, buyer assigned"
            }
            Enum1153::OriginalPurchaseOrder => " Original purchase order",
            Enum1153::GeneralOrderNumber => " General order number",
            Enum1153::PayersFinancialInstitutionAccountNumber => {
                " Payer's financial institution account number"
            }
            Enum1153::ProductionCode => " Production code",
            Enum1153::PromotionDealNumber => " Promotion deal number",
            Enum1153::PlantNumber => " Plant number",
            Enum1153::PrimeContractorContractNumber => " Prime contractor contract number",
            Enum1153::PriceListVersionNumber => " Price list version number",
            Enum1153::PackingListNumber => " Packing list number",
            Enum1153::PriceListNumber => " Price list number",
            Enum1153::PurchaseOrderResponseNumber => "Purchase order response number",
            Enum1153::PurchaseOrderChangeNumber => " Purchase order change number",
            Enum1153::PaymentReference => " Payment reference",
            Enum1153::PriceQuoteNumber => " Price quote number",
            Enum1153::PurchaseOrderNumberSuffix => " Purchase order number suffix",
            Enum1153::PriorPurchaseOrderNumber => " Prior purchase order number",
            Enum1153::PayeesFinancialInstitutionAccountNumber => {
                " Payee's financial institution account number"
            }
            Enum1153::RemittanceAdviceNumber => " Remittance advice number",
            Enum1153::RailRoadRoutingCode => " Rail/road routing code",
            Enum1153::RailwayConsignmentNoteNumber => "Railway consignment note number",
            Enum1153::ReleaseNumber => " Release number",
            Enum1153::ConsignmentReceiptIdentifier => "Consignment receipt identifier",
            Enum1153::ExportReferenceNumber => " Export reference number",
            Enum1153::PayersFinancialInstitutionTransitRoutingNoAch => {
                " Payer's financial institution transit routing No.(ACH"
            }
            Enum1153::PayeesFinancialInstitutionTransitRoutingNo => {
                " Payee's financial institution transit routing No."
            }
            Enum1153::SalesPersonNumber => " Sales person number",
            Enum1153::SalesRegionNumber => " Sales region number",
            Enum1153::SalesDepartmentNumber => " Sales department number",
            Enum1153::SerialNumber => " Serial number",
            Enum1153::AllocatedSeat => "Allocated seat",
            Enum1153::ShipFrom => " Ship from",
            Enum1153::PreviousHighestScheduleNumber => " Previous highest schedule number",
            Enum1153::SidShippersIdentifyingNumberForShipment => {
                " SID (Shipper's identifying number for shipment)"
            }
            Enum1153::SalesOfficeNumber => " Sales office number",
            Enum1153::TransportEquipmentSealIdentifier => " Transport equipment seal identifier",
            Enum1153::ScanLine => " Scan line",
            Enum1153::EquipmentSequenceNumber => " Equipment sequence number",
            Enum1153::ShipmentReferenceNumber => "Shipment reference number",
            Enum1153::SellersReferenceNumber => " Sellers reference number",
            Enum1153::StationReferenceNumber => "Station reference number",
            Enum1153::SwapOrderNumber => " Swap order number",
            Enum1153::SpecificationNumber => " Specification number",
            Enum1153::TruckersBillLading => " Trucker's bill of lading",
            Enum1153::TerminalOperatorsConsignmentReference => {
                "Terminal operator's consignment reference"
            }
            Enum1153::TelexMessageNumber => " Telex message number",
            Enum1153::TransferNumber => " Transfer number",
            Enum1153::TirCarnetNumber => " TIR carnet number",
            Enum1153::TransportInstructionNumber => "Transport instruction number",
            Enum1153::TaxExemptionLicenceNumber => " Tax exemption licence number",
            Enum1153::TransactionReferenceNumber => " Transaction reference number",
            Enum1153::TestReportNumber => " Test report number",
            Enum1153::UpperNumberRange => "Upper number of range",
            Enum1153::UltimateCustomersReferenceNumber => " Ultimate customer's reference number",
            Enum1153::UniqueConsignmentReferenceNumber => "Unique consignment reference number",
            Enum1153::UnitedNationsDangerousGoodsIdentifier => {
                " United Nations Dangerous Goods identifier"
            }
            Enum1153::UltimateCustomersOrderNumber => " Ultimate customer's order number",
            Enum1153::UniformResourceIdentifier => "Uniform Resource Identifier",
            Enum1153::VatRegistrationNumber => " VAT registration number",
            Enum1153::VendorContractNumber => " Vendor contract number",
            Enum1153::TransportEquipmentGrossMassVerificationReference => {
                "Transport equipment gross mass verification reference"
            }
            Enum1153::VesselIdentifier => " Vessel identifier",
            Enum1153::OrderNumberVendor => " Order number (vendor)",
            Enum1153::VoyageNumber => "Voyage number",
            Enum1153::TransportEquipmentGrossMassVerificationOrderReference => {
                "Transport equipment gross mass verification order reference"
            }
            Enum1153::VendorProductNumber => " Vendor product number",
            Enum1153::VendorIdNumber => " Vendor ID number",
            Enum1153::VendorOrderNumberSuffix => " Vendor order number suffix",
            Enum1153::MotorVehicleIdentificationNumber => " Motor vehicle identification number",
            Enum1153::VoucherNumber => " Voucher number",
            Enum1153::WarehouseEntryNumber => " Warehouse entry number",
            Enum1153::WeightAgreementNumber => " Weight agreement number",
            Enum1153::WellNumber => " Well number",
            Enum1153::WarehouseReceiptNumber => " Warehouse receipt number",
            Enum1153::WarehouseStorageLocationNumber => " Warehouse storage location number",
            Enum1153::RailWaybillNumber => " Rail waybill number",
            Enum1153::CompanyPlaceRegistrationNumber => " Company/place registration number",
            Enum1153::CargoControlNumber => " Cargo control number",
            Enum1153::PreviousCargoControlNumber => " Previous cargo control number",
            Enum1153::MutuallyDefinedReferenceNumber => "Mutually defined reference number",
        }
    }
}

impl crate::FromCode for Enum1153 {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "AAA" => Some(Enum1153::OrderAcknowledgementDocumentIdentifier),
            "AAB" => Some(Enum1153::ProformaInvoiceDocumentIdentifier),
            "AAC" => Some(Enum1153::DocumentaryCreditIdentifier),
            "AAD" => Some(Enum1153::ContractDocumentAddendumIdentifier),
            "AAE" => Some(Enum1153::GoodsDeclarationNumber),
            "AAF" => Some(Enum1153::DebitCardNumber),
            "AAG" => Some(Enum1153::OfferNumber),
            "AAH" => Some(Enum1153::BanksBatchInterbankTransactionReferenceNumber),
            "AAI" => Some(Enum1153::BanksIndividualInterbankTransactionReferenceNumber),
            "AAJ" => Some(Enum1153::DeliveryOrderNumber),
            "AAK" => Some(Enum1153::DespatchAdviceNumber),
            "AAL" => Some(Enum1153::DrawingNumber),
            "AAM" => Some(Enum1153::WaybillNumber),
            "AAN" => Some(Enum1153::DeliveryScheduleNumber),
            "AAO" => Some(Enum1153::ConsignmentIdentifierConsigneeAssigned),
            "AAP" => Some(Enum1153::PartialShipmentIdentifier),
            "AAQ" => Some(Enum1153::TransportEquipmentIdentifier),
            "AAR" => Some(Enum1153::MunicipalityAssignedBusinessRegistryNumber),
            "AAS" => Some(Enum1153::TransportContractDocumentIdentifier),
            "AAT" => Some(Enum1153::MasterLabelNumber),
            "AAU" => Some(Enum1153::DespatchNoteDocumentIdentifier),
            "AAV" => Some(Enum1153::EnquiryNumber),
            "AAW" => Some(Enum1153::DocketNumber),
            "AAX" => Some(Enum1153::CivilActionNumber),
            "AAY" => Some(Enum1153::CarriersAgentReferenceNumber),
            "AAZ" => Some(Enum1153::StandardCarrierAlphaCodeScacNumber),
            "ABA" => Some(Enum1153::CustomsValuationDecisionNumber),
            "ABB" => Some(Enum1153::EndUseAuthorizationNumber),
            "ABC" => Some(Enum1153::AntiDumpingCaseNumber),
            "ABD" => Some(Enum1153::CustomsTariffNumber),
            "ABE" => Some(Enum1153::DeclarantsReferenceNumber),
            "ABF" => Some(Enum1153::RepairEstimateNumber),
            "ABG" => Some(Enum1153::CustomsDecisionRequestNumber),
            "ABH" => Some(Enum1153::SubHouseBillLadingNumber),
            "ABI" => Some(Enum1153::TaxPaymentIdentifier),
            "ABJ" => Some(Enum1153::QuotaNumber),
            "ABK" => Some(Enum1153::TransitOnwardCarriageGuaranteeBondNumber),
            "ABL" => Some(Enum1153::CustomsGuaranteeNumber),
            "ABM" => Some(Enum1153::ReplacingPartNumber),
            "ABN" => Some(Enum1153::SellersCatalogueNumber),
            "ABO" => Some(Enum1153::OriginatorsReference),
            "ABP" => Some(Enum1153::DeclarantsCustomsIdentityNumber),
            "ABQ" => Some(Enum1153::ImporterReferenceNumber),
            "ABR" => Some(Enum1153::ExportClearanceInstructionReferenceNumber),
            "ABS" => Some(Enum1153::ImportClearanceInstructionReferenceNumber),
            "ABT" => Some(Enum1153::GoodsDeclarationDocumentIdentifierCustoms),
            "ABU" => Some(Enum1153::ArticleNumber),
            "ABV" => Some(Enum1153::IntraPlantRouting),
            "ABW" => Some(Enum1153::StockKeepingUnitNumber),
            "ABX" => Some(Enum1153::TextElementIdentifierDeletionReference),
            "ABY" => Some(Enum1153::AllotmentIdentificationAir),
            "ABZ" => Some(Enum1153::VehicleLicenceNumber),
            "AC" => Some(Enum1153::AirCargoTransferManifest),
            "ACA" => Some(Enum1153::CargoAcceptanceOrderReferenceNumber),
            "ACB" => Some(Enum1153::UsGovernmentAgencyNumber),
            "ACC" => Some(Enum1153::ShippingUnitIdentification),
            "ACD" => Some(Enum1153::AdditionalReferenceNumber),
            "ACE" => Some(Enum1153::RelatedDocumentNumber),
            "ACF" => Some(Enum1153::AddresseeReference),
            "ACG" => Some(Enum1153::AtaCarnetNumber),
            "ACH" => Some(Enum1153::PackagingUnitIdentification),
            "ACI" => Some(Enum1153::OuterpackagingUnitIdentification),
            "ACJ" => Some(Enum1153::CustomerMaterialSpecificationNumber),
            "ACK" => Some(Enum1153::BankReference),
            "ACL" => Some(Enum1153::PrincipalReferenceNumber),
            "ACN" => Some(Enum1153::CollectionAdviceDocumentIdentifier),
            "ACO" => Some(Enum1153::IronChargeNumber),
            "ACP" => Some(Enum1153::HotRollNumber),
            "ACQ" => Some(Enum1153::ColdRollNumber),
            "ACR" => Some(Enum1153::RailwayWagonNumber),
            "ACT" => Some(Enum1153::UniqueClaimsReferenceNumberSender),
            "ACU" => Some(Enum1153::LossEventNumber),
            "ACV" => Some(Enum1153::EstimateOrderReferenceNumber),
            "ACW" => Some(Enum1153::ReferenceNumberToPreviousMessage),
            "ACX" => Some(Enum1153::BankersAcceptance),
            "ACY" => Some(Enum1153::DutyMemoNumber),
            "ACZ" => Some(Enum1153::EquipmentTransportChargeNumber),
            "ADA" => Some(Enum1153::BuyersItemNumber),
            "ADB" => Some(Enum1153::MaturedCertificateDeposit),
            "ADC" => Some(Enum1153::Loan),
            "ADD" => Some(Enum1153::AnalysisNumberTestNumber),
            "ADE" => Some(Enum1153::AccountNumber),
            "ADF" => Some(Enum1153::TreatyNumber),
            "ADG" => Some(Enum1153::CatastropheNumber),
            "ADI" => Some(Enum1153::BureauSigningStatementReference),
            "ADJ" => Some(Enum1153::CompanySyndicateReference1),
            "ADK" => Some(Enum1153::CompanySyndicateReference2),
            "ADL" => Some(Enum1153::OrderingCustomerConsignmentReferenceNumber),
            "ADM" => Some(Enum1153::ShipownersAuthorizationNumber),
            "ADN" => Some(Enum1153::InlandTransportOrderNumber),
            "ADO" => Some(Enum1153::ContainerWorkOrderReferenceNumber),
            "ADP" => Some(Enum1153::StatementNumber),
            "ADQ" => Some(Enum1153::UniqueMarketReference),
            "ADT" => Some(Enum1153::GroupAccounting),
            "ADU" => Some(Enum1153::BrokerReference1),
            "ADV" => Some(Enum1153::BrokerReference2),
            "ADW" => Some(Enum1153::LloydsClaimsOfficeReference),
            "ADX" => Some(Enum1153::SecureDeliveryTermsAndConditionsAgreementReference),
            "ADY" => Some(Enum1153::ReportNumber),
            "ADZ" => Some(Enum1153::TraderAccountNumber),
            "AE" => Some(Enum1153::AuthorizationForExpenseAfeNumber),
            "AEA" => Some(Enum1153::GovernmentAgencyReferenceNumber),
            "AEB" => Some(Enum1153::AssemblyNumber),
            "AEC" => Some(Enum1153::SymbolNumber),
            "AED" => Some(Enum1153::CommodityNumber),
            "AEE" => Some(Enum1153::Eur1CertificateNumber),
            "AEF" => Some(Enum1153::CustomerProcessSpecificationNumber),
            "AEG" => Some(Enum1153::CustomerSpecificationNumber),
            "AEH" => Some(Enum1153::ApplicableInstructionsOrStandards),
            "AEI" => Some(Enum1153::RegistrationNumberPreviousCustomsDeclaration),
            "AEJ" => Some(Enum1153::PostEntryReference),
            "AEK" => Some(Enum1153::PaymentOrderNumber),
            "AEL" => Some(Enum1153::DeliveryNumberTransport),
            "AEM" => Some(Enum1153::TransportRoute),
            "AEN" => Some(Enum1153::CustomersUnitInventoryNumber),
            "AEO" => Some(Enum1153::ProductReservationNumber),
            "AEP" => Some(Enum1153::ProjectNumber),
            "AEQ" => Some(Enum1153::DrawingListNumber),
            "AER" => Some(Enum1153::ProjectSpecificationNumber),
            "AES" => Some(Enum1153::PrimaryReference),
            "AET" => Some(Enum1153::RequestForCancellationNumber),
            "AEU" => Some(Enum1153::SuppliersControlNumber),
            "AEV" => Some(Enum1153::ShippingNoteNumber),
            "AEW" => Some(Enum1153::EmptyContainerBillNumber),
            "AEX" => Some(Enum1153::NonNegotiableMaritimeTransportDocumentNumber),
            "AEY" => Some(Enum1153::SubstituteAirWaybillNumber),
            "AEZ" => Some(Enum1153::DespatchNotePostParcelsNumber),
            "AF" => Some(Enum1153::AirlinesFlightIdentificationNumber),
            "AFA" => Some(Enum1153::ThroughBillLadingNumber),
            "AFB" => Some(Enum1153::CargoManifestNumber),
            "AFC" => Some(Enum1153::BordereauNumber),
            "AFD" => Some(Enum1153::CustomsItemNumber),
            "AFE" => Some(Enum1153::ExportControlCommodityNumberEccn),
            "AFF" => Some(Enum1153::MarkingLabelReference),
            "AFG" => Some(Enum1153::TariffNumber),
            "AFH" => Some(Enum1153::ReplenishmentPurchaseOrderNumber),
            "AFI" => Some(Enum1153::ImmediateTransportationNoForInBondMovement),
            "AFJ" => Some(Enum1153::TransportationExportationNoForInBondMovement),
            "AFK" => Some(Enum1153::ImmediateExportationNoForInBondMovement),
            "AFL" => Some(Enum1153::AssociatedInvoices),
            "AFM" => Some(Enum1153::SecondaryCustomsReference),
            "AFN" => Some(Enum1153::AccountPartysReference),
            "AFO" => Some(Enum1153::BeneficiarysReference),
            "AFP" => Some(Enum1153::SecondBeneficiarysReference),
            "AFQ" => Some(Enum1153::ApplicantsBankReference),
            "AFR" => Some(Enum1153::IssuingBanksReference),
            "AFS" => Some(Enum1153::BeneficiarysBankReference),
            "AFT" => Some(Enum1153::DirectPaymentValuationNumber),
            "AFU" => Some(Enum1153::DirectPaymentValuationRequestNumber),
            "AFV" => Some(Enum1153::QuantityValuationNumber),
            "AFW" => Some(Enum1153::QuantityValuationRequestNumber),
            "AFX" => Some(Enum1153::BillQuantitiesNumber),
            "AFY" => Some(Enum1153::PaymentValuationNumber),
            "AFZ" => Some(Enum1153::SituationNumber),
            "AGA" => Some(Enum1153::AgreementToPayNumber),
            "AGB" => Some(Enum1153::ContractPartyReferenceNumber),
            "AGC" => Some(Enum1153::AccountPartysBankReference),
            "AGD" => Some(Enum1153::AgentsBankReference),
            "AGE" => Some(Enum1153::AgentsReference),
            "AGF" => Some(Enum1153::ApplicantsReference),
            "AGG" => Some(Enum1153::DisputeNumber),
            "AGH" => Some(Enum1153::CreditRatingAgencysReferenceNumber),
            "AGI" => Some(Enum1153::RequestNumber),
            "AGJ" => Some(Enum1153::SingleTransactionSequenceNumber),
            "AGK" => Some(Enum1153::ApplicationReferenceNumber),
            "AGL" => Some(Enum1153::DeliveryVerificationCertificate),
            "AGM" => Some(Enum1153::NumberTemporaryImportationDocument),
            "AGN" => Some(Enum1153::ReferenceNumberQuotedOnStatement),
            "AGO" => Some(Enum1153::SendersReferenceToOriginalMessage),
            "AGP" => Some(Enum1153::CompanyIssuedEquipmentId),
            "AGQ" => Some(Enum1153::DomesticFlightNumber),
            "AGR" => Some(Enum1153::InternationalFlightNumber),
            "AGS" => Some(Enum1153::EmployerIdentificationNumberServiceBureau),
            "AGT" => Some(Enum1153::ServiceGroupIdentificationNumber),
            "AGU" => Some(Enum1153::MemberNumber),
            "AGV" => Some(Enum1153::PreviousMemberNumber),
            "AGW" => Some(Enum1153::SchemePlanNumber),
            "AGX" => Some(Enum1153::PreviousSchemePlanNumber),
            "AGY" => Some(Enum1153::ReceivingPartysMemberIdentification),
            "AGZ" => Some(Enum1153::PayrollNumber),
            "AHA" => Some(Enum1153::PackagingSpecificationNumber),
            "AHB" => Some(Enum1153::AuthorityIssuedEquipmentIdentification),
            "AHC" => Some(Enum1153::TrainingFlightNumber),
            "AHD" => Some(Enum1153::FundCodeNumber),
            "AHE" => Some(Enum1153::SignalCodeNumber),
            "AHF" => Some(Enum1153::MajorForceProgramNumber),
            "AHG" => Some(Enum1153::NominationNumber),
            "AHH" => Some(Enum1153::LaboratoryRegistrationNumber),
            "AHI" => Some(Enum1153::TransportContractReferenceNumber),
            "AHJ" => Some(Enum1153::PayeesReferenceNumber),
            "AHK" => Some(Enum1153::PayersReferenceNumber),
            "AHL" => Some(Enum1153::CreditorsReferenceNumber),
            "AHM" => Some(Enum1153::DebtorsReferenceNumber),
            "AHN" => Some(Enum1153::JointVentureReferenceNumber),
            "AHO" => Some(Enum1153::ChamberCommerceRegistrationNumber),
            "AHP" => Some(Enum1153::TaxRegistrationNumber),
            "AHQ" => Some(Enum1153::WoolIdentificationNumber),
            "AHR" => Some(Enum1153::WoolTaxReferenceNumber),
            "AHS" => Some(Enum1153::MeatProcessingEstablishmentRegistrationNumber),
            "AHT" => Some(Enum1153::QuarantineTreatmentStatusReferenceNumber),
            "AHU" => Some(Enum1153::RequestForQuoteNumber),
            "AHV" => Some(Enum1153::ManualProcessingAuthorityNumber),
            "AHX" => Some(Enum1153::RateNoteNumber),
            "AHY" => Some(Enum1153::FreightForwarderNumber),
            "AHZ" => Some(Enum1153::CustomsReleaseCode),
            "AIA" => Some(Enum1153::ComplianceCodeNumber),
            "AIB" => Some(Enum1153::DepartmentTransportationBondNumber),
            "AIC" => Some(Enum1153::ExportEstablishmentNumber),
            "AID" => Some(Enum1153::CertificateConformity),
            "AIE" => Some(Enum1153::MinisterialCertificateHomologation),
            "AIF" => Some(Enum1153::PreviousDeliveryInstructionNumber),
            "AIG" => Some(Enum1153::PassportNumber),
            "AIH" => Some(Enum1153::CommonTransactionReferenceNumber),
            "AII" => Some(Enum1153::BanksCommonTransactionReferenceNumber),
            "AIJ" => Some(Enum1153::CustomersIndividualTransactionReferenceNumber),
            "AIK" => Some(Enum1153::BanksIndividualTransactionReferenceNumber),
            "AIL" => Some(Enum1153::CustomersCommonTransactionReferenceNumber),
            "AIM" => Some(Enum1153::IndividualTransactionReferenceNumber),
            "AIN" => Some(Enum1153::ProductSourcingAgreementNumber),
            "AIO" => Some(Enum1153::CustomsTranshipmentNumber),
            "AIP" => Some(Enum1153::CustomsPreferenceInquiryNumber),
            "AIQ" => Some(Enum1153::PackingPlantNumber),
            "AIR" => Some(Enum1153::OriginalCertificateNumber),
            "AIS" => Some(Enum1153::ProcessingPlantNumber),
            "AIT" => Some(Enum1153::SlaughterPlantNumber),
            "AIU" => Some(Enum1153::ChargeCardAccountNumber),
            "AIV" => Some(Enum1153::EventReferenceNumber),
            "AIW" => Some(Enum1153::TransportSectionReferenceNumber),
            "AIX" => Some(Enum1153::ReferredProductForMechanicalAnalysis),
            "AIY" => Some(Enum1153::ReferredProductForChemicalAnalysis),
            "AIZ" => Some(Enum1153::ConsolidatedInvoiceNumber),
            "AJA" => Some(Enum1153::PartReferenceIndicatorInADrawing),
            "AJB" => Some(Enum1153::USCodeFederalRegulationsCfr),
            "AJC" => Some(Enum1153::PurchasingActivityClauseNumber),
            "AJD" => Some(Enum1153::USDefenseFederalAcquisitionRegulationSupplement),
            "AJE" => Some(Enum1153::AgencyClauseNumber),
            "AJF" => Some(Enum1153::CircularPublicationNumber),
            "AJG" => Some(Enum1153::USFederalAcquisitionRegulation),
            "AJH" => Some(Enum1153::USGeneralServicesAdministrationRegulation),
            "AJI" => Some(Enum1153::USFederalInformationResourcesManagementRegulation),
            "AJJ" => Some(Enum1153::Paragraph),
            "AJK" => Some(Enum1153::SpecialInstructionsNumber),
            "AJL" => Some(Enum1153::SiteSpecificProceduresTermsAndConditionsNumber),
            "AJM" => Some(Enum1153::MasterSolicitationProceduresTermsAndConditions),
            "AJN" => Some(Enum1153::USDepartmentVeteransAffairsAcquisitionRegulation),
            "AJO" => Some(Enum1153::MilitaryInterdepartmentalPurchaseRequestMiprNumber),
            "AJP" => Some(Enum1153::ForeignMilitarySalesNumber),
            "AJQ" => Some(Enum1153::DefensePrioritiesAllocationSystemPriorityRating),
            "AJR" => Some(Enum1153::WageDeterminationNumber),
            "AJS" => Some(Enum1153::AgreementNumber),
            "AJT" => Some(Enum1153::StandardIndustryClassificationSicNumber),
            "AJU" => Some(Enum1153::EndItemNumber),
            "AJV" => Some(Enum1153::FederalSupplyScheduleItemNumber),
            "AJW" => Some(Enum1153::TechnicalDocumentNumber),
            "AJX" => Some(Enum1153::TechnicalOrderNumber),
            "AJY" => Some(Enum1153::Suffix),
            "AJZ" => Some(Enum1153::TransportationAccountNumber),
            "AKA" => Some(Enum1153::ContainerDispositionOrderReferenceNumber),
            "AKB" => Some(Enum1153::ContainerPrefix),
            "AKC" => Some(Enum1153::TransportEquipmentReturnReference),
            "AKD" => Some(Enum1153::TransportEquipmentSurveyReference),
            "AKE" => Some(Enum1153::TransportEquipmentSurveyReportNumber),
            "AKF" => Some(Enum1153::TransportEquipmentStuffingOrder),
            "AKG" => Some(Enum1153::VehicleIdentificationNumberVin),
            "AKH" => Some(Enum1153::GovernmentBillLading),
            "AKI" => Some(Enum1153::OrderingCustomersSecondReferenceNumber),
            "AKJ" => Some(Enum1153::DirectDebitReference),
            "AKK" => Some(Enum1153::MeterReadingAtBeginningDelivery),
            "AKL" => Some(Enum1153::MeterReadingAtEndDelivery),
            "AKM" => Some(Enum1153::ReplenishmentPurchaseOrderRangeStartNumber),
            "AKN" => Some(Enum1153::ThirdBanksReference),
            "AKO" => Some(Enum1153::ActionAuthorizationNumber),
            "AKP" => Some(Enum1153::AppropriationNumber),
            "AKQ" => Some(Enum1153::ProductChangeAuthorityNumber),
            "AKR" => Some(Enum1153::GeneralCargoConsignmentReferenceNumber),
            "AKS" => Some(Enum1153::CatalogueSequenceNumber),
            "AKT" => Some(Enum1153::ForwardingOrderNumber),
            "AKU" => Some(Enum1153::TransportEquipmentSurveyReferenceNumber),
            "AKV" => Some(Enum1153::LeaseContractReference),
            "AKW" => Some(Enum1153::TransportCostsReferenceNumber),
            "AKX" => Some(Enum1153::TransportEquipmentStrippingOrder),
            "AKY" => Some(Enum1153::PriorPolicyNumber),
            "AKZ" => Some(Enum1153::PolicyNumber),
            "ALA" => Some(Enum1153::ProcurementBudgetNumber),
            "ALB" => Some(Enum1153::DomesticInventoryManagementCode),
            "ALC" => Some(Enum1153::CustomerReferenceNumberAssignedToPreviousBalance),
            "ALD" => Some(Enum1153::PreviousCreditAdviceReferenceNumber),
            "ALE" => Some(Enum1153::ReportingFormNumber),
            "ALF" => Some(Enum1153::AuthorizationNumberForExceptionToDangerousGoods),
            "ALG" => Some(Enum1153::DangerousGoodsSecurityNumber),
            "ALH" => Some(Enum1153::DangerousGoodsTransportLicenceNumber),
            "ALI" => Some(Enum1153::PreviousRentalAgreementNumber),
            "ALJ" => Some(Enum1153::NextRentalAgreementReasonNumber),
            "ALK" => Some(Enum1153::ConsigneesInvoiceNumber),
            "ALL" => Some(Enum1153::MessageBatchNumber),
            "ALM" => Some(Enum1153::PreviousDeliveryScheduleNumber),
            "ALN" => Some(Enum1153::PhysicalInventoryRecountReferenceNumber),
            "ALO" => Some(Enum1153::ReceivingAdviceNumber),
            "ALP" => Some(Enum1153::ReturnableContainerReferenceNumber),
            "ALQ" => Some(Enum1153::ReturnsNoticeNumber),
            "ALR" => Some(Enum1153::SalesForecastNumber),
            "ALS" => Some(Enum1153::SalesReportNumber),
            "ALT" => Some(Enum1153::PreviousTaxControlNumber),
            "ALU" => Some(Enum1153::AgerdAerospaceGroundEquipmentRequirementDataNumber),
            "ALV" => Some(Enum1153::RegisteredCapitalReference),
            "ALW" => Some(Enum1153::StandardNumberInspectionDocument),
            "ALX" => Some(Enum1153::Model),
            "ALY" => Some(Enum1153::FinancialManagementReference),
            "ALZ" => Some(Enum1153::NotificationForCollectionNumberNoticol),
            "AMA" => Some(Enum1153::PreviousRequestForMeteredReadingReferenceNumber),
            "AMB" => Some(Enum1153::NextRentalAgreementNumber),
            "AMC" => Some(Enum1153::ReferenceNumberARequestForMeteredReading),
            "AMD" => Some(Enum1153::HasteningNumber),
            "AME" => Some(Enum1153::RepairDataRequestNumber),
            "AMF" => Some(Enum1153::ConsumptionDataRequestNumber),
            "AMG" => Some(Enum1153::ProfileNumber),
            "AMH" => Some(Enum1153::CaseNumber),
            "AMI" => Some(Enum1153::GovernmentQualityAssuranceAndControlLevelNumber),
            "AMJ" => Some(Enum1153::PaymentPlanReference),
            "AMK" => Some(Enum1153::ReplacedMeterUnitNumber),
            "AML" => Some(Enum1153::ReplenishmentPurchaseOrderRangeEndNumber),
            "AMM" => Some(Enum1153::InsurerAssignedReferenceNumber),
            "AMN" => Some(Enum1153::CanadianExciseEntryNumber),
            "AMO" => Some(Enum1153::PremiumRateTable),
            "AMP" => Some(Enum1153::AdviseThroughBanksReference),
            "AMQ" => Some(Enum1153::UsDepartmentTransportationBondSuretyCode),
            "AMR" => Some(Enum1153::UsFoodAndDrugAdministrationEstablishmentIndicator),
            "AMS" => Some(Enum1153::UsFederalCommunicationsCommissionFccImport),
            "AMT" => Some(Enum1153::GoodsAndServicesTaxIdentificationNumber),
            "AMU" => Some(Enum1153::IntegratedLogisticSupportCrossReferenceNumber),
            "AMV" => Some(Enum1153::DepartmentNumber),
            "AMW" => Some(Enum1153::BuyersCatalogueNumber),
            "AMX" => Some(Enum1153::FinancialSettlementPartysReferenceNumber),
            "AMY" => Some(Enum1153::StandardsVersionNumber),
            "AMZ" => Some(Enum1153::PipelineNumber),
            "ANA" => Some(Enum1153::AccountServicingBanksReferenceNumber),
            "ANB" => Some(Enum1153::CompletedUnitsPaymentRequestReference),
            "ANC" => Some(Enum1153::PaymentInAdvanceRequestReference),
            "AND" => Some(Enum1153::ParentFile),
            "ANE" => Some(Enum1153::SubFile),
            "ANF" => Some(Enum1153::CadFileLayerConvention),
            "ANG" => Some(Enum1153::TechnicalRegulation),
            "ANH" => Some(Enum1153::PlotFile),
            "ANI" => Some(Enum1153::FileConversionJournal),
            "ANJ" => Some(Enum1153::AuthorizationNumber),
            "ANK" => Some(Enum1153::ReferenceNumberAssignedByThirdParty),
            "ANL" => Some(Enum1153::DepositReferenceNumber),
            "ANM" => Some(Enum1153::NamedBanksReference),
            "ANN" => Some(Enum1153::DraweesReference),
            "ANO" => Some(Enum1153::CaseNeedPartysReference),
            "ANP" => Some(Enum1153::CollectingBanksReference),
            "ANQ" => Some(Enum1153::RemittingBanksReference),
            "ANR" => Some(Enum1153::PrincipalsBankReference),
            "ANS" => Some(Enum1153::PresentingBanksReference),
            "ANT" => Some(Enum1153::ConsigneesReference),
            "ANU" => Some(Enum1153::FinancialTransactionReferenceNumber),
            "ANV" => Some(Enum1153::CreditReferenceNumber),
            "ANW" => Some(Enum1153::ReceivingBanksAuthorizationNumber),
            "ANX" => Some(Enum1153::ClearingReference),
            "ANY" => Some(Enum1153::SendingBanksReferenceNumber),
            "AOA" => Some(Enum1153::DocumentaryPaymentReference),
            "AOD" => Some(Enum1153::AccountingFileReference),
            "AOE" => Some(Enum1153::SendersFileReferenceNumber),
            "AOF" => Some(Enum1153::ReceiversFileReferenceNumber),
            "AOG" => Some(Enum1153::SourceDocumentInternalReference),
            "AOH" => Some(Enum1153::PrincipalsReference),
            "AOI" => Some(Enum1153::DebitReferenceNumber),
            "AOJ" => Some(Enum1153::Calendar),
            "AOK" => Some(Enum1153::WorkShift),
            "AOL" => Some(Enum1153::WorkBreakdownStructure),
            "AOM" => Some(Enum1153::OrganisationBreakdownStructure),
            "AON" => Some(Enum1153::WorkTaskChargeNumber),
            "AOO" => Some(Enum1153::FunctionalWorkGroup),
            "AOP" => Some(Enum1153::WorkTeam),
            "AOQ" => Some(Enum1153::Department),
            "AOR" => Some(Enum1153::StatementWork),
            "AOS" => Some(Enum1153::WorkPackage),
            "AOT" => Some(Enum1153::PlanningPackage),
            "AOU" => Some(Enum1153::CostAccount),
            "AOV" => Some(Enum1153::WorkOrder),
            "AOW" => Some(Enum1153::TransportationControlNumberTcn),
            "AOX" => Some(Enum1153::ConstraintNotation),
            "AOY" => Some(Enum1153::EtermsReference),
            "AOZ" => Some(Enum1153::ImplementationVersionNumber),
            "AP" => Some(Enum1153::AccountsReceivableNumber),
            "APA" => Some(Enum1153::IncorporatedLegalReference),
            "APB" => Some(Enum1153::PaymentInstalmentReferenceNumber),
            "APC" => Some(Enum1153::EquipmentOwnerReferenceNumber),
            "APD" => Some(Enum1153::CedentsClaimNumber),
            "APE" => Some(Enum1153::ReinsurersClaimNumber),
            "APF" => Some(Enum1153::PriceSalesCatalogueResponseReferenceNumber),
            "APG" => Some(Enum1153::GeneralPurposeMessageReferenceNumber),
            "APH" => Some(Enum1153::InvoicingDataSheetReferenceNumber),
            "API" => Some(Enum1153::InventoryReportReferenceNumber),
            "APJ" => Some(Enum1153::CeilingFormulaReferenceNumber),
            "APK" => Some(Enum1153::PriceVariationFormulaReferenceNumber),
            "APL" => Some(Enum1153::ReferenceToAccountServicingBanksMessage),
            "APM" => Some(Enum1153::PartySequenceNumber),
            "APN" => Some(Enum1153::PurchasersRequestReference),
            "APO" => Some(Enum1153::ContractorRequestReference),
            "APP" => Some(Enum1153::AccidentReferenceNumber),
            "APQ" => Some(Enum1153::CommercialAccountSummaryReferenceNumber),
            "APR" => Some(Enum1153::ContractBreakdownReference),
            "APS" => Some(Enum1153::ContractorRegistrationNumber),
            "APT" => Some(Enum1153::ApplicableCoefficientIdentificationNumber),
            "APU" => Some(Enum1153::SpecialBudgetAccountNumber),
            "APV" => Some(Enum1153::AuthorisationForRepairReference),
            "APW" => Some(Enum1153::ManufacturerDefinedRepairRatesReference),
            "APX" => Some(Enum1153::OriginalSubmitterLogNumber),
            "APY" => Some(Enum1153::OriginalSubmitterParentDataMaintenanceRequestDmr),
            "APZ" => Some(Enum1153::OriginalSubmitterChildDataMaintenanceRequestDmr),
            "AQA" => Some(Enum1153::EntryPointAssessmentLogNumber),
            "AQB" => Some(Enum1153::EntryPointAssessmentLogNumberParentDmr),
            "AQC" => Some(Enum1153::EntryPointAssessmentLogNumberChildDmr),
            "AQD" => Some(Enum1153::DataStructureTag),
            "AQE" => Some(Enum1153::CentralSecretariatLogNumber),
            "AQF" => Some(Enum1153::CentralSecretariatLogNumberParentDataMaintenance),
            "AQG" => Some(Enum1153::CentralSecretariatLogNumberChildDataMaintenance),
            "AQH" => Some(Enum1153::InternationalAssessmentLogNumber),
            "AQI" => Some(Enum1153::InternationalAssessmentLogNumberParentData),
            "AQJ" => Some(Enum1153::InternationalAssessmentLogNumberChildDataMaintenance),
            "AQK" => Some(Enum1153::StatusReportNumber),
            "AQL" => Some(Enum1153::MessageDesignGroupNumber),
            "AQM" => Some(Enum1153::UsCustomsServiceUscsEntryCode),
            "AQN" => Some(Enum1153::BeginningJobSequenceNumber),
            "AQO" => Some(Enum1153::SendersClauseNumber),
            "AQP" => Some(Enum1153::DunAndBradstreetCanadas8DigitStandardIndustrial),
            "AQQ" => Some(Enum1153::ActivitePrincipaleExerceeApeIdentifier),
            "AQR" => Some(Enum1153::DunAndBradstreetUs8DigitStandardIndustrial),
            "AQS" => Some(Enum1153::NomenclatureActivityClassificationEconomyNace),
            "AQT" => Some(Enum1153::NormeActiviteFrancaiseNafIdentifier),
            "AQU" => Some(Enum1153::RegisteredContractorActivityType),
            "AQV" => Some(Enum1153::StatisticBundesAmtSbaIdentifier),
            "AQW" => Some(Enum1153::StateOrProvinceAssignedEntityIdentification),
            "AQX" => Some(Enum1153::InstituteSecurityAndFutureMarketDevelopmentIsfmd),
            "AQY" => Some(Enum1153::FileIdentificationNumber),
            "AQZ" => Some(Enum1153::BankruptcyProcedureNumber),
            "ARA" => Some(Enum1153::NationalGovernmentBusinessIdentificationNumber),
            "ARB" => Some(Enum1153::PriorDataUniversalNumberSystemDunsNumber),
            "ARC" => Some(Enum1153::CompaniesRegistryOfficeCroNumber),
            "ARD" => Some(Enum1153::CostaRicanJudicialNumber),
            "ARE" => Some(Enum1153::NumeroDeIdentificacionTributariaNit),
            "ARF" => Some(Enum1153::PatronNumber),
            "ARG" => Some(Enum1153::RegistroInformacionFiscalRifNumber),
            "ARH" => Some(Enum1153::RegistroUnicoDeContribuyenteRucNumber),
            "ARI" => Some(Enum1153::TokyoShokoResearchTsrBusinessIdentifier),
            "ARJ" => Some(Enum1153::PersonalIdentityCardNumber),
            "ARK" => Some(Enum1153::SystemeInformatiquePourLeRepertoireDesEntreprises),
            "ARL" => Some(Enum1153::SystemeInformatiquePourLeRepertoireDesEtablissements),
            "ARM" => Some(Enum1153::PublicationIssueNumber),
            "ARN" => Some(Enum1153::OriginalFilingNumber),
            "ARO" => Some(Enum1153::DocumentPageIdentifier),
            "ARP" => Some(Enum1153::PublicFilingRegistrationNumber),
            "ARQ" => Some(Enum1153::RegiristoFederalDeContribuyentes),
            "ARR" => Some(Enum1153::SocialSecurityNumber),
            "ARS" => Some(Enum1153::DocumentVolumeNumber),
            "ART" => Some(Enum1153::BookNumber),
            "ARU" => Some(Enum1153::StockExchangeCompanyIdentifier),
            "ARV" => Some(Enum1153::ImputationAccount),
            "ARW" => Some(Enum1153::FinancialPhaseReference),
            "ARX" => Some(Enum1153::TechnicalPhaseReference),
            "ARY" => Some(Enum1153::PriorContractorRegistrationNumber),
            "ARZ" => Some(Enum1153::StockAdjustmentNumber),
            "ASA" => Some(Enum1153::DispensationReference),
            "ASB" => Some(Enum1153::InvestmentReferenceNumber),
            "ASC" => Some(Enum1153::AssumingCompany),
            "ASD" => Some(Enum1153::BudgetChapter),
            "ASE" => Some(Enum1153::DutyFreeProductsSecurityNumber),
            "ASF" => Some(Enum1153::DutyFreeProductsReceiptAuthorisationNumber),
            "ASG" => Some(Enum1153::PartyInformationMessageReference),
            "ASH" => Some(Enum1153::FormalStatementReference),
            "ASI" => Some(Enum1153::ProofDeliveryReferenceNumber),
            "ASJ" => Some(Enum1153::SuppliersCreditClaimReferenceNumber),
            "ASK" => Some(Enum1153::PictureActualProduct),
            "ASL" => Some(Enum1153::PictureAGenericProduct),
            "ASM" => Some(Enum1153::TradingPartnerIdentificationNumber),
            "ASN" => Some(Enum1153::PriorTradingPartnerIdentificationNumber),
            "ASO" => Some(Enum1153::Password),
            "ASP" => Some(Enum1153::FormalReportNumber),
            "ASQ" => Some(Enum1153::FundAccountNumber),
            "ASR" => Some(Enum1153::SafeCustodyNumber),
            "ASS" => Some(Enum1153::MasterAccountNumber),
            "AST" => Some(Enum1153::GroupReferenceNumber),
            "ASU" => Some(Enum1153::AccountingTransmissionNumber),
            "ASV" => Some(Enum1153::ProductDataFileNumber),
            "ASW" => Some(Enum1153::CadastroGeralDoContribuinteCgc),
            "ASX" => Some(Enum1153::ForeignResidentIdentificationNumber),
            "ASY" => Some(Enum1153::CdRom),
            "ASZ" => Some(Enum1153::PhysicalMedium),
            "ATA" => Some(Enum1153::FinancialCancellationReferenceNumber),
            "ATB" => Some(Enum1153::PurchaseForExportCustomsAgreementNumber),
            "ATC" => Some(Enum1153::JudgmentNumber),
            "ATD" => Some(Enum1153::SecretariatNumber),
            "ATE" => Some(Enum1153::PreviousBankingStatusMessageReference),
            "ATF" => Some(Enum1153::LastReceivedBankingStatusMessageReference),
            "ATG" => Some(Enum1153::BanksDocumentaryProcedureReference),
            "ATH" => Some(Enum1153::CustomersDocumentaryProcedureReference),
            "ATI" => Some(Enum1153::SafeDepositBoxNumber),
            "ATJ" => Some(Enum1153::ReceivingBankgiroNumber),
            "ATK" => Some(Enum1153::SendingBankgiroNumber),
            "ATL" => Some(Enum1153::BankgiroReference),
            "ATM" => Some(Enum1153::GuaranteeNumber),
            "ATN" => Some(Enum1153::CollectionInstrumentNumber),
            "ATO" => Some(Enum1153::ConvertedPostgiroNumber),
            "ATP" => Some(Enum1153::CostCentreAlignmentNumber),
            "ATQ" => Some(Enum1153::KamerVanKoophandelKvkNumber),
            "ATR" => Some(Enum1153::InstitutBelgoLuxembourgeoisDeCodificationIblcNumber),
            "ATS" => Some(Enum1153::ExternalObjectReference),
            "ATT" => Some(Enum1153::ExceptionalTransportAuthorisationNumber),
            "ATU" => Some(Enum1153::ClaveUnicaDeIdentificacionTributariaCuit),
            "ATV" => Some(Enum1153::RegistroUnicoTributarioRut),
            "ATW" => Some(Enum1153::FlatRackContainerBundleIdentificationNumber),
            "ATX" => Some(Enum1153::TransportEquipmentAcceptanceOrderReference),
            "ATY" => Some(Enum1153::TransportEquipmentReleaseOrderReference),
            "ATZ" => Some(Enum1153::ShipsStayReferenceNumber),
            "AU" => Some(Enum1153::AuthorizationToMeetCompetitionNumber),
            "AUA" => Some(Enum1153::PlacePositioningReference),
            "AUB" => Some(Enum1153::PartyReference),
            "AUC" => Some(Enum1153::IssuedPrescriptionIdentification),
            "AUD" => Some(Enum1153::CollectionReference),
            "AUE" => Some(Enum1153::TravelService),
            "AUF" => Some(Enum1153::ConsignmentStockContract),
            "AUG" => Some(Enum1153::ImportersLetterCreditReference),
            "AUH" => Some(Enum1153::PerformedPrescriptionIdentification),
            "AUI" => Some(Enum1153::ImageReference),
            "AUJ" => Some(Enum1153::ProposedPurchaseOrderReferenceNumber),
            "AUK" => Some(Enum1153::ApplicationForFinancialSupportReferenceNumber),
            "AUL" => Some(Enum1153::ManufacturingQualityAgreementNumber),
            "AUM" => Some(Enum1153::SoftwareEditorReference),
            "AUN" => Some(Enum1153::SoftwareReference),
            "AUO" => Some(Enum1153::SoftwareQualityReference),
            "AUP" => Some(Enum1153::ConsolidatedOrdersReference),
            "AUQ" => Some(Enum1153::CustomsBindingRulingNumber),
            "AUR" => Some(Enum1153::CustomsNonBindingRulingNumber),
            "AUS" => Some(Enum1153::DeliveryRouteReference),
            "AUT" => Some(Enum1153::NetAreaSupplierReference),
            "AUU" => Some(Enum1153::TimeSeriesReference),
            "AUV" => Some(Enum1153::ConnectingPointToCentralGrid),
            "AUW" => Some(Enum1153::MarketingPlanIdentificationNumberMpin),
            "AUX" => Some(Enum1153::EntityReferenceNumberPrevious),
            "AUY" => Some(Enum1153::InternationalStandardIndustrialClassificationIsic),
            "AUZ" => Some(Enum1153::CustomsPreApprovalRulingNumber),
            "AV" => Some(Enum1153::AccountPayableNumber),
            "AVA" => Some(Enum1153::FirstFinancialInstitutionsTransactionReference),
            "AVB" => Some(Enum1153::ProductCharacteristicsDirectory),
            "AVC" => Some(Enum1153::SuppliersCustomerReferenceNumber),
            "AVD" => Some(Enum1153::InventoryReportRequestNumber),
            "AVE" => Some(Enum1153::MeteringPoint),
            "AVF" => Some(Enum1153::PassengerReservationNumber),
            "AVG" => Some(Enum1153::SlaughterhouseApprovalNumber),
            "AVH" => Some(Enum1153::MeatCuttingPlantApprovalNumber),
            "AVI" => Some(Enum1153::CustomerTravelServiceIdentifier),
            "AVJ" => Some(Enum1153::ExportControlClassificationNumber),
            "AVK" => Some(Enum1153::BrokerReference3),
            "AVL" => Some(Enum1153::ConsignmentInformation),
            "AVM" => Some(Enum1153::GoodsItemInformation),
            "AVN" => Some(Enum1153::DangerousGoodsInformation),
            "AVO" => Some(Enum1153::PilotageServicesExemptionNumber),
            "AVP" => Some(Enum1153::PersonRegistrationNumber),
            "AVQ" => Some(Enum1153::PlacePackingApprovalNumber),
            "AVR" => Some(Enum1153::OriginalMandateReference),
            "AVS" => Some(Enum1153::MandateReference),
            "AVT" => Some(Enum1153::ReservationStationIndentifier),
            "AVU" => Some(Enum1153::UniqueGoodsShipmentIdentifier),
            "AVV" => Some(Enum1153::FrameworkAgreementNumber),
            "AVW" => Some(Enum1153::HashValue),
            "AVX" => Some(Enum1153::MovementReferenceNumber),
            "AVY" => Some(Enum1153::EconomicOperatorsRegistrationAndIdentificationNumber),
            "AVZ" => Some(Enum1153::LocalReferenceNumber),
            "AWA" => Some(Enum1153::RateCodeNumber),
            "AWB" => Some(Enum1153::AirWaybillNumber),
            "AWC" => Some(Enum1153::DocumentaryCreditAmendmentNumber),
            "AWD" => Some(Enum1153::AdvisingBanksReference),
            "AWE" => Some(Enum1153::CostCentre),
            "AWF" => Some(Enum1153::WorkItemQuantityDetermination),
            "AWG" => Some(Enum1153::InternalDataProcessNumber),
            "AWH" => Some(Enum1153::CategoryWorkReference),
            "AWI" => Some(Enum1153::PolicyFormNumber),
            "AWJ" => Some(Enum1153::NetArea),
            "AWK" => Some(Enum1153::ServiceProvider),
            "AWL" => Some(Enum1153::ErrorPosition),
            "AWM" => Some(Enum1153::ServiceCategoryReference),
            "AWN" => Some(Enum1153::ConnectedLocation),
            "AWO" => Some(Enum1153::RelatedParty),
            "AWP" => Some(Enum1153::LatestAccountingEntryRecordReference),
            "AWQ" => Some(Enum1153::AccountingEntry),
            "AWR" => Some(Enum1153::DocumentReferenceOriginal),
            "AWS" => Some(Enum1153::HygienicCertificateNumberNational),
            "AWT" => Some(Enum1153::AdministrativeReferenceCode),
            "AWU" => Some(Enum1153::PickUpSheetNumber),
            "AWV" => Some(Enum1153::PhoneNumber),
            "AWW" => Some(Enum1153::BuyersFundNumber),
            "AWX" => Some(Enum1153::CompanyTradingAccountNumber),
            "AWY" => Some(Enum1153::ReservedGoodsIdentifier),
            "AWZ" => Some(Enum1153::HandlingAndMovementReferenceNumber),
            "AXA" => Some(Enum1153::InstructionToDespatchReferenceNumber),
            "AXB" => Some(Enum1153::InstructionForReturnsNumber),
            "AXC" => Some(Enum1153::MeteredServicesConsumptionReportNumber),
            "AXD" => Some(Enum1153::OrderStatusEnquiryNumber),
            "AXE" => Some(Enum1153::FirmBookingReferenceNumber),
            "AXF" => Some(Enum1153::ProductInquiryNumber),
            "AXG" => Some(Enum1153::SplitDeliveryNumber),
            "AXH" => Some(Enum1153::ServiceRelationNumber),
            "AXI" => Some(Enum1153::SerialShippingContainerCode),
            "AXJ" => Some(Enum1153::TestSpecificationNumber),
            "AXK" => Some(Enum1153::TransportStatusReportNumber),
            "AXL" => Some(Enum1153::ToolingContractNumber),
            "AXM" => Some(Enum1153::FormulaReferenceNumber),
            "AXN" => Some(Enum1153::PreAgreementNumber),
            "AXO" => Some(Enum1153::ProductCertificationNumber),
            "AXP" => Some(Enum1153::ConsignmentContractNumber),
            "AXQ" => Some(Enum1153::ProductSpecificationReferenceNumber),
            "AXR" => Some(Enum1153::PayrollDeductionAdviceReference),
            "AXS" => Some(Enum1153::TracesPartyIdentification),
            "AXU" => Some(Enum1153::BlockStowageReference),
            "BA" => Some(Enum1153::BeginningMeterReadingActual),
            "BC" => Some(Enum1153::BuyersContractNumber),
            "BD" => Some(Enum1153::BidNumber),
            "BE" => Some(Enum1153::BeginningMeterReadingEstimated),
            "BH" => Some(Enum1153::HouseBillLadingNumber),
            "BM" => Some(Enum1153::BillLadingNumber),
            "BN" => Some(Enum1153::ConsignmentIdentifierCarrierAssigned),
            "BO" => Some(Enum1153::BlanketOrderNumber),
            "BR" => Some(Enum1153::BrokerOrSalesOfficeNumber),
            "BT" => Some(Enum1153::BatchNumberLotNumber),
            "BTP" => Some(Enum1153::BatteryAndAccumulatorProducerRegistrationNumber),
            "BW" => Some(Enum1153::BlendedWithNumber),
            "CAS" => Some(Enum1153::IataCargoAgentCassAddressNumber),
            "CAT" => Some(Enum1153::MatchingEntriesBalanced),
            "CAU" => Some(Enum1153::EntryFlagging),
            "CAV" => Some(Enum1153::MatchingEntriesUnbalanced),
            "CAW" => Some(Enum1153::DocumentReferenceInternal),
            "CAX" => Some(Enum1153::EuropeanValueAddedTaxIdentification),
            "CAY" => Some(Enum1153::CostAccountingDocument),
            "CAZ" => Some(Enum1153::GridOperatorsCustomerReferenceNumber),
            "CBA" => Some(Enum1153::TicketControlNumber),
            "CBB" => Some(Enum1153::OrderShipmentGroupingReference),
            "CD" => Some(Enum1153::CreditNoteNumber),
            "CEC" => Some(Enum1153::CedingCompany),
            "CED" => Some(Enum1153::DebitLetterNumber),
            "CFE" => Some(Enum1153::ConsigneesFurtherOrder),
            "CFF" => Some(Enum1153::AnimalFarmLicenceNumber),
            "CFO" => Some(Enum1153::ConsignorsFurtherOrder),
            "CG" => Some(Enum1153::ConsigneesOrderNumber),
            "CH" => Some(Enum1153::CustomerCatalogueNumber),
            "CK" => Some(Enum1153::ChequeNumber),
            "CKN" => Some(Enum1153::CheckingNumber),
            "CM" => Some(Enum1153::CreditMemoNumber),
            "CMR" => Some(Enum1153::RoadConsignmentNoteNumber),
            "CN" => Some(Enum1153::CarriersReferenceNumber),
            "CNO" => Some(Enum1153::ChargesNoteDocumentAttachmentIndicator),
            "COF" => Some(Enum1153::CallOffOrderNumber),
            "CP" => Some(Enum1153::ConditionPurchaseDocumentNumber),
            "CR" => Some(Enum1153::CustomerReferenceNumber),
            "CRN" => Some(Enum1153::TransportMeansJourneyIdentifier),
            "CS" => Some(Enum1153::ConditionSaleDocumentNumber),
            "CST" => Some(Enum1153::TeamAssignmentNumber),
            "CT" => Some(Enum1153::ContractNumber),
            "CU" => Some(Enum1153::ConsignmentIdentifierConsignorAssigned),
            "CV" => Some(Enum1153::ContainerOperatorsReferenceNumber),
            "CW" => Some(Enum1153::PackageNumber),
            "CZ" => Some(Enum1153::CooperationContractNumber),
            "DA" => Some(Enum1153::DefermentApprovalNumber),
            "DAN" => Some(Enum1153::DebitAccountNumber),
            "DB" => Some(Enum1153::BuyersDebtorNumber),
            "DI" => Some(Enum1153::DistributorInvoiceNumber),
            "DL" => Some(Enum1153::DebitNoteNumber),
            "DM" => Some(Enum1153::DocumentIdentifier),
            "DQ" => Some(Enum1153::DeliveryNoteNumber),
            "DR" => Some(Enum1153::DockReceiptNumber),
            "EA" => Some(Enum1153::EndingMeterReadingActual),
            "EB" => Some(Enum1153::EmbargoPermitNumber),
            "ED" => Some(Enum1153::ExportDeclaration),
            "EE" => Some(Enum1153::EndingMeterReadingEstimated),
            "EEP" => Some(Enum1153::ElectricalAndElectronicEquipmentProducerRegistration),
            "EI" => Some(Enum1153::EmployersIdentificationNumber),
            "EN" => Some(Enum1153::EmbargoNumber),
            "EQ" => Some(Enum1153::EquipmentNumber),
            "ER" => Some(Enum1153::ContainerEquipmentReceiptNumber),
            "ERN" => Some(Enum1153::ExportersReferenceNumber),
            "ET" => Some(Enum1153::ExcessTransportationNumber),
            "EX" => Some(Enum1153::ExportPermitIdentifier),
            "FC" => Some(Enum1153::FiscalNumber),
            "FF" => Some(Enum1153::ConsignmentIdentifierFreightForwarderAssigned),
            "FI" => Some(Enum1153::FileLineIdentifier),
            "FLW" => Some(Enum1153::FlowReferenceNumber),
            "FN" => Some(Enum1153::FreightBillNumber),
            "FO" => Some(Enum1153::ForeignExchange),
            "FS" => Some(Enum1153::FinalSequenceNumber),
            "FT" => Some(Enum1153::FreeZoneIdentifier),
            "FV" => Some(Enum1153::FileVersionNumber),
            "FX" => Some(Enum1153::ForeignExchangeContractNumber),
            "GA" => Some(Enum1153::StandardsNumber),
            "GC" => Some(Enum1153::GovernmentContractNumber),
            "GD" => Some(Enum1153::StandardsCodeNumber),
            "GDN" => Some(Enum1153::GeneralDeclarationNumber),
            "GN" => Some(Enum1153::GovernmentReferenceNumber),
            "HS" => Some(Enum1153::HarmonisedSystemNumber),
            "HWB" => Some(Enum1153::HouseWaybillNumber),
            "IA" => Some(Enum1153::InternalVendorNumber),
            "IB" => Some(Enum1153::InBondNumber),
            "ICA" => Some(Enum1153::IataCargoAgentCodeNumber),
            "ICE" => Some(Enum1153::InsuranceCertificateReferenceNumber),
            "ICO" => Some(Enum1153::InsuranceContractReferenceNumber),
            "II" => Some(Enum1153::InitialSampleInspectionReportNumber),
            "IL" => Some(Enum1153::InternalOrderNumber),
            "INB" => Some(Enum1153::IntermediaryBroker),
            "INN" => Some(Enum1153::InterchangeNumberNew),
            "INO" => Some(Enum1153::InterchangeNumberOld),
            "IP" => Some(Enum1153::ImportPermitIdentifier),
            "IS" => Some(Enum1153::InvoiceNumberSuffix),
            "IT" => Some(Enum1153::InternalCustomerNumber),
            "IV" => Some(Enum1153::InvoiceDocumentIdentifier),
            "JB" => Some(Enum1153::JobNumber),
            "JE" => Some(Enum1153::EndingJobSequenceNumber),
            "LA" => Some(Enum1153::ShippingLabelSerialNumber),
            "LAN" => Some(Enum1153::LoadingAuthorisationIdentifier),
            "LAR" => Some(Enum1153::LowerNumberInRange),
            "LB" => Some(Enum1153::Lockbox),
            "LC" => Some(Enum1153::LetterCreditNumber),
            "LI" => Some(Enum1153::DocumentLineIdentifier),
            "LO" => Some(Enum1153::LoadPlanningNumber),
            "LRC" => Some(Enum1153::ReservationOfficeIdentifier),
            "LS" => Some(Enum1153::BarCodedLabelSerialNumber),
            "MA" => Some(Enum1153::ShipNoticeManifestNumber),
            "MB" => Some(Enum1153::MasterBillLadingNumber),
            "MF" => Some(Enum1153::ManufacturersPartNumber),
            "MG" => Some(Enum1153::MeterUnitNumber),
            "MH" => Some(Enum1153::ManufacturingOrderNumber),
            "MR" => Some(Enum1153::MessageRecipient),
            "MRN" => Some(Enum1153::MailingReferenceNumber),
            "MS" => Some(Enum1153::MessageSender),
            "MSS" => Some(Enum1153::ManufacturersMaterialSafetyDataSheetNumber),
            "MWB" => Some(Enum1153::MasterAirWaybillNumber),
            "NA" => Some(Enum1153::NorthAmericanHazardousGoodsClassificationNumber),
            "NF" => Some(Enum1153::NotaFiscal),
            "OH" => Some(Enum1153::CurrentInvoiceNumber),
            "OI" => Some(Enum1153::PreviousInvoiceNumber),
            "ON" => Some(Enum1153::OrderDocumentIdentifierBuyerAssigned),
            "OP" => Some(Enum1153::OriginalPurchaseOrder),
            "OR" => Some(Enum1153::GeneralOrderNumber),
            "PB" => Some(Enum1153::PayersFinancialInstitutionAccountNumber),
            "PC" => Some(Enum1153::ProductionCode),
            "PD" => Some(Enum1153::PromotionDealNumber),
            "PE" => Some(Enum1153::PlantNumber),
            "PF" => Some(Enum1153::PrimeContractorContractNumber),
            "PI" => Some(Enum1153::PriceListVersionNumber),
            "PK" => Some(Enum1153::PackingListNumber),
            "PL" => Some(Enum1153::PriceListNumber),
            "POR" => Some(Enum1153::PurchaseOrderResponseNumber),
            "PP" => Some(Enum1153::PurchaseOrderChangeNumber),
            "PQ" => Some(Enum1153::PaymentReference),
            "PR" => Some(Enum1153::PriceQuoteNumber),
            "PS" => Some(Enum1153::PurchaseOrderNumberSuffix),
            "PW" => Some(Enum1153::PriorPurchaseOrderNumber),
            "PY" => Some(Enum1153::PayeesFinancialInstitutionAccountNumber),
            "RA" => Some(Enum1153::RemittanceAdviceNumber),
            "RC" => Some(Enum1153::RailRoadRoutingCode),
            "RCN" => Some(Enum1153::RailwayConsignmentNoteNumber),
            "RE" => Some(Enum1153::ReleaseNumber),
            "REN" => Some(Enum1153::ConsignmentReceiptIdentifier),
            "RF" => Some(Enum1153::ExportReferenceNumber),
            "RR" => Some(Enum1153::PayersFinancialInstitutionTransitRoutingNoAch),
            "RT" => Some(Enum1153::PayeesFinancialInstitutionTransitRoutingNo),
            "SA" => Some(Enum1153::SalesPersonNumber),
            "SB" => Some(Enum1153::SalesRegionNumber),
            "SD" => Some(Enum1153::SalesDepartmentNumber),
            "SE" => Some(Enum1153::SerialNumber),
            "SEA" => Some(Enum1153::AllocatedSeat),
            "SF" => Some(Enum1153::ShipFrom),
            "SH" => Some(Enum1153::PreviousHighestScheduleNumber),
            "SI" => Some(Enum1153::SidShippersIdentifyingNumberForShipment),
            "SM" => Some(Enum1153::SalesOfficeNumber),
            "SN" => Some(Enum1153::TransportEquipmentSealIdentifier),
            "SP" => Some(Enum1153::ScanLine),
            "SQ" => Some(Enum1153::EquipmentSequenceNumber),
            "SRN" => Some(Enum1153::ShipmentReferenceNumber),
            "SS" => Some(Enum1153::SellersReferenceNumber),
            "STA" => Some(Enum1153::StationReferenceNumber),
            "SW" => Some(Enum1153::SwapOrderNumber),
            "SZ" => Some(Enum1153::SpecificationNumber),
            "TB" => Some(Enum1153::TruckersBillLading),
            "TCR" => Some(Enum1153::TerminalOperatorsConsignmentReference),
            "TE" => Some(Enum1153::TelexMessageNumber),
            "TF" => Some(Enum1153::TransferNumber),
            "TI" => Some(Enum1153::TirCarnetNumber),
            "TIN" => Some(Enum1153::TransportInstructionNumber),
            "TL" => Some(Enum1153::TaxExemptionLicenceNumber),
            "TN" => Some(Enum1153::TransactionReferenceNumber),
            "TP" => Some(Enum1153::TestReportNumber),
            "UAR" => Some(Enum1153::UpperNumberRange),
            "UC" => Some(Enum1153::UltimateCustomersReferenceNumber),
            "UCN" => Some(Enum1153::UniqueConsignmentReferenceNumber),
            "UN" => Some(Enum1153::UnitedNationsDangerousGoodsIdentifier),
            "UO" => Some(Enum1153::UltimateCustomersOrderNumber),
            "URI" => Some(Enum1153::UniformResourceIdentifier),
            "VA" => Some(Enum1153::VatRegistrationNumber),
            "VC" => Some(Enum1153::VendorContractNumber),
            "VGR" => Some(Enum1153::TransportEquipmentGrossMassVerificationReference),
            "VM" => Some(Enum1153::VesselIdentifier),
            "VN" => Some(Enum1153::OrderNumberVendor),
            "VON" => Some(Enum1153::VoyageNumber),
            "VOR" => Some(Enum1153::TransportEquipmentGrossMassVerificationOrderReference),
            "VP" => Some(Enum1153::VendorProductNumber),
            "VR" => Some(Enum1153::VendorIdNumber),
            "VS" => Some(Enum1153::VendorOrderNumberSuffix),
            "VT" => Some(Enum1153::MotorVehicleIdentificationNumber),
            "VV" => Some(Enum1153::VoucherNumber),
            "WE" => Some(Enum1153::WarehouseEntryNumber),
            "WM" => Some(Enum1153::WeightAgreementNumber),
            "WN" => Some(Enum1153::WellNumber),
            "WR" => Some(Enum1153::WarehouseReceiptNumber),
            "WS" => Some(Enum1153::WarehouseStorageLocationNumber),
            "WY" => Some(Enum1153::RailWaybillNumber),
            "XA" => Some(Enum1153::CompanyPlaceRegistrationNumber),
            "XC" => Some(Enum1153::CargoControlNumber),
            "XP" => Some(Enum1153::PreviousCargoControlNumber),
            "ZZZ" => Some(Enum1153::MutuallyDefinedReferenceNumber),
            _ => None,
        }
    }
}

// Start: (Version) TryFrom Enum1153 to crate::zugferd_2_3_2::Enum1153
impl std::convert::TryFrom<Enum1153> for crate::zugferd_2_3_2::Enum1153 {
    type Error = ErrFromEnum1153ToCrateZugferd232Enum1153;
    fn try_from(value: Enum1153) -> Result<Self, Self::Error> {
        match value {
            Enum1153::OrderAcknowledgementDocumentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::OrderAcknowledgementDocumentIdentifier),
            Enum1153::ProformaInvoiceDocumentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::ProformaInvoiceDocumentIdentifier),
            Enum1153::DocumentaryCreditIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::DocumentaryCreditIdentifier),
            Enum1153::ContractDocumentAddendumIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::ContractDocumentAddendumIdentifier),
            Enum1153::GoodsDeclarationNumber => Ok(crate::zugferd_2_3_2::Enum1153::GoodsDeclarationNumber),
            Enum1153::DebitCardNumber => Ok(crate::zugferd_2_3_2::Enum1153::DebitCardNumber),
            Enum1153::OfferNumber => Ok(crate::zugferd_2_3_2::Enum1153::OfferNumber),
            Enum1153::BanksBatchInterbankTransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::BanksBatchInterbankTransactionReferenceNumber),
            Enum1153::BanksIndividualInterbankTransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::BanksIndividualInterbankTransactionReferenceNumber),
            Enum1153::DeliveryOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::DeliveryOrderNumber),
            Enum1153::DespatchAdviceNumber => Ok(crate::zugferd_2_3_2::Enum1153::DespatchAdviceNumber),
            Enum1153::DrawingNumber => Ok(crate::zugferd_2_3_2::Enum1153::DrawingNumber),
            Enum1153::WaybillNumber => Ok(crate::zugferd_2_3_2::Enum1153::WaybillNumber),
            Enum1153::DeliveryScheduleNumber => Ok(crate::zugferd_2_3_2::Enum1153::DeliveryScheduleNumber),
            Enum1153::ConsignmentIdentifierConsigneeAssigned => Ok(crate::zugferd_2_3_2::Enum1153::ConsignmentIdentifierConsigneeAssigned),
            Enum1153::PartialShipmentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::PartialShipmentIdentifier),
            Enum1153::TransportEquipmentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentIdentifier),
            Enum1153::MunicipalityAssignedBusinessRegistryNumber => Ok(crate::zugferd_2_3_2::Enum1153::MunicipalityAssignedBusinessRegistryNumber),
            Enum1153::TransportContractDocumentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::TransportContractDocumentIdentifier),
            Enum1153::MasterLabelNumber => Ok(crate::zugferd_2_3_2::Enum1153::MasterLabelNumber),
            Enum1153::DespatchNoteDocumentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::DespatchNoteDocumentIdentifier),
            Enum1153::EnquiryNumber => Ok(crate::zugferd_2_3_2::Enum1153::EnquiryNumber),
            Enum1153::DocketNumber => Ok(crate::zugferd_2_3_2::Enum1153::DocketNumber),
            Enum1153::CivilActionNumber => Ok(crate::zugferd_2_3_2::Enum1153::CivilActionNumber),
            Enum1153::CarriersAgentReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CarriersAgentReferenceNumber),
            Enum1153::StandardCarrierAlphaCodeScacNumber => Ok(crate::zugferd_2_3_2::Enum1153::StandardCarrierAlphaCodeScacNumber),
            Enum1153::CustomsValuationDecisionNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsValuationDecisionNumber),
            Enum1153::EndUseAuthorizationNumber => Ok(crate::zugferd_2_3_2::Enum1153::EndUseAuthorizationNumber),
            Enum1153::AntiDumpingCaseNumber => Ok(crate::zugferd_2_3_2::Enum1153::AntiDumpingCaseNumber),
            Enum1153::CustomsTariffNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsTariffNumber),
            Enum1153::DeclarantsReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::DeclarantsReferenceNumber),
            Enum1153::RepairEstimateNumber => Ok(crate::zugferd_2_3_2::Enum1153::RepairEstimateNumber),
            Enum1153::CustomsDecisionRequestNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsDecisionRequestNumber),
            Enum1153::SubHouseBillLadingNumber => Ok(crate::zugferd_2_3_2::Enum1153::SubHouseBillLadingNumber),
            Enum1153::TaxPaymentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::TaxPaymentIdentifier),
            Enum1153::QuotaNumber => Ok(crate::zugferd_2_3_2::Enum1153::QuotaNumber),
            Enum1153::TransitOnwardCarriageGuaranteeBondNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransitOnwardCarriageGuaranteeBondNumber),
            Enum1153::CustomsGuaranteeNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsGuaranteeNumber),
            Enum1153::ReplacingPartNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReplacingPartNumber),
            Enum1153::SellersCatalogueNumber => Ok(crate::zugferd_2_3_2::Enum1153::SellersCatalogueNumber),
            Enum1153::OriginatorsReference => Ok(crate::zugferd_2_3_2::Enum1153::OriginatorsReference),
            Enum1153::DeclarantsCustomsIdentityNumber => Ok(crate::zugferd_2_3_2::Enum1153::DeclarantsCustomsIdentityNumber),
            Enum1153::ImporterReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ImporterReferenceNumber),
            Enum1153::ExportClearanceInstructionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ExportClearanceInstructionReferenceNumber),
            Enum1153::ImportClearanceInstructionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ImportClearanceInstructionReferenceNumber),
            Enum1153::GoodsDeclarationDocumentIdentifierCustoms => Ok(crate::zugferd_2_3_2::Enum1153::GoodsDeclarationDocumentIdentifierCustoms),
            Enum1153::ArticleNumber => Ok(crate::zugferd_2_3_2::Enum1153::ArticleNumber),
            Enum1153::IntraPlantRouting => Ok(crate::zugferd_2_3_2::Enum1153::IntraPlantRouting),
            Enum1153::StockKeepingUnitNumber => Ok(crate::zugferd_2_3_2::Enum1153::StockKeepingUnitNumber),
            Enum1153::TextElementIdentifierDeletionReference => Ok(crate::zugferd_2_3_2::Enum1153::TextElementIdentifierDeletionReference),
            Enum1153::AllotmentIdentificationAir => Ok(crate::zugferd_2_3_2::Enum1153::AllotmentIdentificationAir),
            Enum1153::VehicleLicenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::VehicleLicenceNumber),
            Enum1153::AirCargoTransferManifest => Ok(crate::zugferd_2_3_2::Enum1153::AirCargoTransferManifest),
            Enum1153::CargoAcceptanceOrderReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CargoAcceptanceOrderReferenceNumber),
            Enum1153::UsGovernmentAgencyNumber => Ok(crate::zugferd_2_3_2::Enum1153::UsGovernmentAgencyNumber),
            Enum1153::ShippingUnitIdentification => Ok(crate::zugferd_2_3_2::Enum1153::ShippingUnitIdentification),
            Enum1153::AdditionalReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::AdditionalReferenceNumber),
            Enum1153::RelatedDocumentNumber => Ok(crate::zugferd_2_3_2::Enum1153::RelatedDocumentNumber),
            Enum1153::AddresseeReference => Ok(crate::zugferd_2_3_2::Enum1153::AddresseeReference),
            Enum1153::AtaCarnetNumber => Ok(crate::zugferd_2_3_2::Enum1153::AtaCarnetNumber),
            Enum1153::PackagingUnitIdentification => Ok(crate::zugferd_2_3_2::Enum1153::PackagingUnitIdentification),
            Enum1153::OuterpackagingUnitIdentification => Ok(crate::zugferd_2_3_2::Enum1153::OuterpackagingUnitIdentification),
            Enum1153::CustomerMaterialSpecificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomerMaterialSpecificationNumber),
            Enum1153::BankReference => Ok(crate::zugferd_2_3_2::Enum1153::BankReference),
            Enum1153::PrincipalReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PrincipalReferenceNumber),
            Enum1153::CollectionAdviceDocumentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::CollectionAdviceDocumentIdentifier),
            Enum1153::IronChargeNumber => Ok(crate::zugferd_2_3_2::Enum1153::IronChargeNumber),
            Enum1153::HotRollNumber => Ok(crate::zugferd_2_3_2::Enum1153::HotRollNumber),
            Enum1153::ColdRollNumber => Ok(crate::zugferd_2_3_2::Enum1153::ColdRollNumber),
            Enum1153::RailwayWagonNumber => Ok(crate::zugferd_2_3_2::Enum1153::RailwayWagonNumber),
            Enum1153::UniqueClaimsReferenceNumberSender => Ok(crate::zugferd_2_3_2::Enum1153::UniqueClaimsReferenceNumberSender),
            Enum1153::LossEventNumber => Ok(crate::zugferd_2_3_2::Enum1153::LossEventNumber),
            Enum1153::EstimateOrderReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::EstimateOrderReferenceNumber),
            Enum1153::ReferenceNumberToPreviousMessage => Ok(crate::zugferd_2_3_2::Enum1153::ReferenceNumberToPreviousMessage),
            Enum1153::BankersAcceptance => Ok(crate::zugferd_2_3_2::Enum1153::BankersAcceptance),
            Enum1153::DutyMemoNumber => Ok(crate::zugferd_2_3_2::Enum1153::DutyMemoNumber),
            Enum1153::EquipmentTransportChargeNumber => Ok(crate::zugferd_2_3_2::Enum1153::EquipmentTransportChargeNumber),
            Enum1153::BuyersItemNumber => Ok(crate::zugferd_2_3_2::Enum1153::BuyersItemNumber),
            Enum1153::MaturedCertificateDeposit => Ok(crate::zugferd_2_3_2::Enum1153::MaturedCertificateDeposit),
            Enum1153::Loan => Ok(crate::zugferd_2_3_2::Enum1153::Loan),
            Enum1153::AnalysisNumberTestNumber => Ok(crate::zugferd_2_3_2::Enum1153::AnalysisNumberTestNumber),
            Enum1153::AccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::AccountNumber),
            Enum1153::TreatyNumber => Ok(crate::zugferd_2_3_2::Enum1153::TreatyNumber),
            Enum1153::CatastropheNumber => Ok(crate::zugferd_2_3_2::Enum1153::CatastropheNumber),
            Enum1153::BureauSigningStatementReference => Ok(crate::zugferd_2_3_2::Enum1153::BureauSigningStatementReference),
            Enum1153::CompanySyndicateReference1 => Ok(crate::zugferd_2_3_2::Enum1153::CompanySyndicateReference1),
            Enum1153::CompanySyndicateReference2 => Ok(crate::zugferd_2_3_2::Enum1153::CompanySyndicateReference2),
            Enum1153::OrderingCustomerConsignmentReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::OrderingCustomerConsignmentReferenceNumber),
            Enum1153::ShipownersAuthorizationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ShipownersAuthorizationNumber),
            Enum1153::InlandTransportOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::InlandTransportOrderNumber),
            Enum1153::ContainerWorkOrderReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ContainerWorkOrderReferenceNumber),
            Enum1153::StatementNumber => Ok(crate::zugferd_2_3_2::Enum1153::StatementNumber),
            Enum1153::UniqueMarketReference => Ok(crate::zugferd_2_3_2::Enum1153::UniqueMarketReference),
            Enum1153::GroupAccounting => Ok(crate::zugferd_2_3_2::Enum1153::GroupAccounting),
            Enum1153::BrokerReference1 => Ok(crate::zugferd_2_3_2::Enum1153::BrokerReference1),
            Enum1153::BrokerReference2 => Ok(crate::zugferd_2_3_2::Enum1153::BrokerReference2),
            Enum1153::LloydsClaimsOfficeReference => Ok(crate::zugferd_2_3_2::Enum1153::LloydsClaimsOfficeReference),
            Enum1153::SecureDeliveryTermsAndConditionsAgreementReference => Ok(crate::zugferd_2_3_2::Enum1153::SecureDeliveryTermsAndConditionsAgreementReference),
            Enum1153::ReportNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReportNumber),
            Enum1153::TraderAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::TraderAccountNumber),
            Enum1153::AuthorizationForExpenseAfeNumber => Ok(crate::zugferd_2_3_2::Enum1153::AuthorizationForExpenseAfeNumber),
            Enum1153::GovernmentAgencyReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::GovernmentAgencyReferenceNumber),
            Enum1153::AssemblyNumber => Ok(crate::zugferd_2_3_2::Enum1153::AssemblyNumber),
            Enum1153::SymbolNumber => Ok(crate::zugferd_2_3_2::Enum1153::SymbolNumber),
            Enum1153::CommodityNumber => Ok(crate::zugferd_2_3_2::Enum1153::CommodityNumber),
            Enum1153::Eur1CertificateNumber => Ok(crate::zugferd_2_3_2::Enum1153::Eur1CertificateNumber),
            Enum1153::CustomerProcessSpecificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomerProcessSpecificationNumber),
            Enum1153::CustomerSpecificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomerSpecificationNumber),
            Enum1153::ApplicableInstructionsOrStandards => Ok(crate::zugferd_2_3_2::Enum1153::ApplicableInstructionsOrStandards),
            Enum1153::RegistrationNumberPreviousCustomsDeclaration => Ok(crate::zugferd_2_3_2::Enum1153::RegistrationNumberPreviousCustomsDeclaration),
            Enum1153::PostEntryReference => Ok(crate::zugferd_2_3_2::Enum1153::PostEntryReference),
            Enum1153::PaymentOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::PaymentOrderNumber),
            Enum1153::DeliveryNumberTransport => Ok(crate::zugferd_2_3_2::Enum1153::DeliveryNumberTransport),
            Enum1153::TransportRoute => Ok(crate::zugferd_2_3_2::Enum1153::TransportRoute),
            Enum1153::CustomersUnitInventoryNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomersUnitInventoryNumber),
            Enum1153::ProductReservationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProductReservationNumber),
            Enum1153::ProjectNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProjectNumber),
            Enum1153::DrawingListNumber => Ok(crate::zugferd_2_3_2::Enum1153::DrawingListNumber),
            Enum1153::ProjectSpecificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProjectSpecificationNumber),
            Enum1153::PrimaryReference => Ok(crate::zugferd_2_3_2::Enum1153::PrimaryReference),
            Enum1153::RequestForCancellationNumber => Ok(crate::zugferd_2_3_2::Enum1153::RequestForCancellationNumber),
            Enum1153::SuppliersControlNumber => Ok(crate::zugferd_2_3_2::Enum1153::SuppliersControlNumber),
            Enum1153::ShippingNoteNumber => Ok(crate::zugferd_2_3_2::Enum1153::ShippingNoteNumber),
            Enum1153::EmptyContainerBillNumber => Ok(crate::zugferd_2_3_2::Enum1153::EmptyContainerBillNumber),
            Enum1153::NonNegotiableMaritimeTransportDocumentNumber => Ok(crate::zugferd_2_3_2::Enum1153::NonNegotiableMaritimeTransportDocumentNumber),
            Enum1153::SubstituteAirWaybillNumber => Ok(crate::zugferd_2_3_2::Enum1153::SubstituteAirWaybillNumber),
            Enum1153::DespatchNotePostParcelsNumber => Ok(crate::zugferd_2_3_2::Enum1153::DespatchNotePostParcelsNumber),
            Enum1153::AirlinesFlightIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::AirlinesFlightIdentificationNumber),
            Enum1153::ThroughBillLadingNumber => Ok(crate::zugferd_2_3_2::Enum1153::ThroughBillLadingNumber),
            Enum1153::CargoManifestNumber => Ok(crate::zugferd_2_3_2::Enum1153::CargoManifestNumber),
            Enum1153::BordereauNumber => Ok(crate::zugferd_2_3_2::Enum1153::BordereauNumber),
            Enum1153::CustomsItemNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsItemNumber),
            Enum1153::ExportControlCommodityNumberEccn => Ok(crate::zugferd_2_3_2::Enum1153::ExportControlCommodityNumberEccn),
            Enum1153::MarkingLabelReference => Ok(crate::zugferd_2_3_2::Enum1153::MarkingLabelReference),
            Enum1153::TariffNumber => Ok(crate::zugferd_2_3_2::Enum1153::TariffNumber),
            Enum1153::ReplenishmentPurchaseOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReplenishmentPurchaseOrderNumber),
            Enum1153::ImmediateTransportationNoForInBondMovement => Ok(crate::zugferd_2_3_2::Enum1153::ImmediateTransportationNoForInBondMovement),
            Enum1153::TransportationExportationNoForInBondMovement => Ok(crate::zugferd_2_3_2::Enum1153::TransportationExportationNoForInBondMovement),
            Enum1153::ImmediateExportationNoForInBondMovement => Ok(crate::zugferd_2_3_2::Enum1153::ImmediateExportationNoForInBondMovement),
            Enum1153::AssociatedInvoices => Ok(crate::zugferd_2_3_2::Enum1153::AssociatedInvoices),
            Enum1153::SecondaryCustomsReference => Ok(crate::zugferd_2_3_2::Enum1153::SecondaryCustomsReference),
            Enum1153::AccountPartysReference => Ok(crate::zugferd_2_3_2::Enum1153::AccountPartysReference),
            Enum1153::BeneficiarysReference => Ok(crate::zugferd_2_3_2::Enum1153::BeneficiarysReference),
            Enum1153::SecondBeneficiarysReference => Ok(crate::zugferd_2_3_2::Enum1153::SecondBeneficiarysReference),
            Enum1153::ApplicantsBankReference => Ok(crate::zugferd_2_3_2::Enum1153::ApplicantsBankReference),
            Enum1153::IssuingBanksReference => Ok(crate::zugferd_2_3_2::Enum1153::IssuingBanksReference),
            Enum1153::BeneficiarysBankReference => Ok(crate::zugferd_2_3_2::Enum1153::BeneficiarysBankReference),
            Enum1153::DirectPaymentValuationNumber => Ok(crate::zugferd_2_3_2::Enum1153::DirectPaymentValuationNumber),
            Enum1153::DirectPaymentValuationRequestNumber => Ok(crate::zugferd_2_3_2::Enum1153::DirectPaymentValuationRequestNumber),
            Enum1153::QuantityValuationNumber => Ok(crate::zugferd_2_3_2::Enum1153::QuantityValuationNumber),
            Enum1153::QuantityValuationRequestNumber => Ok(crate::zugferd_2_3_2::Enum1153::QuantityValuationRequestNumber),
            Enum1153::BillQuantitiesNumber => Ok(crate::zugferd_2_3_2::Enum1153::BillQuantitiesNumber),
            Enum1153::PaymentValuationNumber => Ok(crate::zugferd_2_3_2::Enum1153::PaymentValuationNumber),
            Enum1153::SituationNumber => Ok(crate::zugferd_2_3_2::Enum1153::SituationNumber),
            Enum1153::AgreementToPayNumber => Ok(crate::zugferd_2_3_2::Enum1153::AgreementToPayNumber),
            Enum1153::ContractPartyReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ContractPartyReferenceNumber),
            Enum1153::AccountPartysBankReference => Ok(crate::zugferd_2_3_2::Enum1153::AccountPartysBankReference),
            Enum1153::AgentsBankReference => Ok(crate::zugferd_2_3_2::Enum1153::AgentsBankReference),
            Enum1153::AgentsReference => Ok(crate::zugferd_2_3_2::Enum1153::AgentsReference),
            Enum1153::ApplicantsReference => Ok(crate::zugferd_2_3_2::Enum1153::ApplicantsReference),
            Enum1153::DisputeNumber => Ok(crate::zugferd_2_3_2::Enum1153::DisputeNumber),
            Enum1153::CreditRatingAgencysReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CreditRatingAgencysReferenceNumber),
            Enum1153::RequestNumber => Ok(crate::zugferd_2_3_2::Enum1153::RequestNumber),
            Enum1153::SingleTransactionSequenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::SingleTransactionSequenceNumber),
            Enum1153::ApplicationReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ApplicationReferenceNumber),
            Enum1153::DeliveryVerificationCertificate => Ok(crate::zugferd_2_3_2::Enum1153::DeliveryVerificationCertificate),
            Enum1153::NumberTemporaryImportationDocument => Ok(crate::zugferd_2_3_2::Enum1153::NumberTemporaryImportationDocument),
            Enum1153::ReferenceNumberQuotedOnStatement => Ok(crate::zugferd_2_3_2::Enum1153::ReferenceNumberQuotedOnStatement),
            Enum1153::SendersReferenceToOriginalMessage => Ok(crate::zugferd_2_3_2::Enum1153::SendersReferenceToOriginalMessage),
            Enum1153::CompanyIssuedEquipmentId => Ok(crate::zugferd_2_3_2::Enum1153::CompanyIssuedEquipmentId),
            Enum1153::DomesticFlightNumber => Ok(crate::zugferd_2_3_2::Enum1153::DomesticFlightNumber),
            Enum1153::InternationalFlightNumber => Ok(crate::zugferd_2_3_2::Enum1153::InternationalFlightNumber),
            Enum1153::EmployerIdentificationNumberServiceBureau => Ok(crate::zugferd_2_3_2::Enum1153::EmployerIdentificationNumberServiceBureau),
            Enum1153::ServiceGroupIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ServiceGroupIdentificationNumber),
            Enum1153::MemberNumber => Ok(crate::zugferd_2_3_2::Enum1153::MemberNumber),
            Enum1153::PreviousMemberNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousMemberNumber),
            Enum1153::SchemePlanNumber => Ok(crate::zugferd_2_3_2::Enum1153::SchemePlanNumber),
            Enum1153::PreviousSchemePlanNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousSchemePlanNumber),
            Enum1153::ReceivingPartysMemberIdentification => Ok(crate::zugferd_2_3_2::Enum1153::ReceivingPartysMemberIdentification),
            Enum1153::PayrollNumber => Ok(crate::zugferd_2_3_2::Enum1153::PayrollNumber),
            Enum1153::PackagingSpecificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::PackagingSpecificationNumber),
            Enum1153::AuthorityIssuedEquipmentIdentification => Ok(crate::zugferd_2_3_2::Enum1153::AuthorityIssuedEquipmentIdentification),
            Enum1153::TrainingFlightNumber => Ok(crate::zugferd_2_3_2::Enum1153::TrainingFlightNumber),
            Enum1153::FundCodeNumber => Ok(crate::zugferd_2_3_2::Enum1153::FundCodeNumber),
            Enum1153::SignalCodeNumber => Ok(crate::zugferd_2_3_2::Enum1153::SignalCodeNumber),
            Enum1153::MajorForceProgramNumber => Ok(crate::zugferd_2_3_2::Enum1153::MajorForceProgramNumber),
            Enum1153::NominationNumber => Ok(crate::zugferd_2_3_2::Enum1153::NominationNumber),
            Enum1153::LaboratoryRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::LaboratoryRegistrationNumber),
            Enum1153::TransportContractReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransportContractReferenceNumber),
            Enum1153::PayeesReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PayeesReferenceNumber),
            Enum1153::PayersReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PayersReferenceNumber),
            Enum1153::CreditorsReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CreditorsReferenceNumber),
            Enum1153::DebtorsReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::DebtorsReferenceNumber),
            Enum1153::JointVentureReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::JointVentureReferenceNumber),
            Enum1153::ChamberCommerceRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ChamberCommerceRegistrationNumber),
            Enum1153::TaxRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::TaxRegistrationNumber),
            Enum1153::WoolIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::WoolIdentificationNumber),
            Enum1153::WoolTaxReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::WoolTaxReferenceNumber),
            Enum1153::MeatProcessingEstablishmentRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::MeatProcessingEstablishmentRegistrationNumber),
            Enum1153::QuarantineTreatmentStatusReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::QuarantineTreatmentStatusReferenceNumber),
            Enum1153::RequestForQuoteNumber => Ok(crate::zugferd_2_3_2::Enum1153::RequestForQuoteNumber),
            Enum1153::ManualProcessingAuthorityNumber => Ok(crate::zugferd_2_3_2::Enum1153::ManualProcessingAuthorityNumber),
            Enum1153::RateNoteNumber => Ok(crate::zugferd_2_3_2::Enum1153::RateNoteNumber),
            Enum1153::FreightForwarderNumber => Ok(crate::zugferd_2_3_2::Enum1153::FreightForwarderNumber),
            Enum1153::CustomsReleaseCode => Ok(crate::zugferd_2_3_2::Enum1153::CustomsReleaseCode),
            Enum1153::ComplianceCodeNumber => Ok(crate::zugferd_2_3_2::Enum1153::ComplianceCodeNumber),
            Enum1153::DepartmentTransportationBondNumber => Ok(crate::zugferd_2_3_2::Enum1153::DepartmentTransportationBondNumber),
            Enum1153::ExportEstablishmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::ExportEstablishmentNumber),
            Enum1153::CertificateConformity => Ok(crate::zugferd_2_3_2::Enum1153::CertificateConformity),
            Enum1153::MinisterialCertificateHomologation => Ok(crate::zugferd_2_3_2::Enum1153::MinisterialCertificateHomologation),
            Enum1153::PreviousDeliveryInstructionNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousDeliveryInstructionNumber),
            Enum1153::PassportNumber => Ok(crate::zugferd_2_3_2::Enum1153::PassportNumber),
            Enum1153::CommonTransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CommonTransactionReferenceNumber),
            Enum1153::BanksCommonTransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::BanksCommonTransactionReferenceNumber),
            Enum1153::CustomersIndividualTransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomersIndividualTransactionReferenceNumber),
            Enum1153::BanksIndividualTransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::BanksIndividualTransactionReferenceNumber),
            Enum1153::CustomersCommonTransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomersCommonTransactionReferenceNumber),
            Enum1153::IndividualTransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::IndividualTransactionReferenceNumber),
            Enum1153::ProductSourcingAgreementNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProductSourcingAgreementNumber),
            Enum1153::CustomsTranshipmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsTranshipmentNumber),
            Enum1153::CustomsPreferenceInquiryNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsPreferenceInquiryNumber),
            Enum1153::PackingPlantNumber => Ok(crate::zugferd_2_3_2::Enum1153::PackingPlantNumber),
            Enum1153::OriginalCertificateNumber => Ok(crate::zugferd_2_3_2::Enum1153::OriginalCertificateNumber),
            Enum1153::ProcessingPlantNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProcessingPlantNumber),
            Enum1153::SlaughterPlantNumber => Ok(crate::zugferd_2_3_2::Enum1153::SlaughterPlantNumber),
            Enum1153::ChargeCardAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::ChargeCardAccountNumber),
            Enum1153::EventReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::EventReferenceNumber),
            Enum1153::TransportSectionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransportSectionReferenceNumber),
            Enum1153::ReferredProductForMechanicalAnalysis => Ok(crate::zugferd_2_3_2::Enum1153::ReferredProductForMechanicalAnalysis),
            Enum1153::ReferredProductForChemicalAnalysis => Ok(crate::zugferd_2_3_2::Enum1153::ReferredProductForChemicalAnalysis),
            Enum1153::ConsolidatedInvoiceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ConsolidatedInvoiceNumber),
            Enum1153::PartReferenceIndicatorInADrawing => Ok(crate::zugferd_2_3_2::Enum1153::PartReferenceIndicatorInADrawing),
            Enum1153::USCodeFederalRegulationsCfr => Ok(crate::zugferd_2_3_2::Enum1153::USCodeFederalRegulationsCfr),
            Enum1153::PurchasingActivityClauseNumber => Ok(crate::zugferd_2_3_2::Enum1153::PurchasingActivityClauseNumber),
            Enum1153::USDefenseFederalAcquisitionRegulationSupplement => Ok(crate::zugferd_2_3_2::Enum1153::USDefenseFederalAcquisitionRegulationSupplement),
            Enum1153::AgencyClauseNumber => Ok(crate::zugferd_2_3_2::Enum1153::AgencyClauseNumber),
            Enum1153::CircularPublicationNumber => Ok(crate::zugferd_2_3_2::Enum1153::CircularPublicationNumber),
            Enum1153::USFederalAcquisitionRegulation => Ok(crate::zugferd_2_3_2::Enum1153::USFederalAcquisitionRegulation),
            Enum1153::USGeneralServicesAdministrationRegulation => Ok(crate::zugferd_2_3_2::Enum1153::USGeneralServicesAdministrationRegulation),
            Enum1153::USFederalInformationResourcesManagementRegulation => Ok(crate::zugferd_2_3_2::Enum1153::USFederalInformationResourcesManagementRegulation),
            Enum1153::Paragraph => Ok(crate::zugferd_2_3_2::Enum1153::Paragraph),
            Enum1153::SpecialInstructionsNumber => Ok(crate::zugferd_2_3_2::Enum1153::SpecialInstructionsNumber),
            Enum1153::SiteSpecificProceduresTermsAndConditionsNumber => Ok(crate::zugferd_2_3_2::Enum1153::SiteSpecificProceduresTermsAndConditionsNumber),
            Enum1153::MasterSolicitationProceduresTermsAndConditions => Ok(crate::zugferd_2_3_2::Enum1153::MasterSolicitationProceduresTermsAndConditions),
            Enum1153::USDepartmentVeteransAffairsAcquisitionRegulation => Ok(crate::zugferd_2_3_2::Enum1153::USDepartmentVeteransAffairsAcquisitionRegulation),
            Enum1153::MilitaryInterdepartmentalPurchaseRequestMiprNumber => Ok(crate::zugferd_2_3_2::Enum1153::MilitaryInterdepartmentalPurchaseRequestMiprNumber),
            Enum1153::ForeignMilitarySalesNumber => Ok(crate::zugferd_2_3_2::Enum1153::ForeignMilitarySalesNumber),
            Enum1153::DefensePrioritiesAllocationSystemPriorityRating => Ok(crate::zugferd_2_3_2::Enum1153::DefensePrioritiesAllocationSystemPriorityRating),
            Enum1153::WageDeterminationNumber => Ok(crate::zugferd_2_3_2::Enum1153::WageDeterminationNumber),
            Enum1153::AgreementNumber => Ok(crate::zugferd_2_3_2::Enum1153::AgreementNumber),
            Enum1153::StandardIndustryClassificationSicNumber => Ok(crate::zugferd_2_3_2::Enum1153::StandardIndustryClassificationSicNumber),
            Enum1153::EndItemNumber => Ok(crate::zugferd_2_3_2::Enum1153::EndItemNumber),
            Enum1153::FederalSupplyScheduleItemNumber => Ok(crate::zugferd_2_3_2::Enum1153::FederalSupplyScheduleItemNumber),
            Enum1153::TechnicalDocumentNumber => Ok(crate::zugferd_2_3_2::Enum1153::TechnicalDocumentNumber),
            Enum1153::TechnicalOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::TechnicalOrderNumber),
            Enum1153::Suffix => Ok(crate::zugferd_2_3_2::Enum1153::Suffix),
            Enum1153::TransportationAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransportationAccountNumber),
            Enum1153::ContainerDispositionOrderReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ContainerDispositionOrderReferenceNumber),
            Enum1153::ContainerPrefix => Ok(crate::zugferd_2_3_2::Enum1153::ContainerPrefix),
            Enum1153::TransportEquipmentReturnReference => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentReturnReference),
            Enum1153::TransportEquipmentSurveyReference => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentSurveyReference),
            Enum1153::TransportEquipmentSurveyReportNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentSurveyReportNumber),
            Enum1153::TransportEquipmentStuffingOrder => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentStuffingOrder),
            Enum1153::VehicleIdentificationNumberVin => Ok(crate::zugferd_2_3_2::Enum1153::VehicleIdentificationNumberVin),
            Enum1153::GovernmentBillLading => Ok(crate::zugferd_2_3_2::Enum1153::GovernmentBillLading),
            Enum1153::OrderingCustomersSecondReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::OrderingCustomersSecondReferenceNumber),
            Enum1153::DirectDebitReference => Ok(crate::zugferd_2_3_2::Enum1153::DirectDebitReference),
            Enum1153::MeterReadingAtBeginningDelivery => Ok(crate::zugferd_2_3_2::Enum1153::MeterReadingAtBeginningDelivery),
            Enum1153::MeterReadingAtEndDelivery => Ok(crate::zugferd_2_3_2::Enum1153::MeterReadingAtEndDelivery),
            Enum1153::ReplenishmentPurchaseOrderRangeStartNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReplenishmentPurchaseOrderRangeStartNumber),
            Enum1153::ThirdBanksReference => Ok(crate::zugferd_2_3_2::Enum1153::ThirdBanksReference),
            Enum1153::ActionAuthorizationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ActionAuthorizationNumber),
            Enum1153::AppropriationNumber => Ok(crate::zugferd_2_3_2::Enum1153::AppropriationNumber),
            Enum1153::ProductChangeAuthorityNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProductChangeAuthorityNumber),
            Enum1153::GeneralCargoConsignmentReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::GeneralCargoConsignmentReferenceNumber),
            Enum1153::CatalogueSequenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CatalogueSequenceNumber),
            Enum1153::ForwardingOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::ForwardingOrderNumber),
            Enum1153::TransportEquipmentSurveyReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentSurveyReferenceNumber),
            Enum1153::LeaseContractReference => Ok(crate::zugferd_2_3_2::Enum1153::LeaseContractReference),
            Enum1153::TransportCostsReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransportCostsReferenceNumber),
            Enum1153::TransportEquipmentStrippingOrder => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentStrippingOrder),
            Enum1153::PriorPolicyNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriorPolicyNumber),
            Enum1153::PolicyNumber => Ok(crate::zugferd_2_3_2::Enum1153::PolicyNumber),
            Enum1153::ProcurementBudgetNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProcurementBudgetNumber),
            Enum1153::DomesticInventoryManagementCode => Ok(crate::zugferd_2_3_2::Enum1153::DomesticInventoryManagementCode),
            Enum1153::CustomerReferenceNumberAssignedToPreviousBalance => Ok(crate::zugferd_2_3_2::Enum1153::CustomerReferenceNumberAssignedToPreviousBalance),
            Enum1153::PreviousCreditAdviceReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousCreditAdviceReferenceNumber),
            Enum1153::ReportingFormNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReportingFormNumber),
            Enum1153::AuthorizationNumberForExceptionToDangerousGoods => Ok(crate::zugferd_2_3_2::Enum1153::AuthorizationNumberForExceptionToDangerousGoods),
            Enum1153::DangerousGoodsSecurityNumber => Ok(crate::zugferd_2_3_2::Enum1153::DangerousGoodsSecurityNumber),
            Enum1153::DangerousGoodsTransportLicenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::DangerousGoodsTransportLicenceNumber),
            Enum1153::PreviousRentalAgreementNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousRentalAgreementNumber),
            Enum1153::NextRentalAgreementReasonNumber => Ok(crate::zugferd_2_3_2::Enum1153::NextRentalAgreementReasonNumber),
            Enum1153::ConsigneesInvoiceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ConsigneesInvoiceNumber),
            Enum1153::MessageBatchNumber => Ok(crate::zugferd_2_3_2::Enum1153::MessageBatchNumber),
            Enum1153::PreviousDeliveryScheduleNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousDeliveryScheduleNumber),
            Enum1153::PhysicalInventoryRecountReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PhysicalInventoryRecountReferenceNumber),
            Enum1153::ReceivingAdviceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReceivingAdviceNumber),
            Enum1153::ReturnableContainerReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReturnableContainerReferenceNumber),
            Enum1153::ReturnsNoticeNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReturnsNoticeNumber),
            Enum1153::SalesForecastNumber => Ok(crate::zugferd_2_3_2::Enum1153::SalesForecastNumber),
            Enum1153::SalesReportNumber => Ok(crate::zugferd_2_3_2::Enum1153::SalesReportNumber),
            Enum1153::PreviousTaxControlNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousTaxControlNumber),
            Enum1153::AgerdAerospaceGroundEquipmentRequirementDataNumber => Ok(crate::zugferd_2_3_2::Enum1153::AgerdAerospaceGroundEquipmentRequirementDataNumber),
            Enum1153::RegisteredCapitalReference => Ok(crate::zugferd_2_3_2::Enum1153::RegisteredCapitalReference),
            Enum1153::StandardNumberInspectionDocument => Ok(crate::zugferd_2_3_2::Enum1153::StandardNumberInspectionDocument),
            Enum1153::Model => Ok(crate::zugferd_2_3_2::Enum1153::Model),
            Enum1153::FinancialManagementReference => Ok(crate::zugferd_2_3_2::Enum1153::FinancialManagementReference),
            Enum1153::NotificationForCollectionNumberNoticol => Ok(crate::zugferd_2_3_2::Enum1153::NotificationForCollectionNumberNoticol),
            Enum1153::PreviousRequestForMeteredReadingReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousRequestForMeteredReadingReferenceNumber),
            Enum1153::NextRentalAgreementNumber => Ok(crate::zugferd_2_3_2::Enum1153::NextRentalAgreementNumber),
            Enum1153::ReferenceNumberARequestForMeteredReading => Ok(crate::zugferd_2_3_2::Enum1153::ReferenceNumberARequestForMeteredReading),
            Enum1153::HasteningNumber => Ok(crate::zugferd_2_3_2::Enum1153::HasteningNumber),
            Enum1153::RepairDataRequestNumber => Ok(crate::zugferd_2_3_2::Enum1153::RepairDataRequestNumber),
            Enum1153::ConsumptionDataRequestNumber => Ok(crate::zugferd_2_3_2::Enum1153::ConsumptionDataRequestNumber),
            Enum1153::ProfileNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProfileNumber),
            Enum1153::CaseNumber => Ok(crate::zugferd_2_3_2::Enum1153::CaseNumber),
            Enum1153::GovernmentQualityAssuranceAndControlLevelNumber => Ok(crate::zugferd_2_3_2::Enum1153::GovernmentQualityAssuranceAndControlLevelNumber),
            Enum1153::PaymentPlanReference => Ok(crate::zugferd_2_3_2::Enum1153::PaymentPlanReference),
            Enum1153::ReplacedMeterUnitNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReplacedMeterUnitNumber),
            Enum1153::ReplenishmentPurchaseOrderRangeEndNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReplenishmentPurchaseOrderRangeEndNumber),
            Enum1153::InsurerAssignedReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::InsurerAssignedReferenceNumber),
            Enum1153::CanadianExciseEntryNumber => Ok(crate::zugferd_2_3_2::Enum1153::CanadianExciseEntryNumber),
            Enum1153::PremiumRateTable => Ok(crate::zugferd_2_3_2::Enum1153::PremiumRateTable),
            Enum1153::AdviseThroughBanksReference => Ok(crate::zugferd_2_3_2::Enum1153::AdviseThroughBanksReference),
            Enum1153::UsDepartmentTransportationBondSuretyCode => Ok(crate::zugferd_2_3_2::Enum1153::UsDepartmentTransportationBondSuretyCode),
            Enum1153::UsFoodAndDrugAdministrationEstablishmentIndicator => Ok(crate::zugferd_2_3_2::Enum1153::UsFoodAndDrugAdministrationEstablishmentIndicator),
            Enum1153::UsFederalCommunicationsCommissionFccImport => Ok(crate::zugferd_2_3_2::Enum1153::UsFederalCommunicationsCommissionFccImport),
            Enum1153::GoodsAndServicesTaxIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::GoodsAndServicesTaxIdentificationNumber),
            Enum1153::IntegratedLogisticSupportCrossReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::IntegratedLogisticSupportCrossReferenceNumber),
            Enum1153::DepartmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::DepartmentNumber),
            Enum1153::BuyersCatalogueNumber => Ok(crate::zugferd_2_3_2::Enum1153::BuyersCatalogueNumber),
            Enum1153::FinancialSettlementPartysReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::FinancialSettlementPartysReferenceNumber),
            Enum1153::StandardsVersionNumber => Ok(crate::zugferd_2_3_2::Enum1153::StandardsVersionNumber),
            Enum1153::PipelineNumber => Ok(crate::zugferd_2_3_2::Enum1153::PipelineNumber),
            Enum1153::AccountServicingBanksReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::AccountServicingBanksReferenceNumber),
            Enum1153::CompletedUnitsPaymentRequestReference => Ok(crate::zugferd_2_3_2::Enum1153::CompletedUnitsPaymentRequestReference),
            Enum1153::PaymentInAdvanceRequestReference => Ok(crate::zugferd_2_3_2::Enum1153::PaymentInAdvanceRequestReference),
            Enum1153::ParentFile => Ok(crate::zugferd_2_3_2::Enum1153::ParentFile),
            Enum1153::SubFile => Ok(crate::zugferd_2_3_2::Enum1153::SubFile),
            Enum1153::CadFileLayerConvention => Ok(crate::zugferd_2_3_2::Enum1153::CadFileLayerConvention),
            Enum1153::TechnicalRegulation => Ok(crate::zugferd_2_3_2::Enum1153::TechnicalRegulation),
            Enum1153::PlotFile => Ok(crate::zugferd_2_3_2::Enum1153::PlotFile),
            Enum1153::FileConversionJournal => Ok(crate::zugferd_2_3_2::Enum1153::FileConversionJournal),
            Enum1153::AuthorizationNumber => Ok(crate::zugferd_2_3_2::Enum1153::AuthorizationNumber),
            Enum1153::ReferenceNumberAssignedByThirdParty => Ok(crate::zugferd_2_3_2::Enum1153::ReferenceNumberAssignedByThirdParty),
            Enum1153::DepositReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::DepositReferenceNumber),
            Enum1153::NamedBanksReference => Ok(crate::zugferd_2_3_2::Enum1153::NamedBanksReference),
            Enum1153::DraweesReference => Ok(crate::zugferd_2_3_2::Enum1153::DraweesReference),
            Enum1153::CaseNeedPartysReference => Ok(crate::zugferd_2_3_2::Enum1153::CaseNeedPartysReference),
            Enum1153::CollectingBanksReference => Ok(crate::zugferd_2_3_2::Enum1153::CollectingBanksReference),
            Enum1153::RemittingBanksReference => Ok(crate::zugferd_2_3_2::Enum1153::RemittingBanksReference),
            Enum1153::PrincipalsBankReference => Ok(crate::zugferd_2_3_2::Enum1153::PrincipalsBankReference),
            Enum1153::PresentingBanksReference => Ok(crate::zugferd_2_3_2::Enum1153::PresentingBanksReference),
            Enum1153::ConsigneesReference => Ok(crate::zugferd_2_3_2::Enum1153::ConsigneesReference),
            Enum1153::FinancialTransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::FinancialTransactionReferenceNumber),
            Enum1153::CreditReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CreditReferenceNumber),
            Enum1153::ReceivingBanksAuthorizationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReceivingBanksAuthorizationNumber),
            Enum1153::ClearingReference => Ok(crate::zugferd_2_3_2::Enum1153::ClearingReference),
            Enum1153::SendingBanksReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::SendingBanksReferenceNumber),
            Enum1153::DocumentaryPaymentReference => Ok(crate::zugferd_2_3_2::Enum1153::DocumentaryPaymentReference),
            Enum1153::AccountingFileReference => Ok(crate::zugferd_2_3_2::Enum1153::AccountingFileReference),
            Enum1153::SendersFileReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::SendersFileReferenceNumber),
            Enum1153::ReceiversFileReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReceiversFileReferenceNumber),
            Enum1153::SourceDocumentInternalReference => Ok(crate::zugferd_2_3_2::Enum1153::SourceDocumentInternalReference),
            Enum1153::PrincipalsReference => Ok(crate::zugferd_2_3_2::Enum1153::PrincipalsReference),
            Enum1153::DebitReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::DebitReferenceNumber),
            Enum1153::Calendar => Ok(crate::zugferd_2_3_2::Enum1153::Calendar),
            Enum1153::WorkShift => Ok(crate::zugferd_2_3_2::Enum1153::WorkShift),
            Enum1153::WorkBreakdownStructure => Ok(crate::zugferd_2_3_2::Enum1153::WorkBreakdownStructure),
            Enum1153::OrganisationBreakdownStructure => Ok(crate::zugferd_2_3_2::Enum1153::OrganisationBreakdownStructure),
            Enum1153::WorkTaskChargeNumber => Ok(crate::zugferd_2_3_2::Enum1153::WorkTaskChargeNumber),
            Enum1153::FunctionalWorkGroup => Ok(crate::zugferd_2_3_2::Enum1153::FunctionalWorkGroup),
            Enum1153::WorkTeam => Ok(crate::zugferd_2_3_2::Enum1153::WorkTeam),
            Enum1153::Department => Ok(crate::zugferd_2_3_2::Enum1153::Department),
            Enum1153::StatementWork => Ok(crate::zugferd_2_3_2::Enum1153::StatementWork),
            Enum1153::WorkPackage => Ok(crate::zugferd_2_3_2::Enum1153::WorkPackage),
            Enum1153::PlanningPackage => Ok(crate::zugferd_2_3_2::Enum1153::PlanningPackage),
            Enum1153::CostAccount => Ok(crate::zugferd_2_3_2::Enum1153::CostAccount),
            Enum1153::WorkOrder => Ok(crate::zugferd_2_3_2::Enum1153::WorkOrder),
            Enum1153::TransportationControlNumberTcn => Ok(crate::zugferd_2_3_2::Enum1153::TransportationControlNumberTcn),
            Enum1153::ConstraintNotation => Ok(crate::zugferd_2_3_2::Enum1153::ConstraintNotation),
            Enum1153::EtermsReference => Ok(crate::zugferd_2_3_2::Enum1153::EtermsReference),
            Enum1153::ImplementationVersionNumber => Ok(crate::zugferd_2_3_2::Enum1153::ImplementationVersionNumber),
            Enum1153::AccountsReceivableNumber => Ok(crate::zugferd_2_3_2::Enum1153::AccountsReceivableNumber),
            Enum1153::IncorporatedLegalReference => Ok(crate::zugferd_2_3_2::Enum1153::IncorporatedLegalReference),
            Enum1153::PaymentInstalmentReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PaymentInstalmentReferenceNumber),
            Enum1153::EquipmentOwnerReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::EquipmentOwnerReferenceNumber),
            Enum1153::CedentsClaimNumber => Ok(crate::zugferd_2_3_2::Enum1153::CedentsClaimNumber),
            Enum1153::ReinsurersClaimNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReinsurersClaimNumber),
            Enum1153::PriceSalesCatalogueResponseReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriceSalesCatalogueResponseReferenceNumber),
            Enum1153::GeneralPurposeMessageReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::GeneralPurposeMessageReferenceNumber),
            Enum1153::InvoicingDataSheetReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::InvoicingDataSheetReferenceNumber),
            Enum1153::InventoryReportReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::InventoryReportReferenceNumber),
            Enum1153::CeilingFormulaReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CeilingFormulaReferenceNumber),
            Enum1153::PriceVariationFormulaReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriceVariationFormulaReferenceNumber),
            Enum1153::ReferenceToAccountServicingBanksMessage => Ok(crate::zugferd_2_3_2::Enum1153::ReferenceToAccountServicingBanksMessage),
            Enum1153::PartySequenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PartySequenceNumber),
            Enum1153::PurchasersRequestReference => Ok(crate::zugferd_2_3_2::Enum1153::PurchasersRequestReference),
            Enum1153::ContractorRequestReference => Ok(crate::zugferd_2_3_2::Enum1153::ContractorRequestReference),
            Enum1153::AccidentReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::AccidentReferenceNumber),
            Enum1153::CommercialAccountSummaryReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CommercialAccountSummaryReferenceNumber),
            Enum1153::ContractBreakdownReference => Ok(crate::zugferd_2_3_2::Enum1153::ContractBreakdownReference),
            Enum1153::ContractorRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ContractorRegistrationNumber),
            Enum1153::ApplicableCoefficientIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ApplicableCoefficientIdentificationNumber),
            Enum1153::SpecialBudgetAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::SpecialBudgetAccountNumber),
            Enum1153::AuthorisationForRepairReference => Ok(crate::zugferd_2_3_2::Enum1153::AuthorisationForRepairReference),
            Enum1153::ManufacturerDefinedRepairRatesReference => Ok(crate::zugferd_2_3_2::Enum1153::ManufacturerDefinedRepairRatesReference),
            Enum1153::OriginalSubmitterLogNumber => Ok(crate::zugferd_2_3_2::Enum1153::OriginalSubmitterLogNumber),
            Enum1153::OriginalSubmitterParentDataMaintenanceRequestDmr => Ok(crate::zugferd_2_3_2::Enum1153::OriginalSubmitterParentDataMaintenanceRequestDmr),
            Enum1153::OriginalSubmitterChildDataMaintenanceRequestDmr => Ok(crate::zugferd_2_3_2::Enum1153::OriginalSubmitterChildDataMaintenanceRequestDmr),
            Enum1153::EntryPointAssessmentLogNumber => Ok(crate::zugferd_2_3_2::Enum1153::EntryPointAssessmentLogNumber),
            Enum1153::EntryPointAssessmentLogNumberParentDmr => Ok(crate::zugferd_2_3_2::Enum1153::EntryPointAssessmentLogNumberParentDmr),
            Enum1153::EntryPointAssessmentLogNumberChildDmr => Ok(crate::zugferd_2_3_2::Enum1153::EntryPointAssessmentLogNumberChildDmr),
            Enum1153::DataStructureTag => Ok(crate::zugferd_2_3_2::Enum1153::DataStructureTag),
            Enum1153::CentralSecretariatLogNumber => Ok(crate::zugferd_2_3_2::Enum1153::CentralSecretariatLogNumber),
            Enum1153::CentralSecretariatLogNumberParentDataMaintenance => Ok(crate::zugferd_2_3_2::Enum1153::CentralSecretariatLogNumberParentDataMaintenance),
            Enum1153::CentralSecretariatLogNumberChildDataMaintenance => Ok(crate::zugferd_2_3_2::Enum1153::CentralSecretariatLogNumberChildDataMaintenance),
            Enum1153::InternationalAssessmentLogNumber => Ok(crate::zugferd_2_3_2::Enum1153::InternationalAssessmentLogNumber),
            Enum1153::InternationalAssessmentLogNumberParentData => Ok(crate::zugferd_2_3_2::Enum1153::InternationalAssessmentLogNumberParentData),
            Enum1153::InternationalAssessmentLogNumberChildDataMaintenance => Ok(crate::zugferd_2_3_2::Enum1153::InternationalAssessmentLogNumberChildDataMaintenance),
            Enum1153::StatusReportNumber => Ok(crate::zugferd_2_3_2::Enum1153::StatusReportNumber),
            Enum1153::MessageDesignGroupNumber => Ok(crate::zugferd_2_3_2::Enum1153::MessageDesignGroupNumber),
            Enum1153::UsCustomsServiceUscsEntryCode => Ok(crate::zugferd_2_3_2::Enum1153::UsCustomsServiceUscsEntryCode),
            Enum1153::BeginningJobSequenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::BeginningJobSequenceNumber),
            Enum1153::SendersClauseNumber => Ok(crate::zugferd_2_3_2::Enum1153::SendersClauseNumber),
            Enum1153::DunAndBradstreetCanadas8DigitStandardIndustrial => Ok(crate::zugferd_2_3_2::Enum1153::DunAndBradstreetCanadas8DigitStandardIndustrial),
            Enum1153::ActivitePrincipaleExerceeApeIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::ActivitePrincipaleExerceeApeIdentifier),
            Enum1153::DunAndBradstreetUs8DigitStandardIndustrial => Ok(crate::zugferd_2_3_2::Enum1153::DunAndBradstreetUs8DigitStandardIndustrial),
            Enum1153::NomenclatureActivityClassificationEconomyNace => Ok(crate::zugferd_2_3_2::Enum1153::NomenclatureActivityClassificationEconomyNace),
            Enum1153::NormeActiviteFrancaiseNafIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::NormeActiviteFrancaiseNafIdentifier),
            Enum1153::RegisteredContractorActivityType => Ok(crate::zugferd_2_3_2::Enum1153::RegisteredContractorActivityType),
            Enum1153::StatisticBundesAmtSbaIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::StatisticBundesAmtSbaIdentifier),
            Enum1153::StateOrProvinceAssignedEntityIdentification => Ok(crate::zugferd_2_3_2::Enum1153::StateOrProvinceAssignedEntityIdentification),
            Enum1153::InstituteSecurityAndFutureMarketDevelopmentIsfmd => Ok(crate::zugferd_2_3_2::Enum1153::InstituteSecurityAndFutureMarketDevelopmentIsfmd),
            Enum1153::FileIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::FileIdentificationNumber),
            Enum1153::BankruptcyProcedureNumber => Ok(crate::zugferd_2_3_2::Enum1153::BankruptcyProcedureNumber),
            Enum1153::NationalGovernmentBusinessIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::NationalGovernmentBusinessIdentificationNumber),
            Enum1153::PriorDataUniversalNumberSystemDunsNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriorDataUniversalNumberSystemDunsNumber),
            Enum1153::CompaniesRegistryOfficeCroNumber => Ok(crate::zugferd_2_3_2::Enum1153::CompaniesRegistryOfficeCroNumber),
            Enum1153::CostaRicanJudicialNumber => Ok(crate::zugferd_2_3_2::Enum1153::CostaRicanJudicialNumber),
            Enum1153::NumeroDeIdentificacionTributariaNit => Ok(crate::zugferd_2_3_2::Enum1153::NumeroDeIdentificacionTributariaNit),
            Enum1153::PatronNumber => Ok(crate::zugferd_2_3_2::Enum1153::PatronNumber),
            Enum1153::RegistroInformacionFiscalRifNumber => Ok(crate::zugferd_2_3_2::Enum1153::RegistroInformacionFiscalRifNumber),
            Enum1153::RegistroUnicoDeContribuyenteRucNumber => Ok(crate::zugferd_2_3_2::Enum1153::RegistroUnicoDeContribuyenteRucNumber),
            Enum1153::TokyoShokoResearchTsrBusinessIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::TokyoShokoResearchTsrBusinessIdentifier),
            Enum1153::PersonalIdentityCardNumber => Ok(crate::zugferd_2_3_2::Enum1153::PersonalIdentityCardNumber),
            Enum1153::SystemeInformatiquePourLeRepertoireDesEntreprises => Ok(crate::zugferd_2_3_2::Enum1153::SystemeInformatiquePourLeRepertoireDesEntreprises),
            Enum1153::SystemeInformatiquePourLeRepertoireDesEtablissements => Ok(crate::zugferd_2_3_2::Enum1153::SystemeInformatiquePourLeRepertoireDesEtablissements),
            Enum1153::PublicationIssueNumber => Ok(crate::zugferd_2_3_2::Enum1153::PublicationIssueNumber),
            Enum1153::OriginalFilingNumber => Ok(crate::zugferd_2_3_2::Enum1153::OriginalFilingNumber),
            Enum1153::DocumentPageIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::DocumentPageIdentifier),
            Enum1153::PublicFilingRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::PublicFilingRegistrationNumber),
            Enum1153::RegiristoFederalDeContribuyentes => Ok(crate::zugferd_2_3_2::Enum1153::RegiristoFederalDeContribuyentes),
            Enum1153::SocialSecurityNumber => Ok(crate::zugferd_2_3_2::Enum1153::SocialSecurityNumber),
            Enum1153::DocumentVolumeNumber => Ok(crate::zugferd_2_3_2::Enum1153::DocumentVolumeNumber),
            Enum1153::BookNumber => Ok(crate::zugferd_2_3_2::Enum1153::BookNumber),
            Enum1153::StockExchangeCompanyIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::StockExchangeCompanyIdentifier),
            Enum1153::ImputationAccount => Ok(crate::zugferd_2_3_2::Enum1153::ImputationAccount),
            Enum1153::FinancialPhaseReference => Ok(crate::zugferd_2_3_2::Enum1153::FinancialPhaseReference),
            Enum1153::TechnicalPhaseReference => Ok(crate::zugferd_2_3_2::Enum1153::TechnicalPhaseReference),
            Enum1153::PriorContractorRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriorContractorRegistrationNumber),
            Enum1153::StockAdjustmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::StockAdjustmentNumber),
            Enum1153::DispensationReference => Ok(crate::zugferd_2_3_2::Enum1153::DispensationReference),
            Enum1153::InvestmentReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::InvestmentReferenceNumber),
            Enum1153::AssumingCompany => Ok(crate::zugferd_2_3_2::Enum1153::AssumingCompany),
            Enum1153::BudgetChapter => Ok(crate::zugferd_2_3_2::Enum1153::BudgetChapter),
            Enum1153::DutyFreeProductsSecurityNumber => Ok(crate::zugferd_2_3_2::Enum1153::DutyFreeProductsSecurityNumber),
            Enum1153::DutyFreeProductsReceiptAuthorisationNumber => Ok(crate::zugferd_2_3_2::Enum1153::DutyFreeProductsReceiptAuthorisationNumber),
            Enum1153::PartyInformationMessageReference => Ok(crate::zugferd_2_3_2::Enum1153::PartyInformationMessageReference),
            Enum1153::FormalStatementReference => Ok(crate::zugferd_2_3_2::Enum1153::FormalStatementReference),
            Enum1153::ProofDeliveryReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProofDeliveryReferenceNumber),
            Enum1153::SuppliersCreditClaimReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::SuppliersCreditClaimReferenceNumber),
            Enum1153::PictureActualProduct => Ok(crate::zugferd_2_3_2::Enum1153::PictureActualProduct),
            Enum1153::PictureAGenericProduct => Ok(crate::zugferd_2_3_2::Enum1153::PictureAGenericProduct),
            Enum1153::TradingPartnerIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::TradingPartnerIdentificationNumber),
            Enum1153::PriorTradingPartnerIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriorTradingPartnerIdentificationNumber),
            Enum1153::Password => Ok(crate::zugferd_2_3_2::Enum1153::Password),
            Enum1153::FormalReportNumber => Ok(crate::zugferd_2_3_2::Enum1153::FormalReportNumber),
            Enum1153::FundAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::FundAccountNumber),
            Enum1153::SafeCustodyNumber => Ok(crate::zugferd_2_3_2::Enum1153::SafeCustodyNumber),
            Enum1153::MasterAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::MasterAccountNumber),
            Enum1153::GroupReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::GroupReferenceNumber),
            Enum1153::AccountingTransmissionNumber => Ok(crate::zugferd_2_3_2::Enum1153::AccountingTransmissionNumber),
            Enum1153::ProductDataFileNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProductDataFileNumber),
            Enum1153::CadastroGeralDoContribuinteCgc => Ok(crate::zugferd_2_3_2::Enum1153::CadastroGeralDoContribuinteCgc),
            Enum1153::ForeignResidentIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ForeignResidentIdentificationNumber),
            Enum1153::CdRom => Ok(crate::zugferd_2_3_2::Enum1153::CdRom),
            Enum1153::PhysicalMedium => Ok(crate::zugferd_2_3_2::Enum1153::PhysicalMedium),
            Enum1153::FinancialCancellationReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::FinancialCancellationReferenceNumber),
            Enum1153::PurchaseForExportCustomsAgreementNumber => Ok(crate::zugferd_2_3_2::Enum1153::PurchaseForExportCustomsAgreementNumber),
            Enum1153::JudgmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::JudgmentNumber),
            Enum1153::SecretariatNumber => Ok(crate::zugferd_2_3_2::Enum1153::SecretariatNumber),
            Enum1153::PreviousBankingStatusMessageReference => Ok(crate::zugferd_2_3_2::Enum1153::PreviousBankingStatusMessageReference),
            Enum1153::LastReceivedBankingStatusMessageReference => Ok(crate::zugferd_2_3_2::Enum1153::LastReceivedBankingStatusMessageReference),
            Enum1153::BanksDocumentaryProcedureReference => Ok(crate::zugferd_2_3_2::Enum1153::BanksDocumentaryProcedureReference),
            Enum1153::CustomersDocumentaryProcedureReference => Ok(crate::zugferd_2_3_2::Enum1153::CustomersDocumentaryProcedureReference),
            Enum1153::SafeDepositBoxNumber => Ok(crate::zugferd_2_3_2::Enum1153::SafeDepositBoxNumber),
            Enum1153::ReceivingBankgiroNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReceivingBankgiroNumber),
            Enum1153::SendingBankgiroNumber => Ok(crate::zugferd_2_3_2::Enum1153::SendingBankgiroNumber),
            Enum1153::BankgiroReference => Ok(crate::zugferd_2_3_2::Enum1153::BankgiroReference),
            Enum1153::GuaranteeNumber => Ok(crate::zugferd_2_3_2::Enum1153::GuaranteeNumber),
            Enum1153::CollectionInstrumentNumber => Ok(crate::zugferd_2_3_2::Enum1153::CollectionInstrumentNumber),
            Enum1153::ConvertedPostgiroNumber => Ok(crate::zugferd_2_3_2::Enum1153::ConvertedPostgiroNumber),
            Enum1153::CostCentreAlignmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::CostCentreAlignmentNumber),
            Enum1153::KamerVanKoophandelKvkNumber => Ok(crate::zugferd_2_3_2::Enum1153::KamerVanKoophandelKvkNumber),
            Enum1153::InstitutBelgoLuxembourgeoisDeCodificationIblcNumber => Ok(crate::zugferd_2_3_2::Enum1153::InstitutBelgoLuxembourgeoisDeCodificationIblcNumber),
            Enum1153::ExternalObjectReference => Ok(crate::zugferd_2_3_2::Enum1153::ExternalObjectReference),
            Enum1153::ExceptionalTransportAuthorisationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ExceptionalTransportAuthorisationNumber),
            Enum1153::ClaveUnicaDeIdentificacionTributariaCuit => Ok(crate::zugferd_2_3_2::Enum1153::ClaveUnicaDeIdentificacionTributariaCuit),
            Enum1153::RegistroUnicoTributarioRut => Ok(crate::zugferd_2_3_2::Enum1153::RegistroUnicoTributarioRut),
            Enum1153::FlatRackContainerBundleIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::FlatRackContainerBundleIdentificationNumber),
            Enum1153::TransportEquipmentAcceptanceOrderReference => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentAcceptanceOrderReference),
            Enum1153::TransportEquipmentReleaseOrderReference => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentReleaseOrderReference),
            Enum1153::ShipsStayReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ShipsStayReferenceNumber),
            Enum1153::AuthorizationToMeetCompetitionNumber => Ok(crate::zugferd_2_3_2::Enum1153::AuthorizationToMeetCompetitionNumber),
            Enum1153::PlacePositioningReference => Ok(crate::zugferd_2_3_2::Enum1153::PlacePositioningReference),
            Enum1153::PartyReference => Ok(crate::zugferd_2_3_2::Enum1153::PartyReference),
            Enum1153::IssuedPrescriptionIdentification => Ok(crate::zugferd_2_3_2::Enum1153::IssuedPrescriptionIdentification),
            Enum1153::CollectionReference => Ok(crate::zugferd_2_3_2::Enum1153::CollectionReference),
            Enum1153::TravelService => Ok(crate::zugferd_2_3_2::Enum1153::TravelService),
            Enum1153::ConsignmentStockContract => Ok(crate::zugferd_2_3_2::Enum1153::ConsignmentStockContract),
            Enum1153::ImportersLetterCreditReference => Ok(crate::zugferd_2_3_2::Enum1153::ImportersLetterCreditReference),
            Enum1153::PerformedPrescriptionIdentification => Ok(crate::zugferd_2_3_2::Enum1153::PerformedPrescriptionIdentification),
            Enum1153::ImageReference => Ok(crate::zugferd_2_3_2::Enum1153::ImageReference),
            Enum1153::ProposedPurchaseOrderReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProposedPurchaseOrderReferenceNumber),
            Enum1153::ApplicationForFinancialSupportReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ApplicationForFinancialSupportReferenceNumber),
            Enum1153::ManufacturingQualityAgreementNumber => Ok(crate::zugferd_2_3_2::Enum1153::ManufacturingQualityAgreementNumber),
            Enum1153::SoftwareEditorReference => Ok(crate::zugferd_2_3_2::Enum1153::SoftwareEditorReference),
            Enum1153::SoftwareReference => Ok(crate::zugferd_2_3_2::Enum1153::SoftwareReference),
            Enum1153::SoftwareQualityReference => Ok(crate::zugferd_2_3_2::Enum1153::SoftwareQualityReference),
            Enum1153::ConsolidatedOrdersReference => Ok(crate::zugferd_2_3_2::Enum1153::ConsolidatedOrdersReference),
            Enum1153::CustomsBindingRulingNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsBindingRulingNumber),
            Enum1153::CustomsNonBindingRulingNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsNonBindingRulingNumber),
            Enum1153::DeliveryRouteReference => Ok(crate::zugferd_2_3_2::Enum1153::DeliveryRouteReference),
            Enum1153::NetAreaSupplierReference => Ok(crate::zugferd_2_3_2::Enum1153::NetAreaSupplierReference),
            Enum1153::TimeSeriesReference => Ok(crate::zugferd_2_3_2::Enum1153::TimeSeriesReference),
            Enum1153::ConnectingPointToCentralGrid => Ok(crate::zugferd_2_3_2::Enum1153::ConnectingPointToCentralGrid),
            Enum1153::MarketingPlanIdentificationNumberMpin => Ok(crate::zugferd_2_3_2::Enum1153::MarketingPlanIdentificationNumberMpin),
            Enum1153::EntityReferenceNumberPrevious => Ok(crate::zugferd_2_3_2::Enum1153::EntityReferenceNumberPrevious),
            Enum1153::InternationalStandardIndustrialClassificationIsic => Ok(crate::zugferd_2_3_2::Enum1153::InternationalStandardIndustrialClassificationIsic),
            Enum1153::CustomsPreApprovalRulingNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomsPreApprovalRulingNumber),
            Enum1153::AccountPayableNumber => Ok(crate::zugferd_2_3_2::Enum1153::AccountPayableNumber),
            Enum1153::FirstFinancialInstitutionsTransactionReference => Ok(crate::zugferd_2_3_2::Enum1153::FirstFinancialInstitutionsTransactionReference),
            Enum1153::ProductCharacteristicsDirectory => Ok(crate::zugferd_2_3_2::Enum1153::ProductCharacteristicsDirectory),
            Enum1153::SuppliersCustomerReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::SuppliersCustomerReferenceNumber),
            Enum1153::InventoryReportRequestNumber => Ok(crate::zugferd_2_3_2::Enum1153::InventoryReportRequestNumber),
            Enum1153::MeteringPoint => Ok(crate::zugferd_2_3_2::Enum1153::MeteringPoint),
            Enum1153::PassengerReservationNumber => Ok(crate::zugferd_2_3_2::Enum1153::PassengerReservationNumber),
            Enum1153::SlaughterhouseApprovalNumber => Ok(crate::zugferd_2_3_2::Enum1153::SlaughterhouseApprovalNumber),
            Enum1153::MeatCuttingPlantApprovalNumber => Ok(crate::zugferd_2_3_2::Enum1153::MeatCuttingPlantApprovalNumber),
            Enum1153::CustomerTravelServiceIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::CustomerTravelServiceIdentifier),
            Enum1153::ExportControlClassificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ExportControlClassificationNumber),
            Enum1153::BrokerReference3 => Ok(crate::zugferd_2_3_2::Enum1153::BrokerReference3),
            Enum1153::ConsignmentInformation => Ok(crate::zugferd_2_3_2::Enum1153::ConsignmentInformation),
            Enum1153::GoodsItemInformation => Ok(crate::zugferd_2_3_2::Enum1153::GoodsItemInformation),
            Enum1153::DangerousGoodsInformation => Ok(crate::zugferd_2_3_2::Enum1153::DangerousGoodsInformation),
            Enum1153::PilotageServicesExemptionNumber => Ok(crate::zugferd_2_3_2::Enum1153::PilotageServicesExemptionNumber),
            Enum1153::PersonRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::PersonRegistrationNumber),
            Enum1153::PlacePackingApprovalNumber => Ok(crate::zugferd_2_3_2::Enum1153::PlacePackingApprovalNumber),
            Enum1153::OriginalMandateReference => Ok(crate::zugferd_2_3_2::Enum1153::OriginalMandateReference),
            Enum1153::MandateReference => Ok(crate::zugferd_2_3_2::Enum1153::MandateReference),
            Enum1153::ReservationStationIndentifier => Ok(crate::zugferd_2_3_2::Enum1153::ReservationStationIndentifier),
            Enum1153::UniqueGoodsShipmentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::UniqueGoodsShipmentIdentifier),
            Enum1153::FrameworkAgreementNumber => Ok(crate::zugferd_2_3_2::Enum1153::FrameworkAgreementNumber),
            Enum1153::HashValue => Ok(crate::zugferd_2_3_2::Enum1153::HashValue),
            Enum1153::MovementReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::MovementReferenceNumber),
            Enum1153::EconomicOperatorsRegistrationAndIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::EconomicOperatorsRegistrationAndIdentificationNumber),
            Enum1153::LocalReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::LocalReferenceNumber),
            Enum1153::RateCodeNumber => Ok(crate::zugferd_2_3_2::Enum1153::RateCodeNumber),
            Enum1153::AirWaybillNumber => Ok(crate::zugferd_2_3_2::Enum1153::AirWaybillNumber),
            Enum1153::DocumentaryCreditAmendmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::DocumentaryCreditAmendmentNumber),
            Enum1153::AdvisingBanksReference => Ok(crate::zugferd_2_3_2::Enum1153::AdvisingBanksReference),
            Enum1153::CostCentre => Ok(crate::zugferd_2_3_2::Enum1153::CostCentre),
            Enum1153::WorkItemQuantityDetermination => Ok(crate::zugferd_2_3_2::Enum1153::WorkItemQuantityDetermination),
            Enum1153::InternalDataProcessNumber => Ok(crate::zugferd_2_3_2::Enum1153::InternalDataProcessNumber),
            Enum1153::CategoryWorkReference => Ok(crate::zugferd_2_3_2::Enum1153::CategoryWorkReference),
            Enum1153::PolicyFormNumber => Ok(crate::zugferd_2_3_2::Enum1153::PolicyFormNumber),
            Enum1153::NetArea => Ok(crate::zugferd_2_3_2::Enum1153::NetArea),
            Enum1153::ServiceProvider => Ok(crate::zugferd_2_3_2::Enum1153::ServiceProvider),
            Enum1153::ErrorPosition => Ok(crate::zugferd_2_3_2::Enum1153::ErrorPosition),
            Enum1153::ServiceCategoryReference => Ok(crate::zugferd_2_3_2::Enum1153::ServiceCategoryReference),
            Enum1153::ConnectedLocation => Ok(crate::zugferd_2_3_2::Enum1153::ConnectedLocation),
            Enum1153::RelatedParty => Ok(crate::zugferd_2_3_2::Enum1153::RelatedParty),
            Enum1153::LatestAccountingEntryRecordReference => Ok(crate::zugferd_2_3_2::Enum1153::LatestAccountingEntryRecordReference),
            Enum1153::AccountingEntry => Ok(crate::zugferd_2_3_2::Enum1153::AccountingEntry),
            Enum1153::DocumentReferenceOriginal => Ok(crate::zugferd_2_3_2::Enum1153::DocumentReferenceOriginal),
            Enum1153::HygienicCertificateNumberNational => Ok(crate::zugferd_2_3_2::Enum1153::HygienicCertificateNumberNational),
            Enum1153::AdministrativeReferenceCode => Ok(crate::zugferd_2_3_2::Enum1153::AdministrativeReferenceCode),
            Enum1153::PickUpSheetNumber => Ok(crate::zugferd_2_3_2::Enum1153::PickUpSheetNumber),
            Enum1153::PhoneNumber => Ok(crate::zugferd_2_3_2::Enum1153::PhoneNumber),
            Enum1153::BuyersFundNumber => Ok(crate::zugferd_2_3_2::Enum1153::BuyersFundNumber),
            Enum1153::CompanyTradingAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::CompanyTradingAccountNumber),
            Enum1153::ReservedGoodsIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::ReservedGoodsIdentifier),
            Enum1153::HandlingAndMovementReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::HandlingAndMovementReferenceNumber),
            Enum1153::InstructionToDespatchReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::InstructionToDespatchReferenceNumber),
            Enum1153::InstructionForReturnsNumber => Ok(crate::zugferd_2_3_2::Enum1153::InstructionForReturnsNumber),
            Enum1153::MeteredServicesConsumptionReportNumber => Ok(crate::zugferd_2_3_2::Enum1153::MeteredServicesConsumptionReportNumber),
            Enum1153::OrderStatusEnquiryNumber => Ok(crate::zugferd_2_3_2::Enum1153::OrderStatusEnquiryNumber),
            Enum1153::FirmBookingReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::FirmBookingReferenceNumber),
            Enum1153::ProductInquiryNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProductInquiryNumber),
            Enum1153::SplitDeliveryNumber => Ok(crate::zugferd_2_3_2::Enum1153::SplitDeliveryNumber),
            Enum1153::ServiceRelationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ServiceRelationNumber),
            Enum1153::SerialShippingContainerCode => Ok(crate::zugferd_2_3_2::Enum1153::SerialShippingContainerCode),
            Enum1153::TestSpecificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::TestSpecificationNumber),
            Enum1153::TransportStatusReportNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransportStatusReportNumber),
            Enum1153::ToolingContractNumber => Ok(crate::zugferd_2_3_2::Enum1153::ToolingContractNumber),
            Enum1153::FormulaReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::FormulaReferenceNumber),
            Enum1153::PreAgreementNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreAgreementNumber),
            Enum1153::ProductCertificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProductCertificationNumber),
            Enum1153::ConsignmentContractNumber => Ok(crate::zugferd_2_3_2::Enum1153::ConsignmentContractNumber),
            Enum1153::ProductSpecificationReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ProductSpecificationReferenceNumber),
            Enum1153::PayrollDeductionAdviceReference => Ok(crate::zugferd_2_3_2::Enum1153::PayrollDeductionAdviceReference),
            Enum1153::TracesPartyIdentification => Ok(crate::zugferd_2_3_2::Enum1153::TracesPartyIdentification),
            Enum1153::BeginningMeterReadingActual => Ok(crate::zugferd_2_3_2::Enum1153::BeginningMeterReadingActual),
            Enum1153::BuyersContractNumber => Ok(crate::zugferd_2_3_2::Enum1153::BuyersContractNumber),
            Enum1153::BidNumber => Ok(crate::zugferd_2_3_2::Enum1153::BidNumber),
            Enum1153::BeginningMeterReadingEstimated => Ok(crate::zugferd_2_3_2::Enum1153::BeginningMeterReadingEstimated),
            Enum1153::HouseBillLadingNumber => Ok(crate::zugferd_2_3_2::Enum1153::HouseBillLadingNumber),
            Enum1153::BillLadingNumber => Ok(crate::zugferd_2_3_2::Enum1153::BillLadingNumber),
            Enum1153::ConsignmentIdentifierCarrierAssigned => Ok(crate::zugferd_2_3_2::Enum1153::ConsignmentIdentifierCarrierAssigned),
            Enum1153::BlanketOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::BlanketOrderNumber),
            Enum1153::BrokerOrSalesOfficeNumber => Ok(crate::zugferd_2_3_2::Enum1153::BrokerOrSalesOfficeNumber),
            Enum1153::BatchNumberLotNumber => Ok(crate::zugferd_2_3_2::Enum1153::BatchNumberLotNumber),
            Enum1153::BatteryAndAccumulatorProducerRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::BatteryAndAccumulatorProducerRegistrationNumber),
            Enum1153::BlendedWithNumber => Ok(crate::zugferd_2_3_2::Enum1153::BlendedWithNumber),
            Enum1153::IataCargoAgentCassAddressNumber => Ok(crate::zugferd_2_3_2::Enum1153::IataCargoAgentCassAddressNumber),
            Enum1153::MatchingEntriesBalanced => Ok(crate::zugferd_2_3_2::Enum1153::MatchingEntriesBalanced),
            Enum1153::EntryFlagging => Ok(crate::zugferd_2_3_2::Enum1153::EntryFlagging),
            Enum1153::MatchingEntriesUnbalanced => Ok(crate::zugferd_2_3_2::Enum1153::MatchingEntriesUnbalanced),
            Enum1153::DocumentReferenceInternal => Ok(crate::zugferd_2_3_2::Enum1153::DocumentReferenceInternal),
            Enum1153::EuropeanValueAddedTaxIdentification => Ok(crate::zugferd_2_3_2::Enum1153::EuropeanValueAddedTaxIdentification),
            Enum1153::CostAccountingDocument => Ok(crate::zugferd_2_3_2::Enum1153::CostAccountingDocument),
            Enum1153::GridOperatorsCustomerReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::GridOperatorsCustomerReferenceNumber),
            Enum1153::TicketControlNumber => Ok(crate::zugferd_2_3_2::Enum1153::TicketControlNumber),
            Enum1153::OrderShipmentGroupingReference => Ok(crate::zugferd_2_3_2::Enum1153::OrderShipmentGroupingReference),
            Enum1153::CreditNoteNumber => Ok(crate::zugferd_2_3_2::Enum1153::CreditNoteNumber),
            Enum1153::CedingCompany => Ok(crate::zugferd_2_3_2::Enum1153::CedingCompany),
            Enum1153::DebitLetterNumber => Ok(crate::zugferd_2_3_2::Enum1153::DebitLetterNumber),
            Enum1153::ConsigneesFurtherOrder => Ok(crate::zugferd_2_3_2::Enum1153::ConsigneesFurtherOrder),
            Enum1153::AnimalFarmLicenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::AnimalFarmLicenceNumber),
            Enum1153::ConsignorsFurtherOrder => Ok(crate::zugferd_2_3_2::Enum1153::ConsignorsFurtherOrder),
            Enum1153::ConsigneesOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::ConsigneesOrderNumber),
            Enum1153::CustomerCatalogueNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomerCatalogueNumber),
            Enum1153::ChequeNumber => Ok(crate::zugferd_2_3_2::Enum1153::ChequeNumber),
            Enum1153::CheckingNumber => Ok(crate::zugferd_2_3_2::Enum1153::CheckingNumber),
            Enum1153::CreditMemoNumber => Ok(crate::zugferd_2_3_2::Enum1153::CreditMemoNumber),
            Enum1153::RoadConsignmentNoteNumber => Ok(crate::zugferd_2_3_2::Enum1153::RoadConsignmentNoteNumber),
            Enum1153::CarriersReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CarriersReferenceNumber),
            Enum1153::ChargesNoteDocumentAttachmentIndicator => Ok(crate::zugferd_2_3_2::Enum1153::ChargesNoteDocumentAttachmentIndicator),
            Enum1153::CallOffOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::CallOffOrderNumber),
            Enum1153::ConditionPurchaseDocumentNumber => Ok(crate::zugferd_2_3_2::Enum1153::ConditionPurchaseDocumentNumber),
            Enum1153::CustomerReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CustomerReferenceNumber),
            Enum1153::TransportMeansJourneyIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::TransportMeansJourneyIdentifier),
            Enum1153::ConditionSaleDocumentNumber => Ok(crate::zugferd_2_3_2::Enum1153::ConditionSaleDocumentNumber),
            Enum1153::TeamAssignmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::TeamAssignmentNumber),
            Enum1153::ContractNumber => Ok(crate::zugferd_2_3_2::Enum1153::ContractNumber),
            Enum1153::ConsignmentIdentifierConsignorAssigned => Ok(crate::zugferd_2_3_2::Enum1153::ConsignmentIdentifierConsignorAssigned),
            Enum1153::ContainerOperatorsReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ContainerOperatorsReferenceNumber),
            Enum1153::PackageNumber => Ok(crate::zugferd_2_3_2::Enum1153::PackageNumber),
            Enum1153::CooperationContractNumber => Ok(crate::zugferd_2_3_2::Enum1153::CooperationContractNumber),
            Enum1153::DefermentApprovalNumber => Ok(crate::zugferd_2_3_2::Enum1153::DefermentApprovalNumber),
            Enum1153::DebitAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::DebitAccountNumber),
            Enum1153::BuyersDebtorNumber => Ok(crate::zugferd_2_3_2::Enum1153::BuyersDebtorNumber),
            Enum1153::DistributorInvoiceNumber => Ok(crate::zugferd_2_3_2::Enum1153::DistributorInvoiceNumber),
            Enum1153::DebitNoteNumber => Ok(crate::zugferd_2_3_2::Enum1153::DebitNoteNumber),
            Enum1153::DocumentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::DocumentIdentifier),
            Enum1153::DeliveryNoteNumber => Ok(crate::zugferd_2_3_2::Enum1153::DeliveryNoteNumber),
            Enum1153::DockReceiptNumber => Ok(crate::zugferd_2_3_2::Enum1153::DockReceiptNumber),
            Enum1153::EndingMeterReadingActual => Ok(crate::zugferd_2_3_2::Enum1153::EndingMeterReadingActual),
            Enum1153::EmbargoPermitNumber => Ok(crate::zugferd_2_3_2::Enum1153::EmbargoPermitNumber),
            Enum1153::ExportDeclaration => Ok(crate::zugferd_2_3_2::Enum1153::ExportDeclaration),
            Enum1153::EndingMeterReadingEstimated => Ok(crate::zugferd_2_3_2::Enum1153::EndingMeterReadingEstimated),
            Enum1153::ElectricalAndElectronicEquipmentProducerRegistration => Ok(crate::zugferd_2_3_2::Enum1153::ElectricalAndElectronicEquipmentProducerRegistration),
            Enum1153::EmployersIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::EmployersIdentificationNumber),
            Enum1153::EmbargoNumber => Ok(crate::zugferd_2_3_2::Enum1153::EmbargoNumber),
            Enum1153::EquipmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::EquipmentNumber),
            Enum1153::ContainerEquipmentReceiptNumber => Ok(crate::zugferd_2_3_2::Enum1153::ContainerEquipmentReceiptNumber),
            Enum1153::ExportersReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ExportersReferenceNumber),
            Enum1153::ExcessTransportationNumber => Ok(crate::zugferd_2_3_2::Enum1153::ExcessTransportationNumber),
            Enum1153::ExportPermitIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::ExportPermitIdentifier),
            Enum1153::FiscalNumber => Ok(crate::zugferd_2_3_2::Enum1153::FiscalNumber),
            Enum1153::ConsignmentIdentifierFreightForwarderAssigned => Ok(crate::zugferd_2_3_2::Enum1153::ConsignmentIdentifierFreightForwarderAssigned),
            Enum1153::FileLineIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::FileLineIdentifier),
            Enum1153::FlowReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::FlowReferenceNumber),
            Enum1153::FreightBillNumber => Ok(crate::zugferd_2_3_2::Enum1153::FreightBillNumber),
            Enum1153::ForeignExchange => Ok(crate::zugferd_2_3_2::Enum1153::ForeignExchange),
            Enum1153::FinalSequenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::FinalSequenceNumber),
            Enum1153::FreeZoneIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::FreeZoneIdentifier),
            Enum1153::FileVersionNumber => Ok(crate::zugferd_2_3_2::Enum1153::FileVersionNumber),
            Enum1153::ForeignExchangeContractNumber => Ok(crate::zugferd_2_3_2::Enum1153::ForeignExchangeContractNumber),
            Enum1153::StandardsNumber => Ok(crate::zugferd_2_3_2::Enum1153::StandardsNumber),
            Enum1153::GovernmentContractNumber => Ok(crate::zugferd_2_3_2::Enum1153::GovernmentContractNumber),
            Enum1153::StandardsCodeNumber => Ok(crate::zugferd_2_3_2::Enum1153::StandardsCodeNumber),
            Enum1153::GeneralDeclarationNumber => Ok(crate::zugferd_2_3_2::Enum1153::GeneralDeclarationNumber),
            Enum1153::GovernmentReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::GovernmentReferenceNumber),
            Enum1153::HarmonisedSystemNumber => Ok(crate::zugferd_2_3_2::Enum1153::HarmonisedSystemNumber),
            Enum1153::HouseWaybillNumber => Ok(crate::zugferd_2_3_2::Enum1153::HouseWaybillNumber),
            Enum1153::InternalVendorNumber => Ok(crate::zugferd_2_3_2::Enum1153::InternalVendorNumber),
            Enum1153::InBondNumber => Ok(crate::zugferd_2_3_2::Enum1153::InBondNumber),
            Enum1153::IataCargoAgentCodeNumber => Ok(crate::zugferd_2_3_2::Enum1153::IataCargoAgentCodeNumber),
            Enum1153::InsuranceCertificateReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::InsuranceCertificateReferenceNumber),
            Enum1153::InsuranceContractReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::InsuranceContractReferenceNumber),
            Enum1153::InitialSampleInspectionReportNumber => Ok(crate::zugferd_2_3_2::Enum1153::InitialSampleInspectionReportNumber),
            Enum1153::InternalOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::InternalOrderNumber),
            Enum1153::IntermediaryBroker => Ok(crate::zugferd_2_3_2::Enum1153::IntermediaryBroker),
            Enum1153::InterchangeNumberNew => Ok(crate::zugferd_2_3_2::Enum1153::InterchangeNumberNew),
            Enum1153::InterchangeNumberOld => Ok(crate::zugferd_2_3_2::Enum1153::InterchangeNumberOld),
            Enum1153::ImportPermitIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::ImportPermitIdentifier),
            Enum1153::InvoiceNumberSuffix => Ok(crate::zugferd_2_3_2::Enum1153::InvoiceNumberSuffix),
            Enum1153::InternalCustomerNumber => Ok(crate::zugferd_2_3_2::Enum1153::InternalCustomerNumber),
            Enum1153::InvoiceDocumentIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::InvoiceDocumentIdentifier),
            Enum1153::JobNumber => Ok(crate::zugferd_2_3_2::Enum1153::JobNumber),
            Enum1153::EndingJobSequenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::EndingJobSequenceNumber),
            Enum1153::ShippingLabelSerialNumber => Ok(crate::zugferd_2_3_2::Enum1153::ShippingLabelSerialNumber),
            Enum1153::LoadingAuthorisationIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::LoadingAuthorisationIdentifier),
            Enum1153::LowerNumberInRange => Ok(crate::zugferd_2_3_2::Enum1153::LowerNumberInRange),
            Enum1153::Lockbox => Ok(crate::zugferd_2_3_2::Enum1153::Lockbox),
            Enum1153::LetterCreditNumber => Ok(crate::zugferd_2_3_2::Enum1153::LetterCreditNumber),
            Enum1153::DocumentLineIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::DocumentLineIdentifier),
            Enum1153::LoadPlanningNumber => Ok(crate::zugferd_2_3_2::Enum1153::LoadPlanningNumber),
            Enum1153::ReservationOfficeIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::ReservationOfficeIdentifier),
            Enum1153::BarCodedLabelSerialNumber => Ok(crate::zugferd_2_3_2::Enum1153::BarCodedLabelSerialNumber),
            Enum1153::ShipNoticeManifestNumber => Ok(crate::zugferd_2_3_2::Enum1153::ShipNoticeManifestNumber),
            Enum1153::MasterBillLadingNumber => Ok(crate::zugferd_2_3_2::Enum1153::MasterBillLadingNumber),
            Enum1153::ManufacturersPartNumber => Ok(crate::zugferd_2_3_2::Enum1153::ManufacturersPartNumber),
            Enum1153::MeterUnitNumber => Ok(crate::zugferd_2_3_2::Enum1153::MeterUnitNumber),
            Enum1153::ManufacturingOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::ManufacturingOrderNumber),
            Enum1153::MessageRecipient => Ok(crate::zugferd_2_3_2::Enum1153::MessageRecipient),
            Enum1153::MailingReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::MailingReferenceNumber),
            Enum1153::MessageSender => Ok(crate::zugferd_2_3_2::Enum1153::MessageSender),
            Enum1153::ManufacturersMaterialSafetyDataSheetNumber => Ok(crate::zugferd_2_3_2::Enum1153::ManufacturersMaterialSafetyDataSheetNumber),
            Enum1153::MasterAirWaybillNumber => Ok(crate::zugferd_2_3_2::Enum1153::MasterAirWaybillNumber),
            Enum1153::NorthAmericanHazardousGoodsClassificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::NorthAmericanHazardousGoodsClassificationNumber),
            Enum1153::NotaFiscal => Ok(crate::zugferd_2_3_2::Enum1153::NotaFiscal),
            Enum1153::CurrentInvoiceNumber => Ok(crate::zugferd_2_3_2::Enum1153::CurrentInvoiceNumber),
            Enum1153::PreviousInvoiceNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousInvoiceNumber),
            Enum1153::OrderDocumentIdentifierBuyerAssigned => Ok(crate::zugferd_2_3_2::Enum1153::OrderDocumentIdentifierBuyerAssigned),
            Enum1153::OriginalPurchaseOrder => Ok(crate::zugferd_2_3_2::Enum1153::OriginalPurchaseOrder),
            Enum1153::GeneralOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::GeneralOrderNumber),
            Enum1153::PayersFinancialInstitutionAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::PayersFinancialInstitutionAccountNumber),
            Enum1153::ProductionCode => Ok(crate::zugferd_2_3_2::Enum1153::ProductionCode),
            Enum1153::PromotionDealNumber => Ok(crate::zugferd_2_3_2::Enum1153::PromotionDealNumber),
            Enum1153::PlantNumber => Ok(crate::zugferd_2_3_2::Enum1153::PlantNumber),
            Enum1153::PrimeContractorContractNumber => Ok(crate::zugferd_2_3_2::Enum1153::PrimeContractorContractNumber),
            Enum1153::PriceListVersionNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriceListVersionNumber),
            Enum1153::PackingListNumber => Ok(crate::zugferd_2_3_2::Enum1153::PackingListNumber),
            Enum1153::PriceListNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriceListNumber),
            Enum1153::PurchaseOrderResponseNumber => Ok(crate::zugferd_2_3_2::Enum1153::PurchaseOrderResponseNumber),
            Enum1153::PurchaseOrderChangeNumber => Ok(crate::zugferd_2_3_2::Enum1153::PurchaseOrderChangeNumber),
            Enum1153::PaymentReference => Ok(crate::zugferd_2_3_2::Enum1153::PaymentReference),
            Enum1153::PriceQuoteNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriceQuoteNumber),
            Enum1153::PurchaseOrderNumberSuffix => Ok(crate::zugferd_2_3_2::Enum1153::PurchaseOrderNumberSuffix),
            Enum1153::PriorPurchaseOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::PriorPurchaseOrderNumber),
            Enum1153::PayeesFinancialInstitutionAccountNumber => Ok(crate::zugferd_2_3_2::Enum1153::PayeesFinancialInstitutionAccountNumber),
            Enum1153::RemittanceAdviceNumber => Ok(crate::zugferd_2_3_2::Enum1153::RemittanceAdviceNumber),
            Enum1153::RailRoadRoutingCode => Ok(crate::zugferd_2_3_2::Enum1153::RailRoadRoutingCode),
            Enum1153::RailwayConsignmentNoteNumber => Ok(crate::zugferd_2_3_2::Enum1153::RailwayConsignmentNoteNumber),
            Enum1153::ReleaseNumber => Ok(crate::zugferd_2_3_2::Enum1153::ReleaseNumber),
            Enum1153::ConsignmentReceiptIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::ConsignmentReceiptIdentifier),
            Enum1153::ExportReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ExportReferenceNumber),
            Enum1153::PayersFinancialInstitutionTransitRoutingNoAch => Ok(crate::zugferd_2_3_2::Enum1153::PayersFinancialInstitutionTransitRoutingNoAch),
            Enum1153::PayeesFinancialInstitutionTransitRoutingNo => Ok(crate::zugferd_2_3_2::Enum1153::PayeesFinancialInstitutionTransitRoutingNo),
            Enum1153::SalesPersonNumber => Ok(crate::zugferd_2_3_2::Enum1153::SalesPersonNumber),
            Enum1153::SalesRegionNumber => Ok(crate::zugferd_2_3_2::Enum1153::SalesRegionNumber),
            Enum1153::SalesDepartmentNumber => Ok(crate::zugferd_2_3_2::Enum1153::SalesDepartmentNumber),
            Enum1153::SerialNumber => Ok(crate::zugferd_2_3_2::Enum1153::SerialNumber),
            Enum1153::AllocatedSeat => Ok(crate::zugferd_2_3_2::Enum1153::AllocatedSeat),
            Enum1153::ShipFrom => Ok(crate::zugferd_2_3_2::Enum1153::ShipFrom),
            Enum1153::PreviousHighestScheduleNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousHighestScheduleNumber),
            Enum1153::SidShippersIdentifyingNumberForShipment => Ok(crate::zugferd_2_3_2::Enum1153::SidShippersIdentifyingNumberForShipment),
            Enum1153::SalesOfficeNumber => Ok(crate::zugferd_2_3_2::Enum1153::SalesOfficeNumber),
            Enum1153::TransportEquipmentSealIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentSealIdentifier),
            Enum1153::ScanLine => Ok(crate::zugferd_2_3_2::Enum1153::ScanLine),
            Enum1153::EquipmentSequenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::EquipmentSequenceNumber),
            Enum1153::ShipmentReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::ShipmentReferenceNumber),
            Enum1153::SellersReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::SellersReferenceNumber),
            Enum1153::StationReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::StationReferenceNumber),
            Enum1153::SwapOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::SwapOrderNumber),
            Enum1153::SpecificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::SpecificationNumber),
            Enum1153::TruckersBillLading => Ok(crate::zugferd_2_3_2::Enum1153::TruckersBillLading),
            Enum1153::TerminalOperatorsConsignmentReference => Ok(crate::zugferd_2_3_2::Enum1153::TerminalOperatorsConsignmentReference),
            Enum1153::TelexMessageNumber => Ok(crate::zugferd_2_3_2::Enum1153::TelexMessageNumber),
            Enum1153::TransferNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransferNumber),
            Enum1153::TirCarnetNumber => Ok(crate::zugferd_2_3_2::Enum1153::TirCarnetNumber),
            Enum1153::TransportInstructionNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransportInstructionNumber),
            Enum1153::TaxExemptionLicenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::TaxExemptionLicenceNumber),
            Enum1153::TransactionReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::TransactionReferenceNumber),
            Enum1153::TestReportNumber => Ok(crate::zugferd_2_3_2::Enum1153::TestReportNumber),
            Enum1153::UpperNumberRange => Ok(crate::zugferd_2_3_2::Enum1153::UpperNumberRange),
            Enum1153::UltimateCustomersReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::UltimateCustomersReferenceNumber),
            Enum1153::UniqueConsignmentReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::UniqueConsignmentReferenceNumber),
            Enum1153::UnitedNationsDangerousGoodsIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::UnitedNationsDangerousGoodsIdentifier),
            Enum1153::UltimateCustomersOrderNumber => Ok(crate::zugferd_2_3_2::Enum1153::UltimateCustomersOrderNumber),
            Enum1153::UniformResourceIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::UniformResourceIdentifier),
            Enum1153::VatRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::VatRegistrationNumber),
            Enum1153::VendorContractNumber => Ok(crate::zugferd_2_3_2::Enum1153::VendorContractNumber),
            Enum1153::TransportEquipmentGrossMassVerificationReference => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentGrossMassVerificationReference),
            Enum1153::VesselIdentifier => Ok(crate::zugferd_2_3_2::Enum1153::VesselIdentifier),
            Enum1153::OrderNumberVendor => Ok(crate::zugferd_2_3_2::Enum1153::OrderNumberVendor),
            Enum1153::VoyageNumber => Ok(crate::zugferd_2_3_2::Enum1153::VoyageNumber),
            Enum1153::TransportEquipmentGrossMassVerificationOrderReference => Ok(crate::zugferd_2_3_2::Enum1153::TransportEquipmentGrossMassVerificationOrderReference),
            Enum1153::VendorProductNumber => Ok(crate::zugferd_2_3_2::Enum1153::VendorProductNumber),
            Enum1153::VendorIdNumber => Ok(crate::zugferd_2_3_2::Enum1153::VendorIdNumber),
            Enum1153::VendorOrderNumberSuffix => Ok(crate::zugferd_2_3_2::Enum1153::VendorOrderNumberSuffix),
            Enum1153::MotorVehicleIdentificationNumber => Ok(crate::zugferd_2_3_2::Enum1153::MotorVehicleIdentificationNumber),
            Enum1153::VoucherNumber => Ok(crate::zugferd_2_3_2::Enum1153::VoucherNumber),
            Enum1153::WarehouseEntryNumber => Ok(crate::zugferd_2_3_2::Enum1153::WarehouseEntryNumber),
            Enum1153::WeightAgreementNumber => Ok(crate::zugferd_2_3_2::Enum1153::WeightAgreementNumber),
            Enum1153::WellNumber => Ok(crate::zugferd_2_3_2::Enum1153::WellNumber),
            Enum1153::WarehouseReceiptNumber => Ok(crate::zugferd_2_3_2::Enum1153::WarehouseReceiptNumber),
            Enum1153::WarehouseStorageLocationNumber => Ok(crate::zugferd_2_3_2::Enum1153::WarehouseStorageLocationNumber),
            Enum1153::RailWaybillNumber => Ok(crate::zugferd_2_3_2::Enum1153::RailWaybillNumber),
            Enum1153::CompanyPlaceRegistrationNumber => Ok(crate::zugferd_2_3_2::Enum1153::CompanyPlaceRegistrationNumber),
            Enum1153::CargoControlNumber => Ok(crate::zugferd_2_3_2::Enum1153::CargoControlNumber),
            Enum1153::PreviousCargoControlNumber => Ok(crate::zugferd_2_3_2::Enum1153::PreviousCargoControlNumber),
            Enum1153::MutuallyDefinedReferenceNumber => Ok(crate::zugferd_2_3_2::Enum1153::MutuallyDefinedReferenceNumber),
            Enum1153::BlockStowageReference => Err(ErrFromEnum1153ToCrateZugferd232Enum1153::BlockStowageReference),
        }
    }
}

/// All the variants of Enum1153 that are not matched to any variant of crate::zugferd_2_3_2::Enum1153
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrFromEnum1153ToCrateZugferd232Enum1153 {
    BlockStowageReference,
}

impl std::fmt::Display for ErrFromEnum1153ToCrateZugferd232Enum1153 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrFromEnum1153ToCrateZugferd232Enum1153::BlockStowageReference => write!(f, "BlockStowageReference has no corresponding value in crate::zugferd_2_3_2::Enum1153"),
        }
    }
}

impl std::error::Error for ErrFromEnum1153ToCrateZugferd232Enum1153 {}

impl std::convert::TryFrom<crate::zugferd_2_3_2::Enum1153> for Enum1153 {
    type Error = std::convert::Infallible;
    fn try_from(value: crate::zugferd_2_3_2::Enum1153) -> Result<Enum1153, Self::Error> {
        match value {
            crate::zugferd_2_3_2::Enum1153::OrderAcknowledgementDocumentIdentifier => Ok(Enum1153::OrderAcknowledgementDocumentIdentifier),
            crate::zugferd_2_3_2::Enum1153::ProformaInvoiceDocumentIdentifier => Ok(Enum1153::ProformaInvoiceDocumentIdentifier),
            crate::zugferd_2_3_2::Enum1153::DocumentaryCreditIdentifier => Ok(Enum1153::DocumentaryCreditIdentifier),
            crate::zugferd_2_3_2::Enum1153::ContractDocumentAddendumIdentifier => Ok(Enum1153::ContractDocumentAddendumIdentifier),
            crate::zugferd_2_3_2::Enum1153::GoodsDeclarationNumber => Ok(Enum1153::GoodsDeclarationNumber),
            crate::zugferd_2_3_2::Enum1153::DebitCardNumber => Ok(Enum1153::DebitCardNumber),
            crate::zugferd_2_3_2::Enum1153::OfferNumber => Ok(Enum1153::OfferNumber),
            crate::zugferd_2_3_2::Enum1153::BanksBatchInterbankTransactionReferenceNumber => Ok(Enum1153::BanksBatchInterbankTransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::BanksIndividualInterbankTransactionReferenceNumber => Ok(Enum1153::BanksIndividualInterbankTransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::DeliveryOrderNumber => Ok(Enum1153::DeliveryOrderNumber),
            crate::zugferd_2_3_2::Enum1153::DespatchAdviceNumber => Ok(Enum1153::DespatchAdviceNumber),
            crate::zugferd_2_3_2::Enum1153::DrawingNumber => Ok(Enum1153::DrawingNumber),
            crate::zugferd_2_3_2::Enum1153::WaybillNumber => Ok(Enum1153::WaybillNumber),
            crate::zugferd_2_3_2::Enum1153::DeliveryScheduleNumber => Ok(Enum1153::DeliveryScheduleNumber),
            crate::zugferd_2_3_2::Enum1153::ConsignmentIdentifierConsigneeAssigned => Ok(Enum1153::ConsignmentIdentifierConsigneeAssigned),
            crate::zugferd_2_3_2::Enum1153::PartialShipmentIdentifier => Ok(Enum1153::PartialShipmentIdentifier),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentIdentifier => Ok(Enum1153::TransportEquipmentIdentifier),
            crate::zugferd_2_3_2::Enum1153::MunicipalityAssignedBusinessRegistryNumber => Ok(Enum1153::MunicipalityAssignedBusinessRegistryNumber),
            crate::zugferd_2_3_2::Enum1153::TransportContractDocumentIdentifier => Ok(Enum1153::TransportContractDocumentIdentifier),
            crate::zugferd_2_3_2::Enum1153::MasterLabelNumber => Ok(Enum1153::MasterLabelNumber),
            crate::zugferd_2_3_2::Enum1153::DespatchNoteDocumentIdentifier => Ok(Enum1153::DespatchNoteDocumentIdentifier),
            crate::zugferd_2_3_2::Enum1153::EnquiryNumber => Ok(Enum1153::EnquiryNumber),
            crate::zugferd_2_3_2::Enum1153::DocketNumber => Ok(Enum1153::DocketNumber),
            crate::zugferd_2_3_2::Enum1153::CivilActionNumber => Ok(Enum1153::CivilActionNumber),
            crate::zugferd_2_3_2::Enum1153::CarriersAgentReferenceNumber => Ok(Enum1153::CarriersAgentReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::StandardCarrierAlphaCodeScacNumber => Ok(Enum1153::StandardCarrierAlphaCodeScacNumber),
            crate::zugferd_2_3_2::Enum1153::CustomsValuationDecisionNumber => Ok(Enum1153::CustomsValuationDecisionNumber),
            crate::zugferd_2_3_2::Enum1153::EndUseAuthorizationNumber => Ok(Enum1153::EndUseAuthorizationNumber),
            crate::zugferd_2_3_2::Enum1153::AntiDumpingCaseNumber => Ok(Enum1153::AntiDumpingCaseNumber),
            crate::zugferd_2_3_2::Enum1153::CustomsTariffNumber => Ok(Enum1153::CustomsTariffNumber),
            crate::zugferd_2_3_2::Enum1153::DeclarantsReferenceNumber => Ok(Enum1153::DeclarantsReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::RepairEstimateNumber => Ok(Enum1153::RepairEstimateNumber),
            crate::zugferd_2_3_2::Enum1153::CustomsDecisionRequestNumber => Ok(Enum1153::CustomsDecisionRequestNumber),
            crate::zugferd_2_3_2::Enum1153::SubHouseBillLadingNumber => Ok(Enum1153::SubHouseBillLadingNumber),
            crate::zugferd_2_3_2::Enum1153::TaxPaymentIdentifier => Ok(Enum1153::TaxPaymentIdentifier),
            crate::zugferd_2_3_2::Enum1153::QuotaNumber => Ok(Enum1153::QuotaNumber),
            crate::zugferd_2_3_2::Enum1153::TransitOnwardCarriageGuaranteeBondNumber => Ok(Enum1153::TransitOnwardCarriageGuaranteeBondNumber),
            crate::zugferd_2_3_2::Enum1153::CustomsGuaranteeNumber => Ok(Enum1153::CustomsGuaranteeNumber),
            crate::zugferd_2_3_2::Enum1153::ReplacingPartNumber => Ok(Enum1153::ReplacingPartNumber),
            crate::zugferd_2_3_2::Enum1153::SellersCatalogueNumber => Ok(Enum1153::SellersCatalogueNumber),
            crate::zugferd_2_3_2::Enum1153::OriginatorsReference => Ok(Enum1153::OriginatorsReference),
            crate::zugferd_2_3_2::Enum1153::DeclarantsCustomsIdentityNumber => Ok(Enum1153::DeclarantsCustomsIdentityNumber),
            crate::zugferd_2_3_2::Enum1153::ImporterReferenceNumber => Ok(Enum1153::ImporterReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ExportClearanceInstructionReferenceNumber => Ok(Enum1153::ExportClearanceInstructionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ImportClearanceInstructionReferenceNumber => Ok(Enum1153::ImportClearanceInstructionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::GoodsDeclarationDocumentIdentifierCustoms => Ok(Enum1153::GoodsDeclarationDocumentIdentifierCustoms),
            crate::zugferd_2_3_2::Enum1153::ArticleNumber => Ok(Enum1153::ArticleNumber),
            crate::zugferd_2_3_2::Enum1153::IntraPlantRouting => Ok(Enum1153::IntraPlantRouting),
            crate::zugferd_2_3_2::Enum1153::StockKeepingUnitNumber => Ok(Enum1153::StockKeepingUnitNumber),
            crate::zugferd_2_3_2::Enum1153::TextElementIdentifierDeletionReference => Ok(Enum1153::TextElementIdentifierDeletionReference),
            crate::zugferd_2_3_2::Enum1153::AllotmentIdentificationAir => Ok(Enum1153::AllotmentIdentificationAir),
            crate::zugferd_2_3_2::Enum1153::VehicleLicenceNumber => Ok(Enum1153::VehicleLicenceNumber),
            crate::zugferd_2_3_2::Enum1153::AirCargoTransferManifest => Ok(Enum1153::AirCargoTransferManifest),
            crate::zugferd_2_3_2::Enum1153::CargoAcceptanceOrderReferenceNumber => Ok(Enum1153::CargoAcceptanceOrderReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::UsGovernmentAgencyNumber => Ok(Enum1153::UsGovernmentAgencyNumber),
            crate::zugferd_2_3_2::Enum1153::ShippingUnitIdentification => Ok(Enum1153::ShippingUnitIdentification),
            crate::zugferd_2_3_2::Enum1153::AdditionalReferenceNumber => Ok(Enum1153::AdditionalReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::RelatedDocumentNumber => Ok(Enum1153::RelatedDocumentNumber),
            crate::zugferd_2_3_2::Enum1153::AddresseeReference => Ok(Enum1153::AddresseeReference),
            crate::zugferd_2_3_2::Enum1153::AtaCarnetNumber => Ok(Enum1153::AtaCarnetNumber),
            crate::zugferd_2_3_2::Enum1153::PackagingUnitIdentification => Ok(Enum1153::PackagingUnitIdentification),
            crate::zugferd_2_3_2::Enum1153::OuterpackagingUnitIdentification => Ok(Enum1153::OuterpackagingUnitIdentification),
            crate::zugferd_2_3_2::Enum1153::CustomerMaterialSpecificationNumber => Ok(Enum1153::CustomerMaterialSpecificationNumber),
            crate::zugferd_2_3_2::Enum1153::BankReference => Ok(Enum1153::BankReference),
            crate::zugferd_2_3_2::Enum1153::PrincipalReferenceNumber => Ok(Enum1153::PrincipalReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CollectionAdviceDocumentIdentifier => Ok(Enum1153::CollectionAdviceDocumentIdentifier),
            crate::zugferd_2_3_2::Enum1153::IronChargeNumber => Ok(Enum1153::IronChargeNumber),
            crate::zugferd_2_3_2::Enum1153::HotRollNumber => Ok(Enum1153::HotRollNumber),
            crate::zugferd_2_3_2::Enum1153::ColdRollNumber => Ok(Enum1153::ColdRollNumber),
            crate::zugferd_2_3_2::Enum1153::RailwayWagonNumber => Ok(Enum1153::RailwayWagonNumber),
            crate::zugferd_2_3_2::Enum1153::UniqueClaimsReferenceNumberSender => Ok(Enum1153::UniqueClaimsReferenceNumberSender),
            crate::zugferd_2_3_2::Enum1153::LossEventNumber => Ok(Enum1153::LossEventNumber),
            crate::zugferd_2_3_2::Enum1153::EstimateOrderReferenceNumber => Ok(Enum1153::EstimateOrderReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ReferenceNumberToPreviousMessage => Ok(Enum1153::ReferenceNumberToPreviousMessage),
            crate::zugferd_2_3_2::Enum1153::BankersAcceptance => Ok(Enum1153::BankersAcceptance),
            crate::zugferd_2_3_2::Enum1153::DutyMemoNumber => Ok(Enum1153::DutyMemoNumber),
            crate::zugferd_2_3_2::Enum1153::EquipmentTransportChargeNumber => Ok(Enum1153::EquipmentTransportChargeNumber),
            crate::zugferd_2_3_2::Enum1153::BuyersItemNumber => Ok(Enum1153::BuyersItemNumber),
            crate::zugferd_2_3_2::Enum1153::MaturedCertificateDeposit => Ok(Enum1153::MaturedCertificateDeposit),
            crate::zugferd_2_3_2::Enum1153::Loan => Ok(Enum1153::Loan),
            crate::zugferd_2_3_2::Enum1153::AnalysisNumberTestNumber => Ok(Enum1153::AnalysisNumberTestNumber),
            crate::zugferd_2_3_2::Enum1153::AccountNumber => Ok(Enum1153::AccountNumber),
            crate::zugferd_2_3_2::Enum1153::TreatyNumber => Ok(Enum1153::TreatyNumber),
            crate::zugferd_2_3_2::Enum1153::CatastropheNumber => Ok(Enum1153::CatastropheNumber),
            crate::zugferd_2_3_2::Enum1153::BureauSigningStatementReference => Ok(Enum1153::BureauSigningStatementReference),
            crate::zugferd_2_3_2::Enum1153::CompanySyndicateReference1 => Ok(Enum1153::CompanySyndicateReference1),
            crate::zugferd_2_3_2::Enum1153::CompanySyndicateReference2 => Ok(Enum1153::CompanySyndicateReference2),
            crate::zugferd_2_3_2::Enum1153::OrderingCustomerConsignmentReferenceNumber => Ok(Enum1153::OrderingCustomerConsignmentReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ShipownersAuthorizationNumber => Ok(Enum1153::ShipownersAuthorizationNumber),
            crate::zugferd_2_3_2::Enum1153::InlandTransportOrderNumber => Ok(Enum1153::InlandTransportOrderNumber),
            crate::zugferd_2_3_2::Enum1153::ContainerWorkOrderReferenceNumber => Ok(Enum1153::ContainerWorkOrderReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::StatementNumber => Ok(Enum1153::StatementNumber),
            crate::zugferd_2_3_2::Enum1153::UniqueMarketReference => Ok(Enum1153::UniqueMarketReference),
            crate::zugferd_2_3_2::Enum1153::GroupAccounting => Ok(Enum1153::GroupAccounting),
            crate::zugferd_2_3_2::Enum1153::BrokerReference1 => Ok(Enum1153::BrokerReference1),
            crate::zugferd_2_3_2::Enum1153::BrokerReference2 => Ok(Enum1153::BrokerReference2),
            crate::zugferd_2_3_2::Enum1153::LloydsClaimsOfficeReference => Ok(Enum1153::LloydsClaimsOfficeReference),
            crate::zugferd_2_3_2::Enum1153::SecureDeliveryTermsAndConditionsAgreementReference => Ok(Enum1153::SecureDeliveryTermsAndConditionsAgreementReference),
            crate::zugferd_2_3_2::Enum1153::ReportNumber => Ok(Enum1153::ReportNumber),
            crate::zugferd_2_3_2::Enum1153::TraderAccountNumber => Ok(Enum1153::TraderAccountNumber),
            crate::zugferd_2_3_2::Enum1153::AuthorizationForExpenseAfeNumber => Ok(Enum1153::AuthorizationForExpenseAfeNumber),
            crate::zugferd_2_3_2::Enum1153::GovernmentAgencyReferenceNumber => Ok(Enum1153::GovernmentAgencyReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::AssemblyNumber => Ok(Enum1153::AssemblyNumber),
            crate::zugferd_2_3_2::Enum1153::SymbolNumber => Ok(Enum1153::SymbolNumber),
            crate::zugferd_2_3_2::Enum1153::CommodityNumber => Ok(Enum1153::CommodityNumber),
            crate::zugferd_2_3_2::Enum1153::Eur1CertificateNumber => Ok(Enum1153::Eur1CertificateNumber),
            crate::zugferd_2_3_2::Enum1153::CustomerProcessSpecificationNumber => Ok(Enum1153::CustomerProcessSpecificationNumber),
            crate::zugferd_2_3_2::Enum1153::CustomerSpecificationNumber => Ok(Enum1153::CustomerSpecificationNumber),
            crate::zugferd_2_3_2::Enum1153::ApplicableInstructionsOrStandards => Ok(Enum1153::ApplicableInstructionsOrStandards),
            crate::zugferd_2_3_2::Enum1153::RegistrationNumberPreviousCustomsDeclaration => Ok(Enum1153::RegistrationNumberPreviousCustomsDeclaration),
            crate::zugferd_2_3_2::Enum1153::PostEntryReference => Ok(Enum1153::PostEntryReference),
            crate::zugferd_2_3_2::Enum1153::PaymentOrderNumber => Ok(Enum1153::PaymentOrderNumber),
            crate::zugferd_2_3_2::Enum1153::DeliveryNumberTransport => Ok(Enum1153::DeliveryNumberTransport),
            crate::zugferd_2_3_2::Enum1153::TransportRoute => Ok(Enum1153::TransportRoute),
            crate::zugferd_2_3_2::Enum1153::CustomersUnitInventoryNumber => Ok(Enum1153::CustomersUnitInventoryNumber),
            crate::zugferd_2_3_2::Enum1153::ProductReservationNumber => Ok(Enum1153::ProductReservationNumber),
            crate::zugferd_2_3_2::Enum1153::ProjectNumber => Ok(Enum1153::ProjectNumber),
            crate::zugferd_2_3_2::Enum1153::DrawingListNumber => Ok(Enum1153::DrawingListNumber),
            crate::zugferd_2_3_2::Enum1153::ProjectSpecificationNumber => Ok(Enum1153::ProjectSpecificationNumber),
            crate::zugferd_2_3_2::Enum1153::PrimaryReference => Ok(Enum1153::PrimaryReference),
            crate::zugferd_2_3_2::Enum1153::RequestForCancellationNumber => Ok(Enum1153::RequestForCancellationNumber),
            crate::zugferd_2_3_2::Enum1153::SuppliersControlNumber => Ok(Enum1153::SuppliersControlNumber),
            crate::zugferd_2_3_2::Enum1153::ShippingNoteNumber => Ok(Enum1153::ShippingNoteNumber),
            crate::zugferd_2_3_2::Enum1153::EmptyContainerBillNumber => Ok(Enum1153::EmptyContainerBillNumber),
            crate::zugferd_2_3_2::Enum1153::NonNegotiableMaritimeTransportDocumentNumber => Ok(Enum1153::NonNegotiableMaritimeTransportDocumentNumber),
            crate::zugferd_2_3_2::Enum1153::SubstituteAirWaybillNumber => Ok(Enum1153::SubstituteAirWaybillNumber),
            crate::zugferd_2_3_2::Enum1153::DespatchNotePostParcelsNumber => Ok(Enum1153::DespatchNotePostParcelsNumber),
            crate::zugferd_2_3_2::Enum1153::AirlinesFlightIdentificationNumber => Ok(Enum1153::AirlinesFlightIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::ThroughBillLadingNumber => Ok(Enum1153::ThroughBillLadingNumber),
            crate::zugferd_2_3_2::Enum1153::CargoManifestNumber => Ok(Enum1153::CargoManifestNumber),
            crate::zugferd_2_3_2::Enum1153::BordereauNumber => Ok(Enum1153::BordereauNumber),
            crate::zugferd_2_3_2::Enum1153::CustomsItemNumber => Ok(Enum1153::CustomsItemNumber),
            crate::zugferd_2_3_2::Enum1153::ExportControlCommodityNumberEccn => Ok(Enum1153::ExportControlCommodityNumberEccn),
            crate::zugferd_2_3_2::Enum1153::MarkingLabelReference => Ok(Enum1153::MarkingLabelReference),
            crate::zugferd_2_3_2::Enum1153::TariffNumber => Ok(Enum1153::TariffNumber),
            crate::zugferd_2_3_2::Enum1153::ReplenishmentPurchaseOrderNumber => Ok(Enum1153::ReplenishmentPurchaseOrderNumber),
            crate::zugferd_2_3_2::Enum1153::ImmediateTransportationNoForInBondMovement => Ok(Enum1153::ImmediateTransportationNoForInBondMovement),
            crate::zugferd_2_3_2::Enum1153::TransportationExportationNoForInBondMovement => Ok(Enum1153::TransportationExportationNoForInBondMovement),
            crate::zugferd_2_3_2::Enum1153::ImmediateExportationNoForInBondMovement => Ok(Enum1153::ImmediateExportationNoForInBondMovement),
            crate::zugferd_2_3_2::Enum1153::AssociatedInvoices => Ok(Enum1153::AssociatedInvoices),
            crate::zugferd_2_3_2::Enum1153::SecondaryCustomsReference => Ok(Enum1153::SecondaryCustomsReference),
            crate::zugferd_2_3_2::Enum1153::AccountPartysReference => Ok(Enum1153::AccountPartysReference),
            crate::zugferd_2_3_2::Enum1153::BeneficiarysReference => Ok(Enum1153::BeneficiarysReference),
            crate::zugferd_2_3_2::Enum1153::SecondBeneficiarysReference => Ok(Enum1153::SecondBeneficiarysReference),
            crate::zugferd_2_3_2::Enum1153::ApplicantsBankReference => Ok(Enum1153::ApplicantsBankReference),
            crate::zugferd_2_3_2::Enum1153::IssuingBanksReference => Ok(Enum1153::IssuingBanksReference),
            crate::zugferd_2_3_2::Enum1153::BeneficiarysBankReference => Ok(Enum1153::BeneficiarysBankReference),
            crate::zugferd_2_3_2::Enum1153::DirectPaymentValuationNumber => Ok(Enum1153::DirectPaymentValuationNumber),
            crate::zugferd_2_3_2::Enum1153::DirectPaymentValuationRequestNumber => Ok(Enum1153::DirectPaymentValuationRequestNumber),
            crate::zugferd_2_3_2::Enum1153::QuantityValuationNumber => Ok(Enum1153::QuantityValuationNumber),
            crate::zugferd_2_3_2::Enum1153::QuantityValuationRequestNumber => Ok(Enum1153::QuantityValuationRequestNumber),
            crate::zugferd_2_3_2::Enum1153::BillQuantitiesNumber => Ok(Enum1153::BillQuantitiesNumber),
            crate::zugferd_2_3_2::Enum1153::PaymentValuationNumber => Ok(Enum1153::PaymentValuationNumber),
            crate::zugferd_2_3_2::Enum1153::SituationNumber => Ok(Enum1153::SituationNumber),
            crate::zugferd_2_3_2::Enum1153::AgreementToPayNumber => Ok(Enum1153::AgreementToPayNumber),
            crate::zugferd_2_3_2::Enum1153::ContractPartyReferenceNumber => Ok(Enum1153::ContractPartyReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::AccountPartysBankReference => Ok(Enum1153::AccountPartysBankReference),
            crate::zugferd_2_3_2::Enum1153::AgentsBankReference => Ok(Enum1153::AgentsBankReference),
            crate::zugferd_2_3_2::Enum1153::AgentsReference => Ok(Enum1153::AgentsReference),
            crate::zugferd_2_3_2::Enum1153::ApplicantsReference => Ok(Enum1153::ApplicantsReference),
            crate::zugferd_2_3_2::Enum1153::DisputeNumber => Ok(Enum1153::DisputeNumber),
            crate::zugferd_2_3_2::Enum1153::CreditRatingAgencysReferenceNumber => Ok(Enum1153::CreditRatingAgencysReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::RequestNumber => Ok(Enum1153::RequestNumber),
            crate::zugferd_2_3_2::Enum1153::SingleTransactionSequenceNumber => Ok(Enum1153::SingleTransactionSequenceNumber),
            crate::zugferd_2_3_2::Enum1153::ApplicationReferenceNumber => Ok(Enum1153::ApplicationReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::DeliveryVerificationCertificate => Ok(Enum1153::DeliveryVerificationCertificate),
            crate::zugferd_2_3_2::Enum1153::NumberTemporaryImportationDocument => Ok(Enum1153::NumberTemporaryImportationDocument),
            crate::zugferd_2_3_2::Enum1153::ReferenceNumberQuotedOnStatement => Ok(Enum1153::ReferenceNumberQuotedOnStatement),
            crate::zugferd_2_3_2::Enum1153::SendersReferenceToOriginalMessage => Ok(Enum1153::SendersReferenceToOriginalMessage),
            crate::zugferd_2_3_2::Enum1153::CompanyIssuedEquipmentId => Ok(Enum1153::CompanyIssuedEquipmentId),
            crate::zugferd_2_3_2::Enum1153::DomesticFlightNumber => Ok(Enum1153::DomesticFlightNumber),
            crate::zugferd_2_3_2::Enum1153::InternationalFlightNumber => Ok(Enum1153::InternationalFlightNumber),
            crate::zugferd_2_3_2::Enum1153::EmployerIdentificationNumberServiceBureau => Ok(Enum1153::EmployerIdentificationNumberServiceBureau),
            crate::zugferd_2_3_2::Enum1153::ServiceGroupIdentificationNumber => Ok(Enum1153::ServiceGroupIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::MemberNumber => Ok(Enum1153::MemberNumber),
            crate::zugferd_2_3_2::Enum1153::PreviousMemberNumber => Ok(Enum1153::PreviousMemberNumber),
            crate::zugferd_2_3_2::Enum1153::SchemePlanNumber => Ok(Enum1153::SchemePlanNumber),
            crate::zugferd_2_3_2::Enum1153::PreviousSchemePlanNumber => Ok(Enum1153::PreviousSchemePlanNumber),
            crate::zugferd_2_3_2::Enum1153::ReceivingPartysMemberIdentification => Ok(Enum1153::ReceivingPartysMemberIdentification),
            crate::zugferd_2_3_2::Enum1153::PayrollNumber => Ok(Enum1153::PayrollNumber),
            crate::zugferd_2_3_2::Enum1153::PackagingSpecificationNumber => Ok(Enum1153::PackagingSpecificationNumber),
            crate::zugferd_2_3_2::Enum1153::AuthorityIssuedEquipmentIdentification => Ok(Enum1153::AuthorityIssuedEquipmentIdentification),
            crate::zugferd_2_3_2::Enum1153::TrainingFlightNumber => Ok(Enum1153::TrainingFlightNumber),
            crate::zugferd_2_3_2::Enum1153::FundCodeNumber => Ok(Enum1153::FundCodeNumber),
            crate::zugferd_2_3_2::Enum1153::SignalCodeNumber => Ok(Enum1153::SignalCodeNumber),
            crate::zugferd_2_3_2::Enum1153::MajorForceProgramNumber => Ok(Enum1153::MajorForceProgramNumber),
            crate::zugferd_2_3_2::Enum1153::NominationNumber => Ok(Enum1153::NominationNumber),
            crate::zugferd_2_3_2::Enum1153::LaboratoryRegistrationNumber => Ok(Enum1153::LaboratoryRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::TransportContractReferenceNumber => Ok(Enum1153::TransportContractReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::PayeesReferenceNumber => Ok(Enum1153::PayeesReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::PayersReferenceNumber => Ok(Enum1153::PayersReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CreditorsReferenceNumber => Ok(Enum1153::CreditorsReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::DebtorsReferenceNumber => Ok(Enum1153::DebtorsReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::JointVentureReferenceNumber => Ok(Enum1153::JointVentureReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ChamberCommerceRegistrationNumber => Ok(Enum1153::ChamberCommerceRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::TaxRegistrationNumber => Ok(Enum1153::TaxRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::WoolIdentificationNumber => Ok(Enum1153::WoolIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::WoolTaxReferenceNumber => Ok(Enum1153::WoolTaxReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::MeatProcessingEstablishmentRegistrationNumber => Ok(Enum1153::MeatProcessingEstablishmentRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::QuarantineTreatmentStatusReferenceNumber => Ok(Enum1153::QuarantineTreatmentStatusReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::RequestForQuoteNumber => Ok(Enum1153::RequestForQuoteNumber),
            crate::zugferd_2_3_2::Enum1153::ManualProcessingAuthorityNumber => Ok(Enum1153::ManualProcessingAuthorityNumber),
            crate::zugferd_2_3_2::Enum1153::RateNoteNumber => Ok(Enum1153::RateNoteNumber),
            crate::zugferd_2_3_2::Enum1153::FreightForwarderNumber => Ok(Enum1153::FreightForwarderNumber),
            crate::zugferd_2_3_2::Enum1153::CustomsReleaseCode => Ok(Enum1153::CustomsReleaseCode),
            crate::zugferd_2_3_2::Enum1153::ComplianceCodeNumber => Ok(Enum1153::ComplianceCodeNumber),
            crate::zugferd_2_3_2::Enum1153::DepartmentTransportationBondNumber => Ok(Enum1153::DepartmentTransportationBondNumber),
            crate::zugferd_2_3_2::Enum1153::ExportEstablishmentNumber => Ok(Enum1153::ExportEstablishmentNumber),
            crate::zugferd_2_3_2::Enum1153::CertificateConformity => Ok(Enum1153::CertificateConformity),
            crate::zugferd_2_3_2::Enum1153::MinisterialCertificateHomologation => Ok(Enum1153::MinisterialCertificateHomologation),
            crate::zugferd_2_3_2::Enum1153::PreviousDeliveryInstructionNumber => Ok(Enum1153::PreviousDeliveryInstructionNumber),
            crate::zugferd_2_3_2::Enum1153::PassportNumber => Ok(Enum1153::PassportNumber),
            crate::zugferd_2_3_2::Enum1153::CommonTransactionReferenceNumber => Ok(Enum1153::CommonTransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::BanksCommonTransactionReferenceNumber => Ok(Enum1153::BanksCommonTransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CustomersIndividualTransactionReferenceNumber => Ok(Enum1153::CustomersIndividualTransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::BanksIndividualTransactionReferenceNumber => Ok(Enum1153::BanksIndividualTransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CustomersCommonTransactionReferenceNumber => Ok(Enum1153::CustomersCommonTransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::IndividualTransactionReferenceNumber => Ok(Enum1153::IndividualTransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ProductSourcingAgreementNumber => Ok(Enum1153::ProductSourcingAgreementNumber),
            crate::zugferd_2_3_2::Enum1153::CustomsTranshipmentNumber => Ok(Enum1153::CustomsTranshipmentNumber),
            crate::zugferd_2_3_2::Enum1153::CustomsPreferenceInquiryNumber => Ok(Enum1153::CustomsPreferenceInquiryNumber),
            crate::zugferd_2_3_2::Enum1153::PackingPlantNumber => Ok(Enum1153::PackingPlantNumber),
            crate::zugferd_2_3_2::Enum1153::OriginalCertificateNumber => Ok(Enum1153::OriginalCertificateNumber),
            crate::zugferd_2_3_2::Enum1153::ProcessingPlantNumber => Ok(Enum1153::ProcessingPlantNumber),
            crate::zugferd_2_3_2::Enum1153::SlaughterPlantNumber => Ok(Enum1153::SlaughterPlantNumber),
            crate::zugferd_2_3_2::Enum1153::ChargeCardAccountNumber => Ok(Enum1153::ChargeCardAccountNumber),
            crate::zugferd_2_3_2::Enum1153::EventReferenceNumber => Ok(Enum1153::EventReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::TransportSectionReferenceNumber => Ok(Enum1153::TransportSectionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ReferredProductForMechanicalAnalysis => Ok(Enum1153::ReferredProductForMechanicalAnalysis),
            crate::zugferd_2_3_2::Enum1153::ReferredProductForChemicalAnalysis => Ok(Enum1153::ReferredProductForChemicalAnalysis),
            crate::zugferd_2_3_2::Enum1153::ConsolidatedInvoiceNumber => Ok(Enum1153::ConsolidatedInvoiceNumber),
            crate::zugferd_2_3_2::Enum1153::PartReferenceIndicatorInADrawing => Ok(Enum1153::PartReferenceIndicatorInADrawing),
            crate::zugferd_2_3_2::Enum1153::USCodeFederalRegulationsCfr => Ok(Enum1153::USCodeFederalRegulationsCfr),
            crate::zugferd_2_3_2::Enum1153::PurchasingActivityClauseNumber => Ok(Enum1153::PurchasingActivityClauseNumber),
            crate::zugferd_2_3_2::Enum1153::USDefenseFederalAcquisitionRegulationSupplement => Ok(Enum1153::USDefenseFederalAcquisitionRegulationSupplement),
            crate::zugferd_2_3_2::Enum1153::AgencyClauseNumber => Ok(Enum1153::AgencyClauseNumber),
            crate::zugferd_2_3_2::Enum1153::CircularPublicationNumber => Ok(Enum1153::CircularPublicationNumber),
            crate::zugferd_2_3_2::Enum1153::USFederalAcquisitionRegulation => Ok(Enum1153::USFederalAcquisitionRegulation),
            crate::zugferd_2_3_2::Enum1153::USGeneralServicesAdministrationRegulation => Ok(Enum1153::USGeneralServicesAdministrationRegulation),
            crate::zugferd_2_3_2::Enum1153::USFederalInformationResourcesManagementRegulation => Ok(Enum1153::USFederalInformationResourcesManagementRegulation),
            crate::zugferd_2_3_2::Enum1153::Paragraph => Ok(Enum1153::Paragraph),
            crate::zugferd_2_3_2::Enum1153::SpecialInstructionsNumber => Ok(Enum1153::SpecialInstructionsNumber),
            crate::zugferd_2_3_2::Enum1153::SiteSpecificProceduresTermsAndConditionsNumber => Ok(Enum1153::SiteSpecificProceduresTermsAndConditionsNumber),
            crate::zugferd_2_3_2::Enum1153::MasterSolicitationProceduresTermsAndConditions => Ok(Enum1153::MasterSolicitationProceduresTermsAndConditions),
            crate::zugferd_2_3_2::Enum1153::USDepartmentVeteransAffairsAcquisitionRegulation => Ok(Enum1153::USDepartmentVeteransAffairsAcquisitionRegulation),
            crate::zugferd_2_3_2::Enum1153::MilitaryInterdepartmentalPurchaseRequestMiprNumber => Ok(Enum1153::MilitaryInterdepartmentalPurchaseRequestMiprNumber),
            crate::zugferd_2_3_2::Enum1153::ForeignMilitarySalesNumber => Ok(Enum1153::ForeignMilitarySalesNumber),
            crate::zugferd_2_3_2::Enum1153::DefensePrioritiesAllocationSystemPriorityRating => Ok(Enum1153::DefensePrioritiesAllocationSystemPriorityRating),
            crate::zugferd_2_3_2::Enum1153::WageDeterminationNumber => Ok(Enum1153::WageDeterminationNumber),
            crate::zugferd_2_3_2::Enum1153::AgreementNumber => Ok(Enum1153::AgreementNumber),
            crate::zugferd_2_3_2::Enum1153::StandardIndustryClassificationSicNumber => Ok(Enum1153::StandardIndustryClassificationSicNumber),
            crate::zugferd_2_3_2::Enum1153::EndItemNumber => Ok(Enum1153::EndItemNumber),
            crate::zugferd_2_3_2::Enum1153::FederalSupplyScheduleItemNumber => Ok(Enum1153::FederalSupplyScheduleItemNumber),
            crate::zugferd_2_3_2::Enum1153::TechnicalDocumentNumber => Ok(Enum1153::TechnicalDocumentNumber),
            crate::zugferd_2_3_2::Enum1153::TechnicalOrderNumber => Ok(Enum1153::TechnicalOrderNumber),
            crate::zugferd_2_3_2::Enum1153::Suffix => Ok(Enum1153::Suffix),
            crate::zugferd_2_3_2::Enum1153::TransportationAccountNumber => Ok(Enum1153::TransportationAccountNumber),
            crate::zugferd_2_3_2::Enum1153::ContainerDispositionOrderReferenceNumber => Ok(Enum1153::ContainerDispositionOrderReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ContainerPrefix => Ok(Enum1153::ContainerPrefix),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentReturnReference => Ok(Enum1153::TransportEquipmentReturnReference),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentSurveyReference => Ok(Enum1153::TransportEquipmentSurveyReference),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentSurveyReportNumber => Ok(Enum1153::TransportEquipmentSurveyReportNumber),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentStuffingOrder => Ok(Enum1153::TransportEquipmentStuffingOrder),
            crate::zugferd_2_3_2::Enum1153::VehicleIdentificationNumberVin => Ok(Enum1153::VehicleIdentificationNumberVin),
            crate::zugferd_2_3_2::Enum1153::GovernmentBillLading => Ok(Enum1153::GovernmentBillLading),
            crate::zugferd_2_3_2::Enum1153::OrderingCustomersSecondReferenceNumber => Ok(Enum1153::OrderingCustomersSecondReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::DirectDebitReference => Ok(Enum1153::DirectDebitReference),
            crate::zugferd_2_3_2::Enum1153::MeterReadingAtBeginningDelivery => Ok(Enum1153::MeterReadingAtBeginningDelivery),
            crate::zugferd_2_3_2::Enum1153::MeterReadingAtEndDelivery => Ok(Enum1153::MeterReadingAtEndDelivery),
            crate::zugferd_2_3_2::Enum1153::ReplenishmentPurchaseOrderRangeStartNumber => Ok(Enum1153::ReplenishmentPurchaseOrderRangeStartNumber),
            crate::zugferd_2_3_2::Enum1153::ThirdBanksReference => Ok(Enum1153::ThirdBanksReference),
            crate::zugferd_2_3_2::Enum1153::ActionAuthorizationNumber => Ok(Enum1153::ActionAuthorizationNumber),
            crate::zugferd_2_3_2::Enum1153::AppropriationNumber => Ok(Enum1153::AppropriationNumber),
            crate::zugferd_2_3_2::Enum1153::ProductChangeAuthorityNumber => Ok(Enum1153::ProductChangeAuthorityNumber),
            crate::zugferd_2_3_2::Enum1153::GeneralCargoConsignmentReferenceNumber => Ok(Enum1153::GeneralCargoConsignmentReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CatalogueSequenceNumber => Ok(Enum1153::CatalogueSequenceNumber),
            crate::zugferd_2_3_2::Enum1153::ForwardingOrderNumber => Ok(Enum1153::ForwardingOrderNumber),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentSurveyReferenceNumber => Ok(Enum1153::TransportEquipmentSurveyReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::LeaseContractReference => Ok(Enum1153::LeaseContractReference),
            crate::zugferd_2_3_2::Enum1153::TransportCostsReferenceNumber => Ok(Enum1153::TransportCostsReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentStrippingOrder => Ok(Enum1153::TransportEquipmentStrippingOrder),
            crate::zugferd_2_3_2::Enum1153::PriorPolicyNumber => Ok(Enum1153::PriorPolicyNumber),
            crate::zugferd_2_3_2::Enum1153::PolicyNumber => Ok(Enum1153::PolicyNumber),
            crate::zugferd_2_3_2::Enum1153::ProcurementBudgetNumber => Ok(Enum1153::ProcurementBudgetNumber),
            crate::zugferd_2_3_2::Enum1153::DomesticInventoryManagementCode => Ok(Enum1153::DomesticInventoryManagementCode),
            crate::zugferd_2_3_2::Enum1153::CustomerReferenceNumberAssignedToPreviousBalance => Ok(Enum1153::CustomerReferenceNumberAssignedToPreviousBalance),
            crate::zugferd_2_3_2::Enum1153::PreviousCreditAdviceReferenceNumber => Ok(Enum1153::PreviousCreditAdviceReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ReportingFormNumber => Ok(Enum1153::ReportingFormNumber),
            crate::zugferd_2_3_2::Enum1153::AuthorizationNumberForExceptionToDangerousGoods => Ok(Enum1153::AuthorizationNumberForExceptionToDangerousGoods),
            crate::zugferd_2_3_2::Enum1153::DangerousGoodsSecurityNumber => Ok(Enum1153::DangerousGoodsSecurityNumber),
            crate::zugferd_2_3_2::Enum1153::DangerousGoodsTransportLicenceNumber => Ok(Enum1153::DangerousGoodsTransportLicenceNumber),
            crate::zugferd_2_3_2::Enum1153::PreviousRentalAgreementNumber => Ok(Enum1153::PreviousRentalAgreementNumber),
            crate::zugferd_2_3_2::Enum1153::NextRentalAgreementReasonNumber => Ok(Enum1153::NextRentalAgreementReasonNumber),
            crate::zugferd_2_3_2::Enum1153::ConsigneesInvoiceNumber => Ok(Enum1153::ConsigneesInvoiceNumber),
            crate::zugferd_2_3_2::Enum1153::MessageBatchNumber => Ok(Enum1153::MessageBatchNumber),
            crate::zugferd_2_3_2::Enum1153::PreviousDeliveryScheduleNumber => Ok(Enum1153::PreviousDeliveryScheduleNumber),
            crate::zugferd_2_3_2::Enum1153::PhysicalInventoryRecountReferenceNumber => Ok(Enum1153::PhysicalInventoryRecountReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ReceivingAdviceNumber => Ok(Enum1153::ReceivingAdviceNumber),
            crate::zugferd_2_3_2::Enum1153::ReturnableContainerReferenceNumber => Ok(Enum1153::ReturnableContainerReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ReturnsNoticeNumber => Ok(Enum1153::ReturnsNoticeNumber),
            crate::zugferd_2_3_2::Enum1153::SalesForecastNumber => Ok(Enum1153::SalesForecastNumber),
            crate::zugferd_2_3_2::Enum1153::SalesReportNumber => Ok(Enum1153::SalesReportNumber),
            crate::zugferd_2_3_2::Enum1153::PreviousTaxControlNumber => Ok(Enum1153::PreviousTaxControlNumber),
            crate::zugferd_2_3_2::Enum1153::AgerdAerospaceGroundEquipmentRequirementDataNumber => Ok(Enum1153::AgerdAerospaceGroundEquipmentRequirementDataNumber),
            crate::zugferd_2_3_2::Enum1153::RegisteredCapitalReference => Ok(Enum1153::RegisteredCapitalReference),
            crate::zugferd_2_3_2::Enum1153::StandardNumberInspectionDocument => Ok(Enum1153::StandardNumberInspectionDocument),
            crate::zugferd_2_3_2::Enum1153::Model => Ok(Enum1153::Model),
            crate::zugferd_2_3_2::Enum1153::FinancialManagementReference => Ok(Enum1153::FinancialManagementReference),
            crate::zugferd_2_3_2::Enum1153::NotificationForCollectionNumberNoticol => Ok(Enum1153::NotificationForCollectionNumberNoticol),
            crate::zugferd_2_3_2::Enum1153::PreviousRequestForMeteredReadingReferenceNumber => Ok(Enum1153::PreviousRequestForMeteredReadingReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::NextRentalAgreementNumber => Ok(Enum1153::NextRentalAgreementNumber),
            crate::zugferd_2_3_2::Enum1153::ReferenceNumberARequestForMeteredReading => Ok(Enum1153::ReferenceNumberARequestForMeteredReading),
            crate::zugferd_2_3_2::Enum1153::HasteningNumber => Ok(Enum1153::HasteningNumber),
            crate::zugferd_2_3_2::Enum1153::RepairDataRequestNumber => Ok(Enum1153::RepairDataRequestNumber),
            crate::zugferd_2_3_2::Enum1153::ConsumptionDataRequestNumber => Ok(Enum1153::ConsumptionDataRequestNumber),
            crate::zugferd_2_3_2::Enum1153::ProfileNumber => Ok(Enum1153::ProfileNumber),
            crate::zugferd_2_3_2::Enum1153::CaseNumber => Ok(Enum1153::CaseNumber),
            crate::zugferd_2_3_2::Enum1153::GovernmentQualityAssuranceAndControlLevelNumber => Ok(Enum1153::GovernmentQualityAssuranceAndControlLevelNumber),
            crate::zugferd_2_3_2::Enum1153::PaymentPlanReference => Ok(Enum1153::PaymentPlanReference),
            crate::zugferd_2_3_2::Enum1153::ReplacedMeterUnitNumber => Ok(Enum1153::ReplacedMeterUnitNumber),
            crate::zugferd_2_3_2::Enum1153::ReplenishmentPurchaseOrderRangeEndNumber => Ok(Enum1153::ReplenishmentPurchaseOrderRangeEndNumber),
            crate::zugferd_2_3_2::Enum1153::InsurerAssignedReferenceNumber => Ok(Enum1153::InsurerAssignedReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CanadianExciseEntryNumber => Ok(Enum1153::CanadianExciseEntryNumber),
            crate::zugferd_2_3_2::Enum1153::PremiumRateTable => Ok(Enum1153::PremiumRateTable),
            crate::zugferd_2_3_2::Enum1153::AdviseThroughBanksReference => Ok(Enum1153::AdviseThroughBanksReference),
            crate::zugferd_2_3_2::Enum1153::UsDepartmentTransportationBondSuretyCode => Ok(Enum1153::UsDepartmentTransportationBondSuretyCode),
            crate::zugferd_2_3_2::Enum1153::UsFoodAndDrugAdministrationEstablishmentIndicator => Ok(Enum1153::UsFoodAndDrugAdministrationEstablishmentIndicator),
            crate::zugferd_2_3_2::Enum1153::UsFederalCommunicationsCommissionFccImport => Ok(Enum1153::UsFederalCommunicationsCommissionFccImport),
            crate::zugferd_2_3_2::Enum1153::GoodsAndServicesTaxIdentificationNumber => Ok(Enum1153::GoodsAndServicesTaxIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::IntegratedLogisticSupportCrossReferenceNumber => Ok(Enum1153::IntegratedLogisticSupportCrossReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::DepartmentNumber => Ok(Enum1153::DepartmentNumber),
            crate::zugferd_2_3_2::Enum1153::BuyersCatalogueNumber => Ok(Enum1153::BuyersCatalogueNumber),
            crate::zugferd_2_3_2::Enum1153::FinancialSettlementPartysReferenceNumber => Ok(Enum1153::FinancialSettlementPartysReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::StandardsVersionNumber => Ok(Enum1153::StandardsVersionNumber),
            crate::zugferd_2_3_2::Enum1153::PipelineNumber => Ok(Enum1153::PipelineNumber),
            crate::zugferd_2_3_2::Enum1153::AccountServicingBanksReferenceNumber => Ok(Enum1153::AccountServicingBanksReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CompletedUnitsPaymentRequestReference => Ok(Enum1153::CompletedUnitsPaymentRequestReference),
            crate::zugferd_2_3_2::Enum1153::PaymentInAdvanceRequestReference => Ok(Enum1153::PaymentInAdvanceRequestReference),
            crate::zugferd_2_3_2::Enum1153::ParentFile => Ok(Enum1153::ParentFile),
            crate::zugferd_2_3_2::Enum1153::SubFile => Ok(Enum1153::SubFile),
            crate::zugferd_2_3_2::Enum1153::CadFileLayerConvention => Ok(Enum1153::CadFileLayerConvention),
            crate::zugferd_2_3_2::Enum1153::TechnicalRegulation => Ok(Enum1153::TechnicalRegulation),
            crate::zugferd_2_3_2::Enum1153::PlotFile => Ok(Enum1153::PlotFile),
            crate::zugferd_2_3_2::Enum1153::FileConversionJournal => Ok(Enum1153::FileConversionJournal),
            crate::zugferd_2_3_2::Enum1153::AuthorizationNumber => Ok(Enum1153::AuthorizationNumber),
            crate::zugferd_2_3_2::Enum1153::ReferenceNumberAssignedByThirdParty => Ok(Enum1153::ReferenceNumberAssignedByThirdParty),
            crate::zugferd_2_3_2::Enum1153::DepositReferenceNumber => Ok(Enum1153::DepositReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::NamedBanksReference => Ok(Enum1153::NamedBanksReference),
            crate::zugferd_2_3_2::Enum1153::DraweesReference => Ok(Enum1153::DraweesReference),
            crate::zugferd_2_3_2::Enum1153::CaseNeedPartysReference => Ok(Enum1153::CaseNeedPartysReference),
            crate::zugferd_2_3_2::Enum1153::CollectingBanksReference => Ok(Enum1153::CollectingBanksReference),
            crate::zugferd_2_3_2::Enum1153::RemittingBanksReference => Ok(Enum1153::RemittingBanksReference),
            crate::zugferd_2_3_2::Enum1153::PrincipalsBankReference => Ok(Enum1153::PrincipalsBankReference),
            crate::zugferd_2_3_2::Enum1153::PresentingBanksReference => Ok(Enum1153::PresentingBanksReference),
            crate::zugferd_2_3_2::Enum1153::ConsigneesReference => Ok(Enum1153::ConsigneesReference),
            crate::zugferd_2_3_2::Enum1153::FinancialTransactionReferenceNumber => Ok(Enum1153::FinancialTransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CreditReferenceNumber => Ok(Enum1153::CreditReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ReceivingBanksAuthorizationNumber => Ok(Enum1153::ReceivingBanksAuthorizationNumber),
            crate::zugferd_2_3_2::Enum1153::ClearingReference => Ok(Enum1153::ClearingReference),
            crate::zugferd_2_3_2::Enum1153::SendingBanksReferenceNumber => Ok(Enum1153::SendingBanksReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::DocumentaryPaymentReference => Ok(Enum1153::DocumentaryPaymentReference),
            crate::zugferd_2_3_2::Enum1153::AccountingFileReference => Ok(Enum1153::AccountingFileReference),
            crate::zugferd_2_3_2::Enum1153::SendersFileReferenceNumber => Ok(Enum1153::SendersFileReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ReceiversFileReferenceNumber => Ok(Enum1153::ReceiversFileReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::SourceDocumentInternalReference => Ok(Enum1153::SourceDocumentInternalReference),
            crate::zugferd_2_3_2::Enum1153::PrincipalsReference => Ok(Enum1153::PrincipalsReference),
            crate::zugferd_2_3_2::Enum1153::DebitReferenceNumber => Ok(Enum1153::DebitReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::Calendar => Ok(Enum1153::Calendar),
            crate::zugferd_2_3_2::Enum1153::WorkShift => Ok(Enum1153::WorkShift),
            crate::zugferd_2_3_2::Enum1153::WorkBreakdownStructure => Ok(Enum1153::WorkBreakdownStructure),
            crate::zugferd_2_3_2::Enum1153::OrganisationBreakdownStructure => Ok(Enum1153::OrganisationBreakdownStructure),
            crate::zugferd_2_3_2::Enum1153::WorkTaskChargeNumber => Ok(Enum1153::WorkTaskChargeNumber),
            crate::zugferd_2_3_2::Enum1153::FunctionalWorkGroup => Ok(Enum1153::FunctionalWorkGroup),
            crate::zugferd_2_3_2::Enum1153::WorkTeam => Ok(Enum1153::WorkTeam),
            crate::zugferd_2_3_2::Enum1153::Department => Ok(Enum1153::Department),
            crate::zugferd_2_3_2::Enum1153::StatementWork => Ok(Enum1153::StatementWork),
            crate::zugferd_2_3_2::Enum1153::WorkPackage => Ok(Enum1153::WorkPackage),
            crate::zugferd_2_3_2::Enum1153::PlanningPackage => Ok(Enum1153::PlanningPackage),
            crate::zugferd_2_3_2::Enum1153::CostAccount => Ok(Enum1153::CostAccount),
            crate::zugferd_2_3_2::Enum1153::WorkOrder => Ok(Enum1153::WorkOrder),
            crate::zugferd_2_3_2::Enum1153::TransportationControlNumberTcn => Ok(Enum1153::TransportationControlNumberTcn),
            crate::zugferd_2_3_2::Enum1153::ConstraintNotation => Ok(Enum1153::ConstraintNotation),
            crate::zugferd_2_3_2::Enum1153::EtermsReference => Ok(Enum1153::EtermsReference),
            crate::zugferd_2_3_2::Enum1153::ImplementationVersionNumber => Ok(Enum1153::ImplementationVersionNumber),
            crate::zugferd_2_3_2::Enum1153::AccountsReceivableNumber => Ok(Enum1153::AccountsReceivableNumber),
            crate::zugferd_2_3_2::Enum1153::IncorporatedLegalReference => Ok(Enum1153::IncorporatedLegalReference),
            crate::zugferd_2_3_2::Enum1153::PaymentInstalmentReferenceNumber => Ok(Enum1153::PaymentInstalmentReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::EquipmentOwnerReferenceNumber => Ok(Enum1153::EquipmentOwnerReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CedentsClaimNumber => Ok(Enum1153::CedentsClaimNumber),
            crate::zugferd_2_3_2::Enum1153::ReinsurersClaimNumber => Ok(Enum1153::ReinsurersClaimNumber),
            crate::zugferd_2_3_2::Enum1153::PriceSalesCatalogueResponseReferenceNumber => Ok(Enum1153::PriceSalesCatalogueResponseReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::GeneralPurposeMessageReferenceNumber => Ok(Enum1153::GeneralPurposeMessageReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::InvoicingDataSheetReferenceNumber => Ok(Enum1153::InvoicingDataSheetReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::InventoryReportReferenceNumber => Ok(Enum1153::InventoryReportReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CeilingFormulaReferenceNumber => Ok(Enum1153::CeilingFormulaReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::PriceVariationFormulaReferenceNumber => Ok(Enum1153::PriceVariationFormulaReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ReferenceToAccountServicingBanksMessage => Ok(Enum1153::ReferenceToAccountServicingBanksMessage),
            crate::zugferd_2_3_2::Enum1153::PartySequenceNumber => Ok(Enum1153::PartySequenceNumber),
            crate::zugferd_2_3_2::Enum1153::PurchasersRequestReference => Ok(Enum1153::PurchasersRequestReference),
            crate::zugferd_2_3_2::Enum1153::ContractorRequestReference => Ok(Enum1153::ContractorRequestReference),
            crate::zugferd_2_3_2::Enum1153::AccidentReferenceNumber => Ok(Enum1153::AccidentReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::CommercialAccountSummaryReferenceNumber => Ok(Enum1153::CommercialAccountSummaryReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ContractBreakdownReference => Ok(Enum1153::ContractBreakdownReference),
            crate::zugferd_2_3_2::Enum1153::ContractorRegistrationNumber => Ok(Enum1153::ContractorRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::ApplicableCoefficientIdentificationNumber => Ok(Enum1153::ApplicableCoefficientIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::SpecialBudgetAccountNumber => Ok(Enum1153::SpecialBudgetAccountNumber),
            crate::zugferd_2_3_2::Enum1153::AuthorisationForRepairReference => Ok(Enum1153::AuthorisationForRepairReference),
            crate::zugferd_2_3_2::Enum1153::ManufacturerDefinedRepairRatesReference => Ok(Enum1153::ManufacturerDefinedRepairRatesReference),
            crate::zugferd_2_3_2::Enum1153::OriginalSubmitterLogNumber => Ok(Enum1153::OriginalSubmitterLogNumber),
            crate::zugferd_2_3_2::Enum1153::OriginalSubmitterParentDataMaintenanceRequestDmr => Ok(Enum1153::OriginalSubmitterParentDataMaintenanceRequestDmr),
            crate::zugferd_2_3_2::Enum1153::OriginalSubmitterChildDataMaintenanceRequestDmr => Ok(Enum1153::OriginalSubmitterChildDataMaintenanceRequestDmr),
            crate::zugferd_2_3_2::Enum1153::EntryPointAssessmentLogNumber => Ok(Enum1153::EntryPointAssessmentLogNumber),
            crate::zugferd_2_3_2::Enum1153::EntryPointAssessmentLogNumberParentDmr => Ok(Enum1153::EntryPointAssessmentLogNumberParentDmr),
            crate::zugferd_2_3_2::Enum1153::EntryPointAssessmentLogNumberChildDmr => Ok(Enum1153::EntryPointAssessmentLogNumberChildDmr),
            crate::zugferd_2_3_2::Enum1153::DataStructureTag => Ok(Enum1153::DataStructureTag),
            crate::zugferd_2_3_2::Enum1153::CentralSecretariatLogNumber => Ok(Enum1153::CentralSecretariatLogNumber),
            crate::zugferd_2_3_2::Enum1153::CentralSecretariatLogNumberParentDataMaintenance => Ok(Enum1153::CentralSecretariatLogNumberParentDataMaintenance),
            crate::zugferd_2_3_2::Enum1153::CentralSecretariatLogNumberChildDataMaintenance => Ok(Enum1153::CentralSecretariatLogNumberChildDataMaintenance),
            crate::zugferd_2_3_2::Enum1153::InternationalAssessmentLogNumber => Ok(Enum1153::InternationalAssessmentLogNumber),
            crate::zugferd_2_3_2::Enum1153::InternationalAssessmentLogNumberParentData => Ok(Enum1153::InternationalAssessmentLogNumberParentData),
            crate::zugferd_2_3_2::Enum1153::InternationalAssessmentLogNumberChildDataMaintenance => Ok(Enum1153::InternationalAssessmentLogNumberChildDataMaintenance),
            crate::zugferd_2_3_2::Enum1153::StatusReportNumber => Ok(Enum1153::StatusReportNumber),
            crate::zugferd_2_3_2::Enum1153::MessageDesignGroupNumber => Ok(Enum1153::MessageDesignGroupNumber),
            crate::zugferd_2_3_2::Enum1153::UsCustomsServiceUscsEntryCode => Ok(Enum1153::UsCustomsServiceUscsEntryCode),
            crate::zugferd_2_3_2::Enum1153::BeginningJobSequenceNumber => Ok(Enum1153::BeginningJobSequenceNumber),
            crate::zugferd_2_3_2::Enum1153::SendersClauseNumber => Ok(Enum1153::SendersClauseNumber),
            crate::zugferd_2_3_2::Enum1153::DunAndBradstreetCanadas8DigitStandardIndustrial => Ok(Enum1153::DunAndBradstreetCanadas8DigitStandardIndustrial),
            crate::zugferd_2_3_2::Enum1153::ActivitePrincipaleExerceeApeIdentifier => Ok(Enum1153::ActivitePrincipaleExerceeApeIdentifier),
            crate::zugferd_2_3_2::Enum1153::DunAndBradstreetUs8DigitStandardIndustrial => Ok(Enum1153::DunAndBradstreetUs8DigitStandardIndustrial),
            crate::zugferd_2_3_2::Enum1153::NomenclatureActivityClassificationEconomyNace => Ok(Enum1153::NomenclatureActivityClassificationEconomyNace),
            crate::zugferd_2_3_2::Enum1153::NormeActiviteFrancaiseNafIdentifier => Ok(Enum1153::NormeActiviteFrancaiseNafIdentifier),
            crate::zugferd_2_3_2::Enum1153::RegisteredContractorActivityType => Ok(Enum1153::RegisteredContractorActivityType),
            crate::zugferd_2_3_2::Enum1153::StatisticBundesAmtSbaIdentifier => Ok(Enum1153::StatisticBundesAmtSbaIdentifier),
            crate::zugferd_2_3_2::Enum1153::StateOrProvinceAssignedEntityIdentification => Ok(Enum1153::StateOrProvinceAssignedEntityIdentification),
            crate::zugferd_2_3_2::Enum1153::InstituteSecurityAndFutureMarketDevelopmentIsfmd => Ok(Enum1153::InstituteSecurityAndFutureMarketDevelopmentIsfmd),
            crate::zugferd_2_3_2::Enum1153::FileIdentificationNumber => Ok(Enum1153::FileIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::BankruptcyProcedureNumber => Ok(Enum1153::BankruptcyProcedureNumber),
            crate::zugferd_2_3_2::Enum1153::NationalGovernmentBusinessIdentificationNumber => Ok(Enum1153::NationalGovernmentBusinessIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::PriorDataUniversalNumberSystemDunsNumber => Ok(Enum1153::PriorDataUniversalNumberSystemDunsNumber),
            crate::zugferd_2_3_2::Enum1153::CompaniesRegistryOfficeCroNumber => Ok(Enum1153::CompaniesRegistryOfficeCroNumber),
            crate::zugferd_2_3_2::Enum1153::CostaRicanJudicialNumber => Ok(Enum1153::CostaRicanJudicialNumber),
            crate::zugferd_2_3_2::Enum1153::NumeroDeIdentificacionTributariaNit => Ok(Enum1153::NumeroDeIdentificacionTributariaNit),
            crate::zugferd_2_3_2::Enum1153::PatronNumber => Ok(Enum1153::PatronNumber),
            crate::zugferd_2_3_2::Enum1153::RegistroInformacionFiscalRifNumber => Ok(Enum1153::RegistroInformacionFiscalRifNumber),
            crate::zugferd_2_3_2::Enum1153::RegistroUnicoDeContribuyenteRucNumber => Ok(Enum1153::RegistroUnicoDeContribuyenteRucNumber),
            crate::zugferd_2_3_2::Enum1153::TokyoShokoResearchTsrBusinessIdentifier => Ok(Enum1153::TokyoShokoResearchTsrBusinessIdentifier),
            crate::zugferd_2_3_2::Enum1153::PersonalIdentityCardNumber => Ok(Enum1153::PersonalIdentityCardNumber),
            crate::zugferd_2_3_2::Enum1153::SystemeInformatiquePourLeRepertoireDesEntreprises => Ok(Enum1153::SystemeInformatiquePourLeRepertoireDesEntreprises),
            crate::zugferd_2_3_2::Enum1153::SystemeInformatiquePourLeRepertoireDesEtablissements => Ok(Enum1153::SystemeInformatiquePourLeRepertoireDesEtablissements),
            crate::zugferd_2_3_2::Enum1153::PublicationIssueNumber => Ok(Enum1153::PublicationIssueNumber),
            crate::zugferd_2_3_2::Enum1153::OriginalFilingNumber => Ok(Enum1153::OriginalFilingNumber),
            crate::zugferd_2_3_2::Enum1153::DocumentPageIdentifier => Ok(Enum1153::DocumentPageIdentifier),
            crate::zugferd_2_3_2::Enum1153::PublicFilingRegistrationNumber => Ok(Enum1153::PublicFilingRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::RegiristoFederalDeContribuyentes => Ok(Enum1153::RegiristoFederalDeContribuyentes),
            crate::zugferd_2_3_2::Enum1153::SocialSecurityNumber => Ok(Enum1153::SocialSecurityNumber),
            crate::zugferd_2_3_2::Enum1153::DocumentVolumeNumber => Ok(Enum1153::DocumentVolumeNumber),
            crate::zugferd_2_3_2::Enum1153::BookNumber => Ok(Enum1153::BookNumber),
            crate::zugferd_2_3_2::Enum1153::StockExchangeCompanyIdentifier => Ok(Enum1153::StockExchangeCompanyIdentifier),
            crate::zugferd_2_3_2::Enum1153::ImputationAccount => Ok(Enum1153::ImputationAccount),
            crate::zugferd_2_3_2::Enum1153::FinancialPhaseReference => Ok(Enum1153::FinancialPhaseReference),
            crate::zugferd_2_3_2::Enum1153::TechnicalPhaseReference => Ok(Enum1153::TechnicalPhaseReference),
            crate::zugferd_2_3_2::Enum1153::PriorContractorRegistrationNumber => Ok(Enum1153::PriorContractorRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::StockAdjustmentNumber => Ok(Enum1153::StockAdjustmentNumber),
            crate::zugferd_2_3_2::Enum1153::DispensationReference => Ok(Enum1153::DispensationReference),
            crate::zugferd_2_3_2::Enum1153::InvestmentReferenceNumber => Ok(Enum1153::InvestmentReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::AssumingCompany => Ok(Enum1153::AssumingCompany),
            crate::zugferd_2_3_2::Enum1153::BudgetChapter => Ok(Enum1153::BudgetChapter),
            crate::zugferd_2_3_2::Enum1153::DutyFreeProductsSecurityNumber => Ok(Enum1153::DutyFreeProductsSecurityNumber),
            crate::zugferd_2_3_2::Enum1153::DutyFreeProductsReceiptAuthorisationNumber => Ok(Enum1153::DutyFreeProductsReceiptAuthorisationNumber),
            crate::zugferd_2_3_2::Enum1153::PartyInformationMessageReference => Ok(Enum1153::PartyInformationMessageReference),
            crate::zugferd_2_3_2::Enum1153::FormalStatementReference => Ok(Enum1153::FormalStatementReference),
            crate::zugferd_2_3_2::Enum1153::ProofDeliveryReferenceNumber => Ok(Enum1153::ProofDeliveryReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::SuppliersCreditClaimReferenceNumber => Ok(Enum1153::SuppliersCreditClaimReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::PictureActualProduct => Ok(Enum1153::PictureActualProduct),
            crate::zugferd_2_3_2::Enum1153::PictureAGenericProduct => Ok(Enum1153::PictureAGenericProduct),
            crate::zugferd_2_3_2::Enum1153::TradingPartnerIdentificationNumber => Ok(Enum1153::TradingPartnerIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::PriorTradingPartnerIdentificationNumber => Ok(Enum1153::PriorTradingPartnerIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::Password => Ok(Enum1153::Password),
            crate::zugferd_2_3_2::Enum1153::FormalReportNumber => Ok(Enum1153::FormalReportNumber),
            crate::zugferd_2_3_2::Enum1153::FundAccountNumber => Ok(Enum1153::FundAccountNumber),
            crate::zugferd_2_3_2::Enum1153::SafeCustodyNumber => Ok(Enum1153::SafeCustodyNumber),
            crate::zugferd_2_3_2::Enum1153::MasterAccountNumber => Ok(Enum1153::MasterAccountNumber),
            crate::zugferd_2_3_2::Enum1153::GroupReferenceNumber => Ok(Enum1153::GroupReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::AccountingTransmissionNumber => Ok(Enum1153::AccountingTransmissionNumber),
            crate::zugferd_2_3_2::Enum1153::ProductDataFileNumber => Ok(Enum1153::ProductDataFileNumber),
            crate::zugferd_2_3_2::Enum1153::CadastroGeralDoContribuinteCgc => Ok(Enum1153::CadastroGeralDoContribuinteCgc),
            crate::zugferd_2_3_2::Enum1153::ForeignResidentIdentificationNumber => Ok(Enum1153::ForeignResidentIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::CdRom => Ok(Enum1153::CdRom),
            crate::zugferd_2_3_2::Enum1153::PhysicalMedium => Ok(Enum1153::PhysicalMedium),
            crate::zugferd_2_3_2::Enum1153::FinancialCancellationReferenceNumber => Ok(Enum1153::FinancialCancellationReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::PurchaseForExportCustomsAgreementNumber => Ok(Enum1153::PurchaseForExportCustomsAgreementNumber),
            crate::zugferd_2_3_2::Enum1153::JudgmentNumber => Ok(Enum1153::JudgmentNumber),
            crate::zugferd_2_3_2::Enum1153::SecretariatNumber => Ok(Enum1153::SecretariatNumber),
            crate::zugferd_2_3_2::Enum1153::PreviousBankingStatusMessageReference => Ok(Enum1153::PreviousBankingStatusMessageReference),
            crate::zugferd_2_3_2::Enum1153::LastReceivedBankingStatusMessageReference => Ok(Enum1153::LastReceivedBankingStatusMessageReference),
            crate::zugferd_2_3_2::Enum1153::BanksDocumentaryProcedureReference => Ok(Enum1153::BanksDocumentaryProcedureReference),
            crate::zugferd_2_3_2::Enum1153::CustomersDocumentaryProcedureReference => Ok(Enum1153::CustomersDocumentaryProcedureReference),
            crate::zugferd_2_3_2::Enum1153::SafeDepositBoxNumber => Ok(Enum1153::SafeDepositBoxNumber),
            crate::zugferd_2_3_2::Enum1153::ReceivingBankgiroNumber => Ok(Enum1153::ReceivingBankgiroNumber),
            crate::zugferd_2_3_2::Enum1153::SendingBankgiroNumber => Ok(Enum1153::SendingBankgiroNumber),
            crate::zugferd_2_3_2::Enum1153::BankgiroReference => Ok(Enum1153::BankgiroReference),
            crate::zugferd_2_3_2::Enum1153::GuaranteeNumber => Ok(Enum1153::GuaranteeNumber),
            crate::zugferd_2_3_2::Enum1153::CollectionInstrumentNumber => Ok(Enum1153::CollectionInstrumentNumber),
            crate::zugferd_2_3_2::Enum1153::ConvertedPostgiroNumber => Ok(Enum1153::ConvertedPostgiroNumber),
            crate::zugferd_2_3_2::Enum1153::CostCentreAlignmentNumber => Ok(Enum1153::CostCentreAlignmentNumber),
            crate::zugferd_2_3_2::Enum1153::KamerVanKoophandelKvkNumber => Ok(Enum1153::KamerVanKoophandelKvkNumber),
            crate::zugferd_2_3_2::Enum1153::InstitutBelgoLuxembourgeoisDeCodificationIblcNumber => Ok(Enum1153::InstitutBelgoLuxembourgeoisDeCodificationIblcNumber),
            crate::zugferd_2_3_2::Enum1153::ExternalObjectReference => Ok(Enum1153::ExternalObjectReference),
            crate::zugferd_2_3_2::Enum1153::ExceptionalTransportAuthorisationNumber => Ok(Enum1153::ExceptionalTransportAuthorisationNumber),
            crate::zugferd_2_3_2::Enum1153::ClaveUnicaDeIdentificacionTributariaCuit => Ok(Enum1153::ClaveUnicaDeIdentificacionTributariaCuit),
            crate::zugferd_2_3_2::Enum1153::RegistroUnicoTributarioRut => Ok(Enum1153::RegistroUnicoTributarioRut),
            crate::zugferd_2_3_2::Enum1153::FlatRackContainerBundleIdentificationNumber => Ok(Enum1153::FlatRackContainerBundleIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentAcceptanceOrderReference => Ok(Enum1153::TransportEquipmentAcceptanceOrderReference),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentReleaseOrderReference => Ok(Enum1153::TransportEquipmentReleaseOrderReference),
            crate::zugferd_2_3_2::Enum1153::ShipsStayReferenceNumber => Ok(Enum1153::ShipsStayReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::AuthorizationToMeetCompetitionNumber => Ok(Enum1153::AuthorizationToMeetCompetitionNumber),
            crate::zugferd_2_3_2::Enum1153::PlacePositioningReference => Ok(Enum1153::PlacePositioningReference),
            crate::zugferd_2_3_2::Enum1153::PartyReference => Ok(Enum1153::PartyReference),
            crate::zugferd_2_3_2::Enum1153::IssuedPrescriptionIdentification => Ok(Enum1153::IssuedPrescriptionIdentification),
            crate::zugferd_2_3_2::Enum1153::CollectionReference => Ok(Enum1153::CollectionReference),
            crate::zugferd_2_3_2::Enum1153::TravelService => Ok(Enum1153::TravelService),
            crate::zugferd_2_3_2::Enum1153::ConsignmentStockContract => Ok(Enum1153::ConsignmentStockContract),
            crate::zugferd_2_3_2::Enum1153::ImportersLetterCreditReference => Ok(Enum1153::ImportersLetterCreditReference),
            crate::zugferd_2_3_2::Enum1153::PerformedPrescriptionIdentification => Ok(Enum1153::PerformedPrescriptionIdentification),
            crate::zugferd_2_3_2::Enum1153::ImageReference => Ok(Enum1153::ImageReference),
            crate::zugferd_2_3_2::Enum1153::ProposedPurchaseOrderReferenceNumber => Ok(Enum1153::ProposedPurchaseOrderReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ApplicationForFinancialSupportReferenceNumber => Ok(Enum1153::ApplicationForFinancialSupportReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ManufacturingQualityAgreementNumber => Ok(Enum1153::ManufacturingQualityAgreementNumber),
            crate::zugferd_2_3_2::Enum1153::SoftwareEditorReference => Ok(Enum1153::SoftwareEditorReference),
            crate::zugferd_2_3_2::Enum1153::SoftwareReference => Ok(Enum1153::SoftwareReference),
            crate::zugferd_2_3_2::Enum1153::SoftwareQualityReference => Ok(Enum1153::SoftwareQualityReference),
            crate::zugferd_2_3_2::Enum1153::ConsolidatedOrdersReference => Ok(Enum1153::ConsolidatedOrdersReference),
            crate::zugferd_2_3_2::Enum1153::CustomsBindingRulingNumber => Ok(Enum1153::CustomsBindingRulingNumber),
            crate::zugferd_2_3_2::Enum1153::CustomsNonBindingRulingNumber => Ok(Enum1153::CustomsNonBindingRulingNumber),
            crate::zugferd_2_3_2::Enum1153::DeliveryRouteReference => Ok(Enum1153::DeliveryRouteReference),
            crate::zugferd_2_3_2::Enum1153::NetAreaSupplierReference => Ok(Enum1153::NetAreaSupplierReference),
            crate::zugferd_2_3_2::Enum1153::TimeSeriesReference => Ok(Enum1153::TimeSeriesReference),
            crate::zugferd_2_3_2::Enum1153::ConnectingPointToCentralGrid => Ok(Enum1153::ConnectingPointToCentralGrid),
            crate::zugferd_2_3_2::Enum1153::MarketingPlanIdentificationNumberMpin => Ok(Enum1153::MarketingPlanIdentificationNumberMpin),
            crate::zugferd_2_3_2::Enum1153::EntityReferenceNumberPrevious => Ok(Enum1153::EntityReferenceNumberPrevious),
            crate::zugferd_2_3_2::Enum1153::InternationalStandardIndustrialClassificationIsic => Ok(Enum1153::InternationalStandardIndustrialClassificationIsic),
            crate::zugferd_2_3_2::Enum1153::CustomsPreApprovalRulingNumber => Ok(Enum1153::CustomsPreApprovalRulingNumber),
            crate::zugferd_2_3_2::Enum1153::AccountPayableNumber => Ok(Enum1153::AccountPayableNumber),
            crate::zugferd_2_3_2::Enum1153::FirstFinancialInstitutionsTransactionReference => Ok(Enum1153::FirstFinancialInstitutionsTransactionReference),
            crate::zugferd_2_3_2::Enum1153::ProductCharacteristicsDirectory => Ok(Enum1153::ProductCharacteristicsDirectory),
            crate::zugferd_2_3_2::Enum1153::SuppliersCustomerReferenceNumber => Ok(Enum1153::SuppliersCustomerReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::InventoryReportRequestNumber => Ok(Enum1153::InventoryReportRequestNumber),
            crate::zugferd_2_3_2::Enum1153::MeteringPoint => Ok(Enum1153::MeteringPoint),
            crate::zugferd_2_3_2::Enum1153::PassengerReservationNumber => Ok(Enum1153::PassengerReservationNumber),
            crate::zugferd_2_3_2::Enum1153::SlaughterhouseApprovalNumber => Ok(Enum1153::SlaughterhouseApprovalNumber),
            crate::zugferd_2_3_2::Enum1153::MeatCuttingPlantApprovalNumber => Ok(Enum1153::MeatCuttingPlantApprovalNumber),
            crate::zugferd_2_3_2::Enum1153::CustomerTravelServiceIdentifier => Ok(Enum1153::CustomerTravelServiceIdentifier),
            crate::zugferd_2_3_2::Enum1153::ExportControlClassificationNumber => Ok(Enum1153::ExportControlClassificationNumber),
            crate::zugferd_2_3_2::Enum1153::BrokerReference3 => Ok(Enum1153::BrokerReference3),
            crate::zugferd_2_3_2::Enum1153::ConsignmentInformation => Ok(Enum1153::ConsignmentInformation),
            crate::zugferd_2_3_2::Enum1153::GoodsItemInformation => Ok(Enum1153::GoodsItemInformation),
            crate::zugferd_2_3_2::Enum1153::DangerousGoodsInformation => Ok(Enum1153::DangerousGoodsInformation),
            crate::zugferd_2_3_2::Enum1153::PilotageServicesExemptionNumber => Ok(Enum1153::PilotageServicesExemptionNumber),
            crate::zugferd_2_3_2::Enum1153::PersonRegistrationNumber => Ok(Enum1153::PersonRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::PlacePackingApprovalNumber => Ok(Enum1153::PlacePackingApprovalNumber),
            crate::zugferd_2_3_2::Enum1153::OriginalMandateReference => Ok(Enum1153::OriginalMandateReference),
            crate::zugferd_2_3_2::Enum1153::MandateReference => Ok(Enum1153::MandateReference),
            crate::zugferd_2_3_2::Enum1153::ReservationStationIndentifier => Ok(Enum1153::ReservationStationIndentifier),
            crate::zugferd_2_3_2::Enum1153::UniqueGoodsShipmentIdentifier => Ok(Enum1153::UniqueGoodsShipmentIdentifier),
            crate::zugferd_2_3_2::Enum1153::FrameworkAgreementNumber => Ok(Enum1153::FrameworkAgreementNumber),
            crate::zugferd_2_3_2::Enum1153::HashValue => Ok(Enum1153::HashValue),
            crate::zugferd_2_3_2::Enum1153::MovementReferenceNumber => Ok(Enum1153::MovementReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::EconomicOperatorsRegistrationAndIdentificationNumber => Ok(Enum1153::EconomicOperatorsRegistrationAndIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::LocalReferenceNumber => Ok(Enum1153::LocalReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::RateCodeNumber => Ok(Enum1153::RateCodeNumber),
            crate::zugferd_2_3_2::Enum1153::AirWaybillNumber => Ok(Enum1153::AirWaybillNumber),
            crate::zugferd_2_3_2::Enum1153::DocumentaryCreditAmendmentNumber => Ok(Enum1153::DocumentaryCreditAmendmentNumber),
            crate::zugferd_2_3_2::Enum1153::AdvisingBanksReference => Ok(Enum1153::AdvisingBanksReference),
            crate::zugferd_2_3_2::Enum1153::CostCentre => Ok(Enum1153::CostCentre),
            crate::zugferd_2_3_2::Enum1153::WorkItemQuantityDetermination => Ok(Enum1153::WorkItemQuantityDetermination),
            crate::zugferd_2_3_2::Enum1153::InternalDataProcessNumber => Ok(Enum1153::InternalDataProcessNumber),
            crate::zugferd_2_3_2::Enum1153::CategoryWorkReference => Ok(Enum1153::CategoryWorkReference),
            crate::zugferd_2_3_2::Enum1153::PolicyFormNumber => Ok(Enum1153::PolicyFormNumber),
            crate::zugferd_2_3_2::Enum1153::NetArea => Ok(Enum1153::NetArea),
            crate::zugferd_2_3_2::Enum1153::ServiceProvider => Ok(Enum1153::ServiceProvider),
            crate::zugferd_2_3_2::Enum1153::ErrorPosition => Ok(Enum1153::ErrorPosition),
            crate::zugferd_2_3_2::Enum1153::ServiceCategoryReference => Ok(Enum1153::ServiceCategoryReference),
            crate::zugferd_2_3_2::Enum1153::ConnectedLocation => Ok(Enum1153::ConnectedLocation),
            crate::zugferd_2_3_2::Enum1153::RelatedParty => Ok(Enum1153::RelatedParty),
            crate::zugferd_2_3_2::Enum1153::LatestAccountingEntryRecordReference => Ok(Enum1153::LatestAccountingEntryRecordReference),
            crate::zugferd_2_3_2::Enum1153::AccountingEntry => Ok(Enum1153::AccountingEntry),
            crate::zugferd_2_3_2::Enum1153::DocumentReferenceOriginal => Ok(Enum1153::DocumentReferenceOriginal),
            crate::zugferd_2_3_2::Enum1153::HygienicCertificateNumberNational => Ok(Enum1153::HygienicCertificateNumberNational),
            crate::zugferd_2_3_2::Enum1153::AdministrativeReferenceCode => Ok(Enum1153::AdministrativeReferenceCode),
            crate::zugferd_2_3_2::Enum1153::PickUpSheetNumber => Ok(Enum1153::PickUpSheetNumber),
            crate::zugferd_2_3_2::Enum1153::PhoneNumber => Ok(Enum1153::PhoneNumber),
            crate::zugferd_2_3_2::Enum1153::BuyersFundNumber => Ok(Enum1153::BuyersFundNumber),
            crate::zugferd_2_3_2::Enum1153::CompanyTradingAccountNumber => Ok(Enum1153::CompanyTradingAccountNumber),
            crate::zugferd_2_3_2::Enum1153::ReservedGoodsIdentifier => Ok(Enum1153::ReservedGoodsIdentifier),
            crate::zugferd_2_3_2::Enum1153::HandlingAndMovementReferenceNumber => Ok(Enum1153::HandlingAndMovementReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::InstructionToDespatchReferenceNumber => Ok(Enum1153::InstructionToDespatchReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::InstructionForReturnsNumber => Ok(Enum1153::InstructionForReturnsNumber),
            crate::zugferd_2_3_2::Enum1153::MeteredServicesConsumptionReportNumber => Ok(Enum1153::MeteredServicesConsumptionReportNumber),
            crate::zugferd_2_3_2::Enum1153::OrderStatusEnquiryNumber => Ok(Enum1153::OrderStatusEnquiryNumber),
            crate::zugferd_2_3_2::Enum1153::FirmBookingReferenceNumber => Ok(Enum1153::FirmBookingReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ProductInquiryNumber => Ok(Enum1153::ProductInquiryNumber),
            crate::zugferd_2_3_2::Enum1153::SplitDeliveryNumber => Ok(Enum1153::SplitDeliveryNumber),
            crate::zugferd_2_3_2::Enum1153::ServiceRelationNumber => Ok(Enum1153::ServiceRelationNumber),
            crate::zugferd_2_3_2::Enum1153::SerialShippingContainerCode => Ok(Enum1153::SerialShippingContainerCode),
            crate::zugferd_2_3_2::Enum1153::TestSpecificationNumber => Ok(Enum1153::TestSpecificationNumber),
            crate::zugferd_2_3_2::Enum1153::TransportStatusReportNumber => Ok(Enum1153::TransportStatusReportNumber),
            crate::zugferd_2_3_2::Enum1153::ToolingContractNumber => Ok(Enum1153::ToolingContractNumber),
            crate::zugferd_2_3_2::Enum1153::FormulaReferenceNumber => Ok(Enum1153::FormulaReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::PreAgreementNumber => Ok(Enum1153::PreAgreementNumber),
            crate::zugferd_2_3_2::Enum1153::ProductCertificationNumber => Ok(Enum1153::ProductCertificationNumber),
            crate::zugferd_2_3_2::Enum1153::ConsignmentContractNumber => Ok(Enum1153::ConsignmentContractNumber),
            crate::zugferd_2_3_2::Enum1153::ProductSpecificationReferenceNumber => Ok(Enum1153::ProductSpecificationReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::PayrollDeductionAdviceReference => Ok(Enum1153::PayrollDeductionAdviceReference),
            crate::zugferd_2_3_2::Enum1153::TracesPartyIdentification => Ok(Enum1153::TracesPartyIdentification),
            crate::zugferd_2_3_2::Enum1153::BeginningMeterReadingActual => Ok(Enum1153::BeginningMeterReadingActual),
            crate::zugferd_2_3_2::Enum1153::BuyersContractNumber => Ok(Enum1153::BuyersContractNumber),
            crate::zugferd_2_3_2::Enum1153::BidNumber => Ok(Enum1153::BidNumber),
            crate::zugferd_2_3_2::Enum1153::BeginningMeterReadingEstimated => Ok(Enum1153::BeginningMeterReadingEstimated),
            crate::zugferd_2_3_2::Enum1153::HouseBillLadingNumber => Ok(Enum1153::HouseBillLadingNumber),
            crate::zugferd_2_3_2::Enum1153::BillLadingNumber => Ok(Enum1153::BillLadingNumber),
            crate::zugferd_2_3_2::Enum1153::ConsignmentIdentifierCarrierAssigned => Ok(Enum1153::ConsignmentIdentifierCarrierAssigned),
            crate::zugferd_2_3_2::Enum1153::BlanketOrderNumber => Ok(Enum1153::BlanketOrderNumber),
            crate::zugferd_2_3_2::Enum1153::BrokerOrSalesOfficeNumber => Ok(Enum1153::BrokerOrSalesOfficeNumber),
            crate::zugferd_2_3_2::Enum1153::BatchNumberLotNumber => Ok(Enum1153::BatchNumberLotNumber),
            crate::zugferd_2_3_2::Enum1153::BatteryAndAccumulatorProducerRegistrationNumber => Ok(Enum1153::BatteryAndAccumulatorProducerRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::BlendedWithNumber => Ok(Enum1153::BlendedWithNumber),
            crate::zugferd_2_3_2::Enum1153::IataCargoAgentCassAddressNumber => Ok(Enum1153::IataCargoAgentCassAddressNumber),
            crate::zugferd_2_3_2::Enum1153::MatchingEntriesBalanced => Ok(Enum1153::MatchingEntriesBalanced),
            crate::zugferd_2_3_2::Enum1153::EntryFlagging => Ok(Enum1153::EntryFlagging),
            crate::zugferd_2_3_2::Enum1153::MatchingEntriesUnbalanced => Ok(Enum1153::MatchingEntriesUnbalanced),
            crate::zugferd_2_3_2::Enum1153::DocumentReferenceInternal => Ok(Enum1153::DocumentReferenceInternal),
            crate::zugferd_2_3_2::Enum1153::EuropeanValueAddedTaxIdentification => Ok(Enum1153::EuropeanValueAddedTaxIdentification),
            crate::zugferd_2_3_2::Enum1153::CostAccountingDocument => Ok(Enum1153::CostAccountingDocument),
            crate::zugferd_2_3_2::Enum1153::GridOperatorsCustomerReferenceNumber => Ok(Enum1153::GridOperatorsCustomerReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::TicketControlNumber => Ok(Enum1153::TicketControlNumber),
            crate::zugferd_2_3_2::Enum1153::OrderShipmentGroupingReference => Ok(Enum1153::OrderShipmentGroupingReference),
            crate::zugferd_2_3_2::Enum1153::CreditNoteNumber => Ok(Enum1153::CreditNoteNumber),
            crate::zugferd_2_3_2::Enum1153::CedingCompany => Ok(Enum1153::CedingCompany),
            crate::zugferd_2_3_2::Enum1153::DebitLetterNumber => Ok(Enum1153::DebitLetterNumber),
            crate::zugferd_2_3_2::Enum1153::ConsigneesFurtherOrder => Ok(Enum1153::ConsigneesFurtherOrder),
            crate::zugferd_2_3_2::Enum1153::AnimalFarmLicenceNumber => Ok(Enum1153::AnimalFarmLicenceNumber),
            crate::zugferd_2_3_2::Enum1153::ConsignorsFurtherOrder => Ok(Enum1153::ConsignorsFurtherOrder),
            crate::zugferd_2_3_2::Enum1153::ConsigneesOrderNumber => Ok(Enum1153::ConsigneesOrderNumber),
            crate::zugferd_2_3_2::Enum1153::CustomerCatalogueNumber => Ok(Enum1153::CustomerCatalogueNumber),
            crate::zugferd_2_3_2::Enum1153::ChequeNumber => Ok(Enum1153::ChequeNumber),
            crate::zugferd_2_3_2::Enum1153::CheckingNumber => Ok(Enum1153::CheckingNumber),
            crate::zugferd_2_3_2::Enum1153::CreditMemoNumber => Ok(Enum1153::CreditMemoNumber),
            crate::zugferd_2_3_2::Enum1153::RoadConsignmentNoteNumber => Ok(Enum1153::RoadConsignmentNoteNumber),
            crate::zugferd_2_3_2::Enum1153::CarriersReferenceNumber => Ok(Enum1153::CarriersReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ChargesNoteDocumentAttachmentIndicator => Ok(Enum1153::ChargesNoteDocumentAttachmentIndicator),
            crate::zugferd_2_3_2::Enum1153::CallOffOrderNumber => Ok(Enum1153::CallOffOrderNumber),
            crate::zugferd_2_3_2::Enum1153::ConditionPurchaseDocumentNumber => Ok(Enum1153::ConditionPurchaseDocumentNumber),
            crate::zugferd_2_3_2::Enum1153::CustomerReferenceNumber => Ok(Enum1153::CustomerReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::TransportMeansJourneyIdentifier => Ok(Enum1153::TransportMeansJourneyIdentifier),
            crate::zugferd_2_3_2::Enum1153::ConditionSaleDocumentNumber => Ok(Enum1153::ConditionSaleDocumentNumber),
            crate::zugferd_2_3_2::Enum1153::TeamAssignmentNumber => Ok(Enum1153::TeamAssignmentNumber),
            crate::zugferd_2_3_2::Enum1153::ContractNumber => Ok(Enum1153::ContractNumber),
            crate::zugferd_2_3_2::Enum1153::ConsignmentIdentifierConsignorAssigned => Ok(Enum1153::ConsignmentIdentifierConsignorAssigned),
            crate::zugferd_2_3_2::Enum1153::ContainerOperatorsReferenceNumber => Ok(Enum1153::ContainerOperatorsReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::PackageNumber => Ok(Enum1153::PackageNumber),
            crate::zugferd_2_3_2::Enum1153::CooperationContractNumber => Ok(Enum1153::CooperationContractNumber),
            crate::zugferd_2_3_2::Enum1153::DefermentApprovalNumber => Ok(Enum1153::DefermentApprovalNumber),
            crate::zugferd_2_3_2::Enum1153::DebitAccountNumber => Ok(Enum1153::DebitAccountNumber),
            crate::zugferd_2_3_2::Enum1153::BuyersDebtorNumber => Ok(Enum1153::BuyersDebtorNumber),
            crate::zugferd_2_3_2::Enum1153::DistributorInvoiceNumber => Ok(Enum1153::DistributorInvoiceNumber),
            crate::zugferd_2_3_2::Enum1153::DebitNoteNumber => Ok(Enum1153::DebitNoteNumber),
            crate::zugferd_2_3_2::Enum1153::DocumentIdentifier => Ok(Enum1153::DocumentIdentifier),
            crate::zugferd_2_3_2::Enum1153::DeliveryNoteNumber => Ok(Enum1153::DeliveryNoteNumber),
            crate::zugferd_2_3_2::Enum1153::DockReceiptNumber => Ok(Enum1153::DockReceiptNumber),
            crate::zugferd_2_3_2::Enum1153::EndingMeterReadingActual => Ok(Enum1153::EndingMeterReadingActual),
            crate::zugferd_2_3_2::Enum1153::EmbargoPermitNumber => Ok(Enum1153::EmbargoPermitNumber),
            crate::zugferd_2_3_2::Enum1153::ExportDeclaration => Ok(Enum1153::ExportDeclaration),
            crate::zugferd_2_3_2::Enum1153::EndingMeterReadingEstimated => Ok(Enum1153::EndingMeterReadingEstimated),
            crate::zugferd_2_3_2::Enum1153::ElectricalAndElectronicEquipmentProducerRegistration => Ok(Enum1153::ElectricalAndElectronicEquipmentProducerRegistration),
            crate::zugferd_2_3_2::Enum1153::EmployersIdentificationNumber => Ok(Enum1153::EmployersIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::EmbargoNumber => Ok(Enum1153::EmbargoNumber),
            crate::zugferd_2_3_2::Enum1153::EquipmentNumber => Ok(Enum1153::EquipmentNumber),
            crate::zugferd_2_3_2::Enum1153::ContainerEquipmentReceiptNumber => Ok(Enum1153::ContainerEquipmentReceiptNumber),
            crate::zugferd_2_3_2::Enum1153::ExportersReferenceNumber => Ok(Enum1153::ExportersReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::ExcessTransportationNumber => Ok(Enum1153::ExcessTransportationNumber),
            crate::zugferd_2_3_2::Enum1153::ExportPermitIdentifier => Ok(Enum1153::ExportPermitIdentifier),
            crate::zugferd_2_3_2::Enum1153::FiscalNumber => Ok(Enum1153::FiscalNumber),
            crate::zugferd_2_3_2::Enum1153::ConsignmentIdentifierFreightForwarderAssigned => Ok(Enum1153::ConsignmentIdentifierFreightForwarderAssigned),
            crate::zugferd_2_3_2::Enum1153::FileLineIdentifier => Ok(Enum1153::FileLineIdentifier),
            crate::zugferd_2_3_2::Enum1153::FlowReferenceNumber => Ok(Enum1153::FlowReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::FreightBillNumber => Ok(Enum1153::FreightBillNumber),
            crate::zugferd_2_3_2::Enum1153::ForeignExchange => Ok(Enum1153::ForeignExchange),
            crate::zugferd_2_3_2::Enum1153::FinalSequenceNumber => Ok(Enum1153::FinalSequenceNumber),
            crate::zugferd_2_3_2::Enum1153::FreeZoneIdentifier => Ok(Enum1153::FreeZoneIdentifier),
            crate::zugferd_2_3_2::Enum1153::FileVersionNumber => Ok(Enum1153::FileVersionNumber),
            crate::zugferd_2_3_2::Enum1153::ForeignExchangeContractNumber => Ok(Enum1153::ForeignExchangeContractNumber),
            crate::zugferd_2_3_2::Enum1153::StandardsNumber => Ok(Enum1153::StandardsNumber),
            crate::zugferd_2_3_2::Enum1153::GovernmentContractNumber => Ok(Enum1153::GovernmentContractNumber),
            crate::zugferd_2_3_2::Enum1153::StandardsCodeNumber => Ok(Enum1153::StandardsCodeNumber),
            crate::zugferd_2_3_2::Enum1153::GeneralDeclarationNumber => Ok(Enum1153::GeneralDeclarationNumber),
            crate::zugferd_2_3_2::Enum1153::GovernmentReferenceNumber => Ok(Enum1153::GovernmentReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::HarmonisedSystemNumber => Ok(Enum1153::HarmonisedSystemNumber),
            crate::zugferd_2_3_2::Enum1153::HouseWaybillNumber => Ok(Enum1153::HouseWaybillNumber),
            crate::zugferd_2_3_2::Enum1153::InternalVendorNumber => Ok(Enum1153::InternalVendorNumber),
            crate::zugferd_2_3_2::Enum1153::InBondNumber => Ok(Enum1153::InBondNumber),
            crate::zugferd_2_3_2::Enum1153::IataCargoAgentCodeNumber => Ok(Enum1153::IataCargoAgentCodeNumber),
            crate::zugferd_2_3_2::Enum1153::InsuranceCertificateReferenceNumber => Ok(Enum1153::InsuranceCertificateReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::InsuranceContractReferenceNumber => Ok(Enum1153::InsuranceContractReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::InitialSampleInspectionReportNumber => Ok(Enum1153::InitialSampleInspectionReportNumber),
            crate::zugferd_2_3_2::Enum1153::InternalOrderNumber => Ok(Enum1153::InternalOrderNumber),
            crate::zugferd_2_3_2::Enum1153::IntermediaryBroker => Ok(Enum1153::IntermediaryBroker),
            crate::zugferd_2_3_2::Enum1153::InterchangeNumberNew => Ok(Enum1153::InterchangeNumberNew),
            crate::zugferd_2_3_2::Enum1153::InterchangeNumberOld => Ok(Enum1153::InterchangeNumberOld),
            crate::zugferd_2_3_2::Enum1153::ImportPermitIdentifier => Ok(Enum1153::ImportPermitIdentifier),
            crate::zugferd_2_3_2::Enum1153::InvoiceNumberSuffix => Ok(Enum1153::InvoiceNumberSuffix),
            crate::zugferd_2_3_2::Enum1153::InternalCustomerNumber => Ok(Enum1153::InternalCustomerNumber),
            crate::zugferd_2_3_2::Enum1153::InvoiceDocumentIdentifier => Ok(Enum1153::InvoiceDocumentIdentifier),
            crate::zugferd_2_3_2::Enum1153::JobNumber => Ok(Enum1153::JobNumber),
            crate::zugferd_2_3_2::Enum1153::EndingJobSequenceNumber => Ok(Enum1153::EndingJobSequenceNumber),
            crate::zugferd_2_3_2::Enum1153::ShippingLabelSerialNumber => Ok(Enum1153::ShippingLabelSerialNumber),
            crate::zugferd_2_3_2::Enum1153::LoadingAuthorisationIdentifier => Ok(Enum1153::LoadingAuthorisationIdentifier),
            crate::zugferd_2_3_2::Enum1153::LowerNumberInRange => Ok(Enum1153::LowerNumberInRange),
            crate::zugferd_2_3_2::Enum1153::Lockbox => Ok(Enum1153::Lockbox),
            crate::zugferd_2_3_2::Enum1153::LetterCreditNumber => Ok(Enum1153::LetterCreditNumber),
            crate::zugferd_2_3_2::Enum1153::DocumentLineIdentifier => Ok(Enum1153::DocumentLineIdentifier),
            crate::zugferd_2_3_2::Enum1153::LoadPlanningNumber => Ok(Enum1153::LoadPlanningNumber),
            crate::zugferd_2_3_2::Enum1153::ReservationOfficeIdentifier => Ok(Enum1153::ReservationOfficeIdentifier),
            crate::zugferd_2_3_2::Enum1153::BarCodedLabelSerialNumber => Ok(Enum1153::BarCodedLabelSerialNumber),
            crate::zugferd_2_3_2::Enum1153::ShipNoticeManifestNumber => Ok(Enum1153::ShipNoticeManifestNumber),
            crate::zugferd_2_3_2::Enum1153::MasterBillLadingNumber => Ok(Enum1153::MasterBillLadingNumber),
            crate::zugferd_2_3_2::Enum1153::ManufacturersPartNumber => Ok(Enum1153::ManufacturersPartNumber),
            crate::zugferd_2_3_2::Enum1153::MeterUnitNumber => Ok(Enum1153::MeterUnitNumber),
            crate::zugferd_2_3_2::Enum1153::ManufacturingOrderNumber => Ok(Enum1153::ManufacturingOrderNumber),
            crate::zugferd_2_3_2::Enum1153::MessageRecipient => Ok(Enum1153::MessageRecipient),
            crate::zugferd_2_3_2::Enum1153::MailingReferenceNumber => Ok(Enum1153::MailingReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::MessageSender => Ok(Enum1153::MessageSender),
            crate::zugferd_2_3_2::Enum1153::ManufacturersMaterialSafetyDataSheetNumber => Ok(Enum1153::ManufacturersMaterialSafetyDataSheetNumber),
            crate::zugferd_2_3_2::Enum1153::MasterAirWaybillNumber => Ok(Enum1153::MasterAirWaybillNumber),
            crate::zugferd_2_3_2::Enum1153::NorthAmericanHazardousGoodsClassificationNumber => Ok(Enum1153::NorthAmericanHazardousGoodsClassificationNumber),
            crate::zugferd_2_3_2::Enum1153::NotaFiscal => Ok(Enum1153::NotaFiscal),
            crate::zugferd_2_3_2::Enum1153::CurrentInvoiceNumber => Ok(Enum1153::CurrentInvoiceNumber),
            crate::zugferd_2_3_2::Enum1153::PreviousInvoiceNumber => Ok(Enum1153::PreviousInvoiceNumber),
            crate::zugferd_2_3_2::Enum1153::OrderDocumentIdentifierBuyerAssigned => Ok(Enum1153::OrderDocumentIdentifierBuyerAssigned),
            crate::zugferd_2_3_2::Enum1153::OriginalPurchaseOrder => Ok(Enum1153::OriginalPurchaseOrder),
            crate::zugferd_2_3_2::Enum1153::GeneralOrderNumber => Ok(Enum1153::GeneralOrderNumber),
            crate::zugferd_2_3_2::Enum1153::PayersFinancialInstitutionAccountNumber => Ok(Enum1153::PayersFinancialInstitutionAccountNumber),
            crate::zugferd_2_3_2::Enum1153::ProductionCode => Ok(Enum1153::ProductionCode),
            crate::zugferd_2_3_2::Enum1153::PromotionDealNumber => Ok(Enum1153::PromotionDealNumber),
            crate::zugferd_2_3_2::Enum1153::PlantNumber => Ok(Enum1153::PlantNumber),
            crate::zugferd_2_3_2::Enum1153::PrimeContractorContractNumber => Ok(Enum1153::PrimeContractorContractNumber),
            crate::zugferd_2_3_2::Enum1153::PriceListVersionNumber => Ok(Enum1153::PriceListVersionNumber),
            crate::zugferd_2_3_2::Enum1153::PackingListNumber => Ok(Enum1153::PackingListNumber),
            crate::zugferd_2_3_2::Enum1153::PriceListNumber => Ok(Enum1153::PriceListNumber),
            crate::zugferd_2_3_2::Enum1153::PurchaseOrderResponseNumber => Ok(Enum1153::PurchaseOrderResponseNumber),
            crate::zugferd_2_3_2::Enum1153::PurchaseOrderChangeNumber => Ok(Enum1153::PurchaseOrderChangeNumber),
            crate::zugferd_2_3_2::Enum1153::PaymentReference => Ok(Enum1153::PaymentReference),
            crate::zugferd_2_3_2::Enum1153::PriceQuoteNumber => Ok(Enum1153::PriceQuoteNumber),
            crate::zugferd_2_3_2::Enum1153::PurchaseOrderNumberSuffix => Ok(Enum1153::PurchaseOrderNumberSuffix),
            crate::zugferd_2_3_2::Enum1153::PriorPurchaseOrderNumber => Ok(Enum1153::PriorPurchaseOrderNumber),
            crate::zugferd_2_3_2::Enum1153::PayeesFinancialInstitutionAccountNumber => Ok(Enum1153::PayeesFinancialInstitutionAccountNumber),
            crate::zugferd_2_3_2::Enum1153::RemittanceAdviceNumber => Ok(Enum1153::RemittanceAdviceNumber),
            crate::zugferd_2_3_2::Enum1153::RailRoadRoutingCode => Ok(Enum1153::RailRoadRoutingCode),
            crate::zugferd_2_3_2::Enum1153::RailwayConsignmentNoteNumber => Ok(Enum1153::RailwayConsignmentNoteNumber),
            crate::zugferd_2_3_2::Enum1153::ReleaseNumber => Ok(Enum1153::ReleaseNumber),
            crate::zugferd_2_3_2::Enum1153::ConsignmentReceiptIdentifier => Ok(Enum1153::ConsignmentReceiptIdentifier),
            crate::zugferd_2_3_2::Enum1153::ExportReferenceNumber => Ok(Enum1153::ExportReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::PayersFinancialInstitutionTransitRoutingNoAch => Ok(Enum1153::PayersFinancialInstitutionTransitRoutingNoAch),
            crate::zugferd_2_3_2::Enum1153::PayeesFinancialInstitutionTransitRoutingNo => Ok(Enum1153::PayeesFinancialInstitutionTransitRoutingNo),
            crate::zugferd_2_3_2::Enum1153::SalesPersonNumber => Ok(Enum1153::SalesPersonNumber),
            crate::zugferd_2_3_2::Enum1153::SalesRegionNumber => Ok(Enum1153::SalesRegionNumber),
            crate::zugferd_2_3_2::Enum1153::SalesDepartmentNumber => Ok(Enum1153::SalesDepartmentNumber),
            crate::zugferd_2_3_2::Enum1153::SerialNumber => Ok(Enum1153::SerialNumber),
            crate::zugferd_2_3_2::Enum1153::AllocatedSeat => Ok(Enum1153::AllocatedSeat),
            crate::zugferd_2_3_2::Enum1153::ShipFrom => Ok(Enum1153::ShipFrom),
            crate::zugferd_2_3_2::Enum1153::PreviousHighestScheduleNumber => Ok(Enum1153::PreviousHighestScheduleNumber),
            crate::zugferd_2_3_2::Enum1153::SidShippersIdentifyingNumberForShipment => Ok(Enum1153::SidShippersIdentifyingNumberForShipment),
            crate::zugferd_2_3_2::Enum1153::SalesOfficeNumber => Ok(Enum1153::SalesOfficeNumber),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentSealIdentifier => Ok(Enum1153::TransportEquipmentSealIdentifier),
            crate::zugferd_2_3_2::Enum1153::ScanLine => Ok(Enum1153::ScanLine),
            crate::zugferd_2_3_2::Enum1153::EquipmentSequenceNumber => Ok(Enum1153::EquipmentSequenceNumber),
            crate::zugferd_2_3_2::Enum1153::ShipmentReferenceNumber => Ok(Enum1153::ShipmentReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::SellersReferenceNumber => Ok(Enum1153::SellersReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::StationReferenceNumber => Ok(Enum1153::StationReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::SwapOrderNumber => Ok(Enum1153::SwapOrderNumber),
            crate::zugferd_2_3_2::Enum1153::SpecificationNumber => Ok(Enum1153::SpecificationNumber),
            crate::zugferd_2_3_2::Enum1153::TruckersBillLading => Ok(Enum1153::TruckersBillLading),
            crate::zugferd_2_3_2::Enum1153::TerminalOperatorsConsignmentReference => Ok(Enum1153::TerminalOperatorsConsignmentReference),
            crate::zugferd_2_3_2::Enum1153::TelexMessageNumber => Ok(Enum1153::TelexMessageNumber),
            crate::zugferd_2_3_2::Enum1153::TransferNumber => Ok(Enum1153::TransferNumber),
            crate::zugferd_2_3_2::Enum1153::TirCarnetNumber => Ok(Enum1153::TirCarnetNumber),
            crate::zugferd_2_3_2::Enum1153::TransportInstructionNumber => Ok(Enum1153::TransportInstructionNumber),
            crate::zugferd_2_3_2::Enum1153::TaxExemptionLicenceNumber => Ok(Enum1153::TaxExemptionLicenceNumber),
            crate::zugferd_2_3_2::Enum1153::TransactionReferenceNumber => Ok(Enum1153::TransactionReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::TestReportNumber => Ok(Enum1153::TestReportNumber),
            crate::zugferd_2_3_2::Enum1153::UpperNumberRange => Ok(Enum1153::UpperNumberRange),
            crate::zugferd_2_3_2::Enum1153::UltimateCustomersReferenceNumber => Ok(Enum1153::UltimateCustomersReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::UniqueConsignmentReferenceNumber => Ok(Enum1153::UniqueConsignmentReferenceNumber),
            crate::zugferd_2_3_2::Enum1153::UnitedNationsDangerousGoodsIdentifier => Ok(Enum1153::UnitedNationsDangerousGoodsIdentifier),
            crate::zugferd_2_3_2::Enum1153::UltimateCustomersOrderNumber => Ok(Enum1153::UltimateCustomersOrderNumber),
            crate::zugferd_2_3_2::Enum1153::UniformResourceIdentifier => Ok(Enum1153::UniformResourceIdentifier),
            crate::zugferd_2_3_2::Enum1153::VatRegistrationNumber => Ok(Enum1153::VatRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::VendorContractNumber => Ok(Enum1153::VendorContractNumber),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentGrossMassVerificationReference => Ok(Enum1153::TransportEquipmentGrossMassVerificationReference),
            crate::zugferd_2_3_2::Enum1153::VesselIdentifier => Ok(Enum1153::VesselIdentifier),
            crate::zugferd_2_3_2::Enum1153::OrderNumberVendor => Ok(Enum1153::OrderNumberVendor),
            crate::zugferd_2_3_2::Enum1153::VoyageNumber => Ok(Enum1153::VoyageNumber),
            crate::zugferd_2_3_2::Enum1153::TransportEquipmentGrossMassVerificationOrderReference => Ok(Enum1153::TransportEquipmentGrossMassVerificationOrderReference),
            crate::zugferd_2_3_2::Enum1153::VendorProductNumber => Ok(Enum1153::VendorProductNumber),
            crate::zugferd_2_3_2::Enum1153::VendorIdNumber => Ok(Enum1153::VendorIdNumber),
            crate::zugferd_2_3_2::Enum1153::VendorOrderNumberSuffix => Ok(Enum1153::VendorOrderNumberSuffix),
            crate::zugferd_2_3_2::Enum1153::MotorVehicleIdentificationNumber => Ok(Enum1153::MotorVehicleIdentificationNumber),
            crate::zugferd_2_3_2::Enum1153::VoucherNumber => Ok(Enum1153::VoucherNumber),
            crate::zugferd_2_3_2::Enum1153::WarehouseEntryNumber => Ok(Enum1153::WarehouseEntryNumber),
            crate::zugferd_2_3_2::Enum1153::WeightAgreementNumber => Ok(Enum1153::WeightAgreementNumber),
            crate::zugferd_2_3_2::Enum1153::WellNumber => Ok(Enum1153::WellNumber),
            crate::zugferd_2_3_2::Enum1153::WarehouseReceiptNumber => Ok(Enum1153::WarehouseReceiptNumber),
            crate::zugferd_2_3_2::Enum1153::WarehouseStorageLocationNumber => Ok(Enum1153::WarehouseStorageLocationNumber),
            crate::zugferd_2_3_2::Enum1153::RailWaybillNumber => Ok(Enum1153::RailWaybillNumber),
            crate::zugferd_2_3_2::Enum1153::CompanyPlaceRegistrationNumber => Ok(Enum1153::CompanyPlaceRegistrationNumber),
            crate::zugferd_2_3_2::Enum1153::CargoControlNumber => Ok(Enum1153::CargoControlNumber),
            crate::zugferd_2_3_2::Enum1153::PreviousCargoControlNumber => Ok(Enum1153::PreviousCargoControlNumber),
            crate::zugferd_2_3_2::Enum1153::MutuallyDefinedReferenceNumber => Ok(Enum1153::MutuallyDefinedReferenceNumber),

        }
    }
}
// End: (Version) TryFrom crate::zugferd_2_3_2::Enum1153 to Enum1153
