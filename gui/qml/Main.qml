import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import org.kde.kirigami as Kirigami
import b64replace

Kirigami.ApplicationWindow {
    visible: true
    width: 720
    height: 600
    title: qsTr("Base64 Replacer")

    Backend {
        id: backend
    }

    pageStack.initialPage: Kirigami.Page {
        title: qsTr("Base64 Replacer")
        padding: 0
        actions: [
            Kirigami.Action {
                icon.name: "extract-archive"
                text: qsTr("Decode")
                onTriggered: outputArea.text = backend.replaceAll(inputArea.text)
            }
        ]

        RowLayout {
            anchors.fill: parent
            spacing: 0

            CustomTextArea {
                id: inputArea
                Layout.fillHeight: true
                Layout.fillWidth: true
                Layout.preferredWidth: 1
                label: qsTr("Base64-encoded text:")
            }

            Kirigami.Separator {
                Layout.fillHeight: true
            }

            CustomTextArea {
                id: outputArea
                Layout.fillHeight: true
                Layout.fillWidth: true
                Layout.preferredWidth: 1
                label: qsTr("decoded text:")
                readOnly: true
            }
        }
    }

    component CustomTextArea: ColumnLayout {
        property alias label: titleLabel.text
        property alias readOnly: textArea.readOnly
        property alias text: textArea.text
        Layout.margins: Kirigami.Units.mediumSpacing
        Label {
            id: titleLabel
            font.bold: true
        }
        TextArea {
            id: textArea
            Layout.fillHeight: true
            Layout.fillWidth: true
            font.family: "monospace"
        }
    }
}