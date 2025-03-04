#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HybridConformance {
    /// The included document uses a MINIMUM profile
    TheIncludedDocumentUsesAMinimumProfile,
    /// The included document uses a Basic Without Lines profile
    TheIncludedDocumentUsesABasicWithoutLinesProfile,
    /// The included document uses a Basic profile
    TheIncludedDocumentUsesABasicProfile,
    /// The included document uses a Comfort profile
    TheIncludedDocumentUsesAComfortProfile,
    /// The included document uses a EN 16931 profile
    TheIncludedDocumentUsesAEn16931Profile,
    /// The included document uses a Comfort profile
    TheIncludedDocumentUsesAComfortProfile_Dup,
    /// The included document uses an XRECHNUNG profile
    TheIncludedDocumentUsesAnXrechnungProfile,
}

impl crate::Code for HybridConformance {
    fn code(&self) -> &str {
        match self {
            HybridConformance::TheIncludedDocumentUsesAMinimumProfile => "MINIMUM",
            HybridConformance::TheIncludedDocumentUsesABasicWithoutLinesProfile => "BASIC WL",
            HybridConformance::TheIncludedDocumentUsesABasicProfile => "BASIC",
            HybridConformance::TheIncludedDocumentUsesAComfortProfile => "COMFORT",
            HybridConformance::TheIncludedDocumentUsesAEn16931Profile => "EN 16931",
            HybridConformance::TheIncludedDocumentUsesAComfortProfile_Dup => "EXTENDED",
            HybridConformance::TheIncludedDocumentUsesAnXrechnungProfile => "XRECHNUNG",
        }
    }
}

impl crate::Description for HybridConformance {
    fn description(&self) -> &str {
        match self {
            HybridConformance::TheIncludedDocumentUsesAMinimumProfile => {
                "The included document uses a MINIMUM profile"
            }
            HybridConformance::TheIncludedDocumentUsesABasicWithoutLinesProfile => {
                "The included document uses a Basic Without Lines profile"
            }
            HybridConformance::TheIncludedDocumentUsesABasicProfile => {
                "The included document uses a Basic profile"
            }
            HybridConformance::TheIncludedDocumentUsesAComfortProfile => {
                "The included document uses a Comfort profile"
            }
            HybridConformance::TheIncludedDocumentUsesAEn16931Profile => {
                "The included document uses a EN 16931 profile"
            }
            HybridConformance::TheIncludedDocumentUsesAComfortProfile_Dup => {
                "The included document uses a Comfort profile"
            }
            HybridConformance::TheIncludedDocumentUsesAnXrechnungProfile => {
                "The included document uses an XRECHNUNG profile"
            }
        }
    }
}

impl crate::FromCode for HybridConformance {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "MINIMUM" => Some(HybridConformance::TheIncludedDocumentUsesAMinimumProfile),
            "BASIC WL" => Some(HybridConformance::TheIncludedDocumentUsesABasicWithoutLinesProfile),
            "BASIC" => Some(HybridConformance::TheIncludedDocumentUsesABasicProfile),
            "COMFORT" => Some(HybridConformance::TheIncludedDocumentUsesAComfortProfile),
            "EN 16931" => Some(HybridConformance::TheIncludedDocumentUsesAEn16931Profile),
            "EXTENDED" => Some(HybridConformance::TheIncludedDocumentUsesAComfortProfile_Dup),
            "XRECHNUNG" => Some(HybridConformance::TheIncludedDocumentUsesAnXrechnungProfile),
            _ => None,
        }
    }
}
