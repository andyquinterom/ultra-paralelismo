import math
import random
import time
from array import array

# Python puro, un solo hilo. array('d') guarda los double contiguos (8 B/elemento).
N = 1_000_000_000

def desviacion_estandar(datos):
    n = len(datos)
    media = sum(datos) / n
    total = 0.0
    for x in datos:
        diff = x - media
        total += diff * diff
    return math.sqrt(total / n)


print(f"Generando {N:,} observaciones aleatorias (float)...")
t0 = time.perf_counter()
x = array("d", (random.random() for _ in range(N)))
t1 = time.perf_counter()
print(f"Generacion: {t1 - t0:.3f} s")

times = []

for i in range(0, 10):
    t2 = time.perf_counter()
    std = desviacion_estandar(x)
    t3 = time.perf_counter()
    print(f"Iteracion {i}: {t3 - t2:.3f} s")
    times.append(t3 - t2)

times.sort()
print(f"Desviacion estandar: {std:.6f}")
print(f"{times[5]:.3f}")
