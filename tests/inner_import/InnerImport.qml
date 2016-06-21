import QtQuick 2.2

Item {
  Timer {
    interval: 1000
    running: true
    onTriggered: Qt.quit();
  }
}
