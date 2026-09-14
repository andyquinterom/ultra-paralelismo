import time

import numpy as np

N = 1_000_000_000

rng = np.random.default_rng()
print(f"Generando {N:,} observaciones aleatorias (float64)...")
t0 = time.perf_counter()
x = rng.random(N, dtype=np.float64)
t1 = time.perf_counter()
print(f"Generacion: {t1 - t0:.3f} s")

t2 = time.perf_counter()
std = np.std(x)
t3 = time.perf_counter()
print(f"Desviacion estandar: {std:.6f}")
print(f"Calculo de la desviacion estandar: {t3 - t2:.3f} s")
