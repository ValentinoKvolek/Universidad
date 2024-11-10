.data 

tabla: .word 4,-5,-3,-2,1

longitud: .word 5

total: .word 0


.code

daddi $sp, $0, 0x400 #guardo en sp la direccion de la  pila

daddi $a0,$0,tabla 

ld $a1, longitud($zero) 

jal suma

sd $v1,total($zero)

halt


suma: 

daddi $v0,$0,0
daddi $v1,$0,0
daddi $sp,$sp,-24 #hago espacio en la pila



sd $ra,0($sp) #me guardo la direccion de retorno
sd $s0,8($sp)
sd $s1,16($sp)



lazo: beqz $a1, fin

    ld $t0,0($a0) 

    daddi $s0,$a0,0
    daddi $s1,$v0,0

    daddi $a0,$t0,0 
    
    jal espositivo

    daddi $a0,$s0,0

    dadd $s1,$s1,$v0

    daddi $v0,$s1,0

    daddi $a1,$a1,-1

    daddi $a0,$a0,8

    j lazo

fin: 

    ld $ra,0($sp)
    ld $ra,0($sp) 
    ld $s0,8($sp) 
    ld $s1,16($sp) 
    daddi $sp,$sp, 24 

    jr $ra 


espositivo:

    slt $v0,$0,$a0

    jr $ra