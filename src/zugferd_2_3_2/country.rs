#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Country {
    /// Andorra
    Andorra,
    /// United Arab Emirates (the)
    UnitedArabEmirates,
    /// Afghanistan
    Afghanistan,
    /// Antigua and Barbuda
    AntiguaAndBarbuda,
    /// Anguilla
    Anguilla,
    /// Albania
    Albania,
    /// Armenia
    Armenia,
    /// Angola
    Angola,
    /// Antarctica
    Antarctica,
    /// Argentina
    Argentina,
    /// American Samoa
    AmericanSamoa,
    /// Austria
    Austria,
    /// Australia
    Australia,
    /// Aruba
    Aruba,
    /// Åland Islands
    ÅlandIslands,
    /// Azerbaijan
    Azerbaijan,
    /// Bosnia and Herzegovina
    BosniaAndHerzegovina,
    /// Barbados
    Barbados,
    /// Bangladesh
    Bangladesh,
    /// Belgium
    Belgium,
    /// Burkina Faso
    BurkinaFaso,
    /// Bulgaria
    Bulgaria,
    /// Bahrain
    Bahrain,
    /// Burundi
    Burundi,
    /// Benin
    Benin,
    /// Saint Barthélemy
    SaintBarthélemy,
    /// Bermuda
    Bermuda,
    /// Brunei Darussalam
    BruneiDarussalam,
    /// Bolivia (Plurinational State of)
    BoliviaPlurinationalState,
    /// Bonaire, Sint Eustatius and Saba
    BonaireSintEustatiusAndSaba,
    /// Brazil
    Brazil,
    /// Bahamas (the)
    Bahamas,
    /// Bhutan
    Bhutan,
    /// Bouvet Island
    BouvetIsland,
    /// Botswana
    Botswana,
    /// Belarus
    Belarus,
    /// Belize
    Belize,
    /// Canada
    Canada,
    /// Cocos (Keeling) Islands (the)
    CocosKeelingIslands,
    /// Congo (the Democratic Republic of the)
    CongoDemocraticRepublic,
    /// Central African Republic (the)
    CentralAfricanRepublic,
    /// Congo (the)
    Congo,
    /// Switzerland
    Switzerland,
    /// Côte d'Ivoire
    CôteDivoire,
    /// Cook Islands (the)
    CookIslands,
    /// Chile
    Chile,
    /// Cameroon
    Cameroon,
    /// China
    China,
    /// Colombia
    Colombia,
    /// Costa Rica
    CostaRica,
    /// Cuba
    Cuba,
    /// Cabo Verde
    CaboVerde,
    /// Curaçao
    Curaçao,
    /// Christmas Island
    ChristmasIsland,
    /// Cyprus
    Cyprus,
    /// Czechia
    Czechia,
    /// Germany
    Germany,
    /// Djibouti
    Djibouti,
    /// Denmark
    Denmark,
    /// Dominica
    Dominica,
    /// Dominican Republic (the)
    DominicanRepublic,
    /// Algeria
    Algeria,
    /// Ecuador
    Ecuador,
    /// Estonia
    Estonia,
    /// Egypt
    Egypt,
    /// Western Sahara*
    WesternSahara,
    /// Eritrea
    Eritrea,
    /// Spain
    Spain,
    /// Ethiopia
    Ethiopia,
    /// Finland
    Finland,
    /// Fiji
    Fiji,
    /// Falkland Islands (the) [Malvinas]
    FalklandIslandsMalvinas,
    /// Micronesia (Federated States of)
    MicronesiaFederatedStates,
    /// Faroe Islands (the)
    FaroeIslands,
    /// France
    France,
    /// Gabon
    Gabon,
    /// United Kingdom of Great Britain and Northern Ireland (the)
    UnitedKingdomGreatBritainAndNorthernIreland,
    /// Grenada
    Grenada,
    /// Georgia
    Georgia,
    /// French Guiana
    FrenchGuiana,
    /// Guernsey
    Guernsey,
    /// Ghana
    Ghana,
    /// Gibraltar
    Gibraltar,
    /// Greenland
    Greenland,
    /// Gambia (the)
    Gambia,
    /// Guinea
    Guinea,
    /// Guadeloupe
    Guadeloupe,
    /// Equatorial Guinea
    EquatorialGuinea,
    /// Greece
    Greece,
    /// South Georgia and the South Sandwich Islands
    SouthGeorgiaAndSouthSandwichIslands,
    /// Guatemala
    Guatemala,
    /// Guam
    Guam,
    /// Guinea-Bissau
    GuineaBissau,
    /// Guyana
    Guyana,
    /// Hong Kong
    HongKong,
    /// Heard Island and McDonald Islands
    HeardIslandAndMcdonaldIslands,
    /// Honduras
    Honduras,
    /// Croatia
    Croatia,
    /// Haiti
    Haiti,
    /// Hungary
    Hungary,
    /// Indonesia
    Indonesia,
    /// Ireland
    Ireland,
    /// Israel
    Israel,
    /// Isle of Man
    IsleMan,
    /// India
    India,
    /// British Indian Ocean Territory (the)
    BritishIndianOceanTerritory,
    /// Iraq
    Iraq,
    /// Iran (Islamic Republic of)
    IranIslamicRepublic,
    /// Iceland
    Iceland,
    /// Italy
    Italy,
    /// Jersey
    Jersey,
    /// Jamaica
    Jamaica,
    /// Jordan
    Jordan,
    /// Japan
    Japan,
    /// Kenya
    Kenya,
    /// Kyrgyzstan
    Kyrgyzstan,
    /// Cambodia
    Cambodia,
    /// Kiribati
    Kiribati,
    /// Comoros (the)
    Comoros,
    /// Saint Kitts and Nevis
    SaintKittsAndNevis,
    /// Korea (the Democratic People's Republic of)
    KoreaDemocraticPeoplesRepublic,
    /// Korea (the Republic of)
    KoreaRepublic,
    /// Kuwait
    Kuwait,
    /// Cayman Islands (the)
    CaymanIslands,
    /// Kazakhstan
    Kazakhstan,
    /// Lao People's Democratic Republic (the)
    LaoPeoplesDemocraticRepublic,
    /// Lebanon
    Lebanon,
    /// Saint Lucia
    SaintLucia,
    /// Liechtenstein
    Liechtenstein,
    /// Sri Lanka
    SriLanka,
    /// Liberia
    Liberia,
    /// Lesotho
    Lesotho,
    /// Lithuania
    Lithuania,
    /// Luxembourg
    Luxembourg,
    /// Latvia
    Latvia,
    /// Libya
    Libya,
    /// Morocco
    Morocco,
    /// Monaco
    Monaco,
    /// Moldova (the Republic of)
    MoldovaRepublic,
    /// Montenegro
    Montenegro,
    /// Saint Martin (French part)
    SaintMartinFrenchPart,
    /// Madagascar
    Madagascar,
    /// Marshall Islands (the)
    MarshallIslands,
    /// North Macedonia
    NorthMacedonia,
    /// Mali
    Mali,
    /// Myanmar
    Myanmar,
    /// Mongolia
    Mongolia,
    /// Macao
    Macao,
    /// Northern Mariana Islands (the)
    NorthernMarianaIslands,
    /// Martinique
    Martinique,
    /// Mauritania
    Mauritania,
    /// Montserrat
    Montserrat,
    /// Malta
    Malta,
    /// Mauritius
    Mauritius,
    /// Maldives
    Maldives,
    /// Malawi
    Malawi,
    /// Mexico
    Mexico,
    /// Malaysia
    Malaysia,
    /// Mozambique
    Mozambique,
    /// Namibia
    Namibia,
    /// New Caledonia
    NewCaledonia,
    /// Niger (the)
    Niger,
    /// Norfolk Island
    NorfolkIsland,
    /// Nigeria
    Nigeria,
    /// Nicaragua
    Nicaragua,
    /// Netherlands (the)
    Netherlands,
    /// Norway
    Norway,
    /// Nepal
    Nepal,
    /// Nauru
    Nauru,
    /// Niue
    Niue,
    /// New Zealand
    NewZealand,
    /// Oman
    Oman,
    /// Panama
    Panama,
    /// Peru
    Peru,
    /// French Polynesia
    FrenchPolynesia,
    /// Papua New Guinea
    PapuaNewGuinea,
    /// Philippines (the)
    Philippines,
    /// Pakistan
    Pakistan,
    /// Poland
    Poland,
    /// Saint Pierre and Miquelon
    SaintPierreAndMiquelon,
    /// Pitcairn
    Pitcairn,
    /// Puerto Rico
    PuertoRico,
    /// Palestine, State of
    PalestineState,
    /// Portugal
    Portugal,
    /// Palau
    Palau,
    /// Paraguay
    Paraguay,
    /// Qatar
    Qatar,
    /// Réunion
    Réunion,
    /// Romania
    Romania,
    /// Serbia
    Serbia,
    /// Russian Federation (the)
    RussianFederation,
    /// Rwanda
    Rwanda,
    /// Saudi Arabia
    SaudiArabia,
    /// Solomon Islands
    SolomonIslands,
    /// Seychelles
    Seychelles,
    /// Sudan (the)
    Sudan,
    /// Sweden
    Sweden,
    /// Singapore
    Singapore,
    /// Saint Helena, Ascension and Tristan da Cunha
    SaintHelenaAscensionAndTristanDaCunha,
    /// Slovenia
    Slovenia,
    /// Svalbard and Jan Mayen
    SvalbardAndJanMayen,
    /// Slovakia
    Slovakia,
    /// Sierra Leone
    SierraLeone,
    /// San Marino
    SanMarino,
    /// Senegal
    Senegal,
    /// Somalia
    Somalia,
    /// Suriname
    Suriname,
    /// South Sudan
    SouthSudan,
    /// Sao Tome and Principe
    SaoTomeAndPrincipe,
    /// El Salvador
    ElSalvador,
    /// Sint Maarten (Dutch part)
    SintMaartenDutchPart,
    /// Syrian Arab Republic (the)
    SyrianArabRepublic,
    /// Eswatini
    Eswatini,
    /// Turks and Caicos Islands (the)
    TurksAndCaicosIslands,
    /// Chad
    Chad,
    /// French Southern Territories (the)
    FrenchSouthernTerritories,
    /// Togo
    Togo,
    /// Thailand
    Thailand,
    /// Tajikistan
    Tajikistan,
    /// Tokelau
    Tokelau,
    /// Timor-Leste
    TimorLeste,
    /// Turkmenistan
    Turkmenistan,
    /// Tunisia
    Tunisia,
    /// Tonga
    Tonga,
    /// Türkiye
    Türkiye,
    /// Trinidad and Tobago
    TrinidadAndTobago,
    /// Tuvalu
    Tuvalu,
    /// Taiwan (Province of China)
    TaiwanProvinceChina,
    /// Tanzania, the United Republic of
    TanzaniaUnitedRepublic,
    /// Ukraine
    Ukraine,
    /// Uganda
    Uganda,
    /// United States Minor Outlying Islands (the)
    UnitedStatesMinorOutlyingIslands,
    /// United States of America (the)
    UnitedStatesAmerica,
    /// Uruguay
    Uruguay,
    /// Uzbekistan
    Uzbekistan,
    /// Holy See (the)
    HolySee,
    /// Saint Vincent and the Grenadines
    SaintVincentAndGrenadines,
    /// Venezuela (Bolivarian Republic of)
    VenezuelaBolivarianRepublic,
    /// Virgin Islands (British)
    VirginIslandsBritish,
    /// Virgin Islands (U.S.)
    VirginIslandsUS,
    /// Viet Nam
    VietNam,
    /// Vanuatu
    Vanuatu,
    /// Wallis and Futuna
    WallisAndFutuna,
    /// Samoa
    Samoa,
    /// Yemen
    Yemen,
    /// Mayotte
    Mayotte,
    /// South Africa
    SouthAfrica,
    /// Zambia
    Zambia,
    /// Zimbabwe
    Zimbabwe,
    /// Kosovo
    Kosovo,
    /// United Kingdom (Northern Ireland)
    UnitedKingdomNorthernIreland,
}

impl crate::Code for Country {
    fn code(&self) -> &str {
        match self {
            Country::Andorra => "AD",
            Country::UnitedArabEmirates => "AE",
            Country::Afghanistan => "AF",
            Country::AntiguaAndBarbuda => "AG",
            Country::Anguilla => "AI",
            Country::Albania => "AL",
            Country::Armenia => "AM",
            Country::Angola => "AO",
            Country::Antarctica => "AQ",
            Country::Argentina => "AR",
            Country::AmericanSamoa => "AS",
            Country::Austria => "AT",
            Country::Australia => "AU",
            Country::Aruba => "AW",
            Country::ÅlandIslands => "AX",
            Country::Azerbaijan => "AZ",
            Country::BosniaAndHerzegovina => "BA",
            Country::Barbados => "BB",
            Country::Bangladesh => "BD",
            Country::Belgium => "BE",
            Country::BurkinaFaso => "BF",
            Country::Bulgaria => "BG",
            Country::Bahrain => "BH",
            Country::Burundi => "BI",
            Country::Benin => "BJ",
            Country::SaintBarthélemy => "BL",
            Country::Bermuda => "BM",
            Country::BruneiDarussalam => "BN",
            Country::BoliviaPlurinationalState => "BO",
            Country::BonaireSintEustatiusAndSaba => "BQ",
            Country::Brazil => "BR",
            Country::Bahamas => "BS",
            Country::Bhutan => "BT",
            Country::BouvetIsland => "BV",
            Country::Botswana => "BW",
            Country::Belarus => "BY",
            Country::Belize => "BZ",
            Country::Canada => "CA",
            Country::CocosKeelingIslands => "CC",
            Country::CongoDemocraticRepublic => "CD",
            Country::CentralAfricanRepublic => "CF",
            Country::Congo => "CG",
            Country::Switzerland => "CH",
            Country::CôteDivoire => "CI",
            Country::CookIslands => "CK",
            Country::Chile => "CL",
            Country::Cameroon => "CM",
            Country::China => "CN",
            Country::Colombia => "CO",
            Country::CostaRica => "CR",
            Country::Cuba => "CU",
            Country::CaboVerde => "CV",
            Country::Curaçao => "CW",
            Country::ChristmasIsland => "CX",
            Country::Cyprus => "CY",
            Country::Czechia => "CZ",
            Country::Germany => "DE",
            Country::Djibouti => "DJ",
            Country::Denmark => "DK",
            Country::Dominica => "DM",
            Country::DominicanRepublic => "DO",
            Country::Algeria => "DZ",
            Country::Ecuador => "EC",
            Country::Estonia => "EE",
            Country::Egypt => "EG",
            Country::WesternSahara => "EH",
            Country::Eritrea => "ER",
            Country::Spain => "ES",
            Country::Ethiopia => "ET",
            Country::Finland => "FI",
            Country::Fiji => "FJ",
            Country::FalklandIslandsMalvinas => "FK",
            Country::MicronesiaFederatedStates => "FM",
            Country::FaroeIslands => "FO",
            Country::France => "FR",
            Country::Gabon => "GA",
            Country::UnitedKingdomGreatBritainAndNorthernIreland => "GB",
            Country::Grenada => "GD",
            Country::Georgia => "GE",
            Country::FrenchGuiana => "GF",
            Country::Guernsey => "GG",
            Country::Ghana => "GH",
            Country::Gibraltar => "GI",
            Country::Greenland => "GL",
            Country::Gambia => "GM",
            Country::Guinea => "GN",
            Country::Guadeloupe => "GP",
            Country::EquatorialGuinea => "GQ",
            Country::Greece => "GR",
            Country::SouthGeorgiaAndSouthSandwichIslands => "GS",
            Country::Guatemala => "GT",
            Country::Guam => "GU",
            Country::GuineaBissau => "GW",
            Country::Guyana => "GY",
            Country::HongKong => "HK",
            Country::HeardIslandAndMcdonaldIslands => "HM",
            Country::Honduras => "HN",
            Country::Croatia => "HR",
            Country::Haiti => "HT",
            Country::Hungary => "HU",
            Country::Indonesia => "ID",
            Country::Ireland => "IE",
            Country::Israel => "IL",
            Country::IsleMan => "IM",
            Country::India => "IN",
            Country::BritishIndianOceanTerritory => "IO",
            Country::Iraq => "IQ",
            Country::IranIslamicRepublic => "IR",
            Country::Iceland => "IS",
            Country::Italy => "IT",
            Country::Jersey => "JE",
            Country::Jamaica => "JM",
            Country::Jordan => "JO",
            Country::Japan => "JP",
            Country::Kenya => "KE",
            Country::Kyrgyzstan => "KG",
            Country::Cambodia => "KH",
            Country::Kiribati => "KI",
            Country::Comoros => "KM",
            Country::SaintKittsAndNevis => "KN",
            Country::KoreaDemocraticPeoplesRepublic => "KP",
            Country::KoreaRepublic => "KR",
            Country::Kuwait => "KW",
            Country::CaymanIslands => "KY",
            Country::Kazakhstan => "KZ",
            Country::LaoPeoplesDemocraticRepublic => "LA",
            Country::Lebanon => "LB",
            Country::SaintLucia => "LC",
            Country::Liechtenstein => "LI",
            Country::SriLanka => "LK",
            Country::Liberia => "LR",
            Country::Lesotho => "LS",
            Country::Lithuania => "LT",
            Country::Luxembourg => "LU",
            Country::Latvia => "LV",
            Country::Libya => "LY",
            Country::Morocco => "MA",
            Country::Monaco => "MC",
            Country::MoldovaRepublic => "MD",
            Country::Montenegro => "ME",
            Country::SaintMartinFrenchPart => "MF",
            Country::Madagascar => "MG",
            Country::MarshallIslands => "MH",
            Country::NorthMacedonia => "MK",
            Country::Mali => "ML",
            Country::Myanmar => "MM",
            Country::Mongolia => "MN",
            Country::Macao => "MO",
            Country::NorthernMarianaIslands => "MP",
            Country::Martinique => "MQ",
            Country::Mauritania => "MR",
            Country::Montserrat => "MS",
            Country::Malta => "MT",
            Country::Mauritius => "MU",
            Country::Maldives => "MV",
            Country::Malawi => "MW",
            Country::Mexico => "MX",
            Country::Malaysia => "MY",
            Country::Mozambique => "MZ",
            Country::Namibia => "NA",
            Country::NewCaledonia => "NC",
            Country::Niger => "NE",
            Country::NorfolkIsland => "NF",
            Country::Nigeria => "NG",
            Country::Nicaragua => "NI",
            Country::Netherlands => "NL",
            Country::Norway => "NO",
            Country::Nepal => "NP",
            Country::Nauru => "NR",
            Country::Niue => "NU",
            Country::NewZealand => "NZ",
            Country::Oman => "OM",
            Country::Panama => "PA",
            Country::Peru => "PE",
            Country::FrenchPolynesia => "PF",
            Country::PapuaNewGuinea => "PG",
            Country::Philippines => "PH",
            Country::Pakistan => "PK",
            Country::Poland => "PL",
            Country::SaintPierreAndMiquelon => "PM",
            Country::Pitcairn => "PN",
            Country::PuertoRico => "PR",
            Country::PalestineState => "PS",
            Country::Portugal => "PT",
            Country::Palau => "PW",
            Country::Paraguay => "PY",
            Country::Qatar => "QA",
            Country::Réunion => "RE",
            Country::Romania => "RO",
            Country::Serbia => "RS",
            Country::RussianFederation => "RU",
            Country::Rwanda => "RW",
            Country::SaudiArabia => "SA",
            Country::SolomonIslands => "SB",
            Country::Seychelles => "SC",
            Country::Sudan => "SD",
            Country::Sweden => "SE",
            Country::Singapore => "SG",
            Country::SaintHelenaAscensionAndTristanDaCunha => "SH",
            Country::Slovenia => "SI",
            Country::SvalbardAndJanMayen => "SJ",
            Country::Slovakia => "SK",
            Country::SierraLeone => "SL",
            Country::SanMarino => "SM",
            Country::Senegal => "SN",
            Country::Somalia => "SO",
            Country::Suriname => "SR",
            Country::SouthSudan => "SS",
            Country::SaoTomeAndPrincipe => "ST",
            Country::ElSalvador => "SV",
            Country::SintMaartenDutchPart => "SX",
            Country::SyrianArabRepublic => "SY",
            Country::Eswatini => "SZ",
            Country::TurksAndCaicosIslands => "TC",
            Country::Chad => "TD",
            Country::FrenchSouthernTerritories => "TF",
            Country::Togo => "TG",
            Country::Thailand => "TH",
            Country::Tajikistan => "TJ",
            Country::Tokelau => "TK",
            Country::TimorLeste => "TL",
            Country::Turkmenistan => "TM",
            Country::Tunisia => "TN",
            Country::Tonga => "TO",
            Country::Türkiye => "TR",
            Country::TrinidadAndTobago => "TT",
            Country::Tuvalu => "TV",
            Country::TaiwanProvinceChina => "TW",
            Country::TanzaniaUnitedRepublic => "TZ",
            Country::Ukraine => "UA",
            Country::Uganda => "UG",
            Country::UnitedStatesMinorOutlyingIslands => "UM",
            Country::UnitedStatesAmerica => "US",
            Country::Uruguay => "UY",
            Country::Uzbekistan => "UZ",
            Country::HolySee => "VA",
            Country::SaintVincentAndGrenadines => "VC",
            Country::VenezuelaBolivarianRepublic => "VE",
            Country::VirginIslandsBritish => "VG",
            Country::VirginIslandsUS => "VI",
            Country::VietNam => "VN",
            Country::Vanuatu => "VU",
            Country::WallisAndFutuna => "WF",
            Country::Samoa => "WS",
            Country::Yemen => "YE",
            Country::Mayotte => "YT",
            Country::SouthAfrica => "ZA",
            Country::Zambia => "ZM",
            Country::Zimbabwe => "ZW",
            Country::Kosovo => "1A",
            Country::UnitedKingdomNorthernIreland => "XI",
        }
    }
}

impl crate::Description for Country {
    fn description(&self) -> &str {
        match self {
            Country::Andorra => "Andorra",
            Country::UnitedArabEmirates => "United Arab Emirates (the)",
            Country::Afghanistan => "Afghanistan",
            Country::AntiguaAndBarbuda => "Antigua and Barbuda",
            Country::Anguilla => "Anguilla",
            Country::Albania => "Albania",
            Country::Armenia => "Armenia",
            Country::Angola => "Angola",
            Country::Antarctica => "Antarctica",
            Country::Argentina => "Argentina",
            Country::AmericanSamoa => "American Samoa",
            Country::Austria => "Austria",
            Country::Australia => "Australia",
            Country::Aruba => "Aruba",
            Country::ÅlandIslands => "Åland Islands",
            Country::Azerbaijan => "Azerbaijan",
            Country::BosniaAndHerzegovina => "Bosnia and Herzegovina",
            Country::Barbados => "Barbados",
            Country::Bangladesh => "Bangladesh",
            Country::Belgium => "Belgium",
            Country::BurkinaFaso => "Burkina Faso",
            Country::Bulgaria => "Bulgaria",
            Country::Bahrain => "Bahrain",
            Country::Burundi => "Burundi",
            Country::Benin => "Benin",
            Country::SaintBarthélemy => "Saint Barthélemy",
            Country::Bermuda => "Bermuda",
            Country::BruneiDarussalam => "Brunei Darussalam",
            Country::BoliviaPlurinationalState => "Bolivia (Plurinational State of)",
            Country::BonaireSintEustatiusAndSaba => "Bonaire, Sint Eustatius and Saba",
            Country::Brazil => "Brazil",
            Country::Bahamas => "Bahamas (the)",
            Country::Bhutan => "Bhutan",
            Country::BouvetIsland => "Bouvet Island",
            Country::Botswana => "Botswana",
            Country::Belarus => "Belarus",
            Country::Belize => "Belize",
            Country::Canada => "Canada",
            Country::CocosKeelingIslands => "Cocos (Keeling) Islands (the)",
            Country::CongoDemocraticRepublic => "Congo (the Democratic Republic of the)",
            Country::CentralAfricanRepublic => "Central African Republic (the)",
            Country::Congo => "Congo (the)",
            Country::Switzerland => "Switzerland",
            Country::CôteDivoire => "Côte d'Ivoire",
            Country::CookIslands => "Cook Islands (the)",
            Country::Chile => "Chile",
            Country::Cameroon => "Cameroon",
            Country::China => "China",
            Country::Colombia => "Colombia",
            Country::CostaRica => "Costa Rica",
            Country::Cuba => "Cuba",
            Country::CaboVerde => "Cabo Verde",
            Country::Curaçao => "Curaçao",
            Country::ChristmasIsland => "Christmas Island",
            Country::Cyprus => "Cyprus",
            Country::Czechia => "Czechia",
            Country::Germany => "Germany",
            Country::Djibouti => "Djibouti",
            Country::Denmark => "Denmark",
            Country::Dominica => "Dominica",
            Country::DominicanRepublic => "Dominican Republic (the)",
            Country::Algeria => "Algeria",
            Country::Ecuador => "Ecuador",
            Country::Estonia => "Estonia",
            Country::Egypt => "Egypt",
            Country::WesternSahara => "Western Sahara*",
            Country::Eritrea => "Eritrea",
            Country::Spain => "Spain",
            Country::Ethiopia => "Ethiopia",
            Country::Finland => "Finland",
            Country::Fiji => "Fiji",
            Country::FalklandIslandsMalvinas => "Falkland Islands (the) [Malvinas]",
            Country::MicronesiaFederatedStates => "Micronesia (Federated States of)",
            Country::FaroeIslands => "Faroe Islands (the)",
            Country::France => "France",
            Country::Gabon => "Gabon",
            Country::UnitedKingdomGreatBritainAndNorthernIreland => {
                "United Kingdom of Great Britain and Northern Ireland (the)"
            }
            Country::Grenada => "Grenada",
            Country::Georgia => "Georgia",
            Country::FrenchGuiana => "French Guiana",
            Country::Guernsey => "Guernsey",
            Country::Ghana => "Ghana",
            Country::Gibraltar => "Gibraltar",
            Country::Greenland => "Greenland",
            Country::Gambia => "Gambia (the)",
            Country::Guinea => "Guinea",
            Country::Guadeloupe => "Guadeloupe",
            Country::EquatorialGuinea => "Equatorial Guinea",
            Country::Greece => "Greece",
            Country::SouthGeorgiaAndSouthSandwichIslands => {
                "South Georgia and the South Sandwich Islands"
            }
            Country::Guatemala => "Guatemala",
            Country::Guam => "Guam",
            Country::GuineaBissau => "Guinea-Bissau",
            Country::Guyana => "Guyana",
            Country::HongKong => "Hong Kong",
            Country::HeardIslandAndMcdonaldIslands => "Heard Island and McDonald Islands",
            Country::Honduras => "Honduras",
            Country::Croatia => "Croatia",
            Country::Haiti => "Haiti",
            Country::Hungary => "Hungary",
            Country::Indonesia => "Indonesia",
            Country::Ireland => "Ireland",
            Country::Israel => "Israel",
            Country::IsleMan => "Isle of Man",
            Country::India => "India",
            Country::BritishIndianOceanTerritory => "British Indian Ocean Territory (the)",
            Country::Iraq => "Iraq",
            Country::IranIslamicRepublic => "Iran (Islamic Republic of)",
            Country::Iceland => "Iceland",
            Country::Italy => "Italy",
            Country::Jersey => "Jersey",
            Country::Jamaica => "Jamaica",
            Country::Jordan => "Jordan",
            Country::Japan => "Japan",
            Country::Kenya => "Kenya",
            Country::Kyrgyzstan => "Kyrgyzstan",
            Country::Cambodia => "Cambodia",
            Country::Kiribati => "Kiribati",
            Country::Comoros => "Comoros (the)",
            Country::SaintKittsAndNevis => "Saint Kitts and Nevis",
            Country::KoreaDemocraticPeoplesRepublic => {
                "Korea (the Democratic People's Republic of)"
            }
            Country::KoreaRepublic => "Korea (the Republic of)",
            Country::Kuwait => "Kuwait",
            Country::CaymanIslands => "Cayman Islands (the)",
            Country::Kazakhstan => "Kazakhstan",
            Country::LaoPeoplesDemocraticRepublic => "Lao People's Democratic Republic (the)",
            Country::Lebanon => "Lebanon",
            Country::SaintLucia => "Saint Lucia",
            Country::Liechtenstein => "Liechtenstein",
            Country::SriLanka => "Sri Lanka",
            Country::Liberia => "Liberia",
            Country::Lesotho => "Lesotho",
            Country::Lithuania => "Lithuania",
            Country::Luxembourg => "Luxembourg",
            Country::Latvia => "Latvia",
            Country::Libya => "Libya",
            Country::Morocco => "Morocco",
            Country::Monaco => "Monaco",
            Country::MoldovaRepublic => "Moldova (the Republic of)",
            Country::Montenegro => "Montenegro",
            Country::SaintMartinFrenchPart => "Saint Martin (French part)",
            Country::Madagascar => "Madagascar",
            Country::MarshallIslands => "Marshall Islands (the)",
            Country::NorthMacedonia => "North Macedonia",
            Country::Mali => "Mali",
            Country::Myanmar => "Myanmar",
            Country::Mongolia => "Mongolia",
            Country::Macao => "Macao",
            Country::NorthernMarianaIslands => "Northern Mariana Islands (the)",
            Country::Martinique => "Martinique",
            Country::Mauritania => "Mauritania",
            Country::Montserrat => "Montserrat",
            Country::Malta => "Malta",
            Country::Mauritius => "Mauritius",
            Country::Maldives => "Maldives",
            Country::Malawi => "Malawi",
            Country::Mexico => "Mexico",
            Country::Malaysia => "Malaysia",
            Country::Mozambique => "Mozambique",
            Country::Namibia => "Namibia",
            Country::NewCaledonia => "New Caledonia",
            Country::Niger => "Niger (the)",
            Country::NorfolkIsland => "Norfolk Island",
            Country::Nigeria => "Nigeria",
            Country::Nicaragua => "Nicaragua",
            Country::Netherlands => "Netherlands (the)",
            Country::Norway => "Norway",
            Country::Nepal => "Nepal",
            Country::Nauru => "Nauru",
            Country::Niue => "Niue",
            Country::NewZealand => "New Zealand",
            Country::Oman => "Oman",
            Country::Panama => "Panama",
            Country::Peru => "Peru",
            Country::FrenchPolynesia => "French Polynesia",
            Country::PapuaNewGuinea => "Papua New Guinea",
            Country::Philippines => "Philippines (the)",
            Country::Pakistan => "Pakistan",
            Country::Poland => "Poland",
            Country::SaintPierreAndMiquelon => "Saint Pierre and Miquelon",
            Country::Pitcairn => "Pitcairn",
            Country::PuertoRico => "Puerto Rico",
            Country::PalestineState => "Palestine, State of",
            Country::Portugal => "Portugal",
            Country::Palau => "Palau",
            Country::Paraguay => "Paraguay",
            Country::Qatar => "Qatar",
            Country::Réunion => "Réunion",
            Country::Romania => "Romania",
            Country::Serbia => "Serbia",
            Country::RussianFederation => "Russian Federation (the)",
            Country::Rwanda => "Rwanda",
            Country::SaudiArabia => "Saudi Arabia",
            Country::SolomonIslands => "Solomon Islands",
            Country::Seychelles => "Seychelles",
            Country::Sudan => "Sudan (the)",
            Country::Sweden => "Sweden",
            Country::Singapore => "Singapore",
            Country::SaintHelenaAscensionAndTristanDaCunha => {
                "Saint Helena, Ascension and Tristan da Cunha"
            }
            Country::Slovenia => "Slovenia",
            Country::SvalbardAndJanMayen => "Svalbard and Jan Mayen",
            Country::Slovakia => "Slovakia",
            Country::SierraLeone => "Sierra Leone",
            Country::SanMarino => "San Marino",
            Country::Senegal => "Senegal",
            Country::Somalia => "Somalia",
            Country::Suriname => "Suriname",
            Country::SouthSudan => "South Sudan",
            Country::SaoTomeAndPrincipe => "Sao Tome and Principe",
            Country::ElSalvador => "El Salvador",
            Country::SintMaartenDutchPart => "Sint Maarten (Dutch part)",
            Country::SyrianArabRepublic => "Syrian Arab Republic (the)",
            Country::Eswatini => "Eswatini",
            Country::TurksAndCaicosIslands => "Turks and Caicos Islands (the)",
            Country::Chad => "Chad",
            Country::FrenchSouthernTerritories => "French Southern Territories (the)",
            Country::Togo => "Togo",
            Country::Thailand => "Thailand",
            Country::Tajikistan => "Tajikistan",
            Country::Tokelau => "Tokelau",
            Country::TimorLeste => "Timor-Leste",
            Country::Turkmenistan => "Turkmenistan",
            Country::Tunisia => "Tunisia",
            Country::Tonga => "Tonga",
            Country::Türkiye => "Türkiye",
            Country::TrinidadAndTobago => "Trinidad and Tobago",
            Country::Tuvalu => "Tuvalu",
            Country::TaiwanProvinceChina => "Taiwan (Province of China)",
            Country::TanzaniaUnitedRepublic => "Tanzania, the United Republic of",
            Country::Ukraine => "Ukraine",
            Country::Uganda => "Uganda",
            Country::UnitedStatesMinorOutlyingIslands => {
                "United States Minor Outlying Islands (the)"
            }
            Country::UnitedStatesAmerica => "United States of America (the)",
            Country::Uruguay => "Uruguay",
            Country::Uzbekistan => "Uzbekistan",
            Country::HolySee => "Holy See (the)",
            Country::SaintVincentAndGrenadines => "Saint Vincent and the Grenadines",
            Country::VenezuelaBolivarianRepublic => "Venezuela (Bolivarian Republic of)",
            Country::VirginIslandsBritish => "Virgin Islands (British)",
            Country::VirginIslandsUS => "Virgin Islands (U.S.)",
            Country::VietNam => "Viet Nam",
            Country::Vanuatu => "Vanuatu",
            Country::WallisAndFutuna => "Wallis and Futuna",
            Country::Samoa => "Samoa",
            Country::Yemen => "Yemen",
            Country::Mayotte => "Mayotte",
            Country::SouthAfrica => "South Africa",
            Country::Zambia => "Zambia",
            Country::Zimbabwe => "Zimbabwe",
            Country::Kosovo => "Kosovo",
            Country::UnitedKingdomNorthernIreland => "United Kingdom (Northern Ireland)",
        }
    }
}

impl crate::FromCode for Country {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "AD" => Some(Country::Andorra),
            "AE" => Some(Country::UnitedArabEmirates),
            "AF" => Some(Country::Afghanistan),
            "AG" => Some(Country::AntiguaAndBarbuda),
            "AI" => Some(Country::Anguilla),
            "AL" => Some(Country::Albania),
            "AM" => Some(Country::Armenia),
            "AO" => Some(Country::Angola),
            "AQ" => Some(Country::Antarctica),
            "AR" => Some(Country::Argentina),
            "AS" => Some(Country::AmericanSamoa),
            "AT" => Some(Country::Austria),
            "AU" => Some(Country::Australia),
            "AW" => Some(Country::Aruba),
            "AX" => Some(Country::ÅlandIslands),
            "AZ" => Some(Country::Azerbaijan),
            "BA" => Some(Country::BosniaAndHerzegovina),
            "BB" => Some(Country::Barbados),
            "BD" => Some(Country::Bangladesh),
            "BE" => Some(Country::Belgium),
            "BF" => Some(Country::BurkinaFaso),
            "BG" => Some(Country::Bulgaria),
            "BH" => Some(Country::Bahrain),
            "BI" => Some(Country::Burundi),
            "BJ" => Some(Country::Benin),
            "BL" => Some(Country::SaintBarthélemy),
            "BM" => Some(Country::Bermuda),
            "BN" => Some(Country::BruneiDarussalam),
            "BO" => Some(Country::BoliviaPlurinationalState),
            "BQ" => Some(Country::BonaireSintEustatiusAndSaba),
            "BR" => Some(Country::Brazil),
            "BS" => Some(Country::Bahamas),
            "BT" => Some(Country::Bhutan),
            "BV" => Some(Country::BouvetIsland),
            "BW" => Some(Country::Botswana),
            "BY" => Some(Country::Belarus),
            "BZ" => Some(Country::Belize),
            "CA" => Some(Country::Canada),
            "CC" => Some(Country::CocosKeelingIslands),
            "CD" => Some(Country::CongoDemocraticRepublic),
            "CF" => Some(Country::CentralAfricanRepublic),
            "CG" => Some(Country::Congo),
            "CH" => Some(Country::Switzerland),
            "CI" => Some(Country::CôteDivoire),
            "CK" => Some(Country::CookIslands),
            "CL" => Some(Country::Chile),
            "CM" => Some(Country::Cameroon),
            "CN" => Some(Country::China),
            "CO" => Some(Country::Colombia),
            "CR" => Some(Country::CostaRica),
            "CU" => Some(Country::Cuba),
            "CV" => Some(Country::CaboVerde),
            "CW" => Some(Country::Curaçao),
            "CX" => Some(Country::ChristmasIsland),
            "CY" => Some(Country::Cyprus),
            "CZ" => Some(Country::Czechia),
            "DE" => Some(Country::Germany),
            "DJ" => Some(Country::Djibouti),
            "DK" => Some(Country::Denmark),
            "DM" => Some(Country::Dominica),
            "DO" => Some(Country::DominicanRepublic),
            "DZ" => Some(Country::Algeria),
            "EC" => Some(Country::Ecuador),
            "EE" => Some(Country::Estonia),
            "EG" => Some(Country::Egypt),
            "EH" => Some(Country::WesternSahara),
            "ER" => Some(Country::Eritrea),
            "ES" => Some(Country::Spain),
            "ET" => Some(Country::Ethiopia),
            "FI" => Some(Country::Finland),
            "FJ" => Some(Country::Fiji),
            "FK" => Some(Country::FalklandIslandsMalvinas),
            "FM" => Some(Country::MicronesiaFederatedStates),
            "FO" => Some(Country::FaroeIslands),
            "FR" => Some(Country::France),
            "GA" => Some(Country::Gabon),
            "GB" => Some(Country::UnitedKingdomGreatBritainAndNorthernIreland),
            "GD" => Some(Country::Grenada),
            "GE" => Some(Country::Georgia),
            "GF" => Some(Country::FrenchGuiana),
            "GG" => Some(Country::Guernsey),
            "GH" => Some(Country::Ghana),
            "GI" => Some(Country::Gibraltar),
            "GL" => Some(Country::Greenland),
            "GM" => Some(Country::Gambia),
            "GN" => Some(Country::Guinea),
            "GP" => Some(Country::Guadeloupe),
            "GQ" => Some(Country::EquatorialGuinea),
            "GR" => Some(Country::Greece),
            "GS" => Some(Country::SouthGeorgiaAndSouthSandwichIslands),
            "GT" => Some(Country::Guatemala),
            "GU" => Some(Country::Guam),
            "GW" => Some(Country::GuineaBissau),
            "GY" => Some(Country::Guyana),
            "HK" => Some(Country::HongKong),
            "HM" => Some(Country::HeardIslandAndMcdonaldIslands),
            "HN" => Some(Country::Honduras),
            "HR" => Some(Country::Croatia),
            "HT" => Some(Country::Haiti),
            "HU" => Some(Country::Hungary),
            "ID" => Some(Country::Indonesia),
            "IE" => Some(Country::Ireland),
            "IL" => Some(Country::Israel),
            "IM" => Some(Country::IsleMan),
            "IN" => Some(Country::India),
            "IO" => Some(Country::BritishIndianOceanTerritory),
            "IQ" => Some(Country::Iraq),
            "IR" => Some(Country::IranIslamicRepublic),
            "IS" => Some(Country::Iceland),
            "IT" => Some(Country::Italy),
            "JE" => Some(Country::Jersey),
            "JM" => Some(Country::Jamaica),
            "JO" => Some(Country::Jordan),
            "JP" => Some(Country::Japan),
            "KE" => Some(Country::Kenya),
            "KG" => Some(Country::Kyrgyzstan),
            "KH" => Some(Country::Cambodia),
            "KI" => Some(Country::Kiribati),
            "KM" => Some(Country::Comoros),
            "KN" => Some(Country::SaintKittsAndNevis),
            "KP" => Some(Country::KoreaDemocraticPeoplesRepublic),
            "KR" => Some(Country::KoreaRepublic),
            "KW" => Some(Country::Kuwait),
            "KY" => Some(Country::CaymanIslands),
            "KZ" => Some(Country::Kazakhstan),
            "LA" => Some(Country::LaoPeoplesDemocraticRepublic),
            "LB" => Some(Country::Lebanon),
            "LC" => Some(Country::SaintLucia),
            "LI" => Some(Country::Liechtenstein),
            "LK" => Some(Country::SriLanka),
            "LR" => Some(Country::Liberia),
            "LS" => Some(Country::Lesotho),
            "LT" => Some(Country::Lithuania),
            "LU" => Some(Country::Luxembourg),
            "LV" => Some(Country::Latvia),
            "LY" => Some(Country::Libya),
            "MA" => Some(Country::Morocco),
            "MC" => Some(Country::Monaco),
            "MD" => Some(Country::MoldovaRepublic),
            "ME" => Some(Country::Montenegro),
            "MF" => Some(Country::SaintMartinFrenchPart),
            "MG" => Some(Country::Madagascar),
            "MH" => Some(Country::MarshallIslands),
            "MK" => Some(Country::NorthMacedonia),
            "ML" => Some(Country::Mali),
            "MM" => Some(Country::Myanmar),
            "MN" => Some(Country::Mongolia),
            "MO" => Some(Country::Macao),
            "MP" => Some(Country::NorthernMarianaIslands),
            "MQ" => Some(Country::Martinique),
            "MR" => Some(Country::Mauritania),
            "MS" => Some(Country::Montserrat),
            "MT" => Some(Country::Malta),
            "MU" => Some(Country::Mauritius),
            "MV" => Some(Country::Maldives),
            "MW" => Some(Country::Malawi),
            "MX" => Some(Country::Mexico),
            "MY" => Some(Country::Malaysia),
            "MZ" => Some(Country::Mozambique),
            "NA" => Some(Country::Namibia),
            "NC" => Some(Country::NewCaledonia),
            "NE" => Some(Country::Niger),
            "NF" => Some(Country::NorfolkIsland),
            "NG" => Some(Country::Nigeria),
            "NI" => Some(Country::Nicaragua),
            "NL" => Some(Country::Netherlands),
            "NO" => Some(Country::Norway),
            "NP" => Some(Country::Nepal),
            "NR" => Some(Country::Nauru),
            "NU" => Some(Country::Niue),
            "NZ" => Some(Country::NewZealand),
            "OM" => Some(Country::Oman),
            "PA" => Some(Country::Panama),
            "PE" => Some(Country::Peru),
            "PF" => Some(Country::FrenchPolynesia),
            "PG" => Some(Country::PapuaNewGuinea),
            "PH" => Some(Country::Philippines),
            "PK" => Some(Country::Pakistan),
            "PL" => Some(Country::Poland),
            "PM" => Some(Country::SaintPierreAndMiquelon),
            "PN" => Some(Country::Pitcairn),
            "PR" => Some(Country::PuertoRico),
            "PS" => Some(Country::PalestineState),
            "PT" => Some(Country::Portugal),
            "PW" => Some(Country::Palau),
            "PY" => Some(Country::Paraguay),
            "QA" => Some(Country::Qatar),
            "RE" => Some(Country::Réunion),
            "RO" => Some(Country::Romania),
            "RS" => Some(Country::Serbia),
            "RU" => Some(Country::RussianFederation),
            "RW" => Some(Country::Rwanda),
            "SA" => Some(Country::SaudiArabia),
            "SB" => Some(Country::SolomonIslands),
            "SC" => Some(Country::Seychelles),
            "SD" => Some(Country::Sudan),
            "SE" => Some(Country::Sweden),
            "SG" => Some(Country::Singapore),
            "SH" => Some(Country::SaintHelenaAscensionAndTristanDaCunha),
            "SI" => Some(Country::Slovenia),
            "SJ" => Some(Country::SvalbardAndJanMayen),
            "SK" => Some(Country::Slovakia),
            "SL" => Some(Country::SierraLeone),
            "SM" => Some(Country::SanMarino),
            "SN" => Some(Country::Senegal),
            "SO" => Some(Country::Somalia),
            "SR" => Some(Country::Suriname),
            "SS" => Some(Country::SouthSudan),
            "ST" => Some(Country::SaoTomeAndPrincipe),
            "SV" => Some(Country::ElSalvador),
            "SX" => Some(Country::SintMaartenDutchPart),
            "SY" => Some(Country::SyrianArabRepublic),
            "SZ" => Some(Country::Eswatini),
            "TC" => Some(Country::TurksAndCaicosIslands),
            "TD" => Some(Country::Chad),
            "TF" => Some(Country::FrenchSouthernTerritories),
            "TG" => Some(Country::Togo),
            "TH" => Some(Country::Thailand),
            "TJ" => Some(Country::Tajikistan),
            "TK" => Some(Country::Tokelau),
            "TL" => Some(Country::TimorLeste),
            "TM" => Some(Country::Turkmenistan),
            "TN" => Some(Country::Tunisia),
            "TO" => Some(Country::Tonga),
            "TR" => Some(Country::Türkiye),
            "TT" => Some(Country::TrinidadAndTobago),
            "TV" => Some(Country::Tuvalu),
            "TW" => Some(Country::TaiwanProvinceChina),
            "TZ" => Some(Country::TanzaniaUnitedRepublic),
            "UA" => Some(Country::Ukraine),
            "UG" => Some(Country::Uganda),
            "UM" => Some(Country::UnitedStatesMinorOutlyingIslands),
            "US" => Some(Country::UnitedStatesAmerica),
            "UY" => Some(Country::Uruguay),
            "UZ" => Some(Country::Uzbekistan),
            "VA" => Some(Country::HolySee),
            "VC" => Some(Country::SaintVincentAndGrenadines),
            "VE" => Some(Country::VenezuelaBolivarianRepublic),
            "VG" => Some(Country::VirginIslandsBritish),
            "VI" => Some(Country::VirginIslandsUS),
            "VN" => Some(Country::VietNam),
            "VU" => Some(Country::Vanuatu),
            "WF" => Some(Country::WallisAndFutuna),
            "WS" => Some(Country::Samoa),
            "YE" => Some(Country::Yemen),
            "YT" => Some(Country::Mayotte),
            "ZA" => Some(Country::SouthAfrica),
            "ZM" => Some(Country::Zambia),
            "ZW" => Some(Country::Zimbabwe),
            "1A" => Some(Country::Kosovo),
            "XI" => Some(Country::UnitedKingdomNorthernIreland),
            _ => None,
        }
    }
}
