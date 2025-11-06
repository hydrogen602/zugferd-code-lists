
export enum Filename {
    /**
    * The hybrid document contains a Factur-X / ZUGFeRD XML-file
    *
    * The ConformanceLevel MUST not be XRECHNUNG
    */
    TheHybridDocumentContainsAFacturXZugferdXmlFile = "factur-x.xml",
    /**
    * The hybrid document contains a XRechnung XML file
    *
    * The ConformanceLevel MUST be XRECHNUNG
    */
    TheHybridDocumentContainsAXrechnungXmlFile = "xrechnung.xml",
    /**
    * The hybrid document contains an Order-X XML file
    */
    TheHybridDocumentContainsAnOrderXXmlFile = "order-x.xml",
}

export function description(value: Filename): string {
    switch (value) {
        case Filename.TheHybridDocumentContainsAFacturXZugferdXmlFile: return "The hybrid document contains a Factur-X / ZUGFeRD XML-file";
        case Filename.TheHybridDocumentContainsAXrechnungXmlFile: return "The hybrid document contains a XRechnung XML file";
        case Filename.TheHybridDocumentContainsAnOrderXXmlFile: return "The hybrid document contains an Order-X XML file";
    }
}
