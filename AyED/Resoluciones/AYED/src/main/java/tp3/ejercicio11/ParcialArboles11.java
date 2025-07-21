package main.java.tp3.ejercicio11;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp3.ejercicio1.GeneralTree;

import java.util.List;

public class ParcialArboles11 {



    public static boolean resolver(GeneralTree<Integer> arbol){


        int cantNodos =0;

        int cantNodosAnt =1;  //raiz

        Queue<GeneralTree<Integer>> queue = new Queue<>();

        queue.enqueue(arbol);

        queue.enqueue(null);

        while(!queue.isEmpty()){

           GeneralTree<Integer> aux = queue.dequeue();

            if(aux != null && aux.getData() != null) {

               List<GeneralTree<Integer>> hijos = aux.getChildren();

               for (GeneralTree<Integer> hijo : hijos) {

                   cantNodos++;  //cuanto cant de nodos en ese nivel.

                   if (hijo != null && !hijo.isEmpty()) {
                       queue.enqueue(hijo);   //por cada hijo encolo.
                   }

               }
           }
           else {
               if (!queue.isEmpty()){

                   if((cantNodosAnt +1) != cantNodos){
                       return false;
                   }
                    cantNodosAnt = cantNodos;
                   cantNodos =0;
                   queue.enqueue(null); //marca de nivel

               }

           }

        }

        return true;

    }




}
