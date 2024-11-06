.data

fibonacci: .space 80

primerNum: .word 0

segundoNum: .word 1

.code

    daddi $t6,$zero,10

    daddi $t3,$zero,0

    ld $t0, fibonacci($t3)

    ld $t1,primerNum($zero)

    ld $t2,segundoNum($zero)

    sd $t1,fibonacci($t3)

    daddi $t3,$t3,8

    sd $t2,fibonacci($t3)

    daddi $t3, $t3, 8 

    daddi $t4,$zero,2  ;empiezo a contar en 2 por que ya tengo los dos primero nums

    loop: 
       dadd $t5, $t1, $t2

        sd $t5, fibonacci($t3)
        
        daddi $t3,$t3,8

        dadd $t1, $zero, $t2 ;lo que habia en t2 en t1 

        dadd $t2, $zero, $t5; y t2 es el suma de los numeros

        daddi $t4,$t4,1

    bne $t4, $t6, loop
    
    halt




