#include <QApplication>
#include <QPushButton>
#include <QtQuick/QQuickView>
#include <QTranslator>
int main(int argc, char *argv[]) {
    QApplication a(argc, argv);
    QQuickView view;
    view.setGeometry(QRect(0, 0, 800, 600));
    view.setSource(QUrl("qrc:///view/mainpage.qml"));
    view.show();
    QTranslator tsor;			//创建翻译器
    tsor.load("zh_hans.qm");	//加载语言包
    a.installTranslator(&tsor);	//安装翻译器
    return QApplication::exec();
}