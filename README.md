## Setup

### Tauri

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
. "$HOME/.cargo/env"
cargo install tauri-cli

# optional
cargo install --locked cargo-outdated
```

### Python/trame

Python setup for building sidecar as single file.

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -U pip
pip install trame trame-vtk trame-vuetify pyinstaller

python -m trame.tools.www --output ./src-tauri/www

python -m PyInstaller \
    --clean --noconfirm \
    --distpath src-tauri \
    --name server \
    --hidden-import pkgutil \
    server.py

# Fix versions in Cargo.toml for tauri and tauri-build
# cd src-tauri
# cargo outdated

# Generate icons from image
cargo tauri icon ./app-icon.png

# Bundle app (--debug)
cargo tauri build
open ./src-tauri/target/release/bundle/macos/Cone.app
```
