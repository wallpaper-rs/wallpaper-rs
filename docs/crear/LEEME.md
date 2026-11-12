# Crear escenas

Una **escena** es un fondo de pantalla animado que tú dibujas con capas: imágenes, formas, texto,
partículas, luces, modelos 3D, sonido. Se mueve con el tiempo, con el sonido que suena, con el
puntero o con lo que el usuario elija en sus ajustes. Esta guía enseña a hacer una desde cero,
sin saber nada del programa.

Una escena es una carpeta con tres cosas:

```
mi-escena/
  manifest.json   el nombre, el autor, la licencia y las propiedades de usuario
  scene.json      las capas y todo lo que se mueve
  assets/         las imágenes, sonidos, fuentes y modelos
```

Los dos archivos son de texto (JSON) y los puedes escribir a mano. Hay tres formas de hacer la
misma escena, y puedes mezclarlas: las tres cambian los mismos archivos.

- **A mano**, con un editor de texto. `wprs-ctl scene validate` dice qué está mal y dónde.
- **Con el editor de la app**: la página *Escenas* de `wprs-app`. Es el camino más cómodo para
  colocar capas con el ratón y mover valores en una línea de tiempo.
- **Con la terminal**: las órdenes `wprs-ctl scene ...`. Sirven para hacer en un guion lo mismo
  que hace el editor.

## Qué necesitas

- El programa instalado: la app (`wprs-app`), el servicio que dibuja los fondos y la orden
  `wprs-ctl`.
- Una imagen tuya, PNG, JPEG o WebP, para el tutorial (aquí se llama `cielo.png`).
- Un editor de texto, si vas a escribir a mano.

La mayoría de las órdenes de `wprs-ctl scene` no necesitan el servicio en marcha: crean,
cambian, comprueban y miden la escena. Solo la vista previa en el escritorio lo necesita.

## Las páginas de esta guía

| Página | De qué trata |
|--------|--------------|
| [LEEME.md](LEEME.md) | Esta página: qué es una escena y tu primera escena. |
| [escena.md](escena.md) | El archivo entero: lienzo, reloj, recursos, costo y calidad, valores que se mueven, entradas. |
| [capas.md](capas.md) | Cada tipo de capa, con ejemplo y tabla de campos. |
| [efectos.md](efectos.md) | Los efectos que trae el programa, las máscaras y cómo escribir un efecto propio. |
| [particulas-y-sonido.md](particulas-y-sonido.md) | Sistemas de partículas, sonido y reacción al sonido. |
| [propiedades.md](propiedades.md) | Propiedades de usuario, preajustes y traducciones. |
| [3d.md](3d.md) | Luces, modelos 3D, sombras, cámara y profundidad. |
| [mallas.md](mallas.md) | Imágenes dobladas por un esqueleto, clips, física y cinemática inversa. |
| [editor.md](editor.md) | El editor de la app, herramienta por herramienta. |
| [acciones.md](acciones.md) | Las acciones del editor, que se invocan con `wprs-ctl scene act`. |
| [scripts.md](scripts.md) | Código JavaScript para lo que no se puede describir con valores. |
| [paquete.md](paquete.md) | El manifiesto, firmar, empaquetar y publicar. |
| [cli.md](cli.md) | Las órdenes de `wprs-ctl` para crear, medir y publicar. |

Todos los ejemplos completos están en la carpeta [`ejemplos/`](ejemplos/), una carpeta de escena
por ejemplo. Cada una se comprueba con `wprs-ctl scene validate` y puedes abrirla en el editor o
copiarla y cambiarla.

| Ejemplo | Qué enseña |
|---------|------------|
| [`primera-escena`](ejemplos/primera-escena/) | El tutorial de abajo. |
| [`lienzo-y-reloj`](ejemplos/lienzo-y-reloj/) | Lienzo, paralaje, resplandor, cámara, reloj, hora y fecha. |
| [`formas`](ejemplos/formas/), [`texto`](ejemplos/texto/), [`video`](ejemplos/video/), [`grupos-y-mascaras`](ejemplos/grupos-y-mascaras/) | Las capas. |
| [`animacion-e-interaccion`](ejemplos/animacion-e-interaccion/) | Hojas de dibujos, clics y sonido que salta con un evento. |
| [`valores`](ejemplos/valores/) | Valores que siguen el sonido, el puntero, el reloj y claves. |
| [`efectos-apilados`](ejemplos/efectos-apilados/), [`efecto-propio`](ejemplos/efecto-propio/) | Efectos, máscaras y efectos propios. |
| [`particulas`](ejemplos/particulas/), [`particulas-avanzadas`](ejemplos/particulas-avanzadas/), [`sonido`](ejemplos/sonido/) | Partículas y sonido. |
| [`propiedades`](ejemplos/propiedades/) | Todos los tipos de propiedad de usuario, preajustes y traducciones. |
| [`luces`](ejemplos/luces/), [`modelo-3d`](ejemplos/modelo-3d/), [`profundidad`](ejemplos/profundidad/), [`malla-con-huesos`](ejemplos/malla-con-huesos/) | Luces, 3D, profundidad y huesos. |

## Tu primera escena

Vas a hacer una escena con una imagen de fondo que ondula suavemente, un sol que cruza la pantalla
y un ajuste para que el usuario elija el color del sol. Para cada paso hay dos caminos: la orden
de `wprs-ctl` y dónde está en el editor. El resultado final es la carpeta
[`ejemplos/primera-escena`](ejemplos/primera-escena/).

Pon `cielo.png` (cualquier imagen tuya) en la carpeta donde vas a trabajar.

### 1. Crear la escena

```
wprs-ctl scene new mi-escena --title "Mi primera escena"
```

*Editor:* página *Escenas*, escribe el título, elige la carpeta y pulsa el punto de partida *Vacía*.

Se crea la carpeta `mi-escena` con un `manifest.json`, una `scene.json` sin capas y una carpeta
`assets/`. El lienzo mide 1920 por 1080: es el tamaño de referencia con el que colocas todo, y
el programa lo ajusta a cada pantalla. El paquete recibe el id `org.example.mi-escena` (el nombre de
la carpeta); es el que usan órdenes como `input grant` y conviene cambiarlo antes de publicar.

### 2. Una capa de imagen

```
wprs-ctl scene add-layer mi-escena image --file cielo.png --name Fondo
wprs-ctl scene layer mi-escena fondo transform --size 1920,1080
```

*Editor:* arrastra `cielo.png` a la lista *Capas* (dice «Suelta los archivos para añadirlos como
capas»); luego, en el inspector de la capa, pon el tamaño en 1920 por 1080.

La primera orden copia la imagen a `assets/` como recurso `cielo`, añade una capa que la enseña e
imprime su id: `fondo`. Sin `size`, una imagen se ve de su tamaño real y centrada, y casi nunca es
el del lienzo: con `size` se estira hasta llenarlo, y hay que ponerlo en cada capa de imagen. Cada
capa tiene un id, hecho del nombre, que las demás órdenes usan para nombrarla. Capas, recursos,
sistemas de partículas y líneas de tiempo comparten los ids, así que, si el recurso ya se llamaba
como la capa (`--file fondo.png --name Fondo`), la capa se llama `fondo_2`: lee siempre el id que
imprime la orden (`cli.md`, «`scene add-layer`»).

### 3. Una capa que se mueve

Primero una forma, un círculo amarillo:

```
wprs-ctl scene add-layer mi-escena shape --name Sol
wprs-ctl scene set mi-escena /scene/layers/1/shape '{"ellipse": {"size": [220, 220]}}'
```

*Editor:* *Añadir una capa*, tipo *Forma*; en el inspector, elige la forma *Elipse* y su tamaño.

Después una línea de tiempo que dure 12 segundos y vaya y vuelva, y dos fotogramas clave de la
posición del sol, uno al principio y otro al final:

```
wprs-ctl scene timeline mi-escena vuelta --add --duration 12 --loop pingpong
wprs-ctl scene keyframe mi-escena /scene/layers/1/transform/position 0 '[260, 820]' --timeline vuelta --ease in_out_sine
wprs-ctl scene keyframe mi-escena /scene/layers/1/transform/position 12 '[1660, 260]' --timeline vuelta
```

*Editor:* en *Líneas de tiempo*, *Añadir una línea de tiempo* y ponle 12 segundos y `pingpong`;
con el cabezal en 0, pulsa el rombo de *Posición* de la capa (*Fotograma clave aquí*) y arrastra el
sol abajo a la izquierda; lleva el cabezal a 12 y arrastra el sol arriba a la derecha.

Las capas se cuentan desde 0 desde abajo, y por eso el sol, la segunda, es `/scene/layers/1`. Para ver
lo que hay en cualquier sitio: `wprs-ctl scene show mi-escena /scene/layers/1`. El puntero empieza
por `/scene` (lo de `scene.json`) o por `/manifest` (lo de `manifest.json`); `/layers` solo no existe.

La posición es un punto del lienzo en píxeles con el origen arriba a la izquierda. `pingpong`
hace que la línea de tiempo vaya y vuelva sin parar, `in_out_sine` suaviza el principio del
trayecto.

### 4. Un efecto

Un efecto cambia cómo se ve una capa. `waves` hace ondular el cielo:

```
wprs-ctl scene effects mi-escena fondo add waves --param amplitude=6 --param frequency=3
```

*Editor:* elige la capa del cielo, *Añadir un efecto*, *Ondas*; ajusta *amplitude* (amplitud) y
*frequency* (frecuencia).

Los efectos están en [efectos.md](efectos.md).

### 5. Una propiedad de usuario

Una propiedad de usuario es algo que la persona que usa tu fondo puede cambiar sin tocar nada
más. Esta deja elegir el color del sol:

```
wprs-ctl scene add-property mi-escena color --id color_sol --label "Color del sol"
wprs-ctl scene property mi-escena color_sol --default '"#ffd23f"'
wprs-ctl scene set mi-escena /scene/layers/1/fill '{"prop": "color_sol"}'
```

*Editor:* en *Propiedades de usuario*, *Añadir una propiedad* de tipo color, con su etiqueta y su
valor por defecto; en el relleno de la capa del sol, pulsa el rombo y elige *Seguir una propiedad
de usuario*.

La propiedad se declara en `manifest.json` y la capa la sigue con `{ "prop": "color_sol" }`.

### 6. Comprobar

```
wprs-ctl scene validate mi-escena
```

*Editor:* el panel *Problemas*, que se actualiza mientras trabajas.

No imprime nada si todo está bien. Si algo está mal, dice la ruta del campo y qué esperaba. Esta
orden termina con error cuando hay problemas, y por eso sirve en guiones. Si la escena tiene
scripts (`scripts.md`), también los compila sin ejecutarlos y dice, con el archivo y la línea, un
error de sintaxis o un script que falta.

### 7. Verla

```
wprs-ctl scene preview mi-escena
```

*Editor:* *Ver en el escritorio*, junto al lienzo.

Muestra la escena en el escritorio durante 60 segundos sin aplicarla como fondo, y necesita el
servicio en marcha. Sin él, puedes construir el paquete y dibujar un fotograma:

```
wprs-ctl scene build mi-escena -o mi.wpak
wprs-ctl scene bench mi.wpak --frames 360 --shot cuadro.png
```

`bench` dibuja la escena fuera de pantalla, dice cuánto cuesta cada fotograma y guarda el último
como imagen: con `--frames 360` es el instante 6 s a 60 fotogramas por segundo, cuando el sol va a
la mitad.

`bench` no ejecuta los scripts. Para ver una escena con scripts sin escritorio, y a una hora fija,
usa `wprs-ctl scene render mi-escena -o vista.png --time-of-day 15:42` (`cli.md`, «`scene render`»).
Una escena con partículas se ve vacía en el primer cuadro si su sistema no tiene `prewarm`
(`particulas-y-sonido.md`).

Para aplicarla de verdad como fondo: `wprs-ctl scene apply mi-escena`, que al final dice qué mostró y
dónde (`applied: Mi primera escena 0.1.0 on DP-1, quality Q3`), y para quitarla,
`wprs-ctl clear` (y `wprs-ctl restore` la trae de vuelta). Si cambias la escena, aplícala otra vez:
no hace falta subir la versión, pero tarda en verse mientras el demonio construye y carga.

### Y ahora

La escena que acabas de hacer es la carpeta [`ejemplos/primera-escena`](ejemplos/primera-escena/);
su `scene.json` entero es:

```json
{
  "schema": "wallpaper-rs.scene/1",
  "canvas": { "size": [1920, 1080], "fit": "fill", "background": "#000000" },
  "clock": { "max_fps": "display" },
  "assets": { "cielo": { "path": "assets/cielo.png" } },
  "layers": [
    { "id": "fondo", "type": "image", "image": "cielo", "size": [1920, 1080],
      "effects": [{ "effect": "waves", "params": { "amplitude": 6, "frequency": 3 } }] },
    { "id": "sol", "type": "shape", "shape": { "ellipse": { "size": [220, 220] } },
      "fill": { "prop": "color_sol" },
      "transform": { "position": { "timeline": "vuelta",
        "keys": [[0, [260, 820], "in_out_sine"], [12, [1660, 260]]] } } }
  ],
  "timelines": [{ "id": "vuelta", "duration": 12, "loop": "pingpong" }]
}
```

Sigue con [escena.md](escena.md) para el resto del archivo, con [capas.md](capas.md) para los
tipos de capa y con [paquete.md](paquete.md) cuando quieras dar nombre, licencia y firma a tu
trabajo y compartirlo.

## Tres cosas que conviene saber desde el principio

1. **Los colores** se escriben `"#rrggbb"` o `"#rrggbbaa"`, siempre con 6 u 8 cifras.
2. **Un valor puede moverse.** Casi cualquier número, color, texto o interruptor puede ser un valor
   fijo o un objeto con una fuente: una propiedad (`{ "prop": ... }`), fotogramas clave
   (`{ "keys": ... }`), el sonido (`{ "audio": ... }`), el puntero (`{ "cursor": ... }`) o el reloj
   (`{ "clock": ... }`). Está en [escena.md](escena.md).
3. **Las escenas corren en teléfonos.** Cada fotograma cuesta trabajo; [escena.md](escena.md) explica
   cómo estimarlo y bajarlo.
