package main.java.tp3.ejercicio8;

import main.java.tp3.ejercicio1.GeneralTree;

import java.util.List;

public class Navidad<T>{

    GeneralTree<T> arbol;
    String resultado="YES";


    public Navidad(GeneralTree<T> arbol) {
        this.arbol = arbol;
    }

    public String esAbetoNavidenio(){


        esAbetoRec(arbol);

        return resultado;

    }

    private void esAbetoRec(GeneralTree<T> nodo){

        int contador =0;

        if (nodo.isEmpty()){
            return;
        }

        //si no es una hoja
        if(!nodo.isLeaf()){

            List<GeneralTree<T>> hijos = nodo.getChildren();

            for(GeneralTree<T> hijo : hijos ){

                if(hijo.isLeaf()){
                    contador++;
                }
                else{
                    esAbetoRec(hijo);
                }

            }

            if(contador < 3){
                resultado = "NO";
                return;
            }
        }
    }
}
