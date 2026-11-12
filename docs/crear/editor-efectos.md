# Efectos, máscaras y shaders en el editor de escenas

Esta página explica cómo poner efectos a una capa, limitarlos con máscaras, colocar sus puntos sobre el lienzo, pintar los mapas que usan y escribir tus propios efectos y shaders. El editor se abre con la escena; las demás partes están en [editor](editor.md), [editor-lienzo](editor-lienzo.md) y [editor-dibujo](editor-dibujo.md).

Todo lo que haces aquí es una acción del editor. En la terminal se aplica con `wprs-ctl scene act CARPETA ACCIÓN JSON`; las acciones y sus argumentos están en [acciones](acciones.md). Cada arrastre sobre el lienzo o sobre un deslizador cuenta como un solo paso de **Deshacer**, y cada trazo de pincel también.

## Qué es cada cosa

- Un **efecto** cambia cómo se dibuja una capa: la desenfoca, la tiñe, la deforma, la mueve. Una capa puede tener varios; cada uno trabaja sobre el resultado del anterior.
- Un **parámetro** es un ajuste de un efecto (un radio, un color, una imagen, un punto).
- Una **máscara** limita dónde se ve el efecto, o dónde se ve la capa entera.
- Una **capa de ajuste** no dibuja nada propio: aplica sus efectos a todo lo que hay debajo.
- Un **efecto del paquete** es uno que escribes tú, con un descriptor (la lista de sus parámetros) y un shader en código WGSL. Viaja dentro de la escena.

## Cómo añadir un efecto a una capa

**Dónde está:** el inspector de una capa (cualquiera menos una capa de sonido), en la barra de botones de arriba.

**Pasos:**

1. En la pestaña **Capas**, elige la capa.
2. En el inspector, pulsa **Añadir un efecto**.
3. Se abre el cuadro **Añadir un efecto**. Escribe en el buscador para encontrar un efecto por su nombre o por su descripción, o recorre la lista, agrupada por categoría.
4. Pulsa el efecto. Se añade al final de la lista de efectos de la capa y el cuadro se cierra. **Cancelar** cierra el cuadro sin añadir nada.

Cada entrada de la lista muestra el nombre del efecto, su descripción y su costo, por ejemplo «(coste 2.2)». El costo es el trabajo que le pide a la tarjeta gráfica; mira [Cómo ver cuánto cuesta un efecto](#cómo-ver-cuánto-cuesta-un-efecto).

Los efectos que escribes en el paquete aparecen en la misma lista, con su categoría y su descripción.

**En la terminal:** acción `effects` con `add`; también `wprs-ctl scene effects CARPETA CAPA add`. La lista del catálogo sale con `wprs-ctl scene catalog CARPETA [--category CATEGORÍA] [--json]`. Mira [acciones](acciones.md).

## Cómo ver los efectos de una capa, ordenarlos y quitarlos

**Dónde está:** en el inspector de la capa, el campo **Efectos**.

**Pasos:**

1. Elige la capa. El campo **Efectos** dice cuántos elementos tiene («2 elementos»).
2. Abre el desplegable **Editar…** y elige un **Elemento** (1, 2, 3…). El inspector pasa a ese efecto.
3. Con el efecto elegido, los botones de la barra del inspector son:
   - **Subir un nivel**: vuelve a la capa.
   - **Subir** y **Bajar**: cambian el lugar del efecto en la lista. Cambiar el orden cambia el resultado, porque cada efecto trabaja sobre lo que dejó el anterior.
   - **Quitar** (el icono de la papelera): quita el efecto.
4. Los campos del efecto elegido son **Efecto** (cambia a otro efecto), **Activado** (un interruptor para apagarlo sin quitarlo), **Nombre** (un nombre propio, opcional), **Opcional** y **Máscara**, y debajo sus parámetros.

**Opcional:** si la escena pesa demasiado para el equipo, el programa omite primero los efectos marcados como **Opcional**. Mira [Cómo ver cuánto cuesta un efecto](#cómo-ver-cuánto-cuesta-un-efecto).

**En la terminal:** acción `effects` con `add`, `set` (parámetros, nombre, **Activado**, **Opcional**), `remove` y `move`; también `wprs-ctl scene effects CARPETA CAPA set|remove|move`. Mira [acciones](acciones.md).

## Catálogo de efectos

La tabla lista los efectos que trae el programa, en el orden y con los nombres del cuadro **Añadir un efecto**. La segunda columna es el id que se usa en la terminal.

### Mezcla

| Nombre | Id | Qué hace |
|---|---|---|
| Mezcla por degradado | `blend_gradient` | Muestra una textura según un mapa gris que dice dónde se ve. |
| Mezclar textura | `blend_texture` | Pone una textura sobre la imagen con un modo de mezcla. |
| Disolución | `dissolve` | Deshace la imagen en un patrón granulado. |
| Revelar | `reveal` | Muestra u oculta la imagen con una forma que crece. |

### Desenfoque

| Nombre | Id | Qué hace |
|---|---|---|
| Desenfoque | `blur` | Desenfoca con un radio fijo. |
| Desenfoque de movimiento | `blur_motion` | Desenfoca en una dirección, como si se moviera rápido. |
| Desenfoque radial | `blur_radial` | Desenfoca hacia fuera desde un centro, como un zoom. |
| Contraste local | `local_contrast` | Afila el contraste local sin cambiar la exposición general. |
| Efecto miniatura | `tilt_shift` | Desenfoca fuera de una franja de enfoque. Puede usar un mapa de profundidad. |

### Color

| Nombre | Id | Qué hace |
|---|---|---|
| Croma | `chroma_key` | Quita un color y lo vuelve transparente. |
| Ajustes de color | `color_adjust` | Exposición, contraste, saturación y matiz. |
| Etalonaje | `color_lut` | Gradúa el color con una tabla de color. |
| Posterizar | `posterize` | Reduce la imagen a pocos niveles de color. |
| Pulso | `pulse` | Hace pulsar el brillo, el color, la opacidad o una luz con tinte. |
| Tinte | `tint` | Tiñe la imagen con un color. |

### Distorsión

| Nombre | Id | Qué hace |
|---|---|---|
| Aberración cromática | `chromatic_aberration` | Separa los canales de color hacia los bordes. |
| Paralaje de profundidad | `depth_parallax` | Desplaza por un mapa de profundidad al mover el puntero. |
| Desplazar | `displace` | Mueve los píxeles con un mapa. |
| Distorsión de lente | `lens_distort` | Curva la imagen como una lente. |
| Perspectiva | `perspective` | Lleva las cuatro esquinas a otro cuadrilátero. |
| Pixelar | `pixelate` | Forma bloques grandes, cuadrados o hexagonales. |
| Remolino | `twirl` | Retuerce la imagen alrededor de un punto. |

### Luz

| Nombre | Id | Qué hace |
|---|---|---|
| Resplandor | `bloom` | Hace brillar hacia fuera las zonas claras. |
| Rayos de luz | `god_rays` | Dibuja rayos que salen de una fuente clara. |
| Iris | `iris` | Oscurece fuera de un círculo que sigue al puntero. |
| Destello de lente | `lens_flare` | Dibuja el destello de una lente desde una luz. |
| Haces de luz | `light_shafts` | Dibuja haces suaves de luz de color. |
| Reflejo | `reflection` | Añade un reflejo ondulado bajo la imagen. |
| Destello | `shine` | Pasa una franja de brillo sobre la imagen. |
| Viñeta | `vignette` | Oscurece las esquinas. |

### Movimiento

| Nombre | Id | Qué hace |
|---|---|---|
| Cáusticas | `caustics` | Patrones de luz cambiantes, como el sol en el agua. |
| Nubes y niebla | `clouds` | Nubes o niebla que derivan. |
| Eléctrico | `electric` | Arcos eléctricos animados. |
| Fuego | `fire` | Fuego animado con un degradado de color. |
| Flujo | `flow` | Deforma la imagen a lo largo de un mapa de flujo. |
| Fluido | `fluid` | Un fluido que reacciona al puntero. |
| Fallo digital | `glitch` | Filas desplazadas y colores separados. |
| Desplazamiento continuo | `scroll` | Desplaza la imagen y la repite en los bordes. |
| Sacudida | `shake` | Sacude la imagen en una dirección o por un mapa de flujo pintado. |
| Giro | `spin` | Gira la imagen alrededor de un centro. |
| Vaivén | `sway` | Balancea la imagen como algo colgado, o hace temblar el follaje. |
| Estela de movimiento | `trail` | Deja una estela que se apaga. |
| Anillos en el agua | `water_rings` | Anillos que se expanden, como gotas. |
| Ondulaciones del agua | `water_ripple` | Ondas que reaccionan al puntero y a los clics. |
| Ondas | `waves` | Deforma con uno o dos patrones de ondas. |

### Estilo

| Nombre | Id | Qué hace |
|---|---|---|
| Pantalla CRT | `crt` | Una pantalla antigua con líneas y curvatura. |
| Bordes | `edges` | Dibuja los bordes, con aspecto de boceto opcional. |
| Grano de película | `film_grain` | Grano animado. |
| Destellos | `glitter` | Puntos que centellean. |
| VHS | `vhs` | Una cinta gastada, con ruido y color corrido. |

### Transformación

| Nombre | Id | Qué hace |
|---|---|---|
| Transformación | `transform` | Desplaza, escala, gira e inclina la imagen. |

## Cómo ajustar los parámetros de un efecto

**Dónde está:** inspector, con el efecto elegido (mira [Cómo ver los efectos de una capa](#cómo-ver-los-efectos-de-una-capa-ordenarlos-y-quitarlos)).

**Pasos:**

1. Cada parámetro es un campo con su nombre. Pasa el cursor por el nombre para ver su ayuda.
2. Un número tiene un deslizador y una caja para escribirlo. Un parámetro en grados tiene además un dial. Un color tiene su selector. Una imagen se elige de una lista de las imágenes del paquete.
3. El botón **Valor por defecto** (el icono de volver) deja el parámetro como viene de fábrica.
4. El menú de al lado de cada campo que se puede animar permite poner un fotograma clave, seguir una propiedad de usuario, el puntero o el sonido, o dejarlo fijo. Mira [editor-animacion](editor-animacion.md) y [editor-propiedades](editor-propiedades.md).
5. Un parámetro que sigue algo muestra un botón **Sigue …**; púlsalo para editar el origen.

**En la terminal:** acción `effects` con `set` y `params`. Mira [acciones](acciones.md).

## Cómo colocar los puntos de un efecto sobre el lienzo

Algunos parámetros son un lugar o una dirección: el centro de un remolino, la fuente de unos rayos, las esquinas de una perspectiva. En vez de escribir números, los colocas arrastrando.

**Dónde está:** el lienzo. Las asas aparecen mientras el efecto, o uno de sus parámetros, está elegido en el inspector.

**Pasos:**

1. Elige el efecto en el inspector.
2. Mira el lienzo: cada parámetro con asa dibuja la suya sobre la capa, ya con el movimiento, el padre y el origen de la capa aplicados.
3. Arrastra el asa:
   - Un **punto** (círculo): **Colocar este punto**.
   - Una **esquina** (cuadrado), como las cuatro de la perspectiva: se unen en un cuadrilátero.
   - Una **dirección** (flecha de color neutro, con el asa en la punta): **Apuntar la dirección**. Sale del primer punto del efecto o, si no tiene, del centro de la capa.
   - Un **ángulo** (círculo alrededor de un punto, con el asa en el borde): **Girar el ángulo alrededor de su punto**. Los parámetros en grados tienen también un dial en el inspector.
   - Dos puntos emparejados (por ejemplo inicio y fin) se unen con una línea. El asa de cada extremo es **Colocar este extremo; con Mayús se mueve toda la línea**. El asa cuadrada del medio, **Mover toda la línea**, mueve los dos extremos a la vez.
4. Cada arrastre es un solo paso de **Deshacer**.

En el descriptor de un efecto del paquete, cada parámetro dice qué asa recibe (mira [Cómo editar el descriptor de un efecto](#cómo-editar-el-descriptor-de-un-efecto)).

**Atajo:** mantén **Mayús** mientras arrastras un extremo para mover toda la línea (`SceneEffectPointOverlay.qml`).

**En la terminal:** acción `effects` con `grip` (el parámetro y el punto adonde llevarlo). Mira [acciones](acciones.md).

## Cómo limitar un efecto, o una capa, con una máscara

Una máscara decide dónde se ve el efecto. Donde la máscara está llena el efecto actúa por completo; donde está vacía no actúa. Se puede poner a un efecto o a la capa entera.

**Dónde está:** inspector. Con un efecto elegido el campo se llama **Máscara del efecto:**; con una capa elegida, **Máscara de la capa:**.

**Pasos:**

1. Elige el efecto (o la capa).
2. En **Máscara del efecto:** elige el origen:
   - **Ninguno**: quita la máscara.
   - **Una imagen**: usa una imagen o vídeo del paquete (empieza con el primero; cámbialo en **Imagen:**). Hace falta que el paquete tenga alguno.
   - **Una capa**: usa otra capa de la escena (empieza con la primera; cámbiala en **Capa:**). El efecto actúa donde esa capa está sobre la pantalla. Hace falta que haya otra capa.
   - **Una forma**: dibuja una forma. Elige cuál en **Forma:**: **Rectángulo**, **Elipse**, **Anillo**, **Polígono**, **Estrella** o **Línea**. Empieza centrada en la capa y con la mitad de su tamaño.
3. **Se ve así:** muestra una miniatura de la máscara.
4. Ajusta la máscara:
   - **Canal** (solo con imagen o capa): qué canal se lee, **Brillo**, **Rojo**, **Verde**, **Azul** o **Alfa**. Con una imagen empieza en **Brillo**; con una capa, en **Alfa**.
   - **Invertir: aplicar el efecto donde la máscara está vacía**.
   - **Borde suave** (solo con forma): cuántos píxeles se difumina el borde; un deslizador y una caja.
   - **Fuerza**: de 0 a 1; cuánto pesa la máscara.
5. El lienzo dibuja la máscara sobre la capa mientras el efecto, la capa o la máscara estén elegidos: la caja de la capa en trazo discontinuo y, con una imagen, el canal que se lee teñido sobre la capa; con una capa, la capa que enmascara enmarcada; con una forma, su contorno relleno donde actúa el efecto.

**En la terminal:** acción `effects` con `mask` (para la capa entera, sin `index`); también `wprs-ctl scene effects CARPETA CAPA mask`. Mira [acciones](acciones.md).

## Cómo cambiar el tamaño y el borde de una máscara con las asas

**Dónde está:** el lienzo, con una máscara de forma elegida (el efecto, la capa o su máscara). Las máscaras de imagen y de capa no tienen asas.

**Pasos:**

1. Elige la capa o el efecto cuya máscara es una **forma**.
2. Arrastra el asa cuadrada de la esquina: **Arrastra para dar tamaño a la máscara**. En una línea es el extremo.
3. Arrastra el punto que está más allá del borde: **Borde suave … px: arrastra lejos de la forma para suavizar su borde**. Es el mismo valor que **Borde suave** del inspector; el lienzo dibuja el rango como dos contornos discontinuos, uno más grande y uno más pequeño que la forma.
4. Según la forma hay un asa más:
   - **Anillo**: **Arrastra para ensanchar o estrechar el anillo**.
   - **Rectángulo**: **Arrastra para redondear las esquinas**.
   - **Estrella**: **Arrastra para afilar o despuntar las puntas**.
5. Cada arrastre es un solo paso de **Deshacer**.

El lienzo repite lo que dice el inspector: «El lienzo muestra la máscara sobre la capa: arrastra las asas de una forma para darle tamaño y el punto junto a su borde para suavizarlo.»

**En la terminal:** acción `effects` con `mask_grip` (qué asa y adónde se lleva). Mira [acciones](acciones.md).

## Cómo pintar una máscara, un mapa gris o un mapa de dirección

Algunos efectos leen una imagen que tú pintas: un mapa gris que dice dónde actúa algo, o un mapa de dirección (de flujo) que dice hacia dónde se mueve. El pincel escribe en una imagen del paquete; no hace falta tener una antes.

**Dónde está:** inspector, en el parámetro de imagen del efecto (por ejemplo `map`, `phase` o el de un efecto del paquete que declare un parámetro de máscara). El botón de pintar aparece junto al parámetro.

**Pasos para un mapa gris:**

1. Elige el efecto y busca el parámetro de imagen.
2. Pulsa **Pintar en gris** (en un parámetro de máscara, **Pintar**). El botón queda pulsado.
3. Ajusta:
   - **Borrar**: el pincel pinta 0.
   - **Tamaño del pincel**.
   - **Gris** (o **Valor**, en un parámetro de máscara): cuánto pinta el pincel, de 0 a 1.
   - Solo en un parámetro de máscara: **Dureza** y, si la imagen tiene varios canales, el desplegable de canal (**Rojo**, **Verde**, **Azul**, **Alfa**).
4. Pinta sobre la capa en la vista previa. El canal que pintas se ve teñido. Cada trazo es un paso de **Deshacer**.
5. Vuelve a pulsar el botón para dejar de pintar.

**Pasos para un mapa de dirección:**

1. Pulsa **Pintar la dirección**.
2. **Aplanar** hace que lo pintado quede quieto. **Fuerza** es cuánto se mueve.
3. Arrastra sobre la capa en el sentido en que debe moverse: cada trazo escribe la dirección que llevó, y el resto queda quieto.

**En la terminal:** `wprs-ctl scene paint-mask CARPETA PUNTERO PUNTOS` (trazo en fracciones de la capa), o las acciones `paint_mask` y `comb_flow`. Mira [acciones](acciones.md).

## Cómo hacer un degradado de color

**Dónde está:** inspector, en los parámetros de imagen que son un degradado (por ejemplo el `gradient` del fuego o de los haces de luz) y en los de tabla de color (el de **Etalonaje**).

**Pasos:**

1. Elige el efecto. Bajo el selector de imagen aparece una barra de degradado.
2. Pulsa la barra para añadir una parada (hasta 8). Arrastra una parada para moverla.
3. Debajo de la barra, cada parada tiene su posición, un selector de color y **Quitar la parada**. Queda al menos una.
4. Al dejar de editar, el editor hace la imagen del degradado y la pone en el parámetro. Es un solo paso de **Deshacer**. Si el parámetro ya nombra un degradado hecho aquí, vuelve a hacerlo.

En una tabla de color los tonos oscuros toman las primeras paradas y los claros las últimas.

**En la terminal:** acción `ramp` (paradas, formato `png` o `cube`, parámetro). Mira [acciones](acciones.md).

## Cómo hacer un mapa de profundidad

Algunos efectos, como **Paralaje de profundidad** y **Efecto miniatura**, leen un mapa de profundidad: una imagen gris que dice a qué distancia está cada parte.

**Dónde está:** inspector, bajo el parámetro `depth` del efecto.

**Pasos:**

1. Elige el efecto.
2. Elige en el desplegable la imagen de la que sale la profundidad.
3. Pulsa **Hacer un mapa de profundidad**. El mapa se guarda en el paquete y el parámetro lo usa.
4. Si el botón no está disponible, el aviso de al lado dice qué falta: el modelo de profundidad o la biblioteca que lo ejecuta.
5. También puedes pintar el mapa a mano con **Pintar en gris** (arriba).

**En la terminal:** acción `depth_map`; la pregunta `depth_model` dice si el modelo está listo (`wprs-ctl scene ask CARPETA depth_model '{}'`). Mira [acciones](acciones.md).

## Cómo hacer brillar la escena

**Dónde está:** el inspector de la escena. Para llegar a él, pulsa un lugar del lienzo donde no haya ninguna capa.

**Pasos:**

1. Pulsa un lugar del lienzo donde no haya ninguna capa. El inspector muestra los ajustes de la escena.
2. En **Brillo:** activa **Las partes claras brillan**.
3. Ajusta **Umbral:** (qué tan claro debe ser algo para brillar), **Fuerza:** y **Radio:**.
4. Para comparar, abre **Nivel de la vista previa**, marca **Sin el brillo del lienzo** y mantén pulsado **Mantener para ver el otro lado**: la vista previa enseña el otro lado de la comparación sin el brillo. O pulsa **Comparar el brillo** en la barra del lienzo para ver el lienzo partido en vivo, con el brillo a la izquierda y sin él a la derecha.

Si un solo grupo de capas debe brillar, usa el efecto **Resplandor** en una capa, o en una capa de ajuste.

Para conservar colores más brillantes que el blanco, en el mismo inspector está **Colores brillantes**: **Conservar colores más brillantes que el blanco**, el **Mapeo de tonos** (**Ninguno**, **Fílmico (ACES)** o **Suave (AgX)**) y **Antes y después**, que corta la vista previa para comparar.

**En la terminal:** acción `canvas` con `bloom`; también `wprs-ctl scene canvas CARPETA bloom`. El mapeo de tonos: `wprs-ctl scene tonemap`. Mira [acciones](acciones.md).

## Cómo usar una capa de ajuste

**Dónde está:** pestaña **Capas**, botón **Añadir una capa**, entrada **Ajuste**.

**Pasos:**

1. Pulsa **Añadir una capa** y elige **Ajuste**.
2. Añade efectos a la capa de ajuste como a cualquier capa (mira [Cómo añadir un efecto a una capa](#cómo-añadir-un-efecto-a-una-capa)). Se aplican a todo lo que está debajo de ella en la lista de capas.
3. Para que afecte solo a una parte, escribe un **Tamaño** en el inspector de la capa. Solo se captura ese rectángulo, alrededor de la posición de la capa, y los efectos cambian solo esa parte.
4. En el lienzo se ve qué captura:
   - un rectángulo relleno, la parte capturada;
   - un marco discontinuo, lo máximo que puede capturar (el doble del lienzo por cada lado; se pone rojo si **Tamaño** lo pasa);
   - un texto, «Captura … × … px; el máximo es … × … px»;
   - asas en las esquinas y en el medio de los lados: **Cambia el tamaño de la parte capturada; la capa conserva su escala**.
5. Con **Volver a dibujar** la parte capturada se vuelve a dibujar, con la posición, el giro y la opacidad de la capa, como una lupa o un espejo; su contorno sale discontinuo y verde.

Una capa de ajuste también puede enviar lo que captura a la iluminación RGB del equipo: el interruptor **Enviar lo que capta esta capa a la iluminación RGB** y los botones **Todo el lienzo** y **Una franja** están en su inspector, con una vista de lo que reciben las luces.

**En la terminal:** `wprs-ctl scene add-layer CARPETA adjustment`; el tamaño con la acción `layer` (`transform` con `size`) y las asas con `layer` `grip`; la iluminación con la acción `rgb` o `wprs-ctl scene rgb`. Mira [acciones](acciones.md).

## Cómo cambiar las reglas de despertar de un efecto

Una escena sin nada que cambie se duerme y no gasta. Cada parámetro de un efecto puede pedir que la escena siga despierta mientras no vale cero, esperar a otro parámetro, o cederle el paso.

**Dónde está:** inspector, con el efecto elegido, el campo **Wake** (el programa no lo traduce) al final de los parámetros.

**Pasos:**

1. Elige el efecto. Si es uno del programa, el campo solo muestra sus reglas; un efecto del paquete las deja cambiar.
2. **Siempre pide:** lista lo que el efecto pide siempre.
3. Para cada parámetro hay tres desplegables:
   - **Pide**: **Nada**, **Tiempo**, **El puntero**, **El sonido** o **Clics**. Mientras el parámetro no sea cero, el efecto pide esa entrada.
   - **No espera a nada** o **Espera a** otro parámetro.
   - **No cede a nada** o **Cede a** otro parámetro.
4. La escena duerme cuando ningún parámetro pide nada.

**En la terminal:** acción `effect_rule`; `wprs-ctl scene effect-rule CARPETA EFECTO [PARÁM]` o la pregunta `rules`. Mira [acciones](acciones.md).

## Cómo ver cuánto cuesta un efecto

Cada efecto cuesta unidades de trabajo por cuadro. El programa baja la calidad si la escena pesa demasiado (con batería, si el equipo es lento) y entonces recorta efectos: los ligeros siguen en todos los niveles; los medios y pesados trabajan a menor tamaño o se omiten, los marcados **Opcional** primero.

**Dónde está:** inspector, con un efecto elegido, sección **Coste**.

**Pasos:**

1. Elige el efecto. Bajo **Coste** hay una lista con una línea por nivel: «Q1: 0.56 unidades de trabajo», o «Q0: se deja fuera de la escena en este nivel». La del nivel de la vista previa sale en negrita.
2. **Calidad de la vista previa** cambia el nivel que dibuja la vista previa: **Q0, la más baja**, **Q1**, **Q2** y **Q3, la más alta**. **Reiniciar** vuelve a empezar la vista previa.
3. El botón **Nivel de la vista previa**, en la barra del lienzo, abre un panel con el mismo nivel, **Con batería**, **Menos movimiento**, el resumen de la escena («Dibujada como Q3 al 100 % del tamaño, con 0 efectos dejados fuera») y el costo de cada efecto de la escena, con **Comparar con** otro nivel.
4. Los efectos que siguen al puntero se prueban con **Probar el puntero y los clics**; el panel los nombra.

**En la terminal:** pregunta `effect_cost` o `wprs-ctl scene effect-cost`. Mira [acciones](acciones.md).

## Cómo escribir un efecto del paquete

Un efecto del paquete son dos archivos de la carpeta `effects/` de la escena: `NOMBRE.json` (el descriptor: parámetros, pasadas, costo) y `NOMBRE.wgsl` (el shader).

**Dónde está:** pestaña **Recursos**, en la sección de efectos, bajo el botón **Importar un efecto…**: la fila con **Id de un efecto nuevo**.

**Pasos:**

1. En **Id de un efecto nuevo** escribe un id con letras minúsculas, cifras y guiones bajos (hasta 32).
2. Elige la plantilla en el desplegable:
   - **Un cambio de color**: un tinte fusionado con los efectos vecinos.
   - **Una deformación alrededor de un punto**: mueve la imagen alrededor de un punto que colocas en el lienzo.
   - **Dos pasadas**: un desenfoque a lo largo de un ángulo, en dos pasadas.
   - **Un shader de material**: para materiales de modelos 3D (mira [Cómo escribir un shader de material](#cómo-escribir-un-shader-de-material)).
   - **Un shader de partículas**: para sistemas de partículas (mira [editor-particulas-y-sonido](editor-particulas-y-sonido.md)).
3. Pulsa **Efecto nuevo**. El efecto aparece en la lista **Efectos** con su tamaño.
4. Para usar un efecto de capa, elige una capa y pulsa **Añadir un efecto**: está en la lista, con los del programa.

**En la terminal:** `wprs-ctl scene new-effect CARPETA NOMBRE [--template PLANTILLA]`, o la acción `shader` con `new`. Mira [acciones](acciones.md).

## Cómo escribir y probar el código de un efecto

**Dónde está:** pestaña **Recursos**, lista **Efectos** (la lista aparece cuando el paquete tiene algún efecto). Cada efecto del paquete tiene cuatro botones: **Editar el código WGSL…**, **Editar el descriptor…**, **Reimportar…** y **Quitar**.

**Pasos:**

1. Pulsa **Editar el código WGSL…** en la fila del efecto. Se abre una ventana con el código.
2. Escribe. Medio segundo después de dejar de teclear, el editor comprueba el efecto como lo hace el programa al cargarlo, y el aviso de arriba dice:
   - **Comprobando…**;
   - **Compila; cuesta N unidades**, si todo está bien;
   - **Línea L, columna C: mensaje**, si hay un error. La línea queda marcada en rojo en el texto, y el botón **Ir a la línea** lleva el cursor allí.
3. Marca **Vista previa al escribir: guardar cada versión que compila** para ver el cambio en la vista previa mientras escribes, en las capas que usan el efecto. Cada versión que compila se guarda sola; los pasos seguidos se deshacen juntos.
4. Pulsa **Guardar** para guardar a mano; el título muestra «(sin guardar)» mientras haya cambios. Se guarda aunque el texto no compile.
5. Pulsa **Usar en la capa elegida** para añadir el efecto a la capa que tengas elegida (solo para efectos de capa).
6. **Abrirlo en tu editor…** abre el archivo `.wgsl` con tu editor de texto.
7. **Cerrar** cierra la ventana.

El texto de un shader no puede pasar de 64 KiB. Los mensajes de error los escribe el comprobador, tal cual.

**Atajo:** Ctrl+S dentro del texto guarda el shader, aunque no compile (`StandardKey.Save` en `SceneShaderCode.qml`).

**Qué escribir:** una plantilla de color muestra la forma de una función de efecto.

```wgsl
fn color_tinte(color: vec4<f32>, uv: vec2<f32>) -> vec4<f32> {
    let tinted = vec4<f32>(color.rgb * param(1u).rgb, color.a);
    return mix(color, tinted, param(0u).x);
}
```

`param(N)` lee el parámetro N del descriptor, contando desde 0 en el orden de la lista. Una deformación se llama `uv_<id>` y un cambio de color `color_<id>`.

**En la terminal:** acción `shader` con `write` (el texto del WGSL o el descriptor); la pregunta `shader_check` comprueba el efecto, con texto sin guardar si se le da, y dice línea, columna, costo y asas (`wprs-ctl scene ask CARPETA shader_check '{"effect":"NOMBRE"}'`); `shader_source` da el descriptor y el código. Mira [acciones](acciones.md).

## Cómo editar el descriptor de un efecto

**Dónde está:** pestaña **Recursos**, lista **Efectos**, botón **Editar el descriptor…** del efecto.

**Pasos:**

1. Se abre la ventana **Descriptor de NOMBRE**. Arriba, el aviso dice **Compila** o el error.
2. En **Parámetros** hay una fila por parámetro, de izquierda a derecha:
   - el id;
   - el tipo, con su nombre en inglés (`float`, `color`, `texture`, `position`…);
   - **Valor por defecto**, que se escribe como JSON, y **Mín.** y **Máx.** (vacíos quitan el límite);
   - **Grados**, en los números: el parámetro se mide en grados y recibe un dial;
   - en los parámetros de imagen o de máscara, qué herramientas ofrece: **Herramientas según su id**, **Paradas de degradado**, **Pincel gris**, **Pinceles gris y de dirección** o **Pincel gris y mapa de profundidad**;
   - qué asa recibe sobre el lienzo: «un punto sobre el lienzo», «una esquina sobre el lienzo», «una flecha sobre el lienzo» o «un dial sobre el lienzo», con el parámetro con el que hace pareja;
   - un botón para subirlo en la lista y otro para quitarlo.
3. Para añadir un parámetro, escribe su id en **Id de un parámetro nuevo**, elige el tipo y pulsa **Añadir el parámetro**.
4. **Pasadas** (solo en efectos de capa): una fila por pasada con **Punto de entrada**, su escala en porcentaje, las **Entradas** («previous», «pass:0», «target:soft»…) y el **Objetivo**. **Añadir una pasada** suma otra.
5. **Costo declarado: lecturas, pasadas y escala**: tres números con lo que el efecto pide por cuadro.
6. Para un efecto de capa:
   - **Máscara**: **Mezclar el resultado según la máscara**, o **El shader se escala solo con mask_at**.
   - **Fusionado como**: **Sin fusionar**, **Una deformación de la imagen (uv_)** o **Un cambio de color (color_)**.
   - **Agranda la capa según**: **Nada** o un parámetro numérico.
7. **Costo cuando una capa lo marca como opcional** recalcula la línea de abajo, **Costo por nivel de calidad**, que da las unidades de cada nivel o «se omite».

Cada cambio se guarda y es un paso de **Deshacer**. Lo que el formulario no cambia (la categoría, la descripción y la etiqueta y la ayuda de cada parámetro) está en el archivo `effects/NOMBRE.json`: edítalo con tu editor, o desde la terminal con la acción `shader` (`param` con `label` y `help`, `field` con `category` y `description`).

**En la terminal:** acción `shader` con `param`, `pass` y `field`; `shader_source` y `shader_check` para leerlo y comprobarlo. Mira [acciones](acciones.md).

## Cómo importar, reimportar y quitar un efecto del paquete

**Dónde está:** pestaña **Recursos**, lista **Efectos**.

**Pasos:**

1. **Importar un efecto…** abre un cuadro de archivos: elige el descriptor `.json` y, si no está a su lado, el shader `.wgsl`.
2. **Reimportar…** en la fila de un efecto hace lo mismo y reemplaza el que tienes.
3. **Quitar** borra el efecto del paquete. Se rechaza mientras alguna capa lo use.

**En la terminal:** `wprs-ctl scene import-effect`, `scene remove-effect`; acciones `import_effect` y `remove_effect`. Mira [acciones](acciones.md).

## Cómo escribir un shader de material

Un shader de material cambia cómo se ve la superficie de un modelo 3D. Se escribe como un efecto del paquete, con la plantilla de material.

**Dónde está:** inspector de una capa de modelo, sección **Materiales**, al final. La sección solo aparece si el modelo tiene materiales. Mira [editor-3d](editor-3d.md).

**Pasos:**

1. Escribe el nombre en **Nombre de un shader de material nuevo** (minúsculas, cifras y guiones bajos) y pulsa **Shader de material nuevo**.
2. Bajo la sección aparece una fila con el estado del shader: **NOMBRE se compila**, o el error con su línea y columna.
3. En la fila hay tres acciones: **Editar el código WGSL…**, **Editar el descriptor…** y **Comprobar de nuevo**. El código y el descriptor se editan como los de un efecto.
4. En cada material, el desplegable de shader lista los del paquete con su id. Elige el tuyo.
5. Los parámetros del shader salen debajo: un deslizador para un número, un selector para un color, una casilla para un sí o no y, para una imagen, una lista con las imágenes del paquete y `white`, `black`, `noise` y `normal_flat`.
6. Un aviso recuerda que el shader corre en la tarjeta gráfica dentro de los límites de los efectos del paquete, y que solo uses shaders en los que confíes.

El shader escribe `fn material(s: Surface) -> Surface` en `effects/NOMBRE.wgsl`. Lee `frame`, `param(0u)` a `param(7u)` y hasta cuatro imágenes de sus parámetros de textura con `image0(uv)` a `image3(uv)`.

```wgsl
fn material(s: Surface) -> Surface {
    var o = s;
    let tint = param(0u) * image0(o.uv);
    o.albedo = vec4<f32>(o.albedo.rgb * tint.rgb, o.albedo.a);
    o.emissive = o.emissive + o.albedo.rgb * param(1u).x * (0.5 + 0.5 * sin(frame.time));
    return o;
}
```

**En la terminal:** `wprs-ctl scene new-material CARPETA NOMBRE` (también comprueba un shader e imprime sus errores) o la acción `scene3d` con `new_material` y `material`; la pregunta `material_check`. Mira [acciones](acciones.md).

## Efectos en el editor de imágenes

El editor de imágenes de la aplicación (se abre con **Editar…** en la ficha de una imagen de la biblioteca) tiene sus propias herramientas **Efectos** y **Máscaras**, aparte de los de la escena. No editan escenas: editan la imagen sin tocar su archivo.

### Cómo desenfocar, poner viñeta o superponer un color

**Pasos:**

1. En la lista de la izquierda, elige **Efectos**.
2. **Desenfoque**: el deslizador **Radio**.
3. **Viñeta**: **Cantidad** (negativa oscurece, positiva aclara los bordes), **Punto medio**, **Redondez** y **Difuminado**.
4. **Superposición de color**: **Elegir un color…**, el modo de mezcla (**Normal**, **Multiplicar**, **Trama**, **Superponer**, **Luz suave**, **Color**) y **Opacidad**.

**En la terminal:** `wprs-ctl edit blur`, `edit vignette` y `edit overlay`.

### Cómo limitar una edición con una máscara

**Pasos:**

1. En la lista de la izquierda, elige **Máscaras**.
2. Escribe el **Nombre de la máscara** y elige el tipo: **Degradado lineal**, **Degradado radial**, **Rango de brillo**, **Rango de profundidad** o **Pintada con pincel**.
3. Ajusta los deslizadores del tipo. En los degradados lineal y radial también puedes arrastrar los tiradores sobre la imagen: «Arrastra los tiradores sobre la imagen para colocar la máscara.»
4. Con **Pintada con pincel** pon nombre a la máscara y pinta sobre la imagen. **Tamaño del pincel** y **Dureza** controlan el pincel; **Borrar** quita lo pintado. Cada trazo es un paso de **Deshacer**.
5. **Invertir** da la vuelta a la máscara.
6. Las máscaras hechas salen como fichas sobre el nombre: púlsalas para editarlas, y la cruz de la ficha las quita.

«Limita un efecto a una parte de la imagen con una máscara en la lista de operaciones.»

**En la terminal:** `wprs-ctl edit mask`.
