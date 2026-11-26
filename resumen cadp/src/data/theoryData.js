export const THEORY_MODULES = [
  {
    id: 'm1-algoritmos',
    title: '1. Algoritmos y Acciones Elementales',
    badge: 'Fundamentos',
    summary: 'Concepto formal de computadora, programa, pre y post condiciones, y estructuras de control básicas.',
    keyPoints: [
      {
        concept: 'Computadora y Programa',
        definition: 'Computadora: Máquina digital y sincrónica con capacidad de cálculo lógico/aritmético, controlada por un programa almacenado. Programa: Conjunto finito de instrucciones ejecutables por la máquina que transforma datos de entrada en datos de salida.'
      },
      {
        concept: 'Precondición y Postcondición',
        definition: 'Precondición: Conjunto de condiciones o estados que deben ser verdaderos antes de ejecutar una acción o módulo. Postcondición: Estado que se garantiza verdadero inmediatamente después de la ejecución exitosa de la acción.'
      },
      {
        concept: 'Estructuras de Control de Iteración',
        definition: 'FOR: Iteración determinada. Se ejecuta cuando se conoce a priori la cantidad de repeticiones. Se ejecuta 0 o más veces. WHILE: Iteración indeterminada con control antes. Evalúa condición antes de entrar; puede ejecutarse 0 o más veces. REPEAT..UNTIL: Iteración indeterminada con control después. Se ejecuta al menos 1 vez.'
      }
    ]
  },
  {
    id: 'm2-modularizacion',
    title: '2. Modularización, Ámbito y Parámetros',
    badge: 'Clave de Final',
    summary: 'Técnicas de descomposición, alcance de variables (locales vs globales), pasaje de parámetros y el temido aliasing.',
    keyPoints: [
      {
        concept: 'Modularización y Ventajas',
        definition: 'Acción de dividir un programa complejo en subprogramas (módulos) más pequeños e independientes con objetivos bien definidos. Ventajas: Legibilidad, facilidad de prueba y depuración, trabajo en equipo, abstracción y REUSABILIDAD (ahorro de código y memoria).'
      },
      {
        concept: 'Procedimientos vs Funciones en Pascal',
        definition: 'Procedure: Subalgoritmo que ejecuta una serie de acciones y puede retornar 0, 1 o múltiples valores a través de sus parámetros por referencia. Function: Módulo cuyo objetivo principal es calcular y devolver un ÚNICO valor asociado a su nombre. En Pascal estándar, las funciones solo pueden retornar tipos de datos simples u ordinales (y punteros).'
      },
      {
        concept: 'Pasaje por Valor vs Pasaje por Referencia',
        definition: 'Por Valor (in): El módulo recibe una copia del valor del argumento. Modificar el parámetro dentro del módulo NO altera la variable original. Por Referencia (var): El módulo recibe la dirección de memoria de la variable original. Cualquier modificación sobre el parámetro altera directamente la variable del invocador.'
      },
      {
        concept: 'Alcance (Scope) y Variables Globales vs Locales',
        definition: 'Variable Local: Declarada dentro de un módulo, su ciclo de vida comienza al invocar el módulo y muere al terminar su ejecución (vive en el Stack Frame). Variable Global: Declarada en el programa principal, accesible y modificable por cualquier módulo. PELIGRO: El uso de globales introduce acoplamiento estrecho, efectos colaterales ocultos y dificulta la reutilización.'
      },
      {
        concept: 'Aliasing (Colisión de Identificadores)',
        definition: 'Ocurre cuando una misma celda de memoria puede ser referenciada por dos nombres distintos. Caso típico de examen: pasar una variable global por referencia (var) a un procedimiento que también usa esa variable global directamente. Modificar el parámetro altera la global de inmediato.'
      }
    ]
  },
  {
    id: 'm3-tipos-de-datos',
    title: '3. Tipos de Datos y Registros',
    badge: 'Teoría Obligatoria',
    summary: 'Clasificación de tipos, tipos definidos por el usuario (TDDU), subrangos y operaciones válidas/inválidas sobre registros.',
    keyPoints: [
      {
        concept: 'Tipos Simples vs Compuestos',
        definition: 'Simples: Indivisibles (integer, real, char, boolean, subrangos, punteros). Compuestos/Estructurados: Agrupaciones de múltiples elementos bajo un mismo identificador (records, arrays, strings).'
      },
      {
        concept: 'Tipos Definidos por el Usuario (TDDU) y Subrango',
        definition: 'Permiten modelar el problema con mayor precisión, aumentando la legibilidad, documentación y seguridad semántica (el compilador valida rangos válidos). Subrango: Tipo simple y ordinal definido como subconjunto acotado de un tipo ordinal base (ej. 1..31).'
      },
      {
        concept: 'Operaciones Válidas e Inválidas sobre Registros (Record)',
        definition: 'VÁLIDA: A := B (siempre que A y B sean del MISMO tipo, copia todos los campos). INVÁLIDAS: A = B (no existe comparación relacional en bloque), read(A) y write(A) (la E/S debe realizarse campo por campo simple).'
      }
    ]
  },
  {
    id: 'm4-arreglos',
    title: '4. Arreglos: Vectores y Matrices',
    badge: 'Estructuras Estáticas',
    summary: 'Colecciones homogéneas de tamaño fijo, acceso directo, dimensión física vs dimensión lógica y algoritmos de búsqueda.',
    keyPoints: [
      {
        concept: 'Características de Arreglos',
        definition: 'Estructura de datos estática, homogénea, lineal y de ACCESO DIRECTO (a través de un índice). Su tamaño máximo (dimensión física) se fija en tiempo de compilación.'
      },
      {
        concept: 'Dimensión Física (dimF) vs Dimensión Lógica (dimL)',
        definition: 'dimF: Capacidad máxima total de celdas reservadas en memoria. dimL: Cantidad real de elementos válidos almacenados en un momento determinado de la ejecución (0 <= dimL <= dimF).'
      },
      {
        concept: 'Búsqueda Secuencial vs Búsqueda Dicotómica (Binaria)',
        definition: 'Secuencial: Recorre celda por celda; costo O(N); funciona en arreglos desordenados y ordenados. Dicotómica: Requiere OBLIGATORIAMENTE que el arreglo esté ordenado; divide el espacio a la mitad en cada paso; costo O(log2 N).'
      }
    ]
  },
  {
    id: 'm5-punteros-y-memoria',
    title: '5. Punteros y Memoria Dinámica',
    badge: 'Filtro Letal de Final',
    summary: 'Diferencia estricta entre Memoria Estática (Stack) y Memoria Dinámica (Heap), new, dispose, nil, memory leaks y dangling pointers.',
    keyPoints: [
      {
        concept: 'Memoria Estática (Stack) vs Memoria Dinámica (Heap)',
        definition: 'Estática: Gestionada automáticamente por el compilador en la pila (Stack). Se reservan las variables declaradas en la sección var. Tamaño inmutable durante la ejecución. Dinámica: Gestionada explícitamente por el programador en el montículo (Heap) mediante primitivas new() y dispose(). Tamaño y cantidad de elementos variable.'
      },
      {
        concept: 'La Variable Puntero vs el Dato Apuntado',
        definition: 'La variable puntero en sí misma (p) es una dirección de memoria de 4 bytes que vive en el Stack (memoria estática). El dato al que apunta (p^) reside en el Heap (memoria dinámica).'
      },
      {
        concept: 'Primitivas new(), dispose() y el valor nil',
        definition: 'new(p): Reserva memoria en el Heap y guarda la dirección en p. dispose(p): Libera la memoria apuntada en el Heap (la variable p sigue existiendo en el Stack). nil: Constante que representa un puntero que no apunta a ninguna dirección válida.'
      },
      {
        concept: 'Fuga de Memoria (Memory Leak) vs Puntero Colgante (Dangling Pointer)',
        definition: 'Fuga (Leak): Ocurre cuando se pierde el acceso a un bloque de memoria dinámica sin haberlo liberado antes con dispose (ej. hacer p := nil tras new(p)). Puntero Colgante: Ocurre cuando un puntero conserva la dirección de un bloque que ya fue liberado con dispose(p). Desreferenciarlo provoca un Run-time error o corrupción de datos.'
      }
    ]
  },
  {
    id: 'm6-listas',
    title: '6. Listas Enlazadas',
    badge: 'Práctica y Teoría',
    summary: 'Estructuras lineales dinámicas, nodos auto-referenciados, operaciones primitivas de inserción y eliminación.',
    keyPoints: [
      {
        concept: 'Lista Enlazada vs Vector',
        definition: 'La lista es dinámica (crece o decrece a demanda sin límite dimF) y de ACCESO SECUENCIAL (para llegar al i-ésimo elemento hay que recorrer los anteriores). El vector es de tamaño fijo y de ACCESO DIRECTO mediante índice.'
      },
      {
        concept: 'Operaciones Primitivas',
        definition: 'Agregar Adelante: O(1) tiempo, enlaza nue^.sig := l; l := nue. Agregar Atrás: O(1) si se mantiene puntero al último nodo, O(N) si se recorre toda la lista. Inserción Ordenada: O(N) tiempo en peor caso, usa punteros ant y act.'
      },
      {
        concept: 'Eliminación y Preservación de Enlaces',
        definition: 'Para eliminar un nodo se debe enlazar ant^.sig := act^.sig (o l := l^.sig si es el primero) y liberar la memoria con dispose(act).'
      }
    ]
  },
  {
    id: 'm7-eficiencia',
    title: '7. Eficiencia de Programas (UDT y Memoria)',
    badge: 'Matemática del Final',
    summary: 'Cálculo formal del tiempo de ejecución en UDT, peor y mejor caso, y cálculo de memoria en bytes.',
    keyPoints: [
      {
        concept: 'Concepto de Eficiencia',
        definition: 'Medida del consumo de recursos de una computadora (tiempo de CPU y espacio de memoria) que requiere un algoritmo para resolver un problema. Una solución correcta no necesariamente es eficiente.'
      },
      {
        concept: 'Unidad de Tiempo (UDT) y Fórmulas de Bucles',
        definition: 'Asignación = 1 UDT. Condición = C UDT. For = (3N + 2) + N*(cuerpo). While = C*(N + 1) + N*(cuerpo). Repeat = C*N + N*(cuerpo).'
      },
      {
        concept: 'Memoria Estática y Dinámica',
        definition: 'Memoria Estática = suma de sizeof de todas las variables en var. Memoria Dinámica = suma de sizeof de todos los new() activos en el Heap menos los liberados con dispose().'
      }
    ]
  },
  {
    id: 'm8-correccion',
    title: '8. Corrección de Programas y Tipos de Errores',
    badge: 'Preguntas de Cátedra',
    summary: 'Diferencia estricta entre Verificación, Testing, Debugging, Walkthroughs y clasificación de fallos.',
    keyPoints: [
      {
        concept: 'Clasificación de Errores',
        definition: 'Léxicos/Sintácticos: Incumplimiento de la gramática del lenguaje (detectados en compilación). Semánticos: El programa compila pero hace algo ilógico (ej. división por cero, variable sin inicializar, bucle infinito).'
      },
      {
        concept: 'Técnicas de Validación de Calidad',
        definition: 'Verificación: Demostración formal o analítica de que el programa cumple su especificación. Testing: Ejecución sistemática con casos de prueba y valores límite para descubrir fallos. Debugging: Proceso dinámico de localizar y reparar la causa de un error tras haber sido descubierto en ejecución. Walkthrough: Revisión estática de código ante pares para encontrar defectos de diseño.'
      }
    ]
  }
];
