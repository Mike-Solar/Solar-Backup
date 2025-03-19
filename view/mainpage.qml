import QtQuick 2.0
import QtQuick.Layouts 1.15
Rectangle{
    anchors.fill: parent

    ColumnLayout{
        anchors.fill: parent
        Text{
            anchors.top: parent
            text: "Solar Backup"
            color: "#000000"
        }
        spacing: 10
        RowLayout {
            anchors.bottom: parent
            Button {
                id: "set"
                text: qsTr("设置计划")
                color: "#FFFFFF"
                colorHover: "#eeeeee"
                textColor: "#000000"
                image: "qrc:///assets/plan.svg"
            }
            Button {
                id: "backup"
                color: "#FFFFFF"
                colorHover: "#eeeeee"
                textColor: "#000000"
                text: qsTr("立即备份")
                image: "qrc:///assets/backup.svg"
            }
            Button{
                id: "recovery"
                color: "#FFFFFF"
                colorHover: "#eeeeee"
                textColor: "#000000"
                text: qsTr("还原")
            }
            Button{
                id:"manage"
                color: "#FFFFFF"
                colorHover: "#eeeeee"
                textColor: "#000000"
                text: qsTr("管理备份")
            }
        }
    }
}