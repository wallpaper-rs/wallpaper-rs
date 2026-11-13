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
- Duración `paso` (180 ms) con la curva `pasoCurva`.
- Escalón: `escalon` vale `compacto` hasta 800 px de alto, `amplio` desde 1800 × 1000 y `normal`
  en el resto; la ventana lo fija sola y las medidas que cambian con él cambian solas.

## Íconos

Los íconos son un juego propio (Phosphor, licencia MIT) dentro de la app, como tema de íconos
`wprs`. Un ícono se pide por su nombre de Phosphor y su peso: `house` (normal, en reposo),
`house-fill` (activo) o `house-bold` (atajos). Los nombres de escritorio que la app ya usaba se
resuelven por la tabla `apps/wprs-app/look/icons/names.txt`. Ningún nombre cae en el tema del
sistema: lo que el juego no tiene no se dibuja, y una prueba de la app falla si una pantalla pide un
nombre que el juego no tiene. Los íconos toman el color del texto que los rodea.

## Controles

El estilo de controles es propio (`io.github.wallpaper_rs.style`) y se elige siempre al arrancar.
Dibuja el botón y la llave; los demás controles caen en el estilo básico de Qt con los tonos de
arriba.

- Botón: callado por defecto; `highlighted: true` es la acción principal (en Luna, letra de
  título); `flat: true` es un botón de palabra, sin superficie. La principal de un borrado lleva
  `palette.highlight: Look.coral`. Apagado, pulsado, con el puntero encima y con foco de teclado
  (anillo en Escarcha) se dibujan solos.
- Llave: `Switch`, 40 × 22 px; encendida en Escarcha con el punto en Noche.

Licencias de las letras y los íconos: `apps/wprs-app/licenses/`.
