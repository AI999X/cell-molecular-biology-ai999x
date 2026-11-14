"""cell-molecular-biology-ai999x — offline cell and molecular biology answers with their formula, steps and verification.

    import cell_molecular_biology_ai999x as m
    m.ask("Nernst potential: z = 1, C_out = 5 mM, C_in = 140 mM, T = 37 degC").show()
    m.solve("cmb_hill_equation", L=0.026, K=0.026, n=2.8)
    m.capabilities()                   # every certified capability

94 certified capabilities: membranes and diffusion (Fick's law, diffusion times and RMS displacement, Stokes-Einstein,
membrane permeability and cell equilibration, osmotic pressure, osmolarity and tonicity, Boyle-van 't Hoff volumes,
Nernst and Donnan potentials, channel currents, diffusion-limited rates, FRAP); cell growth and division (exponential
and logistic growth, doubling time, growth-rate fits, Monod and batch Monod growth, cell-cycle phase durations,
hemocytometer and plate counts, transformation efficiency, cell geometry, molecules per cell); DNA and RNA (GC content,
Wallace, basic, salt-adjusted and nearest-neighbour melting temperatures, oligo molecular weight and extinction
coefficient, A260 concentration and purity ratios, protein by A280 and Warburg-Christian, copy number, ligation ratios,
reverse complement, transcription, translation, ORFs, sequencing coverage, library size, mutation rate); gene
expression (PCR amplification and cycles, qPCR delta-delta Ct, Pfaffl ratio, standard curves, transcription and
translation times, steady states, induction dynamics, mRNA half-lives); enzyme kinetics and binding (Michaelis-Menten,
kcat/Km, Lineweaver-Burk and nonlinear fits, competitive, uncompetitive, noncompetitive and mixed inhibition,
Cheng-Prusoff, Hill, MWC, fraction bound, ligand depletion, Kd from rates, Scatchard, binding free energy, Q10, assay
activity); solution biochemistry (Henderson-Hasselbalch, buffer recipes and capacity, C1V1 and serial dilutions,
molarity, per cent w/v, ionic strength, centrifugation and sedimentation).
Questions use one documented form, "<capability phrase>: name = value, ..." with units (M, mM, uM and nM are read);
a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.6.1"
_P = Product("cell_molecular_biology", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__, "engine": _P.engine()})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
figures, engine = _P.figures, _P.engine   # live figures and the engine in use: the Rust engine on Windows/Linux x86_64
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "figures", "engine", "__version__"]
