package main.java.parciales1.p2;

import main.java.tp2.ejercicio1.BinaryTree;

public class ParcialArboles {

    BinaryTree<Integer> arbol;

    public ParcialArboles(BinaryTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public Boolean isTwoTree(int num){

        boolean encontre = false;

        if (arbol != null &&  !arbol.isEmpty()){

            encontre = recorrer(arbol, num);

            return encontre;
        }

        return false;

    }

    private boolean recorrer(BinaryTree<Integer> nodo, int num){

        if (nodo.getData() == num){
            int cantHijosIzq = 0;
            int cantHijosDer = 0;

            if(!nodo.hasLeftChild()){
                cantHijosIzq = -1;
            }
            if(!nodo.hasRightChild()){
                cantHijosDer = -1;
            }

            if(nodo.hasLeftChild() && nodo.hasRightChild()) {
                cantHijosIzq = resolver(nodo.getLeftChild(), cantHijosIzq);
                cantHijosDer = resolver(nodo.getRightChild(), cantHijosDer);
            }


            if(cantHijosIzq == cantHijosDer){
                return true;
            }

        }

        if (nodo.hasLeftChild()){
            if(recorrer(nodo.getLeftChild(), num)){
                return true;
            }
        }
        if(nodo.hasRightChild()){
            if(recorrer(nodo.getRightChild(), num)){
                return true;
            }
        }

        return false;
    }

    private int resolver(BinaryTree<Integer> nodo, int cant){

        if (nodo.hasRightChild() && nodo.hasLeftChild()){
            cant++;
        }

        if (nodo.hasRightChild()){
            cant = resolver(nodo.getRightChild(), cant);
        }
        if (nodo.hasLeftChild()){
            cant = resolver(nodo.getLeftChild(), cant);
        }

        return cant;

    }

    public static void main (String[] args) {

        BinaryTree<Integer> ab = new BinaryTree<Integer>(2);
        ab.addLeftChild(new BinaryTree<Integer>(7));
        ab.addRightChild(new BinaryTree<Integer>(-5));
        ab.getLeftChild().addLeftChild(new BinaryTree<Integer>(23));
        ab.getLeftChild().addRightChild(new BinaryTree<Integer>(6));
        ab.getLeftChild().getLeftChild().addLeftChild(new BinaryTree<Integer>(-3));
        ab.getLeftChild().getRightChild().addLeftChild(new BinaryTree<Integer>(55));
        ab.getLeftChild().getRightChild().getLeftChild().addLeftChild(new BinaryTree<Integer>(9));
        ab.getLeftChild().getRightChild().getLeftChild().addRightChild(new BinaryTree<Integer>(16));
        ab.getRightChild().addLeftChild(new BinaryTree<Integer>(19));
        ab.getRightChild().addRightChild(new BinaryTree<Integer>(4));
        ab.getRightChild().getRightChild().addRightChild(new BinaryTree<Integer>(18));
        ab.getRightChild().getRightChild().getRightChild().addLeftChild(new BinaryTree<Integer>(8));
        ab.getRightChild().getRightChild().getRightChild().addRightChild(new BinaryTree<Integer>(24));
        ab.entreNiveles(0, 4);

        ParcialArboles parcialArb = new ParcialArboles(ab);
        System.out.println("Resultado=" + parcialArb.isTwoTree(2));
        System.out.println("Resultado=" + parcialArb.isTwoTree(7));
        System.out.println("Resultado=" + parcialArb.isTwoTree(-3));
        System.out.println("Resultado=" + parcialArb.isTwoTree(4));
        System.out.println("Resultado=" + parcialArb.isTwoTree(55));
    }


}
