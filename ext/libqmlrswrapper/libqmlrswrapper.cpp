#include "libqmlrswrapper.h"

#include "qrsdynamicobject.h"

#include <QtQuick>
#include <QDebug>

#define rust_fun extern "C"

rust_fun QrsApplicationEngine *qmlrs_create_engine_headless(const char *name, unsigned int len) {
    if (!QCoreApplication::instance()) {
        char *arg = (char *)strndup(name, len);
        char **argp = (char **)malloc(sizeof(char *));
        *argp = arg;

        int *argc = (int *)malloc(sizeof(int));
        *argc = 1;

        new QCoreApplication(*argc, argp);
    }

    return new QrsApplicationEngine();
}

rust_fun QrsApplicationEngine *qmlrs_create_engine(const char *name, unsigned int len) {
    if (!QGuiApplication::instance()) {
        char *arg = (char *)strndup(name, len);
        char **argp = (char **)malloc(sizeof(char *));
        *argp = arg;

        int *argc = (int *)malloc(sizeof(int));
        *argc = 1;

        new QGuiApplication(*argc, argp);
    }

    return new QrsApplicationEngine();
}

rust_fun void qmlrs_destroy_engine(QrsApplicationEngine *engine) {
    delete engine;
}

rust_fun void qmlrs_engine_load_url(QrsApplicationEngine *engine, const char *path, unsigned int len) {
    engine->load(QUrl(QString::fromUtf8(path, len)));
}

rust_fun void qmlrs_engine_load_file(QrsApplicationEngine *engine, const char *path, unsigned int len) {
    engine->load(QUrl::fromLocalFile(QString::fromUtf8(path, len)));
}

rust_fun void qmlrs_engine_load_from_data(QrsApplicationEngine *engine, const char *data, unsigned int len) {
    engine->loadData(QByteArray::fromRawData(data, len), QUrl());
}

rust_fun void qmlrs_engine_set_property(QrsApplicationEngine *engine, const char *name, uint len,
                                        QObject *object) {
    engine->rootContext()->setContextProperty(QString::fromUtf8(name, len), object);
}

rust_fun QVariantList *qmlrs_varlist_create() {
    return new QVariantList();
}

rust_fun void qmlrs_varlist_destroy(QVariantList *list) {
    delete list;
}

rust_fun QVariant *qmlrs_varlist_push(QVariantList *list) {
    list->append(QVariant());
    return (QVariant *)&list->last();
}

rust_fun unsigned int qmlrs_varlist_length(const QVariantList *list) {
    return list->size();
}

rust_fun QVariant *qmlrs_varlist_get(const QVariantList *list, unsigned int i) {
    return (QVariant *)&(*list)[i];
}

rust_fun void qmlrs_app_exec() {
    QGuiApplication::exec();
}

rust_fun void qmlrs_variant_set_int64(QVariant *v, int64_t x) {
    *v = QVariant((qlonglong)x);
}

rust_fun void qmlrs_variant_set_bool(QVariant *v, bool x) {
    *v = QVariant(x);
}

rust_fun void qmlrs_variant_set_invalid(QVariant *v) {
    *v = QVariant();
}

rust_fun void qmlrs_variant_set_string(QVariant *v, unsigned int len, const char *data) {
    *v = QVariant(QString::fromUtf8(data, len));
}

rust_fun QVariant *qmlrs_variant_create() {
    return new QVariant();
}

rust_fun void qmlrs_variant_destroy(QVariant *v) {
    delete v;
}

enum QrsVariantType {
    Invalid = 0, Int64, Bool, String
};

rust_fun QrsVariantType qmlrs_variant_get_type(const QVariant *v) {
    if (!v->isValid())
        return Invalid;

    if (v->type() == (QVariant::Type)QMetaType::QString)
        return String;

    if (v->canConvert(QMetaType::LongLong))
        return Int64;

    if (v->canConvert(QMetaType::Bool))
        return Bool;

    /* Unknown type, not supported on Rust side */
    return Invalid;
}

rust_fun void qmlrs_variant_get_int64(const QVariant *v, int64_t *x) {
    *x = v->toLongLong();
}

rust_fun void qmlrs_variant_get_bool(const QVariant *v, bool *x) {
    *x = v->toBool();
}

rust_fun void qmlrs_variant_get_string_length(const QVariant *v, unsigned int *len) {
    *len = v->toString().toUtf8().size();
}

rust_fun void qmlrs_variant_get_string_data(const QVariant *v, char *data) {
    QByteArray ba = v->toString().toUtf8();
    memcpy(data, ba.data(), ba.size());
}

QrsApplicationEngine::QrsApplicationEngine()
{
}
