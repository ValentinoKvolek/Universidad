export const QUIZ_QUESTIONS = [
  {
    id: 1,
    category: 'Modularización',
    question: 'Un programa modularizado siempre es eficiente.',
    isTrue: false,
    justification: 'Falso. Modularizar y eficiencia son conceptos independientes. La modularización busca mejorar la legibilidad, mantenibilidad, diseño descendente y reusabilidad del código. Sin embargo, dentro de los módulos se pueden utilizar algoritmos ineficientes (ej. O(N^2) en lugar de O(N log N)), y el pasaje de grandes estructuras por valor genera sobrecarga de tiempo y memoria (overhead).',
    docenteKeywords: ['Conceptos independientes', 'Algoritmos ineficientes', 'Overhead de llamadas', 'Reusabilidad y legibilidad']
  },
  {
    id: 2,
    category: 'Estructuras de Datos',
    question: 'En el acceso a los campos de un registro es necesario respetar el orden en que fueron declarados en la sección type.',
    isTrue: false,
    justification: 'Falso. El acceso a los campos de un registro es DIRECTO. Se puede acceder a cualquier campo en cualquier momento usando la notación de punto (variable.campo) sin necesidad de recorrer los campos previos ni respetar su orden de declaración.',
    docenteKeywords: ['Acceso directo', 'Notación de punto', 'Sin orden secuencial']
  },
  {
    id: 3,
    category: 'Memoria y Punteros',
    question: 'Al asignar el valor nil a un puntero p se libera la memoria referenciada en el Heap.',
    isTrue: false,
    justification: 'Falso. Asignar p := nil únicamente corta el enlace entre la variable puntero y la dirección de memoria dinámica, haciendo que el puntero apunte a una dirección nula. La memoria en el Heap NO se libera y queda inaccesible, produciendo una fuga de memoria (Memory Leak). Para liberar la memoria dinámica es indispensable invocar dispose(p).',
    docenteKeywords: ['Corta el enlace', 'Fuga de memoria (Memory Leak)', 'dispose() obligatorio']
  },
  {
    id: 4,
    category: 'Memoria y Punteros',
    question: 'Una variable de tipo puntero siempre se almacena en memoria dinámica.',
    isTrue: false,
    justification: 'Falso. La variable puntero en sí misma (la que guarda la dirección de memoria, de 4 bytes) se almacena en memoria estática (en el Stack) si está declarada en la sección var del programa principal o de un módulo. Lo que puede residir en memoria dinámica (en el Heap) es el dato o bloque al que apunta tras invocar new().',
    docenteKeywords: ['Variable puntero en Stack (estática)', 'Dato apuntado en Heap (dinámica)', '4 bytes']
  },
  {
    id: 5,
    category: 'Estructuras de Datos',
    question: 'Un vector siempre es almacenado en memoria estática.',
    isTrue: false,
    justification: 'Falso. Un vector puede ser alocado dinámicamente en el Heap mediante un puntero a arreglo (ej. type ptrVector = ^vector; new(p)), o puede formar parte del campo de un nodo dinámico de una lista enlazada, en cuyo caso reside en memoria dinámica.',
    docenteKeywords: ['Puntero a arreglo en Heap', 'Campo de nodo dinámico', 'Alocación dinámica']
  },
  {
    id: 6,
    category: 'Modularización',
    question: 'Un módulo función puede retornar un tipo de datos lista (siendo lista un puntero a un nodo).',
    isTrue: true,
    justification: 'Verdadero. En Pascal, las funciones pueden retornar cualquier tipo de dato simple u ordinal. Como una lista enlazada en Pascal se implementa mediante una variable de tipo puntero (^nodo), y el puntero es un tipo de dato simple (una dirección de memoria de 4 bytes), una función puede perfectamente retornar un puntero al primer nodo de una lista.',
    docenteKeywords: ['Tipo de dato simple', 'Puntero = dirección de 4 bytes', 'Retorno válido en función']
  },
  {
    id: 7,
    category: 'Tipos y Sintaxis',
    question: 'Dadas dos variables A y B del mismo tipo registro, la operación A = B es válida para comparar si tienen los mismos datos.',
    isTrue: false,
    justification: 'Falso. En Pascal no está definida la comparación relacional global de registros (ni = ni <>). Para saber si dos registros son iguales, el programador debe comparar manualmente campo por campo simple.',
    docenteKeywords: ['Comparación global no permitida', 'Comparar campo por campo', 'Operación inválida']
  },
  {
    id: 8,
    category: 'Tipos y Sintaxis',
    question: 'Dadas dos variables A y B del mismo tipo registro, la sentencia A := B es una operación válida.',
    isTrue: true,
    justification: 'Verdadero. En Pascal la asignación directa entre registros es válida siempre y cuando ambas variables sean exactamente del mismo tipo de registro. Esto realiza una copia bit a bit campo por campo de todos los valores de B en A.',
    docenteKeywords: ['Asignación válida', 'Mismo tipo de registro', 'Copia campo a campo']
  },
  {
    id: 9,
    category: 'Tipos y Sintaxis',
    question: 'Las instrucciones read(R) y write(R), donde R es una variable de tipo registro, son sentencias válidas de Pascal.',
    isTrue: false,
    justification: 'Falso. Las operaciones de entrada/salida read y write en Pascal solo operan sobre tipos de datos simples (char, integer, real, string). No se puede leer ni escribir un registro en bloque; debe hacerse campo por campo primitivo.',
    docenteKeywords: ['Solo tipos simples', 'Lectura/escritura campo por campo', 'Registro en bloque inválido']
  },
  {
    id: 10,
    category: 'Tipos y Sintaxis',
    question: 'El tipo de datos subrango es ordinal pero no compuesto.',
    isTrue: true,
    justification: 'Verdadero. Un subrango (ej. 1..10 o \'A\'..\'Z\') es un tipo simple y ordinal porque cada elemento tiene un antecesor y un sucesor bien definido, y no contiene partes internas divisibles (a diferencia de los compuestos como registros o arreglos).',
    docenteKeywords: ['Tipo simple', 'Ordinal', 'Antecesor y sucesor', 'No estructurado']
  },
  {
    id: 11,
    category: 'Corrección y Eficiencia',
    question: 'La técnica de corrección de debugging requiere obligatoriamente que el programa haya sido ejecutado al menos una vez.',
    isTrue: true,
    justification: 'Verdadero. El debugging (depuración) es el proceso dinámico de localizar, analizar y reparar la causa de un fallo que ya fue detectado durante la ejecución del programa (en tiempo de prueba o testing). A diferencia de las revisiones estáticas (inspección de código o walkthroughs), el debugging parte de un síntoma de ejecución.',
    docenteKeywords: ['Proceso dinámico', 'Síntoma en ejecución', 'Distinto a walkthrough/inspección']
  },
  {
    id: 12,
    category: 'Corrección y Eficiencia',
    question: 'Un programa que contiene errores semánticos se puede compilar exitosamente.',
    isTrue: true,
    justification: 'Verdadero. El compilador solo detecta errores léxicos y sintácticos (de gramática del lenguaje). Los errores semánticos (de significado o lógica, como usar una variable no inicializada, un bucle infinito o un cálculo matemático erróneo) superan la compilación y provocan fallos en tiempo de ejecución o salidas incorrectas.',
    docenteKeywords: ['Compilación exitosa', 'Error lógico/semántico', 'Supera chequeo sintáctico']
  },
  {
    id: 13,
    category: 'Estructuras de Datos',
    question: 'La estructura de datos Lista Enlazada es una estructura heterogénea.',
    isTrue: false,
    justification: 'Falso. Las listas enlazadas en Pascal estándar son HOMOGÉNEAS, ya que todos los nodos deben almacenar elementos del mismo tipo de dato declarado en el registro nodo (ej. dato: tipoDato).',
    docenteKeywords: ['Homogénea', 'Mismo tipo de elemento en cada nodo']
  },
  {
    id: 14,
    category: 'Estructuras de Control',
    question: 'Las instrucciones dentro de una estructura de control for siempre se ejecutan al menos una vez.',
    isTrue: false,
    justification: 'Falso. Si el valor inicial de la variable de control es estrictamente mayor que el valor final en un bucle ascendente (ej. for i := 10 to 1 do), la condición de cota es falsa desde el inicio y el cuerpo del for se ejecuta CERO veces.',
    docenteKeywords: ['Ejecución 0 veces', 'Valor inicial mayor al final', 'Cero o más veces']
  },
  {
    id: 15,
    category: 'Memoria y Punteros',
    question: 'Para usar una variable de tipo puntero p siempre se debe invocar obligatoriamente new(p).',
    isTrue: false,
    justification: 'Falso. Un puntero se puede inicializar asignándole nil (p := nil) o asignándole la dirección de otro puntero ya existente (p := otroP). new(p) solo se usa cuando se requiere crear un bloque nuevo en el Heap.',
    docenteKeywords: ['Asignación nil', 'Copia de otro puntero', 'new solo para nuevo bloque']
  }
];
