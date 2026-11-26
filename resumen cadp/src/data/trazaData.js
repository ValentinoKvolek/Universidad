export const TRAZA_EXERCISES = [
  {
    id: 'traza-agosto-2025',
    title: 'Final Agosto 2025 - Tema 2: El Doble Aliasing Letal',
    examDate: 'Agosto 2025',
    difficulty: 'Nivel Experto (Trampa de Cátedra)',
    summary: 'La variable global "c" se pasa por referencia al parámetro "b", mientras que el procedimiento usa también la variable global "c". Modificar "b" modifica "c" en tiempo real.',
    code: `program tres;
var c: integer; { Variable GLOBAL }

procedure numero (var a: integer; var b: integer);
begin
  b := 30 DIV 4 + a;
  a := 29 MOD 5 + c;
  if ((a + c) > 10) then 
    b := b - c * 2
  else 
    b := b - c * 3;
  c := b + c;
  writeln('Valor a: ', a, ' Valor b: ', b, ' Valor c: ', c);
end;

var a, b: integer; { Variables locales del programa principal }
begin
  b := 7; c := 5;
  numero(b, c);
  writeln('Valor a: ', a, ' Valor b: ', b, ' Valor c: ', c);
end.`,
    steps: [
      {
        step: 0,
        line: 18,
        description: 'Programa Principal inicia. Global: c=5. Main locals: b=7, a=indefinido.',
        scope: 'main',
        vars: { 'global.c': 5, 'main.b': 7, 'main.a': '?' },
        output: ''
      },
      {
        step: 1,
        line: 19,
        description: 'Llamada numero(b, c). ¡ALERTA DE ALIASING!\n- Parámetro "a" (var) referencia a main.b.\n- Parámetro "b" (var) referencia a global.c.\n- ¡Dentro del procedimiento, el identificador "c" accede a global.c! Por lo tanto, el parámetro "b" y la global "c" SON LA MISMA CELDA DE MEMORIA.',
        scope: 'procedure numero',
        vars: { 'param.a (ref main.b)': 7, 'param.b (ref global.c)': 5, 'global.c': 5 },
        output: ''
      },
      {
        step: 2,
        line: 6,
        description: 'b := 30 DIV 4 + a;\n30 DIV 4 = 7. a = 7 -> 7 + 7 = 14.\nSe asigna 14 a b. Como b referencia a global.c, ¡global.c ahora vale 14!',
        scope: 'procedure numero',
        vars: { 'param.a (ref main.b)': 7, 'param.b (ref global.c)': 14, 'global.c': 14 },
        output: ''
      },
      {
        step: 3,
        line: 7,
        description: 'a := 29 MOD 5 + c;\n29 MOD 5 = 4. c (global) vale 14 -> 4 + 14 = 18.\nSe asigna 18 a a. Como a referencia a main.b, ¡main.b ahora vale 18!',
        scope: 'procedure numero',
        vars: { 'param.a (ref main.b)': 18, 'param.b (ref global.c)': 14, 'global.c': 14 },
        output: ''
      },
      {
        step: 4,
        line: 8,
        description: 'Condición if: ((a + c) > 10) -> (18 + 14 = 32 > 10) -> VERDADERO.\nSe ejecuta then: b := b - c * 2;\nb = 14, c = 14 -> 14 - (14 * 2) = 14 - 28 = -14.\nComo b es global.c, global.c pasa a valer -14.',
        scope: 'procedure numero',
        vars: { 'param.a (ref main.b)': 18, 'param.b (ref global.c)': -14, 'global.c': -14 },
        output: ''
      },
      {
        step: 5,
        line: 12,
        description: 'c := b + c;\n¡b y c son el mismo valor (-14)!\n-14 + (-14) = -28. Se asigna a c (y a b colateralmente): -28.',
        scope: 'procedure numero',
        vars: { 'param.a (ref main.b)': 18, 'param.b (ref global.c)': -28, 'global.c': -28 },
        output: ''
      },
      {
        step: 6,
        line: 13,
        description: 'writeln dentro del procedure. Imprime: Valor a: 18 Valor b: -28 Valor c: -28',
        scope: 'procedure numero',
        vars: { 'param.a': 18, 'param.b': -28, 'global.c': -28 },
        output: 'Valor a: 18 Valor b: -28 Valor c: -28'
      },
      {
        step: 7,
        line: 20,
        description: 'Retorno al main y writeln final.\n- main.a nunca fue asignada en el main (queda indefinida o basura de memoria).\n- main.b fue modificada por el parámetro "a": vale 18.\n- global.c fue modificada: vale -28.',
        scope: 'main',
        vars: { 'main.a': '?', 'main.b': 18, 'global.c': -28 },
        output: 'Valor a: 18 Valor b: -28 Valor c: -28\nValor a: ? Valor b: 18 Valor c: -28'
      }
    ]
  },
  {
    id: 'traza-marzo-2026',
    title: 'Final Marzo 2026: Parámetro por Valor vs Referencia y Global Shadowing',
    examDate: '31/03/2026',
    difficulty: 'Filtro Teórico Reciente',
    summary: 'Procedimiento actualizar con parámetro por valor (ant) y por referencia (nuevo, max), donde max colisiona con la variable global max.',
    code: `program ejercicio2;
var
  max: integer; { Global }
procedure actualizar(ant: integer; var nuevo: integer; var max: integer);
begin
  max := ant MOD 100;
  ant := nuevo div 2;
  if (max > nuevo) then nuevo := max * 3 + ant
  else nuevo := ant * 2 + max;
  writeln('Ant: ', ant, ' Nuevo: ', nuevo, ' Max: ', max);
end;

var inf, sup: integer;
begin
  max := 23; inf := 160; sup := 40;
  actualizar(max, inf, sup);
  writeln('Inf: ', inf, ' Sup: ', sup, ' Max: ', max);
end.`,
    steps: [
      {
        step: 0,
        line: 14,
        description: 'Estado inicial de main: global max = 23, inf = 160, sup = 40.',
        scope: 'main',
        vars: { 'global.max': 23, 'main.inf': 160, 'main.sup': 40 },
        output: ''
      },
      {
        step: 1,
        line: 15,
        description: 'Llamada actualizar(max, inf, sup):\n- "ant" recibe por VALOR el número 23 (copia local).\n- "nuevo" recibe por REFERENCIA a inf (apunta a main.inf).\n- "max" recibe por REFERENCIA a sup (apunta a main.sup).\n- ¡ATENCIÓN! La variable global max queda OCULTA (shadowed) dentro de actualizar porque hay un parámetro formal llamado "max".',
        scope: 'procedure actualizar',
        vars: { 'param.ant (val)': 23, 'param.nuevo (ref inf)': 160, 'param.max (ref sup)': 40, 'global.max': 23 },
        output: ''
      },
      {
        step: 2,
        line: 5,
        description: 'max := ant MOD 100;\n23 MOD 100 = 23.\nSe asigna 23 al parámetro max (que es main.sup). Por lo tanto, sup pasa a valer 23.',
        scope: 'procedure actualizar',
        vars: { 'param.ant (val)': 23, 'param.nuevo (ref inf)': 160, 'param.max (ref sup)': 23, 'global.max': 23 },
        output: ''
      },
      {
        step: 3,
        line: 6,
        description: 'ant := nuevo div 2;\n160 div 2 = 80.\nSe asigna 80 a ant. Como ant es por VALOR, sólo cambia la copia local; la variable original max del main NO se modifica.',
        scope: 'procedure actualizar',
        vars: { 'param.ant (val)': 80, 'param.nuevo (ref inf)': 160, 'param.max (ref sup)': 23, 'global.max': 23 },
        output: ''
      },
      {
        step: 4,
        line: 7,
        description: 'Condición if: (max > nuevo) -> (23 > 160) -> FALSO.\nSe ejecuta else: nuevo := ant * 2 + max;\nant = 80, max = 23 -> 80 * 2 + 23 = 160 + 23 = 183.\nComo nuevo es por referencia a inf, ¡main.inf pasa a valer 183!',
        scope: 'procedure actualizar',
        vars: { 'param.ant (val)': 80, 'param.nuevo (ref inf)': 183, 'param.max (ref sup)': 23, 'global.max': 23 },
        output: ''
      },
      {
        step: 5,
        line: 9,
        description: 'writeln dentro de actualizar:\nImprime: Ant: 80 Nuevo: 183 Max: 23',
        scope: 'procedure actualizar',
        vars: { 'ant': 80, 'nuevo': 183, 'max': 23 },
        output: 'Ant: 80 Nuevo: 183 Max: 23'
      },
      {
        step: 6,
        line: 16,
        description: 'Retorno a main y writeln final:\n- inf cambió a 183.\n- sup cambió a 23.\n- global max NUNCA cambió, sigue valiendo 23.\nImprime: Inf: 183 Sup: 23 Max: 23',
        scope: 'main',
        vars: { 'main.inf': 183, 'main.sup': 23, 'global.max': 23 },
        output: 'Ant: 80 Nuevo: 183 Max: 23\nInf: 183 Sup: 23 Max: 23'
      }
    ]
  }
];
