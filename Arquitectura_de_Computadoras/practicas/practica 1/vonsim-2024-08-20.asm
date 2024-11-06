        org 1000h
        
MENSAJE db "Hola,Buenas Tardes"
FIN     db ?
cant    db 0

        org 2000h
BUCLE:  cmp bx,offset fin
        jz finp
        mov bx, offset mensaje
        cmp byte ptr[bx], 'a'
        inc bx
        jnz bucle
        inc cant
        jmp bucle
finp:   hlt
        end