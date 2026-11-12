package io.github.wallpaper_rs

import android.Manifest.permission.POST_NOTIFICATIONS
import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.os.Build
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.RequestPermission
import androidx.compose.material3.AlertDialog
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.ui.res.stringResource
import androidx.core.content.ContextCompat

enum class Ask { ASK, SHOW, SILENT }

object Notice {
    const val ID = 4207
    private const val CHANNEL = "convert"
    private const val STOP = "io.github.wallpaper_rs.CONVERT_STOP"
    private const val KEPT = "notice"
    private const val ASKED = "asked"
    private var receiver: BroadcastReceiver? = null
    private var last = ""

    fun plan(enabled: Boolean, asked: Boolean, sdk: Int): Ask = when {
        enabled -> Ask.SHOW
        sdk < Build.VERSION_CODES.TIRAMISU || asked -> Ask.SILENT
        else -> Ask.ASK
    }

    private fun manager(context: Context) = context.getSystemService(NotificationManager::class.java)

    private fun asked(context: Context) = context.getSharedPreferences(KEPT, Context.MODE_PRIVATE).getBoolean(ASKED, false)

    fun mustAsk(context: Context) = plan(manager(context).areNotificationsEnabled(), asked(context), Build.VERSION.SDK_INT) == Ask.ASK

    fun asking(context: Context) = context.getSharedPreferences(KEPT, Context.MODE_PRIVATE).edit().putBoolean(ASKED, true).apply()

    fun update(context: Context, uid: String?, name: String, percent: Int, left: Int, stop: () -> Unit) {
        val notices = manager(context)
        if (uid == null || plan(notices.areNotificationsEnabled(), true, Build.VERSION.SDK_INT) != Ask.SHOW) return clear(context)
        val key = "$uid $percent $left"
        if (key == last) return
        last = key
        notices.createNotificationChannel(NotificationChannel(CHANNEL, context.getString(R.string.convert_channel), NotificationManager.IMPORTANCE_LOW))
        listen(context, stop)
        val intent = PendingIntent.getBroadcast(context, 0, Intent(STOP).setPackage(context.packageName), PendingIntent.FLAG_IMMUTABLE)
        val action = Notification.Action.Builder(null, context.getString(R.string.convert_stop), intent).build()
        notices.notify(
            ID,
            Notification.Builder(context, CHANNEL)
                .setSmallIcon(android.R.drawable.stat_sys_download)
                .setContentTitle(context.getString(R.string.converting, name, percent))
                .setContentText(context.resources.getQuantityString(R.plurals.convert_left, left, left).takeIf { left > 0 })
                .setProgress(100, percent, false)
                .setOngoing(true)
                .setOnlyAlertOnce(true)
                .setCategory(Notification.CATEGORY_PROGRESS)
                .addAction(action)
                .build(),
        )
    }

    fun clear(context: Context) {
        last = ""
        manager(context).cancel(ID)
        receiver?.let { runCatching { context.unregisterReceiver(it) } }
        receiver = null
    }

    private fun listen(context: Context, stop: () -> Unit) {
        if (receiver != null) return
        receiver = object : BroadcastReceiver() {
            override fun onReceive(context: Context, intent: Intent) = stop()
        }
        ContextCompat.registerReceiver(context, receiver, IntentFilter(STOP), ContextCompat.RECEIVER_NOT_EXPORTED)
    }
}

@Composable
fun NoticeAsk(main: Main) {
    val request = rememberLauncherForActivityResult(RequestPermission()) {}
    if (!main.noticing.value) return
    AlertDialog(
        onDismissRequest = { main.noticing.value = false },
        confirmButton = { TextButton({ main.noticing.value = false; request.launch(POST_NOTIFICATIONS) }) { Text(stringResource(R.string.convert_notice_allow)) } },
        dismissButton = { TextButton({ main.noticing.value = false }) { Text(stringResource(R.string.convert_notice_no)) } },
        title = { Text(stringResource(R.string.convert_notice_title)) },
        text = { Text(stringResource(R.string.convert_notice_why)) },
    )
}
