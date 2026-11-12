# Scene editor

The scene editor of the app and `wprs-ctl scene` edit a scene folder: `manifest.json`, the
scene file it names as `entry` (`scene.json`) and the assets, by convention under `assets/`. There
is no editor format: the folder is what `wprs-ctl pack build` packs and what the engine reads
(`scene.md`). Both share one implementation, `crates/wprs-editor`, so every action of the app is
also a command, with the same arguments and the same undo history.

## Document

The editor works on one JSON document, `{"manifest": …, "scene": …}`. Every field is addressed by
a JSON Pointer (RFC 6901) into it, such as `/scene/layers/0/opacity`,
`/scene/particles/flakes/init/size` or `/manifest/properties/tint/default`; `~1` stands for `/`
and `~0` for `~` inside a key. The editor keeps its own marks in `scene.ext.editor`, which the
engine ignores: `locked` lists the layers that the canvas does not move, `solo` names the only
layer previews show (its groups and children included; sound layers other than it are muted) and
`hidden` keeps, by layer id, the value of `visible` or `play` that hiding replaced when it was a
binding, keyframes or anything other than `true`.

## Operations

Every change is a list of operations in the style of JSON Patch (RFC 6902):

| Operation | Effect | Inverse |
|-----------|--------|---------|
| `{"op": "add", "path", "value"}` | inserts into a list (an index from 0 to its length, or `-` for the end) or sets a key of an object | `remove`, or `replace` with the old value when the key existed |
| `{"op": "remove", "path"}` | removes an item or a key | `add` with the removed value |
| `{"op": "replace", "path", "value"}` | replaces an existing value | `replace` with the old value |
| `{"op": "move", "from", "path"}` | removes and adds again; a value cannot move into itself | `move` back |

An `add` at index `-` or `0` of a list that does not exist yet creates the list, as long as the
object that holds it exists; undo removes the list again.

A list of operations applies as a whole: when one fails, the ones before it are undone and the
document stays as it was. Applying a list returns its inverses in reverse order, which undo
applies. List indexes are canonical (`0`, `1`…, never `01`).

## Actions

Actions turn into operations against the current document. The app sends them as
`{"<action>": {<arguments>}}`; `wprs-ctl scene <command> <folder> …` takes the same arguments,
saves at once and prints the ids of what it added. `wprs-ctl scene show DIR [POINTER]` prints the
document or the value at a pointer as JSON, and `wprs-ctl scene history DIR` lists the steps that
undo and redo walk through, the undone ones marked.

| Action | Arguments | Command |
|--------|-----------|---------|
| `set` | `pointer`, `value`; creates the objects on the way | `set DIR POINTER VALUE` |
| `add`, `remove`, `move` | `pointer` and `value`, `pointer`, `from` and `to` | `add`, `remove`, `move` |
| `add_layer` | `kind`, optional `id`, `name` (the name of the layer in the author's language), `source` (the asset of an image, video, sound, mesh or model layer, the system of a particles layer), `file` (a picture, video, sound or `.glb` file of the kind of the layer, imported as its asset in the same step; a `.glb` must read as a mesh for a `mesh` layer and as a model for a `model` layer, and the asset takes the name of the file, or `<id>_file` when that is the id of the layer), `into` (a group), `at` (0 is the bottom); the Add layer menu adds an image, video, sound, mesh or model layer from the only asset of its kind, or asks which one when the scene has several, and offers *Layer from a file* with Picture…, Video…, Sound… and Mesh from a .glb file…, each picking a file of that kind (and *3D model from a file…* for models) | `add-layer DIR KIND [--name N] [--source ASSET \| --file FILE]` |
| `rename` | `kind` (`layer`, `timeline`, `asset` or `system`), `id`, `to` (1 to 32 small letters, digits or `_`, free among the layers, assets and systems, or among the timelines); the parents, attachments, masks, effect parameters, bindings, ik targets, `click:`, `enter:` and `leave:` triggers, editor marks (`locked`, `solo`, `hidden`) and particle sources that name a layer, the values that follow a timeline, the fields and property defaults that name an asset (found through the `x-ref` markers of the schema, so that a text or another free string that happens to equal the old id stays as it is; a paste that gives an asset or a system a free id rewrites its references the same way) and the layers and systems that use a system follow the new id; scripts are not rewritten, and `wprs-ctl` and the app list each line of a script that names the old id in quotes as `path:line`. The layer list offers *Rename the layer*, the assets panel *Rename…* on each asset, and the timeline menu and the particle systems their own | `rename DIR KIND ID TO` |
| `paste` | `clip` (what the editor query `{"copy": {"ids"}}` answers: `from`, the folder it was copied from, `layers`, and the `assets`, `particles` and `effects` they use, systems and assets used by those included), optional `into` (a group) and `at`; adds the layers with free ids (the same id while it is free, else `<id>_2`…), adds the assets, systems and effects the scene lacks under free ids, copying their files from `from` (a file of the same path and other content gets `<name>_2`…; when the paste is refused, the files and folders it copied are removed again), rewrites what the pasted parts name to the new ids and drops references to layers the scene does not have. *Copy the layers* in the layer list copies the selected layer and the ones picked with Ctrl into the clipboard of the app and, as text that starts with `wprs-layers:` followed by the clip (with `from`, the folder of the package, so that its files are found), into the system clipboard; *Paste the layers* pastes what the system clipboard holds when it is such a payload, so that copying in one window and pasting in another works, and otherwise the last clip of the app, ignoring any other text | `copy DIR ID...` prints a clip, `paste DIR CLIP [--into G] [--at N]` (CLIP as JSON or a file) |
| `remove_layer`, `duplicate_layer` | `id`; removing a layer also drops what named it from outside it (the `parent` and `inherit` of the layers that follow it, which keep their place on the canvas when their position, rotation, scale and skew are plain values or keyframes (a keyframed value has every key moved by the same correction, an offset for position, rotation and skew and a factor for scale, so the first frame stays where it was), attachments, masks, effect parameters, bindings, ik targets and triggers), and is refused while a particle system takes particles from it; a follower whose place is bound to a prop, a map, audio, the cursor, a clock, a date or a script cannot be kept in place and is left as it is; the editor query `{"unkept": {"id"}}` answers `layers`, the ids of the followers that removing the layer would leave like that, and the Remove button of the layer list asks it first and names them in the notice after the removal; a copy gets new ids for itself and its children, and what the copied layers name among themselves (parents, masks…) points at the new copies; with `template` (`true`) the layer, a picture, text, shape, solid, spectrum, mesh or group of them, is copied as it is into the script `templates` instead, under the first free name of `<id>_template`, `<id>_template_2`… (the layers bar offers it as *Make a template*) | `remove-layer`, `duplicate-layer [--template]` |
| `move_layer` | `id`, `to`, optional `into`; without `into` the layer goes to the top level | `move-layer DIR ID TO` |
| `group`, `ungroup` | `ids` of layers with the same parent, each taken once, and an optional group `id`; the `id` of a group, refused while the group has an opacity, transform, anchor, blend, mask, effects, parallax, visibility or clip of its own that ungrouping would lose | `group`, `ungroup` |
| `hide` | `id`, `hidden`; sets `visible`, or `play` on a sound layer, to `false`, keeping what it held under `hidden`, and puts it back when shown | `hide DIR ID --hidden BOOL` |
| `lock`, `solo` | `id` and `locked`; `id`, or nothing to end the solo | `lock`, `solo` |
| `keyframe` | `pointer`, `time`, optional `value` (the value of the keyframe at that time, or the current value, when omitted), `ease`, `timeline` (an existing timeline; refused until the `timeline` action adds it), `remove`, `from` (moves the keyframe at that time to `time`, keeping its value, easing and handles unless given, as one step; refused when another keyframe is at `time`), `base` (a number, list of numbers or color like the keyframes, which are then added to it: relative values; each number from -1000000 to 1000000; removing the last keyframe leaves the base plus its value, added as the scene adds them: a number to each part of a list and to the red, green and blue of a color (a hex color or a list of 4 parts), a hex color to a color list of 3 or 4 parts, with the alpha kept within 0 to 1, a hex color when neither side is a list; a base and value that cannot be added refuse the removal, and `absolute` leaves the value alone), `absolute` (drops `base`), `in_handle` and `out_handle` (`[seconds, value]` of the Bézier handle before or after the keyframe of a number, a list of numbers or a color (for the last two the value is a share of the way to the neighbouring keyframe, as the scene reads it, except between two keyframes written as one number, which bend in value units as numbers do; texts and switches take none), from 0 s up to the previous or next keyframe and from -1000000 to 1000000 in value, so refused on the first key before it and the last key after it, and a move, insert or removal that would leave a handle past its neighbour is refused too, as is any edit that would leave the scene invalid; they replace its easing, an `ease` replaces them, and a handle that is left out keeps its value), `straighten` (drops the out handle or the easing of the keyframe at `time` and the in handle of the next keyframe, keeps their other handles and is one step; refused together with any other field but `pointer` and `time`); the timeline samples the handles as the scene does, and its Curve and base of the keyframe dialog drags both handles of the curve to the next keyframe live, the preview following, as one step for each drag (the press holds the gesture open, see "History"), plotting the share of the way for lists and colors, straightens it and sets or drops the base | `keyframe DIR POINTER TIME [VALUE] [--ease E] [--timeline T] [--base JSON \| --absolute] [--in-handle DT,DV] [--out-handle DT,DV]`, `--remove`, `--from T` or `--straighten` |
| `set` on a `trigger` or `play_on` | the trigger of a timeline or of a sound layer; the picker of the inspector offers `start`, `beat`, the buttons and settings of the package (`prop:`) and, in scenes, the named events of timelines and clips (`event:`), `click:`, `enter:` and `leave:` of each `solid` layer or layer with `hit` bones, and `media:play`, `media:pause`, `media:track` and `media:cover`, each with its name | `trigger DIR TARGET [TRIGGER]` (a timeline or sound layer id; prints the trigger without one), `--none` to take it away, `--list` for the offered ones; an unknown `media:` name is refused |
| `import` | `file`, optional `id`, `linear` (the picture holds data such as a normal map or a mask) and `atlas` (`[rows, columns]`, 1 to 256 each, of a sprite sheet), both refused for a file that is not a picture; copies the file into `assets/` and declares the asset; a refused import, or a refused `add_layer` with a `file`, takes back the copy it made | `import DIR FILE [--id ID] [--linear] [--atlas ROWS,COLUMNS]` |
| `import_effect` | `descriptor`, optional `wgsl` (the `.wgsl` file beside the descriptor when omitted); checks the effect as a package reader does, copies both files to `effects/<id>.json` and `effects/<id>.wgsl` and declares or replaces `effects.<id>` of the scene | `import-effect DIR DESCRIPTOR [--wgsl FILE]` |
| `remove_effect` | `effect` (an effect of the package); refused while a layer's effect stack or a model's material still names it | `remove-effect DIR EFFECT` |
| `paint_mask` | `pointer` (a parameter of an effect), optional `channel` (0 red to 3 alpha; every color when omitted), `radius` (a fraction of the shorter side), optional `hardness` (0.5) and `value` (1; 0 erases), `points` (fractions of the layer); paints the stroke into the picture the parameter names, or into a new one of 512 × 512 filled with the `paint` of the parameter (black without one), saves the result as a new `linear` picture `masks/<id>-<n>.png` (`.ktx2` with the codec and quality of the mask it replaces when that one is KTX2, or of the picture of the layer when the mask is new); a process that wrote a KTX2 mask or map keeps an exact copy of up to eight of them (of at most 2048 × 2048 pixels) while their files stay unchanged, so the next stroke starts from that copy and the loss of the compression never adds up stroke after stroke and points the asset and the parameter at it, so that undo returns to the previous picture. The inspector shows a Paint button, the channel (for `mask` parameters with 3 or 4 `channels`), Erase, the brush size, its hardness (0 to 1, 0.5 at the start) and the value it paints (0 to 1, 1 at the start; Erase paints 0) next to every `mask` parameter and every `texture` parameter whose descriptor names a `tool` (`brush` paints grey, `direction` combs a flow map through `comb_flow`: the value becomes its strength and the channel is not offered), strokes on the layer in the preview are painted, and while the brush is on the preview tints the channel being painted (or the brightness when every color is painted) over the box of the layer, through the query `mask_picture` and as the strokes change the picture | `paint-mask DIR POINTER X,Y;X,Y;... [--channel r\|g\|b\|a] [--radius R] [--hardness H] [--value V]` |
| `comb_flow` | the fields of `paint_mask`, on a `texture` parameter whose descriptor names the `direction` tool: red and green of every texel the stroke covers turn towards the direction of the stroke there, by `value` (0 paints it still), starting from a still picture (`paint` of the parameter, 0.5 red and green) | `scene act DIR comb_flow JSON` |
| `mesh_from_picture` | `image` (a picture asset), optional `outline` (follow the opaque part instead of a full grid), `cells` (2 to 64 along the longer side, 16 by default) and `id`; cuts the picture into triangles (cells with any opaque pixel, or the outline of the largest opaque region with points inside it), saves them as a binary glTF `meshes/<id>-<n>.glb` of one part named `body` and adds a `mesh` layer of the size of the picture that shows it. The Add layer menu offers it as Mesh from a picture, next to Mesh for an imported `.glb` | `mesh-from-picture DIR IMAGE [--outline] [--cells N] [--id ID]` |
| `effect_rule` | `effect` (an effect of the package), `param`, and any of `uses` (`time`, `cursor`, `audio` or `clicks`), `waits_for` and `yields_to` (another parameter), an empty value removing it; rewrites the descriptor of the effect after checking it as an import does, and the inspector of every effect use shows these rules, which it changes with pickers on effects of the package | `effect-rule DIR EFFECT [PARAM] [--uses U] [--waits-for P] [--yields-to P]`; without `PARAM` it prints the rules of any effect |
| `bone` | `layer` (a mesh layer, see "Mesh layers"), `name` (1 to 64 characters without `/` or `#`, not used by another node of the file), optional `parent` (an empty one hangs it from no other bone), `at` (where its head goes, in fractions of the layer; the middle for a new bone), `to` (a new name; the clips, animation layers, `ik` chains and targets, `physics`, `rules`, `hit` and the `attach` of other layers that name the bone follow), `remove` (its children then hang from its parent, which takes its weights); adds, moves, rehangs, renames or removes the bone, at most 128, keeping the place of its children, and saves the mesh as a new file, so that undo returns to the previous one. In the same step, removing a bone drops what names it: its keys in clips (a clip left without keys and without an animation of the file keeps one at rest on the parent), its place in the bones of animation layers (a layer that only moved that bone goes), the `ik` chains that reach for it, its limits and its place in chains (a chain left with fewer than 2 bones or no longer hanging goes), its `physics`, the `rules` that name it, its place in `hit` and the `attach` of the layers that follow it; rehanging a bone, also by dropping it in the tree, drops the chains it unlinks and the `rules` it would close into a loop | `bone DIR LAYER NAME [--at X,Y] [--parent P \| --root] [--rename NEW] [--remove]`, and `bones DIR LAYER` prints the bones, the other nodes (`points`: attachment points and the nodes of parts), parts, blend shapes and their defaults, expressions, animations and clips, and whether the weights are automatic (with the place of each bone and point and `editable: false` on a model layer, whose bones come from its file and are posed by its clips; every mesh a scene draws, up to the 64 primitives of a mesh layer, is editable; a mesh drawn by several nodes gets a mesh of its own for each of them, named `<mesh>_<node>`, on the first edit, and every primitive is paired with its part by the node and primitive it comes from), and `part-outlines DIR LAYER` prints the outline of every part, as the editor draws it, on any mesh it shows |
| `mesh_depth` | `layer` (a mesh layer, see "Mesh layers"), `points` (a stroke of 1 to 1 024 points, in fractions of the layer), `radius` (above 0 and up to 1 of the longer side) and `by` (−1 000 to 1 000 pixels towards the viewer at the middle of the brush, fading to nothing at its edge; below zero lowers); raises or lowers the depth of the vertices along the stroke, keeps their bones, weights and blend shapes and saves the mesh as a new file, so that undo returns to the previous one. The inspector of the `mesh` field paints it with Paint depth on the preview, the pixels and the brush size; the depth moves vertices with the perspective of `depth` and decides which vertices cover which | `mesh-depth DIR LAYER X,Y;X,Y;... --by PX [--radius R]` |
| `weights` | `layer`, `bone` (the bone painted), `points` (a stroke of 1 to 1 024 points, in fractions of the layer), `mode` (`add`, `subtract`, `smooth` towards the mean of the neighbouring vertices, or `normalise`, which drops the weights under the strength), `by` (the strength at the middle of the brush, 0 to 1), optional `radius` (0.1 of the longer side), `falloff` (0.5: how much of the brush fades towards its edge, 0 hard to 1 soft) and `lock` (bones whose weights the stroke keeps; the painted bone may not be one); each vertex keeps at most 4 bones whose weights add up to 1, the others making room or taking what the painted bone gives up (its parent when no other bone holds the vertex); the weights of the mesh then stay as painted when bones change, and the mesh is saved as a new file | `weights DIR LAYER BONE X,Y;X,Y;... --mode M --by V [--radius R] [--falloff F] [--lock BONE]...`, and `vertex-weights DIR LAYER` prints each vertex with its place, part and weights |
| `auto_weights` | `layer`; binds every vertex to its nearest bones again (the weights of a vertex fall off with its distance to each bone, from its head to the head of its first child) and makes the weights follow the bones until they are painted | `auto-weights DIR LAYER` |
| `clip` | `layer` (a mesh or a model), `id`, optional `add` (a new clip, of 2 s unless `length` is given; on a mesh without animations of its own it starts with a key at 0 of the rest of its first bone, or of the order of its first part, so that it is valid), `remove` (also removes the animation layers that play it), `to` (a new id; animation layers follow), `length` (seconds, sets `to`), `loop`, `animation` (an animation of the file the clip plays whole), `from` (seconds into the animation where the clip starts, keeping its length within the animation), `play` (with `add`: also adds an animation layer that plays the new clip, at most 16; the clip bar offers it as Add a clip and play it in a new animation layer) | `clip DIR LAYER ID [--add [--play] \| --remove] [--rename NEW] [--length S] [--loop L] [--animation NAME] [--from S]` |
| `key` | `layer`, `clip`, `time` (seconds from the start of the clip, its `from`, to the millisecond, as the timeline shows them) and one of `bone` with optional `rotation`, `position` (`[x, y]`), `scale` (`[x, y]`), `angles` (`[x, y, z]` degrees) and `shift` (`[x, y, z]` units), `shape` with optional `weight`, or `order` (a part) with optional `value`; optional `ease` (any easing of the scene format, such as `step`, `hold` or `in_out_sine`, or four numbers `[x1, y1, x2, y2]` for a cubic Bézier; `linear` writes nothing: how the key moves on to the next one), `in_handle` and `out_handle` (`[dt, dv]` Bézier handles into and out of the key, as in `keyframe`: seconds up to the neighbouring key and, between numbers, how much the value differs there, between lists a share of the way; they bend one channel at a time, are refused together with `ease` and keep travelling with the key when it moves or is copied; a key that is given an `ease` loses its handles), `remove`, `from` (moves the key at that time to `time`, or copies it with `copy`); writes the keys of the clip (`bones.<name>.rotation`, `position`, `scale`, `angles` and `shift`, `shapes.<name>`, `order.<part>`); on a model a bone keyed without values keys `angles` and `shift`; `from` is also in seconds from the start of the clip; a bone keyed without values keys the three of them with the pose the clip gives it at that time, the motion of the file included, and when values are given only the named channels are keyed, removed or moved, so a turn leaves the motion of the file in the others | `key DIR LAYER CLIP T (--bone B [--rotation D] [--position X,Y] [--scale S\|X,Y] [--angles X,Y,Z] [--shift X,Y,Z] \| --shape S [--weight W] \| --order P [--value N]) [--ease E] [--in-handle DT,DV] [--out-handle DT,DV] [--remove] [--from T0 [--copy]]`, and `keys DIR LAYER CLIP` prints its keys row by row, in seconds from the start of the clip, with the length and loop of the clip and the blend shapes and parts of the mesh |
| `expression` | `layer`, `name` (1 to 64 characters), optional `shapes` (`{ "<blend shape>": weight }`, −1 to 1), `to` (a new name; the `shapes` of the layer that set it follow), `remove` (also from `shapes`); sets, renames or removes an expression, at most 64 | `expression DIR NAME [--shape S=W]... [--rename NEW] [--remove]` |
| `ik` | `layer`, and `root` with `tip` to add a chain from a bone down to one that hangs below it (2 to 8 bones, at most 16 chains; it starts reaching for the point where its end rests, in pixels of the box from its top left, so nothing moves), or `index` (0 first) to change a chain, with `remove` to remove it; optional `root` with `tip` (a new chain of bones), `target` (`"cursor"`, `"bone:<node>"`, `"layer:<id>"` or a point `[x, y]`), `limits` (`{ "<bone of the chain>": [min, max] }`, −180 to 180 degrees from its rest), `unlimit` (bones whose limits go), `weight` (0 to 1) and `bend` | `ik DIR LAYER add ROOT TIP`, `ik DIR LAYER set N [--root R --tip T]` and `ik DIR LAYER remove N`, with `[--target cursor\|bone:NAME\|layer:ID\|X,Y] [--limit BONE MIN MAX]... [--unlimit BONE]... [--bend B] [--weight W]` |
| `physics` | `layer`, `bones` (nodes of the file), optional `mode`, `stiffness`, `damping`, `inertia`, `wind` (0 to 1), `gravity` (−10 to 10), `rotation` (`[min, max]` degrees within 180) and `translation` (`[min, max]` pixels), `free` (`rotation` or `translation`: the bounds that go); adds a body for each bone that has none (a `spring`, or a `rope` for a new rope) or changes the given settings of the one it has, at most 64; `remove` removes their bodies; `rope` (2 to 9 points in fractions of the layer) with a single name in `bones` and an optional `parent` adds the bones `<name>_1`, `<name>_2`… along the points, each hanging from the one before and the first from `parent`, saves the mesh as a new file and makes the first one swing as a rope | `physics DIR LAYER add BONE... [--mode M] [--stiffness S] [--damping D] [--inertia I] [--gravity G] [--wind W] [--rotation MIN,MAX] [--translation MIN,MAX] [--free rotation\|translation]`, `physics DIR LAYER rope NAME X,Y;X,Y;... [--parent BONE] [...]` and `physics DIR LAYER remove BONE...` |
| `wind` | optional `angle` (degrees clockwise from the right, where it blows to), `strength` (0 to 10 000 pixels a second), `gusts` (0 to 1) and `off`; sets `canvas.wind` from the angle and strength, keeping the one not given from the wind there is (a bound or keyed wind becomes fixed), and `canvas.gusts`, or removes both | `wind DIR [--angle D] [--strength S] [--gusts G \| --noise G] [--off]` |
| `bone_rule` | `layer`, and `bone` with `parent` to add a rule that hands a bone over to another parent as its `weight` goes from 0 to 1, keeping its pose (at most 64), or `index` (0 first) to change a rule, with `remove` to remove it; optional `bone`, `parent` (a different bone of the mesh, refused while they are the same or the rule would close a loop) and `weight` (0 to 1, 1 by default) | `bone-rule DIR LAYER add BONE PARENT [--weight W]`, `bone-rule DIR LAYER set N [--bone B] [--parent P] [--weight W]` and `bone-rule DIR LAYER remove N` |
| `attach` | `layer`, optional `to` (a `mesh` or `model` layer; the one it follows already when omitted), `bone`, `point` (a node of the file, or `[x, y]` in fractions of the layer for a new point there: a node without a mesh named `<layer>_point`, marked `extras.point`, hanging from the bone that leads the nearest vertex, and the mesh is saved as a new file; a layer that already follows a point of its own on that mesh moves it there instead), `offset` (pixels), `nudge` (`[x, y]` pixels of the box of the mesh layer that the layer moves by, turned into `offset` along its bone or point at the rest pose and what it inherits), `inherit` (`position`, `rotation`, `scale`), `detach`; writes `attach`, keeping the offset and what it inherits while it follows the same layer, and refuses a layer that would follow itself, through other layers too. A point marked `extras.point` that the layer leaves (by `detach` or by following something else) goes with it when no other layer follows it and no chain reaches for it | `attach DIR LAYER [--to MESH_LAYER] [--bone NAME \| --point X,Y\|NAME] [--offset X,Y] [--nudge X,Y] [--inherit position,rotation,scale] [--detach]` |
| `point` | `layer`, `name` (as for bones), optional `at` (fractions of the layer; required for a new point), `parent` (a bone; an empty one hangs it from none), `to` (a new name; the `attach` of the layers that follow it and the chains that reach for it as `bone:<name>` follow), `remove` (also drops those `attach` and chains); adds, moves, rehangs, renames or removes an attachment point, a node without a mesh marked `extras.point` that a new point hangs from the bone that leads the mesh there, and saves the mesh as a new file | `point DIR LAYER NAME [--at X,Y] [--parent BONE \| --root] [--rename NEW] [--remove]` |
| `parts` | `layer`, optional `order` (`{ "<part>": n }`, −1000 to 1000, rounded), `hide` and `show` (parts; shown is the default, so the field goes), `clip` (`{ "<part>": "<part it only shows over>" }`, never itself, also through other parts), `unclip` and `depth` (`{ "scale" 0 to 10, "cull", "sort" }` turns depth on and changes those fields, `sort: true` drops the field as it is the default, `{ "off": true }` removes `depth`, not together with the others); writes `parts` and drops what is left empty | `parts DIR LAYER [--order PART N]... [--hide PART]... [--show PART]... [--clip PART TO]... [--unclip PART]... [--depth-scale N] [--cull on\|off] [--sort on\|off] [--flat]` |
| `particles` | one of `create` (`system`, optional `from`, a system to copy, `parent`, a system that starts it, and `on`, `static` when omitted: a system without a layer), `delete` (`system`; refused while a layer or another system names it), `settings` (`system`, optional `capacity`, `seed`, `prewarm`, `space`, `rate` and `emitter`, every emitter when omitted; a rate that follows keyframes or another source is refused unless `replace`; `lock`, the values a particles layer cannot change, `none` removing them), `emit` (`system`, `emitter`, `values` of `bursts` (a list of at most 8 `{ count, every, on, at }`), `follow`, `inherit`, `duration`, `delay`, `periodic` and `audio_gate` as the scene writes them, `null` or `false` removing one, and `clear`), `window` (`system`, `index` of a force, `window`: four shares of the life in order where it fades in, is full, fades out and is gone, none to remove it), `rename` (`system`, `to`; the layers and systems that use it follow; scripts that name it in quotes are not rewritten, and `wprs-ctl` and the app warn about each line of them as `file:line`), `scripts` (`from`, the old name, and `to`, a system that exists: rewrites every plain string literal in the scripts of the scene that is exactly the old name, in double, single or backtick quotes, to the new one; refused while a system still has the old name, and what is only part of a longer text stays; the files change at once and Undo does not bring them back), `start` (`system`, `ranges`: `life`, `speed`, `angle`, `size`, `rotation`, `spin`, `frame`, `color`, `alpha`, `velocity`, `radial`, `depth`, `depth_speed`, `hsv`, `palette` and `gradient` as the scene writes them, where one of `hsv`, `palette` and `gradient` replaces the other two, `depth` and `depth_speed` needing `perspective`, a `null` end or curve of a range keeps the current one and a curve of 1 is dropped, and `clear`: `alpha`, `velocity`, `radial`, `hsv`, `palette` or `gradient`), `child` (`system`, optional `index`, `child` as the scene writes it, `remove`), `over` (`system`, optional `layer`, `size`: `[time, scale]` keys and `color`: `[time, color]` stops, an empty list removes them), `scope` (`system`, `layer`, `own`: `true` gives the layer a render of its own, a copy of the one of the system, and `false` drops it so the layer follows the system again; refused for a layer that draws no particles of the system), `render` (`system`, optional `layer`, `stretch`, `frames` (`none` removes it), `fps`, `mix`, `brightness`, `cycles` (0 removes it) and `order` (`none` removes it)), `shape` (`system`, `emitter`, `shape`: the shape of an emitter as the scene writes it; the next index adds an emitter with a `rate` of 10), `point` (`system`, `point` 0 to 7, optional `at` in pixels of the layer, `follow` (`true` follows the pointer, `false` stops), `angle`, `remove` for the last one; earlier points that are missing start at `[0, 0]`), `remap` (`system`, `remap` to add, with `index` to replace one, or `index` and `remove`), `cutout` (`system`, `range` `[start, end, opacity]`, none to remove it), `orient` (`system`, `orient`, `angle` with `fixed`), `perspective` (`system`, `focal`, or `off` to remove the perspective and the `depth`/`depth_speed` ranges `start` set), `layer` (`system`, `emitter`, `id` of the layer it emits from, `threshold`, `color`, `inherit`, `jitter`, `refresh`), `inherit` (`system`, `child`, `inherit`, `follow`), `shader` (`system`, `shader`, none to remove it), `collider` (`system`, `collider` to add, with `index` to replace one, or `index` and `remove`), `response` (`system`, `index`, `response`: `bounce`, `slide`, `stop`, `stick` or `kill`, optional `bounce` and `friction`), `force` (`system`, `force` to add, with `index` to replace one, keeping its `window` unless the new force gives one, or `index` and `remove`; `keep_distance`, `keep_between` and `remap` operators among them), `boids` (`system`, `boids` with the fields to change, starting from a radius of 40, or `off`), `strand` (`system`, `strand`: `rope` or `trail`, `settings` with the fields to change, or `off`), `texture` (`system`, `texture`, none to remove it), `turbulent` (`system`, `turbulent` with the fields of `init.turbulent` to change, `quiet` to drop its sound band, or `off`), `offset` (`system`, `offset` with the fields of `init.offset_noise` to change, or `off`), `pace` (`system`, optional `emitter`, `max_per_step`, none to remove the limit; every emitter without `emitter`), `lit` (`system`, `lit`, `normal`, a picture, an empty one removes it) and `refract` (`system`, `normal`, required while the system has none, `amount` 0 to 1, 0.3 by default, or `off`); `cutout`, `orient`, `shader`, `strand`, `texture`, `lit` and `refract` take a `layer` to change the render of that particles layer, which starts as a copy of the render of the system. Each edit is refused, changing nothing, when it leaves the scene with a problem it did not have | `particles DIR SYSTEM shape N JSON`, `control-point pN [--at X,Y] [--angle D] [--cursor] [--remove]`, `remap add JSON`, `remap set N JSON`, `remap remove N`, `cutout START,END,OPACITY\|none [--layer L]`, `orient screen\|velocity\|fixed\|none [--angle D] [--layer L]`, `perspective [--focal F] [--off]`, `emit-from-layer N LAYER [--threshold T] [--color] [--inherit] [--jitter J] [--refresh R]`, `inherit N color\|size\|alpha\|velocity... [--follow]`, `shader ID\|none [--layer L]`, `collider add JSON`, `collider set N JSON`, `collider remove N`, `collider response N bounce\|slide\|stop\|stick\|kill [--bounce B] [--friction F]`, `keep distance TARGET [--distance D] [--strength S] [--index N]`, `keep between FROM TO [--strength S] [--index N]`, `keep remove N`, `boids [--radius R] [--separation S] [--alignment A] [--cohesion C] [--max-speed V] [--off]`, `operator remap add JSON`, `operator remap set N JSON`, `operator remap remove N`, `rope [--subdivide N] [--width W] [--uv-scale U] [--uv-scroll V] [--smooth BOOL] [--layer L] [--off]`, `trail [--length N] [--width W] [--segments N] [--subdivide N] [--uv-scale U] [--uv-scroll V] [--textured BOOL] [--layer L] [--off]`, `texture ID\|none [--layer L]`, `turbulent [--forward D] [--scale S] [--noise-speed N] [--offset O] [--speed MIN,MAX] [--phase P] [--audio BAND] [--quiet] [--off]`, `offset [--distance D] [--scale S] [--sign X,Y] [--octaves N] [--noise-scale N] [--noise-speed N] [--off]`, `pace [--emitter N] [--max-per-step N]`, `lit [--normal N] [--off] [--layer L]`, `refract [--normal N] [--amount A] [--off] [--layer L]` (`--show` on either prints the two-sided preview, the displacement and what each quality level keeps), `create [--from S] [--parent S] [--on M]`, `delete`, `settings [--capacity N] [--seed N] [--prewarm S] [--space local\|world] [--rate R] [--emitter N] [--replace] [--lock V,V\|none]`, `start [--life\|--speed\|--angle\|--size\|--rotation\|--spin\|--frame\|--alpha\|--radial\|--depth\|--depth-speed MIN,MAX[,CURVE]] (an end a number, `prop:ID` or `_` to keep it; the curve kept when left out; `--depth` and `--depth-speed` need `perspective`) [--color C,C] [--velocity X,Y:X,Y] [--hsv MIN,MAX [--saturation MIN,MAX] [--value MIN,MAX] [--hue-steps N] \| --palette C,C... [--palette-noise H,S,V] \| --gradient T:C...] [--clear alpha\|velocity\|radial\|hsv\|palette\|gradient]`, `emitter N [--burst COUNT [--every S] [--on beat\|click]] (one burst that replaces the list; several go through `scene act`) [--periodic ON_MIN,ON_MAX:OFF_MIN,OFF_MAX] [--duration S] [--delay S] [--follow B] [--audio-gate B] [--inherit F] [--clear V]`, `window N A,B,C,D\|none`, `rename NEW`, `force gravity X,Y`, `force drag A`, `force turbulence --strength S --scale C [--speed V] [--time-scale T] [--mask MIN,MAX]`, `force oscillate position\|alpha\|size --freq MIN,MAX --scale MIN,MAX [--phase MIN,MAX]`, `force attractor TARGET --strength S [--radius R] [--falloff F]`, `force noise A`, `force cap S`, `force torque A`, `force spin-drag A`, `force vortex TARGET --strength S [--radius R] [--falloff F] [--inner I] [--outer O] [--ring R,W,P]`, `force orbit TARGET --strength S [--radius R] [--falloff F]`, `force slow-near TARGET --inner I --outer O --factor F` (each with `--index N` to replace one), `force remove N`, `child add SYSTEM --on M [--probability P] [--max N] [--offset X,Y] [--angle D] [--scale S] [--points-from]`, `child set N SYSTEM --on M …`, `child remove N`, `size-over-life T:S...\|none [--layer L]`, `color-over-life T:C...\|none [--layer L]`, `render [--stretch T[,MIN,MAX]] [--frames time\|life\|random\|none] [--fps F] [--mix B] [--brightness B] [--cycles N] [--order O\|none] [--layer L]`; `particles DIR SYSTEM handles [--layer L] [--time T] [--seed N] [--sound S]` prints what the canvas draws for the system (with the seed the app shows, the rays match its preview; `--sound` from 0 to 1 is how loud the band that moves the phase of a turbulent start is, as the slider of the editor sets it, and *Keep this seed* writes the seed of the preview to `clock.seed`) and `particles DIR SYSTEM budget [--time T] [--quality Q] [--layer L]` its budget |
| `light` | one of `add` (`light`: `point`, `spot`, `tube` or `directional`, optional `id`, `into` a group and `at`, its position in that group; a new light starts in the middle of the canvas with the defaults of its type: a point light of radius 400 at a height of 120, a spot light of radius 900 at 150 with an intensity of 2, a cone of `[25, 45]` and a tilt of 45°, a tube light of length 400 and radius 240 at 60, a directional light with an intensity of 0.8 and a tilt of 50°; `add_layer` with `kind: light` and the type as `source` does the same), `set` (`layer` and any of `light`, which drops `cone`, `length` and `texture` where the new type has none, `color`, `intensity` 0 to 100, `radius` 1 to 100 000, `z`, `cone` `[inner, outer]` for spot lights, `length` for tube lights, `tilt` −89 to 89, `at`, a point of the canvas the light moves to, and `aim`, a point of the canvas it turns to, both through its parents as the light is placed, with its anchor and the skew of its parents, at `time` (0 by default); `tilt`, `at` and `aim` write a key at `time` when that part of the transform has keys, or replace them with `replace`), `remove` (`layer`), `solo` (`layer`, or none to end it), `mute` (`layer`, `muted`), `volumetric` (`layer` of a point or spot light, optional `density` 0 to 1, `samples` 4 to 64 and `shafts` 0 to 1, starting from 0.2, 16 and 0, or `off`) and `projection` (`layer` of a spot light and one of `asset`, a picture or video of the scene, `from`, a layer that takes no light and reads none that does, or `off`); solo and mute are kept in `/scene/ext/editor/lights` and only change the preview; `set` also drops `volumetric` from tube and directional lights | `light DIR LAYER add [--type T] [--into G] [--at X,Y]`, `light DIR LAYER set [--type T] [--color C] [--intensity I] [--radius R] [--z Z] [--cone IN,OUT] [--length L] [--tilt D] [--at X,Y] [--aim X,Y]`, `light DIR LAYER remove`, `light DIR LAYER solo [--off]`, `light DIR LAYER mute [--off]`, `light DIR LAYER falloff [--time T]` (the points of the falloff curve), `light DIR LAYER volumetric [--density D] [--samples N] [--shafts S] [--off]` or `--show` (the glow against the distance, the size and samples each quality level keeps and on phones), `light DIR LAYER projection [--asset A \| --layer L] [--off]` or `--show` (the pictures, videos and layers it may project and where the picture lands on the canvas), and `lights DIR [--time T]` prints every light with its handles, its dirty box and the count against each device tier, or with `--cost [--quality Q]` what the lighting costs at each quality level |
| `lighting` | `layer` (an image, video, text, mesh, model or particles layer; a particles layer takes only `strength` and `emissive`), optional `normal`, `env`, `roughness_map` and `emissive_map` (pictures of the scene; an empty id removes one), `roughness`, `reflection` and `strength` (0 to 1), `emissive` (0 to 16) and `off` (removes `lighting`); the first change writes the block. A normal, roughness or glow map that is not `linear` becomes `linear` in the same step, or, when the picture is also drawn in colors, the field names a new asset `<id>_linear` of the same file that is; `particles … lit` and `refract` do the same with their normal maps | `lighting DIR LAYER [--normal ID] [--env ID] [--roughness-map ID] [--emissive-map ID] [--roughness R] [--reflection R] [--emissive E] [--strength S] [--off]` |
| `normals` | `asset` (a picture) or `layer` (an image layer, whose picture is used; a video layer, whose first frame is used; or a text layer, whose letters are drawn at the size of the layer with its font), optional `strength` (0 to 16, 2 by default), `blur` (0 to 32 pixels, 1 by default) and `id` (refused when it names an asset that is not a normal map made here); makes a normal map from the edges of the brightness of the picture (a Sobel filter after a Gaussian blur, the same as `wprs-depth` offers), saves it as `normals/<id>-<n>.png` (`.ktx2` with the codec and quality of the picture when that one is KTX2), declares it as a `linear` asset and, with `layer`, sets it as the `normal` of that layer | `normals DIR ASSET --generate [--strength S] [--blur B] [--layer L] [--id ID]`, or `--preview` to print the relief, the map and a lit sphere without keeping anything |
| `paint_map` | `layer`, `map` (`roughness` or `emissive`), `radius`, optional `hardness` and `value` (the gray, 0 to 1) and `points` (fractions of the layer); paints the stroke into the map the layer has when it is a painted map (a `linear` picture under `maps/`) that nothing else uses, or else into a new map that starts as a copy of the one it had, or the size of its picture (at most 512 on the longer side), white for roughness and black for glow, saves it as `maps/<id>-<n>.png` (`.ktx2` with the codec and quality of the map it replaces, or else of the layer's picture, when that one is KTX2), declares it as a `linear` asset and sets `roughness_map` or `emissive_map` | `paint-map DIR LAYER roughness\|emissive --points X,Y;X,Y;... [--radius R] [--hardness H] [--value V]`, or `--heat` to print the map as a heat view |
| `ambient` | optional `ambient` and `sky` (colors), `clear` (removes both); sets `canvas.ambient` and `canvas.sky` | `ambient DIR [--ambient C] [--sky C] [--clear]`, or `--preview` to print the hemisphere they make |
| `import_script` | `file`, a `.js` module; copies it to `scripts/<name>` and declares or replaces its entry in `scripts` with the `events` it exports or handles with `on` and the `uses` it imports or calls (`geometry` for `scene.mesh`, `hierarchy` for `setParent`, `children`, `world` and `parent`, `shortcuts` for `openShortcut`, `spawn` and `order`, `effect`, `console`, `camera` for `scene.camera`, `camera3d`, `parallax`, `bloom`, `background`, `ambient` and `sky`, read as whole names or taken apart with `const { parallax } = scene`, `shared` also for imports of other modules), never its `params`, which are written by hand, refusing scripts that would take the scene past 1 MiB or 64 modules; the author checks what was proposed. With `template` (`empty`; `media`, a module that handles `mediaStatus`, `mediaPlayback`, `mediaProperties`, `mediaThumbnail` and `mediaTimeline` and exports `nowPlaying`, `accentColor`, `shown` and `progress` for values to follow with `fn`; `touch`, whose `update` returns 1 while the pointer is over the output and exports `position` and `pressed`; or `timer`, which counts seconds with `setInterval` and exports `seconds`) it writes that module instead, named `file` or, when `file` is omitted, `<template>.js` (`<template>-2.js` when taken), refused when the file exists; the assets panel offers it as New script…, and lists the scripts of the scene with their size, a view of their source, Open to edit them in another app and Remove | `import-script DIR [FILE] [--template empty\|media\|touch\|timer]` |
| `script` | one of `add` (`path`: a `.js` file of the package that the scene does not list yet; declares it like an import), `remove` (`path`; refused, naming each place, while a value follows the module with `script`), `move` (`path`, `to`: the place in the list, 0 first, which is the order the modules run in), `declare` (`path`, optional `params`: sets `events` and `uses` to what the code uses, keeping the `uses` that code cannot show (`clicks`, `geometry`, `animations`, `shortcuts`, `hierarchy`) and the `params`; with `params` the settings the code reads as `params.name` and the module does not declare are added as numbers), `write` (`path` of a listed module, `text`; saves the code and leaves its declaration alone, so the preview reloads and the check shows what changed; it is a step of the history: the editor keeps each version in its state folder, records the one it wrote under `scene.ext.editor.code`, and undo and redo write the recorded version back, only for the modules that step touched, so a file changed outside the editor is never overwritten by an unrelated undo) or `param` (`path`, `id`, `type` (`number`, `bool`, `text`, `color` or `vec2`), `default`, `min` and `max` for numbers, `to` to rename it, taking the `params` of the values that follow the module along, or `remove`; a new setting starts from the default of its type, a change of type drops `min` and `max`, and the default must fit the range; at most 32 settings) | Not a command of its own: `scene act DIR script JSON`, the generic entry for every action |
| `template` | one of `make` (`layers`, one or more layers of the scene, optional `name`: copies a picture, text, shape, solid, spectrum, mesh or group as it is into `templates` under the name, or `<id>_template`, and several layers as one group of copies; the layers stay), `rename` (`name`, `to`, optional `rewrite`: with it the exact string literals `"name"`, `'name'` and `` `name` `` of the scene's scripts become the new name, as one undoable step; the other lines that name it are never rewritten, and `templates::mentions` lists them with file and line) or `remove` (`name`) | Not a command of its own: `scene act DIR template JSON` |
| `import_assets` | `package`, a `.wpak` of kind `assets`, and `parts`, each `layer:ID`, `particles:ID`, `script:PATH` or `effect:ID` (every part when empty); copies the parts with what they name and their files under `assets/<id>-<version>/`, renames ids in use, credits the files with their license, attribution and `source`, and adds the package to `origin.derived_from` (see "Shared parts"); a scene of the first version is refused | `import-assets DIR PACKAGE [--part KIND:ID]…` |
| `asset` | `asset` and `cover` (`current`, `previous` or `none`) with an optional `placeholder` (a still picture: an asset id or a path of the folder; required for a new asset) makes that asset the cover of what the media player plays, or of what it played before, shown over the placeholder (`none` makes it a plain picture again), refused when the asset is animated or the value unknown. With `field` (a pointer of the scene such as `/scene/layers/0/image`) and `cover` `current` or `previous`, it shows that cover in the field in one undoable step: the asset `asset` when it already is a cover of that kind, otherwise any cover asset of that kind, otherwise a new one named `asset` (or `asset_2` and so on when the id is taken) over `placeholder`, so that a plain picture never becomes a cover; the picture picker offers it as Cover of what plays and Previous cover (made from the chosen picture, named `cover` and `previous_cover`) and, while a cover is chosen, Shown while nothing plays to pick its placeholder. Otherwise `asset` (a picture of the scene that is not KTX2 yet; an `animated` GIF or WebP becomes an array of its frames with their mean frame time in `KTXanimData`) and `ktx2: true`, optional `codec` (`uastc`, sharper and the default, or `etc1s`, smaller) and `quality` (1 to 100, 80 by default; kept in the file under the key `wprsQuality` so that tools that rewrite the picture keep it); encodes the picture as a KTX2 texture with Basis Universal and its mipmaps (left out when the asset sets `mipmaps: false`, wrapped when it repeats or mirrors, linear when it is `linear`) at its exact size (a side that is not a multiple of 4 pixels is padded inside the last blocks, and the card draws only the picture), writes `assets/<name>.ktx2` (`<name>-2.ktx2` when another file takes the name) and points the asset at it; undo points it back, and `prune` removes the old file once nothing names it. The assets panel offers it as Convert to KTX2 on each picture, with the codec, a quality slider, the file size before and after and, for each class of graphics card (BC7 on desktops, ASTC on phones, ETC2 on older phones, uncompressed on the rest), the memory it takes there and a crop of the middle of the picture as that card draws it, next to the same crop of the picture now (the graphics card of the app draws the crops of the classes it reads, the processor decodes the others, and each crop says which; `classes[i].drawn` is `gpu`, `cpu` or `plain`); the dialog encodes again on a worker thread a moment after the slider stops | `asset DIR ID [--cover current\|previous\|none] [--placeholder PICTURE] [--field POINTER]`, `asset DIR ID` (the asset, its size and what it takes on each class of card), `asset DIR ID --ktx2 [--codec uastc\|etc1s] [--quality N]`, or with `--dry-run [--looks DIR]` to only report and write the crops. An animated picture becomes an array that keeps the delay of each of its frames; a Radiance `.hdr` or OpenEXR `.exr` picture becomes a UASTC HDR texture that keeps its light above white; `cube: true` makes a cube map of the asset as an equirectangular panorama (faces a quarter of its width), and `faces` (six square picture assets of one size, `+X`, `-X`, `+Y`, `-Y`, `+Z`, `-Z`) makes a cube map of them, written over the asset `asset`; both are refused together, with fewer or more than six faces, with faces of other sizes and for animations. The dialog offers the same shapes with a picker per face, and each class of graphics card that keeps a shape uncompressed (cube maps, arrays whose frames are not whole blocks) says so, its size on the card counted as such; `wprs-ctl scene asset --ktx2 --cube` or `--faces +X,-X,+Y,-Y,+Z,-Z` does the same |
| `rgb` | `layer` (an adjustment layer) and one or more of `on` (`true` sends what it captures to RGB lighting and sets `permissions.rgb` in the same step, `false` stops and drops the permission when no other layer sends), `size` (`[w, h]`, 1 pixel to twice the canvas: the region around the layer position, which is set to the middle of the canvas when the layer has none) and `whole` (drops `size`: the whole canvas); undo returns both. The inspector of an adjustment layer has a switch for it, Whole canvas and A strip of it, and the strip the lights get, read from the preview 10 times a second through `Rgb1.GetStrip` and drawn as a grid of up to 64 × 16 cells; on the preview the region has corner handles that resize it around the layer position as one gesture, and the layer handles move it | `rgb DIR LAYER [--on true\|false] [--size WxH \| --whole]` |
| `translate` | `pointer` to a text of the manifest or of the scene (the `placeholder` of a `media` text), `language`, `text` (an empty or missing one removes that language), `add` (adds a language the text lacks, starting from the text in the default language, and leaves one it has alone) | `translate DIR POINTER LANGUAGE TEXT`, `--remove` or `--add` |
| `thumbnail` | `file`, `animated`; copies a picture into `assets/` as `previews.thumbnail` or `previews.animated` (`preview` in a first-version manifest) | `thumbnail DIR FILE [--animated]` |
| `shot` | one of `still` (`time`, the second of the scene; its `poster_time` without it: draws the scene and keeps the frame as the thumbnail, at most 1024 pixels), `recording` (`from`, `seconds` 0.1 to 15, `fps` 1 to 30; records a gif of at most 640 pixels and 8 MiB, made smaller until it fits, as the animated preview) or `flashes` (`from`, `seconds` and `fps`, 0, 8 and 15 without them; measures how much the scene flashes and keeps the result for the scene as it is); only scenes, on the graphics card of the process | `thumbnail DIR --capture [--time S]`, `thumbnail DIR --capture --animated [--from S] [--seconds N] [--fps F]`, `scene ask DIR flashes '{"run": true}'` |
| `upgrade` | optional `rating`, `reasons`, `photosensitive`, `derived_from`; moves a first-version manifest to `wallpaper-rs.wpak/2` | `upgrade DIR [--rating R] [--reason R] [--photosensitive B] [--derived-from A]` |
| `add_property` | `kind` (`slider`, `bool`, `color`, `combo`, `text`, `file`, `folder`, `shortcut`, `button`, `group`, `label` or `divider`), optional `id` (`<kind>_<n>` when omitted), `group`, `label`; the property starts valid, last of its group, and a combo with the options `option_1` and `option_2` and empty labels, so no text in a language the package may not use is written | `add-property DIR KIND` |
| `move_property` | `id`, `to` (its position among the properties of its group, 0 first), optional `group` (an empty one takes it out of its group); the `order` of that group becomes 1, 2… | `move-property DIR ID TO` |
| `rename_property` | `id`, `to`; the bindings, `prop:` triggers, groups, conditions (maps and expressions) and preset values that name it follow; scripts are not rewritten, and the command warns about each script that names the old id in quotes | `rename-property DIR ID TO` |
| `remove_property` | `id`; removes the property, its values in presets and in conditions that are maps, and the `group` of its members; refused, naming each place, while a binding, a `prop:` trigger or an expression condition names it; the command warns about scripts as `rename_property` does | `remove-property DIR ID` |
| `condition` | `id`, `text`, `enabled`: sets `visible_if`, or `enabled_if`, to the expression or, when `text` is a JSON object, to that map; refuses a condition that does not check, and an empty one removes it | `condition DIR ID TEXT [--enabled]` |
| `add_preset` | optional `id` (`preset_<n>` when omitted), `label`; a preset without values | `add-preset DIR` |
| `animate` | one of: `layer` (`layer`: a mesh or a model; `add` (a clip: a new animation layer playing it) or `index` (0 first), then optional `clip`, `weight` 0 to 1, `rate` 0 to 16, `additive`, `blend_in` and `blend_out` 0 to 10 s, `enabled`, `bones` (the bones it moves; empty for all), `to` (a new place), `remove`; at most 16); `root` (`layer`: a model with clips, `on`: sets `root_motion`); `preset` (`layer`, `bone`, `preset`: `hair`, `tail`, `cloth`, `antenna`, `chain` or `jelly`, which set `mode`, `stiffness`, `damping`, `inertia`, `gravity` and `wind` of its body, adding one when it has none); `morph` (`layer`, `shape` of the file or an expression, `weight` −1 to 1 or `null` to drop it) | `animate DIR LAYER layer [--add CLIP \| --index N] [--clip C] [--weight W] [--rate R] [--additive B] [--blend-in S] [--blend-out S] [--enabled B] [--bones A,B] [--to N] [--remove]`, `animate DIR LAYER root true\|false`, `animate DIR LAYER preset BONE PRESET`, `animate DIR LAYER morph SHAPE WEIGHT\|none`; `animate DIR LAYER clips` and `bones` print the clips of the layer and of the file with their events and root path, its animation layers, bones and blend shapes |
| `scene3d` | one of: `model` with `add` (`asset`, optional `id`, `into`, `at`: a position), `set` (`layer`, optional `position`, `rotation`, `scale`: three numbers each, the scale never 0, `size`, `env`: a picture, empty removes it, `asset`) or `remove` (`layer`); `group` (optional `id`, `into`, `layers` to gather; makes a group with `space: 3d`); `space` (`layer`, `deep`); `depth_test` (`layer`, `on`; only flat layers inside a 3D group); `material` (`layer`, `name` of a material of the file, optional `shader`: `pbr`, `unlit` or a package effect, `lit`, `double_sided`, `cast_shadows`, `receive_shadows`, `color`, `emissive` (`null` removes them), `params` with `metallic`, `roughness` and `occlusion` 0 to 1 and `normal_scale` 0 to 10, `maps` (per slot `base`, `orm`, `normal`, `occlusion` or `emissive`, a picture asset of the scene, `null` gives back the map of the file), `files` (per slot, a picture file imported as a new asset, as data for `orm`, `normal` and `occlusion`, and chosen in the same step), `clear`; each map of the materials panel is a button with its thumbnail that opens a picker: the map of the file, a picture from a file or any picture of the scene with its thumbnail, and it warns when a data slot holds a picture read as colors); `paint` (`layer`, `name` of a material, `slot`, `color` as three numbers 0 to 1, `radius` as a share of the picture, optional `hardness`, `points` from 0 to 1 over the picture, 1 to 4096 of them) paints a stroke into that map, starting from the picture the scene chose, else the map of the file, else a white (or flat normal) picture of 512 px, and saves it under `maps/` as the picture of the slot (as data for `orm`, `normal` and `occlusion`); *Paint this map…* in the picker of a map opens it flat with a brush, a color, a size and a hardness; `camera` (optional `fov` 1 to 179, `near`, `far`, `eye`, `center`, `up`, `roll` −180 to 180 degrees around the view, `from_view`, `clear`); `view` (the authoring view in `ext.editor.view3d`: `orbit` in degrees, `pan` in pixels, `zoom` as a factor, `frame` (a model layer), `through`, `reset`); `tones` (optional `hdr`, `tonemap`: `none`, `aces` or `agx`, only with `hdr`, `split` 0 to 1 for the before and after preview); `shadows` (`layer`: a point, spot or directional light, optional `on`, `resolution` 256 to 4096, `automatic` (drops it), `bias` 0 to 1; at most 2 lights); `path` with `add` (`id`: a path of two keys, at 0 and 2 s, where the authoring view looks), `key` (`path`, `time`, optional `eye`, `center`, `up`, `fov` 1 to 179, `from_view`, `from` (moves that key), `remove`; a path keeps 2 keys), `remove` (`path`), `set` (`path`, optional `loop`, `ease`, `to`: a new id), `order` (`sequential` or `random`) or `fade` (optional `to`: `black` or `cross`, `duration` 0 to 10, `clear`); `new_material` (`name`: writes `effects/<name>.json` and `.wgsl` from a template of a material shader and adds the effect); `debug` (optional `show`: `mask`, `matte` or `shadows` in `ext.editor.debug3d`, which the preview draws instead of the light; without it the preview lights again) | `model DIR add\|set\|remove\|info …`, `shadows DIR LAYER [--on B] [--resolution N\|--automatic] [--bias B] [--show]`, `camera-path DIR add\|key\|remove\|set\|order\|fade\|show …`, `new-material DIR NAME [--check]`, `debug3d DIR mask\|matte\|shadows\|none`, `group3d DIR add\|space\|depth-test\|show …`, `material DIR LAYER [NAME] … [--key COLOR] [--list]`, `camera3d DIR …`, `view3d DIR …`, `tonemap DIR …`, or `scene3d DIR --model JSON` (one of `--model`, `--group`, `--space`, `--depth-test`, `--material`, `--paint`, `--camera`, `--view`, `--tones`, `--shadows`, `--path`, `--new-material`, `--debug`) as the app sends it; `model-info FILE.glb` and `cost3d DIR` print what the inspector shows |
| `effects` | one of `add` (`layer`, `effect`: a built-in effect or one of the package, optional `name`, `at` (the top when omitted), `params` and `optional`; starts the stack of a layer that has none), `set` (`layer`, `index`, optional `params`, where `null` goes back to the default, `enabled`, `name`, an empty one removes it, and `optional`), `remove` (`layer`, `index`; the last one removes the stack), `move` (`layer`, `index`, `to`), `grip` (`layer`, `index`, `param`, `to`, optional `both`, `at` and `replace`: what a handle of the canvas does when it is dragged to `to`, a point of the canvas in pixels as the preview draws it, under the parents, groups, anchor, origin and parallax of the layer; a `position` or point parameter takes the fraction of the layer under `to`, a direction takes the vector from the first point of the effect (the middle of the layer when it has none) to `to` over a fifth of the shorter side of the layer, each part from −1 to 1, and an angle takes the angle of that same vector in degrees, clockwise from the right, turned by whole circles to the one nearest to the current value within the range of the parameter; with `both`, the other end of a pair of points (`from` and `to`, `start` and `end`, `a` and `b`, `first` and `second`) moves by the same amount; refused for a parameter without a handle, a layer with no size when the parameter is a direction or an angle and a parameter that follows keyframes or another source unless `replace`) `mask_grip` (see Inspector) or `mask` (`layer`, optional `index`, the mask of the layer itself when it is left out, optional `texture`, `from` (a layer) or `shape`, one of them, which replaces the source of the mask, `channel`, `invert`, `feather`, `strength`, or `off`); refuses an effect it does not know, a parameter the effect lacks, a number outside the range of the parameter, a sound or light layer, a mask without a source and any problem the change adds to the scene, as the inspector does | `effects DIR LAYER add EFFECT [--param K=V] [--name N] [--at I] [--optional]`, `set I [--param K=V] [--unset K] [--enabled B] [--name N] [--optional B]`, `remove I`, `move I TO`, `grip` and `mask_grip` through `scene act DIR effects JSON`, `mask I [--texture A] [--from L] [--shape JSON] [--channel C] [--invert B] [--feather PX] [--strength S] [--off]` |
| `layer` | one of `transform` (`layer`, optional `position`, `rotation` (−360 to 360), `scale`, `skew`, `origin`, `tilt` (each angle −89 to 89), `z` (−100000 to 100000) and `size`, a box of an image, video, solid, adjustment or spectrum layer above 0; what is not given stays, and a given field that follows keyframes or another source is refused unless `replace` writes the plain value over it), `grip` (`layer`, `grip`, `to`, optional `snap`, `at` and `replace`: what a handle of the canvas does when it is dragged to `to`, a point of the canvas in pixels as the preview draws it, under the parents, groups, anchor, origin, parallax and 2D camera of the layer; `move` puts the pivot of the layer (its `position`) at `to`, and with `snap` (pixels) first pulls the edges and the middle of the layer to the edges and the middle of the canvas and of the other layers that are not inside it or follow it, when they are that close; `nudge` moves the pivot by `to`; `turn` points the top of the layer at `to`; `origin` moves the pivot to `to` and changes `origin` so that the layer stays where it is; on a tilted or lifted layer, or under a tilted parent, `to` is first brought back onto the layer as the canvas section below tells; `{ "tilt": { "axis" } }` and `lift` set `tilt` and `z` from the depth gizmo; `{ "scale": { "corner", "even" } }` scales the layer so that the point `corner` of its box (fractions, `[1, 1]` the bottom right, `[1, 0.5]` the middle of the right edge, which scales one side) lands on `to` while the pivot stays, keeping the proportions with `even`; `{ "skew": { "edge" } }` skews along the edge whose middle `edge` names, so that it passes through `to`; `{ "size": { "corner" } }` is for an adjustment layer that captures a region (it has `size`): it writes `size`, not the scale, so that the point `corner` of the captured rectangle (the middle of an edge changes one side) lands on `to` while the pivot stays, from 1 pixel up to twice the canvas on each side, in the coordinates of the parents and without the turn, skew or scale of the layer, as the capture is placed; the other grips each write as `transform` does, with `at` the second of the playhead), `fit` (`layer`, `fit`: `cover`, `contain` or `stretch`, and `at` and `replace`: scales and places the layer so that its box covers the canvas, fits inside it or fills it, keeping its rotation and skew, under its parents), `parent` (`layer`, optional `parent`, none to stop following, `keep`, and `at` and `replace`: sets or removes `parent`, refused for the layer itself, a sound or light layer, a layer that follows it already, one inside it, or a chain that would pass 8 layers; with `keep` it writes the position, rotation, skew and scale that leave the layer where it was), `blend` (`layer`, `blend`: a blend mode), `touch` (`layer`, `takes`: `off` lets the pointer through, `all` takes it over the whole box and `alpha` only where the layer is at least `alpha` opaque, 0 to 1, 0.5 or the one it had when omitted; writes or removes `solid`), `figure` (one of `shape`, `stop`, `text_width` or `fonts`, see "Shapes, text and fonts"), `depth` (`layer`, `depth`: `[across, up and down]`, each −10 to 10, the parallax depth of the layer, written as one number when both are the same and removed at 0; it adds `pointer` to the inputs of the manifest when they lack it) or `instance` (`layer`, a particles layer, `values`: `alpha`, `size`, `count`, `speed`, `life`, `time_scale`, `brightness`, `color` or `points`, `null` drawing that value as the system does again, `points` as a list from `p0` or as an object of point numbers from 0 to 7 that changes only those points, `null` giving one back to the system, and `clear`); refuses what the scene would not take | `layer DIR LAYER grip move\|nudge\|turn\|origin\|scale\|skew X,Y [--corner U,V] [--even] [--snap PX] [--at S] [--replace]`, `fit cover\|contain\|stretch`, `parent [PARENT] [--keep]`, `transform [--position X,Y] [--rotation D] [--scale X,Y] [--skew X,Y] [--origin X,Y] [--tilt X,Y] [--z Z] [--size W,H] [--replace]`, `blend MODE`, `touch off|all|alpha [--alpha A]`, `instance [--alpha A] [--size S] [--count C] [--speed S] [--life L] [--time-scale T] [--brightness B] [--color C] [--point X,Y]... [--move-point K=X,Y]... [--unset-point K]... [--unset V] [--clear]` |
| `manifest` | one of `origin` (`kind`, optional `derived_from`, `provider`, `source_url`; replaces the origin), `license` (`license`, an SPDX expression of at most 256 characters), `tags` (`tags`, with `add` or `remove` instead of replacing them), `authors` (`authors`, at most 64; an empty list removes them), `credit` (`paths` of files or folders that end in `/`, `license`, `attribution` and an https `url`; joins the credit that has the same license, attribution and address, up to 256 paths, and refuses a path that is not in the folder), `uncredit` (`paths` to take out of the credits; a credit left without paths goes), `text` (`key`, `language` and `text`, empty or missing to remove: writes `strings.<language>.<key>`, or the file `locale/<language>.json` when that language lives in one; a key has up to 64 letters, digits, `_`, `.` or `-` and `wprs.` is the engine's) or `locale` (`language` and `file`: moves the texts of a language between the manifest and its `locale/<language>.json`, and lists or drops `locale_files` in `requires.features`) or `display` (`values` of `fit`, `zoom`, `position`, `background`, `span`, `flip` and `accent`, and `clear`, settings to remove); refuses what the checks of the manifest refuse, each broken field on its own, so an earlier problem does not hide a new one | `manifest DIR origin KIND [--derived-from AUTHOR\|JSON] [--provider NAME --item-url U --credit C [--terms-url U]] [--source-url U]`, `license L`, `tags T... [--add\|--remove]`, `authors A...`, `display [--fit F] [--zoom Z] [--position X,Y] [--background C] [--span S] [--flip F] [--accent C\|prop:ID] [--clear S]` |
| `property` | `id`, optional `min`, `max` and `step` (sliders only), `default` and `order` (its place among the properties of its group, as `move_property` does); refuses a default outside the range, as validation does | `property DIR ID [--min N] [--max N] [--step N] [--default V] [--order N]` |
| `canvas` | one of `size` (`size`, `[width, height]` from 1 to 16384 pixels; the *Canvas and clock* menu of the canvas toolbar offers the screen sizes of the outline field `screen_sizes`, from HD to 5K, ultrawide 21:9 and 32:9, 16:10, 4:3 and a phone in portrait), `clock` (optional `poster`, the second of `poster_time`, 0 to 86400, rounded to the millisecond, which the same menu takes from the playhead, and `seed` or `fresh`, which writes `seed: null` so that every start picks a new seed; the menu toggles it, a fixed seed of 1 turning it off), `bloom` (optional `threshold`, `strength`, `radius`, `tint`, `hdr` (`[threshold, strength]` of the glare above white, or `none` to remove it; anything else is refused) or `off`; a new glow starts from the defaults) `parallax` (optional `amount`, `delay`, `pointer`, `gyro` or `off`), `camera` (optional `center`, `zoom` 0.1 to 10, `rotation` −360 to 360, `at` and `replace`, as the `transform` of a layer writes over keys, or `off`; the 2D camera of `canvas.camera`, made when it is missing), `camera_grip` (`hold`: `move`, `{ "zoom": { "corner" } }` or `turn`, `to`, a point of the canvas without the camera, optional `at` and `replace`: `move` puts the middle of what the camera shows at `to`, `zoom` sets the zoom that puts the corner `corner` of the screen (fractions) on the line from the middle to `to`, at its distance, and `turn` turns the camera so that the top of the view points at `to`, to the angle nearest to the current one) `probe` (optional `pointer`, a fraction of the canvas from 0 to 1 on each side, `tilt`, −1 to 1 on each side, or `off`: a simulated pointer and tilt of a phone kept in `/scene/ext/editor/probe`, which `seats::seats` feeds to the runtime as a pointer that has settled, so every outline and handle of the canvas shifts by its parallax depth as the scene would) or `camera_view` (`whole`: the canvas shows the whole scene with the frame of the camera, kept in `/scene/ext/editor/camera2d`, which only the preview and the canvas read); the inspector of the canvas shows both with switches and sliders, which change the scene live while dragged, record one step per drag and write nothing when clicked without moving | `canvas DIR size W,H`, `clock [--poster S] [--seed N \| --fresh]`, `bloom [--threshold T] [--strength S] [--radius R] [--tint C] [--hdr T,S\|none] [--off]`, `parallax [--amount A] [--delay D] [--pointer P] [--gyro B] [--off]` |
| `timeline` | `id`, optional `add` (a new timeline, of 5 s unless a length is given; at most 64), `remove` (every value it drove, keyed or tracked, follows the scene clock again), `to` (a new id; every value it drives, keyed or tracked, follows), `duration` (seconds) or `frames` with `fps` (1 to 240), `wrap` (only with `repeat`), `loop` (`once`, `repeat` or `pingpong`; leaving `repeat` drops `wrap` unless it is given too), `speed` (−100 to 100), `delay` (seconds), `autoplay`, `trigger` (empty removes it), `clear_events` (removes every event first), `drop_events` (names, each removing every event of that name, or `name@t` only the one at `t` seconds; refused when none matches) and `add_events` (`{t, name}` from 0 s to the end of the timeline, names of 1 to 32 small letters, digits or `_`, the same name twice at the same time refused, at most 64; kept in order of time); refused when it leaves a problem in the scene. The Timelines menu of the timeline adds, renames and removes them, and *Edit the timeline and its events…* opens a panel with every field (seconds, or frames at a frame rate; how it loops, wrap while it repeats, a speed slider and field, delay, autoplay and the trigger, named as the inspector names it) and a lane of its events: a double click adds one there, a drag moves one (`drop_events` and `add_events` in one step), and the chosen one is renamed or removed | `timeline DIR ID [--add \| --remove] [--rename NEW] [--duration S \| --frames N --fps F] [--wrap B] [--loop L] [--speed X] [--delay S] [--autoplay B] [--trigger T] [--clear-events] [--drop-event NAME[@T]]... [--event T:NAME]...` |
| `pool` | `id` (1 to 32 small letters, digits or `_`), `add` (a new mesh pool of 16 shapes of 1024 vertices unless `max` and `max_vertices` are given; at most 16 pools), `remove` (refused while a layer draws the pool, and it names them), `max` (1 to 256 shapes), `max_vertices` (3 to 65536), `texture` (an image asset), `size` (`[width, height]`, 1 to 4096) and `clear` (`texture`, `size`); refused when it leaves a problem in the scene. The *Shape pools* list of the Assets panel shows each pool with its size and the layers that draw it, adds a shapes layer on it, changes it in a dialog (name, shapes at once, vertices, picture from the image assets and own size) and removes it; `{"pools": {}}` of `rig` lists the pools with their limits, the layers that draw them (`layers`) and the scripts that declare `geometry` (`scripts`) | `scene act DIR pool JSON`, `scene ask DIR pools` |
| `add_layer` with kind `shapes` | a `video` layer that draws a pool: `source` names the pool, the first one when it is left out; refused until a pool exists. The source of any `video` layer offers the pools besides the video assets, so the inspector picks either; the Add layer menu lists *Shapes from a pool* once there is a pool | `scene act DIR add_layer JSON` |
| `uses` | `path` (a script of the scene; every script when left out), `add` and `remove` (names of the `uses` of the script, such as `geometry`, `camera`, `hierarchy` or `shortcuts`); with neither, it declares what the source of the script uses and its `uses` lack. A script that gets `geometry` in a scene with no pool also gets a pool `shapes`. Importing a script (`import_script`) detects `geometry` (`scene.mesh`), `hierarchy` (`setParent`, `children`, `world`, `parent`), `shortcuts` (`openShortcut`) and `camera` along with the other parts; a saved script that uses more than it declares (apart from `audio`, `pointer`, `clicks` and `particles`) is a problem with a one-click fix (the `fix` of the problem is the action to send), shown in the Problems list as *Fix* and in the *What the scripts declare* list of the Assets panel as *Declare what it uses*; `{"uses": {}}` of `rig` lists, for each script, `declared`, `detected` and `missing` | `scene act DIR uses JSON`, `scene ask DIR uses` |
| `film` | one of `trim` (`layer`, `start`, optional `end`; without `end` the video plays to its end; the part must lie inside the video, whose length is read from its header, and last 0.05 s or more), `rate` (`layer`, `rate` from 0 to 4) or `fit` (`layer`; sets `size` to the real size of the video); the layer is a video layer or an image layer that shows a video | `scene act DIR film JSON` |
| `sprites` | one of `grid` (`asset`, `rows`, `columns`, optional `margin`, `spacing`, `count` and `order`: `rows`, `columns`, `rows_reversed` or `columns_reversed`; replaces `atlas` and `rects` with the rectangles of the grid), `rects` (every frame), `add`, `change` (`index`, `rect`), `remove`, `reorder` (`from`, `to`), `play` (`fps` from 0.1 to 120 and `frames`, from 1 to the number of frames) and `clear`; each `rect` is `[x, y, width, height]` inside the picture, with at most 256 frames | `scene act DIR sprites JSON` |
| `keys` | `picks` (lista de `{pointer, times}`: un valor animado o un canal de un clip, `/scene/layers/N/clips/M/bones/B/rotation` y demás, con los segundos de sus claves elegidas), y una de estas ediciones como un solo paso: `shift` (segundos sumados), `scale` (mayor que 0) con `pivot` (estira los tiempos alrededor de ese segundo, junto con el largo de sus asas), `lift` y `value` (suman o fijan la parte `part` del valor: 0 para x o rojo, 1 para y o verde…; `lift` sin `part` suma a todas las partes, salvo el alfa de un color; `value` en listas y colores pide `part`), `remove` (quita las claves elegidas, dejando al menos una), `in_handle`/`out_handle` con `aligned` (una sola clave: el asa dada y, alineada, la otra con la misma pendiente y su propio largo, o un tercio del tramo si no tenía), `straighten` (como en `keyframe`), o pegar: `keys` dentro de cada pick (claves copiadas tal cual) con `at`, el segundo donde cae la primera de todas, reemplazando las claves de esos segundos y animando un valor que era fijo. Se rechazan claves que caen antes de 0 s o a menos de un milisegundo de otra, asas que pasan de su vecina y valores que la escena no lee, con mensajes que nombran el campo; las asas que una clave movida deja más largas que su tramo se acortan hasta la vecina | `scene act DIR keys JSON` |
| `undo`, `redo` | none | `undo`, `redo` |

`film` and `sprites` have no command of their own: `scene act` runs them. The queries `film`
(`{"film": {"layer", "strip"}}`: real size, length, trim and speed of the video of a layer),
`sprites` (`{"sprites": {"asset"}}`: size of the picture, frames, `fps` and `frames`) and `uses`
(`{"uses": {"asset"}}`: every pointer that names the asset, with the layer that holds it) go through
`rig`, like the other queries of the canvas.

A video layer has on the canvas the box of the real size of its video (that of `size` when the
layer sets it), not one of 640 × 360. In the inspector of a video layer, or of an image layer that
shows a video, a panel draws a strip of frames along the real length of the file, with two grips
that trim `start` and `end` to the hundredth of a second, a ruler in seconds, the speed (0 to 4
times, which is what the format takes; the range of `playback.rate` in the inspector ends at 4 for
these layers too), the loop, *Use the size of the video* and *Play the whole video*. Wherever a
texture is picked (the `texture` of a mask, an effect, a mesh or a spot light) the list also holds
the video assets; the lists of normal, environment and particle-pool maps still ask for pictures.

The list of assets shows a thumbnail of each (the picture, the first frame of a video, an "Aa"
sample of a font, an icon for sounds, color tables and models), takes files dropped on it (it
imports them) and, before an asset in use is removed, lists every place that names it and asks to
confirm. Files dropped on the canvas import the pictures, videos and sounds among them and add one
layer for each. The import dialog also offers `.mp3`, `.ttc`, `.otc` and `.woff2` files.

The sprite cutter (the *Cut sprites* button of each picture) shows the picture with its numbered
frames: a grid is cut by rows, columns, margin, spacing, number of frames and order; dragging over
the picture draws a frame, dragging a frame moves it and dragging its corner resizes it; a strip of
the resulting frames plays them at the chosen frames a second, and lets them be reordered, removed
and limited to the first ones (`frames`).

A new layer gets an id made from its `name`, or else from its kind (`text`, `text_2`…; accents
are dropped, other characters become `_`, and a name with no Latin letter or digit falls back to
the kind), and sensible defaults; a text layer starts with its name as its text, and a particles
layer without a `source` also gets a new system. The app sends the name of the kind in the
language of the desktop (`Texto` becomes `texto`), and offers an image, video or sound layer only
once there is an asset of that kind. `keyframe` turns a plain value into
`{"keys": [[time, value]]}`, adds or replaces the key at that time (to the millisecond), keeps the
keys sorted and turns the value back into a plain one when its last key goes; it refuses a value
that follows another source. Keyframe times are seconds on the named timeline, or on the scene
clock without one. In the timeline of the app a keyframe is dragged along its track, to the
hundredth of a second, and dropped with `keyframe` and `from`.

El editor muestrea las claves con el mismo código que el motor (`wprs_scene::sample_json`): lee
la lista como la lee la escena (números, pares, ternas, colores, interruptores, textos y emisores),
llama al mismo muestreo con su suavizado y sus asas Bézier y escribe el resultado como JSON, con
los colores en hexadecimal y mezclados en OKLab como en la pantalla. Así las asas, las vistas
previas, las luces, los relojes y las estimaciones que el editor calcula a un segundo dado coinciden
con lo que dibuja el motor; solo una lista que la escena no puede leer se mezcla aparte, parte por
parte. Una lista de tres números se interpola como una terna (no como color): un color escrito como
`[r, g, b]` se prefiere en hexadecimal o con cuatro partes.

The template picker of the layers bar shows the scene, or one script template alone on the canvas as
a copy of it draws (the preview look `template`). While a template is shown, the layers panel lists
its layers, the canvas puts its move, bone, chain, weight, point and attachment handles on them, and
every action and query that names layers, from `bone`, `ik`, `attach`, `weights` and `parts` to
`hide` and `add_layer`, works on the layers of that template by their own ids, so an attachment, a
parent or a chain target names a layer of the same template as each copy needs; a template holds one
layer, so new layers go inside its group (`group` on its layer makes one) and an action that would
add a second one beside it is refused. An attachment, a chain target `layer:<id>` or a mask may also
name a layer of the scene: the lookup tries the template first and then the scene, a point placed on a
mesh of the scene goes into that mesh, the inspector of a template layer offers *Follow a mesh of the
scene* (a mesh, then one of its bones or points), the chain target picker lists the layers of the
scene after those of the template, and `wprs-ctl scene --inside TEMPLATE attach DIR LAYER --to MESH`
does the same. Actions that are not about layers (properties, conditions, presets, assets, imports,
the manifest, the canvas, timelines, wind, ambient light, particle systems and every `set`, `add`,
`remove`, `move` or `translate` outside `/scene/layers`) act on the whole scene while a template is
shown, so renaming or removing a property sees and changes the bindings of the scene too. Its fields
are the fields of `/scene/templates/<name>` in the inspector. Bones, weights and points live in the
mesh asset, so a mesh layer in a template and the layers of the scene that name the same asset change
together, as any two layers sharing a mesh do; chains, attachments, parts and depth live in the layer
and stay its own. `wprs-ctl scene --inside TEMPLATE …` does the same for every scene command, and `wprs-ctl scene open DIR --json --inside TEMPLATE` prints what the editor shows.

The timeline shows a lane for each own video clock (`playback.own_clock`): a strip of eight frames
taken evenly over the video (decoded once off the interface thread and kept in the cache of the
user, `$XDG_CACHE_HOME/wallpaper-rs/strips`, at most 512 files), the range of the video as a bar whose
grips trim `start` and `end`, its speed, loop and play switches and a mark where the clock is at the
playhead; `wprs-ctl scene clocks DIR [--time T] [--strip N]` prints the same lanes as JSON, with the
files of N frames (at most 16) of each with `--strip`.

`import` copies pictures, videos, sounds, fonts and `.cube` tables. It refuses a file over the
limits of the format (256 assets, 1 GiB per file, 512 MiB of pictures once decoded, sounds of 32
MiB and 10 minutes at most, the ones up to 60 s 180 s together and the longer ones to be streamed,
tables of 16 MiB); MP3 sounds and `.ttc`, `.otc` and `.woff2` fonts are imported too or an asset id in use before it
copies anything. The file is copied in blocks to a hidden file in `assets/` that is renamed when
complete. A file with the same name and other content is stored as `name-2.ext`; one with the same
size and SHA-256 is reused. The app imports, builds and applies on a worker thread, one task at a
time, so the window keeps answering. `prune` deletes the files of `assets/` that the scene, its
previews and its credits no longer name, and the meshes of `meshes/` that neither the scene nor the
history names.

## Acciones desde la línea de comandos

Regla: toda acción del editor se alcanza con `wprs-ctl scene act`. Cada operación del editor es una
variante del enum `Action` de `crates/wprs-editor` (o una consulta de `rigs::query`), y la línea de
comandos no tiene un subcomando propio por herramienta. Solo existen subcomandos dedicados para lo que
una persona o un script hace a mano: crear, importar, previsualizar, capturar, comprobar, empaquetar,
firmar, publicar y los repositorios. Una función nueva del editor entra como `Action` con su prueba
unitaria en `crates/wprs-editor`, y los guiones de integración la ejercitan con `scene act`.

- `wprs-ctl scene act DIR ACCIÓN [JSON]` aplica una acción a la carpeta de escena, igual que la
  aplicación: la misma validación, el mismo historial para deshacer y los mismos mensajes de error.
  `JSON` es el objeto de argumentos de la variante; se puede pasar como argumento, con `--file RUTA` o
  por la entrada estándar si vale `-`. Sin `JSON`, los argumentos son `{}`. Imprime los ids de lo que
  se añadió, o con `--json` las operaciones que cambiaron el documento.
- `wprs-ctl scene act DIR batch JSON` toma una lista JSON de objetos `{"acción": {argumentos}}` y las
  aplica como un solo paso de deshacer (`Project::act_all`): si una falla, no queda ningún cambio en el
  proyecto ni en el historial, y deshacer o rehacer no pueden formar parte de la lista.
- `wprs-ctl scene actions` lista cada acción con una línea; `scene actions NOMBRE` imprime sus
  argumentos (nombre, tipo, si son obligatorios u opcionales, valores de las enumeraciones y valores por
  defecto) y una invocación de ejemplo; con `--json` imprime el esquema JSON de la acción. El esquema
  sale de los tipos con `schemars`, y una prueba de `crates/wprs-editor` (`manual`) falla si una variante
  de `Action` no figura en la lista, no tiene descripción o su ejemplo no se deserializa.
- `wprs-ctl scene ask DIR CONSULTA [JSON] [--layer ID]` hace una consulta del editor (las mismas que la
  aplicación pide con `rigs::query`) e imprime la respuesta como JSON; `scene actions --queries
  [NOMBRE]` las lista y describe. `--inside PLANTILLA` vale para los tres comandos.

## Scripts y plantillas

La sección de scripts del panel de recursos lista cada módulo de `scripts` en el orden en que se
ejecuta: su tamaño, cuántos valores lo siguen, un aviso cuando lo que usa el código difiere de lo que
declara la escena, y botones para subirlo o bajarlo, editar su código, abrir el archivo en la
aplicación del sistema y quitarlo. El botón de adjuntar ofrece los archivos `.js` de `scripts/` que la
escena aún no lista. Al desplegar un módulo se ve la comprobación del código contra su declaración y
sus ajustes.

- La comprobación (`scripts::audit`) lee el código e informa de lo que usa y la entrada no declara, y
  de lo que la entrada declara y el código no usa, para `events`, `uses` y `params` (los ajustes que
  se leen como `params.nombre` o `params["nombre"]`). Las partes que el código no puede mostrar
  (`clicks`, `geometry`, `animations`, `shortcuts`, `hierarchy`) nunca se informan como sin uso. El
  botón *Declare what it uses* ejecuta `script` con `declare`.
- Los ajustes de un módulo son una tabla con una fila por ajuste: su id, su tipo, su valor por defecto
  (un campo para números y textos, una casilla, el selector de color o dos números) y, para los
  números, el mínimo y el máximo. Cada cambio es una acción `script` con `param`; si el módulo rechaza
  un valor, la fila vuelve a como estaba.
- La vista de código edita el módulo: guarda con el botón o con Ctrl+S (`script` con `write`), muestra
  la comprobación sobre el texto, abre el archivo en la aplicación del sistema y lista la API de
  scripts (`scripts::reference`: las partes del módulo `wprs`, las funciones que exporta un módulo y
  las funciones que sigue un valor). Con la aplicación abierta se mira una vez por segundo el tamaño y
  la fecha de modificación de los módulos, solo cuando la escena lista scripts: si un archivo cambia
  fuera de la aplicación, se vuelve a leer el texto de una vista de código sin modificar, se repite la
  comprobación y la vista previa reinicia la escena con el código nuevo, porque el documento que
  recibe lleva una marca de los módulos (`scene.ext.editor.scripts`).
- El menú de enlace de cada valor tiene *Follow a script*: para cada módulo, *every frame* (el
  `update` del módulo) y cada función que exporta además de los eventos (`scripts::functions`); una
  función de un módulo con ajustes se enlaza con `params` rellenado con sus valores por defecto.
- La sección de plantillas lista cada plantilla de `templates` como un árbol de sus capas. Pulsar una
  capa la selecciona, de modo que el inspector muestra sus campos; el botón de vista previa muestra la
  plantilla sola como la dibuja una copia, y cada plantilla se puede renombrar o quitar. *Make a
  template* de la barra de capas toma la capa seleccionada y las marcadas con Ctrl.

Las consultas `scripts::listing` (los módulos con sus funciones, ajustes, los valores que los siguen y
su comprobación, y los archivos libres de `scripts/`), `scripts::audit`, `scripts::reference`,
`scripts::stamp`, `scripts::changes` y `templates::tree` viven en `crates/wprs-editor`; la aplicación
llega a ellas con la consulta `code` del editor de escenas.

## History

Each action that changes something is one step `{label, undo, redo}`. Only gestures merge: the
app marks the changes it sends while a slider or a canvas handle is dragged, and such a change
merges into the previous gesture with the same label, recorded less than 500 ms before, when its
operations only replace values that step set; while a press holds a gesture open (`Project::hold`,
the `hold` of the app on press and release, which every slider of the scene editor sends, and the
canvas for every left press on it, so each handle of the canvas holds it while pressed), its
first change starts a step of its own and the next ones merge into it however long the drag pauses,
and an action that is not part of the gesture closes it; an undo or redo while it is held makes the
rest of the drag a step of its own. Dragging is one step, while typed values, moves,
insertions, removals and every `wprs-ctl scene` command are steps of their own. A new step drops
the steps that were undone; an action that changes nothing once the computed fields follow it,
such as unticking an input the scene uses, records no step. The history keeps at most 500 steps
and 64 MiB of values, dropping the oldest first.

The history is stored with the unsaved document in
`$XDG_STATE_HOME/wallpaper-rs/editor/<hash of the folder>.json` (mode `0600`, folder `0700`),
together with a hash of the files it started from. The app writes it every 30 seconds while
something changed and when the editor closes; `wprs-ctl scene` writes it after each command, so
undo and redo work across commands and between the app and the command line. When a folder opens
with unsaved changes based on the same files, the app offers to recover or discard them, and
`wprs-ctl scene recover [--discard]` does the same. Saving refuses to overwrite files that changed
since the folder was opened. The file goes away when the editor closes with everything saved or
the history is empty; a new one deletes those untouched for 30 days and keeps the 32 newest.
`wprs-app --check` keeps them in a temporary folder.

## Inspector

The inspector is generated from the JSON Schema of scenes (`wprs-ctl scene schema`) and of user
properties (`--property`), produced by `schemars` from the types of `wprs-scene`, so a field added
to the format appears without changes to the editor. `wprs-ctl scene fields DIR POINTER` prints
the fields it shows for a pointer, as JSON. Annotations in the schema guide the widgets:

| Annotation | Meaning |
|------------|---------|
| `x-unit` | unit shown after the label (`px`, `°`, `s`) |
| `x-range` | `[min, max]` of a slider |
| `x-ref` | the value names an `image`, `video`, `sound`, `font` or `lut` asset, a `layer` (for the `layer` shape of a particle emitter only the layers that may feed its system), a particle `system`, a `control` point of the system (`p0` and the `points` it has), a `timeline`, a user `property` that holds a value or, for the `trigger` of a timeline, the triggers listed for `trigger` above, or a built-in `effect`, offered in a list; properties show their labels and effects their translated names |
| `x-key` | a keyframe `[time, value, ease]` |
| `x-curve` | a list of `[t, value]` points edited on a plot: a click adds a point, a drag moves it (one gesture) and a double click removes it; `t` goes from `0` to `1` across and the value up to its `x-range`, or a quarter above the highest point |

Numbers become sliders when they have a range, colors a color picker, enumerations and tagged
variants a list with translated names, pairs two fields, `x-curve` lists a curve, color stops
`[t, color]` rows of a time and a color picker, fixed pairs of points (a box, a line) two rows of
coordinates, objects a group of fields and other lists a count with *Add* and *Edit…*. A number
that may also be a name, such as `clock.max_fps` (`display`), has the widget of the number and the
names in `options`, shown as check boxes. The app names every field, value, effect, easing, layer
kind, asset kind, language and computed feature in the language of the desktop; a test fails when
the schema gains one without a name, and another walks the inspector of every starting point with
every layer kind, variant and list item and fails when an option or a shown value has no Spanish
translation or a raw JSON field holds a plain text. Effect parameters come from the descriptor of
the effect, with its `label` as `title` and its `help` as `help` (a tooltip) in the language of the
desktop; a `texture` or `mask` parameter offers the image assets and every other layer, written as
`{ "layer": "<id>" }`, and every entry of `options` is a field of its own, with its values as
choices and bindable to a property. A parameter that has a handle on the canvas draws it
while the effect use or that field is selected, placed where the renderer puts it under the parents,
groups, anchor, origin and parallax of the layer, and the query `effect_handles` (`layer`, `index`,
`time`) lists them with their canvas position, what the descriptor declares being what decides: a
`position` parameter, a `vec2` named `center`, `source` or `pivot`, or the four corners of a homography
effect such as `perspective` (linked as a quad) is a point; a `vec2` from −1 to 1, such as a direction,
is an arrow from the first point of the effect (the middle of the layer without one) with a handle at
its tip; a `float` with `unit: degrees`, such as the `angle` of `spin` or `waves`, is a dial: a ring
around that same point with a handle that turns it, and the field gets a dial next to its slider, as
particle emitter angles already have; two points whose names pair up (`from` and `to`, `start` and
`end`, `a` and `b`, `first` and `second`) are joined by a line that has a handle in the middle to move
both, and Shift while dragging an end moves both; every drag is one gesture of `effects grip`. A
package effect gets all of this from its own descriptor. An adjustment layer with `size` shows what it captures: the rectangle (in the coordinates of its parents, around its position as `origin` places it, without its own turn, skew or scale, as the renderer captures it), a dashed frame with the most it may capture (twice the canvas on each side, in red when `size` goes past it), with `redraw` the outline where the capture is drawn again, a caption with the size and the limit, and eight handles on the corners and the middles of the sides that edit `size`, not the scale (the generic scale handles are not drawn for such a layer). The query `capture_view` (`layer`, `time`) lists them with their canvas positions and is `null` for any other layer. Above the
fields of an effect use, the inspector offers its
mask: none, a picture, another layer or a shape (rect, ellipse, ring, polygon, star or line, each
started at half the size of the layer), the channel, invert, a soft edge (a slider up to 200 px and
a field up to 10000) and the strength, all through `effects mask`; the same panel offers the mask of a layer itself (`layer.mask`, which takes the same fields; `effects mask` without `index`), with a thumbnail of what the mask looks like (the channel of the picture, the outline of the shape, the name of the masking layer). The canvas draws the mask while its use, the layer or the mask is selected, placed where the renderer draws the layer under its parents, groups, origin and parallax: the box of the layer in dashes; for a picture, the channel it reads (brightness, red, green, blue or alpha), inverted and weakened as the mask is, as a tinted picture over the layer, decoded to at most 512 pixels and kept as a PNG in the cache of the app (the 32 most recent kept); the masking layer framed; and for a shape, its outline filled where the effect applies (everything but the shape when `invert` is on), its feather range as two dashed outlines, one grown and one shrunk by `feather`, and handles: the corner that sizes it (the end of a line, which runs from −`to`/2 to `to`/2 around the middle), a dot past its edge that sets the feather, and one more for the width of a ring, the corner radius of a rect and the depth of the points of a star. The query `mask_view` (`layer`, optional `index`, `time`) lists all of it in canvas pixels, `mask_picture` (`asset`, `channel`, `invert`, `strength`, `tint`) returns the file of the tinted channel, and the action `effects mask_grip` (`layer`, optional `index`, `hold`: `size`, `feather`, `width`, `radius` or `inner`, `to`: a point of the canvas, optional `at`) writes what a handle does when it is dragged to `to`, in the pixels of the layer around its middle: `size` doubles the distance (each side of a rect, ellipse or ring, the larger of the two for a polygon or a star, `to` of a line), `feather` is the distance past the edge (across the line for a line), `width` and `radius` are measured inwards from the edge, and `inner` is the distance of the inner points over the outer radius; it is refused for a handle the shape does not have. Each drag is one gesture.
The outline adds `effect_names`, the `name` of each effect of
the package in the language of the desktop, and `effect_catalog`, the `id`, `category`, `description`
and cost of every built-in effect and effect of the package, the description in the language of the
desktop. *Add an effect* opens a dialog of that list grouped by category, searchable, each effect
with its cost next to its description; `wprs-ctl scene catalog DIR [--category CAT] [--json]` prints
the same list on a terminal, as a table or one JSON object per effect. Problems and refused actions show the message of the validator and of the editor, the
same English text that `wprs-ctl scene validate` and the commands print, under a translated
heading; each problem leads to its field, whose name is translated. A field that may follow a
source (keyframes, a user property, the sound analysis, the pointer or the media player) has
a menu to add or remove a keyframe at the current time, bind it, make it fixed again or edit the
binding itself (timeline, loop, scale, offset, clamp, smoothing). Follow the media player lists
only the `media` values that fit the field (the texts of the track for a text, the numbers and
`playing` for a number, `playing` for a switch, the colors of the cover for a color), each with
what it would show now with the simulated player of the Player tab; on a text that follows the
track, Text while nothing plays opens its `placeholder` with a row per language. An optional field that is set
can go back to its default.

The Player tab is a simulated media player for the preview, so that a scene can be made without
music playing: a switch turns it on, and it has the title, artist and album of a made-up track,
whether it plays, its position and length (sliders up to four hours, with a field in seconds for longer tracks, and times written as `m:ss` or, from one hour, `h:mm:ss` as the scene writes them) and its cover, picked among the pictures of the scene, with the five colors that the scene would
get from it (or from the placeholder) as swatches. Next track gives the made-up track another title
and the next picture as its cover, so that what waits for `media:track` and `media:cover` starts in
the preview, and the playing switch starts what waits for `media:play` or `media:pause`; the track
and its cover stay the same while only the position, length or playing change.
Scripts of the preview get the made-up track too, with their media events, and `wprs-ctl media
simulate OUTPUT FILE.json` shows one on any screen (`dbus.md`, `SimulateMedia`).
The preview then shows that track instead of what a media player plays, without asking for the
`media` grant and without reading the players; it reaches only the preview.

A web package has the same simulated player in its Player tab (switch, title, artist, album, a
cover picked among the pictures of the folder, next track, playing, position and length), which
feeds `window.wprs.onMedia` of the page in its Preview tab: the track goes in `preview.media` of
the `PreviewScene` document (`dbus.md`), so the page gets it, with the cover as a blob and the colors
of the cover, without the `media` grant and also when the page loads after the track started; it
reaches only the preview. Under it, *On a screen* still shows a made-up track from a JSON file
(Simulate from a file…) on the screen the package was last built and applied to (Stop ends it), the
same `SimulateMedia` call `wprs-ctl media simulate` makes; until the package is applied once, the
tab says so instead. `wprs-ctl scene preview DIR --media FILE.json` shows the track in the preview
of a page from a terminal.

The *Try* tab of the scene editor (and of a page package) shows the user properties of the manifest
with the same widget as the settings of a shown package, followed by the presets of the package. Choosing
a preset or changing a value sets `values` and `preset` in the look of the editor, and the preview
document carries the merged map in `preview.values` (`PreviewScene` in `dbus.md`): the daemon shows the
package with those values, over the defaults of its properties, instead of the values saved for the
package, and never writes them anywhere. Secret texts and buttons are left out. *Back to the defaults*
ends the trial. `wprs-ctl scene preview DIR --values FILE.json [--preset ID]` does the same from a
terminal; the file is a JSON object of property ids and values and `--preset` names a preset of the
manifest whose values the file then overrides. `rig` with `{"tried": {...}}` returns the form
(`fields`, `presets`, `values`) that the tab draws.

Test the pointer and clicks, a toggle of the canvas toolbar, turns the mouse over the preview into
a made-up pointer on it instead of an editing tool: while it is on, moving or pressing over the
preview calls `SimulateTouch` (`dbus.md`) with the position as fractions of the output and the
held buttons as a bitmask (1 left, 2 right, 4 middle), so `input.cursor`, `click:`, `enter:` and
`leave:` can be tried without a real pointer there; turning it off, or the pointer leaving the
preview, reports it left. `wprs-ctl input touch OUTPUT X,Y [--buttons N]` does the same on any
screen, real or `preview:`, and `--stop` ends it.

### Shapes, text and fonts

A `shape` layer shows the handles of its shape on the canvas while it is selected, placed where the
renderer draws the layer (under the parents, groups, anchor, origin and parallax) and listed by the
query `shape_handles` (`layer`, `time`): each handle has a `name`, a `kind` (`edge`, `corner`,
`round`, `count`, `depth`, `end` or `point`), its position in fractions of the box of the layer
(`u`) and on the canvas (`at`), and `gradient` has the ends of the gradient on the canvas. A `rect`
has `width`, `height`, `size` (both, with `even` the proportions are kept) and `radius`; an `ellipse`
and a `ring` the same without `radius`, and the ring `band`, its thickness; a `polygon` has `size`
and `sides`, which is the second vertex: dragging it around the shape gives the number of sides; a
`star` has `size`, `points`, the second tip, and `inner`, the depth of the points; a `line` has
`start`, `end` and `width`, and moving an end leaves the other where it is by moving `position`;
`stroke` sets the width of the outline of any shape but a line, and `gradient_from` and
`gradient_to` the ends of the gradient, which are the center and the edge of a radial one. The
handles that resize sit a little inside the box so that they do not cover the grips of the layer.
`layer figure shape` (`layer`, `handle`, `to`, optional `even`, `at`, `replace`) writes what the
handle does when it is dragged to `to`, a point of the canvas in pixels; it keeps the limits of the
scene (3 to 64 sides, 2 to 64 points, the depth from 0.01 to 1, a radius up to half the smaller side).
`layer figure stop` (`layer`, `stop`) edits the stops of the gradient: `{ "make": { "radial" } }`
adds one of two stops, `clear` removes it, `{ "add": { "at", "color" } }` adds a stop (the color of
the one that follows when none is given), `{ "move": { "index", "at" } }`, `{ "color": { "index",
"color" } }` and `{ "remove": { "index" } }`; stops stay sorted, between 2 and 4. The inspector shows
a bar with the gradient: clicking adds a stop, dragging moves it and every stop has a color and a
button to remove it.

The box of a `text` layer is the size the renderer draws (`wprs-type`, the typesetter both share):
the same fonts, wrapping, `line_height`, `letter_spacing`, `padding`, background padding and the
margin of the outline, shadow and blur, so that the handle box of the canvas matches the text. The
query `text_box` (`layer`, `time`) gives its `size`, the `lines` it takes, the `widest` line, the
`corners` and the `handle` at the right edge, and `layer figure text_width` (`layer`, `to`, `at`,
`replace`) sets `wrap.width` so that the right edge lands on `to` while the left edge stays (it
moves `position`); with `remove` it takes `wrap` away. A line break typed in the field of the text
(a multi-line box, Enter writes a new line) is part of the text and takes a line of the box.

`layer figure fonts` (`layer`, optional `font`, `fallback`) sets the font (an empty name takes it
away) and the list of fallback fonts of a text layer, at most 8; a letter the font lacks is drawn
with the first font of the list that has it. The query `font_list` (`layer`) gives the `system`
families installed, sorted, the generic ones, the fonts in the `package` (assets of font files), the
`most` fallback fonts and the `chain` of the layer, each name with its `source`: `package`, `system`,
`generic` or `missing`. The font picker lists them all, each written in its own font and with a
search field, and the list of fallback fonts is edited in place, reordered and warns about a font that
is neither in the package nor installed.

A value of a text that follows the clock or the date has a format. The binding menu of a text offers
the time and the date with ready formats and a sample of each (`HH:mm`, `hh:mm a`,
`EEEE d MMMM yyyy`…), and that of a number the time of the day, its hour, minute, second, weekday and
month. The field of the format shows what it reads now, from the same formatter the scene uses
(query `clock_sample`: `pattern`, `dates`, optional `wall`, the local time in seconds, and `lang`),
and a help with every piece and its sample (query `clock_help`: `tokens` and `numbers`) whose pieces
add themselves to the format when clicked.

## Package details

The manifest is edited in the same inspector: `/manifest` shows the fields of the JSON Schema of
manifests (`wprs-ctl scene schema --manifest`), generated by `schemars` from the types of
`wprs-format`, title first, then description, change notes, version, id, authors, license, content,
photosensitive, origin, credits, previews, links, tags, inputs, platforms, display, requires,
needed packages and default language, and after them the fields the order does not name (such as
`gpu`). `schema`, `kind`, `entry`, `preview`, `playback`, `web` and `permissions`, which
scenes do not use, and the user properties and presets, which have their own lists, are not shown.
`display` offers what applies to the kind of the package: `span` and `accent` for scenes, and
`fit`, `background`, `flip`, `zoom` or `position` only for the pictures and videos that use them;
a value already written there by hand stays visible so that it can be removed. A first-version
manifest shows only the fields that version knows (title, description, version, id, authors,
license, preview and tags). The default language has no way back to a default, since texts in
several languages need it. More annotations guide the widgets:

`accent` has its own control instead of raw JSON, in the inspector of the manifest of scenes and of
web pages: a box that chooses between the dominant color of the preview (no value), a fixed color
(the color chooser, written as `#rrggbb`) and the color of a property (a box with the color
properties of the package, written as `{"prop": "<id>"}`). It sends the `manifest` action `display`
with `values.accent`, or `clear: ["accent"]` for none, so `wprs-ctl scene act` and `scene manifest DIR
display --accent C|prop:ID` do the same. The action refuses a property that is not a color, a
rename of the property moves the accent along, and a property that the accent follows cannot be
removed until the accent changes.

The `playback` group of the manifest is shown for scenes too (their frame rate, volume and
muting; the speed, the part and the loop belong to videos). `playback.max_fps` has a control of its
own, a box with *Not set*, *Follow the screen* and the usual rates (10 to 240 frames a second) and
*Another rate…* for a number from 1 to 240. It sends the `manifest` action `playback` with
`values.max_fps` (`"display"` or a whole number), or `clear: ["max_fps"]` for none, so `wprs-ctl
scene act` and `scene manifest DIR playback` do the same; a rate of 0 or over 240 is refused. The
rate only lowers the ceiling of the scene: what the user or the battery sets still wins.

| Annotation | Meaning |
|------------|---------|
| `x-markup` | a text with the markup of `package.md`, edited on several lines with a view under each one that renders what is typed as the app does everywhere else (paragraphs, lists, bold, italic and links; HTML, images and remote content are not part of the markup, and the links of the view are shown but never followed) |
| `x-required` | required in `wallpaper-rs.wpak/2` (`content`, `photosensitive`, `origin`), so it has no way back to a default; a switch that is not set yet shows *No* and *Yes* |
| `x-computed` | filled by the tools (`requires.features`), shown read-only with translated names but for the features the author chooses |
| `x-ref` | `thumbnail` or `animated`: a file chooser that runs `thumbnail`; `language`: the languages of the texts and of the desktop, offered in a list by their names in the language of the desktop |

Texts (titles, descriptions, change notes, labels, help, options and preset labels) show one row
per language, a plain text under the default language. `translate` takes only a pointer whose
schema is a text and writes where the text lives: a plain text stays plain in the default language
or with an empty `language`, and another language turns it into a map with the plain text under
`default_locale` (set to `en` when missing); a map gets or loses that key, and the last text of a
title cannot go; an `"@key"` text writes `strings.<language>.<key>`. A text that does not exist yet
starts in the default language. An empty text removes that language, or an optional field when it
is plain. *Translate* in the app sends `add`, which never overwrites a text. A first-version
manifest only edits its plain texts and the maps it already has; texts in several languages wait for
`upgrade`. Lists of names (authors, tags, the paths of a credit) are edited one per line, lists of
choices (inputs, the reasons of a mature rating) as check boxes, and previews with a file chooser.

`thumbnail` copies a png, jpeg or webp of at most 1024 pixels a side and 1 MiB as it is, and turns
a larger picture, or a gif, into a jpeg of at most 1024 pixels as `assets/preview.jpg`, lowering
the quality (85, 75, 65) and then the side (768, 512 pixels) until it fits in 1 MiB, or refusing
the picture; an animated preview must already be a gif or webp of at most 640 pixels a side, 15 s
and 8 MiB. Files with the same name and other content are kept as `preview-2.jpg`, and `prune`
deletes the ones no longer named. A preview over its size points at its own field.

Credits have a panel of their own under `/manifest/credits`: the pictures, videos, sounds, fonts,
tables and models of the folder (previews excluded) with a box each, by default only the ones no
credit covers, a license (the common ones are offered, any SPDX expression is accepted), who made
them and an address, and *Credit the chosen files*, which sends `manifest` `credit`; every credit is
listed with a button that sends `uncredit`. The query `credits` (`wprs-ctl scene ask DIR credits`)
answers `assets` (`path`, `kind`, `size`, `credited`), the `credits` and the `missing` paths. A
package that already credits someone, or whose origin is derived or curated, gets a warning at
`/manifest/credits` in the problems list and in `pack check` while some of those files have no credit.

`/manifest/strings` has a table of its own for version 2 manifests: the languages (each with a switch
that keeps its texts in `locale/<language>.json`), every key with a box per language, the missing ones
in red with the summary at the top, the keys no text uses marked, a row to add a key with its first
text, and the texts of the engine (`wprs.*`, in English and Spanish) listed. Every text field has a
*Use a text by key* button that offers the keys of the table and the engine's and sets the field to
`@key`. The query `strings` (`wprs-ctl scene ask DIR strings`) answers the `languages`, where each lives
(`places`: `manifest` or `file`), the `keys` (`used`, `texts` by language, `missing` languages) and the
`engine` texts. The writes to a locale file happen when the action is accepted, after the manifest checks
pass, and are not part of undo (the manifest part of `locale` is).

The thumbnail and the animated preview of a scene can be captured instead of chosen from a file: *Capture
from the scene* next to the picture chooser draws the scene at the second the preview is on and keeps the
frame, and *Record a preview* records a few seconds from there as a gif; both run in the background
(the editor shows it busy), draw with `Renderer::stills` (one view of the scene for all the frames) and
store through the same path as `thumbnail`, so the size rules above apply. Packages that are not scenes
keep the file chooser.

The photosensitive switch has a *Measure the flashes* button that draws 8 s of the scene at 15 frames a
second, takes the brightness of a grid of 32 × 18 cells of every frame and feeds it to the `FlashGuard`
the daemon uses to calm scenes that flash (more than three times a second over a tenth of the picture).
When it flashes and `photosensitive` is not set, the problems list gets a warning at
`/manifest/photosensitive` (while the scene and its properties stay as they were measured), and
`wprs-ctl pack check` measures a scene that does not declare itself photosensitive and prints the same
warning. Sound, the pointer and the clock decide the real picture, so the measure is a hint, not a proof.

Every step of the history ends with the inputs and features the document needs, computed as
`wprs-ctl pack build` does: the inputs the scene uses are added to `inputs` (the ones an author
declares, such as `media` or `network`, stay), an `audio`, `pointer` or `clicks` input that the
scene used before the step and no longer uses goes, and `requires.features` becomes exactly the
computed list, keeping `requires.engine` and a `requires` the author added empty to set it. Undo
and redo restore both with the rest of the step, and setting,
adding, removing or moving anything under `/manifest/requires/features` is refused, except adding
or removing `locale_files`, which the author chooses (a check box under the computed list). The same step sets `permissions.rgb` of a scene while some layer has `rgb` on and drops it when none does,
whatever the edit (the `rgb` action, a `set`, adding, copying or removing a layer); a web page keeps the
switch of its preview. A folder
edited by hand may lack them: a missing input is an error and a missing feature a warning, and
`build` and `apply` first record the step `sync` that fills them and save. Problems also cover the
rules of `features::check` and its warnings (a declared `audio`, `pointer`, `clicks`, `media` or
`network` input that nothing uses), the verdict of `wprs-ctl pack check` for each platform that
refuses the package or plays it without a feature (a warning at `/manifest/requires/features`), a
script that declares nothing but uses more than the first interface (an error at its entry of
`/scene/scripts`), and previews over their size, format or length. Every field that version 2 requires and is missing is a
problem of its own, listed together. Locale files, effect shaders and `.glb` models are read again
only when their size or modification time changes, so editing a value never reads them.

A first-version manifest (`wallpaper-rs.wpak/1`) shows a notice with *Upgrade*. `upgrade` does what
`wprs-ctl pack upgrade` does, with the same code (`wprs_format::upgrade_json`), as one step: it sets
`schema` to `wallpaper-rs.wpak/2`, moves `preview` to `previews.thumbnail`, sets `default_locale`
when the texts are in several languages (`en` when all of them have English, or a language they all
share) and renames the property ids that version 2 no longer accepts (`snow-speed` becomes
`snow_speed`, `snow_speed_2` when taken), together with the bindings and triggers of the scene, the
groups, the maps and expressions of `visible_if` and `enabled_if` and the presets that name them. It
never guesses the rating, the photosensitivity or the origin: `rating`, `reasons`, `photosensitive`
and `derived_from` set them (the origin is original with a rating and no `derived_from`), and what
is not given stays as problems until the author sets it.

## User properties and presets

*User properties* in the *Package* tab lists the properties in the order of the settings form
(`wprs_scene::order`, by `order` and then id, each group followed by its members) and then the
presets, in one list that scrolls. *Add property* offers every kind (`add_property`) and sends no
label, so a new property shows its id until the author gives it one; the arrows move a property
among the ones of its group (`move_property`) and its menu renames it (`rename_property`), moves it
into a group, last, or out of it, just after its group, and removes it (`remove_property`). Renaming
or removing the property that the inspector shows moves the inspector to the new id, or to the
package details. The rename dialog says that scripts are not rewritten and names the scripts that
name the property in quotes; removing such a property asks first. A property shows the fields of the
JSON Schema of properties (`wprs-ctl scene schema --property`) for its kind, label, help, group,
order, icon and conditions first, guided by more annotations:

| Annotation | Meaning |
|------------|---------|
| `x-ref` | `group`: the groups of the manifest, named by their labels, and none for a group, since groups do not nest; `icon`: the icons of `scene.md`; `option`: the values of the options of the combo, named by their labels; `media`: the image and video assets |
| `x-condition` | `visible_if` and `enabled_if`, edited on one line |
| `x-color` | a color picker, with transparency only when `alpha` is set |
| `x-scalar` | a value typed as a number, `true`, `false` or a text |

Labels written as `"@key"` show the text of `strings` in the language of the desktop, else in the
default language, as the settings form does. The default of a slider is a slider from `min` to `max`
with its unit, in whole numbers when `integer` is set; `decimals` go from 0 to 6, `max_len` from 1
to 1024, `every` from 10 to 86400 seconds and `accept` offers `image` and `video` as check boxes.
*Add* under the options of a combo appends an option with an empty label and a value no other option
has. A condition is a JSON object, a map of values, or an expression of the language of `scene.md`;
every keystroke checks it as the validator does, so unknown ids, properties without a value,
comparisons of values of other kinds, conditions that are not true or false and conditions that
depend on themselves through other properties show under the field, and only a condition that
checks is recorded, with `condition`. A problem of a group or a condition points at that field of
the property, and a problem of a preset at its id or at the value it sets.

A slider, a combo or a flag shows *Values by quality level* in place of raw JSON: one row for Q0,
Q1, Q2 and Q3 with the widget of the property itself (the slider with its range and unit, the box
with the choices, the switch), *Default* taking a level out. Each edit is the plain `set` or `remove`
of `/manifest/properties/<id>/quality/<q0..q3>`, so `wprs-ctl scene act DIR set` does the same; the
other kinds of property do not change with the quality level and show no rows. Under the rows the
inspector lists what each level ends up using (the `levels` query of `rig`, `{"levels": {"property":
"<id>"}}`, which applies the same rule as the runtime: a value set for a level holds for the lower
levels until one of their own, and on a slider it is a ceiling that never raises the value the user
chose) next to the quality selector of the preview, the one of the effect costs, so that picking Q0
to Q3 runs the preview at that level, with the values of that level.

*Add preset* adds a preset without a label (`add_preset`) and shows it. A preset shows its id and
label and one row per property it may set (not files, folders, secret texts, buttons, groups, labels
or dividers), with the widget of the kind of the property: a row the preset sets shows its value,
the others the default of the property, and *Default* takes a value out of the preset.

A value that follows a combo or a switch with `{"prop"}`, such as the image of a layer, the texture
of a mask, of an effect or of particles, a video or a color, shows the property it follows, among
the properties that hold a value, and under *Map* one row per option (*Off* and *On* for a switch)
with the widget of the value itself, such as the list of images: the variants of `scene.md`. A row
the map lacks is empty, and the engine then reads the value of the property itself.

Adding, changing or removing properties recomputes `requires.features` like any other step:
`properties2` follows the newer kinds and fields of `scene.md` and a spelling the first version
cannot read, `folder` a folder and `user_videos` a file or folder that accepts videos.

### Mesh layers

The tools for mesh layers edit the file of the layer as a whole, whatever its number of parts, as
long as the scene draws it (at most 64 primitives, as for any mesh layer); a mesh drawn by several
nodes gets one of its own for each of them on the first edit. Bones are
the joints of its skins and the nodes without a mesh, except attachment points (`extras.point`);
the parts of a mesh without a skin are bound to the bones as soon as there is one. Each edit writes
one skin for every part, with the vertices
at their rest place in the layer and each bone bound where it rests, keeps the other contents of the
file (pictures, materials, blend shapes, animations, names) and drops what nothing uses any more.
The skin records in `extras.weights` whether its weights are `auto` (they follow the bones: adding,
moving or removing a bone binds every vertex again) or `painted` (they stay; a new bone starts with
no weight and a removed one leaves its weights to its parent). A file whose skin says nothing keeps
its weights, as painted. When the folder is saved, the files of earlier edits that the history
still names wait in the state folder of the editor (next to the saved history, outside the scene
folder) and come back when undo or redo names them again, and those that nothing names any more
are deleted, so the folder and the package keep only the meshes in use; a new file never takes the
name of one that waits.

The inspector of the `mesh` field shows the bones as a tree: a click picks a bone, also on the
preview, a double click or the pencil renames it, and a bone dropped on another hangs from it, or
from none when dropped below the list. *Place bones on the preview* drags their heads. *Paint
weights on the preview* paints the picked bone with the brush, its strength, size and soft edge,
keeps the bones that are ticked as locked, and colours the triangles of the mesh by the weight of
the picked bone (from blue for none to red for all of it, blended between the vertices) or by the bone
that leads each triangle, with its colour, drawn as one picture of at most 384 pixels a side that
is made again only after an edit; *Auto weights* runs `auto_weights`. The inspector also picks the clip the scene plays (`animations`)
and `hit` bones and `depth` with its scale and back-face hiding, as `set` on those fields, which
`wprs-ctl scene set` does as well. The blend shapes have sliders that only change the preview, and
an expression is saved from them, shown to be changed and saved again, renamed or removed.

*Make a chain on the preview* takes a click on the bone a chain starts from and one on the bone it
ends at, and adds it with `ik`. Each chain is listed with what it reaches for (the pointer, with a
note that it needs the pointer permission, a point, a bone or point of the mesh, or another layer),
a button that bends it the other way, its weight and a button that limits or frees each of its
bones. On the preview a dashed line joins the end of each chain to its target, a chain that reaches
for a point has a ring that is dragged to move it, and each limit is an arc around its bone from the
smallest to the largest angle, with a dot at each end that is dragged to change it. *Pick bones
that swing on the preview* adds or removes the body of the bone clicked, and each body is listed
with its mode, sliders for its stiffness, damping, inertia, gravity and wind, and ticks that bound
its turn and how far its head slides, each with a slider from its smallest to its largest value
(`free` when unticked); on the preview each body has a ring at its head, an arrow for its gravity
with a square at its tip that is dragged up or down to set it, and a bounded turn is an arc from
its smallest to its largest angle with a dot at each end that is dragged to change it. *Draw a rope on the preview* turns a drag
into a `physics` rope of up to 9 new bones that hangs from the picked bone, or the nearest one.
*Simulate* plays the preview with the physics, or holds it without (the preview drops the
`physics` of the layer, only there), and *Reset* sends the layer once without its bodies so that
they start again from the pose. The rules that hand a bone over to another parent are listed with
the bone it moves, a picker for its new parent and a weight slider, a button that removes one, and
pickers for the bone and parent of a new one, all with `bone_rule`. The wind has sliders for its angle, strength and gusts and *Show
on the preview* draws it as an arrow from the middle of the canvas whose tip is dragged; all of
them run `wind`. *Attach layers on the preview* lists the other layers as chips: one dragged onto a
bone of the mesh follows it, one dropped elsewhere on the mesh follows a new point there, and a
picked chip does the same with a click; what the layer inherits is ticked below, the layers that
follow the mesh are listed with a button that lets them go, and a dotted line joins each of them to
its bone or point, ending in a ring that is dragged to move the layer along it (`nudge`). The
attachment points of the mesh are listed with their name, which is edited in place, the bone they
hang from and a button that removes them; *Add a point* places one in the middle, and on the
preview each point is a square that is dragged to move it, all with `point`. The parts of a mesh of several parts are listed from back to front: a part is
dragged by its handle to a new place, which writes the `order` of every part with `parts`, ticked
to be shown, and given the part it only shows over; the picked part and the part it is cut to are
outlined on the preview.

In the inspector every field that names a bone, a point, a part or a blend shape offers those of
its mesh layer: a single bone (`physics`, `rules`, the `attach` of a layer, which names the bones
and points of the layer it follows) or part (`clip_to`) is a list to pick from, a list of bones
(`chain`, the `bones` of animation layers, `hit`) a set of ticks in the order of the skeleton, a
map keyed by bone or part (`limits`, the `bones` of clips, `parts`, `shapes`) a list to add a key
from with its fields below, and the `target` of a chain a list of the pointer, the bones and
points, the other layers and a point. The schema marks these fields with `x-ref` set to `bone`,
`point`, `part`, `shape` or `reach`; the names come from the file of the mesh, read again only when
it changes.

*Edit clips on the timeline* swaps the timelines under the preview for the clips of the layer: a
clip is picked, added, renamed or removed, and given its length and loop; the playhead scrubs,
plays and loops the preview, which plays that clip alone with the sliders of the blend shapes. The
rows are the channels of the bones that have keys and of the picked bone, the order of the parts
that have keys and every blend shape with a slider that keys its weight at the playhead; a key is
picked, dragged to move it (with Ctrl to copy it), given any easing of the scene format, bent with the same two Bézier handles as the keyframes of a timeline (the curve button opens the plot of the curve to the next key of its row; the same applies to the rows of a 3D model) or removed. On the preview the
picked bone has handles to move, turn and scale it at the playhead, in the space of its parent as
the clip keys it, and *Key* writes, with `key`, the channels the handles changed, or the whole pose
of the bone at the playhead when none did. The timeline shows the seconds from the start of the
clip, its `from`, and the keys are written where the playhead is. Only the preview sees
the clip and the sliders chosen here; the scene keeps its own `animations` and `shapes`.

### Particle systems

The *Particle systems* button under the layers list selects `/scene/particles`, and the inspector
shows the systems panel: every system of the scene, with or without a layer, in a field that renames
it, with buttons to edit it, copy it (`create` with `from`), add a system that it starts (`create`
with `parent`), and remove it. A rename that leaves scripts naming the old name shows a warning with
each `file:line` and the button *Rewrite the exact names*, which runs `scripts`. The panel also shows
under a selected system. `scene particles DIR SYSTEM rename NEW` prints the same warning on stderr,
with the action to run: `scene act DIR particles '{"scripts":{"from":"OLD","to":"NEW"}}'`. The query
`{"script_uses": {"name": ID}}` of `rig` lists the `path` and `line` where a script names an id.
Each emitter section has *Most particles started in one step* (`pace` for that emitter) next to its
rate, bursts, timeline, pointer, sound gate and inherit controls.

Selecting a particle system, or a particles layer, draws its tools over the preview in pixels of
the canvas, through the transform of the layer: every emitter shape with handles (the point; the
corners of a box and the center and radius of a circle, around the control point they are anchored to;
the ends of a line; for `around`, the center, a handle for the radius and one for the end of the
arc, with a dot at each place a birth takes; for `between`, its two ends, a handle that bows the
line and one that slides the widest point of the taper, with dots sized as the taper sizes them;
for `layer`, the outline of the layer and the pixels above the threshold of an image, mesh, text
or group layer, or only its outline where none can be read (a model layer, or dynamic text such as
a clock), the control points `p0` to `p7` as markers to drag, and a ring
whose handle sets the focal length of the perspective. A turbulent start draws, from the middle of
the first emitter, a fan of the directions the noise may give, the exact directions of the first
particles born now with the seed the preview runs (`clock.seed`, or while it is `null` one seed the
editor picks for its session and gives the preview, shown by the controls and printed as `seed` by
`handles`; *Keep this seed* writes it to `clock.seed`, and while the start follows a sound band a
slider sets how loud the band is for the rays, which turn with it as the engine turns them) and an arrow with a handle that turns its direction and one on the edge of the fan that widens
or narrows it; a fractal offset draws arrows from a grid of birth places to where the noise moves
them, computed as the engine does, and a ring with a handle for the distance. An end of `around` or `between` that names a
control point moves that point. The inspector of the system lists its emitters with a picker of
the shape, of the control points or the pointer that `around` and `between` follow and of their
count; a picker of the layers it may emit from (never one that holds the system), with the
threshold, its color, its motion and when it is read again; the control points with a toggle to
follow the pointer and a dial for their angle; the remaps of the starting values, each with its
input and output, two-handle ranges for `in` and `out`, its clamps, operation and wave, and the
curve of the output over the input; a cutout with a two-handle range, its opacity and the alpha
ramp it gives; toggles and sliders for the direction, turn, noise speed, spread, phase and speed of
a turbulent start with a picker of the sound band that moves it, and for the distance, size,
detail, scatter, drift and sides of a fractal offset; the orientation with a dial for its angle; the perspective with its focal length
and two-handle ranges for the depth and its speed; what each child takes from its parent particle;
and the particle shaders of the package with a warning that the engine checks them and may leave
them out. The curve is a component of its own that plots any `{ points, x, y }` the editor computes,
so other tools can draw their curves with it. `wprs-ctl scene particles DIR SYSTEM handles` prints
the same data: the transform of the layer and its inverse, the points, each emitter with its
handles, the curves and the lists the pickers offer.

The preview also draws the colliders of the system, colored by their response (red where particles
die, green where they stick): the border of the canvas, a plane as a line with a handle on its
point and one that turns the side particles stay on, a circle with handles on its center and radius,
a box with handles on two corners and a segment with handles on its ends, dashed when they keep
particles inside, a layer as its outline with its field of distances over it (computed from the
picture of an image layer and from the pixels of a text, a mesh texture or the children of a group,
which a group shows approximately) and a mask as the canvas rectangle with the field of distances of its
picture stretched over it (`colliders` of the query: `field` is a list of `[x, y, distance]`, negative
inside, and `approximate` is true when nothing could be read). A force that keeps a distance
draws a ring of that distance around its target, with a handle on the target and one on the ring;
one that keeps particles between two points draws the line between them with a handle on each end;
an `attractor`, a `vortex`, an `orbit` and a `slow_near` (the `pulls` of the query: `index`, `kind`,
`field`, `ref`, `at`, `falloff` and `rings` of `{key, value, px, at}`) draw a dashed ring for each reach they
have (`radius`, `inner`, `outer` and the `ring` of a vortex) with a handle on its top, which sets
it, and a handle on their center or target; and an end or a target that names a control point
moves that point (the pointer has no handle). The steering section of the inspector adds any of
these forces and lists them with the point they follow, their strength (the speed a `slow_near`
keeps), a picker of the `falloff` and a slider for each reach. The inspector lists the
colliders with a picker of the response (bounce, slide, stop, stick or kill) and the bounce and
friction it uses, whether a circle or a box keeps particles inside, a picker of the layers it may
collide with and, for a picture mask, its picture and the opacity from which it is solid; the kept distances with pickers of their targets (a control point, the pointer or a fixed point whose X and Y are typed in pixels of the system, the same picker for the `from`, `to`, `target` and `center` of every force) and sliders for the distance and the strength; a toggle and sliders for the neighbour radius,
the separation, the alignment, the cohesion and the most speed of a flock; the remap operators with
the same editor as the remaps of the starting values and `age` as one more input; and the rope and
the trail of the render with their fields, live on the preview, and a picker of the texture among
the image assets. When a rope is on and an emitter may start more than one particle a step, a note
offers to set `max_per_step` to 1 on every emitter, which keeps the rope even; `wprs-ctl scene
particles DIR SYSTEM rope` prints the same advice.

*Render settings for* picks whose render the sections below edit: the system, for every layer that
draws it, or one of those layers. With a layer picked, the rope, the trail, the texture, the
cutout, the orientation, the shader, the light, the refraction, the colours and sizes over life, the
frames, the drawing order and the other render fields are written to the render of that layer (the
first change starts it as a copy of the render of the system), the panel shows what that layer's
render has now and lists the values that differ from the system's, and *Use the render of the system
again* drops it (`particles` `scope`, reached with `wprs-ctl scene act`). Without a layer picked the
panel shows the render of the system even when a layer draws it with its own.

The particle budget of the inspector reads the real numbers of the running preview when there is
one: `GetParticleStats` of the daemon for the output of the preview gives the particles alive, the
slots, the path that simulates them (graphics card or processor) and why, and the quality and card
class the preview draws at, refreshed every two seconds and after each edit. Only when no preview
runs does the panel show the count of its own simulation, labelled as an estimate; the box that
shows the particles over the preview always uses that simulation.

The particle budget of the inspector simulates the system on the processor up to the time of the
timeline (at most 10 seconds), as the CPU path of the engine does, and tells how many particles live
of its capacity, the slots it keeps on the GPU and on the processor for each device tier (phones and
older computers fall on the two lower ones) with the trail points, neighbours of a flock and rope
segments each keeps, and what each quality level keeps of births, neighbours, trail points and rope
points between particles. When asked, the simulated particles, the rope and a few trails are drawn
over the preview, and clicking a particle draws the neighbour radius of the flock around it and
marks its neighbours. A picker of the quality level, Q0 to Q3, caps the quality of the preview, and
*Restart systems* starts only the particle systems of the preview again, keeping the scene and the
rest of its state; `wprs-ctl scene preview DIR --quality Q --restart` does the same, and `wprs-ctl scene particles DIR SYSTEM budget` prints the
budget as JSON.


The systems panel has a section with the settings of the system, all through `settings`: the most particles alive (`capacity`, up to 200 000), the `seed` typed, picked at random or taken from the preview, the seconds simulated before it shows (`prewarm`) and whether particles move with the layer or stay where they were born (`space`).

Each emitter of the inspector has its rate (a live slider through `settings` with the `emitter`; a
rate that follows keyframes or a link shows it and offers *Fixed rate*, which replaces it) and an
emission lane, from 0 to at least 10 s: the delay, the end of the `duration`, the on and off spans
of `periodic` (the shortest on and off spans in a top row and the longest in a bottom row) and the
the marks of every timed burst (a dot at its `at`, a square at each repetition), with handles that drag the delay, the duration, the length of the on
and off spans (keeping the width of their ranges), the `at` of each burst and the `every` of each repeating one, each drag one gesture,
the scale of the lane held while a handle is dragged; below it the list of bursts (add, remove, size, whether each fires on time, the beat or a
click, and its `at` and `every`), following the pointer, `audio_gate` and `inherit`, all through `emit`. *Drawing order and what layers may change* picks
the render `order` and `cycles` and ticks the values particle layers may change (the rest is `lock`).
A selected particles layer shows its `instance`: a slider for each number, a tint and a reset for
each to the value of the system, and *Draw everything as the system does* (`clear`); with a layer
selected, the control points of the canvas are those of the layer (`particles` with `layer`
answers them with `own`) and dragging one writes only that point in the `points` of its `instance`;
the other points keep following the system, its keyframes and its links. A layer's copy of a
`world` point is drawn in canvas pixels, where its particles feel it.
### Lights

The Add layer menu offers Light with its four types. Every light shows over the preview as a marker
to drag, and the selected one draws its handles, in pixels of the canvas through its parents: a ring
at its radius with a handle on it, a square above it whose distance sets the height `z` (half a pixel
of the preview for each pixel of height), for spot lights a handle that turns it and the edges of the
outer and inner cone with a handle on each, for tube lights a handle on each end that moves, turns and
stretches it, and for directional lights an arrow with a handle that turns it. A check box draws the
box a moving light redraws. The marker moves the light with the same `grip` as the layer handles
(through its anchor and the skew of its parents, with snapping), and the aim and tilt handles and the
tilt slider send `light set` with the time of the playhead and the overwrite switch, so a keyed
position, rotation or tilt gets a key at the current time; the other handles change the fields of the
light with `set`, so a light whose fields have keyframes is keyed at the current time, and lights go
on the timeline like other layers.
The inspector of a light picks its type and color, slides its intensity, radius, height, length,
cone and, for spot and directional lights, its tilt toward the canvas (also a handle on the aim line,
nearer the light the more it faces the canvas), plots its light against the distance with the curve component and toggles solo and mute.

Selecting the canvas lists every light with its type and solo and mute toggles, how many lights the
scene has against the 12 it may have and how many each device tier keeps (8 on T0 and T1), the
pickers of the light around and from above with a sphere lit by them, and the preview quality picker
of the particle budget, which here shows for each level whether the lighting is worked out once (Q0)
or how many pixel lights it shades each frame while a light moves.

The inspector of an image, video or text layer has a Lighting part (a particles layer has its switch
and the glow and strength sliders): a switch that writes or removes
`lighting`, pickers of the normal map, the reflected picture and the roughness and glow maps, sliders
for roughness, reflection, glow and strength, a note when a map is read as colors, Generate normals (an image, video or text layer), a dialog with
sliders for strength and blur that shows the relief of the map lit from the top left, the map itself
and a lit sphere before it keeps the map, and Paint roughness and Paint glow, which paint on the layer
in the preview with a brush size, hardness and gray while the map shows over it as a heat view.
`wprs-ctl scene lights`, `light … falloff`, `normals --preview`, `paint-map --heat` and
`ambient --preview` print the same data.

A point or spot light has a Light the air part: a switch for `volumetric`, sliders for density,
samples and shafts, the glow plotted against the distance (with shafts, a dashed line shows the glow over a part of the layers below darker than around the light), and for each quality level whether the air is off
(Q0), worked out at a quarter of the size with half the samples (Q1) or at half the size (Q2 and Q3),
with a note that phones work at a quarter; the preview shows the glow live at the picked level, and
the selected light shades the ring of the lit air. A spot light has a Projects picker that lists the
pictures and videos of the scene and the layers it may project without a loop, with a thumbnail of
the picture; the preview draws what the light projects inside the cone where it lands on the
canvas, with a dashed outline: the picture, the frame of the video at the playhead (from the cached
strip of 16 frames) or, for a projected image, video or text layer, its picture, frame or letters (other
layer kinds show only the outline). When part of the beam runs along the canvas the footprint is drawn
out to the radius of the light, fainter, with a note. The list of lights marks lights that light the air or project, and the cost of each
quality level adds the samples of the air, the pixels of the layers below that the shafts copy (`shaft_copy`, at the size of the air; once while they stay still, each frame while one moves), the lights shaded on particles and the pixels refraction
reads, with the same rows for phones.

The particle tools have a Light part: Take the lights of the scene, a normal map picker and a preview
of a sprite lit from the front and from behind, and Bend what is behind the particles with a normal
map picker, an amount slider, the displacement map, the shift each quality level keeps and a switch
that draws the displacement over each particle of the processor simulation on the preview. The
particle budget lists how many lights light the particles and whether they refract at each level.
`wprs-ctl scene light … volumetric --show`, `projection --show`, `particles … lit --show`,
`refract --show` and `lights --cost` print the same data.

### Models and 3D

The Add layer menu offers 3D model (once the scene has a `.glb` asset), 3D group and 3D model from a
file, a dialog that reads a binary glTF file before it is imported and tells its triangles,
materials, textures, bones and clips, and for each device tier whether it fits, the largest texture
it gets (2048 px on phones and on T0 and T1) and the texture memory, with a note on converting FBX and
OBJ with Blender; accepting imports the file and adds a model layer, inside the selected group.

A scene with models draws over the preview an authoring view with its own camera, kept in
`ext.editor.view3d`, which only the preview reads: dragging with the right button turns it around its
target, with the middle button or Shift slides it and the wheel brings it closer, and the preview
renders through it while the scene camera shows as a dashed frustum with a handle on its eye, a square
on the point it looks at, both moved in the plane of the view, and a line toward its up. Each model
shows its box, which selects it with a click; the selected model shows three arrows along x, y and z
that move it along that direction, a knob above it that turns it (sideways around y, up and down
around x) and a square on its box corner that scales it. These handles start from the values the
model has at the current time and write `position`, `rotation` and `scale`: a keyed value gets a key at
the current time, a value bound to a property or a script keeps its binding and its handle is dimmed,
and `scene3d` `model` `set` refuses to write over keys or a binding.

The inspector of a model layer shows the counts and the tier verdicts of its file, a Reflects picker of
the pictures of the scene (`env`) with a thumbnail, Look at it in the authoring view, and a Materials
part with a row per material of the file: a ball shaded with the material and the light around and from
above of the scene (as in the light tools), the shader picker (physical, unlit or a package effect),
lit, both sides, casts and takes shadows, tint and glow colors, sliders for metal, roughness, relief and
occlusion, and the thumbnails of the maps of the file (color, metal and roughness, normals, occlusion,
glow). The inspector of a group turns it 3D, and the inspector of a flat layer inside a 3D group
toggles Hide behind nearer models (`depth_test`).

Selecting the canvas also shows the 3D camera: sliders for the field of view and the roll that change
the preview while they move, where it looks from and toward, Set the camera from the view, Look
through the camera and Back to the camera view; and Bright colors: a switch for `hdr`, the tone map
picker and a Before and after slider that keeps `ext.editor.tonemap_split`, so the preview shows the
part on the left without the tone map. The quality picker of the lights panel lists for each level the
triangles, the 3D groups, the work units of the models and whether reflections of pictures are drawn,
and for each tier the quality it starts at, whether the models fit and their texture memory.
`wprs-ctl scene model`, `group3d`, `material`, `camera3d`, `view3d`, `tonemap`, `model-info` and
`cost3d` do the same from the terminal, and `group3d … show` prints the handles the preview draws.

The inspector of a model with a skeleton or clips adds an Animation part: its clips with their length and
animation, On the timeline (the clip timeline of meshes, whose rows for a model key `angles` and `shift`
of the chosen bone and the weights of its blend shapes, and which shows the events of the clip as marks
that seek to them), New clip from the file with the animations of the file and their length, Root motion,
Swinging bones (a bone picker and the presets, with the bodies the layer has), a slider per blend shape
for its weight, and Animation layers: per layer the clip it plays, On, Adds, move up, remove, sliders
for weight, speed, fade in and fade out, and a bone mask picker (Only these bones); the same Animation
layers part shows under the bones of a mesh. In the authoring view the selected model draws its bones
(a line from each bone to its parent and a knob that picks the bone for the timeline and the pickers)
and, with root motion, the path its first clip walks over two turns. The picked bone shows a ring: dragging
the ring turns the bone around the view axis, dragging inside it tips the bone like a trackball, and a
square in its middle moves it in the plane of the view; each drag keys `angles` or `shift` of that bone
in the chosen clip at the playhead (a model without clips gets `clip_1`). A swinging bone draws the fan
its rotation bound allows in its own plane, with Least and Most knobs that drag the bound, and under
Swinging bones each body shows its wind slider, the bounds of its turn and slide and Show the wind on
the preview, which draws the arrow of the wind of the scene. `rigs::query` answers `bone3d` (`layer`,
`bone`, `clip`, `time`, `clip_time`, `from` with the values the drag started at, and `turn` in degrees,
`tilt` `[x, y]` in degrees or `slide` `[x, y]` in pixels) with the `angles` and `shift` to key, and
`bound3d` (`layer`, `bone`, `point`) with the degrees of the bound under a point;
`wprs-ctl scene key … --angles X,Y,Z --shift X,Y,Z` and `scene physics … add BONE --rotation MIN,MAX`
do the same from the terminal.

A material row picks Fur, Foliage or Chroma key besides Physical and Unlit and the material shaders of
the package: fur shows sliders for shells, length and density, the shells each quality level draws and
a comb pad (dragged across for x and y, a slider toward the viewer); foliage shows sway and speed, the
wind of the canvas with its arrow, and Show the sway mask of the vertex colors; chroma shows a key color
picker, tolerance and feather sliders and Show the matte. A shader of the package shows a warning that it
runs on the graphics card within the limits of package effects. Under the materials, New material shader
writes a template, and every material shader of the package shows whether it builds, or its error with
the line and column of its file.

The lights panel shows Shadows: per point, spot or directional light a switch, the size of its map
(automatic or 256 to 4 096) with the size each tier draws, on phones and at Q0, and a bias slider, and
Show where shadows fall, which draws lit places white and shadowed ones red in the preview and the
dashed frusta of the maps in the authoring view. The 3D camera panel shows Camera paths: per path its
length, Loop, the easing, remove, its keys (a click removes one) and Key the view at the current time,
Add from the view, the order, the fade with its length, and Fly through the paths from the camera, which
looks through the scene camera and plays from the start. In the authoring view each path draws its
curve, a knob on each key that moves the eye of that key and a square on the point it looks at, both
moved in the plane of the view at their own depth, a dashed line between them, and a square where the
paths put the camera at the current time. The scene timeline shows a lane per path with its keys, which
drag to another time (`key` with `from`, `camera-path … key PATH T --from T` in the terminal) and seek
the preview with a click; the panel of the timelines takes the presses that no lane takes, because the
Breeze style turns a drag that nothing accepts into a move of the whole window, which kept the keys of a
lane from being dragged. The cost lines add the shadow maps, the fur shells and
the skinned vertices of each quality level and the shadowed lights of each tier. `wprs-ctl scene
animate`, `clip`, `key`, `material`, `shadows`, `camera-path`, `new-material`, `debug3d` and `cost3d`
do the same from the terminal.

## Preview

The app asks the daemon for a virtual output named `preview:<n>` with `RequestStream` and shows its
PipeWire stream on the canvas, as the Plasma wallpaper does. Every change sends the unsaved document
with `PreviewScene` (`dbus.md`), at most once per 50 ms, with `solo` applied; a paused or scrubbed
timeline holds the scene at that second, so a still preview renders only when something changes. A
new document replaces the preview without a crossfade, and a playing preview keeps its time; package
details that are still incomplete, such as a missing rating, do not stop it. *Show on the desktop*
previews on every screen the same way. The app repeats the call every 30 seconds while a preview is
on and ends it when the editor closes; the daemon ends it 60 seconds after the last call. A
minimized window marks the preview output covered. `wprs-ctl scene preview DIR [-o OUTPUT] [--time
T] [--quality Q] [--restart]` shows the saved folder for 60 seconds, at most at that quality level and
started again, and `--stop` ends it; `--media FILE` shows a made-up track from a JSON file of at
most 64 KiB (`title`, `artist`, `album`, `album_artist`, `subtitle`, `genres`, the last three
words or lists of words, `playing`, `position` and `duration` in seconds from 0 to 10 000 000 and
`rate` from −100 to 100, `cover`, a picture inside the folder; texts of at most 512 characters
without control characters) instead of a
media player. The document of a preview may carry `preview: { quality, restart, media, battery, calm }`, which the
daemon reads only for previews; `media` is refused, naming the field, when a value is out of
range, and never reads the media players or needs the `media` grant. `battery` and `calm` are
booleans: the preview then runs as the daemon runs a scene on battery (a lower level, which
takes Q3 to Q2 whatever the automatic choice would be, and at most 60 frames a second) or with
reduced motion (parallax and camera shake stand still, as the daemon does with the desktop
setting). `wprs-ctl scene preview DIR [--battery] [--calm]` sets them.

A preview document that carries `scene.ext.editor.probe` (`{pointer: [x, y], tilt: [x, y]}`, the
pointer as fractions of the canvas from 0 to 1 and the tilt of a phone from −1 to 1, either
part optional) feeds the scene a pointer that rests there and that tilt, so layers with
parallax depth sit in the picture where the canvas outlines put them; out-of-range values are
clamped, and the daemon reads it only for previews. The scene runtime takes the tilt as a
device tilt, so `canvas.parallax.gyro` follows it.

*Preview level*, a button of the canvas toolbar, opens the panel that sets what the preview
shows. It has the same quality selector as the cost panels (Q0, Q1, Q2, Q3 and Restart), *On
battery* and *Reduced motion*, and every change sends the preview again. Under them the panel
answers the editor query `{"level": {"quality", "battery", "calm"}}` (`rigs::query`,
`levels::query`): `quality` is the level the daemon reaches (`asked` is the cap that was
chosen), `render_scale`, `density` and `particles` are what that level keeps of the resolution,
the crowd and the particles, `bloom` says whether the canvas has a glow (`on`), whether it has a
glare above white (`hdr`) and whether this level draws it (`glare`: Q3, or Q2 off battery),
`effects` has one row per effect use with its `layer`, `effect`, whether it is `kept`, its
`scale`, its `units` of work at this level (null when it is left out), the `full_units` it costs
at Q3, the `levels` of all four levels as the effect-cost panel shows them and `follows` when it
reacts to the pointer, `pointer.emitters` are the particle systems that follow the pointer,
`left_out` counts the effects left out and `units` adds up the work that stays. The panel
writes the same rows, in bold when an effect is left out, and names what follows the pointer so
that it can be tried with *Test the pointer and clicks*. The cost panel of an effect marks the
level the daemon would reach, not the one that was asked for.

While *Test the pointer and clicks* is on, the preview shows a ring where the pointer is, red
while a button is held, and every press sends out a pulse from the point it fell on, so that the
author sees where a click reached the scene.

*Compare with* picks a second level and *Without the glow of the canvas* the same frame with the
bloom taken out (`look.bloom: false` leaves `canvas.bloom` out of the previewed document, never
out of the saved one). Holding *Hold to see the other side* shows that second look in the same
preview and letting go returns to the first, so two levels, or the glow on and off, can be
told apart on the same frame; *Split the picture with the other side* in the panel writes both
looks into one picture, the first on the left and the second on the right.

*Save a picture…* and *Save a clip…* render the scene to a file without the desktop, from the
unsaved document, with the same code as `wprs-ctl scene render` (`Renderer::rehearse` of
`wprs-render`): the scene is drawn at the level, with the render scale of that level, battery and
reduced motion that the panel shows, from the second the playhead is at. A picture is one
frame (.png, .jpg, .webp or .avif); a clip is 90 frames at 30 a second, written as an animated
.gif or, for any other extension, as an image sequence named `NAME-0001.EXT`. *Move a pointer
across and click* adds a made-up pointer that crosses the canvas and presses over the middle of
the clip, so that what follows the pointer moves in the file too. The app tells where the files
went with the `rendered` signal.

`wprs-ctl scene render DIR --out FILE [--time S] [--quality Q] [--battery] [--calm] [--frames N
--fps F] [--size PX] [--pointer X,Y[:X,Y] [--click]] [--against Q] [--without-bloom] [--split
F]` does the same from the saved folder: a picture for one frame, an animated .gif or an image
sequence for more (up to 600 frames), a made-up pointer at fractions of the canvas that moves to
the second pair over the clip and presses over its middle with `--click`, and with `--against`
or `--without-bloom` the right of the picture from that level or without the glow, joined at
`--split` (0 to 1) of the width. It prints the files it wrote and, on the error output, the
effects the renderer left out. `wprs-ctl scene bench PACKAGE --quality Q --shot FILE` gives the
frame of a built package at a level together with its timings.

The lines its scripts pass to `console.log` and the kind, and the error of a script or layer that
stopped, are part of `GetState().outputs[OUTPUT]` (`console`, `error`); the Problems panel shows
them under Script console while the preview is on, and `wprs-ctl scene preview DIR --console`
prints them as they change, along with the error, until stopped (Ctrl-C, or another `scene
preview` on the same output).

When a script `uses` an input such as `pointer` or `audio` that the preview output was not
granted, the Problems panel warns of it next to the Script console, and `wprs-ctl state` on a
terminal prints the same warning next to the output, from `Inputs1.GetInputs().shown[OUTPUT]`
(its booleans against `granted`); it prints the stopped-module error there too.

The canvas draws the outline of each layer where the scene draws it, with its parents, groups,
anchor, origin, skew, parallax and 2D camera, from the editor query `seats::seats`, which places
every layer with the same code as the renderer (`Runtime::seats` of `wprs-scene`: the frame its
position lives in and its whole transform); groups get the box around their layers. The selected
layer gets handles: the corners scale it from its pivot (Shift keeps the proportions), the middles
of the edges stretch one side or, with Ctrl, skew it along that edge, a round handle above turns
it, the dot of the pivot moves its origin without moving the layer, and a depth gizmo drawn in the
perspective of the scene tilts and lifts it: a dashed ring around the pivot in the plane of the layer,
the upright axis whose end tilts it around the width (`transform.tilt[0]`), the level axis whose end
tilts it around the height (`tilt[1]`) and a line out of the layer from the middle of its bottom edge,
along the direction `z` moves it, whose diamond lifts it (`transform.z`). The gizmo comes from
`seats::depth` (the outline field `Gizmo.depth`: `pivot`, `ring`, `height`, `width`, `foot`, `lift`
and `reach`, its length in canvas pixels, 0.35 of the shorter side of the layer, from 40 to 400),
which projects every point with the plane the renderer leans the layer by (`Seated.plane` of
`Runtime::seats`, `Plane::lean`, the same code as the scene draws with). The `tilt` grip (`{ "tilt":
{ "axis": 0 } }` or `1`) picks the angle, −89 to 89 degrees, that puts the end of that axis nearest to
the pointer, the one closest to the current angle when two are as near, and `lift` the depth that puts
the end of the line nearest to it. Each handle sends the `grip` of the `layer` action with the
point it is dragged to, so the corner or edge it holds stays under the pointer whatever the
parents do. With *Magnets* (on by default) a dragged layer snaps to the edges and the middle of the
canvas and of the other layers, showing a guide where it snaps, and Alt places it freely; the arrow
keys nudge the selected layer by a pixel, by ten with Shift, as one step per press. The canvas takes the keyboard focus (the `SceneFocus` signal) when a layer is picked in the layer list, when a layer is tapped or dragged and when the empty canvas is tapped, so the arrows work at once; a change of the selection that is not one of those gestures leaves the focus where it is. *Fit to the
canvas* covers the canvas with it, fits it inside or stretches it (`fit`). *Follow a layer* picks
its `parent` with a tap on another layer of the canvas, keeping it where it is (`parent` with
`keep`), and *Stop following* removes it the same way; while a layer follows another a dashed line
joins their pivots, and the list of the `parent` field of the inspector only offers the layers it
may follow, without the ones that would close a loop or pass 8 layers (`seats::parents`). Other
tools of the canvas build on the same place: `Seat::at` takes a point of the box of a layer, in
fractions, to the canvas, and `Seat::fraction` brings it back; the overlay `SceneSeat.qml` does the
same for a layer named by id and sends the grips. A tilted or lifted layer, or one under a tilted
parent, also gets a dashed outline where the scene projects it (`Gizmo.corners`, through the same
perspective as the scene), and its handles sit where the scene shows them (`Gizmo.handles`: the
projected corners, middles of the edges and pivot). The grips bring the pointer back onto the layer:
`turn`, `scale`, `skew` and `size` onto the plane of the layer (`Seat::unlean`), and `move` and
`origin` onto the plane the pivot moves in, the layer at its depth without its own tilt
(`Seat::unlift`), so whatever they hold lands exactly under the pointer. Taps, drags and the hover of a layer take only the points inside its outline (the same polygon the canvas draws, so a skewed layer is hit exactly as drawn), not the rectangle around it. The *2D camera* button of the toolbar adds the camera of the
scene (`canvas.camera`) or removes it, and switches between *View through the camera*, where the
canvas and every handle show the scene as the screen does (`seats::seats` places the layers through
the camera), and *View the whole canvas with the camera frame*, where the preview and the handles
leave the camera out and the frame of what the camera shows is drawn over the canvas, from the outline
field `camera2d` (`seats::lens::frame`: `center`, `zoom`, `rotation`, `corners`, the point `top` of
the knob that turns it, and `whole`; `null` without a camera), computed from the camera the renderer
itself uses (`Runtime::lens` of `wprs-scene`) without its shake. The frame has a knob in the middle
that moves the camera, one per corner that zooms it and one above that turns it, each through the
`camera_grip` of the `canvas` action, so a keyed camera gets a key at the playhead or, with
*Overwrite*, its plain value. A script that moves the camera through `scene.camera` still moves the
preview in either view. The inspector of a layer shows its parallax depth (`parallax`) with a slider per side, across and up
and down, or one for both while *Same depth across and up and down* is on, and a *Parallax range* of
±1, ±3 or ±10 that widens by itself to hold the stored value; each drag is one step of the `depth` of
the `layer` action. While the canvas has parallax, *Try the parallax* in the toolbar puts a simulated
pointer on the canvas (the outline field `probe`), and, with *Follow the tilt of a phone*, a square
whose place from the middle of the canvas is the tilt; dragging them sends `probe` and the outlines and
handles of every layer shift by its depth, and the grips keep working on the shifted layers. The
preview stream itself still follows the real pointer. The toolbar also offers
guides for 16:9, 21:9, 32:9, 4:3, 16:10 and 9:16 screens. *Screens* previews the scene spread over
several screens: two or three side by side, two stacked, or the user's own layout as the daemon
reports it (`GetState().outputs[x].mm`). It frames the part of the canvas that each screen shows
and darkens what the frames between them (`layout.bezel_mm`) and the edges of the layout hide, and
its *Spread over every screen* box sets or removes `display.span: "all-outputs"` in the manifest,
as an undoable step. The parts come from the editor query `{"spread": {"layout", "bezel",
"monitors"}}` (`rigs::query`), which answers `{spread, canvas, screens, bounds, parts: [{id, rect}]}`
with each rect in canvas pixels, or `error` for an unknown layout or screens without a name or a
size (at most 64). `wprs-ctl scene preview DIR --layout dual|triple|vertical|mine --out FILE
[--time S]` writes the same view as a picture, without a desktop: the scene at its poster time, or
at second `S` of the scene clock when `--time` is 0 or more (as the editor shows the moment its
preview is at), each screen's part at its physical place and the frames between them. Dragging a layer that is not locked
selects it and moves it, as one gesture. The handles write with the `layer` action, as
`wprs-ctl scene layer DIR LAYER transform --at S` does: a value with keyframes takes a keyframe at
the playhead instead, and so does each part of a position or scale keyed on its own (`tracks`),
while its plain parts take the new value; a part that follows a link refuses the drag and says so.
*Write over keyframes* in the canvas toolbar makes them write a fixed value over keyframes and
links (`replace`). A drag that starts outside the layers, or any drag with
Shift, draws a rectangle that selects the layers it touches and picks them as Ctrl does in the
layer list, ready for *Group*. Ctrl and the wheel zoom, the wheel or a drag with the middle or
right button pans.

## Pointer areas and sound bindings

*Pointer* in the inspector of a group, image, video, text, shape, solid, spectrum or mesh layer
has three states: lets the pointer through, takes it over its whole box, or takes it where the
layer is opaque, with a slider for the opacity (`layer` action `touch`; the same on the command line
with `wprs-ctl scene act DIR layer '{"touch": {"layer": ID, "takes": "alpha", "alpha": 0.3}}'`).
While a layer that takes the pointer is selected, the canvas outlines the area of every layer that
takes it (solid line for the whole box, dashed for the opacity threshold, with its percentage), the
selected one stronger. The areas come from the editor query `{"touch": true}` (`rigs::query`), which
answers `{layers: [{id, pointer, takes, alpha, bones}]}` for the layers with `solid` or `hit` bones.
The bones that take the pointer of a mesh or model are chosen from the bones of its file in the rig
panel.

The binding menu of a value offers *Follow the sound* with the five named levels and *A band or a
range of bands…*, a picker with the 16, 32 or 64 bands drawn as a strip (click a band or drag over
several, with their pitch in Hz or kHz), how several bands count (average or loudest) and the channel
(both, left or right). *Follow the pointer* picks `x`, `y`, `inside` or `speed`. Both go through the
editor query `{"bands": {...}}`: without `pick` or `direction` it answers `{of, bands: [{index, from,
to}], named: {bass, mid, treble}, levels, pointer}` for `of` 16, 32 or 64; with `pick` (a level name,
a band number or `[first, last]`, with optional `of`, `reduce` and `channel`) or `direction`, and the
`base` source being replaced, it answers `{source, label, hertz}` where `source` keeps the shaping
(`scale`, `offset`, `clamp`, `bounds`, ...) of `base` and drops what only belongs to the old source,
or `{error}`. The value is then written with `set`, one undoable step.

## Shared parts

The *Assets* panel of the scene editor has two tools for `assets` packages (`package.md`, "Shared
resources"). *Make an assets package* lists the layers, particle systems, scripts and package effects
of the scene with the selected layer checked; each checked part takes a title and a thumbnail, and the
package takes a title, an id, a license, the name its parts are credited to and a preview picture.
It saves the scene, writes a new folder with `assets.json`, the files the parts use, the credits of
the scene that cover them and the pictures made into jpegs of at most 1024 pixels and 1 MiB, and
offers to open that folder in the editor, where it is built like a scene. `wprs-ctl pack new-assets
SCENE OUT --part KIND:ID… --id ID --title TITLE --license SPDX --attribution NAME [--thumbnail FILE]
[--part-thumbnail KIND:ID=FILE] [--part-title KIND:ID=TITLE]` does the same.

*Import from an assets package* opens a `.wpak`, shows each part with its thumbnail and, before
anything is copied, the license and credit of every file it brings; the checked parts are imported
with `import_assets`, one step of the history. `wprs-ctl pack parts FILE [--json]` lists the same.

## Web pages and other packages

The editor opens and creates every kind of package folder, not only scenes. A folder whose
manifest has a `kind` other than `scene` or `assets` is a document of the manifest alone
(`{"manifest": …}`, without `scene`): actions on `/manifest` work as in a scene, saving writes only
`manifest.json`, and the problems are those of the manifest, the files of the folder (`validate`
of the format: a missing entry, a preload that is not in the folder) and the verdicts of each
platform. The outline adds `kind` and, for such packages, `files`: every file of the folder but
hidden ones, with `path`, `size`, `kind` (`page`, `script`, `style`, `wasm`, `data`, `image`,
`video`, `sound`, `font` or `other`) and `role` (`entry`, `preload`, `preview`, `manifest`,
`locale`, `signature` or `file`).

The starting point `web` writes `index.html` and `app.js`, a clock that takes its color from a
`color` property, hides nothing and stops its timer while the page is paused, and a valid `web`
manifest (`entry` `index.html`, no inputs, `web.background`). `wprs-ctl scene open DIR` lists the
files of such a folder with their role, kind and size, and `--from FILE.wpak` extracts a package of
any kind into an empty folder.

| Action | Arguments | `wprs-ctl scene manifest DIR` |
|--------|-----------|-------------------------------|
| `manifest` `add_file` | `file`, `to` (a folder of the package, optional); copies the file into a web, picture or video package folder, never over `manifest.json` or its signature, and records nothing in the history | `add-file FILE [--to FOLDER]` |
| `manifest` `media` | `file`; copies a picture or a video into the folder of a `image` or `video` package, as its `entry` (it must be of the kind of the package, a video for a `video` package, and must pass the checks of the format); undo returns to the earlier entry | `scene act DIR manifest '{"media": {"file": "FILE"}}'` |

The starting points `image` and `video` write the manifest of a picture or video package (`kind`, an
`entry` of `picture.png` or `video.mp4` that is not in the folder yet, so the problems ask for the
file) and the *Scenes* page offers them next to the others, `wprs-ctl scene new` as well.

The app opens such folders on a page of their own, *Package*: the file list (size, role, *Add
files…*, *Open the folder*, and per file *Use as the entry page* or *Use as the thumbnail*), the
settings of the kind in the middle, and the inspector and the problems on the right.

For a web page the settings in the middle start with *Page*: a list of the scripts of the folder to
run before the page, with *Add a script…* to copy a `.js` or `.mjs` file in and choose it at once
(`web.preload`, the `web` action with `preload`), a color picker for the color shown while the page
loads (`web.background`, `#rrggbb`) and a slider from 0 to 5120 KiB for the data the page keeps
(`web.storage_kib`, 0 keeps nothing). Each control sends the `manifest` action `web` with `values`
and `clear`, so `wprs-ctl scene act DIR manifest '{"web": {"values": {"storage_kib": 256}}}'` does the
same and so does `wprs-ctl scene manifest DIR web`; a color that is not `#rrggbb`, a size over
5120 KiB or a preload that is not in the folder is refused like any other invalid manifest.

The *Preview* tab next to *Settings* shows the page as it is in the folder, with the manifest as it
is in the editor, inside the window: the preview stream of `PreviewScene` (`dbus.md`) is made once
the tab is first shown and covered, so the page pauses, while another tab is open. *Reload* builds the
page again from its files, which is how a change made in another editor shows; the quality
selector (Q0 to Q3) runs the page at that level of the output and the frame rate selector caps its
frames (15, 30, 60 or not capped), both of the preview only. What the page writes with `console.log`
and the errors it raises appear in the console under the problems. `wprs-ctl scene preview DIR`
does the same on the desktop: for a web page it sends the manifest and the staged build, with
`--quality`, `--restart` (reload) and `--console`.

Under the preview of a web page, *The page and its lights* shows the colors the page sends with
`window.wprs.rgb.setImage` as a grid, read ten times a second from `GetStrip` of the preview output while
the tab is shown, and a switch that sets or removes `permissions.rgb` in the manifest (the same
`set` and `remove` actions on `/manifest/permissions/rgb`). A preview lights the strip without the
grant, with the lights off and without sending anything to the devices, so the author sees what a
user would get before publishing; `wprs-ctl rgb strip -o OUTPUT` reads the same strip for the output of
`wprs-ctl scene preview DIR -o OUTPUT`.

The *Properties* tab of a web page is the list of user properties of the scene editor, the same
widget and the same actions (`add_property`, `set`, `remove`, the property inspector on the right):
sliders, switches, colors, choices, texts, buttons, groups and, for a page, the `file` and `folder`
properties whose choice the page reads through `window.wprs.files` and `window.wprs.random`; the
user picks a folder per screen in the settings of the page. `wprs-ctl scene add-property DIR KIND` and the other property
commands do it from a terminal, and so does `wprs-ctl scene act`.

*Network* lists the https origins the page may reach, one row each with a remove button, and a field
with *Add origin* (at most 8). It sends the `manifest` action `network` (`origins`, with `add` or
`remove` to change the list instead of replacing it): origins are written in lower case without a
trailing slash, a repeated one is kept once, and `network` is added to `inputs` with the first origin
and removed with the last, so the declared input and the origins never disagree; `inputs` refuses to
add or remove `network` by itself. Only a web page takes origins. The same action is
`wprs-ctl scene manifest DIR network ORIGIN... [--add|--remove]`.

A preset package (*Preset* in the settings) chooses its base from the installed packages and then lists
the real settings of that base, in its order, each with the control of the property itself: a color
picker for a color, a slider with the range and unit of a slider, a switch, a box with the labelled
choices of a combo, a field for a text, and *Picture or video…* for each `file` property (the file is
copied into `assets/`). A setting the preset sets is shown in bold and *Default* takes it out; the
others show the default of the base. Every control sends the `manifest` action `preset` (`values` or
`files`, `clear`), so `wprs-ctl scene act DIR manifest '{"preset": {"values": {"speed": 0.4}}}'`
does the same. The list comes from the `base` query of `rig`, `{"base": {"file": "<package or
folder>"}}`, which reads the manifest of the base package, and answers `{id, version, fields, files}`:
the fields are those of the inspector (`widget`, `range`, `unit`, `options`, `names`, `default`,
`value` and `set` from the values of the preset, with `/preset/values/<id>` as pointer) and `files`
the `file` properties with their `accept` and the picture the preset gives. A base that is not
installed shows no settings and asks to install it.

A picture or video package shows, in *Settings*: the file (*Choose the picture…* or *Choose the
video…*, the `media` action, with its size and length), *Display* and, for a video, *Playback*.
*Display* sends the `display` action: the placement `fit` (fill, fit, stretch, center, tile or
span), the mirror `flip` as three buttons, the `background` color, `span` (*Spread over every
screen*) and, for a video, `zoom` (1 to 8) and `position` with an alignment pad: a rectangle with a
handle that is dragged (or clicked) to choose which part of the zoomed video shows, from -1 to 1 on
each side, with magnets at the edges and the middle and a button that centers it again. *Playback*
of a video starts with the trim on a timeline over the real length of the video (the `media` query of
`rig`, `{"media": {}}`, answers `{kind, entry, present, bytes, length, size, aspect}`; `length` comes
from the file, a picture gives its `size` and a video its `aspect`): a strip of thumbnails (the cache
of `Strips`, also used by the clocks of the timeline) with a bar for the part that plays and two grips
that set `start` and `end` of the `playback` action when they are let go (an `end` dragged to the end
of the video is taken out, the grips never cross) and *Whole video*; a speed slider from 0.1 to 4
(logarithmic) and *Loop*; and the volume, muting and frame rate that web pages have. A video package
has a *Presets* tab in place of *Properties*: *Add preset* adds `/manifest/presets/N`, and the
inspector shows one row per video setting (speed, start, stop at, volume, zoom, x and y) with the
slider and the range of the video itself (`start` and `end` up to its length), so `set` on
`/manifest/presets/N/values/<id>` and `scene act` do the same.

The *Preview* tab also exists for pictures and videos: the editor sends the daemon a scene made from
the folder (`wprs_editor::reels::preview_document`): one layer that shows the file at the proportion
of the picture or video, a canvas that places it with `display.fit` and `display.background`, the
mirror and the zoom as the scale and the part shown as the position, and `playback` as the playback
of the layer (speed, start, end and loop) and of the manifest (frame rate, volume and muting). The
quality selector and the frame rate selector are the same as the ones of a web page.

*Required packages* (in the *Settings* tab of every kind of package) lists `requires.packages`, one row
each with a remove button, and a picker to add one: a box that lists the installed packages of the
library (assets packages included, never the package itself) and also takes an id typed by hand, and
a version range. Left empty, the range is the one compatible with the installed version, `^1.2` for
`1.2.7`. It sends the `manifest` action `requires` (`packages`: `{id, version}`, with `add` or
`remove` to change the list instead of replacing it). A `version` that is a plain version, such as
`1.2.7` or `0.4.1`, becomes its compatible range (`^1.2`, `^0.4`, `^0.0.3` for `0.0.3`) and any
other text must be a SemVer requirement, otherwise the action is refused; an id already listed
takes the new range, the package cannot require itself, an id must be valid, at most 16 packages
fit, and `requires.features` follows (`packages` with the first one, removed with the last).
`wprs-ctl scene act DIR manifest '{"requires": {"packages": [{"id": "org.example.fx", "version":
"1.2.7"}], "add": true}}'` does the same from a terminal.

*Inputs* has a check box for each input a page cannot show it uses, since nothing in its code tells
the tools: `audio`, `pointer`, `clicks` and `media`; the `network` box is read-only and follows the
origins. A box sends the `manifest` action `inputs` with `add` or `remove`, which refuses a name that
is not an input. The same action is `wprs-ctl scene manifest DIR inputs NAME... [--add|--remove]`.

*Playback* sets how the page plays by default: the highest frame rate (not set, follow the
screen, or 10 to 240 frames a second), the volume as a slider with *Default* to remove it, and
whether it starts without sound. It sends the `manifest` action `playback` with `values` and
`clear`; `speed`, `start`, `end` and `loop` are only for videos and a page refuses them. What the
user sets for a screen (frame rate, volume, muting) always wins over these defaults. The same
action is `wprs-ctl scene manifest DIR playback --max-fps N --volume V --muted true`.

### Preset packages

The starting point `preset` (the *Preset* card of the *Scenes* page, `wprs-ctl scene new DIR --from
preset`) creates a package of kind `preset` (`package.md`, "Presets"): the manifest, with the
entry `preset.json`, and `preset.json` itself, which the editor reads next to the manifest as the
document part `/preset` (`base`, `values` and `files`), with undo, redo and saving like the rest,
so `set`, `add` and `remove` reach it too. The *Package* page shows, for such a package, the
section *Preset* instead of *Required packages*: a box with the installed packages (or any id typed
by hand) and a version range to choose the base, the values and files already set, one row each
with a remove button, and a row to add a value (a setting id and its value, JSON or plain text such
as `#3d5aff` or `0.4`) or a picture or video that is copied into `assets/`. Title, license, origin
and the preview picture are the ones of any package (*Title, license, origin and more…* and *Use as
the thumbnail*). Everything goes through the `manifest` action `preset`, with `base` (`{id,
version}`, where a plain version such as `1.2.3` becomes its compatible range `^1.2` and a missing
one means any), `values` (setting id to value), `files` (setting id to the path of a picture or
video to copy) and `clear` (setting ids to forget): the base is also written in
`requires.packages`, a setting is in `values` or in `files` and never in both, a file that is not a
picture or a video is refused, and the package cannot be its own base. A preset without a valid
base is a problem of the package until one is chosen. Building, signing and publishing it are the
ones of any package (`wprs-ctl pack build`, `pack sign`, `publish`); the values the installed base
does not know are left out with a warning when the preset is applied. `wprs-ctl scene act DIR
manifest '{"preset": {"base": {"id": "org.example.dusk", "version": "1.2.3"}, "values": {"tint":
"#3d5aff"}}}'` does the same from a terminal.

## Starting points, building and applying

`wprs-ctl scene new DIR --from empty|particles|parallax|clock|web|preset|assets [--id ID] [--title TITLE] [--locale
LANGUAGE]` and the *Scenes* page of the app create a folder from a starting point, with the id
`org.example.<folder name>` (accents dropped, so `Canción` becomes `cancion`) and the folder name as
title unless given: an empty canvas, falling particles with a density property, layered hills that
follow the pointer, or a clock with a color property. The manifest is a valid
`wallpaper-rs.wpak/2` one: `default_locale` is the language of the title, the first language of the
desktop unless given (`es` for `es_ES.UTF-8`), or `en` when the labels of the properties, in English
and Spanish, lack it; `content` `general`, `photosensitive` `false`, `origin` `original`, license
`CC0-1.0`, and the inputs and features the scene uses. The starting point `assets` makes an empty
assets package (`kind` `assets`, entry `assets.json`, feature `assets`) that is edited like a scene: its
layers, particle systems, scripts and effects are the parts it offers, built and shared as in "Shared
parts"; the *Scenes* page lists it as *Assets package*.
`wprs-ctl scene open DIR --from FILE.wpak` extracts a scene package into an empty folder, which is
how the app edits a copy of a scene from the library. The copy drops `manifest.json.minisig`,
which signed the manifest of the original author, and gets the id `<id>.copy`, so it never takes
the place of the original in the library. A copy of a version 2 package is derived: its origin
credits the original (its authors, or its id without them, its title, license and home page) after
the works the original came from, and its authors stay for whoever edits it to fill in. Its
license, credits and files stay as they were. Every package kind opens this way, pictures and videos
included; `--id ID` gives the copy another id than `<id>.copy` (`<id>.copy2` for a copy of a copy),
which must be a valid id other than the original. A license that allows no derived works (an SPDX
expression whose every alternative holds `ND`, such as `CC-BY-ND-4.0`, or one that is empty) refuses
the copy, and nothing is written, unless `--mine` says that the person made the package or has the
permission of its author; `--facts` prints, without unpacking, the title, kind, license, authors,
the verdict (`allowed` or `refused`), the conditions that go with an allowed license (attribution,
share-alike, non-commercial, copyleft, custom terms) and the id of the copy. The app offers *Edit a
copy…* on the page of each package of the library (any kind that has a `.wpak` file) and in the list
of the *Scenes* page, which lists scenes, pages, pictures and videos: the dialog shows the same
facts, lets the person pick the folder and the id, and asks to confirm that the package is theirs
before it unpacks a refused license.

`validate` lists the problems of the document and of the folder with the pointer of the field
concerned, nested layers included, the same messages the app shows; unused files and a signature
left by an earlier build are warnings. With `--json` each problem also carries `said`, a message id
such as `scene.color` or `format.invalid` with its arguments; the app and `wprs-ctl` tell the
messages and refusals in the language of the desktop from `po/<language>/wprs-messages.po`, and the
outline names the steps of `history.undo` and `history.redo` and the layers of the starting points
(`label`) in that language while the ids stay as stored. A starting point made in a language its
labels lack keeps that language, the one of the title, as `default_locale` and copies the English
labels into it. `build` refuses unsaved changes and errors, signs
`manifest.json` with `--key` exactly as `wprs-ctl pack sign` does, or removes an earlier signature
without it, and packs the folder with the code of `wprs-ctl pack build` into `-o FILE` or
`<id>-<version>.wpak` next to the folder. `apply` builds the package into
`$XDG_DATA_HOME/wallpaper-rs/scenes/`, imports it into the library and shows it on an output, or
on every output.

*Review the package…* in the build dialog shows what `wprs_editor::review::report` returns, which
`rig` answers for `{"review": {}}`: every finding of the problems list (level `error`, `warning` or
`note`, message, the JSON Pointer to jump to and the file that holds it: `manifest.json`,
`scene.json` or the missing asset), the notes of `wprs-ctl pack check`, the platforms that refuse the
package or play it without a feature, the features a build adds, and the signature state of the
folder. *Check the signature* answers `{"signature": {"path", "keys"}}` for a built `.wpak` or the
folder, given the public key of the author: `valid` with the fingerprints that signed, `invalid` with
the reason, `unsigned`, or `unchecked` while no key is given. `wprs-ctl pack check DIR|FILE` prints the
same facts and `wprs-ctl pack validate DIR|FILE --key PUB` checks the signature of a folder as well
as of a package.

*Keys…* in the build dialog manages author keys. A pair is two files, `NAME.key` (secret, mode 0600)
and `NAME.pub`, in the keys folder of the app (`$XDG_DATA_HOME/wallpaper-rs/keys`, mode 0700); a key
may be encrypted with a password, which is asked only to create it and to sign. The app never shows or
logs a secret key, replaces a pair only after asking and warning that the old secret key is lost for
good, and gives a key to the build dialog by its path, as `--key` does. The same dialog signs the
hand-over of an online author to a new key (`wprs-ctl pack rotate-key OLD.key NEW.pub --author ID`)
and shows the signature to paste in the account. `wprs-ctl repo keygen NAME` makes the same pair in
the current folder, with the code of `wprs_editor::author_keys`.

*Add to a repository…* in the build dialog is the publishing step of the editor. It opens the folder
of a static repository (made in *Repositories* or with `wprs-ctl repo init`), adds the built package
to it, also unlisted, features it with a note of up to 140 characters or stops featuring it, and
publishes the repository with the secret online key and the days its index lasts. It calls the same
`wprs_format::stall` operations as the *Static repository* tab of *Repositories*, which also keeps the
operations of the root: new repository, signing the root, rotating it, revoking a package or a key and
the author keys. The command line has them as `wprs-ctl repo add [--unlisted]`, `unlist`, `list-again`,
`feature`, `unfeature`, `revoke`, `unrevoke`, `revoke-key`, `init`, `sign-root`, `rotate-root` and `publish`.

## Picture depth tools

The *Depth* page of the picture editor in the app and `wprs-ctl edit depth` shape the depth layers
that parallax moves (`dbus.md`, "Depth layers") through the `depth` section of the edit of a
picture. They are actions of the edit, the same ones the app runs with `editor.run` and
`wprs-ctl` with a command, so every step has the undo history of the picture: each stroke, each
change of the cuts and each mark is one step. All coordinates are fractions of the original
picture, before the crop and the rotation, and a radius is a fraction of its longer side.

| Action | Fields | `wprs-ctl edit depth …` |
|--------|--------|-------------------------|
| `depth_brush` | `points` (a stroke of 1 to 1 024 points, `X,Y;X,Y;...`), `by` (0 to 1, 0.1 by default: the depth added at the middle of the brush, fading to nothing at its edge), `raise` or `lower` (`raise` when neither), `radius` (above 0 and up to 1, 0.05 by default), `falloff` (0 hard to 1 soft, 0.5 by default: how much of the radius fades) and `clear` (removes the earlier strokes first, or alone removes them); adds the stroke to `depth.strokes`, which are applied in order to the adjusted depth map (after `near`, `far`, `gamma`, `dilate` and `smooth`), so every use of the map sees them: the single image warp, the depth masks, the layers and the overlay. At most 256 strokes and 3 000 points in all | `brush FILE X,Y;X,Y;... [--raise\|--lower] [--by V] [--radius R] [--falloff F] [--clear]` |
| `depth_cuts` | `cuts` (1 to 5 growing levels between 0.005 and 0.995, at least 0.005 apart, `L,L,...`) or `auto`; the layers end at those depths instead of at the ones the clustering finds (a layer left without pixels is dropped, and then the cut that bounded it), so the number of layers follows the cuts and not `parallax.layers`; `auto` goes back to the clustering | `cuts FILE L,L,... \| --auto` |
| `fill_mask` | `points`, `radius` and `falloff` as the brush, `refill` or `protect` (one of them with `points`) and `clear`; marks, in `depth.fills`, where the band of a layer, the pixels behind nearer layers that moving can uncover, ignores its reach and is always filled (`refill`), or where it is never filled (`protect`, the pixels stay transparent in the layers behind); a later mark paints over an earlier one where they meet | `fill-mask FILE X,Y;X,Y;... (--refill\|--protect) [--radius R] [--falloff F] [--clear]` |

The layers (`wprs-ctl edit depth layers`) and the key of their cache follow the strokes, cuts and
marks, so a change builds a new set and the old one is trimmed with the cache. The map of the app
and `edit depth export --adjusted` include the strokes. `edit depth show FILE --layer N --out PNG`
saves one layer with its transparency, `--band PNG` also saves the band of that layer, white on
black, computed again with the cuts, strokes and marks of the set, and `--fill lama|simple` picks
the set made with that fill instead of the newest. `edit depth compare FILE --out PNG [--layer N]
[--at X]` builds, when they are missing, the set filled by LaMa and the one filled by the simple
fill, with progress on the standard error, and saves one layer with LaMa on the left of `X` (0.5
by default) and the simple fill on the right; it refuses to run while LaMa is not installed.

In the app the *Depth* tool of the picture editor, once the picture has a depth map, shows the whole
picture, unturned and uncropped, so that what is painted lines up with the original (the crop, the
rotation and the straightening come back with the other tools), and offers:

- *Show the depth map over the picture*: the adjusted map, strokes included, in grey or in false
  colour, over the picture with its own opacity (`Relief.look`, drawn once per change of the edit and
  never per frame).
- *Paint depth*: the brush for `depth_brush`, with raise or lower, size, strength and soft edge. The
  canvas draws the circle of the brush and the stroke while the button is down, thinned to a point
  every quarter of the radius, and sends the stroke on release as one `depth_brush`; undo removes it
  alone. *Remove the strokes* is `depth_brush` with `clear`.
- *Layer cuts*: a strip with the histogram of the depth, far on the left, coloured by layer, and a
  handle for each cut of the layers (the manual ones of `depth.cuts`, else the ones of the newest
  set or the ones the clustering would find). Dragging a handle keeps it at least 0.01 from its
  neighbours and draws, with `Relief.trace`, the outline of every layer tinted on the picture, at 512
  pixels and without building anything; releasing it sends one `depth_cuts`, and the layers are built
  again with those cuts half a second after the last change, a newer change cancelling the build in
  progress. A double click on a handle removes it, a click on the strip or *Add a cut* adds one in the
  widest gap (at most 5), and *Back to automatic* is `depth_cuts` with `auto`.
- *Fill brush*: the same brush for `fill_mask`, with *Refill* and *Protect*, size and soft edge (no
  strength: a mark is all or nothing where it covers at least half of the brush). The marks show
  in the layer views as the band they widen or empty, the layers are built again after the last
  stroke, and *Remove the marks* is `fill_mask` with `clear`.
- *Layers and fill*: a list of the layers, from the farthest, that shows one alone over the picture
  (`Relief.view`), with *Highlight the band that moving can uncover* tinting the pixels the set
  filled (the `--band` of `edit depth show`), and *Compare LaMa with the simple fill*, which shows the
  LaMa layer on the left of a divider that is dragged and the simple one on the right
  (`edit depth compare`). A set that is missing is built then, with its progress and a *Cancel*
  button, and the comparison is offered only while LaMa and ONNX Runtime are installed.

The settings page of the app has, next to the LaMa download, *Check LaMa* (`wprs-ctl models verify
lama-big-fp32`), *Remove LaMa* (`models remove lama-big-fp32`) and *Rebuild the layers*, which
deletes every layer set of the cache (the same as `edit depth layers --clear` for each picture) and
reloads the daemon so that it builds those of the pictures it shows; the button shows how many sets
are kept, or the progress of the `layers` job of the daemon while one runs.
