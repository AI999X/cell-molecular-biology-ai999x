# cell-molecular-biology-ai999x — offline, verified cell and molecular biology solver with step-by-step answers

[![tests](https://github.com/AI999X/cell-molecular-biology-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/cell-molecular-biology-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/cell-molecular-biology-ai999x)](https://pypi.org/project/cell-molecular-biology-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/cell-molecular-biology-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/cell-molecular-biology-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/cell-molecular-biology-ai999x/)** · **[Guides](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

cell-molecular-biology-ai999x is an offline, deterministic Python solver that answers cell and molecular biology questions with verified results from 94 certified capabilities, gives 619 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**cell-molecular-biology-ai999x** is a compact, deterministic cell and molecular biology engine for Python. It reads a question in plain English (or a typed call), picks a certified cell and molecular biology capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified cell and molecular biology capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 619 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10–3.14).
- **Coverage:** membranes and diffusion, cell growth, DNA and RNA, PCR and gene expression, enzyme kinetics and binding, solution biochemistry.

## Model overview

| Property | Value |
|---|---|
| Package | `cell-molecular-biology-ai999x` 1.0.0 (import `cell_molecular_biology_ai999x`) |
| Type | Deterministic neuro-symbolic cell and molecular biology engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 94 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 619 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python ) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install cell-molecular-biology-ai999x
```

```python
import cell_molecular_biology_ai999x as cell_molecular_biology

cell_molecular_biology.ask("Nernst potential: z = 1, C_out = 5 mM, C_in = 140 mM, T = 37 degC").show()
# -> SOLVED — E = -0.0890587, E_mV = -89.0587  (cmb_nernst_potential)
cell_molecular_biology.ask("Nearest-neighbour melting temperature: seq = CGTTCCAAAGATGTGGGCATGAGCTTAC, Na = 50 mM, c1 = 250 nM, c2 = 250 nM").show()
# -> SOLVED — Tm = 62.5816, dH = -221.7, dS = -628.766, dG37 = -26.6884  (cmb_tm_nearest_neighbor)
cell_molecular_biology.ask("qPCR delta-delta Ct: Ct_target_treated = 24.5, Ct_ref_treated = 18, Ct_target_control = 27, Ct_ref_control = 18.2").show()
# -> SOLVED — dCt_treated = 6.5, dCt_control = 8.8, ddCt = -2.3, fold_change = 4.92458  (cmb_qpcr_delta_delta_ct)
cell_molecular_biology.solve("cmb_hill_equation", L=0.026, K=0.026, n=2.8).show()
# -> SOLVED — theta = 0.5, theta_percent = 50  (cmb_hill_equation)
cell_molecular_biology.ask("Buffer recipe: C = 0.1 M, V = 1 L, pH = 7.4, pKa = 7.21").show()
# -> SOLVED — C_base = 60.7661, C_acid = 39.2339, n_base = 0.0607661, n_acid = 0.0392339  (cmb_buffer_preparation)
cell_molecular_biology.ask("Who created you?").message
# -> cell-molecular-biology-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs its own command: `cell-molecular-biology-ai999x "<question>"`, `cell-molecular-biology-ai999x --capabilities`. The outputs above were produced by running this exact code on cell-molecular-biology-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

94 certified procedures. A few of them:

- **Procedures:** A260/A280 and A260/A230 purity ratios; Basic GC melting temperature of an oligonucleotide; Batch culture time under Monod kinetics; Buffer capacity of a weak-acid buffer; Buffer pH after adding strong acid or base; Buffer pH by the Henderson-Hasselbalch equation; Cell concentration from a hemocytometer count; Cell volume after an osmotic change (Boyle-van 't Hoff); Cell-cycle phase durations corrected for the exponential age distribution; Cell-cycle phase durations from phase fractions; Colony-forming units per mL from a plate count; Competitive inhibition rate; Complex concentration with ligand depletion (quadratic binding); Copy number and moles of a DNA or RNA molecule from its mass; Diffusion coefficient from a FRAP half-time (Soumpasis); Diffusion-limited association rate constant (Smoluchowski); Dilution of a stock solution (C1 V1 = C2 V2); Dissociation constant from association and dissociation rates; Donnan equilibrium with an impermeant anion; Doubling time and number of generations from two counts; Enzyme activity and specific activity from a spectrophotometric assay; Equilibration of a solute across a spherical cell's membrane; Exponential growth of a cell population; Extinction coefficient of a DNA oligonucleotide at 260 nm (nearest neighbour) …

`cell_molecular_biology_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified cell and molecular biology capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (membranes and diffusion, cell growth, DNA and RNA, PCR and gene expression, enzyme kinetics and binding, solution biochemistry); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{cell_molecular_biology_ai999x_1_0_0,
  title  = {cell-molecular-biology-ai999x: an offline, verified cell and molecular biology solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/cell-molecular-biology-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/cell-molecular-biology-ai999x/discussions or visit https://ai999x.com.

## Support the project

cell-molecular-biology-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/cell-molecular-biology-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/cell-molecular-biology-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/tree/main/onnx).
