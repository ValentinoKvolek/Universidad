export const FINALS_BANK = [
  {
    id: 'final-marzo-2026',
    date: '31 de Marzo de 2026',
    theme: 'Tema 2',
    overview: 'Examen de mesa reciente con Práctica de Simulación y Teórico de Traza + V/F + Análisis de IA.',
    practiceStatement: `1. Suponga que se quiere implementar un juego de cartas. De cada carta se conoce número (1..12) y palo al que pertenece (oro, copa, espada o basto). Hay dos niños que van a jugar a este juego y para eso disponen de su conjunto de cartas (como máximo 15). En cada ronda del juego los niños comparan entre sí la primera carta de su conjunto, y aquel niño que tiene la carta más chica pone al final de su conjunto de cartas su carta y la de su oponente (suponga que siempre una carta es más baja). El juego finaliza cuando alguno de los dos niños queda sin cartas para jugar. MODULARICE.`,
    practiceSolution: `program juego_cartas;
const
  MAX_CARTAS = 30; { DimF para que quepan las cartas ganadas }
type
  rangoNumero = 1..12;
  tipoPalo = (oro, copa, espada, basto);
  carta = record
    numero: rangoNumero;
    palo: tipoPalo;
  end;
  vectorCartas = array[1..MAX_CARTAS] of carta;
  mazo = record
    cartas: vectorCartas;
    dimL: integer;
  end;

{ Extrae y elimina la primera carta con corrimiento a la izquierda }
procedure sacarPrimeraCarta(var m: mazo; var c: carta);
var
  i: integer;
begin
  c := m.cartas[1];
  for i := 1 to m.dimL - 1 do
    m.cartas[i] := m.cartas[i + 1];
  m.dimL := m.dimL - 1;
end;

{ Agrega una carta al final del mazo }
procedure agregarAlFinal(var m: mazo; c: carta);
begin
  m.dimL := m.dimL + 1;
  m.cartas[m.dimL] := c;
end;

procedure jugarRonda(var nino1, nino2: mazo);
var
  c1, c2: carta;
begin
  sacarPrimeraCarta(nino1, c1);
  sacarPrimeraCarta(nino2, c2);
  
  if (c1.numero < c2.numero) then
  begin
    { Nino 1 tiene la carta mas chica: gana las dos }
    agregarAlFinal(nino1, c1);
    agregarAlFinal(nino1, c2);
  end
  else
  begin
    { Nino 2 tiene la carta mas chica: gana las dos }
    agregarAlFinal(nino2, c2);
    agregarAlFinal(nino2, c1);
  end;
end;

procedure simularJuego(var nino1, nino2: mazo);
begin
  while (nino1.dimL > 0) and (nino2.dimL > 0) do
    jugarRonda(nino1, nino2);
    
  if (nino1.dimL = 0) then
    writeln('El nino 1 se quedo sin cartas. Ganador: Nino 2!')
  else
    writeln('El nino 2 se quedo sin cartas. Ganador: Nino 1!');
end;`,
    docenteCriteria: [
      'Uso obligatorio de procedimientos modulares (sacar, agregar, jugarRonda).',
      'Corrimiento correcto hacia la izquierda para sacar la primera carta.',
      'Control riguroso de condición de corte en el bucle principal: (nino1.dimL > 0) and (nino2.dimL > 0).'
    ]
  },
  {
    id: 'final-feb-2024',
    date: '20 de Febrero de 2024',
    theme: 'Tema 1',
    overview: 'Examen clásico con vector de clientes capicúa, selección de encabezado de módulo (Procedure vs Function), y cálculo formal de memoria y UDT.',
    practiceStatement: `1. Una empresa dispone de una estructura de datos con los clientes de su comercio (500 a lo sumo). De cada cliente se conoce el número de cliente, número de DNI y monto que paga. Se pide implementar un programa que informe la cantidad de clientes cuyo número de DNI es capicúa.
    
2. Para la siguiente situación: "Dado un vector de punteros a registros de empleados con nombre y dni, se quiere realizar un módulo que retorne el puntero al empleado de nombre 'Juan García' que seguro existe." ¿Cuál/es opción/es es/son correcta/s? JUSTIFIQUE:
- Opción 1: Procedimiento con 2 parámetros: uno por valor (vector) y otro por referencia (^empleado).
- Opción 2: Función con un parámetro por valor (vector) que retorne el puntero (^empleado).
- Opción 3: El módulo podría ser una función o un procedimiento.`,
    practiceSolution: `{ Justificación del Punto 2 de Cátedra: }
La Opción 3 es la CORRECTA (puede ser tanto función como procedimiento), con la salvedad de que para estructuras de datos de gran tamaño se debe analizar la eficiencia:
1. Como FUNCIÓN: Es conceptualmente la más adecuada porque el objetivo es devolver un ÚNICO valor (un puntero, que es un tipo simple de 4 bytes).
2. Como PROCEDIMIENTO: También es sintácticamente correcta retornando el puntero en un parámetro de salida (var emp: ^empleado).
NOTA DOCENTE: Pasar el vector por valor (como indican las opciones 1 y 2) genera una copia completa en el Stack (overhead de memoria y tiempo). Sería más eficiente pasarlo por referencia (o const si el lenguaje lo permite), pero ambas opciones son válidas sintácticamente.`,
    docenteCriteria: [
      'Explicar que un puntero es un tipo de dato simple y por eso puede ser retornado por una función.',
      'Diferenciar entre corrección sintáctica y eficiencia de pasaje por valor de arreglos.'
    ]
  },
  {
    id: 'final-agosto-2025',
    date: 'Agosto de 2025',
    theme: 'Tema 2',
    overview: 'Corte de control sobre películas ordenadas por género, traza letal con colisión de variable global c y parámetro var b, y corrección de bugs en arreglos de punteros.',
    practiceStatement: `3. Dada la siguiente declaración y los procesos agregar1 y agregar2, indique para cada uno de ellos si agrega el elemento n al vector de manera correcta. Justifique su respuesta y, en caso de no ser correcta, indicar todos los problemas que presenta:
    
type
  rango = 1..2000;
  rdiml = 0..2000;
  vector = array [rango] of ^boolean;
  valores = record
    dato: vector;
    diml: rdiml;
  end;

procedure agregar1(var v: valores; n: boolean);
begin
  v.dato[v.diml]^ := n;
  v.diml := v.diml + 1;
end;

procedure agregar2(var v: valores; n: boolean);
begin
  v.dato[v.diml + 1]^ := n;
end;`,
    practiceSolution: `NINGUNO DE LOS DOS ES CORRECTO. Justificación detallada:

1. Fallas fatales en agregar1:
   a) Error de rango (Range Check Error / Fuera de límite): Si el vector está inicialmente vacío (v.diml = 0), intenta acceder a v.dato[0]^. Como rango es 1..2000, el índice 0 es INVÁLIDO.
   b) Puntero no alocado (Desreferenciación inválida): v.dato es un arreglo de ^boolean (punteros). ¡Nunca se hace new(v.dato[...])! Intentar desreferenciar v.dato[...]^ sobre un puntero sin inicializar o nil produce Run-time error (segmentation fault).
   c) No controla si el vector está lleno: Si diml = 2000, incrementa a 2001 (fuera del subrango rdiml).

2. Fallas fatales en agregar2:
   a) Nunca actualiza la dimensión lógica: Hace v.diml + 1 en el índice, pero NO incrementa v.diml. El vector permanece con la misma dimensión lógica original.
   b) Mismo error de desreferenciación: Intenta asignar a un puntero que no fue reservado con new().
   c) Desbordamiento: Si v.diml = 2000, v.diml + 1 = 2001, fuera de rango del vector.`,
    docenteCriteria: [
      'Mencionar explícitamente el tipo ^boolean (puntero) y la falta de new().',
      'Identificar el índice 0 cuando diml=0 en agregar1.',
      'Indicar que agregar2 no actualiza la dimensión lógica v.diml.'
    ]
  }
];
