import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

PanelChico {
    id: adder

    readonly property alias address: address

    title: App.tr("Add by link")
    text: App.tr("Paste the address of a repository of yours or someone else's")
    principal: App.tr("Add")
    principalIcon: "link-simple"
    onAccepted: address.text.trim() !== "" && App.act("Add", [address.text.trim()], "Repos1")

    QQC2.TextField {
        id: address

        width: Look.masFicha
        placeholderText: App.tr("Example: https://casa.example/fondos")
        onAccepted: {
            adder.accepted();
            adder.close();
        }
    }
}
