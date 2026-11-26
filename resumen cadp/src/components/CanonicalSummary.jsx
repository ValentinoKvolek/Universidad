import React, { useState } from 'react';
import { THEORY_MODULES } from '../data/theoryData';
import { BookOpen, Search, Sparkles, ChevronDown, ChevronUp, BookmarkCheck, FileText } from 'lucide-react';

export default function CanonicalSummary() {
  const [searchQuery, setSearchQuery] = useState('');
  const [expandedModules, setExpandedModules] = useState({ 'm2-modularizacion': true, 'm5-punteros-y-memoria': true });

  const toggleModule = (id) => {
    setExpandedModules(prev => ({
      ...prev,
      [id]: !prev[id]
    }));
  };

  const expandAll = () => {
    const all = {};
    THEORY_MODULES.forEach(m => all[m.id] = true);
    setExpandedModules(all);
  };

  const collapseAll = () => {
    setExpandedModules({});
  };

  const filteredModules = THEORY_MODULES.filter(m => {
    const q = searchQuery.toLowerCase();
    const matchTitle = m.title.toLowerCase().includes(q);
    const matchSummary = m.summary.toLowerCase().includes(q);
    const matchPoints = m.keyPoints.some(
      kp => kp.concept.toLowerCase().includes(q) || kp.definition.toLowerCase().includes(q)
    );
    return matchTitle || matchSummary || matchPoints;
  });

  return (
    <div className="space-y-6">
      {/* Banner */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-2xl p-6 shadow-xl">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div>
            <h2 className="text-2xl font-bold text-white tracking-tight">
              Súper Resumen Teórico Canónico (Los 8 Módulos de CADP)
            </h2>
            <p className="text-slate-300 text-sm mt-1 max-w-3xl">
              Toda la teoría de la materia sintetizada con el vocabulario técnico exacto exigido en las mesas de final de la Facultad de Informática de la UNLP.
            </p>
          </div>

          <div className="flex gap-2">
            <button
              onClick={expandAll}
              className="px-3 py-2 text-xs bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-xl transition border border-slate-700 font-medium"
            >
              Expandir Todos
            </button>
            <button
              onClick={collapseAll}
              className="px-3 py-2 text-xs bg-slate-800 hover:bg-slate-700 text-slate-300 rounded-xl transition border border-slate-700 font-medium"
            >
              Colapsar Todos
            </button>
          </div>
        </div>

        {/* Search Bar */}
        <div className="mt-6 pt-5 border-t border-slate-800">
          <div className="relative">
            <Search className="w-4 h-4 text-slate-400 absolute left-4 top-1/2 -translate-y-1/2" />
            <input
              type="text"
              placeholder="Buscar concepto o tema (ej. aliasing, dispose, subrango, udt, memoria estática)..."
              value={searchQuery}
              onChange={e => setSearchQuery(e.target.value)}
              className="w-full pl-11 pr-4 py-3 bg-slate-950/90 border border-slate-700 rounded-xl text-sm text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500 transition shadow-inner"
            />
          </div>
        </div>
      </div>

      {/* Modules List */}
      <div className="space-y-4">
        {filteredModules.length === 0 ? (
          <div className="p-8 text-center bg-slate-900 border border-slate-800 rounded-2xl text-slate-400">
            No se encontraron conceptos para "{searchQuery}". Probá con términos como "punteros", "parámetros" o "dimL".
          </div>
        ) : (
          filteredModules.map(module => {
            const isExpanded = !!expandedModules[module.id] || searchQuery.length > 0;

            return (
              <div
                key={module.id}
                className="bg-slate-900 border border-slate-800 rounded-2xl overflow-hidden shadow-md transition"
              >
                {/* Module Header Button */}
                <button
                  onClick={() => toggleModule(module.id)}
                  className="w-full p-5 text-left flex items-center justify-between hover:bg-slate-800/50 transition"
                >
                  <div className="flex items-center gap-3">
                    <div className="p-2 bg-indigo-500/10 border border-indigo-500/20 text-indigo-400 rounded-xl">
                      <FileText className="w-5 h-5" />
                    </div>
                    <div>
                      <div className="flex items-center gap-2">
                        <h3 className="font-bold text-white text-base">
                          {module.title}
                        </h3>
                        <span className="text-[10px] px-2 py-0.5 rounded-full bg-slate-800 border border-slate-700 text-indigo-300 font-mono">
                          {module.badge}
                        </span>
                      </div>
                      <p className="text-xs text-slate-400 mt-0.5">{module.summary}</p>
                    </div>
                  </div>

                  <div className="text-slate-400 p-2">
                    {isExpanded ? <ChevronUp className="w-5 h-5" /> : <ChevronDown className="w-5 h-5" />}
                  </div>
                </button>

                {/* Expanded Content */}
                {isExpanded && (
                  <div className="px-5 pb-5 pt-2 border-t border-slate-800/80 bg-slate-950/40 space-y-4">
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                      {module.keyPoints.map((kp, idx) => (
                        <div
                          key={idx}
                          className="p-4 bg-slate-900/90 border border-slate-800 rounded-xl space-y-2 hover:border-slate-700 transition"
                        >
                          <h4 className="text-xs font-mono font-bold text-indigo-300 uppercase tracking-wider flex items-center gap-1.5">
                            <Sparkles className="w-3.5 h-3.5 text-amber-400 shrink-0" />
                            {kp.concept}
                          </h4>
                          <p className="text-xs text-slate-300 leading-relaxed">
                            {kp.definition}
                          </p>
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            );
          })
        )}
      </div>
    </div>
  );
}
