#pragma once

#include <QtWidgets/QWidget>
#include "ui_icon.h"
#include <QDebug>
#include <QPushButton>
#include <QDir>
#include <QFileDialog>
#include <QFileInfo>
#include <QMessageBox>
#include <QImage>
#include <QIcon>
#include <QTime>

class icon : public QWidget
{
    Q_OBJECT

public:
    icon(QWidget *parent = nullptr);
    ~icon();

    QString inputPath = "";
    QString outputPath = "";
    bool flatten = false;

public slots:
    void slots_inputHandle();
    void slots_outputHandle();
    void slots_resetHandle();
    bool slots_confirmHandle();

private:
    Ui::iconClass ui;

    void initConnect();
};
