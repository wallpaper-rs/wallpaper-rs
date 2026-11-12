# Los comandos de creación de `wprs-ctl`

`wprs-ctl` es el programa de línea de comandos de wallpaper-rs. Esta página recoge los comandos que
usa quien crea escenas y paquetes: crear, importar, ver, capturar, revisar, empaquetar, firmar,
publicar y llevar un repositorio. Cada uno trae una línea que funciona y lo que imprime.

Cada comando explica sus opciones con `--help` (`wprs-ctl scene render --help`), en el idioma del
escritorio. Un comando que falla sale con un código distinto de 0 y escribe `Error: …`.

**Con demonio y sin demonio.** Casi todo lo de esta página funciona sin el demonio. Solo lo necesitan
`scene preview`, `scene apply`, `scene thumbnail --capture`, `lib import`, `remote add` y los comandos
de `pkg` e `input`. La app lo inicia; con ella cerrada, lo inicia el servicio de usuario `wprsd`.

**Las carpetas.** Una escena es una carpeta con `manifest.json`, `scene.json` y sus archivos. Casi
todos los comandos `scene` empiezan con `wprs-ctl scene COMANDO CARPETA`.

En los ejemplos, `mi-escena` es una carpeta de escena, `DP-1` el nombre de una salida (pantalla) y
`org.example.mi_escena` el id de un paquete.

## Crear

### `scene new`: una carpeta de escena nueva

```
wprs-ctl scene new mi-escena --from clock --title "Mi reloj" --locale es
```

No imprime nada y deja `manifest.json`, `scene.json` y `assets/` en `mi-escena/`.

| Opción | Qué hace |
|--------|----------|
| `--from` | El punto de partida: `empty` (el predeterminado), `particles`, `parallax`, `clock`, `web` (una página web) o `preset` (los valores de otro paquete). |
| `--id` | El id del paquete. Sin él, `org.example.<nombre de la carpeta>`. |
| `--title` | El título. Sin él, el nombre de la carpeta. |
| `--locale` | El idioma del título (`es`, `pt-BR`). Sin él, el del escritorio. |

La carpeta que da ya pasa la revisión (`pack check`). Cambia el `id` antes de publicar.

### `scene open`: editar una copia de un paquete

```
wprs-ctl scene open mi-copia --from reloj.wpak
```

```
Reloj con script (org.example.reloj.copy 1.0.0): 3 layers, 0 problems
```

Desempaqueta el `.wpak` en una carpeta nueva como una copia sin firma, con el id `<id>.copy`. Sin
`--from`, describe una carpeta que ya existe. Más opciones: `--id` (otro id para la copia), `--facts`
(imprime en JSON la licencia, el veredicto `allowed` o `refused` y el id de la copia, sin escribir
nada), `--mine` (confirma que el paquete es tuyo o que su autor permite la copia, para licencias que no
permiten obras derivadas) y `--json`. Ver «Editar una copia» en `paquete.md`.

## Importar

### `scene import`: un archivo a la escena

```
wprs-ctl scene import mi-escena fondo.png --id cielo
```

```
cielo
```

Copia una imagen, un vídeo, un sonido, una fuente o una tabla `.cube` a `assets/` y la declara como
un recurso de la escena. Imprime el id del recurso (el nombre del archivo si no das `--id`).
`--linear` lee la imagen como datos (un mapa de normales, una máscara) y no como colores;
`--atlas FILAS,COLUMNAS` la divide en una hoja de sprites.

### `scene import-script`: un script a la escena

```
wprs-ctl scene import-script mi-escena scripts-mios/sigue.js
wprs-ctl scene import-script mi-escena --template timer
```

```
scripts/sigue.js se añadió
scripts/sigue.js ahora usa pointer
```

Copia el módulo a `scripts/` y escribe en `scene.json` qué eventos atiende y qué partes de `wprs`
usa. Con `--template` parte de una plantilla: `empty`, `media`, `touch`, `timer`, `audio`, `spawn`,
`wave`, `daytime`, `setting` o `launches`. Ver `scripts.md`.

### `lib import`: un paquete a la biblioteca (con demonio)

```
wprs-ctl lib import reloj.wpak --wait
```

```
job 1 import done: 0 failed, 1 imported
```

Instala el paquete (o una imagen o un vídeo) en tu biblioteca. Sin `--wait`, imprime el número del
trabajo y vuelve enseguida.

## Ver la escena

### `scene preview`: una vista previa (con demonio)

```
wprs-ctl scene preview mi-escena -o DP-1
```

Muestra la escena en el escritorio durante 60 segundos sin aplicarla ni guardar nada. No imprime
nada. Sin `-o`, en todas las salidas.

| Opción | Qué hace |
|--------|----------|
| `--console` | Imprime lo que los scripts escriben con `console` y el error de la escena, a medida que salen, hasta que lo interrumpas. Ver `scripts.md`. |
| `--stop` | Termina la vista previa. |
| `--restart` | Empieza otra vez la escena, los scripts y las partículas. |
| `--time S` | Mantiene fijo ese segundo (hasta 86400). Con un valor negativo, la escena se reproduce. |
| `--quality q0`…`q3` | El nivel de calidad más alto de la vista previa. |
| `--battery`, `--calm` | La muestra como el demonio la muestra con batería, o con menos movimiento. |
| `--values ajustes.json` | Prueba valores de propiedades, como `{"speed": 2}`, sin guardarlos. |
| `--preset ID` | Parte de un preajuste del paquete antes de aplicar `--values`. |
| `--media pista.json` | Inventa una pista del reproductor: `title`, `artist`, `album`, `playing`, `position`, `duration`… |
| `--layout dual\|triple\|vertical\|mine --out archivo.png` | Dibuja la escena repartida entre varias pantallas, sin escritorio, en un archivo. |

### `scene apply`: construir, importar y mostrar (con demonio)

```
wprs-ctl scene apply mi-escena -o DP-1
```

```
01M77VZVQ51DK8N8PWEP43HRYA
```

Construye el paquete en tu biblioteca y lo muestra en la salida (o en todas). Imprime el id del
elemento de la biblioteca. `--span` la extiende sobre todas las pantallas.

### `input`: entradas simuladas (con demonio)

```
wprs-ctl input grant org.example.mi_escena pointer
wprs-ctl input touch DP-1 0.8,0.2
```

`input grant ESCENA ENTRADA` concede una entrada (`audio`, `pointer`, `clicks`, `media`…) a la
escena que se muestra; `input touch SALIDA X,Y` pone un puntero inventado en esa fracción de la
pantalla (`--stop` lo quita) y `input status` dice qué usa y qué tiene concedido cada pantalla. Sirven
para probar scripts y propiedades sin tocar el ratón.

## Capturar

### `scene render`: una imagen o un clip, sin escritorio

```
wprs-ctl scene render mi-escena -o vista.png --size 640
```

```
vista.png
```

Dibuja la escena como la dibujaría el demonio, sin demonio ni pantalla, y escribe el archivo.
Imprime los archivos que escribió.

| Opción | Qué hace |
|--------|----------|
| `-o ARCHIVO` | `.png`, `.jpg`, `.webp`, `.avif` o `.gif` (animado). Obligatorio. |
| `--time S` | El segundo de la escena del primer cuadro. |
| `--quality q0`…`q3` | El nivel de calidad más alto (`q3` por defecto). |
| `--size PX` | El lado mayor de la imagen (1280 por defecto). |
| `--frames N`, `--fps F` | Con más de un cuadro, un clip: `.gif`, o una secuencia `NOMBRE-0001.EXT` (hasta 600 cuadros). |
| `--pointer X,Y[:X,Y]`, `--click` | Un puntero inventado en esas fracciones del lienzo, que se mueve al segundo par y pulsa hacia la mitad del clip. |
| `--against q0`, `--without-bloom`, `--split F` | La parte derecha se ve a otro nivel o sin brillo, para comparar. |
| `--battery`, `--calm` | Como con batería, o con menos movimiento. |

**No ejecuta los scripts.** El dibujo sale como lo deja `scene.json`. Para ver una escena con scripts,
usa `scene preview`.

### `scene thumbnail`: la imagen del paquete

```
wprs-ctl scene thumbnail mi-escena captura.png
wprs-ctl scene thumbnail mi-escena --capture --time 3
wprs-ctl scene thumbnail mi-escena --capture --animated --seconds 4 --fps 10
```

```
assets/preview.png
```

Pone una vista previa en `assets/` y la escribe en `previews.thumbnail` del manifiesto: la imagen
que das, reducida si pasa de 1024 píxeles o 1 MiB; o, con `--capture`, un cuadro que dibuja la
tarjeta gráfica (con demonio). Con `--animated`, un `.gif` o `.webp` de hasta 640 píxeles de lado,
15 s y 8 MiB (`assets/preview-animated.gif`, en `previews.animated`). `--time` es el segundo que se
guarda, `--from` donde empieza la grabación, `--seconds` (0,1 a 15; 4 por defecto) cuánto dura y
`--fps` (1 a 30; 10 por defecto) a cuántos cuadros por segundo. Tampoco ejecuta los scripts.

## Revisar

### `scene validate`: todos los problemas de la carpeta

```
wprs-ctl scene validate mi-escena
```

```
/manifest/inputs: inputs: la escena usa pointer en /scripts/0 de scene.json; añade "pointer"
warning: /manifest: manifest.json.minisig signs an earlier version; a build without a key removes it
warning: /scene/assets: sobra.png is not used by the scene but goes into the package
Error: 1 problem
```

Una línea por problema, con el JSON Pointer del campo, y después las advertencias con `warning:`. Sale con 1
si hay algún error. `--json` imprime un objeto por problema.

### `pack check`: qué necesita el paquete y dónde se reproduce

```
wprs-ctl pack check mi-escena
wprs-ctl pack check reloj.wpak
```

```
ok scene org.example.reloj 1.0.0 (wallpaper-rs.wpak/2)
title = Reloj con script
rating = general
photosensitive = no
origin = original
needs = scripts (runs scene scripts)
facets = 1080p, android, animated, desktop, landscape, localized, scripts
desktop = plays
android = plays
languages = en, es
```

Sirve con una carpeta o con un `.wpak`. Se detiene en el primer error (`Error: …`). Qué significa cada
línea y cada mensaje: `paquete.md`, «Revisar el paquete». `--json` imprime el resultado en JSON.

### `pack validate`: la firma

```
wprs-ctl pack validate reloj.wpak --key autor.pub
```

```
ok scene org.example.reloj 1.0.0 sha256=<huella sha256> signer=<huella de la clave>
```

Valida uno o varios paquetes (o carpetas) y, con `--key`, comprueba que los firmó esa clave pública
(un archivo o su texto). Sin la firma buena sale `signature rejected: …` y el código 1.

## Empaquetar y firmar

### `pack build`

```
wprs-ctl pack build mi-escena -o reloj.wpak
```

```
<huella sha256> 1383 reloj.wpak
```

Arma el paquete: calcula `requires.features`, lo escribe en el manifiesto y lo comprime. Imprime la
huella SHA-256, el tamaño en bytes y el archivo (con `--json`, `{"path", "sha256", "size"}`). Sin `-o`,
`<id>-<versión>.wpak` en la carpeta actual.

### `scene build`

```
wprs-ctl scene build mi-escena --key autor.key -o reloj.wpak
```

Lo mismo, y con `--key` firma en el mismo paso. Rechaza una escena con errores. Sin `--key`, quita la
firma vieja que haya en la carpeta.

### `repo keygen`: una clave nueva

```
wprs-ctl repo keygen autor
```

```
<huella de la clave pública>
```

Crea `autor.key` (secreta, modo 0600) y `autor.pub` en la carpeta actual y imprime la huella de la
pública. Pide una contraseña dos veces; sin terminal, la lee de `WPRS_KEY_PASSWORD`. `--no-password`
deja la clave sin cifrar. **No compartas nunca el `.key`.**

### `pack sign`

```
wprs-ctl pack sign --key autor.key mi-escena
```

```
<huella de la clave>
```

Completa `requires.features` y firma el manifiesto de la carpeta (`manifest.json.minisig`). Ejecútalo
justo antes de `pack build`, y otra vez después de cualquier cambio.

### `pack rotate-key`

```
wprs-ctl pack rotate-key --author mia autor.key nueva.pub
```

Imprime la firma, con la clave vieja, del traspaso de un autor en línea a una clave nueva; se la das a
`repo rotate-author` o a `server author-key rotate`.

### `pack upgrade`: de la primera versión a la segunda

```
wprs-ctl pack upgrade mi-escena --rating general --photosensitive false
```

Convierte una carpeta de paquete `wallpaper-rs.wpak/1` en `/2`. Nunca adivina la clasificación:
`--rating` y `--photosensitive` son obligatorios (`--reason` para `mature`, `--derived-from AUTOR`
para una obra derivada).

## Repositorios

Un repositorio es una carpeta de archivos firmados que cualquier servidor estático puede servir. El
camino completo, con las claves y lo que imprime cada paso, está en `paquete.md`, «Publicar en un
repositorio». Aquí van sus comandos. Ninguno necesita el demonio.

| Comando | Qué hace |
|---------|----------|
| `repo init DIR --name N --root-key raiz.pub --online-key enlinea.pub [--author ID=autor.pub]` | Crea el repositorio en una carpeta vacía y escribe `root/1.json`. Imprime su estado (`repo_id`, `signers`, `complete`, `anchor`). |
| `repo sign-root DIR --key raiz.key` | Firma la raíz más nueva con una clave raíz. Hasta que `complete = true`, el repositorio no sirve. |
| `repo status DIR` | Imprime el estado de firma de la raíz y la consulta de anclaje (`?fingerprint=…&repo_id=…`) que se agrega a la dirección. |
| `repo add DIR paquete.wpak… [--author ID] [--unlisted]` | Añade paquetes al catálogo; después hay que publicar. Imprime `id versión sha256=… author=…`. |
| `repo publish DIR --key enlinea.key [--days N]` | Firma y escribe el índice. Imprime `index N published with M packages; it expires FECHA`. |
| `repo refresh DIR --key enlinea.key [--days N]` | Vuelve a firmar el índice sin cambios con una caducidad nueva. |
| `repo unlist DIR ID [--version V]`, `repo list-again DIR ID [--version V]` | Oculta de la exploración y la búsqueda (aún se instala por id), o lo vuelve a mostrar. |
| `repo feature DIR ID [--note TEXTO]`, `repo unfeature DIR ID` | Destaca un paquete, o deja de destacarlo. |
| `repo revoke DIR ID --reason R [--version V] [--message M]`, `repo unrevoke DIR ID [--version V]` | Revoca un paquete o una versión (`malware`, `license`, `takedown`, `broken`, `other`), o lo retira. |
| `repo revoke-key DIR HUELLA --reason R` | Revoca una clave de autor. |
| `repo rotate-root DIR [--root-key …] [--online-key …] [--author …]` | Escribe la raíz siguiente con las claves o las reglas cambiadas; se firma con `sign-root` y se publica. |
| `repo register-author SERVIDOR --key autor.pub`, `repo rotate-author`, `repo drop-author-key` | Registra, traspasa o quita la clave de un autor en línea de un servidor. |

`repo add` rechaza los paquetes `imported`, y con `--require-author-signature` en la raíz, los que
no llevan la firma de un autor conocido (`firma rechazada: el paquete debe llevar una firma de
autor`). `repo add` con otro contenido de una versión ya publicada falla con «ya está publicado con
otro contenido».

Para probarlo como lo haría otro equipo (con demonio):

```
wprs-ctl remote add 'file:///ruta/a/mi-repo/?fingerprint=<huella>&repo_id=<id>' --wait
wprs-ctl pkg search reloj
wprs-ctl pkg install org.example.reloj
```

```
job 1 repo_add done: Mi repositorio (1 packages)
```

### Servidor propio

```
wprs-ctl server upload reloj.wpak --wait
```

Sube un paquete ya firmado al repositorio del servidor al que estás enlazado. Registra tu clave
pública antes con `server author-key add`. Lo que revisa el servidor y lo que rechaza: `paquete.md`.

## Las acciones del editor: `scene act`, `scene actions` y `scene ask`

Todo lo que el editor de la app puede hacer con una escena (añadir una capa, mover un recurso,
animar una propiedad, cambiar el manifiesto…) es una **acción**, y se aplica a una carpeta con tres
comandos, los mismos que usa la app. Esta página solo los presenta; `acciones.md` explica cada
acción y sus argumentos.

```
wprs-ctl scene actions
wprs-ctl scene actions hide
wprs-ctl scene act mi-escena hide '{"id": "fecha", "hidden": true}'
wprs-ctl scene ask mi-escena lights
```

- **`scene actions`** lista las acciones con una línea cada una (`set`, `add_layer`, `hide`…). Con el
  nombre de una, imprime sus argumentos y un ejemplo:

  ```
  hide: Show or hide a layer
    hidden: boolean (required)
    id: string (required)

  Example:
    wprs-ctl scene act DIR hide '{"hidden":false,"id":"text"}'
  ```

  `--json` imprime la lista, o el esquema JSON de una acción. `--queries` hace lo mismo con las
  preguntas de `scene ask`.
- **`scene act CARPETA ACCIÓN [JSON]`** aplica una acción y guarda la carpeta. Los argumentos son un
  objeto JSON (`{}` si se omiten); con `-` se leen de la entrada estándar y con `--file RUTA`, de un
  archivo. No imprime nada si sale bien; con `--json` imprime las operaciones que cambiaron el
  documento. La acción `batch` aplica una lista `[{"acción": {argumentos}}, …]` como un solo paso de
  deshacer.
- **`scene ask CARPETA CONSULTA [JSON]`** le hace una pregunta al editor sobre la carpeta (las luces,
  el coste de las partículas, el esqueleto de una capa…) e imprime la respuesta en JSON.

Una acción que dejaría la escena con un error se rechaza y no cambia nada. Cada paso se puede deshacer
con la acción `undo` (y volver a hacer con `redo`).

## Otros comandos útiles

| Comando | Para qué |
|---------|----------|
| `scene bench reloj.wpak --frames 300` | Dibuja un paquete fuera de pantalla e informa del coste de cada cuadro. |
| `scene upgrade mi-escena --rating general --photosensitive false` | Lo mismo que `pack upgrade`. |
| `scene recover mi-escena` | Restaura los cambios sin guardar que la app dejó para una carpeta (`--discard` los descarta). |
| `pack new-assets`, `pack parts` | Hacen y listan paquetes de recursos con partes de una escena (`paquete.md`). |
