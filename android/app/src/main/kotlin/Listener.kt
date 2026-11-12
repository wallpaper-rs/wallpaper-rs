package io.github.wallpaper_rs

import android.service.notification.NotificationListenerService

class Listener : NotificationListenerService() {
    override fun onListenerConnected() = Players.recheck()

    override fun onListenerDisconnected() = Players.recheck()
}
