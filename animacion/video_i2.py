from manim import *
from comun import *


class VideoI2(Scene):
    def construct(self):
        titulo = chip("i2 · 1 pasada", COL_I2, font_size=34, pad_x=0.4, pad_y=0.18)
        titulo.move_to(UP * 3.35)
        formula = MathTypst("sigma = sqrt((sum x^2)/n - mu^2)", font_size=34)
        formula.set_color(COL_GRIS).move_to(UP * 2.35)
        celdas = celdas_datos(DATOS, color=COL_I2).move_to(UP * 1.3)
        self.play(FadeIn(titulo, shift=DOWN * 0.3), run_time=0.7)
        self.play(Write(formula), run_time=1.0)
        self.play(
            LaggedStart(*[FadeIn(c, scale=0.5) for c in celdas], lag_ratio=0.08),
            run_time=1.1,
        )

        acc_ref_x = MathTypst("sum x = 40", font_size=34).move_to(DOWN * 0.2 + LEFT * 1.05)
        lbl_acc_x = MathTypst("sum x =", font_size=34).set_color(COL_I2)
        lbl_acc_x.align_to(acc_ref_x, LEFT).match_y(acc_ref_x)
        val_acc_x = MathTypst("0", font_size=34).set_color(COL_I2)
        val_acc_x.next_to(lbl_acc_x, RIGHT, buff=0.12)

        acc_ref_x2 = MathTypst("sum x^2 = 232", font_size=34).move_to(DOWN * 0.2 + RIGHT * 1.05)
        lbl_acc_x2 = MathTypst("sum x^2 =", font_size=34).set_color(COL_I2)
        lbl_acc_x2.align_to(acc_ref_x2, LEFT).match_y(acc_ref_x2)
        val_acc_x2 = MathTypst("0", font_size=34).set_color(COL_I2)
        val_acc_x2.next_to(lbl_acc_x2, RIGHT, buff=0.12)

        self.play(FadeIn(lbl_acc_x), FadeIn(val_acc_x), FadeIn(lbl_acc_x2), FadeIn(val_acc_x2), run_time=0.6)
        cursor = (
            SurroundingRectangle(celdas[0], buff=0.03)
            .set_fill(COL_I2, 0.4)
            .set_stroke(width=0)
            .set_z_index(-1)
        )
        self.play(FadeIn(cursor), run_time=0.4)
        for k in range(8):
            nx_val = MathTypst(f"{SUMA_PARCIAL[k]}", font_size=34).set_color(COL_I2)
            nx_val.next_to(lbl_acc_x, RIGHT, buff=0.12)
            nx2_val = MathTypst(f"{SUMA_CUAD_PARCIAL[k]}", font_size=34).set_color(COL_I2)
            nx2_val.next_to(lbl_acc_x2, RIGHT, buff=0.12)
            self.play(
                cursor.animate.move_to(celdas[k].get_center()),
                FadeTransform(val_acc_x, nx_val),
                FadeTransform(val_acc_x2, nx2_val),
                run_time=0.32,
            )
            val_acc_x = nx_val
            val_acc_x2 = nx2_val
        self.wait(0.5)
        paso_mu = MathTypst("mu = sum x/n = 40/8 = 5", font_size=28)
        paso_mu.set_color(COL_TEXTO).move_to(DOWN * 1.15)
        self.play(Write(paso_mu), run_time=1.0)
        self.wait(0.4)

        paso_var = MathTypst("sigma^2 = 232/8 - 5^2", font_size=30)
        paso_var.set_color(COL_TEXTO).move_to(DOWN * 1.85)
        self.play(Write(paso_var), run_time=0.9)
        self.wait(0.4)

        paso_var2 = MathTypst("= 29 - 25 = 4", font_size=30)
        paso_var2.set_color(COL_TEXTO).move_to(DOWN * 2.5)
        self.play(Write(paso_var2), run_time=0.9)
        self.wait(0.5)

        sigma = MathTypst("sigma = sqrt(4) = 2", font_size=46)
        sigma.set_color(COL_I2).move_to(DOWN * 3.35)
        self.play(Write(sigma), run_time=1.0)
        self.play(Indicate(sigma, scale_factor=1.15), run_time=0.7)
        self.wait(1.5)
        if self.mobjects:
            self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.6)
