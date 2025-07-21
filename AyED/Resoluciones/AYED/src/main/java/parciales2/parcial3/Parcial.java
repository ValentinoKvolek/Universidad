package main.java.parciales2.parcial3;

import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;

import java.net.HttpURLConnection;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Parcial {

    public List<String> resolver(Graph<Ciudades> mapa, String ciudad, int cantDiasVacas){

        List<String> camino = new ArrayList<String>();
        List<String> caminoact = new ArrayList<String>();

        int max = Integer.MIN_VALUE;

        boolean[] visitados = new boolean[mapa.getSize()];

        if(!mapa.isEmpty() && mapa !=null){

            Vertex<Ciudades> origen = null;
            for (Vertex<Ciudades> o : mapa.getVertices()){
                if (o.getData().nombre.equals(ciudad)){
                    origen = o;
                }
            }
            if (origen != null){
                resolver(origen.getPosition(), max,visitados, mapa, cantDiasVacas, caminoact, camino, 0);
            }
        }

        return camino;
    }

    private int resolver(int i, int max,boolean[] visitados, Graph<Ciudades> mapa , int cantDias, List<String> caminoActual, List<String> camino, int diasTrascurridos){

        visitados[i]= true;

        Vertex<Ciudades> v = mapa.getVertex(i);


        diasTrascurridos += v.getData().dias;

        if(diasTrascurridos <= cantDias){
            caminoActual.add(v.getData().nombre);
        }else {
            if (caminoActual.size() > max){
                camino.clear();
                camino.addAll(caminoActual);
                max = caminoActual.size();
            }
        }

        List<Edge<Ciudades>> edges = mapa.getEdges(v);
        for (Edge<Ciudades> e : edges){
            int j = e.getTarget().getPosition();
            if(!visitados[j]){
                max = resolver(j,max,visitados,mapa,cantDias,caminoActual,camino,diasTrascurridos);
            }
        }

        if(!caminoActual.isEmpty())
            caminoActual.remove(caminoActual.size()-1);

        visitados[i]=false;

        return max;
    }

    public static void main(String args[]) {
        Graph<Ciudades> mapa = new AdjListGraph<Ciudades>();
        Vertex<Ciudades> MarDelPlata = mapa.createVertex(new Ciudades(7, "Mar Del Plata"));
        Vertex<Ciudades> Pila = mapa.createVertex(new Ciudades(1, "Pila"));
        Vertex<Ciudades> Dolores = mapa.createVertex(new Ciudades(1, "Dolores"));
        Vertex<Ciudades> Chascomus = mapa.createVertex(new Ciudades(1, "Chascomús"));
        Vertex<Ciudades> MarAzul = mapa.createVertex(new Ciudades(3, "Mar Azul"));
        Vertex<Ciudades> Pinamar = mapa.createVertex(new Ciudades(4, "Pinamar"));
        Vertex<Ciudades> Madariaga = mapa.createVertex(new Ciudades(1, "Madariaga"));
        Vertex<Ciudades> LaPlata = mapa.createVertex(new Ciudades(5, "La Plata"));
        Vertex<Ciudades> LasGaviotas = mapa.createVertex(new Ciudades(1, "Las Gaviotas"));
        Vertex<Ciudades> Querandi = mapa.createVertex(new Ciudades(1, "Querandi"));
        Vertex<Ciudades> Hudson = mapa.createVertex(new Ciudades(1, "Hudson"));

        mapa.connect(MarDelPlata, Pila);
        mapa.connect(Pila, MarDelPlata);
        mapa.connect(Pila, Dolores);
        mapa.connect(Dolores, Pila);
        mapa.connect(Dolores, Chascomus);
        mapa.connect(Chascomus, Dolores);

        mapa.connect(MarDelPlata, MarAzul);
        mapa.connect(MarAzul, MarDelPlata);
        mapa.connect(MarAzul, Pinamar);
        mapa.connect(Pinamar, MarAzul);
        mapa.connect(Pinamar, Madariaga);
        mapa.connect(Madariaga, Pinamar);
        mapa.connect(Dolores, Madariaga);
        mapa.connect(Madariaga, Dolores);
        mapa.connect(Madariaga, LaPlata);
        mapa.connect(LaPlata, Madariaga);
        mapa.connect(Chascomus, LaPlata);
        mapa.connect(LaPlata, Chascomus);

        mapa.connect(MarAzul, LasGaviotas);
        mapa.connect(LasGaviotas, MarAzul);
        mapa.connect(MarAzul, Querandi);
        mapa.connect(Querandi, MarAzul);
        mapa.connect(LaPlata, Hudson);
        mapa.connect(Hudson, LaPlata);

        Parcial p = new Parcial();
        System.out.println(p.resolver(mapa, "Mar Del Plata", 15));
    }
}
