.data

v: .word -10, 5, -2, 7, 3, 60, -60, 77, -1, 10

max: .word 0 

.code

     daddi r2, r0, 0 ;lo uso para desplazarme
     daddi r4,r0,0 ;r4 lo uso para ver si tengo que guardar el max
     ld r3,max($zero)
     daddi r6,r0,10 ;cant reps

loop:
     ld r1,v(r2)
     slt r4,r3,r1
     bnez r4,almacenarMax
     j sigue

almacenarMax: dadd r3, r0,r1 ; guardo r1 en r3
    
    
sigue: daddi r6,r6,-1
        daddi r2,r2,8 ;para desplazarme
        bnez r6, loop 
    

sd r3,max($zero)
     
    
halt
