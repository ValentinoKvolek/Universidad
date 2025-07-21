package main.java.tp2.ejercicio3;

import main.java.tp2.ejercicio1.BinaryTree;

import java.util.ArrayList;

public class ContadorArbol {

    private BinaryTree<Integer> ab;

    public ContadorArbol(BinaryTree<Integer> arbol) {
        this.ab = arbol;
    }

    public ArrayList<Integer> numerosPares() {

        ArrayList<Integer> resultado = new ArrayList<>();

        numerosParesInOrden(ab, resultado);
        return resultado;
    }
    public ArrayList<Integer> numerosPares2(){
        ArrayList<Integer> resultado2 = new ArrayList<>();
        numerosParesPostOrden(ab, resultado2);
        return resultado2;
    }

    private void numerosParesInOrden(BinaryTree<Integer> arbol, ArrayList<Integer> lista){

        if(arbol.isEmpty()){
            return;
        }

        if(arbol.hasLeftChild())
            numerosParesInOrden(arbol.getLeftChild(),lista);

        if(arbol.getData() % 2 ==0){
            lista.add(arbol.getData());
        }

        if(arbol.hasRightChild())
            numerosParesInOrden(arbol.getRightChild(), lista);

    }

    private void numerosParesPostOrden(BinaryTree<Integer> arbol, ArrayList<Integer> lista){
        if(arbol.isEmpty()){
            return;
        }
        if(arbol.hasLeftChild())
            numerosParesPostOrden(arbol.getLeftChild(),lista);
        if(arbol.hasRightChild())
            numerosParesPostOrden(arbol.getRightChild(),lista);
        if(arbol.getData() % 2 ==0)
            lista.add(arbol.getData());
    }
}