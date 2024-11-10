.data 

tabla: .word 4,5,3,2,1

longitud: .word 5

total: .word 0


.code

daddi $a0,$0,tabla 

ld $a1, longitud($zero) 

jal suma

sd $v0,total($zero)

halt


suma: daddi $v0,$0,0

lazo: beqz $a1, fin
    ld $t0,0($a0)
    dadd $v0,$v0,$t0
    daddi $a0,$a0,8
    daddi $a1,$a1,-1
    j lazo

fin: jr $ra 