export const MEMORY_SCENARIOS = [
  {
    id: 'trampa-examen-2024',
    title: 'La Gran Trampa de Final: new() seguido de nil y dispose()',
    examReference: 'Final Febrero 2024 / Diciembre 2023',
    description: 'En el examen te dan un bucle donde hacen new(v[i]), y si una condición se cumple hacen v[i] := nil. Luego intentan hacer dispose(v[j]). ¿Qué pasa con la memoria y qué errores ocurren?',
    code: `for i := 5 to 15 do begin
  new(v[i]);
  if (e.nota MOD 3 = 0) then
    v[i] := nil
  else
    v[i]^.nombre := e.nombre;
end;
for j := 5 to 10 do dispose(v[j]);`,
    steps: [
      {
        step: 0,
        label: 'Inicio',
        desc: 'Tenemos el vector estático v en el Stack (16 elementos, cada uno es un puntero de 4 bytes). Ninguno apunta a nada aún (indefinidos o nil).',
        stack: [
          { name: 'v[5]', val: 'nil', type: '^info (4B)' },
          { name: 'v[6]', val: 'nil', type: '^info (4B)' },
          { name: 'v[7]', val: 'nil', type: '^info (4B)' }
        ],
        heap: []
      },
      {
        step: 1,
        label: 'new(v[5]) con nota=9 (9 MOD 3 = 0)',
        desc: '1. new(v[5]) pide un bloque de memoria para record info en el Heap (dirección 0x100).\n2. Pero como (9 MOD 3 = 0), ejecuta v[5] := nil;\n3. ¡ATENCIÓN! La flecha se corta, pero el bloque en 0x100 NO SE LIBERÓ: ¡FUGA DE MEMORIA (Memory Leak)!',
        stack: [
          { name: 'v[5]', val: 'nil', type: '^info (4B)', leak: true },
          { name: 'v[6]', val: 'nil', type: '^info (4B)' },
          { name: 'v[7]', val: 'nil', type: '^info (4B)' }
        ],
        heap: [
          { address: '0x100', size: '264B', status: 'LEAK', content: 'info sin puntero' }
        ]
      },
      {
        step: 2,
        label: 'new(v[6]) con nota=8 (8 MOD 3 <> 0)',
        desc: 'new(v[6]) aloca memoria en 0x180. La condición es falsa, así que asigna el nombre. v[6] apunta válidamente al bloque.',
        stack: [
          { name: 'v[5]', val: 'nil', type: '^info (4B)', leak: true },
          { name: 'v[6]', val: '0x180', type: '^info (4B)', valid: true },
          { name: 'v[7]', val: 'nil', type: '^info (4B)' }
        ],
        heap: [
          { address: '0x100', size: '264B', status: 'LEAK', content: 'Fuga huérfana' },
          { address: '0x180', size: '264B', status: 'ACTIVO', content: 'nombre: "Ana", nota: 8' }
        ]
      },
      {
        step: 3,
        label: 'Ejecución de dispose(v[5]) en el segundo bucle',
        desc: '¡CRASH EN TIEMPO DE EJECUCIÓN (Run-time Error)! v[5] vale nil. En Pascal estándar, invocar dispose() sobre un puntero con valor nil o no inicializado provoca un error fatal de acceso o runtime 204.',
        stack: [
          { name: 'v[5]', val: 'nil', type: '^info (4B)', error: true },
          { name: 'v[6]', val: '0x180', type: '^info (4B)' }
        ],
        heap: [
          { address: '0x100', size: '264B', status: 'LEAK', content: 'Fuga huérfana' },
          { address: '0x180', size: '264B', status: 'ACTIVO', content: 'nombre: "Ana", nota: 8' }
        ]
      }
    ]
  },
  {
    id: 'dangling-pointer',
    title: 'Puntero Colgante (Dangling Pointer) y Aliasing en Heap',
    examReference: 'Preguntas teóricas y Justificaciones V/F',
    description: 'Cuando dos punteros apuntan al mismo bloque dinámico y uno hace dispose(), el otro queda apuntando a memoria liberada o reciclada.',
    code: `new(p1);
p1^.dato := 50;
p2 := p1;       { Ambos apuntan al mismo bloque }
dispose(p1);    { Se libera el bloque }
writeln(p2^.dato); { ¡PELIGRO! p2 es un puntero colgante }`,
    steps: [
      {
        step: 0,
        label: 'new(p1) y p1^.dato := 50',
        desc: 'p1 (en Stack) apunta al bloque 0x200 (en Heap).',
        stack: [
          { name: 'p1', val: '0x200', type: '^nodo' },
          { name: 'p2', val: 'indefinido', type: '^nodo' }
        ],
        heap: [
          { address: '0x200', size: '8B', status: 'ACTIVO', content: 'dato: 50, sig: nil' }
        ]
      },
      {
        step: 1,
        label: 'p2 := p1',
        desc: 'Asignación de punteros: p2 copia la dirección 0x200. No se crea memoria nueva, ambos comparten el mismo nodo en el Heap.',
        stack: [
          { name: 'p1', val: '0x200', type: '^nodo' },
          { name: 'p2', val: '0x200', type: '^nodo' }
        ],
        heap: [
          { address: '0x200', size: '8B', status: 'ACTIVO', content: 'dato: 50, sig: nil' }
        ]
      },
      {
        step: 2,
        label: 'dispose(p1)',
        desc: 'dispose libera el bloque 0x200 en el Heap. ¡Pero ojo! p2 en el Stack sigue teniendo guardado el número 0x200. p2 se transformó en un "Puntero Colgante".',
        stack: [
          { name: 'p1', val: '0x200 (liberado)', type: '^nodo', dangling: true },
          { name: 'p2', val: '0x200 (colgante)', type: '^nodo', dangling: true }
        ],
        heap: [
          { address: '0x200', size: '0B', status: 'LIBERADO', content: 'Memoria devuelta al SO' }
        ]
      },
      {
        step: 3,
        label: 'writeln(p2^.dato) -> Error Semántico/Ejecución',
        desc: 'Intentar desreferenciar p2^ produce comportamiento indefinido o violación de acceso (segmentation fault). Para evitarlo se debe hacer p1 := nil y p2 := nil inmediatamente después de dispose.',
        stack: [
          { name: 'p1', val: '0x200', type: '^nodo' },
          { name: 'p2', val: '0x200', type: '^nodo', error: true }
        ],
        heap: []
      }
    ]
  }
];
