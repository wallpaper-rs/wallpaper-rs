# Acciones: escena 3D

La acción de la escena 3D, con todas sus variantes, y las preguntas sobre modelos, materiales, cámara y sombras. Cómo llamarlas, cómo leer su salida y sus errores está en [acciones](acciones.md).

## Acciones

### Escena 3D: cómo se usa

#### `scene3d`

Edita todo lo que tiene que ver con modelos 3D: añadir y mover modelos, materiales, cámara, sombras, recorridos de cámara y arrastres de asas. Cada llamada lleva una sola variante, que es la clave de primer nivel del JSON (`model`, `material`, `camera`...). Las variantes se explican por temas en las secciones siguientes. El editor de la aplicación hace lo mismo con el panel **Modelo 3D**, el panel **Materiales**, el panel **Cámara 3D**, **Recorridos de cámara**, **Luces en 3D**, **Sombras** y **Mapeo de tonos**, y con las asas que dibuja sobre la vista de edición.

Antes de empezar, cuatro ideas comunes:

- Un modelo es un archivo `.glb` que primero se importa con la acción `import`. Su id es el nombre del archivo sin extensión (`figura`, `modelo`). Cada vez que lo pones en la escena se crea una capa de modelo, y el id de esa capa lo eliges tú con `id`. Si no lo das, se inventa uno a partir del nombre del modelo, así que en los ejemplos siempre se da.
- Las posiciones 3D `[x, y, z]` están en las unidades del modelo: x hacia la derecha, y hacia arriba y z hacia quien mira. La cámara empieza en `[0, 0, 5]` mirando al centro `[0, 0, 0]`. Los giros son grados y se aplican primero en x, luego en y y luego en z.
- Las posiciones `[x, y]` de píxeles (trazos, puntos de la vista, arrastres) son píxeles del lienzo, con el origen arriba a la izquierda. Con el lienzo normal, el centro es `[960, 540]`. La consulta [`stage3d`](#stage3d) dice dónde está cada asa y cada modelo en píxeles.
- Un modelo con huesos tiene que tener al menos una animación puesta (acción `clip`) para que sus huesos respondan a las consultas de hueso.

Los ejemplos usan `figura.glb` (un cubo sencillo con el material `paint`) y `modelo.glb` (un muñeco con huesos `root` y `tip`, material `body` y animaciones `bend`, `stroll` y `walk`).

### Escena 3D: modelos y grupos

#### Variante model: añadir, cambiar o quitar un modelo

Tres formas, cada una dentro de `model`.

##### model → add

Pone un modelo ya importado en la escena como capa nueva. En la aplicación es el botón de **Modelo 3D desde un archivo** del panel **Modelo 3D**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `asset` | texto (obligatorio) | Id del modelo importado. |
| `id` | texto | Id de la capa nueva. Por defecto se inventa uno. |
| `into` | texto | Id de un grupo donde meter la capa. Por defecto va en la raíz. |
| `at` | lista de 3 números | Posición `[x, y, z]` inicial en unidades del modelo. Por defecto `[0, 0, 0]`. |

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo","at":[1,0,0]}}}'
```

##### model → set

Cambia la posición, el giro, la escala o el entorno de una capa de modelo. Si la posición, el giro o la escala ya tienen fotogramas clave o están ligados a una propiedad, esa acción se niega: usa `keyframe` o cambia el enlace (o arrastra con [`grab`](#variante-grab-arrastrar-un-asa)).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la capa de modelo. |
| `position` | lista de 3 números | Posición, de −1 000 000 a 1 000 000. |
| `rotation` | lista de 3 números | Giro en grados en x, y y z, de −3600 a 3600. |
| `scale` | lista de 3 números | Escala en x, y y z. Ninguna puede ser 0. |
| `size` | lista de 2 números | Tamaño en píxeles de la caja donde se dibuja un modelo suelto (ancho y alto, de 1 a 100 000). |
| `env` | texto | Id de una imagen que los materiales reflejan (un panorama). Una cadena vacía `""` la quita. |
| `asset` | texto | Id de otro modelo importado para cambiar el modelo de la capa. |

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"model":{"set":{"layer":"cubo","rotation":[0,45,0],"scale":[2,2,2]}}}'
wprs-ctl scene act mi-escena scene3d '{"model":{"set":{"layer":"cubo","env":"foto"}}}'
wprs-ctl scene act mi-escena scene3d '{"model":{"set":{"layer":"cubo","size":[800,600]}}}'
```

##### model → remove

Quita la capa de modelo. El modelo importado se queda en la escena.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Id de la capa de modelo. |

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"model":{"remove":{"layer":"cubo"}}}'
```

#### Variante group: crear un grupo 3D

Crea un grupo 3D. Los modelos, las luces y las capas planas de un grupo 3D se dibujan juntos, con una misma profundidad. Sin `layers` el grupo nace vacío. Con `layers` reúne esas capas dentro del grupo nuevo.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `id` | texto | Id del grupo. Por defecto `world` (con un número si ya existe). |
| `into` | texto | Id de un grupo donde meter el grupo nuevo (solo si nace vacío). |
| `layers` | lista de textos | Ids de las capas que se reúnen en el grupo. Por defecto ninguna. |

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"group":{"id":"mundo","layers":["cubo"]}}'
wprs-ctl scene act mi-escena scene3d '{"group":{"id":"otro"}}'
```

#### Variante space: pasar un grupo a 3D y volver

Convierte un grupo en grupo 3D (`deep: true`) o lo devuelve a grupo plano (`false`). Solo vale para grupos.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Id del grupo. |
| `deep` | sí/no (obligatorio) | `true` lo hace 3D, `false` lo hace plano. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"group","id":"caja"}'
wprs-ctl scene act mi-escena scene3d '{"space":{"layer":"caja","deep":true}}'
wprs-ctl scene act mi-escena scene3d '{"space":{"layer":"caja","deep":false}}'
```

#### Variante depth_test: que una capa plana se oculte tras los modelos

Hace que una capa plana que está dentro de un grupo 3D quede oculta cuando un modelo más cercano la tapa. En la aplicación es **Ocultarse tras los modelos más cercanos**. No vale para modelos, luces ni sonidos, y encenderlo exige que la capa esté dentro de un grupo 3D.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Id de la capa plana. |
| `on` | sí/no (obligatorio) | `true` activa la prueba de profundidad, `false` la quita. |

```sh
wprs-ctl scene act mi-escena scene3d '{"group":{"id":"mundo"}}'
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo","into":"mundo"}'
wprs-ctl scene act mi-escena scene3d '{"depth_test":{"layer":"fondo","on":true}}'
```

### Escena 3D: materiales

Un modelo trae sus materiales (cada uno con un nombre, que sale en [`model_info`](#model_info)). Estas variantes cambian cómo se ven los de una capa sin tocar el archivo.

#### Variante material: cambiar un material

Cambia el material `name` de la capa `layer`. Solo se guarda lo que pones. Se corresponde con el panel **Materiales** de la aplicación. Con `clear: true` se borran todos los cambios y vuelve el material del archivo.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio en la práctica) | Id de la capa de modelo. |
| `name` | texto (obligatorio en la práctica) | Nombre del material tal como está en el archivo del modelo. |
| `shader` | texto | `pbr` (físico, el normal), `unlit` (sin luz), `fur` (pelo), `foliage` (vegetación), `chroma` (croma) o el id de un shader de material del paquete (ver `new_material`). |
| `lit` | sí/no | `false` lo dibuja sin luz. |
| `double_sided` | sí/no | Dibuja las dos caras de cada triángulo. |
| `cast_shadows` | sí/no | Si da sombra (por defecto sí). |
| `receive_shadows` | sí/no | Si recibe sombra (por defecto sí). |
| `color` | texto o lista | Tinte, como `"#rrggbb"` o `[r, g, b, a]` de 0 a 1. `null` lo quita. |
| `emissive` | texto o lista | Brillo propio, mismo formato que `color`. `null` lo quita. |
| `params` | objeto de números | Números del shader. `pbr`: `metallic`, `roughness`, `occlusion` (0 a 1) y `normal_scale` (0 a 10). `fur`: `layers` (1 a 32), `length` (0 a 100), `density` (0 a 1), `comb_x`, `comb_y`, `comb_z` (−1 a 1). `foliage`: `strength` y `speed` (0 a 10). `chroma`: `threshold` y `softness` (0 a 1). |
| `values` | objeto | Parámetros de un shader de material del paquete, comprobados con su descriptor. Las imágenes se dan como id de una imagen de la escena o como `white`, `black`, `noise` o `normal_flat`. `null` quita un valor. |
| `key` | texto o lista | Color que el material `chroma` hace transparente. Solo vale con `chroma`. |
| `maps` | objeto | Por ranura (`base`, `orm`, `normal`, `occlusion`, `emissive`) el id de una imagen de la escena. `null` devuelve el mapa del archivo. |
| `files` | objeto | Por ranura, un archivo de imagen de la carpeta de la escena que se importa como imagen nueva y se elige en el mismo paso. Las ranuras `orm`, `normal` y `occlusion` se leen como datos, no como colores. |
| `clear` | sí/no | `true` borra todos los cambios del material. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena import '{"file":"foto.png"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"cubo","name":"paint","color":"#ff8040","params":{"metallic":0.8,"roughness":0.3},"double_sided":true}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"cubo","name":"paint","maps":{"base":"foto"}}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"cubo","name":"paint","files":{"emissive":"foto.png"}}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"cubo","name":"paint","maps":{"base":null}}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"cubo","name":"paint","clear":true}}'
```

Con pelo, vegetación o croma:

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"muneco","name":"body","shader":"fur","params":{"layers":12,"length":3,"density":0.7}}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"muneco","name":"body","shader":"chroma","key":"#00ff00","params":{"threshold":0.3,"softness":0.1}}}'
```

#### Variante new_material: crear un shader de material nuevo

Escribe en la escena un shader de material a partir de una plantilla: los archivos `effects/NOMBRE.json` (la descripción) y `effects/NOMBRE.wgsl` (el código), y lo añade como efecto. Después se usa con `material` poniendo su nombre en `shader`. En la aplicación es **Shader de material nuevo**. La plantilla trae tres parámetros: `tint` (color), `glow` (número de 0 a 4) y `picture` (imagen).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `name` | texto (obligatorio) | Nombre del shader: de 1 a 32 letras minúsculas, cifras o `_`, y que no exista ya. |

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"new_material":{"name":"brillo"}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"cubo","name":"paint","shader":"brillo","values":{"tint":"#ffcc00","glow":2,"picture":"noise"}}}'
```

La consulta [`material_check`](#material_check) dice si el código compila.

#### Variante paint: pintar un mapa del material

Pinta un trazo directamente en un mapa del material (por ejemplo el color base) y lo guarda como imagen nueva en `maps/`. Si el material ya tiene una imagen elegida en esa ranura, pinta encima; si no, parte del mapa del archivo y, si tampoco hay, de una imagen blanca de 512 píxeles (o una normal plana para `normal`). En la aplicación es **Pintar este mapa…** en el selector de un mapa.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Id de la capa de modelo. |
| `name` | texto (obligatorio) | Nombre del material. |
| `slot` | texto (obligatorio) | `base`, `orm`, `normal`, `occlusion` o `emissive`. |
| `color` | lista de 3 números (obligatorio) | Rojo, verde y azul de 0 a 1. |
| `radius` | número (obligatorio) | Radio del pincel como parte del ancho de la imagen (0.1 es un décimo). |
| `hardness` | número | Dureza del borde, de 0 (suave) a 1. Por defecto 0.5. |
| `points` | lista de pares (obligatorio) | De 1 a 4096 puntos `[u, v]` de 0 a 1 sobre la imagen, con el origen arriba a la izquierda. |

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene act mi-escena scene3d '{"paint":{"layer":"muneco","name":"body","slot":"base","color":[1,0,0],"radius":0.1,"points":[[0.2,0.2],[0.5,0.5]]}}'
```

#### Variante comb: peinar el pelo con el pincel

Peina el pelo de un material `fur`. El trazo se da en píxeles del lienzo sobre el modelo tal como se ve en la vista de edición; cada tramo entre dos puntos pinta la dirección del arrastre sobre la superficie en un mapa de peinado (`comb`) que se guarda en `maps/`. Los puntos de un tramo tienen que caer sobre ese material y estar cerca entre sí (tramos largos que saltan de una parte a otra de la textura se ignoran). Si ningún tramo toca el material, falla. En la aplicación es **Peinar sobre el modelo**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Id de la capa de modelo. |
| `name` | texto (obligatorio) | Nombre del material, que debe tener `shader: fur`. |
| `points` | lista de pares (obligatorio) | De 2 a 4096 puntos del lienzo en píxeles. |
| `radius` | número (obligatorio) | Radio del pincel en píxeles. |
| `strength` | número | Fuerza de 0 a 1. Por defecto 1. |
| `hardness` | número | Dureza del borde. Por defecto 0.5. |
| `erase` | sí/no | `true` deshace el peinado por donde pasa. Por defecto `false`. |
| `time` | número | Segundo de la escena en que se mira el modelo. Por defecto 0. |

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"muneco","name":"body","shader":"fur"}}'
wprs-ctl scene act mi-escena scene3d '{"comb":{"layer":"muneco","name":"body","points":[[955,480],[960,520],[965,560]],"radius":30}}'
wprs-ctl scene act mi-escena scene3d '{"comb":{"layer":"muneco","name":"body","points":[[955,480],[960,520],[965,560]],"radius":30,"erase":true}}'
```

#### Variante key: tomar el color clave del modelo

Pone como color clave de un material `chroma` el color de su superficie bajo un punto de la vista (el color del mapa, del archivo y del tinte, antes de quitar el color). En la aplicación es **Tomar el color clave del modelo**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Id de la capa de modelo. |
| `name` | texto (obligatorio) | Nombre del material, que debe ser `chroma`. |
| `at` | lista de 2 números (obligatorio) | Punto del lienzo en píxeles. Debe caer sobre ese material. |
| `time` | número | Segundo de la escena. Por defecto 0. |

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene act mi-escena scene3d '{"material":{"layer":"muneco","name":"body","shader":"chroma"}}'
wprs-ctl scene act mi-escena scene3d '{"key":{"layer":"muneco","name":"body","at":[960,540]}}'
```

#### Variante debug: ver máscaras y sombras en la vista

Cambia lo que la vista de edición dibuja en lugar de la luz: `mask` (la máscara de balanceo de los colores de vértice), `matte` (el mate, el alfa tras quitar el color clave) o `shadows` (dónde caen las sombras). Solo lo ve quien edita: el fondo de pantalla terminado nunca lo dibuja. Sin `show` vuelve la luz normal. En la aplicación es **Vista de depuración**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `show` | texto | `mask`, `matte` o `shadows`. Sin él se apaga. |
| `layer` | texto | Con `material`, limita la vista a ese material de esa capa. |
| `material` | texto | Nombre del material. Se da junto con `layer` o no se da. |

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene act mi-escena scene3d '{"debug":{"show":"shadows"}}'
wprs-ctl scene act mi-escena scene3d '{"debug":{"show":"matte","layer":"muneco","material":"body"}}'
wprs-ctl scene act mi-escena scene3d '{"debug":{}}'
```

#### Variante sculpt: esculpir el modelo

Esculpe la superficie con un pincel y guarda el resultado como una forma (una forma de mezcla) con nombre, en un archivo `.wpsh` junto al modelo. La forma queda puesta en la capa con peso 1. Repetir el nombre sigue esculpiendo esa forma. Solo se tocan las partes que miran a la cámara. En la aplicación es el pincel de esculpir de las herramientas de huesos del modelo, con los modos **Empujar**, **Hundir** y **Suavizar**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Id de la capa de modelo. |
| `shape` | texto (obligatorio) | Nombre de la forma: de 1 a 64 letras, cifras, `_` o `-`. No puede coincidir con una forma que ya traiga el archivo. Un modelo admite hasta 64. |
| `mode` | texto (obligatorio) | `push` (sacar hacia fuera), `pull` (meter hacia dentro) o `smooth` (suavizar). |
| `points` | lista de pares (obligatorio) | De 1 a 256 puntos del lienzo en píxeles. |
| `radius` | número (obligatorio) | Radio del pincel en píxeles, de 1 a 4096. |
| `strength` | número (obligatorio) | Fuerza, mayor que 0 y hasta 10 (hasta 1 con `smooth`). |
| `softness` | número | Suavidad del borde de 0 a 1. Por defecto 0.5. |
| `time` | número | Segundo de la escena en que se mira el modelo. Por defecto 0. |

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"sculpt":{"layer":"cubo","shape":"panza","mode":"push","points":[[960,540]],"radius":100,"strength":1}}'
wprs-ctl scene act mi-escena scene3d '{"sculpt":{"layer":"cubo","shape":"panza","mode":"smooth","points":[[940,540],[980,540]],"radius":60,"strength":0.5,"softness":0.8}}'
```

### Escena 3D: cámara y vista

Hay dos cosas distintas. La **cámara** es la de la escena: la que verá quien tenga el fondo de pantalla. La **vista** es solo la forma de mirar mientras editas: girar, acercar o desplazar la vista no cambia la cámara de la escena, y nunca se ve en el fondo terminado.

#### Variante camera: la cámara de la escena

Cambia la cámara 3D de la escena. Solo se tocan los campos que das. En la aplicación es el panel **Cámara 3D**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `fov` | número | Campo de visión en grados, de 1 a 179 (por defecto 60). |
| `near` | número | Distancia más cercana que se dibuja, de 0.001 a 1000 (por defecto 0.1). |
| `far` | número | Distancia más lejana, mayor que `near` y hasta 1 000 000 (por defecto 1000). |
| `eye` | lista de 3 números | Dónde está la cámara. Por defecto `[0, 0, 5]`. |
| `center` | lista de 3 números | Punto al que mira. Por defecto `[0, 0, 0]`. |
| `up` | lista de 3 números | Qué dirección es «arriba»; no puede ser `[0, 0, 0]`. Por defecto `[0, 1, 0]`. |
| `roll` | número | Giro de la cámara sobre su línea de visión, de −180 a 180 grados, contado desde la vertical. Cambia `up`. |
| `from_view` | sí/no | `true` pone la cámara donde mira ahora la vista de edición. Por defecto `false`. En la aplicación es **Poner la cámara desde la vista**. |
| `clear` | sí/no | `true` borra la cámara y vuelve a la de por defecto. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena scene3d '{"camera":{"fov":45,"eye":[0,2,8],"center":[0,1,0]}}'
wprs-ctl scene act mi-escena scene3d '{"camera":{"roll":10,"near":0.5,"far":200}}'
wprs-ctl scene act mi-escena scene3d '{"view":{"orbit":[30,10]}}'
wprs-ctl scene act mi-escena scene3d '{"camera":{"from_view":true}}'
wprs-ctl scene act mi-escena scene3d '{"camera":{"clear":true}}'
```

#### Variante view: la vista de edición

Gira, desplaza, acerca o encuadra la vista con la que miras la escena mientras editas. Los valores son cambios: se suman a lo que ya hay. La primera vez que giras o mueves la vista deja de ser la cámara de la escena y aparecen las asas de la cámara.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `orbit` | lista de 2 números | Giro alrededor del punto mirado: `[horizontal, vertical]` en grados. La inclinación vertical se limita a 89 grados. |
| `pan` | lista de 2 números | Desplazamiento `[x, y]` en píxeles del lienzo. |
| `zoom` | número | Factor mayor que 0: 2 acerca el doble, 0.5 aleja a la mitad. |
| `frame` | texto | Id de una capa de modelo visible: la vista la encuadra entera. En la aplicación es **Mirarlo en la vista de edición**. |
| `through` | sí/no | `true` mira por la cámara de la escena (**Mirar por la cámara**), `false` usa la vista libre (**Volver a la vista de la cámara** hace lo contrario). |
| `reset` | sí/no | `true` borra la vista y vuelve a la cámara. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"view":{"orbit":[30,10],"pan":[50,0],"zoom":1.5}}'
wprs-ctl scene act mi-escena scene3d '{"view":{"frame":"cubo"}}'
wprs-ctl scene act mi-escena scene3d '{"view":{"through":true}}'
wprs-ctl scene act mi-escena scene3d '{"view":{"reset":true}}'
```

#### Variante tones: color brillante y mapa de tonos

Activa el color de alto rango (HDR), que conserva colores más brillantes que el blanco, y elige cómo se doblan para mostrarlos. En la aplicación es el panel **Mapeo de tonos**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `hdr` | sí/no | `true` conserva los colores más brillantes que el blanco. `false` apaga el HDR y también el mapa de tonos. |
| `tonemap` | texto | `none` (ninguno), `aces` (fílmico) o `agx` (suave). Solo con HDR encendido. |
| `split` | número | De 0 a 1: muestra en la vista de edición, a la izquierda de ese punto, la imagen sin mapa de tonos y a la derecha con él (**Antes y después**). 0 lo quita. |

```sh
wprs-ctl scene act mi-escena scene3d '{"tones":{"hdr":true,"tonemap":"aces"}}'
wprs-ctl scene act mi-escena scene3d '{"tones":{"split":0.5}}'
wprs-ctl scene act mi-escena scene3d '{"tones":{"hdr":false}}'
```

### Escena 3D: luces

#### Variante light: poner una luz en 3D

Una capa de luz normal alumbra en el plano. Esta variante la sitúa entre los modelos, en un punto 3D, y le da un punto al que apunta (los focos y las luces direccionales). En la aplicación es el interruptor de cada luz del panel **Luces en 3D**; sus puntos también se arrastran con [`grab`](#variante-grab-arrastrar-un-asa).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la capa de luz. |
| `place` | sí/no | `true` la pone en 3D justo donde se ve ahora (y, salvo en las luces de punto, apuntando a donde alumbra ahora). Por defecto `false`. |
| `position` | lista de 3 números | Posición exacta, hasta un millón de unidades. |
| `aim` | lista de 3 números | Punto al que apunta, hasta un millón de unidades. |
| `flat` | sí/no | `true` quita la posición y el punto: vuelve al plano. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena add_layer '{"kind":"light","id":"luz"}'
wprs-ctl scene act mi-escena scene3d '{"light":{"layer":"luz","place":true}}'
wprs-ctl scene act mi-escena scene3d '{"light":{"layer":"luz","position":[2,3,4],"aim":[0,0,0]}}'
wprs-ctl scene act mi-escena scene3d '{"light":{"layer":"luz","flat":true}}'
```

### Escena 3D: sombras

#### Variante shadows: sombras de una luz

Hace que una luz de punto, un foco o una luz direccional dé sombra a los modelos. En la aplicación es el panel **Sombras**. En una escena como mucho 2 luces dan sombra, y los equipos más modestos dibujan menos (la consulta [`shadows3d`](#shadows3d) dice cuántas por nivel).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la capa de luz. |
| `on` | sí/no | `true` enciende la sombra. `false` la apaga y borra su tamaño y su sesgo. |
| `resolution` | entero | Lado del mapa de sombra en píxeles, de 256 a 4096. |
| `automatic` | sí/no | `true` borra el tamaño elegido y deja que el programa lo decida (**Automático**). Por defecto `false`. |
| `bias` | número | Sesgo de 0 a 1 (**Sesgo**). |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"light","id":"luz"}'
wprs-ctl scene act mi-escena scene3d '{"shadows":{"layer":"luz","on":true,"resolution":1024,"bias":0.1}}'
wprs-ctl scene act mi-escena scene3d '{"shadows":{"layer":"luz","automatic":true}}'
wprs-ctl scene act mi-escena scene3d '{"shadows":{"layer":"luz","on":false}}'
```

### Escena 3D: recorridos de cámara

Un recorrido es un camino que sigue la cámara: una lista de posturas (llamadas claves) con su momento en segundos, y la cámara pasa de una a otra suavemente. Una escena tiene como mucho 16 recorridos y cada uno hasta 256 claves, y siempre al menos 2. Los ids son de 1 a 32 letras minúsculas, cifras o `_`. En la aplicación es el panel **Recorridos de cámara**.

#### Variante path: crear, editar y ordenar recorridos

Seis formas, cada una dentro de `path`.

##### path → add

Crea un recorrido nuevo con dos claves, en el segundo 0 y en el segundo 2, las dos donde mira ahora la vista de edición. Es **Añadir desde la vista**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `id` | texto (obligatorio) | Id del recorrido nuevo. |

```sh
wprs-ctl scene act mi-escena scene3d '{"path":{"add":{"id":"vuelo"}}}'
```

##### path → key

Crea, cambia, mueve o quita una clave.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `path` | texto (obligatorio en la práctica) | Id del recorrido. |
| `time` | número | Segundo de la clave, de 0 a 3600. Por defecto 0. |
| `eye` | lista de 3 números | Dónde está la cámara en esa clave. |
| `center` | lista de 3 números | Punto al que mira. |
| `up` | lista de 3 números | Dirección de «arriba». |
| `fov` | número | Campo de visión, de 1 a 179. |
| `from_view` | sí/no | `true` toma la postura de la vista de edición (la clave nueva siempre la toma si no das nada). Por defecto `false`. |
| `from` | número | Mueve la clave que está en este segundo al segundo `time`. |
| `remove` | sí/no | `true` quita la clave que está en `time`. El recorrido no baja de 2 claves. Por defecto `false`. |

Si ya hay una clave en `time`, se cambian solo los campos que des.

```sh
wprs-ctl scene act mi-escena scene3d '{"path":{"add":{"id":"vuelo"}}}'
wprs-ctl scene act mi-escena scene3d '{"path":{"key":{"path":"vuelo","time":1,"eye":[3,1,4],"center":[0,0,0],"fov":50}}}'
wprs-ctl scene act mi-escena scene3d '{"path":{"key":{"path":"vuelo","time":3,"from":1}}}'
wprs-ctl scene act mi-escena scene3d '{"path":{"key":{"path":"vuelo","time":3,"remove":true}}}'
```

##### path → set

Cambia el bucle, la suavidad o el nombre de un recorrido.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `path` | texto (obligatorio) | Id del recorrido. |
| `loop` | sí/no | `true` hace que vuelva al principio y repita (**Bucle**). |
| `ease` | texto | Suavidad del movimiento: `linear`, `step`, `hold`, o `in_`, `out_` o `in_out_` seguido de `sine`, `quad`, `cubic`, `quart`, `quint`, `expo`, `circ`, `back`, `elastic` o `bounce` (por ejemplo `in_out_sine`). |
| `to` | texto | Id nuevo, que no esté en uso. |

```sh
wprs-ctl scene act mi-escena scene3d '{"path":{"add":{"id":"vuelo"}}}'
wprs-ctl scene act mi-escena scene3d '{"path":{"set":{"path":"vuelo","loop":true,"ease":"in_out_sine","to":"paseo"}}}'
```

##### path → remove

Borra un recorrido. Si era el último, desaparece también la lista de recorridos de la cámara.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `path` | texto (obligatorio) | Id del recorrido. |

```sh
wprs-ctl scene act mi-escena scene3d '{"path":{"add":{"id":"vuelo"}}}'
wprs-ctl scene act mi-escena scene3d '{"path":{"remove":{"path":"vuelo"}}}'
```

##### path → order

Elige en qué orden se reproducen varios recorridos: **En orden** o **Al azar**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `order` | texto (obligatorio) | `sequential` (en orden) o `random` (al azar). |

```sh
wprs-ctl scene act mi-escena scene3d '{"path":{"order":{"order":"random"}}}'
```

##### path → fade

Elige cómo se pasa de un recorrido al siguiente: `black` (**Pasando por negro**) o `cross` (**Cruzado**), con su duración.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `to` | texto | `black` o `cross`. |
| `duration` | número | Segundos de 0 a 10. Si no lo das se queda el que había (1 al principio). |
| `clear` | sí/no | `true` borra el fundido (corte seco). Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena scene3d '{"path":{"fade":{"to":"black","duration":2}}}'
wprs-ctl scene act mi-escena scene3d '{"path":{"fade":{"clear":true}}}'
```

### Escena 3D: arrastrar asas

#### Variante grab: arrastrar un asa

Hace lo mismo que arrastrar con el ratón un asa de la vista de edición: una flecha, un anillo o un punto de un modelo, de la cámara, de una clave de un recorrido o de una luz. Se da el punto donde empezó el arrastre y el punto donde acabó (`from` y `to`, en píxeles del lienzo); el programa calcula qué cambia. Las posiciones de las asas salen de la consulta [`stage3d`](#stage3d). Si el campo que se mueve tiene fotogramas clave, el arrastre pone una clave en el segundo `time`; si está ligado a una propiedad, se niega salvo que pongas `replace: true`.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `what` | objeto o texto (obligatorio) | Qué se arrastra. Ver abajo. |
| `handle` | texto (obligatorio) | El asa: `x`, `y`, `z`, `turn_x`, `turn_y`, `turn_z`, `plane`, `depth`, `orbit`, `dolly` o `roll`. |
| `from` | lista de 2 números (obligatorio) | Dónde empezó el arrastre, en píxeles del lienzo. |
| `to` | lista de 2 números (obligatorio) | Dónde acabó. |
| `time` | número | Segundo de la escena en que se hace (para mirar y para poner la clave). Por defecto 0. |
| `replace` | sí/no | `true` sustituye un enlace por el valor nuevo. Por defecto `false`. |

Los valores de `what`:

| `what` | Qué mueve |
|---|---|
| `{"model":{"layer":"ID"}}` | La posición o el giro de una capa de modelo. |
| `"eye"` | La posición de la cámara de la escena. |
| `"center"` | El punto al que mira la cámara de la escena. |
| `{"key":{"path":"ID","key":SEGUNDO}}` | La cámara de la clave de un recorrido que está en ese segundo. Con `"aim":true` mueve el punto al que mira en vez de la cámara. |
| `{"light":{"layer":"ID"}}` | La posición 3D de una luz. Con `"aim":true` mueve el punto al que apunta. La luz tiene que estar alumbrando una vista 3D. |

Los valores de `handle`:

| `handle` | Qué hace |
|---|---|
| `x`, `y`, `z` | Mueve el punto a lo largo de esa dirección (la flecha del asa). |
| `plane` | Lo desliza en el plano de la pantalla. |
| `depth` | Lo acerca o lo aleja de quien mira. |
| `turn_x`, `turn_y`, `turn_z` | Gira el modelo alrededor de esa dirección (los anillos). Solo vale con `model`. |
| `orbit` | Gira la cámara alrededor del otro extremo (de `center` si arrastras `eye`, y al revés). Vale con `eye`, `center` y `key`. |
| `dolly` | Acerca o aleja la cámara sin cambiar adónde mira. Mismos casos que `orbit`. |
| `roll` | Inclina la cámara sobre su línea de visión. Solo vale con `eye` y `center`. |

Una combinación que no tiene sentido (por ejemplo `turn_x` con `eye`) devuelve un error.

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":{"model":{"layer":"cubo"}},"handle":"x","from":[1030,540],"to":[1100,540]}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":{"model":{"layer":"cubo"}},"handle":"turn_y","from":[1023,540],"to":[1050,560]}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":{"model":{"layer":"cubo"}},"handle":"plane","from":[960,540],"to":[1000,500]}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":{"model":{"layer":"cubo"}},"handle":"depth","from":[908,592],"to":[900,600]}}'
```

La cámara se arrastra con la vista libre puesta (si miras por la cámara no se ven sus asas):

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"view":{"orbit":[30,10]}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":"eye","handle":"orbit","from":[960,540],"to":[1000,540]}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":"center","handle":"plane","from":[960,540],"to":[1000,540]}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":"eye","handle":"dolly","from":[960,540],"to":[960,500]}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":"eye","handle":"roll","from":[1000,540],"to":[1000,600]}}'
```

Una clave de un recorrido y una luz:

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"path":{"add":{"id":"vuelo"}}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":{"key":{"path":"vuelo","key":2,"aim":true}},"handle":"plane","from":[960,540],"to":[1000,500]}}'
wprs-ctl scene act mi-escena add_layer '{"kind":"light","id":"luz"}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":{"light":{"layer":"luz"}},"handle":"plane","from":[960,540],"to":[1000,500]}}'
wprs-ctl scene act mi-escena scene3d '{"grab":{"what":{"light":{"layer":"luz","aim":true}},"handle":"x","from":[960,540],"to":[1000,540]}}'
```


## Preguntas

### Escena 3D: modelos y materiales

Estas preguntas leen la escena y no la cambian. Los ejemplos usan `figura.glb` y `modelo.glb`, ya importados. Las posiciones en píxeles son del lienzo (con el origen arriba a la izquierda y el centro en `[960, 540]` en el lienzo normal). Las acciones que las acompañan están en [`scene3d`](#scene3d).

#### `model_info`

Dice qué trae un archivo de modelo: triángulos, materiales, imágenes, huesos, animaciones, tamaño y si cabe en cada nivel de equipo. Es lo que muestra el panel **Modelo 3D** al elegir un modelo.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `asset` | texto | Id de un modelo ya importado en la escena. |
| `file` | texto | En su lugar, ruta de un `.glb` de la carpeta de la escena (no hace falta importarlo). |

La respuesta tiene `triangles`, `vertices`, `size` (ancho, alto y fondo en unidades), `middle` (el centro), `materials` (lista con `name`, `color`, `metallic`, `roughness`, `unlit`, `double_sided` y `maps`, el número de imagen de cada ranura o `null`), `textures` (`size` y `bytes`), `bones`, `clips` (animaciones del archivo con `name` y `length` en segundos), `parts`, `shapes` (formas de mezcla), `tiers` (por nivel `t0` a `t3`: `fits`, `reasons` si no cabe, `triangles` y `texture` máximos y `memory` que gastaría) y `notes`. Con los dos argumentos ausentes responde `null`; si el archivo no se puede leer, `{ "error": ... }`.

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene ask mi-escena model_info '{"asset":"figura"}'
wprs-ctl scene ask mi-escena model_info '{"file":"modelo.glb"}'
```

#### `materials3d`

Lista los materiales de una capa de modelo con todo lo necesario para editarlos: lo que dice el archivo, lo que has cambiado tú y una miniatura. Es el panel **Materiales**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la capa de modelo. |

La respuesta es `null` si la capa no es un modelo. Si no, tiene `layer`, `radius` (radio del modelo), `env` (la imagen de entorno), `shaders` (todos los shaders que se pueden elegir), `packages` (los shaders de material de la escena), `params` (los límites de los números de pelo, vegetación y croma) y `materials`. Cada material tiene `name`, `file` (color, metal, rugosidad, brillo propio, alfa y caras tal como vienen en el archivo), `set` (lo que has cambiado, o `null`), `maps` (por ranura `base`, `orm`, `normal`, `occlusion` y `emissive`: `texture` del archivo y una miniatura `thumb`), `chosen` (las imágenes de la escena elegidas por ranura), `sphere` (una esfera de muestra, imagen PNG) y `pointer` (dónde vive en la escena).

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene ask mi-escena materials3d '{"layer":"muneco"}'
```

#### `material_check`

Comprueba el código de un shader de material de la escena (el que crea `new_material`) y dice si compila. Es la línea «se compila» o el error con línea y columna del panel **Shader de material nuevo**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `name` | texto | Nombre del shader. |

La respuesta tiene `name`, `ok` (sí/no), `material` (si el descriptor es de un shader de material), `error` (el mensaje, o `null`), `line` y `column` del fallo (o `null`) y `source` (el código completo). Si faltan los archivos, `ok` es falso y `error` lo dice.

```sh
wprs-ctl scene act mi-escena scene3d '{"new_material":{"name":"brillo"}}'
wprs-ctl scene ask mi-escena material_check '{"name":"brillo"}'
```

```json
{"name":"brillo","ok":true,"material":true,"error":null,"line":null,"column":null,"source":"..."}
```

#### `depth3d`

Dice cómo se oculta tras los modelos más cercanos una capa de un grupo 3D. Es la nota bajo **Ocultarse tras los modelos más cercanos**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto (obligatorio) | Id de la capa. |

La respuesta es `null` si no hay esa capa. Si la hay: `layer`, `inside` (si está dentro de un grupo 3D), `depth_test` (si la prueba de profundidad está encendida), `particles` (si es una capa de partículas), `way` y `reason`. `way` es `one_by_one` cuando cada partícula se prueba por su cuenta contra los modelos, y `plane` cuando toda la capa se prueba como un plano. `reason` dice por qué unas partículas van en su plano: `perspective` (el sistema no tiene perspectiva) o `children` (un sistema hijo tiene sus propios hijos); es `null` en las demás capas. `caveat` vale `refract_edges` en partículas que refractan: su desvío se oculta tras los modelos más cercanos, pero junto al borde de uno puede arrastrar algunos de sus píxeles, porque la imagen que desvía no guarda profundidad; si no, `null`. `lowest` recuerda que en la calidad más baja toda capa va en su plano.

```sh
wprs-ctl scene act mi-escena scene3d '{"group":{"id":"mundo"}}'
wprs-ctl scene act mi-escena add_layer '{"kind":"solid","id":"fondo","into":"mundo"}'
wprs-ctl scene act mi-escena scene3d '{"depth_test":{"layer":"fondo","on":true}}'
wprs-ctl scene ask mi-escena depth3d '{"layer":"fondo"}'
```

#### `surface3d`

Dice qué parte del modelo hay bajo un punto de la vista de edición. Sirve para saber qué material se toca antes de pintarlo, peinarlo o tomar su color.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `at` | lista de 2 números | Punto del lienzo en píxeles. |
| `time` | número | Segundo de la escena. Por defecto 0. |

La respuesta es `null` si ahí no hay ningún modelo. Si lo hay: `layer`, `material`, `uv` (el lugar en la imagen del material, de 0 a 1), `point` (el punto 3D), `normal` (hacia dónde mira la superficie), `unit` (cuánto mide un píxel del lienzo en ese punto, en unidades del modelo) y `color` (el color de la superficie como `#rrggbb`). Sigue la pose de las animaciones en ese segundo, pero no las formas de mezcla.

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene ask mi-escena surface3d '{"at":[960,540]}'
```

```json
{"layer":"muneco","material":"body","uv":[0.125,1.0],"point":[0.0,0.0,0.25],"normal":[0.0,0.0,1.0],"unit":0.0051,"color":"#ffffff"}
```

### Escena 3D: escenario, coste y sombras

#### `stage3d`

Describe el escenario 3D tal como lo ve la vista de edición en un segundo: dónde está cada modelo, sus asas, la cámara y las luces, todo en píxeles del lienzo. Es la consulta que usa la aplicación para dibujar las asas, y sirve para saber qué puntos dar a [`grab`](#variante-grab-arrastrar-un-asa).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `time` | número | Segundo de la escena. Por defecto 0. |

Es `null` si la escena no se puede leer. La respuesta tiene estas partes:

- `size`: el tamaño del lienzo; `stages`: cuántos escenarios 3D hay (un modelo suelto o cada grupo 3D es uno).
- `view`: la vista de edición (`yaw`, `pitch`, `distance`, `target`, `through`) y `through`, si ahora se mira por la cámara.
- `camera`: la cámara de la escena (`eye`, `center`, `up`, `fov`, `near`, `far`).
- `models`: por cada modelo, `id`, `pointer` (dónde vive en la escena), `stage`, `at` (su centro en píxeles), `arms` (los extremos de las flechas x, y y z), `box` (su caja en pantalla `[x1, y1, x2, y2]`), `depth` y `length`, `position`, `rotation` y `scale` (como están escritos, o `null`), `now` (los valores de ahora), `held` (por cada uno, `fixed` si es un valor, `keyed` si tiene fotogramas clave y `bound` si está ligado a una propiedad) y `turns` (los anillos de giro y `grips`, los puntos donde se agarran, más `deep`, el asa de profundidad). Si el modelo tiene huesos, cada modelo trae además `bones` con la posición de cada hueso, y datos de sus animaciones y colisionadores.
- `frustum` y `handles`: con la vista libre, las líneas de lo que ve la cámara y los puntos de sus asas (`eye`, `center`, `up` y `rig` con `orbit`, `dolly` y `roll`). Mirando por la cámara son vacíos o `null`.
- `flat`: las capas planas de grupos 3D, con si tienen `depth_test`.
- `lights`: las luces, con `id`, `light` (tipo), `space` (si están en 3D), `at`, `aim`, `held` y `aim_held`.

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"view":{"orbit":[30,10]}}'
wprs-ctl scene ask mi-escena stage3d '{"time":0}'
```

#### `cost3d`

Calcula cuánto gasta el escenario 3D en cada nivel de calidad y en qué equipos cabe. Es el aviso de coste del panel **Modelo 3D**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `quality` | texto | `q0`, `q1`, `q2` o `q3`. Se repite en el campo `quality` de la respuesta; las cifras de todos los niveles vienen siempre. |

Es `null` si la escena no se puede leer. La respuesta tiene:

- `models`: los modelos visibles, con `id`, `triangles` y `textures`.
- `levels`: una entrada por nivel (`q0` a `q3`) con los `triangles` del modelo, los `drawn` que se dibujan, los `pixels` del lienzo a esa calidad, `scene` (el coste total), `shadow_texels`, `fur_shells`, `fur_triangles`, `skinned`, `colliders` y `left_out` (avisos de lo que ese nivel deja fuera).
- `tiers`: una entrada por nivel de equipo (`t0` a `t3`) con `fits` (si la escena cabe), `poster` (si se quedaría en una imagen fija), `busy`, `reasons`, `memory`, los máximos de `triangles` y `texture`, la `quality` que usa y las `shadows` que puede dibujar.
- `phone`: los límites de un teléfono; `groups`: cuántos grupos 3D hay.

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene ask mi-escena cost3d '{"quality":"q2"}'
```

#### `shadows3d`

Dice qué luces dan sombra y en qué niveles de equipo se dibujan. Es la información del panel **Sombras**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `time` | número | Segundo de la escena. Por defecto 0. |

La respuesta tiene `lights`, `caps`, `most` y `debug`. `most` es el máximo de luces con sombra de una escena. `caps` dice cuántas luces con sombra dibuja cada nivel de equipo (`t0` a `t3`). `debug` es la vista de depuración elegida, o `null`. Cada luz de `lights` trae `id`, `light` (tipo), `resolution` (el tamaño elegido, o `null` si es automático), `bias`, `tiers` (por nivel de equipo, `side` el lado en píxeles del mapa, y también `phone`, `low` en calidad mínima y `shown`, si ese nivel la dibuja) y `frustum` (los segmentos, en píxeles, con que la vista de edición dibuja el volumen de la sombra).

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena add_layer '{"kind":"light","id":"luz"}'
wprs-ctl scene act mi-escena scene3d '{"shadows":{"layer":"luz","on":true,"resolution":1024}}'
wprs-ctl scene ask mi-escena shadows3d '{"time":0}'
```

#### `paths3d`

Describe los recorridos de cámara: sus claves, la línea que dibujan y qué está pasando en un segundo. Es lo que dibuja el panel **Recorridos de cámara**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `time` | número | Segundo de la escena en que se mira. Por defecto 0. |

La respuesta tiene `paths`, `order`, `fade`, `total`, `time` y `playing`. `total` es la duración de todos los recorridos juntos y `fade` es el fundido (`to` y `duration`), o `null`. `playing` es la cámara en ese segundo (`eye`, `center`, `fov`, `fade`, `cross`, `moving`, `at`), o `null` si ningún recorrido se reproduce. Cada recorrido trae `id`, `loop`, `ease`, `period` (su duración), `keys` (por clave: `t`, `eye`, `center`, `fov` y sus puntos en pantalla `at` y `aim`), `spline` (la línea de la cámara en píxeles) y `aims` (la línea del punto al que mira). Los puntos en pantalla son `null` si no hay ningún modelo que mirar.

```sh
wprs-ctl scene act mi-escena import '{"file":"figura.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"figura","id":"cubo"}}}'
wprs-ctl scene act mi-escena scene3d '{"path":{"add":{"id":"vuelo"}}}'
wprs-ctl scene ask mi-escena paths3d '{"time":1}'
```

### Escena 3D: huesos de un modelo

Estas dos preguntas son las que usa la aplicación al arrastrar un hueso con el ratón. Hacen falta un modelo con huesos y una animación puesta en su capa (acción `clip`), porque sin animación el modelo no tiene esqueleto vivo. En `modelo.glb` los huesos se llaman `root` y `tip`.

#### `bone3d`

Convierte un gesto sobre un hueso en los valores que hay que guardar como fotograma clave: cuántos grados hay que girarlo (`angles`) o cuánto hay que deslizarlo (`shift`). Los gestos se dan en grados o en píxeles de la vista.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la capa de modelo. |
| `bone` | texto | Nombre del hueso. |
| `turn` | número | Giro en grados alrededor de la línea de visión. |
| `tilt` | lista de 2 números | Inclinación `[x, y]` en grados. |
| `slide` | lista de 2 números | Deslizamiento `[x, y]` en píxeles. |
| `time` | número | Segundo de la escena. Por defecto 0. |
| `clip` | texto | Id de la animación cuyos valores se toman como punto de partida. |
| `clip_time` | número | Segundo dentro de esa animación. Por defecto, `time`. |
| `from` | objeto | Valores de partida `{ "angles": [x, y, z], "shift": [x, y, z] }`; manda sobre `clip`. |

La respuesta es `null` si falta la capa, el hueso o la animación viva. Si no, `{ "angles": [x, y, z], "shift": [x, y, z] }`, los valores que se darían al hueso para ese gesto.

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene act mi-escena clip '{"layer":"muneco","id":"paso","animation":"walk","add":true,"play":true}'
wprs-ctl scene ask mi-escena bone3d '{"layer":"muneco","bone":"tip","turn":10}'
wprs-ctl scene ask mi-escena bone3d '{"layer":"muneco","bone":"tip","slide":[20,0],"clip":"paso","clip_time":0.5}'
```

```json
{"angles":[0.0,-0.0,10.0],"shift":[0.0,0.0,0.0]}
```

#### `bound3d`

Dice qué ángulo de giro del hueso queda más cerca de un punto de la vista. Sirve para arrastrar los topes de giro de un cuerpo con balanceo.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la capa de modelo. |
| `bone` | texto | Nombre del hueso. |
| `point` | lista de 2 números | Punto del lienzo en píxeles. |
| `time` | número | Segundo de la escena. Por defecto 0. |

La respuesta es `null` si falta la capa, el hueso o la animación viva. Si no, `{ "degrees": N }`, con N entre −180 y 180.

```sh
wprs-ctl scene act mi-escena import '{"file":"modelo.glb"}'
wprs-ctl scene act mi-escena scene3d '{"model":{"add":{"asset":"modelo","id":"muneco"}}}'
wprs-ctl scene act mi-escena clip '{"layer":"muneco","id":"paso","animation":"walk","add":true,"play":true}'
wprs-ctl scene ask mi-escena bound3d '{"layer":"muneco","bone":"tip","point":[1000,300]}'
```

```json
{"degrees":37.0}
```

