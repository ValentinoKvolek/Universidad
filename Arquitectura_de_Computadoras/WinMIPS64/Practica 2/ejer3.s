.code
daddi $sp, $0, 0x400
daddi $t0, $0, 5
daddi $t1, $0, 8

daddi $sp,$sp,-16 #me guardo dos lugares en la pila.

sd $t0,0($sp) #con este codigo me guardo t0 en la posicion exacta donde quedo mi sp
sd $t1,8($sp) #con este codigo me guardo t1 en la posidicon de sp + 8 por que ya tengo un dato

ld $t0, 0($sp) #con este codigo me traigo lo que tnego en el tope de la pila
ld $t1, 8($sp) # en este paso hago lo mismo nomas que con un desplazamiento de 8 para saltear el primer valor guardado

daddi $sp,$sp,16 #restaurto la pila

halt