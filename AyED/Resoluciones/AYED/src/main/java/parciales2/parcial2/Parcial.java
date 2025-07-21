package main.java.parciales2.parcial2;

import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;

import java.util.Iterator;
import java.util.List;

public class Parcial {

    public int resolver(Graph<Recinto> sitios, int tiempo){

        int max = Integer.MIN_VALUE;

        boolean[] visitados = new boolean[sitios.getSize()];

        if(!sitios.isEmpty() && sitios !=null){
            Vertex<Recinto> origen = buscar(sitios);
            if(origen !=null){
               max = resolver(origen.getPosition(), sitios, max, visitados,  tiempo, 0, 0);
            }
        }
        return max;
    }
    //comentario

    private Vertex<Recinto> buscar(Graph<Recinto> sitios){

        for (Vertex<Recinto> v : sitios.getVertices()){
            if (v.getData().nombre.equals("Entrada"))
                return v;

        }

        return null;
    }

    private int resolver (int i ,Graph<Recinto> sitios ,int max, boolean[] visitados, int tiempo, int tiempoPaseo,int cantRecintosActuales ){

        visitados [i] = true;

        Vertex<Recinto> v = sitios.getVertex(i);

        tiempoPaseo += v.getData().tiempo;

        if(tiempoPaseo <= tiempo){
            cantRecintosActuales++;
        }else{
            if (cantRecintosActuales > max){
                max = cantRecintosActuales;
            }
        }

        List<Edge<Recinto>> edges = sitios.getEdges(v);

        for(Edge<Recinto> e : edges){
            int j = e.getTarget().getPosition();
            if(!visitados[j]){
               max = resolver(j,sitios,max,visitados,tiempo,(tiempoPaseo + e.getWeight()),cantRecintosActuales);
            }
        }

        if(cantRecintosActuales > max) {
            max = cantRecintosActuales;
        }

        visitados[i] = false;

        return  max;

    }


    public static void main(String args[]) {
        Graph<Recinto> grafo = new AdjListGraph<Recinto>();
        Vertex<Recinto> Entrada = grafo.createVertex(new Recinto("Entrada", 15));
        Vertex<Recinto> Cebras = grafo.createVertex(new Recinto("Cebras", 10));
        Vertex<Recinto> Tigres = grafo.createVertex(new Recinto("Tigres", 10));
        Vertex<Recinto> Flamencos = grafo.createVertex(new Recinto("Flamencos", 10));
        Vertex<Recinto> Murcielagos = grafo.createVertex(new Recinto("Murciélagos", 20));
        Vertex<Recinto> Wallabies = grafo.createVertex(new Recinto("Wallabies", 30));
        Vertex<Recinto> Tortugas = grafo.createVertex(new Recinto("Tortugas", 10));
        Vertex<Recinto> Pumas = grafo.createVertex(new Recinto("Pumas", 10));

        grafo.connect(Entrada, Cebras, 10);
        grafo.connect(Cebras, Entrada, 10);
        grafo.connect(Entrada, Tigres, 10);
        grafo.connect(Tigres, Entrada, 10);
        grafo.connect(Entrada, Murcielagos, 20);
        grafo.connect(Murcielagos, Entrada, 20);
        grafo.connect(Entrada, Flamencos, 25);
        grafo.connect(Flamencos, Entrada, 25);

        grafo.connect(Tigres, Cebras, 8);
        grafo.connect(Cebras, Tigres, 8);
        grafo.connect(Cebras, Tortugas, 10);
        grafo.connect(Tortugas, Cebras, 10);
        grafo.connect(Flamencos, Murcielagos, 25);
        grafo.connect(Murcielagos, Flamencos, 25);
        grafo.connect(Murcielagos, Wallabies, 10);
        grafo.connect(Wallabies, Murcielagos, 10);
        grafo.connect(Wallabies, Tortugas, 10);
        grafo.connect(Tortugas, Wallabies, 10);
        grafo.connect(Tortugas, Pumas, 15);
        grafo.connect(Pumas, Tortugas, 15);

        Parcial p = new Parcial();

        System.out.println(p.resolver(grafo, 100));
        System.out.println(p.resolver(grafo, 30));
    }

}
