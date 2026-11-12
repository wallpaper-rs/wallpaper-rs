# Propiedades de usuario

Una propiedad de usuario es un ajuste que el usuario cambia sin tocar tu escena: un color, un
deslizador, una elección, una imagen suya. Tú declaras las propiedades en `manifest.json`, bajo
`properties`, y la escena las usa por su id. El programa las enseña como un formulario, en
*Ajustes de la escena*, y recuerda lo que el usuario elige.

Todos los ejemplos de esta página salen de
[`ejemplos/propiedades`](ejemplos/propiedades/), y pasan `wprs-ctl scene validate`.

## Declarar y usar una propiedad

En el manifiesto:

```json
"color_disco": {
  "alpha": false,
  "default": "#ffb347",
  "group": "apariencia",
  "label": {
    "en": "Disc color",
    "es": "Color del disco"
  },
  "order": 1,
  "type": "color"
}
```

En la escena, cualquier valor puede seguirla con `{ "prop": "color_disco" }`:

```json
    { "id": "disco", "type": "shape", "shape": { "ellipse": { "size": [300, 300] } },
      "fill": { "prop": "color_disco" }, "opacity": { "prop": "opacidad", "scale": 0.01 },
      "transform": { "position": [480, 540],
        "scale": { "timeline": "destello", "keys": [[0, 1], [0.3, 1.5, "out_cubic"], [1, 1, "in_out_sine"]] } } }
```

Qué valor da la propiedad depende de su tipo (ver abajo), y la escena lo toma ya recortado y
redondeado. Solo se puede seguir una propiedad que guarda un valor; un `button`, un `group`, una
`label` y un `divider` no lo guardan.

El id de una propiedad es una letra seguida de hasta 63 letras, cifras o `_`.

Desde la terminal:

```
wprs-ctl scene add-property MI-CARPETA color --id color_disco --label "Color del disco"
wprs-ctl scene property MI-CARPETA color_disco --default '"#ffb347"'
wprs-ctl scene set MI-CARPETA /scene/layers/2/fill '{"prop": "color_disco"}'
```

En el editor, el panel *Propiedades de usuario* lista las propiedades en el orden del formulario;
*Añadir una propiedad* crea una de cualquier tipo, y el inspector edita cada campo de su tipo.
Los enlaces se ponen con el rombo de cada valor, que ofrece *Seguir una propiedad de usuario*
(ver [editor.md](editor.md)).

## Campos comunes

Todas las propiedades pueden llevar:

| Campo | Qué hace |
|-------|----------|
| `type` | El tipo. Obligatorio. |
| `label` | El nombre que se ve. Si falta, se ve el id. |
| `help` | Una ayuda más larga. |
| `group` | El id de una propiedad `group` en la que se mete. |
| `order` | Un número: las propiedades se ordenan por él y después por id. |
| `icon` | Un icono, de la lista de abajo. |
| `visible_if` | Esconde la propiedad mientras la condición no se cumple. |
| `enabled_if` | La apaga (en gris) mientras la condición no se cumple. |
| `quality` | Valores por nivel de calidad, abajo. |

Las condiciones de un grupo valen para todos sus miembros.

## Los tipos

| `type` | Campos | Qué guarda |
|--------|--------|------------|
| `color` | `default`, `alpha` | Un color `#rrggbb` o, con `alpha`, `#rrggbbaa`. |
| `slider` | `min`, `max`, `default`, `step`, `decimals` (0 a 6), `integer`, `unit`, `editable`, `log` | Un número, recortado al rango, ajustado al paso y redondeado a los decimales. |
| `bool` | `default` | `true` o `false`. |
| `combo` | `options`, `default` | Uno de los valores de las opciones. |
| `text` | `default`, `max_len` (1 a 1024), `multiline`, `secret` | Un texto. |
| `file` | `accept` (`image`, `video`), `default_asset` | La ruta de un archivo que elige el usuario, o `null`. |
| `folder` | `accept`, `pick`, `every`, `mode`, `max_files`, `recursive` | La ruta de una carpeta que elige el usuario, o `null`. |
| `shortcut` | | Lo que el usuario eligió abrir, o `null`. |
| `button` | | Nada: pulsarlo es un suceso. |
| `group` | `collapsed` | Nada: lista debajo las propiedades que lo nombran. |
| `label`, `divider` | | Nada: una línea de texto, o una raya con un texto opcional. |

### Deslizador

```json
"opacidad": {
  "default": 100,
  "editable": true,
  "group": "apariencia",
  "help": {
    "en": "How solid the disc looks.",
    "es": "Qué tan sólido se ve el disco."
  },
  "integer": true,
  "label": {
    "en": "Disc opacity",
    "es": "Opacidad del disco"
  },
  "max": 100,
  "min": 10,
  "order": 2,
  "quality": {
    "q0": 50,
    "q1": 70
  },
  "step": 5,
  "type": "slider",
  "unit": "%"
}
```

Sin `step`, el paso sale de `decimals`, y si falta, de los decimales de `min`, `max` y `step` (el
mayor), o dos si no hay. Con `integer` no hay decimales. `min` y `max` se aceptan siempre, aunque
no caigan en la cuadrícula del paso, y ningún redondeo saca un valor del rango. `editable` añade
una caja para escribir el número, `unit` pone una unidad (`%`, `px`...) y `log` reparte el rango en
escala logarítmica (necesita `min` mayor que 0). Un `slider` puede seguirse en la escena con
`scale`, como `{ "prop": "opacidad", "scale": 0.01 }`.

### Elección (`combo`)

`options` es una lista de valores, o de `{ "value", "label", "icon" }`, hasta 64, todos del mismo
tipo (textos, números o `true` y `false`). Una opción con la etiqueta vacía enseña su valor.
En la escena, `map` elige un valor según la opción: `{ "prop": "estilo", "map": { "dia":
"#ffe08a", "noche": "#8fb4ff" } }`. En capas de imagen y de vídeo, parámetros de textura de
efectos, texturas de máscaras y texturas de partículas, `map` cambia de recurso, y así una
propiedad escoge entre variantes de tu escena.

### Texto

`default`, `max_len`, `multiline` (una caja de varias líneas) y `secret`. Un texto secreto no tiene
valor por defecto, se guarda en el llavero del sistema (en el teléfono, sellado por el almacén de
claves de Android) y nunca en la configuración, los registros, los preajustes, los ajustes
exportados ni la sincronización. Una escena lo lee como un texto vacío: solo el contenido que puede
usar la red lo recibe.

### Archivo y carpeta

Un `file` que elige el usuario reemplaza al recurso que nombra `default_asset` allí donde la
escena lo usa (capas de imagen y de vídeo, máscaras, texturas de efectos y de partículas), y se ve
dondequiera que `{ "prop": "id" }` lo nombra, con `default_asset` o sin él. Con `accept: ["video"]`
un vídeo se reproduce en capas de imagen y de vídeo, sin sonido y en bucle. Un cambio solo
reemplaza las texturas que toca: el reloj, las líneas de tiempo, las partículas y los scripts
siguen. `wprs-ctl pack check` avisa si las imágenes de las variantes de un mismo lugar no tienen
la misma proporción.

Una `folder` enseña una de sus imágenes (o vídeos, con `accept`) en una capa de imagen enlazada
con `{ "prop": "id" }`:

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `pick` | `random` | `random`, `sequence` | Al azar o en orden. |
| `every` | 60 | 10 a 86400 s | Cada cuánto cambia, mientras la escena se ve. |
| `mode` | `one` | `one`, `all` | `all` deja a las páginas web listar todos los archivos. |
| `max_files` | 1000 | 1 a 1000 | Cuántos archivos lee, en orden de nombre. |
| `recursive` | 0 | 0 a 2 | Niveles de subcarpetas. |

Solo se lee la propia carpeta (y sus subcarpetas hasta ese nivel), sin seguir enlaces que salgan de
ella ni leer archivos ocultos, y nada se lee mientras la escena no se ve. El usuario elige archivos
y carpetas con el selector de archivos; los preajustes no los fijan y no viajan con los ajustes
exportados ni la sincronización.

### Atajo

`shortcut` guarda lo que el usuario eligió abrir desde el fondo: una dirección `https://` (hasta
2048 caracteres), un archivo, una carpeta o una aplicación por su id de escritorio, nunca una
orden. Solo lo fija el usuario, nunca el autor, un preajuste ni la sincronización. El contenido
nunca ve el destino, solo `true` si hay uno; un script lo abre con `openShortcut(propiedad)` dentro
de un clic real de una capa (ver [scripts.md](scripts.md)). El paquete debe tener el permiso de
clics (ver [paquete.md](paquete.md)).

### Botón, grupo, etiqueta y separador

Un `button` no guarda nada; pulsarlo inicia las líneas de tiempo cuyo `trigger` es `prop:<id>`
y se lo enseña a los scripts durante un fotograma. No se puede pulsar mientras su `enabled_if` o el
de su grupo no se cumple. Un `group` junta propiedades (los grupos no se meten unos en otros) y
con `collapsed` aparece plegado. Una `label` es una línea de texto y un `divider` una raya con un
texto opcional.

```json
  "timelines": [{ "id": "destello", "duration": 1, "loop": "once", "autoplay": false, "trigger": "prop:destello" }]
```

El botón `destello` del ejemplo dispara esa línea de tiempo.

## Condiciones

`visible_if` y `enabled_if` son un mapa `{ "<otra propiedad>": valor }` (tienen que cumplirse
todos) o una expresión corta, nunca código, con ids de propiedades, números, textos entre comillas
`'...'` o `"..."`, `true`, `false`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!` y
paréntesis, como `"estilo == 'noche' || ver_galeria"`. Una expresión tiene hasta 256 caracteres y
8 niveles. Solo compara una propiedad con un valor del mismo tipo (`<` y compañía, solo números).
El validador rechaza ids desconocidos, propiedades sin valor, condiciones que no son verdaderas
ni falsas y condiciones que dependen de sí mismas.

```
wprs-ctl scene condition MI-CARPETA abrir "estilo == 'noche' || ver_galeria"
wprs-ctl scene condition MI-CARPETA carpeta ver_galeria --enabled
```

## Iconos

Un `icon` es uno de `appointment-soon`, `audio-volume-high`, `audio-volume-muted`,
`audio-x-generic`, `camera-photo`, `dialog-information`, `document-open`, `edit-find`,
`emblem-favorite`, `folder`, `go-home`, `image-x-generic`, `input-keyboard`, `input-mouse`,
`media-playback-start`, `media-playlist-repeat`, `media-playlist-shuffle`,
`preferences-desktop-theme`, `preferences-system`, `text-x-generic`, `video-display`,
`video-x-generic`, `view-refresh`, `weather-clear`, `weather-clear-night`, `weather-few-clouds`,
`weather-overcast`, `weather-showers`, `weather-snow`, `weather-storm`, `x-office-calendar` o
`zoom-in`: son iconos del tema del escritorio. Otros nombres de `[a-z0-9-]` se aceptan sin
enseñarse. Los paquetes no traen iconos propios.

## Valores según la calidad

`quality` lleva valores para los niveles `q0` a `q3` (`q3` el más alto). En ese nivel y en los más
bajos, el valor es como mucho ese en un deslizador y exactamente ese en una elección o un
interruptor. Así un modo de ahorro se activa solo con la batería baja. El valor guardado no cambia;
solo cambia lo que ven la escena, sus scripts y las páginas. En el ejemplo, la opacidad nunca pasa
de 50 en `q0` ni de 70 en `q1`.

## Preajustes

Un preajuste es una combinación de valores con un nombre, que el usuario aplica de una vez.

```json
"presets": [
  {
    "id": "calido",
    "label": {
      "en": "Warm",
      "es": "Cálido"
    },
    "values": {
      "color_disco": "#ff8a3d",
      "estilo": "dia",
      "opacidad": 90
    }
  },
  {
    "id": "frio",
    "label": {
      "en": "Cool",
      "es": "Frío"
    },
    "values": {
      "color_disco": "#7dd3fc",
      "estilo": "noche",
      "mensaje": "Buenas noches"
    }
  }
]
```

`presets` es una lista de `{ "id", "label", "values" }`. Un preajuste no puede fijar archivos,
carpetas, textos secretos ni botones. El usuario también puede guardar los valores actuales como
preajuste propio, renombrarlo, borrarlo, exportarlo a un archivo pequeño e importarlo; esos
preajustes pertenecen al id del paquete y sobreviven a las actualizaciones. Si una propiedad
cambia de tipo o desaparece, el valor guardado se ignora y se avisa. Desde la terminal:

```
wprs-ctl scene add-preset MI-CARPETA --id calido --label "Cálido"
wprs-ctl scene set MI-CARPETA /manifest/presets/0/values/color_disco '"#ff8a3d"'
```

Los valores se guardan una vez para todas las pantallas, o por pantalla: una pantalla sin valores
propios sigue y cambia los compartidos hasta que se le da una copia propia.

## Traducir las etiquetas

Las etiquetas, las ayudas, las opciones y los preajustes llevan texto de tres formas:

- un texto: `"label": "Color"`;
- un texto por idioma: `"label": { "es": "Color del disco", "en": "Disc color" }`;
- una clave de las cadenas del manifiesto: `"label": "@nota"`, con las traducciones en `strings`.

```json
"strings": {
  "en": {
    "nota": "Change anything you like: the scene updates at once."
  },
  "es": {
    "nota": "Cambia lo que quieras: la escena se actualiza al momento."
  }
}
```

```json
"nota": {
  "label": "@nota",
  "order": 0,
  "type": "label"
}
```

Con textos por idioma o con `strings`, el manifiesto debe llevar `default_locale` y cada texto
tiene que tener valor en ese idioma. Los idiomas son etiquetas como `es`, `en` o `pt-BR`; el
programa elige la de quien lo usa, y si no está, la del idioma por defecto. Un manifiesto admite hasta 64
idiomas, con hasta 2048 claves cada uno. Las traducciones de un idioma también pueden ir en
`locale/<idioma>.json` si el paquete lo declara (ver [paquete.md](paquete.md)).

Unas pocas etiquetas ya vienen traducidas por el programa: `"@wprs.accent_color"`,
`"@wprs.background_color"`, `"@wprs.brightness"`, `"@wprs.color"`, `"@wprs.density"`,
`"@wprs.opacity"`, `"@wprs.quality"`, `"@wprs.size"`, `"@wprs.speed"` y `"@wprs.volume"`. No hacen
falta `strings`, y tus claves no pueden empezar por `wprs.`.

```
wprs-ctl scene translate MI-CARPETA /manifest/properties/opacidad/label en "Disc opacity"
```

## Lo que el usuario ve

El formulario aparece en la página *Ajustes* de la app, y en el teléfono en la página de la
escena. Cada pantalla puede tener sus propios valores. Para cambiarlos desde la terminal el
usuario tiene `wprs-ctl prop` (ver [cli.md](cli.md)).

## No cubierto aquí

- `scripts`: leer y mover propiedades desde código: [scripts.md](scripts.md).
- El manifiesto entero y los permisos: [paquete.md](paquete.md).
