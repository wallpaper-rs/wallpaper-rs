import QtQuick
import QtQuick.Controls as QQC2
import io.github.wallpaper_rs.app

Row {
    id: votes

    required property string packageId
    property var counts: null
    property bool shown
    readonly property string mine: votes.counts ? votes.counts.mine ?? "none" : "none"

    spacing: Look.e2

    signal voted(var found)

    function cast(vote) {
        const wanted = votes.mine === vote ? "none" : vote;
        App.act("Vote", [votes.packageId, wanted], "Sync1", found => {
            if (!votes)
                return;
            votes.voted({
                "up": found.up,
                "down": found.down,
                "mine": found.mine ?? "none"
            });
        });
    }

    QQC2.Button {
        icon.name: "thumbs-up"
        text: votes.shown && votes.counts ? App.tr("In favour · %1", votes.counts.up) : App.tr("Vote in favour")
        checkable: true
        checked: votes.mine === "up"
        onClicked: votes.cast("up")
    }

    QQC2.Button {
        icon.name: "thumbs-down"
        text: votes.shown && votes.counts ? App.tr("Against · %1", votes.counts.down) : App.tr("Vote against")
        checkable: true
        checked: votes.mine === "down"
        onClicked: votes.cast("down")
    }
}
