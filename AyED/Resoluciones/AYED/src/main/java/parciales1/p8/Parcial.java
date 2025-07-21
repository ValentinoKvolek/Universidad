package main.java.parciales1.p8;

import main.java.tp3.ejercicio1.GeneralTree;
import org.ietf.jgss.GSSName;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Parcial {

    public List<List<Character>> caminosPares(GeneralTree<Character> arbol){

            List<List<Character>> caminos = new ArrayList<>();
            List<Character> caminoAct = new ArrayList<>();

            if (arbol !=  null){
                if (!arbol.isEmpty()){
                    resolver(arbol, caminos, caminoAct);
                }
                return caminos;
            }

        return caminos;

    }

    private void resolver(GeneralTree<Character> nodo, List<List<Character>> caminos, List<Character> caminoAct){

        caminoAct.add(nodo.getData());

        if(nodo.isLeaf()){
            if((caminoAct.size() %2 == 0)){
                caminos.add(new ArrayList<>(caminoAct));
            }
            caminoAct.remove(caminoAct.size()-1);
            return;
        }
        List<GeneralTree<Character>> hijos = nodo.getChildren();
        for(GeneralTree<Character> hijo:hijos){
            resolver(hijo, caminos, caminoAct);
        }
        caminoAct.remove(caminoAct.size()-1);
    }

    public static void main(String args[]) {
        Parcial p = new Parcial();

        List<GeneralTree<Character>> subChildren1 = new LinkedList<GeneralTree<Character>>();
        subChildren1.add(new GeneralTree<Character>('E'));
        GeneralTree<Character> a1 = new GeneralTree<Character>('B', subChildren1);

        List<GeneralTree<Character>> subChildren2 = new LinkedList<GeneralTree<Character>>();
        subChildren2.add(new GeneralTree<Character>('H'));
        subChildren2.add(new GeneralTree<Character>('I'));
        GeneralTree<Character> subAb1 = new GeneralTree<Character>('F', subChildren2);
        List<GeneralTree<Character>> subChildren3 = new LinkedList<GeneralTree<Character>>();
        subChildren3.add(subAb1);
        subChildren3.add(new GeneralTree<Character>('G'));
        GeneralTree<Character> a2 = new GeneralTree<Character>('C', subChildren3);

        List<GeneralTree<Character>> arbol = new LinkedList<GeneralTree<Character>>();
        arbol.add(a1);
        arbol.add(a2);
        arbol.add(new GeneralTree<Character>('D'));
        GeneralTree<Character> a = new GeneralTree<Character>('A', arbol);

        List<List<Character>> lis = p.caminosPares(a);
        for(List<Character>l: lis) {
            System.out.println(l);
        }
    }


}
