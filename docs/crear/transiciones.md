# Crear y probar una transición propia

Un fondo puede traer su propia transición para cuando entra («Al entrar») y otra para cuando sale
(«Al salir»). Sin una propia, quien lo use ve la que tenga elegida en sus ajustes. El formato que
se guarda en `scene.json` está en [efectos](efectos.md#una-transición-propia) y en
[`ejemplos/transicion-propia`](ejemplos/transicion-propia/scene.json).

## Crearla

1. En el editor, abre **Opciones** y baja hasta **Transiciones**.
2. Pulsa **Crear una transición…** en «Al entrar» o en «Al salir». Si el fondo todavía no tiene un
   efecto de transición, el editor escribe uno de partida, «Apertura suave»
   (`transitions/opening.json` y `transitions/opening.wgsl`), que abre la escena desde un centro.
   Si la otra dirección ya tiene un efecto, la nueva usa el mismo.
3. Se abre la herramienta:
   - el lienzo muestra el cruce entre un **fondo anterior** y tu escena, con dos rótulos («Sale …»,
     «Entra … · 45 %»);
   - en el pie, **Progreso** mueve el cruce a mano y el editor de curva muestra cómo avanza en el
     tiempo;
   - a la derecha, el panel tiene el **Efecto**, la **Duración** y los valores del efecto.
4. Pulsa **Listo** para volver al lienzo normal. La transición queda guardada con la escena.

## Ajustarla

- **Duración**: de 0,1 a 10 s. Un valor fuera de ese rango no se guarda y el panel dice por qué
  junto al campo.
- **Curva**: el botón **Curva…** del pie trae las curvas con nombre («suave al final, media»,
  «pareja»…). Las dos asas del editor de curva dibujan una curva propia; con Tab llegas a cada asa
  y las flechas la mueven.
- **Valores del efecto**: los números se ajustan con su deslizador en el panel. Los puntos (por
  ejemplo el centro de «Apertura suave») son asas sobre el lienzo: se arrastran o se mueven con las
  flechas después de llegar a ellas con Tab.
- **Efecto**: la lista trae todos los descriptores del paquete con `"stage": "transition"`. Para
  escribir el tuyo, sigue la guía de [efectos](efectos.md#una-transición-propia) y ponlo en la
  carpeta del paquete; aparece en la lista.
- **Quitar la transición** borra esa dirección. Deshacer la trae de vuelta.

## Probarla

- **Probar la transición** la reproduce entera con su duración y su curva.
- **Fondo anterior** elige contra qué se prueba: un color liso o una imagen tuya.
- El cruce se ve como en **Ver como**: cambia el equipo arriba para probar cada nivel de calidad.
  Si en un nivel la transición no se reproduce, el panel lo avisa y el lienzo muestra el fundido
  simple que vería quien lo use.
- Con **Menos movimiento**, quien lo use ve siempre un fundido simple; el lienzo también lo muestra
  así cuando lo activas en la app.

## Desde la terminal

Todo lo anterior es la acción `transition` del editor:

```sh
wprs-ctl scene act DIR transition '{"way": "in", "make": true}'
wprs-ctl scene act DIR transition '{"way": "in", "duration_ms": 1800, "curve": "out_cubic"}'
wprs-ctl scene act DIR transition '{"way": "in", "params": {"center": [0.3, 0.6]}}'
wprs-ctl scene ask DIR transition
wprs-ctl scene act DIR transition '{"way": "out", "remove": true}'
```

`scene actions transition` muestra todos sus campos. La acción revisa lo mismo que la carga del
fondo y nunca escribe algo que la revisión rechace.

## Límites

- Mientras una transición se reproduce, el programa guarda en la memoria de la tarjeta gráfica tres
  imágenes del tamaño de la pantalla por cada pantalla: el fondo que sale, el que entra y el cruce.
  En pantallas muy grandes o con muchas pantallas eso suma; termina al acabar la transición.
- Una transición cuesta a lo sumo 8 unidades y no usa imágenes, tablas de color, máscaras ni
  emisores; si el efecto pide más, el panel lo dice y no se guarda.
- Los archivos del efecto de partida quedan en la carpeta aunque deshagas la creación; bórralos a
  mano si no los usas.
- La vista previa del cruce usa una foto quieta de la escena en el momento del reloj del editor, no
  la escena animada.
