package main.java.tp2.ejercicio1;

public class Test {

    public static void main(String[] args) {
        BinaryTree<String> A = new BinaryTree<>("A");
        BinaryTree<String> B = new BinaryTree<>("B");
        BinaryTree<String> C = new BinaryTree<>("C");

        A.addLeftChild(B);
        A.addRightChild(C);

        BinaryTree<String> D = new BinaryTree<>("D");

        B.addLeftChild(D);

        BinaryTree<String> E = new BinaryTree<>("E");
        BinaryTree<String> F = new BinaryTree<>("F");

        C.addLeftChild(E);
        C.addRightChild(F);

        BinaryTree<String> G = new BinaryTree<>("G");

        F.addRightChild(G);



        A.entreNiveles(0, 3);

        System.out.println("========================");

        A = A.espejo();

        A.entreNiveles(0, 3);

    }
}
