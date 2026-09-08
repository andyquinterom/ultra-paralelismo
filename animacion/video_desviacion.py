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


class VideoCompleto(Scene):
    def construct(self):
        self.intro()
        self.escena_i1()
        self.escena_i2()
        self.escena_i3()
        self.escena_i4()
        self.comparativa()

    def limpiar(self):
        if self.mobjects:
            self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.5)

    def intro(self):
        arriba = Text("981 ms", font_size=88, weight=BOLD, color=COL_ROJO)
        flecha = Arrow(UP, DOWN, buff=0.18, stroke_width=12, color=COL_TEXTO)
        abajo = Text("32 ms", font_size=88, weight=BOLD, color=COL_I2)
        grupo = VGroup(arriba, flecha, abajo).arrange(DOWN, buff=0.3).move_to(UP * 0.9)
        sub = Text("mismo resultado", font_size=36, color=COL_GRIS)
        sub.next_to(grupo, DOWN, buff=0.55)
        hook = Text("¿cómo?", font_size=30, color=COL_GRIS, slant=ITALIC)
        hook.next_to(sub, DOWN, buff=0.4)
        self.play(Write(arriba), run_time=0.8)
        self.play(GrowArrow(flecha), run_time=0.6)
        self.play(Write(abajo), run_time=0.8)
        self.play(FadeIn(sub, shift=UP * 0.3), run_time=0.7)
        self.play(FadeIn(hook, shift=UP * 0.3), run_time=0.7)
        self.wait(2.0)
        self.limpiar()

    def escena_i1(self):
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
        self.wait(1.4)
        self.limpiar()

    def escena_i2(self):
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
        self.wait(1.4)
        self.limpiar()

    def escena_i3(self):
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
        # alinear las celdas de reg_x2 con reg_x en x
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
            f.move_to(UP * (y0 - i * 0.9))

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
        self.wait(1.4)
        self.limpiar()

    def escena_i4(self):
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
        self.wait(1.4)
        self.limpiar()

    def comparativa(self):
        titulo = Text("el resultado", font_size=40, weight=BOLD, color=COL_TEXTO)
        titulo.move_to(UP * 3.3)
        self.play(Write(titulo), run_time=0.9)
        y_filas = [2.2, 1.05, -0.1, -1.25]
        W = 1.5
        tmax = 981.5
        x_barra = -1.35
        filas = []
        for nombre, tiempo, sp, col, t in TIEMPOS:
            y = y_filas[len(filas)]
            c = chip(nombre, col, font_size=24, pad_x=0.16, pad_y=0.08)
            c.move_to(LEFT * 1.85 + UP * y)
            ancho = max(0.12, W * (t / tmax) ** 0.4)
            barra = Rectangle(width=ancho, height=0.34)
            barra.set_fill(col, 0.95).set_stroke(width=0)
            barra.move_to(RIGHT * (x_barra + ancho / 2) + UP * y)
            etiqueta = Text(f"{tiempo}", font_size=26, color=COL_TEXTO)
            etiqueta.move_to(UP * y).align_to(RIGHT * 0.25, LEFT)
            sp_chip = chip(sp, col, font_size=20, pad_x=0.14, pad_y=0.07)
            sp_chip.move_to(UP * y).align_to(RIGHT * 2.15, RIGHT)
            filas.append(VGroup(c, barra, etiqueta, sp_chip))
        for f in filas:
            self.play(
                FadeIn(f[0], shift=RIGHT * 0.2),
                GrowFromEdge(f[1], LEFT),
                FadeIn(f[2], shift=LEFT * 0.2),
                FadeIn(f[3], shift=LEFT * 0.2),
                run_time=0.8,
            )
            self.wait(0.25)
        self.play(Indicate(filas[3][2], scale_factor=1.4, color=COL_I4), run_time=0.9)
        cierre = Text("Rust · SIMD · rayon", font_size=32, weight=BOLD, color=COL_TEXTO)
        cierre.move_to(DOWN * 2.5)
        sub = Text("el mismo σ = 2 en todas", font_size=26, color=COL_GRIS)
        sub.next_to(cierre, DOWN, buff=0.35)
        self.play(Write(cierre), run_time=1.0)
        self.play(FadeIn(sub, shift=UP * 0.2), run_time=0.7)
        self.wait(2.2)
        self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.9)


class VideoI1(Scene):
    def construct(self):
        vc = VideoCompleto()
        vc.mobjects = self.mobjects
        vc.play = self.play
        vc.wait = self.wait
        vc.escena_i1()
        if self.mobjects:
            self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.6)


class VideoI2(Scene):
    def construct(self):
        vc = VideoCompleto()
        vc.mobjects = self.mobjects
        vc.play = self.play
        vc.wait = self.wait
        vc.escena_i2()
        if self.mobjects:
            self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.6)


class VideoI3(Scene):
    def construct(self):
        vc = VideoCompleto()
        vc.mobjects = self.mobjects
        vc.play = self.play
        vc.wait = self.wait
        vc.escena_i3()
        if self.mobjects:
            self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.6)


class VideoI4(Scene):
    def construct(self):
        vc = VideoCompleto()
        vc.mobjects = self.mobjects
        vc.play = self.play
        vc.wait = self.wait
        vc.escena_i4()
        if self.mobjects:
            self.play(*[FadeOut(m) for m in list(self.mobjects)], run_time=0.6)

