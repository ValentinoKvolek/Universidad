package main.java.tp1.ejercicio9;

public class TestBalanceo {

    //a. Indique qué estructura de datos utilizará para resolver este problema y cómo la utilizará
    //la estructura de datos que usaria para resolver el problema seria el Stack y lo usaria creando push por cada ( que haya y pop por cada ) y retornar un bool.

    public static void main(String[] args) {
        String exp ="{( ) [ ( ) ] }";
        System.out.println("La expresión está balanceada? " + Testeo(exp));
    }

    private static boolean Testeo(String cadena) {
        Stack<Character> stack = new Stack<>();

        for (int i = 0; i < cadena.length(); i++) {
            char car = cadena.charAt(i);

            if (car == '(' || car == '{' || car == '[') {
                stack.push(car); // apilar apertura
            } else if (car == ')' || car == '}' || car == ']') {
                if (stack.isEmpty()) {
                    return false; // si hay un cierre sin antes un apertura false.
                }

                char top = stack.pop(); // ultima apertura

                // si el cirre no concuerda con la apertura false
                if ((car == ')' && top != '(') || (car == '}' && top != '{') || (car == ']' && top != '[')) {
                    return false;
                }
            }
        }

        return stack.isEmpty();
    }

}
