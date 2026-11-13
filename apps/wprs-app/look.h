#pragma once

#include <QtCore/QStringList>
#include <QtGui/QColor>
#include <QtGui/QFont>
#include <QtGui/QFontDatabase>
#include <QtGui/QFontInfo>
#include <QtGui/QGuiApplication>
#include <QtGui/QIcon>
#include <QtGui/QPalette>
#include <QtQuickControls2/QQuickStyle>

inline void installLook(const QString& style, const QString& theme)
{
    for (auto file : { ":/wprs/fonts/Onest.ttf", ":/wprs/fonts/RedHatDisplay.ttf" })
        QFontDatabase::addApplicationFont(QString::fromLatin1(file));
    QIcon::setThemeSearchPaths({ QStringLiteral(":/icons") });
    QIcon::setFallbackSearchPaths({});
    QIcon::setFallbackThemeName(theme);
    QIcon::setThemeName(theme);
    QQuickStyle::setStyle(style);
}

inline void dressLook(const QString& family, int pixels)
{
    QFont font(family);
    font.setPixelSize(pixels);
    QGuiApplication::setFont(font);
}

inline void paintLook(const QString& joined)
{
    const auto tones = joined.split(QLatin1Char(','));
    if (tones.size() < 7)
        return;
    const QColor noche(tones[0]), alta(tones[1]), linea(tones[2]), escarcha(tones[3]), bruma(tones[4]), baja(tones[5]), luna(tones[6]);
    QPalette palette;
    const auto set = [&palette](QPalette::ColorGroup group, std::initializer_list<std::pair<QPalette::ColorRole, QColor>> roles) {
        for (const auto& [role, color] : roles)
            palette.setColor(group, role, color);
    };
    set(QPalette::All,
        { { QPalette::Window, noche }, { QPalette::WindowText, escarcha }, { QPalette::Base, alta },
            { QPalette::AlternateBase, noche }, { QPalette::Text, escarcha }, { QPalette::Button, alta },
            { QPalette::ButtonText, escarcha }, { QPalette::Highlight, luna }, { QPalette::HighlightedText, noche },
            { QPalette::BrightText, noche }, { QPalette::ToolTipBase, alta }, { QPalette::ToolTipText, escarcha },
            { QPalette::PlaceholderText, bruma }, { QPalette::Link, luna }, { QPalette::LinkVisited, bruma },
            { QPalette::Light, linea }, { QPalette::Midlight, alta }, { QPalette::Mid, linea },
            { QPalette::Dark, bruma }, { QPalette::Shadow, noche } });
#if QT_VERSION >= QT_VERSION_CHECK(6, 6, 0)
    palette.setColor(QPalette::All, QPalette::Accent, luna);
#endif
    set(QPalette::Disabled,
        { { QPalette::WindowText, baja }, { QPalette::Text, baja }, { QPalette::ButtonText, baja },
            { QPalette::Highlight, linea }, { QPalette::HighlightedText, bruma }, { QPalette::Light, alta },
            { QPalette::Dark, linea } });
    QGuiApplication::setPalette(palette);
}

inline QString lookStyle()
{
    return QQuickStyle::name();
}

inline QString lookFamily()
{
    return QFontInfo(QGuiApplication::font()).family();
}

inline bool lookHasFamily(const QString& family)
{
    return QFontDatabase::families().contains(family);
}

inline bool lookHasIcon(const QString& name)
{
    return QIcon::hasThemeIcon(name);
}
