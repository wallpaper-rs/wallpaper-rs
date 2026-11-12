#pragma once

#include <QtCore/QCoreApplication>
#include <QtCore/QEvent>

inline void reapDeleted()
{
    QCoreApplication::sendPostedEvents(nullptr, QEvent::DeferredDelete);
}
