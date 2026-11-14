import QtQuick
import io.github.wallpaper_rs.app

QtObject {
    id: check

    required property var page
    readonly property string uid: "01J9Z0X9Z6Q7X4F3K5E1V0R1MS"
    property SceneQualityCheck quality: SceneQualityCheck {
        page: check.page
    }

    function must(ok, text) {
        if (!ok)
            throw new Error(text);
    }

    function make(file, props) {
        const made = Qt.createComponent(file).createObject(page, props);
        must(made !== null, "the piece " + file + " does not load");
        return made;
    }

    function exercise() {
        permisos();
        quieta();
        vacio();
        error();
        preajustes();
        mas();
        quality.exercise();
    }

    function permisos() {
        const open = Permisos.open({
            "shown": {
                "DP-1": {
                    "scene": "pond",
                    "key": "sha256:aa",
                    "title": "Pond",
                    "undecided": ["pointer", "network", "audio", "rgb", "clicks"]
                }
            }
        });
        must(open && open.inputs.join() === "pointer,audio,clicks" && open.name === "Pond", "the grouped question does not list what the scene asks, without network and lights: " + JSON.stringify(open));
        must(Permisos.open({
            "shown": {
                "DP-1": {
                    "scene": "pond",
                    "undecided": ["rgb"]
                }
            }
        }) === null, "the grouped question opens for lights alone");
        const question = make("PreguntaPermisos.qml", {
            "scene": open.scene,
            "key": open.key,
            "name": open.name,
            "inputs": open.inputs
        });
        const all = question.allowed.join();
        question.kept = {
            "audio": false
        };
        const found = [all, question.allowed.join(), question.left.join(), question.title, question.quiet, question.principal];
        question.destroy();
        must(found.join("|") === ["pointer,audio,clicks", "pointer,clicks", "audio", App.tr("“%1” asks for permission to use", "Pond"), App.tr("Without this"), App.tr("Allow")].join("|"), "the grouped question does not split what is allowed from what is left out: " + found.join("|"));
    }

    function quieta() {
        const still = make("SceneStill.qml", {
            "host": {
                "uid": check.uid,
                "flashing": false
            },
            "names": ["DP-1"],
            "outputs": {
                "DP-1": {
                    "item": check.uid,
                    "quality_auto": "still"
                }
            }
        });
        const heavy = still.reason;
        still.outputs = {
            "DP-1": {
                "item": check.uid,
                "still_reason": "covered"
            }
        };
        const covered = still.reason;
        still.host = {
            "uid": check.uid,
            "flashing": true
        };
        const found = [heavy, covered, still.reason, still.words("battery")];
        still.destroy();
        must(found.join() === "heavy,,flash,", "a scene still on its poster does not say why: " + found.join());
    }

    function vacio() {
        const settings = make("ItemSettings.qml", {
            "host": {
                "uid": check.uid,
                "item": {},
                "pkg": {
                    "kind": "scene"
                }
            },
            "screens": []
        });
        const before = settings.empty;
        settings.screens = ["DP-9"];
        settings.bare = 1;
        const found = [before, settings.empty];
        settings.destroy();
        must(found.join() === "false,true", "a scene without options of its own does not say so: " + found.join());
    }

    function error() {
        const broken = make("SceneBroken.qml", {
            "page": {
                "host": {
                    "uid": check.uid,
                    "repo": "Home",
                    "item": {
                        "title": "Aurora"
                    },
                    "pkg": {
                        "kind": "scene",
                        "id": "aurora",
                        "version": "1.0.0"
                    },
                    "details": {}
                },
                "entry": {
                    "broken": "the archive is cut short"
                }
            }
        });
        const found = [broken.hurt, broken.scene, broken.from];
        broken.destroy();
        must(found.join() === "true,true,Home", "a damaged scene does not offer to install it again and to remove it: " + found.join());
    }

    function preajustes() {
        const presets = make("ItemPresets.qml", {
            "page": {
                "target": "",
                "host": {
                    "uid": check.uid,
                    "item": {
                        "title": "Hills"
                    },
                    "details": {
                        "presets": [{
                                "uid": "p1",
                                "name": "Full screen",
                                "value": {
                                    "fit": "fill",
                                    "background": "#402030"
                                }
                            }]
                    }
                }
            }
        });
        const found = [presets.all.length, presets.caption(presets.all[0]).includes(Labels.colour("#402030")) && !presets.caption(presets.all[0]).includes("#")];
        presets.destroy();
        const maker = make("PresetMaker.qml", {});
        found.push(maker.fits[0] === "" && maker.fits.includes("fill"));
        maker.destroy();
        must(found.join() === "1,true,true", "My presets does not list a preset with its placement and fill, or the new preset form lacks its placements: " + found.join());
    }

    function mas() {
        const permits = make("ScenePermisos.qml", {
            "uid": check.uid,
            "screen": "DP-1",
            "inputs": {
                "shown": {
                    "DP-1": {
                        "scene": "pond",
                        "granted": ["pointer"],
                        "without": ["clicks", "rgb"]
                    }
                }
            }
        });
        const found = permits.wanted.join();
        permits.destroy();
        must(found === "pointer,clicks", "More does not list the permissions of the scene: " + found);
    }
}
