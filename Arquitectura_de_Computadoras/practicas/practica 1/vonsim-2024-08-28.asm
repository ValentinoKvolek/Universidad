org 1000h

string db "nashee"
finc db ?

org 2000h

  mov cl , 0
  mov bx , offset string
  mov al, offset finc  - offset string
  mov ah, 'e'
  call CONTAR_CAR
  
org 3000h
CONTAR_CAR: cmp byte ptr[bx], ah
            jz contar
            jmp fin

contar: inc cl
fin: ret

end