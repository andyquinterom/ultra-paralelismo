from manim import *
from comun import *


class VideoI3(Scene):
    def construct(self):
        titulo = chip("i3 · SIMD ×8", COL_I3, font_size=34, pad_x=0.4, pad_y=0.18)
        titulo.move_to(UP * 3.4)
        celdas = celdas_datos(DATOS, lado=0.4, buff=0.05, color=COL_I3, font_size=22)
        celdas.move_to(UP * 2.5)
        self.play(FadeIn(titulo, shift=DOWN * 0.3), run_time=0.7)
        self.play(
            LaggedStart(*[FadeIn(c, scale=0.5) for c in celdas], lag_ratio=0.05),
            run_time=0.8,
        )

        # Vector SIMD x (cargado desde datos)
        etiqueta_vx = MathTypst("v_x =", font_size=28).set_color(COL_I3)
        reg_x = VGroup()
        for d in DATOS:
            c = RoundedRectangle(corner_radius=0.04, width=0.38, height=0.38).set_stroke(COL_I3, 1.5)
            t = Text(f"{d}", font_size=20, color=COL_TEXTO).move_to(c)
            reg_x.add(VGroup(c, t))
        reg_x.arrange(RIGHT, buff=0.05)
        vec_x_grp = VGroup(etiqueta_vx, reg_x).arrange(RIGHT, buff=0.12).move_to(UP * 1.55)

        self.play(
            FadeIn(etiqueta_vx),
            LaggedStart(
                *[TransformFromCopy(celdas[k], reg_x[k]) for k in range(8)],
                lag_ratio=0.04,
            ),
            run_time=0.9,
        )
        self.play(celdas.animate.fade(0.7), run_time=0.4)

        # Operación SIMD: elevar al cuadrado vector * vector en paralelo
        op_simd_lbl = Text("8 multiplicaciones en 1 ciclo", font_size=20, color=COL_ACENTO)
        op_simd_lbl.move_to(UP * 0.95)
        flechas_down = VGroup(*[
            Arrow(reg_x[k].get_bottom() + DOWN * 0.04, reg_x[k].get_bottom() + DOWN * 0.36, buff=0, max_tip_length_to_length_ratio=0.35, stroke_width=2.5, color=COL_ACENTO)
            for k in range(8)
        ])
        
        # Vector SIMD x^2
        etiqueta_vx2 = MathTypst("v_x^2 =", font_size=28).set_color(COL_ACENTO)
        reg_x2 = VGroup()
        for d in DATOS:
            c = RoundedRectangle(corner_radius=0.04, width=0.38, height=0.38).set_stroke(COL_ACENTO, 1.5)
            t = Text(f"{d*d}", font_size=18, color=COL_TEXTO).move_to(c)
            reg_x2.add(VGroup(c, t))
        reg_x2.arrange(RIGHT, buff=0.05)
        vec_x2_grp = VGroup(etiqueta_vx2, reg_x2).arrange(RIGHT, buff=0.12).move_to(UP * 0.4)
        reg_x2.match_x(reg_x)
        etiqueta_vx2.next_to(reg_x2, LEFT, buff=0.12)

        self.play(Create(flechas_down), FadeIn(op_simd_lbl), run_time=0.6)
        self.play(
            FadeIn(etiqueta_vx2),
            LaggedStart(
                *[TransformFromCopy(reg_x[k], reg_x2[k]) for k in range(8)],
                lag_ratio=0.04,
            ),
            run_time=0.9,
        )
        self.wait(0.8)
        self.play(
            FadeOut(flechas_down),
            FadeOut(op_simd_lbl),
            FadeOut(vec_x_grp),
            FadeOut(vec_x2_grp),
            run_time=0.5,
        )

        # Árbol de reducción reduce_sum()
        filas_vals = [list(DATOS)]
        while len(filas_vals[-1]) > 1:
            v = filas_vals[-1]
            filas_vals.append([v[i] + v[i + 1] for i in range(0, len(v), 2)])
        filas = []
        for fila_vals in filas_vals:
            fila = VGroup()
            for val in fila_vals:
                circ = Circle(radius=0.18, stroke_color=COL_I3, stroke_width=2)
                t = Text(f"{val}", font_size=20, color=COL_TEXTO).move_to(circ)
                fila.add(VGroup(circ, t))
            fila.arrange(RIGHT, buff=0.16)
            filas.append(fila)
        y0 = 1.0
        for i, f in enumerate(filas):
            f.move_to(UP * (y0 - i * 0.7))

        reduce_lbl = Text("reduce_sum() horizontal", font_size=24, color=COL_I3)
        reduce_lbl.next_to(filas[0], UP, buff=0.35)
        self.play(FadeIn(reduce_lbl), FadeIn(filas[0], scale=0.6), run_time=0.7)

        for nivel in range(len(filas) - 1):
            abajo, arriba_f = filas[nivel], filas[nivel + 1]
            self.play(
                *[
                    ReplacementTransform(
                        VGroup(abajo[2 * j], abajo[2 * j + 1]).copy(), arriba_f[j]
                    )
                    for j in range(len(arriba_f))
                ],
                run_time=0.7,
            )
            self.wait(0.2)

        res_sum = MathTypst("sum x = 40", font_size=36).set_color(COL_I3)
        res_sum.next_to(filas[-1], DOWN, buff=0.4)
        self.play(FadeIn(res_sum), run_time=0.6)
        self.wait(1.5)
        if self.mobjects:
            self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.6)
