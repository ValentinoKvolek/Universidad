package tp1.ejercicio7;

import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class IncisoG {

    public List<Integer> calcularSucesion (int n) {

        List<Integer> sucesion = new ArrayList<>();

        if (n!= 1) {

            if (n % 2 == 0) { //par
                n = n / 2;
                ;
            } else { //impar
                n = n * 3 + 1;
            }
            sucesion.add(n);
            sucesion.addAll(calcularSucesion(n));
        }
        return sucesion;
    }

    public static void main(String[] args) {

        Scanner scanner = new Scanner(System.in);

        IncisoG e = new IncisoG();

        System.out.println("ingrese el numero");

        int n = scanner.nextInt();

        List<Integer> resultado = e.calcularSucesion(n);

        System.out.println("Sucesión para " + n + ": " + resultado);

    }


}
