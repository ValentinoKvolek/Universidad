package main.java.tp2.ejercicio7;

import main.java.tp2.ejercicio1.BinaryTree;

public class test8_2 {
    public static void main(String[] args) {


        // Crear arbol1
        BinaryTree<Integer> arbol1 = new BinaryTree<>(65);

        BinaryTree<Integer> arbol1_37 = new BinaryTree<>(37);
        BinaryTree<Integer> arbol1_81 = new BinaryTree<>(81);

        arbol1.addLeftChild(arbol1_37);
        arbol1.addRightChild(arbol1_81);

        // Crear arbol2
        BinaryTree<Integer> arbol2 = new BinaryTree<>(65);

        BinaryTree<Integer> arbol2_37 = new BinaryTree<>(37);
        BinaryTree<Integer> arbol2_81 = new BinaryTree<>(81);
        BinaryTree<Integer> arbol2_22 = new BinaryTree<>(22);
        BinaryTree<Integer> arbol2_47 = new BinaryTree<>(47);
        BinaryTree<Integer> arbol2_76 = new BinaryTree<>(76);
        BinaryTree<Integer> arbol2_93 = new BinaryTree<>(93);

        arbol2.addLeftChild(arbol2_37);
        arbol2.addRightChild(arbol2_81);
        arbol2_37.addLeftChild(arbol2_22);
        arbol2_37.addRightChild(arbol2_47);
        arbol2_81.addLeftChild(arbol2_76);
        arbol2_81.addRightChild(arbol2_93);

        // Imprimir los árboles
        System.out.println("arbol1: " + arbol1);
        System.out.println("arbol2: " + arbol2);

        ParcialArboles prueba = new ParcialArboles();

        System.out.println(prueba.esPrefijo(arbol1,arbol2));


    }
}
