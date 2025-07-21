(*

Los árboles B+ representan una mejora sobre los árboles B dado que conservan la propiedad de acceso indexado a los registros del archivo de datos por alguna clave,
pero permiten además un recorrido secuencial rápido. Al igual que en el ejercicio 2,
considere que por un lado se tiene el archivo que contiene la información de los alumnos de la Facultad de Informática (archivo de datos no ordenado)
y por otro lado se tiene un índice al archivo de datos, pero en este caso el índice se estructura como un árbol B+ que ofrece acceso indizado por DNI al archivo de alumnos.
Resuelva los siguientes incisos:
 



a. ¿Cómo se organizan los elementos (claves) de un árbol B+? ¿Qué elementos se encuentran en los nodos internos y que elementos se encuentran en los nodos hojas?

    los elementos los nodos b+ se organizan con claves que son punteros a los datos. en los nodos intermedios se encuentran copias que sirven para separar los nodos 
    en sentido de mayor a menor en este caso tendrian copias de claves y punteros a hijos.
    y los nodos terminarles si tiene todos los datos en este caso: 
    tendrian claves(dni), punteros a registros, y enlaces entre hojas y enlaces entre hojas para recorrido secuencial.
    cada nodo terminal esta conectado para recorrer secuencialmente de forma eficiente y rapida.


b. ¿Qué característica distintiva presentan los nodos hojas de un árbol B+? ¿Por qué?

    la caracteristica distintiva qyue presentan los nodo hojas es la caracteristica  de tener enlaces entre hoja. debido a que esto permite hacer funcional al arbol + 
    y diferenciarlo de los otros , ya que permite una busqueda secuencial efectiva y rapida.

c. Defina en Pascal las estructuras de datos correspondientes para el archivo de alumnos y su índice (árbol B+). Por simplicidad, 
   suponga que todos los nodos del árbol B+ (nodos internos y nodos hojas) tienen el mismo tamaño.

    

 *)

