export const ALGORITHMS = [
  {
    id: 'insertar-ordenado',
    title: 'Inserción Ordenada en Lista Enlazada',
    category: 'Listas',
    difficulty: 'Filtro de Examen',
    description: 'El algoritmo más tomado en finales. Inserta un nuevo elemento manteniendo la lista ordenada de menor a mayor. Maneja lista vacía, inserción al principio y en el medio/final.',
    pascalCode: `procedure insertarOrdenado(var l: lista; d: dato);
var
  nue, act, ant: lista;
begin
  new(nue);
  nue^.elem := d;
  act := l;
  ant := l;
  while (act <> nil) and (act^.elem < d) do
  begin
    ant := act;
    act := act^.sig;
  end;
  if (act = l) then
    l := nue
  else
    ant^.sig := nue;
  nue^.sig := act;
end;`,
    complexity: {
      time: 'O(N) - Peor caso si va al final o la lista es grande',
      space: 'O(1) auxiliar (1 nodo nuevo en Heap)',
      docenteNote: '¡Ojo con el orden de la condición en el while! Si ponés (act^.elem < d) and (act <> nil), en Pascal estándar explota con Run-time error al llegar a nil.'
    },
    steps: [
      {
        stepIndex: 0,
        explanation: 'Estado inicial: La lista contiene los nodos [10, 25, 40]. L apunta al primer nodo (10). Queremos insertar el dato d = 28.',
        activeLine: 4,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: false },
        pointers: { l: 1, ant: null, act: null, nue: null }
      },
      {
        stepIndex: 1,
        explanation: 'new(nue) y nue^.elem := 28: Se aloca en el Heap el nodo con dato 28. El puntero nue apunta al nodo (su campo sig aún está sin enlazar).',
        activeLine: 5,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: null, act: null, nue: 'newNode' }
      },
      {
        stepIndex: 2,
        explanation: 'act := l; ant := l; Ambos punteros auxiliares se inicializan apuntando al primer nodo (10).',
        activeLine: 7,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 1, act: 1, nue: 'newNode' }
      },
      {
        stepIndex: 3,
        explanation: '1ra Evaluación While: ¿(act <> nil) and (act^.elem < 28)? SÍ: act está en 10 y 10 < 28. Entramos al cuerpo.',
        activeLine: 9,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 1, act: 1, nue: 'newNode' }
      },
      {
        stepIndex: 4,
        explanation: 'ant := act: ant toma la posición de act (permanece en nodo 10).',
        activeLine: 11,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 1, act: 1, nue: 'newNode' }
      },
      {
        stepIndex: 5,
        explanation: 'act := act^.sig: ¡Avance de act! act sigue la flecha sig y avanza al nodo 25. ant se queda en el nodo 10.',
        activeLine: 12,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 1, act: 2, nue: 'newNode' }
      },
      {
        stepIndex: 6,
        explanation: '2da Evaluación While: ¿(act <> nil) and (act^.elem < 28)? SÍ: act está en 25 y 25 < 28. Entramos de nuevo al cuerpo.',
        activeLine: 9,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 1, act: 2, nue: 'newNode' }
      },
      {
        stepIndex: 7,
        explanation: 'ant := act: ¡Avance de ant! ant ahora avanza y alcanza a act en el nodo 25.',
        activeLine: 11,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 2, act: 2, nue: 'newNode' }
      },
      {
        stepIndex: 8,
        explanation: 'act := act^.sig: ¡Avance de act! act sigue la flecha sig y avanza al nodo 40. ant se queda en 25.',
        activeLine: 12,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 2, act: 3, nue: 'newNode' }
      },
      {
        stepIndex: 9,
        explanation: '3ra Evaluación While: ¿(act <> nil) and (act^.elem < 28)? act está en 40, pero 40 < 28 es FALSO. ¡El bucle termina! Lugar encontrado: entre ant (25) y act (40).',
        activeLine: 9,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 2, act: 3, nue: 'newNode' }
      },
      {
        stepIndex: 10,
        explanation: 'if (act = l): ¿act (40) es igual a l (10)? Falso. No es inserción al principio de la lista, entra a la rama else.',
        activeLine: 14,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 2, act: 3, nue: 'newNode' }
      },
      {
        stepIndex: 11,
        explanation: 'ant^.sig := nue: ¡Primer enlace! El nodo apuntado por ant (25) redirige su campo sig hacia el nodo nue (28). Se interrumpe el enlace directo de 25 a 40.',
        activeLine: 17,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 2, act: 3, nue: 'newNode' },
        connectAntNue: true
      },
      {
        stepIndex: 12,
        explanation: 'nue^.sig := act: ¡Segundo enlace! El campo sig del nuevo nodo (28) se enlaza al nodo act (40). ¡La lista queda perfectamente conectada: 10 -> 25 -> 28 -> 40 -> nil!',
        activeLine: 18,
        nodes: [{ id: 1, val: 10 }, { id: 2, val: 25 }, { id: 3, val: 40 }],
        newNode: { val: 28, visible: true },
        pointers: { l: 1, ant: 2, act: 3, nue: 'newNode' },
        connectAntNue: true,
        connectNueAct: true
      }
    ]
  },
  {
    id: 'eliminar-repetidos',
    title: 'Eliminar Repetidos en Lista',
    category: 'Listas',
    difficulty: 'Clásico de Final (ej. Mayo 2026)',
    description: 'En el final de Mayo 2026 pidieron eliminar páginas repetidas de cada libro. Este algoritmo recorre la lista y libera la memoria en el Heap de los nodos duplicados.',
    pascalCode: `procedure eliminarRepetidos(var l: lista; var cantElim: integer);
var
  act, ant, aux: lista;
begin
  cantElim := 0;
  act := l;
  ant := nil;
  while (act <> nil) do
  begin
    if (ant <> nil) and (act^.elem = ant^.elem) then
    begin
      aux := act;
      ant^.sig := act^.sig;
      act := act^.sig;
      dispose(aux);
      cantElim := cantElim + 1;
    end
    else
    begin
      ant := act;
      act := act^.sig;
    end;
  end;
end;`,
    complexity: {
      time: 'O(N) - Recorrido lineal único sobre lista ordenada',
      space: 'O(1) auxiliar - Libera memoria con dispose()',
      docenteNote: '¡Error garrafal desaprobatorio: hacer act := act^.sig DESPUÉS de dispose(act)! Siempre guardar en aux antes de dispose o avanzar el puntero antes.'
    },
    steps: [
      {
        stepIndex: 0,
        explanation: 'Lista con páginas duplicadas: [1, 2, 2, 3]. act apunta a 1, ant es nil.',
        activeLine: 7,
        nodes: [{ id: 1, val: 1 }, { id: 2, val: 2 }, { id: 3, val: 2 }, { id: 4, val: 3 }],
        pointers: { l: 1, ant: null, act: 1, aux: null }
      },
      {
        stepIndex: 1,
        explanation: 'Nodo 1: ant es nil, no hay duplicado. Avanzamos: ant:=1, act:=2.',
        activeLine: 19,
        nodes: [{ id: 1, val: 1 }, { id: 2, val: 2 }, { id: 3, val: 2 }, { id: 4, val: 3 }],
        pointers: { l: 1, ant: 1, act: 2, aux: null }
      },
      {
        stepIndex: 2,
        explanation: 'Nodo 2: act(2) <> ant(1). No es repetido. Avanzamos: ant:=2 (primer nodo 2), act:=3 (segundo nodo 2).',
        activeLine: 19,
        nodes: [{ id: 1, val: 1 }, { id: 2, val: 2 }, { id: 3, val: 2 }, { id: 4, val: 3 }],
        pointers: { l: 1, ant: 2, act: 3, aux: null }
      },
      {
        stepIndex: 3,
        explanation: '¡DUPLICADO DETECTADO! act^.elem (2) = ant^.elem (2). Guardamos aux := act (segundo nodo 2).',
        activeLine: 12,
        nodes: [{ id: 1, val: 1 }, { id: 2, val: 2 }, { id: 3, val: 2, isDupe: true }, { id: 4, val: 3 }],
        pointers: { l: 1, ant: 2, act: 3, aux: 3 }
      },
      {
        stepIndex: 4,
        explanation: 'Salteamos el nodo: ant^.sig := act^.sig (el primer 2 ahora apunta al 3). act avanza al 3.',
        activeLine: 14,
        nodes: [{ id: 1, val: 1 }, { id: 2, val: 2 }, { id: 3, val: 2, isDetached: true }, { id: 4, val: 3 }],
        pointers: { l: 1, ant: 2, act: 4, aux: 3 }
      },
      {
        stepIndex: 5,
        explanation: 'dispose(aux): Se libera la memoria del duplicado en el Heap. cantElim incrementa a 1.',
        activeLine: 15,
        nodes: [{ id: 1, val: 1 }, { id: 2, val: 2 }, { id: 4, val: 3 }],
        pointers: { l: 1, ant: 2, act: 4, aux: null }
      },
      {
        stepIndex: 6,
        explanation: 'Fin del recorrido: act llega a nil. Lista final depurada: [1, 2, 3]. Se eliminó 1 repetido.',
        activeLine: 22,
        nodes: [{ id: 1, val: 1 }, { id: 2, val: 2 }, { id: 4, val: 3 }],
        pointers: { l: 1, ant: null, act: null, aux: null }
      }
    ]
  },
  {
    id: 'corte-de-control',
    title: 'Corte de Control sobre Estructuras Ordenadas',
    category: 'Patrones de Final',
    difficulty: 'Común en Ejercicio 1',
    description: 'Aparece en finales como INDEC Canasta Familiar (2024), Ventas por Producto (2023, 2015) y Películas por Género (2025). Procesa grupos homogéneos de datos ordenados.',
    pascalCode: `procedure corteDeControl(l: lista);
var
  actualCategoria: integer;
  totalCategoria, totalGeneral: real;
begin
  totalGeneral := 0;
  while (l <> nil) do
  begin
    actualCategoria := l^.dato.codigo;
    totalCategoria := 0;
    while (l <> nil) and (l^.dato.codigo = actualCategoria) do
    begin
      totalCategoria := totalCategoria + l^.dato.precio;
      l := l^.sig;
    end;
    writeln('Categoria ', actualCategoria, ' Gasto: ', totalCategoria:2:2);
    totalGeneral := totalGeneral + totalCategoria;
  end;
  writeln('Costo Total General: ', totalGeneral:2:2);
end;`,
    complexity: {
      time: 'O(N) - Cada nodo se visita exactamente una vez a pesar del doble while',
      space: 'O(1) auxiliar',
      docenteNote: '¡Atención al examen! Los dos while NO hacen O(N^2). El puntero l avanza en el bucle interior, por lo que cada nodo se procesa 1 sola vez: tiempo lineal O(N).'
    },
    steps: [
      {
        stepIndex: 0,
        explanation: 'Datos ordenados por Categoría: Cat 1 ($50), Cat 1 ($30), Cat 2 ($120). totalGeneral = 0.',
        activeLine: 6,
        nodes: [
          { id: 1, val: 'Cat 1: $50' },
          { id: 2, val: 'Cat 1: $30' },
          { id: 3, val: 'Cat 2: $120' }
        ],
        pointers: { l: 1, actualCat: null, subtotal: 0 }
      },
      {
        stepIndex: 1,
        explanation: 'Entra al grupo Cat 1. actualCategoria := 1; totalCategoria := 0.',
        activeLine: 9,
        nodes: [
          { id: 1, val: 'Cat 1: $50', active: true },
          { id: 2, val: 'Cat 1: $30' },
          { id: 3, val: 'Cat 2: $120' }
        ],
        pointers: { l: 1, actualCat: 1, subtotal: 0 }
      },
      {
        stepIndex: 2,
        explanation: 'Procesa nodo 1: subtotal = 0 + 50 = $50. Avanza l al siguiente nodo.',
        activeLine: 13,
        nodes: [
          { id: 1, val: 'Cat 1: $50', done: true },
          { id: 2, val: 'Cat 1: $30', active: true },
          { id: 3, val: 'Cat 2: $120' }
        ],
        pointers: { l: 2, actualCat: 1, subtotal: 50 }
      },
      {
        stepIndex: 3,
        explanation: 'Procesa nodo 2: mismo código (1). subtotal = 50 + 30 = $80. Avanza l al nodo 3.',
        activeLine: 13,
        nodes: [
          { id: 1, val: 'Cat 1: $50', done: true },
          { id: 2, val: 'Cat 1: $30', done: true },
          { id: 3, val: 'Cat 2: $120' }
        ],
        pointers: { l: 3, actualCat: 1, subtotal: 80 }
      },
      {
        stepIndex: 4,
        explanation: '¡CORTE DE CONTROL! Nodo 3 tiene Cat 2 <> 1. Termina el while interior. Imprime Categ 1 = $80. Suma al total general ($80).',
        activeLine: 16,
        nodes: [
          { id: 1, val: 'Cat 1: $50', done: true },
          { id: 2, val: 'Cat 1: $30', done: true },
          { id: 3, val: 'Cat 2: $120', active: true }
        ],
        pointers: { l: 3, actualCat: 2, subtotal: 0, totalGen: 80 }
      },
      {
        stepIndex: 5,
        explanation: 'Procesa grupo Cat 2: subtotal = $120. Avanza l a nil. Fin del bucle exterior. Total General: $200.',
        activeLine: 19,
        nodes: [
          { id: 1, val: 'Cat 1: $50', done: true },
          { id: 2, val: 'Cat 1: $30', done: true },
          { id: 3, val: 'Cat 2: $120', done: true }
        ],
        pointers: { l: null, actualCat: null, subtotal: 0, totalGen: 200 }
      }
    ]
  },
  {
    id: 'busqueda-dicotomica',
    title: 'Búsqueda Dicotómica (Binaria) en Vector Ordenado',
    category: 'Vectores',
    difficulty: 'Pregunta Clave de Eficiencia',
    description: 'Encuentra un valor dividiendo el espacio de búsqueda a la mitad en cada paso. Requiere obligatoriamente que el vector esté previamente ordenado.',
    pascalCode: `function busquedaDicotomica(v: vector; dimL: integer; x: integer): integer;
var
  pri, ult, medio: integer;
  encontre: boolean;
begin
  pri := 1;
  ult := dimL;
  encontre := false;
  while (pri <= ult) and (not encontre) do
  begin
    medio := (pri + ult) div 2;
    if (v[medio] = x) then
      encontre := true
    else if (v[medio] > x) then
      ult := medio - 1
    else
      pri := medio + 1;
  end;
  if encontre then busquedaDicotomica := medio
  else busquedaDicotomica := 0;
end;`,
    complexity: {
      time: 'O(log2 N) en peor caso vs O(N) en búsqueda secuencial',
      space: 'O(1) auxiliar',
      docenteNote: 'En el final suelen preguntar: ¿Conviene ordenar un vector antes de buscar? Respuesta: Sólo si se van a realizar múltiples búsquedas, porque ordenar cuesta O(N log N) o O(N^2).'
    },
    steps: [
      {
        stepIndex: 0,
        explanation: 'Buscamos x = 42 en vector ordenado: [12, 24, 31, 42, 55, 68, 80]. pri = 1, ult = 7.',
        activeLine: 7,
        vector: [12, 24, 31, 42, 55, 68, 80],
        pointers: { pri: 0, ult: 6, medio: null }
      },
      {
        stepIndex: 1,
        explanation: 'Calculamos medio := (1 + 7) div 2 = 4 (índice 4, valor 42). Comparamos v[medio] con 42.',
        activeLine: 12,
        vector: [12, 24, 31, 42, 55, 68, 80],
        pointers: { pri: 0, ult: 6, medio: 3 }
      },
      {
        stepIndex: 2,
        explanation: '¡Encontrado en 1 sola comparación! v[4] = 42. encontre := true. Retorna posición 4.',
        activeLine: 14,
        vector: [12, 24, 31, 42, 55, 68, 80],
        pointers: { pri: 0, ult: 6, medio: 3, found: true }
      }
    ]
  },
  {
    id: 'vector-insertar-corrimiento',
    title: 'Inserción con Corrimiento en Vector',
    category: 'Vectores',
    difficulty: 'Filtro de Cursada y Final',
    description: 'Inserta un valor en una posición determinada desplazando todos los elementos siguientes una posición a la derecha. Verifica rigurosamente que dimL < dimF.',
    pascalCode: `procedure insertarEnVector(var v: vector; var dimL: integer; dimF: integer; pos: integer; elem: integer; var exito: boolean);
var
  i: integer;
begin
  exito := false;
  if (dimL < dimF) and (pos >= 1) and (pos <= dimL + 1) then
  begin
    for i := dimL downto pos do
      v[i + 1] := v[i];
    v[pos] := elem;
    dimL := dimL + 1;
    exito := true;
  end;
end;`,
    complexity: {
      time: 'O(N) debido al corrimiento de elementos en el peor caso (insertar en pos 1)',
      space: 'O(1) auxiliar',
      docenteNote: 'El bucle for DEBE ser downto. Si lo hacés ascendente to, sobreescribís todos los elementos con el primero.'
    },
    steps: [
      {
        stepIndex: 0,
        explanation: 'Vector con dimL = 4, dimF = 6: [10, 20, 40, 50, _, _]. Queremos insertar 30 en pos 3.',
        activeLine: 6,
        vector: [10, 20, 40, 50, null, null],
        dimL: 4,
        pointers: { pos: 2 }
      },
      {
        stepIndex: 1,
        explanation: 'Corrimiento downto: v[5] := v[4] (50 pasa a la pos 5).',
        activeLine: 10,
        vector: [10, 20, 40, 50, 50, null],
        dimL: 4,
        pointers: { i: 3 }
      },
      {
        stepIndex: 2,
        explanation: 'Corrimiento downto: v[4] := v[3] (40 pasa a la pos 4). El hueco en pos 3 queda libre.',
        activeLine: 10,
        vector: [10, 20, 40, 40, 50, null],
        dimL: 4,
        pointers: { i: 2 }
      },
      {
        stepIndex: 3,
        explanation: 'v[pos] := 30; dimL := dimL + 1; El vector queda [10, 20, 30, 40, 50, _] con dimL = 5.',
        activeLine: 11,
        vector: [10, 20, 30, 40, 50, null],
        dimL: 5,
        pointers: { pos: 2, inserted: true }
      }
    ]
  }
];
