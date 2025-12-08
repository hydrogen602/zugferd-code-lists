#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VATEX {
    /// Exempt based on article 79, point c of Council Directive 2006/112/EC
    ///
    /// Repayment of expenditure is not an exemption in the sense of the VAT Directive but may be handled as such in the context of the EN16931.
    ExemptBasedOnArticle79PointCCouncilDirective2006112Ec,
    /// Exempt based on article 132 of Council Directive 2006/112/EC
    ExemptBasedOnArticle132CouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (a) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (b) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (c) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (d) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (e) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (f) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (g) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (h) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (i) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (j) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (k) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (l) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (m) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (n) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (o) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (p) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec,
    /// Exempt based on article 132, section 1 (q) of Council Directive 2006/112/EC
    ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec,
    /// Exempt based on article 135, section 1 of Council Directive 2006/112/EC
    ExemptBasedOnArticle135Section1CouncilDirective2006112Ec,
    /// Exempt based on article 143 of Council Directive 2006/112/EC
    ExemptBasedOnArticle143CouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (a) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (b) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (c) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (d) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (e) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (f) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (fa) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (g) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (h) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (i) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (j) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (k) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec,
    /// Exempt based on article 143, section 1 (l) of Council Directive 2006/112/EC
    ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec,
    /// Exempt based on article 144 of Council Directive 2006/112/EC
    ExemptBasedOnArticle144CouncilDirective2006112Ec,
    /// Exempt based on article 146 section 1 (e) of Council Directive 2006/112/EC
    ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec,
    /// Exempt based on article 148 of Council Directive 2006/112/EC
    ExemptBasedOnArticle148CouncilDirective2006112Ec,
    /// Exempt based on article 148, section (a) of Council Directive 2006/112/EC
    ExemptBasedOnArticle148SectionACouncilDirective2006112Ec,
    /// Exempt based on article 148, section (b) of Council Directive 2006/112/EC
    ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec,
    /// Exempt based on article 148, section (c) of Council Directive 2006/112/EC
    ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec,
    /// Exempt based on article 148, section (d) of Council Directive 2006/112/EC
    ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec,
    /// Exempt based on article 148, section (e) of Council Directive 2006/112/EC
    ExemptBasedOnArticle148SectionECouncilDirective2006112Ec,
    /// Exempt based on article 148, section (f) of Council Directive 2006/112/EC
    ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec,
    /// Exempt based on article 148, section (g) of Council Directive 2006/112/EC
    ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec,
    /// Exempt based on article 151 of Council Directive 2006/112/EC
    ExemptBasedOnArticle151CouncilDirective2006112Ec,
    /// Exempt based on article 151, section 1 (a) of Council Directive 2006/112/EC
    ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec,
    /// Exempt based on article 151, section 1 (aa) of Council Directive 2006/112/EC
    ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec,
    /// Exempt based on article 151, section 1 (b) of Council Directive 2006/112/EC
    ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec,
    /// Exempt based on article 151, section 1 (c) of Council Directive 2006/112/EC
    ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec,
    /// Exempt based on article 151, section 1 (d) of Council Directive 2006/112/EC
    ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec,
    /// Exempt based on article 151, section 1 (e) of Council Directive 2006/112/EC
    ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec,
    /// Exempt based on article 153 of Council Directive 2006/112/EC
    ExemptBasedOnArticle153CouncilDirective2006112Ec,
    /// Exempt based on article 159 of Council Directive 2006/112/EC
    ExemptBasedOnArticle159CouncilDirective2006112Ec,
    /// Exempt based on article 309 of Council Directive 2006/112/EC
    ExemptBasedOnArticle309CouncilDirective2006112Ec,
    /// Reverse charge
    ///
    /// Only use with VAT category code AE
    ReverseCharge,
    /// Travel agents VAT scheme.
    ///
    /// Only use with VAT category code E
    TravelAgentsVatScheme,
    /// Intra-Community acquisition of second hand goods
    ///
    /// Only use with VAT category code E
    IntraCommunityAcquisitionSecondHandGoods,
    /// Export outside the EU
    ///
    /// Only use with VAT category code G
    ExportOutsideEu,
    /// Intra-Community acquisition of works of art
    ///
    /// Only use with VAT category code E
    IntraCommunityAcquisitionWorksArt,
    /// Intra-community supply
    ///
    /// Only use with VAT category code K
    IntraCommunitySupply,
    /// Intra-Community acquisition of collectors items and antiques
    ///
    /// Only use with VAT category code E
    IntraCommunityAcquisitionCollectorsItemsAndAntiques,
    /// Not subject to VAT
    ///
    /// Only use with VAT category code O
    NotSubjectToVat,
    /// France domestic VAT franchise in base
    ///
    /// For domestic invoicing in France
    FranceDomesticVatFranchiseInBase,
    /// France domestic Credit Notes without VAT, due to supplier forfeit of VAT for discount
    ///
    /// For domestic Credit Notes only in France
    FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount,
    /// Exempt based on 1 of article 261 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn1Article261CodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 2 of article 261 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn2Article261CodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 3 of article 261 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn3Article261CodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 4 of article 261 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn4Article261CodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 5 of article 261 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn5Article261CodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 7 of article 261 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn7Article261CodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 8 of article 261 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn8Article261CodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on article 261 A of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOnArticle261ACodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on article 261 B of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOnArticle261BCodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 1° of article 261 C of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn1Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 2° of article 261 C of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn2Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 3° of article 261 C of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn3Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 1° of article 261 D of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn1Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 1°bis of article 261 D of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn1BisArticle261DCodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 2° of article 261 D of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn2Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 3° of article 261 D of the Code Général des Impôts (CGI ; General tax code) Exonération de TVA - Article 261 D-3° du Code Général des Impôts
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn3Article261DCodeGénéralDesImpôtsCgiGeneralTaxCodeExonérationDeTvaArticle261D3DuCodeGénéralDesImpôts,
    /// Exempt based on 4° of article 261 D of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn4Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 1° of article 261 E of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn1Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 2° of article 261 E of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn2Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on article 277 A of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOnArticle277ACodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on article 275 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOnArticle275CodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on article 298 sexdecies A of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOnArticle298SexdeciesACodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on article 295 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOnArticle295CodeGénéralDesImpôtsCgiGeneralTaxCode,
    /// Exempt based on 2 of article 283 of the Code Général des Impôts (CGI ; General tax code)
    ///
    /// Only for domestic invoicing in France
    ExemptBasedOn2Article283CodeGénéralDesImpôtsCgiGeneralTaxCode,
}

impl std::fmt::Display for VATEX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for VATEX {
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
    }
}

impl crate::Code for VATEX {
    fn code(self) -> &'static str {
        match self {
            VATEX::ExemptBasedOnArticle79PointCCouncilDirective2006112Ec => "VATEX-EU-79-C",
            VATEX::ExemptBasedOnArticle132CouncilDirective2006112Ec => "VATEX-EU-132",
            VATEX::ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec => "VATEX-EU-132-1A",
            VATEX::ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec => "VATEX-EU-132-1B",
            VATEX::ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec => "VATEX-EU-132-1C",
            VATEX::ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec => "VATEX-EU-132-1D",
            VATEX::ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec => "VATEX-EU-132-1E",
            VATEX::ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec => "VATEX-EU-132-1F",
            VATEX::ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec => "VATEX-EU-132-1G",
            VATEX::ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec => "VATEX-EU-132-1H",
            VATEX::ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec => "VATEX-EU-132-1I",
            VATEX::ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec => "VATEX-EU-132-1J",
            VATEX::ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec => "VATEX-EU-132-1K",
            VATEX::ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec => "VATEX-EU-132-1L",
            VATEX::ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec => "VATEX-EU-132-1M",
            VATEX::ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec => "VATEX-EU-132-1N",
            VATEX::ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec => "VATEX-EU-132-1O",
            VATEX::ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec => "VATEX-EU-132-1P",
            VATEX::ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec => "VATEX-EU-132-1Q",
            VATEX::ExemptBasedOnArticle135Section1CouncilDirective2006112Ec => "VATEX-EU-135-1",
            VATEX::ExemptBasedOnArticle143CouncilDirective2006112Ec => "VATEX-EU-143",
            VATEX::ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec => "VATEX-EU-143-1A",
            VATEX::ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec => "VATEX-EU-143-1B",
            VATEX::ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec => "VATEX-EU-143-1C",
            VATEX::ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec => "VATEX-EU-143-1D",
            VATEX::ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec => "VATEX-EU-143-1E",
            VATEX::ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec => "VATEX-EU-143-1F",
            VATEX::ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec => "VATEX-EU-143-1FA",
            VATEX::ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec => "VATEX-EU-143-1G",
            VATEX::ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec => "VATEX-EU-143-1H",
            VATEX::ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec => "VATEX-EU-143-1I",
            VATEX::ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec => "VATEX-EU-143-1J",
            VATEX::ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec => "VATEX-EU-143-1K",
            VATEX::ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec => "VATEX-EU-143-1L",
            VATEX::ExemptBasedOnArticle144CouncilDirective2006112Ec => "VATEX-EU-144",
            VATEX::ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec => "VATEX-EU-146-1E",
            VATEX::ExemptBasedOnArticle148CouncilDirective2006112Ec => "VATEX-EU-148",
            VATEX::ExemptBasedOnArticle148SectionACouncilDirective2006112Ec => "VATEX-EU-148-A",
            VATEX::ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec => "VATEX-EU-148-B",
            VATEX::ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec => "VATEX-EU-148-C",
            VATEX::ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec => "VATEX-EU-148-D",
            VATEX::ExemptBasedOnArticle148SectionECouncilDirective2006112Ec => "VATEX-EU-148-E",
            VATEX::ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec => "VATEX-EU-148-F",
            VATEX::ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec => "VATEX-EU-148-G",
            VATEX::ExemptBasedOnArticle151CouncilDirective2006112Ec => "VATEX-EU-151",
            VATEX::ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec => "VATEX-EU-151-1A",
            VATEX::ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec => "VATEX-EU-151-1AA",
            VATEX::ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec => "VATEX-EU-151-1B",
            VATEX::ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec => "VATEX-EU-151-1C",
            VATEX::ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec => "VATEX-EU-151-1D",
            VATEX::ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec => "VATEX-EU-151-1E",
            VATEX::ExemptBasedOnArticle153CouncilDirective2006112Ec => "VATEX-EU-153",
            VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec => "VATEX-EU-159",
            VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec => "VATEX-EU-309",
            VATEX::ReverseCharge => "VATEX-EU-AE",
            VATEX::TravelAgentsVatScheme => "VATEX-EU-D",
            VATEX::IntraCommunityAcquisitionSecondHandGoods => "VATEX-EU-F",
            VATEX::ExportOutsideEu => "VATEX-EU-G",
            VATEX::IntraCommunityAcquisitionWorksArt => "VATEX-EU-I",
            VATEX::IntraCommunitySupply => "VATEX-EU-IC",
            VATEX::IntraCommunityAcquisitionCollectorsItemsAndAntiques => "VATEX-EU-J",
            VATEX::NotSubjectToVat => "VATEX-EU-O",
            VATEX::FranceDomesticVatFranchiseInBase => "VATEX-FR-FRANCHISE",
            VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount => "VATEX-FR-CNWVAT",
            VATEX::ExemptBasedOn1Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261-1",
            VATEX::ExemptBasedOn2Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261-2",
            VATEX::ExemptBasedOn3Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261-3",
            VATEX::ExemptBasedOn4Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261-4",
            VATEX::ExemptBasedOn5Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261-5",
            VATEX::ExemptBasedOn7Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261-7",
            VATEX::ExemptBasedOn8Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261-8",
            VATEX::ExemptBasedOnArticle261ACodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261A",
            VATEX::ExemptBasedOnArticle261BCodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261B",
            VATEX::ExemptBasedOn1Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261C-1",
            VATEX::ExemptBasedOn2Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261C-2",
            VATEX::ExemptBasedOn3Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261C-3",
            VATEX::ExemptBasedOn1Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261D-1",
            VATEX::ExemptBasedOn1BisArticle261DCodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261D-1BIS",
            VATEX::ExemptBasedOn2Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261D-2",
            VATEX::ExemptBasedOn3Article261DCodeGénéralDesImpôtsCgiGeneralTaxCodeExonérationDeTvaArticle261D3DuCodeGénéralDesImpôts => "VATEX-FR-CGI261D-3",
            VATEX::ExemptBasedOn4Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261D-4",
            VATEX::ExemptBasedOn1Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261E-1",
            VATEX::ExemptBasedOn2Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI261E-2",
            VATEX::ExemptBasedOnArticle277ACodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI277A",
            VATEX::ExemptBasedOnArticle275CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI275",
            VATEX::ExemptBasedOnArticle298SexdeciesACodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-298SEXDECIESA",
            VATEX::ExemptBasedOnArticle295CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-CGI295",
            VATEX::ExemptBasedOn2Article283CodeGénéralDesImpôtsCgiGeneralTaxCode => "VATEX-FR-AE",
        }
    }
}

impl crate::Description for VATEX {
    fn description(self) -> &'static str {
        match self {
            VATEX::ExemptBasedOnArticle79PointCCouncilDirective2006112Ec => "Exempt based on article 79, point c of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132CouncilDirective2006112Ec => "Exempt based on article 132 of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec => "Exempt based on article 132, section 1 (a) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (b) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (c) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (d) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec => "Exempt based on article 132, section 1 (e) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (f) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (g) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (h) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec => "Exempt based on article 132, section 1 (i) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (j) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (k) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (l) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (m) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (n) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (o) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (p) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec => "Exempt based on article 132, section 1 (q) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle135Section1CouncilDirective2006112Ec => "Exempt based on article 135, section 1 of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143CouncilDirective2006112Ec => "Exempt based on article 143 of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec => "Exempt based on article 143, section 1 (a) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (b) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (c) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (d) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec => "Exempt based on article 143, section 1 (e) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (f) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (fa) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (g) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (h) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec => "Exempt based on article 143, section 1 (i) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (j) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (k) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec => "Exempt based on article 143, section 1 (l) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle144CouncilDirective2006112Ec => "Exempt based on article 144 of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec => "Exempt based on article 146 section 1 (e) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle148CouncilDirective2006112Ec => "Exempt based on article 148 of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle148SectionACouncilDirective2006112Ec => "Exempt based on article 148, section (a) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec => "Exempt based on article 148, section (b) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec => "Exempt based on article 148, section (c) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec => "Exempt based on article 148, section (d) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle148SectionECouncilDirective2006112Ec => "Exempt based on article 148, section (e) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec => "Exempt based on article 148, section (f) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec => "Exempt based on article 148, section (g) of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle151CouncilDirective2006112Ec => "Exempt based on article 151 of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec => "Exempt based on article 151, section 1 (a) of Council Directive 2006/112/EC ",
            VATEX::ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec => "Exempt based on article 151, section 1 (aa) of Council Directive 2006/112/EC ",
            VATEX::ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec => "Exempt based on article 151, section 1 (b) of Council Directive 2006/112/EC ",
            VATEX::ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec => "Exempt based on article 151, section 1 (c) of Council Directive 2006/112/EC ",
            VATEX::ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec => "Exempt based on article 151, section 1 (d) of Council Directive 2006/112/EC ",
            VATEX::ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec => "Exempt based on article 151, section 1 (e) of Council Directive 2006/112/EC ",
            VATEX::ExemptBasedOnArticle153CouncilDirective2006112Ec => "Exempt based on article 153 of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec => "Exempt based on article 159 of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec => "Exempt based on article 309 of Council Directive 2006/112/EC ",
            VATEX::ReverseCharge => "Reverse charge",
            VATEX::TravelAgentsVatScheme => "Travel agents VAT scheme.",
            VATEX::IntraCommunityAcquisitionSecondHandGoods => "Intra-Community acquisition of second hand goods",
            VATEX::ExportOutsideEu => "Export outside the EU",
            VATEX::IntraCommunityAcquisitionWorksArt => "Intra-Community acquisition of works of art",
            VATEX::IntraCommunitySupply => "Intra-community supply",
            VATEX::IntraCommunityAcquisitionCollectorsItemsAndAntiques => "Intra-Community acquisition of collectors items and antiques",
            VATEX::NotSubjectToVat => "Not subject to VAT",
            VATEX::FranceDomesticVatFranchiseInBase => "France domestic VAT franchise in base",
            VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount => "France domestic Credit Notes without VAT, due to supplier forfeit of VAT for discount",
            VATEX::ExemptBasedOn1Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 1 of article 261 of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn2Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 2 of article 261 of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn3Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 3 of article 261 of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn4Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 4 of article 261 of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn5Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 5 of article 261 of the Code Général des Impôts (CGI ; General tax code) ",
            VATEX::ExemptBasedOn7Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 7 of article 261 of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn8Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 8 of article 261 of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOnArticle261ACodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on article 261 A of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOnArticle261BCodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on article 261 B of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn1Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 1° of article 261 C of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn2Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 2° of article 261 C of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn3Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 3° of article 261 C of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn1Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 1° of article 261 D of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn1BisArticle261DCodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 1°bis of article 261 D of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn2Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 2° of article 261 D of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn3Article261DCodeGénéralDesImpôtsCgiGeneralTaxCodeExonérationDeTvaArticle261D3DuCodeGénéralDesImpôts => "Exempt based on 3° of article 261 D of the Code Général des Impôts (CGI ; General tax code) Exonération de TVA - Article 261 D-3° du Code Général des Impôts ",
            VATEX::ExemptBasedOn4Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 4° of article 261 D of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn1Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 1° of article 261 E of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn2Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 2° of article 261 E of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOnArticle277ACodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on article 277 A of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOnArticle275CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on article 275 of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOnArticle298SexdeciesACodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on article 298 sexdecies A of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOnArticle295CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on article 295 of the Code Général des Impôts (CGI ; General tax code)",
            VATEX::ExemptBasedOn2Article283CodeGénéralDesImpôtsCgiGeneralTaxCode => "Exempt based on 2 of article 283 of the Code Général des Impôts (CGI ; General tax code)",
        }
    }
}

impl crate::FromCode for VATEX {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "VATEX-EU-79-C" => Some(VATEX::ExemptBasedOnArticle79PointCCouncilDirective2006112Ec),
            "VATEX-EU-132" => Some(VATEX::ExemptBasedOnArticle132CouncilDirective2006112Ec),
            "VATEX-EU-132-1A" => Some(VATEX::ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec),
            "VATEX-EU-132-1B" => Some(VATEX::ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec),
            "VATEX-EU-132-1C" => Some(VATEX::ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec),
            "VATEX-EU-132-1D" => Some(VATEX::ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec),
            "VATEX-EU-132-1E" => Some(VATEX::ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec),
            "VATEX-EU-132-1F" => Some(VATEX::ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec),
            "VATEX-EU-132-1G" => Some(VATEX::ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec),
            "VATEX-EU-132-1H" => Some(VATEX::ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec),
            "VATEX-EU-132-1I" => Some(VATEX::ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec),
            "VATEX-EU-132-1J" => Some(VATEX::ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec),
            "VATEX-EU-132-1K" => Some(VATEX::ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec),
            "VATEX-EU-132-1L" => Some(VATEX::ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec),
            "VATEX-EU-132-1M" => Some(VATEX::ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec),
            "VATEX-EU-132-1N" => Some(VATEX::ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec),
            "VATEX-EU-132-1O" => Some(VATEX::ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec),
            "VATEX-EU-132-1P" => Some(VATEX::ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec),
            "VATEX-EU-132-1Q" => Some(VATEX::ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec),
            "VATEX-EU-135-1" => Some(VATEX::ExemptBasedOnArticle135Section1CouncilDirective2006112Ec),
            "VATEX-EU-143" => Some(VATEX::ExemptBasedOnArticle143CouncilDirective2006112Ec),
            "VATEX-EU-143-1A" => Some(VATEX::ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec),
            "VATEX-EU-143-1B" => Some(VATEX::ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec),
            "VATEX-EU-143-1C" => Some(VATEX::ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec),
            "VATEX-EU-143-1D" => Some(VATEX::ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec),
            "VATEX-EU-143-1E" => Some(VATEX::ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec),
            "VATEX-EU-143-1F" => Some(VATEX::ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec),
            "VATEX-EU-143-1FA" => Some(VATEX::ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec),
            "VATEX-EU-143-1G" => Some(VATEX::ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec),
            "VATEX-EU-143-1H" => Some(VATEX::ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec),
            "VATEX-EU-143-1I" => Some(VATEX::ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec),
            "VATEX-EU-143-1J" => Some(VATEX::ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec),
            "VATEX-EU-143-1K" => Some(VATEX::ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec),
            "VATEX-EU-143-1L" => Some(VATEX::ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec),
            "VATEX-EU-144" => Some(VATEX::ExemptBasedOnArticle144CouncilDirective2006112Ec),
            "VATEX-EU-146-1E" => Some(VATEX::ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec),
            "VATEX-EU-148" => Some(VATEX::ExemptBasedOnArticle148CouncilDirective2006112Ec),
            "VATEX-EU-148-A" => Some(VATEX::ExemptBasedOnArticle148SectionACouncilDirective2006112Ec),
            "VATEX-EU-148-B" => Some(VATEX::ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec),
            "VATEX-EU-148-C" => Some(VATEX::ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec),
            "VATEX-EU-148-D" => Some(VATEX::ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec),
            "VATEX-EU-148-E" => Some(VATEX::ExemptBasedOnArticle148SectionECouncilDirective2006112Ec),
            "VATEX-EU-148-F" => Some(VATEX::ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec),
            "VATEX-EU-148-G" => Some(VATEX::ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec),
            "VATEX-EU-151" => Some(VATEX::ExemptBasedOnArticle151CouncilDirective2006112Ec),
            "VATEX-EU-151-1A" => Some(VATEX::ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec),
            "VATEX-EU-151-1AA" => Some(VATEX::ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec),
            "VATEX-EU-151-1B" => Some(VATEX::ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec),
            "VATEX-EU-151-1C" => Some(VATEX::ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec),
            "VATEX-EU-151-1D" => Some(VATEX::ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec),
            "VATEX-EU-151-1E" => Some(VATEX::ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec),
            "VATEX-EU-153" => Some(VATEX::ExemptBasedOnArticle153CouncilDirective2006112Ec),
            "VATEX-EU-159" => Some(VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec),
            "VATEX-EU-309" => Some(VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec),
            "VATEX-EU-AE" => Some(VATEX::ReverseCharge),
            "VATEX-EU-D" => Some(VATEX::TravelAgentsVatScheme),
            "VATEX-EU-F" => Some(VATEX::IntraCommunityAcquisitionSecondHandGoods),
            "VATEX-EU-G" => Some(VATEX::ExportOutsideEu),
            "VATEX-EU-I" => Some(VATEX::IntraCommunityAcquisitionWorksArt),
            "VATEX-EU-IC" => Some(VATEX::IntraCommunitySupply),
            "VATEX-EU-J" => Some(VATEX::IntraCommunityAcquisitionCollectorsItemsAndAntiques),
            "VATEX-EU-O" => Some(VATEX::NotSubjectToVat),
            "VATEX-FR-FRANCHISE" => Some(VATEX::FranceDomesticVatFranchiseInBase),
            "VATEX-FR-CNWVAT" => Some(VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount),
            "VATEX-FR-CGI261-1" => Some(VATEX::ExemptBasedOn1Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261-2" => Some(VATEX::ExemptBasedOn2Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261-3" => Some(VATEX::ExemptBasedOn3Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261-4" => Some(VATEX::ExemptBasedOn4Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261-5" => Some(VATEX::ExemptBasedOn5Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261-7" => Some(VATEX::ExemptBasedOn7Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261-8" => Some(VATEX::ExemptBasedOn8Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261A" => Some(VATEX::ExemptBasedOnArticle261ACodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261B" => Some(VATEX::ExemptBasedOnArticle261BCodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261C-1" => Some(VATEX::ExemptBasedOn1Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261C-2" => Some(VATEX::ExemptBasedOn2Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261C-3" => Some(VATEX::ExemptBasedOn3Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261D-1" => Some(VATEX::ExemptBasedOn1Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261D-1BIS" => Some(VATEX::ExemptBasedOn1BisArticle261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261D-2" => Some(VATEX::ExemptBasedOn2Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261D-3" => Some(VATEX::ExemptBasedOn3Article261DCodeGénéralDesImpôtsCgiGeneralTaxCodeExonérationDeTvaArticle261D3DuCodeGénéralDesImpôts),
            "VATEX-FR-CGI261D-4" => Some(VATEX::ExemptBasedOn4Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261E-1" => Some(VATEX::ExemptBasedOn1Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI261E-2" => Some(VATEX::ExemptBasedOn2Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI277A" => Some(VATEX::ExemptBasedOnArticle277ACodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI275" => Some(VATEX::ExemptBasedOnArticle275CodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-298SEXDECIESA" => Some(VATEX::ExemptBasedOnArticle298SexdeciesACodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-CGI295" => Some(VATEX::ExemptBasedOnArticle295CodeGénéralDesImpôtsCgiGeneralTaxCode),
            "VATEX-FR-AE" => Some(VATEX::ExemptBasedOn2Article283CodeGénéralDesImpôtsCgiGeneralTaxCode),
            _ => None,
        }
    }
}

// Start: (Version) TryFrom VATEX to crate::zugferd_2_3_3::VATEX
impl std::convert::TryFrom<VATEX> for crate::zugferd_2_3_3::VATEX {
    type Error = ErrFromVatexToCrateZugferd233Vatex;
    fn try_from(value: VATEX) -> Result<Self, Self::Error> {
        match value {
            VATEX::ExemptBasedOnArticle79PointCCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle79PointCCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132CouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132CouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143CouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143CouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle144CouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle144CouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle148CouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148CouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle148SectionACouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionACouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle148SectionECouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionECouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle151CouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151CouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle153CouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle153CouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec),
            VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec),
            VATEX::ReverseCharge => Ok(crate::zugferd_2_3_3::VATEX::ReverseCharge),
            VATEX::TravelAgentsVatScheme => Ok(crate::zugferd_2_3_3::VATEX::TravelAgentsVatScheme),
            VATEX::IntraCommunityAcquisitionSecondHandGoods => Ok(crate::zugferd_2_3_3::VATEX::SecondHandGoodsVatScheme),
            VATEX::ExportOutsideEu => Ok(crate::zugferd_2_3_3::VATEX::ExportOutsideEu),
            VATEX::IntraCommunityAcquisitionWorksArt => Ok(crate::zugferd_2_3_3::VATEX::WorksArtVatScheme),
            VATEX::IntraCommunitySupply => Ok(crate::zugferd_2_3_3::VATEX::IntraCommunitySupply),
            VATEX::IntraCommunityAcquisitionCollectorsItemsAndAntiques => Ok(crate::zugferd_2_3_3::VATEX::CollectorsItemsAndAntiquesVatScheme),
            VATEX::NotSubjectToVat => Ok(crate::zugferd_2_3_3::VATEX::NotSubjectToVat),
            VATEX::FranceDomesticVatFranchiseInBase => Ok(crate::zugferd_2_3_3::VATEX::FranceDomesticVatFranchiseInBase),
            VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount => Ok(crate::zugferd_2_3_3::VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount),
            VATEX::ExemptBasedOn1Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn1Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn2Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn3Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn3Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn4Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn4Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn5Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn5Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn7Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn7Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn8Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn8Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOnArticle261ACodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle261ACodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOnArticle261BCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle261BCodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn1Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn1Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn2Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn3Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn3Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn1Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn1Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn1BisArticle261DCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn1BisArticle261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn2Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn3Article261DCodeGénéralDesImpôtsCgiGeneralTaxCodeExonérationDeTvaArticle261D3DuCodeGénéralDesImpôts => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn3Article261DCodeGénéralDesImpôtsCgiGeneralTaxCodeExonérationDeTvaArticle261D3DuCodeGénéralDesImpôts),
            VATEX::ExemptBasedOn4Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn4Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn1Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn1Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn2Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOnArticle277ACodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle277ACodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOnArticle275CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle275CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOnArticle298SexdeciesACodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle298SexdeciesACodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOnArticle295CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle295CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOn2Article283CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article283CodeGénéralDesImpôtsCgiGeneralTaxCode),
            VATEX::ExemptBasedOnArticle135Section1CouncilDirective2006112Ec => Err(ErrFromVatexToCrateZugferd233Vatex::ExemptBasedOnArticle135Section1CouncilDirective2006112Ec),
        }
    }
}

/// All the variants of VATEX that are not matched to any variant of crate::zugferd_2_3_3::VATEX
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrFromVatexToCrateZugferd233Vatex {
    ExemptBasedOnArticle135Section1CouncilDirective2006112Ec,
}

impl std::fmt::Display for ErrFromVatexToCrateZugferd233Vatex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrFromVatexToCrateZugferd233Vatex::ExemptBasedOnArticle135Section1CouncilDirective2006112Ec => write!(f, "ExemptBasedOnArticle135Section1CouncilDirective2006112Ec has no corresponding value in crate::zugferd_2_3_3::VATEX"),
        }
    }
}

impl std::error::Error for ErrFromVatexToCrateZugferd233Vatex {}

impl std::convert::TryFrom<crate::zugferd_2_3_3::VATEX> for VATEX {
    type Error = std::convert::Infallible;
    fn try_from(value: crate::zugferd_2_3_3::VATEX) -> Result<VATEX, Self::Error> {
        match value {
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle79PointCCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle79PointCCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132CouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132CouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143CouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143CouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle144CouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle144CouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148CouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle148CouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionACouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle148SectionACouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionECouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle148SectionECouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151CouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle151CouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle153CouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle153CouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec => Ok(VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec),
            crate::zugferd_2_3_3::VATEX::ReverseCharge => Ok(VATEX::ReverseCharge),
            crate::zugferd_2_3_3::VATEX::TravelAgentsVatScheme => Ok(VATEX::TravelAgentsVatScheme),
            crate::zugferd_2_3_3::VATEX::SecondHandGoodsVatScheme => Ok(VATEX::IntraCommunityAcquisitionSecondHandGoods),
            crate::zugferd_2_3_3::VATEX::ExportOutsideEu => Ok(VATEX::ExportOutsideEu),
            crate::zugferd_2_3_3::VATEX::WorksArtVatScheme => Ok(VATEX::IntraCommunityAcquisitionWorksArt),
            crate::zugferd_2_3_3::VATEX::IntraCommunitySupply => Ok(VATEX::IntraCommunitySupply),
            crate::zugferd_2_3_3::VATEX::CollectorsItemsAndAntiquesVatScheme => Ok(VATEX::IntraCommunityAcquisitionCollectorsItemsAndAntiques),
            crate::zugferd_2_3_3::VATEX::NotSubjectToVat => Ok(VATEX::NotSubjectToVat),
            crate::zugferd_2_3_3::VATEX::FranceDomesticVatFranchiseInBase => Ok(VATEX::FranceDomesticVatFranchiseInBase),
            crate::zugferd_2_3_3::VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount => Ok(VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn1Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn1Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn2Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn3Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn3Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn4Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn4Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn5Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn5Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn7Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn7Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn8Article261CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn8Article261CodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle261ACodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOnArticle261ACodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle261BCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOnArticle261BCodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn1Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn1Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn2Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn3Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn3Article261CCodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn1Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn1Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn1BisArticle261DCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn1BisArticle261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn2Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn3Article261DCodeGénéralDesImpôtsCgiGeneralTaxCodeExonérationDeTvaArticle261D3DuCodeGénéralDesImpôts => Ok(VATEX::ExemptBasedOn3Article261DCodeGénéralDesImpôtsCgiGeneralTaxCodeExonérationDeTvaArticle261D3DuCodeGénéralDesImpôts),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn4Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn4Article261DCodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn1Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn1Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn2Article261ECodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle277ACodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOnArticle277ACodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle275CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOnArticle275CodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle298SexdeciesACodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOnArticle298SexdeciesACodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOnArticle295CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOnArticle295CodeGénéralDesImpôtsCgiGeneralTaxCode),
            crate::zugferd_2_3_3::VATEX::ExemptBasedOn2Article283CodeGénéralDesImpôtsCgiGeneralTaxCode => Ok(VATEX::ExemptBasedOn2Article283CodeGénéralDesImpôtsCgiGeneralTaxCode),

        }
    }
}
// End: (Version) TryFrom crate::zugferd_2_3_3::VATEX to VATEX
