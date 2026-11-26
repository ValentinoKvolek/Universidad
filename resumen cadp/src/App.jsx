import React, { useState } from 'react';
import {
  Code2,
  Cpu,
  Bug,
  Calculator,
  Award,
  BookOpen,
  GraduationCap
} from 'lucide-react';

import AlgorithmLab from './components/AlgorithmLab';
import MemorySimulator from './components/MemorySimulator';
import TraceDebugger from './components/TraceDebugger';
import UdtMemoryCalculator from './components/UdtMemoryCalculator';
import QuizTrainer from './components/QuizTrainer';
import CanonicalSummary from './components/CanonicalSummary';
import FinalExamsBank from './components/FinalExamsBank';

export default function App() {
  const [activeTab, setActiveTab] = useState('algo');

  const tabs = [
    { id: 'algo', name: 'Algoritmos', icon: Code2 },
    { id: 'memory', name: 'Stack vs Heap', icon: Cpu },
    { id: 'trace', name: 'Trazas', icon: Bug },
    { id: 'calc', name: 'Calculadora UDT', icon: Calculator },
    { id: 'quiz', name: 'Trainer V/F', icon: Award },
    { id: 'summary', name: 'Resumen Teórico', icon: BookOpen },
    { id: 'finals', name: 'Banco de Finales', icon: GraduationCap }
  ];

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-blue-600 selection:text-white">
      {/* Top Header & Integrated Navigation */}
      <header className="sticky top-0 z-40 bg-slate-950/95 backdrop-blur-md border-b border-slate-800 shadow-lg">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-3">
          <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-3">
            {/* Logo & Title */}
            <div className="flex items-center gap-3">
              <div className="w-9 h-9 rounded-xl bg-gradient-to-tr from-blue-600 to-indigo-600 flex items-center justify-center shadow-md shadow-blue-500/20 text-white font-mono font-bold text-base">
                Ψ
              </div>
              <div>
                <h1 className="font-bold text-white text-base tracking-tight leading-tight">
                  CADP Final Master
                </h1>
                <p className="text-[11px] text-slate-400">
                  Conceptos de Algoritmos, Datos y Programación • UNLP
                </p>
              </div>
            </div>

            {/* Navigation Tabs - Responsive Grid/Bar (No Horizontal Scrollbar) */}
            <nav className="grid grid-cols-2 sm:grid-cols-4 lg:flex lg:items-center gap-1.5 p-1 bg-slate-900/90 rounded-2xl border border-slate-800">
              {tabs.map(tab => {
                const Icon = tab.icon;
                const isActive = activeTab === tab.id;

                return (
                  <button
                    key={tab.id}
                    onClick={() => setActiveTab(tab.id)}
                    className={`px-3 py-1.5 rounded-xl text-xs font-medium transition-all flex items-center justify-center gap-1.5 ${
                      isActive
                        ? 'bg-blue-600 text-white font-semibold shadow-sm shadow-blue-600/30 border border-blue-400/30'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/60'
                    }`}
                  >
                    <Icon className={`w-3.5 h-3.5 shrink-0 ${isActive ? 'text-white' : 'text-slate-500'}`} />
                    <span className="truncate">{tab.name}</span>
                  </button>
                );
              })}
            </nav>
          </div>
        </div>
      </header>

      {/* Main Content Area */}
      <main className="flex-1 max-w-[1440px] w-full mx-auto px-4 sm:px-6 lg:px-8 py-6">
        {activeTab === 'algo' && <AlgorithmLab />}
        {activeTab === 'memory' && <MemorySimulator />}
        {activeTab === 'trace' && <TraceDebugger />}
        {activeTab === 'calc' && <UdtMemoryCalculator />}
        {activeTab === 'quiz' && <QuizTrainer />}
        {activeTab === 'summary' && <CanonicalSummary />}
        {activeTab === 'finals' && <FinalExamsBank />}
      </main>

      {/* Footer */}
      <footer className="border-t border-slate-900 bg-slate-950 py-5 text-xs text-slate-500">
        <div className="max-w-7xl mx-auto px-4 flex flex-col sm:flex-row items-center justify-between gap-3">
          <p className="text-center sm:text-left">
            Plataforma interactiva para el Examen Final de <strong>CADP (UNLP)</strong>.
          </p>
          <div className="flex items-center gap-3 text-slate-400">
            <a
              href="https://github.com/ValentinoKvolek"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-slate-900/80 hover:bg-slate-800 text-slate-300 hover:text-white border border-slate-800 hover:border-slate-700 transition-colors font-medium text-xs group"
            >
              <svg className="w-4 h-4 fill-current text-slate-400 group-hover:text-white transition-colors" viewBox="0 0 24 24">
                <path fillRule="evenodd" clipRule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.53 1.032 1.53 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" />
              </svg>
              <span>ValentinoKvolek</span>
            </a>
            <span className="hidden sm:inline text-slate-700">•</span>
            <span className="hidden sm:inline text-[11px] text-slate-500">
              Algoritmos • Memoria • UDT
            </span>
          </div>
        </div>
      </footer>
    </div>
  );
}
