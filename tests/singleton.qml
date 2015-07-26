import QtQuick 2.2
import Test 1.2

Item {
	Component.onCompleted: {
		Person.testSignal.connect(testCB);
		Person.test()
	}

	function testCB() {
		console.log("testCB called");
	}

  Timer {
    interval: 1000
		running: true

		onTriggered: {
			console.log(Person.name)
			console.log(Person)
			Qt.quit()
		}
	}
}
