package tema3;

public class ejer4prac3 {
    public static void main(String[] args) {
        // Instanciar un hotel con 5 habitaciones
        Hotel hotel = new Hotel(5);

        // Crear algunos clientes
        Cliente cliente1 = new Cliente("Juan Perez", 12345678, 30);
        Cliente cliente2 = new Cliente("Maria Gomez", 87654321, 25);

        // Asignar clientes a habitaciones
        hotel.agregarCliente(new Habitacion(), 0);
        hotel.agregarCliente(new Habitacion(), 2);

        // Mostrar el estado del hotel
        System.out.println("Estado del hotel antes del aumento de precios:");
        System.out.println(hotel);

        // Aumentar el precio de las habitaciones
        hotel.Aumentar(500);

        // Mostrar el estado del hotel después del aumento de precios
        System.out.println("Estado del hotel después del aumento de precios:");
        System.out.println(hotel);
    }
}
