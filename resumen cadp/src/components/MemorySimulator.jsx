import React, { useState } from 'react';
import { MEMORY_SCENARIOS } from '../data/memoryData';
import { Layers, Database, AlertOctagon, CheckCircle2, RotateCcw, ArrowRight, ShieldAlert, Cpu } from 'lucide-react';

export default function MemorySimulator() {
  const [selectedScenarioId, setSelectedScenarioId] = useState(MEMORY_SCENARIOS[0].id);
  const [stepIndex, setStepIndex] = useState(0);

  // Sandbox state
  const [sandboxMode, setSandboxMode] = useState(false);
  const [sandboxStack, setSandboxStack] = useState({ p1: 'nil', p2: 'nil' });
  const [sandboxHeap, setSandboxHeap] = useState([]);
  const [sandboxLogs, setSandboxLogs] = useState(['Sandbox iniciado. p1 y p2 valen nil en el Stack.']);

  const scenario = MEMORY_SCENARIOS.find(s => s.id === selectedScenarioId) || MEMORY_SCENARIOS[0];
  const currentStep = scenario.steps[stepIndex] || scenario.steps[0];

  // Sandbox actions
  const handleSandboxNew = (varName) => {
    const newAddr = '0x' + Math.floor(Math.random() * 899 + 100).toString(16).toUpperCase();
    setSandboxHeap(prev => [...prev, { address: newAddr, size: '256B', content: `Nodo alocado para ${varName}`, status: 'ACTIVO' }]);
    setSandboxStack(prev => ({ ...prev, [varName]: newAddr }));
    setSandboxLogs(prev => [`new(${varName}): Se reservó bloque en Heap (${newAddr}) y ${varName} apunta a él.`, ...prev]);
  };

  const handleSandboxNil = (varName) => {
    const oldAddr = sandboxStack[varName];
    setSandboxStack(prev => ({ ...prev, [varName]: 'nil' }));
    if (oldAddr && oldAddr !== 'nil') {
      // Check if any other variable still points to this address
      const otherPoints = Object.entries(sandboxStack).some(([k, v]) => k !== varName && v === oldAddr);
      if (!otherPoints) {
        // Mark as leak in heap
        setSandboxHeap(prev => prev.map(h => h.address === oldAddr ? { ...h, status: 'LEAK' } : h));
        setSandboxLogs(prev => [`⚠️ ${varName} := nil: ¡FUGA DE MEMORIA! Nadie apunta al bloque ${oldAddr} y nunca se hizo dispose().`, ...prev]);
        return;
      }
    }
    setSandboxLogs(prev => [`${varName} := nil: Enlace cortado a nil.`, ...prev]);
  };

  const handleSandboxDispose = (varName) => {
    const targetAddr = sandboxStack[varName];
    if (targetAddr === 'nil' || !targetAddr) {
      setSandboxLogs(prev => [`🚨 ERROR FATAL: dispose(${varName}) sobre un puntero nil provoca RUN-TIME ERROR (cuelgue).`, ...prev]);
      return;
    }
    // Remove from heap
    setSandboxHeap(prev => prev.filter(h => h.address !== targetAddr));
    setSandboxLogs(prev => [
      `dispose(${varName}): Memoria ${targetAddr} liberada del Heap. ¡Ojo! ${varName} sigue valiendo ${targetAddr} en el Stack (Puntero Colgante) hasta que le asignes nil.`,
      ...prev
    ]);
  };

  const handleSandboxReset = () => {
    setSandboxStack({ p1: 'nil', p2: 'nil' });
    setSandboxHeap([]);
    setSandboxLogs(['Sandbox reiniciado. p1 y p2 valen nil.']);
  };

  return (
    <div className="space-y-6">
      {/* Banner */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-2xl p-6 shadow-xl">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div>
            <h2 className="text-2xl font-bold text-white tracking-tight">
              Simulador Visual Stack (Estática) vs Heap (Dinámica)
            </h2>
            <p className="text-slate-300 text-sm mt-1 max-w-3xl">
              Comprendé con precisión quirúrgica dónde vive cada byte: las variables locales en la Pila (<strong className="text-purple-300">Stack</strong>) y los nodos dinámicos en el Montículo (<strong className="text-emerald-300">Heap</strong>). Visualizá fugas de memoria y punteros colgantes.
            </p>
          </div>

          <div className="flex gap-2">
            <button
              onClick={() => setSandboxMode(false)}
              className={`px-4 py-2 rounded-xl text-xs font-semibold transition ${
                !sandboxMode
                  ? 'bg-purple-600 text-white shadow-lg shadow-purple-500/20'
                  : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
              }`}
            >
              Casos de Examen Real
            </button>
            <button
              onClick={() => setSandboxMode(true)}
              className={`px-4 py-2 rounded-xl text-xs font-semibold transition ${
                sandboxMode
                  ? 'bg-purple-600 text-white shadow-lg shadow-purple-500/20'
                  : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
              }`}
            >
              Laboratorio Libre (Sandbox)
            </button>
          </div>
        </div>

        {!sandboxMode && (
          <div className="flex flex-wrap gap-2 mt-6 pt-5 border-t border-slate-800">
            {MEMORY_SCENARIOS.map(s => (
              <button
                key={s.id}
                onClick={() => {
                  setSelectedScenarioId(s.id);
                  setStepIndex(0);
                }}
                className={`px-4 py-2 rounded-xl text-xs font-medium transition ${
                  selectedScenarioId === s.id
                    ? 'bg-purple-600/90 text-white border border-purple-400/40 font-semibold shadow-md'
                    : 'bg-slate-800/80 text-slate-300 hover:bg-slate-700 border border-slate-700/60'
                }`}
              >
                {s.title}
              </button>
            ))}
          </div>
        )}
      </div>

      {!sandboxMode ? (
        /* Modo Casos de Examen */
        <div className="space-y-6">
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
            {/* Visualizer: Stack vs Heap (8 cols) */}
            <div className="lg:col-span-8 bg-slate-900 border border-slate-800 rounded-2xl p-6 shadow-lg space-y-6">
              <div className="flex items-center justify-between pb-4 border-b border-slate-800">
                <div>
                  <h3 className="font-bold text-white text-base flex items-center gap-2">
                    <Layers className="w-5 h-5 text-purple-400" />
                    {scenario.title}
                  </h3>
                  <span className="text-xs text-purple-300 font-mono">
                    Ref: {scenario.examReference}
                  </span>
                </div>
                <div className="text-xs px-3 py-1 bg-slate-800 rounded-lg text-slate-300 font-mono border border-slate-700">
                  Paso {stepIndex + 1} de {scenario.steps.length}
                </div>
              </div>

              {/* The Two Memory Columns */}
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {/* Stack Column */}
                <div className="bg-slate-950/80 border border-purple-900/40 rounded-xl p-4 flex flex-col">
                  <div className="flex items-center justify-between pb-3 mb-3 border-b border-purple-900/30">
                    <div className="flex items-center gap-2">
                      <div className="w-3 h-3 rounded-full bg-purple-500 animate-pulse"></div>
                      <span className="text-xs font-bold uppercase tracking-wider text-purple-300 font-mono">
                        Memoria Estática (Stack)
                      </span>
                    </div>
                    <span className="text-[10px] text-slate-400 font-mono">Tiempo de Compilación</span>
                  </div>

                  <div className="space-y-2 flex-1">
                    {currentStep.stack.map((item, idx) => (
                      <div
                        key={idx}
                        className={`p-3 rounded-xl border transition-all flex items-center justify-between font-mono text-xs ${
                          item.error
                            ? 'bg-rose-950/50 border-rose-500 text-rose-200'
                            : item.leak
                            ? 'bg-amber-950/40 border-amber-500 text-amber-200'
                            : item.dangling
                            ? 'bg-orange-950/40 border-orange-500 text-orange-200'
                            : 'bg-slate-900 border-slate-800 text-slate-200'
                        }`}
                      >
                        <div>
                          <span className="font-bold text-slate-100">{item.name}</span>
                          <span className="text-[10px] text-slate-400 block">{item.type}</span>
                        </div>
                        <div className="text-right">
                          <span className="px-2 py-0.5 rounded bg-slate-950 border border-slate-700 text-slate-300">
                            {item.val}
                          </span>
                        </div>
                      </div>
                    ))}
                  </div>
                  <p className="text-[11px] text-slate-500 mt-3 pt-2 border-t border-slate-900">
                    Las variables puntero ocupan 4 bytes estáticos en el Stack.
                  </p>
                </div>

                {/* Heap Column */}
                <div className="bg-slate-950/80 border border-emerald-900/40 rounded-xl p-4 flex flex-col">
                  <div className="flex items-center justify-between pb-3 mb-3 border-b border-emerald-900/30">
                    <div className="flex items-center gap-2">
                      <div className="w-3 h-3 rounded-full bg-emerald-500"></div>
                      <span className="text-xs font-bold uppercase tracking-wider text-emerald-300 font-mono">
                        Memoria Dinámica (Heap)
                      </span>
                    </div>
                    <span className="text-[10px] text-slate-400 font-mono">Tiempo de Ejecución</span>
                  </div>

                  <div className="space-y-2 flex-1">
                    {currentStep.heap.length === 0 ? (
                      <div className="h-28 flex items-center justify-center text-xs text-slate-500 italic border border-dashed border-slate-800 rounded-xl">
                        Montículo vacío (0 bloques alocados)
                      </div>
                    ) : (
                      currentStep.heap.map((h, idx) => (
                        <div
                          key={idx}
                          className={`p-3 rounded-xl border transition-all text-xs font-mono space-y-1 ${
                            h.status === 'LEAK'
                              ? 'bg-amber-950/60 border-amber-500 text-amber-200 ring-2 ring-amber-500/20'
                              : 'bg-emerald-950/30 border-emerald-600/50 text-emerald-200'
                          }`}
                        >
                          <div className="flex justify-between items-center">
                            <span className="font-bold text-slate-100">{h.address}</span>
                            <span className={`text-[10px] px-1.5 py-0.5 rounded font-bold ${
                              h.status === 'LEAK' ? 'bg-amber-500 text-slate-950' : 'bg-emerald-500/20 text-emerald-300'
                            }`}>
                              {h.status}
                            </span>
                          </div>
                          <div className="text-[11px] text-slate-300">{h.content}</div>
                          <div className="text-[10px] text-slate-400">Tamaño: {h.size}</div>
                        </div>
                      ))
                    )}
                  </div>
                  <p className="text-[11px] text-slate-500 mt-3 pt-2 border-t border-slate-900">
                    Creado con new() y liberado únicamente con dispose().
                  </p>
                </div>
              </div>

              {/* Step Explanation Callout */}
              <div className="p-4 bg-slate-950 border border-purple-900/40 rounded-xl flex items-start gap-3">
                <CheckCircle2 className="w-5 h-5 text-purple-400 shrink-0 mt-0.5" />
                <div>
                  <h4 className="text-xs font-bold uppercase tracking-wider text-purple-300 font-mono">
                    {currentStep.label}
                  </h4>
                  <p className="text-slate-200 text-sm mt-1 whitespace-pre-line leading-relaxed">
                    {currentStep.desc}
                  </p>
                </div>
              </div>

              {/* Stepper Controls */}
              <div className="flex items-center justify-between pt-4 border-t border-slate-800">
                <button
                  onClick={() => setStepIndex(0)}
                  className="p-2 text-slate-400 hover:text-white hover:bg-slate-800 rounded-lg transition"
                >
                  <RotateCcw className="w-4 h-4" />
                </button>

                <div className="flex gap-2">
                  <button
                    disabled={stepIndex === 0}
                    onClick={() => setStepIndex(prev => Math.max(0, prev - 1))}
                    className="px-3 py-1.5 text-xs font-medium bg-slate-800 hover:bg-slate-700 disabled:opacity-40 text-slate-200 rounded-lg transition"
                  >
                    Anterior
                  </button>
                  <button
                    disabled={stepIndex === scenario.steps.length - 1}
                    onClick={() => setStepIndex(prev => Math.min(scenario.steps.length - 1, prev + 1))}
                    className="px-4 py-1.5 text-xs font-semibold bg-purple-600 hover:bg-purple-500 text-white rounded-lg transition flex items-center gap-1 shadow-md shadow-purple-500/20"
                  >
                    Siguiente Paso <ArrowRight className="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>
            </div>

            {/* Right: Code & Docente Trap Warnings (4 cols) */}
            <div className="lg:col-span-4 space-y-4">
              <div className="bg-slate-900 border border-slate-800 rounded-2xl p-5 shadow-lg space-y-4">
                <h4 className="text-xs font-mono font-bold text-slate-400 uppercase tracking-wider">
                  Código de Examen
                </h4>
                <pre className="p-3 bg-slate-950 border border-slate-800 rounded-xl font-mono text-xs text-purple-200 overflow-x-auto leading-relaxed">
                  {scenario.code}
                </pre>
              </div>

              <div className="bg-rose-950/30 border border-rose-800/40 rounded-2xl p-5 shadow-lg space-y-2">
                <div className="flex items-center gap-2 text-rose-400 font-bold text-sm">
                  <AlertOctagon className="w-4 h-4" />
                  Las 3 Reglas de Oro de Cátedra:
                </div>
                <ul className="text-xs text-rose-200/90 space-y-2 list-disc pl-4 leading-relaxed">
                  <li>
                    <strong>new(p)</strong> pide memoria en Heap. Si hacés <strong>p := nil</strong> después, la memoria sigue ocupada y perdiste la dirección: <span className="underline">Fuga de Memoria</span>.
                  </li>
                  <li>
                    <strong>dispose(p)</strong> libera la memoria apuntada en el Heap, pero la variable <strong>p en el Stack sigue existiendo</strong> (ocupando 4 bytes).
                  </li>
                  <li>
                    Hacer <strong>dispose() sobre un puntero nil</strong> provoca <em>Run-time error 204</em> y desaprueba el ejercicio.
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </div>
      ) : (
        /* Sandbox Libre */
        <div className="bg-slate-900 border border-slate-800 rounded-2xl p-6 shadow-xl space-y-6">
          <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 pb-4 border-b border-slate-800">
            <div>
              <h3 className="font-bold text-white text-lg flex items-center gap-2">
                <Database className="w-5 h-5 text-emerald-400" />
                Laboratorio Interactivo de Punteros en Tiempo Real
              </h3>
              <p className="text-xs text-slate-400">
                Experimentá invocando new, dispose y nil sobre dos punteros (p1 y p2).
              </p>
            </div>
            <button
              onClick={handleSandboxReset}
              className="px-3 py-1.5 text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 rounded-lg flex items-center gap-1.5 self-start md:self-auto"
            >
              <RotateCcw className="w-3.5 h-3.5" /> Reiniciar Sandbox
            </button>
          </div>

          {/* Interactive Controls Bar */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {/* Controls for p1 */}
            <div className="p-4 bg-slate-950/70 border border-slate-800 rounded-xl space-y-3">
              <span className="text-xs font-mono font-bold text-purple-300 block">Acciones sobre p1:</span>
              <div className="flex flex-wrap gap-2">
                <button
                  onClick={() => handleSandboxNew('p1')}
                  className="px-3 py-1.5 bg-emerald-600/80 hover:bg-emerald-500 text-white rounded-lg text-xs font-mono font-semibold transition"
                >
                  new(p1)
                </button>
                <button
                  onClick={() => handleSandboxDispose('p1')}
                  className="px-3 py-1.5 bg-rose-600/80 hover:bg-rose-500 text-white rounded-lg text-xs font-mono font-semibold transition"
                >
                  dispose(p1)
                </button>
                <button
                  onClick={() => handleSandboxNil('p1')}
                  className="px-3 py-1.5 bg-amber-600/80 hover:bg-amber-500 text-white rounded-lg text-xs font-mono font-semibold transition"
                >
                  p1 := nil
                </button>
                <button
                  onClick={() => {
                    setSandboxStack(prev => ({ ...prev, p1: prev.p2 }));
                    setSandboxLogs(prev => [`p1 := p2: p1 ahora comparte la dirección ${sandboxStack.p2} con p2.`, ...prev]);
                  }}
                  className="px-3 py-1.5 bg-blue-600/80 hover:bg-blue-500 text-white rounded-lg text-xs font-mono font-semibold transition"
                >
                  p1 := p2
                </button>
              </div>
            </div>

            {/* Controls for p2 */}
            <div className="p-4 bg-slate-950/70 border border-slate-800 rounded-xl space-y-3">
              <span className="text-xs font-mono font-bold text-blue-300 block">Acciones sobre p2:</span>
              <div className="flex flex-wrap gap-2">
                <button
                  onClick={() => handleSandboxNew('p2')}
                  className="px-3 py-1.5 bg-emerald-600/80 hover:bg-emerald-500 text-white rounded-lg text-xs font-mono font-semibold transition"
                >
                  new(p2)
                </button>
                <button
                  onClick={() => handleSandboxDispose('p2')}
                  className="px-3 py-1.5 bg-rose-600/80 hover:bg-rose-500 text-white rounded-lg text-xs font-mono font-semibold transition"
                >
                  dispose(p2)
                </button>
                <button
                  onClick={() => handleSandboxNil('p2')}
                  className="px-3 py-1.5 bg-amber-600/80 hover:bg-amber-500 text-white rounded-lg text-xs font-mono font-semibold transition"
                >
                  p2 := nil
                </button>
                <button
                  onClick={() => {
                    setSandboxStack(prev => ({ ...prev, p2: prev.p1 }));
                    setSandboxLogs(prev => [`p2 := p1: p2 ahora comparte la dirección ${sandboxStack.p1} con p1.`, ...prev]);
                  }}
                  className="px-3 py-1.5 bg-blue-600/80 hover:bg-blue-500 text-white rounded-lg text-xs font-mono font-semibold transition"
                >
                  p2 := p1
                </button>
              </div>
            </div>
          </div>

          {/* Sandbox Live View */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* Stack */}
            <div className="p-4 bg-slate-950 border border-slate-800 rounded-xl">
              <h4 className="text-xs font-mono font-bold text-purple-400 uppercase mb-3">Stack (Estática)</h4>
              <div className="space-y-2">
                <div className="p-3 bg-slate-900 border border-slate-800 rounded-lg flex justify-between font-mono text-xs">
                  <span className="font-bold text-purple-300">p1</span>
                  <span className="text-white px-2 py-0.5 bg-slate-950 rounded">{sandboxStack.p1}</span>
                </div>
                <div className="p-3 bg-slate-900 border border-slate-800 rounded-lg flex justify-between font-mono text-xs">
                  <span className="font-bold text-blue-300">p2</span>
                  <span className="text-white px-2 py-0.5 bg-slate-950 rounded">{sandboxStack.p2}</span>
                </div>
              </div>
            </div>

            {/* Heap */}
            <div className="p-4 bg-slate-950 border border-slate-800 rounded-xl">
              <h4 className="text-xs font-mono font-bold text-emerald-400 uppercase mb-3">Heap (Dinámica)</h4>
              <div className="space-y-2 min-h-[90px]">
                {sandboxHeap.length === 0 ? (
                  <div className="text-xs text-slate-500 italic py-6 text-center">Heap vacío</div>
                ) : (
                  sandboxHeap.map((h, i) => (
                    <div
                      key={i}
                      className={`p-2.5 rounded-lg border font-mono text-xs flex justify-between items-center ${
                        h.status === 'LEAK'
                          ? 'bg-amber-950/60 border-amber-500 text-amber-200'
                          : 'bg-emerald-950/30 border-emerald-600/50 text-emerald-200'
                      }`}
                    >
                      <span>{h.address} ({h.content})</span>
                      <span className="text-[10px] px-1.5 py-0.5 rounded font-bold bg-slate-950">
                        {h.status}
                      </span>
                    </div>
                  ))
                )}
              </div>
            </div>
          </div>

          {/* Activity Log */}
          <div className="p-4 bg-slate-950 border border-slate-800 rounded-xl">
            <h4 className="text-xs font-mono font-bold text-slate-400 uppercase mb-2">Registro de Eventos:</h4>
            <div className="max-h-36 overflow-y-auto space-y-1 text-xs font-mono">
              {sandboxLogs.map((log, i) => (
                <div key={i} className="text-slate-300 border-l-2 border-slate-700 pl-2 py-0.5">
                  {log}
                </div>
              ))}
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
