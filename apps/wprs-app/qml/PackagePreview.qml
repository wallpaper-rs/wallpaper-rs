import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

ColumnLayout {
    id: preview

    required property var page
    property var stream: null
    property bool missing: false
    readonly property var rates: [0, 15, 30, 60]
    readonly property bool web: page.kind === "web"

    function connect() {
        stream?.destroy();
        stream = null;
        if (App.checking || page.editor.node === 0)
            return;
        try {
            stream = Qt.createQmlObject("import QtQuick; import org.kde.pipewire as PipeWire; PipeWire.PipeWireSourceItem { anchors.fill: parent }", screen);
            if (page.editor.serial > 0 && stream.objectSerial !== undefined)
                stream.objectSerial = page.editor.serial;
            else
                stream.nodeId = page.editor.node;
        } catch (error) {
            missing = true;
        }
    }

    function rate(fps) {
        const look = JSON.parse(page.editor.look || "{}");
        if (fps > 0)
            look.fps = fps;
        else
            delete look.fps;
        page.editor.look = JSON.stringify(look);
        SceneLook.preview(page);
    }

    function show(on) {
        if (on) {
            const look = JSON.parse(page.editor.look || "{}");
            look.show = true;
            page.editor.look = JSON.stringify(look);
            page.editor.attach();
            SceneLook.preview(page);
        }
        page.editor.cover(!on);
    }

    spacing: 0
    onVisibleChanged: show(visible)

    Connections {
        target: preview.page.editor

        function onSerialChanged() {
            preview.connect();
        }
    }

    QQC2.ToolBar {
        Layout.fillWidth: true

        RowLayout {
            anchors.fill: parent

            SceneQuality {
                page: preview.page
                again: App.tr("Reload")
            }

            QQC2.Label {
                text: App.tr("Frame rate")
            }

            QQC2.ComboBox {
                model: preview.rates.map(r => r === 0 ? App.tr("Not capped") : App.tr("%1 frames a second", r))
                onActivated: index => preview.rate(preview.rates[index])
            }
        }
    }

    Rectangle {
        id: area

        Layout.fillWidth: true
        Layout.fillHeight: true
        color: Kirigami.Theme.alternateBackgroundColor
        clip: true

        Rectangle {
            id: screen

            readonly property real fitted: Math.min(area.width / 16, area.height / 9)

            anchors.centerIn: parent
            width: fitted * 16
            height: fitted * 9
            color: "black"

            QQC2.Label {
                anchors.centerIn: parent
                width: parent.width * 0.8
                visible: preview.stream === null
                horizontalAlignment: Text.AlignHCenter
                wrapMode: Text.Wrap
                color: "white"
                style: Text.Outline
                text: preview.missing ? App.tr("Install the KPipeWire QML module to see the preview here.") : (preview.web ? App.tr("The page appears here while wprsd runs.") : App.tr("The preview appears here while wprsd runs."))
            }
        }
    }

    QQC2.Label {
        Layout.fillWidth: true
        wrapMode: Text.Wrap
        color: Kirigami.Theme.disabledTextColor
        text: preview.web ? App.tr("Reload builds the page again from its files, so what you changed in another editor shows; what the page writes with console.log and its errors appear in the console on the right.") : App.tr("Reload builds the preview again from its files, so what you changed in another editor shows.")
    }

    Kirigami.Heading {
        Layout.fillWidth: true
        Layout.topMargin: Kirigami.Units.smallSpacing
        level: 4
        text: preview.web ? App.tr("The page and its lights") : App.tr("The preview and its lights")
    }

    QQC2.Switch {
        text: preview.web ? App.tr("Allow the page to send colors to the lights") : App.tr("Allow the package to send colors to the lights")
        checked: preview.page.manifest.permissions?.rgb === true
        onToggled: checked ? preview.page.change("/manifest/permissions/rgb", true) : preview.page.run("remove", {
            "pointer": "/manifest/permissions/rgb"
        })
    }

    LightsStrip {
        Layout.fillWidth: true
        output: preview.page.editor.output
        active: preview.visible
    }
}
