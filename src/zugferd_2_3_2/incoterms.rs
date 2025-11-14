#![allow(non_camel_case_types)]

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum INCOTERMS {
    /// Delivery arranged by supplier
    DeliveryArrangedBySupplier,
    /// Delivery arranged by logistic service provider
    DeliveryArrangedByLogisticServiceProvider,
    /// Cost and Freight
    CostAndFreight,
    /// Cost, Insurance and Freight
    CostInsuranceAndFreight,
    /// Carriage and Insurance Paid to (insert named place of destination)
    CarriageAndInsurancePaidToInsertNamedPlaceDestination,
    /// Carriage Paid To (insert named place of destination)
    CarriagePaidToInsertNamedPlaceDestination,
    /// Delivered At Place (insert named place of destination)
    DeliveredAtPlaceInsertNamedPlaceDestination,
    /// Delivered Duty Paid (insert named place of destination)
    DeliveredDutyPaidInsertNamedPlaceDestination,
    /// Delivered At Place Unloaded (insert named place of unloading)
    DeliveredAtPlaceUnloadedInsertNamedPlaceUnloading,
    /// Ex Works (insert named place of delivery)
    ExWorksInsertNamedPlaceDelivery,
    /// Free Alongside Ship (insert named port of shipment)
    FreeAlongsideShipInsertNamedPortShipment,
    /// Free Carrier (insert named place of delivery)
    FreeCarrierInsertNamedPlaceDelivery,
    ///  Free On Board (insert named port of shipment)
    FreeOnBoardInsertNamedPortShipment,
}

impl crate::Code for INCOTERMS {
    fn code(self) -> &'static str {
        match self {
            INCOTERMS::DeliveryArrangedBySupplier => "1",
            INCOTERMS::DeliveryArrangedByLogisticServiceProvider => "2",
            INCOTERMS::CostAndFreight => "CFR",
            INCOTERMS::CostInsuranceAndFreight => "CIF",
            INCOTERMS::CarriageAndInsurancePaidToInsertNamedPlaceDestination => "CIP",
            INCOTERMS::CarriagePaidToInsertNamedPlaceDestination => "CPT",
            INCOTERMS::DeliveredAtPlaceInsertNamedPlaceDestination => "DAP",
            INCOTERMS::DeliveredDutyPaidInsertNamedPlaceDestination => "DDP",
            INCOTERMS::DeliveredAtPlaceUnloadedInsertNamedPlaceUnloading => "DPU",
            INCOTERMS::ExWorksInsertNamedPlaceDelivery => "EXW",
            INCOTERMS::FreeAlongsideShipInsertNamedPortShipment => "FAS",
            INCOTERMS::FreeCarrierInsertNamedPlaceDelivery => "FCA",
            INCOTERMS::FreeOnBoardInsertNamedPortShipment => "FOB",
        }
    }
}

impl crate::Description for INCOTERMS {
    fn description(self) -> &'static str {
        match self {
            INCOTERMS::DeliveryArrangedBySupplier => "Delivery arranged by supplier",
            INCOTERMS::DeliveryArrangedByLogisticServiceProvider => {
                "Delivery arranged by logistic service provider"
            }
            INCOTERMS::CostAndFreight => "Cost and Freight",
            INCOTERMS::CostInsuranceAndFreight => "Cost, Insurance and Freight",
            INCOTERMS::CarriageAndInsurancePaidToInsertNamedPlaceDestination => {
                "Carriage and Insurance Paid to (insert named place of destination)"
            }
            INCOTERMS::CarriagePaidToInsertNamedPlaceDestination => {
                "Carriage Paid To (insert named place of destination)"
            }
            INCOTERMS::DeliveredAtPlaceInsertNamedPlaceDestination => {
                "Delivered At Place (insert named place of destination)"
            }
            INCOTERMS::DeliveredDutyPaidInsertNamedPlaceDestination => {
                "Delivered Duty Paid (insert named place of destination)"
            }
            INCOTERMS::DeliveredAtPlaceUnloadedInsertNamedPlaceUnloading => {
                "Delivered At Place Unloaded (insert named place of unloading)"
            }
            INCOTERMS::ExWorksInsertNamedPlaceDelivery => {
                "Ex Works (insert named place of delivery)"
            }
            INCOTERMS::FreeAlongsideShipInsertNamedPortShipment => {
                "Free Alongside Ship (insert named port of shipment)"
            }
            INCOTERMS::FreeCarrierInsertNamedPlaceDelivery => {
                "Free Carrier (insert named place of delivery)"
            }
            INCOTERMS::FreeOnBoardInsertNamedPortShipment => {
                " Free On Board (insert named port of shipment)"
            }
        }
    }
}

impl crate::FromCode for INCOTERMS {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "1" => Some(INCOTERMS::DeliveryArrangedBySupplier),
            "2" => Some(INCOTERMS::DeliveryArrangedByLogisticServiceProvider),
            "CFR" => Some(INCOTERMS::CostAndFreight),
            "CIF" => Some(INCOTERMS::CostInsuranceAndFreight),
            "CIP" => Some(INCOTERMS::CarriageAndInsurancePaidToInsertNamedPlaceDestination),
            "CPT" => Some(INCOTERMS::CarriagePaidToInsertNamedPlaceDestination),
            "DAP" => Some(INCOTERMS::DeliveredAtPlaceInsertNamedPlaceDestination),
            "DDP" => Some(INCOTERMS::DeliveredDutyPaidInsertNamedPlaceDestination),
            "DPU" => Some(INCOTERMS::DeliveredAtPlaceUnloadedInsertNamedPlaceUnloading),
            "EXW" => Some(INCOTERMS::ExWorksInsertNamedPlaceDelivery),
            "FAS" => Some(INCOTERMS::FreeAlongsideShipInsertNamedPortShipment),
            "FCA" => Some(INCOTERMS::FreeCarrierInsertNamedPlaceDelivery),
            "FOB" => Some(INCOTERMS::FreeOnBoardInsertNamedPortShipment),
            _ => None,
        }
    }
}
