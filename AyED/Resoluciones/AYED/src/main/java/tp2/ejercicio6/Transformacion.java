package main.java.tp2.ejercicio6;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp2.ejercicio1.BinaryTree;

import java.util.LinkedList;

public class Transformacion {

    BinaryTree<Integer> arbol = new BinaryTree<>();

    public BinaryTree<Integer> suma() {
        transformar(arbol);
        return arbol;
    }

    private int transformar(BinaryTree<Integer> nodo) {

        if (nodo == null || nodo.isEmpty()) {
            return 0;
        }

        int valorOriginal = nodo.getData(); // para no perderlo
        int sumaIzq = 0;
        int sumaDer =0;
        // si es una hoja la suma de sus hijos va a ser 0 pro eso no uso is leaf

        if(nodo.hasLeftChild()) {
            sumaIzq = transformar(nodo.getLeftChild());
        }
        if(nodo.hasRightChild()) {
            sumaDer = transformar(nodo.getRightChild());
        }
        nodo.setData(sumaIzq + sumaDer);

        return valorOriginal + sumaIzq + sumaDer; // retornamos la suma total del subárbol
    }
}