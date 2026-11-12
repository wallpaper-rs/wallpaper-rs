import QtQuick

Image {
    id: picture

    required property var unit
    required property real ratio

    width: 64
    height: 64
    asynchronous: true
    smooth: true
    fillMode: Image.Stretch
    transform: Matrix4x4 {
        matrix: Qt.matrix4x4(picture.unit[0] * picture.ratio / 64, picture.unit[2] * picture.ratio / 64, 0, picture.unit[4] * picture.ratio, picture.unit[1] * picture.ratio / 64, picture.unit[3] * picture.ratio / 64, 0, picture.unit[5] * picture.ratio, 0, 0, 1, 0, 0, 0, 0, 1)
    }
}
