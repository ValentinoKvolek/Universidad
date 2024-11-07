.data

base: .word 5
exponente: .word 4
result: .word 0

.code

daddi $t0, $0, base #por referencia 
daddi $t1, $0, exponente #por referencia 

daddi $sp,$0,0x400

daddi $sp,$sp,-16

sd $t0,0($sp)

sd $t1,8($sp)

jal potencia

sd $v0, result($zero)

halt

potencia: 

    daddi $v0, $zero, 1

    ld $t0, 0($sp)

    ld $t1, 8($sp)

    ld $t0,0($t0)

    ld $t1,0($t1)

    daddi $sp,$sp,16

lazo: beqz $t1, terminar
    daddi $t1, $t1, -1
    dmul $v0, $v0, $t0
    j lazo

terminar: jr $ra