import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Column {
    id: details

    required property var repo
    required property string name
    readonly property bool broken: !!repo.broken || !!repo.last_error
    readonly property alias row: row
    readonly property alias remove: remove
    readonly property alias remover: remover

    spacing: Look.e3

    Fila {
        id: row

        width: parent.width
        text: details.name
        kind: "despliega"
        note: details.repo.bundled ? App.tr("Included on this computer") : (/^[a-z]+:\/\/([^/?#]+)/i.exec(details.repo.url) ?? ["", details.repo.url])[1]
        number: String(details.repo.packages)
        status: details.broken ? App.tr("Needs a look") : details.repo.stale ? App.tr("Out of date") : App.tr("Up to date")
        statusTone: details.broken ? "coral" : details.repo.stale ? "ambar" : "brote"
    }

    Column {
        visible: row.open
        leftPadding: Look.e5
        spacing: Look.e3

        Estado {
            visible: details.broken
            text: details.repo.broken ? App.tr("The stored copy cannot be read: check for news to fetch it again.") : details.repo.last_error ?? ""
            tone: "coral"
            icon: "warning"
        }

        Repeater {
            model: details.repo.warnings

            Estado {
                required property var modelData

                text: Labels.value(modelData)
                tone: "ambar"
                icon: "warning"
            }
        }

        BrowseTerm {
            text: App.tr("Last check · %1", details.repo.checked ? App.time(details.repo.checked) : App.tr("never"))
        }

        BrowseTerm {
            visible: details.repo.hosts.length > 0
            text: App.tr("Contact · %1", details.repo.hosts.join(", "))
        }

        BrowseTerm {
            visible: details.repo.fingerprints.length > 0
            small: true
            text: App.tr("Fingerprint · %1", details.repo.fingerprints.join(", "))
        }

        Row {
            spacing: Look.e3

            QQC2.Button {
                text: App.tr("Check for news")
                icon.name: "arrows-clockwise"
                onClicked: App.act("Refresh", [details.repo.repo_id], "Repos1")
            }

            QQC2.Button {
                id: remove

                visible: !details.repo.bundled
                text: App.tr("Remove the repository…")
                icon.name: "trash"
                onClicked: remover.open()
            }

            BrowseTerm {
                anchors.verticalCenter: parent.verticalCenter
                visible: !!details.repo.bundled
                text: App.tr("The examples stay; you can hide what you do not want from your library.")
            }
        }
    }

    PanelChico {
        id: remover

        opener: remove
        title: App.tr("Remove “%1”?", details.name)
        text: App.tr("You will no longer see its wallpapers here.")
        principal: App.tr("Remove")
        principalIcon: "trash"
        borra: true
        onAccepted: App.act("Remove", [details.repo.repo_id, also.checked], "Repos1")

        QQC2.CheckBox {
            id: also

            text: App.tr("Also uninstall its packages")
        }
    }
}
