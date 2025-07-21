package main.java.parciales1.p14;

import main.java.tp3.ejercicio1.GeneralTree;

import java.sql.ClientInfoStatus;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Parcial {

    public Integer resolver(GeneralTree<Integer> ab){


        if(ab != null){
            if(!ab.isEmpty()){
                int sumaNeg = 0;
                int sumaPos = 0;
                int sumaTotal = 0;
                List<Integer>numeros = new ArrayList<>();

                recorrer(ab,numeros);
                for (Integer n : numeros){
                    if(n > 0){
                        sumaPos += n;
                    }else{
                        sumaNeg+=n;
                    }
                    sumaTotal+=n;
                }
                if(sumaTotal %2 ==0 ){
                    return sumaPos;
                }
                return sumaNeg;
            }
        }
        return 0;
    }

    private void  recorrer (GeneralTree<Integer>nodo, List<Integer> numeros){

        List<GeneralTree<Integer>>hijos = nodo.getChildren();

        for(GeneralTree<Integer> hijo: hijos){
            recorrer(hijo, numeros);
        }

        if(nodo.getData() != null){
            numeros.add(nodo.getData());
        }

    }



    public static void main(String[] args) {
        GeneralTree<Integer> tree = new GeneralTree<>(-3);

        GeneralTree<Integer> tree1 = new GeneralTree<>(2);
        GeneralTree<Integer> tree2 = new GeneralTree<>(11);
        GeneralTree<Integer> tree3 = new GeneralTree<>(6);
        GeneralTree<Integer> tree4 = new GeneralTree<>(-7);
        GeneralTree<Integer> tree5 = new GeneralTree<>(4);
        GeneralTree<Integer> tree6 = new GeneralTree<>(-9);
        GeneralTree<Integer> tree7 = new GeneralTree<>(5);
        GeneralTree<Integer> tree8 = new GeneralTree<>(3);
        GeneralTree<Integer> tree9 = new GeneralTree<>(1);

        tree.addChild(tree1);
        tree.addChild(tree2);
        tree.addChild(tree3);
        tree.addChild(tree4);
        tree.addChild(tree5);
        tree.addChild(tree6);
        tree.addChild(tree7);
        tree.addChild(tree8);
        tree.addChild(tree9);


        Parcial p = new Parcial();
        System.out.println(p.resolver(tree));
    }
}
