pragma Singleton

import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: look

    property color noche: "#0a1322"
    property color nocheAlta: "#121e33"
    property color nocheLinea: "#2a3a57"
    property color bruma: "#b9c4d6"
    property color brumaBaja: "#8f9db4"
    property color luna: "#9cc3f5"
    property color lunaHonda: "#6f9fdc"
    property real velo: 0.8
    readonly property color escarcha: "#f4efe6"
    readonly property color ambar: "#f0c26b"
    readonly property color coral: "#ff9d8a"
    readonly property color brote: "#8fd6a8"
    readonly property color ambarHondo: "#7a5200"
    readonly property color coralHondo: "#a8361f"
    readonly property color broteHondo: "#1d7040"
    readonly property color veloColor: Qt.rgba(noche.r, noche.g, noche.b, velo)

    readonly property string letraTitulo: "Red Hat Display"
    readonly property string letraTexto: "Onest"

    property string escalon: "normal"
    readonly property bool compacto: escalon === "compacto"
    readonly property bool amplio: escalon === "amplio"

    readonly property real tPortada: compacto ? 52 : amplio ? 80 : 66
    readonly property real tPagina: compacto ? 42 : amplio ? 58 : 50
    readonly property real tGrande: compacto ? 26 : amplio ? 38 : 34
    readonly property real tMenu: compacto ? 22 : amplio ? 30 : 27
    readonly property real tFoco: compacto ? 23 : amplio ? 30 : 27
    readonly property real tFila: compacto ? 20 : amplio ? 24 : 22
    readonly property real tSeccion: 20
    readonly property real tAparte: compacto ? 16 : 18
    readonly property real tFiltro: 18
    readonly property real tExplica: 17
    readonly property real tBoton: 16.5
    readonly property real tNombre: 16
    readonly property real tCuerpo: 15
    readonly property real tPie: 13.5
    readonly property real tTecla: 12.5

    readonly property int e1: 4
    readonly property int e2: 8
    readonly property int e3: 12
    readonly property int e4: 16
    readonly property int e5: 24
    readonly property int e6: 32
    readonly property int e7: 48
    readonly property int e8: 64
    readonly property int margen: compacto ? 44 : amplio ? 72 : 56
    readonly property int margenArriba: compacto ? 30 : amplio ? 56 : 44
    readonly property int margenAbajo: compacto ? 24 : amplio ? 44 : 36

    readonly property int radio: 6
    readonly property int radioChico: 4
    readonly property int linea: 1
    readonly property int anillo: 2
    readonly property int anilloHueco: 3

    readonly property int menuAncho: compacto ? 272 : amplio ? 360 : 312
    readonly property int menuAlto: compacto ? 34 : amplio ? 52 : 46
    readonly property int aparteAlto: compacto ? 28 : amplio ? 42 : 38
    readonly property int tapa: compacto ? 200 : amplio ? 300 : 256
    readonly property int tapaFoco: compacto ? 300 : amplio ? 500 : 420
    readonly property int tapaRejilla: compacto ? 200 : amplio ? 260 : 232
    readonly property int huecoEstante: compacto ? 22 : amplio ? 32 : 28
    readonly property int altoControl: 40
    readonly property int altoBoton: compacto ? 38 : 42
    readonly property int altoFila: compacto ? 60 : amplio ? 80 : 74
    readonly property int altoCampo: 34
    readonly property int filasAncho: compacto ? 600 : amplio ? 740 : 660
    readonly property int explicaAncho: compacto ? 260 : amplio ? 340 : 300
    readonly property int atajosAlto: 26
    readonly property int teclaAncho: 28
    readonly property int teclaAlto: 26
    readonly property int teclaRadio: 5
    readonly property int teclaIcono: 13
    readonly property int logoLetra: 17
    readonly property int logoLuna: 14
    readonly property int marcaAncho: 16
    readonly property int marcaAlto: 4
    readonly property int menuSangria: 30
    readonly property int avisoAncho: compacto ? 340 : 480
    readonly property int arribaAlto: 52
    readonly property int tArriba: 18
    readonly property int lateralIzq: 280
    readonly property int barraLienzoAlto: 44
    readonly property int llaveAncho: 40
    readonly property int llaveAlto: 22
    readonly property int llavePunto: 16

    readonly property int paso: 180
    readonly property var pasoCurva: [0.22, 1, 0.36, 1, 1, 1]

    readonly property real tCuenta: 13
    readonly property real tQuitable: 14
    readonly property int icono: 18
    readonly property int filtroLado: 12
    readonly property int filtroHueco: 7
    readonly property int seccionesLado: 14
    readonly property int marcaAncho: 16
    readonly property int marcaAlto: 4
    readonly property int marcaAbajo: 2
    readonly property int buscarAncho: 300
    readonly property int buscarLado: 14
    readonly property int buscarHueco: 10
    readonly property int quitableAlto: 32
    readonly property int quitableBoton: 22
    readonly property int quitableHueco: 6
    readonly property int usoAlto: 26
    readonly property int usoHueco: 6
    readonly property int marcaTapa: 10
    readonly property int elegirLado: 26
    readonly property int filete: 5
    readonly property int notaArriba: 3
    readonly property int pieArriba: 5
    readonly property int accionesArriba: 14
    readonly property real veloTapa: 0.42
    readonly property real veloTapaSobre: 0.2
    readonly property real veloTapaElegida: 0.15
    readonly property real veloElegir: 0.6
    readonly property real tapaApagada: 0.6
    readonly property real apretado: -0.02

    readonly property string tones: [noche, nocheAlta, nocheLinea, escarcha, bruma, brumaBaja, luna].map(String).join(",")
    readonly property Looks bridge: Looks {}

    function tone(set) {
        for (const name of ["noche", "nocheAlta", "nocheLinea", "bruma", "brumaBaja", "luna", "lunaHonda", "velo"]) {
            if (set[name] !== undefined)
                look[name] = set[name];
        }
    }

    function mix(a, b, amount) {
        return Qt.rgba(a.r + (b.r - a.r) * amount, a.g + (b.g - a.g) * amount, a.b + (b.b - a.b) * amount, 1);
    }

    function step(width, height) {
        return height <= 800 ? "compacto" : width >= 1800 && height >= 1000 ? "amplio" : "normal";
    }

    onTonesChanged: bridge.paint(tones)
    Component.onCompleted: {
        bridge.dress(letraTexto, tCuerpo);
        bridge.paint(tones);
    }
}
