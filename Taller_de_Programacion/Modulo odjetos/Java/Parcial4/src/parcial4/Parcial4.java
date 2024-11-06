/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package parcial4;

/**
 *
 * @author valen
 */
public class Parcial4 {

    /**
     * @param args the command line arguments
     */
    public static void main(String[] args) {
        
   
        
        
        AlumnoGrado nuevoAlumnoGrado = new AlumnoGrado("INFORMATICA", 12345678, "VALENTINO", 5);
        
        AlumnoDoctorado nuevoDoctorado = new AlumnoDoctorado("INGENIER0 EN QUIMICA", "UBA", 87654321, "Yoni", 6);

        Materia[] materiasGrado = {
            new Materia("Matemáticas", 7, "2023-10-05"),
            new Materia("Programación", 8, "2023-10-05"),
            new Materia("Física", 6, "2023-10-05"),
            new Materia("Historia", 4, "2023-10-05"),
            new Materia("TESIS", 9, "2023-10-05")
        };

        Materia[] materiasDoctorado = {
            new Materia("Investigación", 8, "2023-10-05"),
            new Materia("nashe", 9, "2023-10-05"),
            new Materia("Proyecto Avanzado", 7, "2023-10-05"),
            new Materia("Química Avanzada", 10, "2023-10-05"),
            new Materia("Bioquímica", 5, "2023-10-05")
        };

        for (Materia materia : materiasGrado) {
            nuevoAlumnoGrado.agregarMateriaAprobada(materia);
        }

        for (Materia materia : materiasDoctorado) {
            nuevoDoctorado.agregarMateriaAprobada(materia);
        }
        
        
        System.out.println(nuevoAlumnoGrado.toString());
        System.out.println(nuevoDoctorado.toString());
        
    }
}
