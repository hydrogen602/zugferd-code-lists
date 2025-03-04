#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Characteristic {
    /// Consolidated weight
    ConsolidatedWeight,
    /// Net weight
    NetWeight,
    /// Goods item gross weight
    GoodsItemGrossWeight,
    /// Total net weight
    TotalNetWeight,
    /// Consignment gross weight
    ConsignmentGrossWeight,
    /// Net net weight
    NetNetWeight,
    /// Stern thrust
    SternThrust,
    /// Bow thrust
    BowThrust,
    /// Hydrate content of an alcoholic product at bottling
    HydrateContentAnAlcoholicProductAtBottling,
    /// Number of units per pallet
    NumberUnitsPerPallet,
    /// Fat content
    FatContent,
    /// Transport means gross weight
    TransportMeansGrossWeight,
    /// Net tonnage of the vessel
    NetTonnageVessel,
    /// Humidity
    Humidity,
    /// Voltage
    Voltage,
    /// Power consumption
    PowerConsumption,
    /// Heat dissipation
    HeatDissipation,
    /// Air flow
    AirFlow,
    /// Shock impact
    ShockImpact,
    /// Operative temperature
    OperativeTemperature,
    /// Non operative temperature
    NonOperativeTemperature,
    /// Gross volume
    GrossVolume,
    /// Net volume
    NetVolume,
    /// Water content
    WaterContent,
    /// Tensile stress
    TensileStress,
    /// Fibrosity
    Fibrosity,
    /// Gauge length
    GaugeLength,
    /// Radius
    Radius,
    /// Straightness
    Straightness,
    /// Strain
    Strain,
    /// Item width when unrolled
    ItemWidthWhenUnrolled,
    /// Item length when unrolled
    ItemLengthWhenUnrolled,
    /// Item area when unrolled
    ItemAreaWhenUnrolled,
    /// Original wort
    OriginalWort,
    /// Volume
    Volume,
    /// Angle
    Angle,
    /// Peg hole horizontal distance from package leftmost edge
    PegHoleHorizontalDistanceFromPackageLeftmostEdge,
    /// Peg hole vertical distance from package top
    PegHoleVerticalDistanceFromPackageTop,
    /// Number of layers per pallet
    NumberLayersPerPallet,
    /// Product strengh, chemical
    ProductStrenghChemical,
    /// Product strength basis, chemical
    ProductStrengthBasisChemical,
    /// Item weight
    ItemWeight,
    /// Payload weight, maximum
    PayloadWeightMaximum,
    /// Weight of conveyance
    WeightConveyance,
    /// Conveyance summer dead weight
    ConveyanceSummerDeadWeight,
    /// Containerized cargo on vessel's weight
    ContainerizedCargoOnVesselsWeight,
    /// Non-containerized cargo on vessel's weight
    NonContainerizedCargoOnVesselsWeight,
    /// Ascertained weight
    AscertainedWeight,
    /// Chargeable weight
    ChargeableWeight,
    /// Estimated gross weight
    EstimatedGrossWeight,
    /// Estimated volume
    EstimatedVolume,
    /// Vessel overall length
    VesselOverallLength,
    /// Loading meters
    LoadingMeters,
    /// Number of axles
    NumberAxles,
    /// Payload
    Payload,
    /// Start position in the length
    StartPositionInLength,
    /// End position in the length
    EndPositionInLength,
    /// Start position in the width
    StartPositionInWidth,
    /// End position in the width
    EndPositionInWidth,
    /// Start position in the thickness
    StartPositionInThickness,
    /// End position in the thickness
    EndPositionInThickness,
    /// Transport container actual filling weight
    TransportContainerActualFillingWeight,
    /// Transport container maximum capacity
    TransportContainerMaximumCapacity,
    /// Declared net weight
    DeclaredNetWeight,
    /// Loading height
    LoadingHeight,
    /// Stacking height
    StackingHeight,
    /// Calculated weight
    CalculatedWeight,
    /// Ferrite
    Ferrite,
    /// Impurity
    Impurity,
    /// Grain size
    GrainSize,
    /// Lanthanides
    Lanthanides,
    /// Elasticity
    Elasticity,
    /// Drained weight
    DrainedWeight,
    /// Gallium
    Gallium,
    /// Strontium
    Strontium,
    /// Area
    Area,
    /// Equipment storage limitation
    EquipmentStorageLimitation,
    /// Radioactive index of transport
    RadioactiveIndexTransport,
    /// Radioactivity
    Radioactivity,
    /// Average gross weight
    AverageGrossWeight,
    /// Forward draft
    ForwardDraft,
    /// After draft
    AfterDraft,
    /// Acidity
    Acidity,
    /// Transport equipment gross weight
    TransportEquipmentGrossWeight,
    /// Total transport equipment gross weight
    TotalTransportEquipmentGrossWeight,
    /// Acidity of juice
    AcidityJuice,
    /// Penetrometry
    Penetrometry,
    /// Durofel
    Durofel,
    /// Juice weight per 100 grams
    JuiceWeightPer100Grams,
    /// Fruit skin colour
    FruitSkinColour,
    /// Angle of bend
    AngleBend,
    /// Fixed incremental measurement
    FixedIncrementalMeasurement,
    /// Durofel D10
    DurofelD10,
    /// Durofel D25
    DurofelD25,
    /// Durofel D50
    DurofelD50,
    /// Maximum stacking weight
    MaximumStackingWeight,
    /// Gross measure cube
    GrossMeasureCube,
    /// Percentage fat content in dry matter
    PercentageFatContentInDryMatter,
    /// Saccharometric content
    SaccharometricContent,
    /// Hydrate content of an alcoholic product after bottling
    HydrateContentAnAlcoholicProductAfterBottling,
    /// Anhydrous content
    AnhydrousContent,
    /// Certified weight
    CertifiedWeight,
    /// Freeboard
    Freeboard,
    /// Maximum vessel draught
    MaximumVesselDraught,
    /// Net explosive weight
    NetExplosiveWeight,
    /// Radioactive criticality safety index
    RadioactiveCriticalitySafetyIndex,
    /// Waste currently on board
    WasteCurrentlyOnBoard,
    /// Waste to be delivered at waste reception facility
    WasteToBeDeliveredAtWasteReceptionFacility,
    /// Waste to be generated until next port of call, estimated
    WasteToBeGeneratedUntilNextPortCallEstimated,
    /// Waste remaining on board at departure
    WasteRemainingOnBoardAtDeparture,
    /// Colour depth
    ColourDepth,
    /// Colour depth, maximum
    ColourDepthMaximum,
    /// Image resolution
    ImageResolution,
    /// Device resolution, maximum
    DeviceResolutionMaximum,
    /// Acoustic absorption coefficient
    AcousticAbsorptionCoefficient,
    /// Billed weight
    BilledWeight,
    /// Breaking load
    BreakingLoad,
    /// Platinum
    Platinum,
    /// Silver
    Silver,
    /// List
    List,
    /// Trim
    Trim,
    /// Free water
    FreeWater,
    /// Bands
    Bands,
    /// API (American Petroleum Institute) gravity
    ApiAmericanPetroleumInstituteGravity,
    /// Petroleum gross observed volume
    PetroleumGrossObservedVolume,
    /// Petroleum gross standard volume
    PetroleumGrossStandardVolume,
    /// Volume variance
    VolumeVariance,
    /// Petroleum net standard volume
    PetroleumNetStandardVolume,
    /// Material on-board quantity, after discharge
    MaterialOnBoardQuantityAfterDischarge,
    /// Petroleum total calculated volume
    PetroleumTotalCalculatedVolume,
    /// Petroleum total observed volume
    PetroleumTotalObservedVolume,
    /// Innage gauge distance
    InnageGaugeDistance,
    /// Petroleum net standard weight
    PetroleumNetStandardWeight,
    /// Sediment and water in petroleum
    SedimentAndWaterInPetroleum,
    /// Observed reference height, tank
    ObservedReferenceHeightTank,
    /// Reference height, tank
    ReferenceHeightTank,
    /// Ullage gauge distance
    UllageGaugeDistance,
    /// Trim correction
    TrimCorrection,
    /// Bow to bridge distance
    BowToBridgeDistance,
    /// Peg hole number
    PegHoleNumber,
    /// Number of inner packs
    NumberInnerPacks,
    /// Number of next level trade items within inner pack
    NumberNextLevelTradeItemsWithinInnerPack,
    /// Number of trade items per pallet layer
    NumberTradeItemsPerPalletLayer,
    /// Packed items layer height
    PackedItemsLayerHeight,
    /// Packing material weight, skin tight covering
    PackingMaterialWeightSkinTightCovering,
    /// Brightness
    Brightness,
    /// Brakes
    Brakes,
    /// Components labelled for recycling percentage
    ComponentsLabelledForRecyclingPercentage,
    /// Renewable plastic components percentage, by net weight
    RenewablePlasticComponentsPercentageByNetWeight,
    /// Clamp pressure, required
    ClampPressureRequired,
    /// Break
    Break,
    /// Ascertained volume
    AscertainedVolume,
    /// Unit weight
    UnitWeight,
    /// Total volume
    TotalVolume,
    /// Unit volume
    UnitVolume,
    /// Vertical center of gravity
    VerticalCenterGravity,
    /// Maximum allowable transport stacking weight
    MaximumAllowableTransportStackingWeight,
    /// Carbon Dioxide
    CarbonDioxide,
    /// Number of base units per pallet
    NumberBaseUnitsPerPallet,
    /// Colony forming unit
    ColonyFormingUnit,
    /// Diluted liquid volume
    DilutedLiquidVolume,
    /// Energy efficiency
    EnergyEfficiency,
    /// Number of layers
    NumberLayers,
    /// Maximum demand
    MaximumDemand,
    /// Number of pallet places
    NumberPalletPlaces,
    /// Package net measurement, cubed
    PackageNetMeasurementCubed,
    /// Power factor
    PowerFactor,
    /// Stacking factor excluding bottom item
    StackingFactorExcludingBottomItem,
    /// Breaking strength
    BreakingStrength,
    /// Breaking strength wet
    BreakingStrengthWet,
    /// Step size
    StepSize,
    /// Number of units per package
    NumberUnitsPerPackage,
    /// Number of units per layer
    NumberUnitsPerLayer,
    /// Weight per running metre
    WeightPerRunningMetre,
    /// Weight per square metre
    WeightPerSquareMetre,
    /// Acidity of meat
    AcidityMeat,
    /// Slaughtering weight
    SlaughteringWeight,
    /// Stacking factor including bottom item
    StackingFactorIncludingBottomItem,
    /// Number of units in the width of a layer
    NumberUnitsInWidthALayer,
    /// Number of units in the depth of a layer
    NumberUnitsInDepthALayer,
    /// Nestable percentage
    NestablePercentage,
    /// Gross weight including carrier's equipment
    GrossWeightIncludingCarriersEquipment,
    /// Sugar content
    SugarContent,
    /// Self-accelerating polymerization temperature (SAPT)
    SelfAcceleratingPolymerizationTemperatureSapt,
    /// Self-accelerating decomposition temperature (SADT)
    SelfAcceleratingDecompositionTemperatureSadt,
    /// Control temperature
    ControlTemperature,
    /// Basis weight
    BasisWeight,
    /// Change
    Change,
    /// Emergency temperature
    EmergencyTemperature,
    /// Colour
    Colour,
    /// Contents of package
    ContentsPackage,
    /// Commercial weight
    CommercialWeight,
    /// Core length
    CoreLength,
    /// Destination weight agreement
    DestinationWeightAgreement,
    /// Diameter
    Diameter,
    /// Delta value L
    DeltaValueL,
    /// Density
    Density,
    /// Depth
    Depth,
    /// Denier
    Denier,
    /// Distance between points
    DistanceBetweenPoints,
    /// Width, boxcar door
    WidthBoxcarDoor,
    /// Estimated new weight
    EstimatedNewWeight,
    /// Elongation
    Elongation,
    /// Deficit weight
    DeficitWeight,
    /// Filament count
    FilamentCount,
    /// Longitudinal flatness
    LongitudinalFlatness,
    /// Flatness
    Flatness,
    /// Transverse flatness
    TransverseFlatness,
    /// Gauge
    Gauge,
    /// Gross weight, maximum
    GrossWeightMaximum,
    /// Hardness
    Hardness,
    /// Height, maximum
    HeightMaximum,
    /// Height dimension
    HeightDimension,
    /// Impact energy
    ImpactEnergy,
    /// Inside diameter
    InsideDiameter,
    /// Legal weight
    LegalWeight,
    /// Length, maximum
    LengthMaximum,
    /// Length
    Length,
    /// Lost end
    LostEnd,
    /// Minimum weight
    MinimumWeight,
    /// Moisture
    Moisture,
    /// Maximum weight
    MaximumWeight,
    /// Actual net weight
    ActualNetWeight,
    /// Outside diameter
    OutsideDiameter,
    /// Pre stretch
    PreStretch,
    /// Per tonne
    PerTonne,
    /// Relative humidity
    RelativeHumidity,
    /// Resistivity
    Resistivity,
    /// Rockwell C
    RockwellC,
    /// Ream weight
    ReamWeight,
    /// Reduction of area
    ReductionArea,
    /// Run (process)
    RunProcess,
    /// Ratio
    Ratio,
    /// Shipped quantity
    ShippedQuantity,
    /// Tare weight
    TareWeight,
    /// Temperature
    Temperature,
    /// Thickness
    Thickness,
    /// Time period
    TimePeriod,
    /// Time
    Time,
    /// Transport equipment verified gross mass (weight)
    TransportEquipmentVerifiedGrossMassWeight,
    /// Height, van door
    HeightVanDoor,
    /// Width, van door
    WidthVanDoor,
    /// Weight per unit of area
    WeightPerUnitArea,
    /// Width dimension
    WidthDimension,
    /// Width, maximum
    WidthMaximum,
    /// Weight per unit of length
    WeightPerUnitLength,
    /// Side height, flat bed with removable sides
    SideHeightFlatBedWithRemovableSides,
    /// Squareness
    Squareness,
    /// Spool size
    SpoolSize,
    /// Yield stress
    YieldStress,
    /// Aluminium
    Aluminium,
    /// Arsenic
    Arsenic,
    /// Boron
    Boron,
    /// Bismuth
    Bismuth,
    /// Carbon
    Carbon,
    /// Calcium
    Calcium,
    /// Columbium
    Columbium,
    /// Cerium
    Cerium,
    /// Chlorine
    Chlorine,
    /// Cobalt
    Cobalt,
    /// Chromium
    Chromium,
    /// Copper
    Copper,
    /// Iron
    Iron,
    /// Iron plus silicon
    IronPlusSilicon,
    /// Germanium
    Germanium,
    /// Hydrogen
    Hydrogen,
    /// Potassium
    Potassium,
    /// Magnesium
    Magnesium,
    /// Manganese
    Manganese,
    /// Molybdenum
    Molybdenum,
    /// Nitrogen
    Nitrogen,
    /// Sodium
    Sodium,
    /// Niobium
    Niobium,
    /// Nickel
    Nickel,
    /// Oxygen
    Oxygen,
    /// Phosphorus
    Phosphorus,
    /// Lead
    Lead,
    /// Sulphur
    Sulphur,
    /// Antimony
    Antimony,
    /// Selenium
    Selenium,
    /// Silicon
    Silicon,
    /// Silicium oxyd
    SiliciumOxyd,
    /// Tin
    Tin,
    /// Tantalium
    Tantalium,
    /// Tellurium
    Tellurium,
    /// Titanium
    Titanium,
    /// Vanadium
    Vanadium,
    /// Tungsten
    Tungsten,
    /// Waste content
    WasteContent,
    /// Zinc
    Zinc,
    /// Zirconium
    Zirconium,
    /// Mutually defined
    MutuallyDefined,
    /// Best before date
    BestBeforeDate,
    /// Colour as text
    ColourAsText,
    /// Commission indicator
    CommissionIndicator,
    /// Deposit system
    DepositSystem,
    /// Deposit type
    DepositType,
    /// Energy efficiency class
    EnergyEfficiencyClass,
    /// Expiration date
    ExpirationDate,
    /// Fee indicator
    FeeIndicator,
    /// Type of article
    TypeArticle,
    /// Material of the product
    MaterialProduct,
    /// Metering point designation, e.g. for electricity or gas
    MeteringPointDesignationEGForElectricityOrGas,
    /// Meter number, e.g. for electricity or gas
    MeterNumberEGForElectricityOrGas,
    /// Organic control body number
    OrganicControlBodyNumber,
    /// Packaging material
    PackagingMaterial,
    /// Type of packaging (code)
    TypePackagingCode,
    /// Number of the action variant
    NumberActionVariant,
    /// Seal number
    SealNumber,
    /// Size code
    SizeCode,
    /// Size designation
    SizeDesignation,
    /// Type of trading unit
    TypeTradingUnit,
    /// Waste code (EWC)
    WasteCodeEwc,
    /// Waste fraction
    WasteFraction,
    /// WEEE registration number
    WeeeRegistrationNumber,
}

impl crate::Code for Characteristic {
    fn code(&self) -> &str {
        match self {
            Characteristic::ConsolidatedWeight => "A",
            Characteristic::NetWeight => "AAA",
            Characteristic::GoodsItemGrossWeight => "AAB",
            Characteristic::TotalNetWeight => "AAC",
            Characteristic::ConsignmentGrossWeight => "AAD",
            Characteristic::NetNetWeight => "AAF",
            Characteristic::SternThrust => "AAG",
            Characteristic::BowThrust => "AAH",
            Characteristic::HydrateContentAnAlcoholicProductAtBottling => "AAI",
            Characteristic::NumberUnitsPerPallet => "AAJ",
            Characteristic::FatContent => "AAK",
            Characteristic::TransportMeansGrossWeight => "AAM",
            Characteristic::NetTonnageVessel => "AAN",
            Characteristic::Humidity => "AAO",
            Characteristic::Voltage => "AAP",
            Characteristic::PowerConsumption => "AAQ",
            Characteristic::HeatDissipation => "AAR",
            Characteristic::AirFlow => "AAS",
            Characteristic::ShockImpact => "AAT",
            Characteristic::OperativeTemperature => "AAU",
            Characteristic::NonOperativeTemperature => "AAV",
            Characteristic::GrossVolume => "AAW",
            Characteristic::NetVolume => "AAX",
            Characteristic::WaterContent => "AAY",
            Characteristic::TensileStress => "AAZ",
            Characteristic::Fibrosity => "ABA",
            Characteristic::GaugeLength => "ABB",
            Characteristic::Radius => "ABC",
            Characteristic::Straightness => "ABD",
            Characteristic::Strain => "ABE",
            Characteristic::ItemWidthWhenUnrolled => "ABF",
            Characteristic::ItemLengthWhenUnrolled => "ABG",
            Characteristic::ItemAreaWhenUnrolled => "ABH",
            Characteristic::OriginalWort => "ABI",
            Characteristic::Volume => "ABJ",
            Characteristic::Angle => "ABK",
            Characteristic::PegHoleHorizontalDistanceFromPackageLeftmostEdge => "ABL",
            Characteristic::PegHoleVerticalDistanceFromPackageTop => "ABM",
            Characteristic::NumberLayersPerPallet => "ABN",
            Characteristic::ProductStrenghChemical => "ABO",
            Characteristic::ProductStrengthBasisChemical => "ABP",
            Characteristic::ItemWeight => "ABS",
            Characteristic::PayloadWeightMaximum => "ABT",
            Characteristic::WeightConveyance => "ABX",
            Characteristic::ConveyanceSummerDeadWeight => "ABY",
            Characteristic::ContainerizedCargoOnVesselsWeight => "ABZ",
            Characteristic::NonContainerizedCargoOnVesselsWeight => "ACA",
            Characteristic::AscertainedWeight => "ACE",
            Characteristic::ChargeableWeight => "ACG",
            Characteristic::EstimatedGrossWeight => "ACN",
            Characteristic::EstimatedVolume => "ACP",
            Characteristic::VesselOverallLength => "ACS",
            Characteristic::LoadingMeters => "ACV",
            Characteristic::NumberAxles => "ACW",
            Characteristic::Payload => "ACX",
            Characteristic::StartPositionInLength => "ADR",
            Characteristic::EndPositionInLength => "ADS",
            Characteristic::StartPositionInWidth => "ADT",
            Characteristic::EndPositionInWidth => "ADU",
            Characteristic::StartPositionInThickness => "ADV",
            Characteristic::EndPositionInThickness => "ADW",
            Characteristic::TransportContainerActualFillingWeight => "ADX",
            Characteristic::TransportContainerMaximumCapacity => "ADY",
            Characteristic::DeclaredNetWeight => "ADZ",
            Characteristic::LoadingHeight => "AEA",
            Characteristic::StackingHeight => "AEB",
            Characteristic::CalculatedWeight => "AEC",
            Characteristic::Ferrite => "AED",
            Characteristic::Impurity => "AEE",
            Characteristic::GrainSize => "AEF",
            Characteristic::Lanthanides => "AEG",
            Characteristic::Elasticity => "AEH",
            Characteristic::DrainedWeight => "AEI",
            Characteristic::Gallium => "AEJ",
            Characteristic::Strontium => "AEK",
            Characteristic::Area => "AEL",
            Characteristic::EquipmentStorageLimitation => "AEM",
            Characteristic::RadioactiveIndexTransport => "AEN",
            Characteristic::Radioactivity => "AEO",
            Characteristic::AverageGrossWeight => "AEP",
            Characteristic::ForwardDraft => "AEQ",
            Characteristic::AfterDraft => "AER",
            Characteristic::Acidity => "AES",
            Characteristic::TransportEquipmentGrossWeight => "AET",
            Characteristic::TotalTransportEquipmentGrossWeight => "AEU",
            Characteristic::AcidityJuice => "AEV",
            Characteristic::Penetrometry => "AEW",
            Characteristic::Durofel => "AEX",
            Characteristic::JuiceWeightPer100Grams => "AEY",
            Characteristic::FruitSkinColour => "AEZ",
            Characteristic::AngleBend => "AF",
            Characteristic::FixedIncrementalMeasurement => "AFA",
            Characteristic::DurofelD10 => "AFB",
            Characteristic::DurofelD25 => "AFC",
            Characteristic::DurofelD50 => "AFD",
            Characteristic::MaximumStackingWeight => "AFE",
            Characteristic::GrossMeasureCube => "AFF",
            Characteristic::PercentageFatContentInDryMatter => "AFG",
            Characteristic::SaccharometricContent => "AFH",
            Characteristic::HydrateContentAnAlcoholicProductAfterBottling => "AFI",
            Characteristic::AnhydrousContent => "AFJ",
            Characteristic::CertifiedWeight => "AFK",
            Characteristic::Freeboard => "AFL",
            Characteristic::MaximumVesselDraught => "AFM",
            Characteristic::NetExplosiveWeight => "AFN",
            Characteristic::RadioactiveCriticalitySafetyIndex => "AFO",
            Characteristic::WasteCurrentlyOnBoard => "AFP",
            Characteristic::WasteToBeDeliveredAtWasteReceptionFacility => "AFQ",
            Characteristic::WasteToBeGeneratedUntilNextPortCallEstimated => "AFR",
            Characteristic::WasteRemainingOnBoardAtDeparture => "AFS",
            Characteristic::ColourDepth => "AFT",
            Characteristic::ColourDepthMaximum => "AFU",
            Characteristic::ImageResolution => "AFV",
            Characteristic::DeviceResolutionMaximum => "AFW",
            Characteristic::AcousticAbsorptionCoefficient => "AFX",
            Characteristic::BilledWeight => "B",
            Characteristic::BreakingLoad => "BL",
            Characteristic::Platinum => "BMY",
            Characteristic::Silver => "BMZ",
            Characteristic::List => "BNA",
            Characteristic::Trim => "BNB",
            Characteristic::FreeWater => "BNC",
            Characteristic::Bands => "BND",
            Characteristic::ApiAmericanPetroleumInstituteGravity => "BNE",
            Characteristic::PetroleumGrossObservedVolume => "BNF",
            Characteristic::PetroleumGrossStandardVolume => "BNG",
            Characteristic::VolumeVariance => "BNH",
            Characteristic::PetroleumNetStandardVolume => "BNI",
            Characteristic::MaterialOnBoardQuantityAfterDischarge => "BNJ",
            Characteristic::PetroleumTotalCalculatedVolume => "BNK",
            Characteristic::PetroleumTotalObservedVolume => "BNL",
            Characteristic::InnageGaugeDistance => "BNM",
            Characteristic::PetroleumNetStandardWeight => "BNN",
            Characteristic::SedimentAndWaterInPetroleum => "BNO",
            Characteristic::ObservedReferenceHeightTank => "BNP",
            Characteristic::ReferenceHeightTank => "BNQ",
            Characteristic::UllageGaugeDistance => "BNR",
            Characteristic::TrimCorrection => "BNS",
            Characteristic::BowToBridgeDistance => "BNT",
            Characteristic::PegHoleNumber => "BNU",
            Characteristic::NumberInnerPacks => "BNV",
            Characteristic::NumberNextLevelTradeItemsWithinInnerPack => "BNW",
            Characteristic::NumberTradeItemsPerPalletLayer => "BNX",
            Characteristic::PackedItemsLayerHeight => "BNY",
            Characteristic::PackingMaterialWeightSkinTightCovering => "BNZ",
            Characteristic::Brightness => "BR",
            Characteristic::Brakes => "BRA",
            Characteristic::ComponentsLabelledForRecyclingPercentage => "BRB",
            Characteristic::RenewablePlasticComponentsPercentageByNetWeight => "BRC",
            Characteristic::ClampPressureRequired => "BRD",
            Characteristic::Break => "BRE",
            Characteristic::AscertainedVolume => "BRF",
            Characteristic::UnitWeight => "BRG",
            Characteristic::TotalVolume => "BRH",
            Characteristic::UnitVolume => "BRI",
            Characteristic::VerticalCenterGravity => "BRJ",
            Characteristic::MaximumAllowableTransportStackingWeight => "BRK",
            Characteristic::CarbonDioxide => "BRL",
            Characteristic::NumberBaseUnitsPerPallet => "BRM",
            Characteristic::ColonyFormingUnit => "BRN",
            Characteristic::DilutedLiquidVolume => "BRO",
            Characteristic::EnergyEfficiency => "BRP",
            Characteristic::NumberLayers => "BRQ",
            Characteristic::MaximumDemand => "BRR",
            Characteristic::NumberPalletPlaces => "BRS",
            Characteristic::PackageNetMeasurementCubed => "BRT",
            Characteristic::PowerFactor => "BRU",
            Characteristic::StackingFactorExcludingBottomItem => "BRV",
            Characteristic::BreakingStrength => "BS",
            Characteristic::BreakingStrengthWet => "BSW",
            Characteristic::StepSize => "BSX",
            Characteristic::NumberUnitsPerPackage => "BSY",
            Characteristic::NumberUnitsPerLayer => "BSZ",
            Characteristic::WeightPerRunningMetre => "BTA",
            Characteristic::WeightPerSquareMetre => "BTB",
            Characteristic::AcidityMeat => "BTC",
            Characteristic::SlaughteringWeight => "BTD",
            Characteristic::StackingFactorIncludingBottomItem => "BTE",
            Characteristic::NumberUnitsInWidthALayer => "BTF",
            Characteristic::NumberUnitsInDepthALayer => "BTG",
            Characteristic::NestablePercentage => "BTH",
            Characteristic::GrossWeightIncludingCarriersEquipment => "BTI",
            Characteristic::SugarContent => "BTJ",
            Characteristic::SelfAcceleratingPolymerizationTemperatureSapt => "BTK",
            Characteristic::SelfAcceleratingDecompositionTemperatureSadt => "BTL",
            Characteristic::ControlTemperature => "BTM",
            Characteristic::BasisWeight => "BW",
            Characteristic::Change => "CHN",
            Characteristic::EmergencyTemperature => "CHO",
            Characteristic::Colour => "CM",
            Characteristic::ContentsPackage => "CT",
            Characteristic::CommercialWeight => "CV",
            Characteristic::CoreLength => "CZ",
            Characteristic::DestinationWeightAgreement => "D",
            Characteristic::Diameter => "DI",
            Characteristic::DeltaValueL => "DL",
            Characteristic::Density => "DN",
            Characteristic::Depth => "DP",
            Characteristic::Denier => "DR",
            Characteristic::DistanceBetweenPoints => "DS",
            Characteristic::WidthBoxcarDoor => "DW",
            Characteristic::EstimatedNewWeight => "E",
            Characteristic::Elongation => "EA",
            Characteristic::DeficitWeight => "F",
            Characteristic::FilamentCount => "FI",
            Characteristic::LongitudinalFlatness => "FL",
            Characteristic::Flatness => "FN",
            Characteristic::TransverseFlatness => "FV",
            Characteristic::Gauge => "GG",
            Characteristic::GrossWeightMaximum => "GW",
            Characteristic::Hardness => "HF",
            Characteristic::HeightMaximum => "HM",
            Characteristic::HeightDimension => "HT",
            Characteristic::ImpactEnergy => "IB",
            Characteristic::InsideDiameter => "ID",
            Characteristic::LegalWeight => "L",
            Characteristic::LengthMaximum => "LM",
            Characteristic::Length => "LN",
            Characteristic::LostEnd => "LND",
            Characteristic::MinimumWeight => "M",
            Characteristic::Moisture => "MO",
            Characteristic::MaximumWeight => "MW",
            Characteristic::ActualNetWeight => "N",
            Characteristic::OutsideDiameter => "OD",
            Characteristic::PreStretch => "PRS",
            Characteristic::PerTonne => "PTN",
            Characteristic::RelativeHumidity => "RA",
            Characteristic::Resistivity => "RF",
            Characteristic::RockwellC => "RJ",
            Characteristic::ReamWeight => "RMW",
            Characteristic::ReductionArea => "RP",
            Characteristic::RunProcess => "RUN",
            Characteristic::Ratio => "RY",
            Characteristic::ShippedQuantity => "SQ",
            Characteristic::TareWeight => "T",
            Characteristic::Temperature => "TC",
            Characteristic::Thickness => "TH",
            Characteristic::TimePeriod => "TN",
            Characteristic::Time => "TT",
            Characteristic::TransportEquipmentVerifiedGrossMassWeight => "VGM",
            Characteristic::HeightVanDoor => "VH",
            Characteristic::WidthVanDoor => "VW",
            Characteristic::WeightPerUnitArea => "WA",
            Characteristic::WidthDimension => "WD",
            Characteristic::WidthMaximum => "WM",
            Characteristic::WeightPerUnitLength => "WU",
            Characteristic::SideHeightFlatBedWithRemovableSides => "XH",
            Characteristic::Squareness => "XQ",
            Characteristic::SpoolSize => "XZ",
            Characteristic::YieldStress => "YS",
            Characteristic::Aluminium => "ZAL",
            Characteristic::Arsenic => "ZAS",
            Characteristic::Boron => "ZB",
            Characteristic::Bismuth => "ZBI",
            Characteristic::Carbon => "ZC",
            Characteristic::Calcium => "ZCA",
            Characteristic::Columbium => "ZCB",
            Characteristic::Cerium => "ZCE",
            Characteristic::Chlorine => "ZCL",
            Characteristic::Cobalt => "ZCO",
            Characteristic::Chromium => "ZCR",
            Characteristic::Copper => "ZCU",
            Characteristic::Iron => "ZFE",
            Characteristic::IronPlusSilicon => "ZFS",
            Characteristic::Germanium => "ZGE",
            Characteristic::Hydrogen => "ZH",
            Characteristic::Potassium => "ZK",
            Characteristic::Magnesium => "ZMG",
            Characteristic::Manganese => "ZMN",
            Characteristic::Molybdenum => "ZMO",
            Characteristic::Nitrogen => "ZN",
            Characteristic::Sodium => "ZNA",
            Characteristic::Niobium => "ZNB",
            Characteristic::Nickel => "ZNI",
            Characteristic::Oxygen => "ZO",
            Characteristic::Phosphorus => "ZP",
            Characteristic::Lead => "ZPB",
            Characteristic::Sulphur => "ZS",
            Characteristic::Antimony => "ZSB",
            Characteristic::Selenium => "ZSE",
            Characteristic::Silicon => "ZSI",
            Characteristic::SiliciumOxyd => "ZSL",
            Characteristic::Tin => "ZSN",
            Characteristic::Tantalium => "ZTA",
            Characteristic::Tellurium => "ZTE",
            Characteristic::Titanium => "ZTI",
            Characteristic::Vanadium => "ZV",
            Characteristic::Tungsten => "ZW",
            Characteristic::WasteContent => "ZWA",
            Characteristic::Zinc => "ZZN",
            Characteristic::Zirconium => "ZZR",
            Characteristic::MutuallyDefined => "ZZZ",
            Characteristic::BestBeforeDate => "BEST_BEFORE_DATE",
            Characteristic::ColourAsText => "COLOR_TEXT",
            Characteristic::CommissionIndicator => "COMMISSION",
            Characteristic::DepositSystem => "DEPOSIT_SYSTEM",
            Characteristic::DepositType => "DEPOSIT_TYPE",
            Characteristic::EnergyEfficiencyClass => "ENERGY_CLASS",
            Characteristic::ExpirationDate => "EXPIRATION_DATE",
            Characteristic::FeeIndicator => "FEE",
            Characteristic::TypeArticle => "KIND_OF_ARTICLE",
            Characteristic::MaterialProduct => "MATERIAL",
            Characteristic::MeteringPointDesignationEGForElectricityOrGas => "METER_LOCATION",
            Characteristic::MeterNumberEGForElectricityOrGas => "METER_NUMBER",
            Characteristic::OrganicControlBodyNumber => "ORGANIC_CONTROL_BODY",
            Characteristic::PackagingMaterial => "PACKAGING_MATERIAL",
            Characteristic::TypePackagingCode => "PACKAGING_TYPE",
            Characteristic::NumberActionVariant => "PROMOTIONAL_VARIANT",
            Characteristic::SealNumber => "SEAL_NUMBER",
            Characteristic::SizeCode => "SIZE_CODE",
            Characteristic::SizeDesignation => "SIZE_TEXT",
            Characteristic::TypeTradingUnit => "TRADING_UNIT",
            Characteristic::WasteCodeEwc => "WASTE_CODE",
            Characteristic::WasteFraction => "WASTE_FRACTION",
            Characteristic::WeeeRegistrationNumber => "WEEE_NUMBER",
        }
    }
}

impl crate::Description for Characteristic {
    fn description(&self) -> &str {
        match self {
            Characteristic::ConsolidatedWeight => "Consolidated weight",
            Characteristic::NetWeight => "Net weight",
            Characteristic::GoodsItemGrossWeight => "Goods item gross weight",
            Characteristic::TotalNetWeight => "Total net weight",
            Characteristic::ConsignmentGrossWeight => "Consignment gross weight",
            Characteristic::NetNetWeight => "Net net weight",
            Characteristic::SternThrust => "Stern thrust",
            Characteristic::BowThrust => "Bow thrust",
            Characteristic::HydrateContentAnAlcoholicProductAtBottling => {
                "Hydrate content of an alcoholic product at bottling"
            }
            Characteristic::NumberUnitsPerPallet => "Number of units per pallet",
            Characteristic::FatContent => "Fat content",
            Characteristic::TransportMeansGrossWeight => "Transport means gross weight",
            Characteristic::NetTonnageVessel => "Net tonnage of the vessel",
            Characteristic::Humidity => "Humidity",
            Characteristic::Voltage => "Voltage",
            Characteristic::PowerConsumption => "Power consumption",
            Characteristic::HeatDissipation => "Heat dissipation",
            Characteristic::AirFlow => "Air flow",
            Characteristic::ShockImpact => "Shock impact",
            Characteristic::OperativeTemperature => "Operative temperature",
            Characteristic::NonOperativeTemperature => "Non operative temperature",
            Characteristic::GrossVolume => "Gross volume",
            Characteristic::NetVolume => "Net volume",
            Characteristic::WaterContent => "Water content",
            Characteristic::TensileStress => "Tensile stress",
            Characteristic::Fibrosity => "Fibrosity",
            Characteristic::GaugeLength => "Gauge length",
            Characteristic::Radius => "Radius",
            Characteristic::Straightness => "Straightness",
            Characteristic::Strain => "Strain",
            Characteristic::ItemWidthWhenUnrolled => "Item width when unrolled",
            Characteristic::ItemLengthWhenUnrolled => "Item length when unrolled",
            Characteristic::ItemAreaWhenUnrolled => "Item area when unrolled",
            Characteristic::OriginalWort => "Original wort",
            Characteristic::Volume => "Volume",
            Characteristic::Angle => "Angle",
            Characteristic::PegHoleHorizontalDistanceFromPackageLeftmostEdge => {
                "Peg hole horizontal distance from package leftmost edge"
            }
            Characteristic::PegHoleVerticalDistanceFromPackageTop => {
                "Peg hole vertical distance from package top"
            }
            Characteristic::NumberLayersPerPallet => "Number of layers per pallet",
            Characteristic::ProductStrenghChemical => "Product strengh, chemical",
            Characteristic::ProductStrengthBasisChemical => "Product strength basis, chemical",
            Characteristic::ItemWeight => "Item weight",
            Characteristic::PayloadWeightMaximum => "Payload weight, maximum",
            Characteristic::WeightConveyance => "Weight of conveyance",
            Characteristic::ConveyanceSummerDeadWeight => "Conveyance summer dead weight",
            Characteristic::ContainerizedCargoOnVesselsWeight => {
                "Containerized cargo on vessel's weight"
            }
            Characteristic::NonContainerizedCargoOnVesselsWeight => {
                "Non-containerized cargo on vessel's weight"
            }
            Characteristic::AscertainedWeight => "Ascertained weight",
            Characteristic::ChargeableWeight => "Chargeable weight",
            Characteristic::EstimatedGrossWeight => "Estimated gross weight",
            Characteristic::EstimatedVolume => "Estimated volume",
            Characteristic::VesselOverallLength => "Vessel overall length",
            Characteristic::LoadingMeters => "Loading meters",
            Characteristic::NumberAxles => "Number of axles",
            Characteristic::Payload => "Payload",
            Characteristic::StartPositionInLength => "Start position in the length",
            Characteristic::EndPositionInLength => "End position in the length",
            Characteristic::StartPositionInWidth => "Start position in the width",
            Characteristic::EndPositionInWidth => "End position in the width",
            Characteristic::StartPositionInThickness => "Start position in the thickness",
            Characteristic::EndPositionInThickness => "End position in the thickness",
            Characteristic::TransportContainerActualFillingWeight => {
                "Transport container actual filling weight"
            }
            Characteristic::TransportContainerMaximumCapacity => {
                "Transport container maximum capacity"
            }
            Characteristic::DeclaredNetWeight => "Declared net weight",
            Characteristic::LoadingHeight => "Loading height",
            Characteristic::StackingHeight => "Stacking height",
            Characteristic::CalculatedWeight => "Calculated weight",
            Characteristic::Ferrite => "Ferrite",
            Characteristic::Impurity => "Impurity",
            Characteristic::GrainSize => "Grain size",
            Characteristic::Lanthanides => "Lanthanides",
            Characteristic::Elasticity => "Elasticity",
            Characteristic::DrainedWeight => "Drained weight",
            Characteristic::Gallium => "Gallium",
            Characteristic::Strontium => "Strontium",
            Characteristic::Area => "Area",
            Characteristic::EquipmentStorageLimitation => "Equipment storage limitation",
            Characteristic::RadioactiveIndexTransport => "Radioactive index of transport",
            Characteristic::Radioactivity => "Radioactivity",
            Characteristic::AverageGrossWeight => "Average gross weight",
            Characteristic::ForwardDraft => "Forward draft",
            Characteristic::AfterDraft => "After draft",
            Characteristic::Acidity => "Acidity",
            Characteristic::TransportEquipmentGrossWeight => "Transport equipment gross weight",
            Characteristic::TotalTransportEquipmentGrossWeight => {
                "Total transport equipment gross weight"
            }
            Characteristic::AcidityJuice => "Acidity of juice",
            Characteristic::Penetrometry => "Penetrometry",
            Characteristic::Durofel => "Durofel",
            Characteristic::JuiceWeightPer100Grams => "Juice weight per 100 grams",
            Characteristic::FruitSkinColour => "Fruit skin colour",
            Characteristic::AngleBend => "Angle of bend",
            Characteristic::FixedIncrementalMeasurement => "Fixed incremental measurement",
            Characteristic::DurofelD10 => "Durofel D10",
            Characteristic::DurofelD25 => "Durofel D25",
            Characteristic::DurofelD50 => "Durofel D50",
            Characteristic::MaximumStackingWeight => "Maximum stacking weight",
            Characteristic::GrossMeasureCube => "Gross measure cube",
            Characteristic::PercentageFatContentInDryMatter => {
                "Percentage fat content in dry matter"
            }
            Characteristic::SaccharometricContent => "Saccharometric content",
            Characteristic::HydrateContentAnAlcoholicProductAfterBottling => {
                "Hydrate content of an alcoholic product after bottling"
            }
            Characteristic::AnhydrousContent => "Anhydrous content",
            Characteristic::CertifiedWeight => "Certified weight",
            Characteristic::Freeboard => "Freeboard",
            Characteristic::MaximumVesselDraught => "Maximum vessel draught",
            Characteristic::NetExplosiveWeight => "Net explosive weight",
            Characteristic::RadioactiveCriticalitySafetyIndex => {
                "Radioactive criticality safety index"
            }
            Characteristic::WasteCurrentlyOnBoard => "Waste currently on board",
            Characteristic::WasteToBeDeliveredAtWasteReceptionFacility => {
                "Waste to be delivered at waste reception facility"
            }
            Characteristic::WasteToBeGeneratedUntilNextPortCallEstimated => {
                "Waste to be generated until next port of call, estimated"
            }
            Characteristic::WasteRemainingOnBoardAtDeparture => {
                "Waste remaining on board at departure"
            }
            Characteristic::ColourDepth => "Colour depth",
            Characteristic::ColourDepthMaximum => "Colour depth, maximum",
            Characteristic::ImageResolution => "Image resolution",
            Characteristic::DeviceResolutionMaximum => "Device resolution, maximum",
            Characteristic::AcousticAbsorptionCoefficient => "Acoustic absorption coefficient",
            Characteristic::BilledWeight => "Billed weight",
            Characteristic::BreakingLoad => "Breaking load",
            Characteristic::Platinum => "Platinum",
            Characteristic::Silver => "Silver",
            Characteristic::List => "List",
            Characteristic::Trim => "Trim",
            Characteristic::FreeWater => "Free water",
            Characteristic::Bands => "Bands",
            Characteristic::ApiAmericanPetroleumInstituteGravity => {
                "API (American Petroleum Institute) gravity"
            }
            Characteristic::PetroleumGrossObservedVolume => "Petroleum gross observed volume",
            Characteristic::PetroleumGrossStandardVolume => "Petroleum gross standard volume",
            Characteristic::VolumeVariance => "Volume variance",
            Characteristic::PetroleumNetStandardVolume => "Petroleum net standard volume",
            Characteristic::MaterialOnBoardQuantityAfterDischarge => {
                "Material on-board quantity, after discharge"
            }
            Characteristic::PetroleumTotalCalculatedVolume => "Petroleum total calculated volume",
            Characteristic::PetroleumTotalObservedVolume => "Petroleum total observed volume",
            Characteristic::InnageGaugeDistance => "Innage gauge distance",
            Characteristic::PetroleumNetStandardWeight => "Petroleum net standard weight",
            Characteristic::SedimentAndWaterInPetroleum => "Sediment and water in petroleum",
            Characteristic::ObservedReferenceHeightTank => "Observed reference height, tank",
            Characteristic::ReferenceHeightTank => "Reference height, tank",
            Characteristic::UllageGaugeDistance => "Ullage gauge distance",
            Characteristic::TrimCorrection => "Trim correction",
            Characteristic::BowToBridgeDistance => "Bow to bridge distance",
            Characteristic::PegHoleNumber => "Peg hole number",
            Characteristic::NumberInnerPacks => "Number of inner packs",
            Characteristic::NumberNextLevelTradeItemsWithinInnerPack => {
                "Number of next level trade items within inner pack"
            }
            Characteristic::NumberTradeItemsPerPalletLayer => {
                "Number of trade items per pallet layer"
            }
            Characteristic::PackedItemsLayerHeight => "Packed items layer height",
            Characteristic::PackingMaterialWeightSkinTightCovering => {
                "Packing material weight, skin tight covering"
            }
            Characteristic::Brightness => "Brightness",
            Characteristic::Brakes => "Brakes",
            Characteristic::ComponentsLabelledForRecyclingPercentage => {
                "Components labelled for recycling percentage"
            }
            Characteristic::RenewablePlasticComponentsPercentageByNetWeight => {
                "Renewable plastic components percentage, by net weight"
            }
            Characteristic::ClampPressureRequired => "Clamp pressure, required",
            Characteristic::Break => "Break",
            Characteristic::AscertainedVolume => "Ascertained volume",
            Characteristic::UnitWeight => "Unit weight",
            Characteristic::TotalVolume => "Total volume",
            Characteristic::UnitVolume => "Unit volume",
            Characteristic::VerticalCenterGravity => "Vertical center of gravity",
            Characteristic::MaximumAllowableTransportStackingWeight => {
                "Maximum allowable transport stacking weight"
            }
            Characteristic::CarbonDioxide => "Carbon Dioxide",
            Characteristic::NumberBaseUnitsPerPallet => "Number of base units per pallet",
            Characteristic::ColonyFormingUnit => "Colony forming unit",
            Characteristic::DilutedLiquidVolume => "Diluted liquid volume",
            Characteristic::EnergyEfficiency => "Energy efficiency",
            Characteristic::NumberLayers => "Number of layers",
            Characteristic::MaximumDemand => "Maximum demand",
            Characteristic::NumberPalletPlaces => "Number of pallet places",
            Characteristic::PackageNetMeasurementCubed => "Package net measurement, cubed",
            Characteristic::PowerFactor => "Power factor",
            Characteristic::StackingFactorExcludingBottomItem => {
                "Stacking factor excluding bottom item"
            }
            Characteristic::BreakingStrength => "Breaking strength",
            Characteristic::BreakingStrengthWet => "Breaking strength wet",
            Characteristic::StepSize => "Step size",
            Characteristic::NumberUnitsPerPackage => "Number of units per package",
            Characteristic::NumberUnitsPerLayer => "Number of units per layer",
            Characteristic::WeightPerRunningMetre => "Weight per running metre",
            Characteristic::WeightPerSquareMetre => "Weight per square metre",
            Characteristic::AcidityMeat => "Acidity of meat",
            Characteristic::SlaughteringWeight => "Slaughtering weight",
            Characteristic::StackingFactorIncludingBottomItem => {
                "Stacking factor including bottom item"
            }
            Characteristic::NumberUnitsInWidthALayer => "Number of units in the width of a layer",
            Characteristic::NumberUnitsInDepthALayer => "Number of units in the depth of a layer",
            Characteristic::NestablePercentage => "Nestable percentage",
            Characteristic::GrossWeightIncludingCarriersEquipment => {
                "Gross weight including carrier's equipment"
            }
            Characteristic::SugarContent => "Sugar content",
            Characteristic::SelfAcceleratingPolymerizationTemperatureSapt => {
                "Self-accelerating polymerization temperature (SAPT)"
            }
            Characteristic::SelfAcceleratingDecompositionTemperatureSadt => {
                "Self-accelerating decomposition temperature (SADT)"
            }
            Characteristic::ControlTemperature => "Control temperature",
            Characteristic::BasisWeight => "Basis weight",
            Characteristic::Change => "Change",
            Characteristic::EmergencyTemperature => "Emergency temperature",
            Characteristic::Colour => "Colour",
            Characteristic::ContentsPackage => "Contents of package",
            Characteristic::CommercialWeight => "Commercial weight",
            Characteristic::CoreLength => "Core length",
            Characteristic::DestinationWeightAgreement => "Destination weight agreement",
            Characteristic::Diameter => "Diameter",
            Characteristic::DeltaValueL => "Delta value L",
            Characteristic::Density => "Density",
            Characteristic::Depth => "Depth",
            Characteristic::Denier => "Denier",
            Characteristic::DistanceBetweenPoints => "Distance between points",
            Characteristic::WidthBoxcarDoor => "Width, boxcar door",
            Characteristic::EstimatedNewWeight => "Estimated new weight",
            Characteristic::Elongation => "Elongation",
            Characteristic::DeficitWeight => "Deficit weight",
            Characteristic::FilamentCount => "Filament count",
            Characteristic::LongitudinalFlatness => "Longitudinal flatness",
            Characteristic::Flatness => "Flatness",
            Characteristic::TransverseFlatness => "Transverse flatness",
            Characteristic::Gauge => "Gauge",
            Characteristic::GrossWeightMaximum => "Gross weight, maximum",
            Characteristic::Hardness => "Hardness",
            Characteristic::HeightMaximum => "Height, maximum",
            Characteristic::HeightDimension => "Height dimension",
            Characteristic::ImpactEnergy => "Impact energy",
            Characteristic::InsideDiameter => "Inside diameter",
            Characteristic::LegalWeight => "Legal weight",
            Characteristic::LengthMaximum => "Length, maximum",
            Characteristic::Length => "Length",
            Characteristic::LostEnd => "Lost end",
            Characteristic::MinimumWeight => "Minimum weight",
            Characteristic::Moisture => "Moisture",
            Characteristic::MaximumWeight => "Maximum weight",
            Characteristic::ActualNetWeight => "Actual net weight",
            Characteristic::OutsideDiameter => "Outside diameter",
            Characteristic::PreStretch => "Pre stretch",
            Characteristic::PerTonne => "Per tonne",
            Characteristic::RelativeHumidity => "Relative humidity",
            Characteristic::Resistivity => "Resistivity",
            Characteristic::RockwellC => "Rockwell C",
            Characteristic::ReamWeight => "Ream weight",
            Characteristic::ReductionArea => "Reduction of area",
            Characteristic::RunProcess => "Run (process)",
            Characteristic::Ratio => "Ratio",
            Characteristic::ShippedQuantity => "Shipped quantity",
            Characteristic::TareWeight => "Tare weight",
            Characteristic::Temperature => "Temperature",
            Characteristic::Thickness => "Thickness",
            Characteristic::TimePeriod => "Time period",
            Characteristic::Time => "Time",
            Characteristic::TransportEquipmentVerifiedGrossMassWeight => {
                "Transport equipment verified gross mass (weight)"
            }
            Characteristic::HeightVanDoor => "Height, van door",
            Characteristic::WidthVanDoor => "Width, van door",
            Characteristic::WeightPerUnitArea => "Weight per unit of area",
            Characteristic::WidthDimension => "Width dimension",
            Characteristic::WidthMaximum => "Width, maximum",
            Characteristic::WeightPerUnitLength => "Weight per unit of length",
            Characteristic::SideHeightFlatBedWithRemovableSides => {
                "Side height, flat bed with removable sides"
            }
            Characteristic::Squareness => "Squareness",
            Characteristic::SpoolSize => "Spool size",
            Characteristic::YieldStress => "Yield stress",
            Characteristic::Aluminium => "Aluminium",
            Characteristic::Arsenic => "Arsenic",
            Characteristic::Boron => "Boron",
            Characteristic::Bismuth => "Bismuth",
            Characteristic::Carbon => "Carbon",
            Characteristic::Calcium => "Calcium",
            Characteristic::Columbium => "Columbium",
            Characteristic::Cerium => "Cerium",
            Characteristic::Chlorine => "Chlorine",
            Characteristic::Cobalt => "Cobalt",
            Characteristic::Chromium => "Chromium",
            Characteristic::Copper => "Copper",
            Characteristic::Iron => "Iron",
            Characteristic::IronPlusSilicon => "Iron plus silicon",
            Characteristic::Germanium => "Germanium",
            Characteristic::Hydrogen => "Hydrogen",
            Characteristic::Potassium => "Potassium",
            Characteristic::Magnesium => "Magnesium",
            Characteristic::Manganese => "Manganese",
            Characteristic::Molybdenum => "Molybdenum",
            Characteristic::Nitrogen => "Nitrogen",
            Characteristic::Sodium => "Sodium",
            Characteristic::Niobium => "Niobium",
            Characteristic::Nickel => "Nickel",
            Characteristic::Oxygen => "Oxygen",
            Characteristic::Phosphorus => "Phosphorus",
            Characteristic::Lead => "Lead",
            Characteristic::Sulphur => "Sulphur",
            Characteristic::Antimony => "Antimony",
            Characteristic::Selenium => "Selenium",
            Characteristic::Silicon => "Silicon",
            Characteristic::SiliciumOxyd => "Silicium oxyd",
            Characteristic::Tin => "Tin",
            Characteristic::Tantalium => "Tantalium",
            Characteristic::Tellurium => "Tellurium",
            Characteristic::Titanium => "Titanium",
            Characteristic::Vanadium => "Vanadium",
            Characteristic::Tungsten => "Tungsten",
            Characteristic::WasteContent => "Waste content",
            Characteristic::Zinc => "Zinc",
            Characteristic::Zirconium => "Zirconium",
            Characteristic::MutuallyDefined => "Mutually defined",
            Characteristic::BestBeforeDate => "Best before date",
            Characteristic::ColourAsText => "Colour as text",
            Characteristic::CommissionIndicator => "Commission indicator",
            Characteristic::DepositSystem => "Deposit system",
            Characteristic::DepositType => "Deposit type",
            Characteristic::EnergyEfficiencyClass => "Energy efficiency class",
            Characteristic::ExpirationDate => "Expiration date",
            Characteristic::FeeIndicator => "Fee indicator",
            Characteristic::TypeArticle => "Type of article",
            Characteristic::MaterialProduct => "Material of the product",
            Characteristic::MeteringPointDesignationEGForElectricityOrGas => {
                "Metering point designation, e.g. for electricity or gas"
            }
            Characteristic::MeterNumberEGForElectricityOrGas => {
                "Meter number, e.g. for electricity or gas"
            }
            Characteristic::OrganicControlBodyNumber => "Organic control body number",
            Characteristic::PackagingMaterial => "Packaging material",
            Characteristic::TypePackagingCode => "Type of packaging (code)",
            Characteristic::NumberActionVariant => "Number of the action variant",
            Characteristic::SealNumber => "Seal number",
            Characteristic::SizeCode => "Size code",
            Characteristic::SizeDesignation => "Size designation",
            Characteristic::TypeTradingUnit => "Type of trading unit",
            Characteristic::WasteCodeEwc => "Waste code (EWC)",
            Characteristic::WasteFraction => "Waste fraction",
            Characteristic::WeeeRegistrationNumber => "WEEE registration number",
        }
    }
}

impl crate::FromCode for Characteristic {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "A" => Some(Characteristic::ConsolidatedWeight),
            "AAA" => Some(Characteristic::NetWeight),
            "AAB" => Some(Characteristic::GoodsItemGrossWeight),
            "AAC" => Some(Characteristic::TotalNetWeight),
            "AAD" => Some(Characteristic::ConsignmentGrossWeight),
            "AAF" => Some(Characteristic::NetNetWeight),
            "AAG" => Some(Characteristic::SternThrust),
            "AAH" => Some(Characteristic::BowThrust),
            "AAI" => Some(Characteristic::HydrateContentAnAlcoholicProductAtBottling),
            "AAJ" => Some(Characteristic::NumberUnitsPerPallet),
            "AAK" => Some(Characteristic::FatContent),
            "AAM" => Some(Characteristic::TransportMeansGrossWeight),
            "AAN" => Some(Characteristic::NetTonnageVessel),
            "AAO" => Some(Characteristic::Humidity),
            "AAP" => Some(Characteristic::Voltage),
            "AAQ" => Some(Characteristic::PowerConsumption),
            "AAR" => Some(Characteristic::HeatDissipation),
            "AAS" => Some(Characteristic::AirFlow),
            "AAT" => Some(Characteristic::ShockImpact),
            "AAU" => Some(Characteristic::OperativeTemperature),
            "AAV" => Some(Characteristic::NonOperativeTemperature),
            "AAW" => Some(Characteristic::GrossVolume),
            "AAX" => Some(Characteristic::NetVolume),
            "AAY" => Some(Characteristic::WaterContent),
            "AAZ" => Some(Characteristic::TensileStress),
            "ABA" => Some(Characteristic::Fibrosity),
            "ABB" => Some(Characteristic::GaugeLength),
            "ABC" => Some(Characteristic::Radius),
            "ABD" => Some(Characteristic::Straightness),
            "ABE" => Some(Characteristic::Strain),
            "ABF" => Some(Characteristic::ItemWidthWhenUnrolled),
            "ABG" => Some(Characteristic::ItemLengthWhenUnrolled),
            "ABH" => Some(Characteristic::ItemAreaWhenUnrolled),
            "ABI" => Some(Characteristic::OriginalWort),
            "ABJ" => Some(Characteristic::Volume),
            "ABK" => Some(Characteristic::Angle),
            "ABL" => Some(Characteristic::PegHoleHorizontalDistanceFromPackageLeftmostEdge),
            "ABM" => Some(Characteristic::PegHoleVerticalDistanceFromPackageTop),
            "ABN" => Some(Characteristic::NumberLayersPerPallet),
            "ABO" => Some(Characteristic::ProductStrenghChemical),
            "ABP" => Some(Characteristic::ProductStrengthBasisChemical),
            "ABS" => Some(Characteristic::ItemWeight),
            "ABT" => Some(Characteristic::PayloadWeightMaximum),
            "ABX" => Some(Characteristic::WeightConveyance),
            "ABY" => Some(Characteristic::ConveyanceSummerDeadWeight),
            "ABZ" => Some(Characteristic::ContainerizedCargoOnVesselsWeight),
            "ACA" => Some(Characteristic::NonContainerizedCargoOnVesselsWeight),
            "ACE" => Some(Characteristic::AscertainedWeight),
            "ACG" => Some(Characteristic::ChargeableWeight),
            "ACN" => Some(Characteristic::EstimatedGrossWeight),
            "ACP" => Some(Characteristic::EstimatedVolume),
            "ACS" => Some(Characteristic::VesselOverallLength),
            "ACV" => Some(Characteristic::LoadingMeters),
            "ACW" => Some(Characteristic::NumberAxles),
            "ACX" => Some(Characteristic::Payload),
            "ADR" => Some(Characteristic::StartPositionInLength),
            "ADS" => Some(Characteristic::EndPositionInLength),
            "ADT" => Some(Characteristic::StartPositionInWidth),
            "ADU" => Some(Characteristic::EndPositionInWidth),
            "ADV" => Some(Characteristic::StartPositionInThickness),
            "ADW" => Some(Characteristic::EndPositionInThickness),
            "ADX" => Some(Characteristic::TransportContainerActualFillingWeight),
            "ADY" => Some(Characteristic::TransportContainerMaximumCapacity),
            "ADZ" => Some(Characteristic::DeclaredNetWeight),
            "AEA" => Some(Characteristic::LoadingHeight),
            "AEB" => Some(Characteristic::StackingHeight),
            "AEC" => Some(Characteristic::CalculatedWeight),
            "AED" => Some(Characteristic::Ferrite),
            "AEE" => Some(Characteristic::Impurity),
            "AEF" => Some(Characteristic::GrainSize),
            "AEG" => Some(Characteristic::Lanthanides),
            "AEH" => Some(Characteristic::Elasticity),
            "AEI" => Some(Characteristic::DrainedWeight),
            "AEJ" => Some(Characteristic::Gallium),
            "AEK" => Some(Characteristic::Strontium),
            "AEL" => Some(Characteristic::Area),
            "AEM" => Some(Characteristic::EquipmentStorageLimitation),
            "AEN" => Some(Characteristic::RadioactiveIndexTransport),
            "AEO" => Some(Characteristic::Radioactivity),
            "AEP" => Some(Characteristic::AverageGrossWeight),
            "AEQ" => Some(Characteristic::ForwardDraft),
            "AER" => Some(Characteristic::AfterDraft),
            "AES" => Some(Characteristic::Acidity),
            "AET" => Some(Characteristic::TransportEquipmentGrossWeight),
            "AEU" => Some(Characteristic::TotalTransportEquipmentGrossWeight),
            "AEV" => Some(Characteristic::AcidityJuice),
            "AEW" => Some(Characteristic::Penetrometry),
            "AEX" => Some(Characteristic::Durofel),
            "AEY" => Some(Characteristic::JuiceWeightPer100Grams),
            "AEZ" => Some(Characteristic::FruitSkinColour),
            "AF" => Some(Characteristic::AngleBend),
            "AFA" => Some(Characteristic::FixedIncrementalMeasurement),
            "AFB" => Some(Characteristic::DurofelD10),
            "AFC" => Some(Characteristic::DurofelD25),
            "AFD" => Some(Characteristic::DurofelD50),
            "AFE" => Some(Characteristic::MaximumStackingWeight),
            "AFF" => Some(Characteristic::GrossMeasureCube),
            "AFG" => Some(Characteristic::PercentageFatContentInDryMatter),
            "AFH" => Some(Characteristic::SaccharometricContent),
            "AFI" => Some(Characteristic::HydrateContentAnAlcoholicProductAfterBottling),
            "AFJ" => Some(Characteristic::AnhydrousContent),
            "AFK" => Some(Characteristic::CertifiedWeight),
            "AFL" => Some(Characteristic::Freeboard),
            "AFM" => Some(Characteristic::MaximumVesselDraught),
            "AFN" => Some(Characteristic::NetExplosiveWeight),
            "AFO" => Some(Characteristic::RadioactiveCriticalitySafetyIndex),
            "AFP" => Some(Characteristic::WasteCurrentlyOnBoard),
            "AFQ" => Some(Characteristic::WasteToBeDeliveredAtWasteReceptionFacility),
            "AFR" => Some(Characteristic::WasteToBeGeneratedUntilNextPortCallEstimated),
            "AFS" => Some(Characteristic::WasteRemainingOnBoardAtDeparture),
            "AFT" => Some(Characteristic::ColourDepth),
            "AFU" => Some(Characteristic::ColourDepthMaximum),
            "AFV" => Some(Characteristic::ImageResolution),
            "AFW" => Some(Characteristic::DeviceResolutionMaximum),
            "AFX" => Some(Characteristic::AcousticAbsorptionCoefficient),
            "B" => Some(Characteristic::BilledWeight),
            "BL" => Some(Characteristic::BreakingLoad),
            "BMY" => Some(Characteristic::Platinum),
            "BMZ" => Some(Characteristic::Silver),
            "BNA" => Some(Characteristic::List),
            "BNB" => Some(Characteristic::Trim),
            "BNC" => Some(Characteristic::FreeWater),
            "BND" => Some(Characteristic::Bands),
            "BNE" => Some(Characteristic::ApiAmericanPetroleumInstituteGravity),
            "BNF" => Some(Characteristic::PetroleumGrossObservedVolume),
            "BNG" => Some(Characteristic::PetroleumGrossStandardVolume),
            "BNH" => Some(Characteristic::VolumeVariance),
            "BNI" => Some(Characteristic::PetroleumNetStandardVolume),
            "BNJ" => Some(Characteristic::MaterialOnBoardQuantityAfterDischarge),
            "BNK" => Some(Characteristic::PetroleumTotalCalculatedVolume),
            "BNL" => Some(Characteristic::PetroleumTotalObservedVolume),
            "BNM" => Some(Characteristic::InnageGaugeDistance),
            "BNN" => Some(Characteristic::PetroleumNetStandardWeight),
            "BNO" => Some(Characteristic::SedimentAndWaterInPetroleum),
            "BNP" => Some(Characteristic::ObservedReferenceHeightTank),
            "BNQ" => Some(Characteristic::ReferenceHeightTank),
            "BNR" => Some(Characteristic::UllageGaugeDistance),
            "BNS" => Some(Characteristic::TrimCorrection),
            "BNT" => Some(Characteristic::BowToBridgeDistance),
            "BNU" => Some(Characteristic::PegHoleNumber),
            "BNV" => Some(Characteristic::NumberInnerPacks),
            "BNW" => Some(Characteristic::NumberNextLevelTradeItemsWithinInnerPack),
            "BNX" => Some(Characteristic::NumberTradeItemsPerPalletLayer),
            "BNY" => Some(Characteristic::PackedItemsLayerHeight),
            "BNZ" => Some(Characteristic::PackingMaterialWeightSkinTightCovering),
            "BR" => Some(Characteristic::Brightness),
            "BRA" => Some(Characteristic::Brakes),
            "BRB" => Some(Characteristic::ComponentsLabelledForRecyclingPercentage),
            "BRC" => Some(Characteristic::RenewablePlasticComponentsPercentageByNetWeight),
            "BRD" => Some(Characteristic::ClampPressureRequired),
            "BRE" => Some(Characteristic::Break),
            "BRF" => Some(Characteristic::AscertainedVolume),
            "BRG" => Some(Characteristic::UnitWeight),
            "BRH" => Some(Characteristic::TotalVolume),
            "BRI" => Some(Characteristic::UnitVolume),
            "BRJ" => Some(Characteristic::VerticalCenterGravity),
            "BRK" => Some(Characteristic::MaximumAllowableTransportStackingWeight),
            "BRL" => Some(Characteristic::CarbonDioxide),
            "BRM" => Some(Characteristic::NumberBaseUnitsPerPallet),
            "BRN" => Some(Characteristic::ColonyFormingUnit),
            "BRO" => Some(Characteristic::DilutedLiquidVolume),
            "BRP" => Some(Characteristic::EnergyEfficiency),
            "BRQ" => Some(Characteristic::NumberLayers),
            "BRR" => Some(Characteristic::MaximumDemand),
            "BRS" => Some(Characteristic::NumberPalletPlaces),
            "BRT" => Some(Characteristic::PackageNetMeasurementCubed),
            "BRU" => Some(Characteristic::PowerFactor),
            "BRV" => Some(Characteristic::StackingFactorExcludingBottomItem),
            "BS" => Some(Characteristic::BreakingStrength),
            "BSW" => Some(Characteristic::BreakingStrengthWet),
            "BSX" => Some(Characteristic::StepSize),
            "BSY" => Some(Characteristic::NumberUnitsPerPackage),
            "BSZ" => Some(Characteristic::NumberUnitsPerLayer),
            "BTA" => Some(Characteristic::WeightPerRunningMetre),
            "BTB" => Some(Characteristic::WeightPerSquareMetre),
            "BTC" => Some(Characteristic::AcidityMeat),
            "BTD" => Some(Characteristic::SlaughteringWeight),
            "BTE" => Some(Characteristic::StackingFactorIncludingBottomItem),
            "BTF" => Some(Characteristic::NumberUnitsInWidthALayer),
            "BTG" => Some(Characteristic::NumberUnitsInDepthALayer),
            "BTH" => Some(Characteristic::NestablePercentage),
            "BTI" => Some(Characteristic::GrossWeightIncludingCarriersEquipment),
            "BTJ" => Some(Characteristic::SugarContent),
            "BTK" => Some(Characteristic::SelfAcceleratingPolymerizationTemperatureSapt),
            "BTL" => Some(Characteristic::SelfAcceleratingDecompositionTemperatureSadt),
            "BTM" => Some(Characteristic::ControlTemperature),
            "BW" => Some(Characteristic::BasisWeight),
            "CHN" => Some(Characteristic::Change),
            "CHO" => Some(Characteristic::EmergencyTemperature),
            "CM" => Some(Characteristic::Colour),
            "CT" => Some(Characteristic::ContentsPackage),
            "CV" => Some(Characteristic::CommercialWeight),
            "CZ" => Some(Characteristic::CoreLength),
            "D" => Some(Characteristic::DestinationWeightAgreement),
            "DI" => Some(Characteristic::Diameter),
            "DL" => Some(Characteristic::DeltaValueL),
            "DN" => Some(Characteristic::Density),
            "DP" => Some(Characteristic::Depth),
            "DR" => Some(Characteristic::Denier),
            "DS" => Some(Characteristic::DistanceBetweenPoints),
            "DW" => Some(Characteristic::WidthBoxcarDoor),
            "E" => Some(Characteristic::EstimatedNewWeight),
            "EA" => Some(Characteristic::Elongation),
            "F" => Some(Characteristic::DeficitWeight),
            "FI" => Some(Characteristic::FilamentCount),
            "FL" => Some(Characteristic::LongitudinalFlatness),
            "FN" => Some(Characteristic::Flatness),
            "FV" => Some(Characteristic::TransverseFlatness),
            "GG" => Some(Characteristic::Gauge),
            "GW" => Some(Characteristic::GrossWeightMaximum),
            "HF" => Some(Characteristic::Hardness),
            "HM" => Some(Characteristic::HeightMaximum),
            "HT" => Some(Characteristic::HeightDimension),
            "IB" => Some(Characteristic::ImpactEnergy),
            "ID" => Some(Characteristic::InsideDiameter),
            "L" => Some(Characteristic::LegalWeight),
            "LM" => Some(Characteristic::LengthMaximum),
            "LN" => Some(Characteristic::Length),
            "LND" => Some(Characteristic::LostEnd),
            "M" => Some(Characteristic::MinimumWeight),
            "MO" => Some(Characteristic::Moisture),
            "MW" => Some(Characteristic::MaximumWeight),
            "N" => Some(Characteristic::ActualNetWeight),
            "OD" => Some(Characteristic::OutsideDiameter),
            "PRS" => Some(Characteristic::PreStretch),
            "PTN" => Some(Characteristic::PerTonne),
            "RA" => Some(Characteristic::RelativeHumidity),
            "RF" => Some(Characteristic::Resistivity),
            "RJ" => Some(Characteristic::RockwellC),
            "RMW" => Some(Characteristic::ReamWeight),
            "RP" => Some(Characteristic::ReductionArea),
            "RUN" => Some(Characteristic::RunProcess),
            "RY" => Some(Characteristic::Ratio),
            "SQ" => Some(Characteristic::ShippedQuantity),
            "T" => Some(Characteristic::TareWeight),
            "TC" => Some(Characteristic::Temperature),
            "TH" => Some(Characteristic::Thickness),
            "TN" => Some(Characteristic::TimePeriod),
            "TT" => Some(Characteristic::Time),
            "VGM" => Some(Characteristic::TransportEquipmentVerifiedGrossMassWeight),
            "VH" => Some(Characteristic::HeightVanDoor),
            "VW" => Some(Characteristic::WidthVanDoor),
            "WA" => Some(Characteristic::WeightPerUnitArea),
            "WD" => Some(Characteristic::WidthDimension),
            "WM" => Some(Characteristic::WidthMaximum),
            "WU" => Some(Characteristic::WeightPerUnitLength),
            "XH" => Some(Characteristic::SideHeightFlatBedWithRemovableSides),
            "XQ" => Some(Characteristic::Squareness),
            "XZ" => Some(Characteristic::SpoolSize),
            "YS" => Some(Characteristic::YieldStress),
            "ZAL" => Some(Characteristic::Aluminium),
            "ZAS" => Some(Characteristic::Arsenic),
            "ZB" => Some(Characteristic::Boron),
            "ZBI" => Some(Characteristic::Bismuth),
            "ZC" => Some(Characteristic::Carbon),
            "ZCA" => Some(Characteristic::Calcium),
            "ZCB" => Some(Characteristic::Columbium),
            "ZCE" => Some(Characteristic::Cerium),
            "ZCL" => Some(Characteristic::Chlorine),
            "ZCO" => Some(Characteristic::Cobalt),
            "ZCR" => Some(Characteristic::Chromium),
            "ZCU" => Some(Characteristic::Copper),
            "ZFE" => Some(Characteristic::Iron),
            "ZFS" => Some(Characteristic::IronPlusSilicon),
            "ZGE" => Some(Characteristic::Germanium),
            "ZH" => Some(Characteristic::Hydrogen),
            "ZK" => Some(Characteristic::Potassium),
            "ZMG" => Some(Characteristic::Magnesium),
            "ZMN" => Some(Characteristic::Manganese),
            "ZMO" => Some(Characteristic::Molybdenum),
            "ZN" => Some(Characteristic::Nitrogen),
            "ZNA" => Some(Characteristic::Sodium),
            "ZNB" => Some(Characteristic::Niobium),
            "ZNI" => Some(Characteristic::Nickel),
            "ZO" => Some(Characteristic::Oxygen),
            "ZP" => Some(Characteristic::Phosphorus),
            "ZPB" => Some(Characteristic::Lead),
            "ZS" => Some(Characteristic::Sulphur),
            "ZSB" => Some(Characteristic::Antimony),
            "ZSE" => Some(Characteristic::Selenium),
            "ZSI" => Some(Characteristic::Silicon),
            "ZSL" => Some(Characteristic::SiliciumOxyd),
            "ZSN" => Some(Characteristic::Tin),
            "ZTA" => Some(Characteristic::Tantalium),
            "ZTE" => Some(Characteristic::Tellurium),
            "ZTI" => Some(Characteristic::Titanium),
            "ZV" => Some(Characteristic::Vanadium),
            "ZW" => Some(Characteristic::Tungsten),
            "ZWA" => Some(Characteristic::WasteContent),
            "ZZN" => Some(Characteristic::Zinc),
            "ZZR" => Some(Characteristic::Zirconium),
            "ZZZ" => Some(Characteristic::MutuallyDefined),
            "BEST_BEFORE_DATE" => Some(Characteristic::BestBeforeDate),
            "COLOR_TEXT" => Some(Characteristic::ColourAsText),
            "COMMISSION" => Some(Characteristic::CommissionIndicator),
            "DEPOSIT_SYSTEM" => Some(Characteristic::DepositSystem),
            "DEPOSIT_TYPE" => Some(Characteristic::DepositType),
            "ENERGY_CLASS" => Some(Characteristic::EnergyEfficiencyClass),
            "EXPIRATION_DATE" => Some(Characteristic::ExpirationDate),
            "FEE" => Some(Characteristic::FeeIndicator),
            "KIND_OF_ARTICLE" => Some(Characteristic::TypeArticle),
            "MATERIAL" => Some(Characteristic::MaterialProduct),
            "METER_LOCATION" => Some(Characteristic::MeteringPointDesignationEGForElectricityOrGas),
            "METER_NUMBER" => Some(Characteristic::MeterNumberEGForElectricityOrGas),
            "ORGANIC_CONTROL_BODY" => Some(Characteristic::OrganicControlBodyNumber),
            "PACKAGING_MATERIAL" => Some(Characteristic::PackagingMaterial),
            "PACKAGING_TYPE" => Some(Characteristic::TypePackagingCode),
            "PROMOTIONAL_VARIANT" => Some(Characteristic::NumberActionVariant),
            "SEAL_NUMBER" => Some(Characteristic::SealNumber),
            "SIZE_CODE" => Some(Characteristic::SizeCode),
            "SIZE_TEXT" => Some(Characteristic::SizeDesignation),
            "TRADING_UNIT" => Some(Characteristic::TypeTradingUnit),
            "WASTE_CODE" => Some(Characteristic::WasteCodeEwc),
            "WASTE_FRACTION" => Some(Characteristic::WasteFraction),
            "WEEE_NUMBER" => Some(Characteristic::WeeeRegistrationNumber),
            _ => None,
        }
    }
}
