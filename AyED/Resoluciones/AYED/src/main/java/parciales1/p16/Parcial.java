package main.java.parciales1.p16;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp2.ejercicio1.BinaryTree;
import main.java.tp3.ejercicio1.GeneralTree;
import org.ietf.jgss.GSSName;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Parcial {

    public Integer resolver(GeneralTree<Integer> ab){
        if(ab != null){
            if(!ab.isEmpty()){

                int producto = 1;
                int profundidad=0;
                int maxProfundidad=0;

                List<Integer> hojasMax = new ArrayList<>();

                Queue<GeneralTree<Integer>> queue = new Queue<>();

                queue.enqueue(ab);
                queue.enqueue(null);

                while(!queue.isEmpty()){

                    GeneralTree<Integer>actual = queue.dequeue();

                    if(actual !=null){

                        if(actual.isLeaf()) {
                            if (profundidad > maxProfundidad) {
                                maxProfundidad = profundidad;
                                hojasMax.clear();
                                hojasMax.add(actual.getData());
                            }else if (profundidad == maxProfundidad) {
                            hojasMax.add(actual.getData());
                            }
                        }
                        List<GeneralTree<Integer>> hijos = actual.getChildren();
                        for(GeneralTree<Integer> hijo : hijos){
                            queue.enqueue(hijo);
                        }
                    }else{
                        if (!queue.isEmpty()) {
                            profundidad++;
                            queue.enqueue(null);
                        }
                    }
                }
                for (Integer valor : hojasMax) {
                    producto *= valor;
                }
                return producto;
            }
        }

        return 0;
    }
    public static void main(String[] args) {
        // Nivel 3 (hojas)
        GeneralTree<Integer> nodo22 = new GeneralTree<>(22);
        GeneralTree<Integer> nodo2 = new GeneralTree<>(2);

        // Nivel 2
        List<GeneralTree<Integer>> hijosDe10 = new LinkedList<>();
        hijosDe10.add(nodo22);
        hijosDe10.add(nodo2);
        GeneralTree<Integer> nodo10 = new GeneralTree<>(10, hijosDe10);

        GeneralTree<Integer> nodo6 = new GeneralTree<>(6);
        GeneralTree<Integer> nodo0 = new GeneralTree<>(0);
        GeneralTree<Integer> nodo8 = new GeneralTree<>(8);

        List<GeneralTree<Integer>> hijosDe45 = new LinkedList<>();
        hijosDe45.add(nodo6);
        hijosDe45.add(nodo0);
        hijosDe45.add(nodo8);
        GeneralTree<Integer> nodo45 = new GeneralTree<>(45, hijosDe45);

        // Nivel 1
        GeneralTree<Integer> nodo1 = new GeneralTree<>(1);
        GeneralTree<Integer> nodo21 = new GeneralTree<>(21);
        nodo21.addChild(nodo10); // 21 -> 10

        List<GeneralTree<Integer>> hijosDe20 = new LinkedList<>();
        hijosDe20.add(nodo1);
        hijosDe20.add(nodo45);
        hijosDe20.add(nodo21);

        // Raíz
        GeneralTree<Integer> raiz = new GeneralTree<>(20, hijosDe20);

        // Test opcional
        Parcial p = new Parcial();
        System.out.println(p.resolver(raiz));
    }


}
