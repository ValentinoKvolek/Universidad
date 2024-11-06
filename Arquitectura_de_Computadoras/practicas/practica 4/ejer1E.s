.data
n: .byte 5
b: .byte 0

.code

    lb r2, n($zero)  #cargo el valor de n en r2
    andi r3, r2, 1   # un and para saber si el ultimo bit de n es 1 (impar ) o (par)

    bnez r3, esImpar      # si r3 no es 0 entonces es impar

    daddi r3, r0, 0       # si no es impar

    sb r3, b($zero)  

    j end 

esImpar:
    daddi r3, r0, 1      
    sb r3, b($zero)  

end:
    halt               
