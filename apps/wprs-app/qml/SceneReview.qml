import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: review

    required property var page
    property var report: ({})
    property var signature: null
    property string built: ""
    readonly property var findings: report.findings ?? []
    readonly property var platforms: report.platforms ?? ({})
    readonly property string verdict: (signature ?? report.signature ?? {}).state ?? "unsigned"

    function ask(query) {
        return JSON.parse(review.page.editor.rig("", JSON.stringify(query)) || "{}");
    }

    function refresh() {
        report = ask({
                "review": {}
            });
        signature = null;
    }

    function check() {
        signature = ask({
                "signature": {
                    "path": built,
                    "keys": [key.text]
                }
            });
    }

    function plays(name) {
        const on = platforms[name] ?? {};
        if (on.refused)
            return App.tr("refused: %1", on.refused);
        return (on.without ?? []).length > 0 ? App.tr("plays without %1", on.without.join(", ")) : App.tr("plays");
    }

    function count(level) {
        return findings.filter(f => f.level === level).length;
    }

    title: App.tr("Review the package")
    padding: Kirigami.Units.largeSpacing
    preferredWidth: Kirigami.Units.gridUnit * 34
    preferredHeight: Kirigami.Units.gridUnit * 30
    standardButtons: Kirigami.Dialog.Close
    onAboutToShow: refresh()

    ColumnLayout {
        spacing: Kirigami.Units.smallSpacing

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: review.findings.length === 0 ? App.tr("Nothing to report: the package passes every check.") : App.tr("%1 errors, %2 warnings, %3 notes", review.count("error"), review.count("warning"), review.count("note"))
        }

        Repeater {
            model: review.findings

            delegate: QQC2.ItemDelegate {
                required property var modelData

                Layout.fillWidth: true
                text: modelData.message
                icon.name: modelData.level === "error" ? "data-error" : modelData.level === "warning" ? "data-warning" : "data-information"
                QQC2.ToolTip.text: modelData.file
                QQC2.ToolTip.visible: hovered && modelData.file !== ""
                onClicked: {
                    review.page.jump(modelData.pointer);
                    review.close();
                }
            }
        }

        Kirigami.Heading {
            level: 4
            text: App.tr("Where it plays")
        }

        Repeater {
            model: Object.keys(review.platforms)

            delegate: QQC2.Label {
                required property string modelData

                Layout.fillWidth: true
                wrapMode: Text.Wrap
                text: modelData + ": " + review.plays(modelData)
            }
        }

        QQC2.Label {
            Layout.fillWidth: true
            visible: (review.report.build_adds ?? []).length > 0
            wrapMode: Text.Wrap
            text: App.tr("Building adds to the features it needs: %1", (review.report.build_adds ?? []).map(f => f.name).join(", "))
        }

        Kirigami.Heading {
            level: 4
            text: App.tr("Signature")
        }

        QQC2.Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: ({
                    "unsigned": App.tr("Not signed: build it with an author key to sign it."),
                    "unchecked": App.tr("Signed; give a public key to check it."),
                    "valid": App.tr("Signature valid, made by %1", ((review.signature ?? {}).signers ?? []).join(", ")),
                    "invalid": App.tr("Signature not valid: %1", (review.signature ?? {}).error ?? "")
                })[review.verdict] ?? ""
        }

        RowLayout {
            Layout.fillWidth: true

            QQC2.TextField {
                Layout.fillWidth: true
                placeholderText: App.tr("Package file; the folder when empty")
                text: review.built
                onTextEdited: review.built = text
            }

            QQC2.Button {
                icon.name: "document-open"
                text: App.tr("Choose…")
                onClicked: packages.open()
            }
        }

        RowLayout {
            Layout.fillWidth: true

            QQC2.TextField {
                id: key

                Layout.fillWidth: true
                placeholderText: App.tr("Public key of the author")
            }

            QQC2.Button {
                icon.name: "document-open"
                text: App.tr("Choose…")
                onClicked: keys.open()
            }

            QQC2.Button {
                text: App.tr("Check the signature")
                icon.name: "security-high"
                enabled: key.text !== ""
                onClicked: review.check()
            }
        }
    }

    FileDialog {
        id: packages

        nameFilters: [App.tr("Packages (*.wpak)")]
        onAccepted: review.built = App.path(selectedFile)
    }

    FileDialog {
        id: keys

        nameFilters: [App.tr("Public keys") + " (*.pub)"]
        onAccepted: key.text = App.path(selectedFile)
    }
}
