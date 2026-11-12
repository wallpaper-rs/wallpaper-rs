# Editor: 3D, luces, cámara y materiales

Esta página cuenta cómo poner modelos 3D en una escena, moverlos, iluminarlos y animarlos desde el editor de escenas. Para el lienzo y el inspector mira [editor-lienzo](editor-lienzo.md); para el esqueleto de una malla plana mira [editor-dibujo](editor-dibujo.md); para las claves y los clips de la línea de tiempo mira [editor-animacion](editor-animacion.md). Los argumentos de cada acción de terminal están en [acciones](acciones.md).

Casi todo lo de esta página aparece en el panel **Inspector** (a la derecha). Algunas partes salen al elegir una capa y otras al elegir el propio lienzo.

| Lo que quieres tocar | Qué eliges |
| --- | --- |
| Un modelo, sus materiales, su animación | La capa **Modelo 3D** |
| Una luz | La capa de luz |
| Luces de toda la escena, luz ambiente, sombras, cámara 3D, recorridos, colores brillantes, coste | El lienzo (la raíz de la escena) |
| Cómo recibe luz una imagen, un vídeo o un texto | Esa capa, parte **Iluminación** |

## Cómo añadir un modelo 3D

Dónde está: botón **Añadir una capa** del panel de capas, entrada **Capa desde un archivo**, opción **Modelo 3D desde un archivo .glb…**.

Pasos:

1. Pulsa **Añadir una capa**, abre **Capa desde un archivo** y elige **Modelo 3D desde un archivo .glb…**.
2. En el diálogo **Modelo 3D desde un archivo**, pulsa **Elegir un archivo glTF binario** y elige un archivo `.glb`.
3. Antes de aceptar, el diálogo cuenta cuántos triángulos, materiales, texturas, huesos y clips tiene el archivo. Por cada nivel de equipo dice si **cabe** o es **demasiado pesado**, qué tamaño máximo de textura conserva y cuánta memoria de texturas usa.
4. Acepta. El archivo se importa a la escena y se añade una capa de modelo. Si tenías un grupo elegido, el modelo entra en él.

Los archivos FBX y OBJ no se leen directamente: ábrelos en Blender y expórtalos como glTF binario (`.glb`).

Si el menú de la capa ya tiene un archivo `.glb` importado, la entrada **Modelo 3D** de la lista de tipos también sirve: te deja elegir el archivo entre los que ya tiene la escena.

Qué verás: la capa aparece en la lista con el icono de modelo y el inspector muestra la parte **Modelo 3D** con los números del archivo y, por nivel de equipo, si cabe.

Ajustes de la capa:

- **Refleja**: elige una imagen de la escena que el modelo refleja en sus superficies brillantes (o **Nada**). Al lado se ve una miniatura.
- **Mirarlo en la vista de edición**: encuadra el modelo en la vista de edición (ver más abajo).
- Para mover, girar y escalar usa las asas de la vista de edición o los campos de posición, giro y escala del inspector.

En la terminal: acción `scene3d` con `model` (`add`, `set`, `remove`); `scene ask model_info` cuenta lo mismo que el diálogo. Ver [acciones](acciones.md).

## Cómo juntar modelos, luces y capas planas en un grupo 3D

Dónde está: la capa de tipo **Grupo 3D**, y en una capa plana dentro de un grupo 3D, la casilla **Ocultarse tras los modelos más cercanos**.

Pasos:

1. Elige un grupo. El inspector ofrece **Grupo 3D: modelos, luces y capas planas comparten profundidad**; actívalo para que el grupo sea 3D.
2. Mete dentro los modelos, las luces y las capas planas que quieras.
3. En una capa plana de ese grupo, marca **Ocultarse tras los modelos más cercanos** para que un modelo que pasa por delante la tape.

En la terminal: `scene3d` con `group`, `space` y `depth_test`.

## Cómo mover, girar y escalar un modelo con las asas

Dónde está: la vista previa. En cuanto la escena tiene un modelo, la vista previa se convierte en una vista de edición con su propia cámara y sobre ella se dibujan las asas.

Qué se dibuja:

- Cada modelo tiene una caja. Un clic en la caja lo elige.
- El modelo elegido muestra tres flechas **X**, **Y** y **Z** (rojo, verde y azul). Cada una lo mueve en esa dirección. El modelo se queda bajo el puntero sea cual sea el ángulo de la vista.
- Tres anillos, **Girar en X**, **Girar en Y** y **Girar en Z**. Arrastra a lo largo del anillo para girar el modelo alrededor de esa dirección de la escena.
- Un cuadrado **Profundidad**: arrastra hacia arriba para alejar el modelo por la línea de visión y hacia abajo para acercarlo.
- Una perilla **Girar**: de lado gira el modelo y arriba o abajo lo inclina.
- Un cuadrado **Tamaño** en la esquina de la caja: arrástralo lejos del modelo para agrandarlo y hacia dentro para reducirlo.

Las asas parten de los valores que el modelo tiene en el tiempo actual.

Pasos:

1. Elige el modelo con un clic en su caja o en la lista de capas.
2. Arrastra el asa que necesites.
3. Suelta. La posición, el giro o la escala se guardan como un solo paso de deshacer.

Si el valor tiene claves, la asa pone una clave en el tiempo actual (con el interruptor **Escribir sobre las claves** activo, pone un valor fijo encima de las claves y los enlaces). Si el valor sigue un enlace a una propiedad o a un script, la asa se ve atenuada y avisa: «La posición sigue un enlace; arrastra con Mayús para reemplazarlo por un valor». Con Mayús mientras arrastras, el enlace se sustituye por el valor. Para la escala, un enlace solo se cambia desde el inspector.

Atajos: no hay atajos de teclado propios para las asas. Los modificadores son Mayús (reemplazar un enlace) y, en las asas de la cámara, las luces y los recorridos, Ctrl (ver abajo).

En la terminal: `scene3d` con `grab` (qué asa, de dónde a dónde y el tiempo), o `scene3d` con `model` `set` para escribir posición, giro y escala directamente. Esta segunda forma rechaza escribir encima de claves o enlaces. Ver [acciones](acciones.md).

## Cómo mover algo en un plano o por la profundidad

Las asas de la cámara, de las luces 3D y de las claves de recorrido se mueven de dos maneras:

- Arrastrar normal: se mueven en el plano de la vista, es decir, paralelas a la pantalla.
- Arrastrar con Ctrl: se mueven por la línea de visión, hacia o desde quien mira.

Para un modelo no hay un asa de plano propia: usa las flechas, que mueven en una sola dirección, y el cuadrado **Profundidad**.

En la terminal: `scene3d` con `grab`. Las asas son `x`, `y`, `z`, `turn_x`, `turn_y`, `turn_z`, `plane`, `depth`, `orbit`, `dolly` y `roll`.

## Cómo moverte por la vista sin tocar la cámara de la escena

Hay dos cámaras. La vista de edición es solo tuya: sirve para mirar alrededor mientras trabajas y no sale en el fondo de pantalla. La cámara de la escena es la que verá el espectador.

Dónde está: la vista previa, cuando la escena tiene modelos.

Pasos:

1. Arrastra con el botón derecho para girar la vista alrededor de su punto de mira.
2. Arrastra con el botón central, o con Mayús y el botón derecho, para desplazarla.
3. Gira la rueda del ratón para acercarte o alejarte.

La vista previa se dibuja con esa vista. La cámara de la escena se ve como un volumen de líneas de trazos con sus asas (ver más abajo). Selecciona el lienzo para ver el panel **Cámara 3D**, que repite este consejo.

Para volver: en el panel **Cámara 3D** pulsa **Volver a la vista de la cámara**. Con **Mirar por la cámara** activo, la vista previa muestra lo que ve la cámara de la escena.

En la terminal: `scene3d` con `view` (`orbit`, `pan`, `zoom`, `frame`, `reset`, `through`).

## Cómo poner y ajustar la cámara de la escena

Dónde está: elige el lienzo; en el inspector, parte **Cámara 3D**.

Pasos:

1. Mueve los deslizadores **Campo de visión** y **Giro**; la vista previa cambia mientras los mueves.
2. Mira el texto «Desde … hacia …», que da el punto desde donde mira la cámara y el punto al que mira.
3. Coloca la vista de edición donde quieras y pulsa **Poner la cámara desde la vista**: la cámara de la escena toma esa posición.
4. Con **Mirar por la cámara** compruebas el resultado; **Volver a la vista de la cámara** devuelve la vista de edición.

También puedes mover la cámara con asas sobre la vista previa (con **Mirar por la cámara** apagado):

- **Cámara**: perilla en el ojo de la cámara. Arrastra para moverla en el plano de la vista; con Ctrl, por la línea de visión.
- **Mira**: cuadrado en el punto que mira la cámara.
- **Giro**: anillo alrededor de su línea de visión. Arrastra a lo largo del anillo para inclinar la cámara.
- **Órbita**: arrastra para hacer girar la cámara alrededor de lo que mira, manteniendo la distancia.
- **Acercar**: arrastra hacia arriba para acercarla a lo que mira y hacia abajo para alejarla.

En la terminal: `scene3d` con `camera` (`eye`, `center`, `up`, `fov`, `roll`, `from_view`) y con `grab` (`orbit`, `dolly`, `roll`, `plane`); `scene ask stage3d` lista las asas.

La **Cámara 2D** (zoom, giro y desplazamiento de una escena plana) es otra cosa; está en [editor-lienzo](editor-lienzo.md).

## Cómo hacer un recorrido de cámara

Un recorrido es una lista de claves de vista: cada clave guarda dónde está la cámara y hacia dónde mira, y la cámara viaja de una a otra.

Dónde está: lienzo, parte **Cámara 3D**, bloque **Recorridos de cámara**.

Pasos:

1. Coloca la vista donde debe empezar el recorrido (puedes usar **Poner la cámara desde la vista**).
2. Escribe un nombre en **Nombre de un recorrido nuevo** (letras minúsculas, cifras y guion bajo, hasta 32 caracteres) y pulsa **Añadir desde la vista**.
3. En la fila del recorrido ves su nombre y su duración, la casilla **Bucle** y el botón de quitar. Debajo, sus claves.
4. Mueve la vista a otro sitio y pulsa **Clave de la vista a los N s** en esa fila para añadir una clave en el tiempo actual.
5. Para quitar una clave pulsa **Quitar esta clave** en su botón.
6. Si hay varios recorridos, elige **En orden** o **Al azar** para el orden, y entre **Corte**, **Pasando por negro** y **Cruzado** para el paso de uno a otro; el cuadro numérico de al lado da la duración de ese paso en segundos.
7. Pulsa **Recorrer los caminos desde la cámara** para ver el vuelo desde el segundo cero.

En la vista previa cada recorrido dibuja su curva, una perilla en cada clave (el ojo en ese momento) y un cuadrado en el punto que mira, unidos por una línea de trazos. Arrastra una perilla para mover el ojo de esa clave; con Ctrl, por la línea de visión. El rótulo es **Camino N a los X s** y **Mira a los X s** en el cuadrado. Un cuadro **Cámara ahora** marca dónde ponen los recorridos la cámara en el tiempo actual.

La línea de tiempo de la escena tiene un carril por recorrido con sus claves: arrastra una a otro tiempo, o haz clic para ir a ese momento en la vista previa.

En la terminal: `scene3d` con `path` (`add`, `key`, `remove`, `set`, `order`, `fade`) y la consulta `paths3d`.

## Cómo añadir y ajustar una luz

Hay cuatro tipos de luz: **Puntual**, **Foco**, **Tubo** y **Direccional**.

Dónde está: botón **Añadir una capa**, submenú **Luz**: **Luz puntual**, **Foco**, **Tubo de luz** y **Luz direccional**.

Pasos:

1. Elige el tipo en el menú. La luz entra como una capa nueva (dentro del grupo si tenías un grupo elegido).
2. En el inspector, parte **Luz**: cambia el tipo con la lista (**Punto**, **Foco**, **Tubo**, **Direccional**) y el color con el selector de al lado.
3. Ajusta los deslizadores:
   - **Intensidad**.
   - **Radio**: hasta dónde llega.
   - **Altura**: la distancia al plano del lienzo.
   - **Longitud**: solo en el tubo.
   - **Centro brillante** y **Cono**: solo en el foco, el ángulo de la parte fuerte y el del haz entero.
   - **Inclinación hacia el lienzo**: en el foco y la direccional, cuánto mira hacia el lienzo.
4. El gráfico **Luz según la distancia** muestra cómo cae la luz al alejarse.
5. Casillas al final: **Mostrar la parte que redibuja al moverse**, **Iluminar la vista previa solo con ella** (solo) y **Dejarla fuera de la vista previa** (silenciar).

Sobre la vista previa cada luz tiene una marca. Arrástrala para moverla (**Arrastra para mover la luz**). La luz elegida dibuja sus asas:

- Un anillo con una perilla en su radio.
- Un cuadrado encima cuya distancia da la altura (**Altura N**: arrastra arriba o abajo para subirla o bajarla).
- En el foco: una perilla que lo gira (**Arrastra para girar la luz**), los bordes del cono exterior y del centro brillante con una perilla en cada uno (**Arrastra para abrir o cerrar el cono** y **el centro brillante del cono**), y una perilla en la línea de la dirección para la inclinación (**Inclinación de N°**).
- En el tubo: una perilla en cada extremo que lo mueve, lo gira y lo alarga.
- En la direccional: una flecha con una perilla que la gira.

Si el valor tiene claves o la luz está animada, las asas ponen una clave en el tiempo actual, igual que en las demás capas. Las luces aparecen también en la línea de tiempo.

En la terminal: acción `light` (`add`, `set`, `remove`, `solo`, `mute`). La consulta `lights` lista las luces y `falloff` da la curva de caída. Ver [acciones](acciones.md).

## Cómo ver todas las luces y su límite

Dónde está: elige el lienzo; en el inspector, parte **Luces**.

Qué ves:

- Una fila por luz con su nombre, su tipo y las marcas **aire** y **proyecta** si las usa. Un clic elige la luz. Cada fila tiene botones **Solo** y **Silenciar**.
- «N de M luces como mucho»: cuántas luces tiene la escena frente a las 12 permitidas.
- Por nivel de equipo, cuántas luces se conservan y cuántas quedan fuera.
- El selector **Calidad de la vista previa** (**Q0, la más baja** a **Q3, la más alta**) con el botón **Reiniciar la vista previa**. Debajo, por nivel de calidad, si la luz se calcula una vez al empezar la escena (Q0) o cuántas luces por píxel se calculan en cada fotograma. Una línea aparte cuenta lo mismo para teléfonos.

En la terminal: consulta `lights` y `lighting` con `time` y `quality`.

## Cómo hacer que un foco proyecte una imagen

Dónde está: inspector del foco, parte **Proyecta**.

Pasos:

1. Elige el foco.
2. En **Proyecta**, elige una imagen o un vídeo de la escena, o una capa que pueda proyectarse (sin bucle). **Nada** lo apaga.
3. La vista previa dibuja dentro del cono, con un contorno de trazos, lo que cae sobre el lienzo. Para un vídeo se muestra el fotograma del cabezal.
4. Si el haz corre a lo largo del lienzo, la imagen se dibuja hasta el radio de la luz, más tenue, con un aviso.

Solo el punto y el foco, y no el tubo, ofrecen aire; solo el foco proyecta.

En la terminal: acción `light` con `projection`; consulta `projection`.

## Cómo hacer luz con aire, haces y volumen

Dónde está: inspector de una luz puntual o un foco, parte **Iluminar el aire**.

Pasos:

1. Activa **Iluminar el aire**.
2. Ajusta **Densidad**, **Muestras** y **Haces**. Con **Haces** el aire se oscurece detrás de lo oscuro de las capas de debajo, y una línea de trazos en el gráfico muestra el resplandor sobre una de esas partes oscuras. Las capas por encima de la luz la tapan.
3. El gráfico dibuja el resplandor según la distancia.
4. Mira las líneas por nivel de calidad: **apagado** en Q0, **la mitad** o **un cuarto** del tamaño con sus muestras. Los teléfonos y los equipos más antiguos usan un cuarto.

La vista previa muestra el resplandor en vivo con el nivel de calidad elegido en el panel **Luces**.

En la terminal: acción `light` con `volumetric`; consulta `air`.

## Cómo dar luz ambiente y de cielo

Dónde está: lienzo, panel **Luces**.

Pasos:

1. Cambia el color de **Luz ambiente** (la que llega de todas partes).
2. Cambia el color de **Luz del cielo** (la que llega desde arriba).
3. La esfera de al lado se ilumina con ambos colores para que veas el efecto.

En la terminal: acción `ambient`; consulta `hemisphere`.

## Cómo hacer que una capa reciba la luz

Dónde está: capa de tipo imagen, vídeo o texto, parte **Iluminación**. Las partículas también tienen la parte, con el interruptor y **Resplandor** y **Fuerza**.

Pasos:

1. Activa **Recibir la luz de las capas de luz**.
2. Elige, si los necesitas, los mapas: **Mapa de normales**, **Imagen reflejada**, **Mapa de rugosidad** y **Mapa de luz propia** (**Sin fijar** quita el mapa). Si una imagen se lee como colores y sombrea mal, aparece un aviso: vuelve a elegirla para leerla como mapa.
3. Ajusta los deslizadores **Rugosidad**, **Reflejo**, **Resplandor** y **Fuerza**.

En la terminal: acción `lighting`.

### Generar un mapa de normales

1. Con la capa elegida, pulsa **Generar normales…**.
2. En el diálogo (**Generar normales a partir de la imagen**, **del fotograma** o **de las letras**) ajusta **Fuerza** y **Desenfoque**. Ves el relieve iluminado desde arriba a la izquierda, el propio mapa y una esfera iluminada antes de guardar.
3. Pulsa **Generar**. El mapa queda como imagen de la escena.

En la terminal: acción `normals`.

### Pintar rugosidad y luz propia

1. En la parte **Iluminación**, pulsa **Pintar la rugosidad sobre la vista previa** o **Pintar la luz propia sobre la vista previa**.
2. Ajusta **Tamaño del pincel**, **Dureza** y **Gris** (el valor que se pinta).
3. Pinta sobre la capa en la vista previa. Mientras pintas, el mapa se ve encima como mapa de calor.

En la terminal: acción `paint_map`; consulta `map`.

Las partículas tienen sus propias opciones de luz y refracción; ver [editor-particulas-y-sonido](editor-particulas-y-sonido.md).

## Cómo dar sombra a las luces

Dónde está: lienzo, panel **Luces**, bloque **Sombras**.

Pasos:

1. En la fila de cada luz puntual, foco o direccional, activa **N da sombra**.
2. Elige el tamaño del mapa de sombra: **Automático** o de 256 a 4096 px. El texto de al lado dice qué tamaño usa cada nivel de equipo, en teléfonos y en Q0.
3. Ajusta **Sesgo** si ves rayas o sombras despegadas.
4. Pulsa **Mostrar dónde caen las sombras y los volúmenes de los mapas**: en la vista previa los lugares iluminados se ven blancos y los de sombra rojos, y la vista de edición dibuja los volúmenes de los mapas con líneas de trazos.

El bloque avisa de cuántas luces pueden dar sombra como máximo.

En la terminal: `scene3d` con `shadows`; consulta `shadows3d`.

## Cómo poner una luz dentro de la escena 3D

Una luz normal se queda en el plano del lienzo. En 3D tiene su propio punto entre los modelos.

Dónde está: lienzo, panel **Luces**, bloque **Luces en 3D**: una casilla **N en 3D** por luz.

Pasos:

1. Activa la casilla de la luz. Desmárcala para devolverla al plano.
2. En la vista de edición la luz tiene una perilla donde ilumina a los modelos (atenuada mientras está en el plano: arrastrarla la pone en 3D). Los focos y las luces direccionales tienen además un cuadrado **Mira**, el punto al que apuntan, con una línea de trazos entre los dos.
3. Arrastra la perilla o el cuadrado en el plano de la vista; con Ctrl, por la línea de visión.

Si la posición está animada, se pone una clave en el tiempo actual.

En la terminal: `scene3d` con `light` (`place`, `flat`, `position`, `aim`) y con `grab` (`what` `light`).

## Cómo ajustar un material

Dónde está: inspector de la capa de modelo, parte **Materiales**. Hay una fila por material del archivo.

Qué trae cada fila:

- Una bola iluminada con el material y con la luz ambiente y del cielo de la escena.
- Una lista con el tipo de material: **Físico**, **Sin luz**, **Pelo**, **Vegetación**, **Croma** y los shaders de material del paquete.
- Casillas **Iluminado**, **Ambos lados**, **Proyecta sombras** y **Recibe sombras**.
- Colores **Tinte** y **Resplandor**.
- Deslizadores **Metal**, **Rugosidad**, **Relieve** y **Oclusión**.
- Las miniaturas de los mapas del archivo: **Color**, **Metal y rugosidad**, **Normales**, **Oclusión** y **Resplandor**.

Pasos:

1. Elige el modelo.
2. En **Materiales**, busca la fila del material y cambia lo que necesites.

En la terminal: `scene3d` con `material`; consulta `materials3d`.

### Tipo Pelo

Con el tipo **Pelo** aparecen:

- **Capas**, **Largo, en centésimas del modelo** y **Densidad**; abajo, cuántas capas dibuja cada nivel de calidad.
- Un cuadro **Peinado**: arrastra para peinar el pelo sobre la superficie; el deslizador **Hacia quien mira** peina hacia el espectador.
- El pincel **Peinar sobre el modelo**: actívalo y arrastra sobre el modelo en la vista. El trazo se dibuja con un anillo del tamaño del pincel y, al soltar, el pelo se peina por donde pasó. Cada trazo se puede deshacer. Ajusta **Tamaño del pincel, en píxeles** y **Fuerza del peinado**; **Borrar** deshace el peinado por donde pases y **Borrar el peinado** lo quita todo.

En la terminal: `scene3d` con `comb`. Para pintar otro mapa del material hay `paint`.

### Tipo Vegetación

Muestra **Vaivén** y **Velocidad**, y el viento del lienzo con su flecha.

### Tipo Croma

Quita un color del modelo. Elige el **Color clave**, o pulsa **Tomar el color clave del modelo** y haz clic sobre el modelo en la vista. Ajusta **Tolerancia** y **Difuminado**.

En la terminal: `scene3d` con `key` (tomar el color de un punto) y `material`.

### Vistas de depuración

Cada fila de material tiene la lista **Vista de depuración**: **Ninguno**, **Máscara de balanceo de los colores de vértice**, **Mate** (la transparencia tras el croma) y **Dónde caen las sombras**. Solo ese material la muestra, y solo en la vista previa del editor; el fondo de pantalla nunca la muestra.

En la terminal: `scene3d` con `debug`.

## Cómo escribir el código de un shader de material

Dónde está: inspector del modelo, **Materiales**, bajo las filas.

Pasos:

1. Escribe un nombre en **Nombre de un shader de material nuevo** y pulsa **Shader de material nuevo**. Se crea una plantilla.
2. El shader aparece con una línea que dice si **se compila** o el error con línea y columna.
3. Pulsa **Editar el código WGSL…** para abrir el editor de código, o **Editar el descriptor…** para sus parámetros. **Comprobar de nuevo** repite la comprobación.
4. En el editor, escribe. Mientras escribes se comprueba el código y se marca la línea que falla (**Ir a la línea**). Con **Vista previa al escribir: guardar cada versión que compila** se guarda cada versión que compila. **Guardar** guarda y se puede deshacer; **Usar en la capa elegida** lo aplica a la capa; **Abrirlo en tu editor…** entrega el archivo al editor del escritorio.
5. El shader escribe `fn material(s: Surface) -> Surface` en `effects/NOMBRE.wgsl`. Lee `frame`, de `param(0u)` a `param(7u)` y hasta cuatro imágenes de sus parámetros de textura con `image0(uv)` a `image3(uv)`.
6. En la fila de un material, elige el shader de la lista. Sus parámetros salen como deslizadores, selectores de color, casillas o listas de imágenes.

El shader corre en la tarjeta gráfica dentro de los límites de los efectos de paquete; usa solo shaders en los que confíes. Más sobre el código de shaders en [editor-efectos](editor-efectos.md).

En la terminal: acción `shader` (`new`, `write`, `param`); consultas `shader_source`, `shader_check` y `material_check`.

## Cómo animar un modelo

Dónde está: inspector de la capa de modelo, parte **Animación** (solo si el modelo tiene esqueleto, clips o formas de mezcla).

Pasos:

1. La lista muestra los clips de la capa con su duración y de dónde vienen. **En la línea de tiempo** abre ese clip para editarlo (ver [editor-animacion](editor-animacion.md)).
2. Para traer una animación del archivo, abre **Clip nuevo del archivo…** y elige una.
3. Activa **Movimiento de raíz: el clip mueve la capa** si el clip debe desplazar el modelo. En la vista de edición se dibuja el camino que recorrerá hacia delante, mezclando todas las capas de animación por su peso.
4. Si el modelo tiene formas de mezcla, cada una tiene un deslizador **Forma N** de peso entre -1 y 1.

### Capas de animación

La parte **Capas de animación** mezcla varios clips a la vez. Primero añade un clip. Por cada capa:

- El clip que reproduce, y **Encendido** y **Suma** (sumar sobre las demás en vez de mezclar).
- Subir, bajar y quitar.
- Deslizadores **Peso**, **Velocidad**, **Entrada gradual** y **Salida gradual**.
- **Solo estos huesos**: limita la capa a unos huesos.
- **Añadir una capa que reproduzca…** crea una capa nueva.

En la terminal: acciones `animate` (`layer`, `root`, `preset`, `morph`), `clip` y `key`; consulta `motion`.

## Cómo editar los huesos de un modelo

Dónde está: la vista de edición, con el modelo elegido.

El modelo elegido dibuja sus huesos: una línea de cada hueso a su padre y una perilla en cada uno. Un clic en la perilla elige el hueso para la línea de tiempo y para los selectores. El hueso elegido muestra:

- Un anillo: arrastra el anillo para girar el hueso en la vista; arrastra dentro para inclinarlo como una bola de seguimiento.
- Un cuadrado **Mover** en el centro: lo mueve en el plano de la vista.

Cada arrastre pone una clave en el clip elegido en el tiempo actual. Un modelo sin clips recibe el clip `clip_1`.

En la terminal: acción `key` (ángulos y desplazamiento del hueso en un clip); consulta `bone3d`.

### Huesos que se balancean

Dónde está: parte **Animación**, bloque **Huesos que se balancean**.

Pasos:

1. Elige un hueso en el selector.
2. Pulsa un preajuste: **Pelo**, **Cola**, **Tela**, **Antena**, **Cadena** o **Gelatina**. El hueso pasa a balancearse como ese tipo de cosa.
3. Cada hueso que se balancea tiene su lista de modo (**Muelle**, **Rígido**, **Cuerda**), el botón de quitar y los deslizadores **Rigidez**, **Amortiguación**, **Inercia**, **Gravedad** y **Viento**, más los límites de su giro y de su deslizamiento.
4. **Mostrar el viento en la vista previa** dibuja la flecha del viento de la escena.
5. En la vista de edición, un hueso que se balancea dibuja el abanico que su giro permite, en su propio plano, con perillas **Mínimo** y **Máximo** que arrastran el límite. Si tiene un límite de deslizamiento, dibuja la caja en la que su cabeza puede moverse y el hueso elegido tiene perillas **Mínimo** y **Máximo** en su borde.

El balanceo se detiene en el nivel de calidad más bajo, donde los huesos mantienen su animación.

En la terminal: acciones `physics` (con límite de giro) y `wind`.

### Colisionadores

Dónde está: parte **Animación**, bloque **Colisionadores**.

Pasos:

1. Elige un hueso. El botón dice **Añadir un colisionador en NOMBRE**.
2. Elige **Cápsula a lo largo del hueso** o **Esfera en su cabeza**, y pulsa añadir.
3. Ajusta el ancho con el deslizador, o en la vista de edición arrastrando la perilla del borde (**Arrastra para elegir el ancho del colisionador**). El colisionador se dibuja con círculos de trazos.
4. **Quitar** lo borra.

Los huesos que se balancean quedan fuera de los colisionadores: todos en los dos niveles de calidad más altos, los dos primeros en el penúltimo y ninguno en el más bajo.

En la terminal: acción `collider`.

### Cadenas que alcanzan

Dónde está: parte **Animación**, bloque **Cadenas que alcanzan**.

Pasos:

1. Elige un hueso que tenga padre. El botón dice **Cadena nueva de A a B**.
2. Elige el objetivo: **Un punto**, **El puntero**, un hueso del modelo o una capa.
3. Ajusta el **Peso**; **Quitar** borra la cadena.
4. Si el objetivo es un punto, arrástralo en la vista (**Arrastra el punto que busca la cadena**).

En la terminal: acción `ik`.

### Esculpir una forma de mezcla

Dónde está: parte **Animación**, bloque **Esculpir una forma de mezcla**.

Pasos:

1. Escribe un nombre en **Nombre de la forma**.
2. Pulsa **Esculpir sobre la vista** y elige **Empujar**, **Hundir** o **Suavizar**.
3. Ajusta **Tamaño** y **Fuerza** y arrastra sobre el modelo en la vista de edición. Cada trazo mueve los puntos bajo el pincel a lo largo de la perpendicular a la superficie, o los empareja, y se guarda en un archivo junto al modelo. **Deshacer** quita el trazo.
4. La forma tiene después su deslizador de peso.

En la terminal: acción `shape_sculpt` y `scene3d` con `sculpt`.

## Cómo conservar colores más brillantes que el blanco

Dónde está: lienzo, parte **Colores brillantes**.

Pasos:

1. Activa **Conservar colores más brillantes que el blanco**.
2. Elige el **Mapeo de tonos**: **Ninguno**, **Fílmico (ACES)** o **Suave (AgX)**.
3. Mueve **Antes y después**: la vista previa muestra la parte izquierda sin mapeo de tonos, para comparar.

Los teléfonos guardan colores de 8 bits antes del mapeo de tonos. En las pantallas HDR los colores más brillantes que el blanco se conservan y se doblan con suavidad hasta el brillo de cada pantalla, en lugar de este mapeo; la vista previa del editor muestra la imagen normal. Si alguna salida tiene HDR activo, el panel dice «HDR activado en: …».

En la terminal: `scene3d` con `tones` (`hdr`, `tonemap`, `split`).

## Cómo saber cuánto cuesta el 3D

Dónde está: lienzo, panel **Luces**, tras el selector **Calidad de la vista previa**. Aparece la parte **Modelos 3D** y una línea por nivel de calidad.

Qué cuenta:

- Por cada nivel de calidad: los triángulos, los grupos 3D, las unidades de trabajo de los modelos y si se dibujan reflejos de imágenes (**con reflejos** o **sin reflejos de imágenes**).
- Por cada nivel de equipo, a qué calidad empieza, si cada modelo **cabe**, **muestra su póster** o es **demasiado pesado**, la memoria de texturas y cuántas luces dan sombra.
- Las sombras (mapas de sombra de N megapíxeles), las capas de pelo y los triángulos de pelo que cada mapa de sombra añade, y los vértices con piel y huesos que se balancean.
- Las luces con aire, los píxeles que copian los haces, las luces sobre partículas y la refracción.

Cuando el coste es demasiado, baja el nivel de calidad, quita luces o reduce capas de pelo y tamaños de textura.

En la terminal: consultas `cost3d`, `lighting` y `budget`. Ver [acciones](acciones.md).
