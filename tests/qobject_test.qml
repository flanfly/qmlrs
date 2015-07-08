import QtQuick 2.2

Item {
  Timer {
    interval: 1000
    running: true
	 onTriggered: {
		 console.log(person_one);
		 Qt.quit();
	 }
  }
}
