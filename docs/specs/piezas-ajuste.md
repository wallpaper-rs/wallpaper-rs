# Piezas de lo que se ajusta y de lo que dice cada pantalla

Piezas QML compartidas de la app de escritorio (`apps/wprs-app/qml/`). Cada pantalla las usa en vez
de dibujar las suyas. Solo reciben datos y avisan con señales: ningún color ni tamaño se pasa
desde la pantalla; todo sale de `Look` (ver `look.md`). Todas se usan con el teclado solo y tienen
nombre para las herramientas de accesibilidad.

Una página de prueba las muestra en cada estado: `wprs-app --pieces adjust` (teclas 1 a 4 cambian
de parte). No es una pantalla para personas: ningún menú lleva a ella. `wprs-app --check` la recorre.

## Filas de opciones

**`Filas`**: la columna de filas. Su ancho deja lugar a la explicación al costado.

**`Fila`**: una fila por ajuste, con su valor a la derecha. Con foco es la barra encendida en
Escarcha y su explicación aparece al costado.
- Datos: `text` (etiqueta), `note` (texto chico debajo) y `noteTone`, `value` (el valor en palabras),
  `number`, `explain`, `status` y `statusTone` (una línea de estado bajo la explicación), `off`
  (apagada: lo que el equipo no tiene; sigue recibiendo el foco y muestra su motivo).
- `kind` elige el tipo de valor:
  - `texto` (solo palabras);
  - `llave` (`checked`, Espacio o Intro la cambia, señal `toggled`);
  - `elige` (‹ valor ›: `options` y `current`, flechas izquierda y derecha, señal `chosen(index)`);
  - `riel` (`from`, `to`, `stepSize`, `at`, flechas, señal `moved(value)`);
  - `campo` (`fieldText`, `placeholder`; Intro entra al campo, Intro o Esc vuelven a la fila,
    señal `edited(text)`);
  - `captura` (`keys`, Intro escucha la combinación, Esc cancela, señal `captured(keys)`);
  - `pliega` (abre un segundo nivel, señal `opened`);
  - `despliega` (abre o cierra la sublista de abajo: `open`, señal `opened`).
- Flechas arriba y abajo pasan de fila en fila.
- No usarla para un formulario de varios campos dentro de un diálogo: ahí van los controles sueltos.

**`Seccion`**: el título chico que agrupa filas.

**`Sublista`**: una lista bajo una fila (las carpetas de la biblioteca), con «Quitar» en cada ítem y
una fila para añadir.
- Datos: `model` (lista de `{name, where, problem}`), `addText` y `removeText`.
- Señales: `removed(index)` (también con Suprimir) y `added`.
- `problem` se escribe en Coral (hondo sobre el ítem encendido).

## Lo que dice cada pantalla

**`Estado`** y **`Nota`**: una línea de estado en palabras con su ícono, y su versión chica.
- `text`, `tone` (`ambar` avisa, `coral` falla, `brote` salió bien; vacío es neutro), `icon` (uno
  propio del tono si no se da).
- `lit`: está sobre algo encendido y usa los tonos hondos.
- `Nota.fitWidth`: ancho máximo; el texto que no entra termina en «…» (la nota bajo una tapa).
- `Nota.wrap`: con `fitWidth`, el texto pasa a la línea siguiente en vez de recortarse; lo usan los avisos que hay que leer enteros (el llavero en Fuentes, sin conexión).
- El color nunca va solo: siempre hay palabras.

**`Vacio`**: lo que hacer cuando no hay nada. `title`, `text`, `action` y `actionIcon` (botón
principal, señal `acted`), `small` (dentro de una lista, sin acción en general).

**`Fallo`**: el problema en palabras de la persona y el botón que lo arregla. `title`, `text`,
`action` (señal `acted`), `details` (texto técnico detrás del pliegue «Detalles»), `detailsOpen`,
`big` (la pantalla entera falló).

**`Detalles`**: el pliegue para texto técnico. `text` (por defecto «Detalles»), `details`, `open`.

**`Aviso`** y **`Avisos`**: un aviso pasivo con una sola acción y la cruz para cerrar.
- `Avisos` es el lugar fijo de la superficie: muestra uno a la vez y deja los demás en cola.
  `show({text, tone, action})` agrega uno; la señal `acted(notice)` devuelve el aviso cuya acción
  se eligió. Se cierra solo después de un rato, salvo mientras el puntero o el foco están encima.
- Para lo que se puede deshacer: `action: qsTr("Undo")`, y la pantalla deshace en `acted`. Lo que se
  puede deshacer no se confirma.
- `Aviso` es la pieza suelta (`text`, `tone`, `action`, señales `acted` y `closed`); las pantallas
  usan `Avisos`.

## Superficies chicas

**`PanelChico`**: el panel opaco de toda superficie chica, puesto junto al control que lo abre
(debajo, o arriba si no entra).
- `opener` (el control), `cover` (foto de cabecera), `title`, `text`, contenido propio dentro.
- Pie: `quiet` (botón callado, señal `rejected`), `principal` y `principalIcon` (la única acción
  principal, señal `accepted`), `foot` (más piezas a la izquierda del pie).
- `borra`: confirmar algo que no se puede deshacer; el botón principal pasa a Coral.
- Esc o un clic fuera lo cierran y el foco vuelve al control que lo abrió.
- Pregunta agrupada: un `PanelChico` con varias **`Pide`** (`icon`, `text`, `reason`).

**`ListaMenu`** con **`Opcion`**: el menú que abre un control.
- `options` (lista de `{text, icon, note, tone, off}` o `{separator: true}`), `current`.
- Flechas, Intro o Espacio eligen; salta separadores y opciones apagadas. Señal `chosen(index)`.

**`Progreso`**: el avance en palabras con su riel. `text`, `count` («2 de 3»), `value` (0 a 1),
`indeterminate`.

## Lo que no hacen

- No pintan la ventana ni las pantallas: eso lo hacen las pantallas con estas piezas.
- Los botones, la llave suelta, el campo de texto y el riel suelto son controles del estilo propio;
  las piezas los usan y no los vuelven a dibujar.
- La fila encendida, el panel chico (también el menú de un control) y el aviso llevan su sombra
  (`Sombra`) solo cuando la ventana dibuja con la tarjeta gráfica; con el dibujo por software se
  ven sin ella. La foto de cabecera del panel chico (`Foto`) redondea sus
  esquinas de arriba con el mismo radio del panel; con el dibujo por software quedan rectas.
