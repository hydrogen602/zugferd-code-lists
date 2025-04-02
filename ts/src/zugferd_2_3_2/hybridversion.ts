
export enum HybridVersion {
    /**
    * The hybrid document is a Factur-X 1.0* compliant document
    *
    * Default value for current specification
    */
    TheHybridDocumentIsAFacturX10CompliantDocument = "1.0",
    /**
    * The hybrid document is a ZUGFeRD 1p0 compliant document
    *
    * Legacy use only. A warning is given if used for a document in the validity period of the current specification 
    */
    TheHybridDocumentIsAZugferd1p0CompliantDocument = "1p0",
    /**
    * The hybrid document is a ZUGFeRD 2p0 compliant document
    *
    * Legacy use only. A warning is given if used for a document in the validity period of the current specification 
    */
    TheHybridDocumentIsAZugferd2p0CompliantDocument = "2p0",
    /**
    * The hybrid document is a ZUGFeRD 2p1 compliant document
    *
    * Legacy use only. A warning is given if used for a document in the validity period of the current specification 
    */
    TheHybridDocumentIsAZugferd2p1CompliantDocument = "2p1",
    /**
    * The hybrid document is a ZUGFeRD 2p2 compliant document
    *
    * Legacy use only. A warning is given if used for a document in the validity period of the current specification 
    */
    TheHybridDocumentIsAZugferd2p2CompliantDocument = "2p2",
}

export function description(value: HybridVersion): string {
    switch (value) {
        case HybridVersion.TheHybridDocumentIsAFacturX10CompliantDocument: return "The hybrid document is a Factur-X 1.0* compliant document";
        case HybridVersion.TheHybridDocumentIsAZugferd1p0CompliantDocument: return "The hybrid document is a ZUGFeRD 1p0 compliant document";
        case HybridVersion.TheHybridDocumentIsAZugferd2p0CompliantDocument: return "The hybrid document is a ZUGFeRD 2p0 compliant document";
        case HybridVersion.TheHybridDocumentIsAZugferd2p1CompliantDocument: return "The hybrid document is a ZUGFeRD 2p1 compliant document";
        case HybridVersion.TheHybridDocumentIsAZugferd2p2CompliantDocument: return "The hybrid document is a ZUGFeRD 2p2 compliant document";
    }
}
