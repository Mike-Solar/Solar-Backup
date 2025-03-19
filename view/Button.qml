import QtQuick 2.0
Rectangle{
    property string colorHover: "#EEEEEE"
    property alias image: label.source
    property alias text: textArea.text
    property alias textColor: textArea.color
    function hover(){
        root.color=colorHover;
    }
        id: root
        width: 400
        height: 400
        color: "#FFFFFF"

        Column {
            id: col
            spacing: 20
            Image {
                width: 40
                height: 40
                id: label
                source: "qrc:///assets/backup.svg"
            }
            Text {
                width: 40
                height: 20
                id: textArea
                color: "#000000"
                text: ""
            }

        }

}