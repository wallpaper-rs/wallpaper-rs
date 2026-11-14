# Ventana de la app de escritorio

La ventana (`apps/wprs-app/qml/Frame.qml`) pone el fondo en uso de borde a borde, el menú de
palabras a la izquierda y cada página en uno de tres esqueletos. Las páginas no dibujan barra
lateral, barra global ni título propio de Kirigami: el esqueleto se encarga.

## Imagen y tonos

- La imagen es el póster (miniatura de 1024 px) del fondo que se ve en la pantalla de la ventana;
  si esa pantalla no tiene fondo, el de la primera que tenga.
- Al cambiar de fondo la imagen nueva entra con un fundido; si el sistema pide menos movimiento
  (duraciones de animación en cero) cambia de golpe.
- Sin fondo en uso se ve una imagen de ejemplo de una escena incluida, más apagada, con la leyenda
  «Imagen de ejemplo: «Nevada», una de las escenas incluidas».
- Con la ventana oculta o minimizada no se pide ni se deriva nada. Los tonos de cada fondo quedan
  en memoria: volver a un fondo ya visto no repite el cálculo.

## Menú

Biblioteca, Explorar, Listas, Fuentes y Crear; aparte, más abajo, Ajustes y Cuenta. La palabra del
lugar actual va en Escarcha y negrita con la marca de acento. Perfiles y preajustes vive dentro de
Ajustes (palabras de sección arriba del panel). El menú es un solo bloque para el teclado.

## Esqueletos

| Esqueleto | Páginas | Partes |
|---|---|---|
| título | Biblioteca, Explorar, Listas, Fuentes, Crear | columna izquierda con logo y menú bajo su velo; lugar «ahora» arriba a la derecha (por ahora la barra de estado: pausa, volumen, trabajos); velo de banda abajo; la página a la derecha del menú o, si la pide, en la banda baja; un lugar de avisos; línea de atajos abajo a la derecha |
| opciones | Ajustes, Perfiles y preajustes, Cuenta | la imagen como franja con su velo; el menú; un panel opaco en Noche con el título de la página, sus acciones, palabras de sección y la ruta con «Esc volver»; la página; avisos sobre los atajos; atajos siempre al pie |
| trabajo | editor de un fondo, editor de escenas, empaquetador | barra superior en Noche con la vuelta («‹ Biblioteca», «‹ Crear»…), el título y las acciones; la página ocupa el resto; avisos arriba a la izquierda del lienzo |

Los escalones de tamaño de `look.md` (compacto hasta 800 px de alto, amplio desde 1800 × 1000) se
aplican solos.

## Teclado

- Flechas: se mueven dentro de un bloque (el menú, una fila, una lista).
- Tab: pasa al bloque siguiente (menú, «ahora», acciones, página).
- Intro: hace lo principal del bloque (en el menú, abre la palabra elegida).
- Esc: vuelve un nivel. En título y opciones lleva el foco al menú, o sube un paso de la ruta si la
  página tiene una. En el esqueleto de trabajo Esc no sale: solo sale la palabra de vuelta, para no
  dejar un editor sin querer.
- El foco se ve con el anillo de Escarcha.
- La línea de atajos solo muestra flechas, Intro, Esc y Tab.

## Avisos

Un solo lugar por esqueleto y un aviso a la vez: primero el error del servicio, después el aviso
de la cuenta y por último el aviso pasajero (`showPassiveNotification`), que se va solo a los 7 s o
con su botón de cerrar. Los tres se dibujan con la misma pieza `Aviso` (ver `piezas-ajuste.md`): el
error del servicio en Coral y con cruz (se oculta hasta que cambie), el pasajero con su acción
(«Deshacer», por ejemplo) y su tono. Mientras el puntero o el foco están sobre el aviso no se va.

## Tamaño

La primera vez la ventana abre al 90 % del área libre de la pantalla, hasta 1600 × 1000. Después
recuerda su tamaño (`~/.config/wallpaper-rs/window.conf`). Al abrir un editor crece hasta
1440 × 900 (o el 90 % de la pantalla, lo que sea menor); en pantallas de menos de 1280 × 800 se
maximiza.

## Lo que una página declara

- `title`: el título que muestra el esqueleto.
- `actions`: las acciones de la página; el esqueleto las pone en su barra.
- `keyHints` (opcional): lista `[{keys: ["left", "right"], text: "…"}]` para la línea de atajos;
  las teclas válidas son `left`, `right`, `up`, `down`, `Intro`, `Esc` y `Tab`. Sin ella la línea
  muestra «Tab Siguiente parte» y «Esc Menú».
- `route` y `routeBack()` (opcional, páginas de opciones): los pasos de la ruta y qué hace Esc.
- `full` (opcional, esqueleto de título): la página ocupa todo el alto; se ocultan el bloque
  «ahora» y sus velos, y un velo parejo en Noche cubre la imagen (la Biblioteca en «Ver todos»).
- `now` (opcional, esqueleto de título): un `Component` que ocupa el lugar de la hora y el estado,
  arriba a la derecha; la página pone ahí su título grande y su resumen.
- Avisos: `applicationWindow().showPassiveNotification(texto, tiempo, acción, función)`. En lugar
  del texto puede ir `{text, tone}` (`tone`: `""`, `"ambar"`, `"coral"` o `"brote"`) para darle su
  tono; la función es lo que hace la acción.

El esqueleto sale del lugar: Ajustes, Perfiles y Cuenta usan el de opciones; los editores, el de
trabajo; el resto, el de título. Una página de título puede pedir otro esqueleto mientras muestra
un nivel propio con `skeleton` («options»): la Biblioteca lo usa para las opciones de un fondo y
Fuentes para añadir una fuente y para sus detalles.

## Banda baja de las pantallas de título

Una página de título puede pedir la banda baja: una franja de margen a margen bajo el menú (palabras
de filtro, estante, ficha del elegido) y, si la necesita, un bloque arriba de la banda a la derecha
del menú. Una página que no la pide queda como siempre, en la columna a la derecha del menú.

- La página declara `bandLow` (el alto mínimo con el que su banda se ve bien; mayor que cero la
  activa), `bandHigh` (opcional: el alto que quiere como máximo; sin él la banda llega hasta el
  menú) y `upperHeight` (opcional: el alto de su bloque de arriba).
- La página declara también `property rect band` y `property rect upper`; el esqueleto las llena
  en coordenadas de la página, que ocupa toda la ventana. La página pone su banda en `band` y su
  bloque en `upper`, sin conocer las medidas de la ventana.
- Con lugar: `band` va del margen izquierdo al derecho, pegada al margen de abajo, con el alto
  entre `bandLow` y `bandHigh` según lo que haya bajo el menú; `upper` va a la derecha del menú,
  justo encima de la banda, con el alto pedido. Los atajos quedan en la esquina de abajo a la
  derecha de la banda.
- Si la banda no entra en el tamaño normal pero sí en el compacto, la ventana pasa al tamaño
  compacto aunque mida más de 800 px de alto, y vuelve al normal cuando crece hasta el alto en que
  la banda entra también ahí. Así no hay altos en los que la banda falte entre el mínimo en que
  cabe (unos 700 px en la hoja de muestra) y cualquier alto mayor. Solo pasa mientras la página
  pide la banda; las demás páginas siguen el tamaño de siempre.
- Sin lugar (ventana baja, o el bloque de arriba no entra): las dos van a la columna de la derecha
  del menú, el bloque arriba y la banda debajo, como en las páginas de siempre. El cambio ocurre al
  redimensionar, sin mover la página de lugar: el foco queda donde estaba y nada tapa el menú.
- El velo de banda cubre la ventana a lo ancho desde 190 px arriba de la banda. Si el texto de la
  banda empieza más abajo que `band`, la página declara `bandFrom` (en sus coordenadas) y el velo
  empieza 190 px arriba de ese punto, para que se lea sobre un fondo claro. Una página sin banda
  puede declarar `bandFrom` igual (la Biblioteca, en el punto de su barra): el velo sube hasta
  190 px arriba de ese punto, pero nunca empieza más abajo que en las demás páginas.
- Una página que ocupa todo el alto (`full`) no usa la banda.
- Con la banda, los avisos van a la derecha del menú, encima del bloque de arriba.
- `wprs-app --pieces band [IMAGEN...]` muestra una página de prueba con la banda (la primera imagen
  hace de fondo).

## Fuentes

Cada proveedor en línea es una tarjeta: nombre, estado, «N fotos, actualizado hace…», el botón
grande «Rotar estas fotos» (crea o reutiliza la lista `source:<fuente>` y la arranca en todas las
salidas), «Actualizar» y «Detalles». Si no hay ninguna, la página muestra las tarjetas de tipos para
añadir una; Bing, NASA y Wikimedia se añaden con un clic, los demás piden primero lo que necesitan
(búsqueda, servidor, clave). Las claves nunca se vuelven a mostrar. Un proveedor en error se trata
como falta de conexión y se avisa una sola vez. Las opciones de Wallhaven siguen en su diálogo.

En los detalles de una fuente, «Fotos guardadas» dice cuánto ocupa lo que trajo y ofrece «Borrar…»:
el panel «¿Borrar las fotos guardadas?» borra esas fotos del equipo y de la biblioteca
(`Providers1.ClearCache`, `wprs-ctl provider clear-cache ID`); la fuente sigue con sus opciones y su
clave y trae fotos nuevas en la próxima actualización. «Quitar esta fuente» quita además la fuente.
