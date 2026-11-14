# El editor de escenas: empezar, guardar, deshacer, vista previa y atajos

Esta es la página de entrada del editor de escenas de la aplicación. Explica cómo abrir o crear una escena, cómo es la ventana, cómo guardar y recuperar, cómo deshacer, qué dice la lista de problemas, cómo ver la escena a cada nivel de calidad y cuánto cuesta, cómo crear el paquete y qué teclas hay.

Las demás páginas del editor tratan cada zona a fondo: [lienzo](editor-lienzo.md), [animación](editor-animacion.md), [dibujo](editor-dibujo.md), [efectos](editor-efectos.md), [partículas y sonido](editor-particulas-y-sonido.md), [3D](editor-3d.md) y [propiedades](editor-propiedades.md). Cada cambio del editor es una acción que también se puede lanzar desde la terminal; la lista completa está en [acciones](acciones.md) y el resto de órdenes en [cli](cli.md).

## Qué es una escena y qué toca el editor

Una escena es una carpeta. Dentro hay un archivo con los datos del paquete (título, licencia, créditos, propiedades), un archivo con la escena (capas, efectos, partículas, relojes) y una carpeta `assets` con las imágenes, vídeos, sonidos y modelos que usa. El editor cambia esa carpeta; no hay un formato propio del editor.

Mientras editas, los cambios viven en la memoria del editor. No llegan a los archivos hasta que pulsas **Guardar**. La terminal es distinta: cada orden `scene act` guarda al momento.

## Cómo es la ventana

La ventana se titula **Escena: <título>**. Tiene cinco zonas.

- **Barra de arriba**: **Volver**, el título con su estado (**Guardado** o **Sin guardar ·** con **Guardar ahora**), **Deshacer**, **Rehacer** y, a la derecha, **Ver de fondo**, **Poner de fondo** y **Publicar…**. Los paquetes que no son escena (imagen, vídeo, página web y preajuste) usan la misma barra; **Ver de fondo** no está en un preajuste, que no se puede ver solo.
- **Columna de la izquierda**, con cinco pestañas:
  - **Capas**: la lista de capas y sus herramientas ([lienzo](editor-lienzo.md)).
  - **Recursos**: los archivos de la escena y los scripts ([lienzo](editor-lienzo.md), [scripts](scripts.md)).
  - **Paquete**: las propiedades de usuario y los preajustes ([propiedades](editor-propiedades.md)).
  - **Reproductor**: un reproductor simulado para probar escenas que siguen la música ([particulas y sonido](editor-particulas-y-sonido.md)).
  - **Probar**: cambia los ajustes del paquete solo en la vista previa. Dice: **Prueba los ajustes del paquete en la vista previa. Solo la vista previa los muestra: no se guarda nada y ninguna pantalla cambia.** El botón **Volver a los valores por defecto** los devuelve a como estaban.
- **Centro**: arriba la barra del lienzo, en medio el lienzo con la vista previa y abajo la franja de tiempo ([animación](editor-animacion.md)).
- **Columna de la derecha**: arriba el inspector, que muestra los campos de lo que has elegido, y abajo la lista de problemas.
- **Avisos**, sobre todo lo demás: un mensaje rojo con el error del editor si algo falla, un mensaje verde con el resultado de lo último que hiciste (con una cruz para cerrarlo) y el aviso de recuperación (mira [Cómo guardar y recuperar](#cómo-guardar-y-recuperar-los-cambios)).

### La barra del lienzo

Casi todos sus botones muestran solo el dibujo; el nombre sale al pasar el ratón. De izquierda a derecha:

- **Reducir**, el porcentaje de zoom, **Ampliar**, **Ajustar** (vuelve al 100 % y centra) y **Probar el puntero y los clics** (un interruptor; mira [Cómo ver la escena a cada nivel de calidad](#cómo-ver-la-escena-a-cada-nivel-de-calidad)).
- **Guía:** un desplegable con **Sin guía** y las proporciones 16:9, 21:9, 32:9, 4:3, 16:10 y 9:16. Dibuja sobre el lienzo el recuadro de esa proporción.
- **Lienzo y reloj**, **Cámara 2D**, **Probar el paralaje**, **Imanes**, **Ajustar al lienzo**, **Seguir a una capa** y **Dejar de seguir** ([lienzo](editor-lienzo.md)).
- **Nivel de la vista previa** (mira más abajo).
- **Escribir sobre las claves**: un interruptor. Apagado, las asas del lienzo añaden una clave en el cabezal a los valores animados y no tocan los valores enlazados. Encendido, escriben un valor fijo encima de las claves y los enlaces.
- **Pantallas:** un desplegable para ver la escena repartida en varias pantallas (**Una pantalla**, **Dos en fila**, **Tres en fila**, **Dos apiladas**, **Mis pantallas**). Con más de una pantalla sale la casilla **Extenderse por todas las pantallas**, que guarda ese ajuste en el paquete.

Con el ratón en el lienzo: la rueda desplaza la vista, **Ctrl** y la rueda acercan o alejan, y arrastrar con el botón central o el derecho mueve la vista.

Si el lienzo no muestra la escena, lee lo que dice. **La vista previa aparece aquí mientras wprsd está en marcha.** significa que falta el demonio. **Instala el módulo QML de KPipeWire para ver aquí la vista previa, o usa Ver en el escritorio.** significa que a tu sistema le falta ese módulo.

## Cómo abrir una carpeta de escena

Dónde está: la página **Escenas** del menú de la aplicación, sección **Abrir**.

Pasos:

1. Pulsa **Abrir una carpeta de escena o paquete…** y elige la carpeta.
2. O pulsa una de las carpetas que ya tienes. Esa lista muestra «título — carpeta» de cada escena guardada en la carpeta `wallpaper-rs` de tus Documentos (o de tu carpeta personal si no tienes Documentos).
3. La aplicación mira qué hay dentro. Una escena o un paquete de recursos se abre en este editor. Una página web, un preajuste, una imagen o un vídeo se abren en la página del paquete ([paquete](paquete.md)).

Si la carpeta tiene cambios sin guardar de una sesión anterior, el editor lo avisa al abrirla (mira [Cómo guardar y recuperar](#cómo-guardar-y-recuperar-los-cambios)).

En la terminal: `wprs-ctl scene open DIR` describe la carpeta. Todas las órdenes `scene` toman la carpeta como primer argumento. Mira [cli](cli.md).

## Cómo crear una escena

Dónde está: la página **Escenas**, arriba.

Pasos:

1. Escribe el **Título:**. Viene con **Mi escena**.
2. Mira la **Carpeta:**. Se rellena sola con el primer nombre libre dentro de la carpeta `wallpaper-rs` de tus Documentos, sacado del título (`mi-escena`, y si está ocupado `mi-escena-2`, y así). Puedes escribir otra ruta. La carpeta tiene que estar vacía o no existir; si no, el editor dice «… no es una carpeta vacía».
3. Pulsa el punto de partida que quieras. La escena se crea y se abre en el editor.

El id del paquete sale de la carpeta (`org.example.` y el nombre de la carpeta sin acentos). Lo puedes cambiar después en la pestaña **Paquete**.

| Punto de partida | Qué trae | Se abre en |
|---|---|---|
| **Vacía** | Un lienzo sin capas. | Este editor |
| **Partículas** | Un cielo con degradado y nieve que cae, con una propiedad **Densidad** para cambiar cuánta cae. | Este editor |
| **Paralaje por capas** | Un cielo, un sol y tres colinas que se mueven distinto con el puntero. | Este editor |
| **Reloj de texto** | Un cielo y dos textos con las horas y minutos y los segundos, con una propiedad **Color**. | Este editor |
| **Paquete de recursos** | Un paquete sin escena que solo guarda partes para compartir (capas, partículas, scripts, efectos). Se edita como una escena. | Este editor |
| **Página web** | Una página web como fondo. | La página del paquete ([paquete](paquete.md)) |
| **Preajuste** | Los valores de otro paquete que otro paquete toma como punto de partida. | La página del paquete |
| **Imagen** | Un paquete de una imagen. | La página del paquete |
| **Vídeo** | Un paquete de un vídeo. | La página del paquete |

El idioma del título es el de tu escritorio; si tu idioma no tiene las etiquetas de las propiedades de ejemplo, se copian las inglesas.

En la terminal: `wprs-ctl scene new DIR --from PUNTO [--title TÍTULO] [--id ID] [--locale IDIOMA]`. Los puntos de partida se llaman `empty`, `particles`, `parallax`, `clock`, `assets`, `web`, `preset`, `image` y `video`. Mira [cli](cli.md).

## Cómo editar una copia de una escena instalada

Una escena instalada no se cambia nunca. Se hace una copia en una carpeta tuya y se edita esa.

Dónde está: dos sitios.

- En la ficha de cualquier elemento de la biblioteca: elige el elemento, pulsa **Detalles** y baja hasta el final del panel; ahí está el botón **Editar una copia…**. Es el camino que siempre está.
- En la página **Escenas**, bajo **Editar una copia de un paquete de la biblioteca**: una fila por paquete. Esa sección solo sale cuando la biblioteca tiene paquetes guardados como archivo `.wpak` (los de ejemplo, los instalados de un repositorio y los que importas como paquete; una imagen o un vídeo sueltos no cuentan), y si no la ves, usa la ficha.

Pasos:

1. Elige el paquete. Se abre el diálogo **Editar una copia**. Explica que la copia se desempaqueta en una carpeta propia con un id propio, que el paquete instalado no cambia, que conserva su licencia y sus créditos y que cita al original como la obra de la que viene.
2. Mira la línea **Paquete:** (título, tipo y versión).
3. En **Carpeta:** deja la que propone o pulsa **Elegir…**.
4. En **Id de la copia:** deja el que propone (el id del original con `.copy` detrás) o escribe otro.
5. Si la licencia pide algo, el diálogo lo dice: **La licencia … pide: …**.
6. Si la licencia no permite obras derivadas, sale un aviso: **La licencia … no permite obras derivadas. Edita una copia solo si hiciste este paquete o si su autor lo permite.** Marca **Hice este paquete o su autor permite una copia** solo si es verdad. Sin la marca el botón no se activa.
7. Pulsa **Editar una copia**. El editor se abre con la copia.

La copia pierde la firma del autor original, porque ya no es su paquete.

En la terminal: `wprs-ctl scene open DIR --from ARCHIVO.wpak [--id ID] [--mine]`. Con `--facts` solo muestra lo que permite la licencia y el id de la copia, sin desempaquetar.

## Cómo guardar y recuperar los cambios

Dónde está: **Guardar** en la barra de arriba.

Pasos:

1. Pulsa **Guardar**. El botón está apagado mientras no haya cambios o el editor esté ocupado.
2. Si algún archivo de la carpeta cambió desde que la abriste (por ejemplo, lo tocó otro programa), el editor no lo pisa: sale el aviso **Otro programa cambió esta carpeta mientras editabas. Tus cambios no se han perdido: elige con qué quedarte.** con tres botones: **Quedarme con lo mío** (guarda tu versión encima), **Tomar lo del otro programa** (descarta lo tuyo y carga la carpeta como está) y **Guardar en otra carpeta…** (guarda lo tuyo aparte y deja la carpeta como la dejó el otro programa).

El editor mira la carpeta cada dos segundos: si el cambio de fuera ocurre cuando no tienes nada sin guardar, recarga solo la escena y la lista de problemas, sin preguntar.

Atajo: el de guardar de tu sistema (mira la [tabla de atajos](#todos-los-atajos-de-teclado)).

Qué pasa con lo que no guardas:

- Cada 30 segundos, al pulsar **Volver** y al cerrar el editor, los cambios sin guardar se apartan en un archivo del editor, fuera de tu carpeta, junto con el historial de deshacer.
- Al abrir la carpeta otra vez, si esos cambios siguen ahí y parten de los mismos archivos, sale el aviso amarillo **Esta carpeta tiene cambios sin guardar de una sesión anterior.** con dos botones: **Recuperar** los trae de vuelta (con su historial) y **Descartar** los borra.
- Ese archivo desaparece cuando cierras el editor con todo guardado.

En la terminal: no hay botón de guardar porque cada acción guarda al momento. Los cambios sin guardar que apartó la aplicación se recuperan con `wprs-ctl scene recover DIR`, o se descartan con `--discard`.

## Cómo deshacer y rehacer

Dónde está: **Deshacer** y **Rehacer** en la barra de arriba. Cada uno se apaga cuando no hay nada que deshacer o rehacer.

Pasos:

1. Pulsa **Deshacer**. Al pasar el ratón, la ayuda dice qué paso se va a deshacer: **Deshacer: <paso>**.
2. Pulsa **Rehacer** para volver a aplicar el paso. La ayuda dice **Rehacer: <paso>**.

Cómo se cuentan los pasos:

- Arrastrar un deslizador o un asa del lienzo es un solo paso, por largo que sea el arrastre.
- Un valor escrito, mover una capa de sitio, añadir o quitar algo son un paso cada uno.
- Si deshaces y haces un cambio distinto, los pasos deshechos se pierden.
- El historial guarda como mucho 500 pasos.
- Un cambio que no cambia nada (por ejemplo, volver a poner el mismo valor) no deja paso.

Atajo: los de deshacer y rehacer de tu sistema (mira la [tabla de atajos](#todos-los-atajos-de-teclado)).

Lista del historial: el editor no tiene una lista de pasos en pantalla. Lo más parecido es la ayuda de los dos botones, que nombra el paso siguiente.

En la terminal: las acciones `undo` y `redo` (`wprs-ctl scene act DIR undo '{}'`) y la orden `wprs-ctl scene history DIR`, que lista los pasos que recorren deshacer y rehacer y marca los deshechos. El historial es el mismo en la aplicación y en la terminal mientras haya cambios sin guardar. Mira [acciones](acciones.md).

## Cómo leer la lista de problemas

Dónde está: abajo de la columna de la derecha. El título dice **Problemas (N)** con el número, o **Sin problemas** y entonces la lista queda de dos líneas.

Cada fila es un problema, con un icono y el mensaje en tu idioma.

- Icono de error (rojo): hay que arreglarlo. Mientras haya uno, no se puede crear el paquete.
- Icono de aviso (amarillo): no impide nada, pero conviene mirarlo.

Qué dicen los avisos y errores:

- Valores de la escena que no valen, o que apuntan a algo que no existe (una capa, un archivo, una línea de tiempo, una propiedad).
- Datos del paquete que faltan o están mal: título, licencia, autores, créditos, textos del paquete que no se encuentran.
- Qué plataformas rechazan el paquete o lo reproducen sin alguna función, con el formato «<plataforma>: rechazado: …» o «<plataforma>: se reproduce sin …».
- Archivos de `assets` que ninguna parte de la escena usa, o una firma que dejó una compilación anterior.
- Una sugerencia sobre destellos, si la escena parpadea mucho.
- Un script que usa algo (sonido, puntero…) sin declararlo: **… usa … pero no lo declara; decláralo en sus usos**.
- Un script que no compila (un error de sintaxis, con su archivo y su línea) o que `scene.json` nombra y no está en la carpeta. El editor compila los scripts sin ejecutarlos, como `wprs-ctl scene validate`.

Pasos:

1. Pulsa una fila. El editor elige la capa, la propiedad o el preajuste donde está el fallo y marca el campo con una línea roja en el inspector.
2. Arregla el valor.
3. Algunas filas tienen un botón **Corregir** a la derecha. Pulsarlo hace el arreglo por ti (por ejemplo, declarar lo que le falta a un script) y cuenta como un paso que se puede deshacer.

Bajo la lista salen más avisos mientras la vista previa funciona:

- Un mensaje rojo con el error de un script que se paró.
- **Un script quiere … pero no está concedido aquí, así que esa parte de la escena está apagada mientras se prueba.**
- **Consola de los scripts**, con lo que escriben los scripts con `console.log` (las líneas de error salen en rojo).

En la terminal: `wprs-ctl scene validate DIR` lista los mismos problemas con el puntero del campo y sale con 1 si hay algo mal. Con `--json` da un objeto por problema. `wprs-ctl scene preview DIR --console` muestra la consola de los scripts. Mira [cli](cli.md).

## Cómo ver la escena en la vista previa y en el escritorio

Dónde está: el lienzo y **Ver en el escritorio** en la barra de arriba.

El lienzo muestra la escena dibujada por el demonio, tal como se vería de fondo. Se redibuja con cada cambio, aunque no lo hayas guardado. El tiempo del cabezal decide qué segundo se ve; si la franja de tiempo está en pausa, la imagen se queda quieta en ese segundo.

Pasos para verla en tus pantallas de verdad:

1. Pulsa **Ver en el escritorio**. Se enciende el interruptor y todas tus pantallas muestran la escena sin aplicarla; sobre el lienzo sale el aviso **Esta escena se muestra en el escritorio como vista previa y no está aplicada. Detén la vista previa cuando termines.**, con el botón **Detener la vista previa**.
2. Vuelve a pulsarlo (o usa ese botón) para apagarla. También se apaga al cerrar el editor, y si la vista previa se termina desde fuera (`wprs-ctl scene preview DIR --stop`, otra escena en la pantalla), el editor lo dice y apaga el interruptor.

Pasos para probar el ratón:

1. Pulsa **Probar el puntero y los clics** en la barra del lienzo.
2. Mueve el ratón sobre el lienzo y pulsa. La escena recibe un puntero simulado: se ve un anillo donde está, rojo mientras pulsas, y cada clic lanza un pulso desde donde cayó. Así ves hasta dónde llega un clic.
3. Apaga el interruptor para volver a editar con el ratón.

En la terminal: `wprs-ctl scene preview DIR` muestra la carpeta guardada durante 60 segundos (`--stop` la termina, `-o` elige la salida). Necesita el demonio. Para una imagen sin escritorio, `wprs-ctl scene render`.

## Cómo ver la escena a cada nivel de calidad

El demonio dibuja la escena a uno de cuatro niveles: **Q0, la más baja**, **Q1**, **Q2** y **Q3, la más alta**. Un nivel bajo dibuja más pequeño y deja fuera lo que pesa más; así la escena sigue fluida en un equipo flojo o con batería. El editor te deja ver cómo queda en cada uno.

Dónde está: el botón **Nivel de la vista previa** de la barra del lienzo. Abre un panel. Se cierra con **Escape**.

Pasos:

1. Abre el panel.
2. En **Calidad de la vista previa** elige el nivel. La vista previa se redibuja al momento. **Reiniciar** vuelve a empezar la escena y las partículas desde el principio.
3. Marca **Con batería** para verla como la dibuja el demonio con batería: baja Q3 a Q2 y dibuja como mucho 60 cuadros por segundo.
4. Marca **Menos movimiento** para verla con el movimiento reducido: el paralaje y el temblor de la cámara se quedan quietos.
5. Lee el resumen que sale bajo las casillas: **Dibujada como Q2 al … % del tamaño, con … efectos dejados fuera**. Si la escena tiene brillo por encima del blanco, dice si ese nivel lo dibuja. Si hay efectos o partículas que siguen al puntero, los nombra para que los pruebes con **Probar el puntero y los clics**.
6. Bajo el resumen hay una línea por cada efecto: **<efecto> en <capa>: <n> unidades de trabajo** o **… se deja fuera de la escena en este nivel**. Los que se dejan fuera salen en negrita.

Para comparar dos niveles, o con y sin brillo:

1. En **Comparar con** elige el segundo nivel. Marca **Sin el brillo del lienzo** para comparar el mismo nivel sin el brillo.
2. Mantén pulsado **Mantener para ver el otro lado**. Mientras lo mantienes, la vista previa enseña el segundo aspecto; al soltar, vuelve al primero.

Para ver el brillo partido en vivo sobre el lienzo, pulsa **Comparar el brillo** en la barra del lienzo (solo aparece si la escena tiene brillo). El lienzo se parte con una línea: a la izquierda se ve con el brillo y a la derecha sin él. Arrastra el asa de la línea para mover el corte. Pulsa el botón otra vez para volver a la vista normal. En la terminal: `wprs-ctl scene preview DIR --without-bloom`.

Para guardar lo que ves:

1. Marca, si quieres, **Mover un puntero por el lienzo y hacer clic** (un puntero simulado cruza el lienzo y pulsa hacia la mitad del clip, para que lo que sigue al puntero se mueva también en el archivo) y **Partir la imagen con el otro lado** (la izquierda con el nivel de arriba y la derecha con el de **Comparar con**).
2. Pulsa **Guardar una imagen…** (un cuadro del segundo donde esté el cabezal; PNG, JPG o WebP) o **Guardar un clip…** (90 cuadros a 30 por segundo; un GIF animado o, con otra extensión, una secuencia de imágenes).
3. Debajo sale **Guardado: <archivo>**.

Marca **Fijar la hora que marca el reloj** y elige la hora y los minutos para que los relojes y los scripts vean esa hora en el archivo (sin marcarlo, la de tu equipo). Si el reproductor simulado de la vista previa está encendido, el archivo lo muestra con su carátula. Todo esto se dibuja desde lo que tienes en el editor, guardado o no, con lo mismo que ves en la vista previa: la capa en solitario, la semilla de las partículas y los scripts.

En la terminal: `wprs-ctl scene preview DIR --quality Q [--battery] [--calm] [--without-bloom] [--restart]` y `wprs-ctl scene render DIR --out ARCHIVO [--time-of-day HORA] [--media PISTA.json] [--quality Q] [--battery] [--calm] [--frames N] [--pointer X,Y[:X,Y]] [--click] [--against Q] [--without-bloom] [--split F]`. Mira [cli](cli.md).

## Cómo ver cuánto cuesta algo

El coste se mide en «unidades de trabajo»: cuanto más alto, más pesa. Se ve en cuatro sitios.

**Un efecto de una capa.** Dónde está: el inspector, al elegir el efecto de la capa, bajo el título **Coste**.

1. Elige el nivel en el desplegable de calidad de ese bloque (**Calidad de la vista previa**). Es el mismo desplegable que el del panel de nivel, así que los dos cambian a la vez.
2. Mira las cuatro filas: **Q3: 1.25 unidades de trabajo**, o **Q0: se deja fuera de la escena en este nivel** si ese nivel descarta el efecto.
3. La fila en negrita es el nivel que el demonio alcanzaría de verdad. Con **Con batería** marcado y Q3 pedido, es Q2.

**Todos los efectos a la vez**: la lista del panel **Nivel de la vista previa**, explicada arriba.

**Modelos 3D y luces.** Dónde está: el panel de luces del inspector, bloque **Modelos 3D** (solo sale si la escena tiene modelos). Por cada nivel dice los triángulos, los grupos 3D, las unidades de trabajo y si hay reflejos. Por cada clase de equipo dice si el modelo **cabe**, **muestra su póster** (una imagen fija en su lugar) o es **demasiado pesado**, con la memoria de texturas. Más detalle en [3D](editor-3d.md).

**Partículas.** Dónde está: las herramientas de un sistema de partículas, bloque **Presupuesto de partículas**. Dice cuántas partículas hay vivas en la vista previa, dónde se calculan (tarjeta gráfica o procesador) y, con **Simular en el procesador y mostrar las partículas sobre la vista previa**, una estimación del editor cuando no hay vista previa en marcha. **Reiniciar los sistemas** vuelve a empezar. Más detalle en [partículas y sonido](editor-particulas-y-sonido.md).

En la terminal: `wprs-ctl scene ask DIR effect_cost`, `cost3d` y `budget`; `wprs-ctl scene catalog DIR` lista todos los efectos con su coste. Mira [acciones](acciones.md).

## Cómo crear el paquete y aplicarlo

Dónde está: **Publicar…** en la barra de arriba, en una escena y en cualquier otro paquete.

El diálogo se llama **Compilar y aplicar**. Mientras el editor trabaja, haya cambios sin guardar o errores en la lista de problemas, los botones están apagados y el diálogo dice por qué: **Espera: el editor sigue trabajando.**, **Tienes cambios sin guardar. Guárdalos para crear el paquete o aplicarlo.** (con un botón **Guardar ahora**) o **La escena tiene N problemas que resolver antes; están en el panel de Problemas.** Los avisos amarillos no lo impiden.

Pasos:

1. Guarda y deja sin errores la lista de problemas.
2. Abre **Publicar…**.
3. Pulsa **Crear el paquete**. El archivo `.wpak` queda junto a la carpeta, con el id y la versión como nombre, o donde pongas en **Archivo del paquete:**. El aviso verde dice dónde quedó y su huella.
4. Para firmarlo, elige la **Clave de autor:** (**Elegir…** abre un archivo; **Claves…** gestiona tus claves) y escribe la **Contraseña de la clave:** si la tiene.
5. Para verlo en una pantalla, elige **Pantalla:** (**Todas las pantallas** o una) y pulsa **Aplicar a la pantalla**. Hace el paquete, lo mete en la biblioteca y lo muestra, y el aviso verde dice qué mostró, en qué pantalla y a qué nivel de calidad (como `wprs-ctl scene apply`). Si cambias la escena y vuelves a aplicar, la pantalla muestra lo nuevo aunque no subas la versión. Para quitar el fondo después, **Quitar el fondo** en la fila de la pantalla, en la página **Ajustes**.
6. **Revisar el paquete…** (en **Antes de construir:**) muestra todo lo que el paquete dirá al mundo antes de crearlo, y **Añadir a un repositorio…** (en **Publicar:**) lo prepara para publicarlo.

Todo esto se explica en [paquete](paquete.md).

En la terminal: `wprs-ctl scene build DIR [-o ARCHIVO] [--key CLAVE]` y `wprs-ctl scene apply DIR [-o SALIDA]`. La primera se niega si hay errores (también un script que no compila), y `scene apply` imprime lo mismo que el aviso verde. Para quitar el fondo: `wprs-ctl clear [-o SALIDA]` y `wprs-ctl restore`.

## El editor de imágenes

Las imágenes de la biblioteca tienen su propio editor, distinto del de escenas. Cambia la imagen sin tocar su archivo: guarda las ediciones aparte.

Dónde está: la ficha de una imagen de la biblioteca, botón **Editar…**. La ventana se titula **Editar <imagen>**.

Barra de arriba: **Volver**, **Guardar**, **Deshacer**, **Rehacer**, **Comparar con el original** (un interruptor), **Ver en el escritorio**, **Exportar…** (guarda la imagen editada como PNG, JPEG, WebP o AVIF) y **Restablecer todo**.

Herramientas de la columna de la izquierda: **Recortar y girar**, **Pantallas**, **Luz y color**, **Curvas**, **Efectos**, **Máscaras**, **Profundidad** y **Paralaje**.

En la terminal: el grupo `wprs-ctl edit` (`edit show`, `set`, `crop`, `curve`, `depth`, `parallax`, `undo`, `redo`, `export`…). Mira [cli](cli.md).

## Todos los atajos de teclado

En «Estándar» se usa la combinación que tu sistema da a esa acción, normalmente **Ctrl+Z** para deshacer, **Ctrl+Mayús+Z** o **Ctrl+Y** para rehacer, **Ctrl+S** para guardar, **Ctrl+C**, **Ctrl+V** y **Supr**.

| Atajo | Qué hace | Dónde funciona |
|---|---|---|
| Deshacer (estándar) | Deshace el último paso | Ventana del editor de escenas; ventana del paquete ([paquete](paquete.md)); editor de imágenes |
| Rehacer (estándar) | Rehace el paso deshecho | Los mismos tres sitios |
| Guardar (estándar) | Guarda la escena (solo si hay cambios) | Ventana del editor de escenas; ventana del paquete |
| Guardar (estándar) | Guarda el script que escribes | Ventana del código de un script (en la pestaña **Recursos**) |
| Guardar (estándar) | Guarda el código del efecto que escribes | Ventana del código de un efecto o de un material (**Guardar**) |
| Flechas | Mueve la capa elegida 1 píxel | El lienzo del editor de escenas, cuando tiene el foco |
| **Mayús** y flechas | Mueve la capa elegida 10 píxeles | El lienzo del editor de escenas |
| **Escape** | Cancela **Seguir a una capa** mientras esperas que toques la capa | El lienzo del editor de escenas |
| **Escape** | Cierra el panel de nivel | **Nivel de la vista previa** |
| Copiar (estándar) | Copia las claves elegidas | **Editor de curvas** (con claves elegidas) |
| Pegar (estándar) | Pega las claves copiadas | **Editor de curvas** |
| **Supr** | Quita las claves elegidas | **Editor de curvas** (con claves elegidas) |
| **F** | Encuadra las claves elegidas | **Editor de curvas** (con claves elegidas) |
| **Inicio** | Encuadra todas las claves | **Editor de curvas** |
| **\\** (barra invertida) | Enciende o apaga **Comparar con el original** | Editor de imágenes |

El lienzo toma el foco del teclado cuando eliges una capa en la lista, cuando tocas o arrastras una capa y cuando tocas el lienzo vacío; por eso las flechas funcionan enseguida. Cada pulsación de una flecha es un paso de deshacer.

Las teclas que cambian cómo funciona el ratón:

| Combinación | Qué hace | Dónde |
|---|---|---|
| **Ctrl** y rueda | Acerca o aleja el lienzo | Lienzo |
| Rueda | Desplaza la vista | Lienzo |
| Botón central o derecho y arrastrar | Mueve la vista | Lienzo |
| **Ctrl** y clic | Suma la capa a las elegidas (para agrupar o copiar) | Lista de capas |
| **Alt** y arrastrar | Coloca la capa sin que se pegue a los imanes | Lienzo |
| **Mayús** y arrastrar una esquina | Escala manteniendo las proporciones | Asas de una capa |
| **Ctrl** y arrastrar el medio de un borde | Sesga la capa a lo largo de ese borde | Asas de una capa |
| **Mayús** y arrastrar un extremo | Mueve la línea entera | Asas de un efecto con dos puntos |
| **Mayús** y arrastrar una esquina de una forma | Cambia el tamaño manteniendo las proporciones | Asas de una forma |
| **Ctrl** y clic en una clave | Suma la clave a las elegidas | **Editor de curvas** |
| **Alt** y arrastrar una clave | Escala los valores de las claves elegidas | **Editor de curvas** |
| **Mayús** y rueda | Desplaza el eje del tiempo | **Editor de curvas** |
| **Ctrl** al empezar a arrastrar una clave de un clip | La copia en vez de moverla | Franja de un clip ([animación](editor-animacion.md)) |
| **Ctrl** y arrastrar un asa | Mueve a lo largo de la línea de visión | Asas 3D de cámaras, luces y modelos ([3D](editor-3d.md)) |
| **Mayús** y arrastrar un asa de un modelo | Reemplaza un valor enlazado por un valor fijo | Asas de un modelo 3D |
| Botón derecho y arrastrar | Gira la vista | Vista 3D |
| Botón central o **Mayús** y arrastrar | Desliza la vista | Vista 3D |
| Rueda | Se acerca | Vista 3D |
