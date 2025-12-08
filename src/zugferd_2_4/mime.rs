#![allow(non_camel_case_types)]

#[cfg_attr(feature = "specta", derive(specta::Type))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
}

impl std::fmt::Display for MIME {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as crate::Code>::code(*self))
    }
}

impl std::str::FromStr for MIME {
    type Err = crate::ParseError<Self>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <Self as crate::FromCode>::from_code(s)
            .ok_or_else(|| crate::ParseError::<Self>::new(s.to_owned()))
    }
}

impl crate::Code for MIME {
    fn code(self) -> &'static str {
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
        }
    }
}

impl crate::Description for MIME {
    fn description(self) -> &'static str {
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
            _ => None,
        }
    }
}

// Start: (Version) TryFrom MIME to crate::zugferd_2_3_3::MIME
impl std::convert::TryFrom<MIME> for crate::zugferd_2_3_3::MIME {
    type Error = std::convert::Infallible;
    fn try_from(value: MIME) -> Result<Self, Self::Error> {
        match value {
            MIME::ApplicationPdf => Ok(crate::zugferd_2_3_3::MIME::ApplicationPdf),
            MIME::ImagePng => Ok(crate::zugferd_2_3_3::MIME::ImagePng),
            MIME::ImageJpeg => Ok(crate::zugferd_2_3_3::MIME::ImageJpeg),
            MIME::TextCsv => Ok(crate::zugferd_2_3_3::MIME::TextCsv),
            MIME::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet => Ok(crate::zugferd_2_3_3::MIME::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet),
            MIME::ApplicationVndOasisOpendocumentSpreadsheet => Ok(crate::zugferd_2_3_3::MIME::ApplicationVndOasisOpendocumentSpreadsheet),

        }
    }
}

impl std::convert::TryFrom<crate::zugferd_2_3_3::MIME> for MIME {
    type Error = ErrFromCrateZugferd233MimeToMime;
    fn try_from(value: crate::zugferd_2_3_3::MIME) -> Result<MIME, Self::Error> {
        match value {
            crate::zugferd_2_3_3::MIME::ApplicationPdf => Ok(MIME::ApplicationPdf),
            crate::zugferd_2_3_3::MIME::ImagePng => Ok(MIME::ImagePng),
            crate::zugferd_2_3_3::MIME::ImageJpeg => Ok(MIME::ImageJpeg),
            crate::zugferd_2_3_3::MIME::TextCsv => Ok(MIME::TextCsv),
            crate::zugferd_2_3_3::MIME::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet => Ok(MIME::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet),
            crate::zugferd_2_3_3::MIME::ApplicationVndOasisOpendocumentSpreadsheet => Ok(MIME::ApplicationVndOasisOpendocumentSpreadsheet),
            crate::zugferd_2_3_3::MIME::ApplicationXml => Err(ErrFromCrateZugferd233MimeToMime::ApplicationXml),
            crate::zugferd_2_3_3::MIME::TextXml => Err(ErrFromCrateZugferd233MimeToMime::TextXml),
        }
    }
}

/// All the variants of crate::zugferd_2_3_3::MIME that are not matched to any variant of MIME
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ErrFromCrateZugferd233MimeToMime {
    ApplicationXml,
    TextXml,
}

impl std::fmt::Display for ErrFromCrateZugferd233MimeToMime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrFromCrateZugferd233MimeToMime::ApplicationXml => {
                write!(f, "ApplicationXml has no corresponding value in MIME")
            }
            ErrFromCrateZugferd233MimeToMime::TextXml => {
                write!(f, "TextXml has no corresponding value in MIME")
            }
        }
    }
}

impl std::error::Error for ErrFromCrateZugferd233MimeToMime {}
// End: (Version) TryFrom crate::zugferd_2_3_3::MIME to MIME
