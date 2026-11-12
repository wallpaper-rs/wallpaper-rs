package io.github.wallpaper_rs

import android.Manifest.permission.POST_NOTIFICATIONS
import android.app.Notification
import android.app.NotificationManager
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class NoticeConvertTest {
    private val instrumentation = InstrumentationRegistry.getInstrumentation()
    private val context = instrumentation.targetContext
    private val manager = context.getSystemService(NotificationManager::class.java)

    private fun shown() = manager.activeNotifications.filter { it.id == Notice.ID }

    private fun allow(on: Boolean) {
        val automation = instrumentation.uiAutomation
        if (on) automation.grantRuntimePermission(context.packageName, POST_NOTIFICATIONS)
        else automation.revokeRuntimePermission(context.packageName, POST_NOTIFICATIONS)
    }

    @After
    fun cleared() = Notice.clear(context)

    @Test
    fun aConversionShowsAnOngoingSilentNotificationThatGoesAwayWhenItEnds() {
        allow(true)
        Notice.update(context, "a", "Clip", 40, 2) {}
        val posted = shown().single()
        assertTrue(posted.notification.flags and Notification.FLAG_ONGOING_EVENT != 0)
        assertEquals(1, posted.notification.actions.size)
        assertEquals(40, posted.notification.extras.getInt(Notification.EXTRA_PROGRESS))
        assertEquals(NotificationManager.IMPORTANCE_LOW, manager.getNotificationChannel("convert").importance)
        Notice.update(context, null, "", 0, 0) {}
        assertTrue(shown().isEmpty())
    }

    @Test
    fun aDeclinedPermissionLeavesTheConversionWithoutANotification() {
        allow(false)
        Notice.update(context, "a", "Clip", 10, 0) {}
        assertTrue(shown().isEmpty())
    }
}
