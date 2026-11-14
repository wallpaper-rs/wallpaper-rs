import QtQuick
import io.github.wallpaper_rs.app

Tapa {
    required property var model
    required property int index
    required property var from
    readonly property string caption: String(from.role(model, from.captionRole) ?? "")
    readonly property bool wants: hovered || lit && from.activeFocus

    picture: from.role(model, from.pictureRole) ?? ""
    pictures: from.role(model, from.picturesRole) ?? []
    moving: wants && from.movingOf ? from.movingOf(index) : ""
    name: from.role(model, from.nameRole) ?? ""
    note: from.role(model, from.noteRole) ?? ""
    noteIcon: from.role(model, from.noteIconRole) ?? ""
    noteTone: from.role(model, from.noteToneRole) ?? ""
    inUse: from.role(model, from.inUseRole) === true
    usedOn: String(from.role(model, from.usedOnRole) ?? "")
    off: from.role(model, from.offRole) === true
    lit: index === from.view.currentIndex
    choosing: from.choosing
    chosen: from.chosen.indexOf(index) >= 0
    onTapped: modifiers => from.tap(index, modifiers)
    onDoubleTapped: from.activated(index)
    onWantsChanged: wants && from.wanted(index)
}
