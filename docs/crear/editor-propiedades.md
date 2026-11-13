# Propiedades de usuario y preajustes en el editor

Una propiedad de usuario es un ajuste que tú declaras y que quien usa el fondo puede cambiar: un deslizador de velocidad, un color, un interruptor, una lista de opciones. Un preajuste es un conjunto de valores de esas propiedades que se aplica con un clic. Esta página explica cómo crearlos, ordenarlos, enlazarlos a las capas, ponerles condiciones, traducirlos y probarlos.

Cómo se escribe cada propiedad dentro de los archivos está en [propiedades](propiedades.md). Los datos del propio paquete (título, licencia, créditos, red, permisos...) están en [paquete](paquete.md); aquí solo se trata lo que toca a propiedades, preajustes y textos traducidos. Las operaciones de la terminal se llaman acciones y están listadas en [acciones](acciones.md). El resto del editor está en [editor](editor.md).

## Dónde está todo

Dónde está: en el panel de la izquierda del editor, la pestaña **Paquete** (las demás son **Capas**, **Recursos**, **Reproductor** y **Probar**).

La pestaña tiene, de arriba abajo:

- **Detalles del paquete**: la fila que abre los datos del paquete en el inspector (mira [paquete](paquete.md)).
- **Propiedades de usuario**: una fila por propiedad, en el orden en que las verá quien use el fondo. Cada grupo va seguido de sus miembros, algo más a la derecha. La fila dice «etiqueta · tipo». Si aún no le has puesto etiqueta, dice el id. Si dejas el puntero encima, se ve el id.
- **Añadir una propiedad**.
- **Preajustes**: una fila por preajuste, con su etiqueta (o su id si no tiene), y un botón **Quitar** al final de la fila.
- **Añadir un preajuste**.

Todo está en una sola lista que se desplaza. Al pulsar una fila, el inspector (el panel de la derecha) muestra los campos de esa propiedad o de ese preajuste. En un paquete de página web, estas dos listas son la pestaña **Propiedades** del panel central; en un paquete de vídeo la pestaña **Preajustes** es otra (mira [Preajustes de un vídeo](#preajustes-de-un-vídeo)).

No hay atajos de teclado para esta zona.

## Qué tipos de propiedad hay

**Añadir una propiedad** ofrece estos doce tipos. Cada uno nace con unos valores que cambias después:

| Tipo | Qué es | Nace con |
|------|--------|----------|
| **Deslizador** | Un número entre un mínimo y un máximo | de 0 a 100, paso 1, valor 50 |
| **Interruptor** | Sí o no | apagado |
| **Color** | Un color | blanco |
| **Selección** | Una lista de opciones, se elige una | dos opciones, `option_1` y `option_2`, sin etiqueta |
| **Texto** | Un texto que escribe el usuario | vacío |
| **Archivo** | Una imagen o un vídeo que elige el usuario | acepta imágenes |
| **Carpeta** | Una carpeta de la que se toman imágenes o vídeos | acepta imágenes |
| **Atajo** | Algo que el usuario elige abrir al pulsar el fondo | sin campos |
| **Botón** | No guarda un valor; pulsarlo es un aviso | sin campos |
| **Grupo** | Reúne propiedades bajo un título | sin campos |
| **Etiqueta** | Una línea de texto en el formulario | sin campos |
| **Separador** | Una línea que cruza el formulario, con título opcional | sin campos |

Las propiedades de tipo archivo, carpeta, atajo y texto secreto las elige quien usa el fondo; ni tú ni un preajuste podéis fijarlas.

## Cómo añadir una propiedad

Dónde está: pestaña **Paquete**, botón **Añadir una propiedad**.

Pasos:

1. Pulsa **Añadir una propiedad**. Se abre un menú con los tipos de la tabla anterior.
2. Elige un tipo. La propiedad se añade al final de la lista con un id que el editor inventa a partir del tipo (`slider_1`, `combo_1`...) y queda elegida en el inspector. Aún no tiene etiqueta, así que la lista muestra el id.
3. En el inspector, escribe la **Etiqueta** (mira [Cómo traducir las etiquetas](#cómo-traducir-las-etiquetas-y-los-textos)). La etiqueta es lo que ve el usuario; el id solo lo usas tú.
4. Rellena el resto de campos de su tipo (mira las secciones siguientes).

El id empieza por una letra y sigue con hasta 63 letras, cifras o guiones bajos. No puede repetirse aunque cambie una mayúscula. Para cambiarlo después mira [Cómo cambiar el nombre o quitar una propiedad](#cómo-cambiar-el-nombre-o-quitar-una-propiedad).

Cada cambio de propiedades vuelve a calcular lo que el paquete necesita (por ejemplo, que usa los tipos más recientes de propiedad), sin que tengas que hacer nada. Todo se puede deshacer como el resto del editor.

En la terminal: acción `add_property` (orden `wprs-ctl scene add-property`, con `--id`, `--group` y `--label`). El tipo `shortcut` se añade con `scene act`. Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena slider --id velocidad --label Velocidad
wprs-ctl scene add-property mi-escena color --id tinte
wprs-ctl scene add-property mi-escena combo --id clima
wprs-ctl scene act mi-escena add_property '{"kind":"shortcut","id":"abrir"}'
```

## Cómo poner etiqueta, ayuda e icono

Dónde está: inspector de la propiedad, campos **Etiqueta**, **Ayuda** e **Icono**.

Pasos:

1. Elige la propiedad en la pestaña **Paquete**.
2. Escribe la **Etiqueta**: el nombre que se ve en el formulario de usuario.
3. Escribe la **Ayuda** si quieres un texto pequeño bajo el control. Es opcional.
4. Elige un **Icono** de la lista. Solo existen los iconos que ofrece la lista; un paquete no puede traer los suyos.

Si pones etiqueta y ayuda en varios idiomas, mira [Cómo traducir las etiquetas](#cómo-traducir-las-etiquetas-y-los-textos).

En la terminal: `translate` escribe una etiqueta o una ayuda; el icono y el resto se fijan con `set` sobre `/manifest/properties/ID/icon`. Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena slider --id velocidad
wprs-ctl scene set mi-escena /manifest/properties/velocidad/icon '"weather-clear"'
wprs-ctl scene set mi-escena /manifest/properties/velocidad/help '"Cuanto mas, mas rapido"'
```

## Cómo fijar el rango, el paso y el valor por defecto de un deslizador

Dónde está: inspector de una propiedad de tipo **Deslizador**. Arriba hay cuatro casillas: **Mínimo**, **Máximo**, **Paso** y **Valor por defecto**. Más abajo están **Decimales**, **Números enteros**, **Unidad**, **Casilla numérica** y **Escala logarítmica**.

Pasos:

1. Elige el deslizador en la pestaña **Paquete**.
2. Escribe el **Mínimo** y el **Máximo**. Los dos valores se aceptan tal como los escribes, aunque no caigan justo en un paso.
3. Escribe el **Paso**, lo que salta el deslizador cada vez. Si lo dejas vacío, el deslizador se mueve según sus decimales.
4. Escribe el **Valor por defecto**: lo que verá quien no toque el ajuste. Se recorta al rango.
5. Si quieres un cartel con la medida (por ejemplo «s» o «%»), escribe la **Unidad**.
6. Pon **Números enteros** para que solo valgan enteros, y **Decimales** (de 0 a 6) para fijar cuántos decimales se redondean y se muestran.
7. Pon **Casilla numérica** para que, además del deslizador, haya una casilla donde escribir el número.
8. Pon **Escala logarítmica** si el rango abarca varias potencias de diez (por ejemplo, de 0,1 a 100). Necesita un mínimo mayor que cero.

El valor que llega a la escena siempre está redondeado a los decimales y alineado al paso.

En la terminal: acción `property` (orden `wprs-ctl scene property`, con `--min`, `--max`, `--step`, `--default` y `--order`). Los demás campos (`decimals`, `integer`, `unit`, `editable`, `log`) se fijan con `set`. Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena slider --id velocidad
wprs-ctl scene property mi-escena velocidad --min 0 --max 10 --step 0.5 --default 2
wprs-ctl scene set mi-escena /manifest/properties/velocidad/unit '"x"'
wprs-ctl scene set mi-escena /manifest/properties/velocidad/decimals 1
wprs-ctl scene set mi-escena /manifest/properties/velocidad/editable true
```

## Cómo preparar los demás tipos

Cada tipo muestra en el inspector solo los campos que le tocan. Todos los tienen en común **Etiqueta**, **Ayuda**, **Grupo**, **Orden**, **Icono**, **Visible cuando** y **Activo cuando**.

- **Interruptor**: **Valor por defecto**, encendido o apagado. Tiene también **Valores por nivel de calidad**.
- **Color**: **Valor por defecto** con el selector de color. **Transparencia** hace que el usuario pueda elegir también la opacidad; sin ella el color es opaco.
- **Selección**: la lista **Opciones**, con el botón **Añadir**. Cada opción tiene un valor y una etiqueta; una opción con la etiqueta vacía muestra su valor. **Añadir** pone una opción con la etiqueta vacía y un valor que ninguna otra tiene (`option_3`...). El **Valor por defecto** es una caja con las opciones, nombradas por su etiqueta. Hay un máximo de 64 opciones, todas del mismo tipo de valor (textos, números o sí y no).
- **Texto**: **Valor por defecto**, **Longitud máxima** (de 1 a 1024 caracteres), **Varias líneas** y **Secreto**. Un texto secreto no tiene valor por defecto, el sistema lo guarda aparte y la escena lo ve vacío; solo lo recibe un contenido al que le hayas dado permiso de red.
- **Archivo**: **Archivos admitidos** (casillas **Imagen** y **Vídeo**) y **Recurso por defecto**, la imagen o vídeo de tu paquete que se usa mientras el usuario no elija otro.
- **Carpeta**: **Archivos admitidos**, **Elegir los archivos** (**Al azar** o **En orden**), **Cada** (los segundos entre un archivo y el siguiente, de 10 a 86400), **Máximo de archivos** (de 1 a 1000), **Qué puede leer el contenido** (**Un archivo cada vez** o **Todos los archivos**, esto último solo importa a las páginas web) y **Niveles de subcarpetas** (de 0 a 2).
- **Atajo**, **Botón**, **Etiqueta** y **Separador**: no tienen campos propios. La etiqueta de un **Separador** puede quedar vacía; entonces solo se ve la línea. Un **Botón** se explica en [Cómo usar un botón como disparador](#cómo-usar-un-botón-como-disparador).
- **Grupo**: tiene **Plegado**, para que en el formulario de usuario nazca cerrado.

Para que un archivo o una carpeta de usuario aparezca en la escena, enlaza el valor de una capa a la propiedad (mira [Cómo hacer que una capa siga una propiedad](#cómo-hacer-que-una-capa-siga-una-propiedad)). Un archivo elegido por el usuario también reemplaza al **Recurso por defecto** allí donde la escena lo use.

En la terminal: `set` sobre `/manifest/properties/ID/CAMPO`, por ejemplo el valor por defecto, las opciones o el límite de un texto. Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena combo --id clima
wprs-ctl scene set mi-escena /manifest/properties/clima/options '[{"value":"sol","label":"Sol"},{"value":"lluvia","label":"Lluvia"}]'
wprs-ctl scene set mi-escena /manifest/properties/clima/default '"sol"'
wprs-ctl scene add-property mi-escena text --id nombre
wprs-ctl scene set mi-escena /manifest/properties/nombre/max_len 20
wprs-ctl scene add-property mi-escena color --id tinte
wprs-ctl scene set mi-escena /manifest/properties/tinte/alpha true
wprs-ctl scene add-property mi-escena folder --id fotos
wprs-ctl scene set mi-escena /manifest/properties/fotos/every 120
```

## Cómo ordenar y agrupar las propiedades

Dónde está: cada fila de **Propiedades de usuario** tiene los botones **Subir**, **Bajar** y **Más**. El menú **Más** tiene **Cambiar el nombre…**, **Mover a un grupo** y **Quitar**.

Para ordenar:

1. Pulsa **Subir** o **Bajar** en la fila. La propiedad se mueve entre las de su mismo grupo (o entre las que no tienen grupo). **Subir** no está activo en la primera y **Bajar** no lo está en la última.

Para agrupar:

1. Crea una propiedad de tipo **Grupo** y ponle etiqueta.
2. En la fila de otra propiedad pulsa **Más**, **Mover a un grupo** y elige el grupo. La propiedad queda la última de ese grupo y la fila se sangra bajo él.
3. Para sacarla, **Más**, **Mover a un grupo**, **Sin grupo**. Queda justo después de su grupo.

También puedes elegir el grupo en el campo **Grupo** del inspector, que ofrece los grupos con su etiqueta. Un grupo no puede estar dentro de otro, así que para un grupo el menú **Mover a un grupo** no está activo. Si pones una condición a un grupo, vale para todos sus miembros.

El campo **Orden** del inspector es un número libre: las propiedades se ordenan por él y, si empatan, por el id. **Subir** y **Bajar** lo reescriben solos; no hace falta tocarlo a mano.

En la terminal: acción `move_property` (orden `wprs-ctl scene move-property DIR ID POSICIÓN`, con `--group`; un grupo vacío la saca del grupo). La posición es entre las propiedades del grupo, 0 la primera. Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena group --id aspecto --label Aspecto
wprs-ctl scene add-property mi-escena slider --id brillo
wprs-ctl scene add-property mi-escena color --id tinte
wprs-ctl scene move-property mi-escena brillo 0 --group aspecto
wprs-ctl scene move-property mi-escena tinte 1 --group aspecto
wprs-ctl scene move-property mi-escena tinte 0 --group ""
```

## Cómo cambiar el nombre o quitar una propiedad

Dónde está: fila de la propiedad, botón **Más**, entradas **Cambiar el nombre…** y **Quitar**.

Para cambiar el id:

1. Pulsa **Más** y **Cambiar el nombre…**. Se abre el diálogo **Cambiar el nombre de la propiedad**.
2. El diálogo avisa de que los enlaces, disparadores, grupos, condiciones y preajustes siguen al id elegido y que los scripts no se reescriben. Si algún script nombra la propiedad entre comillas, el diálogo lo lista: «Estos scripts la nombran y no se reescriben: ...». Esos scripts los corriges tú.
3. Escribe el id elegido y confirma el diálogo. El inspector sigue en la propiedad con el id elegido.

Para quitarla:

1. Pulsa **Más** y **Quitar**. Si algún script la nombra, antes se abre **Quitar la propiedad** con la lista de scripts; confirma el diálogo para seguir.
2. La propiedad desaparece junto con sus valores en los preajustes y su condición. Si era un grupo, sus miembros quedan sin grupo. Si la tenías elegida, el inspector pasa a los datos del paquete.
3. Si una capa, un disparador o una condición todavía la nombra, el editor no la quita y muestra un error que dice qué la nombra («... todavía nombra a ...»). Cambia primero ese enlace y vuelve a intentarlo.

En la terminal: acciones `rename_property` y `remove_property` (órdenes `wprs-ctl scene rename-property DIR ID OTRO` y `scene remove-property DIR ID`). Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena slider --id velocidad
wprs-ctl scene rename-property mi-escena velocidad rapidez
wprs-ctl scene remove-property mi-escena rapidez
```

## Cómo hacer que una capa siga una propiedad

Dónde está: en el inspector de una capa, el botón pequeño que hay a la derecha de cada valor que se puede enlazar (**◇** si es fijo). Si el valor ya sigue una propiedad, el botón muestra **⛓** y debajo hay otro botón que dice **Sigue una propiedad de usuario**, con el id de la propiedad detrás.

Pasos:

1. Elige la capa y busca el valor (la opacidad, el color, si se ve, la imagen de una máscara...). Solo tienen el botón los valores que admiten enlace.
2. Pulsa el botón ◇ y elige **Seguir una propiedad de usuario**. El submenú lista las propiedades que guardan un valor (no los botones, grupos, etiquetas ni separadores), con su etiqueta.
3. Elige la propiedad. El valor pasa a seguirla: el campo se sustituye por el botón **Sigue una propiedad de usuario · id**.
4. Para cambiar la propiedad o ajustar el enlace, pulsa ese botón o la entrada **Editar el origen…** del menú. El inspector muestra el campo **Propiedad de usuario** (una caja con las propiedades) y, según el valor, sus ajustes de escala y rango.
5. Para volver a un valor fijo, abre el menú del botón y elige **Valor fijo**.

El tipo de propiedad tiene que casar con el valor: un deslizador sirve para números, un interruptor para sí o no, un color para colores, un texto para textos, un archivo o una carpeta para imágenes y vídeos. Una propiedad que no existe es un problema de la lista de problemas del editor (mira [editor](editor.md)).

### Un valor distinto para cada opción

Una **Selección** o un **Interruptor** pueden dar un valor distinto de la capa para cada opción. Es lo que sirve para cambiar de imagen según una opción, por ejemplo la imagen de una capa, la textura de una máscara, de un efecto o de unas partículas, un vídeo o un color.

1. Enlaza el valor a la propiedad como arriba.
2. En el inspector aparece el grupo **Mapa**. Bajo él, una caja con las opciones que aún no tienen valor y el botón **Añadir**.
3. Elige una opción y pulsa **Añadir**. Aparece una fila con esa opción (en un interruptor, **Encendido** y **Apagado**) y el control del propio valor: la lista de imágenes, el selector de color, un interruptor.
4. Pon el valor de cada fila.

Una opción sin fila se queda con el valor de la propiedad tal cual lo elija el usuario. Para quitar una fila usa el botón **Valor por defecto** de su campo.

En la terminal: `set` con el puntero del valor y `{"prop": "ID"}`, con `"map"` si cada opción da un valor. Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena combo --id clima
wprs-ctl scene add-layer mi-escena solid --id cielo
wprs-ctl scene set mi-escena /scene/layers/0/visible '{"prop":"clima","map":{"option_1":true,"option_2":false}}'
wprs-ctl scene add-property mi-escena slider --id luz
wprs-ctl scene set mi-escena /scene/layers/0/opacity '{"prop":"luz"}'
```

## Cómo poner condiciones: mostrar y activar

Dónde está: inspector de la propiedad, campos **Visible cuando** y **Activo cuando**. Los dos son una casilla de texto; vacía muestra **Siempre**.

- **Visible cuando** oculta la propiedad mientras la condición no se cumple.
- **Activo cuando** deja la propiedad visible pero apagada (no se puede tocar) mientras la condición no se cumple.

Pasos:

1. Elige la propiedad que debe ocultarse o apagarse (una de las que dependen de otra).
2. Escribe la condición en **Visible cuando** o en **Activo cuando**.
3. Mientras escribes, el editor la comprueba en cada letra. Si falla, el motivo aparece en rojo bajo la casilla y no se guarda nada. Se guarda al salir de la casilla o al pulsar Intro, solo si la condición está bien.
4. Para quitarla, vacía la casilla.

Tres formas de escribirla:

- Una expresión: ids de otras propiedades, números, textos entre comillas simples, `true` y `false`, con `==`, `!=`, `<`, `<=`, `>`, `>=`, `&&`, `||`, `!` y paréntesis. Por ejemplo `clima == 'sol'` o `humor == 2 || velocidad > 1.5`. La ayuda de la casilla (al dejar el puntero encima) lo recuerda.
- Un mapa de valores como `{"clima": "sol"}`, que se cumple si todos coinciden.
- Nada, para que valga siempre.

Reglas: una expresión tiene como mucho 256 caracteres y 8 niveles de paréntesis. Una propiedad se compara solo con un valor de su mismo tipo, y `<` y similares solo con números. El editor rechaza ids que no existen, propiedades que no guardan valor (un botón, un grupo), condiciones que no dan sí o no y condiciones que dependen de sí mismas a través de otras propiedades.

Si pones la condición a un grupo, vale para todos sus miembros.

En la terminal: acción `condition` (orden `wprs-ctl scene condition DIR ID TEXTO`, con `--enabled` para **Activo cuando**; un texto vacío la quita). Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena combo --id clima
wprs-ctl scene add-property mi-escena slider --id viento
wprs-ctl scene condition mi-escena viento "clima == 'option_2'"
wprs-ctl scene condition mi-escena viento --enabled "clima != 'option_1'"
wprs-ctl scene condition mi-escena viento ""
```

## Cómo usar un botón como disparador

Un **Botón** no guarda un valor. Pulsarlo es un aviso que puede poner en marcha líneas de tiempo y sonidos.

Pasos:

1. Añade una propiedad de tipo **Botón** y ponle etiqueta.
2. Elige la línea de tiempo (o la capa de sonido) que debe arrancar con él. Su campo **Disparador** (en los sonidos, **Sonar con**) es una caja que ofrece, entre otras cosas, cada botón y cada propiedad que guarda un valor, con su etiqueta. Elige la tuya. Mira [animación](editor-animacion.md) y [partículas y sonido](editor-particulas-y-sonido.md).
3. A la derecha de la caja, el botón **Activar este disparador en la vista previa** hace lo mismo que el botón de usuario sin salir del editor. Su ayuda dice qué líneas de tiempo y qué sonidos arrancan («Dispara ... en la vista previa, como lo haría la escena, y arranca ...»), o que nada arranca todavía.
4. Elige **Sin fijar** en la caja para que no la dispare nada.

Un botón no se puede pulsar mientras su condición **Activo cuando**, o la de su grupo, no se cumple. El campo también nombra las propiedades que guardan un valor, no solo los botones.

En la terminal: el disparador se fija con `timeline` (campo `trigger`, con el valor `prop:ID`). La pregunta `fires` dice qué arranca un disparador. Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena button --id salto
wprs-ctl scene act mi-escena timeline '{"add":true,"id":"salta","duration":1,"trigger":"prop:salto"}'
wprs-ctl scene ask mi-escena fires '"prop:salto"'
```

## Cómo cambiar un valor según el nivel de calidad

Dónde está: inspector de un **Deslizador**, un **Interruptor** o una **Selección**, grupo **Valores por nivel de calidad**. Los demás tipos no cambian con la calidad y no lo muestran.

Sirve para que un ahorro de energía se active solo: en los niveles bajos la escena ve otro valor (en un deslizador, un tope; en una selección o un interruptor, exactamente ese valor), aunque el usuario haya elegido otro. El valor guardado no cambia.

Pasos:

1. Elige la propiedad.
2. En **Valores por nivel de calidad** hay una fila por nivel: **Q0, la más baja**, **Q1**, **Q2** y **Q3, la más alta**. Cada fila usa el control de la propiedad (el deslizador con su rango, la caja de opciones, el interruptor).
3. Fija el valor de los niveles que quieras. Un nivel con valor propio vale para él y para los más bajos hasta que otro nivel fije el suyo.
4. Para quitar el valor de un nivel, pulsa **Valor por defecto** en su fila.
5. Más abajo, **Valor vigente en cada nivel** lista lo que acaba valiendo cada nivel («Q1: 3», o «Q2: 3 (heredado)» si lo toma de otro). El nivel que usa la vista previa está en negrita.
6. Con el selector de calidad que hay en esa zona, eliges Q0 a Q3 y la vista previa se ve con los valores de ese nivel.

En la terminal: `set` y `remove` sobre `/manifest/properties/ID/quality/q0` (hasta `q3`). Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena slider --id particulas
wprs-ctl scene set mi-escena /manifest/properties/particulas/quality/q0 10
wprs-ctl scene set mi-escena /manifest/properties/particulas/quality/q1 30
wprs-ctl scene remove mi-escena /manifest/properties/particulas/quality/q0
```

Para ver la vista previa a un nivel desde la terminal está `wprs-ctl scene preview DIR --quality q1` (mira [cli](cli.md)).

## Cómo traducir las etiquetas y los textos

Las etiquetas, las ayudas, las etiquetas de las opciones y las de los preajustes son textos que pueden estar en varios idiomas. En el inspector estos campos tienen una fila por idioma.

Dónde está: inspector de una propiedad, un preajuste o el paquete, en cada campo de texto (**Etiqueta**, **Ayuda**...).

Pasos para añadir un idioma:

1. Debajo de las filas hay una casilla con el texto de ayuda **Idioma, como es o pt-BR**. Escribe el código del idioma (dos o tres letras, con región si hace falta: `es`, `pt-BR`).
2. Pulsa **Traducir**. Se añade una fila para ese idioma, que parte del texto del idioma por defecto del paquete. **Traducir** nunca pisa un texto que ya existe.
3. Escribe el texto de la fila. Al salir de la casilla se guarda.
4. Para quitar un idioma, pulsa **Quitar este idioma** al final de su fila. Un texto vacío también lo quita. El último texto de un título no se puede quitar.

Hasta que añadas un segundo idioma, el campo tiene un solo texto, en el idioma por defecto. Quien usa el fondo ve el texto de su idioma y, si no está, el del idioma por defecto. El idioma por defecto del paquete se elige en los datos del paquete (**Idioma por defecto**, mira [paquete](paquete.md)).

Si el paquete usa el formato antiguo de datos, aparece el aviso con el botón **Actualizar** y los textos en varios idiomas esperan a que lo uses.

### Textos por clave

Un texto que empieza por `@` toma sus palabras de una tabla, en vez de escribirlas en cada sitio. Así una frase se traduce una vez y se usa en muchos lugares.

Dónde está: **Detalles del paquete**, campo **Textos traducidos**.

La tabla muestra:

- La lista de idiomas. Cada idioma tiene un interruptor **En su propio archivo** que guarda sus textos en `locale/IDIOMA.json` en vez de en los datos del paquete.
- Cada clave como `@saludo`, con una casilla por idioma. Las que faltan se ven en rojo con el texto **Falta**, y arriba hay un resumen («Faltan N textos en algún idioma.» o «Cada clave tiene un texto en cada idioma.»). Una clave que ningún texto usa lleva **sin usar**. El botón **Quitar esta clave** borra la clave en todos los idiomas.
- Una fila al final para añadir una clave: la casilla **Clave nueva, como saludo**, una caja con el idioma, la casilla **Su texto** y el botón **Añadir**. La clave puede tener letras, cifras, `_`, `.` y `-` (hasta 64) y no puede empezar por `wprs.`.
- **Textos del motor**: las claves `wprs.*`, que el motor ya trae traducidas en inglés y español.

Para usar una clave:

1. Entra en el campo de texto (una etiqueta, un título...).
2. Pulsa el botón de al lado, **Usar un texto por clave**. Ofrece las claves de tu tabla y las del motor.
3. Elige una. El campo pasa a valer `@clave`.

En la terminal: `translate` (orden `wprs-ctl scene translate DIR PUNTERO IDIOMA TEXTO`, con `--add` y `--remove`) para un texto; la acción `manifest` con `text` para una clave de la tabla y con `locale` para guardar un idioma en su archivo. La pregunta `strings` muestra la tabla. Mira [acciones](acciones.md).

```sh
wprs-ctl scene new mi-escena --locale es
wprs-ctl scene add-property mi-escena slider --id velocidad --label Velocidad
wprs-ctl scene translate mi-escena /manifest/properties/velocidad/label en Speed --add
wprs-ctl scene act mi-escena manifest '{"text":{"key":"saludo","language":"es","text":"Hola"}}'
wprs-ctl scene act mi-escena manifest '{"text":{"key":"saludo","language":"en","text":"Hello"}}'
wprs-ctl scene set mi-escena /manifest/properties/velocidad/help '"@saludo"'
wprs-ctl scene ask mi-escena strings '{}'
wprs-ctl scene act mi-escena manifest '{"locale":{"language":"en","file":true}}'
```

## Cómo hacer un preajuste del paquete

Un preajuste da valores a varias propiedades de una vez. Quien usa el fondo lo elige en la lista **Preajustes** de los ajustes de la escena.

Dónde está: pestaña **Paquete**, bajo el título **Preajustes**.

Pasos:

1. Pulsa **Añadir un preajuste**. Se añade uno con id `preset_1` (el número siguiente que esté libre), vacío, y queda elegido en el inspector.
2. En el inspector pon la **Etiqueta** (con idiomas, como cualquier texto). El **Id** se puede cambiar si quieres.
3. Debajo está **Valores**. Hay una fila por cada propiedad que un preajuste puede fijar: no están los archivos, las carpetas, los atajos, los textos secretos, los botones, los grupos, las etiquetas ni los separadores. Cada fila usa el control de su propiedad. Una fila que el preajuste fija muestra su valor; las demás muestran el valor por defecto de la propiedad.
4. Cambia las filas que quieras: el preajuste fija solo esas.
5. Para que una propiedad deje de formar parte del preajuste, pulsa **Valor por defecto** en su fila.
6. Para borrar el preajuste, pulsa **Quitar** en su fila de la lista.

Los preajustes se guardan con el resto del paquete cuando guardas la carpeta (mira [editor](editor.md)). Si renombras o quitas una propiedad, sus valores en los preajustes se renombran o desaparecen con ella. Un valor que no cabe en la propiedad (fuera de rango, una opción que ya no existe) aparece en la lista de problemas, apuntando al valor concreto del preajuste.

Lo que el usuario puede hacer luego con ellos no es cosa tuya: elige uno, **Guardar como preajuste…** los valores actuales con un nombre propio, y puede cambiar el nombre, borrar, exportar e importar los suyos. Los preajustes de usuario no tocan los del paquete.

En la terminal: acción `add_preset` (orden `wprs-ctl scene add-preset`, con `--id` y `--label`) y `set` para llenarlo. Mira [acciones](acciones.md).

```sh
wprs-ctl scene add-property mi-escena slider --id velocidad
wprs-ctl scene add-property mi-escena color --id tinte
wprs-ctl scene add-preset mi-escena --id calmo --label Calmo
wprs-ctl scene set mi-escena /manifest/presets/0/values/velocidad 2
wprs-ctl scene set mi-escena /manifest/presets/0/values/tinte '"#3d5aff"'
wprs-ctl scene remove mi-escena /manifest/presets/0/values/tinte
```

## Cómo probar las propiedades como las verá el usuario

Dónde está: panel de la izquierda, pestaña **Probar**.

La pestaña dice: **Prueba los ajustes del paquete en la vista previa. Solo la vista previa los muestra: no se guarda nada y ninguna pantalla cambia.** Es el mismo formulario que ve quien usa el fondo, con los mismos controles, el mismo orden, los mismos grupos y las mismas condiciones, y con las etiquetas en el idioma de tu escritorio.

Pasos:

1. Abre la pestaña **Probar**. Si el paquete no tiene propiedades, dice **El paquete no declara propiedades para probar.**
2. Mueve un deslizador, cambia un color, pulsa un interruptor. La vista previa lo muestra al momento. Los campos que dependen de una condición aparecen y desaparecen como lo harán para el usuario.
3. Si el paquete tiene preajustes, elige uno en la caja **Preajuste** (la primera entrada, **Valores del paquete**, es sin preajuste). Los controles toman sus valores.
4. Para dejar la prueba, pulsa **Volver a los valores por defecto**.

Lo que pruebas no se guarda en el paquete ni cambia ninguna pantalla real. Para probar los niveles de calidad usa el selector de calidad de la vista previa (mira [Cómo cambiar un valor según el nivel de calidad](#cómo-cambiar-un-valor-según-el-nivel-de-calidad)).

En la terminal: `wprs-ctl scene preview DIR` con `--values ARCHIVO.json` (un objeto como `{"velocidad": 2}`) y `--preset ID`, que parte de un preajuste antes de aplicar los valores. No se guarda nada. Hace falta el demonio del escritorio. Mira [cli](cli.md).

## Preajustes de un vídeo

En un paquete de vídeo, la pestaña **Preajustes** del panel central lista los preajustes del vídeo, con **Añadir un preajuste** y **Quitar**. Un preajuste de vídeo fija con un clic la velocidad, la parte que se reproduce, el volumen, el zoom y la parte visible. Se llena en el inspector al elegirlo. Los datos del paquete (título, licencia, vista previa) están en [paquete](paquete.md).

## Cómo hacer un paquete de preajustes

Un paquete de preajustes no trae escena: da valores a las propiedades de otro paquete que el usuario ya tiene instalado. Es la manera de compartir ajustes bonitos de una escena ajena.

Dónde está: la página **Escenas** de la aplicación tiene el punto de partida **Preajuste**. Al crearlo, el editor abre el paquete y, en sus datos, la sección **Preajuste** aparece en lugar de **Paquetes necesarios**.

Pasos:

1. Crea el paquete desde el punto de partida **Preajuste**.
2. En la sección **Preajuste**, elige el **Paquete base** en la caja (lista los paquetes instalados; también puedes escribir su id a mano).
3. Escribe el **Rango de versiones** con el que es compatible (por ejemplo `^1.2`). Si el paquete base está instalado, la casilla sugiere «Compatible con» su versión y puedes dejarla vacía.
4. Pulsa **Usar como base**. La línea de arriba pasa a decir «Preajuste para ID, versión RANGO».
5. Si el paquete base está instalado, debajo aparece un control por cada ajuste suyo, con la misma forma que en su formulario: deslizadores, colores, interruptores, cajas de opciones y textos. Los ajustes que este preajuste fija salen en negrita y muestran su valor; los demás muestran el valor por defecto del paquete base. Cambia los que quieras. **Valor por defecto** a la derecha de una fila devuelve ese ajuste al paquete base.
6. Los ajustes de imagen o vídeo tienen un botón **Imagen o vídeo…** con el que eliges un archivo, que se copia dentro del paquete. Una fila sin archivo dice **Ningún archivo**.
7. Si el paquete base no está instalado, el editor dice «Instala el paquete base para elegir los ajustes que fija este preajuste.» Instálalo o rellena los valores desde la terminal.

Publicar como preajuste: debajo de los ajustes, la sección **Publicar como preajuste** lista los pasos que faltan, cada uno con una marca verde o un aviso: el paquete base elegido, algún valor o archivo, el título, la licencia, la imagen de vista previa y una revisión sin problemas. **Editar…** abre los datos del paquete para el título y la licencia. Para la vista previa, **Capturar la miniatura** dibuja el paquete base instalado con los valores del preajuste y guarda ese cuadro; **Grabar una vista previa** graba unos segundos como gif, o como webp con la casilla **WebP**; **Elegir una imagen…** usa un archivo tuyo. La miniatura y la grabación se dibujan sin sonido ni puntero: una escena que reacciona a ellos sale en reposo. Cuando todos los pasos están hechos, **Publicar…** abre el diálogo de construir, que empaqueta, firma, añade el paquete a un repositorio estático y lo publica. Es el mismo diálogo de cualquier paquete: pide construir, firmar y elegir repositorio; no hay una página solo para presets. Solo se puede dibujar el preajuste de una escena; el de una página web, una imagen o un video usa una imagen que elijas.

El título, la licencia, el origen y la imagen de vista previa son los de cualquier paquete (mira [paquete](paquete.md)). Un paquete de preajustes sin un paquete base válido tiene un problema en la lista hasta que elijas uno, y un paquete no puede ser su propia base. Compilar, firmar y publicar son las de cualquier paquete.

Los valores que el paquete base no conoce se dejan fuera, con un aviso, cuando el preajuste se aplica.

En la terminal: acción `manifest` con `preset` (campos `base`, `values`, `files` y `clear`), la orden `wprs-ctl scene new DIR --from preset` (con `--base`, `--range`, `--values`, `--license` y `--preview` para empezar con todo hecho), la acción `shot` con `base` para la vista previa y la pregunta `preset_steps` para ver qué falta. Una versión simple como `1.2.3` se convierte en el rango compatible `^1.2`. Mira [acciones](acciones.md).

```sh
wprs-ctl scene new mi-preajuste --from preset
wprs-ctl scene act mi-preajuste manifest '{"preset":{"base":{"id":"org.example.mi-escena","version":"1.2.3"},"values":{"tinte":"#3d5aff","velocidad":0.4}}}'
wprs-ctl scene act mi-preajuste manifest '{"preset":{"clear":["velocidad"]}}'
```
