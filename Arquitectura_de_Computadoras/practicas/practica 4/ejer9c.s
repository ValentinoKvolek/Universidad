.data

cadena: .asciiz "ArquiTectuRa de ComPutaDoras"

mayus: .word 0


.code

daddi $t0,$zero,0 ;lo uso para moverme

daddi $t1,$zero,64; me guardo el char A

daddi $t2,$zero,91 ; me guardo el char Z

ld $t3,mayus($zero) ;para contar la cantidad de mayus

loop:   

    lbu $t4,cadena($t0) ;me posicion en cadena donde este $t0 

    slt $t5,$t1,$t4 ; si t4 es mas grande que A 
    beqz $t5,noEs

    slt $t5,$t4,$t2 ;si t4 es mas chico que Z
    beqz $t5,noEs

    daddi $t3,$t3,1

noEs:
    daddi $t0,$t0,1 ;me muevo de 1 byte por que es ascii

    bnez $t4,loop

sd $t3,mayus($zero)


halt