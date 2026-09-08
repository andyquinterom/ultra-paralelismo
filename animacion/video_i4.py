from manim import *
from comun import *


class VideoI4(Scene):
    def construct(self):
        titulo = chip("i4 · rayon + SIMD", COL_I4, font_size=34, pad_x=0.4, pad_y=0.18)
        titulo.move_to(UP * 3.4)
        barra = RoundedRectangle(corner_radius=0.08, width=3.8, height=0.55)
        barra.set_stroke(COL_I4, 2).move_to(UP * 2.35)
        etiqueta = Text("1 000 000 000 datos", font_size=24, color=COL_TEXTO)
        etiqueta.move_to(barra.get_center())
        self.play(FadeIn(titulo, shift=DOWN * 0.3), run_time=0.7)
        self.play(Create(barra), FadeIn(etiqueta), run_time=0.9)
        self.wait(0.5)

        bloques = VGroup()
        for _ in range(4):
            b = RoundedRectangle(corner_radius=0.06, width=0.86, height=0.55)
            b.set_stroke(COL_I4, 2)
            bloques.add(b)
        bloques.arrange(RIGHT, buff=0.12).move_to(UP * 2.35)
        hilos = VGroup()
        for i in range(4):
            c = Circle(radius=0.3).set_stroke(COL_I4, 2)
            t = Text(f"T{i}", font_size=22, weight=BOLD, color=COL_I4).move_to(c)
            hilos.add(VGroup(c, t))
        hilos.arrange(RIGHT, buff=0.55).move_to(UP * 1.15)
        self.play(FadeOut(etiqueta), FadeIn(bloques), run_time=0.7)
        self.play(barra.animate.fade(0.85), FadeIn(hilos), run_time=0.6)
        lineas = VGroup(
            *[
                Line(
                    bloques[i].get_bottom(),
                    hilos[i].get_top(),
                    stroke_width=2.5,
                    color=COL_GRIS,
                )
                for i in range(4)
            ]
        )
        self.play(Create(lineas), run_time=0.6)
        self.wait(0.3)

        parciales = VGroup()
        for i in range(4):
            p = chip(f"P{i}", COL_I4, font_size=22, pad_x=0.16, pad_y=0.08)
            parciales.add(p)
        parciales.arrange(RIGHT, buff=0.5).move_to(DOWN * 0.65)
        flechas_p = VGroup(
            *[
                Arrow(
                    hilos[i].get_bottom(),
                    parciales[i].get_top(),
                    buff=0.08,
                    stroke_width=4,
                    color=COL_GRIS,
                    max_tip_length_to_length_ratio=0.35,
                )
                for i in range(4)
            ]
        )
        self.play(
            LaggedStart(*[GrowArrow(a) for a in flechas_p], lag_ratio=0.08), run_time=0.7
        )
        self.play(
            LaggedStart(*[FadeIn(p, scale=0.7) for p in parciales], lag_ratio=0.08),
            run_time=0.7,
        )
        self.wait(0.3)

        sigma_chip = chip("Σ", COL_ACENTO, font_size=30, pad_x=0.2, pad_y=0.1)
        sigma_chip.move_to(DOWN * 1.7)
        flechas_s = VGroup(
            *[
                Arrow(
                    parciales[i].get_bottom(),
                    sigma_chip.get_top(),
                    buff=0.08,
                    stroke_width=4,
                    color=COL_GRIS,
                    max_tip_length_to_length_ratio=0.3,
                )
                for i in range(4)
            ]
        )
        self.play(
            LaggedStart(*[GrowArrow(a) for a in flechas_s], lag_ratio=0.06), run_time=0.7
        )
        self.play(FadeIn(sigma_chip, scale=0.7), run_time=0.6)
        res_sigma = MathTypst("sigma = 2", font_size=44)
        res_sigma.set_color(COL_I4)
        res_ms = Text("· 32 ms", font_size=36, weight=BOLD, color=COL_I4)
        resultado = VGroup(res_sigma, res_ms).arrange(RIGHT, buff=0.25)
        resultado.move_to(DOWN * 2.6)
        self.play(Write(resultado), run_time=0.9)
        self.play(Indicate(resultado, scale_factor=1.15), run_time=0.7)
        self.wait(1.5)
        if self.mobjects:
            self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.6)
