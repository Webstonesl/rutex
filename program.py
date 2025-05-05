from PySide6 import QtWidgets
from bs4 import BeautifulSoup


class MainWidget(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()
        self.bs = BeautifulSoup("<html></html>")
        self.layout_ = QtWidgets.QVBoxLayout(self)
        self.html_edit = QtWidgets.QPlainTextEdit()
        self.layout_.addWidget(self.html_edit)
        self.selector_edit = QtWidgets.QLineEdit(self)
        self.layout_.addWidget(self.selector_edit)
        self.html_edit.textChanged.connect(self.update_html)
        self.selector_edit.textEdited.connect(self.search)

    def update_html(self):
        self.bs = BeautifulSoup("<html>%s</html>" % self.html_edit.toPlainText())

    def search(self):
        try:
            for a in self.bs.select(self.selector_edit.text()):
                print(a)
        except:
            pass


def main():
    app = QtWidgets.QApplication([])
    mw = MainWidget()
    mw.show()
    return app.exec()


if __name__ == "__main__":
    main()
