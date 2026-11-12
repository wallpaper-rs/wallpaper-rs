import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Kirigami.FormLayout {
    id: framing

    required property var page
    readonly property var shown: page.manifest.display ?? ({})
    readonly property bool video: page.kind === "video"
    readonly property var fits: ["fill", "fit", "stretch", "center", "tile", "span"]
    readonly property var flips: ["none", "horizontal", "vertical"]
    property var held: null

    function put(values, clear, gesture) {
        page.manifestEdit("display", {
                "values": values,
                "clear": clear ?? []
            }, gesture === true);
    }

    function place(x, y, gesture) {
        put({
                "position": [x, y]
            }, [], gesture);
    }

    QQC2.ComboBox {
        Kirigami.FormData.label: App.tr("Placement:")
        model: [App.tr("Not set"), App.tr("Fill the screen"), App.tr("Fit inside the screen"), App.tr("Stretch"), App.tr("Center"), App.tr("Tile"), App.tr("Span every screen")]
        currentIndex: framing.fits.indexOf(framing.shown.fit) + 1
        onActivated: index => index === 0 ? framing.put({}, ["fit"]) : framing.put({
                    "fit": framing.fits[index - 1]
                })
    }

    RowLayout {
        Kirigami.FormData.label: App.tr("Mirror:")

        Repeater {
            model: framing.flips

            delegate: QQC2.Button {
                required property string modelData
                required property int index

                checkable: true
                autoExclusive: true
                checked: (framing.shown.flip ?? "none") === modelData
                icon.name: ["object-straighten", "object-flip-horizontal", "object-flip-vertical"][index]
                text: [App.tr("None"), App.tr("Horizontally"), App.tr("Vertically")][index]
                onClicked: modelData === "none" ? framing.put({}, ["flip"]) : framing.put({
                        "flip": modelData
                    })
            }
        }
    }

    RowLayout {
        Kirigami.FormData.label: App.tr("Background:")

        SceneColor {
            alpha: false
            hex: framing.shown.background ?? "#000000"
            onPicked: hex => framing.put({
                        "background": hex
                    })
        }

        QQC2.Button {
            text: App.tr("Default")
            icon.name: "edit-undo"
            enabled: framing.shown.background !== undefined
            onClicked: framing.put({}, ["background"])
        }
    }

    QQC2.CheckBox {
        Kirigami.FormData.label: App.tr("Screens:")
        text: App.tr("Spread over every screen")
        checked: framing.shown.span === "all-outputs"
        onToggled: checked ? framing.put({
                "span": "all-outputs"
            }) : framing.put({}, ["span"])
    }

    RowLayout {
        Kirigami.FormData.label: App.tr("Zoom:")
        visible: framing.video

        SceneSlider {
            id: zoom

            Layout.preferredWidth: Kirigami.Units.gridUnit * 12
            editor: framing.page.editor
            from: 1
            to: 8
            stepSize: 0.05
            value: framing.shown.zoom ?? 1
            onMoved: framing.put({
                    "zoom": Math.round(value * 100) / 100
                }, [], true)
        }

        QQC2.Label {
            text: Number(zoom.value).toLocaleString(Qt.locale(), "f", 2) + "×"
        }
    }

    ColumnLayout {
        Kirigami.FormData.label: App.tr("Part shown:")
        Kirigami.FormData.labelAlignment: Qt.AlignTop
        visible: framing.video

        AlignPad {
            px: (framing.held ?? framing.shown.position ?? [0, 0])[0]
            py: (framing.held ?? framing.shown.position ?? [0, 0])[1]
            onMoved: (x, y) => framing.held = [x, y]
            onSettled: (x, y) => {
                framing.held = null;
                framing.place(x, y);
            }
        }

        QQC2.Button {
            text: App.tr("Center")
            icon.name: "zoom-fit-best"
            enabled: framing.shown.position !== undefined
            onClicked: framing.put({}, ["position"])
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: App.tr("What the user sets for the screen always wins over these defaults.")
    }
}
