# Skru
Skru is an open-source and web-based alternative to Usenti, a bitmap editor for paletted images last edited in 2007 (👴🏻) and only compiled for Windows (🤮). This repository is specific to the Skru frontend. For the backend, see [Skrudriver](https://github.com/skrusenti/skrudriver).

This project is still under development and has no releases. Track progress in the [roadmap](https://github.com/orgs/skrusenti/projects/1).

## User-facing Features
- Quick Figma-like UI to edit sprites, bitmaps, and palettes.
- Support for GBA Mode 3 and 4 bitmaps and Mode 0 sprites and tiles.
- Cross-session and cross-user sharing of bitmaps and palettes.
- Export to a paletted C array without download, just copy to clipboard.
- Support for export to C code as source (`.c`) and/or header (`.h`)
- Support for export to `.png` and `.jpeg` with parameterized scaling.
## Technical Features
- Server-side rendering of static elements like toolbars and popups.
- GPU-accelerated rendering of the bitmap canvas.
- Custom parsing and transpiling of bitmaps to C arrays.
## Contributing
To contribute, see [the wiki](https://github.com/skrusenti/skru/wiki).
