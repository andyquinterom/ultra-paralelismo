import os
from manim import *

config.background_color = "#0d1117"
config.pixel_width = 1080
config.pixel_height = 1920
config.frame_width = 4.5
config.frame_height = 8.0

if os.environ.get("PREVIEW"):
    config.pixel_width = 432
    config.pixel_height = 768

DATOS = [2, 4, 4, 4, 5, 5, 7, 9]
SUMA_PARCIAL = [2, 6, 10, 14, 19, 24, 31, 40]
SUMA_CUAD_PARCIAL = [4, 20, 36, 52, 77, 102, 151, 232]
DIF_CUAD = [9, 1, 1, 1, 0, 0, 4, 16]
DIF_PARCIAL = [9, 10, 11, 12, 12, 12, 16, 32]

TIEMPOS = [
    ("i1", "981 ms", "1×", "#58C4DD", 981.5),
    ("i2", "513 ms", "1.9×", "#7BD36E", 513.3),
    ("i3", "103 ms", "9.6×", "#FF9051", 102.7),
    ("i4", "32 ms", "30×", "#C78FE2", 32.4),
]

COL_I1 = "#58C4DD"
COL_I2 = "#7BD36E"
COL_I3 = "#FF9051"
COL_I4 = "#C78FE2"
COL_ACENTO = "#FFD35C"
COL_ROJO = "#FF6B6B"
COL_TEXTO = "#E8E8E8"
COL_GRIS = "#9AA4B2"


def chip(texto, color, font_size=24, pad_x=0.2, pad_y=0.1):
    t = Text(texto, font_size=font_size, weight=BOLD)
    fondo = RoundedRectangle(
        corner_radius=0.1, width=t.width + pad_x * 2, height=t.height + pad_y * 2
    )
    fondo.set_stroke(color, 2).set_fill(color, 0.15)
    return VGroup(fondo, t.move_to(fondo.get_center()))


def celdas_datos(valores, lado=0.46, buff=0.055, color=WHITE, font_size=26):
    grupo = VGroup()
    for v in valores:
        cuadro = RoundedRectangle(corner_radius=0.06, width=lado, height=lado)
        cuadro.set_stroke(color, 1.6)
        num = Text(f"{v:g}", font_size=font_size, color=COL_TEXTO).move_to(cuadro)
        grupo.add(VGroup(cuadro, num))
    grupo.arrange(RIGHT, buff=buff)
    return grupo
