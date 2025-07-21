package main.java.parciales1.p10;

import main.java.tp2.ejercicio1.BinaryTree;

public class Parcial {

    public Integer sumImparesPosMayorA(BinaryTree<Integer> ab, int limite){

        if(ab != null){
            if(!ab.isEmpty()){
                Integer suma =0;
                return suma = resolver(ab,limite,suma);
            }
        }

        return 0;
    }

    private Integer resolver(BinaryTree<Integer> nodo , int limite, int suma){


        if(nodo.hasLeftChild()){
            suma = resolver(nodo.getLeftChild(),limite,suma);
        }

        if(nodo.hasRightChild()){
            suma = resolver(nodo.getRightChild(), limite,suma);
        }

        if( nodo.getData() != null ) {
            if (nodo.getData() % 2 != 0 && nodo.getData() > limite) {
                suma += nodo.getData();
            }
        }

        return suma;

    }
    public static void main(String args[]) {
        BinaryTree<Integer> ab = new BinaryTree<Integer>(7);
        ab.addLeftChild(new BinaryTree<Integer>(56));
        ab.getLeftChild().addLeftChild(new BinaryTree<Integer>(38));
        ab.getLeftChild().addRightChild(new BinaryTree<Integer>(31));
        ab.getLeftChild().getRightChild().addRightChild(new BinaryTree<Integer>(94));
        ab.getLeftChild().getRightChild().getRightChild().addRightChild(new BinaryTree<Integer>(2));
        ab.getLeftChild().getRightChild().getRightChild().getRightChild().addLeftChild(new BinaryTree<Integer>(1));
        ab.getLeftChild().getLeftChild().addLeftChild(new BinaryTree<Integer>(87));
        ab.getLeftChild().getLeftChild().addRightChild(new BinaryTree<Integer>(77));
        ab.getLeftChild().getLeftChild().getRightChild().addLeftChild(new BinaryTree<Integer>(16));
        ab.getLeftChild().getLeftChild().getRightChild().getLeftChild().addRightChild(new BinaryTree<Integer>(43));
        ab.getLeftChild().getLeftChild().getRightChild().getLeftChild().getRightChild().addLeftChild(new BinaryTree<Integer>(9));
        ab.getLeftChild().getLeftChild().getRightChild().getLeftChild().getRightChild().addRightChild(new BinaryTree<Integer>(10));
        ab.addRightChild(new BinaryTree<Integer>(25));
        ab.getRightChild().addLeftChild(new BinaryTree<Integer>(5));
        ab.getRightChild().addRightChild(new BinaryTree<Integer>(6));

        Parcial p = new Parcial();

        System.out.println(p.sumImparesPosMayorA(ab, 30));

    }

}
