import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: avisos

    property var queue: []
    property var shown: null
    readonly property alias aviso: aviso

    signal acted(var notice)

    function show(notice) {
        queue = queue.concat([notice]);
        if (shown === null)
            next();
    }

    function next() {
        shown = queue.length > 0 ? queue[0] : null;
        queue = queue.slice(1);
        if (shown !== null)
            later.restart();
    }

    function close() {
        later.stop();
        next();
    }

    implicitWidth: aviso.implicitWidth
    implicitHeight: shown ? aviso.implicitHeight : 0
    Accessible.role: Accessible.Grouping

    Aviso {
        id: aviso

        visible: avisos.shown !== null
        width: avisos.width > 0 ? Math.min(avisos.width, implicitWidth) : implicitWidth
        text: avisos.shown?.text ?? ""
        tone: avisos.shown?.tone ?? ""
        action: avisos.shown?.action ?? ""
        onActed: {
            const notice = avisos.shown;
            avisos.close();
            avisos.acted(notice);
        }
        onClosed: avisos.close()
        onHeldChanged: held ? later.stop() : avisos.shown && later.restart()
    }

    Timer {
        id: later

        interval: Look.avisoDura
        onTriggered: avisos.close()
    }
}
