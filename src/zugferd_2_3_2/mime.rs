#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum MIME {
    /// application/pdf
    ApplicationPdf,
    /// image/png
    ImagePng,
    /// image/jpeg
    ImageJpeg,
    /// text/csv
    TextCsv,
    /// application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
    ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet,
    /// application/vnd.oasis.opendocument.spreadsheet
    ApplicationVndOasisOpendocumentSpreadsheet,
    /// application/xml
    ApplicationXml,
    /// text/xml
    TextXml,
}

impl crate::Code for MIME {
    fn code(&self) -> &str {
        match self {
            MIME::ApplicationPdf => "application/pdf",
            MIME::ImagePng => "image/png",
            MIME::ImageJpeg => "image/jpeg",
            MIME::TextCsv => "text/csv",
            MIME::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            }
            MIME::ApplicationVndOasisOpendocumentSpreadsheet => {
                "application/vnd.oasis.opendocument.spreadsheet"
            }
            MIME::ApplicationXml => "application/xml",
            MIME::TextXml => "text/xml",
        }
    }
}

impl crate::Description for MIME {
    fn description(&self) -> &str {
        match self {
            MIME::ApplicationPdf => "application/pdf",
            MIME::ImagePng => "image/png",
            MIME::ImageJpeg => "image/jpeg",
            MIME::TextCsv => "text/csv",
            MIME::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            }
            MIME::ApplicationVndOasisOpendocumentSpreadsheet => {
                "application/vnd.oasis.opendocument.spreadsheet"
            }
            MIME::ApplicationXml => "application/xml",
            MIME::TextXml => "text/xml",
        }
    }
}

impl crate::FromCode for MIME {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "application/pdf" => Some(MIME::ApplicationPdf),
            "image/png" => Some(MIME::ImagePng),
            "image/jpeg" => Some(MIME::ImageJpeg),
            "text/csv" => Some(MIME::TextCsv),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
                Some(MIME::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet)
            }
            "application/vnd.oasis.opendocument.spreadsheet" => {
                Some(MIME::ApplicationVndOasisOpendocumentSpreadsheet)
            }
            "application/xml" => Some(MIME::ApplicationXml),
            "text/xml" => Some(MIME::TextXml),
            _ => None,
        }
    }
}
