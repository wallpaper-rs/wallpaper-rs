# Acciones y preguntas del editor desde la terminal

Todo lo que se puede hacer con el editor de escenas se puede hacer también desde la terminal. Cada operación de edición es una **acción**. Cada consulta sobre la escena es una **pregunta**. Esta página explica cómo usarlas. Las páginas que enlaza al final describen cada una, con sus argumentos y un ejemplo que funciona. Para trabajar con el editor gráfico, vea [editor](editor.md).

## Cómo llamar a una acción

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text"}'
wprs-ctl scene act mi-escena hide '{"id":"text","hidden":true}'
```

- `mi-escena` es la carpeta de la escena (la que tiene `manifest.json` y `scene.json`). Se crea con `wprs-ctl scene new mi-escena`.
- La segunda palabra es el nombre de la acción, como `add_layer` o `hide`.
- El último argumento son los argumentos de la acción, escritos como un objeto JSON entre comillas simples. Si la acción no necesita argumentos se puede omitir o escribir `{}`.
- Con `-` en lugar del JSON, la orden lee los argumentos de la entrada estándar. Con `--file RUTA`, los lee de un archivo.
- Cada llamada es un paso que se puede deshacer, igual que en el editor.

Qué imprime:

- Las acciones que crean algo imprimen su id (`add_layer` imprime el id de la capa). Las demás no imprimen nada.
- Con `--json`, imprimen la lista de operaciones de bajo nivel que cambiaron el documento.

## Varias acciones en un solo paso

La acción especial `batch` toma una lista de acciones y las aplica juntas, como un solo paso de deshacer. Si una falla, ninguna se aplica.

```sh
wprs-ctl scene act mi-escena batch '[{"add_layer":{"kind":"solid","id":"fondo"}},{"hide":{"id":"fondo","hidden":true}}]'
wprs-ctl scene act mi-escena undo
```

## Deshacer, rehacer y revisar

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"shape","id":"forma"}'
wprs-ctl scene history mi-escena
wprs-ctl scene act mi-escena undo
wprs-ctl scene act mi-escena redo
wprs-ctl scene validate mi-escena
```

`scene history` lista los pasos que recorren `undo` y `redo`. `scene validate` comprueba la escena y sale con código 1 si algo está mal. Algunas acciones (como `set`) aceptan valores que la comprobación después rechaza; por eso conviene validar al terminar.

## Cómo leer los errores

Un error se imprime en la salida de errores, con el texto `Error:` y la causa, y la orden sale con un código distinto de cero.

| Código | Cuándo ocurre | Ejemplo |
| --- | --- | --- |
| 1 | La acción es válida pero no se puede aplicar a esta escena | `Error: there is no layer "zzz"` |
| 2 | La llamada está mal escrita: acción o pregunta que no existe, argumento que falta o JSON roto | falta el campo `hidden` |

Los mensajes de error salen en inglés. Cuando el nombre de la acción no existe, el mensaje lista todos los nombres válidos. Si una acción falla dentro de `batch`, la escena queda como estaba.

## Cómo saber qué acciones hay

```sh
wprs-ctl scene actions
wprs-ctl scene actions hide
wprs-ctl scene actions --json
```

- Sin nombre, lista todas las acciones con una línea cada una.
- Con un nombre, muestra sus argumentos (si son obligatorios u opcionales y su tipo) y una línea de ejemplo. El ejemplo es solo un esqueleto: algunas acciones necesitan además argumentos que dependen de la escena.
- Con `--json`, imprime la lista, o el esquema JSON de los argumentos, en JSON.
- Con `--queries`, hace lo mismo con las preguntas: `wprs-ctl scene actions --queries` y `wprs-ctl scene actions --queries wind`.

Algunas acciones se llaman con una sola variante: el argumento es un objeto con una clave, que es el nombre de la variante. Por ejemplo `effects` se llama con `{"add":{...}}`, `{"set":{...}}` o `{"remove":{...}}`. Las páginas de más abajo describen cada variante.

## Cómo hacer una pregunta

```sh
wprs-ctl scene ask mi-escena sounds '{}'
```

- La pregunta responde con JSON en la salida estándar. Una pregunta no cambia la escena.
- El JSON final son los argumentos de la pregunta; con `-` o `--file` se leen como en las acciones.
- Muchas preguntas tratan de una capa. La capa se da en los argumentos, o con `--layer ID` cuando los argumentos no la nombran.
- Si falta algo para responder (una capa que no tiene huesos, por ejemplo), la respuesta suele ser `null` o una lista vacía. No es un error.
- `--inside PLANTILLA` hace que la escena que se edita o se consulta sean las capas de una plantilla de script.

## Los ejemplos de estas páginas

Cada ejemplo está en un bloque que se ejecuta sobre una escena recién creada con `wprs-ctl scene new mi-escena`. Las líneas de un bloque van en orden: las primeras preparan lo que necesita la última. Los archivos de ejemplo que se nombran (`foto.png`, `hoja.png`, `sonido.ogg`, `modelo.glb`, `figura.glb`, `video.webm`) están en la carpeta desde donde se ejecuta; use los suyos con otro nombre.

Los nombres de las coordenadas, las unidades y los ids se explican al principio de cada página, en «Antes de empezar».

## Índice

Cada nombre enlaza con su descripción.

### [Capas, recursos y lienzo](acciones-capas.md)

Acciones: [set](acciones-capas.md#set), [add](acciones-capas.md#add), [remove](acciones-capas.md#remove), [move](acciones-capas.md#move), [add_layer](acciones-capas.md#add_layer), [remove_layer](acciones-capas.md#remove_layer), [duplicate_layer](acciones-capas.md#duplicate_layer), [move_layer](acciones-capas.md#move_layer), [group](acciones-capas.md#group), [ungroup](acciones-capas.md#ungroup), [hide](acciones-capas.md#hide), [lock](acciones-capas.md#lock), [solo](acciones-capas.md#solo), [import](acciones-capas.md#import), [import_assets](acciones-capas.md#import_assets), [asset](acciones-capas.md#asset), [rgb](acciones-capas.md#rgb), [rename](acciones-capas.md#rename), [paste](acciones-capas.md#paste), [layer](acciones-capas.md#layer), [canvas](acciones-capas.md#canvas), [film](acciones-capas.md#film), [sprites](acciones-capas.md#sprites)


### [Tiempo, paquete, propiedades y scripts](acciones-tiempo-y-paquete.md)

Acciones: [keyframe](acciones-tiempo-y-paquete.md#keyframe), [import_effect](acciones-tiempo-y-paquete.md#import_effect), [remove_effect](acciones-tiempo-y-paquete.md#remove_effect), [import_script](acciones-tiempo-y-paquete.md#import_script), [translate](acciones-tiempo-y-paquete.md#translate), [thumbnail](acciones-tiempo-y-paquete.md#thumbnail), [shot](acciones-tiempo-y-paquete.md#shot), [upgrade](acciones-tiempo-y-paquete.md#upgrade), [add_property](acciones-tiempo-y-paquete.md#add_property), [move_property](acciones-tiempo-y-paquete.md#move_property), [rename_property](acciones-tiempo-y-paquete.md#rename_property), [remove_property](acciones-tiempo-y-paquete.md#remove_property), [condition](acciones-tiempo-y-paquete.md#condition), [add_preset](acciones-tiempo-y-paquete.md#add_preset), [manifest](acciones-tiempo-y-paquete.md#manifest), [property](acciones-tiempo-y-paquete.md#property), [timeline](acciones-tiempo-y-paquete.md#timeline), [script](acciones-tiempo-y-paquete.md#script), [template](acciones-tiempo-y-paquete.md#template), [pool](acciones-tiempo-y-paquete.md#pool), [uses](acciones-tiempo-y-paquete.md#uses), [shader](acciones-tiempo-y-paquete.md#shader), [undo](acciones-tiempo-y-paquete.md#undo), [redo](acciones-tiempo-y-paquete.md#redo)

Preguntas: [palette](acciones-tiempo-y-paquete.md#palette), [clocks](acciones-tiempo-y-paquete.md#clocks), [pools](acciones-tiempo-y-paquete.md#pools), [uses](acciones-tiempo-y-paquete.md#uses), [shader_source](acciones-tiempo-y-paquete.md#shader_source), [shader_check](acciones-tiempo-y-paquete.md#shader_check), [strings](acciones-tiempo-y-paquete.md#strings), [flashes](acciones-tiempo-y-paquete.md#flashes), [credits](acciones-tiempo-y-paquete.md#credits), [curves](acciones-tiempo-y-paquete.md#curves), [key_clip](acciones-tiempo-y-paquete.md#key_clip), [fires](acciones-tiempo-y-paquete.md#fires)

### [Mallas, huesos y animación](acciones-mallas.md)

Acciones: [mesh_from_picture](acciones-mallas.md#mesh_from_picture), [bone](acciones-mallas.md#bone), [mesh_depth](acciones-mallas.md#mesh_depth), [weights](acciones-mallas.md#weights), [auto_weights](acciones-mallas.md#auto_weights), [clip](acciones-mallas.md#clip), [key](acciones-mallas.md#key), [edit_keys](acciones-mallas.md#edit_keys), [clip_event](acciones-mallas.md#clip_event), [shape_sculpt](acciones-mallas.md#shape_sculpt), [expression](acciones-mallas.md#expression), [ik](acciones-mallas.md#ik), [physics](acciones-mallas.md#physics), [collider](acciones-mallas.md#collider), [bone_rule](acciones-mallas.md#bone_rule), [wind](acciones-mallas.md#wind), [attach](acciones-mallas.md#attach), [point](acciones-mallas.md#point), [parts](acciones-mallas.md#parts), [animate](acciones-mallas.md#animate)

Preguntas: [rig](acciones-mallas.md#rig), [weights](acciones-mallas.md#weights), [pose](acciones-mallas.md#pose), [keys](acciones-mallas.md#keys), [handles](acciones-mallas.md#handles), [motion](acciones-mallas.md#motion), [script_calls](acciones-mallas.md#script_calls), [outline](acciones-mallas.md#outline), [wind](acciones-mallas.md#wind), [wander](acciones-mallas.md#wander)

### [Efectos y máscaras](acciones-efectos.md)

Acciones: [effect_rule](acciones-efectos.md#effect_rule), [paint_mask](acciones-efectos.md#paint_mask), [comb_flow](acciones-efectos.md#comb_flow), [ramp](acciones-efectos.md#ramp), [depth_map](acciones-efectos.md#depth_map), [effects](acciones-efectos.md#effects)

Preguntas: [normals](acciones-efectos.md#normals), [map](acciones-efectos.md#map), [ramp](acciones-efectos.md#ramp), [depth_model](acciones-efectos.md#depth_model), [rules](acciones-efectos.md#rules), [effect_cost](acciones-efectos.md#effect_cost)

### [Partículas, luces y sonido](acciones-particulas-y-luces.md)

Acciones: [particles](acciones-particulas-y-luces.md#particles), [light](acciones-particulas-y-luces.md#light), [lighting](acciones-particulas-y-luces.md#lighting), [normals](acciones-particulas-y-luces.md#normals), [paint_map](acciones-particulas-y-luces.md#paint_map), [ambient](acciones-particulas-y-luces.md#ambient)

Preguntas: [particles](acciones-particulas-y-luces.md#particles), [budget](acciones-particulas-y-luces.md#budget), [lights](acciones-particulas-y-luces.md#lights), [falloff](acciones-particulas-y-luces.md#falloff), [hemisphere](acciones-particulas-y-luces.md#hemisphere), [lighting](acciones-particulas-y-luces.md#lighting), [air](acciones-particulas-y-luces.md#air), [projection](acciones-particulas-y-luces.md#projection), [swarm_light](acciones-particulas-y-luces.md#swarm_light), [spread](acciones-particulas-y-luces.md#spread), [meter](acciones-particulas-y-luces.md#meter), [response](acciones-particulas-y-luces.md#response), [sounds](acciones-particulas-y-luces.md#sounds), [wave](acciones-particulas-y-luces.md#wave)

### [Escena 3D](acciones-3d.md)

Acciones: [scene3d](acciones-3d.md#scene3d)

Preguntas: [model_info](acciones-3d.md#model_info), [stage3d](acciones-3d.md#stage3d), [materials3d](acciones-3d.md#materials3d), [cost3d](acciones-3d.md#cost3d), [shadows3d](acciones-3d.md#shadows3d), [paths3d](acciones-3d.md#paths3d), [material_check](acciones-3d.md#material_check), [bone3d](acciones-3d.md#bone3d), [bound3d](acciones-3d.md#bound3d), [surface3d](acciones-3d.md#surface3d)
