#include "qrsdynamicobject.h"

extern "C" QrsDynamicMetaObject *qmlrs_metaobject_create() {
    return new QrsDynamicMetaObject();
}

extern "C" void qmlrs_metaobject_destroy(QrsDynamicMetaObject *mo) {
    delete mo;
}

extern "C" void qmlrs_metaobject_add_slot(QrsDynamicMetaObject *mo, const char *name, uint name_len,
                                          uint argc)
{
    mo->addSlot(QString::fromUtf8(name, name_len), argc);
}

extern "C" void qmlrs_metaobject_add_signal(QrsDynamicMetaObject *mo, const char *name, uint name_len,
                                            uint argc)
{
    mo->addSignal(QString::fromUtf8(name, name_len), argc);
}

extern "C" QObject *qmlrs_metaobject_instantiate(QrsDynamicMetaObject *mo/*, QrsSlotFunction fun, 
                                                 void *data*/)
{
    return mo->create(0,0/*fun, data*/);
}

extern "C" void qmlrs_object_set_property(QrsDynamicObject *obj, const char *name, uint name_len,
                                            QVariant* val)
{
	 auto n = QString::fromUtf8(name, name_len).toStdString();
	 obj->setProperty(n.c_str(),*val);
}

extern "C" void qmlrs_object_emit_signal(QrsDynamicObject *obj, uint id) {
    obj->emitSignal(id);
}

extern "C" void qmlrs_object_destroy(QrsDynamicObject *obj) {
    delete obj;
}
