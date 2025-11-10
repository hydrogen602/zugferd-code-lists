#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum ICD {
    /// System Information et Repertoire des Entreprise et des Etablissements: SIRENE
    SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene,
    /// Codification Numerique des Etablissments Financiers En Belgique
    CodificationNumeriqueDesEtablissmentsFinanciersEnBelgique,
    /// NBS/OSI NETWORK
    NbsOsiNetwork,
    /// USA FED GOV OSI NETWORK
    UsaFedGovOsiNetwork,
    /// USA DOD OSI NETWORK
    UsaDodOsiNetwork,
    /// Organisationsnummer
    Organisationsnummer,
    /// LE NUMERO NATIONAL
    LeNumeroNational,
    /// SIRET-CODE
    SiretCode,
    /// Organizational Identifiers for Structured Names under ISO 9541 Part 2
    OrganizationalIdentifiersForStructuredNamesUnderIso9541Part2,
    /// International Code Designator for the Identification of OSI-based, Amateur Radio Organizations, Network Objects and Application Services.
    InternationalCodeDesignatorForIdentificationOsiBasedAmateurRadioOrganizationsNetworkObjectsAndApplicationServices,
    /// European Computer Manufacturers Association: ECMA
    EuropeanComputerManufacturersAssociationEcma,
    /// VSA FTP CODE (FTP = File Transfer Protocol)
    VsaFtpCodeFtpFileTransferProtocol,
    /// NIST/OSI Implememts' Workshop
    NistOsiImplememtsWorkshop,
    /// Electronic Data Interchange: EDI
    ElectronicDataInterchangeEdi,
    /// EWOS Object Identifiers
    EwosObjectIdentifiers,
    /// COMMON LANGUAGE
    CommonLanguage,
    /// SNA/OSI Network
    SnaOsiNetwork,
    /// Air Transport Industry Services Communications Network
    AirTransportIndustryServicesCommunicationsNetwork,
    /// European Laboratory for Particle Physics: CERN
    EuropeanLaboratoryForParticlePhysicsCern,
    /// SOCIETY FOR WORLDWIDE INTERBANK FINANCIAL, TELECOMMUNICATION S.W.I.F.T.
    SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT,
    /// OSF Distributed Computing Object Identification
    OsfDistributedComputingObjectIdentification,
    /// Nordic University and Research Network: NORDUnet
    NordicUniversityAndResearchNetworkNordunet,
    /// Digital Equipment Corporation: DEC
    DigitalEquipmentCorporationDec,
    /// OSI ASIA-OCEANIA WORKSHOP
    OsiAsiaOceaniaWorkshop,
    /// NATO ISO 6523 ICDE coding scheme
    NatoIso6523IcdeCodingScheme,
    /// Aeronautical Telecommunications Network (ATN)
    AeronauticalTelecommunicationsNetworkAtn,
    /// International Standard ISO 6523
    InternationalStandardIso6523,
    /// The All-Union Classifier of Enterprises and Organisations
    TheAllUnionClassifierEnterprisesAndOrganisations,
    /// AT&T/OSI Network
    AtTOsiNetwork,
    /// EDI Partner Identification Code
    EdiPartnerIdentificationCode,
    /// Telecom Australia
    TelecomAustralia,
    /// S G W OSI Internetwork
    SGWOsiInternetwork,
    /// Reuter Open Address Standard
    ReuterOpenAddressStandard,
    /// ISO 6523 - ICD
    Iso6523Icd,
    /// TeleTrust Object Identifiers
    TeletrustObjectIdentifiers,
    /// LY-tunnus
    LyTunnus,
    /// The Australian GOSIP Network
    TheAustralianGosipNetwork,
    /// The OZ DOD OSI Network
    TheOzDodOsiNetwork,
    /// Unilever Group Companies
    UnileverGroupCompanies,
    /// Citicorp Global Information Network
    CiticorpGlobalInformationNetwork,
    /// DBP Telekom Object Identifiers
    DbpTelekomObjectIdentifiers,
    /// HydroNETT
    Hydronett,
    /// Thai Industrial Standards Institute (TISI)
    ThaiIndustrialStandardsInstituteTisi,
    /// ICI Company Identification System
    IciCompanyIdentificationSystem,
    /// FUNLOC
    Funloc,
    /// BULL ODI/DSA/UNIX Network
    BullOdiDsaUnixNetwork,
    /// OSINZ
    Osinz,
    /// Auckland Area Health
    AucklandAreaHealth,
    /// Firmenich
    Firmenich,
    /// AGFA-DIS
    AgfaDis,
    /// Society of Motion Picture and Television Engineers (SMPTE)
    SocietyMotionPictureAndTelevisionEngineersSmpte,
    /// Migros_Network M_NETOPZ
    Migros_networkM_netopz,
    /// ISO6523 - ICDPCR
    Iso6523Icdpcr,
    /// Energy Net
    EnergyNet,
    /// Nokia Object Identifiers (NOI)
    NokiaObjectIdentifiersNoi,
    /// Saint Gobain
    SaintGobain,
    /// Siemens Corporate Network
    SiemensCorporateNetwork,
    /// DANZNET
    Danznet,
    /// Data Universal Numbering System (D-U-N-S Number)
    DataUniversalNumberingSystemDUNSNumber,
    /// SOFFEX OSI
    SoffexOsi,
    /// KPN OVN
    KpnOvn,
    /// ascomOSINet
    Ascomosinet,
    /// UTC: Uniforme Transport Code
    UtcUniformeTransportCode,
    /// SOLVAY OSI CODING
    SolvayOsiCoding,
    /// Roche Corporate Network
    RocheCorporateNetwork,
    /// ZellwegerOSINet
    Zellwegerosinet,
    /// Intel Corporation OSI
    IntelCorporationOsi,
    /// SITA Object Identifier Tree
    SitaObjectIdentifierTree,
    /// DaimlerChrysler Corporate Network
    DaimlerchryslerCorporateNetwork,
    /// LEGO /OSI NETWORK
    LegoOsiNetwork,
    /// NAVISTAR/OSI Network
    NavistarOsiNetwork,
    /// ICD Formatted ATM address
    IcdFormattedAtmAddress,
    /// ARINC
    Arinc,
    /// Alcanet/Alcatel-Alsthom Corporate Network
    AlcanetAlcatelAlsthomCorporateNetwork,
    /// Sistema Italiano di Identificazione di ogetti gestito da UNINFO
    SistemaItalianoDiIdentificazioneDiOgettiGestitoDaUninfo,
    /// Sistema Italiano di Indirizzamento di Reti OSI Gestito da UNINFO
    SistemaItalianoDiIndirizzamentoDiRetiOsiGestitoDaUninfo,
    /// Mitel terminal or switching equipment
    MitelTerminalOrSwitchingEquipment,
    /// ATM Forum
    AtmForum,
    /// UK National Health Service Scheme, (EDIRA compliant)
    UkNationalHealthServiceSchemeEdiraCompliant,
    /// International NSAP
    InternationalNsap,
    /// Norwegian Telecommunications Authority's, NTA'S, EDI, identifier scheme (EDIRA compliant)
    NorwegianTelecommunicationsAuthoritysNtasEdiIdentifierSchemeEdiraCompliant,
    /// Advanced Telecommunications Modules Limited, Corporate Network
    AdvancedTelecommunicationsModulesLimitedCorporateNetwork,
    /// Athens Chamber of Commerce & Industry Scheme (EDIRA compliant)
    AthensChamberCommerceIndustrySchemeEdiraCompliant,
    /// Swiss Chambers of Commerce Scheme (EDIRA) compliant
    SwissChambersCommerceSchemeEdiraCompliant,
    /// United States Council for International Business (USCIB) Scheme, (EDIRA compliant)
    UnitedStatesCouncilForInternationalBusinessUscibSchemeEdiraCompliant,
    /// National Federation of Chambers of Commerce & Industry of Belgium, Scheme (EDIRA compliant)
    NationalFederationChambersCommerceIndustryBelgiumSchemeEdiraCompliant,
    /// EAN Location Code
    EanLocationCode,
    /// The Association of British Chambers of Commerce Ltd. Scheme, (EDIRA compliant)
    TheAssociationBritishChambersCommerceLtdSchemeEdiraCompliant,
    /// Internet IP addressing - ISO 6523 ICD encoding
    InternetIpAddressingIso6523IcdEncoding,
    /// Cisco Sysytems / OSI Network
    CiscoSysytemsOsiNetwork,
    /// Revenue Canada Business Number Registration (EDIRA compliant)
    RevenueCanadaBusinessNumberRegistrationEdiraCompliant,
    /// DEUTSCHER INDUSTRIE- UND HANDELSTAG (DIHT) Scheme (EDIRA compliant)
    DeutscherIndustrieUndHandelstagDihtSchemeEdiraCompliant,
    /// Hewlett - Packard Company Internal AM Network
    HewlettPackardCompanyInternalAmNetwork,
    /// DANISH CHAMBER OF COMMERCE Scheme (EDIRA compliant)
    DanishChamberOfCommerceSchemeEdiraCompliant,
    /// FTI - Ediforum Italia, (EDIRA compliant)
    FtiEdiforumItaliaEdiraCompliant,
    /// CHAMBER OF COMMERCE TEL AVIV-JAFFA Scheme (EDIRA compliant)
    ChamberOfCommerceTelAvivJaffaSchemeEdiraCompliant,
    /// Siemens Supervisory Systems Network
    SiemensSupervisorySystemsNetwork,
    /// PNG_ICD Scheme
    Png_icdScheme,
    /// South African Code Allocation
    SouthAfricanCodeAllocation,
    /// HEAG
    Heag,
    /// BT - ICD Coding System
    BtIcdCodingSystem,
    /// Portuguese Chamber of Commerce and Industry Scheme (EDIRA compliant)
    PortugueseChamberCommerceAndIndustrySchemeEdiraCompliant,
    /// Vereniging van Kamers van Koophandel en Fabrieken in Nederland (Association of Chambers of Commerce and Industry in the Netherlands), Scheme (EDIRA compliant)
    VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant,
    /// Association of Swedish Chambers of Commerce and Industry Scheme (EDIRA compliant)
    AssociationSwedishChambersCommerceAndIndustrySchemeEdiraCompliant,
    /// Australian Chambers of Commerce and Industry Scheme (EDIRA compliant)
    AustralianChambersCommerceAndIndustrySchemeEdiraCompliant,
    /// BellSouth ICD AESA (ATM End System Address)
    BellsouthIcdAesaAtmEndSystemAddress,
    /// Bell Atlantic
    BellAtlantic,
    /// Object Identifiers
    ObjectIdentifiers,
    /// ISO register for Standards producing Organizations
    IsoRegisterForStandardsProducingOrganizations,
    /// OriginNet
    Originnet,
    /// Check Point Software Technologies
    CheckPointSoftwareTechnologies,
    /// Pacific Bell Data Communications Network
    PacificBellDataCommunicationsNetwork,
    /// PSS Object Identifiers
    PssObjectIdentifiers,
    /// STENTOR-ICD CODING SYSTEM
    StentorIcdCodingSystem,
    /// ATM-Network ZN'96
    AtmNetworkZn96,
    /// MCI / OSI Network
    MciOsiNetwork,
    /// Advantis
    Advantis,
    /// Affable Software Data Interchange Codes
    AffableSoftwareDataInterchangeCodes,
    /// BB-DATA GmbH
    BbDataGmbh,
    /// BASF Company ATM-Network
    BasfCompanyAtmNetwork,
    /// IOTA Identifiers for Organizations for Telecommunications Addressing using the ICD system format defined in ISO/IEC 8348
    IotaIdentifiersForOrganizationsForTelecommunicationsAddressingUsingIcdSystemFormatDefinedInIsoIec8348,
    /// Henkel Corporate Network (H-Net)
    HenkelCorporateNetworkHNet,
    /// GTE/OSI Network
    GteOsiNetwork,
    /// Dresdner Bank Corporate Network
    DresdnerBankCorporateNetwork,
    /// BCNR (Swiss Clearing Bank Number)
    BcnrSwissClearingBankNumber,
    /// BPI (Swiss Business Partner Identification) code
    BpiSwissBusinessPartnerIdentificationCode,
    /// Directorates of the European Commission
    DirectoratesEuropeanCommission,
    /// Code for the Identification of National Organizations
    CodeForIdentificationNationalOrganizations,
    /// Certicom Object Identifiers
    CerticomObjectIdentifiers,
    /// TC68 OID
    Tc68Oid,
    /// Infonet Services Corporation
    InfonetServicesCorporation,
    /// SIA Object Identifiers
    SiaObjectIdentifiers,
    /// Cable & Wireless Global ATM End-System Address Plan
    CableWirelessGlobalAtmEndSystemAddressPlan,
    /// Global AESA scheme
    GlobalAesaScheme,
    /// France Telecom ATM End System Address Plan
    FranceTelecomAtmEndSystemAddressPlan,
    /// Savvis Communications AESA:.
    SavvisCommunicationsAesa,
    /// Toshiba Organizations, Partners, And Suppliers' (TOPAS) Code
    ToshibaOrganizationsPartnersAndSuppliersTopasCode,
    /// NATO Commercial and Government Entity system
    NatoCommercialAndGovernmentEntitySystem,
    /// SECETI Object Identifiers
    SecetiObjectIdentifiers,
    /// EINESTEINet AG
    EinesteinetAg,
    /// DoDAAC (Department of Defense Activity Address Code)
    DodaacDepartmentDefenseActivityAddressCode,
    /// DGCP (Direction Générale de la Comptabilité Publique)administrative accounting identification scheme
    DgcpDirectionGénéraleDeLaComptabilitéPubliqueAdministrativeAccountingIdentificationScheme,
    /// DGI (Direction Générale des Impots) code
    DgiDirectionGénéraleDesImpotsCode,
    /// Standard Company Code
    StandardCompanyCode,
    /// ITU (International Telecommunications Union)Data Network Identification Codes (DNIC)
    ItuInternationalTelecommunicationsUnionDataNetworkIdentificationCodesDnic,
    /// Global Business Identifier
    GlobalBusinessIdentifier,
    /// Madge Networks Ltd- ICD ATM Addressing Scheme
    MadgeNetworksLtdIcdAtmAddressingScheme,
    /// Australian Business Number (ABN) Scheme
    AustralianBusinessNumberAbnScheme,
    /// Edira Scheme Identifier Code
    EdiraSchemeIdentifierCode,
    /// Concert Global Network Services ICD AESA
    ConcertGlobalNetworkServicesIcdAesa,
    /// Identification number of economic subjects: (ICO)
    IdentificationNumberEconomicSubjectsIco,
    /// Global Crossing AESA (ATM End System Address)
    GlobalCrossingAesaAtmEndSystemAddress,
    /// AUNA
    Auna,
    /// ATM interconnection with the Dutch KPN Telecom
    AtmInterconnectionWithDutchKpnTelecom,
    /// Identification number of economic subject (ICO) Act on State Statistics of 29 November 2'001, § 27
    IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127,
    /// ACTALIS Object Identifiers
    ActalisObjectIdentifiers,
    /// GTIN - Global Trade Item Number
    GtinGlobalTradeItemNumber,
    /// ECCMA Open Technical Directory
    EccmaOpenTechnicalDirectory,
    /// CEN/ISSS Object Identifier Scheme
    CenIsssObjectIdentifierScheme,
    /// US-EPA Facility Identifier
    UsEpaFacilityIdentifier,
    /// TELUS Corporation
    TelusCorporation,
    /// FIEIE Object identifiers
    FieieObjectIdentifiers,
    /// Swissguide Identifier Scheme
    SwissguideIdentifierScheme,
    /// Priority Telecom ATM End System Address Plan
    PriorityTelecomAtmEndSystemAddressPlan,
    /// Vodafone Ireland OSI Addressing
    VodafoneIrelandOsiAddressing,
    /// Swiss Federal Business Identification Number. Central Business names Index (zefix) Identification Number
    SwissFederalBusinessIdentificationNumberCentralBusinessNamesIndexZefixIdentificationNumber,
    /// Teikoku Company Code
    TeikokuCompanyCode,
    /// Luxembourg CP & CPS (Certification Policy and Certification Practice Statement) Index
    LuxembourgCpCpsCertificationPolicyAndCertificationPracticeStatementIndex,
    /// Project Group “Lists of Properties” (PROLIST®)
    ProjectGroupListsPropertiesProlist,
    /// eCI@ss
    EciSs,
    /// StepNexus
    Stepnexus,
    /// Siemens AG
    SiemensAg,
    /// Paradine GmbH
    ParadineGmbh,
    /// Odette International Limited
    OdetteInternationalLimited,
    /// Route1 MobiNET
    Route1Mobinet,
    /// Penango Object Identifiers
    PenangoObjectIdentifiers,
    /// Lithuanian military PKI
    LithuanianMilitaryPki,
    /// Numéro d'identification suisse des enterprises (IDE), Swiss Unique Business Identification Number (UIDB)
    NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb,
    /// DIGSTORG
    Digstorg,
    /// Perceval Object Code
    PercevalObjectCode,
    /// TrustPoint Object Identifiers
    TrustpointObjectIdentifiers,
    /// Amazon Unique Identification Scheme
    AmazonUniqueIdentificationScheme,
    /// Corporate Number of The Social Security and Tax Number System
    CorporateNumberTheSocialSecurityAndTaxNumberSystem,
    /// European Business Identifier (EBID)
    EuropeanBusinessIdentifierEbid,
    /// Organisatie Indentificatie Nummer (OIN)
    OrganisatieIndentificatieNummerOin,
    /// Company Code (Estonia)
    CompanyCodeEstonia,
    /// Organisasjonsnummer
    Organisasjonsnummer,
    /// UBL.BE Party Identifier
    UblBePartyIdentifier,
    /// KOIOS Open Technical Dictionary
    KoiosOpenTechnicalDictionary,
    /// Singapore Nationwide E-lnvoice Framework
    SingaporeNationwideELnvoiceFramework,
    /// Icelandic identifier - Íslensk kennitala
    IcelandicIdentifierÍslenskKennitala,
    /// APPLiA Pl Standard
    AppliaPlStandard,
    /// ERSTORG
    Erstorg,
    /// Legal Entity Identifier (LEI)
    LegalEntityIdentifierLei,
    /// Legal entity code (Lithuania)
    LegalEntityCodeLithuania,
    /// Codice Univoco Unità Organizzativa iPA
    CodiceUnivocoUnitàOrganizzativaIpa,
    /// Indirizzo di Posta Elettronica Certificata
    IndirizzoDiPostaElettronicaCertificata,
    /// eDelivery Network Participant identifier
    EdeliveryNetworkParticipantIdentifier,
    /// Leitweg-ID
    LeitwegId,
    /// CODDEST
    Coddest,
    /// Registre du Commerce et de l’Industrie : RCI
    RegistreDuCommerceEtDeLIndustrieRci,
    /// PiLog Ontology Codification Identifier (POCI)
    PilogOntologyCodificationIdentifierPoci,
    /// Numero d'entreprise / ondernemingsnummer / Unternehmensnummer
    NumeroDentrepriseOndernemingsnummerUnternehmensnummer,
    /// GS1 identification keys
    Gs1IdentificationKeys,
    /// CODICE FISCALE
    CodiceFiscale,
    /// PARTITA IVA
    PartitaIva,
    /// Finnish Organization Identifier
    FinnishOrganizationIdentifier,
    /// Finnish Organization Value Add Tax Identifier
    FinnishOrganizationValueAddTaxIdentifier,
    /// Tradeplace TradePI Standard
    TradeplaceTradepiStandard,
    /// Net service ID
    NetServiceId,
    /// OVTcode
    Ovtcode,
    /// The Netherlands Chamber of Commerce and Industry establishment number
    TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber,
    /// Unified registration number (Latvia)
    UnifiedRegistrationNumberLatvia,
    /// Taxpayer registration code (Latvia)
    TaxpayerRegistrationCodeLatvia,
    /// The Register of Natural Persons (Latvia)
    TheRegisterNaturalPersonsLatvia,
    /// The registered number of the qualified invoice issuer
    TheRegisteredNumberQualifiedInvoiceIssuer,
    /// Metadata Registry Support
    MetadataRegistrySupport,
    /// EU based company
    EuBasedCompany,
    /// FTCTC CODE ROUTAGE
    FtctcCodeRoutage,
    /// FRCTC ELECTRONIC ADDRESS
    FrctcElectronicAddress,
    /// FRCTC Particulier
    FrctcParticulier,
    /// NON - EU based company
    NonEuBasedCompany,
    /// Répertoire des Entreprises et des Etablissements (RIDET)
    RépertoireDesEntreprisesEtDesEtablissementsRidet,
    /// T.A.H.I.T.I (traitement automatique hiérarchisé des institutions de Tahiti et des îles)
    TAHITITraitementAutomatiqueHiérarchiséDesInstitutionsDeTahitiEtDesÎles,
    /// National e-Invoicing Framework
    NationalEInvoicingFramework,
    /// Single taxable company (France)
    SingleTaxableCompanyFrance,
    /// NOBB product number
    NobbProductNumber,
    /// Description not known
    DescriptionNotKnown,
    /// Toimitusosoite ID
    ToimitusosoiteId,
    /// UAE Tax Identification Number (TIN)
    UaeTaxIdentificationNumberTin,
    /// Description not known
    DescriptionNotKnown_Dup,
    /// CPR (Danish person civil registration number)
    CprDanishPersonCivilRegistrationNumber,
    /// Plateforme.s agréée.s à la facturation électronique (PPF/PDP)
    PlateformeSAgrééeSÀLaFacturationÉlectroniquePpfPdp,
    /// EAEU
    Eaeu,
    /// Register of legal persons (in French : Répertoire des personnes morales)
    RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales,
}

impl crate::Code for ICD {
    fn code(self) -> &'static str {
        match self {
            ICD::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene => "0002",
            ICD::CodificationNumeriqueDesEtablissmentsFinanciersEnBelgique => "0003",
            ICD::NbsOsiNetwork => "0004",
            ICD::UsaFedGovOsiNetwork => "0005",
            ICD::UsaDodOsiNetwork => "0006",
            ICD::Organisationsnummer => "0007",
            ICD::LeNumeroNational => "0008",
            ICD::SiretCode => "0009",
            ICD::OrganizationalIdentifiersForStructuredNamesUnderIso9541Part2 => "0010",
            ICD::InternationalCodeDesignatorForIdentificationOsiBasedAmateurRadioOrganizationsNetworkObjectsAndApplicationServices => "0011",
            ICD::EuropeanComputerManufacturersAssociationEcma => "0012",
            ICD::VsaFtpCodeFtpFileTransferProtocol => "0013",
            ICD::NistOsiImplememtsWorkshop => "0014",
            ICD::ElectronicDataInterchangeEdi => "0015",
            ICD::EwosObjectIdentifiers => "0016",
            ICD::CommonLanguage => "0017",
            ICD::SnaOsiNetwork => "0018",
            ICD::AirTransportIndustryServicesCommunicationsNetwork => "0019",
            ICD::EuropeanLaboratoryForParticlePhysicsCern => "0020",
            ICD::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT => "0021",
            ICD::OsfDistributedComputingObjectIdentification => "0022",
            ICD::NordicUniversityAndResearchNetworkNordunet => "0023",
            ICD::DigitalEquipmentCorporationDec => "0024",
            ICD::OsiAsiaOceaniaWorkshop => "0025",
            ICD::NatoIso6523IcdeCodingScheme => "0026",
            ICD::AeronauticalTelecommunicationsNetworkAtn => "0027",
            ICD::InternationalStandardIso6523 => "0028",
            ICD::TheAllUnionClassifierEnterprisesAndOrganisations => "0029",
            ICD::AtTOsiNetwork => "0030",
            ICD::EdiPartnerIdentificationCode => "0031",
            ICD::TelecomAustralia => "0032",
            ICD::SGWOsiInternetwork => "0033",
            ICD::ReuterOpenAddressStandard => "0034",
            ICD::Iso6523Icd => "0035",
            ICD::TeletrustObjectIdentifiers => "0036",
            ICD::LyTunnus => "0037",
            ICD::TheAustralianGosipNetwork => "0038",
            ICD::TheOzDodOsiNetwork => "0039",
            ICD::UnileverGroupCompanies => "0040",
            ICD::CiticorpGlobalInformationNetwork => "0041",
            ICD::DbpTelekomObjectIdentifiers => "0042",
            ICD::Hydronett => "0043",
            ICD::ThaiIndustrialStandardsInstituteTisi => "0044",
            ICD::IciCompanyIdentificationSystem => "0045",
            ICD::Funloc => "0046",
            ICD::BullOdiDsaUnixNetwork => "0047",
            ICD::Osinz => "0048",
            ICD::AucklandAreaHealth => "0049",
            ICD::Firmenich => "0050",
            ICD::AgfaDis => "0051",
            ICD::SocietyMotionPictureAndTelevisionEngineersSmpte => "0052",
            ICD::Migros_networkM_netopz => "0053",
            ICD::Iso6523Icdpcr => "0054",
            ICD::EnergyNet => "0055",
            ICD::NokiaObjectIdentifiersNoi => "0056",
            ICD::SaintGobain => "0057",
            ICD::SiemensCorporateNetwork => "0058",
            ICD::Danznet => "0059",
            ICD::DataUniversalNumberingSystemDUNSNumber => "0060",
            ICD::SoffexOsi => "0061",
            ICD::KpnOvn => "0062",
            ICD::Ascomosinet => "0063",
            ICD::UtcUniformeTransportCode => "0064",
            ICD::SolvayOsiCoding => "0065",
            ICD::RocheCorporateNetwork => "0066",
            ICD::Zellwegerosinet => "0067",
            ICD::IntelCorporationOsi => "0068",
            ICD::SitaObjectIdentifierTree => "0069",
            ICD::DaimlerchryslerCorporateNetwork => "0070",
            ICD::LegoOsiNetwork => "0071",
            ICD::NavistarOsiNetwork => "0072",
            ICD::IcdFormattedAtmAddress => "0073",
            ICD::Arinc => "0074",
            ICD::AlcanetAlcatelAlsthomCorporateNetwork => "0075",
            ICD::SistemaItalianoDiIdentificazioneDiOgettiGestitoDaUninfo => "0076",
            ICD::SistemaItalianoDiIndirizzamentoDiRetiOsiGestitoDaUninfo => "0077",
            ICD::MitelTerminalOrSwitchingEquipment => "0078",
            ICD::AtmForum => "0079",
            ICD::UkNationalHealthServiceSchemeEdiraCompliant => "0080",
            ICD::InternationalNsap => "0081",
            ICD::NorwegianTelecommunicationsAuthoritysNtasEdiIdentifierSchemeEdiraCompliant => "0082",
            ICD::AdvancedTelecommunicationsModulesLimitedCorporateNetwork => "0083",
            ICD::AthensChamberCommerceIndustrySchemeEdiraCompliant => "0084",
            ICD::SwissChambersCommerceSchemeEdiraCompliant => "0085",
            ICD::UnitedStatesCouncilForInternationalBusinessUscibSchemeEdiraCompliant => "0086",
            ICD::NationalFederationChambersCommerceIndustryBelgiumSchemeEdiraCompliant => "0087",
            ICD::EanLocationCode => "0088",
            ICD::TheAssociationBritishChambersCommerceLtdSchemeEdiraCompliant => "0089",
            ICD::InternetIpAddressingIso6523IcdEncoding => "0090",
            ICD::CiscoSysytemsOsiNetwork => "0091",
            ICD::RevenueCanadaBusinessNumberRegistrationEdiraCompliant => "0093",
            ICD::DeutscherIndustrieUndHandelstagDihtSchemeEdiraCompliant => "0094",
            ICD::HewlettPackardCompanyInternalAmNetwork => "0095",
            ICD::DanishChamberOfCommerceSchemeEdiraCompliant => "0096",
            ICD::FtiEdiforumItaliaEdiraCompliant => "0097",
            ICD::ChamberOfCommerceTelAvivJaffaSchemeEdiraCompliant => "0098",
            ICD::SiemensSupervisorySystemsNetwork => "0099",
            ICD::Png_icdScheme => "0100",
            ICD::SouthAfricanCodeAllocation => "0101",
            ICD::Heag => "0102",
            ICD::BtIcdCodingSystem => "0104",
            ICD::PortugueseChamberCommerceAndIndustrySchemeEdiraCompliant => "0105",
            ICD::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant => "0106",
            ICD::AssociationSwedishChambersCommerceAndIndustrySchemeEdiraCompliant => "0107",
            ICD::AustralianChambersCommerceAndIndustrySchemeEdiraCompliant => "0108",
            ICD::BellsouthIcdAesaAtmEndSystemAddress => "0109",
            ICD::BellAtlantic => "0110",
            ICD::ObjectIdentifiers => "0111",
            ICD::IsoRegisterForStandardsProducingOrganizations => "0112",
            ICD::Originnet => "0113",
            ICD::CheckPointSoftwareTechnologies => "0114",
            ICD::PacificBellDataCommunicationsNetwork => "0115",
            ICD::PssObjectIdentifiers => "0116",
            ICD::StentorIcdCodingSystem => "0117",
            ICD::AtmNetworkZn96 => "0118",
            ICD::MciOsiNetwork => "0119",
            ICD::Advantis => "0120",
            ICD::AffableSoftwareDataInterchangeCodes => "0121",
            ICD::BbDataGmbh => "0122",
            ICD::BasfCompanyAtmNetwork => "0123",
            ICD::IotaIdentifiersForOrganizationsForTelecommunicationsAddressingUsingIcdSystemFormatDefinedInIsoIec8348 => "0124",
            ICD::HenkelCorporateNetworkHNet => "0125",
            ICD::GteOsiNetwork => "0126",
            ICD::DresdnerBankCorporateNetwork => "0127",
            ICD::BcnrSwissClearingBankNumber => "0128",
            ICD::BpiSwissBusinessPartnerIdentificationCode => "0129",
            ICD::DirectoratesEuropeanCommission => "0130",
            ICD::CodeForIdentificationNationalOrganizations => "0131",
            ICD::CerticomObjectIdentifiers => "0132",
            ICD::Tc68Oid => "0133",
            ICD::InfonetServicesCorporation => "0134",
            ICD::SiaObjectIdentifiers => "0135",
            ICD::CableWirelessGlobalAtmEndSystemAddressPlan => "0136",
            ICD::GlobalAesaScheme => "0137",
            ICD::FranceTelecomAtmEndSystemAddressPlan => "0138",
            ICD::SavvisCommunicationsAesa => "0139",
            ICD::ToshibaOrganizationsPartnersAndSuppliersTopasCode => "0140",
            ICD::NatoCommercialAndGovernmentEntitySystem => "0141",
            ICD::SecetiObjectIdentifiers => "0142",
            ICD::EinesteinetAg => "0143",
            ICD::DodaacDepartmentDefenseActivityAddressCode => "0144",
            ICD::DgcpDirectionGénéraleDeLaComptabilitéPubliqueAdministrativeAccountingIdentificationScheme => "0145",
            ICD::DgiDirectionGénéraleDesImpotsCode => "0146",
            ICD::StandardCompanyCode => "0147",
            ICD::ItuInternationalTelecommunicationsUnionDataNetworkIdentificationCodesDnic => "0148",
            ICD::GlobalBusinessIdentifier => "0149",
            ICD::MadgeNetworksLtdIcdAtmAddressingScheme => "0150",
            ICD::AustralianBusinessNumberAbnScheme => "0151",
            ICD::EdiraSchemeIdentifierCode => "0152",
            ICD::ConcertGlobalNetworkServicesIcdAesa => "0153",
            ICD::IdentificationNumberEconomicSubjectsIco => "0154",
            ICD::GlobalCrossingAesaAtmEndSystemAddress => "0155",
            ICD::Auna => "0156",
            ICD::AtmInterconnectionWithDutchKpnTelecom => "0157",
            ICD::IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127 => "0158",
            ICD::ActalisObjectIdentifiers => "0159",
            ICD::GtinGlobalTradeItemNumber => "0160",
            ICD::EccmaOpenTechnicalDirectory => "0161",
            ICD::CenIsssObjectIdentifierScheme => "0162",
            ICD::UsEpaFacilityIdentifier => "0163",
            ICD::TelusCorporation => "0164",
            ICD::FieieObjectIdentifiers => "0165",
            ICD::SwissguideIdentifierScheme => "0166",
            ICD::PriorityTelecomAtmEndSystemAddressPlan => "0167",
            ICD::VodafoneIrelandOsiAddressing => "0168",
            ICD::SwissFederalBusinessIdentificationNumberCentralBusinessNamesIndexZefixIdentificationNumber => "0169",
            ICD::TeikokuCompanyCode => "0170",
            ICD::LuxembourgCpCpsCertificationPolicyAndCertificationPracticeStatementIndex => "0171",
            ICD::ProjectGroupListsPropertiesProlist => "0172",
            ICD::EciSs => "0173",
            ICD::Stepnexus => "0174",
            ICD::SiemensAg => "0175",
            ICD::ParadineGmbh => "0176",
            ICD::OdetteInternationalLimited => "0177",
            ICD::Route1Mobinet => "0178",
            ICD::PenangoObjectIdentifiers => "0179",
            ICD::LithuanianMilitaryPki => "0180",
            ICD::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb => "0183",
            ICD::Digstorg => "0184",
            ICD::PercevalObjectCode => "0185",
            ICD::TrustpointObjectIdentifiers => "0186",
            ICD::AmazonUniqueIdentificationScheme => "0187",
            ICD::CorporateNumberTheSocialSecurityAndTaxNumberSystem => "0188",
            ICD::EuropeanBusinessIdentifierEbid => "0189",
            ICD::OrganisatieIndentificatieNummerOin => "0190",
            ICD::CompanyCodeEstonia => "0191",
            ICD::Organisasjonsnummer => "0192",
            ICD::UblBePartyIdentifier => "0193",
            ICD::KoiosOpenTechnicalDictionary => "0194",
            ICD::SingaporeNationwideELnvoiceFramework => "0195",
            ICD::IcelandicIdentifierÍslenskKennitala => "0196",
            ICD::AppliaPlStandard => "0197",
            ICD::Erstorg => "0198",
            ICD::LegalEntityIdentifierLei => "0199",
            ICD::LegalEntityCodeLithuania => "0200",
            ICD::CodiceUnivocoUnitàOrganizzativaIpa => "0201",
            ICD::IndirizzoDiPostaElettronicaCertificata => "0202",
            ICD::EdeliveryNetworkParticipantIdentifier => "0203",
            ICD::LeitwegId => "0204",
            ICD::Coddest => "0205",
            ICD::RegistreDuCommerceEtDeLIndustrieRci => "0206",
            ICD::PilogOntologyCodificationIdentifierPoci => "0207",
            ICD::NumeroDentrepriseOndernemingsnummerUnternehmensnummer => "0208",
            ICD::Gs1IdentificationKeys => "0209",
            ICD::CodiceFiscale => "0210",
            ICD::PartitaIva => "0211",
            ICD::FinnishOrganizationIdentifier => "0212",
            ICD::FinnishOrganizationValueAddTaxIdentifier => "0213",
            ICD::TradeplaceTradepiStandard => "0214",
            ICD::NetServiceId => "0215",
            ICD::Ovtcode => "0216",
            ICD::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber => "0217",
            ICD::UnifiedRegistrationNumberLatvia => "0218",
            ICD::TaxpayerRegistrationCodeLatvia => "0219",
            ICD::TheRegisterNaturalPersonsLatvia => "0220",
            ICD::TheRegisteredNumberQualifiedInvoiceIssuer => "0221",
            ICD::MetadataRegistrySupport => "0222",
            ICD::EuBasedCompany => "0223",
            ICD::FtctcCodeRoutage => "0224",
            ICD::FrctcElectronicAddress => "0225",
            ICD::FrctcParticulier => "0226",
            ICD::NonEuBasedCompany => "0227",
            ICD::RépertoireDesEntreprisesEtDesEtablissementsRidet => "0228",
            ICD::TAHITITraitementAutomatiqueHiérarchiséDesInstitutionsDeTahitiEtDesÎles => "0229",
            ICD::NationalEInvoicingFramework => "0230",
            ICD::SingleTaxableCompanyFrance => "0231",
            ICD::NobbProductNumber => "0232",
            ICD::DescriptionNotKnown => "0233",
            ICD::ToimitusosoiteId => "0234",
            ICD::UaeTaxIdentificationNumberTin => "0235",
            ICD::DescriptionNotKnown_Dup => "0236",
            ICD::CprDanishPersonCivilRegistrationNumber => "0237",
            ICD::PlateformeSAgrééeSÀLaFacturationÉlectroniquePpfPdp => "0238",
            ICD::Eaeu => "0239",
            ICD::RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales => "0240",
        }
    }
}

impl crate::Description for ICD {
    fn description(self) -> &'static str {
        match self {
            ICD::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene => "System Information et Repertoire des Entreprise et des Etablissements: SIRENE ",
            ICD::CodificationNumeriqueDesEtablissmentsFinanciersEnBelgique => "Codification Numerique des Etablissments Financiers En Belgique ",
            ICD::NbsOsiNetwork => "NBS/OSI NETWORK ",
            ICD::UsaFedGovOsiNetwork => "USA FED GOV OSI NETWORK ",
            ICD::UsaDodOsiNetwork => "USA DOD OSI NETWORK ",
            ICD::Organisationsnummer => "Organisationsnummer ",
            ICD::LeNumeroNational => "LE NUMERO NATIONAL ",
            ICD::SiretCode => "SIRET-CODE ",
            ICD::OrganizationalIdentifiersForStructuredNamesUnderIso9541Part2 => "Organizational Identifiers for Structured Names under ISO 9541 Part 2 ",
            ICD::InternationalCodeDesignatorForIdentificationOsiBasedAmateurRadioOrganizationsNetworkObjectsAndApplicationServices => "International Code Designator for the Identification of OSI-based, Amateur Radio Organizations, Network Objects and Application Services. ",
            ICD::EuropeanComputerManufacturersAssociationEcma => "European Computer Manufacturers Association: ECMA ",
            ICD::VsaFtpCodeFtpFileTransferProtocol => "VSA FTP CODE (FTP = File Transfer Protocol) ",
            ICD::NistOsiImplememtsWorkshop => "NIST/OSI Implememts' Workshop ",
            ICD::ElectronicDataInterchangeEdi => "Electronic Data Interchange: EDI ",
            ICD::EwosObjectIdentifiers => "EWOS Object Identifiers ",
            ICD::CommonLanguage => "COMMON LANGUAGE ",
            ICD::SnaOsiNetwork => "SNA/OSI Network ",
            ICD::AirTransportIndustryServicesCommunicationsNetwork => "Air Transport Industry Services Communications Network ",
            ICD::EuropeanLaboratoryForParticlePhysicsCern => "European Laboratory for Particle Physics: CERN ",
            ICD::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT => "SOCIETY FOR WORLDWIDE INTERBANK FINANCIAL, TELECOMMUNICATION S.W.I.F.T. ",
            ICD::OsfDistributedComputingObjectIdentification => "OSF Distributed Computing Object Identification ",
            ICD::NordicUniversityAndResearchNetworkNordunet => "Nordic University and Research Network: NORDUnet ",
            ICD::DigitalEquipmentCorporationDec => "Digital Equipment Corporation: DEC ",
            ICD::OsiAsiaOceaniaWorkshop => "OSI ASIA-OCEANIA WORKSHOP ",
            ICD::NatoIso6523IcdeCodingScheme => "NATO ISO 6523 ICDE coding scheme ",
            ICD::AeronauticalTelecommunicationsNetworkAtn => "Aeronautical Telecommunications Network (ATN) ",
            ICD::InternationalStandardIso6523 => "International Standard ISO 6523 ",
            ICD::TheAllUnionClassifierEnterprisesAndOrganisations => "The All-Union Classifier of Enterprises and Organisations ",
            ICD::AtTOsiNetwork => "AT&T/OSI Network ",
            ICD::EdiPartnerIdentificationCode => "EDI Partner Identification Code ",
            ICD::TelecomAustralia => "Telecom Australia ",
            ICD::SGWOsiInternetwork => "S G W OSI Internetwork ",
            ICD::ReuterOpenAddressStandard => "Reuter Open Address Standard ",
            ICD::Iso6523Icd => "ISO 6523 - ICD ",
            ICD::TeletrustObjectIdentifiers => "TeleTrust Object Identifiers ",
            ICD::LyTunnus => "LY-tunnus ",
            ICD::TheAustralianGosipNetwork => "The Australian GOSIP Network ",
            ICD::TheOzDodOsiNetwork => "The OZ DOD OSI Network ",
            ICD::UnileverGroupCompanies => "Unilever Group Companies ",
            ICD::CiticorpGlobalInformationNetwork => "Citicorp Global Information Network ",
            ICD::DbpTelekomObjectIdentifiers => "DBP Telekom Object Identifiers ",
            ICD::Hydronett => "HydroNETT ",
            ICD::ThaiIndustrialStandardsInstituteTisi => "Thai Industrial Standards Institute (TISI) ",
            ICD::IciCompanyIdentificationSystem => "ICI Company Identification System ",
            ICD::Funloc => "FUNLOC ",
            ICD::BullOdiDsaUnixNetwork => "BULL ODI/DSA/UNIX Network ",
            ICD::Osinz => "OSINZ ",
            ICD::AucklandAreaHealth => "Auckland Area Health ",
            ICD::Firmenich => "Firmenich ",
            ICD::AgfaDis => "AGFA-DIS ",
            ICD::SocietyMotionPictureAndTelevisionEngineersSmpte => "Society of Motion Picture and Television Engineers (SMPTE) ",
            ICD::Migros_networkM_netopz => "Migros_Network M_NETOPZ ",
            ICD::Iso6523Icdpcr => "ISO6523 - ICDPCR ",
            ICD::EnergyNet => "Energy Net ",
            ICD::NokiaObjectIdentifiersNoi => "Nokia Object Identifiers (NOI) ",
            ICD::SaintGobain => "Saint Gobain ",
            ICD::SiemensCorporateNetwork => "Siemens Corporate Network ",
            ICD::Danznet => "DANZNET ",
            ICD::DataUniversalNumberingSystemDUNSNumber => "Data Universal Numbering System (D-U-N-S Number) ",
            ICD::SoffexOsi => "SOFFEX OSI ",
            ICD::KpnOvn => "KPN OVN ",
            ICD::Ascomosinet => "ascomOSINet ",
            ICD::UtcUniformeTransportCode => "UTC: Uniforme Transport Code ",
            ICD::SolvayOsiCoding => "SOLVAY OSI CODING ",
            ICD::RocheCorporateNetwork => "Roche Corporate Network ",
            ICD::Zellwegerosinet => "ZellwegerOSINet ",
            ICD::IntelCorporationOsi => "Intel Corporation OSI ",
            ICD::SitaObjectIdentifierTree => "SITA Object Identifier Tree ",
            ICD::DaimlerchryslerCorporateNetwork => "DaimlerChrysler Corporate Network ",
            ICD::LegoOsiNetwork => "LEGO /OSI NETWORK ",
            ICD::NavistarOsiNetwork => "NAVISTAR/OSI Network ",
            ICD::IcdFormattedAtmAddress => "ICD Formatted ATM address ",
            ICD::Arinc => "ARINC ",
            ICD::AlcanetAlcatelAlsthomCorporateNetwork => "Alcanet/Alcatel-Alsthom Corporate Network ",
            ICD::SistemaItalianoDiIdentificazioneDiOgettiGestitoDaUninfo => "Sistema Italiano di Identificazione di ogetti gestito da UNINFO ",
            ICD::SistemaItalianoDiIndirizzamentoDiRetiOsiGestitoDaUninfo => "Sistema Italiano di Indirizzamento di Reti OSI Gestito da UNINFO ",
            ICD::MitelTerminalOrSwitchingEquipment => "Mitel terminal or switching equipment ",
            ICD::AtmForum => "ATM Forum ",
            ICD::UkNationalHealthServiceSchemeEdiraCompliant => "UK National Health Service Scheme, (EDIRA compliant) ",
            ICD::InternationalNsap => "International NSAP ",
            ICD::NorwegianTelecommunicationsAuthoritysNtasEdiIdentifierSchemeEdiraCompliant => "Norwegian Telecommunications Authority's, NTA'S, EDI, identifier scheme (EDIRA compliant) ",
            ICD::AdvancedTelecommunicationsModulesLimitedCorporateNetwork => "Advanced Telecommunications Modules Limited, Corporate Network ",
            ICD::AthensChamberCommerceIndustrySchemeEdiraCompliant => "Athens Chamber of Commerce & Industry Scheme (EDIRA compliant) ",
            ICD::SwissChambersCommerceSchemeEdiraCompliant => "Swiss Chambers of Commerce Scheme (EDIRA) compliant ",
            ICD::UnitedStatesCouncilForInternationalBusinessUscibSchemeEdiraCompliant => "United States Council for International Business (USCIB) Scheme, (EDIRA compliant) ",
            ICD::NationalFederationChambersCommerceIndustryBelgiumSchemeEdiraCompliant => "National Federation of Chambers of Commerce & Industry of Belgium, Scheme (EDIRA compliant) ",
            ICD::EanLocationCode => "EAN Location Code ",
            ICD::TheAssociationBritishChambersCommerceLtdSchemeEdiraCompliant => "The Association of British Chambers of Commerce Ltd. Scheme, (EDIRA compliant) ",
            ICD::InternetIpAddressingIso6523IcdEncoding => "Internet IP addressing - ISO 6523 ICD encoding ",
            ICD::CiscoSysytemsOsiNetwork => "Cisco Sysytems / OSI Network ",
            ICD::RevenueCanadaBusinessNumberRegistrationEdiraCompliant => "Revenue Canada Business Number Registration (EDIRA compliant) ",
            ICD::DeutscherIndustrieUndHandelstagDihtSchemeEdiraCompliant => "DEUTSCHER INDUSTRIE- UND HANDELSTAG (DIHT) Scheme (EDIRA compliant) ",
            ICD::HewlettPackardCompanyInternalAmNetwork => "Hewlett - Packard Company Internal AM Network ",
            ICD::DanishChamberOfCommerceSchemeEdiraCompliant => "DANISH CHAMBER OF COMMERCE Scheme (EDIRA compliant) ",
            ICD::FtiEdiforumItaliaEdiraCompliant => "FTI - Ediforum Italia, (EDIRA compliant) ",
            ICD::ChamberOfCommerceTelAvivJaffaSchemeEdiraCompliant => "CHAMBER OF COMMERCE TEL AVIV-JAFFA Scheme (EDIRA compliant) ",
            ICD::SiemensSupervisorySystemsNetwork => "Siemens Supervisory Systems Network ",
            ICD::Png_icdScheme => "PNG_ICD Scheme ",
            ICD::SouthAfricanCodeAllocation => "South African Code Allocation ",
            ICD::Heag => "HEAG ",
            ICD::BtIcdCodingSystem => "BT - ICD Coding System ",
            ICD::PortugueseChamberCommerceAndIndustrySchemeEdiraCompliant => "Portuguese Chamber of Commerce and Industry Scheme (EDIRA compliant) ",
            ICD::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant => "Vereniging van Kamers van Koophandel en Fabrieken in Nederland (Association of Chambers of Commerce and Industry in the Netherlands), Scheme (EDIRA compliant) ",
            ICD::AssociationSwedishChambersCommerceAndIndustrySchemeEdiraCompliant => "Association of Swedish Chambers of Commerce and Industry Scheme (EDIRA compliant) ",
            ICD::AustralianChambersCommerceAndIndustrySchemeEdiraCompliant => "Australian Chambers of Commerce and Industry Scheme (EDIRA compliant) ",
            ICD::BellsouthIcdAesaAtmEndSystemAddress => "BellSouth ICD AESA (ATM End System Address) ",
            ICD::BellAtlantic => "Bell Atlantic ",
            ICD::ObjectIdentifiers => "Object Identifiers ",
            ICD::IsoRegisterForStandardsProducingOrganizations => "ISO register for Standards producing Organizations ",
            ICD::Originnet => "OriginNet ",
            ICD::CheckPointSoftwareTechnologies => "Check Point Software Technologies ",
            ICD::PacificBellDataCommunicationsNetwork => "Pacific Bell Data Communications Network ",
            ICD::PssObjectIdentifiers => "PSS Object Identifiers ",
            ICD::StentorIcdCodingSystem => "STENTOR-ICD CODING SYSTEM ",
            ICD::AtmNetworkZn96 => "ATM-Network ZN'96 ",
            ICD::MciOsiNetwork => "MCI / OSI Network ",
            ICD::Advantis => "Advantis ",
            ICD::AffableSoftwareDataInterchangeCodes => "Affable Software Data Interchange Codes ",
            ICD::BbDataGmbh => "BB-DATA GmbH ",
            ICD::BasfCompanyAtmNetwork => "BASF Company ATM-Network ",
            ICD::IotaIdentifiersForOrganizationsForTelecommunicationsAddressingUsingIcdSystemFormatDefinedInIsoIec8348 => "IOTA Identifiers for Organizations for Telecommunications Addressing using the ICD system format defined in ISO/IEC 8348 ",
            ICD::HenkelCorporateNetworkHNet => "Henkel Corporate Network (H-Net) ",
            ICD::GteOsiNetwork => "GTE/OSI Network ",
            ICD::DresdnerBankCorporateNetwork => "Dresdner Bank Corporate Network ",
            ICD::BcnrSwissClearingBankNumber => "BCNR (Swiss Clearing Bank Number) ",
            ICD::BpiSwissBusinessPartnerIdentificationCode => "BPI (Swiss Business Partner Identification) code ",
            ICD::DirectoratesEuropeanCommission => "Directorates of the European Commission ",
            ICD::CodeForIdentificationNationalOrganizations => "Code for the Identification of National Organizations ",
            ICD::CerticomObjectIdentifiers => "Certicom Object Identifiers ",
            ICD::Tc68Oid => "TC68 OID ",
            ICD::InfonetServicesCorporation => "Infonet Services Corporation ",
            ICD::SiaObjectIdentifiers => "SIA Object Identifiers ",
            ICD::CableWirelessGlobalAtmEndSystemAddressPlan => "Cable & Wireless Global ATM End-System Address Plan ",
            ICD::GlobalAesaScheme => "Global AESA scheme ",
            ICD::FranceTelecomAtmEndSystemAddressPlan => "France Telecom ATM End System Address Plan ",
            ICD::SavvisCommunicationsAesa => "Savvis Communications AESA:. ",
            ICD::ToshibaOrganizationsPartnersAndSuppliersTopasCode => "Toshiba Organizations, Partners, And Suppliers' (TOPAS) Code ",
            ICD::NatoCommercialAndGovernmentEntitySystem => "NATO Commercial and Government Entity system ",
            ICD::SecetiObjectIdentifiers => "SECETI Object Identifiers ",
            ICD::EinesteinetAg => "EINESTEINet AG ",
            ICD::DodaacDepartmentDefenseActivityAddressCode => "DoDAAC (Department of Defense Activity Address Code) ",
            ICD::DgcpDirectionGénéraleDeLaComptabilitéPubliqueAdministrativeAccountingIdentificationScheme => "DGCP (Direction Générale de la Comptabilité Publique)administrative accounting identification scheme ",
            ICD::DgiDirectionGénéraleDesImpotsCode => "DGI (Direction Générale des Impots) code ",
            ICD::StandardCompanyCode => "Standard Company Code ",
            ICD::ItuInternationalTelecommunicationsUnionDataNetworkIdentificationCodesDnic => "ITU (International Telecommunications Union)Data Network Identification Codes (DNIC) ",
            ICD::GlobalBusinessIdentifier => "Global Business Identifier ",
            ICD::MadgeNetworksLtdIcdAtmAddressingScheme => "Madge Networks Ltd- ICD ATM Addressing Scheme ",
            ICD::AustralianBusinessNumberAbnScheme => "Australian Business Number (ABN) Scheme ",
            ICD::EdiraSchemeIdentifierCode => "Edira Scheme Identifier Code ",
            ICD::ConcertGlobalNetworkServicesIcdAesa => "Concert Global Network Services ICD AESA ",
            ICD::IdentificationNumberEconomicSubjectsIco => "Identification number of economic subjects: (ICO) ",
            ICD::GlobalCrossingAesaAtmEndSystemAddress => "Global Crossing AESA (ATM End System Address) ",
            ICD::Auna => "AUNA ",
            ICD::AtmInterconnectionWithDutchKpnTelecom => "ATM interconnection with the Dutch KPN Telecom ",
            ICD::IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127 => "Identification number of economic subject (ICO) Act on State Statistics of 29 November 2'001, § 27 ",
            ICD::ActalisObjectIdentifiers => "ACTALIS Object Identifiers ",
            ICD::GtinGlobalTradeItemNumber => "GTIN - Global Trade Item Number ",
            ICD::EccmaOpenTechnicalDirectory => "ECCMA Open Technical Directory ",
            ICD::CenIsssObjectIdentifierScheme => "CEN/ISSS Object Identifier Scheme ",
            ICD::UsEpaFacilityIdentifier => "US-EPA Facility Identifier ",
            ICD::TelusCorporation => "TELUS Corporation ",
            ICD::FieieObjectIdentifiers => "FIEIE Object identifiers ",
            ICD::SwissguideIdentifierScheme => "Swissguide Identifier Scheme ",
            ICD::PriorityTelecomAtmEndSystemAddressPlan => "Priority Telecom ATM End System Address Plan ",
            ICD::VodafoneIrelandOsiAddressing => "Vodafone Ireland OSI Addressing ",
            ICD::SwissFederalBusinessIdentificationNumberCentralBusinessNamesIndexZefixIdentificationNumber => "Swiss Federal Business Identification Number. Central Business names Index (zefix) Identification Number ",
            ICD::TeikokuCompanyCode => "Teikoku Company Code ",
            ICD::LuxembourgCpCpsCertificationPolicyAndCertificationPracticeStatementIndex => "Luxembourg CP & CPS (Certification Policy and Certification Practice Statement) Index ",
            ICD::ProjectGroupListsPropertiesProlist => "Project Group “Lists of Properties” (PROLIST®) ",
            ICD::EciSs => "eCI@ss ",
            ICD::Stepnexus => "StepNexus ",
            ICD::SiemensAg => "Siemens AG ",
            ICD::ParadineGmbh => "Paradine GmbH ",
            ICD::OdetteInternationalLimited => "Odette International Limited ",
            ICD::Route1Mobinet => "Route1 MobiNET ",
            ICD::PenangoObjectIdentifiers => "Penango Object Identifiers ",
            ICD::LithuanianMilitaryPki => "Lithuanian military PKI ",
            ICD::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb => "Numéro d'identification suisse des enterprises (IDE), Swiss Unique Business Identification Number (UIDB) ",
            ICD::Digstorg => "DIGSTORG ",
            ICD::PercevalObjectCode => "Perceval Object Code ",
            ICD::TrustpointObjectIdentifiers => "TrustPoint Object Identifiers ",
            ICD::AmazonUniqueIdentificationScheme => "Amazon Unique Identification Scheme ",
            ICD::CorporateNumberTheSocialSecurityAndTaxNumberSystem => "Corporate Number of The Social Security and Tax Number System ",
            ICD::EuropeanBusinessIdentifierEbid => "European Business Identifier (EBID) ",
            ICD::OrganisatieIndentificatieNummerOin => "Organisatie Indentificatie Nummer (OIN) ",
            ICD::CompanyCodeEstonia => "Company Code (Estonia) ",
            ICD::Organisasjonsnummer => "Organisasjonsnummer ",
            ICD::UblBePartyIdentifier => "UBL.BE Party Identifier ",
            ICD::KoiosOpenTechnicalDictionary => "KOIOS Open Technical Dictionary",
            ICD::SingaporeNationwideELnvoiceFramework => "Singapore Nationwide E-lnvoice Framework ",
            ICD::IcelandicIdentifierÍslenskKennitala => "Icelandic identifier - Íslensk kennitala ",
            ICD::AppliaPlStandard => "APPLiA Pl Standard",
            ICD::Erstorg => "ERSTORG",
            ICD::LegalEntityIdentifierLei => "Legal Entity Identifier (LEI)",
            ICD::LegalEntityCodeLithuania => "Legal entity code (Lithuania)",
            ICD::CodiceUnivocoUnitàOrganizzativaIpa => "Codice Univoco Unità Organizzativa iPA",
            ICD::IndirizzoDiPostaElettronicaCertificata => "Indirizzo di Posta Elettronica Certificata",
            ICD::EdeliveryNetworkParticipantIdentifier => "eDelivery Network Participant identifier",
            ICD::LeitwegId => "Leitweg-ID",
            ICD::Coddest => "CODDEST",
            ICD::RegistreDuCommerceEtDeLIndustrieRci => "Registre du Commerce et de l’Industrie : RCI",
            ICD::PilogOntologyCodificationIdentifierPoci => "PiLog Ontology Codification Identifier (POCI)",
            ICD::NumeroDentrepriseOndernemingsnummerUnternehmensnummer => "Numero d'entreprise / ondernemingsnummer / Unternehmensnummer",
            ICD::Gs1IdentificationKeys => "GS1 identification keys",
            ICD::CodiceFiscale => "CODICE FISCALE",
            ICD::PartitaIva => "PARTITA IVA",
            ICD::FinnishOrganizationIdentifier => "Finnish Organization Identifier",
            ICD::FinnishOrganizationValueAddTaxIdentifier => "Finnish Organization Value Add Tax Identifier",
            ICD::TradeplaceTradepiStandard => "Tradeplace TradePI Standard",
            ICD::NetServiceId => "Net service ID",
            ICD::Ovtcode => "OVTcode",
            ICD::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber => "The Netherlands Chamber of Commerce and Industry establishment number",
            ICD::UnifiedRegistrationNumberLatvia => "Unified registration number (Latvia)",
            ICD::TaxpayerRegistrationCodeLatvia => "Taxpayer registration code (Latvia)",
            ICD::TheRegisterNaturalPersonsLatvia => "The Register of Natural Persons (Latvia)",
            ICD::TheRegisteredNumberQualifiedInvoiceIssuer => "The registered number of the qualified invoice issuer",
            ICD::MetadataRegistrySupport => "Metadata Registry Support",
            ICD::EuBasedCompany => "EU based company",
            ICD::FtctcCodeRoutage => "FTCTC CODE ROUTAGE",
            ICD::FrctcElectronicAddress => "FRCTC ELECTRONIC ADDRESS",
            ICD::FrctcParticulier => "FRCTC Particulier",
            ICD::NonEuBasedCompany => "NON - EU based company",
            ICD::RépertoireDesEntreprisesEtDesEtablissementsRidet => "Répertoire des Entreprises et des Etablissements (RIDET)",
            ICD::TAHITITraitementAutomatiqueHiérarchiséDesInstitutionsDeTahitiEtDesÎles => "T.A.H.I.T.I (traitement automatique hiérarchisé des institutions de Tahiti et des îles)",
            ICD::NationalEInvoicingFramework => "National e-Invoicing Framework",
            ICD::SingleTaxableCompanyFrance => "Single taxable company (France)",
            ICD::NobbProductNumber => "NOBB product number",
            ICD::DescriptionNotKnown => "Description not known",
            ICD::ToimitusosoiteId => "Toimitusosoite ID",
            ICD::UaeTaxIdentificationNumberTin => "UAE Tax Identification Number (TIN)",
            ICD::DescriptionNotKnown_Dup => "Description not known",
            ICD::CprDanishPersonCivilRegistrationNumber => "CPR (Danish person civil registration number)",
            ICD::PlateformeSAgrééeSÀLaFacturationÉlectroniquePpfPdp => "Plateforme.s agréée.s à la facturation électronique (PPF/PDP)",
            ICD::Eaeu => "EAEU",
            ICD::RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales => "Register of legal persons (in French : Répertoire des personnes morales)",
        }
    }
}

impl crate::FromCode for ICD {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "0002" => Some(ICD::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene),
            "0003" => Some(ICD::CodificationNumeriqueDesEtablissmentsFinanciersEnBelgique),
            "0004" => Some(ICD::NbsOsiNetwork),
            "0005" => Some(ICD::UsaFedGovOsiNetwork),
            "0006" => Some(ICD::UsaDodOsiNetwork),
            "0007" => Some(ICD::Organisationsnummer),
            "0008" => Some(ICD::LeNumeroNational),
            "0009" => Some(ICD::SiretCode),
            "0010" => Some(ICD::OrganizationalIdentifiersForStructuredNamesUnderIso9541Part2),
            "0011" => Some(ICD::InternationalCodeDesignatorForIdentificationOsiBasedAmateurRadioOrganizationsNetworkObjectsAndApplicationServices),
            "0012" => Some(ICD::EuropeanComputerManufacturersAssociationEcma),
            "0013" => Some(ICD::VsaFtpCodeFtpFileTransferProtocol),
            "0014" => Some(ICD::NistOsiImplememtsWorkshop),
            "0015" => Some(ICD::ElectronicDataInterchangeEdi),
            "0016" => Some(ICD::EwosObjectIdentifiers),
            "0017" => Some(ICD::CommonLanguage),
            "0018" => Some(ICD::SnaOsiNetwork),
            "0019" => Some(ICD::AirTransportIndustryServicesCommunicationsNetwork),
            "0020" => Some(ICD::EuropeanLaboratoryForParticlePhysicsCern),
            "0021" => Some(ICD::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT),
            "0022" => Some(ICD::OsfDistributedComputingObjectIdentification),
            "0023" => Some(ICD::NordicUniversityAndResearchNetworkNordunet),
            "0024" => Some(ICD::DigitalEquipmentCorporationDec),
            "0025" => Some(ICD::OsiAsiaOceaniaWorkshop),
            "0026" => Some(ICD::NatoIso6523IcdeCodingScheme),
            "0027" => Some(ICD::AeronauticalTelecommunicationsNetworkAtn),
            "0028" => Some(ICD::InternationalStandardIso6523),
            "0029" => Some(ICD::TheAllUnionClassifierEnterprisesAndOrganisations),
            "0030" => Some(ICD::AtTOsiNetwork),
            "0031" => Some(ICD::EdiPartnerIdentificationCode),
            "0032" => Some(ICD::TelecomAustralia),
            "0033" => Some(ICD::SGWOsiInternetwork),
            "0034" => Some(ICD::ReuterOpenAddressStandard),
            "0035" => Some(ICD::Iso6523Icd),
            "0036" => Some(ICD::TeletrustObjectIdentifiers),
            "0037" => Some(ICD::LyTunnus),
            "0038" => Some(ICD::TheAustralianGosipNetwork),
            "0039" => Some(ICD::TheOzDodOsiNetwork),
            "0040" => Some(ICD::UnileverGroupCompanies),
            "0041" => Some(ICD::CiticorpGlobalInformationNetwork),
            "0042" => Some(ICD::DbpTelekomObjectIdentifiers),
            "0043" => Some(ICD::Hydronett),
            "0044" => Some(ICD::ThaiIndustrialStandardsInstituteTisi),
            "0045" => Some(ICD::IciCompanyIdentificationSystem),
            "0046" => Some(ICD::Funloc),
            "0047" => Some(ICD::BullOdiDsaUnixNetwork),
            "0048" => Some(ICD::Osinz),
            "0049" => Some(ICD::AucklandAreaHealth),
            "0050" => Some(ICD::Firmenich),
            "0051" => Some(ICD::AgfaDis),
            "0052" => Some(ICD::SocietyMotionPictureAndTelevisionEngineersSmpte),
            "0053" => Some(ICD::Migros_networkM_netopz),
            "0054" => Some(ICD::Iso6523Icdpcr),
            "0055" => Some(ICD::EnergyNet),
            "0056" => Some(ICD::NokiaObjectIdentifiersNoi),
            "0057" => Some(ICD::SaintGobain),
            "0058" => Some(ICD::SiemensCorporateNetwork),
            "0059" => Some(ICD::Danznet),
            "0060" => Some(ICD::DataUniversalNumberingSystemDUNSNumber),
            "0061" => Some(ICD::SoffexOsi),
            "0062" => Some(ICD::KpnOvn),
            "0063" => Some(ICD::Ascomosinet),
            "0064" => Some(ICD::UtcUniformeTransportCode),
            "0065" => Some(ICD::SolvayOsiCoding),
            "0066" => Some(ICD::RocheCorporateNetwork),
            "0067" => Some(ICD::Zellwegerosinet),
            "0068" => Some(ICD::IntelCorporationOsi),
            "0069" => Some(ICD::SitaObjectIdentifierTree),
            "0070" => Some(ICD::DaimlerchryslerCorporateNetwork),
            "0071" => Some(ICD::LegoOsiNetwork),
            "0072" => Some(ICD::NavistarOsiNetwork),
            "0073" => Some(ICD::IcdFormattedAtmAddress),
            "0074" => Some(ICD::Arinc),
            "0075" => Some(ICD::AlcanetAlcatelAlsthomCorporateNetwork),
            "0076" => Some(ICD::SistemaItalianoDiIdentificazioneDiOgettiGestitoDaUninfo),
            "0077" => Some(ICD::SistemaItalianoDiIndirizzamentoDiRetiOsiGestitoDaUninfo),
            "0078" => Some(ICD::MitelTerminalOrSwitchingEquipment),
            "0079" => Some(ICD::AtmForum),
            "0080" => Some(ICD::UkNationalHealthServiceSchemeEdiraCompliant),
            "0081" => Some(ICD::InternationalNsap),
            "0082" => Some(ICD::NorwegianTelecommunicationsAuthoritysNtasEdiIdentifierSchemeEdiraCompliant),
            "0083" => Some(ICD::AdvancedTelecommunicationsModulesLimitedCorporateNetwork),
            "0084" => Some(ICD::AthensChamberCommerceIndustrySchemeEdiraCompliant),
            "0085" => Some(ICD::SwissChambersCommerceSchemeEdiraCompliant),
            "0086" => Some(ICD::UnitedStatesCouncilForInternationalBusinessUscibSchemeEdiraCompliant),
            "0087" => Some(ICD::NationalFederationChambersCommerceIndustryBelgiumSchemeEdiraCompliant),
            "0088" => Some(ICD::EanLocationCode),
            "0089" => Some(ICD::TheAssociationBritishChambersCommerceLtdSchemeEdiraCompliant),
            "0090" => Some(ICD::InternetIpAddressingIso6523IcdEncoding),
            "0091" => Some(ICD::CiscoSysytemsOsiNetwork),
            "0093" => Some(ICD::RevenueCanadaBusinessNumberRegistrationEdiraCompliant),
            "0094" => Some(ICD::DeutscherIndustrieUndHandelstagDihtSchemeEdiraCompliant),
            "0095" => Some(ICD::HewlettPackardCompanyInternalAmNetwork),
            "0096" => Some(ICD::DanishChamberOfCommerceSchemeEdiraCompliant),
            "0097" => Some(ICD::FtiEdiforumItaliaEdiraCompliant),
            "0098" => Some(ICD::ChamberOfCommerceTelAvivJaffaSchemeEdiraCompliant),
            "0099" => Some(ICD::SiemensSupervisorySystemsNetwork),
            "0100" => Some(ICD::Png_icdScheme),
            "0101" => Some(ICD::SouthAfricanCodeAllocation),
            "0102" => Some(ICD::Heag),
            "0104" => Some(ICD::BtIcdCodingSystem),
            "0105" => Some(ICD::PortugueseChamberCommerceAndIndustrySchemeEdiraCompliant),
            "0106" => Some(ICD::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant),
            "0107" => Some(ICD::AssociationSwedishChambersCommerceAndIndustrySchemeEdiraCompliant),
            "0108" => Some(ICD::AustralianChambersCommerceAndIndustrySchemeEdiraCompliant),
            "0109" => Some(ICD::BellsouthIcdAesaAtmEndSystemAddress),
            "0110" => Some(ICD::BellAtlantic),
            "0111" => Some(ICD::ObjectIdentifiers),
            "0112" => Some(ICD::IsoRegisterForStandardsProducingOrganizations),
            "0113" => Some(ICD::Originnet),
            "0114" => Some(ICD::CheckPointSoftwareTechnologies),
            "0115" => Some(ICD::PacificBellDataCommunicationsNetwork),
            "0116" => Some(ICD::PssObjectIdentifiers),
            "0117" => Some(ICD::StentorIcdCodingSystem),
            "0118" => Some(ICD::AtmNetworkZn96),
            "0119" => Some(ICD::MciOsiNetwork),
            "0120" => Some(ICD::Advantis),
            "0121" => Some(ICD::AffableSoftwareDataInterchangeCodes),
            "0122" => Some(ICD::BbDataGmbh),
            "0123" => Some(ICD::BasfCompanyAtmNetwork),
            "0124" => Some(ICD::IotaIdentifiersForOrganizationsForTelecommunicationsAddressingUsingIcdSystemFormatDefinedInIsoIec8348),
            "0125" => Some(ICD::HenkelCorporateNetworkHNet),
            "0126" => Some(ICD::GteOsiNetwork),
            "0127" => Some(ICD::DresdnerBankCorporateNetwork),
            "0128" => Some(ICD::BcnrSwissClearingBankNumber),
            "0129" => Some(ICD::BpiSwissBusinessPartnerIdentificationCode),
            "0130" => Some(ICD::DirectoratesEuropeanCommission),
            "0131" => Some(ICD::CodeForIdentificationNationalOrganizations),
            "0132" => Some(ICD::CerticomObjectIdentifiers),
            "0133" => Some(ICD::Tc68Oid),
            "0134" => Some(ICD::InfonetServicesCorporation),
            "0135" => Some(ICD::SiaObjectIdentifiers),
            "0136" => Some(ICD::CableWirelessGlobalAtmEndSystemAddressPlan),
            "0137" => Some(ICD::GlobalAesaScheme),
            "0138" => Some(ICD::FranceTelecomAtmEndSystemAddressPlan),
            "0139" => Some(ICD::SavvisCommunicationsAesa),
            "0140" => Some(ICD::ToshibaOrganizationsPartnersAndSuppliersTopasCode),
            "0141" => Some(ICD::NatoCommercialAndGovernmentEntitySystem),
            "0142" => Some(ICD::SecetiObjectIdentifiers),
            "0143" => Some(ICD::EinesteinetAg),
            "0144" => Some(ICD::DodaacDepartmentDefenseActivityAddressCode),
            "0145" => Some(ICD::DgcpDirectionGénéraleDeLaComptabilitéPubliqueAdministrativeAccountingIdentificationScheme),
            "0146" => Some(ICD::DgiDirectionGénéraleDesImpotsCode),
            "0147" => Some(ICD::StandardCompanyCode),
            "0148" => Some(ICD::ItuInternationalTelecommunicationsUnionDataNetworkIdentificationCodesDnic),
            "0149" => Some(ICD::GlobalBusinessIdentifier),
            "0150" => Some(ICD::MadgeNetworksLtdIcdAtmAddressingScheme),
            "0151" => Some(ICD::AustralianBusinessNumberAbnScheme),
            "0152" => Some(ICD::EdiraSchemeIdentifierCode),
            "0153" => Some(ICD::ConcertGlobalNetworkServicesIcdAesa),
            "0154" => Some(ICD::IdentificationNumberEconomicSubjectsIco),
            "0155" => Some(ICD::GlobalCrossingAesaAtmEndSystemAddress),
            "0156" => Some(ICD::Auna),
            "0157" => Some(ICD::AtmInterconnectionWithDutchKpnTelecom),
            "0158" => Some(ICD::IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127),
            "0159" => Some(ICD::ActalisObjectIdentifiers),
            "0160" => Some(ICD::GtinGlobalTradeItemNumber),
            "0161" => Some(ICD::EccmaOpenTechnicalDirectory),
            "0162" => Some(ICD::CenIsssObjectIdentifierScheme),
            "0163" => Some(ICD::UsEpaFacilityIdentifier),
            "0164" => Some(ICD::TelusCorporation),
            "0165" => Some(ICD::FieieObjectIdentifiers),
            "0166" => Some(ICD::SwissguideIdentifierScheme),
            "0167" => Some(ICD::PriorityTelecomAtmEndSystemAddressPlan),
            "0168" => Some(ICD::VodafoneIrelandOsiAddressing),
            "0169" => Some(ICD::SwissFederalBusinessIdentificationNumberCentralBusinessNamesIndexZefixIdentificationNumber),
            "0170" => Some(ICD::TeikokuCompanyCode),
            "0171" => Some(ICD::LuxembourgCpCpsCertificationPolicyAndCertificationPracticeStatementIndex),
            "0172" => Some(ICD::ProjectGroupListsPropertiesProlist),
            "0173" => Some(ICD::EciSs),
            "0174" => Some(ICD::Stepnexus),
            "0175" => Some(ICD::SiemensAg),
            "0176" => Some(ICD::ParadineGmbh),
            "0177" => Some(ICD::OdetteInternationalLimited),
            "0178" => Some(ICD::Route1Mobinet),
            "0179" => Some(ICD::PenangoObjectIdentifiers),
            "0180" => Some(ICD::LithuanianMilitaryPki),
            "0183" => Some(ICD::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb),
            "0184" => Some(ICD::Digstorg),
            "0185" => Some(ICD::PercevalObjectCode),
            "0186" => Some(ICD::TrustpointObjectIdentifiers),
            "0187" => Some(ICD::AmazonUniqueIdentificationScheme),
            "0188" => Some(ICD::CorporateNumberTheSocialSecurityAndTaxNumberSystem),
            "0189" => Some(ICD::EuropeanBusinessIdentifierEbid),
            "0190" => Some(ICD::OrganisatieIndentificatieNummerOin),
            "0191" => Some(ICD::CompanyCodeEstonia),
            "0192" => Some(ICD::Organisasjonsnummer),
            "0193" => Some(ICD::UblBePartyIdentifier),
            "0194" => Some(ICD::KoiosOpenTechnicalDictionary),
            "0195" => Some(ICD::SingaporeNationwideELnvoiceFramework),
            "0196" => Some(ICD::IcelandicIdentifierÍslenskKennitala),
            "0197" => Some(ICD::AppliaPlStandard),
            "0198" => Some(ICD::Erstorg),
            "0199" => Some(ICD::LegalEntityIdentifierLei),
            "0200" => Some(ICD::LegalEntityCodeLithuania),
            "0201" => Some(ICD::CodiceUnivocoUnitàOrganizzativaIpa),
            "0202" => Some(ICD::IndirizzoDiPostaElettronicaCertificata),
            "0203" => Some(ICD::EdeliveryNetworkParticipantIdentifier),
            "0204" => Some(ICD::LeitwegId),
            "0205" => Some(ICD::Coddest),
            "0206" => Some(ICD::RegistreDuCommerceEtDeLIndustrieRci),
            "0207" => Some(ICD::PilogOntologyCodificationIdentifierPoci),
            "0208" => Some(ICD::NumeroDentrepriseOndernemingsnummerUnternehmensnummer),
            "0209" => Some(ICD::Gs1IdentificationKeys),
            "0210" => Some(ICD::CodiceFiscale),
            "0211" => Some(ICD::PartitaIva),
            "0212" => Some(ICD::FinnishOrganizationIdentifier),
            "0213" => Some(ICD::FinnishOrganizationValueAddTaxIdentifier),
            "0214" => Some(ICD::TradeplaceTradepiStandard),
            "0215" => Some(ICD::NetServiceId),
            "0216" => Some(ICD::Ovtcode),
            "0217" => Some(ICD::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber),
            "0218" => Some(ICD::UnifiedRegistrationNumberLatvia),
            "0219" => Some(ICD::TaxpayerRegistrationCodeLatvia),
            "0220" => Some(ICD::TheRegisterNaturalPersonsLatvia),
            "0221" => Some(ICD::TheRegisteredNumberQualifiedInvoiceIssuer),
            "0222" => Some(ICD::MetadataRegistrySupport),
            "0223" => Some(ICD::EuBasedCompany),
            "0224" => Some(ICD::FtctcCodeRoutage),
            "0225" => Some(ICD::FrctcElectronicAddress),
            "0226" => Some(ICD::FrctcParticulier),
            "0227" => Some(ICD::NonEuBasedCompany),
            "0228" => Some(ICD::RépertoireDesEntreprisesEtDesEtablissementsRidet),
            "0229" => Some(ICD::TAHITITraitementAutomatiqueHiérarchiséDesInstitutionsDeTahitiEtDesÎles),
            "0230" => Some(ICD::NationalEInvoicingFramework),
            "0231" => Some(ICD::SingleTaxableCompanyFrance),
            "0232" => Some(ICD::NobbProductNumber),
            "0233" => Some(ICD::DescriptionNotKnown),
            "0234" => Some(ICD::ToimitusosoiteId),
            "0235" => Some(ICD::UaeTaxIdentificationNumberTin),
            "0236" => Some(ICD::DescriptionNotKnown_Dup),
            "0237" => Some(ICD::CprDanishPersonCivilRegistrationNumber),
            "0238" => Some(ICD::PlateformeSAgrééeSÀLaFacturationÉlectroniquePpfPdp),
            "0239" => Some(ICD::Eaeu),
            "0240" => Some(ICD::RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales),
            _ => None,
        }
    }
}
