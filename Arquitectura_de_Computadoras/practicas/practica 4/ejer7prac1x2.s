.data

v: .word -10, 5, -2, 7, 3, 60, -60, 77, -1, 10


.code

daddi r2, r0, 0 ;lo uso para desplazarme
daddi r3,r0, 2 ;lo uso para multiplicar por 2
daddi r6,r0,10 ;cant reps

loop:
    ld r1,v(r2)

    dmul r1,r1,r3

    sd r1,v(r2)

    daddi r2,r2,8 ;para desplazarme


    daddi r6,r6,-1
bnez r6, loop 

halt