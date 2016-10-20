#include "qrsdynamicobject.h"

#include <iostream>

QrsDynamicMetaObject::QrsDynamicMetaObject(QString const& n, QrsSlotFunction fun)
: _metaObject(), _builder()
{
	_slotFunction = fun;
	_builder.setClassName(n.toUtf8());
	_builder.setSuperClass(&QObject::staticMetaObject);
}

QrsDynamicMetaObject::~QrsDynamicMetaObject() {}

void qrsStaticDynamicMetacall(QObject *qobj, QMetaObject::Call c, int id, void **a) {
    qobj->qt_metacall(c, id, a);
}

int QrsDynamicMetaObject::addSlot(QString const& sig) {
		if (_metaObject)
				qFatal("Cannot add slot after object created");

		return _builder.addSlot(sig.toUtf8()).index();
}

int QrsDynamicMetaObject::addSignal(QString const& sig) {
		if (_metaObject)
				qFatal("Cannot add signal after object created");

		return _builder.addSignal(sig.toUtf8()).index();
}

int QrsDynamicMetaObject::addMethod(QString const& sig, QString const& ret) {
		if (_metaObject)
				qFatal("Cannot add method after object created");

		return _builder.addMethod(sig.toUtf8(),ret.toUtf8()).index();
}

void QrsDynamicMetaObject::addProperty(QString const& name, QString const& type, QString const& sig) {
		if (_metaObject)
				qFatal("Cannot add property after object created");

		QMetaPropertyBuilder prop = _builder.addProperty(name.toUtf8(),type.toUtf8(),1);

		prop.setReadable(true);
		prop.setWritable(true);

		int i = _builder.indexOfMethod(sig.toUtf8());
		if(i >= 0) {
			prop.setNotifySignal(_builder.method(i));
		}
}

QObject* QrsDynamicMetaObject::instantiate(void)
{
    if (!_metaObject)
        _metaObject.reset(_builder.toMetaObject());

    return new QrsDynamicObject(*this);
}

QMetaObject* QrsDynamicMetaObject::metaObject(void) {
	return _metaObject.get();
}

QMetaObject const* QrsDynamicMetaObject::metaObject(void) const {
	return _metaObject.get();
}

QVariant QrsDynamicMetaObject::invoke(QObject* self, int id, QVariantList const& args) const {
	QVariant v;
	_slotFunction(self,id,&args,&v);

	return v;
}

QrsDynamicObject::QrsDynamicObject(QrsDynamicMetaObject const& mo)
: QObject(), _metaObject(mo), _properties(mo.metaObject()->propertyCount(),QVariant())
{
}

const QMetaObject* QrsDynamicObject::metaObject() const
{
    return _metaObject.metaObject();
}

void* QrsDynamicObject::qt_metacast(const char* )
{
    return Q_NULLPTR;
}

int QrsDynamicObject::qt_metacall(QMetaObject::Call c, int id, void** a)
{
	switch(c) {
		case QMetaObject::InvokeMetaMethod:
			if (id >= metaObject()->methodCount()) {
				return QObject::qt_metacall(c,id - metaObject()->methodCount(),a);
			} else {
				QMetaMethod mm = metaObject()->method(id);
				QVariantList argv;
				int arg = 0;

				while(arg < mm.parameterCount()) {
					argv.append(QVariant(mm.parameterType(arg),a[arg+1]));
					++arg;
				}

				QVariant r = _metaObject.invoke(this,id - metaObject()->methodOffset(),argv);
				QMetaType::construct(r.type(),a[0],r.data());

				return -1;
			}
		case QMetaObject::ReadProperty:
			if (id >= _properties.size()) {
				return QObject::qt_metacall(c,id - _properties.size(),a);
			} else {
				QVariant const& val = *std::next(_properties.begin(), id);
				QMetaType::construct(val.type(),a[0],val.data());
				return -1;
			}
		case QMetaObject::WriteProperty:
			if (id >= _properties.size()) {
				return QObject::qt_metacall(c,id - _properties.size(),a);
			} else {
				QVariant::Type ty = metaObject()->property(id).type();
				*std::next(_properties.begin(), id) = QVariant(ty,a[0]);
				return -1;
			}
		case QMetaObject::ResetProperty:
			if (id >= _properties.size()) {
				return QObject::qt_metacall(c,id - _properties.size(),a);
			} else {
				*std::next(_properties.begin(), id) = QVariant();
				return -1;
			}
		default:
			/*
			QMetaObject::QueryPropertyDesignable,
			QMetaObject::QueryPropertyScriptable,
			QMetaObject::QueryPropertyStored,
			QMetaObject::QueryPropertyEditable,
			QMetaObject::QueryPropertyUser,
			QMetaObject::CreateInstance,
			QMetaObject::IndexOfMethod,
			QMetaObject::RegisterPropertyMetaType,
			QMetaObject::RegisterMethodArgumentMetaType
			*/
			return QObject::qt_metacall(c, id, a);
	}
}

void QrsDynamicObject::emitSignal(int id, QVariantList const& args)
{
	if(id + metaObject()->methodOffset() >= metaObject()->methodCount()) {
		qFatal("no method with id %d",id);
	}

	QMetaMethod mm = metaObject()->method(id + metaObject()->methodOffset());

	if(mm.parameterCount() != args.size()) {
		qWarning("method '%s' expects %d parameters, got %d",
				mm.name().constData(),mm.parameterCount(),args.size());
		return;
	}

	if(mm.methodType() != QMetaMethod::Signal) {
		qFatal("'%s' is not a signal",mm.name().constData());
	}

	QVariant* argv[] = new QVariant[args.size() + 1];
	int i = 0;

	// return value (unused)
	argv[0] = nullptr;

	while(i < args.size()) {
		argv[i] = new QVariant(args[i]);
		++i;
	}

	QMetaObject::activate(this, metaObject(), id, reinterpret_cast<void**>(argv.data()));

	for(auto v: argv) {
		if(v) {
			delete v;
		}
	}

	delete argv[];
}

QVariant QrsDynamicObject::callMethod(int id, QVariantList const& args) {
	if(id + metaObject()->methodOffset() >= metaObject()->methodCount()) {
		std::cerr << "fatal" << std::endl;
		qFatal("no method with id %d",id);
	}

	QMetaMethod mm = metaObject()->method(id + metaObject()->methodOffset());

	if(mm.parameterCount() != args.size()) {
		qWarning("method '%s' expects %d parameters, got %d",
				mm.name().constData(),mm.parameterCount(),args.size());
		return QVariant();
	}

	if(mm.methodType() == QMetaMethod::Signal) {
		qFatal("'%s' is a signal",mm.name().constData());
	}

	QVariant returned;

	QGenericArgument a0, a1, a2, a3, a4, a5, a6, a7, a8, a9;
	if (args.size() > 9) a9 = Q_ARG(QVariant, args[9]);
	if (args.size() > 8) a8 = Q_ARG(QVariant, args[8]);
	if (args.size() > 7) a7 = Q_ARG(QVariant, args[7]);
	if (args.size() > 6) a6 = Q_ARG(QVariant, args[6]);
	if (args.size() > 5) a5 = Q_ARG(QVariant, args[5]);
	if (args.size() > 4) a4 = Q_ARG(QVariant, args[4]);
	if (args.size() > 3) a3 = Q_ARG(QVariant, args[3]);
	if (args.size() > 2) a2 = Q_ARG(QVariant, args[2]);
	if (args.size() > 1) a1 = Q_ARG(QVariant, args[1]);
	if (args.size() > 0) a0 = Q_ARG(QVariant, args[0]);

	if(mm.returnType() == QMetaType::Void) {
		mm.invoke(this,Qt::BlockingQueuedConnection,a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
	} else {
		mm.invoke(this,Qt::BlockingQueuedConnection,
			Q_RETURN_ARG(QVariant, returned),
			a0, a1, a2, a3, a4, a5, a6, a7, a8, a9);
	}
	return returned;
}
