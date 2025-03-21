package tp1.ejercicio3;

public class Test {
    public static void main(String[] args) {
        Estudiante[] arrayE = new Estudiante[2];
        Profesor[] arrayP = new Profesor[3];

        arrayE[0] = new Estudiante("Juan", "Pérez", 101, "juan.perez@email.com", "Calle 123");
        arrayE[1] = new Estudiante("María", "Gómez", 102, "maria.gomez@email.com", "Avenida 456");

        arrayP[0] = new Profesor("Carlos", "López", 1, "carlos.lopez@email.com", "Matemáticas", "Facultad de Ciencias");
        arrayP[1] = new Profesor("Ana", "Martínez", 2, "ana.martinez@email.com", "Historia", "Facultad de Humanidades");
        arrayP[2] = new Profesor("Pedro", "Fernández", 3, "pedro.fernandez@email.com", "Física", "Facultad de Ingeniería");

        for (int i = 0;  i < arrayE.length; i++){
            System.out.println( arrayE[i].tusDatos());
        }
        for (int i = 0;  i < arrayP.length; i++){
            System.out.println( arrayP[i].tuDatos());
        }
    }
}
