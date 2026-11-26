import React, { useState, useEffect } from 'react';
import { ALGORITHMS } from '../data/algorithmsData';
import { Play, Pause, SkipForward, RotateCcw, CheckCircle2, AlertTriangle, ArrowRight, ArrowDown, Code2, Sparkles } from 'lucide-react';

export default function AlgorithmLab() {
  const [selectedAlgoId, setSelectedAlgoId] = useState(ALGORITHMS[0].id);
  const [currentStep, setCurrentStep] = useState(0);
  const [isPlaying, setIsPlaying] = useState(false);
  const [copied, setCopied] = useState(false);

  const algo = ALGORITHMS.find(a => a.id === selectedAlgoId) || ALGORITHMS[0];
  const step = algo.steps[currentStep] || algo.steps[0];
  const totalSteps = algo.steps.length;

  useEffect(() => {
    setCurrentStep(0);
    setIsPlaying(false);
  }, [selectedAlgoId]);

  useEffect(() => {
    let timer;
    if (isPlaying) {
      timer = setInterval(() => {
        setCurrentStep(prev => {
          if (prev < totalSteps - 1) {
            return prev + 1;
          } else {
            setIsPlaying(false);
            return prev;
          }
        });
      }, 2000);
    }
    return () => clearInterval(timer);
  }, [isPlaying, totalSteps]);

  const handleCopyCode = () => {
    navigator.clipboard.writeText(algo.pascalCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="space-y-6">
      {/* Header Banner */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-2xl p-6 shadow-xl">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div>
            <h2 className="text-2xl font-bold text-white tracking-tight">
              Laboratorio Interactivo de Algoritmos Canónicos
            </h2>
            <p className="text-slate-300 text-sm mt-1 max-w-3xl">
              Si tu código del primer ejercicio tiene un error conceptual en el manejo de punteros o bucles, los docentes no corrigen la teoría. Practicá y visualizá cada paso de los algoritmos clásicos de examen.
            </p>
          </div>
          <button
            onClick={handleCopyCode}
            className="px-4 py-2.5 bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 rounded-xl text-sm font-medium transition flex items-center gap-2 self-start md:self-auto shrink-0"
          >
            <Code2 className="w-4 h-4 text-blue-400" />
            {copied ? '¡Copiado!' : 'Copiar Pascal'}
          </button>
        </div>

        {/* Algorithm Selector Buttons */}
        <div className="flex flex-wrap gap-2 mt-5 pt-4 border-t border-slate-800">
          {ALGORITHMS.map(a => (
            <button
              key={a.id}
              onClick={() => setSelectedAlgoId(a.id)}
              className={`px-3.5 py-1.5 rounded-xl text-xs font-medium transition-all ${
                selectedAlgoId === a.id
                  ? 'bg-blue-600 text-white shadow-sm font-semibold'
                  : 'bg-slate-800/80 text-slate-300 hover:bg-slate-700'
              }`}
            >
              {a.title}
            </button>
          ))}
        </div>
      </div>

      {/* Main Interactive Stage: Side-by-side on large screens (8 cols vs 4 cols) */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start">
        {/* Left Column: Expanded Algorithm Visualizer (8 cols) */}
        <div className="lg:col-span-8 space-y-4">
          <div className="bg-slate-900 border border-slate-800 rounded-2xl p-5 sm:p-6 shadow-xl space-y-4">
            <div className="flex items-center justify-between pb-4 border-b border-slate-800">
              <div>
                <h3 className="font-bold text-base text-white flex items-center gap-2">
                  <Sparkles className="w-4 h-4 text-amber-400" />
                  {algo.title}
                </h3>
                <p className="text-xs text-slate-400 mt-0.5">{algo.description}</p>
              </div>

              {/* Step counter */}
              <div className="px-2.5 py-1 bg-slate-800 border border-slate-700 rounded-lg text-xs font-mono font-medium text-blue-400">
                Paso {currentStep + 1} de {totalSteps}
              </div>
            </div>

            {/* Animation Stage / Arena */}
            <div className="min-h-[380px] py-8 px-4 sm:px-6 flex flex-col justify-center bg-slate-950/80 border border-slate-800/90 rounded-2xl my-4 relative overflow-x-auto shadow-inner">
              {/* Specialized Interactive View for Insertar Ordenado */}
              {algo.id === 'insertar-ordenado' ? (
                <div className="w-fit min-w-full m-auto flex flex-col items-center py-4 px-2">
                  {/* Top Row: The List */}
                  <div className="flex items-center gap-3">
                    {/* Head pointer 'L' */}
                    <div className="flex flex-col items-center">
                      <span className="text-xs font-mono font-bold text-emerald-400 mb-1">L</span>
                      <div className="w-7 h-7 rounded-full bg-emerald-500/20 border-2 border-emerald-500 flex items-center justify-center text-xs font-mono text-emerald-300">
                        ●
                      </div>
                      <span className="text-[10px] text-slate-500 font-mono mt-1">puntero</span>
                    </div>

                    <ArrowRight className="w-5 h-5 text-slate-500 shrink-0" />

                    {/* Node 1: 10 */}
                    <div className="relative">
                      <div className="absolute -top-7 left-0 right-0 flex justify-center gap-1">
                        {step.pointers?.ant === 1 && (
                          <span className="px-1.5 py-0.2 bg-purple-500/30 text-purple-300 border border-purple-400 text-[11px] font-mono font-bold rounded">
                            ant
                          </span>
                        )}
                        {step.pointers?.act === 1 && (
                          <span className="px-1.5 py-0.2 bg-blue-500/30 text-blue-300 border border-blue-400 text-[11px] font-mono font-bold rounded">
                            act
                          </span>
                        )}
                      </div>
                      <div className="w-24 h-16 rounded-xl border-2 border-slate-700 bg-slate-900 flex overflow-hidden shadow-md">
                        <div className="w-2/3 flex items-center justify-center font-mono font-bold text-sm text-slate-200 border-r border-slate-700/60 bg-slate-800/40">
                          10
                        </div>
                        <div className="w-1/3 flex items-center justify-center font-mono text-[10px] text-slate-400 bg-slate-950/40">
                          ^sig
                        </div>
                      </div>
                    </div>

                    {/* Link 10 -> 25 */}
                    <div className="flex items-center shrink-0">
                      <div className="w-5 h-0.5 bg-blue-500/60"></div>
                      <ArrowRight className="w-4 h-4 text-blue-400 -ml-1" />
                    </div>

                    {/* Node 2: 25 */}
                    <div className="relative">
                      <div className="absolute -top-7 left-0 right-0 flex justify-center gap-1">
                        {step.pointers?.ant === 2 && (
                          <span className="px-1.5 py-0.2 bg-purple-500/30 text-purple-300 border border-purple-400 text-[11px] font-mono font-bold rounded">
                            ant
                          </span>
                        )}
                        {step.pointers?.act === 2 && (
                          <span className="px-1.5 py-0.2 bg-blue-500/30 text-blue-300 border border-blue-400 text-[11px] font-mono font-bold rounded">
                            act
                          </span>
                        )}
                      </div>
                      <div className={`w-24 h-16 rounded-xl border-2 flex overflow-hidden shadow-md transition-all ${
                        step.connectAntNue ? 'border-amber-400 bg-amber-950/30 text-white' : 'border-slate-700 bg-slate-900 text-slate-200'
                      }`}>
                        <div className="w-2/3 flex items-center justify-center font-mono font-bold text-sm border-r border-slate-700/60 bg-slate-800/40">
                          25
                        </div>
                        <div className={`w-1/3 flex items-center justify-center font-mono text-[10px] ${
                          step.connectAntNue ? 'bg-amber-500/20 text-amber-300 font-bold' : 'text-slate-400 bg-slate-950/40'
                        }`}>
                          ^sig
                        </div>
                      </div>
                    </div>

                    {/* Connection logic between Node 2 (25) and Node 3 (40) */}
                    {step.connectNueAct ? (
                      /* STEP 12: Union is complete! Node nue (28) is placed in the list with standard default arrows */
                      <>
                        {/* Default Arrow from ant (25) to nue (28) */}
                        <div className="flex flex-col items-center shrink-0 px-1">
                          <span className="text-[9px] font-mono text-amber-400 mb-0.5 font-bold">ant^.sig</span>
                          <div className="flex items-center">
                            <div className="w-6 h-0.5 bg-blue-500/60"></div>
                            <ArrowRight className="w-4 h-4 text-blue-400 -ml-1" />
                          </div>
                        </div>

                        {/* Node nue (28) integrated in line */}
                        <div className="relative">
                          <div className="absolute -top-7 left-0 right-0 flex justify-center gap-1">
                            <span className="px-1.5 py-0.2 bg-emerald-500/30 text-emerald-300 border border-emerald-400 text-[11px] font-mono font-bold rounded shadow">
                              nue
                            </span>
                          </div>
                          <div className="w-24 h-16 rounded-xl border-2 border-emerald-400 bg-emerald-950/40 flex overflow-hidden shadow-lg ring-1 ring-emerald-500/40 transition-all">
                            <div className="w-2/3 flex items-center justify-center font-mono font-bold text-sm border-r border-slate-700/60 bg-slate-800/40 text-emerald-100">
                              {step.newNode.val}
                            </div>
                            <div className="w-1/3 flex items-center justify-center font-mono text-[10px] bg-emerald-500/20 text-emerald-300 font-bold">
                              ^sig
                            </div>
                          </div>
                        </div>

                        {/* Default Arrow from nue (28) to act (40) */}
                        <div className="flex flex-col items-center shrink-0 px-1">
                          <span className="text-[9px] font-mono text-emerald-400 mb-0.5 font-bold">nue^.sig</span>
                          <div className="flex items-center">
                            <div className="w-6 h-0.5 bg-blue-500/60"></div>
                            <ArrowRight className="w-4 h-4 text-blue-400 -ml-1" />
                          </div>
                        </div>
                      </>
                    ) : (
                      /* Steps 0 to 11: Link between 25 and 40 */
                      <div className="flex items-center shrink-0 relative px-2 w-16 justify-center">
                        {!step.connectAntNue ? (
                          /* Normal default arrow */
                          <div className="flex items-center">
                            <div className="w-8 h-0.5 bg-blue-500/60"></div>
                            <ArrowRight className="w-4 h-4 text-blue-400 -ml-1" />
                          </div>
                        ) : (
                          /* Severed link when ant^.sig redirects to nue */
                          <div className="flex items-center text-[10px] font-mono text-rose-400 bg-rose-950/30 border border-rose-500/30 px-1.5 py-0.5 rounded">
                            cortado
                          </div>
                        )}
                      </div>
                    )}

                    {/* Node 3: 40 */}
                    <div className="relative">
                      <div className="absolute -top-7 left-0 right-0 flex justify-center gap-1">
                        {step.pointers?.act === 3 && (
                          <span className="px-1.5 py-0.2 bg-blue-500/30 text-blue-300 border border-blue-400 text-[11px] font-mono font-bold rounded">
                            act
                          </span>
                        )}
                      </div>
                      <div className={`w-24 h-16 rounded-xl border-2 flex overflow-hidden shadow-md transition-all ${
                        step.pointers?.act === 3 ? 'border-blue-400 bg-blue-950/30 text-white' : 'border-slate-700 bg-slate-900 text-slate-200'
                      }`}>
                        <div className="w-2/3 flex items-center justify-center font-mono font-bold text-sm border-r border-slate-700/60 bg-slate-800/40">
                          40
                        </div>
                        <div className="w-1/3 flex items-center justify-center font-mono text-[10px] text-slate-400 bg-slate-950/40">
                          ^sig
                        </div>
                      </div>
                    </div>

                    {/* Nil */}
                    <div className="flex items-center shrink-0 ml-1">
                      <div className="w-3 h-0.5 bg-slate-600"></div>
                      <span className="px-2 py-1 bg-slate-800 border border-slate-700 text-slate-400 rounded text-xs font-mono font-bold">
                        nil
                      </span>
                    </div>
                  </div>

                  {/* Bottom section: Static Node 'nue' in Heap (Steps 0 to 11) */}
                  {step.newNode && step.newNode.visible && !step.connectNueAct && (
                    <div className="mt-6 flex flex-col items-center">
                      {step.connectAntNue ? (
                        <div className="mb-2 flex items-center gap-1.5 px-3 py-1 rounded bg-amber-500/10 border border-amber-500/30 text-amber-300 text-xs font-mono">
                          <span>ant^.sig redirigido a nue</span>
                          <ArrowDown className="w-3.5 h-3.5 text-amber-400 animate-bounce" />
                        </div>
                      ) : (
                        <span className="text-[11px] font-mono font-bold text-amber-400 mb-1">
                          nue (nodo en Heap)
                        </span>
                      )}
                      <div className={`w-24 h-16 rounded-xl border-2 flex overflow-hidden shadow-lg transition-all ${
                        step.connectAntNue
                          ? 'border-amber-400 bg-amber-950/40 text-amber-200 ring-2 ring-amber-400/30'
                          : 'border-amber-400 bg-amber-950/40 text-amber-200'
                      }`}>
                        <div className="w-2/3 flex items-center justify-center font-mono font-bold text-sm border-r border-slate-700/60 bg-slate-800/40 text-white">
                          {step.newNode.val}
                        </div>
                        <div className="w-1/3 flex items-center justify-center font-mono text-[10px] text-slate-400 bg-slate-950/40">
                          ^sig
                        </div>
                      </div>
                    </div>
                  )}

                  {/* Step 12 Success Message */}
                  {step.connectNueAct && (
                    <div className="mt-8 max-w-xl w-full mx-auto px-5 py-3.5 rounded-xl bg-emerald-950/90 border-2 border-emerald-500/80 shadow-xl shadow-emerald-950/50 flex items-center gap-3.5 transition-all">
                      <div className="w-9 h-9 rounded-xl bg-emerald-500/20 border border-emerald-400/50 flex items-center justify-center shrink-0 text-emerald-300">
                        <CheckCircle2 className="w-5 h-5 text-emerald-400" />
                      </div>
                      <div className="text-sm font-sans leading-snug">
                        <div className="font-bold text-emerald-300 text-xs tracking-wider uppercase mb-1 flex items-center gap-1.5">
                          <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse"></span>
                          Unión Completa en Memoria
                        </div>
                        <p className="text-emerald-100 text-xs sm:text-sm">
                          <strong className="text-amber-300 font-mono">ant (25)</strong> se une a <strong className="text-emerald-300 font-mono">nue (28)</strong> y <strong className="text-emerald-300 font-mono">nue (28)</strong> al siguiente <strong className="text-blue-300 font-mono">act (40)</strong> con las flechas normales de la lista.
                        </p>
                      </div>
                    </div>
                  )}
                </div>
              ) : (
                /* Other Linked List Algorithms (Generic) */
                step.nodes && (
                  <div className="w-fit min-w-full m-auto flex items-center justify-center gap-3 py-6 px-2">
                    {/* Head pointer 'L' */}
                    <div className="flex flex-col items-center">
                      <span className="text-xs font-mono font-bold text-emerald-400 mb-1">L</span>
                      <div className="w-7 h-7 rounded-full bg-emerald-500/20 border-2 border-emerald-500 flex items-center justify-center text-xs font-mono text-emerald-300">
                        ●
                      </div>
                      <span className="text-[10px] text-slate-500 font-mono mt-1">puntero</span>
                    </div>

                    <ArrowRight className="w-5 h-5 text-slate-500 shrink-0" />

                    {/* Render Nodes */}
                    {step.nodes.map((node, idx) => {
                      const isAnt = step.pointers?.ant === node.id;
                      const isAct = step.pointers?.act === node.id;
                      const isAux = step.pointers?.aux === node.id;

                      return (
                        <React.Fragment key={node.id || idx}>
                          <div className="relative group">
                            <div className="absolute -top-7 left-0 right-0 flex justify-center gap-1">
                              {isAnt && (
                                <span className="px-1.5 py-0.2 bg-purple-500/30 text-purple-300 border border-purple-400 text-[11px] font-mono font-bold rounded">
                                  ant
                                </span>
                              )}
                              {isAct && (
                                <span className="px-1.5 py-0.2 bg-blue-500/30 text-blue-300 border border-blue-400 text-[11px] font-mono font-bold rounded">
                                  act
                                </span>
                              )}
                              {isAux && (
                                <span className="px-1.5 py-0.2 bg-rose-500/30 text-rose-300 border border-rose-400 text-[11px] font-mono font-bold rounded">
                                  aux
                                </span>
                              )}
                            </div>

                            <div
                              className={`w-24 h-16 rounded-xl border-2 flex overflow-hidden shadow-md transition-all duration-300 ${
                                node.isDupe || node.isDetached
                                  ? 'border-rose-500 bg-rose-950/40 text-rose-200 line-through opacity-75 scale-95'
                                  : isAct
                                  ? 'border-blue-400 bg-blue-950/40 text-white'
                                  : 'border-slate-700 bg-slate-900 text-slate-200'
                              }`}
                            >
                              <div className="w-2/3 flex items-center justify-center font-mono font-bold text-sm border-r border-slate-700/60 bg-slate-800/40">
                                {node.val}
                              </div>
                              <div className="w-1/3 flex items-center justify-center font-mono text-[10px] text-slate-400 bg-slate-950/40">
                                ^sig
                              </div>
                            </div>
                          </div>

                          {idx < step.nodes.length - 1 && (
                            <div className="flex items-center shrink-0">
                              <div className="w-4 h-0.5 bg-blue-500/60"></div>
                              <ArrowRight className="w-4 h-4 text-blue-400 -ml-1" />
                            </div>
                          )}
                        </React.Fragment>
                      );
                    })}

                    <div className="flex items-center shrink-0 ml-1">
                      <div className="w-3 h-0.5 bg-slate-600"></div>
                      <span className="px-2 py-1 bg-slate-800 border border-slate-700 text-slate-400 rounded text-xs font-mono font-bold">
                        nil
                      </span>
                    </div>
                  </div>
                )
              )}

              {/* Vector Visualization if active */}
              {step.vector && (
                <div className="flex flex-col items-center gap-3">
                  <div className="flex gap-1.5 p-2 bg-slate-900/80 border border-slate-800 rounded-xl">
                    {step.vector.map((val, idx) => {
                      const isPri = step.pointers?.pri === idx;
                      const isUlt = step.pointers?.ult === idx;
                      const isMed = step.pointers?.medio === idx;
                      const isPos = step.pointers?.pos === idx;
                      const isI = step.pointers?.i === idx;

                      return (
                        <div key={idx} className="flex flex-col items-center">
                          <div className="h-6 flex items-center gap-0.5">
                            {isPri && <span className="text-[10px] bg-emerald-500/20 text-emerald-300 border border-emerald-500 px-1 rounded font-mono font-bold">pri</span>}
                            {isMed && <span className="text-[10px] bg-amber-500/30 text-amber-300 border border-amber-500 px-1 rounded font-mono font-bold">medio</span>}
                            {isUlt && <span className="text-[10px] bg-rose-500/20 text-rose-300 border border-rose-500 px-1 rounded font-mono font-bold">ult</span>}
                            {isPos && <span className="text-[10px] bg-blue-500/20 text-blue-300 border border-blue-500 px-1 rounded font-mono font-bold">pos</span>}
                            {isI && <span className="text-[10px] bg-purple-500/20 text-purple-300 border border-purple-500 px-1 rounded font-mono font-bold">i</span>}
                          </div>

                          <div
                            className={`w-14 h-14 rounded-lg border-2 flex items-center justify-center font-mono font-bold text-base transition-all ${
                              isMed
                                ? 'border-amber-400 bg-amber-950/50 text-amber-200 ring-2 ring-amber-400/40'
                                : val !== null
                                ? 'border-slate-700 bg-slate-800 text-white'
                                : 'border-dashed border-slate-800 bg-slate-950/50 text-slate-600'
                            }`}
                          >
                            {val !== null ? val : '∅'}
                          </div>
                          <span className="text-[10px] text-slate-500 font-mono mt-1">[{idx + 1}]</span>
                        </div>
                      );
                    })}
                  </div>
                  {step.dimL && (
                    <div className="text-xs text-slate-400 font-mono">
                      dimL = <strong className="text-blue-400">{step.dimL}</strong> | dimF = <strong className="text-slate-200">6</strong>
                    </div>
                  )}
                </div>
              )}
            </div>

            {/* Explanation box of current step */}
            <div className="p-4 bg-slate-950 border border-slate-800 rounded-xl flex items-start gap-3">
              <div className="p-1.5 rounded-lg bg-blue-500/10 border border-blue-500/20 text-blue-400 shrink-0 mt-0.5">
                <CheckCircle2 className="w-4 h-4" />
              </div>
              <div>
                <h4 className="text-xs font-bold uppercase tracking-wider text-blue-400 font-mono">
                  Acción en Tiempo Real
                </h4>
                <p className="text-slate-200 text-sm mt-0.5 leading-relaxed">
                  {step.explanation}
                </p>
              </div>
            </div>

            {/* Playback Controls */}
            <div className="flex items-center justify-between pt-4 mt-4 border-t border-slate-800">
              <button
                onClick={() => setCurrentStep(0)}
                className="p-2 text-slate-400 hover:text-white hover:bg-slate-800 rounded-lg transition"
                title="Reiniciar"
              >
                <RotateCcw className="w-4 h-4" />
              </button>

              <div className="flex items-center gap-2">
                <button
                  disabled={currentStep === 0}
                  onClick={() => setCurrentStep(prev => Math.max(0, prev - 1))}
                  className="px-3 py-1.5 text-xs font-medium bg-slate-800 hover:bg-slate-700 disabled:opacity-40 disabled:hover:bg-slate-800 text-slate-200 border border-slate-700 rounded-lg transition"
                >
                  Anterior
                </button>
                <button
                  onClick={() => setIsPlaying(!isPlaying)}
                  className="px-4 py-1.5 text-xs font-semibold bg-blue-600 hover:bg-blue-500 text-white rounded-lg transition flex items-center gap-1.5 shadow-sm"
                >
                  {isPlaying ? (
                    <>
                      <Pause className="w-3.5 h-3.5" /> Pausar
                    </>
                  ) : (
                    <>
                      <Play className="w-3.5 h-3.5" /> Reproducir
                    </>
                  )}
                </button>
                <button
                  disabled={currentStep === totalSteps - 1}
                  onClick={() => setCurrentStep(prev => Math.min(totalSteps - 1, prev + 1))}
                  className="px-3 py-1.5 text-xs font-medium bg-slate-800 hover:bg-slate-700 disabled:opacity-40 disabled:hover:bg-slate-800 text-slate-200 border border-slate-700 rounded-lg transition flex items-center gap-1"
                >
                  Siguiente <SkipForward className="w-3.5 h-3.5" />
                </button>
              </div>

              {/* Progress dots */}
              <div className="flex gap-1">
                {algo.steps.map((_, i) => (
                  <button
                    key={i}
                    onClick={() => setCurrentStep(i)}
                    className={`w-2 h-2 rounded-full transition-all ${
                      i === currentStep
                        ? 'bg-blue-400 w-4'
                        : i < currentStep
                        ? 'bg-blue-700'
                        : 'bg-slate-700'
                    }`}
                  />
                ))}
              </div>
            </div>
          </div>

        </div>

        {/* Right Col: Pascal Code & Docente Advice Side-by-Side (4 cols) */}
        <div className="lg:col-span-4 space-y-4">
          {/* Pascal Code Card */}
          <div className="bg-slate-900 border border-slate-800 rounded-2xl p-5 shadow-lg flex flex-col">
            <div className="flex items-center justify-between pb-3 border-b border-slate-800 mb-3">
              <span className="text-xs font-mono font-bold text-slate-300 uppercase tracking-wider flex items-center gap-2">
                <Code2 className="w-4 h-4 text-blue-400" /> Código Pascal Canónico
              </span>
              <span className="text-[11px] font-mono text-slate-400">
                {algo.category}
              </span>
            </div>

            {/* Code lines */}
            <div className="bg-slate-950 border border-slate-800/80 rounded-xl p-3.5 font-mono text-xs overflow-x-auto leading-relaxed max-h-[390px]">
              {algo.pascalCode.split('\n').map((line, idx) => {
                const lineNum = idx + 1;
                const isCurrent = step.activeLine === lineNum;

                return (
                  <div
                    key={idx}
                    className={`flex items-start gap-2.5 py-0.5 px-1.5 -mx-1 rounded transition-colors ${
                      isCurrent
                        ? 'bg-amber-500/20 text-amber-200 border-l-2 border-amber-400 font-semibold'
                        : 'text-slate-300 hover:bg-slate-900/60'
                    }`}
                  >
                    <span className="text-slate-600 select-none w-5 text-right shrink-0">
                      {lineNum}
                    </span>
                    <span className="whitespace-pre">{line}</span>
                  </div>
                );
              })}
            </div>
          </div>

          {/* Docente Advice Callout & Complexity */}
          <div className="bg-slate-900 border border-slate-800 rounded-2xl p-5 shadow-lg text-slate-300">
            <div className="flex items-center gap-2 text-amber-300 mb-2.5 font-semibold text-xs uppercase tracking-wide">
              <AlertTriangle className="w-4 h-4 text-amber-400 shrink-0" />
              <span>Criterio de Corrección UNLP</span>
            </div>
            <p className="text-xs leading-relaxed text-slate-300 bg-slate-950/70 p-3 rounded-xl border border-slate-800/80">
              {algo.complexity.docenteNote}
            </p>

            {/* Complexity badges */}
            <div className="mt-3 pt-3 border-t border-slate-800 grid grid-cols-2 gap-2 text-xs">
              <div className="bg-slate-950/80 p-2.5 rounded-xl border border-slate-800/80">
                <span className="text-slate-400 block text-[10px] uppercase font-mono">Tiempo:</span>
                <span className="text-emerald-400 font-semibold font-mono text-xs truncate block">{algo.complexity.time}</span>
              </div>
              <div className="bg-slate-950/80 p-2.5 rounded-xl border border-slate-800/80">
                <span className="text-slate-400 block text-[10px] uppercase font-mono">Memoria:</span>
                <span className="text-blue-400 font-semibold font-mono text-xs truncate block">{algo.complexity.space}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
