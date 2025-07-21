package main.java.parciales2.parcial4;

import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Parcial {

    public List<Camino> resolver(Graph<String> sitios, String origen, String destino, List<String> evitarPasarPor) {

        if (!sitios.isEmpty()) {

            boolean[] visitados = new boolean[sitios.getSize()];
            List<Camino> caminos = new ArrayList<Camino>();

            Camino caminoAct = new Camino();

            Vertex<String> inicio = sitios.search(origen);
            Vertex<String> finalReco = sitios.search(destino);

            if (inicio != null && finalReco != null) {
                resolver(inicio.getPosition(), finalReco.getPosition(), sitios, visitados, caminos, caminoAct, evitarPasarPor);
            }
            return caminos;
        }
        return  new ArrayList<Camino>();
    }


        private void resolver ( int i, int f, Graph<String > sitios,boolean[] visitados, List<Camino > caminos, Camino
        caminoAct, List < String > evitarPasarPor ){

            visitados[i] = true;

            Vertex<String> v = sitios.getVertex(i);

            if (!evitarPasarPor.contains(v.getData())) {
                caminoAct.camino.add(v.getData());
            }else{return;}

            if (v.getData().equals(sitios.getVertex(f).getData())) {
                caminos.add(new Camino(caminoAct));
            }

            List<Edge<String>> edges = sitios.getEdges(v);

            for (Edge<String> e : edges) {

                int j = e.getTarget().getPosition();

                if (!visitados[j]) {
                    caminoAct.totalCuadras += e.getWeight();
                    resolver(j, f, sitios, visitados, caminos, caminoAct, evitarPasarPor);
                    caminoAct.totalCuadras -= e.getWeight();
                }
            }

            visitados[i] = false;
            if (!caminoAct.camino.isEmpty()) {
                caminoAct.camino.remove(caminoAct.camino.size() - 1);
            }
        }


    public static void main(String args[]) {
        Graph<String> grafo = new AdjListGraph();
        Vertex v1 = grafo.createVertex("Estadio Diego Armando Maradona");
        Vertex v2 = grafo.createVertex("Legislatura");
        Vertex v3 = grafo.createVertex("Coliseo Podestá");
        Vertex v4 = grafo.createVertex("MACLA");
        Vertex v5 = grafo.createVertex("Catedral La Plata");
        Vertex v6 = grafo.createVertex("Palacio Campodónico");
        Vertex v7 = grafo.createVertex("Rectorado UNLP");
        Vertex v8 = grafo.createVertex("Museo UNLP");

        grafo.connect(v1, v2, 25);
        grafo.connect(v2, v1, 25);
        grafo.connect(v1, v3, 20);
        grafo.connect(v3, v1, 20);
        grafo.connect(v1, v4, 35);
        grafo.connect(v4, v1, 35);
        grafo.connect(v1, v5, 40);
        grafo.connect(v5, v1, 40);
        grafo.connect(v2, v3, 25);
        grafo.connect(v3, v2, 25);
        grafo.connect(v4, v5, 8);
        grafo.connect(v5, v4, 8);
        grafo.connect(v5, v7, 5);
        grafo.connect(v7, v5, 5);
        grafo.connect(v3, v6, 10);
        grafo.connect(v6, v3, 10);
        grafo.connect(v6, v7, 30);
        grafo.connect(v7, v6, 30);
        grafo.connect(v7, v8, 15);
        grafo.connect(v8, v7, 15);

        List<String> evitarPasarPor = new LinkedList<String>();
        evitarPasarPor.add("Legislatura");
        evitarPasarPor.add("MACLA");

        Parcial p = new Parcial();
        List<Camino> lis = p.resolver(grafo, "Estadio Diego Armando Maradona", "Palacio Campodónico", evitarPasarPor);

        for(Camino aux: lis) {
            System.out.println(aux);
        }
    }
    }
