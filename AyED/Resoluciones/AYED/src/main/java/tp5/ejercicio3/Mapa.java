package main.java.tp5.ejercicio3;

import main.java.tp5.ejercicio1.Edge;
import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;

import java.util.ArrayList;
import java.util.List;
import java.util.Vector;

public class Mapa {

    Graph<String> mapaCiudades;

    public Mapa(Graph<String> mapaCiudades) {
        this.mapaCiudades = mapaCiudades;
    }

    /*
      1. devolverCamino (String ciudad1, String ciudad2): List<String> Retorna la lista de ciudades que se deben atravesar para ir de ciudad1 a ciudad2
       en caso de que se pueda llegar, si no retorna la lista vacía. (Sin tener en cuenta el combustible).
     */
    public List<String> devolverCamino(String ciudad1, String ciudad2) {

        List<String> camino = new ArrayList<String>();

        if (ciudad1 == null || ciudad2 == null) {
            return camino;
        }

        Vertex<String> origen = null;
        Vertex<String> destino = null;

        for (Vertex<String> v : mapaCiudades.getVertices()) {
            if (v.getData().equals(ciudad1)) {
                origen = v;
            }
            if (v.getData().equals(ciudad2)) {
                destino = v;
            }
        }

        if (origen == null || destino == null) {
            return camino; //si no existen, no hay camino posible
        }

        boolean[] marca = new boolean[mapaCiudades.getSize()];


        boolean encontrado = dfs(origen.getPosition(), destino.getPosition(), mapaCiudades, marca, camino);

        if (encontrado) {
            return camino;
        } else {
            return new ArrayList<>();
        }
    }

    private boolean dfs(int i, int f, Graph<String> grafo, boolean[] visitados, List<String> camino) {

        visitados[i] = true;

        Vertex<String> v = grafo.getVertex(i);

        camino.add(v.getData());

        if (v.getData().equals(grafo.getVertex(f).getData())) {

            return true;

        }

        List<Edge<String>> adyacentes = grafo.getEdges(v);

        for (Edge<String> e : adyacentes) {
            int j = e.getTarget().getPosition();

            if (!visitados[j]) {

                if (dfs(j, f, grafo, visitados, camino)) {
                    return true; // si encontre el camino retorno true.
                }

            }
        }

        camino.remove(camino.size() - 1); // si no backtracking

        return false;
    }
    /*
    2. devolverCaminoExceptuando (String ciudad1, String ciudad2, List<String> ciudades): List<String> Retorna la lista de ciudades que forman un camino desde ciudad1 a ciudad2,
     sin pasar por las ciudades que están contenidas en la lista ciudades pasada por parámetro,si no existe camino retorna la lista vacía. (Sin tener en cuenta el combustible).
    */

    public List<String> devolverCaminoExeptuando(String ciudad1, String ciudad2, List<String> ciudades) {

        List<String> camino = new ArrayList<String>();

        if (ciudad1 == null || ciudad2 == null) {
            return camino;
        }

        Vertex<String> origen = this.mapaCiudades.search(ciudad1);
        Vertex<String> destino = this.mapaCiudades.search(ciudad2);

        if (origen == null || destino == null) {
            return camino; //si no existen, no hay camino posible
        }

        boolean[] marca = new boolean[mapaCiudades.getSize()];

        boolean encontrado = dfsExeptuando(origen.getPosition(), destino.getPosition(), mapaCiudades, marca, camino, ciudades);

        if (encontrado) {
            return camino;
        } else {
            return new ArrayList<>();
        }

    }

    private boolean dfsExeptuando(int i, int f, Graph<String> grafo, boolean[] visitados, List<String> camino, List<String> ciudades) {

        visitados[i] = true;
        Vertex<String> v = grafo.getVertex(i);
        if (ciudades.contains(v.getData())) {
            return false;
        }
        camino.add(v.getData());
        if (v.getData().equals(grafo.getVertex(f).getData())) {
            return true;
        }
        List<Edge<String>> adyacentes = grafo.getEdges(v);

        for (Edge<String> e : adyacentes) {
            int j = e.getTarget().getPosition();
            if (!visitados[j]) {
                if (dfsExeptuando(j, f, mapaCiudades, visitados, camino, ciudades)) {
                    return true;
                }
            }
        }
        camino.remove(camino.size() - 1);
        return false;
    }

    /*
    3. caminoMasCorto(String ciudad1, String ciudad2): List<String> Retorna la lista de ciudades que forman el camino más corto para llegar de ciudad1 a ciudad2,
     si no existe camino retorna la lista vacía. (Las rutas poseen la distancia).
    */


    public List<String> caminoMasCorto(String ciudad1, String ciudad2) {

        List<String> caminoMin = new ArrayList<>();

        boolean[] visitados = new boolean[mapaCiudades.getSize()];

        if (!this.mapaCiudades.isEmpty()) {

            Vertex<String> origen = this.mapaCiudades.search(ciudad1);
            Vertex<String> destino = this.mapaCiudades.search(ciudad2);

            if (origen != null || destino != null) {
                caminoMasCortoDfs(origen.getPosition(), destino.getPosition(), mapaCiudades, visitados, new ArrayList<>(), caminoMin, Integer.MAX_VALUE, 0);
            }
        }

        return caminoMin;

    }

    private int caminoMasCortoDfs(int i, int f, Graph<String> grafo, boolean[] visitados, List<String> caminoAct, List<String> caminoMin, int min, int distancia) {

        visitados[i] = true;

        Vertex<String> v = grafo.getVertex(i);

        caminoAct.add(v.getData());

        if (v.getData().equals(grafo.getVertex(f).getData()) && distancia < min) {

            caminoMin.removeAll(caminoMin);
            caminoMin.addAll(caminoAct);
            min = distancia;

        }

        List<Edge<String>> adyacentes = grafo.getEdges(v);

        for (Edge<String> e : adyacentes) {
            int j = e.getTarget().getPosition();
            if (!visitados[j]) {
                distancia = distancia + e.getWeight();
                min = caminoMasCortoDfs(j, f, grafo, visitados, caminoAct, caminoMin, min, distancia);
            }
        }

        caminoAct.remove(caminoAct.size() - 1);
        visitados[i] = false;
        return min;
    }


    /*4. caminoSinCargarCombustible(String ciudad1, String ciudad2, int tanqueAuto): List<String> Retorna la lista de ciudades que forman un camino para llegar de ciudad1 a ciudad2.
    El auto no debe quedarse sin combustible y no puede cargar. Si no existe camino retorna la lista vacía.*/

    public List<String> caminoSinCargarCombustible (String ciudad1, String ciudad2, int tanqueAuto) {

        List<String> camino = new ArrayList<>();

        boolean [] visitados = new boolean[mapaCiudades.getSize()];

        boolean encontrado=  false;

        if (!mapaCiudades.isEmpty()){
            Vertex<String> origen = this.mapaCiudades.search(ciudad1);
            Vertex<String> destino = this.mapaCiudades.search(ciudad2);

            if (origen != null || destino != null) {
                encontrado = caminoSinCargarCombustible(origen.getPosition(), destino.getPosition(),mapaCiudades, visitados, camino,tanqueAuto);
            }
        }

        if(encontrado) {
            return camino;
        }
        return new ArrayList<>();
    }

    private boolean caminoSinCargarCombustible(int i, int f, Graph<String> grafo ,boolean [] visitados, List<String> camino, int combustible) {

        visitados[i] = true;
        Vertex<String> v = grafo.getVertex(i);
        camino.add(v.getData());
        if (v.getData().equals(grafo.getVertex(f).getData())) {
            return true;
        }

        List<Edge<String>> adyacentes = grafo.getEdges(v);

        for (Edge<String> e : adyacentes) {
            int j = e.getTarget().getPosition();
            if (!visitados[j]) {
                int restante = combustible - e.getWeight();
                if (restante >= 0) {
                    if (caminoSinCargarCombustible(j, f, grafo, visitados, camino, restante)) {
                        return true;
                    }
                }
            }
        }

        camino.remove(camino.size() - 1);
        visitados[i] = false;
        return false;
    }

    /*5. caminoConMenorCargaDeCombustible (String ciudad1, String ciudad2, int tanqueAuto): List<String> Retorna la lista de ciudades que
     forman un camino para llegar de ciudad1 a ciudad2 teniendo en cuenta que el auto debe cargar la menor cantidad de veces.
     El auto no se debe quedar sin combustible en medio de una ruta,
     además puede completar su tanque al llegar a cualquier ciudad. Si no existe camino retorna la lista vacía.*/


    public List<String> caminoConMenorCargaDeCombustible (String ciudad1, String ciudad2, int tanqueAuto) {

        List<String> caminoMin = new ArrayList<String>();
        int llenar = tanqueAuto;
        boolean [] visitado = new boolean[mapaCiudades.getSize()];

        if (!this.mapaCiudades.isEmpty()){

            Vertex<String> origen = this.mapaCiudades.search(ciudad1);
            Vertex<String> destino = this.mapaCiudades.search(ciudad2);

            if(origen !=null && destino !=null){
                caminoConMenorCargaDeCombustible(origen.getPosition(), destino.getPosition(),mapaCiudades, visitado, caminoMin, new ArrayList<String>(), Integer.MAX_VALUE, 0, tanqueAuto, llenar);
            }

        }
        return caminoMin;
    }

    private int caminoConMenorCargaDeCombustible(int i, int f, Graph<String> grafo, boolean[] visitados, List<String> caminoMin, List<String> caminoAct, int min, int cargas, int combustible, int llenar){

        visitados[i] = true;

        Vertex<String> v = grafo.getVertex(i);

        caminoAct.add(v.getData());

        if(v.getData().equals(grafo.getVertex(f).getData()) && cargas < min){
            caminoMin.removeAll(caminoMin);
            caminoMin.addAll(caminoAct);
            min = cargas;
        }

        List<Edge<String>> edges = grafo.getEdges(v);

        for (Edge<String> e : edges){

            int j = e.getTarget().getPosition();

            if(!visitados[j]){

                int restante = combustible - e.getWeight();

                if(restante>=0){

                    //si todavia tengo combustile y ademas ya le reste la distancia que hay hasta el otro nodo y no me da 0 puedo llegar sin cargar.
                    min = caminoConMenorCargaDeCombustible(j,f,grafo,visitados,caminoMin,caminoAct,min,cargas,restante,llenar);
                }else
                {
                    restante = llenar - e.getWeight();
                    cargas ++;
                    min = caminoConMenorCargaDeCombustible(j,f,grafo,visitados,caminoMin,caminoAct,min,cargas,restante,llenar);
                }
            }
        }
        caminoAct.remove(caminoAct.size()-1);
        visitados[i]=  false;
        return min;
    }

}
