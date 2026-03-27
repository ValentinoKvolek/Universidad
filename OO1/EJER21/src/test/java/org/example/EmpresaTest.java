package org.example;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.time.LocalDate;

import static org.junit.jupiter.api.Assertions.*;

public class EmpresaTest {

    private Empresa empresa;
    private PersonaFisica persona;
    private ClienteCorporativo corporativo;

    private EnvioLocal envioLocalNormal;
    private EnvioLocal envioLocalRapido;
    private EnvioInterUrbano envioInterurbano;
    private EnvioInternacional envioInternacional;

    @BeforeEach
    public void setUp() {
        empresa = new Empresa();
        persona = new PersonaFisica();
        corporativo = new ClienteCorporativo();

        LocalDate hoy = LocalDate.now();
        LocalDate ayer = hoy.minusDays(1);

        envioLocalNormal = new EnvioLocal(ayer, "La Plata", "La Plata", 500, false);
        envioLocalRapido = new EnvioLocal(hoy, "La Plata", "La Plata", 500, true);
        envioInterurbano = new EnvioInterUrbano(hoy, "La Plata", "Mar del Plata", 300, false, 120);
        envioInternacional = new EnvioInternacional(hoy, "La Plata", "Madrid", 800, true);

        empresa.agregarEnvio(persona, envioLocalNormal);
        empresa.agregarEnvio(persona, envioInterurbano);
        empresa.agregarEnvio(corporativo, envioInternacional);
    }

    @Test
    public void testAgregarEnvioACliente() {
        assertEquals(2, persona.envios.size());
        assertEquals(1, corporativo.envios.size());
    }

    @Test
    public void testEnvioLocalCostoEstandarYRapido() {
        assertEquals(1000, envioLocalNormal.costoEnvio());
        assertEquals(1500, envioLocalRapido.costoEnvio());
    }

    @Test
    public void testEnvioInterurbanoCostoPorDistancia() {
        // 120 km → $25 * peso
        assertEquals(25 * 300, envioInterurbano.costoEnvio());
    }

    @Test
    public void testEnvioInternacionalCostoConRapido() {
        // Peso < 1000 → $10 * peso + 5000 + 800
        assertEquals(5000 + (10 * 800) + 800, envioInternacional.costoEnvio());
    }

    @Test
    public void testMontoTotalPersonaConDescuento() {
        LocalDate inicio = LocalDate.now().minusDays(5);
        LocalDate fin = LocalDate.now().plusDays(1);

        double montoSinDescuento = envioLocalNormal.costoEnvio() + envioInterurbano.costoEnvio();
        double esperado = montoSinDescuento * 0.10;

        assertEquals(esperado, empresa.montoAPagar(persona, inicio, fin));
    }

    @Test
    public void testMontoTotalCorporativoSinDescuento() {
        LocalDate inicio = LocalDate.now().minusDays(5);
        LocalDate fin = LocalDate.now().plusDays(1);

        double esperado = envioInternacional.costoEnvio();
        assertEquals(esperado, empresa.montoAPagar(corporativo, inicio, fin));
    }

    @Test
    public void testEnvioFueraDePeriodoNoCuenta() {
        LocalDate inicio = LocalDate.now().minusDays(10);
        LocalDate fin = LocalDate.now().minusDays(5);
        assertEquals(0, empresa.montoAPagar(persona, inicio, fin));
    }
}
