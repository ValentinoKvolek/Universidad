package org.example;

import java.util.ArrayList;
import java.util.List;

public class ReporteDeConstruccion {

    private List<Pieza> piezas = new ArrayList<>();

    public void agregarPieza(Pieza p) {
        piezas.add(p);
    }

    public double volumenDeMaterial(String material) {
        double total = 0;
        for (Pieza p : piezas) {
            if (p.getMaterial().equalsIgnoreCase(material)) {
                total += p.getVolumen();
            }
        }
        return total;
    }

    public double superficieDeColor(String color) {
        double total = 0;
        for (Pieza p : piezas) {
            if (p.getColor().equalsIgnoreCase(color)) {
                total += p.getSuperficie();
            }
        }
        return total;
    }
}
