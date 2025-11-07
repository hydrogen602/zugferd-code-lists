from py_generate.find_package_src import get_enum_variants


def get_iso_country_variants():
    variants = [
        v.decode("utf-8") for v in get_enum_variants("iso_country", "0.1.4", b"Country")
    ]
