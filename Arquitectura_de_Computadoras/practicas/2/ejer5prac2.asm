HAND EQU 40H
HAND_ESTADO EQU 41H

ORG 1000H

MSJ DB "INGENIERIA E INFORMATICA"
FIN DB ?

ORG 2000H
        
    ;configuro el hand para hacer polling. (7FH = 0111 1111)
    
    IN AL,HAND_ESTADO
    AND AL,07FH
    OUT HAND_ESTADO, AL

    MOV BX, OFFSET MSJ
    MOV CL, OFFSET FIN-OFFSET MSJ
    
    ;no imprimo hasta que este desocupado
    POLL: IN AL, HAND_ESTADO 
    AND AL, 1
    JNZ POLL

    ;envio el dato a imprimir, el strobe es automatico en hand
    MOV AL, [BX]
    OUT HAND, AL
    INC BX
    DEC CL
    JNZ POLL
    INT 0


end