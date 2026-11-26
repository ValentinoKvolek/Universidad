import React, { useState } from 'react';
import { TRAZA_EXERCISES } from '../data/trazaData';
import { Terminal, Bug, Play, RotateCcw, ArrowRight, Link, Eye, HelpCircle } from 'lucide-react';

export default function TraceDebugger() {
  const [selectedTraceId, setSelectedTraceId] = useState(TRAZA_EXERCISES[0].id);
  const [stepIndex, setStepIndex] = useState(0);

  const trace = TRAZA_EXERCISES.find(t => t.id === selectedTraceId) || TRAZA_EXERCISES[0];
  const step = trace.steps[stepIndex] || trace.steps[0];
  const totalSteps = trace.steps.length;

  return (
    <div className="space-y-6">
      {/* Banner */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-2xl p-6 shadow-xl">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div>
            <h2 className="text-2xl font-bold text-white tracking-tight">
              Trazador de Ejecución: Desarmando el Aliasing y Shadowing
            </h2>
            <p className="text-slate-300 text-sm mt-1 max-w-3xl">
              En los parciales las variables siempre tienen nombres claros y no colisionan. En el <strong className="text-emerald-400">Final</strong>, te ponen variables globales que se llaman igual que parámetros formales, o que se pasan por referencia (<code className="text-amber-300 font-mono">var</code>) colisionando con el mismo identificador.
            </p>
          </div>
        </div>

        {/* Trace Selector Pills */}
        <div className="flex flex-wrap gap-2 mt-6 pt-5 border-t border-slate-800">
          {TRAZA_EXERCISES.map(t => (
            <button
              key={t.id}
              onClick={() => {
                setSelectedTraceId(t.id);
                setStepIndex(0);
              }}
              className={`px-4 py-2 rounded-xl text-xs font-medium transition ${
                selectedTraceId === t.id
                  ? 'bg-emerald-600 text-white border border-emerald-400/40 font-semibold shadow-md'
                  : 'bg-slate-800/80 text-slate-300 hover:bg-slate-700 border border-slate-700/60'
              }`}
            >
              <span>{t.title}</span>
            </button>
          ))}
        </div>
      </div>

      {/* Main Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
        {/* Left: Code with Line Highlight (6 cols) */}
        <div className="lg:col-span-6 bg-slate-900 border border-slate-800 rounded-2xl p-5 shadow-lg flex flex-col">
          <div className="flex items-center justify-between pb-3 border-b border-slate-800 mb-3">
            <span className="text-xs font-mono font-bold text-slate-400 uppercase tracking-wider flex items-center gap-2">
              <Terminal className="w-4 h-4 text-emerald-400" /> Código de Traza
            </span>
            <span className="text-[11px] font-mono text-emerald-400">
              Línea activa: {step.line}
            </span>
          </div>

          <div className="bg-slate-950 border border-slate-800 rounded-xl p-4 font-mono text-xs overflow-x-auto flex-1 leading-relaxed">
            {trace.code.split('\n').map((line, idx) => {
              const lineNum = idx + 1;
              const isCurrent = step.line === lineNum;

              return (
                <div
                  key={idx}
                  className={`flex items-start gap-3 py-0.5 px-2 -mx-2 rounded transition-colors ${
                    isCurrent
                      ? 'bg-emerald-500/25 text-emerald-200 border-l-2 border-emerald-400 font-bold'
                      : 'text-slate-300 hover:bg-slate-900/60'
                  }`}
                >
                  <span className="text-slate-600 select-none w-6 text-right shrink-0">
                    {lineNum}
                  </span>
                  <span className="whitespace-pre">{line}</span>
                </div>
              );
            })}
          </div>

          {/* Stepper Controls */}
          <div className="flex items-center justify-between pt-4 mt-4 border-t border-slate-800">
            <button
              onClick={() => setStepIndex(0)}
              className="p-2 text-slate-400 hover:text-white hover:bg-slate-800 rounded-lg transition"
              title="Reiniciar"
            >
              <RotateCcw className="w-4 h-4" />
            </button>

            <div className="flex items-center gap-2">
              <button
                disabled={stepIndex === 0}
                onClick={() => setStepIndex(prev => Math.max(0, prev - 1))}
                className="px-3 py-1.5 text-xs font-medium bg-slate-800 hover:bg-slate-700 disabled:opacity-40 text-slate-200 rounded-lg transition"
              >
                Paso Anterior
              </button>
              <button
                disabled={stepIndex === totalSteps - 1}
                onClick={() => setStepIndex(prev => Math.min(totalSteps - 1, prev + 1))}
                className="px-4 py-1.5 text-xs font-semibold bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg transition flex items-center gap-1.5 shadow-md shadow-emerald-500/20"
              >
                Siguiente Paso <ArrowRight className="w-3.5 h-3.5" />
              </button>
            </div>

            <span className="text-xs font-mono text-slate-400">
              {stepIndex + 1} / {totalSteps}
            </span>
          </div>
        </div>

        {/* Right: Scope Matrix & Terminal (6 cols) */}
        <div className="lg:col-span-6 space-y-4">
          {/* Active Step Details */}
          <div className="bg-slate-900 border border-slate-800 rounded-2xl p-5 shadow-lg space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-xs font-mono font-bold uppercase tracking-wider text-emerald-400 flex items-center gap-1.5">
                <Eye className="w-4 h-4" /> Ámbito Activo:
              </span>
              <span className="px-2.5 py-0.5 rounded bg-emerald-500/20 text-emerald-300 border border-emerald-500/30 font-mono text-xs font-bold">
                {step.scope}
              </span>
            </div>

            <div className="p-4 bg-slate-950 border border-slate-800 rounded-xl">
              <p className="text-slate-200 text-sm whitespace-pre-line leading-relaxed">
                {step.description}
              </p>
            </div>

            {/* Variable Inspection Matrix */}
            <div>
              <h4 className="text-xs font-mono font-bold text-slate-400 uppercase tracking-wider mb-2 flex items-center gap-1.5">
                <Link className="w-3.5 h-3.5 text-blue-400" /> Estado de Variables y Celdas de Memoria:
              </h4>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-2">
                {Object.entries(step.vars).map(([name, val]) => {
                  const isAliased = name.includes('ref') || name.includes('global');
                  return (
                    <div
                      key={name}
                      className={`p-3 rounded-xl border font-mono text-xs flex justify-between items-center ${
                        isAliased
                          ? 'bg-slate-950 border-amber-500/40 text-amber-200'
                          : 'bg-slate-950 border-slate-800 text-slate-200'
                      }`}
                    >
                      <span className="text-slate-300 font-semibold">{name}</span>
                      <span className="px-2 py-0.5 rounded bg-slate-900 font-bold border border-slate-700 text-white">
                        {String(val)}
                      </span>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* Simulated Terminal Output */}
            <div>
              <h4 className="text-xs font-mono font-bold text-slate-400 uppercase tracking-wider mb-2 flex items-center gap-1.5">
                <Terminal className="w-3.5 h-3.5 text-emerald-400" /> Salida de writeln (Consola):
              </h4>
              <div className="p-3 bg-black border border-slate-800 rounded-xl font-mono text-xs text-emerald-400 min-h-[50px] whitespace-pre-wrap">
                {step.output || <span className="text-slate-600 italic">Esperando instrucción writeln...</span>}
              </div>
            </div>
          </div>

          {/* Docente Secret Rule Card */}
          <div className="bg-slate-900 border border-slate-800 rounded-2xl p-5 shadow-lg space-y-2">
            <h4 className="text-xs font-bold text-amber-300 flex items-center gap-1.5 uppercase font-mono">
              <HelpCircle className="w-4 h-4 text-amber-400" /> Regla Práctica para Resolver Trazas en el Examen:
            </h4>
            <p className="text-xs text-slate-300 leading-relaxed">
              En tu hoja de borrador, <strong className="text-white">dibujá cajitas con flechas</strong>. Si una variable global entra como parámetro por referencia (<code className="text-amber-300">var</code>), no crees una cajita nueva: hacé que el nombre del parámetro apunte a la <em>misma cajita global</em>. Cada vez que el procedimiento escriba en ese parámetro, tachá el valor de la global inmediatamente.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
