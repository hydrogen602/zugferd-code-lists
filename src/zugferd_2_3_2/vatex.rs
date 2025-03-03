#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum VATEX {
    ExemptBasedOnArticle79PointCCouncilDirective2006112Ec,
    ExemptBasedOnArticle132CouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec,
    ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec,
    ExemptBasedOnArticle143CouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec,
    ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec,
    ExemptBasedOnArticle144CouncilDirective2006112Ec,
    ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec,
    ExemptBasedOnArticle148CouncilDirective2006112Ec,
    ExemptBasedOnArticle148SectionACouncilDirective2006112Ec,
    ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec,
    ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec,
    ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec,
    ExemptBasedOnArticle148SectionECouncilDirective2006112Ec,
    ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec,
    ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec,
    ExemptBasedOnArticle151CouncilDirective2006112Ec,
    ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec,
    ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec,
    ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec,
    ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec,
    ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec,
    ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec,
    ExemptBasedOnArticle159CouncilDirective2006112Ec,
    ExemptBasedOnArticle309CouncilDirective2006112Ec,
    ReverseCharge,
    TravelAgentsVatScheme,
    SecondHandGoodsVatScheme,
    ExportOutsideEu,
    WorksArtVatScheme,
    IntraCommunitySupply,
    CollectorsItemsAndAntiquesVatScheme,
    NotSubjectToVat,
    FranceDomesticVatFranchiseInBase,
    FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount,
}

impl crate::Code for VATEX {
    fn code(&self) -> &str {
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
            VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec => "VATEX-EU-159",
            VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec => "VATEX-EU-309",
            VATEX::ReverseCharge => "VATEX-EU-AE",
            VATEX::TravelAgentsVatScheme => "VATEX-EU-D",
            VATEX::SecondHandGoodsVatScheme => "VATEX-EU-F",
            VATEX::ExportOutsideEu => "VATEX-EU-G",
            VATEX::WorksArtVatScheme => "VATEX-EU-I",
            VATEX::IntraCommunitySupply => "VATEX-EU-IC",
            VATEX::CollectorsItemsAndAntiquesVatScheme => "VATEX-EU-J",
            VATEX::NotSubjectToVat => "VATEX-EU-O",
            VATEX::FranceDomesticVatFranchiseInBase => "VATEX-FR-FRANCHISE",
            VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount => {
                "VATEX-FR-CNWVAT"
            }
        }
    }
}

impl crate::Description for VATEX {
    fn description(&self) -> &str {
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
            VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec => "Exempt based on article 159 of Council Directive 2006/112/EC",
            VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec => "Exempt based on article 309 of Council Directive 2006/112/EC ",
            VATEX::ReverseCharge => "Reverse charge",
            VATEX::TravelAgentsVatScheme => "Travel agents VAT scheme.",
            VATEX::SecondHandGoodsVatScheme => "Second hand goods VAT scheme.",
            VATEX::ExportOutsideEu => "Export outside the EU",
            VATEX::WorksArtVatScheme => "Works of art VAT scheme.",
            VATEX::IntraCommunitySupply => "Intra-community supply",
            VATEX::CollectorsItemsAndAntiquesVatScheme => "Collectors items and antiques VAT scheme.",
            VATEX::NotSubjectToVat => "Not subject to VAT",
            VATEX::FranceDomesticVatFranchiseInBase => "France domestic VAT franchise in base",
            VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount => "France domestic Credit Notes without VAT, due to supplier forfeit of VAT for discount",
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
            "VATEX-EU-132-1A" => {
                Some(VATEX::ExemptBasedOnArticle132Section1ACouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1B" => {
                Some(VATEX::ExemptBasedOnArticle132Section1BCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1C" => {
                Some(VATEX::ExemptBasedOnArticle132Section1CCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1D" => {
                Some(VATEX::ExemptBasedOnArticle132Section1DCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1E" => {
                Some(VATEX::ExemptBasedOnArticle132Section1ECouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1F" => {
                Some(VATEX::ExemptBasedOnArticle132Section1FCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1G" => {
                Some(VATEX::ExemptBasedOnArticle132Section1GCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1H" => {
                Some(VATEX::ExemptBasedOnArticle132Section1HCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1I" => {
                Some(VATEX::ExemptBasedOnArticle132Section1ICouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1J" => {
                Some(VATEX::ExemptBasedOnArticle132Section1JCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1K" => {
                Some(VATEX::ExemptBasedOnArticle132Section1KCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1L" => {
                Some(VATEX::ExemptBasedOnArticle132Section1LCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1M" => {
                Some(VATEX::ExemptBasedOnArticle132Section1MCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1N" => {
                Some(VATEX::ExemptBasedOnArticle132Section1NCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1O" => {
                Some(VATEX::ExemptBasedOnArticle132Section1OCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1P" => {
                Some(VATEX::ExemptBasedOnArticle132Section1PCouncilDirective2006112Ec)
            }
            "VATEX-EU-132-1Q" => {
                Some(VATEX::ExemptBasedOnArticle132Section1QCouncilDirective2006112Ec)
            }
            "VATEX-EU-143" => Some(VATEX::ExemptBasedOnArticle143CouncilDirective2006112Ec),
            "VATEX-EU-143-1A" => {
                Some(VATEX::ExemptBasedOnArticle143Section1ACouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1B" => {
                Some(VATEX::ExemptBasedOnArticle143Section1BCouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1C" => {
                Some(VATEX::ExemptBasedOnArticle143Section1CCouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1D" => {
                Some(VATEX::ExemptBasedOnArticle143Section1DCouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1E" => {
                Some(VATEX::ExemptBasedOnArticle143Section1ECouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1F" => {
                Some(VATEX::ExemptBasedOnArticle143Section1FCouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1FA" => {
                Some(VATEX::ExemptBasedOnArticle143Section1FaCouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1G" => {
                Some(VATEX::ExemptBasedOnArticle143Section1GCouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1H" => {
                Some(VATEX::ExemptBasedOnArticle143Section1HCouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1I" => {
                Some(VATEX::ExemptBasedOnArticle143Section1ICouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1J" => {
                Some(VATEX::ExemptBasedOnArticle143Section1JCouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1K" => {
                Some(VATEX::ExemptBasedOnArticle143Section1KCouncilDirective2006112Ec)
            }
            "VATEX-EU-143-1L" => {
                Some(VATEX::ExemptBasedOnArticle143Section1LCouncilDirective2006112Ec)
            }
            "VATEX-EU-144" => Some(VATEX::ExemptBasedOnArticle144CouncilDirective2006112Ec),
            "VATEX-EU-146-1E" => {
                Some(VATEX::ExemptBasedOnArticle146Section1ECouncilDirective2006112Ec)
            }
            "VATEX-EU-148" => Some(VATEX::ExemptBasedOnArticle148CouncilDirective2006112Ec),
            "VATEX-EU-148-A" => {
                Some(VATEX::ExemptBasedOnArticle148SectionACouncilDirective2006112Ec)
            }
            "VATEX-EU-148-B" => {
                Some(VATEX::ExemptBasedOnArticle148SectionBCouncilDirective2006112Ec)
            }
            "VATEX-EU-148-C" => {
                Some(VATEX::ExemptBasedOnArticle148SectionCCouncilDirective2006112Ec)
            }
            "VATEX-EU-148-D" => {
                Some(VATEX::ExemptBasedOnArticle148SectionDCouncilDirective2006112Ec)
            }
            "VATEX-EU-148-E" => {
                Some(VATEX::ExemptBasedOnArticle148SectionECouncilDirective2006112Ec)
            }
            "VATEX-EU-148-F" => {
                Some(VATEX::ExemptBasedOnArticle148SectionFCouncilDirective2006112Ec)
            }
            "VATEX-EU-148-G" => {
                Some(VATEX::ExemptBasedOnArticle148SectionGCouncilDirective2006112Ec)
            }
            "VATEX-EU-151" => Some(VATEX::ExemptBasedOnArticle151CouncilDirective2006112Ec),
            "VATEX-EU-151-1A" => {
                Some(VATEX::ExemptBasedOnArticle151Section1ACouncilDirective2006112Ec)
            }
            "VATEX-EU-151-1AA" => {
                Some(VATEX::ExemptBasedOnArticle151Section1AaCouncilDirective2006112Ec)
            }
            "VATEX-EU-151-1B" => {
                Some(VATEX::ExemptBasedOnArticle151Section1BCouncilDirective2006112Ec)
            }
            "VATEX-EU-151-1C" => {
                Some(VATEX::ExemptBasedOnArticle151Section1CCouncilDirective2006112Ec)
            }
            "VATEX-EU-151-1D" => {
                Some(VATEX::ExemptBasedOnArticle151Section1DCouncilDirective2006112Ec)
            }
            "VATEX-EU-151-1E" => {
                Some(VATEX::ExemptBasedOnArticle151Section1ECouncilDirective2006112Ec)
            }
            "VATEX-EU-159" => Some(VATEX::ExemptBasedOnArticle159CouncilDirective2006112Ec),
            "VATEX-EU-309" => Some(VATEX::ExemptBasedOnArticle309CouncilDirective2006112Ec),
            "VATEX-EU-AE" => Some(VATEX::ReverseCharge),
            "VATEX-EU-D" => Some(VATEX::TravelAgentsVatScheme),
            "VATEX-EU-F" => Some(VATEX::SecondHandGoodsVatScheme),
            "VATEX-EU-G" => Some(VATEX::ExportOutsideEu),
            "VATEX-EU-I" => Some(VATEX::WorksArtVatScheme),
            "VATEX-EU-IC" => Some(VATEX::IntraCommunitySupply),
            "VATEX-EU-J" => Some(VATEX::CollectorsItemsAndAntiquesVatScheme),
            "VATEX-EU-O" => Some(VATEX::NotSubjectToVat),
            "VATEX-FR-FRANCHISE" => Some(VATEX::FranceDomesticVatFranchiseInBase),
            "VATEX-FR-CNWVAT" => {
                Some(VATEX::FranceDomesticCreditNotesWithoutVatDueToSupplierForfeitVatForDiscount)
            }
            _ => None,
        }
    }
}
