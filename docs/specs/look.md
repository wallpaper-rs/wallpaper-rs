# Aspecto de la app de escritorio

La app tiene un aspecto propio y siempre oscuro. No sigue el esquema de colores, el tema de íconos,
la letra ni el estilo de controles del sistema: todo sale de la app.

## Tonos

Viven en el singleton QML `Look` (`apps/wprs-app/qml/facade/Look.qml`). Los siete primeros salen
del fondo en uso: la ventana toma el póster del fondo que se ve en su pantalla, saca los tonos con
`wprs-palette` (`Looks.derive`), los guarda por fondo y los aplica con `Look.tone({...})`. Sin
fondo en uso, o si el póster no se puede leer, valen el juego fijo «sin fondo». La ventana y sus
partes se describen en `shell.md`.

| Propiedad | Nombre | Para qué |
|---|---|---|
| `noche` | Noche | tono profundo: superficie de la ventana y de los paneles |
| `nocheAlta` | Noche alta | superficie apenas más clara: botones, campos, tarjetas |
| `nocheLinea` | Noche línea | bordes, separadores, botón con el puntero encima |
| `bruma` | Bruma | texto secundario, íconos de las palabras |
| `brumaBaja` | Bruma baja | texto apagado |
| `luna` | Luna | el acento: lugar actual, acción principal, «En uso» |
| `lunaHonda` | Luna honda | el acento pulsado |
| `velo` | — | opacidad del velo sobre la foto (`veloColor` es Noche con esa opacidad) |
| `escarcha` | Escarcha | texto principal, anillo de foco (fijo) |
| `ambar`, `coral`, `brote` | Ámbar, Coral, Brote | avisos, errores y hecho (fijos) |
| `ambarHondo`, `coralHondo`, `broteHondo` | — | fondos de esos estados (fijos) |

La paleta de la aplicación se arma con estos tonos y los sigue cuando cambian, así que los
controles sin estilo propio y las piezas de Kirigami (páginas, diálogos, formularios, títulos,
mensajes) también los toman. Los colores de estado de Kirigami de la ventana apuntan a Ámbar, Coral
y Brote.

## Letra

Dos letras van dentro de la app: `letraTexto` («Onest», la letra de la aplicación) y
`letraTitulo` («Red Hat Display», menús, títulos y la acción principal). La escala es `tPortada`,
`tPagina`, `tGrande`, `tMenu`, `tFoco`, `tFila`, `tSeccion`, `tAparte`, `tFiltro`, `tExplica`,
`tBoton`, `tNombre`, `tCuerpo` (15 px, el texto corriente), `tPie` y `tTecla`, en píxeles.

## Medidas

- Espacio: `e1` a `e8` (4, 8, 12, 16, 24, 32, 48, 64); `margen`, `margenArriba`, `margenAbajo`.
- Radios `radio` y `radioChico`; grosores `linea`, `anillo` y `anilloHueco` (separación del anillo
  de foco).
- Tamaños: `menuAncho`, `menuAlto`, `aparteAlto`, `tapa`, `tapaFoco`, `tapaRejilla`,
  `huecoEstante`, `altoControl`, `altoBoton`, `altoFila`, `altoCampo`, `filasAncho`,
  `explicaAncho`, `atajosAlto`, `llaveAncho`, `llaveAlto`, `llavePunto`.
- Medidas de los controles: `altoHerr` (32, herramienta y botón chico), `altoOpcion` (38, opción
  de un menú), `radioPanel` (10, panel chico), `rielAlto` y `rielPunto` (riel de 4 px y su punto
  de 14), `marca` (18, casilla y opción redonda), `icono` (18, ícono dentro de un control), `barra`
  (6, barra de desplazamiento) y la letra `tHerr` (14).
- Duración `paso` (180 ms) con la curva `pasoCurva`.
- Escalón: `escalon` vale `compacto` hasta 800 px de alto, `amplio` desde 1800 × 1000 y `normal`
  en el resto; la ventana lo fija sola y las medidas que cambian con él cambian solas.

## Sombras

`Sombra` pone la sombra suave de una superficie: se pone como hija del fondo de la superficie, que
debe ser opaco, y se dibuja debajo de él (`z: -1`) con el mismo `corner`. `kind` elige la sombra:
`tapa` (toda tapa), `foco` (la tapa encendida), `fila` (la fila encendida) y `panel` (panel chico,
menú de un control y aviso). Ejemplo: `Rectangle { radius: Look.radio; color: Look.noche;
Sombra { kind: "panel" } }`. Solo existe cuando la ventana dibuja con la tarjeta gráfica; con el
dibujo por software no se crea y la superficie queda como sin ella. No trabaja por cuadro: se
dibuja una vez y se vuelve a dibujar solo si la superficie cambia.

## Fotos redondeadas

`Foto` es una `Image` (las mismas propiedades, con `PreserveAspectCrop` y carga asíncrona por
defecto) que redondea sus esquinas con `corner` (por defecto `Look.radio`); `lowCorner` cambia solo
las dos de abajo (la cabecera del panel chico usa `corner: Look.radioPanel` y `lowCorner: 0`).
Ejemplo: `Foto { anchors.fill: parent; source: model.picture; sourceSize.width: Look.tapaFoco }`.
Redondea solo cuando la ventana dibuja con la tarjeta gráfica y la imagen ya cargó; con el dibujo
por software la foto se ve con esquinas rectas, como una `Image`.

## Íconos

Los íconos son un juego propio (Phosphor, licencia MIT) dentro de la app, como tema de íconos
`wprs`. Un ícono se pide por su nombre de Phosphor y su peso: `house` (normal, en reposo),
`house-fill` (activo) o `house-bold` (atajos). Los nombres de escritorio que la app ya usaba se
resuelven por la tabla `apps/wprs-app/look/icons/names.txt`. Ningún nombre cae en el tema del
sistema: lo que el juego no tiene no se dibuja, y una prueba de la app falla si una pantalla pide un
nombre que el juego no tiene. Los íconos toman el color del texto que los rodea.

## Controles

El estilo de controles es propio (`io.github.wallpaper_rs.style`) y se elige siempre al arrancar.
Dibuja cada control que la app usa; una prueba de la app (`styled`) falla si una pantalla usa un
control que el estilo no dibuja. Todos tienen reposo, puntero encima, pulsado, apagado y el anillo
de foco de teclado en Escarcha. `wprs-app --check` crea una vez cada archivo del estilo y falla
nombrando el archivo que no carga o no crea su control; además compila cada archivo QML de la app y
del estilo e imprime el error de cada componente que no carga (archivo, línea y mensaje).

- Botón: callado por defecto; `highlighted: true` es la acción principal (en Luna, letra de
  título); `flat: true` es un botón de palabra, sin superficie. La principal de un borrado lleva
  `palette.highlight: Look.coral`. Apagado, pulsado, con el puntero encima y con foco de teclado
  (anillo en Escarcha) se dibujan solos.
- Llave: `Switch`, 40 × 22 px; encendida en Escarcha con el punto en Noche.
- Casilla (`CheckBox`, `CheckDelegate`) y opción redonda (`RadioButton`): de la familia de la
  llave; apagada en Noche línea, marcada en Escarcha con la marca en Noche.
- Campo de texto (`TextField`, `TextArea`, también la búsqueda y la contraseña): Noche alta con
  línea, letra Escarcha, texto de ayuda en Bruma; el anillo aparece al escribir.
- Número (`SpinBox`): un campo con las flechas ‹ › a los lados.
- Lista desplegable (`ComboBox`) y menú (`Menu`, `MenuItem`, `MenuSeparator`): el panel chico
  (Noche, línea, radio 10) con opciones de 38 px; la opción con el puntero encima en Noche alta y
  la elegida con el teclado en Escarcha con letra Noche.
- Herramienta (`ToolButton`): palabra o ícono en Bruma sin superficie; la que queda elegida
  (`checked` o `highlighted`) en Escarcha con letra Noche.
- Pestañas (`TabBar`, `TabButton`): palabras de filtro en letra de título; la actual en Escarcha
  con su marca en Luna.
- Riel (`Slider`, `RangeSlider`, `ProgressBar`, `Dial`): riel de 4 px en Noche línea, lleno en
  Escarcha, punto de 14 px.
- Barras de desplazamiento finas en Noche línea, que aparecen solo al moverse.
- Aviso emergente (`ToolTip`), ventana emergente (`Popup`) y diálogo (`Dialog`): panel en Noche
  con línea; en la fila de botones de un diálogo, Aceptar o Sí es la acción principal.
- Barra (`ToolBar`): Noche alta con una línea hacia el contenido.

Licencias de las letras y los íconos: `apps/wprs-app/licenses/`.

## La bandeja de Plasma

El panel de la bandeja y la app comparten la misma letra, los mismos nombres de tono, las mismas
medidas y los mismos íconos, y los toman de una sola fuente: `scripts/plasma-assets.sh` copia las
letras, deriva `contents/look/Look.qml` de `apps/wprs-app/qml/facade/Look.qml` y copia solo los
íconos que el panel nombra. Lo corren `scripts/package.sh` y la receta de Flatpak; la carpeta
`contents/look` no se guarda en el repositorio.

- Los tonos de cada pantalla los publica el demonio en `GetState` (`outputs.<pantalla>.tones`, ver
  `dbus.md`); el panel viste con los de la primera pantalla o, en la vista de una pantalla, con los de
  esa. Sin fondo del que derivarlos usa los tonos fijos.
- Con una pantalla, su portada es el encabezado del menú. Con más de una hay una fila por pantalla
  (miniatura, nombre del fondo, *Siguiente*, *Elegir…* y *⋯*), y *⋯* abre la vista de esa pantalla:
  *Favorito*, *Ajuste*, *Filtro*, los clics y toques, *Quitar el fondo de esta pantalla*, *Volver al
  fondo de Plasma* y *Recuperar el anterior*.
- *Cambiar con vista previa…* llama a `Playlists1.Browse` y muestra a la derecha la tecla de
  `shortcuts.next` si `shortcuts.enabled` está activo.
- *Más* reúne los cuatro interruptores de sonido y destellos y *Pausar con batería* (Nunca, Siempre
  o por debajo del 20 %).
- La fila de una pantalla no dice por qué un fondo está quieto: el motivo se lee solo en la
  terminal (`wprs-ctl status`). Una escena demasiado pesada para el equipo sigue con su aviso.
- El perfil ya no se elige en la bandeja: se hace en la app. La miniatura de un paquete para adultos
  ya no se difumina por fila.
- El panel no se puede ver sin una sesión de Plasma con el plugin; `scripts/plasma-features.sh` corre
  `tests/tray-row.qml` y `tests/tray-left-out.qml` allí.
