package org.example;
import org.example.Cliente;
import org.example.Item;
import org.example.Plan;
import org.example.PlanGrupal;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.assertEquals;
import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;

public class ClienteTest {

    private Cliente clienteGrupal;
    private Cliente clienteIndividual;
    private Cliente clienteAntiguo;
    private Cliente clienteIndividualSinPenalizacion;

    @BeforeEach
    void setup() {

        Plan planGrupal = new PlanGrupal(3);      // admite hasta 3 IPs
        Plan planIndividual = new PlanIndividual(120);


        Item item1 = new Item("espada", 2, 300.0);
        Item item2 = new Item("arco", 1, 500.0);
        Item item3 = new Item("banana", 3, 400.0);
        Item item4 = new Item("escudo", 1, 700.0);
        Item item5 = new Item("poción", 5, 200.0);

        List<Item> itemsA = List.of(item1, item2);
        List<Item> itemsB = List.of(item3, item4);
        List<Item> itemsC = List.of(item5);
        List<Item> itemsD = List.of(item1, item3, item5);

        Actividad a1 = new ReproduccionVideo("IP1", LocalDate.of(2024, 5, 10), 40, 10);
        Actividad a2 = new SesionDeJuego("IP2", LocalDate.of(2024, 5, 11), 50, itemsA);
        Actividad a3 = new ReproduccionVideo("IP3", LocalDate.of(2024, 5, 12), 60, 8);
        Actividad a4 = new SesionDeJuego("IP4", LocalDate.of(2024, 5, 13), 30, itemsB);
        Actividad a5 = new SesionDeJuego("IP5", LocalDate.of(2024, 5, 14), 45, itemsD);

        List<Actividad> actividadesGrupal = new ArrayList<>();
        actividadesGrupal.add(a1);
        actividadesGrupal.add(a2);
        actividadesGrupal.add(a3);
        actividadesGrupal.add(a4);
        actividadesGrupal.add(a5);


        clienteGrupal = new Cliente(
                "Lucía",
                LocalDate.of(2018, 3, 15),   // alta hace más de 5 años (no exime de penalización todavía)
                planGrupal,
                actividadesGrupal
        );


        Actividad b1 = new ReproduccionVideo("IP10", LocalDate.of(2024, 5, 10), 40, 10);
        Actividad b2 = new SesionDeJuego("IP11", LocalDate.of(2024, 5, 12), 60, itemsC);

        List<Actividad> actividadesIndividual = new ArrayList<>();
        actividadesIndividual.add(b1);
        actividadesIndividual.add(b2);


        clienteIndividual = new Cliente(
                "Valentino",
                LocalDate.of(2022, 5, 10),
                planIndividual,
                actividadesIndividual
        );


        Plan planAntiguo = new PlanGrupal(3);


        Actividad act1 = new ReproduccionVideo("IP-A", LocalDate.of(2024, 5, 10), 30, 10);
        Actividad act2 = new SesionDeJuego("IP-B", LocalDate.of(2024, 5, 11), 45, List.of(new Item("espada", 1, 500.0)));
        Actividad act3 = new ReproduccionVideo("IP-C", LocalDate.of(2024, 5, 12), 60, 8);
        Actividad act4 = new SesionDeJuego("IP-D", LocalDate.of(2024, 5, 13), 40, List.of(new Item("arco", 2, 400.0)));

        List<Actividad> actsAntiguo = new ArrayList<>();
        actsAntiguo.add(act1);
        actsAntiguo.add(act2);
        actsAntiguo.add(act3);
        actsAntiguo.add(act4);

        clienteAntiguo = new Cliente(
                "Marta",
                LocalDate.of(2010, 3, 10),   // fechaAlta muy vieja → más de 10 años
                planAntiguo,
                actsAntiguo
        );

        Item espada = new Item("espada", 2, 300.0);
        Item arco = new Item("arco", 1, 500.0);
        Item banana = new Item("banana", 3, 400.0);
        Item escudo = new Item("escudo", 1, 700.0);
        List<Item> itemsBase = List.of(espada, arco, banana, escudo);

        List<Actividad> actsSofia = List.of(
                new SesionDeJuego("IP-Z", LocalDate.of(2024, 5, 11), 45, itemsBase)
        );
        clienteIndividualSinPenalizacion = new Cliente("Sofía", LocalDate.of(2023, 1, 5), planIndividual, actsSofia);
    }


    @Test
    void test_penalizacion_grupal(){
        double resultado = clienteGrupal.montoPenalizacion(LocalDate.of(2024,5,8), LocalDate.of(2024,5,20));
        assertEquals(resultado,1000);
    }

    @Test
    void test_penalizacion_grupal_antiguedad(){
        double resultado = clienteAntiguo.montoPenalizacion(LocalDate.of(2024,5,8), LocalDate.of(2024,5,20));
        assertEquals(resultado,0.0);
    }
    @Test
    void tets_penalizacion_individual(){
        double resultado = clienteIndividual.montoPenalizacion(LocalDate.of(2024,5,8), LocalDate.of(2024,5,20));
        assertEquals(resultado, 300.0);
    }

    @Test
    void test_sin_penalizacion(){
        double resultado = clienteIndividualSinPenalizacion.montoPenalizacion(LocalDate.of(2024,5,8), LocalDate.of(2024,5,20));
        assertEquals(resultado,0.0);
    }

}
