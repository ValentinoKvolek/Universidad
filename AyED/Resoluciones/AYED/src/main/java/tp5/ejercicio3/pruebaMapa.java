/*
package main.java.tp5.ejercicio3;

import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;
import main.java.tp5.ejercicio3.Mapa;

import java.util.List;

public class pruebaMapa {

    public static void main(String[] args) {

        AdjListGraph<String> ciudades = new AdjListGraph<>();

        Vertex<String> bsas = ciudades.createVertex("Buenos Aires");
        Vertex<String> rosario = ciudades.createVertex("Rosario");
        Vertex<String> cordoba = ciudades.createVertex("Córdoba");
        Vertex<String> mendoza = ciudades.createVertex("Mendoza");

        ciudades.connect(bsas, rosario, 300);
        ciudades.connect(bsas, cordoba, 700);
        ciudades.connect(rosario, cordoba, 400);
        ciudades.connect(cordoba, mendoza, 600);
        ciudades.connect(rosario, mendoza, 900);

        // Si tenés una clase que implementa caminoSinCargarCombustible
        Mapa mapa = new Mapa(ciudades);
        List<String> camino = mapa.caminoSinCargarCombustible("Buenos Aires", "Mendoza", 1500);
        System.out.println(camino); //[Buenos Aires, Rosario, Córdoba, Mendoza]
    }
}
 TEST EJER4*/

package main.java.tp5.ejercicio3;

import main.java.tp5.ejercicio1.Graph;
import main.java.tp5.ejercicio1.Vertex;
import main.java.tp5.ejercicio1.adjList.AdjListGraph;

import java.util.List;

public class pruebaMapa {

    public static void main(String[] args) {
        // Crear el grafo
        AdjListGraph<String> ciudades = new AdjListGraph<>();

        Vertex<String> a = ciudades.createVertex("A");
        Vertex<String> b = ciudades.createVertex("B");
        Vertex<String> c = ciudades.createVertex("C");
        Vertex<String> d = ciudades.createVertex("D");

        ciudades.connect(a, b, 300);  // A → B
        ciudades.connect(b, c, 400);  // B → C
        ciudades.connect(a, d, 600);  // A → D
        ciudades.connect(d, c, 200);  // D → C

        // Crear el objeto que contiene el método caminoConMenorCargaDeCombustible
        Mapa mapa = new Mapa(ciudades);  // Suponiendo que tu clase se llama Mapa y recibe el grafo

        // Probar con un tanque de 700
        List<String> camino = mapa.caminoConMenorCargaDeCombustible("A", "C", 700);


        //Camino con menor cantidad de cargas:
        //[A, B, C]

        System.out.println("Camino con menor cantidad de cargas:");
        System.out.println(camino);
    }
}

