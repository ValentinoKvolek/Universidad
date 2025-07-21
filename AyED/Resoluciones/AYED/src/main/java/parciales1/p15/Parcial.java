package main.java.parciales1.p15;

import main.java.tp3.ejercicio1.GeneralTree;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Parcial {


    public List<GeneralTree<Integer>> resolver(GeneralTree<Integer> ab){
        if(ab != null){
            if(!ab.isEmpty()){
                List<GeneralTree<Integer>> nodos = new ArrayList<>();
                resolver(ab,nodos);
                return nodos;
            }
        }
        return new ArrayList<>();

    }

    private void resolver(GeneralTree<Integer> nodo, List<GeneralTree<Integer>> nodos) {

        if (nodo.isLeaf()) {
            return;
        }

        List<GeneralTree<Integer>> hijos = nodo.getChildren();

        if (hijos.size() >= 1) {
            resolver(hijos.getFirst(), nodos);
        }

        if (nodo.getData() != null) {
            if (hijos.size() % 2 == 0) {
                nodos.add(nodo);
            }
        }

        for (int i = 1 ; i < hijos.size(); i++) {
            GeneralTree<Integer> hijo = hijos.get(i);
            resolver(hijo, nodos);
        }
    }

    public static void main(String[] args) {
        // Subárbol 2 con hijos 5 y 6
        List<GeneralTree<Integer>> hijosDe2 = new LinkedList<>();
        hijosDe2.add(new GeneralTree<>(5));
        hijosDe2.add(new GeneralTree<>(6));
        GeneralTree<Integer> nodo2 = new GeneralTree<>(2, hijosDe2);

        // Subárbol 7 con hijo 9
        List<GeneralTree<Integer>> hijosDe7 = new LinkedList<>();
        hijosDe7.add(new GeneralTree<>(9));
        GeneralTree<Integer> nodo7 = new GeneralTree<>(7, hijosDe7);

        // Subárbol 3 con hijo 7
        List<GeneralTree<Integer>> hijosDe3 = new LinkedList<>();
        hijosDe3.add(nodo7);
        GeneralTree<Integer> nodo3 = new GeneralTree<>(3, hijosDe3);

        // Subárbol 4 con hijo 8
        List<GeneralTree<Integer>> hijosDe4 = new LinkedList<>();
        hijosDe4.add(new GeneralTree<>(8));
        GeneralTree<Integer> nodo4 = new GeneralTree<>(4, hijosDe4);

        // Raíz 1 con hijos 2, 3 y 4
        List<GeneralTree<Integer>> hijosDe1 = new LinkedList<>();
        hijosDe1.add(nodo2);
        hijosDe1.add(nodo3);
        hijosDe1.add(nodo4);
        GeneralTree<Integer> raiz = new GeneralTree<>(1, hijosDe1);

        // Test con el método resolver
        Parcial p = new Parcial();
        List<GeneralTree<Integer>> resultado = p.resolver(raiz);
        for (GeneralTree<Integer> sub : resultado) {
            System.out.print(sub.getData() + " ~ ");
        }
    }

}
