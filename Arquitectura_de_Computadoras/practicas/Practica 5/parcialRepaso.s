#Escribir un programa que lea desde teclado un número en punto flotante y lo compare con el valor almacenado en la celda Valor. 
#Si el valor ingresado por teclado es mayor que el almacenado en Valor, deberá calcular (X - Valor) * X, donde X es el número leído por teclado. 
#Caso contrario, deberá calcular (Valor - X) / X. Por último, deberá imprimir el texto "El resultado es:" junto con el valor calculado.

.data

valor: .byte 10

control: .word 0x10000
data: .word 0x10008

men: .asciiz "El resultado es:"

.code

daddi $s0,$0, control #guardo en s0 la direccion de control
daddi $s1,$0, data #guardo en s1 la direccion de data

jal ingresarNum

ld $a0,0($v0) # a0 = X

ld $a1,valor($0) #a1= valor

jal comparar

bnez $v0, calculo1

#calculo2

dsub $t0,$a1,$a0  #(Valor - X)
ddiv $t1,$t0,$a1 #(Valor - X) / X

j fin


calculo1:
     
     dsub $t0,$a0,$a1  #(X - Valor)
     dmul $t1,$t0,$a0 #(X - Valor) * X

     j fin


fin: 

    daddi $t2,$0,men
    sd $t2,0($s1) #guardo en data el mensanje
    daddi $t2,$0, 4  #no me acuerdo que num madale a control
    sd $t2,0($s0) # pero sea cual sea el num lo guarde en contro :v

    daddi $t2,$t1,0 # pongo en t2 el resultado
    sd $t2,0($s1) #guardo en data 
    daddi $t2,$0, 2 # para imprimir
    sd $t2,0($s0)

halt


ingresarNum: 

    daddi $t0,$0, 8 # guardo en t0 el valor 8
    sd $t0, 0($s0) #control ==  8 para pedir un numero
    ld $v0, 0($s1) #me guardo en v0 el numero ingresado

jr $ra

#a0 = x , $a1 = el numero que esta en VALOR.

comparar: 

    slt $v0,$a1,$a0 

    jr $ra




    