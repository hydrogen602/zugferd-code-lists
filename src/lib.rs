use std::marker::PhantomData;

pub mod zugferd_2_3_2;
pub mod zugferd_2_3_3;
pub mod zugferd_2_4;

pub trait Code {
    fn code(self) -> &'static str;
}

pub trait Description {
    fn description(self) -> &'static str;
}

pub trait FromCode {
    fn from_code(code: &str) -> Option<Self>
    where
        Self: Sized;
}

#[cfg(test)]
mod test {
    // check that pub rules are correctly applied
    #[allow(unused_imports)]
    use crate::zugferd_2_3_2::Country::Germany as Germany232;

    #[allow(unused_imports)]
    use crate::zugferd_2_3_3::Country::Germany as Germany233;

    #[allow(unused_imports)]
    use crate::zugferd_2_4::Country::Germany as Germany24;
}

#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[error("Found invalid code \"{code}\" while trying to parse {type_name}", type_name=std::any::type_name::<T>())]
pub struct ParseError<T> {
    pub code: String,
    type_: PhantomData<T>,
}

impl<T> ParseError<T> {
    pub(crate) fn new(code: String) -> Self {
        Self {
            code,
            type_: PhantomData,
        }
    }
}
