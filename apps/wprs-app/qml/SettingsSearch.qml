import QtQuick
import io.github.wallpaper_rs.app

Buscar {
    id: search

    required property var owner

    placeholder: App.tr("Search a setting")
    text: owner.query
    onTextChanged: owner.query !== text && (owner.query = text)
    onAccepted: owner.found.enter()
    Keys.onDownPressed: owner.found.enter()
    Keys.onEscapePressed: event => {
        if (text === "") {
            event.accepted = false;
            return;
        }
        text = "";
    }
}
