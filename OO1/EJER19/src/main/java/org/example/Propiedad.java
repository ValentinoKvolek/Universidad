package org.example;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;

public class Propiedad {
    private String direccion;
    private String nombreDescriptivo;
    private double precioPorNoche;
    private List<Reserva> reservas = new ArrayList<>();
    private PoliticaCancelacion politica;


    public Propiedad(String direccion, String nombreDescriptivo, double precioPorNoche, List<Reserva> reservas ) {
        this.direccion = direccion;
        this.nombreDescriptivo = nombreDescriptivo;
        this.precioPorNoche = precioPorNoche;
        this.reservas = reservas;

    }


    public boolean estaDisponible(LocalDate desde, LocalDate hasta) {
        for (Reserva r : reservas) {
            if (r.seSuperponeCon(desde, hasta)) {
                return false;
            }
        }
        return true;
    }

    public boolean crearReserva(LocalDate desde, LocalDate hasta) {
        if (estaDisponible(desde, hasta)) {
            Reserva nueva = new Reserva(desde, hasta);
            reservas.add(nueva);
            return  true;
        } else {
            return  false;
        }
    }

    public double calcularIngresoTotal() {
        double total = 0;
        for (Reserva r : reservas) {
            total += r.calcularPrecio(precioPorNoche);
        }
        return total;
    }

    public double cancelarReserva(Reserva reserva, LocalDate fechaCancelacion) {
        if (reserva.yaComenzo()) {
            throw new IllegalStateException("No se puede cancelar una reserva que ya comenzó");
        }
        double reembolso = politica.calcularReembolso(reserva, fechaCancelacion, precioPorNoche);
        reservas.remove(reserva);
        return reembolso;
    }

    public double calcularIngresoEnPeriodo(DateLapse periodo) {
        double total = 0.0;

        for (Reserva r : reservas) {
            if (r.estaDentroDePeriodo(periodo)) {
                total += r.calcularPrecio(precioPorNoche);
            }
        }
        return total;
    }

    public List<Reserva> getReservas() {
        return this.reservas;
    }

    public void setPolitica(PoliticaCancelacion nuevaPolitica) {
        this.politica = nuevaPolitica;
    }



}
