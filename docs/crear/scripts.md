# Scripts

Un script es un archivo de JavaScript que va dentro de la escena. Lo escribes tú y la escena lo
ejecuta en cada cuadro. Sirve para lo que las propiedades de `scene.json` no alcanzan: una cuenta,
una elección al azar, un estado que cambia con el tiempo o con un clic.

Antes de escribir uno, mira si basta con lo que ya tiene la escena. Casi todo lo que se mueve con
el tiempo, el sonido o el puntero se declara en `scene.json` sin código (`escena.md`, `capas.md`,
`propiedades.md`). El script es para la lógica.

## Qué puede y qué no puede hacer

Un script puede leer el tiempo, el puntero, el sonido, las propiedades que elige el usuario y lo que
suena en el reproductor de medios. Puede cambiar las capas, los efectos, las partículas, el sonido,
las líneas de tiempo y la cámara de la escena.

Un script **no** puede:

- abrir archivos ni saber en qué carpeta está algo. Solo ve los archivos del paquete por su nombre
  y, de lo que elige el usuario, un identificador que no es una ruta;
- usar la red ni lanzar otros programas;
- leer el teclado, la batería, los nombres de las salidas o monitores ni la lista de fuentes;
- cambiar las propiedades que elige el usuario. Las lee y nada más;
- crear capas de la nada. Solo copia las plantillas que declara la escena.

Esto lo impone el sistema, no la buena voluntad del script: el script corre en un proceso aparte,
sin archivos, sin red y con un filtro que solo deja pasar las llamadas al sistema que hacen falta
para calcular. En el teléfono corre en un hilo aparte con los mismos límites.

### Límites

| Qué | Límite |
|-----|--------|
| Módulos por escena | 64 en total; más de 16 hacen que el paquete pida un wallpaper-rs más nuevo (`scripts2`) |
| Tamaño | 1 MiB entre todos los módulos |
| Memoria | 32 MiB |
| Pila | 256 KiB |
| Tiempo al cargar y en el primer cuadro | 100 ms |
| Tiempo por cuadro | 4 ms de procesador, sumados todos los módulos |
| Resultado de un cuadro | 256 KiB |
| Líneas de `console` | 20 por segundo, de 512 caracteres cada una |
| Temporizadores | 64 esperando a la vez; ninguno de menos de 16 ms |
| `shared` | 256 valores, 65 536 elementos y 32 niveles por valor |
| `storage` | 64 KiB en cada ámbito |
| Copias de plantillas | 256 a la vez, y 512 capas en total en la escena |
| Capas que informan cada cuadro | las primeras 64 que el script lee o pide |
| Eventos del puntero | 64 por cuadro |

Los resultados llegan al cuadro siguiente, así que un script lento nunca frena el dibujo: como
mucho, la escena se ve con un cuadro de retraso. Si un cuadro se pasa de tiempo o de memoria, ese
cuadro falla entero; tres cuadros seguidos así apagan los scripts de la escena (la escena sigue
dibujando sin ellos). Una función que lanza un error escribe el error en la consola y el cuadro
sigue con los demás módulos; un módulo cuyas funciones fallan en tres cuadros seguidos se detiene
solo, con sus temporizadores.

Un script cuyos resultados no cambian durante 30 cuadros hace que la escena lo ejecute solo dos veces
por segundo, o en su próximo temporizador, hasta que algo cambie. Una escena sin `update` solo corre
los scripts cuando hay un evento: un temporizador, un cambio de propiedades, un evento de línea de
tiempo o del puntero. Exporta `update` solo si lo necesitas: sin él, la escena no dibuja cuadros
para el script.

## El lenguaje

Es JavaScript moderno con módulos: `import`, `export`, funciones flecha, clases, desestructuración,
`Map`, `Set`, `Math`, `JSON`, `Date`. Estas cosas **no** existen: `fetch`, `XMLHttpRequest`,
`require`, `window`, `document`, `process`, `localStorage`, `Intl`, `TextEncoder`,
`structuredClone` ni `WebAssembly`. Las promesas y `async`/`await` no se ejecutan: no las uses.
Los temporizadores son los de la escena (más abajo), no los de un navegador.

Cada archivo es un módulo. Un módulo solo puede importar tres cosas:

```js
import { scene, engine, props } from "wprs";
import { Vec2, mix, smoothstep } from "wprs/math";
import { ayuda } from "./lib/ayuda.js";
```

`wprs` es la interfaz de la escena, `wprs/math` son las funciones de cálculo y la tercera forma
es otro script de la misma escena, con ruta relativa y que esté en la lista `scripts`.

`Math.random()` sigue la semilla del reloj de la escena: con `"clock": { "seed": 7 }` da siempre la
misma serie, y con `"clock": { "seed": null }` una distinta en cada arranque (`escena.md`).

`Date` usa la zona horaria del equipo, también cuando cambia. Para escribir fechas y horas en el
idioma de quien usa el fondo, usa `time.format(...)` (más abajo).

## Cómo se agrega un script a la escena

1. Pon el archivo `.js` en la carpeta de la escena, por ejemplo `scripts/reloj.js`.
2. Lístalo en `scene.json`, en `scripts`:

```json
"scripts": [
  { "path": "scripts/sigue.js", "events": ["update"], "uses": ["pointer"] }
]
```

Hay dos formas de hacerlo sin editar el JSON a mano:

```
wprs-ctl scene import-script mi-escena scripts-mios/sigue.js
wprs-ctl scene import-script mi-escena --template timer
```

La primera copia el archivo a `scripts/` y escribe la declaración (`events` y `uses`) a partir de
lo que exporta e importa. Para `sigue.js` imprime:

```
scripts/sigue.js se añadió
scripts/sigue.js ahora usa pointer
```

`update` no aparece en `events` porque todas las versiones lo llaman sin que se declare. La segunda parte de una plantilla: `empty`, `media`, `touch`,
`timer`, `audio`, `spawn`, `wave`, `daytime`, `setting` o `launches`. En el editor, el panel
*Scripts* tiene *Importar un script…* y hace lo mismo (`editor.md`).

### La declaración

Cada entrada de `scripts` puede decir qué necesita el módulo:

| Campo | Qué es |
|-------|--------|
| `path` | La ruta del archivo dentro del paquete. Cada ruta una sola vez. |
| `events` | Las funciones que exporta y los eventos que atiende: `init`, `update`, `destroy`, `resize`, `properties`, `settings`, `timelineEvent`, `mediaStatus`, `mediaPlayback`, `mediaProperties`, `mediaThumbnail`, `mediaTimeline`, y los del puntero `click`, `down`, `up`, `enter`, `leave`, `move`. |
| `uses` | Las partes de `wprs` que usa: `audio`, `pointer`, `clicks`, `timers`, `layers`, `effects`, `particles`, `math`, `console`, `shared`, `media`, `storage`, `geometry`, `sounds`, `animations`, `camera`, `bloom`, `shortcuts`, `hierarchy` y `folders`. |
| `params` | Hasta 32 ajustes del propio módulo: `{ "tipo": ..., "default": ..., "min": ..., "max": ... }` con tipo `number`, `bool`, `text`, `color` o `vec2`. Los ids son `[a-z0-9_]{1,32}`. |

La declaración tiene dos efectos. El primero: la escena sabe qué entradas usa el script sin leer su
texto (el sonido, el puntero, los clics). El segundo: una versión de wallpaper-rs que no conoce algo
de lo que declaras rechaza el paquete en vez de ejecutarlo mal. `wprs-ctl pack build` pone en el
manifiesto las funciones que hagan falta (`requires.features`, ver `paquete.md`).

Hay dos modos, según lo que declares:

- **Sin declaración** (solo `path`): el módulo recibe una interfaz reducida, igual en todos los
  motores, y solo se llaman `update` y `timelineEvent`. Tiene `engine` (`time`, `dt`, `locale`,
  `daytime`), `time`, `props` (`get`, `label`, `pressed`, `color`), `audio` (`bands` y los niveles),
  `input.cursor` (`x`, `y`, `inside`, `buttons`), `scene.layer(id)` con `position`, `scale`,
  `rotation`, `opacity`, `visible`, `color` y `text`, `timeline` y `particles`. Lo que escribe en
  una capa vale solo para ese cuadro: hay que escribirlo en cada cuadro. No puede importar
  `wprs/math` ni otro módulo, ni usar `console` ni temporizadores. Es el modo del ejemplo del reloj.
- **Con declaración** (`events`, `uses` o `params`): tiene toda la interfaz que describe esta
  página. Para usar `timers`, `layers` o `effects` tiene que declararlos; sin declararlos, la llamada
  falla. Lo que escribe en una capa dura hasta que lo cambie otra vez o llame a `reset`.
  Cualquier evento que no sea `update` y cualquier parte que no sean `audio`, `pointer` y `clicks`
  hacen que el paquete pida `scripts2`.

Las entradas que usa un módulo salen de lo que declara (`uses`) o, sin declaración, de lo que
importa de `wprs` (`audio` y `input`). Además el manifiesto tiene que listarlas en `inputs`
(`paquete.md`): `pack check` te dice cuál falta y en qué parte de `scene.json`.

## Las funciones que exporta un módulo

Un módulo exporta las funciones que quiera. La escena llama a cada una dentro del tiempo de un cuadro.

| Función | Cuándo se llama |
|---------|-----------------|
| `init(ctx)` | Una vez, en el primer cuadro. `ctx.params` son los valores por defecto de `params` y `ctx.engine` es `engine`. |
| `update(frame)` | En cada cuadro mientras se ve la pantalla. Lo que devuelve es el valor de las fuentes `{ "script": "<su ruta>" }` de `scene.json`. |
| `destroy()` | Cuando la escena se detiene. |
| `resize({ screen, canvas })` | Cuando cambia el tamaño o el giro de la pantalla. |
| `properties(cambiadas)` | En el primer cuadro, con todas las propiedades que eligió el usuario; después, solo con las que cambian. |
| `settings({ language, fps, quality })` | En el primer cuadro y cuando cambian el ritmo de cuadros o la calidad. |
| `timelineEvent(e)` | Por cada evento `{ timeline, name, t }` de una línea de tiempo o un clip. |
| `mediaStatus({ enabled })` | En el primer cuadro y cuando el usuario concede o retira `media` (módulos que declaran `media`). |
| `mediaPlayback({ state })` | Cuando el reproductor empieza (`playing`), se pausa (`paused`) o se detiene (`stopped`). |
| `mediaProperties({ title, artist, album, album_artist, subtitle, genres, kind })` | Cuando llega otra pista o cambian sus textos; también con textos vacíos cuando termina. |
| `mediaThumbnail({ has_cover, colors })` | Cuando llega o se va una carátula. `colors` trae `primary`, `secondary`, `tertiary`, `text` y `contrast` como `{ r, g, b, a }`, y nunca la imagen. |
| `mediaTimeline({ position, duration, rate })` | Cuando empieza una pista, cambia su duración o su ritmo, o el reproductor salta. En segundos. |

`frame`, el argumento de `update`, tiene `time` y `dt` (segundos), `props`, `audio`, `cursor`,
`actions`, `wall` (la hora del reloj de pared en segundos), `zone`, `timelines` y `events`. Casi
siempre es más cómodo usar `engine`, `props`, `audio` e `input`, que dan lo mismo.

### Un módulo, muchos valores

Cualquier valor animable de `scene.json` se puede atar a una función de un módulo con
`{ "script", "fn", "params" }`. `fn` es una exportación que no sea de las de arriba. Así un módulo
sirve para muchos valores, cada uno con sus parámetros. Hay dos formas de exportarla:

- una función que se llama en cada cuadro como `fn(valor, ctx)`;
- un objeto con `init(valor, ctx)` (primer cuadro) y `update(valor, ctx)` (cada cuadro).

`valor` es lo que devolvió la última vez (`null` la primera). `ctx` tiene `path` (el JSON Pointer
del valor en `scene.json`), `layer` (el id de la capa que lo tiene), `key` y `params` (los `params`
del módulo con los del enlace encima). Lo que devuelve se suma a `base` como cualquier otra fuente.
`params` sin `fn` no vale. El ejemplo *Aspas que giran* (más abajo) lo usa.

## La interfaz `wprs`

Todo sale de `import { ... } from "wprs"`.

### `engine`: el estado de la escena

| Campo | Qué da |
|-------|--------|
| `time`, `dt` | El tiempo de la escena y lo que pasó desde el cuadro anterior, en segundos. |
| `daytime` | La hora local del día, de 0 (medianoche) a casi 1. |
| `fps`, `quality` | El ritmo de cuadros y el nivel de calidad (0 a 3). |
| `locale`, `language` | El idioma elegido entre los del paquete, en minúsculas, o `""`. |
| `canvas`, `screen` | `[ancho, alto]` del lienzo y de la pantalla, sin nombres de salidas. |
| `device` | `desktop` o `mobile`. |
| `mode` | `wallpaper`, `lockscreen`, `screensaver`, `editor` o `preview`. |
| `orientation` | `landscape` o `portrait`. |

### `time.format(patrón, fecha)`

Da la fecha (la de `fecha`, o ahora) escrita con las letras de los valores `date` de `scene.json`,
en el idioma de `engine.locale`: `time.format("HH:mm")`, `time.format("EEEE d 'de' MMMM")`.

### `props`: las propiedades que elige el usuario

| Función | Qué da |
|---------|--------|
| `props.get(id)` | El valor actual. Un archivo o carpeta elegidos dan `"wpak-user://<id>"`, nunca la ruta, y `null` mientras no haya ninguno. |
| `props.all()` | Todos los valores, en un objeto. |
| `props.label(id)` | El texto de la opción elegida de un combo, en el idioma de quien usa el fondo. |
| `props.color(id)` | Un color como `{ r, g, b, a }` de 0 a 1. |
| `props.pressed(id)` | `true` si se pulsó ese botón desde el cuadro anterior. |

Un script nunca escribe propiedades. Si quiere que algo cambie con una propiedad, la lee en `update`
o en `properties` y cambia las capas.

### `audio`: el sonido

| Parte | Qué da |
|-------|--------|
| `bass`, `mid`, `treble`, `level` | Niveles de 0 a 1. |
| `beat` | `true` en el cuadro de un golpe de ritmo. |
| `bands(n, canal)` | Una lista nueva de `n` bandas (16, 32 o 64) del canal `"left"`, `"right"` o `"average"` (el predeterminado). |
| `buffers(n)` | `{ left, right, average }` como `Float32Array` de `n` bandas que la escena rellena sola antes de cada cuadro. Pídelos una vez, fuera de `update`. |

Sin sonido, o sin que el usuario lo permita, todo vale 0.

### `input`: el puntero

`input.cursor` tiene `x` e `y` (fracciones de la pantalla, de 0 a 1), `inside` (el puntero está en
la pantalla), `buttons` y `left` (el botón izquierdo está pulsado), `scene` (`[x, y]` en píxeles del
lienzo) y `screen` (`[x, y]` en píxeles de la pantalla). `buttons` y `left` valen 0 sin la entrada
`clicks`. Cuando el puntero sale, vuelve suavemente al centro en unos 0,15 s.

### `scene`: las capas y el escenario

| Función | Qué hace |
|---------|----------|
| `scene.layer(id)` | El manejador de una capa. Es siempre el mismo objeto para el mismo id. |
| `scene.layers()` | Los ids de las capas de arriba del todo, en orden de dibujo. |
| `scene.spawn(plantilla, opciones)` | Añade una copia de una plantilla de `templates` (declara `layers`). |
| `scene.destroy(id)` | Quita una copia que hizo un script. |
| `scene.order(id, índice)` | Mueve una capa de arriba del todo; 0 es el fondo. |

`scene.spawn` pone la copia encima de todo, o en `opciones.at`. Su id es `opciones.id` (letras,
dígitos, `_` y `-`, hasta 64) o `<plantilla>_<n>`. Las demás `opciones` son campos de la capa
(`position`, `opacity`...). Los hijos de un grupo copiado se llaman `<id>.<hijo>`. Pasar de 256
copias, o de 512 capas en total, lanza un error.

#### Los campos de una capa

Se leen y se escriben con el nombre: `capa.opacity = 0.5`.

| Campo | Valor |
|-------|-------|
| `position`, `origin`, `skew` | `[x, y]`; `skew` en grados, hasta ±89 |
| `scale`, `parallax` | `[x, y]` o un solo número |
| `rotation` | grados, hacia la derecha |
| `opacity` | 0 a 1 |
| `visible` | `true` o `false` |
| `color` | `[r, g, b, a]` de 0 a 1, `{ r, g, b, a }` o `"#rrggbb"` |
| `size` | `[ancho, alto]` de imágenes, vídeos, sólidos y espectros; el tamaño en puntos de un texto |
| `text`, `font`, `align` | el texto (hasta 4096 caracteres), la fuente (una de la escena, o `sans-serif`, `serif`, `monospace`) y `left`, `center` o `right` |

Los vectores aceptan listas, `{ x, y }` o los de `wprs/math`. Leer un campo da lo que escribió el
script o, si no escribió nada, lo que la escena calculó en el cuadro anterior. Lo que escribes dura
hasta que lo escribas otra vez o llames a `capa.reset("campo")` (`capa.reset()` los suelta todos).
La escena acota lo que recibe: posiciones y tamaños a 100 000 píxeles, escalas a 1000, `origin` y
`parallax` a ±10, opacidad y colores a 0..1.

Otros miembros de una capa:

| Miembro | Qué es |
|---------|--------|
| `id`, `name`, `type`, `initial` | Su identificador, su tipo y la capa tal como la escribe `scene.json`. |
| `on(evento, función)` | Atiende `click`, `down`, `up`, `enter`, `leave` y `move` sobre capas `solid` o huesos `hit` (más abajo). |
| `effect(índice o nombre)` | Un efecto de la capa (declara `effects`). |
| `video`, `anim(clip)` | Control de vídeos y de clips (declara `animations`). |
| `parent`, `children`, `world`, `setParent(id, mantener)` | La jerarquía (declara `hierarchy`). |
| `bones`, `bone(nombre)`, `shape(nombre, peso)`, `impulse(hueso, [vx, vy])`, `hit(x, y)`, `attachment(nombre)` | Huesos, formas de mezcla y anclajes de una capa `mesh` (declara `animations`). |
| `destroy()` | Quita la capa si es una copia que hizo un script. |

**Efectos.** `capa.effect(0)` o `capa.effect("brillo")` (el nombre se pone en `uses`, `[a-z0-9_]{1,32}`)
da `enabled`, `set(parámetro, valor)`, `get(parámetro)`, `reset()`, `index`, `name` y `effect`.
`set` acepta números, vectores, colores, interruptores, modos de mezcla y listas de emisores (hasta
64 `{ from, to, angle, strength }`), los comprueba contra la descripción del efecto y los recorta a
su rango. Las imágenes de un efecto no se pueden cambiar. Lo que pones dura hasta que lo cambies.

**Eventos del puntero.** `capa.on("click", (e) => ...)` recibe `{ type, layer, local, scene, bone }`:
`local` son píxeles de la capa desde su esquina de arriba a la izquierda, `scene` píxeles del lienzo
y `bone` el hueso tocado (`null` si no hay). `enter`, `leave` y `move` siguen a la capa `solid` de
más arriba bajo el puntero. `down` va a esa capa, que recibe también `move` y `up` hasta que se suelten
todos los botones; `click` sigue a un `up` sobre la capa que recibió el `down`. Llegan en el cuadro
siguiente. `enter`, `leave` y `move` piden la entrada `pointer`; `click`, `down` y `up`, `clicks`. Los
clics están apagados en cada pantalla hasta que el usuario los activa, y llegan solo del
compositor o del escritorio, nunca de otro programa.

**Vídeo.** `capa.video` (declara `animations`) tiene `play()`, `pause()`, `stop()` (vuelve al
`start`), `seek(segundos)`, `time`, `duration`, `playing`, `rate` (0 a 16), `volume` (0 a 1) y
`on("ended", función)`.

**Clips.** `capa.anim(clip)` controla las capas de animación de una capa `mesh` que tocan ese clip:
`play()`, `pause()`, `stop()`, `seek(segundos)`, `rate` (0 a 16), `weight` (0 a 1), `time`, `duration`,
`playing` y `on("ended", función)`.

**Jerarquía.** Con `hierarchy`, `capa.parent` es el id de la capa que sigue (o `null`),
`capa.children` los ids de las que la siguen y `capa.world` una `Mat3` de `wprs/math` de los píxeles
de la capa a los del lienzo. `capa.setParent(id, mantener)` hace que siga a otra (`null` para ninguna);
con `mantener` la capa se queda donde está. Una capa no puede seguirse a sí misma ni seguir a una
que la siga, ni tener más de 8 padres encadenados.

#### El escenario

Con `camera` (y `bloom` para el brillo): `scene.camera` (`center`, `zoom`, `rotation`,
`shake: { enabled, amplitude, roughness, speed }`), `scene.parallax` (`amount`, `delay`, `pointer`,
`offset`), `scene.bloom` (`threshold`, `strength`, `radius`, `tint`), `scene.camera3d` (`fov`, `eye`,
`center`, `up`) y los colores `scene.background`, `scene.ambient` y `scene.sky`. Se leen como los
dejó la escena en `canvas` y lo que escribió el script. Escribir un objeto cambia solo los campos
que nombra, dentro de los rangos de `canvas`; escribir `null` devuelve el campo a la escena. El
temblor de la cámara se queda quieto con la opción de reducir el movimiento y en el nivel Q0.

#### Formas propias

Con `geometry` y un `pools` en `scene.json`: `scene.mesh(pool)` da un conjunto de formas con
`create({ vertices, indices, format, visible })`, `shapes` y `clear()`; cada forma tiene `update`,
`destroy`, `id` y `alive`. `vertices` es un `Float32Array` o una lista de números en píxeles del
conjunto, con `format` `"xy"`, `"xyuv"` (el predeterminado), `"xyrgba"` o `"xyuvrgba"`.

### `sound(id)`: un sonido de la escena

Declara `sounds`. `sound(id)` tiene `play()` (sigue donde lo dejó `pause()`, o empieza), `pause()`,
`stop()` (vuelve al principio), `playing` y `volume` (0 a 1, se multiplica por el de la capa y por el
que fija el usuario). La primera llamada le quita el control a `play_on`. El silencio que pone el usuario sigue
valiendo.

### `timeline(id)`: líneas de tiempo

`play()`, `pause()`, `reverse()`, `stop()`, `seek(t)`, `speed` (de −100 a 100; por debajo de cero
corre hacia atrás), `time`, `duration`, `playing` y `on(nombre, función)` para sus eventos y para
`ended`.

### `particles(capa)`: partículas

`play()`, `pause()` (no nacen más, las que hay siguen), `stop()` (las quita hasta el próximo
`play`), `playing`, `emit(n)` (de 1 a 1000), `point(n, [x, y])`, `from(n, id)` y `wall(n, id)` (el
emisor o el choque `n`, de 0 a 7, toma la imagen de la capa `id`; `null` la devuelve) y los campos
`instance.alpha`, `size`, `count`, `speed`, `life`, `time_scale`, `brightness` y `color`, que duran
hasta que los cambies.

### `storage`: lo que la escena recuerda

Declara `storage`. `storage.get(clave, ámbito)`, `set(clave, valor, ámbito)`, `delete`, `clear` y
`keys` guardan valores JSON. El ámbito es `"package"` (el predeterminado, compartido por las
pantallas) o `"output"` (de cada pantalla). Hay 64 KiB por ámbito y `set` lanza un error si te
pasas o si el valor no es JSON. Se guarda solo en el equipo (nunca se sincroniza ni se exporta), en
un archivo privado que se escribe como mucho cada 5 s y al detenerse la escena, y se borra con el
paquete. Las vistas previas del editor leen lo guardado pero no escriben.

### `folder(propiedad, opciones)`: una carpeta elegida

Declara `folders`. Lista una propiedad `folder` con `mode: all`. `files` son identificadores opacos
de sus imágenes (también de sus vídeos con `opciones.videos: true` si la propiedad acepta
`video`). `name(id)` da el nombre del archivo, `kind(id)` da `"image"` o `"video"`, y
`on("added" | "removed" | "changed", función)` recibe `{ prop, files }` cuando la carpeta cambia.
`show(capa, id)` hace que una capa de imagen muestre esa imagen, o una de vídeo la reproduzca
(`null` la devuelve a la suya); hasta ocho archivos a la vez. Nunca ves rutas.

### `media`: lo que suena en el reproductor

Declara `media` y pide la entrada `media`. `media` tiene `enabled`, `state` (`playing`, `paused` o
`stopped`), `playing`, los textos `title`, `artist`, `album`, `album_artist`, `subtitle`, `genres` y
`kind`, `has_cover`, `colors` (como en `mediaThumbnail`), `position` (sigue corriendo entre lecturas),
`duration`, `rate`, `progress` (0 a 1) y `snapshot()` (una copia congelada de todo). Sin el permiso
dice `enabled: false`, `stopped` y textos vacíos. Nunca trae la carátula ni el nombre del
reproductor.

### `shared`: datos entre módulos

Un objeto que comparten todos los módulos de la escena. Cada valor se guarda como una copia
congelada (números, textos, booleanos, listas, arreglos tipados y objetos de ellos), así que para
cambiarlo se asigna de nuevo: `shared.cuenta = shared.cuenta + 1`. No admite funciones, `Map`, `Set`,
`Date` ni getters.

### Temporizadores y consola

| Función | Qué hace |
|---------|----------|
| `setTimeout(f, ms)`, `setInterval(f, ms)` | Temporizadores en el reloj de la escena (declara `timers`). Devuelven una función que los cancela; `clearTimeout` y `clearInterval` la llaman. Corren al principio de un cuadro, como mucho una vez por cuadro cada uno, sin recuperar los periodos que se perdieron, y esperan mientras la escena está en pausa. |
| `console.log`, `info`, `warn`, `error`, `debug` | Escriben en la consola de los scripts (más abajo). |
| `openShortcut(propiedad)` | Abre lo que el usuario eligió en una propiedad `shortcut`. Devuelve `true` solo dentro de un manejador de `click` y hasta cuatro propiedades por cuadro (declara `shortcuts`). |

### `wprs/math`

`Vec2`, `Vec3` y `Vec4` (`add`, `sub`, `mul`, `div`, `min`, `max`, `clamp`, `scale`, `negate`, `abs`,
`floor`, `ceil`, `round`, `dot`, `length`, `lengthSquared`, `normalize`, `distance`, `mix`, `lerp`,
`reflect`, `project`, `equals`, `isZero`, `toArray` y, según el tamaño, `cross`, `angle`, `rotate` y
`perpendicular`), `Mat3` (2D: `translation`, `rotation`, `scaling`, `transformPoint`,
`transformVector`, `decompose`) y `Mat4` (`translation`, `scaling`, `rotationX`, `rotationY`,
`rotationZ`, `fromEuler`, `perspective`, `lookAt`, `transformPoint`, `transformVector`,
`decompose`), ambas con `multiply`, `transpose`, `determinant`, `invert`, `equals` y `get(fila,
columna)`. Además `clamp`, `mix`, `smoothstep`, `step`, `fract`, `remap`, `degrees`, `radians`,
`angleDelta`, `mixAngle`, `angleVector`, `vectorAngle`, `lookAt`, `rgbToHsv` y `hsvToRgb` (el
matiz va de 0 a 1). Los vectores y colores entran y salen como estas clases o como listas.

## Eventos y entradas

Un script se entera de lo que pasa de cuatro maneras.

**El tiempo.** `update` corre en cada cuadro y `engine.time` y `engine.dt` dicen cuánto pasó. Para
hacer algo más tarde o cada cierto tiempo, usa `setTimeout` y `setInterval`. Con una escena sin
`update`, un temporizador despierta la escena solo cuando le toca.

**El puntero.** Lee `input.cursor` en `update` para seguirlo, o atiende `enter`, `leave`, `move`,
`down`, `up` y `click` de una capa con `capa.on(...)`. Una capa solo recibe esos eventos si es de tipo
`solid` (o una capa `mesh` con huesos `hit`). El puntero y los clics son entradas: el manifiesto las
declara en `inputs` y el usuario las concede por paquete. Los clics, además, el usuario los activa
en cada pantalla. El teclado nunca llega a una escena.

**El sonido.** `audio.bass`, `mid`, `treble`, `level` y `beat` en `update`, o las bandas con
`audio.bands()` y `audio.buffers()`. Es la entrada `audio`; sin ella (o sin sonido) todo vale 0.

**Las propiedades.** `props.get(id)` en cualquier momento, o la función `properties(cambiadas)`, que
solo se llama cuando algo cambió. `props.pressed(id)` avisa de un botón.

Otras fuentes: `timelineEvent` y `timeline(id).on(...)` para las líneas de tiempo, `settings` para
el ritmo de cuadros y la calidad, `resize` para el giro de la pantalla y las funciones `media...`
para el reproductor.

Una escena solo se vuelve a dibujar por una entrada que usa.

## Probar y depurar

### `console`

`console.log`, `info`, `warn`, `error` y `debug` escriben líneas con esta forma:

```
log scripts/semaforo.js: estado: roja
error scripts/semaforo.js: TypeError: opacity takes a number
```

Primero el nivel, luego el módulo y luego el texto. Un módulo sin declaración no puede usar
`console`: declara algo, aunque sea `"events": ["update"]`. Un objeto se escribe como JSON. Pasar de 20 líneas
por segundo descarta las de más, y cada línea se corta a 512 caracteres.

### Dónde se ve

- **En el editor**, en el panel *Problemas*, bajo *Consola de los scripts*, mientras la vista previa
  está encendida. Ahí aparecen también las entradas que el script usa y que la vista previa no tiene
  concedidas.
- **En la terminal**, con la vista previa de la escena en una salida:

  ```
  wprs-ctl scene preview mi-escena -o DP-1 --console
  ```

  Muestra las líneas del script a medida que salen y el error cuando cambia, hasta que lo
  interrumpas con Ctrl-C. Si la escena tiene un error de script, `wprs-ctl state` lo imprime junto a
  la salida.
- **En el registro del servicio** (`wprsd`): lo que escribe `console` sale a nivel `debug`
  (`RUST_LOG=wprsd=debug`) y un módulo que se detiene sale a nivel `warn`.

### Qué pasa cuando falla

| Qué ocurre | Qué se ve |
|------------|-----------|
| Una función lanza un error | `error <módulo>: <error>` en la consola; el cuadro sigue con los demás módulos. |
| Un módulo falla tres cuadros seguidos | Se detiene con sus temporizadores. La pantalla dice «Un script de esta escena se detuvo y la escena sigue sin él: `<módulo>`: `<error>`». |
| Un módulo no carga (error de sintaxis, un `import` que no existe) | Se detiene igual, sin llegar a ejecutar nada. |
| Un cuadro se pasa de tiempo o de memoria | Falla ese cuadro entero. Tres seguidos apagan los scripts: «Los scripts de esta escena fallaron tres veces seguidas y están apagados; la escena sigue sin ellos». |

Los textos de los errores del propio JavaScript y los de la interfaz (`opacity takes a number`)
salen en inglés. Los más comunes:

| Mensaje | Qué pasó |
|---------|----------|
| `opacity takes a number` (y otros `... takes ...`) | Escribiste un valor de otro tipo en un campo de una capa. |
| `<módulo> uses timers without declaring it in scene.json` | Falta `timers` (o `layers`, `effects`...) en `uses`. |
| `<módulo> uses X, which a module has to declare in scene.json` | Un módulo sin declaración usó algo que no está en la interfaz reducida. |
| `Error resolving module './x.js' ... only wprs, wprs/math and the scripts of the scene can be imported` | El `import` apunta a algo que no es `wprs`, `wprs/math` ni un script de `scripts`. |
| `invalid property name at a.js:1:26` (y similares) | Error de sintaxis, con archivo, línea y columna. |
| `the script used more than 4ms of CPU time` | El cuadro tardó demasiado; en la carga el límite es de 100 ms. |
| `out of memory` | El script pasó de 32 MiB. |
| `<ruta>: names <fn>, which the script does not export` | Un valor atado con `fn` nombra una función que el módulo no exporta. |

### Lo que `pack check` no ve

`wprs-ctl scene validate` y `wprs-ctl pack check` comprueban `scene.json`, el manifiesto y las
entradas, pero no ejecutan los scripts: un error de sintaxis o una función que falla solo aparece
cuando la escena corre. Prueba siempre la escena con la vista previa y mira la consola antes de
empaquetar (`paquete.md`).

Para probar un script que reacciona a algo que no tienes a mano:

- `wprs-ctl scene preview mi-escena --media pista.json` inventa una pista del reproductor;
- `wprs-ctl scene preview mi-escena --values ajustes.json` prueba valores de propiedades sin
  guardarlos;
- `wprs-ctl scene preview mi-escena --restart` arranca la escena y los scripts de cero.

## Ejemplos completos

Cada ejemplo es una escena entera, con su `manifest.json`, en `docs/crear/ejemplos/<nombre>/`.
Los seis primeros se ejecutan en las pruebas del repositorio durante unos cientos de cuadros, con
puntero, sonido y propiedades simulados, y sus resultados se comprueban. Para verlos en pantalla:

```
wprs-ctl scene preview docs/crear/ejemplos/reloj -o DP-1 --console
```

(Cambia `DP-1` por el nombre de tu salida, o quítalo para usar todas.)

### 1. Un reloj

*Carpeta `reloj`.* El script más corto: sin declaración, usa la interfaz reducida. Escribe la hora
y la fecha en dos capas de texto, en el idioma de quien usa el fondo.

**`scripts/reloj.js`**

```js
import { scene, time } from "wprs";

export function update() {
    scene.layer("hora").text = time.format("HH:mm");
    scene.layer("fecha").text = time.format("EEEE d 'de' MMMM");
}
```

La declaración en `scene.json` es solo `"scripts": [{ "path": "scripts/reloj.js" }]`.

### 2. Una capa que sigue al puntero

*Carpeta `sigue-puntero`.* Tres círculos persiguen al puntero, cada uno con su prisa. El movimiento
suave no depende del ritmo de cuadros: el paso es `1 - exp(-prisa * dt)`. Declara `uses: ["pointer"]`
y el manifiesto declara `"inputs": ["pointer"]`.

**`scripts/sigue.js`**

```js
import { scene, input, engine } from "wprs";

const seguidores = [
    { capa: scene.layer("cabeza"), prisa: 8 },
    { capa: scene.layer("medio"), prisa: 4 },
    { capa: scene.layer("cola"), prisa: 2 },
];
const [cx, cy] = engine.canvas.map((v) => v / 2);
const lugar = seguidores.map(() => [cx, cy]);

export function update() {
    const [px, py] = input.cursor.scene;
    seguidores.forEach(({ capa, prisa }, i) => {
        const paso = 1 - Math.exp(-prisa * engine.dt);
        lugar[i][0] += (px - lugar[i][0]) * paso;
        lugar[i][1] += (py - lugar[i][1]) * paso;
        capa.position = lugar[i];
    });
}
```

### 3. Una reacción al sonido

*Carpeta `reaccion-al-sonido`.* Un anillo crece con los graves (sube rápido y baja despacio) y la pantalla
destella en cada golpe de ritmo. Declara `uses: ["audio"]` y `"inputs": ["audio"]`.

**`scripts/pulso.js`**

```js
import { scene, audio, engine } from "wprs";

const pulso = scene.layer("pulso");
const destello = scene.layer("destello");
let tamano = 1;
let brillo = 0;

export function update() {
    const meta = 1 + audio.bass * 0.8;
    const rapidez = meta > tamano ? 30 : 4;
    tamano += (meta - tamano) * (1 - Math.exp(-rapidez * engine.dt));
    pulso.scale = tamano;
    pulso.opacity = 0.4 + audio.level * 0.6;

    brillo = audio.beat ? 0.25 : Math.max(0, brillo - engine.dt);
    destello.opacity = brillo;
}
```

### 4. Una propiedad que cambia un grupo de capas

*Carpeta `grupo-por-ajuste`.* Una propiedad `momento` (automático, día o noche) muestra uno de dos
grupos de capas. En automático, mira la hora del día cada minuto. Usa `properties()` para
enterarse del cambio y `setInterval` (declara `timers`).

**`scripts/momento.js`**

```js
import { scene, props, engine, setInterval } from "wprs";

function elegido() {
    const ajuste = props.get("momento");
    if (ajuste !== "auto") return ajuste;
    return engine.daytime >= 0.25 && engine.daytime < 0.75 ? "dia" : "noche";
}

function mostrar() {
    const momento = elegido();
    for (const grupo of ["dia", "noche"]) {
        scene.layer(grupo).visible = grupo === momento;
    }
}

export function properties() {
    mostrar();
}

setInterval(mostrar, 60000);
```

### 5. Una elección al azar al empezar

*Carpeta `azar-al-inicio`.* Al arrancar elige una de tres formas y la gira. Con `"seed": null` en
`clock`, cada arranque da otra.

**`scripts/azar.js`**

```js
import { scene } from "wprs";

const formas = ["circulo", "cuadrado", "anillo"];

export function init() {
    const elegida = formas[Math.floor(Math.random() * formas.length)];
    for (const id of formas) {
        scene.layer(id).visible = id === elegida;
    }
    scene.layer(elegida).rotation = Math.random() * 360;
    console.log("forma elegida:", elegida);
}
```

### 6. Una máquina de estados

*Carpeta `maquina-estados`.* Un semáforo. La tabla `estados` dice cuánto dura cada estado y cuál
sigue; `entrar` cambia de estado y `update` mira si ya pasó el tiempo.

**`scripts/semaforo.js`**

```js
import { scene, engine, console } from "wprs";

const estados = {
    verde: { dura: 4, siguiente: "ambar" },
    ambar: { dura: 1.5, siguiente: "roja" },
    roja: { dura: 4, siguiente: "verde" },
};
let actual = "";
let desde = 0;

function entrar(nombre) {
    actual = nombre;
    desde = engine.time;
    for (const luz of Object.keys(estados)) {
        scene.layer(luz).opacity = luz === nombre ? 1 : 0.12;
    }
    console.log("estado:", nombre);
}

export function init() {
    entrar("roja");
}

export function update() {
    if (engine.time - desde >= estados[actual].dura) {
        entrar(estados[actual].siguiente);
    }
}
```

### 7. Un módulo para muchos valores

*Carpeta `giro-por-valor`.* Un solo módulo hace girar dos capas a velocidades distintas: cada
`rotation` de `scene.json` se ata con `{ "script", "fn", "params" }`.

**`scripts/giro.js`**

```js
import { engine } from "wprs";

export const gira = (grados, { params }) => ((grados ?? 0) + params.velocidad * engine.dt) % 360;
```

**En `scene.json`**

```json
"scripts": [{ "path": "scripts/giro.js", "events": ["update"],
              "params": { "velocidad": { "type": "number", "default": 30, "min": -720, "max": 720 } } }]
...
"transform": { "rotation": { "script": "scripts/giro.js", "fn": "gira" } }
"transform": { "rotation": { "script": "scripts/giro.js", "fn": "gira", "params": { "velocidad": -150 } } }
```

## Para seguir

- `paquete.md`: cómo empaquetar, firmar y publicar la escena con sus scripts.
- `cli.md`: los comandos que usa esta página.
- `propiedades.md`: las propiedades que lee `props`.
- `acciones.md`: lo que puede hacer el editor sobre una escena, también por línea de comandos.
