#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Characteristic {
    /// Consolidated weight
    ///
    /// 6313
    ///
    /// The measured consolidated weight.
    ConsolidatedWeight,
    /// Net weight
    ///
    /// 6313
    ///
    /// Weight of goods including any packaging that normally going with the goods.
    NetWeight,
    /// Goods item gross weight
    ///
    /// 6313
    ///
    /// Weight (mass) of goods including packing but excluding the carrier's equipment.
    GoodsItemGrossWeight,
    /// Total net weight
    ///
    /// 6313
    ///
    /// Total weight of goods excluding packaging.
    TotalNetWeight,
    /// Consignment gross weight
    ///
    /// 6313
    ///
    /// Total gross weight (mass) of all goods items referred to as one consignment including packaging but excluding any transport equipment.
    ConsignmentGrossWeight,
    /// Net net weight
    ///
    /// 6313
    ///
    /// Weight (mass) if goods without any packaging.
    NetNetWeight,
    /// Stern thrust
    ///
    /// 6313
    ///
    /// Force exerted by a thruster installed at the stern of a vessel.
    SternThrust,
    /// Bow thrust
    ///
    /// 6313
    ///
    /// Force exerted by a thruster installed at the bow of a vessel.
    BowThrust,
    /// Hydrate content of an alcoholic product at bottling
    ///
    /// 6313
    ///
    /// The hydrate content of an alcoholic product at the moment of bottling.
    HydrateContentAnAlcoholicProductAtBottling,
    /// Number of units per pallet
    ///
    /// 6313
    ///
    /// The number of units contained on a pallet.
    NumberUnitsPerPallet,
    /// Fat content
    ///
    /// 6313
    ///
    /// An indication of the fat content of a product.
    FatContent,
    /// Transport means gross weight
    ///
    /// 6313
    ///
    /// The measure of the overall size of a ship determined in accordance with the provisions of the International Convention on Tonnage Measurement of Ships.
    TransportMeansGrossWeight,
    /// Net tonnage of the vessel
    ///
    /// 6313
    ///
    /// The measure of the useful capacity of a ship determined in accordance with the provisions of the International Convention on Tonnage Measurement of Ships.
    NetTonnageVessel,
    /// Humidity
    ///
    /// 6313
    ///
    /// Degree of moisture.
    Humidity,
    /// Voltage
    ///
    /// 6313
    ///
    /// Electromotive force, or difference of electronic potential between two points.
    Voltage,
    /// Power consumption
    ///
    /// 6313
    ///
    /// Value of energy consumption.
    PowerConsumption,
    /// Heat dissipation
    ///
    /// 6313
    ///
    /// Measurement of the rate of dispersal of heat.
    HeatDissipation,
    /// Air flow
    ///
    /// 6313
    ///
    /// Measurement of the flow of air.
    AirFlow,
    /// Shock impact
    ///
    /// 6313
    ///
    /// Measurement of the impact of a shock.
    ShockImpact,
    /// Operative temperature
    ///
    /// 6313
    ///
    /// Temperature identified system or process works according to specifications.
    OperativeTemperature,
    /// Non operative temperature
    ///
    /// 6313
    ///
    /// Temperature identified system or process does not work according to specifications.
    NonOperativeTemperature,
    /// Gross volume
    ///
    /// 6313
    ///
    /// The volume unadjusted for factors such as temperature or gravity.
    GrossVolume,
    /// Net volume
    ///
    /// 6313
    ///
    /// The volume after adjustment for factors such as temperature or gravity.
    NetVolume,
    /// Water content
    ///
    /// 6313
    ///
    /// Water content in product.
    WaterContent,
    /// Tensile stress
    ///
    /// 6313
    ///
    /// The measured tensile stress.
    TensileStress,
    /// Fibrosity
    ///
    /// 6313
    ///
    /// The measured fibrosity.
    Fibrosity,
    /// Gauge length
    ///
    /// 6313
    ///
    /// The measured gauge length.
    GaugeLength,
    /// Radius
    ///
    /// 6313
    ///
    /// The measured radius.
    Radius,
    /// Straightness
    ///
    /// 6313
    ///
    /// Straightness of the item.
    Straightness,
    /// Strain
    ///
    /// 6313
    ///
    /// The measured strain.
    Strain,
    /// Item width when unrolled
    ///
    /// 6313
    ///
    /// The width of an item when unrolled.
    ItemWidthWhenUnrolled,
    /// Item length when unrolled
    ///
    /// 6313
    ///
    /// The length of an item when unrolled.
    ItemLengthWhenUnrolled,
    /// Item area when unrolled
    ///
    /// 6313
    ///
    /// The area occupied by an item when unrolled.
    ItemAreaWhenUnrolled,
    /// Original wort
    ///
    /// 6313
    ///
    /// Measure of the malt and hops content of beer, before fermentation has taken place.
    OriginalWort,
    /// Volume
    ///
    /// 6313
    ///
    /// The amount of air space taken up by the entity identified in the 6311 qualifier.
    Volume,
    /// Angle
    ///
    /// 6313
    ///
    /// The angle of an object.
    Angle,
    /// Peg hole horizontal distance from package leftmost edge
    ///
    /// 6313
    ///
    /// Horizontal distance from the left most edge of the package to the center of the hole into which the peg is inserted.
    PegHoleHorizontalDistanceFromPackageLeftmostEdge,
    /// Peg hole vertical distance from package top
    ///
    /// 6313
    ///
    /// Vertical distance from the top of the package to the top of the hole into which the peg is inserted.
    PegHoleVerticalDistanceFromPackageTop,
    /// Number of layers per pallet
    ///
    /// 6313
    ///
    /// The number of layers per pallet.
    NumberLayersPerPallet,
    /// Product strengh, chemical
    ///
    /// 6313
    ///
    /// The amount of the single active chemical ingredient within a product.
    ProductStrenghChemical,
    /// Product strength basis, chemical
    ///
    /// 6313
    ///
    /// Amount of product used as the basis for the specification of the chemical product strenght.
    ProductStrengthBasisChemical,
    /// Item weight
    ///
    /// 6313
    ///
    /// Weight at line item level.
    ItemWeight,
    /// Payload weight, maximum
    ///
    /// 6313
    ///
    /// The maximum weight of a payload.
    PayloadWeightMaximum,
    /// Weight of conveyance
    ///
    /// 6313
    ///
    /// Tonnage of conveyance.
    WeightConveyance,
    /// Conveyance summer dead weight
    ///
    /// 6313
    ///
    /// Registered summer dead weight total tonnage of the vessel.
    ConveyanceSummerDeadWeight,
    /// Containerized cargo on vessel's weight
    ///
    /// 6313
    ///
    /// Total weight of containerized cargo on vessel.
    ContainerizedCargoOnVesselsWeight,
    /// Non-containerized cargo on vessel's weight
    ///
    /// 6313
    ///
    /// Total weight of non-containerized cargo on vessel.
    NonContainerizedCargoOnVesselsWeight,
    /// Ascertained weight
    ///
    /// 6313
    ///
    /// Endorsement of the true weight (mass) as ascertained or verified.
    AscertainedWeight,
    /// Chargeable weight
    ///
    /// 6313
    ///
    /// The weight on which charges are based.
    ChargeableWeight,
    /// Estimated gross weight
    ///
    /// 6313
    ///
    /// Estimated weight (mass) of goods, including packing and excluding carrier's.
    EstimatedGrossWeight,
    /// Estimated volume
    ///
    /// 6313
    ///
    /// Estimated size or measure of anything in three dimensions.
    EstimatedVolume,
    /// Vessel overall length
    ///
    /// 6313
    ///
    /// Total overall length of the vessel.
    VesselOverallLength,
    /// Loading meters
    ///
    /// 6313
    ///
    /// The length in a vehicle, whereby the complete width and height over that length is needed for the goods.
    LoadingMeters,
    /// Number of axles
    ///
    /// 6313
    ///
    /// Number of axles of movable equipment or means of transport on wheels.
    NumberAxles,
    /// Payload
    ///
    /// 6313
    ///
    /// The revenue-producing load carried by a means of transport.
    Payload,
    /// Start position in the length
    ///
    /// 6313
    ///
    /// The starting position from the beginning of an item located in the length direction.
    StartPositionInLength,
    /// End position in the length
    ///
    /// 6313
    ///
    /// The end position from the beginning of an item located in the length direction.
    EndPositionInLength,
    /// Start position in the width
    ///
    /// 6313
    ///
    /// The start position from the beginning of an item located in the width direction.
    StartPositionInWidth,
    /// End position in the width
    ///
    /// 6313
    ///
    /// The end position from the beginning of an item located in the width direction.
    EndPositionInWidth,
    /// Start position in the thickness
    ///
    /// 6313
    ///
    /// The start position from the beginning of an item located in the thickness direction.
    StartPositionInThickness,
    /// End position in the thickness
    ///
    /// 6313
    ///
    /// The end position from the beginning of an item located in the thickness direction.
    EndPositionInThickness,
    /// Transport container actual filling weight
    ///
    /// 6313
    ///
    /// Actual filling weight of a transport container.
    TransportContainerActualFillingWeight,
    /// Transport container maximum capacity
    ///
    /// 6313
    ///
    /// Maximum capacity of a transport container.
    TransportContainerMaximumCapacity,
    /// Declared net weight
    ///
    /// 6313
    ///
    /// The declared net weight of a product or products used for invoicing, customs or transport purposes.
    DeclaredNetWeight,
    /// Loading height
    ///
    /// 6313
    ///
    /// Maximum height of products or packages loaded onto a given transportation device or equipment such as a pallet.
    LoadingHeight,
    /// Stacking height
    ///
    /// 6313
    ///
    /// Maximum height up to which the same product or package may be placed one upon the other for storage purposes.
    StackingHeight,
    /// Calculated weight
    ///
    /// 6313
    ///
    /// The calculated weight of the item based on the ordered dimensions.
    CalculatedWeight,
    /// Ferrite
    ///
    /// 6313
    ///
    /// The chemical composition ferrite.
    Ferrite,
    /// Impurity
    ///
    /// 6313
    ///
    /// The impurity of the product i.e. the measurement of other chemical elements not normally appearing in a product.
    Impurity,
    /// Grain size
    ///
    /// 6313
    ///
    /// The grain size.
    GrainSize,
    /// Lanthanides
    ///
    /// 6313
    ///
    /// The chemical element Lanthanides.
    Lanthanides,
    /// Elasticity
    ///
    /// 6313
    ///
    /// The value of the elasticity.
    Elasticity,
    /// Drained weight
    ///
    /// 6313
    ///
    /// The weight of a product when all liquids used in the packaging of the product have been removed.
    DrainedWeight,
    /// Gallium
    ///
    /// 6313
    ///
    /// Measurement of the gallium component.
    Gallium,
    /// Strontium
    ///
    /// 6313
    ///
    /// Measurement of the strontium component.
    Strontium,
    /// Area
    ///
    /// 6313
    ///
    /// Extent or measure of a surface.
    Area,
    /// Equipment storage limitation
    ///
    /// 6313
    ///
    /// Maximum storage limit of the equipment.
    EquipmentStorageLimitation,
    /// Radioactive index of transport
    ///
    /// 6313
    ///
    /// The index of transport determines the maximum radiation level at a distance of 1m from the external surface.
    RadioactiveIndexTransport,
    /// Radioactivity
    ///
    /// 6313
    ///
    /// Activity of radioactive material.
    Radioactivity,
    /// Average gross weight
    ///
    /// 6313
    ///
    /// Weight which is the outcome of the division of the total gross weight by the number of units.
    AverageGrossWeight,
    /// Forward draft
    ///
    /// 6313
    ///
    /// Depth of water from the surface of water to the bottom of the vessel measured at the draft mark of the bow.
    ForwardDraft,
    /// After draft
    ///
    /// 6313
    ///
    /// Depth of water from the surface of water to the bottom of the vessel measured at the draft mark of the stern.
    AfterDraft,
    /// Acidity
    ///
    /// 6313
    ///
    /// The quality, state, or degree of being acid.
    Acidity,
    /// Transport equipment gross weight
    ///
    /// 6313
    ///
    /// Weight of a transport equipment including the cargo and carrier's equipment.
    TransportEquipmentGrossWeight,
    /// Total transport equipment gross weight
    ///
    /// 6313
    ///
    /// Total weight of all transport equipment including the cargo and carrier's equipment.
    TotalTransportEquipmentGrossWeight,
    /// Acidity of juice
    ///
    /// 6313
    ///
    /// Acid measurement of juice.
    AcidityJuice,
    /// Penetrometry
    ///
    /// 6313
    ///
    /// Measurement of force required to drive a standard penetrating stamp.
    Penetrometry,
    /// Durofel
    ///
    /// 6313
    ///
    /// Measurement of the elastic force using a standard penetrating stamp.
    Durofel,
    /// Juice weight per 100 grams
    ///
    /// 6313
    ///
    /// Measurement of weight of juice, based on 100 grams of the entire weight.
    JuiceWeightPer100Grams,
    /// Fruit skin colour
    ///
    /// 6313
    ///
    /// Measurement of the colouring of the epidermis of a fruit.
    FruitSkinColour,
    /// Angle of bend
    ///
    /// 6313
    ///
    /// The measured angle of bend.
    AngleBend,
    /// Fixed incremental measurement
    ///
    /// 6313
    ///
    /// The measurement of the fixed increment.
    FixedIncrementalMeasurement,
    /// Durofel D10
    ///
    /// 6313
    ///
    /// Measure of the elastic force of the pulp of a fruit. It is measured with a penetrating stamp with diameter 10.
    DurofelD10,
    /// Durofel D25
    ///
    /// 6313
    ///
    /// Measure of the elastic force of the pulp of a fruit. It is measured with a penetrating stamp with diameter 25.
    DurofelD25,
    /// Durofel D50
    ///
    /// 6313
    ///
    /// Measure of the elastic force of the pulp of a fruit. It is measured with a penetrating stamp with diameter 50.
    DurofelD50,
    /// Maximum stacking weight
    ///
    /// 6313
    ///
    /// The maximum weight which may be stacked upon a product or package without the product or packaging being crushed.
    MaximumStackingWeight,
    /// Gross measure cube
    ///
    /// 6313
    ///
    /// The total cubic space occupied by an item, taking into account any protruding components, arrived at by multiplying the maximum length, width and height.
    GrossMeasureCube,
    /// Percentage fat content in dry matter
    ///
    /// 6313
    ///
    /// The percentage of fat content in dry matter.
    PercentageFatContentInDryMatter,
    /// Saccharometric content
    ///
    /// 6313
    ///
    /// Measurement of the sugar content of a solution.
    SaccharometricContent,
    /// Hydrate content of an alcoholic product after bottling
    ///
    /// 6313
    ///
    /// The hydrate content which occurs in an alcoholic product after bottling.
    HydrateContentAnAlcoholicProductAfterBottling,
    /// Anhydrous content
    ///
    /// 6313
    ///
    /// The non-water content.
    AnhydrousContent,
    /// Certified weight
    ///
    /// 6313
    ///
    /// Weight which has been certified.
    CertifiedWeight,
    /// Freeboard
    ///
    /// 6313
    ///
    /// The vertical distance between the waterline and the upper edge of the deck line.
    Freeboard,
    /// Maximum vessel draught
    ///
    /// 6313
    ///
    /// The depth of water needed to float the ship (Maximum vessel draught according to Load Line Certificate IMO, MSC/Circ. 920 15 June 1999).
    MaximumVesselDraught,
    /// Net explosive weight
    ///
    /// 6313
    ///
    /// Mass of the explosive part or substance of goods without any packing.
    NetExplosiveWeight,
    /// Radioactive criticality safety index
    ///
    /// 6313
    ///
    /// A number, assigned to fissile material i.e. material capable of sustaining a nuclear chain reaction, which is used to provide control over the accumulation of packages containing such material.
    RadioactiveCriticalitySafetyIndex,
    /// Waste currently on board
    ///
    /// 6313
    ///
    /// Amount of waste on board at the moment of the notification.
    WasteCurrentlyOnBoard,
    /// Waste to be delivered at waste reception facility
    ///
    /// 6313
    ///
    /// Amount of waste to be delivered to a waste reception facility, e.g. in the port of call.
    WasteToBeDeliveredAtWasteReceptionFacility,
    /// Waste to be generated until next port of call, estimated
    ///
    /// 6313
    ///
    /// Estimated amount of waste to be generated between creation of the current notification and arrival in the next port of call, irrespective of use of incinerator or compactor or disposal at sea.
    WasteToBeGeneratedUntilNextPortCallEstimated,
    /// Waste remaining on board at departure
    ///
    /// 6313
    ///
    /// Amount of waste remaining on board when leaving the port of call.
    WasteRemainingOnBoardAtDeparture,
    /// Colour depth
    ///
    /// 6313
    ///
    /// The number of distinct colours represented, e.g. in an image or in a display.
    ColourDepth,
    /// Colour depth, maximum
    ///
    /// 6313
    ///
    /// The maximum number of distinct colours that can be represented, e.g. in an image or in a display.
    ColourDepthMaximum,
    /// Image resolution
    ///
    /// 6313
    ///
    /// The level of detail of an image.
    ImageResolution,
    /// Device resolution, maximum
    ///
    /// 6313
    ///
    /// The maximum level of detail produced by a device.
    DeviceResolutionMaximum,
    /// Acoustic absorption coefficient
    ///
    /// 6313
    ///
    /// The portion of sound energy a surface absorbs, measured at different frequencies.
    AcousticAbsorptionCoefficient,
    /// Billed weight
    ///
    /// 6313
    ///
    /// The measured billed weight.
    BilledWeight,
    /// Breaking load
    ///
    /// 6313
    ///
    /// The measured breaking load.
    BreakingLoad,
    /// Platinum
    ///
    /// 6313
    ///
    /// The measurement of the platinum component.
    Platinum,
    /// Silver
    ///
    /// 6313
    ///
    /// The measurement of the silver component.
    Silver,
    /// List
    ///
    /// 6313
    ///
    /// The leaning or inclination of a vessel expressed in degrees port or starboard.
    List,
    /// Trim
    ///
    /// 6313
    ///
    /// The condition of a vessel with reference to its longitudinal axis.
    Trim,
    /// Free water
    ///
    /// 6313
    ///
    /// The volume of water present in a container that is not in suspension in the contained liquid.
    FreeWater,
    /// Bands
    ///
    /// 6313
    ///
    /// The measured bands.
    Bands,
    /// API (American Petroleum Institute) gravity
    ///
    /// 6313
    ///
    /// The relative density of petroleum liquids as specified by a standard developed by the API.
    ApiAmericanPetroleumInstituteGravity,
    /// Petroleum gross observed volume
    ///
    /// 6313
    ///
    /// The total volume of all petroleum liquids and sediment and water, excluding free water, at observed temperature and pressure.
    PetroleumGrossObservedVolume,
    /// Petroleum gross standard volume
    ///
    /// 6313
    ///
    /// The total volume of all petroleum liquids, sediment, and water excluding free water, corrected by the appropriate volume correction factor for the observed temperature and American Petroleum Institute relative density, or density to a standard temperature.
    PetroleumGrossStandardVolume,
    /// Volume variance
    ///
    /// 6313
    ///
    /// The change in the volume measurement.
    VolumeVariance,
    /// Petroleum net standard volume
    ///
    /// 6313
    ///
    /// The total volume of all petroleum liquids, excluding sediment and water and free water, corrected by the appropriate volume correction factor for the observed temperature and American Petroleum Institute gravity relative to density or to a standard temperature.
    PetroleumNetStandardVolume,
    /// Material on-board quantity, after discharge
    ///
    /// 6313
    ///
    /// The material in vessel tanks, void spaces, and pipelines after discharge.
    MaterialOnBoardQuantityAfterDischarge,
    /// Petroleum total calculated volume
    ///
    /// 6313
    ///
    /// The total volume of all petroleum liquids, sediment and water corrected by the appropriate volume correction factor for the observed temperature and American Petroleum Institute (API) gravity, relative density, or density to a standard temperature.
    PetroleumTotalCalculatedVolume,
    /// Petroleum total observed volume
    ///
    /// 6313
    ///
    /// The total volume of all petroleum liquids, sediment and water and free water at observed temperature and pressure.
    PetroleumTotalObservedVolume,
    /// Innage gauge distance
    ///
    /// 6313
    ///
    /// The measured distance from the surface of the liquid to a fixed datum plate or to the tank bottom.
    InnageGaugeDistance,
    /// Petroleum net standard weight
    ///
    /// 6313
    ///
    /// The total weight of all petroleum liquids excluding sediments, water and free water.
    PetroleumNetStandardWeight,
    /// Sediment and water in petroleum
    ///
    /// 6313
    ///
    /// The measurement of non-hydrocarbon solid material and water in suspension in petroleum liquid.
    SedimentAndWaterInPetroleum,
    /// Observed reference height, tank
    ///
    /// 6313
    ///
    /// The observed distance from the tank bottom or datum plate to the established reference point.
    ObservedReferenceHeightTank,
    /// Reference height, tank
    ///
    /// 6313
    ///
    /// The measured distance from the tank bottom or datum plate to the established reference point.
    ReferenceHeightTank,
    /// Ullage gauge distance
    ///
    /// 6313
    ///
    /// The measured distance from the cargo liquid surface to the reference point.
    UllageGaugeDistance,
    /// Trim correction
    ///
    /// 6313
    ///
    /// The correction applied to the observed gauge or observed volume when a vessel is not on an even keel.
    TrimCorrection,
    /// Bow to bridge distance
    ///
    /// 6313
    ///
    /// The distance between the bow and the bridge of a vessel.
    BowToBridgeDistance,
    /// Peg hole number
    ///
    /// 6313
    ///
    /// Used to identify the peg hole if more than one hole is present in the product or packaging.
    PegHoleNumber,
    /// Number of inner packs
    ///
    /// 6313
    ///
    /// Indicates the number of non coded physical groupings (inner packs) of next lower level trade items within the current trade item level.
    NumberInnerPacks,
    /// Number of next level trade items within inner pack
    ///
    /// 6313
    ///
    /// The number of next lower level trade items contained within the physical non-coded grouping (inner pack).
    NumberNextLevelTradeItemsWithinInnerPack,
    /// Number of trade items per pallet layer
    ///
    /// 6313
    ///
    /// The number of trade items contained on a single layer of a pallet.
    NumberTradeItemsPerPalletLayer,
    /// Packed items layer height
    ///
    /// 6313
    ///
    /// The height of a single layer of packed items.
    PackedItemsLayerHeight,
    /// Packing material weight, skin tight covering
    ///
    /// 6313
    ///
    /// The weight measurement of the packing material used for skin tight covering (e.g. when packaging prepared meats, poultry, cheese, and other food products).
    PackingMaterialWeightSkinTightCovering,
    /// Brightness
    ///
    /// 6313
    ///
    /// The measured brightness.
    Brightness,
    /// Brakes
    ///
    /// 6313
    ///
    /// The measured brakes.
    Brakes,
    /// Components labelled for recycling percentage
    ///
    /// 6313
    ///
    /// Percentage of trade item components that clearly label how to facilitate product disassembly and recycling.
    ComponentsLabelledForRecyclingPercentage,
    /// Renewable plastic components percentage, by net weight
    ///
    /// 6313
    ///
    /// The percentage of the plastic components only made from rapidly renewable plant-based material by net weight of product.
    RenewablePlasticComponentsPercentageByNetWeight,
    /// Clamp pressure, required
    ///
    /// 6313
    ///
    /// The pressure that should be applied by a clamp.
    ClampPressureRequired,
    /// Break
    ///
    /// 6313
    ///
    /// The measured break.
    Break,
    /// Ascertained volume
    ///
    /// 6313
    ///
    /// Endorsement of the true volume as ascertained or verified.
    AscertainedVolume,
    /// Unit weight
    ///
    /// 6313
    ///
    /// The weight of a unit.
    UnitWeight,
    /// Total volume
    ///
    /// 6313
    ///
    /// Total volume of goods and/or parcels.
    TotalVolume,
    /// Unit volume
    ///
    /// 6313
    ///
    /// The volume of a unit.
    UnitVolume,
    /// Vertical center of gravity
    ///
    /// 6313
    ///
    /// Distance of vertical center of gravity relative to item's base.
    VerticalCenterGravity,
    /// Maximum allowable transport stacking weight
    ///
    /// 6313
    ///
    /// Maximum weight allowed to be stowed on top of an item during transport.
    MaximumAllowableTransportStackingWeight,
    /// Carbon Dioxide
    ///
    /// 6313
    ///
    /// Measurement of the carbon dioxide (C02) component.
    CarbonDioxide,
    /// Number of base units per pallet
    ///
    /// 6313
    ///
    /// The number of base units contained on a pallet.
    NumberBaseUnitsPerPallet,
    /// Colony forming unit
    ///
    /// 6313
    ///
    /// Micro-organism colonies that are to be counted under determined conditions.
    ColonyFormingUnit,
    /// Diluted liquid volume
    ///
    /// 6313
    ///
    /// The volume of liquid which results after a dilution agent has been added, e.g. undiluted orange juice of 200ml, after dilution with water the volume of diluted liquid equals 1 litre.
    DilutedLiquidVolume,
    /// Energy efficiency
    ///
    /// 6313
    ///
    /// A measurement of the energy efficiency of an article.
    EnergyEfficiency,
    /// Number of layers
    ///
    /// 6313
    ///
    /// Number of layers of a product or products within a package, container, etc.
    NumberLayers,
    /// Maximum demand
    ///
    /// 6313
    ///
    /// The highest demand recorded during the period of recording of usage of the supply.
    MaximumDemand,
    /// Number of pallet places
    ///
    /// 6313
    ///
    /// The number of pallet places needed to store or to transport pallets (can be stacked).
    NumberPalletPlaces,
    /// Package net measurement, cubed
    ///
    /// 6313
    ///
    /// An indication of the net cubed measurement of a package.
    PackageNetMeasurementCubed,
    /// Power factor
    ///
    /// 6313
    ///
    /// The ratio of the power dissipated (active power in kW) to the total power (which is the product of the input volts times amps given in kVa). When equipment which uses reactive power is being operated the power factor will be less than one.
    PowerFactor,
    /// Stacking factor excluding bottom item
    ///
    /// 6313
    ///
    /// Maximum number of identical items stackable on top of item.
    StackingFactorExcludingBottomItem,
    /// Breaking strength
    ///
    /// 6313
    ///
    /// The measured breaking strength.
    BreakingStrength,
    /// Breaking strength wet
    ///
    /// 6313
    ///
    /// The measured breaking strength when wet.
    BreakingStrengthWet,
    /// Step size
    ///
    /// 6313
    ///
    /// An indication of measurements in which options contained within a Customer Specific Article are available, e.g. 10 metre planks of wood may be sold in step sizes of 2 metres.
    StepSize,
    /// Number of units per package
    ///
    /// 6313
    ///
    /// The number of identified units per package.
    NumberUnitsPerPackage,
    /// Number of units per layer
    ///
    /// 6313
    ///
    /// Number of units of a product or package within one layer of a package, container, etc.
    NumberUnitsPerLayer,
    /// Weight per running metre
    ///
    /// 6313
    ///
    /// A code used to indicate the weight per running metre of floor coverings over floor area.
    WeightPerRunningMetre,
    /// Weight per square metre
    ///
    /// 6313
    ///
    /// A code used to indicate the weight per square metre of floor coverings over floor area.
    WeightPerSquareMetre,
    /// Acidity of meat
    ///
    /// 6313
    ///
    /// The meat's acid quality or condition.
    AcidityMeat,
    /// Slaughtering weight
    ///
    /// 6313
    ///
    /// Weight immediately after slaughter.
    SlaughteringWeight,
    /// Stacking factor including bottom item
    ///
    /// 6313
    ///
    /// Maximum number of items stackable upon each other, including the bottom item.
    StackingFactorIncludingBottomItem,
    /// Number of units in the width of a layer
    ///
    /// 6313
    ///
    /// Number of units of a product or package which make up the width of a layer in a package, container, pallet, etc.
    NumberUnitsInWidthALayer,
    /// Number of units in the depth of a layer
    ///
    /// 6313
    ///
    /// Number of units of a product or package which make up the depth of a layer in a package, container, pallet, etc.
    NumberUnitsInDepthALayer,
    /// Nestable percentage
    ///
    /// 6313
    ///
    /// Extent expressed as a percentage to which an item can be nested within an identical item, e.g. paper cups.
    NestablePercentage,
    /// Gross weight including carrier's equipment
    ///
    /// 6313
    ///
    /// Weight (mass) of goods including packaging and the carrier's equipment. In this context 'carrier's equipment' means any material resources necessary to facilitate the transport and handling of the goods without having the ability to move by its own propulsion, e.g. pallet, container, etc.
    GrossWeightIncludingCarriersEquipment,
    /// Sugar content
    ///
    /// 6313
    ///
    /// The rate of sugar.
    SugarContent,
    /// Self-accelerating polymerization temperature (SAPT)
    ///
    /// 6313
    ///
    /// The lowest temperature at which polymerization may occur for a substance as packed for transport.
    SelfAcceleratingPolymerizationTemperatureSapt,
    /// Self-accelerating decomposition temperature (SADT)
    ///
    /// 6313
    ///
    /// The lowest temperature at which self-accelerating decomposition may occur for a substance as packed for transport.
    SelfAcceleratingDecompositionTemperatureSadt,
    /// Control temperature
    ///
    /// 6313
    ///
    /// The controlled transport temperature to avoid decomposition of self-reactive substances and organic peroxides.
    ControlTemperature,
    /// Basis weight
    ///
    /// 6313
    ///
    /// The measured basis weight.
    BasisWeight,
    /// Change
    ///
    /// 6313
    ///
    /// The measured change.
    Change,
    /// Emergency temperature
    ///
    /// 6313
    ///
    /// The temperature at which emergency response is required for self-reactive substances and organic peroxides.
    EmergencyTemperature,
    /// Colour
    ///
    /// 6313
    ///
    /// The measured colour.
    Colour,
    /// Contents of package
    ///
    /// 6313
    ///
    /// In combination with the other data elements of the actual segment this code indicates the measured content of a package.
    ContentsPackage,
    /// Commercial weight
    ///
    /// 6313
    ///
    /// Item weight considering its maximum possible humidity.
    CommercialWeight,
    /// Core length
    ///
    /// 6313
    ///
    /// To specify length of core on which product is to be placed.
    CoreLength,
    /// Destination weight agreement
    ///
    /// 6313
    ///
    /// The agreed weight of despatched goods whose weight may change during transport.
    DestinationWeightAgreement,
    /// Diameter
    ///
    /// 6313
    ///
    /// Diameter of an article.
    Diameter,
    /// Delta value L
    ///
    /// 6313
    ///
    /// The measured delta value L.
    DeltaValueL,
    /// Density
    ///
    /// 6313
    ///
    /// The measured density.
    Density,
    /// Depth
    ///
    /// 6313
    ///
    /// The measured depth.
    Depth,
    /// Denier
    ///
    /// 6313
    ///
    /// The measured fineness of a material.
    Denier,
    /// Distance between points
    ///
    /// 6313
    ///
    /// The measured distance between points.
    DistanceBetweenPoints,
    /// Width, boxcar door
    ///
    /// 6313
    ///
    /// The measured width of a boxcar door.
    WidthBoxcarDoor,
    /// Estimated new weight
    ///
    /// 6313
    ///
    /// The measured estimated new weight.
    EstimatedNewWeight,
    /// Elongation
    ///
    /// 6313
    ///
    /// The measured elongation.
    Elongation,
    /// Deficit weight
    ///
    /// 6313
    ///
    /// The measured deficit weight.
    DeficitWeight,
    /// Filament count
    ///
    /// 6313
    ///
    /// Used e.g. in textile, print industries.
    FilamentCount,
    /// Longitudinal flatness
    ///
    /// 6313
    ///
    /// The measured longitudinal flatness.
    LongitudinalFlatness,
    /// Flatness
    ///
    /// 6313
    ///
    /// The measured flatness.
    Flatness,
    /// Transverse flatness
    ///
    /// 6313
    ///
    /// The measured transverse flatness.
    TransverseFlatness,
    /// Gauge
    ///
    /// 6313
    ///
    /// The measured gauge.
    Gauge,
    /// Gross weight, maximum
    ///
    /// 6313
    ///
    /// The measured maximum gross weight.
    GrossWeightMaximum,
    /// Hardness
    ///
    /// 6313
    ///
    /// The measured hardness.
    Hardness,
    /// Height, maximum
    ///
    /// 6313
    ///
    /// The measured maximum height.
    HeightMaximum,
    /// Height dimension
    ///
    /// 6313
    ///
    /// Numeric value of height.
    HeightDimension,
    /// Impact energy
    ///
    /// 6313
    ///
    /// The measured impact energy.
    ImpactEnergy,
    /// Inside diameter
    ///
    /// 6313
    ///
    /// The measured inside diameter.
    InsideDiameter,
    /// Legal weight
    ///
    /// 6313
    ///
    /// The measured legal weight.
    LegalWeight,
    /// Length, maximum
    ///
    /// 6313
    ///
    /// The measured maximum length.
    LengthMaximum,
    /// Length
    ///
    /// 6313
    ///
    /// To specify the value of a length dimension.
    Length,
    /// Lost end
    ///
    /// 6313
    ///
    /// The measured lost end.
    LostEnd,
    /// Minimum weight
    ///
    /// 6313
    ///
    /// The measured minimum weight.
    MinimumWeight,
    /// Moisture
    ///
    /// 6313
    ///
    /// Measurement application is the moisture content of the item.
    Moisture,
    /// Maximum weight
    ///
    /// 6313
    ///
    /// The measured maximum weight.
    MaximumWeight,
    /// Actual net weight
    ///
    /// 6313
    ///
    /// The actual weight of the goods excluding packaging.
    ActualNetWeight,
    /// Outside diameter
    ///
    /// 6313
    ///
    /// The measured outside diameter.
    OutsideDiameter,
    /// Pre stretch
    ///
    /// 6313
    ///
    /// Measurement identifying the amount an item has been stretched prior to use.
    PreStretch,
    /// Per tonne
    ///
    /// 6313
    ///
    /// A measurement per tonne.
    PerTonne,
    /// Relative humidity
    ///
    /// 6313
    ///
    /// The measured relative humidity.
    RelativeHumidity,
    /// Resistivity
    ///
    /// 6313
    ///
    /// The measured resistivity.
    Resistivity,
    /// Rockwell C
    ///
    /// 6313
    ///
    /// Hardness in the Rockwell C scale.
    RockwellC,
    /// Ream weight
    ///
    /// 6313
    ///
    /// Measurement indication for paper.
    ReamWeight,
    /// Reduction of area
    ///
    /// 6313
    ///
    /// The measured reduction of area.
    ReductionArea,
    /// Run (process)
    ///
    /// 6313
    ///
    /// The measured run (process).
    RunProcess,
    /// Ratio
    ///
    /// 6313
    ///
    /// The measured ratio.
    Ratio,
    /// Shipped quantity
    ///
    /// 6313
    ///
    /// The measured shipped quantity.
    ShippedQuantity,
    /// Tare weight
    ///
    /// 6313
    ///
    /// Weight excluding goods and loose accessories.
    TareWeight,
    /// Temperature
    ///
    /// 6313
    ///
    /// A measurement in relation to temperature.
    Temperature,
    /// Thickness
    ///
    /// 6313
    ///
    /// The measured thickness.
    Thickness,
    /// Time period
    ///
    /// 6313
    ///
    /// Measurement of a specific length of time.
    TimePeriod,
    /// Time
    ///
    /// 6313
    ///
    /// The measured time.
    Time,
    /// Transport equipment verified gross mass (weight)
    ///
    /// 6313
    ///
    /// The gross mass (weight) of the transport equipment verified according to SOLAS Chapter VI, Regulation 2, paragraphs 4-6.
    TransportEquipmentVerifiedGrossMassWeight,
    /// Height, van door
    ///
    /// 6313
    ///
    /// The height of the door of a van or container.
    HeightVanDoor,
    /// Width, van door
    ///
    /// 6313
    ///
    /// The width of the door of a van or container.
    WidthVanDoor,
    /// Weight per unit of area
    ///
    /// 6313
    ///
    /// The weight per unit of an area.
    WeightPerUnitArea,
    /// Width dimension
    ///
    /// 6313
    ///
    /// Numeric value of width.
    WidthDimension,
    /// Width, maximum
    ///
    /// 6313
    ///
    /// The maximum distance from side to side.
    WidthMaximum,
    /// Weight per unit of length
    ///
    /// 6313
    ///
    /// The weight per unit of length.
    WeightPerUnitLength,
    /// Side height, flat bed with removable sides
    ///
    /// 6313
    ///
    /// The height of the removable sides of a flat bed truck.
    SideHeightFlatBedWithRemovableSides,
    /// Squareness
    ///
    /// 6313
    ///
    /// The measured squareness.
    Squareness,
    /// Spool size
    ///
    /// 6313
    ///
    /// The measured spool size.
    SpoolSize,
    /// Yield stress
    ///
    /// 6313
    ///
    /// The measured yield stress.
    YieldStress,
    /// Aluminium
    ///
    /// 6313
    ///
    /// The measured chemical element aluminium.
    Aluminium,
    /// Arsenic
    ///
    /// 6313
    ///
    /// The measured chemical element arsenic.
    Arsenic,
    /// Boron
    ///
    /// 6313
    ///
    /// The measured chemical element boron.
    Boron,
    /// Bismuth
    ///
    /// 6313
    ///
    /// The measured chemical element bismuth.
    Bismuth,
    /// Carbon
    ///
    /// 6313
    ///
    /// The measured chemical element carbon.
    Carbon,
    /// Calcium
    ///
    /// 6313
    ///
    /// The measured chemical element calcium.
    Calcium,
    /// Columbium
    ///
    /// 6313
    ///
    /// The measured chemical element columbium.
    Columbium,
    /// Cerium
    ///
    /// 6313
    ///
    /// The measured chemical element cerium.
    Cerium,
    /// Chlorine
    ///
    /// 6313
    ///
    /// The measured chemical element chlorine.
    Chlorine,
    /// Cobalt
    ///
    /// 6313
    ///
    /// The measured chemical element cobalt.
    Cobalt,
    /// Chromium
    ///
    /// 6313
    ///
    /// The measured chemical element chromium.
    Chromium,
    /// Copper
    ///
    /// 6313
    ///
    /// The measured chemical element copper.
    Copper,
    /// Iron
    ///
    /// 6313
    ///
    /// The measured chemical element iron.
    Iron,
    /// Iron plus silicon
    ///
    /// 6313
    ///
    /// The measured substance iron plus silicon.
    IronPlusSilicon,
    /// Germanium
    ///
    /// 6313
    ///
    /// The measured chemical element germanium.
    Germanium,
    /// Hydrogen
    ///
    /// 6313
    ///
    /// The measured chemical element hydrogen.
    Hydrogen,
    /// Potassium
    ///
    /// 6313
    ///
    /// The measured chemical element potassium.
    Potassium,
    /// Magnesium
    ///
    /// 6313
    ///
    /// The measured chemical element magnesium.
    Magnesium,
    /// Manganese
    ///
    /// 6313
    ///
    /// The measured chemical element manganese.
    Manganese,
    /// Molybdenum
    ///
    /// 6313
    ///
    /// The measured chemical element molybdenum.
    Molybdenum,
    /// Nitrogen
    ///
    /// 6313
    ///
    /// The measured chemical element nitrogen.
    Nitrogen,
    /// Sodium
    ///
    /// 6313
    ///
    /// The measured chemical element sodium.
    Sodium,
    /// Niobium
    ///
    /// 6313
    ///
    /// The chemical element niobium.
    Niobium,
    /// Nickel
    ///
    /// 6313
    ///
    /// The measured chemical element nickel.
    Nickel,
    /// Oxygen
    ///
    /// 6313
    ///
    /// The measured chemical element oxygen.
    Oxygen,
    /// Phosphorus
    ///
    /// 6313
    ///
    /// The measured chemical element phosphorus.
    Phosphorus,
    /// Lead
    ///
    /// 6313
    ///
    /// The measured chemical element lead.
    Lead,
    /// Sulphur
    ///
    /// 6313
    ///
    /// The measured chemical element sulphur.
    Sulphur,
    /// Antimony
    ///
    /// 6313
    ///
    /// The measured chemical element antimony.
    Antimony,
    /// Selenium
    ///
    /// 6313
    ///
    /// The measured chemical element selenium.
    Selenium,
    /// Silicon
    ///
    /// 6313
    ///
    /// The measured chemical element silicon.
    Silicon,
    /// Silicium oxyd
    ///
    /// 6313
    ///
    /// The measured substance silicium oxyd.
    SiliciumOxyd,
    /// Tin
    ///
    /// 6313
    ///
    /// The measured chemical element tin.
    Tin,
    /// Tantalium
    ///
    /// 6313
    ///
    /// The measured chemical element tantalium.
    Tantalium,
    /// Tellurium
    ///
    /// 6313
    ///
    /// The measured chemical element tellurium.
    Tellurium,
    /// Titanium
    ///
    /// 6313
    ///
    /// The measured chemical element titanium.
    Titanium,
    /// Vanadium
    ///
    /// 6313
    ///
    /// The measured chemical element vanadium.
    Vanadium,
    /// Tungsten
    ///
    /// 6313
    ///
    /// The measured chemical element tungsten.
    Tungsten,
    /// Waste content
    ///
    /// 6313
    ///
    /// The measured waste content.
    WasteContent,
    /// Zinc
    ///
    /// 6313
    ///
    /// The measured chemical element zinc.
    Zinc,
    /// Zirconium
    ///
    /// 6313
    ///
    /// The measured chemical element zirconium.
    Zirconium,
    /// Mutually defined
    ///
    /// 6313
    ///
    /// A code assigned within a code list to be used on an interim basis and as defined among trading partners until a precise code can be assigned to the code list.
    MutuallyDefined,
    /// Best before date
    ///
    /// Factur-X
    BestBeforeDate,
    /// Colour as text
    ///
    /// Factur-X
    ColourAsText,
    /// Commission indicator
    ///
    /// Factur-X
    ///
    /// true / false
    CommissionIndicator,
    /// Deposit system
    ///
    /// Factur-X
    ///
    /// Permitted values: DISPOSABLE RETURNABLE
    DepositSystem,
    /// Deposit type
    ///
    /// Factur-X
    ///
    /// Permitted values: PRODUCT_PACKAGING TRANSPORT_EQUIPMENT
    DepositType,
    /// Energy efficiency class
    ///
    /// Factur-X
    EnergyEfficiencyClass,
    /// Expiration date
    ///
    /// Factur-X
    ExpirationDate,
    /// Fee indicator
    ///
    /// Factur-X
    ///
    /// true = item is a fee
    FeeIndicator,
    /// Type of article
    ///
    /// Factur-X
    ///
    /// Allowed values: GOODS OTHER_SERVICES
    TypeArticle,
    /// Material of the product
    ///
    /// Factur-X
    MaterialProduct,
    /// Metering point designation, e.g. for electricity or gas
    ///
    /// Factur-X
    MeteringPointDesignationEGForElectricityOrGas,
    /// Meter number, e.g. for electricity or gas
    ///
    /// Factur-X
    MeterNumberEGForElectricityOrGas,
    /// Organic control body number
    ///
    /// Factur-X
    OrganicControlBodyNumber,
    /// Packaging material
    ///
    /// Factur-X
    PackagingMaterial,
    /// Type of packaging (code)
    ///
    /// Factur-X
    ///
    /// Type of packaging in coded form. The packaging units from Rec 20 / Rec 21 are used as the code list. Example XBO=bottle
    TypePackagingCode,
    /// Number of the action variant
    ///
    /// Factur-X
    NumberActionVariant,
    /// Seal number
    ///
    /// Factur-X
    SealNumber,
    /// Size code
    ///
    /// Factur-X
    ///
    /// Size information in coded form
    SizeCode,
    /// Size designation
    ///
    /// Factur-X
    ///
    /// Size specifications in text form
    SizeDesignation,
    /// Type of trading unit
    ///
    /// Factur-X
    ///
    /// Permitted values: MIXED_ASSORTMENT = Assortment UNMIXED_UNIT = Container
    TypeTradingUnit,
    /// Waste code (EWC)
    ///
    /// Factur-X
    ///
    /// Waste code according to the European Waste Catalogue (EWC)
    WasteCodeEwc,
    /// Waste fraction
    ///
    /// Factur-X
    WasteFraction,
    /// WEEE registration number
    ///
    /// Factur-X
    ///
    /// WEEE registration number of the manufacturer of the product
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
            Characteristic::HydrateContentAnAlcoholicProductAtBottling => "Hydrate content of an alcoholic product at bottling",
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
            Characteristic::PegHoleHorizontalDistanceFromPackageLeftmostEdge => "Peg hole horizontal distance from package leftmost edge",
            Characteristic::PegHoleVerticalDistanceFromPackageTop => "Peg hole vertical distance from package top",
            Characteristic::NumberLayersPerPallet => "Number of layers per pallet",
            Characteristic::ProductStrenghChemical => "Product strengh, chemical",
            Characteristic::ProductStrengthBasisChemical => "Product strength basis, chemical",
            Characteristic::ItemWeight => "Item weight",
            Characteristic::PayloadWeightMaximum => "Payload weight, maximum",
            Characteristic::WeightConveyance => "Weight of conveyance",
            Characteristic::ConveyanceSummerDeadWeight => "Conveyance summer dead weight",
            Characteristic::ContainerizedCargoOnVesselsWeight => "Containerized cargo on vessel's weight",
            Characteristic::NonContainerizedCargoOnVesselsWeight => "Non-containerized cargo on vessel's weight",
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
            Characteristic::TransportContainerActualFillingWeight => "Transport container actual filling weight",
            Characteristic::TransportContainerMaximumCapacity => "Transport container maximum capacity",
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
            Characteristic::TotalTransportEquipmentGrossWeight => "Total transport equipment gross weight",
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
            Characteristic::PercentageFatContentInDryMatter => "Percentage fat content in dry matter",
            Characteristic::SaccharometricContent => "Saccharometric content",
            Characteristic::HydrateContentAnAlcoholicProductAfterBottling => "Hydrate content of an alcoholic product after bottling",
            Characteristic::AnhydrousContent => "Anhydrous content",
            Characteristic::CertifiedWeight => "Certified weight",
            Characteristic::Freeboard => "Freeboard",
            Characteristic::MaximumVesselDraught => "Maximum vessel draught",
            Characteristic::NetExplosiveWeight => "Net explosive weight",
            Characteristic::RadioactiveCriticalitySafetyIndex => "Radioactive criticality safety index",
            Characteristic::WasteCurrentlyOnBoard => "Waste currently on board",
            Characteristic::WasteToBeDeliveredAtWasteReceptionFacility => "Waste to be delivered at waste reception facility",
            Characteristic::WasteToBeGeneratedUntilNextPortCallEstimated => "Waste to be generated until next port of call, estimated",
            Characteristic::WasteRemainingOnBoardAtDeparture => "Waste remaining on board at departure",
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
            Characteristic::ApiAmericanPetroleumInstituteGravity => "API (American Petroleum Institute) gravity",
            Characteristic::PetroleumGrossObservedVolume => "Petroleum gross observed volume",
            Characteristic::PetroleumGrossStandardVolume => "Petroleum gross standard volume",
            Characteristic::VolumeVariance => "Volume variance",
            Characteristic::PetroleumNetStandardVolume => "Petroleum net standard volume",
            Characteristic::MaterialOnBoardQuantityAfterDischarge => "Material on-board quantity, after discharge",
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
            Characteristic::NumberNextLevelTradeItemsWithinInnerPack => "Number of next level trade items within inner pack",
            Characteristic::NumberTradeItemsPerPalletLayer => "Number of trade items per pallet layer",
            Characteristic::PackedItemsLayerHeight => "Packed items layer height",
            Characteristic::PackingMaterialWeightSkinTightCovering => "Packing material weight, skin tight covering",
            Characteristic::Brightness => "Brightness",
            Characteristic::Brakes => "Brakes",
            Characteristic::ComponentsLabelledForRecyclingPercentage => "Components labelled for recycling percentage",
            Characteristic::RenewablePlasticComponentsPercentageByNetWeight => "Renewable plastic components percentage, by net weight",
            Characteristic::ClampPressureRequired => "Clamp pressure, required",
            Characteristic::Break => "Break",
            Characteristic::AscertainedVolume => "Ascertained volume",
            Characteristic::UnitWeight => "Unit weight",
            Characteristic::TotalVolume => "Total volume",
            Characteristic::UnitVolume => "Unit volume",
            Characteristic::VerticalCenterGravity => "Vertical center of gravity",
            Characteristic::MaximumAllowableTransportStackingWeight => "Maximum allowable transport stacking weight",
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
            Characteristic::StackingFactorExcludingBottomItem => "Stacking factor excluding bottom item",
            Characteristic::BreakingStrength => "Breaking strength",
            Characteristic::BreakingStrengthWet => "Breaking strength wet",
            Characteristic::StepSize => "Step size",
            Characteristic::NumberUnitsPerPackage => "Number of units per package",
            Characteristic::NumberUnitsPerLayer => "Number of units per layer",
            Characteristic::WeightPerRunningMetre => "Weight per running metre",
            Characteristic::WeightPerSquareMetre => "Weight per square metre",
            Characteristic::AcidityMeat => "Acidity of meat",
            Characteristic::SlaughteringWeight => "Slaughtering weight",
            Characteristic::StackingFactorIncludingBottomItem => "Stacking factor including bottom item",
            Characteristic::NumberUnitsInWidthALayer => "Number of units in the width of a layer",
            Characteristic::NumberUnitsInDepthALayer => "Number of units in the depth of a layer",
            Characteristic::NestablePercentage => "Nestable percentage",
            Characteristic::GrossWeightIncludingCarriersEquipment => "Gross weight including carrier's equipment",
            Characteristic::SugarContent => "Sugar content",
            Characteristic::SelfAcceleratingPolymerizationTemperatureSapt => "Self-accelerating polymerization temperature (SAPT)",
            Characteristic::SelfAcceleratingDecompositionTemperatureSadt => "Self-accelerating decomposition temperature (SADT)",
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
            Characteristic::TransportEquipmentVerifiedGrossMassWeight => "Transport equipment verified gross mass (weight)",
            Characteristic::HeightVanDoor => "Height, van door",
            Characteristic::WidthVanDoor => "Width, van door",
            Characteristic::WeightPerUnitArea => "Weight per unit of area",
            Characteristic::WidthDimension => "Width dimension",
            Characteristic::WidthMaximum => "Width, maximum",
            Characteristic::WeightPerUnitLength => "Weight per unit of length",
            Characteristic::SideHeightFlatBedWithRemovableSides => "Side height, flat bed with removable sides",
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
            Characteristic::MeteringPointDesignationEGForElectricityOrGas => "Metering point designation, e.g. for electricity or gas",
            Characteristic::MeterNumberEGForElectricityOrGas => "Meter number, e.g. for electricity or gas",
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
        Self: Sized
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
