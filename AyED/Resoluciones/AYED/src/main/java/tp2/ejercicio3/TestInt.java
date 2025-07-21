package main.java.tp2.ejercicio3;

import main.java.tp2.ejercicio1.BinaryTree;
import main.java.tp2.ejercicio5.ProfundidadDeArbolBinario;

import main.java.tp2.ejercicio3.ContadorArbol;
import main.java.tp2.ejercicio5.ProfundidadDeArbolBinario;

public class TestInt{

    public static void main(String[] args) {
        BinaryTree<Integer> A = new BinaryTree<>(10);
        BinaryTree<Integer> B = new BinaryTree<>(10);
        BinaryTree<Integer> C = new BinaryTree<>(10);

        A.addLeftChild(B);
        A.addRightChild(C);

        BinaryTree<Integer> D = new BinaryTree<>(10);
        BinaryTree<Integer> E = new BinaryTree<>(10);

        B.addLeftChild(D);
        B.addRightChild(E);

        ContadorArbol cont = new ContadorArbol(A);

        System.out.println("numeros sumados por nivel de profundida:");

        System.out.println("========================");

        ProfundidadDeArbolBinario pro = new ProfundidadDeArbolBinario();
        System.out.println(pro.sumaElementosProfundidad(A, 2));

        System.out.println("========================");


        System.out.println(cont.numerosPares());
        System.out.println("========================");
        System.out.println(cont.numerosPares2());

    }
}
