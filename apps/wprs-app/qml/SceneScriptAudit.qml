import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: audit

    required property var page
    required property string path
    property var report: ({})
    readonly property var missing: report.undeclared ?? ({})
    readonly property var extra: report.unused ?? ({})

    function words(found) {
        const kinds = [["events", App.tr("events")], ["uses", App.tr("parts")], ["params", App.tr("settings")]];
        return kinds.filter(kind => (found[kind[0]] ?? []).length > 0).map(kind => App.tr("%1: %2", kind[1], found[kind[0]].join(", "))).join(" · ");
    }

    spacing: 0

    QQC2.Label {
        Layout.fillWidth: true
        visible: audit.report.synced === true
        text: App.tr("What the code uses matches what the scene declares")
        wrapMode: Text.Wrap
        opacity: 0.7
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: audit.words(audit.missing) !== ""
        text: App.tr("Used but not declared: %1", audit.words(audit.missing))
        wrapMode: Text.Wrap
        color: Look.ambar
    }

    QQC2.Label {
        Layout.fillWidth: true
        visible: audit.words(audit.extra) !== ""
        text: App.tr("Declared but not used: %1", audit.words(audit.extra))
        wrapMode: Text.Wrap
        color: Look.ambar
    }

    QQC2.Button {
        visible: audit.report.synced === false
        icon.name: "document-edit"
        text: App.tr("Declare what it uses")
        onClicked: audit.page.run("script", {
                "declare": {
                    "path": audit.path,
                    "params": (audit.missing.params ?? []).length > 0
                }
            })
    }
}
