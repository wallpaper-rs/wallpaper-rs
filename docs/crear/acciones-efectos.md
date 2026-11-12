# Acciones: efectos y máscaras

Las acciones y preguntas de los efectos de una capa y de las máscaras y mapas pintados. Cómo llamarlas, cómo leer su salida y sus errores está en [acciones](acciones.md).

## Acciones

### Efectos de una capa

**Antes de empezar.** Estas acciones comparten tres ideas. Un punto del lienzo es `[x, y]` en píxeles del lienzo, con el origen arriba a la izquierda y la y creciendo hacia abajo (el lienzo mide 1920 por 1080 si no cambias su tamaño). Un color se escribe `"#rrggbb"` o `"#rrggbbaa"`. Los trazos de pincel usan fracciones de la capa: `[0, 0]` es su esquina de arriba a la izquierda y `[1, 1]` la de abajo a la derecha. Los efectos incluidos se listan con `wprs-ctl scene catalog`, y los parámetros de cada uno con la pregunta `rules`.

#### `effects`

Añade, cambia, quita, reordena y enmascara los efectos de una capa, y mueve sus asas y su máscara sobre el lienzo. En la app es la lista **Efectos** de la capa, con **Añadir un efecto** y el cuadro **Máscara del efecto:**. Una capa de sonido o de luz no admite efectos. Un efecto se nombra por su posición en la lista de la capa, que empieza en 0.

Esta acción toma una sola variante a la vez.

##### variante `add`

Añade un efecto al final de la lista de la capa, o en la posición `at`.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa. |
| `effect` | texto | (obligatorio) Nombre del efecto, por ejemplo `blur` o `waves`; puede ser uno incluido o uno del paquete. |
| `name` | texto | Nombre propio para reconocerlo en la lista. Sin valor, no lleva nombre. |
| `at` | entero | Posición en la lista (0 es el primero). Sin valor, va al final. |
| `params` | objeto | Valores iniciales de parámetros, `{"radius": 6}`. Un parámetro que el efecto no tiene, o fuera de su rango, se rechaza. |
| `optional` | sí/no | Si es `true`, el efecto es de los primeros que se dejan fuera cuando el equipo va justo de potencia. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"blur","params":{"radius":6},"name":"suave","optional":true}}'
```

##### variante `set`

Cambia un efecto que ya está en la capa: sus parámetros, si está activo, su nombre o si es opcional. Lo que no nombras se queda como estaba.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa. |
| `index` | entero | (obligatorio) Posición del efecto en la lista. |
| `params` | objeto | Parámetros que cambian. Un valor `null` quita el parámetro y deja el valor por defecto del efecto. |
| `enabled` | sí/no | `false` apaga el efecto sin quitarlo. |
| `name` | texto | Nombre nuevo; un texto vacío lo quita. |
| `optional` | sí/no | Marca o desmarca el efecto como opcional. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"waves"}}'
wprs-ctl scene act mi-escena effects '{"set":{"layer":"image","index":0,"params":{"amplitude":9},"enabled":true,"name":"olas"}}'
```

##### variante `remove`

Quita un efecto de la lista.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa. |
| `index` | entero | (obligatorio) Posición del efecto. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"blur"}}'
wprs-ctl scene act mi-escena effects '{"remove":{"layer":"image","index":0}}'
```

##### variante `move`

Cambia el orden de un efecto. Los efectos se aplican de arriba abajo de la lista.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa. |
| `index` | entero | (obligatorio) Posición actual. |
| `to` | entero | (obligatorio) Posición nueva; no puede pasar del último. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"blur"}}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"vignette"}}'
wprs-ctl scene act mi-escena effects '{"move":{"layer":"image","index":1,"to":0}}'
```

##### variante `grip`

Arrastra hasta un punto del lienzo el asa de un parámetro del efecto: un centro, una esquina, una flecha o un dial. Es lo que hace la app al arrastrar el asa sobre la vista previa. Falla si el parámetro no tiene asa en el lienzo o si ya se mueve con teclas, un sonido o una propiedad.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa. |
| `index` | entero | (obligatorio) Posición del efecto. |
| `param` | texto | (obligatorio) Parámetro cuya asa se arrastra, por ejemplo `center`. |
| `to` | lista de números | (obligatorio) Punto del lienzo `[x, y]` adonde llega el asa. El valor guardado es la posición dentro de la capa, de 0 a 1. |
| `both` | sí/no | Si es `true`, mueve también la otra asa del par (por ejemplo, el final de una línea cuyo comienzo se arrastra) la misma distancia. Por defecto `false`. |
| `at` | número | Segundo en que se mide dónde está la capa, por si se mueve. Por defecto 0. |
| `replace` | sí/no | Si es `true`, pisa un parámetro que seguía otra fuente. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"blur_radial"}}'
wprs-ctl scene act mi-escena effects '{"grip":{"layer":"image","index":0,"param":"center","to":[960,300]}}'
```

##### variante `mask_grip`

Arrastra hasta un punto del lienzo un asa de la máscara en forma de figura: su tamaño, el borde suave, el grosor de un anillo, la esquina redondeada de un rectángulo o el hueco de una estrella. La máscara debe ser una forma (no una imagen ni una capa). La figura se mide desde el centro de la capa, así que el tamaño que resulta es el doble de la distancia del punto al centro.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa. |
| `index` | entero | Posición del efecto cuya máscara se arrastra. Sin valor, es la máscara de la propia capa. |
| `hold` | texto | (obligatorio) Asa que se arrastra: `size` (tamaño), `feather` (borde suave), `width` (grosor, solo anillos), `radius` (esquina, solo rectángulos) o `inner` (hueco, solo estrellas). Una asa que la figura no tiene se rechaza. |
| `to` | lista de números | (obligatorio) Punto del lienzo `[x, y]` adonde llega el asa. |
| `at` | número | Segundo en que se mide dónde está la capa. Por defecto 0. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"blur"}}'
wprs-ctl scene act mi-escena effects '{"mask":{"layer":"image","index":0,"shape":{"ellipse":{"size":[600,400]}},"feather":40}}'
wprs-ctl scene act mi-escena effects '{"mask_grip":{"layer":"image","index":0,"hold":"size","to":[1500,700]}}'
```

##### variante `mask`

Pone, cambia o quita la máscara de un efecto o de la propia capa. Con la máscara, el efecto (o la capa) se ve solo donde la máscara lo permite. Una máscara tiene una sola fuente: una imagen, una capa o una figura. Los cambios que no nombras se conservan.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa. |
| `index` | entero | Posición del efecto. Sin valor, la máscara es la de la capa entera. |
| `texture` | texto | Id de una imagen del paquete como fuente. |
| `from` | texto | Id de otra capa como fuente. |
| `shape` | objeto | Figura como fuente, centrada en la capa y medida en sus píxeles: `{"rect":{"size":[w,h],"radius":r}}`, `{"ellipse":{"size":[w,h]}}`, `{"ring":{"size":[w,h],"width":g}}`, `{"polygon":{"sides":n,"size":d}}`, `{"star":{"points":n,"size":d,"inner":0.5}}` o `{"line":{"to":[x,y],"width":g}}`. |
| `channel` | texto | Canal de la imagen o capa que manda: `r`, `g`, `b`, `a` o `luminance` (brillo). |
| `invert` | sí/no | Si es `true`, el efecto actúa donde la máscara está vacía. |
| `feather` | número | Borde suave, en píxeles. |
| `strength` | número | Fuerza de la máscara, de 0 a 1. |
| `off` | sí/no | Si es `true`, quita la máscara. Por defecto `false`. |

Una máscara nueva necesita una fuente.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"blur"}}'
wprs-ctl scene act mi-escena effects '{"mask":{"layer":"image","index":0,"shape":{"star":{"points":5,"size":300,"inner":0.4}},"feather":20}}'
wprs-ctl scene act mi-escena effects '{"mask":{"layer":"image","index":0,"invert":true}}'
wprs-ctl scene act mi-escena effects '{"mask":{"layer":"image","texture":"foto","channel":"luminance","strength":0.8}}'
wprs-ctl scene act mi-escena effects '{"mask":{"layer":"image","index":0,"off":true}}'
```

#### `effect_rule`

Cambia qué pide un parámetro de un efecto del paquete para animarse: una entrada (el tiempo, el puntero, el sonido o los clics), otro parámetro que debe estar encendido, u otro parámetro que lo releva. La escena solo gasta fotogramas en lo que alguien pide. Solo vale para efectos propios del paquete; los incluidos conservan sus reglas. En la app, la sección **Pide** del efecto. Lo que no nombras se queda; un texto vacío quita esa regla.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `effect` | texto | (obligatorio) Nombre de un efecto del paquete. |
| `param` | texto | (obligatorio) Parámetro del efecto. |
| `uses` | texto | Entrada que pide: `time` (tiempo), `cursor` (puntero), `clicks` (clics) o `audio` (sonido). Vacío, la quita. |
| `waits_for` | texto | Otro parámetro del mismo efecto que debe valer algo distinto de cero para que este pida su entrada. Vacío, lo quita. |
| `yields_to` | texto | Otro parámetro del mismo efecto que, mientras no vale cero, releva a este: este deja de pedir su entrada. Vacío, lo quita. |

```sh
wprs-ctl scene new mi-escena
wprs-ctl scene new-effect mi-escena brillo --template color
wprs-ctl scene act mi-escena effect_rule '{"effect":"brillo","param":"amount","uses":"audio"}'
wprs-ctl scene act mi-escena effect_rule '{"effect":"brillo","param":"amount","waits_for":"color"}'
wprs-ctl scene act mi-escena effect_rule '{"effect":"brillo","param":"amount","uses":"","waits_for":""}'
```

### Máscaras y mapas pintados

Estas acciones pintan una imagen con un trazo de pincel y la ponen en un parámetro del efecto. Cada trazo guarda una imagen nueva en la carpeta del paquete (`masks/`, `maps/`, `ramps/` o `normals/`) y se deshace como un solo paso.

#### `paint_mask`

Pinta un trazo en la imagen que usa un parámetro de un efecto (una máscara o un mapa en gris). Si el parámetro aún no tiene imagen, empieza una de 512 por 512 con el color de relleno del parámetro (negro si no lo declara). En la app es el botón **Pintar** junto al parámetro, con **Borrar**, **Tamaño del pincel**, **Dureza** y **Valor**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `pointer` | texto | (obligatorio) Ruta del parámetro, `/scene/layers/N/effects/M/params/NOMBRE`, con N la posición de la capa y M la del efecto. |
| `points` | lista de listas de números | (obligatorio) Puntos del trazo, `[[x, y], ...]`, como fracciones de la capa (de 0 a 1). De 1 a 4096 puntos. |
| `radius` | número | (obligatorio) Radio del pincel como fracción del lado más corto, entre 0,001 y 0,5. |
| `hardness` | número | Dureza del borde, de 0 (todo suave) a 0,99. Por defecto 0,5. |
| `value` | número | Gris que se pinta, de 0 a 1; 0 borra. Por defecto 1. |
| `channel` | entero | Canal que se pinta: 0 rojo, 1 verde, 2 azul, 3 alfa. Sin valor, pinta rojo, verde y azul a la vez. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"displace"}}'
wprs-ctl scene act mi-escena paint_mask '{"pointer":"/scene/layers/0/effects/0/params/map","points":[[0.2,0.3],[0.8,0.6]],"radius":0.1}'
wprs-ctl scene act mi-escena paint_mask '{"pointer":"/scene/layers/0/effects/0/params/map","points":[[0.5,0.5]],"radius":0.2,"hardness":1,"value":0,"channel":0}'
```

#### `comb_flow`

Peina un mapa de flujo: cada punto que toca el trazo apunta en la dirección en que se arrastró. Sirve para efectos que mueven la imagen según un mapa de dirección, como `flow` o `shake`. Empieza desde un mapa quieto. En la app es **Pintar la dirección**, con **Aplanar** para dejarlo quieto. Los argumentos son los de `paint_mask`, pero `value` es la fuerza del movimiento (0 lo deja quieto) y `channel` no se usa. Necesita al menos dos puntos distintos para tener dirección.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"flow"}}'
wprs-ctl scene act mi-escena comb_flow '{"pointer":"/scene/layers/0/effects/0/params/map","points":[[0.2,0.3],[0.8,0.6]],"radius":0.1,"value":0.8}'
```

#### `ramp`

Hace un degradado a partir de paradas de color y lo pone en un parámetro del efecto. Con el formato `png` guarda una imagen de una fila que se lee de izquierda a derecha (la usan los parámetros `gradient` de efectos como `fire`). Con `cube` guarda una tabla de color que tiñe cada tono según su brillo (la usa `color_lut`). Las paradas se guardan para poder editarlas otra vez. En la app, **Paradas de degradado**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `stops` | lista de objetos | (obligatorio) De 1 a 16 paradas `{"at": 0 a 1, "color": "#rrggbb"}`, en cualquier orden. |
| `pointer` | texto | Ruta del parámetro que recibe el resultado, `/scene/layers/N/effects/M/params/NOMBRE`. Sin valor, solo se crea la imagen. |
| `asset` | texto | Id de un degradado hecho antes con esta acción, para rehacerlo con otras paradas. Cualquier otro id se rechaza. |
| `format` | texto | `png` (por defecto) o `cube`. |
| `width` | entero | Ancho en píxeles, de 16 a 1024. Por defecto 256. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"fire"}}'
wprs-ctl scene act mi-escena ramp '{"stops":[{"at":0,"color":"#000000"},{"at":0.5,"color":"#ff6a00"},{"at":1,"color":"#ffff99"}],"pointer":"/scene/layers/0/effects/0/params/gradient"}'
wprs-ctl scene act mi-escena ramp '{"asset":"gradient","stops":[{"at":0,"color":"#200000"},{"at":1,"color":"#ffffff"}],"width":64}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"color_lut"}}'
wprs-ctl scene act mi-escena ramp '{"stops":[{"at":0,"color":"#000020"},{"at":1,"color":"#ffe0a0"}],"format":"cube","pointer":"/scene/layers/0/effects/1/params/lut"}'
```

#### `depth_map`

Calcula un mapa de profundidad gris de una imagen del paquete (blanco es cerca) con el modelo de profundidad de la app, y lo pone en un parámetro del efecto, por ejemplo el `depth` de `depth_parallax`. Necesita el modelo y su biblioteca instalados; si faltan se rechaza con el comando que los instala (la pregunta `depth_model` lo dice antes). En la app, **Hacer un mapa de profundidad**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `asset` | texto | (obligatorio) Id de una imagen del paquete. |
| `id` | texto | Id de la imagen que se crea. Por defecto, el id de la imagen más `_depth`. |
| `pointer` | texto | Ruta del parámetro que recibe el mapa, `/scene/layers/N/effects/M/params/NOMBRE`. |
| `input` | entero | Tamaño de entrada del modelo: múltiplo de 14 entre 196 y 1036. Por defecto 518. |

Ejemplo (no se prueba aquí porque exige el modelo instalado):

```text
wprs-ctl scene act mi-escena depth_map '{"asset":"foto","pointer":"/scene/layers/0/effects/0/params/depth"}'
```

## Preguntas

### Efectos de una capa

#### `rules`

Dice qué parámetros tiene un efecto y qué pide cada uno para animarse (el tiempo, el puntero, el sonido o los clics). Sirve también para averiguar los nombres de los parámetros de un efecto antes de usarlos en `effects`. Los efectos incluidos y los del paquete responden.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `effect` | texto | (obligatorio) Nombre del efecto. |

La respuesta tiene `effect`, `own` (`true` si es un efecto del paquete, que se puede cambiar con `effect_rule`), `uses` (entradas que pide el efecto entero), `settle` (segundos que tarda en quedarse quieto, o `null`) y `params`, una lista con `id`, `kind` (el tipo del parámetro: `float`, `vec2`, `color`, `bool`, `texture`...), `uses`, `waits_for` y `yields_to`. Si el efecto no existe, la respuesta es `null`.

```sh
wprs-ctl scene ask mi-escena rules '{"effect":"displace"}'
```

#### `effect_cost`

Dice cuánto cuesta un efecto en cada nivel de calidad (de `q0`, el más bajo, a `q3`, el más alto). El coste se mide en unidades de trabajo: el de un efecto sencillo vale 1. Un nivel con `null` significa que el efecto se deja fuera en ese nivel. En la app, la sección **Coste** del efecto.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `effect` | texto | (obligatorio) Nombre del efecto. |
| `optional` | sí/no | Si es `true`, calcula el coste como efecto opcional, que se deja fuera antes. Por defecto `false`. |

La respuesta tiene `effect`, `units` (el coste en la calidad más alta) y `levels`, una lista de `{quality, units}`. Si el efecto no existe, la respuesta es `null`.

```sh
wprs-ctl scene ask mi-escena effect_cost '{"effect":"blur"}'
wprs-ctl scene ask mi-escena effect_cost '{"effect":"fluid","optional":true}'
```

### Máscaras y mapas pintados

#### `ramp`

Devuelve las paradas de color de un degradado hecho con la acción `ramp`, para volver a editarlo.

El argumento no es un objeto sino el id del degradado, escrito entre comillas.

La respuesta tiene `asset` y `stops`, la lista de `{at, color}`; `stops` es `null` si ese id no es un degradado hecho con `ramp`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"fire"}}'
wprs-ctl scene act mi-escena ramp '{"stops":[{"at":0,"color":"#000000"},{"at":1,"color":"#ffff99"}],"pointer":"/scene/layers/0/effects/0/params/gradient"}'
wprs-ctl scene ask mi-escena ramp '"gradient"'
```

#### `depth_model`

Dice si el modelo de profundidad y la biblioteca que lo ejecuta están instalados, es decir, si `depth_map` puede funcionar. No lleva argumentos.

La respuesta tiene `installed` (el modelo), `runtime` (la biblioteca), `ready` (las dos cosas) e `install`, el comando que instala el modelo.

```sh
wprs-ctl scene ask mi-escena depth_model '{}'
```

#### `normals`

Muestra cómo quedaría el mapa de normales que haría la acción `normals`, sin guardar nada. Lleva los mismos argumentos que la acción (`asset` o `layer`, y opcionales `strength`, `blur` y `time`; `id` se ignora). En la app es la vista previa del cuadro **Generar normales…**.

La respuesta tiene `from` (la imagen de la que sale), `size`, y tres imágenes pequeñas incrustadas como texto `data:image/png;base64,...`: `normals` (el mapa), `relief` (el relieve, iluminado desde arriba a la izquierda) y `sphere` (una esfera de ejemplo con esa luz). Si los argumentos no valen, la respuesta es `null`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene ask mi-escena normals '{"layer":"image","strength":3,"blur":2}'
```

#### `map`

Muestra el mapa de rugosidad o el de luz propia de una capa como un mapa de calor.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la capa. También se puede dar con `--layer ID`. |
| `map` | texto | `emissive` (luz propia) o `roughness` (rugosidad, por defecto). |

La respuesta tiene `map` (el id de la imagen del mapa), `size` y `heat`, una imagen incrustada como texto `data:image/png;base64,...`. Si la capa no tiene ese mapa, la respuesta es `{"map": null}`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena paint_map '{"layer":"image","map":"emissive","points":[[0.5,0.5]],"radius":0.2,"hardness":1}'
wprs-ctl scene ask mi-escena map '{"layer":"image","map":"emissive"}'
```

### Calidad y asas de efectos

#### `level`

Dice qué hace un nivel de calidad con la escena: la escala de dibujo, la densidad, las partículas, qué efectos se quedan fuera y cuánto cuesta lo que queda. Sirve para ver cómo se vería el paquete en un equipo más modesto. En la app, la vista previa por nivel de calidad.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `quality` | texto | Nivel: `q0`, `q1`, `q2` o `q3`. Por defecto `q3`, el más alto. |
| `battery` | sí/no | `true` aplica el recorte que se hace con batería. Por defecto `false`. |
| `calm` | sí/no | Se devuelve tal cual en la respuesta, para la vista previa tranquila. |

La respuesta tiene `asked` y `quality` (el nivel que se pidió y el que de verdad se usa), `render_scale`, `density`, `particles`, `bloom` (`on`, `hdr` y `glare`), `pointer` (con los emisores que siguen al puntero), `effects`, `left_out` (cuántos efectos quedan fuera) y `units` (el coste de los que quedan). Cada efecto trae `layer`, `effect`, `optional`, `kept`, `scale`, `units`, `full_units` y `levels`, el coste en cada nivel.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"blur","optional":true}}'
wprs-ctl scene ask mi-escena level '{"quality":"q1"}'
```

#### `effect_handles`

Contesta con las asas que un efecto de una capa enseña sobre el lienzo: los puntos, líneas y cuadriláteros que se arrastran con la variante `grip` de [`effects`](#effects).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa. |
| `index` | entero | Posición del efecto en la capa. Por defecto 0. |
| `time` | número | Segundo en que se mide dónde está la capa. Por defecto 0. |

La respuesta tiene `effect` (el id del efecto), `quad` (si el efecto se arrastra por cuatro esquinas) y `handles`, cada una con `param`, `kind`, `at` (su lugar en el lienzo), `value`, `range`, `anchor` y `partner`. Si el efecto no tiene asas, `handles` viene vacía.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"blur_radial"}}'
wprs-ctl scene ask mi-escena effect_handles '{"layer":"image","index":0,"time":0}'
```

#### `mask_view`

Contesta con la máscara de un efecto de una capa tal como se dibuja sobre el lienzo: de dónde sale, el área, el borde suave y las asas. Se escribe con las variantes `mask` y `mask_grip` de [`effects`](#effects).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa. |
| `index` | entero | Posición del efecto en la capa. |
| `time` | número | Segundo en que se mide. Por defecto 0. |

La respuesta tiene `source` (`shape`, `texture` o `layer`), `kind`, `invert`, `channel`, `strength`, `feather`, `box`, `size`, `area` y `area_u` (el contorno en el lienzo y dentro de la capa), `outer` e `inner` (los bordes del borde suave) y `holds`, las asas con su `grip` y su lugar `at`. Si el efecto no tiene máscara, es `null`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena effects '{"add":{"layer":"image","effect":"blur"}}'
wprs-ctl scene act mi-escena effects '{"mask":{"layer":"image","index":0,"shape":{"ellipse":{"size":[600,400]}},"feather":40}}'
wprs-ctl scene ask mi-escena mask_view '{"layer":"image","index":0,"time":0}'
```

#### `mask_picture`

Prepara una imagen como máscara de un canal y contesta con el archivo que resulta. La app lo usa para ver la máscara de textura sobre la capa.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `asset` | texto | (obligatorio) Id de la imagen. |
| `channel` | texto | Canal: `r`, `g`, `b`, `a` o `luminance`. Por defecto `luminance`. |
| `invert` | sí/no | `true` invierte la máscara. Por defecto `false`. |
| `strength` | número | Fuerza, de 0 a 1. Por defecto 1. |
| `tint` | texto | Color de la máscara, `#rrggbb`. Por defecto blanco. |

La respuesta es `{"path": ...}` con la imagen preparada, que se guarda en la caché. Si el recurso no es una imagen del paquete, es `null`.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene ask mi-escena mask_picture '{"asset":"foto","channel":"luminance"}'
```
