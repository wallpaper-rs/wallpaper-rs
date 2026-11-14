# Cambiar de fondo con el teclado

Con un atajo puedes recorrer tus fondos sin abrir la aplicación: aparece una ventana sobre el
escritorio con el fondo que pondrías, cómo entraría y los que vienen después. Nada cambia hasta que
lo aplicas.

## En la ventana

- **← y →**: el anterior y el siguiente. Si la pantalla reproduce una lista, se recorre esa lista;
  si no, tu biblioteca.
- **Intro**: aplica el que estás mirando, con la transición que dice la ventana.
- **Esc**: cancela; lo que tenías puesto sigue igual.
- **Tab**: con varias pantallas, elige cuál cambia.
- Si no tocas ninguna tecla durante 30 segundos, la ventana se cierra sola y no cambia nada.

Cada vez que pasas a otro fondo la ventana enseña durante cinco segundos cómo entraría: el fundido,
el corte o la transición de su autor. Si pediste menos movimiento (o lo pide tu sistema) no se
reproduce nada y la ventana lo dice: al aplicar habrá un fundido simple. Las escenas y los vídeos se
ven como una imagen quieta; se mueven al aplicarlos.

Volver a pulsar el atajo con la ventana abierta pasa al siguiente: nunca se abre una segunda
ventana. La bandeja del sistema y la terminal abren la misma ventana.

## Activar el atajo

```sh
wprs-ctl config shortcuts.enabled true
```

El programa pide al escritorio dos atajos (propone `Meta+Alt+N` para el siguiente y `Meta+Alt+P`
para el anterior; se cambian con `shortcuts.next` y `shortcuts.previous`). En Plasma, GNOME y
Hyprland el escritorio los registra y puede mostrarte su propio diálogo para elegir otras teclas.

## Donde el atajo no se registra solo

En Sway y los demás compositores wlroots, y en X11, el escritorio no ofrece atajos a los programas.
Ata tú las órdenes a teclas; la ventana se abre igual:

```sh
bindsym $mod+Alt+n exec wprs-ctl next --preview
bindsym $mod+Alt+p exec wprs-ctl prev --preview
```

En X11 haz lo mismo con el gestor de atajos de tu entorno (o `xbindkeys`). Con `--output NOMBRE`
la ventana recorre solo esa pantalla y se abre en ella.

## Límites

- Los fondos que se recorren quedan fijos al empezar: lo que añadas o quites mientras la ventana
  está abierta aparece la próxima vez. De la biblioteca se toman hasta 2000.
- Con varias pantallas y ninguna elegida, todas las pantallas recorren a la vez y la ventana se abre
  donde está el puntero; con Tab eliges una.
- El escritorio puede no aceptar las teclas propuestas (por ejemplo, si ya las usa otra cosa): elige
  otras en su diálogo o ata `wprs-ctl next --preview` a mano.
- Las pantallas se nombran por su conector (`DP-1`, `HDMI-A-1`).
- La ventana no muestra el autor ni el repositorio del fondo: eso está en la biblioteca.
