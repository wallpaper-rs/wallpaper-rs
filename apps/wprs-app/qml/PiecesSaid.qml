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
            "text": App.tr("12 wallpapers were added."),
            "tone": "brote",
            "action": App.tr("Show")
        });
        notices.show({
            "text": App.tr("“Cintas” has flashes: it stays paused until you allow it in Settings."),
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
                text: App.tr("Paused: a window covers the screen")
            }

            Estado {
                tone: "coral"
                text: App.tr("Could not set it: the file is missing")
            }

            Estado {
                tone: "brote"
                text: App.tr("Set a moment ago")
            }

            Nota {
                icon: "clock"
                text: App.tr("Shows the time")
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
                        text: App.tr("Not found")
                    }

                    Estado {
                        lit: true
                        tone: "ambar"
                        icon: "pause-fill"
                        text: App.tr("Paused")
                    }

                    Nota {
                        lit: true
                        tone: "brote"
                        text: App.tr("Set a moment ago")
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
                text: App.tr("“Estanque” was removed from the library.")
                action: App.tr("Undo")
            }
        }

        ColumnLayout {
            Layout.alignment: Qt.AlignTop
            Layout.preferredWidth: Look.avisoAncho
            spacing: Look.e6

            Vacio {
                Layout.fillWidth: true
                title: App.tr("You have no lists yet")
                text: App.tr("A list rotates its wallpapers on a screen. Choose wallpapers in your library and add them to a new list.")
                action: App.tr("New list")
                actionIcon: "plus"
            }

            Vacio {
                Layout.fillWidth: true
                small: true
                title: App.tr("Nothing matches “luna roja”")
                text: App.tr("Try another word or remove the filters.")
            }
        }

        ColumnLayout {
            Layout.alignment: Qt.AlignTop
            spacing: Look.e6

            Fallo {
                title: App.tr("Could not read the library")
                text: App.tr("What you already have is still there. Try again; if it happens again, the details say why.")
                action: App.tr("Try again")
                details: "library: permission denied (os error 13)"
                detailsOpen: true
            }

            Fallo {
                big: true
                title: App.tr("Could not reach the service")
                text: App.tr("Check the connection and try again.")
                action: App.tr("Try again")
                details: "connect: connection refused"
            }
        }
    }
}
