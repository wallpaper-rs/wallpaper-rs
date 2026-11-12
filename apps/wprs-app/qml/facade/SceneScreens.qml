pragma Singleton
import QtQuick

QtObject {
    readonly property var names: ({
            "HD": App.tr("HD"),
            "Full HD": App.tr("Full HD"),
            "QHD": App.tr("QHD"),
            "4K UHD": App.tr("4K UHD"),
            "5K": App.tr("5K"),
            "Ultrawide 21:9": App.tr("Ultrawide 21:9"),
            "Super ultrawide 32:9": App.tr("Super ultrawide 32:9"),
            "16:10": App.tr("16:10"),
            "4:3": App.tr("4:3"),
            "Phone 9:19.5": App.tr("Phone 9:19.5")
        })

    function label(name) {
        return names[name] ?? name;
    }
}
