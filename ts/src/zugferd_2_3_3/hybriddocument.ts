
export enum HybridDocument {
    /**
    * The hybrid document contains an invoice or credit note
    *
    * Only applicable in Factur-X / ZUGFeRD
    */
    TheHybridDocumentContainsAnInvoiceOrCreditNote = "INVOICE",
    /**
    * The hybrid document contains an order
    *
    * Only applicable in Order-X
    */
    TheHybridDocumentContainsAnOrder = "ORDER",
    /**
    * The hybrid document contains an order response
    *
    * Only applicable in Order-X
    */
    TheHybridDocumentContainsAnOrderResponse = "ORDER_RESPONSE",
    /**
    * Thy hybrid document contains an order change
    *
    * Only applicable in Order-X
    */
    ThyHybridDocumentContainsAnOrderChange = "ORDER_CHANGE",
}

export function description(value: HybridDocument): string {
    switch (value) {
        case HybridDocument.TheHybridDocumentContainsAnInvoiceOrCreditNote: return "The hybrid document contains an invoice or credit note";
        case HybridDocument.TheHybridDocumentContainsAnOrder: return "The hybrid document contains an order";
        case HybridDocument.TheHybridDocumentContainsAnOrderResponse: return "The hybrid document contains an order response";
        case HybridDocument.ThyHybridDocumentContainsAnOrderChange: return "Thy hybrid document contains an order change";
    }
}
