# Fondos web

Un fondo web es una página que wallpaper-rs muestra detrás de las ventanas. Todo lo que usa va
dentro del paquete: no hay servidor, la red está cortada salvo que la declares y el usuario la
permita, y la página no puede abrir ventanas, diálogos ni descargas. Esta guía explica cómo hacerla;
`docs/specs/package.md` («Web pages») describe el manifiesto y `docs/specs/web.md` el motor y el
aislamiento. `docs/crear/ejemplos/reloj-web` es un fondo web completo y mínimo; en el árbol del código fuente
`scenes/lights` es uno más grande, con luces RGB (no viene con el programa instalado).

## Lo mínimo

```
mi-fondo/
  manifest.json
  index.html
  app.js
  assets/
    preview.png
```

```json
{
  "schema": "wallpaper-rs.wpak/2",
  "id": "org.example.reloj",
  "version": "1.0.0",
  "title": { "en": "Clock", "es": "Reloj" },
  "default_locale": "en",
  "license": "CC0-1.0",
  "origin": { "kind": "original" },
  "content": { "rating": "general" },
  "photosensitive": false,
  "kind": "web",
  "entry": "index.html",
  "previews": { "thumbnail": "assets/preview.png" },
  "inputs": [],
  "web": { "background": "#000000" },
  "properties": {
    "color": { "type": "color", "default": "#ffffff", "label": { "en": "Color", "es": "Color" } }
  }
}
```

`wprs-ctl scene new mi-fondo --from web` (o *Página web* en la página *Escenas* de la app) crea
esta carpeta con una página de ejemplo (`index.html`, `app.js` y un manifiesto sin `previews`) que
puedes editar en la app o con cualquier editor de texto. La miniatura no la crea: `pack check` pasa
sin ella, pero añádela con una imagen tuya, que `scene thumbnail` copia a `assets/preview.png`, la
reduce si hace falta y escribe en `previews.thumbnail`. El límite es png, jpg o webp de hasta 1024
píxeles de lado y 1 MiB; con una imagen de 1920 píxeles escrita a mano en el manifiesto, `pack check`
falla con «previews.thumbnail no válido». Después:

```
wprs-ctl scene thumbnail mi-fondo captura.png
wprs-ctl pack check mi-fondo
wprs-ctl pack build mi-fondo -o reloj.wpak
wprs-ctl lib import reloj.wpak --wait
```

`pack check` explica lo que falta, `pack build` calcula `requires.features` (aquí, `web`) y arma el
paquete, y `lib import` lo deja en la biblioteca. Las rutas son relativas: `<script src="app.js">`,
`fetch("datos.json")`. El resto del camino (firmar, publicar) es el de cualquier paquete:
`paquete.md`.

Un fondo web es un paquete `kind: web` cuya `entry` es una página `.html` o `.htm`. Puede llevar
imágenes (también `svg`, `hdr` y `exr`), vídeos, `html`, `js`, `mjs`, `css`, `json`, `wasm`,
sonidos, fuentes, modelos (`glb`, `gltf`, `bin`, `obj`, `mtl`, `fbx`), `basis`, shaders (`glsl`,
`vert`, `frag`, `wgsl`), `csv`, `xml` y `txt`. Nunca programas ni bibliotecas; un `.wasm` tiene que
empezar con su firma `\0asm`.

## Dónde se ve

Las páginas las dibuja un programa auxiliar, `wprs-web`, con WPE WebKit 2.54 o más nuevo. Donde ese
motor no está, el demonio rechaza los fondos web con el motivo en lugar de dejar la pantalla en
blanco, y lo dice en `wprs-ctl lib apply`, en la app y en el error de la pantalla. En el teléfono se
usa el WebView del sistema.

Un fondo web que se queda sin responder se reinicia solo:

- el programa auxiliar pregunta a la página cada 5 s; si no contesta en 10 s (un script atascado en
  un bucle), la detiene y la carga de nuevo;
- una página que se cae o se queda sin memoria se carga otra vez a 1, 4 y 9 s; a la cuarta seguida
  se apaga, la pantalla muestra el color de `web.background` y su error dice por qué;
- toma como mucho un procesador y 1,5 GB entre todos sus procesos, y 768 MB cada proceso de página.

## La interfaz `window.wprs`

Está lista antes de tus scripts:

```js
wprs.onProperties((cambiadas) => {
  if (cambiadas.color) document.body.style.color = cambiadas.color.hex;
});
wprs.onPause((pausada) => pausada ? detener() : seguir());
wprs.onSettings(({ fps, locale, volume }) => ajustar(fps, locale, volume));
```

| Miembro | Qué da |
|---------|--------|
| `properties`, `onProperties(f)` | Todas las propiedades cuando registras `f`, y después solo las que cambian. Un color llega como `{ hex, r, g, b, a }` (de 0 a 1), un combo como `{ value, label }`, un archivo como una dirección `wpak-user://` y una carpeta como `wpak-user://<id>/`. |
| `fps`, `quality`, `locale`, `volume`, `onSettings(f)` | El ritmo de cuadros al que se dibuja la página, el nivel de calidad (0 a 3), el idioma elegido entre los del paquete, siempre como una etiqueta de idioma válida (`es-AR`, `en-US`) que se puede pasar a `Intl`, y el volumen del fondo (0 a 1) para el sonido de la página. `f` recibe `{ fps, quality, locale, volume }` al registrarla y cuando cambia algo. |
| `paused`, `onPause(f)` | `true` mientras la página está oculta o en pausa. |
| `onAudio(f)` | 64 bandas por canal, `{ left, right }`, de graves a agudos, unas 30 veces por segundo. Hace falta `audio` en `inputs` y que el usuario lo permita; sin él, llega silencio. |
| puntero y clics | Eventos normales del navegador: `pointermove` si declaras `pointer`; `click`, solo de clics reales y en las pantallas donde el usuario los activa, si declaras `clicks`. El teclado nunca llega. |
| `files(propiedad)`, `randomFile(propiedad)`, `onFiles(propiedad, f)` | Las direcciones de los archivos de una carpeta elegida (más abajo). |
| `onAction(f)` | El id de una propiedad `button` que se pulsó. |
| `openShortcut(propiedad)` | Abre lo que el usuario eligió en una propiedad `shortcut`. Solo funciona dentro de un clic real y como mucho una vez por segundo. |
| `onMedia(f)` | Lo que suena en el reproductor (`{ enabled, state, title, artist, album, cover, colors, position, duration, rate }`), con la carátula como dirección `blob:` y sus colores. Hace falta `media` en `inputs`. Sin el permiso, `{ enabled: false }`. |
| `rgb.setImage(bytes, ancho, alto)` | Envía una imagen RGBA a la iluminación RGB del equipo (se reduce a 64 × 16, hasta 30 veces por segundo). Hace falta `permissions.rgb` y que el usuario lo permita; devuelve `true` solo cuando los colores salen. |

- `requestAnimationFrame` ya va al ritmo que elige el usuario (y más lento con batería): no hace
  falta limitarlo. El ritmo de cuadros nunca pasa de `playback.max_fps`. Una página que no cambia no
  gasta nada: dibuja solo cuando haga falta.
- Con la página oculta, `document.visibilityState` es `"hidden"`, `onPause(true)` avisa y los
  fotogramas se detienen; tras 5 minutos oculta se cierra y vuelve a cargarse al mostrarse.
- `localStorage` e IndexedDB guardan entre arranques si declaras `"web": { "storage_kib": 64 }`
  (de 0 a 5120) y el usuario no lo oculta; sin eso, nada sobrevive. Lo guardado se queda en el
  equipo, nunca se sincroniza y se borra con el paquete o con *Borrar datos*. Si pasa del
  doble de `storage_kib` más 1 MiB, la página arranca sin guardar nada y su error lo dice.
- `wprs.volume` es el volumen del fondo para el sonido de tu página: puede sonar sin que nadie
  toque nada, `<audio>`, `<video>` y `AudioContext` suenan a ese volumen (por el que les pongas) y
  a 0 está silenciada. Los audios y vídeos del paquete o de la carpeta elegida se reproducen desde
  una copia en memoria, de hasta 256 MiB cada uno.
- `web.preload` es un `.js` o `.mjs` del paquete que corre antes de los scripts de la página, y
  `web.background` (`#rrggbb`) es el color que se ve mientras carga.

## Probar y depurar

`console.log` y los errores de la página salen en tres sitios:

- en el editor, en el panel *Problemas*, bajo *Consola de los scripts*, mientras la vista previa
  está encendida (guarda las últimas 50 líneas);
- en la terminal: `wprs-ctl scene preview mi-fondo -o DP-1 --console` las imprime a medida que salen,
  en tus pantallas o en las de prueba, mientras algo mira esa salida y hasta que lo interrumpas;
- en el registro del servicio, a nivel `debug` (`RUST_LOG=wprsd=debug`), hasta 200 líneas por
  ejecución.

`wprs-ctl scene preview mi-fondo --values ajustes.json` prueba valores de propiedades sin guardarlos
y `--media pista.json` inventa una pista del reproductor. La salida de `wprs-ctl state` dice si la
protección del motor está completa (`sandbox`: `on`) o reducida (`reduced`: algún contenedor no
permite los espacios de nombres que usa).

## Carpetas elegidas

Una propiedad `folder` deja que el usuario elija una carpeta de fotos o vídeos. La página nunca ve
rutas, sino direcciones `wpak-user://` que solo sirven dentro de ella:

```js
const una = await wprs.randomFile("fotos");
imagen.src = una;
```

Con `"mode": "all"` también tienes la lista entera y los cambios:

```js
const todas = await wprs.files("fotos");
wprs.onFiles("fotos", ({ added, removed, changed }) => actualizar(added, removed, changed));
```

`changed` trae pares `[antes, ahora]` de los archivos que se reescribieron con el mismo nombre: la
dirección nueva carga el contenido nuevo. El demonio lista la carpeta (`max_files`, `recursive`; sin
archivos ocultos ni enlaces) y la vuelve a mirar cada 5 s mientras la página se ve.

## Lo que ve tu página

Por defecto ve la zona horaria (`new Date()` da la hora local), el idioma (`navigator.language`),
lo que puede hacer la tarjeta gráfica a través de WebGL (en el escritorio, sus límites y extensiones,
no su modelo; en el teléfono, también su modelo) y su propio almacenamiento. El usuario puede
ocultar cada cosa: entonces la página ve UTC, `en-US`, un nombre de tarjeta genérico (WebGL sigue
funcionando; `navigator.gpu` desaparece) y nada guardado. Haz que funcione en todos esos casos.
Nunca ve rutas, nombres de dispositivos, la batería ni la lista de fuentes.

## Qué le prohíbe el aislamiento

Cada capa se aplica a toda página:

1. Solo lee su propio paquete, por direcciones `wpak://<id del paquete>/<ruta>`, y nunca
   `manifest.json` ni su firma. Los archivos que elige el usuario llegan solo por
   `wpak-user://`.
2. Una política de contenido que la página no puede quitar limita las cargas a `wpak:`,
   `wpak-user:`, `data:` y `blob:` (más los orígenes declarados cuando se permite la red); no hay
   objetos, formularios ni `<base>`.
3. Todo lo demás se bloquea, `fetch` y WebSockets incluidos.
4. No hay navegación fuera del paquete, ventanas nuevas, descargas, diálogos, selector de
   archivos, notificaciones ni pantalla completa. Se niega todo permiso (cámara, micrófono,
   ubicación, portapapeles…). WebRTC y la captura de medios están apagados.
5. Sin red declarada y permitida, el programa auxiliar arranca en un espacio de red vacío: ni
   siquiera el proceso de red del motor llega a nada.
6. La página corre en su propio proceso, dentro de un entorno aislado.

## Red

Si de verdad la necesitas, declara los orígenes (hasta 8, `https://` sin ruta, en minúsculas) y
`network` en `inputs`:

```json
"inputs": ["network"],
"permissions": { "network": { "allow": ["https://api.example.org"] } }
```

La página solo llega a esos orígenes y solo después de que el usuario lo permita (`wprs-ctl input
grant <id> network`, o desde la app); mientras tanto `fetch` falla. Una página que lista orígenes
declara `network`, y solo entonces. Evítala si puedes: un fondo sin red funciona en todas partes y
no expone nada.

## En el teléfono

La app de Android muestra el mismo paquete con el WebView del sistema (el motor de Chrome que
Android mantiene al día), con la misma interfaz `window.wprs`. Conviene tenerlo en cuenta:

- La página vive en una dirección `https://…wpak.invalid/` y los archivos elegidos llegan como
  `https://…/:user/<propiedad>/…` en lugar de `wpak-user://`: trátalos como direcciones opacas y usa
  rutas relativas para tus archivos.
- `requestAnimationFrame` va al ritmo que elige el usuario, pero las animaciones CSS y los vídeos van
  al de la pantalla: para ahorrar batería anima con `requestAnimationFrame`.
- El dedo es el puntero (`pointermove`, como un ratón que pasa por encima) si el usuario activa
  *Reaccionar al tacto*, y un toque sin moverse es un `click` si activa *Reaccionar a los toques*.
  Los toques llegan después del primer dibujo de la página. La pantalla de bloqueo no da ninguno.
- El sonido de la página suena solo si el usuario activa el sonido de los vídeos; si no,
  `wprs.volume` es 0.
- `RTCPeerConnection` no existe (se bloquea en todas las páginas) y `console` va a logcat
  (`adb logcat -s wprs`, hasta 200 líneas por carga).
- WebGL depende del teléfono: comprueba que `getContext("webgl")` no devuelve `null` y ten un dibujo
  más sencillo para ese caso.
- El WebView se pausa con el ahorro de energía, con batería baja o con mucho calor: `onPause(true)`
  avisa.

## Pasar una aplicación a fondo web

wallpaper-rs nunca ejecuta programas de un paquete. Un fondo hecho como aplicación (un ejecutable
que dibuja detrás de los iconos) se pasa así:

1. Exporta el proyecto a web desde su motor: los motores de juegos y las bibliotecas 3D más usadas
   generan HTML con WebGL o WebAssembly (`.wasm`, que debe empezar por su firma `\0asm`).
2. Pon el resultado en la carpeta del paquete con `index.html` como `entry` y cambia las rutas
   absolutas por relativas.
3. Sustituye lo que leía del sistema por la interfaz: sus opciones por `properties` y
   `onProperties`, su análisis de audio por `onAudio`, su ratón por los eventos del puntero.
4. Quita lo que no se puede: archivos del equipo (usa una propiedad `folder` o `file`), red sin
   declarar, ventanas, teclado y procesos.
5. Comprueba que dibuja poco: limita el trabajo por fotograma, pausa en `onPause` y no hagas
   bucles con `setInterval` rápidos, que se notan en la batería.

Los ejecutables y las bibliotecas (exe, dll, so, sh, bat, AppImage…) se rechazan al armar el
paquete.
