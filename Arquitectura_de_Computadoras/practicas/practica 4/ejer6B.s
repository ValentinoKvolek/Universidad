.data


V: .word 5, 2, 6

res: .word 0

reps: .byte 3


.code

ld $v0,res($zero) ;pongo en $v0 res

ld $v1,reps($zero)

daddi $t2,r0,0 ;al principio t2 vale 0 por que quieor acceder al 5

loop: 

    ld $t1, V($t2)  ;pongo en t1 el v+8

    dadd $v0,$v0,$t1 ;sumo a res el  5

    daddi $t2,$t2,8 ;sumo en t2 el 8 para correrme al otro numero (2)

    daddi $v1,$v1,-1

bnez $v1,loop

sd $v0,res(r0)

halt