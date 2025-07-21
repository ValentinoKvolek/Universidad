package main.java.tp3.ejercicio8;

import main.java.tp3.ejercicio1.GeneralTree;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Test8 {

    public static void main(String[] args) {

        // Árbol 1 (Sí es abeto)
        GeneralTree<Integer> a1 = new GeneralTree<>(1);
        a1.setChildren(new ArrayList<>(List.of(
                new GeneralTree<>(2),
                new GeneralTree<>(3),
                new GeneralTree<>(4)
        )));

        // A2

        GeneralTree<Integer> a2_2 = new GeneralTree<>(2);
        a2_2.setChildren(new ArrayList<>(List.of(
                new GeneralTree<>(5),
                new GeneralTree<>(6),
                new GeneralTree<>(7)
        )));

        GeneralTree<Integer> a2 = new GeneralTree<>(1);
        a2.setChildren(List.of(
                a2_2,
                new GeneralTree<>(3),
                new GeneralTree<>(4)
        ));

        // A3

        GeneralTree<Integer> a3_3 = new GeneralTree<>(3);
        a3_3.setChildren(List.of(
                new GeneralTree<>(6),
                new GeneralTree<>(7),
                new GeneralTree<>(8)
        ));

        GeneralTree<Integer> a3 = new GeneralTree<>(1);
        a3.setChildren(List.of(
                new GeneralTree<>(2),
                a3_3,
                new GeneralTree<>(4),
                new GeneralTree<>(5)
        ));

        Navidad<Integer> nav1 = new Navidad<>(a1);
        Navidad<Integer> nav2 = new Navidad<>(a2);
        Navidad<Integer> nav3 = new Navidad<>(a3);

        System.out.println("Nav1: " + nav1.esAbetoNavidenio());
        System.out.println("Nav2: " + nav2.esAbetoNavidenio());
        System.out.println("Nav3: " + nav3.esAbetoNavidenio());
    }


}