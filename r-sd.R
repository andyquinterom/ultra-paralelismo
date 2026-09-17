N <- 1000000000

cat(sprintf("Generando %s observaciones aleatorias (double)...\n", format(N, big.mark = ",")))
t0 <- proc.time()[["elapsed"]]
x <- runif(N)
t1 <- proc.time()[["elapsed"]]
cat(sprintf("Generacion: %.3f s\n", t1 - t0))

times <- numeric(10)

for (i in 0:9) {
  t2 <- proc.time()[["elapsed"]]
  s <- sd(x)
  t3 <- proc.time()[["elapsed"]]
  cat(sprintf("Iteracion %d: %.3f s\n", i, t3 - t2))
  times[i + 1] <- t3 - t2
}

times <- sort(times)
cat(sprintf("Desviacion estandar: %.6f\n", s))
cat(sprintf("Mediana: %.3f s\n", times[6]))
