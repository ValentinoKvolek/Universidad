.data

n: .word 4


result: .word 0

.code


daddi $sp, $0, 0x400 #guardo en sp la direccion de la  pila

ld $a0, n($zero)

jal factorial

sd $v0, result($zero)


halt

factorial: 

    beqz $a0, factoriaRecursivo

    daddi $v0,$0,1
    
    daddi $sp,$sp,-16 # cada vez que itero por que a0 no es 0 me voy a guardar el numero que n para despues factorizarlo

    sd $ra, 0($sp)
    sd $a0, 8($sp)

    daddi $a0,$a0,-1

    jal factorial  # hasta que sea cero sigo saltando y guardando tanto el numero n como la pos de retornor en la pila

   factoriaRecursivo:

        daddi $t0,$a0, 0 

        ld $ra, 0($sp) 
        ld $a0, 8($sp) 
        daddi $sp,$sp,16

        dmul $v0,$v0,$a0 #aca hago el factorial.

        daddi $a0,$0,0

        jr $ra