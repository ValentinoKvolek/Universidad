(*

2. Una mejora respecto a la solución propuesta en el ejercicio 1 sería mantener por un lado el archivo que contiene la información de los alumnos de la Facultad de Informática
(archivo de datos no ordenado) y por otro lado mantener un índice al archivo de datos que se estructura como un árbol B que ofrece acceso indizado por DNI de los alumnos. 

*)

//a. Defina en Pascal las estructuras de datos correspondientes para el archivo de alumnos y su índice.

program ejer2_prac4;

const M = //orden del arbol;

type

    alumno = record
        nombre:string[50];
        apellido:string[50];
        dni:integer;
        lengajo:integer;
        ano_ingreso:integer;
    end;

    TNodo = record

        cant_claves:integer;
        claves: array[1..M-1] of integer; //serian los dnis
        enlaces : array[1..M-1] of integer;   //si claves[1] = 12345678, entonces enlaces[1] tiene el número de registro en el archivo donde está el alumno con ese DNI.
                                              //No son hijos, son referencias al archivo de alumnos. 
        hijos: array[1..M] of integer; 

    end;

    TAarchivoAlumnos = file of alumno; //archivo desordenado de alumnos.
    arbolB = file of TNnodo;

    var

        archivoDatos: TAarchivoAlumnos;
        archivoIndice: arbolB;

//a

(*

b. Suponga que cada nodo del árbol B cuenta con un tamaño de 512 bytes.
¿Cuál sería el orden del árbol B (valor de M) que se emplea como índice? Asuma que los números enteros ocupan 4 bytes.
Para este inciso puede emplear una fórmula similar al punto 1b, 
pero considere además que en cada nodo se deben almacenar los M-1 enlaces a los registros correspondientes en el archivo de datos.

    el orden del arbol seria 64 ya que ahora a diferencia del ejer 1b , tenemos que solo ocupar 4 bytes en el tamaño de A que seria la referencia a ese enlace.
    en este caso como la referencia a el archivo alumnos es la clave que es un dni pasaria 4 bytes
    

c. ¿Qué implica que el orden del árbol B sea mayor que en el caso del ejercicio 1?

    Implica lo que ahora tenes dor archivos separados nos permite solo tomar una referencia unica (la clave) para referenciar a un registro alumnos en nuestro 
    archivo de datos alumno. en vez de gastar muchos mas bytes guardando un registro entero.

d. Describa con sus palabras el proceso para buscar el alumno con el DNI 12345678 usando el índice definido en este punto.

    se empezaria desde la raiz preguntando di el dni buscado es igual el dni q esta como clave.
    si no lo es se comprar si el dni buscado es mas chico o mas grande que el que esta en la raiz y eso determina para donde recorrer. 
    sigue buscando el dni. si lo encuentra se en el campo enlace en pque pos del archivo alumnos esta todo el registro del alumno con ese dni. 
    si no sigue buscando. si no lo encontro por que llego a una hoja termina la busqueda .

e.¿Qué ocurre si desea buscar un alumno por su número de legajo?
 ¿Tiene sentido usar el índice que organiza el acceso al archivo de alumnos por DNI?
  ¿Cómo haría para brindar acceso indizado al archivo de alumnos por número de legajo?

  La busqueda seria la misma ya que un numero de lagajo es unico por alumno. lo cual hace que tenga sentdo organizar el acceso de esta manera. al igual que con el dni.
  No tiene sentido usar el índice por DNI para buscar por legajo. El árbol B está ordenado por DNI, y el legajo no está relacionado con ese orden.
  Sería necesario recorrer todos los nodos (búsqueda exhaustiva), lo que es ineficiente.
  para brincdar el acceso haria que mi clave se el legajo y que en enlaces esta la direccion donde esta el registro de ese alumno en el arch

f. Suponga que desea buscar los alumnas que tienen DNI en el rango entre 40000000 y 45000000.
 ¿Qué problemas tiene este tipo de búsquedas con apoyo de un árbol B que solo provee acceso indizado por DNI al archivo de alumnos?

    La búsqueda por rango requiere recorrer secuencialmente desde el DNI inicial hasta el final,
    lo que implica múltiples accesos a nodos no contiguos, aumentando el costo y reduciendo la eficiencia.]


*) 

//c. Defina en Pascal las estructuras de datos correspondientes para el archivo de alumnos y su índice (árbol B+). Por simplicidad, 
//suponga que todos los nodos del árbol B+ (nodos internos y nodos hojas) tienen el mismo tamaño

program ejer3_prac4;

const
    M = 64; {orden del árbol B+, calculado previamente como en el ejercicio 2b}

type
    alumno = record
        nombre: string[50];
        apellido: string[50];
        dni: integer;
        legajo: integer;
        ano_ingreso: integer;
    end;

    {Nodo interno del árbol B+}
    TNodoInterno = record
        cant_claves: integer;
        claves: array[1..M-1] of integer; {copias de claves (DNI)}
        hijos: array[1..M] of integer; {punteros a nodos hijos}
    end;

    {Nodo hoja del árbol B+}
    TNodoHoja = record
        cant_claves: integer;
        claves: array[1..M-1] of integer; {claves (DNI)}
        enlaces: array[1..M-1] of integer; {punteros a registros en el archivo de datos}
        siguiente: integer; {enlace al siguiente nodo hoja}
    end;

    TNodo = record
        case esHoja: boolean of
            true: (hoja: TNodoHoja);
            false: (interno: TNodoInterno);
    end;

    TAarchivoAlumnos = file of alumno;
    arbolBPlus = file of TNodo;

var
    archivoDatos: TAarchivoAlumnos;
    archivoIndice: arbolBPlus;
//c.

(*

d. Describa, con sus palabras, el proceso de búsqueda de un alumno con un DNI específico haciendo uso de la estructura auxiliar
 (índice) que se organiza como un árbol B+. ¿Qué diferencia encuentra respecto a la búsqueda en un índice estructurado como un árbol B?
    
    La busqueda es igual por que estamos hablando de una busqueda directa. los pasos a seguir es irt comparando el dni desde la raiz con las copias de la claves.
    las copias van guiando hasta la hoija y en esa hoja vas directo al dni donde esta el enlace al registro del alumno.

    le diferencia clave es que los nodos internos solo contienen una copia para ir guiando la busqueda, en cambio en un arbol b normal los nodos internos tambien tienen referencia al archi alumno

e. Explique con sus palabras el proceso de búsqueda de los alumnos que tienen DNI en el rango entre 40000000 y 45000000, apoyando la búsqueda en un índice organizado como un árbol B+.
 ¿Qué ventajas encuentra respecto a este tipo de búsquedas en un árbol B?
    
  Buscar 40000000 desde la raíz hasta su hoja, luego recorrer secuencialmente las hojas con siguiente, recolectando DNI en el rango hasta 45000000.
  Ventaja sobre B: recorrido secuencial más rápido gracias a los enlaces entre hojas y datos solo en hojas.

*) 

