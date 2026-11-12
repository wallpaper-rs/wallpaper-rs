# El archivo de la escena

Esta página describe `scene.json` entero: el lienzo, el reloj, los recursos, cuánto cuesta una
escena, los valores que se mueven y las entradas. Las capas están en [capas.md](capas.md), los
efectos en [efectos.md](efectos.md) y las propiedades de usuario en
[propiedades.md](propiedades.md).

Todos los ejemplos de esta página salen de las carpetas de
[`ejemplos/`](ejemplos/), y cada una pasa `wprs-ctl scene validate`.

## Qué hay en la carpeta

Una escena es una carpeta con tres cosas:

- `manifest.json`: el nombre, el autor, la licencia y las propiedades de usuario. Se explica en
  [paquete.md](paquete.md).
- `scene.json`: las capas y todo lo que se mueve. Es lo que describe esta página.
- `assets/`: las imágenes, los sonidos, las fuentes y los modelos que la escena usa.

`scene.json` es un objeto JSON. Solo hace falta `schema` y `canvas`; todo lo demás es opcional.

| Campo | Qué es |
|-------|--------|
| `schema` | Siempre `"wallpaper-rs.scene/1"`. |
| `canvas` | El lienzo: tamaño, ajuste, fondo y efectos de toda la escena. |
| `clock` | El reloj: cuántos fotogramas por segundo y qué momento sirve de póster. |
| `assets` | Los recursos, cada uno con un id. |
| `layers` | Las capas, de abajo a arriba. Ver [capas.md](capas.md). |
| `timelines` | Las líneas de tiempo que mueven valores. |
| `particles` | Los sistemas de partículas. Ver [particulas-y-sonido.md](particulas-y-sonido.md). |
| `effects` | Efectos propios del paquete. Ver [efectos.md](efectos.md). |
| `camera3d` | La cámara de los modelos 3D. Ver [3d.md](3d.md). |
| `audio` | Cómo reacciona la escena al sonido, abajo. |
| `scripts`, `templates`, `pools`, `ext` | Código y plantillas. Ver [scripts.md](scripts.md). |

Si escribes un campo que no existe, un valor fuera de rango o una referencia rota, la escena se
rechaza con la ruta del campo, por ejemplo `/layers/2/effects/0/params/radius is 300, expected
0..=128`. Revisa tu escena con:

```
wprs-ctl scene validate MI-CARPETA
```

No imprime nada si todo está bien, y también compila los scripts de la escena sin ejecutarlos (un
error de sintaxis sale con el archivo y la línea). Para leer o cambiar cualquier campo desde la
terminal, el puntero empieza por `/scene` (lo que hay en `scene.json`: `/scene/layers/0`,
`/scene/canvas/size`) o por `/manifest` (el manifiesto): `wprs-ctl scene show MI-CARPETA /scene/canvas`.

Los colores se escriben `"#rrggbb"` o `"#rrggbbaa"` (siempre con 6 u 8 cifras: `"#fff"` se
rechaza) o como lista `[r, g, b]` o `[r, g, b, a]` con números de 0 a 1.

Las posiciones son píxeles del lienzo con el origen arriba a la izquierda. Los ángulos son grados
y giran en el sentido de las agujas del reloj.

## El lienzo

El lienzo es el tamaño de referencia de tu escena. Sea cual sea la pantalla, tú colocas todo
como si el lienzo midiera `size` píxeles; el programa lo ajusta a cada pantalla.

```json
  "canvas": {
    "size": [1920, 1080], "fit": "fill", "background": "#0b1020", "blend_space": "linear",
    "parallax": { "amount": 1, "delay": 0.15, "pointer": 1 },
    "bloom": { "threshold": 0.8, "strength": 0.6, "radius": 24 },
    "camera": { "zoom": 1.05, "shake": { "enabled": false, "amplitude": 6, "roughness": 0.5, "speed": 1 } }
  }
```

(De [`ejemplos/lienzo-y-reloj`](ejemplos/lienzo-y-reloj/scene.json).)

| Campo | Tipo | Por defecto | Rango | Qué hace |
|-------|------|-------------|-------|----------|
| `size` | `[ancho, alto]` | obligatorio | 1 a 16384 cada uno | Tamaño de referencia en píxeles. |
| `fit` | texto | `fill` | `fill`, `fit`, `stretch`, `center` | Cómo se coloca el lienzo en cada pantalla: `fill` la cubre y recorta lo que sobra, `fit` lo deja entero y el fondo rellena el resto, `stretch` lo estira hasta los bordes, `center` lo deja sin escalar en el medio. |
| `background` | color | `#000000` | | Color de fondo, también el que se ve fuera del lienzo con `fit`. |
| `blend_space` | texto | `linear` | `linear`, `srgb` | `linear` mezcla colores como la luz; `srgb` los mezcla ya codificados, como hacen la mayoría de editores de imagen. |
| `parallax` | objeto | sin paralaje | ver abajo | Cuánto siguen las capas con `parallax` al puntero. |
| `camera` | objeto | sin cámara | ver abajo | Mueve, acerca y gira toda la escena. |
| `bloom` | objeto | sin resplandor | ver abajo | Un resplandor de las partes brillantes sobre toda la escena. |
| `hdr` | sí o no | `false` | | Guarda colores más brillantes que el blanco. |
| `tonemap` | texto | `none` | `none`, `aces`, `agx` | Cómo se aprietan esos colores a la pantalla; solo con `hdr`. |
| `clear` | sí o no | `true` | | Con `false` cada fotograma se dibuja encima del anterior y deja estelas; cuesta más. |
| `perspective` | número | sin definir | 100 a 100000 px | Distancia del espectador para las capas con `transform.tilt` y `transform.z`. |
| `ambient`, `sky` | color | negro | | Luz de alrededor y desde arriba para las capas con luz. Ver [3d.md](3d.md). |
| `wind` | `[x, y]` | sin viento | píxeles por segundo | El viento que mueve huesos con física y follaje. |
| `gusts` | número | 0,35 | 0 a 1 | Cuánto del viento viene en ráfagas. |

### Paralaje

`canvas.parallax` regula el movimiento de las capas que tienen `parallax` (ver
[capas.md](capas.md)).

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `amount` | 1 | 0 a 4 | Cuánto se mueven en total. |
| `delay` | 0,15 | 0 a 2 s | Lo suave que las capas siguen al puntero. |
| `pointer` | 1 | 0 a 1 | Cuánto cuenta el puntero. Con 0 no se necesita puntero. |
| `offset` | `[0, 0]` | -1 a 1 cada uno | Un desplazamiento fijo, como si el puntero estuviera en ese lugar. |
| `gyro` | `false` | | En el teléfono, la inclinación también las mueve. |

`amount`, `pointer` y `offset` pueden seguir una propiedad, una línea de tiempo o el sonido. Si el
sistema pide menos movimiento, las capas se quedan quietas.

### Cámara

`canvas.camera` mueve toda la escena de una vez, sin pasadas extra.

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `center` | el medio | píxeles | El punto del lienzo que se ve en el medio. |
| `zoom` | 1 | 0,1 a 10 | Cuántas veces más grande se ve. |
| `rotation` | 0 | -360 a 360 | Giro en grados. |
| `shake` | sin temblor | | Temblor suave, abajo. |

`shake`: `enabled` (`true`), `amplitude` (8 px, 0 a 1000), `roughness` (0,5, 0 a 1) y `speed`
(1 Hz, 0 a 60). Mientras tiembla pide un fotograma cada vez. Se apaga si el usuario pide menos
movimiento y en el nivel de calidad más bajo. Todos los campos de la cámara pueden seguir un valor.

### Resplandor

`canvas.bloom` hace brillar lo claro de toda la escena, como una capa de ajuste arriba del todo
con el efecto `bloom` (ver [efectos.md](efectos.md)).

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `threshold` | 0,8 | 0 a 4 | Desde qué brillo empieza a brillar. |
| `strength` | 1 | 0 a 8 | Cuánto brilla. Con 0 no cuesta nada. |
| `radius` | 16 | 0 a 128 px | Hasta dónde se extiende. |
| `tint` | blanco | | Color del resplandor. |
| `hdr` | | `threshold` 0 a 16, `strength` 0 a 8 | Valores que reemplazan a los anteriores en el nivel `q3` y en `q2` con corriente. |

## El reloj

```json
  "clock": { "max_fps": 30, "poster_time": 2, "seed": 7 }
```

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `max_fps` | 60 | 1 a 240, o `"display"` (también 0) | Tope de fotogramas por segundo. `"display"` sigue el ritmo de cada pantalla. |
| `poster_time` | 0 | 0 a 86400 s | El instante que se enseña donde no se quiere movimiento y como miniatura de la biblioteca. |
| `seed` | 0 | número entero, o `null` | La semilla de lo que se hace al azar. Con `null` cambia en cada arranque. |
| `step` | cada minuto | 1 a 3600 s | Cada cuánto cambian los valores que siguen al día (ver "Valores"). |

Una escena de reloj o de imagen casi quieta no necesita 60 fotogramas: baja `max_fps` (`1` a `30`)
y ahorra batería. Lo que el usuario fija (`playback.max_fps`) manda sobre tu valor.

## Los recursos

Los recursos son los archivos que la escena usa. Cada uno tiene un id de hasta 32 letras
minúsculas, cifras o `_`, y una ruta dentro de la carpeta.

```json
  "assets": { "cielo": { "path": "assets/cielo.png", "filter": "linear", "wrap": "clamp" } }
```

Las capas nombran el recurso por su id (`"image": "cielo"`). Una escena tiene como mucho 256
recursos.

| Tipo | Formatos | Límites |
|------|----------|---------|
| Imagen | PNG, JPEG, WebP, GIF, BMP, KTX2 | Las imágenes de una escena ocupan como mucho 512 MiB ya abiertas. |
| Fuente | TTF, OTF, TTC, OTC, WOFF2 | Hasta 32 MiB cada una. |
| Vídeo | WebM, MP4, MKV | Se guardan tal cual, sin comprimir de nuevo. |
| Sonido | OGG (Vorbis u Opus), OPUS, FLAC, WAV, MP3 | Ver [particulas-y-sonido.md](particulas-y-sonido.md). |
| Tabla de color | `.cube` | `LUT_3D_SIZE` de 2 a 65, hasta 16 MiB cada una y 64 MiB en total. |
| Modelo 3D | `.glb` (glTF binario) | Hasta 64 MiB cada uno. Ver [3d.md](3d.md). |

`scene.json` pesa como mucho 8 MiB.

Campos de un recurso:

| Campo | Qué hace |
|-------|----------|
| `path` | La ruta del archivo. Obligatorio. |
| `linear` | `true` para imágenes que son datos y no colores: mapas de profundidad, de normales o de flujo, máscaras. |
| `filter` | `linear` (suave) o `nearest` (píxeles nítidos). |
| `wrap` | Qué pasa fuera de la imagen: `clamp`, `repeat` o `mirror`. Con `repeat` o `mirror`, una capa de imagen más grande que la imagen se llena de copias en vez de estirarla. |
| `mipmaps` | `true` o `false` fuerza las copias pequeñas para imágenes que se ven más chicas. |
| `atlas`, `rects` | Una hoja de dibujos: `[filas, columnas]` de celdas iguales, o hasta 256 rectángulos `[x, y, ancho, alto]`. No las dos a la vez. |
| `frames` | Cuántas celdas se reproducen, de 1 a 256. |
| `fps`, `durations` | Fotogramas por segundo (0,1 a 120) o una duración por fotograma (0,001 a 60 s). No los dos. |
| `animated` | `true` reproduce un GIF o WebP animado con sus tiempos; hasta 256 fotogramas. |
| `face` | Qué fuente de una colección `.ttc` o `.otc` (0 a 255). |
| `media` | `current` o `previous`: la carátula de lo que suena o sonó antes. `path` es la imagen mientras no suena nada. |
| `shapes` | En un `.glb`, formas de mezcla esculpidas aparte. Ver [3d.md](3d.md). |

Sin `fps` ni `durations`, una hoja enseña su primer fotograma. Una capa de imagen que enseña una
hoja reproduce sus fotogramas con el reloj de la escena y los controla con `playback` (ver
[capas.md](capas.md)).

Cada imagen se abre una vez y se sube una vez a la tarjeta gráfica, aunque la escena se vea en
varias pantallas. Un vídeo tiene un solo decodificador compartido por todas las capas que lo
enseñan.

Las fuentes del sistema se nombran por familia; solo `sans-serif`, `serif` y `monospace` existen
seguro en todas partes.

### Texturas KTX2

Un recurso de imagen puede ser un archivo `.ktx2` con una textura Basis Universal: una imagen, una
lista de hasta 256 fotogramas (se ordenan en una cuadrícula, como los de un GIF animado) o un cubo
de seis caras, que se vuelve un panorama del doble de ancho que de alto y vale donde vale un
panorama, como el `env` de un modelo (ver [3d.md](3d.md)). Mide hasta 16384 píxeles de lado, tiene
hasta 15 niveles y pesa hasta 256 MiB. El programa la convierte al formato que la tarjeta gráfica
lee mejor, y una escena de texturas KTX2 ocupa cerca de la cuarta parte de memoria de vídeo que
las mismas imágenes en PNG. `linear`, `filter`, `wrap` y `atlas` valen igual que en otras imágenes,
y con `animated: true` los fotogramas se reproducen con los tiempos del archivo. Una textura HDR
conserva su luz por encima del blanco en los lienzos con `hdr`.

Se hacen en el escritorio, desde la conversión a KTX2 del editor o con:

```
wprs-ctl scene asset MI-CARPETA cielo --ktx2 --codec uastc --quality 80
wprs-ctl scene asset MI-CARPETA panorama --cube
```

`--dry-run` solo informa de los tamaños y de lo que recibe cada clase de tarjeta, y `--faces` hace
el cubo con seis imágenes cuadradas. El teléfono solo las lee, no las crea: un paquete hecho en el
escritorio se ve igual en el teléfono.

## Cuánto cuesta una escena

Que tu escena se vea bien en un teléfono depende de cuánto trabajo pide cada fotograma. El
programa lo estima antes de dibujar, sin tarjeta gráfica, en **unidades**. Una unidad es una
pasada de pantalla completa que lee un píxel por píxel.

- Una capa suma las unidades de sus efectos, una más si se dibuja aparte (con efectos, máscara,
  `cache` o un modo de fusión que no sea `normal`, `add`, `multiply` o `screen`) y dos más si su
  modo de fusión copia lo que hay debajo.
- Todo eso se multiplica por la parte del lienzo que la capa cubre. Una capa de ajuste cubre todo.
- Una capa cuya posición puede cambiar (animada, con `parallax`, `parent`, `tilt` o con scripts
  en la escena) cuenta como si cubriera todo el lienzo.
- Cada fotograma suma 1,5 unidades por componer y copiar. Una `camera` hace que toda capa cuente
  como lienzo entero.

### Niveles de calidad

El programa dibuja la escena en uno de cuatro niveles de calidad, del más bajo `q0` al más alto
`q3` (en la app: calidad más baja, baja, media y más alta). Cada nivel hace los efectos más
pequeños y deja algunos de lado. Cada tipo de equipo aguanta una cantidad de unidades:

| Equipo | Unidades por fotograma |
|--------|------------------------|
| Sin tarjeta gráfica, o sin sombreadores de cálculo | 4 |
| Gráficas integradas antiguas y teléfonos | 8 |
| Gráficas integradas nuevas | 16 |
| Tarjetas gráficas dedicadas | 40 |

El nivel que se usa es el más alto cuyo costo cabe. Después, el programa mide el tiempo real de
cada fotograma y sube o baja el nivel por sí solo. Funcionando con batería baja un escalón, con ahorro de
energía otro más, y con calor baja y limita el ritmo. Si una escena no cabe ni en `q0`, el
programa muestra su `poster_time` y no la mueve.

El usuario puede fijar el nivel (`q0` a `q3`) o un mínimo; ese ajuste manda siempre, también con
batería. Entonces una escena pesada pierde fotogramas en vez de detalle.

### Qué hacer para que corra en un teléfono

1. Mira la advertencia. `wprs-ctl scene validate` y `wprs-ctl pack check` avisan cuando la escena
   cuesta más de 8 unidades en `q2`:
   `the scene costs 8.3 units a frame at quality level 2, more than the 8 that older integrated
   and phone GPUs have, so they show it with fewer details`, seguido de cómo bajarlo (quitar capas y
   efectos que no hacen falta o aligerar los más pesados en el panel de cada efecto, que muestra su
   costo en cada nivel). En la app el aviso sale en español. Eso lo produce
   [`ejemplos/efectos-apilados`](ejemplos/efectos-apilados/scene.json), con resplandor, desenfoque,
   viñeta y grano sobre toda la escena.
2. Prefiere pocos efectos de costo bajo. Los efectos pesados (`fluid`, `bloom`, `god_rays`,
   `local_contrast`, `tilt_shift`, `water_ripple`) hacen mucho trabajo; márcalos con
   `"optional": true` para que el programa los quite antes que a los demás.
3. Usa las capas de ajuste con `size` en vez de toda la pantalla, y las máscaras en vez de efectos
   sobre capas enormes.
4. Baja `clock.max_fps` si nada se mueve rápido.
5. Cada mezcla que no sea `normal`, `add`, `multiply` o `screen` lee lo que hay debajo con una
   copia, que en un teléfono cuesta cerca de un milisegundo por capa. Si el aspecto lo permite,
   usa `add` o `screen`.
6. Parte del trabajo se salta solo: lo que no cambia no se vuelve a dibujar, y una escena que no
   se mueve no pide fotogramas.
7. Un tope duro vale para todos: entre todos los efectos de una escena solo se ejecuta una cantidad
   de trabajo por fotograma (2 000, 4 000, 8 000 o 16 000 unidades según el equipo) y como mucho
   16 efectos con memoria propia (`state`). Lo que no cabe se deja fuera con un aviso.

Para medir de verdad, construye el paquete y usa `bench`:

```
wprs-ctl scene build MI-CARPETA -o mi.wpak
wprs-ctl scene bench mi.wpak --quality q2 --tier t1
```

Imprime el costo estimado, el tiempo por fotograma, la memoria que ocupa y las notas de lo que se
dejó fuera. `--shot cuadro.png` guarda el último fotograma como imagen y `--tier t0` a `t3`
aplica los límites de cada clase de equipo (`t1` es el de los teléfonos). Lo mismo pasa con
`--quality auto`, que sube y baja el nivel como lo haría el teléfono. `bench` mide el paquete tal
como lo describe `scene.json`: no ejecuta los scripts. Para ver el resultado de los scripts a una
hora fija, `wprs-ctl scene render MI-CARPETA -o vista.png --time-of-day 15:42`.

### Cuando el usuario pide menos destellos

Si el usuario activa `playback.flash_guard`, el programa mide el brillo de cada fotograma y, si la
escena destella más de tres veces por segundo, la reproduce más despacio hasta que destelle como
mucho tres veces, y si aun así destella más rápido, se detiene en su fotograma. Viene apagado. Si
tu escena destella a propósito, dilo en el manifiesto con `photosensitive` (ver
[paquete.md](paquete.md)).

### Cuando el usuario pide menos movimiento

Si el sistema pide reducir el movimiento, la escena se queda en `poster_time`: el paralaje, el
temblor de cámara y las líneas de tiempo se paran. El fotograma se redibuja una vez por minuto,
para que los relojes sigan bien.

## Valores que se mueven

Casi todo número, color, interruptor, texto e imagen de una capa puede ser un valor fijo o un
objeto con exactamente **una** fuente. Estas son las fuentes:

| Fuente | Ejemplo | Para qué |
|--------|---------|----------|
| Propiedad de usuario | `{ "prop": "tema", "map": { "dia": "#ffe08a", "noche": "#8fb4ff" } }` | Lo que el usuario elige. Ver [propiedades.md](propiedades.md). |
| Fotogramas clave | `{ "timeline": "salto", "keys": [[0, 0.4, "out_bounce"], [3, 0.4]] }` | Una animación en el tiempo. |
| Sonido | `{ "audio": "bass" }`, `{ "audio": 3, "channel": "left" }` | Lo que suena en el equipo. |
| Puntero | `{ "cursor": "x" }` | Dónde está el ratón o el dedo. |
| Reloj | `{ "clock": "HH:mm" }`, `{ "clock": "hour" }` | La hora y la fecha locales. |
| Medios | `{ "media": "title" }` | Lo que suena en el reproductor. |
| Script | `{ "script": "scripts/giro.js" }` | Ver [scripts.md](scripts.md). |
| Capa | `{ "layer": "otra" }` | Solo en parámetros `texture` y `mask` de efectos. |

El ejemplo completo está en [`ejemplos/valores`](ejemplos/valores/scene.json).

### Propiedades

`{ "prop": "id" }` toma el valor de una propiedad de usuario. Con `map` se elige un valor según
la opción: `{ "prop": "tema", "map": { "dia": "#ffe08a", "noche": "#8fb4ff" } }`. En imágenes y
vídeos, `map` cambia de recurso: `{ "prop": "estilo", "map": { "noche": "cielo" } }`.

### Fotogramas clave

Una lista `keys` de `[tiempo, valor]` o `[tiempo, valor, suavizado]`. El tiempo va en segundos de
la línea de tiempo que nombres con `timeline`, o del reloj de la escena si no nombras ninguna.
Sin línea de tiempo, `loop` (`once`, `repeat`, `pingpong`) dice qué pasa al llegar al final. Hasta
256 claves por valor.

El suavizado hasta la clave siguiente puede ser `linear`, `step`, `hold` (salta de golpe), o
`in_`, `out_`, `in_out_` seguido de `sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`,
`back`, `elastic` o `bounce`, o una curva Bézier `[x1, y1, x2, y2]`.

Los colores se mezclan en el espacio OKLab, por tono, saturación y brillo, y el tono toma el
camino corto: de rojo a verde pasa por naranja y no por un oliva oscuro.

Variantes:

| Forma | Ejemplo | Qué hace |
|-------|---------|----------|
| Asas | `[2, 1, { "in": [0.5, 0], "out": [0.5, 0.2] }]` | Curvas Bézier a medida alrededor de una clave. `out: [dt, dv]` es el punto (t + dt, v + dv) y `in: [dt, dv]` el punto (t − dt, v − dv); asas iguales dan una clave suave, `dv: 0` una plana. En posiciones, colores y listas, `dv` es la fracción del camino entre las dos claves. Un lado puede ser una lista de asas, una por componente. |
| Partes por separado | `{ "x": { "keys": ... }, "y": { "cursor": "y", "scale": 100 } }` | Cada parte de una posición, y `r`, `g`, `b`, `a` de un color, es un valor propio. La parte que falta toma el valor de `base`, o 0 (opacidad 1). |
| Relativo | `{ "base": [960, 900], "keys": [[0, [-300, 0]], ...] }` | Las claves se suman a `base`. |

### Sonido

`{ "audio": "bass" }` va de 0 a 1. Hay 64 bandas por canal, de 40 Hz a 16 kHz. Cada una va en
decibelios por encima de -70 dB, relativos a la banda más fuerte de los últimos segundos, así que
la música baja y la fuerte llenan el mismo rango.

- Un número es una banda (`{ "audio": 3 }`); un par es un rango (`{ "audio": [0, 3], "of": 16,
  "reduce": "max" }`, con `of` 16, 32 o 64 y `reduce` `mean` o `max`).
- Los nombres: `bass` (bajo 250 Hz), `mid` (hasta 4 kHz), `treble` (por encima), `level` (todo el
  sonido) y `beat` (1 en cada golpe).
- `channel` es `average` (por defecto), `left` o `right`.
- El valor sube en 20 ms y baja en 200 ms, salvo que la escena cambie `audio`, abajo.

Una escena con `audio` necesita la entrada de audio. Todo lo que sigue al sonido es 0 mientras no
hay audio.

### Puntero

`{ "cursor": "x" }`, con `x`, `y` (de 0 a 1, relativas a la pantalla), `inside` (1 si el puntero
está en la pantalla) y `speed`. El puntero sigue al real con un muelle que se asienta en unos
0,15 s, y vuelve al centro cuando sale. En el teléfono solo cuenta cuando se toca la pantalla.

### Reloj y fechas

- Texto: `{ "clock": "HH:mm:ss a" }` con `HH`, `H`, `hh`, `h`, `mm`, `m`, `ss`, `s` y `a`.
- Número: `{ "clock": "day" }` va de 0 a casi 1 durante el día; además `hour` (0 a 23), `minute`,
  `second` (0 a 59), `weekday` (1 el lunes a 7) y `month` (1 a 12). Todo en hora local. `day`
  cambia como mucho cada minuto, o cada `clock.step` segundos.
- Fechas, solo en texto: `{ "date": "EEEE d 'de' MMMM yyyy" }` con `yyyy`, `yy`, `MMMM`, `MMM`,
  `MM`, `M`, `dd`, `d`, `EEEE`, `EEE`, `D` (día del año) y `w` (semana ISO), más los símbolos de
  la hora. Los nombres salen en el idioma del paquete (español o inglés; inglés para los demás).
  El texto entre comillas simples se queda tal cual, y `''` escribe una comilla.

### Medios

`{ "media": "title" }` sigue lo que toca el reproductor del equipo: los textos `title`, `artist`,
`album`, `album_artist`, `subtitle`, `genres` y `kind`; los números `position`, `duration` (en
segundos), `progress` (0 a 1) y `playing`; y los colores `primary`, `secondary`, `tertiary` de la
carátula, `text` (legible sobre `primary`) y `contrast` (blanco o negro). Con `placeholder` (un
texto, o uno por idioma, hasta 512 caracteres) se dice qué mostrar mientras no suena nada. Los
números también valen como texto (`m:ss` para `position` y `duration`, un porcentaje para
`progress`). Necesita la entrada `media`.

### Retoques de un valor

Un valor que no es fijo acepta, en este orden: `bounds`, `exponent`, `invert`, `scale`, `offset`,
`clamp` y `smooth`.

| Campo | Qué hace |
|-------|----------|
| `bounds: [a, b]` | Lleva el valor a (x − a) / (b − a), recortado a 0 y 1. |
| `exponent` | De 0,1 a 10: eleva el valor, lo curva. |
| `invert` | Lo da la vuelta (1 − valor). |
| `scale`, `offset` | Multiplica y suma. |
| `clamp: [min, max]` | Lo recorta. |
| `smooth` | Segundos de suavizado, o `{ "up": 0.05, "down": 0.4 }` para subir y bajar a distinta velocidad. |

### Capa como textura

`{ "layer": "id" }` en un parámetro `texture` o `mask` de un efecto dibuja esa capa aparte, aunque
tenga `visible: false`, y la usa como imagen. Una capa no puede alimentarse a sí misma.

## Líneas de tiempo

```json
  "timelines": [{ "id": "salto", "duration": 3, "loop": "repeat" }]
```

Hasta 64 por escena.

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `id` | | Su nombre. |
| `duration` | | Segundos. En lugar de esto puede contar `frames` a `fps` (1 a 240), nunca los dos con `duration`. Las claves siguen en segundos. |
| `loop` | `once` | `once`, `repeat` o `pingpong`. |
| `speed` | 1 | -100 a 100. Por debajo de 0 va hacia atrás. |
| `delay` | 0 | Segundos antes de empezar. |
| `autoplay` | `true` | Con `false` espera a su disparador. |
| `trigger` | | Qué la inicia: `start`, `beat`, `prop:<id>` (una propiedad, o un botón), `event:<nombre>`, `click:<capa>`, `enter:<capa>`, `leave:<capa>` o `media:play`, `media:pause`, `media:track`, `media:cover`. |
| `wrap` | `false` | Solo con `repeat`: funde el final con el principio de la vuelta siguiente. |
| `events` | | Hasta 64 `{ "t": segundos, "name": "nombre" }` (`[a-z0-9_]{1,32}`). Cada uno se dispara una vez por vuelta, y pueden iniciar otras líneas de tiempo y sonidos con `event:<nombre>`. |

`click:`, `enter:` y `leave:` funcionan con capas `solid` (ver [capas.md](capas.md)). `click:`
necesita la entrada `clicks` y los otros dos `pointer`.

## Entradas

Algunas escenas necesitan datos del equipo. Cada una es una entrada que declaras en el manifiesto
(`"inputs": ["audio", "pointer"]`) y que el usuario debe permitir:

| Entrada | La usan | Qué ve la escena |
|---------|---------|------------------|
| `audio` | `{ "audio": ... }`, efectos con sonido | El análisis del sonido. |
| `pointer` | `cursor`, `parallax`, `follow` de efectos | Dónde está el puntero. |
| `clicks` | `click:` y `solid` | Los clics en capas `solid`. |
| `media` | `{ "media": ... }`, carátulas | El reproductor de música. |
| `network` | páginas web | Fuera de las escenas de capas. |

No tienes que adivinar. Si usas algo y falta la entrada, `wprs-ctl scene validate` dice por
ejemplo `inputs: the scene uses pointer at /layers/0 of scene.json; add "pointer"`. Para añadirla:

```
wprs-ctl scene manifest MI-CARPETA inputs --add pointer
```

La escena solo se vuelve a dibujar cuando cambia una entrada que usa. Las teclas nunca llegan a
una escena, y los clics están apagados en cada pantalla hasta que el usuario los activa.

## Reacción al sonido de toda la escena

```json
  "audio": { "attack": 0.02, "release": 0.25, "gain": "auto" }
```

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `attack` | 0,02 s | 0,001 a 2 | Cuánto tarda en subir un valor de sonido. |
| `release` | 0,2 s | 0,01 a 10 | Cuánto tarda en bajar. |
| `gain` | `auto` | `auto`, `fixed` | `auto` se ajusta a lo fuerte que suene; `fixed` mide desde el máximo digital, así que sigue el volumen del sistema. |

## No cubierto aquí

- Las capas y sus campos: [capas.md](capas.md).
- Los efectos y los efectos propios: [efectos.md](efectos.md).
- Partículas y sonido: [particulas-y-sonido.md](particulas-y-sonido.md).
- Propiedades de usuario y preajustes: [propiedades.md](propiedades.md).
- Cámara 3D, luces y sombras: [3d.md](3d.md). Mallas con huesos: [mallas.md](mallas.md).
- `scripts`, `templates`, `pools` y `ext`: [scripts.md](scripts.md).
- El manifiesto, la firma y el paquete: [paquete.md](paquete.md).
