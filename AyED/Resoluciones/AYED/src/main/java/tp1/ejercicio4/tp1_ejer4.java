package main.java.tp1.ejercicio4;

public class tp1_ejer4 {

        public static void swap1 (int x, int y) {
            if (x < y) {
                int tmp = x ;
                x = y ;
                y = tmp;
            }
        }
        public static void swap2 (Integer x, Integer y) {
            if (x < y) {
                int tmp = x ;
                x = y ;
                y = (Integer) tmp;
            }
        }

    public static void main(String[] args) {
        int a = 1, b = 2;
        Integer c = (Integer) 3, d = (Integer) 4;
        swap1(a,b);
        swap2(c,d);
        System.out.println("a=" + a + " b=" + b) ;
        System.out.println("c=" + c + " d=" + d) ;
    }
}
