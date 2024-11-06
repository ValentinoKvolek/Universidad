.data

v: .word -10, 5, -2, 7, 3, 60, -60, 77, -1, 10

pos: .byte 0

.code


 daddi r2, r0, 0 ;lo uso para desplazarme

 ld r3,pos(r0);cant pos
 
 daddi r6,r0,10 ;cant reps

loop:

    ld r1,v(r2)

    andi r1,r1,128 ;comparo si es neg

    beqz r1, esPos

    j nosumo

    esPos: daddi r3,r3,1

nosumo: daddi r6,r6,-1

        daddi r2,r2,8 ;para desplazarme

bnez r6, loop   

sd r3,pos($zero)

halt

