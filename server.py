from trame.app import get_server
from trame.ui.vuetify import SinglePageLayout
from trame.widgets import vuetify, vtk

server = get_server()
state, ctrl = server.state, server.controller

with SinglePageLayout(server) as layout:
    with layout.content:
        with vuetify.VContainer(fluid=True, classes="pa-0 fill-height"):
            with vtk.VtkView(ref="view"):
                with vtk.VtkGeometryRepresentation():
                    vtk.VtkAlgorithm(vtk_class="vtkConeSource", state=("{ resolution }",))

    with layout.toolbar:
        vuetify.VSpacer()
        vuetify.VSlider(
            v_model=("resolution", 6),
            min=3, max=60, step=1,
            hide_details=True, dense=True, 
            style="max-width: 300px;",
        )
        with vuetify.VBtn(icon=True, click="$refs.view.resetCamera()"):
            vuetify.VIcon("mdi-crop-free")

if __name__ == "__main__":
    server.start()
