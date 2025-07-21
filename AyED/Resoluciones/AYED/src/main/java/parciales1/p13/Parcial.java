package main.java.parciales1.p13;

import main.java.tp2.ejercicio1.BinaryTree;
import main.java.tp3.ejercicio1.GeneralTree;
import org.ietf.jgss.GSSName;

import java.util.LinkedList;
import java.util.List;

public class Parcial {


    public int resolver(GeneralTree<Integer> ab){
        if(ab != null){
            if(!ab.isEmpty()){
                int max = Integer.MIN_VALUE;
                max= resolver(ab, max);
                return max;
            }
        }
        return 0;
    }

    private int resolver(GeneralTree<Integer> nodo, int max){

        if(nodo.getData() !=null){
            if(nodo.getData() > max){
                max=nodo.getData();
            }
        }

        List<GeneralTree<Integer>> hijos = nodo.getChildren();
        for(GeneralTree<Integer> hijo: hijos){
            int aux;
            aux= resolver(hijo, max);
            if(aux > max){
                max = aux;
            }
        }

        return max;
    }

    public static void main(String[] args) {
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

        Parcial r = new Parcial();
        System.out.println(r.resolver(a));
    }
}
