PA EQU 30H
PB EQU 31H
CA EQU 32H
CB EQU 33H
ORG 1000H
MENSAJE DB "Llave prendida"
finmen db ?
MENSAJE2 DB "Llave apagada"
fin2 db ?
ORG 2000H

mov al, 80h
out CA, al


in al, PA
cmp al , 80H
jz men


mov bx, offset mensaje2
mov al, offset fin2 - offset mensaje2
int 7
int 0



men: mov bx, offset mensaje
      mov al, offset finmen - offset mensaje
      int 7
      int 0

END