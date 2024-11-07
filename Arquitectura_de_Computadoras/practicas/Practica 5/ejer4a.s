.data
base: .word 5
exponente: .word 4
result: .word 0
.code
daddi $a0, $0, base #por referencia 
daddi $a1, $0, exponente #por referencia 
jal potencia
sd $v0, result($zero)
halt
potencia: daddi $v0, $zero, 1
    ld $t0,0($a0)
    ld $t1,0($a1)
    lazo: beqz $t1, terminar
    daddi $t1, $t1, -1
    dmul $v0, $v0, $t0
j lazo
terminar: jr $ra