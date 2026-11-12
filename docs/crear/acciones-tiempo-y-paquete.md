# Acciones: tiempo, paquete, propiedades y scripts

Las acciones y preguntas de las líneas de tiempo y las claves, el paquete, las propiedades de usuario, los scripts, las plantillas y los shaders del paquete. Cómo llamarlas, cómo leer su salida y sus errores está en [acciones](acciones.md).

## Acciones

### Línea de tiempo y claves

Una clave es un valor fijado en un segundo. Cuando un campo tiene claves, la escena calcula lo que vale entre una y otra. En el documento, un campo animado deja de ser un número y pasa a ser `{"keys": [[segundo, valor], ...]}`; una clave puede llevar un tercer elemento con su suavizado. Las claves siguen el reloj de la escena, o el de una línea de tiempo si el campo dice `"timeline"`.

#### `keyframe`

Añade, cambia, mueve o quita una clave de un valor animable (número, lista de números, color, interruptor o texto). Con la primera clave, el campo pasa a ser animado. En la app son la línea de tiempo y el diálogo de la clave.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `pointer` | texto (obligatorio) | Ruta del valor, por ejemplo `/scene/layers/0/opacity`. |
| `time` | número (obligatorio) | Segundo de la clave. |
| `value` | cualquier valor JSON | Valor de la clave. Si falta, se usa el valor que el campo tiene en ese segundo. |
| `ease` | texto o lista de 4 números | Suavizado desde esta clave hasta la siguiente: `linear`, `step`, `hold`, o una de las curvas `in_`, `out_` o `in_out_` seguidas de `sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` o `bounce` (por ejemplo `in_out_sine`); o `[x1, y1, x2, y2]`, una curva de Bézier. |
| `timeline` | texto | Id de una línea de tiempo que ya existe (ver [`timeline`](#timeline)): la animación sigue esa línea y no el reloj de la escena. |
| `remove` | sí/no | `true` quita la clave de `time`. Por defecto `false`. Si quitas la última, el campo vuelve a ser un valor fijo. |
| `from` | número | Mueve la clave que está en el segundo `from` al segundo `time`, con su valor y suavizado. Falla si ya hay otra clave en `time`. |
| `base` | cualquier valor JSON | Hace las claves relativas: se suman a este valor (un número, una lista de números o un color). |
| `absolute` | sí/no | `true` quita `base` y deja los valores tal cual. Por defecto `false`. |
| `in_handle` | lista de 2 números | Asa de Bézier antes de la clave: `[segundos, valor]`. Reemplaza el suavizado. |
| `out_handle` | lista de 2 números | Asa de Bézier después de la clave. Reemplaza el suavizado. |
| `straighten` | sí/no | `true` endereza la curva que sale de esta clave (quita su asa de salida o su suavizado y el asa de entrada de la siguiente). Va solo, con `pointer` y `time`. Por defecto `false`. |

Un asa no puede llegar más lejos que la clave vecina. Para ver el resultado de las curvas, mira la pregunta [`curves`](#curves).

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":0,"value":0}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"value":1,"ease":"in_out_sine"}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":4,"value":0.2,"ease":[0.4,0,0.2,1]}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"in_handle":[0.5,0.1],"out_handle":[0.5,0]}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"straighten":true}'
```

Para mover una clave de un segundo a otro, quitarla y trabajar con claves relativas:

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":0,"value":0}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"value":1}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":3,"from":2}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":3,"remove":true}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/transform/rotation","time":0,"value":0,"base":10}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/transform/rotation","time":1,"value":5,"absolute":true}'
```

Para animar con una línea de tiempo propia:

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena timeline '{"id":"vuelta","add":true,"duration":3}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/transform/position","time":0,"value":[0,0],"timeline":"vuelta"}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/transform/position","time":3,"value":[100,0],"timeline":"vuelta"}'
```

#### `timeline`

Añade, cambia, renombra o quita una línea de tiempo y sus eventos. Una línea de tiempo es un reloj propio de la escena (con duración, repetición, velocidad y un disparador que la arranca) al que se pueden atar valores animados. Imprime el id al añadirla (hasta 64 por escena). En la app, el menú **Líneas de tiempo** y **Editar la línea de tiempo y sus eventos…**.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la línea. |
| `add` | sí/no | `true` crea una línea nueva (de 5 s si no das duración). Por defecto `false`. |
| `remove` | sí/no | `true` la quita; todo lo que animaba vuelve a seguir el reloj de la escena. Por defecto `false`. |
| `to` | texto | Id nuevo. Lo que la sigue se actualiza. |
| `duration` | número | Duración en segundos. No se combina con `frames`. |
| `frames` | entero | Duración en cuadros, a la velocidad `fps`. |
| `fps` | número | Cuadros por segundo para `frames`, de 1 a 240. |
| `loop` | texto | `once` (una vez), `repeat` (se repite) o `pingpong` (va y vuelve). |
| `wrap` | sí/no | Solo con `repeat`: al repetirse, el último valor se une con el primero. |
| `speed` | número | Velocidad de -100 a 100; 1 es la normal. |
| `delay` | número | Espera en segundos antes de empezar. |
| `autoplay` | sí/no | Si arranca sola al cargar la escena. |
| `trigger` | texto | Qué la arranca: `start`, `beat`, `prop:ID` (una propiedad de usuario), `event:NOMBRE` (un evento de otra línea o de un clip), `click:CAPA`, `enter:CAPA` o `leave:CAPA`. Un texto vacío lo quita. |
| `add_events` | lista de objetos | Eventos nuevos: `{"t": segundo, "name": nombre}`; el nombre lleva de 1 a 32 letras minúsculas, dígitos o `_`. Hasta 64, dentro de la duración. |
| `drop_events` | lista de textos | Eventos que se quitan, por nombre (todos los de ese nombre) o `nombre@segundo` (solo el de ese segundo). |
| `clear_events` | sí/no | `true` quita todos los eventos antes que nada. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena timeline '{"id":"vuelta","add":true,"duration":3}'
wprs-ctl scene act mi-escena timeline '{"id":"vuelta","loop":"pingpong","speed":2,"delay":0.5,"trigger":"start","add_events":[{"t":1,"name":"medio"},{"t":2,"name":"casi"}]}'
wprs-ctl scene act mi-escena timeline '{"id":"vuelta","drop_events":["casi"]}'
wprs-ctl scene act mi-escena timeline '{"id":"vuelta","clear_events":true}'
wprs-ctl scene act mi-escena timeline '{"id":"vuelta","to":"giro"}'
wprs-ctl scene act mi-escena timeline '{"id":"giro","remove":true}'
```

```sh
wprs-ctl scene act mi-escena timeline '{"id":"corta","add":true,"frames":60,"fps":30}'
wprs-ctl scene act mi-escena timeline '{"id":"corta","loop":"repeat","wrap":true}'
```

### Paquete

La ficha del paquete (`/manifest`): quién lo hizo, con qué licencia, qué permisos y entradas usa, cómo se ve. Para las propiedades que elige el usuario, mira [Propiedades de usuario](#propiedades-de-usuario).

#### `manifest`

Edita la ficha del paquete. Es una sola acción con varias variantes: se pasa un objeto con el nombre de la variante y dentro sus argumentos. Cada variante rechaza lo que las comprobaciones de la ficha rechazan, y dice qué campo falla.

##### variante `origin`

Dice de dónde viene el paquete. Reemplaza el origen anterior.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `kind` | texto (obligatorio) | `original` (lo hiciste tú desde cero), `derived` (parte de obras de otros), `imported` (traído de otro lugar) o `provider` (de un proveedor en línea). |
| `derived_from` | lista de objetos | Con `derived`: obras de las que parte, cada una con `author` (obligatorio) y, si quieres, `title`, `url` y `license`. |
| `provider` | objeto | Con `provider`: `name` (minúsculas, dígitos o `-`), `item_url` (una dirección `https://`), `credit` (texto sin `<` ni `>`) y, si quieres, `terms_url`. |
| `source_url` | texto | Dirección de donde se trajo. |

```sh
wprs-ctl scene act mi-escena manifest '{"origin":{"kind":"derived","derived_from":[{"author":"Ana Perez","title":"Aurora","url":"https://example.org/aurora","license":"CC-BY-4.0"}]}}'
wprs-ctl scene act mi-escena manifest '{"origin":{"kind":"imported","source_url":"https://example.org/item"}}'
wprs-ctl scene act mi-escena manifest '{"origin":{"kind":"provider","provider":{"name":"galeria","item_url":"https://example.org/item/1","credit":"Ana Perez"}}}'
wprs-ctl scene act mi-escena manifest '{"origin":{"kind":"original"}}'
```

##### variante `license`

Fija la licencia del paquete.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `license` | texto (obligatorio) | Una expresión SPDX de hasta 256 caracteres, como `CC-BY-4.0`. |

```sh
wprs-ctl scene act mi-escena manifest '{"license":{"license":"CC-BY-4.0"}}'
```

##### variante `tags`

Cambia las etiquetas con las que se encuentra el paquete.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `tags` | lista de textos (obligatorio) | Las etiquetas. |
| `add` | sí/no | `true` las suma a las que hay. |
| `remove` | sí/no | `true` quita estas de las que hay. Sin `add` ni `remove`, reemplaza todas. |

```sh
wprs-ctl scene act mi-escena manifest '{"tags":{"tags":["calma","azul"]}}'
wprs-ctl scene act mi-escena manifest '{"tags":{"tags":["noche"],"add":true}}'
wprs-ctl scene act mi-escena manifest '{"tags":{"tags":["azul"],"remove":true}}'
```

##### variante `authors`

Cambia los autores.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `authors` | lista de textos (obligatorio) | Hasta 64 nombres. Una lista vacía los quita. |

```sh
wprs-ctl scene act mi-escena manifest '{"authors":{"authors":["Ana","Luis"]}}'
```

##### variante `display`

Cambia cómo el paquete llena la pantalla y su color de acento. Según el tipo de paquete valen unos campos u otros: en una escena, `span` y `accent`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `values` | objeto | Campos a fijar: `fit` (`fill`, `fit`, `stretch`, `center`, `tile` o `span`), `zoom` (1 a 8), `position` (dos números de -1 a 1), `background` (un color `#rrggbb`), `span` (`all-outputs` hace que una sola imagen cubra todas las pantallas; `per-output` es para imágenes y videos), `flip` (`none`, `horizontal` o `vertical`) y `accent` (un color `#rrggbb`, o `{"prop": "ID"}` con el id de una propiedad de color). Los campos `fit`, `background` y `flip` valen solo para imágenes y videos, y `zoom` y `position` solo para videos. |
| `clear` | lista de textos | Campos que se quitan. |

```sh
wprs-ctl scene act mi-escena manifest '{"display":{"values":{"span":"all-outputs","accent":"#ff8800"}}}'
wprs-ctl scene act mi-escena manifest '{"display":{"clear":["span"]}}'
```

##### variante `playback`

Cambia cómo se reproduce por defecto el paquete. En una escena o página web, `max_fps` (cuadros por segundo máximos), `volume` (0 a 1) y `muted` (sí/no). En un video también `speed`, `start`, `end` y `loop`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `values` | objeto | Campos a fijar. |
| `clear` | lista de textos | Campos que se quitan. |

```sh
wprs-ctl scene act mi-escena manifest '{"playback":{"values":{"max_fps":30,"volume":0.5,"muted":true}}}'
wprs-ctl scene act mi-escena manifest '{"playback":{"clear":["muted"]}}'
```

##### variante `web`

Solo para paquetes de página web (`wprs-ctl scene new CARPETA --from web`). Ajusta la página: `preload` (un archivo del paquete que se ejecuta antes de la página), `background` (el color que se ve mientras carga) y `storage_kib` (cuánto puede guardar, en KiB).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `values` | objeto | Campos a fijar. |
| `clear` | lista de textos | Campos que se quitan. |

```sh
wprs-ctl scene new mi-escena --from web
wprs-ctl scene act mi-escena manifest '{"web":{"values":{"background":"#202020","storage_kib":64}}}'
wprs-ctl scene act mi-escena manifest '{"web":{"clear":["storage_kib"]}}'
```

##### variante `network`

Solo para paquetes de página web. Lista los orígenes `https` a los que la página puede conectarse; con orígenes en la lista, la entrada `network` se declara sola.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `origins` | lista de textos (obligatorio) | Direcciones como `https://example.org`. |
| `add` | sí/no | `true` las suma a las que hay. |
| `remove` | sí/no | `true` quita estas. Sin `add` ni `remove`, reemplaza todas. |

```sh
wprs-ctl scene new mi-escena --from web
wprs-ctl scene act mi-escena manifest '{"network":{"origins":["https://example.org"],"add":true}}'
wprs-ctl scene act mi-escena manifest '{"network":{"origins":["https://example.org"],"remove":true}}'
```

##### variante `inputs`

Declara qué entradas lee el paquete, para que el sistema le dé acceso.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `inputs` | lista de textos (obligatorio) | Entradas: `audio`, `pointer`, `clicks`, `media` o `network`. |
| `add` | sí/no | `true` las suma a las que hay. |
| `remove` | sí/no | `true` quita estas. Sin `add` ni `remove`, reemplaza todas. |

```sh
wprs-ctl scene act mi-escena manifest '{"inputs":{"inputs":["pointer"],"add":true}}'
```

##### variante `requires`

Declara otros paquetes que este necesita (por ejemplo, uno de recursos).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `packages` | lista de objetos (obligatorio) | Cada uno con `id` y, si quieres, `version`. |
| `add` | sí/no | `true` los suma a los que hay. |
| `remove` | sí/no | `true` quita estos. Sin `add` ni `remove`, reemplaza todos. |

```sh
wprs-ctl scene act mi-escena manifest '{"requires":{"packages":[{"id":"org.example.base","version":"1.0.0"}],"add":true}}'
```

##### variante `preset`

Solo para paquetes de ajustes (`wprs-ctl scene new CARPETA --from preset`), que guardan valores para las propiedades de otro paquete.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `base` | objeto | El paquete al que se aplican los ajustes: `id` y, si quieres, `version`. |
| `values` | objeto | Valores de propiedades, por id. |
| `clear` | lista de textos | Ids de propiedades cuyo valor se quita. |
| `files` | objeto | Archivos que se dan a propiedades de tipo archivo: id de la propiedad y ruta del archivo. |

```sh
wprs-ctl scene new mi-escena --from preset
wprs-ctl scene act mi-escena manifest '{"preset":{"base":{"id":"org.example.base","version":"1.0.0"},"values":{"brillo":2}}}'
```

##### variante `credit`

Acredita archivos del paquete con su licencia y autor. Se juntan en un mismo crédito los que tienen la misma licencia, autor y dirección. Para ver qué falta por acreditar, mira la pregunta [`credits`](#credits).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `paths` | lista de textos (obligatorio) | Archivos del paquete (`assets/foto.png`) o carpetas (terminadas en `/`). Hasta 256. Deben existir. |
| `license` | texto (obligatorio) | Licencia SPDX. |
| `attribution` | texto (obligatorio) | Quién lo hizo. |
| `url` | texto | Dirección `https://` del autor o de la obra. |

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena manifest '{"credit":{"paths":["assets/foto.png"],"license":"CC-BY-4.0","attribution":"Ana Perez","url":"https://example.org/ana"}}'
```

##### variante `uncredit`

Quita archivos de los créditos. Un crédito que se queda sin archivos desaparece.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `paths` | lista de textos (obligatorio) | Archivos o carpetas. |

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena manifest '{"credit":{"paths":["assets/foto.png"],"license":"CC-BY-4.0","attribution":"Ana Perez"}}'
wprs-ctl scene act mi-escena manifest '{"uncredit":{"paths":["assets/foto.png"]}}'
```

##### variante `text`

Escribe un texto de la tabla de textos del paquete (`strings`) en un idioma. Los textos de la ficha o de la escena que valen `@clave` lo leen. Para ver la tabla, mira la pregunta [`strings`](#strings).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `key` | texto (obligatorio) | Clave: hasta 64 letras, dígitos, `_`, `.` o `-`. Las que empiezan por `wprs.` son del motor. |
| `language` | texto (obligatorio) | Idioma, como `en` o `pt-BR`. |
| `text` | texto | El texto. Vacío o ausente, lo quita. |

```sh
wprs-ctl scene act mi-escena manifest '{"text":{"key":"saludo","language":"en","text":"Hello"}}'
wprs-ctl scene act mi-escena manifest '{"text":{"key":"saludo","language":"es","text":"Hola"}}'
```

##### variante `locale`

Decide si los textos de un idioma viven en la ficha o en su propio archivo `locale/IDIOMA.json`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `language` | texto (obligatorio) | Idioma. |
| `file` | sí/no (obligatorio) | `true` los pasa a su archivo, `false` los devuelve a la ficha. |

```sh
wprs-ctl scene act mi-escena manifest '{"text":{"key":"saludo","language":"es","text":"Hola"}}'
wprs-ctl scene act mi-escena manifest '{"locale":{"language":"es","file":true}}'
```

##### variante `add_file`

Solo para paquetes de página web, imagen o video: copia un archivo a la carpeta del paquete.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `file` | texto (obligatorio) | Archivo que se copia. |
| `to` | texto | Carpeta del paquete donde dejarlo. Sin ella, en la raíz. |

```sh
wprs-ctl scene new mi-escena --from web
wprs-ctl scene act mi-escena manifest '{"add_file":{"file":"foto.png","to":"img"}}'
```

##### variante `media`

Solo para paquetes de una imagen o de un video: copia el archivo y lo pone como el que se muestra. Como `scene new` solo crea escenas, páginas web y ajustes, el ejemplo convierte antes el paquete en uno de imagen.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `file` | texto (obligatorio) | La imagen o el video. Debe ser del tipo del paquete. |

```sh
wprs-ctl scene act mi-escena set '{"pointer":"/manifest/kind","value":"image"}'
wprs-ctl scene act mi-escena manifest '{"media":{"file":"foto.png"}}'
```

#### `translate`

Escribe, añade o quita el texto de un campo en un idioma. Sirve para los textos de la ficha (`/manifest/title`, `/manifest/description`, `/manifest/changes`), las etiquetas, ayudas y opciones de las propiedades de usuario, y los textos de reserva de los medios. No sirve para el texto de una capa `text` (ese se cambia con [`set`](acciones-capas.md#set)). Al escribir en un idioma distinto del del paquete, un texto simple se convierte en un texto con varios idiomas. En la app, el campo de idiomas junto a cada texto.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `pointer` | texto (obligatorio) | Ruta del campo, por ejemplo `/manifest/title`. |
| `language` | texto (obligatorio) | Idioma, como `es` o `pt-BR`. Vacío es el texto base. |
| `text` | texto | El texto. Vacío o ausente, quita ese idioma (no se puede quitar el último del título). |
| `add` | sí/no | `true` añade el idioma partiendo del texto en el idioma del paquete, y no toca uno que ya existe. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena translate '{"pointer":"/manifest/title","language":"es","text":"Mi escena"}'
wprs-ctl scene act mi-escena translate '{"pointer":"/manifest/title","language":"fr","add":true}'
wprs-ctl scene act mi-escena translate '{"pointer":"/manifest/title","language":"fr"}'
```

#### `thumbnail`

Pone la imagen con que se muestra el paquete en la biblioteca, o su vista animada. Copia el archivo a `assets/` y lo anota en la ficha. Imprime la ruta que usó. Para hacer una captura de la escena en vez de dar un archivo, mira [`shot`](#shot).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `file` | texto (obligatorio) | Archivo de imagen. Si pasa de 1024 píxeles o de 1 MiB, se reduce. |
| `animated` | sí/no | `true` lo usa como vista animada: debe ser un gif o webp de hasta 640 píxeles de lado, 15 s y 8 MiB. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena thumbnail '{"file":"foto.png"}'
wprs-ctl scene act mi-escena shot '{"recording":{"seconds":1,"fps":5}}'
wprs-ctl scene act mi-escena thumbnail '{"file":"mi-escena/assets/preview-animated.gif","animated":true}'
```

#### `shot`

Dibuja la escena para guardar una captura como miniatura, grabar un gif como vista animada, o medir cuánto parpadea. Solo para escenas. Tres variantes.

##### variante `still`

Guarda un cuadro como miniatura (de hasta 1024 píxeles).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `time` | número | Segundo de la escena. Sin él, el cuadro fijo de la escena (`poster`, ver `clock` en [`canvas`](acciones-capas.md#canvas)). |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid"}'
wprs-ctl scene act mi-escena shot '{"still":{"time":1}}'
```

##### variante `recording`

Graba un gif de hasta 640 píxeles y 8 MiB (se reduce hasta que cabe) como vista animada.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `from` | número | Segundo donde empieza. Por defecto 0. |
| `seconds` | número | Duración, de 0.1 a 15. Por defecto 4. |
| `fps` | entero | Cuadros por segundo, de 1 a 30. Por defecto 10. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid"}'
wprs-ctl scene act mi-escena shot '{"recording":{"from":0,"seconds":1,"fps":5}}'
```

##### variante `flashes`

Mide cuánto parpadea la escena. El resultado solo vive en el proceso que lo midió, así que desde la terminal es mejor usar la pregunta [`flashes`](#flashes) con `"run": true`, que mide y contesta en el mismo paso.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `from` | número | Segundo donde empieza. Por defecto 0. |
| `seconds` | número | Duración, de 0.1 a 60. Por defecto 8. |
| `fps` | entero | Cuadros por segundo, de 1 a 30. Por defecto 15. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid"}'
wprs-ctl scene act mi-escena shot '{"flashes":{"seconds":1,"fps":5}}'
```

#### `upgrade`

Pasa la ficha de un paquete hecho con el formato antiguo (`wallpaper-rs.wpak/1`) al formato actual. Lo que no das queda por completar. Si la ficha ya está en el formato actual, la acción se rechaza. Para probarlo sin un paquete antiguo, el ejemplo marca primero la ficha como antigua.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `rating` | texto | Clasificación de contenido: `general` o `mature`. |
| `reasons` | lista de textos | Motivos de una clasificación `mature`: `sexual`, `violence`, `gore` o `language`. |
| `photosensitive` | sí/no | `true` avisa que la escena puede dañar a personas sensibles a los destellos. |
| `derived_from` | lista de textos | Autores de las obras de las que parte el paquete. |

```sh
wprs-ctl scene act mi-escena set '{"pointer":"/manifest/schema","value":"wallpaper-rs.wpak/1"}'
wprs-ctl scene act mi-escena upgrade '{"rating":"mature","reasons":["language"],"photosensitive":false}'
```

### Propiedades de usuario

Las propiedades de usuario son los ajustes que la persona que usa el fondo puede cambiar (deslizadores, interruptores, colores...). Se guardan en `/manifest/properties`. En la app, el editor de propiedades del paquete.

#### `add_property`

Añade una propiedad nueva, al final de su grupo. Empieza válida, con valores por defecto. Imprime su id.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `kind` | texto (obligatorio) | Tipo: `slider` (deslizador de 0 a 100), `bool` (interruptor), `color`, `combo` (lista con dos opciones, `option_1` y `option_2`, con etiqueta vacía), `text`, `file` (un archivo), `folder` (una carpeta), `shortcut` (un atajo), `button`, `group` (agrupa otras), `label` (un texto) o `divider` (una línea). |
| `id` | texto | Id. Por defecto `tipo_1`, `tipo_2`... |
| `group` | texto | Id de un grupo (otra propiedad de tipo `group`) donde ponerla. |
| `label` | texto | Etiqueta que ve el usuario. |

```sh
wprs-ctl scene act mi-escena add_property '{"kind":"group","id":"luces"}'
wprs-ctl scene act mi-escena add_property '{"kind":"slider","id":"brillo","label":"Brillo","group":"luces"}'
wprs-ctl scene act mi-escena add_property '{"kind":"bool"}'
wprs-ctl scene act mi-escena add_property '{"kind":"color","id":"tinte"}'
wprs-ctl scene act mi-escena add_property '{"kind":"combo","id":"modo"}'
wprs-ctl scene act mi-escena add_property '{"kind":"text"}'
wprs-ctl scene act mi-escena add_property '{"kind":"file"}'
wprs-ctl scene act mi-escena add_property '{"kind":"folder"}'
wprs-ctl scene act mi-escena add_property '{"kind":"shortcut"}'
wprs-ctl scene act mi-escena add_property '{"kind":"button"}'
wprs-ctl scene act mi-escena add_property '{"kind":"label"}'
wprs-ctl scene act mi-escena add_property '{"kind":"divider"}'
```

#### `move_property`

Cambia una propiedad de lugar dentro de su grupo, o a otro grupo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la propiedad. |
| `to` | entero (obligatorio) | Posición dentro del grupo (0 es la primera). |
| `group` | texto | Id del grupo de destino. Una cadena vacía la saca de su grupo. |

```sh
wprs-ctl scene act mi-escena add_property '{"kind":"slider","id":"brillo"}'
wprs-ctl scene act mi-escena add_property '{"kind":"bool","id":"activo"}'
wprs-ctl scene act mi-escena move_property '{"id":"activo","to":0}'
```

#### `rename_property`

Cambia el id de una propiedad y corrige todo lo que la nombra: vínculos, disparadores `prop:`, grupos, condiciones y valores de los presets. Los scripts no se reescriben: el comando avisa de cada script que nombra el id viejo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id actual. |
| `to` | texto (obligatorio) | Id nuevo. |

```sh
wprs-ctl scene act mi-escena add_property '{"kind":"slider","id":"brillo"}'
wprs-ctl scene act mi-escena rename_property '{"id":"brillo","to":"luz"}'
```

#### `remove_property`

Quita una propiedad, sus valores en los presets y en las condiciones, y su pertenencia a un grupo. Se rechaza, diciendo dónde, mientras un vínculo, un disparador `prop:` o una expresión de condición la nombre.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la propiedad. |

```sh
wprs-ctl scene act mi-escena add_property '{"kind":"slider","id":"brillo"}'
wprs-ctl scene act mi-escena remove_property '{"id":"brillo"}'
```

#### `property`

Ajusta una propiedad que ya existe: su rango, su paso, su valor por defecto y su lugar. Rechaza un valor por defecto fuera del rango o que no cae en un paso.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la propiedad. |
| `min` | número | Mínimo. Solo deslizadores. |
| `max` | número | Máximo. Solo deslizadores. |
| `step` | número | Paso. Solo deslizadores. |
| `default` | cualquier valor JSON | Valor por defecto, del tipo de la propiedad. |
| `order` | entero | Lugar dentro de su grupo, como `to` de `move_property`. |

```sh
wprs-ctl scene act mi-escena add_property '{"kind":"slider","id":"luz"}'
wprs-ctl scene act mi-escena property '{"id":"luz","min":0,"max":10,"step":0.5,"default":3}'
wprs-ctl scene act mi-escena property '{"id":"luz","order":0}'
```

#### `condition`

Decide cuándo se muestra una propiedad o cuándo se puede cambiar, según el valor de otras. Rechaza una condición que no se puede comprobar (un id que no existe, una comparación entre cosas de distinto tipo, una que depende de sí misma).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la propiedad. |
| `text` | texto | La condición. Una expresión corta con ids de propiedades, números, textos entre comillas, `true`, `false`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `\|\|`, `!` y paréntesis (por ejemplo `activo && modo == "option_1"`), o un objeto JSON escrito como texto, como `{"modo":"option_2"}` (todo debe coincidir). Vacío, quita la condición. Por defecto vacío. |
| `enabled` | sí/no | `false` pone la condición de cuándo se muestra; `true`, la de cuándo se puede cambiar (si no se cumple, aparece apagada). Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_property '{"kind":"bool","id":"activo"}'
wprs-ctl scene act mi-escena add_property '{"kind":"combo","id":"modo"}'
wprs-ctl scene act mi-escena add_property '{"kind":"slider","id":"nivel"}'
wprs-ctl scene act mi-escena condition '{"id":"nivel","text":"activo && modo == \"option_1\""}'
wprs-ctl scene act mi-escena condition '{"id":"nivel","text":"{\"modo\":\"option_2\"}","enabled":true}'
wprs-ctl scene act mi-escena condition '{"id":"nivel","text":""}'
```

#### `add_preset`

Añade un preset: un conjunto de valores de las propiedades que el usuario puede elegir de una vez. Empieza sin valores. Se rellena con `set` en `/manifest/presets/N/values/ID`. Imprime su id.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto | Id. Por defecto `preset_1`, `preset_2`... |
| `label` | texto | Nombre que ve el usuario. |

```sh
wprs-ctl scene act mi-escena add_property '{"kind":"slider","id":"luz"}'
wprs-ctl scene act mi-escena add_preset '{"id":"suave","label":"Suave"}'
wprs-ctl scene act mi-escena set '{"pointer":"/manifest/presets/0/values/luz","value":2}'
```
### Scripts y plantillas

Un script es un módulo de JavaScript del paquete (`scripts/NOMBRE.js`) que la escena lista en `/scene/scripts`. Cada script declara a qué eventos responde (`events`) y qué partes del motor usa (`uses`), para que el paquete pida los permisos justos. El panel **Recursos** de la app los muestra con **Nuevo script…**, su código y **Lo que declaran los scripts**.

#### `import_script`

Copia un módulo `.js` a `scripts/` y lo declara, o crea uno nuevo a partir de una plantilla de arranque. Detecta solo los eventos que maneja y las partes del motor que usa, e imprime un renglón por cada cambio; las `params` (ajustes del script) nunca las propone, se escriben a mano. Se rechaza si la escena pasaría de 1 MiB de scripts o de 64 módulos.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `file` | texto (obligatorio) | Sin plantilla: el archivo `.js` que se copia (ruta desde donde ejecutas el comando). Con plantilla: el nombre del módulo nuevo, que debe terminar en `.js`; con `""` se llama como la plantilla (`empty.js`, `timer.js`...). Si el archivo ya existe, la acción falla. |
| `template` | texto | Plantilla de arranque: `empty` (vacío, con `init` y `update`), `media` (maneja el estado del reproductor de medios), `touch` (responde cuando el puntero está sobre la salida) o `timer` (cuenta segundos). |

```sh
wprs-ctl scene act mi-escena import_script '{"file":"","template":"empty"}'
wprs-ctl scene act mi-escena import_script '{"file":"reloj.js","template":"timer"}'
wprs-ctl scene act mi-escena import_script '{"file":"musica.js","template":"media"}'
wprs-ctl scene act mi-escena import_script '{"file":"tocar.js","template":"touch"}'
```

Con un módulo escrito por ti:

```sh
printf 'on("update", () => {});\nconst c = scene.camera;\n' > mio.js
wprs-ctl scene act mi-escena import_script '{"file":"mio.js"}'
```

#### `script`

Maneja los scripts de la escena. Es una sola acción con varias variantes. Todas piden `path`, la ruta del módulo dentro del paquete, como `scripts/mio.js`.

##### variante `add`

Declara un módulo `.js` que ya está en la carpeta del paquete y la escena aún no lista. Lo declara como `import_script`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `path` | texto (obligatorio) | Ruta del módulo. |

```sh
wprs-ctl scene act mi-escena import_script '{"file":"","template":"empty"}'
wprs-ctl scene act mi-escena script '{"remove":{"path":"scripts/empty.js"}}'
wprs-ctl scene act mi-escena script '{"add":{"path":"scripts/empty.js"}}'
```

##### variante `remove`

Quita el módulo de la lista (el archivo se queda en la carpeta). Se rechaza, diciendo dónde, mientras un valor siga al módulo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `path` | texto (obligatorio) | Ruta del módulo. |

```sh
wprs-ctl scene act mi-escena import_script '{"file":"","template":"empty"}'
wprs-ctl scene act mi-escena script '{"remove":{"path":"scripts/empty.js"}}'
```

##### variante `move`

Cambia el lugar del módulo en la lista, que es el orden en que se ejecutan.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `path` | texto (obligatorio) | Ruta del módulo. |
| `to` | entero (obligatorio) | Posición nueva (0 es el primero). |

```sh
wprs-ctl scene act mi-escena import_script '{"file":"","template":"empty"}'
wprs-ctl scene act mi-escena import_script '{"file":"reloj.js","template":"timer"}'
wprs-ctl scene act mi-escena script '{"move":{"path":"scripts/reloj.js","to":0}}'
```

##### variante `declare`

Pone en `events` y `uses` lo que el código del módulo usa de verdad. Conserva los `uses` que el código no puede mostrar (`clicks`, `geometry`, `animations`, `shortcuts`, `hierarchy`) y los `params`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `path` | texto (obligatorio) | Ruta del módulo. |
| `params` | sí/no | `true` añade también, como números, los ajustes que el código lee como `params.nombre` y el módulo no declara. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena import_script '{"file":"","template":"empty"}'
wprs-ctl scene act mi-escena script '{"write":{"path":"scripts/empty.js","text":"export function update() { const x = params.fuerza; }\n"}}'
wprs-ctl scene act mi-escena script '{"declare":{"path":"scripts/empty.js","params":true}}'
```

##### variante `write`

Guarda el código de un módulo que la escena ya lista. No cambia su declaración: la comprobación de la escena dice qué cambió. Es un paso de **Deshacer**: deshacer y rehacer vuelven a escribir la versión de ese paso.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `path` | texto (obligatorio) | Ruta del módulo. |
| `text` | texto (obligatorio) | El código completo. |

```sh
wprs-ctl scene act mi-escena import_script '{"file":"","template":"empty"}'
wprs-ctl scene act mi-escena script '{"write":{"path":"scripts/empty.js","text":"export function update() {}\n"}}'
```

##### variante `param`

Crea, cambia, renombra o quita un ajuste (`params`) del módulo, que el usuario o la escena pueden dar al script. Hasta 32 por módulo. Un ajuste nuevo empieza con el valor por defecto de su tipo; si cambias el tipo, se pierden `min` y `max`; el valor por defecto debe caber en el rango.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `path` | texto (obligatorio) | Ruta del módulo. |
| `id` | texto (obligatorio) | Nombre del ajuste. |
| `type` | texto | `number`, `bool`, `text`, `color` o `vec2` (dos números). |
| `default` | cualquier valor JSON | Valor por defecto. |
| `min`, `max` | número | Rango, solo para números. |
| `to` | texto | Nombre nuevo; los valores que seguían el ajuste lo siguen. |
| `remove` | sí/no | `true` quita el ajuste. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena import_script '{"file":"","template":"empty"}'
wprs-ctl scene act mi-escena script '{"param":{"path":"scripts/empty.js","id":"velocidad","type":"number","default":2,"min":0,"max":10}}'
wprs-ctl scene act mi-escena script '{"param":{"path":"scripts/empty.js","id":"velocidad","to":"ritmo"}}'
wprs-ctl scene act mi-escena script '{"param":{"path":"scripts/empty.js","id":"ritmo","remove":true}}'
```

#### `uses`

Declara qué partes del motor usa un script, o toma de su código las que faltan. Las partes que se pueden declarar son: `audio`, `pointer`, `clicks`, `media`, `timers`, `storage`, `layers`, `geometry`, `effects`, `particles`, `sounds`, `animations`, `camera`, `bloom`, `math`, `shortcuts`, `console`, `shared`, `hierarchy` y `folders`. Algunas piden una entrada del paquete (`audio`, `pointer`...): si falta, la acción lo dice y hay que declararla antes con la variante `inputs` de [`manifest`](#manifest). Un script que usa `geometry` en una escena sin reservas de formas crea también una reserva `shapes`. Para ver qué declara y qué le falta a cada script, mira la pregunta `uses` (en Preguntas). En la app es **Declarar lo que usa**.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `path` | texto | Ruta de un script de la escena. Sin ella, todos. |
| `add` | lista de textos | Partes que se añaden a lo declarado. |
| `remove` | lista de textos | Partes que se quitan. |

Sin `add` ni `remove`, declara las partes que el código usa y la declaración no tiene.

```sh
wprs-ctl scene act mi-escena import_script '{"file":"","template":"empty"}'
wprs-ctl scene act mi-escena uses '{"path":"scripts/empty.js","add":["timers","math"]}'
wprs-ctl scene act mi-escena uses '{"path":"scripts/empty.js","remove":["math"]}'
wprs-ctl scene act mi-escena script '{"write":{"path":"scripts/empty.js","text":"export function update() { scene.bloom; }\n"}}'
wprs-ctl scene act mi-escena uses '{}'
```

#### `template`

Maneja las plantillas de capas que los scripts repiten (por ejemplo, para disparar muchas copias de una capa). Una plantilla es una capa guardada en `/scene/templates`. Tres variantes. Desde la app: **Hacer plantilla** en una capa; también existe [`duplicate_layer`](acciones-capas.md#duplicate_layer) con `template`.

##### variante `make`

Guarda una o más capas como plantilla. Las capas se quedan en la escena. Valen imagen, texto, forma, color liso, espectro, malla y grupo.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `layers` | lista de textos (obligatorio) | Ids de las capas. Con varias, la plantilla es un grupo con copias de todas. |
| `name` | texto | Nombre de la plantilla. Por defecto `ID_template`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"a"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"shape","id":"b"}'
wprs-ctl scene act mi-escena template '{"make":{"layers":["a","b"],"name":"par"}}'
```

##### variante `rename`

Cambia el nombre de una plantilla.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `name` | texto (obligatorio) | Nombre actual. |
| `to` | texto (obligatorio) | Nombre nuevo. |
| `rewrite` | sí/no | `true` cambia también, en los scripts de la escena, los textos exactos `"nombre"`, `'nombre'` y `` `nombre` ``, en el mismo paso. Los demás renglones que lo nombran no se tocan. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"a"}'
wprs-ctl scene act mi-escena template '{"make":{"layers":["a"],"name":"par"}}'
wprs-ctl scene act mi-escena template '{"rename":{"name":"par","to":"pareja","rewrite":true}}'
```

##### variante `remove`

Quita una plantilla.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `name` | texto (obligatorio) | Nombre de la plantilla. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"a"}'
wprs-ctl scene act mi-escena template '{"make":{"layers":["a"],"name":"par"}}'
wprs-ctl scene act mi-escena template '{"remove":{"name":"par"}}'
```

#### `pool`

Añade, cambia o quita una reserva de formas: el conjunto de formas que un script construye en cada cuadro y que dibujan las capas de tipo `shapes`. En la app, **Reservas de formas** en el panel **Recursos**. Hasta 16 por escena.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `id` | texto (obligatorio) | Id de la reserva: de 1 a 32 letras minúsculas, dígitos o `_`. |
| `add` | sí/no | `true` crea una nueva, con 16 formas de 1024 vértices si no das otras cifras. Por defecto `false`. |
| `remove` | sí/no | `true` la quita. Se rechaza mientras una capa la dibuje (y dice cuáles). Por defecto `false`. |
| `max` | entero | Cuántas formas caben a la vez, de 1 a 256. |
| `max_vertices` | entero | Vértices por forma, de 3 a 65536. |
| `texture` | texto | Id de una imagen de la escena para pintar las formas. |
| `size` | lista de 2 enteros | Tamaño propio de la reserva en píxeles, de 1 a 4096 cada lado. |
| `clear` | lista de textos | Qué se quita: `texture`, `size`. |

```sh
wprs-ctl scene act mi-escena pool '{"id":"formas","add":true}'
wprs-ctl scene act mi-escena pool '{"id":"formas","max":32,"max_vertices":2048}'
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena pool '{"id":"formas","texture":"foto","size":[512,512]}'
wprs-ctl scene act mi-escena pool '{"id":"formas","clear":["texture","size"]}'
wprs-ctl scene act mi-escena add_layer '{"kind":"shapes","source":"formas","id":"mallas"}'
wprs-ctl scene act mi-escena remove_layer '{"id":"mallas"}'
wprs-ctl scene act mi-escena pool '{"id":"formas","remove":true}'
```

### Shaders

Un shader del paquete es un efecto (o un material, o un shader de partículas) que escribes tú: un descriptor JSON que dice qué ajustes tiene, y un archivo WGSL con el código. Viven en `effects/ID.json` y `effects/ID.wgsl`, y se declaran en `/scene/effects`. Una capa los usa como cualquier efecto (ver [`effects`](efectos.md)). Para el código y la forma del descriptor mira [efectos](efectos.md).

#### `shader`

Escribe un efecto o material del paquete: crearlo desde una plantilla, escribir su código o su descriptor, o editar sus ajustes, pasadas y campos. Es una sola acción con cinco variantes. Para leer el código y comprobarlo, mira las preguntas [`shader_source`](#shader_source) y [`shader_check`](#shader_check).

##### variante `new`

Crea un efecto con su descriptor y su WGSL de arranque, listo para usar.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `name` | texto (obligatorio) | Id del efecto. Sin usar y no reservado por el motor. |
| `template` | texto | `color` (cambia el color, es lo normal si no pones nada), `distort` (deforma la imagen), `passes` (efecto en varias pasadas, como un desenfoque), `material` (material de un modelo 3D) o `particle` (para partículas). |

```sh
wprs-ctl scene act mi-escena shader '{"new":{"name":"tinte"}}'
wprs-ctl scene act mi-escena shader '{"new":{"name":"remolino","template":"distort"}}'
wprs-ctl scene act mi-escena shader '{"new":{"name":"desenfoque","template":"passes"}}'
wprs-ctl scene act mi-escena shader '{"new":{"name":"vidrio","template":"material"}}'
wprs-ctl scene act mi-escena shader '{"new":{"name":"chispa","template":"particle"}}'
```

##### variante `write`

Guarda el código WGSL, el descriptor, o los dos. Cada archivo debe ser válido: el descriptor se comprueba antes de guardar (un shader tiene hasta 64 KiB y todos juntos hasta 256 KiB).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `effect` | texto (obligatorio) | Id de un efecto del paquete. |
| `wgsl` | texto | El código completo. |
| `descriptor` | objeto | El descriptor completo; su `id` debe ser el del efecto. |

```sh
wprs-ctl scene act mi-escena shader '{"new":{"name":"tinte"}}'
wprs-ctl scene act mi-escena shader '{"write":{"effect":"tinte","wgsl":"fn color_tinte(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {\n    return color;\n}\n"}}'
wprs-ctl scene act mi-escena shader '{"write":{"effect":"tinte","descriptor":{"id":"tinte","category":"color","fuse":"color","cost":{"taps":1,"passes":1,"scale":1},"params":[{"id":"amount","type":"float","default":1,"min":0,"max":1}]}}}'
```

##### variante `param`

Crea, cambia, renombra, mueve o quita un ajuste del efecto. Si no existe, lo crea (de tipo `float` si no dices otro). Al renombrar, las capas que lo usan se actualizan.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `effect` | texto (obligatorio) | Id del efecto. |
| `id` | texto (obligatorio) | Id del ajuste. |
| `set` | objeto | Campos a fijar: `type` (`float`, `vec2`, `color`, `bool`, `texture`, `lut`, `int`, `enum`, `blend`, `position`, `mask`, `emitters`), `default`, `min`, `max`, `unit`, `label` y `help` (textos por idioma, como `{"en": "Strength"}`), `values`, `channels`, `paint`, `uses`, `yields_to`, `waits_for`, `tools`. |
| `clear` | lista de textos | Campos que se quitan (los mismos, salvo `type` y `default`). |
| `to` | texto | Id nuevo del ajuste. |
| `at` | entero | Lugar del ajuste en la lista (0 es el primero). |
| `remove` | sí/no | `true` quita el ajuste. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena shader '{"new":{"name":"tinte"}}'
wprs-ctl scene act mi-escena shader '{"param":{"effect":"tinte","id":"fuerza","set":{"type":"float","default":0.5,"min":0,"max":2,"label":{"en":"Strength"}}}}'
wprs-ctl scene act mi-escena shader '{"param":{"effect":"tinte","id":"fuerza","to":"energia","at":0}}'
wprs-ctl scene act mi-escena shader '{"param":{"effect":"tinte","id":"energia","clear":["label"]}}'
wprs-ctl scene act mi-escena shader '{"param":{"effect":"tinte","id":"energia","remove":true}}'
```

##### variante `pass`

Edita las pasadas de un efecto en varias pasadas. Sin `index`, añade una nueva al final.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `effect` | texto (obligatorio) | Id del efecto. |
| `index` | entero | Número de la pasada (0 es la primera). |
| `set` | objeto | Campos a fijar: `entry`, `scale`, `inputs`, `output`, `target`, `copy`, `blend`, `vertex`, `grid`, `swap`. |
| `clear` | lista de textos | Campos que se quitan. |
| `remove` | sí/no | `true` quita la pasada `index`. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena shader '{"new":{"name":"desenfoque","template":"passes"}}'
wprs-ctl scene act mi-escena shader '{"pass":{"effect":"desenfoque","index":0,"set":{"scale":0.25}}}'
wprs-ctl scene act mi-escena shader '{"pass":{"effect":"desenfoque","set":{"entry":"fs_desenfoque_extra"}}}'
wprs-ctl scene act mi-escena shader '{"pass":{"effect":"desenfoque","index":0,"clear":["scale"]}}'
wprs-ctl scene act mi-escena shader '{"pass":{"effect":"desenfoque","index":2,"remove":true}}'
```

##### variante `field`

Cambia un campo del descriptor que no es un ajuste ni una pasada.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `effect` | texto (obligatorio) | Id del efecto. |
| `field` | texto (obligatorio) | Uno de: `name`, `category`, `description`, `uses`, `mask_mode`, `cost`, `fuse`, `reach`, `bounded`, `min_fps`, `velocity`, `state`, `solve`, `pace`, `quality`, `targets`, `options`, `compose`. |
| `value` | cualquier valor JSON | El valor nuevo. `name` y `description` son textos por idioma, como `{"en": "Tint"}`. |

```sh
wprs-ctl scene act mi-escena shader '{"new":{"name":"tinte"}}'
wprs-ctl scene act mi-escena shader '{"field":{"effect":"tinte","field":"name","value":{"en":"Tint"}}}'
wprs-ctl scene act mi-escena shader '{"field":{"effect":"tinte","field":"cost","value":{"taps":2,"passes":1,"scale":1}}}'
```

#### `import_effect`

Trae un efecto escrito fuera de la escena: comprueba el descriptor y el WGSL como los comprueba quien abra el paquete, los copia a `effects/ID.json` y `effects/ID.wgsl` y lo declara (si ya existía, lo reemplaza).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `descriptor` | texto (obligatorio) | Archivo `.json` del descriptor. |
| `wgsl` | texto | Archivo `.wgsl`. Sin él, el que está junto al descriptor con el mismo nombre. |

```sh
wprs-ctl scene new origen
wprs-ctl scene act origen shader '{"new":{"name":"tinte"}}'
wprs-ctl scene new mi-escena
wprs-ctl scene act mi-escena import_effect '{"descriptor":"origen/effects/tinte.json"}'
wprs-ctl scene act mi-escena import_effect '{"descriptor":"origen/effects/tinte.json","wgsl":"origen/effects/tinte.wgsl"}'
```

#### `remove_effect`

Quita la declaración de un efecto del paquete (sus archivos se quedan en `effects/`). Se rechaza mientras una capa lo tenga en sus efectos o un modelo lo use como material.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `effect` | texto (obligatorio) | Id del efecto. |

```sh
wprs-ctl scene act mi-escena shader '{"new":{"name":"tinte"}}'
wprs-ctl scene act mi-escena remove_effect '{"effect":"tinte"}'
```

### Deshacer y rehacer

#### `undo`

Deshace el último cambio. Cada acción es un paso. En la app, **Deshacer**. Para ver los pasos: `wprs-ctl scene history mi-escena`.

No tiene argumentos: se pasa `'{}'`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene act mi-escena undo '{}'
```

#### `redo`

Rehace el último cambio deshecho. En la app, **Rehacer**. No tiene argumentos.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene act mi-escena undo '{}'
wprs-ctl scene act mi-escena redo '{}'
```

## Preguntas

### Antes de empezar

Las preguntas se hacen con `wprs-ctl scene ask CARPETA NOMBRE 'JSON'` y contestan con JSON; no cambian nada. Algunas piden un objeto con argumentos, otras un solo valor (como el id de un recurso entre comillas). Las reglas de ids, rutas y coordenadas son las de [las acciones](#antes-de-empezar). Las capas que se copian se piden con [`copy`](acciones-capas.md#copy); desde la terminal hay también `wprs-ctl scene copy CARPETA ID...`.

### Textos, créditos y avisos del paquete

#### `strings`

Contesta con la tabla de textos del paquete: qué idiomas hay, qué claves, en qué idiomas falta cada una, dónde viven los textos y los textos que ya trae el motor. Sirve para saber qué falta traducir antes de publicar. Se escribe con la variante `text` de [`manifest`](#manifest).

No lleva argumentos: se pasa `{}`.

La respuesta tiene: `default` (el idioma base), `languages` (todos los idiomas), `places` (por idioma, `manifest` si sus textos están en la ficha o `file` si están en `locale/IDIOMA.json`), `keys` (por cada clave: `key`, `texts` con el texto de cada idioma, `missing` con los idiomas que le faltan y `used`, que dice si algún texto de la ficha o de la escena la nombra como `@clave`), `engine` (los textos que el motor ya sabe decir, con su `key`, `en` y `es`) y `reads_files` (si el paquete declara que lee archivos de idioma).

```sh
wprs-ctl scene act mi-escena manifest '{"text":{"key":"saludo","language":"en","text":"Hello"}}'
wprs-ctl scene act mi-escena manifest '{"text":{"key":"saludo","language":"es","text":"Hola"}}'
wprs-ctl scene ask mi-escena strings '{}'
```

#### `credits`

Contesta con los archivos del paquete que se pueden acreditar, los créditos que hay y los archivos que no tienen ninguno. Sirve para no publicar archivos de otros sin dar su autor. Se escribe con las variantes `credit` y `uncredit` de [`manifest`](#manifest).

No lleva argumentos: se pasa `{}`.

La respuesta tiene: `assets` (cada archivo con su `path`, su `kind`, su `size` en bytes y `credited`, si algún crédito lo cubre), `credits` (cada uno con `paths`, `license`, `attribution` y `url`) y `missing` (las rutas de los archivos sin crédito).

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena import '{"file":"sonido.ogg"}'
wprs-ctl scene act mi-escena manifest '{"credit":{"paths":["assets/foto.png"],"license":"CC-BY-4.0","attribution":"Ana Perez"}}'
wprs-ctl scene ask mi-escena credits '{}'
```

La última línea da `"missing": ["assets/sonido.ogg"]`.

#### `flashes`

Contesta con el último resultado de medir cuánto parpadea la escena, o con una medición nueva si pides `run`. Sirve para decidir si el paquete debe marcarse como sensible a los destellos. El resultado solo vive mientras la escena no cambie y en el mismo proceso: cada llamada de `wprs-ctl` es un proceso, así que desde la terminal hay que pedir `run`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `run` | sí/no | `true` mide ahora y contesta con el resultado. Si falta, contesta con el último o con `null`. |
| `from` | número | Segundo donde empieza. Por defecto 0. |
| `seconds` | número | Cuánto mide, de 0.1 a 60. Por defecto 8. |
| `fps` | entero | Cuadros por segundo de la medición, de 1 a 30. Por defecto 15. |

La respuesta tiene: `flashing` (si pasó del límite seguro de 3 destellos por segundo), `rate` (el máximo de destellos por segundo), `at` (el segundo del primer parpadeo peligroso, o `null`), y `from`, `seconds` y `fps` de la medición. Si falla, `{"error": ...}`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid"}'
wprs-ctl scene ask mi-escena flashes '{"run":true,"from":0,"seconds":1,"fps":5}'
wprs-ctl scene ask mi-escena flashes '{}'
```

La última línea da `null`: en otro proceso no hay medición guardada.

### Claves, curvas y disparadores

#### `curves`

Contesta con las claves de un valor animado, con sus asas y con la curva ya calculada en puntos, para dibujarla o para revisar un suavizado.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `pointer` | texto (obligatorio) | Ruta de un valor con claves, como `/scene/layers/0/opacity`. |
| `samples` | entero | Cuántos puntos calcular, de 2 a 512. Por defecto 96. |
| `from` | número | Segundo donde empieza la curva. Por defecto, el de la primera clave. |
| `to` | número | Segundo donde termina. Por defecto, el de la última clave (al menos 1 s después de `from`). |
| `picked` | lista de números | Segundos de las claves elegidas, para calcular los límites de la selección. |

La respuesta tiene: `pointer`, `kind` (el tipo del valor, por ejemplo `float`), `unit`, `range` (de qué segundo a cuál se calculó), `keys` (por clave: `t`, `value`, `parts`, `ease`, y `in` y `out`, las asas, con su `dt` en segundos, su `dv` de valor y si el usuario las fijó: `set`), `tracks` (por componente del valor, `name`, `part` y `points`, la lista de pares `[segundo, valor]`) y `bounds` (los límites de todos los valores y de los elegidos). Si el valor no tiene claves, `{"error": ...}`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":0,"value":0}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":2,"value":1}'
wprs-ctl scene ask mi-escena curves '{"pointer":"/scene/layers/0/opacity","samples":3}'
wprs-ctl scene ask mi-escena curves '{"pointer":"/scene/layers/0/opacity","samples":3,"picked":[2]}'
```

La primera de esas preguntas da, entre otras cosas, `"points": [[0.0, 0.0], [1.0, 0.5], [2.0, 1.0]]`.

#### `key_clip`

Convierte claves en un texto que se puede copiar y pegar, o lee un texto así y dice qué claves trae. El texto empieza por `wprs-keys:`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `copy` | lista de objetos | Qué copiar: cada uno con `pointer` (ruta de un valor con claves) y `times` (los segundos de las claves). |
| `read` | texto | Un texto de portapapeles. Si lo das, la pregunta lo lee en vez de copiar. |

Con `copy`, la respuesta tiene `picks` (cada valor con sus `keys`) y `text`, el texto para el portapapeles. Con `read`, la respuesta tiene `picks` con las claves que trae el texto, o `null` si el texto no es de claves. Si una ruta no tiene claves, `{"error": ...}`. Para pegar claves hay una variante de la acción `edit_keys` (ver [acciones](acciones.md)).

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cielo"}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":0,"value":0.2}'
wprs-ctl scene act mi-escena keyframe '{"pointer":"/scene/layers/0/opacity","time":1,"value":1}'
wprs-ctl scene ask mi-escena key_clip '{"copy":[{"pointer":"/scene/layers/0/opacity","times":[0,1]}]}'
wprs-ctl scene ask mi-escena key_clip '{"read":"wprs-keys:{\"picks\":[{\"pointer\":\"/scene/layers/0/opacity\",\"keys\":[[0,0.2]]}]}"}'
wprs-ctl scene ask mi-escena key_clip '{"read":"hola"}'
```

La última pregunta da `{"picks": null}`.

#### `fires`

Contesta con las líneas de tiempo y las capas de sonido que empieza un disparador. Sirve para saber qué ocurre cuando suena un evento o se pulsa algo. El disparador se escribe como en `trigger` de la acción [`timeline`](acciones-tiempo-y-paquete.md#timeline): `start`, `event:NOMBRE`, `click:CAPA`, `prop:ID`...

El argumento es el disparador, como un texto JSON entre comillas.

La respuesta tiene: `trigger`, `timelines` (ids de las líneas que lo tienen como `trigger`) y `sounds` (ids de las capas de sonido cuyo `play_on` es ese disparador).

```sh
wprs-ctl scene act mi-escena timeline '{"id":"uno","add":true,"add_events":[{"t":1,"name":"boom"}]}'
wprs-ctl scene act mi-escena timeline '{"id":"dos","add":true,"trigger":"event:boom"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"sound","file":"sonido.ogg","id":"voz"}'
wprs-ctl scene act mi-escena set '{"pointer":"/scene/layers/0/play_on","value":"event:boom"}'
wprs-ctl scene ask mi-escena fires '"event:boom"'
```

La última pregunta da `{"sounds": ["voz"], "timelines": ["dos"], "trigger": "event:boom"}`.

### Recursos y medios

#### `palette`

Contesta con los colores principales de una imagen, para sacar una paleta a juego con ella.

El argumento es el id de un recurso de imagen, como un texto JSON entre comillas.

La respuesta es una lista de 5 colores `#rrggbb`, o `null` si el recurso no existe o no es una imagen fija.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene ask mi-escena palette '"foto"'
```

#### `clocks`

Contesta con las capas de video que tienen su propio reloj (`playback.own_clock`) y dónde está cada reloj en un segundo de la escena. Sirve para saber en qué cuadro de su video estaría cada una. Solo cuenta los videos que tienen `own_clock` en `true`.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `time` | número | Segundo de la escena. Por defecto 0. |
| `strip` | entero | Cuántas miniaturas pedir de cada video, repartidas por su largo. Por defecto 0, ninguna. |

La respuesta tiene `time` y `clocks`, y cada reloj trae: `layer`, `pointer`, `videos`, `length` (largo del video en segundos), `start`, `end`, `rate`, `loop`, `playing`, `at` (el segundo del video en que está) y `keyed` (si la velocidad tiene claves). Con `strip`, cada reloj trae también `strip`, una lista de `{at, file}` con la miniatura de cada instante.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"video","file":"video.webm","id":"clip"}'
wprs-ctl scene act mi-escena set '{"pointer":"/scene/layers/0/playback","value":{"own_clock":true,"rate":2}}'
wprs-ctl scene ask mi-escena clocks '{"time":0.25}'
```

#### `pools`

Contesta con las reservas de formas de la escena, con las capas que las dibujan y los scripts que las llenan. Sirve para comprobar que cada reserva tiene quien la dibuje y quien la construya. Se editan con la acción [`pool`](acciones-tiempo-y-paquete.md#pool).

No lleva argumentos: se pasa `{}` (o nada).

La respuesta tiene `limit` (cuántas reservas caben, 16) y `pools`, y cada reserva trae: `id`, `pointer`, `max` (formas), `max_vertices`, `texture`, `size`, `layers` (ids de las capas que la dibujan) y `scripts` (los que declaran `geometry`).

```sh
wprs-ctl scene act mi-escena pool '{"id":"formas","add":true}'
wprs-ctl scene act mi-escena add_layer '{"kind":"shapes","source":"formas","id":"mallas"}'
wprs-ctl scene ask mi-escena pools '{}'
```

### Scripts y shaders

#### `script_uses`

Dice en qué líneas de los scripts de la escena aparece un id entre comillas (simples, dobles o de plantilla). Sirve para saber qué scripts siguen nombrando una capa, un sistema de partículas o una plantilla antes de renombrarla o quitarla: la acción `rename` no reescribe los scripts.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `name` | texto | (obligatorio) El id que se busca. |

La respuesta es una lista de `{path, line}`: el script y el número de línea (desde 1). Es una lista vacía si ningún script lo nombra.

```sh
printf 'layers.show("cielo");\n' > uno.js
wprs-ctl scene act mi-escena import_script '{"file":"uno.js"}'
wprs-ctl scene ask mi-escena script_uses '{"name":"cielo"}'
```

#### `uses`

Contesta, por cada script de la escena, qué partes del motor declara, cuáles detecta en su código y cuáles le faltan por declarar. Sirve para comprobar que el paquete pide los permisos que sus scripts necesitan. Lo que falta se arregla con la acción [`uses`](acciones-tiempo-y-paquete.md#uses).

No lleva argumentos: se pasa `{}`.

La respuesta tiene `scripts`, y cada script trae `path`, `pointer`, `declared`, `detected` y `missing`.

```sh
wprs-ctl scene act mi-escena import_script '{"file":"","template":"empty"}'
wprs-ctl scene act mi-escena script '{"write":{"path":"scripts/empty.js","text":"export function update() { scene.bloom; }\n"}}'
wprs-ctl scene ask mi-escena uses '{}'
```

La última pregunta da, para ese script, `"declared": []`, `"detected": ["bloom"]` y `"missing": ["bloom"]`.

#### `shader_source`

Contesta con el descriptor y el código WGSL de un efecto o material del paquete, y con lo que el editor necesita para armar su formulario. Sirve para leer lo que escribió otra persona o lo que se creó con [`shader`](acciones-tiempo-y-paquete.md#shader).

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `effect` | texto (obligatorio) | Id de un efecto del paquete. |

La respuesta tiene: `effect`, `descriptor_path` y `wgsl_path` (dónde viven los archivos), `descriptor` (el texto del descriptor) y `parsed` (el descriptor ya leído), `wgsl` (el código), `stage` (`layer`, `material` o `particle`), `kinds` (los tipos de ajuste que existen), `fields` (los campos que la acción `shader` con `field` puede cambiar), `templates` (las plantillas de `new`) y `tools` (por ajuste, las herramientas de edición que ofrece). Si el efecto no es del paquete, `null`.

```sh
wprs-ctl scene act mi-escena shader '{"new":{"name":"tinte"}}'
wprs-ctl scene ask mi-escena shader_source '{"effect":"tinte"}'
```

#### `shader_check`

Comprueba un efecto del paquete como lo comprueba quien abre el paquete, con su texto guardado o con texto nuevo que aún no guardaste. Dice si es válido y, si no, en qué línea y columna está el error.

| Argumento | Tipo | Qué significa |
|-----------|------|---------------|
| `effect` | texto (obligatorio) | Id de un efecto del paquete. |
| `wgsl` | texto | Código WGSL a probar en vez del guardado. |
| `descriptor` | texto | Descriptor, escrito como texto JSON, a probar en vez del guardado. |

La respuesta tiene: `effect`, `ok` (si es válido), `error` (el mensaje, o `null`), `line` y `column` (dónde, o `null`), `stage`, `units` (el costo relativo), `uses` (las partes del motor que usa) y `handles` (las asas que el lienzo ofrecerá).

```sh
wprs-ctl scene act mi-escena shader '{"new":{"name":"tinte"}}'
wprs-ctl scene ask mi-escena shader_check '{"effect":"tinte"}'
wprs-ctl scene ask mi-escena shader_check '{"effect":"tinte","wgsl":"fn color_tinte(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {\n    return colr;\n}\n"}'
```

La última pregunta da `"ok": false`, `"line": 2` y `"column": 12`, con el aviso de que `colr` no existe.

### Revisión y firma del paquete

#### `review`

Contesta con la revisión del paquete tal como la ve el editor antes de publicar: los problemas, lo que el paquete necesita, en qué plataformas se reproduce y si está firmado. Es la lista de problemas del editor, con las mismas comprobaciones que `scene validate`, más los avisos de plataforma.

No lleva argumentos: se pasa `{}`.

La respuesta tiene `findings` (cada uno con `level`: `error`, `warning` o `note`; `pointer`, `file` y `message`), `needs` y `build_adds` (lo que el paquete necesita y lo que la construcción le añade), `platforms` (por plataforma, `refused` con el motivo y `without` con lo que le falta), `facets` y `signature`.

```sh
wprs-ctl scene ask mi-escena review '{}'
```

#### `signature`

Dice si una carpeta de paquete o un archivo `.wpak` está firmado, y si la firma vale para unas claves.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `keys` | lista de textos | Claves públicas de confianza. Sin ellas, la firma solo se cuenta, no se comprueba. |
| `path` | texto | Carpeta o archivo `.wpak`. Por defecto, la carpeta de la escena. |

La respuesta tiene `state`: `unsigned` (sin firma), `unchecked` (hay firma, pero no se dieron claves; trae `blocks`), `valid` (la firma vale; trae `signers`) o `invalid` (trae `error`).

```sh
wprs-ctl scene ask mi-escena signature '{}'
```

### Propiedades y ajustes

#### `tried`

Contesta con el formulario de las propiedades de usuario tal como se ve al probar un ajuste o unos valores sobre los de fábrica. Sirve para ver qué campos se muestran y con qué valores sin cambiar la escena. En la app, probar un ajuste en la vista previa.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `preset` | texto | Id de un ajuste del paquete. |
| `values` | objeto | Valores que se prueban encima, por id de propiedad. |
| `languages` | lista de textos | Idiomas de las etiquetas. |

La respuesta tiene `fields` (cada campo con su `key`, `kind`, `label`, los límites, `value`, `visible`, `enabled`...), `presets` (los nombres de los ajustes) y `values` (los valores que resultan, ya ajustados a los límites de cada propiedad). Los campos secretos, atajos y botones no se listan.

```sh
wprs-ctl scene act mi-escena add_property '{"kind":"slider","id":"brillo"}'
wprs-ctl scene ask mi-escena tried '{"values":{"brillo":40}}'
```

#### `levels`

Dice qué valor toma una propiedad en cada nivel de calidad y en cuáles tiene un valor propio. El valor propio de un nivel va en el campo `quality` de la propiedad (`/manifest/properties/ID/quality/q1`) y se escribe con [`set`](acciones-capas.md#set).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `property` | texto | (obligatorio) Id de la propiedad. |

La respuesta tiene `property`, `default` y `levels`, una lista con un `{quality, value, own}` por nivel (`own` dice si el nivel fija el valor él mismo). Si la propiedad no existe, es `null`.

```sh
wprs-ctl scene act mi-escena add_property '{"kind":"slider","id":"brillo"}'
wprs-ctl scene act mi-escena set '{"pointer":"/manifest/properties/brillo/quality/q1","value":20}'
wprs-ctl scene ask mi-escena levels '{"property":"brillo"}'
```

#### `base`

Para un paquete de ajustes: contesta con las propiedades del paquete al que se aplica y los valores que el ajuste les da. En la app, el formulario del paquete de ajustes.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `file` | texto | (obligatorio) Carpeta o archivo `.wpak` del paquete base. |

La respuesta tiene `id`, `version`, `fields` (cada propiedad con su `key`, `title`, `range`, `default`, `value` del ajuste, `set`, `pointer` y `widget`) y `files`. Si el paquete base no se puede leer, es `null`.

```sh
wprs-ctl scene new origen
wprs-ctl scene act origen add_property '{"kind":"slider","id":"brillo"}'
wprs-ctl scene new mi-escena --from preset
wprs-ctl scene ask mi-escena base '{"file":"origen"}'
```

#### `media`

Para un paquete de imagen o de vídeo: contesta con su archivo, su tamaño y, si es un vídeo, su largo. En la app, la página del paquete de medios.

No lleva argumentos: se pasa `{}`.

La respuesta tiene `kind` (`image` o `video`), `entry` (el archivo que se espera), `present` (si existe), `bytes`, `file`, `size`, `length` y `aspect`. Con otro tipo de paquete es `null`.

```sh
wprs-ctl scene new mi-escena --from image
wprs-ctl scene ask mi-escena media '{}'
```

### Relojes

#### `clock_sample`

Escribe un patrón de reloj con la hora de un instante y dice cuándo cambia el texto. Sirve para probar un patrón de una capa de texto con reloj.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `pattern` | texto | El patrón, por ejemplo `HH:mm`. Mira `clock_help`. |
| `dates` | sí/no | `true` si el patrón es de fecha. Por defecto `false`. |
| `lang` | texto | Idioma de los nombres de días y meses. Por defecto `en`. |
| `wall` | número | Instante en segundos desde 1970. Por defecto, ahora. |

La respuesta tiene `text` (el patrón ya escrito) y `next` (los segundos que faltan para que cambie).

```sh
wprs-ctl scene ask mi-escena clock_sample '{"pattern":"HH:mm","wall":0}'
```

#### `clock_help`

Lista las piezas que se pueden usar en un patrón de reloj, con un ejemplo de cada una, y los números que da un reloj.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `lang` | texto | Idioma de los ejemplos. Por defecto `en`. |
| `wall` | número | Instante de los ejemplos, en segundos desde 1970. Por defecto, ahora. |

La respuesta tiene `tokens` (cada pieza con `token`, `dates` y `sample`), `numbers` (los valores `day`, `hour`, `minute`, `second`, `weekday` y `month` del instante) y `quote` (el carácter que cita texto literal).

```sh
wprs-ctl scene ask mi-escena clock_help '{"lang":"en","wall":0}'
```

### Historial de cambios

#### `history`

Contesta con los pasos que `undo` deshace y los que `redo` rehace. Es el historial que enseñan las pistas de **Deshacer** y **Rehacer** de la app, y el que lista `scene history`.

No lleva argumentos: se pasa `{}`.

La respuesta tiene `done` (los pasos hechos, del más antiguo al más nuevo), `undone` (los deshechos, el que rehace `redo` primero), `undo` y `redo` (el paso siguiente de cada uno, o `null`) y `dirty` (si hay cambios sin guardar). Cada paso trae `label` (el nombre de la acción) y `text` (el nombre que ve quien edita). El historial se guarda aparte al guardar la escena, así que otra llamada de `wprs-ctl` lo ve.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo"}'
wprs-ctl scene ask mi-escena history '{}'
```
