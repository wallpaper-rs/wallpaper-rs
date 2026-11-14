import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: shelf

    required property var repo
    readonly property var publish: repo.where.publish
    property string stage: ""
    property string file: ""
    property string key: ""
    property bool made: false

    function begin(path) {
        file = path;
        made = repo.making;
        if (made) {
            stage = "key";
            keys.create(keys.shelves(), repo.slug + "-repo", publish.sign.word, false);
            return;
        }
        key = repo.key;
        add();
    }

    function add() {
        stage = "add";
        stall.add(repo.target, JSON.stringify([file]), repo.where.hidden);
    }

    function feature() {
        stage = "feature";
        stall.feature(repo.target, publish.manifest.id ?? "", repo.where.note, true);
    }

    function index() {
        stage = "index";
        stall.publish(repo.target, key, publish.sign.word, repo.where.days);
    }

    function root() {
        stage = "root";
        stall.sign_root(repo.target, key, publish.sign.word);
    }

    function again() {
        key = repo.key;
        stage = "again";
        stall.publish(repo.target, key, publish.sign.word, repo.where.days);
    }

    function failed(error) {
        const was = stage;
        stage = "";
        publish.work.fail({
            "kind": was === "root" ? "raiz" : "otro",
            "title": App.tr("Could not publish to “%1”", repo.name),
            "text": App.tr("Nothing was lost. Try again, or save it as a file."),
            "details": error
        });
    }

    function created(reply) {
        key = reply.secret;
        stage = "create";
        stall.create(repo.target, JSON.stringify({
            "name": repo.name,
            "root_keys": [reply["public"]],
            "online_keys": [reply["public"]],
            "threshold": 1
        }));
    }

    function take(answer) {
        const reply = JSON.parse(answer);
        if (reply.error !== undefined)
            return failed(reply.error);
        if (stage === "create")
            return root();
        if (stage === "root" && made)
            return add();
        if (stage === "root") {
            stage = "";
            publish.fault = null;
            return repo.look();
        }
        if (stage === "add" && repo.where.featured)
            return feature();
        if (stage === "add" || stage === "feature")
            return index();
        const again = stage === "again";
        stage = "";
        repo.remember(key, reply.published ? reply.published.expires : "");
        if (again) {
            publish.fault = null;
            return repo.look();
        }
        publish.work.finish({
            "kind": "repo",
            "name": repo.name,
            "folder": repo.target,
            "made": made,
            "address": repo.address(),
            "expires": reply.published ? reply.published.expires : ""
        });
    }

    Signing {
        id: keys

        onAnswered: (operation, answer) => {
            const reply = JSON.parse(answer);
            if (reply.error !== undefined)
                shelf.failed(reply.error);
            else
                shelf.created(reply);
        }
    }

    Stall {
        id: stall

        onAnswered: answer => shelf.take(answer)
    }
}
