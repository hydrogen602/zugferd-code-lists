export enum INCOTERMS {
  /**
   * Delivery arranged by supplier
   */
  DeliveryArrangedBySupplier = "1",
  /**
   * Delivery arranged by logistic service provider
   */
  DeliveryArrangedByLogisticServiceProvider = "2",
  /**
   * Cost and Freight
   */
  CostAndFreight = "CFR",
  /**
   * Cost, Insurance and Freight
   */
  CostInsuranceAndFreight = "CIF",
  /**
   * Carriage and Insurance Paid to (insert named place of destination)
   */
  CarriageAndInsurancePaidToInsertNamedPlaceDestination = "CIP",
  /**
   * Carriage Paid To (insert named place of destination)
   */
  CarriagePaidToInsertNamedPlaceDestination = "CPT",
  /**
   * Delivered At Place (insert named place of destination)
   */
  DeliveredAtPlaceInsertNamedPlaceDestination = "DAP",
  /**
   * Delivered Duty Paid (insert named place of destination)
   */
  DeliveredDutyPaidInsertNamedPlaceDestination = "DDP",
  /**
   * Delivered At Place Unloaded (insert named place of unloading)
   */
  DeliveredAtPlaceUnloadedInsertNamedPlaceUnloading = "DPU",
  /**
   * Ex Works (insert named place of delivery)
   */
  ExWorksInsertNamedPlaceDelivery = "EXW",
  /**
   * Free Alongside Ship (insert named port of shipment)
   */
  FreeAlongsideShipInsertNamedPortShipment = "FAS",
  /**
   * Free Carrier (insert named place of delivery)
   */
  FreeCarrierInsertNamedPlaceDelivery = "FCA",
  /**
   *  Free On Board (insert named port of shipment)
   */
  FreeOnBoardInsertNamedPortShipment = "FOB",
}

export function description(value: INCOTERMS): string {
  switch (value) {
    case INCOTERMS.DeliveryArrangedBySupplier:
      return "Delivery arranged by supplier";
    case INCOTERMS.DeliveryArrangedByLogisticServiceProvider:
      return "Delivery arranged by logistic service provider";
    case INCOTERMS.CostAndFreight:
      return "Cost and Freight";
    case INCOTERMS.CostInsuranceAndFreight:
      return "Cost, Insurance and Freight";
    case INCOTERMS.CarriageAndInsurancePaidToInsertNamedPlaceDestination:
      return "Carriage and Insurance Paid to (insert named place of destination)";
    case INCOTERMS.CarriagePaidToInsertNamedPlaceDestination:
      return "Carriage Paid To (insert named place of destination)";
    case INCOTERMS.DeliveredAtPlaceInsertNamedPlaceDestination:
      return "Delivered At Place (insert named place of destination)";
    case INCOTERMS.DeliveredDutyPaidInsertNamedPlaceDestination:
      return "Delivered Duty Paid (insert named place of destination)";
    case INCOTERMS.DeliveredAtPlaceUnloadedInsertNamedPlaceUnloading:
      return "Delivered At Place Unloaded (insert named place of unloading)";
    case INCOTERMS.ExWorksInsertNamedPlaceDelivery:
      return "Ex Works (insert named place of delivery)";
    case INCOTERMS.FreeAlongsideShipInsertNamedPortShipment:
      return "Free Alongside Ship (insert named port of shipment)";
    case INCOTERMS.FreeCarrierInsertNamedPlaceDelivery:
      return "Free Carrier (insert named place of delivery)";
    case INCOTERMS.FreeOnBoardInsertNamedPortShipment:
      return " Free On Board (insert named port of shipment)";
  }
}
