package io.github.wallpaper_rs

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import androidx.work.ExistingPeriodicWorkPolicy
import androidx.work.NetworkType
import androidx.work.PeriodicWorkRequest
import androidx.work.WorkInfo
import androidx.work.WorkManager
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.runBlocking
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class UpdatesTest {
    private val context = InstrumentationRegistry.getInstrumentation().targetContext

    @Test
    fun withoutRepositoriesNothingChecksForUpdates() {
        storage()
        val work = WorkManager.getInstance(context)
        val waiting = PeriodicWorkRequest.Builder(RepoCheck::class.java, 1, TimeUnit.DAYS).build()
        work.enqueueUniquePeriodicWork(Updates.WORK, ExistingPeriodicWorkPolicy.KEEP, waiting).result.get()
        Updates.schedule(context)
        assertTrue(waitFor { work.getWorkInfosForUniqueWork(Updates.WORK).get().all { it.state == WorkInfo.State.CANCELLED } })
        assertEquals(0, Updates.count())
    }

    @Test
    fun theDailyCheckFollowsTheMobileDataSwitch() {
        storage()
        val work = WorkManager.getInstance(context)
        val network = Native.setting(NETWORK, null)
        val needed = { work.getWorkInfosForUniqueWork(Updates.WORK).get().single().constraints.requiredNetworkType }
        try {
            Native.setting(NETWORK, "any")
            Updates.schedule(context, repos = 1)
            assertTrue(waitFor { needed() == NetworkType.CONNECTED })
            Native.setting(NETWORK, "unmetered")
            Updates.schedule(context, repos = 1)
            assertTrue("turning mobile data off reaches the waiting check", waitFor { needed() == NetworkType.UNMETERED })
        } finally {
            Native.setting(NETWORK, network)
            work.cancelUniqueWork(Updates.WORK).result.get()
        }
    }

    @Test
    fun turningTheDailyCheckOffStopsIt() {
        storage()
        val work = WorkManager.getInstance(context)
        val states = { work.getWorkInfosForUniqueWork(Updates.WORK).get().map { it.state } }
        try {
            Updates.schedule(context, repos = 1)
            assertTrue("on by default, as on the computer", waitFor { states() == listOf(WorkInfo.State.ENQUEUED) })
            Native.setting(Updates.SETTING, "false")
            Updates.schedule(context, repos = 1)
            assertTrue(waitFor { states().all { it == WorkInfo.State.CANCELLED } })
        } finally {
            Native.setting(Updates.SETTING, "true")
            work.cancelUniqueWork(Updates.WORK).result.get()
        }
    }

    @Test
    fun aCheckTurnedOffElsewhereCancelsItselfOnItsNextRun() {
        storage()
        val work = WorkManager.getInstance(context)
        val states = { work.getWorkInfosForUniqueWork(Updates.WORK).get().map { it.state } }
        try {
            Updates.schedule(context, repos = 1)
            assertTrue(waitFor { states() == listOf(WorkInfo.State.ENQUEUED) })
            Native.setting(Updates.SETTING, "false")
            Updates.check(context)
            assertTrue("a synced switch stops the daily wakeup", waitFor { states().all { it == WorkInfo.State.CANCELLED } })
        } finally {
            Native.setting(Updates.SETTING, "true")
            work.cancelUniqueWork(Updates.WORK).result.get()
        }
    }

    @Test
    fun automaticUpdatesKeepTheDailyCheckWhileTheCheckAloneIsOff() {
        storage()
        val work = WorkManager.getInstance(context)
        val states = { work.getWorkInfosForUniqueWork(Updates.WORK).get().map { it.state } }
        try {
            Native.setting(Updates.SETTING, "false")
            Native.setting(Updates.AUTO, "true")
            Updates.schedule(context, repos = 1)
            assertTrue("updating by itself needs the check", waitFor { states() == listOf(WorkInfo.State.ENQUEUED) })
            Native.setting(Updates.AUTO, "false")
            Updates.schedule(context, repos = 1)
            assertTrue(waitFor { states().all { it == WorkInfo.State.CANCELLED } })
        } finally {
            Native.setting(Updates.SETTING, "true")
            Native.setting(Updates.AUTO, "false")
            work.cancelUniqueWork(Updates.WORK).result.get()
        }
    }

    @Test
    fun syncingRestoringAndLinkingScheduleTheDailyCheckWhenThereAreRepositories() {
        storage()
        val work = WorkManager.getInstance(context)
        val states = { work.getWorkInfosForUniqueWork(Updates.WORK).get().map { it.state } }
        try {
            Updates.counted = { 1 }
            Sync.network(context)
            assertTrue("a change of network rules schedules it", waitFor { states() == listOf(WorkInfo.State.ENQUEUED) })
            work.cancelUniqueWork(Updates.WORK).result.get()
            assertTrue(waitFor { states().all { it == WorkInfo.State.CANCELLED } })
            runBlocking { Updates.settled(context) }
            assertTrue("the end of a sync, restore or link schedules it", waitFor { states().any { it == WorkInfo.State.ENQUEUED } })
            Updates.counted = { 0 }
            runBlocking { Updates.settled(context) }
            assertTrue("without repositories nothing stays", waitFor { states().all { it == WorkInfo.State.CANCELLED } })
        } finally {
            Updates.counted = { Updates.repos() }
            work.cancelUniqueWork(Updates.WORK).result.get()
        }
    }

    private companion object {
        const val NETWORK = "sync_network"
    }
}
