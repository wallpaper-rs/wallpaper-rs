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
| título | Biblioteca, Explorar, Listas, Fuentes, Crear | columna izquierda con logo y menú bajo su velo; lugar «ahora» arriba a la derecha (por ahora la barra de estado: pausa, volumen, trabajos); velo de banda abajo; la página a la derecha del menú; un lugar de avisos; línea de atajos abajo a la derecha |
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
con su botón de cerrar.

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
- Avisos: `applicationWindow().showPassiveNotification(texto, tiempo, acción, función)`.

El esqueleto sale del lugar: Ajustes, Perfiles y Cuenta usan el de opciones; los editores, el de
trabajo; el resto, el de título.
