# Partículas y sonido

Esta página trata de dos cosas que una escena puede tener además de imágenes: sistemas de
partículas (chispas, nieve, luciérnagas) y sonido (uno que suena y otro que se ve moverse).

Los ejemplos salen de [`ejemplos/particulas`](ejemplos/particulas/scene.json),
[`ejemplos/particulas-avanzadas`](ejemplos/particulas-avanzadas/scene.json) y
[`ejemplos/sonido`](ejemplos/sonido/scene.json), y pasan `wprs-ctl scene validate`.

# Partículas

Un sistema de partículas es una receta: de dónde nacen, cómo salen, qué fuerzas las mueven y
cómo se dibujan. La receta va en `particles` de `scene.json`, con un id por sistema, y una capa
`particles` la dibuja.

```json
    { "id": "chispas", "type": "particles", "system": "chispas", "blend": "add",
      "instance": { "alpha": 0.9, "count": 1 } }
```

```json
  "particles": {
    "chispas": {
      "capacity": 4000, "seed": 7, "prewarm": 2,
      "emitters": [{ "shape": { "box": [[0, 1080], [1920, 1100]] }, "rate": { "prop": "densidad" } }],
      "init": { "life": [2, 5], "speed": [200, 500], "angle": [250, 290], "size": [4, 12],
                "color": ["#ffd27a", "#ff7a3d"] },
      "forces": [{ "gravity": [0, -20] }, { "turbulence": { "strength": 30, "scale": 0.004 } },
                 { "drag": 0.4 }],
      "collide": [{ "bounds": "canvas", "bounce": 0.3 }],
      "render": { "size_over_life": [[0, 0], [0.1, 1], [1, 0]], "color_over_life": [[0, "#ffffff"], [1, "#ffffff00"]] }
    },
    "nieve": {
      "capacity": 3000,
      "emitters": [{ "shape": { "line": [[0, -20], [1920, -20]] }, "rate": 120 }],
      "init": { "life": [8, 12], "speed": [40, 90], "angle": [80, 100], "size": [3, 8], "color": ["#ffffff", "#cfe6ff"] },
      "forces": [{ "wind": [12, 0] }, { "noise": 6 }],
      "render": { "size_over_life": [[0, 1], [1, 1]] }
    }
  }
```

Una escena puede tener hasta 32 sistemas y entre todos hasta 200 000 partículas. Varias capas
pueden dibujar el mismo sistema: el sistema se mueve una sola vez por fotograma y reparte su
presupuesto entre ellas. Las posiciones son píxeles del lienzo; el `transform` de la capa mueve,
gira y escala todo el sistema alrededor del centro del lienzo.

La capa `particles`:

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `system` | obligatorio | El id del sistema. |
| `color` | blanco | Color que multiplica las partículas. |
| `instance` | | Lo que cambia solo para esta capa. Ver "Instancias". |
| `render` | el del sistema | Un aspecto propio de esta capa, que reemplaza al del sistema. |

Se aplican los campos de toda capa: `blend` (`add` va bien con partículas que brillan), `opacity`,
`mask` y `effects`.

## Empezar con un punto de partida

Una capa de partículas nueva ya trae un sistema que se ve: nieve que cae de arriba abajo sobre todo
el lienzo, con turbulencia suave y `prewarm`, de modo que está llena desde el primer cuadro.

```
wprs-ctl scene add-layer mi-escena particles --name Nieve
```

Imprime el id de la capa y del sistema (`nieve`). Para otro comienzo, la acción `particles` con
`kit` rellena de una vez los emisores, la vida, las fuerzas y los colores del sistema (los hijos se
conservan) con `snow`, `sparks` (chispas que suben), `rain` o `dust` (motas que flotan), ajustados al
tamaño del lienzo:

```
wprs-ctl scene act mi-escena particles '{"kit":{"system":"nieve","kit":"sparks"}}'
```

En el editor es el desplegable *Empezar con…* de los ajustes del sistema
([editor-particulas-y-sonido.md](editor-particulas-y-sonido.md)). Después ajusta lo que quieras.

**Si lo escribes a mano, pon `prewarm`.** Un sistema sin `prewarm` arranca vacío: las partículas
nacen poco a poco, y una imagen suelta del primer instante (`scene render`, una miniatura) sale sin
ninguna. Pon en `prewarm` los segundos que tardan en llenarse (hasta 30), del orden de la vida de
las partículas (`init.life`): el punto de partida de la nieve, con vidas de 12 a 18 s, usa 14.

## El sistema

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `capacity` | | 1 a 200000 | Cuántas partículas caben vivas a la vez. |
| `seed` | 0 | | La semilla del sistema, que se mezcla con la de `clock.seed`. |
| `prewarm` | 0 | 0 a 30 s | Segundos que se simulan antes de la primera vez que se ve. |
| `space` | `local` | `local`, `world` | En `world` el `transform` de la capa solo coloca las partículas al nacer; después ya no las mueve. |
| `emitters` | | hasta 8 | De dónde y cuánto nacen. |
| `init` | | | Cómo salen. |
| `forces` | | hasta 16 | Lo que las mueve. |
| `collide` | | hasta 8 | Con qué chocan. |
| `render` | discos suaves | | Cómo se dibujan. |
| `children` | | hasta 4 | Sistemas que nacen de estas partículas. |
| `points` | | hasta 8 | Puntos de control, `p0` a `p7`. |
| `perspective` | sin profundidad | `focal` 1 a 100000 px | Da profundidad a las partículas. |
| `lock` | | | Ajustes que las capas y los scripts no pueden cambiar con `instance`. |

La simulación avanza en pasos de 1/60 s, hasta 15 por fotograma. Con el mismo `seed` y los mismos
fotogramas salen las mismas partículas. Mientras no vive ni nace ninguna, el sistema salta hasta
su próxima ráfaga y la escena puede dormir. Una escena en pausa libera sus partículas y las
vuelve a empezar, precalentadas, al seguir.

### Emisores

Cada emisor tiene una `shape`, un ritmo y, si quieres, ráfagas. En el panel de partículas se añaden con «Añadir un emisor» y se quitan con el botón de cada emisor (un sistema conserva al menos uno).

| Campo | Qué hace |
|-------|----------|
| `shape` | De dónde nace: ver abajo. |
| `rate` | Partículas por segundo; puede seguir un valor (una propiedad, el sonido...). |
| `bursts` | Hasta 8 ráfagas, cada una de `count` partículas: `on` cada golpe de música (`beat`) o clic (`click`); o una vez a los `at` segundos de empezar el emisor (0 por defecto) y, con `every`, otra vez cada tantos segundos. |
| `follow` | `cursor`: la forma sigue al puntero. |
| `inherit` | Fracción de la velocidad del puntero que heredan. |
| `duration` | Segundos tras los que el emisor se apaga (0, nunca). |
| `delay` | Segundos antes de empezar (0 a 86400). |
| `periodic` | `{ "on": [min, max], "off": [min, max] }`: segundos de emisión y de pausa, elegidos con la semilla. |
| `max_per_step` | Como mucho tantos nacimientos por paso de 1/60 s (1 a 1000). |
| `audio_gate` | `true`: no nace nada mientras el sonido está en silencio. |

Las formas son:

| Forma | Ejemplo | Dónde nacen |
|-------|---------|-------------|
| `point` | `{ "point": [x, y] }` | En un punto. |
| `box` | `{ "box": [[x0, y0], [x1, y1]] }` | En un rectángulo. También `{ "from", "to", "inner": [ancho, alto], "at" }` para un rectángulo hueco alrededor de un punto de control. |
| `circle` | `{ "circle": { "center", "radius", "inner" } }` | En un disco, o en un anillo si `inner` es mayor que 0. También `scale: [x, y]` (0,01 a 100), `sign: [x, y]` (-1, 0 o 1: medios círculos o cuartos) y `at`. |
| `line` | `{ "line": [[x0, y0], [x1, y1]] }` | Sobre una línea. |
| `edge` | `"edge"` | En el borde del lienzo. |
| `mask` | `{ "mask": { "texture", "threshold" } }` | Donde el alfa de una imagen estirada sobre el lienzo supera `threshold` (0,5 por defecto). |
| `layer` | `{ "layer": { "id", "threshold", "color", "inherit", "jitter", "refresh" } }` | Donde una capa es opaca (hasta 4096 puntos), con el color y el movimiento de la capa. `refresh` es `static` o `on_change`. |
| `around` | `{ "around": { "at", "count", "arc", "radius", "speed", "mode", "restart" } }` | Uno tras otro alrededor de un punto: `count` sitios (1 a 4096) en `arc` grados, girando `speed` grados por segundo. `mode`: `repeat` o `mirror`. |
| `between` | `{ "between": { "from", "to", "count", "span", "mode", "arc", "taper" } }` | A lo largo de una línea o arco entre dos puntos. |

### Cómo salen (`init`)

Cada valor es un rango `[mínimo, máximo]` que se elige al azar para cada partícula. Con un tercer
número, `[mín, máx, exponente]` (de 0,1 a 10), el azar se inclina hacia un extremo.

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `life` | `[1, 2]` | Segundos de vida. |
| `speed` | `[10, 40]` | Píxeles por segundo. |
| `angle` | `[0, 360]` | Dirección en grados, en el sentido del reloj, 0 hacia la derecha (270 es hacia arriba). |
| `size` | `[2, 6]` | Tamaño en píxeles. |
| `rotation`, `spin` | `[0, 0]` | Giro inicial, y giro por segundo, en grados. |
| `frame` | `[0, 0]` | Primer fotograma de la hoja de dibujos. |
| `color` | blanco | Dos colores entre los que se elige. |
| `alpha` | | Un rango de opacidad. |
| `hsv` | | `{ "hue": [0, 360], "steps", "sat", "val" }`: color por tono, saturación y valor. |
| `palette` | | `{ "colors": [...], "noise": [h, s, v] }`: uno de hasta 10 colores, con un poco de variación. |
| `gradient` | | Hasta 8 `[posición, color]`, leídos en un punto al azar. |
| `velocity` | | `[[vx0, vy0], [vx1, vy1]]`, en lugar de `speed` y `angle`. |
| `radial` | | Velocidad que las aleja del centro de la forma. |
| `inherit` | | `{ "from": "cursor" o un punto, "speed": [min, max] }`: heredan la velocidad del puntero o de un punto de control. |
| `turbulent` | | Una velocidad inicial que gira con un ruido suave. |
| `offset_noise` | | Un desplazamiento fractal del lugar de nacimiento. |
| `remap` | | Hasta 8 reglas que llevan una entrada (`life`, `speed`, `age`, `daytime`, `runtime`, `random`, `distance:pN`, `audio:<banda>`, `cursor:x`, `cursor:y`) a un valor inicial (`output`: `size`, `alpha`, `color`, `speed`, `life`, `rotation`, `spin`, `frame`) con `in`, `out`, `op` (`set`, `mul`, `add`, `sub`) y `wave` (`none`, `sine`, `noise`). |
| `depth`, `depth_speed` | | Profundidad inicial y su cambio, en píxeles; necesitan `perspective`. |

Los extremos de los rangos numéricos (`life`, `speed`, `angle`, `size`...) pueden seguir valores,
como `"size": [2, { "audio": "bass", "scale": 8 }]`; se leen en cada fotograma. Los colores y las
velocidades se leen al cargar.

### Fuerzas

Una lista de hasta 16, aplicadas en cada paso:

| Fuerza | Qué hace |
|--------|----------|
| `{ "gravity": [x, y] }` o `wind` | Una aceleración constante, en píxeles por segundo al cuadrado. |
| `{ "drag": k }` | Las frena. |
| `{ "turbulence": { "strength", "scale" } }` | Un ruido que las arrastra. También `speed`, `time_scale` y `mask`. |
| `{ "attractor": { "target", "strength" } }` | Las atrae (con fuerza negativa, las aleja). `target` es un punto, `"cursor"` o un punto de control. Con `radius` no actúa más allá, y `falloff` es `constant`, `linear` o `inverse_square`. |
| `{ "vortex": { "center", "strength" } }` | Las hace girar alrededor de un centro. También `inner`, `outer` y `ring`. |
| `{ "orbit": { "center", "strength" } }` | Las hace orbitar. |
| `{ "noise": fuerza }` | Un temblor al azar. |
| `{ "oscillate": { "what", "freq", "scale", "phase", "mask" } }` | Las hace oscilar en posición, opacidad o tamaño. |
| `{ "cap": velocidad }` | Limita la velocidad. |
| `{ "torque": k }`, `{ "spin_drag": k }` | Cambian y frenan el giro. |
| `{ "slow_near": { "target", "inner", "outer", "factor" } }` | Las frenan cerca de un punto. |
| `{ "keep_distance": { "target", "distance", "strength" } }` | Las mantienen a una distancia, como un muelle. |
| `{ "keep_between": { "from", "to", "strength" } }` | Las atraen a un segmento. |
| `{ "boids": { "radius", "separation", "alignment", "cohesion", "max_speed" } }` | Bandadas: se separan, se alinean y se juntan con sus vecinas. Hasta 8192 partículas. |
| `{ "remap": { ... } }` | Una regla de `init.remap` aplicada en cada paso. |

Cualquier fuerza acepta `window: [entra_ini, entra_fin, sale_ini, sale_fin]`, fracciones de la
vida en las que entra y sale. La gravedad y las fuerzas pueden seguir valores, como
`{ "noise": { "audio": "bass", "scale": 300 } }`.

### Choques

Hasta 8 colisionadores, cada uno `{ "bounds", "bounce", "friction", "kill", "stick" }`. `bounds`:
`"canvas"`, `{ "plane": { "point", "normal" } }`, `{ "circle": { "center", "radius", "inside" } }`,
`{ "box": { "from", "to", "inside" } }`, `{ "segment": { "from", "to" } }`,
`{ "layer": { "id" } }` (la parte opaca de una capa) o `{ "mask": { "texture", "threshold" } }`.
`bounce` y `friction` van de 0 a 1; `kill` borra la partícula al tocar; `stick` la deja pegada.

### Cómo se dibujan (`render`)

| Campo | Qué hace |
|-------|----------|
| `texture` | Un recurso de imagen, que se parte en fotogramas con el `atlas` o los `rects` del recurso. Sin él, discos suaves. |
| `frames` | `time` (a `fps`, 12 por defecto), `life` o `random`. |
| `mix` | Mezcla un fotograma con el siguiente. |
| `cycles` | Cuántas veces se repiten los fotogramas en la vida (1 a 64, con `frames: life`). |
| `size_over_life` | Lista de `[t, valor]` (hasta 64): multiplica el tamaño. | En el panel de partículas, «A lo largo de la vida de una partícula» da una curva para arrastrar los puntos y, para el color, una barra de degradado.
| `color_over_life` | Lo mismo, con colores. |
| `brightness` | 0 a 16, puede seguir un valor. |
| `stretch` | Segundos de movimiento a lo largo de los que se estira la partícula; o `{ "time", "min", "max" }` en píxeles. |
| `trail` | Una estela: `length` (1 a 16 pasos, o hasta 64 con `segments`), `width` (0 a 1), `segments`, `subdivide` (0 a 7), `uv_scale`, `uv_scroll`, `textured`. |
| `rope` | Une las partículas en una tira, por orden de nacimiento: `width`, `subdivide`, `smooth`, `uv_scale`, `uv_scroll`. |
| `order` | Cuál se dibuja encima: `newest`, `oldest`, `largest`, `smallest` o `lowest`. |
| `cutout` | `[inicio, fin, opacidad]`: convierte el alfa de una textura suave en un borde duro. |
| `orient` | `screen`, `velocity` o `fixed` (con `angle`). |
| `lit`, `normal` | `lit`: las ilumina con las luces de la escena; `normal` es un mapa de normales `linear`. |
| `refract` | `{ "normal", "amount" }`: refractan lo que hay detrás. |
| `shader` | Un efecto propio con `stage: "particle"` (ver [efectos.md](efectos.md)). |

Los niveles de calidad `q2`, `q1` y `q0` dejan nacer solo el 75 %, el 50 % y el 25 % de las
partículas, y recortan o quitan las estelas.

### Puntos de control, hijas y profundidad

`points` define hasta 8 puntos (`p0` a `p7`) con `at` (que puede seguir un valor), `follow:
"cursor"`, `space` y `angle`. Los puntos se pueden usar en formas, fuerzas y `init.inherit`.
`p0` es el origen del sistema salvo que `points` lo cambie. Un punto que sigue al puntero vuelve
al centro cuando el puntero sale.

`children` hace que otros sistemas nazcan de estas partículas: cada hija es
`{ "system", "on", "probability", "max", "offset", "angle", "scale", "points_from", "inherit",
"follow_inherit" }`. `on` es `static` (nacen alrededor del origen del padre), `spawn` o `death`
(una ráfaga donde nace o muere cada partícula del padre) o `follow` (nacen a su ritmo en cada
partícula viva y van con ella). `inherit` copia `color`, `size`, `alpha` o `velocity`. Pueden
anidarse dos niveles y nunca volver a sí mismas.

Con `perspective`, cada partícula guarda una profundidad `z` y se dibuja más pequeña cuanto más
lejos. `init.depth` y `init.depth_speed` la fijan.

```json
    { "id": "estrellas", "type": "particles", "system": "estrellas", "blend": "add" }
```

```json
  "particles": {
    "cohetes": {
      "capacity": 200,
      "emitters": [{ "shape": { "point": [960, 1040] }, "rate": 0, "bursts": [{ "count": 1, "every": 1.6 }] }],
      "init": { "life": [1.1, 1.5], "speed": [700, 850], "angle": [255, 285], "size": [10, 12], "color": ["#ffe9a8", "#ffb347"] },
      "forces": [{ "gravity": [0, 500] }],
      "render": { "trail": { "length": 12, "width": 0.6 } },
      "children": [{ "system": "estrellas", "on": "death", "inherit": ["color"] }]
    },
    "luciernagas": {
      "capacity": 600, "space": "world", "prewarm": 3,
      "points": [{ "at": [960, 540] }, { "follow": "cursor" }],
      "emitters": [{ "shape": { "around": { "at": "p0", "count": 24, "radius": 420, "speed": 20 } }, "rate": 40,
                     "periodic": { "on": [4, 6], "off": [1, 2] }, "max_per_step": 4 }],
      "init": { "life": [3, 6], "speed": [10, 30, 2], "angle": [0, 360], "size": [6, 14],
                "hsv": { "hue": [50, 120], "sat": [0.5, 0.9], "val": [0.9, 1] }, "alpha": [0.5, 1],
                "remap": [{ "input": "life", "in": [3, 6], "output": "size", "out": [1, 0.3], "op": "mul" }] },
      "forces": [{ "turbulence": { "strength": 40, "scale": 0.003 } },
                 { "attractor": { "target": "p1", "strength": 4000, "radius": 500, "falloff": "linear" } },
                 { "oscillate": { "what": "alpha", "freq": [0.5, 1.5], "scale": [0.2, 0.5] } },
                 { "drag": 0.3, "window": [0, 0.2, 0.8, 1] }],
      "render": { "size_over_life": [[0, 0], [0.2, 1], [1, 0]], "cutout": [0.1, 0.9, 1], "order": "newest" }
    },
    "estrellas": {
      "capacity": 3000,
      "emitters": [{ "shape": { "point": [0, 0] }, "rate": 0, "bursts": [{ "count": 60 }] }],
      "init": { "life": [0.9, 1.6], "speed": [80, 320], "angle": [0, 360], "size": [5, 9], "color": ["#ffffff", "#9ad1ff"] },
      "forces": [{ "gravity": [0, 160] }, { "drag": 0.8 }],
      "render": { "size_over_life": [[0, 1], [1, 0]], "color_over_life": [[0, "#ffffff"], [1, "#ffffff00"]] }
    }
  }
```

### Instancias

`instance` en una capa de partículas cambia el sistema solo para esa capa: `alpha`, `size`,
`brightness` y `color` multiplican el aspecto; `count` (0 a 4) la fracción de nacimientos;
`speed` y `life` multiplican la velocidad y la vida iniciales; `time_scale` la rapidez del
tiempo; `points` mueve los puntos de control. Todos pueden seguir valores. Una capa con `count`,
`speed`, `life`, `time_scale` o `points` mueve sus propias partículas; las demás comparten las
del sistema. Con un valor de `count` por encima de 1, la capa reserva tantas veces `capacity`:
si lo enlazas a una propiedad, ponle `clamp`.

### Cuántas caben

El presupuesto depende del equipo: 2 000 partículas con el dibujo en software, 5 000 en equipos
sin sombreadores de cálculo y 10 000, 50 000 o 200 000 en la tarjeta gráfica según su clase. Las
estelas se limitan a 0, 4, 8 o 16 pasos. En el nivel de calidad más bajo caben menos. `wprs-ctl
scene bench` dice dónde corren, cuántas hay vivas y cuánto tarda la simulación; si corren en el
procesador (tarjeta sin sombreadores de cálculo), el estado lo dice y se ven menos.

La capa [`cohetes` del ejemplo](ejemplos/particulas-avanzadas/scene.json) hace volar un cohete
con estela que estalla en estrellas, y hay luciérnagas en el aire.

# Sonido

Hay dos formas de usar el sonido: ponerlo (una capa `sound`) y oírlo (valores que siguen el sonido
que suena en el equipo).

## Una capa de sonido

```json
    { "id": "ambiente", "type": "sound", "sound": "zumbido", "loop": true, "fade_in": 2, "fade_out": 2,
      "play": { "prop": "sonido_activo" },
      "volume": { "timeline": "marea", "keys": [[0, 0.3], [10, 0.8, "in_out_sine"], [20, 0.3, "in_out_sine"]] },
      "pan": -0.2 }
```

Una capa `sound` no dibuja nada y no admite campos visuales (`transform`, `opacity`, `effects`...);
tampoco puede ser máscara. Una escena tiene hasta 8, que se mezclan.

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `sound` | | | El id de un recurso de sonido. |
| `sounds` | | 1 a 16 | Una lista de recursos, en lugar de `sound`. Pon uno de los dos. |
| `loop` | `true` | | Repite sin cortes; con `false` suena una vez cada vez que `play` se enciende. |
| `play` | `true` | | Empieza al encenderse y para al apagarse. Puede seguir una propiedad o claves. |
| `volume` | 1 | 0 a 1 | Cambia con 20 ms de suavizado. |
| `pan` | 0 | -1 a 1 | Izquierda a derecha. |
| `fade_in`, `fade_out` | 0 | 0 a 60 s | Cuánto tarda en subir al empezar y en bajar al parar. |
| `mode` | `sequence` | `sequence`, `random` | Con `sounds`, en orden o al azar (sin repetir seguidas; con la semilla de la escena). |
| `pause` | | `[mín, máx]` de 0 a 600 s | Silencio entre sonidos, elegido con la semilla. |
| `stream` | `false` | | Lee los archivos mientras suenan, para los largos. |
| `play_on` | | | Un disparador que lo inicia: `start`, `beat`, `prop:<id>`, `event:<nombre>`, `click:<capa>`, `enter:<capa>`, `leave:<capa>` o `media:<qué>`. Suena de nuevo cada vez que salta. |

`play`, `volume` y `pan` pueden ser un valor fijo o seguir una propiedad o fotogramas clave del
reloj de la escena, pero nunca el análisis del sonido, el puntero ni un script: la escena no
puede oír su propio sonido.

Los recursos de sonido son OGG (Vorbis u Opus), OPUS, FLAC, WAV o MP3 (solo si se llaman `.mp3`),
mono o estéreo, de 8 a 192 kHz. Cada uno pesa hasta 8 MiB y dura hasta 60 s; todos juntos, hasta
180 s. Con `stream`, hasta 32 MiB y 10 minutos cada uno y 64 MiB en total, fuera de los 180 s.

El sonido sigue el reloj de la escena: suena desde el punto en que la escena dice que debe ir,
y si se desvía más de 60 ms se vuelve a poner en su sitio. Un sonido con `play_on` suena donde
salta el disparador.

Toda escena con sonido tiene una propiedad `volume` (un deslizador de 0 a 100 %, 100 por defecto)
que se añade sola; el manifiesto puede declararla para cambiar su etiqueta, su valor o su orden,
siempre que siga siendo un deslizador de 0 a 100. Las escenas empiezan en silencio, como los
vídeos, y siguen los ajustes `playback.muted` y `playback.volume` que fija el usuario. Si la escena se ve
en varias pantallas, solo una reproduce su sonido, y en la pantalla de bloqueo no suena nada. En el
teléfono solo suena en la pantalla de inicio, y se detiene con el ahorro de batería, con poca batería
o con calor.

El ejemplo [`animacion-e-interaccion`](ejemplos/animacion-e-interaccion/scene.json) reproduce un
sonido al hacer clic en un botón: su línea de tiempo dispara un evento y la capa de sonido lo
usa con `play_on`:

```json
    { "id": "sonido", "type": "sound", "sound": "toque", "loop": false, "play_on": "event:golpe", "volume": 0.6 }
```

## Que la escena reaccione al sonido

Una escena puede moverse con el sonido que suena en el equipo (música, vídeos...). Necesita la
entrada `audio` en el manifiesto (ver [escena.md](escena.md)); el usuario la concede una vez.

Hay tres formas:

1. **Valores**: cualquier número, color o interruptor puede seguir el sonido con
   `{ "audio": "bass" }`, una banda, o un rango de bandas, como se explica en
   [escena.md](escena.md). Con `scale`, `offset`, `clamp` y `smooth` se ajusta cuánto y qué tan
   rápido.
2. **Capas de espectro**: una capa `spectrum` dibuja las barras, el círculo o la onda del sonido
   (ver [capas.md](capas.md)).
3. **Cosas que ya miran el sonido**: el efecto `pulse` (con `audio_band`), el `rate` y los
   `burst` de las partículas con `on: "beat"`, `audio_gate`, y los sombreadores propios con
   `frame.bass`, `frame.beat` y las demás.

```json
    { "id": "latido", "type": "shape", "shape": { "ellipse": { "size": [220, 220] } }, "fill": "#ffffff",
      "opacity": { "audio": "level", "scale": 1.2, "offset": 0.2, "clamp": [0.2, 1], "smooth": 0.1 },
      "transform": { "scale": { "audio": "bass", "scale": 0.5, "offset": 1, "smooth": { "up": 0.03, "down": 0.3 } } } }
```

```json
    { "id": "anillo", "type": "spectrum", "size": [500, 500], "style": "circle", "bands": 64, "channel": "left",
      "color": "#f472b6", "visible": { "prop": "ver_anillo" } }
```

Los valores de sonido van de 0 a 1. `bass`, `mid` y `treble` promedian las bandas por debajo de
250 Hz, hasta 4 kHz y por encima; `level` es la fuerza de todo el sonido; `beat` vale 1 en un
golpe. El análisis tiene 64 bandas por canal (de 40 Hz a 16 kHz), y cada una va en decibelios
por encima de -70 dB, relativos a la banda más fuerte de los últimos segundos; así la música
baja y la fuerte llenan el mismo rango (con `gain` `fixed` se mide desde el máximo digital).

Un bloque `audio` en la raíz de `scene.json` cambia lo rápido que suben y bajan esos valores
(`attack` y `release`) y si se ajustan a lo fuerte que suena la música o al volumen del sistema
(`gain`): ver [escena.md](escena.md).

Una escena solo se vuelve a dibujar cuando cambia lo que oye. Sin audio en el equipo, o sin que el
usuario lo permita, todo lo que sigue al sonido vale 0.

## No cubierto aquí

- Los sonidos que se disparan desde scripts: [scripts.md](scripts.md).
- La carátula y el título de la música que suena (`media`): [escena.md](escena.md).
