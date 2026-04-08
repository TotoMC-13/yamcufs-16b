# Fibonacci calculator
# En t0 guardo F-2
# En t1 guardo F-1
# En t2 guardo F Actual
# En t3 guardo el contador
# En a0 guardo el n maximo de la secuencia de Fibonacci

# Inicialización para n = 23 (Limite 16b C2)
addi a0, r0, 6   # a0 = 6
addi t0, r0, 2   # Uso t0 momentaneamente para hacer el shift
sll a0, a0, t0   # a0 = 24 (6 desplazado 2 posiciones)
addi a0, a0, -1  # a0 = 23 (Ajustamos al limite seguro con signo)


addi t0, r0, 0 # Inicializo F-2
addi t1, r0, 1 # Inicializo F-1
addi t3, r0, 1 # Arrancamos el contador en 1 porque ya tenemos pre-calculado n=1

loop:
beq a0, t3, halt # Chequeamos si ya llegamos al n deseado
add t2, t1, t0 # t2 = 1
add t0, t1, r0 # Muevo n-1 a n-2
add t1, t2, r0 # Muevo el resultado nuevo a n-1 para calcular el proximo
addi t3, t3, 1
j loop

halt:
j halt