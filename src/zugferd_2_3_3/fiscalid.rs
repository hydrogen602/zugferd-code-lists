#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum FiscalID {
    /// Fiscal number
    FiscalNumber,
}

impl std::fmt::Display for FiscalID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for FiscalID {
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
    }
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
