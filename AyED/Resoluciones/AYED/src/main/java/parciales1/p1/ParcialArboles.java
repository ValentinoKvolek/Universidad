package main.java.parciales1.p1;

import main.java.tp3.ejercicio1.GeneralTree;

import java.util.ArrayList;
import java.util.List;

public class ParcialArboles {

    GeneralTree<Integer> arbol;

    public ParcialArboles(GeneralTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public List<Integer>camino(Integer num){

        List<Integer> resultado = new ArrayList<Integer>();
        boolean encontre = false;

        if (arbol != null && !arbol.isEmpty()){
            encontre = resolver(arbol, resultado, new ArrayList<>(), num);
            if(encontre)
                return resultado;
        }
        return new ArrayList<>();
    }

    private boolean resolver(GeneralTree<Integer> nodo, List<Integer> resultado, List<Integer> caminoActual, Integer num){

        if (nodo.isLeaf()) {
            caminoActual.add(nodo.getData());
            resultado.addAll(caminoActual);
            return true;
        }

        List<GeneralTree<Integer>> hijos = nodo.getChildren();

        if (hijos.size() >= num){
            caminoActual.add(nodo.getData());
        }else{
            return false;
        }

        for (GeneralTree<Integer> hijo : hijos) {
            if (resolver(hijo, resultado, caminoActual, num)) {
                return true;
            }
        }
        
        caminoActual.remove(caminoActual.size()-1);
        return false;
    }



    public static void main(String args[]) {
        GeneralTree<Integer> arbol = new GeneralTree<Integer>(10);

        GeneralTree<Integer> subAb1 = new GeneralTree<Integer>(5);
        subAb1.addChild(new GeneralTree<Integer>(-6));
        GeneralTree<Integer> subAb2 = new GeneralTree<Integer>(22);
        subAb2.addChild(new GeneralTree<Integer>(28));
        subAb2.addChild(new GeneralTree<Integer>(55));
        subAb2.addChild(new GeneralTree<Integer>(18));
        GeneralTree<Integer> a1 = new GeneralTree<Integer>(8);
        a1.addChild(subAb1);
        a1.addChild(subAb2);

        arbol.addChild(a1);
        arbol.addChild(new GeneralTree<Integer>(42));

        GeneralTree<Integer> subAb3 = new GeneralTree<Integer>(19);
        subAb3.addChild(new GeneralTree<Integer>(4));
        GeneralTree<Integer> a2 = new GeneralTree<Integer>(-5);
        a2.addChild(subAb3);
        a2.addChild(new GeneralTree<Integer>(-9));

        arbol.addChild(a2);

        ParcialArboles p = new ParcialArboles(arbol);

        System.out.println(p.camino(2));
        System.out.println(p.camino(3));
    }

}
