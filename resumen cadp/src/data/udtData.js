export const UDT_RULES = [
  {
    name: 'Asignación simple',
    formula: '1 \\text{ UDT}',
    example: 'x := 10;',
    explanation: 'Copiar o asignar un valor simple a una celda de memoria toma 1 Unidad de Tiempo.'
  },
  {
    name: 'Evaluación de Condición',
    formula: 'C \\text{ UDT}',
    example: 'a > 10 \\quad \\text{(1 UDT)} \\qquad (a > 10) \\text{ and } (b < 5) \\quad \\text{(3 UDT: 2 comparaciones + 1 operador lógico)}',
    explanation: 'Cada comparación cuesta 1 UDT. Cada conectivo lógico (and, or, not) suma 1 UDT adicional.'
  },
  {
    name: 'Estructura IF-THEN / ELSE',
    formula: 'C + \\text{Cuerpo Peor Caso}',
    example: 'if (cond) then accion1 else accion2;',
    explanation: 'Se evalúa la condición (C) y se suma el tiempo de la rama más costosa (análisis de peor caso exigido por la cátedra).'
  },
  {
    name: 'Bucle FOR ascendente/descendente',
    formula: '(3N + 2) + N \\times (\\text{Cuerpo})',
    example: 'for i := A to B do { ... } \\quad N = B - A + 1',
    explanation: 'El overhead del FOR de cátedra es (3N + 2), derivado de: 1 asignación inicial (i:=A) + (N+1) comparaciones de cota + 2N (incrementos y asignaciones de paso) = 3N + 2.'
  },
  {
    name: 'Bucle WHILE',
    formula: 'C \\times (N + 1) + N \\times (\\text{Cuerpo})',
    example: 'while (condicion) do { ... } \\quad (N \\ge 0)',
    explanation: 'La condición se evalúa N veces verdaderas + 1 vez falsa final que rompe el ciclo = (N + 1) evaluaciones.'
  },
  {
    name: 'Bucle REPEAT..UNTIL',
    formula: 'N \\times (\\text{Cuerpo}) + C \\times N',
    example: 'repeat { ... } until (condicion); \\quad (N \\ge 1)',
    explanation: 'El cuerpo se ejecuta primero al menos una vez, y la condición se evalúa exactamente N veces.'
  }
];

export const MEMORY_CALCULATOR_DEFAULT_TYPES = [
  { typeName: 'Char', defaultBytes: 1 },
  { typeName: 'Integer', defaultBytes: 6, note: 'Varía según el examen: en Dic 2023 dieron 8B, en Feb 2024 dieron 6B, estándar 4B' },
  { typeName: 'Real', defaultBytes: 10, note: 'En Dic 2023 dieron 12B, en Feb 2024 dieron 10B, estándar 8B' },
  { typeName: 'Boolean', defaultBytes: 1 },
  { typeName: 'Puntero (^T)', defaultBytes: 4, note: 'Siempre 4 bytes en estática (dirección de 32 bits)' },
  { typeName: 'String', defaultBytes: 256, note: 'Longitud + 1 byte (si no está acotado string[N] ocupa 255 + 1 = 256 bytes)' }
];

export const REAL_EXAM_MEMORY_PROBLEMS = [
  {
    id: 'final-feb-2024',
    title: 'Examen Final 20/02/2024 - Tema 1 (Punto 3 y 4)',
    typeTable: {
      Char: 1,
      Integer: 6,
      Real: 10,
      Boolean: 1,
      String: 256,
      Puntero: 4
    },
    code: `program ejercicio3;
const fin = 20;
type 
  info = record
    nombre: string;     { 256 bytes }
    nota: integer;       { 6 bytes }
    datos: ^integer;     { 4 bytes }
  end;                   { TOTAL info = 266 bytes }
  vector = array [5..fin] of ^info; { 16 elementos * 4B = 64 bytes }
var 
  v: vector;             { 64 bytes }
  i, j: integer;         { 6B + 6B = 12 bytes }
  e: info;               { 266 bytes }
begin                    { TOTAL ESTÁTICA = 64 + 12 + 266 = 342 bytes }
  for i := 5 to 15 do begin
    read(e.nombre); read(e.nota);
    new(v[i]);
    if (e.nota MOD 3 = 0) then
      v[i] := nil
    else begin
      v[i]^.nombre := e.nombre;
      v[i]^.nota := e.nota + (e.nota MOD 5);
    end;
  end;
  for j := 5 to 10 do dispose(v[j]);
end.`,
    estaticaExplanation: [
      { item: 'v (vector [5..20] de ^info)', calc: '(20 - 5 + 1) * 4 bytes = 16 * 4B', result: 64 },
      { item: 'i, j (2 enteros)', calc: '2 * 6 bytes', result: 12 },
      { item: 'e (record info: string 256B + int 6B + ptr 4B)', calc: '256 + 6 + 4', result: 266 },
      { item: 'TOTAL MEMORIA ESTÁTICA', calc: '64 + 12 + 266', result: 342 }
    ],
    dinamicaExplanation: [
      { item: 'Ciclo 1: for i := 5 to 15', calc: 'Se ejecuta 11 veces (15 - 5 + 1). En cada iteración se invoca new(v[i]), reservando sizeof(info) = 266 bytes en el Heap. Total pedido = 11 * 266B = 2.926 bytes.', result: 2926 },
      { item: 'Trampa v[i] := nil', note: 'Asignar nil NO libera memoria, genera fugas (leaks). Sigue ocupando espacio en el Heap.' },
      { item: 'Ciclo 2: for j := 5 to 10 do dispose(v[j])', note: '6 iteraciones (10 - 5 + 1). Si alguna posición tenía nil, ¡provoca RUN-TIME ERROR! En el mejor caso teórico donde todos eran no-nil: 6 * 266B = 1.596 bytes liberados.' },
      { item: 'Memoria Dinámica Final (en caso sin crash)', calc: '2.926B - 1.596B', result: 1330 }
    ],
    udtBreakdown: [
      { block: 'Primer FOR (i:=5 to 15)', N: 11, calcOverhead: '3(11) + 2 = 35 UDT', bodyUDT: 'read(1) + read(1) + new(1) + if_cond(2) + rama_peor(4) = 9 UDT', total: '35 + 11*9 = 134 UDT' },
      { block: 'Segundo FOR (j:=5 to 10)', N: 6, calcOverhead: '3(6) + 2 = 20 UDT', bodyUDT: 'dispose = 1 UDT', total: '20 + 6*1 = 26 UDT' },
      { block: 'TOTAL UDT DEL PROGRAMA', total: '134 + 26 = 160 UDT' }
    ]
  }
];
