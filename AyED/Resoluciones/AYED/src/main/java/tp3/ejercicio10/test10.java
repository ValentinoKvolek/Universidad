package main.java.tp3.ejercicio10;

import main.java.tp3.ejercicio1.GeneralTree;

public class test10 {
    public static void main(String[] args) {
        GeneralTree<Integer> tree = new GeneralTree<>(1);

        GeneralTree<Integer> tree_1 = new GeneralTree<>(0);
        GeneralTree<Integer> tree_2 = new GeneralTree<>(1);
        GeneralTree<Integer> tree_3 = new GeneralTree<>(1);

        tree.addChild(tree_1);
        tree.addChild(tree_2);
        tree.addChild(tree_3);

        GeneralTree<Integer> tree_1_1 = new GeneralTree<>(1);
        GeneralTree<Integer> tree_1_2 = new GeneralTree<>(1);

        tree_1.addChild(tree_1_1);
        tree_1.addChild(tree_1_2);

        GeneralTree<Integer> tree_1_1_1 = new GeneralTree<>(1);
        GeneralTree<Integer> tree_1_1_2 = new GeneralTree<>(1);
        GeneralTree<Integer> tree_1_1_3 = new GeneralTree<>(1);

        tree_1_1.addChild(tree_1_1_1);
        tree_1_1.addChild(tree_1_1_2);
        tree_1_1.addChild(tree_1_1_3);

        //

        GeneralTree<Integer> tree_2_1 = new GeneralTree<>(1);
        GeneralTree<Integer> tree_2_2 = new GeneralTree<>(0);

        tree_2.addChild(tree_2_1);
        tree_2.addChild(tree_2_2);

        GeneralTree<Integer> tree_2_2_1 = new GeneralTree<>(0);

        tree_2_2.addChild(tree_2_2_1);

        GeneralTree<Integer> tree_2_2_1_1 = new GeneralTree<>(1);

        tree_2_2_1.addChild(tree_2_2_1_1);

        //

        GeneralTree<Integer> tree_3_1 = new GeneralTree<>(0);

        tree_3.addChild(tree_3_1);

        GeneralTree<Integer> tree_3_1_1 = new GeneralTree<>(0);

        tree_3_1.addChild(tree_3_1_1);

        GeneralTree<Integer> tree_3_1_1_1 = new GeneralTree<>(0);
        GeneralTree<Integer> tree_3_1_1_2 = new GeneralTree<>(0);

        tree_3_1_1.addChild(tree_3_1_1_1);
        tree_3_1_1.addChild(tree_3_1_1_2);


    }

}
