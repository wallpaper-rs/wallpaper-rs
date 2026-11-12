package io.github.wallpaper_rs

import android.graphics.Bitmap
import android.graphics.Color
import java.io.ByteArrayOutputStream
import java.net.ServerSocket
import java.util.concurrent.atomic.AtomicInteger
import kotlin.concurrent.thread

class FakeDav(private val pictures: Int = 3) : AutoCloseable {
    private val socket = ServerSocket(0, 8, java.net.InetAddress.getByName("127.0.0.1"))
    val listings = AtomicInteger()
    val downloads = AtomicInteger()
    val url get() = "http://127.0.0.1:${socket.localPort}/dav/"

    init {
        thread(isDaemon = true) {
            while (!socket.isClosed) {
                val client = runCatching { socket.accept() }.getOrNull() ?: break
                thread(isDaemon = true) { client.use { answer(it) } }
            }
        }
    }

    private fun answer(client: java.net.Socket) {
        val input = client.getInputStream()
        val head = StringBuilder()
        while (!head.endsWith("\r\n\r\n")) head.append(input.read().takeIf { it >= 0 }?.toChar() ?: return)
        val length = Regex("(?i)content-length: *(\\d+)").find(head)?.groupValues?.get(1)?.toInt() ?: 0
        repeat(length) { input.read() }
        val (method, path) = head.lines().first().split(' ').let { it[0] to it[1] }
        val (status, body) = when {
            method == "PROPFIND" -> "207 Multi-Status" to listing().toByteArray().also { listings.incrementAndGet() }
            method == "GET" -> "200 OK" to picture(path.substringAfterLast("/n").substringBefore('.').toIntOrNull() ?: 0).also { downloads.incrementAndGet() }
            else -> "404 Not Found" to ByteArray(0)
        }
        client.getOutputStream().apply {
            write("HTTP/1.1 $status\r\nContent-Length: ${body.size}\r\nConnection: close\r\n\r\n".toByteArray())
            write(body)
            flush()
        }
    }

    private fun listing(): String {
        val folder = "<d:response><d:href>/dav/</d:href><d:propstat><d:prop><d:resourcetype><d:collection/></d:resourcetype></d:prop></d:propstat></d:response>"
        val files = (0 until pictures).joinToString("") { "<d:response><d:href>/dav/n$it.png</d:href><d:propstat><d:prop><d:resourcetype/></d:prop></d:propstat></d:response>" }
        return "<?xml version=\"1.0\"?><d:multistatus xmlns:d=\"DAV:\">$folder$files</d:multistatus>"
    }

    private fun picture(n: Int): ByteArray = ByteArrayOutputStream().also { out ->
        Bitmap.createBitmap(16, 8, Bitmap.Config.ARGB_8888).apply { eraseColor(Color.rgb(n * 60, 255 - n * 60, 90)) }.compress(Bitmap.CompressFormat.PNG, 100, out)
    }.toByteArray()

    override fun close() = socket.close()
}
