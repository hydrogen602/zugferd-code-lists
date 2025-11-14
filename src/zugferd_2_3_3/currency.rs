#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Currency {
    /// UAE Dirham
    UaeDirham,
    /// Afghani
    Afghani,
    /// Lek
    Lek,
    /// Armenian Dram
    ArmenianDram,
    /// Netherlands Antillean Guilder
    NetherlandsAntilleanGuilder,
    /// Kwanza
    Kwanza,
    /// Argentine Peso
    ArgentinePeso,
    /// Australian Dollar
    AustralianDollar,
    /// Aruban Florin
    ArubanFlorin,
    /// Azerbaijan Manat
    AzerbaijanManat,
    /// Convertible Mark
    ConvertibleMark,
    /// Barbados Dollar
    BarbadosDollar,
    /// Taka
    Taka,
    /// Bulgarian Lev
    BulgarianLev,
    /// Bahraini Dinar
    BahrainiDinar,
    /// Burundi Franc
    BurundiFranc,
    /// Bermudian Dollar
    BermudianDollar,
    /// Brunei Dollar
    BruneiDollar,
    /// Boliviano
    Boliviano,
    /// Mvdol
    Mvdol,
    /// Brazilian Real
    BrazilianReal,
    /// Bahamian Dollar
    BahamianDollar,
    /// Ngultrum
    Ngultrum,
    /// Pula
    Pula,
    /// Belarusian Ruble
    BelarusianRuble,
    /// Belize Dollar
    BelizeDollar,
    /// Canadian Dollar
    CanadianDollar,
    /// Congolese Franc
    CongoleseFranc,
    /// WIR Euro
    WirEuro,
    /// Swiss Franc
    SwissFranc,
    /// WIR Franc
    WirFranc,
    /// Unidad de Fomento
    UnidadDeFomento,
    /// Chilean Peso
    ChileanPeso,
    /// Yuan Renminbi
    YuanRenminbi,
    /// Colombian Peso
    ColombianPeso,
    /// Unidad de Valor Real
    UnidadDeValorReal,
    /// Costa Rican Colon
    CostaRicanColon,
    /// Cuban Peso
    CubanPeso,
    /// Cabo Verde Escudo
    CaboVerdeEscudo,
    /// Czech Koruna
    CzechKoruna,
    /// Djibouti Franc
    DjiboutiFranc,
    /// Danish Krone
    DanishKrone,
    /// Dominican Peso
    DominicanPeso,
    /// Algerian Dinar
    AlgerianDinar,
    /// Egyptian Pound
    EgyptianPound,
    /// Nakfa
    Nakfa,
    /// Ethiopian Birr
    EthiopianBirr,
    /// Euro
    Euro,
    /// Fiji Dollar
    FijiDollar,
    /// Falkland Islands Pound
    FalklandIslandsPound,
    /// Pound Sterling
    PoundSterling,
    /// Lari
    Lari,
    /// Ghana Cedi
    GhanaCedi,
    /// Gibraltar Pound
    GibraltarPound,
    /// Dalasi
    Dalasi,
    /// Guinean Franc
    GuineanFranc,
    /// Quetzal
    Quetzal,
    /// Guyana Dollar
    GuyanaDollar,
    /// Hong Kong Dollar
    HongKongDollar,
    /// Lempira
    Lempira,
    /// Gourde
    Gourde,
    /// Forint
    Forint,
    /// Rupiah
    Rupiah,
    /// New Israeli Sheqel
    NewIsraeliSheqel,
    /// Indian Rupee
    IndianRupee,
    /// Iraqi Dinar
    IraqiDinar,
    /// Iranian Rial
    IranianRial,
    /// Iceland Krona
    IcelandKrona,
    /// Jamaican Dollar
    JamaicanDollar,
    /// Jordanian Dinar
    JordanianDinar,
    /// Yen
    Yen,
    /// Kenyan Shilling
    KenyanShilling,
    /// Som
    Som,
    /// Riel
    Riel,
    /// Comorian Franc
    ComorianFranc,
    /// North Korean Won
    NorthKoreanWon,
    /// Won
    Won,
    /// Kuwaiti Dinar
    KuwaitiDinar,
    /// Cayman Islands Dollar
    CaymanIslandsDollar,
    /// Tenge
    Tenge,
    /// Lao Kip
    LaoKip,
    /// Lebanese Pound
    LebanesePound,
    /// Sri Lanka Rupee
    SriLankaRupee,
    /// Liberian Dollar
    LiberianDollar,
    /// Loti
    Loti,
    /// Libyan Dinar
    LibyanDinar,
    /// Moroccan Dirham
    MoroccanDirham,
    /// Moldovan Leu
    MoldovanLeu,
    /// Malagasy Ariary
    MalagasyAriary,
    /// Denar
    Denar,
    /// Kyat
    Kyat,
    /// Tugrik
    Tugrik,
    /// Pataca
    Pataca,
    /// Ouguiya
    Ouguiya,
    /// Mauritius Rupee
    MauritiusRupee,
    /// Rufiyaa
    Rufiyaa,
    /// Malawi Kwacha
    MalawiKwacha,
    /// Mexican Peso
    MexicanPeso,
    /// Mexican Unidad de Inversion (UDI)
    MexicanUnidadDeInversionUdi,
    /// Malaysian Ringgit
    MalaysianRinggit,
    /// Mozambique Metical
    MozambiqueMetical,
    /// Namibia Dollar
    NamibiaDollar,
    /// Naira
    Naira,
    /// Cordoba Oro
    CordobaOro,
    /// Norwegian Krone
    NorwegianKrone,
    /// Nepalese Rupee
    NepaleseRupee,
    /// New Zealand Dollar
    NewZealandDollar,
    /// Rial Omani
    RialOmani,
    /// Balboa
    Balboa,
    /// Sol
    Sol,
    /// Kina
    Kina,
    /// Philippine Peso
    PhilippinePeso,
    /// Pakistan Rupee
    PakistanRupee,
    /// Zloty
    Zloty,
    /// Guarani
    Guarani,
    /// Qatari Rial
    QatariRial,
    /// Romanian Leu
    RomanianLeu,
    /// Serbian Dinar
    SerbianDinar,
    /// Russian Ruble
    RussianRuble,
    /// Rwanda Franc
    RwandaFranc,
    /// Saudi Riyal
    SaudiRiyal,
    /// Solomon Islands Dollar
    SolomonIslandsDollar,
    /// Seychelles Rupee
    SeychellesRupee,
    /// Sudanese Pound
    SudanesePound,
    /// Swedish Krona
    SwedishKrona,
    /// Singapore Dollar
    SingaporeDollar,
    /// Saint Helena Pound
    SaintHelenaPound,
    /// Sierra Leone (new valuation 2022)
    SierraLeoneNewValuation2022,
    /// Somali Shilling
    SomaliShilling,
    /// Surinam Dollar
    SurinamDollar,
    /// South Sudanese Pound
    SouthSudanesePound,
    /// Dobra
    Dobra,
    /// El Salvador Colon
    ElSalvadorColon,
    /// Syrian Pound
    SyrianPound,
    /// Lilangeni
    Lilangeni,
    /// Baht
    Baht,
    /// Somoni
    Somoni,
    /// Turkmenistan New Manat
    TurkmenistanNewManat,
    /// Tunisian Dinar
    TunisianDinar,
    /// Pa’anga
    PaAnga,
    /// Turkish Lira
    TurkishLira,
    /// Trinidad and Tobago Dollar
    TrinidadAndTobagoDollar,
    /// New Taiwan Dollar
    NewTaiwanDollar,
    /// Tanzanian Shilling
    TanzanianShilling,
    /// Hryvnia
    Hryvnia,
    /// Uganda Shilling
    UgandaShilling,
    /// US Dollar
    UsDollar,
    /// US Dollar (Next day)
    UsDollarNextDay,
    /// Uruguay Peso en Unidades Indexadas (UI)
    UruguayPesoEnUnidadesIndexadasUi,
    /// Peso Uruguayo
    PesoUruguayo,
    /// Unidad Previsional
    UnidadPrevisional,
    /// Uzbekistan Sum
    UzbekistanSum,
    /// Bolívar Soberano, new valuation
    BolívarSoberanoNewValuation,
    /// Bolívar Soberano
    BolívarSoberano,
    /// Dong
    Dong,
    /// Vatu
    Vatu,
    /// Tala
    Tala,
    /// CFA Franc BEAC
    CfaFrancBeac,
    /// Silver
    Silver,
    /// Gold
    Gold,
    /// Bond Markets Unit European Composite Unit (EURCO)
    BondMarketsUnitEuropeanCompositeUnitEurco,
    /// Bond Markets Unit European Monetary Unit (E.M.U.-6)
    BondMarketsUnitEuropeanMonetaryUnitEMU6,
    /// Bond Markets Unit European Unit of Account 9 (E.U.A.-9)
    BondMarketsUnitEuropeanUnitAccount9EUA9,
    /// Bond Markets Unit European Unit of Account 17 (E.U.A.-17)
    BondMarketsUnitEuropeanUnitAccount17EUA17,
    /// East Caribbean Dollar
    EastCaribbeanDollar,
    /// SDR (Special Drawing Right)
    SdrSpecialDrawingRight,
    /// CFA Franc BCEAO
    CfaFrancBceao,
    /// Palladium
    Palladium,
    /// CFP Franc
    CfpFranc,
    /// Platinum
    Platinum,
    /// Sucre
    Sucre,
    /// Codes specifically reserved for testing purposes
    CodesSpecificallyReservedForTestingPurposes,
    /// ADB Unit of Account
    AdbUnitAccount,
    /// The codes assigned for transactions where no currency is involved
    TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved,
    /// Yemeni Rial
    YemeniRial,
    /// Rand
    Rand,
    /// Zambian Kwacha
    ZambianKwacha,
    /// Zimbabwe Gold
    ZimbabweGold,
}

impl crate::Code for Currency {
    fn code(self) -> &'static str {
        match self {
            Currency::UaeDirham => "AED",
            Currency::Afghani => "AFN",
            Currency::Lek => "ALL",
            Currency::ArmenianDram => "AMD",
            Currency::NetherlandsAntilleanGuilder => "ANG",
            Currency::Kwanza => "AOA",
            Currency::ArgentinePeso => "ARS",
            Currency::AustralianDollar => "AUD",
            Currency::ArubanFlorin => "AWG",
            Currency::AzerbaijanManat => "AZN",
            Currency::ConvertibleMark => "BAM",
            Currency::BarbadosDollar => "BBD",
            Currency::Taka => "BDT",
            Currency::BulgarianLev => "BGN",
            Currency::BahrainiDinar => "BHD",
            Currency::BurundiFranc => "BIF",
            Currency::BermudianDollar => "BMD",
            Currency::BruneiDollar => "BND",
            Currency::Boliviano => "BOB",
            Currency::Mvdol => "BOV",
            Currency::BrazilianReal => "BRL",
            Currency::BahamianDollar => "BSD",
            Currency::Ngultrum => "BTN",
            Currency::Pula => "BWP",
            Currency::BelarusianRuble => "BYN",
            Currency::BelizeDollar => "BZD",
            Currency::CanadianDollar => "CAD",
            Currency::CongoleseFranc => "CDF",
            Currency::WirEuro => "CHE",
            Currency::SwissFranc => "CHF",
            Currency::WirFranc => "CHW",
            Currency::UnidadDeFomento => "CLF",
            Currency::ChileanPeso => "CLP",
            Currency::YuanRenminbi => "CNY",
            Currency::ColombianPeso => "COP",
            Currency::UnidadDeValorReal => "COU",
            Currency::CostaRicanColon => "CRC",
            Currency::CubanPeso => "CUP",
            Currency::CaboVerdeEscudo => "CVE",
            Currency::CzechKoruna => "CZK",
            Currency::DjiboutiFranc => "DJF",
            Currency::DanishKrone => "DKK",
            Currency::DominicanPeso => "DOP",
            Currency::AlgerianDinar => "DZD",
            Currency::EgyptianPound => "EGP",
            Currency::Nakfa => "ERN",
            Currency::EthiopianBirr => "ETB",
            Currency::Euro => "EUR",
            Currency::FijiDollar => "FJD",
            Currency::FalklandIslandsPound => "FKP",
            Currency::PoundSterling => "GBP",
            Currency::Lari => "GEL",
            Currency::GhanaCedi => "GHS",
            Currency::GibraltarPound => "GIP",
            Currency::Dalasi => "GMD",
            Currency::GuineanFranc => "GNF",
            Currency::Quetzal => "GTQ",
            Currency::GuyanaDollar => "GYD",
            Currency::HongKongDollar => "HKD",
            Currency::Lempira => "HNL",
            Currency::Gourde => "HTG",
            Currency::Forint => "HUF",
            Currency::Rupiah => "IDR",
            Currency::NewIsraeliSheqel => "ILS",
            Currency::IndianRupee => "INR",
            Currency::IraqiDinar => "IQD",
            Currency::IranianRial => "IRR",
            Currency::IcelandKrona => "ISK",
            Currency::JamaicanDollar => "JMD",
            Currency::JordanianDinar => "JOD",
            Currency::Yen => "JPY",
            Currency::KenyanShilling => "KES",
            Currency::Som => "KGS",
            Currency::Riel => "KHR",
            Currency::ComorianFranc => "KMF",
            Currency::NorthKoreanWon => "KPW",
            Currency::Won => "KRW",
            Currency::KuwaitiDinar => "KWD",
            Currency::CaymanIslandsDollar => "KYD",
            Currency::Tenge => "KZT",
            Currency::LaoKip => "LAK",
            Currency::LebanesePound => "LBP",
            Currency::SriLankaRupee => "LKR",
            Currency::LiberianDollar => "LRD",
            Currency::Loti => "LSL",
            Currency::LibyanDinar => "LYD",
            Currency::MoroccanDirham => "MAD",
            Currency::MoldovanLeu => "MDL",
            Currency::MalagasyAriary => "MGA",
            Currency::Denar => "MKD",
            Currency::Kyat => "MMK",
            Currency::Tugrik => "MNT",
            Currency::Pataca => "MOP",
            Currency::Ouguiya => "MRU",
            Currency::MauritiusRupee => "MUR",
            Currency::Rufiyaa => "MVR",
            Currency::MalawiKwacha => "MWK",
            Currency::MexicanPeso => "MXN",
            Currency::MexicanUnidadDeInversionUdi => "MXV",
            Currency::MalaysianRinggit => "MYR",
            Currency::MozambiqueMetical => "MZN",
            Currency::NamibiaDollar => "NAD",
            Currency::Naira => "NGN",
            Currency::CordobaOro => "NIO",
            Currency::NorwegianKrone => "NOK",
            Currency::NepaleseRupee => "NPR",
            Currency::NewZealandDollar => "NZD",
            Currency::RialOmani => "OMR",
            Currency::Balboa => "PAB",
            Currency::Sol => "PEN",
            Currency::Kina => "PGK",
            Currency::PhilippinePeso => "PHP",
            Currency::PakistanRupee => "PKR",
            Currency::Zloty => "PLN",
            Currency::Guarani => "PYG",
            Currency::QatariRial => "QAR",
            Currency::RomanianLeu => "RON",
            Currency::SerbianDinar => "RSD",
            Currency::RussianRuble => "RUB",
            Currency::RwandaFranc => "RWF",
            Currency::SaudiRiyal => "SAR",
            Currency::SolomonIslandsDollar => "SBD",
            Currency::SeychellesRupee => "SCR",
            Currency::SudanesePound => "SDG",
            Currency::SwedishKrona => "SEK",
            Currency::SingaporeDollar => "SGD",
            Currency::SaintHelenaPound => "SHP",
            Currency::SierraLeoneNewValuation2022 => "SLE",
            Currency::SomaliShilling => "SOS",
            Currency::SurinamDollar => "SRD",
            Currency::SouthSudanesePound => "SSP",
            Currency::Dobra => "STN",
            Currency::ElSalvadorColon => "SVC",
            Currency::SyrianPound => "SYP",
            Currency::Lilangeni => "SZL",
            Currency::Baht => "THB",
            Currency::Somoni => "TJS",
            Currency::TurkmenistanNewManat => "TMT",
            Currency::TunisianDinar => "TND",
            Currency::PaAnga => "TOP",
            Currency::TurkishLira => "TRY",
            Currency::TrinidadAndTobagoDollar => "TTD",
            Currency::NewTaiwanDollar => "TWD",
            Currency::TanzanianShilling => "TZS",
            Currency::Hryvnia => "UAH",
            Currency::UgandaShilling => "UGX",
            Currency::UsDollar => "USD",
            Currency::UsDollarNextDay => "USN",
            Currency::UruguayPesoEnUnidadesIndexadasUi => "UYI",
            Currency::PesoUruguayo => "UYU",
            Currency::UnidadPrevisional => "UYW",
            Currency::UzbekistanSum => "UZS",
            Currency::BolívarSoberanoNewValuation => "VED",
            Currency::BolívarSoberano => "VES",
            Currency::Dong => "VND",
            Currency::Vatu => "VUV",
            Currency::Tala => "WST",
            Currency::CfaFrancBeac => "XAF",
            Currency::Silver => "XAG",
            Currency::Gold => "XAU",
            Currency::BondMarketsUnitEuropeanCompositeUnitEurco => "XBA",
            Currency::BondMarketsUnitEuropeanMonetaryUnitEMU6 => "XBB",
            Currency::BondMarketsUnitEuropeanUnitAccount9EUA9 => "XBC",
            Currency::BondMarketsUnitEuropeanUnitAccount17EUA17 => "XBD",
            Currency::EastCaribbeanDollar => "XCD",
            Currency::SdrSpecialDrawingRight => "XDR",
            Currency::CfaFrancBceao => "XOF",
            Currency::Palladium => "XPD",
            Currency::CfpFranc => "XPF",
            Currency::Platinum => "XPT",
            Currency::Sucre => "XSU",
            Currency::CodesSpecificallyReservedForTestingPurposes => "XTS",
            Currency::AdbUnitAccount => "XUA",
            Currency::TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved => "XXX",
            Currency::YemeniRial => "YER",
            Currency::Rand => "ZAR",
            Currency::ZambianKwacha => "ZMW",
            Currency::ZimbabweGold => "ZWG",
        }
    }
}

impl crate::Description for Currency {
    fn description(self) -> &'static str {
        match self {
            Currency::UaeDirham => "UAE Dirham",
            Currency::Afghani => "Afghani",
            Currency::Lek => "Lek",
            Currency::ArmenianDram => "Armenian Dram",
            Currency::NetherlandsAntilleanGuilder => "Netherlands Antillean Guilder",
            Currency::Kwanza => "Kwanza",
            Currency::ArgentinePeso => "Argentine Peso",
            Currency::AustralianDollar => "Australian Dollar",
            Currency::ArubanFlorin => "Aruban Florin",
            Currency::AzerbaijanManat => "Azerbaijan Manat",
            Currency::ConvertibleMark => "Convertible Mark",
            Currency::BarbadosDollar => "Barbados Dollar",
            Currency::Taka => "Taka",
            Currency::BulgarianLev => "Bulgarian Lev",
            Currency::BahrainiDinar => "Bahraini Dinar",
            Currency::BurundiFranc => "Burundi Franc",
            Currency::BermudianDollar => "Bermudian Dollar",
            Currency::BruneiDollar => "Brunei Dollar",
            Currency::Boliviano => "Boliviano",
            Currency::Mvdol => "Mvdol",
            Currency::BrazilianReal => "Brazilian Real",
            Currency::BahamianDollar => "Bahamian Dollar",
            Currency::Ngultrum => "Ngultrum",
            Currency::Pula => "Pula",
            Currency::BelarusianRuble => "Belarusian Ruble",
            Currency::BelizeDollar => "Belize Dollar",
            Currency::CanadianDollar => "Canadian Dollar",
            Currency::CongoleseFranc => "Congolese Franc",
            Currency::WirEuro => "WIR Euro",
            Currency::SwissFranc => "Swiss Franc",
            Currency::WirFranc => "WIR Franc",
            Currency::UnidadDeFomento => "Unidad de Fomento",
            Currency::ChileanPeso => "Chilean Peso",
            Currency::YuanRenminbi => "Yuan Renminbi",
            Currency::ColombianPeso => "Colombian Peso",
            Currency::UnidadDeValorReal => "Unidad de Valor Real",
            Currency::CostaRicanColon => "Costa Rican Colon",
            Currency::CubanPeso => "Cuban Peso",
            Currency::CaboVerdeEscudo => "Cabo Verde Escudo",
            Currency::CzechKoruna => "Czech Koruna",
            Currency::DjiboutiFranc => "Djibouti Franc",
            Currency::DanishKrone => "Danish Krone",
            Currency::DominicanPeso => "Dominican Peso",
            Currency::AlgerianDinar => "Algerian Dinar",
            Currency::EgyptianPound => "Egyptian Pound",
            Currency::Nakfa => "Nakfa",
            Currency::EthiopianBirr => "Ethiopian Birr",
            Currency::Euro => "Euro",
            Currency::FijiDollar => "Fiji Dollar",
            Currency::FalklandIslandsPound => "Falkland Islands Pound",
            Currency::PoundSterling => "Pound Sterling",
            Currency::Lari => "Lari",
            Currency::GhanaCedi => "Ghana Cedi",
            Currency::GibraltarPound => "Gibraltar Pound",
            Currency::Dalasi => "Dalasi",
            Currency::GuineanFranc => "Guinean Franc",
            Currency::Quetzal => "Quetzal",
            Currency::GuyanaDollar => "Guyana Dollar",
            Currency::HongKongDollar => "Hong Kong Dollar",
            Currency::Lempira => "Lempira",
            Currency::Gourde => "Gourde",
            Currency::Forint => "Forint",
            Currency::Rupiah => "Rupiah",
            Currency::NewIsraeliSheqel => "New Israeli Sheqel",
            Currency::IndianRupee => "Indian Rupee",
            Currency::IraqiDinar => "Iraqi Dinar",
            Currency::IranianRial => "Iranian Rial",
            Currency::IcelandKrona => "Iceland Krona",
            Currency::JamaicanDollar => "Jamaican Dollar",
            Currency::JordanianDinar => "Jordanian Dinar",
            Currency::Yen => "Yen",
            Currency::KenyanShilling => "Kenyan Shilling",
            Currency::Som => "Som",
            Currency::Riel => "Riel",
            Currency::ComorianFranc => "Comorian Franc ",
            Currency::NorthKoreanWon => "North Korean Won",
            Currency::Won => "Won",
            Currency::KuwaitiDinar => "Kuwaiti Dinar",
            Currency::CaymanIslandsDollar => "Cayman Islands Dollar",
            Currency::Tenge => "Tenge",
            Currency::LaoKip => "Lao Kip",
            Currency::LebanesePound => "Lebanese Pound",
            Currency::SriLankaRupee => "Sri Lanka Rupee",
            Currency::LiberianDollar => "Liberian Dollar",
            Currency::Loti => "Loti",
            Currency::LibyanDinar => "Libyan Dinar",
            Currency::MoroccanDirham => "Moroccan Dirham",
            Currency::MoldovanLeu => "Moldovan Leu",
            Currency::MalagasyAriary => "Malagasy Ariary",
            Currency::Denar => "Denar",
            Currency::Kyat => "Kyat",
            Currency::Tugrik => "Tugrik",
            Currency::Pataca => "Pataca",
            Currency::Ouguiya => "Ouguiya",
            Currency::MauritiusRupee => "Mauritius Rupee",
            Currency::Rufiyaa => "Rufiyaa",
            Currency::MalawiKwacha => "Malawi Kwacha",
            Currency::MexicanPeso => "Mexican Peso",
            Currency::MexicanUnidadDeInversionUdi => "Mexican Unidad de Inversion (UDI)",
            Currency::MalaysianRinggit => "Malaysian Ringgit",
            Currency::MozambiqueMetical => "Mozambique Metical",
            Currency::NamibiaDollar => "Namibia Dollar",
            Currency::Naira => "Naira",
            Currency::CordobaOro => "Cordoba Oro",
            Currency::NorwegianKrone => "Norwegian Krone",
            Currency::NepaleseRupee => "Nepalese Rupee",
            Currency::NewZealandDollar => "New Zealand Dollar",
            Currency::RialOmani => "Rial Omani",
            Currency::Balboa => "Balboa",
            Currency::Sol => "Sol",
            Currency::Kina => "Kina",
            Currency::PhilippinePeso => "Philippine Peso",
            Currency::PakistanRupee => "Pakistan Rupee",
            Currency::Zloty => "Zloty",
            Currency::Guarani => "Guarani",
            Currency::QatariRial => "Qatari Rial",
            Currency::RomanianLeu => "Romanian Leu",
            Currency::SerbianDinar => "Serbian Dinar",
            Currency::RussianRuble => "Russian Ruble",
            Currency::RwandaFranc => "Rwanda Franc",
            Currency::SaudiRiyal => "Saudi Riyal",
            Currency::SolomonIslandsDollar => "Solomon Islands Dollar",
            Currency::SeychellesRupee => "Seychelles Rupee",
            Currency::SudanesePound => "Sudanese Pound",
            Currency::SwedishKrona => "Swedish Krona",
            Currency::SingaporeDollar => "Singapore Dollar",
            Currency::SaintHelenaPound => "Saint Helena Pound",
            Currency::SierraLeoneNewValuation2022 => "Sierra Leone (new valuation 2022)",
            Currency::SomaliShilling => "Somali Shilling",
            Currency::SurinamDollar => "Surinam Dollar",
            Currency::SouthSudanesePound => "South Sudanese Pound",
            Currency::Dobra => "Dobra",
            Currency::ElSalvadorColon => "El Salvador Colon",
            Currency::SyrianPound => "Syrian Pound",
            Currency::Lilangeni => "Lilangeni",
            Currency::Baht => "Baht",
            Currency::Somoni => "Somoni",
            Currency::TurkmenistanNewManat => "Turkmenistan New Manat",
            Currency::TunisianDinar => "Tunisian Dinar",
            Currency::PaAnga => "Pa’anga",
            Currency::TurkishLira => "Turkish Lira",
            Currency::TrinidadAndTobagoDollar => "Trinidad and Tobago Dollar",
            Currency::NewTaiwanDollar => "New Taiwan Dollar",
            Currency::TanzanianShilling => "Tanzanian Shilling",
            Currency::Hryvnia => "Hryvnia",
            Currency::UgandaShilling => "Uganda Shilling",
            Currency::UsDollar => "US Dollar",
            Currency::UsDollarNextDay => "US Dollar (Next day)",
            Currency::UruguayPesoEnUnidadesIndexadasUi => "Uruguay Peso en Unidades Indexadas (UI)",
            Currency::PesoUruguayo => "Peso Uruguayo",
            Currency::UnidadPrevisional => "Unidad Previsional",
            Currency::UzbekistanSum => "Uzbekistan Sum",
            Currency::BolívarSoberanoNewValuation => "Bolívar Soberano, new valuation",
            Currency::BolívarSoberano => "Bolívar Soberano",
            Currency::Dong => "Dong",
            Currency::Vatu => "Vatu",
            Currency::Tala => "Tala",
            Currency::CfaFrancBeac => "CFA Franc BEAC",
            Currency::Silver => "Silver",
            Currency::Gold => "Gold",
            Currency::BondMarketsUnitEuropeanCompositeUnitEurco => {
                "Bond Markets Unit European Composite Unit (EURCO)"
            }
            Currency::BondMarketsUnitEuropeanMonetaryUnitEMU6 => {
                "Bond Markets Unit European Monetary Unit (E.M.U.-6)"
            }
            Currency::BondMarketsUnitEuropeanUnitAccount9EUA9 => {
                "Bond Markets Unit European Unit of Account 9 (E.U.A.-9)"
            }
            Currency::BondMarketsUnitEuropeanUnitAccount17EUA17 => {
                "Bond Markets Unit European Unit of Account 17 (E.U.A.-17)"
            }
            Currency::EastCaribbeanDollar => "East Caribbean Dollar",
            Currency::SdrSpecialDrawingRight => "SDR (Special Drawing Right)",
            Currency::CfaFrancBceao => "CFA Franc BCEAO",
            Currency::Palladium => "Palladium",
            Currency::CfpFranc => "CFP Franc",
            Currency::Platinum => "Platinum",
            Currency::Sucre => "Sucre",
            Currency::CodesSpecificallyReservedForTestingPurposes => {
                "Codes specifically reserved for testing purposes"
            }
            Currency::AdbUnitAccount => "ADB Unit of Account",
            Currency::TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved => {
                "The codes assigned for transactions where no currency is involved"
            }
            Currency::YemeniRial => "Yemeni Rial",
            Currency::Rand => "Rand",
            Currency::ZambianKwacha => "Zambian Kwacha",
            Currency::ZimbabweGold => "Zimbabwe Gold",
        }
    }
}

impl crate::FromCode for Currency {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "AED" => Some(Currency::UaeDirham),
            "AFN" => Some(Currency::Afghani),
            "ALL" => Some(Currency::Lek),
            "AMD" => Some(Currency::ArmenianDram),
            "ANG" => Some(Currency::NetherlandsAntilleanGuilder),
            "AOA" => Some(Currency::Kwanza),
            "ARS" => Some(Currency::ArgentinePeso),
            "AUD" => Some(Currency::AustralianDollar),
            "AWG" => Some(Currency::ArubanFlorin),
            "AZN" => Some(Currency::AzerbaijanManat),
            "BAM" => Some(Currency::ConvertibleMark),
            "BBD" => Some(Currency::BarbadosDollar),
            "BDT" => Some(Currency::Taka),
            "BGN" => Some(Currency::BulgarianLev),
            "BHD" => Some(Currency::BahrainiDinar),
            "BIF" => Some(Currency::BurundiFranc),
            "BMD" => Some(Currency::BermudianDollar),
            "BND" => Some(Currency::BruneiDollar),
            "BOB" => Some(Currency::Boliviano),
            "BOV" => Some(Currency::Mvdol),
            "BRL" => Some(Currency::BrazilianReal),
            "BSD" => Some(Currency::BahamianDollar),
            "BTN" => Some(Currency::Ngultrum),
            "BWP" => Some(Currency::Pula),
            "BYN" => Some(Currency::BelarusianRuble),
            "BZD" => Some(Currency::BelizeDollar),
            "CAD" => Some(Currency::CanadianDollar),
            "CDF" => Some(Currency::CongoleseFranc),
            "CHE" => Some(Currency::WirEuro),
            "CHF" => Some(Currency::SwissFranc),
            "CHW" => Some(Currency::WirFranc),
            "CLF" => Some(Currency::UnidadDeFomento),
            "CLP" => Some(Currency::ChileanPeso),
            "CNY" => Some(Currency::YuanRenminbi),
            "COP" => Some(Currency::ColombianPeso),
            "COU" => Some(Currency::UnidadDeValorReal),
            "CRC" => Some(Currency::CostaRicanColon),
            "CUP" => Some(Currency::CubanPeso),
            "CVE" => Some(Currency::CaboVerdeEscudo),
            "CZK" => Some(Currency::CzechKoruna),
            "DJF" => Some(Currency::DjiboutiFranc),
            "DKK" => Some(Currency::DanishKrone),
            "DOP" => Some(Currency::DominicanPeso),
            "DZD" => Some(Currency::AlgerianDinar),
            "EGP" => Some(Currency::EgyptianPound),
            "ERN" => Some(Currency::Nakfa),
            "ETB" => Some(Currency::EthiopianBirr),
            "EUR" => Some(Currency::Euro),
            "FJD" => Some(Currency::FijiDollar),
            "FKP" => Some(Currency::FalklandIslandsPound),
            "GBP" => Some(Currency::PoundSterling),
            "GEL" => Some(Currency::Lari),
            "GHS" => Some(Currency::GhanaCedi),
            "GIP" => Some(Currency::GibraltarPound),
            "GMD" => Some(Currency::Dalasi),
            "GNF" => Some(Currency::GuineanFranc),
            "GTQ" => Some(Currency::Quetzal),
            "GYD" => Some(Currency::GuyanaDollar),
            "HKD" => Some(Currency::HongKongDollar),
            "HNL" => Some(Currency::Lempira),
            "HTG" => Some(Currency::Gourde),
            "HUF" => Some(Currency::Forint),
            "IDR" => Some(Currency::Rupiah),
            "ILS" => Some(Currency::NewIsraeliSheqel),
            "INR" => Some(Currency::IndianRupee),
            "IQD" => Some(Currency::IraqiDinar),
            "IRR" => Some(Currency::IranianRial),
            "ISK" => Some(Currency::IcelandKrona),
            "JMD" => Some(Currency::JamaicanDollar),
            "JOD" => Some(Currency::JordanianDinar),
            "JPY" => Some(Currency::Yen),
            "KES" => Some(Currency::KenyanShilling),
            "KGS" => Some(Currency::Som),
            "KHR" => Some(Currency::Riel),
            "KMF" => Some(Currency::ComorianFranc),
            "KPW" => Some(Currency::NorthKoreanWon),
            "KRW" => Some(Currency::Won),
            "KWD" => Some(Currency::KuwaitiDinar),
            "KYD" => Some(Currency::CaymanIslandsDollar),
            "KZT" => Some(Currency::Tenge),
            "LAK" => Some(Currency::LaoKip),
            "LBP" => Some(Currency::LebanesePound),
            "LKR" => Some(Currency::SriLankaRupee),
            "LRD" => Some(Currency::LiberianDollar),
            "LSL" => Some(Currency::Loti),
            "LYD" => Some(Currency::LibyanDinar),
            "MAD" => Some(Currency::MoroccanDirham),
            "MDL" => Some(Currency::MoldovanLeu),
            "MGA" => Some(Currency::MalagasyAriary),
            "MKD" => Some(Currency::Denar),
            "MMK" => Some(Currency::Kyat),
            "MNT" => Some(Currency::Tugrik),
            "MOP" => Some(Currency::Pataca),
            "MRU" => Some(Currency::Ouguiya),
            "MUR" => Some(Currency::MauritiusRupee),
            "MVR" => Some(Currency::Rufiyaa),
            "MWK" => Some(Currency::MalawiKwacha),
            "MXN" => Some(Currency::MexicanPeso),
            "MXV" => Some(Currency::MexicanUnidadDeInversionUdi),
            "MYR" => Some(Currency::MalaysianRinggit),
            "MZN" => Some(Currency::MozambiqueMetical),
            "NAD" => Some(Currency::NamibiaDollar),
            "NGN" => Some(Currency::Naira),
            "NIO" => Some(Currency::CordobaOro),
            "NOK" => Some(Currency::NorwegianKrone),
            "NPR" => Some(Currency::NepaleseRupee),
            "NZD" => Some(Currency::NewZealandDollar),
            "OMR" => Some(Currency::RialOmani),
            "PAB" => Some(Currency::Balboa),
            "PEN" => Some(Currency::Sol),
            "PGK" => Some(Currency::Kina),
            "PHP" => Some(Currency::PhilippinePeso),
            "PKR" => Some(Currency::PakistanRupee),
            "PLN" => Some(Currency::Zloty),
            "PYG" => Some(Currency::Guarani),
            "QAR" => Some(Currency::QatariRial),
            "RON" => Some(Currency::RomanianLeu),
            "RSD" => Some(Currency::SerbianDinar),
            "RUB" => Some(Currency::RussianRuble),
            "RWF" => Some(Currency::RwandaFranc),
            "SAR" => Some(Currency::SaudiRiyal),
            "SBD" => Some(Currency::SolomonIslandsDollar),
            "SCR" => Some(Currency::SeychellesRupee),
            "SDG" => Some(Currency::SudanesePound),
            "SEK" => Some(Currency::SwedishKrona),
            "SGD" => Some(Currency::SingaporeDollar),
            "SHP" => Some(Currency::SaintHelenaPound),
            "SLE" => Some(Currency::SierraLeoneNewValuation2022),
            "SOS" => Some(Currency::SomaliShilling),
            "SRD" => Some(Currency::SurinamDollar),
            "SSP" => Some(Currency::SouthSudanesePound),
            "STN" => Some(Currency::Dobra),
            "SVC" => Some(Currency::ElSalvadorColon),
            "SYP" => Some(Currency::SyrianPound),
            "SZL" => Some(Currency::Lilangeni),
            "THB" => Some(Currency::Baht),
            "TJS" => Some(Currency::Somoni),
            "TMT" => Some(Currency::TurkmenistanNewManat),
            "TND" => Some(Currency::TunisianDinar),
            "TOP" => Some(Currency::PaAnga),
            "TRY" => Some(Currency::TurkishLira),
            "TTD" => Some(Currency::TrinidadAndTobagoDollar),
            "TWD" => Some(Currency::NewTaiwanDollar),
            "TZS" => Some(Currency::TanzanianShilling),
            "UAH" => Some(Currency::Hryvnia),
            "UGX" => Some(Currency::UgandaShilling),
            "USD" => Some(Currency::UsDollar),
            "USN" => Some(Currency::UsDollarNextDay),
            "UYI" => Some(Currency::UruguayPesoEnUnidadesIndexadasUi),
            "UYU" => Some(Currency::PesoUruguayo),
            "UYW" => Some(Currency::UnidadPrevisional),
            "UZS" => Some(Currency::UzbekistanSum),
            "VED" => Some(Currency::BolívarSoberanoNewValuation),
            "VES" => Some(Currency::BolívarSoberano),
            "VND" => Some(Currency::Dong),
            "VUV" => Some(Currency::Vatu),
            "WST" => Some(Currency::Tala),
            "XAF" => Some(Currency::CfaFrancBeac),
            "XAG" => Some(Currency::Silver),
            "XAU" => Some(Currency::Gold),
            "XBA" => Some(Currency::BondMarketsUnitEuropeanCompositeUnitEurco),
            "XBB" => Some(Currency::BondMarketsUnitEuropeanMonetaryUnitEMU6),
            "XBC" => Some(Currency::BondMarketsUnitEuropeanUnitAccount9EUA9),
            "XBD" => Some(Currency::BondMarketsUnitEuropeanUnitAccount17EUA17),
            "XCD" => Some(Currency::EastCaribbeanDollar),
            "XDR" => Some(Currency::SdrSpecialDrawingRight),
            "XOF" => Some(Currency::CfaFrancBceao),
            "XPD" => Some(Currency::Palladium),
            "XPF" => Some(Currency::CfpFranc),
            "XPT" => Some(Currency::Platinum),
            "XSU" => Some(Currency::Sucre),
            "XTS" => Some(Currency::CodesSpecificallyReservedForTestingPurposes),
            "XUA" => Some(Currency::AdbUnitAccount),
            "XXX" => Some(Currency::TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved),
            "YER" => Some(Currency::YemeniRial),
            "ZAR" => Some(Currency::Rand),
            "ZMW" => Some(Currency::ZambianKwacha),
            "ZWG" => Some(Currency::ZimbabweGold),
            _ => None,
        }
    }
}

#[cfg(feature = "iso_currency")]
impl std::convert::TryFrom<Currency> for iso_currency::Currency {
    type Error = ErrFromCurrency;
    fn try_from(value: Currency) -> Result<Self, Self::Error> {
        match value {
            Currency::UaeDirham => Ok(iso_currency::Currency::AED),
            Currency::Afghani => Ok(iso_currency::Currency::AFN),
            Currency::Lek => Ok(iso_currency::Currency::ALL),
            Currency::ArmenianDram => Ok(iso_currency::Currency::AMD),
            Currency::Kwanza => Ok(iso_currency::Currency::AOA),
            Currency::ArgentinePeso => Ok(iso_currency::Currency::ARS),
            Currency::AustralianDollar => Ok(iso_currency::Currency::AUD),
            Currency::ArubanFlorin => Ok(iso_currency::Currency::AWG),
            Currency::AzerbaijanManat => Ok(iso_currency::Currency::AZN),
            Currency::ConvertibleMark => Ok(iso_currency::Currency::BAM),
            Currency::BarbadosDollar => Ok(iso_currency::Currency::BBD),
            Currency::Taka => Ok(iso_currency::Currency::BDT),
            Currency::BulgarianLev => Ok(iso_currency::Currency::BGN),
            Currency::BahrainiDinar => Ok(iso_currency::Currency::BHD),
            Currency::BurundiFranc => Ok(iso_currency::Currency::BIF),
            Currency::BermudianDollar => Ok(iso_currency::Currency::BMD),
            Currency::BruneiDollar => Ok(iso_currency::Currency::BND),
            Currency::Boliviano => Ok(iso_currency::Currency::BOB),
            Currency::Mvdol => Ok(iso_currency::Currency::BOV),
            Currency::BrazilianReal => Ok(iso_currency::Currency::BRL),
            Currency::BahamianDollar => Ok(iso_currency::Currency::BSD),
            Currency::Ngultrum => Ok(iso_currency::Currency::BTN),
            Currency::Pula => Ok(iso_currency::Currency::BWP),
            Currency::BelarusianRuble => Ok(iso_currency::Currency::BYN),
            Currency::BelizeDollar => Ok(iso_currency::Currency::BZD),
            Currency::CanadianDollar => Ok(iso_currency::Currency::CAD),
            Currency::CongoleseFranc => Ok(iso_currency::Currency::CDF),
            Currency::WirEuro => Ok(iso_currency::Currency::CHE),
            Currency::SwissFranc => Ok(iso_currency::Currency::CHF),
            Currency::WirFranc => Ok(iso_currency::Currency::CHW),
            Currency::UnidadDeFomento => Ok(iso_currency::Currency::CLF),
            Currency::ChileanPeso => Ok(iso_currency::Currency::CLP),
            Currency::YuanRenminbi => Ok(iso_currency::Currency::CNY),
            Currency::ColombianPeso => Ok(iso_currency::Currency::COP),
            Currency::UnidadDeValorReal => Ok(iso_currency::Currency::COU),
            Currency::CostaRicanColon => Ok(iso_currency::Currency::CRC),
            Currency::CubanPeso => Ok(iso_currency::Currency::CUP),
            Currency::CaboVerdeEscudo => Ok(iso_currency::Currency::CVE),
            Currency::CzechKoruna => Ok(iso_currency::Currency::CZK),
            Currency::DjiboutiFranc => Ok(iso_currency::Currency::DJF),
            Currency::DanishKrone => Ok(iso_currency::Currency::DKK),
            Currency::DominicanPeso => Ok(iso_currency::Currency::DOP),
            Currency::AlgerianDinar => Ok(iso_currency::Currency::DZD),
            Currency::EgyptianPound => Ok(iso_currency::Currency::EGP),
            Currency::Nakfa => Ok(iso_currency::Currency::ERN),
            Currency::EthiopianBirr => Ok(iso_currency::Currency::ETB),
            Currency::Euro => Ok(iso_currency::Currency::EUR),
            Currency::FijiDollar => Ok(iso_currency::Currency::FJD),
            Currency::FalklandIslandsPound => Ok(iso_currency::Currency::FKP),
            Currency::PoundSterling => Ok(iso_currency::Currency::GBP),
            Currency::Lari => Ok(iso_currency::Currency::GEL),
            Currency::GhanaCedi => Ok(iso_currency::Currency::GHS),
            Currency::GibraltarPound => Ok(iso_currency::Currency::GIP),
            Currency::Dalasi => Ok(iso_currency::Currency::GMD),
            Currency::GuineanFranc => Ok(iso_currency::Currency::GNF),
            Currency::Quetzal => Ok(iso_currency::Currency::GTQ),
            Currency::GuyanaDollar => Ok(iso_currency::Currency::GYD),
            Currency::HongKongDollar => Ok(iso_currency::Currency::HKD),
            Currency::Lempira => Ok(iso_currency::Currency::HNL),
            Currency::Gourde => Ok(iso_currency::Currency::HTG),
            Currency::Forint => Ok(iso_currency::Currency::HUF),
            Currency::Rupiah => Ok(iso_currency::Currency::IDR),
            Currency::NewIsraeliSheqel => Ok(iso_currency::Currency::ILS),
            Currency::IndianRupee => Ok(iso_currency::Currency::INR),
            Currency::IraqiDinar => Ok(iso_currency::Currency::IQD),
            Currency::IranianRial => Ok(iso_currency::Currency::IRR),
            Currency::IcelandKrona => Ok(iso_currency::Currency::ISK),
            Currency::JamaicanDollar => Ok(iso_currency::Currency::JMD),
            Currency::JordanianDinar => Ok(iso_currency::Currency::JOD),
            Currency::Yen => Ok(iso_currency::Currency::JPY),
            Currency::KenyanShilling => Ok(iso_currency::Currency::KES),
            Currency::Som => Ok(iso_currency::Currency::KGS),
            Currency::Riel => Ok(iso_currency::Currency::KHR),
            Currency::ComorianFranc => Ok(iso_currency::Currency::KMF),
            Currency::NorthKoreanWon => Ok(iso_currency::Currency::KPW),
            Currency::Won => Ok(iso_currency::Currency::KRW),
            Currency::KuwaitiDinar => Ok(iso_currency::Currency::KWD),
            Currency::CaymanIslandsDollar => Ok(iso_currency::Currency::KYD),
            Currency::Tenge => Ok(iso_currency::Currency::KZT),
            Currency::LaoKip => Ok(iso_currency::Currency::LAK),
            Currency::LebanesePound => Ok(iso_currency::Currency::LBP),
            Currency::SriLankaRupee => Ok(iso_currency::Currency::LKR),
            Currency::LiberianDollar => Ok(iso_currency::Currency::LRD),
            Currency::Loti => Ok(iso_currency::Currency::LSL),
            Currency::LibyanDinar => Ok(iso_currency::Currency::LYD),
            Currency::MoroccanDirham => Ok(iso_currency::Currency::MAD),
            Currency::MoldovanLeu => Ok(iso_currency::Currency::MDL),
            Currency::MalagasyAriary => Ok(iso_currency::Currency::MGA),
            Currency::Denar => Ok(iso_currency::Currency::MKD),
            Currency::Kyat => Ok(iso_currency::Currency::MMK),
            Currency::Tugrik => Ok(iso_currency::Currency::MNT),
            Currency::Pataca => Ok(iso_currency::Currency::MOP),
            Currency::Ouguiya => Ok(iso_currency::Currency::MRU),
            Currency::MauritiusRupee => Ok(iso_currency::Currency::MUR),
            Currency::Rufiyaa => Ok(iso_currency::Currency::MVR),
            Currency::MalawiKwacha => Ok(iso_currency::Currency::MWK),
            Currency::MexicanPeso => Ok(iso_currency::Currency::MXN),
            Currency::MexicanUnidadDeInversionUdi => Ok(iso_currency::Currency::MXV),
            Currency::MalaysianRinggit => Ok(iso_currency::Currency::MYR),
            Currency::MozambiqueMetical => Ok(iso_currency::Currency::MZN),
            Currency::NamibiaDollar => Ok(iso_currency::Currency::NAD),
            Currency::Naira => Ok(iso_currency::Currency::NGN),
            Currency::CordobaOro => Ok(iso_currency::Currency::NIO),
            Currency::NorwegianKrone => Ok(iso_currency::Currency::NOK),
            Currency::NepaleseRupee => Ok(iso_currency::Currency::NPR),
            Currency::NewZealandDollar => Ok(iso_currency::Currency::NZD),
            Currency::RialOmani => Ok(iso_currency::Currency::OMR),
            Currency::Balboa => Ok(iso_currency::Currency::PAB),
            Currency::Sol => Ok(iso_currency::Currency::PEN),
            Currency::Kina => Ok(iso_currency::Currency::PGK),
            Currency::PhilippinePeso => Ok(iso_currency::Currency::PHP),
            Currency::PakistanRupee => Ok(iso_currency::Currency::PKR),
            Currency::Zloty => Ok(iso_currency::Currency::PLN),
            Currency::Guarani => Ok(iso_currency::Currency::PYG),
            Currency::QatariRial => Ok(iso_currency::Currency::QAR),
            Currency::RomanianLeu => Ok(iso_currency::Currency::RON),
            Currency::SerbianDinar => Ok(iso_currency::Currency::RSD),
            Currency::RussianRuble => Ok(iso_currency::Currency::RUB),
            Currency::RwandaFranc => Ok(iso_currency::Currency::RWF),
            Currency::SaudiRiyal => Ok(iso_currency::Currency::SAR),
            Currency::SolomonIslandsDollar => Ok(iso_currency::Currency::SBD),
            Currency::SeychellesRupee => Ok(iso_currency::Currency::SCR),
            Currency::SudanesePound => Ok(iso_currency::Currency::SDG),
            Currency::SwedishKrona => Ok(iso_currency::Currency::SEK),
            Currency::SingaporeDollar => Ok(iso_currency::Currency::SGD),
            Currency::SaintHelenaPound => Ok(iso_currency::Currency::SHP),
            Currency::SierraLeoneNewValuation2022 => Ok(iso_currency::Currency::SLE),
            Currency::SomaliShilling => Ok(iso_currency::Currency::SOS),
            Currency::SurinamDollar => Ok(iso_currency::Currency::SRD),
            Currency::SouthSudanesePound => Ok(iso_currency::Currency::SSP),
            Currency::Dobra => Ok(iso_currency::Currency::STN),
            Currency::ElSalvadorColon => Ok(iso_currency::Currency::SVC),
            Currency::SyrianPound => Ok(iso_currency::Currency::SYP),
            Currency::Lilangeni => Ok(iso_currency::Currency::SZL),
            Currency::Baht => Ok(iso_currency::Currency::THB),
            Currency::Somoni => Ok(iso_currency::Currency::TJS),
            Currency::TurkmenistanNewManat => Ok(iso_currency::Currency::TMT),
            Currency::TunisianDinar => Ok(iso_currency::Currency::TND),
            Currency::PaAnga => Ok(iso_currency::Currency::TOP),
            Currency::TurkishLira => Ok(iso_currency::Currency::TRY),
            Currency::TrinidadAndTobagoDollar => Ok(iso_currency::Currency::TTD),
            Currency::NewTaiwanDollar => Ok(iso_currency::Currency::TWD),
            Currency::TanzanianShilling => Ok(iso_currency::Currency::TZS),
            Currency::Hryvnia => Ok(iso_currency::Currency::UAH),
            Currency::UgandaShilling => Ok(iso_currency::Currency::UGX),
            Currency::UsDollar => Ok(iso_currency::Currency::USD),
            Currency::UsDollarNextDay => Ok(iso_currency::Currency::USN),
            Currency::UruguayPesoEnUnidadesIndexadasUi => Ok(iso_currency::Currency::UYI),
            Currency::PesoUruguayo => Ok(iso_currency::Currency::UYU),
            Currency::UnidadPrevisional => Ok(iso_currency::Currency::UYW),
            Currency::UzbekistanSum => Ok(iso_currency::Currency::UZS),
            Currency::BolívarSoberanoNewValuation => Ok(iso_currency::Currency::VED),
            Currency::BolívarSoberano => Ok(iso_currency::Currency::VES),
            Currency::Dong => Ok(iso_currency::Currency::VND),
            Currency::Vatu => Ok(iso_currency::Currency::VUV),
            Currency::Tala => Ok(iso_currency::Currency::WST),
            Currency::CfaFrancBeac => Ok(iso_currency::Currency::XAF),
            Currency::Silver => Ok(iso_currency::Currency::XAG),
            Currency::Gold => Ok(iso_currency::Currency::XAU),
            Currency::BondMarketsUnitEuropeanCompositeUnitEurco => Ok(iso_currency::Currency::XBA),
            Currency::BondMarketsUnitEuropeanMonetaryUnitEMU6 => Ok(iso_currency::Currency::XBB),
            Currency::BondMarketsUnitEuropeanUnitAccount9EUA9 => Ok(iso_currency::Currency::XBC),
            Currency::BondMarketsUnitEuropeanUnitAccount17EUA17 => Ok(iso_currency::Currency::XBD),
            Currency::EastCaribbeanDollar => Ok(iso_currency::Currency::XCD),
            Currency::SdrSpecialDrawingRight => Ok(iso_currency::Currency::XDR),
            Currency::CfaFrancBceao => Ok(iso_currency::Currency::XOF),
            Currency::Palladium => Ok(iso_currency::Currency::XPD),
            Currency::CfpFranc => Ok(iso_currency::Currency::XPF),
            Currency::Platinum => Ok(iso_currency::Currency::XPT),
            Currency::Sucre => Ok(iso_currency::Currency::XSU),
            Currency::CodesSpecificallyReservedForTestingPurposes => {
                Ok(iso_currency::Currency::XTS)
            }
            Currency::AdbUnitAccount => Ok(iso_currency::Currency::XUA),
            Currency::TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved => {
                Ok(iso_currency::Currency::XXX)
            }
            Currency::YemeniRial => Ok(iso_currency::Currency::YER),
            Currency::Rand => Ok(iso_currency::Currency::ZAR),
            Currency::ZambianKwacha => Ok(iso_currency::Currency::ZMW),
            Currency::ZimbabweGold => Ok(iso_currency::Currency::ZWG),
            Currency::NetherlandsAntilleanGuilder => {
                Err(ErrFromCurrency::NetherlandsAntilleanGuilder)
            }
        }
    }
}

#[cfg(feature = "iso_currency")]
/// All the variants of Currency that are not matched to any variant of iso_currency::Currency
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrFromCurrency {
    NetherlandsAntilleanGuilder,
}

#[cfg(feature = "iso_currency")]
impl std::fmt::Display for ErrFromCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrFromCurrency::NetherlandsAntilleanGuilder => write!(
                f,
                "NetherlandsAntilleanGuilder has no corresponding value in iso_currency::Currency"
            ),
        }
    }
}

#[cfg(feature = "iso_currency")]
impl std::error::Error for ErrFromCurrency {}

#[cfg(feature = "iso_currency")]
impl std::convert::TryFrom<iso_currency::Currency> for Currency {
    type Error = ErrFromIsoCurrencyCurrency;
    fn try_from(value: iso_currency::Currency) -> Result<Currency, Self::Error> {
        match value {
            iso_currency::Currency::AED => Ok(Currency::UaeDirham),
            iso_currency::Currency::AFN => Ok(Currency::Afghani),
            iso_currency::Currency::ALL => Ok(Currency::Lek),
            iso_currency::Currency::AMD => Ok(Currency::ArmenianDram),
            iso_currency::Currency::AOA => Ok(Currency::Kwanza),
            iso_currency::Currency::ARS => Ok(Currency::ArgentinePeso),
            iso_currency::Currency::AUD => Ok(Currency::AustralianDollar),
            iso_currency::Currency::AWG => Ok(Currency::ArubanFlorin),
            iso_currency::Currency::AZN => Ok(Currency::AzerbaijanManat),
            iso_currency::Currency::BAM => Ok(Currency::ConvertibleMark),
            iso_currency::Currency::BBD => Ok(Currency::BarbadosDollar),
            iso_currency::Currency::BDT => Ok(Currency::Taka),
            iso_currency::Currency::BGN => Ok(Currency::BulgarianLev),
            iso_currency::Currency::BHD => Ok(Currency::BahrainiDinar),
            iso_currency::Currency::BIF => Ok(Currency::BurundiFranc),
            iso_currency::Currency::BMD => Ok(Currency::BermudianDollar),
            iso_currency::Currency::BND => Ok(Currency::BruneiDollar),
            iso_currency::Currency::BOB => Ok(Currency::Boliviano),
            iso_currency::Currency::BOV => Ok(Currency::Mvdol),
            iso_currency::Currency::BRL => Ok(Currency::BrazilianReal),
            iso_currency::Currency::BSD => Ok(Currency::BahamianDollar),
            iso_currency::Currency::BTN => Ok(Currency::Ngultrum),
            iso_currency::Currency::BWP => Ok(Currency::Pula),
            iso_currency::Currency::BYN => Ok(Currency::BelarusianRuble),
            iso_currency::Currency::BZD => Ok(Currency::BelizeDollar),
            iso_currency::Currency::CAD => Ok(Currency::CanadianDollar),
            iso_currency::Currency::CDF => Ok(Currency::CongoleseFranc),
            iso_currency::Currency::CHE => Ok(Currency::WirEuro),
            iso_currency::Currency::CHF => Ok(Currency::SwissFranc),
            iso_currency::Currency::CHW => Ok(Currency::WirFranc),
            iso_currency::Currency::CLF => Ok(Currency::UnidadDeFomento),
            iso_currency::Currency::CLP => Ok(Currency::ChileanPeso),
            iso_currency::Currency::CNY => Ok(Currency::YuanRenminbi),
            iso_currency::Currency::COP => Ok(Currency::ColombianPeso),
            iso_currency::Currency::COU => Ok(Currency::UnidadDeValorReal),
            iso_currency::Currency::CRC => Ok(Currency::CostaRicanColon),
            iso_currency::Currency::CUP => Ok(Currency::CubanPeso),
            iso_currency::Currency::CVE => Ok(Currency::CaboVerdeEscudo),
            iso_currency::Currency::CZK => Ok(Currency::CzechKoruna),
            iso_currency::Currency::DJF => Ok(Currency::DjiboutiFranc),
            iso_currency::Currency::DKK => Ok(Currency::DanishKrone),
            iso_currency::Currency::DOP => Ok(Currency::DominicanPeso),
            iso_currency::Currency::DZD => Ok(Currency::AlgerianDinar),
            iso_currency::Currency::EGP => Ok(Currency::EgyptianPound),
            iso_currency::Currency::ERN => Ok(Currency::Nakfa),
            iso_currency::Currency::ETB => Ok(Currency::EthiopianBirr),
            iso_currency::Currency::EUR => Ok(Currency::Euro),
            iso_currency::Currency::FJD => Ok(Currency::FijiDollar),
            iso_currency::Currency::FKP => Ok(Currency::FalklandIslandsPound),
            iso_currency::Currency::GBP => Ok(Currency::PoundSterling),
            iso_currency::Currency::GEL => Ok(Currency::Lari),
            iso_currency::Currency::GHS => Ok(Currency::GhanaCedi),
            iso_currency::Currency::GIP => Ok(Currency::GibraltarPound),
            iso_currency::Currency::GMD => Ok(Currency::Dalasi),
            iso_currency::Currency::GNF => Ok(Currency::GuineanFranc),
            iso_currency::Currency::GTQ => Ok(Currency::Quetzal),
            iso_currency::Currency::GYD => Ok(Currency::GuyanaDollar),
            iso_currency::Currency::HKD => Ok(Currency::HongKongDollar),
            iso_currency::Currency::HNL => Ok(Currency::Lempira),
            iso_currency::Currency::HTG => Ok(Currency::Gourde),
            iso_currency::Currency::HUF => Ok(Currency::Forint),
            iso_currency::Currency::IDR => Ok(Currency::Rupiah),
            iso_currency::Currency::ILS => Ok(Currency::NewIsraeliSheqel),
            iso_currency::Currency::INR => Ok(Currency::IndianRupee),
            iso_currency::Currency::IQD => Ok(Currency::IraqiDinar),
            iso_currency::Currency::IRR => Ok(Currency::IranianRial),
            iso_currency::Currency::ISK => Ok(Currency::IcelandKrona),
            iso_currency::Currency::JMD => Ok(Currency::JamaicanDollar),
            iso_currency::Currency::JOD => Ok(Currency::JordanianDinar),
            iso_currency::Currency::JPY => Ok(Currency::Yen),
            iso_currency::Currency::KES => Ok(Currency::KenyanShilling),
            iso_currency::Currency::KGS => Ok(Currency::Som),
            iso_currency::Currency::KHR => Ok(Currency::Riel),
            iso_currency::Currency::KMF => Ok(Currency::ComorianFranc),
            iso_currency::Currency::KPW => Ok(Currency::NorthKoreanWon),
            iso_currency::Currency::KRW => Ok(Currency::Won),
            iso_currency::Currency::KWD => Ok(Currency::KuwaitiDinar),
            iso_currency::Currency::KYD => Ok(Currency::CaymanIslandsDollar),
            iso_currency::Currency::KZT => Ok(Currency::Tenge),
            iso_currency::Currency::LAK => Ok(Currency::LaoKip),
            iso_currency::Currency::LBP => Ok(Currency::LebanesePound),
            iso_currency::Currency::LKR => Ok(Currency::SriLankaRupee),
            iso_currency::Currency::LRD => Ok(Currency::LiberianDollar),
            iso_currency::Currency::LSL => Ok(Currency::Loti),
            iso_currency::Currency::LYD => Ok(Currency::LibyanDinar),
            iso_currency::Currency::MAD => Ok(Currency::MoroccanDirham),
            iso_currency::Currency::MDL => Ok(Currency::MoldovanLeu),
            iso_currency::Currency::MGA => Ok(Currency::MalagasyAriary),
            iso_currency::Currency::MKD => Ok(Currency::Denar),
            iso_currency::Currency::MMK => Ok(Currency::Kyat),
            iso_currency::Currency::MNT => Ok(Currency::Tugrik),
            iso_currency::Currency::MOP => Ok(Currency::Pataca),
            iso_currency::Currency::MRU => Ok(Currency::Ouguiya),
            iso_currency::Currency::MUR => Ok(Currency::MauritiusRupee),
            iso_currency::Currency::MVR => Ok(Currency::Rufiyaa),
            iso_currency::Currency::MWK => Ok(Currency::MalawiKwacha),
            iso_currency::Currency::MXN => Ok(Currency::MexicanPeso),
            iso_currency::Currency::MXV => Ok(Currency::MexicanUnidadDeInversionUdi),
            iso_currency::Currency::MYR => Ok(Currency::MalaysianRinggit),
            iso_currency::Currency::MZN => Ok(Currency::MozambiqueMetical),
            iso_currency::Currency::NAD => Ok(Currency::NamibiaDollar),
            iso_currency::Currency::NGN => Ok(Currency::Naira),
            iso_currency::Currency::NIO => Ok(Currency::CordobaOro),
            iso_currency::Currency::NOK => Ok(Currency::NorwegianKrone),
            iso_currency::Currency::NPR => Ok(Currency::NepaleseRupee),
            iso_currency::Currency::NZD => Ok(Currency::NewZealandDollar),
            iso_currency::Currency::OMR => Ok(Currency::RialOmani),
            iso_currency::Currency::PAB => Ok(Currency::Balboa),
            iso_currency::Currency::PEN => Ok(Currency::Sol),
            iso_currency::Currency::PGK => Ok(Currency::Kina),
            iso_currency::Currency::PHP => Ok(Currency::PhilippinePeso),
            iso_currency::Currency::PKR => Ok(Currency::PakistanRupee),
            iso_currency::Currency::PLN => Ok(Currency::Zloty),
            iso_currency::Currency::PYG => Ok(Currency::Guarani),
            iso_currency::Currency::QAR => Ok(Currency::QatariRial),
            iso_currency::Currency::RON => Ok(Currency::RomanianLeu),
            iso_currency::Currency::RSD => Ok(Currency::SerbianDinar),
            iso_currency::Currency::RUB => Ok(Currency::RussianRuble),
            iso_currency::Currency::RWF => Ok(Currency::RwandaFranc),
            iso_currency::Currency::SAR => Ok(Currency::SaudiRiyal),
            iso_currency::Currency::SBD => Ok(Currency::SolomonIslandsDollar),
            iso_currency::Currency::SCR => Ok(Currency::SeychellesRupee),
            iso_currency::Currency::SDG => Ok(Currency::SudanesePound),
            iso_currency::Currency::SEK => Ok(Currency::SwedishKrona),
            iso_currency::Currency::SGD => Ok(Currency::SingaporeDollar),
            iso_currency::Currency::SHP => Ok(Currency::SaintHelenaPound),
            iso_currency::Currency::SLE => Ok(Currency::SierraLeoneNewValuation2022),
            iso_currency::Currency::SOS => Ok(Currency::SomaliShilling),
            iso_currency::Currency::SRD => Ok(Currency::SurinamDollar),
            iso_currency::Currency::SSP => Ok(Currency::SouthSudanesePound),
            iso_currency::Currency::STN => Ok(Currency::Dobra),
            iso_currency::Currency::SVC => Ok(Currency::ElSalvadorColon),
            iso_currency::Currency::SYP => Ok(Currency::SyrianPound),
            iso_currency::Currency::SZL => Ok(Currency::Lilangeni),
            iso_currency::Currency::THB => Ok(Currency::Baht),
            iso_currency::Currency::TJS => Ok(Currency::Somoni),
            iso_currency::Currency::TMT => Ok(Currency::TurkmenistanNewManat),
            iso_currency::Currency::TND => Ok(Currency::TunisianDinar),
            iso_currency::Currency::TOP => Ok(Currency::PaAnga),
            iso_currency::Currency::TRY => Ok(Currency::TurkishLira),
            iso_currency::Currency::TTD => Ok(Currency::TrinidadAndTobagoDollar),
            iso_currency::Currency::TWD => Ok(Currency::NewTaiwanDollar),
            iso_currency::Currency::TZS => Ok(Currency::TanzanianShilling),
            iso_currency::Currency::UAH => Ok(Currency::Hryvnia),
            iso_currency::Currency::UGX => Ok(Currency::UgandaShilling),
            iso_currency::Currency::USD => Ok(Currency::UsDollar),
            iso_currency::Currency::USN => Ok(Currency::UsDollarNextDay),
            iso_currency::Currency::UYI => Ok(Currency::UruguayPesoEnUnidadesIndexadasUi),
            iso_currency::Currency::UYU => Ok(Currency::PesoUruguayo),
            iso_currency::Currency::UYW => Ok(Currency::UnidadPrevisional),
            iso_currency::Currency::UZS => Ok(Currency::UzbekistanSum),
            iso_currency::Currency::VED => Ok(Currency::BolívarSoberanoNewValuation),
            iso_currency::Currency::VES => Ok(Currency::BolívarSoberano),
            iso_currency::Currency::VND => Ok(Currency::Dong),
            iso_currency::Currency::VUV => Ok(Currency::Vatu),
            iso_currency::Currency::WST => Ok(Currency::Tala),
            iso_currency::Currency::XAF => Ok(Currency::CfaFrancBeac),
            iso_currency::Currency::XAG => Ok(Currency::Silver),
            iso_currency::Currency::XAU => Ok(Currency::Gold),
            iso_currency::Currency::XBA => Ok(Currency::BondMarketsUnitEuropeanCompositeUnitEurco),
            iso_currency::Currency::XBB => Ok(Currency::BondMarketsUnitEuropeanMonetaryUnitEMU6),
            iso_currency::Currency::XBC => Ok(Currency::BondMarketsUnitEuropeanUnitAccount9EUA9),
            iso_currency::Currency::XBD => Ok(Currency::BondMarketsUnitEuropeanUnitAccount17EUA17),
            iso_currency::Currency::XCD => Ok(Currency::EastCaribbeanDollar),
            iso_currency::Currency::XDR => Ok(Currency::SdrSpecialDrawingRight),
            iso_currency::Currency::XOF => Ok(Currency::CfaFrancBceao),
            iso_currency::Currency::XPD => Ok(Currency::Palladium),
            iso_currency::Currency::XPF => Ok(Currency::CfpFranc),
            iso_currency::Currency::XPT => Ok(Currency::Platinum),
            iso_currency::Currency::XSU => Ok(Currency::Sucre),
            iso_currency::Currency::XTS => {
                Ok(Currency::CodesSpecificallyReservedForTestingPurposes)
            }
            iso_currency::Currency::XUA => Ok(Currency::AdbUnitAccount),
            iso_currency::Currency::XXX => {
                Ok(Currency::TheCodesAssignedForTransactionsWhereNoCurrencyIsInvolved)
            }
            iso_currency::Currency::YER => Ok(Currency::YemeniRial),
            iso_currency::Currency::ZAR => Ok(Currency::Rand),
            iso_currency::Currency::ZMW => Ok(Currency::ZambianKwacha),
            iso_currency::Currency::ZWG => Ok(Currency::ZimbabweGold),
            iso_currency::Currency::XCG => Err(ErrFromIsoCurrencyCurrency::XCG),
            iso_currency::Currency::CUC => Err(ErrFromIsoCurrencyCurrency::CUC),
            iso_currency::Currency::HRK => Err(ErrFromIsoCurrencyCurrency::HRK),
            iso_currency::Currency::SLL => Err(ErrFromIsoCurrencyCurrency::SLL),
            iso_currency::Currency::ZWL => Err(ErrFromIsoCurrencyCurrency::ZWL),
        }
    }
}

#[cfg(feature = "iso_currency")]
/// All the variants of iso_currency::Currency that are not matched to any variant of Currency
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrFromIsoCurrencyCurrency {
    XCG,
    CUC,
    HRK,
    SLL,
    ZWL,
}

#[cfg(feature = "iso_currency")]
impl std::fmt::Display for ErrFromIsoCurrencyCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrFromIsoCurrencyCurrency::XCG => {
                write!(f, "XCG has no corresponding value in Currency")
            }
            ErrFromIsoCurrencyCurrency::CUC => {
                write!(f, "CUC has no corresponding value in Currency")
            }
            ErrFromIsoCurrencyCurrency::HRK => {
                write!(f, "HRK has no corresponding value in Currency")
            }
            ErrFromIsoCurrencyCurrency::SLL => {
                write!(f, "SLL has no corresponding value in Currency")
            }
            ErrFromIsoCurrencyCurrency::ZWL => {
                write!(f, "ZWL has no corresponding value in Currency")
            }
        }
    }
}

#[cfg(feature = "iso_currency")]
impl std::error::Error for ErrFromIsoCurrencyCurrency {}
