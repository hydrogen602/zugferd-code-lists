#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Date {
    /// CCYYMMDD
    ///
    /// Calendar date: C = Century ; Y = Year ; M = Month ; D = Day.
    Ccyymmdd,
    /// CCYYMMDDHHMMZHHMM
    ///
    /// Calendar date including time and time zone expressed in hours and minutes. ZHHMM = time zone given as offset from Coordinated Universal Time (UTC).
    Ccyymmddhhmmzhhmm,
}

impl crate::Code for Date {
    fn code(&self) -> &str {
        match self {
            Date::Ccyymmdd => "102",
            Date::Ccyymmddhhmmzhhmm => "205",
        }
    }
}

impl crate::Description for Date {
    fn description(&self) -> &str {
        match self {
            Date::Ccyymmdd => "CCYYMMDD",
            Date::Ccyymmddhhmmzhhmm => "CCYYMMDDHHMMZHHMM",
        }
    }
}

impl crate::FromCode for Date {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized,
    {
        match code {
            "102" => Some(Date::Ccyymmdd),
            "205" => Some(Date::Ccyymmddhhmmzhhmm),
            _ => None,
        }
    }
}
