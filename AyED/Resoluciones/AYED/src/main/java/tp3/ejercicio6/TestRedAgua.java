package main.java.tp3.ejercicio6;

import main.java.tp3.ejercicio1.GeneralTree;

public class TestRedAgua {
    public static void main(String[] args) {
        GeneralTree<Character> A = new GeneralTree<>('A');
        GeneralTree<Character> B = new GeneralTree<>('B');
        GeneralTree<Character> C = new GeneralTree<>('C');
        GeneralTree<Character> D = new GeneralTree<>('D');
        GeneralTree<Character> E = new GeneralTree<>('E');
        GeneralTree<Character> F = new GeneralTree<>('F');
        GeneralTree<Character> G = new GeneralTree<>('G');
        GeneralTree<Character> H = new GeneralTree<>('H');
        GeneralTree<Character> I = new GeneralTree<>('I');
        GeneralTree<Character> J = new GeneralTree<>('J');
        GeneralTree<Character> K = new GeneralTree<>('K');
        GeneralTree<Character> L = new GeneralTree<>('L');
        GeneralTree<Character> M = new GeneralTree<>('M');
        GeneralTree<Character> N = new GeneralTree<>('N');
        GeneralTree<Character> P = new GeneralTree<>('P');

        // Armar estructura de caños según la imagen
        A.addChild(B);
        A.addChild(C);
        A.addChild(D);
        A.addChild(E);

        C.addChild(F);
        C.addChild(G);
        G.addChild(L);

        D.addChild(H);
        D.addChild(I);
        D.addChild(J);
        D.addChild(K);
        D.addChild(P);

        J.addChild(M);
        J.addChild(N);

        // Probar
        RedDeAguaPotable red = new RedDeAguaPotable();
        red.estructuraDeCanos = A;

        double resultado = red.minimoCaudal(1000);
        System.out.println("Caudal mínimo: " + resultado);
    }
}
