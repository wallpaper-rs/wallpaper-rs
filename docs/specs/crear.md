# Página Crear

Es donde un autor empieza: un proyecto nuevo, uno reciente, una copia de un fondo instalado o una
carpeta. Usa el esqueleto de título; el editor se abre como siempre, con `host.work`.

## Qué muestra

- «Seguir con lo tuyo»: los proyectos recientes, el más nuevo primero. Se guardan en
  `recent-projects.json`, en el directorio de estado (40 como máximo) y se suman las
  carpetas que haya en la carpeta de proyectos. Una carpeta que ya no existe sigue en la fila, marcada
  «Su carpeta ya no está», con «Buscarla…» y «Quitar de la lista».
- «Empezar uno nuevo»: una escena (vacía, partículas, paralaje por capas, reloj de texto), un fondo
  de un solo archivo (imagen, vídeo, página web) y, plegado en «Más», preajuste y paquete de recursos.
- «Aprender con un ejemplo»: las escenas de ejemplo primero y después los demás paquetes instalados.
  «Editar una copia…» abre un panel con la licencia y el nombre de la copia.
- «Abrir una carpeta…»: si la carpeta no tiene escena ni paquete, avisa con «Elegir otra».

Sin proyectos y con ejemplos instalados, la página abre en «Aprender con un ejemplo». El pie sigue
la tarjeta marcada: título, una línea, «Cambiar…» (título y carpeta) y el botón principal.
«Cambiar…» deja en una sola fila el nombre del punto de partida, «Título», «Carpeta» y «Crear», con
el foco en «Título»; la carpeta se muestra con `~` para la carpeta personal y también se puede
escribir así. Intro en cualquiera de los dos campos crea; Esc vuelve a las tarjetas con la misma
tarjeta marcada. En una ventana demasiado angosta los campos pasan debajo del título y «Crear»
queda en la línea del título.

## Cómo se abre el editor

`App.compose(carpeta, punto de partida, título)` crea la carpeta si hace falta y abre el editor de
escenas o el empaquetador con `host.work`. Para imagen y vídeo se elige antes el archivo: su ruta
viaja en `App.media` y el empaquetador la toma (`App.takeMedia()`, que la deja vacía) y la aplica
con `manifestEdit("media", {file})` tras crear.
Al cargar cualquier proyecto se anota en la lista de recientes.

## Límites

- Las tarjetas de «Empezar uno nuevo» son imágenes fijas o íconos, sin vista previa animada.
