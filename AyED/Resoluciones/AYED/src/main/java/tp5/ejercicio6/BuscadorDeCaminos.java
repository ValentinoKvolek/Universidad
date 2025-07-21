package main.java.tp5.ejercicio6;

import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;
import org.w3c.dom.ls.LSInput;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class BuscadorDeCaminos {

    Graph<String> bosque;

    public BuscadorDeCaminos(Graph<String> bosque) {
        this.bosque = bosque;
    }

    public List<List<String>> recorridosMasSeguros(){

        List<List<String>> caminos = new LinkedList<List<String>>();
        List<String> camino = new LinkedList<String>();
        boolean [] visitados = new boolean[bosque.getSize()];

        if(!bosque.isEmpty()){
            Vertex<String> origen = bosque.search("Casa Caperucita");
            Vertex<String> destino = bosque.search("Casa Abuelita");

            if(origen !=null && destino !=null){
                recorridosMasSeguros(origen.getPosition(), destino.getPosition(), bosque,visitados, caminos, camino);
            }
        }

        return caminos;
    }

    private void recorridosMasSeguros(int i, int f,Graph<String> grafo, boolean[] visitados, List<List<String>> caminos, List<String> caminoAct){

        visitados[i]=true;

        Vertex<String> v = grafo.getVertex(i);
        caminoAct.add(v.getData());

        if(v.getData().equals(grafo.getVertex(f).getData())) {
            caminos.add(new LinkedList<String>(caminoAct));
        }

        List<Edge<String>> edges = grafo.getEdges(v);
        for (Edge<String> e : edges){
            int j = e.getTarget().getPosition();
            if(!visitados[j]){
                if(e.getWeight() <= 5) {
                    recorridosMasSeguros(j, f, grafo, visitados, caminos, caminoAct);
                }
            }
        }

        caminoAct.remove(caminoAct.size()-1);
        visitados[i]=false ;
    }

    public static void main (String[] args) {
        Graph<String> bosque = new AdjListGraph<String>();
        Vertex<String> v1 = bosque.createVertex("Casa Caperucita");
        Vertex<String> v2 = bosque.createVertex("Claro 3");
        Vertex<String> v3 = bosque.createVertex("Claro 1");
        Vertex<String> v4 = bosque.createVertex("Claro 2");
        Vertex<String> v5 = bosque.createVertex("Claro 5");
        Vertex<String> v6 = bosque.createVertex("Claro 4");
        Vertex<String> v7 = bosque.createVertex("Casa Abuelita");
        bosque.connect(v1, v2, 4);
        bosque.connect(v2, v1, 4);
        bosque.connect(v1, v3, 3);
        bosque.connect(v3, v1, 3);
        bosque.connect(v1, v4, 4);
        bosque.connect(v4, v1, 4);
        bosque.connect(v2, v5, 15);
        bosque.connect(v5, v2, 15);
        bosque.connect(v3, v5, 3);
        bosque.connect(v5, v3, 3);
        bosque.connect(v4, v3, 4);
        bosque.connect(v3, v4, 4);
        bosque.connect(v4, v5, 11);
        bosque.connect(v5, v4, 11);
        bosque.connect(v4, v6, 10);
        bosque.connect(v6, v4, 10);
        bosque.connect(v4, v3, 4);
        bosque.connect(v3, v4, 4);
        bosque.connect(v5, v7, 4);
        bosque.connect(v7, v5, 4);
        bosque.connect(v6, v7, 9);
        bosque.connect(v7, v6, 9);
        BuscadorDeCaminos bos = new BuscadorDeCaminos(bosque);
        List<List<String>> lis = bos.recorridosMasSeguros();
        for(List<String> l: lis) {
            System.out.println(l);
        }
    }
}
