package main.java.tp2.ejercicio1;

import main.java.tp1.ejercicio8.Queue;

public class RedBinariaLlena{


    private BinaryTree<Integer> tree;

    public RedBinariaLlena(BinaryTree<Integer> ab){

        this.tree = ab;

        int max =tree.getData();
        System.out.println(retardoReevio(tree,max));
    }

    public int retardoReevio(BinaryTree<Integer> tree, int max) {

        if (!tree.isEmpty()) {
            if (tree.hasRightChild() && tree.hasLeftChild()) {
                if (tree.getLeftChild().getData() > tree.getRightChild().getData()) {

                    max = max + tree.getLeftChild().getData();
                    return retardoReevio(tree.getLeftChild(), max);


                } else {
                    max = max + tree.getRightChild().getData();
                    return retardoReevio(tree.getRightChild(), max);
                }
            }
        }
        return max;
    }
}
