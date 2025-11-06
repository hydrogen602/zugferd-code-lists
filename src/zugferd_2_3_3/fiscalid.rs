#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum FiscalID {
    /// Fiscal number
    FiscalNumber,
}

impl crate::Code for FiscalID {
    fn code(&self) -> &str {
        match self {
            FiscalID::FiscalNumber => "FC",
        }
    }
}

impl crate::Description for FiscalID {
    fn description(&self) -> &str {
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
