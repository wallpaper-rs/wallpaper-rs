import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

PanelChico {
    id: keying

    property var shown: null
    readonly property var rows: Object.keys(shown?.authors ?? {}).reduce((all, author) => all.concat(shown.authors[author].map(fingerprint => ({
                                "author": author,
                                "fingerprint": fingerprint
                            }))), [])

    signal revoke(string fingerprint)

    title: App.tr("Author keys")
    width: Look.panelChico * 3 / 2
    quiet: App.tr("Close")

    ColumnLayout {
        width: keying.width - 2 * Look.e4

        Repeater {
            model: keying.rows

            delegate: RowLayout {
                required property var modelData
                readonly property bool gone: (keying.shown?.revoked_keys ?? []).includes(modelData.fingerprint)

                QQC2.Label {
                    Layout.fillWidth: true
                    wrapMode: Text.WrapAnywhere
                    text: modelData.author + " · " + modelData.fingerprint + (gone ? " · " + App.tr("revoked") : "")
                }

                QQC2.Button {
                    text: App.tr("Revoke…")
                    enabled: !parent.gone
                    onClicked: keying.revoke(modelData.fingerprint)
                }
            }
        }

        RowLayout {
            QQC2.TextField {
                id: other

                Layout.fillWidth: true
                placeholderText: App.tr("Fingerprint of another key, such as one an author registered online")
            }

            QQC2.Button {
                text: App.tr("Revoke…")
                enabled: other.text.trim() !== ""
                onClicked: keying.revoke(other.text.trim())
            }
        }
    }
}
