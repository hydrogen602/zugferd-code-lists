#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum HybridVersion {
    /// The hybrid document is a Factur-X 1.0* compliant document
    ///
    /// Default value for current specification
    TheHybridDocumentIsAFacturX10CompliantDocument,
    /// The hybrid document is a ZUGFeRD 1p0 compliant document
    ///
    /// Legacy use only. A warning is given if used for a document in the validity period of the current specification
    TheHybridDocumentIsAZugferd1p0CompliantDocument,
    /// The hybrid document is a ZUGFeRD 2p0 compliant document
    ///
    /// Legacy use only. A warning is given if used for a document in the validity period of the current specification
    TheHybridDocumentIsAZugferd2p0CompliantDocument,
    /// The hybrid document is a ZUGFeRD 2p1 compliant document
    ///
    /// Legacy use only. A warning is given if used for a document in the validity period of the current specification
    TheHybridDocumentIsAZugferd2p1CompliantDocument,
    /// The hybrid document is a ZUGFeRD 2p2 compliant document
    ///
    /// Legacy use only. A warning is given if used for a document in the validity period of the current specification
    TheHybridDocumentIsAZugferd2p2CompliantDocument,
}

impl std::fmt::Display for HybridVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for HybridVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s).ok_or(())
    }
}

impl crate::Code for HybridVersion {
    fn code(self) -> &'static str {
        match self {
            HybridVersion::TheHybridDocumentIsAFacturX10CompliantDocument => "1.0",
            HybridVersion::TheHybridDocumentIsAZugferd1p0CompliantDocument => "1p0",
            HybridVersion::TheHybridDocumentIsAZugferd2p0CompliantDocument => "2p0",
            HybridVersion::TheHybridDocumentIsAZugferd2p1CompliantDocument => "2p1",
            HybridVersion::TheHybridDocumentIsAZugferd2p2CompliantDocument => "2p2",
        }
    }
}

impl crate::Description for HybridVersion {
    fn description(self) -> &'static str {
        match self {
            HybridVersion::TheHybridDocumentIsAFacturX10CompliantDocument => {
                "The hybrid document is a Factur-X 1.0* compliant document"
            }
            HybridVersion::TheHybridDocumentIsAZugferd1p0CompliantDocument => {
                "The hybrid document is a ZUGFeRD 1p0 compliant document"
            }
            HybridVersion::TheHybridDocumentIsAZugferd2p0CompliantDocument => {
                "The hybrid document is a ZUGFeRD 2p0 compliant document"
            }
            HybridVersion::TheHybridDocumentIsAZugferd2p1CompliantDocument => {
                "The hybrid document is a ZUGFeRD 2p1 compliant document"
            }
            HybridVersion::TheHybridDocumentIsAZugferd2p2CompliantDocument => {
                "The hybrid document is a ZUGFeRD 2p2 compliant document"
            }
        }
    }
}

impl crate::FromCode for HybridVersion {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "1.0" => Some(HybridVersion::TheHybridDocumentIsAFacturX10CompliantDocument),
            "1p0" => Some(HybridVersion::TheHybridDocumentIsAZugferd1p0CompliantDocument),
            "2p0" => Some(HybridVersion::TheHybridDocumentIsAZugferd2p0CompliantDocument),
            "2p1" => Some(HybridVersion::TheHybridDocumentIsAZugferd2p1CompliantDocument),
            "2p2" => Some(HybridVersion::TheHybridDocumentIsAZugferd2p2CompliantDocument),
            _ => None,
        }
    }
}
