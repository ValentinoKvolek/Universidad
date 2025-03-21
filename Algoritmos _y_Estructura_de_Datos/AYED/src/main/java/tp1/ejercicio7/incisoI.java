package tp1.ejercicio7;

import java.util.Collections;
import java.util.LinkedList;
import java.util.List;

public class incisoI {

    public static void main(String[] args) {

        LinkedList<Integer> lista = new LinkedList<>();
        lista.add(1);
        lista.add(4);
        lista.add(8);
        LinkedList<Integer> lista2 = new LinkedList<>();
        lista2.add(2);
        lista2.add(5);
        lista2.add(9);

        int resultado = sumarLinkedList(0, lista, lista.size());

        System.out.println("Suma total: " + resultado);

        System.out.println(combinar(lista,lista2));



    }

    public static int sumarLinkedList(int r, LinkedList<Integer> lista, int size) {
        if (size == 0) {
            return r;
        }

        r += lista.get(size - 1);
        return sumarLinkedList(r, lista, size - 1);
    }
    public static LinkedList combinar (LinkedList<Integer> lista,LinkedList<Integer> lista2){
        lista.addAll(lista2);
        Collections.sort(lista);
        return lista;
    }
}
