import os, sys
import bpy, bpy_types
import bmesh

from mathutils import *
from math import *

# "C:\BlenderVersions\stable\blender-4.0.0+stable.878f71061b8e\blender.exe"
# example:
# /Applications/Blender-3.3/Blender.app/Contents/MacOS/Blender --background srcart/fpz7d_assets.blend --python scripts/export_mesh.py

C = bpy.context
D = bpy.data

SCENE_NAME = "unnamed"

def process_mesh( meshObj ):

    # Reset Transform
    bpy.ops.object.select_all(action='DESELECT')
    print("Reset location on ", meshObj.name )
    meshObj.select_set(True)
    bpy.ops.object.location_clear(clear_delta=False)

    # Select mesh and export it 
    meshObj.select_set( True )
    filename = os.path.join( "assets", meshObj.name + ".gltf" )
    print( f"Will export {filename}")

    bpy.ops.export_scene.gltf(
        filepath=filename,
        use_selection = True,
        export_apply =True,
    )

def process_objs( ):
    for obj in D.objects:
        if (obj.type == 'MESH'):

            # Ignore "Build" objects, these are intermediates
            # TODO: check that these are set not visible so they don't get exported
            if not obj.name.startswith( "BUILD_"):
                process_mesh( obj  )

def main():
    global SCENE_NAME
    for a in range(len(sys.argv)):
        if (sys.argv[a] == '--'):
            break

        # Get the original scene name
        # TODO make better, right now this assumes that it will follow -b
        if (sys.argv[a] == '-b') or (sys.argv[a] == '--background'):
            sceneName = sys.argv[a + 1]

    sceneName = os.path.basename(sceneName)
    sceneName = os.path.splitext(sceneName)[0]

    SCENE_NAME = sceneName

    print("Scene is ", sceneName)    
    process_objs()

if __name__ == '__main__':
    main()    