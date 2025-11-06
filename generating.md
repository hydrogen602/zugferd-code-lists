# Generating Definitions based on spec files

## Finding the spec files

[ZUGFeRD 2.3.2 spec files](https://www.ferd-net.de/en/downloads/publications/details/zugferd-232-deutsch)

[ZUGFeRD 2.3.3 spec files](https://www.ferd-net.de/en/downloads/publications/details/zugferd-233-deutsch)

1. Grab the spec files
2. Extract the zip to a directory
3. Locate the code lists file, it should start with `4. EN16931+FacturX code lists values`
4. Copy it into `spec/zugferd_2_3_2` or `spec/zugferd_2_3_3` (depending on the version)
5. run `make gen` to generate the definitions
