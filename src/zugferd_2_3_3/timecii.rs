#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum TimeCII {
    /// Date of invoice
    DateInvoice,
    /// Date of delivery of goods to establishments/domicile/site
    DateDeliveryGoodsToEstablishmentsDomicileSite,
    /// Payment date
    PaymentDate,
}

impl std::fmt::Display for TimeCII {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for TimeCII {
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
    }
}

impl crate::Code for TimeCII {
    fn code(self) -> &'static str {
        match self {
            TimeCII::DateInvoice => "5",
            TimeCII::DateDeliveryGoodsToEstablishmentsDomicileSite => "29",
            TimeCII::PaymentDate => "72",
        }
    }
}

impl crate::Description for TimeCII {
    fn description(self) -> &'static str {
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
