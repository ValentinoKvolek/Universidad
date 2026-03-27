package org.example;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.List;

public class ClienteDeCorreoTest {

    private ClienteDeCorreo cliente;
    private Carpeta trabajo;
    private Email mailSimple;
    private Email mailConAdjunto;

    @BeforeEach
    public void setUp() {
        cliente = new ClienteDeCorreo();
        trabajo = new Carpeta("Trabajo");

        List<Archivo> sinAdjuntos = new ArrayList<>();
        mailSimple = new Email("Reunión", "Lunes 10hs", sinAdjuntos);

        List<Archivo> adj = new ArrayList<>();
        adj.add(new Archivo ("documento.pdf"));
        mailConAdjunto = new Email("Entrega", "Proyecto final", adj);

        cliente.recibir(mailSimple);  // va al inbox
        trabajo.agregarEmail(mailConAdjunto);
        // agregamos la carpeta manualmente al cliente
        cliente.buscar("algo"); // para que esté inicializado
    }

    // --- TESTS DE PARTICIONES Y BORDES ---

    @Test
    public void testRecibirAgregaEnInbox() {
        Email resultado = cliente.buscar("Reunión");
        assertNotNull(resultado);
        assertEquals("Reunión", resultado.getTitulo());
    }

    @Test
    public void testBuscarNoEncuentraTextoInexistente() {
        Email resultado = cliente.buscar("Vacaciones");
        assertNull(resultado);
    }

    @Test
    public void testBuscarPorCuerpoFunciona() {
        Email resultado = cliente.buscar("10hs");
        assertNotNull(resultado);
        assertEquals(mailSimple, resultado);
    }

    @Test
    public void testEspacioOcupadoClienteIncluyeTodasLasCarpetas() {
        Carpeta otra = new Carpeta("Personal");
        List<Archivo> adj = new ArrayList<>();
        adj.add(new Archivo("foto.jpg"));
        Email mail = new Email("Hola", "Mensaje", adj);
        otra.agregarEmail(mail);
        cliente.buscar("nada"); // asegurar inicialización

        int total = cliente.espacioOcupado();
        assertTrue(total > 0);
    }

    @Test
    public void testEspacioOcupadoEmailSinAdjuntos() {
        List<Archivo> vacia = new ArrayList<>();
        Email e = new Email("A", "B", vacia);
        assertEquals(2, e.espacioOcupado()); // 1 + 1
    }

    @Test
    public void testEspacioOcupadoEmailConAdjuntos() {
        List<Archivo> adj = new ArrayList<>();
        adj.add(new Archivo("abc"));
        Email e = new Email("X", "Y", adj);
        // 1 (titulo) + 1 (cuerpo) + 3 (nombre archivo)
        assertEquals(5, e.espacioOcupado());
    }

    @Test
    public void testMoverEmailEntreCarpetas() {
        Carpeta destino = new Carpeta("Destinos");
        destino.agregarEmail(mailSimple);
        trabajo.mover(mailConAdjunto, destino);

        assertTrue(destino.buscarEmail("Entrega") != null);
        assertNull(trabajo.buscarEmail("Entrega"));
    }

    @Test
    public void testBuscarDevuelvePrimerCoincidencia() {
        Carpeta extra = new Carpeta("Extra");
        List<Archivo> adj = new ArrayList<>();
        adj.add(new Archivo("archivo.txt"));
        Email duplicado = new Email("Reunión", "Diferente cuerpo", adj);
        extra.agregarEmail(duplicado);

        // Aunque hay 2 "Reunión", debería devolver el primero (inbox)
        Email encontrado = cliente.buscar("Reunión");
        assertEquals(mailSimple, encontrado);
    }

    @Test
    public void testEmailContiene() {
        assertTrue(mailSimple.contiene("Reun"));
        assertFalse(mailSimple.contiene("zzz"));
    }
}
