package org.example;
import java.time.LocalDate;
import java.util.List;

public class Usuario {
    private String nombre;
    private String direccion;
    private Integer dni;
    private List<Propiedad> propiedades;


    public Usuario(String nombre, String direccion, Integer dni, List<Propiedad> propiedades) {
        this.nombre = nombre;
        this.direccion = direccion;
        this.dni = dni;
        this.propiedades = propiedades;
    }

    public boolean crearReserva(LocalDate fechaI , LocalDate fechaF , Propiedad propiedad){
        return propiedad.crearReserva(fechaI, fechaF);
    }


    public double cancelarReserva(Propiedad propiedad, Reserva reserva, LocalDate fechaCancelacion) {
        return propiedad.cancelarReserva(reserva, fechaCancelacion);
    }

    public double ingresosTotales(DateLapse periodo) {
        double total = 0.0;
        for (Propiedad p : propiedades) {
            total += p.calcularIngresoEnPeriodo(periodo);
        }
        return total * 0.75;
    }

    public void cambiarPolitica(Propiedad propiedad, PoliticaCancelacion nuevaPolitica) {
        propiedad.setPolitica(nuevaPolitica);
    }

}
