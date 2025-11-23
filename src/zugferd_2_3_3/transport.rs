#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum TRANSPORT {
    /// Transport mode not specified
    ///
    /// Transport mode has not been specified_x000D_ Notes:_x000D_ 1) This code can be used when the mode is not known or when information on it is not available at the time of issuing the document concerned.
    TransportModeNotSpecified,
    /// Maritime transport
    ///
    /// Transport of goods and/or persons is by sea.
    MaritimeTransport,
    /// Rail transport
    ///
    /// Transport of goods and/or persons is by rail.
    RailTransport,
    /// Road transport
    ///
    /// Transport of goods and/or persons is by road.
    RoadTransport,
    /// Air transport
    ///
    /// Transport of goods and/or persons is by air.
    AirTransport,
    /// Mail
    ///
    /// Method to convey goods is by mail_x000D_ Notes:_x000D_ 1) This code is provided for practical reasons, despite the fact that mail is not a genuine mode of transport. In many countries, the value of merchandise exported and imported by mail is considerable, but the exporter or importer concerned would be unable to state by which mode postal items had been conveyed.
    Mail,
    /// Multimodal transpo
    ///
    /// Method to convey goods and/or persons is by multimodal transport._x000D_ Notes:_x000D_ 1) This code is provided for practical reasons, despite the fact that multimodal transport is not a genuine mode of transport. It can be used when goods are carried by at least two different modes from a place at which the goods are taken in charge by a transport operator to a place designated for delivery, on the basis of one transport contract. (Operations of pick-up and delivery of goods carried out in the performance of a single mode of transport, as defined in such a contract, shall not be considered as multimodal transport).
    MultimodalTranspo,
    /// Fixed transport installation
    ///
    /// Transport of item is via a fixed transport installation._x000D_ Notes:_x000D_ 1) This code applies to installations for continuous transport such as pipelines, ropeways and electric power lines.
    FixedTransportInstallation,
    /// Inland water transport
    ///
    /// Transport of goods and/or persons is by inland water.
    InlandWaterTransport,
    /// Transport mode not applicable
    ///
    /// The mode of transport is not applicable.
    TransportModeNotApplicable,
}

impl std::fmt::Display for TRANSPORT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for TRANSPORT {
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
    }
}

impl crate::Code for TRANSPORT {
    fn code(self) -> &'static str {
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
    fn description(self) -> &'static str {
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
