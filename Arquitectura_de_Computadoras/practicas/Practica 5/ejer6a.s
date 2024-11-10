.data

n: .word 7


result: .word 0

.code

ld $a0, n($zero)

jal factorial

sd $v0, result($zero)


halt

factorial: 

    daddi $v0,$v0,1

    lazo:   beqz $a0, fin

            dmul $v0,$v0,$a0

            daddi $a0,$a0, -1


            j lazo
            
    fin: jr $ra


