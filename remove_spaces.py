from pathlib import Path

if __name__ == "__main__":
    for file in Path("spec").glob("**/*.xlsx"):
        if " " not in file.name:
            continue
        new_name = file.name.replace(" ", "_")
        file.rename(file.parent / new_name)
        print(f"Renamed {file} to {new_name}")
