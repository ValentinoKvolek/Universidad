package main.java.tp2.ejercicio7;

import main.java.tp2.ejercicio1.BinaryTree;

public class Test9{
    public static void main(String[] args) {
        // Raíz
        BinaryTree<Integer> raiz = new BinaryTree<>(20);

        // Subárbol izquierdo
        BinaryTree<Integer> nodo5 = new BinaryTree<>(5);
        BinaryTree<Integer> nodoNeg5 = new BinaryTree<>(-5);
        BinaryTree<Integer> nodo10 = new BinaryTree<>(10);
        BinaryTree<Integer> nodo1 = new BinaryTree<>(1);

        raiz.addLeftChild(nodo5);
        nodo5.addLeftChild(nodoNeg5);
        nodo5.addRightChild(nodo10);
        nodo10.addLeftChild(nodo1);

        // Subárbol derecho
        BinaryTree<Integer> nodo30 = new BinaryTree<>(30);
        BinaryTree<Integer> nodo50 = new BinaryTree<>(50);
        BinaryTree<Integer> nodoNeg9 = new BinaryTree<>(-9);
        BinaryTree<Integer> nodo4 = new BinaryTree<>(4);
        BinaryTree<Integer> nodo6 = new BinaryTree<>(6);

        raiz.addRightChild(nodo30);
        nodo30.addLeftChild(nodo50);
        nodo30.addRightChild(nodoNeg9);
        nodo50.addRightChild(nodo4);
        nodo4.addRightChild(nodo6);

        // Imprimir el árbol
        System.out.println("Árbol del ejemplo: " + raiz);

        ParcialArboles prueba = new ParcialArboles();
        System.out.println(prueba.sumAndDif(raiz));
    }
}