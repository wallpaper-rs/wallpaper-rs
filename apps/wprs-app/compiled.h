#pragma once

#include <QtCore/QDirIterator>
#include <QtCore/QUrl>
#include <QtQml/QQmlComponent>
#include <QtQml/QQmlEngine>

inline void compileQml()
{
    QQmlEngine engine;
    engine.addImportPath(QStringLiteral("qrc:/qt/qml"));
    for (auto root : { ":/qt/qml/io/github/wallpaper_rs/app/qml", ":/qt/qml/io/github/wallpaper_rs/style" }) {
        QDirIterator files(QString::fromLatin1(root), { QStringLiteral("*.qml") }, QDir::Files, QDirIterator::Subdirectories);
        while (files.hasNext()) {
            const QUrl url(QStringLiteral("qrc") + files.next());
            QQmlComponent component(&engine, url, QQmlComponent::PreferSynchronous);
            if (component.isError())
                qWarning("%s", qPrintable(component.errorString().trimmed()));
        }
    }
}
