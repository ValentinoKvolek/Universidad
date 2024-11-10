.data

n: .word 2
m: .word 3

resultfinal: .word 0

.code

daddi $sp, $0, 0x400 #guardo en sp la direccion de la  pila


ld $a0, m($zero) #paso a m
ld $a1, n($zero) #paso a n 

jal comb


sd $v0, resultfinal($zero)



halt
#recibe M por $a0 y N por $a1, calculando M!/(N!*(M-N)!)
comb: 

    daddi $sp,$sp,-8
    sd $ra,0($sp) #me guardo la direccion de retorno

    daddi $sp,$sp,-8
    sd $s0,0($sp) #guardo los registro s0 en la pila

    daddi $sp,$sp,-8
    sd $s1,0($sp) #guardo los registro s0 en la pila


    daddi $s0, $a0, 0 #guarde M
    daddi $s1, $a1, 0 #guarde N

    jal factorial

    daddi $t0, $v0,0 # m!  

    daddi $a0, $s1,0  #a0 con N

    jal factorial

    daddi $t1, $v0,0 # n!

    dsub $t2,$s0,$s1 # M-N

    daddi $a0,$t2,0 #lo guardo en a0 (el resultado de m-n)

    jal factorial

    daddi $t3,$v0,0 #(M-N)!

    dmul $t4,$t1,$t3 # n! * (m-n)!

    ddiv $v0,$t0,$t4 # m! / (n! * (n-m)!)

    #vuelvo a el estado inicial de la pila
    
    ld $s1,0($sp)
    daddi $sp,$sp,8

    ld $s0,0($sp) 
    daddi $sp,$sp,8

    
    ld $ra,0($sp) 
    daddi $sp,$sp,8
    
    jr $ra


factorial: 

    daddi $v0,$0,1

    lazo:   beqz $a0, fin

            dmul $v0,$v0,$a0

            daddi $a0,$a0, -1

            j lazo
            
    fin: jr $ra

