package main.java.parciales2.parcial5;

import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;

import java.util.ArrayList;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.List;

public class Parcial {

    public List<String> recorrido (Graph<String> grafo, int cantLocalidades, int canNafta, List<String> localidadesExceptuadas){

        List<String> camino = new ArrayList<String>();
        boolean encontre =false;
        if (grafo!=null) {

            if (!grafo.isEmpty()) {

                Vertex<String> origen = grafo.search("Mendoza");
                boolean[] visitados = new boolean[grafo.getSize()];

                if(origen != null){
                    encontre = recorrido(origen, grafo,cantLocalidades, canNafta,localidadesExceptuadas, camino , visitados);
                }

            }

        }

        if (encontre)
            return camino;

        return new ArrayList<String>();
    }

    private boolean recorrido (Vertex<String> origen,Graph<String> grafo ,int cantLocalidades, int canNafta, List<String> localidadesExeptuadas, List<String> camino, boolean[] visitados){

        visitados[origen.getPosition()]= true;


        if(localidadesExeptuadas.contains(origen.getData()))
            return false;

        camino.add(origen.getData());

        if(camino.size() >= cantLocalidades){
            return true;
        }


        boolean encontrado = false;

        Iterator<Edge<String>> iter = grafo.getEdges(origen).iterator();

        while(iter.hasNext() && !encontrado){

            Edge<String> e = iter.next();

            int j = e.getTarget().getPosition();

            if(!visitados[j]){
                if((canNafta -  e.getWeight() >= 0)){
                    encontrado = recorrido(e.getTarget(), grafo, cantLocalidades, (canNafta-e.getWeight()), localidadesExeptuadas, camino,visitados);
                }
            }
        }


        if(!encontrado) {

            camino.remove(camino.size() - 1);
            visitados[origen.getPosition()] = false;
        }

        return encontrado;

    }

    public static void main(String args[]) {
        Graph<String> grafo = new AdjListGraph<String>();
        Vertex<String> v1 = grafo.createVertex("Mendoza");
        Vertex<String> v2 = grafo.createVertex("Tunuyán");
        Vertex<String> v3 = grafo.createVertex("San Martin");
        Vertex<String> v4 = grafo.createVertex("La Paz");
        Vertex<String> v5 = grafo.createVertex("Santa Rosa");
        Vertex<String> v6 = grafo.createVertex("San Rafael");
        Vertex<String> v7 = grafo.createVertex("Malargue");
        Vertex<String> v8 = grafo.createVertex("General Alvear");

        grafo.connect(v1, v2, 10);
        grafo.connect(v1, v3, 6);
        grafo.connect(v2, v3, 10);
        grafo.connect(v3, v4, 8);
        grafo.connect(v4, v5, 2);
        grafo.connect(v3, v6, 13);
        grafo.connect(v6, v2, 11);
        grafo.connect(v6, v8, 7);
        grafo.connect(v2, v7, 12);
        grafo.connect(v8, v7, 6);

        List<String> localidadesExceptuadas = new LinkedList<String>();
        localidadesExceptuadas.add("General Alvear");
        localidadesExceptuadas.add("La Paz");

        Parcial p = new Parcial();

        System.out.println(p.recorrido(grafo, 5, 80, localidadesExceptuadas));
    }
}
