import os,sys,shutil
import_dir=os.path.abspath(os.path.join(os.path.dirname(__file__), './lib'))
sys.path.append(import_dir)
sys.path.append(import_dir+"""/textual""")
sys.path.append(import_dir+"""/textual_canvas""")
from lib import manage, textual, textual_canvas
from lib.textual_canvas.textual_canvas import Canvas
from lib.textual.textual.binding import Binding
from lib.textual.textual.app import App, ComposeResult
from lib.textual.textual.color import Gradient, Color
from lib.textual.textual.widgets import Static, Header, Footer, ProgressBar
from lib.textual.textual.theme import Theme
