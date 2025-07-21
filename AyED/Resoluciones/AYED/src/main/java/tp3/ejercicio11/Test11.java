package main.java.tp3.ejercicio11;

import main.java.tp3.ejercicio1.GeneralTree;

public class Test11 {
    public static void main(String[] args) {
        System.out.println("Árbol que debería devolver TRUE:");
        System.out.println(resolverTrue());

        System.out.println("\nÁrbol que debería devolver FALSE:");
        System.out.println(resolverFalse());
    }

    public static boolean resolverTrue() {
        GeneralTree<Integer> root = new GeneralTree<>(2);

        GeneralTree<Integer> n1 = new GeneralTree<>(1);
        GeneralTree<Integer> n25 = new GeneralTree<>(25);
        root.addChild(n1);
        root.addChild(n25);

        GeneralTree<Integer> n5 = new GeneralTree<>(5);
        GeneralTree<Integer> n4 = new GeneralTree<>(4);
        GeneralTree<Integer> n13 = new GeneralTree<>(13);
        n1.addChild(n5);
        n1.addChild(n4);
        n25.addChild(n13);

        GeneralTree<Integer> n18 = new GeneralTree<>(18);
        GeneralTree<Integer> n11 = new GeneralTree<>(11);
        GeneralTree<Integer> n7 = new GeneralTree<>(7);
        GeneralTree<Integer> n9 = new GeneralTree<>(9);
        n5.addChild(n18);
        n4.addChild(n11);
        n4.addChild(n7);
        n13.addChild(n9);

        GeneralTree<Integer> n83 = new GeneralTree<>(83);
        GeneralTree<Integer> n33 = new GeneralTree<>(33);
        GeneralTree<Integer> n12 = new GeneralTree<>(12);
        GeneralTree<Integer> n17 = new GeneralTree<>(17);
        GeneralTree<Integer> n6 = new GeneralTree<>(6);
        n18.addChild(n83);
        n18.addChild(n33);
        n11.addChild(n12);
        n7.addChild(n17);
        n9.addChild(n6);

        return ParcialArboles11.resolver(root);
    }

    public static boolean resolverFalse() {
        GeneralTree<Integer> root = new GeneralTree<>(2);

        GeneralTree<Integer> n1 = new GeneralTree<>(1);
        GeneralTree<Integer> n25 = new GeneralTree<>(25);
        root.addChild(n1);
        root.addChild(n25);

        GeneralTree<Integer> n5 = new GeneralTree<>(5);
        GeneralTree<Integer> n4 = new GeneralTree<>(4);
        GeneralTree<Integer> n13 = new GeneralTree<>(13);
        n1.addChild(n5);
        n1.addChild(n4);
        n25.addChild(n13);

        GeneralTree<Integer> n18 = new GeneralTree<>(18);
        GeneralTree<Integer> n11 = new GeneralTree<>(11);
        GeneralTree<Integer> n3 = new GeneralTree<>(3);  // ❌ este es distinto al anterior
        n5.addChild(n18);
        n4.addChild(n11);
        n13.addChild(n3); // ❌ en lugar de tener 4 nodos, nivel 3 tiene 3 → da false

        GeneralTree<Integer> n83 = new GeneralTree<>(83);
        GeneralTree<Integer> n33 = new GeneralTree<>(33);
        GeneralTree<Integer> n12 = new GeneralTree<>(12);
        GeneralTree<Integer> n17 = new GeneralTree<>(17);
        GeneralTree<Integer> n9 = new GeneralTree<>(9);
        n18.addChild(n83);
        n18.addChild(n33);
        n11.addChild(n12);
        n3.addChild(n17);
        n3.addChild(n9);

        return ParcialArboles11.resolver(root);
    }
}
