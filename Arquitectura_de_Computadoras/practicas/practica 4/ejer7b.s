.data

v: .word 1


.code

    daddi r6,r0,9 ;cant repts
    
    daddi r2, r0, 0 ;lo uso para desplazarme

    daddi r4, r0, 2;
    
    ld r1,v(r2); me pos en 1

loop:
    
    dadd r1,r1,r4 ;le sumo al impar que esta 2

    daddi r2,r2,8 ;para desplazarme

    sd r1,v(r2) ;guardo el impar siguiente 

    daddi r6,r6,-1 ; asi hago 9 veces (por que ya tengo el 1)

bnez r6, loop 

halt