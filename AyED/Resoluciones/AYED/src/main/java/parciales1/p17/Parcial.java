package main.java.parciales1.p17;

import main.java.tp2.ejercicio1.BinaryTree;

public class Parcial {

    public BinaryTree<Integer> resolverEspejo(BinaryTree<Integer> ab){

        if(ab != null){
            if(!ab.isEmpty()){
                BinaryTree<Integer> nuevoArbol=  new BinaryTree<>();
                nuevoArbol.setData(ab.getData());
                resolver(ab,nuevoArbol,0 );
                return nuevoArbol;
            }
        }
        return new BinaryTree<>();
    }

    private void resolver(BinaryTree<Integer> nodo, BinaryTree<Integer>na, int valorAcumulado){

        if(nodo.getData() != null) {
            valorAcumulado += nodo.getData();
        }

        if(nodo.hasLeftChild()){
            int aux =valorAcumulado + nodo.getLeftChild().getData();
            BinaryTree<Integer> nuevoNodo = new BinaryTree<>();
            nuevoNodo.setData(aux);
            na.addRightChild(nuevoNodo);
            resolver(nodo.getLeftChild(), na.getRightChild(), valorAcumulado);
        }
        if(nodo.hasRightChild()){
            int aux = valorAcumulado + nodo.getRightChild().getData();
            BinaryTree<Integer> nuevoNodo = new BinaryTree<>();
            nuevoNodo.setData(aux);
            na.addLeftChild(nuevoNodo);
            resolver(nodo.getRightChild(), na.getLeftChild(),valorAcumulado);
        }

        return;
    }

    public static void main(String[] args) {
        Parcial p = new Parcial();

        BinaryTree<Integer> tree = new BinaryTree<>(4);

        BinaryTree<Integer> treeL = new BinaryTree<>(2);
        BinaryTree<Integer> treeR = new BinaryTree<>(7);

        BinaryTree<Integer> treeLL = new BinaryTree<>(1);
        BinaryTree<Integer> treeLR = new BinaryTree<>(2);

        tree.addLeftChild(treeL);
        tree.addRightChild(treeR);

        treeL.addLeftChild(treeLL);
        treeL.addRightChild(treeLR);

        System.out.println(tree);
        System.out.println();
        System.out.println(p.resolverEspejo(tree));
    }
}
