# Biblioteca de la app de escritorio

La Biblioteca muestra los fondos guardados sobre la imagen del fondo en uso.

## Estante

- Una fila de tapas abajo, sobre la imagen del fondo en uso; la tapa con foco está encendida y bajo ella
  aparece su ficha: nombre, pie (tipo y tamaño) y las acciones.
- «Poner de fondo» (Intro) lo pone enseguida, sin diálogo. Con dos o más pantallas dice «Poner de fondo
  en …» y lo pone en la pantalla elegida (la primera, mientras no haya otra elección); un fondo que se
  extiende por todas las pantallas se pone extendido. Un aviso dice dónde quedó.
- «Opciones ›» abre el panel del fondo.
- La tapa del fondo en uso lleva «En uso»; con dos o más pantallas lleva «En …» con el nombre de cada
  pantalla donde está.
- Una nota en la tapa avisa de un archivo que falta o está dañado, de un fondo retirado por su repositorio
  o de uno con destellos (apagado hasta que el usuario lo permita).
- La barra de arriba filtra por Todos, Escenas, Imágenes, Vídeos y Favoritos; «Más» abre Paquetes,
  Recursos, Web, las etiquetas y «Renombrar la etiqueta…». El número de fondos se ve junto a la palabra
  elegida. El buscador acepta `tag:`, `kind:`, `fav:` y `sort:`.
- «Añadir fondos…» abre el selector de archivos y después el panel «Añadir N fondos», que pregunta si
  copiarlos a la biblioteca o dejarlos donde están. «Escribir las rutas…» cambia la lista por un campo
  con una ruta por línea. Al terminar, el aviso «Se añadieron N fondos.» con «Ver» vuelve al estante en
  Todos, donde los nuevos quedan primero. Soltar archivos sobre la biblioteca los añade sin copiarlos,
  con el mismo aviso.
- El velo de abajo sube hasta detrás de la barra, para que las palabras se lean también sobre un fondo
  claro; su opacidad sigue los tonos que la app saca de la imagen.
- Teclas: ← → para moverse, ↓ o «Ver todos» muestra la rejilla, Mayús con una flecha (o Mayús+clic)
  elige los fondos entre el anterior y el nuevo, Ctrl+A elige todos, Esc deja de elegir.

## Ver todos

La misma colección en una rejilla con un velo parejo sobre la imagen; la ficha del fondo con foco queda
fija abajo. Esc o «Ver el estante» vuelve al estante.

## Varios elegidos

Ctrl+clic o Espacio (al elegir) eligen fondos. En lugar de la ficha aparece la línea de la selección:
«N elegidos» («1 elegido» con uno), «Rotar estos cada…», «Lista», «Etiquetas», «Más» (favorito,
valoración, permitir los destellos, enviar los archivos a los otros equipos, quitar) y «Dejar de elegir»
(Esc). Cada acción abre un panel chico junto a su botón: «Rotar N fondos» con 10 min, 30 min, 1 h, Cada
día, «Otro…» (segundos, minutos u horas) y «Usar una carpeta en su lugar…»; «Lista» con las listas
hechas a mano y «Una lista nueva…»; «Etiquetas» con las que se añaden y las que se quitan; y las
preguntas de valorar, permitir los destellos y quitar (con «Eliminar también las copias importadas»).
Con varias pantallas la rotación arranca en la pantalla elegida.

## Archivo dañado o que falta

La ficha dice en Coral que no se pudo abrir el archivo y ofrece «Mostrar en la carpeta» (Intro) y
«Quitar de la biblioteca…», que pregunta antes de quitar; los detalles muestran el error.

## Sin resultados y fallos

Una búsqueda sin resultados dice «Nada coincide con «…»» y ofrece «Quitar los filtros».

## Primer arranque

Con la biblioteca vacía la página no muestra la barra de filtros: dice «Todavía no tienes fondos» y ofrece
tres caminos, «Explorar repositorios» (principal), «Añadir una fuente» y «Añadir una carpeta», con los
ejemplos incluidos (los del repositorio `bundled`) en una fila debajo; pulsar uno lo instala y lo pone de
fondo. Soltar archivos o una carpeta sobre la página muestra «Suelta para añadirlos a tu biblioteca».

El primer fondo se pone solo: cuando la biblioteca está vacía, ninguna pantalla muestra un fondo y el
repositorio de ejemplos ya está, la app instala el primero que no pide nada (ni entradas ni permisos) y
no parpadea, lo aplica a todas las pantallas con `Apply` y lo dice en un aviso con «Deshacer». Deshacer
llama a `ClearWallpaper` y deja el escritorio como estaba. Sucede una sola vez: el archivo
`wallpaper-rs/first-run` de la carpeta de configuración lo recuerda, aunque la biblioteca se vacíe después.
Si el repositorio de ejemplos no está (instalación sin ejemplos) solo se ven los tres caminos.

Si el servicio no responde, la página muestra solo el fallo, «El servicio de fondos no responde», con
«Reiniciar el servicio» y los detalles; reintenta cada tres segundos y vuelve sola cuando el servicio
contesta. El botón pide a systemd de la sesión que reinicie `wprsd.service` (como el applet de la bandeja);
sin esa unidad pide al bus que active `io.github.wallpaper_rs.Daemon`. Si ninguna de las dos funciona, el
texto lo dice y el motivo queda en «Detalles». En la app es la llamada `Service.Restart` de su tabla.

## Opciones de un fondo

Usa el esqueleto de opciones: el título es el nombre del fondo, con «Favorito» y «Poner de fondo»
arriba, y la ruta «Biblioteca › nombre» (Esc vuelve). El primer nivel muestra la vista previa, por
qué la escena está quieta si lo está, «Opciones del autor» (las de la escena en cada pantalla donde
está puesta), «Mis preajustes», lo que el paquete usa, «De su autora» (variantes y fondo base),
«Tuyo» (etiquetas, listas) y «Más». Una escena puesta sin opciones propias dice «Esta escena no
tiene opciones propias» y deja «Mis preajustes» y «Más».

El nivel «Más» (ruta «Biblioteca › nombre › Más») empieza por «Permisos de esta escena»: una fila por
cada cosa que la escena pide (puntero, sonido, clics, lo que suena y sus variantes con red), con
«Permitido» o «Sin permiso»; cambiarla responde con `Inputs1.Decide`. Siguen «Esta escena» (sonido,
«Editar una copia…»), el nombre, la valoración, los datos del archivo, «Seguir moviéndose» con
movimiento reducido, la privacidad de una página web y sus luces, «Editar…» y «Quitar de la
biblioteca…».

## Mis preajustes

El nivel «Mis preajustes» (ruta «Biblioteca › nombre › Mis preajustes») lista los preajustes del
fondo y los globales (`Profiles1.ListPresets`), con su colocación y su relleno. Intro usa el
preajuste en ese fondo (`Library1.Apply` con el preajuste); «Eliminar…» pide confirmación en un
panel junto a la fila. «Nuevo preajuste…» abre un panel con «Nombre», «Colocación» y «Color de
relleno»; Tab pasa al campo siguiente, Esc cancela y «Guardar» crea un preajuste global
(`Profiles1.SavePreset` con elemento vacío).

## Pregunta de permisos

Cuando una pantalla muestra una escena que pide cosas que la persona todavía no respondió (el campo
`undecided` de `GetInputs().shown`), la app hace una sola pregunta, «“nombre” pide permiso para usar»,
con una línea por cosa (puntero, sonido del equipo, clics y toques, lo que suena, y lo que suena o el
sonido con red), su explicación y «Permitir» o «Sin esto» en cada una. «Permitir» al pie aplica lo
elegido y «Sin esto» deja todo afuera, con `Inputs1.Decide`; la escena ya funciona mientras tanto sin
esos permisos, y si algo quedó afuera un aviso dice dónde cambiarlo. La red y las luces no se
preguntan acá. Se pregunta una vez por escena en cada sesión de la app; lo respondido se recuerda en
el servicio. La pieza es `PreguntaPermisos` y otras pantallas la usan con `ask(escena, clave,
nombre, cosas)`.

## Quieta en su póster

Si la escena puesta está quieta, el panel lo dice con su causa: una ventana tapa la pantalla, para
ahorrar batería, pantalla bloqueada, reproducción en pausa (`outputs.<pantalla>.still_reason`), o es
demasiado pesada para el equipo con la calidad automática (`quality_auto` = `still`). En este último
caso ofrece «Reproducir igual», que fija `playback.quality.<pantalla>` en `min-q1`, y además la app
muestra una vez un aviso con el mismo botón. Un fondo con destellos mientras están apagados en
Ajustes (`flash_disabled`) también lo dice; el ajuste general solo se cambia en Ajustes.

## Escena dañada

Si el archivo falta o está dañado, el panel muestra «No se pudo abrir esta escena» con el motivo,
«Volver a instalarla» cuando vino de un repositorio, «Mostrar en la carpeta» y «Quitar de la
biblioteca…» con confirmación.
