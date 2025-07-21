package main.java.tp3.ejercicio4;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp3.ejercicio1.GeneralTree;

public class AnalizadorArbol {



    public double devolverMaximoPromedio (GeneralTree<AreaEmpresa> arbol){

        //general una cola del tipo de generaltree que uso:
        Queue<GeneralTree<AreaEmpresa>> queue = new Queue<GeneralTree<AreaEmpresa>>();

        double promMax = Double.MIN_VALUE;

        queue.enqueue(arbol); //pongo la raiz

        while(!queue.isEmpty()){

            double sumaTotal = 0;
            int cantNodo = queue.size(); //pongo la cantidad de nodos que tieene mi nivel act

            for (int i = 0; i<cantNodo; i++){
                //por cada nodo de este nivel recorro
                GeneralTree<AreaEmpresa> aux = queue.dequeue(); //desencolo;
                sumaTotal += aux.getData().latencia;

                //encolo todo los loos hijos de los nodos de este nivel
                for (GeneralTree<AreaEmpresa> hijo: aux.getChildren()){
                    if(hijo != null && !hijo.isEmpty()){
                        queue.enqueue(hijo);
                    }
                }
            }

            //cuando salgo de el nivel.
            if((sumaTotal/ cantNodo) > promMax){

                promMax = (sumaTotal/ cantNodo);

            }

        }

        return promMax;

    }

}
