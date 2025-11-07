import json
from pathlib import Path
from subprocess import run
import tree_sitter_rust as tsrust
from tree_sitter import Language, Node, Parser


def get_src_dir(
    package_name: str,
    version: str,
    index: str = "index.crates.io",
    cargo_dir: Path | str = Path("~/.cargo"),
):
    package_name = package_name.replace("-", "_")

    cargo_dir = Path(cargo_dir).expanduser()
    index_src = cargo_dir / "registry" / "src"
    index_file_candidates = list(index_src.glob(f"{index}-*"))
    match index_file_candidates:
        case []:
            raise FileNotFoundError(f"No index file found in {index_src}")
        case [index_file]:
            pass
        case _:
            raise FileNotFoundError(
                f"Ambiguous: multiple index files found in {index_src}"
            )

    package_dir = index_file / f"{package_name}-{version}"
    if not package_dir.exists():
        raise FileNotFoundError(
            f"Package {package_name} {version} not found in {index_file}"
        )
    package_src = package_dir / "src"
    if not package_src.exists():
        raise FileNotFoundError(
            f"Package {package_name} {version} has no src directory"
        )
    return package_src


def find_enum_in_src(package_src: Path, enum_name: bytes):
    for file in package_src.glob("**/*.rs"):
        with open(file, "rb") as f:
            content = f.read()
        if enum_name in content and (enum_node := parse_enum(content, enum_name)):
            return enum_node
    raise Exception(f"Enum {enum_name} not found in {package_src}")


def parse_enum(src_code: bytes, enum_name: bytes) -> Node | None:
    parser = Parser(Language(tsrust.language()))
    tree = parser.parse(src_code)

    for node in tree.root_node.children:
        if node.type != "enum_item":
            continue

        ident = [
            child.text for child in node.children if child.type == "type_identifier"
        ]
        assert (
            len(ident) == 1
        ), f"Rust syntax error? Expected 1 type_identifier in an enum, got {len(ident)}"
        ident = ident[0]

        if ident == enum_name:
            return node

    return None


def parse_enum_variants(enum_node: Node) -> list[bytes]:
    variant_ls = [
        child for child in enum_node.children if child.type == "enum_variant_list"
    ]
    assert len(variant_ls) == 1, "Expected 1 enum_variant_list in an enum"
    variant_ls = variant_ls[0]
    variant_nodes = [
        child for child in variant_ls.children if child.type == "enum_variant"
    ]

    def assert_one[T](ls: list[T | None]) -> T:
        assert len(ls) == 1, f"Expected 1 item, got {len(ls)}"
        assert ls[0] is not None, f"Expected non-None item, got {ls[0]}"
        return ls[0]

    return [
        assert_one(
            [
                child.text
                for child in variant_node.children
                if child.type == "identifier"
            ]
        )
        for variant_node in variant_nodes
    ]


def find_build_rs_out(package_name: str):
    out = run(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"],
        check=True,
        capture_output=True,
    )
    metadata = json.loads(out.stdout)
    build_dir = Path(metadata["build_directory"])
    if not build_dir.exists():
        raise FileNotFoundError(f"Build directory {build_dir} not found")

    possible_dirs = [
        p / "out"
        for p in (build_dir / "debug" / "build").glob(f"{package_name}-*")
        if (p / "out").exists()
    ]
    match possible_dirs:
        case []:
            raise FileNotFoundError(
                f"No build directory found for {package_name}, has the project been built at least once with --profile dev?"
            )
        case [dir]:
            return dir
        case _:
            raise FileNotFoundError(
                f"Ambiguous: multiple build directories found for {package_name}"
            )


def get_enum_variants(
    package_name: str, version: str, enum_name: bytes, generated_by_build_rs: bool
) -> list[bytes]:
    if generated_by_build_rs:
        package_src = find_build_rs_out(package_name)
    else:
        package_src = get_src_dir(package_name, version)
    enum_node = find_enum_in_src(package_src, enum_name)
    return parse_enum_variants(enum_node)


if __name__ == "__main__":
    print(
        get_enum_variants(
            "iso_country", "0.1.4", b"Country", generated_by_build_rs=False
        )
    )
