from PyQt5.QtWidgets import (
    QGridLayout,
    QLabel,
    QWidget,
    QFileDialog,
    QMainWindow,
    QPushButton,
    QInputDialog,
)
from PyQt5.QtGui import QPixmap, QImage, QColor
from PyQt5.QtCore import Qt


class QGrid(QGridLayout):
    def __init__(self, window=None):
        if window is None:
            super().__init__(window)
        super().__init__()
        self.setSpacing(0)
        self.setContentsMargins(0, 0, 0, 0)

    def addWidget(self, widget, row, column, rowSpan=1, columnSpan=1):
        super().addWidget(widget, row, column, rowSpan, columnSpan)
        self.setAlignment(widget, Qt.AlignmentFlag.AlignCenter)


class QObjects:
    @staticmethod
    def canvas(width: int, height: int) -> QLabel:
        img = QLabel()
        image = QImage(width, height, QImage.Format.Format_RGB32)
        QObjects._fill_rgb_gradient(image, width, height)
        put_image_on_canvas(img, image)
        return img

    @staticmethod
    def _fill_rgb_gradient(image: QImage, width: int, height: int) -> None:
        color = QColor()
        x_ratio = 360 / width  # 360º of the color spectrum (HSL representation)
        bright_ratio = 255 / height
        for x in range(width):
            for y in range(height):
                color.setHsl(int(x * x_ratio), 255, int((height - y) * bright_ratio))
                image.setPixel(x, y, color.rgb())

    @staticmethod
    def label(text: str) -> QLabel:
        l = QLabel()
        l.setText(text)
        return l

    @staticmethod
    def button(
        name="Button",
        func=None,
        shortcut=None,
        tooltip=None,
    ) -> QPushButton:

        btn = QPushButton(name)
        if func:
            btn.clicked.connect(func)
        if shortcut:
            btn.setShortcut(shortcut)
        if tooltip:
            btn.setToolTip(tooltip)
        return btn


class QChildWindow(QMainWindow):
    def __init__(self, parent: QMainWindow, title: str, width: int, height: int):
        super().__init__(parent)
        self.setWindowTitle(title)
        self.setFixedSize(width, height)


class QDialogs(QWidget):
    def get_open_path(self):
        filename, _ = QFileDialog.getOpenFileName(
            self, "Open Image", "", "Image Files (*.png *.jpg *.bmp)"
        )
        return filename

    def get_save_path(self):
        filename, _ = QFileDialog.getSaveFileName(
            self, "Save Image", "image.bmp", "Image Files (*.png *.jpg *.bmp)"
        )
        return filename


def display_grid_on_window(window: QMainWindow, grid: QGrid) -> None:
    """
    Set the layout of a window.
    """
    widget = QWidget()
    widget.setLayout(grid)
    window.setCentralWidget(widget)
    window.show()


def get_image_from_canvas(canvas: QLabel) -> QImage:
    return canvas.pixmap().toImage()


def get_pixmap_from_image(image: QImage) -> QPixmap:
    return QPixmap.fromImage(image)


def get_image_from_pixmap(pixmap: QPixmap) -> QImage:
    return pixmap.toImage()


def get_pixmap_from_canvas(canvas: QLabel) -> QPixmap:
    return canvas.pixmap()


def put_pixmap_on_canvas(canvas: QLabel, pixmap: QPixmap) -> None:
    canvas.setPixmap(pixmap)


def put_image_on_canvas(canvas: QLabel, image: QImage) -> None:
    canvas.setPixmap(QPixmap.fromImage(image))


def display_int_input_dialog(
    title: str, low: int, high: int, default: int = None
) -> int:
    dialog = QInputDialog()
    dialog.setWindowTitle(title)
    dialog.setLabelText("Enter a number:")
    dialog.setInputMode(QInputDialog.InputMode.IntInput)
    dialog.setIntRange(low, high)

    if default:
        dialog.setIntValue(default)
    else:
        dialog.setIntValue(low)

    dialog.setCancelButtonText("Cancel")
    dialog.setOkButtonText("Ok")
    dialog.exec_()
    if dialog.result() == QInputDialog.DialogCode.Accepted:
        return dialog.intValue()
    return -1


def display_float_input_dialog(
    title: str, low: float, high: float, default: float = None
) -> float:
    dialog = QInputDialog()
    dialog.setWindowTitle(title)
    dialog.setLabelText("Enter a number:")
    dialog.setInputMode(QInputDialog.InputMode.DoubleInput)
    dialog.setDoubleRange(low, high)

    if default:
        dialog.setDoubleValue(default)
    else:
        dialog.setDoubleValue(low)

    dialog.setCancelButtonText("Cancel")
    dialog.setOkButtonText("Ok")
    dialog.exec_()
    if dialog.result() == QInputDialog.DialogCode.Accepted:
        return dialog.doubleValue()
    return -1