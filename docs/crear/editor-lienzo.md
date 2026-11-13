# Lienzo, capas e inspector del editor

Esta página explica cómo colocar y ajustar las capas de una escena con el ratón y con el inspector: el lienzo, la lista de capas, el inspector, las plantillas, las partes compartidas y los recursos.

La ventana del editor tiene tres zonas. A la izquierda, un panel con pestañas: **Capas**, **Recursos**, **Paquete**, **Reproductor** y **Probar**. En el centro, el lienzo con su barra de herramientas y, debajo, la zona de tiempo (mira [animación](editor-animacion.md)). A la derecha, el inspector y, debajo, la lista de problemas. El resto del editor está en [editor](editor.md). Las acciones de la terminal están en [acciones](acciones.md).

## Cómo funciona el lienzo

- El lienzo es la vista previa de la escena. Aparece mientras el demonio de fondos (wprsd) está en marcha. Si no hay vista previa, el lienzo lo dice con un texto sobre el fondo.
- Sobre la vista previa se dibujan las **asas** de la capa elegida: el contorno, cuadraditos en las esquinas y los lados, un círculo para girarla y otro para su pivote.
- Los clics y los arrastres se miden en píxeles del lienzo de la escena, no en píxeles de la pantalla. Lo que ves ampliado o reducido sigue valiendo lo mismo.
- Si el valor que mueves tiene fotogramas clave, mira [Cómo escribir sobre las claves](#cómo-escribir-sobre-las-claves-o-añadir-una-clave).

En la terminal, el editor no tiene lienzo, pero cada cosa que haces con las asas es una acción: `layer` con `transform` o con `grip`. Mira [acciones](acciones.md).

## Cómo elegir una capa

Dónde está: en el lienzo y en la pestaña **Capas**.

Pasos:

1. Haz clic sobre la capa en el lienzo. Se elige la capa cuyo contorno contiene el punto. Las capas dibujan su zona de clic con la forma real de su contorno, así que una capa girada o inclinada solo responde dentro de su forma.
2. O haz clic en su fila en la pestaña **Capas**.
3. Para elegir el lienzo mismo (tamaño, fondo, brillo y paralaje), haz clic en un sitio vacío del lienzo. El inspector pasa a mostrar **Lienzo**.

Una capa elegida se dibuja con el contorno entero y las asas. Una capa **bloqueada** se puede elegir, pero no muestra asas ni se deja arrastrar (mira [Cómo bloquear, ocultar y aislar capas](#cómo-bloquear-ocultar-y-aislar-capas)).

En la terminal: no hay "elegir"; cada acción nombra su capa con su id.

## Cómo elegir varias capas

El lienzo trabaja siempre con una capa elegida. Para varias capas a la vez se usa la lista de capas.

Dónde está: pestaña **Capas**.

Pasos:

1. Haz clic en una capa para elegirla.
2. Mantén Ctrl y haz clic en otras filas para marcarlas. Ctrl y clic en una fila ya marcada la desmarca. Un clic sin Ctrl deja solo la capa elegida.
3. Con la selección hecha, usa **Agrupar**, **Hacer plantilla** o **Copiar las capas**. Esos tres botones actúan sobre la capa elegida y las marcadas con Ctrl.

Arrastrar en el lienzo mueve solo la capa elegida. No hay selección con recuadro ni mover varias capas con un solo arrastre; para eso agrúpalas y mueve el grupo.

Atajo: Ctrl + clic en la fila de la capa (`SceneLayerRow.qml`).

En la terminal: `group` recibe la lista de ids; `template` con `make` recibe la lista de ids. Mira [acciones](acciones.md).

## Cómo mover una capa

Dónde está: en el lienzo, con la capa elegida.

Pasos:

1. Arrastra la capa desde cualquier punto de su contorno. Si no estaba elegida, el arrastre la elige y la mueve.
2. Suelta. Todo el arrastre cuenta como un solo paso del historial.
3. Para moverla poco a poco, haz clic en el lienzo o en la lista y pulsa las flechas del teclado. Una flecha mueve 1 píxel; con Mayús, 10 píxeles.
4. Para escribir la posición exacta, usa el campo **Posición (px)** del grupo **Transformación** del inspector.

Atajo: flechas, y Mayús con flechas para 10 píxeles (`SceneOverlays.qml`).

En la terminal: `layer` con `transform` y `position` pone la posición exacta; con `grip` y `nudge` mueve un paso, como las flechas.


```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"cartel"}'
wprs-ctl scene act mi-escena layer '{"transform":{"layer":"cartel","position":[960,540],"rotation":15}}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"cartel","grip":"nudge","to":[10,0]}}'
wprs-ctl scene act mi-escena layer '{"grip":{"layer":"cartel","grip":{"scale":{"corner":[1,1],"even":true}},"to":[1200,700]}}'
```

## Cómo ajustar con imanes y guías

Dónde está: barra del lienzo, botón **Imanes** (activado de entrada) y lista **Guía:**.

Pasos:

1. Con **Imanes** activado, al arrastrar una capa sus bordes y su centro se pegan a los bordes y al centro del lienzo y de las demás capas. Una línea de color marca cada alineación.
2. Mantén Alt mientras arrastras para colocarla sin pegarse.
3. Los imanes solo afectan al movimiento con el ratón. Las asas de escala y de giro no se pegan.
4. En **Guía:** elige una proporción (**Sin guía**, 16:9, 21:9, 32:9, 4:3, 16:10 o 9:16). Se dibuja un marco de esa proporción dentro del lienzo, para ver qué queda a la vista en pantallas de esa forma. La guía no se guarda en la escena.

Atajo: Alt mientras arrastras (`SceneGizmo.qml`).

En la terminal: `layer` con `grip` acepta `snap` (distancia en píxeles) cuando el agarre es `move`.

## Cómo escalar y estirar una capa

Dónde está: en el lienzo, asas cuadradas de la capa elegida.

Pasos:

1. Arrastra una asa de esquina para escalar desde el pivote. Con Mayús se mantienen las proporciones.
2. Arrastra una asa del medio de un lado para estirar solo ese lado.
3. O escribe **Escala** en el grupo **Transformación** del inspector.

Atajo: Mayús al arrastrar una esquina (`SceneGizmoKnobs.qml`).

En la terminal: `layer` con `grip` y `scale` (opción `even` para mantener proporciones), o `transform` con `scale`.

## Cómo sesgar una capa

Dónde está: en el lienzo, asas del medio de los lados.

Pasos:

1. Mantén Ctrl y arrastra el asa del medio de un lado. La capa se sesga a lo largo de ese borde.
2. También puedes escribir el campo **Inclinación** de dos valores sin unidad del grupo **Transformación**. Ese es el sesgo. El otro campo llamado **Inclinación**, en grados, es la inclinación en profundidad (mira [Cómo inclinar y acercar una capa](#cómo-inclinar-y-acercar-una-capa-en-profundidad)).

Atajo: Ctrl al arrastrar el asa de un lado (`SceneGizmoKnobs.qml`).

En la terminal: `layer` con `grip` y `skew`, o `transform` con `skew`.

## Cómo girar una capa y mover su pivote

Dónde está: en el lienzo. El círculo de giro está sobre el borde de arriba de la capa. El pivote es un círculo de otro color en el centro de giro.

Pasos:

1. Arrastra el círculo de giro: la capa gira alrededor del pivote.
2. Para cambiar el centro de giro y de escala, arrastra el círculo del pivote. La capa se queda donde está; solo cambia su **Origen**.
3. O escribe **Giro (°)** y **Origen** en el grupo **Transformación**.

En la terminal: `layer` con `grip` y `turn` o `origin`, o `transform` con `rotation` y `origin`.

## Cómo inclinar y acercar una capa en profundidad

Dónde está: en el lienzo, cerca del pivote de la capa elegida. Hay un anillo de puntos y tres asas pequeñas unidas al pivote por líneas.

Pasos:

1. Arrastra el extremo del eje vertical hacia el pivote o lejos de él: inclina la capa sobre su ancho.
2. Arrastra el extremo del eje horizontal: inclina la capa sobre su alto.
3. Arrastra la asa en forma de rombo a lo largo de la línea que sale de la capa: la acerca a quien mira o la aleja. Ese valor es la **Profundidad (px)**.
4. Cada asa muestra en su ayuda los grados o los píxeles actuales. Los mismos valores están en el grupo **Transformación** del inspector (**Inclinación** en grados y **Profundidad (px)**).

Una capa inclinada se dibuja con su contorno proyectado, con trazo discontinuo.

En la terminal: `layer` con `grip` y `tilt` (con `axis` 0 o 1) o `lift`, o `transform` con `tilt` y `z`.

## Cómo ajustar una capa al lienzo

Dónde está: barra del lienzo, botón **Ajustar al lienzo** (activo si hay una capa elegida y no está bloqueada).

Pasos:

1. Elige la capa.
2. Abre el botón y elige **Cubrir el lienzo** (llena el lienzo, recortando lo que sobre), **Encajar dentro del lienzo** (se ve entera) o **Estirar al lienzo** (llena sin respetar la proporción).

En la terminal: `layer` con `fit` (`cover`, `contain` o `stretch`).


```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"fondo"}'
wprs-ctl scene act mi-escena layer '{"fit":{"layer":"fondo","fit":"cover"}}'
```

## Cómo cambiar el tamaño de lo que captura un ajuste

Dónde está: en el lienzo, cuando eliges una capa de tipo **Ajuste** que tiene un tamaño propio.

Pasos:

1. Se dibuja el rectángulo que la capa captura, con asas en las esquinas y los lados. Un texto indica **Captura X × Y px; el máximo es ...**.
2. Arrastra un asa: cambia el tamaño de la parte capturada y la capa conserva su escala. Las asas de escala normales no se dibujan en esta capa.

Mira [efectos](editor-efectos.md) para el resto de las capas de ajuste.

## Cómo mover las asas de formas y textos

Dónde está: en el lienzo, al elegir una capa de tipo **Forma** o **Texto**.

- Una forma muestra asas propias: ancho, alto, tamaño (con Mayús mantiene las proporciones), esquinas redondeadas, grosor del anillo, número de lados o de puntas, profundidad de las puntas, principio y final de una línea, grosor del contorno y los extremos del degradado. Cada asa explica en su ayuda lo que hace (**Fijar el ancho**, **Redondear las esquinas**...).
- Un texto muestra un asa en el borde derecho para fijar su ancho de ajuste (**Fijar el ancho del texto; doble clic para dejar de ajustarlo**) y una etiqueta con **X × Y px, N líneas**.

Cómo dibujar y editar formas y textos está en [dibujo](editor-dibujo.md).

En la terminal: `layer` con `figure` (`shape` o `text_width`).

## Cómo acercar, alejar y desplazar el lienzo

Dónde está: barra del lienzo, a la izquierda: **Reducir**, el porcentaje, **Ampliar** y **Ajustar**.

Pasos:

1. **Ampliar** y **Reducir** cambian el zoom en pasos de un cuarto. El zoom va del 10 % al 1600 %.
2. **Ajustar** vuelve al 100 % y centra el lienzo.
3. Con el ratón: Ctrl y rueda cambian el zoom (un 10 % por muesca); la rueda sola desplaza la vista; arrastrar con el botón del medio o el derecho también la desplaza.

El zoom y el desplazamiento solo cambian lo que ves en el editor; no se guardan en la escena.

Atajo: Ctrl + rueda para el zoom (`SceneCanvas.qml`).

En la terminal: no tiene equivalente.

## Cómo bloquear, ocultar y aislar capas

Dónde está: en cada fila de la pestaña **Capas** hay tres botones: **Ocultar**, **Bloquear** y **Solo**.

Pasos:

1. **Ocultar**: la capa deja de dibujarse y su nombre se ve atenuado. Púlsalo otra vez para mostrarla.
2. **Bloquear**: la capa no se puede mover con el ratón, no muestra asas y los botones de la barra del lienzo que actúan sobre la capa elegida (**Ajustar al lienzo**, **Seguir a una capa**, **Dejar de seguir**) se apagan. Sirve para no mover por descuido un fondo.
3. **Solo**: la escena muestra únicamente esa capa. Al soltar el botón se muestran todas.

Un botón sin activar se ve más tenue que uno activo.

En la terminal: `hide` (con `hidden`), `lock` (con `locked`) y `solo` (sin `id` para quitar el aislamiento).


```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"cartel"}'
wprs-ctl scene act mi-escena hide '{"id":"cartel","hidden":true}'
wprs-ctl scene act mi-escena lock '{"id":"cartel","locked":true}'
wprs-ctl scene act mi-escena solo '{"id":"cartel"}'
wprs-ctl scene act mi-escena solo '{}'
```

## Cómo escribir sobre las claves o añadir una clave

Dónde está: barra del lienzo, botón **Escribir sobre las claves**.

Cuando un valor de la capa (posición, giro...) tiene fotogramas clave, las asas del lienzo no pueden cambiarlo "a secas". Qué hacen depende de este botón:

- Apagado: las asas añaden una clave en el cabezal a los valores animados y no tocan los valores enlazados.
- Encendido: las asas escriben un valor fijo encima de las claves y de los enlaces.

En la terminal: las acciones `layer` aceptan `at` (instante de la clave) y `replace` (escribir encima).

## Cómo cambiar el tamaño del lienzo y el reloj de la escena

Dónde está: barra del lienzo, botón **Lienzo y reloj**.

Pasos:

1. Abre el botón. En **Tamaño de pantalla** elige una medida de la lista: HD, Full HD, QHD, 4K UHD, 5K, Ultrawide 21:9, Super ultrawide 32:9, 16:10, 4:3 o Phone 9:19.5. La medida activa lleva una marca.
2. **Usar el cabezal como fotograma fijo (x s)** toma el instante donde está el cabezal como imagen fija de la escena.
3. **Una semilla nueva cada vez que empieza la escena** hace que lo que usa números al azar sea distinto en cada arranque. Apagado, siempre es igual.

Las demás opciones del lienzo (fondo, brillo y paralaje) están en el inspector al elegir **Lienzo**. Mira [escena](escena.md).

En la terminal: `canvas` con `size`, `clock` (con `poster`, `seed` o `fresh`).

## Cómo activar el brillo y el paralaje del lienzo

Dónde está: inspector, al hacer clic en un sitio vacío del lienzo (**Lienzo**).

Pasos:

1. Interruptor **Brillo:** **Las partes claras brillan**. Al activarlo aparecen **Umbral**, **Fuerza** y **Radio**.
2. Interruptor **Paralaje:** **Las capas se mueven en profundidad**. Al activarlo aparecen **Profundidad**, **Retraso**, **Puntero** y **Desplazamiento** (dos deslizadores, uno para cada dirección).
3. Cada valor, menos **Retraso**, tiene un botón de enlace para que siga un origen en vez de un número fijo.
4. Casilla **Seguir la inclinación del teléfono**: el paralaje reacciona también a la inclinación del teléfono.
5. Cada capa decide cuánto se mueve con su campo **Profundidad del paralaje:** en el inspector de la capa.

En la terminal: `canvas` con `bloom` y `parallax`.

## Cómo probar el paralaje en el lienzo

Dónde está: barra del lienzo, botón **Probar el paralaje**. Solo aparece si el paralaje del lienzo está activado.

Pasos:

1. Activa el botón. Aparece un círculo **Puntero** en el centro del lienzo.
2. Arrástralo: cada capa se mueve según su profundidad de paralaje, como si el puntero estuviera ahí.
3. Si el paralaje sigue la inclinación del teléfono, aparece además un cuadrado **Inclinación** y una línea discontinua desde el centro. Arrástralo lejos del centro para inclinar el teléfono simulado hacia ese lado.
4. Con una capa elegida y el botón activo, aparece un círculo verde **Profundidad** con una línea discontinua desde el centro de la capa. Arrástralo para fijar la profundidad de paralaje de la capa: cada cuarto del ancho del lienzo vale 1, de -10 a 10. Si la profundidad es la misma a lo ancho y en alto, el círculo se mueve en diagonal y escribe el mismo valor en las dos; mantén Mayús para fijarlas por separado. Una profundidad que sigue un enlace o fotogramas clave no lleva asa.
5. Aunque el botón esté apagado, si eliges **Lienzo** aparece un círculo **Centro del paralaje** con una línea desde el centro del lienzo: arrástralo para fijar dónde reposa el paralaje (**Desplazamiento**), de -1 a 1 en cada dirección.
6. Desactiva el botón para volver a la vista normal.

En la terminal: `canvas` con `probe` (`pointer`, `tilt` o `off`) y con `parallax` y su `offset`, y `layer` con `depth`.

## Cómo probar el puntero y los clics

Dónde está: barra del lienzo, a la derecha de **Ajustar**, botón **Probar el puntero y los clics**.

Pasos:

1. Actívalo. Mientras está activo, el ratón sobre la vista previa deja de editar y pasa a ser un puntero simulado: un círculo marca dónde está y un anillo rojo se expande en cada clic.
2. Mueve y pulsa sobre la vista previa para probar lo que reacciona al puntero (enlaces al puntero, clics, entrar y salir de un área).
3. Desactívalo, o saca el ratón de la vista previa, y el puntero simulado se va.

En la terminal: `wprs-ctl input touch` con la posición y los botones; mira [cli](cli.md).

## Cómo usar una cámara 2D en el lienzo

Dónde está: barra del lienzo, botón **Cámara 2D**.

Pasos:

1. Abre el botón y elige **Añadir una cámara 2D**.
2. Con la cámara añadida, el mismo menú ofrece **Ver a través de la cámara** (el lienzo muestra la escena como la ve la cámara), **Ver el lienzo entero con el marco de la cámara** y **Quitar la cámara**.
3. En la vista entera aparece el marco de la cámara con tres asas: una esquina para el zoom (**Zoom de la cámara**), un círculo **Cámara** en el centro para moverla y otro para girarla.

En la terminal: `canvas` con `camera`, `camera_grip` y `camera_view`.

## Cómo añadir una capa

Dónde está: pestaña **Capas**, botón **Añadir una capa**.

Pasos:

1. Abre el botón. El menú lista los tipos: **Imagen**, **Vídeo**, **Texto**, **Forma**, **Color sólido**, **Espectro del sonido**, **Partículas**, **Sonido**, **Grupo**, **Ajuste**, **Malla**, **Modelo 3D**, **Grupo 3D** y **Formas de una reserva**.
2. Los tipos que necesitan un archivo (imagen, vídeo, sonido, malla, modelo 3D) están apagados con el aviso **importa antes un archivo de este tipo** mientras la escena no tenga un recurso de ese tipo. **Formas de una reserva** pide antes una reserva (**agrega una reserva primero**).
3. Si hay un solo recurso del tipo, la capa lo usa. Si hay varios, un menú **Qué recurso** deja elegir.
4. Si tenías un grupo elegido, la capa entra en ese grupo.
5. **Luz** abre otro menú: **Luz puntual**, **Foco**, **Tubo de luz** y **Luz direccional**.
6. **Capa desde un archivo** elige un archivo del disco y crea la capa con él: **Imagen…**, **Vídeo…**, **Sonido…**, **Malla desde un archivo .glb…** o **Modelo 3D desde un archivo .glb…**.
7. **Malla desde una imagen…** crea una malla a partir de un recurso de imagen. Mira [dibujo](editor-dibujo.md).
8. El botón **Sistemas de partículas** de la misma fila abre la gestión de los sistemas de partículas de la escena (mira [partículas y sonido](editor-particulas-y-sonido.md)).

Cada capa recibe un id propio, que puedes cambiar (mira [Cómo cambiar el id de una capa](#cómo-cambiar-el-id-de-una-capa)).

En la terminal: `add_layer` (con `kind`, y `file` o `source`) y `light` con `add`. Mira [acciones](acciones.md).


```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"text","id":"titulo"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"group","id":"cielo"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo","into":"cielo"}'
wprs-ctl scene act mi-escena move_layer '{"id":"titulo","to":0}'
wprs-ctl scene act mi-escena light '{"add":{"light":"spot"}}'
wprs-ctl scene act mi-escena rename '{"kind":"layer","id":"titulo","to":"nombre"}'
wprs-ctl scene act mi-escena duplicate_layer '{"id":"nombre"}'
wprs-ctl scene act mi-escena ungroup '{"id":"cielo"}'
```

## Cómo agrupar y desagrupar capas

Dónde está: pestaña **Capas**, botones **Agrupar**, **Desagrupar**, **Meter en un grupo** y **Sacar del grupo**.

Pasos:

1. Elige una capa y marca otras con Ctrl (mira [Cómo elegir varias capas](#cómo-elegir-varias-capas)). Pulsa **Agrupar**: se crea un grupo con todas.
2. Elige un grupo y pulsa **Desagrupar** para disolverlo. Sus capas se quedan donde estaban.
3. Para llevar una capa a un grupo ya hecho: elígela, pulsa **Meter en un grupo** y elige el grupo en el menú.
4. **Sacar del grupo** devuelve la capa elegida a la lista principal.

Un grupo mueve, gira y oculta a todas sus capas a la vez. Las capas de dentro se ven sangradas en la lista. Mira [capas](capas.md).

En la terminal: `group`, `ungroup`, `move_layer` (con `into` para entrar en un grupo).

## Cómo ordenar las capas

Dónde está: pestaña **Capas**. La lista muestra arriba la capa que se dibuja encima de todas.

Pasos:

1. Elige la capa y pulsa **Subir** o **Bajar**: cambia un puesto dentro de su grupo.
2. O arrastra el asa de la izquierda de su fila (**Arrastra para mover la capa**) hasta otra fila. Una línea marca el sitio. Al soltar, la capa toma el puesto de esa fila, dentro del mismo grupo que ella.
3. Los botones están apagados si la capa ya está arriba o abajo del todo.

En la terminal: `move_layer` con `id` y `to` (posición) y, si hace falta, `into`.

## Cómo cambiar el id de una capa

Dónde está: pestaña **Capas**, botón **Cambiar el id de la capa**.

Pasos:

1. Elige la capa y pulsa el botón. Se abre **Cambiar el id de X**.
2. Escribe el id que quieres. Solo valen letras minúsculas, números y guion bajo, hasta 32 caracteres.
3. Acepta. Los vínculos, padres, máscaras, efectos, disparadores y líneas de tiempo que la nombran siguen al id elegido.
4. Si algún script nombra la capa, el diálogo lista las líneas de script que no se reescriben; corrígelas a mano.

En la terminal: `rename` con `kind` `layer`. Sirve igual para recursos, líneas de tiempo y sistemas de partículas.

## Cómo duplicar, quitar, copiar y pegar capas

Dónde está: pestaña **Capas**, botones **Duplicar**, **Quitar**, **Copiar las capas** y **Pegar las capas**.

Pasos:

1. **Duplicar** hace una copia de la capa elegida con un id libre.
2. **Quitar** borra la capa elegida. Si otra capa la seguía y no se pudo dejar donde estaba, un aviso lo dice; revisa dónde ha quedado.
3. **Copiar las capas** copia la capa elegida y las marcadas con Ctrl, junto con lo que usan, al portapapeles como texto.
4. **Pegar las capas** (activo cuando el portapapeles tiene capas copiadas) las pega con ids libres y copia sus archivos. Si tenías un grupo elegido, entran en él.
5. Copiar y pegar sirve entre escenas: copia en una ventana del editor y pega en otra.

Atajo: ninguno; son botones.

En la terminal: `duplicate_layer` y `remove_layer`. `paste` recibe el contenido copiado en `clip`; la copia al portapapeles la hace solo la aplicación.

## Cómo hacer que una capa siga a otra (padre)

Dónde está: barra del lienzo, botones **Seguir a una capa** y **Dejar de seguir**. También el campo **Padre** del inspector.

Pasos:

1. Elige la capa que debe seguir.
2. Activa **Seguir a una capa** y haz clic en el lienzo sobre la capa que será su padre. La capa no se mueve de su sitio; desde ahora se mueve con su padre.
3. Una línea discontinua une el pivote de la capa con el de su padre.
4. Escape cancela la elección.
5. **Dejar de seguir** (visible si la capa tiene padre) suelta la capa sin moverla.
6. También puedes elegir el padre en el campo **Padre** del inspector.

Atajo: Escape para cancelar (`SceneOverlays.qml`).

En la terminal: `layer` con `parent` (con `keep` para no mover la capa).

## Cómo usar el inspector

Dónde está: columna de la derecha.

Qué muestra: el título es el id de la capa elegida, o el camino de lo elegido (por ejemplo **Lienzo** o una línea de tiempo). Debajo hay una fila de botones y, después, los campos.

Botones de la fila:

- **Subir un nivel**: elige lo que contiene a lo elegido.
- **Subir** y **Bajar**: cambian el puesto de un elemento de una lista (una clave, un efecto, un emisor...).
- **Añadir un efecto**: abre la lista de efectos de la capa (mira [efectos](editor-efectos.md)). No aparece en las capas de sonido.
- **Quitar**: borra el elemento de lista elegido.
- **Editar el sistema de partículas**: en una capa de partículas, abre su sistema (mira [partículas y sonido](editor-particulas-y-sonido.md)).

Cada campo muestra su nombre (con la unidad entre paréntesis si la tiene) y, al pasar el ratón sobre el nombre, una ayuda. Según el campo hay un deslizador, un cuadro numérico, un selector de color, una lista de opciones, una casilla o dos cuadros para un par de valores.

Pasos:

1. Elige una capa. Cambia un campo escribiendo o moviendo el deslizador. Un arrastre largo cuenta como un solo paso del historial.
2. Un campo opcional que ya tiene valor muestra **Valor por defecto**: lo quita y vuelve al valor de fábrica. Un grupo de campos opcional muestra **Añadir** para crearlo.
3. El botón de la derecha de cada campo (◇, ◈, ◆ o ⛓) lo anima o lo enlaza. Mira [animación](editor-animacion.md) y [propiedades](editor-propiedades.md).
4. Un campo enlazado muestra un botón **Sigue X**; púlsalo para pasar a editar el enlace.

El inspector sale del formato de escena: si el formato gana un campo, aparece solo. Los campos de cada tipo de capa están en [capas](capas.md).

En la terminal: `wprs-ctl scene fields DIR PUNTERO` lista los campos que el inspector muestra, como JSON. Para cambiar uno, la acción `set` (valor), `add`, `remove` o `move` sobre un puntero JSON. Mira [acciones](acciones.md).

## Cómo hacer que una capa reciba el puntero

Dónde está: inspector de una capa de tipo grupo, imagen, vídeo, texto, forma, color sólido, espectro o malla, sección **Puntero**.

Pasos:

1. Elige una de tres opciones: **Deja pasar el puntero** (de entrada), **Toma el puntero en toda su caja** o **Toma el puntero donde es opaca**.
2. Con la tercera aparece un deslizador **Opaca desde el X %**: el puntero cuenta solo donde la capa es más opaca que ese porcentaje.
3. Con elegida una capa que toma el puntero, el lienzo dibuja el área de cada capa que toma el puntero: línea continua para toda la caja, discontinua para el umbral (con su porcentaje) y más marcada la de la capa elegida.
4. Esas áreas activan los clics y la entrada y salida del puntero en líneas de tiempo, sonidos y scripts. Hace falta el permiso del puntero. Para probarlo sin tocar la pantalla real usa [Cómo probar el puntero y los clics](#cómo-probar-el-puntero-y-los-clics).

En la terminal: `layer` con `touch` (`takes`: `off`, `all` o `alpha`; y `alpha`).


```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"boton"}'
wprs-ctl scene act mi-escena layer '{"touch":{"layer":"boton","takes":"alpha","alpha":0.3}}'
```

## Cómo hacer una plantilla

Una plantilla es una o varias capas guardadas aparte, que no se ven en la escena. Los scripts dibujan copias de ellas cuando quieren (mira [scripts](scripts.md)).

Dónde está: pestaña **Capas**, botón **Hacer plantilla**, y la lista **Plantillas** en la pestaña **Recursos**.

Pasos:

1. Elige una capa de tipo grupo, imagen, texto, forma, color sólido, espectro o malla, y marca otras con Ctrl si quieres.
2. Pulsa **Hacer plantilla**. Las capas se copian a una plantilla; las originales siguen en la escena.
3. La plantilla sale en la lista **Plantillas** de la pestaña **Recursos**, con su nombre y sus capas. Cada capa se elige con un clic y se edita con el inspector.
4. Si algún script nombra la plantilla, su nombre sale como **nombre · la nombran** seguido de los scripts.
5. Entre los botones de la pestaña **Capas** aparece una lista (solo si hay plantillas): **La escena** o **Plantilla X sola**. Elige una plantilla para verla sola en el lienzo, como se dibuja una copia, y edita sus capas con el panel de capas y las herramientas del lienzo. Elige **La escena** para volver.
6. En la lista **Plantillas** cada plantilla tiene tres botones: **Mostrarla sola**, **Cambiar el nombre…** y **Quitar**. Al renombrar, la casilla **Reescribir los nombres exactos en los scripts** cambia el nombre en los scripts que la nombran con ese texto exacto; las líneas que la nombran de otra forma se listan y no se reescriben.

En la terminal: `template` con `make`, `rename` (con `rewrite`) o `remove`. Ver una plantilla sola no tiene equivalente.


```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"estrella"}'
wprs-ctl scene act mi-escena template '{"make":{"layers":["estrella"],"name":"chispa"}}'
wprs-ctl scene act mi-escena template '{"rename":{"name":"chispa","to":"brillo"}}'
```

## Cómo compartir partes con un paquete de recursos

Un paquete de recursos es un archivo `.wpak` con capas, sistemas de partículas, scripts y efectos que otros autores pueden traer a su escena, con la licencia y el crédito de cada archivo.

Dónde está: pestaña **Recursos**, botones **Crear un paquete de recursos…** e **Importar de un paquete de recursos…**.

Para crear un paquete:

1. Elige antes la capa que quieres compartir (se marca sola).
2. Pulsa **Crear un paquete de recursos…**. En el diálogo rellena **Título:**, **Id del paquete:**, **Licencia:**, **Acreditar a:** (tu nombre, tal como te acreditarán otros autores) y **Carpeta:**.
3. Con **Vista previa:** y **Elegir una imagen…** eliges una imagen para el paquete.
4. La lista muestra las capas, los sistemas de partículas, los scripts y los efectos de la escena. Marca los que quieres. Cada parte marcada admite un **Título de la parte** y una **Miniatura…**.
5. Pulsa **Crear el paquete**. Se guarda la escena y se crea una carpeta con los archivos y los créditos. **Abrirlo en el editor** abre esa carpeta; el paquete se construye desde allí.

Para traer partes de un paquete:

1. Pulsa **Importar de un paquete de recursos…**. El diálogo lista los paquetes de recursos instalados en la biblioteca: elige uno. También puedes escribir la ruta de un archivo `.wpak` y pulsar **Usar la ruta**, o elegirlo con el selector de archivos.
2. El diálogo muestra el título, la versión, la licencia y los autores, y cada parte con su miniatura y, por cada archivo, su licencia y su crédito.
3. Marca las partes que quieres y acepta. Las partes y sus archivos se copian a tu escena, que así funciona sin el paquete; los créditos y el origen se escriben solos. Es un solo paso del historial.

En la terminal: `import_assets` (con `package` y `parts`). Los subcomandos `wprs-ctl pack new-assets` y `wprs-ctl pack parts` hacen lo mismo que los dos diálogos; mira [paquete](paquete.md) y [cli](cli.md).

## Cómo importar archivos arrastrándolos

Dónde está: sobre el lienzo y sobre la lista de la pestaña **Recursos**.

Pasos:

1. Arrastra archivos desde el explorador de archivos hasta la ventana. Al pasar por encima aparece un marco con un texto.
2. Sobre el lienzo (**Suelta los archivos para añadirlos como capas**): cada imagen (png, jpg, jpeg, webp, gif, bmp, ktx2), vídeo (webm, mp4, mkv) o sonido (ogg, opus, flac, wav, mp3) se importa y se añade como una capa. Cualquier otro tipo de archivo se importa solo como recurso.
3. Sobre la lista de recursos (**Suelta los archivos para importarlos**): todo se importa como recurso, sin crear capas.

En la terminal: `add_layer` con `file` (recurso y capa a la vez) o `import` (solo recurso).

## Cómo ver y gestionar los recursos

Dónde está: pestaña **Recursos**.

La lista muestra cada recurso con una miniatura y una línea: id, tipo, tamaño y datos extra (**HDR**, mapa cúbico, número de cuadros, carátula de lo que suena). Un clic en una fila elige el recurso y su inspector. Al final de cada fila hay botones.

Pasos:

1. **Importar…** abre el selector de archivos (imágenes, vídeos, sonidos, fuentes, tablas de color y modelos) y deja elegir varios.
2. **Importar una hoja de sprites o un mapa de datos…** importa una imagen con sus celdas o con datos para otro uso. Mira [dibujo](editor-dibujo.md).
3. **Importar un script…** trae un archivo de script JavaScript. **Nuevo script…** crea uno desde una plantilla: **Módulo vacío** o módulos que siguen al reproductor, al puntero, al sonido, a la hora del día, a un ajuste o a un contador de arranques. Mira [scripts](scripts.md).
4. En cada fila: **Cambiar el nombre…** (cambia su id y lo que lo nombra), **Cortar sprites…** y **Convertir a KTX2…** (solo imágenes) y **Quitar**.
5. **Quitar** avisa si el recurso está en uso: lista los sitios que lo usan y ofrece **Quitar de todos modos**; esos sitios se quedan sin imagen, vídeo o archivo.
6. **Borrar los archivos sin usar** elimina de la carpeta de recursos lo que la escena no usa.
7. Debajo de la lista están también las listas de scripts, plantillas, efectos del paquete y reservas.

En la terminal: `import`, `remove` (con el puntero del recurso), `rename` con `kind` `asset`, `asset`, `import_script`, `sprites` y `wprs-ctl scene prune DIR` para borrar lo que no se usa.
