import QtQuick
import io.github.wallpaper_rs.app

Item {
    id: shelf

    required property var repo
    readonly property var publish: repo.where.publish
    property string stage: ""
    property string file: ""
    property string key: ""
    property string rootKey: ""
    property string rootPublic: ""
    property string named: ""
    property string added: ""
    property bool made: false

    function begin(path) {
        file = path;
        made = repo.making;
        if (made) {
            stage = "rootkey";
            named = repo.slug.slice(0, 40) + "-" + Date.now().toString(36);
            keys.create(keys.shelves(), named + "-root", publish.sign.word, false);
            return;
        }
        key = repo.key;
        rootKey = repo.entry().root ?? "";
        add();
    }

    function add() {
        stage = "add";
        stall.add(repo.target, JSON.stringify([file]), repo.where.hidden);
    }

    function feature() {
        stage = "feature";
        stall.feature(repo.target, added, repo.where.note, true);
    }

    function index() {
        stage = "index";
        stall.publish(repo.target, key, publish.sign.word, repo.where.days);
    }

    function root() {
        stage = "root";
        stall.signRoot(repo.target, rootKey, publish.sign.word);
    }

    function again() {
        key = repo.key;
        rootKey = repo.entry().root ?? "";
        stage = "again";
        stall.publish(repo.target, key, publish.sign.word, repo.where.days);
    }

    function failed(error, field) {
        const was = stage;
        const taken = was === "add" && field === "version";
        stage = "";
        publish.work.fail({
            "kind": was === "root" ? "raiz" : taken ? "version" : "otro",
            "title": taken ? App.tr("Version %1 already exists in “%2”", publish.version, repo.name) : App.tr("Could not publish to “%1”", repo.name),
            "text": taken ? App.tr("A published version is not replaced. Upload it with the next number.") : App.tr("Nothing was lost. Try again, or save it as a file."),
            "details": error
        });
    }

    function created(reply) {
        if (stage === "rootkey") {
            rootKey = reply.secret;
            rootPublic = reply["public"];
            stage = "onlinekey";
            return keys.create(keys.shelves(), named + "-online", publish.sign.word, false);
        }
        key = reply.secret;
        stage = "create";
        const signer = publish.unsigned ? null : publish.sign.current;
        const authors = {};
        if (signer !== null && signer["public"])
            authors[Publishing.authorId(signer.name)] = [signer["public"]];
        stall.create(repo.target, JSON.stringify({
            "name": repo.name,
            "root_keys": [rootPublic],
            "online_keys": [reply["public"]],
            "authors": authors,
            "threshold": 1
        }));
    }

    function take(answer) {
        const reply = JSON.parse(answer);
        if (reply.error !== undefined)
            return failed(reply.error, reply.field);
        if (stage === "create")
            return root();
        if (stage === "root" && made)
            return add();
        if (stage === "root") {
            stage = "";
            publish.fault = null;
            return repo.look();
        }
        if (stage === "add")
            added = String((reply.added ?? [""])[0]).split(" ")[0];
        if (stage === "add" && repo.where.featured)
            return feature();
        if (stage === "add" || stage === "feature")
            return index();
        const again = stage === "again";
        stage = "";
        repo.remember(key, rootKey, reply.published ? reply.published.expires : "");
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
