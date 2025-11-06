#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum LineReason {
    /// Regular item position (standard case)
    RegularItemPositionStandardCase,
    /// Subtotal or group
    SubtotalOrGroup,
    /// For information only
    ForInformationOnly,
}

impl crate::Code for LineReason {
    fn code(&self) -> &str {
        match self {
            LineReason::RegularItemPositionStandardCase => "DETAIL",
            LineReason::SubtotalOrGroup => "GROUP",
            LineReason::ForInformationOnly => "INFORMATION",
        }
    }
}

impl crate::Description for LineReason {
    fn description(&self) -> &str {
        match self {
            LineReason::RegularItemPositionStandardCase => "Regular item position (standard case)",
            LineReason::SubtotalOrGroup => "Subtotal or group",
            LineReason::ForInformationOnly => "For information only",
        }
    }
}

impl crate::FromCode for LineReason {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized
    {
        match code {
            "DETAIL" => Some(LineReason::RegularItemPositionStandardCase),
            "GROUP" => Some(LineReason::SubtotalOrGroup),
            "INFORMATION" => Some(LineReason::ForInformationOnly),
            _ => None,
        }
    }
}
