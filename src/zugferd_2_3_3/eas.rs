#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum EAS {
    /// System Information et Repertoire des Entreprise et des Etablissements: SIRENE
    SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene,
    /// Organisationsnummer
    Organisationsnummer,
    /// SIRET-CODE
    SiretCode,
    /// LY-tunnus
    LyTunnus,
    /// Data Universal Numbering System (D-U-N-S Number)
    DataUniversalNumberingSystemDUNSNumber,
    /// EAN Location Code
    EanLocationCode,
    /// The Danish Business Authority - P-number (DK:P)
    TheDanishBusinessAuthorityPNumberDkP,
    /// FTI - Ediforum Italia, (EDIRA compliant)
    FtiEdiforumItaliaEdiraCompliant,
    /// Vereniging van Kamers van Koophandel en Fabrieken in Nederland (Association of Chambers of Commerce and Industry in the Netherlands), Scheme (EDIRA compliant)
    VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant,
    /// Directorates of the European Commission
    DirectoratesEuropeanCommission,
    /// SIA Object Identifiers
    SiaObjectIdentifiers,
    /// SECETI Object Identifiers
    SecetiObjectIdentifiers,
    /// Standard Company Code
    StandardCompanyCode,
    /// Australian Business Number (ABN) Scheme
    AustralianBusinessNumberAbnScheme,
    /// Identification number of economic subjects: (ICO)
    IdentificationNumberEconomicSubjectsIco,
    /// Identification number of economic subject (ICO) Act on State Statistics of 29 November 2001, § 27
    IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127,
    /// Teikoku Company Code
    TeikokuCompanyCode,
    /// Odette International Limited
    OdetteInternationalLimited,
    /// Numéro d'identification suisse des enterprises (IDE), Swiss Unique Business Identification Number (UIDB)
    NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb,
    /// DIGSTORG
    Digstorg,
    /// Corporate Number of The Social Security and Tax Number System
    CorporateNumberTheSocialSecurityAndTaxNumberSystem,
    /// Dutch Originator's Identification Number
    DutchOriginatorsIdentificationNumber,
    /// Centre of Registers and Information Systems of the Ministry of Justice
    CentreRegistersAndInformationSystemsMinistryJustice,
    /// Enhetsregisteret ved Bronnoysundregisterne
    EnhetsregisteretVedBronnoysundregisterne,
    /// UBL.BE party identifier
    UblBePartyIdentifier,
    /// KOIOS Open Technical Dictionary
    KoiosOpenTechnicalDictionary,
    /// Singapore UEN identifier
    SingaporeUenIdentifier,
    /// Kennitala - Iceland legal id for individuals and legal entities
    KennitalaIcelandLegalIdForIndividualsAndLegalEntities,
    /// ERSTORG
    Erstorg,
    /// Global legal entity identifier (GLEIF)
    GlobalLegalEntityIdentifierGleif,
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
    /// Net service ID
    NetServiceId,
    /// OVTcode
    Ovtcode,
    /// The Netherlands Chamber of Commerce and Industry establishment number
    TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber,
    /// Unified registration number (Latvia)
    UnifiedRegistrationNumberLatvia,
    ///  The registered number of the qualified invoice issuer
    TheRegisteredNumberQualifiedInvoiceIssuer,
    ///  FRCTC ELECTRONIC ADDRESS
    FrctcElectronicAddress,
    ///  National e-Invoicing Framework
    NationalEInvoicingFramework,
    /// UAE Tax Identification Number (TIN)
    UaeTaxIdentificationNumberTin,
    /// Register of legal persons (in French : Répertoire des personnes morales)
    RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales,
    /// Hungary VAT number
    HungaryVatNumber,
    /// Business Registers Network
    BusinessRegistersNetwork,
    /// Österreichische Umsatzsteuer-Identifikationsnummer
    ÖsterreichischeUmsatzsteuerIdentifikationsnummer,
    /// Österreichisches Verwaltungs bzw. Organisationskennzeichen
    ÖsterreichischesVerwaltungsBzwOrganisationskennzeichen,
    /// SOCIETY FOR WORLDWIDE INTERBANK FINANCIAL, TELECOMMUNICATION S.W.I.F.T
    SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT,
    /// Kennziffer des Unternehmensregisters
    KennzifferDesUnternehmensregisters,
    /// Agencia Española de Administración Tributaria
    AgenciaEspañolaDeAdministraciónTributaria,
    /// Andorra VAT number
    AndorraVatNumber,
    /// Albania VAT number
    AlbaniaVatNumber,
    /// Bosnia and Herzegovina VAT number
    BosniaAndHerzegovinaVatNumber,
    /// Belgium VAT number
    BelgiumVatNumber,
    /// Bulgaria VAT number
    BulgariaVatNumber,
    /// Switzerland VAT number
    SwitzerlandVatNumber,
    /// Cyprus VAT number
    CyprusVatNumber,
    /// Czech Republic VAT number
    CzechRepublicVatNumber,
    /// Germany VAT number
    GermanyVatNumber,
    /// Estonia VAT number
    EstoniaVatNumber,
    /// United Kingdom VAT number
    UnitedKingdomVatNumber,
    /// Greece VAT number
    GreeceVatNumber,
    /// Croatia VAT number
    CroatiaVatNumber,
    /// Ireland VAT number
    IrelandVatNumber,
    /// Liechtenstein VAT number
    LiechtensteinVatNumber,
    /// Lithuania VAT number
    LithuaniaVatNumber,
    /// Luxemburg VAT number
    LuxemburgVatNumber,
    /// Latvia VAT number
    LatviaVatNumber,
    /// Monaco VAT number
    MonacoVatNumber,
    /// Montenegro VAT number
    MontenegroVatNumber,
    /// Macedonia, the former Yugoslav Republic of VAT number
    MacedoniaFormerYugoslavRepublicVatNumber,
    /// Malta VAT number
    MaltaVatNumber,
    /// Netherlands VAT number
    NetherlandsVatNumber,
    /// Poland VAT number
    PolandVatNumber,
    /// Portugal VAT number
    PortugalVatNumber,
    /// Romania VAT number
    RomaniaVatNumber,
    /// Serbia VAT number
    SerbiaVatNumber,
    /// Slovenia VAT number
    SloveniaVatNumber,
    /// Slovakia VAT number
    SlovakiaVatNumber,
    /// San Marino VAT number
    SanMarinoVatNumber,
    /// Turkey VAT number
    TurkeyVatNumber,
    /// Holy See (Vatican City State) VAT number
    HolySeeVaticanCityStateVatNumber,
    /// French VAT number
    FrenchVatNumber,
    /// Employer Identification Number (EIN, USA)
    EmployerIdentificationNumberEinUsa,
    /// O.F.T.P. (ODETTE File Transfer Protocol)
    OFTPOdetteFileTransferProtocol,
    /// X.400 address for mail text
    X400AddressForMailText,
    /// AS2 exchange
    As2Exchange,
    /// File Transfer Protocol
    FileTransferProtocol,
    /// Electronic mail (SMPT)
    ElectronicMailSmpt,
}

impl std::fmt::Display for EAS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for EAS {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s).ok_or(())
    }
}

impl crate::Code for EAS {
    fn code(self) -> &'static str {
        match self {
            EAS::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene => "0002",
            EAS::Organisationsnummer => "0007",
            EAS::SiretCode => "0009",
            EAS::LyTunnus => "0037",
            EAS::DataUniversalNumberingSystemDUNSNumber => "0060",
            EAS::EanLocationCode => "0088",
            EAS::TheDanishBusinessAuthorityPNumberDkP => "0096",
            EAS::FtiEdiforumItaliaEdiraCompliant => "0097",
            EAS::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant => "0106",
            EAS::DirectoratesEuropeanCommission => "0130",
            EAS::SiaObjectIdentifiers => "0135",
            EAS::SecetiObjectIdentifiers => "0142",
            EAS::StandardCompanyCode => "0147",
            EAS::AustralianBusinessNumberAbnScheme => "0151",
            EAS::IdentificationNumberEconomicSubjectsIco => "0154",
            EAS::IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127 => "0158",
            EAS::TeikokuCompanyCode => "0170",
            EAS::OdetteInternationalLimited => "0177",
            EAS::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb => "0183",
            EAS::Digstorg => "0184",
            EAS::CorporateNumberTheSocialSecurityAndTaxNumberSystem => "0188",
            EAS::DutchOriginatorsIdentificationNumber => "0190",
            EAS::CentreRegistersAndInformationSystemsMinistryJustice => "0191",
            EAS::EnhetsregisteretVedBronnoysundregisterne => "0192",
            EAS::UblBePartyIdentifier => "0193",
            EAS::KoiosOpenTechnicalDictionary => "0194",
            EAS::SingaporeUenIdentifier => "0195",
            EAS::KennitalaIcelandLegalIdForIndividualsAndLegalEntities => "0196",
            EAS::Erstorg => "0198",
            EAS::GlobalLegalEntityIdentifierGleif => "0199",
            EAS::LegalEntityCodeLithuania => "0200",
            EAS::CodiceUnivocoUnitàOrganizzativaIpa => "0201",
            EAS::IndirizzoDiPostaElettronicaCertificata => "0202",
            EAS::EdeliveryNetworkParticipantIdentifier => "0203",
            EAS::LeitwegId => "0204",
            EAS::Coddest => "0205",
            EAS::NumeroDentrepriseOndernemingsnummerUnternehmensnummer => "0208",
            EAS::Gs1IdentificationKeys => "0209",
            EAS::CodiceFiscale => "0210",
            EAS::PartitaIva => "0211",
            EAS::FinnishOrganizationIdentifier => "0212",
            EAS::FinnishOrganizationValueAddTaxIdentifier => "0213",
            EAS::NetServiceId => "0215",
            EAS::Ovtcode => "0216",
            EAS::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber => "0217",
            EAS::UnifiedRegistrationNumberLatvia => "0218",
            EAS::TheRegisteredNumberQualifiedInvoiceIssuer => "0221",
            EAS::FrctcElectronicAddress => "0225",
            EAS::NationalEInvoicingFramework => "0230",
            EAS::UaeTaxIdentificationNumberTin => "0235",
            EAS::RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales => "0240",
            EAS::HungaryVatNumber => "9910",
            EAS::BusinessRegistersNetwork => "9913",
            EAS::ÖsterreichischeUmsatzsteuerIdentifikationsnummer => "9914",
            EAS::ÖsterreichischesVerwaltungsBzwOrganisationskennzeichen => "9915",
            EAS::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT => "9918",
            EAS::KennzifferDesUnternehmensregisters => "9919",
            EAS::AgenciaEspañolaDeAdministraciónTributaria => "9920",
            EAS::AndorraVatNumber => "9922",
            EAS::AlbaniaVatNumber => "9923",
            EAS::BosniaAndHerzegovinaVatNumber => "9924",
            EAS::BelgiumVatNumber => "9925",
            EAS::BulgariaVatNumber => "9926",
            EAS::SwitzerlandVatNumber => "9927",
            EAS::CyprusVatNumber => "9928",
            EAS::CzechRepublicVatNumber => "9929",
            EAS::GermanyVatNumber => "9930",
            EAS::EstoniaVatNumber => "9931",
            EAS::UnitedKingdomVatNumber => "9932",
            EAS::GreeceVatNumber => "9933",
            EAS::CroatiaVatNumber => "9934",
            EAS::IrelandVatNumber => "9935",
            EAS::LiechtensteinVatNumber => "9936",
            EAS::LithuaniaVatNumber => "9937",
            EAS::LuxemburgVatNumber => "9938",
            EAS::LatviaVatNumber => "9939",
            EAS::MonacoVatNumber => "9940",
            EAS::MontenegroVatNumber => "9941",
            EAS::MacedoniaFormerYugoslavRepublicVatNumber => "9942",
            EAS::MaltaVatNumber => "9943",
            EAS::NetherlandsVatNumber => "9944",
            EAS::PolandVatNumber => "9945",
            EAS::PortugalVatNumber => "9946",
            EAS::RomaniaVatNumber => "9947",
            EAS::SerbiaVatNumber => "9948",
            EAS::SloveniaVatNumber => "9949",
            EAS::SlovakiaVatNumber => "9950",
            EAS::SanMarinoVatNumber => "9951",
            EAS::TurkeyVatNumber => "9952",
            EAS::HolySeeVaticanCityStateVatNumber => "9953",
            EAS::FrenchVatNumber => "9957",
            EAS::EmployerIdentificationNumberEinUsa => "9959",
            EAS::OFTPOdetteFileTransferProtocol => "AN",
            EAS::X400AddressForMailText => "AQ ",
            EAS::As2Exchange => "AS ",
            EAS::FileTransferProtocol => "AU ",
            EAS::ElectronicMailSmpt => "EM",
        }
    }
}

impl crate::Description for EAS {
    fn description(self) -> &'static str {
        match self {
            EAS::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene => "System Information et Repertoire des Entreprise et des Etablissements: SIRENE",
            EAS::Organisationsnummer => "Organisationsnummer",
            EAS::SiretCode => "SIRET-CODE",
            EAS::LyTunnus => "LY-tunnus",
            EAS::DataUniversalNumberingSystemDUNSNumber => "Data Universal Numbering System (D-U-N-S Number)",
            EAS::EanLocationCode => "EAN Location Code",
            EAS::TheDanishBusinessAuthorityPNumberDkP => "The Danish Business Authority - P-number (DK:P)",
            EAS::FtiEdiforumItaliaEdiraCompliant => "FTI - Ediforum Italia, (EDIRA compliant)",
            EAS::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant => "Vereniging van Kamers van Koophandel en Fabrieken in Nederland (Association of Chambers of Commerce and Industry in the Netherlands), Scheme (EDIRA compliant)",
            EAS::DirectoratesEuropeanCommission => "Directorates of the European Commission",
            EAS::SiaObjectIdentifiers => "SIA Object Identifiers",
            EAS::SecetiObjectIdentifiers => "SECETI Object Identifiers",
            EAS::StandardCompanyCode => "Standard Company Code",
            EAS::AustralianBusinessNumberAbnScheme => "Australian Business Number (ABN) Scheme",
            EAS::IdentificationNumberEconomicSubjectsIco => "Identification number of economic subjects: (ICO)",
            EAS::IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127 => "Identification number of economic subject (ICO) Act on State Statistics of 29 November 2001, § 27",
            EAS::TeikokuCompanyCode => "Teikoku Company Code",
            EAS::OdetteInternationalLimited => "Odette International Limited ",
            EAS::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb => "Numéro d'identification suisse des enterprises (IDE), Swiss Unique Business Identification Number (UIDB) ",
            EAS::Digstorg => "DIGSTORG",
            EAS::CorporateNumberTheSocialSecurityAndTaxNumberSystem => "Corporate Number of The Social Security and Tax Number System",
            EAS::DutchOriginatorsIdentificationNumber => "Dutch Originator's Identification Number",
            EAS::CentreRegistersAndInformationSystemsMinistryJustice => "Centre of Registers and Information Systems of the Ministry of Justice",
            EAS::EnhetsregisteretVedBronnoysundregisterne => "Enhetsregisteret ved Bronnoysundregisterne ",
            EAS::UblBePartyIdentifier => "UBL.BE party identifier",
            EAS::KoiosOpenTechnicalDictionary => "KOIOS Open Technical Dictionary",
            EAS::SingaporeUenIdentifier => "Singapore UEN identifier",
            EAS::KennitalaIcelandLegalIdForIndividualsAndLegalEntities => "Kennitala - Iceland legal id for individuals and legal entities",
            EAS::Erstorg => "ERSTORG",
            EAS::GlobalLegalEntityIdentifierGleif => "Global legal entity identifier (GLEIF)",
            EAS::LegalEntityCodeLithuania => "Legal entity code (Lithuania)",
            EAS::CodiceUnivocoUnitàOrganizzativaIpa => "Codice Univoco Unità Organizzativa iPA",
            EAS::IndirizzoDiPostaElettronicaCertificata => "Indirizzo di Posta Elettronica Certificata",
            EAS::EdeliveryNetworkParticipantIdentifier => "eDelivery Network Participant identifier",
            EAS::LeitwegId => "Leitweg-ID",
            EAS::Coddest => "CODDEST",
            EAS::NumeroDentrepriseOndernemingsnummerUnternehmensnummer => "Numero d'entreprise / ondernemingsnummer / Unternehmensnummer",
            EAS::Gs1IdentificationKeys => "GS1 identification keys",
            EAS::CodiceFiscale => "CODICE FISCALE",
            EAS::PartitaIva => "PARTITA IVA",
            EAS::FinnishOrganizationIdentifier => "Finnish Organization Identifier",
            EAS::FinnishOrganizationValueAddTaxIdentifier => "Finnish Organization Value Add Tax Identifier",
            EAS::NetServiceId => "Net service ID",
            EAS::Ovtcode => "OVTcode",
            EAS::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber => "The Netherlands Chamber of Commerce and Industry establishment number",
            EAS::UnifiedRegistrationNumberLatvia => "Unified registration number (Latvia)",
            EAS::TheRegisteredNumberQualifiedInvoiceIssuer => " The registered number of the qualified invoice issuer",
            EAS::FrctcElectronicAddress => " FRCTC ELECTRONIC ADDRESS",
            EAS::NationalEInvoicingFramework => " National e-Invoicing Framework",
            EAS::UaeTaxIdentificationNumberTin => "UAE Tax Identification Number (TIN)",
            EAS::RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales => "Register of legal persons (in French : Répertoire des personnes morales)",
            EAS::HungaryVatNumber => "Hungary VAT number",
            EAS::BusinessRegistersNetwork => "Business Registers Network ",
            EAS::ÖsterreichischeUmsatzsteuerIdentifikationsnummer => "Österreichische Umsatzsteuer-Identifikationsnummer ",
            EAS::ÖsterreichischesVerwaltungsBzwOrganisationskennzeichen => "Österreichisches Verwaltungs bzw. Organisationskennzeichen",
            EAS::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT => "SOCIETY FOR WORLDWIDE INTERBANK FINANCIAL, TELECOMMUNICATION S.W.I.F.T",
            EAS::KennzifferDesUnternehmensregisters => "Kennziffer des Unternehmensregisters ",
            EAS::AgenciaEspañolaDeAdministraciónTributaria => "Agencia Española de Administración Tributaria ",
            EAS::AndorraVatNumber => "Andorra VAT number",
            EAS::AlbaniaVatNumber => "Albania VAT number",
            EAS::BosniaAndHerzegovinaVatNumber => "Bosnia and Herzegovina VAT number",
            EAS::BelgiumVatNumber => "Belgium VAT number",
            EAS::BulgariaVatNumber => "Bulgaria VAT number",
            EAS::SwitzerlandVatNumber => "Switzerland VAT number",
            EAS::CyprusVatNumber => "Cyprus VAT number",
            EAS::CzechRepublicVatNumber => "Czech Republic VAT number",
            EAS::GermanyVatNumber => "Germany VAT number",
            EAS::EstoniaVatNumber => "Estonia VAT number",
            EAS::UnitedKingdomVatNumber => "United Kingdom VAT number",
            EAS::GreeceVatNumber => "Greece VAT number",
            EAS::CroatiaVatNumber => "Croatia VAT number",
            EAS::IrelandVatNumber => "Ireland VAT number",
            EAS::LiechtensteinVatNumber => "Liechtenstein VAT number",
            EAS::LithuaniaVatNumber => "Lithuania VAT number",
            EAS::LuxemburgVatNumber => "Luxemburg VAT number",
            EAS::LatviaVatNumber => "Latvia VAT number",
            EAS::MonacoVatNumber => "Monaco VAT number",
            EAS::MontenegroVatNumber => "Montenegro VAT number",
            EAS::MacedoniaFormerYugoslavRepublicVatNumber => "Macedonia, the former Yugoslav Republic of VAT number",
            EAS::MaltaVatNumber => "Malta VAT number",
            EAS::NetherlandsVatNumber => "Netherlands VAT number",
            EAS::PolandVatNumber => "Poland VAT number",
            EAS::PortugalVatNumber => "Portugal VAT number",
            EAS::RomaniaVatNumber => "Romania VAT number",
            EAS::SerbiaVatNumber => "Serbia VAT number",
            EAS::SloveniaVatNumber => "Slovenia VAT number",
            EAS::SlovakiaVatNumber => "Slovakia VAT number",
            EAS::SanMarinoVatNumber => "San Marino VAT number",
            EAS::TurkeyVatNumber => "Turkey VAT number",
            EAS::HolySeeVaticanCityStateVatNumber => "Holy See (Vatican City State) VAT number",
            EAS::FrenchVatNumber => "French VAT number",
            EAS::EmployerIdentificationNumberEinUsa => "Employer Identification Number (EIN, USA)",
            EAS::OFTPOdetteFileTransferProtocol => "O.F.T.P. (ODETTE File Transfer Protocol)",
            EAS::X400AddressForMailText => "X.400 address for mail text",
            EAS::As2Exchange => "AS2 exchange ",
            EAS::FileTransferProtocol => "File Transfer Protocol",
            EAS::ElectronicMailSmpt => "Electronic mail (SMPT)",
        }
    }
}

impl crate::FromCode for EAS {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "0002" => Some(EAS::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene),
            "0007" => Some(EAS::Organisationsnummer),
            "0009" => Some(EAS::SiretCode),
            "0037" => Some(EAS::LyTunnus),
            "0060" => Some(EAS::DataUniversalNumberingSystemDUNSNumber),
            "0088" => Some(EAS::EanLocationCode),
            "0096" => Some(EAS::TheDanishBusinessAuthorityPNumberDkP),
            "0097" => Some(EAS::FtiEdiforumItaliaEdiraCompliant),
            "0106" => Some(EAS::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant),
            "0130" => Some(EAS::DirectoratesEuropeanCommission),
            "0135" => Some(EAS::SiaObjectIdentifiers),
            "0142" => Some(EAS::SecetiObjectIdentifiers),
            "0147" => Some(EAS::StandardCompanyCode),
            "0151" => Some(EAS::AustralianBusinessNumberAbnScheme),
            "0154" => Some(EAS::IdentificationNumberEconomicSubjectsIco),
            "0158" => Some(EAS::IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127),
            "0170" => Some(EAS::TeikokuCompanyCode),
            "0177" => Some(EAS::OdetteInternationalLimited),
            "0183" => Some(EAS::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb),
            "0184" => Some(EAS::Digstorg),
            "0188" => Some(EAS::CorporateNumberTheSocialSecurityAndTaxNumberSystem),
            "0190" => Some(EAS::DutchOriginatorsIdentificationNumber),
            "0191" => Some(EAS::CentreRegistersAndInformationSystemsMinistryJustice),
            "0192" => Some(EAS::EnhetsregisteretVedBronnoysundregisterne),
            "0193" => Some(EAS::UblBePartyIdentifier),
            "0194" => Some(EAS::KoiosOpenTechnicalDictionary),
            "0195" => Some(EAS::SingaporeUenIdentifier),
            "0196" => Some(EAS::KennitalaIcelandLegalIdForIndividualsAndLegalEntities),
            "0198" => Some(EAS::Erstorg),
            "0199" => Some(EAS::GlobalLegalEntityIdentifierGleif),
            "0200" => Some(EAS::LegalEntityCodeLithuania),
            "0201" => Some(EAS::CodiceUnivocoUnitàOrganizzativaIpa),
            "0202" => Some(EAS::IndirizzoDiPostaElettronicaCertificata),
            "0203" => Some(EAS::EdeliveryNetworkParticipantIdentifier),
            "0204" => Some(EAS::LeitwegId),
            "0205" => Some(EAS::Coddest),
            "0208" => Some(EAS::NumeroDentrepriseOndernemingsnummerUnternehmensnummer),
            "0209" => Some(EAS::Gs1IdentificationKeys),
            "0210" => Some(EAS::CodiceFiscale),
            "0211" => Some(EAS::PartitaIva),
            "0212" => Some(EAS::FinnishOrganizationIdentifier),
            "0213" => Some(EAS::FinnishOrganizationValueAddTaxIdentifier),
            "0215" => Some(EAS::NetServiceId),
            "0216" => Some(EAS::Ovtcode),
            "0217" => Some(EAS::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber),
            "0218" => Some(EAS::UnifiedRegistrationNumberLatvia),
            "0221" => Some(EAS::TheRegisteredNumberQualifiedInvoiceIssuer),
            "0225" => Some(EAS::FrctcElectronicAddress),
            "0230" => Some(EAS::NationalEInvoicingFramework),
            "0235" => Some(EAS::UaeTaxIdentificationNumberTin),
            "0240" => Some(EAS::RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales),
            "9910" => Some(EAS::HungaryVatNumber),
            "9913" => Some(EAS::BusinessRegistersNetwork),
            "9914" => Some(EAS::ÖsterreichischeUmsatzsteuerIdentifikationsnummer),
            "9915" => Some(EAS::ÖsterreichischesVerwaltungsBzwOrganisationskennzeichen),
            "9918" => Some(EAS::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT),
            "9919" => Some(EAS::KennzifferDesUnternehmensregisters),
            "9920" => Some(EAS::AgenciaEspañolaDeAdministraciónTributaria),
            "9922" => Some(EAS::AndorraVatNumber),
            "9923" => Some(EAS::AlbaniaVatNumber),
            "9924" => Some(EAS::BosniaAndHerzegovinaVatNumber),
            "9925" => Some(EAS::BelgiumVatNumber),
            "9926" => Some(EAS::BulgariaVatNumber),
            "9927" => Some(EAS::SwitzerlandVatNumber),
            "9928" => Some(EAS::CyprusVatNumber),
            "9929" => Some(EAS::CzechRepublicVatNumber),
            "9930" => Some(EAS::GermanyVatNumber),
            "9931" => Some(EAS::EstoniaVatNumber),
            "9932" => Some(EAS::UnitedKingdomVatNumber),
            "9933" => Some(EAS::GreeceVatNumber),
            "9934" => Some(EAS::CroatiaVatNumber),
            "9935" => Some(EAS::IrelandVatNumber),
            "9936" => Some(EAS::LiechtensteinVatNumber),
            "9937" => Some(EAS::LithuaniaVatNumber),
            "9938" => Some(EAS::LuxemburgVatNumber),
            "9939" => Some(EAS::LatviaVatNumber),
            "9940" => Some(EAS::MonacoVatNumber),
            "9941" => Some(EAS::MontenegroVatNumber),
            "9942" => Some(EAS::MacedoniaFormerYugoslavRepublicVatNumber),
            "9943" => Some(EAS::MaltaVatNumber),
            "9944" => Some(EAS::NetherlandsVatNumber),
            "9945" => Some(EAS::PolandVatNumber),
            "9946" => Some(EAS::PortugalVatNumber),
            "9947" => Some(EAS::RomaniaVatNumber),
            "9948" => Some(EAS::SerbiaVatNumber),
            "9949" => Some(EAS::SloveniaVatNumber),
            "9950" => Some(EAS::SlovakiaVatNumber),
            "9951" => Some(EAS::SanMarinoVatNumber),
            "9952" => Some(EAS::TurkeyVatNumber),
            "9953" => Some(EAS::HolySeeVaticanCityStateVatNumber),
            "9957" => Some(EAS::FrenchVatNumber),
            "9959" => Some(EAS::EmployerIdentificationNumberEinUsa),
            "AN" => Some(EAS::OFTPOdetteFileTransferProtocol),
            "AQ " => Some(EAS::X400AddressForMailText),
            "AS " => Some(EAS::As2Exchange),
            "AU " => Some(EAS::FileTransferProtocol),
            "EM" => Some(EAS::ElectronicMailSmpt),
            _ => None,
        }
    }
}
