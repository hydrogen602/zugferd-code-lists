#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum TimeCII {
    /// Date of invoice
    DateInvoice,
    /// Date of delivery of goods to establishments/domicile/site
    DateDeliveryGoodsToEstablishmentsDomicileSite,
    /// Payment date
    PaymentDate,
}

impl crate::Code for TimeCII {
    fn code(&self) -> &str {
        match self {
            TimeCII::DateInvoice => "5",
            TimeCII::DateDeliveryGoodsToEstablishmentsDomicileSite => "29",
            TimeCII::PaymentDate => "72",
        }
    }
}

impl crate::Description for TimeCII {
    fn description(&self) -> &str {
        match self {
            TimeCII::DateInvoice => "Date of invoice",
            TimeCII::DateDeliveryGoodsToEstablishmentsDomicileSite => {
                "Date of delivery of goods to establishments/domicile/site"
            }
            TimeCII::PaymentDate => "Payment date",
        }
    }
}

impl crate::FromCode for TimeCII {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "5" => Some(TimeCII::DateInvoice),
            "29" => Some(TimeCII::DateDeliveryGoodsToEstablishmentsDomicileSite),
            "72" => Some(TimeCII::PaymentDate),
            _ => None,
        }
    }
}
