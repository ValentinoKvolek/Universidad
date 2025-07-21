package main.java.parciales1.p11;

import main.java.tp2.ejercicio1.BinaryTree;
import main.java.tp3.ejercicio1.GeneralTree;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Parcial {


    public List<Integer> caminoMasLejano(GeneralTree<Integer> ab){

        if(ab !=null){
            if(!ab.isEmpty()){
                List<Integer> camino = new ArrayList<Integer>();
                resolver(ab, camino, new ArrayList<Integer>());
                return camino;
            }
        }
        return new ArrayList<Integer>();

    }


    private void resolver(GeneralTree<Integer>nodo, List<Integer>camino, List<Integer> caminoAct){

        if(nodo.getData() != null){
            caminoAct.add(nodo.getData());
        }

        if(nodo.isLeaf()){
            if(caminoAct.size() > camino.size()){
                camino.clear();
                camino.addAll(caminoAct);
            }
        }

        List<GeneralTree<Integer>> hijos  = nodo.getChildren();
        for (GeneralTree<Integer> hijo: hijos){
            resolver(hijo, camino,caminoAct);
        }

        caminoAct.remove(caminoAct.size()-1);
    }

    public static void main(String main[]) {
        List<GeneralTree<Integer>> subChildren1 = new LinkedList<GeneralTree<Integer>>();
        subChildren1.add(new GeneralTree<Integer>(2));
        GeneralTree<Integer> subAb1 = new GeneralTree<Integer>(7, subChildren1);
        List<GeneralTree<Integer>> subChildren2 = new LinkedList<GeneralTree<Integer>>();
        subChildren2.add(new GeneralTree<Integer>(4));
        subChildren2.add(subAb1);
        subChildren2.add(new GeneralTree<Integer>(6));
        GeneralTree<Integer> a1 = new GeneralTree<Integer>(3, subChildren2);

        List<GeneralTree<Integer>> subChildren3 = new LinkedList<GeneralTree<Integer>>();
        subChildren3.add(new GeneralTree<Integer>(1));
        subChildren3.add(new GeneralTree<Integer>(9));
        subChildren3.add(new GeneralTree<Integer>(10));
        GeneralTree<Integer> a2 = new GeneralTree<Integer>(5, subChildren3);

        List<GeneralTree<Integer>> arbol = new LinkedList<GeneralTree<Integer>>();
        arbol.add(a1);
        arbol.add(a2);
        GeneralTree<Integer> a = new GeneralTree<Integer>(8, arbol);

        Parcial cam = new Parcial();
        System.out.println(cam.caminoMasLejano(a));
    }
}
