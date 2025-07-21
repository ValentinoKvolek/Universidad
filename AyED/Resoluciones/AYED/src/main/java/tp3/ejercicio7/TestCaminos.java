package main.java.tp3.ejercicio7;

import main.java.tp3.ejercicio1.GeneralTree;
import main.java.tp3.ejercicio7.Caminos;

public class TestCaminos {
    public static void main(String[] args) {
        // Crear los nodos del árbol
        GeneralTree<Integer> nodo12 = new GeneralTree<>(12);
        GeneralTree<Integer> nodo17 = new GeneralTree<>(17);
        GeneralTree<Integer> nodo9 = new GeneralTree<>(9);
        GeneralTree<Integer> nodo15 = new GeneralTree<>(15);
        GeneralTree<Integer> nodo10 = new GeneralTree<>(10);
        GeneralTree<Integer> nodo6 = new GeneralTree<>(6);
        GeneralTree<Integer> nodo1 = new GeneralTree<>(1);
        GeneralTree<Integer> nodo8 = new GeneralTree<>(8);
        GeneralTree<Integer> nodo14 = new GeneralTree<>(14);
        GeneralTree<Integer> nodo18 = new GeneralTree<>(18);
        GeneralTree<Integer> nodo16 = new GeneralTree<>(16);
        GeneralTree<Integer> nodo7 = new GeneralTree<>(7);

        // Construir el árbol
        nodo12.addChild(nodo17);
        nodo12.addChild(nodo9);
        nodo12.addChild(nodo15);

        nodo17.addChild(nodo10);
        nodo17.addChild(nodo6);
        nodo6.addChild(nodo1);

        nodo9.addChild(nodo8);

        nodo15.addChild(nodo14);
        nodo15.addChild(nodo18);
        nodo14.addChild(nodo16);
        nodo14.addChild(nodo7);

        // Crear el objeto Caminos y probar el método
        Caminos caminos = new Caminos();
        caminos.arbol = nodo12; // Asignar el árbol
        System.out.println(caminos.caminoAHojaMasLejana());
    }
}