package main.java.tp1.ejercicio8;

import java.util.LinkedList;

public class DoubleEndedQueue<T> {

    private LinkedList<T> dato;

    public T dequeueFirst() {
        return dato.removeFirst();
    }

}