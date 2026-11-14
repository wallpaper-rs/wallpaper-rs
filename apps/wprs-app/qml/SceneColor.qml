import QtQuick
import QtQuick.Dialogs
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

RowLayout {
    id: swatch

    property string hex: "#ffffff"
    property bool alpha: true
    readonly property alias field: field

    signal picked(string hex)

    function normalized(typed) {
        let text = String(typed).trim().toLowerCase();
        if (!text.startsWith("#"))
            text = "#" + text;
        if (/^#[0-9a-f]{3}$/.test(text))
            text = "#" + text.slice(1).split("").map(c => c + c).join("");
        if (!/^#([0-9a-f]{6}|[0-9a-f]{8})$/.test(text) || (text.length === 9 && !alpha))
            return "";
        return text.replace(/^(#[0-9a-f]{6})ff$/, "$1");
    }

    function typed() {
        const next = normalized(field.text);
        if (next !== "" && next !== normalized(hex))
            picked(next);
        else
            field.text = hex;
    }

    QQC2.Button {
        icon.name: "color-picker"
        icon.color: swatch.hex.length === 9 ? "#" + swatch.hex.slice(7) + swatch.hex.slice(1, 7) : swatch.hex
        display: QQC2.AbstractButton.IconOnly
        text: App.tr("Pick a color")
        QQC2.ToolTip.visible: hovered
        QQC2.ToolTip.text: text
        onClicked: dialog.open()
    }

    QQC2.TextField {
        id: field

        Layout.fillWidth: true
        Layout.minimumWidth: Look.e8
        Layout.maximumWidth: implicitWidth
        implicitWidth: Kirigami.Units.gridUnit * 6
        maximumLength: 9
        text: swatch.hex
        selectByMouse: true
        Accessible.name: App.tr("Color as hex digits")
        QQC2.ToolTip.visible: hovered && activeFocus
        QQC2.ToolTip.text: App.tr("Type the color as #rrggbb and press Enter")
        onEditingFinished: swatch.typed()
    }

    ColorDialog {
        id: dialog

        selectedColor: swatch.hex.length === 9 ? "#" + swatch.hex.slice(7) + swatch.hex.slice(1, 7) : swatch.hex
        options: swatch.alpha ? ColorDialog.ShowAlphaChannel : 0
        onAccepted: swatch.picked("#" + [selectedColor.r, selectedColor.g, selectedColor.b].concat(swatch.alpha ? [selectedColor.a] : []).map(v => Math.round(v * 255).toString(16).padStart(2, "0")).join("").replace(/^(#[0-9a-f]{6})ff$/, "$1"))
    }
}
