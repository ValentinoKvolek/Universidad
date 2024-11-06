PA EQU 30H
PB EQU 31H
CA EQU 32H
CB EQU 33H

ORG 2000H
mov al, 0h
out cb, al
mov al,11000011b
out pb,al
int 0
END