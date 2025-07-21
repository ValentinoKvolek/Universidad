package main.java.parciales1.p9;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp2.ejercicio1.BinaryTree;

public class NivelArbol {
    BinaryTree<Integer> arbol;

    public NivelArbol(BinaryTree<Integer> arbol) {
        this.arbol = arbol;
    }

    public BinaryTree<Integer> minEnNivelAB (int n){

        BinaryTree<Integer> result = new BinaryTree<>();
        boolean encontre = false;

        if(arbol!= null){
            if(!arbol.isEmpty()){
                Queue<BinaryTree<Integer>> queue = new Queue<>();
                int nivel = 0;
                int min = Integer.MAX_VALUE;

                queue.enqueue(arbol);
                queue.enqueue(null);

                while(!queue.isEmpty()){
                    BinaryTree<Integer> actual = queue.dequeue();
                    if(actual != null){
                        if((nivel) == n-1){
                            encontre = true;
                            if(actual.isLeaf()){
                                if(actual.getData() < min){
                                    min = actual.getData();
                                }
                            }
                        }
                        if(actual.hasLeftChild()){
                            queue.enqueue(actual.getLeftChild());
                        }
                        if(actual.hasRightChild()){
                            queue.enqueue(actual.getRightChild());
                        }
                    }else {
                        nivel++;
                        if(encontre){
                            if(min == Integer.MAX_VALUE){
                                result.setData(null);
                                return result;
                            }
                            result.setData(min);
                            return result;
                        }
                        queue.enqueue(null);
                    }
                }
            }
        }
        return null;
    }

    public static void main(String[] args) {
        BinaryTree<Integer> ab = new BinaryTree<Integer>(2);
        ab.addLeftChild(new BinaryTree<Integer>(7));
        ab.getLeftChild().addLeftChild(new BinaryTree<Integer>(2));
        ab.getLeftChild().addRightChild(new BinaryTree<Integer>(6));
        ab.getLeftChild().getRightChild().addLeftChild(new BinaryTree<Integer>(5));
        ab.getLeftChild().getRightChild().addRightChild(new BinaryTree<Integer>(11));
        ab.addRightChild(new BinaryTree<Integer>(5));
        ab.getRightChild().addRightChild(new BinaryTree<Integer>(9));
        ab.getRightChild().getRightChild().addLeftChild(new BinaryTree<Integer>(4));

        NivelArbol a = new NivelArbol(ab);

        System.out.println(a.minEnNivelAB(1).getData());
        System.out.println(a.minEnNivelAB(3).getData());
        System.out.println(a.minEnNivelAB(4).getData());

        System.out.println("");

    }
}
