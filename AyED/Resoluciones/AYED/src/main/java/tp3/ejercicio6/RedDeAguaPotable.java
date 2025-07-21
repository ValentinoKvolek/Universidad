package main.java.tp3.ejercicio6;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp3.ejercicio1.GeneralTree;


public class RedDeAguaPotable {


    public GeneralTree<Character> estructuraDeCanos;


    public double minimoCaudal(double caudal){

        return minimoCaudalRec(this.estructuraDeCanos, caudal);

    }

    private double minimoCaudalRec(GeneralTree<Character> nodo, double caudalActual){

        // si es una hoja revuelvo el caudal actual que tengo.
        if(nodo.isLeaf()){
            return caudalActual;
        }

        Double caudalMin = Double.MAX_VALUE;

        int hijos = nodo.getChildren().size(); // con esto puedo saber la cantidad de hijos que tiene el nodo act

        double caudalNuevo = caudalActual/hijos; // mi caudal nuevo es el caudal act div la cant de hijos que tiene el nodo

        for (GeneralTree<Character> hijo : nodo.getChildren()) {

            //recorro y voy actualizando el caudal nuevo hasta que encuentre una hoja
            double caudalHijo = minimoCaudalRec(hijo, caudalNuevo);

            //una vez que retorna calculo si el caudal de esa hoja es el min que me quedo.
            if (caudalHijo < caudalMin) {
                caudalMin = caudalHijo;
            }
        }

        return caudalMin;
    }
}
