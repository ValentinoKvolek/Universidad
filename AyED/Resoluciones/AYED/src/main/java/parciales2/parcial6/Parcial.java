package main.java.parciales2.parcial6;


import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;

import java.util.Iterator;
import java.util.List;

public class Parcial {

    public int resolver(Graph<Ciudad> ciudades, String origen, String destino, int maxControles){

        int max = 0;
        if(ciudades != null){
            if(!ciudades.isEmpty()){
                boolean[] visitados = new boolean[ciudades.getSize()];
                Vertex<Ciudad> inicio = buscar(ciudades,  origen);
                Vertex<Ciudad> destinoFinal = buscar(ciudades, destino);
                if(inicio !=null && destinoFinal !=null ){
                   max = resolver(inicio,destinoFinal,ciudades,maxControles,max,visitados,0);
                }
            }
        }

        return max;
    }

    private int resolver(Vertex<Ciudad> vertice, Vertex<Ciudad> destinoFinal, Graph<Ciudad> ciudades, int maxControles, int max, boolean[] visitados, int dias){

        visitados[vertice.getPosition()] = true;

        dias += vertice.getData().limiteDias;


        if (vertice.getData().nombre.equals(destinoFinal.getData().nombre)){
            if(dias > max){
                max = dias;
            }
            visitados[vertice.getPosition()] = false;
            return max;

        }

        List<Edge<Ciudad>> edges = ciudades.getEdges(vertice);

        for (Edge<Ciudad> e : edges){
            int j = e.getTarget().getPosition();
            if(!visitados[j]){
                if(e.getWeight() <=maxControles){
                    max = resolver(e.getTarget(),destinoFinal,ciudades,maxControles,max,visitados,dias);
                }
            }
        }

        visitados[vertice.getPosition()] = false;

        return max;
    }




    private Vertex<Ciudad> buscar(Graph<Ciudad> grafo, String dato) {

        Iterator<Vertex<Ciudad>> i = grafo.getVertices().iterator();
        Vertex<Ciudad> vf = null;
        boolean encontre = false;
        while (i.hasNext() && !encontre) {
            Vertex<Ciudad> v = i.next();
            if (v.getData().nombre.equals(dato)) {
                vf = v;
                encontre = true;
            }
        }
        if (encontre) {
            return vf;
        }
        return null;
    }


    public static void main(String args[]) {
        Graph<Ciudad> grafo = new AdjListGraph<Ciudad>();
        //Descarte Saladillo, Lobos y Pinamar
        Vertex<Ciudad> v1 = grafo.createVertex(new Ciudad("Suipacha", 3));
        Vertex<Ciudad> v2 = grafo.createVertex(new Ciudad("Carlos Keen", 2));
        Vertex<Ciudad> v3 = grafo.createVertex(new Ciudad("Moreno", 2));
        Vertex<Ciudad> v4 = grafo.createVertex(new Ciudad("Quilmes", 3));
        Vertex<Ciudad> v5 = grafo.createVertex(new Ciudad("Navarro", 1));
        Vertex<Ciudad> v6 = grafo.createVertex(new Ciudad("Cañuelas", 2));
        Vertex<Ciudad> v7 = grafo.createVertex(new Ciudad("Abasto", 3));
        Vertex<Ciudad> v8 = grafo.createVertex(new Ciudad("La Plata", 1));

        grafo.connect(v1, v2, 2);
        grafo.connect(v2, v1, 2);
        grafo.connect(v2, v3, 2);
        grafo.connect(v3, v2, 2);
        grafo.connect(v3, v4, 2);
        grafo.connect(v4, v3, 2);
        grafo.connect(v1, v5, 2);
        grafo.connect(v5, v1, 2);
        grafo.connect(v5, v6, 2);
        grafo.connect(v6, v5, 2);
        grafo.connect(v6, v7, 2);
        grafo.connect(v7, v6, 2);
        grafo.connect(v7, v3, 3);
        grafo.connect(v3, v7, 3);
        grafo.connect(v7, v8, 2);
        grafo.connect(v8, v7, 2);
        grafo.connect(v8, v4, 2);
        grafo.connect(v4, v8, 2);

        Parcial p = new Parcial();

        System.out.println(p.resolver(grafo, "La Plata", "Suipacha", 2));
        //System.out.println(p.resolver(grafo, "La Plata", "Carlos Keen", 2));
    }


}
