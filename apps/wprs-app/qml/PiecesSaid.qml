import QtQuick
import QtQuick.Layouts
import io.github.wallpaper_rs.app

FocusScope {
    id: part

    readonly property alias notices: notices

    function enter() {
        notices.queue = [];
        notices.shown = null;
        notices.show({
            "text": qsTr("12 wallpapers were added."),
            "tone": "brote",
            "action": qsTr("Show")
        });
        notices.show({
            "text": qsTr("“Cintas” has flashes: it stays paused until you allow it in Settings."),
            "tone": "ambar"
        });
        forceActiveFocus(Qt.TabFocusReason);
    }

    GridLayout {
        anchors.fill: parent
        columns: 2
        columnSpacing: Look.e7
        rowSpacing: Look.e6

        ColumnLayout {
            Layout.alignment: Qt.AlignTop
            spacing: Look.e4

            Estado {
                tone: "ambar"
                icon: "pause-fill"
                text: qsTr("Paused: a window covers the screen")
            }

            Estado {
                tone: "coral"
                text: qsTr("Could not set it: the file is missing")
            }

            Estado {
                tone: "brote"
                text: qsTr("Set a moment ago")
            }

            Nota {
                icon: "clock"
                text: qsTr("Shows the time")
            }

            Rectangle {
                Layout.preferredWidth: lit.implicitWidth + 2 * Look.e4
                Layout.preferredHeight: Look.altoCampo
                radius: Look.radio
                color: Look.escarcha

                Row {
                    id: lit

                    anchors.centerIn: parent
                    spacing: Look.e4

                    Estado {
                        lit: true
                        tone: "coral"
                        text: qsTr("Not found")
                    }

                    Estado {
                        lit: true
                        tone: "ambar"
                        icon: "pause-fill"
                        text: qsTr("Paused")
                    }

                    Nota {
                        lit: true
                        tone: "brote"
                        text: qsTr("Set a moment ago")
                    }
                }
            }
        }

        ColumnLayout {
            Layout.alignment: Qt.AlignTop
            spacing: Look.e4

            Avisos {
                id: notices
            }

            Aviso {
                tone: "coral"
                text: qsTr("“Estanque” was removed from the library.")
                action: qsTr("Undo")
            }
        }

        ColumnLayout {
            Layout.alignment: Qt.AlignTop
            spacing: Look.e6

            Vacio {
                title: qsTr("You have no lists yet")
                text: qsTr("A list rotates its wallpapers on a screen. Choose wallpapers in your library and add them to a new list.")
                action: qsTr("New list")
                actionIcon: "plus"
            }

            Vacio {
                small: true
                title: qsTr("Nothing matches “luna roja”")
                text: qsTr("Try another word or remove the filters.")
            }
        }

        ColumnLayout {
            Layout.alignment: Qt.AlignTop
            spacing: Look.e6

            Fallo {
                title: qsTr("Could not read the library")
                text: qsTr("What you already have is still there. Try again; if it happens again, the details say why.")
                action: qsTr("Try again")
                details: "library: permission denied (os error 13)"
                detailsOpen: true
            }

            Fallo {
                big: true
                title: qsTr("Could not reach the service")
                text: qsTr("Check the connection and try again.")
                action: qsTr("Try again")
                details: "connect: connection refused"
            }
        }
    }
}
