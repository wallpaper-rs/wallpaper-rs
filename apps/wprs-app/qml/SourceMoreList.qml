import QtQuick
import io.github.wallpaper_rs.app

Filas {
    id: list

    required property var more
    readonly property string level: more.level
    readonly property var rows: level === "collections" ? more.found.map(c => collection(c)) : SourceMoreWords.flagged.includes(level) ? (more.field(level).values || []).map(v => flag(v)) : (SourceMoreWords.keys[level] || []).map(key => setting(key))

    function enter() {
        const first = shelf.itemAt(0);
        if (first)
            first.forceActiveFocus(Qt.TabFocusReason);
    }

    function flag(value) {
        return {
            "kind": "llave",
            "text": SourceMoreWords.choice(level, value),
            "checked": more.picked(level).includes(value),
            "act": () => more.toggle(level, value)
        };
    }

    function collection(c) {
        return {
            "kind": "llave",
            "text": c.label + (c["public"] ? "" : " · " + App.tr("private")),
            "value": App.tr("%1 photos", c.count),
            "checked": more.shown.collection === more.owner + "/" + c.id,
            "act": () => more.collect(c.id)
        };
    }

    function setting(key) {
        const f = more.field(key);
        const value = String(more.shown[key] ?? "");
        if (key === "atleast") {
            const sizes = [""].concat(more.field("resolutions").values || []);
            return {
                "kind": "elige",
                "text": SourceMoreWords.field(key),
                "options": sizes.map(v => v === "" ? App.tr("Any") : SourceMoreWords.choice(key, v)),
                "current": Math.max(0, sizes.indexOf(value)),
                "pick": at => more.set({
                        [key]: sizes[at]
                    })
            };
        }
        if (SourceMoreWords.flagged.includes(key))
            return {
                "kind": "pliega",
                "text": SourceMoreWords.field(key),
                "value": more.words(key),
                "act": () => more.deeper(key)
            };
        if (f.kind === "enum")
            return {
                "kind": "elige",
                "text": SourceMoreWords.field(key),
                "options": f.values.map(v => SourceMoreWords.choice(key, v)),
                "current": Math.max(0, f.values.indexOf(value)),
                "pick": at => more.set({
                        [key]: f.values[at]
                    })
            };
        return {
            "kind": "campo",
            "text": SourceMoreWords.field(key),
            "fieldText": value,
            "edit": text => more.set({
                    [key]: text.trim()
                })
        };
    }

    Seccion {
        text: SourceMoreWords.level(list.level)
    }

    Repeater {
        id: shelf

        model: list.rows

        delegate: Fila {
            required property var modelData

            kind: modelData.kind
            text: modelData.text
            value: modelData.value ?? ""
            checked: modelData.checked ?? false
            options: modelData.options ?? []
            current: modelData.current ?? 0
            fieldText: modelData.fieldText ?? ""
            onToggled: modelData.act()
            onOpened: modelData.act()
            onChosen: index => modelData.pick(index)
            onEdited: text => modelData.edit(text)
        }
    }

    Vacio {
        visible: list.level === "collections" && list.more.found.length === 0
        small: true
        title: App.tr("No collections")
        text: App.tr("This user has no public collections.")
    }
}
