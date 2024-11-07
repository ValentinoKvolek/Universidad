.data
base: .word 5
exponente: .word 4
result: .word 0

.code
daddi $sp, $0, 0x400

ld $t0, base($zero)
ld $t1, exponente($zero)

daddi $sp,$sp,-16

sd $t0,0($sp)
sd $t1,8($sp)

jal potencia

sd $v0, result($zero)

halt

potencia: daddi $v0, $zero, 1

    ld $t2, 0($sp)
    ld $t3, 8($sp)
        daddi $sp,$sp,16

lazo: beqz $t3, terminar
    daddi $t3, $t3, -1
    dmul $v0, $v0, $t2
j lazo
terminar: jr $ra