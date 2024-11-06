.data

mensaje: .asciiz "ingrese una clave de 4 digitos \n"

mensaje2: .asciiz "contraseña incorrecta"

mensaje3: .asciiz "contraseña correcta"

CONTROL: .word 0x10000

DATA: .word 0x10008

caracteres: .space 4

password: .ascii "aaaa"

X: .word 0

stringAux: .ascii " intentos restantes"



.code

    daddi $t9, $zero, 6 #intentos

    ld $t0, CONTROL($zero) #guardo el VALOR  en t0

    ld $t1, DATA($zero) #guardo el VALOR  en t1


intento: daddi $t9, $t9, -1 

    sd $t9, X($zero)

    daddi $t2, $zero , mensaje #guardo la direccion en t2

    sd $t2, 0 ($t1) # DATA recibe el puntero al comienzo del mensaje

    daddi $t2, $zero, 4 #aca remplazo el valor de t2 y lo pongo en 4 para imprimir ascci

    sd $t2, 0($t0) ; le paso a CONTROL el 4 y imprime

    daddi $t2, $zero , X #guardo la direccion en t2

    sd $t2, 0 ($t1) # DATA recibe el puntero al comienzo del mensaje

    daddi $t2, $zero, 1 #aca remplazo el valor de t2 y lo pongo en 4 para imprimir ascii

    sd $t2, 0($t0) ; le paso a CONTROL el 1 y imprime

    daddi $t2, $zero , stringAux #guardo la direccion en t2

    sd $t2, 0 ($t1) # DATA recibe el puntero al comienzo del mensaje

    daddi $t2, $zero, 4 #aca remplazo el valor de t2 y lo pongo en 4 para imprimir ascci

    sd $t2, 0($t0) ; le paso a CONTROL el 4 y imprime

    daddi $t3, $zero , 4 #para ingresar 4 char
    daddi $t8, $zero , 0

loop:

        daddi $t2, $zero, 9 # pongo el 9 para leer un char

        sd $t2,0($t0) #control == 9

        #presiono una tecla y la cpu guarda el char en DATA

        lbu $t2, 0($t1) #lo tomo el caracter en t2

        #guardo en variable

        sb $t2, caracteres($t8) 

        daddi $t3, $t3, -1
        daddi $t8,$t8, 1

    bnez $t3, loop

    daddi $t3, $zero , 4 #cant vecees
    daddi $t7, $zero , 0 # lo uso para desplazarme en ascci

lazo:

    ld $t5,caracteres($t7)
    ld $t6,password($t7)

    bne $t5,$t6,noEs

    daddi $t7, $t7, 1
    daddi $t3, $t3, -1

beqz $t3, lazo

daddi $t2, $zero , mensaje3 #guardo la direccion en t2

daddi $t3, $zero , 4 #para ingresar 4 char

sd $t2, 0 ($t1) # DATA recibe el puntero al comienzo del mensaje

daddi $t2, $zero, 4 #aca remplazo el valor de t2 y lo pongo en 4 para imprimir ascci

sd $t2, 0($t0) ; le paso a CONTROL el 4 y imprime

j fin


noEs:   daddi $t2, $zero , mensaje2 #guardo la direccion en t2

        daddi $t3, $zero , 4 #para ingresar 4 char

        sd $t2, 0 ($t1) # DATA recibe el puntero al comienzo del mensaje

        daddi $t2, $zero, 4 #aca remplazo el valor de t2 y lo pongo en 4 para imprimir ascci

        sd $t2, 0($t0) ; le paso a CONTROL el 4 y imprime

        beqz $t9,fin

    j intento


fin: halt
