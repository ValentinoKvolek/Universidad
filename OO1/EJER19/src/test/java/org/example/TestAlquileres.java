package org.example;

import org.example.Propiedad;
import org.example.Reserva;
import org.example.Usuario;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.time.LocalDate;
import java.util.ArrayList;

import static org.junit.jupiter.api.Assertions.*;

public class TestAlquileres {

    private Usuario usuario;
    private Propiedad propiedad1;
    private Reserva reserva1;
    private Reserva reserva2;
    private Reserva reserva3;

    @BeforeEach
    void setUp() {
        LocalDate desde = LocalDate.of(2025, 2, 21);
        LocalDate hasta = LocalDate.of(2025, 2, 28);
        LocalDate desde2 = LocalDate.of(2025, 3, 21);
        LocalDate hasta2 = LocalDate.of(2025, 3, 28);
        LocalDate desde3 = LocalDate.of(2025, 10, 18);
        LocalDate hasta3 = LocalDate.of(2025, 10, 25);

        reserva1 = new Reserva(desde, hasta);
        reserva2 = new Reserva(desde2, hasta2);
        reserva3 = new Reserva(desde3, hasta3);

        ArrayList<Reserva> reservas = new ArrayList<>();
        reservas.add(reserva1);
        reservas.add(reserva2);
        reservas.add(reserva3);

        propiedad1 = new Propiedad("J. P. Yerman", "Yoni House" ,300.0, reservas, "flexible");

        ArrayList<Propiedad> propiedades = new ArrayList<>();
        propiedades.add(propiedad1);

        usuario = new Usuario("Yoni", "Calle 60", 24241, propiedades);
    }

    @Test
    void crearReserva_valida_devuelveTrueYAgregaReserva() {
        LocalDate desde = LocalDate.of(2026, 2, 21);
        LocalDate hasta = LocalDate.of(2026, 2, 28);

        boolean resultado = usuario.crearReserva(desde, hasta, propiedad1);

        assertTrue(resultado, "Debería permitir crear una reserva válida");
    }

    @Test
    void crearReserva_superpuesta_devuelveFalse() {
        LocalDate desde = LocalDate.of(2025, 2, 22);
        LocalDate hasta = LocalDate.of(2025, 2, 27);

        boolean resultado = usuario.crearReserva(desde, hasta, propiedad1);

        assertFalse(resultado, "No debería permitir una reserva completamente superpuesta");
    }

    @Test
    void crearReserva_inicioAntes_devuelveFalse() {
        LocalDate desde = LocalDate.of(2025, 2, 19);
        LocalDate hasta = LocalDate.of(2025, 2, 27);

        boolean resultado = usuario.crearReserva(desde, hasta, propiedad1);

        assertFalse(resultado, "No debería permitir una reserva que empieza antes y termina dentro");

    }

    @Test
    void crearReserva_finDespues_devuelveFalse() {
        LocalDate desde = LocalDate.of(2025, 2, 22);
        LocalDate hasta = LocalDate.of(2025, 3, 28);

        boolean resultado = usuario.crearReserva(desde, hasta, propiedad1);

        assertFalse(resultado, "No debería permitir una reserva que termina después de otra existente");
    }

    @Test
    void estaDisponible_test_devuelveTrue(){
        LocalDate desde = LocalDate.of(2026, 2, 21);
        LocalDate hasta = LocalDate.of(2026, 2, 28);
        boolean resultado = propiedad1.estaDisponible(desde,hasta);
        assertTrue(resultado,"Deberia estar disponible esta fecha");
    }

    @Test
    void estaDisponible_fechaDentroDelRango(){
        LocalDate desde = LocalDate.of(2025, 2, 22);
        LocalDate hasta = LocalDate.of(2025, 2, 27);
        boolean resultado = propiedad1.estaDisponible(desde,hasta);
        assertFalse(resultado, "Deberia dar error ya que la fecha es superpuesta");
    }

    @Test
    void cancelarReserva_test_exitosa() {

        int cantidadInicial = propiedad1.getReservas().size();

        propiedad1.cancelarReserva(reserva1);

        assertEquals(cantidadInicial - 1, propiedad1.getReservas().size(),
                "Después de cancelar, debería haber una reserva menos");
        assertFalse(propiedad1.getReservas().contains(reserva1),
                "La reserva cancelada no debería seguir en la lista");

    }

    @Test
    void cancelarReserva_noExiste_test() {
        LocalDate desde = LocalDate.of(2025, 4, 1);
        LocalDate hasta = LocalDate.of(2025, 4, 10);
        Reserva reservaInexistente = new Reserva(desde, hasta);

        int cantidadInicial = propiedad1.getReservas().size();
        propiedad1.cancelarReserva(reservaInexistente);

        assertEquals(cantidadInicial, propiedad1.getReservas().size(),
                "No debería cambiar la cantidad si la reserva no existe");
    }

    @Test
    void cancelarReserva_yaComenzo() {

        int cantidadInicial = propiedad1.getReservas().size();

        propiedad1.cancelarReserva(reserva3);

        assertEquals(cantidadInicial, propiedad1.getReservas().size(),
                "No debería cambiar, no se puede cancelar algo que ya empezo)");
    }








}
