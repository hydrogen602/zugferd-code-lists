#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Unit {
    /// group
    ///
    /// rec20
    Group,
    /// outfit
    ///
    /// rec20
    Outfit,
    /// ration
    ///
    /// rec20
    Ration,
    /// shot
    ///
    /// rec20
    Shot,
    /// stick, military
    ///
    /// rec20
    StickMilitary,
    /// twenty foot container
    ///
    /// rec20
    TwentyFootContainer,
    /// forty foot container
    ///
    /// rec20
    FortyFootContainer,
    /// decilitre per gram
    ///
    /// rec20
    DecilitrePerGram,
    /// gram per cubic centimetre
    ///
    /// rec20
    GramPerCubicCentimetre,
    /// theoretical pound
    ///
    /// rec20
    TheoreticalPound,
    /// gram per square centimetre
    ///
    /// rec20
    GramPerSquareCentimetre,
    /// theoretical ton
    ///
    /// rec20
    TheoreticalTon,
    /// kilogram per square metre
    ///
    /// rec20
    KilogramPerSquareMetre,
    /// kilopascal square metre per gram
    ///
    /// rec20
    KilopascalSquareMetrePerGram,
    /// kilopascal per millimetre
    ///
    /// rec20
    KilopascalPerMillimetre,
    /// millilitre per square centimetre second
    ///
    /// rec20
    MillilitrePerSquareCentimetreSecond,
    /// ounce per square foot
    ///
    /// rec20
    OuncePerSquareFoot,
    /// ounce per square foot per 0,01inch
    ///
    /// rec20
    OuncePerSquareFootPer001inch,
    /// millilitre per second
    ///
    /// rec20
    MillilitrePerSecond,
    /// millilitre per minute
    ///
    /// rec20
    MillilitrePerMinute,
    /// sitas
    ///
    /// rec20
    Sitas,
    /// mesh
    ///
    /// rec20
    Mesh,
    /// net kilogram
    ///
    /// rec20
    NetKilogram,
    /// part per million
    ///
    /// rec20
    PartPerMillion,
    /// percent weight
    ///
    /// rec20
    PercentWeight,
    /// part per billion (US)
    ///
    /// rec20
    PartPerBillionUs,
    /// millipascal
    ///
    /// rec20
    Millipascal,
    /// milli-inch
    ///
    /// rec20
    MilliInch,
    /// pound per square inch absolute
    ///
    /// rec20
    PoundPerSquareInchAbsolute,
    /// henry
    ///
    /// rec20
    Henry,
    /// foot pound-force
    ///
    /// rec20
    FootPoundForce,
    /// pound per cubic foot
    ///
    /// rec20
    PoundPerCubicFoot,
    /// poise
    ///
    /// rec20
    Poise,
    /// stokes
    ///
    /// rec20
    Stokes,
    /// fixed rate
    ///
    /// rec20
    FixedRate,
    /// radian per second
    ///
    /// rec20
    RadianPerSecond,
    /// radian per second squared
    ///
    /// rec20
    RadianPerSecondSquared,
    /// roentgen
    ///
    /// rec20
    Roentgen,
    /// volt AC
    ///
    /// rec20
    VoltAc,
    /// volt DC
    ///
    /// rec20
    VoltDc,
    /// British thermal unit (international table) per hour
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerHour,
    /// cubic centimetre per second
    ///
    /// rec20
    CubicCentimetrePerSecond,
    /// cubic foot per hour
    ///
    /// rec20
    CubicFootPerHour,
    /// cubic foot per minute
    ///
    /// rec20
    CubicFootPerMinute,
    /// centimetre per second
    ///
    /// rec20
    CentimetrePerSecond,
    /// decibel
    ///
    /// rec20
    Decibel,
    /// kilobyte
    ///
    /// rec20
    Kilobyte,
    /// kilobecquerel
    ///
    /// rec20
    Kilobecquerel,
    /// kilocurie
    ///
    /// rec20
    Kilocurie,
    /// megagram
    ///
    /// rec20
    Megagram,
    /// metre per minute
    ///
    /// rec20
    MetrePerMinute,
    /// milliroentgen
    ///
    /// rec20
    Milliroentgen,
    /// millivolt
    ///
    /// rec20
    Millivolt,
    /// megajoule
    ///
    /// rec20
    Megajoule,
    /// manmonth
    ///
    /// rec20
    Manmonth,
    /// centistokes
    ///
    /// rec20
    Centistokes,
    /// microlitre
    ///
    /// rec20
    Microlitre,
    /// micrometre (micron)
    ///
    /// rec20
    MicrometreMicron,
    /// milliampere
    ///
    /// rec20
    Milliampere,
    /// megabyte
    ///
    /// rec20
    Megabyte,
    /// milligram per hour
    ///
    /// rec20
    MilligramPerHour,
    /// megabecquerel
    ///
    /// rec20
    Megabecquerel,
    /// microfarad
    ///
    /// rec20
    Microfarad,
    /// newton per metre
    ///
    /// rec20
    NewtonPerMetre,
    /// ounce inch
    ///
    /// rec20
    OunceInch,
    /// ounce foot
    ///
    /// rec20
    OunceFoot,
    /// picofarad
    ///
    /// rec20
    Picofarad,
    /// pound per hour
    ///
    /// rec20
    PoundPerHour,
    /// ton (US) per hour
    ///
    /// rec20
    TonUsPerHour,
    /// kilolitre per hour
    ///
    /// rec20
    KilolitrePerHour,
    /// barrel (US) per minute
    ///
    /// rec20
    BarrelUsPerMinute,
    /// batch
    ///
    /// rec20
    Batch,
    /// MMSCF/day
    ///
    /// rec20
    MmscfDay,
    /// hydraulic horse power
    ///
    /// rec20
    HydraulicHorsePower,
    /// ampere square metre per joule second
    ///
    /// rec20
    AmpereSquareMetrePerJouleSecond,
    /// angstrom
    ///
    /// rec20
    Angstrom,
    /// astronomical unit
    ///
    /// rec20
    AstronomicalUnit,
    /// attojoule
    ///
    /// rec20
    Attojoule,
    /// barn
    ///
    /// rec20
    Barn,
    /// barn per electronvolt
    ///
    /// rec20
    BarnPerElectronvolt,
    /// barn per steradian electronvolt
    ///
    /// rec20
    BarnPerSteradianElectronvolt,
    /// barn per steradian
    ///
    /// rec20
    BarnPerSteradian,
    /// becquerel per kilogram
    ///
    /// rec20
    BecquerelPerKilogram,
    /// becquerel per cubic metre
    ///
    /// rec20
    BecquerelPerCubicMetre,
    /// ampere per centimetre
    ///
    /// rec20
    AmperePerCentimetre,
    /// British thermal unit (international table) per second square foot degree Rankine
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerSecondSquareFootDegreeRankine,
    /// British thermal unit (international table) per pound degree Rankine
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerPoundDegreeRankine,
    /// British thermal unit (international table) per second foot degree Rankine
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerSecondFootDegreeRankine,
    /// British thermal unit (international table) per hour square foot degree Rankine
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerHourSquareFootDegreeRankine,
    /// candela per square metre
    ///
    /// rec20
    CandelaPerSquareMetre,
    /// coulomb metre
    ///
    /// rec20
    CoulombMetre,
    /// coulomb metre squared per volt
    ///
    /// rec20
    CoulombMetreSquaredPerVolt,
    /// coulomb per cubic centimetre
    ///
    /// rec20
    CoulombPerCubicCentimetre,
    /// coulomb per cubic metre
    ///
    /// rec20
    CoulombPerCubicMetre,
    /// ampere per millimetre
    ///
    /// rec20
    AmperePerMillimetre,
    /// coulomb per cubic millimetre
    ///
    /// rec20
    CoulombPerCubicMillimetre,
    /// coulomb per kilogram second
    ///
    /// rec20
    CoulombPerKilogramSecond,
    /// coulomb per mole
    ///
    /// rec20
    CoulombPerMole,
    /// coulomb per square centimetre
    ///
    /// rec20
    CoulombPerSquareCentimetre,
    /// coulomb per square metre
    ///
    /// rec20
    CoulombPerSquareMetre,
    /// coulomb per square millimetre
    ///
    /// rec20
    CoulombPerSquareMillimetre,
    /// cubic centimetre per mole
    ///
    /// rec20
    CubicCentimetrePerMole,
    /// cubic decimetre per mole
    ///
    /// rec20
    CubicDecimetrePerMole,
    /// cubic metre per coulomb
    ///
    /// rec20
    CubicMetrePerCoulomb,
    /// cubic metre per kilogram
    ///
    /// rec20
    CubicMetrePerKilogram,
    /// ampere per square centimetre
    ///
    /// rec20
    AmperePerSquareCentimetre,
    /// cubic metre per mole
    ///
    /// rec20
    CubicMetrePerMole,
    /// ampere per square metre
    ///
    /// rec20
    AmperePerSquareMetre,
    /// curie per kilogram
    ///
    /// rec20
    CuriePerKilogram,
    /// deadweight tonnage
    ///
    /// rec20
    DeadweightTonnage,
    /// decalitre
    ///
    /// rec20
    Decalitre,
    /// decametre
    ///
    /// rec20
    Decametre,
    /// decitex
    ///
    /// rec20
    Decitex,
    /// degree Rankine
    ///
    /// rec20
    DegreeRankine,
    /// denier
    ///
    /// rec20
    Denier,
    /// ampere square metre
    ///
    /// rec20
    AmpereSquareMetre,
    /// electronvolt
    ///
    /// rec20
    Electronvolt,
    /// electronvolt per metre
    ///
    /// rec20
    ElectronvoltPerMetre,
    /// electronvolt square metre
    ///
    /// rec20
    ElectronvoltSquareMetre,
    /// electronvolt square metre per kilogram
    ///
    /// rec20
    ElectronvoltSquareMetrePerKilogram,
    /// 8-part cloud cover
    ///
    /// rec20
    _8PartCloudCover,
    /// ampere per square metre kelvin squared
    ///
    /// rec20
    AmperePerSquareMetreKelvinSquared,
    /// exajoule
    ///
    /// rec20
    Exajoule,
    /// farad per metre
    ///
    /// rec20
    FaradPerMetre,
    /// ampere per square millimetre
    ///
    /// rec20
    AmperePerSquareMillimetre,
    /// femtojoule
    ///
    /// rec20
    Femtojoule,
    /// femtometre
    ///
    /// rec20
    Femtometre,
    /// foot per second squared
    ///
    /// rec20
    FootPerSecondSquared,
    /// foot pound-force per second
    ///
    /// rec20
    FootPoundForcePerSecond,
    /// freight ton
    ///
    /// rec20
    FreightTon,
    /// gal
    ///
    /// rec20
    Gal,
    /// ampere second
    ///
    /// rec20
    AmpereSecond,
    /// gigacoulomb per cubic metre
    ///
    /// rec20
    GigacoulombPerCubicMetre,
    /// gigaelectronvolt
    ///
    /// rec20
    Gigaelectronvolt,
    /// gigahertz
    ///
    /// rec20
    Gigahertz,
    /// gigaohm
    ///
    /// rec20
    Gigaohm,
    /// gigaohm metre
    ///
    /// rec20
    GigaohmMetre,
    /// gigapascal
    ///
    /// rec20
    Gigapascal,
    /// rate
    ///
    /// rec20
    Rate,
    /// gigawatt
    ///
    /// rec20
    Gigawatt,
    /// gon
    ///
    /// rec20
    Gon,
    /// gram per cubic metre
    ///
    /// rec20
    GramPerCubicMetre,
    /// gram per mole
    ///
    /// rec20
    GramPerMole,
    /// gray
    ///
    /// rec20
    Gray,
    /// gray per second
    ///
    /// rec20
    GrayPerSecond,
    /// hectopascal
    ///
    /// rec20
    Hectopascal,
    /// henry per metre
    ///
    /// rec20
    HenryPerMetre,
    /// bit
    ///
    /// rec20
    Bit,
    /// ball
    ///
    /// rec20
    Ball,
    /// bulk pack
    ///
    /// rec20
    BulkPack,
    /// acre
    ///
    /// rec20
    Acre,
    /// activity
    ///
    /// rec20
    Activity,
    /// byte
    ///
    /// rec20
    Byte,
    /// ampere per metre
    ///
    /// rec20
    AmperePerMetre,
    /// additional minute
    ///
    /// rec20
    AdditionalMinute,
    /// average minute per call
    ///
    /// rec20
    AverageMinutePerCall,
    /// fathom
    ///
    /// rec20
    Fathom,
    /// access line
    ///
    /// rec20
    AccessLine,
    /// ampere hour
    ///
    /// rec20
    AmpereHour,
    /// ampere
    ///
    /// rec20
    Ampere,
    /// year
    ///
    /// rec20
    Year,
    /// troy ounce or apothecary ounce
    ///
    /// rec20
    TroyOunceOrApothecaryOunce,
    /// anti-hemophilic factor (AHF) unit
    ///
    /// rec20
    AntiHemophilicFactorAhfUnit,
    /// assortment
    ///
    /// rec20
    Assortment,
    /// alcoholic strength by mass
    ///
    /// rec20
    AlcoholicStrengthByMass,
    /// alcoholic strength by volume
    ///
    /// rec20
    AlcoholicStrengthByVolume,
    /// standard atmosphere
    ///
    /// rec20
    StandardAtmosphere,
    /// american wire gauge
    ///
    /// rec20
    AmericanWireGauge,
    /// assembly
    ///
    /// rec20
    Assembly,
    /// British thermal unit (international table) per pound
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerPound,
    /// barrel (US) per day
    ///
    /// rec20
    BarrelUsPerDay,
    /// bit per second
    ///
    /// rec20
    BitPerSecond,
    /// joule per kilogram kelvin
    ///
    /// rec20
    JoulePerKilogramKelvin,
    /// joule per metre
    ///
    /// rec20
    JoulePerMetre,
    /// joule per square metre
    ///
    /// rec20
    JoulePerSquareMetre,
    /// joule per metre to the fourth power
    ///
    /// rec20
    JoulePerMetreToFourthPower,
    /// joule per mole
    ///
    /// rec20
    JoulePerMole,
    /// joule per mole kelvin
    ///
    /// rec20
    JoulePerMoleKelvin,
    /// credit
    ///
    /// rec20
    Credit,
    /// joule second
    ///
    /// rec20
    JouleSecond,
    /// digit
    ///
    /// rec20
    Digit,
    /// joule square metre per kilogram
    ///
    /// rec20
    JouleSquareMetrePerKilogram,
    /// kelvin per watt
    ///
    /// rec20
    KelvinPerWatt,
    /// kiloampere
    ///
    /// rec20
    Kiloampere,
    /// kiloampere per square metre
    ///
    /// rec20
    KiloamperePerSquareMetre,
    /// kiloampere per metre
    ///
    /// rec20
    KiloamperePerMetre,
    /// kilobecquerel per kilogram
    ///
    /// rec20
    KilobecquerelPerKilogram,
    /// kilocoulomb
    ///
    /// rec20
    Kilocoulomb,
    /// kilocoulomb per cubic metre
    ///
    /// rec20
    KilocoulombPerCubicMetre,
    /// kilocoulomb per square metre
    ///
    /// rec20
    KilocoulombPerSquareMetre,
    /// kiloelectronvolt
    ///
    /// rec20
    Kiloelectronvolt,
    /// batting pound
    ///
    /// rec20
    BattingPound,
    /// gibibit
    ///
    /// rec20
    Gibibit,
    /// kilogram metre per second
    ///
    /// rec20
    KilogramMetrePerSecond,
    /// kilogram metre squared
    ///
    /// rec20
    KilogramMetreSquared,
    /// kilogram metre squared per second
    ///
    /// rec20
    KilogramMetreSquaredPerSecond,
    /// kilogram per cubic decimetre
    ///
    /// rec20
    KilogramPerCubicDecimetre,
    /// kilogram per litre
    ///
    /// rec20
    KilogramPerLitre,
    /// barrel, imperial
    ///
    /// rec20
    BarrelImperial,
    /// kilojoule per kelvin
    ///
    /// rec20
    KilojoulePerKelvin,
    /// kilojoule per kilogram
    ///
    /// rec20
    KilojoulePerKilogram,
    /// kilojoule per kilogram kelvin
    ///
    /// rec20
    KilojoulePerKilogramKelvin,
    /// kilojoule per mole
    ///
    /// rec20
    KilojoulePerMole,
    /// kilomole
    ///
    /// rec20
    Kilomole,
    /// kilomole per cubic metre
    ///
    /// rec20
    KilomolePerCubicMetre,
    /// kilonewton
    ///
    /// rec20
    Kilonewton,
    /// kilonewton metre
    ///
    /// rec20
    KilonewtonMetre,
    /// kiloohm
    ///
    /// rec20
    Kiloohm,
    /// kiloohm metre
    ///
    /// rec20
    KiloohmMetre,
    /// kilosecond
    ///
    /// rec20
    Kilosecond,
    /// kilosiemens
    ///
    /// rec20
    Kilosiemens,
    /// kilosiemens per metre
    ///
    /// rec20
    KilosiemensPerMetre,
    /// kilovolt per metre
    ///
    /// rec20
    KilovoltPerMetre,
    /// kiloweber per metre
    ///
    /// rec20
    KiloweberPerMetre,
    /// light year
    ///
    /// rec20
    LightYear,
    /// litre per mole
    ///
    /// rec20
    LitrePerMole,
    /// lumen hour
    ///
    /// rec20
    LumenHour,
    /// lumen per square metre
    ///
    /// rec20
    LumenPerSquareMetre,
    /// lumen per watt
    ///
    /// rec20
    LumenPerWatt,
    /// lumen second
    ///
    /// rec20
    LumenSecond,
    /// lux hour
    ///
    /// rec20
    LuxHour,
    /// lux second
    ///
    /// rec20
    LuxSecond,
    /// megaampere per square metre
    ///
    /// rec20
    MegaamperePerSquareMetre,
    /// megabecquerel per kilogram
    ///
    /// rec20
    MegabecquerelPerKilogram,
    /// gigabit
    ///
    /// rec20
    Gigabit,
    /// megacoulomb per cubic metre
    ///
    /// rec20
    MegacoulombPerCubicMetre,
    /// cycle
    ///
    /// rec20
    Cycle,
    /// megacoulomb per square metre
    ///
    /// rec20
    MegacoulombPerSquareMetre,
    /// megaelectronvolt
    ///
    /// rec20
    Megaelectronvolt,
    /// megagram per cubic metre
    ///
    /// rec20
    MegagramPerCubicMetre,
    /// meganewton
    ///
    /// rec20
    Meganewton,
    /// meganewton metre
    ///
    /// rec20
    MeganewtonMetre,
    /// megaohm
    ///
    /// rec20
    Megaohm,
    /// megaohm metre
    ///
    /// rec20
    MegaohmMetre,
    /// megasiemens per metre
    ///
    /// rec20
    MegasiemensPerMetre,
    /// megavolt
    ///
    /// rec20
    Megavolt,
    /// megavolt per metre
    ///
    /// rec20
    MegavoltPerMetre,
    /// joule per cubic metre
    ///
    /// rec20
    JoulePerCubicMetre,
    /// gigabit per second
    ///
    /// rec20
    GigabitPerSecond,
    /// reciprocal metre squared reciprocal second
    ///
    /// rec20
    ReciprocalMetreSquaredReciprocalSecond,
    /// inch per linear foot
    ///
    /// rec20
    InchPerLinearFoot,
    /// metre to the fourth power
    ///
    /// rec20
    MetreToFourthPower,
    /// microampere
    ///
    /// rec20
    Microampere,
    /// microbar
    ///
    /// rec20
    Microbar,
    /// microcoulomb
    ///
    /// rec20
    Microcoulomb,
    /// microcoulomb per cubic metre
    ///
    /// rec20
    MicrocoulombPerCubicMetre,
    /// microcoulomb per square metre
    ///
    /// rec20
    MicrocoulombPerSquareMetre,
    /// microfarad per metre
    ///
    /// rec20
    MicrofaradPerMetre,
    /// microhenry
    ///
    /// rec20
    Microhenry,
    /// microhenry per metre
    ///
    /// rec20
    MicrohenryPerMetre,
    /// micronewton
    ///
    /// rec20
    Micronewton,
    /// micronewton metre
    ///
    /// rec20
    MicronewtonMetre,
    /// microohm
    ///
    /// rec20
    Microohm,
    /// microohm metre
    ///
    /// rec20
    MicroohmMetre,
    /// micropascal
    ///
    /// rec20
    Micropascal,
    /// microradian
    ///
    /// rec20
    Microradian,
    /// microsecond
    ///
    /// rec20
    Microsecond,
    /// microsiemens
    ///
    /// rec20
    Microsiemens,
    /// bar [unit of pressure]
    ///
    /// rec20
    BarUnitPressure,
    /// base box
    ///
    /// rec20
    BaseBox,
    /// board foot
    ///
    /// rec20
    BoardFoot,
    /// brake horse power
    ///
    /// rec20
    BrakeHorsePower,
    /// billion (EUR)
    ///
    /// rec20
    BillionEur,
    /// dry barrel (US)
    ///
    /// rec20
    DryBarrelUs,
    /// barrel (US)
    ///
    /// rec20
    BarrelUs,
    /// hundred board foot
    ///
    /// rec20
    HundredBoardFoot,
    /// beats per minute
    ///
    /// rec20
    BeatsPerMinute,
    /// becquerel
    ///
    /// rec20
    Becquerel,
    /// British thermal unit (international table)
    ///
    /// rec20
    BritishThermalUnitInternationalTable,
    /// bushel (US)
    ///
    /// rec20
    BushelUs,
    /// bushel (UK)
    ///
    /// rec20
    BushelUk,
    /// call
    ///
    /// rec20
    Call,
    /// millifarad
    ///
    /// rec20
    Millifarad,
    /// milligal
    ///
    /// rec20
    Milligal,
    /// milligram per metre
    ///
    /// rec20
    MilligramPerMetre,
    /// milligray
    ///
    /// rec20
    Milligray,
    /// millihenry
    ///
    /// rec20
    Millihenry,
    /// millijoule
    ///
    /// rec20
    Millijoule,
    /// millimetre per second
    ///
    /// rec20
    MillimetrePerSecond,
    /// millimetre squared per second
    ///
    /// rec20
    MillimetreSquaredPerSecond,
    /// millimole
    ///
    /// rec20
    Millimole,
    /// mole per kilogram
    ///
    /// rec20
    MolePerKilogram,
    /// millinewton
    ///
    /// rec20
    Millinewton,
    /// kibibit
    ///
    /// rec20
    Kibibit,
    /// millinewton per metre
    ///
    /// rec20
    MillinewtonPerMetre,
    /// milliohm metre
    ///
    /// rec20
    MilliohmMetre,
    /// millipascal second
    ///
    /// rec20
    MillipascalSecond,
    /// milliradian
    ///
    /// rec20
    Milliradian,
    /// millisecond
    ///
    /// rec20
    Millisecond,
    /// millisiemens
    ///
    /// rec20
    Millisiemens,
    /// millisievert
    ///
    /// rec20
    Millisievert,
    /// millitesla
    ///
    /// rec20
    Millitesla,
    /// microvolt per metre
    ///
    /// rec20
    MicrovoltPerMetre,
    /// millivolt per metre
    ///
    /// rec20
    MillivoltPerMetre,
    /// milliwatt
    ///
    /// rec20
    Milliwatt,
    /// milliwatt per square metre
    ///
    /// rec20
    MilliwattPerSquareMetre,
    /// milliweber
    ///
    /// rec20
    Milliweber,
    /// mole
    ///
    /// rec20
    Mole,
    /// mole per cubic decimetre
    ///
    /// rec20
    MolePerCubicDecimetre,
    /// mole per cubic metre
    ///
    /// rec20
    MolePerCubicMetre,
    /// kilobit
    ///
    /// rec20
    Kilobit,
    /// mole per litre
    ///
    /// rec20
    MolePerLitre,
    /// nanoampere
    ///
    /// rec20
    Nanoampere,
    /// nanocoulomb
    ///
    /// rec20
    Nanocoulomb,
    /// nanofarad
    ///
    /// rec20
    Nanofarad,
    /// nanofarad per metre
    ///
    /// rec20
    NanofaradPerMetre,
    /// nanohenry
    ///
    /// rec20
    Nanohenry,
    /// nanohenry per metre
    ///
    /// rec20
    NanohenryPerMetre,
    /// nanometre
    ///
    /// rec20
    Nanometre,
    /// nanoohm metre
    ///
    /// rec20
    NanoohmMetre,
    /// nanosecond
    ///
    /// rec20
    Nanosecond,
    /// nanotesla
    ///
    /// rec20
    Nanotesla,
    /// nanowatt
    ///
    /// rec20
    Nanowatt,
    /// neper
    ///
    /// rec20
    Neper,
    /// neper per second
    ///
    /// rec20
    NeperPerSecond,
    /// picometre
    ///
    /// rec20
    Picometre,
    /// newton metre second
    ///
    /// rec20
    NewtonMetreSecond,
    /// newton metre squared per kilogram squared
    ///
    /// rec20
    NewtonMetreSquaredPerKilogramSquared,
    /// newton per square metre
    ///
    /// rec20
    NewtonPerSquareMetre,
    /// newton per square millimetre
    ///
    /// rec20
    NewtonPerSquareMillimetre,
    /// newton second
    ///
    /// rec20
    NewtonSecond,
    /// newton second per metre
    ///
    /// rec20
    NewtonSecondPerMetre,
    /// octave
    ///
    /// rec20
    Octave,
    /// ohm centimetre
    ///
    /// rec20
    OhmCentimetre,
    /// ohm metre
    ///
    /// rec20
    OhmMetre,
    /// one
    ///
    /// rec20
    One,
    /// parsec
    ///
    /// rec20
    Parsec,
    /// pascal per kelvin
    ///
    /// rec20
    PascalPerKelvin,
    /// pascal second
    ///
    /// rec20
    PascalSecond,
    /// pascal second per cubic metre
    ///
    /// rec20
    PascalSecondPerCubicMetre,
    /// pascal second per metre
    ///
    /// rec20
    PascalSecondPerMetre,
    /// petajoule
    ///
    /// rec20
    Petajoule,
    /// phon
    ///
    /// rec20
    Phon,
    /// centipoise
    ///
    /// rec20
    Centipoise,
    /// picoampere
    ///
    /// rec20
    Picoampere,
    /// picocoulomb
    ///
    /// rec20
    Picocoulomb,
    /// picofarad per metre
    ///
    /// rec20
    PicofaradPerMetre,
    /// picohenry
    ///
    /// rec20
    Picohenry,
    /// kilobit per second
    ///
    /// rec20
    KilobitPerSecond,
    /// picowatt
    ///
    /// rec20
    Picowatt,
    /// picowatt per square metre
    ///
    /// rec20
    PicowattPerSquareMetre,
    /// pound-force
    ///
    /// rec20
    PoundForce,
    /// kilovolt ampere hour
    ///
    /// rec20
    KilovoltAmpereHour,
    /// millicoulomb per kilogram
    ///
    /// rec20
    MillicoulombPerKilogram,
    /// rad
    ///
    /// rec20
    Rad,
    /// radian
    ///
    /// rec20
    Radian,
    /// radian square metre per mole
    ///
    /// rec20
    RadianSquareMetrePerMole,
    /// radian square metre per kilogram
    ///
    /// rec20
    RadianSquareMetrePerKilogram,
    /// radian per metre
    ///
    /// rec20
    RadianPerMetre,
    /// reciprocal angstrom
    ///
    /// rec20
    ReciprocalAngstrom,
    /// reciprocal cubic metre
    ///
    /// rec20
    ReciprocalCubicMetre,
    /// reciprocal cubic metre per second
    ///
    /// rec20
    ReciprocalCubicMetrePerSecond,
    /// reciprocal electron volt per cubic metre
    ///
    /// rec20
    ReciprocalElectronVoltPerCubicMetre,
    /// reciprocal henry
    ///
    /// rec20
    ReciprocalHenry,
    /// coil group
    ///
    /// rec20
    CoilGroup,
    /// reciprocal joule per cubic metre
    ///
    /// rec20
    ReciprocalJoulePerCubicMetre,
    /// reciprocal kelvin or kelvin to the power minus one
    ///
    /// rec20
    ReciprocalKelvinOrKelvinToPowerMinusOne,
    /// reciprocal metre
    ///
    /// rec20
    ReciprocalMetre,
    /// reciprocal square metre
    ///
    /// rec20
    ReciprocalSquareMetre,
    /// reciprocal minute
    ///
    /// rec20
    ReciprocalMinute,
    /// reciprocal mole
    ///
    /// rec20
    ReciprocalMole,
    /// reciprocal pascal or pascal to the power minus one
    ///
    /// rec20
    ReciprocalPascalOrPascalToPowerMinusOne,
    /// reciprocal second
    ///
    /// rec20
    ReciprocalSecond,
    /// reciprocal second per metre squared
    ///
    /// rec20
    ReciprocalSecondPerMetreSquared,
    /// carrying capacity in metric ton
    ///
    /// rec20
    CarryingCapacityInMetricTon,
    /// candela
    ///
    /// rec20
    Candela,
    /// degree Celsius
    ///
    /// rec20
    DegreeCelsius,
    /// hundred
    ///
    /// rec20
    Hundred,
    /// card
    ///
    /// rec20
    Card,
    /// centigram
    ///
    /// rec20
    Centigram,
    /// coulomb per kilogram
    ///
    /// rec20
    CoulombPerKilogram,
    /// hundred leave
    ///
    /// rec20
    HundredLeave,
    /// centilitre
    ///
    /// rec20
    Centilitre,
    /// square centimetre
    ///
    /// rec20
    SquareCentimetre,
    /// cubic centimetre
    ///
    /// rec20
    CubicCentimetre,
    /// centimetre
    ///
    /// rec20
    Centimetre,
    /// hundred pack
    ///
    /// rec20
    HundredPack,
    /// cental (UK)
    ///
    /// rec20
    CentalUk,
    /// coulomb
    ///
    /// rec20
    Coulomb,
    /// content gram
    ///
    /// rec20
    ContentGram,
    /// metric carat
    ///
    /// rec20
    MetricCarat,
    /// content ton (metric)
    ///
    /// rec20
    ContentTonMetric,
    /// curie
    ///
    /// rec20
    Curie,
    /// hundred pound (cwt) / hundred weight (US)
    ///
    /// rec20
    HundredPoundCwtHundredWeightUs,
    /// hundred weight (UK)
    ///
    /// rec20
    HundredWeightUk,
    /// kilowatt hour per hour
    ///
    /// rec20
    KilowattHourPerHour,
    /// lot [unit of weight]
    ///
    /// rec20
    LotUnitWeight,
    /// reciprocal second per steradian
    ///
    /// rec20
    ReciprocalSecondPerSteradian,
    /// siemens per metre
    ///
    /// rec20
    SiemensPerMetre,
    /// mebibit
    ///
    /// rec20
    Mebibit,
    /// siemens square metre per mole
    ///
    /// rec20
    SiemensSquareMetrePerMole,
    /// sievert
    ///
    /// rec20
    Sievert,
    /// sone
    ///
    /// rec20
    Sone,
    /// square centimetre per erg
    ///
    /// rec20
    SquareCentimetrePerErg,
    /// square centimetre per steradian erg
    ///
    /// rec20
    SquareCentimetrePerSteradianErg,
    /// metre kelvin
    ///
    /// rec20
    MetreKelvin,
    /// square metre kelvin per watt
    ///
    /// rec20
    SquareMetreKelvinPerWatt,
    /// reciprocal second per steradian metre squared
    ///
    /// rec20
    ReciprocalSecondPerSteradianMetreSquared,
    /// square metre per joule
    ///
    /// rec20
    SquareMetrePerJoule,
    /// square metre per kilogram
    ///
    /// rec20
    SquareMetrePerKilogram,
    /// square metre per mole
    ///
    /// rec20
    SquareMetrePerMole,
    /// pen gram (protein)
    ///
    /// rec20
    PenGramProtein,
    /// square metre per steradian
    ///
    /// rec20
    SquareMetrePerSteradian,
    /// square metre per steradian joule
    ///
    /// rec20
    SquareMetrePerSteradianJoule,
    /// square metre per volt second
    ///
    /// rec20
    SquareMetrePerVoltSecond,
    /// steradian
    ///
    /// rec20
    Steradian,
    /// terahertz
    ///
    /// rec20
    Terahertz,
    /// terajoule
    ///
    /// rec20
    Terajoule,
    /// terawatt
    ///
    /// rec20
    Terawatt,
    /// terawatt hour
    ///
    /// rec20
    TerawattHour,
    /// tesla
    ///
    /// rec20
    Tesla,
    /// tex
    ///
    /// rec20
    Tex,
    /// megabit
    ///
    /// rec20
    Megabit,
    /// tonne per cubic metre
    ///
    /// rec20
    TonnePerCubicMetre,
    /// tropical year
    ///
    /// rec20
    TropicalYear,
    /// unified atomic mass unit
    ///
    /// rec20
    UnifiedAtomicMassUnit,
    /// var
    ///
    /// rec20
    Var,
    /// volt squared per kelvin squared
    ///
    /// rec20
    VoltSquaredPerKelvinSquared,
    /// volt - ampere
    ///
    /// rec20
    VoltAmpere,
    /// volt per centimetre
    ///
    /// rec20
    VoltPerCentimetre,
    /// volt per kelvin
    ///
    /// rec20
    VoltPerKelvin,
    /// millivolt per kelvin
    ///
    /// rec20
    MillivoltPerKelvin,
    /// kilogram per square centimetre
    ///
    /// rec20
    KilogramPerSquareCentimetre,
    /// volt per metre
    ///
    /// rec20
    VoltPerMetre,
    /// volt per millimetre
    ///
    /// rec20
    VoltPerMillimetre,
    /// watt per kelvin
    ///
    /// rec20
    WattPerKelvin,
    /// watt per metre kelvin
    ///
    /// rec20
    WattPerMetreKelvin,
    /// watt per square metre
    ///
    /// rec20
    WattPerSquareMetre,
    /// watt per square metre kelvin
    ///
    /// rec20
    WattPerSquareMetreKelvin,
    /// watt per square metre kelvin to the fourth power
    ///
    /// rec20
    WattPerSquareMetreKelvinToFourthPower,
    /// watt per steradian
    ///
    /// rec20
    WattPerSteradian,
    /// watt per steradian square metre
    ///
    /// rec20
    WattPerSteradianSquareMetre,
    /// weber per metre
    ///
    /// rec20
    WeberPerMetre,
    /// roentgen per second
    ///
    /// rec20
    RoentgenPerSecond,
    /// weber per millimetre
    ///
    /// rec20
    WeberPerMillimetre,
    /// minute [unit of angle]
    ///
    /// rec20
    MinuteUnitAngle,
    /// second [unit of angle]
    ///
    /// rec20
    SecondUnitAngle,
    /// book
    ///
    /// rec20
    Book,
    /// round
    ///
    /// rec20
    Round,
    /// number of words
    ///
    /// rec20
    NumberWords,
    /// inch to the fourth power
    ///
    /// rec20
    InchToFourthPower,
    /// joule square metre
    ///
    /// rec20
    JouleSquareMetre,
    /// kilogram per mole
    ///
    /// rec20
    KilogramPerMole,
    /// megacoulomb
    ///
    /// rec20
    Megacoulomb,
    /// megajoule per second
    ///
    /// rec20
    MegajoulePerSecond,
    /// microwatt
    ///
    /// rec20
    Microwatt,
    /// microtesla
    ///
    /// rec20
    Microtesla,
    /// microvolt
    ///
    /// rec20
    Microvolt,
    /// millinewton metre
    ///
    /// rec20
    MillinewtonMetre,
    /// microwatt per square metre
    ///
    /// rec20
    MicrowattPerSquareMetre,
    /// millicoulomb
    ///
    /// rec20
    Millicoulomb,
    /// millimole per kilogram
    ///
    /// rec20
    MillimolePerKilogram,
    /// millicoulomb per cubic metre
    ///
    /// rec20
    MillicoulombPerCubicMetre,
    /// millicoulomb per square metre
    ///
    /// rec20
    MillicoulombPerSquareMetre,
    /// rem
    ///
    /// rec20
    Rem,
    /// second per cubic metre
    ///
    /// rec20
    SecondPerCubicMetre,
    /// second per cubic metre radian
    ///
    /// rec20
    SecondPerCubicMetreRadian,
    /// joule per gram
    ///
    /// rec20
    JoulePerGram,
    /// decare
    ///
    /// rec20
    Decare,
    /// ten day
    ///
    /// rec20
    TenDay,
    /// day
    ///
    /// rec20
    Day,
    /// dry pound
    ///
    /// rec20
    DryPound,
    /// Decibel-milliwatts
    ///
    /// Rec20
    DecibelMilliwatts,
    /// Decibel watt
    ///
    /// Rec20
    DecibelWatt,
    /// degree [unit of angle]
    ///
    /// rec20
    DegreeUnitAngle,
    /// decade
    ///
    /// rec20
    Decade,
    /// decigram
    ///
    /// rec20
    Decigram,
    /// decagram
    ///
    /// rec20
    Decagram,
    /// decilitre
    ///
    /// rec20
    Decilitre,
    /// cubic decametre
    ///
    /// rec20
    CubicDecametre,
    /// square decimetre
    ///
    /// rec20
    SquareDecimetre,
    /// standard kilolitre
    ///
    /// rec20
    StandardKilolitre,
    /// cubic decimetre
    ///
    /// rec20
    CubicDecimetre,
    /// decimetre
    ///
    /// rec20
    Decimetre,
    /// decinewton metre
    ///
    /// rec20
    DecinewtonMetre,
    /// dozen piece
    ///
    /// rec20
    DozenPiece,
    /// dozen pair
    ///
    /// rec20
    DozenPair,
    /// displacement tonnage
    ///
    /// rec20
    DisplacementTonnage,
    /// dram (US)
    ///
    /// rec20
    DramUs,
    /// dram (UK)
    ///
    /// rec20
    DramUk,
    /// dozen roll
    ///
    /// rec20
    DozenRoll,
    /// dry ton
    ///
    /// rec20
    DryTon,
    /// decitonne
    ///
    /// rec20
    Decitonne,
    /// pennyweight
    ///
    /// rec20
    Pennyweight,
    /// dozen
    ///
    /// rec20
    Dozen,
    /// dozen pack
    ///
    /// rec20
    DozenPack,
    /// newton per square centimetre
    ///
    /// rec20
    NewtonPerSquareCentimetre,
    /// megawatt hour per hour
    ///
    /// rec20
    MegawattHourPerHour,
    /// megawatt per hertz
    ///
    /// rec20
    MegawattPerHertz,
    /// milliampere hour
    ///
    /// rec20
    MilliampereHour,
    /// degree day
    ///
    /// rec20
    DegreeDay,
    /// mille
    ///
    /// rec20
    Mille,
    /// kilocalorie (international table)
    ///
    /// rec20
    KilocalorieInternationalTable,
    /// kilocalorie (thermochemical) per hour
    ///
    /// rec20
    KilocalorieThermochemicalPerHour,
    /// million Btu(IT) per hour
    ///
    /// rec20
    MillionBtuItPerHour,
    /// cubic foot per second
    ///
    /// rec20
    CubicFootPerSecond,
    /// tonne per hour
    ///
    /// rec20
    TonnePerHour,
    /// ping
    ///
    /// rec20
    Ping,
    /// megabit per second
    ///
    /// rec20
    MegabitPerSecond,
    /// shares
    ///
    /// rec20
    Shares,
    /// TEU
    ///
    /// rec20
    Teu,
    /// tyre
    ///
    /// rec20
    Tyre,
    /// active unit
    ///
    /// rec20
    ActiveUnit,
    /// dose
    ///
    /// rec20
    Dose,
    /// air dry ton
    ///
    /// rec20
    AirDryTon,
    /// strand
    ///
    /// rec20
    Strand,
    /// square metre per litre
    ///
    /// rec20
    SquareMetrePerLitre,
    /// litre per hour
    ///
    /// rec20
    LitrePerHour,
    /// foot per thousand
    ///
    /// rec20
    FootPerThousand,
    /// gigabyte
    ///
    /// rec20
    Gigabyte,
    /// terabyte
    ///
    /// rec20
    Terabyte,
    /// petabyte
    ///
    /// rec20
    Petabyte,
    /// pixel
    ///
    /// rec20
    Pixel,
    /// megapixel
    ///
    /// rec20
    Megapixel,
    /// dots per inch
    ///
    /// rec20
    DotsPerInch,
    /// gross kilogram
    ///
    /// rec20
    GrossKilogram,
    /// part per hundred thousand
    ///
    /// rec20
    PartPerHundredThousand,
    /// kilogram-force per square millimetre
    ///
    /// rec20
    KilogramForcePerSquareMillimetre,
    /// kilogram-force per square centimetre
    ///
    /// rec20
    KilogramForcePerSquareCentimetre,
    /// joule per square centimetre
    ///
    /// rec20
    JoulePerSquareCentimetre,
    /// kilogram-force metre per square centimetre
    ///
    /// rec20
    KilogramForceMetrePerSquareCentimetre,
    /// milliohm
    ///
    /// rec20
    Milliohm,
    /// kilowatt hour per cubic metre
    ///
    /// rec20
    KilowattHourPerCubicMetre,
    /// kilowatt hour per kelvin
    ///
    /// rec20
    KilowattHourPerKelvin,
    /// service unit
    ///
    /// rec20
    ServiceUnit,
    /// working day
    ///
    /// rec20
    WorkingDay,
    /// accounting unit
    ///
    /// rec20
    AccountingUnit,
    /// job
    ///
    /// rec20
    Job,
    /// run foot
    ///
    /// rec20
    RunFoot,
    /// test
    ///
    /// rec20
    Test,
    /// trip
    ///
    /// rec20
    Trip,
    /// use
    ///
    /// rec20
    Use,
    /// well
    ///
    /// rec20
    Well,
    /// zone
    ///
    /// rec20
    Zone,
    /// exabit per second
    ///
    /// rec20
    ExabitPerSecond,
    /// exbibyte
    ///
    /// rec20
    Exbibyte,
    /// pebibyte
    ///
    /// rec20
    Pebibyte,
    /// tebibyte
    ///
    /// rec20
    Tebibyte,
    /// gibibyte
    ///
    /// rec20
    Gibibyte,
    /// mebibyte
    ///
    /// rec20
    Mebibyte,
    /// kibibyte
    ///
    /// rec20
    Kibibyte,
    /// exbibit per metre
    ///
    /// rec20
    ExbibitPerMetre,
    /// exbibit per square metre
    ///
    /// rec20
    ExbibitPerSquareMetre,
    /// exbibit per cubic metre
    ///
    /// rec20
    ExbibitPerCubicMetre,
    /// gigabyte per second
    ///
    /// rec20
    GigabytePerSecond,
    /// gibibit per metre
    ///
    /// rec20
    GibibitPerMetre,
    /// gibibit per square metre
    ///
    /// rec20
    GibibitPerSquareMetre,
    /// gibibit per cubic metre
    ///
    /// rec20
    GibibitPerCubicMetre,
    /// kibibit per metre
    ///
    /// rec20
    KibibitPerMetre,
    /// kibibit per square metre
    ///
    /// rec20
    KibibitPerSquareMetre,
    /// kibibit per cubic metre
    ///
    /// rec20
    KibibitPerCubicMetre,
    /// mebibit per metre
    ///
    /// rec20
    MebibitPerMetre,
    /// mebibit per square metre
    ///
    /// rec20
    MebibitPerSquareMetre,
    /// mebibit per cubic metre
    ///
    /// rec20
    MebibitPerCubicMetre,
    /// petabit
    ///
    /// rec20
    Petabit,
    /// petabit per second
    ///
    /// rec20
    PetabitPerSecond,
    /// pebibit per metre
    ///
    /// rec20
    PebibitPerMetre,
    /// pebibit per square metre
    ///
    /// rec20
    PebibitPerSquareMetre,
    /// pebibit per cubic metre
    ///
    /// rec20
    PebibitPerCubicMetre,
    /// terabit
    ///
    /// rec20
    Terabit,
    /// terabit per second
    ///
    /// rec20
    TerabitPerSecond,
    /// tebibit per metre
    ///
    /// rec20
    TebibitPerMetre,
    /// tebibit per cubic metre
    ///
    /// rec20
    TebibitPerCubicMetre,
    /// tebibit per square metre
    ///
    /// rec20
    TebibitPerSquareMetre,
    /// bit per metre
    ///
    /// rec20
    BitPerMetre,
    /// bit per square metre
    ///
    /// rec20
    BitPerSquareMetre,
    /// reciprocal centimetre
    ///
    /// rec20
    ReciprocalCentimetre,
    /// reciprocal day
    ///
    /// rec20
    ReciprocalDay,
    /// cubic decimetre per hour
    ///
    /// rec20
    CubicDecimetrePerHour,
    /// kilogram per hour
    ///
    /// rec20
    KilogramPerHour,
    /// kilomole per second
    ///
    /// rec20
    KilomolePerSecond,
    /// mole per second
    ///
    /// rec20
    MolePerSecond,
    /// degree per second
    ///
    /// rec20
    DegreePerSecond,
    /// millimetre per degree Celcius metre
    ///
    /// rec20
    MillimetrePerDegreeCelciusMetre,
    /// degree Celsius per kelvin
    ///
    /// rec20
    DegreeCelsiusPerKelvin,
    /// hectopascal per bar
    ///
    /// rec20
    HectopascalPerBar,
    /// each
    ///
    /// rec20
    Each,
    /// electronic mail box
    ///
    /// rec20
    ElectronicMailBox,
    /// equivalent gallon
    ///
    /// rec20
    EquivalentGallon,
    /// bit per cubic metre
    ///
    /// rec20
    BitPerCubicMetre,
    /// kelvin per kelvin
    ///
    /// rec20
    KelvinPerKelvin,
    /// kilopascal per bar
    ///
    /// rec20
    KilopascalPerBar,
    /// millibar per bar
    ///
    /// rec20
    MillibarPerBar,
    /// megapascal per bar
    ///
    /// rec20
    MegapascalPerBar,
    /// poise per bar
    ///
    /// rec20
    PoisePerBar,
    /// pascal per bar
    ///
    /// rec20
    PascalPerBar,
    /// milliampere per inch
    ///
    /// rec20
    MilliamperePerInch,
    /// kelvin per hour
    ///
    /// rec20
    KelvinPerHour,
    /// kelvin per minute
    ///
    /// rec20
    KelvinPerMinute,
    /// kelvin per second
    ///
    /// rec20
    KelvinPerSecond,
    /// slug
    ///
    /// rec20
    Slug,
    /// gram per kelvin
    ///
    /// rec20
    GramPerKelvin,
    /// kilogram per kelvin
    ///
    /// rec20
    KilogramPerKelvin,
    /// milligram per kelvin
    ///
    /// rec20
    MilligramPerKelvin,
    /// pound-force per foot
    ///
    /// rec20
    PoundForcePerFoot,
    /// kilogram square centimetre
    ///
    /// rec20
    KilogramSquareCentimetre,
    /// kilogram square millimetre
    ///
    /// rec20
    KilogramSquareMillimetre,
    /// pound inch squared
    ///
    /// rec20
    PoundInchSquared,
    /// pound-force inch
    ///
    /// rec20
    PoundForceInch,
    /// pound-force foot per ampere
    ///
    /// rec20
    PoundForceFootPerAmpere,
    /// gram per cubic decimetre
    ///
    /// rec20
    GramPerCubicDecimetre,
    /// kilogram per kilomol
    ///
    /// rec20
    KilogramPerKilomol,
    /// gram per hertz
    ///
    /// rec20
    GramPerHertz,
    /// gram per day
    ///
    /// rec20
    GramPerDay,
    /// gram per hour
    ///
    /// rec20
    GramPerHour,
    /// gram per minute
    ///
    /// rec20
    GramPerMinute,
    /// gram per second
    ///
    /// rec20
    GramPerSecond,
    /// kilogram per day
    ///
    /// rec20
    KilogramPerDay,
    /// kilogram per minute
    ///
    /// rec20
    KilogramPerMinute,
    /// milligram per day
    ///
    /// rec20
    MilligramPerDay,
    /// milligram per minute
    ///
    /// rec20
    MilligramPerMinute,
    /// milligram per second
    ///
    /// rec20
    MilligramPerSecond,
    /// gram per day kelvin
    ///
    /// rec20
    GramPerDayKelvin,
    /// gram per hour kelvin
    ///
    /// rec20
    GramPerHourKelvin,
    /// gram per minute kelvin
    ///
    /// rec20
    GramPerMinuteKelvin,
    /// gram per second kelvin
    ///
    /// rec20
    GramPerSecondKelvin,
    /// kilogram per day kelvin
    ///
    /// rec20
    KilogramPerDayKelvin,
    /// kilogram per hour kelvin
    ///
    /// rec20
    KilogramPerHourKelvin,
    /// kilogram per minute kelvin
    ///
    /// rec20
    KilogramPerMinuteKelvin,
    /// kilogram per second kelvin
    ///
    /// rec20
    KilogramPerSecondKelvin,
    /// milligram per day kelvin
    ///
    /// rec20
    MilligramPerDayKelvin,
    /// milligram per hour kelvin
    ///
    /// rec20
    MilligramPerHourKelvin,
    /// milligram per minute kelvin
    ///
    /// rec20
    MilligramPerMinuteKelvin,
    /// milligram per second kelvin
    ///
    /// rec20
    MilligramPerSecondKelvin,
    /// newton per millimetre
    ///
    /// rec20
    NewtonPerMillimetre,
    /// pound-force per inch
    ///
    /// rec20
    PoundForcePerInch,
    /// rod [unit of distance]
    ///
    /// rec20
    RodUnitDistance,
    /// micrometre per kelvin
    ///
    /// rec20
    MicrometrePerKelvin,
    /// centimetre per kelvin
    ///
    /// rec20
    CentimetrePerKelvin,
    /// metre per kelvin
    ///
    /// rec20
    MetrePerKelvin,
    /// millimetre per kelvin
    ///
    /// rec20
    MillimetrePerKelvin,
    /// milliohm per metre
    ///
    /// rec20
    MilliohmPerMetre,
    /// ohm per mile (statute mile)
    ///
    /// rec20
    OhmPerMileStatuteMile,
    /// ohm per kilometre
    ///
    /// rec20
    OhmPerKilometre,
    /// milliampere per pound-force per square inch
    ///
    /// rec20
    MilliamperePerPoundForcePerSquareInch,
    /// reciprocal bar
    ///
    /// rec20
    ReciprocalBar,
    /// milliampere per bar
    ///
    /// rec20
    MilliamperePerBar,
    /// degree Celsius per bar
    ///
    /// rec20
    DegreeCelsiusPerBar,
    /// kelvin per bar
    ///
    /// rec20
    KelvinPerBar,
    /// gram per day bar
    ///
    /// rec20
    GramPerDayBar,
    /// gram per hour bar
    ///
    /// rec20
    GramPerHourBar,
    /// gram per minute bar
    ///
    /// rec20
    GramPerMinuteBar,
    /// gram per second bar
    ///
    /// rec20
    GramPerSecondBar,
    /// kilogram per day bar
    ///
    /// rec20
    KilogramPerDayBar,
    /// kilogram per hour bar
    ///
    /// rec20
    KilogramPerHourBar,
    /// kilogram per minute bar
    ///
    /// rec20
    KilogramPerMinuteBar,
    /// kilogram per second bar
    ///
    /// rec20
    KilogramPerSecondBar,
    /// milligram per day bar
    ///
    /// rec20
    MilligramPerDayBar,
    /// milligram per hour bar
    ///
    /// rec20
    MilligramPerHourBar,
    /// milligram per minute bar
    ///
    /// rec20
    MilligramPerMinuteBar,
    /// milligram per second bar
    ///
    /// rec20
    MilligramPerSecondBar,
    /// gram per bar
    ///
    /// rec20
    GramPerBar,
    /// milligram per bar
    ///
    /// rec20
    MilligramPerBar,
    /// milliampere per millimetre
    ///
    /// rec20
    MilliamperePerMillimetre,
    /// pascal second per kelvin
    ///
    /// rec20
    PascalSecondPerKelvin,
    /// inch of water
    ///
    /// rec20
    InchWater,
    /// inch of mercury
    ///
    /// rec20
    InchMercury,
    /// water horse power
    ///
    /// rec20
    WaterHorsePower,
    /// bar per kelvin
    ///
    /// rec20
    BarPerKelvin,
    /// hectopascal per kelvin
    ///
    /// rec20
    HectopascalPerKelvin,
    /// kilopascal per kelvin
    ///
    /// rec20
    KilopascalPerKelvin,
    /// millibar per kelvin
    ///
    /// rec20
    MillibarPerKelvin,
    /// megapascal per kelvin
    ///
    /// rec20
    MegapascalPerKelvin,
    /// poise per kelvin
    ///
    /// rec20
    PoisePerKelvin,
    /// volt per litre minute
    ///
    /// rec20
    VoltPerLitreMinute,
    /// newton centimetre
    ///
    /// rec20
    NewtonCentimetre,
    /// newton metre per degree
    ///
    /// rec20
    NewtonMetrePerDegree,
    /// newton metre per ampere
    ///
    /// rec20
    NewtonMetrePerAmpere,
    /// bar litre per second
    ///
    /// rec20
    BarLitrePerSecond,
    /// bar cubic metre per second
    ///
    /// rec20
    BarCubicMetrePerSecond,
    /// hectopascal litre per second
    ///
    /// rec20
    HectopascalLitrePerSecond,
    /// hectopascal cubic metre per second
    ///
    /// rec20
    HectopascalCubicMetrePerSecond,
    /// millibar litre per second
    ///
    /// rec20
    MillibarLitrePerSecond,
    /// millibar cubic metre per second
    ///
    /// rec20
    MillibarCubicMetrePerSecond,
    /// megapascal litre per second
    ///
    /// rec20
    MegapascalLitrePerSecond,
    /// megapascal cubic metre per second
    ///
    /// rec20
    MegapascalCubicMetrePerSecond,
    /// pascal litre per second
    ///
    /// rec20
    PascalLitrePerSecond,
    /// degree Fahrenheit
    ///
    /// rec20
    DegreeFahrenheit,
    /// farad
    ///
    /// rec20
    Farad,
    /// fibre metre
    ///
    /// rec20
    FibreMetre,
    /// thousand cubic foot
    ///
    /// rec20
    ThousandCubicFoot,
    /// hundred cubic metre
    ///
    /// rec20
    HundredCubicMetre,
    /// micromole
    ///
    /// rec20
    Micromole,
    /// failures in time
    ///
    /// rec20
    FailuresInTime,
    /// flake ton
    ///
    /// rec20
    FlakeTon,
    /// Formazin nephelometric unit
    ///
    /// Rec20
    FormazinNephelometricUnit,
    /// foot
    ///
    /// rec20
    Foot,
    /// pound per square foot
    ///
    /// rec20
    PoundPerSquareFoot,
    /// foot per minute
    ///
    /// rec20
    FootPerMinute,
    /// foot per second
    ///
    /// rec20
    FootPerSecond,
    /// square foot
    ///
    /// rec20
    SquareFoot,
    /// cubic foot
    ///
    /// rec20
    CubicFoot,
    /// pascal cubic metre per second
    ///
    /// rec20
    PascalCubicMetrePerSecond,
    /// centimetre per bar
    ///
    /// rec20
    CentimetrePerBar,
    /// metre per bar
    ///
    /// rec20
    MetrePerBar,
    /// millimetre per bar
    ///
    /// rec20
    MillimetrePerBar,
    /// square inch per second
    ///
    /// rec20
    SquareInchPerSecond,
    /// square metre per second kelvin
    ///
    /// rec20
    SquareMetrePerSecondKelvin,
    /// stokes per kelvin
    ///
    /// rec20
    StokesPerKelvin,
    /// gram per cubic centimetre bar
    ///
    /// rec20
    GramPerCubicCentimetreBar,
    /// gram per cubic decimetre bar
    ///
    /// rec20
    GramPerCubicDecimetreBar,
    /// gram per litre bar
    ///
    /// rec20
    GramPerLitreBar,
    /// gram per cubic metre bar
    ///
    /// rec20
    GramPerCubicMetreBar,
    /// gram per millilitre bar
    ///
    /// rec20
    GramPerMillilitreBar,
    /// kilogram per cubic centimetre bar
    ///
    /// rec20
    KilogramPerCubicCentimetreBar,
    /// kilogram per litre bar
    ///
    /// rec20
    KilogramPerLitreBar,
    /// kilogram per cubic metre bar
    ///
    /// rec20
    KilogramPerCubicMetreBar,
    /// newton metre per kilogram
    ///
    /// rec20
    NewtonMetrePerKilogram,
    /// US gallon per minute
    ///
    /// rec20
    UsGallonPerMinute,
    /// pound-force foot per pound
    ///
    /// rec20
    PoundForceFootPerPound,
    /// cup [unit of volume]
    ///
    /// rec20
    CupUnitVolume,
    /// peck
    ///
    /// rec20
    Peck,
    /// tablespoon (US)
    ///
    /// rec20
    TablespoonUs,
    /// teaspoon (US)
    ///
    /// rec20
    TeaspoonUs,
    /// stere
    ///
    /// rec20
    Stere,
    /// cubic centimetre per kelvin
    ///
    /// rec20
    CubicCentimetrePerKelvin,
    /// litre per kelvin
    ///
    /// rec20
    LitrePerKelvin,
    /// cubic metre per kelvin
    ///
    /// rec20
    CubicMetrePerKelvin,
    /// Imperial gallon per minute
    ///
    /// rec20
    ImperialGallonPerMinute,
    /// millilitre per kelvin
    ///
    /// rec20
    MillilitrePerKelvin,
    /// kilogram per cubic centimetre
    ///
    /// rec20
    KilogramPerCubicCentimetre,
    /// ounce (avoirdupois) per cubic yard
    ///
    /// rec20
    OunceAvoirdupoisPerCubicYard,
    /// gram per cubic centimetre kelvin
    ///
    /// rec20
    GramPerCubicCentimetreKelvin,
    /// gram per cubic decimetre kelvin
    ///
    /// rec20
    GramPerCubicDecimetreKelvin,
    /// gram per litre kelvin
    ///
    /// rec20
    GramPerLitreKelvin,
    /// gram per cubic metre kelvin
    ///
    /// rec20
    GramPerCubicMetreKelvin,
    /// gram per millilitre kelvin
    ///
    /// rec20
    GramPerMillilitreKelvin,
    /// kilogram per cubic centimetre kelvin
    ///
    /// rec20
    KilogramPerCubicCentimetreKelvin,
    /// kilogram per litre kelvin
    ///
    /// rec20
    KilogramPerLitreKelvin,
    /// kilogram per cubic metre kelvin
    ///
    /// rec20
    KilogramPerCubicMetreKelvin,
    /// square metre per second bar
    ///
    /// rec20
    SquareMetrePerSecondBar,
    /// microsiemens per centimetre
    ///
    /// rec20
    MicrosiemensPerCentimetre,
    /// microsiemens per metre
    ///
    /// rec20
    MicrosiemensPerMetre,
    /// nanosiemens per centimetre
    ///
    /// rec20
    NanosiemensPerCentimetre,
    /// nanosiemens per metre
    ///
    /// rec20
    NanosiemensPerMetre,
    /// stokes per bar
    ///
    /// rec20
    StokesPerBar,
    /// cubic centimetre per day
    ///
    /// rec20
    CubicCentimetrePerDay,
    /// cubic centimetre per hour
    ///
    /// rec20
    CubicCentimetrePerHour,
    /// cubic centimetre per minute
    ///
    /// rec20
    CubicCentimetrePerMinute,
    /// gallon (US) per hour
    ///
    /// rec20
    GallonUsPerHour,
    /// litre per second
    ///
    /// rec20
    LitrePerSecond,
    /// cubic metre per day
    ///
    /// rec20
    CubicMetrePerDay,
    /// cubic metre per minute
    ///
    /// rec20
    CubicMetrePerMinute,
    /// millilitre per day
    ///
    /// rec20
    MillilitrePerDay,
    /// millilitre per hour
    ///
    /// rec20
    MillilitrePerHour,
    /// cubic inch per hour
    ///
    /// rec20
    CubicInchPerHour,
    /// cubic inch per minute
    ///
    /// rec20
    CubicInchPerMinute,
    /// cubic inch per second
    ///
    /// rec20
    CubicInchPerSecond,
    /// milliampere per litre minute
    ///
    /// rec20
    MilliamperePerLitreMinute,
    /// volt per bar
    ///
    /// rec20
    VoltPerBar,
    /// cubic centimetre per day kelvin
    ///
    /// rec20
    CubicCentimetrePerDayKelvin,
    /// cubic centimetre per hour kelvin
    ///
    /// rec20
    CubicCentimetrePerHourKelvin,
    /// cubic centimetre per minute kelvin
    ///
    /// rec20
    CubicCentimetrePerMinuteKelvin,
    /// cubic centimetre per second kelvin
    ///
    /// rec20
    CubicCentimetrePerSecondKelvin,
    /// litre per day kelvin
    ///
    /// rec20
    LitrePerDayKelvin,
    /// litre per hour kelvin
    ///
    /// rec20
    LitrePerHourKelvin,
    /// litre per minute kelvin
    ///
    /// rec20
    LitrePerMinuteKelvin,
    /// litre per second kelvin
    ///
    /// rec20
    LitrePerSecondKelvin,
    /// cubic metre per day kelvin
    ///
    /// rec20
    CubicMetrePerDayKelvin,
    /// cubic metre per hour kelvin
    ///
    /// rec20
    CubicMetrePerHourKelvin,
    /// cubic metre per minute kelvin
    ///
    /// rec20
    CubicMetrePerMinuteKelvin,
    /// cubic metre per second kelvin
    ///
    /// rec20
    CubicMetrePerSecondKelvin,
    /// millilitre per day kelvin
    ///
    /// rec20
    MillilitrePerDayKelvin,
    /// millilitre per hour kelvin
    ///
    /// rec20
    MillilitrePerHourKelvin,
    /// millilitre per minute kelvin
    ///
    /// rec20
    MillilitrePerMinuteKelvin,
    /// millilitre per second kelvin
    ///
    /// rec20
    MillilitrePerSecondKelvin,
    /// millimetre to the fourth power
    ///
    /// rec20
    MillimetreToFourthPower,
    /// cubic centimetre per day bar
    ///
    /// rec20
    CubicCentimetrePerDayBar,
    /// cubic centimetre per hour bar
    ///
    /// rec20
    CubicCentimetrePerHourBar,
    /// cubic centimetre per minute bar
    ///
    /// rec20
    CubicCentimetrePerMinuteBar,
    /// cubic centimetre per second bar
    ///
    /// rec20
    CubicCentimetrePerSecondBar,
    /// litre per day bar
    ///
    /// rec20
    LitrePerDayBar,
    /// litre per hour bar
    ///
    /// rec20
    LitrePerHourBar,
    /// litre per minute bar
    ///
    /// rec20
    LitrePerMinuteBar,
    /// litre per second bar
    ///
    /// rec20
    LitrePerSecondBar,
    /// cubic metre per day bar
    ///
    /// rec20
    CubicMetrePerDayBar,
    /// cubic metre per hour bar
    ///
    /// rec20
    CubicMetrePerHourBar,
    /// cubic metre per minute bar
    ///
    /// rec20
    CubicMetrePerMinuteBar,
    /// cubic metre per second bar
    ///
    /// rec20
    CubicMetrePerSecondBar,
    /// millilitre per day bar
    ///
    /// rec20
    MillilitrePerDayBar,
    /// millilitre per hour bar
    ///
    /// rec20
    MillilitrePerHourBar,
    /// millilitre per minute bar
    ///
    /// rec20
    MillilitrePerMinuteBar,
    /// millilitre per second bar
    ///
    /// rec20
    MillilitrePerSecondBar,
    /// cubic centimetre per bar
    ///
    /// rec20
    CubicCentimetrePerBar,
    /// litre per bar
    ///
    /// rec20
    LitrePerBar,
    /// cubic metre per bar
    ///
    /// rec20
    CubicMetrePerBar,
    /// millilitre per bar
    ///
    /// rec20
    MillilitrePerBar,
    /// microhenry per kiloohm
    ///
    /// rec20
    MicrohenryPerKiloohm,
    /// microhenry per ohm
    ///
    /// rec20
    MicrohenryPerOhm,
    /// gallon (US) per day
    ///
    /// rec20
    GallonUsPerDay,
    /// gigabecquerel
    ///
    /// rec20
    Gigabecquerel,
    /// gram, dry weight
    ///
    /// rec20
    GramDryWeight,
    /// pound per gallon (US)
    ///
    /// rec20
    PoundPerGallonUs,
    /// gram per metre (gram per 100 centimetres)
    ///
    /// rec20
    GramPerMetreGramPer100Centimetres,
    /// gram of fissile isotope
    ///
    /// rec20
    GramFissileIsotope,
    /// great gross
    ///
    /// rec20
    GreatGross,
    /// gill (US)
    ///
    /// rec20
    GillUs,
    /// gram, including container
    ///
    /// rec20
    GramIncludingContainer,
    /// gill (UK)
    ///
    /// rec20
    GillUk,
    /// gram, including inner packaging
    ///
    /// rec20
    GramIncludingInnerPackaging,
    /// gram per millilitre
    ///
    /// rec20
    GramPerMillilitre,
    /// gram per litre
    ///
    /// rec20
    GramPerLitre,
    /// dry gallon (US)
    ///
    /// rec20
    DryGallonUs,
    /// gallon (UK)
    ///
    /// rec20
    GallonUk,
    /// gallon (US)
    ///
    /// rec20
    GallonUs,
    /// gram per square metre
    ///
    /// rec20
    GramPerSquareMetre,
    /// milligram per square metre
    ///
    /// rec20
    MilligramPerSquareMetre,
    /// milligram per cubic metre
    ///
    /// rec20
    MilligramPerCubicMetre,
    /// microgram per cubic metre
    ///
    /// rec20
    MicrogramPerCubicMetre,
    /// gram
    ///
    /// rec20
    Gram,
    /// grain
    ///
    /// rec20
    Grain,
    /// gross
    ///
    /// rec20
    Gross,
    /// gigajoule
    ///
    /// rec20
    Gigajoule,
    /// gigawatt hour
    ///
    /// rec20
    GigawattHour,
    /// henry per kiloohm
    ///
    /// rec20
    HenryPerKiloohm,
    /// henry per ohm
    ///
    /// rec20
    HenryPerOhm,
    /// millihenry per kiloohm
    ///
    /// rec20
    MillihenryPerKiloohm,
    /// millihenry per ohm
    ///
    /// rec20
    MillihenryPerOhm,
    /// pascal second per bar
    ///
    /// rec20
    PascalSecondPerBar,
    /// microbecquerel
    ///
    /// rec20
    Microbecquerel,
    /// reciprocal year
    ///
    /// rec20
    ReciprocalYear,
    /// reciprocal hour
    ///
    /// rec20
    ReciprocalHour,
    /// reciprocal month
    ///
    /// rec20
    ReciprocalMonth,
    /// degree Celsius per hour
    ///
    /// rec20
    DegreeCelsiusPerHour,
    /// degree Celsius per minute
    ///
    /// rec20
    DegreeCelsiusPerMinute,
    /// degree Celsius per second
    ///
    /// rec20
    DegreeCelsiusPerSecond,
    /// square centimetre per gram
    ///
    /// rec20
    SquareCentimetrePerGram,
    /// square decametre
    ///
    /// rec20
    SquareDecametre,
    /// square hectometre
    ///
    /// rec20
    SquareHectometre,
    /// cubic hectometre
    ///
    /// rec20
    CubicHectometre,
    /// cubic kilometre
    ///
    /// rec20
    CubicKilometre,
    /// blank
    ///
    /// rec20
    Blank,
    /// volt square inch per pound-force
    ///
    /// rec20
    VoltSquareInchPerPoundForce,
    /// volt per inch
    ///
    /// rec20
    VoltPerInch,
    /// volt per microsecond
    ///
    /// rec20
    VoltPerMicrosecond,
    /// percent per kelvin
    ///
    /// rec20
    PercentPerKelvin,
    /// ohm per metre
    ///
    /// rec20
    OhmPerMetre,
    /// degree per metre
    ///
    /// rec20
    DegreePerMetre,
    /// microfarad per kilometre
    ///
    /// rec20
    MicrofaradPerKilometre,
    /// microgram per litre
    ///
    /// rec20
    MicrogramPerLitre,
    /// square micrometre (square micron)
    ///
    /// rec20
    SquareMicrometreSquareMicron,
    /// ampere per kilogram
    ///
    /// rec20
    AmperePerKilogram,
    /// ampere squared second
    ///
    /// rec20
    AmpereSquaredSecond,
    /// farad per kilometre
    ///
    /// rec20
    FaradPerKilometre,
    /// hertz metre
    ///
    /// rec20
    HertzMetre,
    /// kelvin metre per watt
    ///
    /// rec20
    KelvinMetrePerWatt,
    /// megaohm per kilometre
    ///
    /// rec20
    MegaohmPerKilometre,
    /// megaohm per metre
    ///
    /// rec20
    MegaohmPerMetre,
    /// megaampere
    ///
    /// rec20
    Megaampere,
    /// megahertz kilometre
    ///
    /// rec20
    MegahertzKilometre,
    /// newton per ampere
    ///
    /// rec20
    NewtonPerAmpere,
    /// newton metre watt to the power minus 0,5
    ///
    /// rec20
    NewtonMetreWattToPowerMinus05,
    /// pascal per metre
    ///
    /// rec20
    PascalPerMetre,
    /// siemens per centimetre
    ///
    /// rec20
    SiemensPerCentimetre,
    /// teraohm
    ///
    /// rec20
    Teraohm,
    /// volt second per metre
    ///
    /// rec20
    VoltSecondPerMetre,
    /// volt per second
    ///
    /// rec20
    VoltPerSecond,
    /// watt per cubic metre
    ///
    /// rec20
    WattPerCubicMetre,
    /// attofarad
    ///
    /// rec20
    Attofarad,
    /// centimetre per hour
    ///
    /// rec20
    CentimetrePerHour,
    /// reciprocal cubic centimetre
    ///
    /// rec20
    ReciprocalCubicCentimetre,
    /// decibel per kilometre
    ///
    /// rec20
    DecibelPerKilometre,
    /// decibel per metre
    ///
    /// rec20
    DecibelPerMetre,
    /// kilogram per bar
    ///
    /// rec20
    KilogramPerBar,
    /// kilogram per cubic decimetre kelvin
    ///
    /// rec20
    KilogramPerCubicDecimetreKelvin,
    /// kilogram per cubic decimetre bar
    ///
    /// rec20
    KilogramPerCubicDecimetreBar,
    /// kilogram per square metre second
    ///
    /// rec20
    KilogramPerSquareMetreSecond,
    /// inch per two pi radiant
    ///
    /// rec20
    InchPerTwoPiRadiant,
    /// metre per volt second
    ///
    /// rec20
    MetrePerVoltSecond,
    /// square metre per newton
    ///
    /// rec20
    SquareMetrePerNewton,
    /// cubic metre per cubic metre
    ///
    /// rec20
    CubicMetrePerCubicMetre,
    /// millisiemens per centimetre
    ///
    /// rec20
    MillisiemensPerCentimetre,
    /// millivolt per minute
    ///
    /// rec20
    MillivoltPerMinute,
    /// milligram per square centimetre
    ///
    /// rec20
    MilligramPerSquareCentimetre,
    /// milligram per gram
    ///
    /// rec20
    MilligramPerGram,
    /// millilitre per cubic metre
    ///
    /// rec20
    MillilitrePerCubicMetre,
    /// millimetre per year
    ///
    /// rec20
    MillimetrePerYear,
    /// millimetre per hour
    ///
    /// rec20
    MillimetrePerHour,
    /// millimole per gram
    ///
    /// rec20
    MillimolePerGram,
    /// picopascal per kilometre
    ///
    /// rec20
    PicopascalPerKilometre,
    /// picosecond
    ///
    /// rec20
    Picosecond,
    /// percent per month
    ///
    /// rec20
    PercentPerMonth,
    /// percent per hectobar
    ///
    /// rec20
    PercentPerHectobar,
    /// percent per decakelvin
    ///
    /// rec20
    PercentPerDecakelvin,
    /// watt per metre
    ///
    /// rec20
    WattPerMetre,
    /// decapascal
    ///
    /// rec20
    Decapascal,
    /// gram per millimetre
    ///
    /// rec20
    GramPerMillimetre,
    /// module width
    ///
    /// rec20
    ModuleWidth,
    /// French gauge
    ///
    /// rec20
    FrenchGauge,
    /// rack unit
    ///
    /// rec20
    RackUnit,
    /// millimetre per minute
    ///
    /// rec20
    MillimetrePerMinute,
    /// big point
    ///
    /// rec20
    BigPoint,
    /// litre per kilogram
    ///
    /// rec20
    LitrePerKilogram,
    /// gram millimetre
    ///
    /// rec20
    GramMillimetre,
    /// reciprocal week
    ///
    /// rec20
    ReciprocalWeek,
    /// piece
    ///
    /// rec20
    Piece,
    /// megaohm kilometre
    ///
    /// rec20
    MegaohmKilometre,
    /// percent per ohm
    ///
    /// rec20
    PercentPerOhm,
    /// percent per degree
    ///
    /// rec20
    PercentPerDegree,
    /// percent per ten thousand
    ///
    /// rec20
    PercentPerTenThousand,
    /// percent per one hundred thousand
    ///
    /// rec20
    PercentPerOneHundredThousand,
    /// percent per hundred
    ///
    /// rec20
    PercentPerHundred,
    /// percent per thousand
    ///
    /// rec20
    PercentPerThousand,
    /// percent per volt
    ///
    /// rec20
    PercentPerVolt,
    /// percent per bar
    ///
    /// rec20
    PercentPerBar,
    /// percent per inch
    ///
    /// rec20
    PercentPerInch,
    /// percent per metre
    ///
    /// rec20
    PercentPerMetre,
    /// hank
    ///
    /// rec20
    Hank,
    /// Piece Day
    ///
    /// Rec20
    PieceDay,
    /// hectobar
    ///
    /// rec20
    Hectobar,
    /// hundred boxes
    ///
    /// rec20
    HundredBoxes,
    /// hundred count
    ///
    /// rec20
    HundredCount,
    /// hundred kilogram, dry weight
    ///
    /// rec20
    HundredKilogramDryWeight,
    /// head
    ///
    /// rec20
    Head,
    /// hectogram
    ///
    /// rec20
    Hectogram,
    /// hundred cubic foot
    ///
    /// rec20
    HundredCubicFoot,
    /// hundred international unit
    ///
    /// rec20
    HundredInternationalUnit,
    /// hundred kilogram, net mass
    ///
    /// rec20
    HundredKilogramNetMass,
    /// hectolitre
    ///
    /// rec20
    Hectolitre,
    /// mile per hour (statute mile)
    ///
    /// rec20
    MilePerHourStatuteMile,
    /// Piece Month
    ///
    /// Rec20
    PieceMonth,
    /// million cubic metre
    ///
    /// rec20
    MillionCubicMetre,
    /// hectometre
    ///
    /// rec20
    Hectometre,
    /// hectolitre of pure alcohol
    ///
    /// rec20
    HectolitrePureAlcohol,
    /// hertz
    ///
    /// rec20
    Hertz,
    /// hour
    ///
    /// rec20
    Hour,
    /// Piece Week
    ///
    /// Rec20
    PieceWeek,
    /// inch pound (pound inch)
    ///
    /// rec20
    InchPoundPoundInch,
    /// person
    ///
    /// rec20
    Person,
    /// inch
    ///
    /// rec20
    Inch,
    /// square inch
    ///
    /// rec20
    SquareInch,
    /// cubic inch
    ///
    /// rec20
    CubicInch,
    /// international sugar degree
    ///
    /// rec20
    InternationalSugarDegree,
    /// inch per second
    ///
    /// rec20
    InchPerSecond,
    /// international unit per gram
    ///
    /// rec20
    InternationalUnitPerGram,
    /// inch per second squared
    ///
    /// rec20
    InchPerSecondSquared,
    /// percent per millimetre
    ///
    /// rec20
    PercentPerMillimetre,
    /// per mille per psi
    ///
    /// rec20
    PerMillePerPsi,
    /// degree API
    ///
    /// rec20
    DegreeApi,
    /// degree Baume (origin scale)
    ///
    /// rec20
    DegreeBaumeOriginScale,
    /// degree Baume (US heavy)
    ///
    /// rec20
    DegreeBaumeUsHeavy,
    /// degree Baume (US light)
    ///
    /// rec20
    DegreeBaumeUsLight,
    /// degree Balling
    ///
    /// rec20
    DegreeBalling,
    /// degree Brix
    ///
    /// rec20
    DegreeBrix,
    /// degree Fahrenheit hour square foot per British thermal unit (thermochemical)
    ///
    /// rec20
    DegreeFahrenheitHourSquareFootPerBritishThermalUnitThermochemical,
    /// joule per kilogram
    ///
    /// rec20
    JoulePerKilogram,
    /// degree Fahrenheit per kelvin
    ///
    /// rec20
    DegreeFahrenheitPerKelvin,
    /// degree Fahrenheit per bar
    ///
    /// rec20
    DegreeFahrenheitPerBar,
    /// degree Fahrenheit hour square foot per British thermal unit (international table)
    ///
    /// rec20
    DegreeFahrenheitHourSquareFootPerBritishThermalUnitInternationalTable,
    /// degree Fahrenheit per hour
    ///
    /// rec20
    DegreeFahrenheitPerHour,
    /// degree Fahrenheit per minute
    ///
    /// rec20
    DegreeFahrenheitPerMinute,
    /// degree Fahrenheit per second
    ///
    /// rec20
    DegreeFahrenheitPerSecond,
    /// reciprocal degree Fahrenheit
    ///
    /// rec20
    ReciprocalDegreeFahrenheit,
    /// degree Oechsle
    ///
    /// rec20
    DegreeOechsle,
    /// degree Rankine per hour
    ///
    /// rec20
    DegreeRankinePerHour,
    /// degree Rankine per minute
    ///
    /// rec20
    DegreeRankinePerMinute,
    /// degree Rankine per second
    ///
    /// rec20
    DegreeRankinePerSecond,
    /// degree Twaddell
    ///
    /// rec20
    DegreeTwaddell,
    /// micropoise
    ///
    /// rec20
    Micropoise,
    /// microgram per kilogram
    ///
    /// rec20
    MicrogramPerKilogram,
    /// microgram per cubic metre kelvin
    ///
    /// rec20
    MicrogramPerCubicMetreKelvin,
    /// microgram per cubic metre bar
    ///
    /// rec20
    MicrogramPerCubicMetreBar,
    /// microlitre per litre
    ///
    /// rec20
    MicrolitrePerLitre,
    /// baud
    ///
    /// rec20
    Baud,
    /// British thermal unit (mean)
    ///
    /// rec20
    BritishThermalUnitMean,
    /// British thermal unit (international table) foot per hour square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitInternationalTableFootPerHourSquareFootDegreeFahrenheit,
    /// British thermal unit (international table) inch per hour square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitInternationalTableInchPerHourSquareFootDegreeFahrenheit,
    /// British thermal unit (international table) inch per second square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitInternationalTableInchPerSecondSquareFootDegreeFahrenheit,
    /// British thermal unit (international table) per pound degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit,
    /// British thermal unit (international table) per minute
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerMinute,
    /// British thermal unit (international table) per second
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerSecond,
    /// British thermal unit (thermochemical) foot per hour square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitThermochemicalFootPerHourSquareFootDegreeFahrenheit,
    /// British thermal unit (thermochemical) per hour
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerHour,
    /// British thermal unit (thermochemical) inch per hour square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitThermochemicalInchPerHourSquareFootDegreeFahrenheit,
    /// British thermal unit (thermochemical) inch per second square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitThermochemicalInchPerSecondSquareFootDegreeFahrenheit,
    /// British thermal unit (thermochemical) per pound degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerPoundDegreeFahrenheit,
    /// British thermal unit (thermochemical) per minute
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerMinute,
    /// British thermal unit (thermochemical) per second
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerSecond,
    /// coulomb square metre per kilogram
    ///
    /// rec20
    CoulombSquareMetrePerKilogram,
    /// megabaud
    ///
    /// rec20
    Megabaud,
    /// watt second
    ///
    /// rec20
    WattSecond,
    /// bar per bar
    ///
    /// rec20
    BarPerBar,
    /// barrel (UK petroleum)
    ///
    /// rec20
    BarrelUkPetroleum,
    /// barrel (UK petroleum) per minute
    ///
    /// rec20
    BarrelUkPetroleumPerMinute,
    /// barrel (UK petroleum) per day
    ///
    /// rec20
    BarrelUkPetroleumPerDay,
    /// barrel (UK petroleum) per hour
    ///
    /// rec20
    BarrelUkPetroleumPerHour,
    /// barrel (UK petroleum) per second
    ///
    /// rec20
    BarrelUkPetroleumPerSecond,
    /// barrel (US petroleum) per hour
    ///
    /// rec20
    BarrelUsPetroleumPerHour,
    /// barrel (US petroleum) per second
    ///
    /// rec20
    BarrelUsPetroleumPerSecond,
    /// bushel (UK) per day
    ///
    /// rec20
    BushelUkPerDay,
    /// bushel (UK) per hour
    ///
    /// rec20
    BushelUkPerHour,
    /// bushel (UK) per minute
    ///
    /// rec20
    BushelUkPerMinute,
    /// bushel (UK) per second
    ///
    /// rec20
    BushelUkPerSecond,
    /// bushel (US dry) per day
    ///
    /// rec20
    BushelUsDryPerDay,
    /// bushel (US dry) per hour
    ///
    /// rec20
    BushelUsDryPerHour,
    /// bushel (US dry) per minute
    ///
    /// rec20
    BushelUsDryPerMinute,
    /// bushel (US dry) per second
    ///
    /// rec20
    BushelUsDryPerSecond,
    /// centinewton metre
    ///
    /// rec20
    CentinewtonMetre,
    /// centipoise per kelvin
    ///
    /// rec20
    CentipoisePerKelvin,
    /// centipoise per bar
    ///
    /// rec20
    CentipoisePerBar,
    /// calorie (mean)
    ///
    /// rec20
    CalorieMean,
    /// calorie (international table) per gram degree Celsius
    ///
    /// rec20
    CalorieInternationalTablePerGramDegreeCelsius,
    /// calorie (thermochemical) per centimetre second degree Celsius
    ///
    /// rec20
    CalorieThermochemicalPerCentimetreSecondDegreeCelsius,
    /// calorie (thermochemical) per gram degree Celsius
    ///
    /// rec20
    CalorieThermochemicalPerGramDegreeCelsius,
    /// calorie (thermochemical) per minute
    ///
    /// rec20
    CalorieThermochemicalPerMinute,
    /// calorie (thermochemical) per second
    ///
    /// rec20
    CalorieThermochemicalPerSecond,
    /// clo
    ///
    /// rec20
    Clo,
    /// centimetre per second kelvin
    ///
    /// rec20
    CentimetrePerSecondKelvin,
    /// centimetre per second bar
    ///
    /// rec20
    CentimetrePerSecondBar,
    /// cubic centimetre per cubic metre
    ///
    /// rec20
    CubicCentimetrePerCubicMetre,
    /// cubic decimetre per day
    ///
    /// rec20
    CubicDecimetrePerDay,
    /// cubic decimetre per cubic metre
    ///
    /// rec20
    CubicDecimetrePerCubicMetre,
    /// cubic decimetre per minute
    ///
    /// rec20
    CubicDecimetrePerMinute,
    /// cubic decimetre per second
    ///
    /// rec20
    CubicDecimetrePerSecond,
    /// ounce (UK fluid) per day
    ///
    /// rec20
    OunceUkFluidPerDay,
    /// ounce (UK fluid) per hour
    ///
    /// rec20
    OunceUkFluidPerHour,
    /// ounce (UK fluid) per minute
    ///
    /// rec20
    OunceUkFluidPerMinute,
    /// ounce (UK fluid) per second
    ///
    /// rec20
    OunceUkFluidPerSecond,
    /// ounce (US fluid) per day
    ///
    /// rec20
    OunceUsFluidPerDay,
    /// joule per kelvin
    ///
    /// rec20
    JoulePerKelvin,
    /// megajoule per kilogram
    ///
    /// rec20
    MegajoulePerKilogram,
    /// megajoule per cubic metre
    ///
    /// rec20
    MegajoulePerCubicMetre,
    /// pipeline joint
    ///
    /// rec20
    PipelineJoint,
    /// joule
    ///
    /// rec20
    Joule,
    /// hundred metre
    ///
    /// rec20
    HundredMetre,
    /// number of jewels
    ///
    /// rec20
    NumberJewels,
    /// kilowatt demand
    ///
    /// rec20
    KilowattDemand,
    /// ounce (US fluid) per hour
    ///
    /// rec20
    OunceUsFluidPerHour,
    /// ounce (US fluid) per minute
    ///
    /// rec20
    OunceUsFluidPerMinute,
    /// ounce (US fluid) per second
    ///
    /// rec20
    OunceUsFluidPerSecond,
    /// foot per degree Fahrenheit
    ///
    /// rec20
    FootPerDegreeFahrenheit,
    /// foot per hour
    ///
    /// rec20
    FootPerHour,
    /// foot pound-force per hour
    ///
    /// rec20
    FootPoundForcePerHour,
    /// foot pound-force per minute
    ///
    /// rec20
    FootPoundForcePerMinute,
    /// foot per psi
    ///
    /// rec20
    FootPerPsi,
    /// foot per second degree Fahrenheit
    ///
    /// rec20
    FootPerSecondDegreeFahrenheit,
    /// foot per second psi
    ///
    /// rec20
    FootPerSecondPsi,
    /// kilovolt ampere reactive demand
    ///
    /// rec20
    KilovoltAmpereReactiveDemand,
    /// reciprocal cubic foot
    ///
    /// rec20
    ReciprocalCubicFoot,
    /// cubic foot per degree Fahrenheit
    ///
    /// rec20
    CubicFootPerDegreeFahrenheit,
    /// cubic foot per day
    ///
    /// rec20
    CubicFootPerDay,
    /// cubic foot per psi
    ///
    /// rec20
    CubicFootPerPsi,
    /// gallon (UK) per day
    ///
    /// rec20
    GallonUkPerDay,
    /// gallon (UK) per hour
    ///
    /// rec20
    GallonUkPerHour,
    /// gallon (UK) per second
    ///
    /// rec20
    GallonUkPerSecond,
    /// kilovolt ampere reactive hour
    ///
    /// rec20
    KilovoltAmpereReactiveHour,
    /// gallon (US liquid) per second
    ///
    /// rec20
    GallonUsLiquidPerSecond,
    /// gram-force per square centimetre
    ///
    /// rec20
    GramForcePerSquareCentimetre,
    /// gill (UK) per day
    ///
    /// rec20
    GillUkPerDay,
    /// gill (UK) per hour
    ///
    /// rec20
    GillUkPerHour,
    /// gill (UK) per minute
    ///
    /// rec20
    GillUkPerMinute,
    /// gill (UK) per second
    ///
    /// rec20
    GillUkPerSecond,
    /// gill (US) per day
    ///
    /// rec20
    GillUsPerDay,
    /// gill (US) per hour
    ///
    /// rec20
    GillUsPerHour,
    /// gill (US) per minute
    ///
    /// rec20
    GillUsPerMinute,
    /// gill (US) per second
    ///
    /// rec20
    GillUsPerSecond,
    /// standard acceleration of free fall
    ///
    /// rec20
    StandardAccelerationFreeFall,
    /// grain per gallon (US)
    ///
    /// rec20
    GrainPerGallonUs,
    /// horsepower (boiler)
    ///
    /// rec20
    HorsepowerBoiler,
    /// horsepower (electric)
    ///
    /// rec20
    HorsepowerElectric,
    /// inch per degree Fahrenheit
    ///
    /// rec20
    InchPerDegreeFahrenheit,
    /// inch per psi
    ///
    /// rec20
    InchPerPsi,
    /// inch per second degree Fahrenheit
    ///
    /// rec20
    InchPerSecondDegreeFahrenheit,
    /// inch per second psi
    ///
    /// rec20
    InchPerSecondPsi,
    /// reciprocal cubic inch
    ///
    /// rec20
    ReciprocalCubicInch,
    /// kilobaud
    ///
    /// rec20
    Kilobaud,
    /// kilocalorie (mean)
    ///
    /// rec20
    KilocalorieMean,
    /// kilocalorie (international table) per hour metre degree Celsius
    ///
    /// rec20
    KilocalorieInternationalTablePerHourMetreDegreeCelsius,
    /// kilocalorie (thermochemical)
    ///
    /// rec20
    KilocalorieThermochemical,
    /// kilocalorie (thermochemical) per minute
    ///
    /// rec20
    KilocalorieThermochemicalPerMinute,
    /// kilocalorie (thermochemical) per second
    ///
    /// rec20
    KilocalorieThermochemicalPerSecond,
    /// kilomole per hour
    ///
    /// rec20
    KilomolePerHour,
    /// kilomole per cubic metre kelvin
    ///
    /// rec20
    KilomolePerCubicMetreKelvin,
    /// kilolitre
    ///
    /// rec20
    Kilolitre,
    /// kilomole per cubic metre bar
    ///
    /// rec20
    KilomolePerCubicMetreBar,
    /// kilomole per minute
    ///
    /// rec20
    KilomolePerMinute,
    /// litre per litre
    ///
    /// rec20
    LitrePerLitre,
    /// reciprocal litre
    ///
    /// rec20
    ReciprocalLitre,
    /// pound (avoirdupois) per degree Fahrenheit
    ///
    /// rec20
    PoundAvoirdupoisPerDegreeFahrenheit,
    /// pound (avoirdupois) square foot
    ///
    /// rec20
    PoundAvoirdupoisSquareFoot,
    /// pound (avoirdupois) per day
    ///
    /// rec20
    PoundAvoirdupoisPerDay,
    /// pound per foot hour
    ///
    /// rec20
    PoundPerFootHour,
    /// pound per foot second
    ///
    /// rec20
    PoundPerFootSecond,
    /// pound (avoirdupois) per cubic foot degree Fahrenheit
    ///
    /// rec20
    PoundAvoirdupoisPerCubicFootDegreeFahrenheit,
    /// pound (avoirdupois) per cubic foot psi
    ///
    /// rec20
    PoundAvoirdupoisPerCubicFootPsi,
    /// pound (avoirdupois) per gallon (UK)
    ///
    /// rec20
    PoundAvoirdupoisPerGallonUk,
    /// pound (avoirdupois) per hour degree Fahrenheit
    ///
    /// rec20
    PoundAvoirdupoisPerHourDegreeFahrenheit,
    /// pound (avoirdupois) per hour psi
    ///
    /// rec20
    PoundAvoirdupoisPerHourPsi,
    /// pound (avoirdupois) per cubic inch degree Fahrenheit
    ///
    /// rec20
    PoundAvoirdupoisPerCubicInchDegreeFahrenheit,
    /// pound (avoirdupois) per cubic inch psi
    ///
    /// rec20
    PoundAvoirdupoisPerCubicInchPsi,
    /// pound (avoirdupois) per psi
    ///
    /// rec20
    PoundAvoirdupoisPerPsi,
    /// pound (avoirdupois) per minute
    ///
    /// rec20
    PoundAvoirdupoisPerMinute,
    /// pound (avoirdupois) per minute degree Fahrenheit
    ///
    /// rec20
    PoundAvoirdupoisPerMinuteDegreeFahrenheit,
    /// pound (avoirdupois) per minute psi
    ///
    /// rec20
    PoundAvoirdupoisPerMinutePsi,
    /// pound (avoirdupois) per second
    ///
    /// rec20
    PoundAvoirdupoisPerSecond,
    /// pound (avoirdupois) per second degree Fahrenheit
    ///
    /// rec20
    PoundAvoirdupoisPerSecondDegreeFahrenheit,
    /// pound (avoirdupois) per second psi
    ///
    /// rec20
    PoundAvoirdupoisPerSecondPsi,
    /// pound per cubic yard
    ///
    /// rec20
    PoundPerCubicYard,
    /// pound-force per square foot
    ///
    /// rec20
    PoundForcePerSquareFoot,
    /// pound-force per square inch degree Fahrenheit
    ///
    /// rec20
    PoundForcePerSquareInchDegreeFahrenheit,
    /// psi cubic inch per second
    ///
    /// rec20
    PsiCubicInchPerSecond,
    /// psi litre per second
    ///
    /// rec20
    PsiLitrePerSecond,
    /// psi cubic metre per second
    ///
    /// rec20
    PsiCubicMetrePerSecond,
    /// psi cubic yard per second
    ///
    /// rec20
    PsiCubicYardPerSecond,
    /// pound-force second per square foot
    ///
    /// rec20
    PoundForceSecondPerSquareFoot,
    /// pound-force second per square inch
    ///
    /// rec20
    PoundForceSecondPerSquareInch,
    /// reciprocal psi
    ///
    /// rec20
    ReciprocalPsi,
    /// quart (UK liquid) per day
    ///
    /// rec20
    QuartUkLiquidPerDay,
    /// quart (UK liquid) per hour
    ///
    /// rec20
    QuartUkLiquidPerHour,
    /// quart (UK liquid) per minute
    ///
    /// rec20
    QuartUkLiquidPerMinute,
    /// quart (UK liquid) per second
    ///
    /// rec20
    QuartUkLiquidPerSecond,
    /// quart (US liquid) per day
    ///
    /// rec20
    QuartUsLiquidPerDay,
    /// quart (US liquid) per hour
    ///
    /// rec20
    QuartUsLiquidPerHour,
    /// cake
    ///
    /// rec20
    Cake,
    /// katal
    ///
    /// rec20
    Katal,
    /// kilocharacter
    ///
    /// rec20
    Kilocharacter,
    /// kilobar
    ///
    /// rec20
    Kilobar,
    /// kilogram of choline chloride
    ///
    /// rec20
    KilogramCholineChloride,
    /// kilogram drained net weight
    ///
    /// rec20
    KilogramDrainedNetWeight,
    /// kelvin
    ///
    /// rec20
    Kelvin,
    /// kilogram
    ///
    /// rec20
    Kilogram,
    /// kilogram per second
    ///
    /// rec20
    KilogramPerSecond,
    /// kilogram of hydrogen peroxide
    ///
    /// rec20
    KilogramHydrogenPeroxide,
    /// kilohertz
    ///
    /// rec20
    Kilohertz,
    /// kilogram per millimetre width
    ///
    /// rec20
    KilogramPerMillimetreWidth,
    /// kilogram, including container
    ///
    /// rec20
    KilogramIncludingContainer,
    /// kilogram, including inner packaging
    ///
    /// rec20
    KilogramIncludingInnerPackaging,
    /// kilosegment
    ///
    /// rec20
    Kilosegment,
    /// kilojoule
    ///
    /// rec20
    Kilojoule,
    /// kilogram per metre
    ///
    /// rec20
    KilogramPerMetre,
    /// lactic dry material percentage
    ///
    /// rec20
    LacticDryMaterialPercentage,
    /// kilolux
    ///
    /// rec20
    Kilolux,
    /// kilogram of methylamine
    ///
    /// rec20
    KilogramMethylamine,
    /// kilometre per hour
    ///
    /// rec20
    KilometrePerHour,
    /// square kilometre
    ///
    /// rec20
    SquareKilometre,
    /// kilogram per cubic metre
    ///
    /// rec20
    KilogramPerCubicMetre,
    /// kilometre
    ///
    /// rec20
    Kilometre,
    /// kilogram of nitrogen
    ///
    /// rec20
    KilogramNitrogen,
    /// kilonewton per square metre
    ///
    /// rec20
    KilonewtonPerSquareMetre,
    /// kilogram named substance
    ///
    /// rec20
    KilogramNamedSubstance,
    /// knot
    ///
    /// rec20
    Knot,
    /// milliequivalence caustic potash per gram of product
    ///
    /// rec20
    MilliequivalenceCausticPotashPerGramProduct,
    /// kilopascal
    ///
    /// rec20
    Kilopascal,
    /// kilogram of potassium hydroxide (caustic potash)
    ///
    /// rec20
    KilogramPotassiumHydroxideCausticPotash,
    /// kilogram of potassium oxide
    ///
    /// rec20
    KilogramPotassiumOxide,
    /// kilogram of phosphorus pentoxide (phosphoric anhydride)
    ///
    /// rec20
    KilogramPhosphorusPentoxidePhosphoricAnhydride,
    /// kiloroentgen
    ///
    /// rec20
    Kiloroentgen,
    /// kilogram of substance 90 % dry
    ///
    /// rec20
    KilogramSubstance90Dry,
    /// kilogram of sodium hydroxide (caustic soda)
    ///
    /// rec20
    KilogramSodiumHydroxideCausticSoda,
    /// kit
    ///
    /// rec20
    Kit,
    /// kilotonne
    ///
    /// rec20
    Kilotonne,
    /// kilogram of uranium
    ///
    /// rec20
    KilogramUranium,
    /// kilovolt - ampere
    ///
    /// rec20
    KilovoltAmpere,
    /// kilovar
    ///
    /// rec20
    Kilovar,
    /// kilovolt
    ///
    /// rec20
    Kilovolt,
    /// kilogram per millimetre
    ///
    /// rec20
    KilogramPerMillimetre,
    /// kilowatt hour
    ///
    /// rec20
    KilowattHour,
    /// Kilowatt hour per normalized cubic metre
    ///
    /// rec20
    KilowattHourPerNormalizedCubicMetre,
    /// kilogram of tungsten trioxide
    ///
    /// rec20
    KilogramTungstenTrioxide,
    /// Kilowatt hour per standard cubic metre
    ///
    /// rec20
    KilowattHourPerStandardCubicMetre,
    /// kilowatt
    ///
    /// rec20
    Kilowatt,
    /// kilowatt year
    ///
    /// Rec20
    KilowattYear,
    /// millilitre per kilogram
    ///
    /// rec20
    MillilitrePerKilogram,
    /// quart (US liquid) per minute
    ///
    /// rec20
    QuartUsLiquidPerMinute,
    /// quart (US liquid) per second
    ///
    /// rec20
    QuartUsLiquidPerSecond,
    /// metre per second kelvin
    ///
    /// rec20
    MetrePerSecondKelvin,
    /// metre per second bar
    ///
    /// rec20
    MetrePerSecondBar,
    /// square metre hour degree Celsius per kilocalorie (international table)
    ///
    /// rec20
    SquareMetreHourDegreeCelsiusPerKilocalorieInternationalTable,
    /// millipascal second per kelvin
    ///
    /// rec20
    MillipascalSecondPerKelvin,
    /// millipascal second per bar
    ///
    /// rec20
    MillipascalSecondPerBar,
    /// milligram per cubic metre kelvin
    ///
    /// rec20
    MilligramPerCubicMetreKelvin,
    /// milligram per cubic metre bar
    ///
    /// rec20
    MilligramPerCubicMetreBar,
    /// millilitre per litre
    ///
    /// rec20
    MillilitrePerLitre,
    /// litre per minute
    ///
    /// rec20
    LitrePerMinute,
    /// reciprocal cubic millimetre
    ///
    /// rec20
    ReciprocalCubicMillimetre,
    /// cubic millimetre per cubic metre
    ///
    /// rec20
    CubicMillimetrePerCubicMetre,
    /// mole per hour
    ///
    /// rec20
    MolePerHour,
    /// mole per kilogram kelvin
    ///
    /// rec20
    MolePerKilogramKelvin,
    /// mole per kilogram bar
    ///
    /// rec20
    MolePerKilogramBar,
    /// mole per litre kelvin
    ///
    /// rec20
    MolePerLitreKelvin,
    /// mole per litre bar
    ///
    /// rec20
    MolePerLitreBar,
    /// mole per cubic metre kelvin
    ///
    /// rec20
    MolePerCubicMetreKelvin,
    /// mole per cubic metre bar
    ///
    /// rec20
    MolePerCubicMetreBar,
    /// mole per minute
    ///
    /// rec20
    MolePerMinute,
    /// milliroentgen aequivalent men
    ///
    /// rec20
    MilliroentgenAequivalentMen,
    /// nanogram per kilogram
    ///
    /// rec20
    NanogramPerKilogram,
    /// ounce (avoirdupois) per day
    ///
    /// rec20
    OunceAvoirdupoisPerDay,
    /// ounce (avoirdupois) per hour
    ///
    /// rec20
    OunceAvoirdupoisPerHour,
    /// ounce (avoirdupois) per minute
    ///
    /// rec20
    OunceAvoirdupoisPerMinute,
    /// ounce (avoirdupois) per second
    ///
    /// rec20
    OunceAvoirdupoisPerSecond,
    /// ounce (avoirdupois) per gallon (UK)
    ///
    /// rec20
    OunceAvoirdupoisPerGallonUk,
    /// ounce (avoirdupois) per gallon (US)
    ///
    /// rec20
    OunceAvoirdupoisPerGallonUs,
    /// ounce (avoirdupois) per cubic inch
    ///
    /// rec20
    OunceAvoirdupoisPerCubicInch,
    /// ounce (avoirdupois)-force
    ///
    /// rec20
    OunceAvoirdupoisForce,
    /// ounce (avoirdupois)-force inch
    ///
    /// rec20
    OunceAvoirdupoisForceInch,
    /// picosiemens per metre
    ///
    /// rec20
    PicosiemensPerMetre,
    /// peck (UK)
    ///
    /// rec20
    PeckUk,
    /// peck (UK) per day
    ///
    /// rec20
    PeckUkPerDay,
    /// peck (UK) per hour
    ///
    /// rec20
    PeckUkPerHour,
    /// peck (UK) per minute
    ///
    /// rec20
    PeckUkPerMinute,
    /// peck (UK) per second
    ///
    /// rec20
    PeckUkPerSecond,
    /// peck (US dry) per day
    ///
    /// rec20
    PeckUsDryPerDay,
    /// peck (US dry) per hour
    ///
    /// rec20
    PeckUsDryPerHour,
    /// peck (US dry) per minute
    ///
    /// rec20
    PeckUsDryPerMinute,
    /// peck (US dry) per second
    ///
    /// rec20
    PeckUsDryPerSecond,
    /// psi per psi
    ///
    /// rec20
    PsiPerPsi,
    /// pint (UK) per day
    ///
    /// rec20
    PintUkPerDay,
    /// pint (UK) per hour
    ///
    /// rec20
    PintUkPerHour,
    /// pint (UK) per minute
    ///
    /// rec20
    PintUkPerMinute,
    /// pint (UK) per second
    ///
    /// rec20
    PintUkPerSecond,
    /// pint (US liquid) per day
    ///
    /// rec20
    PintUsLiquidPerDay,
    /// pint (US liquid) per hour
    ///
    /// rec20
    PintUsLiquidPerHour,
    /// pint (US liquid) per minute
    ///
    /// rec20
    PintUsLiquidPerMinute,
    /// pint (US liquid) per second
    ///
    /// rec20
    PintUsLiquidPerSecond,
    /// slug per day
    ///
    /// rec20
    SlugPerDay,
    /// slug per foot second
    ///
    /// rec20
    SlugPerFootSecond,
    /// slug per cubic foot
    ///
    /// rec20
    SlugPerCubicFoot,
    /// slug per hour
    ///
    /// rec20
    SlugPerHour,
    /// slug per minute
    ///
    /// rec20
    SlugPerMinute,
    /// slug per second
    ///
    /// rec20
    SlugPerSecond,
    /// tonne per kelvin
    ///
    /// rec20
    TonnePerKelvin,
    /// tonne per bar
    ///
    /// rec20
    TonnePerBar,
    /// tonne per day
    ///
    /// rec20
    TonnePerDay,
    /// tonne per day kelvin
    ///
    /// rec20
    TonnePerDayKelvin,
    /// tonne per day bar
    ///
    /// rec20
    TonnePerDayBar,
    /// tonne per hour kelvin
    ///
    /// rec20
    TonnePerHourKelvin,
    /// tonne per hour bar
    ///
    /// rec20
    TonnePerHourBar,
    /// tonne per cubic metre kelvin
    ///
    /// rec20
    TonnePerCubicMetreKelvin,
    /// tonne per cubic metre bar
    ///
    /// rec20
    TonnePerCubicMetreBar,
    /// tonne per minute
    ///
    /// rec20
    TonnePerMinute,
    /// tonne per minute kelvin
    ///
    /// rec20
    TonnePerMinuteKelvin,
    /// tonne per minute bar
    ///
    /// rec20
    TonnePerMinuteBar,
    /// tonne per second
    ///
    /// rec20
    TonnePerSecond,
    /// tonne per second kelvin
    ///
    /// rec20
    TonnePerSecondKelvin,
    /// tonne per second bar
    ///
    /// rec20
    TonnePerSecondBar,
    /// ton (UK shipping)
    ///
    /// rec20
    TonUkShipping,
    /// ton long per day
    ///
    /// rec20
    TonLongPerDay,
    /// ton (US shipping)
    ///
    /// rec20
    TonUsShipping,
    /// ton short per degree Fahrenheit
    ///
    /// rec20
    TonShortPerDegreeFahrenheit,
    /// ton short per day
    ///
    /// rec20
    TonShortPerDay,
    /// ton short per hour degree Fahrenheit
    ///
    /// rec20
    TonShortPerHourDegreeFahrenheit,
    /// ton short per hour psi
    ///
    /// rec20
    TonShortPerHourPsi,
    /// ton short per psi
    ///
    /// rec20
    TonShortPerPsi,
    /// ton (UK long) per cubic yard
    ///
    /// rec20
    TonUkLongPerCubicYard,
    /// ton (US short) per cubic yard
    ///
    /// rec20
    TonUsShortPerCubicYard,
    /// ton-force (US short)
    ///
    /// rec20
    TonForceUsShort,
    /// common year
    ///
    /// rec20
    CommonYear,
    /// sidereal year
    ///
    /// rec20
    SiderealYear,
    /// yard per degree Fahrenheit
    ///
    /// rec20
    YardPerDegreeFahrenheit,
    /// yard per psi
    ///
    /// rec20
    YardPerPsi,
    /// pound per cubic inch
    ///
    /// rec20
    PoundPerCubicInch,
    /// lactose excess percentage
    ///
    /// rec20
    LactoseExcessPercentage,
    /// pound
    ///
    /// rec20
    Pound,
    /// troy pound (US)
    ///
    /// rec20
    TroyPoundUs,
    /// litre per day
    ///
    /// rec20
    LitrePerDay,
    /// leaf
    ///
    /// rec20
    Leaf,
    /// linear foot
    ///
    /// rec20
    LinearFoot,
    /// labour hour
    ///
    /// rec20
    LabourHour,
    /// link
    ///
    /// rec20
    Link,
    /// linear metre
    ///
    /// rec20
    LinearMetre,
    /// length
    ///
    /// rec20
    Length,
    /// lot [unit of procurement]
    ///
    /// rec20
    LotUnitProcurement,
    /// liquid pound
    ///
    /// rec20
    LiquidPound,
    /// litre of pure alcohol
    ///
    /// rec20
    LitrePureAlcohol,
    /// layer
    ///
    /// rec20
    Layer,
    /// lump sum
    ///
    /// rec20
    LumpSum,
    /// ton (UK) or long ton (US)
    ///
    /// rec20
    TonUkOrLongTonUs,
    /// litre
    ///
    /// rec20
    Litre,
    /// metric ton, lubricating oil
    ///
    /// rec20
    MetricTonLubricatingOil,
    /// lumen
    ///
    /// rec20
    Lumen,
    /// lux
    ///
    /// rec20
    Lux,
    /// linear yard
    ///
    /// rec20
    LinearYard,
    /// milligram per litre
    ///
    /// rec20
    MilligramPerLitre,
    /// reciprocal cubic yard
    ///
    /// rec20
    ReciprocalCubicYard,
    /// cubic yard per degree Fahrenheit
    ///
    /// rec20
    CubicYardPerDegreeFahrenheit,
    /// cubic yard per day
    ///
    /// rec20
    CubicYardPerDay,
    /// cubic yard per hour
    ///
    /// rec20
    CubicYardPerHour,
    /// cubic yard per psi
    ///
    /// rec20
    CubicYardPerPsi,
    /// cubic yard per minute
    ///
    /// rec20
    CubicYardPerMinute,
    /// cubic yard per second
    ///
    /// rec20
    CubicYardPerSecond,
    /// kilohertz metre
    ///
    /// rec20
    KilohertzMetre,
    /// gigahertz metre
    ///
    /// rec20
    GigahertzMetre,
    /// Beaufort
    ///
    /// rec20
    Beaufort,
    /// reciprocal megakelvin or megakelvin to the power minus one
    ///
    /// rec20
    ReciprocalMegakelvinOrMegakelvinToPowerMinusOne,
    /// reciprocal kilovolt - ampere reciprocal hour
    ///
    /// rec20
    ReciprocalKilovoltAmpereReciprocalHour,
    /// millilitre per square centimetre minute
    ///
    /// rec20
    MillilitrePerSquareCentimetreMinute,
    /// newton per centimetre
    ///
    /// rec20
    NewtonPerCentimetre,
    /// ohm kilometre
    ///
    /// rec20
    OhmKilometre,
    /// percent per degree Celsius
    ///
    /// rec20
    PercentPerDegreeCelsius,
    /// gigaohm per metre
    ///
    /// rec20
    GigaohmPerMetre,
    /// megahertz metre
    ///
    /// rec20
    MegahertzMetre,
    /// kilogram per kilogram
    ///
    /// rec20
    KilogramPerKilogram,
    /// reciprocal volt - ampere reciprocal second
    ///
    /// rec20
    ReciprocalVoltAmpereReciprocalSecond,
    /// kilogram per kilometre
    ///
    /// rec20
    KilogramPerKilometre,
    /// pascal second per litre
    ///
    /// rec20
    PascalSecondPerLitre,
    /// millimole per litre
    ///
    /// rec20
    MillimolePerLitre,
    /// newton metre per square metre
    ///
    /// rec20
    NewtonMetrePerSquareMetre,
    /// millivolt - ampere
    ///
    /// rec20
    MillivoltAmpere,
    /// 30-day month
    ///
    /// rec20
    _30DayMonth,
    /// actual/360
    ///
    /// rec20
    Actual360,
    /// kilometre per second squared
    ///
    /// rec20
    KilometrePerSecondSquared,
    /// centimetre per second squared
    ///
    /// rec20
    CentimetrePerSecondSquared,
    /// monetary value
    ///
    /// rec20
    MonetaryValue,
    /// yard per second squared
    ///
    /// rec20
    YardPerSecondSquared,
    /// millimetre per second squared
    ///
    /// rec20
    MillimetrePerSecondSquared,
    /// mile (statute mile) per second squared
    ///
    /// rec20
    MileStatuteMilePerSecondSquared,
    /// mil
    ///
    /// rec20
    Mil,
    /// revolution
    ///
    /// rec20
    Revolution,
    /// degree [unit of angle] per second squared
    ///
    /// rec20
    DegreeUnitAnglePerSecondSquared,
    /// revolution per minute
    ///
    /// rec20
    RevolutionPerMinute,
    /// circular mil
    ///
    /// rec20
    CircularMil,
    /// square mile (based on U.S. survey foot)
    ///
    /// rec20
    SquareMileBasedOnUSSurveyFoot,
    /// chain (based on U.S. survey foot)
    ///
    /// rec20
    ChainBasedOnUSSurveyFoot,
    /// microcurie
    ///
    /// rec20
    Microcurie,
    /// furlong
    ///
    /// rec20
    Furlong,
    /// foot (U.S. survey)
    ///
    /// rec20
    FootUSSurvey,
    /// mile (based on U.S. survey foot)
    ///
    /// rec20
    MileBasedOnUSSurveyFoot,
    /// metre per pascal
    ///
    /// rec20
    MetrePerPascal,
    /// metre per radiant
    ///
    /// rec20
    MetrePerRadiant,
    /// shake
    ///
    /// rec20
    Shake,
    /// mile per minute
    ///
    /// rec20
    MilePerMinute,
    /// mile per second
    ///
    /// rec20
    MilePerSecond,
    /// metre per second pascal
    ///
    /// rec20
    MetrePerSecondPascal,
    /// metre per hour
    ///
    /// rec20
    MetrePerHour,
    /// inch per year
    ///
    /// rec20
    InchPerYear,
    /// kilometre per second
    ///
    /// rec20
    KilometrePerSecond,
    /// inch per minute
    ///
    /// rec20
    InchPerMinute,
    /// yard per second
    ///
    /// rec20
    YardPerSecond,
    /// yard per minute
    ///
    /// rec20
    YardPerMinute,
    /// yard per hour
    ///
    /// rec20
    YardPerHour,
    /// acre-foot (based on U.S. survey foot)
    ///
    /// rec20
    AcreFootBasedOnUSSurveyFoot,
    /// cord (128 ft3)
    ///
    /// rec20
    Cord128Ft3,
    /// cubic mile (UK statute)
    ///
    /// rec20
    CubicMileUkStatute,
    /// micro-inch
    ///
    /// rec20
    MicroInch,
    /// ton, register
    ///
    /// rec20
    TonRegister,
    /// cubic metre per pascal
    ///
    /// rec20
    CubicMetrePerPascal,
    /// bel
    ///
    /// rec20
    Bel,
    /// kilogram per cubic metre pascal
    ///
    /// rec20
    KilogramPerCubicMetrePascal,
    /// kilogram per pascal
    ///
    /// rec20
    KilogramPerPascal,
    /// kilopound-force
    ///
    /// rec20
    KilopoundForce,
    /// poundal
    ///
    /// rec20
    Poundal,
    /// kilogram metre per second squared
    ///
    /// rec20
    KilogramMetrePerSecondSquared,
    /// pond
    ///
    /// rec20
    Pond,
    /// square foot per hour
    ///
    /// rec20
    SquareFootPerHour,
    /// stokes per pascal
    ///
    /// rec20
    StokesPerPascal,
    /// square centimetre per second
    ///
    /// rec20
    SquareCentimetrePerSecond,
    /// square metre per second pascal
    ///
    /// rec20
    SquareMetrePerSecondPascal,
    /// denier
    ///
    /// rec20
    Denier_Dup,
    /// pound per yard
    ///
    /// rec20
    PoundPerYard,
    /// ton, assay
    ///
    /// rec20
    TonAssay,
    /// pfund
    ///
    /// rec20
    Pfund,
    /// kilogram per second pascal
    ///
    /// rec20
    KilogramPerSecondPascal,
    /// tonne per month
    ///
    /// rec20
    TonnePerMonth,
    /// tonne per year
    ///
    /// rec20
    TonnePerYear,
    /// million Btu per 1000 cubic foot
    ///
    /// rec20
    MillionBtuPer1000CubicFoot,
    /// kilopound per hour
    ///
    /// rec20
    KilopoundPerHour,
    /// pound per pound
    ///
    /// rec20
    PoundPerPound,
    /// pound-force foot
    ///
    /// rec20
    PoundForceFoot,
    /// newton metre per radian
    ///
    /// rec20
    NewtonMetrePerRadian,
    /// kilogram metre
    ///
    /// rec20
    KilogramMetre,
    /// poundal foot
    ///
    /// rec20
    PoundalFoot,
    /// poundal inch
    ///
    /// rec20
    PoundalInch,
    /// dyne metre
    ///
    /// rec20
    DyneMetre,
    /// kilogram centimetre per second
    ///
    /// rec20
    KilogramCentimetrePerSecond,
    /// gram centimetre per second
    ///
    /// rec20
    GramCentimetrePerSecond,
    /// megavolt ampere reactive hour
    ///
    /// rec20
    MegavoltAmpereReactiveHour,
    /// megalitre
    ///
    /// rec20
    Megalitre,
    /// megametre
    ///
    /// rec20
    Megametre,
    /// megavar
    ///
    /// rec20
    Megavar,
    /// megawatt
    ///
    /// rec20
    Megawatt,
    /// thousand standard brick equivalent
    ///
    /// rec20
    ThousandStandardBrickEquivalent,
    /// thousand board foot
    ///
    /// rec20
    ThousandBoardFoot,
    /// millibar
    ///
    /// rec20
    Millibar,
    /// microgram
    ///
    /// rec20
    Microgram,
    /// millicurie
    ///
    /// rec20
    Millicurie,
    /// air dry metric ton
    ///
    /// rec20
    AirDryMetricTon,
    /// milligram
    ///
    /// rec20
    Milligram,
    /// megahertz
    ///
    /// rec20
    Megahertz,
    /// square mile (statute mile)
    ///
    /// rec20
    SquareMileStatuteMile,
    /// thousand
    ///
    /// rec20
    Thousand,
    /// minute [unit of time]
    ///
    /// rec20
    MinuteUnitTime,
    /// million
    ///
    /// rec20
    Million,
    /// million international unit
    ///
    /// rec20
    MillionInternationalUnit,
    /// Square Metre Day
    ///
    /// Rec20
    SquareMetreDay,
    /// Square Metre Month
    ///
    /// Rec20
    SquareMetreMonth,
    /// Square Metre Week
    ///
    /// Rec20
    SquareMetreWeek,
    /// milliard
    ///
    /// rec20
    Milliard,
    /// millilitre
    ///
    /// rec20
    Millilitre,
    /// square millimetre
    ///
    /// rec20
    SquareMillimetre,
    /// cubic millimetre
    ///
    /// rec20
    CubicMillimetre,
    /// millimetre
    ///
    /// rec20
    Millimetre,
    /// kilogram, dry weight
    ///
    /// rec20
    KilogramDryWeight,
    /// Mega Joule per Normalised cubic Metre
    ///
    /// Rec20
    MegaJoulePerNormalisedCubicMetre,
    /// month
    ///
    /// rec20
    Month,
    /// megapascal
    ///
    /// rec20
    Megapascal,
    /// Cubic Metre Day
    ///
    /// Rec20
    CubicMetreDay,
    /// cubic metre per hour
    ///
    /// rec20
    CubicMetrePerHour,
    /// Cubic Metre Month
    ///
    /// Rec20
    CubicMetreMonth,
    /// cubic metre per second
    ///
    /// rec20
    CubicMetrePerSecond,
    /// Cubic Metre Week
    ///
    /// Rec20
    CubicMetreWeek,
    /// Metre Day
    ///
    /// Rec20
    MetreDay,
    /// Metre Month
    ///
    /// Rec20
    MetreMonth,
    /// Metre Week
    ///
    /// Rec20
    MetreWeek,
    /// metre per second squared
    ///
    /// rec20
    MetrePerSecondSquared,
    /// square metre
    ///
    /// rec20
    SquareMetre,
    /// cubic metre
    ///
    /// rec20
    CubicMetre,
    /// metre
    ///
    /// rec20
    Metre,
    /// metre per second
    ///
    /// rec20
    MetrePerSecond,
    /// milihertz
    ///
    /// Rec20
    Milihertz,
    /// megavolt - ampere
    ///
    /// rec20
    MegavoltAmpere,
    /// megawatt hour (1000 kW.h)
    ///
    /// rec20
    MegawattHour1000KwH,
    /// pen calorie
    ///
    /// rec20
    PenCalorie,
    /// pound foot per second
    ///
    /// rec20
    PoundFootPerSecond,
    /// pound inch per second
    ///
    /// rec20
    PoundInchPerSecond,
    /// Pferdestaerke
    ///
    /// rec20
    Pferdestaerke,
    /// centimetre of mercury (0 ºC)
    ///
    /// rec20
    CentimetreMercury0Oc,
    /// centimetre of water (4 ºC)
    ///
    /// rec20
    CentimetreWater4Oc,
    /// foot of water (39.2 ºF)
    ///
    /// rec20
    FootWater392Of,
    /// inch of mercury (32 ºF)
    ///
    /// rec20
    InchMercury32Of,
    /// inch of mercury (60 ºF)
    ///
    /// rec20
    InchMercury60Of,
    /// inch of water (39.2 ºF)
    ///
    /// rec20
    InchWater392Of,
    /// inch of water (60 ºF)
    ///
    /// rec20
    InchWater60Of,
    /// kip per square inch
    ///
    /// rec20
    KipPerSquareInch,
    /// poundal per square foot
    ///
    /// rec20
    PoundalPerSquareFoot,
    /// ounce (avoirdupois) per square inch
    ///
    /// rec20
    OunceAvoirdupoisPerSquareInch,
    /// conventional metre of water
    ///
    /// rec20
    ConventionalMetreWater,
    /// gram per square millimetre
    ///
    /// rec20
    GramPerSquareMillimetre,
    /// pound per square yard
    ///
    /// rec20
    PoundPerSquareYard,
    /// poundal per square inch
    ///
    /// rec20
    PoundalPerSquareInch,
    /// foot to the fourth power
    ///
    /// rec20
    FootToFourthPower,
    /// cubic decimetre per kilogram
    ///
    /// rec20
    CubicDecimetrePerKilogram,
    /// cubic foot per pound
    ///
    /// rec20
    CubicFootPerPound,
    /// print point
    ///
    /// rec20
    PrintPoint,
    /// cubic inch per pound
    ///
    /// rec20
    CubicInchPerPound,
    /// kilonewton per metre
    ///
    /// rec20
    KilonewtonPerMetre,
    /// poundal per inch
    ///
    /// rec20
    PoundalPerInch,
    /// pound-force per yard
    ///
    /// rec20
    PoundForcePerYard,
    /// poundal second per square foot
    ///
    /// rec20
    PoundalSecondPerSquareFoot,
    /// poise per pascal
    ///
    /// rec20
    PoisePerPascal,
    /// newton second per square metre
    ///
    /// rec20
    NewtonSecondPerSquareMetre,
    /// kilogram per metre second
    ///
    /// rec20
    KilogramPerMetreSecond,
    /// kilogram per metre minute
    ///
    /// rec20
    KilogramPerMetreMinute,
    /// kilogram per metre day
    ///
    /// rec20
    KilogramPerMetreDay,
    /// kilogram per metre hour
    ///
    /// rec20
    KilogramPerMetreHour,
    /// gram per centimetre second
    ///
    /// rec20
    GramPerCentimetreSecond,
    /// poundal second per square inch
    ///
    /// rec20
    PoundalSecondPerSquareInch,
    /// pound per foot minute
    ///
    /// rec20
    PoundPerFootMinute,
    /// pound per foot day
    ///
    /// rec20
    PoundPerFootDay,
    /// cubic metre per second pascal
    ///
    /// rec20
    CubicMetrePerSecondPascal,
    /// foot poundal
    ///
    /// rec20
    FootPoundal,
    /// inch poundal
    ///
    /// rec20
    InchPoundal,
    /// watt per square centimetre
    ///
    /// rec20
    WattPerSquareCentimetre,
    /// watt per square inch
    ///
    /// rec20
    WattPerSquareInch,
    /// British thermal unit (international table) per square foot hour
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerSquareFootHour,
    /// British thermal unit (thermochemical) per square foot hour
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerSquareFootHour,
    /// British thermal unit (thermochemical) per square foot minute
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerSquareFootMinute,
    /// British thermal unit (international table) per square foot second
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerSquareFootSecond,
    /// British thermal unit (thermochemical) per square foot second
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerSquareFootSecond,
    /// British thermal unit (international table) per square inch second
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerSquareInchSecond,
    /// calorie (thermochemical) per square centimetre minute
    ///
    /// rec20
    CalorieThermochemicalPerSquareCentimetreMinute,
    /// calorie (thermochemical) per square centimetre second
    ///
    /// rec20
    CalorieThermochemicalPerSquareCentimetreSecond,
    /// British thermal unit (international table) per cubic foot
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerCubicFoot,
    /// British thermal unit (thermochemical) per cubic foot
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerCubicFoot,
    /// British thermal unit (international table) per degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerDegreeFahrenheit,
    /// British thermal unit (thermochemical) per degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerDegreeFahrenheit,
    /// British thermal unit (international table) per degree Rankine
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerDegreeRankine,
    /// British thermal unit (thermochemical) per degree Rankine
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerDegreeRankine,
    /// British thermal unit (thermochemical) per pound degree Rankine
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerPoundDegreeRankine,
    /// kilocalorie (international table) per gram kelvin
    ///
    /// rec20
    KilocalorieInternationalTablePerGramKelvin,
    /// British thermal unit (39 ºF)
    ///
    /// rec20
    BritishThermalUnit39Of,
    /// British thermal unit (59 ºF)
    ///
    /// rec20
    BritishThermalUnit59Of,
    /// British thermal unit (60 ºF)
    ///
    /// rec20
    BritishThermalUnit60Of,
    /// calorie (20 ºC)
    ///
    /// rec20
    Calorie20Oc,
    /// quad (1015 BtuIT)
    ///
    /// rec20
    Quad1015Btuit,
    /// therm (EC)
    ///
    /// rec20
    ThermEc,
    /// therm (U.S.)
    ///
    /// rec20
    ThermUS,
    /// British thermal unit (thermochemical) per pound
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerPound,
    /// British thermal unit (international table) per hour square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerHourSquareFootDegreeFahrenheit,
    /// British thermal unit (thermochemical) per hour square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerHourSquareFootDegreeFahrenheit,
    /// British thermal unit (international table) per second square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerSecondSquareFootDegreeFahrenheit,
    /// British thermal unit (thermochemical) per second square foot degree Fahrenheit
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerSecondSquareFootDegreeFahrenheit,
    /// kilowatt per square metre kelvin
    ///
    /// rec20
    KilowattPerSquareMetreKelvin,
    /// kelvin per pascal
    ///
    /// rec20
    KelvinPerPascal,
    /// watt per metre degree Celsius
    ///
    /// rec20
    WattPerMetreDegreeCelsius,
    /// kilowatt per metre kelvin
    ///
    /// rec20
    KilowattPerMetreKelvin,
    /// kilowatt per metre degree Celsius
    ///
    /// rec20
    KilowattPerMetreDegreeCelsius,
    /// metre per degree Celcius metre
    ///
    /// rec20
    MetrePerDegreeCelciusMetre,
    /// degree Fahrenheit hour per British thermal unit (international table)
    ///
    /// rec20
    DegreeFahrenheitHourPerBritishThermalUnitInternationalTable,
    /// degree Fahrenheit hour per British thermal unit (thermochemical)
    ///
    /// rec20
    DegreeFahrenheitHourPerBritishThermalUnitThermochemical,
    /// degree Fahrenheit second per British thermal unit (international table)
    ///
    /// rec20
    DegreeFahrenheitSecondPerBritishThermalUnitInternationalTable,
    /// degree Fahrenheit second per British thermal unit (thermochemical)
    ///
    /// rec20
    DegreeFahrenheitSecondPerBritishThermalUnitThermochemical,
    /// degree Fahrenheit hour square foot per British thermal unit (international table) inch
    ///
    /// rec20
    DegreeFahrenheitHourSquareFootPerBritishThermalUnitInternationalTableInch,
    /// degree Fahrenheit hour square foot per British thermal unit (thermochemical) inch
    ///
    /// rec20
    DegreeFahrenheitHourSquareFootPerBritishThermalUnitThermochemicalInch,
    /// kilofarad
    ///
    /// rec20
    Kilofarad,
    /// reciprocal joule
    ///
    /// rec20
    ReciprocalJoule,
    /// picosiemens
    ///
    /// rec20
    Picosiemens,
    /// ampere per pascal
    ///
    /// rec20
    AmperePerPascal,
    /// franklin
    ///
    /// rec20
    Franklin,
    /// ampere minute
    ///
    /// rec20
    AmpereMinute,
    /// biot
    ///
    /// rec20
    Biot,
    /// gilbert
    ///
    /// rec20
    Gilbert,
    /// volt per pascal
    ///
    /// rec20
    VoltPerPascal,
    /// picovolt
    ///
    /// rec20
    Picovolt,
    /// milligram per kilogram
    ///
    /// rec20
    MilligramPerKilogram,
    /// number of articles
    ///
    /// rec20
    NumberArticles,
    /// number of cells
    ///
    /// rec20
    NumberCells,
    /// newton
    ///
    /// rec20
    Newton,
    /// message
    ///
    /// rec20
    Message,
    /// nil
    ///
    /// rec20
    Nil,
    /// number of international units
    ///
    /// rec20
    NumberInternationalUnits,
    /// load
    ///
    /// rec20
    Load,
    /// Normalised cubic metre
    ///
    /// rec20
    NormalisedCubicMetre,
    /// nautical mile
    ///
    /// rec20
    NauticalMile,
    /// number of packs
    ///
    /// rec20
    NumberPacks,
    /// number of parts
    ///
    /// rec20
    NumberParts,
    /// net ton
    ///
    /// rec20
    NetTon,
    /// Nephelometric turbidity unit
    ///
    /// Rec20
    NephelometricTurbidityUnit,
    /// newton metre
    ///
    /// rec20
    NewtonMetre,
    /// part per thousand
    ///
    /// rec20
    PartPerThousand,
    /// panel
    ///
    /// rec20
    Panel,
    /// ozone depletion equivalent
    ///
    /// rec20
    OzoneDepletionEquivalent,
    /// ODS Grams
    ///
    /// rec20
    OdsGrams,
    /// ODS Kilograms
    ///
    /// rec20
    OdsKilograms,
    /// ODS Milligrams
    ///
    /// rec20
    OdsMilligrams,
    /// ohm
    ///
    /// rec20
    Ohm,
    /// ounce per square yard
    ///
    /// rec20
    OuncePerSquareYard,
    /// ounce (avoirdupois)
    ///
    /// rec20
    OunceAvoirdupois,
    /// oscillations per minute
    ///
    /// rec20
    OscillationsPerMinute,
    /// overtime hour
    ///
    /// rec20
    OvertimeHour,
    /// fluid ounce (US)
    ///
    /// rec20
    FluidOunceUs,
    /// fluid ounce (UK)
    ///
    /// rec20
    FluidOunceUk,
    /// percent
    ///
    /// rec20
    Percent,
    /// coulomb per metre
    ///
    /// rec20
    CoulombPerMetre,
    /// kiloweber
    ///
    /// rec20
    Kiloweber,
    /// gamma
    ///
    /// rec20
    Gamma,
    /// kilotesla
    ///
    /// rec20
    Kilotesla,
    /// joule per second
    ///
    /// rec20
    JoulePerSecond,
    /// joule per minute
    ///
    /// rec20
    JoulePerMinute,
    /// joule per hour
    ///
    /// rec20
    JoulePerHour,
    /// joule per day
    ///
    /// rec20
    JoulePerDay,
    /// kilojoule per second
    ///
    /// rec20
    KilojoulePerSecond,
    /// kilojoule per minute
    ///
    /// rec20
    KilojoulePerMinute,
    /// pound per foot
    ///
    /// rec20
    PoundPerFoot,
    /// kilojoule per hour
    ///
    /// rec20
    KilojoulePerHour,
    /// kilojoule per day
    ///
    /// rec20
    KilojoulePerDay,
    /// nanoohm
    ///
    /// rec20
    Nanoohm,
    /// ohm circular-mil per foot
    ///
    /// rec20
    OhmCircularMilPerFoot,
    /// kilohenry
    ///
    /// rec20
    Kilohenry,
    /// lumen per square foot
    ///
    /// rec20
    LumenPerSquareFoot,
    /// phot
    ///
    /// rec20
    Phot,
    /// footcandle
    ///
    /// rec20
    Footcandle,
    /// candela per square inch
    ///
    /// rec20
    CandelaPerSquareInch,
    /// footlambert
    ///
    /// rec20
    Footlambert,
    /// lambert
    ///
    /// rec20
    Lambert,
    /// stilb
    ///
    /// rec20
    Stilb,
    /// candela per square foot
    ///
    /// rec20
    CandelaPerSquareFoot,
    /// kilocandela
    ///
    /// rec20
    Kilocandela,
    /// millicandela
    ///
    /// rec20
    Millicandela,
    /// Hefner-Kerze
    ///
    /// rec20
    HefnerKerze,
    /// international candle
    ///
    /// rec20
    InternationalCandle,
    /// British thermal unit (international table) per square foot
    ///
    /// rec20
    BritishThermalUnitInternationalTablePerSquareFoot,
    /// British thermal unit (thermochemical) per square foot
    ///
    /// rec20
    BritishThermalUnitThermochemicalPerSquareFoot,
    /// calorie (thermochemical) per square centimetre
    ///
    /// rec20
    CalorieThermochemicalPerSquareCentimetre,
    /// langley
    ///
    /// rec20
    Langley,
    /// decade (logarithmic)
    ///
    /// rec20
    DecadeLogarithmic,
    /// pascal squared second
    ///
    /// rec20
    PascalSquaredSecond,
    /// bel per metre
    ///
    /// rec20
    BelPerMetre,
    /// pound mole
    ///
    /// rec20
    PoundMole,
    /// pound mole per second
    ///
    /// rec20
    PoundMolePerSecond,
    /// pound mole per minute
    ///
    /// rec20
    PoundMolePerMinute,
    /// kilomole per kilogram
    ///
    /// rec20
    KilomolePerKilogram,
    /// pound mole per pound
    ///
    /// rec20
    PoundMolePerPound,
    /// newton square metre per ampere
    ///
    /// rec20
    NewtonSquareMetrePerAmpere,
    /// five pack
    ///
    /// rec20
    FivePack,
    /// weber metre
    ///
    /// rec20
    WeberMetre,
    /// mol per kilogram pascal
    ///
    /// rec20
    MolPerKilogramPascal,
    /// mol per cubic metre pascal
    ///
    /// rec20
    MolPerCubicMetrePascal,
    /// unit pole
    ///
    /// rec20
    UnitPole,
    /// milligray per second
    ///
    /// rec20
    MilligrayPerSecond,
    /// microgray per second
    ///
    /// rec20
    MicrograyPerSecond,
    /// nanogray per second
    ///
    /// rec20
    NanograyPerSecond,
    /// gray per minute
    ///
    /// rec20
    GrayPerMinute,
    /// milligray per minute
    ///
    /// rec20
    MilligrayPerMinute,
    /// microgray per minute
    ///
    /// rec20
    MicrograyPerMinute,
    /// nanogray per minute
    ///
    /// rec20
    NanograyPerMinute,
    /// gray per hour
    ///
    /// rec20
    GrayPerHour,
    /// milligray per hour
    ///
    /// rec20
    MilligrayPerHour,
    /// microgray per hour
    ///
    /// rec20
    MicrograyPerHour,
    /// nanogray per hour
    ///
    /// rec20
    NanograyPerHour,
    /// sievert per second
    ///
    /// rec20
    SievertPerSecond,
    /// millisievert per second
    ///
    /// rec20
    MillisievertPerSecond,
    /// microsievert per second
    ///
    /// rec20
    MicrosievertPerSecond,
    /// nanosievert per second
    ///
    /// rec20
    NanosievertPerSecond,
    /// rem per second
    ///
    /// rec20
    RemPerSecond,
    /// sievert per hour
    ///
    /// rec20
    SievertPerHour,
    /// millisievert per hour
    ///
    /// rec20
    MillisievertPerHour,
    /// microsievert per hour
    ///
    /// rec20
    MicrosievertPerHour,
    /// nanosievert per hour
    ///
    /// rec20
    NanosievertPerHour,
    /// sievert per minute
    ///
    /// rec20
    SievertPerMinute,
    /// millisievert per minute
    ///
    /// rec20
    MillisievertPerMinute,
    /// microsievert per minute
    ///
    /// rec20
    MicrosievertPerMinute,
    /// nanosievert per minute
    ///
    /// rec20
    NanosievertPerMinute,
    /// reciprocal square inch
    ///
    /// rec20
    ReciprocalSquareInch,
    /// pascal square metre per kilogram
    ///
    /// rec20
    PascalSquareMetrePerKilogram,
    /// millipascal per metre
    ///
    /// rec20
    MillipascalPerMetre,
    /// kilopascal per metre
    ///
    /// rec20
    KilopascalPerMetre,
    /// hectopascal per metre
    ///
    /// rec20
    HectopascalPerMetre,
    /// standard atmosphere per metre
    ///
    /// rec20
    StandardAtmospherePerMetre,
    /// technical atmosphere per metre
    ///
    /// rec20
    TechnicalAtmospherePerMetre,
    /// torr per metre
    ///
    /// rec20
    TorrPerMetre,
    /// psi per inch
    ///
    /// rec20
    PsiPerInch,
    /// cubic metre per second square metre
    ///
    /// rec20
    CubicMetrePerSecondSquareMetre,
    /// rhe
    ///
    /// rec20
    Rhe,
    /// pound-force foot per inch
    ///
    /// rec20
    PoundForceFootPerInch,
    /// pound-force inch per inch
    ///
    /// rec20
    PoundForceInchPerInch,
    /// perm (0 ºC)
    ///
    /// rec20
    Perm0Oc,
    /// perm (23 ºC)
    ///
    /// rec20
    Perm23Oc,
    /// byte per second
    ///
    /// rec20
    BytePerSecond,
    /// kilobyte per second
    ///
    /// rec20
    KilobytePerSecond,
    /// megabyte per second
    ///
    /// rec20
    MegabytePerSecond,
    /// reciprocal volt
    ///
    /// rec20
    ReciprocalVolt,
    /// reciprocal radian
    ///
    /// rec20
    ReciprocalRadian,
    /// pascal to the power sum of stoichiometric numbers
    ///
    /// rec20
    PascalToPowerSumStoichiometricNumbers,
    /// mole per cubiv metre to the power sum of stoichiometric numbers
    ///
    /// rec20
    MolePerCubivMetreToPowerSumStoichiometricNumbers,
    /// pascal
    ///
    /// rec20
    Pascal,
    /// pad
    ///
    /// rec20
    Pad,
    /// proof litre
    ///
    /// rec20
    ProofLitre,
    /// proof gallon
    ///
    /// rec20
    ProofGallon,
    /// pitch
    ///
    /// rec20
    Pitch,
    /// degree Plato
    ///
    /// rec20
    DegreePlato,
    /// pound per inch of length
    ///
    /// rec20
    PoundPerInchLength,
    /// page per inch
    ///
    /// rec20
    PagePerInch,
    /// pair
    ///
    /// rec20
    Pair,
    /// pound-force per square inch
    ///
    /// rec20
    PoundForcePerSquareInch,
    /// dry pint (US)
    ///
    /// rec20
    DryPintUs,
    /// pint (UK)
    ///
    /// rec20
    PintUk,
    /// liquid pint (US)
    ///
    /// rec20
    LiquidPintUs,
    /// portion
    ///
    /// rec20
    Portion,
    /// joule per tesla
    ///
    /// rec20
    JoulePerTesla,
    /// erlang
    ///
    /// rec20
    Erlang,
    /// octet
    ///
    /// rec20
    Octet,
    /// octet per second
    ///
    /// rec20
    OctetPerSecond,
    /// shannon
    ///
    /// rec20
    Shannon,
    /// hartley
    ///
    /// rec20
    Hartley,
    /// natural unit of information
    ///
    /// rec20
    NaturalUnitInformation,
    /// shannon per second
    ///
    /// rec20
    ShannonPerSecond,
    /// hartley per second
    ///
    /// rec20
    HartleyPerSecond,
    /// natural unit of information per second
    ///
    /// rec20
    NaturalUnitInformationPerSecond,
    /// second per kilogramm
    ///
    /// rec20
    SecondPerKilogramm,
    /// watt square metre
    ///
    /// rec20
    WattSquareMetre,
    /// second per radian cubic metre
    ///
    /// rec20
    SecondPerRadianCubicMetre,
    /// weber to the power minus one
    ///
    /// rec20
    WeberToPowerMinusOne,
    /// reciprocal inch
    ///
    /// rec20
    ReciprocalInch,
    /// dioptre
    ///
    /// rec20
    Dioptre,
    /// one per one
    ///
    /// rec20
    OnePerOne,
    /// newton metre per metre
    ///
    /// rec20
    NewtonMetrePerMetre,
    /// kilogram per square metre pascal second
    ///
    /// rec20
    KilogramPerSquareMetrePascalSecond,
    /// microgram per hectogram
    ///
    /// rec20
    MicrogramPerHectogram,
    /// meal
    ///
    /// rec20
    Meal,
    /// pH (potential of Hydrogen)
    ///
    /// rec20
    PhPotentialHydrogen,
    /// kilojoule per gram
    ///
    /// rec20
    KilojoulePerGram,
    /// femtolitre
    ///
    /// rec20
    Femtolitre,
    /// picolitre
    ///
    /// rec20
    Picolitre,
    /// nanolitre
    ///
    /// rec20
    Nanolitre,
    /// megawatts per minute
    ///
    /// rec20
    MegawattsPerMinute,
    /// square metre per cubic metre
    ///
    /// rec20
    SquareMetrePerCubicMetre,
    /// Standard cubic metre per day
    ///
    /// rec20
    StandardCubicMetrePerDay,
    /// Standard cubic metre per hour
    ///
    /// rec20
    StandardCubicMetrePerHour,
    /// Normalized cubic metre per day
    ///
    /// rec20
    NormalizedCubicMetrePerDay,
    /// Normalized cubic metre per hour
    ///
    /// rec20
    NormalizedCubicMetrePerHour,
    /// Joule per normalised cubic metre
    ///
    /// rec20
    JoulePerNormalisedCubicMetre,
    /// Joule per standard cubic metre
    ///
    /// rec20
    JoulePerStandardCubicMetre,
    /// page - facsimile
    ///
    /// rec20
    PageFacsimile,
    /// quarter (of a year)
    ///
    /// rec20
    QuarterAYear,
    /// page - hardcopy
    ///
    /// rec20
    PageHardcopy,
    /// quire
    ///
    /// rec20
    Quire,
    /// dry quart (US)
    ///
    /// rec20
    DryQuartUs,
    /// quart (UK)
    ///
    /// rec20
    QuartUk,
    /// liquid quart (US)
    ///
    /// rec20
    LiquidQuartUs,
    /// quarter (UK)
    ///
    /// rec20
    QuarterUk,
    /// pica
    ///
    /// rec20
    Pica,
    /// thousand cubic metre
    ///
    /// rec20
    ThousandCubicMetre,
    /// running or operating hour
    ///
    /// rec20
    RunningOrOperatingHour,
    /// ream
    ///
    /// rec20
    Ream,
    /// room
    ///
    /// rec20
    Room,
    /// pound per ream
    ///
    /// rec20
    PoundPerReam,
    /// revolutions per minute
    ///
    /// rec20
    RevolutionsPerMinute,
    /// revolutions per second
    ///
    /// rec20
    RevolutionsPerSecond,
    /// revenue ton mile
    ///
    /// rec20
    RevenueTonMile,
    /// square foot per second
    ///
    /// rec20
    SquareFootPerSecond,
    /// square metre per second
    ///
    /// rec20
    SquareMetrePerSecond,
    /// half year (6 months)
    ///
    /// rec20
    HalfYear6Months,
    /// score
    ///
    /// rec20
    Score,
    /// scruple
    ///
    /// rec20
    Scruple,
    /// second [unit of time]
    ///
    /// rec20
    SecondUnitTime,
    /// set
    ///
    /// rec20
    Set,
    /// segment
    ///
    /// rec20
    Segment,
    /// siemens
    ///
    /// rec20
    Siemens,
    /// Standard cubic metre
    ///
    /// rec20
    StandardCubicMetre,
    /// mile (statute mile)
    ///
    /// rec20
    MileStatuteMile,
    /// square
    ///
    /// rec20
    Square,
    /// square, roofing
    ///
    /// rec20
    SquareRoofing,
    /// strip
    ///
    /// rec20
    Strip,
    /// stick
    ///
    /// rec20
    Stick,
    /// stone (UK)
    ///
    /// rec20
    StoneUk,
    /// stick, cigarette
    ///
    /// rec20
    StickCigarette,
    /// standard litre
    ///
    /// rec20
    StandardLitre,
    /// ton (US) or short ton (UK/US)
    ///
    /// rec20
    TonUsOrShortTonUkUs,
    /// straw
    ///
    /// rec20
    Straw,
    /// skein
    ///
    /// rec20
    Skein,
    /// shipment
    ///
    /// rec20
    Shipment,
    /// syringe
    ///
    /// rec20
    Syringe,
    /// telecommunication line in service
    ///
    /// rec20
    TelecommunicationLineInService,
    /// thousand piece
    ///
    /// rec20
    ThousandPiece,
    /// kiloampere hour (thousand ampere hour)
    ///
    /// rec20
    KiloampereHourThousandAmpereHour,
    /// total acid number
    ///
    /// rec20
    TotalAcidNumber,
    /// thousand square inch
    ///
    /// rec20
    ThousandSquareInch,
    /// metric ton, including container
    ///
    /// rec20
    MetricTonIncludingContainer,
    /// metric ton, including inner packaging
    ///
    /// rec20
    MetricTonIncludingInnerPackaging,
    /// tonne kilometre
    ///
    /// rec20
    TonneKilometre,
    /// kilogram of imported meat, less offal
    ///
    /// rec20
    KilogramImportedMeatLessOffal,
    /// tonne (metric ton)
    ///
    /// rec20
    TonneMetricTon,
    /// ten pack
    ///
    /// rec20
    TenPack,
    /// teeth per inch
    ///
    /// rec20
    TeethPerInch,
    /// ten pair
    ///
    /// rec20
    TenPair,
    /// thousand cubic metre per day
    ///
    /// rec20
    ThousandCubicMetrePerDay,
    /// trillion (EUR)
    ///
    /// rec20
    TrillionEur,
    /// ten set
    ///
    /// rec20
    TenSet,
    /// ten thousand sticks
    ///
    /// rec20
    TenThousandSticks,
    /// treatment
    ///
    /// rec20
    Treatment,
    /// tablet
    ///
    /// rec20
    Tablet,
    /// telecommunication line in service average
    ///
    /// rec20
    TelecommunicationLineInServiceAverage,
    /// telecommunication port
    ///
    /// rec20
    TelecommunicationPort,
    /// volt - ampere per kilogram
    ///
    /// rec20
    VoltAmperePerKilogram,
    /// volt
    ///
    /// rec20
    Volt,
    /// percent volume
    ///
    /// rec20
    PercentVolume,
    /// wet kilo
    ///
    /// rec20
    WetKilo,
    /// watt per kilogram
    ///
    /// rec20
    WattPerKilogram,
    /// wet pound
    ///
    /// rec20
    WetPound,
    /// cord
    ///
    /// rec20
    Cord,
    /// wet ton
    ///
    /// rec20
    WetTon,
    /// weber
    ///
    /// rec20
    Weber,
    /// week
    ///
    /// rec20
    Week,
    /// wine gallon
    ///
    /// rec20
    WineGallon,
    /// watt hour
    ///
    /// rec20
    WattHour,
    /// working month
    ///
    /// rec20
    WorkingMonth,
    /// standard
    ///
    /// rec20
    Standard,
    /// watt
    ///
    /// rec20
    Watt,
    /// Gunter's chain
    ///
    /// rec20
    GuntersChain,
    /// square yard
    ///
    /// rec20
    SquareYard,
    /// cubic yard
    ///
    /// rec20
    CubicYard,
    /// yard
    ///
    /// rec20
    Yard,
    /// hanging container
    ///
    /// rec20
    HangingContainer,
    /// nanomole
    ///
    /// Rec20
    Nanomole,
    /// page
    ///
    /// rec20
    Page,
    /// mutually defined
    ///
    /// rec20
    MutuallyDefined,
    /// Drum, steel
    ///
    /// rec21
    DrumSteel,
    /// Drum, aluminium
    ///
    /// rec21
    DrumAluminium,
    /// Drum, plywood
    ///
    /// rec21
    DrumPlywood,
    /// Container, flexible
    ///
    /// rec21
    ContainerFlexible,
    /// Drum, fibre
    ///
    /// rec21
    DrumFibre,
    /// Drum, wooden
    ///
    /// rec21
    DrumWooden,
    /// Barrel, wooden
    ///
    /// rec21
    BarrelWooden,
    /// Jerrican, steel
    ///
    /// rec21
    JerricanSteel,
    /// Jerrican, plastic
    ///
    /// rec21
    JerricanPlastic,
    /// Bag, super bulk
    ///
    /// rec21
    BagSuperBulk,
    /// Bag, polybag
    ///
    /// rec21
    BagPolybag,
    /// Box, steel
    ///
    /// rec21
    BoxSteel,
    /// Box, aluminium
    ///
    /// rec21
    BoxAluminium,
    /// Box, natural wood
    ///
    /// rec21
    BoxNaturalWood,
    /// Box, plywood
    ///
    /// rec21
    BoxPlywood,
    /// Box, reconstituted wood
    ///
    /// rec21
    BoxReconstitutedWood,
    /// Box, fibreboard
    ///
    /// rec21
    BoxFibreboard,
    /// Box, plastic
    ///
    /// rec21
    BoxPlastic,
    /// Bag, woven plastic
    ///
    /// rec21
    BagWovenPlastic,
    /// Bag, textile
    ///
    /// rec21
    BagTextile,
    /// Bag, paper
    ///
    /// rec21
    BagPaper,
    /// Composite packaging, plastic receptacle
    ///
    /// rec21
    CompositePackagingPlasticReceptacle,
    /// Composite packaging, glass receptacle
    ///
    /// rec21
    CompositePackagingGlassReceptacle,
    /// Case, car
    ///
    /// rec21
    CaseCar,
    /// Case, wooden
    ///
    /// rec21
    CaseWooden,
    /// Pallet, wooden
    ///
    /// rec21
    PalletWooden,
    /// Crate, wooden
    ///
    /// rec21
    CrateWooden,
    /// Bundle, wooden
    ///
    /// rec21
    BundleWooden,
    /// Intermediate bulk container, rigid plastic
    ///
    /// rec21
    IntermediateBulkContainerRigidPlastic,
    /// Receptacle, fibre
    ///
    /// rec21
    ReceptacleFibre,
    /// Receptacle, paper
    ///
    /// rec21
    ReceptaclePaper,
    /// Receptacle, wooden
    ///
    /// rec21
    ReceptacleWooden,
    /// Aerosol
    ///
    /// rec21
    Aerosol,
    /// Pallet, modular, collars 80cms * 60cms
    ///
    /// rec21
    PalletModularCollars80cms60cms,
    /// Pallet, shrinkwrapped
    ///
    /// rec21
    PalletShrinkwrapped,
    /// Pallet, 100cms * 110cms
    ///
    /// rec21
    Pallet100cms110cms,
    /// Clamshell
    ///
    /// rec21
    Clamshell,
    /// Cone
    ///
    /// rec21
    Cone,
    /// Ball
    ///
    /// rec21
    Ball_Dup,
    /// Ampoule, non-protected
    ///
    /// rec21
    AmpouleNonProtected,
    /// Ampoule, protected
    ///
    /// rec21
    AmpouleProtected,
    /// Atomizer
    ///
    /// rec21
    Atomizer,
    /// Capsule
    ///
    /// rec21
    Capsule,
    /// Belt
    ///
    /// rec21
    Belt,
    /// Barrel
    ///
    /// rec21
    Barrel,
    /// Bobbin
    ///
    /// rec21
    Bobbin,
    /// Bottlecrate / bottlerack
    ///
    /// rec21
    BottlecrateBottlerack,
    /// Board
    ///
    /// rec21
    Board,
    /// Bundle
    ///
    /// rec21
    Bundle,
    /// Balloon, non-protected
    ///
    /// rec21
    BalloonNonProtected,
    /// Bag
    ///
    /// rec21
    Bag,
    /// Bunch
    ///
    /// rec21
    Bunch,
    /// Bin
    ///
    /// rec21
    Bin,
    /// Bucket
    ///
    /// rec21
    Bucket,
    /// Basket
    ///
    /// rec21
    Basket,
    /// Bale, compressed
    ///
    /// rec21
    BaleCompressed,
    /// Basin
    ///
    /// rec21
    Basin,
    /// Bale, non-compressed
    ///
    /// rec21
    BaleNonCompressed,
    /// Bottle, non-protected, cylindrical
    ///
    /// rec21
    BottleNonProtectedCylindrical,
    /// Balloon, protected
    ///
    /// rec21
    BalloonProtected,
    /// Bottle, protected cylindrical
    ///
    /// rec21
    BottleProtectedCylindrical,
    /// Bar
    ///
    /// rec21
    Bar,
    /// Bottle, non-protected, bulbous
    ///
    /// rec21
    BottleNonProtectedBulbous,
    /// Bolt
    ///
    /// rec21
    Bolt,
    /// Butt
    ///
    /// rec21
    Butt,
    /// Bottle, protected bulbous
    ///
    /// rec21
    BottleProtectedBulbous,
    /// Box, for liquids
    ///
    /// rec21
    BoxForLiquids,
    /// Box
    ///
    /// rec21
    Box,
    /// Board, in bundle/bunch/truss
    ///
    /// rec21
    BoardInBundleBunchTruss,
    /// Bars, in bundle/bunch/truss
    ///
    /// rec21
    BarsInBundleBunchTruss,
    /// Can, rectangular
    ///
    /// rec21
    CanRectangular,
    /// Crate, beer
    ///
    /// rec21
    CrateBeer,
    /// Churn
    ///
    /// rec21
    Churn,
    /// Can, with handle and spout
    ///
    /// rec21
    CanWithHandleAndSpout,
    /// Creel
    ///
    /// rec21
    Creel,
    /// Coffer
    ///
    /// rec21
    Coffer,
    /// Cage
    ///
    /// rec21
    Cage,
    /// Chest
    ///
    /// rec21
    Chest,
    /// Canister
    ///
    /// rec21
    Canister,
    /// Coffin
    ///
    /// rec21
    Coffin,
    /// Cask
    ///
    /// rec21
    Cask,
    /// Coil
    ///
    /// rec21
    Coil,
    /// Card
    ///
    /// rec21
    Card_Dup,
    /// Container, not otherwise specified as transport equipment
    ///
    /// rec21
    ContainerNotOtherwiseSpecifiedAsTransportEquipment,
    /// Carboy, non-protected
    ///
    /// rec21
    CarboyNonProtected,
    /// Carboy, protected
    ///
    /// rec21
    CarboyProtected,
    /// Cartridge
    ///
    /// rec21
    Cartridge,
    /// Crate
    ///
    /// rec21
    Crate,
    /// Case
    ///
    /// rec21
    Case,
    /// Carton
    ///
    /// rec21
    Carton,
    /// Cup
    ///
    /// rec21
    Cup,
    /// Cover
    ///
    /// rec21
    Cover,
    /// Cage, roll
    ///
    /// rec21
    CageRoll,
    /// Can, cylindrical
    ///
    /// rec21
    CanCylindrical,
    /// Cylinder
    ///
    /// rec21
    Cylinder,
    /// Canvas
    ///
    /// rec21
    Canvas,
    /// Crate, multiple layer, plastic
    ///
    /// rec21
    CrateMultipleLayerPlastic,
    /// Crate, multiple layer, wooden
    ///
    /// rec21
    CrateMultipleLayerWooden,
    /// Crate, multiple layer, cardboard
    ///
    /// rec21
    CrateMultipleLayerCardboard,
    /// Cage, Commonwealth Handling Equipment Pool (CHEP)
    ///
    /// rec21
    CageCommonwealthHandlingEquipmentPoolChep,
    /// Box, Commonwealth Handling Equipment Pool (CHEP), Eurobox
    ///
    /// rec21
    BoxCommonwealthHandlingEquipmentPoolChepEurobox,
    /// Drum, iron
    ///
    /// rec21
    DrumIron,
    /// Demijohn, non-protected
    ///
    /// rec21
    DemijohnNonProtected,
    /// Crate, bulk, cardboard
    ///
    /// rec21
    CrateBulkCardboard,
    /// Crate, bulk, plastic
    ///
    /// rec21
    CrateBulkPlastic,
    /// Crate, bulk, wooden
    ///
    /// rec21
    CrateBulkWooden,
    /// Dispenser
    ///
    /// rec21
    Dispenser,
    /// Demijohn, protected
    ///
    /// rec21
    DemijohnProtected,
    /// Drum
    ///
    /// rec21
    Drum,
    /// Tray, one layer no cover, plastic
    ///
    /// rec21
    TrayOneLayerNoCoverPlastic,
    /// Tray, one layer no cover, wooden
    ///
    /// rec21
    TrayOneLayerNoCoverWooden,
    /// Tray, one layer no cover, polystyrene
    ///
    /// rec21
    TrayOneLayerNoCoverPolystyrene,
    /// Tray, one layer no cover, cardboard
    ///
    /// rec21
    TrayOneLayerNoCoverCardboard,
    /// Tray, two layers no cover, plastic tray
    ///
    /// rec21
    TrayTwoLayersNoCoverPlasticTray,
    /// Tray, two layers no cover, wooden
    ///
    /// rec21
    TrayTwoLayersNoCoverWooden,
    /// Tray, two layers no cover, cardboard
    ///
    /// rec21
    TrayTwoLayersNoCoverCardboard,
    /// Bag, plastic
    ///
    /// rec21
    BagPlastic,
    /// Case, with pallet base
    ///
    /// rec21
    CaseWithPalletBase,
    /// Case, with pallet base, wooden
    ///
    /// rec21
    CaseWithPalletBaseWooden,
    /// Case, with pallet base, cardboard
    ///
    /// rec21
    CaseWithPalletBaseCardboard,
    /// Case, with pallet base, plastic
    ///
    /// rec21
    CaseWithPalletBasePlastic,
    /// Case, with pallet base, metal
    ///
    /// rec21
    CaseWithPalletBaseMetal,
    /// Case, isothermic
    ///
    /// rec21
    CaseIsothermic,
    /// Envelope
    ///
    /// rec21
    Envelope,
    /// Flexibag
    ///
    /// rec21
    Flexibag,
    /// Crate, fruit
    ///
    /// rec21
    CrateFruit,
    /// Crate, framed
    ///
    /// rec21
    CrateFramed,
    /// Flexitank
    ///
    /// rec21
    Flexitank,
    /// Firkin
    ///
    /// rec21
    Firkin,
    /// Flask
    ///
    /// rec21
    Flask,
    /// Footlocker
    ///
    /// rec21
    Footlocker,
    /// Filmpack
    ///
    /// rec21
    Filmpack,
    /// Frame
    ///
    /// rec21
    Frame,
    /// Foodtainer
    ///
    /// rec21
    Foodtainer,
    /// Cart, flatbed
    ///
    /// rec21
    CartFlatbed,
    /// Bag, flexible container
    ///
    /// rec21
    BagFlexibleContainer,
    /// Bottle, gas
    ///
    /// rec21
    BottleGas,
    /// Girder
    ///
    /// rec21
    Girder,
    /// Container, gallon
    ///
    /// rec21
    ContainerGallon,
    /// Receptacle, glass
    ///
    /// rec21
    ReceptacleGlass,
    /// Tray, containing horizontally stacked flat items
    ///
    /// rec21
    TrayContainingHorizontallyStackedFlatItems,
    /// Bag, gunny
    ///
    /// rec21
    BagGunny,
    /// Girders, in bundle/bunch/truss
    ///
    /// rec21
    GirdersInBundleBunchTruss,
    /// Basket, with handle, plastic
    ///
    /// rec21
    BasketWithHandlePlastic,
    /// Basket, with handle, wooden
    ///
    /// rec21
    BasketWithHandleWooden,
    /// Basket, with handle, cardboard
    ///
    /// rec21
    BasketWithHandleCardboard,
    /// Hogshead
    ///
    /// rec21
    Hogshead,
    /// Hanger
    ///
    /// rec21
    Hanger,
    /// Hamper
    ///
    /// rec21
    Hamper,
    /// Package, display, wooden
    ///
    /// rec21
    PackageDisplayWooden,
    /// Package, display, cardboard
    ///
    /// rec21
    PackageDisplayCardboard,
    /// Package, display, plastic
    ///
    /// rec21
    PackageDisplayPlastic,
    /// Package, display, metal
    ///
    /// rec21
    PackageDisplayMetal,
    /// Package, show
    ///
    /// rec21
    PackageShow,
    /// Package, flow
    ///
    /// rec21
    PackageFlow,
    /// Package, paper wrapped
    ///
    /// rec21
    PackagePaperWrapped,
    /// Drum, plastic
    ///
    /// rec21
    DrumPlastic,
    /// Package, cardboard, with bottle grip-holes
    ///
    /// rec21
    PackageCardboardWithBottleGripHoles,
    /// Tray, rigid, lidded stackable (CEN TS 14482:2002)
    ///
    /// rec21
    TrayRigidLiddedStackableCenTs144822002,
    /// Ingot
    ///
    /// rec21
    Ingot,
    /// Ingots, in bundle/bunch/truss
    ///
    /// rec21
    IngotsInBundleBunchTruss,
    /// Bag, jumbo
    ///
    /// rec21
    BagJumbo,
    /// Jerrican, rectangular
    ///
    /// rec21
    JerricanRectangular,
    /// Jug
    ///
    /// rec21
    Jug,
    /// Jar
    ///
    /// rec21
    Jar,
    /// Jutebag
    ///
    /// rec21
    Jutebag,
    /// Jerrican, cylindrical
    ///
    /// rec21
    JerricanCylindrical,
    /// Keg
    ///
    /// rec21
    Keg,
    /// Kit
    ///
    /// rec21
    Kit_Dup,
    /// Luggage
    ///
    /// rec21
    Luggage,
    /// Log
    ///
    /// rec21
    Log,
    /// Lot
    ///
    /// rec21
    Lot,
    /// Lug
    ///
    /// rec21
    Lug,
    /// Liftvan
    ///
    /// rec21
    Liftvan,
    /// Logs, in bundle/bunch/truss
    ///
    /// rec21
    LogsInBundleBunchTruss,
    /// Crate, metal
    ///
    /// rec21
    CrateMetal,
    /// Bag, multiply
    ///
    /// rec21
    BagMultiply,
    /// Crate, milk
    ///
    /// rec21
    CrateMilk,
    /// Container, metal
    ///
    /// rec21
    ContainerMetal,
    /// Receptacle, metal
    ///
    /// rec21
    ReceptacleMetal,
    /// Sack, multi-wall
    ///
    /// rec21
    SackMultiWall,
    /// Mat
    ///
    /// rec21
    Mat,
    /// Receptacle, plastic wrapped
    ///
    /// rec21
    ReceptaclePlasticWrapped,
    /// Matchbox
    ///
    /// rec21
    Matchbox,
    /// Not available
    ///
    /// rec21
    NotAvailable,
    /// Unpacked or unpackaged
    ///
    /// rec21
    UnpackedOrUnpackaged,
    /// Unpacked or unpackaged, single unit
    ///
    /// rec21
    UnpackedOrUnpackagedSingleUnit,
    /// Unpacked or unpackaged, multiple units
    ///
    /// rec21
    UnpackedOrUnpackagedMultipleUnits,
    /// Nest
    ///
    /// rec21
    Nest,
    /// Net
    ///
    /// rec21
    Net,
    /// Net, tube, plastic
    ///
    /// rec21
    NetTubePlastic,
    /// Net, tube, textile
    ///
    /// rec21
    NetTubeTextile,
    /// Two sided cage on wheels with fixing strap
    ///
    /// Rec21
    TwoSidedCageOnWheelsWithFixingStrap,
    /// Trolley
    ///
    /// Rec21
    Trolley,
    /// Oneway pallet ISO 0 - 1/2 EURO Pallet
    ///
    /// Rec21
    OnewayPalletIso012EuroPallet,
    /// Oneway pallet ISO 1 - 1/1 EURO Pallet
    ///
    /// Rec21
    OnewayPalletIso111EuroPallet,
    /// Oneway pallet ISO 2 - 2/1 EURO Pallet
    ///
    /// Rec21
    OnewayPalletIso221EuroPallet,
    /// Pallet with exceptional dimensions
    ///
    /// Rec21
    PalletWithExceptionalDimensions,
    /// Wooden pallet 40 cm x 80 cm
    ///
    /// Rec21
    WoodenPallet40CmX80Cm,
    /// Plastic pallet SRS 60 cm x 80 cm
    ///
    /// Rec21
    PlasticPalletSrs60CmX80Cm,
    /// Plastic pallet SRS 80 cm x 120 cm
    ///
    /// Rec21
    PlasticPalletSrs80CmX120Cm,
    /// Pallet, CHEP 40 cm x 60 cm
    ///
    /// rec21
    PalletChep40CmX60Cm,
    /// Pallet, CHEP 80 cm x 120 cm
    ///
    /// rec21
    PalletChep80CmX120Cm,
    /// Pallet, CHEP 100 cm x 120 cm
    ///
    /// rec21
    PalletChep100CmX120Cm,
    /// Pallet, AS 4068-1993
    ///
    /// rec21
    PalletAs40681993,
    /// Pallet, ISO T11
    ///
    /// rec21
    PalletIsoT11,
    /// Platform, unspecified weight or dimension
    ///
    /// rec21
    PlatformUnspecifiedWeightOrDimension,
    /// Pallet ISO 0 - 1/2 EURO Pallet
    ///
    /// Rec21
    PalletIso012EuroPallet,
    /// Pallet ISO 1 - 1/1 EURO Pallet
    ///
    /// Rec21
    PalletIso111EuroPallet,
    /// Pallet ISO 2 – 2/1 EURO Pallet
    ///
    /// Rec21
    PalletIso221EuroPallet,
    /// 1/4 EURO Pallet
    ///
    /// Rec21
    _14EuroPallet,
    /// Block
    ///
    /// rec21
    Block,
    /// 1/8 EURO Pallet
    ///
    /// Rec21
    _18EuroPallet,
    /// Synthetic pallet ISO 1
    ///
    /// Rec21
    SyntheticPalletIso1,
    /// Synthetic pallet ISO 2
    ///
    /// Rec21
    SyntheticPalletIso2,
    /// Wholesaler pallet
    ///
    /// Rec21
    WholesalerPallet,
    /// Pallet 80 X 100 cm
    ///
    /// Rec21
    Pallet80X100Cm,
    /// Pallet 60 X 100 cm
    ///
    /// Rec21
    Pallet60X100Cm,
    /// Oneway pallet
    ///
    /// Rec21
    OnewayPallet,
    /// Octabin
    ///
    /// rec21
    Octabin,
    /// Container, outer
    ///
    /// rec21
    ContainerOuter,
    /// Returnable pallet
    ///
    /// Rec21
    ReturnablePallet,
    /// Large bag, pallet sized
    ///
    /// Rec21
    LargeBagPalletSized,
    /// A wheeled pallet with raised rim (81 x 67 x 135)
    ///
    /// Rec21
    AWheeledPalletWithRaisedRim81X67X135,
    /// A Wheeled pallet with raised rim (81 x 72 x 135)
    ///
    /// Rec21
    AWheeledPalletWithRaisedRim81X72X135,
    /// Wheeled pallet with raised rim ( 81 x 60 x 16)
    ///
    /// Rec21
    WheeledPalletWithRaisedRim81X60X16,
    /// CHEP pallet 60 cm x 80 cm
    ///
    /// Rec21
    ChepPallet60CmX80Cm,
    /// Pan
    ///
    /// rec21
    Pan,
    /// LPR pallet 60 cm x 80 cm
    ///
    /// Rec21
    LprPallet60CmX80Cm,
    /// LPR pallet 80 cm x 120 cm
    ///
    /// Rec21
    LprPallet80CmX120Cm,
    /// Packet
    ///
    /// rec21
    Packet,
    /// Pallet, box Combined open-ended box and pallet
    ///
    /// rec21
    PalletBoxCombinedOpenEndedBoxAndPallet,
    /// Parcel
    ///
    /// rec21
    Parcel,
    /// Pallet, modular, collars 80cms * 100cms
    ///
    /// rec21
    PalletModularCollars80cms100cms,
    /// Pallet, modular, collars 80cms * 120cms
    ///
    /// rec21
    PalletModularCollars80cms120cms,
    /// Pen
    ///
    /// rec21
    Pen,
    /// Plate
    ///
    /// rec21
    Plate,
    /// Pitcher
    ///
    /// rec21
    Pitcher,
    /// Pipe
    ///
    /// rec21
    Pipe,
    /// Punnet
    ///
    /// rec21
    Punnet,
    /// Package
    ///
    /// rec21
    Package,
    /// Pail
    ///
    /// rec21
    Pail,
    /// Plank
    ///
    /// rec21
    Plank,
    /// Pouch
    ///
    /// rec21
    Pouch,
    /// Piece
    ///
    /// rec21
    Piece_Dup,
    /// Receptacle, plastic
    ///
    /// rec21
    ReceptaclePlastic,
    /// Pot
    ///
    /// rec21
    Pot,
    /// Tray
    ///
    /// rec21
    Tray,
    /// Pipes, in bundle/bunch/truss
    ///
    /// rec21
    PipesInBundleBunchTruss,
    /// Pallet
    ///
    /// rec21
    Pallet,
    /// Plates, in bundle/bunch/truss
    ///
    /// rec21
    PlatesInBundleBunchTruss,
    /// Planks, in bundle/bunch/truss
    ///
    /// rec21
    PlanksInBundleBunchTruss,
    /// Drum, steel, non-removable head
    ///
    /// rec21
    DrumSteelNonRemovableHead,
    /// Drum, steel, removable head
    ///
    /// rec21
    DrumSteelRemovableHead,
    /// Drum, aluminium, non-removable head
    ///
    /// rec21
    DrumAluminiumNonRemovableHead,
    /// Drum, aluminium, removable head
    ///
    /// rec21
    DrumAluminiumRemovableHead,
    /// Drum, plastic, non-removable head
    ///
    /// rec21
    DrumPlasticNonRemovableHead,
    /// Drum, plastic, removable head
    ///
    /// rec21
    DrumPlasticRemovableHead,
    /// Barrel, wooden, bung type
    ///
    /// rec21
    BarrelWoodenBungType,
    /// Barrel, wooden, removable head
    ///
    /// rec21
    BarrelWoodenRemovableHead,
    /// Jerrican, steel, non-removable head
    ///
    /// rec21
    JerricanSteelNonRemovableHead,
    /// Jerrican, steel, removable head
    ///
    /// rec21
    JerricanSteelRemovableHead,
    /// Jerrican, plastic, non-removable head
    ///
    /// rec21
    JerricanPlasticNonRemovableHead,
    /// Jerrican, plastic, removable head
    ///
    /// rec21
    JerricanPlasticRemovableHead,
    /// Box, wooden, natural wood, ordinary
    ///
    /// rec21
    BoxWoodenNaturalWoodOrdinary,
    /// Box, wooden, natural wood, with sift proof walls
    ///
    /// rec21
    BoxWoodenNaturalWoodWithSiftProofWalls,
    /// Box, plastic, expanded
    ///
    /// rec21
    BoxPlasticExpanded,
    /// Box, plastic, solid
    ///
    /// rec21
    BoxPlasticSolid,
    /// Rod
    ///
    /// rec21
    Rod,
    /// Ring
    ///
    /// rec21
    Ring,
    /// Rack, clothing hanger
    ///
    /// rec21
    RackClothingHanger,
    /// Rack
    ///
    /// rec21
    Rack,
    /// Reel
    ///
    /// rec21
    Reel,
    /// Roll
    ///
    /// rec21
    Roll,
    /// Rednet
    ///
    /// rec21
    Rednet,
    /// Rods, in bundle/bunch/truss
    ///
    /// rec21
    RodsInBundleBunchTruss,
    /// Sack
    ///
    /// rec21
    Sack,
    /// Slab
    ///
    /// rec21
    Slab,
    /// Crate, shallow
    ///
    /// rec21
    CrateShallow,
    /// Spindle
    ///
    /// rec21
    Spindle,
    /// Sea-chest
    ///
    /// rec21
    SeaChest,
    /// Sachet
    ///
    /// rec21
    Sachet,
    /// Skid
    ///
    /// rec21
    Skid,
    /// Case, skeleton
    ///
    /// rec21
    CaseSkeleton,
    /// Slipsheet
    ///
    /// rec21
    Slipsheet,
    /// Sheetmetal
    ///
    /// rec21
    Sheetmetal,
    /// Spool
    ///
    /// rec21
    Spool,
    /// Sheet, plastic wrapping
    ///
    /// rec21
    SheetPlasticWrapping,
    /// Case, steel
    ///
    /// rec21
    CaseSteel,
    /// Sheet
    ///
    /// rec21
    Sheet,
    /// Suitcase
    ///
    /// rec21
    Suitcase,
    /// Envelope, steel
    ///
    /// rec21
    EnvelopeSteel,
    /// Shrinkwrapped
    ///
    /// rec21
    Shrinkwrapped,
    /// Set
    ///
    /// Rec21
    Set_Dup,
    /// Sleeve
    ///
    /// rec21
    Sleeve,
    /// Sheets, in bundle/bunch/truss
    ///
    /// rec21
    SheetsInBundleBunchTruss,
    /// Tablet
    ///
    /// rec21
    Tablet_Dup,
    /// Tub
    ///
    /// rec21
    Tub,
    /// Tea-chest
    ///
    /// rec21
    TeaChest,
    /// Tube, collapsible
    ///
    /// rec21
    TubeCollapsible,
    /// Tyre
    ///
    /// rec21
    Tyre_Dup,
    /// Tank container, generic
    ///
    /// rec21
    TankContainerGeneric,
    /// Tierce
    ///
    /// rec21
    Tierce,
    /// Tank, rectangular
    ///
    /// rec21
    TankRectangular,
    /// Tub, with lid
    ///
    /// rec21
    TubWithLid,
    /// Tin
    ///
    /// rec21
    Tin,
    /// Tun
    ///
    /// rec21
    Tun,
    /// Trunk
    ///
    /// rec21
    Trunk,
    /// Truss
    ///
    /// rec21
    Truss,
    /// Bag, tote
    ///
    /// rec21
    BagTote,
    /// Tube
    ///
    /// rec21
    Tube,
    /// Tube, with nozzle
    ///
    /// rec21
    TubeWithNozzle,
    /// Pallet, triwall
    ///
    /// rec21
    PalletTriwall,
    /// Tank, cylindrical
    ///
    /// rec21
    TankCylindrical,
    /// Tubes, in bundle/bunch/truss
    ///
    /// rec21
    TubesInBundleBunchTruss,
    /// Uncaged
    ///
    /// rec21
    Uncaged,
    /// Unit
    ///
    /// rec21
    Unit,
    /// Vat
    ///
    /// rec21
    Vat,
    /// Bulk, gas (at 1031 mbar and 15°C)
    ///
    /// rec21
    BulkGasAt1031MbarAnd15C,
    /// Vial
    ///
    /// rec21
    Vial,
    /// Vanpack
    ///
    /// rec21
    Vanpack,
    /// Bulk, liquid
    ///
    /// rec21
    BulkLiquid,
    /// Vehicle
    ///
    /// rec21
    Vehicle,
    /// Bulk, solid, large particles (“nodules”)
    ///
    /// rec21
    BulkSolidLargeParticlesNodules,
    /// Vacuum-packed
    ///
    /// rec21
    VacuumPacked,
    /// Bulk, liquefied gas (at abnormal temperature/pressure)
    ///
    /// rec21
    BulkLiquefiedGasAtAbnormalTemperaturePressure,
    /// Bulk, solid, granular particles (“grains”)
    ///
    /// rec21
    BulkSolidGranularParticlesGrains,
    /// Bulk, scrap metal
    ///
    /// rec21
    BulkScrapMetal,
    /// Bulk, solid, fine particles (“powders”)
    ///
    /// rec21
    BulkSolidFineParticlesPowders,
    /// Intermediate bulk container
    ///
    /// rec21
    IntermediateBulkContainer,
    /// Wickerbottle
    ///
    /// rec21
    Wickerbottle,
    /// Intermediate bulk container, steel
    ///
    /// rec21
    IntermediateBulkContainerSteel,
    /// Intermediate bulk container, aluminium
    ///
    /// rec21
    IntermediateBulkContainerAluminium,
    /// Intermediate bulk container, metal
    ///
    /// rec21
    IntermediateBulkContainerMetal,
    /// Intermediate bulk container, steel, pressurised > 10 kpa
    ///
    /// rec21
    IntermediateBulkContainerSteelPressurised10Kpa,
    /// Intermediate bulk container, aluminium, pressurised > 10 kpa
    ///
    /// rec21
    IntermediateBulkContainerAluminiumPressurised10Kpa,
    /// Intermediate bulk container, metal, pressure 10 kpa
    ///
    /// rec21
    IntermediateBulkContainerMetalPressure10Kpa,
    /// Intermediate bulk container, steel, liquid
    ///
    /// rec21
    IntermediateBulkContainerSteelLiquid,
    /// Intermediate bulk container, aluminium, liquid
    ///
    /// rec21
    IntermediateBulkContainerAluminiumLiquid,
    /// Intermediate bulk container, metal, liquid
    ///
    /// rec21
    IntermediateBulkContainerMetalLiquid,
    /// Intermediate bulk container, woven plastic, without coat/liner
    ///
    /// rec21
    IntermediateBulkContainerWovenPlasticWithoutCoatLiner,
    /// Intermediate bulk container, woven plastic, coated
    ///
    /// rec21
    IntermediateBulkContainerWovenPlasticCoated,
    /// Intermediate bulk container, woven plastic, with liner
    ///
    /// rec21
    IntermediateBulkContainerWovenPlasticWithLiner,
    /// Intermediate bulk container, woven plastic, coated and liner
    ///
    /// rec21
    IntermediateBulkContainerWovenPlasticCoatedAndLiner,
    /// Intermediate bulk container, plastic film
    ///
    /// rec21
    IntermediateBulkContainerPlasticFilm,
    /// Intermediate bulk container, textile with out coat/liner
    ///
    /// rec21
    IntermediateBulkContainerTextileWithOutCoatLiner,
    /// Intermediate bulk container, natural wood, with inner liner
    ///
    /// rec21
    IntermediateBulkContainerNaturalWoodWithInnerLiner,
    /// Intermediate bulk container, textile, coated
    ///
    /// rec21
    IntermediateBulkContainerTextileCoated,
    /// Intermediate bulk container, textile, with liner
    ///
    /// rec21
    IntermediateBulkContainerTextileWithLiner,
    /// Intermediate bulk container, textile, coated and liner
    ///
    /// rec21
    IntermediateBulkContainerTextileCoatedAndLiner,
    /// Intermediate bulk container, plywood, with inner liner
    ///
    /// rec21
    IntermediateBulkContainerPlywoodWithInnerLiner,
    /// Intermediate bulk container, reconstituted wood, with inner liner
    ///
    /// rec21
    IntermediateBulkContainerReconstitutedWoodWithInnerLiner,
    /// Bag, woven plastic, without inner coat/liner
    ///
    /// rec21
    BagWovenPlasticWithoutInnerCoatLiner,
    /// Bag, woven plastic, sift proof
    ///
    /// rec21
    BagWovenPlasticSiftProof,
    /// Bag, woven plastic, water resistant
    ///
    /// rec21
    BagWovenPlasticWaterResistant,
    /// Bag, plastics film
    ///
    /// rec21
    BagPlasticsFilm,
    /// Bag, textile, without inner coat/liner
    ///
    /// rec21
    BagTextileWithoutInnerCoatLiner,
    /// Bag, textile, sift proof
    ///
    /// rec21
    BagTextileSiftProof,
    /// Bag, textile, water resistant
    ///
    /// rec21
    BagTextileWaterResistant,
    /// Bag, paper, multi-wall
    ///
    /// rec21
    BagPaperMultiWall,
    /// Bag, paper, multi-wall, water resistant
    ///
    /// rec21
    BagPaperMultiWallWaterResistant,
    /// Composite packaging, plastic receptacle in steel drum
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInSteelDrum,
    /// Composite packaging, plastic receptacle in steel crate box
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInSteelCrateBox,
    /// Composite packaging, plastic receptacle in aluminium drum
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInAluminiumDrum,
    /// Composite packaging, plastic receptacle in aluminium crate
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInAluminiumCrate,
    /// Composite packaging, plastic receptacle in wooden box
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInWoodenBox,
    /// Composite packaging, plastic receptacle in plywood drum
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInPlywoodDrum,
    /// Composite packaging, plastic receptacle in plywood box
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInPlywoodBox,
    /// Composite packaging, plastic receptacle in fibre drum
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInFibreDrum,
    /// Composite packaging, plastic receptacle in fibreboard box
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInFibreboardBox,
    /// Composite packaging, plastic receptacle in plastic drum
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInPlasticDrum,
    /// Composite packaging, plastic receptacle in solid plastic box
    ///
    /// rec21
    CompositePackagingPlasticReceptacleInSolidPlasticBox,
    /// Composite packaging, glass receptacle in steel drum
    ///
    /// rec21
    CompositePackagingGlassReceptacleInSteelDrum,
    /// Composite packaging, glass receptacle in steel crate box
    ///
    /// rec21
    CompositePackagingGlassReceptacleInSteelCrateBox,
    /// Composite packaging, glass receptacle in aluminium drum
    ///
    /// rec21
    CompositePackagingGlassReceptacleInAluminiumDrum,
    /// Composite packaging, glass receptacle in aluminium crate
    ///
    /// rec21
    CompositePackagingGlassReceptacleInAluminiumCrate,
    /// Composite packaging, glass receptacle in wooden box
    ///
    /// rec21
    CompositePackagingGlassReceptacleInWoodenBox,
    /// Composite packaging, glass receptacle in plywood drum
    ///
    /// rec21
    CompositePackagingGlassReceptacleInPlywoodDrum,
    /// Composite packaging, glass receptacle in wickerwork hamper
    ///
    /// rec21
    CompositePackagingGlassReceptacleInWickerworkHamper,
    /// Composite packaging, glass receptacle in fibre drum
    ///
    /// rec21
    CompositePackagingGlassReceptacleInFibreDrum,
    /// Composite packaging, glass receptacle in fibreboard box
    ///
    /// rec21
    CompositePackagingGlassReceptacleInFibreboardBox,
    /// Composite packaging, glass receptacle in expandable plastic pack
    ///
    /// rec21
    CompositePackagingGlassReceptacleInExpandablePlasticPack,
    /// Composite packaging, glass receptacle in solid plastic pack
    ///
    /// rec21
    CompositePackagingGlassReceptacleInSolidPlasticPack,
    /// Intermediate bulk container, paper, multi-wall
    ///
    /// rec21
    IntermediateBulkContainerPaperMultiWall,
    /// Bag, large
    ///
    /// rec21
    BagLarge,
    /// Intermediate bulk container, paper, multi-wall, water resistant
    ///
    /// rec21
    IntermediateBulkContainerPaperMultiWallWaterResistant,
    /// Intermediate bulk container, rigid plastic, with structural equipment, solids
    ///
    /// rec21
    IntermediateBulkContainerRigidPlasticWithStructuralEquipmentSolids,
    /// Intermediate bulk container, rigid plastic, freestanding, solids
    ///
    /// rec21
    IntermediateBulkContainerRigidPlasticFreestandingSolids,
    /// Intermediate bulk container, rigid plastic, with structural equipment, pressurised
    ///
    /// rec21
    IntermediateBulkContainerRigidPlasticWithStructuralEquipmentPressurised,
    /// Intermediate bulk container, rigid plastic, freestanding, pressurised
    ///
    /// rec21
    IntermediateBulkContainerRigidPlasticFreestandingPressurised,
    /// Intermediate bulk container, rigid plastic, with structural equipment, liquids
    ///
    /// rec21
    IntermediateBulkContainerRigidPlasticWithStructuralEquipmentLiquids,
    /// Intermediate bulk container, rigid plastic, freestanding, liquids
    ///
    /// rec21
    IntermediateBulkContainerRigidPlasticFreestandingLiquids,
    /// Intermediate bulk container, composite, rigid plastic, solids
    ///
    /// rec21
    IntermediateBulkContainerCompositeRigidPlasticSolids,
    /// Intermediate bulk container, composite, flexible plastic, solids
    ///
    /// rec21
    IntermediateBulkContainerCompositeFlexiblePlasticSolids,
    /// Intermediate bulk container, composite, rigid plastic, pressurised
    ///
    /// rec21
    IntermediateBulkContainerCompositeRigidPlasticPressurised,
    /// Intermediate bulk container, composite, flexible plastic, pressurised
    ///
    /// rec21
    IntermediateBulkContainerCompositeFlexiblePlasticPressurised,
    /// Intermediate bulk container, composite, rigid plastic, liquids
    ///
    /// rec21
    IntermediateBulkContainerCompositeRigidPlasticLiquids,
    /// Intermediate bulk container, composite, flexible plastic, liquids
    ///
    /// rec21
    IntermediateBulkContainerCompositeFlexiblePlasticLiquids,
    /// Intermediate bulk container, composite
    ///
    /// rec21
    IntermediateBulkContainerComposite,
    /// Intermediate bulk container, fibreboard
    ///
    /// rec21
    IntermediateBulkContainerFibreboard,
    /// Intermediate bulk container, flexible
    ///
    /// rec21
    IntermediateBulkContainerFlexible,
    /// Intermediate bulk container, metal, other than steel
    ///
    /// rec21
    IntermediateBulkContainerMetalOtherThanSteel,
    /// Intermediate bulk container, natural wood
    ///
    /// rec21
    IntermediateBulkContainerNaturalWood,
    /// Intermediate bulk container, plywood
    ///
    /// rec21
    IntermediateBulkContainerPlywood,
    /// Intermediate bulk container, reconstituted wood
    ///
    /// rec21
    IntermediateBulkContainerReconstitutedWood,
    /// Mutually defined
    ///
    /// rec21
    MutuallyDefined_Dup,
}

impl crate::Code for Unit {
    fn code(&self) -> &str {
        match self {
            Unit::Group => "10",
            Unit::Outfit => "11",
            Unit::Ration => "13",
            Unit::Shot => "14",
            Unit::StickMilitary => "15",
            Unit::TwentyFootContainer => "20",
            Unit::FortyFootContainer => "21",
            Unit::DecilitrePerGram => "22",
            Unit::GramPerCubicCentimetre => "23",
            Unit::TheoreticalPound => "24",
            Unit::GramPerSquareCentimetre => "25",
            Unit::TheoreticalTon => "27",
            Unit::KilogramPerSquareMetre => "28",
            Unit::KilopascalSquareMetrePerGram => "33",
            Unit::KilopascalPerMillimetre => "34",
            Unit::MillilitrePerSquareCentimetreSecond => "35",
            Unit::OuncePerSquareFoot => "37",
            Unit::OuncePerSquareFootPer001inch => "38",
            Unit::MillilitrePerSecond => "40",
            Unit::MillilitrePerMinute => "41",
            Unit::Sitas => "56",
            Unit::Mesh => "57",
            Unit::NetKilogram => "58",
            Unit::PartPerMillion => "59",
            Unit::PercentWeight => "60",
            Unit::PartPerBillionUs => "61",
            Unit::Millipascal => "74",
            Unit::MilliInch => "77",
            Unit::PoundPerSquareInchAbsolute => "80",
            Unit::Henry => "81",
            Unit::FootPoundForce => "85",
            Unit::PoundPerCubicFoot => "87",
            Unit::Poise => "89",
            Unit::Stokes => "91",
            Unit::FixedRate => "1I",
            Unit::RadianPerSecond => "2A",
            Unit::RadianPerSecondSquared => "2B",
            Unit::Roentgen => "2C",
            Unit::VoltAc => "2G",
            Unit::VoltDc => "2H",
            Unit::BritishThermalUnitInternationalTablePerHour => "2I",
            Unit::CubicCentimetrePerSecond => "2J",
            Unit::CubicFootPerHour => "2K",
            Unit::CubicFootPerMinute => "2L",
            Unit::CentimetrePerSecond => "2M",
            Unit::Decibel => "2N",
            Unit::Kilobyte => "2P",
            Unit::Kilobecquerel => "2Q",
            Unit::Kilocurie => "2R",
            Unit::Megagram => "2U",
            Unit::MetrePerMinute => "2X",
            Unit::Milliroentgen => "2Y",
            Unit::Millivolt => "2Z",
            Unit::Megajoule => "3B",
            Unit::Manmonth => "3C",
            Unit::Centistokes => "4C",
            Unit::Microlitre => "4G",
            Unit::MicrometreMicron => "4H",
            Unit::Milliampere => "4K",
            Unit::Megabyte => "4L",
            Unit::MilligramPerHour => "4M",
            Unit::Megabecquerel => "4N",
            Unit::Microfarad => "4O",
            Unit::NewtonPerMetre => "4P",
            Unit::OunceInch => "4Q",
            Unit::OunceFoot => "4R",
            Unit::Picofarad => "4T",
            Unit::PoundPerHour => "4U",
            Unit::TonUsPerHour => "4W",
            Unit::KilolitrePerHour => "4X",
            Unit::BarrelUsPerMinute => "5A",
            Unit::Batch => "5B",
            Unit::MmscfDay => "5E",
            Unit::HydraulicHorsePower => "5J",
            Unit::AmpereSquareMetrePerJouleSecond => "A10",
            Unit::Angstrom => "A11",
            Unit::AstronomicalUnit => "A12",
            Unit::Attojoule => "A13",
            Unit::Barn => "A14",
            Unit::BarnPerElectronvolt => "A15",
            Unit::BarnPerSteradianElectronvolt => "A16",
            Unit::BarnPerSteradian => "A17",
            Unit::BecquerelPerKilogram => "A18",
            Unit::BecquerelPerCubicMetre => "A19",
            Unit::AmperePerCentimetre => "A2",
            Unit::BritishThermalUnitInternationalTablePerSecondSquareFootDegreeRankine => "A20",
            Unit::BritishThermalUnitInternationalTablePerPoundDegreeRankine => "A21",
            Unit::BritishThermalUnitInternationalTablePerSecondFootDegreeRankine => "A22",
            Unit::BritishThermalUnitInternationalTablePerHourSquareFootDegreeRankine => "A23",
            Unit::CandelaPerSquareMetre => "A24",
            Unit::CoulombMetre => "A26",
            Unit::CoulombMetreSquaredPerVolt => "A27",
            Unit::CoulombPerCubicCentimetre => "A28",
            Unit::CoulombPerCubicMetre => "A29",
            Unit::AmperePerMillimetre => "A3",
            Unit::CoulombPerCubicMillimetre => "A30",
            Unit::CoulombPerKilogramSecond => "A31",
            Unit::CoulombPerMole => "A32",
            Unit::CoulombPerSquareCentimetre => "A33",
            Unit::CoulombPerSquareMetre => "A34",
            Unit::CoulombPerSquareMillimetre => "A35",
            Unit::CubicCentimetrePerMole => "A36",
            Unit::CubicDecimetrePerMole => "A37",
            Unit::CubicMetrePerCoulomb => "A38",
            Unit::CubicMetrePerKilogram => "A39",
            Unit::AmperePerSquareCentimetre => "A4",
            Unit::CubicMetrePerMole => "A40",
            Unit::AmperePerSquareMetre => "A41",
            Unit::CuriePerKilogram => "A42",
            Unit::DeadweightTonnage => "A43",
            Unit::Decalitre => "A44",
            Unit::Decametre => "A45",
            Unit::Decitex => "A47",
            Unit::DegreeRankine => "A48",
            Unit::Denier => "A49",
            Unit::AmpereSquareMetre => "A5",
            Unit::Electronvolt => "A53",
            Unit::ElectronvoltPerMetre => "A54",
            Unit::ElectronvoltSquareMetre => "A55",
            Unit::ElectronvoltSquareMetrePerKilogram => "A56",
            Unit::_8PartCloudCover => "A59",
            Unit::AmperePerSquareMetreKelvinSquared => "A6",
            Unit::Exajoule => "A68",
            Unit::FaradPerMetre => "A69",
            Unit::AmperePerSquareMillimetre => "A7",
            Unit::Femtojoule => "A70",
            Unit::Femtometre => "A71",
            Unit::FootPerSecondSquared => "A73",
            Unit::FootPoundForcePerSecond => "A74",
            Unit::FreightTon => "A75",
            Unit::Gal => "A76",
            Unit::AmpereSecond => "A8",
            Unit::GigacoulombPerCubicMetre => "A84",
            Unit::Gigaelectronvolt => "A85",
            Unit::Gigahertz => "A86",
            Unit::Gigaohm => "A87",
            Unit::GigaohmMetre => "A88",
            Unit::Gigapascal => "A89",
            Unit::Rate => "A9",
            Unit::Gigawatt => "A90",
            Unit::Gon => "A91",
            Unit::GramPerCubicMetre => "A93",
            Unit::GramPerMole => "A94",
            Unit::Gray => "A95",
            Unit::GrayPerSecond => "A96",
            Unit::Hectopascal => "A97",
            Unit::HenryPerMetre => "A98",
            Unit::Bit => "A99",
            Unit::Ball => "AA",
            Unit::BulkPack => "AB",
            Unit::Acre => "ACR",
            Unit::Activity => "ACT",
            Unit::Byte => "AD",
            Unit::AmperePerMetre => "AE",
            Unit::AdditionalMinute => "AH",
            Unit::AverageMinutePerCall => "AI",
            Unit::Fathom => "AK",
            Unit::AccessLine => "AL",
            Unit::AmpereHour => "AMH",
            Unit::Ampere => "AMP",
            Unit::Year => "ANN",
            Unit::TroyOunceOrApothecaryOunce => "APZ",
            Unit::AntiHemophilicFactorAhfUnit => "AQ",
            Unit::Assortment => "AS",
            Unit::AlcoholicStrengthByMass => "ASM",
            Unit::AlcoholicStrengthByVolume => "ASU",
            Unit::StandardAtmosphere => "ATM",
            Unit::AmericanWireGauge => "AWG",
            Unit::Assembly => "AY",
            Unit::BritishThermalUnitInternationalTablePerPound => "AZ",
            Unit::BarrelUsPerDay => "B1",
            Unit::BitPerSecond => "B10",
            Unit::JoulePerKilogramKelvin => "B11",
            Unit::JoulePerMetre => "B12",
            Unit::JoulePerSquareMetre => "B13",
            Unit::JoulePerMetreToFourthPower => "B14",
            Unit::JoulePerMole => "B15",
            Unit::JoulePerMoleKelvin => "B16",
            Unit::Credit => "B17",
            Unit::JouleSecond => "B18",
            Unit::Digit => "B19",
            Unit::JouleSquareMetrePerKilogram => "B20",
            Unit::KelvinPerWatt => "B21",
            Unit::Kiloampere => "B22",
            Unit::KiloamperePerSquareMetre => "B23",
            Unit::KiloamperePerMetre => "B24",
            Unit::KilobecquerelPerKilogram => "B25",
            Unit::Kilocoulomb => "B26",
            Unit::KilocoulombPerCubicMetre => "B27",
            Unit::KilocoulombPerSquareMetre => "B28",
            Unit::Kiloelectronvolt => "B29",
            Unit::BattingPound => "B3",
            Unit::Gibibit => "B30",
            Unit::KilogramMetrePerSecond => "B31",
            Unit::KilogramMetreSquared => "B32",
            Unit::KilogramMetreSquaredPerSecond => "B33",
            Unit::KilogramPerCubicDecimetre => "B34",
            Unit::KilogramPerLitre => "B35",
            Unit::BarrelImperial => "B4",
            Unit::KilojoulePerKelvin => "B41",
            Unit::KilojoulePerKilogram => "B42",
            Unit::KilojoulePerKilogramKelvin => "B43",
            Unit::KilojoulePerMole => "B44",
            Unit::Kilomole => "B45",
            Unit::KilomolePerCubicMetre => "B46",
            Unit::Kilonewton => "B47",
            Unit::KilonewtonMetre => "B48",
            Unit::Kiloohm => "B49",
            Unit::KiloohmMetre => "B50",
            Unit::Kilosecond => "B52",
            Unit::Kilosiemens => "B53",
            Unit::KilosiemensPerMetre => "B54",
            Unit::KilovoltPerMetre => "B55",
            Unit::KiloweberPerMetre => "B56",
            Unit::LightYear => "B57",
            Unit::LitrePerMole => "B58",
            Unit::LumenHour => "B59",
            Unit::LumenPerSquareMetre => "B60",
            Unit::LumenPerWatt => "B61",
            Unit::LumenSecond => "B62",
            Unit::LuxHour => "B63",
            Unit::LuxSecond => "B64",
            Unit::MegaamperePerSquareMetre => "B66",
            Unit::MegabecquerelPerKilogram => "B67",
            Unit::Gigabit => "B68",
            Unit::MegacoulombPerCubicMetre => "B69",
            Unit::Cycle => "B7",
            Unit::MegacoulombPerSquareMetre => "B70",
            Unit::Megaelectronvolt => "B71",
            Unit::MegagramPerCubicMetre => "B72",
            Unit::Meganewton => "B73",
            Unit::MeganewtonMetre => "B74",
            Unit::Megaohm => "B75",
            Unit::MegaohmMetre => "B76",
            Unit::MegasiemensPerMetre => "B77",
            Unit::Megavolt => "B78",
            Unit::MegavoltPerMetre => "B79",
            Unit::JoulePerCubicMetre => "B8",
            Unit::GigabitPerSecond => "B80",
            Unit::ReciprocalMetreSquaredReciprocalSecond => "B81",
            Unit::InchPerLinearFoot => "B82",
            Unit::MetreToFourthPower => "B83",
            Unit::Microampere => "B84",
            Unit::Microbar => "B85",
            Unit::Microcoulomb => "B86",
            Unit::MicrocoulombPerCubicMetre => "B87",
            Unit::MicrocoulombPerSquareMetre => "B88",
            Unit::MicrofaradPerMetre => "B89",
            Unit::Microhenry => "B90",
            Unit::MicrohenryPerMetre => "B91",
            Unit::Micronewton => "B92",
            Unit::MicronewtonMetre => "B93",
            Unit::Microohm => "B94",
            Unit::MicroohmMetre => "B95",
            Unit::Micropascal => "B96",
            Unit::Microradian => "B97",
            Unit::Microsecond => "B98",
            Unit::Microsiemens => "B99",
            Unit::BarUnitPressure => "BAR",
            Unit::BaseBox => "BB",
            Unit::BoardFoot => "BFT",
            Unit::BrakeHorsePower => "BHP",
            Unit::BillionEur => "BIL",
            Unit::DryBarrelUs => "BLD",
            Unit::BarrelUs => "BLL",
            Unit::HundredBoardFoot => "BP",
            Unit::BeatsPerMinute => "BPM",
            Unit::Becquerel => "BQL",
            Unit::BritishThermalUnitInternationalTable => "BTU",
            Unit::BushelUs => "BUA",
            Unit::BushelUk => "BUI",
            Unit::Call => "C0",
            Unit::Millifarad => "C10",
            Unit::Milligal => "C11",
            Unit::MilligramPerMetre => "C12",
            Unit::Milligray => "C13",
            Unit::Millihenry => "C14",
            Unit::Millijoule => "C15",
            Unit::MillimetrePerSecond => "C16",
            Unit::MillimetreSquaredPerSecond => "C17",
            Unit::Millimole => "C18",
            Unit::MolePerKilogram => "C19",
            Unit::Millinewton => "C20",
            Unit::Kibibit => "C21",
            Unit::MillinewtonPerMetre => "C22",
            Unit::MilliohmMetre => "C23",
            Unit::MillipascalSecond => "C24",
            Unit::Milliradian => "C25",
            Unit::Millisecond => "C26",
            Unit::Millisiemens => "C27",
            Unit::Millisievert => "C28",
            Unit::Millitesla => "C29",
            Unit::MicrovoltPerMetre => "C3",
            Unit::MillivoltPerMetre => "C30",
            Unit::Milliwatt => "C31",
            Unit::MilliwattPerSquareMetre => "C32",
            Unit::Milliweber => "C33",
            Unit::Mole => "C34",
            Unit::MolePerCubicDecimetre => "C35",
            Unit::MolePerCubicMetre => "C36",
            Unit::Kilobit => "C37",
            Unit::MolePerLitre => "C38",
            Unit::Nanoampere => "C39",
            Unit::Nanocoulomb => "C40",
            Unit::Nanofarad => "C41",
            Unit::NanofaradPerMetre => "C42",
            Unit::Nanohenry => "C43",
            Unit::NanohenryPerMetre => "C44",
            Unit::Nanometre => "C45",
            Unit::NanoohmMetre => "C46",
            Unit::Nanosecond => "C47",
            Unit::Nanotesla => "C48",
            Unit::Nanowatt => "C49",
            Unit::Neper => "C50",
            Unit::NeperPerSecond => "C51",
            Unit::Picometre => "C52",
            Unit::NewtonMetreSecond => "C53",
            Unit::NewtonMetreSquaredPerKilogramSquared => "C54",
            Unit::NewtonPerSquareMetre => "C55",
            Unit::NewtonPerSquareMillimetre => "C56",
            Unit::NewtonSecond => "C57",
            Unit::NewtonSecondPerMetre => "C58",
            Unit::Octave => "C59",
            Unit::OhmCentimetre => "C60",
            Unit::OhmMetre => "C61",
            Unit::One => "C62",
            Unit::Parsec => "C63",
            Unit::PascalPerKelvin => "C64",
            Unit::PascalSecond => "C65",
            Unit::PascalSecondPerCubicMetre => "C66",
            Unit::PascalSecondPerMetre => "C67",
            Unit::Petajoule => "C68",
            Unit::Phon => "C69",
            Unit::Centipoise => "C7",
            Unit::Picoampere => "C70",
            Unit::Picocoulomb => "C71",
            Unit::PicofaradPerMetre => "C72",
            Unit::Picohenry => "C73",
            Unit::KilobitPerSecond => "C74",
            Unit::Picowatt => "C75",
            Unit::PicowattPerSquareMetre => "C76",
            Unit::PoundForce => "C78",
            Unit::KilovoltAmpereHour => "C79",
            Unit::MillicoulombPerKilogram => "C8",
            Unit::Rad => "C80",
            Unit::Radian => "C81",
            Unit::RadianSquareMetrePerMole => "C82",
            Unit::RadianSquareMetrePerKilogram => "C83",
            Unit::RadianPerMetre => "C84",
            Unit::ReciprocalAngstrom => "C85",
            Unit::ReciprocalCubicMetre => "C86",
            Unit::ReciprocalCubicMetrePerSecond => "C87",
            Unit::ReciprocalElectronVoltPerCubicMetre => "C88",
            Unit::ReciprocalHenry => "C89",
            Unit::CoilGroup => "C9",
            Unit::ReciprocalJoulePerCubicMetre => "C90",
            Unit::ReciprocalKelvinOrKelvinToPowerMinusOne => "C91",
            Unit::ReciprocalMetre => "C92",
            Unit::ReciprocalSquareMetre => "C93",
            Unit::ReciprocalMinute => "C94",
            Unit::ReciprocalMole => "C95",
            Unit::ReciprocalPascalOrPascalToPowerMinusOne => "C96",
            Unit::ReciprocalSecond => "C97",
            Unit::ReciprocalSecondPerMetreSquared => "C99",
            Unit::CarryingCapacityInMetricTon => "CCT",
            Unit::Candela => "CDL",
            Unit::DegreeCelsius => "CEL",
            Unit::Hundred => "CEN",
            Unit::Card => "CG",
            Unit::Centigram => "CGM",
            Unit::CoulombPerKilogram => "CKG",
            Unit::HundredLeave => "CLF",
            Unit::Centilitre => "CLT",
            Unit::SquareCentimetre => "CMK",
            Unit::CubicCentimetre => "CMQ",
            Unit::Centimetre => "CMT",
            Unit::HundredPack => "CNP",
            Unit::CentalUk => "CNT",
            Unit::Coulomb => "COU",
            Unit::ContentGram => "CTG",
            Unit::MetricCarat => "CTM",
            Unit::ContentTonMetric => "CTN",
            Unit::Curie => "CUR",
            Unit::HundredPoundCwtHundredWeightUs => "CWA",
            Unit::HundredWeightUk => "CWI",
            Unit::KilowattHourPerHour => "D03",
            Unit::LotUnitWeight => "D04",
            Unit::ReciprocalSecondPerSteradian => "D1",
            Unit::SiemensPerMetre => "D10",
            Unit::Mebibit => "D11",
            Unit::SiemensSquareMetrePerMole => "D12",
            Unit::Sievert => "D13",
            Unit::Sone => "D15",
            Unit::SquareCentimetrePerErg => "D16",
            Unit::SquareCentimetrePerSteradianErg => "D17",
            Unit::MetreKelvin => "D18",
            Unit::SquareMetreKelvinPerWatt => "D19",
            Unit::ReciprocalSecondPerSteradianMetreSquared => "D2",
            Unit::SquareMetrePerJoule => "D20",
            Unit::SquareMetrePerKilogram => "D21",
            Unit::SquareMetrePerMole => "D22",
            Unit::PenGramProtein => "D23",
            Unit::SquareMetrePerSteradian => "D24",
            Unit::SquareMetrePerSteradianJoule => "D25",
            Unit::SquareMetrePerVoltSecond => "D26",
            Unit::Steradian => "D27",
            Unit::Terahertz => "D29",
            Unit::Terajoule => "D30",
            Unit::Terawatt => "D31",
            Unit::TerawattHour => "D32",
            Unit::Tesla => "D33",
            Unit::Tex => "D34",
            Unit::Megabit => "D36",
            Unit::TonnePerCubicMetre => "D41",
            Unit::TropicalYear => "D42",
            Unit::UnifiedAtomicMassUnit => "D43",
            Unit::Var => "D44",
            Unit::VoltSquaredPerKelvinSquared => "D45",
            Unit::VoltAmpere => "D46",
            Unit::VoltPerCentimetre => "D47",
            Unit::VoltPerKelvin => "D48",
            Unit::MillivoltPerKelvin => "D49",
            Unit::KilogramPerSquareCentimetre => "D5",
            Unit::VoltPerMetre => "D50",
            Unit::VoltPerMillimetre => "D51",
            Unit::WattPerKelvin => "D52",
            Unit::WattPerMetreKelvin => "D53",
            Unit::WattPerSquareMetre => "D54",
            Unit::WattPerSquareMetreKelvin => "D55",
            Unit::WattPerSquareMetreKelvinToFourthPower => "D56",
            Unit::WattPerSteradian => "D57",
            Unit::WattPerSteradianSquareMetre => "D58",
            Unit::WeberPerMetre => "D59",
            Unit::RoentgenPerSecond => "D6",
            Unit::WeberPerMillimetre => "D60",
            Unit::MinuteUnitAngle => "D61",
            Unit::SecondUnitAngle => "D62",
            Unit::Book => "D63",
            Unit::Round => "D65",
            Unit::NumberWords => "D68",
            Unit::InchToFourthPower => "D69",
            Unit::JouleSquareMetre => "D73",
            Unit::KilogramPerMole => "D74",
            Unit::Megacoulomb => "D77",
            Unit::MegajoulePerSecond => "D78",
            Unit::Microwatt => "D80",
            Unit::Microtesla => "D81",
            Unit::Microvolt => "D82",
            Unit::MillinewtonMetre => "D83",
            Unit::MicrowattPerSquareMetre => "D85",
            Unit::Millicoulomb => "D86",
            Unit::MillimolePerKilogram => "D87",
            Unit::MillicoulombPerCubicMetre => "D88",
            Unit::MillicoulombPerSquareMetre => "D89",
            Unit::Rem => "D91",
            Unit::SecondPerCubicMetre => "D93",
            Unit::SecondPerCubicMetreRadian => "D94",
            Unit::JoulePerGram => "D95",
            Unit::Decare => "DAA",
            Unit::TenDay => "DAD",
            Unit::Day => "DAY",
            Unit::DryPound => "DB",
            Unit::DecibelMilliwatts => "DBM",
            Unit::DecibelWatt => "DBW",
            Unit::DegreeUnitAngle => "DD",
            Unit::Decade => "DEC",
            Unit::Decigram => "DG",
            Unit::Decagram => "DJ",
            Unit::Decilitre => "DLT",
            Unit::CubicDecametre => "DMA",
            Unit::SquareDecimetre => "DMK",
            Unit::StandardKilolitre => "DMO",
            Unit::CubicDecimetre => "DMQ",
            Unit::Decimetre => "DMT",
            Unit::DecinewtonMetre => "DN",
            Unit::DozenPiece => "DPC",
            Unit::DozenPair => "DPR",
            Unit::DisplacementTonnage => "DPT",
            Unit::DramUs => "DRA",
            Unit::DramUk => "DRI",
            Unit::DozenRoll => "DRL",
            Unit::DryTon => "DT",
            Unit::Decitonne => "DTN",
            Unit::Pennyweight => "DWT",
            Unit::Dozen => "DZN",
            Unit::DozenPack => "DZP",
            Unit::NewtonPerSquareCentimetre => "E01",
            Unit::MegawattHourPerHour => "E07",
            Unit::MegawattPerHertz => "E08",
            Unit::MilliampereHour => "E09",
            Unit::DegreeDay => "E10",
            Unit::Mille => "E12",
            Unit::KilocalorieInternationalTable => "E14",
            Unit::KilocalorieThermochemicalPerHour => "E15",
            Unit::MillionBtuItPerHour => "E16",
            Unit::CubicFootPerSecond => "E17",
            Unit::TonnePerHour => "E18",
            Unit::Ping => "E19",
            Unit::MegabitPerSecond => "E20",
            Unit::Shares => "E21",
            Unit::Teu => "E22",
            Unit::Tyre => "E23",
            Unit::ActiveUnit => "E25",
            Unit::Dose => "E27",
            Unit::AirDryTon => "E28",
            Unit::Strand => "E30",
            Unit::SquareMetrePerLitre => "E31",
            Unit::LitrePerHour => "E32",
            Unit::FootPerThousand => "E33",
            Unit::Gigabyte => "E34",
            Unit::Terabyte => "E35",
            Unit::Petabyte => "E36",
            Unit::Pixel => "E37",
            Unit::Megapixel => "E38",
            Unit::DotsPerInch => "E39",
            Unit::GrossKilogram => "E4",
            Unit::PartPerHundredThousand => "E40",
            Unit::KilogramForcePerSquareMillimetre => "E41",
            Unit::KilogramForcePerSquareCentimetre => "E42",
            Unit::JoulePerSquareCentimetre => "E43",
            Unit::KilogramForceMetrePerSquareCentimetre => "E44",
            Unit::Milliohm => "E45",
            Unit::KilowattHourPerCubicMetre => "E46",
            Unit::KilowattHourPerKelvin => "E47",
            Unit::ServiceUnit => "E48",
            Unit::WorkingDay => "E49",
            Unit::AccountingUnit => "E50",
            Unit::Job => "E51",
            Unit::RunFoot => "E52",
            Unit::Test => "E53",
            Unit::Trip => "E54",
            Unit::Use => "E55",
            Unit::Well => "E56",
            Unit::Zone => "E57",
            Unit::ExabitPerSecond => "E58",
            Unit::Exbibyte => "E59",
            Unit::Pebibyte => "E60",
            Unit::Tebibyte => "E61",
            Unit::Gibibyte => "E62",
            Unit::Mebibyte => "E63",
            Unit::Kibibyte => "E64",
            Unit::ExbibitPerMetre => "E65",
            Unit::ExbibitPerSquareMetre => "E66",
            Unit::ExbibitPerCubicMetre => "E67",
            Unit::GigabytePerSecond => "E68",
            Unit::GibibitPerMetre => "E69",
            Unit::GibibitPerSquareMetre => "E70",
            Unit::GibibitPerCubicMetre => "E71",
            Unit::KibibitPerMetre => "E72",
            Unit::KibibitPerSquareMetre => "E73",
            Unit::KibibitPerCubicMetre => "E74",
            Unit::MebibitPerMetre => "E75",
            Unit::MebibitPerSquareMetre => "E76",
            Unit::MebibitPerCubicMetre => "E77",
            Unit::Petabit => "E78",
            Unit::PetabitPerSecond => "E79",
            Unit::PebibitPerMetre => "E80",
            Unit::PebibitPerSquareMetre => "E81",
            Unit::PebibitPerCubicMetre => "E82",
            Unit::Terabit => "E83",
            Unit::TerabitPerSecond => "E84",
            Unit::TebibitPerMetre => "E85",
            Unit::TebibitPerCubicMetre => "E86",
            Unit::TebibitPerSquareMetre => "E87",
            Unit::BitPerMetre => "E88",
            Unit::BitPerSquareMetre => "E89",
            Unit::ReciprocalCentimetre => "E90",
            Unit::ReciprocalDay => "E91",
            Unit::CubicDecimetrePerHour => "E92",
            Unit::KilogramPerHour => "E93",
            Unit::KilomolePerSecond => "E94",
            Unit::MolePerSecond => "E95",
            Unit::DegreePerSecond => "E96",
            Unit::MillimetrePerDegreeCelciusMetre => "E97",
            Unit::DegreeCelsiusPerKelvin => "E98",
            Unit::HectopascalPerBar => "E99",
            Unit::Each => "EA",
            Unit::ElectronicMailBox => "EB",
            Unit::EquivalentGallon => "EQ",
            Unit::BitPerCubicMetre => "F01",
            Unit::KelvinPerKelvin => "F02",
            Unit::KilopascalPerBar => "F03",
            Unit::MillibarPerBar => "F04",
            Unit::MegapascalPerBar => "F05",
            Unit::PoisePerBar => "F06",
            Unit::PascalPerBar => "F07",
            Unit::MilliamperePerInch => "F08",
            Unit::KelvinPerHour => "F10",
            Unit::KelvinPerMinute => "F11",
            Unit::KelvinPerSecond => "F12",
            Unit::Slug => "F13",
            Unit::GramPerKelvin => "F14",
            Unit::KilogramPerKelvin => "F15",
            Unit::MilligramPerKelvin => "F16",
            Unit::PoundForcePerFoot => "F17",
            Unit::KilogramSquareCentimetre => "F18",
            Unit::KilogramSquareMillimetre => "F19",
            Unit::PoundInchSquared => "F20",
            Unit::PoundForceInch => "F21",
            Unit::PoundForceFootPerAmpere => "F22",
            Unit::GramPerCubicDecimetre => "F23",
            Unit::KilogramPerKilomol => "F24",
            Unit::GramPerHertz => "F25",
            Unit::GramPerDay => "F26",
            Unit::GramPerHour => "F27",
            Unit::GramPerMinute => "F28",
            Unit::GramPerSecond => "F29",
            Unit::KilogramPerDay => "F30",
            Unit::KilogramPerMinute => "F31",
            Unit::MilligramPerDay => "F32",
            Unit::MilligramPerMinute => "F33",
            Unit::MilligramPerSecond => "F34",
            Unit::GramPerDayKelvin => "F35",
            Unit::GramPerHourKelvin => "F36",
            Unit::GramPerMinuteKelvin => "F37",
            Unit::GramPerSecondKelvin => "F38",
            Unit::KilogramPerDayKelvin => "F39",
            Unit::KilogramPerHourKelvin => "F40",
            Unit::KilogramPerMinuteKelvin => "F41",
            Unit::KilogramPerSecondKelvin => "F42",
            Unit::MilligramPerDayKelvin => "F43",
            Unit::MilligramPerHourKelvin => "F44",
            Unit::MilligramPerMinuteKelvin => "F45",
            Unit::MilligramPerSecondKelvin => "F46",
            Unit::NewtonPerMillimetre => "F47",
            Unit::PoundForcePerInch => "F48",
            Unit::RodUnitDistance => "F49",
            Unit::MicrometrePerKelvin => "F50",
            Unit::CentimetrePerKelvin => "F51",
            Unit::MetrePerKelvin => "F52",
            Unit::MillimetrePerKelvin => "F53",
            Unit::MilliohmPerMetre => "F54",
            Unit::OhmPerMileStatuteMile => "F55",
            Unit::OhmPerKilometre => "F56",
            Unit::MilliamperePerPoundForcePerSquareInch => "F57",
            Unit::ReciprocalBar => "F58",
            Unit::MilliamperePerBar => "F59",
            Unit::DegreeCelsiusPerBar => "F60",
            Unit::KelvinPerBar => "F61",
            Unit::GramPerDayBar => "F62",
            Unit::GramPerHourBar => "F63",
            Unit::GramPerMinuteBar => "F64",
            Unit::GramPerSecondBar => "F65",
            Unit::KilogramPerDayBar => "F66",
            Unit::KilogramPerHourBar => "F67",
            Unit::KilogramPerMinuteBar => "F68",
            Unit::KilogramPerSecondBar => "F69",
            Unit::MilligramPerDayBar => "F70",
            Unit::MilligramPerHourBar => "F71",
            Unit::MilligramPerMinuteBar => "F72",
            Unit::MilligramPerSecondBar => "F73",
            Unit::GramPerBar => "F74",
            Unit::MilligramPerBar => "F75",
            Unit::MilliamperePerMillimetre => "F76",
            Unit::PascalSecondPerKelvin => "F77",
            Unit::InchWater => "F78",
            Unit::InchMercury => "F79",
            Unit::WaterHorsePower => "F80",
            Unit::BarPerKelvin => "F81",
            Unit::HectopascalPerKelvin => "F82",
            Unit::KilopascalPerKelvin => "F83",
            Unit::MillibarPerKelvin => "F84",
            Unit::MegapascalPerKelvin => "F85",
            Unit::PoisePerKelvin => "F86",
            Unit::VoltPerLitreMinute => "F87",
            Unit::NewtonCentimetre => "F88",
            Unit::NewtonMetrePerDegree => "F89",
            Unit::NewtonMetrePerAmpere => "F90",
            Unit::BarLitrePerSecond => "F91",
            Unit::BarCubicMetrePerSecond => "F92",
            Unit::HectopascalLitrePerSecond => "F93",
            Unit::HectopascalCubicMetrePerSecond => "F94",
            Unit::MillibarLitrePerSecond => "F95",
            Unit::MillibarCubicMetrePerSecond => "F96",
            Unit::MegapascalLitrePerSecond => "F97",
            Unit::MegapascalCubicMetrePerSecond => "F98",
            Unit::PascalLitrePerSecond => "F99",
            Unit::DegreeFahrenheit => "FAH",
            Unit::Farad => "FAR",
            Unit::FibreMetre => "FBM",
            Unit::ThousandCubicFoot => "FC",
            Unit::HundredCubicMetre => "FF",
            Unit::Micromole => "FH",
            Unit::FailuresInTime => "FIT",
            Unit::FlakeTon => "FL",
            Unit::FormazinNephelometricUnit => "FNU",
            Unit::Foot => "FOT",
            Unit::PoundPerSquareFoot => "FP",
            Unit::FootPerMinute => "FR",
            Unit::FootPerSecond => "FS",
            Unit::SquareFoot => "FTK",
            Unit::CubicFoot => "FTQ",
            Unit::PascalCubicMetrePerSecond => "G01",
            Unit::CentimetrePerBar => "G04",
            Unit::MetrePerBar => "G05",
            Unit::MillimetrePerBar => "G06",
            Unit::SquareInchPerSecond => "G08",
            Unit::SquareMetrePerSecondKelvin => "G09",
            Unit::StokesPerKelvin => "G10",
            Unit::GramPerCubicCentimetreBar => "G11",
            Unit::GramPerCubicDecimetreBar => "G12",
            Unit::GramPerLitreBar => "G13",
            Unit::GramPerCubicMetreBar => "G14",
            Unit::GramPerMillilitreBar => "G15",
            Unit::KilogramPerCubicCentimetreBar => "G16",
            Unit::KilogramPerLitreBar => "G17",
            Unit::KilogramPerCubicMetreBar => "G18",
            Unit::NewtonMetrePerKilogram => "G19",
            Unit::UsGallonPerMinute => "G2",
            Unit::PoundForceFootPerPound => "G20",
            Unit::CupUnitVolume => "G21",
            Unit::Peck => "G23",
            Unit::TablespoonUs => "G24",
            Unit::TeaspoonUs => "G25",
            Unit::Stere => "G26",
            Unit::CubicCentimetrePerKelvin => "G27",
            Unit::LitrePerKelvin => "G28",
            Unit::CubicMetrePerKelvin => "G29",
            Unit::ImperialGallonPerMinute => "G3",
            Unit::MillilitrePerKelvin => "G30",
            Unit::KilogramPerCubicCentimetre => "G31",
            Unit::OunceAvoirdupoisPerCubicYard => "G32",
            Unit::GramPerCubicCentimetreKelvin => "G33",
            Unit::GramPerCubicDecimetreKelvin => "G34",
            Unit::GramPerLitreKelvin => "G35",
            Unit::GramPerCubicMetreKelvin => "G36",
            Unit::GramPerMillilitreKelvin => "G37",
            Unit::KilogramPerCubicCentimetreKelvin => "G38",
            Unit::KilogramPerLitreKelvin => "G39",
            Unit::KilogramPerCubicMetreKelvin => "G40",
            Unit::SquareMetrePerSecondBar => "G41",
            Unit::MicrosiemensPerCentimetre => "G42",
            Unit::MicrosiemensPerMetre => "G43",
            Unit::NanosiemensPerCentimetre => "G44",
            Unit::NanosiemensPerMetre => "G45",
            Unit::StokesPerBar => "G46",
            Unit::CubicCentimetrePerDay => "G47",
            Unit::CubicCentimetrePerHour => "G48",
            Unit::CubicCentimetrePerMinute => "G49",
            Unit::GallonUsPerHour => "G50",
            Unit::LitrePerSecond => "G51",
            Unit::CubicMetrePerDay => "G52",
            Unit::CubicMetrePerMinute => "G53",
            Unit::MillilitrePerDay => "G54",
            Unit::MillilitrePerHour => "G55",
            Unit::CubicInchPerHour => "G56",
            Unit::CubicInchPerMinute => "G57",
            Unit::CubicInchPerSecond => "G58",
            Unit::MilliamperePerLitreMinute => "G59",
            Unit::VoltPerBar => "G60",
            Unit::CubicCentimetrePerDayKelvin => "G61",
            Unit::CubicCentimetrePerHourKelvin => "G62",
            Unit::CubicCentimetrePerMinuteKelvin => "G63",
            Unit::CubicCentimetrePerSecondKelvin => "G64",
            Unit::LitrePerDayKelvin => "G65",
            Unit::LitrePerHourKelvin => "G66",
            Unit::LitrePerMinuteKelvin => "G67",
            Unit::LitrePerSecondKelvin => "G68",
            Unit::CubicMetrePerDayKelvin => "G69",
            Unit::CubicMetrePerHourKelvin => "G70",
            Unit::CubicMetrePerMinuteKelvin => "G71",
            Unit::CubicMetrePerSecondKelvin => "G72",
            Unit::MillilitrePerDayKelvin => "G73",
            Unit::MillilitrePerHourKelvin => "G74",
            Unit::MillilitrePerMinuteKelvin => "G75",
            Unit::MillilitrePerSecondKelvin => "G76",
            Unit::MillimetreToFourthPower => "G77",
            Unit::CubicCentimetrePerDayBar => "G78",
            Unit::CubicCentimetrePerHourBar => "G79",
            Unit::CubicCentimetrePerMinuteBar => "G80",
            Unit::CubicCentimetrePerSecondBar => "G81",
            Unit::LitrePerDayBar => "G82",
            Unit::LitrePerHourBar => "G83",
            Unit::LitrePerMinuteBar => "G84",
            Unit::LitrePerSecondBar => "G85",
            Unit::CubicMetrePerDayBar => "G86",
            Unit::CubicMetrePerHourBar => "G87",
            Unit::CubicMetrePerMinuteBar => "G88",
            Unit::CubicMetrePerSecondBar => "G89",
            Unit::MillilitrePerDayBar => "G90",
            Unit::MillilitrePerHourBar => "G91",
            Unit::MillilitrePerMinuteBar => "G92",
            Unit::MillilitrePerSecondBar => "G93",
            Unit::CubicCentimetrePerBar => "G94",
            Unit::LitrePerBar => "G95",
            Unit::CubicMetrePerBar => "G96",
            Unit::MillilitrePerBar => "G97",
            Unit::MicrohenryPerKiloohm => "G98",
            Unit::MicrohenryPerOhm => "G99",
            Unit::GallonUsPerDay => "GB",
            Unit::Gigabecquerel => "GBQ",
            Unit::GramDryWeight => "GDW",
            Unit::PoundPerGallonUs => "GE",
            Unit::GramPerMetreGramPer100Centimetres => "GF",
            Unit::GramFissileIsotope => "GFI",
            Unit::GreatGross => "GGR",
            Unit::GillUs => "GIA",
            Unit::GramIncludingContainer => "GIC",
            Unit::GillUk => "GII",
            Unit::GramIncludingInnerPackaging => "GIP",
            Unit::GramPerMillilitre => "GJ",
            Unit::GramPerLitre => "GL",
            Unit::DryGallonUs => "GLD",
            Unit::GallonUk => "GLI",
            Unit::GallonUs => "GLL",
            Unit::GramPerSquareMetre => "GM",
            Unit::MilligramPerSquareMetre => "GO",
            Unit::MilligramPerCubicMetre => "GP",
            Unit::MicrogramPerCubicMetre => "GQ",
            Unit::Gram => "GRM",
            Unit::Grain => "GRN",
            Unit::Gross => "GRO",
            Unit::Gigajoule => "GV",
            Unit::GigawattHour => "GWH",
            Unit::HenryPerKiloohm => "H03",
            Unit::HenryPerOhm => "H04",
            Unit::MillihenryPerKiloohm => "H05",
            Unit::MillihenryPerOhm => "H06",
            Unit::PascalSecondPerBar => "H07",
            Unit::Microbecquerel => "H08",
            Unit::ReciprocalYear => "H09",
            Unit::ReciprocalHour => "H10",
            Unit::ReciprocalMonth => "H11",
            Unit::DegreeCelsiusPerHour => "H12",
            Unit::DegreeCelsiusPerMinute => "H13",
            Unit::DegreeCelsiusPerSecond => "H14",
            Unit::SquareCentimetrePerGram => "H15",
            Unit::SquareDecametre => "H16",
            Unit::SquareHectometre => "H18",
            Unit::CubicHectometre => "H19",
            Unit::CubicKilometre => "H20",
            Unit::Blank => "H21",
            Unit::VoltSquareInchPerPoundForce => "H22",
            Unit::VoltPerInch => "H23",
            Unit::VoltPerMicrosecond => "H24",
            Unit::PercentPerKelvin => "H25",
            Unit::OhmPerMetre => "H26",
            Unit::DegreePerMetre => "H27",
            Unit::MicrofaradPerKilometre => "H28",
            Unit::MicrogramPerLitre => "H29",
            Unit::SquareMicrometreSquareMicron => "H30",
            Unit::AmperePerKilogram => "H31",
            Unit::AmpereSquaredSecond => "H32",
            Unit::FaradPerKilometre => "H33",
            Unit::HertzMetre => "H34",
            Unit::KelvinMetrePerWatt => "H35",
            Unit::MegaohmPerKilometre => "H36",
            Unit::MegaohmPerMetre => "H37",
            Unit::Megaampere => "H38",
            Unit::MegahertzKilometre => "H39",
            Unit::NewtonPerAmpere => "H40",
            Unit::NewtonMetreWattToPowerMinus05 => "H41",
            Unit::PascalPerMetre => "H42",
            Unit::SiemensPerCentimetre => "H43",
            Unit::Teraohm => "H44",
            Unit::VoltSecondPerMetre => "H45",
            Unit::VoltPerSecond => "H46",
            Unit::WattPerCubicMetre => "H47",
            Unit::Attofarad => "H48",
            Unit::CentimetrePerHour => "H49",
            Unit::ReciprocalCubicCentimetre => "H50",
            Unit::DecibelPerKilometre => "H51",
            Unit::DecibelPerMetre => "H52",
            Unit::KilogramPerBar => "H53",
            Unit::KilogramPerCubicDecimetreKelvin => "H54",
            Unit::KilogramPerCubicDecimetreBar => "H55",
            Unit::KilogramPerSquareMetreSecond => "H56",
            Unit::InchPerTwoPiRadiant => "H57",
            Unit::MetrePerVoltSecond => "H58",
            Unit::SquareMetrePerNewton => "H59",
            Unit::CubicMetrePerCubicMetre => "H60",
            Unit::MillisiemensPerCentimetre => "H61",
            Unit::MillivoltPerMinute => "H62",
            Unit::MilligramPerSquareCentimetre => "H63",
            Unit::MilligramPerGram => "H64",
            Unit::MillilitrePerCubicMetre => "H65",
            Unit::MillimetrePerYear => "H66",
            Unit::MillimetrePerHour => "H67",
            Unit::MillimolePerGram => "H68",
            Unit::PicopascalPerKilometre => "H69",
            Unit::Picosecond => "H70",
            Unit::PercentPerMonth => "H71",
            Unit::PercentPerHectobar => "H72",
            Unit::PercentPerDecakelvin => "H73",
            Unit::WattPerMetre => "H74",
            Unit::Decapascal => "H75",
            Unit::GramPerMillimetre => "H76",
            Unit::ModuleWidth => "H77",
            Unit::FrenchGauge => "H79",
            Unit::RackUnit => "H80",
            Unit::MillimetrePerMinute => "H81",
            Unit::BigPoint => "H82",
            Unit::LitrePerKilogram => "H83",
            Unit::GramMillimetre => "H84",
            Unit::ReciprocalWeek => "H85",
            Unit::Piece => "H87",
            Unit::MegaohmKilometre => "H88",
            Unit::PercentPerOhm => "H89",
            Unit::PercentPerDegree => "H90",
            Unit::PercentPerTenThousand => "H91",
            Unit::PercentPerOneHundredThousand => "H92",
            Unit::PercentPerHundred => "H93",
            Unit::PercentPerThousand => "H94",
            Unit::PercentPerVolt => "H95",
            Unit::PercentPerBar => "H96",
            Unit::PercentPerInch => "H98",
            Unit::PercentPerMetre => "H99",
            Unit::Hank => "HA",
            Unit::PieceDay => "HAD",
            Unit::Hectobar => "HBA",
            Unit::HundredBoxes => "HBX",
            Unit::HundredCount => "HC",
            Unit::HundredKilogramDryWeight => "HDW",
            Unit::Head => "HEA",
            Unit::Hectogram => "HGM",
            Unit::HundredCubicFoot => "HH",
            Unit::HundredInternationalUnit => "HIU",
            Unit::HundredKilogramNetMass => "HKM",
            Unit::Hectolitre => "HLT",
            Unit::MilePerHourStatuteMile => "HM",
            Unit::PieceMonth => "HMO",
            Unit::MillionCubicMetre => "HMQ",
            Unit::Hectometre => "HMT",
            Unit::HectolitrePureAlcohol => "HPA",
            Unit::Hertz => "HTZ",
            Unit::Hour => "HUR",
            Unit::PieceWeek => "HWE",
            Unit::InchPoundPoundInch => "IA",
            Unit::Person => "IE",
            Unit::Inch => "INH",
            Unit::SquareInch => "INK",
            Unit::CubicInch => "INQ",
            Unit::InternationalSugarDegree => "ISD",
            Unit::InchPerSecond => "IU",
            Unit::InternationalUnitPerGram => "IUG",
            Unit::InchPerSecondSquared => "IV",
            Unit::PercentPerMillimetre => "J10",
            Unit::PerMillePerPsi => "J12",
            Unit::DegreeApi => "J13",
            Unit::DegreeBaumeOriginScale => "J14",
            Unit::DegreeBaumeUsHeavy => "J15",
            Unit::DegreeBaumeUsLight => "J16",
            Unit::DegreeBalling => "J17",
            Unit::DegreeBrix => "J18",
            Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitThermochemical => "J19",
            Unit::JoulePerKilogram => "J2",
            Unit::DegreeFahrenheitPerKelvin => "J20",
            Unit::DegreeFahrenheitPerBar => "J21",
            Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitInternationalTable => "J22",
            Unit::DegreeFahrenheitPerHour => "J23",
            Unit::DegreeFahrenheitPerMinute => "J24",
            Unit::DegreeFahrenheitPerSecond => "J25",
            Unit::ReciprocalDegreeFahrenheit => "J26",
            Unit::DegreeOechsle => "J27",
            Unit::DegreeRankinePerHour => "J28",
            Unit::DegreeRankinePerMinute => "J29",
            Unit::DegreeRankinePerSecond => "J30",
            Unit::DegreeTwaddell => "J31",
            Unit::Micropoise => "J32",
            Unit::MicrogramPerKilogram => "J33",
            Unit::MicrogramPerCubicMetreKelvin => "J34",
            Unit::MicrogramPerCubicMetreBar => "J35",
            Unit::MicrolitrePerLitre => "J36",
            Unit::Baud => "J38",
            Unit::BritishThermalUnitMean => "J39",
            Unit::BritishThermalUnitInternationalTableFootPerHourSquareFootDegreeFahrenheit => {
                "J40"
            }
            Unit::BritishThermalUnitInternationalTableInchPerHourSquareFootDegreeFahrenheit => {
                "J41"
            }
            Unit::BritishThermalUnitInternationalTableInchPerSecondSquareFootDegreeFahrenheit => {
                "J42"
            }
            Unit::BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit => "J43",
            Unit::BritishThermalUnitInternationalTablePerMinute => "J44",
            Unit::BritishThermalUnitInternationalTablePerSecond => "J45",
            Unit::BritishThermalUnitThermochemicalFootPerHourSquareFootDegreeFahrenheit => "J46",
            Unit::BritishThermalUnitThermochemicalPerHour => "J47",
            Unit::BritishThermalUnitThermochemicalInchPerHourSquareFootDegreeFahrenheit => "J48",
            Unit::BritishThermalUnitThermochemicalInchPerSecondSquareFootDegreeFahrenheit => "J49",
            Unit::BritishThermalUnitThermochemicalPerPoundDegreeFahrenheit => "J50",
            Unit::BritishThermalUnitThermochemicalPerMinute => "J51",
            Unit::BritishThermalUnitThermochemicalPerSecond => "J52",
            Unit::CoulombSquareMetrePerKilogram => "J53",
            Unit::Megabaud => "J54",
            Unit::WattSecond => "J55",
            Unit::BarPerBar => "J56",
            Unit::BarrelUkPetroleum => "J57",
            Unit::BarrelUkPetroleumPerMinute => "J58",
            Unit::BarrelUkPetroleumPerDay => "J59",
            Unit::BarrelUkPetroleumPerHour => "J60",
            Unit::BarrelUkPetroleumPerSecond => "J61",
            Unit::BarrelUsPetroleumPerHour => "J62",
            Unit::BarrelUsPetroleumPerSecond => "J63",
            Unit::BushelUkPerDay => "J64",
            Unit::BushelUkPerHour => "J65",
            Unit::BushelUkPerMinute => "J66",
            Unit::BushelUkPerSecond => "J67",
            Unit::BushelUsDryPerDay => "J68",
            Unit::BushelUsDryPerHour => "J69",
            Unit::BushelUsDryPerMinute => "J70",
            Unit::BushelUsDryPerSecond => "J71",
            Unit::CentinewtonMetre => "J72",
            Unit::CentipoisePerKelvin => "J73",
            Unit::CentipoisePerBar => "J74",
            Unit::CalorieMean => "J75",
            Unit::CalorieInternationalTablePerGramDegreeCelsius => "J76",
            Unit::CalorieThermochemicalPerCentimetreSecondDegreeCelsius => "J78",
            Unit::CalorieThermochemicalPerGramDegreeCelsius => "J79",
            Unit::CalorieThermochemicalPerMinute => "J81",
            Unit::CalorieThermochemicalPerSecond => "J82",
            Unit::Clo => "J83",
            Unit::CentimetrePerSecondKelvin => "J84",
            Unit::CentimetrePerSecondBar => "J85",
            Unit::CubicCentimetrePerCubicMetre => "J87",
            Unit::CubicDecimetrePerDay => "J90",
            Unit::CubicDecimetrePerCubicMetre => "J91",
            Unit::CubicDecimetrePerMinute => "J92",
            Unit::CubicDecimetrePerSecond => "J93",
            Unit::OunceUkFluidPerDay => "J95",
            Unit::OunceUkFluidPerHour => "J96",
            Unit::OunceUkFluidPerMinute => "J97",
            Unit::OunceUkFluidPerSecond => "J98",
            Unit::OunceUsFluidPerDay => "J99",
            Unit::JoulePerKelvin => "JE",
            Unit::MegajoulePerKilogram => "JK",
            Unit::MegajoulePerCubicMetre => "JM",
            Unit::PipelineJoint => "JNT",
            Unit::Joule => "JOU",
            Unit::HundredMetre => "JPS",
            Unit::NumberJewels => "JWL",
            Unit::KilowattDemand => "K1",
            Unit::OunceUsFluidPerHour => "K10",
            Unit::OunceUsFluidPerMinute => "K11",
            Unit::OunceUsFluidPerSecond => "K12",
            Unit::FootPerDegreeFahrenheit => "K13",
            Unit::FootPerHour => "K14",
            Unit::FootPoundForcePerHour => "K15",
            Unit::FootPoundForcePerMinute => "K16",
            Unit::FootPerPsi => "K17",
            Unit::FootPerSecondDegreeFahrenheit => "K18",
            Unit::FootPerSecondPsi => "K19",
            Unit::KilovoltAmpereReactiveDemand => "K2",
            Unit::ReciprocalCubicFoot => "K20",
            Unit::CubicFootPerDegreeFahrenheit => "K21",
            Unit::CubicFootPerDay => "K22",
            Unit::CubicFootPerPsi => "K23",
            Unit::GallonUkPerDay => "K26",
            Unit::GallonUkPerHour => "K27",
            Unit::GallonUkPerSecond => "K28",
            Unit::KilovoltAmpereReactiveHour => "K3",
            Unit::GallonUsLiquidPerSecond => "K30",
            Unit::GramForcePerSquareCentimetre => "K31",
            Unit::GillUkPerDay => "K32",
            Unit::GillUkPerHour => "K33",
            Unit::GillUkPerMinute => "K34",
            Unit::GillUkPerSecond => "K35",
            Unit::GillUsPerDay => "K36",
            Unit::GillUsPerHour => "K37",
            Unit::GillUsPerMinute => "K38",
            Unit::GillUsPerSecond => "K39",
            Unit::StandardAccelerationFreeFall => "K40",
            Unit::GrainPerGallonUs => "K41",
            Unit::HorsepowerBoiler => "K42",
            Unit::HorsepowerElectric => "K43",
            Unit::InchPerDegreeFahrenheit => "K45",
            Unit::InchPerPsi => "K46",
            Unit::InchPerSecondDegreeFahrenheit => "K47",
            Unit::InchPerSecondPsi => "K48",
            Unit::ReciprocalCubicInch => "K49",
            Unit::Kilobaud => "K50",
            Unit::KilocalorieMean => "K51",
            Unit::KilocalorieInternationalTablePerHourMetreDegreeCelsius => "K52",
            Unit::KilocalorieThermochemical => "K53",
            Unit::KilocalorieThermochemicalPerMinute => "K54",
            Unit::KilocalorieThermochemicalPerSecond => "K55",
            Unit::KilomolePerHour => "K58",
            Unit::KilomolePerCubicMetreKelvin => "K59",
            Unit::Kilolitre => "K6",
            Unit::KilomolePerCubicMetreBar => "K60",
            Unit::KilomolePerMinute => "K61",
            Unit::LitrePerLitre => "K62",
            Unit::ReciprocalLitre => "K63",
            Unit::PoundAvoirdupoisPerDegreeFahrenheit => "K64",
            Unit::PoundAvoirdupoisSquareFoot => "K65",
            Unit::PoundAvoirdupoisPerDay => "K66",
            Unit::PoundPerFootHour => "K67",
            Unit::PoundPerFootSecond => "K68",
            Unit::PoundAvoirdupoisPerCubicFootDegreeFahrenheit => "K69",
            Unit::PoundAvoirdupoisPerCubicFootPsi => "K70",
            Unit::PoundAvoirdupoisPerGallonUk => "K71",
            Unit::PoundAvoirdupoisPerHourDegreeFahrenheit => "K73",
            Unit::PoundAvoirdupoisPerHourPsi => "K74",
            Unit::PoundAvoirdupoisPerCubicInchDegreeFahrenheit => "K75",
            Unit::PoundAvoirdupoisPerCubicInchPsi => "K76",
            Unit::PoundAvoirdupoisPerPsi => "K77",
            Unit::PoundAvoirdupoisPerMinute => "K78",
            Unit::PoundAvoirdupoisPerMinuteDegreeFahrenheit => "K79",
            Unit::PoundAvoirdupoisPerMinutePsi => "K80",
            Unit::PoundAvoirdupoisPerSecond => "K81",
            Unit::PoundAvoirdupoisPerSecondDegreeFahrenheit => "K82",
            Unit::PoundAvoirdupoisPerSecondPsi => "K83",
            Unit::PoundPerCubicYard => "K84",
            Unit::PoundForcePerSquareFoot => "K85",
            Unit::PoundForcePerSquareInchDegreeFahrenheit => "K86",
            Unit::PsiCubicInchPerSecond => "K87",
            Unit::PsiLitrePerSecond => "K88",
            Unit::PsiCubicMetrePerSecond => "K89",
            Unit::PsiCubicYardPerSecond => "K90",
            Unit::PoundForceSecondPerSquareFoot => "K91",
            Unit::PoundForceSecondPerSquareInch => "K92",
            Unit::ReciprocalPsi => "K93",
            Unit::QuartUkLiquidPerDay => "K94",
            Unit::QuartUkLiquidPerHour => "K95",
            Unit::QuartUkLiquidPerMinute => "K96",
            Unit::QuartUkLiquidPerSecond => "K97",
            Unit::QuartUsLiquidPerDay => "K98",
            Unit::QuartUsLiquidPerHour => "K99",
            Unit::Cake => "KA",
            Unit::Katal => "KAT",
            Unit::Kilocharacter => "KB",
            Unit::Kilobar => "KBA",
            Unit::KilogramCholineChloride => "KCC",
            Unit::KilogramDrainedNetWeight => "KDW",
            Unit::Kelvin => "KEL",
            Unit::Kilogram => "KGM",
            Unit::KilogramPerSecond => "KGS",
            Unit::KilogramHydrogenPeroxide => "KHY",
            Unit::Kilohertz => "KHZ",
            Unit::KilogramPerMillimetreWidth => "KI",
            Unit::KilogramIncludingContainer => "KIC",
            Unit::KilogramIncludingInnerPackaging => "KIP",
            Unit::Kilosegment => "KJ",
            Unit::Kilojoule => "KJO",
            Unit::KilogramPerMetre => "KL",
            Unit::LacticDryMaterialPercentage => "KLK",
            Unit::Kilolux => "KLX",
            Unit::KilogramMethylamine => "KMA",
            Unit::KilometrePerHour => "KMH",
            Unit::SquareKilometre => "KMK",
            Unit::KilogramPerCubicMetre => "KMQ",
            Unit::Kilometre => "KMT",
            Unit::KilogramNitrogen => "KNI",
            Unit::KilonewtonPerSquareMetre => "KNM",
            Unit::KilogramNamedSubstance => "KNS",
            Unit::Knot => "KNT",
            Unit::MilliequivalenceCausticPotashPerGramProduct => "KO",
            Unit::Kilopascal => "KPA",
            Unit::KilogramPotassiumHydroxideCausticPotash => "KPH",
            Unit::KilogramPotassiumOxide => "KPO",
            Unit::KilogramPhosphorusPentoxidePhosphoricAnhydride => "KPP",
            Unit::Kiloroentgen => "KR",
            Unit::KilogramSubstance90Dry => "KSD",
            Unit::KilogramSodiumHydroxideCausticSoda => "KSH",
            Unit::Kit => "KT",
            Unit::Kilotonne => "KTN",
            Unit::KilogramUranium => "KUR",
            Unit::KilovoltAmpere => "KVA",
            Unit::Kilovar => "KVR",
            Unit::Kilovolt => "KVT",
            Unit::KilogramPerMillimetre => "KW",
            Unit::KilowattHour => "KWH",
            Unit::KilowattHourPerNormalizedCubicMetre => "KWN",
            Unit::KilogramTungstenTrioxide => "KWO",
            Unit::KilowattHourPerStandardCubicMetre => "KWS",
            Unit::Kilowatt => "KWT",
            Unit::KilowattYear => "KWY",
            Unit::MillilitrePerKilogram => "KX",
            Unit::QuartUsLiquidPerMinute => "L10",
            Unit::QuartUsLiquidPerSecond => "L11",
            Unit::MetrePerSecondKelvin => "L12",
            Unit::MetrePerSecondBar => "L13",
            Unit::SquareMetreHourDegreeCelsiusPerKilocalorieInternationalTable => "L14",
            Unit::MillipascalSecondPerKelvin => "L15",
            Unit::MillipascalSecondPerBar => "L16",
            Unit::MilligramPerCubicMetreKelvin => "L17",
            Unit::MilligramPerCubicMetreBar => "L18",
            Unit::MillilitrePerLitre => "L19",
            Unit::LitrePerMinute => "L2",
            Unit::ReciprocalCubicMillimetre => "L20",
            Unit::CubicMillimetrePerCubicMetre => "L21",
            Unit::MolePerHour => "L23",
            Unit::MolePerKilogramKelvin => "L24",
            Unit::MolePerKilogramBar => "L25",
            Unit::MolePerLitreKelvin => "L26",
            Unit::MolePerLitreBar => "L27",
            Unit::MolePerCubicMetreKelvin => "L28",
            Unit::MolePerCubicMetreBar => "L29",
            Unit::MolePerMinute => "L30",
            Unit::MilliroentgenAequivalentMen => "L31",
            Unit::NanogramPerKilogram => "L32",
            Unit::OunceAvoirdupoisPerDay => "L33",
            Unit::OunceAvoirdupoisPerHour => "L34",
            Unit::OunceAvoirdupoisPerMinute => "L35",
            Unit::OunceAvoirdupoisPerSecond => "L36",
            Unit::OunceAvoirdupoisPerGallonUk => "L37",
            Unit::OunceAvoirdupoisPerGallonUs => "L38",
            Unit::OunceAvoirdupoisPerCubicInch => "L39",
            Unit::OunceAvoirdupoisForce => "L40",
            Unit::OunceAvoirdupoisForceInch => "L41",
            Unit::PicosiemensPerMetre => "L42",
            Unit::PeckUk => "L43",
            Unit::PeckUkPerDay => "L44",
            Unit::PeckUkPerHour => "L45",
            Unit::PeckUkPerMinute => "L46",
            Unit::PeckUkPerSecond => "L47",
            Unit::PeckUsDryPerDay => "L48",
            Unit::PeckUsDryPerHour => "L49",
            Unit::PeckUsDryPerMinute => "L50",
            Unit::PeckUsDryPerSecond => "L51",
            Unit::PsiPerPsi => "L52",
            Unit::PintUkPerDay => "L53",
            Unit::PintUkPerHour => "L54",
            Unit::PintUkPerMinute => "L55",
            Unit::PintUkPerSecond => "L56",
            Unit::PintUsLiquidPerDay => "L57",
            Unit::PintUsLiquidPerHour => "L58",
            Unit::PintUsLiquidPerMinute => "L59",
            Unit::PintUsLiquidPerSecond => "L60",
            Unit::SlugPerDay => "L63",
            Unit::SlugPerFootSecond => "L64",
            Unit::SlugPerCubicFoot => "L65",
            Unit::SlugPerHour => "L66",
            Unit::SlugPerMinute => "L67",
            Unit::SlugPerSecond => "L68",
            Unit::TonnePerKelvin => "L69",
            Unit::TonnePerBar => "L70",
            Unit::TonnePerDay => "L71",
            Unit::TonnePerDayKelvin => "L72",
            Unit::TonnePerDayBar => "L73",
            Unit::TonnePerHourKelvin => "L74",
            Unit::TonnePerHourBar => "L75",
            Unit::TonnePerCubicMetreKelvin => "L76",
            Unit::TonnePerCubicMetreBar => "L77",
            Unit::TonnePerMinute => "L78",
            Unit::TonnePerMinuteKelvin => "L79",
            Unit::TonnePerMinuteBar => "L80",
            Unit::TonnePerSecond => "L81",
            Unit::TonnePerSecondKelvin => "L82",
            Unit::TonnePerSecondBar => "L83",
            Unit::TonUkShipping => "L84",
            Unit::TonLongPerDay => "L85",
            Unit::TonUsShipping => "L86",
            Unit::TonShortPerDegreeFahrenheit => "L87",
            Unit::TonShortPerDay => "L88",
            Unit::TonShortPerHourDegreeFahrenheit => "L89",
            Unit::TonShortPerHourPsi => "L90",
            Unit::TonShortPerPsi => "L91",
            Unit::TonUkLongPerCubicYard => "L92",
            Unit::TonUsShortPerCubicYard => "L93",
            Unit::TonForceUsShort => "L94",
            Unit::CommonYear => "L95",
            Unit::SiderealYear => "L96",
            Unit::YardPerDegreeFahrenheit => "L98",
            Unit::YardPerPsi => "L99",
            Unit::PoundPerCubicInch => "LA",
            Unit::LactoseExcessPercentage => "LAC",
            Unit::Pound => "LBR",
            Unit::TroyPoundUs => "LBT",
            Unit::LitrePerDay => "LD",
            Unit::Leaf => "LEF",
            Unit::LinearFoot => "LF",
            Unit::LabourHour => "LH",
            Unit::Link => "LK",
            Unit::LinearMetre => "LM",
            Unit::Length => "LN",
            Unit::LotUnitProcurement => "LO",
            Unit::LiquidPound => "LP",
            Unit::LitrePureAlcohol => "LPA",
            Unit::Layer => "LR",
            Unit::LumpSum => "LS",
            Unit::TonUkOrLongTonUs => "LTN",
            Unit::Litre => "LTR",
            Unit::MetricTonLubricatingOil => "LUB",
            Unit::Lumen => "LUM",
            Unit::Lux => "LUX",
            Unit::LinearYard => "LY",
            Unit::MilligramPerLitre => "M1",
            Unit::ReciprocalCubicYard => "M10",
            Unit::CubicYardPerDegreeFahrenheit => "M11",
            Unit::CubicYardPerDay => "M12",
            Unit::CubicYardPerHour => "M13",
            Unit::CubicYardPerPsi => "M14",
            Unit::CubicYardPerMinute => "M15",
            Unit::CubicYardPerSecond => "M16",
            Unit::KilohertzMetre => "M17",
            Unit::GigahertzMetre => "M18",
            Unit::Beaufort => "M19",
            Unit::ReciprocalMegakelvinOrMegakelvinToPowerMinusOne => "M20",
            Unit::ReciprocalKilovoltAmpereReciprocalHour => "M21",
            Unit::MillilitrePerSquareCentimetreMinute => "M22",
            Unit::NewtonPerCentimetre => "M23",
            Unit::OhmKilometre => "M24",
            Unit::PercentPerDegreeCelsius => "M25",
            Unit::GigaohmPerMetre => "M26",
            Unit::MegahertzMetre => "M27",
            Unit::KilogramPerKilogram => "M29",
            Unit::ReciprocalVoltAmpereReciprocalSecond => "M30",
            Unit::KilogramPerKilometre => "M31",
            Unit::PascalSecondPerLitre => "M32",
            Unit::MillimolePerLitre => "M33",
            Unit::NewtonMetrePerSquareMetre => "M34",
            Unit::MillivoltAmpere => "M35",
            Unit::_30DayMonth => "M36",
            Unit::Actual360 => "M37",
            Unit::KilometrePerSecondSquared => "M38",
            Unit::CentimetrePerSecondSquared => "M39",
            Unit::MonetaryValue => "M4",
            Unit::YardPerSecondSquared => "M40",
            Unit::MillimetrePerSecondSquared => "M41",
            Unit::MileStatuteMilePerSecondSquared => "M42",
            Unit::Mil => "M43",
            Unit::Revolution => "M44",
            Unit::DegreeUnitAnglePerSecondSquared => "M45",
            Unit::RevolutionPerMinute => "M46",
            Unit::CircularMil => "M47",
            Unit::SquareMileBasedOnUSSurveyFoot => "M48",
            Unit::ChainBasedOnUSSurveyFoot => "M49",
            Unit::Microcurie => "M5",
            Unit::Furlong => "M50",
            Unit::FootUSSurvey => "M51",
            Unit::MileBasedOnUSSurveyFoot => "M52",
            Unit::MetrePerPascal => "M53",
            Unit::MetrePerRadiant => "M55",
            Unit::Shake => "M56",
            Unit::MilePerMinute => "M57",
            Unit::MilePerSecond => "M58",
            Unit::MetrePerSecondPascal => "M59",
            Unit::MetrePerHour => "M60",
            Unit::InchPerYear => "M61",
            Unit::KilometrePerSecond => "M62",
            Unit::InchPerMinute => "M63",
            Unit::YardPerSecond => "M64",
            Unit::YardPerMinute => "M65",
            Unit::YardPerHour => "M66",
            Unit::AcreFootBasedOnUSSurveyFoot => "M67",
            Unit::Cord128Ft3 => "M68",
            Unit::CubicMileUkStatute => "M69",
            Unit::MicroInch => "M7",
            Unit::TonRegister => "M70",
            Unit::CubicMetrePerPascal => "M71",
            Unit::Bel => "M72",
            Unit::KilogramPerCubicMetrePascal => "M73",
            Unit::KilogramPerPascal => "M74",
            Unit::KilopoundForce => "M75",
            Unit::Poundal => "M76",
            Unit::KilogramMetrePerSecondSquared => "M77",
            Unit::Pond => "M78",
            Unit::SquareFootPerHour => "M79",
            Unit::StokesPerPascal => "M80",
            Unit::SquareCentimetrePerSecond => "M81",
            Unit::SquareMetrePerSecondPascal => "M82",
            Unit::Denier_Dup => "M83",
            Unit::PoundPerYard => "M84",
            Unit::TonAssay => "M85",
            Unit::Pfund => "M86",
            Unit::KilogramPerSecondPascal => "M87",
            Unit::TonnePerMonth => "M88",
            Unit::TonnePerYear => "M89",
            Unit::MillionBtuPer1000CubicFoot => "M9",
            Unit::KilopoundPerHour => "M90",
            Unit::PoundPerPound => "M91",
            Unit::PoundForceFoot => "M92",
            Unit::NewtonMetrePerRadian => "M93",
            Unit::KilogramMetre => "M94",
            Unit::PoundalFoot => "M95",
            Unit::PoundalInch => "M96",
            Unit::DyneMetre => "M97",
            Unit::KilogramCentimetrePerSecond => "M98",
            Unit::GramCentimetrePerSecond => "M99",
            Unit::MegavoltAmpereReactiveHour => "MAH",
            Unit::Megalitre => "MAL",
            Unit::Megametre => "MAM",
            Unit::Megavar => "MAR",
            Unit::Megawatt => "MAW",
            Unit::ThousandStandardBrickEquivalent => "MBE",
            Unit::ThousandBoardFoot => "MBF",
            Unit::Millibar => "MBR",
            Unit::Microgram => "MC",
            Unit::Millicurie => "MCU",
            Unit::AirDryMetricTon => "MD",
            Unit::Milligram => "MGM",
            Unit::Megahertz => "MHZ",
            Unit::SquareMileStatuteMile => "MIK",
            Unit::Thousand => "MIL",
            Unit::MinuteUnitTime => "MIN",
            Unit::Million => "MIO",
            Unit::MillionInternationalUnit => "MIU",
            Unit::SquareMetreDay => "MKD",
            Unit::SquareMetreMonth => "MKM",
            Unit::SquareMetreWeek => "MKW",
            Unit::Milliard => "MLD",
            Unit::Millilitre => "MLT",
            Unit::SquareMillimetre => "MMK",
            Unit::CubicMillimetre => "MMQ",
            Unit::Millimetre => "MMT",
            Unit::KilogramDryWeight => "MND",
            Unit::MegaJoulePerNormalisedCubicMetre => "MNJ",
            Unit::Month => "MON",
            Unit::Megapascal => "MPA",
            Unit::CubicMetreDay => "MQD",
            Unit::CubicMetrePerHour => "MQH",
            Unit::CubicMetreMonth => "MQM",
            Unit::CubicMetrePerSecond => "MQS",
            Unit::CubicMetreWeek => "MQW",
            Unit::MetreDay => "MRD",
            Unit::MetreMonth => "MRM",
            Unit::MetreWeek => "MRW",
            Unit::MetrePerSecondSquared => "MSK",
            Unit::SquareMetre => "MTK",
            Unit::CubicMetre => "MTQ",
            Unit::Metre => "MTR",
            Unit::MetrePerSecond => "MTS",
            Unit::Milihertz => "MTZ",
            Unit::MegavoltAmpere => "MVA",
            Unit::MegawattHour1000KwH => "MWH",
            Unit::PenCalorie => "N1",
            Unit::PoundFootPerSecond => "N10",
            Unit::PoundInchPerSecond => "N11",
            Unit::Pferdestaerke => "N12",
            Unit::CentimetreMercury0Oc => "N13",
            Unit::CentimetreWater4Oc => "N14",
            Unit::FootWater392Of => "N15",
            Unit::InchMercury32Of => "N16",
            Unit::InchMercury60Of => "N17",
            Unit::InchWater392Of => "N18",
            Unit::InchWater60Of => "N19",
            Unit::KipPerSquareInch => "N20",
            Unit::PoundalPerSquareFoot => "N21",
            Unit::OunceAvoirdupoisPerSquareInch => "N22",
            Unit::ConventionalMetreWater => "N23",
            Unit::GramPerSquareMillimetre => "N24",
            Unit::PoundPerSquareYard => "N25",
            Unit::PoundalPerSquareInch => "N26",
            Unit::FootToFourthPower => "N27",
            Unit::CubicDecimetrePerKilogram => "N28",
            Unit::CubicFootPerPound => "N29",
            Unit::PrintPoint => "N3",
            Unit::CubicInchPerPound => "N30",
            Unit::KilonewtonPerMetre => "N31",
            Unit::PoundalPerInch => "N32",
            Unit::PoundForcePerYard => "N33",
            Unit::PoundalSecondPerSquareFoot => "N34",
            Unit::PoisePerPascal => "N35",
            Unit::NewtonSecondPerSquareMetre => "N36",
            Unit::KilogramPerMetreSecond => "N37",
            Unit::KilogramPerMetreMinute => "N38",
            Unit::KilogramPerMetreDay => "N39",
            Unit::KilogramPerMetreHour => "N40",
            Unit::GramPerCentimetreSecond => "N41",
            Unit::PoundalSecondPerSquareInch => "N42",
            Unit::PoundPerFootMinute => "N43",
            Unit::PoundPerFootDay => "N44",
            Unit::CubicMetrePerSecondPascal => "N45",
            Unit::FootPoundal => "N46",
            Unit::InchPoundal => "N47",
            Unit::WattPerSquareCentimetre => "N48",
            Unit::WattPerSquareInch => "N49",
            Unit::BritishThermalUnitInternationalTablePerSquareFootHour => "N50",
            Unit::BritishThermalUnitThermochemicalPerSquareFootHour => "N51",
            Unit::BritishThermalUnitThermochemicalPerSquareFootMinute => "N52",
            Unit::BritishThermalUnitInternationalTablePerSquareFootSecond => "N53",
            Unit::BritishThermalUnitThermochemicalPerSquareFootSecond => "N54",
            Unit::BritishThermalUnitInternationalTablePerSquareInchSecond => "N55",
            Unit::CalorieThermochemicalPerSquareCentimetreMinute => "N56",
            Unit::CalorieThermochemicalPerSquareCentimetreSecond => "N57",
            Unit::BritishThermalUnitInternationalTablePerCubicFoot => "N58",
            Unit::BritishThermalUnitThermochemicalPerCubicFoot => "N59",
            Unit::BritishThermalUnitInternationalTablePerDegreeFahrenheit => "N60",
            Unit::BritishThermalUnitThermochemicalPerDegreeFahrenheit => "N61",
            Unit::BritishThermalUnitInternationalTablePerDegreeRankine => "N62",
            Unit::BritishThermalUnitThermochemicalPerDegreeRankine => "N63",
            Unit::BritishThermalUnitThermochemicalPerPoundDegreeRankine => "N64",
            Unit::KilocalorieInternationalTablePerGramKelvin => "N65",
            Unit::BritishThermalUnit39Of => "N66",
            Unit::BritishThermalUnit59Of => "N67",
            Unit::BritishThermalUnit60Of => "N68",
            Unit::Calorie20Oc => "N69",
            Unit::Quad1015Btuit => "N70",
            Unit::ThermEc => "N71",
            Unit::ThermUS => "N72",
            Unit::BritishThermalUnitThermochemicalPerPound => "N73",
            Unit::BritishThermalUnitInternationalTablePerHourSquareFootDegreeFahrenheit => "N74",
            Unit::BritishThermalUnitThermochemicalPerHourSquareFootDegreeFahrenheit => "N75",
            Unit::BritishThermalUnitInternationalTablePerSecondSquareFootDegreeFahrenheit => "N76",
            Unit::BritishThermalUnitThermochemicalPerSecondSquareFootDegreeFahrenheit => "N77",
            Unit::KilowattPerSquareMetreKelvin => "N78",
            Unit::KelvinPerPascal => "N79",
            Unit::WattPerMetreDegreeCelsius => "N80",
            Unit::KilowattPerMetreKelvin => "N81",
            Unit::KilowattPerMetreDegreeCelsius => "N82",
            Unit::MetrePerDegreeCelciusMetre => "N83",
            Unit::DegreeFahrenheitHourPerBritishThermalUnitInternationalTable => "N84",
            Unit::DegreeFahrenheitHourPerBritishThermalUnitThermochemical => "N85",
            Unit::DegreeFahrenheitSecondPerBritishThermalUnitInternationalTable => "N86",
            Unit::DegreeFahrenheitSecondPerBritishThermalUnitThermochemical => "N87",
            Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitInternationalTableInch => {
                "N88"
            }
            Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitThermochemicalInch => "N89",
            Unit::Kilofarad => "N90",
            Unit::ReciprocalJoule => "N91",
            Unit::Picosiemens => "N92",
            Unit::AmperePerPascal => "N93",
            Unit::Franklin => "N94",
            Unit::AmpereMinute => "N95",
            Unit::Biot => "N96",
            Unit::Gilbert => "N97",
            Unit::VoltPerPascal => "N98",
            Unit::Picovolt => "N99",
            Unit::MilligramPerKilogram => "nan",
            Unit::NumberArticles => "NAR",
            Unit::NumberCells => "NCL",
            Unit::Newton => "NEW",
            Unit::Message => "NF",
            Unit::Nil => "NIL",
            Unit::NumberInternationalUnits => "NIU",
            Unit::Load => "NL",
            Unit::NormalisedCubicMetre => "NM3",
            Unit::NauticalMile => "NMI",
            Unit::NumberPacks => "NMP",
            Unit::NumberParts => "NPT",
            Unit::NetTon => "NT",
            Unit::NephelometricTurbidityUnit => "NTU",
            Unit::NewtonMetre => "NU",
            Unit::PartPerThousand => "NX",
            Unit::Panel => "OA",
            Unit::OzoneDepletionEquivalent => "ODE",
            Unit::OdsGrams => "ODG",
            Unit::OdsKilograms => "ODK",
            Unit::OdsMilligrams => "ODM",
            Unit::Ohm => "OHM",
            Unit::OuncePerSquareYard => "ON",
            Unit::OunceAvoirdupois => "ONZ",
            Unit::OscillationsPerMinute => "OPM",
            Unit::OvertimeHour => "OT",
            Unit::FluidOunceUs => "OZA",
            Unit::FluidOunceUk => "OZI",
            Unit::Percent => "P1",
            Unit::CoulombPerMetre => "P10",
            Unit::Kiloweber => "P11",
            Unit::Gamma => "P12",
            Unit::Kilotesla => "P13",
            Unit::JoulePerSecond => "P14",
            Unit::JoulePerMinute => "P15",
            Unit::JoulePerHour => "P16",
            Unit::JoulePerDay => "P17",
            Unit::KilojoulePerSecond => "P18",
            Unit::KilojoulePerMinute => "P19",
            Unit::PoundPerFoot => "P2",
            Unit::KilojoulePerHour => "P20",
            Unit::KilojoulePerDay => "P21",
            Unit::Nanoohm => "P22",
            Unit::OhmCircularMilPerFoot => "P23",
            Unit::Kilohenry => "P24",
            Unit::LumenPerSquareFoot => "P25",
            Unit::Phot => "P26",
            Unit::Footcandle => "P27",
            Unit::CandelaPerSquareInch => "P28",
            Unit::Footlambert => "P29",
            Unit::Lambert => "P30",
            Unit::Stilb => "P31",
            Unit::CandelaPerSquareFoot => "P32",
            Unit::Kilocandela => "P33",
            Unit::Millicandela => "P34",
            Unit::HefnerKerze => "P35",
            Unit::InternationalCandle => "P36",
            Unit::BritishThermalUnitInternationalTablePerSquareFoot => "P37",
            Unit::BritishThermalUnitThermochemicalPerSquareFoot => "P38",
            Unit::CalorieThermochemicalPerSquareCentimetre => "P39",
            Unit::Langley => "P40",
            Unit::DecadeLogarithmic => "P41",
            Unit::PascalSquaredSecond => "P42",
            Unit::BelPerMetre => "P43",
            Unit::PoundMole => "P44",
            Unit::PoundMolePerSecond => "P45",
            Unit::PoundMolePerMinute => "P46",
            Unit::KilomolePerKilogram => "P47",
            Unit::PoundMolePerPound => "P48",
            Unit::NewtonSquareMetrePerAmpere => "P49",
            Unit::FivePack => "P5",
            Unit::WeberMetre => "P50",
            Unit::MolPerKilogramPascal => "P51",
            Unit::MolPerCubicMetrePascal => "P52",
            Unit::UnitPole => "P53",
            Unit::MilligrayPerSecond => "P54",
            Unit::MicrograyPerSecond => "P55",
            Unit::NanograyPerSecond => "P56",
            Unit::GrayPerMinute => "P57",
            Unit::MilligrayPerMinute => "P58",
            Unit::MicrograyPerMinute => "P59",
            Unit::NanograyPerMinute => "P60",
            Unit::GrayPerHour => "P61",
            Unit::MilligrayPerHour => "P62",
            Unit::MicrograyPerHour => "P63",
            Unit::NanograyPerHour => "P64",
            Unit::SievertPerSecond => "P65",
            Unit::MillisievertPerSecond => "P66",
            Unit::MicrosievertPerSecond => "P67",
            Unit::NanosievertPerSecond => "P68",
            Unit::RemPerSecond => "P69",
            Unit::SievertPerHour => "P70",
            Unit::MillisievertPerHour => "P71",
            Unit::MicrosievertPerHour => "P72",
            Unit::NanosievertPerHour => "P73",
            Unit::SievertPerMinute => "P74",
            Unit::MillisievertPerMinute => "P75",
            Unit::MicrosievertPerMinute => "P76",
            Unit::NanosievertPerMinute => "P77",
            Unit::ReciprocalSquareInch => "P78",
            Unit::PascalSquareMetrePerKilogram => "P79",
            Unit::MillipascalPerMetre => "P80",
            Unit::KilopascalPerMetre => "P81",
            Unit::HectopascalPerMetre => "P82",
            Unit::StandardAtmospherePerMetre => "P83",
            Unit::TechnicalAtmospherePerMetre => "P84",
            Unit::TorrPerMetre => "P85",
            Unit::PsiPerInch => "P86",
            Unit::CubicMetrePerSecondSquareMetre => "P87",
            Unit::Rhe => "P88",
            Unit::PoundForceFootPerInch => "P89",
            Unit::PoundForceInchPerInch => "P90",
            Unit::Perm0Oc => "P91",
            Unit::Perm23Oc => "P92",
            Unit::BytePerSecond => "P93",
            Unit::KilobytePerSecond => "P94",
            Unit::MegabytePerSecond => "P95",
            Unit::ReciprocalVolt => "P96",
            Unit::ReciprocalRadian => "P97",
            Unit::PascalToPowerSumStoichiometricNumbers => "P98",
            Unit::MolePerCubivMetreToPowerSumStoichiometricNumbers => "P99",
            Unit::Pascal => "PAL",
            Unit::Pad => "PD",
            Unit::ProofLitre => "PFL",
            Unit::ProofGallon => "PGL",
            Unit::Pitch => "PI",
            Unit::DegreePlato => "PLA",
            Unit::PoundPerInchLength => "PO",
            Unit::PagePerInch => "PQ",
            Unit::Pair => "PR",
            Unit::PoundForcePerSquareInch => "PS",
            Unit::DryPintUs => "PTD",
            Unit::PintUk => "PTI",
            Unit::LiquidPintUs => "PTL",
            Unit::Portion => "PTN",
            Unit::JoulePerTesla => "Q10",
            Unit::Erlang => "Q11",
            Unit::Octet => "Q12",
            Unit::OctetPerSecond => "Q13",
            Unit::Shannon => "Q14",
            Unit::Hartley => "Q15",
            Unit::NaturalUnitInformation => "Q16",
            Unit::ShannonPerSecond => "Q17",
            Unit::HartleyPerSecond => "Q18",
            Unit::NaturalUnitInformationPerSecond => "Q19",
            Unit::SecondPerKilogramm => "Q20",
            Unit::WattSquareMetre => "Q21",
            Unit::SecondPerRadianCubicMetre => "Q22",
            Unit::WeberToPowerMinusOne => "Q23",
            Unit::ReciprocalInch => "Q24",
            Unit::Dioptre => "Q25",
            Unit::OnePerOne => "Q26",
            Unit::NewtonMetrePerMetre => "Q27",
            Unit::KilogramPerSquareMetrePascalSecond => "Q28",
            Unit::MicrogramPerHectogram => "Q29",
            Unit::Meal => "Q3",
            Unit::PhPotentialHydrogen => "Q30",
            Unit::KilojoulePerGram => "Q31",
            Unit::Femtolitre => "Q32",
            Unit::Picolitre => "Q33",
            Unit::Nanolitre => "Q34",
            Unit::MegawattsPerMinute => "Q35",
            Unit::SquareMetrePerCubicMetre => "Q36",
            Unit::StandardCubicMetrePerDay => "Q37",
            Unit::StandardCubicMetrePerHour => "Q38",
            Unit::NormalizedCubicMetrePerDay => "Q39",
            Unit::NormalizedCubicMetrePerHour => "Q40",
            Unit::JoulePerNormalisedCubicMetre => "Q41",
            Unit::JoulePerStandardCubicMetre => "Q42",
            Unit::PageFacsimile => "QA",
            Unit::QuarterAYear => "QAN",
            Unit::PageHardcopy => "QB",
            Unit::Quire => "QR",
            Unit::DryQuartUs => "QTD",
            Unit::QuartUk => "QTI",
            Unit::LiquidQuartUs => "QTL",
            Unit::QuarterUk => "QTR",
            Unit::Pica => "R1",
            Unit::ThousandCubicMetre => "R9",
            Unit::RunningOrOperatingHour => "RH",
            Unit::Ream => "RM",
            Unit::Room => "ROM",
            Unit::PoundPerReam => "RP",
            Unit::RevolutionsPerMinute => "RPM",
            Unit::RevolutionsPerSecond => "RPS",
            Unit::RevenueTonMile => "RT",
            Unit::SquareFootPerSecond => "S3",
            Unit::SquareMetrePerSecond => "S4",
            Unit::HalfYear6Months => "SAN",
            Unit::Score => "SCO",
            Unit::Scruple => "SCR",
            Unit::SecondUnitTime => "SEC",
            Unit::Set => "SET",
            Unit::Segment => "SG",
            Unit::Siemens => "SIE",
            Unit::StandardCubicMetre => "SM3",
            Unit::MileStatuteMile => "SMI",
            Unit::Square => "SQ",
            Unit::SquareRoofing => "SQR",
            Unit::Strip => "SR",
            Unit::Stick => "STC",
            Unit::StoneUk => "STI",
            Unit::StickCigarette => "STK",
            Unit::StandardLitre => "STL",
            Unit::TonUsOrShortTonUkUs => "STN",
            Unit::Straw => "STW",
            Unit::Skein => "SW",
            Unit::Shipment => "SX",
            Unit::Syringe => "SYR",
            Unit::TelecommunicationLineInService => "T0",
            Unit::ThousandPiece => "T3",
            Unit::KiloampereHourThousandAmpereHour => "TAH",
            Unit::TotalAcidNumber => "TAN",
            Unit::ThousandSquareInch => "TI",
            Unit::MetricTonIncludingContainer => "TIC",
            Unit::MetricTonIncludingInnerPackaging => "TIP",
            Unit::TonneKilometre => "TKM",
            Unit::KilogramImportedMeatLessOffal => "TMS",
            Unit::TonneMetricTon => "TNE",
            Unit::TenPack => "TP",
            Unit::TeethPerInch => "TPI",
            Unit::TenPair => "TPR",
            Unit::ThousandCubicMetrePerDay => "TQD",
            Unit::TrillionEur => "TRL",
            Unit::TenSet => "TST",
            Unit::TenThousandSticks => "TTS",
            Unit::Treatment => "U1",
            Unit::Tablet => "U2",
            Unit::TelecommunicationLineInServiceAverage => "UB",
            Unit::TelecommunicationPort => "UC",
            Unit::VoltAmperePerKilogram => "VA",
            Unit::Volt => "VLT",
            Unit::PercentVolume => "VP",
            Unit::WetKilo => "W2",
            Unit::WattPerKilogram => "WA",
            Unit::WetPound => "WB",
            Unit::Cord => "WCD",
            Unit::WetTon => "WE",
            Unit::Weber => "WEB",
            Unit::Week => "WEE",
            Unit::WineGallon => "WG",
            Unit::WattHour => "WHR",
            Unit::WorkingMonth => "WM",
            Unit::Standard => "WSD",
            Unit::Watt => "WTT",
            Unit::GuntersChain => "X1",
            Unit::SquareYard => "YDK",
            Unit::CubicYard => "YDQ",
            Unit::Yard => "YRD",
            Unit::HangingContainer => "Z11",
            Unit::Nanomole => "Z9",
            Unit::Page => "ZP",
            Unit::MutuallyDefined => "ZZ",
            Unit::DrumSteel => "X1A",
            Unit::DrumAluminium => "X1B",
            Unit::DrumPlywood => "X1D",
            Unit::ContainerFlexible => "X1F",
            Unit::DrumFibre => "X1G",
            Unit::DrumWooden => "X1W",
            Unit::BarrelWooden => "X2C",
            Unit::JerricanSteel => "X3A",
            Unit::JerricanPlastic => "X3H",
            Unit::BagSuperBulk => "X43",
            Unit::BagPolybag => "X44",
            Unit::BoxSteel => "X4A",
            Unit::BoxAluminium => "X4B",
            Unit::BoxNaturalWood => "X4C",
            Unit::BoxPlywood => "X4D",
            Unit::BoxReconstitutedWood => "X4F",
            Unit::BoxFibreboard => "X4G",
            Unit::BoxPlastic => "X4H",
            Unit::BagWovenPlastic => "X5H",
            Unit::BagTextile => "X5L",
            Unit::BagPaper => "X5M",
            Unit::CompositePackagingPlasticReceptacle => "X6H",
            Unit::CompositePackagingGlassReceptacle => "X6P",
            Unit::CaseCar => "X7A",
            Unit::CaseWooden => "X7B",
            Unit::PalletWooden => "X8A",
            Unit::CrateWooden => "X8B",
            Unit::BundleWooden => "X8C",
            Unit::IntermediateBulkContainerRigidPlastic => "XAA",
            Unit::ReceptacleFibre => "XAB",
            Unit::ReceptaclePaper => "XAC",
            Unit::ReceptacleWooden => "XAD",
            Unit::Aerosol => "XAE",
            Unit::PalletModularCollars80cms60cms => "XAF",
            Unit::PalletShrinkwrapped => "XAG",
            Unit::Pallet100cms110cms => "XAH",
            Unit::Clamshell => "XAI",
            Unit::Cone => "XAJ",
            Unit::Ball_Dup => "XAL",
            Unit::AmpouleNonProtected => "XAM",
            Unit::AmpouleProtected => "XAP",
            Unit::Atomizer => "XAT",
            Unit::Capsule => "XAV",
            Unit::Belt => "XB4",
            Unit::Barrel => "XBA",
            Unit::Bobbin => "XBB",
            Unit::BottlecrateBottlerack => "XBC",
            Unit::Board => "XBD",
            Unit::Bundle => "XBE",
            Unit::BalloonNonProtected => "XBF",
            Unit::Bag => "XBG",
            Unit::Bunch => "XBH",
            Unit::Bin => "XBI",
            Unit::Bucket => "XBJ",
            Unit::Basket => "XBK",
            Unit::BaleCompressed => "XBL",
            Unit::Basin => "XBM",
            Unit::BaleNonCompressed => "XBN",
            Unit::BottleNonProtectedCylindrical => "XBO",
            Unit::BalloonProtected => "XBP",
            Unit::BottleProtectedCylindrical => "XBQ",
            Unit::Bar => "XBR",
            Unit::BottleNonProtectedBulbous => "XBS",
            Unit::Bolt => "XBT",
            Unit::Butt => "XBU",
            Unit::BottleProtectedBulbous => "XBV",
            Unit::BoxForLiquids => "XBW",
            Unit::Box => "XBX",
            Unit::BoardInBundleBunchTruss => "XBY",
            Unit::BarsInBundleBunchTruss => "XBZ",
            Unit::CanRectangular => "XCA",
            Unit::CrateBeer => "XCB",
            Unit::Churn => "XCC",
            Unit::CanWithHandleAndSpout => "XCD",
            Unit::Creel => "XCE",
            Unit::Coffer => "XCF",
            Unit::Cage => "XCG",
            Unit::Chest => "XCH",
            Unit::Canister => "XCI",
            Unit::Coffin => "XCJ",
            Unit::Cask => "XCK",
            Unit::Coil => "XCL",
            Unit::Card_Dup => "XCM",
            Unit::ContainerNotOtherwiseSpecifiedAsTransportEquipment => "XCN",
            Unit::CarboyNonProtected => "XCO",
            Unit::CarboyProtected => "XCP",
            Unit::Cartridge => "XCQ",
            Unit::Crate => "XCR",
            Unit::Case => "XCS",
            Unit::Carton => "XCT",
            Unit::Cup => "XCU",
            Unit::Cover => "XCV",
            Unit::CageRoll => "XCW",
            Unit::CanCylindrical => "XCX",
            Unit::Cylinder => "XCY",
            Unit::Canvas => "XCZ",
            Unit::CrateMultipleLayerPlastic => "XDA",
            Unit::CrateMultipleLayerWooden => "XDB",
            Unit::CrateMultipleLayerCardboard => "XDC",
            Unit::CageCommonwealthHandlingEquipmentPoolChep => "XDG",
            Unit::BoxCommonwealthHandlingEquipmentPoolChepEurobox => "XDH",
            Unit::DrumIron => "XDI",
            Unit::DemijohnNonProtected => "XDJ",
            Unit::CrateBulkCardboard => "XDK",
            Unit::CrateBulkPlastic => "XDL",
            Unit::CrateBulkWooden => "XDM",
            Unit::Dispenser => "XDN",
            Unit::DemijohnProtected => "XDP",
            Unit::Drum => "XDR",
            Unit::TrayOneLayerNoCoverPlastic => "XDS",
            Unit::TrayOneLayerNoCoverWooden => "XDT",
            Unit::TrayOneLayerNoCoverPolystyrene => "XDU",
            Unit::TrayOneLayerNoCoverCardboard => "XDV",
            Unit::TrayTwoLayersNoCoverPlasticTray => "XDW",
            Unit::TrayTwoLayersNoCoverWooden => "XDX",
            Unit::TrayTwoLayersNoCoverCardboard => "XDY",
            Unit::BagPlastic => "XEC",
            Unit::CaseWithPalletBase => "XED",
            Unit::CaseWithPalletBaseWooden => "XEE",
            Unit::CaseWithPalletBaseCardboard => "XEF",
            Unit::CaseWithPalletBasePlastic => "XEG",
            Unit::CaseWithPalletBaseMetal => "XEH",
            Unit::CaseIsothermic => "XEI",
            Unit::Envelope => "XEN",
            Unit::Flexibag => "XFB",
            Unit::CrateFruit => "XFC",
            Unit::CrateFramed => "XFD",
            Unit::Flexitank => "XFE",
            Unit::Firkin => "XFI",
            Unit::Flask => "XFL",
            Unit::Footlocker => "XFO",
            Unit::Filmpack => "XFP",
            Unit::Frame => "XFR",
            Unit::Foodtainer => "XFT",
            Unit::CartFlatbed => "XFW",
            Unit::BagFlexibleContainer => "XFX",
            Unit::BottleGas => "XGB",
            Unit::Girder => "XGI",
            Unit::ContainerGallon => "XGL",
            Unit::ReceptacleGlass => "XGR",
            Unit::TrayContainingHorizontallyStackedFlatItems => "XGU",
            Unit::BagGunny => "XGY",
            Unit::GirdersInBundleBunchTruss => "XGZ",
            Unit::BasketWithHandlePlastic => "XHA",
            Unit::BasketWithHandleWooden => "XHB",
            Unit::BasketWithHandleCardboard => "XHC",
            Unit::Hogshead => "XHG",
            Unit::Hanger => "XHN",
            Unit::Hamper => "XHR",
            Unit::PackageDisplayWooden => "XIA",
            Unit::PackageDisplayCardboard => "XIB",
            Unit::PackageDisplayPlastic => "XIC",
            Unit::PackageDisplayMetal => "XID",
            Unit::PackageShow => "XIE",
            Unit::PackageFlow => "XIF",
            Unit::PackagePaperWrapped => "XIG",
            Unit::DrumPlastic => "XIH",
            Unit::PackageCardboardWithBottleGripHoles => "XIK",
            Unit::TrayRigidLiddedStackableCenTs144822002 => "XIL",
            Unit::Ingot => "XIN",
            Unit::IngotsInBundleBunchTruss => "XIZ",
            Unit::BagJumbo => "XJB",
            Unit::JerricanRectangular => "XJC",
            Unit::Jug => "XJG",
            Unit::Jar => "XJR",
            Unit::Jutebag => "XJT",
            Unit::JerricanCylindrical => "XJY",
            Unit::Keg => "XKG",
            Unit::Kit_Dup => "XKI",
            Unit::Luggage => "XLE",
            Unit::Log => "XLG",
            Unit::Lot => "XLT",
            Unit::Lug => "XLU",
            Unit::Liftvan => "XLV",
            Unit::LogsInBundleBunchTruss => "XLZ",
            Unit::CrateMetal => "XMA",
            Unit::BagMultiply => "XMB",
            Unit::CrateMilk => "XMC",
            Unit::ContainerMetal => "XME",
            Unit::ReceptacleMetal => "XMR",
            Unit::SackMultiWall => "XMS",
            Unit::Mat => "XMT",
            Unit::ReceptaclePlasticWrapped => "XMW",
            Unit::Matchbox => "XMX",
            Unit::NotAvailable => "XNA",
            Unit::UnpackedOrUnpackaged => "XNE",
            Unit::UnpackedOrUnpackagedSingleUnit => "XNF",
            Unit::UnpackedOrUnpackagedMultipleUnits => "XNG",
            Unit::Nest => "XNS",
            Unit::Net => "XNT",
            Unit::NetTubePlastic => "XNU",
            Unit::NetTubeTextile => "XNV",
            Unit::TwoSidedCageOnWheelsWithFixingStrap => "XO1",
            Unit::Trolley => "XO2",
            Unit::OnewayPalletIso012EuroPallet => "XO3",
            Unit::OnewayPalletIso111EuroPallet => "XO4",
            Unit::OnewayPalletIso221EuroPallet => "XO5",
            Unit::PalletWithExceptionalDimensions => "XO6",
            Unit::WoodenPallet40CmX80Cm => "XO7",
            Unit::PlasticPalletSrs60CmX80Cm => "XO8",
            Unit::PlasticPalletSrs80CmX120Cm => "XO9",
            Unit::PalletChep40CmX60Cm => "XOA",
            Unit::PalletChep80CmX120Cm => "XOB",
            Unit::PalletChep100CmX120Cm => "XOC",
            Unit::PalletAs40681993 => "XOD",
            Unit::PalletIsoT11 => "XOE",
            Unit::PlatformUnspecifiedWeightOrDimension => "XOF",
            Unit::PalletIso012EuroPallet => "XOG",
            Unit::PalletIso111EuroPallet => "XOH",
            Unit::PalletIso221EuroPallet => "XOI",
            Unit::_14EuroPallet => "XOJ",
            Unit::Block => "XOK",
            Unit::_18EuroPallet => "XOL",
            Unit::SyntheticPalletIso1 => "XOM",
            Unit::SyntheticPalletIso2 => "XON",
            Unit::WholesalerPallet => "XOP",
            Unit::Pallet80X100Cm => "XOQ",
            Unit::Pallet60X100Cm => "XOR",
            Unit::OnewayPallet => "XOS",
            Unit::Octabin => "XOT",
            Unit::ContainerOuter => "XOU",
            Unit::ReturnablePallet => "XOV",
            Unit::LargeBagPalletSized => "XOW",
            Unit::AWheeledPalletWithRaisedRim81X67X135 => "XOX",
            Unit::AWheeledPalletWithRaisedRim81X72X135 => "XOY",
            Unit::WheeledPalletWithRaisedRim81X60X16 => "XOZ",
            Unit::ChepPallet60CmX80Cm => "XP1",
            Unit::Pan => "XP2",
            Unit::LprPallet60CmX80Cm => "XP3",
            Unit::LprPallet80CmX120Cm => "XP4",
            Unit::Packet => "XPA",
            Unit::PalletBoxCombinedOpenEndedBoxAndPallet => "XPB",
            Unit::Parcel => "XPC",
            Unit::PalletModularCollars80cms100cms => "XPD",
            Unit::PalletModularCollars80cms120cms => "XPE",
            Unit::Pen => "XPF",
            Unit::Plate => "XPG",
            Unit::Pitcher => "XPH",
            Unit::Pipe => "XPI",
            Unit::Punnet => "XPJ",
            Unit::Package => "XPK",
            Unit::Pail => "XPL",
            Unit::Plank => "XPN",
            Unit::Pouch => "XPO",
            Unit::Piece_Dup => "XPP",
            Unit::ReceptaclePlastic => "XPR",
            Unit::Pot => "XPT",
            Unit::Tray => "XPU",
            Unit::PipesInBundleBunchTruss => "XPV",
            Unit::Pallet => "XPX",
            Unit::PlatesInBundleBunchTruss => "XPY",
            Unit::PlanksInBundleBunchTruss => "XPZ",
            Unit::DrumSteelNonRemovableHead => "XQA",
            Unit::DrumSteelRemovableHead => "XQB",
            Unit::DrumAluminiumNonRemovableHead => "XQC",
            Unit::DrumAluminiumRemovableHead => "XQD",
            Unit::DrumPlasticNonRemovableHead => "XQF",
            Unit::DrumPlasticRemovableHead => "XQG",
            Unit::BarrelWoodenBungType => "XQH",
            Unit::BarrelWoodenRemovableHead => "XQJ",
            Unit::JerricanSteelNonRemovableHead => "XQK",
            Unit::JerricanSteelRemovableHead => "XQL",
            Unit::JerricanPlasticNonRemovableHead => "XQM",
            Unit::JerricanPlasticRemovableHead => "XQN",
            Unit::BoxWoodenNaturalWoodOrdinary => "XQP",
            Unit::BoxWoodenNaturalWoodWithSiftProofWalls => "XQQ",
            Unit::BoxPlasticExpanded => "XQR",
            Unit::BoxPlasticSolid => "XQS",
            Unit::Rod => "XRD",
            Unit::Ring => "XRG",
            Unit::RackClothingHanger => "XRJ",
            Unit::Rack => "XRK",
            Unit::Reel => "XRL",
            Unit::Roll => "XRO",
            Unit::Rednet => "XRT",
            Unit::RodsInBundleBunchTruss => "XRZ",
            Unit::Sack => "XSA",
            Unit::Slab => "XSB",
            Unit::CrateShallow => "XSC",
            Unit::Spindle => "XSD",
            Unit::SeaChest => "XSE",
            Unit::Sachet => "XSH",
            Unit::Skid => "XSI",
            Unit::CaseSkeleton => "XSK",
            Unit::Slipsheet => "XSL",
            Unit::Sheetmetal => "XSM",
            Unit::Spool => "XSO",
            Unit::SheetPlasticWrapping => "XSP",
            Unit::CaseSteel => "XSS",
            Unit::Sheet => "XST",
            Unit::Suitcase => "XSU",
            Unit::EnvelopeSteel => "XSV",
            Unit::Shrinkwrapped => "XSW",
            Unit::Set_Dup => "XSX",
            Unit::Sleeve => "XSY",
            Unit::SheetsInBundleBunchTruss => "XSZ",
            Unit::Tablet_Dup => "XT1",
            Unit::Tub => "XTB",
            Unit::TeaChest => "XTC",
            Unit::TubeCollapsible => "XTD",
            Unit::Tyre_Dup => "XTE",
            Unit::TankContainerGeneric => "XTG",
            Unit::Tierce => "XTI",
            Unit::TankRectangular => "XTK",
            Unit::TubWithLid => "XTL",
            Unit::Tin => "XTN",
            Unit::Tun => "XTO",
            Unit::Trunk => "XTR",
            Unit::Truss => "XTS",
            Unit::BagTote => "XTT",
            Unit::Tube => "XTU",
            Unit::TubeWithNozzle => "XTV",
            Unit::PalletTriwall => "XTW",
            Unit::TankCylindrical => "XTY",
            Unit::TubesInBundleBunchTruss => "XTZ",
            Unit::Uncaged => "XUC",
            Unit::Unit => "XUN",
            Unit::Vat => "XVA",
            Unit::BulkGasAt1031MbarAnd15C => "XVG",
            Unit::Vial => "XVI",
            Unit::Vanpack => "XVK",
            Unit::BulkLiquid => "XVL",
            Unit::Vehicle => "XVN",
            Unit::BulkSolidLargeParticlesNodules => "XVO",
            Unit::VacuumPacked => "XVP",
            Unit::BulkLiquefiedGasAtAbnormalTemperaturePressure => "XVQ",
            Unit::BulkSolidGranularParticlesGrains => "XVR",
            Unit::BulkScrapMetal => "XVS",
            Unit::BulkSolidFineParticlesPowders => "XVY",
            Unit::IntermediateBulkContainer => "XWA",
            Unit::Wickerbottle => "XWB",
            Unit::IntermediateBulkContainerSteel => "XWC",
            Unit::IntermediateBulkContainerAluminium => "XWD",
            Unit::IntermediateBulkContainerMetal => "XWF",
            Unit::IntermediateBulkContainerSteelPressurised10Kpa => "XWG",
            Unit::IntermediateBulkContainerAluminiumPressurised10Kpa => "XWH",
            Unit::IntermediateBulkContainerMetalPressure10Kpa => "XWJ",
            Unit::IntermediateBulkContainerSteelLiquid => "XWK",
            Unit::IntermediateBulkContainerAluminiumLiquid => "XWL",
            Unit::IntermediateBulkContainerMetalLiquid => "XWM",
            Unit::IntermediateBulkContainerWovenPlasticWithoutCoatLiner => "XWN",
            Unit::IntermediateBulkContainerWovenPlasticCoated => "XWP",
            Unit::IntermediateBulkContainerWovenPlasticWithLiner => "XWQ",
            Unit::IntermediateBulkContainerWovenPlasticCoatedAndLiner => "XWR",
            Unit::IntermediateBulkContainerPlasticFilm => "XWS",
            Unit::IntermediateBulkContainerTextileWithOutCoatLiner => "XWT",
            Unit::IntermediateBulkContainerNaturalWoodWithInnerLiner => "XWU",
            Unit::IntermediateBulkContainerTextileCoated => "XWV",
            Unit::IntermediateBulkContainerTextileWithLiner => "XWW",
            Unit::IntermediateBulkContainerTextileCoatedAndLiner => "XWX",
            Unit::IntermediateBulkContainerPlywoodWithInnerLiner => "XWY",
            Unit::IntermediateBulkContainerReconstitutedWoodWithInnerLiner => "XWZ",
            Unit::BagWovenPlasticWithoutInnerCoatLiner => "XXA",
            Unit::BagWovenPlasticSiftProof => "XXB",
            Unit::BagWovenPlasticWaterResistant => "XXC",
            Unit::BagPlasticsFilm => "XXD",
            Unit::BagTextileWithoutInnerCoatLiner => "XXF",
            Unit::BagTextileSiftProof => "XXG",
            Unit::BagTextileWaterResistant => "XXH",
            Unit::BagPaperMultiWall => "XXJ",
            Unit::BagPaperMultiWallWaterResistant => "XXK",
            Unit::CompositePackagingPlasticReceptacleInSteelDrum => "XYA",
            Unit::CompositePackagingPlasticReceptacleInSteelCrateBox => "XYB",
            Unit::CompositePackagingPlasticReceptacleInAluminiumDrum => "XYC",
            Unit::CompositePackagingPlasticReceptacleInAluminiumCrate => "XYD",
            Unit::CompositePackagingPlasticReceptacleInWoodenBox => "XYF",
            Unit::CompositePackagingPlasticReceptacleInPlywoodDrum => "XYG",
            Unit::CompositePackagingPlasticReceptacleInPlywoodBox => "XYH",
            Unit::CompositePackagingPlasticReceptacleInFibreDrum => "XYJ",
            Unit::CompositePackagingPlasticReceptacleInFibreboardBox => "XYK",
            Unit::CompositePackagingPlasticReceptacleInPlasticDrum => "XYL",
            Unit::CompositePackagingPlasticReceptacleInSolidPlasticBox => "XYM",
            Unit::CompositePackagingGlassReceptacleInSteelDrum => "XYN",
            Unit::CompositePackagingGlassReceptacleInSteelCrateBox => "XYP",
            Unit::CompositePackagingGlassReceptacleInAluminiumDrum => "XYQ",
            Unit::CompositePackagingGlassReceptacleInAluminiumCrate => "XYR",
            Unit::CompositePackagingGlassReceptacleInWoodenBox => "XYS",
            Unit::CompositePackagingGlassReceptacleInPlywoodDrum => "XYT",
            Unit::CompositePackagingGlassReceptacleInWickerworkHamper => "XYV",
            Unit::CompositePackagingGlassReceptacleInFibreDrum => "XYW",
            Unit::CompositePackagingGlassReceptacleInFibreboardBox => "XYX",
            Unit::CompositePackagingGlassReceptacleInExpandablePlasticPack => "XYY",
            Unit::CompositePackagingGlassReceptacleInSolidPlasticPack => "XYZ",
            Unit::IntermediateBulkContainerPaperMultiWall => "XZA",
            Unit::BagLarge => "XZB",
            Unit::IntermediateBulkContainerPaperMultiWallWaterResistant => "XZC",
            Unit::IntermediateBulkContainerRigidPlasticWithStructuralEquipmentSolids => "XZD",
            Unit::IntermediateBulkContainerRigidPlasticFreestandingSolids => "XZF",
            Unit::IntermediateBulkContainerRigidPlasticWithStructuralEquipmentPressurised => "XZG",
            Unit::IntermediateBulkContainerRigidPlasticFreestandingPressurised => "XZH",
            Unit::IntermediateBulkContainerRigidPlasticWithStructuralEquipmentLiquids => "XZJ",
            Unit::IntermediateBulkContainerRigidPlasticFreestandingLiquids => "XZK",
            Unit::IntermediateBulkContainerCompositeRigidPlasticSolids => "XZL",
            Unit::IntermediateBulkContainerCompositeFlexiblePlasticSolids => "XZM",
            Unit::IntermediateBulkContainerCompositeRigidPlasticPressurised => "XZN",
            Unit::IntermediateBulkContainerCompositeFlexiblePlasticPressurised => "XZP",
            Unit::IntermediateBulkContainerCompositeRigidPlasticLiquids => "XZQ",
            Unit::IntermediateBulkContainerCompositeFlexiblePlasticLiquids => "XZR",
            Unit::IntermediateBulkContainerComposite => "XZS",
            Unit::IntermediateBulkContainerFibreboard => "XZT",
            Unit::IntermediateBulkContainerFlexible => "XZU",
            Unit::IntermediateBulkContainerMetalOtherThanSteel => "XZV",
            Unit::IntermediateBulkContainerNaturalWood => "XZW",
            Unit::IntermediateBulkContainerPlywood => "XZX",
            Unit::IntermediateBulkContainerReconstitutedWood => "XZY",
            Unit::MutuallyDefined_Dup => "XZZ",
        }
    }
}

impl crate::Description for Unit {
    fn description(&self) -> &str {
        match self {
            Unit::Group => "group",
            Unit::Outfit => "outfit",
            Unit::Ration => "ration",
            Unit::Shot => "shot",
            Unit::StickMilitary => "stick, military",
            Unit::TwentyFootContainer => "twenty foot container",
            Unit::FortyFootContainer => "forty foot container",
            Unit::DecilitrePerGram => "decilitre per gram",
            Unit::GramPerCubicCentimetre => "gram per cubic centimetre",
            Unit::TheoreticalPound => "theoretical pound",
            Unit::GramPerSquareCentimetre => "gram per square centimetre",
            Unit::TheoreticalTon => "theoretical ton",
            Unit::KilogramPerSquareMetre => "kilogram per square metre",
            Unit::KilopascalSquareMetrePerGram => "kilopascal square metre per gram",
            Unit::KilopascalPerMillimetre => "kilopascal per millimetre",
            Unit::MillilitrePerSquareCentimetreSecond => "millilitre per square centimetre second",
            Unit::OuncePerSquareFoot => "ounce per square foot",
            Unit::OuncePerSquareFootPer001inch => "ounce per square foot per 0,01inch",
            Unit::MillilitrePerSecond => "millilitre per second",
            Unit::MillilitrePerMinute => "millilitre per minute",
            Unit::Sitas => "sitas",
            Unit::Mesh => "mesh",
            Unit::NetKilogram => "net kilogram",
            Unit::PartPerMillion => "part per million",
            Unit::PercentWeight => "percent weight",
            Unit::PartPerBillionUs => "part per billion (US)",
            Unit::Millipascal => "millipascal",
            Unit::MilliInch => "milli-inch",
            Unit::PoundPerSquareInchAbsolute => "pound per square inch absolute",
            Unit::Henry => "henry",
            Unit::FootPoundForce => "foot pound-force",
            Unit::PoundPerCubicFoot => "pound per cubic foot",
            Unit::Poise => "poise",
            Unit::Stokes => "stokes",
            Unit::FixedRate => "fixed rate",
            Unit::RadianPerSecond => "radian per second",
            Unit::RadianPerSecondSquared => "radian per second squared",
            Unit::Roentgen => "roentgen",
            Unit::VoltAc => "volt AC",
            Unit::VoltDc => "volt DC",
            Unit::BritishThermalUnitInternationalTablePerHour => "British thermal unit (international table) per hour",
            Unit::CubicCentimetrePerSecond => "cubic centimetre per second",
            Unit::CubicFootPerHour => "cubic foot per hour",
            Unit::CubicFootPerMinute => "cubic foot per minute",
            Unit::CentimetrePerSecond => "centimetre per second",
            Unit::Decibel => "decibel",
            Unit::Kilobyte => "kilobyte",
            Unit::Kilobecquerel => "kilobecquerel",
            Unit::Kilocurie => "kilocurie",
            Unit::Megagram => "megagram",
            Unit::MetrePerMinute => "metre per minute",
            Unit::Milliroentgen => "milliroentgen",
            Unit::Millivolt => "millivolt",
            Unit::Megajoule => "megajoule",
            Unit::Manmonth => "manmonth",
            Unit::Centistokes => "centistokes",
            Unit::Microlitre => "microlitre",
            Unit::MicrometreMicron => "micrometre (micron)",
            Unit::Milliampere => "milliampere",
            Unit::Megabyte => "megabyte",
            Unit::MilligramPerHour => "milligram per hour",
            Unit::Megabecquerel => "megabecquerel",
            Unit::Microfarad => "microfarad",
            Unit::NewtonPerMetre => "newton per metre",
            Unit::OunceInch => "ounce inch",
            Unit::OunceFoot => "ounce foot",
            Unit::Picofarad => "picofarad",
            Unit::PoundPerHour => "pound per hour",
            Unit::TonUsPerHour => "ton (US) per hour",
            Unit::KilolitrePerHour => "kilolitre per hour",
            Unit::BarrelUsPerMinute => "barrel (US) per minute",
            Unit::Batch => "batch",
            Unit::MmscfDay => "MMSCF/day",
            Unit::HydraulicHorsePower => "hydraulic horse power",
            Unit::AmpereSquareMetrePerJouleSecond => "ampere square metre per joule second",
            Unit::Angstrom => "angstrom",
            Unit::AstronomicalUnit => "astronomical unit",
            Unit::Attojoule => "attojoule",
            Unit::Barn => "barn",
            Unit::BarnPerElectronvolt => "barn per electronvolt",
            Unit::BarnPerSteradianElectronvolt => "barn per steradian electronvolt",
            Unit::BarnPerSteradian => "barn per steradian",
            Unit::BecquerelPerKilogram => "becquerel per kilogram",
            Unit::BecquerelPerCubicMetre => "becquerel per cubic metre",
            Unit::AmperePerCentimetre => "ampere per centimetre",
            Unit::BritishThermalUnitInternationalTablePerSecondSquareFootDegreeRankine => "British thermal unit (international table) per second square foot degree Rankine",
            Unit::BritishThermalUnitInternationalTablePerPoundDegreeRankine => "British thermal unit (international table) per pound degree Rankine",
            Unit::BritishThermalUnitInternationalTablePerSecondFootDegreeRankine => "British thermal unit (international table) per second foot degree Rankine",
            Unit::BritishThermalUnitInternationalTablePerHourSquareFootDegreeRankine => "British thermal unit (international table) per hour square foot degree Rankine",
            Unit::CandelaPerSquareMetre => "candela per square metre",
            Unit::CoulombMetre => "coulomb metre",
            Unit::CoulombMetreSquaredPerVolt => "coulomb metre squared per volt",
            Unit::CoulombPerCubicCentimetre => "coulomb per cubic centimetre",
            Unit::CoulombPerCubicMetre => "coulomb per cubic metre",
            Unit::AmperePerMillimetre => "ampere per millimetre",
            Unit::CoulombPerCubicMillimetre => "coulomb per cubic millimetre",
            Unit::CoulombPerKilogramSecond => "coulomb per kilogram second",
            Unit::CoulombPerMole => "coulomb per mole",
            Unit::CoulombPerSquareCentimetre => "coulomb per square centimetre",
            Unit::CoulombPerSquareMetre => "coulomb per square metre",
            Unit::CoulombPerSquareMillimetre => "coulomb per square millimetre",
            Unit::CubicCentimetrePerMole => "cubic centimetre per mole",
            Unit::CubicDecimetrePerMole => "cubic decimetre per mole",
            Unit::CubicMetrePerCoulomb => "cubic metre per coulomb",
            Unit::CubicMetrePerKilogram => "cubic metre per kilogram",
            Unit::AmperePerSquareCentimetre => "ampere per square centimetre",
            Unit::CubicMetrePerMole => "cubic metre per mole",
            Unit::AmperePerSquareMetre => "ampere per square metre",
            Unit::CuriePerKilogram => "curie per kilogram",
            Unit::DeadweightTonnage => "deadweight tonnage",
            Unit::Decalitre => "decalitre",
            Unit::Decametre => "decametre",
            Unit::Decitex => "decitex",
            Unit::DegreeRankine => "degree Rankine",
            Unit::Denier => "denier",
            Unit::AmpereSquareMetre => "ampere square metre",
            Unit::Electronvolt => "electronvolt",
            Unit::ElectronvoltPerMetre => "electronvolt per metre",
            Unit::ElectronvoltSquareMetre => "electronvolt square metre",
            Unit::ElectronvoltSquareMetrePerKilogram => "electronvolt square metre per kilogram",
            Unit::_8PartCloudCover => "8-part cloud cover",
            Unit::AmperePerSquareMetreKelvinSquared => "ampere per square metre kelvin squared",
            Unit::Exajoule => "exajoule",
            Unit::FaradPerMetre => "farad per metre",
            Unit::AmperePerSquareMillimetre => "ampere per square millimetre",
            Unit::Femtojoule => "femtojoule",
            Unit::Femtometre => "femtometre",
            Unit::FootPerSecondSquared => "foot per second squared",
            Unit::FootPoundForcePerSecond => "foot pound-force per second",
            Unit::FreightTon => "freight ton",
            Unit::Gal => "gal",
            Unit::AmpereSecond => "ampere second",
            Unit::GigacoulombPerCubicMetre => "gigacoulomb per cubic metre",
            Unit::Gigaelectronvolt => "gigaelectronvolt",
            Unit::Gigahertz => "gigahertz",
            Unit::Gigaohm => "gigaohm",
            Unit::GigaohmMetre => "gigaohm metre",
            Unit::Gigapascal => "gigapascal",
            Unit::Rate => "rate",
            Unit::Gigawatt => "gigawatt",
            Unit::Gon => "gon",
            Unit::GramPerCubicMetre => "gram per cubic metre",
            Unit::GramPerMole => "gram per mole",
            Unit::Gray => "gray",
            Unit::GrayPerSecond => "gray per second",
            Unit::Hectopascal => "hectopascal",
            Unit::HenryPerMetre => "henry per metre",
            Unit::Bit => "bit",
            Unit::Ball => "ball",
            Unit::BulkPack => "bulk pack",
            Unit::Acre => "acre",
            Unit::Activity => "activity",
            Unit::Byte => "byte",
            Unit::AmperePerMetre => "ampere per metre",
            Unit::AdditionalMinute => "additional minute",
            Unit::AverageMinutePerCall => "average minute per call",
            Unit::Fathom => "fathom",
            Unit::AccessLine => "access line",
            Unit::AmpereHour => "ampere hour",
            Unit::Ampere => "ampere",
            Unit::Year => "year",
            Unit::TroyOunceOrApothecaryOunce => "troy ounce or apothecary ounce",
            Unit::AntiHemophilicFactorAhfUnit => "anti-hemophilic factor (AHF) unit",
            Unit::Assortment => "assortment",
            Unit::AlcoholicStrengthByMass => "alcoholic strength by mass",
            Unit::AlcoholicStrengthByVolume => "alcoholic strength by volume",
            Unit::StandardAtmosphere => "standard atmosphere",
            Unit::AmericanWireGauge => "american wire gauge",
            Unit::Assembly => "assembly",
            Unit::BritishThermalUnitInternationalTablePerPound => "British thermal unit (international table) per pound",
            Unit::BarrelUsPerDay => "barrel (US) per day",
            Unit::BitPerSecond => "bit per second",
            Unit::JoulePerKilogramKelvin => "joule per kilogram kelvin",
            Unit::JoulePerMetre => "joule per metre",
            Unit::JoulePerSquareMetre => "joule per square metre",
            Unit::JoulePerMetreToFourthPower => "joule per metre to the fourth power",
            Unit::JoulePerMole => "joule per mole",
            Unit::JoulePerMoleKelvin => "joule per mole kelvin",
            Unit::Credit => "credit",
            Unit::JouleSecond => "joule second",
            Unit::Digit => "digit",
            Unit::JouleSquareMetrePerKilogram => "joule square metre per kilogram",
            Unit::KelvinPerWatt => "kelvin per watt",
            Unit::Kiloampere => "kiloampere",
            Unit::KiloamperePerSquareMetre => "kiloampere per square metre",
            Unit::KiloamperePerMetre => "kiloampere per metre",
            Unit::KilobecquerelPerKilogram => "kilobecquerel per kilogram",
            Unit::Kilocoulomb => "kilocoulomb",
            Unit::KilocoulombPerCubicMetre => "kilocoulomb per cubic metre",
            Unit::KilocoulombPerSquareMetre => "kilocoulomb per square metre",
            Unit::Kiloelectronvolt => "kiloelectronvolt",
            Unit::BattingPound => "batting pound",
            Unit::Gibibit => "gibibit",
            Unit::KilogramMetrePerSecond => "kilogram metre per second",
            Unit::KilogramMetreSquared => "kilogram metre squared",
            Unit::KilogramMetreSquaredPerSecond => "kilogram metre squared per second",
            Unit::KilogramPerCubicDecimetre => "kilogram per cubic decimetre",
            Unit::KilogramPerLitre => "kilogram per litre",
            Unit::BarrelImperial => "barrel, imperial",
            Unit::KilojoulePerKelvin => "kilojoule per kelvin",
            Unit::KilojoulePerKilogram => "kilojoule per kilogram",
            Unit::KilojoulePerKilogramKelvin => "kilojoule per kilogram kelvin",
            Unit::KilojoulePerMole => "kilojoule per mole",
            Unit::Kilomole => "kilomole",
            Unit::KilomolePerCubicMetre => "kilomole per cubic metre",
            Unit::Kilonewton => "kilonewton",
            Unit::KilonewtonMetre => "kilonewton metre",
            Unit::Kiloohm => "kiloohm",
            Unit::KiloohmMetre => "kiloohm metre",
            Unit::Kilosecond => "kilosecond",
            Unit::Kilosiemens => "kilosiemens",
            Unit::KilosiemensPerMetre => "kilosiemens per metre",
            Unit::KilovoltPerMetre => "kilovolt per metre",
            Unit::KiloweberPerMetre => "kiloweber per metre",
            Unit::LightYear => "light year",
            Unit::LitrePerMole => "litre per mole",
            Unit::LumenHour => "lumen hour",
            Unit::LumenPerSquareMetre => "lumen per square metre",
            Unit::LumenPerWatt => "lumen per watt",
            Unit::LumenSecond => "lumen second",
            Unit::LuxHour => "lux hour",
            Unit::LuxSecond => "lux second",
            Unit::MegaamperePerSquareMetre => "megaampere per square metre",
            Unit::MegabecquerelPerKilogram => "megabecquerel per kilogram",
            Unit::Gigabit => "gigabit",
            Unit::MegacoulombPerCubicMetre => "megacoulomb per cubic metre",
            Unit::Cycle => "cycle",
            Unit::MegacoulombPerSquareMetre => "megacoulomb per square metre",
            Unit::Megaelectronvolt => "megaelectronvolt",
            Unit::MegagramPerCubicMetre => "megagram per cubic metre",
            Unit::Meganewton => "meganewton",
            Unit::MeganewtonMetre => "meganewton metre",
            Unit::Megaohm => "megaohm",
            Unit::MegaohmMetre => "megaohm metre",
            Unit::MegasiemensPerMetre => "megasiemens per metre",
            Unit::Megavolt => "megavolt",
            Unit::MegavoltPerMetre => "megavolt per metre",
            Unit::JoulePerCubicMetre => "joule per cubic metre",
            Unit::GigabitPerSecond => "gigabit per second",
            Unit::ReciprocalMetreSquaredReciprocalSecond => "reciprocal metre squared reciprocal second",
            Unit::InchPerLinearFoot => "inch per linear foot",
            Unit::MetreToFourthPower => "metre to the fourth power",
            Unit::Microampere => "microampere",
            Unit::Microbar => "microbar",
            Unit::Microcoulomb => "microcoulomb",
            Unit::MicrocoulombPerCubicMetre => "microcoulomb per cubic metre",
            Unit::MicrocoulombPerSquareMetre => "microcoulomb per square metre",
            Unit::MicrofaradPerMetre => "microfarad per metre",
            Unit::Microhenry => "microhenry",
            Unit::MicrohenryPerMetre => "microhenry per metre",
            Unit::Micronewton => "micronewton",
            Unit::MicronewtonMetre => "micronewton metre",
            Unit::Microohm => "microohm",
            Unit::MicroohmMetre => "microohm metre",
            Unit::Micropascal => "micropascal",
            Unit::Microradian => "microradian",
            Unit::Microsecond => "microsecond",
            Unit::Microsiemens => "microsiemens",
            Unit::BarUnitPressure => "bar [unit of pressure]",
            Unit::BaseBox => "base box",
            Unit::BoardFoot => "board foot",
            Unit::BrakeHorsePower => "brake horse power",
            Unit::BillionEur => "billion (EUR)",
            Unit::DryBarrelUs => "dry barrel (US)",
            Unit::BarrelUs => "barrel (US)",
            Unit::HundredBoardFoot => "hundred board foot",
            Unit::BeatsPerMinute => "beats per minute",
            Unit::Becquerel => "becquerel",
            Unit::BritishThermalUnitInternationalTable => "British thermal unit (international table)",
            Unit::BushelUs => "bushel (US)",
            Unit::BushelUk => "bushel (UK)",
            Unit::Call => "call",
            Unit::Millifarad => "millifarad",
            Unit::Milligal => "milligal",
            Unit::MilligramPerMetre => "milligram per metre",
            Unit::Milligray => "milligray",
            Unit::Millihenry => "millihenry",
            Unit::Millijoule => "millijoule",
            Unit::MillimetrePerSecond => "millimetre per second",
            Unit::MillimetreSquaredPerSecond => "millimetre squared per second",
            Unit::Millimole => "millimole",
            Unit::MolePerKilogram => "mole per kilogram",
            Unit::Millinewton => "millinewton",
            Unit::Kibibit => "kibibit",
            Unit::MillinewtonPerMetre => "millinewton per metre",
            Unit::MilliohmMetre => "milliohm metre",
            Unit::MillipascalSecond => "millipascal second",
            Unit::Milliradian => "milliradian",
            Unit::Millisecond => "millisecond",
            Unit::Millisiemens => "millisiemens",
            Unit::Millisievert => "millisievert",
            Unit::Millitesla => "millitesla",
            Unit::MicrovoltPerMetre => "microvolt per metre",
            Unit::MillivoltPerMetre => "millivolt per metre",
            Unit::Milliwatt => "milliwatt",
            Unit::MilliwattPerSquareMetre => "milliwatt per square metre",
            Unit::Milliweber => "milliweber",
            Unit::Mole => "mole",
            Unit::MolePerCubicDecimetre => "mole per cubic decimetre",
            Unit::MolePerCubicMetre => "mole per cubic metre",
            Unit::Kilobit => "kilobit",
            Unit::MolePerLitre => "mole per litre",
            Unit::Nanoampere => "nanoampere",
            Unit::Nanocoulomb => "nanocoulomb",
            Unit::Nanofarad => "nanofarad",
            Unit::NanofaradPerMetre => "nanofarad per metre",
            Unit::Nanohenry => "nanohenry",
            Unit::NanohenryPerMetre => "nanohenry per metre",
            Unit::Nanometre => "nanometre",
            Unit::NanoohmMetre => "nanoohm metre",
            Unit::Nanosecond => "nanosecond",
            Unit::Nanotesla => "nanotesla",
            Unit::Nanowatt => "nanowatt",
            Unit::Neper => "neper",
            Unit::NeperPerSecond => "neper per second",
            Unit::Picometre => "picometre",
            Unit::NewtonMetreSecond => "newton metre second",
            Unit::NewtonMetreSquaredPerKilogramSquared => "newton metre squared per kilogram squared",
            Unit::NewtonPerSquareMetre => "newton per square metre",
            Unit::NewtonPerSquareMillimetre => "newton per square millimetre",
            Unit::NewtonSecond => "newton second",
            Unit::NewtonSecondPerMetre => "newton second per metre",
            Unit::Octave => "octave",
            Unit::OhmCentimetre => "ohm centimetre",
            Unit::OhmMetre => "ohm metre",
            Unit::One => "one",
            Unit::Parsec => "parsec",
            Unit::PascalPerKelvin => "pascal per kelvin",
            Unit::PascalSecond => "pascal second",
            Unit::PascalSecondPerCubicMetre => "pascal second per cubic metre",
            Unit::PascalSecondPerMetre => "pascal second per metre",
            Unit::Petajoule => "petajoule",
            Unit::Phon => "phon",
            Unit::Centipoise => "centipoise",
            Unit::Picoampere => "picoampere",
            Unit::Picocoulomb => "picocoulomb",
            Unit::PicofaradPerMetre => "picofarad per metre",
            Unit::Picohenry => "picohenry",
            Unit::KilobitPerSecond => "kilobit per second",
            Unit::Picowatt => "picowatt",
            Unit::PicowattPerSquareMetre => "picowatt per square metre",
            Unit::PoundForce => "pound-force",
            Unit::KilovoltAmpereHour => "kilovolt ampere hour",
            Unit::MillicoulombPerKilogram => "millicoulomb per kilogram",
            Unit::Rad => "rad",
            Unit::Radian => "radian",
            Unit::RadianSquareMetrePerMole => "radian square metre per mole",
            Unit::RadianSquareMetrePerKilogram => "radian square metre per kilogram",
            Unit::RadianPerMetre => "radian per metre",
            Unit::ReciprocalAngstrom => "reciprocal angstrom",
            Unit::ReciprocalCubicMetre => "reciprocal cubic metre",
            Unit::ReciprocalCubicMetrePerSecond => "reciprocal cubic metre per second",
            Unit::ReciprocalElectronVoltPerCubicMetre => "reciprocal electron volt per cubic metre",
            Unit::ReciprocalHenry => "reciprocal henry",
            Unit::CoilGroup => "coil group",
            Unit::ReciprocalJoulePerCubicMetre => "reciprocal joule per cubic metre",
            Unit::ReciprocalKelvinOrKelvinToPowerMinusOne => "reciprocal kelvin or kelvin to the power minus one",
            Unit::ReciprocalMetre => "reciprocal metre",
            Unit::ReciprocalSquareMetre => "reciprocal square metre",
            Unit::ReciprocalMinute => "reciprocal minute",
            Unit::ReciprocalMole => "reciprocal mole",
            Unit::ReciprocalPascalOrPascalToPowerMinusOne => "reciprocal pascal or pascal to the power minus one",
            Unit::ReciprocalSecond => "reciprocal second",
            Unit::ReciprocalSecondPerMetreSquared => "reciprocal second per metre squared",
            Unit::CarryingCapacityInMetricTon => "carrying capacity in metric ton",
            Unit::Candela => "candela",
            Unit::DegreeCelsius => "degree Celsius",
            Unit::Hundred => "hundred",
            Unit::Card => "card",
            Unit::Centigram => "centigram",
            Unit::CoulombPerKilogram => "coulomb per kilogram",
            Unit::HundredLeave => "hundred leave",
            Unit::Centilitre => "centilitre",
            Unit::SquareCentimetre => "square centimetre",
            Unit::CubicCentimetre => "cubic centimetre",
            Unit::Centimetre => "centimetre",
            Unit::HundredPack => "hundred pack",
            Unit::CentalUk => "cental (UK)",
            Unit::Coulomb => "coulomb",
            Unit::ContentGram => "content gram",
            Unit::MetricCarat => "metric carat",
            Unit::ContentTonMetric => "content ton (metric)",
            Unit::Curie => "curie",
            Unit::HundredPoundCwtHundredWeightUs => "hundred pound (cwt) / hundred weight (US)",
            Unit::HundredWeightUk => "hundred weight (UK)",
            Unit::KilowattHourPerHour => "kilowatt hour per hour",
            Unit::LotUnitWeight => "lot  [unit of weight]",
            Unit::ReciprocalSecondPerSteradian => "reciprocal second per steradian",
            Unit::SiemensPerMetre => "siemens per metre",
            Unit::Mebibit => "mebibit",
            Unit::SiemensSquareMetrePerMole => "siemens square metre per mole",
            Unit::Sievert => "sievert",
            Unit::Sone => "sone",
            Unit::SquareCentimetrePerErg => "square centimetre per erg",
            Unit::SquareCentimetrePerSteradianErg => "square centimetre per steradian erg",
            Unit::MetreKelvin => "metre kelvin",
            Unit::SquareMetreKelvinPerWatt => "square metre kelvin per watt",
            Unit::ReciprocalSecondPerSteradianMetreSquared => "reciprocal second per steradian metre squared",
            Unit::SquareMetrePerJoule => "square metre per joule",
            Unit::SquareMetrePerKilogram => "square metre per kilogram",
            Unit::SquareMetrePerMole => "square metre per mole",
            Unit::PenGramProtein => "pen gram (protein)",
            Unit::SquareMetrePerSteradian => "square metre per steradian",
            Unit::SquareMetrePerSteradianJoule => "square metre per steradian joule",
            Unit::SquareMetrePerVoltSecond => "square metre per volt second",
            Unit::Steradian => "steradian",
            Unit::Terahertz => "terahertz",
            Unit::Terajoule => "terajoule",
            Unit::Terawatt => "terawatt",
            Unit::TerawattHour => "terawatt hour",
            Unit::Tesla => "tesla",
            Unit::Tex => "tex",
            Unit::Megabit => "megabit",
            Unit::TonnePerCubicMetre => "tonne per cubic metre",
            Unit::TropicalYear => "tropical year",
            Unit::UnifiedAtomicMassUnit => "unified atomic mass unit",
            Unit::Var => "var",
            Unit::VoltSquaredPerKelvinSquared => "volt squared per kelvin squared",
            Unit::VoltAmpere => "volt - ampere",
            Unit::VoltPerCentimetre => "volt per centimetre",
            Unit::VoltPerKelvin => "volt per kelvin",
            Unit::MillivoltPerKelvin => "millivolt per kelvin",
            Unit::KilogramPerSquareCentimetre => "kilogram per square centimetre",
            Unit::VoltPerMetre => "volt per metre",
            Unit::VoltPerMillimetre => "volt per millimetre",
            Unit::WattPerKelvin => "watt per kelvin",
            Unit::WattPerMetreKelvin => "watt per metre kelvin",
            Unit::WattPerSquareMetre => "watt per square metre",
            Unit::WattPerSquareMetreKelvin => "watt per square metre kelvin",
            Unit::WattPerSquareMetreKelvinToFourthPower => "watt per square metre kelvin to the fourth power",
            Unit::WattPerSteradian => "watt per steradian",
            Unit::WattPerSteradianSquareMetre => "watt per steradian square metre",
            Unit::WeberPerMetre => "weber per metre",
            Unit::RoentgenPerSecond => "roentgen per second",
            Unit::WeberPerMillimetre => "weber per millimetre",
            Unit::MinuteUnitAngle => "minute [unit of angle]",
            Unit::SecondUnitAngle => "second [unit of angle]",
            Unit::Book => "book",
            Unit::Round => "round",
            Unit::NumberWords => "number of words",
            Unit::InchToFourthPower => "inch to the fourth power",
            Unit::JouleSquareMetre => "joule square metre",
            Unit::KilogramPerMole => "kilogram per mole",
            Unit::Megacoulomb => "megacoulomb",
            Unit::MegajoulePerSecond => "megajoule per second",
            Unit::Microwatt => "microwatt",
            Unit::Microtesla => "microtesla",
            Unit::Microvolt => "microvolt",
            Unit::MillinewtonMetre => "millinewton metre",
            Unit::MicrowattPerSquareMetre => "microwatt per square metre",
            Unit::Millicoulomb => "millicoulomb",
            Unit::MillimolePerKilogram => "millimole per kilogram",
            Unit::MillicoulombPerCubicMetre => "millicoulomb per cubic metre",
            Unit::MillicoulombPerSquareMetre => "millicoulomb per square metre",
            Unit::Rem => "rem",
            Unit::SecondPerCubicMetre => "second per cubic metre",
            Unit::SecondPerCubicMetreRadian => "second per cubic metre radian",
            Unit::JoulePerGram => "joule per gram",
            Unit::Decare => "decare",
            Unit::TenDay => "ten day",
            Unit::Day => "day",
            Unit::DryPound => "dry pound",
            Unit::DecibelMilliwatts => "Decibel-milliwatts",
            Unit::DecibelWatt => "Decibel watt",
            Unit::DegreeUnitAngle => "degree [unit of angle]",
            Unit::Decade => "decade",
            Unit::Decigram => "decigram",
            Unit::Decagram => "decagram",
            Unit::Decilitre => "decilitre",
            Unit::CubicDecametre => "cubic decametre",
            Unit::SquareDecimetre => "square decimetre",
            Unit::StandardKilolitre => "standard kilolitre",
            Unit::CubicDecimetre => "cubic decimetre",
            Unit::Decimetre => "decimetre",
            Unit::DecinewtonMetre => "decinewton metre",
            Unit::DozenPiece => "dozen piece",
            Unit::DozenPair => "dozen pair",
            Unit::DisplacementTonnage => "displacement tonnage",
            Unit::DramUs => "dram (US)",
            Unit::DramUk => "dram (UK)",
            Unit::DozenRoll => "dozen roll",
            Unit::DryTon => "dry ton",
            Unit::Decitonne => "decitonne",
            Unit::Pennyweight => "pennyweight",
            Unit::Dozen => "dozen",
            Unit::DozenPack => "dozen pack",
            Unit::NewtonPerSquareCentimetre => "newton per square centimetre",
            Unit::MegawattHourPerHour => "megawatt hour per hour",
            Unit::MegawattPerHertz => "megawatt per hertz",
            Unit::MilliampereHour => "milliampere hour",
            Unit::DegreeDay => "degree day",
            Unit::Mille => "mille",
            Unit::KilocalorieInternationalTable => "kilocalorie (international table)",
            Unit::KilocalorieThermochemicalPerHour => "kilocalorie (thermochemical) per hour",
            Unit::MillionBtuItPerHour => "million Btu(IT) per hour",
            Unit::CubicFootPerSecond => "cubic foot per second",
            Unit::TonnePerHour => "tonne per hour",
            Unit::Ping => "ping",
            Unit::MegabitPerSecond => "megabit per second",
            Unit::Shares => "shares",
            Unit::Teu => "TEU",
            Unit::Tyre => "tyre",
            Unit::ActiveUnit => "active unit",
            Unit::Dose => "dose",
            Unit::AirDryTon => "air dry ton",
            Unit::Strand => "strand",
            Unit::SquareMetrePerLitre => "square metre per litre",
            Unit::LitrePerHour => "litre per hour",
            Unit::FootPerThousand => "foot per thousand",
            Unit::Gigabyte => "gigabyte",
            Unit::Terabyte => "terabyte",
            Unit::Petabyte => "petabyte",
            Unit::Pixel => "pixel",
            Unit::Megapixel => "megapixel",
            Unit::DotsPerInch => "dots per inch",
            Unit::GrossKilogram => "gross kilogram",
            Unit::PartPerHundredThousand => "part per hundred thousand",
            Unit::KilogramForcePerSquareMillimetre => "kilogram-force per square millimetre",
            Unit::KilogramForcePerSquareCentimetre => "kilogram-force per square centimetre",
            Unit::JoulePerSquareCentimetre => "joule per square centimetre",
            Unit::KilogramForceMetrePerSquareCentimetre => "kilogram-force metre per square centimetre",
            Unit::Milliohm => "milliohm",
            Unit::KilowattHourPerCubicMetre => "kilowatt hour per cubic metre",
            Unit::KilowattHourPerKelvin => "kilowatt hour per kelvin",
            Unit::ServiceUnit => "service unit",
            Unit::WorkingDay => "working day",
            Unit::AccountingUnit => "accounting unit",
            Unit::Job => "job",
            Unit::RunFoot => "run foot",
            Unit::Test => "test",
            Unit::Trip => "trip",
            Unit::Use => "use",
            Unit::Well => "well",
            Unit::Zone => "zone",
            Unit::ExabitPerSecond => "exabit per second",
            Unit::Exbibyte => "exbibyte",
            Unit::Pebibyte => "pebibyte",
            Unit::Tebibyte => "tebibyte",
            Unit::Gibibyte => "gibibyte",
            Unit::Mebibyte => "mebibyte",
            Unit::Kibibyte => "kibibyte",
            Unit::ExbibitPerMetre => "exbibit per metre",
            Unit::ExbibitPerSquareMetre => "exbibit per square metre",
            Unit::ExbibitPerCubicMetre => "exbibit per cubic metre",
            Unit::GigabytePerSecond => "gigabyte per second",
            Unit::GibibitPerMetre => "gibibit per metre",
            Unit::GibibitPerSquareMetre => "gibibit per square metre",
            Unit::GibibitPerCubicMetre => "gibibit per cubic metre",
            Unit::KibibitPerMetre => "kibibit per metre",
            Unit::KibibitPerSquareMetre => "kibibit per square metre",
            Unit::KibibitPerCubicMetre => "kibibit per cubic metre",
            Unit::MebibitPerMetre => "mebibit per metre",
            Unit::MebibitPerSquareMetre => "mebibit per square metre",
            Unit::MebibitPerCubicMetre => "mebibit per cubic metre",
            Unit::Petabit => "petabit",
            Unit::PetabitPerSecond => "petabit per second",
            Unit::PebibitPerMetre => "pebibit per metre",
            Unit::PebibitPerSquareMetre => "pebibit per square metre",
            Unit::PebibitPerCubicMetre => "pebibit per cubic metre",
            Unit::Terabit => "terabit",
            Unit::TerabitPerSecond => "terabit per second",
            Unit::TebibitPerMetre => "tebibit per metre",
            Unit::TebibitPerCubicMetre => "tebibit per cubic metre",
            Unit::TebibitPerSquareMetre => "tebibit per square metre",
            Unit::BitPerMetre => "bit per metre",
            Unit::BitPerSquareMetre => "bit per square metre",
            Unit::ReciprocalCentimetre => "reciprocal centimetre",
            Unit::ReciprocalDay => "reciprocal day",
            Unit::CubicDecimetrePerHour => "cubic decimetre per hour",
            Unit::KilogramPerHour => "kilogram per hour",
            Unit::KilomolePerSecond => "kilomole per second",
            Unit::MolePerSecond => "mole per second",
            Unit::DegreePerSecond => "degree per second",
            Unit::MillimetrePerDegreeCelciusMetre => "millimetre per degree Celcius metre",
            Unit::DegreeCelsiusPerKelvin => "degree Celsius per kelvin",
            Unit::HectopascalPerBar => "hectopascal per bar",
            Unit::Each => "each",
            Unit::ElectronicMailBox => "electronic mail box",
            Unit::EquivalentGallon => "equivalent gallon",
            Unit::BitPerCubicMetre => "bit per cubic metre",
            Unit::KelvinPerKelvin => "kelvin per kelvin",
            Unit::KilopascalPerBar => "kilopascal per bar",
            Unit::MillibarPerBar => "millibar per bar",
            Unit::MegapascalPerBar => "megapascal per bar",
            Unit::PoisePerBar => "poise per bar",
            Unit::PascalPerBar => "pascal per bar",
            Unit::MilliamperePerInch => "milliampere per inch",
            Unit::KelvinPerHour => "kelvin per hour",
            Unit::KelvinPerMinute => "kelvin per minute",
            Unit::KelvinPerSecond => "kelvin per second",
            Unit::Slug => "slug",
            Unit::GramPerKelvin => "gram per kelvin",
            Unit::KilogramPerKelvin => "kilogram per kelvin",
            Unit::MilligramPerKelvin => "milligram per kelvin",
            Unit::PoundForcePerFoot => "pound-force per foot",
            Unit::KilogramSquareCentimetre => "kilogram square centimetre",
            Unit::KilogramSquareMillimetre => "kilogram square millimetre",
            Unit::PoundInchSquared => "pound inch squared",
            Unit::PoundForceInch => "pound-force inch",
            Unit::PoundForceFootPerAmpere => "pound-force foot per ampere",
            Unit::GramPerCubicDecimetre => "gram per cubic decimetre",
            Unit::KilogramPerKilomol => "kilogram per kilomol",
            Unit::GramPerHertz => "gram per hertz",
            Unit::GramPerDay => "gram per day",
            Unit::GramPerHour => "gram per hour",
            Unit::GramPerMinute => "gram per minute",
            Unit::GramPerSecond => "gram per second",
            Unit::KilogramPerDay => "kilogram per day",
            Unit::KilogramPerMinute => "kilogram per minute",
            Unit::MilligramPerDay => "milligram per day",
            Unit::MilligramPerMinute => "milligram per minute",
            Unit::MilligramPerSecond => "milligram per second",
            Unit::GramPerDayKelvin => "gram per day kelvin",
            Unit::GramPerHourKelvin => "gram per hour kelvin",
            Unit::GramPerMinuteKelvin => "gram per minute kelvin",
            Unit::GramPerSecondKelvin => "gram per second kelvin",
            Unit::KilogramPerDayKelvin => "kilogram per day kelvin",
            Unit::KilogramPerHourKelvin => "kilogram per hour kelvin",
            Unit::KilogramPerMinuteKelvin => "kilogram per minute kelvin",
            Unit::KilogramPerSecondKelvin => "kilogram per second kelvin",
            Unit::MilligramPerDayKelvin => "milligram per day kelvin",
            Unit::MilligramPerHourKelvin => "milligram per hour kelvin",
            Unit::MilligramPerMinuteKelvin => "milligram per minute kelvin",
            Unit::MilligramPerSecondKelvin => "milligram per second kelvin",
            Unit::NewtonPerMillimetre => "newton per millimetre",
            Unit::PoundForcePerInch => "pound-force per inch",
            Unit::RodUnitDistance => "rod [unit of distance]",
            Unit::MicrometrePerKelvin => "micrometre per kelvin",
            Unit::CentimetrePerKelvin => "centimetre per kelvin",
            Unit::MetrePerKelvin => "metre per kelvin",
            Unit::MillimetrePerKelvin => "millimetre per kelvin",
            Unit::MilliohmPerMetre => "milliohm per metre",
            Unit::OhmPerMileStatuteMile => "ohm per mile (statute mile)",
            Unit::OhmPerKilometre => "ohm per kilometre",
            Unit::MilliamperePerPoundForcePerSquareInch => "milliampere per pound-force per square inch",
            Unit::ReciprocalBar => "reciprocal bar",
            Unit::MilliamperePerBar => "milliampere per bar",
            Unit::DegreeCelsiusPerBar => "degree Celsius per bar",
            Unit::KelvinPerBar => "kelvin per bar",
            Unit::GramPerDayBar => "gram per day bar",
            Unit::GramPerHourBar => "gram per hour bar",
            Unit::GramPerMinuteBar => "gram per minute bar",
            Unit::GramPerSecondBar => "gram per second bar",
            Unit::KilogramPerDayBar => "kilogram per day bar",
            Unit::KilogramPerHourBar => "kilogram per hour bar",
            Unit::KilogramPerMinuteBar => "kilogram per minute bar",
            Unit::KilogramPerSecondBar => "kilogram per second bar",
            Unit::MilligramPerDayBar => "milligram per day bar",
            Unit::MilligramPerHourBar => "milligram per hour bar",
            Unit::MilligramPerMinuteBar => "milligram per minute bar",
            Unit::MilligramPerSecondBar => "milligram per second bar",
            Unit::GramPerBar => "gram per bar",
            Unit::MilligramPerBar => "milligram per bar",
            Unit::MilliamperePerMillimetre => "milliampere per millimetre",
            Unit::PascalSecondPerKelvin => "pascal second per kelvin",
            Unit::InchWater => "inch of water",
            Unit::InchMercury => "inch of mercury",
            Unit::WaterHorsePower => "water horse power",
            Unit::BarPerKelvin => "bar per kelvin",
            Unit::HectopascalPerKelvin => "hectopascal per kelvin",
            Unit::KilopascalPerKelvin => "kilopascal per kelvin",
            Unit::MillibarPerKelvin => "millibar per kelvin",
            Unit::MegapascalPerKelvin => "megapascal per kelvin",
            Unit::PoisePerKelvin => "poise per kelvin",
            Unit::VoltPerLitreMinute => "volt per litre minute",
            Unit::NewtonCentimetre => "newton centimetre",
            Unit::NewtonMetrePerDegree => "newton metre per degree",
            Unit::NewtonMetrePerAmpere => "newton metre per ampere",
            Unit::BarLitrePerSecond => "bar litre per second",
            Unit::BarCubicMetrePerSecond => "bar cubic metre per second",
            Unit::HectopascalLitrePerSecond => "hectopascal litre per second",
            Unit::HectopascalCubicMetrePerSecond => "hectopascal cubic metre per second",
            Unit::MillibarLitrePerSecond => "millibar litre per second",
            Unit::MillibarCubicMetrePerSecond => "millibar cubic metre per second",
            Unit::MegapascalLitrePerSecond => "megapascal litre per second",
            Unit::MegapascalCubicMetrePerSecond => "megapascal cubic metre per second",
            Unit::PascalLitrePerSecond => "pascal litre per second",
            Unit::DegreeFahrenheit => "degree Fahrenheit",
            Unit::Farad => "farad",
            Unit::FibreMetre => "fibre metre",
            Unit::ThousandCubicFoot => "thousand cubic foot",
            Unit::HundredCubicMetre => "hundred cubic metre",
            Unit::Micromole => "micromole",
            Unit::FailuresInTime => "failures in time",
            Unit::FlakeTon => "flake ton",
            Unit::FormazinNephelometricUnit => "Formazin nephelometric unit",
            Unit::Foot => "foot",
            Unit::PoundPerSquareFoot => "pound per square foot",
            Unit::FootPerMinute => "foot per minute",
            Unit::FootPerSecond => "foot per second",
            Unit::SquareFoot => "square foot",
            Unit::CubicFoot => "cubic foot",
            Unit::PascalCubicMetrePerSecond => "pascal cubic metre per second",
            Unit::CentimetrePerBar => "centimetre per bar",
            Unit::MetrePerBar => "metre per bar",
            Unit::MillimetrePerBar => "millimetre per bar",
            Unit::SquareInchPerSecond => "square inch per second",
            Unit::SquareMetrePerSecondKelvin => "square metre per second kelvin",
            Unit::StokesPerKelvin => "stokes per kelvin",
            Unit::GramPerCubicCentimetreBar => "gram per cubic centimetre bar",
            Unit::GramPerCubicDecimetreBar => "gram per cubic decimetre bar",
            Unit::GramPerLitreBar => "gram per litre bar",
            Unit::GramPerCubicMetreBar => "gram per cubic metre bar",
            Unit::GramPerMillilitreBar => "gram per millilitre bar",
            Unit::KilogramPerCubicCentimetreBar => "kilogram per cubic centimetre bar",
            Unit::KilogramPerLitreBar => "kilogram per litre bar",
            Unit::KilogramPerCubicMetreBar => "kilogram per cubic metre bar",
            Unit::NewtonMetrePerKilogram => "newton metre per kilogram",
            Unit::UsGallonPerMinute => "US gallon per minute",
            Unit::PoundForceFootPerPound => "pound-force foot per pound",
            Unit::CupUnitVolume => "cup [unit of volume]",
            Unit::Peck => "peck",
            Unit::TablespoonUs => "tablespoon (US)",
            Unit::TeaspoonUs => "teaspoon (US)",
            Unit::Stere => "stere",
            Unit::CubicCentimetrePerKelvin => "cubic centimetre per kelvin",
            Unit::LitrePerKelvin => "litre per kelvin",
            Unit::CubicMetrePerKelvin => "cubic metre per kelvin",
            Unit::ImperialGallonPerMinute => "Imperial gallon per minute",
            Unit::MillilitrePerKelvin => "millilitre per kelvin",
            Unit::KilogramPerCubicCentimetre => "kilogram per cubic centimetre",
            Unit::OunceAvoirdupoisPerCubicYard => "ounce (avoirdupois) per cubic yard",
            Unit::GramPerCubicCentimetreKelvin => "gram per cubic centimetre kelvin",
            Unit::GramPerCubicDecimetreKelvin => "gram per cubic decimetre kelvin",
            Unit::GramPerLitreKelvin => "gram per litre kelvin",
            Unit::GramPerCubicMetreKelvin => "gram per cubic metre kelvin",
            Unit::GramPerMillilitreKelvin => "gram per millilitre kelvin",
            Unit::KilogramPerCubicCentimetreKelvin => "kilogram per cubic centimetre kelvin",
            Unit::KilogramPerLitreKelvin => "kilogram per litre kelvin",
            Unit::KilogramPerCubicMetreKelvin => "kilogram per cubic metre kelvin",
            Unit::SquareMetrePerSecondBar => "square metre per second bar",
            Unit::MicrosiemensPerCentimetre => "microsiemens per centimetre",
            Unit::MicrosiemensPerMetre => "microsiemens per metre",
            Unit::NanosiemensPerCentimetre => "nanosiemens per centimetre",
            Unit::NanosiemensPerMetre => "nanosiemens per metre",
            Unit::StokesPerBar => "stokes per bar",
            Unit::CubicCentimetrePerDay => "cubic centimetre per day",
            Unit::CubicCentimetrePerHour => "cubic centimetre per hour",
            Unit::CubicCentimetrePerMinute => "cubic centimetre per minute",
            Unit::GallonUsPerHour => "gallon (US) per hour",
            Unit::LitrePerSecond => "litre per second",
            Unit::CubicMetrePerDay => "cubic metre per day",
            Unit::CubicMetrePerMinute => "cubic metre per minute",
            Unit::MillilitrePerDay => "millilitre per day",
            Unit::MillilitrePerHour => "millilitre per hour",
            Unit::CubicInchPerHour => "cubic inch per hour",
            Unit::CubicInchPerMinute => "cubic inch per minute",
            Unit::CubicInchPerSecond => "cubic inch per second",
            Unit::MilliamperePerLitreMinute => "milliampere per litre minute",
            Unit::VoltPerBar => "volt per bar",
            Unit::CubicCentimetrePerDayKelvin => "cubic centimetre per day kelvin",
            Unit::CubicCentimetrePerHourKelvin => "cubic centimetre per hour kelvin",
            Unit::CubicCentimetrePerMinuteKelvin => "cubic centimetre per minute kelvin",
            Unit::CubicCentimetrePerSecondKelvin => "cubic centimetre per second kelvin",
            Unit::LitrePerDayKelvin => "litre per day kelvin",
            Unit::LitrePerHourKelvin => "litre per hour kelvin",
            Unit::LitrePerMinuteKelvin => "litre per minute kelvin",
            Unit::LitrePerSecondKelvin => "litre per second kelvin",
            Unit::CubicMetrePerDayKelvin => "cubic metre per day kelvin",
            Unit::CubicMetrePerHourKelvin => "cubic metre per hour kelvin",
            Unit::CubicMetrePerMinuteKelvin => "cubic metre per minute kelvin",
            Unit::CubicMetrePerSecondKelvin => "cubic metre per second kelvin",
            Unit::MillilitrePerDayKelvin => "millilitre per day kelvin",
            Unit::MillilitrePerHourKelvin => "millilitre per hour kelvin",
            Unit::MillilitrePerMinuteKelvin => "millilitre per minute kelvin",
            Unit::MillilitrePerSecondKelvin => "millilitre per second kelvin",
            Unit::MillimetreToFourthPower => "millimetre to the fourth power",
            Unit::CubicCentimetrePerDayBar => "cubic centimetre per day bar",
            Unit::CubicCentimetrePerHourBar => "cubic centimetre per hour bar",
            Unit::CubicCentimetrePerMinuteBar => "cubic centimetre per minute bar",
            Unit::CubicCentimetrePerSecondBar => "cubic centimetre per second bar",
            Unit::LitrePerDayBar => "litre per day bar",
            Unit::LitrePerHourBar => "litre per hour bar",
            Unit::LitrePerMinuteBar => "litre per minute bar",
            Unit::LitrePerSecondBar => "litre per second bar",
            Unit::CubicMetrePerDayBar => "cubic metre per day bar",
            Unit::CubicMetrePerHourBar => "cubic metre per hour bar",
            Unit::CubicMetrePerMinuteBar => "cubic metre per minute bar",
            Unit::CubicMetrePerSecondBar => "cubic metre per second bar",
            Unit::MillilitrePerDayBar => "millilitre per day bar",
            Unit::MillilitrePerHourBar => "millilitre per hour bar",
            Unit::MillilitrePerMinuteBar => "millilitre per minute bar",
            Unit::MillilitrePerSecondBar => "millilitre per second bar",
            Unit::CubicCentimetrePerBar => "cubic centimetre per bar",
            Unit::LitrePerBar => "litre per bar",
            Unit::CubicMetrePerBar => "cubic metre per bar",
            Unit::MillilitrePerBar => "millilitre per bar",
            Unit::MicrohenryPerKiloohm => "microhenry per kiloohm",
            Unit::MicrohenryPerOhm => "microhenry per ohm",
            Unit::GallonUsPerDay => "gallon (US) per day",
            Unit::Gigabecquerel => "gigabecquerel",
            Unit::GramDryWeight => "gram, dry weight",
            Unit::PoundPerGallonUs => "pound per gallon (US)",
            Unit::GramPerMetreGramPer100Centimetres => "gram per metre (gram per 100 centimetres)",
            Unit::GramFissileIsotope => "gram of fissile isotope",
            Unit::GreatGross => "great gross",
            Unit::GillUs => "gill (US)",
            Unit::GramIncludingContainer => "gram, including container",
            Unit::GillUk => "gill (UK)",
            Unit::GramIncludingInnerPackaging => "gram, including inner packaging",
            Unit::GramPerMillilitre => "gram per millilitre",
            Unit::GramPerLitre => "gram per litre",
            Unit::DryGallonUs => "dry gallon (US)",
            Unit::GallonUk => "gallon (UK)",
            Unit::GallonUs => "gallon (US)",
            Unit::GramPerSquareMetre => "gram per square metre",
            Unit::MilligramPerSquareMetre => "milligram per square metre",
            Unit::MilligramPerCubicMetre => "milligram per cubic metre",
            Unit::MicrogramPerCubicMetre => "microgram per cubic metre",
            Unit::Gram => "gram",
            Unit::Grain => "grain",
            Unit::Gross => "gross",
            Unit::Gigajoule => "gigajoule",
            Unit::GigawattHour => "gigawatt hour",
            Unit::HenryPerKiloohm => "henry per kiloohm",
            Unit::HenryPerOhm => "henry per ohm",
            Unit::MillihenryPerKiloohm => "millihenry per kiloohm",
            Unit::MillihenryPerOhm => "millihenry per ohm",
            Unit::PascalSecondPerBar => "pascal second per bar",
            Unit::Microbecquerel => "microbecquerel",
            Unit::ReciprocalYear => "reciprocal year",
            Unit::ReciprocalHour => "reciprocal hour",
            Unit::ReciprocalMonth => "reciprocal month",
            Unit::DegreeCelsiusPerHour => "degree Celsius per hour",
            Unit::DegreeCelsiusPerMinute => "degree Celsius per minute",
            Unit::DegreeCelsiusPerSecond => "degree Celsius per second",
            Unit::SquareCentimetrePerGram => "square centimetre per gram",
            Unit::SquareDecametre => "square decametre",
            Unit::SquareHectometre => "square hectometre",
            Unit::CubicHectometre => "cubic hectometre",
            Unit::CubicKilometre => "cubic kilometre",
            Unit::Blank => "blank",
            Unit::VoltSquareInchPerPoundForce => "volt square inch per pound-force",
            Unit::VoltPerInch => "volt per inch",
            Unit::VoltPerMicrosecond => "volt per microsecond",
            Unit::PercentPerKelvin => "percent per kelvin",
            Unit::OhmPerMetre => "ohm per metre",
            Unit::DegreePerMetre => "degree per metre",
            Unit::MicrofaradPerKilometre => "microfarad per kilometre",
            Unit::MicrogramPerLitre => "microgram per litre",
            Unit::SquareMicrometreSquareMicron => "square micrometre (square micron)",
            Unit::AmperePerKilogram => "ampere per kilogram",
            Unit::AmpereSquaredSecond => "ampere squared second",
            Unit::FaradPerKilometre => "farad per kilometre",
            Unit::HertzMetre => "hertz metre",
            Unit::KelvinMetrePerWatt => "kelvin metre per watt",
            Unit::MegaohmPerKilometre => "megaohm per kilometre",
            Unit::MegaohmPerMetre => "megaohm per metre",
            Unit::Megaampere => "megaampere",
            Unit::MegahertzKilometre => "megahertz kilometre",
            Unit::NewtonPerAmpere => "newton per ampere",
            Unit::NewtonMetreWattToPowerMinus05 => "newton metre watt to the power minus 0,5",
            Unit::PascalPerMetre => "pascal per metre",
            Unit::SiemensPerCentimetre => "siemens per centimetre",
            Unit::Teraohm => "teraohm",
            Unit::VoltSecondPerMetre => "volt second per metre",
            Unit::VoltPerSecond => "volt per second",
            Unit::WattPerCubicMetre => "watt per cubic metre",
            Unit::Attofarad => "attofarad",
            Unit::CentimetrePerHour => "centimetre per hour",
            Unit::ReciprocalCubicCentimetre => "reciprocal cubic centimetre",
            Unit::DecibelPerKilometre => "decibel per kilometre",
            Unit::DecibelPerMetre => "decibel per metre",
            Unit::KilogramPerBar => "kilogram per bar",
            Unit::KilogramPerCubicDecimetreKelvin => "kilogram per cubic decimetre kelvin",
            Unit::KilogramPerCubicDecimetreBar => "kilogram per cubic decimetre bar",
            Unit::KilogramPerSquareMetreSecond => "kilogram per square metre second",
            Unit::InchPerTwoPiRadiant => "inch per two pi radiant",
            Unit::MetrePerVoltSecond => "metre per volt second",
            Unit::SquareMetrePerNewton => "square metre per newton",
            Unit::CubicMetrePerCubicMetre => "cubic metre per cubic metre",
            Unit::MillisiemensPerCentimetre => "millisiemens per centimetre",
            Unit::MillivoltPerMinute => "millivolt per minute",
            Unit::MilligramPerSquareCentimetre => "milligram per square centimetre",
            Unit::MilligramPerGram => "milligram per gram",
            Unit::MillilitrePerCubicMetre => "millilitre per cubic metre",
            Unit::MillimetrePerYear => "millimetre per year",
            Unit::MillimetrePerHour => "millimetre per hour",
            Unit::MillimolePerGram => "millimole per gram",
            Unit::PicopascalPerKilometre => "picopascal per kilometre",
            Unit::Picosecond => "picosecond",
            Unit::PercentPerMonth => "percent per month",
            Unit::PercentPerHectobar => "percent per hectobar",
            Unit::PercentPerDecakelvin => "percent per decakelvin",
            Unit::WattPerMetre => "watt per metre",
            Unit::Decapascal => "decapascal",
            Unit::GramPerMillimetre => "gram per millimetre",
            Unit::ModuleWidth => "module width",
            Unit::FrenchGauge => "French gauge",
            Unit::RackUnit => "rack unit",
            Unit::MillimetrePerMinute => "millimetre per minute",
            Unit::BigPoint => "big point",
            Unit::LitrePerKilogram => "litre per kilogram",
            Unit::GramMillimetre => "gram millimetre",
            Unit::ReciprocalWeek => "reciprocal week",
            Unit::Piece => "piece",
            Unit::MegaohmKilometre => "megaohm kilometre",
            Unit::PercentPerOhm => "percent per ohm",
            Unit::PercentPerDegree => "percent per degree",
            Unit::PercentPerTenThousand => "percent per ten thousand",
            Unit::PercentPerOneHundredThousand => "percent per one hundred thousand",
            Unit::PercentPerHundred => "percent per hundred",
            Unit::PercentPerThousand => "percent per thousand",
            Unit::PercentPerVolt => "percent per volt",
            Unit::PercentPerBar => "percent per bar",
            Unit::PercentPerInch => "percent per inch",
            Unit::PercentPerMetre => "percent per metre",
            Unit::Hank => "hank",
            Unit::PieceDay => "Piece Day",
            Unit::Hectobar => "hectobar",
            Unit::HundredBoxes => "hundred boxes",
            Unit::HundredCount => "hundred count",
            Unit::HundredKilogramDryWeight => "hundred kilogram, dry weight",
            Unit::Head => "head",
            Unit::Hectogram => "hectogram",
            Unit::HundredCubicFoot => "hundred cubic foot",
            Unit::HundredInternationalUnit => "hundred international unit",
            Unit::HundredKilogramNetMass => "hundred kilogram, net mass",
            Unit::Hectolitre => "hectolitre",
            Unit::MilePerHourStatuteMile => "mile per hour (statute mile)",
            Unit::PieceMonth => "Piece Month",
            Unit::MillionCubicMetre => "million cubic metre",
            Unit::Hectometre => "hectometre",
            Unit::HectolitrePureAlcohol => "hectolitre of pure alcohol",
            Unit::Hertz => "hertz",
            Unit::Hour => "hour",
            Unit::PieceWeek => "Piece Week",
            Unit::InchPoundPoundInch => "inch pound (pound inch)",
            Unit::Person => "person",
            Unit::Inch => "inch",
            Unit::SquareInch => "square inch",
            Unit::CubicInch => "cubic inch",
            Unit::InternationalSugarDegree => "international sugar degree",
            Unit::InchPerSecond => "inch per second",
            Unit::InternationalUnitPerGram => "international unit per gram",
            Unit::InchPerSecondSquared => "inch per second squared",
            Unit::PercentPerMillimetre => "percent per millimetre",
            Unit::PerMillePerPsi => "per mille per psi",
            Unit::DegreeApi => "degree API",
            Unit::DegreeBaumeOriginScale => "degree Baume (origin scale)",
            Unit::DegreeBaumeUsHeavy => "degree Baume (US heavy)",
            Unit::DegreeBaumeUsLight => "degree Baume (US light)",
            Unit::DegreeBalling => "degree Balling",
            Unit::DegreeBrix => "degree Brix",
            Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitThermochemical => "degree Fahrenheit hour square foot per British thermal unit (thermochemical)",
            Unit::JoulePerKilogram => "joule per kilogram",
            Unit::DegreeFahrenheitPerKelvin => "degree Fahrenheit per kelvin",
            Unit::DegreeFahrenheitPerBar => "degree Fahrenheit per bar",
            Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitInternationalTable => "degree Fahrenheit hour square foot per British thermal unit (international table)",
            Unit::DegreeFahrenheitPerHour => "degree Fahrenheit per hour",
            Unit::DegreeFahrenheitPerMinute => "degree Fahrenheit per minute",
            Unit::DegreeFahrenheitPerSecond => "degree Fahrenheit per second",
            Unit::ReciprocalDegreeFahrenheit => "reciprocal degree Fahrenheit",
            Unit::DegreeOechsle => "degree Oechsle",
            Unit::DegreeRankinePerHour => "degree Rankine per hour",
            Unit::DegreeRankinePerMinute => "degree Rankine per minute",
            Unit::DegreeRankinePerSecond => "degree Rankine per second",
            Unit::DegreeTwaddell => "degree Twaddell",
            Unit::Micropoise => "micropoise",
            Unit::MicrogramPerKilogram => "microgram per kilogram",
            Unit::MicrogramPerCubicMetreKelvin => "microgram per cubic metre kelvin",
            Unit::MicrogramPerCubicMetreBar => "microgram per cubic metre bar",
            Unit::MicrolitrePerLitre => "microlitre per litre",
            Unit::Baud => "baud",
            Unit::BritishThermalUnitMean => "British thermal unit (mean)",
            Unit::BritishThermalUnitInternationalTableFootPerHourSquareFootDegreeFahrenheit => "British thermal unit (international table) foot per hour square foot degree Fahrenheit",
            Unit::BritishThermalUnitInternationalTableInchPerHourSquareFootDegreeFahrenheit => "British thermal unit (international table) inch per hour square foot degree Fahrenheit",
            Unit::BritishThermalUnitInternationalTableInchPerSecondSquareFootDegreeFahrenheit => "British thermal unit (international table) inch per second square foot degree Fahrenheit",
            Unit::BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit => "British thermal unit (international table) per pound degree Fahrenheit",
            Unit::BritishThermalUnitInternationalTablePerMinute => "British thermal unit (international table) per minute",
            Unit::BritishThermalUnitInternationalTablePerSecond => "British thermal unit (international table) per second",
            Unit::BritishThermalUnitThermochemicalFootPerHourSquareFootDegreeFahrenheit => "British thermal unit (thermochemical) foot per hour square foot degree Fahrenheit",
            Unit::BritishThermalUnitThermochemicalPerHour => "British thermal unit (thermochemical) per hour",
            Unit::BritishThermalUnitThermochemicalInchPerHourSquareFootDegreeFahrenheit => "British thermal unit (thermochemical) inch per hour square foot degree Fahrenheit",
            Unit::BritishThermalUnitThermochemicalInchPerSecondSquareFootDegreeFahrenheit => "British thermal unit (thermochemical) inch per second square foot degree Fahrenheit",
            Unit::BritishThermalUnitThermochemicalPerPoundDegreeFahrenheit => "British thermal unit (thermochemical) per pound degree Fahrenheit",
            Unit::BritishThermalUnitThermochemicalPerMinute => "British thermal unit (thermochemical) per minute",
            Unit::BritishThermalUnitThermochemicalPerSecond => "British thermal unit (thermochemical) per second",
            Unit::CoulombSquareMetrePerKilogram => "coulomb square metre per kilogram",
            Unit::Megabaud => "megabaud",
            Unit::WattSecond => "watt second",
            Unit::BarPerBar => "bar per bar",
            Unit::BarrelUkPetroleum => "barrel (UK petroleum)",
            Unit::BarrelUkPetroleumPerMinute => "barrel (UK petroleum) per minute",
            Unit::BarrelUkPetroleumPerDay => "barrel (UK petroleum) per day",
            Unit::BarrelUkPetroleumPerHour => "barrel (UK petroleum) per hour",
            Unit::BarrelUkPetroleumPerSecond => "barrel (UK petroleum) per second",
            Unit::BarrelUsPetroleumPerHour => "barrel (US petroleum) per hour",
            Unit::BarrelUsPetroleumPerSecond => "barrel (US petroleum) per second",
            Unit::BushelUkPerDay => "bushel (UK) per day",
            Unit::BushelUkPerHour => "bushel (UK) per hour",
            Unit::BushelUkPerMinute => "bushel (UK) per minute",
            Unit::BushelUkPerSecond => "bushel (UK) per second",
            Unit::BushelUsDryPerDay => "bushel (US dry) per day",
            Unit::BushelUsDryPerHour => "bushel (US dry) per hour",
            Unit::BushelUsDryPerMinute => "bushel (US dry) per minute",
            Unit::BushelUsDryPerSecond => "bushel (US dry) per second",
            Unit::CentinewtonMetre => "centinewton metre",
            Unit::CentipoisePerKelvin => "centipoise per kelvin",
            Unit::CentipoisePerBar => "centipoise per bar",
            Unit::CalorieMean => "calorie (mean)",
            Unit::CalorieInternationalTablePerGramDegreeCelsius => "calorie (international table) per gram degree Celsius",
            Unit::CalorieThermochemicalPerCentimetreSecondDegreeCelsius => "calorie (thermochemical) per centimetre second degree Celsius",
            Unit::CalorieThermochemicalPerGramDegreeCelsius => "calorie (thermochemical) per gram degree Celsius",
            Unit::CalorieThermochemicalPerMinute => "calorie (thermochemical) per minute",
            Unit::CalorieThermochemicalPerSecond => "calorie (thermochemical) per second",
            Unit::Clo => "clo",
            Unit::CentimetrePerSecondKelvin => "centimetre per second kelvin",
            Unit::CentimetrePerSecondBar => "centimetre per second bar",
            Unit::CubicCentimetrePerCubicMetre => "cubic centimetre per cubic metre",
            Unit::CubicDecimetrePerDay => "cubic decimetre per day",
            Unit::CubicDecimetrePerCubicMetre => "cubic decimetre per cubic metre",
            Unit::CubicDecimetrePerMinute => "cubic decimetre per minute",
            Unit::CubicDecimetrePerSecond => "cubic decimetre per second",
            Unit::OunceUkFluidPerDay => "ounce (UK fluid) per day",
            Unit::OunceUkFluidPerHour => "ounce (UK fluid) per hour",
            Unit::OunceUkFluidPerMinute => "ounce (UK fluid) per minute",
            Unit::OunceUkFluidPerSecond => "ounce (UK fluid) per second",
            Unit::OunceUsFluidPerDay => "ounce (US fluid) per day",
            Unit::JoulePerKelvin => "joule per kelvin",
            Unit::MegajoulePerKilogram => "megajoule per kilogram",
            Unit::MegajoulePerCubicMetre => "megajoule per cubic metre",
            Unit::PipelineJoint => "pipeline joint",
            Unit::Joule => "joule",
            Unit::HundredMetre => "hundred metre",
            Unit::NumberJewels => "number of jewels",
            Unit::KilowattDemand => "kilowatt demand",
            Unit::OunceUsFluidPerHour => "ounce (US fluid) per hour",
            Unit::OunceUsFluidPerMinute => "ounce (US fluid) per minute",
            Unit::OunceUsFluidPerSecond => "ounce (US fluid) per second",
            Unit::FootPerDegreeFahrenheit => "foot per degree Fahrenheit",
            Unit::FootPerHour => "foot per hour",
            Unit::FootPoundForcePerHour => "foot pound-force per hour",
            Unit::FootPoundForcePerMinute => "foot pound-force per minute",
            Unit::FootPerPsi => "foot per psi",
            Unit::FootPerSecondDegreeFahrenheit => "foot per second degree Fahrenheit",
            Unit::FootPerSecondPsi => "foot per second psi",
            Unit::KilovoltAmpereReactiveDemand => "kilovolt ampere reactive demand",
            Unit::ReciprocalCubicFoot => "reciprocal cubic foot",
            Unit::CubicFootPerDegreeFahrenheit => "cubic foot per degree Fahrenheit",
            Unit::CubicFootPerDay => "cubic foot per day",
            Unit::CubicFootPerPsi => "cubic foot per psi",
            Unit::GallonUkPerDay => "gallon (UK) per day",
            Unit::GallonUkPerHour => "gallon (UK) per hour",
            Unit::GallonUkPerSecond => "gallon (UK) per second",
            Unit::KilovoltAmpereReactiveHour => "kilovolt ampere reactive hour",
            Unit::GallonUsLiquidPerSecond => "gallon (US liquid) per second",
            Unit::GramForcePerSquareCentimetre => "gram-force per square centimetre",
            Unit::GillUkPerDay => "gill (UK) per day",
            Unit::GillUkPerHour => "gill (UK) per hour",
            Unit::GillUkPerMinute => "gill (UK) per minute",
            Unit::GillUkPerSecond => "gill (UK) per second",
            Unit::GillUsPerDay => "gill (US) per day",
            Unit::GillUsPerHour => "gill (US) per hour",
            Unit::GillUsPerMinute => "gill (US) per minute",
            Unit::GillUsPerSecond => "gill (US) per second",
            Unit::StandardAccelerationFreeFall => "standard acceleration of free fall",
            Unit::GrainPerGallonUs => "grain per gallon (US)",
            Unit::HorsepowerBoiler => "horsepower (boiler)",
            Unit::HorsepowerElectric => "horsepower (electric)",
            Unit::InchPerDegreeFahrenheit => "inch per degree Fahrenheit",
            Unit::InchPerPsi => "inch per psi",
            Unit::InchPerSecondDegreeFahrenheit => "inch per second degree Fahrenheit",
            Unit::InchPerSecondPsi => "inch per second psi",
            Unit::ReciprocalCubicInch => "reciprocal cubic inch",
            Unit::Kilobaud => "kilobaud",
            Unit::KilocalorieMean => "kilocalorie (mean)",
            Unit::KilocalorieInternationalTablePerHourMetreDegreeCelsius => "kilocalorie (international table) per hour metre degree Celsius",
            Unit::KilocalorieThermochemical => "kilocalorie (thermochemical)",
            Unit::KilocalorieThermochemicalPerMinute => "kilocalorie (thermochemical) per minute",
            Unit::KilocalorieThermochemicalPerSecond => "kilocalorie (thermochemical) per second",
            Unit::KilomolePerHour => "kilomole per hour",
            Unit::KilomolePerCubicMetreKelvin => "kilomole per cubic metre kelvin",
            Unit::Kilolitre => "kilolitre",
            Unit::KilomolePerCubicMetreBar => "kilomole per cubic metre bar",
            Unit::KilomolePerMinute => "kilomole per minute",
            Unit::LitrePerLitre => "litre per litre",
            Unit::ReciprocalLitre => "reciprocal litre",
            Unit::PoundAvoirdupoisPerDegreeFahrenheit => "pound (avoirdupois) per degree Fahrenheit",
            Unit::PoundAvoirdupoisSquareFoot => "pound (avoirdupois) square foot",
            Unit::PoundAvoirdupoisPerDay => "pound (avoirdupois) per day",
            Unit::PoundPerFootHour => "pound per foot hour",
            Unit::PoundPerFootSecond => "pound per foot second",
            Unit::PoundAvoirdupoisPerCubicFootDegreeFahrenheit => "pound (avoirdupois) per cubic foot degree Fahrenheit",
            Unit::PoundAvoirdupoisPerCubicFootPsi => "pound (avoirdupois) per cubic foot psi",
            Unit::PoundAvoirdupoisPerGallonUk => "pound (avoirdupois) per gallon (UK)",
            Unit::PoundAvoirdupoisPerHourDegreeFahrenheit => "pound (avoirdupois) per hour degree Fahrenheit",
            Unit::PoundAvoirdupoisPerHourPsi => "pound (avoirdupois) per hour psi",
            Unit::PoundAvoirdupoisPerCubicInchDegreeFahrenheit => "pound (avoirdupois) per cubic inch degree Fahrenheit",
            Unit::PoundAvoirdupoisPerCubicInchPsi => "pound (avoirdupois) per cubic inch psi",
            Unit::PoundAvoirdupoisPerPsi => "pound (avoirdupois) per psi",
            Unit::PoundAvoirdupoisPerMinute => "pound (avoirdupois) per minute",
            Unit::PoundAvoirdupoisPerMinuteDegreeFahrenheit => "pound (avoirdupois) per minute degree Fahrenheit",
            Unit::PoundAvoirdupoisPerMinutePsi => "pound (avoirdupois) per minute psi",
            Unit::PoundAvoirdupoisPerSecond => "pound (avoirdupois) per second",
            Unit::PoundAvoirdupoisPerSecondDegreeFahrenheit => "pound (avoirdupois) per second degree Fahrenheit",
            Unit::PoundAvoirdupoisPerSecondPsi => "pound (avoirdupois) per second psi",
            Unit::PoundPerCubicYard => "pound per cubic yard",
            Unit::PoundForcePerSquareFoot => "pound-force per square foot",
            Unit::PoundForcePerSquareInchDegreeFahrenheit => "pound-force per square inch degree Fahrenheit",
            Unit::PsiCubicInchPerSecond => "psi cubic inch per second",
            Unit::PsiLitrePerSecond => "psi litre per second",
            Unit::PsiCubicMetrePerSecond => "psi cubic metre per second",
            Unit::PsiCubicYardPerSecond => "psi cubic yard per second",
            Unit::PoundForceSecondPerSquareFoot => "pound-force second per square foot",
            Unit::PoundForceSecondPerSquareInch => "pound-force second per square inch",
            Unit::ReciprocalPsi => "reciprocal psi",
            Unit::QuartUkLiquidPerDay => "quart (UK liquid) per day",
            Unit::QuartUkLiquidPerHour => "quart (UK liquid) per hour",
            Unit::QuartUkLiquidPerMinute => "quart (UK liquid) per minute",
            Unit::QuartUkLiquidPerSecond => "quart (UK liquid) per second",
            Unit::QuartUsLiquidPerDay => "quart (US liquid) per day",
            Unit::QuartUsLiquidPerHour => "quart (US liquid) per hour",
            Unit::Cake => "cake",
            Unit::Katal => "katal",
            Unit::Kilocharacter => "kilocharacter",
            Unit::Kilobar => "kilobar",
            Unit::KilogramCholineChloride => "kilogram of choline chloride",
            Unit::KilogramDrainedNetWeight => "kilogram drained net weight",
            Unit::Kelvin => "kelvin",
            Unit::Kilogram => "kilogram",
            Unit::KilogramPerSecond => "kilogram per second",
            Unit::KilogramHydrogenPeroxide => "kilogram of hydrogen peroxide",
            Unit::Kilohertz => "kilohertz",
            Unit::KilogramPerMillimetreWidth => "kilogram per millimetre width",
            Unit::KilogramIncludingContainer => "kilogram, including container",
            Unit::KilogramIncludingInnerPackaging => "kilogram, including inner packaging",
            Unit::Kilosegment => "kilosegment",
            Unit::Kilojoule => "kilojoule",
            Unit::KilogramPerMetre => "kilogram per metre",
            Unit::LacticDryMaterialPercentage => "lactic dry material percentage",
            Unit::Kilolux => "kilolux",
            Unit::KilogramMethylamine => "kilogram of methylamine",
            Unit::KilometrePerHour => "kilometre per hour",
            Unit::SquareKilometre => "square kilometre",
            Unit::KilogramPerCubicMetre => "kilogram per cubic metre",
            Unit::Kilometre => "kilometre",
            Unit::KilogramNitrogen => "kilogram of nitrogen",
            Unit::KilonewtonPerSquareMetre => "kilonewton per square metre",
            Unit::KilogramNamedSubstance => "kilogram named substance",
            Unit::Knot => "knot",
            Unit::MilliequivalenceCausticPotashPerGramProduct => "milliequivalence caustic potash per gram of product",
            Unit::Kilopascal => "kilopascal",
            Unit::KilogramPotassiumHydroxideCausticPotash => "kilogram of potassium hydroxide (caustic potash)",
            Unit::KilogramPotassiumOxide => "kilogram of potassium oxide",
            Unit::KilogramPhosphorusPentoxidePhosphoricAnhydride => "kilogram of phosphorus pentoxide (phosphoric anhydride)",
            Unit::Kiloroentgen => "kiloroentgen",
            Unit::KilogramSubstance90Dry => "kilogram of substance 90 % dry",
            Unit::KilogramSodiumHydroxideCausticSoda => "kilogram of sodium hydroxide (caustic soda)",
            Unit::Kit => "kit",
            Unit::Kilotonne => "kilotonne",
            Unit::KilogramUranium => "kilogram of uranium",
            Unit::KilovoltAmpere => "kilovolt - ampere",
            Unit::Kilovar => "kilovar",
            Unit::Kilovolt => "kilovolt",
            Unit::KilogramPerMillimetre => "kilogram per millimetre",
            Unit::KilowattHour => "kilowatt hour",
            Unit::KilowattHourPerNormalizedCubicMetre => "Kilowatt hour per normalized cubic metre",
            Unit::KilogramTungstenTrioxide => "kilogram of tungsten trioxide",
            Unit::KilowattHourPerStandardCubicMetre => "Kilowatt hour per standard cubic metre",
            Unit::Kilowatt => "kilowatt",
            Unit::KilowattYear => "kilowatt year",
            Unit::MillilitrePerKilogram => "millilitre per kilogram",
            Unit::QuartUsLiquidPerMinute => "quart (US liquid) per minute",
            Unit::QuartUsLiquidPerSecond => "quart (US liquid) per second",
            Unit::MetrePerSecondKelvin => "metre per second kelvin",
            Unit::MetrePerSecondBar => "metre per second bar",
            Unit::SquareMetreHourDegreeCelsiusPerKilocalorieInternationalTable => "square metre hour degree Celsius per kilocalorie (international table)",
            Unit::MillipascalSecondPerKelvin => "millipascal second per kelvin",
            Unit::MillipascalSecondPerBar => "millipascal second per bar",
            Unit::MilligramPerCubicMetreKelvin => "milligram per cubic metre kelvin",
            Unit::MilligramPerCubicMetreBar => "milligram per cubic metre bar",
            Unit::MillilitrePerLitre => "millilitre per litre",
            Unit::LitrePerMinute => "litre per minute",
            Unit::ReciprocalCubicMillimetre => "reciprocal cubic millimetre",
            Unit::CubicMillimetrePerCubicMetre => "cubic millimetre per cubic metre",
            Unit::MolePerHour => "mole per hour",
            Unit::MolePerKilogramKelvin => "mole per kilogram kelvin",
            Unit::MolePerKilogramBar => "mole per kilogram bar",
            Unit::MolePerLitreKelvin => "mole per litre kelvin",
            Unit::MolePerLitreBar => "mole per litre bar",
            Unit::MolePerCubicMetreKelvin => "mole per cubic metre kelvin",
            Unit::MolePerCubicMetreBar => "mole per cubic metre bar",
            Unit::MolePerMinute => "mole per minute",
            Unit::MilliroentgenAequivalentMen => "milliroentgen aequivalent men",
            Unit::NanogramPerKilogram => "nanogram per kilogram",
            Unit::OunceAvoirdupoisPerDay => "ounce (avoirdupois) per day",
            Unit::OunceAvoirdupoisPerHour => "ounce (avoirdupois) per hour",
            Unit::OunceAvoirdupoisPerMinute => "ounce (avoirdupois) per minute",
            Unit::OunceAvoirdupoisPerSecond => "ounce (avoirdupois) per second",
            Unit::OunceAvoirdupoisPerGallonUk => "ounce (avoirdupois) per gallon (UK)",
            Unit::OunceAvoirdupoisPerGallonUs => "ounce (avoirdupois) per gallon (US)",
            Unit::OunceAvoirdupoisPerCubicInch => "ounce (avoirdupois) per cubic inch",
            Unit::OunceAvoirdupoisForce => "ounce (avoirdupois)-force",
            Unit::OunceAvoirdupoisForceInch => "ounce (avoirdupois)-force inch",
            Unit::PicosiemensPerMetre => "picosiemens per metre",
            Unit::PeckUk => "peck (UK)",
            Unit::PeckUkPerDay => "peck (UK) per day",
            Unit::PeckUkPerHour => "peck (UK) per hour",
            Unit::PeckUkPerMinute => "peck (UK) per minute",
            Unit::PeckUkPerSecond => "peck (UK) per second",
            Unit::PeckUsDryPerDay => "peck (US dry) per day",
            Unit::PeckUsDryPerHour => "peck (US dry) per hour",
            Unit::PeckUsDryPerMinute => "peck (US dry) per minute",
            Unit::PeckUsDryPerSecond => "peck (US dry) per second",
            Unit::PsiPerPsi => "psi per psi",
            Unit::PintUkPerDay => "pint (UK) per day",
            Unit::PintUkPerHour => "pint (UK) per hour",
            Unit::PintUkPerMinute => "pint (UK) per minute",
            Unit::PintUkPerSecond => "pint (UK) per second",
            Unit::PintUsLiquidPerDay => "pint (US liquid) per day",
            Unit::PintUsLiquidPerHour => "pint (US liquid) per hour",
            Unit::PintUsLiquidPerMinute => "pint (US liquid) per minute",
            Unit::PintUsLiquidPerSecond => "pint (US liquid) per second",
            Unit::SlugPerDay => "slug per day",
            Unit::SlugPerFootSecond => "slug per foot second",
            Unit::SlugPerCubicFoot => "slug per cubic foot",
            Unit::SlugPerHour => "slug per hour",
            Unit::SlugPerMinute => "slug per minute",
            Unit::SlugPerSecond => "slug per second",
            Unit::TonnePerKelvin => "tonne per kelvin",
            Unit::TonnePerBar => "tonne per bar",
            Unit::TonnePerDay => "tonne per day",
            Unit::TonnePerDayKelvin => "tonne per day kelvin",
            Unit::TonnePerDayBar => "tonne per day bar",
            Unit::TonnePerHourKelvin => "tonne per hour kelvin",
            Unit::TonnePerHourBar => "tonne per hour bar",
            Unit::TonnePerCubicMetreKelvin => "tonne per cubic metre kelvin",
            Unit::TonnePerCubicMetreBar => "tonne per cubic metre bar",
            Unit::TonnePerMinute => "tonne per minute",
            Unit::TonnePerMinuteKelvin => "tonne per minute kelvin",
            Unit::TonnePerMinuteBar => "tonne per minute bar",
            Unit::TonnePerSecond => "tonne per second",
            Unit::TonnePerSecondKelvin => "tonne per second kelvin",
            Unit::TonnePerSecondBar => "tonne per second bar",
            Unit::TonUkShipping => "ton (UK shipping)",
            Unit::TonLongPerDay => "ton long per day",
            Unit::TonUsShipping => "ton (US shipping)",
            Unit::TonShortPerDegreeFahrenheit => "ton short per degree Fahrenheit",
            Unit::TonShortPerDay => "ton short per day",
            Unit::TonShortPerHourDegreeFahrenheit => "ton short per hour degree Fahrenheit",
            Unit::TonShortPerHourPsi => "ton short per hour psi",
            Unit::TonShortPerPsi => "ton short per psi",
            Unit::TonUkLongPerCubicYard => "ton (UK long) per cubic yard",
            Unit::TonUsShortPerCubicYard => "ton (US short) per cubic yard",
            Unit::TonForceUsShort => "ton-force (US short)",
            Unit::CommonYear => "common year",
            Unit::SiderealYear => "sidereal year",
            Unit::YardPerDegreeFahrenheit => "yard per degree Fahrenheit",
            Unit::YardPerPsi => "yard per psi",
            Unit::PoundPerCubicInch => "pound per cubic inch",
            Unit::LactoseExcessPercentage => "lactose excess percentage",
            Unit::Pound => "pound",
            Unit::TroyPoundUs => "troy pound (US)",
            Unit::LitrePerDay => "litre per day",
            Unit::Leaf => "leaf",
            Unit::LinearFoot => "linear foot",
            Unit::LabourHour => "labour hour",
            Unit::Link => "link",
            Unit::LinearMetre => "linear metre",
            Unit::Length => "length",
            Unit::LotUnitProcurement => "lot  [unit of procurement]",
            Unit::LiquidPound => "liquid pound",
            Unit::LitrePureAlcohol => "litre of pure alcohol",
            Unit::Layer => "layer",
            Unit::LumpSum => "lump sum",
            Unit::TonUkOrLongTonUs => "ton (UK) or long ton (US)",
            Unit::Litre => "litre",
            Unit::MetricTonLubricatingOil => "metric ton, lubricating oil",
            Unit::Lumen => "lumen",
            Unit::Lux => "lux",
            Unit::LinearYard => "linear yard",
            Unit::MilligramPerLitre => "milligram per litre",
            Unit::ReciprocalCubicYard => "reciprocal cubic yard",
            Unit::CubicYardPerDegreeFahrenheit => "cubic yard per degree Fahrenheit",
            Unit::CubicYardPerDay => "cubic yard per day",
            Unit::CubicYardPerHour => "cubic yard per hour",
            Unit::CubicYardPerPsi => "cubic yard per psi",
            Unit::CubicYardPerMinute => "cubic yard per minute",
            Unit::CubicYardPerSecond => "cubic yard per second",
            Unit::KilohertzMetre => "kilohertz metre",
            Unit::GigahertzMetre => "gigahertz metre",
            Unit::Beaufort => "Beaufort",
            Unit::ReciprocalMegakelvinOrMegakelvinToPowerMinusOne => "reciprocal megakelvin or megakelvin to the power minus one",
            Unit::ReciprocalKilovoltAmpereReciprocalHour => "reciprocal kilovolt - ampere reciprocal hour",
            Unit::MillilitrePerSquareCentimetreMinute => "millilitre per square centimetre minute",
            Unit::NewtonPerCentimetre => "newton per centimetre",
            Unit::OhmKilometre => "ohm kilometre",
            Unit::PercentPerDegreeCelsius => "percent per degree Celsius",
            Unit::GigaohmPerMetre => "gigaohm per metre",
            Unit::MegahertzMetre => "megahertz metre",
            Unit::KilogramPerKilogram => "kilogram per kilogram",
            Unit::ReciprocalVoltAmpereReciprocalSecond => "reciprocal volt - ampere reciprocal second",
            Unit::KilogramPerKilometre => "kilogram per kilometre",
            Unit::PascalSecondPerLitre => "pascal second per litre",
            Unit::MillimolePerLitre => "millimole per litre",
            Unit::NewtonMetrePerSquareMetre => "newton metre per square metre",
            Unit::MillivoltAmpere => "millivolt - ampere",
            Unit::_30DayMonth => "30-day month",
            Unit::Actual360 => "actual/360",
            Unit::KilometrePerSecondSquared => "kilometre per second squared",
            Unit::CentimetrePerSecondSquared => "centimetre per second squared",
            Unit::MonetaryValue => "monetary value",
            Unit::YardPerSecondSquared => "yard per second squared",
            Unit::MillimetrePerSecondSquared => "millimetre per second squared",
            Unit::MileStatuteMilePerSecondSquared => "mile (statute mile) per second squared",
            Unit::Mil => "mil",
            Unit::Revolution => "revolution",
            Unit::DegreeUnitAnglePerSecondSquared => "degree [unit of angle] per second squared",
            Unit::RevolutionPerMinute => "revolution per minute ",
            Unit::CircularMil => "circular mil ",
            Unit::SquareMileBasedOnUSSurveyFoot => "square mile (based on U.S. survey foot) ",
            Unit::ChainBasedOnUSSurveyFoot => "chain (based on U.S. survey foot)",
            Unit::Microcurie => "microcurie",
            Unit::Furlong => "furlong",
            Unit::FootUSSurvey => "foot (U.S. survey) ",
            Unit::MileBasedOnUSSurveyFoot => "mile (based on U.S. survey foot) ",
            Unit::MetrePerPascal => "metre per pascal",
            Unit::MetrePerRadiant => "metre per radiant",
            Unit::Shake => "shake",
            Unit::MilePerMinute => "mile per minute ",
            Unit::MilePerSecond => "mile per second ",
            Unit::MetrePerSecondPascal => "metre per second pascal",
            Unit::MetrePerHour => "metre per hour",
            Unit::InchPerYear => "inch per year",
            Unit::KilometrePerSecond => "kilometre per second ",
            Unit::InchPerMinute => "inch per minute",
            Unit::YardPerSecond => "yard per second",
            Unit::YardPerMinute => "yard per minute",
            Unit::YardPerHour => "yard per hour",
            Unit::AcreFootBasedOnUSSurveyFoot => "acre-foot (based on U.S. survey foot)",
            Unit::Cord128Ft3 => "cord (128 ft3)",
            Unit::CubicMileUkStatute => "cubic mile (UK statute)",
            Unit::MicroInch => "micro-inch",
            Unit::TonRegister => "ton, register ",
            Unit::CubicMetrePerPascal => "cubic metre per pascal",
            Unit::Bel => "bel",
            Unit::KilogramPerCubicMetrePascal => "kilogram per cubic metre pascal",
            Unit::KilogramPerPascal => "kilogram per pascal",
            Unit::KilopoundForce => "kilopound-force",
            Unit::Poundal => "poundal",
            Unit::KilogramMetrePerSecondSquared => "kilogram metre per second squared",
            Unit::Pond => "pond",
            Unit::SquareFootPerHour => "square foot per hour ",
            Unit::StokesPerPascal => "stokes per pascal",
            Unit::SquareCentimetrePerSecond => "square centimetre per second",
            Unit::SquareMetrePerSecondPascal => "square metre per second pascal",
            Unit::Denier_Dup => "denier ",
            Unit::PoundPerYard => "pound per yard ",
            Unit::TonAssay => "ton, assay",
            Unit::Pfund => "pfund",
            Unit::KilogramPerSecondPascal => "kilogram per second pascal",
            Unit::TonnePerMonth => "tonne per month",
            Unit::TonnePerYear => "tonne per year",
            Unit::MillionBtuPer1000CubicFoot => "million Btu per 1000 cubic foot",
            Unit::KilopoundPerHour => "kilopound per hour",
            Unit::PoundPerPound => "pound per pound",
            Unit::PoundForceFoot => "pound-force foot",
            Unit::NewtonMetrePerRadian => "newton metre per radian",
            Unit::KilogramMetre => "kilogram metre",
            Unit::PoundalFoot => "poundal foot",
            Unit::PoundalInch => "poundal inch",
            Unit::DyneMetre => "dyne metre",
            Unit::KilogramCentimetrePerSecond => "kilogram centimetre per second",
            Unit::GramCentimetrePerSecond => "gram centimetre per second",
            Unit::MegavoltAmpereReactiveHour => "megavolt ampere reactive hour",
            Unit::Megalitre => "megalitre",
            Unit::Megametre => "megametre",
            Unit::Megavar => "megavar",
            Unit::Megawatt => "megawatt",
            Unit::ThousandStandardBrickEquivalent => "thousand standard brick equivalent",
            Unit::ThousandBoardFoot => "thousand board foot",
            Unit::Millibar => "millibar",
            Unit::Microgram => "microgram",
            Unit::Millicurie => "millicurie",
            Unit::AirDryMetricTon => "air dry metric ton",
            Unit::Milligram => "milligram",
            Unit::Megahertz => "megahertz",
            Unit::SquareMileStatuteMile => "square mile (statute mile)",
            Unit::Thousand => "thousand",
            Unit::MinuteUnitTime => "minute [unit of time]",
            Unit::Million => "million",
            Unit::MillionInternationalUnit => "million international unit",
            Unit::SquareMetreDay => "Square Metre Day",
            Unit::SquareMetreMonth => "Square Metre Month",
            Unit::SquareMetreWeek => "Square Metre Week",
            Unit::Milliard => "milliard",
            Unit::Millilitre => "millilitre",
            Unit::SquareMillimetre => "square millimetre",
            Unit::CubicMillimetre => "cubic millimetre",
            Unit::Millimetre => "millimetre",
            Unit::KilogramDryWeight => "kilogram, dry weight",
            Unit::MegaJoulePerNormalisedCubicMetre => "Mega Joule per Normalised cubic Metre",
            Unit::Month => "month",
            Unit::Megapascal => "megapascal",
            Unit::CubicMetreDay => "Cubic Metre Day",
            Unit::CubicMetrePerHour => "cubic metre per hour",
            Unit::CubicMetreMonth => "Cubic Metre Month",
            Unit::CubicMetrePerSecond => "cubic metre per second",
            Unit::CubicMetreWeek => "Cubic Metre Week",
            Unit::MetreDay => "Metre Day",
            Unit::MetreMonth => "Metre Month",
            Unit::MetreWeek => "Metre Week",
            Unit::MetrePerSecondSquared => "metre per second squared",
            Unit::SquareMetre => "square metre",
            Unit::CubicMetre => "cubic metre",
            Unit::Metre => "metre",
            Unit::MetrePerSecond => "metre per second",
            Unit::Milihertz => "milihertz",
            Unit::MegavoltAmpere => "megavolt - ampere",
            Unit::MegawattHour1000KwH => "megawatt hour (1000 kW.h)",
            Unit::PenCalorie => "pen calorie",
            Unit::PoundFootPerSecond => "pound foot per second",
            Unit::PoundInchPerSecond => "pound inch per second",
            Unit::Pferdestaerke => "Pferdestaerke",
            Unit::CentimetreMercury0Oc => "centimetre of mercury (0 ºC)",
            Unit::CentimetreWater4Oc => "centimetre of water (4 ºC)",
            Unit::FootWater392Of => "foot of water (39.2 ºF)",
            Unit::InchMercury32Of => "inch of mercury (32 ºF)",
            Unit::InchMercury60Of => "inch of mercury (60 ºF)",
            Unit::InchWater392Of => "inch of water (39.2 ºF)",
            Unit::InchWater60Of => "inch of water (60 ºF)",
            Unit::KipPerSquareInch => "kip per square inch",
            Unit::PoundalPerSquareFoot => "poundal per square foot ",
            Unit::OunceAvoirdupoisPerSquareInch => "ounce (avoirdupois) per square inch ",
            Unit::ConventionalMetreWater => "conventional metre of water",
            Unit::GramPerSquareMillimetre => "gram per square millimetre",
            Unit::PoundPerSquareYard => "pound per square yard",
            Unit::PoundalPerSquareInch => "poundal per square inch",
            Unit::FootToFourthPower => "foot to the fourth power ",
            Unit::CubicDecimetrePerKilogram => "cubic decimetre per kilogram",
            Unit::CubicFootPerPound => "cubic foot per pound",
            Unit::PrintPoint => "print point",
            Unit::CubicInchPerPound => "cubic inch per pound",
            Unit::KilonewtonPerMetre => "kilonewton per metre",
            Unit::PoundalPerInch => "poundal per inch",
            Unit::PoundForcePerYard => "pound-force per yard",
            Unit::PoundalSecondPerSquareFoot => "poundal second per square foot ",
            Unit::PoisePerPascal => "poise per pascal",
            Unit::NewtonSecondPerSquareMetre => "newton second per square metre",
            Unit::KilogramPerMetreSecond => "kilogram per metre second",
            Unit::KilogramPerMetreMinute => "kilogram per metre minute",
            Unit::KilogramPerMetreDay => "kilogram per metre day",
            Unit::KilogramPerMetreHour => "kilogram per metre hour",
            Unit::GramPerCentimetreSecond => "gram per centimetre second",
            Unit::PoundalSecondPerSquareInch => "poundal second per square inch",
            Unit::PoundPerFootMinute => "pound per foot minute",
            Unit::PoundPerFootDay => "pound per foot day",
            Unit::CubicMetrePerSecondPascal => "cubic metre per second pascal",
            Unit::FootPoundal => "foot poundal",
            Unit::InchPoundal => "inch poundal",
            Unit::WattPerSquareCentimetre => "watt per square centimetre ",
            Unit::WattPerSquareInch => "watt per square inch ",
            Unit::BritishThermalUnitInternationalTablePerSquareFootHour => "British thermal unit (international table) per square foot hour",
            Unit::BritishThermalUnitThermochemicalPerSquareFootHour => "British thermal unit (thermochemical) per square foot hour",
            Unit::BritishThermalUnitThermochemicalPerSquareFootMinute => "British thermal unit (thermochemical) per square foot minute",
            Unit::BritishThermalUnitInternationalTablePerSquareFootSecond => "British thermal unit (international table) per square foot second",
            Unit::BritishThermalUnitThermochemicalPerSquareFootSecond => "British thermal unit (thermochemical) per square foot second",
            Unit::BritishThermalUnitInternationalTablePerSquareInchSecond => "British thermal unit (international table) per square inch second",
            Unit::CalorieThermochemicalPerSquareCentimetreMinute => "calorie (thermochemical) per square centimetre minute",
            Unit::CalorieThermochemicalPerSquareCentimetreSecond => "calorie (thermochemical) per square centimetre second",
            Unit::BritishThermalUnitInternationalTablePerCubicFoot => "British thermal unit (international table) per cubic foot ",
            Unit::BritishThermalUnitThermochemicalPerCubicFoot => "British thermal unit (thermochemical) per cubic foot",
            Unit::BritishThermalUnitInternationalTablePerDegreeFahrenheit => "British thermal unit (international table) per degree Fahrenheit",
            Unit::BritishThermalUnitThermochemicalPerDegreeFahrenheit => "British thermal unit (thermochemical) per degree Fahrenheit",
            Unit::BritishThermalUnitInternationalTablePerDegreeRankine => "British thermal unit (international table) per degree Rankine",
            Unit::BritishThermalUnitThermochemicalPerDegreeRankine => "British thermal unit (thermochemical) per degree Rankine",
            Unit::BritishThermalUnitThermochemicalPerPoundDegreeRankine => "British thermal unit (thermochemical) per pound degree Rankine",
            Unit::KilocalorieInternationalTablePerGramKelvin => "kilocalorie (international table) per gram kelvin",
            Unit::BritishThermalUnit39Of => "British thermal unit (39 ºF) ",
            Unit::BritishThermalUnit59Of => "British thermal unit (59 ºF)",
            Unit::BritishThermalUnit60Of => "British thermal unit (60 ºF) ",
            Unit::Calorie20Oc => "calorie (20 ºC) ",
            Unit::Quad1015Btuit => "quad (1015 BtuIT)",
            Unit::ThermEc => "therm (EC)",
            Unit::ThermUS => "therm (U.S.)",
            Unit::BritishThermalUnitThermochemicalPerPound => "British thermal unit (thermochemical) per pound",
            Unit::BritishThermalUnitInternationalTablePerHourSquareFootDegreeFahrenheit => "British thermal unit (international table) per hour square foot degree Fahrenheit",
            Unit::BritishThermalUnitThermochemicalPerHourSquareFootDegreeFahrenheit => "British thermal unit (thermochemical) per hour square foot degree Fahrenheit",
            Unit::BritishThermalUnitInternationalTablePerSecondSquareFootDegreeFahrenheit => "British thermal unit (international table) per second square foot degree Fahrenheit",
            Unit::BritishThermalUnitThermochemicalPerSecondSquareFootDegreeFahrenheit => "British thermal unit (thermochemical) per second square foot degree Fahrenheit",
            Unit::KilowattPerSquareMetreKelvin => "kilowatt per square metre kelvin",
            Unit::KelvinPerPascal => "kelvin per pascal",
            Unit::WattPerMetreDegreeCelsius => "watt per metre degree Celsius",
            Unit::KilowattPerMetreKelvin => "kilowatt per metre kelvin",
            Unit::KilowattPerMetreDegreeCelsius => "kilowatt per metre degree Celsius",
            Unit::MetrePerDegreeCelciusMetre => "metre per degree Celcius metre",
            Unit::DegreeFahrenheitHourPerBritishThermalUnitInternationalTable => "degree Fahrenheit hour per British thermal unit (international table)",
            Unit::DegreeFahrenheitHourPerBritishThermalUnitThermochemical => "degree Fahrenheit hour per British thermal unit (thermochemical)",
            Unit::DegreeFahrenheitSecondPerBritishThermalUnitInternationalTable => "degree Fahrenheit second per British thermal unit (international table)",
            Unit::DegreeFahrenheitSecondPerBritishThermalUnitThermochemical => "degree Fahrenheit second per British thermal unit (thermochemical)",
            Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitInternationalTableInch => "degree Fahrenheit hour square foot per British thermal unit (international table) inch",
            Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitThermochemicalInch => "degree Fahrenheit hour square foot per British thermal unit (thermochemical) inch",
            Unit::Kilofarad => "kilofarad",
            Unit::ReciprocalJoule => "reciprocal joule",
            Unit::Picosiemens => "picosiemens",
            Unit::AmperePerPascal => "ampere per pascal",
            Unit::Franklin => "franklin",
            Unit::AmpereMinute => "ampere minute",
            Unit::Biot => "biot",
            Unit::Gilbert => "gilbert",
            Unit::VoltPerPascal => "volt per pascal",
            Unit::Picovolt => "picovolt",
            Unit::MilligramPerKilogram => "milligram per kilogram",
            Unit::NumberArticles => "number of articles",
            Unit::NumberCells => "number of cells",
            Unit::Newton => "newton",
            Unit::Message => "message",
            Unit::Nil => "nil",
            Unit::NumberInternationalUnits => "number of international units",
            Unit::Load => "load",
            Unit::NormalisedCubicMetre => "Normalised cubic metre",
            Unit::NauticalMile => "nautical mile",
            Unit::NumberPacks => "number of packs",
            Unit::NumberParts => "number of parts",
            Unit::NetTon => "net ton",
            Unit::NephelometricTurbidityUnit => "Nephelometric turbidity unit",
            Unit::NewtonMetre => "newton metre",
            Unit::PartPerThousand => "part per thousand",
            Unit::Panel => "panel",
            Unit::OzoneDepletionEquivalent => "ozone depletion equivalent",
            Unit::OdsGrams => "ODS Grams",
            Unit::OdsKilograms => "ODS Kilograms",
            Unit::OdsMilligrams => "ODS Milligrams",
            Unit::Ohm => "ohm",
            Unit::OuncePerSquareYard => "ounce per square yard",
            Unit::OunceAvoirdupois => "ounce (avoirdupois)",
            Unit::OscillationsPerMinute => "oscillations per minute",
            Unit::OvertimeHour => "overtime hour",
            Unit::FluidOunceUs => "fluid ounce (US)",
            Unit::FluidOunceUk => "fluid ounce (UK)",
            Unit::Percent => "percent",
            Unit::CoulombPerMetre => "coulomb per metre",
            Unit::Kiloweber => "kiloweber",
            Unit::Gamma => "gamma",
            Unit::Kilotesla => "kilotesla",
            Unit::JoulePerSecond => "joule per second",
            Unit::JoulePerMinute => "joule per minute",
            Unit::JoulePerHour => "joule per hour",
            Unit::JoulePerDay => "joule per day",
            Unit::KilojoulePerSecond => "kilojoule per second",
            Unit::KilojoulePerMinute => "kilojoule per minute",
            Unit::PoundPerFoot => "pound per foot",
            Unit::KilojoulePerHour => "kilojoule per hour",
            Unit::KilojoulePerDay => "kilojoule per day",
            Unit::Nanoohm => "nanoohm",
            Unit::OhmCircularMilPerFoot => "ohm circular-mil per foot ",
            Unit::Kilohenry => "kilohenry",
            Unit::LumenPerSquareFoot => "lumen per square foot ",
            Unit::Phot => "phot",
            Unit::Footcandle => "footcandle",
            Unit::CandelaPerSquareInch => "candela per square inch ",
            Unit::Footlambert => "footlambert",
            Unit::Lambert => "lambert",
            Unit::Stilb => "stilb",
            Unit::CandelaPerSquareFoot => "candela per square foot",
            Unit::Kilocandela => "kilocandela",
            Unit::Millicandela => "millicandela",
            Unit::HefnerKerze => "Hefner-Kerze",
            Unit::InternationalCandle => "international candle ",
            Unit::BritishThermalUnitInternationalTablePerSquareFoot => "British thermal unit (international table) per square foot",
            Unit::BritishThermalUnitThermochemicalPerSquareFoot => "British thermal unit (thermochemical) per square foot",
            Unit::CalorieThermochemicalPerSquareCentimetre => "calorie (thermochemical) per square centimetre ",
            Unit::Langley => "langley",
            Unit::DecadeLogarithmic => "decade (logarithmic)",
            Unit::PascalSquaredSecond => "pascal squared second",
            Unit::BelPerMetre => "bel per metre",
            Unit::PoundMole => "pound mole",
            Unit::PoundMolePerSecond => "pound mole per second",
            Unit::PoundMolePerMinute => "pound mole per minute",
            Unit::KilomolePerKilogram => "kilomole per kilogram",
            Unit::PoundMolePerPound => "pound mole per pound",
            Unit::NewtonSquareMetrePerAmpere => "newton square metre per ampere",
            Unit::FivePack => "five pack",
            Unit::WeberMetre => "weber metre",
            Unit::MolPerKilogramPascal => "mol per kilogram pascal",
            Unit::MolPerCubicMetrePascal => "mol per cubic metre pascal",
            Unit::UnitPole => "unit pole ",
            Unit::MilligrayPerSecond => "milligray per second",
            Unit::MicrograyPerSecond => "microgray per second",
            Unit::NanograyPerSecond => "nanogray per second",
            Unit::GrayPerMinute => "gray per minute",
            Unit::MilligrayPerMinute => "milligray per minute",
            Unit::MicrograyPerMinute => "microgray per minute",
            Unit::NanograyPerMinute => "nanogray per minute",
            Unit::GrayPerHour => "gray per hour",
            Unit::MilligrayPerHour => "milligray per hour",
            Unit::MicrograyPerHour => "microgray per hour",
            Unit::NanograyPerHour => "nanogray per hour",
            Unit::SievertPerSecond => "sievert per second",
            Unit::MillisievertPerSecond => "millisievert per second",
            Unit::MicrosievertPerSecond => "microsievert per second",
            Unit::NanosievertPerSecond => "nanosievert per second",
            Unit::RemPerSecond => "rem per second",
            Unit::SievertPerHour => "sievert per hour",
            Unit::MillisievertPerHour => "millisievert per hour",
            Unit::MicrosievertPerHour => "microsievert per hour",
            Unit::NanosievertPerHour => "nanosievert per hour",
            Unit::SievertPerMinute => "sievert per minute",
            Unit::MillisievertPerMinute => "millisievert per minute",
            Unit::MicrosievertPerMinute => "microsievert per minute",
            Unit::NanosievertPerMinute => "nanosievert per minute",
            Unit::ReciprocalSquareInch => "reciprocal square inch",
            Unit::PascalSquareMetrePerKilogram => "pascal square metre per kilogram",
            Unit::MillipascalPerMetre => "millipascal per metre",
            Unit::KilopascalPerMetre => "kilopascal per metre",
            Unit::HectopascalPerMetre => "hectopascal per metre",
            Unit::StandardAtmospherePerMetre => "standard atmosphere per metre",
            Unit::TechnicalAtmospherePerMetre => "technical atmosphere per metre",
            Unit::TorrPerMetre => "torr per metre",
            Unit::PsiPerInch => "psi per inch",
            Unit::CubicMetrePerSecondSquareMetre => "cubic metre per second square metre",
            Unit::Rhe => "rhe",
            Unit::PoundForceFootPerInch => "pound-force foot per inch",
            Unit::PoundForceInchPerInch => "pound-force inch per inch ",
            Unit::Perm0Oc => "perm (0 ºC) ",
            Unit::Perm23Oc => "perm (23 ºC) ",
            Unit::BytePerSecond => "byte per second",
            Unit::KilobytePerSecond => "kilobyte per second",
            Unit::MegabytePerSecond => "megabyte per second",
            Unit::ReciprocalVolt => "reciprocal volt",
            Unit::ReciprocalRadian => "reciprocal radian",
            Unit::PascalToPowerSumStoichiometricNumbers => "pascal to the power sum of stoichiometric numbers",
            Unit::MolePerCubivMetreToPowerSumStoichiometricNumbers => "mole per cubiv metre to the power sum of stoichiometric numbers",
            Unit::Pascal => "pascal",
            Unit::Pad => "pad",
            Unit::ProofLitre => "proof litre",
            Unit::ProofGallon => "proof gallon",
            Unit::Pitch => "pitch",
            Unit::DegreePlato => "degree Plato",
            Unit::PoundPerInchLength => "pound per inch of length",
            Unit::PagePerInch => "page per inch",
            Unit::Pair => "pair",
            Unit::PoundForcePerSquareInch => "pound-force per square inch",
            Unit::DryPintUs => "dry pint (US)",
            Unit::PintUk => "pint (UK)",
            Unit::LiquidPintUs => "liquid pint (US)",
            Unit::Portion => "portion",
            Unit::JoulePerTesla => "joule per tesla",
            Unit::Erlang => "erlang",
            Unit::Octet => "octet",
            Unit::OctetPerSecond => "octet per second",
            Unit::Shannon => "shannon",
            Unit::Hartley => "hartley",
            Unit::NaturalUnitInformation => "natural unit of information",
            Unit::ShannonPerSecond => "shannon per second",
            Unit::HartleyPerSecond => "hartley per second",
            Unit::NaturalUnitInformationPerSecond => "natural unit of information per second",
            Unit::SecondPerKilogramm => "second per kilogramm",
            Unit::WattSquareMetre => "watt square metre",
            Unit::SecondPerRadianCubicMetre => "second per radian cubic metre",
            Unit::WeberToPowerMinusOne => "weber to the power minus one",
            Unit::ReciprocalInch => "reciprocal inch",
            Unit::Dioptre => "dioptre",
            Unit::OnePerOne => "one per one",
            Unit::NewtonMetrePerMetre => "newton metre per metre",
            Unit::KilogramPerSquareMetrePascalSecond => "kilogram per square metre pascal second",
            Unit::MicrogramPerHectogram => "microgram per hectogram",
            Unit::Meal => "meal",
            Unit::PhPotentialHydrogen => "pH (potential of Hydrogen)",
            Unit::KilojoulePerGram => "kilojoule per gram",
            Unit::Femtolitre => "femtolitre",
            Unit::Picolitre => "picolitre",
            Unit::Nanolitre => "nanolitre",
            Unit::MegawattsPerMinute => "megawatts per minute",
            Unit::SquareMetrePerCubicMetre => "square metre per cubic metre",
            Unit::StandardCubicMetrePerDay => "Standard cubic metre per day",
            Unit::StandardCubicMetrePerHour => "Standard cubic metre per hour",
            Unit::NormalizedCubicMetrePerDay => "Normalized cubic metre per day",
            Unit::NormalizedCubicMetrePerHour => "Normalized cubic metre per hour",
            Unit::JoulePerNormalisedCubicMetre => "Joule per normalised cubic metre",
            Unit::JoulePerStandardCubicMetre => "Joule per standard cubic metre",
            Unit::PageFacsimile => "page - facsimile",
            Unit::QuarterAYear => "quarter (of a year)",
            Unit::PageHardcopy => "page - hardcopy",
            Unit::Quire => "quire",
            Unit::DryQuartUs => "dry quart (US)",
            Unit::QuartUk => "quart (UK)",
            Unit::LiquidQuartUs => "liquid quart (US)",
            Unit::QuarterUk => "quarter (UK)",
            Unit::Pica => "pica",
            Unit::ThousandCubicMetre => "thousand cubic metre",
            Unit::RunningOrOperatingHour => "running or operating hour",
            Unit::Ream => "ream",
            Unit::Room => "room",
            Unit::PoundPerReam => "pound per ream",
            Unit::RevolutionsPerMinute => "revolutions per minute",
            Unit::RevolutionsPerSecond => "revolutions per second",
            Unit::RevenueTonMile => "revenue ton mile",
            Unit::SquareFootPerSecond => "square foot per second",
            Unit::SquareMetrePerSecond => "square metre per second",
            Unit::HalfYear6Months => "half year (6 months)",
            Unit::Score => "score",
            Unit::Scruple => "scruple",
            Unit::SecondUnitTime => "second [unit of time]",
            Unit::Set => "set",
            Unit::Segment => "segment",
            Unit::Siemens => "siemens",
            Unit::StandardCubicMetre => "Standard cubic metre",
            Unit::MileStatuteMile => "mile (statute mile)",
            Unit::Square => "square",
            Unit::SquareRoofing => "square, roofing",
            Unit::Strip => "strip",
            Unit::Stick => "stick",
            Unit::StoneUk => "stone (UK)",
            Unit::StickCigarette => "stick, cigarette",
            Unit::StandardLitre => "standard litre",
            Unit::TonUsOrShortTonUkUs => "ton (US) or short ton (UK/US)",
            Unit::Straw => "straw",
            Unit::Skein => "skein",
            Unit::Shipment => "shipment",
            Unit::Syringe => "syringe",
            Unit::TelecommunicationLineInService => "telecommunication line in service",
            Unit::ThousandPiece => "thousand piece",
            Unit::KiloampereHourThousandAmpereHour => "kiloampere hour (thousand ampere hour)",
            Unit::TotalAcidNumber => "total acid number",
            Unit::ThousandSquareInch => "thousand square inch",
            Unit::MetricTonIncludingContainer => "metric ton, including container",
            Unit::MetricTonIncludingInnerPackaging => "metric ton, including inner packaging",
            Unit::TonneKilometre => "tonne kilometre",
            Unit::KilogramImportedMeatLessOffal => "kilogram of imported meat, less offal",
            Unit::TonneMetricTon => "tonne (metric ton)",
            Unit::TenPack => "ten pack",
            Unit::TeethPerInch => "teeth per inch",
            Unit::TenPair => "ten pair",
            Unit::ThousandCubicMetrePerDay => "thousand cubic metre per day",
            Unit::TrillionEur => "trillion (EUR)",
            Unit::TenSet => "ten set",
            Unit::TenThousandSticks => "ten thousand sticks",
            Unit::Treatment => "treatment",
            Unit::Tablet => "tablet",
            Unit::TelecommunicationLineInServiceAverage => "telecommunication line in service average",
            Unit::TelecommunicationPort => "telecommunication port",
            Unit::VoltAmperePerKilogram => "volt - ampere per kilogram",
            Unit::Volt => "volt",
            Unit::PercentVolume => "percent volume",
            Unit::WetKilo => "wet kilo",
            Unit::WattPerKilogram => "watt per kilogram",
            Unit::WetPound => "wet pound",
            Unit::Cord => "cord",
            Unit::WetTon => "wet ton",
            Unit::Weber => "weber",
            Unit::Week => "week",
            Unit::WineGallon => "wine gallon",
            Unit::WattHour => "watt hour",
            Unit::WorkingMonth => "working month",
            Unit::Standard => "standard",
            Unit::Watt => "watt",
            Unit::GuntersChain => "Gunter's chain",
            Unit::SquareYard => "square yard",
            Unit::CubicYard => "cubic yard",
            Unit::Yard => "yard",
            Unit::HangingContainer => "hanging container",
            Unit::Nanomole => "nanomole",
            Unit::Page => "page",
            Unit::MutuallyDefined => "mutually defined",
            Unit::DrumSteel => "Drum, steel",
            Unit::DrumAluminium => "Drum, aluminium",
            Unit::DrumPlywood => "Drum, plywood",
            Unit::ContainerFlexible => "Container, flexible",
            Unit::DrumFibre => "Drum, fibre",
            Unit::DrumWooden => "Drum, wooden",
            Unit::BarrelWooden => "Barrel, wooden",
            Unit::JerricanSteel => "Jerrican, steel",
            Unit::JerricanPlastic => "Jerrican, plastic",
            Unit::BagSuperBulk => "Bag, super bulk",
            Unit::BagPolybag => "Bag, polybag",
            Unit::BoxSteel => "Box, steel",
            Unit::BoxAluminium => "Box, aluminium",
            Unit::BoxNaturalWood => "Box, natural wood",
            Unit::BoxPlywood => "Box, plywood",
            Unit::BoxReconstitutedWood => "Box, reconstituted wood",
            Unit::BoxFibreboard => "Box, fibreboard",
            Unit::BoxPlastic => "Box, plastic",
            Unit::BagWovenPlastic => "Bag, woven plastic",
            Unit::BagTextile => "Bag, textile ",
            Unit::BagPaper => "Bag, paper ",
            Unit::CompositePackagingPlasticReceptacle => "Composite packaging, plastic receptacle",
            Unit::CompositePackagingGlassReceptacle => "Composite packaging, glass receptacle",
            Unit::CaseCar => "Case, car",
            Unit::CaseWooden => "Case, wooden",
            Unit::PalletWooden => "Pallet, wooden",
            Unit::CrateWooden => "Crate, wooden",
            Unit::BundleWooden => "Bundle, wooden",
            Unit::IntermediateBulkContainerRigidPlastic => "Intermediate bulk container, rigid plastic",
            Unit::ReceptacleFibre => "Receptacle, fibre ",
            Unit::ReceptaclePaper => "Receptacle, paper ",
            Unit::ReceptacleWooden => "Receptacle, wooden ",
            Unit::Aerosol => "Aerosol",
            Unit::PalletModularCollars80cms60cms => "Pallet, modular, collars 80cms * 60cms ",
            Unit::PalletShrinkwrapped => "Pallet, shrinkwrapped ",
            Unit::Pallet100cms110cms => "Pallet, 100cms * 110cms ",
            Unit::Clamshell => "Clamshell",
            Unit::Cone => "Cone",
            Unit::Ball_Dup => "Ball",
            Unit::AmpouleNonProtected => "Ampoule, non-protected ",
            Unit::AmpouleProtected => "Ampoule, protected ",
            Unit::Atomizer => "Atomizer ",
            Unit::Capsule => "Capsule",
            Unit::Belt => "Belt",
            Unit::Barrel => "Barrel ",
            Unit::Bobbin => "Bobbin ",
            Unit::BottlecrateBottlerack => "Bottlecrate / bottlerack ",
            Unit::Board => "Board",
            Unit::Bundle => "Bundle ",
            Unit::BalloonNonProtected => "Balloon, non-protected ",
            Unit::Bag => "Bag",
            Unit::Bunch => "Bunch",
            Unit::Bin => "Bin",
            Unit::Bucket => "Bucket ",
            Unit::Basket => "Basket ",
            Unit::BaleCompressed => "Bale, compressed ",
            Unit::Basin => "Basin",
            Unit::BaleNonCompressed => "Bale, non-compressed ",
            Unit::BottleNonProtectedCylindrical => "Bottle, non-protected, cylindrical ",
            Unit::BalloonProtected => "Balloon, protected ",
            Unit::BottleProtectedCylindrical => "Bottle, protected cylindrical",
            Unit::Bar => "Bar",
            Unit::BottleNonProtectedBulbous => "Bottle, non-protected, bulbous ",
            Unit::Bolt => "Bolt ",
            Unit::Butt => "Butt ",
            Unit::BottleProtectedBulbous => "Bottle, protected bulbous",
            Unit::BoxForLiquids => "Box, for liquids",
            Unit::Box => "Box",
            Unit::BoardInBundleBunchTruss => "Board, in bundle/bunch/truss ",
            Unit::BarsInBundleBunchTruss => "Bars, in bundle/bunch/truss",
            Unit::CanRectangular => "Can, rectangular ",
            Unit::CrateBeer => "Crate, beer",
            Unit::Churn => "Churn",
            Unit::CanWithHandleAndSpout => "Can, with handle and spout ",
            Unit::Creel => "Creel",
            Unit::Coffer => "Coffer ",
            Unit::Cage => "Cage ",
            Unit::Chest => "Chest",
            Unit::Canister => "Canister ",
            Unit::Coffin => "Coffin ",
            Unit::Cask => "Cask ",
            Unit::Coil => "Coil ",
            Unit::Card_Dup => "Card",
            Unit::ContainerNotOtherwiseSpecifiedAsTransportEquipment => "Container, not otherwise specified as transport equipment",
            Unit::CarboyNonProtected => "Carboy, non-protected",
            Unit::CarboyProtected => "Carboy, protected",
            Unit::Cartridge => "Cartridge",
            Unit::Crate => "Crate",
            Unit::Case => "Case ",
            Unit::Carton => "Carton ",
            Unit::Cup => "Cup",
            Unit::Cover => "Cover",
            Unit::CageRoll => "Cage, roll ",
            Unit::CanCylindrical => "Can, cylindrical ",
            Unit::Cylinder => "Cylinder ",
            Unit::Canvas => "Canvas ",
            Unit::CrateMultipleLayerPlastic => "Crate, multiple layer, plastic ",
            Unit::CrateMultipleLayerWooden => "Crate, multiple layer, wooden",
            Unit::CrateMultipleLayerCardboard => "Crate, multiple layer, cardboard ",
            Unit::CageCommonwealthHandlingEquipmentPoolChep => "Cage, Commonwealth Handling Equipment Pool  (CHEP)",
            Unit::BoxCommonwealthHandlingEquipmentPoolChepEurobox => "Box, Commonwealth Handling Equipment Pool (CHEP), Eurobox",
            Unit::DrumIron => "Drum, iron ",
            Unit::DemijohnNonProtected => "Demijohn, non-protected",
            Unit::CrateBulkCardboard => "Crate, bulk, cardboard ",
            Unit::CrateBulkPlastic => "Crate, bulk, plastic ",
            Unit::CrateBulkWooden => "Crate, bulk, wooden",
            Unit::Dispenser => "Dispenser",
            Unit::DemijohnProtected => "Demijohn, protected",
            Unit::Drum => "Drum ",
            Unit::TrayOneLayerNoCoverPlastic => "Tray, one layer no cover, plastic",
            Unit::TrayOneLayerNoCoverWooden => "Tray, one layer no cover, wooden ",
            Unit::TrayOneLayerNoCoverPolystyrene => "Tray, one layer no cover, polystyrene",
            Unit::TrayOneLayerNoCoverCardboard => "Tray, one layer no cover, cardboard",
            Unit::TrayTwoLayersNoCoverPlasticTray => "Tray, two layers no cover, plastic tray",
            Unit::TrayTwoLayersNoCoverWooden => "Tray, two layers no cover, wooden",
            Unit::TrayTwoLayersNoCoverCardboard => "Tray, two layers no cover, cardboard ",
            Unit::BagPlastic => "Bag, plastic ",
            Unit::CaseWithPalletBase => "Case, with pallet base ",
            Unit::CaseWithPalletBaseWooden => "Case, with pallet base, wooden ",
            Unit::CaseWithPalletBaseCardboard => "Case, with pallet base, cardboard",
            Unit::CaseWithPalletBasePlastic => "Case, with pallet base, plastic",
            Unit::CaseWithPalletBaseMetal => "Case, with pallet base, metal",
            Unit::CaseIsothermic => "Case, isothermic ",
            Unit::Envelope => "Envelope ",
            Unit::Flexibag => "Flexibag",
            Unit::CrateFruit => "Crate, fruit ",
            Unit::CrateFramed => "Crate, framed",
            Unit::Flexitank => "Flexitank",
            Unit::Firkin => "Firkin ",
            Unit::Flask => "Flask",
            Unit::Footlocker => "Footlocker ",
            Unit::Filmpack => "Filmpack ",
            Unit::Frame => "Frame",
            Unit::Foodtainer => "Foodtainer",
            Unit::CartFlatbed => "Cart, flatbed",
            Unit::BagFlexibleContainer => "Bag, flexible container",
            Unit::BottleGas => "Bottle, gas",
            Unit::Girder => "Girder ",
            Unit::ContainerGallon => "Container, gallon",
            Unit::ReceptacleGlass => "Receptacle, glass ",
            Unit::TrayContainingHorizontallyStackedFlatItems => "Tray, containing horizontally stacked flat items",
            Unit::BagGunny => "Bag, gunny",
            Unit::GirdersInBundleBunchTruss => "Girders, in bundle/bunch/truss ",
            Unit::BasketWithHandlePlastic => "Basket, with handle, plastic ",
            Unit::BasketWithHandleWooden => "Basket, with handle, wooden",
            Unit::BasketWithHandleCardboard => "Basket, with handle, cardboard ",
            Unit::Hogshead => "Hogshead ",
            Unit::Hanger => "Hanger",
            Unit::Hamper => "Hamper ",
            Unit::PackageDisplayWooden => "Package, display, wooden ",
            Unit::PackageDisplayCardboard => "Package, display, cardboard",
            Unit::PackageDisplayPlastic => "Package, display, plastic",
            Unit::PackageDisplayMetal => "Package, display, metal",
            Unit::PackageShow => "Package, show",
            Unit::PackageFlow => "Package, flow ",
            Unit::PackagePaperWrapped => "Package, paper wrapped",
            Unit::DrumPlastic => "Drum, plastic",
            Unit::PackageCardboardWithBottleGripHoles => "Package, cardboard, with bottle grip-holes ",
            Unit::TrayRigidLiddedStackableCenTs144822002 => "Tray, rigid, lidded stackable (CEN TS 14482:2002)",
            Unit::Ingot => "Ingot",
            Unit::IngotsInBundleBunchTruss => "Ingots, in bundle/bunch/truss",
            Unit::BagJumbo => "Bag, jumbo",
            Unit::JerricanRectangular => "Jerrican, rectangular",
            Unit::Jug => "Jug",
            Unit::Jar => "Jar",
            Unit::Jutebag => "Jutebag",
            Unit::JerricanCylindrical => "Jerrican, cylindrical",
            Unit::Keg => "Keg",
            Unit::Kit_Dup => "Kit",
            Unit::Luggage => "Luggage",
            Unit::Log => "Log",
            Unit::Lot => "Lot",
            Unit::Lug => "Lug",
            Unit::Liftvan => "Liftvan",
            Unit::LogsInBundleBunchTruss => "Logs, in bundle/bunch/truss",
            Unit::CrateMetal => "Crate, metal",
            Unit::BagMultiply => "Bag, multiply",
            Unit::CrateMilk => "Crate, milk",
            Unit::ContainerMetal => "Container, metal",
            Unit::ReceptacleMetal => "Receptacle, metal ",
            Unit::SackMultiWall => "Sack, multi-wall ",
            Unit::Mat => "Mat",
            Unit::ReceptaclePlasticWrapped => "Receptacle, plastic wrapped ",
            Unit::Matchbox => "Matchbox ",
            Unit::NotAvailable => "Not available",
            Unit::UnpackedOrUnpackaged => "Unpacked or unpackaged ",
            Unit::UnpackedOrUnpackagedSingleUnit => "Unpacked or unpackaged, single unit",
            Unit::UnpackedOrUnpackagedMultipleUnits => "Unpacked or unpackaged, multiple units",
            Unit::Nest => "Nest ",
            Unit::Net => "Net",
            Unit::NetTubePlastic => "Net, tube, plastic ",
            Unit::NetTubeTextile => "Net, tube, textile ",
            Unit::TwoSidedCageOnWheelsWithFixingStrap => "Two sided cage on wheels with fixing strap",
            Unit::Trolley => "Trolley",
            Unit::OnewayPalletIso012EuroPallet => "Oneway pallet ISO 0 - 1/2 EURO Pallet",
            Unit::OnewayPalletIso111EuroPallet => "Oneway pallet ISO 1 - 1/1 EURO Pallet",
            Unit::OnewayPalletIso221EuroPallet => "Oneway pallet ISO 2 - 2/1 EURO Pallet",
            Unit::PalletWithExceptionalDimensions => "Pallet with exceptional dimensions",
            Unit::WoodenPallet40CmX80Cm => "Wooden pallet  40 cm x 80 cm",
            Unit::PlasticPalletSrs60CmX80Cm => "Plastic pallet SRS 60 cm x 80 cm",
            Unit::PlasticPalletSrs80CmX120Cm => "Plastic pallet SRS 80 cm x 120 cm",
            Unit::PalletChep40CmX60Cm => "Pallet, CHEP 40 cm x 60 cm",
            Unit::PalletChep80CmX120Cm => "Pallet, CHEP 80 cm x 120 cm",
            Unit::PalletChep100CmX120Cm => "Pallet, CHEP 100 cm x 120 cm",
            Unit::PalletAs40681993 => "Pallet, AS 4068-1993",
            Unit::PalletIsoT11 => "Pallet, ISO T11",
            Unit::PlatformUnspecifiedWeightOrDimension => "Platform, unspecified weight or dimension",
            Unit::PalletIso012EuroPallet => "Pallet ISO 0 - 1/2 EURO Pallet",
            Unit::PalletIso111EuroPallet => "Pallet ISO 1 - 1/1 EURO Pallet",
            Unit::PalletIso221EuroPallet => "Pallet ISO 2 – 2/1 EURO Pallet",
            Unit::_14EuroPallet => "1/4 EURO Pallet",
            Unit::Block => "Block",
            Unit::_18EuroPallet => "1/8 EURO Pallet",
            Unit::SyntheticPalletIso1 => "Synthetic pallet ISO 1",
            Unit::SyntheticPalletIso2 => "Synthetic pallet ISO 2",
            Unit::WholesalerPallet => "Wholesaler pallet",
            Unit::Pallet80X100Cm => "Pallet 80 X 100 cm",
            Unit::Pallet60X100Cm => "Pallet 60 X 100 cm",
            Unit::OnewayPallet => "Oneway pallet",
            Unit::Octabin => "Octabin",
            Unit::ContainerOuter => "Container, outer",
            Unit::ReturnablePallet => "Returnable pallet",
            Unit::LargeBagPalletSized => "Large bag, pallet sized",
            Unit::AWheeledPalletWithRaisedRim81X67X135 => "A wheeled pallet with raised rim (81 x 67 x 135)",
            Unit::AWheeledPalletWithRaisedRim81X72X135 => "A Wheeled pallet with raised rim (81 x 72 x 135)",
            Unit::WheeledPalletWithRaisedRim81X60X16 => "Wheeled pallet with raised rim ( 81 x 60 x 16)",
            Unit::ChepPallet60CmX80Cm => "CHEP pallet 60 cm x 80 cm ",
            Unit::Pan => "Pan",
            Unit::LprPallet60CmX80Cm => "LPR pallet 60 cm x 80 cm",
            Unit::LprPallet80CmX120Cm => "LPR pallet 80 cm x 120 cm",
            Unit::Packet => "Packet ",
            Unit::PalletBoxCombinedOpenEndedBoxAndPallet => "Pallet, box Combined open-ended box and pallet",
            Unit::Parcel => "Parcel ",
            Unit::PalletModularCollars80cms100cms => "Pallet, modular, collars 80cms * 100cms ",
            Unit::PalletModularCollars80cms120cms => "Pallet, modular, collars 80cms * 120cms ",
            Unit::Pen => "Pen ",
            Unit::Plate => "Plate",
            Unit::Pitcher => "Pitcher",
            Unit::Pipe => "Pipe ",
            Unit::Punnet => "Punnet",
            Unit::Package => "Package",
            Unit::Pail => "Pail ",
            Unit::Plank => "Plank",
            Unit::Pouch => "Pouch",
            Unit::Piece_Dup => "Piece",
            Unit::ReceptaclePlastic => "Receptacle, plastic ",
            Unit::Pot => "Pot",
            Unit::Tray => "Tray ",
            Unit::PipesInBundleBunchTruss => "Pipes, in bundle/bunch/truss ",
            Unit::Pallet => "Pallet ",
            Unit::PlatesInBundleBunchTruss => "Plates, in bundle/bunch/truss",
            Unit::PlanksInBundleBunchTruss => "Planks, in bundle/bunch/truss",
            Unit::DrumSteelNonRemovableHead => "Drum, steel, non-removable head",
            Unit::DrumSteelRemovableHead => "Drum, steel, removable head",
            Unit::DrumAluminiumNonRemovableHead => "Drum, aluminium, non-removable head",
            Unit::DrumAluminiumRemovableHead => "Drum, aluminium, removable head",
            Unit::DrumPlasticNonRemovableHead => "Drum, plastic, non-removable head",
            Unit::DrumPlasticRemovableHead => "Drum, plastic, removable head",
            Unit::BarrelWoodenBungType => "Barrel, wooden, bung type",
            Unit::BarrelWoodenRemovableHead => "Barrel, wooden, removable head ",
            Unit::JerricanSteelNonRemovableHead => "Jerrican, steel, non-removable head",
            Unit::JerricanSteelRemovableHead => "Jerrican, steel, removable head",
            Unit::JerricanPlasticNonRemovableHead => "Jerrican, plastic, non-removable head",
            Unit::JerricanPlasticRemovableHead => "Jerrican, plastic, removable head",
            Unit::BoxWoodenNaturalWoodOrdinary => "Box, wooden, natural wood, ordinary",
            Unit::BoxWoodenNaturalWoodWithSiftProofWalls => "Box, wooden, natural wood, with sift proof walls",
            Unit::BoxPlasticExpanded => "Box, plastic, expanded ",
            Unit::BoxPlasticSolid => "Box, plastic, solid",
            Unit::Rod => "Rod",
            Unit::Ring => "Ring ",
            Unit::RackClothingHanger => "Rack, clothing hanger",
            Unit::Rack => "Rack ",
            Unit::Reel => "Reel ",
            Unit::Roll => "Roll ",
            Unit::Rednet => "Rednet ",
            Unit::RodsInBundleBunchTruss => "Rods, in bundle/bunch/truss",
            Unit::Sack => "Sack ",
            Unit::Slab => "Slab",
            Unit::CrateShallow => "Crate, shallow ",
            Unit::Spindle => "Spindle",
            Unit::SeaChest => "Sea-chest",
            Unit::Sachet => "Sachet ",
            Unit::Skid => "Skid ",
            Unit::CaseSkeleton => "Case, skeleton ",
            Unit::Slipsheet => "Slipsheet ",
            Unit::Sheetmetal => "Sheetmetal ",
            Unit::Spool => "Spool ",
            Unit::SheetPlasticWrapping => "Sheet, plastic wrapping",
            Unit::CaseSteel => "Case, steel",
            Unit::Sheet => "Sheet",
            Unit::Suitcase => "Suitcase ",
            Unit::EnvelopeSteel => "Envelope, steel",
            Unit::Shrinkwrapped => "Shrinkwrapped  ",
            Unit::Set_Dup => "Set",
            Unit::Sleeve => "Sleeve",
            Unit::SheetsInBundleBunchTruss => "Sheets, in bundle/bunch/truss",
            Unit::Tablet_Dup => "Tablet",
            Unit::Tub => "Tub",
            Unit::TeaChest => "Tea-chest",
            Unit::TubeCollapsible => "Tube, collapsible",
            Unit::Tyre_Dup => "Tyre",
            Unit::TankContainerGeneric => "Tank container, generic",
            Unit::Tierce => "Tierce",
            Unit::TankRectangular => "Tank, rectangular",
            Unit::TubWithLid => "Tub, with lid",
            Unit::Tin => "Tin",
            Unit::Tun => "Tun",
            Unit::Trunk => "Trunk",
            Unit::Truss => "Truss",
            Unit::BagTote => "Bag, tote",
            Unit::Tube => "Tube ",
            Unit::TubeWithNozzle => "Tube, with nozzle ",
            Unit::PalletTriwall => "Pallet, triwall",
            Unit::TankCylindrical => "Tank, cylindrical",
            Unit::TubesInBundleBunchTruss => "Tubes, in bundle/bunch/truss ",
            Unit::Uncaged => "Uncaged ",
            Unit::Unit => "Unit",
            Unit::Vat => "Vat",
            Unit::BulkGasAt1031MbarAnd15C => "Bulk, gas (at 1031 mbar and 15°C)",
            Unit::Vial => "Vial ",
            Unit::Vanpack => "Vanpack ",
            Unit::BulkLiquid => "Bulk, liquid ",
            Unit::Vehicle => "Vehicle",
            Unit::BulkSolidLargeParticlesNodules => "Bulk, solid, large particles (“nodules”) ",
            Unit::VacuumPacked => "Vacuum-packed",
            Unit::BulkLiquefiedGasAtAbnormalTemperaturePressure => "Bulk, liquefied gas (at abnormal temperature/pressure) ",
            Unit::BulkSolidGranularParticlesGrains => "Bulk, solid, granular particles (“grains”) ",
            Unit::BulkScrapMetal => "Bulk, scrap metal",
            Unit::BulkSolidFineParticlesPowders => "Bulk, solid, fine particles (“powders”)",
            Unit::IntermediateBulkContainer => "Intermediate bulk container",
            Unit::Wickerbottle => "Wickerbottle ",
            Unit::IntermediateBulkContainerSteel => "Intermediate bulk container, steel ",
            Unit::IntermediateBulkContainerAluminium => "Intermediate bulk container, aluminium ",
            Unit::IntermediateBulkContainerMetal => "Intermediate bulk container, metal ",
            Unit::IntermediateBulkContainerSteelPressurised10Kpa => "Intermediate bulk container, steel, pressurised > 10 kpa",
            Unit::IntermediateBulkContainerAluminiumPressurised10Kpa => "Intermediate bulk container, aluminium, pressurised > 10 kpa",
            Unit::IntermediateBulkContainerMetalPressure10Kpa => "Intermediate bulk container, metal, pressure 10 kpa ",
            Unit::IntermediateBulkContainerSteelLiquid => "Intermediate bulk container, steel, liquid ",
            Unit::IntermediateBulkContainerAluminiumLiquid => "Intermediate bulk container, aluminium, liquid ",
            Unit::IntermediateBulkContainerMetalLiquid => "Intermediate bulk container, metal, liquid ",
            Unit::IntermediateBulkContainerWovenPlasticWithoutCoatLiner => "Intermediate bulk container, woven plastic, without coat/liner ",
            Unit::IntermediateBulkContainerWovenPlasticCoated => "Intermediate bulk container, woven plastic, coated ",
            Unit::IntermediateBulkContainerWovenPlasticWithLiner => "Intermediate bulk container, woven plastic, with liner ",
            Unit::IntermediateBulkContainerWovenPlasticCoatedAndLiner => "Intermediate bulk container, woven plastic, coated and liner ",
            Unit::IntermediateBulkContainerPlasticFilm => "Intermediate bulk container, plastic film",
            Unit::IntermediateBulkContainerTextileWithOutCoatLiner => "Intermediate bulk container, textile with out coat/liner ",
            Unit::IntermediateBulkContainerNaturalWoodWithInnerLiner => "Intermediate bulk container, natural wood, with inner liner",
            Unit::IntermediateBulkContainerTextileCoated => "Intermediate bulk container, textile, coated ",
            Unit::IntermediateBulkContainerTextileWithLiner => "Intermediate bulk container, textile, with liner ",
            Unit::IntermediateBulkContainerTextileCoatedAndLiner => "Intermediate bulk container, textile, coated and liner ",
            Unit::IntermediateBulkContainerPlywoodWithInnerLiner => "Intermediate bulk container, plywood, with inner liner ",
            Unit::IntermediateBulkContainerReconstitutedWoodWithInnerLiner => "Intermediate bulk container, reconstituted wood, with inner liner",
            Unit::BagWovenPlasticWithoutInnerCoatLiner => "Bag, woven plastic, without inner coat/liner ",
            Unit::BagWovenPlasticSiftProof => "Bag, woven plastic, sift proof ",
            Unit::BagWovenPlasticWaterResistant => "Bag, woven plastic, water resistant",
            Unit::BagPlasticsFilm => "Bag, plastics film ",
            Unit::BagTextileWithoutInnerCoatLiner => "Bag, textile, without inner coat/liner ",
            Unit::BagTextileSiftProof => "Bag, textile, sift proof ",
            Unit::BagTextileWaterResistant => "Bag, textile, water resistant",
            Unit::BagPaperMultiWall => "Bag, paper, multi-wall ",
            Unit::BagPaperMultiWallWaterResistant => "Bag, paper, multi-wall, water resistant",
            Unit::CompositePackagingPlasticReceptacleInSteelDrum => "Composite packaging, plastic receptacle in steel drum",
            Unit::CompositePackagingPlasticReceptacleInSteelCrateBox => "Composite packaging, plastic receptacle in steel crate box",
            Unit::CompositePackagingPlasticReceptacleInAluminiumDrum => "Composite packaging, plastic receptacle in aluminium drum",
            Unit::CompositePackagingPlasticReceptacleInAluminiumCrate => "Composite packaging, plastic receptacle in aluminium crate",
            Unit::CompositePackagingPlasticReceptacleInWoodenBox => "Composite packaging, plastic receptacle in wooden box",
            Unit::CompositePackagingPlasticReceptacleInPlywoodDrum => "Composite packaging, plastic receptacle in plywood drum",
            Unit::CompositePackagingPlasticReceptacleInPlywoodBox => "Composite packaging, plastic receptacle in plywood box ",
            Unit::CompositePackagingPlasticReceptacleInFibreDrum => "Composite packaging, plastic receptacle in fibre drum",
            Unit::CompositePackagingPlasticReceptacleInFibreboardBox => "Composite packaging, plastic receptacle in fibreboard box",
            Unit::CompositePackagingPlasticReceptacleInPlasticDrum => "Composite packaging, plastic receptacle in plastic drum",
            Unit::CompositePackagingPlasticReceptacleInSolidPlasticBox => "Composite packaging, plastic receptacle in solid plastic box",
            Unit::CompositePackagingGlassReceptacleInSteelDrum => "Composite packaging, glass receptacle in steel drum",
            Unit::CompositePackagingGlassReceptacleInSteelCrateBox => "Composite packaging, glass receptacle in steel crate box",
            Unit::CompositePackagingGlassReceptacleInAluminiumDrum => "Composite packaging, glass receptacle in aluminium drum",
            Unit::CompositePackagingGlassReceptacleInAluminiumCrate => "Composite packaging, glass receptacle in aluminium crate",
            Unit::CompositePackagingGlassReceptacleInWoodenBox => "Composite packaging, glass receptacle in wooden box",
            Unit::CompositePackagingGlassReceptacleInPlywoodDrum => "Composite packaging, glass receptacle in plywood drum",
            Unit::CompositePackagingGlassReceptacleInWickerworkHamper => "Composite packaging, glass receptacle in wickerwork hamper",
            Unit::CompositePackagingGlassReceptacleInFibreDrum => "Composite packaging, glass receptacle in fibre drum",
            Unit::CompositePackagingGlassReceptacleInFibreboardBox => "Composite packaging, glass receptacle in fibreboard box ",
            Unit::CompositePackagingGlassReceptacleInExpandablePlasticPack => "Composite packaging, glass receptacle in expandable plastic pack",
            Unit::CompositePackagingGlassReceptacleInSolidPlasticPack => "Composite packaging, glass receptacle in solid plastic pack",
            Unit::IntermediateBulkContainerPaperMultiWall => "Intermediate bulk container, paper, multi-wall ",
            Unit::BagLarge => "Bag, large ",
            Unit::IntermediateBulkContainerPaperMultiWallWaterResistant => "Intermediate bulk container, paper, multi-wall, water resistant",
            Unit::IntermediateBulkContainerRigidPlasticWithStructuralEquipmentSolids => "Intermediate bulk container, rigid plastic, with structural equipment, solids",
            Unit::IntermediateBulkContainerRigidPlasticFreestandingSolids => "Intermediate bulk container, rigid plastic, freestanding, solids",
            Unit::IntermediateBulkContainerRigidPlasticWithStructuralEquipmentPressurised => "Intermediate bulk container, rigid plastic, with structural equipment, pressurised",
            Unit::IntermediateBulkContainerRigidPlasticFreestandingPressurised => "Intermediate bulk container, rigid plastic, freestanding, pressurised",
            Unit::IntermediateBulkContainerRigidPlasticWithStructuralEquipmentLiquids => "Intermediate bulk container, rigid plastic, with structural equipment, liquids ",
            Unit::IntermediateBulkContainerRigidPlasticFreestandingLiquids => "Intermediate bulk container, rigid plastic, freestanding, liquids",
            Unit::IntermediateBulkContainerCompositeRigidPlasticSolids => "Intermediate bulk container, composite, rigid plastic, solids",
            Unit::IntermediateBulkContainerCompositeFlexiblePlasticSolids => "Intermediate bulk container, composite, flexible plastic, solids",
            Unit::IntermediateBulkContainerCompositeRigidPlasticPressurised => "Intermediate bulk container, composite, rigid plastic, pressurised ",
            Unit::IntermediateBulkContainerCompositeFlexiblePlasticPressurised => "Intermediate bulk container, composite, flexible plastic, pressurised",
            Unit::IntermediateBulkContainerCompositeRigidPlasticLiquids => "Intermediate bulk container, composite, rigid plastic, liquids",
            Unit::IntermediateBulkContainerCompositeFlexiblePlasticLiquids => "Intermediate bulk container, composite, flexible plastic, liquids ",
            Unit::IntermediateBulkContainerComposite => "Intermediate bulk container, composite",
            Unit::IntermediateBulkContainerFibreboard => "Intermediate bulk container, fibreboard",
            Unit::IntermediateBulkContainerFlexible => "Intermediate bulk container, flexible",
            Unit::IntermediateBulkContainerMetalOtherThanSteel => "Intermediate bulk container, metal, other than steel",
            Unit::IntermediateBulkContainerNaturalWood => "Intermediate bulk container, natural wood",
            Unit::IntermediateBulkContainerPlywood => "Intermediate bulk container, plywood",
            Unit::IntermediateBulkContainerReconstitutedWood => "Intermediate bulk container, reconstituted wood",
            Unit::MutuallyDefined_Dup => "Mutually defined ",
        }
    }
}

impl crate::FromCode for Unit {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "10" => Some(Unit::Group),
            "11" => Some(Unit::Outfit),
            "13" => Some(Unit::Ration),
            "14" => Some(Unit::Shot),
            "15" => Some(Unit::StickMilitary),
            "20" => Some(Unit::TwentyFootContainer),
            "21" => Some(Unit::FortyFootContainer),
            "22" => Some(Unit::DecilitrePerGram),
            "23" => Some(Unit::GramPerCubicCentimetre),
            "24" => Some(Unit::TheoreticalPound),
            "25" => Some(Unit::GramPerSquareCentimetre),
            "27" => Some(Unit::TheoreticalTon),
            "28" => Some(Unit::KilogramPerSquareMetre),
            "33" => Some(Unit::KilopascalSquareMetrePerGram),
            "34" => Some(Unit::KilopascalPerMillimetre),
            "35" => Some(Unit::MillilitrePerSquareCentimetreSecond),
            "37" => Some(Unit::OuncePerSquareFoot),
            "38" => Some(Unit::OuncePerSquareFootPer001inch),
            "40" => Some(Unit::MillilitrePerSecond),
            "41" => Some(Unit::MillilitrePerMinute),
            "56" => Some(Unit::Sitas),
            "57" => Some(Unit::Mesh),
            "58" => Some(Unit::NetKilogram),
            "59" => Some(Unit::PartPerMillion),
            "60" => Some(Unit::PercentWeight),
            "61" => Some(Unit::PartPerBillionUs),
            "74" => Some(Unit::Millipascal),
            "77" => Some(Unit::MilliInch),
            "80" => Some(Unit::PoundPerSquareInchAbsolute),
            "81" => Some(Unit::Henry),
            "85" => Some(Unit::FootPoundForce),
            "87" => Some(Unit::PoundPerCubicFoot),
            "89" => Some(Unit::Poise),
            "91" => Some(Unit::Stokes),
            "1I" => Some(Unit::FixedRate),
            "2A" => Some(Unit::RadianPerSecond),
            "2B" => Some(Unit::RadianPerSecondSquared),
            "2C" => Some(Unit::Roentgen),
            "2G" => Some(Unit::VoltAc),
            "2H" => Some(Unit::VoltDc),
            "2I" => Some(Unit::BritishThermalUnitInternationalTablePerHour),
            "2J" => Some(Unit::CubicCentimetrePerSecond),
            "2K" => Some(Unit::CubicFootPerHour),
            "2L" => Some(Unit::CubicFootPerMinute),
            "2M" => Some(Unit::CentimetrePerSecond),
            "2N" => Some(Unit::Decibel),
            "2P" => Some(Unit::Kilobyte),
            "2Q" => Some(Unit::Kilobecquerel),
            "2R" => Some(Unit::Kilocurie),
            "2U" => Some(Unit::Megagram),
            "2X" => Some(Unit::MetrePerMinute),
            "2Y" => Some(Unit::Milliroentgen),
            "2Z" => Some(Unit::Millivolt),
            "3B" => Some(Unit::Megajoule),
            "3C" => Some(Unit::Manmonth),
            "4C" => Some(Unit::Centistokes),
            "4G" => Some(Unit::Microlitre),
            "4H" => Some(Unit::MicrometreMicron),
            "4K" => Some(Unit::Milliampere),
            "4L" => Some(Unit::Megabyte),
            "4M" => Some(Unit::MilligramPerHour),
            "4N" => Some(Unit::Megabecquerel),
            "4O" => Some(Unit::Microfarad),
            "4P" => Some(Unit::NewtonPerMetre),
            "4Q" => Some(Unit::OunceInch),
            "4R" => Some(Unit::OunceFoot),
            "4T" => Some(Unit::Picofarad),
            "4U" => Some(Unit::PoundPerHour),
            "4W" => Some(Unit::TonUsPerHour),
            "4X" => Some(Unit::KilolitrePerHour),
            "5A" => Some(Unit::BarrelUsPerMinute),
            "5B" => Some(Unit::Batch),
            "5E" => Some(Unit::MmscfDay),
            "5J" => Some(Unit::HydraulicHorsePower),
            "A10" => Some(Unit::AmpereSquareMetrePerJouleSecond),
            "A11" => Some(Unit::Angstrom),
            "A12" => Some(Unit::AstronomicalUnit),
            "A13" => Some(Unit::Attojoule),
            "A14" => Some(Unit::Barn),
            "A15" => Some(Unit::BarnPerElectronvolt),
            "A16" => Some(Unit::BarnPerSteradianElectronvolt),
            "A17" => Some(Unit::BarnPerSteradian),
            "A18" => Some(Unit::BecquerelPerKilogram),
            "A19" => Some(Unit::BecquerelPerCubicMetre),
            "A2" => Some(Unit::AmperePerCentimetre),
            "A20" => {
                Some(Unit::BritishThermalUnitInternationalTablePerSecondSquareFootDegreeRankine)
            }
            "A21" => Some(Unit::BritishThermalUnitInternationalTablePerPoundDegreeRankine),
            "A22" => Some(Unit::BritishThermalUnitInternationalTablePerSecondFootDegreeRankine),
            "A23" => Some(Unit::BritishThermalUnitInternationalTablePerHourSquareFootDegreeRankine),
            "A24" => Some(Unit::CandelaPerSquareMetre),
            "A26" => Some(Unit::CoulombMetre),
            "A27" => Some(Unit::CoulombMetreSquaredPerVolt),
            "A28" => Some(Unit::CoulombPerCubicCentimetre),
            "A29" => Some(Unit::CoulombPerCubicMetre),
            "A3" => Some(Unit::AmperePerMillimetre),
            "A30" => Some(Unit::CoulombPerCubicMillimetre),
            "A31" => Some(Unit::CoulombPerKilogramSecond),
            "A32" => Some(Unit::CoulombPerMole),
            "A33" => Some(Unit::CoulombPerSquareCentimetre),
            "A34" => Some(Unit::CoulombPerSquareMetre),
            "A35" => Some(Unit::CoulombPerSquareMillimetre),
            "A36" => Some(Unit::CubicCentimetrePerMole),
            "A37" => Some(Unit::CubicDecimetrePerMole),
            "A38" => Some(Unit::CubicMetrePerCoulomb),
            "A39" => Some(Unit::CubicMetrePerKilogram),
            "A4" => Some(Unit::AmperePerSquareCentimetre),
            "A40" => Some(Unit::CubicMetrePerMole),
            "A41" => Some(Unit::AmperePerSquareMetre),
            "A42" => Some(Unit::CuriePerKilogram),
            "A43" => Some(Unit::DeadweightTonnage),
            "A44" => Some(Unit::Decalitre),
            "A45" => Some(Unit::Decametre),
            "A47" => Some(Unit::Decitex),
            "A48" => Some(Unit::DegreeRankine),
            "A49" => Some(Unit::Denier),
            "A5" => Some(Unit::AmpereSquareMetre),
            "A53" => Some(Unit::Electronvolt),
            "A54" => Some(Unit::ElectronvoltPerMetre),
            "A55" => Some(Unit::ElectronvoltSquareMetre),
            "A56" => Some(Unit::ElectronvoltSquareMetrePerKilogram),
            "A59" => Some(Unit::_8PartCloudCover),
            "A6" => Some(Unit::AmperePerSquareMetreKelvinSquared),
            "A68" => Some(Unit::Exajoule),
            "A69" => Some(Unit::FaradPerMetre),
            "A7" => Some(Unit::AmperePerSquareMillimetre),
            "A70" => Some(Unit::Femtojoule),
            "A71" => Some(Unit::Femtometre),
            "A73" => Some(Unit::FootPerSecondSquared),
            "A74" => Some(Unit::FootPoundForcePerSecond),
            "A75" => Some(Unit::FreightTon),
            "A76" => Some(Unit::Gal),
            "A8" => Some(Unit::AmpereSecond),
            "A84" => Some(Unit::GigacoulombPerCubicMetre),
            "A85" => Some(Unit::Gigaelectronvolt),
            "A86" => Some(Unit::Gigahertz),
            "A87" => Some(Unit::Gigaohm),
            "A88" => Some(Unit::GigaohmMetre),
            "A89" => Some(Unit::Gigapascal),
            "A9" => Some(Unit::Rate),
            "A90" => Some(Unit::Gigawatt),
            "A91" => Some(Unit::Gon),
            "A93" => Some(Unit::GramPerCubicMetre),
            "A94" => Some(Unit::GramPerMole),
            "A95" => Some(Unit::Gray),
            "A96" => Some(Unit::GrayPerSecond),
            "A97" => Some(Unit::Hectopascal),
            "A98" => Some(Unit::HenryPerMetre),
            "A99" => Some(Unit::Bit),
            "AA" => Some(Unit::Ball),
            "AB" => Some(Unit::BulkPack),
            "ACR" => Some(Unit::Acre),
            "ACT" => Some(Unit::Activity),
            "AD" => Some(Unit::Byte),
            "AE" => Some(Unit::AmperePerMetre),
            "AH" => Some(Unit::AdditionalMinute),
            "AI" => Some(Unit::AverageMinutePerCall),
            "AK" => Some(Unit::Fathom),
            "AL" => Some(Unit::AccessLine),
            "AMH" => Some(Unit::AmpereHour),
            "AMP" => Some(Unit::Ampere),
            "ANN" => Some(Unit::Year),
            "APZ" => Some(Unit::TroyOunceOrApothecaryOunce),
            "AQ" => Some(Unit::AntiHemophilicFactorAhfUnit),
            "AS" => Some(Unit::Assortment),
            "ASM" => Some(Unit::AlcoholicStrengthByMass),
            "ASU" => Some(Unit::AlcoholicStrengthByVolume),
            "ATM" => Some(Unit::StandardAtmosphere),
            "AWG" => Some(Unit::AmericanWireGauge),
            "AY" => Some(Unit::Assembly),
            "AZ" => Some(Unit::BritishThermalUnitInternationalTablePerPound),
            "B1" => Some(Unit::BarrelUsPerDay),
            "B10" => Some(Unit::BitPerSecond),
            "B11" => Some(Unit::JoulePerKilogramKelvin),
            "B12" => Some(Unit::JoulePerMetre),
            "B13" => Some(Unit::JoulePerSquareMetre),
            "B14" => Some(Unit::JoulePerMetreToFourthPower),
            "B15" => Some(Unit::JoulePerMole),
            "B16" => Some(Unit::JoulePerMoleKelvin),
            "B17" => Some(Unit::Credit),
            "B18" => Some(Unit::JouleSecond),
            "B19" => Some(Unit::Digit),
            "B20" => Some(Unit::JouleSquareMetrePerKilogram),
            "B21" => Some(Unit::KelvinPerWatt),
            "B22" => Some(Unit::Kiloampere),
            "B23" => Some(Unit::KiloamperePerSquareMetre),
            "B24" => Some(Unit::KiloamperePerMetre),
            "B25" => Some(Unit::KilobecquerelPerKilogram),
            "B26" => Some(Unit::Kilocoulomb),
            "B27" => Some(Unit::KilocoulombPerCubicMetre),
            "B28" => Some(Unit::KilocoulombPerSquareMetre),
            "B29" => Some(Unit::Kiloelectronvolt),
            "B3" => Some(Unit::BattingPound),
            "B30" => Some(Unit::Gibibit),
            "B31" => Some(Unit::KilogramMetrePerSecond),
            "B32" => Some(Unit::KilogramMetreSquared),
            "B33" => Some(Unit::KilogramMetreSquaredPerSecond),
            "B34" => Some(Unit::KilogramPerCubicDecimetre),
            "B35" => Some(Unit::KilogramPerLitre),
            "B4" => Some(Unit::BarrelImperial),
            "B41" => Some(Unit::KilojoulePerKelvin),
            "B42" => Some(Unit::KilojoulePerKilogram),
            "B43" => Some(Unit::KilojoulePerKilogramKelvin),
            "B44" => Some(Unit::KilojoulePerMole),
            "B45" => Some(Unit::Kilomole),
            "B46" => Some(Unit::KilomolePerCubicMetre),
            "B47" => Some(Unit::Kilonewton),
            "B48" => Some(Unit::KilonewtonMetre),
            "B49" => Some(Unit::Kiloohm),
            "B50" => Some(Unit::KiloohmMetre),
            "B52" => Some(Unit::Kilosecond),
            "B53" => Some(Unit::Kilosiemens),
            "B54" => Some(Unit::KilosiemensPerMetre),
            "B55" => Some(Unit::KilovoltPerMetre),
            "B56" => Some(Unit::KiloweberPerMetre),
            "B57" => Some(Unit::LightYear),
            "B58" => Some(Unit::LitrePerMole),
            "B59" => Some(Unit::LumenHour),
            "B60" => Some(Unit::LumenPerSquareMetre),
            "B61" => Some(Unit::LumenPerWatt),
            "B62" => Some(Unit::LumenSecond),
            "B63" => Some(Unit::LuxHour),
            "B64" => Some(Unit::LuxSecond),
            "B66" => Some(Unit::MegaamperePerSquareMetre),
            "B67" => Some(Unit::MegabecquerelPerKilogram),
            "B68" => Some(Unit::Gigabit),
            "B69" => Some(Unit::MegacoulombPerCubicMetre),
            "B7" => Some(Unit::Cycle),
            "B70" => Some(Unit::MegacoulombPerSquareMetre),
            "B71" => Some(Unit::Megaelectronvolt),
            "B72" => Some(Unit::MegagramPerCubicMetre),
            "B73" => Some(Unit::Meganewton),
            "B74" => Some(Unit::MeganewtonMetre),
            "B75" => Some(Unit::Megaohm),
            "B76" => Some(Unit::MegaohmMetre),
            "B77" => Some(Unit::MegasiemensPerMetre),
            "B78" => Some(Unit::Megavolt),
            "B79" => Some(Unit::MegavoltPerMetre),
            "B8" => Some(Unit::JoulePerCubicMetre),
            "B80" => Some(Unit::GigabitPerSecond),
            "B81" => Some(Unit::ReciprocalMetreSquaredReciprocalSecond),
            "B82" => Some(Unit::InchPerLinearFoot),
            "B83" => Some(Unit::MetreToFourthPower),
            "B84" => Some(Unit::Microampere),
            "B85" => Some(Unit::Microbar),
            "B86" => Some(Unit::Microcoulomb),
            "B87" => Some(Unit::MicrocoulombPerCubicMetre),
            "B88" => Some(Unit::MicrocoulombPerSquareMetre),
            "B89" => Some(Unit::MicrofaradPerMetre),
            "B90" => Some(Unit::Microhenry),
            "B91" => Some(Unit::MicrohenryPerMetre),
            "B92" => Some(Unit::Micronewton),
            "B93" => Some(Unit::MicronewtonMetre),
            "B94" => Some(Unit::Microohm),
            "B95" => Some(Unit::MicroohmMetre),
            "B96" => Some(Unit::Micropascal),
            "B97" => Some(Unit::Microradian),
            "B98" => Some(Unit::Microsecond),
            "B99" => Some(Unit::Microsiemens),
            "BAR" => Some(Unit::BarUnitPressure),
            "BB" => Some(Unit::BaseBox),
            "BFT" => Some(Unit::BoardFoot),
            "BHP" => Some(Unit::BrakeHorsePower),
            "BIL" => Some(Unit::BillionEur),
            "BLD" => Some(Unit::DryBarrelUs),
            "BLL" => Some(Unit::BarrelUs),
            "BP" => Some(Unit::HundredBoardFoot),
            "BPM" => Some(Unit::BeatsPerMinute),
            "BQL" => Some(Unit::Becquerel),
            "BTU" => Some(Unit::BritishThermalUnitInternationalTable),
            "BUA" => Some(Unit::BushelUs),
            "BUI" => Some(Unit::BushelUk),
            "C0" => Some(Unit::Call),
            "C10" => Some(Unit::Millifarad),
            "C11" => Some(Unit::Milligal),
            "C12" => Some(Unit::MilligramPerMetre),
            "C13" => Some(Unit::Milligray),
            "C14" => Some(Unit::Millihenry),
            "C15" => Some(Unit::Millijoule),
            "C16" => Some(Unit::MillimetrePerSecond),
            "C17" => Some(Unit::MillimetreSquaredPerSecond),
            "C18" => Some(Unit::Millimole),
            "C19" => Some(Unit::MolePerKilogram),
            "C20" => Some(Unit::Millinewton),
            "C21" => Some(Unit::Kibibit),
            "C22" => Some(Unit::MillinewtonPerMetre),
            "C23" => Some(Unit::MilliohmMetre),
            "C24" => Some(Unit::MillipascalSecond),
            "C25" => Some(Unit::Milliradian),
            "C26" => Some(Unit::Millisecond),
            "C27" => Some(Unit::Millisiemens),
            "C28" => Some(Unit::Millisievert),
            "C29" => Some(Unit::Millitesla),
            "C3" => Some(Unit::MicrovoltPerMetre),
            "C30" => Some(Unit::MillivoltPerMetre),
            "C31" => Some(Unit::Milliwatt),
            "C32" => Some(Unit::MilliwattPerSquareMetre),
            "C33" => Some(Unit::Milliweber),
            "C34" => Some(Unit::Mole),
            "C35" => Some(Unit::MolePerCubicDecimetre),
            "C36" => Some(Unit::MolePerCubicMetre),
            "C37" => Some(Unit::Kilobit),
            "C38" => Some(Unit::MolePerLitre),
            "C39" => Some(Unit::Nanoampere),
            "C40" => Some(Unit::Nanocoulomb),
            "C41" => Some(Unit::Nanofarad),
            "C42" => Some(Unit::NanofaradPerMetre),
            "C43" => Some(Unit::Nanohenry),
            "C44" => Some(Unit::NanohenryPerMetre),
            "C45" => Some(Unit::Nanometre),
            "C46" => Some(Unit::NanoohmMetre),
            "C47" => Some(Unit::Nanosecond),
            "C48" => Some(Unit::Nanotesla),
            "C49" => Some(Unit::Nanowatt),
            "C50" => Some(Unit::Neper),
            "C51" => Some(Unit::NeperPerSecond),
            "C52" => Some(Unit::Picometre),
            "C53" => Some(Unit::NewtonMetreSecond),
            "C54" => Some(Unit::NewtonMetreSquaredPerKilogramSquared),
            "C55" => Some(Unit::NewtonPerSquareMetre),
            "C56" => Some(Unit::NewtonPerSquareMillimetre),
            "C57" => Some(Unit::NewtonSecond),
            "C58" => Some(Unit::NewtonSecondPerMetre),
            "C59" => Some(Unit::Octave),
            "C60" => Some(Unit::OhmCentimetre),
            "C61" => Some(Unit::OhmMetre),
            "C62" => Some(Unit::One),
            "C63" => Some(Unit::Parsec),
            "C64" => Some(Unit::PascalPerKelvin),
            "C65" => Some(Unit::PascalSecond),
            "C66" => Some(Unit::PascalSecondPerCubicMetre),
            "C67" => Some(Unit::PascalSecondPerMetre),
            "C68" => Some(Unit::Petajoule),
            "C69" => Some(Unit::Phon),
            "C7" => Some(Unit::Centipoise),
            "C70" => Some(Unit::Picoampere),
            "C71" => Some(Unit::Picocoulomb),
            "C72" => Some(Unit::PicofaradPerMetre),
            "C73" => Some(Unit::Picohenry),
            "C74" => Some(Unit::KilobitPerSecond),
            "C75" => Some(Unit::Picowatt),
            "C76" => Some(Unit::PicowattPerSquareMetre),
            "C78" => Some(Unit::PoundForce),
            "C79" => Some(Unit::KilovoltAmpereHour),
            "C8" => Some(Unit::MillicoulombPerKilogram),
            "C80" => Some(Unit::Rad),
            "C81" => Some(Unit::Radian),
            "C82" => Some(Unit::RadianSquareMetrePerMole),
            "C83" => Some(Unit::RadianSquareMetrePerKilogram),
            "C84" => Some(Unit::RadianPerMetre),
            "C85" => Some(Unit::ReciprocalAngstrom),
            "C86" => Some(Unit::ReciprocalCubicMetre),
            "C87" => Some(Unit::ReciprocalCubicMetrePerSecond),
            "C88" => Some(Unit::ReciprocalElectronVoltPerCubicMetre),
            "C89" => Some(Unit::ReciprocalHenry),
            "C9" => Some(Unit::CoilGroup),
            "C90" => Some(Unit::ReciprocalJoulePerCubicMetre),
            "C91" => Some(Unit::ReciprocalKelvinOrKelvinToPowerMinusOne),
            "C92" => Some(Unit::ReciprocalMetre),
            "C93" => Some(Unit::ReciprocalSquareMetre),
            "C94" => Some(Unit::ReciprocalMinute),
            "C95" => Some(Unit::ReciprocalMole),
            "C96" => Some(Unit::ReciprocalPascalOrPascalToPowerMinusOne),
            "C97" => Some(Unit::ReciprocalSecond),
            "C99" => Some(Unit::ReciprocalSecondPerMetreSquared),
            "CCT" => Some(Unit::CarryingCapacityInMetricTon),
            "CDL" => Some(Unit::Candela),
            "CEL" => Some(Unit::DegreeCelsius),
            "CEN" => Some(Unit::Hundred),
            "CG" => Some(Unit::Card),
            "CGM" => Some(Unit::Centigram),
            "CKG" => Some(Unit::CoulombPerKilogram),
            "CLF" => Some(Unit::HundredLeave),
            "CLT" => Some(Unit::Centilitre),
            "CMK" => Some(Unit::SquareCentimetre),
            "CMQ" => Some(Unit::CubicCentimetre),
            "CMT" => Some(Unit::Centimetre),
            "CNP" => Some(Unit::HundredPack),
            "CNT" => Some(Unit::CentalUk),
            "COU" => Some(Unit::Coulomb),
            "CTG" => Some(Unit::ContentGram),
            "CTM" => Some(Unit::MetricCarat),
            "CTN" => Some(Unit::ContentTonMetric),
            "CUR" => Some(Unit::Curie),
            "CWA" => Some(Unit::HundredPoundCwtHundredWeightUs),
            "CWI" => Some(Unit::HundredWeightUk),
            "D03" => Some(Unit::KilowattHourPerHour),
            "D04" => Some(Unit::LotUnitWeight),
            "D1" => Some(Unit::ReciprocalSecondPerSteradian),
            "D10" => Some(Unit::SiemensPerMetre),
            "D11" => Some(Unit::Mebibit),
            "D12" => Some(Unit::SiemensSquareMetrePerMole),
            "D13" => Some(Unit::Sievert),
            "D15" => Some(Unit::Sone),
            "D16" => Some(Unit::SquareCentimetrePerErg),
            "D17" => Some(Unit::SquareCentimetrePerSteradianErg),
            "D18" => Some(Unit::MetreKelvin),
            "D19" => Some(Unit::SquareMetreKelvinPerWatt),
            "D2" => Some(Unit::ReciprocalSecondPerSteradianMetreSquared),
            "D20" => Some(Unit::SquareMetrePerJoule),
            "D21" => Some(Unit::SquareMetrePerKilogram),
            "D22" => Some(Unit::SquareMetrePerMole),
            "D23" => Some(Unit::PenGramProtein),
            "D24" => Some(Unit::SquareMetrePerSteradian),
            "D25" => Some(Unit::SquareMetrePerSteradianJoule),
            "D26" => Some(Unit::SquareMetrePerVoltSecond),
            "D27" => Some(Unit::Steradian),
            "D29" => Some(Unit::Terahertz),
            "D30" => Some(Unit::Terajoule),
            "D31" => Some(Unit::Terawatt),
            "D32" => Some(Unit::TerawattHour),
            "D33" => Some(Unit::Tesla),
            "D34" => Some(Unit::Tex),
            "D36" => Some(Unit::Megabit),
            "D41" => Some(Unit::TonnePerCubicMetre),
            "D42" => Some(Unit::TropicalYear),
            "D43" => Some(Unit::UnifiedAtomicMassUnit),
            "D44" => Some(Unit::Var),
            "D45" => Some(Unit::VoltSquaredPerKelvinSquared),
            "D46" => Some(Unit::VoltAmpere),
            "D47" => Some(Unit::VoltPerCentimetre),
            "D48" => Some(Unit::VoltPerKelvin),
            "D49" => Some(Unit::MillivoltPerKelvin),
            "D5" => Some(Unit::KilogramPerSquareCentimetre),
            "D50" => Some(Unit::VoltPerMetre),
            "D51" => Some(Unit::VoltPerMillimetre),
            "D52" => Some(Unit::WattPerKelvin),
            "D53" => Some(Unit::WattPerMetreKelvin),
            "D54" => Some(Unit::WattPerSquareMetre),
            "D55" => Some(Unit::WattPerSquareMetreKelvin),
            "D56" => Some(Unit::WattPerSquareMetreKelvinToFourthPower),
            "D57" => Some(Unit::WattPerSteradian),
            "D58" => Some(Unit::WattPerSteradianSquareMetre),
            "D59" => Some(Unit::WeberPerMetre),
            "D6" => Some(Unit::RoentgenPerSecond),
            "D60" => Some(Unit::WeberPerMillimetre),
            "D61" => Some(Unit::MinuteUnitAngle),
            "D62" => Some(Unit::SecondUnitAngle),
            "D63" => Some(Unit::Book),
            "D65" => Some(Unit::Round),
            "D68" => Some(Unit::NumberWords),
            "D69" => Some(Unit::InchToFourthPower),
            "D73" => Some(Unit::JouleSquareMetre),
            "D74" => Some(Unit::KilogramPerMole),
            "D77" => Some(Unit::Megacoulomb),
            "D78" => Some(Unit::MegajoulePerSecond),
            "D80" => Some(Unit::Microwatt),
            "D81" => Some(Unit::Microtesla),
            "D82" => Some(Unit::Microvolt),
            "D83" => Some(Unit::MillinewtonMetre),
            "D85" => Some(Unit::MicrowattPerSquareMetre),
            "D86" => Some(Unit::Millicoulomb),
            "D87" => Some(Unit::MillimolePerKilogram),
            "D88" => Some(Unit::MillicoulombPerCubicMetre),
            "D89" => Some(Unit::MillicoulombPerSquareMetre),
            "D91" => Some(Unit::Rem),
            "D93" => Some(Unit::SecondPerCubicMetre),
            "D94" => Some(Unit::SecondPerCubicMetreRadian),
            "D95" => Some(Unit::JoulePerGram),
            "DAA" => Some(Unit::Decare),
            "DAD" => Some(Unit::TenDay),
            "DAY" => Some(Unit::Day),
            "DB" => Some(Unit::DryPound),
            "DBM" => Some(Unit::DecibelMilliwatts),
            "DBW" => Some(Unit::DecibelWatt),
            "DD" => Some(Unit::DegreeUnitAngle),
            "DEC" => Some(Unit::Decade),
            "DG" => Some(Unit::Decigram),
            "DJ" => Some(Unit::Decagram),
            "DLT" => Some(Unit::Decilitre),
            "DMA" => Some(Unit::CubicDecametre),
            "DMK" => Some(Unit::SquareDecimetre),
            "DMO" => Some(Unit::StandardKilolitre),
            "DMQ" => Some(Unit::CubicDecimetre),
            "DMT" => Some(Unit::Decimetre),
            "DN" => Some(Unit::DecinewtonMetre),
            "DPC" => Some(Unit::DozenPiece),
            "DPR" => Some(Unit::DozenPair),
            "DPT" => Some(Unit::DisplacementTonnage),
            "DRA" => Some(Unit::DramUs),
            "DRI" => Some(Unit::DramUk),
            "DRL" => Some(Unit::DozenRoll),
            "DT" => Some(Unit::DryTon),
            "DTN" => Some(Unit::Decitonne),
            "DWT" => Some(Unit::Pennyweight),
            "DZN" => Some(Unit::Dozen),
            "DZP" => Some(Unit::DozenPack),
            "E01" => Some(Unit::NewtonPerSquareCentimetre),
            "E07" => Some(Unit::MegawattHourPerHour),
            "E08" => Some(Unit::MegawattPerHertz),
            "E09" => Some(Unit::MilliampereHour),
            "E10" => Some(Unit::DegreeDay),
            "E12" => Some(Unit::Mille),
            "E14" => Some(Unit::KilocalorieInternationalTable),
            "E15" => Some(Unit::KilocalorieThermochemicalPerHour),
            "E16" => Some(Unit::MillionBtuItPerHour),
            "E17" => Some(Unit::CubicFootPerSecond),
            "E18" => Some(Unit::TonnePerHour),
            "E19" => Some(Unit::Ping),
            "E20" => Some(Unit::MegabitPerSecond),
            "E21" => Some(Unit::Shares),
            "E22" => Some(Unit::Teu),
            "E23" => Some(Unit::Tyre),
            "E25" => Some(Unit::ActiveUnit),
            "E27" => Some(Unit::Dose),
            "E28" => Some(Unit::AirDryTon),
            "E30" => Some(Unit::Strand),
            "E31" => Some(Unit::SquareMetrePerLitre),
            "E32" => Some(Unit::LitrePerHour),
            "E33" => Some(Unit::FootPerThousand),
            "E34" => Some(Unit::Gigabyte),
            "E35" => Some(Unit::Terabyte),
            "E36" => Some(Unit::Petabyte),
            "E37" => Some(Unit::Pixel),
            "E38" => Some(Unit::Megapixel),
            "E39" => Some(Unit::DotsPerInch),
            "E4" => Some(Unit::GrossKilogram),
            "E40" => Some(Unit::PartPerHundredThousand),
            "E41" => Some(Unit::KilogramForcePerSquareMillimetre),
            "E42" => Some(Unit::KilogramForcePerSquareCentimetre),
            "E43" => Some(Unit::JoulePerSquareCentimetre),
            "E44" => Some(Unit::KilogramForceMetrePerSquareCentimetre),
            "E45" => Some(Unit::Milliohm),
            "E46" => Some(Unit::KilowattHourPerCubicMetre),
            "E47" => Some(Unit::KilowattHourPerKelvin),
            "E48" => Some(Unit::ServiceUnit),
            "E49" => Some(Unit::WorkingDay),
            "E50" => Some(Unit::AccountingUnit),
            "E51" => Some(Unit::Job),
            "E52" => Some(Unit::RunFoot),
            "E53" => Some(Unit::Test),
            "E54" => Some(Unit::Trip),
            "E55" => Some(Unit::Use),
            "E56" => Some(Unit::Well),
            "E57" => Some(Unit::Zone),
            "E58" => Some(Unit::ExabitPerSecond),
            "E59" => Some(Unit::Exbibyte),
            "E60" => Some(Unit::Pebibyte),
            "E61" => Some(Unit::Tebibyte),
            "E62" => Some(Unit::Gibibyte),
            "E63" => Some(Unit::Mebibyte),
            "E64" => Some(Unit::Kibibyte),
            "E65" => Some(Unit::ExbibitPerMetre),
            "E66" => Some(Unit::ExbibitPerSquareMetre),
            "E67" => Some(Unit::ExbibitPerCubicMetre),
            "E68" => Some(Unit::GigabytePerSecond),
            "E69" => Some(Unit::GibibitPerMetre),
            "E70" => Some(Unit::GibibitPerSquareMetre),
            "E71" => Some(Unit::GibibitPerCubicMetre),
            "E72" => Some(Unit::KibibitPerMetre),
            "E73" => Some(Unit::KibibitPerSquareMetre),
            "E74" => Some(Unit::KibibitPerCubicMetre),
            "E75" => Some(Unit::MebibitPerMetre),
            "E76" => Some(Unit::MebibitPerSquareMetre),
            "E77" => Some(Unit::MebibitPerCubicMetre),
            "E78" => Some(Unit::Petabit),
            "E79" => Some(Unit::PetabitPerSecond),
            "E80" => Some(Unit::PebibitPerMetre),
            "E81" => Some(Unit::PebibitPerSquareMetre),
            "E82" => Some(Unit::PebibitPerCubicMetre),
            "E83" => Some(Unit::Terabit),
            "E84" => Some(Unit::TerabitPerSecond),
            "E85" => Some(Unit::TebibitPerMetre),
            "E86" => Some(Unit::TebibitPerCubicMetre),
            "E87" => Some(Unit::TebibitPerSquareMetre),
            "E88" => Some(Unit::BitPerMetre),
            "E89" => Some(Unit::BitPerSquareMetre),
            "E90" => Some(Unit::ReciprocalCentimetre),
            "E91" => Some(Unit::ReciprocalDay),
            "E92" => Some(Unit::CubicDecimetrePerHour),
            "E93" => Some(Unit::KilogramPerHour),
            "E94" => Some(Unit::KilomolePerSecond),
            "E95" => Some(Unit::MolePerSecond),
            "E96" => Some(Unit::DegreePerSecond),
            "E97" => Some(Unit::MillimetrePerDegreeCelciusMetre),
            "E98" => Some(Unit::DegreeCelsiusPerKelvin),
            "E99" => Some(Unit::HectopascalPerBar),
            "EA" => Some(Unit::Each),
            "EB" => Some(Unit::ElectronicMailBox),
            "EQ" => Some(Unit::EquivalentGallon),
            "F01" => Some(Unit::BitPerCubicMetre),
            "F02" => Some(Unit::KelvinPerKelvin),
            "F03" => Some(Unit::KilopascalPerBar),
            "F04" => Some(Unit::MillibarPerBar),
            "F05" => Some(Unit::MegapascalPerBar),
            "F06" => Some(Unit::PoisePerBar),
            "F07" => Some(Unit::PascalPerBar),
            "F08" => Some(Unit::MilliamperePerInch),
            "F10" => Some(Unit::KelvinPerHour),
            "F11" => Some(Unit::KelvinPerMinute),
            "F12" => Some(Unit::KelvinPerSecond),
            "F13" => Some(Unit::Slug),
            "F14" => Some(Unit::GramPerKelvin),
            "F15" => Some(Unit::KilogramPerKelvin),
            "F16" => Some(Unit::MilligramPerKelvin),
            "F17" => Some(Unit::PoundForcePerFoot),
            "F18" => Some(Unit::KilogramSquareCentimetre),
            "F19" => Some(Unit::KilogramSquareMillimetre),
            "F20" => Some(Unit::PoundInchSquared),
            "F21" => Some(Unit::PoundForceInch),
            "F22" => Some(Unit::PoundForceFootPerAmpere),
            "F23" => Some(Unit::GramPerCubicDecimetre),
            "F24" => Some(Unit::KilogramPerKilomol),
            "F25" => Some(Unit::GramPerHertz),
            "F26" => Some(Unit::GramPerDay),
            "F27" => Some(Unit::GramPerHour),
            "F28" => Some(Unit::GramPerMinute),
            "F29" => Some(Unit::GramPerSecond),
            "F30" => Some(Unit::KilogramPerDay),
            "F31" => Some(Unit::KilogramPerMinute),
            "F32" => Some(Unit::MilligramPerDay),
            "F33" => Some(Unit::MilligramPerMinute),
            "F34" => Some(Unit::MilligramPerSecond),
            "F35" => Some(Unit::GramPerDayKelvin),
            "F36" => Some(Unit::GramPerHourKelvin),
            "F37" => Some(Unit::GramPerMinuteKelvin),
            "F38" => Some(Unit::GramPerSecondKelvin),
            "F39" => Some(Unit::KilogramPerDayKelvin),
            "F40" => Some(Unit::KilogramPerHourKelvin),
            "F41" => Some(Unit::KilogramPerMinuteKelvin),
            "F42" => Some(Unit::KilogramPerSecondKelvin),
            "F43" => Some(Unit::MilligramPerDayKelvin),
            "F44" => Some(Unit::MilligramPerHourKelvin),
            "F45" => Some(Unit::MilligramPerMinuteKelvin),
            "F46" => Some(Unit::MilligramPerSecondKelvin),
            "F47" => Some(Unit::NewtonPerMillimetre),
            "F48" => Some(Unit::PoundForcePerInch),
            "F49" => Some(Unit::RodUnitDistance),
            "F50" => Some(Unit::MicrometrePerKelvin),
            "F51" => Some(Unit::CentimetrePerKelvin),
            "F52" => Some(Unit::MetrePerKelvin),
            "F53" => Some(Unit::MillimetrePerKelvin),
            "F54" => Some(Unit::MilliohmPerMetre),
            "F55" => Some(Unit::OhmPerMileStatuteMile),
            "F56" => Some(Unit::OhmPerKilometre),
            "F57" => Some(Unit::MilliamperePerPoundForcePerSquareInch),
            "F58" => Some(Unit::ReciprocalBar),
            "F59" => Some(Unit::MilliamperePerBar),
            "F60" => Some(Unit::DegreeCelsiusPerBar),
            "F61" => Some(Unit::KelvinPerBar),
            "F62" => Some(Unit::GramPerDayBar),
            "F63" => Some(Unit::GramPerHourBar),
            "F64" => Some(Unit::GramPerMinuteBar),
            "F65" => Some(Unit::GramPerSecondBar),
            "F66" => Some(Unit::KilogramPerDayBar),
            "F67" => Some(Unit::KilogramPerHourBar),
            "F68" => Some(Unit::KilogramPerMinuteBar),
            "F69" => Some(Unit::KilogramPerSecondBar),
            "F70" => Some(Unit::MilligramPerDayBar),
            "F71" => Some(Unit::MilligramPerHourBar),
            "F72" => Some(Unit::MilligramPerMinuteBar),
            "F73" => Some(Unit::MilligramPerSecondBar),
            "F74" => Some(Unit::GramPerBar),
            "F75" => Some(Unit::MilligramPerBar),
            "F76" => Some(Unit::MilliamperePerMillimetre),
            "F77" => Some(Unit::PascalSecondPerKelvin),
            "F78" => Some(Unit::InchWater),
            "F79" => Some(Unit::InchMercury),
            "F80" => Some(Unit::WaterHorsePower),
            "F81" => Some(Unit::BarPerKelvin),
            "F82" => Some(Unit::HectopascalPerKelvin),
            "F83" => Some(Unit::KilopascalPerKelvin),
            "F84" => Some(Unit::MillibarPerKelvin),
            "F85" => Some(Unit::MegapascalPerKelvin),
            "F86" => Some(Unit::PoisePerKelvin),
            "F87" => Some(Unit::VoltPerLitreMinute),
            "F88" => Some(Unit::NewtonCentimetre),
            "F89" => Some(Unit::NewtonMetrePerDegree),
            "F90" => Some(Unit::NewtonMetrePerAmpere),
            "F91" => Some(Unit::BarLitrePerSecond),
            "F92" => Some(Unit::BarCubicMetrePerSecond),
            "F93" => Some(Unit::HectopascalLitrePerSecond),
            "F94" => Some(Unit::HectopascalCubicMetrePerSecond),
            "F95" => Some(Unit::MillibarLitrePerSecond),
            "F96" => Some(Unit::MillibarCubicMetrePerSecond),
            "F97" => Some(Unit::MegapascalLitrePerSecond),
            "F98" => Some(Unit::MegapascalCubicMetrePerSecond),
            "F99" => Some(Unit::PascalLitrePerSecond),
            "FAH" => Some(Unit::DegreeFahrenheit),
            "FAR" => Some(Unit::Farad),
            "FBM" => Some(Unit::FibreMetre),
            "FC" => Some(Unit::ThousandCubicFoot),
            "FF" => Some(Unit::HundredCubicMetre),
            "FH" => Some(Unit::Micromole),
            "FIT" => Some(Unit::FailuresInTime),
            "FL" => Some(Unit::FlakeTon),
            "FNU" => Some(Unit::FormazinNephelometricUnit),
            "FOT" => Some(Unit::Foot),
            "FP" => Some(Unit::PoundPerSquareFoot),
            "FR" => Some(Unit::FootPerMinute),
            "FS" => Some(Unit::FootPerSecond),
            "FTK" => Some(Unit::SquareFoot),
            "FTQ" => Some(Unit::CubicFoot),
            "G01" => Some(Unit::PascalCubicMetrePerSecond),
            "G04" => Some(Unit::CentimetrePerBar),
            "G05" => Some(Unit::MetrePerBar),
            "G06" => Some(Unit::MillimetrePerBar),
            "G08" => Some(Unit::SquareInchPerSecond),
            "G09" => Some(Unit::SquareMetrePerSecondKelvin),
            "G10" => Some(Unit::StokesPerKelvin),
            "G11" => Some(Unit::GramPerCubicCentimetreBar),
            "G12" => Some(Unit::GramPerCubicDecimetreBar),
            "G13" => Some(Unit::GramPerLitreBar),
            "G14" => Some(Unit::GramPerCubicMetreBar),
            "G15" => Some(Unit::GramPerMillilitreBar),
            "G16" => Some(Unit::KilogramPerCubicCentimetreBar),
            "G17" => Some(Unit::KilogramPerLitreBar),
            "G18" => Some(Unit::KilogramPerCubicMetreBar),
            "G19" => Some(Unit::NewtonMetrePerKilogram),
            "G2" => Some(Unit::UsGallonPerMinute),
            "G20" => Some(Unit::PoundForceFootPerPound),
            "G21" => Some(Unit::CupUnitVolume),
            "G23" => Some(Unit::Peck),
            "G24" => Some(Unit::TablespoonUs),
            "G25" => Some(Unit::TeaspoonUs),
            "G26" => Some(Unit::Stere),
            "G27" => Some(Unit::CubicCentimetrePerKelvin),
            "G28" => Some(Unit::LitrePerKelvin),
            "G29" => Some(Unit::CubicMetrePerKelvin),
            "G3" => Some(Unit::ImperialGallonPerMinute),
            "G30" => Some(Unit::MillilitrePerKelvin),
            "G31" => Some(Unit::KilogramPerCubicCentimetre),
            "G32" => Some(Unit::OunceAvoirdupoisPerCubicYard),
            "G33" => Some(Unit::GramPerCubicCentimetreKelvin),
            "G34" => Some(Unit::GramPerCubicDecimetreKelvin),
            "G35" => Some(Unit::GramPerLitreKelvin),
            "G36" => Some(Unit::GramPerCubicMetreKelvin),
            "G37" => Some(Unit::GramPerMillilitreKelvin),
            "G38" => Some(Unit::KilogramPerCubicCentimetreKelvin),
            "G39" => Some(Unit::KilogramPerLitreKelvin),
            "G40" => Some(Unit::KilogramPerCubicMetreKelvin),
            "G41" => Some(Unit::SquareMetrePerSecondBar),
            "G42" => Some(Unit::MicrosiemensPerCentimetre),
            "G43" => Some(Unit::MicrosiemensPerMetre),
            "G44" => Some(Unit::NanosiemensPerCentimetre),
            "G45" => Some(Unit::NanosiemensPerMetre),
            "G46" => Some(Unit::StokesPerBar),
            "G47" => Some(Unit::CubicCentimetrePerDay),
            "G48" => Some(Unit::CubicCentimetrePerHour),
            "G49" => Some(Unit::CubicCentimetrePerMinute),
            "G50" => Some(Unit::GallonUsPerHour),
            "G51" => Some(Unit::LitrePerSecond),
            "G52" => Some(Unit::CubicMetrePerDay),
            "G53" => Some(Unit::CubicMetrePerMinute),
            "G54" => Some(Unit::MillilitrePerDay),
            "G55" => Some(Unit::MillilitrePerHour),
            "G56" => Some(Unit::CubicInchPerHour),
            "G57" => Some(Unit::CubicInchPerMinute),
            "G58" => Some(Unit::CubicInchPerSecond),
            "G59" => Some(Unit::MilliamperePerLitreMinute),
            "G60" => Some(Unit::VoltPerBar),
            "G61" => Some(Unit::CubicCentimetrePerDayKelvin),
            "G62" => Some(Unit::CubicCentimetrePerHourKelvin),
            "G63" => Some(Unit::CubicCentimetrePerMinuteKelvin),
            "G64" => Some(Unit::CubicCentimetrePerSecondKelvin),
            "G65" => Some(Unit::LitrePerDayKelvin),
            "G66" => Some(Unit::LitrePerHourKelvin),
            "G67" => Some(Unit::LitrePerMinuteKelvin),
            "G68" => Some(Unit::LitrePerSecondKelvin),
            "G69" => Some(Unit::CubicMetrePerDayKelvin),
            "G70" => Some(Unit::CubicMetrePerHourKelvin),
            "G71" => Some(Unit::CubicMetrePerMinuteKelvin),
            "G72" => Some(Unit::CubicMetrePerSecondKelvin),
            "G73" => Some(Unit::MillilitrePerDayKelvin),
            "G74" => Some(Unit::MillilitrePerHourKelvin),
            "G75" => Some(Unit::MillilitrePerMinuteKelvin),
            "G76" => Some(Unit::MillilitrePerSecondKelvin),
            "G77" => Some(Unit::MillimetreToFourthPower),
            "G78" => Some(Unit::CubicCentimetrePerDayBar),
            "G79" => Some(Unit::CubicCentimetrePerHourBar),
            "G80" => Some(Unit::CubicCentimetrePerMinuteBar),
            "G81" => Some(Unit::CubicCentimetrePerSecondBar),
            "G82" => Some(Unit::LitrePerDayBar),
            "G83" => Some(Unit::LitrePerHourBar),
            "G84" => Some(Unit::LitrePerMinuteBar),
            "G85" => Some(Unit::LitrePerSecondBar),
            "G86" => Some(Unit::CubicMetrePerDayBar),
            "G87" => Some(Unit::CubicMetrePerHourBar),
            "G88" => Some(Unit::CubicMetrePerMinuteBar),
            "G89" => Some(Unit::CubicMetrePerSecondBar),
            "G90" => Some(Unit::MillilitrePerDayBar),
            "G91" => Some(Unit::MillilitrePerHourBar),
            "G92" => Some(Unit::MillilitrePerMinuteBar),
            "G93" => Some(Unit::MillilitrePerSecondBar),
            "G94" => Some(Unit::CubicCentimetrePerBar),
            "G95" => Some(Unit::LitrePerBar),
            "G96" => Some(Unit::CubicMetrePerBar),
            "G97" => Some(Unit::MillilitrePerBar),
            "G98" => Some(Unit::MicrohenryPerKiloohm),
            "G99" => Some(Unit::MicrohenryPerOhm),
            "GB" => Some(Unit::GallonUsPerDay),
            "GBQ" => Some(Unit::Gigabecquerel),
            "GDW" => Some(Unit::GramDryWeight),
            "GE" => Some(Unit::PoundPerGallonUs),
            "GF" => Some(Unit::GramPerMetreGramPer100Centimetres),
            "GFI" => Some(Unit::GramFissileIsotope),
            "GGR" => Some(Unit::GreatGross),
            "GIA" => Some(Unit::GillUs),
            "GIC" => Some(Unit::GramIncludingContainer),
            "GII" => Some(Unit::GillUk),
            "GIP" => Some(Unit::GramIncludingInnerPackaging),
            "GJ" => Some(Unit::GramPerMillilitre),
            "GL" => Some(Unit::GramPerLitre),
            "GLD" => Some(Unit::DryGallonUs),
            "GLI" => Some(Unit::GallonUk),
            "GLL" => Some(Unit::GallonUs),
            "GM" => Some(Unit::GramPerSquareMetre),
            "GO" => Some(Unit::MilligramPerSquareMetre),
            "GP" => Some(Unit::MilligramPerCubicMetre),
            "GQ" => Some(Unit::MicrogramPerCubicMetre),
            "GRM" => Some(Unit::Gram),
            "GRN" => Some(Unit::Grain),
            "GRO" => Some(Unit::Gross),
            "GV" => Some(Unit::Gigajoule),
            "GWH" => Some(Unit::GigawattHour),
            "H03" => Some(Unit::HenryPerKiloohm),
            "H04" => Some(Unit::HenryPerOhm),
            "H05" => Some(Unit::MillihenryPerKiloohm),
            "H06" => Some(Unit::MillihenryPerOhm),
            "H07" => Some(Unit::PascalSecondPerBar),
            "H08" => Some(Unit::Microbecquerel),
            "H09" => Some(Unit::ReciprocalYear),
            "H10" => Some(Unit::ReciprocalHour),
            "H11" => Some(Unit::ReciprocalMonth),
            "H12" => Some(Unit::DegreeCelsiusPerHour),
            "H13" => Some(Unit::DegreeCelsiusPerMinute),
            "H14" => Some(Unit::DegreeCelsiusPerSecond),
            "H15" => Some(Unit::SquareCentimetrePerGram),
            "H16" => Some(Unit::SquareDecametre),
            "H18" => Some(Unit::SquareHectometre),
            "H19" => Some(Unit::CubicHectometre),
            "H20" => Some(Unit::CubicKilometre),
            "H21" => Some(Unit::Blank),
            "H22" => Some(Unit::VoltSquareInchPerPoundForce),
            "H23" => Some(Unit::VoltPerInch),
            "H24" => Some(Unit::VoltPerMicrosecond),
            "H25" => Some(Unit::PercentPerKelvin),
            "H26" => Some(Unit::OhmPerMetre),
            "H27" => Some(Unit::DegreePerMetre),
            "H28" => Some(Unit::MicrofaradPerKilometre),
            "H29" => Some(Unit::MicrogramPerLitre),
            "H30" => Some(Unit::SquareMicrometreSquareMicron),
            "H31" => Some(Unit::AmperePerKilogram),
            "H32" => Some(Unit::AmpereSquaredSecond),
            "H33" => Some(Unit::FaradPerKilometre),
            "H34" => Some(Unit::HertzMetre),
            "H35" => Some(Unit::KelvinMetrePerWatt),
            "H36" => Some(Unit::MegaohmPerKilometre),
            "H37" => Some(Unit::MegaohmPerMetre),
            "H38" => Some(Unit::Megaampere),
            "H39" => Some(Unit::MegahertzKilometre),
            "H40" => Some(Unit::NewtonPerAmpere),
            "H41" => Some(Unit::NewtonMetreWattToPowerMinus05),
            "H42" => Some(Unit::PascalPerMetre),
            "H43" => Some(Unit::SiemensPerCentimetre),
            "H44" => Some(Unit::Teraohm),
            "H45" => Some(Unit::VoltSecondPerMetre),
            "H46" => Some(Unit::VoltPerSecond),
            "H47" => Some(Unit::WattPerCubicMetre),
            "H48" => Some(Unit::Attofarad),
            "H49" => Some(Unit::CentimetrePerHour),
            "H50" => Some(Unit::ReciprocalCubicCentimetre),
            "H51" => Some(Unit::DecibelPerKilometre),
            "H52" => Some(Unit::DecibelPerMetre),
            "H53" => Some(Unit::KilogramPerBar),
            "H54" => Some(Unit::KilogramPerCubicDecimetreKelvin),
            "H55" => Some(Unit::KilogramPerCubicDecimetreBar),
            "H56" => Some(Unit::KilogramPerSquareMetreSecond),
            "H57" => Some(Unit::InchPerTwoPiRadiant),
            "H58" => Some(Unit::MetrePerVoltSecond),
            "H59" => Some(Unit::SquareMetrePerNewton),
            "H60" => Some(Unit::CubicMetrePerCubicMetre),
            "H61" => Some(Unit::MillisiemensPerCentimetre),
            "H62" => Some(Unit::MillivoltPerMinute),
            "H63" => Some(Unit::MilligramPerSquareCentimetre),
            "H64" => Some(Unit::MilligramPerGram),
            "H65" => Some(Unit::MillilitrePerCubicMetre),
            "H66" => Some(Unit::MillimetrePerYear),
            "H67" => Some(Unit::MillimetrePerHour),
            "H68" => Some(Unit::MillimolePerGram),
            "H69" => Some(Unit::PicopascalPerKilometre),
            "H70" => Some(Unit::Picosecond),
            "H71" => Some(Unit::PercentPerMonth),
            "H72" => Some(Unit::PercentPerHectobar),
            "H73" => Some(Unit::PercentPerDecakelvin),
            "H74" => Some(Unit::WattPerMetre),
            "H75" => Some(Unit::Decapascal),
            "H76" => Some(Unit::GramPerMillimetre),
            "H77" => Some(Unit::ModuleWidth),
            "H79" => Some(Unit::FrenchGauge),
            "H80" => Some(Unit::RackUnit),
            "H81" => Some(Unit::MillimetrePerMinute),
            "H82" => Some(Unit::BigPoint),
            "H83" => Some(Unit::LitrePerKilogram),
            "H84" => Some(Unit::GramMillimetre),
            "H85" => Some(Unit::ReciprocalWeek),
            "H87" => Some(Unit::Piece),
            "H88" => Some(Unit::MegaohmKilometre),
            "H89" => Some(Unit::PercentPerOhm),
            "H90" => Some(Unit::PercentPerDegree),
            "H91" => Some(Unit::PercentPerTenThousand),
            "H92" => Some(Unit::PercentPerOneHundredThousand),
            "H93" => Some(Unit::PercentPerHundred),
            "H94" => Some(Unit::PercentPerThousand),
            "H95" => Some(Unit::PercentPerVolt),
            "H96" => Some(Unit::PercentPerBar),
            "H98" => Some(Unit::PercentPerInch),
            "H99" => Some(Unit::PercentPerMetre),
            "HA" => Some(Unit::Hank),
            "HAD" => Some(Unit::PieceDay),
            "HBA" => Some(Unit::Hectobar),
            "HBX" => Some(Unit::HundredBoxes),
            "HC" => Some(Unit::HundredCount),
            "HDW" => Some(Unit::HundredKilogramDryWeight),
            "HEA" => Some(Unit::Head),
            "HGM" => Some(Unit::Hectogram),
            "HH" => Some(Unit::HundredCubicFoot),
            "HIU" => Some(Unit::HundredInternationalUnit),
            "HKM" => Some(Unit::HundredKilogramNetMass),
            "HLT" => Some(Unit::Hectolitre),
            "HM" => Some(Unit::MilePerHourStatuteMile),
            "HMO" => Some(Unit::PieceMonth),
            "HMQ" => Some(Unit::MillionCubicMetre),
            "HMT" => Some(Unit::Hectometre),
            "HPA" => Some(Unit::HectolitrePureAlcohol),
            "HTZ" => Some(Unit::Hertz),
            "HUR" => Some(Unit::Hour),
            "HWE" => Some(Unit::PieceWeek),
            "IA" => Some(Unit::InchPoundPoundInch),
            "IE" => Some(Unit::Person),
            "INH" => Some(Unit::Inch),
            "INK" => Some(Unit::SquareInch),
            "INQ" => Some(Unit::CubicInch),
            "ISD" => Some(Unit::InternationalSugarDegree),
            "IU" => Some(Unit::InchPerSecond),
            "IUG" => Some(Unit::InternationalUnitPerGram),
            "IV" => Some(Unit::InchPerSecondSquared),
            "J10" => Some(Unit::PercentPerMillimetre),
            "J12" => Some(Unit::PerMillePerPsi),
            "J13" => Some(Unit::DegreeApi),
            "J14" => Some(Unit::DegreeBaumeOriginScale),
            "J15" => Some(Unit::DegreeBaumeUsHeavy),
            "J16" => Some(Unit::DegreeBaumeUsLight),
            "J17" => Some(Unit::DegreeBalling),
            "J18" => Some(Unit::DegreeBrix),
            "J19" => Some(Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitThermochemical),
            "J2" => Some(Unit::JoulePerKilogram),
            "J20" => Some(Unit::DegreeFahrenheitPerKelvin),
            "J21" => Some(Unit::DegreeFahrenheitPerBar),
            "J22" => {
                Some(Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitInternationalTable)
            }
            "J23" => Some(Unit::DegreeFahrenheitPerHour),
            "J24" => Some(Unit::DegreeFahrenheitPerMinute),
            "J25" => Some(Unit::DegreeFahrenheitPerSecond),
            "J26" => Some(Unit::ReciprocalDegreeFahrenheit),
            "J27" => Some(Unit::DegreeOechsle),
            "J28" => Some(Unit::DegreeRankinePerHour),
            "J29" => Some(Unit::DegreeRankinePerMinute),
            "J30" => Some(Unit::DegreeRankinePerSecond),
            "J31" => Some(Unit::DegreeTwaddell),
            "J32" => Some(Unit::Micropoise),
            "J33" => Some(Unit::MicrogramPerKilogram),
            "J34" => Some(Unit::MicrogramPerCubicMetreKelvin),
            "J35" => Some(Unit::MicrogramPerCubicMetreBar),
            "J36" => Some(Unit::MicrolitrePerLitre),
            "J38" => Some(Unit::Baud),
            "J39" => Some(Unit::BritishThermalUnitMean),
            "J40" => Some(
                Unit::BritishThermalUnitInternationalTableFootPerHourSquareFootDegreeFahrenheit,
            ),
            "J41" => Some(
                Unit::BritishThermalUnitInternationalTableInchPerHourSquareFootDegreeFahrenheit,
            ),
            "J42" => Some(
                Unit::BritishThermalUnitInternationalTableInchPerSecondSquareFootDegreeFahrenheit,
            ),
            "J43" => Some(Unit::BritishThermalUnitInternationalTablePerPoundDegreeFahrenheit),
            "J44" => Some(Unit::BritishThermalUnitInternationalTablePerMinute),
            "J45" => Some(Unit::BritishThermalUnitInternationalTablePerSecond),
            "J46" => {
                Some(Unit::BritishThermalUnitThermochemicalFootPerHourSquareFootDegreeFahrenheit)
            }
            "J47" => Some(Unit::BritishThermalUnitThermochemicalPerHour),
            "J48" => {
                Some(Unit::BritishThermalUnitThermochemicalInchPerHourSquareFootDegreeFahrenheit)
            }
            "J49" => {
                Some(Unit::BritishThermalUnitThermochemicalInchPerSecondSquareFootDegreeFahrenheit)
            }
            "J50" => Some(Unit::BritishThermalUnitThermochemicalPerPoundDegreeFahrenheit),
            "J51" => Some(Unit::BritishThermalUnitThermochemicalPerMinute),
            "J52" => Some(Unit::BritishThermalUnitThermochemicalPerSecond),
            "J53" => Some(Unit::CoulombSquareMetrePerKilogram),
            "J54" => Some(Unit::Megabaud),
            "J55" => Some(Unit::WattSecond),
            "J56" => Some(Unit::BarPerBar),
            "J57" => Some(Unit::BarrelUkPetroleum),
            "J58" => Some(Unit::BarrelUkPetroleumPerMinute),
            "J59" => Some(Unit::BarrelUkPetroleumPerDay),
            "J60" => Some(Unit::BarrelUkPetroleumPerHour),
            "J61" => Some(Unit::BarrelUkPetroleumPerSecond),
            "J62" => Some(Unit::BarrelUsPetroleumPerHour),
            "J63" => Some(Unit::BarrelUsPetroleumPerSecond),
            "J64" => Some(Unit::BushelUkPerDay),
            "J65" => Some(Unit::BushelUkPerHour),
            "J66" => Some(Unit::BushelUkPerMinute),
            "J67" => Some(Unit::BushelUkPerSecond),
            "J68" => Some(Unit::BushelUsDryPerDay),
            "J69" => Some(Unit::BushelUsDryPerHour),
            "J70" => Some(Unit::BushelUsDryPerMinute),
            "J71" => Some(Unit::BushelUsDryPerSecond),
            "J72" => Some(Unit::CentinewtonMetre),
            "J73" => Some(Unit::CentipoisePerKelvin),
            "J74" => Some(Unit::CentipoisePerBar),
            "J75" => Some(Unit::CalorieMean),
            "J76" => Some(Unit::CalorieInternationalTablePerGramDegreeCelsius),
            "J78" => Some(Unit::CalorieThermochemicalPerCentimetreSecondDegreeCelsius),
            "J79" => Some(Unit::CalorieThermochemicalPerGramDegreeCelsius),
            "J81" => Some(Unit::CalorieThermochemicalPerMinute),
            "J82" => Some(Unit::CalorieThermochemicalPerSecond),
            "J83" => Some(Unit::Clo),
            "J84" => Some(Unit::CentimetrePerSecondKelvin),
            "J85" => Some(Unit::CentimetrePerSecondBar),
            "J87" => Some(Unit::CubicCentimetrePerCubicMetre),
            "J90" => Some(Unit::CubicDecimetrePerDay),
            "J91" => Some(Unit::CubicDecimetrePerCubicMetre),
            "J92" => Some(Unit::CubicDecimetrePerMinute),
            "J93" => Some(Unit::CubicDecimetrePerSecond),
            "J95" => Some(Unit::OunceUkFluidPerDay),
            "J96" => Some(Unit::OunceUkFluidPerHour),
            "J97" => Some(Unit::OunceUkFluidPerMinute),
            "J98" => Some(Unit::OunceUkFluidPerSecond),
            "J99" => Some(Unit::OunceUsFluidPerDay),
            "JE" => Some(Unit::JoulePerKelvin),
            "JK" => Some(Unit::MegajoulePerKilogram),
            "JM" => Some(Unit::MegajoulePerCubicMetre),
            "JNT" => Some(Unit::PipelineJoint),
            "JOU" => Some(Unit::Joule),
            "JPS" => Some(Unit::HundredMetre),
            "JWL" => Some(Unit::NumberJewels),
            "K1" => Some(Unit::KilowattDemand),
            "K10" => Some(Unit::OunceUsFluidPerHour),
            "K11" => Some(Unit::OunceUsFluidPerMinute),
            "K12" => Some(Unit::OunceUsFluidPerSecond),
            "K13" => Some(Unit::FootPerDegreeFahrenheit),
            "K14" => Some(Unit::FootPerHour),
            "K15" => Some(Unit::FootPoundForcePerHour),
            "K16" => Some(Unit::FootPoundForcePerMinute),
            "K17" => Some(Unit::FootPerPsi),
            "K18" => Some(Unit::FootPerSecondDegreeFahrenheit),
            "K19" => Some(Unit::FootPerSecondPsi),
            "K2" => Some(Unit::KilovoltAmpereReactiveDemand),
            "K20" => Some(Unit::ReciprocalCubicFoot),
            "K21" => Some(Unit::CubicFootPerDegreeFahrenheit),
            "K22" => Some(Unit::CubicFootPerDay),
            "K23" => Some(Unit::CubicFootPerPsi),
            "K26" => Some(Unit::GallonUkPerDay),
            "K27" => Some(Unit::GallonUkPerHour),
            "K28" => Some(Unit::GallonUkPerSecond),
            "K3" => Some(Unit::KilovoltAmpereReactiveHour),
            "K30" => Some(Unit::GallonUsLiquidPerSecond),
            "K31" => Some(Unit::GramForcePerSquareCentimetre),
            "K32" => Some(Unit::GillUkPerDay),
            "K33" => Some(Unit::GillUkPerHour),
            "K34" => Some(Unit::GillUkPerMinute),
            "K35" => Some(Unit::GillUkPerSecond),
            "K36" => Some(Unit::GillUsPerDay),
            "K37" => Some(Unit::GillUsPerHour),
            "K38" => Some(Unit::GillUsPerMinute),
            "K39" => Some(Unit::GillUsPerSecond),
            "K40" => Some(Unit::StandardAccelerationFreeFall),
            "K41" => Some(Unit::GrainPerGallonUs),
            "K42" => Some(Unit::HorsepowerBoiler),
            "K43" => Some(Unit::HorsepowerElectric),
            "K45" => Some(Unit::InchPerDegreeFahrenheit),
            "K46" => Some(Unit::InchPerPsi),
            "K47" => Some(Unit::InchPerSecondDegreeFahrenheit),
            "K48" => Some(Unit::InchPerSecondPsi),
            "K49" => Some(Unit::ReciprocalCubicInch),
            "K50" => Some(Unit::Kilobaud),
            "K51" => Some(Unit::KilocalorieMean),
            "K52" => Some(Unit::KilocalorieInternationalTablePerHourMetreDegreeCelsius),
            "K53" => Some(Unit::KilocalorieThermochemical),
            "K54" => Some(Unit::KilocalorieThermochemicalPerMinute),
            "K55" => Some(Unit::KilocalorieThermochemicalPerSecond),
            "K58" => Some(Unit::KilomolePerHour),
            "K59" => Some(Unit::KilomolePerCubicMetreKelvin),
            "K6" => Some(Unit::Kilolitre),
            "K60" => Some(Unit::KilomolePerCubicMetreBar),
            "K61" => Some(Unit::KilomolePerMinute),
            "K62" => Some(Unit::LitrePerLitre),
            "K63" => Some(Unit::ReciprocalLitre),
            "K64" => Some(Unit::PoundAvoirdupoisPerDegreeFahrenheit),
            "K65" => Some(Unit::PoundAvoirdupoisSquareFoot),
            "K66" => Some(Unit::PoundAvoirdupoisPerDay),
            "K67" => Some(Unit::PoundPerFootHour),
            "K68" => Some(Unit::PoundPerFootSecond),
            "K69" => Some(Unit::PoundAvoirdupoisPerCubicFootDegreeFahrenheit),
            "K70" => Some(Unit::PoundAvoirdupoisPerCubicFootPsi),
            "K71" => Some(Unit::PoundAvoirdupoisPerGallonUk),
            "K73" => Some(Unit::PoundAvoirdupoisPerHourDegreeFahrenheit),
            "K74" => Some(Unit::PoundAvoirdupoisPerHourPsi),
            "K75" => Some(Unit::PoundAvoirdupoisPerCubicInchDegreeFahrenheit),
            "K76" => Some(Unit::PoundAvoirdupoisPerCubicInchPsi),
            "K77" => Some(Unit::PoundAvoirdupoisPerPsi),
            "K78" => Some(Unit::PoundAvoirdupoisPerMinute),
            "K79" => Some(Unit::PoundAvoirdupoisPerMinuteDegreeFahrenheit),
            "K80" => Some(Unit::PoundAvoirdupoisPerMinutePsi),
            "K81" => Some(Unit::PoundAvoirdupoisPerSecond),
            "K82" => Some(Unit::PoundAvoirdupoisPerSecondDegreeFahrenheit),
            "K83" => Some(Unit::PoundAvoirdupoisPerSecondPsi),
            "K84" => Some(Unit::PoundPerCubicYard),
            "K85" => Some(Unit::PoundForcePerSquareFoot),
            "K86" => Some(Unit::PoundForcePerSquareInchDegreeFahrenheit),
            "K87" => Some(Unit::PsiCubicInchPerSecond),
            "K88" => Some(Unit::PsiLitrePerSecond),
            "K89" => Some(Unit::PsiCubicMetrePerSecond),
            "K90" => Some(Unit::PsiCubicYardPerSecond),
            "K91" => Some(Unit::PoundForceSecondPerSquareFoot),
            "K92" => Some(Unit::PoundForceSecondPerSquareInch),
            "K93" => Some(Unit::ReciprocalPsi),
            "K94" => Some(Unit::QuartUkLiquidPerDay),
            "K95" => Some(Unit::QuartUkLiquidPerHour),
            "K96" => Some(Unit::QuartUkLiquidPerMinute),
            "K97" => Some(Unit::QuartUkLiquidPerSecond),
            "K98" => Some(Unit::QuartUsLiquidPerDay),
            "K99" => Some(Unit::QuartUsLiquidPerHour),
            "KA" => Some(Unit::Cake),
            "KAT" => Some(Unit::Katal),
            "KB" => Some(Unit::Kilocharacter),
            "KBA" => Some(Unit::Kilobar),
            "KCC" => Some(Unit::KilogramCholineChloride),
            "KDW" => Some(Unit::KilogramDrainedNetWeight),
            "KEL" => Some(Unit::Kelvin),
            "KGM" => Some(Unit::Kilogram),
            "KGS" => Some(Unit::KilogramPerSecond),
            "KHY" => Some(Unit::KilogramHydrogenPeroxide),
            "KHZ" => Some(Unit::Kilohertz),
            "KI" => Some(Unit::KilogramPerMillimetreWidth),
            "KIC" => Some(Unit::KilogramIncludingContainer),
            "KIP" => Some(Unit::KilogramIncludingInnerPackaging),
            "KJ" => Some(Unit::Kilosegment),
            "KJO" => Some(Unit::Kilojoule),
            "KL" => Some(Unit::KilogramPerMetre),
            "KLK" => Some(Unit::LacticDryMaterialPercentage),
            "KLX" => Some(Unit::Kilolux),
            "KMA" => Some(Unit::KilogramMethylamine),
            "KMH" => Some(Unit::KilometrePerHour),
            "KMK" => Some(Unit::SquareKilometre),
            "KMQ" => Some(Unit::KilogramPerCubicMetre),
            "KMT" => Some(Unit::Kilometre),
            "KNI" => Some(Unit::KilogramNitrogen),
            "KNM" => Some(Unit::KilonewtonPerSquareMetre),
            "KNS" => Some(Unit::KilogramNamedSubstance),
            "KNT" => Some(Unit::Knot),
            "KO" => Some(Unit::MilliequivalenceCausticPotashPerGramProduct),
            "KPA" => Some(Unit::Kilopascal),
            "KPH" => Some(Unit::KilogramPotassiumHydroxideCausticPotash),
            "KPO" => Some(Unit::KilogramPotassiumOxide),
            "KPP" => Some(Unit::KilogramPhosphorusPentoxidePhosphoricAnhydride),
            "KR" => Some(Unit::Kiloroentgen),
            "KSD" => Some(Unit::KilogramSubstance90Dry),
            "KSH" => Some(Unit::KilogramSodiumHydroxideCausticSoda),
            "KT" => Some(Unit::Kit),
            "KTN" => Some(Unit::Kilotonne),
            "KUR" => Some(Unit::KilogramUranium),
            "KVA" => Some(Unit::KilovoltAmpere),
            "KVR" => Some(Unit::Kilovar),
            "KVT" => Some(Unit::Kilovolt),
            "KW" => Some(Unit::KilogramPerMillimetre),
            "KWH" => Some(Unit::KilowattHour),
            "KWN" => Some(Unit::KilowattHourPerNormalizedCubicMetre),
            "KWO" => Some(Unit::KilogramTungstenTrioxide),
            "KWS" => Some(Unit::KilowattHourPerStandardCubicMetre),
            "KWT" => Some(Unit::Kilowatt),
            "KWY" => Some(Unit::KilowattYear),
            "KX" => Some(Unit::MillilitrePerKilogram),
            "L10" => Some(Unit::QuartUsLiquidPerMinute),
            "L11" => Some(Unit::QuartUsLiquidPerSecond),
            "L12" => Some(Unit::MetrePerSecondKelvin),
            "L13" => Some(Unit::MetrePerSecondBar),
            "L14" => Some(Unit::SquareMetreHourDegreeCelsiusPerKilocalorieInternationalTable),
            "L15" => Some(Unit::MillipascalSecondPerKelvin),
            "L16" => Some(Unit::MillipascalSecondPerBar),
            "L17" => Some(Unit::MilligramPerCubicMetreKelvin),
            "L18" => Some(Unit::MilligramPerCubicMetreBar),
            "L19" => Some(Unit::MillilitrePerLitre),
            "L2" => Some(Unit::LitrePerMinute),
            "L20" => Some(Unit::ReciprocalCubicMillimetre),
            "L21" => Some(Unit::CubicMillimetrePerCubicMetre),
            "L23" => Some(Unit::MolePerHour),
            "L24" => Some(Unit::MolePerKilogramKelvin),
            "L25" => Some(Unit::MolePerKilogramBar),
            "L26" => Some(Unit::MolePerLitreKelvin),
            "L27" => Some(Unit::MolePerLitreBar),
            "L28" => Some(Unit::MolePerCubicMetreKelvin),
            "L29" => Some(Unit::MolePerCubicMetreBar),
            "L30" => Some(Unit::MolePerMinute),
            "L31" => Some(Unit::MilliroentgenAequivalentMen),
            "L32" => Some(Unit::NanogramPerKilogram),
            "L33" => Some(Unit::OunceAvoirdupoisPerDay),
            "L34" => Some(Unit::OunceAvoirdupoisPerHour),
            "L35" => Some(Unit::OunceAvoirdupoisPerMinute),
            "L36" => Some(Unit::OunceAvoirdupoisPerSecond),
            "L37" => Some(Unit::OunceAvoirdupoisPerGallonUk),
            "L38" => Some(Unit::OunceAvoirdupoisPerGallonUs),
            "L39" => Some(Unit::OunceAvoirdupoisPerCubicInch),
            "L40" => Some(Unit::OunceAvoirdupoisForce),
            "L41" => Some(Unit::OunceAvoirdupoisForceInch),
            "L42" => Some(Unit::PicosiemensPerMetre),
            "L43" => Some(Unit::PeckUk),
            "L44" => Some(Unit::PeckUkPerDay),
            "L45" => Some(Unit::PeckUkPerHour),
            "L46" => Some(Unit::PeckUkPerMinute),
            "L47" => Some(Unit::PeckUkPerSecond),
            "L48" => Some(Unit::PeckUsDryPerDay),
            "L49" => Some(Unit::PeckUsDryPerHour),
            "L50" => Some(Unit::PeckUsDryPerMinute),
            "L51" => Some(Unit::PeckUsDryPerSecond),
            "L52" => Some(Unit::PsiPerPsi),
            "L53" => Some(Unit::PintUkPerDay),
            "L54" => Some(Unit::PintUkPerHour),
            "L55" => Some(Unit::PintUkPerMinute),
            "L56" => Some(Unit::PintUkPerSecond),
            "L57" => Some(Unit::PintUsLiquidPerDay),
            "L58" => Some(Unit::PintUsLiquidPerHour),
            "L59" => Some(Unit::PintUsLiquidPerMinute),
            "L60" => Some(Unit::PintUsLiquidPerSecond),
            "L63" => Some(Unit::SlugPerDay),
            "L64" => Some(Unit::SlugPerFootSecond),
            "L65" => Some(Unit::SlugPerCubicFoot),
            "L66" => Some(Unit::SlugPerHour),
            "L67" => Some(Unit::SlugPerMinute),
            "L68" => Some(Unit::SlugPerSecond),
            "L69" => Some(Unit::TonnePerKelvin),
            "L70" => Some(Unit::TonnePerBar),
            "L71" => Some(Unit::TonnePerDay),
            "L72" => Some(Unit::TonnePerDayKelvin),
            "L73" => Some(Unit::TonnePerDayBar),
            "L74" => Some(Unit::TonnePerHourKelvin),
            "L75" => Some(Unit::TonnePerHourBar),
            "L76" => Some(Unit::TonnePerCubicMetreKelvin),
            "L77" => Some(Unit::TonnePerCubicMetreBar),
            "L78" => Some(Unit::TonnePerMinute),
            "L79" => Some(Unit::TonnePerMinuteKelvin),
            "L80" => Some(Unit::TonnePerMinuteBar),
            "L81" => Some(Unit::TonnePerSecond),
            "L82" => Some(Unit::TonnePerSecondKelvin),
            "L83" => Some(Unit::TonnePerSecondBar),
            "L84" => Some(Unit::TonUkShipping),
            "L85" => Some(Unit::TonLongPerDay),
            "L86" => Some(Unit::TonUsShipping),
            "L87" => Some(Unit::TonShortPerDegreeFahrenheit),
            "L88" => Some(Unit::TonShortPerDay),
            "L89" => Some(Unit::TonShortPerHourDegreeFahrenheit),
            "L90" => Some(Unit::TonShortPerHourPsi),
            "L91" => Some(Unit::TonShortPerPsi),
            "L92" => Some(Unit::TonUkLongPerCubicYard),
            "L93" => Some(Unit::TonUsShortPerCubicYard),
            "L94" => Some(Unit::TonForceUsShort),
            "L95" => Some(Unit::CommonYear),
            "L96" => Some(Unit::SiderealYear),
            "L98" => Some(Unit::YardPerDegreeFahrenheit),
            "L99" => Some(Unit::YardPerPsi),
            "LA" => Some(Unit::PoundPerCubicInch),
            "LAC" => Some(Unit::LactoseExcessPercentage),
            "LBR" => Some(Unit::Pound),
            "LBT" => Some(Unit::TroyPoundUs),
            "LD" => Some(Unit::LitrePerDay),
            "LEF" => Some(Unit::Leaf),
            "LF" => Some(Unit::LinearFoot),
            "LH" => Some(Unit::LabourHour),
            "LK" => Some(Unit::Link),
            "LM" => Some(Unit::LinearMetre),
            "LN" => Some(Unit::Length),
            "LO" => Some(Unit::LotUnitProcurement),
            "LP" => Some(Unit::LiquidPound),
            "LPA" => Some(Unit::LitrePureAlcohol),
            "LR" => Some(Unit::Layer),
            "LS" => Some(Unit::LumpSum),
            "LTN" => Some(Unit::TonUkOrLongTonUs),
            "LTR" => Some(Unit::Litre),
            "LUB" => Some(Unit::MetricTonLubricatingOil),
            "LUM" => Some(Unit::Lumen),
            "LUX" => Some(Unit::Lux),
            "LY" => Some(Unit::LinearYard),
            "M1" => Some(Unit::MilligramPerLitre),
            "M10" => Some(Unit::ReciprocalCubicYard),
            "M11" => Some(Unit::CubicYardPerDegreeFahrenheit),
            "M12" => Some(Unit::CubicYardPerDay),
            "M13" => Some(Unit::CubicYardPerHour),
            "M14" => Some(Unit::CubicYardPerPsi),
            "M15" => Some(Unit::CubicYardPerMinute),
            "M16" => Some(Unit::CubicYardPerSecond),
            "M17" => Some(Unit::KilohertzMetre),
            "M18" => Some(Unit::GigahertzMetre),
            "M19" => Some(Unit::Beaufort),
            "M20" => Some(Unit::ReciprocalMegakelvinOrMegakelvinToPowerMinusOne),
            "M21" => Some(Unit::ReciprocalKilovoltAmpereReciprocalHour),
            "M22" => Some(Unit::MillilitrePerSquareCentimetreMinute),
            "M23" => Some(Unit::NewtonPerCentimetre),
            "M24" => Some(Unit::OhmKilometre),
            "M25" => Some(Unit::PercentPerDegreeCelsius),
            "M26" => Some(Unit::GigaohmPerMetre),
            "M27" => Some(Unit::MegahertzMetre),
            "M29" => Some(Unit::KilogramPerKilogram),
            "M30" => Some(Unit::ReciprocalVoltAmpereReciprocalSecond),
            "M31" => Some(Unit::KilogramPerKilometre),
            "M32" => Some(Unit::PascalSecondPerLitre),
            "M33" => Some(Unit::MillimolePerLitre),
            "M34" => Some(Unit::NewtonMetrePerSquareMetre),
            "M35" => Some(Unit::MillivoltAmpere),
            "M36" => Some(Unit::_30DayMonth),
            "M37" => Some(Unit::Actual360),
            "M38" => Some(Unit::KilometrePerSecondSquared),
            "M39" => Some(Unit::CentimetrePerSecondSquared),
            "M4" => Some(Unit::MonetaryValue),
            "M40" => Some(Unit::YardPerSecondSquared),
            "M41" => Some(Unit::MillimetrePerSecondSquared),
            "M42" => Some(Unit::MileStatuteMilePerSecondSquared),
            "M43" => Some(Unit::Mil),
            "M44" => Some(Unit::Revolution),
            "M45" => Some(Unit::DegreeUnitAnglePerSecondSquared),
            "M46" => Some(Unit::RevolutionPerMinute),
            "M47" => Some(Unit::CircularMil),
            "M48" => Some(Unit::SquareMileBasedOnUSSurveyFoot),
            "M49" => Some(Unit::ChainBasedOnUSSurveyFoot),
            "M5" => Some(Unit::Microcurie),
            "M50" => Some(Unit::Furlong),
            "M51" => Some(Unit::FootUSSurvey),
            "M52" => Some(Unit::MileBasedOnUSSurveyFoot),
            "M53" => Some(Unit::MetrePerPascal),
            "M55" => Some(Unit::MetrePerRadiant),
            "M56" => Some(Unit::Shake),
            "M57" => Some(Unit::MilePerMinute),
            "M58" => Some(Unit::MilePerSecond),
            "M59" => Some(Unit::MetrePerSecondPascal),
            "M60" => Some(Unit::MetrePerHour),
            "M61" => Some(Unit::InchPerYear),
            "M62" => Some(Unit::KilometrePerSecond),
            "M63" => Some(Unit::InchPerMinute),
            "M64" => Some(Unit::YardPerSecond),
            "M65" => Some(Unit::YardPerMinute),
            "M66" => Some(Unit::YardPerHour),
            "M67" => Some(Unit::AcreFootBasedOnUSSurveyFoot),
            "M68" => Some(Unit::Cord128Ft3),
            "M69" => Some(Unit::CubicMileUkStatute),
            "M7" => Some(Unit::MicroInch),
            "M70" => Some(Unit::TonRegister),
            "M71" => Some(Unit::CubicMetrePerPascal),
            "M72" => Some(Unit::Bel),
            "M73" => Some(Unit::KilogramPerCubicMetrePascal),
            "M74" => Some(Unit::KilogramPerPascal),
            "M75" => Some(Unit::KilopoundForce),
            "M76" => Some(Unit::Poundal),
            "M77" => Some(Unit::KilogramMetrePerSecondSquared),
            "M78" => Some(Unit::Pond),
            "M79" => Some(Unit::SquareFootPerHour),
            "M80" => Some(Unit::StokesPerPascal),
            "M81" => Some(Unit::SquareCentimetrePerSecond),
            "M82" => Some(Unit::SquareMetrePerSecondPascal),
            "M83" => Some(Unit::Denier_Dup),
            "M84" => Some(Unit::PoundPerYard),
            "M85" => Some(Unit::TonAssay),
            "M86" => Some(Unit::Pfund),
            "M87" => Some(Unit::KilogramPerSecondPascal),
            "M88" => Some(Unit::TonnePerMonth),
            "M89" => Some(Unit::TonnePerYear),
            "M9" => Some(Unit::MillionBtuPer1000CubicFoot),
            "M90" => Some(Unit::KilopoundPerHour),
            "M91" => Some(Unit::PoundPerPound),
            "M92" => Some(Unit::PoundForceFoot),
            "M93" => Some(Unit::NewtonMetrePerRadian),
            "M94" => Some(Unit::KilogramMetre),
            "M95" => Some(Unit::PoundalFoot),
            "M96" => Some(Unit::PoundalInch),
            "M97" => Some(Unit::DyneMetre),
            "M98" => Some(Unit::KilogramCentimetrePerSecond),
            "M99" => Some(Unit::GramCentimetrePerSecond),
            "MAH" => Some(Unit::MegavoltAmpereReactiveHour),
            "MAL" => Some(Unit::Megalitre),
            "MAM" => Some(Unit::Megametre),
            "MAR" => Some(Unit::Megavar),
            "MAW" => Some(Unit::Megawatt),
            "MBE" => Some(Unit::ThousandStandardBrickEquivalent),
            "MBF" => Some(Unit::ThousandBoardFoot),
            "MBR" => Some(Unit::Millibar),
            "MC" => Some(Unit::Microgram),
            "MCU" => Some(Unit::Millicurie),
            "MD" => Some(Unit::AirDryMetricTon),
            "MGM" => Some(Unit::Milligram),
            "MHZ" => Some(Unit::Megahertz),
            "MIK" => Some(Unit::SquareMileStatuteMile),
            "MIL" => Some(Unit::Thousand),
            "MIN" => Some(Unit::MinuteUnitTime),
            "MIO" => Some(Unit::Million),
            "MIU" => Some(Unit::MillionInternationalUnit),
            "MKD" => Some(Unit::SquareMetreDay),
            "MKM" => Some(Unit::SquareMetreMonth),
            "MKW" => Some(Unit::SquareMetreWeek),
            "MLD" => Some(Unit::Milliard),
            "MLT" => Some(Unit::Millilitre),
            "MMK" => Some(Unit::SquareMillimetre),
            "MMQ" => Some(Unit::CubicMillimetre),
            "MMT" => Some(Unit::Millimetre),
            "MND" => Some(Unit::KilogramDryWeight),
            "MNJ" => Some(Unit::MegaJoulePerNormalisedCubicMetre),
            "MON" => Some(Unit::Month),
            "MPA" => Some(Unit::Megapascal),
            "MQD" => Some(Unit::CubicMetreDay),
            "MQH" => Some(Unit::CubicMetrePerHour),
            "MQM" => Some(Unit::CubicMetreMonth),
            "MQS" => Some(Unit::CubicMetrePerSecond),
            "MQW" => Some(Unit::CubicMetreWeek),
            "MRD" => Some(Unit::MetreDay),
            "MRM" => Some(Unit::MetreMonth),
            "MRW" => Some(Unit::MetreWeek),
            "MSK" => Some(Unit::MetrePerSecondSquared),
            "MTK" => Some(Unit::SquareMetre),
            "MTQ" => Some(Unit::CubicMetre),
            "MTR" => Some(Unit::Metre),
            "MTS" => Some(Unit::MetrePerSecond),
            "MTZ" => Some(Unit::Milihertz),
            "MVA" => Some(Unit::MegavoltAmpere),
            "MWH" => Some(Unit::MegawattHour1000KwH),
            "N1" => Some(Unit::PenCalorie),
            "N10" => Some(Unit::PoundFootPerSecond),
            "N11" => Some(Unit::PoundInchPerSecond),
            "N12" => Some(Unit::Pferdestaerke),
            "N13" => Some(Unit::CentimetreMercury0Oc),
            "N14" => Some(Unit::CentimetreWater4Oc),
            "N15" => Some(Unit::FootWater392Of),
            "N16" => Some(Unit::InchMercury32Of),
            "N17" => Some(Unit::InchMercury60Of),
            "N18" => Some(Unit::InchWater392Of),
            "N19" => Some(Unit::InchWater60Of),
            "N20" => Some(Unit::KipPerSquareInch),
            "N21" => Some(Unit::PoundalPerSquareFoot),
            "N22" => Some(Unit::OunceAvoirdupoisPerSquareInch),
            "N23" => Some(Unit::ConventionalMetreWater),
            "N24" => Some(Unit::GramPerSquareMillimetre),
            "N25" => Some(Unit::PoundPerSquareYard),
            "N26" => Some(Unit::PoundalPerSquareInch),
            "N27" => Some(Unit::FootToFourthPower),
            "N28" => Some(Unit::CubicDecimetrePerKilogram),
            "N29" => Some(Unit::CubicFootPerPound),
            "N3" => Some(Unit::PrintPoint),
            "N30" => Some(Unit::CubicInchPerPound),
            "N31" => Some(Unit::KilonewtonPerMetre),
            "N32" => Some(Unit::PoundalPerInch),
            "N33" => Some(Unit::PoundForcePerYard),
            "N34" => Some(Unit::PoundalSecondPerSquareFoot),
            "N35" => Some(Unit::PoisePerPascal),
            "N36" => Some(Unit::NewtonSecondPerSquareMetre),
            "N37" => Some(Unit::KilogramPerMetreSecond),
            "N38" => Some(Unit::KilogramPerMetreMinute),
            "N39" => Some(Unit::KilogramPerMetreDay),
            "N40" => Some(Unit::KilogramPerMetreHour),
            "N41" => Some(Unit::GramPerCentimetreSecond),
            "N42" => Some(Unit::PoundalSecondPerSquareInch),
            "N43" => Some(Unit::PoundPerFootMinute),
            "N44" => Some(Unit::PoundPerFootDay),
            "N45" => Some(Unit::CubicMetrePerSecondPascal),
            "N46" => Some(Unit::FootPoundal),
            "N47" => Some(Unit::InchPoundal),
            "N48" => Some(Unit::WattPerSquareCentimetre),
            "N49" => Some(Unit::WattPerSquareInch),
            "N50" => Some(Unit::BritishThermalUnitInternationalTablePerSquareFootHour),
            "N51" => Some(Unit::BritishThermalUnitThermochemicalPerSquareFootHour),
            "N52" => Some(Unit::BritishThermalUnitThermochemicalPerSquareFootMinute),
            "N53" => Some(Unit::BritishThermalUnitInternationalTablePerSquareFootSecond),
            "N54" => Some(Unit::BritishThermalUnitThermochemicalPerSquareFootSecond),
            "N55" => Some(Unit::BritishThermalUnitInternationalTablePerSquareInchSecond),
            "N56" => Some(Unit::CalorieThermochemicalPerSquareCentimetreMinute),
            "N57" => Some(Unit::CalorieThermochemicalPerSquareCentimetreSecond),
            "N58" => Some(Unit::BritishThermalUnitInternationalTablePerCubicFoot),
            "N59" => Some(Unit::BritishThermalUnitThermochemicalPerCubicFoot),
            "N60" => Some(Unit::BritishThermalUnitInternationalTablePerDegreeFahrenheit),
            "N61" => Some(Unit::BritishThermalUnitThermochemicalPerDegreeFahrenheit),
            "N62" => Some(Unit::BritishThermalUnitInternationalTablePerDegreeRankine),
            "N63" => Some(Unit::BritishThermalUnitThermochemicalPerDegreeRankine),
            "N64" => Some(Unit::BritishThermalUnitThermochemicalPerPoundDegreeRankine),
            "N65" => Some(Unit::KilocalorieInternationalTablePerGramKelvin),
            "N66" => Some(Unit::BritishThermalUnit39Of),
            "N67" => Some(Unit::BritishThermalUnit59Of),
            "N68" => Some(Unit::BritishThermalUnit60Of),
            "N69" => Some(Unit::Calorie20Oc),
            "N70" => Some(Unit::Quad1015Btuit),
            "N71" => Some(Unit::ThermEc),
            "N72" => Some(Unit::ThermUS),
            "N73" => Some(Unit::BritishThermalUnitThermochemicalPerPound),
            "N74" => {
                Some(Unit::BritishThermalUnitInternationalTablePerHourSquareFootDegreeFahrenheit)
            }
            "N75" => Some(Unit::BritishThermalUnitThermochemicalPerHourSquareFootDegreeFahrenheit),
            "N76" => {
                Some(Unit::BritishThermalUnitInternationalTablePerSecondSquareFootDegreeFahrenheit)
            }
            "N77" => {
                Some(Unit::BritishThermalUnitThermochemicalPerSecondSquareFootDegreeFahrenheit)
            }
            "N78" => Some(Unit::KilowattPerSquareMetreKelvin),
            "N79" => Some(Unit::KelvinPerPascal),
            "N80" => Some(Unit::WattPerMetreDegreeCelsius),
            "N81" => Some(Unit::KilowattPerMetreKelvin),
            "N82" => Some(Unit::KilowattPerMetreDegreeCelsius),
            "N83" => Some(Unit::MetrePerDegreeCelciusMetre),
            "N84" => Some(Unit::DegreeFahrenheitHourPerBritishThermalUnitInternationalTable),
            "N85" => Some(Unit::DegreeFahrenheitHourPerBritishThermalUnitThermochemical),
            "N86" => Some(Unit::DegreeFahrenheitSecondPerBritishThermalUnitInternationalTable),
            "N87" => Some(Unit::DegreeFahrenheitSecondPerBritishThermalUnitThermochemical),
            "N88" => Some(
                Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitInternationalTableInch,
            ),
            "N89" => {
                Some(Unit::DegreeFahrenheitHourSquareFootPerBritishThermalUnitThermochemicalInch)
            }
            "N90" => Some(Unit::Kilofarad),
            "N91" => Some(Unit::ReciprocalJoule),
            "N92" => Some(Unit::Picosiemens),
            "N93" => Some(Unit::AmperePerPascal),
            "N94" => Some(Unit::Franklin),
            "N95" => Some(Unit::AmpereMinute),
            "N96" => Some(Unit::Biot),
            "N97" => Some(Unit::Gilbert),
            "N98" => Some(Unit::VoltPerPascal),
            "N99" => Some(Unit::Picovolt),
            "nan" => Some(Unit::MilligramPerKilogram),
            "NAR" => Some(Unit::NumberArticles),
            "NCL" => Some(Unit::NumberCells),
            "NEW" => Some(Unit::Newton),
            "NF" => Some(Unit::Message),
            "NIL" => Some(Unit::Nil),
            "NIU" => Some(Unit::NumberInternationalUnits),
            "NL" => Some(Unit::Load),
            "NM3" => Some(Unit::NormalisedCubicMetre),
            "NMI" => Some(Unit::NauticalMile),
            "NMP" => Some(Unit::NumberPacks),
            "NPT" => Some(Unit::NumberParts),
            "NT" => Some(Unit::NetTon),
            "NTU" => Some(Unit::NephelometricTurbidityUnit),
            "NU" => Some(Unit::NewtonMetre),
            "NX" => Some(Unit::PartPerThousand),
            "OA" => Some(Unit::Panel),
            "ODE" => Some(Unit::OzoneDepletionEquivalent),
            "ODG" => Some(Unit::OdsGrams),
            "ODK" => Some(Unit::OdsKilograms),
            "ODM" => Some(Unit::OdsMilligrams),
            "OHM" => Some(Unit::Ohm),
            "ON" => Some(Unit::OuncePerSquareYard),
            "ONZ" => Some(Unit::OunceAvoirdupois),
            "OPM" => Some(Unit::OscillationsPerMinute),
            "OT" => Some(Unit::OvertimeHour),
            "OZA" => Some(Unit::FluidOunceUs),
            "OZI" => Some(Unit::FluidOunceUk),
            "P1" => Some(Unit::Percent),
            "P10" => Some(Unit::CoulombPerMetre),
            "P11" => Some(Unit::Kiloweber),
            "P12" => Some(Unit::Gamma),
            "P13" => Some(Unit::Kilotesla),
            "P14" => Some(Unit::JoulePerSecond),
            "P15" => Some(Unit::JoulePerMinute),
            "P16" => Some(Unit::JoulePerHour),
            "P17" => Some(Unit::JoulePerDay),
            "P18" => Some(Unit::KilojoulePerSecond),
            "P19" => Some(Unit::KilojoulePerMinute),
            "P2" => Some(Unit::PoundPerFoot),
            "P20" => Some(Unit::KilojoulePerHour),
            "P21" => Some(Unit::KilojoulePerDay),
            "P22" => Some(Unit::Nanoohm),
            "P23" => Some(Unit::OhmCircularMilPerFoot),
            "P24" => Some(Unit::Kilohenry),
            "P25" => Some(Unit::LumenPerSquareFoot),
            "P26" => Some(Unit::Phot),
            "P27" => Some(Unit::Footcandle),
            "P28" => Some(Unit::CandelaPerSquareInch),
            "P29" => Some(Unit::Footlambert),
            "P30" => Some(Unit::Lambert),
            "P31" => Some(Unit::Stilb),
            "P32" => Some(Unit::CandelaPerSquareFoot),
            "P33" => Some(Unit::Kilocandela),
            "P34" => Some(Unit::Millicandela),
            "P35" => Some(Unit::HefnerKerze),
            "P36" => Some(Unit::InternationalCandle),
            "P37" => Some(Unit::BritishThermalUnitInternationalTablePerSquareFoot),
            "P38" => Some(Unit::BritishThermalUnitThermochemicalPerSquareFoot),
            "P39" => Some(Unit::CalorieThermochemicalPerSquareCentimetre),
            "P40" => Some(Unit::Langley),
            "P41" => Some(Unit::DecadeLogarithmic),
            "P42" => Some(Unit::PascalSquaredSecond),
            "P43" => Some(Unit::BelPerMetre),
            "P44" => Some(Unit::PoundMole),
            "P45" => Some(Unit::PoundMolePerSecond),
            "P46" => Some(Unit::PoundMolePerMinute),
            "P47" => Some(Unit::KilomolePerKilogram),
            "P48" => Some(Unit::PoundMolePerPound),
            "P49" => Some(Unit::NewtonSquareMetrePerAmpere),
            "P5" => Some(Unit::FivePack),
            "P50" => Some(Unit::WeberMetre),
            "P51" => Some(Unit::MolPerKilogramPascal),
            "P52" => Some(Unit::MolPerCubicMetrePascal),
            "P53" => Some(Unit::UnitPole),
            "P54" => Some(Unit::MilligrayPerSecond),
            "P55" => Some(Unit::MicrograyPerSecond),
            "P56" => Some(Unit::NanograyPerSecond),
            "P57" => Some(Unit::GrayPerMinute),
            "P58" => Some(Unit::MilligrayPerMinute),
            "P59" => Some(Unit::MicrograyPerMinute),
            "P60" => Some(Unit::NanograyPerMinute),
            "P61" => Some(Unit::GrayPerHour),
            "P62" => Some(Unit::MilligrayPerHour),
            "P63" => Some(Unit::MicrograyPerHour),
            "P64" => Some(Unit::NanograyPerHour),
            "P65" => Some(Unit::SievertPerSecond),
            "P66" => Some(Unit::MillisievertPerSecond),
            "P67" => Some(Unit::MicrosievertPerSecond),
            "P68" => Some(Unit::NanosievertPerSecond),
            "P69" => Some(Unit::RemPerSecond),
            "P70" => Some(Unit::SievertPerHour),
            "P71" => Some(Unit::MillisievertPerHour),
            "P72" => Some(Unit::MicrosievertPerHour),
            "P73" => Some(Unit::NanosievertPerHour),
            "P74" => Some(Unit::SievertPerMinute),
            "P75" => Some(Unit::MillisievertPerMinute),
            "P76" => Some(Unit::MicrosievertPerMinute),
            "P77" => Some(Unit::NanosievertPerMinute),
            "P78" => Some(Unit::ReciprocalSquareInch),
            "P79" => Some(Unit::PascalSquareMetrePerKilogram),
            "P80" => Some(Unit::MillipascalPerMetre),
            "P81" => Some(Unit::KilopascalPerMetre),
            "P82" => Some(Unit::HectopascalPerMetre),
            "P83" => Some(Unit::StandardAtmospherePerMetre),
            "P84" => Some(Unit::TechnicalAtmospherePerMetre),
            "P85" => Some(Unit::TorrPerMetre),
            "P86" => Some(Unit::PsiPerInch),
            "P87" => Some(Unit::CubicMetrePerSecondSquareMetre),
            "P88" => Some(Unit::Rhe),
            "P89" => Some(Unit::PoundForceFootPerInch),
            "P90" => Some(Unit::PoundForceInchPerInch),
            "P91" => Some(Unit::Perm0Oc),
            "P92" => Some(Unit::Perm23Oc),
            "P93" => Some(Unit::BytePerSecond),
            "P94" => Some(Unit::KilobytePerSecond),
            "P95" => Some(Unit::MegabytePerSecond),
            "P96" => Some(Unit::ReciprocalVolt),
            "P97" => Some(Unit::ReciprocalRadian),
            "P98" => Some(Unit::PascalToPowerSumStoichiometricNumbers),
            "P99" => Some(Unit::MolePerCubivMetreToPowerSumStoichiometricNumbers),
            "PAL" => Some(Unit::Pascal),
            "PD" => Some(Unit::Pad),
            "PFL" => Some(Unit::ProofLitre),
            "PGL" => Some(Unit::ProofGallon),
            "PI" => Some(Unit::Pitch),
            "PLA" => Some(Unit::DegreePlato),
            "PO" => Some(Unit::PoundPerInchLength),
            "PQ" => Some(Unit::PagePerInch),
            "PR" => Some(Unit::Pair),
            "PS" => Some(Unit::PoundForcePerSquareInch),
            "PTD" => Some(Unit::DryPintUs),
            "PTI" => Some(Unit::PintUk),
            "PTL" => Some(Unit::LiquidPintUs),
            "PTN" => Some(Unit::Portion),
            "Q10" => Some(Unit::JoulePerTesla),
            "Q11" => Some(Unit::Erlang),
            "Q12" => Some(Unit::Octet),
            "Q13" => Some(Unit::OctetPerSecond),
            "Q14" => Some(Unit::Shannon),
            "Q15" => Some(Unit::Hartley),
            "Q16" => Some(Unit::NaturalUnitInformation),
            "Q17" => Some(Unit::ShannonPerSecond),
            "Q18" => Some(Unit::HartleyPerSecond),
            "Q19" => Some(Unit::NaturalUnitInformationPerSecond),
            "Q20" => Some(Unit::SecondPerKilogramm),
            "Q21" => Some(Unit::WattSquareMetre),
            "Q22" => Some(Unit::SecondPerRadianCubicMetre),
            "Q23" => Some(Unit::WeberToPowerMinusOne),
            "Q24" => Some(Unit::ReciprocalInch),
            "Q25" => Some(Unit::Dioptre),
            "Q26" => Some(Unit::OnePerOne),
            "Q27" => Some(Unit::NewtonMetrePerMetre),
            "Q28" => Some(Unit::KilogramPerSquareMetrePascalSecond),
            "Q29" => Some(Unit::MicrogramPerHectogram),
            "Q3" => Some(Unit::Meal),
            "Q30" => Some(Unit::PhPotentialHydrogen),
            "Q31" => Some(Unit::KilojoulePerGram),
            "Q32" => Some(Unit::Femtolitre),
            "Q33" => Some(Unit::Picolitre),
            "Q34" => Some(Unit::Nanolitre),
            "Q35" => Some(Unit::MegawattsPerMinute),
            "Q36" => Some(Unit::SquareMetrePerCubicMetre),
            "Q37" => Some(Unit::StandardCubicMetrePerDay),
            "Q38" => Some(Unit::StandardCubicMetrePerHour),
            "Q39" => Some(Unit::NormalizedCubicMetrePerDay),
            "Q40" => Some(Unit::NormalizedCubicMetrePerHour),
            "Q41" => Some(Unit::JoulePerNormalisedCubicMetre),
            "Q42" => Some(Unit::JoulePerStandardCubicMetre),
            "QA" => Some(Unit::PageFacsimile),
            "QAN" => Some(Unit::QuarterAYear),
            "QB" => Some(Unit::PageHardcopy),
            "QR" => Some(Unit::Quire),
            "QTD" => Some(Unit::DryQuartUs),
            "QTI" => Some(Unit::QuartUk),
            "QTL" => Some(Unit::LiquidQuartUs),
            "QTR" => Some(Unit::QuarterUk),
            "R1" => Some(Unit::Pica),
            "R9" => Some(Unit::ThousandCubicMetre),
            "RH" => Some(Unit::RunningOrOperatingHour),
            "RM" => Some(Unit::Ream),
            "ROM" => Some(Unit::Room),
            "RP" => Some(Unit::PoundPerReam),
            "RPM" => Some(Unit::RevolutionsPerMinute),
            "RPS" => Some(Unit::RevolutionsPerSecond),
            "RT" => Some(Unit::RevenueTonMile),
            "S3" => Some(Unit::SquareFootPerSecond),
            "S4" => Some(Unit::SquareMetrePerSecond),
            "SAN" => Some(Unit::HalfYear6Months),
            "SCO" => Some(Unit::Score),
            "SCR" => Some(Unit::Scruple),
            "SEC" => Some(Unit::SecondUnitTime),
            "SET" => Some(Unit::Set),
            "SG" => Some(Unit::Segment),
            "SIE" => Some(Unit::Siemens),
            "SM3" => Some(Unit::StandardCubicMetre),
            "SMI" => Some(Unit::MileStatuteMile),
            "SQ" => Some(Unit::Square),
            "SQR" => Some(Unit::SquareRoofing),
            "SR" => Some(Unit::Strip),
            "STC" => Some(Unit::Stick),
            "STI" => Some(Unit::StoneUk),
            "STK" => Some(Unit::StickCigarette),
            "STL" => Some(Unit::StandardLitre),
            "STN" => Some(Unit::TonUsOrShortTonUkUs),
            "STW" => Some(Unit::Straw),
            "SW" => Some(Unit::Skein),
            "SX" => Some(Unit::Shipment),
            "SYR" => Some(Unit::Syringe),
            "T0" => Some(Unit::TelecommunicationLineInService),
            "T3" => Some(Unit::ThousandPiece),
            "TAH" => Some(Unit::KiloampereHourThousandAmpereHour),
            "TAN" => Some(Unit::TotalAcidNumber),
            "TI" => Some(Unit::ThousandSquareInch),
            "TIC" => Some(Unit::MetricTonIncludingContainer),
            "TIP" => Some(Unit::MetricTonIncludingInnerPackaging),
            "TKM" => Some(Unit::TonneKilometre),
            "TMS" => Some(Unit::KilogramImportedMeatLessOffal),
            "TNE" => Some(Unit::TonneMetricTon),
            "TP" => Some(Unit::TenPack),
            "TPI" => Some(Unit::TeethPerInch),
            "TPR" => Some(Unit::TenPair),
            "TQD" => Some(Unit::ThousandCubicMetrePerDay),
            "TRL" => Some(Unit::TrillionEur),
            "TST" => Some(Unit::TenSet),
            "TTS" => Some(Unit::TenThousandSticks),
            "U1" => Some(Unit::Treatment),
            "U2" => Some(Unit::Tablet),
            "UB" => Some(Unit::TelecommunicationLineInServiceAverage),
            "UC" => Some(Unit::TelecommunicationPort),
            "VA" => Some(Unit::VoltAmperePerKilogram),
            "VLT" => Some(Unit::Volt),
            "VP" => Some(Unit::PercentVolume),
            "W2" => Some(Unit::WetKilo),
            "WA" => Some(Unit::WattPerKilogram),
            "WB" => Some(Unit::WetPound),
            "WCD" => Some(Unit::Cord),
            "WE" => Some(Unit::WetTon),
            "WEB" => Some(Unit::Weber),
            "WEE" => Some(Unit::Week),
            "WG" => Some(Unit::WineGallon),
            "WHR" => Some(Unit::WattHour),
            "WM" => Some(Unit::WorkingMonth),
            "WSD" => Some(Unit::Standard),
            "WTT" => Some(Unit::Watt),
            "X1" => Some(Unit::GuntersChain),
            "YDK" => Some(Unit::SquareYard),
            "YDQ" => Some(Unit::CubicYard),
            "YRD" => Some(Unit::Yard),
            "Z11" => Some(Unit::HangingContainer),
            "Z9" => Some(Unit::Nanomole),
            "ZP" => Some(Unit::Page),
            "ZZ" => Some(Unit::MutuallyDefined),
            "X1A" => Some(Unit::DrumSteel),
            "X1B" => Some(Unit::DrumAluminium),
            "X1D" => Some(Unit::DrumPlywood),
            "X1F" => Some(Unit::ContainerFlexible),
            "X1G" => Some(Unit::DrumFibre),
            "X1W" => Some(Unit::DrumWooden),
            "X2C" => Some(Unit::BarrelWooden),
            "X3A" => Some(Unit::JerricanSteel),
            "X3H" => Some(Unit::JerricanPlastic),
            "X43" => Some(Unit::BagSuperBulk),
            "X44" => Some(Unit::BagPolybag),
            "X4A" => Some(Unit::BoxSteel),
            "X4B" => Some(Unit::BoxAluminium),
            "X4C" => Some(Unit::BoxNaturalWood),
            "X4D" => Some(Unit::BoxPlywood),
            "X4F" => Some(Unit::BoxReconstitutedWood),
            "X4G" => Some(Unit::BoxFibreboard),
            "X4H" => Some(Unit::BoxPlastic),
            "X5H" => Some(Unit::BagWovenPlastic),
            "X5L" => Some(Unit::BagTextile),
            "X5M" => Some(Unit::BagPaper),
            "X6H" => Some(Unit::CompositePackagingPlasticReceptacle),
            "X6P" => Some(Unit::CompositePackagingGlassReceptacle),
            "X7A" => Some(Unit::CaseCar),
            "X7B" => Some(Unit::CaseWooden),
            "X8A" => Some(Unit::PalletWooden),
            "X8B" => Some(Unit::CrateWooden),
            "X8C" => Some(Unit::BundleWooden),
            "XAA" => Some(Unit::IntermediateBulkContainerRigidPlastic),
            "XAB" => Some(Unit::ReceptacleFibre),
            "XAC" => Some(Unit::ReceptaclePaper),
            "XAD" => Some(Unit::ReceptacleWooden),
            "XAE" => Some(Unit::Aerosol),
            "XAF" => Some(Unit::PalletModularCollars80cms60cms),
            "XAG" => Some(Unit::PalletShrinkwrapped),
            "XAH" => Some(Unit::Pallet100cms110cms),
            "XAI" => Some(Unit::Clamshell),
            "XAJ" => Some(Unit::Cone),
            "XAL" => Some(Unit::Ball_Dup),
            "XAM" => Some(Unit::AmpouleNonProtected),
            "XAP" => Some(Unit::AmpouleProtected),
            "XAT" => Some(Unit::Atomizer),
            "XAV" => Some(Unit::Capsule),
            "XB4" => Some(Unit::Belt),
            "XBA" => Some(Unit::Barrel),
            "XBB" => Some(Unit::Bobbin),
            "XBC" => Some(Unit::BottlecrateBottlerack),
            "XBD" => Some(Unit::Board),
            "XBE" => Some(Unit::Bundle),
            "XBF" => Some(Unit::BalloonNonProtected),
            "XBG" => Some(Unit::Bag),
            "XBH" => Some(Unit::Bunch),
            "XBI" => Some(Unit::Bin),
            "XBJ" => Some(Unit::Bucket),
            "XBK" => Some(Unit::Basket),
            "XBL" => Some(Unit::BaleCompressed),
            "XBM" => Some(Unit::Basin),
            "XBN" => Some(Unit::BaleNonCompressed),
            "XBO" => Some(Unit::BottleNonProtectedCylindrical),
            "XBP" => Some(Unit::BalloonProtected),
            "XBQ" => Some(Unit::BottleProtectedCylindrical),
            "XBR" => Some(Unit::Bar),
            "XBS" => Some(Unit::BottleNonProtectedBulbous),
            "XBT" => Some(Unit::Bolt),
            "XBU" => Some(Unit::Butt),
            "XBV" => Some(Unit::BottleProtectedBulbous),
            "XBW" => Some(Unit::BoxForLiquids),
            "XBX" => Some(Unit::Box),
            "XBY" => Some(Unit::BoardInBundleBunchTruss),
            "XBZ" => Some(Unit::BarsInBundleBunchTruss),
            "XCA" => Some(Unit::CanRectangular),
            "XCB" => Some(Unit::CrateBeer),
            "XCC" => Some(Unit::Churn),
            "XCD" => Some(Unit::CanWithHandleAndSpout),
            "XCE" => Some(Unit::Creel),
            "XCF" => Some(Unit::Coffer),
            "XCG" => Some(Unit::Cage),
            "XCH" => Some(Unit::Chest),
            "XCI" => Some(Unit::Canister),
            "XCJ" => Some(Unit::Coffin),
            "XCK" => Some(Unit::Cask),
            "XCL" => Some(Unit::Coil),
            "XCM" => Some(Unit::Card_Dup),
            "XCN" => Some(Unit::ContainerNotOtherwiseSpecifiedAsTransportEquipment),
            "XCO" => Some(Unit::CarboyNonProtected),
            "XCP" => Some(Unit::CarboyProtected),
            "XCQ" => Some(Unit::Cartridge),
            "XCR" => Some(Unit::Crate),
            "XCS" => Some(Unit::Case),
            "XCT" => Some(Unit::Carton),
            "XCU" => Some(Unit::Cup),
            "XCV" => Some(Unit::Cover),
            "XCW" => Some(Unit::CageRoll),
            "XCX" => Some(Unit::CanCylindrical),
            "XCY" => Some(Unit::Cylinder),
            "XCZ" => Some(Unit::Canvas),
            "XDA" => Some(Unit::CrateMultipleLayerPlastic),
            "XDB" => Some(Unit::CrateMultipleLayerWooden),
            "XDC" => Some(Unit::CrateMultipleLayerCardboard),
            "XDG" => Some(Unit::CageCommonwealthHandlingEquipmentPoolChep),
            "XDH" => Some(Unit::BoxCommonwealthHandlingEquipmentPoolChepEurobox),
            "XDI" => Some(Unit::DrumIron),
            "XDJ" => Some(Unit::DemijohnNonProtected),
            "XDK" => Some(Unit::CrateBulkCardboard),
            "XDL" => Some(Unit::CrateBulkPlastic),
            "XDM" => Some(Unit::CrateBulkWooden),
            "XDN" => Some(Unit::Dispenser),
            "XDP" => Some(Unit::DemijohnProtected),
            "XDR" => Some(Unit::Drum),
            "XDS" => Some(Unit::TrayOneLayerNoCoverPlastic),
            "XDT" => Some(Unit::TrayOneLayerNoCoverWooden),
            "XDU" => Some(Unit::TrayOneLayerNoCoverPolystyrene),
            "XDV" => Some(Unit::TrayOneLayerNoCoverCardboard),
            "XDW" => Some(Unit::TrayTwoLayersNoCoverPlasticTray),
            "XDX" => Some(Unit::TrayTwoLayersNoCoverWooden),
            "XDY" => Some(Unit::TrayTwoLayersNoCoverCardboard),
            "XEC" => Some(Unit::BagPlastic),
            "XED" => Some(Unit::CaseWithPalletBase),
            "XEE" => Some(Unit::CaseWithPalletBaseWooden),
            "XEF" => Some(Unit::CaseWithPalletBaseCardboard),
            "XEG" => Some(Unit::CaseWithPalletBasePlastic),
            "XEH" => Some(Unit::CaseWithPalletBaseMetal),
            "XEI" => Some(Unit::CaseIsothermic),
            "XEN" => Some(Unit::Envelope),
            "XFB" => Some(Unit::Flexibag),
            "XFC" => Some(Unit::CrateFruit),
            "XFD" => Some(Unit::CrateFramed),
            "XFE" => Some(Unit::Flexitank),
            "XFI" => Some(Unit::Firkin),
            "XFL" => Some(Unit::Flask),
            "XFO" => Some(Unit::Footlocker),
            "XFP" => Some(Unit::Filmpack),
            "XFR" => Some(Unit::Frame),
            "XFT" => Some(Unit::Foodtainer),
            "XFW" => Some(Unit::CartFlatbed),
            "XFX" => Some(Unit::BagFlexibleContainer),
            "XGB" => Some(Unit::BottleGas),
            "XGI" => Some(Unit::Girder),
            "XGL" => Some(Unit::ContainerGallon),
            "XGR" => Some(Unit::ReceptacleGlass),
            "XGU" => Some(Unit::TrayContainingHorizontallyStackedFlatItems),
            "XGY" => Some(Unit::BagGunny),
            "XGZ" => Some(Unit::GirdersInBundleBunchTruss),
            "XHA" => Some(Unit::BasketWithHandlePlastic),
            "XHB" => Some(Unit::BasketWithHandleWooden),
            "XHC" => Some(Unit::BasketWithHandleCardboard),
            "XHG" => Some(Unit::Hogshead),
            "XHN" => Some(Unit::Hanger),
            "XHR" => Some(Unit::Hamper),
            "XIA" => Some(Unit::PackageDisplayWooden),
            "XIB" => Some(Unit::PackageDisplayCardboard),
            "XIC" => Some(Unit::PackageDisplayPlastic),
            "XID" => Some(Unit::PackageDisplayMetal),
            "XIE" => Some(Unit::PackageShow),
            "XIF" => Some(Unit::PackageFlow),
            "XIG" => Some(Unit::PackagePaperWrapped),
            "XIH" => Some(Unit::DrumPlastic),
            "XIK" => Some(Unit::PackageCardboardWithBottleGripHoles),
            "XIL" => Some(Unit::TrayRigidLiddedStackableCenTs144822002),
            "XIN" => Some(Unit::Ingot),
            "XIZ" => Some(Unit::IngotsInBundleBunchTruss),
            "XJB" => Some(Unit::BagJumbo),
            "XJC" => Some(Unit::JerricanRectangular),
            "XJG" => Some(Unit::Jug),
            "XJR" => Some(Unit::Jar),
            "XJT" => Some(Unit::Jutebag),
            "XJY" => Some(Unit::JerricanCylindrical),
            "XKG" => Some(Unit::Keg),
            "XKI" => Some(Unit::Kit_Dup),
            "XLE" => Some(Unit::Luggage),
            "XLG" => Some(Unit::Log),
            "XLT" => Some(Unit::Lot),
            "XLU" => Some(Unit::Lug),
            "XLV" => Some(Unit::Liftvan),
            "XLZ" => Some(Unit::LogsInBundleBunchTruss),
            "XMA" => Some(Unit::CrateMetal),
            "XMB" => Some(Unit::BagMultiply),
            "XMC" => Some(Unit::CrateMilk),
            "XME" => Some(Unit::ContainerMetal),
            "XMR" => Some(Unit::ReceptacleMetal),
            "XMS" => Some(Unit::SackMultiWall),
            "XMT" => Some(Unit::Mat),
            "XMW" => Some(Unit::ReceptaclePlasticWrapped),
            "XMX" => Some(Unit::Matchbox),
            "XNA" => Some(Unit::NotAvailable),
            "XNE" => Some(Unit::UnpackedOrUnpackaged),
            "XNF" => Some(Unit::UnpackedOrUnpackagedSingleUnit),
            "XNG" => Some(Unit::UnpackedOrUnpackagedMultipleUnits),
            "XNS" => Some(Unit::Nest),
            "XNT" => Some(Unit::Net),
            "XNU" => Some(Unit::NetTubePlastic),
            "XNV" => Some(Unit::NetTubeTextile),
            "XO1" => Some(Unit::TwoSidedCageOnWheelsWithFixingStrap),
            "XO2" => Some(Unit::Trolley),
            "XO3" => Some(Unit::OnewayPalletIso012EuroPallet),
            "XO4" => Some(Unit::OnewayPalletIso111EuroPallet),
            "XO5" => Some(Unit::OnewayPalletIso221EuroPallet),
            "XO6" => Some(Unit::PalletWithExceptionalDimensions),
            "XO7" => Some(Unit::WoodenPallet40CmX80Cm),
            "XO8" => Some(Unit::PlasticPalletSrs60CmX80Cm),
            "XO9" => Some(Unit::PlasticPalletSrs80CmX120Cm),
            "XOA" => Some(Unit::PalletChep40CmX60Cm),
            "XOB" => Some(Unit::PalletChep80CmX120Cm),
            "XOC" => Some(Unit::PalletChep100CmX120Cm),
            "XOD" => Some(Unit::PalletAs40681993),
            "XOE" => Some(Unit::PalletIsoT11),
            "XOF" => Some(Unit::PlatformUnspecifiedWeightOrDimension),
            "XOG" => Some(Unit::PalletIso012EuroPallet),
            "XOH" => Some(Unit::PalletIso111EuroPallet),
            "XOI" => Some(Unit::PalletIso221EuroPallet),
            "XOJ" => Some(Unit::_14EuroPallet),
            "XOK" => Some(Unit::Block),
            "XOL" => Some(Unit::_18EuroPallet),
            "XOM" => Some(Unit::SyntheticPalletIso1),
            "XON" => Some(Unit::SyntheticPalletIso2),
            "XOP" => Some(Unit::WholesalerPallet),
            "XOQ" => Some(Unit::Pallet80X100Cm),
            "XOR" => Some(Unit::Pallet60X100Cm),
            "XOS" => Some(Unit::OnewayPallet),
            "XOT" => Some(Unit::Octabin),
            "XOU" => Some(Unit::ContainerOuter),
            "XOV" => Some(Unit::ReturnablePallet),
            "XOW" => Some(Unit::LargeBagPalletSized),
            "XOX" => Some(Unit::AWheeledPalletWithRaisedRim81X67X135),
            "XOY" => Some(Unit::AWheeledPalletWithRaisedRim81X72X135),
            "XOZ" => Some(Unit::WheeledPalletWithRaisedRim81X60X16),
            "XP1" => Some(Unit::ChepPallet60CmX80Cm),
            "XP2" => Some(Unit::Pan),
            "XP3" => Some(Unit::LprPallet60CmX80Cm),
            "XP4" => Some(Unit::LprPallet80CmX120Cm),
            "XPA" => Some(Unit::Packet),
            "XPB" => Some(Unit::PalletBoxCombinedOpenEndedBoxAndPallet),
            "XPC" => Some(Unit::Parcel),
            "XPD" => Some(Unit::PalletModularCollars80cms100cms),
            "XPE" => Some(Unit::PalletModularCollars80cms120cms),
            "XPF" => Some(Unit::Pen),
            "XPG" => Some(Unit::Plate),
            "XPH" => Some(Unit::Pitcher),
            "XPI" => Some(Unit::Pipe),
            "XPJ" => Some(Unit::Punnet),
            "XPK" => Some(Unit::Package),
            "XPL" => Some(Unit::Pail),
            "XPN" => Some(Unit::Plank),
            "XPO" => Some(Unit::Pouch),
            "XPP" => Some(Unit::Piece_Dup),
            "XPR" => Some(Unit::ReceptaclePlastic),
            "XPT" => Some(Unit::Pot),
            "XPU" => Some(Unit::Tray),
            "XPV" => Some(Unit::PipesInBundleBunchTruss),
            "XPX" => Some(Unit::Pallet),
            "XPY" => Some(Unit::PlatesInBundleBunchTruss),
            "XPZ" => Some(Unit::PlanksInBundleBunchTruss),
            "XQA" => Some(Unit::DrumSteelNonRemovableHead),
            "XQB" => Some(Unit::DrumSteelRemovableHead),
            "XQC" => Some(Unit::DrumAluminiumNonRemovableHead),
            "XQD" => Some(Unit::DrumAluminiumRemovableHead),
            "XQF" => Some(Unit::DrumPlasticNonRemovableHead),
            "XQG" => Some(Unit::DrumPlasticRemovableHead),
            "XQH" => Some(Unit::BarrelWoodenBungType),
            "XQJ" => Some(Unit::BarrelWoodenRemovableHead),
            "XQK" => Some(Unit::JerricanSteelNonRemovableHead),
            "XQL" => Some(Unit::JerricanSteelRemovableHead),
            "XQM" => Some(Unit::JerricanPlasticNonRemovableHead),
            "XQN" => Some(Unit::JerricanPlasticRemovableHead),
            "XQP" => Some(Unit::BoxWoodenNaturalWoodOrdinary),
            "XQQ" => Some(Unit::BoxWoodenNaturalWoodWithSiftProofWalls),
            "XQR" => Some(Unit::BoxPlasticExpanded),
            "XQS" => Some(Unit::BoxPlasticSolid),
            "XRD" => Some(Unit::Rod),
            "XRG" => Some(Unit::Ring),
            "XRJ" => Some(Unit::RackClothingHanger),
            "XRK" => Some(Unit::Rack),
            "XRL" => Some(Unit::Reel),
            "XRO" => Some(Unit::Roll),
            "XRT" => Some(Unit::Rednet),
            "XRZ" => Some(Unit::RodsInBundleBunchTruss),
            "XSA" => Some(Unit::Sack),
            "XSB" => Some(Unit::Slab),
            "XSC" => Some(Unit::CrateShallow),
            "XSD" => Some(Unit::Spindle),
            "XSE" => Some(Unit::SeaChest),
            "XSH" => Some(Unit::Sachet),
            "XSI" => Some(Unit::Skid),
            "XSK" => Some(Unit::CaseSkeleton),
            "XSL" => Some(Unit::Slipsheet),
            "XSM" => Some(Unit::Sheetmetal),
            "XSO" => Some(Unit::Spool),
            "XSP" => Some(Unit::SheetPlasticWrapping),
            "XSS" => Some(Unit::CaseSteel),
            "XST" => Some(Unit::Sheet),
            "XSU" => Some(Unit::Suitcase),
            "XSV" => Some(Unit::EnvelopeSteel),
            "XSW" => Some(Unit::Shrinkwrapped),
            "XSX" => Some(Unit::Set_Dup),
            "XSY" => Some(Unit::Sleeve),
            "XSZ" => Some(Unit::SheetsInBundleBunchTruss),
            "XT1" => Some(Unit::Tablet_Dup),
            "XTB" => Some(Unit::Tub),
            "XTC" => Some(Unit::TeaChest),
            "XTD" => Some(Unit::TubeCollapsible),
            "XTE" => Some(Unit::Tyre_Dup),
            "XTG" => Some(Unit::TankContainerGeneric),
            "XTI" => Some(Unit::Tierce),
            "XTK" => Some(Unit::TankRectangular),
            "XTL" => Some(Unit::TubWithLid),
            "XTN" => Some(Unit::Tin),
            "XTO" => Some(Unit::Tun),
            "XTR" => Some(Unit::Trunk),
            "XTS" => Some(Unit::Truss),
            "XTT" => Some(Unit::BagTote),
            "XTU" => Some(Unit::Tube),
            "XTV" => Some(Unit::TubeWithNozzle),
            "XTW" => Some(Unit::PalletTriwall),
            "XTY" => Some(Unit::TankCylindrical),
            "XTZ" => Some(Unit::TubesInBundleBunchTruss),
            "XUC" => Some(Unit::Uncaged),
            "XUN" => Some(Unit::Unit),
            "XVA" => Some(Unit::Vat),
            "XVG" => Some(Unit::BulkGasAt1031MbarAnd15C),
            "XVI" => Some(Unit::Vial),
            "XVK" => Some(Unit::Vanpack),
            "XVL" => Some(Unit::BulkLiquid),
            "XVN" => Some(Unit::Vehicle),
            "XVO" => Some(Unit::BulkSolidLargeParticlesNodules),
            "XVP" => Some(Unit::VacuumPacked),
            "XVQ" => Some(Unit::BulkLiquefiedGasAtAbnormalTemperaturePressure),
            "XVR" => Some(Unit::BulkSolidGranularParticlesGrains),
            "XVS" => Some(Unit::BulkScrapMetal),
            "XVY" => Some(Unit::BulkSolidFineParticlesPowders),
            "XWA" => Some(Unit::IntermediateBulkContainer),
            "XWB" => Some(Unit::Wickerbottle),
            "XWC" => Some(Unit::IntermediateBulkContainerSteel),
            "XWD" => Some(Unit::IntermediateBulkContainerAluminium),
            "XWF" => Some(Unit::IntermediateBulkContainerMetal),
            "XWG" => Some(Unit::IntermediateBulkContainerSteelPressurised10Kpa),
            "XWH" => Some(Unit::IntermediateBulkContainerAluminiumPressurised10Kpa),
            "XWJ" => Some(Unit::IntermediateBulkContainerMetalPressure10Kpa),
            "XWK" => Some(Unit::IntermediateBulkContainerSteelLiquid),
            "XWL" => Some(Unit::IntermediateBulkContainerAluminiumLiquid),
            "XWM" => Some(Unit::IntermediateBulkContainerMetalLiquid),
            "XWN" => Some(Unit::IntermediateBulkContainerWovenPlasticWithoutCoatLiner),
            "XWP" => Some(Unit::IntermediateBulkContainerWovenPlasticCoated),
            "XWQ" => Some(Unit::IntermediateBulkContainerWovenPlasticWithLiner),
            "XWR" => Some(Unit::IntermediateBulkContainerWovenPlasticCoatedAndLiner),
            "XWS" => Some(Unit::IntermediateBulkContainerPlasticFilm),
            "XWT" => Some(Unit::IntermediateBulkContainerTextileWithOutCoatLiner),
            "XWU" => Some(Unit::IntermediateBulkContainerNaturalWoodWithInnerLiner),
            "XWV" => Some(Unit::IntermediateBulkContainerTextileCoated),
            "XWW" => Some(Unit::IntermediateBulkContainerTextileWithLiner),
            "XWX" => Some(Unit::IntermediateBulkContainerTextileCoatedAndLiner),
            "XWY" => Some(Unit::IntermediateBulkContainerPlywoodWithInnerLiner),
            "XWZ" => Some(Unit::IntermediateBulkContainerReconstitutedWoodWithInnerLiner),
            "XXA" => Some(Unit::BagWovenPlasticWithoutInnerCoatLiner),
            "XXB" => Some(Unit::BagWovenPlasticSiftProof),
            "XXC" => Some(Unit::BagWovenPlasticWaterResistant),
            "XXD" => Some(Unit::BagPlasticsFilm),
            "XXF" => Some(Unit::BagTextileWithoutInnerCoatLiner),
            "XXG" => Some(Unit::BagTextileSiftProof),
            "XXH" => Some(Unit::BagTextileWaterResistant),
            "XXJ" => Some(Unit::BagPaperMultiWall),
            "XXK" => Some(Unit::BagPaperMultiWallWaterResistant),
            "XYA" => Some(Unit::CompositePackagingPlasticReceptacleInSteelDrum),
            "XYB" => Some(Unit::CompositePackagingPlasticReceptacleInSteelCrateBox),
            "XYC" => Some(Unit::CompositePackagingPlasticReceptacleInAluminiumDrum),
            "XYD" => Some(Unit::CompositePackagingPlasticReceptacleInAluminiumCrate),
            "XYF" => Some(Unit::CompositePackagingPlasticReceptacleInWoodenBox),
            "XYG" => Some(Unit::CompositePackagingPlasticReceptacleInPlywoodDrum),
            "XYH" => Some(Unit::CompositePackagingPlasticReceptacleInPlywoodBox),
            "XYJ" => Some(Unit::CompositePackagingPlasticReceptacleInFibreDrum),
            "XYK" => Some(Unit::CompositePackagingPlasticReceptacleInFibreboardBox),
            "XYL" => Some(Unit::CompositePackagingPlasticReceptacleInPlasticDrum),
            "XYM" => Some(Unit::CompositePackagingPlasticReceptacleInSolidPlasticBox),
            "XYN" => Some(Unit::CompositePackagingGlassReceptacleInSteelDrum),
            "XYP" => Some(Unit::CompositePackagingGlassReceptacleInSteelCrateBox),
            "XYQ" => Some(Unit::CompositePackagingGlassReceptacleInAluminiumDrum),
            "XYR" => Some(Unit::CompositePackagingGlassReceptacleInAluminiumCrate),
            "XYS" => Some(Unit::CompositePackagingGlassReceptacleInWoodenBox),
            "XYT" => Some(Unit::CompositePackagingGlassReceptacleInPlywoodDrum),
            "XYV" => Some(Unit::CompositePackagingGlassReceptacleInWickerworkHamper),
            "XYW" => Some(Unit::CompositePackagingGlassReceptacleInFibreDrum),
            "XYX" => Some(Unit::CompositePackagingGlassReceptacleInFibreboardBox),
            "XYY" => Some(Unit::CompositePackagingGlassReceptacleInExpandablePlasticPack),
            "XYZ" => Some(Unit::CompositePackagingGlassReceptacleInSolidPlasticPack),
            "XZA" => Some(Unit::IntermediateBulkContainerPaperMultiWall),
            "XZB" => Some(Unit::BagLarge),
            "XZC" => Some(Unit::IntermediateBulkContainerPaperMultiWallWaterResistant),
            "XZD" => Some(Unit::IntermediateBulkContainerRigidPlasticWithStructuralEquipmentSolids),
            "XZF" => Some(Unit::IntermediateBulkContainerRigidPlasticFreestandingSolids),
            "XZG" => {
                Some(Unit::IntermediateBulkContainerRigidPlasticWithStructuralEquipmentPressurised)
            }
            "XZH" => Some(Unit::IntermediateBulkContainerRigidPlasticFreestandingPressurised),
            "XZJ" => {
                Some(Unit::IntermediateBulkContainerRigidPlasticWithStructuralEquipmentLiquids)
            }
            "XZK" => Some(Unit::IntermediateBulkContainerRigidPlasticFreestandingLiquids),
            "XZL" => Some(Unit::IntermediateBulkContainerCompositeRigidPlasticSolids),
            "XZM" => Some(Unit::IntermediateBulkContainerCompositeFlexiblePlasticSolids),
            "XZN" => Some(Unit::IntermediateBulkContainerCompositeRigidPlasticPressurised),
            "XZP" => Some(Unit::IntermediateBulkContainerCompositeFlexiblePlasticPressurised),
            "XZQ" => Some(Unit::IntermediateBulkContainerCompositeRigidPlasticLiquids),
            "XZR" => Some(Unit::IntermediateBulkContainerCompositeFlexiblePlasticLiquids),
            "XZS" => Some(Unit::IntermediateBulkContainerComposite),
            "XZT" => Some(Unit::IntermediateBulkContainerFibreboard),
            "XZU" => Some(Unit::IntermediateBulkContainerFlexible),
            "XZV" => Some(Unit::IntermediateBulkContainerMetalOtherThanSteel),
            "XZW" => Some(Unit::IntermediateBulkContainerNaturalWood),
            "XZX" => Some(Unit::IntermediateBulkContainerPlywood),
            "XZY" => Some(Unit::IntermediateBulkContainerReconstitutedWood),
            "XZZ" => Some(Unit::MutuallyDefined_Dup),
            _ => None,
        }
    }
}
