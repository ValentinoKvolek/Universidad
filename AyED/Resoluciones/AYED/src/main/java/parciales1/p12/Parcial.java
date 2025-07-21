package main.java.parciales1.p12;

import main.java.tp2.ejercicio1.BinaryTree;

import java.util.ArrayList;
import java.util.List;

public class Parcial {

    public List<Integer> resolver(BinaryTree<Integer>ab){
        if(ab != null){
            if (!ab.isEmpty()){
                List<Integer> resultado = new ArrayList<>();
                resolver(ab,resultado);
                return resultado;
            }
        }

        return new ArrayList<>();
    }

    private int resolver(BinaryTree<Integer> nodo, List<Integer> resultado){
        if(nodo.isLeaf()){
            if(nodo.getData() != null){
                resultado.add(nodo.getData());
                return 1;
            }
        }
        int cantIzq =0;
        int cantDer =0;
        if(nodo.hasLeftChild()){
            cantIzq = resolver(nodo.getLeftChild(), resultado);
        }
        if(nodo.hasRightChild()){
            cantDer = resolver(nodo.getRightChild(), resultado);
        }
        if(cantIzq == cantDer){
            resultado.add(nodo.getData());
        }
        return cantDer+cantIzq+ 1; // el nodo ;
    }

    public static void main(String[] args) {
        BinaryTree<Integer> ab = new BinaryTree<Integer>(2);
        ab.addLeftChild(new BinaryTree<Integer>(1));
        ab.getLeftChild().addLeftChild(new BinaryTree<Integer>(16));
        ab.getLeftChild().addRightChild(new BinaryTree<Integer>(6));
        ab.addRightChild(new BinaryTree<Integer>(5));
        ab.getRightChild().addRightChild(new BinaryTree<Integer>(8));
        ab.getRightChild().getRightChild().addLeftChild(new BinaryTree<Integer>(22));

        Parcial p = new Parcial();

        System.out.println(p.resolver(ab));
    }

}
