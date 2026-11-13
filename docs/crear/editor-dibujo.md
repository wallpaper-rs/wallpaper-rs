# Dibujo en el editor: formas, texto, mapas, sprites y profundidad

Esta página explica las herramientas del editor de escenas con las que se dibuja y se da forma: las formas con su degradado, el texto con su fuente, los mapas pintados, los sprites y la profundidad de las imágenes. Las mallas con huesos tienen su página: [editor-mallas](editor-mallas.md). Cada tarea dice dónde está, los pasos, y el equivalente en la terminal.

Antes de empezar:

- La **vista previa** es el lienzo del editor, donde se ven las capas. Casi todas las herramientas de esta página dibujan sus asas y sus pinceles sobre ella.
- A la izquierda hay cinco pestañas: **Capas**, **Recursos**, **Paquete**, **Reproductor** y **Probar**. Las capas se añaden con **Añadir una capa**, en la pestaña **Capas**; los archivos se importan en **Recursos**.
- Al elegir una capa, el inspector muestra sus campos. Los nombres de campo que no tienen traducción se ven en inglés (por ejemplo "Radius", "Gradient", "Stroke"); aquí van entre comillas tal como aparecen.
- Cada trazo y cada arrastre que suelta el ratón es un paso que se puede deshacer. Cómo deshacer, los atajos y el resto de la ventana están en [editor](editor.md).
- Cada acción tiene su equivalente en la terminal con `wprs-ctl scene act CARPETA ACCIÓN JSON`; los argumentos están en [acciones](acciones.md). La lista de la terminal es `wprs-ctl scene actions`.

## Cómo añadir una forma y elegir su figura

**Dónde está:** pestaña **Capas**, botón **Añadir una capa**, entrada **Forma**.

**Pasos:**

1. Pulsa **Añadir una capa** y elige **Forma**. La capa empieza como un rectángulo.
2. Con la capa elegida, mira el inspector. El campo "Shape" es una lista con las figuras: rectángulo, elipse, anillo, polígono, estrella y línea. Cada una tiene sus propios campos (tamaño, radio, lados, puntas, grosor…).
3. El color de relleno es el campo "Fill". El contorno es el grupo "Stroke": pulsa **Añadir** para tenerlo.

**En la terminal:** `add_layer` con `"kind":"shape"`, y `set` para cambiar la figura o cualquier campo. Ver [acciones](acciones.md).

## Cómo cambiar una forma con las asas del lienzo

Mientras una capa de forma está elegida, la vista previa dibuja sus asas encima. Se mueven con el ratón y cada arrastre se guarda como un paso.

**Dónde está:** en la vista previa, sobre la forma elegida.

**Pasos:**

1. Elige la capa de forma en la lista de capas, o pulsa la forma en la vista previa.
2. Arrastra un asa. Pasa el ratón por encima para ver su ayuda.
3. Suelta. El cambio queda como un paso.

Las asas que cambian el tamaño están un poco dentro de la caja de la capa, para no tapar las agarraderas de la capa. Las cuadradas cambian medidas (esquinas y bordes); las redondas cambian un número (esquinas redondeadas, lados, puntas, profundidad de las puntas, grosor del anillo).

| Figura | Asas | Qué hace cada una |
|--------|------|--------------------|
| Rectángulo | ancho, alto, tamaño (esquina), radio | **Fijar el ancho**, **Fijar el alto**, **Cambiar el tamaño; Mayús mantiene las proporciones**, **Redondear las esquinas** |
| Elipse | ancho, alto, tamaño | las mismas, sin radio |
| Anillo | ancho, alto, tamaño, grosor | además **Fijar el grosor del anillo** |
| Polígono | tamaño, lados | arrastra el asa de lados alrededor de la figura: **Arrastra alrededor de la forma para fijar el número de lados** (de 3 a 64) |
| Estrella | tamaño, puntas, profundidad | **Arrastra alrededor de la forma para fijar el número de puntas** (de 2 a 64) y **Fijar la profundidad de las puntas** (de 0,01 a 1) |
| Línea | principio, final, grosor | **Mover el principio de la línea**, **Mover el final de la línea**; al mover un extremo, el otro se queda donde estaba |
| Cualquiera menos línea, con contorno | grosor del contorno | **Fijar el grosor del contorno** |
| Cualquiera, con degradado | principio y final del degradado | **Principio del degradado, o su centro si es radial** y **Final del degradado, o su borde si es radial** |

El radio de las esquinas no pasa de la mitad del lado menor. Las asas de contorno y de degradado solo aparecen cuando la capa tiene ese contorno o ese degradado.

**Atajo:** mantener **Mayús** mientras arrastras el asa de tamaño conserva las proporciones.

**En la terminal:** la acción `layer` con la variante `figure` y `shape`: `{"figure":{"shape":{"layer":"ID","handle":"NOMBRE","to":[X,Y]}}}`. `handle` es el nombre del asa (`width`, `height`, `size`, `radius`, `band`, `sides`, `points`, `inner`, `start`, `end`, `stroke`, `gradient_from`, `gradient_to`) y `to` es un punto del lienzo en píxeles. Con `"even":true` se conservan las proporciones. No hay una pregunta de terminal que dé la posición de las asas.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"shape","id":"caja"}'
wprs-ctl scene act mi-escena layer '{"figure":{"shape":{"layer":"caja","handle":"size","to":[1200,700]}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"shape":{"layer":"caja","handle":"radius","to":[1100,300]}}}'
```

## Cómo poner un degradado y editar sus paradas

**Dónde está:** inspector de la capa de forma, grupo "Gradient".

**Pasos:**

1. En el grupo "Gradient" pulsa **Añadir**. La forma se rellena con un degradado.
2. En la barra del degradado, pulsa un punto para añadir una parada. Hay entre 2 y 4. Arrastra una parada para moverla.
3. Debajo de la barra, cada parada tiene su posición (de 0 a 1), un botón de color y el botón **Quitar la parada**. No se puede dejar menos de 2.
4. La casilla del campo "Radial" cambia el degradado a radial. Arrastra sobre la forma sus dos asas, que tienen otro color que las demás: la primera es el centro y la segunda el borde (en uno lineal, el principio y el final).
5. Para quitar el degradado, pulsa el botón **Quitar** del grupo "Gradient".

La barra dice: **Pulsa la barra para añadir una parada, 4 como mucho, y arrastra una parada para moverla**.

**En la terminal:** `layer` con `figure` y `stop`: `make` crea el degradado con dos paradas (y `radial`), `clear` lo quita, `add`, `move`, `color` y `remove` editan las paradas. Las paradas quedan ordenadas.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"shape","id":"caja"}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"caja","stop":{"make":{"radial":false}}}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"caja","stop":{"add":{"at":0.5,"color":"#ff8800"}}}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"caja","stop":{"move":{"index":1,"at":0.3}}}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"caja","stop":{"color":{"index":0,"color":"#112233"}}}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"stop":{"layer":"caja","stop":{"remove":{"index":1}}}}}'
```

## Cómo escribir un texto con saltos de línea

**Dónde está:** **Añadir una capa** > **Texto**; luego el campo "Text" del inspector.

**Pasos:**

1. Añade la capa de texto. Empieza con su nombre como texto.
2. Escribe en el campo "Text". Es una caja de varias líneas: Intro escribe un salto de línea, que cuenta como una línea de la caja.
3. Sobre la vista previa, la caja del texto se dibuja con el tamaño real que tendrá, con la misma fuente, el mismo ajuste de línea, el mismo espaciado y los márgenes de contorno y sombra. Encima de su esquina superior izquierda se lee el tamaño y las líneas, como **800 × 120 px, 2 líneas**.

**En la terminal:** `add_layer` con `"kind":"text"` y `set` sobre el campo del texto.

## Cómo fijar el ancho del texto con el asa

**Dónde está:** vista previa, borde derecho de la caja del texto.

**Pasos:**

1. Elige la capa de texto. La caja se ve con una línea discontinua si el texto no tiene ancho fijo, y continua si lo tiene.
2. Arrastra el cuadrado del borde derecho. Ayuda: **Arrastra para ajustar el texto a un ancho**. El texto se parte en líneas a ese ancho; el borde izquierdo no se mueve.
3. Para volver a no ajustar, haz doble clic en el cuadrado. Ayuda del asa ya ajustada: **Fijar el ancho del texto; doble clic para dejar de ajustarlo**.

El asa escribe el campo "Width" del grupo "Wrap" de la capa.

**En la terminal:** `layer` con `figure` y `text_width`: `to` es el punto del lienzo donde cae el borde derecho; con `"remove":true` se quita el ajuste.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"titulo"}'
wprs-ctl scene act mi-escena layer '{"figure":{"text_width":{"layer":"titulo","to":[900,100]}}}'
wprs-ctl scene act mi-escena layer '{"figure":{"text_width":{"layer":"titulo","remove":true}}}'
```

## Cómo elegir la fuente y las fuentes de respaldo

**Dónde está:** inspector de la capa de texto, campos "Font" y "Fallback".

**Pasos:**

1. En el campo "Font", pulsa el botón con el nombre de la fuente (dice **Fuente por defecto** si no hay ninguna). Se abre una lista.
2. Escribe en **Buscar fuentes** para filtrar. Cada nombre se escribe con su propia fuente, y debajo dice de dónde viene: **En este paquete** (fuentes que importaste), **La que use el sistema para este estilo** (genéricas, como serif o monospace) o **Instalada en este equipo**.
3. Pulsa una fuente para elegirla. El botón de borrar, **Usar la fuente por defecto**, la quita.
4. En el campo "Fallback" hay la lista de fuentes de respaldo, hasta 8. Una letra que la fuente no tiene se dibuja con la primera de esta lista que la tenga. **Añadir una fuente de respaldo** abre la misma lista de selección. Cada fila tiene **Subir**, **Bajar** y **Quitar**.
5. Si una fuente de la lista ya no está en el paquete ni instalada, aparece el aviso **%1 no está en el paquete ni instalada aquí, así que se usa la fuente siguiente**, con su nombre.

Para que una fuente viaje con la escena, impórtala antes en la pestaña **Recursos** con **Importar…** (acepta archivos ttf, otf, ttc, otc y woff2).

**En la terminal:** `layer` con `figure` y `fonts`: `font` (texto vacío la quita) y `fallback` (lista de nombres).

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"titulo"}'
wprs-ctl scene act mi-escena layer '{"figure":{"fonts":{"layer":"titulo","font":"serif","fallback":["monospace"]}}}'
```

## Cómo activar la luz y elegir mapas

Una capa de imagen, de vídeo, de texto, de malla, de modelo o de partículas puede recibir la luz de las capas de luz. Sus mapas dicen cómo la recibe (rugosidad, luz propia y normales).

**Dónde está:** inspector de la capa, sección **Iluminación**.

**Pasos:**

1. Activa el interruptor **Recibir la luz de las capas de luz**.
2. Elige, en cada lista, una imagen de los recursos: **Mapa de normales**, **Imagen reflejada**, **Mapa de rugosidad** y **Mapa de luz propia**. **Sin fijar** deja el campo vacío.
3. Ajusta los deslizadores **Rugosidad**, **Reflejo**, **Resplandor** y **Fuerza**. En las partículas solo están **Resplandor** y **Fuerza**; su mapa de normales está en el sistema de partículas.
4. Si un mapa se cargó como imagen de colores, aparece un aviso: **… se lee como colores y sombrea mal; vuelve a elegirlo para leerlo como mapa.**

**En la terminal:** `lighting` con `layer`, y `off` para apagarla. Los mapas son `normal`, `env`, `roughness_map` y `emissive_map`.

## Cómo pintar la rugosidad o la luz propia sobre la vista previa

**Dónde está:** sección **Iluminación**, debajo de los deslizadores.

**Pasos:**

1. Pulsa **Pintar la rugosidad sobre la vista previa** o **Pintar la luz propia sobre la vista previa** (se enciende como interruptor).
2. Ajusta **Tamaño del pincel**, **Dureza** y **Gris**. **Gris** es el valor que se escribe: de 0 a 1.
3. Arrastra sobre la capa en la vista previa. Mientras pintas, la capa se tiñe con un mapa de calor del mapa que pintas.
4. Suelta: el trazo se guarda como un paso. Si la capa no tenía mapa propio, se crea uno con el tamaño de su imagen (hasta 512 en el lado mayor), blanco para la rugosidad y negro para la luz propia, y queda elegido en la lista.

Para quitar el pincel, vuelve a pulsar el botón.

**En la terminal:** `paint_map` (`layer`, `map` con `roughness` o `emissive`, `radius`, `hardness`, `value`, `points`); los puntos son fracciones de la capa, de 0 a 1. La pregunta `map` da el mapa de calor.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"image","id":"fondo","source":"foto"}'
wprs-ctl scene act mi-escena lighting '{"layer":"fondo","roughness":0.6,"emissive":2,"strength":0.8}'
wprs-ctl scene act mi-escena paint_map '{"layer":"fondo","map":"roughness","radius":0.1,"hardness":0.5,"value":0.8,"points":[[0.2,0.2],[0.4,0.4]]}'
wprs-ctl scene ask mi-escena map '{"layer":"fondo","map":"emissive"}'
```

## Cómo generar un mapa de normales

**Dónde está:** sección **Iluminación**, botón **Generar normales…** (solo en capas de imagen, vídeo y texto, con la luz activada).

**Pasos:**

1. Pulsa **Generar normales…**. El título dice si se genera de la imagen, de las letras o de un fotograma.
2. En una capa de vídeo, elige el fotograma con el deslizador (**Fotograma a los N s**); **Cabezal** toma el segundo en que estás.
3. Ajusta **Fuerza** (de 0 a 16) y **Desenfoque** (de 0 a 32). La ventana muestra tres imágenes: el relieve, el mapa y una esfera iluminada con él. **El relieve se ilumina desde arriba a la izquierda; el mapa se guarda como imagen del paquete.**
4. Pulsa **Generar**. El mapa queda guardado como imagen del paquete y elegido en **Mapa de normales**.

**En la terminal:** `normals` (`layer` o `asset`, `strength`, `blur`, `time`). La pregunta `normals` muestra la vista previa sin guardar nada.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"image","id":"fondo","source":"foto"}'
wprs-ctl scene act mi-escena lighting '{"layer":"fondo"}'
wprs-ctl scene ask mi-escena normals '{"layer":"fondo","strength":3,"blur":1}'
wprs-ctl scene act mi-escena normals '{"layer":"fondo","strength":3,"blur":1}'
```

Los mapas de los materiales de los modelos 3D se pintan con otra ventana: ver [editor-3d](editor-3d.md).

## Cómo cortar una hoja de sprites

**Dónde está:** pestaña **Recursos**, en la fila de cada imagen, botón **Cortar sprites…** (solo icono; la ayuda dice el nombre).

**Pasos:**

1. Importa la imagen con **Importar…** si aún no está en la escena.
2. Pulsa **Cortar sprites…** en su fila. Se abre la ventana **Cortar los sprites de NOMBRE**.
3. Para cortar en cuadrícula, rellena **Filas**, **Columnas**, **Margen**, **Separación** y **Cuadros (0 para todos)**, elige el orden (**De izquierda a derecha y de arriba abajo**, **De arriba abajo y de izquierda a derecha**, **Al revés, por filas** o **Al revés, por columnas**) y pulsa **Cortar la cuadrícula**.
4. Para dibujar un cuadro a mano, arrastra sobre la imagen. Arrastra un cuadro para moverlo y su esquina para cambiarle el tamaño. Cada cuadro lleva su número.
5. Debajo hay la tira de cuadros. Elige uno y usa **Antes** y **Después** para ordenarlo, **Quitar el cuadro** para borrarlo. **Quitar todos los cuadros** los borra todos.
6. La vista previa de la tira reproduce los cuadros: **Reproducir** y **Pausar**. **Fotogramas por segundo** fija la velocidad y **Cuadros que se reproducen** limita la animación a los primeros. Los cuadros que sobran se ven apagados.

Hasta 256 cuadros por imagen. La escena muestra esos cuadros en las capas de imagen animadas y en las partículas.

**En la terminal:** `sprites`, una de estas variantes: `grid`, `rects`, `add`, `change`, `remove`, `reorder`, `play` o `clear`. Cada rectángulo es `[x, y, ancho, alto]` en píxeles de la imagen. `import` también acepta `atlas` para una hoja de celdas iguales.

```sh
wprs-ctl scene act mi-escena import '{"file":"hoja.png"}'
wprs-ctl scene act mi-escena sprites '{"grid":{"asset":"hoja","rows":1,"columns":4}}'
wprs-ctl scene act mi-escena sprites '{"add":{"asset":"hoja","rect":[0,0,50,50]}}'
wprs-ctl scene act mi-escena sprites '{"change":{"asset":"hoja","index":0,"rect":[2,2,60,60]}}'
wprs-ctl scene act mi-escena sprites '{"reorder":{"asset":"hoja","from":0,"to":1}}'
wprs-ctl scene act mi-escena sprites '{"play":{"asset":"hoja","fps":12,"frames":3}}'
wprs-ctl scene act mi-escena sprites '{"remove":{"asset":"hoja","index":0}}'
```

## Cómo abrir las herramientas de profundidad de una imagen

Con la profundidad, una imagen se separa en capas que el paralaje mueve a distinto ritmo. Las capas se cortan por regiones: las partes conectadas de la imagen entre un salto de profundidad y otro. Así una superficie continua, como un suelo, nunca se parte ni se estira. Estas herramientas no están en el editor de escenas sino en el editor de imágenes.

**Dónde está:** en la biblioteca, abre una imagen y pulsa **Editar…**; en la columna de herramientas del editor de imágenes elige **Profundidad**.

El editor muestra entonces la imagen entera, sin girar ni recortar, para que lo que pintes coincida con el original. Las demás herramientas (recorte, giro) se ven al volver a ellas.

Los cambios son pasos del editor de imágenes: **Deshacer** y **Rehacer** (con los atajos estándar del sistema) valen para cada trazo, cada cambio de cortes y cada marca.

## Cómo generar o importar el mapa de profundidad

**Pasos:**

1. En **Profundidad**, la imagen aún no tiene mapa: blanco es cerca y negro es lejos.
2. Elige en **Ejecutar el modelo en** si va en **El dispositivo más rápido**, **El procesador** o la tarjeta gráfica.
3. Pulsa **Generar el mapa**. La primera vez pregunta **¿Descargar el modelo de profundidad?**: el mapa se calcula en este equipo y la imagen no sale de él; **Descargar** baja los archivos una sola vez, o **Instalar desde un archivo…** usa archivos que ya tengas. Si las descargas están desactivadas en los ajustes, solo queda esa segunda opción.
4. Mientras trabaja se ve el avance (**Descargando el modelo…**, **Cargando el modelo…**, **Calculando la profundidad…**, **Guardando el mapa…**) y el botón **Cancelar**.
5. Si ya tienes un mapa hecho, usa **Importar un mapa…**. **Guardar junto a la imagen** lo guarda como archivo junto a la imagen. **Quitar el mapa** lo borra.
6. Ajusta el mapa con **Nivel cercano**, **Nivel lejano**, **Curva**, **Suavizado (px)** y **Ensanchar los bordes cercanos (px)**. La casilla **Falso color: rojo cerca y azul lejos** cambia cómo se ve el mapa en la ventana.

Las herramientas siguientes aparecen en cuanto hay un mapa.

**En la terminal:** `wprs-ctl edit depth` (ver [cli](cli.md)): `generate`, `import`, `remove`, `export`, y las opciones `--near`, `--far`, `--gamma`, `--smooth`, `--dilate` sobre la imagen.

## Cómo ver el mapa sobre la imagen

**Dónde está:** **Profundidad sobre la imagen**.

**Pasos:** enciende **Mostrar el mapa de profundidad sobre la imagen**, ajusta **Opacidad** y elige con **Falso color: rojo cerca y azul lejos** si se ve en grises o en color. El mapa que se dibuja ya incluye tus trazos.

## Cómo retocar el mapa con el pincel de profundidad

**Dónde está:** **Pincel de profundidad**.

**Pasos:**

1. Pulsa **Pintar profundidad**.
2. Elige **Subir** (acerca) o **Bajar** (aleja).
3. Ajusta **Tamaño del pincel**, **Fuerza** y **Borde suave**.
4. Arrastra sobre la imagen. Se dibuja el círculo del pincel y el trazo; al soltar se guarda un paso.
5. Debajo se lee cuántos trazos hay (**N trazos**), con **Deshacer** (quita el último) y **Quitar los trazos** (quita todos).

**En la terminal:** `wprs-ctl edit depth brush ARCHIVO "X,Y;X,Y" [--raise|--lower] [--by] [--radius] [--falloff] [--clear]`. Las coordenadas son fracciones de la imagen original.

```sh
wprs-ctl edit depth brush foto.png '0.2,0.2;0.4,0.3' --raise --by 0.2 --radius 0.05 --falloff 0.5
wprs-ctl edit depth brush foto.png '0.5,0.5' --lower
wprs-ctl edit depth brush foto.png --clear
```

## Cómo cambiar los cortes de las capas

**Dónde está:** **Cortes de capa**.

La franja muestra el histograma de la profundidad, **Lejos** a la izquierda y **Cerca** a la derecha, coloreado por capa, con un asa por cada corte. Debajo dice **N capas, cortadas por la propia imagen** o **N capas, cortadas donde pusiste las asas**, seguido de **N regiones** cuando la imagen ya tiene capas. Los cortes agrupan las regiones en capas según su profundidad media.

**Pasos:**

1. Arrastra un asa para mover un corte. Mientras arrastras se ven los contornos de las capas sobre la imagen, y al soltar las capas se reconstruyen medio segundo después del último cambio.
2. Haz doble clic en un asa para quitar el corte, o pulsa en la franja para añadir uno. **Añadir un corte** lo pone en el hueco más ancho. Hay como mucho 5 cortes y cada uno está al menos a 0,01 del vecino.
3. **Quitar el corte** borra el elegido (o el último). **Volver a automático** deja que la propia imagen decida.
4. **Mostrar las capas sobre la imagen** pinta el contorno de cada capa tintado.

La ayuda de la ventana dice: **Arrastra un asa para mover un corte, haz doble clic para quitarlo y clic en la franja para añadir otro.**

Si la imagen se queda quieta, una línea en rojo dice por qué (ver más abajo).

**En la terminal:** `wprs-ctl edit depth cuts ARCHIVO 0.3,0.6` o `--auto`.

```sh
wprs-ctl edit depth cuts foto.png 0.3,0.6
wprs-ctl edit depth cuts foto.png --auto
```

## Cómo decidir qué se rellena detrás de los objetos cercanos

Cuando una capa cercana se mueve, descubre un trozo de lo que había detrás; esa zona se rellena. El pincel de relleno marca dónde.

**Dónde está:** **Pincel de relleno**.

**Pasos:**

1. Pulsa **Pintar marcas de relleno**.
2. **Rellenar** marca zonas que se rellenan siempre; **Proteger** marca zonas que nunca se rellenan (quedan transparentes en las capas de atrás).
3. Ajusta **Tamaño del pincel** y **Borde suave**. Aquí no hay fuerza: una marca vale entera donde cubre al menos la mitad del pincel.
4. Arrastra sobre la imagen. Una marca posterior pisa a la anterior donde se cruzan. Al soltar se reconstruyen las capas.
5. **N marcas** cuenta las marcas; **Quitar las marcas** las borra todas.

**En la terminal:** `wprs-ctl edit depth fill-mask ARCHIVO "X,Y;X,Y" (--refill|--protect) [--radius] [--falloff] [--clear]`.

```sh
wprs-ctl edit depth fill-mask foto.png '0.5,0.5;0.6,0.6' --protect --radius 0.08
wprs-ctl edit depth fill-mask foto.png --clear
```

## Cómo ver una capa y comparar rellenos

**Dónde está:** **Capas y relleno**.

**Pasos:**

1. Elige una capa en la lista: **Ninguno**, o **Capa 1, la más lejana**, **Capa 2**… hasta **la más cercana**. La imagen enseña esa capa sola, con su transparencia. Si el conjunto de capas no está hecho, se construye entonces, con una barra de avance y **Cancelar**.
2. **Resaltar la franja que el movimiento puede descubrir** tiñe los píxeles que se rellenaron.
3. **Comparar LaMa con el relleno simple** divide la imagen: a la izquierda el relleno de LaMa y a la derecha el **Relleno simple**. **Dónde termina LaMa** mueve la división (también se arrastra la línea). Solo está disponible si LaMa está instalado; si no, dice **LaMa no está instalado, así que no hay nada que comparar. Descárgalo en los ajustes.**

**En la terminal:** `wprs-ctl edit depth layers ARCHIVO` construye las capas, `show ARCHIVO --layer N --out PNG [--band PNG]` guarda una y `compare ARCHIVO --out PNG` guarda la comparación. En los ajustes de la aplicación están **Comprobar LaMa**, **Quitar LaMa** y **Recalcular las capas**; en la terminal, `wprs-ctl models verify` y `models remove`.

## Cómo unir, dividir y mover regiones

**Dónde está:** **Paralaje**, grupo **Regiones** (la imagen debe tener mapa de profundidad).

Una región es una parte de la imagen entre saltos de profundidad; se mueve como una pieza y va entera a una capa. La herramienta pinta cada región sobre la imagen, de azul (la más lejana) a rojo (la más cercana), con un número que es su lugar en el orden de profundidad: 1 es la más cercana.

**Pasos:**

1. Pulsa regiones sobre la imagen, o márcalas en la lista, para elegirlas; otro clic las suelta.
2. **Unir** hace una sola región con las elegidas: comparten capa y se mueven juntas. Sirve cuando una cosa se partió en trozos que no deberían separarse.
3. **Acercar** y **Alejar** mueven las regiones elegidas en profundidad, tanto como diga **Mover**.
4. Para cortar una región en dos, activa **Dividir con un trazo** y dibuja a través de ella; al soltar se corta. Cada parte puede ir a una capa distinta.
5. **Deshacer** y **Rehacer** valen para cada unión, corte y movimiento. **Devolver las regiones** quita todos.
6. **Ver el movimiento mientras editas** (cuando las capas están construidas) muestra la imagen moviéndose: el puntero sobre la imagen, el punto del cuadro o la **Órbita** hacen de puntero. Las flechas mueven el punto cuando el cuadro tiene el foco.

Si un cambio baja el límite de movimiento, la herramienta avisa con el cambio que lo causó y el límite de antes y de ahora; menos movimiento evita que la imagen se estire. Arriba, una barra muestra cómo avanza la construcción de las capas, la de la app y la del escritorio.

**En la terminal:**

```sh
wprs-ctl edit depth regions foto.png
wprs-ctl edit depth join foto.png "0.25,0.5;0.7,0.5"
wprs-ctl edit depth split foto.png "0.0,0.5;1.0,0.5" --radius 0.004
wprs-ctl edit depth move foto.png 0.25,0.5 --by -0.2
wprs-ctl edit depth move foto.png --clear
wprs-ctl edit undo foto.png
```

`regions` lista las regiones desde la más cercana, cada una con un punto dentro: ese punto es el que se da a `join` y a `move`. `--png` guarda las regiones pintadas y `--json` imprime la lista como JSON. Con `edit depth layers foto.png --list` se ve el límite de movimiento de cada juego de capas.

## Cómo se mueve la imagen y por qué a veces se queda quieta

Cada capa se mueve con su propio campo de profundidad suave, y el movimiento total tiene un límite: nunca pasa de lo que las capas cubren, así que no se ve el borde del fondo ni se estira un trozo de imagen. Con una fuerza alta, el límite recorta el desplazamiento en vez de deformar la imagen. Las capas se construyen a la resolución de la pantalla (la app y `wprs-ctl` las construyen a 1920 como mucho, y el teléfono también).

Mientras las capas no están listas, o si no pueden moverla sin estirarla, la imagen se queda quieta. El motivo se lee en la línea roja de **Cortes de capa**, en la herramienta **Paralaje** y en `wprs-ctl edit depth layers ARCHIVO` (en la salida de errores; `--list` muestra además las regiones, el desplazamiento seguro y si cada juego se mueve). Los motivos son: no hay mapa de profundidad, no hay modelo de profundidad, el relleno está apagado, la imagen tiene operaciones de color, las capas se están construyendo, no se pudieron construir, o su profundidad no permite moverla sin estirarla. El daemon dice lo mismo en `GetState`, en el campo `still` de cada salida.

## Cómo descargar el modelo desde la herramienta de paralaje

**Dónde está:** **Paralaje**, en **Detalles > Editar imagen**.

Si enciendes el paralaje y falta el modelo de profundidad, la herramienta muestra enseguida **La imagen queda quieta: el modelo de profundidad no está instalado** con el tamaño de la descarga, y el botón **Descargar el modelo de profundidad…**. **Nada se descarga hasta que lo pidas**: el botón abre la ventana de consentimiento **¿Descargar el modelo de profundidad?**, y después se ve el avance con **Cancelar**. Si el modelo ya está pero la imagen no tiene mapa, el botón es **Generar el mapa**. El teléfono muestra el mismo aviso con un botón de descarga.

**En la terminal:** `wprs-ctl edit parallax ARCHIVO` imprime el aviso con el tamaño y el comando `wprs-ctl models download depth-anything-v2-small`.
