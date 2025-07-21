package main.java.tp3.ejercicio1;

public class prueba {
        public static void main(String[] args) {
            GeneralTree<Integer> raiz = new GeneralTree<>(1);
            GeneralTree<Integer> nodo2 = new GeneralTree<>(2);
            GeneralTree<Integer> nodo3 = new GeneralTree<>(3);
            GeneralTree<Integer> nodo4 = new GeneralTree<>(4);
            GeneralTree<Integer> nodo5 = new GeneralTree<>(5);
            GeneralTree<Integer> nodo6 = new GeneralTree<>(6);

            // Agregando hijos a la raíz
            raiz.addChild(nodo2);
            raiz.addChild(nodo3);
            raiz.addChild(nodo4);

            // Agregando hijos a nodo2
            nodo2.addChild(nodo5);
            nodo2.addChild(nodo6);

            // Estructura del árbol:
            //         1
            //     /   |   \
            //    2    3    4
            //   / \
            //  5   6

            System.out.println("Nivel del nodo 1: " + raiz.nivel(1)); // 0
            System.out.println("Nivel del nodo 2: " + raiz.nivel(2)); // 1
            System.out.println("Nivel del nodo 5: " + raiz.nivel(5)); // 2
            System.out.println("Nivel del nodo 99: " + raiz.nivel(99)); // -1 (no existe)
            System.out.println("Ancho del árbol: " + raiz.ancho()); // Debe imprimir 3 (nivel con nodos 2,3,4)
        }

}
