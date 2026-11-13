# Partículas y sonido en el editor

Cómo crear y ajustar sistemas de partículas, capas de sonido y enlaces que siguen el sonido, con las herramientas del editor de escenas. Para abrir el editor, el lienzo y el inspector, mira [editor](editor.md) y [editor-lienzo](editor-lienzo.md). Para el formato de los datos, mira [particulas-y-sonido](particulas-y-sonido.md). Las acciones de la terminal están en [acciones](acciones.md).

## Cómo funcionan las partículas

Un **sistema de partículas** es la receta: cuántas nacen, desde dónde, cómo se mueven, cuánto viven y cómo se dibujan. Una **capa de partículas** dibuja un sistema en el lienzo. Un sistema puede existir sin capa (por ejemplo, para que otro lo inicie). Varias capas pueden dibujar el mismo sistema, cada una con sus propios retoques.

Los controles de un sistema aparecen en el inspector cuando eliges ese sistema. Algunos tienen herramientas sobre la vista previa (asas que arrastras). Las asas están en píxeles del lienzo y respetan la transformación de la capa.

## Cómo añadir una capa de partículas

**Dónde está.** Botón **Añadir una capa** de la lista de capas.

**Pasos**

1. Pulsa **Añadir una capa**.
2. Elige **Partículas**.
3. La capa nace con un sistema que ya funciona: nieve que cae de arriba abajo, ya visible desde el primer cuadro, con turbulencia suave y colores y tamaños de salida.
4. En **Empezar con…** del inspector elige **Nieve**, **Chispas**, **Lluvia** o **Polvo** para rellenar de una vez los emisores, la vida y las fuerzas con un punto de partida; después ajústalo a tu gusto.

**En la terminal.** Acción `add_layer` con `kind` igual a `particles`, y la variante `kit` de `particles` para el punto de partida. Mira [acciones](acciones.md).

## Cómo abrir el sistema de una capa

**Dónde está.** Inspector de la capa de partículas.

**Pasos**

1. Selecciona la capa de partículas.
2. Pulsa el botón **Editar el sistema de partículas** (icono de copo de nieve, arriba del inspector).
3. El inspector pasa a mostrar los ajustes del sistema.

La capa en sí muestra el bloque **Esta capa sobre su sistema**, que se explica más abajo.

**En la terminal.** `wprs-ctl scene particles DIR SISTEMA handles` imprime lo que el editor dibuja para ese sistema. Mira [acciones](acciones.md).

## Cómo ver, crear, copiar, renombrar y quitar sistemas

**Dónde está.** Botón **Sistemas de partículas** (icono de copo, solo icono) bajo la lista de capas. Su ayuda dice «Crear, copiar, renombrar y quitar los sistemas de partículas de la escena». Muestra **Sistemas de partículas de la escena**. También aparece arriba del inspector de cualquier sistema.

**Pasos**

1. Pulsa el botón **Sistemas de partículas**. Si no hay ninguno, lee **La escena todavía no tiene sistemas de partículas.**
2. Cada fila tiene el nombre del sistema en un campo y cuatro botones:
   - **Editar este sistema**: lo abre.
   - **Copiar este sistema**: crea una copia con nombre nuevo y la abre.
   - **Añadir un sistema que este inicia**: crea un sistema hijo y lo abre.
   - **Quitar el sistema**.
3. Para renombrar, cambia el texto del campo y confirma. Los nombres admiten letras minúsculas, cifras y guion bajo, hasta 32 caracteres.
4. Para crear uno sin capa, pulsa **Añadir un sistema sin capa**.

Si algún script todavía nombra el nombre viejo, el editor lo avisa con cada línea y el botón **Reescribir los nombres exactos**.

**En la terminal.** Acción `particles` con las claves `create` (con `from` para copiar o `parent` para hijo), `delete`, `rename` y `scripts`. También `wprs-ctl scene particles DIR SISTEMA create|delete|rename`. Mira [acciones](acciones.md).

## Cómo ajustar un sistema

**Dónde está.** Inspector del sistema, bloque **Ajustes de este sistema**.

**Pasos**

1. **Máximo de partículas vivas**: el tope de partículas a la vez (de 1 a 200 000).
2. **Semilla**: el número que decide el azar. Escribe uno, pulsa **Elegir una semilla al azar** o pulsa **Usar la semilla de la vista previa** (aparece cuando la de la vista previa es otra) para fijar la que ves ahora.
3. **Segundos simulados antes de mostrarse**: cuántos segundos corre el sistema antes de verse, para que no empiece vacío.
4. **Las partículas se mueven con**: **La capa** (si mueves la capa, se mueven con ella) o **El sitio donde nacieron** (la capa y los puntos de control solo colocan las partículas cuando nacen).

**En la terminal.** Acción `particles`, clave `settings`. Mira [acciones](acciones.md).

## Cómo elegir la forma de un emisor

Un sistema tiene de uno a cuatro emisores. Cada uno es un bloque **Emisor 1**, **Emisor 2**, etc., bajo **Emisores**. El panel no añade ni quita emisores: para añadir uno se escribe la forma con el número siguiente por la terminal (acción `particles`, clave `shape`, con el índice del emisor nuevo).

**Dónde está.** Inspector del sistema, bloque **Emisores**.

**Pasos**

1. En el emisor, abre la lista de formas y elige una:
   - **Punto**: nacen en un punto.
   - **Caja**: dentro de un rectángulo.
   - **Círculo**: dentro de un círculo.
   - **Línea**: sobre una línea.
   - **Borde**: en el borde del lienzo.
   - **Alrededor de un punto**: sobre un arco alrededor de un punto de control.
   - **Entre dos puntos**: a lo largo de una línea entre dos puntos.
   - **Desde una capa**: desde los píxeles de otra capa.
2. En el lienzo aparecen las asas de la forma. Arrástralas: la esquina de una caja, el centro y el radio de un círculo, los extremos de una línea.
3. En **Alrededor de un punto** y **Entre dos puntos**, elige en las listas a qué punto de control siguen (o **un punto**, fijo), y escribe cuántas partículas nacen en el número de la derecha (de 1 a 4096). Un extremo ligado a un punto de control mueve ese punto cuando lo arrastras.

**En la terminal.** Acción `particles`, clave `shape`. Mira [acciones](acciones.md).

## Cómo emitir desde los píxeles de otra capa

**Dónde está.** Emisor con forma **Desde una capa**.

**Pasos**

1. Elige la **Capa** que sirve de origen (nunca la que dibuja este sistema).
2. **Umbral**: nacen partículas en los píxeles más opacos que ese valor. La vista previa marca esos píxeles.
3. Marca **Su color** para que cada partícula nazca con el color del píxel.
4. Marca **Su movimiento** para que herede el movimiento de la capa.
5. **Volver a leer**: **Una vez** o **Cuando cambia la capa**.

Si la capa no se puede leer píxel a píxel (un modelo 3D o un texto dinámico como un reloj), la vista previa marca solo su caja y el motor emite desde sus píxeles opacos.

**En la terminal.** Acción `particles`, clave `layer`. También `wprs-ctl scene particles DIR SISTEMA emit-from-layer`. Mira [acciones](acciones.md).

## Cómo ajustar el ritmo de un emisor

**Dónde está.** Dentro de cada bloque **Emisor**.

**Pasos**

1. El control deslizante da el ritmo en «N por segundo». Si el ritmo sigue claves de la línea de tiempo o un enlace, se lee **El ritmo sigue claves o un enlace** y el control está bloqueado: pulsa **Ritmo fijo** para reemplazarlo por un número.
2. Marca **Máximo de partículas que nacen en un paso** y fija el número, para que un salto grande de ritmo no suelte muchas de golpe.
3. Marca **Seguir el puntero** para que el emisor siga al puntero de usuario.
4. Marca **Solo mientras suena algo** para que emita solo si hay sonido.
5. **Velocidad tomada del emisor**: qué fracción de la velocidad del puntero heredan las partículas (de 0 a 1). Sirve con **Seguir el puntero**.

**En la terminal.** Acciones `particles`: `settings` (con `emitter` y `rate`), `pace` y `emit`. Mira [acciones](acciones.md).

## Cómo dar ráfagas y pausas a un emisor

**Dónde está.** Dentro de cada **Emisor**: una línea de tiempo corta y la lista **Ráfagas**.

**Pasos para la línea de tiempo**

1. Arrastra el inicio para **Empezar a los N s**.
2. Marca **Parar al cabo de un rato** y arrastra el final para **Parar a los N s**.
3. Marca **Emitir en tramos y pausas** para alternar tramos de emisión y pausas. Arrastra los extremos de los tramos y las pausas: la fila de arriba es la duración más corta y la de abajo la más larga.
4. La línea muestra «Se muestran N s» según su escala. Cada arrastre es un solo paso del historial.

**Pasos para las ráfagas**

1. Pulsa **Añadir una ráfaga** (hasta 8 por emisor).
2. Escribe cuántas partículas suelta.
3. Elige cuándo se dispara: **con el tiempo**, **con el ritmo** (el pulso del sonido) o **con un clic**.
4. Con «con el tiempo», fija **Tras N s** y, si marcas **Repetir**, **Cada N s**. Sobre la línea se ven como un punto (el primer disparo) y cuadrados (las repeticiones); arrastrarlos cambia esos tiempos.
5. **Quitar esta ráfaga** la elimina.

**En la terminal.** Acción `particles`, clave `emit`. Mira [acciones](acciones.md).

## Cómo usar puntos de control

Los **puntos de control** son marcas (`p0` a `p7`) que pueden usar las formas, las fuerzas y los colisionadores.

**Dónde está.** Inspector del sistema, bloque **Puntos de control**.

**Pasos**

1. Pulsa **Añadir un punto de control** (hasta 8).
2. Arrástralo en el lienzo. Con una capa de partículas seleccionada, el punto que arrastras es solo de esa capa; sin capa, es del sistema.
3. Marca **Sigue al puntero** para que siga al puntero de usuario.
4. Usa el ángulo (**Giro de N°**) para girarlo.
5. **Quitar** borra el último punto de la lista (el primero no se puede quitar).

**En la terminal.** Acción `particles`, clave `point`. También `wprs-ctl scene particles DIR SISTEMA control-point`. Mira [acciones](acciones.md).

## Cómo fijar con qué empieza cada partícula

**Dónde está.** Inspector del sistema, bloque **Rangos iniciales**.

**Pasos**

1. Cada fila es un rango con dos asas: **Vida en segundos**, **Velocidad**, **Dirección**, **Tamaño**, **Giro**, **Velocidad de giro** y **Alfa**. Cada partícula toma un valor entre los dos extremos.
2. **Curva**: con el control deslizante inclinas el reparto. Por debajo de 1 se eligen más a menudo valores cerca del extremo alto, por encima de 1 cerca del bajo.
3. En cada extremo puedes dejar **Extremo bajo fijado aquí** o **Sigue AJUSTE**, para que ese extremo siga un ajuste de usuario (mira [editor-propiedades](editor-propiedades.md)).

**En la terminal.** Acción `particles`, clave `start`. También `wprs-ctl scene particles DIR SISTEMA start`. Mira [acciones](acciones.md).

## Cómo elegir los colores de salida

**Dónde está.** Bloque **Rangos iniciales**, debajo de las filas, parte **Colores iniciales**.

**Pasos.** Elige un modo en la lista:

- **Entre dos colores**: dos selectores de color; cada partícula toma uno entre ambos.
- **Tonos, saturación y brillo**: tres rangos con dos asas (**Tono**, **Saturación**, **Valor**). Marca **Elegir el tono entre pasos** para que el tono salte entre un número de valores fijos.
- **De una paleta**: una lista de colores con **Añadir un color** y **Quitar el color**, y tres controles de ruido (**Ruido del tono**, **Ruido de la saturación**, **Ruido del brillo**) que desvían un poco cada partícula.
- **A lo largo de un degradado**: una barra de degradado para editar las paradas.

**En la terminal.** Acción `particles`, clave `start`, con `ranges` de color. Mira [acciones](acciones.md).

## Cómo girar o desplazar el nacimiento con un ruido

**Dónde está.** Inspector del sistema, bloque **Direcciones de salida y desfases**.

**Pasos para girar las direcciones**

1. Marca **Girar las direcciones de salida con un ruido**.
2. Ajusta **Dirección**, **Girar hasta**, **Velocidad del ruido**, **Dispersión entre partículas** y **Fase**.
3. La vista previa dibuja un abanico con las direcciones posibles, y rayos que son las primeras partículas nacidas con la semilla actual. Con una flecha arrastras la dirección, y con un asa del borde abres o cierras el abanico.
4. Pulsa **Conservar esta semilla** para que cada inicio de la escena dibuje lo que ves.
5. En **Sonido en los rayos**, elige una banda (**Sin sonido**, **Graves**, **Medios**, **Agudos**, **Nivel**, **Pulso**) para que el ruido se mueva con el sonido. Un control deslizante dice cuánto suena esa banda para los rayos de la vista previa. Con **El sonido multiplica la fase**, el sonido mueve la fase.

**Pasos para desplazar nacimientos**

1. Marca **Mover los nacimientos con un ruido fractal**.
2. Ajusta **Distancia**, **Tamaño del patrón**, **Detalle**, **Dispersión** y **Deriva**.
3. Elige hacia dónde mueve: **A lo ancho**, **A lo alto**, y de qué lado (**Ambos lados**, **Lado positivo**, **Lado negativo**).
4. Sobre la vista previa se ven flechas desde los lugares de nacimiento hasta donde los lleva el ruido, y un anillo con un asa para la distancia.

**En la terminal.** Acciones `particles`: `turbulent` y `offset`. También `wprs-ctl scene particles DIR SISTEMA turbulent|offset`. Mira [acciones](acciones.md).

## Cómo reasignar valores

Una **reasignación** convierte una entrada (por ejemplo, el azar o el sonido) en un valor de la partícula (por ejemplo, el tamaño).

**Dónde está.** Dos bloques del inspector:

- **Valores iniciales desde entradas**: se calculan al nacer (hasta 8).
- **Valores que cambian mientras viven las partículas**: se aplican en cada paso (aceptan además la entrada `age`).

**Pasos**

1. Pulsa **Añadir una reasignación**.
2. Elige la **entrada** en la primera lista y la **salida** en la segunda. Las listas muestran los nombres tal cual: entradas como `random`, `life`, `speed`, `daytime`, `runtime`, `cursor:x`, `cursor:y`, `audio:bass`, `audio:mid`, `audio:treble`, `audio:level`, `distance:p0` (y `age` en el segundo bloque); salidas como `size`, `alpha`, `color`, `speed`, `life`, `rotation`, `spin`, `frame`.
3. Con los dos rangos de asas fijas qué parte de la entrada se mide y a qué parte de la salida llega.
4. Marca **Limitar la entrada** o **Limitar la salida** para que no pase de esos extremos.
5. Elige cómo se combina con el valor actual: **Fijar**, **Multiplicar**, **Añadir** o **Restar**.
6. Elige la forma: **Recta**, **Onda senoidal** o **Ruido**. Un gráfico muestra la curva de salida contra entrada.
7. **Quitar esta reasignación** la elimina.

**En la terminal.** Acción `particles`, clave `remap` (valores iniciales) o `force` con `remap` (valores durante la vida). También `wprs-ctl scene particles DIR SISTEMA remap|operator`. Mira [acciones](acciones.md).

## Cómo añadir atracciones, distancias y torbellinos

**Dónde está.** Inspector del sistema, bloque **Atracciones, distancias y bandadas**.

**Pasos**

1. Abre la lista **Añadir una fuerza** y elige una:
   - **Mantener una distancia**: las partículas se quedan a una distancia fija de un punto.
   - **Mantener entre dos puntos**: se quedan entre dos puntos.
   - **Atraer hacia un punto**.
   - **Arremolinar alrededor de un punto**.
   - **Girar alrededor de un punto**.
   - **Frenar cerca de un punto**.
2. Elige el punto: un punto de control, el puntero o un punto fijo (**un punto fijo**, con sus coordenadas en píxeles del sistema).
3. Ajusta la **Fuerza** (o **Velocidad conservada** en el frenado) y, donde aplique, **Se debilita con la distancia**: **Como lo hace la fuerza por defecto**, **No se debilita**, **En proporción** o **Con el cuadrado**.
4. Cada alcance (radio, borde interior, borde exterior) tiene un control deslizante. En el lienzo se ve como un anillo discontinuo con un asa arriba; la fuerza tiene otra asa en su centro.
5. En las fuerzas de distancia, el control **Distancia N px** fija la distancia; en el lienzo se ve un anillo con un asa en el objetivo y otra en el anillo.
6. **Quitar esta fuerza** la elimina.

**En la terminal.** Acción `particles`, clave `force`. También `wprs-ctl scene particles DIR SISTEMA force|keep`. Mira [acciones](acciones.md).

## Cómo hacer que las partículas se muevan en bandada

**Dónde está.** Bloque **Atracciones, distancias y bandadas**.

**Pasos**

1. Marca **Moverse en bandadas**.
2. Ajusta **Radio de las vecinas**, **Separación**, **Alineación**, **Cohesión** y **Velocidad máxima**.

**En la terminal.** Acción `particles`, clave `boids`. Mira [acciones](acciones.md).

## Cómo limitar cuándo actúa cada fuerza

**Dónde está.** Bloque **En qué parte de la vida actúa cada fuerza**. Solo aparece si el sistema tiene fuerzas.

**Pasos**

1. Marca **FUERZA: solo en parte de la vida** en la fuerza que quieras limitar.
2. Arrastra las asas del dibujo: de izquierda a derecha es la vida de una partícula; la fuerza aparece, actúa por completo y se desvanece donde la forma sube, se mantiene y baja.
3. Desmarca la casilla para que la fuerza actúe siempre.

**En la terminal.** Acción `particles`, clave `window`. También `wprs-ctl scene particles DIR SISTEMA window`. Mira [acciones](acciones.md).

## Cómo poner paredes (colisionadores)

**Dónde está.** Inspector del sistema, bloque **Colisionadores**. Se dibujan sobre la vista previa: en rojo donde las partículas mueren y en verde donde se pegan.

**Pasos**

1. Abre la lista **Añadir un colisionador** (hasta 8) y elige: **Lienzo** (el borde del lienzo), **Plano**, **Círculo**, **Caja**, **Segmento**, **Capa** o **Máscara de imagen**.
2. Arrastra las asas en el lienzo. Un plano tiene un asa en su punto y otra que gira el lado donde se quedan las partículas. El círculo tiene centro y radio; la caja, dos esquinas; el segmento, sus extremos. La capa tiene un asa en su centro: suéltala sobre otra capa del lienzo para que las partículas choquen con esa capa. La máscara tiene un asa en el borde superior: arrástrala a lo largo del borde para fijar la opacidad desde la que es sólida. El borde del lienzo no tiene nada que ajustar y no lleva asa.
3. Elige qué pasa al chocar: **Rebote**, **Deslizar**, **Detener**, **Pegar** o **Eliminar**.
4. Con **Rebote**, ajusta el rebote y la **Fricción**.
5. En círculo y caja, marca **Mantener las partículas dentro** para que no salgan (el dibujo pasa a línea discontinua).
6. En **Capa**, elige la capa contra la que chocan. En **Máscara de imagen**, elige la **Imagen** y la opacidad desde la que es sólida (**Sólido desde la opacidad N**).
7. **Quitar este colisionador** lo elimina.

**En la terminal.** Acciones `particles`: `collider` y `response`. También `wprs-ctl scene particles DIR SISTEMA collider`. Mira [acciones](acciones.md).

## Cómo hacer cuerdas y estelas

**Dónde está.** Inspector del sistema, bloque **Cuerdas y estelas**.

**Pasos**

1. En **Textura**, elige una imagen de la escena (o **Sin textura**).
2. Marca **Unir las partículas con una cuerda**. Ajusta **Anchura**, **Puntos entre partículas**, **Repeticiones de la textura**, **Desplazamiento de la textura** y **Curva suave**.
3. Si un emisor suelta más de una partícula por paso, la cuerda se amontona. El editor lo avisa y ofrece **Una partícula por paso**, que lo arregla.
4. Marca **Dejar una estela**. Ajusta **Pasos**, **Anchura**, **Puntos guardados**, **Puntos entre los puntos guardados**, **Repeticiones de la textura** y **Desplazamiento de la textura**. **Mostrar la textura a lo largo de la estela** pone la textura sobre la estela.
5. La vista previa se actualiza mientras mueves los controles.

**En la terminal.** Acciones `particles`: `strand` (cuerda o estela), `texture` y `pace`. También `wprs-ctl scene particles DIR SISTEMA rope|trail|texture` y `rope` para imprimir el consejo. Mira [acciones](acciones.md).

## Cómo cambiar el aspecto de las partículas

**Dónde está.** Inspector del sistema, bloque **Aspecto**, y los bloques **Luz** y **Doblar lo que hay detrás de las partículas**.

**Pasos**

1. **Recortar el alfa de la textura**: marca la casilla y ajusta el rango de dos asas y la **Opacidad**.
2. Elige hacia dónde miran los sprites: **Hacia la pantalla**, **Según el movimiento** o **Con un ángulo fijo** (con **Giro de N°**).
3. **Perspectiva**: activa la profundidad. La **Distancia focal** controla cuánto encoge la profundidad; en el lienzo hay un anillo con un asa. **Profundidad** y **Velocidad en profundidad** son rangos de dos asas.
4. En **Luz**, marca **Recibir las luces de la escena** y elige un **Mapa de normales**. Una miniatura muestra un sprite iluminado por delante y por detrás (cada sprite recibe hasta 8 luces en ambas caras).
5. En **Doblar lo que hay detrás de las partículas**, elige un mapa de normales y la cantidad. Marca **Mostrar el desplazamiento sobre la vista previa** para ver cuánto desplaza cada partícula. Una nota dice qué calidades lo omiten.

**En la terminal.** Acción `particles`, claves `cutout`, `orient`, `perspective`, `light` y `shader`. También `wprs-ctl scene particles DIR SISTEMA cutout|orient|perspective|lit|refract|shader`. Mira [acciones](acciones.md).

## Cómo elegir el orden de dibujo y qué pueden cambiar las capas

**Dónde está.** Inspector del sistema, bloque **Orden de dibujo y lo que pueden cambiar las capas**.

**Pasos**

1. En **Encima**, elige qué partículas se dibujan encima: **Sin orden fijo**, **Las más nuevas**, **Las más viejas**, **Las más grandes**, **Las más pequeñas** o **Las más bajas**.
2. Fija las veces que se repiten los fotogramas de una textura en una vida (0 es una sola vez).
3. En la lista de casillas, desmarca los valores que las capas no deben cambiar; quedan bloqueados.

**En la terminal.** Acción `particles`, claves `render` y `settings` (con `lock`). Mira [acciones](acciones.md).

## Cómo dibujar un sistema distinto en cada capa

**Dónde está.** Inspector del sistema, selector **Ajustes de dibujo de**.

**Pasos**

1. Elige **El sistema, para todas las capas** o **Solo la capa NOMBRE**.
2. Con una capa elegida, tu primer cambio le da un dibujo propio, copiado del sistema. El panel dice en qué se diferencia.
3. **Volver a usar el dibujo del sistema** lo deshace.

El selector cubre estas partes del dibujo: **Estirar**, **Fotogramas**, **Fotogramas por segundo**, **Mezclar fotogramas**, **Brillo**, **Ciclos**, **Orden de dibujo**, **Cuerda**, **Estela**, **Textura**, **Recorte**, **Orientación**, **Ángulo**, **Shader**, **Luz**, **Mapa de normales**, **Refracción**, **Tamaño a lo largo de la vida** y **Color a lo largo de la vida**.

**En la terminal.** Acción `particles`, clave `scope` (con `layer` y `own`). El panel no tiene controles para el tamaño y el color a lo largo de la vida: se escriben con la clave `over`, o con `wprs-ctl scene particles DIR SISTEMA size-over-life|color-over-life`. Mira [acciones](acciones.md).

## Cómo retocar una capa sobre su sistema

**Dónde está.** Inspector de una capa de partículas, bloque **Esta capa sobre su sistema**.

**Pasos**

1. Mueve los controles que quieras: **Opacidad**, **Tamaño, veces**, **Parte de las partículas**, **Velocidad, veces**, **Vida, veces**, **Ritmo del tiempo, veces** y **Brillo**. Los que el sistema bloqueó dicen «bloqueado por el sistema».
2. **Tinte** cambia el color. Cada control tiene un botón para volver a **Como lo dibuja el sistema**.
3. Los puntos de control del lienzo son de esta capa: arrastrar uno le da a la capa el suyo propio.
4. **Dibujar todo como el sistema** quita todos los retoques.

**En la terminal.** Acción `layer`, clave `instance`. Mira [acciones](acciones.md).

## Cómo hacer que un sistema inicie otros

**Dónde está.** Inspector del sistema padre, bloque de hijos (al final).

**Pasos**

1. Elige cuándo empieza el hijo: **Todo el tiempo**, **Al nacer una partícula**, **Al morir una partícula** o **Siguiendo a cada partícula**.
2. Pulsa **Nuevo sistema hijo**.
3. En cada hijo, **El hijo NOMBRE toma de su padre:** marca **Color**, **Tamaño**, **Alfa** o **Velocidad** (y la velocidad en profundidad cuando padre e hijo tienen perspectiva). **En cada paso mientras lo sigue** repite la herencia.
4. **Quitar el hijo** lo elimina.
5. **Shader de partículas** elige un efecto del paquete para dibujar; si el motor no puede construirlo o es demasiado lento, se omite y las partículas se dibujan sin él.

**En la terminal.** Acciones `particles`: `create` (con `parent` y `on`), `child`, `inherit` y `shader`. Mira [acciones](acciones.md).

## Cómo medir el coste de un sistema

**Dónde está.** Inspector del sistema, bloque **Presupuesto de partículas**.

**Pasos**

1. Si hay una vista previa en marcha, el bloque dice cuántas partículas hay vivas, de cuántos espacios, y si las simula la tarjeta gráfica o el procesador. También dice con qué calidad dibuja la vista previa.
2. Sin vista previa, las cuentas son una estimación del editor (lo dice una nota).
3. Marca **Simular en el procesador y mostrar las partículas sobre la vista previa** para ver las partículas del cálculo del editor sobre la vista previa. Al pulsar una, se dibuja el radio de vecinas de la bandada.
4. Elige la **Calidad de la vista previa**: **Q0, la más baja**, **Q1**, **Q2** o **Q3, la más alta**. Ves qué conserva cada nivel (nacimientos, vecinas, puntos de estela y de cuerda).
5. **Reiniciar los sistemas** arranca de nuevo solo las partículas, sin tocar el resto de la escena.
6. Para cada clase de equipo (los teléfonos y los equipos viejos caen en las dos más bajas) hay una fila con los espacios, los puntos de estela, las vecinas y los tramos de cuerda.

**En la terminal.** `wprs-ctl scene particles DIR SISTEMA budget` imprime el presupuesto. `wprs-ctl scene preview DIR --quality Q --restart` hace lo de la calidad y el reinicio. Mira [acciones](acciones.md).

## Cómo añadir un sonido

**Dónde está.** Botón **Añadir una capa**.

**Pasos**

1. Pulsa **Añadir una capa**.
2. Para importar un archivo nuevo, abre **Capa desde un archivo** y elige **Sonido…**. Aceptan `.ogg`, `.opus`, `.flac`, `.wav` y `.mp3`.
3. Para usar un sonido que la escena ya tiene, elige **Sonido** en la lista de capas. Esa opción está desactivada hasta que la escena tenga un archivo de sonido; si hay más de uno, el editor pregunta **Qué recurso**.
4. La capa de sonido no dibuja nada. Una escena admite hasta 8 capas de sonido, que se mezclan.

**En la terminal.** Acción `add_layer` con `kind` igual a `sound` y `file` o `source`. Mira [acciones](acciones.md).

## Cómo escuchar un sonido y ver su forma de onda

**Dónde está.** Inspector, al elegir el lienzo o una capa de sonido: bloque **Sonidos de la escena** (con el lienzo) o **Sonido** (con una capa).

**Pasos**

1. Cada capa muestra su nombre (con «silenciada» si lo está), cómo empieza (**Empieza con la escena** o **Empieza con** el disparador) y un botón **Escuchar**.
2. Pulsa **Escuchar** para que la vista previa reproduzca una vez solo ese sonido; las demás capas de sonido callan y el disparador se ignora. Pulsa **Dejar de escuchar**, escucha otro o sal del panel para volver a la escena. El volumen y el silencio de la aplicación se aplican como en un fondo real.
3. Debajo de cada sonido aparece su nombre con su duración y su **Forma de onda del sonido**.
4. **Sonidos que ninguna capa reproduce** lista los archivos de sonido sin capa, también con **Escuchar** y forma de onda.
5. Con el lienzo seleccionado, el botón **Editar la capa de sonido** (flecha) abre esa capa.

**En la terminal.** `wprs-ctl scene ask DIR sounds` lista las capas de sonido y los archivos; `wprs-ctl scene ask DIR wave` da la forma de onda de un archivo (argumentos `asset` y `buckets`). Mira [acciones](acciones.md).

## Cómo poner entrada y salida graduales

**Dónde está.** Debajo de cada sonido, sobre la **Forma de onda del sonido**.

**Pasos**

1. Verás dos puntos redondos sobre la forma de onda: **Entrada gradual** (a la izquierda) y **Salida gradual** (a la derecha).
2. Arrastra cada uno hasta donde quieras que el sonido termine de subir o empiece a bajar. Una línea dibuja la subida y la bajada.
3. Al soltar, el valor se guarda en segundos (de 0 a 60).
4. Si la capa tiene varios sonidos, la entrada va en el primero y la salida en el último.
5. También puedes escribir los dos valores en los campos **Entrada gradual** y **Salida gradual** de la capa.

El formato no tiene recorte de inicio y fin de un sonido: las entradas y salidas graduales dan forma a la parte audible.

**En la terminal.** Acción `set` sobre `fade_in` y `fade_out` de la capa. Mira [acciones](acciones.md).

## Cómo elegir cuándo suena (el disparador)

**Dónde está.** Campo **Sonar con** de la capa de sonido.

**Pasos**

1. Abre la lista. Sin valor se lee **Sin fijar**, y el sonido empieza con la escena.
2. Elige un disparador: el inicio, el pulso, un botón o ajuste del paquete, un evento, un clic o la entrada y salida del puntero sobre una capa, o el reproductor multimedia.
3. Pulsa el botón de reproducir junto a la lista (**Activar este disparador en la vista previa**) para probarlo. La ayuda dice qué líneas de tiempo y qué sonidos arranca; si nada de la escena arranca con él, lo dice.

**En la terminal.** Acción `set` sobre `play_on`. `wprs-ctl scene ask DIR fires` dice qué arranca con un disparador. Mira [acciones](acciones.md).

## Cómo ajustar el volumen y otros campos de una capa de sonido

**Dónde está.** Inspector de la capa de sonido.

**Pasos.** Estos son los campos de la capa:

- **Sonido** o **Sonidos**: el archivo, o una lista de 1 a 16.
- **Modo** (con varios sonidos): **En orden** o **Al azar**.
- **Pausar**: segundos de silencio entre sonidos (rango).
- **En flujo**: lee el archivo mientras suena, para sonidos largos.
- **Sonar con**: el disparador.
- **Bucle**: repite sin hueco (activado por defecto). Si lo apagas, suena una vez cada vez que se enciende.
- **Reproducir**: enciende y apaga el sonido.
- **Volumen** (de 0 a 1) y **Panorama** (de −1 izquierda a 1 derecha).
- **Entrada gradual** y **Salida gradual**.

**Volumen**, **Panorama** y **Reproducir** admiten claves de la línea de tiempo y ajustes de usuario (el botón de enlace de cada campo). No pueden seguir el análisis de sonido, el puntero ni un script: una escena no puede oír su propio sonido.

Toda escena con sonido tiene además un ajuste de usuario de volumen, un control deslizante de 0 a 100 que se añade solo. El usuario también puede silenciar los fondos con sonido.

**En la terminal.** Acción `set` sobre cada campo. Mira [acciones](acciones.md).

## Cómo enlazar un valor al sonido

Un campo de número, de pareja de números o de color puede seguir el sonido. El editor lo mide en 16, 32 o 64 bandas, de graves a agudos.

**Dónde está.** Junto a cada campo hay un botón de enlace: ◇ si tiene un valor fijo, ⛓ si sigue algo, ◈ o ◆ si tiene claves. Al pulsarlo se abre un menú.

**Pasos**

1. Pulsa el botón de enlace del campo.
2. Abre **Seguir el sonido** y elige uno de los cinco niveles con nombre: **Graves**, **Medios**, **Agudos**, **Nivel** (cuánto suena en total) o **Pulso** (vale 1 en cada golpe de ritmo).
3. El campo pasa a seguir ese nivel. El botón de enlace muestra «Sigue Sonido · NIVEL».

Para volver a un valor fijo, abre el menú y elige **Valor fijo**.

**En la terminal.** Acción `set` sobre el campo, con el enlace como valor. Mira [acciones](acciones.md).

## Cómo seguir una banda o un rango de bandas

**Dónde está.** Menú de enlace del campo, **Seguir el sonido**, **Una banda o un rango de bandas…**. Abre el cuadro **Seguir el sonido**.

**Pasos**

1. Los botones de arriba (**Graves**, **Medios**, **Agudos**, **Nivel**, **Pulso**) eligen un nivel con nombre.
2. En la tira de bandas, pulsa una banda o arrastra sobre varias. Las que eliges se marcan; debajo se lee su frecuencia («Banda 3: 200 Hz a 250 Hz» o «Bandas 3 a 6: …»). Los extremos de la tira indican la frecuencia más baja y la más alta.
3. En **Bandas**, elige **16 bandas**, **32 bandas** o **64 bandas**. Con más bandas, cada una es más estrecha.
4. En **Varias bandas cuentan como**, elige **Su promedio** o **La más fuerte**.
5. En **Canal**, elige **Ambos**, **Izquierda** o **Derecha**.
6. Una barra viva sobre la tira muestra los niveles de las bandas mientras suena algo (necesita el permiso de sonido, mira más abajo).
7. Pulsa **Aceptar** para enlazar, o **Cancelar**.

**En la terminal.** Acción `set` sobre el campo. Mira [acciones](acciones.md).

## Cómo ver los niveles del sonido

**Dónde está.** Menú de enlace del campo, **Probar y dar forma a la respuesta**, **Ver los niveles del sonido…**. Solo se puede usar con un campo que sigue el sonido.

**Pasos**

1. Elige **Ver los niveles del sonido…**. Se abre el cuadro **Niveles del sonido**.
2. Las barras son las bandas del sonido que oye la vista previa, unas diez veces por segundo. Las bandas que lee el enlace se marcan con un borde y un color más fuerte.
3. Una barra de progreso muestra el valor que da el enlace, y una línea dice «El enlace da X ahora (Y antes de darle forma)». «Antes de darle forma» es el valor sin los ajustes de **Límites**, **Exponente**, **Escala**, **Desplazamiento**, **Limitar a** e **Invertir**.
4. Cierra el cuadro cuando termines: al cerrarlo, el editor deja de leer los niveles.

El cuadro puede avisar de dos cosas:

- **La escena todavía no puede seguir el sonido. Permite la entrada de sonido del paquete para ver los niveles.** El paquete debe declarar la entrada de sonido y la persona que lo usa concederla. El aviso trae el botón **Permitir el sonido**: concede la entrada en un clic (el escritorio puede pedir confirmarlo en una notificación). Si la escena mostrada todavía no usa sonido, el cuadro lo dice: enlaza un valor al sonido, guarda y muestra la escena en una pantalla. Mira [paquete](paquete.md).
- **Esperando sonido. Reproduce algo y deja la vista previa en marcha; los niveles solo se leen mientras esta ventana está abierta y no se graba nada.**

Los niveles son los que la escena misma oiría. No se graba, ni se guarda, ni se envía nada.

**En la terminal.** `wprs-ctl scene ask DIR meter` da las bandas, la parte marcada, el valor y su valor sin forma (argumentos `source`, `reading` y `of`). Mira [acciones](acciones.md).

## Cómo dar forma a la respuesta del sonido

**Dónde está.** Menú de enlace del campo, **Editar el origen…**. El inspector muestra los campos del enlace.

**Pasos.** Ajusta los campos que quieras:

- **Sonido**: el nivel o la banda que se sigue.
- **Número de bandas**, **Combinar bandas** (**Media** o **Más alto**) y **Canal**: lo mismo que el selector de bandas.
- **Límites**: de qué valor del sonido a qué valor va la respuesta.
- **Exponente**: curva de la respuesta.
- **Escala** y **Desplazamiento**: multiplican y suman al valor.
- **Limitar a**: no deja que pase de esos extremos.
- **Invertir**: da la vuelta a la respuesta.
- **Suavizado**: mira la sección siguiente.

Mira el efecto en vivo con **Ver los niveles del sonido…**: la barra de progreso muestra el valor ya con la forma.

**En la terminal.** Acción `set` sobre cada campo del enlace. Mira [acciones](acciones.md).

## Cómo suavizar los cambios

**Dónde está.** Menú de enlace del campo, **Probar y dar forma a la respuesta**, **Suavizar los cambios…**. Está disponible en los enlaces de un número, una pareja de números o un color.

**Pasos**

1. Elige **Suavizar los cambios…**. El enlace recibe un suavizado de 0,2 s y el inspector lo abre.
2. Mueve el control **Segundos** (de 0 a 5) o escribe el valor (hasta 60).
3. Para que el suavizado sea distinto al subir y al bajar, activa **Distinta velocidad al subir y al bajar**. Aparecen **Al subir (s)** y **Al bajar (s)**.
4. El gráfico **Respuesta del suavizado a un salto hacia arriba y hacia abajo** muestra cómo reacciona el valor a un salto. Debajo se lee «Se asienta en X s al subir y en Y s al bajar», que son tres veces la constante.

**En la terminal.** Acción `set` sobre el campo `smooth` del enlace. `wprs-ctl scene ask DIR response` da los puntos del gráfico (argumento `smooth`, un número o `{ "up", "down" }`). Mira [acciones](acciones.md).

## Cómo probar un enlace al puntero

**Dónde está.** Menú de enlace del campo, **Probar y dar forma a la respuesta**, **Probar el puntero**. Aparece solo en campos que siguen al puntero.

**Pasos**

1. Elige **Probar el puntero**. Un puntero inventado recorre la vista previa.
2. Para pararlo, quita la marca de la misma entrada del menú.

Para enlazar un campo al puntero usa **Seguir el puntero** en el menú de enlace: **Posición horizontal**, **Posición vertical**, **Sobre la pantalla** o **Velocidad**.

**En la terminal.** `wprs-ctl scene ask DIR wander` da el recorrido inventado (argumento `time`). Mira [acciones](acciones.md).

## Atajos de teclado

Ninguna de estas pantallas define atajos propios.
