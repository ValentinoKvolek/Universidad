package main.java.parciales1.p5;

import main.java.tp3.ejercicio1.GeneralTree;

import java.util.List;

public class Parcial {

    public static boolean esDeSeleccion(GeneralTree<Integer> arbol){

        boolean encontre = false;
        if(arbol != null) {

            if (!arbol.isEmpty()) {

                encontre = resolver(arbol);

            }
        }
        return encontre;
    }


    private static boolean resolver(GeneralTree<Integer> nodo){

        if (nodo.isLeaf()){
            return true;
        }

        List<GeneralTree<Integer>> hijos = nodo.getChildren();

        int min = Integer.MAX_VALUE;

        for(GeneralTree<Integer> hijo : hijos){

            if(hijo.getData() < min){
                min = hijo.getData();
            }

            if(!resolver(hijo)){
                return false;
            }

        }

        return (min == nodo.getData());
    }

    public static void main(String[] args) {

        GeneralTree<Integer> tree = new GeneralTree<>(12);

        GeneralTree<Integer> tree1 = new GeneralTree<>(12);
        GeneralTree<Integer> tree2 = new GeneralTree<>(25);

        GeneralTree<Integer> tree11 = new GeneralTree<>(35);
        GeneralTree<Integer> tree12 = new GeneralTree<>(12);

        GeneralTree<Integer> tree111 = new GeneralTree<>(35);

        GeneralTree<Integer> tree121 = new GeneralTree<>(12);
        GeneralTree<Integer> tree122 = new GeneralTree<>(35);
        GeneralTree<Integer> tree123 = new GeneralTree<>(23);

        GeneralTree<Integer> tree21 = new GeneralTree<>(25);

        GeneralTree<Integer> tree211 = new GeneralTree<>(25);

        tree.addChild(tree1);
        tree.addChild(tree2);

        tree1.addChild(tree11);
        tree1.addChild(tree12);

        tree2.addChild(tree21);

        tree11.addChild(tree111);

        tree12.addChild(tree121);
        tree12.addChild(tree122);
        tree12.addChild(tree123);

        tree21.addChild(tree211);

        System.out.println(esDeSeleccion(tree));
    }
}
