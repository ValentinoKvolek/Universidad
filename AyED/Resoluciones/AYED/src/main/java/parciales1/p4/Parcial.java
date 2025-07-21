package main.java.parciales1.p4;

import main.java.tp2.ejercicio1.BinaryTree;

public class Parcial {

    BinaryTree<Integer> arbol;

    public Parcial(BinaryTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public boolean resolver(int k){

        boolean esMonodistante = false;

        if (!arbol.isEmpty() || arbol != null){

            esMonodistante = recorrer(k, arbol, 0);

        }

        return esMonodistante;

    }


    private boolean recorrer ( int k , BinaryTree<Integer> nodo, int suma){

        suma += nodo.getData();

        if (nodo.isLeaf()){

            if ( suma != k ) {
                return false;
            }

        }

        if(nodo.hasLeftChild()){
            if(!recorrer(k,nodo.getLeftChild(),suma)){
                return false;
            }
        }
        if (nodo.hasRightChild()){
            if (!recorrer(k,nodo.getRightChild(), suma)){
                return false;
            }
        }

        return true;
    }


    public static void main(String args[]) {

        BinaryTree<Integer> ab = new BinaryTree<Integer>(1);
        ab.addLeftChild(new BinaryTree<Integer>(3));
        ab.getLeftChild().addLeftChild(new BinaryTree<Integer>(3));
        //ab.getLeftChild().getLeftChild().addLeftChild(new BinaryTree<Integer>(1));
        ab.addRightChild(new BinaryTree<Integer>(4));
        ab.getRightChild().addLeftChild(new BinaryTree<Integer>(1));
        ab.getRightChild().getLeftChild().addLeftChild(new BinaryTree<Integer>(1));
        ab.getRightChild().addRightChild(new BinaryTree<Integer>(2));

        Parcial a = new Parcial(ab);
        System.out.println(a.resolver(7));

    }

}
