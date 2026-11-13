# Mallas, huesos y animación

Una capa `mesh` dibuja una imagen doblada por un esqueleto, como un personaje recortado en
piezas que mueve los brazos. Los modelos 3D (`model`) usan la misma animación. Esta página
explica la malla, los huesos y todo lo que los mueve: clips, capas de animación, cinemática
inversa, física, reglas, formas de mezcla y puntos de enganche.

El ejemplo es [`ejemplos/malla-con-huesos`](ejemplos/malla-con-huesos/scene.json), que pasa
`wprs-ctl scene validate`. Para los modelos 3D y las luces ver [3d.md](3d.md).

## La capa de malla

Una malla es un archivo glTF binario (`.glb`) de partes planas: posiciones (con una profundidad
opcional), coordenadas de textura y, para doblarse, pesos de hueso con un esqueleto. Una escena
puede tener hasta 16 capas `mesh` y `model` en total.

No hace falta escribir el `.glb` a mano. El editor recorta una malla de una imagen de la escena,
por una cuadrícula o por su contorno, y coloca los huesos (ver [editor.md](editor.md)). Desde la
terminal:

```
wprs-ctl scene mesh-from-picture MI-CARPETA cielo --outline
wprs-ctl scene bones MI-CARPETA figura
```

`mesh-from-picture` recorta una capa de malla de un recurso de imagen (por su contorno con
`--outline`, o por una cuadrícula de `--cells` celdas), lista para recibir huesos; `scene bone`
añade, mueve, cuelga y renombra huesos, y `scene weights` y `scene auto-weights` dan y reparten
sus pesos en los vértices (como mucho 4 huesos por vértice).

`bones` imprime los huesos, partes, formas de mezcla, expresiones, animaciones y clips de una
capa de malla, y es la forma de ver con qué nombres cuentas.

```json
    { "id": "figura", "type": "mesh", "mesh": "muneco", "texture": "cielo", "size": [100, 100], "transform": { "scale": 8 },
      "clips": [
        { "id": "asentir", "loop": "pingpong",
          "bones": { "neck": { "rotation": [[0, -12], [0.8, 12, "in_out_sine"]] } },
          "shapes": { "smile": [[0, 0], [0.8, 1]] },
          "events": [{ "t": 0.4, "name": "mitad" }] }
      ],
      "animations": [{ "clip": "asentir", "weight": 1, "rate": 1, "blend_in": 0.3 }],
      "physics": [{ "bone": "neck", "mode": "spring", "stiffness": 0.6, "damping": 0.4, "inertia": 0.5 }],
      "ik": [{ "chain": ["hip", "neck"], "target": [60, 20], "limits": { "neck": [-30, 30] }, "weight": 0.3, "bend": "clockwise" }],
      "rules": [{ "bone": "neck", "parent": "hip", "weight": 1 }],
      "parts": { "chest": { "order": 1 }, "legs": { "order": 0 } },
      "shapes": { "smile": 0.2 } }
```

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `mesh` | obligatorio | El recurso `.glb`. |
| `texture` | las del archivo | Un recurso de imagen o de vídeo (o un valor) que reemplaza las texturas del archivo. Sin él, cada parte enseña la imagen de su material (hasta 64 por archivo; una imagen que se repite sobre su parte, con coordenadas fuera de 0 a 1, se repite hasta 4 veces por lado, según el modo de envoltura del muestreador de su textura en el archivo: repetir, repetir en espejo o fijar al borde; más allá de 4 veces se ve el borde), o blanco. |
| `size` | el lienzo | La caja en píxeles que llena la malla. |
| `color` | blanco | Color que multiplica la malla. |
| `parts` | | Por parte (el nombre de una malla del archivo): `order`, `clip_to` y `visible`. |
| `depth` | | `{ "scale", "cull", "sort" }`: profundidad de los vértices. |
| `clips` | | Hasta 64 estiramientos de animación. |
| `animations` | | Hasta 16 capas de animación. |
| `ik` | | Hasta 16 cadenas que alcanzan un objetivo. |
| `physics` | | Hasta 64 cuerpos que se balancean. |
| `rules` | | Hasta 64 reglas que cambian el padre de un hueso. |
| `shapes`, `expressions` | | Formas de mezcla y expresiones. |
| `hit` | | Huesos que reciben el puntero. |

Las posiciones del archivo están en píxeles, desde el medio de la caja que llena la capa, con
`x` a la derecha e `y` hacia arriba, como los escribe glTF; las coordenadas de textura empiezan
arriba a la izquierda. El tamaño de la malla es el de sus coordenadas: la figura del ejemplo mide
20 por 40 píxeles y por eso la capa lleva `"transform": { "scale": 8 }`. Las partes que salen de la caja
en reposo tienen un margen alrededor, de hasta la mitad de la caja por lado, y no se cortan; las que una
pose lleva más lejos que eso se cortan en el borde del margen, así que `size` les deja sitio.

Cada nodo del archivo es una articulación: los huesos de los esqueletos, los nodos que llevan una
malla sin esqueleto (que se mueven en una pieza) y los nodos sin malla, como los puntos de
enganche, que siguen a su padre. La pose de reposo es la transformación de cada nodo. Cada vértice
sigue a hasta 4 huesos cuyos pesos suman 1 (con 0,02 de margen).

El motor dibuja la malla una vez y solo la vuelve a dibujar cuando cambian la pose, las formas de
mezcla, el orden de las partes o un vídeo de textura. Un esqueleto quieto no pide fotogramas. En el
nivel de calidad más bajo, un esqueleto en movimiento pide como mucho 30 fotogramas por segundo.

### Límites del archivo

Se rechaza un archivo con un ciclo de huesos o un nodo con dos padres, un vértice cuyos pesos no
suman 1 o que sigue a un hueso de ningún esqueleto, un índice fuera de los vértices, primitivas que
no sean triángulos (las tiras y los abanicos se leen como triángulos), claves de animación
desordenadas, una forma de mezcla repetida, más de 1 024 nodos, 65 536 vértices, 1 048 576 esquinas
de triángulo, 256 huesos y partes, 64 primitivas dibujadas, 524 288 vértices de formas de mezcla o
4 194 304 valores de animación, o 128 MiB de mallas en total.

## Partes

Las partes se dibujan por `order` (de -1000 a 1000, 0 por defecto; puede seguir claves) y, si
empatan, en el orden del archivo. `clip_to` es otra parte sobre la que esta solo se ve (un
recorte con plantilla), y `visible` la oculta.

```json
"parts": { "chest": { "order": 1 }, "legs": { "order": 0 } }
```

`depth` hace que la profundidad de los vértices (la z del archivo, hacia quien mira, en píxeles)
los empuje con la perspectiva de `canvas.perspective` o de la cámara 3D, incluso bajo `tilt`.
`scale` va de 0 a 10 (1 por defecto), `cull` esconde los triángulos vistos por detrás y `sort`
(`true` por defecto) dibuja las partes de atrás adelante cada fotograma. Con `depth`, los
vértices más cercanos tapan a los lejanos sea cual sea el orden de sus partes.

## Clips

Un clip es un tramo de animación con nombre. Puede venir del archivo o llevar claves propias.

```json
    { "id": "figura", "type": "mesh", "mesh": "muneco", "texture": "cielo", "size": [100, 100], "transform": { "scale": 8 },
      "clips": [
        { "id": "asentir", "loop": "pingpong",
          "bones": { "neck": { "rotation": [[0, -12], [0.8, 12, "in_out_sine"]] } },
          "shapes": { "smile": [[0, 0], [0.8, 1]] },
          "events": [{ "t": 0.4, "name": "mitad" }] }
      ],
      "animations": [{ "clip": "asentir", "weight": 1, "rate": 1, "blend_in": 0.3 }],
      "physics": [{ "bone": "neck", "mode": "spring", "stiffness": 0.6, "damping": 0.4, "inertia": 0.5 }],
      "ik": [{ "chain": ["hip", "neck"], "target": [60, 20], "limits": { "neck": [-30, 30] }, "weight": 0.3, "bend": "clockwise" }],
      "rules": [{ "bone": "neck", "parent": "hip", "weight": 1 }],
      "parts": { "chest": { "order": 1 }, "legs": { "order": 0 } },
      "shapes": { "smile": 0.2 } }
```

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `id` | obligatorio | El nombre del clip. |
| `animation` | la primera del archivo | Qué animación del archivo usa. Sin ella y sin claves, usa la primera. |
| `from`, `to` | 0 y el final | El tramo, en segundos. |
| `loop` | `repeat` | `repeat`, `pingpong` o `once`. |
| `events` | | Sucesos con nombre `{ "t", "name" }`, en segundos desde el principio del clip. Se disparan cada vez que el clip los pasa, y empiezan lo que `event:<nombre>` dispare; llegan a los scripts como `timelineEvent` con la línea de tiempo `<capa>#<clip>`. |
| `bones` | | Claves propias por hueso. |
| `shapes` | | Claves propias por forma de mezcla. |
| `order` | | Claves propias del orden de cada parte. |

Las claves son listas de `[tiempo, valor]` con un suavizado opcional, como en las líneas de
tiempo (ver [escena.md](escena.md)). Un clip con claves propias y sin `animation` dura hasta su
última clave.

| Clave | Qué hace |
|-------|----------|
| `bones` | Por hueso: `rotation` en grados, en el sentido del reloj; `position` `[x, y]` en píxeles, con y hacia abajo, en el espacio de su padre; y `scale` `[x, y]`. Se suman a su pose de reposo y reemplazan las pistas del archivo para ese hueso. En un modelo 3D también `angles` (grados alrededor de x, y, z) y `shift` (unidades del modelo). |
| `shapes` | Por forma de mezcla: su peso, de -1 a 1. |
| `order` | Por parte: el `order` que toma mientras suena el clip, redondeado; `parts.<nombre>.order` gana si está puesto. |

Desde la terminal: `wprs-ctl scene clip`, `scene key`, `scene keys`.

## Capas de animación

`animations` mezcla clips, en orden: primero las capas que reemplazan y luego las aditivas.

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `clip` | obligatorio | | El id de un clip. |
| `weight` | 1 | 0 a 1 | Cuánto pesa. Puede seguir valores. |
| `rate` | 1 | 0 a 16 | La velocidad. |
| `additive` | `false` | | Suma lo que el clip aparta del reposo, por su peso. |
| `blend_in`, `blend_out` | 0 | 0 a 10 s | Cuánto tarda el peso en subir al empezar o encenderse, y en bajar al apagarse, pararse o (con `once`) terminar. |
| `enabled` | `true` | | Enciende o apaga la capa. Dos capas que intercambian `enabled` se funden. |
| `bones` | todos | | Limita la capa a estos huesos. |

Una capa que reemplaza lleva cada hueso desde donde lo dejaron las anteriores hacia su pose, por
su peso; las rotaciones se mezclan por el camino corto. Los huesos sin pista se quedan en su pose
de reposo. Un clip `once` termina y se queda en su última pose, y avisa a los scripts. Las formas
de mezcla siguen la misma mezcla. `wprs-ctl scene animate` añade capas de animación y lista los
clips y los huesos de una capa de malla o de modelo.

## Cinemática inversa

`ik` hace que una cadena de huesos alcance un objetivo, como un brazo que llega a una mano.

| Campo | Por defecto | Qué hace |
|-------|-------------|----------|
| `chain` | obligatorio | De 2 a 8 huesos, cada uno colgado del anterior. |
| `target` | obligatorio | Un punto `[x, y]` en píxeles de la caja desde su esquina de arriba a la izquierda (puede seguir valores), `"cursor"` (el puntero, que necesita la entrada `pointer`), `"bone:<nombre>"` o `"layer:<id>"`. |
| `limits` | | Por hueso `[mín, máx]` grados respecto a su reposo. |
| `weight` | 1 | De 0 a 1: mezcla el giro con la pose. |
| `bend` | `clockwise` | `clockwise` o `counterclockwise`: hacia dónde se dobla cada articulación. |

Una cadena de dos huesos se resuelve exactamente; una más larga hace hasta 10 rondas. Un objetivo
fuera de alcance estira la cadena hacia él.

## Física de huesos

`physics` hace que un hueso se balancee. Cada cuerpo da un paso por fotograma dibujado, y el
movimiento coincide a cualquier ritmo.

| Campo | Por defecto | Rango | Qué hace |
|-------|-------------|-------|----------|
| `bone` | obligatorio | | El hueso. |
| `mode` | `spring` | `spring`, `rigid`, `rope` | `spring` tira de la punta de su hueso hacia donde la pose la pone; `rigid` se balancea como un péndulo flojo; `rope` une el hueso y sus primeros hijos (hasta 8 eslabones) en una cuerda que cuelga y se mece. |
| `stiffness` | 0,5 | 0 a 1 | La rigidez del muelle. |
| `damping` | 0,5 | 0 a 1 | Qué tan rápido se calma. |
| `inertia` | 0,5 | 0 a 1 | Cuánto se queda atrás cuando se mueve su padre. |
| `wind` | 0 | 0 a 1 | Cuánto lo empuja `canvas.wind`. |
| `gravity` | 0 | -10 a 10 | Lo tira hacia abajo (98 píxeles por segundo al cuadrado por cada unidad). |
| `rotation` | | `[mín, máx]` grados | Cuánto puede girar cada eslabón desde su pose. |
| `translation` | | `[mín, máx]` px | Cuánto puede moverse la cabeza del hueso. |

El viento de la escena (`canvas.wind`, con `canvas.gusts`) se fija desde la terminal con
`wprs-ctl scene wind`. En los modelos 3D, la física se mueve en el plano x e y del modelo y cuenta
una unidad del modelo como 100 píxeles para la gravedad, el viento y la traslación. El viento y la gravedad actúan según x, y y -y del modelo, no del mundo: un modelo girado los recibe girados. Un cuerpo que
no se ha movido en medio segundo, con la pose y el viento quietos, no pide fotogramas. En el
nivel más bajo los cuerpos se quedan con la pose de la animación. `wprs-ctl scene physics`
crea los cuerpos y dibuja una cuerda de huesos nuevos.

## Reglas de huesos

`rules` cambia el padre de un hueso mientras `weight` va de 0 a 1, manteniendo su pose, como una
espada que pasa de una roca a una mano. Cada regla es `{ "bone", "parent", "weight" }` y no puede
cerrar un ciclo de padres.

## Formas de mezcla y expresiones

Las formas de mezcla son deformaciones que vienen en el archivo (una sonrisa). `shapes` les da un
peso de 0 a 1 por nombre:

(En el ejemplo, `"shapes": { "smile": 0.2 }`; en lugar de un número puede ir un valor que se mueve,
como `{ "prop": "animo" }`.)

`expressions` define hasta 64 conjuntos con nombre de pesos de formas, `{ "<nombre>": { "<forma>":
peso } }` (los pesos de -1 a 1), cuyo nombre `shapes` también puede fijar. Como mucho 8 formas
actúan a la vez sobre cada parte: las más pesadas.

## Pegar capas a un hueso

`attach` hace que una capa siga un hueso o un punto de una malla o un modelo, en lugar de un
`parent`:

| Campo | Qué hace |
|-------|----------|
| `layer` | La capa de malla o de modelo a la que se pega. |
| `bone` o `point` | El hueso o el punto de enganche. |
| `offset` | `[x, y]` en píxeles a lo largo del hueso. |
| `inherit` | Lo que toma: `position`, `rotation` y `scale` (todo por defecto). Sin `position`, la capa se queda en el origen de la malla. |

La capa se oculta con su malla, y su propio `transform` se aplica encima. Una capa no puede
seguirse a sí misma, ni a través de otras. Con `wprs-ctl scene attach` y `scene point` se pegan
y se crean los puntos de enganche.

## Tocar un hueso

`hit` lista los huesos cuya zona recibe el puntero para los scripts: la capa recibe el puntero
como una capa `solid`, y cada suceso lleva el hueso listado que dobla el triángulo dibujado bajo
el puntero, o el listado del que cuelga. También sirven los disparadores `click:`, `enter:` y
`leave:` de la capa.

## Modelos animados

Un modelo 3D con cualquiera de `clips`, `animations`, `ik`, `physics`, `rules`, `shapes`,
`expressions` o `root_motion` se lee con su esqueleto: las articulaciones y los nodos con mallas
se vuelven huesos. Todo funciona como en las mallas, mezclando las animaciones del archivo. Los
objetivos de `ik` de un modelo son `bone:`, un punto `[x, y]` en píxeles de la caja, `layer:` y
`cursor`.

Con `root_motion: true`, el hueso raíz conserva la pose que tiene cuando empiezan los clips, y lo
que los clips le hacen mueve la capa: en x, y y z, girando donde el clip gira, y acumulando cada
vuelta de un clip que se repite, de modo que un paso camina y un paso que gira camina en círculo.
Necesita `clips`.

```json
      { "id": "persona", "type": "model", "model": "figura", "position": [1.8, -0.2, 0], "size": [800, 800],
        "clips": [{ "id": "andar", "animation": "walk", "loop": "repeat" }],
        "animations": [{ "clip": "andar" }],
        "materials": { "body": { "color": "#9ad1ff" } } }
```

Un modelo animado cuesta lo mismo en todos los niveles de calidad y en todas las categorías: unas
pocas muestras por hueso raíz por fotograma. Un modelo sin ninguno de esos campos aplica una sola
vez los pesos de su archivo y no cuesta nada más mientras se reproduce.

## No cubierto aquí

- Los scripts que empujan huesos (`impulse`) y los que ponen la pose: [scripts.md](scripts.md).
- Cómo se ve cada herramienta del editor (esqueleto, pesos, pintura de profundidad): [editor.md](editor.md)
  y [acciones.md](acciones.md).
