## Setup

Python setup for building sidecar as single file.

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -U pip
pip install trame pyinstaller

export SIDECAR_SUFFIX=`rustc -Vv | grep host | cut -f2 -d' '`

python -m PyInstaller \
    --clean --noconfirm \
    --distpath src-tauri \
    --name bin \
    --hidden-import pkgutil \
    --collect-data trame_client \
    --collect-data trame_components \
    --collect-data trame_vuetify \
    --collect-data trame_vtk \
    server.py

mv src-tauri/bin/server src-tauri/bin/server-${SIDECAR_SUFFIX}
```
