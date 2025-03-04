#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum TRANSPORT {
    /// Transport mode not specified
    TransportModeNotSpecified,
    /// Maritime transport
    MaritimeTransport,
    /// Rail transport
    RailTransport,
    /// Road transport
    RoadTransport,
    /// Air transport
    AirTransport,
    /// Mail
    Mail,
    /// Multimodal transpo
    MultimodalTranspo,
    /// Fixed transport installation
    FixedTransportInstallation,
    /// Inland water transport
    InlandWaterTransport,
    /// Transport mode not applicable
    TransportModeNotApplicable,
}

impl crate::Code for TRANSPORT {
    fn code(&self) -> &str {
        match self {
            TRANSPORT::TransportModeNotSpecified => "0",
            TRANSPORT::MaritimeTransport => "1",
            TRANSPORT::RailTransport => "2",
            TRANSPORT::RoadTransport => "3",
            TRANSPORT::AirTransport => "4",
            TRANSPORT::Mail => "5",
            TRANSPORT::MultimodalTranspo => "6",
            TRANSPORT::FixedTransportInstallation => "7",
            TRANSPORT::InlandWaterTransport => "8",
            TRANSPORT::TransportModeNotApplicable => "9",
        }
    }
}

impl crate::Description for TRANSPORT {
    fn description(&self) -> &str {
        match self {
            TRANSPORT::TransportModeNotSpecified => "Transport mode not specified",
            TRANSPORT::MaritimeTransport => "Maritime transport",
            TRANSPORT::RailTransport => "Rail transport",
            TRANSPORT::RoadTransport => "Road transport",
            TRANSPORT::AirTransport => "Air transport",
            TRANSPORT::Mail => "Mail",
            TRANSPORT::MultimodalTranspo => "Multimodal transpo",
            TRANSPORT::FixedTransportInstallation => "Fixed transport installation",
            TRANSPORT::InlandWaterTransport => "Inland water transport",
            TRANSPORT::TransportModeNotApplicable => "Transport mode not applicable",
        }
    }
}

impl crate::FromCode for TRANSPORT {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "0" => Some(TRANSPORT::TransportModeNotSpecified),
            "1" => Some(TRANSPORT::MaritimeTransport),
            "2" => Some(TRANSPORT::RailTransport),
            "3" => Some(TRANSPORT::RoadTransport),
            "4" => Some(TRANSPORT::AirTransport),
            "5" => Some(TRANSPORT::Mail),
            "6" => Some(TRANSPORT::MultimodalTranspo),
            "7" => Some(TRANSPORT::FixedTransportInstallation),
            "8" => Some(TRANSPORT::InlandWaterTransport),
            "9" => Some(TRANSPORT::TransportModeNotApplicable),
            _ => None,
        }
    }
}
