#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Language {
    /// Afar
    Afar,
    /// Abkhazian
    Abkhazian,
    /// Achinese
    Achinese,
    /// Acoli
    Acoli,
    /// Adangme
    Adangme,
    /// Adyghe; Adygei
    AdygheAdygei,
    /// Afro-Asiatic (Other)
    AfroAsiaticOther,
    /// Afrihili
    Afrihili,
    /// Afrikaans
    Afrikaans,
    /// Ainu
    Ainu,
    /// Akan
    Akan,
    /// Akkadian
    Akkadian,
    /// Albanian
    Albanian,
    /// Aleut
    Aleut,
    /// Algonquian languages
    AlgonquianLanguages,
    /// Southern Altai
    SouthernAltai,
    /// Amharic
    Amharic,
    /// English, Old (ca.450-1100)
    EnglishOldCa4501100,
    /// Angika
    Angika,
    /// Apache languages
    ApacheLanguages,
    /// Arabic
    Arabic,
    /// Aramaic
    Aramaic,
    /// Aragonese
    Aragonese,
    /// Armenian
    Armenian,
    /// Mapudungun; Mapuche
    MapudungunMapuche,
    /// Arapaho
    Arapaho,
    /// Artificial (Other)
    ArtificialOther,
    /// Arawak
    Arawak,
    /// Assamese
    Assamese,
    /// Asturian; Bable
    AsturianBable,
    /// Athapascan languages
    AthapascanLanguages,
    /// Australian languages
    AustralianLanguages,
    /// Avaric
    Avaric,
    /// Avestan
    Avestan,
    /// Awadhi
    Awadhi,
    /// Aymara
    Aymara,
    /// Azerbaijani
    Azerbaijani,
    /// Banda languages
    BandaLanguages,
    /// Bamileke languages
    BamilekeLanguages,
    /// Bashkir
    Bashkir,
    /// Baluchi
    Baluchi,
    /// Bambara
    Bambara,
    /// Balinese
    Balinese,
    /// Basque
    Basque,
    /// Basa
    Basa,
    /// Baltic (Other)
    BalticOther,
    /// Beja
    Beja,
    /// Belarusian
    Belarusian,
    /// Bemba
    Bemba,
    /// Bengali
    Bengali,
    /// Berber (Other)
    BerberOther,
    /// Bhojpuri
    Bhojpuri,
    /// Bihari
    Bihari,
    /// Bikol
    Bikol,
    /// Bini; Edo
    BiniEdo,
    /// Bislama
    Bislama,
    /// Siksika
    Siksika,
    /// Bantu (Other)
    BantuOther,
    /// Bosnian
    Bosnian,
    /// Braj
    Braj,
    /// Breton
    Breton,
    /// Batak languages
    BatakLanguages,
    /// Buriat
    Buriat,
    /// Buginese
    Buginese,
    /// Bulgarian
    Bulgarian,
    /// Burmese
    Burmese,
    /// Blin; Bilin
    BlinBilin,
    /// Caddo
    Caddo,
    /// Central American Indian (Other)
    CentralAmericanIndianOther,
    /// Galibi Carib
    GalibiCarib,
    /// Catalan; Valencian
    CatalanValencian,
    /// Caucasian (Other)
    CaucasianOther,
    /// Cebuano
    Cebuano,
    /// Celtic (Other)
    CelticOther,
    /// Chamorro
    Chamorro,
    /// Chibcha
    Chibcha,
    /// Chechen
    Chechen,
    /// Chagatai
    Chagatai,
    /// Chinese
    Chinese,
    /// Chuukese
    Chuukese,
    /// Mari
    Mari,
    /// Chinook jargon
    ChinookJargon,
    /// Choctaw
    Choctaw,
    /// Chipewyan
    Chipewyan,
    /// Cherokee
    Cherokee,
    /// Church Slavic; Old Slavonic; Church Slavonic; Old Bulgarian; Old Church Slavonic
    ChurchSlavicOldSlavonicChurchSlavonicOldBulgarianOldChurchSlavonic,
    /// Chuvash
    Chuvash,
    /// Cheyenne
    Cheyenne,
    /// Chamic languages
    ChamicLanguages,
    /// Coptic
    Coptic,
    /// Cornish
    Cornish,
    /// Corsican
    Corsican,
    /// Creoles and pidgins, English based (Other)
    CreolesAndPidginsEnglishBasedOther,
    /// Creoles and pidgins, French-based (Other)
    CreolesAndPidginsFrenchBasedOther,
    /// Creoles and pidgins, Portuguese-based (Other)
    CreolesAndPidginsPortugueseBasedOther,
    /// Cree
    Cree,
    /// Crimean Tatar; Crimean Turkish
    CrimeanTatarCrimeanTurkish,
    /// Creoles and pidgins (Other)
    CreolesAndPidginsOther,
    /// Kashubian
    Kashubian,
    /// Cushitic (Other)
    CushiticOther,
    /// Czech
    Czech,
    /// Dakota
    Dakota,
    /// Danish
    Danish,
    /// Dargwa
    Dargwa,
    /// Land Dayak languages
    LandDayakLanguages,
    /// Delaware
    Delaware,
    /// Slave (Athapascan)
    SlaveAthapascan,
    /// Dogrib
    Dogrib,
    /// Dinka
    Dinka,
    /// Divehi; Dhivehi; Maldivian
    DivehiDhivehiMaldivian,
    /// Dogri
    Dogri,
    /// Dravidian (Other)
    DravidianOther,
    /// Lower Sorbian
    LowerSorbian,
    /// Duala
    Duala,
    /// Dutch, Middle (ca.1050-1350)
    DutchMiddleCa10501350,
    /// Dutch; Flemish
    DutchFlemish,
    /// Dyula
    Dyula,
    /// Dzongkha
    Dzongkha,
    /// Efik
    Efik,
    /// Egyptian (Ancient)
    EgyptianAncient,
    /// Ekajuk
    Ekajuk,
    /// Elamite
    Elamite,
    /// English
    English,
    /// English, Middle (1100-1500)
    EnglishMiddle11001500,
    /// Esperanto
    Esperanto,
    /// Estonian
    Estonian,
    /// Ewe
    Ewe,
    /// Ewondo
    Ewondo,
    /// Fang
    Fang,
    /// Faroese
    Faroese,
    /// Fanti
    Fanti,
    /// Fijian
    Fijian,
    /// Filipino; Pilipino
    FilipinoPilipino,
    /// Finnish
    Finnish,
    /// Finno-Ugrian (Other)
    FinnoUgrianOther,
    /// Fon
    Fon,
    /// French
    French,
    /// French, Middle (ca.1400-1600)
    FrenchMiddleCa14001600,
    /// French, Old (842-ca.1400)
    FrenchOld842Ca1400,
    /// Northern Frisian
    NorthernFrisian,
    /// Eastern Frisian
    EasternFrisian,
    /// Western Frisian
    WesternFrisian,
    /// Fulah
    Fulah,
    /// Friulian
    Friulian,
    /// Ga
    Ga,
    /// Gayo
    Gayo,
    /// Gbaya
    Gbaya,
    /// Germanic (Other)
    GermanicOther,
    /// Georgian
    Georgian,
    /// German
    German,
    /// Geez
    Geez,
    /// Gilbertese
    Gilbertese,
    /// Gaelic; Scottish Gaelic
    GaelicScottishGaelic,
    /// Irish
    Irish,
    /// Galician
    Galician,
    /// Manx
    Manx,
    /// German, Middle High (ca.1050-1500)
    GermanMiddleHighCa10501500,
    /// German, Old High (ca.750-1050)
    GermanOldHighCa7501050,
    /// Gondi
    Gondi,
    /// Gorontalo
    Gorontalo,
    /// Gothic
    Gothic,
    /// Grebo
    Grebo,
    /// Greek, Ancient (to 1453)
    GreekAncientTo1453,
    /// Greek, Modern (1453-)
    GreekModern1453,
    /// Guarani
    Guarani,
    /// Swiss German; Alemannic
    SwissGermanAlemannic,
    /// Gujarati
    Gujarati,
    /// Gwich'in
    Gwichin,
    /// Haida
    Haida,
    /// Haitian; Haitian Creole
    HaitianHaitianCreole,
    /// Hausa
    Hausa,
    /// Hawaiian
    Hawaiian,
    /// Hebrew
    Hebrew,
    /// Herero
    Herero,
    /// Hiligaynon
    Hiligaynon,
    /// Himachali
    Himachali,
    /// Hindi
    Hindi,
    /// Hittite
    Hittite,
    /// Hmong
    Hmong,
    /// Hiri Motu
    HiriMotu,
    /// Upper Sorbian
    UpperSorbian,
    /// Hungarian
    Hungarian,
    /// Hupa
    Hupa,
    /// Iban
    Iban,
    /// Igbo
    Igbo,
    /// Icelandic
    Icelandic,
    /// Ido
    Ido,
    /// Sichuan Yi
    SichuanYi,
    /// Ijo languages
    IjoLanguages,
    /// Inuktitut
    Inuktitut,
    /// Interlingue
    Interlingue,
    /// Iloko
    Iloko,
    /// Interlingua (International Auxiliary Language Association)
    InterlinguaInternationalAuxiliaryLanguageAssociation,
    /// Indic (Other)
    IndicOther,
    /// Indonesian
    Indonesian,
    /// Indo-European (Other)
    IndoEuropeanOther,
    /// Ingush
    Ingush,
    /// Inupiaq
    Inupiaq,
    /// Iranian (Other)
    IranianOther,
    /// Iroquoian languages
    IroquoianLanguages,
    /// Italian
    Italian,
    /// Javanese
    Javanese,
    /// Lojban
    Lojban,
    /// Japanese
    Japanese,
    /// Judeo-Persian
    JudeoPersian,
    /// Judeo-Arabic
    JudeoArabic,
    /// Kara-Kalpak
    KaraKalpak,
    /// Kabyle
    Kabyle,
    /// Kachin; Jingpho
    KachinJingpho,
    /// Kalaallisut; Greenlandic
    KalaallisutGreenlandic,
    /// Kamba
    Kamba,
    /// Kannada
    Kannada,
    /// Karen languages
    KarenLanguages,
    /// Kashmiri
    Kashmiri,
    /// Kanuri
    Kanuri,
    /// Kawi
    Kawi,
    /// Kazakh
    Kazakh,
    /// Kabardian
    Kabardian,
    /// Khasi
    Khasi,
    /// Khoisan (Other)
    KhoisanOther,
    /// Central Khmer
    CentralKhmer,
    /// Khotanese
    Khotanese,
    /// Kikuyu; Gikuyu
    KikuyuGikuyu,
    /// Kinyarwanda
    Kinyarwanda,
    /// Kirghiz; Kyrgyz
    KirghizKyrgyz,
    /// Kimbundu
    Kimbundu,
    /// Konkani
    Konkani,
    /// Komi
    Komi,
    /// Kongo
    Kongo,
    /// Korean
    Korean,
    /// Kosraean
    Kosraean,
    /// Kpelle
    Kpelle,
    /// Karachay-Balkar
    KarachayBalkar,
    /// Karelian
    Karelian,
    /// Kru languages
    KruLanguages,
    /// Kurukh
    Kurukh,
    /// Kuanyama; Kwanyama
    KuanyamaKwanyama,
    /// Kumyk
    Kumyk,
    /// Kurdish
    Kurdish,
    /// Kutenai
    Kutenai,
    /// Ladino
    Ladino,
    /// Lahnda
    Lahnda,
    /// Lamba
    Lamba,
    /// Lao
    Lao,
    /// Latin
    Latin,
    /// Latvian
    Latvian,
    /// Lezghian
    Lezghian,
    /// Limburgan; Limburger; Limburgish
    LimburganLimburgerLimburgish,
    /// Lingala
    Lingala,
    /// Lithuanian
    Lithuanian,
    /// Mongo
    Mongo,
    /// Lozi
    Lozi,
    /// Luxembourgish; Letzeburgesch
    LuxembourgishLetzeburgesch,
    /// Luba-Lulua
    LubaLulua,
    /// Luba-Katanga
    LubaKatanga,
    /// Ganda
    Ganda,
    /// Luiseno
    Luiseno,
    /// Lunda
    Lunda,
    /// Luo (Kenya and Tanzania)
    LuoKenyaAndTanzania,
    /// Lushai
    Lushai,
    /// Macedonian
    Macedonian,
    /// Madurese
    Madurese,
    /// Magahi
    Magahi,
    /// Marshallese
    Marshallese,
    /// Maithili
    Maithili,
    /// Makasar
    Makasar,
    /// Malayalam
    Malayalam,
    /// Mandingo
    Mandingo,
    /// Maori
    Maori,
    /// Austronesian (Other)
    AustronesianOther,
    /// Marathi
    Marathi,
    /// Masai
    Masai,
    /// Malay
    Malay,
    /// Moksha
    Moksha,
    /// Mandar
    Mandar,
    /// Mende
    Mende,
    /// Irish, Middle (900-1200)
    IrishMiddle9001200,
    /// Mi'kmaq; Micmac
    MikmaqMicmac,
    /// Minangkabau
    Minangkabau,
    /// Miscellaneous languages
    MiscellaneousLanguages,
    /// Mon-Khmer (Other)
    MonKhmerOther,
    /// Malagasy
    Malagasy,
    /// Maltese
    Maltese,
    /// Manchu
    Manchu,
    /// Manipuri
    Manipuri,
    /// Manobo languages
    ManoboLanguages,
    /// Mohawk
    Mohawk,
    /// Moldavian
    Moldavian,
    /// Mongolian
    Mongolian,
    /// Mossi
    Mossi,
    /// Multiple languages
    MultipleLanguages,
    /// Munda languages
    MundaLanguages,
    /// Creek
    Creek,
    /// Mirandese
    Mirandese,
    /// Marwari
    Marwari,
    /// Mayan languages
    MayanLanguages,
    /// Erzya
    Erzya,
    /// Nahuatl languages
    NahuatlLanguages,
    /// North American Indian
    NorthAmericanIndian,
    /// Neapolitan
    Neapolitan,
    /// Nauru
    Nauru,
    /// Navajo; Navaho
    NavajoNavaho,
    /// Ndebele, South; South Ndebele
    NdebeleSouthSouthNdebele,
    /// Ndebele, North; North Ndebele
    NdebeleNorthNorthNdebele,
    /// Ndonga
    Ndonga,
    /// Low German; Low Saxon; German, Low; Saxon, Low
    LowGermanLowSaxonGermanLowSaxonLow,
    /// Nepali
    Nepali,
    /// Nepal Bhasa; Newari
    NepalBhasaNewari,
    /// Nias
    Nias,
    /// Niger-Kordofanian (Other)
    NigerKordofanianOther,
    /// Niuean
    Niuean,
    /// Norwegian Nynorsk; Nynorsk, Norwegian
    NorwegianNynorskNynorskNorwegian,
    /// Bokmål, Norwegian; Norwegian Bokmål
    BokmålNorwegianNorwegianBokmål,
    /// Nogai
    Nogai,
    /// Norse, Old
    NorseOld,
    /// Norwegian
    Norwegian,
    /// N'Ko
    Nko,
    /// Pedi; Sepedi; Northern Sotho
    PediSepediNorthernSotho,
    /// Nubian languages
    NubianLanguages,
    /// Classical Newari; Old Newari; Classical Nepal Bhasa
    ClassicalNewariOldNewariClassicalNepalBhasa,
    /// Chichewa; Chewa; Nyanja
    ChichewaChewaNyanja,
    /// Nyamwezi
    Nyamwezi,
    /// Nyankole
    Nyankole,
    /// Nyoro
    Nyoro,
    /// Nzima
    Nzima,
    /// Occitan (post 1500); Provençal
    OccitanPost1500Provençal,
    /// Ojibwa
    Ojibwa,
    /// Oriya
    Oriya,
    /// Oromo
    Oromo,
    /// Osage
    Osage,
    /// Ossetian; Ossetic
    OssetianOssetic,
    /// Turkish, Ottoman (1500-1928)
    TurkishOttoman15001928,
    /// Otomian languages
    OtomianLanguages,
    /// Papuan (Other)
    PapuanOther,
    /// Pangasinan
    Pangasinan,
    /// Pahlavi
    Pahlavi,
    /// Pampanga
    Pampanga,
    /// Panjabi; Punjabi
    PanjabiPunjabi,
    /// Papiamento
    Papiamento,
    /// Palauan
    Palauan,
    /// Persian, Old (ca.600-400 B.C.)
    PersianOldCa600400BC,
    /// Persian
    Persian,
    /// Philippine (Other)
    PhilippineOther,
    /// Phoenician
    Phoenician,
    /// Pali
    Pali,
    /// Polish
    Polish,
    /// Pohnpeian
    Pohnpeian,
    /// Portuguese
    Portuguese,
    /// Prakrit languages
    PrakritLanguages,
    /// Provençal, Old (to 1500)
    ProvençalOldTo1500,
    /// Pushto
    Pushto,
    /// Quechua
    Quechua,
    /// Rajasthani
    Rajasthani,
    /// Rapanui
    Rapanui,
    /// Rarotongan; Cook Islands Maori
    RarotonganCookIslandsMaori,
    /// Romance (Other)
    RomanceOther,
    /// Romansh
    Romansh,
    /// Romany
    Romany,
    /// Romanian
    Romanian,
    /// Rundi
    Rundi,
    /// Aromanian; Arumanian; Macedo-Romanian
    AromanianArumanianMacedoRomanian,
    /// Russian
    Russian,
    /// Sandawe
    Sandawe,
    /// Sango
    Sango,
    /// Yakut
    Yakut,
    /// South American Indian (Other)
    SouthAmericanIndianOther,
    /// Salishan languages
    SalishanLanguages,
    /// Samaritan Aramaic
    SamaritanAramaic,
    /// Sanskrit
    Sanskrit,
    /// Sasak
    Sasak,
    /// Santali
    Santali,
    /// Serbian
    Serbian,
    /// Sicilian
    Sicilian,
    /// Scots
    Scots,
    /// Croatian
    Croatian,
    /// Selkup
    Selkup,
    /// Semitic (Other)
    SemiticOther,
    /// Irish, Old (to 900)
    IrishOldTo900,
    /// Sign Languages
    SignLanguages,
    /// Shan
    Shan,
    /// Sidamo
    Sidamo,
    /// Sinhala; Sinhalese
    SinhalaSinhalese,
    /// Siouan languages
    SiouanLanguages,
    /// Sino-Tibetan (Other)
    SinoTibetanOther,
    /// Slavic (Other)
    SlavicOther,
    /// Slovak
    Slovak,
    /// Slovenian
    Slovenian,
    /// Southern Sami
    SouthernSami,
    /// Northern Sami
    NorthernSami,
    /// Sami languages (Other)
    SamiLanguagesOther,
    /// Lule Sami
    LuleSami,
    /// Inari Sami
    InariSami,
    /// Samoan
    Samoan,
    /// Skolt Sami
    SkoltSami,
    /// Shona
    Shona,
    /// Sindhi
    Sindhi,
    /// Soninke
    Soninke,
    /// Sogdian
    Sogdian,
    /// Somali
    Somali,
    /// Songhai languages
    SonghaiLanguages,
    /// Sotho, Southern
    SothoSouthern,
    /// Spanish; Castilian
    SpanishCastilian,
    /// Sardinian
    Sardinian,
    /// Sranan Tongo
    SrananTongo,
    /// Serer
    Serer,
    /// Nilo-Saharan (Other)
    NiloSaharanOther,
    /// Swati
    Swati,
    /// Sukuma
    Sukuma,
    /// Sundanese
    Sundanese,
    /// Susu
    Susu,
    /// Sumerian
    Sumerian,
    /// Swahili
    Swahili,
    /// Swedish
    Swedish,
    /// Syriac
    Syriac,
    /// Tahitian
    Tahitian,
    /// Tai (Other)
    TaiOther,
    /// Tamil
    Tamil,
    /// Tatar
    Tatar,
    /// Telugu
    Telugu,
    /// Timne
    Timne,
    /// Tereno
    Tereno,
    /// Tetum
    Tetum,
    /// Tajik
    Tajik,
    /// Tagalog
    Tagalog,
    /// Thai
    Thai,
    /// Tibetan
    Tibetan,
    /// Tigre
    Tigre,
    /// Tigrinya
    Tigrinya,
    /// Tiv
    Tiv,
    /// Tokelau
    Tokelau,
    /// Klingon; tlhIngan-Hol
    KlingonTlhinganHol,
    /// Tlingit
    Tlingit,
    /// Tamashek
    Tamashek,
    /// Tonga (Nyasa)
    TongaNyasa,
    /// Tonga (Tonga Islands)
    TongaTongaIslands,
    /// Tok Pisin
    TokPisin,
    /// Tsimshian
    Tsimshian,
    /// Tswana
    Tswana,
    /// Tsonga
    Tsonga,
    /// Turkmen
    Turkmen,
    /// Tumbuka
    Tumbuka,
    /// Tupi languages
    TupiLanguages,
    /// Turkish
    Turkish,
    /// Altaic (Other)
    AltaicOther,
    /// Tuvalu
    Tuvalu,
    /// Twi
    Twi,
    /// Tuvinian
    Tuvinian,
    /// Udmurt
    Udmurt,
    /// Ugaritic
    Ugaritic,
    /// Uighur; Uyghur
    UighurUyghur,
    /// Ukrainian
    Ukrainian,
    /// Umbundu
    Umbundu,
    /// Undetermined
    Undetermined,
    /// Urdu
    Urdu,
    /// Uzbek
    Uzbek,
    /// Vai
    Vai,
    /// Venda
    Venda,
    /// Vietnamese
    Vietnamese,
    /// Volapük
    Volapük,
    /// Votic
    Votic,
    /// Wakashan languages
    WakashanLanguages,
    /// Walamo
    Walamo,
    /// Waray
    Waray,
    /// Washo
    Washo,
    /// Welsh
    Welsh,
    /// Sorbian languages
    SorbianLanguages,
    /// Walloon
    Walloon,
    /// Wolof
    Wolof,
    /// Kalmyk; Oirat
    KalmykOirat,
    /// Xhosa
    Xhosa,
    /// Yao
    Yao,
    /// Yapese
    Yapese,
    /// Yiddish
    Yiddish,
    /// Yoruba
    Yoruba,
    /// Yupik languages
    YupikLanguages,
    /// Zapotec
    Zapotec,
    /// Zenaga
    Zenaga,
    /// Zhuang; Chuang
    ZhuangChuang,
    /// Zande languages
    ZandeLanguages,
    /// Zulu
    Zulu,
    /// Zuni
    Zuni,
    /// No linguistic content
    NoLinguisticContent,
    /// Zaza; Dimili; Dimli; Kirdki; Kirmanjki; Zazaki.
    ZazaDimiliDimliKirdkiKirmanjkiZazaki,
}

impl crate::Code for Language {
    fn code(self) -> &'static str {
        match self {
            Language::Afar => "aar",
            Language::Abkhazian => "abk",
            Language::Achinese => "ace",
            Language::Acoli => "ach",
            Language::Adangme => "ada",
            Language::AdygheAdygei => "ady",
            Language::AfroAsiaticOther => "afa",
            Language::Afrihili => "afh",
            Language::Afrikaans => "afr",
            Language::Ainu => "ain",
            Language::Akan => "aka",
            Language::Akkadian => "akk",
            Language::Albanian => "alb",
            Language::Aleut => "ale",
            Language::AlgonquianLanguages => "alg",
            Language::SouthernAltai => "alt",
            Language::Amharic => "amh",
            Language::EnglishOldCa4501100 => "ang",
            Language::Angika => "anp",
            Language::ApacheLanguages => "apa",
            Language::Arabic => "ara",
            Language::Aramaic => "arc",
            Language::Aragonese => "arg",
            Language::Armenian => "arm",
            Language::MapudungunMapuche => "arn",
            Language::Arapaho => "arp",
            Language::ArtificialOther => "art",
            Language::Arawak => "arw",
            Language::Assamese => "asm",
            Language::AsturianBable => "ast",
            Language::AthapascanLanguages => "ath",
            Language::AustralianLanguages => "aus",
            Language::Avaric => "ava",
            Language::Avestan => "ave",
            Language::Awadhi => "awa",
            Language::Aymara => "aym",
            Language::Azerbaijani => "aze",
            Language::BandaLanguages => "bad",
            Language::BamilekeLanguages => "bai",
            Language::Bashkir => "bak",
            Language::Baluchi => "bal",
            Language::Bambara => "bam",
            Language::Balinese => "ban",
            Language::Basque => "baq",
            Language::Basa => "bas",
            Language::BalticOther => "bat",
            Language::Beja => "bej",
            Language::Belarusian => "bel",
            Language::Bemba => "bem",
            Language::Bengali => "ben",
            Language::BerberOther => "ber",
            Language::Bhojpuri => "bho",
            Language::Bihari => "bih",
            Language::Bikol => "bik",
            Language::BiniEdo => "bin",
            Language::Bislama => "bis",
            Language::Siksika => "bla",
            Language::BantuOther => "bnt",
            Language::Bosnian => "bos",
            Language::Braj => "bra",
            Language::Breton => "bre",
            Language::BatakLanguages => "btk",
            Language::Buriat => "bua",
            Language::Buginese => "bug",
            Language::Bulgarian => "bul",
            Language::Burmese => "bur",
            Language::BlinBilin => "byn",
            Language::Caddo => "cad",
            Language::CentralAmericanIndianOther => "cai",
            Language::GalibiCarib => "car",
            Language::CatalanValencian => "cat",
            Language::CaucasianOther => "cau",
            Language::Cebuano => "ceb",
            Language::CelticOther => "cel",
            Language::Chamorro => "cha",
            Language::Chibcha => "chb",
            Language::Chechen => "che",
            Language::Chagatai => "chg",
            Language::Chinese => "chi",
            Language::Chuukese => "chk",
            Language::Mari => "chm",
            Language::ChinookJargon => "chn",
            Language::Choctaw => "cho",
            Language::Chipewyan => "chp",
            Language::Cherokee => "chr",
            Language::ChurchSlavicOldSlavonicChurchSlavonicOldBulgarianOldChurchSlavonic => "chu",
            Language::Chuvash => "chv",
            Language::Cheyenne => "chy",
            Language::ChamicLanguages => "cmc",
            Language::Coptic => "cop",
            Language::Cornish => "cor",
            Language::Corsican => "cos",
            Language::CreolesAndPidginsEnglishBasedOther => "cpe",
            Language::CreolesAndPidginsFrenchBasedOther => "cpf",
            Language::CreolesAndPidginsPortugueseBasedOther => "cpp",
            Language::Cree => "cre",
            Language::CrimeanTatarCrimeanTurkish => "crh",
            Language::CreolesAndPidginsOther => "crp",
            Language::Kashubian => "csb",
            Language::CushiticOther => "cus",
            Language::Czech => "cze",
            Language::Dakota => "dak",
            Language::Danish => "dan",
            Language::Dargwa => "dar",
            Language::LandDayakLanguages => "day",
            Language::Delaware => "del",
            Language::SlaveAthapascan => "den",
            Language::Dogrib => "dgr",
            Language::Dinka => "din",
            Language::DivehiDhivehiMaldivian => "div",
            Language::Dogri => "doi",
            Language::DravidianOther => "dra",
            Language::LowerSorbian => "dsb",
            Language::Duala => "dua",
            Language::DutchMiddleCa10501350 => "dum",
            Language::DutchFlemish => "dut",
            Language::Dyula => "dyu",
            Language::Dzongkha => "dzo",
            Language::Efik => "efi",
            Language::EgyptianAncient => "egy",
            Language::Ekajuk => "eka",
            Language::Elamite => "elx",
            Language::English => "eng",
            Language::EnglishMiddle11001500 => "enm",
            Language::Esperanto => "epo",
            Language::Estonian => "est",
            Language::Ewe => "ewe",
            Language::Ewondo => "ewo",
            Language::Fang => "fan",
            Language::Faroese => "fao",
            Language::Fanti => "fat",
            Language::Fijian => "fij",
            Language::FilipinoPilipino => "fil",
            Language::Finnish => "fin",
            Language::FinnoUgrianOther => "fiu",
            Language::Fon => "fon",
            Language::French => "fre",
            Language::FrenchMiddleCa14001600 => "frm",
            Language::FrenchOld842Ca1400 => "fro",
            Language::NorthernFrisian => "frr",
            Language::EasternFrisian => "frs",
            Language::WesternFrisian => "fry",
            Language::Fulah => "ful",
            Language::Friulian => "fur",
            Language::Ga => "gaa",
            Language::Gayo => "gay",
            Language::Gbaya => "gba",
            Language::GermanicOther => "gem",
            Language::Georgian => "geo",
            Language::German => "ger",
            Language::Geez => "gez",
            Language::Gilbertese => "gil",
            Language::GaelicScottishGaelic => "gla",
            Language::Irish => "gle",
            Language::Galician => "glg",
            Language::Manx => "glv",
            Language::GermanMiddleHighCa10501500 => "gmh",
            Language::GermanOldHighCa7501050 => "goh",
            Language::Gondi => "gon",
            Language::Gorontalo => "gor",
            Language::Gothic => "got",
            Language::Grebo => "grb",
            Language::GreekAncientTo1453 => "grc",
            Language::GreekModern1453 => "gre",
            Language::Guarani => "grn",
            Language::SwissGermanAlemannic => "gsw",
            Language::Gujarati => "guj",
            Language::Gwichin => "gwi",
            Language::Haida => "hai",
            Language::HaitianHaitianCreole => "hat",
            Language::Hausa => "hau",
            Language::Hawaiian => "haw",
            Language::Hebrew => "heb",
            Language::Herero => "her",
            Language::Hiligaynon => "hil",
            Language::Himachali => "him",
            Language::Hindi => "hin",
            Language::Hittite => "hit",
            Language::Hmong => "hmn",
            Language::HiriMotu => "hmo",
            Language::UpperSorbian => "hsb",
            Language::Hungarian => "hun",
            Language::Hupa => "hup",
            Language::Iban => "iba",
            Language::Igbo => "ibo",
            Language::Icelandic => "ice",
            Language::Ido => "ido",
            Language::SichuanYi => "iii",
            Language::IjoLanguages => "ijo",
            Language::Inuktitut => "iku",
            Language::Interlingue => "ile",
            Language::Iloko => "ilo",
            Language::InterlinguaInternationalAuxiliaryLanguageAssociation => "ina",
            Language::IndicOther => "inc",
            Language::Indonesian => "ind",
            Language::IndoEuropeanOther => "ine",
            Language::Ingush => "inh",
            Language::Inupiaq => "ipk",
            Language::IranianOther => "ira",
            Language::IroquoianLanguages => "iro",
            Language::Italian => "ita",
            Language::Javanese => "jav",
            Language::Lojban => "jbo",
            Language::Japanese => "jpn",
            Language::JudeoPersian => "jpr",
            Language::JudeoArabic => "jrb",
            Language::KaraKalpak => "kaa",
            Language::Kabyle => "kab",
            Language::KachinJingpho => "kac",
            Language::KalaallisutGreenlandic => "kal",
            Language::Kamba => "kam",
            Language::Kannada => "kan",
            Language::KarenLanguages => "kar",
            Language::Kashmiri => "kas",
            Language::Kanuri => "kau",
            Language::Kawi => "kaw",
            Language::Kazakh => "kaz",
            Language::Kabardian => "kbd",
            Language::Khasi => "kha",
            Language::KhoisanOther => "khi",
            Language::CentralKhmer => "khm",
            Language::Khotanese => "kho",
            Language::KikuyuGikuyu => "kik",
            Language::Kinyarwanda => "kin",
            Language::KirghizKyrgyz => "kir",
            Language::Kimbundu => "kmb",
            Language::Konkani => "kok",
            Language::Komi => "kom",
            Language::Kongo => "kon",
            Language::Korean => "kor",
            Language::Kosraean => "kos",
            Language::Kpelle => "kpe",
            Language::KarachayBalkar => "krc",
            Language::Karelian => "krl",
            Language::KruLanguages => "kro",
            Language::Kurukh => "kru",
            Language::KuanyamaKwanyama => "kua",
            Language::Kumyk => "kum",
            Language::Kurdish => "kur",
            Language::Kutenai => "kut",
            Language::Ladino => "lad",
            Language::Lahnda => "lah",
            Language::Lamba => "lam",
            Language::Lao => "lao",
            Language::Latin => "lat",
            Language::Latvian => "lav",
            Language::Lezghian => "lez",
            Language::LimburganLimburgerLimburgish => "lim",
            Language::Lingala => "lin",
            Language::Lithuanian => "lit",
            Language::Mongo => "lol",
            Language::Lozi => "loz",
            Language::LuxembourgishLetzeburgesch => "ltz",
            Language::LubaLulua => "lua",
            Language::LubaKatanga => "lub",
            Language::Ganda => "lug",
            Language::Luiseno => "lui",
            Language::Lunda => "lun",
            Language::LuoKenyaAndTanzania => "luo",
            Language::Lushai => "lus",
            Language::Macedonian => "mac",
            Language::Madurese => "mad",
            Language::Magahi => "mag",
            Language::Marshallese => "mah",
            Language::Maithili => "mai",
            Language::Makasar => "mak",
            Language::Malayalam => "mal",
            Language::Mandingo => "man",
            Language::Maori => "mao",
            Language::AustronesianOther => "map",
            Language::Marathi => "mar",
            Language::Masai => "mas",
            Language::Malay => "may",
            Language::Moksha => "mdf",
            Language::Mandar => "mdr",
            Language::Mende => "men",
            Language::IrishMiddle9001200 => "mga",
            Language::MikmaqMicmac => "mic",
            Language::Minangkabau => "min",
            Language::MiscellaneousLanguages => "mis",
            Language::MonKhmerOther => "mkh",
            Language::Malagasy => "mlg",
            Language::Maltese => "mlt",
            Language::Manchu => "mnc",
            Language::Manipuri => "mni",
            Language::ManoboLanguages => "mno",
            Language::Mohawk => "moh",
            Language::Moldavian => "mol",
            Language::Mongolian => "mon",
            Language::Mossi => "mos",
            Language::MultipleLanguages => "mul",
            Language::MundaLanguages => "mun",
            Language::Creek => "mus",
            Language::Mirandese => "mwl",
            Language::Marwari => "mwr",
            Language::MayanLanguages => "myn",
            Language::Erzya => "myv",
            Language::NahuatlLanguages => "nah",
            Language::NorthAmericanIndian => "nai",
            Language::Neapolitan => "nap",
            Language::Nauru => "nau",
            Language::NavajoNavaho => "nav",
            Language::NdebeleSouthSouthNdebele => "nbl",
            Language::NdebeleNorthNorthNdebele => "nde",
            Language::Ndonga => "ndo",
            Language::LowGermanLowSaxonGermanLowSaxonLow => "nds",
            Language::Nepali => "nep",
            Language::NepalBhasaNewari => "new",
            Language::Nias => "nia",
            Language::NigerKordofanianOther => "nic",
            Language::Niuean => "niu",
            Language::NorwegianNynorskNynorskNorwegian => "nno",
            Language::BokmålNorwegianNorwegianBokmål => "nob",
            Language::Nogai => "nog",
            Language::NorseOld => "non",
            Language::Norwegian => "nor",
            Language::Nko => "nqo",
            Language::PediSepediNorthernSotho => "nso",
            Language::NubianLanguages => "nub",
            Language::ClassicalNewariOldNewariClassicalNepalBhasa => "nwc",
            Language::ChichewaChewaNyanja => "nya",
            Language::Nyamwezi => "nym",
            Language::Nyankole => "nyn",
            Language::Nyoro => "nyo",
            Language::Nzima => "nzi",
            Language::OccitanPost1500Provençal => "oci",
            Language::Ojibwa => "oji",
            Language::Oriya => "ori",
            Language::Oromo => "orm",
            Language::Osage => "osa",
            Language::OssetianOssetic => "oss",
            Language::TurkishOttoman15001928 => "ota",
            Language::OtomianLanguages => "oto",
            Language::PapuanOther => "paa",
            Language::Pangasinan => "pag",
            Language::Pahlavi => "pal",
            Language::Pampanga => "pam",
            Language::PanjabiPunjabi => "pan",
            Language::Papiamento => "pap",
            Language::Palauan => "pau",
            Language::PersianOldCa600400BC => "peo",
            Language::Persian => "per",
            Language::PhilippineOther => "phi",
            Language::Phoenician => "phn",
            Language::Pali => "pli",
            Language::Polish => "pol",
            Language::Pohnpeian => "pon",
            Language::Portuguese => "por",
            Language::PrakritLanguages => "pra",
            Language::ProvençalOldTo1500 => "pro",
            Language::Pushto => "pus",
            Language::Quechua => "que",
            Language::Rajasthani => "raj",
            Language::Rapanui => "rap",
            Language::RarotonganCookIslandsMaori => "rar",
            Language::RomanceOther => "roa",
            Language::Romansh => "roh",
            Language::Romany => "rom",
            Language::Romanian => "rum",
            Language::Rundi => "run",
            Language::AromanianArumanianMacedoRomanian => "rup",
            Language::Russian => "rus",
            Language::Sandawe => "sad",
            Language::Sango => "sag",
            Language::Yakut => "sah",
            Language::SouthAmericanIndianOther => "sai",
            Language::SalishanLanguages => "sal",
            Language::SamaritanAramaic => "sam",
            Language::Sanskrit => "san",
            Language::Sasak => "sas",
            Language::Santali => "sat",
            Language::Serbian => "scc",
            Language::Sicilian => "scn",
            Language::Scots => "sco",
            Language::Croatian => "scr",
            Language::Selkup => "sel",
            Language::SemiticOther => "sem",
            Language::IrishOldTo900 => "sga",
            Language::SignLanguages => "sgn",
            Language::Shan => "shn",
            Language::Sidamo => "sid",
            Language::SinhalaSinhalese => "sin",
            Language::SiouanLanguages => "sio",
            Language::SinoTibetanOther => "sit",
            Language::SlavicOther => "sla",
            Language::Slovak => "slo",
            Language::Slovenian => "slv",
            Language::SouthernSami => "sma",
            Language::NorthernSami => "sme",
            Language::SamiLanguagesOther => "smi",
            Language::LuleSami => "smj",
            Language::InariSami => "smn",
            Language::Samoan => "smo",
            Language::SkoltSami => "sms",
            Language::Shona => "sna",
            Language::Sindhi => "snd",
            Language::Soninke => "snk",
            Language::Sogdian => "sog",
            Language::Somali => "som",
            Language::SonghaiLanguages => "son",
            Language::SothoSouthern => "sot",
            Language::SpanishCastilian => "spa",
            Language::Sardinian => "srd",
            Language::SrananTongo => "srn",
            Language::Serer => "srr",
            Language::NiloSaharanOther => "ssa",
            Language::Swati => "ssw",
            Language::Sukuma => "suk",
            Language::Sundanese => "sun",
            Language::Susu => "sus",
            Language::Sumerian => "sux",
            Language::Swahili => "swa",
            Language::Swedish => "swe",
            Language::Syriac => "syr",
            Language::Tahitian => "tah",
            Language::TaiOther => "tai",
            Language::Tamil => "tam",
            Language::Tatar => "tat",
            Language::Telugu => "tel",
            Language::Timne => "tem",
            Language::Tereno => "ter",
            Language::Tetum => "tet",
            Language::Tajik => "tgk",
            Language::Tagalog => "tgl",
            Language::Thai => "tha",
            Language::Tibetan => "tib",
            Language::Tigre => "tig",
            Language::Tigrinya => "tir",
            Language::Tiv => "tiv",
            Language::Tokelau => "tkl",
            Language::KlingonTlhinganHol => "tlh",
            Language::Tlingit => "tli",
            Language::Tamashek => "tmh",
            Language::TongaNyasa => "tog",
            Language::TongaTongaIslands => "ton",
            Language::TokPisin => "tpi",
            Language::Tsimshian => "tsi",
            Language::Tswana => "tsn",
            Language::Tsonga => "tso",
            Language::Turkmen => "tuk",
            Language::Tumbuka => "tum",
            Language::TupiLanguages => "tup",
            Language::Turkish => "tur",
            Language::AltaicOther => "tut",
            Language::Tuvalu => "tvl",
            Language::Twi => "twi",
            Language::Tuvinian => "tyv",
            Language::Udmurt => "udm",
            Language::Ugaritic => "uga",
            Language::UighurUyghur => "uig",
            Language::Ukrainian => "ukr",
            Language::Umbundu => "umb",
            Language::Undetermined => "und",
            Language::Urdu => "urd",
            Language::Uzbek => "uzb",
            Language::Vai => "vai",
            Language::Venda => "ven",
            Language::Vietnamese => "vie",
            Language::Volapük => "vol",
            Language::Votic => "vot",
            Language::WakashanLanguages => "wak",
            Language::Walamo => "wal",
            Language::Waray => "war",
            Language::Washo => "was",
            Language::Welsh => "wel",
            Language::SorbianLanguages => "wen",
            Language::Walloon => "wln",
            Language::Wolof => "wol",
            Language::KalmykOirat => "xal",
            Language::Xhosa => "xho",
            Language::Yao => "yao",
            Language::Yapese => "yap",
            Language::Yiddish => "yid",
            Language::Yoruba => "yor",
            Language::YupikLanguages => "ypk",
            Language::Zapotec => "zap",
            Language::Zenaga => "zen",
            Language::ZhuangChuang => "zha",
            Language::ZandeLanguages => "znd",
            Language::Zulu => "zul",
            Language::Zuni => "zun",
            Language::NoLinguisticContent => "zxx",
            Language::ZazaDimiliDimliKirdkiKirmanjkiZazaki => "zza",
        }
    }
}

impl crate::Description for Language {
    fn description(self) -> &'static str {
        match self {
            Language::Afar => "Afar",
            Language::Abkhazian => "Abkhazian",
            Language::Achinese => "Achinese",
            Language::Acoli => "Acoli",
            Language::Adangme => "Adangme",
            Language::AdygheAdygei => "Adyghe; Adygei",
            Language::AfroAsiaticOther => "Afro-Asiatic (Other)",
            Language::Afrihili => "Afrihili",
            Language::Afrikaans => "Afrikaans",
            Language::Ainu => "Ainu",
            Language::Akan => "Akan",
            Language::Akkadian => "Akkadian",
            Language::Albanian => "Albanian",
            Language::Aleut => "Aleut",
            Language::AlgonquianLanguages => "Algonquian languages",
            Language::SouthernAltai => "Southern Altai",
            Language::Amharic => "Amharic",
            Language::EnglishOldCa4501100 => "English, Old (ca.450-1100)",
            Language::Angika => "Angika",
            Language::ApacheLanguages => "Apache languages",
            Language::Arabic => "Arabic",
            Language::Aramaic => "Aramaic",
            Language::Aragonese => "Aragonese",
            Language::Armenian => "Armenian",
            Language::MapudungunMapuche => "Mapudungun; Mapuche",
            Language::Arapaho => "Arapaho",
            Language::ArtificialOther => "Artificial (Other)",
            Language::Arawak => "Arawak",
            Language::Assamese => "Assamese",
            Language::AsturianBable => "Asturian; Bable",
            Language::AthapascanLanguages => "Athapascan languages",
            Language::AustralianLanguages => "Australian languages",
            Language::Avaric => "Avaric",
            Language::Avestan => "Avestan",
            Language::Awadhi => "Awadhi",
            Language::Aymara => "Aymara",
            Language::Azerbaijani => "Azerbaijani",
            Language::BandaLanguages => "Banda languages",
            Language::BamilekeLanguages => "Bamileke languages",
            Language::Bashkir => "Bashkir",
            Language::Baluchi => "Baluchi",
            Language::Bambara => "Bambara",
            Language::Balinese => "Balinese",
            Language::Basque => "Basque",
            Language::Basa => "Basa",
            Language::BalticOther => "Baltic (Other)",
            Language::Beja => "Beja",
            Language::Belarusian => "Belarusian",
            Language::Bemba => "Bemba",
            Language::Bengali => "Bengali",
            Language::BerberOther => "Berber (Other)",
            Language::Bhojpuri => "Bhojpuri",
            Language::Bihari => "Bihari",
            Language::Bikol => "Bikol",
            Language::BiniEdo => "Bini; Edo",
            Language::Bislama => "Bislama",
            Language::Siksika => "Siksika",
            Language::BantuOther => "Bantu (Other)",
            Language::Bosnian => "Bosnian",
            Language::Braj => "Braj",
            Language::Breton => "Breton",
            Language::BatakLanguages => "Batak languages",
            Language::Buriat => "Buriat",
            Language::Buginese => "Buginese",
            Language::Bulgarian => "Bulgarian",
            Language::Burmese => "Burmese",
            Language::BlinBilin => "Blin; Bilin",
            Language::Caddo => "Caddo",
            Language::CentralAmericanIndianOther => "Central American Indian (Other)",
            Language::GalibiCarib => "Galibi Carib",
            Language::CatalanValencian => "Catalan; Valencian",
            Language::CaucasianOther => "Caucasian (Other)",
            Language::Cebuano => "Cebuano",
            Language::CelticOther => "Celtic (Other)",
            Language::Chamorro => "Chamorro",
            Language::Chibcha => "Chibcha",
            Language::Chechen => "Chechen",
            Language::Chagatai => "Chagatai",
            Language::Chinese => "Chinese",
            Language::Chuukese => "Chuukese",
            Language::Mari => "Mari",
            Language::ChinookJargon => "Chinook jargon",
            Language::Choctaw => "Choctaw",
            Language::Chipewyan => "Chipewyan",
            Language::Cherokee => "Cherokee",
            Language::ChurchSlavicOldSlavonicChurchSlavonicOldBulgarianOldChurchSlavonic => {
                "Church Slavic; Old Slavonic; Church Slavonic; Old Bulgarian; Old Church Slavonic"
            }
            Language::Chuvash => "Chuvash",
            Language::Cheyenne => "Cheyenne",
            Language::ChamicLanguages => "Chamic languages",
            Language::Coptic => "Coptic",
            Language::Cornish => "Cornish",
            Language::Corsican => "Corsican",
            Language::CreolesAndPidginsEnglishBasedOther => {
                "Creoles and pidgins, English based (Other)"
            }
            Language::CreolesAndPidginsFrenchBasedOther => {
                "Creoles and pidgins, French-based (Other)"
            }
            Language::CreolesAndPidginsPortugueseBasedOther => {
                "Creoles and pidgins, Portuguese-based (Other)"
            }
            Language::Cree => "Cree",
            Language::CrimeanTatarCrimeanTurkish => "Crimean Tatar; Crimean Turkish",
            Language::CreolesAndPidginsOther => "Creoles and pidgins (Other)",
            Language::Kashubian => "Kashubian",
            Language::CushiticOther => "Cushitic (Other)",
            Language::Czech => "Czech",
            Language::Dakota => "Dakota",
            Language::Danish => "Danish",
            Language::Dargwa => "Dargwa",
            Language::LandDayakLanguages => "Land Dayak languages",
            Language::Delaware => "Delaware",
            Language::SlaveAthapascan => "Slave (Athapascan)",
            Language::Dogrib => "Dogrib",
            Language::Dinka => "Dinka",
            Language::DivehiDhivehiMaldivian => "Divehi; Dhivehi; Maldivian",
            Language::Dogri => "Dogri",
            Language::DravidianOther => "Dravidian (Other)",
            Language::LowerSorbian => "Lower Sorbian",
            Language::Duala => "Duala",
            Language::DutchMiddleCa10501350 => "Dutch, Middle (ca.1050-1350)",
            Language::DutchFlemish => "Dutch; Flemish",
            Language::Dyula => "Dyula",
            Language::Dzongkha => "Dzongkha",
            Language::Efik => "Efik",
            Language::EgyptianAncient => "Egyptian (Ancient)",
            Language::Ekajuk => "Ekajuk",
            Language::Elamite => "Elamite",
            Language::English => "English",
            Language::EnglishMiddle11001500 => "English, Middle (1100-1500)",
            Language::Esperanto => "Esperanto",
            Language::Estonian => "Estonian",
            Language::Ewe => "Ewe",
            Language::Ewondo => "Ewondo",
            Language::Fang => "Fang",
            Language::Faroese => "Faroese",
            Language::Fanti => "Fanti",
            Language::Fijian => "Fijian",
            Language::FilipinoPilipino => "Filipino; Pilipino",
            Language::Finnish => "Finnish",
            Language::FinnoUgrianOther => "Finno-Ugrian (Other)",
            Language::Fon => "Fon",
            Language::French => "French",
            Language::FrenchMiddleCa14001600 => "French, Middle (ca.1400-1600)",
            Language::FrenchOld842Ca1400 => "French, Old (842-ca.1400)",
            Language::NorthernFrisian => "Northern Frisian",
            Language::EasternFrisian => "Eastern Frisian",
            Language::WesternFrisian => "Western Frisian",
            Language::Fulah => "Fulah",
            Language::Friulian => "Friulian",
            Language::Ga => "Ga",
            Language::Gayo => "Gayo",
            Language::Gbaya => "Gbaya",
            Language::GermanicOther => "Germanic (Other)",
            Language::Georgian => "Georgian",
            Language::German => "German",
            Language::Geez => "Geez",
            Language::Gilbertese => "Gilbertese",
            Language::GaelicScottishGaelic => "Gaelic; Scottish Gaelic",
            Language::Irish => "Irish",
            Language::Galician => "Galician",
            Language::Manx => "Manx",
            Language::GermanMiddleHighCa10501500 => "German, Middle High (ca.1050-1500)",
            Language::GermanOldHighCa7501050 => "German, Old High (ca.750-1050)",
            Language::Gondi => "Gondi",
            Language::Gorontalo => "Gorontalo",
            Language::Gothic => "Gothic",
            Language::Grebo => "Grebo",
            Language::GreekAncientTo1453 => "Greek, Ancient (to 1453)",
            Language::GreekModern1453 => "Greek, Modern (1453-)",
            Language::Guarani => "Guarani",
            Language::SwissGermanAlemannic => "Swiss German; Alemannic",
            Language::Gujarati => "Gujarati",
            Language::Gwichin => "Gwich'in",
            Language::Haida => "Haida",
            Language::HaitianHaitianCreole => "Haitian; Haitian Creole",
            Language::Hausa => "Hausa",
            Language::Hawaiian => "Hawaiian",
            Language::Hebrew => "Hebrew",
            Language::Herero => "Herero",
            Language::Hiligaynon => "Hiligaynon",
            Language::Himachali => "Himachali",
            Language::Hindi => "Hindi",
            Language::Hittite => "Hittite",
            Language::Hmong => "Hmong",
            Language::HiriMotu => "Hiri Motu",
            Language::UpperSorbian => "Upper Sorbian",
            Language::Hungarian => "Hungarian",
            Language::Hupa => "Hupa",
            Language::Iban => "Iban",
            Language::Igbo => "Igbo",
            Language::Icelandic => "Icelandic",
            Language::Ido => "Ido",
            Language::SichuanYi => "Sichuan Yi",
            Language::IjoLanguages => "Ijo languages",
            Language::Inuktitut => "Inuktitut",
            Language::Interlingue => "Interlingue",
            Language::Iloko => "Iloko",
            Language::InterlinguaInternationalAuxiliaryLanguageAssociation => {
                "Interlingua (International Auxiliary Language Association)"
            }
            Language::IndicOther => "Indic (Other)",
            Language::Indonesian => "Indonesian",
            Language::IndoEuropeanOther => "Indo-European (Other)",
            Language::Ingush => "Ingush",
            Language::Inupiaq => "Inupiaq",
            Language::IranianOther => "Iranian (Other)",
            Language::IroquoianLanguages => "Iroquoian languages",
            Language::Italian => "Italian",
            Language::Javanese => "Javanese",
            Language::Lojban => "Lojban",
            Language::Japanese => "Japanese",
            Language::JudeoPersian => "Judeo-Persian",
            Language::JudeoArabic => "Judeo-Arabic",
            Language::KaraKalpak => "Kara-Kalpak",
            Language::Kabyle => "Kabyle",
            Language::KachinJingpho => "Kachin; Jingpho",
            Language::KalaallisutGreenlandic => "Kalaallisut; Greenlandic",
            Language::Kamba => "Kamba",
            Language::Kannada => "Kannada",
            Language::KarenLanguages => "Karen languages",
            Language::Kashmiri => "Kashmiri",
            Language::Kanuri => "Kanuri",
            Language::Kawi => "Kawi",
            Language::Kazakh => "Kazakh",
            Language::Kabardian => "Kabardian",
            Language::Khasi => "Khasi",
            Language::KhoisanOther => "Khoisan (Other)",
            Language::CentralKhmer => "Central Khmer",
            Language::Khotanese => "Khotanese",
            Language::KikuyuGikuyu => "Kikuyu; Gikuyu",
            Language::Kinyarwanda => "Kinyarwanda",
            Language::KirghizKyrgyz => "Kirghiz; Kyrgyz",
            Language::Kimbundu => "Kimbundu",
            Language::Konkani => "Konkani",
            Language::Komi => "Komi",
            Language::Kongo => "Kongo",
            Language::Korean => "Korean",
            Language::Kosraean => "Kosraean",
            Language::Kpelle => "Kpelle",
            Language::KarachayBalkar => "Karachay-Balkar",
            Language::Karelian => "Karelian",
            Language::KruLanguages => "Kru languages",
            Language::Kurukh => "Kurukh",
            Language::KuanyamaKwanyama => "Kuanyama; Kwanyama",
            Language::Kumyk => "Kumyk",
            Language::Kurdish => "Kurdish",
            Language::Kutenai => "Kutenai",
            Language::Ladino => "Ladino",
            Language::Lahnda => "Lahnda",
            Language::Lamba => "Lamba",
            Language::Lao => "Lao",
            Language::Latin => "Latin",
            Language::Latvian => "Latvian",
            Language::Lezghian => "Lezghian",
            Language::LimburganLimburgerLimburgish => "Limburgan; Limburger; Limburgish",
            Language::Lingala => "Lingala",
            Language::Lithuanian => "Lithuanian",
            Language::Mongo => "Mongo",
            Language::Lozi => "Lozi",
            Language::LuxembourgishLetzeburgesch => "Luxembourgish; Letzeburgesch",
            Language::LubaLulua => "Luba-Lulua",
            Language::LubaKatanga => "Luba-Katanga",
            Language::Ganda => "Ganda",
            Language::Luiseno => "Luiseno",
            Language::Lunda => "Lunda",
            Language::LuoKenyaAndTanzania => "Luo (Kenya and Tanzania)",
            Language::Lushai => "Lushai",
            Language::Macedonian => "Macedonian",
            Language::Madurese => "Madurese",
            Language::Magahi => "Magahi",
            Language::Marshallese => "Marshallese",
            Language::Maithili => "Maithili",
            Language::Makasar => "Makasar",
            Language::Malayalam => "Malayalam",
            Language::Mandingo => "Mandingo",
            Language::Maori => "Maori",
            Language::AustronesianOther => "Austronesian (Other)",
            Language::Marathi => "Marathi",
            Language::Masai => "Masai",
            Language::Malay => "Malay",
            Language::Moksha => "Moksha",
            Language::Mandar => "Mandar",
            Language::Mende => "Mende",
            Language::IrishMiddle9001200 => "Irish, Middle (900-1200)",
            Language::MikmaqMicmac => "Mi'kmaq; Micmac",
            Language::Minangkabau => "Minangkabau",
            Language::MiscellaneousLanguages => "Miscellaneous languages",
            Language::MonKhmerOther => "Mon-Khmer (Other)",
            Language::Malagasy => "Malagasy",
            Language::Maltese => "Maltese",
            Language::Manchu => "Manchu",
            Language::Manipuri => "Manipuri",
            Language::ManoboLanguages => "Manobo languages",
            Language::Mohawk => "Mohawk",
            Language::Moldavian => "Moldavian",
            Language::Mongolian => "Mongolian",
            Language::Mossi => "Mossi",
            Language::MultipleLanguages => "Multiple languages",
            Language::MundaLanguages => "Munda languages",
            Language::Creek => "Creek",
            Language::Mirandese => "Mirandese",
            Language::Marwari => "Marwari",
            Language::MayanLanguages => "Mayan languages",
            Language::Erzya => "Erzya",
            Language::NahuatlLanguages => "Nahuatl languages",
            Language::NorthAmericanIndian => "North American Indian",
            Language::Neapolitan => "Neapolitan",
            Language::Nauru => "Nauru",
            Language::NavajoNavaho => "Navajo; Navaho",
            Language::NdebeleSouthSouthNdebele => "Ndebele, South; South Ndebele",
            Language::NdebeleNorthNorthNdebele => "Ndebele, North; North Ndebele",
            Language::Ndonga => "Ndonga",
            Language::LowGermanLowSaxonGermanLowSaxonLow => {
                "Low German; Low Saxon; German, Low; Saxon, Low"
            }
            Language::Nepali => "Nepali",
            Language::NepalBhasaNewari => "Nepal Bhasa; Newari",
            Language::Nias => "Nias",
            Language::NigerKordofanianOther => "Niger-Kordofanian (Other)",
            Language::Niuean => "Niuean",
            Language::NorwegianNynorskNynorskNorwegian => "Norwegian Nynorsk; Nynorsk, Norwegian",
            Language::BokmålNorwegianNorwegianBokmål => "Bokmål, Norwegian; Norwegian Bokmål",
            Language::Nogai => "Nogai",
            Language::NorseOld => "Norse, Old",
            Language::Norwegian => "Norwegian",
            Language::Nko => "N'Ko",
            Language::PediSepediNorthernSotho => "Pedi; Sepedi; Northern Sotho",
            Language::NubianLanguages => "Nubian languages",
            Language::ClassicalNewariOldNewariClassicalNepalBhasa => {
                "Classical Newari; Old Newari; Classical Nepal Bhasa"
            }
            Language::ChichewaChewaNyanja => "Chichewa; Chewa; Nyanja",
            Language::Nyamwezi => "Nyamwezi",
            Language::Nyankole => "Nyankole",
            Language::Nyoro => "Nyoro",
            Language::Nzima => "Nzima",
            Language::OccitanPost1500Provençal => "Occitan (post 1500); Provençal",
            Language::Ojibwa => "Ojibwa",
            Language::Oriya => "Oriya",
            Language::Oromo => "Oromo",
            Language::Osage => "Osage",
            Language::OssetianOssetic => "Ossetian; Ossetic",
            Language::TurkishOttoman15001928 => "Turkish, Ottoman (1500-1928)",
            Language::OtomianLanguages => "Otomian languages",
            Language::PapuanOther => "Papuan (Other)",
            Language::Pangasinan => "Pangasinan",
            Language::Pahlavi => "Pahlavi",
            Language::Pampanga => "Pampanga",
            Language::PanjabiPunjabi => "Panjabi; Punjabi",
            Language::Papiamento => "Papiamento",
            Language::Palauan => "Palauan",
            Language::PersianOldCa600400BC => "Persian, Old (ca.600-400 B.C.)",
            Language::Persian => "Persian",
            Language::PhilippineOther => "Philippine (Other)",
            Language::Phoenician => "Phoenician",
            Language::Pali => "Pali",
            Language::Polish => "Polish",
            Language::Pohnpeian => "Pohnpeian",
            Language::Portuguese => "Portuguese",
            Language::PrakritLanguages => "Prakrit languages",
            Language::ProvençalOldTo1500 => "Provençal, Old (to 1500)",
            Language::Pushto => "Pushto",
            Language::Quechua => "Quechua",
            Language::Rajasthani => "Rajasthani",
            Language::Rapanui => "Rapanui",
            Language::RarotonganCookIslandsMaori => "Rarotongan; Cook Islands Maori",
            Language::RomanceOther => "Romance (Other)",
            Language::Romansh => "Romansh",
            Language::Romany => "Romany",
            Language::Romanian => "Romanian",
            Language::Rundi => "Rundi",
            Language::AromanianArumanianMacedoRomanian => "Aromanian; Arumanian; Macedo-Romanian",
            Language::Russian => "Russian",
            Language::Sandawe => "Sandawe",
            Language::Sango => "Sango",
            Language::Yakut => "Yakut",
            Language::SouthAmericanIndianOther => "South American Indian (Other)",
            Language::SalishanLanguages => "Salishan languages",
            Language::SamaritanAramaic => "Samaritan Aramaic",
            Language::Sanskrit => "Sanskrit",
            Language::Sasak => "Sasak",
            Language::Santali => "Santali",
            Language::Serbian => "Serbian",
            Language::Sicilian => "Sicilian",
            Language::Scots => "Scots",
            Language::Croatian => "Croatian",
            Language::Selkup => "Selkup",
            Language::SemiticOther => "Semitic (Other)",
            Language::IrishOldTo900 => "Irish, Old (to 900)",
            Language::SignLanguages => "Sign Languages",
            Language::Shan => "Shan",
            Language::Sidamo => "Sidamo",
            Language::SinhalaSinhalese => "Sinhala; Sinhalese",
            Language::SiouanLanguages => "Siouan languages",
            Language::SinoTibetanOther => "Sino-Tibetan (Other)",
            Language::SlavicOther => "Slavic (Other)",
            Language::Slovak => "Slovak",
            Language::Slovenian => "Slovenian",
            Language::SouthernSami => "Southern Sami",
            Language::NorthernSami => "Northern Sami",
            Language::SamiLanguagesOther => "Sami languages (Other)",
            Language::LuleSami => "Lule Sami",
            Language::InariSami => "Inari Sami",
            Language::Samoan => "Samoan",
            Language::SkoltSami => "Skolt Sami",
            Language::Shona => "Shona",
            Language::Sindhi => "Sindhi",
            Language::Soninke => "Soninke",
            Language::Sogdian => "Sogdian",
            Language::Somali => "Somali",
            Language::SonghaiLanguages => "Songhai languages",
            Language::SothoSouthern => "Sotho, Southern",
            Language::SpanishCastilian => "Spanish; Castilian",
            Language::Sardinian => "Sardinian",
            Language::SrananTongo => "Sranan Tongo",
            Language::Serer => "Serer",
            Language::NiloSaharanOther => "Nilo-Saharan (Other)",
            Language::Swati => "Swati",
            Language::Sukuma => "Sukuma",
            Language::Sundanese => "Sundanese",
            Language::Susu => "Susu",
            Language::Sumerian => "Sumerian",
            Language::Swahili => "Swahili",
            Language::Swedish => "Swedish",
            Language::Syriac => "Syriac",
            Language::Tahitian => "Tahitian",
            Language::TaiOther => "Tai (Other)",
            Language::Tamil => "Tamil",
            Language::Tatar => "Tatar",
            Language::Telugu => "Telugu",
            Language::Timne => "Timne",
            Language::Tereno => "Tereno",
            Language::Tetum => "Tetum",
            Language::Tajik => "Tajik",
            Language::Tagalog => "Tagalog",
            Language::Thai => "Thai",
            Language::Tibetan => "Tibetan",
            Language::Tigre => "Tigre",
            Language::Tigrinya => "Tigrinya",
            Language::Tiv => "Tiv",
            Language::Tokelau => "Tokelau",
            Language::KlingonTlhinganHol => "Klingon; tlhIngan-Hol",
            Language::Tlingit => "Tlingit",
            Language::Tamashek => "Tamashek",
            Language::TongaNyasa => "Tonga (Nyasa)",
            Language::TongaTongaIslands => "Tonga (Tonga Islands)",
            Language::TokPisin => "Tok Pisin",
            Language::Tsimshian => "Tsimshian",
            Language::Tswana => "Tswana",
            Language::Tsonga => "Tsonga",
            Language::Turkmen => "Turkmen",
            Language::Tumbuka => "Tumbuka",
            Language::TupiLanguages => "Tupi languages",
            Language::Turkish => "Turkish",
            Language::AltaicOther => "Altaic (Other)",
            Language::Tuvalu => "Tuvalu",
            Language::Twi => "Twi",
            Language::Tuvinian => "Tuvinian",
            Language::Udmurt => "Udmurt",
            Language::Ugaritic => "Ugaritic",
            Language::UighurUyghur => "Uighur; Uyghur",
            Language::Ukrainian => "Ukrainian",
            Language::Umbundu => "Umbundu",
            Language::Undetermined => "Undetermined",
            Language::Urdu => "Urdu",
            Language::Uzbek => "Uzbek",
            Language::Vai => "Vai",
            Language::Venda => "Venda",
            Language::Vietnamese => "Vietnamese",
            Language::Volapük => "Volapük",
            Language::Votic => "Votic",
            Language::WakashanLanguages => "Wakashan languages",
            Language::Walamo => "Walamo",
            Language::Waray => "Waray",
            Language::Washo => "Washo",
            Language::Welsh => "Welsh",
            Language::SorbianLanguages => "Sorbian languages",
            Language::Walloon => "Walloon",
            Language::Wolof => "Wolof",
            Language::KalmykOirat => "Kalmyk; Oirat",
            Language::Xhosa => "Xhosa",
            Language::Yao => "Yao",
            Language::Yapese => "Yapese",
            Language::Yiddish => "Yiddish",
            Language::Yoruba => "Yoruba",
            Language::YupikLanguages => "Yupik languages",
            Language::Zapotec => "Zapotec",
            Language::Zenaga => "Zenaga",
            Language::ZhuangChuang => "Zhuang; Chuang",
            Language::ZandeLanguages => "Zande languages",
            Language::Zulu => "Zulu",
            Language::Zuni => "Zuni",
            Language::NoLinguisticContent => "No linguistic content",
            Language::ZazaDimiliDimliKirdkiKirmanjkiZazaki => {
                "Zaza; Dimili; Dimli; Kirdki; Kirmanjki; Zazaki."
            }
        }
    }
}

impl crate::FromCode for Language {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "aar" => Some(Language::Afar),
            "abk" => Some(Language::Abkhazian),
            "ace" => Some(Language::Achinese),
            "ach" => Some(Language::Acoli),
            "ada" => Some(Language::Adangme),
            "ady" => Some(Language::AdygheAdygei),
            "afa" => Some(Language::AfroAsiaticOther),
            "afh" => Some(Language::Afrihili),
            "afr" => Some(Language::Afrikaans),
            "ain" => Some(Language::Ainu),
            "aka" => Some(Language::Akan),
            "akk" => Some(Language::Akkadian),
            "alb" => Some(Language::Albanian),
            "ale" => Some(Language::Aleut),
            "alg" => Some(Language::AlgonquianLanguages),
            "alt" => Some(Language::SouthernAltai),
            "amh" => Some(Language::Amharic),
            "ang" => Some(Language::EnglishOldCa4501100),
            "anp" => Some(Language::Angika),
            "apa" => Some(Language::ApacheLanguages),
            "ara" => Some(Language::Arabic),
            "arc" => Some(Language::Aramaic),
            "arg" => Some(Language::Aragonese),
            "arm" => Some(Language::Armenian),
            "arn" => Some(Language::MapudungunMapuche),
            "arp" => Some(Language::Arapaho),
            "art" => Some(Language::ArtificialOther),
            "arw" => Some(Language::Arawak),
            "asm" => Some(Language::Assamese),
            "ast" => Some(Language::AsturianBable),
            "ath" => Some(Language::AthapascanLanguages),
            "aus" => Some(Language::AustralianLanguages),
            "ava" => Some(Language::Avaric),
            "ave" => Some(Language::Avestan),
            "awa" => Some(Language::Awadhi),
            "aym" => Some(Language::Aymara),
            "aze" => Some(Language::Azerbaijani),
            "bad" => Some(Language::BandaLanguages),
            "bai" => Some(Language::BamilekeLanguages),
            "bak" => Some(Language::Bashkir),
            "bal" => Some(Language::Baluchi),
            "bam" => Some(Language::Bambara),
            "ban" => Some(Language::Balinese),
            "baq" => Some(Language::Basque),
            "bas" => Some(Language::Basa),
            "bat" => Some(Language::BalticOther),
            "bej" => Some(Language::Beja),
            "bel" => Some(Language::Belarusian),
            "bem" => Some(Language::Bemba),
            "ben" => Some(Language::Bengali),
            "ber" => Some(Language::BerberOther),
            "bho" => Some(Language::Bhojpuri),
            "bih" => Some(Language::Bihari),
            "bik" => Some(Language::Bikol),
            "bin" => Some(Language::BiniEdo),
            "bis" => Some(Language::Bislama),
            "bla" => Some(Language::Siksika),
            "bnt" => Some(Language::BantuOther),
            "bos" => Some(Language::Bosnian),
            "bra" => Some(Language::Braj),
            "bre" => Some(Language::Breton),
            "btk" => Some(Language::BatakLanguages),
            "bua" => Some(Language::Buriat),
            "bug" => Some(Language::Buginese),
            "bul" => Some(Language::Bulgarian),
            "bur" => Some(Language::Burmese),
            "byn" => Some(Language::BlinBilin),
            "cad" => Some(Language::Caddo),
            "cai" => Some(Language::CentralAmericanIndianOther),
            "car" => Some(Language::GalibiCarib),
            "cat" => Some(Language::CatalanValencian),
            "cau" => Some(Language::CaucasianOther),
            "ceb" => Some(Language::Cebuano),
            "cel" => Some(Language::CelticOther),
            "cha" => Some(Language::Chamorro),
            "chb" => Some(Language::Chibcha),
            "che" => Some(Language::Chechen),
            "chg" => Some(Language::Chagatai),
            "chi" => Some(Language::Chinese),
            "chk" => Some(Language::Chuukese),
            "chm" => Some(Language::Mari),
            "chn" => Some(Language::ChinookJargon),
            "cho" => Some(Language::Choctaw),
            "chp" => Some(Language::Chipewyan),
            "chr" => Some(Language::Cherokee),
            "chu" => {
                Some(Language::ChurchSlavicOldSlavonicChurchSlavonicOldBulgarianOldChurchSlavonic)
            }
            "chv" => Some(Language::Chuvash),
            "chy" => Some(Language::Cheyenne),
            "cmc" => Some(Language::ChamicLanguages),
            "cop" => Some(Language::Coptic),
            "cor" => Some(Language::Cornish),
            "cos" => Some(Language::Corsican),
            "cpe" => Some(Language::CreolesAndPidginsEnglishBasedOther),
            "cpf" => Some(Language::CreolesAndPidginsFrenchBasedOther),
            "cpp" => Some(Language::CreolesAndPidginsPortugueseBasedOther),
            "cre" => Some(Language::Cree),
            "crh" => Some(Language::CrimeanTatarCrimeanTurkish),
            "crp" => Some(Language::CreolesAndPidginsOther),
            "csb" => Some(Language::Kashubian),
            "cus" => Some(Language::CushiticOther),
            "cze" => Some(Language::Czech),
            "dak" => Some(Language::Dakota),
            "dan" => Some(Language::Danish),
            "dar" => Some(Language::Dargwa),
            "day" => Some(Language::LandDayakLanguages),
            "del" => Some(Language::Delaware),
            "den" => Some(Language::SlaveAthapascan),
            "dgr" => Some(Language::Dogrib),
            "din" => Some(Language::Dinka),
            "div" => Some(Language::DivehiDhivehiMaldivian),
            "doi" => Some(Language::Dogri),
            "dra" => Some(Language::DravidianOther),
            "dsb" => Some(Language::LowerSorbian),
            "dua" => Some(Language::Duala),
            "dum" => Some(Language::DutchMiddleCa10501350),
            "dut" => Some(Language::DutchFlemish),
            "dyu" => Some(Language::Dyula),
            "dzo" => Some(Language::Dzongkha),
            "efi" => Some(Language::Efik),
            "egy" => Some(Language::EgyptianAncient),
            "eka" => Some(Language::Ekajuk),
            "elx" => Some(Language::Elamite),
            "eng" => Some(Language::English),
            "enm" => Some(Language::EnglishMiddle11001500),
            "epo" => Some(Language::Esperanto),
            "est" => Some(Language::Estonian),
            "ewe" => Some(Language::Ewe),
            "ewo" => Some(Language::Ewondo),
            "fan" => Some(Language::Fang),
            "fao" => Some(Language::Faroese),
            "fat" => Some(Language::Fanti),
            "fij" => Some(Language::Fijian),
            "fil" => Some(Language::FilipinoPilipino),
            "fin" => Some(Language::Finnish),
            "fiu" => Some(Language::FinnoUgrianOther),
            "fon" => Some(Language::Fon),
            "fre" => Some(Language::French),
            "frm" => Some(Language::FrenchMiddleCa14001600),
            "fro" => Some(Language::FrenchOld842Ca1400),
            "frr" => Some(Language::NorthernFrisian),
            "frs" => Some(Language::EasternFrisian),
            "fry" => Some(Language::WesternFrisian),
            "ful" => Some(Language::Fulah),
            "fur" => Some(Language::Friulian),
            "gaa" => Some(Language::Ga),
            "gay" => Some(Language::Gayo),
            "gba" => Some(Language::Gbaya),
            "gem" => Some(Language::GermanicOther),
            "geo" => Some(Language::Georgian),
            "ger" => Some(Language::German),
            "gez" => Some(Language::Geez),
            "gil" => Some(Language::Gilbertese),
            "gla" => Some(Language::GaelicScottishGaelic),
            "gle" => Some(Language::Irish),
            "glg" => Some(Language::Galician),
            "glv" => Some(Language::Manx),
            "gmh" => Some(Language::GermanMiddleHighCa10501500),
            "goh" => Some(Language::GermanOldHighCa7501050),
            "gon" => Some(Language::Gondi),
            "gor" => Some(Language::Gorontalo),
            "got" => Some(Language::Gothic),
            "grb" => Some(Language::Grebo),
            "grc" => Some(Language::GreekAncientTo1453),
            "gre" => Some(Language::GreekModern1453),
            "grn" => Some(Language::Guarani),
            "gsw" => Some(Language::SwissGermanAlemannic),
            "guj" => Some(Language::Gujarati),
            "gwi" => Some(Language::Gwichin),
            "hai" => Some(Language::Haida),
            "hat" => Some(Language::HaitianHaitianCreole),
            "hau" => Some(Language::Hausa),
            "haw" => Some(Language::Hawaiian),
            "heb" => Some(Language::Hebrew),
            "her" => Some(Language::Herero),
            "hil" => Some(Language::Hiligaynon),
            "him" => Some(Language::Himachali),
            "hin" => Some(Language::Hindi),
            "hit" => Some(Language::Hittite),
            "hmn" => Some(Language::Hmong),
            "hmo" => Some(Language::HiriMotu),
            "hsb" => Some(Language::UpperSorbian),
            "hun" => Some(Language::Hungarian),
            "hup" => Some(Language::Hupa),
            "iba" => Some(Language::Iban),
            "ibo" => Some(Language::Igbo),
            "ice" => Some(Language::Icelandic),
            "ido" => Some(Language::Ido),
            "iii" => Some(Language::SichuanYi),
            "ijo" => Some(Language::IjoLanguages),
            "iku" => Some(Language::Inuktitut),
            "ile" => Some(Language::Interlingue),
            "ilo" => Some(Language::Iloko),
            "ina" => Some(Language::InterlinguaInternationalAuxiliaryLanguageAssociation),
            "inc" => Some(Language::IndicOther),
            "ind" => Some(Language::Indonesian),
            "ine" => Some(Language::IndoEuropeanOther),
            "inh" => Some(Language::Ingush),
            "ipk" => Some(Language::Inupiaq),
            "ira" => Some(Language::IranianOther),
            "iro" => Some(Language::IroquoianLanguages),
            "ita" => Some(Language::Italian),
            "jav" => Some(Language::Javanese),
            "jbo" => Some(Language::Lojban),
            "jpn" => Some(Language::Japanese),
            "jpr" => Some(Language::JudeoPersian),
            "jrb" => Some(Language::JudeoArabic),
            "kaa" => Some(Language::KaraKalpak),
            "kab" => Some(Language::Kabyle),
            "kac" => Some(Language::KachinJingpho),
            "kal" => Some(Language::KalaallisutGreenlandic),
            "kam" => Some(Language::Kamba),
            "kan" => Some(Language::Kannada),
            "kar" => Some(Language::KarenLanguages),
            "kas" => Some(Language::Kashmiri),
            "kau" => Some(Language::Kanuri),
            "kaw" => Some(Language::Kawi),
            "kaz" => Some(Language::Kazakh),
            "kbd" => Some(Language::Kabardian),
            "kha" => Some(Language::Khasi),
            "khi" => Some(Language::KhoisanOther),
            "khm" => Some(Language::CentralKhmer),
            "kho" => Some(Language::Khotanese),
            "kik" => Some(Language::KikuyuGikuyu),
            "kin" => Some(Language::Kinyarwanda),
            "kir" => Some(Language::KirghizKyrgyz),
            "kmb" => Some(Language::Kimbundu),
            "kok" => Some(Language::Konkani),
            "kom" => Some(Language::Komi),
            "kon" => Some(Language::Kongo),
            "kor" => Some(Language::Korean),
            "kos" => Some(Language::Kosraean),
            "kpe" => Some(Language::Kpelle),
            "krc" => Some(Language::KarachayBalkar),
            "krl" => Some(Language::Karelian),
            "kro" => Some(Language::KruLanguages),
            "kru" => Some(Language::Kurukh),
            "kua" => Some(Language::KuanyamaKwanyama),
            "kum" => Some(Language::Kumyk),
            "kur" => Some(Language::Kurdish),
            "kut" => Some(Language::Kutenai),
            "lad" => Some(Language::Ladino),
            "lah" => Some(Language::Lahnda),
            "lam" => Some(Language::Lamba),
            "lao" => Some(Language::Lao),
            "lat" => Some(Language::Latin),
            "lav" => Some(Language::Latvian),
            "lez" => Some(Language::Lezghian),
            "lim" => Some(Language::LimburganLimburgerLimburgish),
            "lin" => Some(Language::Lingala),
            "lit" => Some(Language::Lithuanian),
            "lol" => Some(Language::Mongo),
            "loz" => Some(Language::Lozi),
            "ltz" => Some(Language::LuxembourgishLetzeburgesch),
            "lua" => Some(Language::LubaLulua),
            "lub" => Some(Language::LubaKatanga),
            "lug" => Some(Language::Ganda),
            "lui" => Some(Language::Luiseno),
            "lun" => Some(Language::Lunda),
            "luo" => Some(Language::LuoKenyaAndTanzania),
            "lus" => Some(Language::Lushai),
            "mac" => Some(Language::Macedonian),
            "mad" => Some(Language::Madurese),
            "mag" => Some(Language::Magahi),
            "mah" => Some(Language::Marshallese),
            "mai" => Some(Language::Maithili),
            "mak" => Some(Language::Makasar),
            "mal" => Some(Language::Malayalam),
            "man" => Some(Language::Mandingo),
            "mao" => Some(Language::Maori),
            "map" => Some(Language::AustronesianOther),
            "mar" => Some(Language::Marathi),
            "mas" => Some(Language::Masai),
            "may" => Some(Language::Malay),
            "mdf" => Some(Language::Moksha),
            "mdr" => Some(Language::Mandar),
            "men" => Some(Language::Mende),
            "mga" => Some(Language::IrishMiddle9001200),
            "mic" => Some(Language::MikmaqMicmac),
            "min" => Some(Language::Minangkabau),
            "mis" => Some(Language::MiscellaneousLanguages),
            "mkh" => Some(Language::MonKhmerOther),
            "mlg" => Some(Language::Malagasy),
            "mlt" => Some(Language::Maltese),
            "mnc" => Some(Language::Manchu),
            "mni" => Some(Language::Manipuri),
            "mno" => Some(Language::ManoboLanguages),
            "moh" => Some(Language::Mohawk),
            "mol" => Some(Language::Moldavian),
            "mon" => Some(Language::Mongolian),
            "mos" => Some(Language::Mossi),
            "mul" => Some(Language::MultipleLanguages),
            "mun" => Some(Language::MundaLanguages),
            "mus" => Some(Language::Creek),
            "mwl" => Some(Language::Mirandese),
            "mwr" => Some(Language::Marwari),
            "myn" => Some(Language::MayanLanguages),
            "myv" => Some(Language::Erzya),
            "nah" => Some(Language::NahuatlLanguages),
            "nai" => Some(Language::NorthAmericanIndian),
            "nap" => Some(Language::Neapolitan),
            "nau" => Some(Language::Nauru),
            "nav" => Some(Language::NavajoNavaho),
            "nbl" => Some(Language::NdebeleSouthSouthNdebele),
            "nde" => Some(Language::NdebeleNorthNorthNdebele),
            "ndo" => Some(Language::Ndonga),
            "nds" => Some(Language::LowGermanLowSaxonGermanLowSaxonLow),
            "nep" => Some(Language::Nepali),
            "new" => Some(Language::NepalBhasaNewari),
            "nia" => Some(Language::Nias),
            "nic" => Some(Language::NigerKordofanianOther),
            "niu" => Some(Language::Niuean),
            "nno" => Some(Language::NorwegianNynorskNynorskNorwegian),
            "nob" => Some(Language::BokmålNorwegianNorwegianBokmål),
            "nog" => Some(Language::Nogai),
            "non" => Some(Language::NorseOld),
            "nor" => Some(Language::Norwegian),
            "nqo" => Some(Language::Nko),
            "nso" => Some(Language::PediSepediNorthernSotho),
            "nub" => Some(Language::NubianLanguages),
            "nwc" => Some(Language::ClassicalNewariOldNewariClassicalNepalBhasa),
            "nya" => Some(Language::ChichewaChewaNyanja),
            "nym" => Some(Language::Nyamwezi),
            "nyn" => Some(Language::Nyankole),
            "nyo" => Some(Language::Nyoro),
            "nzi" => Some(Language::Nzima),
            "oci" => Some(Language::OccitanPost1500Provençal),
            "oji" => Some(Language::Ojibwa),
            "ori" => Some(Language::Oriya),
            "orm" => Some(Language::Oromo),
            "osa" => Some(Language::Osage),
            "oss" => Some(Language::OssetianOssetic),
            "ota" => Some(Language::TurkishOttoman15001928),
            "oto" => Some(Language::OtomianLanguages),
            "paa" => Some(Language::PapuanOther),
            "pag" => Some(Language::Pangasinan),
            "pal" => Some(Language::Pahlavi),
            "pam" => Some(Language::Pampanga),
            "pan" => Some(Language::PanjabiPunjabi),
            "pap" => Some(Language::Papiamento),
            "pau" => Some(Language::Palauan),
            "peo" => Some(Language::PersianOldCa600400BC),
            "per" => Some(Language::Persian),
            "phi" => Some(Language::PhilippineOther),
            "phn" => Some(Language::Phoenician),
            "pli" => Some(Language::Pali),
            "pol" => Some(Language::Polish),
            "pon" => Some(Language::Pohnpeian),
            "por" => Some(Language::Portuguese),
            "pra" => Some(Language::PrakritLanguages),
            "pro" => Some(Language::ProvençalOldTo1500),
            "pus" => Some(Language::Pushto),
            "que" => Some(Language::Quechua),
            "raj" => Some(Language::Rajasthani),
            "rap" => Some(Language::Rapanui),
            "rar" => Some(Language::RarotonganCookIslandsMaori),
            "roa" => Some(Language::RomanceOther),
            "roh" => Some(Language::Romansh),
            "rom" => Some(Language::Romany),
            "rum" => Some(Language::Romanian),
            "run" => Some(Language::Rundi),
            "rup" => Some(Language::AromanianArumanianMacedoRomanian),
            "rus" => Some(Language::Russian),
            "sad" => Some(Language::Sandawe),
            "sag" => Some(Language::Sango),
            "sah" => Some(Language::Yakut),
            "sai" => Some(Language::SouthAmericanIndianOther),
            "sal" => Some(Language::SalishanLanguages),
            "sam" => Some(Language::SamaritanAramaic),
            "san" => Some(Language::Sanskrit),
            "sas" => Some(Language::Sasak),
            "sat" => Some(Language::Santali),
            "scc" => Some(Language::Serbian),
            "scn" => Some(Language::Sicilian),
            "sco" => Some(Language::Scots),
            "scr" => Some(Language::Croatian),
            "sel" => Some(Language::Selkup),
            "sem" => Some(Language::SemiticOther),
            "sga" => Some(Language::IrishOldTo900),
            "sgn" => Some(Language::SignLanguages),
            "shn" => Some(Language::Shan),
            "sid" => Some(Language::Sidamo),
            "sin" => Some(Language::SinhalaSinhalese),
            "sio" => Some(Language::SiouanLanguages),
            "sit" => Some(Language::SinoTibetanOther),
            "sla" => Some(Language::SlavicOther),
            "slo" => Some(Language::Slovak),
            "slv" => Some(Language::Slovenian),
            "sma" => Some(Language::SouthernSami),
            "sme" => Some(Language::NorthernSami),
            "smi" => Some(Language::SamiLanguagesOther),
            "smj" => Some(Language::LuleSami),
            "smn" => Some(Language::InariSami),
            "smo" => Some(Language::Samoan),
            "sms" => Some(Language::SkoltSami),
            "sna" => Some(Language::Shona),
            "snd" => Some(Language::Sindhi),
            "snk" => Some(Language::Soninke),
            "sog" => Some(Language::Sogdian),
            "som" => Some(Language::Somali),
            "son" => Some(Language::SonghaiLanguages),
            "sot" => Some(Language::SothoSouthern),
            "spa" => Some(Language::SpanishCastilian),
            "srd" => Some(Language::Sardinian),
            "srn" => Some(Language::SrananTongo),
            "srr" => Some(Language::Serer),
            "ssa" => Some(Language::NiloSaharanOther),
            "ssw" => Some(Language::Swati),
            "suk" => Some(Language::Sukuma),
            "sun" => Some(Language::Sundanese),
            "sus" => Some(Language::Susu),
            "sux" => Some(Language::Sumerian),
            "swa" => Some(Language::Swahili),
            "swe" => Some(Language::Swedish),
            "syr" => Some(Language::Syriac),
            "tah" => Some(Language::Tahitian),
            "tai" => Some(Language::TaiOther),
            "tam" => Some(Language::Tamil),
            "tat" => Some(Language::Tatar),
            "tel" => Some(Language::Telugu),
            "tem" => Some(Language::Timne),
            "ter" => Some(Language::Tereno),
            "tet" => Some(Language::Tetum),
            "tgk" => Some(Language::Tajik),
            "tgl" => Some(Language::Tagalog),
            "tha" => Some(Language::Thai),
            "tib" => Some(Language::Tibetan),
            "tig" => Some(Language::Tigre),
            "tir" => Some(Language::Tigrinya),
            "tiv" => Some(Language::Tiv),
            "tkl" => Some(Language::Tokelau),
            "tlh" => Some(Language::KlingonTlhinganHol),
            "tli" => Some(Language::Tlingit),
            "tmh" => Some(Language::Tamashek),
            "tog" => Some(Language::TongaNyasa),
            "ton" => Some(Language::TongaTongaIslands),
            "tpi" => Some(Language::TokPisin),
            "tsi" => Some(Language::Tsimshian),
            "tsn" => Some(Language::Tswana),
            "tso" => Some(Language::Tsonga),
            "tuk" => Some(Language::Turkmen),
            "tum" => Some(Language::Tumbuka),
            "tup" => Some(Language::TupiLanguages),
            "tur" => Some(Language::Turkish),
            "tut" => Some(Language::AltaicOther),
            "tvl" => Some(Language::Tuvalu),
            "twi" => Some(Language::Twi),
            "tyv" => Some(Language::Tuvinian),
            "udm" => Some(Language::Udmurt),
            "uga" => Some(Language::Ugaritic),
            "uig" => Some(Language::UighurUyghur),
            "ukr" => Some(Language::Ukrainian),
            "umb" => Some(Language::Umbundu),
            "und" => Some(Language::Undetermined),
            "urd" => Some(Language::Urdu),
            "uzb" => Some(Language::Uzbek),
            "vai" => Some(Language::Vai),
            "ven" => Some(Language::Venda),
            "vie" => Some(Language::Vietnamese),
            "vol" => Some(Language::Volapük),
            "vot" => Some(Language::Votic),
            "wak" => Some(Language::WakashanLanguages),
            "wal" => Some(Language::Walamo),
            "war" => Some(Language::Waray),
            "was" => Some(Language::Washo),
            "wel" => Some(Language::Welsh),
            "wen" => Some(Language::SorbianLanguages),
            "wln" => Some(Language::Walloon),
            "wol" => Some(Language::Wolof),
            "xal" => Some(Language::KalmykOirat),
            "xho" => Some(Language::Xhosa),
            "yao" => Some(Language::Yao),
            "yap" => Some(Language::Yapese),
            "yid" => Some(Language::Yiddish),
            "yor" => Some(Language::Yoruba),
            "ypk" => Some(Language::YupikLanguages),
            "zap" => Some(Language::Zapotec),
            "zen" => Some(Language::Zenaga),
            "zha" => Some(Language::ZhuangChuang),
            "znd" => Some(Language::ZandeLanguages),
            "zul" => Some(Language::Zulu),
            "zun" => Some(Language::Zuni),
            "zxx" => Some(Language::NoLinguisticContent),
            "zza" => Some(Language::ZazaDimiliDimliKirdkiKirmanjkiZazaki),
            _ => None,
        }
    }
}
