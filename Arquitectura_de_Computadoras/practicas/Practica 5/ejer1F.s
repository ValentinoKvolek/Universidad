.data

CONTROL: .word 0x10000
DATA: .word 0x10008

base1: .word 3
base2: .word 2
exponente: .word 0
result: .word 0
aux: .word 0
aux2: .word 0

.code

    ld $s0, CONTROL($zero) #guardo control en s0

    ld $s1, DATA($zero) #guardo data en s1

    ld $a0, base1($zero) #guardo lo que esta en base en a0

    daddi $t2, $zero, 8 # para leer un numero
    sd $t2,0($s0) #control == 8

    ld $t2, 0($s1) #lo tomo el caracter de data  y lo pongo en t2

    sd $t2, exponente($zero) #coloco el numero que lei en la variable exponete

    ld $a1, exponente($zero) #guardo lo qeu esta en exponente en a1

    jal potencia

    sd $v0, aux($zero)


    ld $a0, base2($zero) #guardo lo que esta en base en a0
    ld $a1, exponente($zero) #guardo lo qeu esta en exponente en a1

    jal potencia

    sd $v0, aux2($zero)

    ld $t1,aux($zero)
    ld $t3,aux2($zero)

    dadd $v0,$t3,$t1

    sd $v0,0($s1) #guardo lo que me dio en data

    daddi $t2, $zero, 2 # para impimer numero
    sd $t2,0($s0) #control == 2

halt

potencia:  daddi $v0, $zero, 1
    
    lazo: beqz $a1, terminar
        daddi $a1, $a1, -1
        dmul $v0, $v0, $a0
    j lazo

terminar: jr $ra