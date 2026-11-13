# Los comandos de creación de `wprs-ctl`

`wprs-ctl` es el programa de línea de comandos de wallpaper-rs. Esta página recoge los comandos que
usa quien crea escenas y paquetes: crear, importar, ver, capturar, revisar, empaquetar, firmar,
publicar y llevar un repositorio. Cada uno trae una línea que funciona y lo que imprime.

Cada comando explica sus opciones con `--help` (`wprs-ctl scene render --help`), en el idioma del
escritorio. Un comando que falla sale con un código distinto de 0 y escribe `Error: …`.

**Con demonio y sin demonio.** Casi todo lo de esta página funciona sin el demonio. Solo lo necesitan
`scene preview`, `scene apply`, `scene thumbnail --capture`, `lib import`, `remote add`, `clear`,
`restore` y los comandos de `pkg` e `input`. La app lo inicia; con ella cerrada, lo inicia el
servicio de usuario `wprsd`.

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

## Cambiar la escena con órdenes

Estas son las órdenes que usa el tutorial de [LEEME.md](LEEME.md) para armar una escena sin el editor.
Cambian la carpeta al momento y no necesitan el demonio.

### `scene add-layer`: una capa nueva

```
wprs-ctl scene add-layer mi-escena image --file cielo.png --name Fondo
```

```
fondo
```

Añade la capa y escribe su id, el nombre con el que las demás órdenes la nombran. El id sale del
`--name` (o, si no lo das, del tipo de capa) en minúsculas. Capas, recursos, sistemas de partículas y
líneas de tiempo comparten los ids: si el que saldría ya lo tiene cualquiera de ellos, se le añade
`_2`, `_3`… y la orden imprime el que usó. Una imagen que importas con `--file` se llama como su
archivo, así que `--file sunset.png --name Sunset` da la capa `sunset_2`: mira siempre lo que
imprime la orden. Con `--id` eliges tú el id; si ya está en uso, la orden falla con `the id fondo is
in use` y no cambia nada.

Una capa de imagen sin tamaño se ve de su tamaño real y centrada, no del tamaño del lienzo. Para
cubrirlo: `wprs-ctl scene layer mi-escena fondo transform --size 1920,1080`. Una capa de partículas
nueva trae ya una nieve que cae sobre todo el lienzo (ver [particulas-y-sonido.md](particulas-y-sonido.md)).

### `scene show`: leer cualquier valor

```
wprs-ctl scene show mi-escena /scene/layers/0
wprs-ctl scene show mi-escena /manifest/title
wprs-ctl scene show mi-escena
```

Imprime en JSON el valor que está en un JSON Pointer; sin puntero, los dos documentos juntos. El
puntero **empieza siempre por `/scene`** (el contenido de `scene.json`: `/scene/layers`,
`/scene/timelines`, `/scene/particles/nieve`) o por `/manifest` (el de `manifest.json`:
`/manifest/title`, `/manifest/inputs`). `/layers` no existe y la orden dice `Error: nothing is at that
pointer`; lo mismo pasa con una lista que aún no tiene nada, como `/scene/timelines` en una escena
sin líneas de tiempo. Las capas se cuentan desde 0 desde abajo: la primera que añadiste es
`/scene/layers/0`. `scene set mi-escena PUNTERO VALOR` escribe un valor en cualquiera de esos sitios,
y `scene validate` dice qué está mal con el mismo tipo de puntero.

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

Muestra la escena en el escritorio durante 60 segundos sin aplicarla ni guardar nada. Dice en qué
salidas la muestra y cómo terminarla. Sin `-o`, en todas las salidas.

| Opción | Qué hace |
|--------|----------|
| `--console` | Imprime lo que los scripts escriben con `console` y el error de la escena, con archivo y línea, a medida que salen, hasta que lo interrumpas. Sirve en cualquier salida, también en las de tus pantallas, pero la escena solo corre (y solo escribe) mientras algo mira esa salida. Ver `scripts.md`. |
| `--stop` | Termina la vista previa y dice en qué salidas la terminó. Si la app tenía encendido *Mostrar en el escritorio*, lo apaga también. |
| `--restart` | Empieza otra vez la escena, los scripts y las partículas. |
| `--time S` | Mantiene fijo ese segundo (hasta 86400). Con un valor negativo, la escena se reproduce. |
| `--quality q0`…`q3` | El nivel de calidad más alto de la vista previa. |
| `--battery`, `--calm` | La muestra como el demonio la muestra con batería, o con menos movimiento. |
| `--without-bloom` | La muestra sin el brillo del lienzo, para compararla con el brillo. |
| `--values ajustes.json` | Prueba valores de propiedades, como `{"speed": 2}`, sin guardarlos. |
| `--preset ID` | Parte de un preajuste del paquete antes de aplicar `--values`. |
| `--media pista.json` | Inventa una pista del reproductor: `title`, `artist`, `album`, `playing`, `position`, `duration`… |
| `--layout dual\|triple\|vertical\|mine --out archivo.png` | Dibuja la escena repartida entre varias pantallas, sin escritorio, en un archivo. |

### `scene apply`: construir, importar y mostrar (con demonio)

```
wprs-ctl scene apply mi-escena -o DP-1
```

```
applied: Mi reloj 1.0.2 on DP-1, quality Q3
```

Construye el paquete, lo añade a tu biblioteca y lo muestra en la salida (o en todas). Va diciendo
en la salida de error lo que hace (`Building the package…`, `Adding the package to the library…`,
`Showing it on the screen…`) y, si el demonio tarda porque tiene otros trabajos, avisa cada 5
segundos. Al final imprime el título, la versión, la pantalla y el nivel de calidad con que la
muestra; el texto sale en tu idioma. Termina con error si la pantalla dice que no pudo mostrarla.
Si cambias la escena sin subir la versión, la pantalla muestra lo nuevo igual. `--span` la extiende
sobre todas las pantallas.

Para quitar el fondo que acabas de aplicar:

```
wprs-ctl clear -o DP-1
wprs-ctl restore -o DP-1
```

`clear` quita el fondo de la salida (de todas sin `-o`) y deja solo el color de fondo; no imprime
nada. `restore` trae el que había, y dice `There is no earlier wallpaper to bring back.` si no hay
ninguno (por ejemplo, si después de quitarlo pusiste otro). Aplicar otro fondo olvida el anterior; quitar dos veces
conserva el primero. En la app están en *Ajustes*, en la fila de cada pantalla, como *Quitar el fondo* y
*Recuperar el fondo*.

### `input`: entradas simuladas (con demonio)

```
wprs-ctl input grant org.example.mi-escena pointer
wprs-ctl input touch DP-1 0.8,0.2
```

`input grant ESCENA ENTRADA` concede una entrada (`audio`, `pointer`, `clicks`, `media`…) a la
escena que se muestra (`ESCENA` es el `id` de su manifiesto, que `scene new mi-escena` deja como
`org.example.mi-escena`; no imprime nada, y `input status` confirma el permiso); `input touch SALIDA X,Y` pone un puntero inventado en esa fracción de la
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
Imprime los archivos que escribió. Dibuja lo mismo que la vista previa del editor: si elegiste
«mostrar solo» una capa, solo esa; con la semilla de las partículas fijada (una escena sin
`clock.seed` sale igual en cada archivo de la misma corrida) y con las luces silenciadas o en solitario.

| Opción | Qué hace |
|--------|----------|
| `-o ARCHIVO` | `.png`, `.jpg`, `.webp`, `.avif` o `.gif` (animado). Obligatorio. |
| `--time S` | El segundo de la escena del primer cuadro. |
| `--time-of-day HORA` | La hora de pared que ven la escena y sus scripts: `HH:MM[:SS]` de hoy o `AAAA-MM-DD HH:MM[:SS]`. Sin ella, la hora de este equipo; ponla para que la imagen salga igual siempre. |
| `--media ARCHIVO` | Una pista inventada del reproductor (JSON: `title`, `artist`, `album`, `playing`, `position`, `duration`, `rate`, `cover`…; la carátula es una imagen de la carpeta) que ven los scripts y las imágenes que siguen a la carátula. Sin ella, no hay reproductor. |
| `--quality q0`…`q3` | El nivel de calidad más alto (`q3` por defecto). |
| `--size PX` | El lado mayor de la imagen (1280 por defecto). |
| `--frames N`, `--fps F` | Con más de un cuadro, un clip: `.gif`, o una secuencia `NOMBRE-0001.EXT` (hasta 600 cuadros). |
| `--pointer X,Y[:X,Y]`, `--click` | Un puntero inventado en esas fracciones del lienzo, que se mueve al segundo par y pulsa hacia la mitad del clip. |
| `--against q0`, `--without-bloom`, `--split F` | La parte derecha se ve a otro nivel o sin brillo, para comparar. |
| `--battery`, `--calm` | Como con batería, o con menos movimiento. |
| `--language IDIOMA` | El idioma que ven los textos por idioma y los scripts (`es`, `pt-BR`); si el paquete no lo tiene, el idioma por defecto. |

**Ejecuta los scripts.** Para los cuadros que dibuja, corre los scripts de la escena con la hora de
pared de `--time-of-day` (o la de este equipo) y escribe en la salida de error lo que imprimen con
`console` (`script: log scripts/reloj.js: ...`) y sus errores, con archivo y línea
(`script: error scripts/reloj.js: TypeError: ... (scripts/reloj.js line 4)`). Antes del primer cuadro
da unos pasos a los scripts para que arranquen, así que las líneas de `console` salen repetidas (una por paso y por cuadro). No simula el sonido; el reproductor solo si pasas `--media`. Los scripts
nunca corren en el proceso de `wprs-ctl` (ni en el de la app): van a un proceso aparte con sandbox,
el mismo que usa el demonio y con los mismos límites de tiempo y memoria. Si ese proceso no puede
arrancar, dibuja sin scripts y lo dice en una línea en la salida de error.

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
`--fps` (1 a 30; 10 por defecto) a cuántos cuadros por segundo. Con `--capture` los scripts de la
escena sí corren, en el proceso aislado de siempre y con la hora de este equipo, así que un texto
que llenan sale con su valor y no con el inicial. No simula el reproductor. El cuadro sale sin sonido ni puntero: una escena que reacciona a ellos se ve en reposo.

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

**También revisa los scripts, sin ejecutarlos.** Compila cada módulo de `scripts/` y avisa de un
error de sintaxis o de un archivo que `scene.json` nombra y no está, con el archivo y la línea:

```
/scene/scripts/0/path: scripts/semaforo.js:4: unexpected token in expression: ';'
/scene/scripts/1/path: scripts/falta.js is listed in scripts but is not in the folder
Error: 2 problems
```

La línea que da el motor es donde se dio cuenta del error, que a veces es la siguiente a la que
tiene la errata. Lo que solo falla al correr (una función que lanza un error, un valor del tipo
equivocado) no se ve aquí: ver `scripts.md`, «Qué ve `validate` y qué no».

Entre las advertencias hay dos que nombran la capa: la que no se ve nunca porque otra de encima tapa todo el
lienzo sin transparencia (`La capa «foto» nunca se ve: «cielo», que está encima, tapa todo el lienzo…`) y la figura
que queda del todo fuera del lienzo. No impiden construir el paquete; el panel de problemas del editor las
muestra igual.

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

Sirve con una carpeta o con un `.wpak`. Se detiene en el primer error de la escena o del manifiesto
(`Error: …`). Los errores de los scripts (sintaxis, archivo que falta) no la detienen: salen como
líneas `problem:` debajo de la primera línea, que entonces dice `problems scene …` en vez de
`ok scene …`, y la orden termina con `Error: N problems` y el código 1:

```
problems scene org.example.reloj 1.0.0 (wallpaper-rs.wpak/2)
problem: scripts/reloj.js:4: unexpected token in expression: ';'
```

Qué significa cada línea y cada mensaje: `paquete.md`, «Revisar el paquete». `--json` imprime el
resultado en JSON, con los mismos problemas en `problems`.

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
`<id>-<versión>.wpak` en la carpeta actual. No compila los scripts: ejecuta antes `pack check` (o usa
`scene build`, que sí los revisa).

Si la carpeta trae una firma (`manifest.json.minisig`) hecha con otra versión del manifiesto, se
niega a armar un paquete que llevaría una firma que no corresponde:

```
Error: invalid manifest.json.minisig: it signs another version of manifest.json; run pack sign again, or delete it to build unsigned
```

Vuelve a ejecutar `pack sign` (o borra la firma para construir sin ella). `pack build` no necesita la
clave para darse cuenta, porque la firma lleva la huella del manifiesto que firmó. Las firmas
hechas con una versión anterior del programa no la llevan; para ellas, `--key autor.pub` (el archivo
de la clave pública o su texto) dice con qué clave comprobar que la firma aún vale, y sin esa
opción `pack build` pide volver a firmar. Con una clave que no es la de la firma, o con un manifiesto
cambiado, el mensaje es el mismo. `--key` no firma nada.

### `scene build`

```
wprs-ctl scene build mi-escena --key autor.key -o reloj.wpak
```

Lo mismo, y con `--key` (aquí la clave **secreta**, `autor.key`) firma en el mismo paso. Rechaza una
escena con errores, también los de sintaxis de sus scripts. Sin `--key`, quita la firma vieja que haya
en la carpeta y construye sin firmar: `scene build` nunca deja un paquete con una firma que no
corresponde.

### `repo keygen`: una clave nueva

```
wprs-ctl repo keygen autor
```

```
<huella de la clave pública>
```

Crea `autor.key` (secreta, modo 0600) y `autor.pub` en la carpeta actual y imprime la huella de la
pública. Pide una contraseña dos veces; sin terminal, la lee de la variable de entorno
`WPRS_KEY_PASSWORD` (también `pack sign` y `repo sign-root` la usan para abrir la clave), y si no
está avisa antes de crear nada. `--no-password` deja la clave sin cifrar. Al terminar dice cuál
archivo es el secreto y cuál el público. **No compartas nunca el `.key`.**

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
| `repo init DIR --name N --root-key raiz.pub --online-key enlinea.pub [--author ID=autor.pub]` | Crea el repositorio en una carpeta vacía y escribe `root/1.json`. Imprime su estado (`repo_id`, `signers`, `complete`, `anchor`, `address`). |
| `repo sign-root DIR --key raiz.key` | Firma la raíz más nueva con una clave raíz. Hasta que `complete = true`, el repositorio no sirve. |
| `repo status DIR` | Imprime el estado de firma de la raíz, la consulta de anclaje (`?fingerprint=…&repo_id=…`) y `address`, la dirección completa de la carpeta que se pega en *Añadir un repositorio* o en `remote add`. |
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
wprs-ctl pkg install org.example.reloj --wait
```

```
job 1 repo_add done: Mi repositorio (1 packages)
job 2 install done: org.example.reloj 1.0.0
Installed org.example.reloj 1.0.0.
```

La dirección es la línea `address` de `repo status`; también vale la carpeta con la misma consulta
(`wprs-ctl remote add 'mi-repo/?fingerprint=…&repo_id=…'`). Una carpeta sin esa consulta se rechaza
con `needs a fingerprint or repo_id query parameter`. Sin `--wait`, `remote add` y `pkg install`
solo imprimen el número del trabajo.

### Servidor propio

El servidor, su arranque sin Docker, el enlace de cada equipo y lo que se sincroniza están en el
README, «Servidor propio». Con un equipo ya enlazado:

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
| `scene bench reloj.wpak --frames 300` | Dibuja un paquete fuera de pantalla e informa del coste de cada cuadro (sin ejecutar los scripts). |
| `scene upgrade mi-escena --rating general --photosensitive false` | Lo mismo que `pack upgrade`. |
| `scene recover mi-escena` | Restaura los cambios sin guardar que la app dejó para una carpeta (`--discard` los descarta). |
| `pack new-assets`, `pack parts` | Hacen y listan paquetes de recursos con partes de una escena (`paquete.md`). |
