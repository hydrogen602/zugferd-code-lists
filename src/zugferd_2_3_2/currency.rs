#![allow(non_camel_case_types)]

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
    /// Peso Convertible
    PesoConvertible,
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
    /// Zimbabwe Dollar
    ZimbabweDollar,
}

impl crate::Code for Currency {
    fn code(&self) -> &str {
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
            Currency::PesoConvertible => "CUC",
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
            Currency::ZimbabweDollar => "ZWL",
        }
    }
}

impl crate::Description for Currency {
    fn description(&self) -> &str {
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
            Currency::PesoConvertible => "Peso Convertible",
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
            Currency::ZimbabweDollar => "Zimbabwe Dollar",
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
            "CUC" => Some(Currency::PesoConvertible),
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
            "ZWL" => Some(Currency::ZimbabweDollar),
            _ => None,
        }
    }
}
