package main.java.tp2.ejercicio1;

public class ejer4 {
    public static class BinaryTreeTest {

        public static void main(String[] args) {

            BinaryTree<Integer> A = new BinaryTree<>(10);
            BinaryTree<Integer> B = new BinaryTree<>(2);
            BinaryTree<Integer> C = new BinaryTree<>(3);

            A.addLeftChild(B);
            A.addRightChild(C);

            BinaryTree<Integer> D = new BinaryTree<>(5);
            BinaryTree<Integer> E = new BinaryTree<>(4);

            B.addLeftChild(D);
            B.addRightChild(E);

            BinaryTree<Integer> F = new BinaryTree<>(9);
            BinaryTree<Integer> G = new BinaryTree<>(8);

            C.addLeftChild(F);
            C.addRightChild(G);

            D.addLeftChild(new BinaryTree<>(7));
            D.addRightChild(new BinaryTree<>(8));

            E.addLeftChild(new BinaryTree<>(5));
            E.addRightChild(new BinaryTree<>(6));

            F.addLeftChild(new BinaryTree<>(12));
            F.addRightChild(new BinaryTree<>(8));

            G.addLeftChild(new BinaryTree<>(2));
            G.addRightChild(new BinaryTree<>(1));

            RedBinariaLlena prueba = new RedBinariaLlena(A);

        }


    }


}
