.data
n: .byte 8
l: .byte 0

.code

ld r1,l($zero) ;r1 es l
ld r4,n($zero) ; r4 pongo N
daddi r5,r0,2 ;r5 es 2


loop:
      ddivu r4,r4,r5 ; divido a n por 2

      daddi r1,r1,1 ;l=l+1

      bnez r4, loop ; n>0

      sd r1,l($zero)

halt