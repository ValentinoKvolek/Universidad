# Escribir la subrutina MIN_MAX que recibe la dirección de comienzo de una tabla y la cantidad de elementos,
# y devuelve el valor máximo y el mínimo de dicha tabla. 
#Usando la subrutina, implementar un programa que obtenga el min y el max de 2 tablas. 
#Por último, imprimir en la pantalla gráfica un punto de color Verde (0,255,0) en la coordenada (mínimo_tabla2, máximo_tabla1) y otro de color Azul (0, 0, 255) en la coordenada (mínimo_tabla1, máximo_tabla2).
# Usar la convención para nombrar a los registros

.data

tabla: .byte 1,5,10,2,5
tabla2: .byte 1,2,11,3,5


vectoryoni: .byte 3,4,5,6,10,40,12

verde: .byte 0,255,0,0
azul: .byte 0,0,255,0

control: .word 0x10000
data: .word 0x10008

.code

ld $s0,control($0) #control en s0
ld $s1,data($0) #data en s1
daddi $a0,$0, tabla # le paso a a0 la direccion de tabla.
daddi $a1,$0,5 # pongo en a1 la longitud.

jal MIN_MAX 

sd $v0,0($s2) #guardo mi max1 en s2
sd $v1,0($s3) #guardo mi min1 en s3

# reseteo la tabla:
daddi $a0,$0, tabla2 # le paso a a0 la direccion de tabla.
daddi $a1,$0,5 # pongo en a1 la longitud.

jal MIN_MAX 

sd $v0,0($s4) #guardo mi max2 en s4
sd $v1,0($s5) #guardo mi min2 en s5

#Imprimir1
lwu $t0,verde($0) 
sb $s5,4($s1) #guardo Y (minimo tabla 2) en data
sb $s2,5($s1) #guardo X (maximo tabla 1) en data
sw $t0,0($s1) #guardo color en data
daddi $t0,$0,5  
sd $t0,0($s0) # para pintar el pixel

#Imprimir2
lwu $t0,azul($0) 
sb $s3,4($s1) #guardo y (minimo tabla 1) en data
sb $s4,5($s1) #guardo x (maximo tabla 2) en data
sw $t0,0($s1) #guardo color en data
daddi $t0,$0,5  
sd $t0,0($s0) # para pintar el pixel

halt

#a0 = tabla, a1 = longitud de tabla.

MIN_MAX: 

    daddi $v0,$0,-99 # v0 = Max
    daddi $v1,$0,99 # v1 = Min
    daddi $t0,$0,0 # numero temporal.

lazo:
    beqz $a1, fin
    lb $t0,0($a0) #cargo el numero
    slt $t1,$t0,$v0 #comparo el max
    bnez $t1, saveMax 
    slt $t1,$v0,$t0 #comparo el min
    bnez $t1, saveMin
seguir:
    daddi $a0,$a0,1  #aumento a0 para recorrer mi tabla
    daddi $a1,$a1,-1 #descuento para el corte
    j lazo

saveMax: 
    daddi $v0,$0,$t0 #guardo Max.
j seguir

saveMin:
    daddi $v1,$0,$t0 #guardo Min.
j seguir

fin: 
    jr $ra





