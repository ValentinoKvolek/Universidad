package main.java.parciales2.parcial4;

import java.util.ArrayList;
import java.util.List;

public class Camino {
    public List<String> camino;
    public int totalCuadras;

    public Camino() {
        this.camino = new ArrayList<>();
        this.totalCuadras = 0;
    }


    public Camino(Camino otro) {
        this.camino = new ArrayList<>(otro.camino);
        this.totalCuadras = otro.totalCuadras;
    }

    @Override
    public String toString() {
        return camino + "  (" + totalCuadras ;
    }
}
