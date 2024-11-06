.data
    coorX: .byte 24 #coordenada X de un punto
    coorY: .byte 24 #coordenada Y de un punto
    color: .byte 255, 0, 255, 0 #color: máximo rojo + máximo azul => magenta
    CONTROL: .word 0x10000
    DATA: .word 0x10008
.code 

    ld $t0, CONTROL($zero)
    ld $t1, DATA($zero)

    daddi $t5,$zero,0
    daddi $t6,$zero,49


    daddi $t2, $zero, 8 # para leer un numero
    sd $t2,0($t0) #control == 8
    lbu $t2, 0($t1) #lo tomo el caracter en t2

    #guardo en variable

    sb $t2, coorX($zero)

    daddi $t2, $zero, 8 # para leer un numero
    sd $t2,0($t0) #control == 8
    lbu $t2, 0($t1) #lo tomo el caracter en t2

    #guardo en variable

    sb $t2, coorY($zero)




loop: lwu $t2, color($zero) #t2 = calor del color a pintar
    sw $t2, 0($t1) #data recibe valor del color a pintar
    lbu $t2, coorX($zero) #t2 = valor de coor x
    sb $t2, 5($t1) #data +  5  = recibe cord x
    lbu $t2,coorY($zero)#t2 = y
    sb $t2, 4($t1) #dara +  4 recibe coords de y

    daddi $t2, $zero, 5 # t2 = 5 funcion 5 :  salida grafica
    sd $t2, 0($t0)


    daddi $t5,$t5,1
    sb $t5, coorY($zero)
    daddi $t6,$t6,-1

    bnez $t6,loop


    halt



