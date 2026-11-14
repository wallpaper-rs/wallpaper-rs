use super::w;
#[rustfmt::skip]
pub const ABOUT: [(&str, &str); 81] = [
    (
        "set",
        w(
            "Set the value at a JSON Pointer, creating the objects on the way; a value of the wrong kind is refused, one that breaks a rule is kept and the problems it adds are listed",
        ),
    ),
    (
        "add",
        w(
            "Insert a value into a list or an object at a JSON Pointer; the problems it adds are listed as for set",
        ),
    ),
    ("remove", w("Remove the value at a JSON Pointer")),
    ("move", w("Move the value at one JSON Pointer to another")),
    ("add_layer", w("Add a layer of a kind, optionally from a source, into a group or at a position")),
    ("remove_layer", w("Remove a layer by id")),
    ("duplicate_layer", w("Copy a layer, or make a template of it")),
    ("move_layer", w("Move a layer to a position, optionally into a group")),
    ("group", w("Put layers into a new group")),
    ("ungroup", w("Dissolve a group, keeping its layers")),
    ("hide", w("Show or hide a layer")),
    (
        "lock",
        w(
            "Mark a layer locked or unlocked; only the canvas handles of the app honour the mark, while actions, the inspector and wprs-ctl still change a locked layer",
        ),
    ),
    ("solo", w("Show only one layer, or all of them when no id is given")),
    ("keyframe", w("Add, change or remove a keyframe of an animated value")),
    ("import", w("Import a picture as an asset, optionally as a linear image or an atlas")),
    ("import_effect", w("Import an effect from a descriptor and its shader")),
    ("remove_effect", w("Remove an imported effect")),
    ("import_script", w("Copy the script module in file into the scene, or make one from a template, which needs no file")),
    ("import_assets", w("Copy assets from another package into the scene")),
    ("asset", w("Change how an asset is stored: texture compression, cover, placeholder, cube faces")),
    ("rgb", w("Turn the lighting of a layer on or off, or set its zone size")),
    ("translate", w("Set, add or remove the text of a manifest field or of a text layer in one language")),
    ("rename", w("Rename a layer, timeline, asset or particle system and what refers to it")),
    ("paste", w("Paste layers copied with the copy query, into a group or at a position")),
    ("thumbnail", w("Set the thumbnail of the package, or its animated preview")),
    (
        "shot",
        w(
            "Draw the scene: keep a frame as the thumbnail, record a short gif as the animated preview, or measure how much it flashes",
        ),
    ),
    ("upgrade", w("Move a first-version manifest to the current format")),
    ("add_property", w("Add a user property to the manifest")),
    ("move_property", w("Move a user property to a position or group")),
    ("rename_property", w("Rename a user property and what refers to it")),
    ("remove_property", w("Remove a user property")),
    ("condition", w("Add, change or remove a condition that shows or hides parts of the scene")),
    ("effect_rule", w("Set the scheduling rules of an effect parameter")),
    ("add_preset", w("Add a preset of the user properties")),
    ("paint_mask", w("Paint a stroke into the mask picture of an effect parameter")),
    ("comb_flow", w("Comb a flow map of an effect parameter so that it points along the stroke")),
    ("mesh_from_picture", w("Make a mesh from the outline of the picture of a layer")),
    ("bone", w("Add, move, rename, parent or remove the bones of a mesh")),
    ("mesh_depth", w("Paint the depth of a mesh")),
    ("weights", w("Paint the weights of the bones on a mesh")),
    ("auto_weights", w("Compute the weights of the bones on a mesh")),
    ("clip", w("Create, change or remove an animation clip of a rig")),
    ("key", w("Set or remove a key of a bone in a clip")),
    (
        "edit_keys",
        w(
            "Edit keys picked in the curve editor: move, scale, lift or set a part of them, bend or straighten handles of every part or of one part, link the parts of the handles, remove, or paste copied keys",
        ),
    ),
    ("clip_event", w("Add, move, rename or remove a named event of a clip, which fires when the clip passes it")),
    (
        "shape_sculpt",
        w(
            "Sculpt a blend shape of a mesh with a brush that pushes, pulls, smooths or erases, making the shape when it is new",
        ),
    ),
    ("expression", w("Add, change or remove a facial expression of a rig")),
    ("ik", w("Add, change or remove an inverse kinematics chain")),
    ("physics", w("Make bones follow physics: stiffness, damping and mode")),
    ("collider", w("Add, change or remove a capsule or sphere that keeps the swinging bones of a model or a mesh out")),
    ("bone_rule", w("Add, change or remove a rule that drives a bone from another")),
    ("wind", w("Set the wind that moves the physical bones and particles")),
    ("attach", w("Attach a layer to a bone or to a point of another layer")),
    ("point", w("Add, move, rename or remove a named point of a layer")),
    ("parts", w("Order, hide or show the parts of a layered picture")),
    (
        "particles",
        w(
            "Edit a particle system: starting kits, shape, bursts, forces, colliders, strands, lighting and settings",
        ),
    ),
    (
        "light",
        w(
            "Add, change, mute or remove the lights of the scene; add places a light at a position in its group, set moves it to a point of the canvas",
        ),
    ),
    ("lighting", w("Set the lighting model of the scene")),
    ("normals", w("Make a normal map for a layer")),
    ("paint_map", w("Paint a roughness or emissive map")),
    ("ramp", w("Make a gradient picture or color table from color stops and use it in an effect parameter")),
    ("depth_map", w("Make a grey depth map of a picture with the depth model and use it in an effect parameter")),
    ("ambient", w("Set the ambient light")),
    (
        "scene3d",
        w(
            "Edit a 3D layer: models, materials, sculpted shapes, camera, view, tones, shadows, camera paths and 3D handle drags",
        ),
    ),
    ("animate", w("Edit the animation of a mesh or model layer: clips, presets and morphs")),
    (
        "effects",
        w(
            "Add, set, remove, move or mask the effects of a layer, and drag their handles and masks on the canvas",
        ),
    ),
    (
        "layer",
        w(
            "Edit the transform, blend or instancing of a layer, or start from a ready-made clock or date text and a movement that spins or drifts forever",
        ),
    ),
    (
        "manifest",
        w(
            "Edit the manifest: origin, license, tags, authors, credits, texts by key, locale files, display, playback, web, network, inputs, and the title and thumbnail of each part a resource package offers",
        ),
    ),
    ("property", w("Edit a user property of the manifest")),
    ("canvas", w("Edit the canvas: bloom and parallax")),
    ("timeline", w("Add, change or remove a timeline and its events")),
    ("script", w("Attach, remove, move or declare a script, write its code, or set its typed settings")),
    ("template", w("Make a template from layers, or rename or remove one")),
    ("pool", w("Add, change or remove a mesh pool: the shapes that scripts build and shapes layers draw")),
    (
        "uses",
        w(
            "Declare the parts of the scripting interface a script uses, or take undeclared ones from its source",
        ),
    ),
    ("film", w("Trim a video layer to a part of the video, set its speed, or size its box from the video")),
    (
        "sprites",
        w(
            "Cut a picture into frames with a grid or drawn rectangles, or change, add, remove and reorder them",
        ),
    ),
    (
        "shader",
        w(
            "Write a package effect or material shader: make one from a template, write its WGSL or descriptor, or edit its parameters, passes and fields",
        ),
    ),
    ("transition", w("Make, change or remove the transition the scene comes in or leaves with: its effect, duration, curve and values")),
    ("undo", w("Undo the last change")),
    ("redo", w("Redo the last undone change")),
];
