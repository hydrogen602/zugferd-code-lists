
export enum TRANSPORT {
    /**
    * Transport mode not specified
    *
    * Transport mode has not been specified_x000D_ Notes:_x000D_ 1) This code can be used when the mode is not known or when information on it is not available at the time of issuing the document concerned.
    */
    TransportModeNotSpecified = "0",
    /**
    * Maritime transport
    *
    * Transport of goods and/or persons is by sea.
    */
    MaritimeTransport = "1",
    /**
    * Rail transport
    *
    * Transport of goods and/or persons is by rail.
    */
    RailTransport = "2",
    /**
    * Road transport
    *
    * Transport of goods and/or persons is by road.
    */
    RoadTransport = "3",
    /**
    * Air transport
    *
    * Transport of goods and/or persons is by air.
    */
    AirTransport = "4",
    /**
    * Mail
    *
    * Method to convey goods is by mail_x000D_ Notes:_x000D_ 1) This code is provided for practical reasons, despite the fact that mail is not a genuine mode of transport. In many countries, the value of merchandise exported and imported by mail is considerable, but the exporter or importer concerned would be unable to state by which mode postal items had been conveyed.
    */
    Mail = "5",
    /**
    * Multimodal transpo
    *
    * Method to convey goods and/or persons is by multimodal transport._x000D_ Notes:_x000D_ 1) This code is provided for practical reasons, despite the fact that multimodal transport is not a genuine mode of transport. It can be used when goods are carried by at least two different modes from a place at which the goods are taken in charge by a transport operator to a place designated for delivery, on the basis of one transport contract. (Operations of pick-up and delivery of goods carried out in the performance of a single mode of transport, as defined in such a contract, shall not be considered as multimodal transport).
    */
    MultimodalTranspo = "6",
    /**
    * Fixed transport installation
    *
    * Transport of item is via a fixed transport installation._x000D_ Notes:_x000D_ 1) This code applies to installations for continuous transport such as pipelines, ropeways and electric power lines.
    */
    FixedTransportInstallation = "7",
    /**
    * Inland water transport
    *
    * Transport of goods and/or persons is by inland water.
    */
    InlandWaterTransport = "8",
    /**
    * Transport mode not applicable
    *
    * The mode of transport is not applicable.
    */
    TransportModeNotApplicable = "9",
}

export function description(value: TRANSPORT): string {
    switch (value) {
        case TRANSPORT.TransportModeNotSpecified: return "Transport mode not specified";
        case TRANSPORT.MaritimeTransport: return "Maritime transport";
        case TRANSPORT.RailTransport: return "Rail transport";
        case TRANSPORT.RoadTransport: return "Road transport";
        case TRANSPORT.AirTransport: return "Air transport";
        case TRANSPORT.Mail: return "Mail";
        case TRANSPORT.MultimodalTranspo: return "Multimodal transpo";
        case TRANSPORT.FixedTransportInstallation: return "Fixed transport installation";
        case TRANSPORT.InlandWaterTransport: return "Inland water transport";
        case TRANSPORT.TransportModeNotApplicable: return "Transport mode not applicable";
    }
}
