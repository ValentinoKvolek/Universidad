.data

vector: .word 5, 10 , 20, -10

color:  .byte 0,0,255,0

control: .word 0x10000
data: .word 0x10008


.code

ld $s0,control($0)
ld $s1,data($0)
daddi $sp,$0,0x400

daddi $a0,$0,vector # en a0 guardo la direccion de mi vector

jal imprimir


halt


#a0 = mi direc del vector
imprimir:
    
    daddi $s0,$0,0  # donde guardo mi x
    daddi $s1,$0,0  # donde guardo mi y
    daddi $t1,$0,4 # esta mi condicion de corte

    daddi $sp,$sp, -8
    sd $ra,0($sp)

bucle: beqz $t1,fin

        ld $a2,0($a0)  # donde guardo mi x
        daddi $a0,$a0,8
        ld $a3,0($a0)  # donde guardo mi y
        daddi $a0,$a0,8

        daddi $t1,$t1,-2

        jal ENTRE

        beqz $v0, seguir
        beqz $v1, seguir

        jal IMPRIMIR

        j bucle

    seguir:

        j bucle
    
fin: 

    ld $ra,0($sp)
    daddi $sp,$sp,8

    jr $ra

#a2 = X a3 = Y
ENTRE: 
    # comparacion de x
    daddi $t0,$0,-1
    daddi $t1,$0,50

    slt $t3,$t0,$a2
    beqz $t3, fin
    slt $t3,$a2,$t1
    beqz $t3, fin

    fin: $v0,$0,$t3 #v0 va a volver al programa con un 1 si me sirve el valor de X, si no con un 0
    
    # comparacion de y
    daddi $t0,$0,-1
    daddi $t1,$0,50

    slt $t3,$t0,$a3
    beqz $t3, fin2
    slt $t3,$a3,$t1
    beqz $t3, fin2

    fin2: $v1,$0,$t3 #v1 va a volver al programa con un 1 si me sirve el valor de Y, si no con un 0

    jr $ra

#a2 = X a3 = Y

IMPRIMIR: 

    lwu $t0,color($0) #t0 tiene mi color.

    sw $t0,0($s1) # guardo color en data
    sb $a2,5($s1) #guardio mi x
    sb $a3,4($s1) #guardio mi y
    daddi $t0,$0,5
    sd $t0,0($s0) #control en 5

    jr $ra








