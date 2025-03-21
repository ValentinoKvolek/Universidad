package tp1.ejercicio7;

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
        List<Integer> resultado = invertirArrayList(arrayList);
        System.out.println(resultado);


    }


    public static List invertirArrayList(ArrayList<Integer> arrayList){

        Collections.reverse(arrayList);

        return arrayList;

    }

}
