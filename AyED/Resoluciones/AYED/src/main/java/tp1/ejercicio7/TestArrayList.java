package tp1.ejercicio7;

import tp1.ejercicio3.Estudiante;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

public class  TestArrayList {

    public static void main(String[] args) {

    }

    public static void main(int[] args) {


        ArrayList<Integer> ints = new ArrayList<Integer>();

        for (int i = 0; i < args.length; i++) {
            ints.add(Integer.valueOf(args[i]));
        }

        for (Integer i: ints ){
            System.out.print(i+ " ");
        }

        System.out.println("inciso c : ");

        incisoD();

        System.out.println(esCapicua(ints));

    }

    public static void incisoD(){

        Scanner scanner = new Scanner(System.in);

        //d_1
        Estudiante[] estudiantes = new Estudiante[3];

        estudiantes[0] = new Estudiante("Juan", "Pérez", 101, "juan.perez@email.com", "Calle 123");
        estudiantes[1] = new Estudiante("María", "Gómez", 102, "maria.gomez@email.com", "Avenida 456");
        estudiantes[2] = new Estudiante("Santiago", "Zamudio", 102, "santi.gomez@email.com", "asd 456");

        ArrayList<Estudiante> listaEstudiantes = new ArrayList<Estudiante>(List.of(estudiantes));

        //d_2

        ArrayList<Estudiante> listaEstudiantes2 = new ArrayList<Estudiante>();

        for (int i = 0; i < listaEstudiantes.size(); i++) {
            listaEstudiantes2.add(listaEstudiantes.get(i));
        }

        for (Estudiante i: listaEstudiantes ){
            System.out.println(i.tusDatos()+ " ");
        }

        System.out.println( "la copia de mi arrayList es : ");

        for (Estudiante i: listaEstudiantes2 ){
            System.out.println(i.tusDatos()+ " ");
        }

        //d_3
        listaEstudiantes2.get(1).setDireccion("Diagonal 73");

        for (Estudiante i: listaEstudiantes ){
            System.out.println(i.tusDatos()+" ");
        }
        System.out.println( "la copia de mi arrayList es : ");
        for (Estudiante i: listaEstudiantes2 ){
            System.out.println(i.tusDatos()+"\n");
        }

        //E
        boolean  existe = false;

        Estudiante nuevoEstudiante = new Estudiante();

        System.out.println("ingrese el nombre del estudiante: ");
        String nombre = scanner.nextLine();
        nuevoEstudiante.setNombre(nombre);

        System.out.println("ingrese el apellido del estudiante");
        String apellido = scanner.nextLine();
        nuevoEstudiante.setApellido(apellido);

        for(Estudiante i: listaEstudiantes2 ) {
            if(i.getNombre().equals(nuevoEstudiante.getNombre()) && i.getApellido().equals(nuevoEstudiante.getApellido())){
                existe =  true;
            }
        }
        if (!existe){

            System.out.println("ingrese la comision");
            int comision = scanner.nextInt();
            nuevoEstudiante.setComision(comision);
            scanner.nextLine(); // Consumir el salto de línea

            System.out.println("Ingrese el email:" );
            String email = scanner.nextLine();
            nuevoEstudiante.setEmail(email);

            System.out.println("ingrese la direccion: ");
            String direc = scanner.nextLine();
            nuevoEstudiante.setDireccion(direc);

            listaEstudiantes2.add(nuevoEstudiante);

            System.out.println("lista con el estudiante agredado");

            for (Estudiante i: listaEstudiantes2 ){
                System.out.println(i.tusDatos()+" ");
            }

        } else{
            System.out.println("el estudiante ya existe");
        }

    }

    public static boolean esCapicua(ArrayList<Integer> lista){
        int inicio = 0;
        int fin = lista.size()-1;
        while (inicio<fin){
            if(!lista.get(inicio).equals(lista.get(fin))){
                return false;
            }
            inicio++;
            fin--;
        }
        return true;
    }
}
