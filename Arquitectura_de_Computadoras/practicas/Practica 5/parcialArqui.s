#Escribir un programa que use la información de 8 pares de números guardados en la variable pares,
# y dibuje en la pantalla gráfica 8 puntos negros de acuerdo a la diferencia en valor absoluto de los valores de cada par.
#La variable pares contiene todos los valores de los pares, en la primera y segunda posición se guarda el primer par,
# en posición 3 y el segundo, y así sucesivamente.
#El procesamiento de los pares lo debe hacer la subrutina GRAFICAR que recibe por referencia el arreglo de pares y su cantidad. 
#Para cada par de números leídos calcula la diferencia en valor absoluto y 
#dibuja en la pantalla gráfica del simulador un punto negro en la posición (diferencia, diferencia). 
#El cálculo de la diferencia se realiza en una subrutina que recibe los dos números del par y devuelve la diferencia en valor absoluto. 
#Recuerde que la diferencia en valor absoluto se obtiene restando el número mayor al menor.

.data

CONTROL: .word 0x10000

DATA: .word 0x10008

negro: .byte 255,255,255,0

pares: .word 1,15,5,25,20,25,5,20,35,40,49,3,13,5,17,4

.code

ld $s0,CONTROL($0) #lo que esta en s0 lo cargo en control.
ld $s1,DATA($0) #lo que esta en s1 lo cargo en data.
daddi $sp,$0,0x400 #inicializo la pila
daddi $a0,$0,pares # coloco en a0 la direccion de pares
daddi $a1,$0, 16 #a1 tiene la cantidad.

jal GRAFICAR

halt

#a0 = direc pares,  #a1 = cant
GRAFICAR: 

    #como voy a irme a mi subrutina que calcula diferencia me tengo que guarde mi direc. de retorno del programa principal.
    daddi $sp, $sp , -8 # hago espacio.
    sd $ra,0($sp)  #guardo la direccion de retorno.

bucle: beqz $a1,finBucle

    ld $a2,0($a0) #cargo X1 en a2
    daddi $a0,$a0,1
    ld $a3,0($a0) #cargo X2 en a3
    daddi $a0,$a0,1

    jal DIFERENCIA
    ld $s2,0($v0) #s2 = X

    ld $a2,0($a0) #cargo y1 en a2
    daddi $a0,$a0,1
    ld $a3,0($a0) #cargo y2 en a3
    daddi $a0,$a0,1

    jal DIFERENCIA
    ld $s3,0($v0) #s3 = Y

    lwu $t0,negro($0) #guardo color
    sd $t0, 0($s1)  #guardo en data el color
    sd $s2,5($s1) #guardo en data el X
    sd $s3,4($s1) #guardo en data el Y
    daddi $t0,$0,5
    sd $t0,0($s0) #Imprimo.

    daddi $a1,$a1, -4

    j bucle

finBucle: 

    #dejo la pila como estaba 
    ld $ra,0($sp)
    daddi $sp, $sp , 8
    jr $ra


# recibe a2= num1 y en a3 = num2 
DIFERENCIA: 

    slt $t0, $a3,$a2 #si mi num 1 es mayor que el num 2 en t0 queda el 1
    beqz $t0,resta2 #si esto no es cierto resta 2
    # si no hago la resta 1: 
    dsub $t1,$a2,$a3 # NUM1 - NUM2
    j fin

resta2: dsub $t1,$a3,$a2 # NUM2 - NUM1
fin: lbu $v0,0($t1) #resultado sin signo (valor absoluto)

    jr $ra
    






    
