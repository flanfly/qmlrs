#ifndef libqmlrswrapper_H
#define libqmlrswrapper_H

#include <QtQuick>

class QrsInterface;

class QrsApplicationEngine : public QQmlApplicationEngine {
    Q_OBJECT

public:
    QrsApplicationEngine();
};

#endif // libqmlrswrapper_H
