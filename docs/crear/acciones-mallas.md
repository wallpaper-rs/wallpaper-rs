# Acciones: mallas, huesos y animación

Las acciones y preguntas para crear mallas, colocar huesos, pintar pesos, hacer clips y claves, y mover con cinemática y física. Cómo llamarlas, cómo leer su salida y sus errores está en [acciones](acciones.md).

## Acciones

### Antes de empezar con mallas, huesos y animación

- Una **capa de malla** muestra una imagen sobre una malla de triángulos que se deforma con huesos. Una **capa de modelo** muestra un modelo 3D de un archivo `.glb`. Casi todas las acciones de esta sección piden el id de una de las dos en `layer`. Las que solo sirven para mallas o solo para modelos lo dicen.
- Una malla se crea de dos maneras: con `mesh_from_picture` a partir de una imagen, o con `add_layer` de tipo `mesh` y un archivo `.glb` (en la app, **Añadir una capa**).
- Los puntos de un trazo y los lugares de huesos y puntos son `[x, y]` en fracciones de la capa: `[0, 0]` es la esquina de arriba a la izquierda y `[1, 1]` la de abajo a la derecha. Un radio de pincel también es una fracción, del lado más largo de la capa.
- Una capa de malla creada desde una imagen mide lo mismo que la imagen. Las medidas en píxeles de esta sección (profundidad, posición de una clave, destino de una cadena) son píxeles de esa capa.
- Los ángulos van en grados. Un ángulo positivo gira el hueso en el sentido de las agujas del reloj, tal como lo ves en pantalla.
- Un **clip** es una animación con nombre de una capa de malla o de modelo. Para que se reproduzca, la capa necesita una **capa de animación** que lo toque: `animate` con la variante `layer`, o `clip` con `play`.
- Cada cambio se puede deshacer de un paso, también los trazos de pincel. Los trazos que cambian la forma de la malla guardan una malla nueva y dejan la anterior para deshacer.

### Crear una malla

#### `mesh_from_picture`

Corta una imagen de la escena en triángulos y añade una capa de malla que la muestra. Es el primer paso para dar huesos a una imagen. En la app es **Añadir una capa**, **Malla desde una imagen…**, con las opciones **Seguir el contorno de la imagen** y **Celdas a lo largo del lado mayor**.

La imagen tiene que estar importada antes (`import`). El id de la capa es también el de la malla.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `image` | texto (obligatorio) | Id de la imagen de la escena que se corta. |
| `outline` | sí/no | `false` (por defecto) cubre con una cuadrícula las celdas que tienen algo opaco. `true` sigue el borde de la parte opaca más grande y pone puntos dentro. |
| `cells` | entero | Celdas a lo largo del lado más largo, de 2 a 64. Por defecto 16. Un valor fuera de ese rango se ajusta al más cercano. |
| `id` | texto | Id de la capa nueva. Si falta, se forma con el de la imagen y `_mesh`. Un id en uso se rechaza. |

La imagen no puede pasar de 8192 píxeles por lado.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
```

Con el contorno y más celdas:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla","outline":true,"cells":32}'
```

La malla recién cortada tiene una sola pieza llamada `body` y ningún hueso. Para ver cómo quedó el borde, usa la pregunta `outline`.

#### `mesh_depth`

Sube o baja la profundidad de los vértices de una malla a lo largo de un trazo. Sirve para dar relieve a la malla: los vértices más cercanos tapan a los más lejanos. En la app, **Pintar la profundidad sobre la vista previa**, con **Píxeles hacia el espectador** y **Tamaño del pincel**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla. |
| `points` | lista de `[x, y]` (obligatorio) | El trazo, de 1 a 1024 puntos, en fracciones de la capa. |
| `radius` | número (obligatorio) | Tamaño del pincel como fracción del lado más largo: mayor que 0 y hasta 1. |
| `by` | número (obligatorio) | Píxeles hacia el espectador en el centro del pincel, de -1000 a 1000. Se desvanece hasta 0 en el borde del pincel. Un valor negativo aleja los vértices. |

Se conservan los huesos, los pesos y las formas de mezcla.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena mesh_depth '{"layer":"malla","points":[[0.3,0.5],[0.7,0.5]],"radius":0.2,"by":40}'
```

Para que la profundidad se note hay que activarla en la capa con `parts` y su opción `depth`.

### Huesos y pesos

Los **pesos** dicen cuánto manda cada hueso sobre cada vértice. Cada vértice se reparte entre hasta 4 huesos y sus pesos suman 1.

Mientras los pesos son automáticos, siguen a los huesos: al añadir, mover o quitar un hueso, la malla se vuelve a atar sola. Cuando pintas pesos con `weights`, pasan a ser pintados y se conservan aunque cambien los huesos. `auto_weights` los devuelve a automáticos. Una malla recién cortada de una imagen empieza con pesos automáticos.

#### `bone`

Añade, mueve, cambia de padre, renombra o quita un hueso de una malla. En la app, el árbol **Huesos** del inspector de la capa y **Colocar huesos sobre la vista previa**, que arrastra la cabeza de cada hueso.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla. |
| `name` | texto (obligatorio) | Nombre del hueso, de 1 a 64 caracteres sin `/` ni `#`. Si no existe, se crea. No puede repetir el de otro hueso, punto o pieza de la malla. |
| `parent` | texto | Hueso del que cuelga. Un texto vacío lo deja sin padre. Si lo omites en un hueso existente, no cambia. |
| `at` | `[x, y]` | Dónde va su cabeza, en fracciones de la capa. Un hueso nuevo sin `at` va al centro. Los hijos del hueso movido conservan su lugar. |
| `to` | texto | Nombre nuevo. Los clips, capas de animación, cadenas, cuerpos con física, reglas, huesos que reciben el puntero y capas sujetas a ese hueso lo siguen. |
| `remove` | sí/no | `true` quita el hueso: sus hijos cuelgan de su padre y este recibe sus pesos. Con él también se quitan las claves, cadenas, cuerpos y reglas que lo nombraban. Un hueso que lleva una pieza de la malla no se puede quitar. |

Una malla admite hasta 128 huesos. Un hueso no puede colgar de sí mismo ni de sus hijos.

Crear tres huesos encadenados:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"brazo","at":[0.5,0.2]}'
```

Mover, renombrar, cambiar de padre y quitar:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","at":[0.6,0.5]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","to":"antebrazo"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"antebrazo","parent":""}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"antebrazo","remove":true}'
```

Para ver los huesos de una capa, usa la pregunta `rig`.

#### `auto_weights`

Ata de nuevo cada vértice a sus huesos más cercanos y hace que los pesos sigan a los huesos hasta que vuelvas a pintarlos. Descarta los pesos pintados. En la app, **Pesos automáticos**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla. Necesita al menos un hueso. |

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena auto_weights '{"layer":"malla"}'
```

#### `weights`

Pinta los pesos de un hueso con un trazo de pincel. En la app, **Pintar los pesos sobre la vista previa**: eliges el **Hueso**, el **Pincel** (**Añadir**, **Restar**, **Suavizar**, **Normalizar**), la **Fuerza**, el **Tamaño del pincel**, el **Borde suave** y los **Huesos bloqueados**. **Mostrar** cambia entre el **Peso del hueso** (azul sin peso, rojo todo el peso) y el **Hueso que manda en cada parte**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla. |
| `bone` | texto (obligatorio) | Hueso que se pinta. |
| `points` | lista de `[x, y]` (obligatorio) | El trazo, de 1 a 1024 puntos, en fracciones de la capa. |
| `mode` | texto (obligatorio) | `add` suma peso al hueso. `subtract` se lo resta. `smooth` acerca el peso de cada vértice al de sus vecinos. `normalise` quita los pesos menores que `by`. |
| `by` | número (obligatorio) | Fuerza en el centro del pincel, de 0 a 1. |
| `radius` | número | Tamaño del pincel como fracción del lado más largo, de 0 (sin incluir) a 1. Por defecto 0,1. |
| `falloff` | número | Cuánto del pincel es borde suave, de 0 (duro) a 1 (todo suave). Por defecto 0,5. |
| `lock` | lista de textos | Huesos cuyo peso el trazo no toca. Por defecto ninguno. El hueso que pintas no puede estar entre ellos. |

Si un vértice ya tiene 4 huesos, el trazo hace sitio quitando peso a los otros. El peso que cede el hueso pintado pasa a su padre o a otro hueso que sujete el vértice.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena weights '{"layer":"malla","bone":"brazo","points":[[0.5,0.4],[0.5,0.6]],"mode":"add","by":0.8}'
```

Suavizar, protegiendo la raíz, con un pincel mayor:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena weights '{"layer":"malla","bone":"brazo","points":[[0.5,0.5]],"mode":"smooth","by":0.5,"radius":0.3,"falloff":0.8,"lock":["raiz"]}'
```

El mapa de calor del resultado lo da la pregunta `weights` (la pregunta, no esta acción).

### Clips y claves

#### `clip`

Crea, cambia o quita un clip de una capa de malla o de modelo. En la app, **Editar los clips en la línea de tiempo** (en el inspector de la capa) abre la barra de clips: **Añadir un clip**, **Añadir un clip y reproducirlo en una capa de animación nueva**, **Cambiar el nombre del clip**, **Quitar el clip**, la **Longitud** y **Repetir**, **Ida y vuelta** o **Una vez**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `id` | texto (obligatorio) | Id del clip: de 1 a 32 letras minúsculas, cifras o `_`. |
| `add` | sí/no | `true` crea el clip. Por defecto dura 2 segundos. En una malla sin animaciones propias empieza con una clave en 0 s con el reposo de su primer hueso (o el orden de su primera pieza). Un id repetido se rechaza. Se admiten hasta 64 clips por capa. |
| `remove` | sí/no | `true` quita el clip y las capas de animación que lo tocan. |
| `to` | texto | Id nuevo. Las capas de animación que lo tocan lo siguen. |
| `length` | número | Duración en segundos, mayor que 0 y hasta 3600. |
| `loop` | texto | `once` (una vez), `repeat` (repetir, el valor por defecto) o `pingpong` (ida y vuelta). |
| `animation` | texto | Nombre de una animación del archivo `.glb` de la capa. El clip la reproduce entera y toma su duración. |
| `from` | número | Segundo de la animación del archivo donde empieza el clip. Mantiene la duración del clip dentro de la animación. |
| `play` | sí/no | Solo junto con `add`: añade además una capa de animación que reproduce el clip nuevo (hasta 16 por capa). |

Un clip con una animación del archivo (`animation`) reproduce el movimiento del archivo; las claves que añadas con `key` se suman encima.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":1.5,"loop":"pingpong","play":true}'
```

Cambiar la duración, renombrar y quitar:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","length":3,"loop":"once"}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","to":"adios"}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"adios","remove":true}'
```

Un clip que reproduce una animación del archivo de un modelo:

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"model","file":"modelo.glb","id":"muneco"}'
wprs-ctl scene act mi-escena clip '{"layer":"muneco","id":"paso","add":true,"animation":"walk"}'
```

Los clips y las animaciones del archivo de una capa los lista la pregunta `motion`.

#### `key`

Pone, mueve, copia o quita una clave de un clip. Una clave fija el valor de un canal en un segundo, y entre dos claves el valor se mueve poco a poco. En la app, la línea de tiempo de clips: el botón **Clave** escribe los canales que cambiaron las asas del hueso elegido, **Curva de la clave** curva el tramo hasta la clave siguiente y **Quitar la clave** la borra.

Cada clave es de una sola cosa: un hueso, una forma de mezcla o el orden de una pieza.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `clip` | texto (obligatorio) | Id del clip. |
| `time` | número (obligatorio) | Segundo de la clave, contado desde el inicio del clip y redondeado al milisegundo. De 0 a 3600. |
| `bone` | texto | Hueso de la clave. |
| `rotation` | número | Solo malla: giro del hueso en grados. |
| `position` | `[x, y]` | Solo malla: cuánto se desplaza el hueso, en píxeles de la capa. `x` hacia la derecha, `y` hacia abajo. |
| `scale` | `[x, y]` | Solo malla: factor de tamaño del hueso. 1 es el tamaño de reposo. |
| `angles` | `[x, y, z]` | Solo modelo: giro del hueso en grados en cada eje. |
| `shift` | `[x, y, z]` | Solo modelo: desplazamiento del hueso en unidades del modelo. |
| `shape` | texto | Forma de mezcla de la clave. |
| `weight` | número | Peso de la forma, de -1 a 1. Sin él se usa el valor que el clip da en ese segundo. |
| `order` | texto | Pieza cuyo orden de dibujo se anima. |
| `value` | número | Orden de la pieza (se redondea a un entero). Sin él se usa el valor que el clip da en ese segundo. |
| `ease` | texto o lista de 4 números | Cómo se mueve el valor hasta la clave siguiente: `linear` (por defecto), `step`, `hold`, o `in_`, `out_` o `in_out_` seguido de `sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` o `bounce` (por ejemplo `in_out_sine`). También cuatro números `[x1, y1, x2, y2]` de una curva de Bézier cúbica. Una clave con `ease` pierde sus asas. |
| `in_handle` | `[dt, dv]` | Asa de Bézier con la que la curva llega a la clave: segundos hasta la clave anterior y cuánto cambia el valor allí. Curva un solo canal. No se combina con `ease`. |
| `out_handle` | `[dt, dv]` | Lo mismo para la curva que sale hacia la clave siguiente. |
| `remove` | sí/no | `true` quita la clave de ese segundo. |
| `from` | número | Segundo de una clave existente: la mueve a `time`, o la copia si pones `copy`. |
| `copy` | sí/no | Con `from`: deja la clave original y pone una copia. |

Con un hueso de malla y sin valores, la clave toma los tres canales (`rotation`, `position`, `scale`) con la pose que el clip da en ese segundo. Con valores, solo se tocan los canales que nombras. En un modelo, un hueso sin valores toma `angles` y `shift`.

Los rangos son: `rotation` y `position` hasta 100000 en valor absoluto, `scale` hasta 1000. Las asas se extienden desde 0 s hasta la clave vecina.

**Clave de un hueso de una malla**

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40,"ease":"in_out_sine"}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":2,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","position":[10,5],"scale":[1.2,1.2]}'
```

Mover una clave, copiarla y quitarla:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1.5,"bone":"brazo","from":1}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0.5,"bone":"brazo","from":1.5,"copy":true}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0.5,"bone":"brazo","remove":true}'
```

Con asas de Bézier:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0,"bone":"brazo","rotation":0,"out_handle":[0.4,25]}'
```

**Clave de un hueso de un modelo**

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"model","file":"modelo.glb","id":"muneco"}'
wprs-ctl scene act mi-escena clip '{"layer":"muneco","id":"cabeceo","add":true,"length":1}'
wprs-ctl scene act mi-escena key '{"layer":"muneco","clip":"cabeceo","time":0.5,"bone":"tip","angles":[0,20,0],"shift":[0,0.1,0]}'
```

**Clave de una forma de mezcla**

La forma de mezcla tiene que existir: la trae el archivo o la creas con `shape_sculpt`.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena shape_sculpt '{"layer":"malla","shape":"sonrisa","points":[[0.4,0.5],[0.6,0.5]],"mode":"push","by":20,"radius":0.2}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"shape":"sonrisa","weight":1}'
```

**Clave del orden de una pieza**

Anima el orden de dibujo de una pieza (ver `parts`). Pide una pieza que exista.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"cambio","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"cambio","time":1,"order":"body","value":5}'
```

Las claves de un clip las muestra la pregunta `keys` y la pose que dan, la pregunta `pose`.

#### `edit_keys`

Edita varias claves a la vez, ya elegidas: las mueve en el tiempo, estira los tiempos, sube o fija su valor, curva o endereza sus asas, las quita o pega claves copiadas. Es la acción del **Editor de curvas** de la app (el botón **Editor de curvas**): **Estirar tiempos ×**, **Escalar valores ×**, **Asas alineadas** o **Asas rotas**, **Partes unidas** o **Partes separadas**, **Enderezar**, **Copiar**, **Pegar en el cabezal** y **Quitar las claves elegidas**. Sirve para cualquier valor animado de la escena, no solo para los clips.

Cada llamada hace una sola cosa, y es un solo paso de deshacer.

Cada elemento de `picks` elige claves:

| Campo de `picks` | Tipo | Qué significa |
|---|---|---|
| `pointer` | texto | Dónde está el valor animado en el archivo de la escena. Para un canal de un clip es `/scene/layers/N/clips/M/bones/HUESO/rotation` (también `position`, `scale`, `angles`, `shift`), `/scene/layers/N/clips/M/shapes/FORMA` o `/scene/layers/N/clips/M/order/PIEZA`. `N` es el lugar de la capa en la escena y `M` el del clip en la capa, los dos contados desde 0. La pregunta `motion` da el de la capa (`pointer`) y el orden de sus clips. |
| `times` | lista de números | Segundos de las claves elegidas, tal como están escritos en el archivo. Si el clip empieza más allá del segundo 0 de su animación (`from` en `clip`), suma ese valor a los tiempos que ves en la barra de clips. |
| `keys` | lista de claves | Solo para pegar: claves copiadas, cada una `[tiempo, valor]` o `[tiempo, valor, curva]`. |

| Argumento | Tipo | Qué significa |
|---|---|---|
| `picks` | lista de objetos (obligatorio) | Las claves sobre las que se actúa. |
| `shift` | número | Segundos que se suman a los tiempos de las claves elegidas. Por defecto 0. |
| `scale` | número | Estira los tiempos de las claves elegidas, y el largo de sus asas, alrededor de `pivot`. Mayor que 0. |
| `pivot` | número | Segundo alrededor del cual estira `scale`. Por defecto 0. |
| `part` | entero | Qué parte de un valor de varias partes tocan `lift`, `value`, `value_scale` y las asas: 0 es la primera (x o rojo), 1 la segunda (y o verde), y así. |
| `lift` | número | Se suma al valor de las claves elegidas. Sin `part`, a todas las partes (en un color, menos el alfa). Por defecto 0. |
| `value` | número | Fija el valor de la parte `part`. En listas y colores `part` es obligatorio. |
| `value_scale` | número | Multiplica el valor de las claves elegidas alrededor de `value_pivot`. Escala también las asas. |
| `value_pivot` | número | Valor alrededor del cual escala `value_scale`. Sin él, el medio de los valores elegidos de cada parte. |
| `in_handle` | `[dt, dv]` | Pone el asa de entrada de una clave. |
| `out_handle` | `[dt, dv]` | Pone el asa de salida de una clave. |
| `aligned` | sí/no | Con una sola asa dada: la otra queda alineada con la misma pendiente (con su propio largo, o un tercio del tramo si no tenía). Por defecto `false`. |
| `link` | sí/no | Junta las asas por parte de una clave en un solo par para todas las partes, con la forma de `part` (la primera si falta). |
| `straighten` | sí/no | Endereza la curva entre la clave elegida y la siguiente: quita las asas de ese tramo. Va sola. |
| `remove` | sí/no | Quita las claves elegidas. Siempre queda al menos una clave en cada valor. |
| `at` | número | Para pegar: segundo donde cae la primera de las claves pegadas. Las claves de esos segundos se reemplazan. |

Para curvar asas (`in_handle`, `out_handle`, `link`, `straighten`) se elige una sola clave: un `pick` con un solo tiempo. Se rechazan claves que quedarían antes de 0 s o a menos de un milisegundo de otra, y asas que pasan de la clave vecina.

Primero un clip con una rotación animada:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":2,"bone":"brazo","rotation":0}'
```

Mover una clave medio segundo y subir su valor:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":2,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/rotation","times":[1]}],"shift":0.5}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/rotation","times":[1.5]}],"lift":10}'
```

Estirar todos los tiempos a la mitad:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":2,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/rotation","times":[0,1,2]}],"scale":0.5}'
```

Multiplicar los valores de dos claves alrededor de 0 y quitar una:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0,"bone":"brazo","rotation":10}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":2,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/rotation","times":[0,1]}],"value_scale":0.5,"value_pivot":0}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/rotation","times":[2]}],"remove":true}'
```

Fijar una sola parte de un valor de dos partes (la `y` de una posición):

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","position":[10,5]}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/position","times":[1]}],"part":1,"value":30}'
```

Asas: poner una de salida con la de entrada alineada, enderezar el tramo y unir las partes:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":2,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","position":[10,5]}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/rotation","times":[1]}],"out_handle":[0.3,10],"aligned":true}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/rotation","times":[1]}],"straighten":true}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/position","times":[1]}],"link":true}'
```

Pegar claves copiadas:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":0,"bone":"brazo","rotation":0}'
wprs-ctl scene act mi-escena edit_keys '{"picks":[{"pointer":"/scene/layers/0/clips/0/bones/brazo/rotation","keys":[[0,0],[0.4,25]]}],"at":1.2}'
```

#### `clip_event`

Añade, mueve, renombra o quita un evento de un clip. Un evento es una marca con nombre en un segundo del clip: se dispara cuando el clip pasa por ella, y los disparadores y los scripts pueden arrancar con su nombre. En la app, la franja **Eventos** sobre las filas del clip: doble clic añade un evento, arrastrar la marca lo mueve y un clic abre **Nombre del evento**, **Cambiar el nombre** y **Quitar**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `clip` | texto (obligatorio) | Id del clip. |
| `name` | texto (obligatorio) | Nombre del evento, de 1 a 64 caracteres sin `/` ni `#`. |
| `time` | número | Segundo del evento, desde el inicio del clip y dentro de su duración. Si ya hay un evento con ese nombre, lo mueve. Si no, lo crea. |
| `at` | número | Segundo en el que está hoy el evento, para distinguir eventos con el mismo nombre. |
| `to` | texto | Nombre nuevo. |
| `remove` | sí/no | `true` quita el evento. |

Un clip admite hasta 64 eventos y los guarda ordenados por tiempo.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena clip_event '{"layer":"malla","clip":"saludo","name":"hola","time":0.5}'
```

Mover, renombrar y quitar:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena clip_event '{"layer":"malla","clip":"saludo","name":"hola","time":0.5}'
wprs-ctl scene act mi-escena clip_event '{"layer":"malla","clip":"saludo","name":"hola","time":1.2}'
wprs-ctl scene act mi-escena clip_event '{"layer":"malla","clip":"saludo","name":"hola","to":"adios"}'
wprs-ctl scene act mi-escena clip_event '{"layer":"malla","clip":"saludo","name":"adios","remove":true}'
```

### Formas de mezcla y expresiones

Una **forma de mezcla** es una deformación guardada de la malla (una sonrisa, un ojo cerrado). Su peso va de 0 (sin efecto) a 1 (toda la deformación). Una **expresión** es un conjunto de formas con su peso, guardado con un nombre.

#### `shape_sculpt`

Esculpe una forma de mezcla de una malla con un pincel. Con un nombre nuevo, el primer trazo crea la forma. En la app, **Esculpir una forma de mezcla en la vista previa**: **Forma** (o **Forma nueva…**), el **Pincel** (**Empujar hacia fuera**, **Tirar hacia dentro**, **Suavizar**, **Borrar**), **Fuerza**, **Tamaño del pincel** y **Borde suave**. La vista previa muestra la forma con todo su peso mientras esculpes.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla (no vale un modelo). |
| `shape` | texto (obligatorio) | Nombre de la forma, de 1 a 64 caracteres sin `/` ni `#`. Un nombre que la malla no tiene crea la forma. Hasta 64 por malla. |
| `points` | lista de `[x, y]` (obligatorio) | El trazo, de 1 a 1024 puntos, en fracciones de la capa. |
| `mode` | texto (obligatorio) | `push` aleja del trazo los vértices bajo el pincel. `pull` los acerca. `smooth` mezcla la forma de cada vértice con la de sus vecinos. `erase` la vuelve a nada. |
| `by` | número (obligatorio) | Con `push` y `pull`, píxeles de 0 a 1000 en el centro del pincel. Con `smooth` y `erase`, de 0 a 1. |
| `radius` | número | Tamaño del pincel como fracción del lado más largo, de 0 (sin incluir) a 1. Por defecto 0,1. |
| `falloff` | número | Cuánto del pincel es borde suave, de 0 a 1. Por defecto 0,5. |

Cambia solo los desplazamientos de esa forma.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena shape_sculpt '{"layer":"malla","shape":"sonrisa","points":[[0.4,0.5],[0.6,0.5]],"mode":"push","by":20,"radius":0.2}'
```

Suavizar y borrar:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena shape_sculpt '{"layer":"malla","shape":"sonrisa","points":[[0.5,0.5]],"mode":"pull","by":20}'
wprs-ctl scene act mi-escena shape_sculpt '{"layer":"malla","shape":"sonrisa","points":[[0.5,0.5]],"mode":"smooth","by":0.5}'
wprs-ctl scene act mi-escena shape_sculpt '{"layer":"malla","shape":"sonrisa","points":[[0.5,0.5]],"mode":"erase","by":1}'
```

#### `expression`

Crea, cambia, renombra o quita una expresión: un nombre al que va ligado un reparto de pesos de formas de mezcla. En la app, la lista **Expresiones**: **Guardar los deslizadores en esta expresión**, **Guardar los deslizadores**, **Cambiar el nombre** y **Quitar**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `name` | texto (obligatorio) | Nombre de la expresión, de 1 a 64 caracteres. Si no existe, se crea. |
| `shapes` | objeto | Pesos de las formas, como `{"sonrisa": 1}`: cada forma debe existir y su peso va de -1 a 1. Reemplaza el reparto anterior de esa expresión. |
| `to` | texto | Nombre nuevo. |
| `remove` | sí/no | `true` quita la expresión (y su peso en `shapes` de la capa). |

Una capa admite hasta 64 expresiones. Una expresión se maneja luego como una forma más: se le pone un peso con `animate` (`morph`) o con claves.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena shape_sculpt '{"layer":"malla","shape":"sonrisa","points":[[0.4,0.5],[0.6,0.5]],"mode":"push","by":20,"radius":0.2}'
wprs-ctl scene act mi-escena expression '{"layer":"malla","name":"feliz","shapes":{"sonrisa":1}}'
```

Renombrar y quitar:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena shape_sculpt '{"layer":"malla","shape":"sonrisa","points":[[0.4,0.5],[0.6,0.5]],"mode":"push","by":20,"radius":0.2}'
wprs-ctl scene act mi-escena expression '{"layer":"malla","name":"feliz","shapes":{"sonrisa":1}}'
wprs-ctl scene act mi-escena expression '{"layer":"malla","name":"feliz","to":"alegre"}'
wprs-ctl scene act mi-escena expression '{"layer":"malla","name":"alegre","remove":true}'
```

### Cinemática y física

#### `ik`

Añade, cambia o quita una cadena de cinemática inversa: huesos que se estiran y doblan solos para que el último llegue a un destino. Sirve para que un brazo alcance un punto, un hueso o el puntero. En la app, **Cadenas que alcanzan** y **Crear una cadena en la vista previa** (haces clic en el hueso donde empieza y luego en el hueso donde termina). Cada cadena tiene **Doblar hacia el otro lado**, **Peso**, **Limitar** y **Libre**; en la vista previa arrastras el anillo de un destino que es un punto y los puntos de los límites.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `index` | entero | Lugar de la cadena que se cambia o quita (0 es la primera). Sin él, se añade una cadena nueva. |
| `root` | texto | Hueso donde empieza la cadena. Va junto con `tip`. |
| `tip` | texto | Hueso donde termina: tiene que colgar por debajo de `root`. La cadena tiene de 2 a 8 huesos. |
| `target` | texto o `[x, y]` | Destino: `"cursor"` (el puntero), `"bone:NOMBRE"`, `"layer:ID"` o un punto `[x, y]` en píxeles de la capa, desde su esquina de arriba a la izquierda. Una cadena nueva sin `target` apunta al lugar donde está su final en reposo, y no se mueve. |
| `limits` | objeto | Límites de giro por hueso, `{"brazo": [-90, 90]}`: mínimo y máximo en grados desde el reposo, de -180 a 180. El hueso debe ser de la cadena. |
| `unlimit` | lista de textos | Huesos cuyo límite se quita. |
| `weight` | número | Cuánto manda la cadena, de 0 a 1. Por defecto 1. |
| `bend` | texto | `clockwise` (sentido horario, por defecto) o `counterclockwise` hacia dónde se dobla. |
| `remove` | sí/no | Con `index`: quita la cadena. |

Una capa admite hasta 16 cadenas. Una cadena que sigue al puntero necesita el permiso del puntero, y la escena avisa al usuario de que lo sigue.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"brazo","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena ik '{"layer":"malla","root":"raiz","tip":"mano","target":[128,20]}'
```

Cambiar la cadena: límites, hacia dónde se dobla y peso; luego que siga al puntero, a un hueso, y quitarla:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"brazo","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena ik '{"layer":"malla","root":"raiz","tip":"mano"}'
wprs-ctl scene act mi-escena ik '{"layer":"malla","index":0,"limits":{"brazo":[-90,90]},"bend":"counterclockwise","weight":0.8}'
wprs-ctl scene act mi-escena ik '{"layer":"malla","index":0,"target":"cursor"}'
wprs-ctl scene act mi-escena ik '{"layer":"malla","index":0,"target":"bone:raiz","unlimit":["brazo"]}'
wprs-ctl scene act mi-escena ik '{"layer":"malla","index":0,"remove":true}'
```

Las cadenas y sus destinos los lista la pregunta `handles`.

#### `physics`

Hace que huesos se balanceen con física, o cambia o quita esa física. Sirve para pelo, colas, telas o antenas. En la app, en una malla **Huesos con física**: **Elegir en la vista previa los huesos que se balancean**, **Dibujar una cuerda en la vista previa**, **Simular** y **Restablecer**; en un modelo, **Huesos que se balancean** con modos ya hechos (**Pelo**, **Cola**, **Tela**, **Antena**, **Cadena**, **Gelatina**, que son la variante `preset` de `animate`).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `bones` | lista de textos (obligatorio) | Huesos que se balancean. A cada uno que no tiene física se la añade, y a los demás les cambia los valores que des. Hasta 64 por capa. |
| `mode` | texto | `spring` (muelle, el valor de un hueso nuevo), `rigid` (rígido) o `rope` (cuerda). |
| `stiffness` | número | Cuánto vuelve el hueso a su pose, de 0 a 1. Por defecto 0,5. |
| `damping` | número | Cuánto frena el balanceo, de 0 a 1. Por defecto 0,5. |
| `inertia` | número | Cuánto se queda atrás cuando se mueve su padre, de 0 a 1. Por defecto 0,5. |
| `gravity` | número | Tirón vertical, de -10 a 10. Un valor positivo lo tira hacia abajo. Por defecto 0. |
| `wind` | número | Cuánto le afecta el viento (ver `wind`), de 0 a 1. Por defecto 0. |
| `rotation` | `[mín, máx]` | Acota el giro, en grados dentro de ±180. |
| `translation` | `[mín, máx]` | Acota cuánto se desliza su cabeza, en píxeles. |
| `free` | lista de textos | Cotas que se quitan: `"rotation"` o `"translation"`. |
| `remove` | sí/no | `true` quita la física de esos huesos. |
| `rope` | lista de `[x, y]` | De 2 a 9 puntos, en fracciones de la capa, que dibujan una cuerda. Pide un solo nombre en `bones`: se crean los huesos `NOMBRE_1`, `NOMBRE_2`… a lo largo de los puntos, cada uno colgando del anterior, y el primero se balancea como cuerda. Solo en mallas. |
| `parent` | texto | Con `rope`: hueso del que cuelga el primero de la cuerda. |

Una cuerda se quita nombrando su primer hueso (`NOMBRE_1`).

Los modelos con colisionadores mantienen fuera de ellos los huesos que se balancean (ver `collider`). El balanceo se detiene en el nivel de calidad más bajo, donde los huesos conservan su animación.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"raiz","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena physics '{"layer":"malla","bones":["mano"],"mode":"spring","stiffness":0.4,"gravity":2}'
```

Acotar el giro, soltar la cota y quitar la física:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"raiz","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena physics '{"layer":"malla","bones":["mano"],"mode":"rigid","rotation":[-30,30],"translation":[-5,5],"inertia":0.5,"damping":0.2,"wind":0.3}'
wprs-ctl scene act mi-escena physics '{"layer":"malla","bones":["mano"],"free":["rotation"]}'
wprs-ctl scene act mi-escena physics '{"layer":"malla","bones":["mano"],"remove":true}'
```

Una cuerda colgada de la raíz:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena physics '{"layer":"malla","bones":["cola"],"rope":[[0.5,0.9],[0.4,0.95],[0.3,1.0]],"parent":"raiz"}'
```

#### `collider`

Añade, cambia o quita una cápsula o una esfera que mantiene fuera los huesos que se balancean de un modelo. Sirve para que el pelo no atraviese la cabeza. En la app, **Colisionadores**: **Elige un hueso para añadir un colisionador**, **Cápsula a lo largo del hueso** o **Esfera en su cabeza**, y **Quitar**. En la vista previa arrastras el ancho.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de modelo. |
| `index` | entero | Lugar del colisionador que se cambia o quita (0 es el primero). Sin él se añade uno nuevo. |
| `bone` | texto | Hueso al que va ligado. Obligatorio al añadir. |
| `shape` | texto | `capsule` (a lo largo del hueso) o `sphere` (en su cabeza). |
| `radius` | número | Radio en unidades del modelo, de 0 a 100. Un colisionador nuevo empieza en 0,1. |
| `offset` | `[x, y, z]` | Desplazamiento en el espacio del hueso. Tres ceros lo dejan sin desplazamiento. |
| `remove` | sí/no | Con `index`: quita el colisionador. |

Un modelo admite hasta 8 colisionadores. Los huesos que se balancean quedan fuera de los colisionadores: de todos en los dos niveles de calidad más altos, de los dos primeros en el penúltimo y de ninguno en el más bajo, donde los huesos conservan su animación.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"model","file":"modelo.glb","id":"muneco"}'
wprs-ctl scene act mi-escena collider '{"layer":"muneco","bone":"tip","shape":"capsule","radius":0.2}'
```

Cambiar y quitar:

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"model","file":"modelo.glb","id":"muneco"}'
wprs-ctl scene act mi-escena collider '{"layer":"muneco","bone":"tip","radius":0.2}'
wprs-ctl scene act mi-escena collider '{"layer":"muneco","index":0,"shape":"sphere","offset":[0,0.1,0]}'
wprs-ctl scene act mi-escena collider '{"layer":"muneco","index":0,"remove":true}'
```

#### `bone_rule`

Añade, cambia o quita una regla que pasa un hueso a otro padre a medida que su peso sube de 0 a 1, conservando su pose. Sirve, por ejemplo, para que una mano suelte el cuerpo y siga un objeto. En la app, **Reglas que pasan un hueso a otro padre**: **Hueso**, **Nuevo padre**, **Peso**, **Añadir una regla** y **Quitar**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla. |
| `index` | entero | Lugar de la regla que se cambia o quita (0 es la primera). Sin él se añade una regla nueva. |
| `bone` | texto | Hueso que cambia de padre. Obligatorio al añadir. |
| `parent` | texto | Padre al que pasa. Debe ser otro hueso y no cerrar un lazo. Obligatorio al añadir. |
| `weight` | número | De 0 (cuelga de su padre de siempre) a 1 (cuelga del nuevo). Por defecto 1. |
| `remove` | sí/no | Con `index`: quita la regla. |

Hasta 64 reglas por capa. La acción escribe un peso fijo; en la escena ese peso se puede animar como otros valores (ver [propiedades](propiedades.md)).

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"brazo","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena bone_rule '{"layer":"malla","bone":"mano","parent":"raiz","weight":0.5}'
```

Cambiar el peso y quitar:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena bone_rule '{"layer":"malla","bone":"brazo","parent":"raiz"}'
wprs-ctl scene act mi-escena bone_rule '{"layer":"malla","index":0,"weight":0.2}'
wprs-ctl scene act mi-escena bone_rule '{"layer":"malla","index":0,"remove":true}'
```

#### `wind`

Pone el viento de la escena, que mueve los huesos con física y las partículas. Es uno solo para toda la escena. En la app, **Viento**: **Ángulo**, **Fuerza**, **Ráfagas**, **Mostrar en la vista previa** (una flecha desde el centro del lienzo cuya punta se arrastra) y **Sin viento**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `angle` | número | Hacia dónde sopla, en grados en el sentido de las agujas del reloj desde la derecha: 0 hacia la derecha, 90 hacia abajo. |
| `strength` | número | Fuerza en píxeles por segundo, de 0 a 10000. |
| `gusts` | número | Cuánto varía con ráfagas, de 0 a 1. |
| `off` | sí/no | `true` quita el viento y las ráfagas. Por defecto `false`. |

Hay que dar al menos `angle`, `strength` o `gusts` (o `off`). Lo que no des se queda como estaba. Un viento que seguía claves o un enlace pasa a ser fijo.

```sh
wprs-ctl scene act mi-escena wind '{"angle":0,"strength":300,"gusts":0.5}'
```

Cambiar solo la dirección y quitar el viento:

```sh
wprs-ctl scene act mi-escena wind '{"angle":90}'
wprs-ctl scene act mi-escena wind '{"off":true}'
```

El viento actual lo da la pregunta `wind`.

### Piezas y enganches

#### `attach`

Sujeta una capa a un hueso o a un punto de una capa de malla o de modelo, para que lo siga. Sirve para ponerle un sombrero o un objeto en la mano. En la app, **Sujetar capas en la vista previa**: arrastras una capa sobre un hueso, o sobre la malla para un punto nuevo allí; en **Capas que siguen esta malla** puedes soltarlas, y el anillo de cada una se arrastra para moverla.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | La capa que sigue. Puede ser de cualquier tipo. |
| `to` | texto | Capa de malla o de modelo que se sigue. Si falta, la que ya sigue. Una capa no puede seguirse a sí misma, ni a través de otras. |
| `bone` | texto | Hueso que sigue. No se combina con `point`. |
| `point` | texto o `[x, y]` | Un punto de la capa seguida, por su nombre, o un `[x, y]` en fracciones de la capa para crear un punto nuevo ahí (solo en mallas). El punto nuevo se llama `ID_point` y cuelga del hueso que manda en el vértice más cercano. |
| `offset` | `[x, y]` | Desplazamiento en píxeles, en el espacio del hueso o punto. |
| `nudge` | `[x, y]` | Mueve la capa esos píxeles de la capa de malla, en pantalla; la acción lo convierte en `offset`. |
| `inherit` | lista de textos | Qué hereda del hueso: `position`, `rotation`, `scale`. Si falta, hereda las tres. |
| `detach` | sí/no | `true` suelta la capa. |

Si sigues a la misma capa y no das `bone`, `point`, `offset` o `inherit`, se conservan los que tenía. Un punto marcado como de sujeción que nadie más usa se borra cuando la capa se suelta o sigue otra cosa.

Primero una malla con un hueso y una capa de imagen:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"raiz","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena add_layer '{"kind":"image","source":"foto","id":"gorro"}'
wprs-ctl scene act mi-escena attach '{"layer":"gorro","to":"malla","bone":"mano","offset":[5,5],"inherit":["position","rotation"]}'
```

Sujetar a un punto nuevo, empujarla y soltarla:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena add_layer '{"kind":"image","source":"foto","id":"gorro"}'
wprs-ctl scene act mi-escena attach '{"layer":"gorro","to":"malla","point":[0.5,0.5]}'
wprs-ctl scene act mi-escena attach '{"layer":"gorro","nudge":[10,0]}'
wprs-ctl scene act mi-escena attach '{"layer":"gorro","detach":true}'
```

Sujetar a un punto con nombre (ver `point`):

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena point '{"layer":"malla","name":"anillo","at":[0.5,0.2],"parent":"raiz"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"image","source":"foto","id":"gorro"}'
wprs-ctl scene act mi-escena attach '{"layer":"gorro","to":"malla","point":"anillo"}'
```

#### `point`

Añade, mueve, cambia de hueso, renombra o quita un punto con nombre de una malla, un lugar donde sujetar capas que no es un hueso. En la app, **Puntos de sujeción**: **Añadir un punto**, el nombre se edita en la lista, el menú del hueso del que cuelga (o **Ningún hueso**) y **Quitar el punto**; en la vista previa cada punto es un cuadrado que se arrastra.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla. |
| `name` | texto (obligatorio) | Nombre del punto, igual que un hueso. Si no existe, se crea. |
| `at` | `[x, y]` | Lugar en fracciones de la capa. Obligatorio en un punto nuevo. |
| `parent` | texto | Hueso del que cuelga. Un texto vacío lo deja sin padre. Un punto nuevo sin `parent` cuelga del hueso que manda en esa zona de la malla. |
| `to` | texto | Nombre nuevo. Las capas sujetas a él y las cadenas que lo buscan lo siguen. |
| `remove` | sí/no | `true` quita el punto y las sujeciones y cadenas que lo nombraban. |

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena point '{"layer":"malla","name":"anillo","at":[0.5,0.2],"parent":"raiz"}'
```

Mover, renombrar y quitar:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena point '{"layer":"malla","name":"anillo","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena point '{"layer":"malla","name":"anillo","at":[0.4,0.3],"parent":"raiz"}'
wprs-ctl scene act mi-escena point '{"layer":"malla","name":"anillo","to":"dedo"}'
wprs-ctl scene act mi-escena point '{"layer":"malla","name":"dedo","remove":true}'
```

#### `parts`

Ordena, oculta o muestra las piezas de una malla y activa o ajusta su profundidad. Una malla de un archivo puede tener varias piezas; la que sale de una imagen tiene una, `body`. En la app, **Partes, de atrás hacia delante**: arrastras la manija de una pieza para cambiar su lugar, la casilla **Visible** la oculta o muestra, y el menú **Sobre todo** elige sobre qué pieza solo se ve. La profundidad está en el mismo inspector: **Profundidad: los vértices cercanos tapan a los lejanos**, **Perspectiva de la profundidad** y **Ocultar las caras vistas por detrás**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `order` | objeto | Orden de dibujo por pieza, `{"body": 3}`: de -1000 a 1000, se redondea. Un valor mayor se dibuja delante. |
| `hide` | lista de textos | Piezas que se ocultan. |
| `show` | lista de textos | Piezas que se vuelven a mostrar. Una pieza no puede estar en `hide` y en `show`. |
| `clip` | objeto | `{"pieza": "otra"}`: la pieza solo se ve sobre `otra`. Nunca sobre sí misma, ni a través de otras. |
| `unclip` | lista de textos | Piezas a las que se les quita esa restricción. |
| `depth` | objeto | Profundidad de la malla. Con `scale` (de 0 a 10), `cull` (sí/no: oculta las caras vistas por detrás) y `sort` (sí/no: dibuja las piezas de atrás hacia delante; es lo normal, así que `true` borra el campo), activa la profundidad y cambia esos campos. `{"off": true}` la desactiva y no se combina con los demás. |

Hay que dar al menos algo que cambiar. Las piezas deben existir (las lista `rig`).

`clip` y `unclip` piden una malla con más de una pieza, como la de un archivo `.glb` con varias; una malla hecha desde una imagen solo tiene `body`, y esas dos opciones no tienen ejemplo aquí.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena parts '{"layer":"malla","order":{"body":3}}'
```

Ocultar y mostrar:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena parts '{"layer":"malla","hide":["body"]}'
wprs-ctl scene act mi-escena parts '{"layer":"malla","show":["body"]}'
```

Activar la profundidad, ajustarla y desactivarla:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena parts '{"layer":"malla","depth":{"scale":2,"cull":true}}'
wprs-ctl scene act mi-escena parts '{"layer":"malla","depth":{"sort":false}}'
wprs-ctl scene act mi-escena parts '{"layer":"malla","depth":{"off":true}}'
```

### Reproducir y mezclar animaciones

#### `animate`

Edita cómo se anima una capa de malla o de modelo. Toma una sola variante.

##### variante `layer`

Añade, cambia, reordena o quita una capa de animación: cada una reproduce un clip y se mezclan en orden. En la app, **Capas de animación**: **Añadir una capa que reproduzca…**, y por capa **Encendido**, **Suma**, **Peso**, **Velocidad**, **Entrada gradual**, **Salida gradual** y **Solo estos huesos**. Primero hay que crear un clip.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `add` | texto | Id de un clip: añade una capa de animación nueva que lo reproduce. Los demás campos de la misma llamada (`weight`, `rate`…) se aplican a esa capa nueva. |
| `index` | entero | Sin `add`: lugar de la capa de animación que se cambia (0 es la primera). |
| `clip` | texto | Cambia el clip que reproduce. |
| `remove` | sí/no | `true` quita la capa de animación. |
| `weight` | número | Cuánto manda, de 0 a 1. Por defecto 1. |
| `rate` | número | Velocidad de reproducción, de 0 a 16. Por defecto 1. |
| `additive` | sí/no | `true` suma el clip a lo que hay debajo en vez de reemplazarlo. Por defecto `false`. |
| `blend_in` | número | Segundos de entrada gradual, de 0 a 10. |
| `blend_out` | número | Segundos de salida gradual, de 0 a 10. |
| `enabled` | sí/no | `false` apaga la capa sin quitarla. Por defecto `true`. |
| `bones` | lista de textos | Solo estos huesos reciben el clip. Una lista vacía los vuelve a incluir a todos. |
| `to` | entero | Mueve la capa a ese lugar de la lista. |

Una capa admite hasta 16 capas de animación.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true}'
wprs-ctl scene act mi-escena animate '{"layer":{"layer":"malla","add":"saludo","weight":0.8,"rate":1.5}}'
```

Cambiar la capa que ya existe, mover y quitar:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"otro","add":true}'
wprs-ctl scene act mi-escena animate '{"layer":{"layer":"malla","add":"saludo"}}'
wprs-ctl scene act mi-escena animate '{"layer":{"layer":"malla","add":"otro","additive":true}}'
wprs-ctl scene act mi-escena animate '{"layer":{"layer":"malla","index":1,"bones":["brazo"],"blend_in":0.3,"blend_out":0.3,"enabled":true}}'
wprs-ctl scene act mi-escena animate '{"layer":{"layer":"malla","index":1,"clip":"saludo","to":0}}'
wprs-ctl scene act mi-escena animate '{"layer":{"layer":"malla","index":0,"remove":true}}'
```

##### variante `root`

Enciende o apaga el movimiento de raíz de un modelo: el desplazamiento de su hueso raíz mueve la capa entera. Solo para capas de modelo, y para encenderlo la capa necesita un clip que mueva la raíz.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de modelo. |
| `on` | sí/no (obligatorio) | `true` lo enciende, `false` lo apaga. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"model","file":"modelo.glb","id":"muneco"}'
wprs-ctl scene act mi-escena clip '{"layer":"muneco","id":"paso","add":true,"animation":"walk"}'
wprs-ctl scene act mi-escena animate '{"root":{"layer":"muneco","on":true}}'
```

##### variante `preset`

Pone a un hueso la física de un modo ya hecho. Es lo que hacen los botones **Pelo**, **Cola**, **Tela**, **Antena**, **Cadena** y **Gelatina** de **Huesos que se balancean**. Si el hueso ya tenía física, la reemplaza.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `bone` | texto (obligatorio) | Hueso que se balancea. |
| `preset` | texto (obligatorio) | `hair` (pelo), `tail` (cola), `cloth` (tela), `antenna` (antena), `chain` (cadena) o `jelly` (gelatina). |

| Modo | Tipo | Rigidez | Frenado | Inercia | Gravedad | Viento |
|---|---|---|---|---|---|---|
| `hair` | `rope` | 0,2 | 0,3 | 0,7 | 1,0 | 0,6 |
| `tail` | `rope` | 0,4 | 0,4 | 0,5 | 0,5 | 0,3 |
| `cloth` | `spring` | 0,3 | 0,5 | 0,6 | 2,0 | 0,8 |
| `antenna` | `spring` | 0,7 | 0,2 | 0,4 | 0,0 | 0,2 |
| `chain` | `rigid` | 0,1 | 0,2 | 0,8 | 3,0 | 0,1 |
| `jelly` | `spring` | 0,5 | 0,1 | 0,9 | 0,0 | 0,0 |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"model","file":"modelo.glb","id":"muneco"}'
wprs-ctl scene act mi-escena animate '{"preset":{"layer":"muneco","bone":"tip","preset":"hair"}}'
```

##### variante `morph`

Fija el peso de una forma de mezcla o de una expresión de la capa, sin un clip. En la app, los deslizadores de **Formas de mezcla en la vista previa** y **Expresiones en la vista previa** solo cambian la vista previa; esta acción es la que cambia la escena.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Capa de malla o de modelo. |
| `shape` | texto (obligatorio) | Forma de mezcla o expresión. |
| `weight` | número | Peso, de -1 a 1. Si falta, se quita el peso fijado. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"model","file":"modelo.glb","id":"muneco"}'
wprs-ctl scene act mi-escena animate '{"morph":{"layer":"muneco","shape":"bulge","weight":0.6}}'
wprs-ctl scene act mi-escena animate '{"morph":{"layer":"muneco","shape":"bulge"}}'
```

## Preguntas

### Antes de empezar con las preguntas de mallas y animación

- Casi todas estas preguntas hablan de una capa de malla o de modelo. En el terminal la capa va con la opción `--layer ID` después de los argumentos. `motion` y `script_calls` la toman también dentro del JSON (`"layer"`). Sin capa, o con una que no es de malla ni de modelo, la respuesta es `null`.
- Los lugares `[x, y]` de las respuestas son fracciones de la capa: `[0, 0]` es la esquina de arriba a la izquierda y `[1, 1]` la de abajo a la derecha.
- Los ejemplos parten de una malla hecha desde una imagen, con dos huesos y un clip:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
```

### Qué tiene una capa

#### `rig`

Lista los huesos, puntos, piezas, formas de mezcla y clips de una capa de malla o de modelo. Es la pregunta por defecto de la capa.

Sin argumentos (`'{}'`). La respuesta tiene:

- `bones`: los huesos, cada uno con `name`, `parent` (o `null`) y, en una malla, `at` (el lugar de su cabeza).
- `points`: los demás nodos del archivo: las piezas de la malla (`part: true`) y los puntos de sujeción (`part: false`), con `name`, `at` y `parent`.
- `parts`: nombres de las piezas.
- `shapes` y `defaults`: formas de mezcla y su peso de reposo.
- `animations`: animaciones del archivo `.glb`, con `name` y `length` en segundos.
- `clips`: ids de los clips de la capa.
- `expressions`: las expresiones con los pesos de sus formas.
- `hit`: huesos que reciben el puntero.
- `depth`: ajuste de profundidad, o `null`.
- `auto`: `true` si los pesos de la malla siguen a los huesos.
- `editable`: `true` si se pueden editar los huesos. En un modelo es `false`: sus huesos vienen de su archivo.
- `model`: `true` si la capa es un modelo.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene ask mi-escena rig '{}' --layer malla
```

Con un modelo:

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"model","file":"modelo.glb","id":"muneco"}'
wprs-ctl scene ask mi-escena rig '{}' --layer muneco
```

#### `motion`

Describe los clips y todo el movimiento de una capa de malla o de modelo en una sola respuesta: es lo que muestra el inspector de la capa.

Argumento: `{"layer": "ID"}`, o `--layer`. La respuesta tiene:

- `layer`, `pointer` (dónde está la capa en el archivo de la escena, p. ej. `/scene/layers/0`) y `model`.
- `clips`: cada clip con `id`, `animation`, `from`, `to`, `length`, `loop`, `events`, `bones` y `shapes` (cuántos canales tiene) y `path`, el recorrido del hueso raíz en un modelo.
- `files`: animaciones del archivo con `name` y `length`.
- `layers`: las capas de animación, con `index`, `clip`, `weight`, `rate`, `additive`, `blend_in`, `blend_out`, `enabled` y `bones`.
- `bones`, `shapes` y `defaults`, y `weights` (pesos fijados con `animate morph`).
- `physics`, `colliders`, `ik` y `root_motion`: lo mismo que escriben las acciones del mismo nombre.
- `presets`: los modos de `animate` con la variante `preset`.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2,"play":true}'
wprs-ctl scene ask mi-escena motion '{"layer":"malla"}'
```

#### `outline`

Da el borde de cada pieza de una malla, tal como la dibuja el editor.

Sin argumentos (`'{}'`) y con `--layer`. La respuesta es un objeto con un campo por pieza (p. ej. `body`), y cada una es una lista de segmentos `[[x1, y1], [x2, y2]]`: los lados de los triángulos que no comparte ninguno otro.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla","cells":4}'
wprs-ctl scene ask mi-escena outline '{}' --layer malla
```

### Claves y pose

#### `keys`

Da las claves de un clip, fila por fila, en segundos desde el inicio del clip.

Argumento: el id del clip, como un texto JSON (`'"saludo"'`), y `--layer`. La respuesta tiene:

- `clip`, `length` (segundos), `loop` y `model`.
- `rows`: una por canal con claves: `kind` (`bone`, `shape` u `order`), `name` (el hueso, la forma o la pieza), `channel` (`rotation`, `position`, `scale`, `angles` o `shift` en huesos; vacío en formas y órdenes) y `keys`, cada una con `t` (segundo), `value` y `ease` (la curva hasta la clave siguiente, `"linear"` si no hay otra).
- `events`: los eventos del clip.
- `shapes`, `defaults` y `parts`: lo que la capa ofrece para animar.

Si la capa o el clip no existen, la respuesta es `null`.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
wprs-ctl scene ask mi-escena keys '"saludo"' --layer malla
```

#### `pose`

Da la pose de una malla en un segundo de un clip: dónde queda cada hueso. Se puede probar una clave sin guardarla.

Argumento: un objeto, y `--layer`.

| Campo | Tipo | Qué significa |
|---|---|---|
| `clip` | texto (obligatorio) | Id del clip. |
| `time` | número | Segundo, desde el inicio del clip. Por defecto 0. Se ajusta a la duración según el `loop` del clip. |
| `bone` | texto | Un hueso al que se le pone, solo para esta pregunta, una clave en ese segundo. |
| `rotation` | número | Giro de esa clave, en grados. |
| `position` | `[x, y]` | Desplazamiento de esa clave, en píxeles de la capa. |
| `scale` | `[x, y]` | Factor de tamaño de esa clave. |

La respuesta tiene `time` y `length` (segundos), `size` (el tamaño de la capa en píxeles) y `bones`: cada hueso con `at` (su cabeza), `angle` (hacia dónde apunta, en grados), `rotation`, `position` y `scale` (sus canales en ese segundo) y `frame`, el espacio de su padre (cuatro números).

La pregunta es para mallas; en un modelo no da una pose utilizable.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene act mi-escena key '{"layer":"malla","clip":"saludo","time":1,"bone":"brazo","rotation":40}'
wprs-ctl scene ask mi-escena pose '{"clip":"saludo","time":1}' --layer malla
```

Probar una clave sin guardarla:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena clip '{"layer":"malla","id":"saludo","add":true,"length":2}'
wprs-ctl scene ask mi-escena pose '{"clip":"saludo","time":1,"bone":"brazo","rotation":20,"position":[0,10]}' --layer malla
```

#### `weights`

Da el mapa de pesos de un hueso sobre la malla, como dos imágenes. Es lo que pinta la app en **Pintar los pesos sobre la vista previa**.

Argumento: el nombre del hueso, como un texto JSON (`'"brazo"'`), y `--layer`. La respuesta tiene:

- `bone`: el hueso pedido.
- `heat`: una imagen PNG, como dirección `data:image/png;base64,…`, con el peso del hueso en cada triángulo: azul sin peso, rojo todo el peso.
- `owner`: otra imagen PNG con el color del hueso que más manda en cada triángulo.
- `bones` y `colors`: los huesos y el color de cada uno en `owner`.
- `size`: tamaño de las imágenes en píxeles (hasta 384 por lado).
- `auto`: `true` si los pesos son automáticos.

Un nombre de hueso vacío da `heat` sin peso.

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene ask mi-escena weights '"brazo"' --layer malla
```

#### `handles`

Da los lugares que la app dibuja para editar cadenas de cinemática inversa y cuerpos con física: dónde están sus huesos, sus finales y sus destinos.

Sin argumentos (`'{}'`) y con `--layer`. La respuesta tiene `size` (tamaño de la capa en píxeles) y dos listas:

- `ik`: una por cadena, con `chain` (los huesos), `at` (la cabeza de cada uno), `end` (el final del último), `target` (el destino, si es un punto o un hueso; `null` si es el puntero, otra capa o un enlace), `follows` (`point`, `bone`, `layer`, `cursor` o `bound`), `bend` y `limits` (por hueso: `bone`, `at`, `rest` el ángulo de reposo en grados, `min` y `max`).
- `bodies`: uno por hueso con física, con `bone`, `mode`, `gravity`, `at` (su cabeza), `tip` (su punta), `rest` (el ángulo de reposo en grados), `rotation` y `translation` (las cotas, o `null`).

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"brazo","parent":"raiz","at":[0.5,0.5]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"brazo","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena ik '{"layer":"malla","root":"raiz","tip":"mano","target":[128,20]}'
wprs-ctl scene act mi-escena physics '{"layer":"malla","bones":["mano"],"gravity":1}'
wprs-ctl scene ask mi-escena handles '{}' --layer malla
```

### Scripts, viento y puntero

#### `script_calls`

Da, para una capa de malla o de modelo, los nombres que un script puede mover y la línea de código que lo hace.

Argumento: `{"layer": "ID"}` y, para pedir el código de una llamada, también `kind`, `name` y `what`. Sin ellos, la respuesta lista:

- `bones`: huesos y puntos de la capa.
- `shapes`: formas de mezcla y expresiones.
- `bodies`: huesos con física.
- `calls`: las llamadas que existen por clase: `bone` (`turn`, `move`, `scale`, `read`, `reset`), `shape` (`set`, `read`) e `impulse` (`push`).
- `uses`: lo que el script debe declarar para usarlas (`animations`).

Con `kind` (`bone`, `shape` o `impulse`), `name` (un nombre de esa clase que la capa tiene; para `impulse`, un hueso con física) y `what` (una de las llamadas de arriba), la respuesta trae `code`, una línea de script lista para pegar. Un nombre que la capa no tiene, o una llamada que no existe, da un error.

Para qué sirven estas llamadas en un script, mira [scripts](scripts.md).

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"raiz","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena physics '{"layer":"malla","bones":["mano"]}'
wprs-ctl scene ask mi-escena script_calls '{"layer":"malla"}'
```

Pedir el código para girar un hueso y para dar un empujón a uno con física:

```sh
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena mesh_from_picture '{"image":"foto","id":"malla"}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"raiz","at":[0.5,0.9]}'
wprs-ctl scene act mi-escena bone '{"layer":"malla","name":"mano","parent":"raiz","at":[0.5,0.2]}'
wprs-ctl scene act mi-escena physics '{"layer":"malla","bones":["mano"]}'
wprs-ctl scene ask mi-escena script_calls '{"layer":"malla","kind":"bone","name":"mano","what":"turn"}'
wprs-ctl scene ask mi-escena script_calls '{"layer":"malla","kind":"impulse","name":"mano","what":"push"}'
```

#### `wind`

Da el viento de la escena como una flecha.

Sin argumentos (`'{}'`). La respuesta tiene `angle` (grados en el sentido de las agujas del reloj desde la derecha), `strength` (píxeles por segundo), `gusts` (de 0 a 1) y `fixed`: `true` si el viento es un valor fijo y `false` si sigue claves o un enlace.

```sh
wprs-ctl scene act mi-escena wind '{"angle":90,"strength":200,"gusts":0.2}'
wprs-ctl scene ask mi-escena wind '{}'
```

#### `wander`

Da dónde está en un segundo el puntero inventado que recorre la vista previa cuando pulsas **Probar el puntero** en un enlace al puntero. Sirve para saber, sin vista previa, por dónde pasará.

Argumento: `{"time": SEGUNDOS}` (por defecto 0). La respuesta tiene `x` e `y`, el lugar del puntero en fracciones del lienzo, y `buttons`, que siempre es 0.

```sh
wprs-ctl scene ask mi-escena wander '{"time":1.5}'
```
