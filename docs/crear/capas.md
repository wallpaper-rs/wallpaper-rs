# Las capas

Una escena se dibuja con capas, de abajo a arriba: la primera de la lista queda al fondo y la
última encima. Esta página dice qué tipos de capa hay, qué campos tiene cada una y cómo se
colocan. Los efectos de una capa están en [efectos.md](efectos.md), los valores que se mueven en
[escena.md](escena.md).

Todos los ejemplos salen de las carpetas de [`ejemplos/`](ejemplos/) y pasan
`wprs-ctl scene validate`.

## Lo que toda capa tiene

Cada capa lleva un `id`, que puede tener de 1 a 32 letras minúsculas, cifras o `_`, y un `type`.
Los ids no se repiten. En el editor, además, las capas, los recursos y los sistemas de partículas
comparten nombres: no repitas un id entre ellos aunque la escena lo permita.

Una escena puede tener hasta 256 capas, metidas unas en otras hasta 8 niveles.

| Campo | Tipo | Por defecto | Rango | Qué hace |
|-------|------|-------------|-------|----------|
| `id` | texto | obligatorio | `[a-z0-9_]{1,32}` | El nombre de la capa. |
| `type` | texto | obligatorio | ver abajo | El tipo de capa. |
| `transform` | objeto | sin cambios | ver "Colocar una capa" | Posición, giro, escala, inclinación. |
| `anchor` | texto | ninguno | `top_left`, `top`, `top_right`, `left`, `center`, `right`, `bottom_left`, `bottom`, `bottom_right` | Cuelga la capa de ese punto de la pantalla. |
| `opacity` | número | 1 | 0 a 1 | Qué tan visible es. |
| `visible` | sí o no | `true` | | Si se dibuja. |
| `blend` | texto | `normal` | ver "Mezclar" | Cómo se mezcla con lo de debajo. |
| `mask` | texto u objeto | sin máscara | ver "Máscaras" | Deja ver la capa solo en una parte. |
| `effects` | lista | vacía | hasta 16 | Efectos de la capa. Ver [efectos.md](efectos.md). |
| `parallax` | número o `[x, y]` | 0 | -10 a 10 cada uno | Cuánto sigue la capa al puntero. |
| `brightness` | número | 1 | 0 a 16 | Multiplica el color. Por encima de 1 solo se nota con `canvas.hdr`. |
| `color` | color | blanco | | Multiplica el color de capas de imagen, vídeo, partículas y grupo; en un grupo, a todas sus capas juntas. |
| `parent` | id | ninguno | | La capa sigue a otra. Ver "Capas que siguen a otras". |
| `inherit` | lista | `transform`, `visible` | `transform`, `visible`, `opacity` | Qué toma del padre; solo con `parent`. |
| `solid` | sí o no, o `{ "alpha": 0 a 1 }` | `false` | | La capa recibe clics y toques. Ver "Capas que reaccionan". |
| `cache` | sí o no | `false` | | Dibuja la capa aparte, en su propia imagen. Cuenta en el costo (ver [escena.md](escena.md)). |
| `lighting` | objeto | sin luz | | Cómo recibe la luz de las capas `light`. Ver [3d.md](3d.md). |
| `attach` | objeto | ninguno | | Pega la capa a un hueso. Ver [mallas.md](mallas.md). |
| `depth_test` | sí o no | `false` | | Dentro de un grupo 3D, deja que los modelos la tapen. Ver [3d.md](3d.md). |

Casi todos esos números, colores e interruptores pueden ser un valor fijo o seguir una propiedad,
una línea de tiempo, el sonido o el puntero (ver "Valores que se mueven" en [escena.md](escena.md)).

Los tipos de capa son: `image`, `video`, `text`, `shape`, `solid`, `spectrum`, `particles`,
`sound`, `group`, `adjustment`, `light`, `mesh` y `model`.

## Colocar una capa

```json
"transform": { "position": [x, y], "scale": [1, 1], "rotation": 0, "skew": [0, 0], "origin": [0.5, 0.5] }
```

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `position` | el medio del lienzo | píxeles del lienzo | Dónde queda el punto de origen de la capa. |
| `scale` | `[1, 1]` | | Escala en x e y; un solo número vale para los dos. |
| `rotation` | 0 | -360 a 360 | Giro en grados, en el sentido de las agujas del reloj. |
| `skew` | `[0, 0]` | grados | Deformación oblicua. |
| `origin` | `[0.5, 0.5]` | fracción de la capa | El punto alrededor del que gira y escala. Con `anchor`, el propio punto del ancla. |
| `tilt` | `[0, 0]` | -89 a 89 grados cada uno | Inclinación en profundidad. Necesita `canvas.perspective` para verse. Ver [3d.md](3d.md). |
| `z` | 0 | -100000 a 100000 px | Acercar o alejar de quien mira. Ver [3d.md](3d.md). |

Sin `position`, una capa queda en el centro del lienzo. Con `position`, el origen de la capa se
pone en ese punto del lienzo: `[0, 0]` es la esquina de arriba a la izquierda, `x` crece hacia la
derecha e `y` hacia abajo. `origin` es una fracción del cuadro de la propia capa; con `[0, 0]` la
posición es su esquina de arriba a la izquierda.

Con `anchor`, la posición se mide desde ese punto de la parte visible de la pantalla, y el origen
de la capa es ese mismo punto de ella: una capa con `"anchor": "bottom_right"` y
`"position": [-100, -100]` deja su esquina de abajo a la derecha a 100 píxeles de la esquina de
abajo a la derecha. Es la forma de poner cosas "pegadas al borde" en cualquier pantalla.

Un grupo no tiene cuadro propio: su `position` desplaza a todas las capas de dentro esa distancia
(y su `rotation` y su `scale` las giran y escalan juntas). Sin `position` el grupo no las mueve.

`parallax` mueve la capa con el puntero: la capa se desplaza su `parallax` por el tamaño del
lienzo por `canvas.parallax.amount` por lo lejos que está el puntero del medio de la pantalla.
Una capa con valor 0,02 se mueve poco, como algo lejano; 0,05 se mueve más, como algo cercano.
Hace falta la entrada `pointer` (ver [escena.md](escena.md)).

## Mezclar

`blend` dice cómo se mezcla la capa con lo que hay debajo. Los modos son:

- Básicos: `normal`, `add`, `multiply`, `screen`.
- Con base en el brillo: `lighten`, `darken`, `overlay`, `soft_light`, `hard_light`,
  `difference`, `exclusion`, `color_dodge`, `color_burn`.
- Con base en el color: `hue`, `saturation`, `color`, `luminosity`.
- Otros: `linear_burn`, `linear_dodge`, `darker_color`, `lighter_color`, `vivid_light`,
  `linear_light`, `pin_light`, `hard_mix`, `subtract`, `divide`, `reflect`, `glow`, `phoenix`,
  `average`, `negation`, `tint`, `diffuse_light`.

Los primeros cuatro son los más baratos (ver [escena.md](escena.md)): `add` y `screen` suelen
servir para luces, chispas y brillos. `add` y `diffuse_light` no recortan en 1 mientras mezclan,
así que guardan la luz por encima del blanco para las capas siguientes y para `canvas.hdr`.
`darker_color` y `lighter_color` ignoran la `opacity` de la capa.

## Máscaras

Con `mask`, la capa solo se ve donde la máscara lo permite. La máscara puede ser:

- el id de un recurso de imagen: `"mask": "mancha"`;
- un objeto con exactamente una de estas tres cosas: `texture` (un recurso de imagen), `layer`
  (el id de otra capa) o `shape` (una forma, con los mismos campos que una capa `shape`).

Y, además:

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `channel` | `luminance` con una imagen, `a` con una capa o una forma | `r`, `g`, `b`, `a`, `luminance` | Qué parte de la imagen o capa manda. Una forma manda por su silueta. |
| `invert` | `false` | | Da la vuelta a la máscara. |
| `feather` | 0 | 0 a 10000 | Suaviza el borde. |
| `strength` | 1 | 0 a 1 | Debilita la máscara: con 0 no hace nada. |

Una capa que se usa de máscara, o un grupo que la contiene, puede llevar `"visible": false`: no
se ve, pero enmascara igual. Las máscaras no pueden referirse unas a otras en círculo.
Una máscara también puede ir en un efecto (ver [efectos.md](efectos.md)).

## Capas que siguen a otras

`parent` hace que una capa siga el movimiento de otra capa dibujada, sin tener que meterla en un
grupo. La posición de la capa pasa a medirse desde la posición del padre, en el sistema girado y
escalado del padre. `inherit` dice qué sigue: `transform`, `visible` y `opacity` (por defecto, las
dos primeras). Las cadenas pueden llegar a 8 padres y no pueden volver a sí mismas. El orden de
dibujo no cambia.

## Capas que reaccionan

Con `"solid": true` la capa recibe el puntero sobre su cuadro; con `{ "alpha": 0.5 }` solo donde
es al menos así de opaca. Eso permite las líneas de tiempo con `trigger` `click:<capa>`,
`enter:<capa>` y `leave:<capa>` y los sonidos con `play_on` (ver
[escena.md](escena.md) y [particulas-y-sonido.md](particulas-y-sonido.md)). `click:` pide la
entrada `clicks` y `enter:` y `leave:` piden `pointer`. Las capas se prueban de arriba abajo y las
ocultas nunca reciben nada.

El ejemplo [`animacion-e-interaccion`](ejemplos/animacion-e-interaccion/scene.json) hace crecer
un botón al hacer clic y suena un golpe:

```json
    { "id": "boton", "type": "shape", "shape": { "rect": { "size": [360, 140], "radius": 70 } }, "fill": "#3b82f6",
      "solid": true, "transform": { "position": [1250, 540],
        "scale": { "timeline": "pulsar", "keys": [[0, 1], [0.15, 0.9, "out_quad"], [0.4, 1, "out_back"]] } } }
```

```json
  "timelines": [
    { "id": "pulsar", "duration": 0.4, "loop": "once", "autoplay": false, "trigger": "click:boton",
      "events": [{ "t": 0.15, "name": "golpe" }] }
  ]
```

## Capa de imagen (`image`)

Dibuja un recurso de imagen. Sin `size`, la imagen se enseña a su tamaño real, centrada; con
`size` se estira a ese tamaño.

```json
    { "id": "fondo", "type": "image", "image": "cielo", "size": [1920, 1080],
      "effects": [{ "effect": "waves", "params": { "amplitude": 6, "frequency": 3 } }] }
```

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `image` | obligatorio | El id del recurso, o un valor: `{ "prop": "imagen" }` para una propiedad de archivo o `{ "prop": "estilo", "map": { "noche": "cielo" } }` para elegir entre recursos. |
| `size` | el tamaño de la imagen | `[ancho, alto]` en píxeles, de 0,001 a 100000. |
| `color` | blanco | Color que multiplica la imagen. |
| `playback` | | Para hojas de dibujos, GIF y vídeos. Abajo. |

Si el recurso tiene `wrap` `repeat` o `mirror` y `size` es más grande que la imagen, la imagen se
repite en vez de estirarse. Una imagen con una propiedad de archivo muestra lo que elija el
usuario, y cualquier archivo nuevo mantiene el reloj, las líneas de tiempo y las partículas de la
escena.

`playback` rige imágenes en movimiento y vídeos:

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `rate` | 1 | 0 a 16 (vídeo: 0, o 0,1 a 4) | La velocidad. Con 0 se detiene un vídeo. |
| `playing` | `true` | | Reproducir o parar. |
| `loop` | `true` | | Repetir al terminar. |
| `start`, `end` | 0, el final | segundos | El tramo que se reproduce. |
| `frame` | | | Qué fotograma de una hoja se enseña (puede seguir claves, el puntero o el sonido). |
| `own_clock` | `false` | | Solo en capas de vídeo: reloj propio, con su ritmo, tramo y pausa. Cuesta un decodificador más. |

Una hoja de dibujos (recurso con `atlas` o `rects`, ver [escena.md](escena.md)) reproduce sus
fotogramas con el reloj de la escena: todas las capas que enseñan la misma hoja al mismo ritmo
enseñan el mismo fotograma. En el nivel de calidad más bajo se queda en el primer fotograma del
tramo.

```json
    { "id": "baile", "type": "image", "image": "bola", "size": [256, 256],
      "transform": { "position": [700, 540] },
      "playback": { "rate": 1, "loop": true } }
```

## Capa de vídeo (`video`)

Reproduce un recurso de vídeo (WebM, MP4 o MKV), en bucle salvo que digas lo contrario.

```json
    { "id": "pelicula", "type": "video", "video": "cuadros", "size": [1920, 960],
      "playback": { "rate": 1, "loop": true, "start": 0 } }
```

Tiene los mismos campos que una capa de imagen, pero con `video` en vez de `image` (el id del
recurso, o un valor con `prop` y `map`). Todas las capas que enseñan el mismo vídeo comparten un
decodificador y un reloj, que dirige la primera de ellas; `own_clock` le da su reloj propio:

```json
    { "id": "pequena", "type": "video", "video": "cuadros", "size": [384, 192],
      "transform": { "position": [1700, 960] },
      "playback": { "own_clock": true, "rate": 0.5 } }
```

Mientras ninguna capa visible enseña el vídeo, queda quieto y no pide fotogramas. En batería su
decodificador no va más rápido que `clock.max_fps`. Con una propiedad de archivo que acepta
`video` el usuario puede elegir su propio vídeo.

## Capa de texto (`text`)

```json
    { "id": "titulo", "type": "text", "text": "Buenos días", "font": "sans-serif", "size": 160,
      "color": "#ffffff", "align": "center",
      "outline": { "color": "#3b4a6b", "width": 6 },
      "shadow": { "color": "#00000099", "offset": [6, 8], "blur": 12 },
      "transform": { "position": [960, 300] } }
```

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `text` | obligatorio | | Un texto, o un valor: `{ "clock": "HH:mm" }`, `{ "date": "EEE d MMM yyyy" }`, `{ "prop": "mensaje" }`, `{ "media": "title" }`. |
| `font` | la del sistema | | El id de un recurso de fuente, o una familia del sistema (`sans-serif`, `serif`, `monospace`). |
| `fallback` | ninguna | hasta 8 | Más fuentes para los caracteres que `font` no tiene. |
| `size` | 48 | 1 a 1024 px | El tamaño de las letras. |
| `color` | blanco | | El color. |
| `align` | `center` | `left`, `center`, `right` | Alineación de las líneas. |
| `valign` | `center` | `top`, `center`, `bottom` | Qué borde del texto queda en la posición mientras cambian las líneas. |
| `wrap` | sin partir | `width` 1 a 16384 px, `lines` 1 a 1000, `ellipsis` | Parte líneas al ancho dado, deja como mucho `lines` y termina un texto cortado con puntos suspensivos. |
| `line_height` | | 0,5 a 4 veces el tamaño | La altura de línea. |
| `letter_spacing` | 0 | -100 a 1000 px | Espacio entre letras. |
| `background` | sin caja | `color`, `padding` `[x, y]`, `radius` | Una caja detrás del texto. |
| `padding` | 0 | 0 a 1024 px | Margen alrededor para los efectos. |
| `outline` | sin contorno | `color`, `width` 0 a 64 px | Un contorno. |
| `shadow` | sin sombra | `color`, `offset` `[x, y]`, `blur` 0 a 64 px | Una sombra. |
| `blur` | 0 | 0 a 64 px | Desenfoca el texto. |
| `render` | `auto` | `auto`, `raster`, `sdf` | Cómo se dibuja: `sdf` usa un campo de distancias para que el tamaño cambie sin volver a dibujar las letras; `raster` dibuja de nuevo al tamaño que tiene en pantalla. |

`letter_spacing`, `outline`, `shadow` y `blur` pueden seguir valores.
Un texto se vuelve a componer solo cuando cambian el texto, el tamaño en pantalla o la
composición; su color y su posición no lo hacen. Cada carácter sale de la primera fuente de
`font` y `fallback` que lo tiene, y si ninguna, de `sans-serif`. Las letras se colocan una detrás
de otra con el espaciado de la fuente, sin unir ni reordenar: la escritura de derecha a izquierda
y las escrituras que necesitan formas unidas (árabe, escrituras de la India) no se ven bien. Una
fuente que no se puede leer se cambia por la del sistema.

Los relojes y las fechas se explican en [escena.md](escena.md). El ejemplo
[`texto`](ejemplos/texto/scene.json) lleva texto con contorno, sombra, caja, ajuste de líneas,
hora y fecha:

```json
    { "id": "parrafo", "type": "text",
      "text": "Un texto largo se parte en líneas del ancho que tú digas y termina con puntos suspensivos si no cabe.",
      "font": "serif", "size": 44, "color": "#dbe4f5", "align": "left", "valign": "top",
      "line_height": 1.3, "letter_spacing": 1,
      "wrap": { "width": 900, "lines": 3, "ellipsis": true },
      "background": { "color": "#00000066", "padding": [24, 16], "radius": 18 },
      "transform": { "position": [510, 520] } }
```

```json
    { "id": "hora", "type": "text", "text": { "clock": "HH:mm:ss" }, "font": "monospace", "size": 90,
      "color": "#7dd3fc", "anchor": "bottom_right", "transform": { "position": [-80, -60] } }
```

## Capa de forma (`shape`)

```json
    { "id": "panel", "type": "shape", "shape": { "rect": { "size": [1200, 500], "radius": 40 } },
      "gradient": { "from": [0, 0], "to": [0, 1], "stops": [[0, "#2b3a67"], [1, "#101828"]] },
      "stroke": { "color": "#8aa4ff", "width": 4 } }
```

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `shape` | obligatorio | Una de las seis formas de abajo. |
| `fill` | ninguno | El color de relleno. |
| `gradient` | ninguno | Un degradado que reemplaza al relleno. |
| `stroke` | ninguno | Un borde. |

Las formas:

| Forma | Campos | Rango |
|-------|--------|-------|
| `rect` | `size` `[ancho, alto]`, `radius` (0 por defecto) | `radius` hasta la mitad del lado menor |
| `ellipse` | `size` `[ancho, alto]` | |
| `ring` | `size` `[ancho, alto]`, `width` | `width` de 0,001 hasta la mitad del lado menor |
| `polygon` | `sides`, `size` | `sides` de 3 a 64 |
| `star` | `points`, `size`, `inner` (0,5 por defecto) | `points` de 2 a 64, `inner` de 0,01 a 1 |
| `line` | `to` `[x, y]`, `width` | `to` de -100000 a 100000 |

Una `line` mide lo que dice `to` y queda centrada en la posición de la capa.

`gradient`: `from` (`[0, 0]`) y `to` (`[0, 1]`) son fracciones del cuadro de la forma; `radial`
(`false`) lo hace circular desde `from`, con `to` marcando el borde; `stops` es una lista de 2 a 4
`[posición, color]` con las posiciones de 0 a 1 en orden. `stroke`: `color` (blanco) y `width`
(0 a 10000 px).

```json
    { "id": "estrella", "type": "shape", "shape": { "star": { "points": 5, "size": 120, "inner": 0.45 } },
      "fill": "#f472b6", "transform": { "position": [1280, 540] } }
```

```json
    { "id": "brillo", "type": "shape", "shape": { "ellipse": { "size": [700, 700] } }, "blend": "add",
      "gradient": { "radial": true, "from": [0.5, 0.5], "to": [1, 0.5],
        "stops": [[0, "#ffffff60"], [0.5, "#5b7cff30"], [1, "#5b7cff00"]] } }
```

## Capa de color (`solid`)

Rellena un rectángulo de un color. Sin `size` cubre todo el lienzo.

```json
    { "id": "fondo", "type": "solid", "color": "#101828" }
```

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `color` | obligatorio | El color (puede ser un valor que se mueve). |
| `size` | el lienzo | `[ancho, alto]` en píxeles. |

Con una capa de color a pantalla completa, un `blend` `add` y una máscara de otra capa se hacen
resplandores (ver [`grupos-y-mascaras`](ejemplos/grupos-y-mascaras/scene.json)).

## Capa de espectro (`spectrum`)

Dibuja el sonido que suena en el equipo. Necesita la entrada `audio`.

```json
    { "id": "espectro", "type": "spectrum", "size": [1600, 360], "style": "bars", "bands": 32, "gap": 0.3,
      "color": "#7dd3fc", "anchor": "bottom", "transform": { "position": [0, -80] } }
```

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `size` | obligatorio | | `[ancho, alto]` de la capa. |
| `style` | `bars` | `bars`, `circle`, `wave` | Barras, círculo u onda. |
| `bands` | 32 | 16, 32 o 64 | Cuántas barras. |
| `channel` | `average` | `average`, `left`, `right` | Qué canal. |
| `color` | blanco | | El color. |
| `gap` | 0,2 | 0 a 0,95 | El hueco entre barras. |

## Capa de grupo (`group`)

Junta capas. Sirve para moverlas, girarlas, mezclarlas o aplicarles efectos todas a la vez.

```json
    { "id": "ventana", "type": "group", "clip": true,
      "layers": [
        { "id": "marco", "type": "shape", "shape": { "rect": { "size": [700, 420], "radius": 60 } }, "fill": "#ffffff" },
        { "id": "interior", "type": "image", "image": "cielo", "size": [1400, 790],
          "transform": { "position": { "timeline": "paseo", "keys": [[0, [760, 540]], [8, [1160, 540]]] } },
          "effects": [{ "effect": "color_adjust", "params": { "saturation": 1.6, "contrast": 1.2 } }] }
      ] }
```

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `layers` | obligatorio | Las capas de dentro. |
| `clip` | `false` | Las capas siguientes a la primera solo se ven donde se ve la primera. |
| `color` | blanco | Color que multiplica a todas juntas. |
| `space` | `2d` | Con `3d`, el grupo dibuja sus modelos juntos con una profundidad compartida. Ver [3d.md](3d.md). |

Un grupo con efectos, máscara, opacidad menor que 1 o `cache` se dibuja primero aparte y se
compone entero (una pasada más). El `transform` de un grupo mueve a todas sus capas (ver
"Colocar una capa").

## Capa de ajuste (`adjustment`)

No dibuja nada propio: aplica sus `effects` a todo lo que hay debajo de ella (el lienzo, o su
grupo).

```json
    { "id": "retoque", "type": "adjustment", "size": [900, 500],
      "transform": { "position": [960, 540] },
      "effects": [{ "effect": "vignette", "params": { "intensity": 0.5 } }] }
```

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `effects` | | Los efectos que se aplican. |
| `size` | todo el lienzo | `[ancho, alto]`, hasta el doble del lienzo: solo ese rectángulo alrededor de la posición se captura y se cambia. |
| `redraw` | `false` | Vuelve a dibujar lo capturado con el `transform`, la opacidad, la mezcla y la máscara de la capa, como una lupa o un espejo. |
| `rgb` | `false` | Envía lo que captura a la iluminación RGB del equipo. Hace falta `permissions.rgb` en el manifiesto y que el usuario lo permita. |

Con `size` el costo baja porque solo se tratan esos píxeles, pero la captura es una copia y una
pasada más.

## Capas que se explican en otra página

- `particles`: un sistema de partículas. Ver [particulas-y-sonido.md](particulas-y-sonido.md).
- `sound`: un sonido de fondo, sin dibujo. Ver [particulas-y-sonido.md](particulas-y-sonido.md).
- `light`: una luz que ilumina las capas con `lighting`. Ver [3d.md](3d.md).
- `model`: un modelo 3D. Ver [3d.md](3d.md).
- `mesh`: una imagen doblada por un esqueleto. Ver [mallas.md](mallas.md).

## No cubierto aquí

- `lighting` y las luces: [3d.md](3d.md).
- `attach`, los huesos y los clips de mallas y modelos: [mallas.md](mallas.md).
- Capas y plantillas que crean los scripts (`templates`, `pools`): [scripts.md](scripts.md).
