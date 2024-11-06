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
mov al, 0ffh
out CA, al
mov al, 0h
out CB, al
bucle: in al, pA
out PB,al
jmp bucle
END