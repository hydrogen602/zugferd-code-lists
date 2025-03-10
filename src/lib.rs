pub mod zugferd_2_3_2;

pub trait Code {
    fn code(&self) -> &str;
}

pub trait Description {
    fn description(&self) -> &str;
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
    use crate::zugferd_2_3_2::Country::Germany;
}
