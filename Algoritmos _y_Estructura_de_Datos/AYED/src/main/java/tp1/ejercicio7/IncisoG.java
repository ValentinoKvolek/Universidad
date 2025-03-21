package tp1.ejercicio7;

import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class IncisoG {

    public List<Integer> calcularSucesion (int n) {

        List<Integer> sucesion = new ArrayList<>();
        calcularSucesionRecursivo(n, sucesion);
        return sucesion;
    }

    public void calcularSucesionRecursivo(int num, List<Integer> sucesion){

        sucesion.add(num);

        if(num == 1) {
            return; //llegue al caso base.
        }

        if(num % 2 == 0 ){   //par
            calcularSucesionRecursivo(num/2,sucesion);
        }
        else{ //impar
            calcularSucesionRecursivo(num*3 +1,sucesion);
        }
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
