package main.java.tp3.ejercicio4;

import main.java.tp3.ejercicio1.GeneralTree;

public class Test {
    public static void main(String[] args) {
        // Nivel 2
        GeneralTree<AreaEmpresa> a = new GeneralTree<>(new AreaEmpresa(4, "A"));
        GeneralTree<AreaEmpresa> b = new GeneralTree<>(new AreaEmpresa(7, "B"));
        GeneralTree<AreaEmpresa> c = new GeneralTree<>(new AreaEmpresa(5, "C"));
        GeneralTree<AreaEmpresa> d = new GeneralTree<>(new AreaEmpresa(6, "D"));
        GeneralTree<AreaEmpresa> e = new GeneralTree<>(new AreaEmpresa(10, "E"));
        GeneralTree<AreaEmpresa> f = new GeneralTree<>(new AreaEmpresa(18, "F"));
        GeneralTree<AreaEmpresa> g = new GeneralTree<>(new AreaEmpresa(9, "G"));
        GeneralTree<AreaEmpresa> h = new GeneralTree<>(new AreaEmpresa(12, "H"));
        GeneralTree<AreaEmpresa> i = new GeneralTree<>(new AreaEmpresa(19, "I"));

        // Nivel 1
        GeneralTree<AreaEmpresa> j = new GeneralTree<>(new AreaEmpresa(13, "J"));
        j.addChild(a);
        j.addChild(b);
        j.addChild(c);

        GeneralTree<AreaEmpresa> k = new GeneralTree<>(new AreaEmpresa(25, "K"));
        k.addChild(d);
        k.addChild(e);
        k.addChild(f);

        GeneralTree<AreaEmpresa> l = new GeneralTree<>(new AreaEmpresa(10, "L"));
        l.addChild(g);
        l.addChild(h);
        l.addChild(i);

        // Raíz
        GeneralTree<AreaEmpresa> m = new GeneralTree<>(new AreaEmpresa(14, "M"));
        m.addChild(j);
        m.addChild(k);
        m.addChild(l);

        // Probar método
        AnalizadorArbol analizador = new AnalizadorArbol();
        double resultado = analizador.devolverMaximoPromedio(m);
        System.out.println("El mayor promedio entre los niveles es: " + resultado); // Debería dar 16.0
    }
}
