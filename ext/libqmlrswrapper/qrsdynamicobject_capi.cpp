#include <iostream>

#include "qrsdynamicobject.h"

extern "C" void qmlrs_register_singleton_type(const char* uri, unsigned int uri_len, int major, int minor,
																				const char* type, unsigned int type_len, QrsSingletonFunction fun) {
    QString module = QString::fromUtf8(uri, uri_len);
    QString typenam = QString::fromUtf8(type, type_len);

		qmlRegisterSingletonType<QObject>(module.toStdString().c_str(),major,minor,typenam.toStdString().c_str(),fun);
}

extern "C" QrsDynamicMetaObject *qmlrs_metaobject_create(const char* name, unsigned int name_len, QrsSlotFunction fun) {
    QString n = QString::fromUtf8(name, name_len);
    return new QrsDynamicMetaObject(n,fun);
}

extern "C" void qmlrs_metaobject_destroy(QrsDynamicMetaObject *mo) {
    delete mo;
}

extern "C" int qmlrs_metaobject_add_slot(QrsDynamicMetaObject *mo, const char *sig, unsigned int sig_len)
{
    return mo->addSlot(QString::fromUtf8(sig, sig_len));
}

extern "C" int qmlrs_metaobject_add_signal(QrsDynamicMetaObject *mo, const char *sig, unsigned int sig_len)
{
    return mo->addSignal(QString::fromUtf8(sig, sig_len));
}

extern "C" int qmlrs_metaobject_add_method(QrsDynamicMetaObject *mo, const char *sig, unsigned int sig_len)
{
    return mo->addMethod(QString::fromUtf8(sig, sig_len));
}

extern "C" void qmlrs_metaobject_add_property(QrsDynamicMetaObject *mo, const char *name, unsigned int name_len,
                                             const char *type, unsigned int type_len,const char *sig, unsigned int sig_len)
{
    mo->addProperty(QString::fromUtf8(name, name_len), QString::fromUtf8(type, type_len),
				QString::fromUtf8(sig, sig_len));
}

extern "C" QObject *qmlrs_metaobject_instantiate(QrsDynamicMetaObject *mo)
{
    return mo->instantiate();
}

extern "C" void qmlrs_object_set_property(QrsDynamicObject *obj, const char *name, uint name_len,
                                            QVariant* val)
{
    QString n = QString::fromUtf8(name, name_len);
    QQmlProperty prop(obj,n);

    prop.write(*val);
}

extern "C" void qmlrs_object_call(QrsDynamicObject *obj, int id, QVariantList const* args, QVariant *ret) {
	*ret = obj->callMethod(id,*args);
}

extern "C" void qmlrs_object_emit(QrsDynamicObject *obj, int id, QVariantList const* args) {
	obj->emitSignal(id,*args);
}

extern "C" void qmlrs_object_delete_later(QrsDynamicObject *obj) {
    obj->deleteLater();
}
