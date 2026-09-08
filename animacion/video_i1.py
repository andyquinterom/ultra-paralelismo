from manim import *
from comun import *


class VideoI1(Scene):
    def construct(self):
        titulo = chip("i1 · dos pasadas", COL_I1, font_size=34, pad_x=0.4, pad_y=0.18)
        titulo.move_to(UP * 3.35)
        formula = MathTypst("sigma = sqrt((sum (x_i - mu)^2) / n)", font_size=34)
        formula.set_color(COL_GRIS).move_to(UP * 2.4)
        celdas = celdas_datos(DATOS).move_to(UP * 1.35)
        self.play(FadeIn(titulo, shift=DOWN * 0.3), run_time=0.7)
        self.play(Write(formula), run_time=1.0)
        self.play(
            LaggedStart(*[FadeIn(c, scale=0.5) for c in celdas], lag_ratio=0.08),
            run_time=1.1,
        )

        cursor = (
            SurroundingRectangle(celdas[0], buff=0.03)
            .set_fill(COL_ACENTO, 0.45)
            .set_stroke(width=0)
            .set_z_index(-1)
        )
        acc_ref1 = MathTypst("sum x = 40", font_size=36).move_to(DOWN * 0.1)
        lbl_acc1 = MathTypst("sum x =", font_size=36).set_color(COL_I1)
        lbl_acc1.align_to(acc_ref1, LEFT).match_y(acc_ref1)
        val_acc1 = MathTypst("0", font_size=36).set_color(COL_I1)
        val_acc1.next_to(lbl_acc1, RIGHT, buff=0.15)
        label1 = Text("pasada 1 · sumar", font_size=26, color=COL_GRIS)
        label1.next_to(acc_ref1, DOWN, buff=0.35)
        self.play(FadeIn(cursor), FadeIn(lbl_acc1), FadeIn(val_acc1), run_time=0.6)
        self.play(FadeIn(label1), run_time=0.5)
        for k in range(8):
            nuevo_val = MathTypst(f"{SUMA_PARCIAL[k]}", font_size=36).set_color(COL_I1)
            nuevo_val.next_to(lbl_acc1, RIGHT, buff=0.15)
            self.play(
                cursor.animate.move_to(celdas[k].get_center()),
                FadeTransform(val_acc1, nuevo_val),
                run_time=0.32,
            )
            val_acc1 = nuevo_val
        self.wait(0.4)
        media = MathTypst("mu = 40/8 = 5", font_size=36)
        media.set_color(COL_TEXTO).move_to(DOWN * 1.35)
        self.play(FadeOut(label1), Write(media), run_time=0.9)
        self.wait(0.5)

        label2 = Text("pasada 2 · sumar cuadrados", font_size=26, color=COL_GRIS)
        label2.next_to(acc_ref1, DOWN, buff=0.35)
        acc_ref2 = MathTypst("sum (x - mu)^2 = 32", font_size=36).move_to(DOWN * 0.1)
        lbl_acc2 = MathTypst("sum (x - mu)^2 =", font_size=36).set_color(COL_ACENTO)
        lbl_acc2.align_to(acc_ref2, LEFT).match_y(acc_ref2)
        val_acc2 = MathTypst("0", font_size=36).set_color(COL_ACENTO)
        val_acc2.next_to(lbl_acc2, RIGHT, buff=0.15)
        self.play(
            FadeIn(label2),
            FadeTransform(lbl_acc1, lbl_acc2),
            FadeTransform(val_acc1, val_acc2),
            cursor.animate.move_to(celdas[0].get_center()),
            run_time=0.7,
        )
        diffs = []
        for k in range(8):
            d = Text(f"{DIF_CUAD[k]}", font_size=24, color=COL_ACENTO)
            d.move_to(celdas[k].get_top() + UP * 0.3)
            diffs.append(d)
            nuevo_val = MathTypst(f"{DIF_PARCIAL[k]}", font_size=36).set_color(COL_ACENTO)
            nuevo_val.next_to(lbl_acc2, RIGHT, buff=0.15)
            self.play(
                cursor.animate.move_to(celdas[k].get_center()),
                FadeIn(d, scale=0.5),
                FadeTransform(val_acc2, nuevo_val),
                run_time=0.32,
            )
            val_acc2 = nuevo_val
        self.wait(0.4)
        varianza = MathTypst('"var" = 32/8 = 4', font_size=36)
        varianza.set_color(COL_TEXTO).move_to(DOWN * 1.35)
        self.play(Transform(media, varianza), run_time=0.9)
        self.wait(0.4)
        sigma = MathTypst("sigma = sqrt(4) = 2", font_size=52)
        sigma.set_color(COL_I1).move_to(DOWN * 2.45)
        self.play(Write(sigma), run_time=1.0)
        self.play(Indicate(sigma, scale_factor=1.25), run_time=0.7)
        self.wait(1.5)
        if self.mobjects:
            self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.6)
