.data

cadena: .asciiz "adbdcdedfdgdhdid" # cadena a analizar
car: .ascii "d" # carácter buscado
cant: .word 0 # cantidad de veces que se repite el carácter car en cadena.

.code

daddi $t0,$zero,0

lbu $t2,car($zero) ;char d

ld $t3,cant($zero) ;cant

loop:

    lbu $t1,cadena($t0) 

    bne $t1,$t2, noCuenta

    daddi $t3,$t3,1

noCuenta:
    daddi $t0,$t0,1
    
    bnez $t1,loop ;como esta en asciiz yo ya se que va a terminar con 0

    sd $t3,cant($zero)
halt