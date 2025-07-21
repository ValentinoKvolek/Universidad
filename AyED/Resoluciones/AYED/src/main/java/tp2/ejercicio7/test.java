package main.java.tp2.ejercicio7;

import main.java.tp2.ejercicio1.BinaryTree;

public class test {

    public static void main(String[] args) {

        BinaryTree<Integer> V2 = new BinaryTree<>(2);

        BinaryTree<Integer> V7 = new BinaryTree<>(7);
        BinaryTree<Integer> V5 = new BinaryTree<>(-5);

        BinaryTree<Integer> V23 = new BinaryTree<>(23);
        BinaryTree<Integer> V6 = new BinaryTree<>(6);
        BinaryTree<Integer> V19 = new BinaryTree<>(19);

        BinaryTree<Integer> V3 = new BinaryTree<>(-3);
        BinaryTree<Integer> V55 = new BinaryTree<>(55);
        BinaryTree<Integer> V11 = new BinaryTree<>(11);
        BinaryTree<Integer> V4 = new BinaryTree<>(4);

        BinaryTree<Integer> V18 = new BinaryTree<>(18);

        V2.addLeftChild(V7);
        V2.addRightChild(V5);

        V7.addLeftChild(V23);
        V7.addRightChild(V6);

        V5.addLeftChild(V19);

        V23.addLeftChild(V3);

        V6.addLeftChild(V55);
        V6.addRightChild(V11);

        V19.addRightChild(V4);

        V4.addLeftChild(V18);

        System.out.println(V2);

        ParcialArboles parcialArboles = new ParcialArboles(V2);

        System.out.println("num = 7: " + parcialArboles.isLeftTree(7));
        System.out.println();
        System.out.println("num = 2: " + parcialArboles.isLeftTree(2));
        System.out.println();
        System.out.println("num = -5: " + parcialArboles.isLeftTree(-5));
        System.out.println();
        System.out.println("num = 19: " + parcialArboles.isLeftTree(19));
        System.out.println();
        System.out.println("num = -3: " + parcialArboles.isLeftTree(-3));
    }


}
