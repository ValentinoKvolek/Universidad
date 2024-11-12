.data

rojo: .byte 255,0,0,0

azul: .byte 0,0,255,0

DIR_CONTROL: .word 0x10000 # DIR_CONTROL tiene la DIRECCIÓN del registro CONTROL
DIR_DATA: .word 0x10008 # IDEM para DATA

.code


daddi $sp,$0,0x400 #pongo sp en el tope de la "pila"
ld $s0, DIR_CONTROL($zero) # Carga el valor 0x10000 en $s0
ld $s1, DIR_DATA($zero) # Carga el valor 0x10008 en $s1

daddi $a0,$0,0 # y

ld $a2, rojo($zero) 

ld $a3, azul($zero)

daddi $t5,$0,50

lazito: daddi $t5,$0,50
         beq $a0,$t5,fin2
jal fila_alternativa

jal alternar_color

daddi $a0,$a0,1

j lazito

fin2:

halt

fila_alternativa:  
                daddi $sp,$sp , -8

                sd $ra, 0($sp) # me guardo la direccion de retorno

                daddi $a1,$0,0

                daddi $t1,$0,50

        lazo:   daddi $t1,$0,50
        
                beq $a1,$t1,fin

                jal alternar_color

                jal pintar

                daddi $a1,$a1,1

                j lazo

        fin: 

                ld $ra, 0($sp)
                daddi $sp,$sp , 8

                jr $ra


#a0 = y , a1 = x , a2 = color
pintar: 

        sb $a0,4($s1) #guardo y en data

        sb $a1,5($s1) #guardo x en data

        sw $a2,0($s1) #guardo color en data

        daddi $t0,$0,5  

        sd $t0,0($s0) # para pintar el pixel

        jr $ra


#a2=rojo #a3=azul
alternar_color: 

    daddi $t0,$a2,0 # me guardo el color actual
    daddi $a2,$a3,0 #alterno el color en a2
    daddi $a3,$t0,0 #pongo colo anterio en a3 para volver al bucle

    jr $ra
    


        
