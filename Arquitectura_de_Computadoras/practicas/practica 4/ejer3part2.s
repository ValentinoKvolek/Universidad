.data

CONTROL: .word 0x10000

DATA: .word 0x10008

num1: .byte 0

num2: .byte 0

operacion: .ascii 

suma: .ascii "+"

resta: .ascii "-"

mull: .ascii "*"

divv: .ascii "/"



.code

ld $s0,suma($zero)
ld $s1,resta($zero)
ld $s2,mull($zero)
ld $s3,divv($zero)

ld $t0, CONTROL($zero)
ld $t1, DATA($zero)

daddi $t2, $zero, 8 # para leer un numero
sd $t2,0($t0) #control == 8

#presiono una tecla y la cpu guarda el num en DATA

lbu $t2, 0($t1) #lo tomo el caracter en t2

#guardo en variable

sb $t2, num1($zero)


daddi $t2,$zero,9 #leer ascii
sd $t2,0($t0) #control == 9
lbu $t2, 0($t1) #lo tomo el caracter en t2
sb $t2, operacion($zero)



#una vez guarde los dos numeros los sumo
ld $t3,num1($zero)

ld $t4,num2($zero)

ld $t6,operacion($zero)


daddi $t2, $zero, 8 # para leer un numero
sd $t2,0($t0) #control == 8

#presiono una tecla y la cpu guarda el num en DATA

lbu $t2, 0($t1) #lo tomo el caracter en t2

#guardo en variable

sb $t2, num2($zero)

beq $t6,$s0,sumar
beq $t6,$s1,restar
beq $t6,$s2,multiplicar
beq $t6,$s3,dividir

sumar:

    dadd $t5,$t3,$t4
    dadd $t7,$zero,%t5

    j fin
restar: 

    dsub $t5,$t3,$t4
    dadd $t7,$zero,%t5
    j fin

dividir:

    ddiv $t5,$t3,$t4
    dadd $t7,$zero,%t5
    j fin

multiplicar: 

    dmul $t5,$t3,$t4
    dadd $t7,$zero,%t5
    j fin

fin:


    sd $t5,0($t1) #guardo lo que dio la suma en data para impimir
    daddi $t2,$zero, 2
    sd $t2,0($t0) #control == 2


halt