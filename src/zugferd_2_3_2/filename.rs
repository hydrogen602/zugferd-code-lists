#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Filename {
    /// The hybrid document contains a Factur-X / ZUGFeRD XML-file
    ///
    /// The ConformanceLevel MUST not be XRECHNUNG
    TheHybridDocumentContainsAFacturXZugferdXmlFile,
    /// The hybrid document contains a XRechnung XML file
    ///
    /// The ConformanceLevel MUST be XRECHNUNG
    TheHybridDocumentContainsAXrechnungXmlFile,
    /// The hybrid document contains an Order-X XML file
    TheHybridDocumentContainsAnOrderXXmlFile,
}

impl crate::Code for Filename {
    fn code(self) -> &'static str {
        match self {
            Filename::TheHybridDocumentContainsAFacturXZugferdXmlFile => "factur-x.xml",
            Filename::TheHybridDocumentContainsAXrechnungXmlFile => "xrechnung.xml",
            Filename::TheHybridDocumentContainsAnOrderXXmlFile => "order-x.xml",
        }
    }
}

impl crate::Description for Filename {
    fn description(self) -> &'static str {
        match self {
            Filename::TheHybridDocumentContainsAFacturXZugferdXmlFile => {
                "The hybrid document contains a Factur-X / ZUGFeRD XML-file"
            }
            Filename::TheHybridDocumentContainsAXrechnungXmlFile => {
                "The hybrid document contains a XRechnung XML file"
            }
            Filename::TheHybridDocumentContainsAnOrderXXmlFile => {
                "The hybrid document contains an Order-X XML file"
            }
        }
    }
}

impl crate::FromCode for Filename {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "factur-x.xml" => Some(Filename::TheHybridDocumentContainsAFacturXZugferdXmlFile),
            "xrechnung.xml" => Some(Filename::TheHybridDocumentContainsAXrechnungXmlFile),
            "order-x.xml" => Some(Filename::TheHybridDocumentContainsAnOrderXXmlFile),
            _ => None,
        }
    }
}
