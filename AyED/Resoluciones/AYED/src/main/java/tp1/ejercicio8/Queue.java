package main.java.tp1.ejercicio8;
import java.util.LinkedList;
import java.util.List;

public class Queue<T> extends Sequence {

    List<T> data;
    int size =0;

    public Queue() {
        this.data = new LinkedList<T>();
    }

    public void enqueue(T dato) {
        data.addLast(dato);
        size++;

    }

    public T dequeue() {
        return data.remove(0);
    }


    public T head() {
        return data.get(0);
    }

    @Override
    public int size(){
        return data.size();
    }

    @Override
    public boolean isEmpty(){
        return data.size()==0;
    }

    @Override
    public String toString() {
        String str = "[";
        for (T i : data) {
            str = str + i + ",";
            str = str.substring(0, str.length() - 2) + "]";
        }
        return str;
    }

}