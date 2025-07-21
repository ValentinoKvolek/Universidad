package main.java.parciales1.p7;

import main.java.tp2.ejercicio1.BinaryTree;

public class Parcial {

    BinaryTree<Personaje>arbol;

    public Parcial(BinaryTree<Personaje> arbol) {
        this.arbol = arbol;
    }

    public Personaje princesaAccesible (){

        Personaje resultado = new Personaje();

        if(arbol != null){
            if(!arbol.isEmpty()){
                resolver(arbol,resultado);
                return resultado;
            }
        }
        return new Personaje();
    }

    private boolean resolver(BinaryTree<Personaje> nodo, Personaje resultado){

        if (nodo.isLeaf()){
            if (nodo.getData().esPrincesa()){
                resultado.setNombre(nodo.getData().nombre);
                resultado.setTipo(nodo.getData().tipo);
                return true;
            }
        }

        if (nodo.hasLeftChild()){
            if(!nodo.getLeftChild().getData().esDragon()){
                if(resolver(nodo.getLeftChild(), resultado)){
                    return true;
                }
            }
        }

        if (nodo.hasRightChild()){
            if(!nodo.getRightChild().getData().esDragon()){
                if (resolver(nodo.getRightChild(), resultado)){
                    return true;
                }
            }
        }
        return false;
    }



    public static void main(String args[]) {

        BinaryTree<Personaje> ab = new BinaryTree<Personaje>(new Personaje("X", "Prueba"));
        ab.addLeftChild(new BinaryTree<Personaje>(new Personaje("Dragon", "Roberto")));
        ab.getLeftChild().addLeftChild(new BinaryTree<Personaje>(new Personaje("Princesa", "Roxanne")));
        ab.getLeftChild().addRightChild(new BinaryTree<Personaje>(new Personaje("Y", "Prueba")));
        ab.addRightChild(new BinaryTree<Personaje>(new Personaje("Z", "Prueba")));
        ab.getRightChild().addRightChild(new BinaryTree<Personaje>(new Personaje("W", "Prueba")));
        ab.getRightChild().getRightChild().addLeftChild(new BinaryTree<Personaje>(new Personaje("Princesa", "Diana")));

        Parcial p = new Parcial(ab);
        System.out.println(p.princesaAccesible());

    }


}




class Personaje{

    String nombre;
    String tipo;

    public Personaje(String tipo, String nombre) {
        this.tipo = tipo;
        this.nombre = nombre;
    }

    public Personaje() {

    }

    public String getNombre() {
        return nombre;
    }

    public void setNombre(String nombre) {
        this.nombre = nombre;
    }

    public String getTipo() {
        return tipo;
    }

    public void setTipo(String tipo) {
        this.tipo = tipo;
    }

    public boolean esDragon() {
        return this.tipo.equals("Dragon");
    }

    public boolean esPrincesa() {
        return this.tipo.equals("Princesa");
    }

    @Override
    public String toString() {
        return "Personaje{" +
                "nombre='" + nombre + '\'' +
                ", tipo='" + tipo + '\'' +
                '}';
    }
}

