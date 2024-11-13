.data

CONTROL: .word 0x10000

DATA: .word 0x10008

men: .asciiz "Debe ingresar un numero"
men2: .asciiz "la cantida de impares fue : "

a: .byte 0

bb: .byte 10

max: .byte 20

tabla: .space 20

.code

daddi $sp,$0,0x400
ld $s0, CONTROL($zero) #guardo control en s0
ld $s1, DATA($zero) #guardo data en s1

daddi $s2,$0,0 #contador para long y despla
daddi $s3,$0,0
 
lazito: jal INGRESAR_NUMERO
    sb $v0,tabla($s2)

    daddi $s2,$s2,1
    dadd $s3,$s3,$v0 

    ld $t7,max($zero)
    slt $t0,$s3,$t7
    beqz $t0,fin2

j lazito


fin2:

    daddi $a0, $0, tabla # pongo en a0 la direccion de tabla

    dadd $a1,$0,$s2 # en a1 esta mi longitud(cantidad de elemento a contar) 

    jal PROCESAR_NUMEROS 

halt

INGRESAR_NUMERO: 

    daddi $sp,$sp,-8
    sd $ra, 0($sp) # me guardo la direccion de retorno

lazo: 
    daddi $t2, $zero, 8 # para leer un numero
    sd $t2,0($s0) #control == 8
    ld $t2, 0($s1) #lo tomo el caracter de data  y lo pongo en t2
    daddi $a0,$t2,0 # pongo el numero ingresado en a0 

    ld $a1,a($zero) # a1 = 0
    ld $a2,bb($zero) # a2 = 10
    
    jal ENTRE

    bnez $v0,imprimir # v0  = 1

     daddi $t2, $zero , men
     sd $t2, 0 ($s1)
     daddi $t2, $zero, 4
     sd $t2, 0($s0)

    j lazo

    imprimir:

        daddi $v0,$a0,0 
        ld $ra, 0($sp)
        daddi $sp,$sp,8
jr $ra

# a0 = num, a1:= 0, a2 = 10.
# B<N<A = 1 si no  0
ENTRE:
    slt $t0, $a1, $a0 # Si B<N
    beqz $t0,fin
    slt $t0,$a0,$a2  #si N<A 
    beqz $t0,fin
    fin: dadd $v0,$0,$t0  #t0 puede ser 1 0 0 dependiendo n
    jr $ra

#a0 la direccion de TABLA
#a1 recibe la cantidad de elementos a contar

PROCESAR_NUMEROS: 
    daddi $t0, $0,0 # desplamiento
    daddi $t5,$0,0 # contador
    daddi $sp,$sp,-8
    sd $ra, 0($sp) # me guardo la direccion de retorno

bucle:
    beqz $a1, fin3
    dadd $t1,$t0,$a0 #sumando la ubicacion de la tabla mas el indice para desplazarme
    lb $a2,0($t1) #pongo el numero a2
    daddi $a1,$a1,-1
    daddi $t0,$t0,1
    jal ES_IMPAR
    dadd $t5,$t5,$v0 
par:
    j bucle

fin3:

    daddi $t2, $zero , men2
    sd $t2, 0 ($s1)
    daddi $t2, $zero, 4
    sd $t2, 0($s0)

    dadd $t2,$0,$t5
    sd $t2,0($s1) # pongo el numero en data
    daddi $t2,$0,2
    sd $t2,0($s0) #control == 2

    ld $ra, 0($sp) 
    daddi $sp,$sp,8

    jr $ra

ES_IMPAR: 

    andi $v0,$a2,1

    jr $ra










