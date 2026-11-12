# Animación en el editor: líneas de tiempo, claves, curvas y clips

Esta página explica cómo hacer que los valores de una escena cambien con el tiempo: líneas de tiempo, fotogramas clave, el editor de curvas, los clips de huesos y modelos, los relojes de los vídeos y las reglas de reposo de los efectos.

La franja de abajo del editor, bajo el lienzo, es la zona de tiempo. Muestra las líneas de tiempo de la escena. Cuando eliges un clip de un esqueleto o de un modelo, la misma franja pasa a mostrar los huesos de ese clip (mira [Cómo editar un clip](#cómo-editar-un-clip-de-un-esqueleto-o-un-modelo)). Para el resto del editor mira [editor](editor.md), [lienzo](editor-lienzo.md) y [dibujo](editor-dibujo.md). Las acciones de la terminal están en [acciones](acciones.md).

## Cómo funciona el tiempo de una escena

- Toda escena tiene un **reloj de la escena**. Un valor animado lo sigue mientras no le digas otra cosa.
- Puedes añadir **líneas de tiempo** propias. Cada una tiene su duración, su modo de repetición, su velocidad y su manera de arrancar. Cada valor animado elige qué reloj sigue: el de la escena o una de tus líneas.
- Un **fotograma clave** fija el valor en un instante. Entre dos claves el valor se mueve según la **curva** del tramo.
- Un **evento** es una marca con nombre en una línea (o en un clip). Los disparadores y los scripts pueden arrancar cosas con ese nombre.
- Un vídeo con **reloj propio** tiene su propia franja con inicio, fin y velocidad (mira [Cómo recortar un vídeo](#cómo-recortar-un-vídeo-y-cambiar-su-velocidad)).

## Cómo poner una clave en un valor

Dónde está: en el inspector, junto a cada valor que se puede animar hay un botón pequeño. Muestra **◇** si el valor es fijo, **◈** si tiene claves pero ninguna en el instante actual y **◆** si hay una clave justo en este instante. El símbolo **⛓** indica que el valor sigue otra cosa (una propiedad, el sonido, un script...) y no se puede dar claves mientras la siga.

Pasos:

1. Elige la capa y, con el cabezal de la franja de tiempo, el instante donde quieres la clave (mira [Cómo mover el cabezal](#cómo-mover-el-cabezal-y-reproducir)).
2. Cambia el valor a lo que quieres en ese instante.
3. Pulsa el botón ◇ junto al valor y elige **Fotograma clave aquí**. La clave queda con el valor que se ve en ese momento.
4. Repite en otros instantes. Cuando el valor ya tiene claves, el mismo menú ofrece **Quitar el fotograma clave de aquí** (activo solo si hay una clave en el instante actual).
5. Para volver a un valor sin animar, abre el menú y elige **Valor fijo**. El valor queda con lo que se ve en ese instante.

El menú del botón ofrece también otros orígenes para el valor (propiedad de usuario, sonido, puntero, reproductor, script...). Eso se explica en [propiedades](editor-propiedades.md) y en [particulas y sonido](editor-particulas-y-sonido.md).

Cuando la escena aún no tiene ninguna clave, la franja de tiempo dice: **Aún no hay fotogramas clave: el botón ◇ junto a un valor añade uno en el instante actual.**

En la terminal: acción `keyframe` (puntero del valor, instante y valor). Mira [acciones](acciones.md).

## Cómo mover el cabezal y reproducir

Dónde está: barra de arriba de la franja de tiempo.

Pasos:

1. Pulsa **Reproducir** (cambia a **Pausar** mientras suena) para ver la animación en la vista previa.
2. Arrastra la barra deslizante para ir a un instante. A su izquierda se ve el instante actual, por ejemplo «1.25 s».
3. Al pulsar una clave de la franja, el cabezal salta a su instante.

Mientras reproduces, el cabezal avanza solo y vuelve al principio al llegar al final de la duración de la franja.

En la terminal: la posición del cabezal es solo de la vista previa del editor; no es una acción. Para ver un instante concreto se usa la captura de [cli](cli.md).

## Cómo ver y mover las claves en la franja de tiempo

Dónde está: la franja de tiempo, una fila por cada valor animado. Cada fila dice «capa · valor» y tiene un selector con el reloj que sigue. Las claves son rombos **◆** sobre una barra que va de 0 a la duración.

Pasos:

1. Pulsa un rombo para elegir la clave. Se pinta de otro color y el cabezal salta a su instante.
2. Arrástralo a izquierda o derecha para cambiar su instante. Mientras arrastras se ve el valor en segundos («1.25 s»). Al soltar queda como un solo paso de deshacer.
3. Para quitarla, con la clave elegida pulsa **Quitar el fotograma clave** en la barra de arriba.

En la terminal: acción `keyframe` con `from` para mover una clave y `remove` para quitarla. Mira [acciones](acciones.md).

## Cómo cambiar la suavidad de un tramo (la curva entre dos claves)

Dónde está: barra de arriba de la franja de tiempo, con una clave elegida. El selector de la barra elige cómo va el valor desde esa clave hasta la siguiente.

Pasos:

1. Elige una clave en la franja.
2. En el selector de la barra, elige la forma del tramo. Las opciones son **Lineal**, **Salto** (el valor cambia de golpe en la clave siguiente) y, para cada familia de curvas, **Entrada suave**, **Salida suave** y **Entrada y salida suaves**. Las familias son: cuadrática, cúbica, cuártica, quíntica, sinusoidal, exponencial, circular, con retroceso, con rebote y elástica. Por ejemplo «Entrada suave (cuadrática)».
3. Para dibujar la curva a mano, pulsa **Curva y base del fotograma clave** (icono de curva). Se abre **Curva hasta el siguiente fotograma clave** con dos asas. Arrástralas: en un número la altura es el valor; en las demás cosas (listas de números y colores) la altura es la parte del camino, de 0 a 1.
4. Si la clave no tiene otra después, el diálogo lo avisa: las asas solo curvan el tramo entre dos claves de números, listas de números o colores. Elige una que tenga otra después.
5. **Enderezar la curva** (aparece cuando la curva tiene asas propias) devuelve el tramo a la forma del selector.

Cada arrastre de asas es un solo paso de deshacer.

En la terminal: acción `keyframe` con `ease`, `in_handle`, `out_handle` o `straighten`. Mira [acciones](acciones.md).

## Cómo hacer relativas las claves (valor base)

Sirve para que las claves se sumen a un valor base en vez de ser valores absolutos. Así puedes cambiar el valor base de la capa sin rehacer la animación.

Dónde está: el mismo diálogo **Curva hasta el siguiente fotograma clave**, debajo del dibujo de las asas.

Pasos:

1. Elige una clave y pulsa **Curva y base del fotograma clave**.
2. Escribe el valor base en el campo («Un número, [x, y] o un color») y pulsa **Fijar la base**. Desde entonces el texto dice: **Los fotogramas clave se suman a este valor base:**.
3. Para volver a claves absolutas, pulsa **Absoluto**.

En la terminal: acción `keyframe` con `base` o `absolute`.

## Cómo crear una línea de tiempo

Dónde está: botón **Líneas de tiempo** (icono de cronómetro), a la derecha de la barra de arriba de la franja de tiempo.

Pasos:

1. Pulsa **Líneas de tiempo** y elige **Añadir una línea de tiempo**. Se crea una con nombre «timeline_1» (o el siguiente libre), de 5 segundos y que se repite.
2. En el mismo menú aparece cada línea con su nombre y su duración. Dentro de cada una tienes **Editar la línea de tiempo y sus eventos…**, **Ver en el inspector**, **Cambiar el nombre…** y **Quitar; los valores que mueve siguen el reloj de la escena**.
3. Para que un valor siga la línea, en su fila de la franja de tiempo abre el selector de reloj y elige la línea. El primer elemento, **Reloj de la escena**, lo devuelve al reloj general.
4. Para cambiar el nombre: **Cambiar el nombre…**. Solo vale minúsculas, cifras y guion bajo (hasta 32 caracteres).

En la terminal: acción `timeline` con `add`, `to` (nombre nuevo) o `remove`; la duración con `duration`. Para que un valor siga la línea se pone `timeline` en la acción `keyframe`, o se usa `set` sobre el puntero del valor. Mira [acciones](acciones.md).

## Cómo ajustar una línea de tiempo (duración, repetición, velocidad, arranque)

Dónde está: botón **Líneas de tiempo**, luego el nombre de la línea y **Editar la línea de tiempo y sus eventos…**. Se abre el diálogo **Línea de tiempo** con el nombre.

Pasos y campos:

1. **Duración en:** elige **Segundos** o **Fotogramas**.
   - En segundos, escribe la duración en **Segundos:**.
   - En fotogramas, elige cuántos **Cuadros:** tiene la línea y a cuántos fotogramas por segundo (**a** «30 fps»). La duración sale de dividir una cosa por la otra.
2. **Se reproduce:** **Una vez**, **Repitiéndose** o **Ida y vuelta**.
3. **Fundir el final con el principio:** solo se puede marcar cuando se repite. Hace que al volver al principio no se note el salto.
4. **Velocidad:** barra y campo de texto, de −4 a 4. Un valor negativo reproduce hacia atrás. La barra deja un solo paso de deshacer por arrastre.
5. **Retraso en segundos:** cuánto espera la línea antes de empezar.
6. **Arrancar sola:** si está marcada, la línea empieza en cuanto carga la escena. Si no, espera a su disparador.
7. **La arranca:** elige un disparador de la lista, o **Nada**. La lista trae los disparadores que la escena conoce.
8. **Dispararla en la vista previa:** simula ese disparador en la vista previa. Arranca esta línea y todas las líneas y sonidos con el mismo disparador, como lo haría la escena. Es la forma de probar una línea que no arranca sola.

Cada cambio es un paso de deshacer.

En la terminal: acción `timeline` con `duration`, `frames`, `fps`, `loop`, `wrap`, `speed`, `delay`, `autoplay` y `trigger`. Para ver qué arranca un disparador, la consulta `fires` de `scene ask`. Mira [acciones](acciones.md).

## Cómo poner eventos en una línea de tiempo

Los eventos son marcas con nombre en la línea. Una acción o un script puede esperar a uno por su nombre.

Dónde está: diálogo **Línea de tiempo**, campo **Eventos:**.

Pasos:

1. Haz doble clic en la franja del diálogo, en el instante donde quieres el evento. Se crea con el nombre «event_1» (o el siguiente libre). El texto de ayuda dice: **Haz doble clic en la franja para añadir ahí un evento; los disparadores y los scripts arrancan con su nombre.**
2. Pulsa la marca para elegirla. Aparecen su nombre (cámbialo y confirma), su instante («a los 2.5 s») y el botón **Quitar**.
3. Para moverla, arrástrala. Al pasar el puntero se ve «nombre a los 1.5 s: arrástralo para moverlo».

En la terminal: acción `timeline` con `add_events`, `drop_events` y `clear_events`. Mira [acciones](acciones.md).

## Cómo hacer que algo arranque con un disparador

El campo de disparador de una capa o de una línea de tiempo es un selector con un botón de prueba.

Dónde está: los campos de disparador del inspector y el diálogo **Línea de tiempo** (**La arranca:**).

Pasos:

1. Abre el selector y elige el disparador. **Sin fijar** deja el campo vacío.
2. Pulsa el botón **Activar este disparador en la vista previa** (icono de reproducir). Al pasar el puntero dice qué arranca: «Dispara X en la vista previa, como lo haría la escena, y arranca la línea de tiempo A, el sonido B». Si nada arranca con él, avisa: «todavía nada de la escena arranca con él».

En la terminal: consulta `fires` de `scene ask` para ver qué arranca un disparador. Mira [acciones](acciones.md).

## Cómo trabajar con el editor de curvas

Sirve para ver las curvas de los valores animados, mover varias claves a la vez y copiar y pegar.

Dónde está: botón **Editor de curvas** (icono de gráfica) en la barra de la franja de tiempo, y el mismo botón en la barra de un clip. Se activa cuando hay claves que mostrar. Si no las hay, el diálogo dice «No hay claves que mostrar: primero agrega fotogramas clave a un valor». Se abre el diálogo **Editor de curvas**.

Qué muestra: si hay una clave elegida, los valores animados de esa capa (en un clip, los canales del hueso o de la fila elegidos); si no, todos los valores animados. Como máximo se dibujan doce valores a la vez. Cada valor se parte en pistas: x, y y z de una posición; rojo, verde, azul y alfa de un color; o el número solo. Cada pista tiene su casilla debajo del gráfico para mostrarla u ocultarla, con el nombre «capa · parte» y su color. Cada pista se dibuja con su propia escala. La línea roja vertical es el cabezal.

### Elegir claves

1. Un clic en una clave la elige.
2. Ctrl o Mayúsculas más clic la añade a la selección o la quita de ella.
3. Arrastrar sobre el fondo dibuja un rectángulo y elige las claves de dentro. Con Ctrl o Mayúsculas se suman a las que ya estaban elegidas.
4. Un clic en el fondo, sin arrastrar, deja la selección vacía.
5. A la derecha de las herramientas se ve cuántas hay: «3 elegidas».

### Mover claves

1. Arrastra una clave elegida: se mueve en el tiempo (a izquierda y derecha) y en el valor (arriba y abajo). Todas las elegidas se mueven lo mismo.
2. El movimiento vertical solo se aplica si las claves elegidas son del mismo tipo y unidad (por ejemplo, todas posiciones). Si no, solo se mueven en el tiempo.
3. Con Alt, el arrastre vertical escala los valores alrededor del medio de las claves elegidas de esa parte, en vez de moverlos.
4. Las dos cajas de texto de la barra hacen lo mismo con un número exacto: **Estirar tiempos ×** (necesita al menos dos claves; estira desde la primera) y **Escalar valores ×** (necesita al menos dos del mismo tipo). Escribe el factor y pulsa Intro.
5. En el gráfico, la manija a la derecha de la caja de selección estira los tiempos al arrastrarla.

Cada arrastre es un solo paso de deshacer.

### Curvar un tramo con las asas

1. Elige una clave. Aparecen sus dos asas redondas (entrada y salida). Las que no se han tocado se ven con trazo discontinuo.
2. Arrastra un asa: cambia su largo en el tiempo y su altura. El tiempo no pasa de la clave vecina.
3. **Asas alineadas** (el modo de partida): al arrastrar un asa, la otra gira con ella y conserva su largo. **Asas rotas**: cada asa se mueve sola.
4. **Partes unidas** (el modo de partida): en las listas y los colores, el asa dibujada en una pista mueve la curva de todas las partes. **Partes separadas**: cada pista tiene sus propias asas, en las unidades de esa parte. Al volver a **Partes unidas** con una clave elegida que las tiene separadas, todas toman la forma de su primera parte.
5. **Enderezar** (con una sola clave elegida) deja el tramo sin asas propias.

### Copiar, pegar y quitar

| Qué | Botón | Atajo |
| --- | --- | --- |
| Copiar las claves elegidas | **Copiar** | Ctrl+C |
| Pegar en el cabezal | **Pegar en el cabezal** | Ctrl+V |
| Quitar las claves elegidas | **Quitar las claves elegidas** | Supr |
| Encuadrar lo elegido | **Encuadrar lo elegido (F)** | F |
| Encuadrar todo | **Encuadrar todo (Inicio)** | Inicio |

Los atajos solo valen con el editor de curvas abierto. Las claves copiadas van también al portapapeles del sistema como texto, así que puedes pegarlas en otra ventana del editor o en otro proyecto. Si el portapapeles tiene otro texto, se pegan las últimas claves copiadas en la aplicación. Si copiaste claves de un solo valor y ese valor no está en el gráfico, se pegan en el valor elegido.

### Mover la vista

- La rueda acerca o aleja el tiempo alrededor del puntero.
- Ctrl y rueda acerca o aleja el valor.
- Mayúsculas y rueda (o la rueda horizontal) desplaza el tiempo.
- Arrastrar con el botón del medio mueve la vista en el tiempo y en el valor.
- **Encuadrar lo elegido (F)** y **Encuadrar todo (Inicio)** ajustan la vista a las claves elegidas o a todas.

En la terminal: acción `edit_keys` para mover, escalar, estirar, curvar, enderezar, unir, quitar y pegar; consultas `curves` (claves por parte con asas y curva muestreada) y `key_clip` (texto del portapapeles de claves) de `scene ask`. Mira [acciones](acciones.md).

## Cómo editar un clip de un esqueleto o un modelo

Un clip es una animación con nombre de los huesos de una capa con esqueleto o de un modelo 3D. Una capa puede tener varios clips (andar, saludar...). Cada clip tiene una duración, un modo de repetición, claves por hueso y eventos.

Dónde está:

- En una capa con esqueleto: inspector de la capa, sección de huesos, botón **Editar los clips en la línea de tiempo** (en esa sección también está la lista **Clip de la vista previa**). Si la capa no tiene clips, crea uno llamado «clip_1» y lo abre.
- En un modelo 3D: sección **Animación** del inspector. Cada clip aparece como «nombre, 2 s de archivo.glb» (o «de sus propias claves» si no viene del archivo) con el botón **En la línea de tiempo**.
- Con el clip abierto, la franja de tiempo cambia: arriba la barra del clip, debajo la fila de eventos y debajo una fila por cada canal.

Pasos para crear y organizar clips (barra de arriba del clip):

1. El selector de la izquierda cambia de clip.
2. **Añadir un clip** (icono de más) abre un menú: **Añadir un clip** o **Añadir un clip y reproducirlo en una capa de animación nueva**.
3. **Cambiar el nombre del clip** y **Quitar el clip** actúan sobre el clip abierto.
4. **Longitud** es la duración en segundos (con un decimal).
5. El selector de repetición ofrece **Repetir**, **Ida y vuelta** y **Una vez**.
6. **Volver a las líneas de tiempo** (icono de cerrar) devuelve la franja al modo de las líneas.

Un modelo 3D con animaciones en su archivo tiene además **Clip nuevo del archivo…**: elige una animación del archivo y se crea un clip con su nombre, ya abierto.

En la terminal: acción `clip` (crear, renombrar con `to`, `length`, `loop`, quitar, `animation` para tomarla del archivo del modelo, `play` para una capa de animación nueva). Consulta `motion` de `scene ask` para ver los clips de una capa. Mira [acciones](acciones.md).

## Cómo poner claves a un hueso, una forma o un orden

Dónde está: franja de tiempo con un clip abierto. Cada fila es un canal:

- Un hueso tiene un canal por cada cosa que mueve: en una malla, rotación, posición y escala; en un modelo 3D, ángulos y desplazamiento. La fila se llama «hueso · canal».
- **Forma X**: el peso de una forma de mezcla, de −1 a 1. Tiene una barra para fijarlo en el instante actual.
- **Orden de X**: el orden de dibujo de una parte, con un campo numérico.

Pasos:

1. Elige el hueso (en el lienzo o en la lista de huesos). Sus filas salen en negrita.
2. Mueve el cabezal al instante deseado con la barra del clip (**Reproducir** / **Pausar**, el instante y la barra).
3. Coloca el hueso en la pose que quieres.
4. Pulsa **Clave** (icono de clave). Solo se puede si hay un hueso elegido. Guarda la pose actual del hueso.
5. Para una forma o un orden, cambia su barra o su campo en la fila: pone una clave en el instante actual.
6. Para mover una clave, arrástrala por la fila. Con Ctrl pulsado al empezar a arrastrar, la clave se copia en vez de moverse (la ayuda dice «Copiar a 1.25 s» o «Mover a 1.25 s»).
7. Pulsa una clave para elegirla (el cabezal salta a ella). Una clave con forma de **■** es de tipo salto; las demás son **◆**.
8. El selector de curva de la barra del clip cambia la suavidad del tramo, igual que en las líneas de tiempo. Si dibujaste la curva a mano, el selector dice **Curva propia**.
9. **Curva de la clave** abre **Curva hasta la siguiente clave**, con las dos asas. **Quitar la clave** borra la elegida.
10. Pulsa en la barra de una fila, lejos de las claves, para llevar el cabezal a ese instante.

El **Editor de curvas** de la barra del clip abre las claves del hueso elegido (o de todos), con todas las herramientas de la sección anterior.

En la terminal: acción `key` (una clave de un canal: `bone`, `shape` u `order`, `time`, el valor y `ease`; `from` mueve, `copy` copia, `remove` quita). Consulta `keys` de `scene ask` para listar las claves de un clip. Mira [acciones](acciones.md).

## Cómo poner eventos en un clip

Dónde está: fila **Eventos** del clip abierto, bajo su barra.

Pasos:

1. Haz doble clic en la fila, en el instante donde quieres el evento. Se crea con el nombre «event_1» (o el siguiente libre).
2. Arrástralo para moverlo. Al pasar el puntero se ve su nombre y su instante.
3. Púlsalo: el cabezal salta a él y se abre un cuadro con **Nombre del evento**, **Cambiar el nombre** y **Quitar**.

Cuando el clip pasa por la marca, se dispara el evento con ese nombre.

En la terminal: acción `clip_event` (`name`, `time`, `at`, `to`, `remove`). Mira [acciones](acciones.md).

## Cómo mezclar clips (capas de animación)

Dónde está: inspector de la capa con esqueleto o del modelo, sección **Capas de animación**.

Pasos:

1. Primero añade un clip. Si no hay ninguno, el texto dice «Añade primero un clip; cada capa reproduce un clip y se mezclan en orden».
2. Elige un clip en **Añadir una capa que reproduzca…**.
3. En cada capa de animación: elige el clip, marca **Encendido** para activarla y **Suma** para que se sume a las anteriores en vez de reemplazarlas.
4. Las barras ajustan **Peso** (0 a 1), **Velocidad** (0 a 4), **Entrada gradual** y **Salida gradual** (en segundos).
5. **Solo estos huesos** limita la capa a los huesos que elijas.
6. La flecha hacia arriba sube la capa en el orden de mezcla y el icono de papelera la quita.
7. Para ver un clip solo en la vista previa, la lista **Clip de la vista previa** de la capa con esqueleto elige uno (o **Ninguno**).
8. En un clip con movimiento, **Movimiento de raíz: el clip mueve la capa** hace que la raíz del clip desplace la capa entera.

En la terminal: acción `animate` (`layer`, `root`, `preset`, `morph`). Mira [acciones](acciones.md).

## Cómo recortar un vídeo y cambiar su velocidad

Dónde está: inspector de una capa de vídeo (o de imagen animada), sección **Vídeo**. Debajo del título se ve el tamaño («1280 × 720 px») y la duración («8.00 s de duración» o «duración desconocida»).

Pasos:

1. Arrastra el asa izquierda y la derecha de la barra: marcan el **Inicio** y el **Fin** que se reproducen. Al arrastrar un asa se ve «Inicio 1.50 s» o «Fin 6.00 s». La parte fuera queda oscura. La barra muestra fotogramas del vídeo para orientarte.
2. **Velocidad** (campo numérico, de 0 a 4 veces, «1.00×»): un vídeo se reproduce de 0 a 4 veces su velocidad. Está desactivado si la velocidad ya es un valor animado.
3. **Bucle** hace que vuelva a empezar al terminar.
4. **Usar el tamaño del vídeo** ajusta el cuadro de la capa a las medidas del vídeo.
5. **Reproducir todo el vídeo** quita el recorte.

En la terminal: acción `film` con `trim`, `rate` o `fit`. Mira [acciones](acciones.md).

## Cómo ver el reloj propio de los vídeos en la franja de tiempo

Los vídeos con reloj propio (la opción de reproducción de la capa de vídeo, que el editor ofrece solo donde vale) tienen una franja en la zona de tiempo.

Dónde está: bajo las líneas de tiempo, una fila por vídeo con la etiqueta «capa · reloj propio».

Qué ves y qué puedes cambiar:

1. La franja muestra ocho fotogramas repartidos por el vídeo, el tramo de reproducción como una barra con dos asas y una marca roja donde está el reloj en el cabezal.
2. Arrastra las asas para cambiar el inicio y el fin. Al soltar queda guardado («Inicio 1.00 s», «Fin 4.00 s»).
3. El campo «1.00×» cambia la velocidad del reloj («Velocidad de este reloj»).
4. **Bucle** y **Se reproduce** activan la repetición y la reproducción de ese reloj.

En la terminal: `wprs-ctl scene clocks DIR [--time T] [--strip N]` imprime las mismas franjas (con `--strip` guarda los fotogramas); los valores se cambian con `set` sobre el puntero del reloj. Mira [acciones](acciones.md) y [cli](cli.md).

## Cómo animar una hoja de sprites

Una hoja de sprites es una imagen con varios cuadros, que se reproducen como una animación cuadro a cuadro.

Dónde está: panel de recursos, botón **Cortar sprites…** (icono de cuadrícula) en la fila de una imagen. Se abre **Cortar los sprites de** el nombre del recurso.

Pasos:

1. Para cortar en cuadrícula, pon **Filas**, **Columnas**, **Margen**, **Separación** y **Cuadros (0 para todos)**; elige el orden de lectura (por ejemplo **De izquierda a derecha y de arriba abajo**) y pulsa **Cortar la cuadrícula**.
2. Para dibujar un cuadro a mano, arrastra sobre la imagen. Arrastra un cuadro para moverlo y su esquina para cambiarle el tamaño.
3. En la tira de abajo, la vista previa reproduce los cuadros. **Pausar** / **Reproducir** la detiene. **Fotogramas por segundo** fija la velocidad y **Cuadros que se reproducen** cuántos de la tira se usan.
4. Elige un cuadro de la tira y usa **Antes** o **Después** para cambiar su orden, o **Quitar el cuadro** para borrarlo. **Quitar todos los cuadros** vacía la hoja.

El dibujo y la edición de los cuadros están también en [dibujo](editor-dibujo.md).

En la terminal: acción `sprites` (`grid`, `rects`, `add`, `change`, `remove`, `reorder`, `play`, `clear`). Mira [acciones](acciones.md).

## Cómo decidir cuándo despierta una escena (reglas de reposo de un efecto)

Una escena duerme cuando nada de lo que contiene necesita moverse. Un efecto del paquete dice qué parámetros necesitan despertar la escena y por qué.

Dónde está: inspector de un efecto, campo de reglas.

Qué ves: una fila por parámetro con tres selectores.

1. **Pide** (el primero): qué entrada necesita el parámetro mientras no vale cero. Opciones: **Nada**, **Tiempo**, **El puntero**, **El sonido**, **Clics**.
2. **Espera a** otro parámetro (o **No espera a nada**): el parámetro solo pide cuando el otro tampoco vale cero.
3. **Cede a** otro parámetro (o **No cede a nada**): el parámetro se aparta cuando el otro pide.

Sobre los efectos incluidos: sus reglas se muestran, pero no se pueden cambiar. El texto dice: **Un efecto incluido conserva estas reglas; un efecto del paquete las cambia aquí.** Si el efecto siempre pide algo, se lee «Siempre pide: ...». La escena duerme cuando ningún parámetro pide nada.

En la terminal: acción `effect_rule` (`effect`, `param`, `uses`, `waits_for`, `yields_to`). Mira [acciones](acciones.md) y [efectos](editor-efectos.md).

## Resumen de atajos

Solo hay atajos de teclado dentro del **Editor de curvas**:

| Atajo | Qué hace |
| --- | --- |
| Clic | Elige una clave |
| Ctrl o Mayúsculas + clic | Suma o quita una clave de la selección |
| Alt + arrastrar | Escala los valores de las claves elegidas |
| Ctrl+C | Copiar |
| Ctrl+V | Pegar en el cabezal |
| Supr | Quitar las claves elegidas |
| F | Encuadrar lo elegido |
| Inicio | Encuadrar todo |
| Rueda | Acerca o aleja el tiempo |
| Ctrl + rueda | Acerca o aleja el valor |
| Mayúsculas + rueda | Desplaza el tiempo |
| Botón del medio + arrastrar | Mueve la vista |

En la franja de clips, Ctrl al empezar a arrastrar una clave la copia en vez de moverla.
