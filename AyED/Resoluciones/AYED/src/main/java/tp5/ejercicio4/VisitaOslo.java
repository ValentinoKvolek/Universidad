package main.java.tp5.ejercicio4;

import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class VisitaOslo {

    public List<String> paseoEnBici(Graph<String> lugares, String destino, int maxTiempo, List<String> lugaresRestringidos) {

        List<String> camino = new ArrayList<String>();
        boolean[] visitados = new boolean[lugares.getSize()];
        boolean encontre = false;

        if (!lugares.isEmpty()) {

            Vertex<String> origen = lugares.search("Ayuntamiento");
            Vertex<String> llegada = lugares.search(destino);

            if (origen != null && llegada != null) {
                encontre = paseoEnBici(origen.getPosition(), llegada.getPosition(),camino, visitados ,lugares, maxTiempo, lugaresRestringidos, 0);
            }
        }

        if (encontre) {
            return camino;
        }
        return new ArrayList<>();
    }

    private boolean paseoEnBici(int i, int f,List<String> camino ,boolean[] visitados, Graph<String> lugares, int maxTiempo, List<String> lugaresRestringidos, int sumaTiempo) {

        Vertex<String> v = lugares.getVertex(i);

        if (lugaresRestringidos.contains(v.getData())) {
            return false;
        }

        visitados[i] = true;
        camino.add(v.getData());

        if(v.getData().equals(lugares.getVertex(f).getData())){
            return true; //encontre el camino
        }

        List<Edge<String>> edges = lugares.getEdges(v);

        for (Edge<String> e : edges){

            int j = e.getTarget().getPosition();

            int total = sumaTiempo + e.getWeight();
            if(!visitados[j] && total <= maxTiempo) {
                if (paseoEnBici(j, f, camino, visitados, lugares, maxTiempo, lugaresRestringidos, total)) {
                    return true;
                }
            }
        }

        camino.remove(camino.size()-1);
        visitados[i]=false;

        return false;

    }


    public static void main(String[] args) {
        Graph<String> lugares = new AdjListGraph<String>();
        Vertex<String> v1 = lugares.createVertex("Holmenkollen");
        Vertex<String> v2 = lugares.createVertex("Parque Vigeland");
        Vertex<String> v3 = lugares.createVertex("Galería Nacional");
        Vertex<String> v4 = lugares.createVertex("Parque Botánico");
        Vertex<String> v5 = lugares.createVertex("Museo Munch");
        Vertex<String> v6 = lugares.createVertex("FolkMuseum");
        Vertex<String> v7 = lugares.createVertex("Palacio Real");
        Vertex<String> v8 = lugares.createVertex("Ayuntamiento");
        Vertex<String> v9 = lugares.createVertex("El Tigre");
        Vertex<String> v10 = lugares.createVertex("Akker Brigge");
        Vertex<String> v11 = lugares.createVertex("Museo Fram");
        Vertex<String> v12 = lugares.createVertex("Museo Vikingo");
        Vertex<String> v13 = lugares.createVertex("La Opera");
        Vertex<String> v14 = lugares.createVertex("Museo del Barco Polar");
        Vertex<String> v15 = lugares.createVertex("Fortaleza Akershus");

        lugares.connect(v1, v2, 30);
        lugares.connect(v2, v1, 30);
        lugares.connect(v2, v3, 10);
        lugares.connect(v3, v2, 10);
        lugares.connect(v3, v4, 15);
        lugares.connect(v4, v3, 15);
        lugares.connect(v4, v5, 1);
        lugares.connect(v5, v4, 1);

        lugares.connect(v5, v9, 15);
        lugares.connect(v9, v5, 15);
        lugares.connect(v9, v13, 5);
        lugares.connect(v13, v9, 5);
        lugares.connect(v13, v15, 10);
        lugares.connect(v15, v13, 10);

        lugares.connect(v2, v6, 20);
        lugares.connect(v6, v2, 20);
        lugares.connect(v6, v7, 5);
        lugares.connect(v7, v6, 5);
        lugares.connect(v7, v8, 5);
        lugares.connect(v8, v7, 5);
        lugares.connect(v6, v10, 30);
        lugares.connect(v10, v6, 30);
        lugares.connect(v10, v8, 20);
        lugares.connect(v8, v10, 20);
        lugares.connect(v8, v4, 10);
        lugares.connect(v4, v8, 10);
        lugares.connect(v8, v9, 15);
        lugares.connect(v9, v8, 15);

        lugares.connect(v6, v11, 5);
        lugares.connect(v11, v6, 5);
        lugares.connect(v10, v12, 30);
        lugares.connect(v12, v10, 30);
        lugares.connect(v11, v14, 5);
        lugares.connect(v14, v11, 5);
        lugares.connect(v12, v14, 5);
        lugares.connect(v14, v12, 5);

        List<String> lugaresRestringidos = new LinkedList<String>();
        lugaresRestringidos.add("Akker Brigge");
        lugaresRestringidos.add("Palacio Real");
        VisitaOslo vis = new VisitaOslo();
        List<String> camino = vis.paseoEnBici(lugares, "Museo Vikingo", 120, lugaresRestringidos);

        System.out.println(camino);
    }

}

