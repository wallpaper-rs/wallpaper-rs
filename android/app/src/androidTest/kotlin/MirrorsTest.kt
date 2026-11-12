package io.github.wallpaper_rs

import android.net.Uri
import android.provider.MediaStore
import androidx.test.platform.app.InstrumentationRegistry
import androidx.work.NetworkType
import androidx.work.WorkInfo
import androidx.work.WorkManager
import java.util.concurrent.TimeUnit
import org.json.JSONArray
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(Baselined::class)
class MirrorsTest {
    private val context = InstrumentationRegistry.getInstrumentation().targetContext
    private val tree = "content://com.android.externalstorage.documents/tree/primary%3APictures"

    private val cloud = "content://org.example.cloud.documents/tree/root%3APhotos"

    private fun plan(vararg kinds: String, at: String = tree) = JSONObject().put("tree", at).put("accept", JSONArray(kinds.toList()))

    @Test
    fun aChosenFolderWakesTheCopyOnlyWhenItsFilesChange() {
        val triggers = Mirrors.triggers(listOf(plan("image")))
        assertEquals(Uri.parse("$tree/document/primary%3APictures/children"), triggers.first())
        assertTrue(MediaStore.Images.Media.EXTERNAL_CONTENT_URI in triggers)
        assertFalse("pictures only do not wake for videos", MediaStore.Video.Media.EXTERNAL_CONTENT_URI in triggers)
        assertEquals(3, Mirrors.triggers(listOf(plan("image", "video"), plan("image"))).size)
        val elsewhere = Mirrors.triggers(listOf(plan("image", "video", at = cloud)))
        assertEquals("a cloud announces its own reads, so its folders never wake the copy", emptyList<Uri>(), elsewhere)
        assertEquals(triggers, Mirrors.triggers(listOf(plan("image"), plan("image", at = cloud))))
        val work = WorkManager.getInstance(context)
        work.enqueueUniqueWork(TEST, androidx.work.ExistingWorkPolicy.REPLACE, Mirrors.request(triggers)).result.get()
        val info = work.getWorkInfosForUniqueWork(TEST).get().single()
        val watched = info.constraints.contentUriTriggers.map { it.uri to it.isTriggeredForDescendants }
        assertEquals(triggers.map { it to true }.toSet(), watched.toSet())
        assertTrue(info.constraints.requiresBatteryNotLow())
        assertEquals(WorkInfo.State.ENQUEUED, info.state)
        work.cancelUniqueWork(TEST).result.get()
    }

    @Test
    fun foldersOfACloudWaitForAnUnmeteredNetworkInTheBackground() {
        val (phone, far) = plan("image") to plan("image", at = cloud)
        assertEquals("a change copies only the folders of the phone", listOf(phone), Mirrors.now(listOf(phone, far), waiting = false))
        assertEquals("clouds are copied by the job that waits for an unmetered network", listOf(far), Mirrors.now(listOf(phone, far), waiting = true))
        val work = WorkManager.getInstance(context)
        work.enqueueUniqueWork(TEST, androidx.work.ExistingWorkPolicy.REPLACE, Mirrors.waiting()).result.get()
        val info = work.getWorkInfosForUniqueWork(TEST).get().single()
        assertEquals(NetworkType.UNMETERED, info.constraints.requiredNetworkType)
        assertTrue(info.constraints.requiresBatteryNotLow())
        work.cancelUniqueWork(TEST).result.get()
    }

    @Test
    fun aBusyPhoneDoesNotRelistTheFoldersAtEveryNewPhoto() {
        val triggers = Mirrors.triggers(listOf(plan("image")))
        val work = WorkManager.getInstance(context)
        work.enqueueUniqueWork(TEST, androidx.work.ExistingWorkPolicy.REPLACE, Mirrors.request(triggers, Mirrors.GAP_S)).result.get()
        val info = work.getWorkInfosForUniqueWork(TEST).get().single()
        assertEquals(TimeUnit.SECONDS.toMillis(Mirrors.GAP_S), info.initialDelayMillis)
        assertEquals(triggers.map { it to true }.toSet(), info.constraints.contentUriTriggers.map { it.uri to it.isTriggeredForDescendants }.toSet())
        work.cancelUniqueWork(TEST).result.get()
    }

    @Test
    fun cloudFoldersAreCopiedOnTheirOwnDailySchedule() {
        val work = WorkManager.getInstance(context)
        work.enqueueUniquePeriodicWork(TEST, androidx.work.ExistingPeriodicWorkPolicy.REPLACE, Mirrors.daily()).result.get()
        val info = work.getWorkInfosForUniqueWork(TEST).get().single()
        assertEquals(TimeUnit.DAYS.toMillis(1), info.periodicityInfo?.repeatIntervalMillis)
        assertEquals(NetworkType.UNMETERED, info.constraints.requiredNetworkType)
        assertTrue(info.constraints.requiresBatteryNotLow())
        assertTrue(info.tags.contains(Remirror::class.java.name))
        work.cancelUniqueWork(TEST).result.get()
    }

    @Test
    fun withoutChosenFoldersNothingWaitsForChanges() {
        storage()
        val work = WorkManager.getInstance(context)
        val triggers = listOf(MediaStore.Images.Media.EXTERNAL_CONTENT_URI)
        work.enqueueUniqueWork(Mirrors.WORK, androidx.work.ExistingWorkPolicy.REPLACE, Mirrors.request(triggers)).result.get()
        work.enqueueUniqueWork(Mirrors.CLOUD, androidx.work.ExistingWorkPolicy.REPLACE, Mirrors.waiting()).result.get()
        work.enqueueUniquePeriodicWork(Mirrors.DAILY, androidx.work.ExistingPeriodicWorkPolicy.REPLACE, Mirrors.daily()).result.get()
        Mirrors.watch(context)
        listOf(Mirrors.WORK, Mirrors.CLOUD, Mirrors.DAILY).forEach { name ->
            assertTrue(waitFor { work.getWorkInfosForUniqueWork(name).get().all { it.state == WorkInfo.State.CANCELLED } })
        }
        assertFalse(context.remirror())
    }

    private companion object {
        const val TEST = "mirror-test"
    }
}
