package io.github.wallpaper_rs

import android.util.Base64
import androidx.test.platform.app.InstrumentationRegistry
import java.net.InetAddress
import java.net.ServerSocket
import java.security.KeyFactory
import java.security.KeyStore
import java.security.cert.CertificateFactory
import java.security.cert.X509Certificate
import java.security.spec.PKCS8EncodedKeySpec
import javax.net.ssl.KeyManagerFactory
import javax.net.ssl.SSLContext

object Loopback {
    private val pem by lazy { InstrumentationRegistry.getInstrumentation().context.assets.open("loopback-tls.pem").use { it.readBytes().decodeToString() } }

    val certificate: X509Certificate by lazy {
        CertificateFactory.getInstance("X.509").generateCertificate(pem.substringBefore("-----BEGIN PRIVATE KEY-----").byteInputStream()) as X509Certificate
    }

    fun socket(secure: Boolean): ServerSocket {
        val loopback = InetAddress.getByName("127.0.0.1")
        if (!secure) return ServerSocket(0, 8, loopback)
        val body = pem.substringAfter("-----BEGIN PRIVATE KEY-----").substringBefore("-----END PRIVATE KEY-----")
        val key = KeyFactory.getInstance("EC").generatePrivate(PKCS8EncodedKeySpec(Base64.decode(body, Base64.DEFAULT)))
        val store = KeyStore.getInstance(KeyStore.getDefaultType()).apply {
            load(null)
            setKeyEntry("loopback", key, CharArray(0), arrayOf(certificate))
        }
        val keys = KeyManagerFactory.getInstance(KeyManagerFactory.getDefaultAlgorithm()).apply { init(store, CharArray(0)) }
        val tls = SSLContext.getInstance("TLS").apply { init(keys.keyManagers, null, null) }
        return tls.serverSocketFactory.createServerSocket(0, 8, loopback)
    }
}
