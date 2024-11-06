.data
n: .byte 4
f: .word 1
i: .byte 1

.code

ld r1, n($zero) ; r1 es mi N

ld r2, f($zero) ; r2 es mi F

ld r3, i($zero) ; r3 es mi i



loop:

        dmul r2,r2,r3 ;multiplico a n n veces

        daddi r3,r3, 1 ;sumo n veces 1 al numero

        daddi r1, r1, -1 ; n-1

        bnez r1, loop ;  si n no es  0 sigo

sd r2, f($zero)

halt


