import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PublishError {
    id: fault

    required property var where
    required property string place
    readonly property alias button: main
    readonly property var publish: where.publish
    readonly property var shown: publish.fault ?? ({})
    readonly property string kind: shown.kind ?? ""
    readonly property string next: Publishing.nextVersion(publish.version)

    visible: publish.fault !== null && publish.at === 3 && publish.place === place
    title: shown.title ?? ""
    text: shown.text ?? ""
    details: shown.details ?? ""
    Keys.onReturnPressed: main.clicked()

    QQC2.Button {
        id: main

        highlighted: true
        visible: text !== "" && (fault.kind !== "version" || fault.publish.editing)
        text: ({
                "version": App.tr("Upload as %1", fault.next),
                "registro": App.tr("Register it"),
                "red": App.tr("Try again"),
                "otro": App.tr("Try again"),
                "raiz": App.tr("Sign it now"),
                "indice": App.tr("Publish again")
            })[fault.kind] ?? ""
        onClicked: {
            if (fault.kind === "version")
                fault.publish.page.change("/manifest/version", fault.next);
            if (fault.kind === "version")
                fault.publish.reload();
            if (fault.kind === "registro")
                fault.publish.work.register(true);
            else if (fault.kind === "raiz")
                fault.where.repo.signRoot();
            else if (fault.kind === "indice")
                fault.where.repo.republish();
            else
                fault.publish.work.run();
        }
    }

    KeyCap {
        visible: main.visible
        key: "Intro"
    }

    QQC2.Button {
        visible: fault.kind === "red" && fault.publish.editing
        flat: true
        text: App.tr("Save the file")
        onClicked: {
            fault.where.pick("file");
            fault.publish.work.run();
        }
    }
}
