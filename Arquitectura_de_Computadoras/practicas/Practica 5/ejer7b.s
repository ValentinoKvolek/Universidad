    .data

    cont: .word 0

    string: .asciiz "ddad"

    aux: .ascii "a"

    .code

    daddi $a0, $0, string #por referencia 

    lbu $a1, aux($zero)

    jal contiene

    sd $v0, cont($zero)

    halt 
    
contiene:
    daddi $v0, $0, 0                    # contiene (boolean)

    loopContiene:
        lb $t0, 0($a0)
        beq $t0, $a1, siContiene
        beqz $t0, noContiene

        daddi $a0, $a0, 1
        j loopContiene
    
    siContiene:
        daddi $v0, $0, 1
        jr $ra
    
    noContiene:
        jr $ra