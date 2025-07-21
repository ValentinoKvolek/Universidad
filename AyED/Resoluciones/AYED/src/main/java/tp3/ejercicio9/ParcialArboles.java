package main.java.tp3.ejercicio9;

import main.java.tp3.ejercicio1.GeneralTree;

import java.util.List;

public class ParcialArboles {


    public  boolean  esDeSeleccion(GeneralTree<Integer> arbol){

        //devuelve true si el árbol recibido por parámetro es de selección, falso sino lo es.

        return esDeSeleccionRec(arbol);

    }

    private boolean esDeSeleccionRec(GeneralTree<Integer> nodo){

            if(nodo.isEmpty()){
                return true;
            }

            int hijoMin=Integer.MAX_VALUE;

            if(!nodo.isLeaf()){

                List<GeneralTree<Integer>> hijos = nodo.getChildren();

                for (GeneralTree<Integer> hijo : hijos){

                    if(hijo.getData() < hijoMin){

                        hijoMin = hijo.getData();

                    }

                    if (!esDeSeleccionRec(hijo)){
                        return  false;
                    }
                }

                if(nodo.getData() != hijoMin){
                    return false;
                }
            }

            return true;
    }
}
