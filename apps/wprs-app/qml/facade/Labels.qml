pragma Singleton
import QtQuick

QtObject {
    readonly property var fields: ({
            "playback.volume": App.tr("Volume"),
            "playback.muted": App.tr("Muted"),
            "playback.max_fps": App.tr("Frames per second"),
            "playback.software_parallax": App.tr("Depth parallax with software rendering"),
            "parallax.fill": App.tr("Fill behind near objects in depth parallax"),
            "parallax.layers": App.tr("Depth layers of the parallax"),
            "playback.allow_flashing": App.tr("Allow wallpapers that flash"),
            "playback.scene_sound": App.tr("Let scenes play their sounds"),
            "playback.scene_capture": App.tr("Let scenes react to the sound that plays"),
            "playback.duck": App.tr("Silence wallpapers while another application plays sound"),
            "transition.kind": App.tr("Transition"),
            "transition.duration_ms": App.tr("Transition length (ms)"),
            "shortcuts.enabled": App.tr("Keys to browse wallpapers"),
            "shortcuts.next": App.tr("Key for the next wallpaper"),
            "shortcuts.previous": App.tr("Key for the previous wallpaper"),
            "transition.less_motion": App.tr("Less motion: plain fades instead of transitions"),
            "pause.when_covered": App.tr("Pause when a window covers the screen"),
            "pause.when_locked": App.tr("Pause while the screen is locked"),
            "library.watch": App.tr("Watch library folders for changes"),
            "library.thumb_cache_mb": App.tr("Thumbnail cache (MB)"),
            "library.hide_mature": App.tr("Hide mature content"),
            "library.show_mature": App.tr("Show mature content without blur"),
            "repos.auto_refresh": App.tr("Refresh repositories every day"),
            "repos.auto_update": App.tr("Update repositories automatically"),
            "repos.vote_counts": App.tr("Show vote counts from your server in the catalog"),
            "layout.bezel_mm": App.tr("Screen frame width for spanning (mm)"),
            "models.allow_download": App.tr("Allow downloading the depth model"),
            "media.remote_covers": App.tr("Fetch covers from the internet"),
            "render.gpu": App.tr("Graphics card"),
            "surfaces.layer_namespace": App.tr("Surface name"),
            "playback.lowest_below_percent": App.tr("Lowest scene quality on battery below this charge (%, 0 = never)"),
            "playback.quality": App.tr("Scene quality"),
            "playback.auto_pace": App.tr("Automatic frame rate"),
            "order": App.tr("Order"),
            "interval": App.tr("Change every"),
            "at": App.tr("Change at (HH:MM, …)"),
            "days": App.tr("Days"),
            "synced": App.tr("Same picture on every screen"),
            "resume": App.tr("Resume where it stopped"),
            "advance_while_paused": App.tr("Advance while paused"),
            "skip_videos_on_battery": App.tr("Skip videos on battery"),
            "transition_ms": App.tr("Transition length (ms)"),
            "outputs": App.tr("When exactly these screens are connected"),
            "on_battery": App.tr("When on battery"),
            "priority": App.tr("Priority"),
            "fit": App.tr("Placement"),
            "background": App.tr("Background color"),
            "refresh": App.tr("Sync every"),
            "cache_mb": App.tr("Cache size (MB)"),
            "market": App.tr("Market"),
            "uhd": App.tr("Ultra HD"),
            "count": App.tr("Pictures"),
            "videos": App.tr("Include videos"),
            "width": App.tr("Width"),
            "query": App.tr("Search"),
            "categories": App.tr("Categories"),
            "purity": App.tr("Purity"),
            "sorting": App.tr("Sorting"),
            "atleast": App.tr("Minimum resolution"),
            "url": App.tr("Address"),
            "user": App.tr("User name"),
            "server": App.tr("Server"),
            "folder": App.tr("Folder"),
            "album": App.tr("Album"),
            "api_key": App.tr("API key"),
            "password": App.tr("Password")
        })
    function feature(name) {
        const parts = String(name).replace(/^effect\./, "").split(".");
        const effect = SceneWords.effect(parts[0]);
        const used = parts.length > 1 ? App.tr("sets %1 on the %2 effect", SceneWords.label(parts[1]), effect) : App.tr("uses the %1 effect", effect);
        return LabelFeatureWords.features[name] ?? used;
    }

    function field(key) {
        return fields[key] ?? WallhavenWords.fields[key] ?? key;
    }

    function tag(name) {
        return LabelTags.words[String(name).toLowerCase()] ?? String(name);
    }

    function value(text) {
        return LabelWords.values[text] ?? WallhavenWords.values[text] ?? String(text ?? "");
    }
}
