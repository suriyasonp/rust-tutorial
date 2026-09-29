"""Prepare a temporary MkDocs tree from canonical repository Markdown.

Run `python tools/build_site_source.py` from anywhere. MkDocs then reads
site-src. Do not edit the generated tree; edit README, docs, or labs instead.
"""

from pathlib import Path
import shutil

ROOT = Path(__file__).resolve().parents[1]
DEST = ROOT / "site-src"

if DEST.exists():
    shutil.rmtree(DEST)
DEST.mkdir()


def copy_markdown(source: Path, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    content = source.read_text(encoding="utf-8")
    # The repository uses README.md as its home. MkDocs uses index.md.
    # Rewrite navigation to the home page only in the generated site tree.
    content = content.replace("../README.md", "../index.md")
    destination.write_text(content, encoding="utf-8")


copy_markdown(ROOT / "README.md", DEST / "index.md")
for folder in ("docs", "labs"):
    for source in sorted((ROOT / folder).glob("*.md")):
        copy_markdown(source, DEST / folder / source.name)

copy_markdown(ROOT / "solutions" / "README.md", DEST / "solutions" / "README.md")

# Preserve the links from bonus labs to their runnable source snapshots.
for folder in ("sensor-validation", "log-summary"):
    source = ROOT / "solutions" / folder / "src" / "main.rs"
    destination = DEST / "solutions" / folder / "src" / "main.rs"
    destination.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(source, destination)

shutil.copyfile(ROOT / "LICENSE", DEST / "LICENSE")
(DEST / "stylesheets").mkdir()
shutil.copyfile(ROOT / "site-assets" / "course.css", DEST / "stylesheets" / "course.css")

print(f"Prepared site source at {DEST}")
