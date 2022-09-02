#include "icon.h"
#include <array>
#include <execution>
#include <thread>
icon::icon(QWidget *parent)
    : QWidget(parent)
{
    ui.setupUi(this);
	this->initConnect();
	this->setWindowIcon(QIcon(":/icon.ico"));
}

icon::~icon()
{}

void icon::initConnect()
{
	connect(ui.btnInput, &QPushButton::clicked, this, &icon::slots_inputHandle);
	connect(ui.btnOutput, &QPushButton::clicked, this, &icon::slots_outputHandle);
	connect(ui.btnReset, &QPushButton::clicked, this, &icon::slots_resetHandle);
	connect(ui.btnConfirm, &QPushButton::clicked, this, &icon::slots_confirmHandle);
}

//选择要处理的图片事件
void icon::slots_inputHandle()
{
	QString path = QFileDialog::getOpenFileName(this, "choose image file", "", "(*.jpg *.jpeg *.png)");
	if (path != "") {
		ui.leInput->setText(path);
		inputPath = path;
	}
}

//选择输出路径事件
void icon::slots_outputHandle()
{
	QString path = QFileDialog::getExistingDirectory(this, "save path", "");
	if (path != "") {
		ui.leOutput->setText(path);
		outputPath = path;
	}
}

//重置输入框以及 checkbox 事件
void icon::slots_resetHandle()
{
	ui.leInput->setText("");
	ui.leOutput->setText("");
	ui.cbFlatten->setChecked(false);
	inputPath = "";
	outputPath = "";
	flatten = false;
}

//格式转换
bool icon::slots_confirmHandle()
{
	QTime time;
	time.start();
	QString appPath = QCoreApplication::applicationDirPath();
	if (inputPath == "") {
		QString tempPath = appPath + "/icon.png";
		qDebug() << "tempPath: " << tempPath << endl;
		QFileInfo file(tempPath);
		if (file.exists()) {
			inputPath = tempPath;
		}
		else {
			QMessageBox::warning(this, "Warning", "image file is not exists.");
			return 1;
		}
	}

	flatten = ui.cbFlatten->checkState();

	if (outputPath == "") {
		outputPath = appPath;
	}
	QString tempPath = outputPath + "/build";
	QString win = tempPath;
	QString mac = tempPath;
	QString png = tempPath;
	QDir dir(tempPath);
	if (dir.exists()) {
		dir.removeRecursively();
	}
	dir.mkdir(tempPath);
	if (!flatten) {
		dir.mkdir(tempPath + "/win");
		win = tempPath + "/win";
		dir.mkdir(tempPath + "/mac");
		mac = tempPath + "/mac";
		dir.mkdir(tempPath + "/png");
		png = tempPath + "/png";
	}

	QImage img;
	if (img.load(inputPath)) {
		int width = img.width();
		int height = img.height();
		if (width != height) {
			QMessageBox::warning(this, "Warning", "Please use a picture with uniform width and height.");
			return 1;
		}
		if (width < 512 || height < 512) {
			QMessageBox::warning(this, "Warning", "Picture width and height cannot be less than 512.");
			return 1;
		}
	}
	else {
		QMessageBox::warning(this, "Warning", "Picture format is not supported.");
		return 1;
	}
	std::thread threadPool[2];
	threadPool[0] = std::thread([&] {
		QImage img256 = img.scaled(256, 256);
		img256.save(win + "/icon.ico", "ico", 100);
	});
	threadPool[1] = std::thread([&] {
		QImage img512 = img.scaled(512, 512);
		img512.save(mac + "/icon.icns", "icns", 100);
	});
	int pngs[7] = {16, 32, 48, 64, 128, 256, 512};


	for (int i = 0; i < 2; ++i) {
		threadPool[i].join();
	}

	std::for_each(std::execution::par, std::begin(pngs), std::end(pngs), [png, img](const int size) {
		QImage p = img.scaled(size, size);
		QString name = QString("/%1x%1.png").arg(size);
		p.save(png + name, "png", 100);
	});

	int t = time.elapsed();
	QMessageBox box;
	box.setText(QString("Picture conversion succeeded. time: %1 ms.").arg(t));
	box.exec();
	return 0;
}
