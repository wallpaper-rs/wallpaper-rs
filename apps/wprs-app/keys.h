#pragma once

#include <QtCore/QCoreApplication>
#include <QtCore/QEvent>
#include <QtCore/QMetaObject>
#include <QtCore/QPoint>
#include <QtGui/QGuiApplication>
#include <QtGui/QKeyEvent>
#include <QtGui/QPointingDevice>
#include <QtGui/QWheelEvent>
#include <QtQuick/QQuickItem>
#include <QtQuick/QQuickWindow>

namespace keys {

inline bool is(const QObject* object, const char* name)
{
    for (auto meta = object->metaObject(); meta; meta = meta->superClass()) {
        if (qstrcmp(meta->className(), name) == 0)
            return true;
    }
    return false;
}

inline bool turns(const QObject* object)
{
    for (auto name : { "QQuickComboBox", "QQuickSlider", "QQuickRangeSlider", "QQuickSpinBox", "QQuickDial", "QQuickTumbler" }) {
        if (is(object, name))
            return true;
    }
    return false;
}

inline QQuickItem* under(QQuickItem* item, const QPointF& at)
{
    if (!item->isVisible() || !item->isEnabled())
        return nullptr;
    if (item->clip() && !item->contains(at))
        return nullptr;
    const auto children = item->childItems();
    for (auto it = children.crbegin(); it != children.crend(); ++it) {
        if (auto hit = under(*it, item->mapToItem(*it, at)))
            return hit;
    }
    return item->contains(at) ? item : nullptr;
}

inline void listen(QQuickItem* item, bool on)
{
    for (auto child : item->childItems()) {
        if (is(child, "QQuickMouseArea") && child->property("acceptedButtons").toInt() == Qt::NoButton)
            child->setEnabled(on);
        else
            listen(child, on);
    }
}

inline void guard(QQuickWindow* window, const QPointF& at)
{
    auto item = under(window->contentItem(), at);
    while (item) {
        if (turns(item)) {
            const bool clicked = item->hasActiveFocus() && item->property("focusReason").toInt() == Qt::MouseFocusReason;
            item->setProperty("wheelEnabled", clicked);
            listen(item, clicked);
            return;
        }
        item = item->parentItem();
    }
}

inline bool pressed(QQuickWindow* window, QKeyEvent* event)
{
    if (event->isAutoRepeat() || (event->modifiers() & ~Qt::KeypadModifier))
        return false;
    if (event->key() != Qt::Key_Return && event->key() != Qt::Key_Enter)
        return false;
    auto item = window->activeFocusItem();
    if (!item || !is(item, "QQuickAbstractButton") || item->property("focusPolicy").toInt() == Qt::NoFocus)
        return false;
    for (auto type : { QEvent::KeyPress, QEvent::KeyRelease }) {
        QKeyEvent space(type, Qt::Key_Space, Qt::NoModifier, QStringLiteral(" "));
        QCoreApplication::sendEvent(window, &space);
    }
    return true;
}

class Filter : public QObject {
public:
    bool eventFilter(QObject* object, QEvent* event) override
    {
        auto window = qobject_cast<QQuickWindow*>(object);
        if (!window)
            return false;
        if (event->type() == QEvent::Wheel)
            guard(window, static_cast<QWheelEvent*>(event)->position());
        else if (event->type() == QEvent::KeyPress)
            return pressed(window, static_cast<QKeyEvent*>(event));
        return false;
    }
};

}

inline void installKeys()
{
    static keys::Filter filter;
    if (auto app = QCoreApplication::instance())
        app->installEventFilter(&filter);
}

inline QQuickWindow* keyWindow()
{
    for (auto window : QGuiApplication::topLevelWindows()) {
        if (auto quick = qobject_cast<QQuickWindow*>(window))
            return quick;
    }
    return nullptr;
}

inline void sendKey(int key, int modifiers)
{
    if (auto window = keyWindow()) {
        for (auto type : { QEvent::KeyPress, QEvent::KeyRelease }) {
            QKeyEvent event(type, key, Qt::KeyboardModifiers(modifiers));
            QCoreApplication::sendEvent(window, &event);
        }
    }
}

inline void sendWheel(double x, double y, int delta)
{
    if (auto window = keyWindow()) {
        static QPointingDevice mouse(QStringLiteral("probe"), 1, QInputDevice::DeviceType::Mouse, QPointingDevice::PointerType::Generic, QInputDevice::Capability::Scroll | QInputDevice::Capability::Position, 1, 3);
        const QPointF at(x, y);
        QWheelEvent event(at, window->mapToGlobal(at), QPoint(), QPoint(0, delta), Qt::NoButton, Qt::NoModifier, Qt::NoScrollPhase, false, Qt::MouseEventNotSynthesized, &mouse);
        QCoreApplication::sendEvent(window, &event);
    }
}
