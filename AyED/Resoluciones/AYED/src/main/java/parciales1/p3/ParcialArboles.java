package main.java.parciales1.p3;

import main.java.tp2.ejercicio1.BinaryTree;

public class ParcialArboles {

    public static BinaryTree<int[]> sumAndDiff (BinaryTree<Integer> arbol){

        BinaryTree<int[]> arbolNuevo = new BinaryTree<>();

        if (!arbol.isEmpty() || arbol != null){

          arbolNuevo = recorrido(arbol, 0 , 0);

          return arbolNuevo;

        }

        return new BinaryTree<>();

    }

    private static BinaryTree<int[]> recorrido(BinaryTree<Integer> nodoActual, int suma, int resta){

        BinaryTree<int[]> arbolNuevo = new BinaryTree<>();

        int [] nuevoNodo = new int[2];

        nuevoNodo[0] = nodoActual.getData() + suma;
        suma = nuevoNodo[0];

        nuevoNodo[1] = nodoActual.getData() - resta;
        resta = nuevoNodo[1];

        arbolNuevo.setData(nuevoNodo);

        if (nodoActual.isLeaf()){
            return arbolNuevo;
        }

        if(nodoActual.hasLeftChild()){
            arbolNuevo.addLeftChild(recorrido(nodoActual.getLeftChild(), suma, nodoActual.getData()));
        }
        if(nodoActual.hasRightChild()){
            arbolNuevo.addRightChild(recorrido(nodoActual.getRightChild(),suma, nodoActual.getData()));
        }

        return arbolNuevo;
    }


    public static void main(String[] args) {
        
        BinaryTree<Integer> tree = new BinaryTree<>(20);

        BinaryTree<Integer> treeL = new BinaryTree<>(5);
        BinaryTree<Integer> treeR = new BinaryTree<>(30);

        BinaryTree<Integer> treeLL = new BinaryTree<>(-5);
        BinaryTree<Integer> treeLR = new BinaryTree<>(10);

        BinaryTree<Integer> treeRL = new BinaryTree<>(50);
        BinaryTree<Integer> treeRR = new BinaryTree<>(-5);

        BinaryTree<Integer> treeLRL = new BinaryTree<>(1);

        BinaryTree<Integer> treeRLR = new BinaryTree<>(4);

        BinaryTree<Integer> treeRLRR = new BinaryTree<>(6);

        tree.addLeftChild(treeL);
        tree.addRightChild(treeR);

        treeL.addLeftChild(treeLL);
        treeL.addRightChild(treeLR);

        treeR.addLeftChild(treeRL);
        treeR.addRightChild(treeRR);

        treeLR.addLeftChild(treeLRL);

        treeRL.addRightChild(treeRLR);

        treeRLR.addRightChild(treeRLRR);

        System.out.println(sumAndDiff(tree));

    }


}
