#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum FiscalID {
    /// Fiscal number
    FiscalNumber,
}

impl crate::Code for FiscalID {
    fn code(self) -> &'static str {
        match self {
            FiscalID::FiscalNumber => "FC",
        }
    }
}

impl crate::Description for FiscalID {
    fn description(self) -> &'static str {
        match self {
            FiscalID::FiscalNumber => "Fiscal number",
        }
    }
}

impl crate::FromCode for FiscalID {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "FC" => Some(FiscalID::FiscalNumber),
            _ => None,
        }
    }
}
