# zugferd code lists

![NPM Version](https://img.shields.io/npm/v/zugferd-code-lists)
![Crates.io Version](https://img.shields.io/crates/v/zugferd-code-lists)

This project takes the many code lists that are part of the ZUGFeRD specification and converts them into rust and typescript enums for use in other projects.

**This project is in no way or form affiliated with the official ZUGFeRD project.**

# Status

Versions supported:

- ZUGFeRD 2.3.2
- ZUGFeRD 2.3.3

# Changelog

- v0.1.9: added ZUGFeRD 2.3.3
- v0.1.10: added VAT CAT list
- v0.1.11:
  - Added `TryFrom` impls for conversion to and from the crates `iso_currency` and `iso_country`
  - Fixed bug where code `NA` would be rendered as `nan` in the generated code
  - Added `Fiscal ID` list (new in ZUGFeRD 2.3.3)
- v0.1.12:
  - Added `Debug`, `Display`, and `Error` traits for generated errors for `TryFrom` for `iso_currency` and `iso_country`
- v0.1.13:
  - Adjusted the traits `Code` and `Description` to consider the fact that all code lists are `Copy` and all strings returned live for `'static`. This should make the API more flexible as previously there was a lifetime dependence between a code value and the `&str` returned by `Code::code`

# Future Plans

- `TryFrom` impls for conversion between versions
