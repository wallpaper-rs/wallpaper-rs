# Mallas, huesos y expresiones en el editor

Esta página explica las herramientas del editor de escenas para las capas de malla: cortar una malla desde una imagen, colocar huesos, pintar pesos, dar profundidad, cadenas que alcanzan un objetivo, física, formas de mezcla y expresiones, partes, sujetar capas y posar un hueso. Cada tarea dice dónde está, los pasos y el equivalente en la terminal. El resto del dibujo (formas, texto, mapas, sprites e imágenes con profundidad) está en [editor-dibujo](editor-dibujo.md).

Antes de empezar:

- La **vista previa** es el lienzo del editor. Las herramientas de las mallas dibujan sus asas y pinceles sobre ella.
- Las capas se añaden en la pestaña **Capas** con **Añadir una capa**; los archivos se importan en la pestaña **Recursos** con **Importar…**.
- Los nombres de campo que no tienen traducción se ven en inglés (por ejemplo "Mesh", "Hit", "Position"); aquí van entre comillas tal como aparecen.
- Cada trazo y cada arrastre que suelta el ratón es un paso que se puede deshacer. Los atajos y el resto de la ventana están en [editor](editor.md).
- Cada acción tiene su equivalente en la terminal con `wprs-ctl scene act CARPETA ACCIÓN JSON`; los argumentos están en [acciones](acciones.md).

## Qué es una malla

Una malla es una capa que se puede deformar con huesos. Todas las herramientas de esta página están en el campo de la malla del inspector (se llama "Mesh") y dibujan sobre la vista previa. Una capa de malla puede tener hasta 64 piezas. Los huesos son las articulaciones; cada hueso mueve los vértices según sus pesos.

## Cómo hacer una malla desde una imagen

**Dónde está:** pestaña **Capas** > **Añadir una capa** > **Malla desde una imagen…** (está apagada hasta que haya una imagen en los recursos).

**Pasos:**

1. Importa la imagen en **Recursos** > **Importar…**.
2. Elige **Malla desde una imagen…**. Se abre la ventana del mismo nombre.
3. Escoge la imagen en la lista.
4. Marca **Seguir el contorno de la imagen** si quieres que la malla siga el borde de lo dibujado y deje fuera lo transparente. Sin marcar, se corta en una cuadrícula.
5. Fija **Celdas a lo largo del lado mayor** (de 2 a 64; por defecto 16) y acepta el diálogo.
6. La capa aparece con la imagen como textura. Ahora coloca los huesos, como dice la ventana: **Luego coloca sus huesos en el inspector y arrástralos sobre la vista previa.**

Si ya tienes un archivo de modelo (`.glb`) en los recursos, **Añadir una capa** > **Malla** crea una capa de malla con él y se edita igual. Una capa de **Modelo 3D** no tiene la colocación de huesos: sus huesos vienen de su archivo, se ven en el inspector y los posan sus clips (ver [editor-3d](editor-3d.md)).

**En la terminal:** `mesh_from_picture` (`image`, `outline`, `cells`, `id`).

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","outline":true,"cells":12,"id":"muneco"}'
```

## Cómo colocar, mover, renombrar y quitar huesos

**Dónde está:** inspector de la capa de malla, lista **Huesos**.

**Pasos:**

1. Escribe el nombre en **Nombre de un hueso nuevo**, elige su padre en la lista (**Sin padre** o un hueso) y pulsa **Añadir**. El hueso empieza en el centro de la malla y queda elegido.
2. Pulsa **Colocar huesos sobre la vista previa** y arrastra la cabeza de cada hueso. Las líneas unen cada hueso con su padre. Pulsa un hueso (en la lista o en la vista previa) para elegirlo.
3. Para cambiar el nombre, haz doble clic en el nombre o pulsa **Cambiar el nombre**.
4. Para cambiar de padre, arrastra el asa de una fila encima de otro hueso. Para que no cuelgue de ninguno, suéltala en **Suelta aquí un hueso para que no cuelgue de otro**.
5. **Quitar** borra el hueso. Si los pesos ya estaban pintados, los de ese hueso pasan a su padre.

Cuando añades, mueves o quitas un hueso, los pesos se vuelven a calcular solos mientras no los hayas pintado. Después de pintarlos se conservan: un hueso que añadas después empieza sin peso.

Una malla puede tener hasta 128 huesos con esta herramienta.

**En la terminal:** `bone` (`layer`, `name`, y `at`, `parent`, `to` para renombrar, `remove`). `at` es una posición en fracciones de la malla (0,5 y 0,5 es el centro). Las preguntas `rig` (huesos, puntos, partes y clips de la capa) y `handles` dan el estado.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","cells":12,"id":"muneco"}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"cuerpo","at":[0.5,0.7]}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"brazo","parent":"cuerpo","at":[0.8,0.4]}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"brazo","to":"mano"}'
wprs-ctl scene ask mi-escena rig '{"layer":"muneco"}'
```

## Cómo pintar los pesos de los huesos

Los pesos dicen cuánto mueve cada hueso a cada vértice.

**Dónde está:** inspector de la malla, **Pintar los pesos sobre la vista previa** (hace falta al menos un hueso).

**Pasos:**

1. Pulsa **Pintar los pesos sobre la vista previa**. Se elige un hueso si no había.
2. Elige el hueso en **Hueso**, o pulsa uno en la vista previa o en la lista.
3. Elige el **Pincel**: **Añadir**, **Restar**, **Suavizar** o **Normalizar**. Ajusta **Fuerza**, **Tamaño del pincel** y **Borde suave**.
4. En **Mostrar** elige **Peso del hueso** (la malla se tiñe de azul, sin peso, a rojo, todo el peso) o **Hueso que manda en cada parte** (cada triángulo con el color del hueso que más pesa).
5. En **Huesos bloqueados** marca los huesos cuyo peso no debe cambiar mientras pintas.
6. Arrastra sobre la malla. Cada trazo es un paso.
7. **Pesos automáticos** vuelve a calcular todos los pesos a partir de la posición de los huesos. El texto de al lado dice si los pesos siguen a los huesos o si se conservan.

**En la terminal:** `weights` (`layer`, `bone`, `mode`, `by`, `radius`, `falloff`, `lock`, `points`) y `auto_weights`. La pregunta `weights` con el nombre del hueso da el mapa de calor.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","cells":12,"id":"muneco"}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"cuerpo","at":[0.5,0.7]}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"brazo","parent":"cuerpo","at":[0.8,0.4]}'
wprs-ctl scene act mi-escena weights '{"layer":"muneco","bone":"brazo","mode":"add","by":0.5,"radius":0.15,"points":[[0.7,0.4],[0.8,0.4]]}'
wprs-ctl scene act mi-escena auto_weights '{"layer":"muneco"}'
```

## Cómo dar profundidad a una malla

La profundidad hace que los vértices más cercanos tapen a los lejanos.

**Dónde está:** inspector de la malla, debajo de los pesos.

**Pasos:**

1. Marca **Profundidad: los vértices cercanos tapan a los lejanos**.
2. Mueve **Perspectiva de la profundidad** (de 0 a 10). También se arrastra desde la vista previa: una línea en la parte de abajo tiene un cuadrado con el valor encima.
3. Marca **Ocultar las caras vistas por detrás** si quieres esconder lo que mira hacia atrás.
4. **Dibujar las piezas de atrás hacia adelante, para que los bordes suaves dejen ver lo que hay detrás** viene marcada.
5. Para acercar o alejar vértices, pulsa **Pintar la profundidad sobre la vista previa**, fija **Píxeles hacia el espectador** (de -1000 a 1000; negativo aleja) y **Tamaño del pincel**, y arrastra sobre la malla.

**En la terminal:** `mesh_depth` para pintar (`layer`, `by`, `radius`, `points`) y `parts` con `depth` (`scale`, `cull`, `sort`, `off`) para los ajustes.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","cells":12,"id":"muneco"}'
wprs-ctl scene act mi-escena parts '{"layer":"muneco","depth":{"scale":2}}'
wprs-ctl scene act mi-escena mesh_depth '{"layer":"muneco","by":20,"radius":0.15,"points":[[0.5,0.5]]}'
```

## Cómo hacer que una cadena de huesos alcance un objetivo

Una cadena (cinemática inversa, "ik") dobla varios huesos para que el último llegue a un objetivo.

**Dónde está:** inspector de la malla, **Cadenas que alcanzan**.

**Pasos:**

1. Pulsa **Crear una cadena en la vista previa** (hacen falta dos huesos o más).
2. Haz clic en el hueso donde empieza la cadena. Luego, en el hueso donde termina, por debajo del primero.
3. La cadena aparece en la lista, con sus huesos separados por flechas. Elige a qué llega: **Alcanza Un punto**, **Alcanza El puntero**, un hueso o punto de la malla u otra capa.
4. **Peso** mezcla el efecto con la pose normal (de 0 a 1).
5. **Doblar hacia el otro lado** cambia el sentido en que se dobla. En la vista previa lo hace también el botón redondo junto a la cadena: **Se dobla en sentido horario; pulsa para doblarla al otro lado**.
6. Para limitar el giro de un hueso, elígelo en la lista de la cadena y pulsa **Limitar**; **Libre** quita el límite. Cada límite es un arco alrededor del hueso con un punto en cada extremo: arrástralos para cambiar el ángulo menor y el mayor.
7. Una línea discontinua une el final de la cadena con su objetivo; si el objetivo es un punto, arrastra el anillo para moverlo.

Si la cadena sigue al puntero, la escena necesita el permiso del puntero y avisa al usuario.

**En la terminal:** `ik` (`layer`, `root`, `tip`, `target`, `index`, `bend`, `weight`, `limits`, `unlimit`, `remove`). `target` es `"cursor"` (el puntero), `"bone:NOMBRE"` (un hueso o punto de la malla), `"layer:ID"` (otra capa) o un punto `[x, y]` en píxeles del lienzo.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","cells":12,"id":"muneco"}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"cuerpo","at":[0.5,0.7]}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"brazo","parent":"cuerpo","at":[0.8,0.4]}'
wprs-ctl scene act mi-escena ik '{"layer":"muneco","root":"cuerpo","tip":"brazo","target":[900,500]}'
wprs-ctl scene act mi-escena ik '{"layer":"muneco","index":0,"bend":"counterclockwise","weight":0.8,"limits":{"brazo":[-45,45]}}'
```

## Cómo hacer que los huesos se balanceen: física, cuerda y viento

**Dónde está:** inspector de la malla, **Huesos con física**, **Viento**.

**Pasos:**

1. Pulsa **Elegir en la vista previa los huesos que se balancean** y haz clic en un hueso para que se balancee; otro clic en él lo detiene.
2. Cada hueso con física aparece en la lista con su modo (**Muelle**, **Rígido** o "Rope") y cinco deslizadores: "Stiffness", **Amortiguación**, "Inertia", "Gravity" (de -10 a 10) y "Wind" (cuánto le afecta el viento).
3. Marca **Acotar su giro, en grados** y **Acotar cuánto se desliza su cabeza, en píxeles** para poner límites con un deslizador de dos extremos (empiezan en -30 y 30).
4. En la vista previa cada hueso tiene un anillo en la cabeza y una flecha de gravedad: arrastra el cuadrado de la punta hacia arriba o abajo para fijarla (**Arrastra hacia arriba o abajo para fijar la gravedad de NOMBRE**). Un giro acotado es un arco con un punto en cada extremo.
5. **Dibujar una cuerda en la vista previa**: arrastra por donde cuelga la cuerda. Recibe hasta 9 huesos que cuelgan del hueso elegido o del más cercano.
6. **Simular** reproduce la vista previa con la física, o la deja quieta. **Restablecer** la vuelve a lanzar desde la pose.
7. En **Viento**, ajusta **Ángulo** (de -180 a 180 grados), **Fuerza** (de 0 a 1000 px/s) y **Ráfagas**. **Mostrar en la vista previa** dibuja el viento como una flecha desde el centro del lienzo y se arrastra su punta. **Sin viento** lo apaga. Si el viento sigue un enlace o fotogramas clave, se avisa y fijarlo aquí lo vuelve fijo.

**En la terminal:** `physics` (`layer`, `bones`, `mode`, `stiffness`, `damping`, `inertia`, `gravity`, `wind`, `rotation`, `translation`, `free`, `rope`, `parent`, `remove`) y `wind` (`angle`, `strength`, `gusts`, `off`). La pregunta `wind` da la flecha.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","cells":12,"id":"muneco"}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"cuerpo","at":[0.5,0.7]}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"cola","parent":"cuerpo","at":[0.2,0.8]}'
wprs-ctl scene act mi-escena physics '{"layer":"muneco","bones":["cola"],"mode":"spring","stiffness":0.4,"gravity":3}'
wprs-ctl scene act mi-escena physics '{"layer":"muneco","bones":["soga"],"rope":[[0.3,0.2],[0.3,0.35],[0.3,0.5]],"parent":"cuerpo"}'
wprs-ctl scene act mi-escena wind '{"angle":90,"strength":120,"gusts":0.4}'
```

## Cómo elegir los huesos que reciben el puntero

**Dónde está:** inspector de la malla, **Huesos que reciben el puntero**.

**Pasos:**

1. Pulsa **Elegirlos en la vista previa**.
2. Haz clic en un hueso para que reciba el puntero; otro clic lo suelta. Los que lo reciben se dibujan con un anillo grueso.
3. También se puede elegir un hueso en la lista de abajo y pulsar **Añadir**; cada fila tiene **Quitar**.

Los scripts de la escena saben cuál de estos huesos tocó el puntero, si la escena tiene el permiso del puntero.

**En la terminal:** `set` sobre el campo "Hit" de la capa de malla, con una lista de nombres de hueso como valor (por ejemplo, `{"pointer":"/layers/0/hit","value":["mano"]}` si la malla es la primera capa).

## Cómo pasar un hueso a otro padre con una regla

**Dónde está:** inspector de la malla, **Reglas que pasan un hueso a otro padre**.

**Pasos:**

1. En **Regla nueva** elige el **Hueso** y el **Nuevo padre**, y pulsa **Añadir una regla**.
2. Cada regla se lista como "hueso a padre". Cambia su padre, ajusta su **Peso** (de 0 a 1) o pulsa **Quitar**.

**En la terminal:** `bone_rule` (`layer`, `bone`, `parent`, `weight`, `index`, `remove`).

## Cómo esculpir una forma de mezcla y guardar expresiones

Una forma de mezcla es un cambio de la malla (una sonrisa, un parpadeo) que se mezcla con un peso. Una expresión es un conjunto guardado de pesos.

**Dónde está:** inspector de la malla.

**Pasos para esculpir:**

1. Pulsa **Esculpir una forma de mezcla en la vista previa**. El botón se activa cuando hay una forma elegida o un nombre escrito en el paso siguiente.
2. En **Forma** elige una existente, o **Forma nueva…** y escribe su **Nombre** (la forma se crea con el primer trazo).
3. Elige el **Pincel**: **Empujar hacia fuera**, **Tirar hacia dentro**, **Suavizar** o **Borrar**. Ajusta **Fuerza**, **Tamaño del pincel** y **Borde suave**.
4. Arrastra sobre la malla. La vista previa enseña la forma con todo su peso mientras esculpes. Cada trazo es un paso.

**Pasos para las expresiones:**

1. Cuando la malla ya tiene formas de mezcla, **Formas de mezcla en la vista previa** tiene un deslizador por forma, de -1 a 1. Solo cambian la vista previa. **Restablecer la vista previa** los apaga.
2. Escribe un nombre en **Nombre de una expresión nueva** y pulsa **Guardar los deslizadores**.
3. En **Expresiones**, cada una tiene tres botones: **Mostrar en la vista previa para editar**, **Guardar los deslizadores en esta expresión** (para guardarla de nuevo) y **Quitar**. Al editar una aparece un campo para **Cambiar el nombre**.
4. **Expresiones en la vista previa** tiene un deslizador por expresión, de 0 a 1, que las mezcla.

**En la terminal:** `shape_sculpt` (`layer`, `shape`, `mode` con `push`, `pull`, `smooth` o `erase`, `by`, `radius`, `falloff`, `points`) y `expression` (`layer`, `name`, `shapes`, `to`, `remove`).

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","cells":12,"id":"muneco"}'
wprs-ctl scene act mi-escena shape_sculpt '{"layer":"muneco","shape":"sonrisa","mode":"push","by":10,"radius":0.1,"points":[[0.5,0.5]]}'
wprs-ctl scene act mi-escena expression '{"layer":"muneco","name":"feliz","shapes":{"sonrisa":1}}'
```

## Cómo ordenar las partes de una malla

Si la malla tiene más de una parte, el inspector las lista.

**Dónde está:** inspector de la malla, **Partes, de atrás hacia delante**.

**Pasos:**

1. Arrastra el asa de una fila a otro lugar para cambiar el orden.
2. La casilla **Visible** de cada parte la muestra u oculta.
3. Pulsa el nombre de la parte para marcarla: se dibuja con su contorno sobre la vista previa.
4. La lista de la derecha de cada fila dice **Sobre todo** o **Solo sobre OTRA PARTE**: la parte solo se ve sobre la otra, que se dibuja también con su contorno.

**En la terminal:** `parts` (`layer`, `order`, `show`, `hide`, `clip`, `unclip`).

## Cómo sujetar capas a una malla

Una capa sujeta sigue a un hueso o a un punto de la malla.

**Dónde está:** inspector de la malla, **Capas que siguen esta malla** y **Puntos de sujeción**.

**Pasos:**

1. Pulsa **Sujetar capas en la vista previa** (hace falta otra capa). Debajo aparece una ficha por cada otra capa.
2. Arrastra una ficha sobre un hueso: la capa lo sigue. Suéltala en otro lugar de la malla: se crea allí un punto de sujeción. O pulsa una ficha y luego haz clic en la vista previa.
3. Marca qué hereda la capa: posición, rotación y escala ("Position", "Rotation", "Scale").
4. Una línea punteada une cada capa con su hueso o punto, y termina en un anillo: arrástralo para mover la capa a lo largo de lo que sigue.
5. La lista **Capas que siguen esta malla** dice, por ejemplo, **sombrero sigue a brazo**; el botón de cada fila la suelta.
6. En **Puntos de sujeción** cada punto tiene su nombre (se edita ahí), el hueso del que cuelga (**Ningún hueso** si no cuelga de ninguno) y **Quitar el punto**. **Añadir un punto** pone uno en el centro. En la vista previa cada punto es un cuadrado que se arrastra.

**En la terminal:** `attach` (`layer`, `to`, `bone` o `point`, `inherit`, `nudge`, `detach`) y `point` (`layer`, `name`, `at`, `parent`, `to`, `remove`).

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","cells":12,"id":"muneco"}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"cuerpo","at":[0.5,0.7]}'
wprs-ctl scene act mi-escena point '{"layer":"muneco","name":"mano","parent":"cuerpo","at":[0.9,0.4]}'
wprs-ctl scene act mi-escena add_layer '{"kind":"shape","id":"sombrero"}'
wprs-ctl scene act mi-escena attach '{"layer":"sombrero","to":"muneco","bone":"cuerpo","inherit":["position","rotation"]}'
wprs-ctl scene act mi-escena attach '{"layer":"sombrero","nudge":[10,0]}'
```

## Cómo posar un hueso con las asas

Para animar una malla se posa un hueso en un instante de un clip y se guarda como clave.

**Dónde está:** inspector de la malla, **Editar los clips en la línea de tiempo**.

**Pasos:**

1. Pulsa **Editar los clips en la línea de tiempo**. Si la malla no tenía ningún clip, se crea uno vacío. El clip se elige, se reproduce y se hace bucle como se explica en [editor-animacion](editor-animacion.md).
2. Elige un hueso: pulsa su nombre en la lista o su punto en la vista previa.
3. Pon el cabezal en el segundo que quieras. El hueso elegido muestra tres asas: un círculo en su cabeza para **Mover el hueso**, un punto sobre su eje para **Girar el hueso** y un cuadrado para **Escalar el hueso**. Los valores se miden en el espacio del padre, tal como el clip los guarda.
4. Arrastra las asas. Solo la vista previa cambia.
5. Pulsa **Clave**. Guarda los canales que moviste; si no moviste ninguno, la pose completa del hueso en ese segundo. **Clave** está apagada si no hay hueso elegido.
6. **Volver a las líneas de tiempo** cierra el modo de clips.

La **Clave** de los canales, la curva, las claves que se mueven y los eventos del clip están en [editor-animacion](editor-animacion.md). **Clip de la vista previa** elige qué clip reproduce la vista previa de la malla.

**En la terminal:** `clip` (`layer`, `id`, `add`, `length`, `loop`) y `key` (`layer`, `clip`, `bone`, `time`, `rotation`, `position`, `scale`, `ease`). La pregunta `pose` da la malla posada y `keys`, las claves de un clip.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","cells":12,"id":"muneco"}'
wprs-ctl scene act mi-escena bone '{"layer":"muneco","name":"brazo","at":[0.8,0.4]}'
wprs-ctl scene act mi-escena clip '{"layer":"muneco","id":"saludo","add":true,"length":1}'
wprs-ctl scene act mi-escena key '{"layer":"muneco","clip":"saludo","bone":"brazo","time":0.5,"rotation":30}'
wprs-ctl scene ask mi-escena pose '{"layer":"muneco","clip":"saludo","time":0.5}'
```

