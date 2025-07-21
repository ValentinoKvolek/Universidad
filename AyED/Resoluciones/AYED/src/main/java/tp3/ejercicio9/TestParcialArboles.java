package main.java.tp3.ejercicio9;

import main.java.tp3.ejercicio1.GeneralTree;

import java.util.ArrayList;
import java.util.List;

public class TestParcialArboles {

    public static void main(String[] args) {
        // Árbol que debe retornar true
        GeneralTree<Integer> t1_7 = new GeneralTree<>(35);
        GeneralTree<Integer> t1_8 = new GeneralTree<>(83);
        GeneralTree<Integer> t1_9 = new GeneralTree<>(90);
        GeneralTree<Integer> t1_10 = new GeneralTree<>(33);
        GeneralTree<Integer> t1_6 = new GeneralTree<>(33);
        t1_6.setChildren(List.of(t1_7, t1_8, t1_9, t1_10));

        GeneralTree<Integer> t1_3 = new GeneralTree<>(14);
        GeneralTree<Integer> t1_4 = new GeneralTree<>(12);
        GeneralTree<Integer> t1_5 = t1_6;

        GeneralTree<Integer> t1_2 = new GeneralTree<>(12);
        t1_2.setChildren(List.of(t1_3, t1_4, t1_5));

        GeneralTree<Integer> t1_1 = new GeneralTree<>(35);
        t1_1.setChildren(List.of(new GeneralTree<>(35)));

        GeneralTree<Integer> t1_left = new GeneralTree<>(12);
        t1_left.setChildren(List.of(t1_1, t1_2));

        GeneralTree<Integer> t1_right = new GeneralTree<>(25);
        t1_right.setChildren(List.of(new GeneralTree<>(25)));

        GeneralTree<Integer> root1 = new GeneralTree<>(12);
        root1.setChildren(List.of(t1_left, t1_right));

        // Árbol que debe retornar false (por nodo con valor 18 y un hijo 14)
        GeneralTree<Integer> t2_7 = new GeneralTree<>(35);
        GeneralTree<Integer> t2_8 = new GeneralTree<>(83);
        GeneralTree<Integer> t2_9 = new GeneralTree<>(90);
        GeneralTree<Integer> t2_10 = new GeneralTree<>(33);
        GeneralTree<Integer> t2_6 = new GeneralTree<>(33);
        t2_6.setChildren(List.of(t2_7, t2_8, t2_9, t2_10));

        GeneralTree<Integer> t2_3 = new GeneralTree<>(14); // menor que padre 18
        GeneralTree<Integer> t2_4 = new GeneralTree<>(18);
        GeneralTree<Integer> t2_5 = t2_6;

        GeneralTree<Integer> t2_2 = new GeneralTree<>(18);
        t2_2.setChildren(List.of(t2_3, t2_4, t2_5));

        GeneralTree<Integer> t2_1 = new GeneralTree<>(35);
        t2_1.setChildren(List.of(new GeneralTree<>(35)));

        GeneralTree<Integer> t2_left = new GeneralTree<>(12);
        t2_left.setChildren(List.of(t2_1, t2_2));

        GeneralTree<Integer> t2_right = new GeneralTree<>(25);
        t2_right.setChildren(List.of(new GeneralTree<>(25)));

        GeneralTree<Integer> root2 = new GeneralTree<>(12);
        root2.setChildren(List.of(t2_left, t2_right));

        // Prueba
        ParcialArboles parcial = new ParcialArboles();
        System.out.println("Árbol 1 (esperado true): " + parcial.esDeSeleccion(root1));
        System.out.println("Árbol 2 (esperado false): " + parcial.esDeSeleccion(root2));
    }
}
