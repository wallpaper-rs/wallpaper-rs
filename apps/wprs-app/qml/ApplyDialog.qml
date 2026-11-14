import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.Dialog {
    id: dialog

    property string uid
    property var presets: []
    property bool flashing
    property var exposes: []
    property bool known
    property bool spread
    property bool sounds
    readonly property var fits: App.schema.fits ?? ["fill", "fit", "stretch", "center", "tile"]

    function show(uid) {
        dialog.uid = uid;
        dialog.presets = [];
        dialog.flashing = false;
        dialog.exposes = [];
        dialog.known = false;
        dialog.spread = false;
        dialog.sounds = false;
        sound.currentIndex = 0;
        fit.currentIndex = 0;
        App.query("GetItem", [uid], "Library1", reply => {
            if (!dialog)
                return;
            if (dialog.uid !== uid)
                return;
            const pkg = reply.item.package;
            dialog.flashing = pkg?.flash_disabled === true;
            dialog.exposes = pkg?.exposes ?? [];
            dialog.sounds = reply.item.kind === "video" || (pkg?.facets ?? []).some(f => f === "audio" || f === "sound");
            dialog.spread = reply.about?.spread === true;
            if (dialog.spread && dialog.fits.includes("span"))
                fit.currentIndex = fit.indexOfValue("span");
            dialog.known = true;
        });
        for (const item of [uid, ""])
            App.query("ListPresets", [item], "Profiles1", reply => dialog && (dialog.presets = dialog.presets.concat(reply.presets)));
        open();
    }

    function choices(first, values, text) {
        return [
            {
                "value": "",
                "text": first
            }
        ].concat(values.map(value => ({
                    "value": value.uid ?? value,
                    "text": text(value)
                })));
    }

    title: App.tr("Apply to…")
    standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel
    onOpened: standardButton(Kirigami.Dialog.Ok).enabled = Qt.binding(() => dialog.known && !dialog.flashing)
    onAccepted: {
        const args = [uid, output.currentValue, fit.currentValue, preset.currentValue];
        const done = () => dialog && dialog.hear();
        App.act("Apply", args, "Library1", done);
    }

    function hear() {
        if (sound.currentValue === "")
            return;
        const volume = sound.currentValue === "false" ? String(Math.round(level.value * 100)) : "";
        App.act("SetSound", [[uid], sound.currentValue, volume], "Library1");
    }

    Kirigami.FormLayout {
        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: dialog.flashing
            type: Kirigami.MessageType.Warning
            text: App.tr("This wallpaper flashes or shows fast patterns that can trigger seizures in people with photosensitive epilepsy. Wallpapers that flash are disabled: allow them in the settings to apply it.")
        }

        PrivacyNotice {
            Layout.fillWidth: true
            exposes: dialog.exposes
        }

        QQC2.ComboBox {
            id: output

            Kirigami.FormData.label: App.tr("Screen:")
            model: dialog.choices(App.tr("All screens"), App.outputs, name => name)
            textRole: "text"
            valueRole: "value"
        }

        QQC2.ComboBox {
            id: fit

            Kirigami.FormData.label: App.tr("Placement:")
            model: dialog.choices(App.tr("Keep"), dialog.fits, value => Labels.value(value))
            textRole: "text"
            valueRole: "value"
        }

        Kirigami.InlineMessage {
            Layout.fillWidth: true
            visible: dialog.spread
            type: Kirigami.MessageType.Information
            text: App.tr("This wallpaper is made to spread over every screen, so it is placed across them. Choose another placement to show it whole on each screen.")
        }

        EditLayout {
            Layout.fillWidth: true
            visible: fit.currentValue === "span"
        }

        QQC2.ComboBox {
            id: sound

            Kirigami.FormData.label: App.tr("Sound:")
            visible: dialog.sounds
            model: [
                {
                    "value": "",
                    "text": App.tr("Keep")
                },
                {
                    "value": "false",
                    "text": App.tr("Play")
                },
                {
                    "value": "true",
                    "text": App.tr("Silent")
                }
            ]
            textRole: "text"
            valueRole: "value"
        }

        QQC2.Slider {
            id: level

            Kirigami.FormData.label: App.tr("Volume:")
            visible: dialog.sounds && sound.currentValue === "false"
            from: 0
            to: 1
            value: 1
        }

        QQC2.ComboBox {
            id: preset

            Kirigami.FormData.label: App.tr("Preset:")
            model: dialog.choices(App.tr("None"), dialog.presets, value => value.name)
            textRole: "text"
            valueRole: "value"

            delegate: QQC2.ItemDelegate {
                id: named

                required property var modelData
                required property int index

                width: ListView.view?.width ?? implicitWidth
                text: modelData.text
                highlighted: preset.highlightedIndex === index

                contentItem: QQC2.Label {
                    text: named.text
                    color: named.highlighted ? Kirigami.Theme.highlightedTextColor : Kirigami.Theme.textColor
                    textFormat: Text.PlainText
                    elide: Text.ElideRight
                }
            }
        }
    }
}
