# Los efectos

Un efecto cambia cómo se ve una capa: la desenfoca, la tiñe, la hace ondular. Esta página explica
cómo se ponen los efectos, qué efectos trae el programa, cómo se limitan con una máscara y cómo
escribir uno propio.

Los ejemplos salen de [`ejemplos/efectos-apilados`](ejemplos/efectos-apilados/scene.json) y de
[`ejemplos/efecto-propio`](ejemplos/efecto-propio/scene.json), y pasan `wprs-ctl scene validate`.

## Poner efectos en una capa

Cada capa tiene una lista `effects` de hasta 16 efectos. Se aplican en orden: el primero actúa
sobre la capa tal cual, el segundo sobre el resultado del primero, y así.

```json
    { "id": "fondo", "type": "image", "image": "cielo", "size": [1920, 1080],
      "effects": [
        { "effect": "color_adjust", "name": "color", "enabled": { "prop": "color_vivo" },
          "params": { "saturation": 1.8, "contrast": 1.15 } },
        { "effect": "waves", "params": { "amplitude": 10, "frequency": 5, "speed": 1.5 },
          "mask": { "texture": "mancha", "channel": "r", "feather": 0 } },
        { "effect": "blur", "params": { "radius": 6 },
          "mask": { "shape": { "rect": { "size": [1920, 260] } }, "invert": true, "strength": 0.8 },
          "optional": true }
      ] }
```

Cada elemento de la lista es un uso del efecto:

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `effect` | obligatorio | El id del efecto, de los de abajo o uno del paquete. |
| `params` | los de cada efecto | Los parámetros que cambias. Cada uno puede ser un valor fijo o seguir una propiedad, el tiempo, el sonido o el puntero. |
| `enabled` | `true` | `false` lo apaga sin quitarlo. Puede seguir una propiedad. |
| `mask` | sin máscara | Limita el efecto a una parte de la capa. Ver "Máscaras". |
| `optional` | `false` | `true` deja fuera el efecto, y no la escena, donde no pueda correr. |
| `name` | | Un nombre propio de este uso, con el que lo llaman los scripts y los enlaces. |

Los parámetros que no pones valen lo que dice su descriptor. En los parámetros de tipo imagen
(`texture`) escribes el id de un recurso de imagen, y en los de tipo tabla (`lut`) el de un
recurso `.cube`. Una imagen también puede ser `{ "layer": "id" }`, una capa dibujada aparte (ver
[escena.md](escena.md)).

Los puntos como `center` o `source` son fracciones de la capa (del lienzo, en una capa de ajuste).
Los que tienen un interruptor `follow` pasan al puntero mientras está en la pantalla y vuelven
suavemente a su sitio en un cuarto de segundo cuando se va.

Para ponerlos desde la terminal:

```
wprs-ctl scene effects MI-CARPETA fondo add waves --param amplitude=6 --param frequency=3
wprs-ctl scene effects MI-CARPETA fondo set 0 --param amplitude=10
wprs-ctl scene effects MI-CARPETA fondo move 0 1
wprs-ctl scene effects MI-CARPETA fondo remove 0
```

(`set`, `move` y `remove` toman el lugar del efecto en la lista, empezando por 0.)
En el editor se hace desde la lista de efectos de la capa, con *Añadir un efecto*.

### Efectos sobre todo lo de abajo

Una capa de ajuste (`adjustment`) aplica sus efectos a todo lo que hay debajo de ella. Es la forma
de dar un acabado a toda la escena, como el resplandor y el grano del ejemplo:

```json
    { "id": "pantalla", "type": "adjustment",
      "effects": [
        { "effect": "bloom", "params": { "threshold": 0.7, "intensity": 0.8, "radius": 24 } },
        { "effect": "vignette", "params": { "intensity": 0.45, "color": "#000010" } },
        { "effect": "film_grain", "params": { "amount": 0.04, "fps": 12 } }
      ] }
```

El lienzo también tiene un resplandor propio, `canvas.bloom`, que hace lo mismo que un `bloom`
arriba del todo (ver [escena.md](escena.md)).

### Calidad y costo

El nivel de calidad baja con la batería, el calor o la potencia del equipo. Entonces los efectos
cambian así:

- Los efectos **ligeros** corren en todos los niveles.
- Los **medios** se quitan en `q0`, y en `q1` si los marcaste `optional`.
- Los **pesados** corren sus pasadas interiores a un cuarto de tamaño en `q1`.
- La pasada que escribe el resultado siempre va a tamaño completo.

El catálogo dice qué efectos son ligeros, medios o pesados. Un efecto que no corre en el nivel
actual no pide su entrada, y tampoco uno cuyo `enabled` está apagado por una propiedad. Lo que
cuesta cada uno y cuánto cuesta la escena entera se explica en [escena.md](escena.md).

Los desenfoques y las distorsiones tratan los bordes de la pantalla como los bordes de la imagen:
una capa que llega más allá de la pantalla conserva el color de sus píxeles más exteriores.

## Máscaras

Una máscara limita un efecto a una parte de la capa, y puede ser una imagen, otra capa o una forma:

```json
    { "id": "fondo", "type": "image", "image": "cielo", "size": [1920, 1080],
      "effects": [
        { "effect": "color_adjust", "name": "color", "enabled": { "prop": "color_vivo" },
          "params": { "saturation": 1.8, "contrast": 1.15 } },
        { "effect": "waves", "params": { "amplitude": 10, "frequency": 5, "speed": 1.5 },
          "mask": { "texture": "mancha", "channel": "r", "feather": 0 } },
        { "effect": "blur", "params": { "radius": 6 },
          "mask": { "shape": { "rect": { "size": [1920, 260] } }, "invert": true, "strength": 0.8 },
          "optional": true }
      ] }
```

(En el primer efecto con máscara, `waves` solo actúa donde la imagen `mancha` es clara; en el
segundo, `blur` solo actúa fuera de un rectángulo, y con la fuerza al 80 %.)

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `texture` | | Un recurso de imagen. Debe ser `linear` si es una máscara de datos. |
| `layer` | | El id de otra capa. Se dibuja aparte y enmascara por donde cae en pantalla. |
| `shape` | | Una forma (ver [capas.md](capas.md)), centrada en la capa y medida en sus píxeles. |
| `channel` | `luminance` con imagen, `a` con capa o forma | `r`, `g`, `b`, `a` o `luminance`. |
| `invert` | `false` | Enmascara el exterior. |
| `feather` | 0 | Suaviza el borde. |
| `strength` | 1 | De 0 a 1: `mezcla(1, máscara, fuerza)`. |

Pon exactamente una de `texture`, `layer` y `shape`. Desde la terminal:

```
wprs-ctl scene effects MI-CARPETA fondo mask 1 --shape '{"rect": {"size": [1920, 260]}}' --invert true
```

Con `scene paint-mask` se pinta una máscara de imagen con trazos y, en el editor, con un pincel
(ver [editor.md](editor.md)).

## Los efectos que trae el programa

Cada efecto lleva su nombre en español como aparece en la app, su costo y sus parámetros, con lo
que valen por defecto y el rango que el programa acepta. Los rangos se comprueban al validar:
`/layers/2/effects/0/params/radius is 300, expected 0..=128`.

Algunos datos que valen para varios:

- `tilt_shift` desenfoca fuera de una franja o, si le das un `depth`, según un mapa de profundidad.
- `glitch` se queda quieto hasta que pones `bursts` o `beat`.
- `film_grain` solo cambia con el tiempo si `fps` es mayor que 0.
- `depth_parallax` no cuesta nada mientras no mueva nada (sin órbita, con el puntero lejos o con
  `strength` en 0). `overscan` agranda la capa para que no se vean los bordes.
- Los efectos con una fase (`phase`) son imágenes `linear` cuyo brillo desplaza el tiempo de cada
  punto, para que las partes vecinas no se muevan juntas.
- Las imágenes `gradient` son de una sola fila, y se leen de izquierda a derecha.

### Movimiento

**`caustics`** (Cáusticas, ligero). Luces móviles como el sol a través del agua.

Parámetros: `color` (color, por defecto `#cfefff`); `scale` (número, por defecto 2; de 0,1 a 16); `speed` (número, por defecto 0,5; de 0 a 8); `strength` (número, por defecto 0,6; de 0 a 4); `perspective` (número, por defecto 0; de 0 a 1).

**`clouds`** (Nubes y niebla, medio). Nubes o niebla que pasan, hechas con ruido.

Parámetros: `color` (color, por defecto `#ffffff`); `shade` (color, por defecto `#a7b0c2`); `opacity` (número, por defecto 0,8; de 0 a 1); `coverage` (número, por defecto 0,45; de 0 a 1); `softness` (número, por defecto 0,3; de 0,01 a 1); `size` (número, por defecto 320; de 8 a 16384); `speed` (`[x, y]`, por defecto `[6, 0]`; de -1024 a 1024); `warp` (número, por defecto 0,8; de 0 a 4); `octaves` (número, por defecto 5; de 1 a 8); `height` (número, por defecto 1; de 0 a 1).

**`electric`** (Eléctrico, ligero). Líneas finas y crepitantes donde se cruzan dos ruidos que se desplazan.

Parámetros: `color_a` (color, por defecto `#66ccff`); `color_b` (color, por defecto `#ffffff`); `bounds` (`[x, y]`, por defecto `[0.4, 0.6]`; de 0 a 1); `scale` (número, por defecto 2; de 0,1 a 16); `smoothness` (número, por defecto 0,1; de 0 a 1); `speed` (número, por defecto 1; de 0 a 8); `multiply` (número, por defecto 1; de 0 a 8); `blend` (modo de mezcla, por defecto `add`).

**`fire`** (Fuego, ligero). Levanta y hace parpadear las llamas que ya hay en la imagen, y las colorea según el brillo con un degradado.

Parámetros: `speed` (número, por defecto 1; de 0 a 8); `strength` (número, por defecto 8; de 0 a 64); `scale` (número, por defecto 3; de 0,1 a 16); `gradient` (imagen).

**`flow`** (Flujo, ligero). Deforma la imagen a lo largo de un mapa de flujo; sin mapa usa un campo de flujo propio.

Parámetros: `map` (imagen); `strength` (número, por defecto 8; de 0 a 512); `speed` (número, por defecto 0,25; de 0 a 8); `direction` (`[x, y]`, por defecto `[1, 0]`; de -1 a 1); `noise` (sí/no, por defecto no); `phase` (imagen).

**`fluid`** (Fluido, pesado). Humo, fuego o líquido simulados sobre la capa, con emisores de punto y de línea y el puntero. Cada emisor es `{ "from", "to", "angle", "strength" }`, en fracciones de la capa (de -2 a 4); `angle` de -360 a 360 (-90 es hacia arriba) y `strength` de 0 a 64; hasta 64 emisores.

Parámetros: `render` (opción, por defecto 0): 0 degradado, 1 color del emisor, 2 la propia imagen, 3 solo distorsión; `gradient` (imagen); `color` (color, por defecto `#ffffff`); `velocity_decay` (número, por defecto 0,98; de 0 a 1); `dye_decay` (número, por defecto 0,97; de 0 a 1); `push` (número, por defecto 1; de 0 a 8); `emitters` (lista de emisores); `strength` (número, por defecto 1; de 0 a 8); `blend` (modo de mezcla, por defecto `normal`); `perspective` (número, por defecto 0; de 0 a 1); `opaque` (sí/no, por defecto no).

**`glitch`** (Fallo digital, ligero). Fallo digital con filas desplazadas y colores partidos. Se queda quieto hasta que pones `bursts` o `beat` (un golpe lo mantiene `length` segundos).

Parámetros: `bursts` (número, por defecto 0; de 0 a 10); `beat` (sí/no, por defecto no); `length` (número, por defecto 0,2; de 0,02 a 2); `rows` (número, por defecto 24; de 1 a 256); `threshold` (número, por defecto 0,7; de 0 a 1); `shift` (número, por defecto 24; de 0 a 512); `split` (número, por defecto 4; de 0 a 64); `jitter` (número, por defecto 2; de 0 a 64); `speed` (número, por defecto 12; de 0 a 60).

**`scroll`** (Desplazamiento continuo, ligero). Desplaza la imagen y la repite en los bordes (o no, con `repeat_x` y `repeat_y` apagados).

Parámetros: `speed` (`[x, y]`, por defecto `[0.01, 0]`; de -4 a 4); `repeat_x` (sí/no, por defecto sí); `repeat_y` (sí/no, por defecto sí).

**`shake`** (Sacudida, ligero). Sacude la imagen a lo largo de una dirección o de un mapa de flujo pintado, con ruido o con un empuje periódico.

Parámetros: `strength` (número, por defecto 0; de 0 a 256); `speed` (número, por defecto 2; de 0 a 60); `direction` (`[x, y]`, por defecto `[1, 1]`; de -1 a 1); `noise` (número, por defecto 0; de 0 a 1); `friction` (número, por defecto 0; de 0 a 1); `bounds` (`[x, y]`, por defecto `[1, 1]`; de 0 a 1); `phase` (imagen); `flow` (imagen); `motion` (opción, por defecto 0): 0 ruido, 1 empuje periódico; `side` (opción, por defecto 0): 0 a los dos lados, 1 solo hacia delante, 2 solo hacia atrás. El empuje periódico es un empuje suave con el retorno que fija `friction`, no el diente de sierra exacto del efecto de paquete `flow_shake`; al migrar un paquete puedes conservar su efecto.

**`spin`** (Giro, ligero). Gira la parte de la imagen cercana a un centro, con menos fuerza hacia `radius`.

Parámetros: `center` (punto, por defecto `[0.5, 0.5]`); `angle` (número, por defecto 0; de -360 a 360); `speed` (número, por defecto 0; de -720 a 720); `radius` (número, por defecto 0,5; de 0 a 2).

**`sway`** (Vaivén, ligero). Balancea la imagen como algo colgado, o con `foliage` hace temblar las hojas.

Parámetros: `amplitude` (número, por defecto 6; de 0 a 256); `speed` (número, por defecto 0,4; de 0 a 10); `stiffness` (número, por defecto 2; de 0 a 8); `spread` (número, por defecto 1,5; de 0 a 64); `swing` (sí/no, por defecto no); `angle` (número, por defecto 3; de 0 a 90); `pivot` (`[x, y]`, por defecto `[0.5, 0]`; de -1 a 2); `double` (sí/no, por defecto no); `phase` (imagen); `foliage` (sí/no, por defecto no); `ratio` (número, por defecto 1; de 0,1 a 10).

**`trail`** (Estela de movimiento, medio). Deja copias que se desvanecen de lo que dibujaron los efectos anteriores, igual a cualquier ritmo de fotogramas.

Parámetros: `persistence` (número, por defecto 0,8; de 0 a 0,99).

**`water_rings`** (Anillos en el agua, ligero). Anillos que se expanden, como gotas en el agua.

Parámetros: `amplitude` (número, por defecto 4; de 0 a 128); `wavelength` (número, por defecto 48; de 2 a 4096); `speed` (número, por defecto 32; de -1024 a 1024); `falloff` (número, por defecto 400; de 1 a 16384); `center` (`[x, y]`, por defecto `[0.5, 0.5]`; de -1 a 2).

**`water_ripple`** (Ondulaciones del agua, medio). Ondas de agua que reaccionan al puntero y a los clics.

Parámetros: `strength` (número, por defecto 32; de 0 a 256); `damping` (número, por defecto 0,96; de 0,5 a 0,97); `radius` (número, por defecto 24; de 1 a 256); `cursor` (número, por defecto 1; de 0 a 8); `click` (número, por defecto 1; de 0 a 8); `beat` (número, por defecto 0; de 0 a 8); `specular` (número, por defecto 0,15; de 0 a 2); `perspective` (número, por defecto 0; de 0 a 1); `reflection` (imagen); `tint` (color, por defecto `#ffffff`); `walls` (imagen); `speed` (número, por defecto 1; de 0 a 4).

**`waves`** (Ondas, ligero). Distorsiona la imagen con una o dos ondas.

Parámetros: `amplitude` (número, por defecto 8; de 0 a 256); `frequency` (número, por defecto 4; de 0 a 64); `speed` (número, por defecto 1; de -20 a 20); `direction` (número, por defecto 0; de -360 a 360); `count` (entero, por defecto 1; de 1 a 2); `angle2` (número, por defecto 90; de -360 a 360); `perspective` (número, por defecto 0; de 0 a 1); `phase` (imagen).

### Luz

**`bloom`** (Resplandor, medio). Hace brillar hacia fuera las partes claras. Con `tint` tiñe el resplandor.

Parámetros: `threshold` (número, por defecto 0,8; de 0 a 4); `intensity` (número, por defecto 1; de 0 a 8); `radius` (número, por defecto 16; de 0 a 128); `knee` (número, por defecto 0,5; de 0 a 1); `tint` (color, por defecto `#ffffff`).

**`god_rays`** (Rayos de luz, medio). Rayos de luz que salen de una fuente brillante.

Parámetros: `source` (`[x, y]`, por defecto `[0.5, 0.15]`; de -1 a 2); `threshold` (número, por defecto 0,7; de 0 a 4); `density` (número, por defecto 0,8; de 0 a 1); `decay` (número, por defecto 0,96; de 0,5 a 1); `exposure` (número, por defecto 0,6; de 0 a 4); `color` (color, por defecto `#fff4e0`); `samples` (número, por defecto 12; de 4 a 16); `follow` (sí/no, por defecto no); `mode` (opción, por defecto 0): 0 el mismo corrimiento alejándose del centro, 1 el mismo corrimiento según `angle`, 2 estrella; `angle` (número, por defecto 90; de -360 a 360); `noise` (número, por defecto 0; de 0 a 1); `edges` (entero, por defecto 6; de 2 a 16).

**`iris`** (Iris, ligero). Oscurece fuera de un círculo que sigue al puntero, como un foco. Necesita el puntero.

Parámetros: `range` (número, por defecto 12; de 0 a 512); `follow` (número, por defecto 0,05; de 0 a 1); `center` (`[x, y]`, por defecto `[0.5, 0.5]`; de -1 a 2); `saccades` (sí/no, por defecto sí); `rate` (número, por defecto 1; de 0,1 a 10); `background` (sí/no, por defecto no).

**`lens_flare`** (Destello de lente, medio). Un núcleo, un halo y reflejos a lo largo de la línea que va de una luz al centro.

Parámetros: `source` (punto, por defecto `[0.3, 0.3]`); `follow` (sí/no, por defecto no); `intensity` (número, por defecto 1; de 0 a 4); `ghosts` (entero, por defecto 4; de 0 a 8); `halo` (número, por defecto 0,3; de 0 a 1); `color` (color, por defecto `#ffffff`).

**`light_shafts`** (Haces de luz, ligero). Haces de luz suaves y de color hechos con ruido, que añaden luz (se ven también sobre una capa `solid` transparente).

Parámetros: `direction` (opción, por defecto 0): 0 lineal, 1 el mismo corrimiento alejándose del centro, 2 desde la esquina; `angle` (número, por defecto 30; de -360 a 360); `center` (punto, por defecto `[0.5, 0]`); `color` (color, por defecto `#fff2cc`); `gradient` (imagen); `exponent` (número, por defecto 1; de 0,1 a 8); `intensity` (número, por defecto 1; de 0 a 4); `feather` (número, por defecto 0,3; de 0 a 1); `smoothness` (número, por defecto 0,5; de 0 a 1); `speed` (número, por defecto 0,2; de -8 a 8).

**`reflection`** (Reflejo, ligero). Un reflejo ondulado bajo la imagen.

Parámetros: `line` (número, por defecto 0,6; de 0 a 1); `opacity` (número, por defecto 0,6; de 0 a 1); `fade` (número, por defecto 1; de 0 a 1); `amplitude` (número, por defecto 2; de 0 a 128); `frequency` (número, por defecto 12; de 0 a 512); `speed` (número, por defecto 0,5; de -20 a 20); `angle` (número, por defecto 0; de -360 a 360).

**`shine`** (Destello, ligero). Pasa una banda de brillo, un anillo o un destello por la imagen.

Parámetros: `angle` (número, por defecto 30; de -360 a 360); `width` (número, por defecto 0,12; de 0 a 1); `softness` (número, por defecto 0,6; de 0 a 1); `intensity` (número, por defecto 0,4; de 0 a 4); `color` (color, por defecto `#ffffff`); `speed` (número, por defecto 0,25; de -4 a 4); `gap` (número, por defecto 1; de 0 a 8); `phase` (número, por defecto 0; de 0 a 1); `blend` (modo de mezcla, por defecto `add`); `gradient` (imagen); `style` (opción, por defecto 0): 0 línea, 1 doble línea, 2 anillo, 3 destello.

**`vignette`** (Viñeta, ligero). Oscurece las esquinas.

Parámetros: `intensity` (número, por defecto 0,35; de 0 a 1); `radius` (número, por defecto 1; de 0 a 2); `softness` (número, por defecto 0,65; de 0 a 1); `roundness` (número, por defecto 1; de 0 a 1); `center` (`[x, y]`, por defecto `[0.5, 0.5]`; de -1 a 2); `color` (color, por defecto `#000000`).

### Color

**`chroma_key`** (Croma, ligero). Quita un color y lo hace transparente, como una pantalla verde.

Parámetros: `key` (color, por defecto `#00ff00`); `tolerance` (número, por defecto 0,1; de 0 a 1); `softness` (número, por defecto 0,08; de 0 a 1); `spill` (número, por defecto 0,5; de 0 a 1).

**`color_adjust`** (Ajustes de color, ligero). Ajusta exposición, contraste, saturación y tono.

Parámetros: `exposure` (número, por defecto 0; de -5 a 5); `contrast` (número, por defecto 1; de 0 a 4); `saturation` (número, por defecto 1; de 0 a 4); `hue` (número, por defecto 0; de -180 a 180).

**`color_lut`** (Etalonaje, ligero). Etalonaje de color con una tabla `.cube`.

Parámetros: `lut` (tabla); `intensity` (número, por defecto 1; de 0 a 1).

**`posterize`** (Posterizar, ligero). Reduce la imagen a pocos niveles de color.

Parámetros: `levels` (número, por defecto 6; de 2 a 64); `dither` (número, por defecto 0,5; de 0 a 1).

**`pulse`** (Pulso, ligero). Hace latir el brillo (0), el color (1), la opacidad (2) o una luz teñida (3), con el tiempo, con ruido o con una banda del sonido (`audio_band`, de 1 a 64; necesita la entrada de audio solo entonces).

Parámetros: `what` (opción, por defecto 0): 0 brillo, 1 color, 2 opacidad, 3 luz teñida; `speed` (número, por defecto 1; de 0 a 16); `bounds` (`[x, y]`, por defecto `[0, 1]`; de 0 a 4); `tint_low` (color, por defecto `#000000`); `tint_high` (color, por defecto `#ffffff`); `noise` (número, por defecto 0; de 0 a 1); `noise_speed` (número, por defecto 1; de 0 a 16); `phase` (imagen); `audio_band` (entero, por defecto 0; de 0 a 64).

**`tint`** (Tinte, ligero). Tiñe la imagen con un color (modo 0) o la colorea (modo 1: el color toma el brillo de cada punto).

Parámetros: `color` (color, por defecto `#ff8800`); `blend` (modo de mezcla, por defecto `multiply`); `amount` (número, por defecto 1; de 0 a 1); `mode` (opción, por defecto 0): 0 tinte, 1 colorear.

### Desenfoque

**`blur`** (Desenfoque, medio). Desenfoca la imagen con un radio fijo. `blend` y `amount` mezclan la imagen desenfocada con la nítida.

Parámetros: `radius` (número, por defecto 8; de 0 a 128); `blend` (modo de mezcla, por defecto `normal`); `amount` (número, por defecto 1; de 0 a 1).

**`blur_motion`** (Desenfoque de movimiento, medio). Desenfoca en una dirección, como si se moviera rápido. Con `follow` lo hace según el movimiento real de la capa, en píxeles por segundo por `shutter`, hasta `length`.

Parámetros: `angle` (número, por defecto 0; de -360 a 360); `length` (número, por defecto 16; de 0 a 256); `samples` (número, por defecto 12; de 2 a 32); `follow` (sí/no, por defecto no); `shutter` (número, por defecto 0,05; de 0 a 0,5).

**`blur_radial`** (Desenfoque radial, medio). Desenfoca hacia fuera desde un centro, como un zoom.

Parámetros: `center` (`[x, y]`, por defecto `[0.5, 0.5]`; de -1 a 2); `strength` (número, por defecto 0,1; de 0 a 1); `samples` (número, por defecto 8; de 2 a 16); `follow` (sí/no, por defecto no).

**`local_contrast`** (Contraste local, medio). Da contraste alrededor de los bordes sin cambiar la exposición general; conviene marcarlo `optional`.

Parámetros: `radius` (número, por defecto 8; de 1 a 64); `amount` (número, por defecto 0,5; de 0 a 4); `luma_only` (sí/no, por defecto sí).

**`tilt_shift`** (Efecto miniatura, medio). Desenfoca fuera de una franja enfocada, como una foto en miniatura; con `depth` desenfoca según una imagen de profundidad.

Parámetros: `focus` (número, por defecto 0,5; de 0 a 1); `width` (número, por defecto 0,15; de 0 a 1); `softness` (número, por defecto 0,25; de 0 a 1); `angle` (número, por defecto 0; de -360 a 360); `radius` (número, por defecto 12; de 0 a 64); `depth` (imagen).

### Distorsión

**`chromatic_aberration`** (Aberración cromática, ligero). Separa los canales de color cerca de los bordes, como una lente barata.

Parámetros: `amount` (número, por defecto 3; de -64 a 64); `center` (`[x, y]`, por defecto `[0.5, 0.5]`; de -1 a 2); `mode` (opción, por defecto 0): 0 crece con la distancia al centro, 1 el mismo corrimiento según `angle`, 2 el mismo corrimiento alejándose del centro, 3 crece con el cuadrado de la distancia; `angle` (número, por defecto 0; de -360 a 360).

**`depth_parallax`** (Paralaje de profundidad, medio). Desplaza cada punto según un mapa de profundidad, hacia el puntero (`follow`) o en una órbita lenta (`orbit`), para un efecto 3D. `depth` es un recurso `linear` donde lo claro está cerca. Mientras no moviera nada no cuesta ninguna pasada.

Parámetros: `depth` (imagen); `strength` (número, por defecto 24; de 0 a 256); `focus` (número, por defecto 0,5; de 0 a 1); `steps` (número, por defecto 16; de 8 a 24); `overscan` (número, por defecto 1; de 1 a 1,25); `follow` (sí/no, por defecto sí); `orbit` (sí/no, por defecto no); `scale` (`[x, y]`, por defecto `[1, 1]`; de -1 a 1); `smooth` (sí/no, por defecto no).

**`displace`** (Desplazar, ligero). Desplaza píxeles con un mapa de normales (modo 0) o de desplazamientos (modo 1), o con ruido si no das mapa.

Parámetros: `map` (imagen); `mode` (opción, por defecto 0): 0 normales, 1 desplazamientos; `scale` (número, por defecto 1; de 0,1 a 16); `strength` (número, por defecto 8; de 0 a 256); `scroll` (`[x, y]`, por defecto `[0, 0]`; de -4 a 4); `ratio` (número, por defecto 1; de 0,1 a 10).

**`lens_distort`** (Distorsión de lente, ligero). Distorsión de barril o de cojín, como una lente.

Parámetros: `k1` (número, por defecto 0,1; de -1 a 1); `k2` (número, por defecto 0; de -1 a 1); `fit` (sí/no, por defecto sí); `center` (`[x, y]`, por defecto `[0.5, 0.5]`; de -1 a 2); `background` (sí/no, por defecto no).

**`perspective`** (Perspectiva, ligero). Deforma las cuatro esquinas de la imagen hacia otro cuadrilátero.

Parámetros: `top_left` (`[x, y]`, por defecto `[0.04, 0]`; de -2 a 3); `top_right` (`[x, y]`, por defecto `[0.96, 0]`; de -2 a 3); `bottom_right` (`[x, y]`, por defecto `[1, 1]`; de -2 a 3); `bottom_left` (`[x, y]`, por defecto `[0, 1]`; de -2 a 3).

**`pixelate`** (Pixelar, ligero). Convierte la imagen en grandes píxeles cuadrados o hexagonales.

Parámetros: `size` (número, por defecto 8; de 1 a 256); `hex` (sí/no, por defecto no).

**`twirl`** (Remolino, ligero). Retuerce la imagen alrededor de un centro.

Parámetros: `angle` (número, por defecto 60; de -1080 a 1080); `radius` (número, por defecto 256; de 0 a 16384); `center` (`[x, y]`, por defecto `[0.5, 0.5]`; de -1 a 2); `aspect` (número, por defecto 1; de 0,1 a 10); `inner` (número, por defecto 0; de 0 a 1); `noise` (número, por defecto 0; de 0 a 1); `repeat` (sí/no, por defecto no).

### Estilo

**`crt`** (Pantalla CRT, ligero). Pantalla antigua de tubo, con líneas de barrido y curvatura.

Parámetros: `curvature` (número, por defecto 0,1; de 0 a 1); `pitch` (número, por defecto 4; de 2 a 64); `scan` (número, por defecto 0,3; de 0 a 1); `phosphor` (número, por defecto 0,15; de 0 a 1); `vignette` (número, por defecto 0,25; de 0 a 1); `glow` (número, por defecto 0,15; de 0 a 1).

**`edges`** (Bordes, medio). Dibuja los bordes, con un aspecto de boceto opcional.

Parámetros: `strength` (número, por defecto 1; de 0 a 1); `threshold` (número, por defecto 0,25; de 0 a 4); `width` (número, por defecto 1; de 0,5 a 8); `color` (color, por defecto `#000000`); `sketch` (sí/no, por defecto no); `paper` (color, por defecto `#ffffff`).

**`film_grain`** (Grano de película, ligero). Grano de película; solo cambia con el tiempo si `fps` es mayor que 0.

Parámetros: `amount` (número, por defecto 0,05; de 0 a 1); `size` (número, por defecto 1,5; de 0,5 a 16); `fps` (número, por defecto 0; de 0 a 60); `mono` (sí/no, por defecto no); `bias` (número, por defecto 0; de -1 a 1); `blend` (modo de mezcla, por defecto `overlay`).

**`glitter`** (Destellos, ligero). Chispas pequeñas con forma de estrella en una cuadrícula.

Parámetros: `density` (número, por defecto 0,3; de 0 a 1); `size` (número, por defecto 2; de 0,5 a 16); `speed` (número, por defecto 1; de 0 a 8); `color` (color, por defecto `#ffffff`).

**`vhs`** (VHS, ligero). Cinta de vídeo gastada: línea de seguimiento que rueda, filas rotas, nieve, rayas y colores corridos.

Parámetros: `strength` (número, por defecto 0,5; de 0 a 2); `speed` (número, por defecto 1; de 0 a 8); `noise` (número, por defecto 0,3; de 0 a 1); `aberration` (número, por defecto 2; de 0 a 16); `band` (número, por defecto 1; de 0 a 1); `artifacts` (número, por defecto 0; de 0 a 3); `invert` (sí/no, por defecto no).

### Mezcla

**`blend_gradient`** (Mezcla por degradado, ligero). Mezcla otra imagen con un mapa de degradado que dice dónde se ve; sin mapa, de izquierda a derecha.

Parámetros: `texture` (imagen); `map` (imagen); `progress` (número, por defecto 0,5; de 0 a 1); `softness` (número, por defecto 0,1; de 0 a 1).

**`blend_texture`** (Mezclar textura, ligero). Pone una imagen encima de la capa con un modo de mezcla. Puedes encadenar varios; nunca hace la capa más opaca.

Parámetros: `texture` (imagen); `blend` (modo de mezcla, por defecto `multiply`); `amount` (número, por defecto 1; de 0 a 1); `offset` (`[x, y]`, por defecto `[0, 0]`; de -2 a 2); `scale` (`[x, y]`, por defecto `[1, 1]`; de 0,01 a 16).

**`dissolve`** (Disolución, ligero). Disuelve la imagen en un patrón granulado.

Parámetros: `progress` (número, por defecto 0; de 0 a 1); `size` (número, por defecto 48; de 1 a 4096); `edge` (número, por defecto 0,06; de 0 a 1); `glow` (color, por defecto `#ffa040`).

**`reveal`** (Revelar, ligero). Enseña otra imagen a través de un círculo o una elipse que puede seguir al puntero.

Parámetros: `texture` (imagen); `shape` (opción, por defecto 0): 0 círculo, 1 elipse; `radius` (número, por defecto 0,15; de 0 a 1); `softness` (número, por defecto 0,3; de 0 a 1); `follow` (sí/no, por defecto no); `invert` (sí/no, por defecto no); `center` (punto, por defecto `[0.5, 0.5]`).

### Transformación

**`transform`** (Transformación, ligero). Mueve, escala, gira y sesga la imagen después de los efectos anteriores, dentro de su cuadro.

Parámetros: `offset` (`[x, y]`, por defecto `[0, 0]`; de -2 a 2); `scale` (`[x, y]`, por defecto `[1, 1]`; de 0,01 a 16); `angle` (número, por defecto 0; de -360 a 360); `skew` (`[x, y]`, por defecto `[0, 0]`; de -89 a 89); `pivot` (punto, por defecto `[0.5, 0.5]`).

## Escribir un efecto propio

Un paquete puede traer hasta 32 efectos propios. Cada uno son dos archivos de hasta 64 KiB: un
**descriptor** (`.json`) que dice qué parámetros tiene y cuánto cuesta, y un **sombreador**
(`.wgsl`) que dice qué hace con cada píxel. Todos los sombreadores de una escena suman hasta 256 KiB.

Se declaran en `scene.json`, con un id (`[a-z0-9_]{1,32}`, distinto de los de los efectos del
programa), y las capas los usan como cualquier otro:

```json
  "effects": {
    "duotono": { "descriptor": "effects/duotono.json", "wgsl": "effects/duotono.wgsl" },
    "niebla": { "descriptor": "effects/niebla.json", "wgsl": "effects/niebla.wgsl" }
  }
```

```json
    { "id": "fondo", "type": "image", "image": "cielo", "size": [1920, 1080],
      "effects": [{ "effect": "duotono",
        "params": { "sombra": "#12122e", "luz": "#ffcf8a", "cantidad": { "prop": "intensidad" } } },
        { "effect": "niebla", "params": { "color": "#dfe6ff", "cantidad": 0.5 } }] }
```

### El descriptor

```json
{
  "id": "duotono",
  "name": { "es": "Duotono", "en": "Duotone" },
  "mask_mode": "strength",
  "params": [
    { "id": "sombra", "type": "color", "default": "#1b1b3a" },
    { "id": "luz", "type": "color", "default": "#ffd9a0" },
    { "id": "cantidad", "type": "float", "default": 1, "min": 0, "max": 1 }
  ],
  "fuse": "color",
  "cost": { "taps": 1, "passes": 1, "scale": 1 }
}
```

| Campo | Qué hace |
|-------|----------|
| `id` | Igual que en `effects` de la escena. |
| `name` | El nombre, un texto o uno por idioma: `{ "es": "Duotono", "en": "Duotone" }`. |
| `params` | Hasta 24 parámetros: `float`, `int`, `bool`, `color`, `vec2`, `texture`, `lut`, `enum`, `blend`, `position`, `mask` y `emitters`. Cada uno con `id`, `default`, `min` y `max`. |
| `passes` | Las pasadas de dibujo. Sin `fuse`, al menos una: `{ "entry": "fs_niebla" }`. |
| `fuse` | `color` o `uv`, para efectos de una sola pasada que se pueden fundir con los vecinos. |
| `cost` | **Obligatorio**: `{ "taps", "passes", "scale" }`: lecturas de imagen por píxel, número de pasadas y tamaño de las pasadas como fracción de la capa. |
| `mask_mode` | `mix` (lo normal: mezcla con la entrada según la máscara) o `strength` (tú escalas con `mask_at(uv)`; conviene a las distorsiones). |
| `uses` | `time`, `cursor` o `audio`: lo que el efecto necesita. El programa también lo deduce de tu sombreador. |
| `reach` | Un parámetro `float`, en píxeles, en que el efecto agranda la capa. |
| `min_fps` | De 1 a 240: el ritmo con que el efecto aún se ve fluido. |
| `pace` | Cuándo cambia un efecto que depende del tiempo: `{ "rate": "<parámetro>" }`, `{ "steps": [alto, bajo] }`, `{ "sweep": [...] }` o `"outside"`. |
| `velocity` | Un parámetro `bool` que, activado, da al sombreador la velocidad de la capa. |
| `state` | `{ "scale", "settle", "fade" }`: texturas que se guardan entre fotogramas, para simulaciones. |
| `targets` | Hasta 8 texturas con nombre `{ "scale", "format" }`, para encadenar pasadas. |
| `options` | Hasta 8 variantes del sombreador (un `override` de WGSL), compiladas al cargar la escena. |
| `compose`, `solve`, `bounded`, `quality` | Opciones avanzadas: dibujar la capa sobre lo de abajo primero, resolver una homografía, limitar el trabajo a la parte iluminada de una máscara, y cambiar parámetros por nivel de calidad. |
| `stage` | `material` o `particle`, para sombreadores de un material 3D o de un dibujo de partículas (ver [3d.md](3d.md) y [particulas-y-sonido.md](particulas-y-sonido.md)). |

Un descriptor de efecto propio no admite `category` ni `description`: son solo del catálogo del
programa, y escribirlos es un error.

Cada pasada puede tener `entry`, `scale` (fracción del tamaño de la capa), `inputs` (por defecto
`["previous"]`: `previous`, `input`, `source`, `mask`, `pass:N`, `state`, `backdrop`,
`asset:<parámetro>` o `target:<nombre>`), `output`, `target`, `blend` (`replace`, `normal`, `add`,
`multiply`), `copy` y `swap`. Un efecto tiene hasta 16 pasadas.

### El sombreador

Un efecto fundido (`fuse`) define una función cuyo nombre lleva el id del efecto:

- con `"fuse": "color"`: `fn color_<id>(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32>`, sobre
  colores premultiplicados por el alfa;
- con `"fuse": "uv"`: `fn uv_<id>(uv: vec2<f32>) -> vec2<f32>`, que dice de dónde lee cada punto.

```wgsl
fn color_duotono(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let c = straight(color);
    let luma = dot(c.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    let toned = mix(from_linear(param(0u).rgb), from_linear(param(1u).rgb), luma);
    let rgb = mix(c.rgb, toned, param(2u).x * mask_at(uv));
    return vec4<f32>(rgb * c.a, c.a);
}
```

Un efecto con pasadas define una función por pasada:

```wgsl
@fragment
fn fs_niebla(in: VertexOut) -> @location(0) vec4<f32> {
    let c = tap(in.uv);
    let haze = from_linear(param(0u).rgb) * c.a;
    let amount = param(1u).x * mask_at(in.uv) * in.uv.y;
    return vec4<f32>(mix(c.rgb, haze, amount), c.a);
}
```

```json
{
  "id": "niebla",
  "name": { "es": "Niebla", "en": "Haze" },
  "mask_mode": "strength",
  "params": [
    { "id": "color", "type": "color", "default": "#c8d4f0" },
    { "id": "cantidad", "type": "float", "default": 0.4, "min": 0, "max": 1 }
  ],
  "passes": [{ "entry": "fs_niebla" }],
  "cost": { "taps": 1, "passes": 1, "scale": 1 }
}
```

Las coordenadas van de (0, 0) arriba a la izquierda a (1, 1) abajo a la derecha. Lo que el
sombreador puede usar:

| Nombre | Qué es |
|--------|--------|
| `param(n)` | El parámetro número *n* (desde `0u`), en `.x`, `.xy` o `.rgb`. |
| `tap(uv)`, `sample(t, uv)` | Leer la imagen de entrada, u otra textura. |
| `mask_at(uv)` | La máscara del uso, de 0 a 1. |
| `px(píxeles)` | Píxeles convertidos al espacio de coordenadas. |
| `straight(c)` | Quita el premultiplicado por alfa. |
| `to_linear`, `from_linear` | Pasan entre los valores del efecto y la luz lineal según `canvas.blend_space`. |
| `frame.time`, `frame.dt` | El tiempo y lo que ha pasado desde el último fotograma. |
| `frame.cursor`, `frame.inside`, `frame.buttons` | El puntero. |
| `frame.bass`, `mid`, `treble`, `level`, `beat`, `band(i)` | El sonido. |
| `layer_uv(uv)`, `layer_extent()` | Dónde cae el punto dentro de la capa, y su tamaño. |
| `oklab`, `from_oklab`, `gauss`, `hash`, `hash2`, `noise1`, `value_noise`, `gradient_noise`, `fbm(p, octaves)`, `bayer(pixel)` | Ayudas de color y de ruido. |

Un efecto no puede declarar variables de enlace propias: la imagen de entrada, la máscara y hasta
ocho imágenes más llegan por `inputs` y `params`.

Reglas del sombreador: cada bucle tiene que contar con una variable entera desde un literal hasta
un límite literal o una constante (`for (var i = 0u; i < 16u; i++)`), y como mucho 1 024 vueltas;
no valen `while`. Se revisa el trabajo que puede hacer antes de llegar al controlador gráfico.
Si algo falla, el programa dice qué: `/effects/duotono: ...` con la línea y la columna del error.

### Probarlo

```
wprs-ctl scene import-effect MI-CARPETA efecto.json
wprs-ctl scene validate MI-CARPETA
```

`import-effect` copia el descriptor y el `.wgsl` que está junto a él a la carpeta y los
comprueba como lo hace quien abre un paquete. Con `wprs-ctl scene build` y `scene bench` mides su
costo. En el editor, *Efecto nuevo* parte de una plantilla y enseña los errores en el código.

### Otros sombreadores

Con `"stage": "material"`, el sombreador define `fn material(s: Surface) -> Surface` y da forma al
material de un modelo (ver [3d.md](3d.md)); con `"stage": "particle"`, define
`fn particle(p: Sprite) -> Sprite` y cambia cómo se dibuja cada partícula (ver
[particulas-y-sonido.md](particulas-y-sonido.md)).

### Una transición propia

Un fondo puede traer su propia transición para cuando entra y, si quiere, otra para cuando sale.
Se declara en `scene.json`, bajo `transition`, con `in` y `out`; cada una nombra su descriptor
(`"stage": "transition"`) y su `.wgsl`, dura de 100 a 10000 ms (`duration_ms`), lleva una curva
(`curve`, los mismos nombres que las claves) y valores fijos para sus parámetros (`params`). El
sombreador define `fn transition(c: Crossing) -> Crossing`: `before(uv)` es la imagen que se
estaba viendo, `after(uv)` la que entra y `c.progress` va de 0 a 1; el color que deja se dibuja
opaco. Mira [`ejemplos/transicion-propia`](ejemplos/transicion-propia/scene.json).

Una transición no puede usar imágenes, tablas de color, máscaras ni emisores, y cuesta a lo sumo
8 unidades: una liviana se ve en todos los niveles de calidad y una mediana solo en los niveles 2
y 3. Si no pasa las comprobaciones, la escena igual se abre: `wprs-ctl pack check` dice por qué y
el fondo entra con el fundido de siempre. Lo que el usuario elige manda: sin transición, el cambio
es directo, y con movimiento reducido es un fundido. Al pasar de un fondo con `out` a uno con `in`,
primero se ve la salida y después la entrada. En el teléfono, por ahora, entra con su fundido.

## No cubierto aquí

- Efectos de las partículas y de los materiales 3D: [particulas-y-sonido.md](particulas-y-sonido.md)
  y [3d.md](3d.md).
- El editor de efectos, el pincel de máscaras y los editores de rampas: [editor.md](editor.md).
