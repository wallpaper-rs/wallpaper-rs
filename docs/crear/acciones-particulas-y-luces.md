# Acciones: partículas, luces y sonido

Las acciones y preguntas de los sistemas de partículas, las luces, la luz ambiente y el sonido. Cómo llamarlas, cómo leer su salida y sus errores está en [acciones](acciones.md).

## Acciones

### Partículas

**Antes de empezar.** Un sistema de partículas tiene un id (por ejemplo `chispas`) y vive en la escena; una capa de partículas lo dibuja. Al añadir una capa de partículas, `add_layer` con `"kind":"particles"` crea la capa y un sistema con su mismo id. Todas las variantes de `particles` llevan `system`, el id del sistema, salvo `scripts`. Los emisores, las fuerzas, los choques y los hijos se nombran por su posición en su lista, desde 0. Los puntos de control se llaman `p0` a `p7`. Las variantes que cambian el dibujo (`cutout`, `orient`, `shader`, `strand`, `texture`, `lit`, `refract`, `over`, `render`) aceptan `layer`: con una capa de partículas, el cambio vale solo para esa capa, que desde entonces dibuja con una copia propia del dibujo del sistema; sin `layer`, vale para el sistema y para todas las capas que lo dibujan. Cada cambio se rechaza, sin tocar nada, si deja la escena con un problema que no tenía.

#### `particles`

Edita un sistema de partículas: su forma de emitir, sus puntos de control, sus fuerzas, sus choques, sus hijos, cómo se dibujan y cómo les afecta la luz. Cada llamada usa una sola variante. En la app son los paneles del sistema de partículas.

##### variante `create`

Crea un sistema nuevo, vacío o copia de otro. No crea ninguna capa; para verlo, añade una capa de partículas que lo nombre en `source`. Con `parent`, queda como hijo de otro sistema.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema nuevo; no puede estar en uso. |
| `from` | texto | Sistema que se copia. Sin valor, empieza con un sistema básico. |
| `parent` | texto | Sistema del que será hijo. |
| `on` | texto | Con `parent`, cuándo nace el hijo: `static` (por defecto, por su cuenta), `spawn` (con cada partícula que nace), `death` (cuando muere) o `follow` (siguiéndola). |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"create":{"system":"humo","from":"chispas"}}'
wprs-ctl scene act mi-escena particles '{"create":{"system":"estela","parent":"chispas","on":"death"}}'
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"capa-humo","source":"humo"}'
```

##### variante `delete`

Borra un sistema. Se rechaza mientras una capa u otro sistema lo nombre.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"create":{"system":"humo"}}'
wprs-ctl scene act mi-escena particles '{"delete":{"system":"humo"}}'
```

##### variante `rename`

Da otro id a un sistema. Las capas y los sistemas que lo nombran se actualizan; los guiones que lo nombran entre comillas no (usa `scripts`).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id actual. |
| `to` | texto | (obligatorio) Id nuevo; no puede estar en uso. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"rename":{"system":"chispas","to":"brasas"}}'
```

##### variante `scripts`

Después de renombrar un sistema, reescribe en los guiones de la escena cada texto que es exactamente el id viejo (entre comillas dobles, simples o invertidas) por el nuevo. Lo que solo contiene el nombre dentro de un texto más largo no se toca. Los archivos cambian al momento y Deshacer no los devuelve. Se rechaza mientras exista un sistema con el id viejo.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `from` | texto | (obligatorio) Id viejo. |
| `to` | texto | (obligatorio) Id de un sistema que existe. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"rename":{"system":"chispas","to":"brasas"}}'
wprs-ctl scene act mi-escena particles '{"scripts":{"from":"chispas","to":"brasas"}}'
```

##### variante `settings`

Ajustes generales del sistema. Solo cambia lo que nombras.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `capacity` | entero | Máximo de partículas vivas a la vez, de 1 a 200000. |
| `seed` | entero | Semilla del azar: la misma semilla da las mismas partículas. |
| `prewarm` | número | Segundos que se simulan antes de mostrar el sistema, para que no arranque vacío (de 0 a 30). |
| `space` | texto | `local` (las partículas siguen a la capa) o `world` (la capa solo las coloca al nacer). |
| `rate` | número | Partículas por segundo de los emisores. |
| `emitter` | entero | Con `rate`, un solo emisor; sin valor, todos. |
| `replace` | sí/no | Con `rate`, pisa un ritmo que seguía teclas u otra fuente. Por defecto `false`. |
| `lock` | lista de textos | Valores que una capa no puede cambiar por su cuenta: `alpha`, `size`, `count`, `speed`, `life`, `time_scale`, `brightness`, `color`, `points`. Con `["none"]` se quitan todos. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"settings":{"system":"chispas","capacity":3000,"seed":7,"prewarm":2,"space":"world","rate":80,"lock":["alpha","size"]}}'
wprs-ctl scene act mi-escena particles '{"settings":{"system":"chispas","lock":["none"]}}'
```

##### variante `shape`

Cambia la forma de un emisor, es decir, de dónde nacen las partículas. Con el número del emisor siguiente al último, añade un emisor nuevo con 10 partículas por segundo.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `emitter` | entero | (obligatorio) Posición del emisor. |
| `shape` | cualquier valor JSON | (obligatorio) La forma, en píxeles del lienzo: `"edge"` (el borde), `{"point":[x,y]}`, `{"box":[[x0,y0],[x1,y1]]}`, `{"line":[[x0,y0],[x1,y1]]}`, `{"circle":{"center":[x,y],"radius":r,"inner":r2}}` (un anillo si `inner` es mayor que 0), `{"mask":{"texture":"id","threshold":0.5}}` (donde la imagen estirada sobre el lienzo es opaca), `{"around":{"at":"p0","count":8,"radius":100}}` (en turnos alrededor de un punto) o `{"between":{"from":"p0","to":"p1","count":10}}` (a lo largo de una línea entre dos puntos). |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"shape":{"system":"chispas","emitter":0,"shape":{"circle":{"center":[960,540],"radius":300,"inner":100}}}}'
wprs-ctl scene act mi-escena particles '{"shape":{"system":"chispas","emitter":1,"shape":{"box":[[0,1000],[1920,1080]]}}}'
```

##### variante `layer`

Hace que un emisor nazca de donde otra capa es opaca, con su color y su movimiento. Es una forma `layer` del emisor, con sus propios ajustes. La capa no puede ser la que dibuja el propio sistema.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `emitter` | entero | (obligatorio) Posición del emisor. |
| `id` | texto | (obligatorio) Id de la capa de la que nacen. |
| `threshold` | número | Opacidad mínima que cuenta como sólido, de 0 a 1. Por defecto 0,5. |
| `color` | sí/no | Si es `true`, las partículas nacen con el color de la capa. Por defecto `false`. |
| `inherit` | sí/no | Si es `true`, heredan su movimiento. Por defecto `false`. |
| `jitter` | número | Dispersión del punto de nacimiento. |
| `refresh` | texto | `static` (lee la capa una vez) o `on_change` (otra vez cada vez que cambia). |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png","id":"cuadro"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"layer":{"system":"chispas","emitter":0,"id":"cuadro","threshold":0.4,"color":true,"inherit":true,"jitter":2,"refresh":"on_change"}}'
```

##### variante `emit`

Cambia valores de un emisor que no son su forma ni su ritmo: ráfagas, seguir al puntero, duración, retardo, pausas periódicas y silencio. Un valor `null` o `false` quita ese valor.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `emitter` | entero | (obligatorio) Posición del emisor. |
| `values` | objeto | Valores que cambian: `bursts` (hasta 8 ráfagas `{"count": n, "every": segundos, "on": "beat" o "click", "at": segundos}`), `follow` (`"cursor"`: la forma sigue al puntero), `inherit` (fracción de la velocidad del puntero que heredan), `duration` (segundos tras los que deja de emitir), `delay` (segundos de espera antes de empezar), `periodic` (`{"on":[min,max],"off":[min,max]}`, segundos de emisión y de pausa) y `audio_gate` (`true`: no nace nada mientras el sonido calla). |
| `clear` | lista de textos | Nombres de valores que se quitan. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"emit":{"system":"chispas","emitter":0,"values":{"bursts":[{"count":30,"every":2}],"delay":1,"duration":10}}}'
wprs-ctl scene act mi-escena particles '{"emit":{"system":"chispas","emitter":0,"values":{"periodic":{"on":[1,2],"off":[1,3]},"audio_gate":true},"clear":["duration"]}}'
```

##### variante `pace`

Limita cuántas partículas puede hacer nacer un emisor en cada paso de 1/60 de segundo; las que sobran esperan al paso siguiente.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `emitter` | entero | Emisor al que se aplica. Sin valor, todos. |
| `max_per_step` | entero | Máximo de nacimientos por paso (al menos 1). Sin valor, quita el límite. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"pace":{"system":"chispas","max_per_step":4}}'
```

##### variante `point`

Pone, mueve o quita un punto de control (`p0` a `p7`), un lugar que las formas, las fuerzas y los hijos pueden nombrar. Los puntos que faltan antes del que pones empiezan en `[0, 0]`.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `point` | entero | (obligatorio) Número del punto, de 0 a 7. |
| `at` | lista de números | Lugar `[x, y]` en píxeles de la capa. |
| `follow` | sí/no | `true` hace que siga al puntero; `false` lo suelta. |
| `angle` | número | Giro del punto en grados, de −360 a 360. |
| `remove` | sí/no | Quita el punto; solo se puede quitar el último. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"point":{"system":"chispas","point":0,"at":[100,50],"angle":30}}'
wprs-ctl scene act mi-escena particles '{"point":{"system":"chispas","point":1,"follow":true}}'
wprs-ctl scene act mi-escena particles '{"point":{"system":"chispas","point":1,"remove":true}}'
```

##### variante `start`

Cambia los valores con que nace cada partícula. Cada rango es `[mínimo, máximo]` y la partícula toma un valor al azar entre los dos; con un tercer número (un exponente de 0,1 a 10) el azar se inclina hacia un extremo. Un extremo `null` conserva el actual.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `ranges` | objeto | Rangos que cambian: `life` (segundos de vida), `speed` (píxeles por segundo), `angle` (grados, 0 hacia la derecha), `size` (píxeles), `rotation`, `spin` (grados por segundo), `frame` (primer cuadro de una hoja de imágenes), `color` (dos colores que se mezclan al azar), `alpha` (opacidad), `velocity` (`[[vx0,vy0],[vx1,vy1]]`, en lugar de `speed` y `angle`), `radial` (velocidad que se aleja del centro de la forma), `depth` y `depth_speed` (necesitan perspectiva), y uno solo de `hsv` (`{"hue":[0,360],"steps":6,"sat":[a,b],"val":[a,b]}`), `palette` (`{"colors":[...],"noise":[h,s,v]}`) o `gradient` (paradas `[t, color]`), que reemplazan a `color`. |
| `clear` | lista de textos | Rangos que se quitan: `alpha`, `velocity`, `radial`, `hsv`, `palette` o `gradient`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"start":{"system":"chispas","ranges":{"life":[1,3],"size":[4,10,2],"color":["#ffd27a","#ff7a3d"]}}}'
wprs-ctl scene act mi-escena particles '{"start":{"system":"chispas","ranges":{"gradient":[[0,"#ff0000"],[1,"#0000ff"]],"alpha":[0.5,1]},"clear":["velocity"]}}'
```

##### variante `turbulent`

Da a cada partícula una velocidad inicial que sigue un ruido que se mueve, como el aire. Solo cambia los campos que nombras.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `turbulent` | objeto | Campos: `forward` (empuje hacia delante), `scale` (tamaño del remolino), `noise_speed` (rapidez del ruido), `offset`, `speed` (`[mín, máx]`), `phase` y `audio` (banda del sonido que mueve la fase: `bass`, `mid`, `treble` o `level`). |
| `quiet` | sí/no | Si es `true`, quita la banda de sonido. Por defecto `false`. |
| `off` | sí/no | Si es `true`, quita la turbulencia entera. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"turbulent":{"system":"chispas","turbulent":{"forward":20,"scale":0.01,"audio":"bass"}}}'
wprs-ctl scene act mi-escena particles '{"turbulent":{"system":"chispas","quiet":true}}'
```

##### variante `offset`

Desplaza el punto de nacimiento de cada partícula con un ruido, para que no nazcan en línea. Solo cambia los campos que nombras.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `offset` | objeto | Campos: `distance` (píxeles), `scale`, `sign`, `octaves` (de 1 a 6), `noise_scale`, `noise_speed`. |
| `off` | sí/no | Si es `true`, quita el desplazamiento. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"offset":{"system":"chispas","offset":{"distance":30,"scale":0.01,"octaves":3}}}'
```

##### variante `remap`

Añade, cambia o quita una regla que convierte un dato (la vida, la distancia a un punto, el sonido...) en un valor de la partícula. Sin `index` añade al final.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `remap` | objeto | La regla: `input` (`daytime`, `runtime`, `life`, `age`, `speed`, `random`, `distance:p0`..`distance:p7`, `audio:banda` o `cursor:x` / `cursor:y`), `in` (`[desde, hasta]` del dato), `output` (`size`, `alpha`, `color`, `speed`, `life`, `rotation`, `spin` o `frame`), `out` (`[desde, hasta]` del resultado), y opcionales `clamp_in` y `clamp_out` (`true` para no pasarse), `op` (`set`, `mul`, `add` o `sub`) y `wave` (`sine` o `noise`). |
| `index` | entero | Regla que se reemplaza o se quita. |
| `remove` | sí/no | Con `index`, la quita. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"remap":{"system":"chispas","remap":{"input":"life","in":[0,1],"output":"alpha","out":[1,0]}}}'
wprs-ctl scene act mi-escena particles '{"remap":{"system":"chispas","index":0,"remove":true}}'
```

##### variante `force`

Añade, reemplaza o quita una fuerza que actúa sobre las partículas. Sin `index` añade al final; con `index` y `force`, reemplaza esa fuerza y conserva su ventana de vida; con `index` y `remove`, la quita.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `force` | cualquier valor JSON | La fuerza: `{"gravity":[x,y]}`, `{"wind":[x,y]}`, `{"drag":k}`, `{"turbulence":{"strength":s,"scale":c}}`, `{"attractor":{"target":"cursor","strength":s}}` (negativa, repele), `{"vortex":{"center":[x,y],"strength":s}}`, `{"orbit":{"center":[x,y],"strength":s}}`, `{"noise":s}`, `{"cap":velocidad}`, `{"torque":k}`, `{"spin_drag":k}`, `{"oscillate":{"what":"position","freq":[a,b],"scale":[a,b]}}`, `{"slow_near":{"target":[x,y],"inner":r1,"outer":r2,"factor":f}}`, `{"keep_distance":{"target":"p0","distance":d,"strength":s}}` o `{"keep_between":{"from":"p0","to":"p1","strength":s}}`. Un objetivo es un punto, `"cursor"` o un punto de control. |
| `index` | entero | Fuerza que se reemplaza o se quita. |
| `remove` | sí/no | Con `index`, la quita. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"force":{"system":"chispas","force":{"gravity":[0,-30]}}}'
wprs-ctl scene act mi-escena particles '{"force":{"system":"chispas","force":{"attractor":{"target":"cursor","strength":5000}}}}'
wprs-ctl scene act mi-escena particles '{"force":{"system":"chispas","index":0,"force":{"gravity":[0,-60]}}}'
wprs-ctl scene act mi-escena particles '{"force":{"system":"chispas","index":1,"remove":true}}'
```

##### variante `window`

Pone o quita la ventana de vida de una fuerza: en qué parte de la vida de la partícula aparece, está entera y desaparece.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `index` | entero | (obligatorio) Posición de la fuerza. |
| `window` | lista de números | Cuatro fracciones de la vida, de 0 a 1 y en orden: `[empieza a entrar, entra del todo, empieza a salir, ha salido]`. Sin valor, quita la ventana. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"force":{"system":"chispas","force":{"gravity":[0,-30]}}}'
wprs-ctl scene act mi-escena particles '{"window":{"system":"chispas","index":0,"window":[0,0.1,0.8,1]}}'
```

##### variante `boids`

Hace que las partículas se muevan en bandada (se separan, se alinean y se juntan). Solo cambia los campos que nombras; si no había bandada, empieza con radio 40. Vale para sistemas de hasta 8192 partículas.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `boids` | objeto | Campos: `radius` (distancia a la que ven a las vecinas, de 1 a 10000), `separation`, `alignment`, `cohesion` (de 0 a 10) y `max_speed`. |
| `off` | sí/no | Si es `true`, quita la bandada. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"boids":{"system":"chispas","boids":{"radius":50,"cohesion":0.8}}}'
wprs-ctl scene act mi-escena particles '{"boids":{"system":"chispas","off":true}}'
```

##### variante `collider`

Añade, reemplaza o quita un choque: una pared contra la que las partículas rebotan, se deslizan, se paran, se pegan o mueren. Sin `index` añade al final.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `collider` | cualquier valor JSON | El choque: `{"bounds": pared, "bounce": 0 a 1, "friction": 0 a 1, "kill": true, "stick": true}`. La pared puede ser `"canvas"` (los bordes del lienzo), `{"plane":{"point":[x,y],"normal":[x,y]}}` (se quedan del lado al que apunta la normal), `{"circle":{"center":[x,y],"radius":r}}` (se quedan fuera), `{"box":{"from":[x,y],"to":[x,y],"inside":true}}`, `{"segment":{"from":[x,y],"to":[x,y]}}`, `{"mask":{"texture":"id","threshold":0.5}}` (la parte opaca de una imagen) o `{"layer":{"id":"capa"}}` (la forma opaca de una capa). |
| `index` | entero | Choque que se reemplaza o se quita. |
| `remove` | sí/no | Con `index`, lo quita. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"collider":{"system":"chispas","collider":{"bounds":"canvas","bounce":0.4}}}'
wprs-ctl scene act mi-escena particles '{"collider":{"system":"chispas","collider":{"bounds":{"segment":{"from":[0,500],"to":[1000,600]}},"bounce":0.5}}}'
wprs-ctl scene act mi-escena particles '{"collider":{"system":"chispas","index":1,"remove":true}}'
```

##### variante `response`

Cambia qué hace un choque ya puesto cuando una partícula lo toca.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `index` | entero | (obligatorio) Posición del choque. |
| `response` | texto | (obligatorio) `bounce` (rebota), `slide` (se desliza), `stop` (se para), `stick` (se pega) o `kill` (muere). |
| `bounce` | número | Con `bounce`, cuánto rebota, de 0 a 1. Por defecto conserva el actual (0,5 si no tenía). |
| `friction` | número | Rozamiento, de 0 a 1. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"collider":{"system":"chispas","collider":{"bounds":"canvas","bounce":0.4}}}'
wprs-ctl scene act mi-escena particles '{"response":{"system":"chispas","index":0,"response":"slide","friction":0.2}}'
```

##### variante `child`

Añade, reemplaza o quita un hijo: otro sistema que nace con cada partícula, cuando muere o siguiéndola. Sin `index` añade al final. Cada sistema puede tener hasta 4 hijos y no pueden ser más de dos niveles.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema padre. |
| `child` | cualquier valor JSON | El hijo: `{"system": "id", "on": "static" o "spawn" o "death" o "follow", "probability": 0 a 1, "max": n, "offset": [x,y], "angle": grados, "scale": 0 a 100, "points_from": true}`. Solo `system` y `on` son obligatorios. |
| `index` | entero | Hijo que se reemplaza o se quita. |
| `remove` | sí/no | Con `index`, lo quita. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"create":{"system":"humo"}}'
wprs-ctl scene act mi-escena particles '{"child":{"system":"chispas","child":{"system":"humo","on":"spawn","probability":0.5}}}'
wprs-ctl scene act mi-escena particles '{"child":{"system":"chispas","index":0,"remove":true}}'
```

##### variante `inherit`

Elige qué toma un hijo de la partícula que lo hizo nacer.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema padre. |
| `child` | entero | (obligatorio) Posición del hijo. |
| `inherit` | lista de textos | Qué hereda: `color`, `size`, `alpha` y `velocity`. Una lista vacía quita la herencia. |
| `follow` | sí/no | Si es `true`, el hijo lo toma de nuevo en cada paso mientras la sigue. Solo tiene sentido en un hijo que sigue (`follow`). |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"create":{"system":"estela","parent":"chispas","on":"follow"}}'
wprs-ctl scene act mi-escena particles '{"inherit":{"system":"chispas","child":0,"inherit":["color","size"],"follow":true}}'
```

##### variante `strand`

Une las partículas en una cuerda (en el orden en que nacen) o deja una estela tras cada una. Solo cambia los campos que nombras.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `strand` | texto | (obligatorio) `rope` (cuerda) o `trail` (estela). |
| `layer` | texto | Capa de partículas a la que vale. |
| `settings` | objeto | Campos de la cuerda: `subdivide` (de 0 a 7), `width`, `uv_scale`, `uv_scroll`, `smooth`; de la estela: `length` (pasos, 32 al empezar), `width` (de 0 a 1, 0,5 al empezar), `segments`, `subdivide`, `uv_scale`, `uv_scroll`, `textured`. |
| `off` | sí/no | Si es `true`, la quita. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"strand":{"system":"chispas","strand":"trail","settings":{"length":8}}}'
wprs-ctl scene act mi-escena particles '{"strand":{"system":"chispas","strand":"rope"}}'
wprs-ctl scene act mi-escena particles '{"strand":{"system":"chispas","strand":"trail","off":true}}'
```

##### variante `texture`

Elige la imagen con que se dibuja cada partícula. Sin imagen, son discos suaves.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `layer` | texto | Capa de partículas a la que vale. |
| `texture` | texto | Id de una imagen del paquete. Sin valor, quita la imagen. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"texture":{"system":"chispas","texture":"foto"}}'
```

##### variante `render`

Ajusta cómo se dibujan las partículas. Solo cambia lo que nombras.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `layer` | texto | Capa de partículas a la que vale. |
| `stretch` | cualquier valor JSON | Cuánto se estira cada partícula en la dirección en que se mueve: segundos de movimiento (`0.05`), o `{"time": s, "min": px, "max": px}`. `0` lo quita. |
| `frames` | texto | Cómo avanzan los cuadros de una hoja de imágenes: `time` (con el tiempo), `life` (con la vida) o `random`; `none` lo quita. |
| `fps` | número | Cuadros por segundo con `frames` en `time`. |
| `mix` | sí/no | Si es `true`, funde cada cuadro con el siguiente. |
| `brightness` | número | Brillo, de 0 a 16. |
| `cycles` | entero | Cuántas veces se repiten los cuadros en una vida (de 1 a 64); solo vale con `frames` en `life`. `0` lo quita. |
| `order` | texto | Qué partículas se dibujan encima: `newest`, `oldest`, `largest`, `smallest` o `lowest`; `none` lo quita. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"render":{"system":"chispas","stretch":0.05,"frames":"life","fps":12,"mix":true,"brightness":2,"cycles":2,"order":"newest"}}'
wprs-ctl scene act mi-escena particles '{"render":{"system":"chispas","stretch":0,"frames":"none","cycles":0,"order":"none"}}'
```

##### variante `over`

Hace que el tamaño y el color de cada partícula cambien a lo largo de su vida. Estas curvas se multiplican por el tamaño y el color con que nace.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `layer` | texto | Capa de partículas a la que vale. |
| `size` | lista de listas de números | Claves `[tiempo, escala]`, con el tiempo de 0 a 1 de la vida. Una lista vacía quita la curva. |
| `color` | lista de listas | Paradas `[tiempo, "#color"]`. Una lista vacía quita la curva. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"over":{"system":"chispas","size":[[0,0],[0.1,1],[1,0]],"color":[[0,"#ffffff"],[1,"#ff000000"]]}}'
wprs-ctl scene act mi-escena particles '{"over":{"system":"chispas","size":[]}}'
```

##### variante `scope`

Da a una capa de partículas un dibujo propio, copia del del sistema, o se lo quita para que vuelva a seguir al sistema. Se rechaza para una capa que no dibuja ese sistema.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `layer` | texto | (obligatorio) Id de la capa de partículas. |
| `own` | sí/no | (obligatorio) `true` le da un dibujo propio; `false` lo suelta. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"scope":{"system":"chispas","layer":"chispas","own":true}}'
wprs-ctl scene act mi-escena particles '{"scope":{"system":"chispas","layer":"chispas","own":false}}'
```

##### variante `cutout`

Reescribe la opacidad de la imagen de las partículas: por debajo de `inicio` pasa a 0, por encima de `fin` pasa a `opacidad`, y entre los dos sube en rampa.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `layer` | texto | Capa de partículas a la que vale. |
| `range` | lista de números | `[inicio, fin, opacidad]`, con valores de 0 a 1. Sin valor, quita el recorte. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"cutout":{"system":"chispas","range":[0.2,0.8,1]}}'
```

##### variante `orient`

Elige hacia dónde miran las partículas.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `layer` | texto | Capa de partículas a la que vale. |
| `orient` | texto | `screen` (derechas), `velocity` (hacia donde van) o `fixed` (un ángulo fijo). Sin valor, quita la orientación. |
| `angle` | número | Ángulo en grados, solo con `fixed`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"orient":{"system":"chispas","orient":"velocity"}}'
wprs-ctl scene act mi-escena particles '{"orient":{"system":"chispas","orient":"fixed","angle":45}}'
```

##### variante `perspective`

Da profundidad al sistema, con una distancia focal en píxeles, para usar `depth` y `depth_speed` en `start`.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `focal` | número | Distancia focal en píxeles, de 1 a 100000. Sin valor, usa 800 si no había. |
| `off` | sí/no | Si es `true`, quita la perspectiva y los rangos `depth` y `depth_speed`. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"perspective":{"system":"chispas","focal":600}}'
wprs-ctl scene act mi-escena particles '{"perspective":{"system":"chispas","off":true}}'
```

##### variante `shader`

Dibuja las partículas con un efecto del paquete hecho para partículas.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `layer` | texto | Capa de partículas a la que vale. |
| `shader` | texto | Nombre del efecto de partículas del paquete. Sin valor, lo quita. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"shader":{"system":"chispas"}}'
```

##### variante `lit`

Hace que las luces de la escena iluminen las partículas (hasta 8 luces). Con `normal`, un mapa de normales les da relieve.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `layer` | texto | Capa de partículas a la que vale. |
| `lit` | sí/no | `true` las ilumina; `false` deja de iluminarlas. |
| `normal` | texto | Id de una imagen del paquete que sirve de mapa de normales. Un texto vacío lo quita. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"lit":{"system":"chispas","lit":true,"normal":"foto"}}'
```

##### variante `refract`

Hace que las partículas no se vean a sí mismas, sino que doblen lo que hay detrás, como gotas de agua, según un mapa de normales.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | (obligatorio) Id del sistema. |
| `layer` | texto | Capa de partículas a la que vale. |
| `normal` | texto | Id de una imagen del paquete que sirve de mapa de normales. Obligatorio mientras el sistema no tenga uno. |
| `amount` | número | Cuánto doblan, de 0 a 1. Por defecto 0,3. |
| `off` | sí/no | Si es `true`, quita la refracción. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene act mi-escena particles '{"refract":{"system":"chispas","normal":"foto","amount":0.4}}'
wprs-ctl scene act mi-escena particles '{"refract":{"system":"chispas","off":true}}'
```

### Luces y luz ambiental

#### `light`

Añade, cambia, silencia o quita las luces de la escena. Una luz es una capa de tipo luz: no dibuja nada, pero ilumina las capas que tienen iluminación (véase `lighting`). Una escena admite hasta 12 luces; en teléfonos y equipos antiguos solo iluminan las 8 primeras. En la app, el panel **Luces**, con las asas sobre el lienzo. Cada llamada usa una sola variante.

##### variante `add`

Añade una luz en el centro del lienzo con los valores de su tipo: punto (radio 400, altura 120), foco (radio 900, altura 150, intensidad 2, cono de 25 a 45 grados, inclinación 45), tubo (largo 400, radio 240, altura 60) o direccional (intensidad 0,8, inclinación 50).

| Argumento | Tipo | Qué significa |
|---|---|---|
| `light` | texto | `point` (punto, por defecto), `spot` (foco), `tube` (tubo) o `directional` (direccional). |
| `id` | texto | Id de la capa de luz. Sin valor, usa el nombre del tipo. |
| `into` | texto | Id de un grupo donde entra. |
| `at` | lista de números | Lugar `[x, y]` de la luz dentro de su grupo, en píxeles, y se guarda como la `position` de su transformación. En la lista principal coincide con los píxeles del lienzo; dentro de un grupo que está movido, la luz queda en el lugar del lienzo que resulta de sumarle el del grupo. Sin valor, el centro del lienzo. Para dejar una luz en un punto del lienzo, usa `at` de la variante `set`. |

```sh
wprs-ctl scene act mi-escena light '{"add":{"light":"spot","id":"foco","at":[960,300]}}'
wprs-ctl scene act mi-escena light '{"add":{}}'
```

##### variante `set`

Cambia una luz. Solo cambia lo que nombras. `color`, `intensity`, `radius`, `z`, `tilt`, `at` y `aim` se escriben como una tecla en el segundo `time` si ese valor ya tiene teclas, y se rechazan si el valor sigue un sonido o una propiedad, salvo con `replace`.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa de luz. |
| `light` | texto | Tipo nuevo: `point`, `spot`, `tube` o `directional`. Quita lo que el tipo nuevo no usa (cono, largo, imagen proyectada, aire). |
| `color` | cualquier valor JSON | Color de la luz, `"#rrggbb"` o `[r, g, b, a]`. |
| `intensity` | número | Intensidad, de 0 a 100. |
| `radius` | número | Radio de alcance en píxeles, de 1 a 100000. |
| `z` | número | Altura sobre el lienzo en píxeles, de −100000 a 100000. |
| `cone` | lista de números | Solo focos: `[interior, exterior]` en grados, de 0 a 180; el exterior no puede ser menor que el interior. |
| `length` | número | Solo tubos: largo en píxeles, de 1 a 100000. |
| `tilt` | número | Inclinación hacia el lienzo en grados, de −89 a 89. |
| `at` | lista de números | Punto del lienzo adonde se lleva la luz. |
| `aim` | lista de números | Punto del lienzo hacia el que apunta (focos, tubos y direccionales). |
| `time` | número | Segundo en que se escribe la tecla (0 o más). Por defecto 0. |
| `replace` | sí/no | Si es `true`, reemplaza las teclas por un valor fijo. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena light '{"add":{"light":"spot","id":"foco"}}'
wprs-ctl scene act mi-escena light '{"set":{"layer":"foco","color":"#ffcc88","intensity":3,"radius":700,"z":200,"cone":[20,40],"tilt":30,"at":[500,300],"aim":[960,800]}}'
wprs-ctl scene act mi-escena light '{"set":{"layer":"foco","light":"point"}}'
```

##### variante `remove`

Quita una luz.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa de luz. |

```sh
wprs-ctl scene act mi-escena light '{"add":{"light":"point","id":"punto"}}'
wprs-ctl scene act mi-escena light '{"remove":{"layer":"punto"}}'
```

##### variante `solo`

Hace que la vista previa se ilumine solo con esa luz. No cambia la escena que se publica. Sin `layer`, termina el solo.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la capa de luz. Sin valor, termina el solo. |

```sh
wprs-ctl scene act mi-escena light '{"add":{"light":"point","id":"punto"}}'
wprs-ctl scene act mi-escena light '{"solo":{"layer":"punto"}}'
wprs-ctl scene act mi-escena light '{"solo":{}}'
```

##### variante `mute`

Deja una luz fuera de la vista previa, o la vuelve a poner. No cambia la escena que se publica.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la capa de luz. |
| `muted` | sí/no | `true` la silencia; `false` (por defecto) la vuelve a poner. |

```sh
wprs-ctl scene act mi-escena light '{"add":{"light":"point","id":"punto"}}'
wprs-ctl scene act mi-escena light '{"mute":{"layer":"punto","muted":true}}'
wprs-ctl scene act mi-escena light '{"mute":{"layer":"punto"}}'
```

##### variante `volumetric`

Hace que una luz de punto o de foco ilumine el aire a su alrededor, como niebla con haces de luz. Solo cambia lo que nombras; empieza con densidad 0,2, 16 muestras y haces 0.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de la luz (de punto o de foco). |
| `density` | número | Densidad, de 0 a 1. |
| `samples` | entero | Muestras por píxel, de 4 a 64; más muestras, más coste. |
| `shafts` | número | De 0 a 1: cuánto frenan el brillo las partes oscuras de las capas de debajo, que así dibujan haces. |
| `off` | sí/no | Si es `true`, quita el aire. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena light '{"add":{"light":"spot","id":"foco"}}'
wprs-ctl scene act mi-escena light '{"volumetric":{"layer":"foco","density":0.3,"samples":24,"shafts":0.5}}'
wprs-ctl scene act mi-escena light '{"volumetric":{"layer":"foco","off":true}}'
```

##### variante `projection`

Hace que un foco proyecte una imagen, un vídeo o una capa, como un proyector.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id del foco. |
| `asset` | texto | Id de una imagen o un vídeo del paquete. |
| `from` | texto | Id de una capa que se proyecta; no puede recibir luz ni leer una capa que la reciba. |
| `off` | sí/no | Si es `true`, quita la proyección. Por defecto `false`. |

Hay que dar `asset` o `from`, uno solo.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena light '{"add":{"light":"spot","id":"foco"}}'
wprs-ctl scene act mi-escena light '{"projection":{"layer":"foco","asset":"foto"}}'
wprs-ctl scene act mi-escena light '{"projection":{"layer":"foco","off":true}}'
```

#### `lighting`

Fija cómo recibe la luz una capa de imagen, vídeo, texto, malla, modelo o partículas: relieve, rugosidad, reflejo y luz propia. Una capa sin esto no recibe luz de las capas de luz. En la app, la sección **Iluminación** de la capa. Solo cambia lo que nombras. Las capas de partículas aceptan solo `strength` y `emissive`.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la capa. Obligatorio en la práctica: sin él se rechaza. |
| `normal` | texto | Id de una imagen que sirve de mapa de normales (relieve). Un texto vacío lo quita. |
| `roughness` | número | Rugosidad, de 0 (liso, brillo nítido) a 1 (rugoso, brillo difuso). |
| `roughness_map` | texto | Id de una imagen en gris que multiplica la rugosidad. Un texto vacío la quita. |
| `reflection` | número | Cuánto refleja, de 0 a 1. |
| `env` | texto | Id de una imagen que se refleja como si fuera el entorno. |
| `emissive` | número | Luz propia, de 0 a 16. Se ve incluso a oscuras. |
| `emissive_map` | texto | Id de una imagen en gris que multiplica la luz propia. Un texto vacío la quita. |
| `strength` | número | De 0 a 1: cuánto del color iluminado reemplaza al color normal. |
| `off` | sí/no | Si es `true`, quita toda la iluminación de la capa. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena lighting '{"layer":"image","roughness":0.3,"reflection":0.2,"emissive":1,"strength":0.9}'
wprs-ctl scene act mi-escena lighting '{"layer":"image","env":"foto","reflection":0.5}'
wprs-ctl scene act mi-escena lighting '{"layer":"image","off":true}'
```

#### `normals`

Hace un mapa de normales (un relieve) a partir de los bordes de brillo de una imagen y lo guarda como imagen del paquete. Con `layer`, además lo pone como mapa de normales de esa capa. En la app, **Generar normales…** en la sección **Iluminación**, con **Fuerza** y **Desenfoque**. La pregunta `normals` muestra el resultado antes de guardarlo.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `asset` | texto | Id de una imagen del paquete. |
| `layer` | texto | Id de una capa de imagen (usa su imagen), de vídeo (usa un fotograma) o de texto (usa sus letras); además recibe el mapa. |
| `strength` | número | Fuerza del relieve, de 0 a 16. Por defecto 2. |
| `blur` | número | Desenfoque previo en píxeles, de 0 a 32. Por defecto 1. |
| `id` | texto | Id de la imagen que se crea. Solo vale un id nuevo o el de un mapa hecho antes por esta acción. Sin valor, usa el de la imagen más `_normals`. |
| `time` | número | Con una capa de vídeo, el segundo del fotograma. Por defecto 0. |

Hay que dar `asset` o `layer`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena normals '{"layer":"image","strength":3,"blur":2,"id":"relieve"}'
wprs-ctl scene act mi-escena normals '{"asset":"foto"}'
```

#### `paint_map`

Pinta con un trazo el mapa de rugosidad o el de luz propia de una capa. Si la capa aún no tiene mapa, empieza uno nuevo con el tamaño de su imagen (hasta 512 en el lado largo), blanco para la rugosidad y negro para la luz propia. En la app, **Pintar la rugosidad sobre la vista previa** y **Pintar la luz propia sobre la vista previa**, con **Tamaño del pincel**, **Dureza** y **Gris**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | (obligatorio) Id de una capa de imagen, vídeo, texto, malla o modelo. |
| `map` | texto | (obligatorio) `roughness` (rugosidad) o `emissive` (luz propia). |
| `points` | lista de listas de números | (obligatorio) Puntos del trazo, `[[x, y], ...]`, como fracciones de la capa (de 0 a 1). De 1 a 4096 puntos. |
| `radius` | número | (obligatorio) Radio del pincel como fracción del lado más corto, entre 0,001 y 0,5. |
| `hardness` | número | Dureza del borde, de 0 a 0,99. Por defecto 0,5. |
| `value` | número | Gris que se pinta, de 0 a 1. Por defecto 1. |

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena paint_map '{"layer":"image","map":"roughness","points":[[0.2,0.2],[0.8,0.5]],"radius":0.1,"value":0.2}'
wprs-ctl scene act mi-escena paint_map '{"layer":"image","map":"emissive","points":[[0.5,0.5]],"radius":0.2,"hardness":1}'
```

#### `ambient`

Fija la luz de alrededor de la escena y la del cielo, que se suman a las luces en las capas con iluminación: la luz ambiente cae sobre lo que mira hacia abajo y la del cielo sobre lo que mira hacia arriba. Sin ninguna, es negro. En la app, **Luz ambiente** y **Luz del cielo**. La pregunta `hemisphere` muestra la esfera que forman.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `ambient` | cualquier valor JSON | Color de la luz de alrededor, `"#rrggbb"` o `[r, g, b, a]`. |
| `sky` | cualquier valor JSON | Color de la luz del cielo; si falta, vale como la luz ambiente. |
| `clear` | sí/no | Si es `true`, quita las dos. Por defecto `false`. |

```sh
wprs-ctl scene act mi-escena ambient '{"ambient":"#202040","sky":"#8090ff"}'
wprs-ctl scene act mi-escena ambient '{"clear":true}'
```

## Preguntas

### Partículas

Las dos primeras preguntas son sobre un sistema de partículas. El sistema se da con `--layer ID` al final de la orden (sin él la respuesta es `null`); el argumento `layer` de dentro, si lo hay, es la capa de partículas cuyo dibujo propio se mira.

#### `particles`

Dice todo lo que se ve y se puede tocar de un sistema de partículas en un segundo dado: dónde están sus puntos de control y sus emisores, qué fuerzas y choques tiene, cómo se dibujan, qué datos puede usar una regla `remap` y cómo es la vista previa de la turbulencia. Es lo que dibuja la app en el lienzo al editar un sistema.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Capa de partículas cuyo dibujo propio se mira. Sin valor, la primera que dibuja el sistema. |
| `time` | número | Segundo de la escena. Por defecto 0. |
| `sound` | número | Volumen de la banda que mueve la fase de una turbulencia, de 0 a 1. Por defecto 0. |

La respuesta tiene `points` (puntos de control con su lugar en el lienzo), `emitters` (de cada emisor, la forma con sus medidas en el lienzo y `max_per_step`), `colliders`, `pulls` (atractores, remolinos y órbitas), `keeps`, `boids`, `operators`, `strands` (cuerda y estela), `children`, `remaps` (cada regla con su curva), `inputs` y `steady_inputs` (los datos que una regla puede leer), `outputs` (lo que puede escribir), `domains` (el rango de cada dato), `render`, `scope` (qué capas dibujan el sistema y en qué se diferencian), `cutout`, `orient`, `shaders` (efectos de partículas que se pueden usar), `fields` (la turbulencia y el desplazamiento de nacimiento), `seed` y `sources` (capas de las que podrían nacer). Si el sistema no existe, la respuesta es `null`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene ask mi-escena particles '{"layer":"chispas","time":1.5,"sound":0.5}' --layer chispas
```

#### `budget`

Dice cuánto cuesta un sistema de partículas: simula el sistema hasta un segundo dado en un nivel de calidad y cuenta cuántas partículas hay y cuánto espacio ocupan en cada tipo de equipo. En la app, el panel de coste del sistema.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Capa de partículas cuyo dibujo propio se mira. Sin valor, la primera que dibuja el sistema. |
| `time` | número | Segundo hasta el que se simula, de 0 a 10. Por defecto 0. |
| `quality` | texto | Nivel de calidad: `q0` (el más bajo) a `q3` (el más alto, por defecto). |

La respuesta tiene `capacity`, `alive` (partículas vivas en ese segundo), `slots` (plazas que reserva), `tiers` (por cada tipo de equipo, de `t0` a `t3`, las plazas en la tarjeta gráfica y en el procesador), `levels` (por cada calidad, la parte de partículas que nacen, vecinas de bandada, pasos de estela y subdivisiones de cuerda), `boids`, `particles` (hasta 300 partículas como `[x, y, número]` en píxeles del lienzo), `rope` y `trails`. Si el sistema no existe, la respuesta es `null`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene ask mi-escena budget '{"layer":"chispas","time":1.5,"quality":"q2"}' --layer chispas
```

#### `swarm_light`

Dice cómo les afecta la luz a las partículas de un sistema: si están iluminadas, qué mapa de normales usan, si refractan y qué se queda en cada nivel de calidad. En la app, el panel de luz del sistema de partículas.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `system` | texto | Id del sistema. También se puede dar con `--layer ID`. |
| `layer` | texto | Capa de partículas cuyo dibujo propio se mira. |

La respuesta tiene `lit` (sí/no), `normal`, `refract` (la refracción o `null`), `pictures` (imágenes del paquete que sirven de mapa), `sides` (una imagen incrustada de una esfera vista por delante y por detrás con esa luz), `field`, `size`, `shift` (cuánto desvía la refracción, en píxeles), `levels` (por calidad, cuántas luces y qué refracción) y `phone` (lo que queda en un teléfono). Si el sistema no existe, la respuesta es `null`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"particles","id":"chispas"}'
wprs-ctl scene ask mi-escena swarm_light '{"system":"chispas"}'
```

### Luces y luz ambiental

#### `lights`

Lista las luces de la escena en un segundo dado, con sus asas, la caja que se vuelve a dibujar cuando se mueven y cuántas se quedan en cada tipo de equipo. En la app, el panel **Luces**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `time` | número | Segundo de la escena. Por defecto 0. |

La respuesta tiene `count`, `max` (12), `time`, `solo` y `muted` (el solo y las silenciadas de la vista previa), `tiers` (por cada tipo de equipo, de `t0` a `t3`: `cap` luces que ilumina, `kept` las que se quedan y `left_out` las que sobran) y `lights`. Cada luz tiene `id`, `pointer`, `type`, `at` (su lugar en el lienzo), `aim` (hacia dónde apunta, si apunta), `angle`, `color`, `intensity`, `radius`, `z`, `tilt`, `cone` (en focos), `box` (la caja que se redibuja, `[x, y, ancho, alto]`), `radius_knob`, `z_knob` y `tilt_knob` (dónde están las asas), `texture`, `volumetric`, `muted` y `solo`.

```sh
wprs-ctl scene act mi-escena light '{"add":{"light":"spot","id":"foco"}}'
wprs-ctl scene ask mi-escena lights '{"time":0}'
```

#### `falloff`

Dibuja cómo se apaga una luz con la distancia: la curva de intensidad desde la luz hasta donde acaba su radio. En la app, **Luz según la distancia**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la luz. También se puede dar con `--layer ID`. |
| `time` | número | Segundo de la escena. Por defecto 0. |

La respuesta tiene `curve` (`points`, una lista de `[distancia, intensidad]`, y los límites `x` e `y` para dibujarla), `intensity`, `radius` y `marks`, distancias que conviene marcar (por ejemplo, donde empieza a apagarse el cono). Si la capa no es una luz, la respuesta es `null`.

```sh
wprs-ctl scene act mi-escena light '{"add":{"light":"spot","id":"foco"}}'
wprs-ctl scene ask mi-escena falloff '{"layer":"foco","time":0}'
```

#### `lighting`

Dice cuánto cuesta la iluminación de la escena en un segundo dado y en un nivel de calidad: cuántas luces trabajan, sobre cuántas capas y cuántos píxeles, y cuánto aire y refracción hay. En la app, el resumen del panel **Luces**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `time` | número | Segundo de la escena. Por defecto 0. |
| `quality` | texto | Nivel de calidad: `q0` a `q3`. Por defecto `q3`. |

La respuesta tiene `levels` (una fila por calidad con `lights`, `lit_layers`, `lit_particles`, `shaded` —píxeles iluminados—, `per_frame`, `baked` —`true` si la luz se calcula una sola vez al empezar—, `air`, `air_taps`, `shaft_copy` y `refract`), `lit_layers` y `lit_systems` (qué se ilumina), `animated`, `volumetric` (cuántas luces iluminan el aire), `tiers`, `phone`, `quality` y, para modelos 3D, `models`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena lighting '{"layer":"image","roughness":0.3}'
wprs-ctl scene act mi-escena light '{"add":{"light":"point","id":"punto"}}'
wprs-ctl scene ask mi-escena lighting '{"time":0,"quality":"q2"}'
```

#### `air`

Dice cómo se ve y cuánto cuesta el aire iluminado de una luz de punto o de foco. En la app, **Iluminar el aire**.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id de la luz. También se puede dar con `--layer ID`. |
| `time` | número | Segundo de la escena. Por defecto 0. |

Si la luz tiene aire, la respuesta tiene `on` (`true`), `density`, `samples`, `shafts`, `box` (la zona que cubre), `curve` (cómo se apaga con la distancia), `shaded` (la misma curva con los haces, solo si `shafts` es mayor que 0), `marks`, `levels` (por calidad, si hay aire, a qué fracción del tamaño y con cuántas muestras), `phone` (lo mismo en un teléfono), `taps` (lecturas por fotograma en `q1`, `q2` y `q3`) y `fits`. Sin aire, tiene solo `on` en `false` y `fits`, que dice si ese tipo de luz puede tener aire (solo las de punto y de foco). Si la capa no es una luz, la respuesta es `null`.

```sh
wprs-ctl scene act mi-escena light '{"add":{"light":"point","id":"punto"}}'
wprs-ctl scene act mi-escena light '{"volumetric":{"layer":"punto","density":0.3}}'
wprs-ctl scene ask mi-escena air '{"layer":"punto","time":0}'
```

#### `projection`

Dice qué proyecta un foco y dónde cae la imagen en el lienzo. En la app, la sección **Proyecta** de la luz.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layer` | texto | Id del foco. También se puede dar con `--layer ID`. |
| `time` | número | Segundo de la escena. Por defecto 0. |

La respuesta tiene `current` (lo que proyecta, o `null`), `kind` (`picture`, `video` o `layer`), `landed` (`true` si la imagen cae dentro del lienzo), `quad` (las cuatro esquinas de la imagen proyectada, en píxeles del lienzo), `image` (la imagen incrustada), `spot` (`true` si la luz es un foco) y `pictures`, `videos` y `layers`, lo que se puede proyectar. Si la capa no es una luz, la respuesta es `null`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"image","file":"foto.png"}'
wprs-ctl scene act mi-escena light '{"add":{"light":"spot","id":"foco"}}'
wprs-ctl scene act mi-escena light '{"projection":{"layer":"foco","asset":"foto"}}'
wprs-ctl scene ask mi-escena projection '{"layer":"foco","time":0}'
```

#### `hemisphere`

Muestra la luz ambiente y la del cielo de la escena como una esfera que va de un color al otro. No lleva argumentos. En la app, la esfera junto a **Luz ambiente** y **Luz del cielo**.

La respuesta tiene `ambient` y `sky` (colores como `[r, g, b, a]`, de 0 a 1; negro si no hay), `size` y `sphere`, una imagen incrustada como texto `data:image/png;base64,...`.

```sh
wprs-ctl scene act mi-escena ambient '{"ambient":"#202040","sky":"#8090ff"}'
wprs-ctl scene ask mi-escena hemisphere '{}'
```

### Varias pantallas

#### `spread`

Dice qué parte del lienzo enseñaría cada pantalla si la escena se repartiera por varias pantallas a la vez, con pantallas simuladas. Sirve para comprobar el reparto sin tener las pantallas.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `layout` | texto | (obligatorio) La disposición: `dual` (dos pantallas en fila), `triple` (tres en fila), `vertical` (dos una sobre otra) o `mine` (la tuya, con `monitors`). |
| `bezel` | número | Milímetros de marco de cada pantalla, que separan las imágenes. Por defecto 0. |
| `screen` | texto | Tamaño de las pantallas simuladas: `"ANCHOxALTO"` en milímetros (`"527x296"`) o una diagonal como `"27in"` o `"34in@21:9"`. Por defecto 527 por 296 milímetros. De 50 a 3000 mm por lado. |
| `monitors` | lista de objetos | Con `mine`, las pantallas: `{"id": "nombre", "mm": [x, y, ancho, alto]}`, con la posición y el tamaño en milímetros. Entre 1 y el máximo que admite una disposición. |

La respuesta tiene `spread` (`true` si el paquete pide repartirse por todas las pantallas), `canvas` (el tamaño del lienzo), `screens` (las pantallas usadas), `bounds` (el rectángulo que ocupan todas, en milímetros) y `parts`, una lista de `{id, rect}` con el rectángulo `[x, y, ancho, alto]` del lienzo, en píxeles, que enseña cada pantalla. Si algo no vale, trae `error` y `parts` vacío.

```sh
wprs-ctl scene ask mi-escena spread '{"layout":"dual"}'
wprs-ctl scene ask mi-escena spread '{"layout":"vertical","bezel":10,"screen":"27in@21:9"}'
wprs-ctl scene ask mi-escena spread '{"layout":"mine","monitors":[{"id":"izq","mm":[0,0,600,340]},{"id":"der","mm":[620,0,527,296]}]}'
```

### Sonido

**Cómo es una capa de sonido.** Las capas de sonido se editan con `add_layer` de tipo `sound` y con `set` sobre sus campos; no tienen acciones propias. `add_layer` con `"kind":"sound"` y `"file":"sonido.ogg"` crea la capa, importa el archivo y le da el id `sound`. Sus campos son: `sound` (el id de un sonido del paquete) o `sounds` (una lista de 1 a 16 sonidos; una capa lleva uno de los dos), `loop` (si se repite sin cortes; sí por defecto), `play` (si suena; sí por defecto), `volume` (de 0 a 1), `pan` (de −1 izquierda a 1 derecha), `fade_in` y `fade_out` (segundos de subida y de bajada, de 0 a 60), `mode` (con `sounds`: `sequence` o `random`), `pause` (`[mín, máx]` segundos de silencio entre sonidos, de 0 a 600), `stream` (sí para sonidos largos) y `play_on` (un disparador, como `start`, `beat` o `event:nombre`, que arranca el sonido cada vez que se cumple). `volume`, `pan` y `play` pueden seguir una propiedad de usuario o teclas; no pueden seguir el sonido que se oye (`validate` lo rechaza). Ocultar la capa con `hide` la silencia.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"sound","file":"sonido.ogg","id":"viento"}'
wprs-ctl scene act mi-escena set '{"pointer":"/scene/layers/0/loop","value":false}'
wprs-ctl scene act mi-escena set '{"pointer":"/scene/layers/0/fade_in","value":2}'
wprs-ctl scene act mi-escena set '{"pointer":"/scene/layers/0/volume","value":0.6}'
wprs-ctl scene act mi-escena set '{"pointer":"/scene/layers/0/pan","value":-0.3}'
```

#### `sounds`

Lista las capas de sonido de la escena y los sonidos del paquete. No lleva argumentos. En la app, **Sonidos de la escena**.

La respuesta tiene `layers`, con una fila por capa: `id`, `pointer`, `sounds` (los sonidos que usa), `mode`, `loop`, `stream`, `play_on`, `fade_in`, `fade_out`, `volume` (el número, o `null` si lo mueven teclas o una propiedad) y `muted` (sí/no); y `assets`, los sonidos del paquete con `name`, `path` y `bytes`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"sound","file":"sonido.ogg","id":"viento"}'
wprs-ctl scene act mi-escena set '{"pointer":"/scene/layers/0/play_on","value":"start"}'
wprs-ctl scene ask mi-escena sounds '{}'
```

#### `wave`

Calcula la forma de onda de un sonido del paquete: el nivel más alto de cada tramo, para dibujarlo. Se guarda en una caché y la segunda vez responde al momento.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `asset` | texto | (obligatorio) Id de un sonido del paquete. |
| `buckets` | entero | En cuántos tramos se divide el sonido, de 1 a 2048. Por defecto 256. |

La respuesta tiene `seconds` (la duración), `channels` y `peaks`, una lista de `buckets` números de 0 a 1. Si el sonido no existe o no se puede leer, trae `error`.

```sh
wprs-ctl scene act mi-escena add_layer '{"kind":"sound","file":"sonido.ogg"}'
wprs-ctl scene ask mi-escena wave '{"asset":"sonido","buckets":8}'
```

#### `meter`

Dice a qué reacciona un enlace al sonido y qué valor da con un nivel de sonido dado: qué bandas están marcadas, cómo quedan sus niveles y qué valor sale antes y después de darle forma. Un enlace al sonido es el valor `{"audio": ...}` que se pone en una propiedad de una capa o de un efecto (por ejemplo, la opacidad), y que se enseña como **Seguir el sonido**. El sonido se analiza en 64 bandas por canal, de graves a agudos, y cada nivel va de 0 a 1.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `source` | objeto | (obligatorio) El enlace: `audio` es `"bass"` (graves), `"mid"` (medios), `"treble"` (agudos), `"level"` (volumen de todo), `"beat"` (1 en cada pulso), el número de una banda (`3`) o un rango (`[0, 3]`). Admite `of` (cuántas bandas tiene la rejilla: 16, 32 o 64, por defecto 64), `reduce` (`mean`, el promedio, o `max`, el máximo, para un rango), `channel` (`left`, `right` o el promedio, por defecto), y la forma: `bounds`, `exponent`, `invert`, `scale`, `offset` y `clamp`. El campo `smooth` también es válido en un enlace, pero esta pregunta no lo aplica: para ver su efecto usa `response`. |
| `reading` | objeto | Lo que se oyó: `listening` (sí/no; sin `true`, la respuesta solo trae las bandas marcadas), `raw` (los niveles de las bandas: una lista para el canal izquierdo y otra para el derecho, de hasta 64 números cada una), `loud` (lo mismo, medido en valor absoluto; se usa si la escena fija `audio.gain` en `fixed`), `loudness`, `beat` (sí/no), `silent` (sí/no) y `granted` (sí/no; por defecto `true`). |
| `of` | entero | Cuántas bandas tiene la rejilla de la respuesta: 16, 32 o 64. Por defecto 64. |

La respuesta tiene `granted`, `listening`, `of` y `marked` (`[primera, última]` banda marcada en esa rejilla, o `null` si el enlace es el volumen o el pulso). Con `listening` en `true` añade `bands` (los niveles plegados a `of` bandas), `levels` (`bass`, `mid`, `treble`, `level` y `beat`), `silent`, `raw` (el valor antes de darle forma) y `value` (el valor ya con la forma: `bounds`, `exponent`, `invert`, `scale`, `offset` y `clamp`). Si el enlace está mal escrito o `of` no es 16, 32 ni 64, trae `error`.

```sh
wprs-ctl scene ask mi-escena meter '{"source":{"audio":[0,3],"of":16},"reading":{"listening":true,"raw":[[0.5,0.2,0.1,0.9],[0.5,0.2,0.1,0.9]],"loudness":0.5},"of":16}'
wprs-ctl scene ask mi-escena meter '{"source":{"audio":"bass","scale":2,"clamp":[0,1]},"reading":{"listening":false}}'
```

#### `response`

Dibuja cómo responde un enlace al sonido con `smooth`: sube hasta el máximo y vuelve a cero, y cuánto tarda en asentarse en cada sentido. Sirve para elegir `smooth` sin probarlo a oído.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `smooth` | objeto o número | (obligatorio) `{"up": segundos, "down": segundos}` (cuánto tarda en subir y en bajar, de 0 a 60), o un solo número para los dos sentidos. |

La respuesta tiene `up`, `down`, `settles` (los segundos que tarda en asentarse al subir y al bajar), `rises` y `falls` (los segundos en que el sonido sube y baja de golpe en el dibujo), `duration` y `points`, 96 pares `[segundo, valor]` con la curva de 0 a 1. Si los tiempos no valen, trae `error`.

```sh
wprs-ctl scene ask mi-escena response '{"smooth":{"up":0.05,"down":0.4}}'
wprs-ctl scene ask mi-escena response '{"smooth":0.3}'
```

### Bandas de audio

#### `bands`

Contesta con la división del sonido en bandas: el rango de frecuencias de cada una y qué bandas son los graves, medios y agudos. Sirve para elegir qué bandas escucha una conexión de sonido. En la app, el selector de bandas.

| Argumento | Tipo | Qué significa |
|---|---|---|
| `of` | entero | En cuántas bandas se divide: 16, 32 o 64. Por defecto 16. |

La respuesta tiene `of`, `bands` (cada banda con `index`, `from` y `to` en hercios), `named` (el rango de `bass`, `mid` y `treble`), `levels` (los niveles con nombre) y `pointer` (las direcciones del puntero que se pueden leer). Con `pick` o `direction` en lugar de `of`, contesta con lo que lee esa elección. Si algo no vale, trae `error`.

```sh
wprs-ctl scene ask mi-escena bands '{"of":16}'
```
