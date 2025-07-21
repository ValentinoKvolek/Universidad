package main.java.tp3.ejercicio10;

import main.java.tp3.ejercicio1.GeneralTree;
import org.ietf.jgss.GSSName;

import java.util.LinkedList;
import java.util.List;

public class ParcialArboles10 {


    public static List<Integer> resolver(GeneralTree<Integer> arbol){
        List<Integer> camMax = new LinkedList<>();
        List<Integer> camAct = new LinkedList<>();
        Maximo max = new Maximo(-1);
        int maxAct=  -1;
        int nivel = 0;

        resolverR(arbol,camMax,camAct, max, maxAct, nivel);
        return camMax;
    }

    public static void resolverR(GeneralTree<Integer> nodo,List<Integer> camMax, List<Integer> camAct, Maximo max, int maxAct, int nivel) {

        if(nodo.getData() == 1){
            camAct.add(nodo.getData());
            maxAct+= nodo.getData()* nivel;
        }

        if(!nodo.isLeaf()){
            nivel++;
            List<GeneralTree<Integer>> nodos = nodo.getChildren();
            for(GeneralTree<Integer> n : nodos){
                resolverR(n, camMax, camAct, max, maxAct, nivel);
            }
        }
        if(maxAct > max.getMax()){
            max.setMax(maxAct);
            camMax.removeAll(camMax);
            camMax.addAll(camAct);
            maxAct -= nodo.getData() * nivel; // Restar el peso del nodo actual
        }

        if(nodo.getData() == 1){
            camAct.removeLast();
        }
    }
}