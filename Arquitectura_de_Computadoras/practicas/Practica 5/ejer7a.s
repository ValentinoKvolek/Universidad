.data

string: .asciiz "LUCIA"
cont: .word 0

.code

daddi $a0, $0, string #por referencia 

jal longitud

sd $v0, cont($zero)

halt

longitud: 

        lbu $t0,0($a0) #cargo la direccio de memoria en t0

        daddi $t1, $zero,1

        daddi $v0,$0,0 #pongo en v0 un 0 por las dudas

        lazo: 
            beqz $t0, fin   
            lbu $t0,0($t1)
            daddi $t1,$t1,1
            daddi $v0,$v0,1
            j lazo
        fin: 

        jr $ra
            
                
