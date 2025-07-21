package main.java.parciales2.parcial1;

import main.java.tp1.ejercicio8.Queue;
import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;


import java.util.LinkedList;
import java.util.List;

public class Parcial {

    public List<Usuario> invitacionMasterClass (Graph<String> red, String usuario, int distancia, int limite){

        List<Usuario> invitados = new LinkedList<Usuario>();

        boolean[] visitados = new boolean[red.getSize()];

        boolean encontre = false;

        if(!red.isEmpty()){
            Vertex<String> origen = red.search(usuario);

            if (origen != null){
                encontre = recorridoBfs(origen.getPosition(),invitados,red, visitados, distancia,limite);
            }
        }
        if(encontre){
            return invitados;
        }
        return new LinkedList<Usuario>();
    }

    private boolean recorridoBfs(int i, List<Usuario> invitados,Graph<String> red, boolean[] visitados, int distancia, int limite){

        visitados[i] = true;

        Vertex<String> v = red.getVertex(i);
        Queue<Vertex<String>> queue = new Queue<Vertex<String>>();
        queue.enqueue(v);
        queue.enqueue(null);
        int nivel= 0;

        while(!queue.isEmpty()){

            Vertex<String> aux = queue.dequeue();

            if(aux !=null){

                if(nivel <= distancia && nivel > 0 ){
                    if(invitados.size() < limite){
                        Usuario nuevo = new Usuario(aux.getData(), nivel);
                        invitados.add(nuevo);
                    }else {return  true;}
                }

                List<Edge<String>> edges = red.getEdges(aux);
                for (Edge<String> e : edges){
                    int j = e.getTarget().getPosition();
                    if(!visitados[j]){
                        visitados[j]=true;
                        queue.enqueue(e.getTarget());
                    }
                }

            }else {
                nivel++;
                queue.enqueue(null);
            }
        }

        return false;
    }

    public static void main(String[] args) {
        Graph<String> grafo = new AdjListGraph<String>();
        Vertex<String> v1 = grafo.createVertex("Lionel");
        Vertex<String> v2 = grafo.createVertex("Rodrigo");
        Vertex<String> v3 = grafo.createVertex("Ángel");
        Vertex<String> v4 = grafo.createVertex("Emiliano");
        Vertex<String> v5 = grafo.createVertex("Julián");
        Vertex<String> v6 = grafo.createVertex("Diego");
        Vertex<String> v7 = grafo.createVertex("Lautaro");
        Vertex<String> v8 = grafo.createVertex("Enzo");

        grafo.connect(v1, v2);
        grafo.connect(v2, v1);

        grafo.connect(v1, v3);
        grafo.connect(v3, v1);

        grafo.connect(v2, v4);
        grafo.connect(v4, v2);

        grafo.connect(v2, v5);
        grafo.connect(v5, v2);

        grafo.connect(v3, v5);
        grafo.connect(v5, v3);

        grafo.connect(v3, v6);
        grafo.connect(v6, v3);

        grafo.connect(v6, v7);
        grafo.connect(v7, v6);

        grafo.connect(v5, v7);
        grafo.connect(v7, v5);

        grafo.connect(v6, v8);
        grafo.connect(v8, v6);

        grafo.connect(v4, v8);
        grafo.connect(v8, v4);

        grafo.connect(v4, v7);
        grafo.connect(v7, v4);

        Parcial p = new Parcial();

        System.out.println(p.invitacionMasterClass(grafo, "Lionel", 2, 4));
        System.out.println(p.invitacionMasterClass(grafo, "Juancito", 1, 2));
    }
}
