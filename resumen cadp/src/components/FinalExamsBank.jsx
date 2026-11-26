import React, { useState } from 'react';
import { FINALS_BANK } from '../data/finalsData';
import { GraduationCap, Calendar, CheckCircle2, AlertTriangle, Code2, Copy, Check, FileCheck } from 'lucide-react';

export default function FinalExamsBank() {
  const [selectedExamId, setSelectedExamId] = useState(FINALS_BANK[0].id);
  const [copied, setCopied] = useState(false);

  const exam = FINALS_BANK.find(e => e.id === selectedExamId) || FINALS_BANK[0];

  const handleCopy = () => {
    navigator.clipboard.writeText(exam.practiceSolution);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="space-y-6">
      {/* Banner */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-2xl p-6 shadow-xl">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div>
            <h2 className="text-2xl font-bold text-white tracking-tight">
              Banco de Finales Resueltos con Criterio de Corrección
            </h2>
            <p className="text-slate-300 text-sm mt-1 max-w-3xl">
              Modelos de examen completos con su resolución práctica modularizada, análisis teórico y las advertencias de cátedra para no perder puntos.
            </p>
          </div>
        </div>

        {/* Exam Selector Pills */}
        <div className="flex flex-wrap gap-2 mt-6 pt-5 border-t border-slate-800">
          {FINALS_BANK.map(f => (
            <button
              key={f.id}
              onClick={() => setSelectedExamId(f.id)}
              className={`px-4 py-2 rounded-xl text-xs font-medium transition flex items-center gap-2 ${
                selectedExamId === f.id
                  ? 'bg-emerald-600 text-white font-semibold border border-emerald-400/40 shadow-lg'
                  : 'bg-slate-800/80 text-slate-300 hover:bg-slate-700 border border-slate-700/60'
              }`}
            >
              <Calendar className="w-3.5 h-3.5" />
              <span>{f.date}</span>
              <span className="text-[10px] px-1.5 py-0.2 rounded bg-slate-900/60 text-emerald-300">
                {f.theme}
              </span>
            </button>
          ))}
        </div>
      </div>

      {/* Selected Exam Display */}
      <div className="space-y-6">
        {/* Practice Problem Card */}
        <div className="bg-slate-900 border border-slate-800 rounded-2xl p-6 shadow-xl space-y-4">
          <div className="flex items-center justify-between pb-4 border-b border-slate-800">
            <div>
              <span className="text-xs font-mono font-bold uppercase text-amber-400 block mb-1">
                Ejercicio 1 (Condición Excluyente)
              </span>
              <h3 className="text-lg font-bold text-white">
                Enunciado de la Parte Práctica
              </h3>
            </div>
            <button
              onClick={handleCopy}
              className="px-3 py-1.5 bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 rounded-xl text-xs font-medium transition flex items-center gap-1.5"
            >
              {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
              {copied ? 'Copiado' : 'Copiar Pascal'}
            </button>
          </div>

          <div className="p-4 bg-slate-950 border border-slate-800 rounded-xl text-slate-200 text-sm whitespace-pre-line leading-relaxed">
            {exam.practiceStatement}
          </div>

          {/* Practice Solution Code */}
          <div>
            <h4 className="text-xs font-mono font-bold text-emerald-400 uppercase tracking-wider mb-2 flex items-center gap-1.5">
              <Code2 className="w-4 h-4" /> Resolución Modelo Canónica en Pascal:
            </h4>
            <pre className="p-4 bg-slate-950 border border-slate-800/80 rounded-xl font-mono text-xs text-slate-200 overflow-x-auto leading-relaxed max-h-[380px]">
              {exam.practiceSolution}
            </pre>
          </div>

          {/* Docente Correction Criteria */}
          {exam.docenteCriteria && (
            <div className="mt-4 pt-4 border-t border-slate-800/80 bg-amber-950/20 -mx-6 -mb-6 p-6 rounded-b-2xl border-amber-900/30">
              <span className="text-xs font-mono font-bold text-amber-300 uppercase flex items-center gap-1.5 mb-2">
                <FileCheck className="w-4 h-4 text-amber-400" /> Criterios de Evaluación del Docente:
              </span>
              <ul className="text-xs text-slate-300 space-y-1.5 list-disc pl-4">
                {exam.docenteCriteria.map((c, i) => (
                  <li key={i}>{c}</li>
                ))}
              </ul>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
