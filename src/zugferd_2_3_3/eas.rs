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
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
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

// Start: (Version) TryFrom EAS to crate::zugferd_2_3_2::EAS
impl std::convert::TryFrom<EAS> for crate::zugferd_2_3_2::EAS {
    type Error = ErrFromEasToCrateZugferd232Eas;
    fn try_from(value: EAS) -> Result<Self, Self::Error> {
        match value {
            EAS::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene => Ok(crate::zugferd_2_3_2::EAS::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene),
            EAS::Organisationsnummer => Ok(crate::zugferd_2_3_2::EAS::Organisationsnummer),
            EAS::SiretCode => Ok(crate::zugferd_2_3_2::EAS::SiretCode),
            EAS::LyTunnus => Ok(crate::zugferd_2_3_2::EAS::LyTunnus),
            EAS::DataUniversalNumberingSystemDUNSNumber => Ok(crate::zugferd_2_3_2::EAS::DataUniversalNumberingSystemDUNSNumber),
            EAS::EanLocationCode => Ok(crate::zugferd_2_3_2::EAS::EanLocationCode),
            EAS::TheDanishBusinessAuthorityPNumberDkP => Ok(crate::zugferd_2_3_2::EAS::DanishChamberOfCommerceSchemeEdiraCompliant),
            EAS::FtiEdiforumItaliaEdiraCompliant => Ok(crate::zugferd_2_3_2::EAS::FtiEdiforumItaliaEdiraCompliant),
            EAS::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant => Ok(crate::zugferd_2_3_2::EAS::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant),
            EAS::DirectoratesEuropeanCommission => Ok(crate::zugferd_2_3_2::EAS::DirectoratesEuropeanCommission),
            EAS::SiaObjectIdentifiers => Ok(crate::zugferd_2_3_2::EAS::SiaObjectIdentifiers),
            EAS::SecetiObjectIdentifiers => Ok(crate::zugferd_2_3_2::EAS::SecetiObjectIdentifiers),
            EAS::StandardCompanyCode => Ok(crate::zugferd_2_3_2::EAS::StandardCompanyCode),
            EAS::AustralianBusinessNumberAbnScheme => Ok(crate::zugferd_2_3_2::EAS::AustralianBusinessNumberAbnScheme),
            EAS::TeikokuCompanyCode => Ok(crate::zugferd_2_3_2::EAS::TeikokuCompanyCode),
            EAS::OdetteInternationalLimited => Ok(crate::zugferd_2_3_2::EAS::OdetteInternationalLimited),
            EAS::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb => Ok(crate::zugferd_2_3_2::EAS::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb),
            EAS::Digstorg => Ok(crate::zugferd_2_3_2::EAS::Digstorg),
            EAS::CorporateNumberTheSocialSecurityAndTaxNumberSystem => Ok(crate::zugferd_2_3_2::EAS::CorporateNumberTheSocialSecurityAndTaxNumberSystem),
            EAS::DutchOriginatorsIdentificationNumber => Ok(crate::zugferd_2_3_2::EAS::DutchOriginatorsIdentificationNumber),
            EAS::CentreRegistersAndInformationSystemsMinistryJustice => Ok(crate::zugferd_2_3_2::EAS::CentreRegistersAndInformationSystemsMinistryJustice),
            EAS::EnhetsregisteretVedBronnoysundregisterne => Ok(crate::zugferd_2_3_2::EAS::EnhetsregisteretVedBronnoysundregisterne),
            EAS::UblBePartyIdentifier => Ok(crate::zugferd_2_3_2::EAS::UblBePartyIdentifier),
            EAS::KoiosOpenTechnicalDictionary => Ok(crate::zugferd_2_3_2::EAS::KoiosOpenTechnicalDictionary),
            EAS::SingaporeUenIdentifier => Ok(crate::zugferd_2_3_2::EAS::SingaporeUenIdentifier),
            EAS::KennitalaIcelandLegalIdForIndividualsAndLegalEntities => Ok(crate::zugferd_2_3_2::EAS::KennitalaIcelandLegalIdForIndividualsAndLegalEntities),
            EAS::Erstorg => Ok(crate::zugferd_2_3_2::EAS::Erstorg),
            EAS::GlobalLegalEntityIdentifierGleif => Ok(crate::zugferd_2_3_2::EAS::GlobalLegalEntityIdentifierGleif),
            EAS::LegalEntityCodeLithuania => Ok(crate::zugferd_2_3_2::EAS::LegalEntityCodeLithuania),
            EAS::CodiceUnivocoUnitàOrganizzativaIpa => Ok(crate::zugferd_2_3_2::EAS::CodiceUnivocoUnitàOrganizzativaIpa),
            EAS::IndirizzoDiPostaElettronicaCertificata => Ok(crate::zugferd_2_3_2::EAS::IndirizzoDiPostaElettronicaCertificata),
            EAS::EdeliveryNetworkParticipantIdentifier => Ok(crate::zugferd_2_3_2::EAS::EdeliveryNetworkParticipantIdentifier),
            EAS::LeitwegId => Ok(crate::zugferd_2_3_2::EAS::LeitwegId),
            EAS::Coddest => Ok(crate::zugferd_2_3_2::EAS::Coddest),
            EAS::NumeroDentrepriseOndernemingsnummerUnternehmensnummer => Ok(crate::zugferd_2_3_2::EAS::NumeroDentrepriseOndernemingsnummerUnternehmensnummer),
            EAS::Gs1IdentificationKeys => Ok(crate::zugferd_2_3_2::EAS::Gs1IdentificationKeys),
            EAS::CodiceFiscale => Ok(crate::zugferd_2_3_2::EAS::CodiceFiscale),
            EAS::PartitaIva => Ok(crate::zugferd_2_3_2::EAS::PartitaIva),
            EAS::FinnishOrganizationIdentifier => Ok(crate::zugferd_2_3_2::EAS::FinnishOrganizationIdentifier),
            EAS::FinnishOrganizationValueAddTaxIdentifier => Ok(crate::zugferd_2_3_2::EAS::FinnishOrganizationValueAddTaxIdentifier),
            EAS::NetServiceId => Ok(crate::zugferd_2_3_2::EAS::NetServiceId),
            EAS::Ovtcode => Ok(crate::zugferd_2_3_2::EAS::Ovtcode),
            EAS::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber => Ok(crate::zugferd_2_3_2::EAS::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber),
            EAS::UnifiedRegistrationNumberLatvia => Ok(crate::zugferd_2_3_2::EAS::UnifiedRegistrationNumberLatvia),
            EAS::TheRegisteredNumberQualifiedInvoiceIssuer => Ok(crate::zugferd_2_3_2::EAS::TheRegisteredNumberQualifiedInvoiceIssuer),
            EAS::FrctcElectronicAddress => Ok(crate::zugferd_2_3_2::EAS::FrctcElectronicAddress),
            EAS::NationalEInvoicingFramework => Ok(crate::zugferd_2_3_2::EAS::NationalEInvoicingFramework),
            EAS::UaeTaxIdentificationNumberTin => Ok(crate::zugferd_2_3_2::EAS::UaeTaxIdentificationNumberTin),
            EAS::HungaryVatNumber => Ok(crate::zugferd_2_3_2::EAS::HungaryVatNumber),
            EAS::BusinessRegistersNetwork => Ok(crate::zugferd_2_3_2::EAS::BusinessRegistersNetwork),
            EAS::ÖsterreichischeUmsatzsteuerIdentifikationsnummer => Ok(crate::zugferd_2_3_2::EAS::ÖsterreichischeUmsatzsteuerIdentifikationsnummer),
            EAS::ÖsterreichischesVerwaltungsBzwOrganisationskennzeichen => Ok(crate::zugferd_2_3_2::EAS::ÖsterreichischesVerwaltungsBzwOrganisationskennzeichen),
            EAS::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT => Ok(crate::zugferd_2_3_2::EAS::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT),
            EAS::KennzifferDesUnternehmensregisters => Ok(crate::zugferd_2_3_2::EAS::KennzifferDesUnternehmensregisters),
            EAS::AgenciaEspañolaDeAdministraciónTributaria => Ok(crate::zugferd_2_3_2::EAS::AgenciaEspañolaDeAdministraciónTributaria),
            EAS::AndorraVatNumber => Ok(crate::zugferd_2_3_2::EAS::AndorraVatNumber),
            EAS::AlbaniaVatNumber => Ok(crate::zugferd_2_3_2::EAS::AlbaniaVatNumber),
            EAS::BosniaAndHerzegovinaVatNumber => Ok(crate::zugferd_2_3_2::EAS::BosniaAndHerzegovinaVatNumber),
            EAS::BelgiumVatNumber => Ok(crate::zugferd_2_3_2::EAS::BelgiumVatNumber),
            EAS::BulgariaVatNumber => Ok(crate::zugferd_2_3_2::EAS::BulgariaVatNumber),
            EAS::SwitzerlandVatNumber => Ok(crate::zugferd_2_3_2::EAS::SwitzerlandVatNumber),
            EAS::CyprusVatNumber => Ok(crate::zugferd_2_3_2::EAS::CyprusVatNumber),
            EAS::CzechRepublicVatNumber => Ok(crate::zugferd_2_3_2::EAS::CzechRepublicVatNumber),
            EAS::GermanyVatNumber => Ok(crate::zugferd_2_3_2::EAS::GermanyVatNumber),
            EAS::EstoniaVatNumber => Ok(crate::zugferd_2_3_2::EAS::EstoniaVatNumber),
            EAS::UnitedKingdomVatNumber => Ok(crate::zugferd_2_3_2::EAS::UnitedKingdomVatNumber),
            EAS::GreeceVatNumber => Ok(crate::zugferd_2_3_2::EAS::GreeceVatNumber),
            EAS::CroatiaVatNumber => Ok(crate::zugferd_2_3_2::EAS::CroatiaVatNumber),
            EAS::IrelandVatNumber => Ok(crate::zugferd_2_3_2::EAS::IrelandVatNumber),
            EAS::LiechtensteinVatNumber => Ok(crate::zugferd_2_3_2::EAS::LiechtensteinVatNumber),
            EAS::LithuaniaVatNumber => Ok(crate::zugferd_2_3_2::EAS::LithuaniaVatNumber),
            EAS::LuxemburgVatNumber => Ok(crate::zugferd_2_3_2::EAS::LuxemburgVatNumber),
            EAS::LatviaVatNumber => Ok(crate::zugferd_2_3_2::EAS::LatviaVatNumber),
            EAS::MonacoVatNumber => Ok(crate::zugferd_2_3_2::EAS::MonacoVatNumber),
            EAS::MontenegroVatNumber => Ok(crate::zugferd_2_3_2::EAS::MontenegroVatNumber),
            EAS::MacedoniaFormerYugoslavRepublicVatNumber => Ok(crate::zugferd_2_3_2::EAS::MacedoniaFormerYugoslavRepublicVatNumber),
            EAS::MaltaVatNumber => Ok(crate::zugferd_2_3_2::EAS::MaltaVatNumber),
            EAS::NetherlandsVatNumber => Ok(crate::zugferd_2_3_2::EAS::NetherlandsVatNumber),
            EAS::PolandVatNumber => Ok(crate::zugferd_2_3_2::EAS::PolandVatNumber),
            EAS::PortugalVatNumber => Ok(crate::zugferd_2_3_2::EAS::PortugalVatNumber),
            EAS::RomaniaVatNumber => Ok(crate::zugferd_2_3_2::EAS::RomaniaVatNumber),
            EAS::SerbiaVatNumber => Ok(crate::zugferd_2_3_2::EAS::SerbiaVatNumber),
            EAS::SloveniaVatNumber => Ok(crate::zugferd_2_3_2::EAS::SloveniaVatNumber),
            EAS::SlovakiaVatNumber => Ok(crate::zugferd_2_3_2::EAS::SlovakiaVatNumber),
            EAS::SanMarinoVatNumber => Ok(crate::zugferd_2_3_2::EAS::SanMarinoVatNumber),
            EAS::TurkeyVatNumber => Ok(crate::zugferd_2_3_2::EAS::TurkeyVatNumber),
            EAS::HolySeeVaticanCityStateVatNumber => Ok(crate::zugferd_2_3_2::EAS::HolySeeVaticanCityStateVatNumber),
            EAS::FrenchVatNumber => Ok(crate::zugferd_2_3_2::EAS::FrenchVatNumber),
            EAS::EmployerIdentificationNumberEinUsa => Ok(crate::zugferd_2_3_2::EAS::EmployerIdentificationNumberEinUsa),
            EAS::OFTPOdetteFileTransferProtocol => Ok(crate::zugferd_2_3_2::EAS::OFTPOdetteFileTransferProtocol),
            EAS::X400AddressForMailText => Ok(crate::zugferd_2_3_2::EAS::X400AddressForMailText),
            EAS::As2Exchange => Ok(crate::zugferd_2_3_2::EAS::As2Exchange),
            EAS::FileTransferProtocol => Ok(crate::zugferd_2_3_2::EAS::FileTransferProtocol),
            EAS::ElectronicMailSmpt => Ok(crate::zugferd_2_3_2::EAS::ElectronicMailSmpt),
            EAS::IdentificationNumberEconomicSubjectsIco => Err(ErrFromEasToCrateZugferd232Eas::IdentificationNumberEconomicSubjectsIco),
            EAS::IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127 => Err(ErrFromEasToCrateZugferd232Eas::IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127),
            EAS::RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales => Err(ErrFromEasToCrateZugferd232Eas::RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales),
        }
    }
}

/// All the variants of EAS that are not matched to any variant of crate::zugferd_2_3_2::EAS
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrFromEasToCrateZugferd232Eas {
    IdentificationNumberEconomicSubjectsIco,
    IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127,
    RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales,
}

impl std::fmt::Display for ErrFromEasToCrateZugferd232Eas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrFromEasToCrateZugferd232Eas::IdentificationNumberEconomicSubjectsIco => write!(f, "IdentificationNumberEconomicSubjectsIco has no corresponding value in crate::zugferd_2_3_2::EAS"),
            ErrFromEasToCrateZugferd232Eas::IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127 => write!(f, "IdentificationNumberEconomicSubjectIcoActOnStateStatistics29November200127 has no corresponding value in crate::zugferd_2_3_2::EAS"),
            ErrFromEasToCrateZugferd232Eas::RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales => write!(f, "RegisterLegalPersonsInFrenchRépertoireDesPersonnesMorales has no corresponding value in crate::zugferd_2_3_2::EAS"),
        }
    }
}

impl std::error::Error for ErrFromEasToCrateZugferd232Eas {}

impl std::convert::TryFrom<crate::zugferd_2_3_2::EAS> for EAS {
    type Error = ErrFromCrateZugferd232EasToEas;
    fn try_from(value: crate::zugferd_2_3_2::EAS) -> Result<EAS, Self::Error> {
        match value {
            crate::zugferd_2_3_2::EAS::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene => Ok(EAS::SystemInformationEtRepertoireDesEntrepriseEtDesEtablissementsSirene),
            crate::zugferd_2_3_2::EAS::Organisationsnummer => Ok(EAS::Organisationsnummer),
            crate::zugferd_2_3_2::EAS::SiretCode => Ok(EAS::SiretCode),
            crate::zugferd_2_3_2::EAS::LyTunnus => Ok(EAS::LyTunnus),
            crate::zugferd_2_3_2::EAS::DataUniversalNumberingSystemDUNSNumber => Ok(EAS::DataUniversalNumberingSystemDUNSNumber),
            crate::zugferd_2_3_2::EAS::EanLocationCode => Ok(EAS::EanLocationCode),
            crate::zugferd_2_3_2::EAS::DanishChamberOfCommerceSchemeEdiraCompliant => Ok(EAS::TheDanishBusinessAuthorityPNumberDkP),
            crate::zugferd_2_3_2::EAS::FtiEdiforumItaliaEdiraCompliant => Ok(EAS::FtiEdiforumItaliaEdiraCompliant),
            crate::zugferd_2_3_2::EAS::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant => Ok(EAS::VerenigingVanKamersVanKoophandelEnFabriekenInNederlandAssociationChambersCommerceAndIndustryInNetherlandsSchemeEdiraCompliant),
            crate::zugferd_2_3_2::EAS::DirectoratesEuropeanCommission => Ok(EAS::DirectoratesEuropeanCommission),
            crate::zugferd_2_3_2::EAS::SiaObjectIdentifiers => Ok(EAS::SiaObjectIdentifiers),
            crate::zugferd_2_3_2::EAS::SecetiObjectIdentifiers => Ok(EAS::SecetiObjectIdentifiers),
            crate::zugferd_2_3_2::EAS::StandardCompanyCode => Ok(EAS::StandardCompanyCode),
            crate::zugferd_2_3_2::EAS::AustralianBusinessNumberAbnScheme => Ok(EAS::AustralianBusinessNumberAbnScheme),
            crate::zugferd_2_3_2::EAS::TeikokuCompanyCode => Ok(EAS::TeikokuCompanyCode),
            crate::zugferd_2_3_2::EAS::OdetteInternationalLimited => Ok(EAS::OdetteInternationalLimited),
            crate::zugferd_2_3_2::EAS::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb => Ok(EAS::NuméroDidentificationSuisseDesEnterprisesIdeSwissUniqueBusinessIdentificationNumberUidb),
            crate::zugferd_2_3_2::EAS::Digstorg => Ok(EAS::Digstorg),
            crate::zugferd_2_3_2::EAS::CorporateNumberTheSocialSecurityAndTaxNumberSystem => Ok(EAS::CorporateNumberTheSocialSecurityAndTaxNumberSystem),
            crate::zugferd_2_3_2::EAS::DutchOriginatorsIdentificationNumber => Ok(EAS::DutchOriginatorsIdentificationNumber),
            crate::zugferd_2_3_2::EAS::CentreRegistersAndInformationSystemsMinistryJustice => Ok(EAS::CentreRegistersAndInformationSystemsMinistryJustice),
            crate::zugferd_2_3_2::EAS::EnhetsregisteretVedBronnoysundregisterne => Ok(EAS::EnhetsregisteretVedBronnoysundregisterne),
            crate::zugferd_2_3_2::EAS::UblBePartyIdentifier => Ok(EAS::UblBePartyIdentifier),
            crate::zugferd_2_3_2::EAS::KoiosOpenTechnicalDictionary => Ok(EAS::KoiosOpenTechnicalDictionary),
            crate::zugferd_2_3_2::EAS::SingaporeUenIdentifier => Ok(EAS::SingaporeUenIdentifier),
            crate::zugferd_2_3_2::EAS::KennitalaIcelandLegalIdForIndividualsAndLegalEntities => Ok(EAS::KennitalaIcelandLegalIdForIndividualsAndLegalEntities),
            crate::zugferd_2_3_2::EAS::Erstorg => Ok(EAS::Erstorg),
            crate::zugferd_2_3_2::EAS::GlobalLegalEntityIdentifierGleif => Ok(EAS::GlobalLegalEntityIdentifierGleif),
            crate::zugferd_2_3_2::EAS::LegalEntityCodeLithuania => Ok(EAS::LegalEntityCodeLithuania),
            crate::zugferd_2_3_2::EAS::CodiceUnivocoUnitàOrganizzativaIpa => Ok(EAS::CodiceUnivocoUnitàOrganizzativaIpa),
            crate::zugferd_2_3_2::EAS::IndirizzoDiPostaElettronicaCertificata => Ok(EAS::IndirizzoDiPostaElettronicaCertificata),
            crate::zugferd_2_3_2::EAS::EdeliveryNetworkParticipantIdentifier => Ok(EAS::EdeliveryNetworkParticipantIdentifier),
            crate::zugferd_2_3_2::EAS::LeitwegId => Ok(EAS::LeitwegId),
            crate::zugferd_2_3_2::EAS::Coddest => Ok(EAS::Coddest),
            crate::zugferd_2_3_2::EAS::NumeroDentrepriseOndernemingsnummerUnternehmensnummer => Ok(EAS::NumeroDentrepriseOndernemingsnummerUnternehmensnummer),
            crate::zugferd_2_3_2::EAS::Gs1IdentificationKeys => Ok(EAS::Gs1IdentificationKeys),
            crate::zugferd_2_3_2::EAS::CodiceFiscale => Ok(EAS::CodiceFiscale),
            crate::zugferd_2_3_2::EAS::PartitaIva => Ok(EAS::PartitaIva),
            crate::zugferd_2_3_2::EAS::FinnishOrganizationIdentifier => Ok(EAS::FinnishOrganizationIdentifier),
            crate::zugferd_2_3_2::EAS::FinnishOrganizationValueAddTaxIdentifier => Ok(EAS::FinnishOrganizationValueAddTaxIdentifier),
            crate::zugferd_2_3_2::EAS::NetServiceId => Ok(EAS::NetServiceId),
            crate::zugferd_2_3_2::EAS::Ovtcode => Ok(EAS::Ovtcode),
            crate::zugferd_2_3_2::EAS::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber => Ok(EAS::TheNetherlandsChamberCommerceAndIndustryEstablishmentNumber),
            crate::zugferd_2_3_2::EAS::UnifiedRegistrationNumberLatvia => Ok(EAS::UnifiedRegistrationNumberLatvia),
            crate::zugferd_2_3_2::EAS::TheRegisteredNumberQualifiedInvoiceIssuer => Ok(EAS::TheRegisteredNumberQualifiedInvoiceIssuer),
            crate::zugferd_2_3_2::EAS::FrctcElectronicAddress => Ok(EAS::FrctcElectronicAddress),
            crate::zugferd_2_3_2::EAS::NationalEInvoicingFramework => Ok(EAS::NationalEInvoicingFramework),
            crate::zugferd_2_3_2::EAS::UaeTaxIdentificationNumberTin => Ok(EAS::UaeTaxIdentificationNumberTin),
            crate::zugferd_2_3_2::EAS::HungaryVatNumber => Ok(EAS::HungaryVatNumber),
            crate::zugferd_2_3_2::EAS::BusinessRegistersNetwork => Ok(EAS::BusinessRegistersNetwork),
            crate::zugferd_2_3_2::EAS::ÖsterreichischeUmsatzsteuerIdentifikationsnummer => Ok(EAS::ÖsterreichischeUmsatzsteuerIdentifikationsnummer),
            crate::zugferd_2_3_2::EAS::ÖsterreichischesVerwaltungsBzwOrganisationskennzeichen => Ok(EAS::ÖsterreichischesVerwaltungsBzwOrganisationskennzeichen),
            crate::zugferd_2_3_2::EAS::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT => Ok(EAS::SocietyForWorldwideInterbankFinancialTelecommunicationSWIFT),
            crate::zugferd_2_3_2::EAS::KennzifferDesUnternehmensregisters => Ok(EAS::KennzifferDesUnternehmensregisters),
            crate::zugferd_2_3_2::EAS::AgenciaEspañolaDeAdministraciónTributaria => Ok(EAS::AgenciaEspañolaDeAdministraciónTributaria),
            crate::zugferd_2_3_2::EAS::AndorraVatNumber => Ok(EAS::AndorraVatNumber),
            crate::zugferd_2_3_2::EAS::AlbaniaVatNumber => Ok(EAS::AlbaniaVatNumber),
            crate::zugferd_2_3_2::EAS::BosniaAndHerzegovinaVatNumber => Ok(EAS::BosniaAndHerzegovinaVatNumber),
            crate::zugferd_2_3_2::EAS::BelgiumVatNumber => Ok(EAS::BelgiumVatNumber),
            crate::zugferd_2_3_2::EAS::BulgariaVatNumber => Ok(EAS::BulgariaVatNumber),
            crate::zugferd_2_3_2::EAS::SwitzerlandVatNumber => Ok(EAS::SwitzerlandVatNumber),
            crate::zugferd_2_3_2::EAS::CyprusVatNumber => Ok(EAS::CyprusVatNumber),
            crate::zugferd_2_3_2::EAS::CzechRepublicVatNumber => Ok(EAS::CzechRepublicVatNumber),
            crate::zugferd_2_3_2::EAS::GermanyVatNumber => Ok(EAS::GermanyVatNumber),
            crate::zugferd_2_3_2::EAS::EstoniaVatNumber => Ok(EAS::EstoniaVatNumber),
            crate::zugferd_2_3_2::EAS::UnitedKingdomVatNumber => Ok(EAS::UnitedKingdomVatNumber),
            crate::zugferd_2_3_2::EAS::GreeceVatNumber => Ok(EAS::GreeceVatNumber),
            crate::zugferd_2_3_2::EAS::CroatiaVatNumber => Ok(EAS::CroatiaVatNumber),
            crate::zugferd_2_3_2::EAS::IrelandVatNumber => Ok(EAS::IrelandVatNumber),
            crate::zugferd_2_3_2::EAS::LiechtensteinVatNumber => Ok(EAS::LiechtensteinVatNumber),
            crate::zugferd_2_3_2::EAS::LithuaniaVatNumber => Ok(EAS::LithuaniaVatNumber),
            crate::zugferd_2_3_2::EAS::LuxemburgVatNumber => Ok(EAS::LuxemburgVatNumber),
            crate::zugferd_2_3_2::EAS::LatviaVatNumber => Ok(EAS::LatviaVatNumber),
            crate::zugferd_2_3_2::EAS::MonacoVatNumber => Ok(EAS::MonacoVatNumber),
            crate::zugferd_2_3_2::EAS::MontenegroVatNumber => Ok(EAS::MontenegroVatNumber),
            crate::zugferd_2_3_2::EAS::MacedoniaFormerYugoslavRepublicVatNumber => Ok(EAS::MacedoniaFormerYugoslavRepublicVatNumber),
            crate::zugferd_2_3_2::EAS::MaltaVatNumber => Ok(EAS::MaltaVatNumber),
            crate::zugferd_2_3_2::EAS::NetherlandsVatNumber => Ok(EAS::NetherlandsVatNumber),
            crate::zugferd_2_3_2::EAS::PolandVatNumber => Ok(EAS::PolandVatNumber),
            crate::zugferd_2_3_2::EAS::PortugalVatNumber => Ok(EAS::PortugalVatNumber),
            crate::zugferd_2_3_2::EAS::RomaniaVatNumber => Ok(EAS::RomaniaVatNumber),
            crate::zugferd_2_3_2::EAS::SerbiaVatNumber => Ok(EAS::SerbiaVatNumber),
            crate::zugferd_2_3_2::EAS::SloveniaVatNumber => Ok(EAS::SloveniaVatNumber),
            crate::zugferd_2_3_2::EAS::SlovakiaVatNumber => Ok(EAS::SlovakiaVatNumber),
            crate::zugferd_2_3_2::EAS::SanMarinoVatNumber => Ok(EAS::SanMarinoVatNumber),
            crate::zugferd_2_3_2::EAS::TurkeyVatNumber => Ok(EAS::TurkeyVatNumber),
            crate::zugferd_2_3_2::EAS::HolySeeVaticanCityStateVatNumber => Ok(EAS::HolySeeVaticanCityStateVatNumber),
            crate::zugferd_2_3_2::EAS::FrenchVatNumber => Ok(EAS::FrenchVatNumber),
            crate::zugferd_2_3_2::EAS::EmployerIdentificationNumberEinUsa => Ok(EAS::EmployerIdentificationNumberEinUsa),
            crate::zugferd_2_3_2::EAS::OFTPOdetteFileTransferProtocol => Ok(EAS::OFTPOdetteFileTransferProtocol),
            crate::zugferd_2_3_2::EAS::X400AddressForMailText => Ok(EAS::X400AddressForMailText),
            crate::zugferd_2_3_2::EAS::As2Exchange => Ok(EAS::As2Exchange),
            crate::zugferd_2_3_2::EAS::FileTransferProtocol => Ok(EAS::FileTransferProtocol),
            crate::zugferd_2_3_2::EAS::ElectronicMailSmpt => Ok(EAS::ElectronicMailSmpt),
            crate::zugferd_2_3_2::EAS::DanishMinistryInteriorAndHealth => Err(ErrFromCrateZugferd232EasToEas::DanishMinistryInteriorAndHealth),
        }
    }
}

/// All the variants of crate::zugferd_2_3_2::EAS that are not matched to any variant of EAS
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrFromCrateZugferd232EasToEas {
    DanishMinistryInteriorAndHealth,
}

impl std::fmt::Display for ErrFromCrateZugferd232EasToEas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrFromCrateZugferd232EasToEas::DanishMinistryInteriorAndHealth => write!(
                f,
                "DanishMinistryInteriorAndHealth has no corresponding value in EAS"
            ),
        }
    }
}

impl std::error::Error for ErrFromCrateZugferd232EasToEas {}
// End: (Version) TryFrom crate::zugferd_2_3_2::EAS to EAS
