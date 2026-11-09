import threading
import webbrowser
from pathlib import Path

from session_py import Session
from session_py.viewer import MatrixEquation, serve

scene = Session("1 · Matrix multiplication\nWatch the first row multiply the column vector.")
equation = MatrixEquation(scene, left=900, center_y=280, cell=64)
a = equation.matrix("A", [[1, 2], [3, 4]], r"\mathbf{A}")
equation.symbol(r"\times", "times")
x = equation.matrix("x", [[5], [6]], r"\mathbf{x}")
equation.symbol("=", "equals")
b = equation.matrix("B", [[17], [39]], r"\mathbf{B}")
equation.product(a, x, b, row=0, col=0, at=(1240, 520))
deck = [[scene]]

if __name__ == "__main__":
    viewer = serve(deck, directory=Path(__file__).with_name("generated-one-slide"))
    webbrowser.open(viewer.url)
    print(viewer.url)
    try:
        threading.Event().wait()
    except KeyboardInterrupt:
        pass
    finally:
        viewer.close()
