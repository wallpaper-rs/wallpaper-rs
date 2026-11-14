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
lugar actual va en Escarcha y negrita con la marca de acento. Las disposiciones (antes perfiles) viven
dentro de Ajustes, en la pestaña Pantallas. El menú es un solo bloque para el teclado.

## Esqueletos

| Esqueleto | Páginas | Partes |
|---|---|---|
| título | Biblioteca, Explorar, Listas, Fuentes, Crear | columna izquierda con logo y menú bajo su velo; lugar «ahora» arriba a la derecha (ver «Bloque ahora»); velo de banda abajo; la página a la derecha del menú o, si la pide, en la banda baja; un lugar de avisos; línea de atajos abajo a la derecha |
| opciones | Ajustes, Cuenta | la imagen como franja con su velo; el menú; un panel opaco en Noche con el título de la página, sus acciones, palabras de sección y la ruta con «Esc volver»; la página; avisos sobre los atajos; atajos siempre al pie |
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
En una página con banda baja el aviso va arriba de la banda, a la izquierda, sin tapar la fila de
filtros ni la ficha; en las demás, al pie de la columna.

## Tamaño

La primera vez la ventana abre al 90 % del área libre de la pantalla, hasta 1600 × 1000. Después
recuerda su tamaño (`~/.config/wallpaper-rs/window.conf`). Al abrir un editor crece hasta
1440 × 900 (o el 90 % de la pantalla, lo que sea menor); en pantallas de menos de 1280 × 800 se
maximiza.

## Lo que una página declara

- `title`: el título que muestra el esqueleto.
- `actions`: las acciones de la página; el esqueleto las pone en su barra.
- `bar` (opcional, solo en el esqueleto de trabajo): un componente que ocupa la barra superior después del
  título, en lugar de las acciones; lo usa una página cuya barra tiene un orden propio (estado, grupos).
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

El esqueleto sale del lugar: Ajustes y Cuenta usan el de opciones; los editores, el de
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
(búsqueda, servidor, clave). Las claves nunca se vuelven a mostrar. Una fuente en error dice su propio
problema en su tarjeta. Solo cuando una fuente no llega a su servidor (`offline` del daemon) la
lista muestra arriba el aviso «No hay conexión…» con «Reintentar», y esa tarjeta sigue con su
resumen normal. En una fuente de
Wallhaven, «Más opciones» abre en la misma página dos grupos sin desplazamiento: «Qué fotos»
(categorías, contenido y tamaño y colores, cada uno con su nivel de casillas) y «Cómo buscarlas»
(orden con «Mezclar otra vez» cuando es al azar, colecciones de un usuario, parecidos a una foto y
«Otras opciones de búsqueda» con las palabras, el sentido, el período, las páginas y el tope). Cada
cambio se guarda al momento; Esc vuelve un nivel. Los colores se nombran por su código. Guardar las
claves en un archivo privado, cuando no hay llavero, se confirma en un panel chico propio.

En los detalles de una fuente, «Fotos guardadas» dice cuánto ocupa lo que trajo y ofrece «Borrar…»:
el panel «¿Borrar las fotos guardadas?» borra esas fotos del equipo y de la biblioteca
(`Providers1.ClearCache`, `wprs-ctl provider clear-cache ID`); la fuente sigue con sus opciones y su
clave y trae fotos nuevas en la próxima actualización. «Quitar esta fuente» quita además la fuente.

## Ajustes

- Tres pestañas: Fondos, Pantallas y Equipo. Cada pestaña declara sus filas en `rows`
  (`{text, value, place, explain, off}`): con eso el buscador del encabezado filtra las filas de las
  tres por su etiqueta, sin distinguir mayúsculas ni tildes, y al elegir un resultado abre la
  pestaña y el lugar de la fila y le da el foco. Sin resultados dice «Ningún ajuste habla de…» y
  propone palabras para probar.
- Cada fila que abre algo lleva a un lugar con ruta y «Esc volver»: Más, Carpetas de la
  biblioteca, Listas por pantalla, Pausar, Más de la pantalla y Disposiciones.
- Pantallas muestra la calidad de las escenas (Automática, Ahorrar batería, Máxima; los siete
  niveles en «Más de…»), la transición de cada pantalla con el motivo del último cambio, la
  general, Menos movimiento, Pausar y Disposiciones. Una fila cuyo equipo no se detecta (la
  batería) queda en gris con «no detectada», y lo que la persona fijó se mantiene.
- Disposiciones: cada una es una fila con sus reglas; Intro ofrece Usarla, Reglas automáticas,
  Cambiar el nombre y Eliminarla. Al conectar o quitar una pantalla, si ninguna disposición guarda
  ese conjunto, un aviso ofrece guardarla y la guarda con esas pantallas como regla.
- La fila de cada carpeta de la biblioteca ofrece Revisarla otra vez, Rotar estas fotos (una lista
  con esa carpeta en todas las pantallas) y Quitarla, con confirmación.

## Bloque ahora

Arriba a la derecha de las pantallas de título está lo que hay de fondo en la pantalla elegida:

- Con una pantalla dice «En tu escritorio»; con varias muestra una tira con la miniatura de cada
  una y su nombre, y elegir una cambia la pantalla de la página (`target`) o la del bloque.
- Título del fondo, su tipo y quién lo da; «Puesto hace un momento» durante un minuto después de
  cambiarlo con la ventana visible.
- Mandos: pausar o seguir, volumen general con silencio (solo si el fondo suena), favorito, ajuste
  y filtro de esa pantalla.
- Si la pantalla rota una lista: «Lista «Tarde» · cambia en 7 min» con Anterior, Siguiente y
  «Pausar la lista». El reloj se actualiza una vez por minuto y solo con la ventana visible.
  «Pausar la lista» detiene la rotación y ofrece Reanudar en el aviso; con la regla `resume` vuelve
  al mismo fondo.
- Poner un fondo a mano sobre una pantalla que rota avisa «La rotación de «Tarde» se detuvo porque
  pusiste «Aurora boreal».» con Reanudar.
- Trabajos en curso («Haciendo las miniaturas: 9 de 12») con Cancelar, y los fallidos con su motivo.
- Si la escena de esa pantalla pesa demasiado y queda en su póster (`quality_auto` = `still`), una
  línea Ámbar «Quieta en su póster» bajo la línea de tipo. El motivo de una pausa no se muestra: va
  solo a la terminal.
- Sin ningún fondo puesto: «En tu escritorio», «Aún no tienes fondos», «Elige por dónde empezar: lo
  primero que entre en tu biblioteca queda de fondo.» y la nota de la imagen de ejemplo, sin mandos.
- En la Biblioteca y en Listas, que usan la banda baja, el bloque va entero. Solo se acorta (sin
  tira, sin línea de tipo y sin trabajos) en una página que pone su propio bloque arriba de la
  banda. En una ventana baja la banda pasa a la columna y el bloque queda arriba, entero.

## Listas

La página de listas usa la banda baja: filtro Todas y En marcha (cada palabra con su número),
«Nueva lista…» y un estante de tapas hechas con cuatro miniaturas de la lista. La ficha dice cuántos
fondos tiene, cada cuánto cambia y si está detenida, con «Poner en marcha» (o «… en» la pantalla
elegida), Intro y «Editar ›». Sin listas muestra «Aún no tienes listas» con el camino desde la
biblioteca.

Editar abre el esqueleto de opciones con la ruta «Listas › Tarde»: Nombre, «Cambia cada» (10 min,
30 min, 1 h, Cada día u «Otro…», que admite segundos, minutos u horas), Pantalla (con varias), los
fondos con Quitar, «Añadir fondos…» y «Añadir una carpeta…», y Más (historial, búsqueda, reglas y
quitar la lista). Arriba están «Cambia en», «Pausar la lista» y «Detener». Quitar una lista ofrece
Deshacer en el aviso, que la vuelve a crear con sus fondos y sus reglas.

«Fondos» se despliega en el mismo nivel: cada fondo trae su miniatura, «en pantalla ahora» si se
está mostrando y Quitar. En una lista manual el orden se cambia arrastrando el asa de la fila o con
Ctrl+Arriba y Ctrl+Abajo sobre el fondo elegido; la fila elegida conserva el foco al moverse. Si la
lista cambió mientras tanto, el aviso lo dice y la vuelve a cargar. Una lista con búsqueda muestra
sus fondos sin asas ni Quitar. «Añadir una carpeta…» copia los fondos de la carpeta una vez; seguirla
para que los archivos nuevos entren solos está en Más, «Seguir una carpeta…». Se muestran hasta 500
fondos.

Más se arma con filas: Búsqueda (vacía es una lista manual; con texto dice cuántos fondos coinciden),
«Convertir en manual» en una lista con búsqueda, «Seguir una carpeta…», una fila por regla de la lista
(llave, elección o campo según la regla), Historial (pliegue con lo que mostró hace poco, lo más
reciente primero, y la pantalla cuando hay varias) y «Quitar la lista». Abajo se ven las teclas
Moverse, Cambiar y Volver.
