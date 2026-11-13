pub const TEXTS: super::Texts = &[
    (
        "This server is run by {}. By creating an account you accept these terms.",
        "Este servidor lo opera {}. Al crear una cuenta aceptas estos términos.",
    ),
    ("Your content", "Tu contenido"),
    (
        "You keep your rights. You confirm that you may share what you upload under the licence you declare, and you allow the operator to store, sign and distribute it in the package repository of this server.",
        "Conservas tus derechos. Confirmas que puedes compartir lo que subes con la licencia que declaras, y permites que quien opera el servidor lo guarde, lo firme y lo distribuya en el repositorio de fondos de este servidor.",
    ),
    ("Rules", "Reglas"),
    ("content policy", "política de contenido"),
    (
        "Uploads follow the {}. Do not upload illegal content, malware or work you have no right to share.",
        "Las subidas siguen la {}. No subas contenido ilegal, malware ni obras que no tengas derecho a compartir.",
    ),
    (
        "Moderators may reject, remove or revoke content. Every restriction comes with a statement of reasons on your account page. Three strikes within twelve months suspend uploads; repeated or serious abuse can suspend the account.",
        "Los moderadores pueden rechazar, retirar o revocar contenido. Cada restricción trae su explicación de motivos en la página de tu cuenta. Tres avisos en doce meses suspenden las subidas; el abuso repetido o grave puede suspender la cuenta.",
    ),
    ("Notices and appeals", "Avisos y apelaciones"),
    (
        "Anyone can send a {}. Authors can reply with a {} or write to {}.",
        "Cualquiera puede enviar un {}. Quien subió el fondo puede responder con un {} o escribir a {}.",
    ),
    ("No warranty", "Sin garantía"),
    (
        "The service is provided as is, without any warranty of availability or fitness.",
        "El servicio se ofrece tal como está, sin garantía de disponibilidad ni de que sirva para un fin concreto.",
    ),
    ("Controller: {}, contact {}.", "Responsable: {}, contacto {}."),
    ("What is stored", "Qué se guarda"),
    (
        "Your handle, a password hash (Argon2id) and your role.",
        "Tu usuario, un hash de tu contraseña (Argon2id) y tu rol.",
    ),
    (
        "Your devices under generic names (Device 1, Device 2…) with their public keys, and hashes of your tokens, never the tokens.",
        "Tus dispositivos con nombres genéricos (Dispositivo 1, Dispositivo 2…) y sus claves públicas, y hashes de tus tokens, nunca los tokens.",
    ),
    (
        "Packages you upload and moderation records about them.",
        "Los fondos que subes y los registros de moderación sobre ellos.",
    ),
    (
        "Synced library, playlists and settings. With end-to-end encryption, the default, the server only holds ciphertext it cannot read.",
        "Biblioteca, listas y ajustes sincronizados. Con el cifrado de extremo a extremo, que es lo normal, el servidor solo guarda texto cifrado que no puede leer.",
    ),
    ("What is not", "Qué no se guarda"),
    (
        "No tracking, analytics or third-party resources, and one strictly necessary session cookie. IP addresses and user agents are not logged unless the operator turns on abuse logging, which keeps them at most {} days.",
        "Sin rastreo, sin analíticas y sin recursos de terceros, y una sola cookie de sesión, la estrictamente necesaria. Las direcciones IP y los agentes de usuario no se registran salvo que quien opera el servidor active el registro de abusos, que los guarda como máximo {} días.",
    ),
    ("How long", "Por cuánto tiempo"),
    (
        "Sessions 30 days, device codes 10 minutes, device messages 7 days, deleted sync items 180 days, the name, email and signature of a legal notice {} days, a package withdrawn by the moderators {} days, as evidence and for a counter-notice. Everything else until you delete it or your account.",
        "Las sesiones 30 días, los códigos de dispositivo 10 minutos, los mensajes entre dispositivos 7 días, lo borrado de la sincronización 180 días, el nombre, el correo y la firma de un aviso legal {} días, y un fondo retirado por los moderadores {} días, como prueba y para un contra-aviso. Todo lo demás hasta que lo borres tú o borres tu cuenta.",
    ),
    ("Your rights", "Tus derechos"),
    ("account page", "página de tu cuenta"),
    (
        "Download or delete all your data from your {}. For anything else write to {}.",
        "Descarga o borra todos tus datos desde la {}. Para cualquier otra cosa escribe a {}.",
    ),
    (
        "Wallpapers only: images, videos and scenes in .wpak packages with a preview image.",
        "Solo fondos: imágenes, videos y escenas en paquetes .wpak con una imagen de vista previa.",
    ),
    ("Accepted licences: {}.", "Licencias aceptadas: {}."),
    (
        "Mature content is accepted only when the package rates it mature in its manifest (\"content\": {\"rating\": \"mature\"} with its reasons): it carries that label in the catalog, in search and in the API, and clients and this site blur its previews until the viewer chooses to show them. Mature content without that rating is removed.",
        "El contenido maduro solo se acepta cuando el paquete lo clasifica como maduro en su manifiesto (\"content\": {\"rating\": \"mature\"} con sus motivos): lleva esa etiqueta en el catálogo, en la búsqueda y en la API, y los clientes y este sitio difuminan sus vistas previas hasta que quien mira decide mostrarlas. El contenido maduro sin esa clasificación se retira.",
    ),
    (
        "No illegal content, no sexual content involving minors, no malware, no impersonation.",
        "Nada de contenido ilegal, nada de contenido sexual con menores, nada de malware y nada de suplantación.",
    ),
    (
        "Upload only your own work or work whose licence allows sharing, and credit its authors.",
        "Sube solo obra tuya u obra cuya licencia permita compartirla, y da crédito a sus autores.",
    ),
    (
        "Content converted from other formats or products needs proof that you may share it.",
        "El contenido convertido desde otros formatos o productos necesita una prueba de que puedes compartirlo.",
    ),
    (
        "Packages that flash or show fast patterns must set \"photosensitive\": true in their wallpaper-rs.wpak/2 manifest: they can trigger seizures in people with photosensitive epilepsy, and clients warn before showing them.",
        "Los paquetes con destellos o patrones rápidos deben poner \"photosensitive\": true en su manifiesto wallpaper-rs.wpak/2: pueden provocar convulsiones en personas con epilepsia fotosensible, y los clientes avisan antes de mostrarlos.",
    ),
];
