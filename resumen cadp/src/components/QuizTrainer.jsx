import React, { useState } from 'react';
import { QUIZ_QUESTIONS } from '../data/quizData';
import { HelpCircle, CheckCircle2, XCircle, RotateCcw, ArrowRight, Award, Tag, Sparkles } from 'lucide-react';

export default function QuizTrainer() {
  const [selectedCategory, setSelectedCategory] = useState('Todas');
  const [currentIndex, setCurrentIndex] = useState(0);
  const [userAnswers, setUserAnswers] = useState({}); // { [questionId]: boolean }
  const [revealed, setRevealed] = useState({}); // { [questionId]: boolean }

  const categories = ['Todas', ...new Set(QUIZ_QUESTIONS.map(q => q.category))];

  const filteredQuestions = selectedCategory === 'Todas'
    ? QUIZ_QUESTIONS
    : QUIZ_QUESTIONS.filter(q => q.category === selectedCategory);

  const currentQ = filteredQuestions[currentIndex] || filteredQuestions[0];
  const hasAnswered = userAnswers[currentQ.id] !== undefined;
  const isCorrect = userAnswers[currentQ.id] === currentQ.isTrue;

  const handleAnswer = (answerBool) => {
    if (hasAnswered) return;
    setUserAnswers(prev => ({ ...prev, [currentQ.id]: answerBool }));
    setRevealed(prev => ({ ...prev, [currentQ.id]: true }));
  };

  const handleNext = () => {
    if (currentIndex < filteredQuestions.length - 1) {
      setCurrentIndex(prev => prev + 1);
    }
  };

  const handlePrev = () => {
    if (currentIndex > 0) {
      setCurrentIndex(prev => prev - 1);
    }
  };

  const handleReset = () => {
    setUserAnswers({});
    setRevealed({});
    setCurrentIndex(0);
  };

  // Score stats
  const totalAnswered = Object.keys(userAnswers).length;
  const correctCount = Object.entries(userAnswers).filter(
    ([id, ans]) => {
      const q = QUIZ_QUESTIONS.find(item => item.id === Number(id));
      return q && q.isTrue === ans;
    }
  ).length;

  return (
    <div className="space-y-6">
      {/* Banner */}
      <div className="bg-slate-900/90 border border-slate-800 rounded-2xl p-6 shadow-xl">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
          <div>
            <h2 className="text-2xl font-bold text-white tracking-tight">
              Entrenador de Verdadero o Falso con Criterio de Cátedra
            </h2>
            <p className="text-slate-300 text-sm mt-1 max-w-3xl">
              En el final, poner solo "Verdadero" o "Falso" vale 0 puntos. Lo que califica el docente es la <strong className="text-blue-300">justificación conceptual</strong> usando el vocabulario técnico exacto de la materia.
            </p>
          </div>

          {/* Score Counter */}
          <div className="flex items-center gap-4 bg-slate-950/80 border border-slate-800 px-5 py-3 rounded-2xl self-start md:self-auto">
            <div>
              <span className="text-[10px] text-slate-400 uppercase font-mono block">Aciertos:</span>
              <span className="text-lg font-mono font-bold text-emerald-400">
                {correctCount} / {totalAnswered}
              </span>
            </div>
            <button
              onClick={handleReset}
              className="p-2 text-slate-400 hover:text-white hover:bg-slate-800 rounded-lg transition"
              title="Reiniciar cuestionario"
            >
              <RotateCcw className="w-4 h-4" />
            </button>
          </div>
        </div>

        {/* Category Pills */}
        <div className="flex flex-wrap gap-2 mt-6 pt-5 border-t border-slate-800">
          {categories.map(cat => (
            <button
              key={cat}
              onClick={() => {
                setSelectedCategory(cat);
                setCurrentIndex(0);
              }}
              className={`px-3 py-1.5 rounded-xl text-xs font-medium transition ${
                selectedCategory === cat
                  ? 'bg-blue-600 text-white font-semibold shadow-md'
                  : 'bg-slate-800/80 text-slate-300 hover:bg-slate-700'
              }`}
            >
              {cat}
            </button>
          ))}
        </div>
      </div>

      {/* Main Question Card */}
      <div className="bg-slate-900 border border-slate-800 rounded-2xl p-6 shadow-xl space-y-6">
        <div className="flex items-center justify-between pb-4 border-b border-slate-800">
          <span className="px-3 py-1 bg-slate-800 border border-slate-700 text-slate-300 rounded-lg text-xs font-mono font-semibold">
            {currentQ.category}
          </span>
          <span className="text-xs font-mono text-slate-400">
            Pregunta {currentIndex + 1} de {filteredQuestions.length}
          </span>
        </div>

        {/* Question Statement */}
        <div className="p-6 bg-slate-950 border border-slate-800 rounded-2xl">
          <p className="text-lg md:text-xl font-medium text-white leading-relaxed">
            "{currentQ.question}"
          </p>
        </div>

        {/* Choice Buttons */}
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <button
            disabled={hasAnswered}
            onClick={() => handleAnswer(true)}
            className={`py-4 px-6 rounded-2xl border-2 font-bold text-base transition-all flex items-center justify-center gap-2 ${
              hasAnswered
                ? currentQ.isTrue
                  ? 'border-emerald-500 bg-emerald-950/40 text-emerald-200'
                  : userAnswers[currentQ.id] === true
                  ? 'border-rose-500 bg-rose-950/40 text-rose-200'
                  : 'border-slate-800 bg-slate-950 text-slate-600'
                : 'border-slate-700 bg-slate-800/80 hover:bg-emerald-950/40 hover:border-emerald-500 text-slate-200'
            }`}
          >
            <CheckCircle2 className="w-5 h-5" /> VERDADERO
          </button>

          <button
            disabled={hasAnswered}
            onClick={() => handleAnswer(false)}
            className={`py-4 px-6 rounded-2xl border-2 font-bold text-base transition-all flex items-center justify-center gap-2 ${
              hasAnswered
                ? !currentQ.isTrue
                  ? 'border-emerald-500 bg-emerald-950/40 text-emerald-200'
                  : userAnswers[currentQ.id] === false
                  ? 'border-rose-500 bg-rose-950/40 text-rose-200'
                  : 'border-slate-800 bg-slate-950 text-slate-600'
                : 'border-slate-700 bg-slate-800/80 hover:bg-rose-950/40 hover:border-rose-500 text-slate-200'
            }`}
          >
            <XCircle className="w-5 h-5" /> FALSO
          </button>
        </div>

        {/* Justification Box (Appears when answered) */}
        {hasAnswered && (
          <div
            className={`p-6 rounded-2xl border transition-all space-y-4 ${
              isCorrect
                ? 'bg-emerald-950/30 border-emerald-600/40 text-emerald-200'
                : 'bg-rose-950/30 border-rose-600/40 text-rose-200'
            }`}
          >
            <div className="flex items-center gap-2">
              {isCorrect ? (
                <>
                  <CheckCircle2 className="w-5 h-5 text-emerald-400" />
                  <span className="font-bold text-sm text-emerald-400 uppercase tracking-wider font-mono">
                    ¡Respuesta Correcta!
                  </span>
                </>
              ) : (
                <>
                  <XCircle className="w-5 h-5 text-rose-400" />
                  <span className="font-bold text-sm text-rose-400 uppercase tracking-wider font-mono">
                    Respuesta Incorrecta (Es {currentQ.isTrue ? 'VERDADERO' : 'FALSO'})
                  </span>
                </>
              )}
            </div>

            <div>
              <h4 className="text-xs font-mono font-bold text-slate-400 uppercase tracking-wider mb-1">
                Justificación Canónica de Cátedra UNLP:
              </h4>
              <p className="text-slate-100 text-sm leading-relaxed">
                {currentQ.justification}
              </p>
            </div>

            {/* Docente Keywords */}
            {currentQ.docenteKeywords && (
              <div className="pt-3 border-t border-slate-800/80">
                <span className="text-[11px] font-mono text-slate-400 uppercase tracking-wider block mb-2 flex items-center gap-1.5">
                  <Tag className="w-3.5 h-3.5 text-amber-400" /> Palabras Clave Obligatorias en el Examen:
                </span>
                <div className="flex flex-wrap gap-2">
                  {currentQ.docenteKeywords.map((kw, i) => (
                    <span
                      key={i}
                      className="px-2.5 py-1 rounded-lg bg-slate-900 border border-slate-700 text-amber-300 font-mono text-xs font-medium"
                    >
                      {kw}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>
        )}

        {/* Stepper Footer */}
        <div className="flex items-center justify-between pt-4 border-t border-slate-800">
          <button
            disabled={currentIndex === 0}
            onClick={handlePrev}
            className="px-4 py-2 bg-slate-800 hover:bg-slate-700 disabled:opacity-40 text-slate-200 text-xs font-medium rounded-xl transition"
          >
            Pregunta Anterior
          </button>
          <button
            disabled={currentIndex === filteredQuestions.length - 1}
            onClick={handleNext}
            className="px-5 py-2 bg-blue-600 hover:bg-blue-500 disabled:opacity-40 text-white text-xs font-semibold rounded-xl transition flex items-center gap-1.5 shadow-md shadow-blue-500/20"
          >
            Siguiente Pregunta <ArrowRight className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    </div>
  );
}
