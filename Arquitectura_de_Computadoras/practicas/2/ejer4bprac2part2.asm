PA EQU 30H
PB EQU 31H
CA EQU 32H
CB EQU 33H

ORG 1000H
mensaje db "ORGANIZACION Y ARQUITECTURA DE COMPUTADORAS"
fin db ?

ORG 2000H

        ;configuracion del pio: 
        
         mov al, 11111101b ;configuro busy y strobe
         out CA,al
         mov al, 0
         out CB, al ;pb salida.

          ;recorro el string:
          mov bx, offset mensaje
          
          ;voy preguntando si esta libre
    poll: in al,PA 
          and al,1
          jnz poll
          
          ;si esta libre: 
          mov al,[bx]
          out PB,AL
          
          ;señal de strobe en 1:
          
          in al,PA ;tomamos el estado
          or al,2;fuerzo el strobe a 1
          out PA,al
          
          ;señal de strobe en 0:
          in al,PA
          and al,0FDH;fuerzo el strobe en 0
          out PA,al   

          ;recorro el string:
          inc BX
          cmp bx,offset fin
          jnz poll
          int 0
END