
.data

x: .space 10
y: .space 10

a: .byte 0
b: .byte 50

control:  .word 0x10000
data: .word 0x10008

.code 

daddi $sp, $0, 0x400
ld $s0,control($zero)
ld $s1,data($zero)
daddi $a0,$0,x #le paso la direccion de un solo vector
daddi $a1,$0,10 #cantidad de valores a leer
daddi $s2,$0,0 # longitud
jal leerCoordenadas


halt

#a0=direcc de vector x, a1= 10

leerCoordenadas: 
    daddi $s3,$a1,0
    daddi $t0,$0,8
    sd $t0,0($s0) # control =8 
    ld $t1,0($s1) # t1 me guardo el numero q ingrese
    # lei x
    

jr $ra