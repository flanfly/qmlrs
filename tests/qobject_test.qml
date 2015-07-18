import QtQuick 2.2

Item {
	Connections {
		target: person_one
		onNameChanged: {
			console.log(person_one.name)
		}
	}

  Timer {
    interval: 1000
		running: true

		onTriggered: {
			console.log(person_one.name)
			person_one.name = "Lutz";

			person_one.func(42);

			timer2.triggered.connect(person_one.test_slot);
			timer2.running = true;
		}
	}

	Timer {
		id: timer2
		interval: 2000
		running: false
	}

	Timer {
		interval: 5000
		running: true

		onTriggered: {
			Qt.quit()
		}
	}
}
