#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

impl std::fmt::Display for HybridDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for HybridDocument {
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
    }
}

impl crate::Code for HybridDocument {
    fn code(self) -> &'static str {
        match self {
            HybridDocument::TheHybridDocumentContainsAnInvoiceOrCreditNote => "INVOICE",
            HybridDocument::TheHybridDocumentContainsAnOrder => "ORDER",
            HybridDocument::TheHybridDocumentContainsAnOrderResponse => "ORDER_RESPONSE",
            HybridDocument::ThyHybridDocumentContainsAnOrderChange => "ORDER_CHANGE",
        }
    }
}

impl crate::Description for HybridDocument {
    fn description(self) -> &'static str {
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
