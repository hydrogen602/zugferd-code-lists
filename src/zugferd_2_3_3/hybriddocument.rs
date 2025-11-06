#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HybridDocument {
    /// The hybrid document contains an invoice or credit note
    ///
    /// Only applicable in Factur-X / ZUGFeRD
    TheHybridDocumentContainsAnInvoiceOrCreditNote,
    /// The hybrid document contains an order
    ///
    /// Only applicable in Order-X
    TheHybridDocumentContainsAnOrder,
    /// The hybrid document contains an order response
    ///
    /// Only applicable in Order-X
    TheHybridDocumentContainsAnOrderResponse,
    /// Thy hybrid document contains an order change
    ///
    /// Only applicable in Order-X
    ThyHybridDocumentContainsAnOrderChange,
}

impl crate::Code for HybridDocument {
    fn code(&self) -> &str {
        match self {
            HybridDocument::TheHybridDocumentContainsAnInvoiceOrCreditNote => "INVOICE",
            HybridDocument::TheHybridDocumentContainsAnOrder => "ORDER",
            HybridDocument::TheHybridDocumentContainsAnOrderResponse => "ORDER_RESPONSE",
            HybridDocument::ThyHybridDocumentContainsAnOrderChange => "ORDER_CHANGE",
        }
    }
}

impl crate::Description for HybridDocument {
    fn description(&self) -> &str {
        match self {
            HybridDocument::TheHybridDocumentContainsAnInvoiceOrCreditNote => {
                "The hybrid document contains an invoice or credit note"
            }
            HybridDocument::TheHybridDocumentContainsAnOrder => {
                "The hybrid document contains an order"
            }
            HybridDocument::TheHybridDocumentContainsAnOrderResponse => {
                "The hybrid document contains an order response"
            }
            HybridDocument::ThyHybridDocumentContainsAnOrderChange => {
                "Thy hybrid document contains an order change"
            }
        }
    }
}

impl crate::FromCode for HybridDocument {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "INVOICE" => Some(HybridDocument::TheHybridDocumentContainsAnInvoiceOrCreditNote),
            "ORDER" => Some(HybridDocument::TheHybridDocumentContainsAnOrder),
            "ORDER_RESPONSE" => Some(HybridDocument::TheHybridDocumentContainsAnOrderResponse),
            "ORDER_CHANGE" => Some(HybridDocument::ThyHybridDocumentContainsAnOrderChange),
            _ => None,
        }
    }
}
