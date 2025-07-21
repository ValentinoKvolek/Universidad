package main.java.parciales1.p6;

import main.java.tp2.ejercicio1.BinaryTree;

import java.util.ArrayList;
import java.util.List;

public class ParcialArboles {

    BinaryTree<Integer> arbol;

    public ParcialArboles(BinaryTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public Parcial procesar(){

        Parcial p = new Parcial();
        if (arbol !=null){
            if(!arbol.isEmpty()){
                recorrer(arbol,p);
                return p;
            }
        }
        return p;
    }

    private void recorrer(BinaryTree<Integer> nodo,Parcial p){
        int hijos = 0;

        if(nodo.isLeaf()){
            if(nodo.getData() % 2 != 0){
                p.cantImpares++;
                return;
            }
        }

        if (nodo.hasLeftChild()){
            hijos++;
            recorrer(nodo.getLeftChild(), p);
        }
        if(nodo.hasRightChild()){
            hijos++;
            recorrer(nodo.getRightChild(), p);
        }

        if ((hijos == 1) && (nodo.getData() % 2 != 0 )){
            p.numeros.add(nodo.getData());
        }
        if(nodo.getData() % 2 != 0){
            p.cantImpares++;
        }
    }

    public static void main(String[] args) {

            BinaryTree<Integer> ab = new BinaryTree<Integer>(10);

            ab.addLeftChild(new BinaryTree<Integer>(6));
            ab.getLeftChild().addLeftChild(new BinaryTree<Integer>(1));
            ab.getLeftChild().getLeftChild().addRightChild(new BinaryTree<Integer>(20));
            ab.getLeftChild().addRightChild(new BinaryTree<>(9));
            ab.getLeftChild().getRightChild().addLeftChild(new BinaryTree<>(11));

            ab.addRightChild(new BinaryTree<Integer>(2));
            ab.getRightChild().addLeftChild(new BinaryTree<Integer>(8));
            ab.getRightChild().getLeftChild().addLeftChild(new BinaryTree<Integer>(2));
            ab.getRightChild().addRightChild(new BinaryTree<Integer>(3));
            ab.getRightChild().getRightChild().addRightChild(new BinaryTree<>(4));
            ab.getRightChild().getLeftChild().addRightChild(new BinaryTree<>(4));

            System.out.println(ab);

            ParcialArboles p = new ParcialArboles (ab);
            Parcial parc = p.procesar();

            System.out.println(" ");

            System.out.println(parc.getNumeros());
            System.out.println(parc.getCantImpares());

    }

}





class Parcial{

    List<Integer> numeros = new ArrayList<>();
    int cantImpares;

    public Parcial() {}

    public int getCantImpares() {
        return cantImpares;
    }

    public void setCantImpares(int cantImpares) {
        this.cantImpares = cantImpares;
    }

    public List<Integer> getNumeros() {
        return numeros;
    }

    public void setNumeros(List<Integer> numeros) {
        this.numeros = numeros;
    }
}