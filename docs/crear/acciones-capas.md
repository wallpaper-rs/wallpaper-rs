# Acciones: capas, recursos y lienzo

Las acciones y preguntas para cambiar el documento, las capas, sus transformaciones, los recursos y el lienzo. Cómo llamarlas, cómo leer su salida y sus errores está en [acciones](acciones.md).

## Acciones

### Antes de empezar

Todas las acciones de esta sección se llaman igual: `wprs-ctl scene act CARPETA NOMBRE 'JSON'`. `CARPETA` es la carpeta de la escena (en los ejemplos, `mi-escena`) y `JSON` son los argumentos. Cada acción es un solo paso de **Deshacer**. Los archivos que se nombran (`foto.png`, `sonido.ogg`...) se buscan desde la carpeta en que ejecutas el comando.

**El documento.** Una escena guarda dos partes. `/manifest` es la ficha del paquete (título, licencia, propiedades de usuario, presets). `/scene` es la escena: `/scene/layers` (capas), `/scene/assets` (recursos), `/scene/canvas` (lienzo), `/scene/timelines` (líneas de tiempo), `/scene/effects` (efectos del paquete), `/scene/scripts`, `/scene/templates`, `/scene/pools` y `/scene/particles`. Para ver cualquier parte: `wprs-ctl scene show mi-escena RUTA`. Las rutas (JSON Pointer) empiezan por `/` y separan los niveles con `/`:

| Ruta | Qué es |
|------|--------|
| `/scene/layers/0` | La primera capa de la lista, la de más abajo (la que se dibuja primero). |
| `/scene/layers/0/opacity` | Un campo de esa capa. |
| `/scene/layers/1/transform/position` | La posición de la segunda capa. |
| `/scene/layers/2/layers/0` | La primera capa dentro de la capa 2, que es un grupo. |
| `/scene/assets/foto/path` | El archivo del recurso `foto`. |
| `/scene/canvas/size` | El tamaño del lienzo. |
| `/manifest/title` | El título del paquete. |
| `/manifest/properties/brillo` | La propiedad de usuario `brillo`. |

**Ids.** Cada capa, recurso, sistema de partículas y línea de tiempo tiene un id de 1 a 32 letras minúsculas, dígitos o `_`. Capas, recursos y sistemas comparten un solo juego de nombres: no puede haber una capa y un recurso con el mismo id.

**Coordenadas.** Salvo que se diga otra cosa, un punto `[x, y]` está en píxeles del lienzo, con `[0, 0]` arriba a la izquierda, `x` hacia la derecha e `y` hacia abajo. Los ángulos son grados en el sentido de las agujas del reloj. El tiempo es en segundos.

**Valores que se animan.** Varias acciones de capa y de lienzo tienen `at` (un segundo) y `replace`. Si el campo ya tiene claves, `at` escribe una clave en ese segundo. Si el campo sigue otra fuente (una propiedad, el audio...), la acción se rechaza, salvo que pongas `"replace": true`, que lo sustituye por un valor fijo.

**Copiar capas.** Copiar no es una pregunta de `scene ask`. Se hace con `wprs-ctl scene copy CARPETA ID...`, que imprime el texto que luego recibe `paste`.

### Cambiar el documento a bajo nivel

Estas cuatro acciones editan cualquier parte del documento por su ruta. No comprueban rangos ni nombres de campo: para saber si el resultado es válido, ejecuta `wprs-ctl scene validate mi-escena`. Las ventanas de la app hacen lo mismo con sus campos.

#### `set`

Escribe un valor en una ruta. Si en el camino faltan objetos, los crea. Si la ruta ya existe, reemplaza el valor.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `pointer` | texto (obligatorio) | Ruta del valor, por ejemplo `/scene/canvas/background`. |
| `value` | cualquier valor JSON (obligatorio) | El valor nuevo: número, texto, lista u objeto. |

```sh
wprs-ctl scene act mi-escena set '{"pointer":"/scene/canvas/background","value":"#112233"}'
```

#### `add`

Inserta un valor en una lista o en un objeto. En una lista, el último nivel de la ruta es la posición (0 es el primero) o `-` para ponerlo al final. En un objeto, es el nombre del campo nuevo (si ya existía, lo reemplaza).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `pointer` | texto (obligatorio) | Ruta donde insertar, por ejemplo `/scene/layers/-`. |
| `value` | cualquier valor JSON (obligatorio) | Lo que se inserta. |

```sh
wprs-ctl scene act mi-escena add '{"pointer":"/scene/layers/-","value":{"id":"fondo","type":"solid","color":"#102030"}}'
```

#### `remove`

Quita el valor de una ruta. Falla si la ruta no existe.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `pointer` | texto (obligatorio) | Ruta de lo que se quita. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene act mi-escena remove '{"pointer":"/scene/layers/0/color"}'
```

#### `move`

Mueve un valor de una ruta a otra, por ejemplo una capa a otra posición de la lista. No se puede mover un valor dentro de sí mismo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `from` | texto (obligatorio) | Ruta de origen. |
| `to` | texto (obligatorio) | Ruta de destino. En una lista, el último nivel es la posición. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"titulo"}'
wprs-ctl scene act mi-escena move '{"from":"/scene/layers/1","to":"/scene/layers/0"}'
```

### Capas

Para mover, escalar o girar una capa mira [Colocar y dar forma a una capa](#colocar-y-dar-forma-a-una-capa). La lista de capas está en el panel de capas de la app.

#### `add_layer`

Añade una capa de un tipo, al final de la lista (arriba del todo) o donde se pida. Imprime el id de la capa nueva. En la app es **Añadir una capa**, y **Capa desde un archivo** importa el archivo en el mismo paso.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `kind` | texto (obligatorio) | Tipo de capa. Uno de los de la tabla de abajo. |
| `id` | texto | Id de la capa. Si falta, sale de `name` o del tipo (`image`, `image_2`...). Si ya está en uso, la acción falla. |
| `name` | texto | Nombre para construir el id. En una capa `text` es además el texto que muestra (por defecto, `Text`). |
| `source` | texto | De dónde sale lo que muestra la capa. Ver abajo. |
| `file` | texto | Archivo (ruta desde donde ejecutas el comando) que se importa como recurso y se usa en la capa nueva. Debe ser del tipo de la capa: imagen, video, sonido o `.glb`. Solo vale para `image`, `video`, `sound`, `mesh` y `model`. |
| `into` | texto | Id de un grupo donde meter la capa. Sin él va a la lista principal. |
| `at` | entero | Posición en la lista: 0 es abajo del todo. Si es mayor que el largo, va al final. |

| `kind` | Qué crea | Qué es `source` |
|--------|----------|------------------|
| `image` | Una imagen. | Id de un recurso de imagen. Sin él, el primero que haya. |
| `video` | Un video. | Id de un recurso de video. Sin él, el primero que haya. |
| `sound` | Un sonido. | Id de un recurso de sonido. Sin él, el primero que haya. |
| `text` | Un texto. | No se usa. |
| `shape` | Una forma (rectángulo de 480 por 320 px). | No se usa. |
| `solid` | Un color liso que llena el lienzo. | No se usa. |
| `spectrum` | Un espectro del audio (960 por 240 px). | No se usa. |
| `particles` | Una capa que dibuja un sistema de partículas. | Nombre del sistema. Si no existe, se crea uno con valores por defecto. Sin él, usa el id de la capa. |
| `group` | Un grupo de capas. | No se usa. |
| `adjustment` | Una capa de ajuste: sus efectos cambian lo que hay debajo. | No se usa. |
| `mesh` | Una malla articulada de un archivo `.glb`. | Id de un recurso `.glb`. |
| `model` | Un modelo 3D de un archivo `.glb`. | Id de un recurso `.glb`. |
| `group3d` | Un grupo 3D. | No se usa. |
| `light` | Una luz. | `point`, `spot`, `tube` o `directional`. Sin él, `point`. |
| `shapes` | Formas que dibujan los scripts desde una reserva (`pool`). | Id de la reserva. Sin él, la primera. Antes hay que crear una con [`pool`](acciones-tiempo-y-paquete.md#pool). |

Si pides un tipo que necesita un recurso y no hay ninguno, la acción dice qué importar primero.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"text","name":"Titulo","at":0}'
wprs-ctl scene act mi-escena add_layer '{"kind":"group","id":"adorno"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"shape","into":"adorno"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"light","source":"spot"}'
```

#### `remove_layer`

Quita una capa y todo lo que la nombraba desde fuera: padres, máscaras, parámetros de efectos, ataduras a huesos, disparadores. Se rechaza mientras un sistema de partículas toma partículas de esa capa. En la app es **Quitar**.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la capa. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene act mi-escena remove_layer '{"id":"fondo"}'
```

#### `duplicate_layer`

Copia una capa justo encima de la original, con ids nuevos (`id_copy`). Si la capa es un grupo, copia también las de dentro. Imprime el id de la copia. Con `template` no hace una capa nueva: guarda la capa como plantilla para que un script la repita (ver [`template`](acciones-tiempo-y-paquete.md#template)). En la app es **Duplicar** y **Hacer plantilla**.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la capa que se copia. |
| `template` | sí/no | `true` la guarda en `/scene/templates` con el nombre `id_template` (o `id_template_2`...). Solo valen imagen, texto, forma, color liso, espectro, malla y grupo. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"titulo"}'
wprs-ctl scene act mi-escena duplicate_layer '{"id":"titulo"}'
wprs-ctl scene act mi-escena duplicate_layer '{"id":"titulo","template":true}'
```

#### `move_layer`

Cambia una capa de lugar: a otra posición de su lista, o a otro grupo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la capa. |
| `to` | entero (obligatorio) | Posición final en la lista de destino (0 es abajo del todo). Si es mayor que el largo, queda arriba del todo. |
| `into` | texto | Id de un grupo de destino. Sin él, la capa pasa a la lista principal. |

Para meter una capa en un grupo, el grupo tiene que estar más abajo en la lista que la capa; si está más arriba, la acción falla con un error de ruta. Primero baja el grupo con otro `move_layer`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"group","id":"adorno"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"titulo"}'
wprs-ctl scene act mi-escena move_layer '{"id":"titulo","to":0,"into":"adorno"}'
wprs-ctl scene act mi-escena move_layer '{"id":"titulo","to":0}'
```

#### `group`

Mete varias capas en un grupo nuevo, que queda en el lugar de la más baja. Imprime el id del grupo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `ids` | lista de textos (obligatorio) | Ids de las capas. Deben estar en la misma lista (todas en la principal o todas en el mismo grupo). |
| `id` | texto | Id del grupo. Por defecto `group`, `group_2`... |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"titulo"}'
wprs-ctl scene act mi-escena group '{"ids":["fondo","titulo"],"id":"cartel"}'
```

#### `ungroup`

Disuelve un grupo y deja sus capas en su lugar. Se rechaza si el grupo tiene algo propio que se perdería (opacidad, transformación, mezcla, máscara, efectos, parallax, visibilidad o recorte). En la app es **Desagrupar**.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id del grupo. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"group","id":"adorno"}'
wprs-ctl scene act mi-escena ungroup '{"id":"adorno"}'
```

#### `hide`

Oculta una capa o la vuelve a mostrar. En una capa de sonido silencia el sonido. Al ocultar guarda lo que valía antes y lo devuelve al mostrar.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la capa. |
| `hidden` | sí/no (obligatorio) | `true` oculta, `false` muestra. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene act mi-escena hide '{"id":"fondo","hidden":true}'
wprs-ctl scene act mi-escena hide '{"id":"fondo","hidden":false}'
```

#### `lock`

Bloquea una capa para que no se cambie en el lienzo de la app, o la desbloquea. Es una marca del editor (queda en `/scene/ext/editor/locked`): los comandos de la terminal siguen pudiendo cambiarla. En la app es **Bloquear**.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la capa. |
| `locked` | sí/no (obligatorio) | `true` bloquea, `false` desbloquea. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene act mi-escena lock '{"id":"fondo","locked":true}'
```

#### `solo`

Hace que las vistas previas del editor muestren solo una capa (y lo que la contiene), o todas otra vez. No cambia la escena que se guarda. En la app es **Solo**.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto | Id de la capa que se queda sola. Sin él, termina el solo. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene act mi-escena solo '{"id":"fondo"}'
wprs-ctl scene act mi-escena solo '{}'
```

#### `rename`

Cambia el id de una capa, línea de tiempo, recurso o sistema de partículas y corrige todo lo que lo nombra: padres, máscaras, parámetros de efectos, valores que siguen una línea de tiempo, campos que nombran un recurso, capas que usan un sistema. Los scripts no se reescriben: el comando imprime cada línea de script que nombra el id viejo para que la cambies a mano. En la app es **Cambiar el id de la capa** y **Cambiar el nombre…** en cada recurso.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `kind` | texto (obligatorio) | Qué se renombra: `layer`, `timeline`, `asset` o `system`. |
| `id` | texto (obligatorio) | Id actual. |
| `to` | texto (obligatorio) | Id nuevo: de 1 a 32 letras minúsculas, dígitos o `_`, y libre. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena rename '{"kind":"layer","id":"cielo","to":"fondo"}'
wprs-ctl scene act mi-escena rename '{"kind":"asset","id":"foto","to":"paisaje"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"polvo"}'
wprs-ctl scene act mi-escena rename '{"kind":"system","id":"polvo","to":"lluvia"}'
wprs-ctl scene act mi-escena timeline '{"id":"intro","add":true}'
wprs-ctl scene act mi-escena rename '{"kind":"timeline","id":"intro","to":"entrada"}'
```

#### `paste`

Pega capas copiadas con `wprs-ctl scene copy`, en esta escena o en otra. Las capas llevan ids libres (el mismo id si está libre, si no `id_2`...); los recursos, sistemas y efectos que usan se añaden si faltan, con sus archivos. En la app son **Copiar las capas** y **Pegar las capas**.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `clip` | objeto (obligatorio) | Lo que imprime `scene copy`: `layers` (las capas) y, si hacen falta, `assets`, `particles`, `effects` y `from` (la carpeta de la que se copiaron, para encontrar los archivos). |
| `into` | texto | Id de un grupo donde pegar. |
| `at` | entero | Posición donde pegar (0 es abajo del todo). Sin ella, arriba del todo. |

```sh
wprs-ctl scene act mi-escena paste '{"clip":{"layers":[{"id":"saludo","type":"text","text":"Hola","size":96,"color":"#ffffff"}]}}'
wprs-ctl scene act mi-escena add_layer '{"kind":"group","id":"adorno"}'
wprs-ctl scene act mi-escena paste '{"clip":{"layers":[{"id":"saludo","type":"text","text":"Hola","size":96,"color":"#ffffff"}]},"into":"adorno","at":0}'
```

Para obtener un `clip` de verdad, copia una capa de la escena:

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"saludo"}'
wprs-ctl scene copy mi-escena saludo
```

#### `rgb`

Hace que una capa de ajuste envíe lo que captura a la iluminación RGB de tu equipo, o que deje de hacerlo, y elige la zona que captura. Al activarlo pide el permiso `rgb` del paquete; al apagarlo (si ninguna otra capa lo usa) lo quita. Solo vale para capas `adjustment`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de una capa de ajuste. |
| `on` | sí/no | `true` envía a la iluminación RGB, `false` deja de enviar. |
| `size` | lista de 2 números | Ancho y alto en píxeles de la zona alrededor de la capa (de 1 hasta el doble del lienzo). Si la capa no tiene posición, la pone en el medio del lienzo. |
| `whole` | sí/no | `true` captura todo el lienzo (quita `size`). Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"adjustment","id":"ajuste"}'
wprs-ctl scene act mi-escena rgb '{"layer":"ajuste","on":true}'
wprs-ctl scene act mi-escena rgb '{"layer":"ajuste","size":[400,200]}'
wprs-ctl scene act mi-escena rgb '{"layer":"ajuste","whole":true}'
```
### Colocar y dar forma a una capa

#### `layer`

Una sola acción con varias variantes: se pasa un objeto con el nombre de la variante (`transform`, `grip`, `fit`, `parent`, `blend`, `depth`, `instance`, `touch` o `figure`) y dentro sus argumentos. Todas piden `layer`, el id de la capa. Lo que no se nombra se queda como estaba. Si un campo sigue una animación u otra fuente, ver «Valores que se animan» más arriba.

##### variante `transform`

Escribe a la vez posición, giro, escala, inclinación y tamaño de una capa. En la app son las asas del lienzo y los campos de transformación de la capa.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa. |
| `position` | lista de 2 números | Posición del punto de anclaje de la capa en el lienzo, en píxeles. |
| `rotation` | número | Giro en grados, de -360 a 360. |
| `scale` | lista de 2 números | Escala en `x` y en `y`; 1 es el tamaño original. |
| `skew` | lista de 2 números | Sesgado en grados. |
| `origin` | lista de 2 números | Punto de anclaje como fracción de la caja de la capa: `[0.5, 0.5]` es el centro, `[0, 0]` la esquina de arriba a la izquierda. |
| `tilt` | lista de 2 números | Inclinación en 3D en grados, de -89 a 89, alrededor del ancho y del alto de la capa. |
| `z` | número | Altura hacia el espectador, de -100000 a 100000. |
| `size` | lista de 2 números | Ancho y alto de la caja en píxeles (más de 0). Solo capas de imagen, video, color liso, ajuste y espectro. |
| `at` | número | Segundo en que se escribe, si el campo tiene claves. |
| `replace` | sí/no | `true` escribe un valor fijo aunque el campo siga claves u otra fuente. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena layer '{"transform":{"layer":"cielo","position":[400,300],"rotation":10,"scale":[1.5,1.5],"size":[800,600]}}'
```

##### variante `grip`

Hace lo que haría arrastrar un asa del lienzo hasta el punto `to`. Es la forma de repetir un gesto de la app desde la terminal: la acción calcula los números de posición, giro, escala o sesgado que dejan el asa en `to`, teniendo en cuenta los padres, grupos y la cámara.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa. |
| `grip` | texto u objeto (obligatorio) | Qué asa se arrastra. Ver la tabla de abajo. |
| `to` | lista de 2 números (obligatorio) | Punto del lienzo en píxeles adonde llega el asa. Con `nudge` es el desplazamiento. |
| `snap` | número | Distancia en píxeles. Con `move`, si un borde o el centro de la capa queda a menos de esta distancia de los bordes o el centro del lienzo o de otras capas, se pega. |
| `at` | número | Segundo en que se escribe, si el campo tiene claves. |
| `replace` | sí/no | Como en `transform`. |

| `grip` | Qué hace |
|--------|----------|
| `"move"` | Pone el punto de anclaje de la capa en `to`. |
| `"nudge"` | Mueve la capa por `to` (por ejemplo `[10, 0]`, diez píxeles a la derecha). |
| `"turn"` | Gira la capa para que su parte de arriba apunte a `to`. |
| `"origin"` | Mueve el punto de anclaje a `to` sin que la capa se mueva. |
| `{"scale":{"corner":[1,1],"even":true}}` | Escala para que la esquina `corner` (fracciones de la caja: `[1,1]` abajo a la derecha, `[1,0.5]` el medio del borde derecho) caiga en `to`. Con `even`, mantiene la proporción. |
| `{"skew":{"edge":[0.5,0]}}` | Sesga a lo largo del borde cuyo medio es `edge`, para que pase por `to`. |
| `{"size":{"corner":[1,1]}}` | Para una capa de ajuste que captura una región: cambia `size` para que la esquina caiga en `to`. |
| `{"tilt":{"axis":0}}` | Inclina la capa alrededor de su ancho (`axis` 0) o de su alto (`axis` 1) hasta acercar su borde a `to`. |
| `"lift"` | Cambia `z` para acercar la capa a `to`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"cielo","grip":"move","to":[500,500],"snap":10}}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"cielo","grip":"nudge","to":[10,0]}}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"cielo","grip":"turn","to":[900,100]}}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"cielo","grip":{"scale":{"corner":[1,1],"even":true}},"to":[900,700]}}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"cielo","grip":{"skew":{"edge":[0.5,0]}},"to":[600,100]}}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"cielo","grip":{"tilt":{"axis":0}},"to":[600,100]}}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"cielo","grip":"lift","to":[600,100]}}'
wprs-ctl scene act mi-escena add_layer '{"kind":"adjustment","id":"ajuste"}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"ajuste","grip":{"size":{"corner":[1,1]}},"to":[900,700]}}'
```

##### variante `fit`

Escala y coloca la capa para que cubra el lienzo, quepa en él o lo llene deformándose. Mantiene el giro y el sesgado.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa. |
| `fit` | texto (obligatorio) | `cover` (cubre todo el lienzo, puede recortarse), `contain` (cabe entera) o `stretch` (llena el lienzo deformándose). |
| `at`, `replace` | | Como en `transform`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena layer '{"fit":{"layer":"cielo","fit":"cover"}}'
```

##### variante `parent`

Hace que una capa siga a otra (su padre): se mueve, gira y escala con ella. Se rechaza para la misma capa, para sonidos y luces, si el padre ya la sigue o está dentro de ella, y si la cadena pasa de 8 capas.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa que sigue. |
| `parent` | texto | Id del padre. Sin él, la capa deja de seguir a su padre. |
| `keep` | sí/no | `true` reescribe posición, giro, sesgado y escala para que la capa se quede donde estaba. Por defecto `false`. |
| `at`, `replace` | | Como en `transform`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"titulo"}'
wprs-ctl scene act mi-escena layer '{"parent":{"layer":"titulo","parent":"cielo","keep":true}}'
wprs-ctl scene act mi-escena layer '{"parent":{"layer":"titulo"}}'
```

##### variante `blend`

Cambia el modo de mezcla de la capa con lo que hay debajo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa. |
| `blend` | texto (obligatorio) | Uno de: `normal`, `add`, `multiply`, `screen`, `lighten`, `darken`, `overlay`, `soft_light`, `hard_light`, `difference`, `exclusion`, `color_dodge`, `color_burn`, `hue`, `saturation`, `color`, `luminosity`, `linear_burn`, `linear_dodge`, `darker_color`, `lighter_color`, `vivid_light`, `linear_light`, `pin_light`, `hard_mix`, `subtract`, `divide`, `reflect`, `glow`, `phoenix`, `average`, `negation`, `tint`, `diffuse_light`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena layer '{"blend":{"layer":"cielo","blend":"multiply"}}'
```

##### variante `depth`

Pone la profundidad de parallax de la capa: cuánto se mueve con el puntero o la inclinación del teléfono. Si el paquete no declara la entrada `pointer`, la añade.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa. |
| `depth` | lista de 2 números (obligatorio) | Profundidad horizontal y vertical, cada una de -10 a 10. Si las dos son iguales se guarda un solo número; con `[0, 0]` se quita. |
| `replace` | sí/no | `true` escribe un valor fijo aunque el parallax siga otra fuente. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena layer '{"depth":{"layer":"cielo","depth":[2,1]}}'
```

##### variante `touch`

Elige si una capa recibe el puntero (para clics y para que un script o un disparador sepa que se pasa por encima).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa. |
| `takes` | texto (obligatorio) | `off` deja pasar el puntero, `all` lo toma en toda la caja, `alpha` solo donde la capa es opaca. |
| `alpha` | número | Con `alpha`, cuán opaco debe ser un punto para tomar el puntero, de 0 a 1. Si falta, 0.5 o el valor que tenía. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena layer '{"touch":{"layer":"cielo","takes":"alpha","alpha":0.3}}'
```

##### variante `instance`

Cambia, solo para una capa de partículas, lo que hace el sistema que dibuja (a menos que el sistema bloquee esos ajustes).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de una capa de partículas. |
| `values` | objeto | Qué cambiar. Campos: `alpha`, `size`, `speed`, `life`, `time_scale` y `brightness` (multiplicadores), `count` (multiplica los nacimientos, de 0 a 4), `color` (un color) y `points` (puntos de control `p0` a `p7`: una lista de posiciones, o un objeto con los números 0 a 7 que solo cambia esos). Un campo en `null` vuelve a lo que dice el sistema. |
| `clear` | sí/no | `true` quita todos los ajustes de la capa. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"polvo"}'
wprs-ctl scene act mi-escena layer '{"instance":{"layer":"polvo","values":{"alpha":0.5,"size":2,"count":2,"speed":1.5,"brightness":1.2,"color":"#ffcc00"}}}'
wprs-ctl scene act mi-escena layer '{"instance":{"layer":"polvo","values":{"points":{"1":[100,100]}}}}'
wprs-ctl scene act mi-escena layer '{"instance":{"layer":"polvo","values":{"alpha":null}}}'
wprs-ctl scene act mi-escena layer '{"instance":{"layer":"polvo","clear":true}}'
```

##### variante `figure`

Edita la figura de una capa de forma o de texto. Dentro de `figure` se elige una de cuatro cosas: `shape`, `stop`, `text_width` o `fonts`.

###### `figure` con `shape`

Hace lo que haría arrastrar un asa de la forma hasta el punto `to`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de una capa de forma. |
| `handle` | texto (obligatorio) | Nombre del asa. Según la forma: rectángulo `width`, `height`, `size`, `radius`; elipse `width`, `height`, `size`; anillo igual y `band` (el grosor); polígono `size`, `sides` (el número de lados); estrella `size`, `points` (las puntas), `inner` (la profundidad); línea `start`, `end`, `width`. Cualquier forma que no sea línea y tenga contorno: `stroke`. Si tiene degradado: `gradient_from`, `gradient_to`. |
| `to` | lista de 2 números (obligatorio) | Punto del lienzo, en píxeles, adonde llega el asa. |
| `even` | sí/no | Mantiene la proporción. Por defecto `false`. |
| `at`, `replace` | | Como en `transform`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"shape","id":"forma"}'
wprs-ctl scene act mi-escena layer '{"figure":{"shape":{"layer":"forma","handle":"radius","to":[1000,400]}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"shape":{"layer":"forma","handle":"width","to":[1200,540],"even":true}}}'
```

###### `figure` con `stop`

Edita los colores de un degradado de la forma (de 2 a 4 paradas, siempre en orden).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de una capa de forma. |
| `stop` | texto u objeto (obligatorio) | Qué hacer. Ver la tabla de abajo. |

| `stop` | Qué hace |
|--------|----------|
| `{"make":{"radial":false}}` | Crea un degradado de dos paradas; con `"radial": true` es circular. |
| `"clear"` | Quita el degradado. |
| `{"add":{"at":0.5,"color":"#ff0000"}}` | Añade una parada en `at` (0 a 1) con un color; sin `color` toma el de la siguiente. |
| `{"move":{"index":1,"at":0.3}}` | Mueve la parada número `index` (0 es la primera) a `at`. |
| `{"color":{"index":0,"color":"#00ff00"}}` | Cambia el color de una parada. |
| `{"remove":{"index":1}}` | Quita una parada. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"shape","id":"forma"}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"forma","stop":{"make":{"radial":false}}}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"forma","stop":{"add":{"at":0.5,"color":"#ff0000"}}}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"forma","stop":{"move":{"index":1,"at":0.3}}}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"forma","stop":{"color":{"index":0,"color":"#00ff00"}}}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"forma","stop":{"remove":{"index":1}}}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"forma","stop":"clear"}}}'
```

###### `figure` con `text_width`

Fija el ancho de un texto de varias líneas moviendo su borde derecho a `to` sin mover el izquierdo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de una capa de texto. |
| `to` | lista de 2 números | Punto del lienzo adonde llega el borde derecho. |
| `remove` | sí/no | `true` quita el ancho fijo. Por defecto `false`. |
| `at`, `replace` | | Como en `transform`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"texto"}'
wprs-ctl scene act mi-escena layer '{"figure":{"text_width":{"layer":"texto","to":[1400,540]}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"text_width":{"layer":"texto","remove":true}}}'
```

###### `figure` con `fonts`

Elige la tipografía de un texto y las de reserva, que se usan para las letras que la principal no tiene.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de una capa de texto. |
| `font` | texto | Nombre de la fuente; una cadena vacía la quita. |
| `fallback` | lista de textos | Fuentes de reserva, en orden, hasta 8. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"texto"}'
wprs-ctl scene act mi-escena layer '{"figure":{"fonts":{"layer":"texto","font":"serif","fallback":["sans-serif"]}}}'
```
### Recursos

Los recursos son los archivos de la escena: imágenes, videos, sonidos, fuentes, tablas de color `.cube` y modelos `.glb`. Viven en la carpeta `assets/` y se declaran en `/scene/assets`. La lista está en el panel **Recursos** de la app. Para efectos y scripts que se importan como archivos, mira [Shaders](acciones.md) y [Scripts y plantillas](acciones.md).

#### `import`

Copia un archivo a `assets/` y lo declara como recurso. Imprime el id del recurso (por defecto, el nombre del archivo sin extensión). Si ya hay un archivo igual en `assets/`, lo reutiliza. Admite imágenes, videos, sonidos, fuentes, tablas `.cube` y `.glb`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `file` | texto (obligatorio) | Archivo que se copia, con su ruta desde donde ejecutas el comando. |
| `id` | texto | Id del recurso. Si ya existe, la acción falla. |
| `linear` | sí/no | `true` si la imagen guarda datos (un mapa de normales, una máscara) y no colores; así no se corrige su color. Solo para imágenes. Por defecto `false`. |
| `atlas` | lista de 2 enteros | `[filas, columnas]` de una hoja de sprites regular, de 1 a 256 cada una. Solo para imágenes. Para cortes irregulares usa [`sprites`](#sprites). |

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena import '{"file":"foto.png","id":"mapa","linear":true}'
wprs-ctl scene act mi-escena import '{"file":"hoja.png","id":"hoja","atlas":[2,4]}'
wprs-ctl scene act mi-escena import '{"file":"sonido.ogg"}'
wprs-ctl scene act mi-escena import '{"file":"modelo.glb","id":"modelo"}'
wprs-ctl scene act mi-escena import '{"file":"video.webm"}'
```

#### `asset`

Cambia cómo se guarda un recurso de imagen. Hace una de tres cosas, según los argumentos: convertirlo a textura comprimida (KTX2), hacerlo la portada de lo que reproduce el reproductor de medios, o mostrar esa portada en un campo de la escena. En la app, **Convertir a KTX2…** está en cada imagen del panel **Recursos**.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `asset` | texto (obligatorio) | Id del recurso. |
| `ktx2` | sí/no | `true` convierte la imagen en textura KTX2 con sus mipmaps, escribe `assets/NOMBRE.ktx2` y apunta el recurso a ese archivo. No sirve en una imagen que ya es KTX2. Por defecto `false`. |
| `codec` | texto | Con `ktx2`: `uastc` (más nítido, es lo normal) o `etc1s` (más pequeño). |
| `quality` | entero | Con `ktx2`: calidad de 1 a 100. Por defecto 80. |
| `cube` | sí/no | Con `ktx2`: la imagen es un panorama y se convierte en un cubo de seis caras. |
| `faces` | lista de 6 textos | Con `ktx2`: ids de seis imágenes cuadradas del mismo tamaño (`+X`, `-X`, `+Y`, `-Y`, `+Z`, `-Z`, en ese orden) que forman un cubo. Se escribe sobre `asset`. No se combina con `cube`. |
| `cover` | texto | `current` hace del recurso la portada de lo que suena ahora, `previous` la de lo que sonó antes, y `none` lo vuelve una imagen normal. |
| `placeholder` | texto | Con `cover`: la imagen fija que se ve mientras no hay portada (un id de recurso o un archivo de la carpeta). Es obligatoria si el recurso es nuevo. |
| `field` | texto | Con `cover` (`current` o `previous`): ruta de un campo de la escena, como `/scene/layers/0/image`. Pone allí la portada, y si no hay ninguna la crea con el nombre `asset`. |

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena asset '{"asset":"foto","ktx2":true,"codec":"etc1s","quality":50}'
wprs-ctl scene act mi-escena import '{"file":"foto.png","id":"panorama"}'
wprs-ctl scene act mi-escena asset '{"asset":"panorama","ktx2":true,"cube":true}'
wprs-ctl scene act mi-escena import '{"file":"foto.png","id":"portada"}'
wprs-ctl scene act mi-escena asset '{"asset":"portada","cover":"current","placeholder":"foto"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"image","source":"foto","id":"cielo"}'
wprs-ctl scene act mi-escena asset '{"asset":"anterior","cover":"previous","placeholder":"foto","field":"/scene/layers/0/image"}'
```

#### `sprites`

Corta una imagen en cuadros para una animación: con una rejilla o con rectángulos dibujados a mano. Una capa de imagen que use el recurso reproduce los cuadros. Una variante por gesto; todas piden `asset`, el id de una imagen. Un recurso tiene como máximo 256 cuadros. Un rectángulo es `[x, y, ancho, alto]` en píxeles de la imagen, y debe caber dentro de ella.

##### variante `grid`

Corta una rejilla regular. Reemplaza los cuadros anteriores.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `asset` | texto (obligatorio) | Id de la imagen. |
| `columns` | entero (obligatorio) | Columnas de la rejilla. |
| `rows` | entero (obligatorio) | Filas de la rejilla. |
| `count` | entero | Cuántos cuadros usar; por defecto todos. |
| `margin` | entero | Borde de la imagen sin usar, en píxeles. Por defecto 0. |
| `spacing` | entero | Espacio entre cuadros, en píxeles. Por defecto 0. |
| `order` | texto | Orden de lectura: `rows` (por filas), `columns` (por columnas), `rows_reversed`, `columns_reversed`. Por defecto `rows`. |

```sh
wprs-ctl scene act mi-escena import '{"file":"hoja.png","id":"hoja"}'
wprs-ctl scene act mi-escena sprites '{"grid":{"asset":"hoja","columns":4,"rows":2}}'
wprs-ctl scene act mi-escena sprites '{"grid":{"asset":"hoja","columns":4,"rows":2,"count":6,"order":"columns"}}'
```

##### variante `rects`

Define todos los cuadros a la vez con rectángulos.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `asset` | texto (obligatorio) | Id de la imagen. |
| `rects` | lista de listas de 4 enteros (obligatorio) | Los cuadros, en orden. |

```sh
wprs-ctl scene act mi-escena import '{"file":"hoja.png","id":"hoja"}'
wprs-ctl scene act mi-escena sprites '{"rects":{"asset":"hoja","rects":[[0,0,100,90],[100,0,100,90]]}}'
```

##### variante `add`

Añade un cuadro al final.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `asset` | texto (obligatorio) | Id de la imagen. |
| `rect` | lista de 4 enteros (obligatorio) | El cuadro nuevo. |

```sh
wprs-ctl scene act mi-escena import '{"file":"hoja.png","id":"hoja"}'
wprs-ctl scene act mi-escena sprites '{"add":{"asset":"hoja","rect":[200,0,100,90]}}'
```

##### variante `change`

Cambia el rectángulo de un cuadro.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `asset` | texto (obligatorio) | Id de la imagen. |
| `index` | entero (obligatorio) | Número del cuadro (0 es el primero). |
| `rect` | lista de 4 enteros (obligatorio) | El rectángulo nuevo. |

```sh
wprs-ctl scene act mi-escena import '{"file":"hoja.png","id":"hoja"}'
wprs-ctl scene act mi-escena sprites '{"grid":{"asset":"hoja","columns":4,"rows":2}}'
wprs-ctl scene act mi-escena sprites '{"change":{"asset":"hoja","index":0,"rect":[0,0,50,50]}}'
```

##### variante `remove`

Quita un cuadro.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `asset` | texto (obligatorio) | Id de la imagen. |
| `index` | entero (obligatorio) | Número del cuadro. |

```sh
wprs-ctl scene act mi-escena import '{"file":"hoja.png","id":"hoja"}'
wprs-ctl scene act mi-escena sprites '{"grid":{"asset":"hoja","columns":4,"rows":2}}'
wprs-ctl scene act mi-escena sprites '{"remove":{"asset":"hoja","index":1}}'
```

##### variante `reorder`

Mueve un cuadro a otra posición.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `asset` | texto (obligatorio) | Id de la imagen. |
| `from` | entero (obligatorio) | Posición actual. |
| `to` | entero (obligatorio) | Posición nueva. |

```sh
wprs-ctl scene act mi-escena import '{"file":"hoja.png","id":"hoja"}'
wprs-ctl scene act mi-escena sprites '{"grid":{"asset":"hoja","columns":4,"rows":2}}'
wprs-ctl scene act mi-escena sprites '{"reorder":{"asset":"hoja","from":0,"to":2}}'
```

##### variante `play`

Fija cómo se reproducen los cuadros.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `asset` | texto (obligatorio) | Id de la imagen. |
| `fps` | número | Cuadros por segundo, de 0.1 a 120. |
| `frames` | entero | Cuántos cuadros se reproducen, de 1 al número de cuadros. |

```sh
wprs-ctl scene act mi-escena import '{"file":"hoja.png","id":"hoja"}'
wprs-ctl scene act mi-escena sprites '{"grid":{"asset":"hoja","columns":4,"rows":2}}'
wprs-ctl scene act mi-escena sprites '{"play":{"asset":"hoja","fps":12,"frames":6}}'
```

##### variante `clear`

Quita el corte: la imagen vuelve a ser una imagen normal.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `asset` | texto (obligatorio) | Id de la imagen. |

```sh
wprs-ctl scene act mi-escena import '{"file":"hoja.png","id":"hoja"}'
wprs-ctl scene act mi-escena sprites '{"grid":{"asset":"hoja","columns":4,"rows":2}}'
wprs-ctl scene act mi-escena sprites '{"clear":{"asset":"hoja"}}'
```

#### `film`

Ajusta una capa de video (o una capa de imagen que muestra un video): recortarlo, cambiar su velocidad o dar a su caja el tamaño real del video. Tres variantes.

##### variante `trim`

Reproduce solo una parte del video. La parte debe caer dentro del video y durar al menos 0.05 s.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa. |
| `start` | número (obligatorio) | Segundo donde empieza. |
| `end` | número | Segundo donde termina. Sin él, hasta el final. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"video","file":"video.webm","id":"clip"}'
wprs-ctl scene act mi-escena film '{"trim":{"layer":"clip","start":0.1,"end":0.5}}'
```

##### variante `rate`

Cambia la velocidad.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa. |
| `rate` | número (obligatorio) | Velocidad de 0 a 4; 1 es la normal. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"video","file":"video.webm","id":"clip"}'
wprs-ctl scene act mi-escena film '{"rate":{"layer":"clip","rate":0.5}}'
```

##### variante `fit`

Pone en `size` el tamaño real del video.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layer` | texto (obligatorio) | Id de la capa. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"video","file":"video.webm","id":"clip"}'
wprs-ctl scene act mi-escena film '{"fit":{"layer":"clip"}}'
```

#### `import_assets`

Copia partes de otro paquete de recursos (un `.wpak` de tipo recursos) a la escena. Las partes van a `assets/ID-VERSION/`, se renombran si sus ids ya existen, quedan acreditadas con su licencia y autor, y el paquete se anota en el origen de la escena como obra derivada. Imprime los ids que añadió.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `package` | texto (obligatorio) | Archivo `.wpak` del paquete de recursos. |
| `parts` | lista de textos | Qué partes traer: `layer:ID`, `particles:ID`, `script:RUTA` o `effect:ID`. Vacía (por defecto), todas. |

```sh
wprs-ctl scene new origen
wprs-ctl scene act origen add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act origen manifest '{"credit":{"paths":["assets/foto.png"],"license":"CC-BY-4.0","attribution":"Ana Perez"}}'
wprs-ctl pack new-assets origen partes --part layer:cielo --id org.example.partes --title Partes --license CC-BY-4.0 --attribution "Ana Perez"
wprs-ctl pack build partes -o partes.wpak
wprs-ctl scene new mi-escena
wprs-ctl scene act mi-escena import_assets '{"package":"partes.wpak","parts":["layer:cielo"]}'
```

### Lienzo

#### `canvas`

Cambia el lienzo: su tamaño, el reloj, el brillo, el parallax, la cámara 2D y las ayudas de la vista previa. Es una sola acción con varias variantes. En la app, casi todo está en el inspector del lienzo y en el menú **Lienzo y reloj**.

##### variante `size`

Fija el tamaño de referencia del lienzo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `size` | lista de 2 enteros (obligatorio) | Ancho y alto en píxeles, de 1 a 16384 cada uno. |

```sh
wprs-ctl scene act mi-escena canvas '{"size":{"size":[1280,720]}}'
```

##### variante `clock`

Fija el cuadro que se muestra donde no hay movimiento y la semilla de lo aleatorio.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `poster` | número | Segundo del cuadro fijo (la imagen de la escena sin movimiento), de 0 a 86400. |
| `seed` | entero | Semilla de los efectos procedurales: con la misma, siempre sale igual. |
| `fresh` | sí/no | `true` hace que cada inicio use una semilla nueva. No se combina con `seed`. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena canvas '{"clock":{"poster":2.5,"seed":7}}'
wprs-ctl scene act mi-escena canvas '{"clock":{"fresh":true}}'
```

##### variante `bloom`

Hace que lo brillante «brille» (resplandor). Lo que no se nombra empieza con los valores por defecto la primera vez.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `threshold` | número | Desde qué brillo empieza el resplandor, de 0 a 4. Por defecto 0.8. |
| `strength` | número | Fuerza, de 0 a 8. Por defecto 1. |
| `radius` | número | Alcance en píxeles, de 0 a 128. Por defecto 16. |
| `tint` | texto | Color del resplandor, como `#ffddaa`. |
| `hdr` | lista de 2 números o `"none"` | `[umbral, fuerza]` del resplandor de colores más brillantes que el blanco (umbral de 0 a 16, fuerza de 0 a 8); `"none"` lo quita. |
| `off` | sí/no | `true` quita el resplandor. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena canvas '{"bloom":{"threshold":0.8,"strength":0.6,"radius":8,"tint":"#ffddaa"}}'
wprs-ctl scene act mi-escena canvas '{"bloom":{"hdr":[1.5,0.5]}}'
wprs-ctl scene act mi-escena canvas '{"bloom":{"hdr":"none"}}'
wprs-ctl scene act mi-escena canvas '{"bloom":{"off":true}}'
```

##### variante `parallax`

Hace que la escena se desplace en profundidad según el puntero o la inclinación de un teléfono. Cuánto se mueve cada capa lo decide su profundidad (ver `depth` en [`layer`](#layer)).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `amount` | número | Fuerza general, de 0 a 4. |
| `delay` | número | Retraso suave en segundos, de 0 a 2. |
| `pointer` | número | Cuánto manda el puntero, de 0 a 1. |
| `gyro` | sí/no | `true` también usa la inclinación del teléfono. |
| `offset` | lista de 2 números | Desplazamiento fijo, de -1 a 1 en cada dirección. |
| `off` | sí/no | `true` quita el parallax. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena canvas '{"parallax":{"amount":1.5,"delay":0.2,"pointer":1,"gyro":true}}'
wprs-ctl scene act mi-escena canvas '{"parallax":{"off":true}}'
```

##### variante `camera`

Pone la cámara 2D de la escena (la crea si falta).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `center` | lista de 2 números | Punto del lienzo, en píxeles, que queda en el centro de lo que se ve. |
| `zoom` | número | Zoom de 0.1 a 10. |
| `rotation` | número | Giro en grados, de -360 a 360. |
| `at` | número | Segundo en que se escribe, si el campo tiene claves. |
| `replace` | sí/no | Como en `transform` de [`layer`](#layer). |
| `off` | sí/no | `true` quita la cámara. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena canvas '{"camera":{"center":[960,540],"zoom":1.5,"rotation":5}}'
wprs-ctl scene act mi-escena canvas '{"camera":{"off":true}}'
```

##### variante `camera_grip`

Hace lo que haría arrastrar un asa de la cámara en el lienzo, hasta el punto `to`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `hold` | texto u objeto (obligatorio) | `"move"` pone en `to` el medio de lo que ve la cámara; `{"zoom":{"corner":[1,1]}}` ajusta el zoom para que la esquina `corner` de la pantalla (fracciones) quede sobre la línea del centro a `to`; `"turn"` gira la cámara para que su arriba apunte a `to`. |
| `to` | lista de 2 números (obligatorio) | Punto del lienzo en píxeles. |
| `at`, `replace` | | Como en `camera`. |

```sh
wprs-ctl scene act mi-escena canvas '{"camera_grip":{"hold":"move","to":[800,500]}}'
wprs-ctl scene act mi-escena canvas '{"camera_grip":{"hold":{"zoom":{"corner":[1,1]}},"to":[1500,900]}}'
wprs-ctl scene act mi-escena canvas '{"camera_grip":{"hold":"turn","to":[1500,100]}}'
```

##### variante `camera_view`

Hace que la vista previa muestre toda la escena con el marco de la cámara. Solo cambia la vista del editor.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `whole` | sí/no (obligatorio) | `true` muestra toda la escena, `false` vuelve a lo que ve la cámara. |

```sh
wprs-ctl scene act mi-escena canvas '{"camera_view":{"whole":true}}'
```

##### variante `probe`

Simula un puntero y una inclinación de teléfono en la vista previa, para ver cómo se mueve el parallax. Solo cambia la vista del editor.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `pointer` | lista de 2 números | Posición simulada del puntero como fracción del lienzo, de 0 a 1 en cada lado. |
| `tilt` | lista de 2 números | Inclinación simulada, de -1 a 1 en cada lado. |
| `off` | sí/no | `true` quita la simulación. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena canvas '{"probe":{"pointer":[0.7,0.3],"tilt":[0.2,0]}}'
wprs-ctl scene act mi-escena canvas '{"probe":{"off":true}}'
```
