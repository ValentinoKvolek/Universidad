CONT EQU 10H
COMP EQU 11H
EOI EQU 20H
IMR EQU 21H
IRR EQU 22H
ISR EQU 23H
INT0 EQU 24H
INT1 EQU 25H
INT2 EQU 26h


F10 EQU 10
CLK EQU 15

ORG 40
IP_F10 DW F10_RUT ;IDE  EN EL VECTOR DE POSICIONES

ORG 60
IP_CLK DW RUT_CLK ;IDE  EN EL VECTOR DE POSICIONES


ORG 1000H
CONTADOR DB 39H
FINAL DB 0
          
ORG 2000H
CALL CONTAR

CLI 
  MOV AL,11111110B ;CONFIGURO IMR PARA QUE LE DE BOLA AL  F10
  OUT IMR,AL
  MOV AL,F10
  OUT INT0,AL
  MOV AL,CLK
  OUT INT1,AL
STI

BUCLE:CMP FINAL,0
JZ BUCLE


INT 0


ORG 3000H
RUT_CLK: MOV BX,OFFSET CONTADOR
        MOV AL,1
        INT 7
        CMP CONTADOR, 30H
        JNZ SIGUE
        MOV AL,11111111B;me enoje no le doy bola a nadie.
        OUT IMR,AL
        MOV FINAL, 1

        
SIGUE:DEC CONTADOR;decremento el contador
      MOV AL,0
      OUT CONT,AL
      MOV AL,EOI
      OUT EOI,AL
      
      IRET
        
CONTAR: MOV BX, OFFSET CONTADOR
REPITE: INT 6
        MOV AL,[BX]
        CMP AL,30H ;VERIFICO SI ES UN NUMERO ENTRE 1..9
        JS REPITE
        CMP AL,39H ;VERIFICO SI ES UN NUMERO ENTRE 1..9
        JNS REPITE
        RET
        
        
ORG 3200H

F10_RUT: MOV AL,11111101B ;LE DOY BOLA AL CLOCK
          OUT IMR,AL
          MOV AL,0
          
          OUT CONT,AL;PONGO EL CONTADOR  EN 0
          MOV AL,1
          OUT COMP,AL ;PONGO EL COMPARADOR EN 1
          
          ;RESETEO EL EOI
          MOV AL,EOI
          OUT EOI,AL
          IRET   

END
