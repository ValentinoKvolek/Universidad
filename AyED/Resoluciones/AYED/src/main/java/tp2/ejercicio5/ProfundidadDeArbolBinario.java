package main.java.tp2.ejercicio5;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp2.ejercicio1.BinaryTree;

public class ProfundidadDeArbolBinario {

    BinaryTree<Integer> arbol = new BinaryTree<>();


    public int sumaElementosProfundidad(BinaryTree<Integer> arbol, int profundidad) {

        int aux=0;
        int suma= 0;

        BinaryTree<Integer> ab = null;
        Queue<BinaryTree<Integer>> cola = new Queue<BinaryTree<Integer>>();
        cola.enqueue(arbol);

        cola.enqueue(null);
        while (!cola.isEmpty()) {

            ab = cola.dequeue();

            if(aux >= profundidad) {

                if(ab != null) {
                    suma += ab.getData();
                }
            }

            if (ab != null) {
                if (ab.hasLeftChild()) {
                    cola.enqueue(ab.getLeftChild());
                }
                if (ab.hasRightChild()) {
                    cola.enqueue(ab.getRightChild());
                }
            } else if (!cola.isEmpty()) {
                aux +=1; //sumo el contador de nivelesS
                cola.enqueue(null);  // aca le manda un null para saber si cambio de nivel
            }
        }
        return suma;
    }
}
