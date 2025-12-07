# ZUGFeRD spec files

## Example: How to add ZUGFeRD 2.3.3 spec files

Download the package from [ZUGFeRD 2.3.3 spec files](https://www.ferd-net.de/en/downloads/publications/details/zugferd-233-deutsch), locate the code lists file `4. EN16931+FacturX code lists values v15 - used from 2025-05-15.xlsx`, and copy it into this directory.

After that, run

```sh
make remove-spaces
```

in the top-level project directory.
