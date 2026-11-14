import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import io.github.wallpaper_rs.app

Item {
    id: frame

    required property var page
    required property int current
    property var sections: []
    readonly property var route: page?.route ?? []
    readonly property alias seat: seat
    readonly property point noticeAt: Qt.point(panel.x + Look.e8, height - Look.e6 - Look.atajosAlto - Look.e4)
    readonly property point hintsAt: Qt.point(panel.x + Look.e8, height - Look.e6)

    signal chosen(int page)
    signal escaped

    Keys.onEscapePressed: frame.escaped()

    Rectangle {
        width: Look.menuAncho
        height: frame.height
        gradient: Gradient {
            orientation: Gradient.Horizontal

            GradientStop {
                position: 0
                color: Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0.78)
            }

            GradientStop {
                position: 1
                color: Qt.rgba(Look.noche.r, Look.noche.g, Look.noche.b, 0.55)
            }
        }
    }

    Rectangle {
        id: panel

        x: Look.menuAncho
        width: frame.width - x
        height: frame.height
        color: Look.noche

        ColumnLayout {
            id: head

            x: Look.e8
            y: Look.margenArriba + Look.e3
            width: panel.width - Look.e8 * 2
            spacing: Look.e5

            RowLayout {
                Layout.fillWidth: true
                spacing: Look.e5

                Text {
                    text: frame.page?.title ?? ""
                    color: Look.escarcha
                    font.family: Look.letraTitulo
                    font.pixelSize: Look.tPagina
                    font.weight: Font.Bold
                    font.letterSpacing: -0.03 * Look.tPagina
                    Accessible.role: Accessible.Heading
                    Accessible.name: text
                }

                Kirigami.ActionToolBar {
                    Layout.fillWidth: true
                    alignment: Qt.AlignRight
                    actions: frame.page?.actions ?? []
                }

                Loader {
                    active: !!(frame.page && frame.page.head)
                    sourceComponent: frame.page && frame.page.head ? frame.page.head : null
                }
            }

            Row {
                visible: frame.sections.length > 1
                spacing: Look.e5

                Repeater {
                    model: frame.sections

                    delegate: SectionWord {
                        required property var modelData

                        text: modelData[1]
                        current: frame.current === modelData[0]
                        onClicked: frame.chosen(modelData[0])
                    }
                }
            }

            RowLayout {
                visible: frame.route.length > 0
                spacing: Look.e2

                Repeater {
                    model: frame.route

                    delegate: RowLayout {
                        id: step

                        required property string modelData
                        required property int index
                        readonly property bool last: index === frame.route.length - 1

                        spacing: Look.e2

                        Text {
                            text: step.modelData
                            color: step.last ? Look.escarcha : Look.bruma
                            font.family: Look.letraTitulo
                            font.pixelSize: Look.tSeccion
                            font.weight: step.last ? Font.Bold : Font.Normal
                        }

                        Kirigami.Icon {
                            visible: !step.last
                            Layout.preferredWidth: Look.e4
                            Layout.preferredHeight: Look.e4
                            source: "caret-right"
                            color: Look.brumaBaja
                        }
                    }
                }

                Hints {
                    Layout.leftMargin: Look.e3
                    model: [{
                            "keys": ["Esc"],
                            "text": App.tr("back")
                        }]
                }
            }
        }

        Item {
            id: seat

            x: Look.e8
            y: head.y + head.height + Look.e5
            width: panel.width - Look.e8 * 2
            height: panel.height - y - Look.e6 - Look.atajosAlto - Look.e5
        }
    }
}
