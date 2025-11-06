export enum Characteristic {
  /**
   * Consolidated weight
   *
   * 6313
   *
   * The measured consolidated weight.
   */
  ConsolidatedWeight = "A",
  /**
   * Net weight
   *
   * 6313
   *
   * Weight of goods including any packaging that normally going with the goods.
   */
  NetWeight = "AAA",
  /**
   * Goods item gross weight
   *
   * 6313
   *
   * Weight (mass) of goods including packing but excluding the carrier's equipment.
   */
  GoodsItemGrossWeight = "AAB",
  /**
   * Total net weight
   *
   * 6313
   *
   * Total weight of goods excluding packaging.
   */
  TotalNetWeight = "AAC",
  /**
   * Consignment gross weight
   *
   * 6313
   *
   * Total gross weight (mass) of all goods items referred to as one consignment including packaging but excluding any transport equipment.
   */
  ConsignmentGrossWeight = "AAD",
  /**
   * Net net weight
   *
   * 6313
   *
   * Weight (mass) if goods without any packaging.
   */
  NetNetWeight = "AAF",
  /**
   * Stern thrust
   *
   * 6313
   *
   * Force exerted by a thruster installed at the stern of a vessel.
   */
  SternThrust = "AAG",
  /**
   * Bow thrust
   *
   * 6313
   *
   * Force exerted by a thruster installed at the bow of a vessel.
   */
  BowThrust = "AAH",
  /**
   * Hydrate content of an alcoholic product at bottling
   *
   * 6313
   *
   * The hydrate content of an alcoholic product at the moment of bottling.
   */
  HydrateContentAnAlcoholicProductAtBottling = "AAI",
  /**
   * Number of units per pallet
   *
   * 6313
   *
   * The number of units contained on a pallet.
   */
  NumberUnitsPerPallet = "AAJ",
  /**
   * Fat content
   *
   * 6313
   *
   * An indication of the fat content of a product.
   */
  FatContent = "AAK",
  /**
   * Transport means gross weight
   *
   * 6313
   *
   * The measure of the overall size of a ship determined in accordance with the provisions of the International Convention on Tonnage Measurement of Ships.
   */
  TransportMeansGrossWeight = "AAM",
  /**
   * Net tonnage of the vessel
   *
   * 6313
   *
   * The measure of the useful capacity of a ship determined in accordance with the provisions of the International Convention on Tonnage Measurement of Ships.
   */
  NetTonnageVessel = "AAN",
  /**
   * Humidity
   *
   * 6313
   *
   * Degree of moisture.
   */
  Humidity = "AAO",
  /**
   * Voltage
   *
   * 6313
   *
   * Electromotive force, or difference of electronic potential between two points.
   */
  Voltage = "AAP",
  /**
   * Power consumption
   *
   * 6313
   *
   * Value of energy consumption.
   */
  PowerConsumption = "AAQ",
  /**
   * Heat dissipation
   *
   * 6313
   *
   * Measurement of the rate of dispersal of heat.
   */
  HeatDissipation = "AAR",
  /**
   * Air flow
   *
   * 6313
   *
   * Measurement of the flow of air.
   */
  AirFlow = "AAS",
  /**
   * Shock impact
   *
   * 6313
   *
   * Measurement of the impact of a shock.
   */
  ShockImpact = "AAT",
  /**
   * Operative temperature
   *
   * 6313
   *
   * Temperature identified system or process works according to specifications.
   */
  OperativeTemperature = "AAU",
  /**
   * Non operative temperature
   *
   * 6313
   *
   * Temperature identified system or process does not work according to specifications.
   */
  NonOperativeTemperature = "AAV",
  /**
   * Gross volume
   *
   * 6313
   *
   * The volume unadjusted for factors such as temperature or gravity.
   */
  GrossVolume = "AAW",
  /**
   * Net volume
   *
   * 6313
   *
   * The volume after adjustment for factors such as temperature or gravity.
   */
  NetVolume = "AAX",
  /**
   * Water content
   *
   * 6313
   *
   * Water content in product.
   */
  WaterContent = "AAY",
  /**
   * Tensile stress
   *
   * 6313
   *
   * The measured tensile stress.
   */
  TensileStress = "AAZ",
  /**
   * Fibrosity
   *
   * 6313
   *
   * The measured fibrosity.
   */
  Fibrosity = "ABA",
  /**
   * Gauge length
   *
   * 6313
   *
   * The measured gauge length.
   */
  GaugeLength = "ABB",
  /**
   * Radius
   *
   * 6313
   *
   * The measured radius.
   */
  Radius = "ABC",
  /**
   * Straightness
   *
   * 6313
   *
   * Straightness of the item.
   */
  Straightness = "ABD",
  /**
   * Strain
   *
   * 6313
   *
   * The measured strain.
   */
  Strain = "ABE",
  /**
   * Item width when unrolled
   *
   * 6313
   *
   * The width of an item when unrolled.
   */
  ItemWidthWhenUnrolled = "ABF",
  /**
   * Item length when unrolled
   *
   * 6313
   *
   * The length of an item when unrolled.
   */
  ItemLengthWhenUnrolled = "ABG",
  /**
   * Item area when unrolled
   *
   * 6313
   *
   * The area occupied by an item when unrolled.
   */
  ItemAreaWhenUnrolled = "ABH",
  /**
   * Original wort
   *
   * 6313
   *
   * Measure of the malt and hops content of beer, before fermentation has taken place.
   */
  OriginalWort = "ABI",
  /**
   * Volume
   *
   * 6313
   *
   * The amount of air space taken up by the entity identified in the 6311 qualifier.
   */
  Volume = "ABJ",
  /**
   * Angle
   *
   * 6313
   *
   * The angle of an object.
   */
  Angle = "ABK",
  /**
   * Peg hole horizontal distance from package leftmost edge
   *
   * 6313
   *
   * Horizontal distance from the left most edge of the package to the center of the hole into which the peg is inserted.
   */
  PegHoleHorizontalDistanceFromPackageLeftmostEdge = "ABL",
  /**
   * Peg hole vertical distance from package top
   *
   * 6313
   *
   * Vertical distance from the top of the package to the top of the hole into which the peg is inserted.
   */
  PegHoleVerticalDistanceFromPackageTop = "ABM",
  /**
   * Number of layers per pallet
   *
   * 6313
   *
   * The number of layers per pallet.
   */
  NumberLayersPerPallet = "ABN",
  /**
   * Product strengh, chemical
   *
   * 6313
   *
   * The amount of the single active chemical ingredient within a product.
   */
  ProductStrenghChemical = "ABO",
  /**
   * Product strength basis, chemical
   *
   * 6313
   *
   * Amount of product used as the basis for the specification of the chemical product strenght.
   */
  ProductStrengthBasisChemical = "ABP",
  /**
   * Item weight
   *
   * 6313
   *
   * Weight at line item level.
   */
  ItemWeight = "ABS",
  /**
   * Payload weight, maximum
   *
   * 6313
   *
   * The maximum weight of a payload.
   */
  PayloadWeightMaximum = "ABT",
  /**
   * Weight of conveyance
   *
   * 6313
   *
   * Tonnage of conveyance.
   */
  WeightConveyance = "ABX",
  /**
   * Conveyance summer dead weight
   *
   * 6313
   *
   * Registered summer dead weight total tonnage of the vessel.
   */
  ConveyanceSummerDeadWeight = "ABY",
  /**
   * Containerized cargo on vessel's weight
   *
   * 6313
   *
   * Total weight of containerized cargo on vessel.
   */
  ContainerizedCargoOnVesselsWeight = "ABZ",
  /**
   * Non-containerized cargo on vessel's weight
   *
   * 6313
   *
   * Total weight of non-containerized cargo on vessel.
   */
  NonContainerizedCargoOnVesselsWeight = "ACA",
  /**
   * Ascertained weight
   *
   * 6313
   *
   * Endorsement of the true weight (mass) as ascertained or verified.
   */
  AscertainedWeight = "ACE",
  /**
   * Chargeable weight
   *
   * 6313
   *
   * The weight on which charges are based.
   */
  ChargeableWeight = "ACG",
  /**
   * Estimated gross weight
   *
   * 6313
   *
   * Estimated weight (mass) of goods, including packing and excluding carrier's.
   */
  EstimatedGrossWeight = "ACN",
  /**
   * Estimated volume
   *
   * 6313
   *
   * Estimated size or measure of anything in three dimensions.
   */
  EstimatedVolume = "ACP",
  /**
   * Vessel overall length
   *
   * 6313
   *
   * Total overall length of the vessel.
   */
  VesselOverallLength = "ACS",
  /**
   * Loading meters
   *
   * 6313
   *
   * The length in a vehicle, whereby the complete width and height over that length is needed for the goods.
   */
  LoadingMeters = "ACV",
  /**
   * Number of axles
   *
   * 6313
   *
   * Number of axles of movable equipment or means of transport on wheels.
   */
  NumberAxles = "ACW",
  /**
   * Payload
   *
   * 6313
   *
   * The revenue-producing load carried by a means of transport.
   */
  Payload = "ACX",
  /**
   * Start position in the length
   *
   * 6313
   *
   * The starting position from the beginning of an item located in the length direction.
   */
  StartPositionInLength = "ADR",
  /**
   * End position in the length
   *
   * 6313
   *
   * The end position from the beginning of an item located in the length direction.
   */
  EndPositionInLength = "ADS",
  /**
   * Start position in the width
   *
   * 6313
   *
   * The start position from the beginning of an item located in the width direction.
   */
  StartPositionInWidth = "ADT",
  /**
   * End position in the width
   *
   * 6313
   *
   * The end position from the beginning of an item located in the width direction.
   */
  EndPositionInWidth = "ADU",
  /**
   * Start position in the thickness
   *
   * 6313
   *
   * The start position from the beginning of an item located in the thickness direction.
   */
  StartPositionInThickness = "ADV",
  /**
   * End position in the thickness
   *
   * 6313
   *
   * The end position from the beginning of an item located in the thickness direction.
   */
  EndPositionInThickness = "ADW",
  /**
   * Transport container actual filling weight
   *
   * 6313
   *
   * Actual filling weight of a transport container.
   */
  TransportContainerActualFillingWeight = "ADX",
  /**
   * Transport container maximum capacity
   *
   * 6313
   *
   * Maximum capacity of a transport container.
   */
  TransportContainerMaximumCapacity = "ADY",
  /**
   * Declared net weight
   *
   * 6313
   *
   * The declared net weight of a product or products used for invoicing, customs or transport purposes.
   */
  DeclaredNetWeight = "ADZ",
  /**
   * Loading height
   *
   * 6313
   *
   * Maximum height of products or packages loaded onto a given transportation device or equipment such as a pallet.
   */
  LoadingHeight = "AEA",
  /**
   * Stacking height
   *
   * 6313
   *
   * Maximum height up to which the same product or package may be placed one upon the other for storage purposes.
   */
  StackingHeight = "AEB",
  /**
   * Calculated weight
   *
   * 6313
   *
   * The calculated weight of the item based on the ordered dimensions.
   */
  CalculatedWeight = "AEC",
  /**
   * Ferrite
   *
   * 6313
   *
   * The chemical composition ferrite.
   */
  Ferrite = "AED",
  /**
   * Impurity
   *
   * 6313
   *
   * The impurity of the product i.e. the measurement of other chemical elements not normally appearing in a product.
   */
  Impurity = "AEE",
  /**
   * Grain size
   *
   * 6313
   *
   * The grain size.
   */
  GrainSize = "AEF",
  /**
   * Lanthanides
   *
   * 6313
   *
   * The chemical element Lanthanides.
   */
  Lanthanides = "AEG",
  /**
   * Elasticity
   *
   * 6313
   *
   * The value of the elasticity.
   */
  Elasticity = "AEH",
  /**
   * Drained weight
   *
   * 6313
   *
   * The weight of a product when all liquids used in the packaging of the product have been removed.
   */
  DrainedWeight = "AEI",
  /**
   * Gallium
   *
   * 6313
   *
   * Measurement of the gallium component.
   */
  Gallium = "AEJ",
  /**
   * Strontium
   *
   * 6313
   *
   * Measurement of the strontium component.
   */
  Strontium = "AEK",
  /**
   * Area
   *
   * 6313
   *
   * Extent or measure of a surface.
   */
  Area = "AEL",
  /**
   * Equipment storage limitation
   *
   * 6313
   *
   * Maximum storage limit of the equipment.
   */
  EquipmentStorageLimitation = "AEM",
  /**
   * Radioactive index of transport
   *
   * 6313
   *
   * The index of transport determines the maximum radiation level at a distance of 1m from the external surface.
   */
  RadioactiveIndexTransport = "AEN",
  /**
   * Radioactivity
   *
   * 6313
   *
   * Activity of radioactive material.
   */
  Radioactivity = "AEO",
  /**
   * Average gross weight
   *
   * 6313
   *
   * Weight which is the outcome of the division of the total gross weight by the number of units.
   */
  AverageGrossWeight = "AEP",
  /**
   * Forward draft
   *
   * 6313
   *
   * Depth of water from the surface of water to the bottom of the vessel measured at the draft mark of the bow.
   */
  ForwardDraft = "AEQ",
  /**
   * After draft
   *
   * 6313
   *
   * Depth of water from the surface of water to the bottom of the vessel measured at the draft mark of the stern.
   */
  AfterDraft = "AER",
  /**
   * Acidity
   *
   * 6313
   *
   * The quality, state, or degree of being acid.
   */
  Acidity = "AES",
  /**
   * Transport equipment gross weight
   *
   * 6313
   *
   * Weight of a transport equipment including the cargo and carrier's equipment.
   */
  TransportEquipmentGrossWeight = "AET",
  /**
   * Total transport equipment gross weight
   *
   * 6313
   *
   * Total weight of all transport equipment including the cargo and carrier's equipment.
   */
  TotalTransportEquipmentGrossWeight = "AEU",
  /**
   * Acidity of juice
   *
   * 6313
   *
   * Acid measurement of juice.
   */
  AcidityJuice = "AEV",
  /**
   * Penetrometry
   *
   * 6313
   *
   * Measurement of force required to drive a standard penetrating stamp.
   */
  Penetrometry = "AEW",
  /**
   * Durofel
   *
   * 6313
   *
   * Measurement of the elastic force using a standard penetrating stamp.
   */
  Durofel = "AEX",
  /**
   * Juice weight per 100 grams
   *
   * 6313
   *
   * Measurement of weight of juice, based on 100 grams of the entire weight.
   */
  JuiceWeightPer100Grams = "AEY",
  /**
   * Fruit skin colour
   *
   * 6313
   *
   * Measurement of the colouring of the epidermis of a fruit.
   */
  FruitSkinColour = "AEZ",
  /**
   * Angle of bend
   *
   * 6313
   *
   * The measured angle of bend.
   */
  AngleBend = "AF",
  /**
   * Fixed incremental measurement
   *
   * 6313
   *
   * The measurement of the fixed increment.
   */
  FixedIncrementalMeasurement = "AFA",
  /**
   * Durofel D10
   *
   * 6313
   *
   * Measure of the elastic force of the pulp of a fruit. It is measured with a penetrating stamp with diameter 10.
   */
  DurofelD10 = "AFB",
  /**
   * Durofel D25
   *
   * 6313
   *
   * Measure of the elastic force of the pulp of a fruit. It is measured with a penetrating stamp with diameter 25.
   */
  DurofelD25 = "AFC",
  /**
   * Durofel D50
   *
   * 6313
   *
   * Measure of the elastic force of the pulp of a fruit. It is measured with a penetrating stamp with diameter 50.
   */
  DurofelD50 = "AFD",
  /**
   * Maximum stacking weight
   *
   * 6313
   *
   * The maximum weight which may be stacked upon a product or package without the product or packaging being crushed.
   */
  MaximumStackingWeight = "AFE",
  /**
   * Gross measure cube
   *
   * 6313
   *
   * The total cubic space occupied by an item, taking into account any protruding components, arrived at by multiplying the maximum length, width and height.
   */
  GrossMeasureCube = "AFF",
  /**
   * Percentage fat content in dry matter
   *
   * 6313
   *
   * The percentage of fat content in dry matter.
   */
  PercentageFatContentInDryMatter = "AFG",
  /**
   * Saccharometric content
   *
   * 6313
   *
   * Measurement of the sugar content of a solution.
   */
  SaccharometricContent = "AFH",
  /**
   * Hydrate content of an alcoholic product after bottling
   *
   * 6313
   *
   * The hydrate content which occurs in an alcoholic product after bottling.
   */
  HydrateContentAnAlcoholicProductAfterBottling = "AFI",
  /**
   * Anhydrous content
   *
   * 6313
   *
   * The non-water content.
   */
  AnhydrousContent = "AFJ",
  /**
   * Certified weight
   *
   * 6313
   *
   * Weight which has been certified.
   */
  CertifiedWeight = "AFK",
  /**
   * Freeboard
   *
   * 6313
   *
   * The vertical distance between the waterline and the upper edge of the deck line.
   */
  Freeboard = "AFL",
  /**
   * Maximum vessel draught
   *
   * 6313
   *
   * The depth of water needed to float the ship (Maximum vessel draught according to Load Line Certificate IMO, MSC/Circ. 920 15 June 1999).
   */
  MaximumVesselDraught = "AFM",
  /**
   * Net explosive weight
   *
   * 6313
   *
   * Mass of the explosive part or substance of goods without any packing.
   */
  NetExplosiveWeight = "AFN",
  /**
   * Radioactive criticality safety index
   *
   * 6313
   *
   * A number, assigned to fissile material i.e. material capable of sustaining a nuclear chain reaction, which is used to provide control over the accumulation of packages containing such material.
   */
  RadioactiveCriticalitySafetyIndex = "AFO",
  /**
   * Waste currently on board
   *
   * 6313
   *
   * Amount of waste on board at the moment of the notification.
   */
  WasteCurrentlyOnBoard = "AFP",
  /**
   * Waste to be delivered at waste reception facility
   *
   * 6313
   *
   * Amount of waste to be delivered to a waste reception facility, e.g. in the port of call.
   */
  WasteToBeDeliveredAtWasteReceptionFacility = "AFQ",
  /**
   * Waste to be generated until next port of call, estimated
   *
   * 6313
   *
   * Estimated amount of waste to be generated between creation of the current notification and arrival in the next port of call, irrespective of use of incinerator or compactor or disposal at sea.
   */
  WasteToBeGeneratedUntilNextPortCallEstimated = "AFR",
  /**
   * Waste remaining on board at departure
   *
   * 6313
   *
   * Amount of waste remaining on board when leaving the port of call.
   */
  WasteRemainingOnBoardAtDeparture = "AFS",
  /**
   * Colour depth
   *
   * 6313
   *
   * The number of distinct colours represented, e.g. in an image or in a display.
   */
  ColourDepth = "AFT",
  /**
   * Colour depth, maximum
   *
   * 6313
   *
   * The maximum number of distinct colours that can be represented, e.g. in an image or in a display.
   */
  ColourDepthMaximum = "AFU",
  /**
   * Image resolution
   *
   * 6313
   *
   * The level of detail of an image.
   */
  ImageResolution = "AFV",
  /**
   * Device resolution, maximum
   *
   * 6313
   *
   * The maximum level of detail produced by a device.
   */
  DeviceResolutionMaximum = "AFW",
  /**
   * Acoustic absorption coefficient
   *
   * 6313
   *
   * The portion of sound energy a surface absorbs, measured at different frequencies.
   */
  AcousticAbsorptionCoefficient = "AFX",
  /**
   * Billed weight
   *
   * 6313
   *
   * The measured billed weight.
   */
  BilledWeight = "B",
  /**
   * Breaking load
   *
   * 6313
   *
   * The measured breaking load.
   */
  BreakingLoad = "BL",
  /**
   * Platinum
   *
   * 6313
   *
   * The measurement of the platinum component.
   */
  Platinum = "BMY",
  /**
   * Silver
   *
   * 6313
   *
   * The measurement of the silver component.
   */
  Silver = "BMZ",
  /**
   * List
   *
   * 6313
   *
   * The leaning or inclination of a vessel expressed in degrees port or starboard.
   */
  List = "BNA",
  /**
   * Trim
   *
   * 6313
   *
   * The condition of a vessel with reference to its longitudinal axis.
   */
  Trim = "BNB",
  /**
   * Free water
   *
   * 6313
   *
   * The volume of water present in a container that is not in suspension in the contained liquid.
   */
  FreeWater = "BNC",
  /**
   * Bands
   *
   * 6313
   *
   * The measured bands.
   */
  Bands = "BND",
  /**
   * API (American Petroleum Institute) gravity
   *
   * 6313
   *
   * The relative density of petroleum liquids as specified by a standard developed by the API.
   */
  ApiAmericanPetroleumInstituteGravity = "BNE",
  /**
   * Petroleum gross observed volume
   *
   * 6313
   *
   * The total volume of all petroleum liquids and sediment and water, excluding free water, at observed temperature and pressure.
   */
  PetroleumGrossObservedVolume = "BNF",
  /**
   * Petroleum gross standard volume
   *
   * 6313
   *
   * The total volume of all petroleum liquids, sediment, and water excluding free water, corrected by the appropriate volume correction factor for the observed temperature and American Petroleum Institute relative density, or density to a standard temperature.
   */
  PetroleumGrossStandardVolume = "BNG",
  /**
   * Volume variance
   *
   * 6313
   *
   * The change in the volume measurement.
   */
  VolumeVariance = "BNH",
  /**
   * Petroleum net standard volume
   *
   * 6313
   *
   * The total volume of all petroleum liquids, excluding sediment and water and free water, corrected by the appropriate volume correction factor for the observed temperature and American Petroleum Institute gravity relative to density or to a standard temperature.
   */
  PetroleumNetStandardVolume = "BNI",
  /**
   * Material on-board quantity, after discharge
   *
   * 6313
   *
   * The material in vessel tanks, void spaces, and pipelines after discharge.
   */
  MaterialOnBoardQuantityAfterDischarge = "BNJ",
  /**
   * Petroleum total calculated volume
   *
   * 6313
   *
   * The total volume of all petroleum liquids, sediment and water corrected by the appropriate volume correction factor for the observed temperature and American Petroleum Institute (API) gravity, relative density, or density to a standard temperature.
   */
  PetroleumTotalCalculatedVolume = "BNK",
  /**
   * Petroleum total observed volume
   *
   * 6313
   *
   * The total volume of all petroleum liquids, sediment and water and free water at observed temperature and pressure.
   */
  PetroleumTotalObservedVolume = "BNL",
  /**
   * Innage gauge distance
   *
   * 6313
   *
   * The measured distance from the surface of the liquid to a fixed datum plate or to the tank bottom.
   */
  InnageGaugeDistance = "BNM",
  /**
   * Petroleum net standard weight
   *
   * 6313
   *
   * The total weight of all petroleum liquids excluding sediments, water and free water.
   */
  PetroleumNetStandardWeight = "BNN",
  /**
   * Sediment and water in petroleum
   *
   * 6313
   *
   * The measurement of non-hydrocarbon solid material and water in suspension in petroleum liquid.
   */
  SedimentAndWaterInPetroleum = "BNO",
  /**
   * Observed reference height, tank
   *
   * 6313
   *
   * The observed distance from the tank bottom or datum plate to the established reference point.
   */
  ObservedReferenceHeightTank = "BNP",
  /**
   * Reference height, tank
   *
   * 6313
   *
   * The measured distance from the tank bottom or datum plate to the established reference point.
   */
  ReferenceHeightTank = "BNQ",
  /**
   * Ullage gauge distance
   *
   * 6313
   *
   * The measured distance from the cargo liquid surface to the reference point.
   */
  UllageGaugeDistance = "BNR",
  /**
   * Trim correction
   *
   * 6313
   *
   * The correction applied to the observed gauge or observed volume when a vessel is not on an even keel.
   */
  TrimCorrection = "BNS",
  /**
   * Bow to bridge distance
   *
   * 6313
   *
   * The distance between the bow and the bridge of a vessel.
   */
  BowToBridgeDistance = "BNT",
  /**
   * Peg hole number
   *
   * 6313
   *
   * Used to identify the peg hole if more than one hole is present in the product or packaging.
   */
  PegHoleNumber = "BNU",
  /**
   * Number of inner packs
   *
   * 6313
   *
   * Indicates the number of non coded physical groupings (inner packs) of next lower level trade items within the current trade item level.
   */
  NumberInnerPacks = "BNV",
  /**
   * Number of next level trade items within inner pack
   *
   * 6313
   *
   * The number of next lower level trade items contained within the physical non-coded grouping (inner pack).
   */
  NumberNextLevelTradeItemsWithinInnerPack = "BNW",
  /**
   * Number of trade items per pallet layer
   *
   * 6313
   *
   * The number of trade items contained on a single layer of a pallet.
   */
  NumberTradeItemsPerPalletLayer = "BNX",
  /**
   * Packed items layer height
   *
   * 6313
   *
   * The height of a single layer of packed items.
   */
  PackedItemsLayerHeight = "BNY",
  /**
   * Packing material weight, skin tight covering
   *
   * 6313
   *
   * The weight measurement of the packing material used for skin tight covering (e.g. when packaging prepared meats, poultry, cheese, and other food products).
   */
  PackingMaterialWeightSkinTightCovering = "BNZ",
  /**
   * Brightness
   *
   * 6313
   *
   * The measured brightness.
   */
  Brightness = "BR",
  /**
   * Brakes
   *
   * 6313
   *
   * The measured brakes.
   */
  Brakes = "BRA",
  /**
   * Components labelled for recycling percentage
   *
   * 6313
   *
   * Percentage of trade item components that clearly label how to facilitate product disassembly and recycling.
   */
  ComponentsLabelledForRecyclingPercentage = "BRB",
  /**
   * Renewable plastic components percentage, by net weight
   *
   * 6313
   *
   * The percentage of the plastic components only made from rapidly renewable plant-based material by net weight of product.
   */
  RenewablePlasticComponentsPercentageByNetWeight = "BRC",
  /**
   * Clamp pressure, required
   *
   * 6313
   *
   * The pressure that should be applied by a clamp.
   */
  ClampPressureRequired = "BRD",
  /**
   * Break
   *
   * 6313
   *
   * The measured break.
   */
  Break = "BRE",
  /**
   * Ascertained volume
   *
   * 6313
   *
   * Endorsement of the true volume as ascertained or verified.
   */
  AscertainedVolume = "BRF",
  /**
   * Unit weight
   *
   * 6313
   *
   * The weight of a unit.
   */
  UnitWeight = "BRG",
  /**
   * Total volume
   *
   * 6313
   *
   * Total volume of goods and/or parcels.
   */
  TotalVolume = "BRH",
  /**
   * Unit volume
   *
   * 6313
   *
   * The volume of a unit.
   */
  UnitVolume = "BRI",
  /**
   * Vertical center of gravity
   *
   * 6313
   *
   * Distance of vertical center of gravity relative to item's base.
   */
  VerticalCenterGravity = "BRJ",
  /**
   * Maximum allowable transport stacking weight
   *
   * 6313
   *
   * Maximum weight allowed to be stowed on top of an item during transport.
   */
  MaximumAllowableTransportStackingWeight = "BRK",
  /**
   * Carbon Dioxide
   *
   * 6313
   *
   * Measurement of the carbon dioxide (C02) component.
   */
  CarbonDioxide = "BRL",
  /**
   * Number of base units per pallet
   *
   * 6313
   *
   * The number of base units contained on a pallet.
   */
  NumberBaseUnitsPerPallet = "BRM",
  /**
   * Colony forming unit
   *
   * 6313
   *
   * Micro-organism colonies that are to be counted under determined conditions.
   */
  ColonyFormingUnit = "BRN",
  /**
   * Diluted liquid volume
   *
   * 6313
   *
   * The volume of liquid which results after a dilution agent has been added, e.g. undiluted orange juice of 200ml, after dilution with water the volume of diluted liquid equals 1 litre.
   */
  DilutedLiquidVolume = "BRO",
  /**
   * Energy efficiency
   *
   * 6313
   *
   * A measurement of the energy efficiency of an article.
   */
  EnergyEfficiency = "BRP",
  /**
   * Number of layers
   *
   * 6313
   *
   * Number of layers of a product or products within a package, container, etc.
   */
  NumberLayers = "BRQ",
  /**
   * Maximum demand
   *
   * 6313
   *
   * The highest demand recorded during the period of recording of usage of the supply.
   */
  MaximumDemand = "BRR",
  /**
   * Number of pallet places
   *
   * 6313
   *
   * The number of pallet places needed to store or to transport pallets (can be stacked).
   */
  NumberPalletPlaces = "BRS",
  /**
   * Package net measurement, cubed
   *
   * 6313
   *
   * An indication of the net cubed measurement of a package.
   */
  PackageNetMeasurementCubed = "BRT",
  /**
   * Power factor
   *
   * 6313
   *
   * The ratio of the power dissipated (active power in kW) to the total power (which is the product of the input volts times amps given in kVa). When equipment which uses reactive power is being operated the power factor will be less than one.
   */
  PowerFactor = "BRU",
  /**
   * Stacking factor excluding bottom item
   *
   * 6313
   *
   * Maximum number of identical items stackable on top of item.
   */
  StackingFactorExcludingBottomItem = "BRV",
  /**
   * Breaking strength
   *
   * 6313
   *
   * The measured breaking strength.
   */
  BreakingStrength = "BS",
  /**
   * Breaking strength wet
   *
   * 6313
   *
   * The measured breaking strength when wet.
   */
  BreakingStrengthWet = "BSW",
  /**
   * Step size
   *
   * 6313
   *
   * An indication of measurements in which options contained within a Customer Specific Article are available, e.g. 10 metre planks of wood may be sold in step sizes of 2 metres.
   */
  StepSize = "BSX",
  /**
   * Number of units per package
   *
   * 6313
   *
   * The number of identified units per package.
   */
  NumberUnitsPerPackage = "BSY",
  /**
   * Number of units per layer
   *
   * 6313
   *
   * Number of units of a product or package within one layer of a package, container, etc.
   */
  NumberUnitsPerLayer = "BSZ",
  /**
   * Weight per running metre
   *
   * 6313
   *
   * A code used to indicate the weight per running metre of floor coverings over floor area.
   */
  WeightPerRunningMetre = "BTA",
  /**
   * Weight per square metre
   *
   * 6313
   *
   * A code used to indicate the weight per square metre of floor coverings over floor area.
   */
  WeightPerSquareMetre = "BTB",
  /**
   * Acidity of meat
   *
   * 6313
   *
   * The meat's acid quality or condition.
   */
  AcidityMeat = "BTC",
  /**
   * Slaughtering weight
   *
   * 6313
   *
   * Weight immediately after slaughter.
   */
  SlaughteringWeight = "BTD",
  /**
   * Stacking factor including bottom item
   *
   * 6313
   *
   * Maximum number of items stackable upon each other, including the bottom item.
   */
  StackingFactorIncludingBottomItem = "BTE",
  /**
   * Number of units in the width of a layer
   *
   * 6313
   *
   * Number of units of a product or package which make up the width of a layer in a package, container, pallet, etc.
   */
  NumberUnitsInWidthALayer = "BTF",
  /**
   * Number of units in the depth of a layer
   *
   * 6313
   *
   * Number of units of a product or package which make up the depth of a layer in a package, container, pallet, etc.
   */
  NumberUnitsInDepthALayer = "BTG",
  /**
   * Nestable percentage
   *
   * 6313
   *
   * Extent expressed as a percentage to which an item can be nested within an identical item, e.g. paper cups.
   */
  NestablePercentage = "BTH",
  /**
   * Gross weight including carrier's equipment
   *
   * 6313
   *
   * Weight (mass) of goods including packaging and the carrier's equipment. In this context 'carrier's equipment' means any material resources necessary to facilitate the transport and handling of the goods without having the ability to move by its own propulsion, e.g. pallet, container, etc.
   */
  GrossWeightIncludingCarriersEquipment = "BTI",
  /**
   * Sugar content
   *
   * 6313
   *
   * The rate of sugar.
   */
  SugarContent = "BTJ",
  /**
   * Self-accelerating polymerization temperature (SAPT)
   *
   * 6313
   *
   * The lowest temperature at which polymerization may occur for a substance as packed for transport.
   */
  SelfAcceleratingPolymerizationTemperatureSapt = "BTK",
  /**
   * Self-accelerating decomposition temperature (SADT)
   *
   * 6313
   *
   * The lowest temperature at which self-accelerating decomposition may occur for a substance as packed for transport.
   */
  SelfAcceleratingDecompositionTemperatureSadt = "BTL",
  /**
   * Control temperature
   *
   * 6313
   *
   * The controlled transport temperature to avoid decomposition of self-reactive substances and organic peroxides.
   */
  ControlTemperature = "BTM",
  /**
   * Basis weight
   *
   * 6313
   *
   * The measured basis weight.
   */
  BasisWeight = "BW",
  /**
   * Change
   *
   * 6313
   *
   * The measured change.
   */
  Change = "CHN",
  /**
   * Emergency temperature
   *
   * 6313
   *
   * The temperature at which emergency response is required for self-reactive substances and organic peroxides.
   */
  EmergencyTemperature = "CHO",
  /**
   * Colour
   *
   * 6313
   *
   * The measured colour.
   */
  Colour = "CM",
  /**
   * Contents of package
   *
   * 6313
   *
   * In combination with the other data elements of the actual segment this code indicates the measured content of a package.
   */
  ContentsPackage = "CT",
  /**
   * Commercial weight
   *
   * 6313
   *
   * Item weight considering its maximum possible humidity.
   */
  CommercialWeight = "CV",
  /**
   * Core length
   *
   * 6313
   *
   * To specify length of core on which product is to be placed.
   */
  CoreLength = "CZ",
  /**
   * Destination weight agreement
   *
   * 6313
   *
   * The agreed weight of despatched goods whose weight may change during transport.
   */
  DestinationWeightAgreement = "D",
  /**
   * Diameter
   *
   * 6313
   *
   * Diameter of an article.
   */
  Diameter = "DI",
  /**
   * Delta value L
   *
   * 6313
   *
   * The measured delta value L.
   */
  DeltaValueL = "DL",
  /**
   * Density
   *
   * 6313
   *
   * The measured density.
   */
  Density = "DN",
  /**
   * Depth
   *
   * 6313
   *
   * The measured depth.
   */
  Depth = "DP",
  /**
   * Denier
   *
   * 6313
   *
   * The measured fineness of a material.
   */
  Denier = "DR",
  /**
   * Distance between points
   *
   * 6313
   *
   * The measured distance between points.
   */
  DistanceBetweenPoints = "DS",
  /**
   * Width, boxcar door
   *
   * 6313
   *
   * The measured width of a boxcar door.
   */
  WidthBoxcarDoor = "DW",
  /**
   * Estimated new weight
   *
   * 6313
   *
   * The measured estimated new weight.
   */
  EstimatedNewWeight = "E",
  /**
   * Elongation
   *
   * 6313
   *
   * The measured elongation.
   */
  Elongation = "EA",
  /**
   * Deficit weight
   *
   * 6313
   *
   * The measured deficit weight.
   */
  DeficitWeight = "F",
  /**
   * Filament count
   *
   * 6313
   *
   * Used e.g. in textile, print industries.
   */
  FilamentCount = "FI",
  /**
   * Longitudinal flatness
   *
   * 6313
   *
   * The measured longitudinal flatness.
   */
  LongitudinalFlatness = "FL",
  /**
   * Flatness
   *
   * 6313
   *
   * The measured flatness.
   */
  Flatness = "FN",
  /**
   * Transverse flatness
   *
   * 6313
   *
   * The measured transverse flatness.
   */
  TransverseFlatness = "FV",
  /**
   * Gauge
   *
   * 6313
   *
   * The measured gauge.
   */
  Gauge = "GG",
  /**
   * Gross weight, maximum
   *
   * 6313
   *
   * The measured maximum gross weight.
   */
  GrossWeightMaximum = "GW",
  /**
   * Hardness
   *
   * 6313
   *
   * The measured hardness.
   */
  Hardness = "HF",
  /**
   * Height, maximum
   *
   * 6313
   *
   * The measured maximum height.
   */
  HeightMaximum = "HM",
  /**
   * Height dimension
   *
   * 6313
   *
   * Numeric value of height.
   */
  HeightDimension = "HT",
  /**
   * Impact energy
   *
   * 6313
   *
   * The measured impact energy.
   */
  ImpactEnergy = "IB",
  /**
   * Inside diameter
   *
   * 6313
   *
   * The measured inside diameter.
   */
  InsideDiameter = "ID",
  /**
   * Legal weight
   *
   * 6313
   *
   * The measured legal weight.
   */
  LegalWeight = "L",
  /**
   * Length, maximum
   *
   * 6313
   *
   * The measured maximum length.
   */
  LengthMaximum = "LM",
  /**
   * Length
   *
   * 6313
   *
   * To specify the value of a length dimension.
   */
  Length = "LN",
  /**
   * Lost end
   *
   * 6313
   *
   * The measured lost end.
   */
  LostEnd = "LND",
  /**
   * Minimum weight
   *
   * 6313
   *
   * The measured minimum weight.
   */
  MinimumWeight = "M",
  /**
   * Moisture
   *
   * 6313
   *
   * Measurement application is the moisture content of the item.
   */
  Moisture = "MO",
  /**
   * Maximum weight
   *
   * 6313
   *
   * The measured maximum weight.
   */
  MaximumWeight = "MW",
  /**
   * Actual net weight
   *
   * 6313
   *
   * The actual weight of the goods excluding packaging.
   */
  ActualNetWeight = "N",
  /**
   * Outside diameter
   *
   * 6313
   *
   * The measured outside diameter.
   */
  OutsideDiameter = "OD",
  /**
   * Pre stretch
   *
   * 6313
   *
   * Measurement identifying the amount an item has been stretched prior to use.
   */
  PreStretch = "PRS",
  /**
   * Per tonne
   *
   * 6313
   *
   * A measurement per tonne.
   */
  PerTonne = "PTN",
  /**
   * Relative humidity
   *
   * 6313
   *
   * The measured relative humidity.
   */
  RelativeHumidity = "RA",
  /**
   * Resistivity
   *
   * 6313
   *
   * The measured resistivity.
   */
  Resistivity = "RF",
  /**
   * Rockwell C
   *
   * 6313
   *
   * Hardness in the Rockwell C scale.
   */
  RockwellC = "RJ",
  /**
   * Ream weight
   *
   * 6313
   *
   * Measurement indication for paper.
   */
  ReamWeight = "RMW",
  /**
   * Reduction of area
   *
   * 6313
   *
   * The measured reduction of area.
   */
  ReductionArea = "RP",
  /**
   * Run (process)
   *
   * 6313
   *
   * The measured run (process).
   */
  RunProcess = "RUN",
  /**
   * Ratio
   *
   * 6313
   *
   * The measured ratio.
   */
  Ratio = "RY",
  /**
   * Shipped quantity
   *
   * 6313
   *
   * The measured shipped quantity.
   */
  ShippedQuantity = "SQ",
  /**
   * Tare weight
   *
   * 6313
   *
   * Weight excluding goods and loose accessories.
   */
  TareWeight = "T",
  /**
   * Temperature
   *
   * 6313
   *
   * A measurement in relation to temperature.
   */
  Temperature = "TC",
  /**
   * Thickness
   *
   * 6313
   *
   * The measured thickness.
   */
  Thickness = "TH",
  /**
   * Time period
   *
   * 6313
   *
   * Measurement of a specific length of time.
   */
  TimePeriod = "TN",
  /**
   * Time
   *
   * 6313
   *
   * The measured time.
   */
  Time = "TT",
  /**
   * Transport equipment verified gross mass (weight)
   *
   * 6313
   *
   * The gross mass (weight) of the transport equipment verified according to SOLAS Chapter VI, Regulation 2, paragraphs 4-6.
   */
  TransportEquipmentVerifiedGrossMassWeight = "VGM",
  /**
   * Height, van door
   *
   * 6313
   *
   * The height of the door of a van or container.
   */
  HeightVanDoor = "VH",
  /**
   * Width, van door
   *
   * 6313
   *
   * The width of the door of a van or container.
   */
  WidthVanDoor = "VW",
  /**
   * Weight per unit of area
   *
   * 6313
   *
   * The weight per unit of an area.
   */
  WeightPerUnitArea = "WA",
  /**
   * Width dimension
   *
   * 6313
   *
   * Numeric value of width.
   */
  WidthDimension = "WD",
  /**
   * Width, maximum
   *
   * 6313
   *
   * The maximum distance from side to side.
   */
  WidthMaximum = "WM",
  /**
   * Weight per unit of length
   *
   * 6313
   *
   * The weight per unit of length.
   */
  WeightPerUnitLength = "WU",
  /**
   * Side height, flat bed with removable sides
   *
   * 6313
   *
   * The height of the removable sides of a flat bed truck.
   */
  SideHeightFlatBedWithRemovableSides = "XH",
  /**
   * Squareness
   *
   * 6313
   *
   * The measured squareness.
   */
  Squareness = "XQ",
  /**
   * Spool size
   *
   * 6313
   *
   * The measured spool size.
   */
  SpoolSize = "XZ",
  /**
   * Yield stress
   *
   * 6313
   *
   * The measured yield stress.
   */
  YieldStress = "YS",
  /**
   * Aluminium
   *
   * 6313
   *
   * The measured chemical element aluminium.
   */
  Aluminium = "ZAL",
  /**
   * Arsenic
   *
   * 6313
   *
   * The measured chemical element arsenic.
   */
  Arsenic = "ZAS",
  /**
   * Boron
   *
   * 6313
   *
   * The measured chemical element boron.
   */
  Boron = "ZB",
  /**
   * Bismuth
   *
   * 6313
   *
   * The measured chemical element bismuth.
   */
  Bismuth = "ZBI",
  /**
   * Carbon
   *
   * 6313
   *
   * The measured chemical element carbon.
   */
  Carbon = "ZC",
  /**
   * Calcium
   *
   * 6313
   *
   * The measured chemical element calcium.
   */
  Calcium = "ZCA",
  /**
   * Columbium
   *
   * 6313
   *
   * The measured chemical element columbium.
   */
  Columbium = "ZCB",
  /**
   * Cerium
   *
   * 6313
   *
   * The measured chemical element cerium.
   */
  Cerium = "ZCE",
  /**
   * Chlorine
   *
   * 6313
   *
   * The measured chemical element chlorine.
   */
  Chlorine = "ZCL",
  /**
   * Cobalt
   *
   * 6313
   *
   * The measured chemical element cobalt.
   */
  Cobalt = "ZCO",
  /**
   * Chromium
   *
   * 6313
   *
   * The measured chemical element chromium.
   */
  Chromium = "ZCR",
  /**
   * Copper
   *
   * 6313
   *
   * The measured chemical element copper.
   */
  Copper = "ZCU",
  /**
   * Iron
   *
   * 6313
   *
   * The measured chemical element iron.
   */
  Iron = "ZFE",
  /**
   * Iron plus silicon
   *
   * 6313
   *
   * The measured substance iron plus silicon.
   */
  IronPlusSilicon = "ZFS",
  /**
   * Germanium
   *
   * 6313
   *
   * The measured chemical element germanium.
   */
  Germanium = "ZGE",
  /**
   * Hydrogen
   *
   * 6313
   *
   * The measured chemical element hydrogen.
   */
  Hydrogen = "ZH",
  /**
   * Potassium
   *
   * 6313
   *
   * The measured chemical element potassium.
   */
  Potassium = "ZK",
  /**
   * Magnesium
   *
   * 6313
   *
   * The measured chemical element magnesium.
   */
  Magnesium = "ZMG",
  /**
   * Manganese
   *
   * 6313
   *
   * The measured chemical element manganese.
   */
  Manganese = "ZMN",
  /**
   * Molybdenum
   *
   * 6313
   *
   * The measured chemical element molybdenum.
   */
  Molybdenum = "ZMO",
  /**
   * Nitrogen
   *
   * 6313
   *
   * The measured chemical element nitrogen.
   */
  Nitrogen = "ZN",
  /**
   * Sodium
   *
   * 6313
   *
   * The measured chemical element sodium.
   */
  Sodium = "ZNA",
  /**
   * Niobium
   *
   * 6313
   *
   * The chemical element niobium.
   */
  Niobium = "ZNB",
  /**
   * Nickel
   *
   * 6313
   *
   * The measured chemical element nickel.
   */
  Nickel = "ZNI",
  /**
   * Oxygen
   *
   * 6313
   *
   * The measured chemical element oxygen.
   */
  Oxygen = "ZO",
  /**
   * Phosphorus
   *
   * 6313
   *
   * The measured chemical element phosphorus.
   */
  Phosphorus = "ZP",
  /**
   * Lead
   *
   * 6313
   *
   * The measured chemical element lead.
   */
  Lead = "ZPB",
  /**
   * Sulphur
   *
   * 6313
   *
   * The measured chemical element sulphur.
   */
  Sulphur = "ZS",
  /**
   * Antimony
   *
   * 6313
   *
   * The measured chemical element antimony.
   */
  Antimony = "ZSB",
  /**
   * Selenium
   *
   * 6313
   *
   * The measured chemical element selenium.
   */
  Selenium = "ZSE",
  /**
   * Silicon
   *
   * 6313
   *
   * The measured chemical element silicon.
   */
  Silicon = "ZSI",
  /**
   * Silicium oxyd
   *
   * 6313
   *
   * The measured substance silicium oxyd.
   */
  SiliciumOxyd = "ZSL",
  /**
   * Tin
   *
   * 6313
   *
   * The measured chemical element tin.
   */
  Tin = "ZSN",
  /**
   * Tantalium
   *
   * 6313
   *
   * The measured chemical element tantalium.
   */
  Tantalium = "ZTA",
  /**
   * Tellurium
   *
   * 6313
   *
   * The measured chemical element tellurium.
   */
  Tellurium = "ZTE",
  /**
   * Titanium
   *
   * 6313
   *
   * The measured chemical element titanium.
   */
  Titanium = "ZTI",
  /**
   * Vanadium
   *
   * 6313
   *
   * The measured chemical element vanadium.
   */
  Vanadium = "ZV",
  /**
   * Tungsten
   *
   * 6313
   *
   * The measured chemical element tungsten.
   */
  Tungsten = "ZW",
  /**
   * Waste content
   *
   * 6313
   *
   * The measured waste content.
   */
  WasteContent = "ZWA",
  /**
   * Zinc
   *
   * 6313
   *
   * The measured chemical element zinc.
   */
  Zinc = "ZZN",
  /**
   * Zirconium
   *
   * 6313
   *
   * The measured chemical element zirconium.
   */
  Zirconium = "ZZR",
  /**
   * Mutually defined
   *
   * 6313
   *
   * A code assigned within a code list to be used on an interim basis and as defined among trading partners until a precise code can be assigned to the code list.
   */
  MutuallyDefined = "ZZZ",
  /**
   * Best before date
   *
   * Factur-X
   */
  BestBeforeDate = "BEST_BEFORE_DATE",
  /**
   * Colour as text
   *
   * Factur-X
   */
  ColourAsText = "COLOR_TEXT",
  /**
   * Commission indicator
   *
   * Factur-X
   *
   * true / false
   */
  CommissionIndicator = "COMMISSION",
  /**
   * Deposit system
   *
   * Factur-X
   *
   * Permitted values: DISPOSABLE RETURNABLE
   */
  DepositSystem = "DEPOSIT_SYSTEM",
  /**
   * Deposit type
   *
   * Factur-X
   *
   * Permitted values: PRODUCT_PACKAGING TRANSPORT_EQUIPMENT
   */
  DepositType = "DEPOSIT_TYPE",
  /**
   * Energy efficiency class
   *
   * Factur-X
   */
  EnergyEfficiencyClass = "ENERGY_CLASS",
  /**
   * Expiration date
   *
   * Factur-X
   */
  ExpirationDate = "EXPIRATION_DATE",
  /**
   * Fee indicator
   *
   * Factur-X
   *
   * true = item is a fee
   */
  FeeIndicator = "FEE",
  /**
   * Type of article
   *
   * Factur-X
   *
   * Allowed values: GOODS OTHER_SERVICES
   */
  TypeArticle = "KIND_OF_ARTICLE",
  /**
   * Material of the product
   *
   * Factur-X
   */
  MaterialProduct = "MATERIAL",
  /**
   * Metering point designation, e.g. for electricity or gas
   *
   * Factur-X
   */
  MeteringPointDesignationEGForElectricityOrGas = "METER_LOCATION",
  /**
   * Meter number, e.g. for electricity or gas
   *
   * Factur-X
   */
  MeterNumberEGForElectricityOrGas = "METER_NUMBER",
  /**
   * Organic control body number
   *
   * Factur-X
   */
  OrganicControlBodyNumber = "ORGANIC_CONTROL_BODY",
  /**
   * Packaging material
   *
   * Factur-X
   */
  PackagingMaterial = "PACKAGING_MATERIAL",
  /**
   * Type of packaging (code)
   *
   * Factur-X
   *
   * Type of packaging in coded form. The packaging units from Rec 20 / Rec 21 are used as the code list. Example XBO=bottle
   */
  TypePackagingCode = "PACKAGING_TYPE",
  /**
   * Number of the action variant
   *
   * Factur-X
   */
  NumberActionVariant = "PROMOTIONAL_VARIANT",
  /**
   * Seal number
   *
   * Factur-X
   */
  SealNumber = "SEAL_NUMBER",
  /**
   * Size code
   *
   * Factur-X
   *
   * Size information in coded form
   */
  SizeCode = "SIZE_CODE",
  /**
   * Size designation
   *
   * Factur-X
   *
   * Size specifications in text form
   */
  SizeDesignation = "SIZE_TEXT",
  /**
   * Type of trading unit
   *
   * Factur-X
   *
   * Permitted values: MIXED_ASSORTMENT = Assortment UNMIXED_UNIT = Container
   */
  TypeTradingUnit = "TRADING_UNIT",
  /**
   * Waste code (EWC)
   *
   * Factur-X
   *
   * Waste code according to the European Waste Catalogue (EWC)
   */
  WasteCodeEwc = "WASTE_CODE",
  /**
   * Waste fraction
   *
   * Factur-X
   */
  WasteFraction = "WASTE_FRACTION",
  /**
   * WEEE registration number
   *
   * Factur-X
   *
   * WEEE registration number of the manufacturer of the product
   */
  WeeeRegistrationNumber = "WEEE_NUMBER",
}

export function description(value: Characteristic): string {
  switch (value) {
    case Characteristic.ConsolidatedWeight:
      return "Consolidated weight";
    case Characteristic.NetWeight:
      return "Net weight";
    case Characteristic.GoodsItemGrossWeight:
      return "Goods item gross weight";
    case Characteristic.TotalNetWeight:
      return "Total net weight";
    case Characteristic.ConsignmentGrossWeight:
      return "Consignment gross weight";
    case Characteristic.NetNetWeight:
      return "Net net weight";
    case Characteristic.SternThrust:
      return "Stern thrust";
    case Characteristic.BowThrust:
      return "Bow thrust";
    case Characteristic.HydrateContentAnAlcoholicProductAtBottling:
      return "Hydrate content of an alcoholic product at bottling";
    case Characteristic.NumberUnitsPerPallet:
      return "Number of units per pallet";
    case Characteristic.FatContent:
      return "Fat content";
    case Characteristic.TransportMeansGrossWeight:
      return "Transport means gross weight";
    case Characteristic.NetTonnageVessel:
      return "Net tonnage of the vessel";
    case Characteristic.Humidity:
      return "Humidity";
    case Characteristic.Voltage:
      return "Voltage";
    case Characteristic.PowerConsumption:
      return "Power consumption";
    case Characteristic.HeatDissipation:
      return "Heat dissipation";
    case Characteristic.AirFlow:
      return "Air flow";
    case Characteristic.ShockImpact:
      return "Shock impact";
    case Characteristic.OperativeTemperature:
      return "Operative temperature";
    case Characteristic.NonOperativeTemperature:
      return "Non operative temperature";
    case Characteristic.GrossVolume:
      return "Gross volume";
    case Characteristic.NetVolume:
      return "Net volume";
    case Characteristic.WaterContent:
      return "Water content";
    case Characteristic.TensileStress:
      return "Tensile stress";
    case Characteristic.Fibrosity:
      return "Fibrosity";
    case Characteristic.GaugeLength:
      return "Gauge length";
    case Characteristic.Radius:
      return "Radius";
    case Characteristic.Straightness:
      return "Straightness";
    case Characteristic.Strain:
      return "Strain";
    case Characteristic.ItemWidthWhenUnrolled:
      return "Item width when unrolled";
    case Characteristic.ItemLengthWhenUnrolled:
      return "Item length when unrolled";
    case Characteristic.ItemAreaWhenUnrolled:
      return "Item area when unrolled";
    case Characteristic.OriginalWort:
      return "Original wort";
    case Characteristic.Volume:
      return "Volume";
    case Characteristic.Angle:
      return "Angle";
    case Characteristic.PegHoleHorizontalDistanceFromPackageLeftmostEdge:
      return "Peg hole horizontal distance from package leftmost edge";
    case Characteristic.PegHoleVerticalDistanceFromPackageTop:
      return "Peg hole vertical distance from package top";
    case Characteristic.NumberLayersPerPallet:
      return "Number of layers per pallet";
    case Characteristic.ProductStrenghChemical:
      return "Product strengh, chemical";
    case Characteristic.ProductStrengthBasisChemical:
      return "Product strength basis, chemical";
    case Characteristic.ItemWeight:
      return "Item weight";
    case Characteristic.PayloadWeightMaximum:
      return "Payload weight, maximum";
    case Characteristic.WeightConveyance:
      return "Weight of conveyance";
    case Characteristic.ConveyanceSummerDeadWeight:
      return "Conveyance summer dead weight";
    case Characteristic.ContainerizedCargoOnVesselsWeight:
      return "Containerized cargo on vessel's weight";
    case Characteristic.NonContainerizedCargoOnVesselsWeight:
      return "Non-containerized cargo on vessel's weight";
    case Characteristic.AscertainedWeight:
      return "Ascertained weight";
    case Characteristic.ChargeableWeight:
      return "Chargeable weight";
    case Characteristic.EstimatedGrossWeight:
      return "Estimated gross weight";
    case Characteristic.EstimatedVolume:
      return "Estimated volume";
    case Characteristic.VesselOverallLength:
      return "Vessel overall length";
    case Characteristic.LoadingMeters:
      return "Loading meters";
    case Characteristic.NumberAxles:
      return "Number of axles";
    case Characteristic.Payload:
      return "Payload";
    case Characteristic.StartPositionInLength:
      return "Start position in the length";
    case Characteristic.EndPositionInLength:
      return "End position in the length";
    case Characteristic.StartPositionInWidth:
      return "Start position in the width";
    case Characteristic.EndPositionInWidth:
      return "End position in the width";
    case Characteristic.StartPositionInThickness:
      return "Start position in the thickness";
    case Characteristic.EndPositionInThickness:
      return "End position in the thickness";
    case Characteristic.TransportContainerActualFillingWeight:
      return "Transport container actual filling weight";
    case Characteristic.TransportContainerMaximumCapacity:
      return "Transport container maximum capacity";
    case Characteristic.DeclaredNetWeight:
      return "Declared net weight";
    case Characteristic.LoadingHeight:
      return "Loading height";
    case Characteristic.StackingHeight:
      return "Stacking height";
    case Characteristic.CalculatedWeight:
      return "Calculated weight";
    case Characteristic.Ferrite:
      return "Ferrite";
    case Characteristic.Impurity:
      return "Impurity";
    case Characteristic.GrainSize:
      return "Grain size";
    case Characteristic.Lanthanides:
      return "Lanthanides";
    case Characteristic.Elasticity:
      return "Elasticity";
    case Characteristic.DrainedWeight:
      return "Drained weight";
    case Characteristic.Gallium:
      return "Gallium";
    case Characteristic.Strontium:
      return "Strontium";
    case Characteristic.Area:
      return "Area";
    case Characteristic.EquipmentStorageLimitation:
      return "Equipment storage limitation";
    case Characteristic.RadioactiveIndexTransport:
      return "Radioactive index of transport";
    case Characteristic.Radioactivity:
      return "Radioactivity";
    case Characteristic.AverageGrossWeight:
      return "Average gross weight";
    case Characteristic.ForwardDraft:
      return "Forward draft";
    case Characteristic.AfterDraft:
      return "After draft";
    case Characteristic.Acidity:
      return "Acidity";
    case Characteristic.TransportEquipmentGrossWeight:
      return "Transport equipment gross weight";
    case Characteristic.TotalTransportEquipmentGrossWeight:
      return "Total transport equipment gross weight";
    case Characteristic.AcidityJuice:
      return "Acidity of juice";
    case Characteristic.Penetrometry:
      return "Penetrometry";
    case Characteristic.Durofel:
      return "Durofel";
    case Characteristic.JuiceWeightPer100Grams:
      return "Juice weight per 100 grams";
    case Characteristic.FruitSkinColour:
      return "Fruit skin colour";
    case Characteristic.AngleBend:
      return "Angle of bend";
    case Characteristic.FixedIncrementalMeasurement:
      return "Fixed incremental measurement";
    case Characteristic.DurofelD10:
      return "Durofel D10";
    case Characteristic.DurofelD25:
      return "Durofel D25";
    case Characteristic.DurofelD50:
      return "Durofel D50";
    case Characteristic.MaximumStackingWeight:
      return "Maximum stacking weight";
    case Characteristic.GrossMeasureCube:
      return "Gross measure cube";
    case Characteristic.PercentageFatContentInDryMatter:
      return "Percentage fat content in dry matter";
    case Characteristic.SaccharometricContent:
      return "Saccharometric content";
    case Characteristic.HydrateContentAnAlcoholicProductAfterBottling:
      return "Hydrate content of an alcoholic product after bottling";
    case Characteristic.AnhydrousContent:
      return "Anhydrous content";
    case Characteristic.CertifiedWeight:
      return "Certified weight";
    case Characteristic.Freeboard:
      return "Freeboard";
    case Characteristic.MaximumVesselDraught:
      return "Maximum vessel draught";
    case Characteristic.NetExplosiveWeight:
      return "Net explosive weight";
    case Characteristic.RadioactiveCriticalitySafetyIndex:
      return "Radioactive criticality safety index";
    case Characteristic.WasteCurrentlyOnBoard:
      return "Waste currently on board";
    case Characteristic.WasteToBeDeliveredAtWasteReceptionFacility:
      return "Waste to be delivered at waste reception facility";
    case Characteristic.WasteToBeGeneratedUntilNextPortCallEstimated:
      return "Waste to be generated until next port of call, estimated";
    case Characteristic.WasteRemainingOnBoardAtDeparture:
      return "Waste remaining on board at departure";
    case Characteristic.ColourDepth:
      return "Colour depth";
    case Characteristic.ColourDepthMaximum:
      return "Colour depth, maximum";
    case Characteristic.ImageResolution:
      return "Image resolution";
    case Characteristic.DeviceResolutionMaximum:
      return "Device resolution, maximum";
    case Characteristic.AcousticAbsorptionCoefficient:
      return "Acoustic absorption coefficient";
    case Characteristic.BilledWeight:
      return "Billed weight";
    case Characteristic.BreakingLoad:
      return "Breaking load";
    case Characteristic.Platinum:
      return "Platinum";
    case Characteristic.Silver:
      return "Silver";
    case Characteristic.List:
      return "List";
    case Characteristic.Trim:
      return "Trim";
    case Characteristic.FreeWater:
      return "Free water";
    case Characteristic.Bands:
      return "Bands";
    case Characteristic.ApiAmericanPetroleumInstituteGravity:
      return "API (American Petroleum Institute) gravity";
    case Characteristic.PetroleumGrossObservedVolume:
      return "Petroleum gross observed volume";
    case Characteristic.PetroleumGrossStandardVolume:
      return "Petroleum gross standard volume";
    case Characteristic.VolumeVariance:
      return "Volume variance";
    case Characteristic.PetroleumNetStandardVolume:
      return "Petroleum net standard volume";
    case Characteristic.MaterialOnBoardQuantityAfterDischarge:
      return "Material on-board quantity, after discharge";
    case Characteristic.PetroleumTotalCalculatedVolume:
      return "Petroleum total calculated volume";
    case Characteristic.PetroleumTotalObservedVolume:
      return "Petroleum total observed volume";
    case Characteristic.InnageGaugeDistance:
      return "Innage gauge distance";
    case Characteristic.PetroleumNetStandardWeight:
      return "Petroleum net standard weight";
    case Characteristic.SedimentAndWaterInPetroleum:
      return "Sediment and water in petroleum";
    case Characteristic.ObservedReferenceHeightTank:
      return "Observed reference height, tank";
    case Characteristic.ReferenceHeightTank:
      return "Reference height, tank";
    case Characteristic.UllageGaugeDistance:
      return "Ullage gauge distance";
    case Characteristic.TrimCorrection:
      return "Trim correction";
    case Characteristic.BowToBridgeDistance:
      return "Bow to bridge distance";
    case Characteristic.PegHoleNumber:
      return "Peg hole number";
    case Characteristic.NumberInnerPacks:
      return "Number of inner packs";
    case Characteristic.NumberNextLevelTradeItemsWithinInnerPack:
      return "Number of next level trade items within inner pack";
    case Characteristic.NumberTradeItemsPerPalletLayer:
      return "Number of trade items per pallet layer";
    case Characteristic.PackedItemsLayerHeight:
      return "Packed items layer height";
    case Characteristic.PackingMaterialWeightSkinTightCovering:
      return "Packing material weight, skin tight covering";
    case Characteristic.Brightness:
      return "Brightness";
    case Characteristic.Brakes:
      return "Brakes";
    case Characteristic.ComponentsLabelledForRecyclingPercentage:
      return "Components labelled for recycling percentage";
    case Characteristic.RenewablePlasticComponentsPercentageByNetWeight:
      return "Renewable plastic components percentage, by net weight";
    case Characteristic.ClampPressureRequired:
      return "Clamp pressure, required";
    case Characteristic.Break:
      return "Break";
    case Characteristic.AscertainedVolume:
      return "Ascertained volume";
    case Characteristic.UnitWeight:
      return "Unit weight";
    case Characteristic.TotalVolume:
      return "Total volume";
    case Characteristic.UnitVolume:
      return "Unit volume";
    case Characteristic.VerticalCenterGravity:
      return "Vertical center of gravity";
    case Characteristic.MaximumAllowableTransportStackingWeight:
      return "Maximum allowable transport stacking weight";
    case Characteristic.CarbonDioxide:
      return "Carbon Dioxide";
    case Characteristic.NumberBaseUnitsPerPallet:
      return "Number of base units per pallet";
    case Characteristic.ColonyFormingUnit:
      return "Colony forming unit";
    case Characteristic.DilutedLiquidVolume:
      return "Diluted liquid volume";
    case Characteristic.EnergyEfficiency:
      return "Energy efficiency";
    case Characteristic.NumberLayers:
      return "Number of layers";
    case Characteristic.MaximumDemand:
      return "Maximum demand";
    case Characteristic.NumberPalletPlaces:
      return "Number of pallet places";
    case Characteristic.PackageNetMeasurementCubed:
      return "Package net measurement, cubed";
    case Characteristic.PowerFactor:
      return "Power factor";
    case Characteristic.StackingFactorExcludingBottomItem:
      return "Stacking factor excluding bottom item";
    case Characteristic.BreakingStrength:
      return "Breaking strength";
    case Characteristic.BreakingStrengthWet:
      return "Breaking strength wet";
    case Characteristic.StepSize:
      return "Step size";
    case Characteristic.NumberUnitsPerPackage:
      return "Number of units per package";
    case Characteristic.NumberUnitsPerLayer:
      return "Number of units per layer";
    case Characteristic.WeightPerRunningMetre:
      return "Weight per running metre";
    case Characteristic.WeightPerSquareMetre:
      return "Weight per square metre";
    case Characteristic.AcidityMeat:
      return "Acidity of meat";
    case Characteristic.SlaughteringWeight:
      return "Slaughtering weight";
    case Characteristic.StackingFactorIncludingBottomItem:
      return "Stacking factor including bottom item";
    case Characteristic.NumberUnitsInWidthALayer:
      return "Number of units in the width of a layer";
    case Characteristic.NumberUnitsInDepthALayer:
      return "Number of units in the depth of a layer";
    case Characteristic.NestablePercentage:
      return "Nestable percentage";
    case Characteristic.GrossWeightIncludingCarriersEquipment:
      return "Gross weight including carrier's equipment";
    case Characteristic.SugarContent:
      return "Sugar content";
    case Characteristic.SelfAcceleratingPolymerizationTemperatureSapt:
      return "Self-accelerating polymerization temperature (SAPT)";
    case Characteristic.SelfAcceleratingDecompositionTemperatureSadt:
      return "Self-accelerating decomposition temperature (SADT)";
    case Characteristic.ControlTemperature:
      return "Control temperature";
    case Characteristic.BasisWeight:
      return "Basis weight";
    case Characteristic.Change:
      return "Change";
    case Characteristic.EmergencyTemperature:
      return "Emergency temperature";
    case Characteristic.Colour:
      return "Colour";
    case Characteristic.ContentsPackage:
      return "Contents of package";
    case Characteristic.CommercialWeight:
      return "Commercial weight";
    case Characteristic.CoreLength:
      return "Core length";
    case Characteristic.DestinationWeightAgreement:
      return "Destination weight agreement";
    case Characteristic.Diameter:
      return "Diameter";
    case Characteristic.DeltaValueL:
      return "Delta value L";
    case Characteristic.Density:
      return "Density";
    case Characteristic.Depth:
      return "Depth";
    case Characteristic.Denier:
      return "Denier";
    case Characteristic.DistanceBetweenPoints:
      return "Distance between points";
    case Characteristic.WidthBoxcarDoor:
      return "Width, boxcar door";
    case Characteristic.EstimatedNewWeight:
      return "Estimated new weight";
    case Characteristic.Elongation:
      return "Elongation";
    case Characteristic.DeficitWeight:
      return "Deficit weight";
    case Characteristic.FilamentCount:
      return "Filament count";
    case Characteristic.LongitudinalFlatness:
      return "Longitudinal flatness";
    case Characteristic.Flatness:
      return "Flatness";
    case Characteristic.TransverseFlatness:
      return "Transverse flatness";
    case Characteristic.Gauge:
      return "Gauge";
    case Characteristic.GrossWeightMaximum:
      return "Gross weight, maximum";
    case Characteristic.Hardness:
      return "Hardness";
    case Characteristic.HeightMaximum:
      return "Height, maximum";
    case Characteristic.HeightDimension:
      return "Height dimension";
    case Characteristic.ImpactEnergy:
      return "Impact energy";
    case Characteristic.InsideDiameter:
      return "Inside diameter";
    case Characteristic.LegalWeight:
      return "Legal weight";
    case Characteristic.LengthMaximum:
      return "Length, maximum";
    case Characteristic.Length:
      return "Length";
    case Characteristic.LostEnd:
      return "Lost end";
    case Characteristic.MinimumWeight:
      return "Minimum weight";
    case Characteristic.Moisture:
      return "Moisture";
    case Characteristic.MaximumWeight:
      return "Maximum weight";
    case Characteristic.ActualNetWeight:
      return "Actual net weight";
    case Characteristic.OutsideDiameter:
      return "Outside diameter";
    case Characteristic.PreStretch:
      return "Pre stretch";
    case Characteristic.PerTonne:
      return "Per tonne";
    case Characteristic.RelativeHumidity:
      return "Relative humidity";
    case Characteristic.Resistivity:
      return "Resistivity";
    case Characteristic.RockwellC:
      return "Rockwell C";
    case Characteristic.ReamWeight:
      return "Ream weight";
    case Characteristic.ReductionArea:
      return "Reduction of area";
    case Characteristic.RunProcess:
      return "Run (process)";
    case Characteristic.Ratio:
      return "Ratio";
    case Characteristic.ShippedQuantity:
      return "Shipped quantity";
    case Characteristic.TareWeight:
      return "Tare weight";
    case Characteristic.Temperature:
      return "Temperature";
    case Characteristic.Thickness:
      return "Thickness";
    case Characteristic.TimePeriod:
      return "Time period";
    case Characteristic.Time:
      return "Time";
    case Characteristic.TransportEquipmentVerifiedGrossMassWeight:
      return "Transport equipment verified gross mass (weight)";
    case Characteristic.HeightVanDoor:
      return "Height, van door";
    case Characteristic.WidthVanDoor:
      return "Width, van door";
    case Characteristic.WeightPerUnitArea:
      return "Weight per unit of area";
    case Characteristic.WidthDimension:
      return "Width dimension";
    case Characteristic.WidthMaximum:
      return "Width, maximum";
    case Characteristic.WeightPerUnitLength:
      return "Weight per unit of length";
    case Characteristic.SideHeightFlatBedWithRemovableSides:
      return "Side height, flat bed with removable sides";
    case Characteristic.Squareness:
      return "Squareness";
    case Characteristic.SpoolSize:
      return "Spool size";
    case Characteristic.YieldStress:
      return "Yield stress";
    case Characteristic.Aluminium:
      return "Aluminium";
    case Characteristic.Arsenic:
      return "Arsenic";
    case Characteristic.Boron:
      return "Boron";
    case Characteristic.Bismuth:
      return "Bismuth";
    case Characteristic.Carbon:
      return "Carbon";
    case Characteristic.Calcium:
      return "Calcium";
    case Characteristic.Columbium:
      return "Columbium";
    case Characteristic.Cerium:
      return "Cerium";
    case Characteristic.Chlorine:
      return "Chlorine";
    case Characteristic.Cobalt:
      return "Cobalt";
    case Characteristic.Chromium:
      return "Chromium";
    case Characteristic.Copper:
      return "Copper";
    case Characteristic.Iron:
      return "Iron";
    case Characteristic.IronPlusSilicon:
      return "Iron plus silicon";
    case Characteristic.Germanium:
      return "Germanium";
    case Characteristic.Hydrogen:
      return "Hydrogen";
    case Characteristic.Potassium:
      return "Potassium";
    case Characteristic.Magnesium:
      return "Magnesium";
    case Characteristic.Manganese:
      return "Manganese";
    case Characteristic.Molybdenum:
      return "Molybdenum";
    case Characteristic.Nitrogen:
      return "Nitrogen";
    case Characteristic.Sodium:
      return "Sodium";
    case Characteristic.Niobium:
      return "Niobium";
    case Characteristic.Nickel:
      return "Nickel";
    case Characteristic.Oxygen:
      return "Oxygen";
    case Characteristic.Phosphorus:
      return "Phosphorus";
    case Characteristic.Lead:
      return "Lead";
    case Characteristic.Sulphur:
      return "Sulphur";
    case Characteristic.Antimony:
      return "Antimony";
    case Characteristic.Selenium:
      return "Selenium";
    case Characteristic.Silicon:
      return "Silicon";
    case Characteristic.SiliciumOxyd:
      return "Silicium oxyd";
    case Characteristic.Tin:
      return "Tin";
    case Characteristic.Tantalium:
      return "Tantalium";
    case Characteristic.Tellurium:
      return "Tellurium";
    case Characteristic.Titanium:
      return "Titanium";
    case Characteristic.Vanadium:
      return "Vanadium";
    case Characteristic.Tungsten:
      return "Tungsten";
    case Characteristic.WasteContent:
      return "Waste content";
    case Characteristic.Zinc:
      return "Zinc";
    case Characteristic.Zirconium:
      return "Zirconium";
    case Characteristic.MutuallyDefined:
      return "Mutually defined";
    case Characteristic.BestBeforeDate:
      return "Best before date";
    case Characteristic.ColourAsText:
      return "Colour as text";
    case Characteristic.CommissionIndicator:
      return "Commission indicator";
    case Characteristic.DepositSystem:
      return "Deposit system";
    case Characteristic.DepositType:
      return "Deposit type";
    case Characteristic.EnergyEfficiencyClass:
      return "Energy efficiency class";
    case Characteristic.ExpirationDate:
      return "Expiration date";
    case Characteristic.FeeIndicator:
      return "Fee indicator";
    case Characteristic.TypeArticle:
      return "Type of article";
    case Characteristic.MaterialProduct:
      return "Material of the product";
    case Characteristic.MeteringPointDesignationEGForElectricityOrGas:
      return "Metering point designation, e.g. for electricity or gas";
    case Characteristic.MeterNumberEGForElectricityOrGas:
      return "Meter number, e.g. for electricity or gas";
    case Characteristic.OrganicControlBodyNumber:
      return "Organic control body number";
    case Characteristic.PackagingMaterial:
      return "Packaging material";
    case Characteristic.TypePackagingCode:
      return "Type of packaging (code)";
    case Characteristic.NumberActionVariant:
      return "Number of the action variant";
    case Characteristic.SealNumber:
      return "Seal number";
    case Characteristic.SizeCode:
      return "Size code";
    case Characteristic.SizeDesignation:
      return "Size designation";
    case Characteristic.TypeTradingUnit:
      return "Type of trading unit";
    case Characteristic.WasteCodeEwc:
      return "Waste code (EWC)";
    case Characteristic.WasteFraction:
      return "Waste fraction";
    case Characteristic.WeeeRegistrationNumber:
      return "WEEE registration number";
  }
}
