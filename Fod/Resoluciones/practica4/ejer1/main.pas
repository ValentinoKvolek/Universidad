(*

1. Considere que desea almacenar en un archivo la información correspondiente a los alumnos de la Facultad de Informática de la UNLP.
 De los mismos deberá guardarse nombre y apellido, DNI, legajo y año de ingreso. Suponga que dicho archivo se organiza como un árbol B de orden M.

 *)

 program ejer1_prac4;
 
const M = ?? //orden del arbol.

type

    alumno = record
        nombre:string[50];
        apellido:string[50];
        dni:integer;
        legajo:integer;
        ano_ingreso:integer;
    end;

//a. Defina en Pascal las estructuras de datos necesarias para organizar el archivo de alumnos como un árbol B de orden M.

    TNodo = record 
        cant_datos: integer;
        alumnos: array[1..M-1] of alumno;
        hijos: array[1..M] of integer;
    end;

    arbolB = file of TNodo;

var
    archivoDatos: arbolB;
//a

(*

b. Suponga que la estructura de datos que representa una persona (registro de persona) ocupa 64 bytes, que cada nodo del árbol B tiene un tamaño de 512 bytes 
y que los números enteros ocupan 4 bytes, ¿cuántos registros de persona entrarían en un nodo del árbol B? ¿Cuál sería el orden del árbol B en este caso (el valor de M)?
Para resolver este inciso, puede utilizar la fórmula N = (M-1) * A + M * B + C, donde N es el tamaño del nodo (en bytes),
A es el tamaño de un registro (en bytes), B es el tamaño de cada enlace a un hijo y C es el tamaño que ocupa el campo referido a la cantidad de claves.
El objetivo es reemplazar estas variables con los valores dados y obtener el valor de M (M debe ser un número entero, ignorar la parte decimal).

    512 (tamaño del nodo arbolB) =  (M-1) * 64(Tamaño regs) + M * 4( cada enlace es un entero) + 4 (cada clave al ser un entero ocupa tambien 4 ) 

    haciendo el calculo y despejando m te da que m  = 8.411 que rendondeando como dice el enunciado da 8. lo que quiere decir que cada nodo tiene 8 registros
    y que el orden del arbol es 8.

c. ¿Qué impacto tiene sobre el valor de M organizar el archivo con toda la información de los alumnos como un árbol B?

    el impacto es que en la formula planteada anteriormente va a cambiar el numero A debido a que cada registro de alumno va a pesar de manera diferente:
    demostracion: 

        nombre :51 bytes , apellido 51 bytes, dni, legajo y anio como son integers cada uno va a ocupar 4 bytes. 
        por los que nos queda un total de 114 bytes 
        
        512 =  (m-1) * 114 + M * 4 + 4 

        que da un M = 5; 

d. ¿Qué dato seleccionaría como clave de identificación para organizar los elementos (alumnos) en el árbol B? ¿Hay más de una opción?

    Eligiria el dato de dni ya que es una dato que si o si me va  direccionar a un alumno unico. tambien esta la otra opcion de elegir legajo ya que al igual que dni
    es unico.

e. Describa el proceso de búsqueda de un alumno por el criterio de ordenamiento especificado en el punto previo.
¿Cuántas lecturas de nodos se necesitan para encontrar un alumno por su clave de identificación en el peor y en el mejor de los casos? ¿Cuáles serían estos casos?

    la busqueda seria la siguiente: 
    Comenzar en el nodo raiz 
    comparar el dni buscado con las claves del nodo(son dnis ordenados)
    si coincide deolver el registro asociado a ese dni.
    si no , seleccionar el subarbol hijo correspondiente y repetir el proceso.
    continuar hasta encontrar el dni o llegar una sin clave.

no puedo calcular cuantas lecturas voy a tener en el peor de los caso ya que no conozco la altura.
pero bueno en el mejor de los casos seria un 1 accesos (nodo raiz).
yu el peor de los casos seria lecturas H donde h es la altura.

f. ¿Qué ocurre si desea buscar un alumno por un criterio diferente? ¿Cuántas lecturas serían necesarias en el peor de los casos?

    depende del criterio que estemos hablando, si hablamos del lengajo seria exactamente igual por que sigue sindo unico. pero si estamos hablando de nombre tambien tendriamos que hacer
    otro tipo de validaciones para ver si ese nombre es de verdad el alumno que etsamos buscando.(como relacionar apellidos y dni) y eso implica otros costos de accesos 
    adicionales. 
*)



