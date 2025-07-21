package main.java.tp1.ejercicio7;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class IncisoH {

    public static void main(String[] args) {

        ArrayList<Integer> arrayList = new ArrayList<Integer>();
        arrayList.add(1);
        arrayList.add(2);
        arrayList.add(3);
        arrayList.add(4);
        arrayList.add(5);
        System.out.println(arrayList);
        invertirArrayList(arrayList);
        System.out.println(arrayList);


    }


    public static void invertirArrayList(ArrayList<Integer> arrayList){

        if(!arrayList.isEmpty()){
             int primero = arrayList.get(0);
             arrayList.remove(0);
             invertirArrayList(arrayList);
             arrayList.add(primero);
        }
    }
}
