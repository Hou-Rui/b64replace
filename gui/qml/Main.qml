pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import org.kde.kirigami as Kirigami
import b64replace

Kirigami.ApplicationWindow {
    id: root
    width: 840
    height: 680
    title: qsTr("B64Replace GUI")

    Backend {
        id: backend
        input: inputPage.text
    }

    pageStack.defaultColumnWidth: root.width / 2
    pageStack.initialPage: [inputPage, outputPage]

    component TextAreaPage: Kirigami.ScrollablePage {
        id: page

        property alias readOnly: codeArea.readOnly
        property alias text: codeArea.text

        horizontalScrollBarPolicy: ScrollBar.AlwaysOn
        padding: 0
        
        RowLayout {
            height: Math.max(page.height, implicitHeight)
            width: Math.max(page.width, implicitWidth)
            spacing: 0
            
            ListView {
                id: lineNumbers
                property TextMetrics textMetrics: TextMetrics {
                    text: "99999"
                    font: codeArea.font
                }

                Layout.fillHeight: true
                Layout.preferredWidth: textMetrics.width
                Layout.topMargin: codeArea.topPadding
                Layout.bottomMargin: codeArea.bottomPadding

                model: codeArea.text.split(/\n/g)
                clip: true

                delegate: Label {
                    required property string modelData
                    required property int index

                    width: lineNumbers.width
                    height: lineText.height
                    padding: 0

                    Text {
                        id: lineNumber
                        text: parent.index + 1
                        anchors.horizontalCenter: parent.horizontalCenter
                        font: codeArea.font
                        color: Kirigami.Theme.disabledTextColor
                    }

                    Text {
                        id: lineText
                        visible: false
                        text: parent.modelData
                        font: codeArea.font

                        width: codeArea.width
                        leftPadding: codeArea.leftPadding
                        rightPadding: codeArea.rightPadding
                        wrapMode: Text.WordWrap
                    }
                }
            }

            TextArea {
                id: codeArea
                Layout.fillHeight: true
                Layout.fillWidth: true
                font.family: "monospace"
                wrapMode: Text.WordWrap
                background: Rectangle {
                    Kirigami.Theme.colorSet: Kirigami.Theme.View
                    color: Kirigami.Theme.backgroundColor
                }
            }
        }
    }

    TextAreaPage {
        id: inputPage
        title: qsTr("Input")
        actions: [
            Kirigami.Action {
                icon.name: "edit-clear-all"
                text: qsTr("Clear")
                onTriggered: inputPage.text = ""
            }
        ]
    }

    TextAreaPage {
        id: outputPage
        title: qsTr("Output")
        readOnly: true
        text: backend.output

        header: Kirigami.InlineMessage {
            type: Kirigami.MessageType.Error
            text: backend.error
            visible: backend.error !== ""
            showCloseButton: true
        }

        actions: [
            Kirigami.Action {
                displayComponent: RowLayout {
                    Label {
                        text: qsTr("Template:")
                    }
                    TextField {
                        Layout.fillWidth: true
                        placeholderText: qsTr("^{}$")
                        font.family: "monospace"
                        onTextChanged: backend.template = text
                    }
                }
            }
        ]
    }
}