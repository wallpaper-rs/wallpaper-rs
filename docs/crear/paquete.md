# De la carpeta de la escena a un paquete que otros instalan

Una escena es una carpeta con `scene.json`, un `manifest.json` y sus archivos. Para que otras
personas la instalen, la conviertes en un **paquete**: un archivo `.wpak`, que es un ZIP con el
manifiesto, la escena y lo que usa. Esta página cubre todo el camino, con sus comandos
(`cli.md` los recoge todos):

1. Escribir el manifiesto y poner una vista previa.
2. Revisar el paquete.
3. Firmarlo y empaquetarlo.
4. Probar la instalación.
5. Publicarlo en un repositorio.

Los mismos pasos valen para fondos web (`fondos-web.md`), imágenes y vídeos; lo que cambia es el
tipo (`kind`) y el archivo de entrada (`entry`).

| `kind` | `entry` | Qué lleva |
|--------|---------|-----------|
| `scene` | `scene.json` | Imágenes, vídeos, sonidos, fuentes, scripts, modelos, efectos propios |
| `web` | una página `.html` | Su página con sus archivos |
| `image` | una imagen | La imagen |
| `video` | un vídeo `.webm`, `.mp4` o `.mkv` | El vídeo |
| `preset` | `preset.json` | Los valores de otro paquete |
| `assets` | un documento de escena | Partes de escenas para otros autores |

Un paquete nunca lleva programas ni bibliotecas (`.exe`, `.dll`, `.so`, `.sh`, `.bat`, `AppImage`…):
se rechazan por la extensión. Tampoco enlaces simbólicos ni nombres de archivo con `..`, `\` o `:`.
Como mucho tiene 4096 archivos, 1 GiB por archivo y 2 GiB en total.

## El manifiesto

`manifest.json` está en la raíz del paquete. Crear la escena con `wprs-ctl scene new` (o desde la
página *Escenas* de la app) ya escribe uno válido:

```
wprs-ctl scene new mi-escena --from clock --title "Mi reloj" --locale es
```

Deja en la carpeta `manifest.json`, `scene.json` y `assets/`. Puedes editarlo con
cualquier editor de texto; el editor de la app tiene un diálogo para cada campo (*Título, licencia,
origen y más…*).

### Todos los campos

| Campo | Tipo | Obligatorio | Qué es |
|-------|------|-------------|--------|
| `schema` | texto | sí | `wallpaper-rs.wpak/2`. La versión `/1` sigue funcionando, pero no admite lo marcado abajo como «solo `/2`». |
| `id` | texto | sí | Un nombre único en minúsculas, con puntos, como `org.example.reloj`. Cada parte son letras minúsculas, cifras, `_` o `-`. No lo cambies entre versiones. |
| `version` | texto | sí | Una versión semántica: `1.0.0`, `1.2.3`. Sube la versión en cada publicación. |
| `kind` | texto | sí | `scene`, `web`, `image`, `video`, `preset` o `assets`. |
| `entry` | texto | sí | El archivo con que empieza: `scene.json`, `index.html`… |
| `title` | texto o mapa | sí | De 1 a 128 caracteres en una línea. En `/2` puede ser `{ "en": "Dusk", "es": "Atardecer" }` o `"@clave"`. |
| `description` | texto, mapa o `"@clave"` | no | Hasta 8000 caracteres, con el formato de abajo. |
| `changes` | texto | no | Las notas de esta versión, hasta 2000 caracteres, con el mismo formato. Solo `/2`. |
| `default_locale` | texto | sí, con textos en varios idiomas | El idioma de respaldo, como `en` o `es`. Cada texto tiene que tener valor en él. Solo `/2`. |
| `strings` | objeto | no | Los textos por idioma y clave: `{ "es": { "velocidad": "Velocidad" } }`. Se usan con `"@velocidad"`. Hasta 64 idiomas y 2048 claves cada uno. Solo `/2`. |
| `authors` | lista de textos | no | Hasta 64 nombres de 1 a 128 caracteres (sin `<` ni `>` en `/2`). |
| `license` | texto | sí | Una expresión SPDX: `CC0-1.0`, `CC-BY-4.0`, `MIT`, `CC-BY-SA-4.0 OR MIT`. Ver «Licencias». |
| `tags` | lista de textos | no | Hasta 32 etiquetas de hasta 32 caracteres. Libres; para géneros se recomiendan `abstract`, `anime`, `animals`, `cars`, `cityscape`, `fantasy`, `games`, `minimal`, `mountains`, `music`, `nature`, `pixel-art`, `retro`, `sci-fi`, `space`, `water`, `winter`. |
| `platforms` | lista | no | `desktop`, `android` o ambos: para qué dispositivos es. Solo estrecha: nunca hace que un paquete corra donde no puede. Solo `/2`. |
| `content` | objeto | sí | `{ "rating": "general" }` o `{ "rating": "mature", "descriptors": ["violence"] }`. Ver «Clasificación». Solo `/2`. |
| `photosensitive` | `true` o `false` | sí | `true` si el paquete destella o muestra patrones rápidos que pueden provocar convulsiones. Solo `/2`. |
| `origin` | objeto | sí | De dónde viene la obra. Ver «Origen». Solo `/2`. |
| `credits` | lista | no | Hasta 64 `{ paths, license, attribution, url?, source? }`: quién hizo qué archivos y con qué licencia. Una ruta que termina en `/` es una carpeta. Solo `/2`. |
| `links` | objeto | no | `{ homepage, source, support, donate }`, cada uno opcional, con `https://` y hasta 512 caracteres. Solo `/2`. |
| `previews` | objeto | no | `{ thumbnail, animated }`. Ver «Vistas previas». Solo `/2` (en `/1` es `preview`). |
| `requires` | objeto | no | `{ engine, features, packages, gpu }`: lo que el paquete necesita. `pack build` rellena `features` solo. `packages` lista hasta 16 `{ id, version }` de otros paquetes que se instalan con él. `gpu` es `any` o `compute`. Solo `/2`. |
| `inputs` | lista | no, pero la escena puede exigirla | `audio`, `pointer`, `clicks`, `media` o `network`: lo que el paquete pide al usuario. Declararlo no lo concede: el usuario lo permite por paquete. Ver «Entradas». Solo `/2`. |
| `permissions` | objeto | no | `{ network: { allow: [...] }, rgb }`: los orígenes de red de una página web y el permiso para enviar colores a la iluminación RGB. Solo `/2`. |
| `web` | objeto | solo en `kind: web` | `{ preload, background, storage_kib }`. Ver `fondos-web.md`. |
| `playback` | objeto | no | Cómo se reproduce por defecto: `speed`, `start`, `end`, `loop`, `max_fps`, `volume`, `muted`. Para vídeos, páginas y escenas. |
| `display` | objeto | no | Cómo se coloca una imagen o un vídeo (`fit`, `background`, `span`, `flip`, `zoom`, `position`) y el color de acento que sugiere (`accent`). |
| `properties` | objeto | no | Las propiedades que el usuario cambia (`propiedades.md`). |
| `presets` | lista | no | Valores ya elegidos de las propiedades, con nombre (`propiedades.md`). |

Un manifiesto completo:

```json
{
  "schema": "wallpaper-rs.wpak/2",
  "id": "org.example.atardecer",
  "version": "1.3.0",
  "kind": "scene",
  "entry": "scene.json",
  "default_locale": "en",
  "title": { "en": "Dusk", "es": "Atardecer" },
  "description": { "en": "Hills at **dusk**.", "es": "Colinas al **atardecer**." },
  "authors": ["Ejemplo"],
  "license": "CC-BY-4.0",
  "tags": ["nature", "minimal"],
  "origin": { "kind": "original" },
  "content": { "rating": "general" },
  "photosensitive": false,
  "previews": { "thumbnail": "assets/preview.png" },
  "links": { "source": "https://example.org/atardecer" },
  "inputs": ["pointer"]
}
```

Los textos (`description`, `changes`) admiten un formato mínimo y seguro: párrafos separados por
una línea vacía, listas con `- ` o `* `, `**negrita**`, `*cursiva*` y enlaces `[texto](https://…)`.
No se admite HTML, imágenes ni enlaces que no sean `https://`. Los demás textos (títulos, etiquetas,
opciones) son texto plano.

### Idiomas

Un texto puede ser plano, un mapa de idioma a texto (`{ "en": "…", "es": "…" }`) o una clave
`"@clave"` de `strings`. Los idiomas son etiquetas como `en`, `es` o `pt-BR`. El programa elige el
primero que coincida con los idiomas que el usuario tiene configurados, luego `default_locale`, luego `en`. Las etiquetas
comunes (`color`, `velocidad`, `volumen`, `calidad`…) ya las traduce el programa: se escriben
`"@wprs.color"` y no necesitan `strings`.

### Licencias

`license` es una expresión SPDX: uno o varios identificadores unidos con `AND`, `OR` o `WITH`
(`CC-BY-4.0 OR MIT`). El programa solo exige que esté bien escrita (hasta 256 caracteres). Lo que
sí hace con ella es decidir si alguien puede **editar una copia** de tu paquete.

| Licencia | Qué permite |
|----------|-------------|
| `CC0-1.0` | Todo, sin condiciones: usar, cambiar y redistribuir. |
| `CC-BY-4.0` | Usar, cambiar y redistribuir **con crédito** a su autor. |
| `CC-BY-SA-4.0` | Lo mismo, y lo derivado se comparte con la misma licencia. |
| `CC-BY-NC-4.0` | Con crédito y **sin uso comercial**. |
| `CC-BY-ND-4.0`, `CC-BY-NC-ND-4.0` | Redistribuir sin cambios. **No permiten obras derivadas.** |
| `MIT`, `Apache-2.0` | Para código (scripts, shaders): usar y cambiar con aviso de la licencia. |
| `GPL-3.0-or-later`, `AGPL-3.0-or-later`, `LGPL-3.0-or-later` | Con copyleft: lo derivado se publica con la misma licencia. |
| `LicenseRef-<nombre>` | Condiciones propias, que describes tú en el paquete. |

Lo que el programa hace con cada una: al abrir una copia para editarla, calcula si la licencia
permite obras derivadas y qué condiciones lleva (crédito, compartir igual, sin uso comercial,
copyleft, condiciones propias). El texto legal de la licencia es el que manda; el programa solo
avisa.

**Licencias que no permiten obras derivadas.** Una licencia no las permite si todas sus
alternativas llevan `ND` (por ejemplo `CC-BY-ND-4.0`) o si está vacía. Editar una copia de un
paquete así se rechaza y no se escribe nada, salvo que confirmes que el paquete es tuyo o que su
autor lo permite:

- en la app, *Editar una copia…* muestra la licencia y una casilla: *Hice este paquete o su autor
  permite una copia*. Sin marcarla, no sigue;
- en la terminal, `wprs-ctl scene open CARPETA --from paquete.wpak --mine`. Sin `--mine` falla con
  «<licencia> does not allow derived works».

Un servidor propio solo acepta las licencias de su lista (por defecto `CC0-1.0`, `CC-BY-4.0` y
`CC-BY-SA-4.0`; quien lo administra puede cambiarla). Un repositorio estático acepta cualquiera.

### Clasificación

`content.rating` es `general` o `mature` (para adultos). La eliges tú; el programa nunca la deduce.
`mature` obliga a nombrar al menos un motivo, sin repetir, en `descriptors`: `sexual`, `violence`,
`gore` o `language`. Las aplicaciones difuminan las vistas previas de un paquete `mature` hasta que
el usuario decide mostrarlas, y pueden ocultarlo de las búsquedas. Un servidor propio rechaza los
paquetes `mature`. Un paquete de la primera versión no tiene clasificación y se muestra como «sin
clasificar».

### Fotosensibilidad

`photosensitive` es `true` cuando el paquete destella o muestra patrones rápidos que pueden
provocar convulsiones. Se marca a mano. Como ayuda, el editor tiene el botón *Medir los destellos*, y `pack check`
mide la escena si no la marcaste: dibuja unos segundos y avisa si hay destellos de más de tres
veces por segundo sobre una décima parte de la imagen («la escena destelló más de N veces por
segundo cerca del segundo S de un dibujo; si alguien puede verlo, marca photosensitive»). Es una
pista, no una prueba: el sonido, el puntero y el reloj deciden la imagen real. Con `true`, la app,
`wprs-ctl` y la app de Android avisan antes de aplicarlo, y el demonio no lo aplica hasta que el
usuario lo confirma; las listas de reproducción lo saltan.

### Origen

| `origin.kind` | Cuándo | Qué más lleva |
|---------------|--------|---------------|
| `original` | Una obra hecha para wallpaper-rs. | Nada. |
| `derived` | La rehiciste a mano a partir de otras obras. | `derived_from`: de 1 a 64 `{ author, title?, url?, license? }` (sin `<` ni `>`). |
| `imported` | Una conversión de otro formato para tu uso personal. | Nada. **Nunca se publica**: `repo add` y el servidor la rechazan. |
| `provider` | Una imagen o un vídeo bajado de una fuente en línea. | `provider: { name, item_url, credit, terms_url? }`. Solo se publica con `CC0-1.0`, `CC-PDM-1.0`, `CC-PDDC`, `CC-BY-2.0`, `CC-BY-3.0` o `CC-BY-4.0`. |

Cualquiera puede añadir `source_url` (la página `https://` de la obra original). Las
aplicaciones muestran el origen, el crédito del proveedor y los paquetes de los que vienen los
archivos con créditos.

### Entradas

`inputs` dice qué le pide el paquete al usuario: el sonido del equipo (`audio`), el puntero
(`pointer`), los clics (`clicks`), lo que suena en el reproductor (`media`) y la red (`network`, solo
para páginas web). Para las escenas, la revisión calcula qué entradas usa la escena con cualquier
valor de sus propiedades y **rechaza** un manifiesto que no las declare, diciendo cuál falta y dónde
la usa: `la escena usa pointer en /scripts/0 de scene.json; añade "pointer"`. Si declaras una
entrada que nada usa, avisa. Un paquete solo recibe las entradas que declara. El teclado nunca llega
a un paquete.

## Vistas previas

`previews.thumbnail` es la imagen que se ve en las bibliotecas. `previews.animated` es opcional.

| Campo | Formato | Límites |
|-------|---------|---------|
| `thumbnail` | png, jpg o webp | hasta 1024 píxeles de lado y 1 MiB; el archivo debe ser lo que dice su extensión |
| `animated` | gif o webp | hasta 640 píxeles de lado, 15 s y 8 MiB |

Tres formas de ponerlas, todas dentro de la carpeta:

```
wprs-ctl scene thumbnail mi-escena captura.png
wprs-ctl scene thumbnail mi-escena --capture --time 3
wprs-ctl scene thumbnail mi-escena --capture --animated --seconds 4 --fps 10
```

La primera copia una imagen que ya tienes a `assets/preview.png` (la reduce si pasa de 1024
píxeles o 1 MiB) y escribe `previews.thumbnail`. Las otras dos dibujan la escena con la tarjeta
gráfica del equipo y piden que el demonio esté en marcha. `--time` es el segundo de la escena que se
guarda (sin él, el segundo de su fotograma fijo, `clock.poster_time`).

**Una captura no ejecuta los scripts.** `scene thumbnail --capture` y `scene render` dibujan la
escena como la deja `scene.json`: un texto que un script rellena sale con su valor inicial. Pon en
`scene.json` valores de partida que se vean bien, o usa una imagen hecha aparte.

`wprs-ctl scene render mi-escena -o vista.png --size 640` dibuja una imagen sin escritorio, sin
demonio y sin guardar nada en la carpeta; sirve para ver el resultado, para hacer clips (`--frames`)
y para comparar niveles de calidad (`--against q1`).

## Revisar el paquete

Hay tres comandos de revisión. Ninguno necesita el demonio.

| Comando | Para qué |
|---------|----------|
| `wprs-ctl scene validate CARPETA` | Lista **todos** los problemas de la escena y del manifiesto, cada uno con el JSON Pointer del campo (`/manifest/inputs`, `/scene/layers/3`). Sale con 1 si hay errores. `--json` da un objeto por problema. |
| `wprs-ctl pack check CARPETA_O_ARCHIVO` | Resume qué necesita, usa y declara el paquete, y si el escritorio y el teléfono lo reproducen. Se detiene en el primer error. |
| `wprs-ctl pack validate CARPETA_O_ARCHIVO --key autor.pub` | Comprueba que el paquete es válido y, con `--key`, que lo firmó esa clave. |

Un paquete sano:

```
$ wprs-ctl pack check docs/crear/ejemplos/sigue-puntero
ok scene org.example.sigue_puntero 1.0.0 (wallpaper-rs.wpak/2)
title = Sigue al puntero
rating = general
photosensitive = no
origin = original
inputs = pointer
needs = inputs.pointer (follows the pointer), newer_keys (writes newer settings with their default values), scripts (runs scene scripts)
build adds = inputs.pointer (follows the pointer), newer_keys (writes newer settings with their default values), scripts (runs scene scripts)
facets = 1080p, android, animated, desktop, landscape, localized, pointer, scripts
desktop = plays
android = plays
languages = en, es
```

| Línea | Qué dice |
|-------|----------|
| `ok scene …` | El tipo, el id, la versión y el esquema. Si hay un error, en lugar de esto sale `Error: …`. |
| `title`, `rating`, `photosensitive`, `origin` | Lo que declara el manifiesto. |
| `inputs` | Las entradas que declara. |
| `needs` | Las funciones del motor que usa el paquete. Una versión de wallpaper-rs que no las tenga rechaza el paquete diciendo que necesita una más nueva. |
| `build adds` | Las de `needs` que faltan en `requires.features` del manifiesto. `pack build` las escribe solo. |
| `facets` | Las etiquetas que el programa calcula del contenido (entradas, tamaño, orientación, `scripts`, `clock`, `customizable`…). No se pueden falsear: salen del paquete, no de lo que digas. Sirven para buscar (`facet:audio`). |
| `desktop`, `android` | `plays`, `plays without …` (lo reproduce sin alguna función) o `refuses …` (lo rechaza, con el motivo). |
| `languages` | Los idiomas de los textos. |

La revisión de una carpeta distingue **errores** (impiden construir el paquete) de **avisos**. Los
más comunes:

| Mensaje | Qué significa y cómo se arregla |
|---------|---------------------------------|
| `inputs: la escena usa pointer en /scripts/0 de scene.json; añade "pointer"` (error) | La escena usa una entrada que el manifiesto no declara. Añádela a `inputs`. |
| `inputs declara audio, pero nada del paquete lo usa` (aviso) | Declaras una entrada que nada usa. Quítala: si no, el usuario ve un permiso que no hace falta. |
| `requires.features: añade "scripts" …; pack build rellena esta lista` (aviso) | Falta una función en `requires.features`. No hace falta arreglarlo a mano: `pack build` y `pack sign` la rellenan. |
| `falta el archivo «falta.png»` (error) | El manifiesto o la escena nombra un archivo que no está en la carpeta. Ponlo, o quita la referencia. |
| `previews.thumbnail: «gran.png» tiene 3000000 bytes, más de los 1048576 permitidos` (error) | La miniatura pasa de 1 MiB o de 1024 píxeles de lado. Redúcela con `scene thumbnail`. |
| `content: es obligatorio: una clasificación general o para adultos y sus motivos` (error) | Falta `content`. Ver «Clasificación». |
| `photosensitive: es obligatorio: true si el paquete destella` (error) | Falta decir si el paquete destella. |
| `origin: es obligatorio: original, o derivado con las obras de las que procede` / `origin.derived_from: derived packages credit 1 to 64 works…` (error) | Falta `origin`, o es `derived` sin las obras de las que viene. |
| `version: se esperaba una versión semántica` (error) | Escribe `1.0.0`, no `uno`. |
| `id: se esperaba un nombre DNS inverso en minúsculas como org.example.name` (error) | El id tiene mayúsculas o no tiene puntos. |
| `license: se esperaba una expresión de licencia SPDX …` (error) | La licencia está vacía o mal escrita. |
| `default_locale: es obligatorio con cadenas o textos traducidos` (error) | Hay textos por idioma y no dices cuál es el de respaldo. |
| `/scene/assets: sobra.png is not used by the scene but goes into the package` (aviso) | Un archivo que ninguna parte de la escena usa. Se empaqueta igual y ocupa espacio: bórralo. |
| `/manifest: manifest.json.minisig signs an earlier version; a build without a key removes it` (aviso) | Cambiaste algo después de firmar. Firma de nuevo (ver «Firmar»). |
| `/scene/layers/3: data did not match any variant of untagged enum Value` (error) | Una capa tiene un valor que no es de ningún tipo permitido. El número es la posición de la capa: mira ahí su campo mal escrito. |
| Un paquete de más de 1 GiB, o con una imagen de más de 8192 píxeles de lado o 32 megapíxeles | Pierde la etiqueta `android` y `pack check` dice por qué. |

`pack check` no ejecuta los scripts ni abre las imágenes de la escena para ver si se ven bien:
ver `scripts.md`, «Lo que `pack check` no ve».

## Empaquetar

```
wprs-ctl pack build mi-escena -o reloj.wpak
```

Escribe el paquete y una línea con su huella SHA-256, su tamaño en bytes y el archivo:

```
<huella sha256> 1383 reloj.wpak
```

Antes de empaquetar, `pack build` calcula `requires.features` y escribe en el manifiesto las que
falten (y lo cuenta en la salida de error, como `requires.features += scripts (runs scene scripts)`).
Sin `-o`, deja `<id>-<versión>.wpak` en la carpeta actual.
`wprs-ctl scene build CARPETA --key autor.key` hace lo mismo y además firma en el mismo paso.

## Firmar

Firmar demuestra que el paquete lo hizo quien tiene la clave y que nadie lo cambió después. Un
repositorio puede exigir que los paquetes lleven la firma de un autor que él conoce.

**Crear la clave.** Un par de archivos, uno secreto y uno público, en la carpeta actual:

```
wprs-ctl repo keygen autor
```

Pide una contraseña (dos veces) y deja `autor.key` y `autor.pub`, e imprime la huella de la clave
pública. Con `--no-password` la clave secreta queda sin cifrar; hazlo solo para claves de
automatización. Sin terminal para preguntar, la contraseña se toma de la variable de entorno
`WPRS_KEY_PASSWORD`. En la app, *Claves de autor* en el diálogo de construir hace lo mismo y guarda
el par en `$XDG_DATA_HOME/wallpaper-rs/keys` (modo 0700).

**Dónde se guarda.** `autor.key` es un archivo con permisos 0600 (solo tú lo lees) y `autor.pub` es
público. Guarda la clave secreta en un lugar tuyo y con copia de seguridad: si la pierdes, no
puedes firmar actualizaciones con la misma identidad.

**Qué no compartir nunca.**

- El archivo `.key` (la clave secreta) ni su contraseña. Ni en el repositorio de tu código, ni en el
  paquete, ni en un mensaje, ni en una captura.
- Los archivos de otras claves de un repositorio (la clave raíz y la clave en línea): son de quien
  administra el repositorio.

Compartir la `.pub` y la huella sí está bien: es lo que permite a otros comprobar tus firmas.

**Firmar y empaquetar.**

```
wprs-ctl pack sign --key autor.key mi-escena
wprs-ctl pack build mi-escena -o reloj.wpak
wprs-ctl pack validate reloj.wpak --key autor.pub
```

`pack sign` completa `requires.features` y escribe `manifest.json.minisig` junto al manifiesto.
Comprueba con `pack validate --key`: si la firma es de esa clave, imprime `signer=` con su huella.

**Firma después de cada cambio.** La firma cubre los bytes exactos de `manifest.json`. Si cambias el
manifiesto (hasta la versión) o el programa lo reescribe, hay que firmar de nuevo antes de
`pack build`; si no, el paquete lleva una firma que no corresponde y `pack validate --key` dice
`signature rejected: trusted comment is not "wpak manifest <id> <versión>"`. Para no olvidarlo,
usa `scene build --key` o encadena `pack sign` y `pack build`.

## Probar la instalación

Con el demonio en marcha (lo inicia la app):

```
wprs-ctl lib import reloj.wpak --wait
wprs-ctl scene apply mi-escena
wprs-ctl scene preview mi-escena
```

- `lib import` instala el paquete en tu biblioteca: así lo verá quien lo instale. Imprime
  `job 1 import done: 0 failed, 1 imported`.
- `scene apply` construye la carpeta, la importa y la muestra en una salida (`-o DP-1`) o en todas.
- `scene preview` la muestra 60 segundos sin guardar nada.

## Publicar en un repositorio

Un **repositorio** es una carpeta de archivos estáticos que cualquier servidor web, recurso
compartido o ruta `file://` puede servir. Todo lo que un cliente usa va firmado, así que un espejo o
un servidor no necesita ser de confianza.

### Las claves del repositorio

| Rol | Qué firma | Cómo se guarda |
|-----|-----------|----------------|
| **raíz** (`root`) | La raíz: la lista de claves y las reglas del repositorio. | Fuera de línea. Se usa pocas veces al año. Puede haber varias y exigir que firmen *t* de *n*. |
| **en línea** (`online`) | El índice, que caduca (7 días por defecto). | En el equipo que publica, sin contraseña si lo hace un programa. Se usa en cada publicación. |
| **autor** | El `manifest.json` de cada paquete. | La del autor: la misma `autor.key` de arriba. |

### Crear el repositorio

```
wprs-ctl repo keygen raiz
wprs-ctl repo keygen enlinea --no-password
wprs-ctl repo init mi-repo --name "Mi repositorio" --root-key raiz.pub --online-key enlinea.pub --author mia=autor.pub --require-author-signature true
wprs-ctl repo sign-root --key raiz.key mi-repo
```

`repo init` escribe `root/1.json` en una carpeta vacía y dice que falta firmarlo; `sign-root`
lo firma con la clave raíz y `repo status mi-repo` muestra el resultado:

```
repo_id = <64 cifras hexadecimales>
version = 1
signers = <huella de la clave raíz>
threshold = 1
complete = true
anchor = ?fingerprint=<huella de la clave raíz>&repo_id=<id>
```

`--author ID=autor.pub` registra a un autor con su clave pública (repítelo para varios) y
`--require-author-signature true` rechaza los paquetes que no firmó ningún autor de la raíz. Hay
más ajustes: `--threshold`, `--root-days` (la raíz dura 365 días por defecto), `--entry-max-days`
(14), `--max-package-size`, `--mirror URL`, `--online-authors` (deja que los autores se
registren en un servidor).

### Añadir paquetes y publicar

```
wprs-ctl repo add mi-repo reloj.wpak --author mia
wprs-ctl repo publish --key enlinea.key mi-repo
```

`repo add` valida el paquete, comprueba su firma contra el autor y lo copia a `packages/<id>/` con
su vista previa (`--unlisted` lo deja fuera de la exploración y de la búsqueda, aunque se instala por
id). Nunca admite paquetes `imported`. `repo publish` firma y escribe el índice:

```
org.example.reloj 1.0.0 sha256=<huella sha256> author=mia
index 1 published with 1 packages; it expires 2026-11-19 12:17:16 UTC
```

El índice caduca. Antes de la fecha, vuelve a firmarlo sin cambiar nada:

```
wprs-ctl repo refresh --key enlinea.key mi-repo
```

Un repositorio con el índice caducado se muestra como «desactualizado» y las instalaciones piden
confirmación. `--days` cambia lo que dura (hasta `entry_max_days`).

### Servirlo

Copia la carpeta entera a cualquier servidor de archivos estáticos. Sirve `entry/latest` con
`Cache-Control: no-cache` y todo lo demás como inmutable. Para que otros lo agreguen, dales la
dirección con la consulta que imprime `repo status`:

```
https://example.org/fondos/?fingerprint=<huella de la clave raíz>&repo_id=<id>
```

Quien la reciba la agrega con `wprs-ctl remote add 'DIRECCIÓN'` o desde *Repositorios* en la app.
Para probarlo en tu equipo, una dirección `file://` con la misma consulta sirve:

```
wprs-ctl remote add 'file:///ruta/a/mi-repo/?fingerprint=…&repo_id=…' --wait
wprs-ctl pkg search reloj
wprs-ctl pkg install org.example.reloj
```

(Con el demonio en marcha. `remote add` con solo una ruta de carpeta, sin esa forma, falla con
«not a valid URL»).

### Actualizar un paquete publicado

Una versión publicada **no se puede reemplazar**: añadir otro contenido con la misma versión se
rechaza con «ya está publicado con otro contenido». Para actualizar, sube la versión:

1. Cambia `version` en el manifiesto (`1.0.0` a `1.1.0`) y, si quieres, `changes`.
2. `wprs-ctl pack sign --key autor.key mi-escena`
3. `wprs-ctl pack build mi-escena -o reloj-1.1.wpak`
4. `wprs-ctl repo add mi-repo reloj-1.1.wpak --author mia`
5. `wprs-ctl repo publish --key enlinea.key mi-repo`

Quien ya lo instaló ve la actualización al refrescar el repositorio (`wprs-ctl remote refresh`) y la
instala con `wprs-ctl pkg upgrade`. Nunca se baja de versión sin confirmación.

Para retirar algo: `repo unlist mi-repo ID [--version V]` lo saca de la exploración (se instala por
id) y `repo list-again` lo devuelve; `repo revoke mi-repo ID --reason broken [--version V]
[--message TEXTO]` lo revoca (razones: `malware`, `license`, `takedown`, `broken`, `other`) y los
clientes lo desactivan. Después de cada uno, `repo publish`. `repo feature mi-repo ID --note
TEXTO` lo destaca.

Para cambiar las claves o las reglas del repositorio: `repo rotate-root` escribe la raíz siguiente,
se firma con las claves de la raíz anterior y de la nueva, y se publica. La app tiene lo mismo en
*Repositorios*, pestaña *Repositorio estático*.

### Un servidor propio

Si tienes un servidor propio de wallpaper-rs, no hace falta una carpeta: se sube el paquete ya
firmado con `wprs-ctl server upload reloj.wpak --wait`, y el servidor lo revisa antes de publicarlo
(`wprs-ctl server author-key add` registra tu clave pública de autor). Solo acepta las licencias de
su lista y rechaza los paquetes `mature` e `imported`.

## Editar una copia de un paquete instalado

Para partir de un paquete que ya tienes (tuyo o de otro autor), se desempaqueta como una copia con un
id nuevo: el instalado nunca cambia.

```
wprs-ctl scene open mi-copia --from reloj.wpak --facts
wprs-ctl scene open mi-copia --from reloj.wpak
```

La primera imprime, sin escribir nada, la licencia, el veredicto (`allowed` o `refused`), las
condiciones y el id de la copia:

```
{"authors":[],"conditions":[],"credits":0,"id":"org.example.reloj","kind":"scene","license":"CC0-1.0","new_id":"org.example.reloj.copy","reason":"","title":"Reloj con script","verdict":"allowed","version":"1.0.0"}
```

La segunda desempaqueta y describe la carpeta. La copia:

- tiene el id `<id>.copy` (`--id` pone otro; `.copy2` si ya hay una copia de una copia);
- pierde la firma del autor original (`manifest.json.minisig`);
- conserva la licencia, los créditos y los archivos;
- pasa a ser `derived`: su `origin` cita al original (autores, título, licencia y página).

Tienes que escribir quién eres en `authors`. Si la licencia no permite obras derivadas, falla con
«… does not allow derived works: edita una copia solo si lo hiciste tú o si su autor lo permite»;
con `--mine` dices que el paquete es tuyo o que tienes el permiso. En la app: *Editar una copia…*
en la página de cada paquete de la biblioteca y en *Escenas*.

## Vídeos

Un fondo de vídeo es un paquete `kind: video` cuya `entry` es un `.webm`, `.mp4` o `.mkv`. Sus
ajustes (velocidad, inicio y final del bucle, volumen, zoom y posición) los fija el autor en
`playback` y `display` y el usuario los cambia por pantalla; hay un ejemplo en la especificación de
paquetes (`docs/specs/package.md`, «Video settings»).

El escritorio reproduce lo que sus decodificadores abren, y al añadir un vídeo comprueba que puede
abrirlo. `wprs-ctl` no convierte vídeos: prepáralos antes con la herramienta que prefieras.

El teléfono sí convierte. Al añadir un vídeo, la app comprueba que el teléfono tiene un
decodificador para su pista. Si solo puede decodificarlo con el procesador, pero sí decodifica por
hardware H.264 a ese tamaño o a 1080p o 720p, lo convierte a una copia H.264 con el codificador de
Android, y muestra el avance. En *Ajustes*, *Convertir los vídeos que el teléfono no reproduce con
fluidez* elige entre *Convertirlos* (la copia toma el lugar del original), *Convertir y conservar
el original* y *Nunca*. Los vídeos HDR de 10 bits salen siempre como H.264 de 8 bits con el rango
estándar. Por eso, para que un vídeo funcione bien en todas partes sin conversión, lo más seguro es
H.264 en MP4, a 1080p como mucho.

Los vídeos dentro de una escena (capas `video`) son archivos del paquete como los demás; se añaden
con `wprs-ctl scene import mi-escena fondo.mp4`.

## Antes de publicar: una lista

1. `wprs-ctl scene validate` sin errores y `wprs-ctl pack check` en `ok`.
2. Las entradas del manifiesto son las que la escena usa, ni una más.
3. `content` y `photosensitive` dicen la verdad, y la licencia es la que quieres.
4. La escena con scripts se vio en la vista previa y la consola no tiene errores.
5. La vista previa se ve bien sin scripts (una captura no los ejecuta).
6. `version` subió, y `pack sign` y `pack build` se hicieron después del último cambio.
7. `pack validate --key autor.pub` imprime `ok … signer=`.
8. No hay claves, contraseñas ni rutas personales dentro de la carpeta.

## Para seguir

- `scripts.md`: los scripts de la escena.
- `fondos-web.md`: páginas web como fondo.
- `cli.md`: todos los comandos de creación.
- `propiedades.md`: las propiedades y los preajustes del manifiesto.
