#include "icon.h"
#include <QtWidgets/QApplication>

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    icon w;
    w.show();
    return a.exec();
}
