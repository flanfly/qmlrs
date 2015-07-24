#ifndef QRSDYNAMICOBJECT_H
#define QRSDYNAMICOBJECT_H

#include <memory>
#include <QtCore>
#include <QtQml>
#include "private/qmetaobjectbuilder_p.h"

#if Q_MOC_OUTPUT_REVISION != 67
#error "Unsupport Qt version. Qt with Q_MOC_OUTPUT_REVISION == 67 is required."
#endif

extern "C" typedef void *(QrsSlotFunction)(void *data, int slot, QVariantList const* args,QVariant *retval);
extern "C" typedef QObject *(QrsSingletonFunction)(QQmlEngine*,QJSEngine*);

class QrsDynamicMetaObject
{
public:
    QrsDynamicMetaObject(const QString& n, QrsSlotFunction fun);
    virtual ~QrsDynamicMetaObject();

		int addSlot(QString const& sig);
		int addMethod(QString const& sig);
		int addSignal(QString const& sig);
		void addProperty(QString const& name, QString const& type, QString const& sig);

    QObject *instantiate(void);
		QMetaObject *metaObject(void);
		QMetaObject const *metaObject(void) const;

		QVariant invoke(QObject* self, int id, QVariantList const& argv) const;

private:
		QMetaObjectBuilder _builder;
		std::unique_ptr<QMetaObject> _metaObject;
		QrsSlotFunction *_slotFunction;
};

class QrsDynamicObject : public QObject
{
public:
    QrsDynamicObject(QrsDynamicMetaObject const& mo);

    virtual const QMetaObject* metaObject() const;
    virtual void* qt_metacast(const char* );
    virtual int qt_metacall(QMetaObject::Call , int , void** );

    QVariant callMethod(int id,QVariantList const& args);
    void emitSignal(int id,QVariantList const& args);

private:
		QrsDynamicMetaObject const& _metaObject;
		std::list<QVariant> _properties;
};

#endif // QRSDYNAMICOBJECT_H
