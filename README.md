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

python -m PyInstaller \
    --clean --noconfirm \
    --distpath src-tauri \
    --name server \
    --hidden-import pkgutil \
    --collect-data trame_client \
    --collect-data trame_vuetify \
    --collect-data trame_vtk \
    server.py

cd src-tauri

# Fix versions in Cargo.toml for tauri and tauri-build
# cargo outdated

cargo tauri build
open target/release/bundle/macos/Cone.app
```
