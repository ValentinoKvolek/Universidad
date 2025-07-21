package main.java.tp3.ejercicio7;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp3.ejercicio1.GeneralTree;

import java.util.LinkedList;
import java.util.List;



public class Caminos {

    public GeneralTree<Integer> arbol;

    public List<Integer> caminoAHojaMasLejana (){


        List<Integer> camino = new LinkedList<>();  //me guardo el camino max

        List<Integer> maximo = new LinkedList<Integer>();


        if (arbol == null || arbol.isEmpty()) {
            return new LinkedList<>(); //lista vacia
        }

        caminoAHojaMasLejanaRec(arbol,camino, maximo);
        return maximo;

    }

    private void caminoAHojaMasLejanaRec (GeneralTree<Integer> nodo, List<Integer> camino ,List<Integer> maximo){


        if (nodo.isEmpty()){
            return;
        }

        camino.add(nodo.getData()); //agrego el nodo al camino.

        if(nodo.isLeaf()){

            if(camino.size() > maximo.size()){
                maximo.clear();
                maximo.addAll(camino);
            }
        }

        for(GeneralTree<Integer> hijo: nodo.getChildren()) {
            caminoAHojaMasLejanaRec(hijo, camino, maximo);
        }

        camino.remove(camino.size() - 1);  //se van eliminando los nodos que ya se procesaron sus hijos.
    }

}
