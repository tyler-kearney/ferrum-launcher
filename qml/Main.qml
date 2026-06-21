import Ferrum 1.0

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ApplicationWindow {
    visible: true
    width: 1100
    height: 700

    title: "Ferrum Launcher"
    color: "#111111"

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 12

        //
        // HEADER
        //
        Label {
            text: "Applications"
            font.pixelSize: 34
            font.weight: Font.DemiBold
            color: "#f0f0f0"

            Layout.alignment: Qt.AlignLeft
        }

        //
        // CATEGORY BAR
        //
        Flickable {
            Layout.fillWidth: true
            Layout.preferredHeight: 48

            contentWidth: categoryRow.width
            clip: true

            Row {
                id: categoryRow
                spacing: 8

                Repeater {
                    model: ["Developer", "Utilities", "Audio", "Games", "Social", "Productivity", "Graphics"]

                    Button {
                        text: modelData

                        background: Rectangle {
                            radius: 10
                            color: "#2a2a2a"
                        }

                        contentItem: Text {
                            text: parent.text
                            color: "#dddddd"
                            font.pixelSize: 15
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                    }
                }
            }
        }

        //
        // MAIN CONTENT
        //
        RowLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 14

            //
            // APP GRID
            //
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true

                radius: 18
                color: "#1b1b1b"

                border.color: "#2f2f2f"
                border.width: 1
                ScrollView {
                    anchors.fill: parent
                    anchors.margins: 16

                    clip: true

                    GridView {
                        id: appGrid
                        anchors.fill: parent
                        anchors.margins: 16

                        cellWidth: 110
                        cellHeight: 110

                        model: AppModel {}

                        delegate: Item {
                            width: 90
                            height: 90

                            MouseArea {
                                anchors.fill: parent

                                onClicked: {
                                    appGrid.model.launch_app(name);
                                }
                            }

                            Button {
                                anchors.top: parent.top
                                anchors.right: parent.right

                                width: 24
                                height: 24

                                text: pinned ? "*" : "☆"

                                onClicked: {
                                    appGrid.model.toggle_pin(path);
                                }
                            }

                            Column {
                                anchors.centerIn: parent
                                spacing: 8

                                Image {
                                    width: 64
                                    height: 64

                                    source: icon

                                    fillMode: Image.PreserveAspectFit
                                }

                                Label {
                                    text: name
                                    color: "#e0e0e0"
                                    font.pixelSize: 13

                                    horizontalAlignment: Text.AlignHCenter
                                    width: 90
                                    elide: Text.ElideRight
                                }
                            }
                        }
                    }
                }
            }

            //
            // PINNED PANEL
            //
            Rectangle {
                Layout.preferredWidth: 260
                Layout.fillHeight: true

                radius: 18
                color: "#1b1b1b"

                border.color: "#2f2f2f"
                border.width: 1

                ScrollView {
                    anchors.fill: parent

                    ListView {
                        id: pinnedList

                        width: parent.width
                        height: parent.height

                        model: appGrid.model.pinned_count()

                        delegate: Rectangle {
                            width: ListView.view.width
                            height: 48

                            color: "transparent"

                            Row {
                                anchors.fill: parent
                                anchors.margins: 8
                                spacing: 8

                                Image {
                                    width: 32
                                    height: 32

                                    source: appGrid.model.pinned_icon(index)

                                    fillMode: Image.PreserveAspectFit
                                }

                                Label {
                                    text: appGrid.model.pinned_name(index)
                                    color: "#f0f0f0"

                                    verticalAlignment: Text.AlignVCenter
                                }
                            }

                            MouseArea {
                                anchors.fill: parent

                                onClicked: {
                                    appGrid.model.launch_app(appGrid.model.pinned_name(index));
                                }
                            }
                        }

                        Button {
                            text: "Refresh"

                            onClicked: {
                                pinnedList.forceLayout;
                            }
                        }
                    }
                }
            }
        }
    }

    //
    // SEARCH BOX
    //
    Rectangle {
        Layout.fillWidth: true
        Layout.preferredHeight: 56

        radius: 16
        color: "#1b1b1b"

        border.color: "#2f2f2f"
        border.width: 1

        TextField {
            id: searchField

            anchors.fill: parent
            anchors.margins: 8

            placeholderText: "Search applications..."
            color: "#f0f0f0"

            background: null

            onTextChanged: {
                appGrid.model.set_search(text);
            }
        }
    }
}
