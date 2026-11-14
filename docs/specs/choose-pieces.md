# Piezas para elegir fondos

Piezas QML compartidas de las pantallas donde una persona elige (biblioteca, explorar, listas,
fuentes). Viven en `apps/wprs-app/qml/` dentro del módulo de la app y solo usan los tokens de `Look`
(ver `look.md`). Sus propiedades son datos y señales: nunca colores ni tamaños. Todas se manejan con
el teclado y tienen nombre para las herramientas de accesibilidad.

`wprs-app --pieces choose [IMAGEN...]` abre una página de prueba con cada pieza en sus estados (las
imágenes dadas se usan como portadas; se reconocen por la carpeta de la escena de ejemplo). No es una
pantalla para personas y ningún menú lleva a ella. `wprs-app --check` la crea y prueba su teclado.

## Filtro

Palabras de filtro con su cuenta; la actual lleva la marca del acento.

- `words`: lista de `{ "text": …, "count": … }` (la cuenta es opcional).
- `current`: índice de la palabra actual. `sections`: palabras más grandes, para las pestañas de una
  pantalla de opciones. `label`: nombre del grupo para accesibilidad.
- Señal `picked(index)`: la pantalla cambia `current`.
- Teclado: Tab entra al grupo, flechas izquierda y derecha mueven el anillo de foco, Intro o Espacio
  eligen.
- No usar para acciones (eso es un botón en palabras) ni para más de una línea de opciones.

## Buscar

Campo de búsqueda con la lupa.

- `text`, `placeholder`; señal `accepted()` con Intro.
- No usar para escribir valores de un ajuste: eso es el campo de texto de una fila de opciones.

## Quitable

Palabra de filtro aplicada que se puede quitar («Para adultos ✕»).

- `text`; señal `removed()`. El botón de quitar recibe el foco (Tab) y responde a Intro y Espacio.

## Tapa

Una portada: cuadro 16:9 con la imagen, nombre y nota.

- `picture` (URL), `name`, `note`, `noteIcon` (nombre de ícono), `noteTone` (`""`, `"ambar"`,
  `"coral"` o `"brote"`).
- Estados: en reposo está atenuada; `lit` la agranda (salvo con `even`) y le pone el filete de foco;
  el puntero encima aclara el velo; `off` la apaga (debe ir con su motivo en `note`); `inUse` pone
  «En uso»; `choosing` muestra la marca de elegir y `chosen` la llena.
- `even`: tamaño fijo dado por quien la contiene (rejilla); encendida no crece y muestra el nombre
  en negrita.
- Lo que se le pone como hijo aparece debajo de la tapa encendida y agrandada (la ficha).
- Señales `tapped(modifiers)` y `doubleTapped()`.
- La imagen carga fuera del hilo principal.
- Normalmente no se usa sola: la crean `Estante` y `Rejilla`.

## Ficha y FichaFija

`Ficha`: título, pie y acciones de la tapa encendida, siempre debajo de ella. `FichaFija`: lo mismo
en una línea fija al pie de «Ver todos», con las acciones a la derecha y una línea arriba.

- `title`, `caption`; los hijos son las acciones (botones y la tecla Intro junto a la principal).
- En `FichaFija` sin `title` las acciones ocupan la línea: así se le pone una `Seleccion`.
- Una sola acción principal por ficha.

## Uso

La marca «En uso» de una tapa. Sin propiedades. Solo la pone la tapa.

## Elegir y Seleccion

`Elegir` es la marca redonda de una tapa cuando se eligen varias (`chosen`). `Seleccion` es la línea
que toma el lugar de la ficha: «N elegidos» y sus acciones.

- `Seleccion.count`; los hijos son las acciones (la principal, las calladas, «Dejar de elegir» en
  palabras y la tecla Esc).
- Esc para dejar de elegir lo maneja la pantalla.

## Estante

La primera fila de una colección: tapas en reposo, una encendida y agrandada con su ficha debajo. Se
sale por el borde derecho: la pantalla le da el ancho hasta el borde de la ventana.

- `model`: cualquier modelo de lista (o un arreglo de objetos). Los nombres de los roles se eligen
  con `pictureRole` (`picture`), `nameRole` (`name`), `captionRole` (`caption`), `noteRole`,
  `noteIconRole`, `noteToneRole`, `inUseRole` (`inUse`) y `offRole` (`off`). No conoce la biblioteca.
- `currentIndex`: la tapa encendida. `currentName` y `currentCaption` la describen.
- `actions`: componente con las acciones de la ficha. `selection`: componente que reemplaza a la
  ficha mientras se elige y hay algo elegido.
- `choosing`, `chosen` (lista de índices); `label` para accesibilidad.
- Señales `activated(index)` (Intro o doble clic) y `toggled(index)` (Espacio o clic mientras se
  elige, o Ctrl+clic). La pantalla actualiza `chosen`.
- Teclado: flechas izquierda y derecha mueven la tapa encendida y el estante la sigue.
- Solo crea las tapas que se ven más una de reserva, y reutiliza las que salen de la vista.

## Rejilla

«Ver todos»: toda la colección en tapas del mismo tamaño, tantas columnas como entren.

- Las mismas propiedades y señales que `Estante`, salvo `actions` y `selection`: la ficha de «Ver
  todos» es una `FichaFija` que pone la pantalla, con `currentName` y `currentCaption`.
- Flechas en las cuatro direcciones.
- La pantalla le da alto; solo crea las filas visibles más una.

## Límites

- Las esquinas de la imagen de una tapa son rectas: el dibujo en software no recorta imágenes
  redondeadas sin efectos de capa.
- Una tapa apagada se atenúa más; no pasa a gris.
- La sombra de las tapas (`Sombra`) se ve solo cuando la ventana dibuja con la tarjeta gráfica.
- Los botones de la ficha muestran su anillo de foco cuando se llega a ellos con Tab.
