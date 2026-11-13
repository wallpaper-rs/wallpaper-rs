package io.github.wallpaper_rs

import androidx.test.platform.app.InstrumentationRegistry
import java.io.ByteArrayOutputStream
import java.io.DataOutputStream
import kotlinx.coroutines.runBlocking
import org.json.JSONArray
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Assert.fail
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

@CleanApp
@RunWith(Baselined::class)
class HttpsTest {
    private val roots = InstrumentationRegistry.getInstrumentation().targetContext.noBackupFilesDir.resolve("roots.der")
    private var kept: ByteArray? = null

    @Before
    fun saved() {
        storage()
        kept = roots.takeIf { it.exists() }?.readBytes()
        roots.writeBytes(ByteArray(0))
    }

    @After
    fun restored() {
        kept?.let(roots::writeBytes) ?: roots.delete()
        runBlocking {
            val left = (account("providers") as JSONObject).getJSONArray("providers")
            (0 until left.length()).forEach { account("provider_remove", "id" to left.getJSONObject(it).getString("id"), "purge" to true) }
        }
    }

    private fun synced(): Any? = runBlocking { account("provider_sync", "id" to "webdav", "force" to true) }

    private fun providers() = runBlocking { account("providers") as JSONObject }.getJSONArray("providers")

    @Test
    fun anUntrustedCertificateIsAnErrorThatSaysSoAndTheUsersOwnAuthorityIsTrusted() {
        FakeDav(secure = true).use { dav ->
            runBlocking {
                account("provider_add", "kind" to "webdav", "options" to JSONObject().put("url", dav.url).put("user", "me"), "secret" to "pw", "on" to false)
                account("provider_on", "id" to "webdav", "on" to true)
            }
            try {
                synced()
                fail("a certificate nobody trusts was accepted")
            } catch (refused: Refused) {
                assertEquals(refused.message, "android.provider_untrusted", refused.id)
            }
            val error = providers().getJSONObject(0).optJSONObject("last_error")
            assertEquals("the page shows why: ${providers()}", "android.provider_untrusted", error?.optString("id"))
            assertEquals(0, dav.downloads.get())
            val der = Loopback.certificate.encoded
            roots.writeBytes(ByteArrayOutputStream().also { out -> DataOutputStream(out).use { it.writeInt(der.size); it.write(der) } }.toByteArray())
            val report = synced() as JSONArray
            assertEquals("what the source brought: $report", 3, report.getJSONObject(0).getInt("brought"))
            assertEquals(3, dav.downloads.get())
            assertTrue(providers().getJSONObject(0).isNull("last_error"))
        }
    }
}
