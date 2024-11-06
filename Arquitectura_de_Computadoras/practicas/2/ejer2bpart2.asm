PA EQU 30H
CA EQU 32H
ORG 1000H
no db "Libre"

ORG 2000H
mov al, 11111101b ;configuro busy y strobe
out CA, al

poll: in al,PA ;voy preguntando si esta libre
in al,1
jnz poll

;imprimo que esta libre
mov bx, offset no
mov al,5
int 7
int 0



END