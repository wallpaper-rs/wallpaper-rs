pub const TEXTS: super::Texts = &[
    ("Your wallpaper server", "Tu servidor de fondos"),
    ("It is running", "Está andando"),
    ("It still needs setting up: four steps", "Falta configurarlo: cuatro pasos"),
    ("Your server is running.", "Tu servidor está andando."),
    (
        "Four steps are left before other people install wallpapers from here. To start, write the setup token.",
        "Faltan cuatro pasos para que otras personas instalen fondos desde aquí. Para empezar, escribe el código de arranque.",
    ),
    (
        "The server writes it in its messages the first time it starts.",
        "El servidor lo escribe en sus mensajes la primera vez que arranca.",
    ),
    (
        "This code does not work: it is copied wrong, it was used already or more than 24 hours went by. Ask for a new one and paste it again.",
        "Este código no sirve: está mal copiado, ya se usó o pasaron más de 24 horas. Pide uno nuevo y pégalo otra vez.",
    ),
    ("Where the code is", "Dónde está el código"),
    (
        "On the server's computer, open its folder and look at its messages with {}: look for the line «setup token». If more than 24 hours went by or you used it already, ask for a new one in that folder:",
        "En la computadora del servidor, abre su carpeta y mira sus mensajes con {}: busca la línea «setup token». Si pasaron más de 24 horas o ya lo usaste, pide uno nuevo en esa carpeta:",
    ),
    ("Start", "Empezar"),
    ("Set up your server", "Configurar tu servidor"),
    ("Setting up the server · step {} of 4", "Configurar el servidor · paso {} de 4"),
    ("Setup steps", "Pasos de la configuración"),
    ("Network", "Red"),
    ("Account", "Cuenta"),
    ("Who gets in", "Quién entra"),
    ("Change", "Cambiar"),
    ("Next", "Siguiente"),
    ("How do the others reach this server?", "¿Cómo llegan los demás a este servidor?"),
    ("Only at home", "Solo en casa"),
    (
        "For the devices of your network. The server makes its own certificate and announces itself on the local network; each phone recognises it by reading a QR code at the end.",
        "Para los equipos de tu red. El servidor crea su propio certificado y se anuncia en la red local; cada teléfono lo reconoce leyendo un código QR al final.",
    ),
    ("I have my certificate", "Tengo mi certificado"),
    (
        "If you already have a name of your own and a certificate for it.",
        "Si ya tienes un nombre propio y un certificado para él.",
    ),
    ("Certificate file", "Archivo del certificado"),
    ("Key file", "Archivo de la clave"),
    (
        "The path inside the server. Leave both empty when the program in front of this server holds the certificate.",
        "La ruta dentro del servidor. Deja las dos vacías si el certificado lo tiene el programa que está delante de este servidor.",
    ),
    (
        "If the certificate comes from an authority of your own and not from a public one, each phone needs it installed before connecting.",
        "Si el certificado es de una autoridad propia y no de una pública, cada teléfono necesita tenerla instalada antes de conectarse.",
    ),
    ("Check again", "Comprobar otra vez"),
    (
        "This server's public address starts with http://.",
        "La dirección pública de este servidor empieza con http://.",
    ),
    (
        "Phones and the app need https://. Change WPRS_PUBLIC_URL in the server's .env file, start it again and come back.",
        "Los teléfonos y la app necesitan https://. Cambia WPRS_PUBLIC_URL en el archivo .env del servidor, vuelve a arrancarlo y regresa.",
    ),
    (
        "With this certificate the others could not get in.",
        "Con este certificado los demás no podrían entrar.",
    ),
    (
        "Correct what is marked below or choose «Only at home».",
        "Corrige lo marcado abajo o elige «Solo en casa».",
    ),
    (
        "Write the path of this file too, or leave both empty.",
        "Escribe también la ruta de este archivo, o deja las dos vacías.",
    ),
    (
        "The server finds no file at this path, or cannot read it.",
        "El servidor no encuentra ningún archivo en esta ruta, o no puede leerlo.",
    ),
    (
        "This file holds no certificate in PEM format.",
        "Este archivo no tiene ningún certificado en formato PEM.",
    ),
    (
        "This file holds no private key in PEM format.",
        "Este archivo no tiene ninguna clave privada en formato PEM.",
    ),
    (
        "This certificate expired on {}. Renew it and check again.",
        "Este certificado venció el {}. Renuévalo y vuelve a comprobar.",
    ),
    ("This certificate is valid only from {}.", "Este certificado vale recién desde el {}."),
    (
        "This certificate is not for {}, the address of this server.",
        "Este certificado no es para {}, la dirección de este servidor.",
    ),
    (
        "This key is not the one of this certificate. Choose the one that came with it.",
        "Esta clave no es la de este certificado. Elige la que salió con él.",
    ),
    ("{2} {1}, {3}", "{1} de {2} de {3}"),
    ("Administrator account", "Cuenta de administrador"),
    (
        "With this account you get in to manage, review and upload.",
        "Con esta cuenta entras a administrar, revisar y subir.",
    ),
];
