.data

CONTROL: .word 0x10000
DATA: .word 0x10008
base: .byte 0
altura: .byte 0 
superficie: .double 0

.code
daddi $t7,$zero,2
ld $t0, CONTROL($zero)
ld $t1, DATA($zero)

daddi $t2, $zero, 8 # para leer un numero
sd $t2,0($t0) #control == 8
#presiono una tecla y la cpu guarda el num en DATA
lbu $t2, 0($t1) #lo tomo el caracter en t
#guardo en variable
sb $t2, base($zero)
daddi $t2, $zero, 8 # para leer un numero
sd $t2,0($t0) #control == 8
#presiono una tecla y la cpu guarda el num en DATA
lbu $t2, 0($t1) #lo tomo el caracter en t
#guardo en variable
sb $t2, altura($zero)

ld $t3,base($zero)

ld $t4,altura($zero)

dmul $t5,$t3,$t4

ddiv $t6,$t5,$t7


sd $t6,superficie($zero)



halt