.data


V: .word 5, 2, 6

res: .word 0

reps: .byte 3


.code

ld $v0,res($zero) ;pongo en $v0 res

ld $v1,reps($zero)

daddi $t2, $zero, V  ; t2 tiene la direccion de V

loop: 

    ld $t1, 0($t2) 

    dadd $v0,$v0,$t1 ;sumo a res el  N

    daddi $t2,$t2,8 ;sumo en t2 el 8 para correrme al otro numero (2)

    daddi $v1,$v1,-1

bnez $v1,loop

sd $v0,res(r0)

halt