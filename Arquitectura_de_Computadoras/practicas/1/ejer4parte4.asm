org 1000h

string db "nhee"
fin db ?

org 2000h

  mov cl , 0
  mov dl,0
  mov bx , offset string
  mov al, offset fin - offset string
  mov ah, 'e'
  
 bucle: call CONTAR_CAR
      inc bx
      inc dl
      cmp al,dl
      jz termino
      jnz bucle


  
  
org 3000h

CONTAR_CAR: cmp byte ptr[bx], ah
            jz contar
            ret

            
            contar: inc cl
            ret



termino: hlt
 end