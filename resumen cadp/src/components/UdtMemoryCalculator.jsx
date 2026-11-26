import React, { useState } from 'react';
import { UDT_RULES, MEMORY_CALCULATOR_DEFAULT_TYPES, REAL_EXAM_MEMORY_PROBLEMS } from '../data/udtData';
import { Calculator, Clock, HardDrive, HelpCircle, Check, Sparkles, AlertCircle } from 'lucide-react';

export default function UdtMemoryCalculator() {
  const [activeTab, setActiveTab] = useState('udt'); // 'udt' | 'memory'

  // UDT Interactive Sandbox state
  const [loopType, setLoopType] = useState('for'); // 'for' | 'while' | 'repeat'
  const [nVal, setNVal] = useState(10);
  const [condCost, setCondCost] = useState(1);
  const [bodyCost, setBodyCost] = useState(5);

  // Custom Memory Sandbox state
  const [customTypes, setCustomTypes] = useState({
    char: 1,
    integer: 6,
    real: 10,
    boolean: 1,
    puntero: 4,
    string: 256
  });

  // Calculate UDT for the interactive tester
  const calculateUDT = () => {
    const N = Number(nVal) || 0;
    const C = Number(condCost) || 1;
    const B = Number(bodyCost) || 0;

    if (loopType === 'for') {
      const overhead = 3 * N + 2;
      const bodyTotal = N * B;
      return {
        formulaText: `(3 \\times ${N} + 2) + ${N} \\times (${B})`,
        overhead,
        bodyTotal,
        total: overhead + bodyTotal,
        explanation: `Overhead del FOR = 3(${N}) + 2 = ${overhead} UDT. Cuerpo = ${N} * ${B} = ${bodyTotal} UDT.`
      };
    } else if (loopType === 'while') {
      const condTotal = C * (N + 1);
      const bodyTotal = N * B;
      return {
        formulaText: `${C} \\times (${N} + 1) + ${N} \\times (${B})`,
        overhead: condTotal,
        bodyTotal,
        total: condTotal + bodyTotal,
        explanation: `Condición evaluada (${N} + 1) veces = ${condTotal} UDT. Cuerpo = ${N} * ${B} = ${bodyTotal} UDT.`
      };
    } else {
      // Repeat
      const condTotal = C * N;
      const bodyTotal = N * B;
      return {
        formulaText: `${N} \\times (${B}) + ${C} \\times ${N}`,
        overhead: condTotal,
        bodyTotal,
        total: condTotal + bodyTotal,
        explanation: `Cuerpo ejecutado ${N} veces = ${bodyTotal} UDT. Condición evaluada ${N} veces = ${condTotal} UDT.`
      };
    }
  };

  const udtResult = calculateUDT();
  const examProblem = REAL_EXAM_MEMORY_PROBLEMS[0];

  return (
    <div className="space-y-6">
      {/* Banner */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-2xl p-6 shadow-xl">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div>
            <h2 className="text-2xl font-bold text-white tracking-tight">
              Calculadora & Desglosador de UDT y Memoria (Bytes)
            </h2>
            <p className="text-slate-300 text-sm mt-1 max-w-3xl">
              En el final te piden obligatoriamente mostrar el <strong className="text-amber-300">desglose paso a paso</strong> de la memoria estática, la memoria dinámica y el tiempo de ejecución en UDT usando las fórmulas oficiales de la cátedra UNLP.
            </p>
          </div>

          <div className="flex gap-2">
            <button
              onClick={() => setActiveTab('udt')}
              className={`px-4 py-2.5 rounded-xl text-xs font-semibold transition flex items-center gap-1.5 ${
                activeTab === 'udt'
                  ? 'bg-amber-600 text-white shadow-lg shadow-amber-500/20'
                  : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
              }`}
            >
              <Clock className="w-3.5 h-3.5" /> Fórmulas UDT (Tiempo)
            </button>
            <button
              onClick={() => setActiveTab('memory')}
              className={`px-4 py-2.5 rounded-xl text-xs font-semibold transition flex items-center gap-1.5 ${
                activeTab === 'memory'
                  ? 'bg-amber-600 text-white shadow-lg shadow-amber-500/20'
                  : 'bg-slate-800 text-slate-300 hover:bg-slate-700'
              }`}
            >
              <HardDrive className="w-3.5 h-3.5" /> Memoria Estática y Dinámica
            </button>
          </div>
        </div>
      </div>

      {activeTab === 'udt' ? (
        /* UDT Section */
        <div className="space-y-6">
          {/* Official Formula Reference Cards */}
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {UDT_RULES.map((rule, idx) => (
              <div key={idx} className="bg-slate-900 border border-slate-800 rounded-2xl p-5 shadow-md flex flex-col justify-between">
                <div>
                  <h4 className="text-xs font-mono font-bold text-amber-400 uppercase tracking-wider mb-2">
                    {rule.name}
                  </h4>
                  <div className="p-3 bg-slate-950 border border-slate-800 rounded-xl font-mono text-sm text-white mb-3 text-center">
                    {rule.formula}
                  </div>
                  <p className="text-xs text-slate-300 leading-relaxed">
                    {rule.explanation}
                  </p>
                </div>
                {rule.example && (
                  <div className="mt-3 pt-3 border-t border-slate-800/80 font-mono text-[11px] text-slate-400">
                    Ej: <code className="text-amber-200">{rule.example}</code>
                  </div>
                )}
              </div>
            ))}
          </div>

          {/* Interactive Loop Calculator Widget */}
          <div className="bg-slate-900 border border-slate-800 rounded-2xl p-6 shadow-xl space-y-6">
            <div className="flex items-center justify-between pb-4 border-b border-slate-800">
              <div className="flex items-center gap-2">
                <Sparkles className="w-5 h-5 text-amber-400" />
                <h3 className="font-bold text-lg text-white">
                  Simulador de Cálculo de UDT para Bucles
                </h3>
              </div>
              <span className="text-xs font-mono text-slate-400">
                Aplica la fórmula matemática oficial
              </span>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 items-center">
              {/* Form inputs (6 cols) */}
              <div className="lg:col-span-6 space-y-4">
                <div>
                  <label className="text-xs font-mono text-slate-400 block mb-1">
                    Tipo de Estructura de Control:
                  </label>
                  <div className="grid grid-cols-3 gap-2">
                    {['for', 'while', 'repeat'].map(t => (
                      <button
                        key={t}
                        onClick={() => setLoopType(t)}
                        className={`py-2 rounded-xl text-xs font-mono font-bold uppercase transition ${
                          loopType === t
                            ? 'bg-amber-600 text-white shadow-md'
                            : 'bg-slate-800 text-slate-400 hover:bg-slate-700'
                        }`}
                      >
                        {t}
                      </button>
                    ))}
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="text-xs font-mono text-slate-400 block mb-1">
                      Iteraciones (N):
                    </label>
                    <input
                      type="number"
                      min="1"
                      max="1000"
                      value={nVal}
                      onChange={e => setNVal(e.target.value)}
                      className="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-xl text-sm font-mono text-white focus:outline-none focus:border-amber-500"
                    />
                  </div>

                  {loopType !== 'for' && (
                    <div>
                      <label className="text-xs font-mono text-slate-400 block mb-1">
                        Costo Condición (C):
                      </label>
                      <input
                        type="number"
                        min="1"
                        max="20"
                        value={condCost}
                        onChange={e => setCondCost(e.target.value)}
                        className="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-xl text-sm font-mono text-white focus:outline-none focus:border-amber-500"
                      />
                    </div>
                  )}

                  <div>
                    <label className="text-xs font-mono text-slate-400 block mb-1">
                      Costo del Cuerpo (UDT):
                    </label>
                    <input
                      type="number"
                      min="0"
                      max="100"
                      value={bodyCost}
                      onChange={e => setBodyCost(e.target.value)}
                      className="w-full px-3 py-2 bg-slate-950 border border-slate-700 rounded-xl text-sm font-mono text-white focus:outline-none focus:border-amber-500"
                    />
                  </div>
                </div>
              </div>

              {/* Output Result Card (6 cols) */}
              <div className="lg:col-span-6 bg-slate-950 border border-amber-900/40 rounded-2xl p-6 text-center space-y-3">
                <span className="text-xs font-mono font-bold uppercase tracking-wider text-amber-400">
                  Total de Unidades de Tiempo (UDT)
                </span>
                <div className="text-5xl font-black text-white font-mono tracking-tight">
                  {udtResult.total} <span className="text-xl text-amber-400 font-normal">UDT</span>
                </div>
                <div className="p-3 bg-slate-900 border border-slate-800 rounded-xl font-mono text-xs text-slate-300">
                  {udtResult.explanation}
                </div>
              </div>
            </div>
          </div>
        </div>
      ) : (
        /* Memory Section */
        <div className="space-y-6">
          {/* Exam Problem Walkthrough */}
          <div className="bg-slate-900 border border-slate-800 rounded-2xl p-6 shadow-xl space-y-6">
            <div className="flex items-center justify-between pb-4 border-b border-slate-800">
              <div>
                <h3 className="font-bold text-lg text-white">
                  {examProblem.title}
                </h3>
                <p className="text-xs text-slate-400">
                  Caso real tomado en mesa presencial con tabla de tipos arbitraria dada por la cátedra.
                </p>
              </div>
              <span className="px-3 py-1 bg-amber-500/20 text-amber-300 border border-amber-500/30 rounded-lg text-xs font-mono font-bold">
                Resolución Oficial
              </span>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
              {/* Code display (5 cols) */}
              <div className="lg:col-span-5 bg-slate-950 border border-slate-800 rounded-xl p-4 font-mono text-xs text-slate-300 leading-relaxed overflow-x-auto">
                <div className="text-amber-400 font-bold mb-2 pb-2 border-b border-slate-800">
                  Tabla de Tamaños del Examen:
                </div>
                <div className="grid grid-cols-2 gap-1 mb-4 text-[11px] text-slate-400">
                  <div>Char: 1B</div>
                  <div>Integer: 6B</div>
                  <div>Real: 10B</div>
                  <div>Boolean: 1B</div>
                  <div>Puntero: 4B</div>
                  <div>String: 256B</div>
                </div>
                <pre>{examProblem.code}</pre>
              </div>

              {/* Step-by-Step Breakdown (7 cols) */}
              <div className="lg:col-span-7 space-y-4">
                {/* Static memory breakdown */}
                <div className="bg-slate-950/80 border border-purple-900/40 rounded-xl p-4 space-y-3">
                  <h4 className="text-xs font-mono font-bold text-purple-300 uppercase flex items-center justify-between">
                    <span>1. Desglose de Memoria Estática (Stack)</span>
                    <span className="text-sm text-white font-black">342 Bytes</span>
                  </h4>
                  <div className="space-y-1.5 text-xs font-mono">
                    {examProblem.estaticaExplanation.map((item, idx) => (
                      <div key={idx} className="flex justify-between items-center py-1 border-b border-slate-900">
                        <span className="text-slate-300">{item.item}</span>
                        <span className="text-slate-400 font-bold">{item.calc}</span>
                      </div>
                    ))}
                  </div>
                </div>

                {/* Dynamic memory breakdown */}
                <div className="bg-slate-950/80 border border-emerald-900/40 rounded-xl p-4 space-y-3">
                  <h4 className="text-xs font-mono font-bold text-emerald-300 uppercase flex items-center justify-between">
                    <span>2. Desglose de Memoria Dinámica (Heap)</span>
                    <span className="text-sm text-white font-black">2.926 B pedidos</span>
                  </h4>
                  <div className="space-y-2 text-xs leading-relaxed text-slate-300">
                    {examProblem.dinamicaExplanation.map((item, idx) => (
                      <div key={idx} className="p-2 bg-slate-900/80 rounded-lg border border-slate-800">
                        <strong className="text-emerald-400 font-mono block mb-0.5">{item.item}</strong>
                        <p>{item.calc || item.note}</p>
                      </div>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
