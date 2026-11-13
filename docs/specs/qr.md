# Código QR de conexión

Un código «conectar a este servidor» lleva la dirección del servidor, la huella de su certificado cuando el servidor usa uno propio (autofirmado), y una invitación si hay una. No lleva nada secreto: la huella es pública (cualquiera que abra la conexión TLS la ve) y el código de invitación es el mismo que ya va en el enlace de invitación.

El módulo compartido es el crate `wprs-qr`. Construye el texto, lo valida de forma estricta y dibuja el código. No lee imágenes: leer con la cámara es cosa del teléfono.

## Texto

```
wprs://connect/1?server=<dirección>[&fp=<huella>][&invite=<código>]
```

- Versión: el `1` de la ruta. Otro número se rechaza con «versión no admitida»; lo que no empieza por `wprs://connect/` se rechaza como «no es un código de conexión».
- Los campos van en ese orden, sin repetirse, sin otros, sin espacios ni saltos de línea al final. Máximo 256 caracteres.
- `server`: `https://` + host + `:puerto` opcional + ruta opcional (segmentos de letras, cifras y `._~-`, sin `..`, sin segmentos vacíos). El host va en minúsculas: nombre DNS (etiquetas de `a-z0-9-`), IPv4 o IPv6 entre corchetes. Sin usuario, sin `?`, sin `#`, sin barra final. Solo `https`, porque el código lleva una huella de certificado.
- `fp`: opcional. Un servidor autofirmado (`WPRS_TLS=self-signed`) siempre la lleva; uno cuyo certificado se valida de la forma normal (por ejemplo, con su propio certificado de una autoridad) va sin ella y el teléfono lo valida como cualquier sitio. Es el SHA-256 del certificado en DER, 64 dígitos hexadecimales en minúsculas (lo que imprime `wprs-server fingerprint`).
- `invite`: de 8 a 64 caracteres de `A-Za-z0-9_-`. El servidor genera 32 hexadecimales.
- No hay codificación por porcentaje: lo que no cabe en esa gramática no se puede llevar.

El enlace de invitación (`<servidor>/register?invite=<código>`) no se pone en el código tal cual: el código lleva `invite` y quien lo lee reconstruye el enlace con `Link::invitation_url`. Así el teléfono sabe a qué servidor pertenece la invitación y qué huella comparar.

## Dibujo

- Nivel de corrección M, tamaño automático (versión 1 a 40), zona silenciosa de 4 módulos.
- Matriz: `Code::size()` y `Code::dark(x, y)`.
- SVG: `Code::svg()` es un único `<path>` con `fill="currentColor"` y sin fondo, así toma el color del texto que lo rodea; el fondo es el de la página. Una pantalla clara con texto oscuro y una oscura con texto claro lo dibujan bien; si el escáner del otro lado no lee el negativo, hay que usar una superficie clara.
- Terminal: `Code::blocks(light_terminal)` usa medios bloques (dos filas por línea). Por defecto se dibuja para terminales oscuros (los módulos claros son el bloque), que los lectores aceptan; con `light_terminal` se invierte.

## Dónde se usa

| Quién | Cómo |
| --- | --- |
| `wprs-server` | `qr::connect_svg(server, fingerprint, invite)` devuelve el SVG en línea para las páginas; `qr::own_connect_svg` lo arma para cualquiera de las dos formas de correr el servidor: con la huella si es autofirmado, sin ella si no |
| `wprs-ctl` | `wprs-ctl connect-code SERVIDOR [--fingerprint HUELLA] [--invite CÓDIGO] [--draw] [--light]` imprime el texto y, con `--draw`, el código con bloques |
| `wprs-android` | `Native.connectCode(texto)` solo analiza: responde un JSON con `server`, `fingerprint` (nulo si no hay) e `invite`, o el error con su mensaje |

## Pruebas

`cargo test -p wprs-qr`: ida y vuelta del texto, entradas hostiles (campos repetidos, desordenados, mayúsculas, caracteres de control, longitud, puertos, rutas), una matriz de referencia completa y los patrones fijos del estándar (buscadores, temporización, módulo oscuro) para tres tamaños. Cada dibujo se decodifica con un lector independiente (`rqrr`, solo en pruebas) y debe devolver el mismo texto.
