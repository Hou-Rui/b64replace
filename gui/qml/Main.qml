pragma ComponentBehavior: Bound

import QtQuick
import QtQuick.Controls

import org.kde.kirigami as Kirigami
import b64replace

Kirigami.ApplicationWindow {
    id: root
    width: 720
    height: 600
    title: qsTr("B64Replace GUI")

    Backend {
        id: backend
        input: inputPage.text
    }

    pageStack.defaultColumnWidth: root.width / 2
    pageStack.initialPage: [inputPage, outputPage]

    component TextAreaPage: Kirigami.ScrollablePage {
        id: page

        property alias readOnly: textArea.readOnly
        property alias text: textArea.text

        horizontalScrollBarPolicy: ScrollBar.AlwaysOn
        padding: 0

        TextArea {
            id: textArea
            font.family: "monospace"
            wrapMode: Text.WordWrap
            height: Math.max(page.height, implicitHeight)
            width: Math.max(page.width, implicitWidth)
            background: Rectangle {
                Kirigami.Theme.colorSet: Kirigami.Theme.View
                color: Kirigami.Theme.backgroundColor
            }
        }
    }

    TextAreaPage {
        id: inputPage
        title: qsTr("Input")
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
                displayComponent: TextField {
                    placeholderText: qsTr("Custom capture template...")
                    font.family: "monospace"
                    onTextChanged: backend.template = text
                }
            }
        ]
    }
}