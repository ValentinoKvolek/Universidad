.data


V: .word 5, 2, 6

res: .word 0


.code

ld $v0,res($zero) ;pongo en $v0 res

daddi $t2,r0,0 ;al principio t2 vale 0 por que quieor acceder al 5

ld $t1, V($t2)  ;pongo en t1 el 5

dadd $v0,$v0,$t1 ;sumo a res el  5

daddi $t2,$t2,8 ;sumo en t2 el 8 para correrme al otro numero (2)

ld $t1, V($t2) ;pongo en t1 el 2

dadd $v0,$v0,$t1 ;sumo a res el  2

daddi $t2,$t2,8 ;sumo en t2 el 8 para correrme al otro numero (6)

ld $t1, V($t2) ;pongo en t1 el 6

dadd $v0,$v0,$t1 ;sumo a res el  6

sd $v0,res(r0)


halt