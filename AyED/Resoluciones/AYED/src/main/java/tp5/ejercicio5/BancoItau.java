package main.java.tp5.ejercicio5;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;

import java.util.ArrayList;
import java.util.List;


public class BancoItau {


    public List<Persona> listaJubilados(Graph<Persona> grafo, Persona empleado, int gradoSeparacion){

        List<Persona> jubilados = new ArrayList<>();

        boolean[] visitados = new boolean[ grafo.getSize()];

        if(!grafo.isEmpty()) {
            Vertex<Persona> origen = grafo.search(empleado);

            if (origen != null) {
                this.listaJubilados(origen.getPosition(), grafo, visitados, jubilados, gradoSeparacion, 0);
            }

            return jubilados;
        }

        return new ArrayList<>();

    }

    private void listaJubilados (int i, Graph<Persona> grafo ,boolean[] visitados, List<Persona> jubilados, int gradoSeparacion , int distancia ){

        Queue<Vertex<Persona>> queue = new Queue<Vertex<Persona>>();
        queue.enqueue(grafo.getVertex(i));
        queue.enqueue(null); //marca de nivel
        visitados[i]= true;

        while(!queue.isEmpty()){

            Vertex<Persona> v = queue.dequeue();

            if(v != null) { //como no es la marca de nivel

                if (v.getData().jubilado && distancia < gradoSeparacion && jubilados.size() < 40) {
                    jubilados.add(v.getData());
                    if (jubilados.size() == 40) return;
                }

                List<Edge<Persona>> edges = grafo.getEdges(v);

                for (Edge<Persona> e : edges){
                    int j = e.getTarget().getPosition();
                    if(! visitados[j]){
                        visitados[j] = true;
                        queue.enqueue(e.getTarget());
                    }
                }
            }else {
                if(!queue.isEmpty()){

                    distancia ++;

                    if(distancia >= gradoSeparacion){
                        return;
                    }

                    queue.enqueue(null);
                }
                else {
                    return;
                }
            }
        }
    }

    public static void main(String[] args) {

        Graph<Persona> grafo = new AdjListGraph<>();


        Vertex<Persona> v1 = grafo.createVertex(new Persona(false, "Matias", "AAA")); // empleado
        Vertex<Persona> v2 = grafo.createVertex(new Persona(true, "Julian", "BBB"));  // jubilado
        Vertex<Persona> v3 = grafo.createVertex(new Persona(true, "Francisco", "CCC")); // jubilado
        Vertex<Persona> v4 = grafo.createVertex(new Persona(false, "Valentin", "DDD")); // empleado
        Vertex<Persona> v5 = grafo.createVertex(new Persona(true, "Omar", "EEE"));     // jubilado
        Vertex<Persona> v6 = grafo.createVertex(new Persona(false, "Rosana", "FFF"));  // empleado
        Vertex<Persona> v7 = grafo.createVertex(new Persona(true, "Maria", "GGG"));    // jubilado
        Vertex<Persona> v8 = grafo.createVertex(new Persona(true, "Sandra", "HHH"));   // jubilado
        Vertex<Persona> v9 = grafo.createVertex(new Persona(true, "Matheo", "III"));   // jubilado

        // conexiones bidireccionales
        grafo.connect(v1, v2); grafo.connect(v2, v1);
        grafo.connect(v2, v3); grafo.connect(v3, v2);
        grafo.connect(v1, v9); grafo.connect(v9, v1);
        grafo.connect(v9, v8); grafo.connect(v8, v9);
        grafo.connect(v1, v4); grafo.connect(v4, v1);
        grafo.connect(v1, v6); grafo.connect(v6, v1);
        grafo.connect(v4, v5); grafo.connect(v5, v4);
        grafo.connect(v5, v7); grafo.connect(v7, v5);

        BancoItau banco = new BancoItau();

        Persona empleado = v1.getData(); // Matias
        int gradoSeparacion = 3;

        List<Persona> resultado = banco.listaJubilados(grafo, empleado, gradoSeparacion);

        System.out.println("Jubilados alcanzados por Matias (grado ≤ " + gradoSeparacion + "):");
        for (Persona p : resultado) {
            System.out.println("- " + p.getNombre() + " (" + p.getDomicilio() + ")");
        }
    }
}
