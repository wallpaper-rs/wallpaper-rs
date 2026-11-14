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
«Probar de nuevo» y los detalles; reintenta cada tres segundos y vuelve sola cuando el servicio contesta.
No ofrece reiniciar el servicio: la app no tiene cómo hacerlo.

## Opciones de un fondo

Usa el esqueleto de opciones: el título es el nombre del fondo, con «Favorito» y «Poner de fondo»
arriba, y la ruta «Biblioteca › nombre» (Esc vuelve). El primer nivel muestra la vista previa, lo que
el paquete usa, «De su autora» (variantes y fondo base), «Tuyo» (ajustes por pantalla, sonido,
preajustes, etiquetas, listas) y «Más». El nivel «Más» tiene el nombre, la valoración, los datos del
archivo, «Seguir moviéndose» con movimiento reducido, la privacidad de una página web y sus luces,
«Editar…» y «Quitar de la biblioteca…».
