import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: asking

    required property var host
    property var asked: ({})
    readonly property var open: Permisos.open(host.inputs)
    readonly property alias question: question

    onOpenChanged: {
        if (!open || asked[open.scene + "/" + open.key])
            return;
        asked[open.scene + "/" + open.key] = true;
        question.ask(open.scene, open.key, open.name, open.inputs);
    }

    PreguntaPermisos {
        id: question

        x: parent ? Math.round((parent.width - width) / 2) : 0
        y: parent ? Math.round(parent.height / 6) : 0
        onAnswered: (allow, deny) => {
            if (deny.length > 0)
                applicationWindow().showPassiveNotification(App.tr("“%1” plays without what you left out. You can allow it in its options, under More.", question.name));
        }
    }
}
