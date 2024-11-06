
.data 

    texto: .asciiz "Hola, Mundo!" ; El mensaje a mostrar
    CONTROL: .word 0x10000
    DATA: .word 0x10008

.code
    daddi $t3, $zero , 4
    ld $t0, CONTROL($zero) ;$t0 = dirección de CONTROL
    ld $t1, DATA($zero) ;$t1 = dirección de DATA
    daddi $t2, $zero, texto ; $t2 = dirección del mensaje a mostrar 
    sd $t2, 0($t1); DATA recibe el puntero al comienzo del mensaje
    loop:
        daddi $t2, $zero, 4 ;para escribir en ascii
        sd $t2, 0($t0) ;le mando a contro el 4 para imprimir 
        daddi $t3, $t3 , -1
    bnez $t3,loop
halt