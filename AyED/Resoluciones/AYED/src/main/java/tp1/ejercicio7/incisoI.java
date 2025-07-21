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

    public static LinkedList combinar (LinkedList<Integer> lista,LinkedList<Integer> lista2) {

        LinkedList<Integer> newList = new LinkedList<>();
        while (!lista.isEmpty() && !lista2.isEmpty()) {
            if (lista.getFirst() < lista2.getFirst()) {
                newList.add(lista.removeFirst()); // cuando eliminas, a la vez devuelve el eliminado
            } else
                newList.add(lista2.removeFirst());
        }
        newList.addAll(lista);
        newList.addAll(lista2);
        return newList;
    }

    }
