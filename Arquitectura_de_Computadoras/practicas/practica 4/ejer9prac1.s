.data

string: .asciiz "ArquiTectuRa de ComPutaDoras"

longitud:  .byte 0

.code

daddi $t0,$zero,0 ;para moverme

ld $t3,longitud($zero) ;para longitud

loop: 

        lbu $t1,string($t0) ;agarro un ascii sin que me importe el signo 

        daddi $t3,$t3,1 ;cuanto un char

        daddi $t0,$t0,1 ; memuevo de a 1 por que cada char ocupa un byte

        bnez $t1, loop


        sd $t3,longitud($zero)

    

halt