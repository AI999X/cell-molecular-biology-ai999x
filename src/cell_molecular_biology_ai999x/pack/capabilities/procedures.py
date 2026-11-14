"""Cell and molecular biology procedures: deterministic capabilities with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Only what can be calculated is a capability; descriptive biology is not.

Topics: membranes and diffusion (Fick's first law, mean-square displacement and diffusion times, Stokes-Einstein,
membrane permeability, equilibration of a cell across its membrane, osmotic pressure, osmolarity and tonicity, the
Boyle-van 't Hoff cell volume, Nernst and Donnan potentials, single-channel currents, diffusion-limited association,
FRAP); cell growth and division (exponential growth, doubling time, growth-rate fits, logistic growth, Monod kinetics
and batch Monod growth, cell-cycle phase durations by fraction and by the exponential age distribution, hemocytometer
counts, plate counts, transformation efficiency, cell geometry and molecules per cell); DNA and RNA (GC content, the
Wallace, basic and salt-adjusted melting temperatures, nearest-neighbour melting temperature, oligo molecular weight and
extinction coefficient, concentration from A260, absorbance ratios, protein by A280 and Warburg-Christian, copy number,
ligation ratios, reverse complement, transcription, translation and open reading frames, genome coverage, library size,
mutation rate by the p0 method); gene expression (PCR amplification and cycles needed, qPCR delta-delta Ct, the Pfaffl
ratio, standard curves and quantification, transcription and translation times, steady states, induction dynamics,
mRNA half-life fits); enzyme kinetics and binding (Michaelis-Menten, the substrate for a fraction of Vmax, kcat and
catalytic efficiency, Lineweaver-Burk and nonlinear fits, competitive, uncompetitive, noncompetitive and mixed
inhibition, Cheng-Prusoff, the Hill equation, Hill coefficients from EC10 and EC90, fraction bound, ligand depletion,
Kd from rates, Scatchard fits, binding free energy, competitive occupancy, the MWC model, Q10, assay activity) and
solution biochemistry (Henderson-Hasselbalch, buffer preparation, buffer capacity, buffers after acid or base, C1V1
and serial dilutions, molarity and mass, percent w/v, ionic strength, centrifugation and sedimentation); microscopy
(magnification, Abbe resolution, specimen size from a micrograph); water potential and the direction of water
movement; free energy of metabolism (at cellular concentrations, from reduction potentials); ATP yield per glucose;
microbial killing (D-value, z-value, thermal death time).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported inside
the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, re).

How the references differ from the shipped code:
- Biopython for sequences: gc_fraction, MeltingTemp.Tm_Wallace, Tm_GC (user parameter set and value set 7) and Tm_NN
  (table DNA_NN4, salt correction 5), molecular_weight with IUPACData weights, Seq.reverse_complement, transcribe and
  translate (standard table) and a six-frame ORF search on Biopython's translations;
- scipy: solve_bvp for steady diffusion profiles, solve_ivp (DOP853) for growth, equilibration, induction and
  sedimentation, quad for Monod batch time, logistic time, cell-cycle age distributions, Gaussian displacements and
  Smoluchowski fluxes, brentq for every inverse problem, stats.linregress for every straight-line fit, optimize.curve_fit
  for the nonlinear Michaelis-Menten fit, special.i0e/i1e for the Soumpasis FRAP curve, special.expit for Hill curves,
  stats.poisson for coverage and fluctuation tests, scipy.constants for R, F, k, N_A and g;
- numpy linear solves of rapid-equilibrium enzyme and binding schemes and of gene-expression steady states; sympy
  derivatives for buffer capacity and the MWC partition function;
- weaker references (the same published formula evaluated a different way) are listed in procedures_certified.json.

Conventions (fixed, so run and reference agree exactly):
- SI internally (concentrations mol/m^3 = mM, rates 1/s, times s); companion outputs give customary units (mM, uM, nM,
  ug/mL, h, min, mV, kJ/mol). Concentrations may be written in M, mM, uM, nM, pM (read here as mol/L, ...), molar
  masses in g/mol, Da or kDa, osmolarities in Osm/L or mOsm/L; the shared unit reader has no "M", "Da", "Osm" or "d", so
  this module rewrites them before the runtime converts the value exactly once.
- Constants: k = 1.380649e-23 J/K, e = 1.602176634e-19 C, N_A = 6.02214076e23 /mol (exact SI), R = k N_A, F = e N_A,
  g = 9.80665 m/s^2. Temperatures are read in K or degC.
- Sequences are written 5'->3' in A, C, G, T (or U for RNA), case ignored; any other letter is refused.
- Melting temperatures: Wallace 2(A+T) + 4(G+C); basic 64.9 + 41(G+C-16.4)/N (N >= 14); salt-adjusted 81.5 + 16.6
  log10[Na+] + 0.41 %GC - 600/N (N >= 14); nearest neighbour with SantaLucia-Hicks (2004) unified parameters, the
  SantaLucia (1998) entropy correction 0.368 (N-1) ln[Na+], R = 1.987 cal/(mol K), k = c1 - c2/2 (self-complementary:
  c1), as Biopython computes them.
- Oligo molecular weights: Biopython's average nucleotide-monophosphate weights minus (n-1) waters (5'-phosphate); the
  5'-hydroxyl form subtracts HPO3. Extinction coefficients at 260 nm by the nearest-neighbour table of Cantor and
  Tinoco (single strand). A260 factors: dsDNA 50, ssDNA 33, RNA 40 ug/mL per unit at 1 cm. Copy number from NEB mass
  formulas (dsDNA 617.96 n + 36.04, ssDNA 308.97 n + 18.02, RNA 320.5 n + 159.0 g/mol).
- Open reading frames: ATG to the first in-frame stop, the most upstream ATG after the previous stop, on both strands;
  the longest wins, ties broken by + strand first, then the lowest start; coordinates 1-based on the given strand.
- Cell-cycle phases are given in the order G1, S, G2, M; the age-corrected form uses the exponential age distribution
  n(a) = 2 ln 2 / T 2^(-a/T).
- Tonicity: isotonic within 1 % of the cell's osmolarity.
- Lists carry no unit: their unit is stated in each input (hours, minutes, mM, cells per square, cycles).
- Results built from exp, log or roots are rounded to 12 significant digits and exact zeros are written as 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`Fick diffusion flux: D = 1000 um^2/s, C1 = 10 mM, C2 = 2 mM, dx = 10 um`). Physical values carry a unit; fractions may
be written with %; lists in brackets; choices and sequences as words. Any other number, an unknown name or a name given
twice is refused, never guessed.
"""
from __future__ import annotations

import math
import re

# ================================================================================================ constants
K_B = 1.380649e-23               # J/K
Q_E = 1.602176634e-19            # C
N_A = 6.02214076e23              # 1/mol
R_GAS = K_B * N_A                # J/(mol K)
FARADAY = Q_E * N_A              # C/mol
G0 = 9.80665                     # m/s^2
T0C = 273.15                     # K
LN2 = math.log(2.0)
HOUR = 3600.0
MINUTE = 60.0


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    if abs(a) < 1e-15 and abs(b) < 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _isnum(v):
    return isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v)


def _fin(*vals):
    for v in vals:
        _need(_isnum(v), "every input must be a finite number")


def _pos(name, *vals):
    for v in vals:
        _need(_isnum(v) and v > 0, f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(_isnum(v) and v >= 0, f"{name} must be zero or positive")


def _frac(name, v, lo_open=True, hi_open=True):
    _need(_isnum(v), f"{name} must be a number")
    _need((v > 0 if lo_open else v >= 0) and (v < 1 if hi_open else v <= 1), f"{name} must be a fraction between 0 and 1")


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sig(x, n=4):
    """x rounded to n significant digits (random inputs written as a user would write them)."""
    return float(f"{x:.{n}g}")


def _mean(xs):
    s = 0.0
    for x in xs:
        s += x
    return s / len(xs)


def _linfit(x, y):
    """Ordinary least squares y = a + b x: (slope, intercept, r^2)."""
    n = len(x)
    mx, my = _mean(x), _mean(y)
    sxx, sxy, syy = 0.0, 0.0, 0.0
    for i in range(n):
        dx, dy = x[i] - mx, y[i] - my
        sxx += dx * dx
        sxy += dx * dy
        syy += dy * dy
    _need(sxx > 0, "the x values must not all be equal")
    b = sxy / sxx
    a = my - b * mx
    r2 = 1.0 if syy == 0 else sxy * sxy / (sxx * syy)
    return b, a, r2


def _bisect(f, a, b, n=400):
    """Root of f on [a, b] by bisection (f(a), f(b) of opposite signs), to full double precision."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need((fa > 0) != (fb > 0), "no root in the interval")
    for _ in range(n):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm > 0) == (fa > 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


# ---------------------------------------------------------------------------------------------- reference helpers
def _brentq(f, a, b):
    from scipy.optimize import brentq
    return float(brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500))


def _quad(f, a, b):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=400)
    return float(v)


def _ivp(fun, t_end, y0, rtol=1e-13):
    """Reference helper: scipy DOP853 integration from 0 to t_end; returns the final state."""
    from scipy.integrate import solve_ivp
    atol = [1e-30 + 1e-20 * max(1e-300, abs(v)) for v in y0]
    sol = solve_ivp(fun, (0.0, t_end), list(y0), method="DOP853", rtol=rtol, atol=atol)
    if not sol.success:
        raise RuntimeError(sol.message)
    return [float(v) for v in sol.y[:, -1]]


def _linregress(x, y):
    from scipy.stats import linregress
    r = linregress(x, y)
    return float(r.slope), float(r.intercept), float(r.rvalue) ** 2


def _sc():
    import scipy.constants as sc
    return sc


# ================================================================================================ units
_KU = {"temperature": "K", "voltage": "V", "current": "A", "length": "m", "area": "m^2", "volume": "m^3", "time": "s",
       "mass": "kg", "pressure": "Pa", "amount": "mol", "molar_concentration": "mol/m^3", "mass_concentration": "kg/m^3",
       "molar_mass": "kg/mol", "diffusion_coefficient": "m^2/s", "molar_flux": "mol/(m^2*s)",
       "membrane_permeability": "m/s", "dynamic_viscosity": "Pa*s", "first_order_rate_constant": "1/s",
       "frequency": "Hz", "second_order_rate_constant": "m^3/(mol*s)", "reaction_rate": "mol/(m^3*s)",
       "molar_absorptivity": "m^2/mol", "molar_energy": "J/mol", "conductance": "S", "angular_frequency": "rad/s",
       "number": ""}

RPM = 2.0 * math.pi / 60.0       # rad/s per rpm

# how a value of a kind is written in templates and questions (unit, factor from SI); SI when absent
_SHOW = {"molar_concentration": ("mM", 1.0), "mass_concentration": ("ug/mL", 1e3), "molar_mass": ("g/mol", 1e3),
         "diffusion_coefficient": ("um^2/s", 1e12), "membrane_permeability": ("cm/s", 1e2),
         "dynamic_viscosity": ("mPa*s", 1e3), "first_order_rate_constant": ("1/h", HOUR), "frequency": ("1/h", HOUR),
         "second_order_rate_constant": ("1/(M*s)", 1e3), "reaction_rate": ("uM/min", 1e3 * MINUTE),
         "molar_absorptivity": ("L/(mol*cm)", 10.0), "molar_energy": ("kJ/mol", 1e-3), "conductance": ("pS", 1e12),
         "angular_frequency": ("rpm", 1.0 / RPM), "voltage": ("mV", 1e3), "current": ("pA", 1e12),
         "amount": ("mmol", 1e3), "volume": ("mL", 1e6), "mass": ("mg", 1e6), "pressure": ("kPa", 1e-3)}

# unit words the shared reader lacks, rewritten here (each value is still converted exactly once, by the runtime)
_UNIT_WORDS = {"M": "(mol/L)", "mM": "(mmol/L)", "uM": "(umol/L)", "µM": "(umol/L)", "μM": "(umol/L)",
               "nM": "(nmol/L)", "pM": "(pmol/L)", "fM": "(fmol/L)", "Da": "(g/mol)", "kDa": "(kg/mol)",
               "d": "day", "Osm": "mol", "mOsm": "mmol", "uOsm": "umol"}


def _unit_rewrite(u):
    """Rewrite molar (M), dalton (Da), osmole (Osm) and day (d) words, and the micro sign, for the shared reader."""
    out, tok = "", ""
    for ch in u + " ":
        if ch.isalpha() or ch == "µ" or ch == "μ":
            tok += ch
            continue
        if tok:
            if tok in _UNIT_WORDS:
                out += _UNIT_WORDS[tok]
            else:
                out += tok.replace("µ", "u").replace("μ", "u")
            tok = ""
        out += ch
    return out[:-1]


def _written_unit(num, unit, kind):
    """'value unit' for the runtime."""
    return f"{num} {_unit_rewrite(unit.strip())}"


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-']*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]]*\])\s*$")
_PCT = {"%": 0.01, "percent": 0.01, "per cent": 0.01}


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _squash(text):
    """A choice word without case, spaces, hyphens, apostrophes or underscores ('ds DNA' and 'dsDNA' agree)."""
    s = ""
    for ch in text.lower():
        if ch not in " -_'":
            s += ch
    return s


def _choice_value(text, choices):
    """The documented choice a written word names, compared without case, spaces, hyphens or underscores; a word
    that names no choice is passed on as written (the runtime then refuses it as an invalid choice)."""
    w = _squash(text.strip())
    for c in choices:
        if _squash(c) == w:
            return c
    return text.strip()


def _reader(phrases, inputs, aliases=None):
    """The one documented question form, '<phrase>: name = value, ...', for a procedure with these inputs.

    The reading guard is built in: every stated number is bound to a named input (any other text or number refuses
    the whole question); the capability phrase must open the question and be followed by the colon; a physical value
    without its unit, or a unit on a pure number, is refused; values are passed with their units so the runtime
    converts each exactly once."""
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    names = {}
    by = {}
    for i in inputs:
        names[i["symbol"]] = i["symbol"]
        by[i["symbol"]] = i
    for i in inputs:                       # a lower-case spelling only where it names no other input
        names.setdefault(i["symbol"].lower(), i["symbol"])
    for a, s in (aliases or {}).items():
        names[a] = s
        names[a.lower()] = s

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mv = _VEC.match(seg)
            if mv:
                key = names.get(mv.group(1), names.get(mv.group(1).lower()))
                if key is None or key in out or by[key].get("type") != "vector":
                    return None
                out[key] = mv.group(2).strip()
                continue
            mw = _WORD.match(seg)
            if mw:
                key = names.get(mw.group(1), names.get(mw.group(1).lower()))
                if key is not None and key not in out and by[key].get("type") == "choice":
                    out[key] = _choice_value(mw.group(2), by[key]["choices"])
                    continue
                if key is not None and key not in out and by[key].get("type") == "text":
                    out[key] = mw.group(2).strip()
                    continue
                return None
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = names.get(ms.group(1), names.get(ms.group(1).lower()))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            num, unit = ms.group(2), ms.group(3).strip()
            if t in ("vector", "choice", "text"):
                return None
            kind = spec.get("kind", "number")
            if t == "integer":
                if unit and unit not in spec.get("words", ()):
                    return None
                out[key] = num
                continue
            if kind != "number":
                if not unit:
                    return None
                out[key] = _written_unit(num, unit, kind)
                continue
            if unit:
                if spec.get("pct") and unit.lower() in _PCT:
                    out[key] = float(num) * _PCT[unit.lower()]
                    continue
                if unit in spec.get("words", ()):
                    out[key] = float(num)
                    continue
                return None
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '20' for a fraction: 20 % or 20? refused; write 20% or 0.2
            if spec.get("pct_required"):
                return None                       # a per cent w/v written without '%' (0.9 = 0.9 % or 90 %?) is refused
            out[key] = float(num)
        return out
    return parse


def _fmt(x):
    return f"{x:.10g}"


def _written(spec, v):
    """An input value as a user writes it in the documented form."""
    t = spec.get("type", "number")
    if t == "vector":
        return "[" + ", ".join(_fmt(float(x)) for x in v) + "]"
    if t in ("choice", "text"):
        return str(v)
    if t == "integer":
        if spec.get("words"):
            return f"{int(v)} {spec['words'][0]}"
        return str(int(v))
    kind = spec.get("kind", "number")
    if kind != "number":
        unit, f = spec.get("show") or _SHOW.get(kind) or (_KU[kind], 1.0)
        if unit == "degC":
            return f"{_fmt(float(v) - T0C)} degC"
        return f"{_fmt(float(v) * f)} {unit}"
    if spec.get("pct"):
        return f"{_fmt(float(v) * 100.0)}%"
    if spec.get("words"):
        return f"{_fmt(float(v))} {spec['words'][0]}"
    return _fmt(float(v))


# ================================================================================================ input/output specs
def _phys(sym, kind, meaning, required=True, show=None):
    _KU[kind]                                   # every physical kind has its SI unit here
    d = {"symbol": sym, "kind": kind, "meaning": meaning, "required": required}
    if show:
        d["show"] = show
    return d


def _numin(sym, meaning, required=True, pct=False, words=None, pct_required=False):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if pct or pct_required:
        d["pct"] = True
    if pct_required:
        d["pct_required"] = True
    if words:
        d["words"] = list(words)
    return d


def _intin(sym, meaning, lo=None, hi=None, required=True, words=None, nonzero=False):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    if words:
        d["words"] = list(words)
    if nonzero:
        d["condition"] = f"{sym} != 0"
    return d


def _vecin(sym, meaning, lo=1, hi=50, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _textin(sym, meaning, required=True, lo=1, hi=2000):
    return {"symbol": sym, "type": "text", "meaning": meaning, "required": required, "min_length": lo, "max_length": hi}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}
WEAK: dict = {}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), aliases=None, extra_q=(), verify=None, weak=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them."""
    parse = _reader(phrases, inputs, aliases)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex_inputs):
        return f"{phrases[0]}: " + ", ".join(f"{k} = {_written(by[k], v)}" for k, v in ex_inputs.items())
    templates = [ask_text(e["inputs"]) for e in examples[:2]]
    questions = [{"q": ask_text(e["inputs"]), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        partial = {k: v for k, v in examples[0]["inputs"].items() if k != req[0]}
        questions.append({"q": ask_text(partial), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]["inputs"]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    questions += [{"q": q["q"], "outputs": examples[q["like"]]["outputs"]} if "like" in q else q for q in extra_q]
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": list(examples), "questions": questions}
    LEVELS[key] = level
    if weak:
        WEAK[key] = weak


def _ex(inputs, outputs=None):
    return {"inputs": inputs, "outputs": dict(outputs or {})}


_UM = ("um", 1e6)
_NM = ("nm", 1e9)
_CM = ("cm", 1e2)
_UMOLAR = ("uM", 1e3)
_NMOLAR = ("nM", 1e6)
_HOURS = ("h", 1.0 / HOUR)
_MINUTES = ("min", 1.0 / MINUTE)
_SECONDS = ("s", 1.0)
_PER_S = ("1/s", 1.0)
_PER_MIN = ("1/min", MINUTE)
_ML = ("mL", 1e6)
_UL = ("uL", 1e9)
_FL = ("fL", 1e18)
_NG = ("ng", 1e12)
_KELVIN = ("K", 1.0)


# ================================================================================================ MEMBRANES AND DIFFUSION
def fick_flux(D, C1, C2, dx):
    _pos("the diffusion coefficient D and the distance dx", D, dx)
    _nonneg("the concentrations C1 and C2", C1, C2)
    J = D * (C1 - C2) / dx
    return _out(J=J, J_nmol_cm2_s=J * 1e5)


def _ref_fick_flux(D, C1, C2, dx):
    import numpy as np
    from scipy.integrate import solve_bvp
    x = np.linspace(0.0, dx, 11)
    sol = solve_bvp(lambda s, y: np.vstack([y[1], np.zeros_like(y[0])]),
                    lambda ya, yb: np.array([ya[0] - C1, yb[0] - C2]), x, np.zeros((2, x.size)), tol=1e-12)
    J = -D * float(sol.sol(0.0)[1])
    return {"J": J, "J_nmol_cm2_s": J * 1e5}


def _rand_fick(rng):
    return {"D": _sig(_lu(rng, 1e-12, 2e-9), 3), "C1": _sig(rng.uniform(0.5, 150.0), 3),
            "C2": _sig(rng.uniform(0.0, 100.0), 3), "dx": _sig(_lu(rng, 5e-9, 1e-3), 3)}


_add("cmb_fick_flux",
     name="Fick's first law: steady diffusion flux", topic="membranes and diffusion / Fick's law",
     purpose="The steady one-dimensional diffusive flux between two concentrations a distance apart (positive from side 1 to side 2).",
     inputs=[_phys("D", "diffusion_coefficient", "diffusion coefficient of the solute"),
             _phys("C1", "molar_concentration", "concentration on side 1"),
             _phys("C2", "molar_concentration", "concentration on side 2"),
             _phys("dx", "length", "distance between the two sides", show=_UM)],
     outputs=[_o("J", "molar_flux", "molar flux from side 1 to side 2"),
              _o("J_nmol_cm2_s", "number", "flux in nmol per cm^2 per second")],
     primary="J", conditions=["D > 0, dx > 0", "C1, C2 >= 0", "steady state, linear profile"],
     method="J = D (C1 - C2) / dx (Fick's first law with a linear steady profile)",
     latex=r"J = -D\frac{dC}{dx} = D\,\frac{C_1 - C_2}{\Delta x}",
     run=fick_flux, reference=_ref_fick_flux,
     wrong=lambda D, C1, C2, dx: _out(J=D * (C1 - C2) * dx, J_nmol_cm2_s=D * (C1 - C2) * dx * 1e5),
     rand=_rand_fick,
     phrases=("Fick diffusion flux", "Fick's first law flux", "Diffusion flux"),
     examples=[_ex({"D": 1e-9, "C1": 10.0, "C2": 2.0, "dx": 1e-5}), _ex({"D": 5e-10, "C1": 0.15, "C2": 0.0, "dx": 2e-6})],
     level="beginner",
     extra_q=[{"q": "Diffusion flux: D = 1e-5 cm^2/s, C1 = 10 mM, C2 = 2 mM, dx = 10 um", "like": 0}])


def rms_displacement(D, t, dims):
    _pos("D and t", D, t)
    _need(dims in (1, 2, 3), "dims must be 1, 2 or 3")
    msd = 2.0 * dims * D * t
    rms = math.sqrt(msd)
    return _out(rms=rms, rms_um=rms * 1e6, msd=msd)


def _ref_rms(D, t, dims):
    from scipy.stats import norm
    s = math.sqrt(2.0 * D * t)
    per_axis = _quad(lambda x: x * x * norm.pdf(x, scale=s), -40.0 * s, 40.0 * s)
    msd = dims * per_axis
    return {"rms": math.sqrt(msd), "rms_um": math.sqrt(msd) * 1e6, "msd": msd}


_add("cmb_rms_displacement",
     name="Root-mean-square displacement of a diffusing particle", topic="membranes and diffusion / random walk",
     purpose="The mean-square and root-mean-square displacement of a particle diffusing freely for a time t in 1, 2 or 3 dimensions.",
     inputs=[_phys("D", "diffusion_coefficient", "diffusion coefficient"),
             _phys("t", "time", "elapsed time", show=_SECONDS),
             _intin("dims", "number of dimensions (1, 2 or 3)", lo=1, hi=3)],
     outputs=[_o("rms", "length", "root-mean-square displacement"),
              _o("rms_um", "number", "root-mean-square displacement in micrometres"),
              _o("msd", "area", "mean-square displacement")],
     primary="rms", conditions=["D > 0, t > 0", "dims = 1, 2 or 3"],
     method="<r^2> = 2 d D t; rms = sqrt(2 d D t)",
     latex=r"\langle r^2\rangle = 2dDt,\quad r_{\mathrm{rms}} = \sqrt{2dDt}",
     run=rms_displacement, reference=_ref_rms,
     wrong=lambda D, t, dims: (lambda m: _out(rms=math.sqrt(m), rms_um=math.sqrt(m) * 1e6, msd=m))(dims * D * t),
     rand=lambda rng: {"D": _sig(_lu(rng, 1e-13, 2e-9), 3), "t": _sig(_lu(rng, 1e-3, 1e4), 3), "dims": rng.choice([1, 2, 3])},
     phrases=("Root-mean-square displacement", "RMS displacement", "Mean-square displacement"),
     examples=[_ex({"D": 1e-9, "t": 1.0, "dims": 3}), _ex({"D": 1e-12, "t": 60.0, "dims": 2})],
     level="beginner",
     extra_q=[{"q": "RMS displacement: D = 1000 um^2/s, t = 1000 ms, dims = 3", "like": 0}])


def diffusion_time(x, D, dims):
    _pos("x and D", x, D)
    _need(dims in (1, 2, 3), "dims must be 1, 2 or 3")
    t = x * x / (2.0 * dims * D)
    return _out(t=t, t_min=t / MINUTE)


def _ref_diffusion_time(x, D, dims):
    from scipy.stats import norm
    def msd(t):
        return dims * norm.var(scale=math.sqrt(2.0 * D * t))
    tg = x * x / D
    t = _brentq(lambda tt: msd(tt) / (x * x) - 1.0, tg * 1e-3, tg * 10.0)
    return {"t": t, "t_min": t / MINUTE}


_add("cmb_diffusion_time",
     name="Time to diffuse a distance", topic="membranes and diffusion / random walk",
     purpose="The time for the root-mean-square displacement of a diffusing molecule to reach a distance x in 1, 2 or 3 dimensions.",
     inputs=[_phys("x", "length", "distance (root-mean-square displacement to reach)", show=_UM),
             _phys("D", "diffusion_coefficient", "diffusion coefficient"),
             _intin("dims", "number of dimensions (1, 2 or 3)", lo=1, hi=3)],
     outputs=[_o("t", "time", "diffusion time"), _o("t_min", "number", "diffusion time in minutes")],
     primary="t", conditions=["x > 0, D > 0", "dims = 1, 2 or 3"],
     method="t = x^2 / (2 d D)", latex=r"t = \frac{x^2}{2dD}",
     run=diffusion_time, reference=_ref_diffusion_time,
     wrong=lambda x, D, dims: _out(t=x * x / (dims * D), t_min=x * x / (dims * D) / MINUTE),
     rand=lambda rng: {"x": _sig(_lu(rng, 1e-8, 1e-2), 3), "D": _sig(_lu(rng, 1e-13, 2e-9), 3), "dims": rng.choice([1, 2, 3])},
     phrases=("Diffusion time", "Time to diffuse"),
     examples=[_ex({"x": 1e-5, "D": 1e-9, "dims": 3}), _ex({"x": 1e-3, "D": 5e-10, "dims": 1})],
     level="beginner",
     extra_q=[{"q": "Time to diffuse: x = 0.01 mm, D = 1000 um^2/s, dims = 3", "like": 0}])


def stokes_einstein(T, eta, r):
    _pos("T, eta and r", T, eta, r)
    f = 6.0 * math.pi * eta * r
    D = K_B * T / f
    return _out(D=D, D_um2_s=D * 1e12, friction=f)


def _ref_stokes_einstein(T, eta, r):
    sc = _sc()
    f = 6.0 * sc.pi * eta * r
    D = sc.k * T / f
    return {"D": D, "D_um2_s": D * 1e12, "friction": f}


_add("cmb_stokes_einstein",
     name="Stokes-Einstein diffusion coefficient", topic="membranes and diffusion / diffusion coefficient",
     purpose="The diffusion coefficient of a sphere of hydrodynamic radius r in a fluid of viscosity eta at temperature T.",
     inputs=[_phys("T", "temperature", "absolute temperature (degC is converted)", show=_KELVIN),
             _phys("eta", "dynamic_viscosity", "viscosity of the medium"),
             _phys("r", "length", "hydrodynamic radius", show=_NM)],
     outputs=[_o("D", "diffusion_coefficient", "diffusion coefficient"),
              _o("D_um2_s", "number", "diffusion coefficient in um^2/s"),
              _o("friction", "number", "friction coefficient 6 pi eta r in kg/s")],
     primary="D", conditions=["T, eta, r > 0", "a sphere at low Reynolds number"],
     method="D = k T / (6 pi eta r)", latex=r"D = \frac{k_B T}{6\pi\eta r}",
     run=stokes_einstein, reference=_ref_stokes_einstein,
     wrong=lambda T, eta, r: (lambda f: _out(D=K_B * T / f, D_um2_s=K_B * T / f * 1e12, friction=f))(4.0 * math.pi * eta * r),
     rand=lambda rng: {"T": _sig(rng.uniform(277.0, 315.0), 4), "eta": _sig(_lu(rng, 6e-4, 5e-2), 3), "r": _sig(_lu(rng, 2e-10, 1e-6), 3)},
     phrases=("Stokes-Einstein diffusion coefficient", "Stokes-Einstein equation"),
     examples=[_ex({"T": 298.15, "eta": 0.00089, "r": 2.5e-9}), _ex({"T": 310.15, "eta": 0.0007, "r": 5e-8})],
     level="intermediate",
     extra_q=[{"q": "Stokes-Einstein equation: T = 25 degC, eta = 0.89 mPa*s, r = 2.5 nm", "like": 0}],
     weak="the same Stokes-Einstein relation evaluated with scipy.constants")


def membrane_permeability(D, K, L, C_out, C_in):
    _pos("D, K and L", D, K, L)
    _nonneg("the concentrations", C_out, C_in)
    P = K * D / L
    J = P * (C_out - C_in)
    return _out(P=P, P_cm_s=P * 100.0, J=J)


def _ref_membrane_permeability(D, K, L, C_out, C_in):
    import numpy as np
    from scipy.integrate import solve_bvp
    x = np.linspace(0.0, L, 11)
    sol = solve_bvp(lambda s, y: np.vstack([y[1], np.zeros_like(y[0])]),
                    lambda ya, yb: np.array([ya[0] - K * C_out, yb[0] - K * C_in]), x, np.zeros((2, x.size)), tol=1e-12)
    J = -D * float(sol.sol(0.0)[1])
    dC = C_out - C_in
    P = J / dC if dC != 0 else K * D / L
    return {"P": P, "P_cm_s": P * 100.0, "J": J}


def _rand_perm(rng):
    co = _sig(rng.uniform(1.0, 150.0), 3)
    ci = _sig(rng.uniform(0.0, 150.0), 3)
    if ci == co:
        ci = _sig(co * 0.5, 3)
    return {"D": _sig(_lu(rng, 1e-12, 1e-9), 3), "K": _sig(_lu(rng, 1e-4, 10.0), 3), "L": _sig(rng.uniform(3e-9, 1e-8), 3),
            "C_out": co, "C_in": ci}


_add("cmb_membrane_permeability",
     name="Membrane permeability coefficient and flux", topic="membranes and diffusion / permeability",
     purpose="The permeability coefficient P = K D / L of a lipid membrane for a solute with partition coefficient K, and the inward flux P (C_out - C_in).",
     inputs=[_phys("D", "diffusion_coefficient", "diffusion coefficient of the solute inside the membrane"),
             _numin("K", "membrane/water partition coefficient"),
             _phys("L", "length", "membrane thickness", show=_NM),
             _phys("C_out", "molar_concentration", "concentration outside the cell"),
             _phys("C_in", "molar_concentration", "concentration inside the cell")],
     outputs=[_o("P", "membrane_permeability", "permeability coefficient"),
              _o("P_cm_s", "number", "permeability coefficient in cm/s"),
              _o("J", "molar_flux", "inward flux (positive into the cell)")],
     primary="P", conditions=["D, K, L > 0", "concentrations >= 0", "steady state, linear profile in the membrane"],
     method="P = K D / L; J = P (C_out - C_in)", latex=r"P = \frac{K D}{L},\quad J = P\,(C_{out} - C_{in})",
     run=membrane_permeability, reference=_ref_membrane_permeability,
     wrong=lambda D, K, L, C_out, C_in: (lambda P: _out(P=P, P_cm_s=P * 100.0, J=P * (C_out - C_in)))(D / (K * L)),
     rand=_rand_perm,
     phrases=("Membrane permeability", "Permeability coefficient"),
     examples=[_ex({"D": 1e-10, "K": 0.01, "L": 5e-9, "C_out": 10.0, "C_in": 1.0}),
               _ex({"D": 5e-11, "K": 2.0, "L": 4e-9, "C_out": 0.5, "C_in": 2.0})],
     level="beginner",
     extra_q=[{"q": "Permeability coefficient: D = 100 um^2/s, K = 0.01, L = 5 nm, C_out = 10 mM, C_in = 1000 uM", "like": 0}])


def cell_equilibration(r, P, C_out, C0, t):
    _pos("r and P", r, P)
    _nonneg("the concentrations and t", C_out, C0, t)
    k = 3.0 * P / r
    C = C_out + (C0 - C_out) * math.exp(-k * t)
    return _out(C=C, tau=1.0 / k, t_half=LN2 / k)


def _ref_cell_equilibration(r, P, C_out, C0, t):
    k = P * (4.0 * math.pi * r * r) / (4.0 / 3.0 * math.pi * r ** 3)
    C = _ivp(lambda tt, y: [k * (C_out - y[0])], t, [C0])[0] if t > 0 else C0
    tau = 1.0 / k
    th = _brentq(lambda s: math.exp(-k * s) - 0.5, 0.0, 10.0 * tau)
    return {"C": C, "tau": tau, "t_half": th}


def _rand_equil(rng):
    r = _sig(_lu(rng, 1e-6, 5e-5), 3)
    P = _sig(_lu(rng, 1e-10, 1e-5), 3)
    tau = r / (3.0 * P)
    return {"r": r, "P": P, "C_out": _sig(rng.uniform(0.5, 100.0), 3), "C0": _sig(rng.uniform(0.0, 50.0), 3),
            "t": _sig(tau * rng.uniform(0.05, 3.0), 3)}


_add("cmb_cell_equilibration",
     name="Equilibration of a solute across a spherical cell's membrane", topic="membranes and diffusion / permeability",
     purpose="The internal concentration of a permeant solute at time t for a spherical cell (A/V = 3/r) bathed at a constant outside concentration, with the time constant and half-time.",
     inputs=[_phys("r", "length", "cell radius", show=_UM),
             _phys("P", "membrane_permeability", "membrane permeability coefficient"),
             _phys("C_out", "molar_concentration", "constant outside concentration"),
             _phys("C0", "molar_concentration", "initial inside concentration"),
             _phys("t", "time", "elapsed time", show=_SECONDS)],
     outputs=[_o("C", "molar_concentration", "inside concentration at time t"),
              _o("tau", "time", "time constant r/(3P)"), _o("t_half", "time", "half-time of equilibration")],
     primary="C", conditions=["r, P > 0", "C_out, C0, t >= 0", "well-mixed cell of constant volume, outside unchanged"],
     method="dC/dt = P (A/V) (C_out - C), A/V = 3/r: C = C_out + (C0 - C_out) exp(-3Pt/r)",
     latex=r"C(t) = C_{out} + (C_0 - C_{out})\,e^{-3Pt/r}",
     run=cell_equilibration, reference=_ref_cell_equilibration,
     wrong=lambda r, P, C_out, C0, t: (lambda k: _out(C=C_out + (C0 - C_out) * math.exp(-k * t), tau=1.0 / k, t_half=LN2 / k))(P / r),
     rand=_rand_equil,
     phrases=("Cell equilibration across the membrane", "Membrane equilibration"),
     examples=[_ex({"r": 1e-5, "P": 1e-7, "C_out": 10.0, "C0": 0.0, "t": 30.0}),
               _ex({"r": 5e-6, "P": 2e-8, "C_out": 1.0, "C0": 5.0, "t": 60.0})],
     level="intermediate",
     extra_q=[{"q": "Membrane equilibration: r = 10 um, P = 1e-5 cm/s, C_out = 10 mM, C0 = 0 mM, t = 0.5 min", "like": 0}])


def osmotic_pressure(i, c, T):
    _pos("the van 't Hoff factor i, the concentration c and T", i, c, T)
    pi = i * c * R_GAS * T
    return _out(pi=pi, pi_atm=pi / 101325.0, pi_kPa=pi / 1000.0)


def _ref_osmotic_pressure(i, c, T):
    sc = _sc()
    pi = i * c * sc.R * T
    return {"pi": pi, "pi_atm": pi / sc.atm, "pi_kPa": pi / sc.kilo}


_add("cmb_osmotic_pressure",
     name="Osmotic pressure (van 't Hoff)", topic="membranes and diffusion / osmosis",
     purpose="The osmotic pressure of a dilute solution from the solute concentration, the van 't Hoff factor and the temperature.",
     inputs=[_numin("i", "van 't Hoff factor (particles per formula unit, e.g. 2 for NaCl)"),
             _phys("c", "molar_concentration", "solute concentration"),
             _phys("T", "temperature", "absolute temperature (degC is converted)", show=_KELVIN)],
     outputs=[_o("pi", "pressure", "osmotic pressure"), _o("pi_atm", "number", "osmotic pressure in atm"),
              _o("pi_kPa", "number", "osmotic pressure in kPa")],
     primary="pi", conditions=["i, c, T > 0", "ideal dilute solution"],
     method="Pi = i c R T", latex=r"\Pi = i\,c\,R\,T",
     run=osmotic_pressure, reference=_ref_osmotic_pressure,
     wrong=lambda i, c, T: _out(pi=c * R_GAS * T, pi_atm=c * R_GAS * T / 101325.0, pi_kPa=c * R_GAS * T / 1000.0),
     rand=lambda rng: {"i": rng.choice([1.0, 2.0, 3.0, 1.8, 1.9]), "c": _sig(_lu(rng, 1.0, 1000.0), 3), "T": _sig(rng.uniform(273.15, 315.0), 5)},
     phrases=("Osmotic pressure", "Van 't Hoff osmotic pressure"),
     examples=[_ex({"i": 2.0, "c": 150.0, "T": 310.15}), _ex({"i": 1.0, "c": 300.0, "T": 298.15})],
     level="beginner",
     extra_q=[{"q": "Osmotic pressure: i = 2, c = 0.15 M, T = 37 degC", "like": 0}],
     weak="the same van 't Hoff relation evaluated with scipy.constants")


def osmolarity_tonicity(c, i, osm_cell):
    _need(len(c) == len(i) and len(c) >= 1, "give one van 't Hoff factor per concentration")
    for v in c:
        _need(_isnum(v) and v >= 0, "concentrations must be zero or positive")
    for v in i:
        _need(_isnum(v) and v > 0, "van 't Hoff factors must be positive")
    _pos("the cell's osmolarity", osm_cell)
    osm = 0.0
    for k in range(len(c)):
        osm += c[k] * i[k]
    cell = osm_cell
    diff = osm - cell
    if abs(diff) <= 0.01 * cell:
        word = "isotonic"
    elif diff > 0:
        word = "hypertonic"
    else:
        word = "hypotonic"
    return _out(osmolarity_mOsm=osm, difference_mOsm=diff, tonicity=word)


def _ref_osmolarity(c, i, osm_cell):
    import numpy as np
    osm = float(np.dot(np.array(c, dtype=float), np.array(i, dtype=float)))
    rel = (osm - osm_cell) / osm_cell
    word = "isotonic" if abs(rel) <= 0.01 else ("hypertonic" if rel > 0 else "hypotonic")
    return {"osmolarity_mOsm": osm, "difference_mOsm": osm - osm_cell, "tonicity": word}


def _rand_osm(rng):
    n = rng.randint(1, 4)
    c = [_sig(rng.uniform(1.0, 200.0), 3) for _ in range(n)]
    i = [rng.choice([1.0, 2.0, 3.0]) for _ in range(n)]
    return {"c": c, "i": i, "osm_cell": _sig(rng.uniform(250.0, 350.0), 3)}


_add("cmb_osmolarity_tonicity",
     name="Osmolarity of a solution and its tonicity for a cell", topic="membranes and diffusion / osmosis",
     purpose="The total osmolarity of a solution of several solutes (sum of i c) and whether it is hypertonic, hypotonic or isotonic (within 1 %) relative to the cell's osmolarity, assuming impermeant solutes.",
     inputs=[_vecin("c", "solute concentrations in mM (one per solute)", 1, 20),
             _vecin("i", "van 't Hoff factor of each solute", 1, 20),
             _phys("osm_cell", "molar_concentration", "osmolarity of the cell interior", show=("mOsm/L", 1.0))],
     outputs=[_o("osmolarity_mOsm", "number", "osmolarity of the solution in mOsm/L"),
              _o("difference_mOsm", "number", "solution minus cell osmolarity in mOsm/L"),
              _o("tonicity", "number", "hypertonic, hypotonic or isotonic (within 1 %)")],
     primary="osmolarity_mOsm", conditions=["one factor per concentration", "solutes impermeant (tonicity = osmolarity)"],
     method="osmolarity = sum i_k c_k; compared with the cell's osmolarity (isotonic within 1 %)",
     latex=r"\mathrm{osm} = \sum_k i_k c_k",
     run=osmolarity_tonicity, reference=_ref_osmolarity,
     wrong=lambda c, i, osm_cell: osmolarity_tonicity(c, [1.0] * len(c), osm_cell) if any(v != 1.0 for v in i) else osmolarity_tonicity(c, [2.0] * len(c), osm_cell),
     rand=_rand_osm,
     phrases=("Osmolarity and tonicity", "Tonicity of a solution", "Osmolarity"),
     examples=[_ex({"c": [150.0], "i": [2.0], "osm_cell": 300.0}), _ex({"c": [100.0, 50.0], "i": [2.0, 1.0], "osm_cell": 290.0})],
     level="beginner",
     extra_q=[{"q": "Tonicity of a solution: c = [150], i = [2], osm_cell = 0.3 Osm/L", "like": 0}],
     weak="numpy dot product and the same 1 % rule")


def cell_volume_osmotic(V0, b, osm0, osm1):
    _pos("V0, osm0 and osm1", V0, osm0, osm1)
    _frac("the osmotically inactive fraction b", b, lo_open=False)
    Vb = b * V0
    V = Vb + (V0 - Vb) * osm0 / osm1
    return _out(V=V, V_fL=V * 1e18, ratio=V / V0)


def _ref_cell_volume(V0, b, osm0, osm1):
    Vb = b * V0
    n = osm0 * (V0 - Vb)
    V = _brentq(lambda v: n / (v - Vb) - osm1, Vb + 1e-12 * V0, 1e3 * V0)
    return {"V": V, "V_fL": V * 1e18, "ratio": V / V0}


_add("cmb_cell_volume_osmotic",
     name="Cell volume after an osmotic change (Boyle-van 't Hoff)", topic="membranes and diffusion / osmosis",
     purpose="The equilibrium volume of a cell moved from osmolarity osm0 to osm1, with an osmotically inactive volume fraction b (an ideal osmometer).",
     inputs=[_phys("V0", "volume", "initial cell volume", show=_FL),
             _numin("b", "osmotically inactive fraction of the initial volume (0 to 1)", pct=True),
             _phys("osm0", "molar_concentration", "initial (isotonic) osmolarity", show=("mOsm/L", 1.0)),
             _phys("osm1", "molar_concentration", "new outside osmolarity", show=("mOsm/L", 1.0))],
     outputs=[_o("V", "volume", "new equilibrium cell volume"), _o("V_fL", "number", "new volume in femtolitres"),
              _o("ratio", "number", "new volume relative to the initial volume")],
     primary="V", conditions=["V0, osm0, osm1 > 0", "0 <= b < 1", "impermeant solutes, water in equilibrium"],
     method="V = Vb + (V0 - Vb) osm0 / osm1 with Vb = b V0",
     latex=r"V = V_b + (V_0 - V_b)\,\frac{\mathrm{osm}_0}{\mathrm{osm}_1}",
     run=cell_volume_osmotic, reference=_ref_cell_volume,
     wrong=lambda V0, b, osm0, osm1: _out(V=V0 * osm0 / osm1, V_fL=V0 * osm0 / osm1 * 1e18, ratio=osm0 / osm1),
     rand=lambda rng: {"V0": _sig(_lu(rng, 1e-17, 1e-12), 3), "b": _sig(rng.uniform(0.05, 0.6), 3),
                       "osm0": _sig(rng.uniform(270.0, 320.0), 3), "osm1": _sig(_lu(rng, 80.0, 1500.0), 3)},
     phrases=("Cell volume after an osmotic change", "Boyle-van 't Hoff cell volume"),
     examples=[_ex({"V0": 9e-17, "b": 0.4, "osm0": 300.0, "osm1": 600.0}), _ex({"V0": 2e-15, "b": 0.2, "osm0": 290.0, "osm1": 150.0})],
     level="intermediate",
     extra_q=[{"q": "Boyle-van 't Hoff cell volume: V0 = 90 um^3, b = 40%, osm0 = 300 mOsm/L, osm1 = 0.6 Osm/L", "like": 0}])


def nernst_potential(z, C_out, C_in, T):
    _need(isinstance(z, int) and not isinstance(z, bool) and z != 0, "the charge number z must be a non-zero whole number")
    _pos("the concentrations and T", C_out, C_in, T)
    E = R_GAS * T / (z * FARADAY) * math.log(C_out / C_in)
    return _out(E=E, E_mV=E * 1e3)


def _ref_nernst(z, C_out, C_in, T):
    sc = _sc()
    g = lambda E: z * sc.physical_constants["Faraday constant"][0] * E + sc.R * T * math.log(C_in / C_out)
    E = _brentq(g, -2.0, 2.0)
    return {"E": E, "E_mV": E * 1e3}


def _rand_nernst(rng):
    return {"z": rng.choice([1, 1, 2, -1, 2, -1, 3]), "C_out": _sig(_lu(rng, 0.001, 500.0), 3),
            "C_in": _sig(_lu(rng, 0.0001, 500.0), 3), "T": _sig(rng.uniform(280.0, 315.0), 5)}


_add("cmb_nernst_potential",
     name="Nernst equilibrium potential of an ion across a cell membrane", topic="membranes and diffusion / membrane potential",
     purpose="The equilibrium (reversal) potential inside relative to outside for an ion of charge z from its outside and inside concentrations.",
     inputs=[_intin("z", "charge number of the ion (e.g. 1 for K+, 2 for Ca2+, -1 for Cl-)", lo=-3, hi=3, nonzero=True),
             _phys("C_out", "molar_concentration", "outside concentration"),
             _phys("C_in", "molar_concentration", "inside concentration"),
             _phys("T", "temperature", "absolute temperature (degC is converted)", show=_KELVIN)],
     outputs=[_o("E", "voltage", "equilibrium potential (inside minus outside)"), _o("E_mV", "number", "equilibrium potential in mV")],
     primary="E", conditions=["z a non-zero whole number", "concentrations > 0", "T > 0"],
     method="E = (R T / z F) ln(C_out / C_in)", latex=r"E = \frac{RT}{zF}\ln\frac{C_{out}}{C_{in}}",
     run=nernst_potential, reference=_ref_nernst,
     wrong=lambda z, C_out, C_in, T: (lambda E: _out(E=E, E_mV=E * 1e3))(R_GAS * T / (z * FARADAY) * math.log10(C_out / C_in)),
     rand=_rand_nernst,
     phrases=("Nernst potential", "Nernst equilibrium potential", "Equilibrium potential of an ion"),
     examples=[_ex({"z": 1, "C_out": 5.0, "C_in": 140.0, "T": 310.15}), _ex({"z": 2, "C_out": 2.0, "C_in": 0.0001, "T": 310.15})],
     level="beginner",
     extra_q=[{"q": "Nernst potential: z = 1, C_out = 5 mM, C_in = 0.14 M, T = 37 degC", "like": 0},
              {"q": "Nernst potential: z = 0, C_out = 5 mM, C_in = 140 mM, T = 310 K", "outcome": "INVALID_INPUT"}])


def donnan_equilibrium(A, zA, c, T):
    _pos("the impermeant anion concentration A, the salt concentration c and T", A, c, T)
    _need(isinstance(zA, int) and not isinstance(zA, bool) and zA >= 1, "zA is the magnitude of the anion's charge (1, 2, 3)")
    q = zA * A
    Cl_in = (-q + math.sqrt(q * q + 4.0 * c * c)) / 2.0
    K_in = Cl_in + q
    E = R_GAS * T / FARADAY * math.log(c / K_in)
    return _out(K_in=K_in, Cl_in=Cl_in, ratio=K_in / c, E=E, E_mV=E * 1e3, excess_mOsm=K_in + Cl_in + A - 2.0 * c)


def _ref_donnan(A, zA, c, T):
    sc = _sc()
    q = zA * A
    r = _brentq(lambda rr: c * rr - c / rr - q, 1.0, 1.0 + q / c + 10.0)
    K_in, Cl_in = c * r, c / r
    E = sc.R * T / sc.physical_constants["Faraday constant"][0] * math.log(1.0 / r)
    return {"K_in": K_in, "Cl_in": Cl_in, "ratio": r, "E": E, "E_mV": E * 1e3, "excess_mOsm": K_in + Cl_in + A - 2.0 * c}


_add("cmb_donnan_equilibrium",
     name="Donnan equilibrium with an impermeant anion", topic="membranes and diffusion / membrane potential",
     purpose="The inside cation and anion concentrations, the Donnan ratio, the Donnan potential and the osmotic excess when a cell holds impermeant anions (charge -zA) bathed in a monovalent salt (KCl) of concentration c.",
     inputs=[_phys("A", "molar_concentration", "concentration of impermeant anions inside"),
             _intin("zA", "magnitude of the impermeant anion's charge (1, 2 or 3)", lo=1, hi=3),
             _phys("c", "molar_concentration", "outside concentration of the monovalent salt"),
             _phys("T", "temperature", "absolute temperature (degC is converted)", show=_KELVIN)],
     outputs=[_o("K_in", "molar_concentration", "inside cation concentration"),
              _o("Cl_in", "molar_concentration", "inside permeant anion concentration"),
              _o("ratio", "number", "Donnan ratio K_in / K_out = Cl_out / Cl_in"),
              _o("E", "voltage", "Donnan potential (inside minus outside)"), _o("E_mV", "number", "Donnan potential in mV"),
              _o("excess_mOsm", "number", "inside minus outside osmolarity in mOsm/L")],
     primary="K_in", conditions=["A, c, T > 0", "zA = 1, 2 or 3", "ideal solutions, electroneutral compartments"],
     method="K_in Cl_in = c^2 and K_in = Cl_in + zA A: Cl_in = (-zA A + sqrt(zA^2 A^2 + 4c^2))/2; E = (RT/F) ln(c/K_in)",
     latex=r"[\mathrm{Cl}]_i = \frac{-z_A A + \sqrt{z_A^2A^2 + 4c^2}}{2},\quad E = \frac{RT}{F}\ln\frac{c}{[\mathrm{K}]_i}",
     run=donnan_equilibrium, reference=_ref_donnan,
     wrong=lambda A, zA, c, T: (lambda K: _out(K_in=K, Cl_in=c, ratio=K / c, E=-R_GAS * T / FARADAY * math.log(c / K),
                                                E_mV=-R_GAS * T / FARADAY * math.log(c / K) * 1e3, excess_mOsm=K + A - c))(c + zA * A),
     rand=lambda rng: {"A": _sig(_lu(rng, 1.0, 200.0), 3), "zA": rng.choice([1, 1, 2, 3]), "c": _sig(_lu(rng, 5.0, 200.0), 3),
                       "T": _sig(rng.uniform(280.0, 315.0), 5)},
     phrases=("Donnan equilibrium", "Donnan potential"),
     examples=[_ex({"A": 100.0, "zA": 1, "c": 100.0, "T": 298.15}), _ex({"A": 50.0, "zA": 2, "c": 150.0, "T": 310.15})],
     level="advanced")


def channel_current(N, Po, gamma, V, E):
    _need(isinstance(N, int) and N >= 1, "N must be a whole number of channels, at least 1")
    _frac("the open probability Po", Po, lo_open=False, hi_open=False)
    _pos("the single-channel conductance gamma", gamma)
    _fin(V, E)
    G = N * Po * gamma
    I = G * (V - E)
    return _out(I=I, I_pA=I * 1e12, G=G, i_single=gamma * (V - E))


def _ref_channel_current(N, Po, gamma, V, E):
    from fractions import Fraction as Fr
    G = Fr(N) * Fr(Po) * Fr(gamma)
    I = G * (Fr(V) - Fr(E))
    return {"I": float(I), "I_pA": float(I) * 1e12, "G": float(G), "i_single": float(Fr(gamma) * (Fr(V) - Fr(E)))}


_add("cmb_channel_current",
     name="Macroscopic current through ion channels", topic="membranes and diffusion / ion channels",
     purpose="The whole-cell current I = N Po gamma (V - E) through N channels of single-channel conductance gamma and open probability Po, and the single-channel current.",
     inputs=[_intin("N", "number of channels", lo=1),
             _numin("Po", "open probability (0 to 1)", pct=True),
             _phys("gamma", "conductance", "single-channel conductance"),
             _phys("V", "voltage", "membrane potential"), _phys("E", "voltage", "reversal potential of the channel")],
     outputs=[_o("I", "current", "macroscopic current (positive outward)"), _o("I_pA", "number", "current in pA"),
              _o("G", "conductance", "total open conductance N Po gamma"),
              _o("i_single", "current", "single open-channel current gamma (V - E)")],
     primary="I", conditions=["N >= 1", "0 <= Po <= 1", "gamma > 0", "ohmic open channels"],
     method="I = N Po gamma (V - E)", latex=r"I = N P_o \gamma\,(V - E_{rev})",
     run=channel_current, reference=_ref_channel_current,
     wrong=lambda N, Po, gamma, V, E: _out(I=N * gamma * (V - E), I_pA=N * gamma * (V - E) * 1e12, G=N * gamma, i_single=gamma * (V - E)),
     rand=lambda rng: {"N": rng.randint(1, 5000), "Po": _sig(rng.uniform(0.02, 0.9), 3), "gamma": _sig(_lu(rng, 1e-12, 2e-10), 3),
                       "V": _sig(rng.uniform(-0.1, 0.05), 3), "E": _sig(rng.uniform(-0.1, 0.06), 3)},
     phrases=("Ion channel current", "Channel current"),
     examples=[_ex({"N": 1000, "Po": 0.3, "gamma": 2e-11, "V": -0.04, "E": -0.09}),
               _ex({"N": 50, "Po": 0.8, "gamma": 1e-10, "V": 0.0, "E": 0.05})],
     level="intermediate",
     extra_q=[{"q": "Channel current: N = 1000, Po = 30%, gamma = 0.02 nS, V = -40 mV, E = -90 mV", "like": 0}],
     weak="the same relation evaluated in exact rational arithmetic")


def smoluchowski_rate(DA, DB, rA, rB):
    _pos("the diffusion coefficients and radii", DA, DB, rA, rB)
    k = 4.0 * math.pi * (DA + DB) * (rA + rB) * N_A
    return _out(k=k, k_per_M_s=k * 1e3)


def _ref_smoluchowski(DA, DB, rA, rB):
    sc = _sc()
    D, R = DA + DB, rA + rB
    resistance = _quad(lambda x: 1.0 / (4.0 * sc.pi * D * (R / x) ** 2) * R / (x * x), 1e-300, 1.0)   # r = R/x
    k = sc.N_A / resistance
    return {"k": k, "k_per_M_s": k * 1e3}


_add("cmb_diffusion_limited_rate",
     name="Diffusion-limited association rate constant (Smoluchowski)", topic="membranes and diffusion / diffusion-limited reactions",
     purpose="The maximum (diffusion-limited) bimolecular association rate constant of two spheres that react on contact.",
     inputs=[_phys("DA", "diffusion_coefficient", "diffusion coefficient of molecule A"),
             _phys("DB", "diffusion_coefficient", "diffusion coefficient of molecule B"),
             _phys("rA", "length", "radius of molecule A", show=_NM), _phys("rB", "length", "radius of molecule B", show=_NM)],
     outputs=[_o("k", "second_order_rate_constant", "diffusion-limited rate constant"),
              _o("k_per_M_s", "number", "rate constant in 1/(M s)")],
     primary="k", conditions=["all inputs > 0", "reaction on every contact, no interaction forces"],
     method="k = 4 pi (DA + DB)(rA + rB) N_A", latex=r"k_D = 4\pi (D_A + D_B)(r_A + r_B) N_A",
     run=smoluchowski_rate, reference=_ref_smoluchowski,
     wrong=lambda DA, DB, rA, rB: _out(k=4.0 * math.pi * DA * rA * N_A, k_per_M_s=4.0 * math.pi * DA * rA * N_A * 1e3),
     rand=lambda rng: {"DA": _sig(_lu(rng, 1e-11, 2e-9), 3), "DB": _sig(_lu(rng, 1e-11, 2e-9), 3),
                       "rA": _sig(_lu(rng, 2e-10, 1e-8), 3), "rB": _sig(_lu(rng, 2e-10, 1e-8), 3)},
     phrases=("Diffusion-limited rate constant", "Smoluchowski rate constant"),
     examples=[_ex({"DA": 1e-10, "DB": 1e-9, "rA": 2e-9, "rB": 5e-10}), _ex({"DA": 6e-11, "DB": 6e-11, "rA": 3e-9, "rB": 3e-9})],
     level="advanced",
     weak="the steady radial flux into an absorbing sphere integrated numerically (scipy quad)")


def _soumpasis(x):
    """exp(-x) (I0(x) + I1(x)) by the power series (x up to 20): the Soumpasis FRAP recovery at x = 2 tauD / t."""
    term0, term1 = 1.0, x / 2.0
    s0, s1 = term0, term1
    k = 0
    while True:
        k += 1
        term0 *= (x / 2.0) ** 2 / (k * k)
        term1 *= (x / 2.0) ** 2 / (k * (k + 1))
        s0 += term0
        s1 += term1
        if term0 < 1e-18 * s0 and term1 < 1e-18 * s1:
            break
        if k > 500:
            break
    return math.exp(-x) * (s0 + s1)


def _soumpasis_root():
    """x* with exp(-x)(I0(x) + I1(x)) = 1/2 (the half-recovery point)."""
    return _bisect(lambda x: _soumpasis(x) - 0.5, 0.1, 10.0)


def frap_diffusion(w, t_half):
    _pos("the bleach radius w and the half-time t_half", w, t_half)
    xs = _soumpasis_root()
    tauD = xs * t_half / 2.0
    D = w * w / (4.0 * tauD)
    return _out(D=D, D_um2_s=D * 1e12, tauD=tauD, gamma=tauD / t_half)


def _ref_frap(w, t_half):
    from scipy.special import i0e, i1e
    xs = _brentq(lambda x: float(i0e(x) + i1e(x)) - 0.5, 0.1, 10.0)
    tauD = xs * t_half / 2.0
    D = w * w / (4.0 * tauD)
    return {"D": D, "D_um2_s": D * 1e12, "tauD": tauD, "gamma": tauD / t_half}


_add("cmb_frap_diffusion",
     name="Diffusion coefficient from a FRAP half-time (Soumpasis)", topic="membranes and diffusion / FRAP",
     purpose="The two-dimensional diffusion coefficient from the half-time of fluorescence recovery after photobleaching a uniform circular spot of radius w (Soumpasis' exact solution, D = 0.224 w^2 / t_half).",
     inputs=[_phys("w", "length", "radius of the bleached spot", show=_UM), _phys("t_half", "time", "half-time of recovery", show=_SECONDS)],
     outputs=[_o("D", "diffusion_coefficient", "diffusion coefficient"), _o("D_um2_s", "number", "D in um^2/s"),
              _o("tauD", "time", "characteristic diffusion time w^2/(4D)"), _o("gamma", "number", "tauD / t_half (about 1.12)")],
     primary="D", conditions=["w, t_half > 0", "2-D free diffusion, uniform circular bleach, full mobile fraction"],
     method="f(t) = exp(-2 tauD/t)[I0(2 tauD/t) + I1(2 tauD/t)]; solve f(t_half) = 1/2 for tauD; D = w^2/(4 tauD)",
     latex=r"f(t) = e^{-2\tau_D/t}\left[I_0\!\left(\tfrac{2\tau_D}{t}\right) + I_1\!\left(\tfrac{2\tau_D}{t}\right)\right],\quad D = \frac{w^2}{4\tau_D}",
     run=frap_diffusion, reference=_ref_frap,
     wrong=lambda w, t_half: _out(D=w * w / (4.0 * t_half), D_um2_s=w * w / (4.0 * t_half) * 1e12, tauD=t_half, gamma=1.0),
     rand=lambda rng: {"w": _sig(_lu(rng, 2e-7, 2e-5), 3), "t_half": _sig(_lu(rng, 0.05, 200.0), 3)},
     phrases=("FRAP diffusion coefficient", "Diffusion coefficient from FRAP"),
     examples=[_ex({"w": 1e-6, "t_half": 0.5}), _ex({"w": 2.5e-6, "t_half": 12.0})],
     level="expert")


# ================================================================================================ CELL GROWTH AND DIVISION
def exponential_growth(N0, td, t):
    _pos("N0 and the doubling time td", N0, td)
    _nonneg("the time t", t)
    n = t / td
    N = N0 * 2.0 ** n
    mu = LN2 / td
    return _out(N=N, generations=n, fold=N / N0, mu=mu, mu_per_h=mu * HOUR)


def _ref_exponential_growth(N0, td, t):
    sc = _sc()
    mu = math.log(2.0) / td
    N = _ivp(lambda tt, y: [mu * y[0]], t, [N0])[0] if t > 0 else N0
    return {"N": N, "generations": math.log2(N / N0), "fold": N / N0, "mu": mu, "mu_per_h": mu * sc.hour}


_add("cmb_exponential_growth",
     name="Exponential growth of a cell population", topic="cell growth / exponential growth",
     purpose="The number of cells after a time t of exponential growth from N0 with doubling time td, the number of generations and the specific growth rate.",
     inputs=[_numin("N0", "initial number (or concentration) of cells"),
             _phys("td", "time", "doubling (generation) time", show=_HOURS), _phys("t", "time", "elapsed time", show=_HOURS)],
     outputs=[_o("N", "number", "number of cells at time t"), _o("generations", "number", "number of generations t/td"),
              _o("fold", "number", "fold increase N/N0"), _o("mu", "first_order_rate_constant", "specific growth rate ln2/td"),
              _o("mu_per_h", "number", "specific growth rate per hour")],
     primary="N", conditions=["N0, td > 0", "t >= 0", "balanced exponential growth (no lag, no limitation)"],
     method="N = N0 2^(t/td); mu = ln 2 / td", latex=r"N = N_0\,2^{t/t_d} = N_0 e^{\mu t},\quad \mu = \frac{\ln 2}{t_d}",
     run=exponential_growth, reference=_ref_exponential_growth,
     wrong=lambda N0, td, t: _out(N=N0 * math.exp(t / td), generations=t / td, fold=math.exp(t / td), mu=1.0 / td, mu_per_h=HOUR / td),
     rand=lambda rng: {"N0": _sig(_lu(rng, 10.0, 1e7), 3), "td": _sig(_lu(rng, 600.0, 2e5), 3), "t": _sig(_lu(rng, 600.0, 4e5), 3)},
     phrases=("Exponential cell growth", "Exponential growth of cells", "Cell population growth"),
     examples=[_ex({"N0": 1000.0, "td": 1200.0, "t": 7200.0}), _ex({"N0": 2e5, "td": 86400.0, "t": 259200.0})],
     level="beginner",
     extra_q=[{"q": "Exponential cell growth: N0 = 1000, td = 20 min, t = 2 h", "like": 0}])


def doubling_from_counts(N0, N, t):
    _pos("N0, N and t", N0, N, t)
    _need(N > N0, "the final count must exceed the initial count")
    n = math.log2(N / N0)
    td = t / n
    mu = LN2 / td
    return _out(td=td, td_h=td / HOUR, generations=n, mu=mu, mu_per_h=mu * HOUR)


def _ref_doubling(N0, N, t):
    td = _brentq(lambda d: math.log(N0) + t / d * math.log(2.0) - math.log(N), t / (math.log2(N / N0) * 1e3), t / math.log2(N / N0) * 1e3)
    return {"td": td, "td_h": td / 3600.0, "generations": t / td, "mu": math.log(2.0) / td, "mu_per_h": math.log(2.0) / td * 3600.0}


_add("cmb_doubling_time_from_counts",
     name="Doubling time and number of generations from two counts", topic="cell growth / doubling time",
     purpose="The doubling (generation) time, the number of generations and the specific growth rate from the cell counts at the start and end of an interval of exponential growth.",
     inputs=[_numin("N0", "initial cell count (or concentration)"), _numin("N", "final cell count (same unit)"),
             _phys("t", "time", "elapsed time", show=_HOURS)],
     outputs=[_o("td", "time", "doubling time"), _o("td_h", "number", "doubling time in hours"),
              _o("generations", "number", "number of generations log2(N/N0)"),
              _o("mu", "first_order_rate_constant", "specific growth rate"), _o("mu_per_h", "number", "specific growth rate per hour")],
     primary="td", conditions=["N > N0 > 0", "t > 0", "exponential growth over the interval"],
     method="n = log2(N/N0) = 3.32 log10(N/N0); td = t/n; mu = ln 2 / td",
     latex=r"n = \log_2\frac{N}{N_0},\quad t_d = \frac{t}{n},\quad \mu = \frac{\ln(N/N_0)}{t}",
     run=doubling_from_counts, reference=_ref_doubling,
     wrong=lambda N0, N, t: (lambda td: _out(td=td, td_h=td / HOUR, generations=t / td, mu=LN2 / td, mu_per_h=LN2 / td * HOUR))(t / math.log(N / N0)),
     rand=lambda rng: (lambda n0: {"N0": n0, "N": _sig(n0 * _lu(rng, 1.5, 1e5), 4), "t": _sig(_lu(rng, 1800.0, 5e5), 3)})(_sig(_lu(rng, 10.0, 1e7), 3)),
     phrases=("Doubling time from cell counts", "Generation time from counts", "Doubling time"),
     examples=[_ex({"N0": 1e4, "N": 1.6e5, "t": 28800.0}), _ex({"N0": 5e5, "N": 2e6, "t": 172800.0})],
     level="beginner",
     extra_q=[{"q": "Doubling time: N0 = 10000, N = 160000, t = 480 min", "like": 0},
              {"q": "Doubling time: N0 = 10000, N = 5000, t = 8 h", "outcome": "INVALID_INPUT"}])


def growth_rate_fit(t, N):
    _need(len(t) == len(N) and len(t) >= 3, "give at least 3 times and one count per time")
    for v in N:
        _need(_isnum(v) and v > 0, "counts must be positive")
    for v in t:
        _fin(v)
    y = [math.log(v) for v in N]
    b, a, r2 = _linfit(t, y)
    _need(b > 0, "the counts do not increase: no growth rate can be fitted")
    return _out(mu_per_h=b, td_h=LN2 / b, N0_fit=math.exp(a), r2=r2)


def _ref_growth_fit(t, N):
    import numpy as np
    s, i, r2 = _linregress(t, np.log(N))
    return {"mu_per_h": s, "td_h": math.log(2.0) / s, "N0_fit": math.exp(i), "r2": r2}


def _rand_growth_fit(rng):
    n = rng.randint(4, 9)
    mu = rng.uniform(0.1, 1.5)
    n0 = _lu(rng, 100.0, 1e6)
    ts = sorted(set(round(rng.uniform(0.0, 12.0), 1) for _ in range(n + 4)))[:n]
    return {"t": ts, "N": [_sig(n0 * math.exp(mu * x) * math.exp(rng.gauss(0.0, 0.05)), 4) for x in ts]}


_add("cmb_growth_rate_fit",
     name="Specific growth rate fitted to growth-curve data", topic="cell growth / growth-curve fit",
     purpose="The specific growth rate, doubling time and back-extrapolated initial count from counts (or OD) measured during exponential growth, by a least-squares line through ln N against time.",
     inputs=[_vecin("t", "sampling times in hours", 3, 100), _vecin("N", "cell counts or optical densities (positive)", 3, 100)],
     outputs=[_o("mu_per_h", "number", "specific growth rate per hour"), _o("td_h", "number", "doubling time in hours"),
              _o("N0_fit", "number", "fitted count at t = 0"), _o("r2", "number", "coefficient of determination of the log-linear fit")],
     primary="mu_per_h", conditions=["at least 3 points", "counts > 0, increasing", "exponential phase only"],
     method="least-squares line ln N = ln N0 + mu t; td = ln 2 / mu",
     latex=r"\ln N = \ln N_0 + \mu t,\quad t_d = \frac{\ln 2}{\mu}",
     run=growth_rate_fit, reference=_ref_growth_fit,
     wrong=lambda t, N: (lambda f: _out(mu_per_h=f[0], td_h=LN2 / f[0], N0_fit=10.0 ** f[1], r2=f[2]))(_linfit(t, [math.log10(v) for v in N])),
     rand=_rand_growth_fit,
     phrases=("Growth rate fit", "Specific growth rate fit", "Fit the growth rate"),
     examples=[_ex({"t": [0.0, 1.0, 2.0, 3.0, 4.0], "N": [1000.0, 1980.0, 4100.0, 7900.0, 16200.0]}),
               _ex({"t": [0.0, 2.0, 4.0, 6.0], "N": [0.05, 0.11, 0.21, 0.4]})],
     level="intermediate")


def logistic_growth(N0, K, r, t):
    _pos("N0, K and r", N0, K, r)
    _nonneg("the time t", t)
    _need(N0 < K, "the initial population must be below the carrying capacity")
    a = (K - N0) / N0
    ae = a * math.exp(-r * t)
    N = K / (1.0 + ae)
    gap = ae / (1.0 + ae)                         # 1 - N/K without cancellation near saturation
    return _out(N=N, dNdt=r * N * gap, dNdt_per_h=r * N * gap * HOUR,
                t_inflection=math.log(a) / r, t_inflection_h=math.log(a) / r / HOUR)


def _ref_logistic(N0, K, r, t):
    # integrate the gap u = K - N (du/dt = -r u (K - u)/K) so that 1 - N/K keeps its relative accuracy
    u = _ivp(lambda tt, y: [-r * y[0] * (K - y[0]) / K], t, [K - N0])[0] if t > 0 else K - N0
    N = K - u
    if N0 < K / 2.0:
        ti = _brentq(lambda s: K / (1.0 + (K - N0) / N0 * math.exp(-r * s)) - K / 2.0, 0.0, 50.0 / r)
    else:
        ti = -_brentq(lambda s: K / (1.0 + (K - N0) / N0 * math.exp(r * s)) - K / 2.0, 0.0, 50.0 / r)
    return {"N": N, "dNdt": r * N * u / K, "dNdt_per_h": r * N * u / K * 3600.0,
            "t_inflection": ti, "t_inflection_h": ti / 3600.0}


def _rand_logistic(rng):
    K = _sig(_lu(rng, 1e4, 1e10), 3)
    r = _sig(_lu(rng, 1e-5, 1e-3), 3)
    return {"N0": _sig(K * _lu(rng, 1e-5, 0.8), 3), "K": K, "r": r, "t": _sig(rng.uniform(0.0, 15.0) / r, 3)}


_add("cmb_logistic_growth",
     name="Logistic growth of a population", topic="cell growth / logistic growth",
     purpose="The population at time t under logistic growth with carrying capacity K and intrinsic rate r, the growth rate dN/dt there and the time of the inflection point (N = K/2).",
     inputs=[_numin("N0", "initial population"), _numin("K", "carrying capacity (same unit)"),
             _phys("r", "first_order_rate_constant", "intrinsic growth rate"), _phys("t", "time", "elapsed time", show=_HOURS)],
     outputs=[_o("N", "number", "population at time t"), _o("dNdt", "number", "growth rate at time t in cells per second"),
              _o("dNdt_per_h", "number", "growth rate at time t in cells per hour"),
              _o("t_inflection", "time", "time when N = K/2 (negative if already past)"),
              _o("t_inflection_h", "number", "time of the inflection point in hours")],
     primary="N", conditions=["0 < N0 < K", "r > 0", "t >= 0"],
     method="N = K / (1 + ((K - N0)/N0) e^(-r t)); dN/dt = r N (1 - N/K); t_infl = ln((K - N0)/N0)/r",
     latex=r"N(t) = \frac{K}{1 + \frac{K - N_0}{N_0}e^{-rt}},\quad \frac{dN}{dt} = rN\left(1 - \frac{N}{K}\right)",
     run=logistic_growth, reference=_ref_logistic,
     wrong=lambda N0, K, r, t: (lambda N: _out(N=N, dNdt=r * N * (1.0 - N / K), dNdt_per_h=r * N * (1.0 - N / K) * HOUR,
                                               t_inflection=math.log(K / N0) / r, t_inflection_h=math.log(K / N0) / r / HOUR))(K / (1.0 + (K / N0) * math.exp(-r * t))),
     rand=_rand_logistic,
     phrases=("Logistic growth", "Logistic population growth"),
     examples=[_ex({"N0": 1e4, "K": 1e8, "r": 0.0002, "t": 36000.0}), _ex({"N0": 5e7, "K": 1e9, "r": 1e-4, "t": 18000.0})],
     level="intermediate",
     extra_q=[{"q": "Logistic growth: N0 = 10000, K = 100000000, r = 0.72 1/h, t = 600 min", "like": 0}])


def logistic_time_to_reach(N0, K, r, N):
    _pos("N0, K, r and N", N0, K, r, N)
    _need(N0 < N < K, "the target must lie between the initial population and the carrying capacity")
    t = math.log(N * (K - N0) / (N0 * (K - N))) / r
    return _out(t=t, t_h=t / HOUR)


def _ref_logistic_time(N0, K, r, N):
    t = _quad(lambda x: 1.0 / (r * x * (1.0 - x / K)), N0, N)
    return {"t": t, "t_h": t / 3600.0}


def _rand_logistic_time(rng):
    K = _sig(_lu(rng, 1e4, 1e10), 3)
    n0 = _sig(K * _lu(rng, 1e-5, 0.5), 3)
    return {"N0": n0, "K": K, "r": _sig(_lu(rng, 1e-5, 1e-3), 3), "N": _sig(n0 + (K - n0) * rng.uniform(0.05, 0.95), 3)}


_add("cmb_logistic_time_to_reach",
     name="Time for a logistic population to reach a size", topic="cell growth / logistic growth",
     purpose="The time for a population growing logistically from N0 (carrying capacity K, rate r) to reach N.",
     inputs=[_numin("N0", "initial population"), _numin("K", "carrying capacity"),
             _phys("r", "first_order_rate_constant", "intrinsic growth rate"), _numin("N", "target population")],
     outputs=[_o("t", "time", "time to reach N"), _o("t_h", "number", "time in hours")],
     primary="t", conditions=["0 < N0 < N < K", "r > 0"],
     method="t = (1/r) ln[ N (K - N0) / (N0 (K - N)) ]",
     latex=r"t = \frac{1}{r}\ln\frac{N (K - N_0)}{N_0 (K - N)}",
     run=logistic_time_to_reach, reference=_ref_logistic_time,
     wrong=lambda N0, K, r, N: _out(t=math.log(N / N0) / r, t_h=math.log(N / N0) / r / HOUR),
     rand=_rand_logistic_time,
     phrases=("Logistic time to reach", "Time to reach a population size"),
     examples=[_ex({"N0": 1e4, "K": 1e8, "r": 0.0002, "N": 5e7}), _ex({"N0": 100.0, "K": 1e6, "r": 5e-5, "N": 9e5})],
     level="intermediate",
     extra_q=[{"q": "Logistic time to reach: N0 = 10000, K = 100000000, r = 0.72 1/h, N = 200000000", "outcome": "INVALID_INPUT"}])


def monod_growth(mu_max, S, Ks):
    _pos("mu_max and Ks", mu_max, Ks)
    _nonneg("the substrate concentration S", S)
    mu = mu_max * S / (Ks + S)
    out = {"mu": mu, "mu_per_h": mu * HOUR, "fraction": S / (Ks + S)}
    out["td_h"] = LN2 / mu / HOUR if mu > 0 else 0.0
    return _out(**out)


def _ref_monod(mu_max, S, Ks):
    # chemostat steady state: at dilution rate D the residual substrate is Ks D/(mu_max - D); find D giving S
    if S == 0:
        return {"mu": 0.0, "mu_per_h": 0.0, "fraction": 0.0, "td_h": 0.0}
    D = _brentq(lambda d: Ks * d / (mu_max - d) - S, 0.0, mu_max * (1.0 - 1e-15))
    return {"mu": D, "mu_per_h": D * 3600.0, "fraction": D / mu_max, "td_h": math.log(2.0) / D / 3600.0}


_add("cmb_monod_growth_rate",
     name="Monod specific growth rate", topic="cell growth / Monod kinetics",
     purpose="The specific growth rate of a microbial culture limited by one substrate (Monod), its fraction of the maximum and the corresponding doubling time.",
     inputs=[_phys("mu_max", "first_order_rate_constant", "maximum specific growth rate"),
             _phys("S", "mass_concentration", "limiting substrate concentration", show=("mg/L", 1e3)),
             _phys("Ks", "mass_concentration", "half-saturation constant", show=("mg/L", 1e3))],
     outputs=[_o("mu", "first_order_rate_constant", "specific growth rate"), _o("mu_per_h", "number", "growth rate per hour"),
              _o("fraction", "number", "mu / mu_max"), _o("td_h", "number", "doubling time ln2/mu in hours (0 when mu = 0)")],
     primary="mu", conditions=["mu_max, Ks > 0", "S >= 0", "a single limiting substrate"],
     method="mu = mu_max S / (Ks + S)", latex=r"\mu = \mu_{max}\frac{S}{K_s + S}",
     run=monod_growth, reference=_ref_monod,
     wrong=lambda mu_max, S, Ks: (lambda mu: _out(mu=mu, mu_per_h=mu * HOUR, fraction=S / Ks, td_h=LN2 / mu / HOUR if mu > 0 else 0.0))(mu_max * S / Ks),
     rand=lambda rng: {"mu_max": _sig(_lu(rng, 2e-5, 5e-4), 3), "S": _sig(_lu(rng, 1e-4, 10.0), 3), "Ks": _sig(_lu(rng, 1e-4, 1.0), 3)},
     phrases=("Monod growth rate", "Monod specific growth rate"),
     examples=[_ex({"mu_max": 0.0002, "S": 0.05, "Ks": 0.01}), _ex({"mu_max": 1e-4, "S": 0.002, "Ks": 0.006})],
     level="beginner",
     extra_q=[{"q": "Monod growth rate: mu_max = 0.72 1/h, S = 50 mg/L, Ks = 10 mg/L", "like": 0}],
     weak="the inverse chemostat steady state (residual substrate Ks D/(mu_max - D)) solved by brentq")


def monod_batch_time(X0, S0, Y, mu_max, Ks, X):
    _pos("X0, S0, Y, mu_max and Ks", X0, S0, Y, mu_max, Ks)
    B = X0 + Y * S0
    _need(X0 < X < B, "the target biomass must lie between X0 and X0 + Y S0")
    S = (B - X) / Y
    t = (math.log(X / X0) + Ks * Y / B * (math.log(X / X0) - math.log((B - X) / (B - X0)))) / mu_max
    return _out(t=t, t_h=t / HOUR, S=S, S_g_L=S)


def _ref_monod_batch(X0, S0, Y, mu_max, Ks, X):
    B = X0 + Y * S0
    t = _quad(lambda x: (Ks + (B - x) / Y) / (mu_max * ((B - x) / Y) * x), X0, X)
    return {"t": t, "t_h": t / 3600.0, "S": (B - X) / Y, "S_g_L": (B - X) / Y}


def _rand_monod_batch(rng):
    X0 = _sig(_lu(rng, 0.01, 1.0), 3)
    Y = _sig(rng.uniform(0.2, 0.7), 3)
    S0 = _sig(_lu(rng, 1.0, 50.0), 3)
    return {"X0": X0, "S0": S0, "Y": Y, "mu_max": _sig(_lu(rng, 5e-5, 5e-4), 3), "Ks": _sig(_lu(rng, 0.01, 2.0), 3),
            "X": _sig(X0 + Y * S0 * rng.uniform(0.05, 0.95), 3)}


_add("cmb_monod_batch_time",
     name="Batch culture time under Monod kinetics", topic="cell growth / Monod kinetics",
     purpose="The time for a batch culture (constant yield Y, Monod growth) to grow from biomass X0 to X, and the substrate left then.",
     inputs=[_phys("X0", "mass_concentration", "initial biomass concentration", show=("g/L", 1.0)),
             _phys("S0", "mass_concentration", "initial substrate concentration", show=("g/L", 1.0)),
             _numin("Y", "biomass yield (g biomass per g substrate)"),
             _phys("mu_max", "first_order_rate_constant", "maximum specific growth rate"),
             _phys("Ks", "mass_concentration", "half-saturation constant", show=("g/L", 1.0)),
             _phys("X", "mass_concentration", "target biomass concentration", show=("g/L", 1.0))],
     outputs=[_o("t", "time", "time to reach X"), _o("t_h", "number", "time in hours"),
              _o("S", "mass_concentration", "substrate remaining"), _o("S_g_L", "number", "substrate remaining in g/L")],
     primary="t", conditions=["all inputs > 0", "X0 < X < X0 + Y S0", "no maintenance or death"],
     method="S = S0 - (X - X0)/Y; integrate dX/dt = mu_max S X/(Ks + S): t = [ln(X/X0) + (Ks Y/B)(ln(X/X0) - ln((B - X)/(B - X0)))]/mu_max, B = X0 + Y S0",
     latex=r"t = \frac{1}{\mu_{max}}\left[\left(1 + \frac{K_sY}{B}\right)\ln\frac{X}{X_0} - \frac{K_sY}{B}\ln\frac{B - X}{B - X_0}\right]",
     run=monod_batch_time, reference=_ref_monod_batch,
     wrong=lambda X0, S0, Y, mu_max, Ks, X: _out(t=math.log(X / X0) / mu_max, t_h=math.log(X / X0) / mu_max / HOUR,
                                                  S=S0 - (X - X0) / Y, S_g_L=S0 - (X - X0) / Y),
     rand=_rand_monod_batch,
     phrases=("Monod batch growth time", "Batch culture time"),
     examples=[_ex({"X0": 0.1, "S0": 10.0, "Y": 0.5, "mu_max": 0.0002, "Ks": 0.2, "X": 4.0}),
               _ex({"X0": 0.05, "S0": 5.0, "Y": 0.4, "mu_max": 1e-4, "Ks": 1.0, "X": 1.0})],
     level="expert",
     extra_q=[{"q": "Batch culture time: X0 = 0.1 g/L, S0 = 10 g/L, Y = 0.5, mu_max = 0.72 1/h, Ks = 200 mg/L, X = 4 g/L", "like": 0}])


def _phase_check(fractions):
    _need(len(fractions) == 4, "give four fractions: G1, S, G2, M")
    s = 0.0
    for f in fractions:
        _need(_isnum(f) and f > 0, "every phase fraction must be positive")
        s += f
    _need(abs(s - 1.0) <= 1e-6, "the four fractions must add up to 1")


def cell_cycle_simple(Tc, fractions):
    _pos("the cycle time Tc", Tc)
    _phase_check(fractions)
    d = [f * Tc for f in fractions]
    return _out(G1_h=d[0] / HOUR, S_h=d[1] / HOUR, G2_h=d[2] / HOUR, M_h=d[3] / HOUR)


def _ref_cycle_simple(Tc, fractions):
    import numpy as np
    d = np.array(fractions, dtype=float) * Tc / 3600.0
    return {"G1_h": float(d[0]), "S_h": float(d[1]), "G2_h": float(d[2]), "M_h": float(d[3])}


def _rand_fractions(rng):
    w = [rng.uniform(0.3, 1.0), rng.uniform(0.2, 0.6), rng.uniform(0.1, 0.3), rng.uniform(0.03, 0.12)]
    s = sum(w)
    f = [round(x / s, 3) for x in w[:3]]
    f.append(round(1.0 - f[0] - f[1] - f[2], 3))
    return f


_add("cmb_cell_cycle_phase_durations",
     name="Cell-cycle phase durations from phase fractions", topic="cell growth / cell cycle",
     purpose="The durations of G1, S, G2 and M from the fraction of an asynchronous population in each phase (simple proportion to the cycle time).",
     inputs=[_phys("Tc", "time", "total cell-cycle time", show=_HOURS),
             _vecin("fractions", "fractions of cells in G1, S, G2 and M (in this order, summing to 1)", 4, 4)],
     outputs=[_o("G1_h", "number", "G1 duration in hours"), _o("S_h", "number", "S duration in hours"),
              _o("G2_h", "number", "G2 duration in hours"), _o("M_h", "number", "M duration in hours")],
     primary="G1_h", conditions=["Tc > 0", "four positive fractions summing to 1"],
     method="duration of a phase = fraction x Tc", latex=r"t_{phase} = f_{phase}\,T_c",
     run=cell_cycle_simple, reference=_ref_cycle_simple,
     wrong=lambda Tc, fractions: _out(G1_h=fractions[0] * Tc * LN2 / HOUR, S_h=fractions[1] * Tc * LN2 / HOUR,
                                      G2_h=fractions[2] * Tc * LN2 / HOUR, M_h=fractions[3] * Tc * LN2 / HOUR),
     rand=lambda rng: {"Tc": _sig(_lu(rng, 3600.0 * 8, 3600.0 * 48), 3), "fractions": _rand_fractions(rng)},
     phrases=("Cell-cycle phase durations", "Cell cycle phase durations"),
     examples=[_ex({"Tc": 86400.0, "fractions": [0.45, 0.35, 0.15, 0.05]}), _ex({"Tc": 72000.0, "fractions": [0.5, 0.3, 0.16, 0.04]})],
     level="beginner",
     weak="numpy elementwise product of the same proportions")


def cell_cycle_age_corrected(Tc, fractions):
    _pos("the cycle time Tc", Tc)
    _phase_check(fractions)
    ends, F = [], 0.0
    for k in range(4):
        F += fractions[k]
        if k == 3:
            ends.append(Tc)
        else:
            ends.append(-Tc * math.log2(1.0 - F / 2.0))
    d = [ends[0], ends[1] - ends[0], ends[2] - ends[1], ends[3] - ends[2]]
    return _out(G1_h=d[0] / HOUR, S_h=d[1] / HOUR, G2_h=d[2] / HOUR, M_h=d[3] / HOUR)


def _ref_cycle_age(Tc, fractions):
    dens = lambda a: 2.0 * math.log(2.0) / Tc * 2.0 ** (-a / Tc)
    ends, F = [], 0.0
    for k in range(3):
        F += fractions[k]
        ends.append(_brentq(lambda a: _quad(dens, 0.0, a) - F, 0.0, Tc))
    ends.append(Tc)
    d = [ends[0], ends[1] - ends[0], ends[2] - ends[1], ends[3] - ends[2]]
    return {"G1_h": d[0] / 3600.0, "S_h": d[1] / 3600.0, "G2_h": d[2] / 3600.0, "M_h": d[3] / 3600.0}


_add("cmb_cell_cycle_age_corrected",
     name="Cell-cycle phase durations corrected for the exponential age distribution", topic="cell growth / cell cycle",
     purpose="The durations of G1, S, G2 and M from phase fractions of an exponentially growing asynchronous population, using the age distribution n(a) = 2 ln2/T 2^(-a/T) (young cells are over-represented).",
     inputs=[_phys("Tc", "time", "total cell-cycle (doubling) time", show=_HOURS),
             _vecin("fractions", "fractions of cells in G1, S, G2 and M (in this order, summing to 1)", 4, 4)],
     outputs=[_o("G1_h", "number", "G1 duration in hours"), _o("S_h", "number", "S duration in hours"),
              _o("G2_h", "number", "G2 duration in hours"), _o("M_h", "number", "M duration in hours")],
     primary="G1_h", conditions=["Tc > 0", "four positive fractions summing to 1", "balanced exponential growth"],
     method="fraction younger than age a: F(a) = 2(1 - 2^(-a/T)); phase end ages a_k = -T log2(1 - F_k/2) from cumulative fractions",
     latex=r"F(a) = 2\left(1 - 2^{-a/T}\right),\quad a_k = -T\log_2\!\left(1 - \frac{F_k}{2}\right)",
     run=cell_cycle_age_corrected, reference=_ref_cycle_age,
     wrong=lambda Tc, fractions: cell_cycle_simple(Tc, fractions),
     rand=lambda rng: {"Tc": _sig(_lu(rng, 3600.0 * 8, 3600.0 * 48), 3), "fractions": _rand_fractions(rng)},
     phrases=("Age-corrected cell-cycle phase durations", "Cell-cycle phase durations from an exponential population"),
     examples=[_ex({"Tc": 86400.0, "fractions": [0.45, 0.35, 0.15, 0.05]}), _ex({"Tc": 72000.0, "fractions": [0.5, 0.3, 0.16, 0.04]})],
     level="advanced")


def hemocytometer(counts, dilution, V=None):
    _need(len(counts) >= 1, "give the counts of at least one square")
    for v in counts:
        _need(_isnum(v) and v >= 0, "counts must be zero or positive")
    _need(_isnum(dilution) and dilution >= 1, "the dilution factor must be at least 1")
    m = _mean(counts)
    conc = m * dilution * 1e4
    out = {"mean_count": m, "cells_per_mL": conc}
    if V is not None:
        _pos("the suspension volume V", V)
        out["total_cells"] = conc * V * 1e6
    return _out(**out)


def _ref_hemocytometer(counts, dilution, V=None):
    import numpy as np
    m = float(np.mean(counts))
    square_volume_mL = 1e-3 * 1e-3 * 1e-4 * 1e6     # 1 mm x 1 mm x 0.1 mm in m^3, then mL
    conc = m * dilution / square_volume_mL
    out = {"mean_count": m, "cells_per_mL": conc}
    if V is not None:
        out["total_cells"] = conc * V * 1e6
    return out


_add("cmb_hemocytometer_count",
     name="Cell concentration from a hemocytometer count", topic="cell growth / cell counting",
     purpose="The cell concentration from counts in the large (1 mm x 1 mm x 0.1 mm) squares of a hemocytometer, the dilution factor and, optionally, the total cells in a suspension volume.",
     inputs=[_vecin("counts", "cells counted in each large square", 1, 25),
             _numin("dilution", "dilution factor of the counted sample (1 for undiluted, 2 for 1:1 trypan blue)"),
             _phys("V", "volume", "volume of the cell suspension (optional)", required=False)],
     outputs=[_o("mean_count", "number", "mean count per large square"), _o("cells_per_mL", "number", "cells per mL"),
              _o("total_cells", "number", "total cells in V (when V is given)")],
     primary="cells_per_mL", conditions=["counts >= 0", "dilution >= 1", "large squares of 0.1 uL"],
     method="cells/mL = mean count x dilution x 10^4", latex=r"c = \bar n \times d \times 10^4\ \mathrm{mL^{-1}}",
     run=hemocytometer, reference=_ref_hemocytometer,
     wrong=lambda counts, dilution, V=None: (lambda m: _out(mean_count=m, cells_per_mL=m * dilution * 1e3,
                                                            **({"total_cells": m * dilution * 1e3 * V * 1e6} if V is not None else {})))(_mean(counts)),
     rand=lambda rng: dict({"counts": [float(rng.randint(10, 200)) for _ in range(rng.randint(1, 9))], "dilution": rng.choice([1.0, 2.0, 5.0, 10.0])},
                           **({"V": _sig(rng.uniform(1e-6, 5e-5), 3)} if rng.random() < 0.5 else {})),
     phrases=("Hemocytometer cell count", "Cell concentration from a hemocytometer"),
     examples=[_ex({"counts": [52.0, 48.0, 55.0, 45.0], "dilution": 2.0}), _ex({"counts": [120.0, 110.0], "dilution": 10.0, "V": 1e-5})],
     level="beginner",
     weak="numpy mean and the chamber volume written out in SI")


def cfu_count(colonies, dilution, V):
    _need(isinstance(colonies, int) and colonies >= 0, "the colony count must be a whole number")
    _need(_isnum(dilution) and dilution >= 1, "the dilution factor must be at least 1 (e.g. 1e6 for a 10^-6 dilution)")
    _pos("the plated volume V", V)
    c = colonies * dilution / (V * 1e6)
    return _out(cfu_per_mL=c, log10_cfu_per_mL=math.log10(c) if c > 0 else 0.0)


def _ref_cfu(colonies, dilution, V):
    from fractions import Fraction as Fr
    c = float(Fr(colonies) * Fr(dilution) / (Fr(V) * 10 ** 6))
    return {"cfu_per_mL": c, "log10_cfu_per_mL": math.log10(c) if c > 0 else 0.0}


_add("cmb_plate_count",
     name="Colony-forming units per mL from a plate count", topic="cell growth / cell counting",
     purpose="The viable count (CFU per mL) of the original culture from the colonies on a plate, the dilution factor of the plated sample and the plated volume.",
     inputs=[_intin("colonies", "colonies counted on the plate", lo=0),
             _numin("dilution", "dilution factor of the plated sample (1e6 for a 10^-6 dilution)"),
             _phys("V", "volume", "volume plated")],
     outputs=[_o("cfu_per_mL", "number", "colony-forming units per mL of the original culture"),
              _o("log10_cfu_per_mL", "number", "log10 of CFU per mL (0 when no colonies)")],
     primary="cfu_per_mL", conditions=["colonies a whole number", "dilution >= 1", "V > 0"],
     method="CFU/mL = colonies x dilution factor / volume plated (mL)", latex=r"\mathrm{CFU/mL} = \frac{n \times DF}{V_{plated}}",
     run=cfu_count, reference=_ref_cfu,
     wrong=lambda colonies, dilution, V: (lambda c: _out(cfu_per_mL=c, log10_cfu_per_mL=math.log10(c) if c > 0 else 0.0))(colonies * dilution * V * 1e6),
     rand=lambda rng: {"colonies": rng.randint(30, 300), "dilution": rng.choice([1e3, 1e4, 1e5, 1e6, 1e7]), "V": rng.choice([1e-7, 5e-8, 1e-6, 2e-7])},
     phrases=("Plate count", "Colony-forming units per mL", "CFU per mL"),
     examples=[_ex({"colonies": 150, "dilution": 1e6, "V": 1e-7}), _ex({"colonies": 42, "dilution": 1e4, "V": 1e-6})],
     level="beginner",
     extra_q=[{"q": "CFU per mL: colonies = 150, dilution = 1e6, V = 100 uL", "like": 0}],
     weak="the same definition in exact rational arithmetic")


def transformation_efficiency(colonies, m_DNA, V_plated, V_total):
    _need(isinstance(colonies, int) and colonies >= 0, "the colony count must be a whole number")
    _pos("the DNA mass and the volumes", m_DNA, V_plated, V_total)
    _need(V_plated <= V_total, "the plated volume cannot exceed the total volume")
    ug_plated = m_DNA * 1e9 * V_plated / V_total
    eff = colonies / ug_plated
    return _out(efficiency=eff, log10_efficiency=math.log10(eff) if eff > 0 else 0.0, DNA_plated_ug=ug_plated)


def _ref_transformation(colonies, m_DNA, V_plated, V_total):
    from fractions import Fraction as Fr
    frac = Fr(V_plated) / Fr(V_total)
    ug = Fr(m_DNA) * 10 ** 9 * frac
    eff = float(Fr(colonies) / ug)
    return {"efficiency": eff, "log10_efficiency": math.log10(eff) if eff > 0 else 0.0, "DNA_plated_ug": float(ug)}


_add("cmb_transformation_efficiency",
     name="Transformation efficiency", topic="cell growth / cell counting",
     purpose="Transformants per microgram of DNA from the colonies on a plate, the DNA mass used and the fraction of the transformation plated.",
     inputs=[_intin("colonies", "transformant colonies counted", lo=0), _phys("m_DNA", "mass", "DNA mass used in the transformation", show=_NG),
             _phys("V_plated", "volume", "volume plated", show=_UL), _phys("V_total", "volume", "total volume after recovery", show=_UL)],
     outputs=[_o("efficiency", "number", "transformants per microgram of DNA"),
              _o("log10_efficiency", "number", "log10 of the efficiency"), _o("DNA_plated_ug", "number", "micrograms of DNA plated")],
     primary="efficiency", conditions=["colonies a whole number", "m_DNA, volumes > 0", "V_plated <= V_total"],
     method="efficiency = colonies / (ug DNA x V_plated / V_total)",
     latex=r"\eta = \frac{n}{m_{DNA}\,(V_{plated}/V_{total})}",
     run=transformation_efficiency, reference=_ref_transformation,
     wrong=lambda colonies, m_DNA, V_plated, V_total: (lambda e: _out(efficiency=e, log10_efficiency=math.log10(e) if e > 0 else 0.0, DNA_plated_ug=m_DNA * 1e9))(colonies / (m_DNA * 1e9)),
     rand=lambda rng: {"colonies": rng.randint(20, 500), "m_DNA": _sig(_lu(rng, 1e-13, 1e-10), 3),
                       "V_plated": rng.choice([5e-8, 1e-7, 2e-7]), "V_total": rng.choice([5e-7, 1e-6, 1.1e-6])},
     phrases=("Transformation efficiency",),
     examples=[_ex({"colonies": 200, "m_DNA": 1e-12, "V_plated": 1e-7, "V_total": 1e-6}),
               _ex({"colonies": 85, "m_DNA": 1e-11, "V_plated": 5e-8, "V_total": 1.05e-6})],
     level="beginner",
     weak="the same definition in exact rational arithmetic")


def cell_geometry(d):
    _pos("the diameter d", d)
    r = d / 2.0
    V = 4.0 / 3.0 * math.pi * r ** 3
    A = 4.0 * math.pi * r * r
    return _out(V=V, V_fL=V * 1e18, A=A, A_um2=A * 1e12, SA_V_per_um=A / V * 1e-6)


def _ref_cell_geometry(d):
    import sympy as sp
    rr = sp.Symbol("rr", positive=True)
    R = d / 2.0
    V = _quad(lambda x: 4.0 * math.pi * x * x, 0.0, R)
    A = float(sp.diff(sp.Rational(4, 3) * sp.pi * rr ** 3, rr).subs(rr, R))
    return {"V": V, "V_fL": V * 1e18, "A": A, "A_um2": A * 1e12, "SA_V_per_um": A / V * 1e-6}


_add("cmb_cell_geometry",
     name="Volume, surface area and surface-to-volume ratio of a spherical cell", topic="cell growth / cell size",
     purpose="The volume (fL), surface area and surface-to-volume ratio of a spherical cell of a given diameter.",
     inputs=[_phys("d", "length", "cell diameter", show=_UM)],
     outputs=[_o("V", "volume", "cell volume"), _o("V_fL", "number", "volume in femtolitres (um^3)"),
              _o("A", "area", "surface area"), _o("A_um2", "number", "surface area in um^2"),
              _o("SA_V_per_um", "number", "surface-to-volume ratio in 1/um")],
     primary="V", conditions=["d > 0", "a sphere"],
     method="V = (4/3) pi r^3, A = 4 pi r^2, A/V = 3/r", latex=r"V = \tfrac{4}{3}\pi r^3,\quad A = 4\pi r^2,\quad \frac{A}{V} = \frac{3}{r}",
     run=cell_geometry, reference=_ref_cell_geometry,
     wrong=lambda d: (lambda r: _out(V=4.0 / 3.0 * math.pi * r ** 3, V_fL=4.0 / 3.0 * math.pi * r ** 3 * 1e18, A=4.0 * math.pi * r * r,
                                     A_um2=4.0 * math.pi * r * r * 1e12, SA_V_per_um=3.0 / r * 1e-6))(d),
     rand=lambda rng: {"d": _sig(_lu(rng, 5e-7, 1e-4), 3)},
     phrases=("Spherical cell volume and surface area", "Cell volume and surface area"),
     examples=[_ex({"d": 1e-5}), _ex({"d": 1e-6})],
     level="beginner",
     weak="scipy quad of 4 pi r^2 for the volume and a sympy derivative for the area")


def molecules_per_cell(c, V):
    _pos("the concentration c and the volume V", c, V)
    N = c * V * N_A
    one = 1.0 / (V * N_A)
    return _out(N=N, one_molecule=one, one_molecule_nM=one * 1e6)


def _ref_molecules(c, V):
    sc = _sc()
    n_mol = c * V
    return {"N": n_mol * sc.Avogadro, "one_molecule": 1.0 / sc.Avogadro / V, "one_molecule_nM": 1.0 / sc.Avogadro / V * 1e6}


_add("cmb_molecules_per_cell",
     name="Number of molecules in a cell from a concentration", topic="cell growth / cell size",
     purpose="The number of molecules of a species at concentration c in a compartment of volume V, and the concentration that one molecule per compartment represents.",
     inputs=[_phys("c", "molar_concentration", "concentration", show=_NMOLAR), _phys("V", "volume", "cell (compartment) volume", show=_FL)],
     outputs=[_o("N", "number", "number of molecules"), _o("one_molecule", "molar_concentration", "concentration of one molecule in V"),
              _o("one_molecule_nM", "number", "one molecule per compartment in nM")],
     primary="N", conditions=["c, V > 0"], method="N = c V N_A", latex=r"N = c\,V\,N_A",
     run=molecules_per_cell, reference=_ref_molecules,
     wrong=lambda c, V: _out(N=c * V * N_A * 1e-3, one_molecule=1e3 / (V * N_A), one_molecule_nM=1e9 / (V * N_A)),
     rand=lambda rng: {"c": _sig(_lu(rng, 1e-6, 10.0), 3), "V": _sig(_lu(rng, 1e-18, 1e-12), 3)},
     phrases=("Molecules per cell", "Number of molecules in a cell"),
     examples=[_ex({"c": 1e-6, "V": 1e-18}), _ex({"c": 0.1, "V": 2e-15})],
     level="intermediate",
     extra_q=[{"q": "Molecules per cell: c = 1 nM, V = 1 fL", "like": 0}],
     weak="the same definition with scipy.constants.Avogadro")


# ================================================================================================ DNA AND RNA
_DNA_COMP = {"A": "T", "T": "A", "G": "C", "C": "G"}
_RNA_COMP = {"A": "U", "U": "A", "G": "C", "C": "G"}


def _clean_seq(text, rna_ok=True, dna_ok=True):
    """An upper-case sequence of A, C, G, T (DNA) or A, C, G, U (RNA); spaces ignored; anything else refused."""
    _need(isinstance(text, str), "give the sequence as letters")
    s = ""
    for ch in text.upper():
        if ch == " ":
            continue
        _need(ch in "ACGTU", "the sequence may contain only A, C, G, T or U")
        s += ch
    _need(len(s) >= 1, "the sequence is empty")
    has_t, has_u = "T" in s, "U" in s
    _need(not (has_t and has_u), "a sequence cannot contain both T and U")
    _need(rna_ok or not has_u, "give a DNA sequence (A, C, G, T)")
    _need(dna_ok or not has_t, "give an RNA sequence (A, C, G, U)")
    return s, has_u


def _revcomp(s, rna=False):
    comp = _RNA_COMP if rna else _DNA_COMP
    out = ""
    for k in range(len(s) - 1, -1, -1):
        out += comp[s[k]]
    return out


def _rand_dna(rng, lo, hi):
    return "".join(rng.choice("ACGT") for _ in range(rng.randint(lo, hi)))


def gc_content(seq):
    s, rna = _clean_seq(seq)
    g, c = s.count("G"), s.count("C")
    n = len(s)
    return _out(gc_fraction=(g + c) / n, gc_percent=100.0 * (g + c) / n, length=n, G=g, C=c,
                A=s.count("A"), T_or_U=s.count("U") if rna else s.count("T"))


def _ref_gc(seq):
    from Bio.SeqUtils import gc_fraction
    s = seq.upper().replace(" ", "")
    f = float(gc_fraction(s))
    return {"gc_fraction": f, "gc_percent": 100.0 * f, "length": len(s), "G": s.count("G"), "C": s.count("C"),
            "A": s.count("A"), "T_or_U": s.count("T") + s.count("U")}


_add("cmb_gc_content",
     name="GC content of a DNA or RNA sequence", topic="DNA and RNA / sequence composition",
     purpose="The fraction and percentage of G + C in a DNA or RNA sequence, with the base counts and the length.",
     inputs=[_textin("seq", "DNA or RNA sequence (A, C, G, T or U)")],
     outputs=[_o("gc_fraction", "number", "fraction of G + C"), _o("gc_percent", "number", "GC content in per cent"),
              _o("length", "number", "length in bases"), _o("G", "number", "number of G"), _o("C", "number", "number of C"),
              _o("A", "number", "number of A"), _o("T_or_U", "number", "number of T (DNA) or U (RNA)")],
     primary="gc_fraction", conditions=["only A, C, G, T or U"],
     method="GC = (n_G + n_C) / N", latex=r"\mathrm{GC} = \frac{n_G + n_C}{N}",
     run=gc_content, reference=_ref_gc,
     wrong=lambda seq: (lambda s: _out(gc_fraction=s.count("G") / len(s), gc_percent=100.0 * s.count("G") / len(s), length=len(s),
                                       G=s.count("G"), C=s.count("C"), A=s.count("A"), T_or_U=s.count("T") + s.count("U")))(seq.upper().replace(" ", "")),
     rand=lambda rng: {"seq": _rand_dna(rng, 8, 120) if rng.random() < 0.7 else _rand_dna(rng, 8, 60).replace("T", "U")},
     phrases=("GC content", "G+C content"),
     examples=[_ex({"seq": "ATGCGCGATTACGC"}), _ex({"seq": "GGAUCUUCGGAUCU"})],
     level="beginner")


def tm_wallace(seq):
    s, _ = _clean_seq(seq, rna_ok=False)
    _need(len(s) >= 2, "the primer needs at least 2 bases")
    at, gc = s.count("A") + s.count("T"), s.count("G") + s.count("C")
    return _out(Tm=2.0 * at + 4.0 * gc, AT=at, GC=gc)


def _ref_tm_wallace(seq):
    from Bio.SeqUtils import MeltingTemp as mt
    s = seq.upper().replace(" ", "")
    return {"Tm": float(mt.Tm_Wallace(s)), "AT": s.count("A") + s.count("T"), "GC": s.count("G") + s.count("C")}


_add("cmb_tm_wallace",
     name="Primer melting temperature by the Wallace rule", topic="DNA and RNA / melting temperature",
     purpose="The approximate melting temperature (degC) of a short DNA primer (about 14-20 nt) by the Wallace rule, 2 degC per A/T and 4 degC per G/C.",
     inputs=[_textin("seq", "DNA primer sequence (A, C, G, T)")],
     outputs=[_o("Tm", "number", "melting temperature in degC"), _o("AT", "number", "number of A and T"), _o("GC", "number", "number of G and C")],
     primary="Tm", conditions=["DNA letters only", "at least 2 bases (meant for 14-20 nt)"],
     method="Tm = 2 (A + T) + 4 (G + C) degC", latex=r"T_m = 2(n_A + n_T) + 4(n_G + n_C)\ ^\circ\mathrm{C}",
     run=tm_wallace, reference=_ref_tm_wallace,
     wrong=lambda seq: (lambda s: _out(Tm=4.0 * (s.count("A") + s.count("T")) + 2.0 * (s.count("G") + s.count("C")),
                                       AT=s.count("A") + s.count("T"), GC=s.count("G") + s.count("C")))(seq.upper().replace(" ", "")),
     rand=lambda rng: {"seq": _rand_dna(rng, 10, 30)},
     phrases=("Wallace melting temperature", "Melting temperature by the Wallace rule", "Wallace rule Tm"),
     examples=[_ex({"seq": "ACGTTGCAATGCCGTA"}), _ex({"seq": "GCGCAATTAGCCTAGG"})],
     level="beginner")


def tm_basic(seq):
    s, _ = _clean_seq(seq, rna_ok=False)
    _need(len(s) >= 14, "the basic formula is for 14 nt or more (use the Wallace rule for shorter primers)")
    n = len(s)
    gc = s.count("G") + s.count("C")
    return _out(Tm=64.9 + 41.0 * (gc - 16.4) / n, gc_percent=100.0 * gc / n, length=n)


def _ref_tm_basic(seq):
    from Bio.SeqUtils import MeltingTemp as mt
    from Bio.SeqUtils import gc_fraction
    s = seq.upper().replace(" ", "")
    return {"Tm": float(mt.Tm_GC(s, userset=(64.9, 0.41, 672.4, 0), saltcorr=0)), "gc_percent": 100.0 * float(gc_fraction(s)),
            "length": len(s)}


_add("cmb_tm_basic",
     name="Basic GC melting temperature of an oligonucleotide", topic="DNA and RNA / melting temperature",
     purpose="The basic melting temperature of a DNA oligonucleotide of 14 nt or more, Tm = 64.9 + 41 (G + C - 16.4) / N (50 mM Na+ assumed, no salt term).",
     inputs=[_textin("seq", "DNA sequence (A, C, G, T), at least 14 nt", lo=1)],
     outputs=[_o("Tm", "number", "melting temperature in degC"), _o("gc_percent", "number", "GC content in per cent"),
              _o("length", "number", "length in nt")],
     primary="Tm", conditions=["DNA letters only", "N >= 14"],
     method="Tm = 64.9 + 41 (nG + nC - 16.4) / N", latex=r"T_m = 64.9 + 41\,\frac{n_G + n_C - 16.4}{N}",
     run=tm_basic, reference=_ref_tm_basic,
     wrong=lambda seq: (lambda s: _out(Tm=64.9 + 41.0 * (s.count("G") + s.count("C")) / len(s),
                                       gc_percent=100.0 * (s.count("G") + s.count("C")) / len(s), length=len(s)))(seq.upper().replace(" ", "")),
     rand=lambda rng: {"seq": _rand_dna(rng, 14, 60)},
     phrases=("Basic melting temperature", "Basic GC melting temperature", "Basic Tm"),
     examples=[_ex({"seq": "AGCGGATAACAATTTCACACAGGA"}), _ex({"seq": "GTAAAACGACGGCCAGT"})],
     level="beginner",
     extra_q=[{"q": "Basic Tm: seq = ACGTACGT", "outcome": "INVALID_INPUT"}])


def tm_salt_adjusted(seq, Na):
    s, _ = _clean_seq(seq, rna_ok=False)
    _need(len(s) >= 14, "the salt-adjusted formula is for 14 nt or more")
    _pos("the sodium concentration", Na)
    n = len(s)
    pgc = 100.0 * (s.count("G") + s.count("C")) / n
    return _out(Tm=81.5 + 0.41 * pgc - 600.0 / n + 16.6 * math.log10(Na * 1e-3), gc_percent=pgc, salt_term=16.6 * math.log10(Na * 1e-3))


def _ref_tm_salt(seq, Na):
    from Bio.SeqUtils import MeltingTemp as mt
    from Bio.SeqUtils import gc_fraction
    s = seq.upper().replace(" ", "")
    tm = float(mt.Tm_GC(s, valueset=7, Na=Na, K=0, Tris=0, Mg=0, dNTPs=0))
    corr = float(mt.salt_correction(Na=Na, K=0, Tris=0, Mg=0, dNTPs=0, method=1, seq=s))
    return {"Tm": tm, "gc_percent": 100.0 * float(gc_fraction(s)), "salt_term": corr}


_add("cmb_tm_salt_adjusted",
     name="Salt-adjusted melting temperature of an oligonucleotide", topic="DNA and RNA / melting temperature",
     purpose="The melting temperature of a DNA oligonucleotide of 14 nt or more with a sodium correction: Tm = 81.5 + 16.6 log10[Na+] + 0.41 %GC - 600/N.",
     inputs=[_textin("seq", "DNA sequence (A, C, G, T), at least 14 nt"),
             _phys("Na", "molar_concentration", "monovalent cation (Na+) concentration")],
     outputs=[_o("Tm", "number", "melting temperature in degC"), _o("gc_percent", "number", "GC content in per cent"),
              _o("salt_term", "number", "salt term 16.6 log10[Na+] in degC")],
     primary="Tm", conditions=["DNA letters only", "N >= 14", "Na+ > 0"],
     method="Tm = 81.5 + 16.6 log10([Na+]/M) + 0.41 %GC - 600/N",
     latex=r"T_m = 81.5 + 16.6\log_{10}[\mathrm{Na^+}] + 0.41\,\%GC - \frac{600}{N}",
     run=tm_salt_adjusted, reference=_ref_tm_salt,
     wrong=lambda seq, Na: (lambda s: (lambda p: _out(Tm=81.5 + 0.41 * p - 600.0 / len(s) + 16.6 * math.log10(Na), gc_percent=p,
                                                      salt_term=16.6 * math.log10(Na)))(100.0 * (s.count("G") + s.count("C")) / len(s)))(seq.upper().replace(" ", "")),
     rand=lambda rng: {"seq": _rand_dna(rng, 14, 70), "Na": _sig(_lu(rng, 10.0, 1000.0), 3)},
     phrases=("Salt-adjusted melting temperature", "Salt adjusted Tm"),
     examples=[_ex({"seq": "AGCGGATAACAATTTCACACAGGA", "Na": 50.0}), _ex({"seq": "GTAAAACGACGGCCAGT", "Na": 100.0})],
     level="intermediate",
     extra_q=[{"q": "Salt adjusted Tm: seq = AGCGGATAACAATTTCACACAGGA, Na = 0.05 M", "like": 0}])


# SantaLucia & Hicks (2004) unified nearest-neighbour parameters (dH kcal/mol, dS cal/(mol K)), as in Biopython DNA_NN4
_NN4 = {"AA/TT": (-7.6, -21.3), "AT/TA": (-7.2, -20.4), "TA/AT": (-7.2, -21.3), "CA/GT": (-8.5, -22.7),
        "GT/CA": (-8.4, -22.4), "CT/GA": (-7.8, -21.0), "GA/CT": (-8.2, -22.2), "CG/GC": (-10.6, -27.2),
        "GC/CG": (-9.8, -24.4), "GG/CC": (-8.0, -19.9)}
_NN4_INIT = (0.2, -5.7)
_NN4_INIT_AT = (2.2, 6.9)
_NN4_SYM = (0.0, -1.4)


def _nn_sums(s):
    c = ""
    for ch in s:
        c += _DNA_COMP[ch]
    dh, ds = _NN4_INIT[0], _NN4_INIT[1]
    ends = s[0] + s[-1]
    at = ends.count("A") + ends.count("T")
    dh += _NN4_INIT_AT[0] * at
    ds += _NN4_INIT_AT[1] * at
    for i in range(len(s) - 1):
        nb = s[i:i + 2] + "/" + c[i:i + 2]
        rv = nb[::-1]
        v = _NN4[nb] if nb in _NN4 else _NN4[rv]
        dh += v[0]
        ds += v[1]
    selfcomp = s == _revcomp(s)
    if selfcomp:
        dh += _NN4_SYM[0]
        ds += _NN4_SYM[1]
    return dh, ds, selfcomp


def tm_nearest_neighbor(seq, Na, c1, c2):
    s, _ = _clean_seq(seq, rna_ok=False)
    _need(len(s) >= 2, "the sequence needs at least 2 bases")
    _pos("Na and c1", Na, c1)
    _nonneg("c2", c2)
    dh, ds, selfcomp = _nn_sums(s)
    if selfcomp:
        k = c1
    else:
        _need(c1 > c2 / 2.0, "c1 must exceed c2/2 (k = c1 - c2/2 must be positive)")
        k = c1 - c2 / 2.0
    k_molar = k * 1e-3                                  # mol/m^3 -> mol/L
    ds_corr = ds + 0.368 * (len(s) - 1) * math.log(Na * 1e-3)
    tm = (1000.0 * dh) / (ds_corr + 1.987 * math.log(k_molar)) - 273.15
    dg37 = dh - 310.15 * ds_corr / 1000.0
    return _out(Tm=tm, dH=dh, dS=ds_corr, dG37=dg37)


def _ref_tm_nn(seq, Na, c1, c2):
    from Bio.SeqUtils import MeltingTemp as mt
    from Bio.Seq import Seq
    s = seq.upper().replace(" ", "")
    rc = str(Seq(s).reverse_complement())
    selfc = s == rc
    tm = float(mt.Tm_NN(s, nn_table=mt.DNA_NN4, Na=Na, K=0, Tris=0, Mg=0, dNTPs=0, dnac1=c1 * 1e6, dnac2=c2 * 1e6,
                        saltcorr=5, selfcomp=selfc))
    t = mt.DNA_NN4
    comp = str(Seq(s).complement())
    dh = t["init"][0] + t["init_A/T"][0] * (s[0] in "AT") + t["init_A/T"][0] * (s[-1] in "AT")
    ds = t["init"][1] + t["init_A/T"][1] * (s[0] in "AT") + t["init_A/T"][1] * (s[-1] in "AT")
    for i in range(len(s) - 1):
        key = s[i:i + 2] + "/" + comp[i:i + 2]
        val = t.get(key) or t[key[::-1]]
        dh += val[0]
        ds += val[1]
    if selfc:
        dh += t["sym"][0]
        ds += t["sym"][1]
    ds += float(mt.salt_correction(Na=Na, K=0, Tris=0, Mg=0, dNTPs=0, method=5, seq=s))
    return {"Tm": tm, "dH": dh, "dS": ds, "dG37": dh - 310.15 * ds / 1000.0}


def _rand_tm_nn(rng):
    s = _rand_dna(rng, 10, 35)
    if rng.random() < 0.15:
        h = _rand_dna(rng, 5, 12)
        s = h + _revcomp(h)
    c1 = _sig(_lu(rng, 1e-5, 1e-3), 3)           # 10 nM .. 1 uM in mol/m^3
    return {"seq": s, "Na": _sig(_lu(rng, 10.0, 1000.0), 3), "c1": c1, "c2": _sig(c1 * rng.uniform(0.0, 1.5), 3)}


_add("cmb_tm_nearest_neighbor",
     name="Nearest-neighbour melting temperature of a DNA duplex", topic="DNA and RNA / melting temperature",
     purpose="The melting temperature of a perfectly matched DNA duplex by nearest-neighbour thermodynamics (SantaLucia-Hicks 2004 unified parameters) with the SantaLucia (1998) sodium correction, and the duplex dH, dS and dG at 37 degC.",
     inputs=[_textin("seq", "DNA sequence 5'->3' (A, C, G, T)"),
             _phys("Na", "molar_concentration", "monovalent cation (Na+) concentration"),
             _phys("c1", "molar_concentration", "concentration of the more concentrated strand", show=_NMOLAR),
             _phys("c2", "molar_concentration", "concentration of the less concentrated strand (0 if in large excess)", show=_NMOLAR)],
     outputs=[_o("Tm", "number", "melting temperature in degC"), _o("dH", "number", "duplex enthalpy in kcal/mol"),
              _o("dS", "number", "salt-corrected duplex entropy in cal/(mol K)"), _o("dG37", "number", "free energy at 37 degC in kcal/mol")],
     primary="Tm", conditions=["DNA letters, length >= 2", "Na+ > 0", "c1 > c2/2 (self-complementary: k = c1)"],
     method="sum nearest-neighbour dH and dS with initiation and terminal A/T terms (+ symmetry); dS += 0.368 (N-1) ln[Na+]; Tm = 1000 dH / (dS + R ln k) - 273.15, k = c1 - c2/2",
     latex=r"T_m = \frac{1000\,\Delta H^\circ}{\Delta S^\circ + 0.368(N-1)\ln[\mathrm{Na^+}] + R\ln k} - 273.15",
     run=tm_nearest_neighbor, reference=_ref_tm_nn,
     wrong=lambda seq, Na, c1, c2: (lambda s: (lambda r: _out(Tm=(1000.0 * r[0]) / (r[1] + 1.987 * math.log((c1 if r[2] else c1 - c2 / 2.0) * 1e-3)) - 273.15,
                                                             dH=r[0], dS=r[1], dG37=r[0] - 310.15 * r[1] / 1000.0))(_nn_sums(s)))(seq.upper().replace(" ", "")),
     rand=_rand_tm_nn,
     phrases=("Nearest-neighbour melting temperature", "Nearest-neighbor melting temperature", "Nearest-neighbour Tm"),
     examples=[_ex({"seq": "CGTTCCAAAGATGTGGGCATGAGCTTAC", "Na": 50.0, "c1": 0.00025, "c2": 0.00025}),
               _ex({"seq": "GCGAATTCGC", "Na": 100.0, "c1": 0.0005, "c2": 0.0})],
     level="expert",
     extra_q=[{"q": "Nearest-neighbour Tm: seq = CGTTCCAAAGATGTGGGCATGAGCTTAC, Na = 50 mM, c1 = 0.25 uM, c2 = 250 nM", "like": 0}])


_ATOM = {"H": 1.00794, "O": 15.9994, "P": 30.973762}
_HPO3 = _ATOM["H"] + _ATOM["P"] + 3.0 * _ATOM["O"]
_WATER = 18.0153
_DNA_MONO = {"A": 331.2218, "C": 307.1971, "G": 347.2212, "T": 322.2085}   # Biopython IUPACData average weights
_RNA_MONO = {"A": 347.2212, "C": 323.1965, "G": 363.2206, "U": 324.1813}


def _strand_mw(s, rna):
    tab = _RNA_MONO if rna else _DNA_MONO
    w = 0.0
    for ch in s:
        w += tab[ch]
    return w - (len(s) - 1) * _WATER


def oligo_molecular_weight(seq, strands):
    s, rna = _clean_seq(seq)
    _need(strands in ("single", "double"), "strands: single or double")
    mw = _strand_mw(s, rna)
    n_str = 1
    if strands == "double":
        comp = _RNA_COMP if rna else _DNA_COMP
        c = ""
        for ch in s:
            c += comp[ch]
        mw += _strand_mw(c, rna)
        n_str = 2
    return _out(MW=mw / 1000.0, MW_g_mol=mw, MW_hydroxyl_g_mol=mw - n_str * _HPO3)


def _ref_oligo_mw(seq, strands):
    from Bio.SeqUtils import molecular_weight
    from Bio.Data import IUPACData
    s = seq.upper().replace(" ", "")
    kind = "RNA" if "U" in s else "DNA"
    mw = float(molecular_weight(s, seq_type=kind, double_stranded=(strands == "double")))
    aw = IUPACData.atom_weights
    hpo3 = aw["H"] + aw["P"] + 3 * aw["O"]
    return {"MW": mw / 1000.0, "MW_g_mol": mw, "MW_hydroxyl_g_mol": mw - (2 if strands == "double" else 1) * hpo3}


_add("cmb_oligo_molecular_weight",
     name="Molecular weight of a DNA or RNA oligonucleotide", topic="DNA and RNA / molecular weight",
     purpose="The average molecular weight of a single- or double-stranded DNA or RNA oligonucleotide, with a 5'-phosphate (sum of nucleotide monophosphates minus waters) and as the 5'-hydroxyl form of synthetic oligos.",
     inputs=[_textin("seq", "DNA or RNA sequence 5'->3'"), _choice("strands", ("single", "double"), "single strand or the duplex with its complement")],
     outputs=[_o("MW", "molar_mass", "molecular weight (5'-phosphate)"), _o("MW_g_mol", "number", "molecular weight in g/mol (5'-phosphate)"),
              _o("MW_hydroxyl_g_mol", "number", "molecular weight in g/mol with 5'-hydroxyl ends")],
     primary="MW", conditions=["only A, C, G, T (DNA) or A, C, G, U (RNA)"],
     method="MW = sum of nucleotide monophosphate weights - (n - 1) x 18.0153; 5'-OH form minus HPO3 (79.98) per strand",
     latex=r"M = \sum_i m_i - (n - 1)\,M_{H_2O}",
     run=oligo_molecular_weight, reference=_ref_oligo_mw,
     wrong=lambda seq, strands: (lambda s: (lambda w: _out(MW=w / 1000.0, MW_g_mol=w, MW_hydroxyl_g_mol=w - _HPO3))(
         sum((_RNA_MONO if "U" in s else _DNA_MONO)[ch] for ch in s)))(seq.upper().replace(" ", "")),
     rand=lambda rng: {"seq": _rand_dna(rng, 5, 60) if rng.random() < 0.7 else _rand_dna(rng, 5, 40).replace("T", "U"),
                       "strands": rng.choice(["single", "double"])},
     phrases=("Oligonucleotide molecular weight", "Oligo molecular weight", "Molecular weight of an oligonucleotide"),
     examples=[_ex({"seq": "ATGCATGCAT", "strands": "single"}), _ex({"seq": "GGAUCC", "strands": "double"})],
     level="intermediate")


_EPS1 = {"A": 15400.0, "C": 7400.0, "G": 11500.0, "T": 8700.0}
_EPS2 = {"AA": 27400.0, "AC": 21200.0, "AG": 25000.0, "AT": 22800.0, "CA": 21200.0, "CC": 14600.0, "CG": 18000.0,
         "CT": 15200.0, "GA": 25200.0, "GC": 17600.0, "GG": 21600.0, "GT": 20000.0, "TA": 23400.0, "TC": 16200.0,
         "TG": 19000.0, "TT": 16800.0}


def oligo_extinction(seq):
    s, _ = _clean_seq(seq, rna_ok=False)
    n = len(s)
    if n == 1:
        eps = _EPS1[s]
    else:
        eps = 0.0
        for i in range(n - 1):
            eps += _EPS2[s[i:i + 2]]
        for i in range(1, n - 1):
            eps -= _EPS1[s[i]]
    mw = _strand_mw(s, False) - _HPO3
    return _out(eps=eps / 10.0, eps_L_mol_cm=eps, ug_per_A260=mw / eps * 1000.0, nmol_per_A260=1e6 / eps)


def _ref_oligo_extinction(seq):
    from Bio.SeqUtils import molecular_weight
    from Bio.Data import IUPACData
    s = seq.upper().replace(" ", "")
    pairs = [s[i:i + 2] for i in range(len(s) - 1)]
    # each internal base is shared by two neighbouring pairs: average pair contribution minus half of each shared base
    eps = _EPS1[s[0]] if len(s) == 1 else sum(_EPS2[p] for p in pairs) - sum(_EPS1[b] for b in s[1:-1])
    aw = IUPACData.atom_weights
    mw = float(molecular_weight(s, seq_type="DNA")) - (aw["H"] + aw["P"] + 3 * aw["O"])
    return {"eps": eps / 10.0, "eps_L_mol_cm": eps, "ug_per_A260": mw / eps * 1000.0, "nmol_per_A260": 1e6 / eps}


_add("cmb_oligo_extinction_coefficient",
     name="Extinction coefficient of a DNA oligonucleotide at 260 nm (nearest neighbour)", topic="DNA and RNA / quantification",
     purpose="The molar extinction coefficient at 260 nm of a single-stranded DNA oligonucleotide by the nearest-neighbour method, with micrograms and nanomoles per A260 unit (5'-hydroxyl oligo, 1 mL, 1 cm).",
     inputs=[_textin("seq", "DNA oligonucleotide 5'->3' (A, C, G, T)")],
     outputs=[_o("eps", "molar_absorptivity", "molar extinction coefficient at 260 nm"),
              _o("eps_L_mol_cm", "number", "extinction coefficient in L/(mol cm)"),
              _o("ug_per_A260", "number", "micrograms per A260 unit (per mL at 1 cm)"),
              _o("nmol_per_A260", "number", "nanomoles per A260 unit (per mL at 1 cm)")],
     primary="eps", conditions=["DNA letters only", "single strand"],
     method="eps = sum of nearest-neighbour pair values - sum of internal single-base values (Cantor-Warshaw-Shapiro table)",
     latex=r"\varepsilon_{260} = \sum_{i=1}^{N-1}\varepsilon_{i,i+1} - \sum_{i=2}^{N-1}\varepsilon_i",
     run=oligo_extinction, reference=_ref_oligo_extinction,
     wrong=lambda seq: (lambda s: (lambda e: _out(eps=e / 10.0, eps_L_mol_cm=e, ug_per_A260=(_strand_mw(s, False) - _HPO3) / e * 1000.0,
                                                  nmol_per_A260=1e6 / e))(sum(_EPS1[ch] for ch in s)))(seq.upper().replace(" ", "")),
     rand=lambda rng: {"seq": _rand_dna(rng, 6, 60)},
     phrases=("Oligonucleotide extinction coefficient", "Oligo extinction coefficient", "Extinction coefficient of an oligo"),
     examples=[_ex({"seq": "ATGCATGCAT"}), _ex({"seq": "GTAAAACGACGGCCAGT"})],
     level="advanced",
     weak="the same published nearest-neighbour table summed by pairs and shared bases; the mass from Biopython")


_A260_FACTOR = {"dsDNA": 50.0, "ssDNA": 33.0, "RNA": 40.0}


def nucleic_acid_a260(A260, molecule, dilution=1.0, l=0.01, V=None):
    _nonneg("A260", A260)
    _need(molecule in _A260_FACTOR, "molecule: dsDNA, ssDNA or RNA")
    _need(_isnum(dilution) and dilution >= 1, "the dilution factor must be at least 1")
    _pos("the path length l", l)
    ug_ml = A260 * _A260_FACTOR[molecule] * dilution / (l * 100.0)
    out = {"c": ug_ml * 1e-3, "c_ug_mL": ug_ml}
    if V is not None:
        _pos("the volume V", V)
        out["mass_ug"] = ug_ml * V * 1e6
    return _out(**out)


def _ref_a260(A260, molecule, dilution=1.0, l=0.01, V=None):
    a = {"dsDNA": 0.020, "ssDNA": 1.0 / 33.0, "RNA": 0.025}[molecule]   # specific absorptivity, mL/(ug cm)
    ug_ml = A260 / (a * l * 100.0) * dilution
    out = {"c": ug_ml / 1000.0, "c_ug_mL": ug_ml}
    if V is not None:
        out["mass_ug"] = ug_ml * V * 1e6
    return out


_add("cmb_nucleic_acid_a260",
     name="Nucleic acid concentration from A260", topic="DNA and RNA / quantification",
     purpose="The concentration of dsDNA, ssDNA or RNA from its absorbance at 260 nm (50, 33 and 40 ug/mL per unit at 1 cm), with the dilution and path length, and the mass in a volume.",
     inputs=[_numin("A260", "absorbance at 260 nm (blank-corrected)"),
             _choice("molecule", ("dsDNA", "ssDNA", "RNA"), "dsDNA, ssDNA or RNA"),
             _numin("dilution", "dilution factor of the measured sample (default 1)", required=False),
             _phys("l", "length", "path length (default 1 cm)", required=False, show=_CM),
             _phys("V", "volume", "volume of the stock (optional, for the total mass)", required=False, show=_UL)],
     outputs=[_o("c", "mass_concentration", "concentration of the stock"), _o("c_ug_mL", "number", "concentration in ug/mL (= ng/uL)"),
              _o("mass_ug", "number", "micrograms in V (when V is given)")],
     primary="c", conditions=["A260 >= 0 (best within 0.1-1.0)", "dilution >= 1", "l > 0"],
     method="c (ug/mL) = A260 x factor x dilution / l(cm); factor 50 (dsDNA), 33 (ssDNA), 40 (RNA)",
     latex=r"c = A_{260}\times F \times DF / \ell",
     run=nucleic_acid_a260, reference=_ref_a260,
     wrong=lambda A260, molecule, dilution=1.0, l=0.01, V=None: nucleic_acid_a260(A260, {"dsDNA": "RNA", "RNA": "ssDNA", "ssDNA": "dsDNA"}[molecule], dilution, l, V),
     rand=lambda rng: dict({"A260": _sig(rng.uniform(0.05, 1.5), 3), "molecule": rng.choice(["dsDNA", "ssDNA", "RNA"])},
                           **({"dilution": rng.choice([1.0, 10.0, 20.0, 50.0])} if rng.random() < 0.6 else {}),
                           **({"l": rng.choice([0.01, 0.001, 0.005])} if rng.random() < 0.3 else {}),
                           **({"V": _sig(rng.uniform(1e-8, 1e-6), 3)} if rng.random() < 0.3 else {})),
     phrases=("Nucleic acid concentration from A260", "DNA concentration from A260", "RNA concentration from A260", "Concentration from A260"),
     examples=[_ex({"A260": 0.5, "molecule": "dsDNA", "dilution": 10.0}), _ex({"A260": 0.25, "molecule": "RNA", "dilution": 50.0, "V": 5e-8})],
     level="beginner",
     extra_q=[{"q": "DNA concentration from A260: A260 = 0.5, molecule = ds DNA, dilution = 10", "like": 0}],
     weak="the same Beer-Lambert relation written with the specific absorptivities")


def absorbance_ratios(A260, A280, A230=None, A320=0.0):
    _pos("A260 and A280", A260, A280)
    _nonneg("A320", A320)
    a260, a280 = A260 - A320, A280 - A320
    _need(a260 > 0 and a280 > 0, "the background A320 must be below A260 and A280")
    out = {"ratio_260_280": a260 / a280}
    if A230 is not None:
        _pos("A230", A230)
        a230 = A230 - A320
        _need(a230 > 0, "the background A320 must be below A230")
        out["ratio_260_230"] = a260 / a230
    return _out(**out)


def _ref_ratios(A260, A280, A230=None, A320=0.0):
    from fractions import Fraction as Fr
    out = {"ratio_260_280": float((Fr(A260) - Fr(A320)) / (Fr(A280) - Fr(A320)))}
    if A230 is not None:
        out["ratio_260_230"] = float((Fr(A260) - Fr(A320)) / (Fr(A230) - Fr(A320)))
    return out


def _rand_ratios(rng):
    a260 = _sig(rng.uniform(0.2, 1.5), 3)
    d = {"A260": a260, "A280": _sig(a260 / rng.uniform(1.4, 2.2), 3), "A320": _sig(rng.uniform(0.005, 0.05), 2)}
    if rng.random() < 0.6:
        d["A230"] = _sig(a260 / rng.uniform(1.2, 2.4), 3)
    return d


_add("cmb_absorbance_ratios",
     name="A260/A280 and A260/A230 purity ratios", topic="DNA and RNA / quantification",
     purpose="The A260/A280 and A260/A230 ratios of a nucleic acid sample after subtracting the A320 background (about 1.8 for pure DNA, 2.0 for pure RNA; lower values suggest protein or organic contaminants).",
     inputs=[_numin("A260", "absorbance at 260 nm"), _numin("A280", "absorbance at 280 nm"),
             _numin("A230", "absorbance at 230 nm (optional)", required=False), _numin("A320", "background absorbance at 320 nm (default 0)", required=False)],
     outputs=[_o("ratio_260_280", "number", "A260/A280 after background subtraction"),
              _o("ratio_260_230", "number", "A260/A230 after background subtraction (when A230 is given)")],
     primary="ratio_260_280", conditions=["absorbances > 0", "A320 below the other readings"],
     method="ratio = (A260 - A320)/(A280 - A320); (A260 - A320)/(A230 - A320)",
     latex=r"\frac{A_{260} - A_{320}}{A_{280} - A_{320}}",
     run=absorbance_ratios, reference=_ref_ratios,
     wrong=lambda A260, A280, A230=None, A320=0.0: _out(**dict({"ratio_260_280": A280 / A260}, **({"ratio_260_230": A230 / A260} if A230 is not None else {}))),
     rand=_rand_ratios,
     phrases=("Absorbance ratios", "A260/A280 ratio", "Purity ratios"),
     examples=[_ex({"A260": 0.9, "A280": 0.5, "A230": 0.42}), _ex({"A260": 0.62, "A280": 0.33, "A320": 0.02})],
     level="beginner",
     weak="the same ratios in exact rational arithmetic")


def warburg_christian(A280, A260):
    _nonneg("the absorbances", A280, A260)
    p = 1.55 * A280 - 0.76 * A260
    _need(p > 0, "the absorbances give no positive protein estimate (nucleic acid dominates)")
    return _out(protein=p, protein_mg_mL=p, ratio_280_260=A280 / A260 if A260 > 0 else 0.0)


def _ref_warburg(A280, A260):
    import numpy as np
    p = float(np.dot([1.55, -0.76], [A280, A260]))
    return {"protein": p, "protein_mg_mL": p, "ratio_280_260": A280 / A260 if A260 > 0 else 0.0}


_add("cmb_warburg_christian",
     name="Protein concentration by Warburg-Christian (A280 and A260)", topic="DNA and RNA / quantification",
     purpose="The protein concentration (mg/mL) of a sample containing nucleic acid from its absorbances at 280 and 260 nm, protein = 1.55 A280 - 0.76 A260 (1 cm).",
     inputs=[_numin("A280", "absorbance at 280 nm"), _numin("A260", "absorbance at 260 nm")],
     outputs=[_o("protein", "mass_concentration", "protein concentration"), _o("protein_mg_mL", "number", "protein in mg/mL"),
              _o("ratio_280_260", "number", "A280/A260")],
     primary="protein", conditions=["absorbances >= 0, 1 cm path", "a positive estimate"],
     method="protein (mg/mL) = 1.55 A280 - 0.76 A260", latex=r"c_{prot} = 1.55A_{280} - 0.76A_{260}\ \mathrm{mg/mL}",
     run=warburg_christian, reference=_ref_warburg,
     wrong=lambda A280, A260: _out(protein=1.55 * A260 - 0.76 * A280, protein_mg_mL=1.55 * A260 - 0.76 * A280, ratio_280_260=A280 / A260 if A260 > 0 else 0.0),
     rand=lambda rng: (lambda a280: {"A280": a280, "A260": _sig(a280 * rng.uniform(0.5, 1.5), 3)})(_sig(rng.uniform(0.1, 1.5), 3)),
     phrases=("Warburg-Christian protein", "Protein by Warburg-Christian"),
     examples=[_ex({"A280": 0.8, "A260": 0.5}), _ex({"A280": 1.2, "A260": 0.9})],
     level="intermediate",
     weak="the same published coefficients applied as a numpy dot product")


def protein_a280(A280, eps, MW=None, l=0.01):
    _nonneg("A280", A280)
    _pos("the extinction coefficient and the path length", eps, l)
    c = A280 / (eps * l)                       # mol/m^3 (eps in m^2/mol, l in m)
    out = {"c": c, "c_uM": c * 1e3}
    if MW is not None:
        _pos("the molar mass", MW)
        out["c_mg_mL"] = c * MW
    return _out(**out)


def _ref_protein_a280(A280, eps, MW=None, l=0.01):
    eps_M = eps * 10.0                         # L/(mol cm)
    c_M = A280 / (eps_M * l * 100.0)
    out = {"c": c_M * 1e3, "c_uM": c_M * 1e6}
    if MW is not None:
        out["c_mg_mL"] = c_M * MW * 1e3
    return out


_add("cmb_protein_concentration_a280",
     name="Protein concentration from A280 and the extinction coefficient", topic="DNA and RNA / quantification",
     purpose="The molar (and, with the molar mass, mass) concentration of a protein from its absorbance at 280 nm and molar extinction coefficient (Beer-Lambert).",
     inputs=[_numin("A280", "absorbance at 280 nm"), _phys("eps", "molar_absorptivity", "molar extinction coefficient at 280 nm"),
             _phys("MW", "molar_mass", "molar mass of the protein (optional)", required=False),
             _phys("l", "length", "path length (default 1 cm)", required=False, show=_CM)],
     outputs=[_o("c", "molar_concentration", "protein concentration"), _o("c_uM", "number", "concentration in uM"),
              _o("c_mg_mL", "number", "concentration in mg/mL (when MW is given)")],
     primary="c", conditions=["A280 >= 0", "eps, l > 0"],
     method="c = A / (eps l); mass concentration = c x MW", latex=r"c = \frac{A_{280}}{\varepsilon\,\ell}",
     run=protein_a280, reference=_ref_protein_a280,
     wrong=lambda A280, eps, MW=None, l=0.01: protein_a280(A280 * 2.303, eps, MW, l),
     rand=lambda rng: dict({"A280": _sig(rng.uniform(0.05, 1.5), 3), "eps": _sig(_lu(rng, 500.0, 2e4), 3)},
                           **({"MW": _sig(_lu(rng, 5.0, 200.0), 3)} if rng.random() < 0.6 else {}),
                           **({"l": rng.choice([0.01, 0.001])} if rng.random() < 0.3 else {})),
     phrases=("Protein concentration from A280", "Protein concentration by A280"),
     examples=[_ex({"A280": 0.75, "eps": 4357.0, "MW": 66.43}), _ex({"A280": 0.3, "eps": 3790.0})],
     level="beginner",
     extra_q=[{"q": "Protein concentration from A280: A280 = 0.75, eps = 43570 L/(mol*cm), MW = 66.43 kDa", "like": 0}],
     weak="the same Beer-Lambert relation in customary units")


_COPY_MW = {"dsDNA": (617.96, 36.04), "ssDNA": (308.97, 18.02), "RNA": (320.5, 159.0)}


def dna_copy_number(m, n, molecule):
    _pos("the mass m", m)
    _need(isinstance(n, int) and n >= 1, "the length n must be a whole number of bases or base pairs")
    _need(molecule in _COPY_MW, "molecule: dsDNA, ssDNA or RNA")
    a, b = _COPY_MW[molecule]
    mw = a * n + b                              # g/mol
    mol = m * 1e3 / mw
    return _out(MW=mw / 1000.0, MW_g_mol=mw, pmol=mol * 1e12, copies=mol * N_A)


def _ref_copy_number(m, n, molecule):
    sc = _sc()
    a, b = {"dsDNA": (617.96, 36.04), "ssDNA": (308.97, 18.02), "RNA": (320.5, 159.0)}[molecule]
    mw = a * n + b
    moles = (m / sc.gram) / mw
    return {"MW": mw / 1000.0, "MW_g_mol": mw, "pmol": moles / sc.pico, "copies": moles * sc.Avogadro}


_add("cmb_dna_copy_number",
     name="Copy number and moles of a DNA or RNA molecule from its mass", topic="DNA and RNA / quantification",
     purpose="The molecular weight, picomoles and number of copies in a mass of dsDNA, ssDNA or RNA of a given length (NEB average-mass formulas).",
     inputs=[_phys("m", "mass", "mass of nucleic acid", show=_NG),
             _intin("n", "length in base pairs (dsDNA) or bases (ssDNA, RNA)", lo=1, words=("bp", "nt", "bases")),
             _choice("molecule", ("dsDNA", "ssDNA", "RNA"), "dsDNA, ssDNA or RNA")],
     outputs=[_o("MW", "molar_mass", "molecular weight"), _o("MW_g_mol", "number", "molecular weight in g/mol"),
              _o("pmol", "number", "picomoles"), _o("copies", "number", "number of molecules")],
     primary="copies", conditions=["m > 0", "n a whole number >= 1"],
     method="MW = 617.96 n + 36.04 (dsDNA), 308.97 n + 18.02 (ssDNA), 320.5 n + 159.0 (RNA) g/mol; copies = m / MW x N_A",
     latex=r"N = \frac{m}{M}\,N_A",
     run=dna_copy_number, reference=_ref_copy_number,
     wrong=lambda m, n, molecule: (lambda mw: _out(MW=mw / 1000.0, MW_g_mol=mw, pmol=m * 1e3 / mw * 1e12, copies=m * 1e3 / mw * N_A))(650.0 * n),
     rand=lambda rng: {"m": _sig(_lu(rng, 1e-15, 1e-8), 3), "n": rng.randint(20, 50000), "molecule": rng.choice(["dsDNA", "ssDNA", "RNA"])},
     phrases=("DNA copy number", "Copy number from mass", "Nucleic acid copy number"),
     examples=[_ex({"m": 1e-12, "n": 3000, "molecule": "dsDNA"}), _ex({"m": 5e-11, "n": 1500, "molecule": "RNA"})],
     level="intermediate",
     extra_q=[{"q": "DNA copy number: m = 1 ng, n = 3000 bp, molecule = dsDNA", "like": 0}],
     weak="the same published mass formulas with scipy.constants")


def ligation_insert_mass(m_vector, L_vector, L_insert, ratio):
    _pos("the vector mass, the lengths and the ratio", m_vector, ratio)
    _need(isinstance(L_vector, int) and isinstance(L_insert, int) and L_vector >= 1 and L_insert >= 1, "lengths are whole numbers of bp")
    m_ins = ratio * m_vector * L_insert / L_vector
    mol_vec = m_vector * 1e3 / (660.0 * L_vector)
    return _out(m_insert=m_ins, m_insert_ng=m_ins * 1e12, vector_fmol=mol_vec * 1e15, insert_fmol=ratio * mol_vec * 1e15)


def _ref_ligation(m_vector, L_vector, L_insert, ratio):
    mw_v, mw_i = 660.0 * L_vector, 660.0 * L_insert
    n_v = m_vector * 1e3 / mw_v
    n_i = ratio * n_v
    m_i = n_i * mw_i / 1e3
    return {"m_insert": m_i, "m_insert_ng": m_i * 1e12, "vector_fmol": n_v * 1e15, "insert_fmol": n_i * 1e15}


_add("cmb_ligation_insert_mass",
     name="Insert mass for a ligation at an insert:vector molar ratio", topic="DNA and RNA / cloning",
     purpose="The mass of insert DNA needed for a ligation at a chosen insert:vector molar ratio, and the femtomoles of vector and insert (660 g/mol per bp).",
     inputs=[_phys("m_vector", "mass", "mass of vector DNA", show=_NG),
             _intin("L_vector", "vector length in bp", lo=1, words=("bp",)), _intin("L_insert", "insert length in bp", lo=1, words=("bp",)),
             _numin("ratio", "insert:vector molar ratio (e.g. 3 for 3:1)")],
     outputs=[_o("m_insert", "mass", "mass of insert"), _o("m_insert_ng", "number", "mass of insert in ng"),
              _o("vector_fmol", "number", "vector in fmol"), _o("insert_fmol", "number", "insert in fmol")],
     primary="m_insert", conditions=["masses, ratio > 0", "lengths whole numbers >= 1"],
     method="m_insert = ratio x m_vector x L_insert / L_vector", latex=r"m_{ins} = r\,m_{vec}\,\frac{L_{ins}}{L_{vec}}",
     run=ligation_insert_mass, reference=_ref_ligation,
     wrong=lambda m_vector, L_vector, L_insert, ratio: (lambda mi: _out(m_insert=mi, m_insert_ng=mi * 1e12, vector_fmol=m_vector * 1e3 / (660.0 * L_vector) * 1e15,
                                                                        insert_fmol=ratio * m_vector * 1e3 / (660.0 * L_vector) * 1e15))(ratio * m_vector * L_vector / L_insert),
     rand=lambda rng: {"m_vector": _sig(_lu(rng, 1e-11, 2e-10), 3), "L_vector": rng.randint(2500, 10000), "L_insert": rng.randint(100, 4000),
                       "ratio": rng.choice([1.0, 2.0, 3.0, 5.0, 0.5])},
     phrases=("Ligation insert mass", "Insert mass for ligation"),
     examples=[_ex({"m_vector": 5e-11, "L_vector": 3000, "L_insert": 1000, "ratio": 3.0}),
               _ex({"m_vector": 1e-10, "L_vector": 5400, "L_insert": 750, "ratio": 1.0})],
     level="intermediate",
     extra_q=[{"q": "Ligation insert mass: m_vector = 50 ng, L_vector = 3000 bp, L_insert = 1000 bp, ratio = 3", "like": 0}],
     weak="the same proportion computed through moles at 660 g/mol per bp")


def reverse_complement(seq):
    s, rna = _clean_seq(seq)
    comp = _RNA_COMP if rna else _DNA_COMP
    c = ""
    for ch in s:
        c += comp[ch]
    rev = ""
    for k in range(len(s) - 1, -1, -1):
        rev += s[k]
    return _out(reverse_complement=_revcomp(s, rna), complement=c, reverse=rev)


def _ref_revcomp(seq):
    from Bio.Seq import Seq
    s = Seq(seq.upper().replace(" ", ""))
    if "U" in str(s):
        return {"reverse_complement": str(s.reverse_complement_rna()), "complement": str(s.complement_rna()), "reverse": str(s)[::-1]}
    return {"reverse_complement": str(s.reverse_complement()), "complement": str(s.complement()), "reverse": str(s)[::-1]}


_add("cmb_reverse_complement",
     name="Reverse complement of a DNA or RNA sequence", topic="DNA and RNA / sequence operations",
     purpose="The reverse complement (the other strand read 5'->3'), the complement and the reverse of a DNA or RNA sequence.",
     inputs=[_textin("seq", "DNA or RNA sequence 5'->3'")],
     outputs=[_o("reverse_complement", "number", "reverse complement 5'->3'"), _o("complement", "number", "complement 3'->5'"),
              _o("reverse", "number", "the sequence reversed")],
     primary="reverse_complement", conditions=["only A, C, G, T or U"],
     method="complement A-T (A-U), G-C, then reverse", latex=r"\mathrm{rc}(s) = \overline{s_N}\cdots\overline{s_1}",
     run=reverse_complement, reference=_ref_revcomp,
     wrong=lambda seq: (lambda o: dict(o, reverse_complement=o["complement"]))(reverse_complement(seq)),
     rand=lambda rng: {"seq": _rand_dna(rng, 4, 80) if rng.random() < 0.75 else _rand_dna(rng, 4, 50).replace("T", "U")},
     phrases=("Reverse complement", "Reverse complement of a sequence"),
     examples=[_ex({"seq": "ATGCGTACCTGA"}), _ex({"seq": "GGAUCCUUA"})],
     level="beginner")


def transcribe(seq, strand):
    s, rna = _clean_seq(seq, rna_ok=False)
    _need(strand in ("coding", "template"), "strand: coding or template")
    src = s if strand == "coding" else _revcomp(s)
    m = ""
    for ch in src:
        m += "U" if ch == "T" else ch
    return _out(mRNA=m, length=len(m))


def _ref_transcribe(seq, strand):
    from Bio.Seq import Seq
    s = Seq(seq.upper().replace(" ", ""))
    m = s.transcribe() if strand == "coding" else s.reverse_complement().transcribe()
    return {"mRNA": str(m), "length": len(m)}


_add("cmb_transcribe",
     name="Transcription of a DNA strand into mRNA", topic="DNA and RNA / sequence operations",
     purpose="The mRNA (5'->3') transcribed from a DNA sequence given 5'->3' as either the coding (sense) strand or the template (antisense) strand.",
     inputs=[_textin("seq", "DNA sequence 5'->3'"), _choice("strand", ("coding", "template"), "whether the given strand is the coding or the template strand")],
     outputs=[_o("mRNA", "number", "mRNA sequence 5'->3'"), _o("length", "number", "length in nt")],
     primary="mRNA", conditions=["DNA letters only"],
     method="coding strand: T -> U; template strand: reverse complement, then T -> U",
     latex=r"\mathrm{mRNA} = \mathrm{coding}(T\to U) = \mathrm{rc}(\mathrm{template})(T\to U)",
     run=transcribe, reference=_ref_transcribe,
     wrong=lambda seq, strand: transcribe(seq, "template" if strand == "coding" else "coding"),
     rand=lambda rng: {"seq": _rand_dna(rng, 4, 90), "strand": rng.choice(["coding", "template"])},
     phrases=("Transcribe DNA", "Transcription of DNA", "mRNA from DNA"),
     examples=[_ex({"seq": "ATGGCCTTTAAG", "strand": "coding"}), _ex({"seq": "TACCGGAAATTC", "strand": "template"})],
     level="beginner")


_CODONS = {}
_AA_ORDER = "FFLLSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG"
_B = "TCAG"
for _i in range(4):
    for _j in range(4):
        for _k in range(4):
            _CODONS[_B[_i] + _B[_j] + _B[_k]] = _AA_ORDER[16 * _i + 4 * _j + _k]


def translate(seq):
    s, rna = _clean_seq(seq)
    d = s.replace("U", "T")
    n = len(d) // 3
    _need(n >= 1, "the sequence needs at least one codon")
    full = ""
    for k in range(n):
        full += _CODONS[d[3 * k:3 * k + 3]]
    stop = full.find("*")
    protein = full if stop < 0 else full[:stop]
    return _out(protein=protein, full_translation=full, length_aa=len(protein), stops=full.count("*"))


def _ref_translate(seq):
    from Bio.Seq import Seq
    s = seq.upper().replace(" ", "").replace("U", "T")
    s = s[:3 * (len(s) // 3)]
    full = str(Seq(s).translate(table=1))
    prot = str(Seq(s).translate(table=1, to_stop=True))
    return {"protein": prot, "full_translation": full, "length_aa": len(prot), "stops": full.count("*")}


_add("cmb_translate",
     name="Translation of an mRNA or DNA sequence into protein", topic="DNA and RNA / sequence operations",
     purpose="The protein encoded from the first base (frame 1) by the standard genetic code: up to the first stop codon, and the full translation with stops as *. A trailing incomplete codon is ignored.",
     inputs=[_textin("seq", "mRNA or coding-strand DNA 5'->3'")],
     outputs=[_o("protein", "number", "amino acids up to the first stop (one-letter code)"),
              _o("full_translation", "number", "full translation with * for stops"),
              _o("length_aa", "number", "length of the protein up to the first stop"), _o("stops", "number", "number of stop codons")],
     primary="protein", conditions=["only A, C, G, T or U", "at least one codon"],
     method="read codons from base 1 with the standard code (NCBI table 1)", latex=r"\text{codon} \mapsto \text{amino acid (table 1)}",
     run=translate, reference=_ref_translate,
     wrong=lambda seq: translate(seq[1:]) if len(seq) > 3 else {"protein": "?"},
     rand=lambda rng: {"seq": ("ATG" + _rand_dna(rng, 6, 120)) if rng.random() < 0.6 else ("AUG" + _rand_dna(rng, 6, 90).replace("T", "U"))},
     phrases=("Translate", "Translate a sequence", "Protein translation"),
     examples=[_ex({"seq": "AUGGCCAUUGUAAUGGGCCGCUGAAAGGGUGCCCGAUAG"}), _ex({"seq": "ATGAAACGCATTAGCACCACCATTACCACCACCATCACCATTACCACAGGTAACGGTGCGGGCTGA"})],
     level="intermediate")


def orf_finder(seq, min_aa):
    s, _ = _clean_seq(seq, rna_ok=False)
    _need(isinstance(min_aa, int) and min_aa >= 1, "min_aa must be a whole number of codons, at least 1")
    L = len(s)
    best = None
    count = 0
    for strand in ("+", "-"):
        t = s if strand == "+" else _revcomp(s)
        for frame in range(3):
            start = -1
            k = frame
            while k + 3 <= L:
                cod = t[k:k + 3]
                aa = _CODONS[cod]
                if aa == "*":
                    if start >= 0:
                        n_aa = (k - start) // 3
                        if n_aa >= min_aa:
                            count += 1
                            prot = ""
                            for q in range(start, k, 3):
                                prot += _CODONS[t[q:q + 3]]
                            cand = (n_aa, strand, start + 1, k + 3, prot)
                            if best is None or n_aa > best[0] or (n_aa == best[0] and (strand, start + 1) < (best[1], best[2])):
                                best = cand
                    start = -1
                elif cod == "ATG" and start < 0:
                    start = k
                k += 3
    _need(best is not None, "no open reading frame of at least min_aa codons")
    n_aa, strand, st, en, prot = best
    return _out(strand=strand, start=st, end=en, length_nt=en - st + 1, length_aa=n_aa, protein=prot, n_orfs=count)


def _ref_orf(seq, min_aa):
    from Bio.Seq import Seq
    s = Seq(seq.upper().replace(" ", ""))
    L = len(s)
    found = []
    for strand, t in (("+", s), ("-", s.reverse_complement())):
        for frame in range(3):
            sub = t[frame:]
            sub = sub[:3 * (len(sub) // 3)]
            prot = str(sub.translate(table=1))
            pos = 0
            for seg in prot.split("*")[:-1]:      # segments closed by a stop
                m = seg.find("M")
                if m >= 0 and len(seg) - m >= min_aa:
                    st = frame + 3 * (pos + m)
                    found.append((len(seg) - m, strand, st + 1, frame + 3 * (pos + len(seg)) + 3, seg[m:]))
                pos += len(seg) + 1
    if not found:
        raise ValueError("no ORF")
    found.sort(key=lambda f: (-f[0], f[1], f[2]))
    n_aa, strand, st, en, prot = found[0]
    return {"strand": strand, "start": st, "end": en, "length_nt": en - st + 1, "length_aa": n_aa, "protein": prot, "n_orfs": len(found)}


def _rand_orf(rng):
    body = "".join(rng.choice([c for c in _CODONS if _CODONS[c] != "*"]) for _ in range(rng.randint(15, 60)))
    orf = "ATG" + body + rng.choice(["TAA", "TAG", "TGA"])
    if rng.random() < 0.4:
        orf = _revcomp(orf)
    s = _rand_dna(rng, 0, 40) + orf + _rand_dna(rng, 0, 40)
    return {"seq": s, "min_aa": rng.randint(5, 15)}


_add("cmb_orf_finder",
     name="Longest open reading frame on both strands", topic="DNA and RNA / sequence operations",
     purpose="The longest open reading frame (ATG to the first in-frame stop) in all six frames of a DNA sequence with at least min_aa codons, its strand, coordinates on that strand, protein and the number of such ORFs.",
     inputs=[_textin("seq", "DNA sequence 5'->3'"), _intin("min_aa", "minimum ORF length in codons (amino acids, stop excluded)", lo=1)],
     outputs=[_o("strand", "number", "+ (given strand) or - (reverse complement)"),
              _o("start", "number", "1-based start of the ATG on that strand (read 5'->3')"),
              _o("end", "number", "1-based end of the stop codon on that strand"),
              _o("length_nt", "number", "ORF length in nt including the stop"), _o("length_aa", "number", "protein length in amino acids"),
              _o("protein", "number", "encoded protein (one-letter code)"), _o("n_orfs", "number", "number of ORFs of at least min_aa codons")],
     primary="length_aa", conditions=["DNA letters only", "min_aa >= 1", "at least one ORF"],
     method="scan the three frames of each strand; an ORF starts at the first ATG after the previous stop and ends at the next stop; keep the longest (ties: + strand, then the lowest start)",
     latex=r"\mathrm{ORF} = \mathrm{ATG}\,(\mathrm{NNN})^{n}\,\{\mathrm{TAA,TAG,TGA}\}",
     run=orf_finder, reference=_ref_orf,
     wrong=lambda seq, min_aa: (lambda o: dict(o, length_aa=o["length_aa"] + 1, length_nt=o["length_nt"] - 3))(orf_finder(seq, min_aa)),
     rand=_rand_orf,
     phrases=("Find the longest ORF", "Open reading frame", "Longest open reading frame"),
     examples=[_ex({"seq": "CCATGAAACGCATTAGCACCACCATTACCACCACCATCACCATTACCACAGGTAACGGTGCGGGCTGAGG", "min_aa": 10}),
               _ex({"seq": "TTATTACTTCCAGCCAGCCATTTGAAGCTAACGG", "min_aa": 5})],
     level="advanced")


def genome_coverage(L, N, G):
    _need(isinstance(L, int) and isinstance(N, int) and L >= 1 and N >= 1, "read length and read count are whole numbers")
    _pos("the genome size G", G)
    c = L * N / G
    p0 = math.exp(-c)
    # a genome with reads is at least one contig: N e^-C counts the gaps between islands and falls below 1 at high
    # depth (60x gave "1.751e-20 contigs"; review 2026-11-14)
    return _out(coverage=c, fraction_uncovered=p0, bases_uncovered=G * p0, contigs=max(1.0, N * p0))


def _ref_coverage(L, N, G):
    from scipy.stats import poisson
    c = L * N / G
    p0 = float(poisson.pmf(0, c))
    return {"coverage": c, "fraction_uncovered": p0, "bases_uncovered": G * p0, "contigs": max(1.0, N * p0)}


_add("cmb_genome_coverage",
     name="Sequencing coverage and uncovered fraction (Lander-Waterman)", topic="DNA and RNA / genomics",
     purpose="The mean sequencing depth C = L N / G, the expected fraction and number of bases not covered (e^-C) and the expected number of contigs (N e^-C, at least one).",
     inputs=[_intin("L", "read length in bp", lo=1, words=("bp", "nt")), _intin("N", "number of reads", lo=1),
             _numin("G", "genome size in bp", words=("bp",))],
     outputs=[_o("coverage", "number", "mean coverage (depth)"), _o("fraction_uncovered", "number", "fraction of bases not covered e^-C"),
              _o("bases_uncovered", "number", "expected bases not covered"), _o("contigs", "number", "expected number of contigs max(1, N e^-C)")],
     primary="coverage", conditions=["L, N whole numbers >= 1", "G > 0", "reads placed uniformly at random"],
     method="C = L N / G; P(0 reads) = e^-C (Poisson)", latex=r"C = \frac{LN}{G},\quad P_0 = e^{-C}",
     run=genome_coverage, reference=_ref_coverage,
     wrong=lambda L, N, G: (lambda c: _out(coverage=c, fraction_uncovered=1.0 - c / (1.0 + c), bases_uncovered=G / (1.0 + c), contigs=N / (1.0 + c)))(L * N / G),
     rand=lambda rng: (lambda G: {"L": rng.choice([50, 100, 150, 250, 500]), "N": int(G * rng.uniform(0.2, 8.0) / 150) + 1, "G": G})(_sig(_lu(rng, 1e4, 1e9), 3)),
     phrases=("Genome coverage", "Sequencing coverage"),
     examples=[_ex({"L": 150, "N": 2000000, "G": 5e6}), _ex({"L": 100, "N": 30000, "G": 1e6})],
     level="intermediate")


def library_size(G, I, P):
    _pos("the genome size and the insert size", G, I)
    _need(I < G, "the insert must be smaller than the genome")
    _frac("the probability P", P)
    n = math.log(1.0 - P) / math.log(1.0 - I / G)
    clones = int(math.ceil(n - 1e-9))
    return _out(clones_exact=n, clones=clones)


def _ref_library(G, I, P):
    f = I / G
    n = _brentq(lambda x: 1.0 - (1.0 - f) ** x - P, 0.0, 100.0 * G / I * max(1.0, -math.log(1.0 - P)))
    return {"clones_exact": n, "clones": int(math.ceil(n - 1e-9))}


_add("cmb_library_size",
     name="Number of clones for a genomic library (Clarke-Carbon)", topic="DNA and RNA / genomics",
     purpose="The number of random clones needed so that any given sequence is present with probability P, for a genome of size G and inserts of size I.",
     inputs=[_numin("G", "genome size in bp", words=("bp",)), _numin("I", "insert size in bp", words=("bp",)),
             _numin("P", "desired probability (e.g. 99%)", pct=True)],
     outputs=[_o("clones_exact", "number", "number of clones (real value)"), _o("clones", "number", "number of clones rounded up")],
     primary="clones_exact", conditions=["0 < I < G", "0 < P < 1"],
     method="N = ln(1 - P) / ln(1 - I/G)", latex=r"N = \frac{\ln(1 - P)}{\ln(1 - I/G)}",
     run=library_size, reference=_ref_library,
     wrong=lambda G, I, P: (lambda n: _out(clones_exact=n, clones=int(math.ceil(n - 1e-9))))(-math.log(1.0 - P) * I / G),
     rand=lambda rng: (lambda G: {"G": G, "I": _sig(G * _lu(rng, 1e-6, 0.05), 3), "P": rng.choice([0.9, 0.95, 0.99, 0.999, 0.5])})(_sig(_lu(rng, 1e6, 3e9), 3)),
     phrases=("Genomic library size", "Number of clones for a library"),
     examples=[_ex({"G": 3e9, "I": 40000.0, "P": 0.99}), _ex({"G": 4.6e6, "I": 5000.0, "P": 0.95})],
     level="intermediate",
     extra_q=[{"q": "Genomic library size: G = 3e9 bp, I = 40000 bp, P = 99%", "like": 0}])


def mutation_rate_p0(n0, n, Nt):
    _need(isinstance(n0, int) and isinstance(n, int) and n >= 1 and 1 <= n0 < n, "n0 must be a whole number between 1 and n - 1")
    _pos("the cells per culture Nt", Nt)
    p0 = n0 / n
    m = -math.log(p0)
    return _out(p0=p0, m=m, mutation_rate=m / Nt)


def _ref_p0(n0, n, Nt):
    from scipy.stats import poisson
    p0 = n0 / n
    m = _brentq(lambda x: float(poisson.pmf(0, x)) - p0, 1e-12, 50.0)
    return {"p0": p0, "m": m, "mutation_rate": m / Nt}


_add("cmb_mutation_rate_p0",
     name="Mutation rate by the fluctuation-test p0 method", topic="DNA and RNA / mutation",
     purpose="The expected number of mutations per culture m = -ln p0 from the fraction of parallel cultures without mutants (Luria-Delbruck p0 method) and the mutation rate m / Nt.",
     inputs=[_intin("n0", "number of cultures with no mutants", lo=1), _intin("n", "total number of cultures", lo=2),
             _numin("Nt", "final number of cells per culture")],
     outputs=[_o("p0", "number", "fraction of cultures without mutants"), _o("m", "number", "mean mutations per culture"),
              _o("mutation_rate", "number", "mutations per cell per generation (m / Nt)")],
     primary="m", conditions=["1 <= n0 < n", "Nt > 0", "Poisson number of mutational events per culture"],
     method="p0 = e^-m, so m = -ln(n0/n); mu = m / Nt", latex=r"p_0 = e^{-m},\quad \mu = \frac{-\ln p_0}{N_t}",
     run=mutation_rate_p0, reference=_ref_p0,
     wrong=lambda n0, n, Nt: (lambda m: _out(p0=n0 / n, m=m, mutation_rate=m / Nt))(1.0 - n0 / n),
     rand=lambda rng: (lambda n: {"n0": rng.randint(1, n - 1), "n": n, "Nt": _sig(_lu(rng, 1e6, 1e10), 3)})(rng.randint(10, 100)),
     phrases=("Mutation rate by the p0 method", "Fluctuation test p0 method"),
     examples=[_ex({"n0": 11, "n": 20, "Nt": 2e8}), _ex({"n0": 3, "n": 48, "Nt": 5.6e8})],
     level="advanced")


# ================================================================================================ PCR AND GENE EXPRESSION
def pcr_amplification(N0, E, n):
    _pos("the initial copies N0", N0)
    _frac("the efficiency E", E, lo_open=True, hi_open=False)
    _need(isinstance(n, int) and 0 <= n <= 60, "the number of cycles must be a whole number from 0 to 60")
    fold = (1.0 + E) ** n
    return _out(N=N0 * fold, fold=fold, log10_fold=n * math.log10(1.0 + E))


def _ref_pcr(N0, E, n):
    import numpy as np
    fold = float(np.prod(np.full(n, 1.0 + E))) if n > 0 else 1.0
    return {"N": N0 * fold, "fold": fold, "log10_fold": math.log10(fold)}


_add("cmb_pcr_amplification",
     name="PCR amplification after n cycles", topic="PCR and gene expression / PCR",
     purpose="The number of copies after n PCR cycles from N0 template copies at an amplification efficiency E (1 = perfect doubling).",
     inputs=[_numin("N0", "initial template copies"), _numin("E", "amplification efficiency per cycle (0 to 1, e.g. 95%)", pct=True),
             _intin("n", "number of cycles", lo=0, hi=60, words=("cycles",))],
     outputs=[_o("N", "number", "copies after n cycles"), _o("fold", "number", "fold amplification (1 + E)^n"),
              _o("log10_fold", "number", "log10 of the fold amplification")],
     primary="N", conditions=["N0 > 0", "0 < E <= 1", "0 <= n <= 60", "exponential phase (no plateau)"],
     method="N = N0 (1 + E)^n", latex=r"N = N_0(1 + E)^n",
     run=pcr_amplification, reference=_ref_pcr,
     wrong=lambda N0, E, n: _out(N=N0 * 2.0 ** n * E, fold=2.0 ** n * E, log10_fold=math.log10(2.0 ** n * E)),
     rand=lambda rng: {"N0": _sig(_lu(rng, 1.0, 1e6), 3), "E": _sig(rng.uniform(0.6, 0.98), 3), "n": rng.randint(1, 40)},
     phrases=("PCR amplification", "PCR copies after n cycles"),
     examples=[_ex({"N0": 100.0, "E": 1.0, "n": 30}), _ex({"N0": 1000.0, "E": 0.9, "n": 25})],
     level="beginner",
     extra_q=[{"q": "PCR amplification: N0 = 1000, E = 90%, n = 25 cycles", "like": 1}],
     weak="numpy product of the per-cycle factors")


def pcr_cycles_needed(N0, N, E):
    _pos("N0 and N", N0, N)
    _need(N > N0, "the target must exceed the starting copies")
    _frac("the efficiency E", E, lo_open=True, hi_open=False)
    n = math.log(N / N0) / math.log(1.0 + E)
    w = round(n)
    cycles = int(w) if abs(n - w) <= 1e-9 * max(1.0, n) else int(math.ceil(n))
    return _out(cycles_exact=n, cycles=cycles)


def _ref_pcr_cycles(N0, N, E):
    n = _brentq(lambda x: math.log(N0) + x * math.log1p(E) - math.log(N), 0.0, 1e4)
    w = round(n)
    return {"cycles_exact": n, "cycles": int(w) if abs(n - w) <= 1e-9 * max(1.0, n) else int(math.ceil(n))}


_add("cmb_pcr_cycles_needed",
     name="PCR cycles needed to reach a number of copies", topic="PCR and gene expression / PCR",
     purpose="The number of PCR cycles needed to amplify N0 copies to N at efficiency E (exact value and whole cycles, rounded up).",
     inputs=[_numin("N0", "initial template copies"), _numin("N", "target number of copies"),
             _numin("E", "amplification efficiency per cycle (0 to 1)", pct=True)],
     outputs=[_o("cycles_exact", "number", "cycles needed (real value)"), _o("cycles", "number", "whole cycles needed")],
     primary="cycles_exact", conditions=["N > N0 > 0", "0 < E <= 1"],
     method="n = ln(N/N0) / ln(1 + E)", latex=r"n = \frac{\ln(N/N_0)}{\ln(1 + E)}",
     run=pcr_cycles_needed, reference=_ref_pcr_cycles,
     wrong=lambda N0, N, E: (lambda n: _out(cycles_exact=n, cycles=int(math.ceil(n))))(math.log2(N / N0)),
     rand=lambda rng: (lambda n0: {"N0": n0, "N": _sig(n0 * _lu(rng, 10.0, 1e12), 3), "E": _sig(rng.uniform(0.6, 0.97), 3)})(_sig(_lu(rng, 1.0, 1e5), 3)),
     phrases=("PCR cycles needed", "Number of PCR cycles"),
     examples=[_ex({"N0": 100.0, "N": 1e10, "E": 0.95}), _ex({"N0": 10.0, "N": 1e9, "E": 0.85})],
     level="intermediate")


def qpcr_ddct(Ct_target_treated, Ct_ref_treated, Ct_target_control, Ct_ref_control):
    _fin(Ct_target_treated, Ct_ref_treated, Ct_target_control, Ct_ref_control)
    for v in (Ct_target_treated, Ct_ref_treated, Ct_target_control, Ct_ref_control):
        _need(0 < v <= 60, "Ct values lie between 0 and 60 cycles")
    dt = Ct_target_treated - Ct_ref_treated
    dc = Ct_target_control - Ct_ref_control
    dd = dt - dc
    return _out(dCt_treated=dt, dCt_control=dc, ddCt=dd, fold_change=2.0 ** (-dd), log2_fold_change=-dd)


def _ref_ddct(Ct_target_treated, Ct_ref_treated, Ct_target_control, Ct_ref_control):
    from fractions import Fraction as Fr
    dt = Fr(Ct_target_treated) - Fr(Ct_ref_treated)
    dc = Fr(Ct_target_control) - Fr(Ct_ref_control)
    dd = dt - dc
    return {"dCt_treated": float(dt), "dCt_control": float(dc), "ddCt": float(dd), "fold_change": math.exp(-float(dd) * math.log(2.0)),
            "log2_fold_change": -float(dd)}


_add("cmb_qpcr_delta_delta_ct",
     name="qPCR relative expression by the delta-delta Ct method", topic="PCR and gene expression / qPCR",
     purpose="The fold change in a target gene's expression between treated and control samples, normalized to a reference gene, by 2^-ddCt (100 % efficiency assumed).",
     inputs=[_numin("Ct_target_treated", "Ct of the target gene in the treated sample"),
             _numin("Ct_ref_treated", "Ct of the reference gene in the treated sample"),
             _numin("Ct_target_control", "Ct of the target gene in the control sample"),
             _numin("Ct_ref_control", "Ct of the reference gene in the control sample")],
     outputs=[_o("dCt_treated", "number", "Ct target - Ct reference, treated"), _o("dCt_control", "number", "Ct target - Ct reference, control"),
              _o("ddCt", "number", "dCt treated - dCt control"), _o("fold_change", "number", "relative expression 2^-ddCt"),
              _o("log2_fold_change", "number", "log2 fold change (-ddCt)")],
     primary="fold_change", conditions=["0 < Ct <= 60", "efficiencies of target and reference near 100 %"],
     method="ddCt = (Ct_t - Ct_r)_treated - (Ct_t - Ct_r)_control; fold = 2^-ddCt",
     latex=r"\Delta\Delta C_t = \Delta C_{t,treated} - \Delta C_{t,control},\quad \text{fold} = 2^{-\Delta\Delta C_t}",
     run=qpcr_ddct, reference=_ref_ddct,
     wrong=lambda Ct_target_treated, Ct_ref_treated, Ct_target_control, Ct_ref_control: (lambda o: dict(o, fold_change=_r(2.0 ** o["ddCt"]), log2_fold_change=o["ddCt"]))(
         qpcr_ddct(Ct_target_treated, Ct_ref_treated, Ct_target_control, Ct_ref_control)),
     rand=lambda rng: {"Ct_target_treated": _sig(rng.uniform(18.0, 34.0), 4), "Ct_ref_treated": _sig(rng.uniform(15.0, 25.0), 4),
                       "Ct_target_control": _sig(rng.uniform(18.0, 34.0), 4), "Ct_ref_control": _sig(rng.uniform(15.0, 25.0), 4)},
     phrases=("qPCR delta-delta Ct", "Delta-delta Ct fold change", "ddCt fold change"),
     examples=[_ex({"Ct_target_treated": 24.5, "Ct_ref_treated": 18.0, "Ct_target_control": 27.0, "Ct_ref_control": 18.2}),
               _ex({"Ct_target_treated": 30.1, "Ct_ref_treated": 20.0, "Ct_target_control": 28.0, "Ct_ref_control": 20.3})],
     level="beginner",
     weak="the same differences in exact arithmetic and 2^x written as exp(x ln 2)")


def qpcr_pfaffl(E_target, E_ref, Ct_target_control, Ct_target_sample, Ct_ref_control, Ct_ref_sample):
    _frac("E_target", E_target, hi_open=False)
    _frac("E_ref", E_ref, hi_open=False)
    for v in (Ct_target_control, Ct_target_sample, Ct_ref_control, Ct_ref_sample):
        _need(_isnum(v) and 0 < v <= 60, "Ct values lie between 0 and 60 cycles")
    dt = Ct_target_control - Ct_target_sample
    dr = Ct_ref_control - Ct_ref_sample
    ratio = (1.0 + E_target) ** dt / (1.0 + E_ref) ** dr
    return _out(ratio=ratio, log2_ratio=math.log2(ratio), dCt_target=dt, dCt_ref=dr)


def _ref_pfaffl(E_target, E_ref, Ct_target_control, Ct_target_sample, Ct_ref_control, Ct_ref_sample):
    dt = Ct_target_control - Ct_target_sample
    dr = Ct_ref_control - Ct_ref_sample
    lr = dt * math.log(1.0 + E_target) - dr * math.log(1.0 + E_ref)
    return {"ratio": math.exp(lr), "log2_ratio": lr / math.log(2.0), "dCt_target": dt, "dCt_ref": dr}


_add("cmb_qpcr_pfaffl_ratio",
     name="qPCR relative expression with efficiency correction (Pfaffl)", topic="PCR and gene expression / qPCR",
     purpose="The efficiency-corrected expression ratio of a target gene in a sample versus a control, normalized to a reference gene (Pfaffl 2001).",
     inputs=[_numin("E_target", "amplification efficiency of the target assay (0 to 1)", pct=True),
             _numin("E_ref", "amplification efficiency of the reference assay (0 to 1)", pct=True),
             _numin("Ct_target_control", "target Ct in the control"), _numin("Ct_target_sample", "target Ct in the sample"),
             _numin("Ct_ref_control", "reference Ct in the control"), _numin("Ct_ref_sample", "reference Ct in the sample")],
     outputs=[_o("ratio", "number", "expression ratio sample/control"), _o("log2_ratio", "number", "log2 of the ratio"),
              _o("dCt_target", "number", "Ct control - Ct sample for the target"), _o("dCt_ref", "number", "Ct control - Ct sample for the reference")],
     primary="ratio", conditions=["0 < E <= 1", "0 < Ct <= 60"],
     method="ratio = (1 + E_t)^(Ct_t,control - Ct_t,sample) / (1 + E_r)^(Ct_r,control - Ct_r,sample)",
     latex=r"\text{ratio} = \frac{(1 + E_t)^{\Delta C_{t,target}}}{(1 + E_r)^{\Delta C_{t,ref}}}",
     run=qpcr_pfaffl, reference=_ref_pfaffl,
     wrong=lambda E_target, E_ref, Ct_target_control, Ct_target_sample, Ct_ref_control, Ct_ref_sample: qpcr_pfaffl(
         E_target, E_ref, Ct_target_sample, Ct_target_control, Ct_ref_sample, Ct_ref_control),
     rand=lambda rng: {"E_target": _sig(rng.uniform(0.8, 1.0), 3), "E_ref": _sig(rng.uniform(0.8, 1.0), 3),
                       "Ct_target_control": _sig(rng.uniform(20.0, 32.0), 4), "Ct_target_sample": _sig(rng.uniform(20.0, 32.0), 4),
                       "Ct_ref_control": _sig(rng.uniform(15.0, 24.0), 4), "Ct_ref_sample": _sig(rng.uniform(15.0, 24.0), 4)},
     phrases=("Pfaffl expression ratio", "Efficiency-corrected qPCR ratio"),
     examples=[_ex({"E_target": 0.95, "E_ref": 0.9, "Ct_target_control": 26.0, "Ct_target_sample": 23.5, "Ct_ref_control": 18.0, "Ct_ref_sample": 18.3}),
               _ex({"E_target": 1.0, "E_ref": 1.0, "Ct_target_control": 25.0, "Ct_target_sample": 27.0, "Ct_ref_control": 17.0, "Ct_ref_sample": 17.0})],
     level="intermediate",
     extra_q=[{"q": "Pfaffl expression ratio: E_target = 95%, E_ref = 90%, Ct_target_control = 26, Ct_target_sample = 23.5, Ct_ref_control = 18, Ct_ref_sample = 18.3", "like": 0}],
     weak="the same ratio evaluated in logarithms")


def qpcr_standard_curve(quantity, Ct):
    _need(len(quantity) == len(Ct) and len(Ct) >= 3, "give at least 3 standards with one Ct each")
    for v in quantity:
        _need(_isnum(v) and v > 0, "standard quantities must be positive")
    x = [math.log10(v) for v in quantity]
    m, b, r2 = _linfit(x, Ct)
    _need(m < 0, "Ct must fall as the quantity rises (a negative slope)")
    amp = 10.0 ** (-1.0 / m)
    return _out(slope=m, intercept=b, r2=r2, efficiency=amp - 1.0, amplification_factor=amp)


def _ref_std_curve(quantity, Ct):
    import numpy as np
    m, b, r2 = _linregress(np.log10(quantity), Ct)
    return {"slope": m, "intercept": b, "r2": r2, "efficiency": 10.0 ** (-1.0 / m) - 1.0, "amplification_factor": 10.0 ** (-1.0 / m)}


def _rand_std(rng):
    n = rng.randint(4, 7)
    top = _lu(rng, 1e5, 1e8)
    df = rng.choice([10.0, 5.0, 4.0])
    q = [_sig(top / df ** k, 4) for k in range(n)]
    eff = rng.uniform(0.85, 1.05)
    m = -1.0 / math.log10(1.0 + eff)
    b = rng.uniform(35.0, 42.0)
    return {"quantity": q, "Ct": [_sig(b + m * math.log10(v) + rng.gauss(0.0, 0.08), 4) for v in q]}


_add("cmb_qpcr_standard_curve",
     name="qPCR standard curve: slope, intercept and efficiency", topic="PCR and gene expression / qPCR",
     purpose="The least-squares standard curve Ct = m log10(quantity) + b from a dilution series, its r^2 and the amplification efficiency 10^(-1/m) - 1.",
     inputs=[_vecin("quantity", "starting quantities of the standards (copies or ng)", 3, 30), _vecin("Ct", "measured Ct of each standard", 3, 30)],
     outputs=[_o("slope", "number", "slope m (about -3.32 at 100 %)"), _o("intercept", "number", "intercept b (Ct of one unit)"),
              _o("r2", "number", "coefficient of determination"), _o("efficiency", "number", "efficiency 10^(-1/m) - 1"),
              _o("amplification_factor", "number", "amplification per cycle 10^(-1/m)")],
     primary="efficiency", conditions=["at least 3 standards", "quantities > 0", "negative slope"],
     method="least squares Ct = m log10 q + b; E = 10^(-1/m) - 1", latex=r"C_t = m\log_{10}q + b,\quad E = 10^{-1/m} - 1",
     run=qpcr_standard_curve, reference=_ref_std_curve,
     wrong=lambda quantity, Ct: (lambda o: dict(o, efficiency=o["amplification_factor"], amplification_factor=_r(o["amplification_factor"] + 1.0)))(qpcr_standard_curve(quantity, Ct)),
     rand=_rand_std,
     phrases=("qPCR standard curve", "Standard curve efficiency"),
     examples=[_ex({"quantity": [1e6, 1e5, 1e4, 1e3, 1e2], "Ct": [15.1, 18.5, 21.8, 25.2, 28.5]}),
               _ex({"quantity": [100.0, 20.0, 4.0, 0.8], "Ct": [20.0, 22.4, 24.7, 27.1]})],
     level="intermediate")


def qpcr_quantify(slope, intercept, Ct):
    _need(_isnum(slope) and slope < 0, "the slope of a standard curve is negative")
    _fin(intercept, Ct)
    q = 10.0 ** ((Ct - intercept) / slope)
    return _out(quantity=q, log10_quantity=(Ct - intercept) / slope)


def _ref_quantify(slope, intercept, Ct):
    x = _brentq(lambda lx: slope * lx + intercept - Ct, -60.0, 60.0)
    return {"quantity": 10.0 ** x, "log10_quantity": x}


_add("cmb_qpcr_quantify",
     name="Quantity from a Ct and a standard curve", topic="PCR and gene expression / qPCR",
     purpose="The starting quantity of an unknown from its Ct and the standard curve Ct = m log10 q + b.",
     inputs=[_numin("slope", "standard-curve slope m (negative)"), _numin("intercept", "standard-curve intercept b"),
             _numin("Ct", "Ct of the unknown")],
     outputs=[_o("quantity", "number", "starting quantity (unit of the standards)"), _o("log10_quantity", "number", "log10 of the quantity")],
     primary="quantity", conditions=["slope < 0", "Ct inside the range of the standards"],
     method="q = 10^((Ct - b)/m)", latex=r"q = 10^{(C_t - b)/m}",
     run=qpcr_quantify, reference=_ref_quantify,
     wrong=lambda slope, intercept, Ct: _out(quantity=10.0 ** ((intercept - Ct) / slope), log10_quantity=(intercept - Ct) / slope),
     rand=lambda rng: {"slope": _sig(rng.uniform(-3.8, -3.1), 4), "intercept": _sig(rng.uniform(34.0, 42.0), 4), "Ct": _sig(rng.uniform(14.0, 33.0), 4)},
     phrases=("Quantity from a standard curve", "qPCR quantification"),
     examples=[_ex({"slope": -3.32, "intercept": 38.0, "Ct": 24.7}), _ex({"slope": -3.45, "intercept": 36.2, "Ct": 30.0})],
     level="intermediate")


def transcription_time(n, rate, k_init=None):
    _need(isinstance(n, int) and n >= 1, "the gene length must be a whole number of nucleotides")
    _pos("the elongation rate", rate)
    t = n / rate
    out = {"t": t, "t_min": t / MINUTE}
    if k_init is not None:
        _pos("the initiation rate", k_init)
        out["polymerases"] = k_init * t
        out["spacing_nt"] = rate / k_init
    return _out(**out)


def _ref_transcription_time(n, rate, k_init=None):
    from fractions import Fraction as Fr
    t = Fr(n) / Fr(rate)
    out = {"t": float(t), "t_min": float(t / 60)}
    if k_init is not None:
        out["polymerases"] = float(Fr(k_init) * t)
        out["spacing_nt"] = float(Fr(rate) / Fr(k_init))
    return out


_add("cmb_transcription_time",
     name="Time to transcribe a gene", topic="PCR and gene expression / transcription",
     purpose="The time for RNA polymerase to transcribe a gene of n nucleotides at an elongation rate, and, with an initiation rate, the number of polymerases on the gene and their spacing.",
     inputs=[_intin("n", "gene (transcript) length in nucleotides", lo=1, words=("nt", "bp", "nucleotides")),
             _numin("rate", "elongation rate in nucleotides per second", words=("nt/s", "nucleotides/s", "bp/s")),
             _phys("k_init", "frequency", "initiation rate (optional)", required=False, show=("1/min", MINUTE))],
     outputs=[_o("t", "time", "transcription time"), _o("t_min", "number", "time in minutes"),
              _o("polymerases", "number", "polymerases on the gene at steady state (when k_init is given)"),
              _o("spacing_nt", "number", "mean spacing between polymerases in nt (when k_init is given)")],
     primary="t", conditions=["n >= 1", "rate > 0", "constant elongation rate"],
     method="t = n / rate; polymerases = k_init t; spacing = rate / k_init", latex=r"t = \frac{n}{v},\quad N_{pol} = k_{init}\,t",
     run=transcription_time, reference=_ref_transcription_time,
     wrong=lambda n, rate, k_init=None: _out(**dict({"t": n * rate, "t_min": n * rate / MINUTE},
                                                    **({"polymerases": k_init * n * rate, "spacing_nt": k_init * rate} if k_init is not None else {}))),
     rand=lambda rng: dict({"n": rng.randint(500, 200000), "rate": _sig(rng.uniform(10.0, 80.0), 3)},
                           **({"k_init": _sig(_lu(rng, 0.005, 0.5), 3)} if rng.random() < 0.5 else {})),
     phrases=("Transcription time", "Time to transcribe a gene"),
     examples=[_ex({"n": 20000, "rate": 40.0}), _ex({"n": 6000, "rate": 50.0, "k_init": 0.1})],
     level="beginner",
     extra_q=[{"q": "Transcription time: n = 20000 nt, rate = 40 nt/s", "like": 0}],
     weak="the same division in exact rational arithmetic")


def translation_time(n_aa, rate, k_init=None):
    _need(isinstance(n_aa, int) and n_aa >= 1, "the protein length must be a whole number of amino acids")
    _pos("the elongation rate", rate)
    t = n_aa / rate
    out = {"t": t, "t_min": t / MINUTE, "coding_nt": 3 * (n_aa + 1)}
    if k_init is not None:
        _pos("the initiation rate", k_init)
        out["ribosomes"] = k_init * t
        out["spacing_codons"] = rate / k_init
    return _out(**out)


def _ref_translation_time(n_aa, rate, k_init=None):
    from fractions import Fraction as Fr
    t = Fr(n_aa) / Fr(rate)
    out = {"t": float(t), "t_min": float(t / 60), "coding_nt": 3 * n_aa + 3}
    if k_init is not None:
        out["ribosomes"] = float(Fr(k_init) * t)
        out["spacing_codons"] = float(Fr(rate) / Fr(k_init))
    return out


_add("cmb_translation_time",
     name="Time to translate a protein", topic="PCR and gene expression / translation",
     purpose="The time for a ribosome to synthesise a protein of n_aa amino acids at an elongation rate, the coding length with the stop codon, and, with an initiation rate, the ribosomes per mRNA and their spacing.",
     inputs=[_intin("n_aa", "protein length in amino acids", lo=1, words=("aa", "amino acids", "residues")),
             _numin("rate", "elongation rate in amino acids per second", words=("aa/s", "codons/s", "amino acids/s")),
             _phys("k_init", "frequency", "initiation rate per mRNA (optional)", required=False, show=("1/min", MINUTE))],
     outputs=[_o("t", "time", "translation time"), _o("t_min", "number", "time in minutes"),
              _o("coding_nt", "number", "coding sequence length in nt including the stop codon"),
              _o("ribosomes", "number", "ribosomes per mRNA at steady state (when k_init is given)"),
              _o("spacing_codons", "number", "mean spacing between ribosomes in codons (when k_init is given)")],
     primary="t", conditions=["n_aa >= 1", "rate > 0"],
     method="t = n_aa / rate; coding length = 3 (n_aa + 1); ribosomes = k_init t", latex=r"t = \frac{n_{aa}}{v},\quad N_{rib} = k_{init}\,t",
     run=translation_time, reference=_ref_translation_time,
     wrong=lambda n_aa, rate, k_init=None: (lambda o: dict(o, t=_r(3.0 * n_aa / rate), t_min=_r(3.0 * n_aa / rate / MINUTE)))(translation_time(n_aa, rate, k_init)),
     rand=lambda rng: dict({"n_aa": rng.randint(50, 3000), "rate": _sig(rng.uniform(2.0, 20.0), 3)},
                           **({"k_init": _sig(_lu(rng, 0.01, 1.0), 3)} if rng.random() < 0.5 else {})),
     phrases=("Translation time", "Time to translate a protein"),
     examples=[_ex({"n_aa": 300, "rate": 6.0}), _ex({"n_aa": 450, "rate": 15.0, "k_init": 0.2})],
     level="beginner",
     extra_q=[{"q": "Translation time: n_aa = 300 aa, rate = 6 aa/s", "like": 0}],
     weak="the same division in exact rational arithmetic")


def expression_steady_state(k_tx, delta_m, k_tl, delta_p):
    _pos("all rates", k_tx, delta_m, k_tl, delta_p)
    m = k_tx / delta_m
    p = k_tl * m / delta_p
    return _out(m_ss=m, p_ss=p, mRNA_half_life=LN2 / delta_m, protein_half_life=LN2 / delta_p,
                mRNA_half_life_min=LN2 / delta_m / MINUTE, protein_half_life_h=LN2 / delta_p / HOUR)


def _ref_expression_ss(k_tx, delta_m, k_tl, delta_p):
    import numpy as np
    A = np.array([[-delta_m, 0.0], [k_tl, -delta_p]])
    x = np.linalg.solve(A, np.array([-k_tx, 0.0]))
    hm = _brentq(lambda t: math.exp(-delta_m * t) - 0.5, 0.0, 10.0 / delta_m)
    hp = _brentq(lambda t: math.exp(-delta_p * t) - 0.5, 0.0, 10.0 / delta_p)
    return {"m_ss": float(x[0]), "p_ss": float(x[1]), "mRNA_half_life": hm, "protein_half_life": hp,
            "mRNA_half_life_min": hm / 60.0, "protein_half_life_h": hp / 3600.0}


_add("cmb_expression_steady_state",
     name="Steady-state mRNA and protein numbers", topic="PCR and gene expression / gene expression",
     purpose="The steady-state numbers of mRNA (k_tx/delta_m) and protein (k_tl m/delta_p) per cell for constant transcription, translation and first-order decay (dilution included), with the half-lives.",
     inputs=[_phys("k_tx", "frequency", "transcription rate (mRNA made per cell per unit time)"),
             _phys("delta_m", "first_order_rate_constant", "mRNA decay rate constant"),
             _phys("k_tl", "frequency", "translation rate (proteins per mRNA per unit time)"),
             _phys("delta_p", "first_order_rate_constant", "protein decay (plus dilution) rate constant")],
     outputs=[_o("m_ss", "number", "steady-state mRNA per cell"), _o("p_ss", "number", "steady-state protein per cell"),
              _o("mRNA_half_life", "time", "mRNA half-life"), _o("protein_half_life", "time", "protein half-life"),
              _o("mRNA_half_life_min", "number", "mRNA half-life in minutes"), _o("protein_half_life_h", "number", "protein half-life in hours")],
     primary="p_ss", conditions=["all rates > 0", "constant rates, first-order decay"],
     method="dm/dt = k_tx - delta_m m, dp/dt = k_tl m - delta_p p; at steady state m = k_tx/delta_m, p = k_tl k_tx/(delta_m delta_p)",
     latex=r"m^* = \frac{k_{tx}}{\delta_m},\quad p^* = \frac{k_{tl}\,k_{tx}}{\delta_m\,\delta_p}",
     run=expression_steady_state, reference=_ref_expression_ss,
     wrong=lambda k_tx, delta_m, k_tl, delta_p: (lambda o: dict(o, p_ss=_r(k_tl / delta_p)))(expression_steady_state(k_tx, delta_m, k_tl, delta_p)),
     rand=lambda rng: {"k_tx": _sig(_lu(rng, 1e-4, 1e-1), 3), "delta_m": _sig(_lu(rng, 1e-5, 5e-3), 3),
                       "k_tl": _sig(_lu(rng, 1e-3, 1.0), 3), "delta_p": _sig(_lu(rng, 1e-6, 1e-3), 3)},
     phrases=("Gene expression steady state", "Steady-state mRNA and protein"),
     examples=[_ex({"k_tx": 0.01, "delta_m": 0.002, "k_tl": 0.05, "delta_p": 0.0001}),
               _ex({"k_tx": 1.0 / 60.0, "delta_m": 1.0 / 300.0, "k_tl": 0.2, "delta_p": 2e-4})],
     level="intermediate",
     extra_q=[{"q": "Gene expression steady state: k_tx = 36 1/h, delta_m = 7.2 1/h, k_tl = 180 1/h, delta_p = 0.36 1/h", "like": 0}])


def induction_dynamics(k_tx, delta_m, k_tl, delta_p, t):
    _pos("all rates", k_tx, delta_m, k_tl, delta_p)
    _nonneg("the time t", t)
    m = k_tx / delta_m * (1.0 - math.exp(-delta_m * t))
    d = delta_p - delta_m
    if abs(d) <= 1e-9 * max(delta_p, delta_m):
        cross = t * math.exp(-delta_m * t)
    else:
        cross = (math.exp(-delta_m * t) - math.exp(-delta_p * t)) / d
    p = k_tl * k_tx / delta_m * ((1.0 - math.exp(-delta_p * t)) / delta_p - cross)
    p_ss = k_tl * k_tx / (delta_m * delta_p)
    return _out(m=m, p=p, fraction_of_steady_state=p / p_ss)


def _ref_induction(k_tx, delta_m, k_tl, delta_p, t):
    if t == 0:
        return {"m": 0.0, "p": 0.0, "fraction_of_steady_state": 0.0}
    m, p = _ivp(lambda tt, y: [k_tx - delta_m * y[0], k_tl * y[0] - delta_p * y[1]], t, [0.0, 0.0])
    return {"m": m, "p": p, "fraction_of_steady_state": p * delta_m * delta_p / (k_tl * k_tx)}


def _rand_induction(rng):
    dm = _sig(_lu(rng, 1e-4, 5e-3), 3)
    dp = _sig(dm * rng.choice([rng.uniform(0.02, 0.8), rng.uniform(1.3, 4.0)]), 3)
    return {"k_tx": _sig(_lu(rng, 1e-3, 1e-1), 3), "delta_m": dm, "k_tl": _sig(_lu(rng, 1e-3, 1.0), 3), "delta_p": dp,
            "t": _sig(rng.uniform(0.1, 4.0) / min(dm, dp), 3)}


_add("cmb_induction_dynamics",
     name="mRNA and protein after switching a gene on", topic="PCR and gene expression / gene expression",
     purpose="The mRNA and protein numbers a time t after a gene is switched on from zero (constant transcription and translation, first-order decay), and the protein's fraction of its steady state.",
     inputs=[_phys("k_tx", "frequency", "transcription rate"), _phys("delta_m", "first_order_rate_constant", "mRNA decay rate constant"),
             _phys("k_tl", "frequency", "translation rate per mRNA"), _phys("delta_p", "first_order_rate_constant", "protein decay (plus dilution) rate constant"),
             _phys("t", "time", "time since induction", show=_HOURS)],
     outputs=[_o("m", "number", "mRNA per cell at time t"), _o("p", "number", "protein per cell at time t"),
              _o("fraction_of_steady_state", "number", "p / p_ss")],
     primary="p", conditions=["all rates > 0", "t >= 0", "m(0) = p(0) = 0"],
     method="m = (k_tx/delta_m)(1 - e^(-delta_m t)); p = (k_tl k_tx/delta_m)[(1 - e^(-delta_p t))/delta_p - (e^(-delta_m t) - e^(-delta_p t))/(delta_p - delta_m)] (t e^(-delta t) when equal)",
     latex=r"p(t) = \frac{k_{tl}k_{tx}}{\delta_m}\left[\frac{1 - e^{-\delta_p t}}{\delta_p} - \frac{e^{-\delta_m t} - e^{-\delta_p t}}{\delta_p - \delta_m}\right]",
     run=induction_dynamics, reference=_ref_induction,
     wrong=lambda k_tx, delta_m, k_tl, delta_p, t: (lambda ps: _out(m=k_tx / delta_m * (1.0 - math.exp(-delta_m * t)), p=ps * (1.0 - math.exp(-delta_p * t)),
                                                                   fraction_of_steady_state=1.0 - math.exp(-delta_p * t)))(k_tl * k_tx / (delta_m * delta_p)),
     rand=_rand_induction,
     phrases=("Gene induction dynamics", "Protein after induction"),
     examples=[_ex({"k_tx": 0.01, "delta_m": 0.002, "k_tl": 0.05, "delta_p": 0.0001, "t": 7200.0}),
               _ex({"k_tx": 0.02, "delta_m": 0.001, "k_tl": 0.1, "delta_p": 0.003, "t": 1800.0})],
     level="advanced")


def mrna_half_life_fit(t, level):
    _need(len(t) == len(level) and len(t) >= 3, "give at least 3 times and one level per time")
    for v in level:
        _need(_isnum(v) and v > 0, "levels must be positive")
    b, a, r2 = _linfit(t, [math.log(v) for v in level])
    _need(b < 0, "the levels do not decay: no half-life can be fitted")
    k = -b
    return _out(k_per_min=k, half_life_min=LN2 / k, initial_fit=math.exp(a), r2=r2)


def _ref_half_life_fit(t, level):
    import numpy as np
    s, i, r2 = _linregress(t, np.log(level))
    return {"k_per_min": -s, "half_life_min": math.log(2.0) / -s, "initial_fit": math.exp(i), "r2": r2}


def _rand_half_life(rng):
    n = rng.randint(4, 8)
    k = rng.uniform(0.005, 0.2)
    ts = sorted(set(round(rng.uniform(0.0, 3.0 / k), 0) for _ in range(n + 4)))[:n]
    return {"t": ts, "level": [_sig(100.0 * math.exp(-k * x) * math.exp(rng.gauss(0.0, 0.04)), 4) for x in ts]}


_add("cmb_mrna_half_life_fit",
     name="mRNA half-life fitted to a decay time course", topic="PCR and gene expression / gene expression",
     purpose="The decay rate constant and half-life of an mRNA (or protein) from levels measured after transcription is stopped, by a least-squares line through ln(level) against time.",
     inputs=[_vecin("t", "times after the shut-off in minutes", 3, 100), _vecin("level", "measured levels (relative units, positive)", 3, 100)],
     outputs=[_o("k_per_min", "number", "decay rate constant per minute"), _o("half_life_min", "number", "half-life in minutes"),
              _o("initial_fit", "number", "fitted level at t = 0"), _o("r2", "number", "coefficient of determination of the log-linear fit")],
     primary="half_life_min", conditions=["at least 3 points", "levels > 0 and decaying", "first-order decay"],
     method="least squares ln(level) = ln(L0) - k t; t1/2 = ln 2 / k", latex=r"\ln L = \ln L_0 - kt,\quad t_{1/2} = \frac{\ln 2}{k}",
     run=mrna_half_life_fit, reference=_ref_half_life_fit,
     wrong=lambda t, level: (lambda f: _out(k_per_min=-f[0], half_life_min=LN2 / -f[0], initial_fit=10.0 ** f[1], r2=f[2]))(_linfit(t, [math.log10(v) for v in level])),
     rand=_rand_half_life,
     phrases=("mRNA half-life fit", "Half-life from a decay time course"),
     examples=[_ex({"t": [0.0, 30.0, 60.0, 90.0, 120.0], "level": [100.0, 70.0, 50.5, 35.2, 24.8]}),
               _ex({"t": [0.0, 10.0, 20.0, 40.0], "level": [1.0, 0.62, 0.4, 0.15]})],
     level="intermediate")


# ================================================================================================ ENZYME KINETICS AND BINDING
def _re_scheme(S, I, Km, Ki, Kii):
    """Reference helper: rapid-equilibrium enzyme species (E, ES, EI, ESI) per unit total enzyme by a numpy linear
    solve; Ki (EI) or Kii (ESI) may be None (no such complex). Returns the ES fraction."""
    import numpy as np
    A = np.zeros((4, 4))
    b = np.zeros(4)
    A[0] = [1.0, 1.0, 1.0, 1.0]
    b[0] = 1.0
    A[1] = [S / Km, -1.0, 0.0, 0.0]                    # ES = E S / Km
    if Ki is None:
        A[2] = [0.0, 0.0, 1.0, 0.0]
    else:
        A[2] = [I / Ki, 0.0, -1.0, 0.0]                # EI = E I / Ki
    if Kii is None:
        A[3] = [0.0, 0.0, 0.0, 1.0]
    else:
        A[3] = [0.0, I / Kii, 0.0, -1.0]               # ESI = ES I / Kii
    x = np.linalg.solve(A, b)
    return float(x[1])


def michaelis_menten(Vmax, Km, S):
    _pos("Vmax and Km", Vmax, Km)
    _nonneg("the substrate concentration S", S)
    v = Vmax * S / (Km + S)
    return _out(v=v, v_uM_min=v * 1e3 * MINUTE, fraction=S / (Km + S))


def _ref_mm(Vmax, Km, S):
    f = _re_scheme(S, 0.0, Km, None, None)
    return {"v": Vmax * f, "v_uM_min": Vmax * f * 6e4, "fraction": f}


_add("cmb_michaelis_menten_rate",
     name="Michaelis-Menten reaction rate", topic="enzyme kinetics / Michaelis-Menten",
     purpose="The initial rate of an enzyme-catalysed reaction at substrate concentration S from Vmax and Km, and the fraction of Vmax.",
     inputs=[_phys("Vmax", "reaction_rate", "maximum rate"), _phys("Km", "molar_concentration", "Michaelis constant", show=_UMOLAR),
             _phys("S", "molar_concentration", "substrate concentration", show=_UMOLAR)],
     outputs=[_o("v", "reaction_rate", "initial rate"), _o("v_uM_min", "number", "rate in uM/min"), _o("fraction", "number", "v / Vmax")],
     primary="v", conditions=["Vmax, Km > 0", "S >= 0", "initial-rate conditions, S >> E"],
     method="v = Vmax S / (Km + S)", latex=r"v = \frac{V_{max}[S]}{K_m + [S]}",
     run=michaelis_menten, reference=_ref_mm,
     wrong=lambda Vmax, Km, S: _out(v=Vmax * S / Km, v_uM_min=Vmax * S / Km * 6e4, fraction=S / Km),
     rand=lambda rng: {"Vmax": _sig(_lu(rng, 1e-7, 1e-3), 3), "Km": _sig(_lu(rng, 1e-4, 10.0), 3), "S": _sig(_lu(rng, 1e-4, 50.0), 3)},
     phrases=("Michaelis-Menten rate", "Michaelis-Menten velocity", "Enzyme reaction rate"),
     examples=[_ex({"Vmax": 1e-3 / 60.0 * 100.0, "Km": 0.05, "S": 0.1}), _ex({"Vmax": 2e-6, "Km": 2.0, "S": 0.5})],
     level="beginner",
     extra_q=[{"q": "Michaelis-Menten rate: Vmax = 100 uM/min, Km = 50 uM, S = 0.1 mM", "like": 0}],
     weak="the ES fraction of the rapid-equilibrium scheme by a numpy linear solve (the same hyperbola)")


def substrate_for_fraction(Km, f):
    _pos("Km", Km)
    _frac("the fraction of Vmax f", f)
    S = Km * f / (1.0 - f)
    return _out(S=S, S_uM=S * 1e3, S_over_Km=f / (1.0 - f))


def _ref_substrate_fraction(Km, f):
    S = _brentq(lambda s: s / (Km + s) - f, 0.0, Km * (f / (1.0 - f)) * 10.0 + Km)
    return {"S": S, "S_uM": S * 1e3, "S_over_Km": S / Km}


_add("cmb_substrate_for_fraction_of_vmax",
     name="Substrate concentration for a fraction of Vmax", topic="enzyme kinetics / Michaelis-Menten",
     purpose="The substrate concentration at which a Michaelis-Menten enzyme runs at a fraction f of Vmax (e.g. 90 % needs 9 Km).",
     inputs=[_phys("Km", "molar_concentration", "Michaelis constant", show=_UMOLAR), _numin("f", "fraction of Vmax (0 to 1)", pct=True)],
     outputs=[_o("S", "molar_concentration", "required substrate concentration"), _o("S_uM", "number", "S in uM"),
              _o("S_over_Km", "number", "S as a multiple of Km")],
     primary="S", conditions=["Km > 0", "0 < f < 1"],
     method="S = Km f / (1 - f)", latex=r"[S] = K_m\frac{f}{1 - f}",
     run=substrate_for_fraction, reference=_ref_substrate_fraction,
     wrong=lambda Km, f: _out(S=Km * f, S_uM=Km * f * 1e3, S_over_Km=f),
     rand=lambda rng: {"Km": _sig(_lu(rng, 1e-4, 10.0), 3), "f": _sig(rng.uniform(0.05, 0.99), 3)},
     phrases=("Substrate for a fraction of Vmax", "Substrate concentration for a fraction of Vmax"),
     examples=[_ex({"Km": 0.05, "f": 0.9}), _ex({"Km": 2.0, "f": 0.25})],
     level="beginner",
     extra_q=[{"q": "Substrate for a fraction of Vmax: Km = 50 uM, f = 90%", "like": 0}])


def kcat_efficiency(Vmax, Et, Km):
    _pos("Vmax, Et and Km", Vmax, Et, Km)
    kcat = Vmax / Et
    eff = kcat / Km
    return _out(kcat=kcat, kcat_per_s=kcat, efficiency=eff, efficiency_per_M_s=eff * 1e3, turnover_time=1.0 / kcat)


def _ref_kcat(Vmax, Et, Km):
    from fractions import Fraction as Fr
    kcat = Fr(Vmax) / Fr(Et)
    eff = kcat / Fr(Km)
    return {"kcat": float(kcat), "kcat_per_s": float(kcat), "efficiency": float(eff), "efficiency_per_M_s": float(eff * 1000),
            "turnover_time": float(1 / kcat)}


_add("cmb_kcat_catalytic_efficiency",
     name="Turnover number kcat and catalytic efficiency kcat/Km", topic="enzyme kinetics / Michaelis-Menten",
     purpose="The turnover number kcat = Vmax/[E]t, the catalytic efficiency kcat/Km (compare with the diffusion limit of about 1e8-1e9 per M per s) and the time per turnover.",
     inputs=[_phys("Vmax", "reaction_rate", "maximum rate"), _phys("Et", "molar_concentration", "total enzyme concentration", show=_NMOLAR),
             _phys("Km", "molar_concentration", "Michaelis constant", show=_UMOLAR)],
     outputs=[_o("kcat", "first_order_rate_constant", "turnover number"), _o("kcat_per_s", "number", "kcat in 1/s"),
              _o("efficiency", "second_order_rate_constant", "catalytic efficiency kcat/Km"),
              _o("efficiency_per_M_s", "number", "kcat/Km in 1/(M s)"), _o("turnover_time", "time", "time per catalytic event 1/kcat")],
     primary="kcat", conditions=["Vmax, Et, Km > 0"], method="kcat = Vmax / [E]t; efficiency = kcat / Km",
     latex=r"k_{cat} = \frac{V_{max}}{[E]_t},\quad \eta = \frac{k_{cat}}{K_m}",
     run=kcat_efficiency, reference=_ref_kcat,
     wrong=lambda Vmax, Et, Km: (lambda k: _out(kcat=k, kcat_per_s=k, efficiency=k * Km, efficiency_per_M_s=k * Km * 1e3, turnover_time=1.0 / k))(Vmax / Et),
     rand=lambda rng: {"Vmax": _sig(_lu(rng, 1e-7, 1e-3), 3), "Et": _sig(_lu(rng, 1e-7, 1e-3), 3), "Km": _sig(_lu(rng, 1e-4, 10.0), 3)},
     phrases=("kcat and catalytic efficiency", "Catalytic efficiency", "Turnover number"),
     examples=[_ex({"Vmax": 1e-3, "Et": 1e-5, "Km": 0.025}), _ex({"Vmax": 5e-6, "Et": 2e-6, "Km": 1.5})],
     level="intermediate",
     extra_q=[{"q": "Catalytic efficiency: Vmax = 60 uM/min, Et = 10 nM, Km = 25 uM", "like": 0}],
     weak="the same definitions in exact rational arithmetic")


def lineweaver_burk(S, v):
    _need(len(S) == len(v) and len(S) >= 3, "give at least 3 substrate concentrations with one rate each")
    for a in S:
        _need(_isnum(a) and a > 0, "substrate concentrations must be positive")
    for a in v:
        _need(_isnum(a) and a > 0, "rates must be positive")
    x = [1.0 / a for a in S]
    y = [1.0 / a for a in v]
    m, b, r2 = _linfit(x, y)
    _need(b > 0 and m > 0, "the double-reciprocal line must have a positive slope and intercept")
    Vmax = 1.0 / b
    return _out(Km=m / b, Vmax=Vmax, slope=m, intercept=b, r2=r2)


def _ref_lb(S, v):
    import numpy as np
    m, b = np.polyfit(1.0 / np.array(S, dtype=float), 1.0 / np.array(v, dtype=float), 1)
    r = np.corrcoef(1.0 / np.array(S, dtype=float), 1.0 / np.array(v, dtype=float))[0, 1]
    return {"Km": float(m / b), "Vmax": float(1.0 / b), "slope": float(m), "intercept": float(b), "r2": float(r * r)}


def _rand_mm_data(rng, noise):
    Km = _lu(rng, 0.5, 50.0)
    V = _lu(rng, 1.0, 100.0)
    n = rng.randint(5, 9)
    S = sorted(set(_sig(Km * _lu(rng, 0.1, 10.0), 3) for _ in range(n + 3)))[:n]
    return {"S": S, "v": [_sig(V * s / (Km + s) * (1.0 + rng.gauss(0.0, noise)), 4) for s in S]}


_add("cmb_lineweaver_burk_fit",
     name="Km and Vmax from a Lineweaver-Burk plot", topic="enzyme kinetics / Michaelis-Menten",
     purpose="Km and Vmax from initial-rate data by the double-reciprocal (Lineweaver-Burk) least-squares line 1/v = (Km/Vmax)(1/S) + 1/Vmax.",
     inputs=[_vecin("S", "substrate concentrations (any one unit, e.g. uM)", 3, 50), _vecin("v", "initial rates (any one unit)", 3, 50)],
     outputs=[_o("Km", "number", "Michaelis constant (unit of S)"), _o("Vmax", "number", "maximum rate (unit of v)"),
              _o("slope", "number", "slope Km/Vmax"), _o("intercept", "number", "intercept 1/Vmax"), _o("r2", "number", "r^2 of the double-reciprocal line")],
     primary="Km", conditions=["at least 3 points", "S, v > 0", "positive slope and intercept"],
     method="least squares 1/v = a + b/S; Vmax = 1/a; Km = b/a", latex=r"\frac{1}{v} = \frac{K_m}{V_{max}}\frac{1}{[S]} + \frac{1}{V_{max}}",
     run=lineweaver_burk, reference=_ref_lb,
     wrong=lambda S, v: (lambda o: dict(o, Km=o["slope"]))(lineweaver_burk(S, v)),
     rand=lambda rng: _rand_mm_data(rng, 0.02),
     phrases=("Lineweaver-Burk fit", "Lineweaver-Burk plot", "Double-reciprocal fit"),
     examples=[_ex({"S": [1.0, 2.0, 5.0, 10.0, 20.0, 50.0], "v": [16.7, 28.6, 50.0, 66.7, 80.0, 90.9]}),
               _ex({"S": [0.5, 1.0, 2.0, 4.0], "v": [2.1, 3.4, 5.1, 6.6]})],
     level="intermediate",
     weak="numpy polyfit of the same double-reciprocal transform")


def _mm_profile(S, v, Km):
    a, b = 0.0, 0.0
    for k in range(len(S)):
        g = S[k] / (Km + S[k])
        a += v[k] * g
        b += g * g
    return a, b


def _mm_dprofile(S, v, Km):
    """d/dKm of the profiled objective -a^2/b, up to the positive factor 1/b^2: -a (2 a' b - a b'), with
    a' = sum v g' and b' = 2 sum g g'."""
    a, b, da, db = 0.0, 0.0, 0.0, 0.0
    for k in range(len(S)):
        g = S[k] / (Km + S[k])
        gp = -S[k] / (Km + S[k]) ** 2
        a += v[k] * g
        b += g * g
        da += v[k] * gp
        db += 2.0 * g * gp
    return -a * (2.0 * da * b - a * db)


def mm_nonlinear_fit(S, v):
    _need(len(S) == len(v) and len(S) >= 3, "give at least 3 substrate concentrations with one rate each")
    for a in S:
        _need(_isnum(a) and a > 0, "substrate concentrations must be positive")
    for a in v:
        _need(_isnum(a) and a > 0, "rates must be positive")
    lo, hi = min(S) * 1e-4, max(S) * 1e4
    # the profiled least-squares objective -a^2/b has its minimum where d/dKm = 0 (sign change from - to +)
    grid = [lo * (hi / lo) ** (k / 400.0) for k in range(401)]
    best, fbest = None, None
    for k in range(400):
        f1, f2 = _mm_dprofile(S, v, grid[k]), _mm_dprofile(S, v, grid[k + 1])
        if f1 < 0 <= f2:
            r = _bisect(lambda x: _mm_dprofile(S, v, x), grid[k], grid[k + 1])
            a, b = _mm_profile(S, v, r)
            obj = -a * a / b
            if best is None or obj < fbest:
                best, fbest = r, obj
    _need(best is not None, "the data do not define a Michaelis-Menten curve (no interior least-squares minimum)")
    Km = best
    a, b = _mm_profile(S, v, Km)
    Vmax = a / b
    rss, mv = 0.0, _mean(v)
    sst = 0.0
    for k in range(len(S)):
        e = v[k] - Vmax * S[k] / (Km + S[k])
        rss += e * e
        sst += (v[k] - mv) ** 2
    return _out(Km=Km, Vmax=Vmax, rss=rss, r2=1.0 - rss / sst)


def _ref_mm_fit(S, v):
    import numpy as np
    from scipy.optimize import curve_fit
    S_, v_ = np.array(S, dtype=float), np.array(v, dtype=float)
    f = lambda s, V, K: V * s / (K + s)
    from scipy.optimize import fsolve
    p, _ = curve_fit(f, S_, v_, p0=[max(v), float(np.median(S_))], method="lm", maxfev=20000)

    def normal_equations(q):                       # the stationarity conditions of the residual sum of squares
        V, K = q
        g = S_ / (K + S_)
        gp = -S_ / (K + S_) ** 2
        r = v_ - V * g
        return [float(np.sum(r * g)), float(np.sum(r * gp))]
    p = fsolve(normal_equations, p, xtol=1e-15)    # polish to full precision (LM stops on the objective's change)
    V, K = float(p[0]), float(p[1])
    res = v_ - f(S_, V, K)
    rss = float(np.sum(res ** 2))
    return {"Km": K, "Vmax": V, "rss": rss, "r2": 1.0 - rss / float(np.sum((v_ - v_.mean()) ** 2))}


_add("cmb_michaelis_menten_nonlinear_fit",
     name="Km and Vmax by nonlinear least squares", topic="enzyme kinetics / Michaelis-Menten",
     purpose="Km and Vmax fitted directly to initial-rate data by nonlinear least squares on v = Vmax S/(Km + S) (no linearizing transform), with the residual sum of squares and r^2.",
     inputs=[_vecin("S", "substrate concentrations (any one unit)", 3, 50), _vecin("v", "initial rates (any one unit)", 3, 50)],
     outputs=[_o("Km", "number", "Michaelis constant (unit of S)"), _o("Vmax", "number", "maximum rate (unit of v)"),
              _o("rss", "number", "residual sum of squares"), _o("r2", "number", "coefficient of determination")],
     primary="Km", conditions=["at least 3 points", "S, v > 0", "an interior least-squares minimum"],
     method="Vmax profiled out (Vmax = sum v g / sum g^2, g = S/(Km + S)); the stationary point in Km found by bisection",
     latex=r"\min_{V,K}\sum_i\left(v_i - \frac{V S_i}{K + S_i}\right)^2",
     run=mm_nonlinear_fit, reference=_ref_mm_fit,
     wrong=lambda S, v: (lambda o: _out(Km=o["Km"], Vmax=o["Vmax"], rss=0.0, r2=o["r2"]))(lineweaver_burk(S, v)),
     rand=lambda rng: _rand_mm_data(rng, 0.03),
     phrases=("Nonlinear Michaelis-Menten fit", "Michaelis-Menten nonlinear fit"),
     examples=[_ex({"S": [1.0, 2.0, 5.0, 10.0, 20.0, 50.0], "v": [16.0, 29.5, 49.0, 68.0, 79.0, 92.0]}),
               _ex({"S": [0.5, 1.0, 2.0, 4.0, 8.0], "v": [2.0, 3.5, 5.0, 6.7, 7.9]})],
     level="expert")


_INHIB = {"competitive": (True, False), "uncompetitive": (False, True), "noncompetitive": (True, True)}


def _inhibited(Vmax, Km, S, I, Ki, mode):
    _pos("Vmax, Km and Ki", Vmax, Km, Ki)
    _nonneg("S and I", S, I)
    a = 1.0 + I / Ki
    if mode == "competitive":
        Km_app, V_app = Km * a, Vmax
    elif mode == "uncompetitive":
        Km_app, V_app = Km / a, Vmax / a
    else:
        Km_app, V_app = Km, Vmax / a
    v = V_app * S / (Km_app + S)
    v0 = Vmax * S / (Km + S)
    return _out(v=v, Km_app=Km_app, Vmax_app=V_app, percent_inhibition=100.0 * (1.0 - v / v0) if v0 > 0 else 0.0)


def _ref_inhibited(Vmax, Km, S, I, Ki, mode):
    e, es = _INHIB[mode]
    f = _re_scheme(S, I, Km, Ki if e else None, Ki if es else None)
    v = Vmax * f
    # apparent Vmax: the rate at a saturating substrate level; apparent Km: the S giving half of it
    V_app = Vmax * _re_scheme(1e14 * Km, I, Km, Ki if e else None, Ki if es else None)
    Km_app = _brentq(lambda s: _re_scheme(s, I, Km, Ki if e else None, Ki if es else None) * Vmax - V_app / 2.0, 1e-9 * Km, 1e9 * Km)
    v0 = Vmax * _re_scheme(S, 0.0, Km, None, None)
    return {"v": v, "Km_app": Km_app, "Vmax_app": V_app, "percent_inhibition": 100.0 * (1.0 - v / v0) if v0 > 0 else 0.0}


def _rand_inhib(rng):
    Km = _sig(_lu(rng, 1e-3, 5.0), 3)
    Ki = _sig(_lu(rng, 1e-4, 5.0), 3)
    return {"Vmax": _sig(_lu(rng, 1e-7, 1e-3), 3), "Km": Km, "S": _sig(Km * _lu(rng, 0.1, 10.0), 3), "I": _sig(Ki * _lu(rng, 0.2, 10.0), 3), "Ki": Ki}


def _inhib_spec(mode, key, name, purpose, latex, method, wrong_mode, examples, level):
    def run(Vmax, Km, S, I, Ki):
        return _inhibited(Vmax, Km, S, I, Ki, mode)

    def ref(Vmax, Km, S, I, Ki):
        return _ref_inhibited(Vmax, Km, S, I, Ki, mode)

    def wrong(Vmax, Km, S, I, Ki):
        return _inhibited(Vmax, Km, S, I, Ki, wrong_mode)
    _add(key, name=name, topic="enzyme kinetics / inhibition", purpose=purpose,
         inputs=[_phys("Vmax", "reaction_rate", "maximum rate without inhibitor"),
                 _phys("Km", "molar_concentration", "Michaelis constant", show=_UMOLAR),
                 _phys("S", "molar_concentration", "substrate concentration", show=_UMOLAR),
                 _phys("I", "molar_concentration", "inhibitor concentration", show=_UMOLAR),
                 _phys("Ki", "molar_concentration", "inhibition (dissociation) constant", show=_UMOLAR)],
         outputs=[_o("v", "reaction_rate", "rate with the inhibitor"), _o("Km_app", "molar_concentration", "apparent Km"),
                  _o("Vmax_app", "reaction_rate", "apparent Vmax"), _o("percent_inhibition", "number", "per cent inhibition at this S")],
         primary="v", conditions=["Vmax, Km, Ki > 0", "S, I >= 0", "rapid-equilibrium inhibitor binding"],
         method=method, latex=latex, run=run, reference=ref, wrong=wrong, rand=_rand_inhib,
         phrases=(name.split(" (")[0], mode.capitalize() + " inhibition"),
         examples=examples, level=level)


_inhib_spec("competitive", "cmb_competitive_inhibition", "Competitive inhibition rate",
            "The rate of an enzyme with a competitive inhibitor (binding free enzyme only): Km rises by (1 + I/Ki), Vmax unchanged.",
            r"v = \frac{V_{max}[S]}{K_m(1 + [I]/K_i) + [S]}", "v = Vmax S / (Km (1 + I/Ki) + S)", "uncompetitive",
            [_ex({"Vmax": 1e-4, "Km": 0.05, "S": 0.1, "I": 0.02, "Ki": 0.01}), _ex({"Vmax": 2e-6, "Km": 1.0, "S": 0.5, "I": 0.5, "Ki": 0.25})],
            "intermediate")
_inhib_spec("uncompetitive", "cmb_uncompetitive_inhibition", "Uncompetitive inhibition rate",
            "The rate of an enzyme with an uncompetitive inhibitor (binding the enzyme-substrate complex only): Km and Vmax both fall by (1 + I/Ki).",
            r"v = \frac{V_{max}[S]}{K_m + [S](1 + [I]/K_i)}", "v = Vmax S / (Km + S (1 + I/Ki))", "competitive",
            [_ex({"Vmax": 1e-4, "Km": 0.05, "S": 0.1, "I": 0.02, "Ki": 0.01}), _ex({"Vmax": 2e-6, "Km": 1.0, "S": 0.5, "I": 0.5, "Ki": 0.25})],
            "intermediate")
_inhib_spec("noncompetitive", "cmb_noncompetitive_inhibition", "Noncompetitive inhibition rate",
            "The rate of an enzyme with a pure noncompetitive inhibitor (equal affinity for E and ES): Vmax falls by (1 + I/Ki), Km unchanged.",
            r"v = \frac{V_{max}[S]}{(K_m + [S])(1 + [I]/K_i)}", "v = Vmax S / ((Km + S)(1 + I/Ki))", "competitive",
            [_ex({"Vmax": 1e-4, "Km": 0.05, "S": 0.1, "I": 0.02, "Ki": 0.01}), _ex({"Vmax": 2e-6, "Km": 1.0, "S": 0.5, "I": 0.5, "Ki": 0.25})],
            "intermediate")


def mixed_inhibition(Vmax, Km, S, I, Ki, Kii):
    _pos("Vmax, Km, Ki and Kii", Vmax, Km, Ki, Kii)
    _nonneg("S and I", S, I)
    a, ap = 1.0 + I / Ki, 1.0 + I / Kii
    v = Vmax * S / (Km * a + S * ap)
    return _out(v=v, Km_app=Km * a / ap, Vmax_app=Vmax / ap, alpha=Kii / Ki)


def _ref_mixed(Vmax, Km, S, I, Ki, Kii):
    f = _re_scheme(S, I, Km, Ki, Kii)
    V_app = Vmax * _re_scheme(1e14 * Km, I, Km, Ki, Kii)
    Km_app = _brentq(lambda s: _re_scheme(s, I, Km, Ki, Kii) * Vmax - V_app / 2.0, 1e-9 * Km, 1e9 * Km)
    return {"v": Vmax * f, "Km_app": Km_app, "Vmax_app": V_app, "alpha": Kii / Ki}


_add("cmb_mixed_inhibition",
     name="Mixed inhibition rate", topic="enzyme kinetics / inhibition",
     purpose="The rate, apparent Km and apparent Vmax of an enzyme with a mixed inhibitor that binds free enzyme (Ki) and the enzyme-substrate complex (Kii) with different affinities.",
     inputs=[_phys("Vmax", "reaction_rate", "maximum rate without inhibitor"), _phys("Km", "molar_concentration", "Michaelis constant", show=_UMOLAR),
             _phys("S", "molar_concentration", "substrate concentration", show=_UMOLAR), _phys("I", "molar_concentration", "inhibitor concentration", show=_UMOLAR),
             _phys("Ki", "molar_concentration", "dissociation constant of EI", show=_UMOLAR),
             _phys("Kii", "molar_concentration", "dissociation constant of ESI", show=_UMOLAR)],
     outputs=[_o("v", "reaction_rate", "rate with the inhibitor"), _o("Km_app", "molar_concentration", "apparent Km"),
              _o("Vmax_app", "reaction_rate", "apparent Vmax"), _o("alpha", "number", "Kii / Ki")],
     primary="v", conditions=["Vmax, Km, Ki, Kii > 0", "S, I >= 0", "rapid equilibrium"],
     method="v = Vmax S / (Km (1 + I/Ki) + S (1 + I/Kii)); Km_app = Km (1 + I/Ki)/(1 + I/Kii); Vmax_app = Vmax/(1 + I/Kii)",
     latex=r"v = \frac{V_{max}[S]}{K_m\left(1 + \frac{[I]}{K_i}\right) + [S]\left(1 + \frac{[I]}{K_i'}\right)}",
     run=mixed_inhibition, reference=_ref_mixed,
     wrong=lambda Vmax, Km, S, I, Ki, Kii: mixed_inhibition(Vmax, Km, S, I, Kii, Ki),
     rand=lambda rng: (lambda d: dict(d, Kii=_sig(d["Ki"] * rng.choice([rng.uniform(0.1, 0.6), rng.uniform(1.8, 10.0)]), 3)))(_rand_inhib(rng)),
     phrases=("Mixed inhibition rate", "Mixed inhibition"),
     examples=[_ex({"Vmax": 1e-4, "Km": 0.05, "S": 0.1, "I": 0.02, "Ki": 0.01, "Kii": 0.04}),
               _ex({"Vmax": 2e-6, "Km": 1.0, "S": 0.5, "I": 0.5, "Ki": 1.0, "Kii": 0.2})],
     level="advanced")


def ic50_to_ki(mode, IC50, S, Km):
    _need(mode in _INHIB, "mode: competitive, uncompetitive or noncompetitive")
    _pos("IC50, S and Km", IC50, S, Km)
    if mode == "competitive":
        Ki = IC50 / (1.0 + S / Km)
    elif mode == "uncompetitive":
        Ki = IC50 / (1.0 + Km / S)
    else:
        Ki = IC50
    return _out(Ki=Ki, Ki_uM=Ki * 1e3)


def _ref_ic50(mode, IC50, S, Km):
    e, es = _INHIB[mode]
    v0 = _re_scheme(S, 0.0, Km, None, None)
    Ki = _brentq(lambda k: _re_scheme(S, IC50, Km, k if e else None, k if es else None) - v0 / 2.0, IC50 * 1e-9, IC50 * 1e3)
    return {"Ki": Ki, "Ki_uM": Ki * 1e3}


_add("cmb_ic50_to_ki",
     name="Inhibition constant from IC50 (Cheng-Prusoff)", topic="enzyme kinetics / inhibition",
     purpose="The inhibition constant Ki from a measured IC50 at substrate concentration S for a competitive, uncompetitive or noncompetitive inhibitor (Cheng-Prusoff relations).",
     inputs=[_choice("mode", ("competitive", "uncompetitive", "noncompetitive"), "mechanism of inhibition"),
             _phys("IC50", "molar_concentration", "inhibitor concentration giving 50 % inhibition", show=_UMOLAR),
             _phys("S", "molar_concentration", "substrate concentration of the assay", show=_UMOLAR),
             _phys("Km", "molar_concentration", "Michaelis constant", show=_UMOLAR)],
     outputs=[_o("Ki", "molar_concentration", "inhibition constant"), _o("Ki_uM", "number", "Ki in uM")],
     primary="Ki", conditions=["IC50, S, Km > 0", "inhibitor not depleted (I >> E)"],
     method="competitive Ki = IC50/(1 + S/Km); uncompetitive Ki = IC50/(1 + Km/S); noncompetitive Ki = IC50",
     latex=r"K_i = \frac{IC_{50}}{1 + [S]/K_m}\ \text{(competitive)}",
     run=ic50_to_ki, reference=_ref_ic50,
     wrong=lambda mode, IC50, S, Km: _out(Ki=IC50 * (1.0 + S / Km), Ki_uM=IC50 * (1.0 + S / Km) * 1e3),
     rand=lambda rng: {"mode": rng.choice(["competitive", "uncompetitive", "noncompetitive"]), "IC50": _sig(_lu(rng, 1e-5, 10.0), 3),
                       "S": _sig(_lu(rng, 1e-3, 10.0), 3), "Km": _sig(_lu(rng, 1e-3, 10.0), 3)},
     phrases=("Ki from IC50", "Cheng-Prusoff Ki", "Inhibition constant from IC50"),
     examples=[_ex({"mode": "competitive", "IC50": 0.01, "S": 0.1, "Km": 0.05}), _ex({"mode": "uncompetitive", "IC50": 0.5, "S": 1.0, "Km": 3.0})],
     level="advanced",
     extra_q=[{"q": "Ki from IC50: mode = Competitive, IC50 = 10 uM, S = 0.1 mM, Km = 50 uM", "like": 0}])


def hill_equation(L, K, n):
    _pos("L, K and n", L, K, n)
    x = (L / K) ** n
    theta = x / (1.0 + x)
    return _out(theta=theta, theta_percent=100.0 * theta)


def _ref_hill(L, K, n):
    from scipy.special import expit
    th = float(expit(n * (math.log(L) - math.log(K))))
    return {"theta": th, "theta_percent": 100.0 * th}


_add("cmb_hill_equation",
     name="Fractional saturation by the Hill equation", topic="enzyme kinetics and binding / cooperativity",
     purpose="The fractional saturation (or response) of a cooperative binding protein at ligand concentration L, with half-saturation K and Hill coefficient n.",
     inputs=[_phys("L", "molar_concentration", "free ligand concentration", show=_UMOLAR),
             _phys("K", "molar_concentration", "ligand concentration for half saturation (K0.5)", show=_UMOLAR),
             _numin("n", "Hill coefficient")],
     outputs=[_o("theta", "number", "fractional saturation"), _o("theta_percent", "number", "saturation in per cent")],
     primary="theta", conditions=["L, K, n > 0"],
     method="theta = L^n / (K^n + L^n)", latex=r"\theta = \frac{[L]^n}{K_{0.5}^n + [L]^n}",
     run=hill_equation, reference=_ref_hill,
     wrong=lambda L, K, n: _out(theta=L / (K + L), theta_percent=100.0 * L / (K + L)) if abs(n - 1.0) > 0.05 else _out(theta=L ** 2 / (K ** 2 + L ** 2), theta_percent=100.0 * L ** 2 / (K ** 2 + L ** 2)),
     rand=lambda rng: (lambda K: {"L": _sig(K * _lu(rng, 0.1, 10.0), 3), "K": K, "n": _sig(rng.uniform(0.5, 4.0), 3)})(_sig(_lu(rng, 1e-4, 10.0), 3)),
     phrases=("Hill equation", "Hill fractional saturation"),
     examples=[_ex({"L": 0.026, "K": 0.026, "n": 2.8}), _ex({"L": 0.01, "K": 0.02, "n": 2.0})],
     level="intermediate",
     weak="the logistic form expit(n ln(L/K)) (scipy.special)")


def hill_ligand_for_fraction(K, n, theta):
    _pos("K and n", K, n)
    _frac("the fractional saturation theta", theta)
    L = K * (theta / (1.0 - theta)) ** (1.0 / n)
    return _out(L=L, L_uM=L * 1e3)


def _ref_hill_ligand(K, n, theta):
    from scipy.special import logit
    L = K * math.exp(float(logit(theta)) / n)
    return {"L": L, "L_uM": L * 1e3}


_add("cmb_hill_ligand_for_fraction",
     name="Ligand concentration for a given saturation (Hill)", topic="enzyme kinetics and binding / cooperativity",
     purpose="The ligand concentration that gives a fractional saturation theta for a cooperative protein with half-saturation K and Hill coefficient n.",
     inputs=[_phys("K", "molar_concentration", "half-saturation concentration K0.5", show=_UMOLAR), _numin("n", "Hill coefficient"),
             _numin("theta", "fractional saturation (0 to 1)", pct=True)],
     outputs=[_o("L", "molar_concentration", "required ligand concentration"), _o("L_uM", "number", "L in uM")],
     primary="L", conditions=["K, n > 0", "0 < theta < 1"],
     method="L = K (theta/(1 - theta))^(1/n)", latex=r"[L] = K_{0.5}\left(\frac{\theta}{1 - \theta}\right)^{1/n}",
     run=hill_ligand_for_fraction, reference=_ref_hill_ligand,
     wrong=lambda K, n, theta: _out(L=K * (theta / (1.0 - theta)) ** n, L_uM=K * (theta / (1.0 - theta)) ** n * 1e3),
     rand=lambda rng: {"K": _sig(_lu(rng, 1e-4, 10.0), 3), "n": _sig(rng.choice([rng.uniform(0.5, 0.85), rng.uniform(1.3, 4.0)]), 3),
                       "theta": _sig(rng.choice([rng.uniform(0.02, 0.4), rng.uniform(0.6, 0.98)]), 3)},
     phrases=("Hill ligand concentration for a saturation", "Ligand for a fractional saturation"),
     examples=[_ex({"K": 0.026, "n": 2.8, "theta": 0.9}), _ex({"K": 0.005, "n": 1.5, "theta": 0.2})],
     level="intermediate",
     extra_q=[{"q": "Ligand for a fractional saturation: K = 26 uM, n = 2.8, theta = 90%", "like": 0}])


def hill_from_ec(EC10, EC90):
    _pos("EC10 and EC90", EC10, EC90)
    _need(EC90 > EC10, "EC90 must exceed EC10")
    n = math.log(81.0) / math.log(EC90 / EC10)
    return _out(n=n, EC50=math.sqrt(EC10 * EC90), ratio=EC90 / EC10)


def _ref_hill_ec(EC10, EC90):
    # solve the two Hill conditions theta(EC10) = 0.1, theta(EC90) = 0.9 for n and K
    def K_of(n):
        return EC10 * (9.0) ** (1.0 / n)
    n = _brentq(lambda nn: (EC90 / K_of(nn)) ** nn / (1.0 + (EC90 / K_of(nn)) ** nn) - 0.9, 0.01, 100.0)
    return {"n": n, "EC50": K_of(n), "ratio": EC90 / EC10}


_add("cmb_hill_coefficient_from_ec",
     name="Hill coefficient from EC10 and EC90", topic="enzyme kinetics and binding / cooperativity",
     purpose="The Hill coefficient n = ln 81 / ln(EC90/EC10) and the EC50 (their geometric mean) of a Hill dose-response curve (81-fold range for n = 1).",
     inputs=[_phys("EC10", "molar_concentration", "concentration giving 10 % of the maximum", show=_UMOLAR),
             _phys("EC90", "molar_concentration", "concentration giving 90 % of the maximum", show=_UMOLAR)],
     outputs=[_o("n", "number", "Hill coefficient"), _o("EC50", "molar_concentration", "half-maximal concentration"),
              _o("ratio", "number", "EC90 / EC10")],
     primary="n", conditions=["0 < EC10 < EC90", "a Hill curve"],
     method="n = ln 81 / ln(EC90/EC10); EC50 = sqrt(EC10 EC90)", latex=r"n_H = \frac{\ln 81}{\ln(EC_{90}/EC_{10})}",
     run=hill_from_ec, reference=_ref_hill_ec,
     wrong=lambda EC10, EC90: _out(n=math.log10(81.0) / math.log10(EC90 / EC10) * 0.5, EC50=(EC10 + EC90) / 2.0, ratio=EC90 / EC10),
     rand=lambda rng: (lambda e10: {"EC10": e10, "EC90": _sig(e10 * _lu(rng, 3.0, 500.0), 3)})(_sig(_lu(rng, 1e-5, 1.0), 3)),
     phrases=("Hill coefficient from EC10 and EC90", "Hill coefficient from EC values"),
     examples=[_ex({"EC10": 0.001, "EC90": 0.081}), _ex({"EC10": 0.01, "EC90": 0.09})],
     level="advanced")


def fraction_bound(L, Kd, Rt=None):
    _pos("L and Kd", L, Kd)
    th = L / (Kd + L)
    out = {"theta": th, "theta_percent": 100.0 * th}
    if Rt is not None:
        _pos("the receptor concentration Rt", Rt)
        out["bound"] = th * Rt
    return _out(**out)


def _ref_fraction_bound(L, Kd, Rt=None):
    from scipy.special import expit
    th = float(expit(math.log(L) - math.log(Kd)))
    out = {"theta": th, "theta_percent": 100.0 * th}
    if Rt is not None:
        out["bound"] = th * Rt
    return out


_add("cmb_fraction_bound",
     name="Fraction of receptor bound at a free ligand concentration", topic="enzyme kinetics and binding / binding",
     purpose="The fractional occupancy L/(Kd + L) of a single-site receptor at free ligand concentration L, and the bound concentration for a receptor total Rt (ligand not depleted).",
     inputs=[_phys("L", "molar_concentration", "free ligand concentration", show=_NMOLAR),
             _phys("Kd", "molar_concentration", "dissociation constant", show=_NMOLAR),
             _phys("Rt", "molar_concentration", "total receptor concentration (optional)", required=False, show=_NMOLAR)],
     outputs=[_o("theta", "number", "fraction bound"), _o("theta_percent", "number", "per cent bound"),
              _o("bound", "molar_concentration", "bound complex concentration (when Rt is given)")],
     primary="theta", conditions=["L, Kd > 0", "single independent site", "ligand in excess (free = total)"],
     method="theta = L / (Kd + L)", latex=r"\theta = \frac{[L]}{K_d + [L]}",
     run=fraction_bound, reference=_ref_fraction_bound,
     wrong=lambda L, Kd, Rt=None: _out(**dict({"theta": Kd / (Kd + L), "theta_percent": 100.0 * Kd / (Kd + L)}, **({"bound": Kd / (Kd + L) * Rt} if Rt is not None else {}))),
     rand=lambda rng: dict((lambda K: {"L": _sig(K * _lu(rng, 0.05, 20.0), 3), "Kd": K})(_sig(_lu(rng, 1e-7, 1.0), 3)),
                           **({"Rt": _sig(_lu(rng, 1e-7, 1e-3), 3)} if rng.random() < 0.4 else {})),
     phrases=("Fraction bound", "Fractional occupancy", "Receptor occupancy"),
     examples=[_ex({"L": 1e-5, "Kd": 5e-6}), _ex({"L": 2e-4, "Kd": 1e-3, "Rt": 5e-5})],
     level="beginner",
     weak="the logistic form expit(ln L - ln Kd) (scipy.special)")


def ligand_depletion_binding(Rt, Lt, Kd):
    _pos("Rt, Lt and Kd", Rt, Lt, Kd)
    s = Rt + Lt + Kd
    # the smaller root of RL^2 - s RL + Rt Lt = 0, written in the cancellation-free form
    RL = 2.0 * Rt * Lt / (s + math.sqrt(s * s - 4.0 * Rt * Lt))
    return _out(RL=RL, fraction_receptor_bound=RL / Rt, free_ligand=Lt - RL, fraction_ligand_bound=RL / Lt)


def _ref_depletion(Rt, Lt, Kd):
    RL = _brentq(lambda x: x * Kd - (Rt - x) * (Lt - x), 0.0, min(Rt, Lt))
    return {"RL": RL, "fraction_receptor_bound": RL / Rt, "free_ligand": Lt - RL, "fraction_ligand_bound": RL / Lt}


_add("cmb_ligand_depletion_binding",
     name="Complex concentration with ligand depletion (quadratic binding)", topic="enzyme kinetics and binding / binding",
     purpose="The concentration of receptor-ligand complex when neither component is in excess (exact quadratic solution), with the fractions bound and the free ligand.",
     inputs=[_phys("Rt", "molar_concentration", "total receptor concentration", show=_NMOLAR),
             _phys("Lt", "molar_concentration", "total ligand concentration", show=_NMOLAR),
             _phys("Kd", "molar_concentration", "dissociation constant", show=_NMOLAR)],
     outputs=[_o("RL", "molar_concentration", "complex concentration"), _o("fraction_receptor_bound", "number", "RL / Rt"),
              _o("free_ligand", "molar_concentration", "free ligand Lt - RL"), _o("fraction_ligand_bound", "number", "RL / Lt")],
     primary="RL", conditions=["Rt, Lt, Kd > 0", "single site, 1:1 binding at equilibrium"],
     method="RL = [(Rt + Lt + Kd) - sqrt((Rt + Lt + Kd)^2 - 4 Rt Lt)] / 2",
     latex=r"[RL] = \frac{(R_t + L_t + K_d) - \sqrt{(R_t + L_t + K_d)^2 - 4R_tL_t}}{2}",
     run=ligand_depletion_binding, reference=_ref_depletion,
     wrong=lambda Rt, Lt, Kd: (lambda RL: _out(RL=RL, fraction_receptor_bound=RL / Rt, free_ligand=Lt - RL, fraction_ligand_bound=RL / Lt))(Rt * Lt / (Kd + Lt)),
     rand=lambda rng: (lambda K: {"Rt": _sig(K * _lu(rng, 0.1, 50.0), 3), "Lt": _sig(K * _lu(rng, 0.1, 50.0), 3), "Kd": K})(_sig(_lu(rng, 1e-7, 1e-2), 3)),
     phrases=("Quadratic binding with ligand depletion", "Ligand depletion binding", "Tight binding complex"),
     examples=[_ex({"Rt": 1e-5, "Lt": 2e-5, "Kd": 1e-5}), _ex({"Rt": 1e-4, "Lt": 5e-5, "Kd": 2e-6})],
     level="advanced")


def kd_from_rates(kon, koff):
    _pos("kon and koff", kon, koff)
    Kd = koff / kon
    return _out(Kd=Kd, Kd_nM=Kd * 1e6, Ka_per_M=kon / koff * 1e3, half_life=LN2 / koff, residence_time=1.0 / koff)


def _ref_kd_rates(kon, koff):
    # equilibrium of the association-dissociation kinetics at a ligand level L: theta = kon L/(kon L + koff); Kd is the L giving 1/2
    Kd = _brentq(lambda L: kon * L / (kon * L + koff) - 0.5, 0.0, 1e6 * koff / kon)
    th = _brentq(lambda t: math.exp(-koff * t) - 0.5, 0.0, 10.0 / koff)
    return {"Kd": Kd, "Kd_nM": Kd * 1e6, "Ka_per_M": 1e3 / Kd, "half_life": th, "residence_time": 1.0 / koff}


_add("cmb_kd_from_rates",
     name="Dissociation constant from association and dissociation rates", topic="enzyme kinetics and binding / binding kinetics",
     purpose="The equilibrium dissociation constant Kd = koff/kon, the association constant, the complex half-life and the residence time.",
     inputs=[_phys("kon", "second_order_rate_constant", "association rate constant"),
             _phys("koff", "first_order_rate_constant", "dissociation rate constant", show=_PER_S)],
     outputs=[_o("Kd", "molar_concentration", "dissociation constant"), _o("Kd_nM", "number", "Kd in nM"),
              _o("Ka_per_M", "number", "association constant in 1/M"), _o("half_life", "time", "complex half-life ln2/koff"),
              _o("residence_time", "time", "mean residence time 1/koff")],
     primary="Kd", conditions=["kon, koff > 0", "simple 1:1 binding"],
     method="Kd = koff/kon; t1/2 = ln 2/koff; tau = 1/koff", latex=r"K_d = \frac{k_{off}}{k_{on}},\quad t_{1/2} = \frac{\ln 2}{k_{off}}",
     run=kd_from_rates, reference=_ref_kd_rates,
     wrong=lambda kon, koff: (lambda K: _out(Kd=K, Kd_nM=K * 1e6, Ka_per_M=1.0 / K, half_life=LN2 / koff, residence_time=1.0 / koff))(kon / koff),
     rand=lambda rng: {"kon": _sig(_lu(rng, 1e0, 1e6), 3), "koff": _sig(_lu(rng, 1e-5, 10.0), 3)},
     phrases=("Kd from rate constants", "Dissociation constant from rates"),
     examples=[_ex({"kon": 1e3, "koff": 1e-3}), _ex({"kon": 5e2, "koff": 0.2})],
     level="intermediate",
     extra_q=[{"q": "Kd from rate constants: kon = 1e6 1/(M*s), koff = 0.001 1/s", "like": 0}])


def scatchard_fit(B, F):
    _need(len(B) == len(F) and len(B) >= 3, "give at least 3 bound and free concentrations")
    for a in B:
        _need(_isnum(a) and a > 0, "bound concentrations must be positive")
    for a in F:
        _need(_isnum(a) and a > 0, "free concentrations must be positive")
    y = [B[k] / F[k] for k in range(len(B))]
    m, b, r2 = _linfit(B, y)
    _need(m < 0, "the Scatchard line must have a negative slope")
    return _out(Kd=-1.0 / m, Bmax=-b / m, slope=m, intercept=b, r2=r2)


def _ref_scatchard(B, F):
    import numpy as np
    B_ = np.array(B, dtype=float)
    y = B_ / np.array(F, dtype=float)
    m, b = np.polyfit(B_, y, 1)
    r = np.corrcoef(B_, y)[0, 1]
    return {"Kd": float(-1.0 / m), "Bmax": float(-b / m), "slope": float(m), "intercept": float(b), "r2": float(r * r)}


def _rand_scatchard(rng):
    Kd = _lu(rng, 0.5, 50.0)
    Bmax = _lu(rng, 10.0, 1000.0)
    n = rng.randint(5, 9)
    F = sorted(set(_sig(Kd * _lu(rng, 0.1, 10.0), 3) for _ in range(n + 3)))[:n]
    return {"B": [_sig(Bmax * f / (Kd + f) * (1.0 + rng.gauss(0.0, 0.02)), 4) for f in F], "F": F}


_add("cmb_scatchard_fit",
     name="Kd and Bmax from a Scatchard plot", topic="enzyme kinetics and binding / binding",
     purpose="The dissociation constant and the total binding sites from saturation-binding data by the Scatchard least-squares line B/F = Bmax/Kd - B/Kd.",
     inputs=[_vecin("B", "bound ligand concentrations (any one unit, e.g. nM or fmol/mg)", 3, 50),
             _vecin("F", "free ligand concentrations (e.g. nM)", 3, 50)],
     outputs=[_o("Kd", "number", "dissociation constant (unit of F)"), _o("Bmax", "number", "total binding sites (unit of B)"),
              _o("slope", "number", "slope -1/Kd"), _o("intercept", "number", "intercept Bmax/Kd"), _o("r2", "number", "r^2 of the Scatchard line")],
     primary="Kd", conditions=["at least 3 points", "B, F > 0", "negative slope (one class of sites)"],
     method="least squares B/F = a + m B; Kd = -1/m; Bmax = -a/m", latex=r"\frac{B}{F} = \frac{B_{max}}{K_d} - \frac{B}{K_d}",
     run=scatchard_fit, reference=_ref_scatchard,
     wrong=lambda B, F: (lambda o: dict(o, Kd=_r(-o["slope"]), Bmax=_r(o["intercept"])))(scatchard_fit(B, F)),
     rand=_rand_scatchard,
     phrases=("Scatchard fit", "Scatchard plot"),
     examples=[_ex({"B": [16.7, 28.6, 50.0, 66.7, 80.0], "F": [1.0, 2.0, 5.0, 10.0, 20.0]}),
               _ex({"B": [9.0, 16.5, 28.0, 41.0], "F": [0.5, 1.0, 2.0, 4.0]})],
     level="advanced",
     weak="numpy polyfit of the same Scatchard transform")


def binding_free_energy(Kd, T):
    _pos("Kd and T", Kd, T)
    dG = R_GAS * T * math.log(Kd * 1e-3)
    return _out(dG=dG, dG_kJ_mol=dG / 1000.0, dG_kcal_mol=dG / 4184.0)


def _ref_binding_dg(Kd, T):
    sc = _sc()
    Ka = 1.0 / (Kd / 1000.0)                 # 1/M, standard state 1 M
    dG = -sc.R * T * math.log(Ka)
    return {"dG": dG, "dG_kJ_mol": dG / 1000.0, "dG_kcal_mol": dG / sc.calorie / 1000.0}


_add("cmb_binding_free_energy",
     name="Standard binding free energy from Kd", topic="enzyme kinetics and binding / binding",
     purpose="The standard free energy of binding dG = RT ln(Kd / 1 M) (negative for favourable binding) at temperature T.",
     inputs=[_phys("Kd", "molar_concentration", "dissociation constant", show=_NMOLAR),
             _phys("T", "temperature", "absolute temperature (degC is converted)", show=_KELVIN)],
     outputs=[_o("dG", "molar_energy", "standard binding free energy"), _o("dG_kJ_mol", "number", "dG in kJ/mol"),
              _o("dG_kcal_mol", "number", "dG in kcal/mol")],
     primary="dG", conditions=["Kd, T > 0", "standard state 1 M"],
     method="dG = R T ln(Kd/1 M) = -R T ln Ka", latex=r"\Delta G^\circ = RT\ln\frac{K_d}{1\ \mathrm{M}}",
     run=binding_free_energy, reference=_ref_binding_dg,
     wrong=lambda Kd, T: (lambda g: _out(dG=g, dG_kJ_mol=g / 1000.0, dG_kcal_mol=g / 4184.0))(R_GAS * T * math.log10(Kd * 1e-3)),
     rand=lambda rng: {"Kd": _sig(_lu(rng, 1e-9, 10.0), 3), "T": _sig(rng.uniform(277.0, 315.0), 5)},
     phrases=("Binding free energy", "Free energy of binding from Kd"),
     examples=[_ex({"Kd": 1e-5, "T": 298.15}), _ex({"Kd": 2.5e-3, "T": 310.15})],
     level="intermediate",
     extra_q=[{"q": "Binding free energy: Kd = 10 nM, T = 25 degC", "like": 0}],
     weak="the same relation written through Ka with scipy.constants")


def competitive_occupancy(L, Kd, I, Ki):
    _pos("Kd and Ki", Kd, Ki)
    _nonneg("L and I", L, I)
    den = 1.0 + L / Kd + I / Ki
    return _out(occupancy_L=L / Kd / den, occupancy_I=I / Ki / den, free_fraction=1.0 / den, Kd_app=Kd * (1.0 + I / Ki))


def _ref_comp_occupancy(L, Kd, I, Ki):
    import numpy as np
    A = np.array([[1.0, 1.0, 1.0], [L / Kd, -1.0, 0.0], [I / Ki, 0.0, -1.0]])
    x = np.linalg.solve(A, np.array([1.0, 0.0, 0.0]))
    Kd_app = _brentq(lambda l: l / Kd / (1.0 + l / Kd + I / Ki) - 0.5, 0.0, 1e6 * Kd * (1.0 + I / Ki))
    return {"occupancy_L": float(x[1]), "occupancy_I": float(x[2]), "free_fraction": float(x[0]), "Kd_app": Kd_app}


_add("cmb_competitive_occupancy",
     name="Receptor occupancy with a competing ligand", topic="enzyme kinetics and binding / binding",
     purpose="The fractions of a receptor occupied by a ligand and by a competitor binding the same site, the free fraction and the apparent Kd of the ligand in the competitor's presence (no depletion).",
     inputs=[_phys("L", "molar_concentration", "free ligand concentration", show=_NMOLAR), _phys("Kd", "molar_concentration", "ligand dissociation constant", show=_NMOLAR),
             _phys("I", "molar_concentration", "free competitor concentration", show=_NMOLAR), _phys("Ki", "molar_concentration", "competitor dissociation constant", show=_NMOLAR)],
     outputs=[_o("occupancy_L", "number", "fraction occupied by the ligand"), _o("occupancy_I", "number", "fraction occupied by the competitor"),
              _o("free_fraction", "number", "fraction of free receptor"), _o("Kd_app", "molar_concentration", "apparent Kd of the ligand Kd (1 + I/Ki)")],
     primary="occupancy_L", conditions=["Kd, Ki > 0", "L, I >= 0", "one site, mutually exclusive binding, no depletion"],
     method="occupancy_L = (L/Kd) / (1 + L/Kd + I/Ki)", latex=r"\theta_L = \frac{[L]/K_d}{1 + [L]/K_d + [I]/K_i}",
     run=competitive_occupancy, reference=_ref_comp_occupancy,
     wrong=lambda L, Kd, I, Ki: _out(occupancy_L=L / (Kd + L), occupancy_I=I / (Ki + I), free_fraction=Kd / (Kd + L), Kd_app=Kd),
     rand=lambda rng: (lambda K, Kc: {"L": _sig(K * _lu(rng, 0.1, 10.0), 3), "Kd": K, "I": _sig(Kc * _lu(rng, 0.1, 10.0), 3), "Ki": Kc})(
         _sig(_lu(rng, 1e-7, 1e-2), 3), _sig(_lu(rng, 1e-7, 1e-2), 3)),
     phrases=("Competitive receptor occupancy", "Occupancy with a competitor"),
     examples=[_ex({"L": 1e-5, "Kd": 5e-6, "I": 2e-5, "Ki": 1e-5}), _ex({"L": 1e-4, "Kd": 1e-4, "I": 1e-3, "Ki": 5e-5})],
     level="advanced")


def mwc_saturation(S, KR, c, L0, n):
    _pos("S, KR, c and L0", S, KR, c, L0)
    _need(isinstance(n, int) and 1 <= n <= 12, "the number of sites n must be a whole number from 1 to 12")
    a = S / KR
    num = a * (1.0 + a) ** (n - 1) + L0 * c * a * (1.0 + c * a) ** (n - 1)
    Z = (1.0 + a) ** n + L0 * (1.0 + c * a) ** n
    return _out(Y=num / Z, R_fraction=(1.0 + a) ** n / Z)


def _ref_mwc(S, KR, c, L0, n):
    import sympy as sp
    al = sp.Symbol("al", positive=True)
    Z = (1 + al) ** n + sp.Float(L0, 30) * (1 + sp.Float(c, 30) * al) ** n
    Y = al * sp.diff(sp.log(Z), al) / n
    a = sp.Float(S, 30) / sp.Float(KR, 30)
    return {"Y": float(Y.subs(al, a).evalf(25)), "R_fraction": float(((1 + al) ** n / Z).subs(al, a).evalf(25))}


_add("cmb_mwc_saturation",
     name="Fractional saturation in the MWC (concerted) model", topic="enzyme kinetics and binding / cooperativity",
     purpose="The fractional saturation and the fraction of molecules in the R state of an allosteric protein with n sites in the Monod-Wyman-Changeux model (L0 = [T]/[R] without ligand, c = KR/KT).",
     inputs=[_phys("S", "molar_concentration", "ligand concentration", show=_UMOLAR),
             _phys("KR", "molar_concentration", "dissociation constant of a site in the R state", show=_UMOLAR),
             _numin("c", "ratio KR/KT (0 < c < 1)"), _numin("L0", "allosteric constant [T0]/[R0]"),
             _intin("n", "number of binding sites", lo=1, hi=12)],
     outputs=[_o("Y", "number", "fractional saturation"), _o("R_fraction", "number", "fraction of molecules in the R state")],
     primary="Y", conditions=["S, KR, c, L0 > 0", "1 <= n <= 12", "concerted R/T transition"],
     method="alpha = S/KR; Y = [alpha (1 + alpha)^(n-1) + L0 c alpha (1 + c alpha)^(n-1)] / [(1 + alpha)^n + L0 (1 + c alpha)^n]",
     latex=r"Y = \frac{\alpha(1+\alpha)^{n-1} + L_0 c\alpha(1+c\alpha)^{n-1}}{(1+\alpha)^n + L_0(1+c\alpha)^n}",
     run=mwc_saturation, reference=_ref_mwc,
     wrong=lambda S, KR, c, L0, n: (lambda a: _out(Y=(a * (1.0 + a) ** (n - 1) + L0 * c * a * (1.0 + c * a) ** (n - 1)) / ((1.0 + a) ** n + L0 * (1.0 + a) ** n),
                                                     R_fraction=1.0 / (1.0 + L0)))(S / KR),
     rand=lambda rng: (lambda K: {"S": _sig(K * _lu(rng, 0.05, 50.0), 3), "KR": K, "c": _sig(_lu(rng, 1e-3, 0.5), 3),
                                  "L0": _sig(_lu(rng, 1.0, 1e5), 3), "n": rng.randint(2, 6)})(_sig(_lu(rng, 1e-4, 1.0), 3)),
     phrases=("MWC fractional saturation", "Monod-Wyman-Changeux saturation"),
     examples=[_ex({"S": 0.01, "KR": 0.002, "c": 0.01, "L0": 9000.0, "n": 4}), _ex({"S": 0.5, "KR": 0.1, "c": 0.1, "L0": 100.0, "n": 2})],
     level="expert")


def q10_rate(k1, T1, T2, Q10):
    _pos("k1, T1, T2 and Q10", k1, T1, T2, Q10)
    _need(T1 != T2, "the two temperatures must differ")
    k2 = k1 * Q10 ** ((T2 - T1) / 10.0)
    Ea = R_GAS * math.log(k2 / k1) / (1.0 / T1 - 1.0 / T2)
    return _out(k2=k2, ratio=k2 / k1, Ea=Ea, Ea_kJ_mol=Ea / 1000.0)


def _ref_q10(k1, T1, T2, Q10):
    sc = _sc()
    lr = (T2 - T1) / 10.0 * math.log(Q10)
    k2 = k1 * math.exp(lr)
    # Arrhenius line through the two points: slope of ln k against 1/T is -Ea/R
    s, _, _ = _linregress([1.0 / T1, 1.0 / T2], [math.log(k1), math.log(k1) + lr])
    Ea = -s * sc.R
    return {"k2": k2, "ratio": k2 / k1, "Ea": Ea, "Ea_kJ_mol": Ea / 1000.0}


_add("cmb_q10_temperature",
     name="Rate at another temperature from Q10", topic="enzyme kinetics / temperature",
     purpose="A biological rate at temperature T2 from its value at T1 and the temperature coefficient Q10, and the equivalent Arrhenius activation energy over that interval.",
     inputs=[_numin("k1", "rate at T1 (any unit)"), _phys("T1", "temperature", "first temperature (degC is converted)", show=_KELVIN),
             _phys("T2", "temperature", "second temperature (degC is converted)", show=_KELVIN), _numin("Q10", "temperature coefficient Q10")],
     outputs=[_o("k2", "number", "rate at T2 (unit of k1)"), _o("ratio", "number", "k2 / k1"),
              _o("Ea", "molar_energy", "equivalent activation energy"), _o("Ea_kJ_mol", "number", "activation energy in kJ/mol")],
     primary="k2", conditions=["k1, Q10 > 0", "T1 != T2, both > 0 K"],
     method="k2 = k1 Q10^((T2 - T1)/10); Ea = R ln(k2/k1) / (1/T1 - 1/T2)",
     latex=r"k_2 = k_1 Q_{10}^{(T_2 - T_1)/10},\quad E_a = \frac{R\ln(k_2/k_1)}{1/T_1 - 1/T_2}",
     run=q10_rate, reference=_ref_q10,
     wrong=lambda k1, T1, T2, Q10: q10_rate(k1, T1, T1 + 2.0 * (T2 - T1), Q10),
     rand=lambda rng: (lambda T1: {"k1": _sig(_lu(rng, 1e-3, 1e3), 3), "T1": T1, "T2": _sig(T1 + rng.choice([-1.0, 1.0]) * rng.uniform(3.0, 25.0), 5),
                                   "Q10": _sig(rng.uniform(1.5, 3.5), 3)})(_sig(rng.uniform(280.0, 310.0), 5)),
     phrases=("Q10 temperature coefficient", "Rate from Q10"),
     examples=[_ex({"k1": 10.0, "T1": 298.15, "T2": 308.15, "Q10": 2.0}), _ex({"k1": 4.5, "T1": 310.15, "T2": 293.15, "Q10": 2.5})],
     level="intermediate",
     extra_q=[{"q": "Rate from Q10: k1 = 10, T1 = 25 degC, T2 = 35 degC, Q10 = 2", "like": 0}])


def enzyme_assay_activity(dA, dt, eps, l, V_assay, V_enzyme, protein=None):
    _pos("dt, eps, l and the volumes", dt, eps, l, V_assay, V_enzyme)
    _fin(dA)
    _need(dA != 0, "the absorbance change must not be zero")
    _need(V_enzyme <= V_assay, "the enzyme volume cannot exceed the assay volume")
    rate = abs(dA) / dt / (eps * l)                       # mol/(m^3 s) in the cuvette
    umol_min = rate * V_assay * 1e6 * MINUTE              # umol/min in the cuvette
    u_per_ml = umol_min / (V_enzyme * 1e6)
    out = {"rate": rate, "units": umol_min, "units_per_mL": u_per_ml}
    if protein is not None:
        _pos("the protein concentration", protein)
        out["specific_activity"] = u_per_ml / protein      # U/mg (protein in kg/m^3 = mg/mL)
    return _out(**out)


def _ref_assay(dA, dt, eps, l, V_assay, V_enzyme, protein=None):
    eps_M = eps * 10.0                                    # L/(mol cm)
    dA_min = abs(dA) / (dt / 60.0)
    c_rate_M_min = dA_min / (eps_M * l * 100.0)           # mol/(L min)
    umol_min = c_rate_M_min * (V_assay * 1000.0) * 1e6
    out = {"rate": c_rate_M_min * 1000.0 / 60.0, "units": umol_min, "units_per_mL": umol_min / (V_enzyme * 1e6)}
    if protein is not None:
        out["specific_activity"] = out["units_per_mL"] / protein
    return out


_add("cmb_enzyme_assay_activity",
     name="Enzyme activity and specific activity from a spectrophotometric assay", topic="enzyme kinetics / assays",
     purpose="The enzyme activity (units = umol/min) in a cuvette from the absorbance change over a time (Beer-Lambert), the activity per mL of the enzyme sample and, with its protein concentration, the specific activity (U/mg).",
     inputs=[_numin("dA", "absorbance change over dt"), _phys("dt", "time", "time of the absorbance change", show=_MINUTES),
             _phys("eps", "molar_absorptivity", "molar absorptivity of the measured product or substrate (NADH 6220 L/(mol cm))"),
             _phys("l", "length", "path length", show=_CM), _phys("V_assay", "volume", "total assay volume", show=_ML),
             _phys("V_enzyme", "volume", "volume of enzyme sample added", show=_UL),
             _phys("protein", "mass_concentration", "protein concentration of the enzyme sample (optional)", required=False, show=("mg/mL", 1.0))],
     outputs=[_o("rate", "reaction_rate", "rate of concentration change in the cuvette"), _o("units", "number", "activity in the cuvette in umol/min (U)"),
              _o("units_per_mL", "number", "activity of the enzyme sample in U/mL"), _o("specific_activity", "number", "specific activity in U/mg (when protein is given)")],
     primary="units_per_mL", conditions=["dA != 0", "dt, eps, l, volumes > 0", "V_enzyme <= V_assay", "linear initial rate"],
     method="rate = |dA|/dt / (eps l); U = rate x V_assay (umol/min); U/mL = U / V_enzyme; U/mg = (U/mL) / (mg/mL)",
     latex=r"\text{U/mL} = \frac{(\Delta A/\Delta t)\,V_{assay}}{\varepsilon\,\ell\,V_{enz}}",
     run=enzyme_assay_activity, reference=_ref_assay,
     wrong=lambda dA, dt, eps, l, V_assay, V_enzyme, protein=None: enzyme_assay_activity(dA, dt, eps, l, V_enzyme, V_enzyme, protein),
     rand=lambda rng: dict({"dA": _sig(rng.uniform(0.02, 0.5) * rng.choice([1.0, -1.0]), 3), "dt": rng.choice([30.0, 60.0, 120.0, 180.0]),
                            "eps": rng.choice([622.0, 1360.0, 1810.0]), "l": 0.01, "V_assay": rng.choice([1e-6, 3e-6, 2e-7]),
                            "V_enzyme": rng.choice([1e-8, 2e-8, 5e-8])},
                           **({"protein": _sig(rng.uniform(0.05, 5.0), 3)} if rng.random() < 0.6 else {})),
     phrases=("Enzyme assay activity", "Enzyme activity from absorbance", "Specific activity from an assay"),
     examples=[_ex({"dA": 0.12, "dt": 60.0, "eps": 622.0, "l": 0.01, "V_assay": 1e-6, "V_enzyme": 2e-8, "protein": 0.5}),
               _ex({"dA": -0.3, "dt": 120.0, "eps": 622.0, "l": 0.01, "V_assay": 3e-6, "V_enzyme": 5e-8})],
     level="intermediate",
     extra_q=[{"q": "Enzyme assay activity: dA = 0.12, dt = 60 s, eps = 6220 L/(mol*cm), l = 1 cm, V_assay = 1 mL, V_enzyme = 20 uL, protein = 0.5 mg/mL", "like": 0}],
     weak="the same Beer-Lambert computation in customary units")


# ================================================================================================ SOLUTION BIOCHEMISTRY
def henderson_hasselbalch(pKa, A, HA):
    _fin(pKa)
    _pos("the base and acid concentrations", A, HA)
    pH = pKa + math.log10(A / HA)
    return _out(pH=pH, ratio=A / HA, fraction_base=A / (A + HA))


def _ref_hh(pKa, A, HA):
    Ka = 10.0 ** (-pKa)
    H = _brentq(lambda h: h * A / HA - Ka, 0.0, 1e3)
    return {"pH": -math.log10(H), "ratio": A / HA, "fraction_base": A / (A + HA)}


_add("cmb_henderson_hasselbalch",
     name="Buffer pH by the Henderson-Hasselbalch equation", topic="solution biochemistry / buffers",
     purpose="The pH of a buffer from the pKa and the concentrations of the conjugate base and the weak acid.",
     inputs=[_numin("pKa", "pKa of the weak acid"), _phys("A", "molar_concentration", "conjugate base concentration"),
             _phys("HA", "molar_concentration", "weak acid concentration")],
     outputs=[_o("pH", "number", "buffer pH"), _o("ratio", "number", "[A-]/[HA]"), _o("fraction_base", "number", "fraction in the base form")],
     primary="pH", conditions=["A, HA > 0", "buffer concentrations well above [H+] and [OH-]"],
     method="pH = pKa + log10([A-]/[HA])", latex=r"\mathrm{pH} = \mathrm{p}K_a + \log_{10}\frac{[\mathrm{A^-}]}{[\mathrm{HA}]}",
     run=henderson_hasselbalch, reference=_ref_hh,
     wrong=lambda pKa, A, HA: _out(pH=pKa + math.log10(HA / A), ratio=HA / A, fraction_base=HA / (A + HA)),
     rand=lambda rng: {"pKa": _sig(rng.uniform(2.0, 11.0), 3), "A": _sig(_lu(rng, 1.0, 500.0), 3), "HA": _sig(_lu(rng, 1.0, 500.0), 3)},
     phrases=("Henderson-Hasselbalch", "Henderson-Hasselbalch buffer pH", "Buffer pH"),
     examples=[_ex({"pKa": 7.21, "A": 60.0, "HA": 40.0}), _ex({"pKa": 4.76, "A": 10.0, "HA": 100.0})],
     level="beginner",
     extra_q=[{"q": "Buffer pH: pKa = 7.21, A = 0.06 M, HA = 40 mM", "like": 0}],
     weak="the acid-dissociation equilibrium solved for [H+] by brentq")


def buffer_preparation(C, V, pH, pKa, MW_acid=None, MW_base=None):
    _pos("C and V", C, V)
    _fin(pH, pKa)
    r = 10.0 ** (pH - pKa)
    cb = C * r / (1.0 + r)
    ca = C - cb
    out = {"C_base": cb, "C_acid": ca, "n_base": cb * V, "n_acid": ca * V, "ratio": r}
    if MW_acid is not None:
        _pos("the acid molar mass", MW_acid)
        out["m_acid_g"] = ca * V * MW_acid * 1000.0
    if MW_base is not None:
        _pos("the base molar mass", MW_base)
        out["m_base_g"] = cb * V * MW_base * 1000.0
    return _out(**out)


def _ref_buffer_prep(C, V, pH, pKa, MW_acid=None, MW_base=None):
    fb = _brentq(lambda f: pKa + math.log10(f / (1.0 - f)) - pH, 1e-300, 1.0 - 1e-16)
    cb, ca = C * fb, C * (1.0 - fb)
    out = {"C_base": cb, "C_acid": ca, "n_base": cb * V, "n_acid": ca * V, "ratio": fb / (1.0 - fb)}
    if MW_acid is not None:
        out["m_acid_g"] = ca * V * MW_acid * 1000.0
    if MW_base is not None:
        out["m_base_g"] = cb * V * MW_base * 1000.0
    return out


_add("cmb_buffer_preparation",
     name="Recipe for a buffer of given pH and concentration", topic="solution biochemistry / buffers",
     purpose="The concentrations, amounts and (with molar masses) masses of the weak acid and its conjugate base needed for a volume V of buffer at total concentration C and a target pH.",
     inputs=[_phys("C", "molar_concentration", "total buffer concentration"), _phys("V", "volume", "volume of buffer"),
             _numin("pH", "target pH"), _numin("pKa", "pKa of the buffer acid"),
             _phys("MW_acid", "molar_mass", "molar mass of the acid form (optional)", required=False),
             _phys("MW_base", "molar_mass", "molar mass of the base form (optional)", required=False)],
     outputs=[_o("C_base", "molar_concentration", "base-form concentration"), _o("C_acid", "molar_concentration", "acid-form concentration"),
              _o("n_base", "amount", "amount of base form"), _o("n_acid", "amount", "amount of acid form"), _o("ratio", "number", "[A-]/[HA]"),
              _o("m_acid_g", "number", "grams of the acid form (when MW_acid is given)"), _o("m_base_g", "number", "grams of the base form (when MW_base is given)")],
     primary="C_base", conditions=["C, V > 0", "Henderson-Hasselbalch valid (pH within about 1 of pKa)"],
     method="r = 10^(pH - pKa); C_base = C r/(1 + r); C_acid = C - C_base; n = C V; m = n M",
     latex=r"\frac{[\mathrm{A^-}]}{[\mathrm{HA}]} = 10^{\mathrm{pH} - \mathrm{p}K_a}",
     run=buffer_preparation, reference=_ref_buffer_prep,
     wrong=lambda C, V, pH, pKa, MW_acid=None, MW_base=None: buffer_preparation(C, V, 2.0 * pKa - pH, pKa, MW_acid, MW_base),
     rand=lambda rng: (lambda pKa: dict({"C": _sig(_lu(rng, 5.0, 500.0), 3), "V": _sig(_lu(rng, 1e-4, 2e-3), 3), "pKa": pKa,
                                         "pH": _sig(pKa + rng.choice([-1.0, 1.0]) * rng.uniform(0.05, 1.0), 3)},
                                        **({"MW_acid": _sig(rng.uniform(0.05, 0.3), 4), "MW_base": _sig(rng.uniform(0.05, 0.3), 4)} if rng.random() < 0.5 else {})))(_sig(rng.uniform(3.0, 10.0), 3)),
     phrases=("Buffer preparation", "Buffer recipe"),
     examples=[_ex({"C": 100.0, "V": 1e-3, "pH": 7.4, "pKa": 7.21, "MW_acid": 0.13799, "MW_base": 0.14196}),
               _ex({"C": 50.0, "V": 5e-4, "pH": 4.5, "pKa": 4.76})],
     level="intermediate",
     extra_q=[{"q": "Buffer recipe: C = 0.1 M, V = 1 L, pH = 7.4, pKa = 7.21, MW_acid = 137.99 g/mol, MW_base = 141.96 g/mol", "like": 0}])


def buffer_capacity(C, pKa, pH):
    _pos("C", C)
    _fin(pKa, pH)
    H = 10.0 ** (-pH)
    Ka = 10.0 ** (-pKa)
    Kw = 1e-14
    Cm = C * 1e-3                                # mol/L
    beta = math.log(10.0) * (H + Kw / H + Cm * Ka * H / (Ka + H) ** 2)
    beta_buffer = math.log(10.0) * Cm * Ka * H / (Ka + H) ** 2
    return _out(beta=beta * 1e3, beta_mM=beta * 1e3, beta_buffer_mM=beta_buffer * 1e3)


def _ref_buffer_capacity(C, pKa, pH):
    import sympy as sp
    p = sp.Symbol("p", real=True)
    H = 10 ** (-p)
    Ka = sp.Float(10.0 ** (-pKa), 30)
    Cm = sp.Float(C * 1e-3, 30)
    Cb = Cm * Ka / (Ka + H) + sp.Float(1e-14, 30) / H - H      # strong base needed (charge balance)
    beta = float(sp.diff(Cb, p).subs(p, sp.Float(pH, 30)).evalf(25))
    bb = float(sp.diff(Cm * Ka / (Ka + H), p).subs(p, sp.Float(pH, 30)).evalf(25))
    return {"beta": beta * 1e3, "beta_mM": beta * 1e3, "beta_buffer_mM": bb * 1e3}


_add("cmb_buffer_capacity",
     name="Buffer capacity of a weak-acid buffer", topic="solution biochemistry / buffers",
     purpose="The buffer capacity beta = dCb/dpH (strong base per unit pH change) of a monoprotic buffer of total concentration C at a pH, including water, and the buffer's own contribution (25 degC, Kw = 1e-14).",
     inputs=[_phys("C", "molar_concentration", "total buffer concentration"), _numin("pKa", "pKa of the buffer acid"), _numin("pH", "pH")],
     outputs=[_o("beta", "molar_concentration", "buffer capacity per pH unit"), _o("beta_mM", "number", "buffer capacity in mM per pH unit"),
              _o("beta_buffer_mM", "number", "contribution of the buffer alone in mM per pH unit")],
     primary="beta", conditions=["C > 0", "monoprotic acid", "Kw = 1e-14 (25 degC), ideal solution"],
     method="beta = 2.303 ([H+] + Kw/[H+] + C Ka [H+] / (Ka + [H+])^2)",
     latex=r"\beta = 2.303\left([\mathrm{H^+}] + \frac{K_w}{[\mathrm{H^+}]} + \frac{C K_a[\mathrm{H^+}]}{(K_a + [\mathrm{H^+}])^2}\right)",
     run=buffer_capacity, reference=_ref_buffer_capacity,
     wrong=lambda C, pKa, pH: (lambda o: dict(o, beta=_r(o["beta"] / math.log(10.0)), beta_mM=_r(o["beta_mM"] / math.log(10.0))))(buffer_capacity(C, pKa, pH)),
     rand=lambda rng: (lambda pKa: {"C": _sig(_lu(rng, 1.0, 500.0), 3), "pKa": pKa, "pH": _sig(pKa + rng.uniform(-2.0, 2.0), 3)})(_sig(rng.uniform(3.0, 10.0), 3)),
     phrases=("Buffer capacity",),
     examples=[_ex({"C": 100.0, "pKa": 7.21, "pH": 7.21}), _ex({"C": 20.0, "pKa": 4.76, "pH": 5.5})],
     level="advanced")


def buffer_after_addition(C, V, pH0, pKa, n_add, added):
    _pos("C and V", C, V)
    _fin(pH0, pKa)
    _pos("the amount added", n_add)
    _need(added in ("acid", "base"), "added: acid or base (strong)")
    r = 10.0 ** (pH0 - pKa)
    nb = C * V * r / (1.0 + r)
    na = C * V - nb
    if added == "base":
        _need(n_add < na, "the base added exceeds the acid form: the buffer is exhausted")
        nb2, na2 = nb + n_add, na - n_add
    else:
        _need(n_add < nb, "the acid added exceeds the base form: the buffer is exhausted")
        nb2, na2 = nb - n_add, na + n_add
    pH = pKa + math.log10(nb2 / na2)
    return _out(pH=pH, delta_pH=pH - pH0, n_base=nb2, n_acid=na2)


def _ref_buffer_add(C, V, pH0, pKa, n_add, added):
    fb = _brentq(lambda f: pKa + math.log10(f / (1.0 - f)) - pH0, 1e-300, 1.0 - 1e-16)
    nb, na = C * V * fb, C * V * (1.0 - fb)
    s = 1.0 if added == "base" else -1.0
    nb2, na2 = nb + s * n_add, na - s * n_add
    H = _brentq(lambda h: h * nb2 / na2 - 10.0 ** (-pKa), 0.0, 1e3)
    pH = -math.log10(H)
    return {"pH": pH, "delta_pH": pH - pH0, "n_base": nb2, "n_acid": na2}


def _rand_buffer_add(rng):
    pKa = _sig(rng.uniform(3.0, 10.0), 3)
    C, V = _sig(_lu(rng, 10.0, 500.0), 3), _sig(_lu(rng, 1e-4, 1e-3), 3)
    pH0 = _sig(pKa + rng.uniform(-0.8, 0.8), 3)
    return {"C": C, "V": V, "pH0": pH0, "pKa": pKa, "n_add": _sig(C * V * rng.uniform(0.01, 0.2), 3), "added": rng.choice(["acid", "base"])}


_add("cmb_buffer_after_addition",
     name="Buffer pH after adding strong acid or base", topic="solution biochemistry / buffers",
     purpose="The pH of a buffer (total concentration C, volume V, initial pH0) after adding an amount of strong acid or strong base, by Henderson-Hasselbalch on the new amounts.",
     inputs=[_phys("C", "molar_concentration", "total buffer concentration"), _phys("V", "volume", "buffer volume"),
             _numin("pH0", "initial pH"), _numin("pKa", "pKa of the buffer acid"),
             _phys("n_add", "amount", "amount of strong acid or base added"), _choice("added", ("acid", "base"), "strong acid or strong base")],
     outputs=[_o("pH", "number", "new pH"), _o("delta_pH", "number", "change in pH"),
              _o("n_base", "amount", "base form after the addition"), _o("n_acid", "amount", "acid form after the addition")],
     primary="pH", conditions=["C, V, n_add > 0", "the addition smaller than the form it consumes", "Henderson-Hasselbalch valid"],
     method="split C V by the initial ratio 10^(pH0 - pKa); base converts HA to A- (acid the reverse); pH = pKa + log10(n_A/n_HA)",
     latex=r"\mathrm{pH} = \mathrm{p}K_a + \log_{10}\frac{n_{A^-} \pm n}{n_{HA} \mp n}",
     run=buffer_after_addition, reference=_ref_buffer_add,
     wrong=lambda C, V, pH0, pKa, n_add, added: buffer_after_addition(C, V, pH0, pKa, n_add, "acid" if added == "base" else "base"),
     rand=_rand_buffer_add,
     phrases=("Buffer pH after adding acid or base", "Buffer after addition"),
     examples=[_ex({"C": 100.0, "V": 1e-4, "pH0": 7.21, "pKa": 7.21, "n_add": 1e-3, "added": "base"}),
               _ex({"C": 50.0, "V": 5e-4, "pH0": 4.76, "pKa": 4.76, "n_add": 2e-3, "added": "acid"})],
     level="intermediate",
     extra_q=[{"q": "Buffer after addition: C = 100 mM, V = 100 mL, pH0 = 7.21, pKa = 7.21, n_add = 1 mmol, added = strong base", "outcome": "INVALID_INPUT"},
              {"q": "Buffer after addition: C = 100 mM, V = 100 mL, pH0 = 7.21, pKa = 7.21, n_add = 1 mmol, added = Base", "like": 0}])


def c1v1_dilution(C1, C2, V2):
    _pos("C1, C2 and V2", C1, C2, V2)
    _need(C2 <= C1, "the final concentration cannot exceed the stock concentration")
    V1 = C2 * V2 / C1
    return _out(V1=V1, V1_uL=V1 * 1e9, V_diluent=V2 - V1, dilution_factor=C1 / C2)


def _ref_c1v1(C1, C2, V2):
    from fractions import Fraction as Fr
    V1 = Fr(C2) * Fr(V2) / Fr(C1)
    return {"V1": float(V1), "V1_uL": float(V1 * 10 ** 9), "V_diluent": float(Fr(V2) - V1), "dilution_factor": float(Fr(C1) / Fr(C2))}


_add("cmb_c1v1_dilution",
     name="Dilution of a stock solution (C1 V1 = C2 V2)", topic="solution biochemistry / dilutions",
     purpose="The volume of stock and of diluent needed to make a volume V2 at concentration C2 from a stock at C1.",
     inputs=[_phys("C1", "molar_concentration", "stock concentration"), _phys("C2", "molar_concentration", "final concentration"),
             _phys("V2", "volume", "final volume")],
     outputs=[_o("V1", "volume", "volume of stock"), _o("V1_uL", "number", "volume of stock in uL"),
              _o("V_diluent", "volume", "volume of diluent"), _o("dilution_factor", "number", "dilution factor C1/C2")],
     primary="V1", conditions=["C1, C2, V2 > 0", "C2 <= C1", "volumes additive"],
     method="V1 = C2 V2 / C1; diluent = V2 - V1", latex=r"C_1V_1 = C_2V_2",
     run=c1v1_dilution, reference=_ref_c1v1,
     wrong=lambda C1, C2, V2: (lambda V1: _out(V1=V1, V1_uL=V1 * 1e9, V_diluent=V2 - V1, dilution_factor=C2 / C1))(C1 * V2 / C2),
     rand=lambda rng: (lambda c1: {"C1": c1, "C2": _sig(c1 * _lu(rng, 1e-4, 0.9), 3), "V2": _sig(_lu(rng, 1e-6, 1e-3), 3)})(_sig(_lu(rng, 1.0, 5000.0), 3)),
     phrases=("C1V1 dilution", "Dilution of a stock", "C1V1 = C2V2"),
     examples=[_ex({"C1": 1000.0, "C2": 50.0, "V2": 1e-5}), _ex({"C1": 5000.0, "C2": 150.0, "V2": 5e-4})],
     level="beginner",
     extra_q=[{"q": "C1V1 dilution: C1 = 1 M, C2 = 50 mM, V2 = 10 mL", "like": 0}],
     weak="the same proportion in exact rational arithmetic")


def serial_dilution(C0, f, n, V=None):
    _pos("C0", C0)
    _need(_isnum(f) and f > 1, "the dilution factor per step must exceed 1")
    _need(isinstance(n, int) and 1 <= n <= 30, "the number of steps must be a whole number from 1 to 30")
    concs = []
    c = C0
    for _ in range(n):
        c = c / f
        concs.append(c)
    out = {"C_final": concs[-1], "total_dilution": f ** n, "concentrations_mM": concs}
    if V is not None:
        _pos("the tube volume V", V)
        out["transfer"] = V / f
        out["diluent"] = V - V / f
    return _out(**out)


def _ref_serial(C0, f, n, V=None):
    import numpy as np
    concs = C0 * np.cumprod(np.full(n, 1.0 / f))
    out = {"C_final": float(concs[-1]), "total_dilution": float(C0 / concs[-1]), "concentrations_mM": [float(x) for x in concs]}
    if V is not None:
        out["transfer"] = V / f
        out["diluent"] = V * (f - 1.0) / f
    return out


_add("cmb_serial_dilution",
     name="Serial dilution series", topic="solution biochemistry / dilutions",
     purpose="The concentrations after each step of a serial dilution with a constant factor, the final concentration and total dilution, and (with the tube volume) the transfer and diluent volumes per step.",
     inputs=[_phys("C0", "molar_concentration", "starting concentration"), _numin("f", "dilution factor per step (e.g. 10)"),
             _intin("n", "number of dilution steps", lo=1, hi=30, words=("steps",)),
             _phys("V", "volume", "final volume in each tube (optional)", required=False, show=_UL)],
     outputs=[_o("C_final", "molar_concentration", "concentration after n steps"), _o("total_dilution", "number", "total dilution factor f^n"),
              _o("concentrations_mM", "number", "concentration after each step in mM"),
              _o("transfer", "volume", "volume carried to the next tube (when V is given)"), _o("diluent", "volume", "diluent per tube (when V is given)")],
     primary="C_final", conditions=["C0 > 0", "f > 1", "1 <= n <= 30"],
     method="C_k = C0 / f^k; transfer = V/f, diluent = V - V/f", latex=r"C_k = \frac{C_0}{f^k}",
     run=serial_dilution, reference=_ref_serial,
     wrong=lambda C0, f, n, V=None: serial_dilution(C0, f, n + 1, V) if n < 30 else serial_dilution(C0, f, n - 1, V),
     rand=lambda rng: dict({"C0": _sig(_lu(rng, 0.01, 1000.0), 3), "f": rng.choice([2.0, 3.0, 5.0, 10.0, 4.0]), "n": rng.randint(1, 10)},
                           **({"V": _sig(_lu(rng, 1e-7, 1e-5), 3)} if rng.random() < 0.5 else {})),
     phrases=("Serial dilution", "Serial dilution series"),
     examples=[_ex({"C0": 10.0, "f": 10.0, "n": 4}), _ex({"C0": 1.0, "f": 2.0, "n": 6, "V": 1e-7})],
     level="beginner",
     extra_q=[{"q": "Serial dilution: C0 = 10 mM, f = 10, n = 4 steps", "like": 0}],
     weak="numpy cumulative product of the same factors")


def mass_for_molarity(C, V, MW):
    _pos("C, V and MW", C, V, MW)
    m = C * V * MW
    return _out(m=m, m_g=m * 1000.0, m_mg=m * 1e6, n=C * V)


def _ref_mass_molarity(C, V, MW):
    from fractions import Fraction as Fr
    n = Fr(C) * Fr(V)
    m = n * Fr(MW)
    return {"m": float(m), "m_g": float(m * 1000), "m_mg": float(m * 10 ** 6), "n": float(n)}


_add("cmb_mass_for_molarity",
     name="Mass of solute for a solution of given molarity", topic="solution biochemistry / molarity",
     purpose="The mass of a compound to weigh out for a volume V of solution at molar concentration C.",
     inputs=[_phys("C", "molar_concentration", "desired concentration"), _phys("V", "volume", "volume of solution"),
             _phys("MW", "molar_mass", "molar mass of the compound")],
     outputs=[_o("m", "mass", "mass to weigh"), _o("m_g", "number", "mass in grams"), _o("m_mg", "number", "mass in milligrams"),
              _o("n", "amount", "amount of substance")],
     primary="m", conditions=["C, V, MW > 0"], method="m = C V MW", latex=r"m = C\,V\,M",
     run=mass_for_molarity, reference=_ref_mass_molarity,
     wrong=lambda C, V, MW: _out(m=C * V / MW, m_g=C * V / MW * 1000.0, m_mg=C * V / MW * 1e6, n=C * V),
     rand=lambda rng: {"C": _sig(_lu(rng, 0.01, 2000.0), 3), "V": _sig(_lu(rng, 1e-6, 2e-3), 3), "MW": _sig(_lu(rng, 0.02, 150.0), 4)},
     phrases=("Mass for a molar solution", "Mass of solute for a molarity", "Mass for molarity"),
     examples=[_ex({"C": 150.0, "V": 5e-4, "MW": 0.05844}), _ex({"C": 0.5, "V": 1e-4, "MW": 66.43})],
     level="beginner",
     extra_q=[{"q": "Mass for molarity: C = 0.15 M, V = 500 mL, MW = 58.44 g/mol", "like": 0}],
     weak="the same product in exact rational arithmetic")


def molarity_from_mass(m, MW, V):
    _pos("m, MW and V", m, MW, V)
    C = m / MW / V
    return _out(C=C, C_mM=C, C_M=C / 1000.0, n=m / MW)


def _ref_molarity_mass(m, MW, V):
    from fractions import Fraction as Fr
    n = Fr(m) / Fr(MW)
    C = n / Fr(V)
    return {"C": float(C), "C_mM": float(C), "C_M": float(C / 1000), "n": float(n)}


_add("cmb_molarity_from_mass",
     name="Molar concentration from mass and volume", topic="solution biochemistry / molarity",
     purpose="The molar concentration of a solution made by dissolving a mass of compound to a final volume V.",
     inputs=[_phys("m", "mass", "mass of solute"), _phys("MW", "molar_mass", "molar mass"), _phys("V", "volume", "final volume")],
     outputs=[_o("C", "molar_concentration", "molar concentration"), _o("C_mM", "number", "concentration in mM"),
              _o("C_M", "number", "concentration in M"), _o("n", "amount", "amount of substance")],
     primary="C", conditions=["m, MW, V > 0"], method="C = m / (MW V)", latex=r"C = \frac{m}{M\,V}",
     run=molarity_from_mass, reference=_ref_molarity_mass,
     wrong=lambda m, MW, V: _out(C=m * MW / V, C_mM=m * MW / V, C_M=m * MW / V / 1000.0, n=m * MW),
     rand=lambda rng: {"m": _sig(_lu(rng, 1e-7, 1e-1), 3), "MW": _sig(_lu(rng, 0.02, 150.0), 4), "V": _sig(_lu(rng, 1e-6, 2e-3), 3)},
     phrases=("Molarity from mass", "Molar concentration from mass"),
     examples=[_ex({"m": 0.00292, "MW": 0.05844, "V": 5e-4}), _ex({"m": 1e-5, "MW": 66.43, "V": 1e-6})],
     level="beginner",
     extra_q=[{"q": "Molarity from mass: m = 2.92 g, MW = 58.44 g/mol, V = 500 mL", "like": 0}],
     weak="the same quotient in exact rational arithmetic")


def percent_to_molarity(p, MW):
    _frac("the per cent w/v as a fraction", p, lo_open=True, hi_open=True)
    _pos("MW", MW)
    rho = p * 1000.0                             # g/mL fraction -> kg/m^3 (1 % w/v = 10 g/L)
    C = rho / MW
    return _out(C=C, C_mM=C, C_M=C / 1000.0, mass_concentration=rho, g_per_L=rho)


def _ref_percent(p, MW):
    g_per_100mL = p * 100.0
    g_per_L = g_per_100mL * 10.0
    C_M = g_per_L / (MW * 1000.0)
    return {"C": C_M * 1000.0, "C_mM": C_M * 1000.0, "C_M": C_M, "mass_concentration": g_per_L, "g_per_L": g_per_L}


_add("cmb_percent_wv_to_molarity",
     name="Molarity of a per cent (w/v) solution", topic="solution biochemistry / molarity",
     purpose="The molar concentration and grams per litre of a solution given in per cent weight per volume (g per 100 mL), e.g. 0.9 % NaCl.",
     inputs=[_numin("p", "concentration in per cent w/v (written with %, e.g. 0.9%)", pct_required=True), _phys("MW", "molar_mass", "molar mass of the solute")],
     outputs=[_o("C", "molar_concentration", "molar concentration"), _o("C_mM", "number", "concentration in mM"), _o("C_M", "number", "concentration in M"),
              _o("mass_concentration", "mass_concentration", "mass concentration"), _o("g_per_L", "number", "grams per litre")],
     primary="C", conditions=["0 < p < 100 %", "MW > 0"],
     method="g/L = 10 x (% w/v); C = (g/L) / MW", latex=r"C = \frac{10\times(\%\,w/v)}{M}",
     run=percent_to_molarity, reference=_ref_percent,
     wrong=lambda p, MW: (lambda r: _out(C=r / MW, C_mM=r / MW, C_M=r / MW / 1000.0, mass_concentration=r, g_per_L=r))(p * 100.0),
     rand=lambda rng: {"p": _sig(_lu(rng, 1e-4, 0.3), 3), "MW": _sig(_lu(rng, 0.02, 1.0), 4)},
     phrases=("Per cent w/v to molarity", "Percent w/v to molarity"),
     examples=[_ex({"p": 0.009, "MW": 0.05844}), _ex({"p": 0.05, "MW": 0.18016})],
     level="beginner",
     extra_q=[{"q": "Percent w/v to molarity: p = 0.9%, MW = 58.44 g/mol", "like": 0},
              {"q": "Percent w/v to molarity: p = 0.9, MW = 58.44 g/mol", "outcome": "UNRECOGNIZED_FORMAT"}])


def ionic_strength(c, z):
    _need(len(c) == len(z) and len(c) >= 1, "give one charge per concentration")
    for v in c:
        _need(_isnum(v) and v >= 0, "concentrations must be zero or positive")
    for v in z:
        _need(_isnum(v) and v == int(v) and v != 0, "charges must be non-zero whole numbers")
    s = 0.0
    for k in range(len(c)):
        s += c[k] * z[k] * z[k]
    I = 0.5 * s
    return _out(I=I, I_mM=I, I_M=I / 1000.0)


def _ref_ionic(c, z):
    import numpy as np
    I = 0.5 * float(np.sum(np.array(c, dtype=float) * np.array(z, dtype=float) ** 2))
    return {"I": I, "I_mM": I, "I_M": I / 1000.0}


def _rand_ionic(rng):
    n = rng.randint(1, 5)
    return {"c": [_sig(_lu(rng, 0.1, 500.0), 3) for _ in range(n)], "z": [float(rng.choice([1, -1, 2, -2, 3, -1, 1])) for _ in range(n)]}


_add("cmb_ionic_strength",
     name="Ionic strength of a solution", topic="solution biochemistry / ionic strength",
     purpose="The ionic strength I = (1/2) sum c z^2 of a solution from the concentration and charge of each ion.",
     inputs=[_vecin("c", "ion concentrations in mM (one per ion)", 1, 30), _vecin("z", "charge of each ion (e.g. 1, -1, 2)", 1, 30)],
     outputs=[_o("I", "molar_concentration", "ionic strength"), _o("I_mM", "number", "ionic strength in mM"), _o("I_M", "number", "ionic strength in M")],
     primary="I", conditions=["one charge per concentration", "charges non-zero whole numbers"],
     method="I = 1/2 sum c_i z_i^2", latex=r"I = \tfrac{1}{2}\sum_i c_i z_i^2",
     run=ionic_strength, reference=_ref_ionic,
     wrong=lambda c, z: (lambda s: _out(I=s, I_mM=s, I_M=s / 1000.0))(sum(c[k] * abs(z[k]) for k in range(len(c)))),
     rand=_rand_ionic,
     phrases=("Ionic strength",),
     examples=[_ex({"c": [150.0, 150.0], "z": [1.0, -1.0]}), _ex({"c": [10.0, 20.0, 5.0], "z": [2.0, -1.0, 1.0]})],
     level="intermediate",
     weak="numpy sum of the same terms")


def centrifuge_rcf(rpm, r):
    _pos("the rotor speed and the radius", rpm, r)
    rcf = rpm * rpm * r / G0
    return _out(RCF=rcf, rpm_value=rpm / RPM)


def _ref_rcf(rpm, r):
    sc = _sc()
    rev_per_min = rpm * 60.0 / (2.0 * sc.pi)
    omega = rev_per_min * 2.0 * sc.pi / sc.minute
    return {"RCF": omega ** 2 * r / sc.g, "rpm_value": rev_per_min}


_add("cmb_centrifuge_rcf",
     name="Relative centrifugal force from rotor speed", topic="solution biochemistry / centrifugation",
     purpose="The relative centrifugal force (x g) at radius r for a rotor speed in rpm.",
     inputs=[_phys("rpm", "angular_frequency", "rotor speed"), _phys("r", "length", "radius from the rotor axis", show=_CM)],
     outputs=[_o("RCF", "number", "relative centrifugal force in multiples of g"), _o("rpm_value", "number", "rotor speed in rpm")],
     primary="RCF", conditions=["rpm, r > 0"], method="RCF = omega^2 r / g = 1.118e-5 r(cm) rpm^2",
     latex=r"\mathrm{RCF} = \frac{\omega^2 r}{g} = 1.118\times10^{-5}\,r_{cm}\,N_{rpm}^2",
     run=centrifuge_rcf, reference=_ref_rcf,
     wrong=lambda rpm, r: _out(RCF=(rpm / RPM) ** 2 * r / G0, rpm_value=rpm / RPM),
     rand=lambda rng: {"rpm": _sig(_lu(rng, 500.0, 100000.0), 4) * RPM, "r": _sig(rng.uniform(0.04, 0.2), 3)},
     phrases=("Relative centrifugal force", "Centrifuge RCF"),
     examples=[_ex({"rpm": 3000.0 * RPM, "r": 0.1}), _ex({"rpm": 14000.0 * RPM, "r": 0.084})],
     level="beginner",
     weak="the same relation evaluated with scipy.constants")


def centrifuge_rpm(RCF, r):
    _pos("the RCF and the radius", RCF, r)
    w = math.sqrt(RCF * G0 / r)
    return _out(omega=w, rpm_value=w / RPM)


def _ref_centrifuge_rpm(RCF, r):
    sc = _sc()
    w = _brentq(lambda om: om * om * r / sc.g - RCF, 0.0, 1e6)
    return {"omega": w, "rpm_value": w * sc.minute / (2.0 * sc.pi)}


_add("cmb_centrifuge_rpm",
     name="Rotor speed for a required centrifugal force", topic="solution biochemistry / centrifugation",
     purpose="The rotor speed (rpm) that gives a required relative centrifugal force (x g) at radius r.",
     inputs=[_numin("RCF", "required relative centrifugal force in multiples of g", words=("g", "x g", "xg")),
             _phys("r", "length", "radius from the rotor axis", show=_CM)],
     outputs=[_o("omega", "angular_frequency", "angular speed"), _o("rpm_value", "number", "rotor speed in rpm")],
     primary="omega", conditions=["RCF, r > 0"], method="rpm = sqrt(RCF g / r) x 60 / (2 pi)",
     latex=r"N_{rpm} = \frac{60}{2\pi}\sqrt{\frac{\mathrm{RCF}\,g}{r}}",
     run=centrifuge_rpm, reference=_ref_centrifuge_rpm,
     wrong=lambda RCF, r: (lambda w: _out(omega=w, rpm_value=w / RPM))(math.sqrt(RCF / r)),
     rand=lambda rng: {"RCF": _sig(_lu(rng, 100.0, 5e5), 3), "r": _sig(rng.uniform(0.04, 0.2), 3)},
     phrases=("Centrifuge speed for an RCF", "Rotor speed for an RCF"),
     examples=[_ex({"RCF": 1000.0, "r": 0.1}), _ex({"RCF": 16000.0, "r": 0.084})],
     level="beginner",
     extra_q=[{"q": "Rotor speed for an RCF: RCF = 1000 g, r = 10 cm", "like": 0}])


def sedimentation_time(s, rpm, r_min, r_max):
    _pos("s, the rotor speed and the radii", s, rpm, r_min, r_max)
    _need(r_max > r_min, "r_max must exceed r_min")
    sec = s * 1e-13
    t = math.log(r_max / r_min) / (rpm * rpm * sec)
    k = math.log(r_max / r_min) / (rpm * rpm) / HOUR * 1e13
    return _out(t=t, t_h=t / HOUR, k_factor=k)


def _ref_sedimentation(s, rpm, r_min, r_max):
    from scipy.integrate import solve_ivp
    sec = s * 1e-13
    w2s = rpm * rpm * sec
    ev = lambda t, y: y[0] - r_max
    ev.terminal = True
    t_guess = math.log(r_max / r_min) / w2s
    sol = solve_ivp(lambda t, y: [w2s * y[0]], (0.0, 2.0 * t_guess), [r_min], method="DOP853", rtol=1e-13, atol=1e-18, events=ev)
    t = float(sol.t_events[0][0])
    return {"t": t, "t_h": t / 3600.0, "k_factor": t * s / 3600.0}


_add("cmb_sedimentation_time",
     name="Time to pellet a particle in a centrifuge (k-factor)", topic="solution biochemistry / centrifugation",
     purpose="The time to sediment a particle of sedimentation coefficient s from r_min to r_max at a rotor speed, and the rotor's k-factor (t in hours = k / s).",
     inputs=[_numin("s", "sedimentation coefficient in svedbergs", words=("S", "svedberg", "svedbergs")),
             _phys("rpm", "angular_frequency", "rotor speed"), _phys("r_min", "length", "starting (minimum) radius", show=_CM),
             _phys("r_max", "length", "radius of the pellet (maximum radius)", show=_CM)],
     outputs=[_o("t", "time", "sedimentation time"), _o("t_h", "number", "time in hours"), _o("k_factor", "number", "k-factor (t_h = k / s)")],
     primary="t", conditions=["s, rpm > 0", "0 < r_min < r_max", "constant s (no diffusion)"],
     method="dr/dt = s omega^2 r: t = ln(r_max/r_min) / (omega^2 s); k = ln(r_max/r_min) / omega^2 x 1e13 / 3600",
     latex=r"t = \frac{\ln(r_{max}/r_{min})}{\omega^2 s},\quad k = \frac{\ln(r_{max}/r_{min})}{\omega^2}\cdot\frac{10^{13}}{3600}",
     run=sedimentation_time, reference=_ref_sedimentation,
     wrong=lambda s, rpm, r_min, r_max: (lambda t: _out(t=t, t_h=t / HOUR, k_factor=t * s / HOUR))((r_max - r_min) / (rpm * rpm * s * 1e-13 * r_min)),
     rand=lambda rng: (lambda rmin: {"s": _sig(_lu(rng, 2.0, 5000.0), 3), "rpm": _sig(_lu(rng, 5000.0, 100000.0), 3) * RPM, "r_min": rmin,
                                     "r_max": _sig(rmin + rng.uniform(0.01, 0.06), 3)})(_sig(rng.uniform(0.03, 0.08), 3)),
     phrases=("Sedimentation time", "Pelleting time", "Centrifuge k-factor"),
     examples=[_ex({"s": 70.0, "rpm": 40000.0 * RPM, "r_min": 0.05, "r_max": 0.1}), _ex({"s": 4.4, "rpm": 60000.0 * RPM, "r_min": 0.06, "r_max": 0.085})],
     level="advanced",
     extra_q=[{"q": "Sedimentation time: s = 70 S, rpm = 40000 rpm, r_min = 5 cm, r_max = 10 cm", "like": 0}])


# ================================================================================================ levels (final placement)
for _k in ("cmb_membrane_permeability", "cmb_monod_growth_rate", "cmb_transformation_efficiency", "cmb_tm_basic",
           "cmb_protein_concentration_a280", "cmb_qpcr_delta_delta_ct", "cmb_transcription_time", "cmb_translation_time",
           "cmb_substrate_for_fraction_of_vmax", "cmb_fraction_bound", "cmb_serial_dilution", "cmb_percent_wv_to_molarity",
           "cmb_centrifuge_rcf", "cmb_centrifuge_rpm"):
    LEVELS[_k] = "intermediate"
for _k in ("cmb_cell_equilibration", "cmb_cell_volume_osmotic", "cmb_logistic_time_to_reach", "cmb_oligo_molecular_weight",
           "cmb_genome_coverage", "cmb_library_size", "cmb_qpcr_pfaffl_ratio", "cmb_expression_steady_state",
           "cmb_hill_ligand_for_fraction", "cmb_enzyme_assay_activity", "cmb_buffer_after_addition", "cmb_stokes_einstein",
           "cmb_mrna_half_life_fit", "cmb_growth_rate_fit", "cmb_binding_free_energy", "cmb_qpcr_standard_curve"):
    LEVELS[_k] = "advanced"
for _k in ("cmb_donnan_equilibrium", "cmb_induction_dynamics", "cmb_mixed_inhibition", "cmb_cell_cycle_age_corrected",
           "cmb_buffer_capacity", "cmb_ligand_depletion_binding"):
    LEVELS[_k] = "expert"


# ================================================================================================ worked-example values
# computed by run() and checked against every reference on the box before certification (scipy, Biopython, numpy, sympy)
_EXV = {
    'cmb_fick_flux': [{'J': 0.0008, 'J_nmol_cm2_s': 80.0}, {'J': 3.75e-05, 'J_nmol_cm2_s': 3.75}],
    'cmb_rms_displacement': [{'rms': 7.74596669241e-05, 'rms_um': 77.4596669241, 'msd': 6e-09}, {'rms': 1.54919333848e-05, 'rms_um': 15.4919333848, 'msd': 2.4e-10}],
    'cmb_diffusion_time': [{'t': 0.0166666666667, 't_min': 0.000277777777778}, {'t': 1000.0, 't_min': 16.6666666667}],
    'cmb_stokes_einstein': [{'D': 9.81492438178e-11, 'D_um2_s': 98.1492438178, 'friction': 4.19402619254e-11}, {'D': 6.49061577187e-12, 'D_um2_s': 6.49061577187, 'friction': 6.59734457254e-10}],
    'cmb_membrane_permeability': [{'P': 0.0002, 'P_cm_s': 0.02, 'J': 0.0018}, {'P': 0.025, 'P_cm_s': 2.5, 'J': -0.0375}],
    'cmb_cell_equilibration': [{'C': 5.93430340259, 'tau': 33.3333333333, 't_half': 23.1049060187}, {'C': 2.94700902384, 'tau': 83.3333333333, 't_half': 57.7622650467}],
    'cmb_osmotic_pressure': [{'pi': 773619.174306, 'pi_atm': 7.635027627, 'pi_kPa': 773.619174306}, {'pi': 743687.108881, 'pi_atm': 7.33962110911, 'pi_kPa': 743.687108881}],
    'cmb_osmolarity_tonicity': [{'osmolarity_mOsm': 300.0, 'difference_mOsm': 0.0, 'tonicity': 'isotonic'}, {'osmolarity_mOsm': 250.0, 'difference_mOsm': -40.0, 'tonicity': 'hypotonic'}],
    'cmb_cell_volume_osmotic': [{'V': 6.3e-17, 'V_fL': 63.0, 'ratio': 0.7}, {'V': 3.49333333333e-15, 'V_fL': 3493.33333333, 'ratio': 1.74666666667}],
    'cmb_nernst_potential': [{'E': -0.0890586940367, 'E_mV': -89.0586940367}, {'E': 0.132343567921, 'E_mV': 132.343567921}],
    'cmb_donnan_equilibrium': [{'K_in': 161.803398875, 'Cl_in': 61.803398875, 'ratio': 1.61803398875, 'E': -0.0123635728893, 'E_mV': -12.3635728893, 'excess_mOsm': 123.60679775}, {'K_in': 208.113883008, 'Cl_in': 108.113883008, 'ratio': 1.38742588672, 'E': -0.00875164854174, 'E_mV': -8.75164854174, 'excess_mOsm': 66.2277660168}],
    'cmb_channel_current': [{'I': 3e-10, 'I_pA': 300.0, 'G': 6e-09, 'i_single': 1e-12}, {'I': -2e-10, 'I_pA': -200.0, 'G': 4e-09, 'i_single': -5e-12}],
    'cmb_diffusion_limited_rate': [{'k': 20811024.4875, 'k_per_M_s': 20811024487.5}, {'k': 5448704.5931, 'k_per_M_s': 5448704593.1}],
    'cmb_frap_diffusion': [{'D': 4.47295575496e-13, 'D_um2_s': 0.447295575496, 'tauD': 0.558914538162, 'gamma': 1.11782907632}, {'D': 1.16483222785e-13, 'D_um2_s': 0.116483222785, 'tauD': 13.4139489159, 'gamma': 1.11782907632}],
    'cmb_exponential_growth': [{'N': 64000.0, 'generations': 6.0, 'fold': 64.0, 'mu': 0.000577622650467, 'mu_per_h': 2.07944154168}, {'N': 1600000.0, 'generations': 3.0, 'fold': 8.0, 'mu': 8.02253681204e-06, 'mu_per_h': 0.0288811325233}],
    'cmb_doubling_time_from_counts': [{'td': 7200.0, 'td_h': 2.0, 'generations': 4.0, 'mu': 9.62704417444e-05, 'mu_per_h': 0.34657359028}, {'td': 86400.0, 'td_h': 24.0, 'generations': 2.0, 'mu': 8.02253681204e-06, 'mu_per_h': 0.0288811325233}],
    'cmb_growth_rate_fit': [{'mu_per_h': 0.695378839924, 'td_h': 0.996790728684, 'N0_fit': 998.435109896, 'r2': 0.999807398525}, {'mu_per_h': 0.344247589498, 'td_h': 2.01351353417, 'N0_fit': 0.0521970559445, 'r2': 0.997405590444}],
    'cmb_logistic_growth': [{'N': 11813193.4853, 'dNdt': 2083.53561642, 'dNdt_per_h': 7500728.2191, 't_inflection': 46051.2018349, 't_inflection_h': 12.7920005097}, {'N': 241506291.576, 'dNdt': 18318.1002705, 'dNdt_per_h': 65945160.9738, 't_inflection': 29444.3897917, 't_inflection_h': 8.17899716435}],
    'cmb_logistic_time_to_reach': [{'t': 46051.2018349, 't_h': 12.7920005097}, {'t': 228149.298886, 't_h': 63.3748052462}],
    'cmb_monod_growth_rate': [{'mu': 0.000166666666667, 'mu_per_h': 0.6, 'fraction': 0.833333333333, 'td_h': 1.15524530093}, {'mu': 2.5e-05, 'mu_per_h': 0.09, 'fraction': 0.25, 'td_h': 7.70163533955}],
    'cmb_monod_batch_time': [{'t': 18954.4960144, 't_h': 5.26513778177, 'S': 2.2, 'S_g_L': 2.2}, {'t': 37059.9359842, 't_h': 10.2944266623, 'S': 2.625, 'S_g_L': 2.625}],
    'cmb_cell_cycle_phase_durations': [{'G1_h': 10.8, 'S_h': 8.4, 'G2_h': 3.6, 'M_h': 1.2}, {'G1_h': 10.0, 'S_h': 6.0, 'G2_h': 3.2, 'M_h': 0.8}],
    'cmb_cell_cycle_age_corrected': [{'G1_h': 8.82556282801, 'S_h': 8.86161143198, 'G2_h': 4.62348187062, 'M_h': 1.68934386939}, {'G1_h': 8.30074998558, 'S_h': 6.43856189775, 'G2_h': 4.12901754935, 'M_h': 1.13167056733}],
    'cmb_hemocytometer_count': [{'mean_count': 50.0, 'cells_per_mL': 1000000.0}, {'mean_count': 115.0, 'cells_per_mL': 11500000.0, 'total_cells': 115000000.0}],
    'cmb_plate_count': [{'cfu_per_mL': 1500000000.0, 'log10_cfu_per_mL': 9.17609125906}, {'cfu_per_mL': 420000.0, 'log10_cfu_per_mL': 5.6232492904}],
    'cmb_transformation_efficiency': [{'efficiency': 2000000.0, 'log10_efficiency': 6.30102999566, 'DNA_plated_ug': 0.0001}, {'efficiency': 178500.0, 'log10_efficiency': 5.25163822045, 'DNA_plated_ug': 0.00047619047619}],
    'cmb_cell_geometry': [{'V': 5.23598775598e-16, 'V_fL': 523.598775598, 'A': 3.14159265359e-10, 'A_um2': 314.159265359, 'SA_V_per_um': 0.6}, {'V': 5.23598775598e-19, 'V_fL': 0.523598775598, 'A': 3.14159265359e-12, 'A_um2': 3.14159265359, 'SA_V_per_um': 6.0}],
    'cmb_molecules_per_cell': [{'N': 0.602214076, 'one_molecule': 1.66053906717e-06, 'one_molecule_nM': 1.66053906717}, {'N': 120442815.2, 'one_molecule': 8.30269533587e-10, 'one_molecule_nM': 0.000830269533587}],
    'cmb_gc_content': [{'gc_fraction': 0.571428571429, 'gc_percent': 57.1428571429, 'length': 14, 'G': 4, 'C': 4, 'A': 3, 'T_or_U': 3}, {'gc_fraction': 0.5, 'gc_percent': 50.0, 'length': 14, 'G': 4, 'C': 3, 'A': 2, 'T_or_U': 5}],
    'cmb_tm_wallace': [{'Tm': 48.0, 'AT': 8, 'GC': 8}, {'Tm': 50.0, 'AT': 7, 'GC': 9}],
    'cmb_tm_basic': [{'Tm': 53.9666666667, 'gc_percent': 41.6666666667, 'length': 24}, {'Tm': 47.0529411765, 'gc_percent': 52.9411764706, 'length': 17}],
    'cmb_tm_salt_adjusted': [{'Tm': 51.9862354053, 'gc_percent': 41.6666666667, 'salt_term': -21.597097928}, {'Tm': 51.3117647059, 'gc_percent': 52.9411764706, 'salt_term': -16.6}],
    'cmb_tm_nearest_neighbor': [{'Tm': 62.5816284138, 'dH': -221.7, 'dS': -628.76559587, 'dG37': -26.6883504409}, {'Tm': 39.2579488965, 'dH': -79.4, 'dS': -225.326161828, 'dG37': -9.51509090905}],
    'cmb_oligo_molecular_weight': [{'MW': 3.1069898, 'MW_g_mol': 3106.9898, 'MW_hydroxyl_g_mol': 3027.009898}, {'MW': 3.9083204, 'MW_g_mol': 3908.3204, 'MW_hydroxyl_g_mol': 3748.360596}],
    'cmb_oligo_extinction_coefficient': [{'eps': 9800.0, 'eps_L_mol_cm': 98000.0, 'ug_per_A260': 30.887856102, 'nmol_per_A260': 10.2040816327}, {'eps': 17250.0, 'eps_L_mol_cm': 172500.0, 'ug_per_A260': 30.3096666551, 'nmol_per_A260': 5.79710144928}],
    'cmb_nucleic_acid_a260': [{'c': 0.25, 'c_ug_mL': 250.0}, {'c': 0.5, 'c_ug_mL': 500.0, 'mass_ug': 25.0}],
    'cmb_absorbance_ratios': [{'ratio_260_280': 1.8, 'ratio_260_230': 2.14285714286}, {'ratio_260_280': 1.93548387097}],
    'cmb_warburg_christian': [{'protein': 0.86, 'protein_mg_mL': 0.86, 'ratio_280_260': 1.6}, {'protein': 1.176, 'protein_mg_mL': 1.176, 'ratio_280_260': 1.33333333333}],
    'cmb_protein_concentration_a280': [{'c': 0.017213679137, 'c_uM': 17.213679137, 'c_mg_mL': 1.14350470507}, {'c': 0.00791556728232, 'c_uM': 7.91556728232}],
    'cmb_dna_copy_number': [{'MW': 1853.91604, 'MW_g_mol': 1853916.04, 'pmol': 0.000539398752923, 'copies': 324833521.587}, {'MW': 480.909, 'MW_g_mol': 480909.0, 'pmol': 0.103969773907, 'copies': 62612061325.5}],
    'cmb_ligation_insert_mass': [{'m_insert': 5e-11, 'm_insert_ng': 50.0, 'vector_fmol': 25.2525252525, 'insert_fmol': 75.7575757576}, {'m_insert': 1.38888888889e-11, 'm_insert_ng': 13.8888888889, 'vector_fmol': 28.0583613917, 'insert_fmol': 28.0583613917}],
    'cmb_reverse_complement': [{'reverse_complement': 'TCAGGTACGCAT', 'complement': 'TACGCATGGACT', 'reverse': 'AGTCCATGCGTA'}, {'reverse_complement': 'UAAGGAUCC', 'complement': 'CCUAGGAAU', 'reverse': 'AUUCCUAGG'}],
    'cmb_transcribe': [{'mRNA': 'AUGGCCUUUAAG', 'length': 12}, {'mRNA': 'GAAUUUCCGGUA', 'length': 12}],
    'cmb_translate': [{'protein': 'MAIVMGR', 'full_translation': 'MAIVMGR*KGAR*', 'length_aa': 7, 'stops': 2}, {'protein': 'MKRISTTITTTITITTGNGAG', 'full_translation': 'MKRISTTITTTITITTGNGAG*', 'length_aa': 21, 'stops': 1}],
    'cmb_orf_finder': [{'strand': '+', 'start': 3, 'end': 68, 'length_nt': 66, 'length_aa': 21, 'protein': 'MKRISTTITTTITITTGNGAG', 'n_orfs': 1}, {'strand': '-', 'start': 14, 'end': 31, 'length_nt': 18, 'length_aa': 5, 'protein': 'MAGWK', 'n_orfs': 1}],
    'cmb_genome_coverage': [{'coverage': 60.0, 'fraction_uncovered': 8.7565107627e-27, 'bases_uncovered': 4.37825538135e-20, 'contigs': 1.0}, {'coverage': 3.0, 'fraction_uncovered': 0.0497870683679, 'bases_uncovered': 49787.0683679, 'contigs': 1493.61205104}],
    'cmb_library_size': [{'clones_exact': 345385.46136, 'clones': 345386}, {'clones_exact': 2754.57555403, 'clones': 2755}],
    'cmb_mutation_rate_p0': [{'p0': 0.55, 'm': 0.597837000756, 'mutation_rate': 2.98918500378e-09}, {'p0': 0.0625, 'm': 2.77258872224, 'mutation_rate': 4.95105128971e-09}],
    'cmb_pcr_amplification': [{'N': 107374182400.0, 'fold': 1073741824.0, 'log10_fold': 9.03089986992}, {'N': 9307649568.83, 'fold': 9307649.56883, 'log10_fold': 6.96884002382}],
    'cmb_pcr_cycles_needed': [{'cycles_exact': 27.5829148887, 'cycles': 28}, {'cycles_exact': 29.9432879662, 'cycles': 30}],
    'cmb_qpcr_delta_delta_ct': [{'dCt_treated': 6.5, 'dCt_control': 8.8, 'ddCt': -2.3, 'fold_change': 4.92457765338, 'log2_fold_change': 2.3}, {'dCt_treated': 10.1, 'dCt_control': 7.7, 'ddCt': 2.4, 'fold_change': 0.189464570814, 'log2_fold_change': -2.4}],
    'cmb_qpcr_pfaffl_ratio': [{'ratio': 6.43743132062, 'log2_ratio': 2.6864851355, 'dCt_target': 2.5, 'dCt_ref': -0.3}, {'ratio': 0.25, 'log2_ratio': -2.0, 'dCt_target': -2.0, 'dCt_ref': 0.0}],
    'cmb_qpcr_standard_curve': [{'slope': -3.35, 'intercept': 35.22, 'r2': 0.999973268703, 'efficiency': 0.988416978008, 'amplification_factor': 1.98841697801}, {'slope': -3.37639667705, 'intercept': 26.7627933541, 'r2': 0.999928186715, 'efficiency': 0.977760644702, 'amplification_factor': 1.9777606447}],
    'cmb_qpcr_quantify': [{'quantity': 10139.6764313, 'log10_quantity': 4.00602409639}, {'quantity': 62.6760256005, 'log10_quantity': 1.79710144928}],
    'cmb_transcription_time': [{'t': 500.0, 't_min': 8.33333333333}, {'t': 120.0, 't_min': 2.0, 'polymerases': 12.0, 'spacing_nt': 500.0}],
    'cmb_translation_time': [{'t': 50.0, 't_min': 0.833333333333, 'coding_nt': 903}, {'t': 30.0, 't_min': 0.5, 'coding_nt': 1353, 'ribosomes': 6.0, 'spacing_codons': 75.0}],
    'cmb_expression_steady_state': [{'m_ss': 5.0, 'p_ss': 2500.0, 'mRNA_half_life': 346.57359028, 'protein_half_life': 6931.4718056, 'mRNA_half_life_min': 5.77622650467, 'protein_half_life_h': 1.92540883489}, {'m_ss': 5.0, 'p_ss': 5000.0, 'mRNA_half_life': 207.944154168, 'protein_half_life': 3465.7359028, 'mRNA_half_life_min': 3.4657359028, 'protein_half_life_h': 0.962704417444}],
    'cmb_induction_dynamics': [{'m': 4.99999721305, 'p': 1219.07308397, 'fraction_of_steady_state': 0.487629233589}, {'m': 16.6940222356, 'p': 502.873305426, 'fraction_of_steady_state': 0.754309958139}],
    'cmb_mrna_half_life_fit': [{'k_per_min': 0.0115870074169, 'half_life_min': 59.8210698948, 'initial_fit': 99.9556057618, 'r2': 0.999799332174}, {'k_per_min': 0.0473036091229, 'half_life_min': 14.6531563534, 'initial_fit': 1.00496380931, 'r2': 0.999571864107}],
    'cmb_michaelis_menten_rate': [{'v': 0.00111111111111, 'v_uM_min': 66.6666666667, 'fraction': 0.666666666667}, {'v': 4e-07, 'v_uM_min': 0.024, 'fraction': 0.2}],
    'cmb_substrate_for_fraction_of_vmax': [{'S': 0.45, 'S_uM': 450.0, 'S_over_Km': 9.0}, {'S': 0.666666666667, 'S_uM': 666.666666667, 'S_over_Km': 0.333333333333}],
    'cmb_kcat_catalytic_efficiency': [{'kcat': 100.0, 'kcat_per_s': 100.0, 'efficiency': 4000.0, 'efficiency_per_M_s': 4000000.0, 'turnover_time': 0.01}, {'kcat': 2.5, 'kcat_per_s': 2.5, 'efficiency': 1.66666666667, 'efficiency_per_M_s': 1666.66666667, 'turnover_time': 0.4}],
    'cmb_lineweaver_burk_fit': [{'Km': 4.9822689045, 'Vmax': 99.8888410873, 'slope': 0.0498781330354, 'intercept': 0.0100111282613, 'r2': 0.999999664252}, {'Km': 1.77206114918, 'Vmax': 9.52250922509, 'slope': 0.186091828036, 'intercept': 0.105014337751, 'r2': 0.999774868381}],
    'cmb_michaelis_menten_nonlinear_fit': [{'Km': 5.07927661205, 'Vmax': 100.713684406, 'rss': 5.86752641346, 'r2': 0.998641527345}, {'Km': 1.85740381716, 'Vmax': 9.75371065808, 'rss': 0.0172641320742, 'r2': 0.999234338652}],
    'cmb_competitive_inhibition': [{'v': 4e-05, 'Km_app': 0.15, 'Vmax_app': 0.0001, 'percent_inhibition': 40.0}, {'v': 2.85714285714e-07, 'Km_app': 3.0, 'Vmax_app': 2e-06, 'percent_inhibition': 57.1428571429}],
    'cmb_uncompetitive_inhibition': [{'v': 2.85714285714e-05, 'Km_app': 0.0166666666667, 'Vmax_app': 3.33333333333e-05, 'percent_inhibition': 57.1428571429}, {'v': 4e-07, 'Km_app': 0.333333333333, 'Vmax_app': 6.66666666667e-07, 'percent_inhibition': 40.0}],
    'cmb_noncompetitive_inhibition': [{'v': 2.22222222222e-05, 'Km_app': 0.05, 'Vmax_app': 3.33333333333e-05, 'percent_inhibition': 66.6666666667}, {'v': 2.22222222222e-07, 'Km_app': 1.0, 'Vmax_app': 6.66666666667e-07, 'percent_inhibition': 66.6666666667}],
    'cmb_mixed_inhibition': [{'v': 3.33333333333e-05, 'Km_app': 0.1, 'Vmax_app': 6.66666666667e-05, 'alpha': 4.0}, {'v': 3.07692307692e-07, 'Km_app': 0.428571428571, 'Vmax_app': 5.71428571429e-07, 'alpha': 0.2}],
    'cmb_ic50_to_ki': [{'Ki': 0.00333333333333, 'Ki_uM': 3.33333333333}, {'Ki': 0.125, 'Ki_uM': 125.0}],
    'cmb_hill_equation': [{'theta': 0.5, 'theta_percent': 50.0}, {'theta': 0.2, 'theta_percent': 20.0}],
    'cmb_hill_ligand_for_fraction': [{'L': 0.0569867965402, 'L_uM': 56.9867965402}, {'L': 0.00198425131496, 'L_uM': 1.98425131496}],
    'cmb_hill_coefficient_from_ec': [{'n': 1.0, 'EC50': 0.009, 'ratio': 81.0}, {'n': 2.0, 'EC50': 0.03, 'ratio': 9.0}],
    'cmb_fraction_bound': [{'theta': 0.666666666667, 'theta_percent': 66.6666666667}, {'theta': 0.166666666667, 'theta_percent': 16.6666666667, 'bound': 8.33333333333e-06}],
    'cmb_ligand_depletion_binding': [{'RL': 5.85786437627e-06, 'fraction_receptor_bound': 0.585786437627, 'free_ligand': 1.41421356237e-05, 'fraction_ligand_bound': 0.292893218813}, {'RL': 4.81432234456e-05, 'fraction_receptor_bound': 0.481432234456, 'free_ligand': 1.85677655437e-06, 'fraction_ligand_bound': 0.962864468913}],
    'cmb_kd_from_rates': [{'Kd': 1e-06, 'Kd_nM': 1.0, 'Ka_per_M': 1000000000.0, 'half_life': 693.14718056, 'residence_time': 1000.0}, {'Kd': 0.0004, 'Kd_nM': 400.0, 'Ka_per_M': 2500000.0, 'half_life': 3.4657359028, 'residence_time': 5.0}],
    'cmb_scatchard_fit': [{'Kd': 4.98648892749, 'Bmax': 99.9303765767, 'slope': -0.20054190725, 'intercept': 20.0402283109, 'r2': 0.99999711189}, {'Kd': 4.14004859731, 'Bmax': 84.4319637729, 'slope': -0.241543058372, 'intercept': 20.393954754, 'r2': 0.992705787788}],
    'cmb_binding_free_energy': [{'dG': -45664.0760203, 'dG_kJ_mol': -45.6640760203, 'dG_kcal_mol': -10.9139761043}, {'dG': -33263.6126368, 'dG_kJ_mol': -33.2636126368, 'dG_kcal_mol': -7.95019422487}],
    'cmb_competitive_occupancy': [{'occupancy_L': 0.4, 'occupancy_I': 0.4, 'free_fraction': 0.2, 'Kd_app': 1.5e-05}, {'occupancy_L': 0.0454545454545, 'occupancy_I': 0.909090909091, 'free_fraction': 0.0454545454545, 'Kd_app': 0.0021}],
    'cmb_mwc_saturation': [{'Y': 0.130842539341, 'R_fraction': 0.105920807646}, {'Y': 0.402298850575, 'R_fraction': 0.137931034483}],
    'cmb_q10_temperature': [{'k2': 20.0, 'ratio': 2.0, 'Ea': 52948.8621657, 'Ea_kJ_mol': 52.9488621657}, {'k2': 0.947795187121, 'ratio': 0.210621152693, 'Ea': 69267.4440937, 'Ea_kJ_mol': 69.2674440937}],
    'cmb_enzyme_assay_activity': [{'rate': 0.00032154340836, 'units': 0.0192926045016, 'units_per_mL': 0.96463022508, 'specific_activity': 1.92926045016}, {'rate': 0.00040192926045, 'units': 0.072347266881, 'units_per_mL': 1.44694533762}],
    'cmb_henderson_hasselbalch': [{'pH': 7.38609125906, 'ratio': 1.5, 'fraction_base': 0.6}, {'pH': 3.76, 'ratio': 0.1, 'fraction_base': 0.0909090909091}],
    'cmb_buffer_preparation': [{'C_base': 60.7661064127, 'C_acid': 39.2338935873, 'n_base': 0.0607661064127, 'n_acid': 0.0392338935873, 'ratio': 1.54881661891, 'm_acid_g': 5.41388497611, 'm_base_g': 8.62635646634}, {'C_base': 17.732377478, 'C_acid': 32.267622522, 'n_base': 0.00886618873902, 'n_acid': 0.016133811261, 'ratio': 0.549540873858}],
    'cmb_buffer_capacity': [{'beta': 57.5651427367, 'beta_mM': 57.5651427367, 'beta_buffer_mM': 57.5646273249}, {'beta': 6.00565127571, 'beta_mM': 6.00565127571, 'beta_buffer_mM': 5.9983625809}],
    'cmb_buffer_after_addition': [{'pH': 7.38609125906, 'delta_pH': 0.176091259056, 'n_base': 0.006, 'n_acid': 0.004}, {'pH': 4.61982129683, 'delta_pH': -0.140178703165, 'n_base': 0.0105, 'n_acid': 0.0145}],
    'cmb_c1v1_dilution': [{'V1': 5e-07, 'V1_uL': 500.0, 'V_diluent': 9.5e-06, 'dilution_factor': 20.0}, {'V1': 1.5e-05, 'V1_uL': 15000.0, 'V_diluent': 0.000485, 'dilution_factor': 33.3333333333}],
    'cmb_serial_dilution': [{'C_final': 0.001, 'total_dilution': 10000.0, 'concentrations_mM': [1.0, 0.1, 0.01, 0.001]}, {'C_final': 0.015625, 'total_dilution': 64.0, 'concentrations_mM': [0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625], 'transfer': 5e-08, 'diluent': 5e-08}],
    'cmb_mass_for_molarity': [{'m': 0.004383, 'm_g': 4.383, 'm_mg': 4383.0, 'n': 0.075}, {'m': 0.0033215, 'm_g': 3.3215, 'm_mg': 3321.5, 'n': 5e-05}],
    'cmb_molarity_from_mass': [{'C': 99.9315537303, 'C_mM': 99.9315537303, 'C_M': 0.0999315537303, 'n': 0.0499657768652}, {'C': 0.15053439711, 'C_mM': 0.15053439711, 'C_M': 0.00015053439711, 'n': 1.5053439711e-07}],
    'cmb_percent_wv_to_molarity': [{'C': 154.004106776, 'C_mM': 154.004106776, 'C_M': 0.154004106776, 'mass_concentration': 9.0, 'g_per_L': 9.0}, {'C': 277.531083481, 'C_mM': 277.531083481, 'C_M': 0.277531083481, 'mass_concentration': 50.0, 'g_per_L': 50.0}],
    'cmb_ionic_strength': [{'I': 150.0, 'I_mM': 150.0, 'I_M': 0.15}, {'I': 32.5, 'I_mM': 32.5, 'I_M': 0.0325}],
    'cmb_centrifuge_rcf': [{'RCF': 1006.41956235, 'rpm_value': 3000.0}, {'RCF': 18410.7685272, 'rpm_value': 14000.0}],
    'cmb_centrifuge_rpm': [{'omega': 313.155712067, 'rpm_value': 2990.41677197}, {'omega': 1366.72357605, 'rpm_value': 13051.2487781}],
    'cmb_sedimentation_time': [{'t': 5643.52174066, 't_h': 1.56764492796, 'k_factor': 109.735144957}, {'t': 20051.6173488, 't_h': 5.56989370799, 'k_factor': 24.5075323151}],
}

for _k, _vals in _EXV.items():
    if _k in PROCEDURES:
        for _e, _v in zip(PROCEDURES[_k]["examples"], _vals):
            _e["outputs"].update(_v)


# ================================================================================================ CORE GAPS (2026-11-12)
# Microscopy, water potential, free energy of metabolism, ATP yield of respiration and heat killing of microbes. The
# worked-example values below are hand-worked (exact where the inputs allow) and are written into each example.
# Conventions: Abbe resolution limit d = lambda / (2 NA); water potential psi = -i C R T + psi_p (C in mol/m^3 gives
# Pa); free energies use the biochemical standard state (1 M, pH 7 folded into dG0'; water and H+ omitted) and a
# species with a coefficient is listed that many times; ATP per glucose counts 4 substrate-level ATP (net 2 from
# glycolysis, 2 GTP/ATP from the citric acid cycle), 8 mitochondrial NADH, 2 FADH2 and 2 cytosolic NADH that enter
# through the named shuttle (malate-aspartate: as NADH; glycerol-phosphate: as FADH2); fermentation yields 2 ATP; heat
# killing is first-order (log-linear survivor curves, D-value and z-value in their classical definitions).

_MM = ("mm", 1e3)
_MPA = ("MPa", 1e-6)
_VOLT = ("V", 1.0)


def microscope_resolution(M_obj, M_oc, wavelength, NA):
    _pos("the magnifications, the wavelength and the numerical aperture", M_obj, M_oc, wavelength, NA)
    _need(NA <= 1.6, "the numerical aperture of a light microscope cannot exceed about 1.6 (oil immersion is about 1.4)")
    M = M_obj * M_oc
    d = wavelength / (2.0 * NA)
    return _out(M=M, d=d, d_nm=d * 1e9, d_um=d * 1e6)


def _ref_microscope_resolution(M_obj, M_oc, wavelength, NA):
    import numpy as np
    # Abbe's grating argument: the finest period p still imaged is the one whose first diffraction order (sin = lambda/p)
    # is just collected when the illumination arrives at the edge of an aperture of the same NA (oblique illumination)
    p = _brentq(lambda pp: wavelength / pp - NA - NA, wavelength / 4.0, wavelength * 100.0)
    M = float(np.prod([M_obj, M_oc]))
    return {"M": M, "d": p, "d_nm": p * 1e9, "d_um": p * 1e6}


_add("cmb_microscope_resolution",
     name="Light microscope: total magnification and resolution limit", topic="microscopy / magnification and resolution",
     purpose="The total magnification of a compound light microscope (objective x eyepiece) and its resolution limit d = lambda / (2 NA) (Abbe).",
     inputs=[_numin("M_obj", "magnification of the objective lens (e.g. 40 for 40x)", words=("x",)),
             _numin("M_oc", "magnification of the eyepiece (ocular) lens (e.g. 10 for 10x)", words=("x",)),
             _phys("wavelength", "length", "wavelength of the light", show=_NM),
             _numin("NA", "numerical aperture of the objective")],
     outputs=[_o("M", "number", "total magnification"), _o("d", "length", "resolution limit (smallest resolvable separation)"),
              _o("d_nm", "number", "resolution limit in nanometres"), _o("d_um", "number", "resolution limit in micrometres")],
     primary="d", conditions=["magnifications > 0", "wavelength > 0", "0 < NA <= 1.6"],
     method="M = M_objective x M_eyepiece; d = lambda / (2 NA) (Abbe)",
     latex=r"M = M_{obj}\,M_{oc},\quad d = \frac{\lambda}{2\,\mathrm{NA}}",
     run=microscope_resolution, reference=_ref_microscope_resolution,
     wrong=lambda M_obj, M_oc, wavelength, NA: _out(M=M_obj * M_oc, d=wavelength / NA, d_nm=wavelength / NA * 1e9, d_um=wavelength / NA * 1e6),
     rand=lambda rng: {"M_obj": rng.choice([4.0, 10.0, 20.0, 40.0, 60.0, 100.0]), "M_oc": rng.choice([5.0, 10.0, 15.0, 20.0]),
                       "wavelength": _sig(rng.uniform(4e-7, 7e-7), 3), "NA": _sig(rng.uniform(0.1, 1.4), 3)},
     phrases=("Microscope resolution", "Microscope magnification and resolution", "Resolution limit of a microscope"),
     examples=[_ex({"M_obj": 40.0, "M_oc": 10.0, "wavelength": 5.5e-7, "NA": 0.65},
                   {"M": 400.0, "d": 4.23076923077e-07, "d_nm": 423.076923077, "d_um": 0.423076923077}),
               _ex({"M_obj": 100.0, "M_oc": 10.0, "wavelength": 4.5e-7, "NA": 1.25},
                   {"M": 1000.0, "d": 1.8e-07, "d_nm": 180.0, "d_um": 0.18})],
     level="beginner",
     extra_q=[{"q": "Microscope resolution: M_obj = 40 x, M_oc = 10 x, wavelength = 0.55 um, NA = 0.65", "like": 0},
              {"q": "Microscope resolution: M_obj = 40, M_oc = 10, wavelength = 550 nm, NA = 2", "outcome": "INVALID_INPUT"}],
     weak="Abbe's grating condition re-solved with brentq (the same Abbe limit reached through the diffraction-order argument)")


def specimen_size(image_size, M=None, bar_image=None, bar_label=None):
    _pos("the measured image size", image_size)
    bars = bar_image is not None or bar_label is not None
    _need(not (M is not None and bars), "give either the magnification M or the scale bar (bar_image and bar_label), not both")
    if M is not None:
        _pos("the magnification M", M)
        Meff = M
        size = image_size / M
    else:
        _need(bar_image is not None and bar_label is not None,
              "give the magnification M, or both the scale bar's length on the image (bar_image) and the length it stands for (bar_label)")
        _pos("the scale-bar lengths", bar_image, bar_label)
        Meff = bar_image / bar_label
        size = image_size * bar_label / bar_image
    return _out(size=size, size_um=size * 1e6, M_image=Meff)


def _ref_specimen_size(image_size, M=None, bar_image=None, bar_label=None):
    from fractions import Fraction as Fr
    if M is not None:
        Meff = Fr(M)
        size = Fr(image_size) / Fr(M)
    else:
        Meff = Fr(bar_image) / Fr(bar_label)
        size = Fr(image_size) / Meff
    return {"size": float(size), "size_um": float(size * 10 ** 6), "M_image": float(Meff)}


def _wrong_specimen_size(image_size, M=None, bar_image=None, bar_label=None):
    if M is not None:
        size = image_size * M                            # multiplied instead of divided
        return _out(size=size, size_um=size * 1e6, M_image=M)
    size = image_size * bar_image / bar_label              # scale-bar ratio inverted
    return _out(size=size, size_um=size * 1e6, M_image=bar_image / bar_label)


def _rand_specimen(rng):
    img = _sig(rng.uniform(2e-3, 6e-2), 3)
    if rng.random() < 0.5:
        return {"image_size": img, "M": rng.choice([40.0, 100.0, 400.0, 1000.0, 2500.0, 10000.0])}
    return {"image_size": img, "bar_image": _sig(rng.uniform(5e-3, 4e-2), 3), "bar_label": rng.choice([1e-6, 2e-6, 5e-6, 1e-5, 2e-5, 5e-5, 1e-4])}


_add("cmb_specimen_size",
     name="Actual size of a specimen from a micrograph", topic="microscopy / magnification and resolution",
     purpose="The actual size of an object from its size measured on an image (drawing or micrograph), by the magnification or by the image's scale bar; also the magnification of the image.",
     inputs=[_phys("image_size", "length", "size of the object measured on the image", show=_MM),
             _numin("M", "magnification of the image (give this or the scale bar)", required=False, words=("x",)),
             _phys("bar_image", "length", "length of the scale bar measured on the image (optional, with bar_label)", required=False, show=_MM),
             _phys("bar_label", "length", "length the scale bar stands for (optional, with bar_image)", required=False, show=_UM)],
     outputs=[_o("size", "length", "actual size of the object"), _o("size_um", "number", "actual size in micrometres"),
              _o("M_image", "number", "magnification of the image")],
     primary="size", conditions=["all lengths > 0", "either the magnification or both scale-bar lengths, not both"],
     method="actual size = image size / M; with a scale bar M = bar length on the image / length it represents",
     latex=r"\text{actual size} = \frac{\text{image size}}{M},\quad M = \frac{\text{bar on image}}{\text{bar label}}",
     run=specimen_size, reference=_ref_specimen_size, wrong=_wrong_specimen_size, rand=_rand_specimen,
     phrases=("Specimen size from a micrograph", "Actual size from magnification", "Actual size of a specimen"),
     examples=[_ex({"image_size": 0.012, "M": 400.0}, {"size": 3e-05, "size_um": 30.0, "M_image": 400.0}),
               _ex({"image_size": 0.018, "bar_image": 0.025, "bar_label": 1e-05}, {"size": 7.2e-06, "size_um": 7.2, "M_image": 2500.0})],
     level="beginner",
     extra_q=[{"q": "Actual size from magnification: image_size = 1.2 cm, M = 400 x", "like": 0},
              {"q": "Specimen size from a micrograph: image_size = 18 mm, M = 400, bar_image = 25 mm, bar_label = 10 um", "outcome": "INVALID_INPUT"}],
     weak="the same definition in exact rational arithmetic")


def water_potential(i, C, T, psi_p):
    _pos("the van 't Hoff factor i and T", i, T)
    _nonneg("the solute concentration C", C)
    _fin(psi_p)
    psi_s = -i * C * R_GAS * T
    psi = psi_s + psi_p
    return _out(psi_s=psi_s, psi_s_MPa=psi_s * 1e-6, psi=psi, psi_MPa=psi * 1e-6)


def _ref_water_potential(i, C, T, psi_p):
    sc = _sc()
    c_mol_per_L = C * sc.liter                                      # mol/m^3 -> mol/L
    pi = i * c_mol_per_L * sc.R * T / sc.liter                      # osmotic pressure (van 't Hoff) in Pa
    psi_s = -pi
    return {"psi_s": psi_s, "psi_s_MPa": psi_s / sc.mega, "psi": psi_s + psi_p, "psi_MPa": (psi_s + psi_p) / sc.mega}


_add("cmb_water_potential",
     name="Water potential of a plant cell or solution (solute plus pressure potential)", topic="membranes and diffusion / water potential",
     purpose="The solute potential psi_s = -i C R T and the water potential psi = psi_s + psi_p of a solution or cell (pure water at atmospheric pressure = 0).",
     inputs=[_numin("i", "ionization (van 't Hoff) constant: 1 for sucrose, about 2 for NaCl"),
             _phys("C", "molar_concentration", "molar concentration of the solute"),
             _phys("T", "temperature", "absolute temperature (degC is converted)", show=_KELVIN),
             _phys("psi_p", "pressure", "pressure potential (turgor; 0 for an open solution)", show=_MPA)],
     outputs=[_o("psi_s", "pressure", "solute (osmotic) potential"), _o("psi_s_MPa", "number", "solute potential in MPa"),
              _o("psi", "pressure", "water potential"), _o("psi_MPa", "number", "water potential in MPa")],
     primary="psi", conditions=["i > 0, T > 0", "C >= 0", "ideal dilute solution"],
     method="psi_s = -i C R T; psi = psi_s + psi_p", latex=r"\Psi_s = -iCRT,\quad \Psi = \Psi_s + \Psi_p",
     run=water_potential, reference=_ref_water_potential,
     wrong=lambda i, C, T, psi_p: (lambda s: _out(psi_s=s, psi_s_MPa=s * 1e-6, psi=s + psi_p, psi_MPa=(s + psi_p) * 1e-6))(i * C * R_GAS * T),
     rand=lambda rng: {"i": rng.choice([1.0, 1.0, 1.8, 2.0, 3.0]), "C": _sig(_lu(rng, 1.0, 1000.0), 3),
                       "T": _sig(rng.uniform(278.0, 313.0), 5), "psi_p": _sig(rng.uniform(0.0, 1e6), 3)},
     phrases=("Water potential", "Solute potential and water potential"),
     examples=[_ex({"i": 1.0, "C": 150.0, "T": 293.15, "psi_p": 0.0},
                   {"psi_s": -365607.707477, "psi_s_MPa": -0.365607707477, "psi": -365607.707477, "psi_MPa": -0.365607707477}),
               _ex({"i": 2.0, "C": 100.0, "T": 298.15, "psi_p": 300000.0},
                   {"psi_s": -495791.40592, "psi_s_MPa": -0.49579140592, "psi": -195791.40592, "psi_MPa": -0.19579140592})],
     level="beginner",
     extra_q=[{"q": "Water potential: i = 1, C = 0.15 M, T = 20 degC, psi_p = 0 MPa", "like": 0},
              {"q": "Water potential: i = 0, C = 0.15 M, T = 293.15 K, psi_p = 0 MPa", "outcome": "INVALID_INPUT"}],
     weak="the same relation evaluated with scipy.constants through litres (the osmotic pressure route)")


def water_movement(i_cell, C_cell, P_cell, i_out, C_out, P_out, T):
    _pos("the van 't Hoff factors and T", i_cell, i_out, T)
    _nonneg("the concentrations", C_cell, C_out)
    _fin(P_cell, P_out)
    psi_cell = -i_cell * C_cell * R_GAS * T + P_cell
    psi_out = -i_out * C_out * R_GAS * T + P_out
    d = psi_out - psi_cell
    if abs(d) <= 1e-9 * (abs(psi_cell) + abs(psi_out)):
        d = 0.0
    if d > 0:
        word = "into the cell"
    elif d < 0:
        word = "out of the cell"
    else:
        word = "no net movement"
    return _out(psi_cell=psi_cell, psi_out=psi_out, dpsi=d, dpsi_MPa=d * 1e-6, direction=word)


def _ref_water_movement(i_cell, C_cell, P_cell, i_out, C_out, P_out, T):
    sc = _sc()
    pc = -i_cell * C_cell * sc.R * T + P_cell
    po = -i_out * C_out * sc.R * T + P_out
    d = po - pc
    if abs(d) <= 1e-9 * (abs(pc) + abs(po)):
        d = 0.0
    word = "no net movement" if d == 0 else ("into the cell" if pc < po else "out of the cell")
    return {"psi_cell": pc, "psi_out": po, "dpsi": d, "dpsi_MPa": d / sc.mega, "direction": word}


def _wrong_water_movement(i_cell, C_cell, P_cell, i_out, C_out, P_out, T):
    r = water_movement(i_cell, C_cell, P_cell, i_out, C_out, P_out, T)
    flip = {"into the cell": "out of the cell", "out of the cell": "into the cell", "no net movement": "no net movement"}
    return _out(psi_cell=r["psi_cell"], psi_out=r["psi_out"], dpsi=-r["dpsi"], dpsi_MPa=-r["dpsi_MPa"], direction=flip[r["direction"]])


_add("cmb_water_movement",
     name="Direction of water movement between a cell and its surroundings", topic="membranes and diffusion / water potential",
     purpose="The water potentials of a cell and of the surrounding solution and the direction of net water movement (from higher to lower water potential).",
     inputs=[_numin("i_cell", "ionization constant of the cell's solute"),
             _phys("C_cell", "molar_concentration", "solute concentration in the cell"),
             _phys("P_cell", "pressure", "pressure potential of the cell (turgor)", show=_MPA),
             _numin("i_out", "ionization constant of the outside solute"),
             _phys("C_out", "molar_concentration", "solute concentration outside"),
             _phys("P_out", "pressure", "pressure potential outside (0 for an open solution)", show=_MPA),
             _phys("T", "temperature", "absolute temperature (degC is converted)", show=_KELVIN)],
     outputs=[_o("psi_cell", "pressure", "water potential of the cell"), _o("psi_out", "pressure", "water potential outside"),
              _o("dpsi", "pressure", "outside minus cell water potential"), _o("dpsi_MPa", "number", "outside minus cell water potential in MPa"),
              _o("direction", "number", "net water movement: into the cell, out of the cell or no net movement")],
     primary="direction", conditions=["i > 0, T > 0", "concentrations >= 0", "equal within 1e-9 = no net movement"],
     method="psi = -i C R T + psi_p for each side; water moves from the higher to the lower water potential",
     latex=r"\Psi = -iCRT + \Psi_p,\quad \text{water moves toward lower } \Psi",
     run=water_movement, reference=_ref_water_movement, wrong=_wrong_water_movement,
     rand=lambda rng: {"i_cell": rng.choice([1.0, 2.0]), "C_cell": _sig(rng.uniform(50.0, 800.0), 3), "P_cell": _sig(rng.uniform(0.0, 8e5), 3),
                       "i_out": rng.choice([1.0, 2.0]), "C_out": _sig(rng.uniform(0.0, 800.0), 3), "P_out": 0.0,
                       "T": _sig(rng.uniform(278.0, 313.0), 5)},
     phrases=("Water movement between a cell and its surroundings", "Direction of water movement", "Osmotic water movement"),
     examples=[_ex({"i_cell": 1.0, "C_cell": 200.0, "P_cell": 200000.0, "i_out": 1.0, "C_out": 100.0, "P_out": 0.0, "T": 298.15},
                   {"psi_cell": -295791.40592, "psi_out": -247895.70296, "dpsi": 47895.70296, "dpsi_MPa": 0.04789570296,
                    "direction": "into the cell"}),
               _ex({"i_cell": 1.0, "C_cell": 300.0, "P_cell": 0.0, "i_out": 2.0, "C_out": 200.0, "P_out": 0.0, "T": 310.15},
                   {"psi_cell": -773619.174306, "psi_out": -1031492.23241, "dpsi": -257873.058102, "dpsi_MPa": -0.257873058102,
                    "direction": "out of the cell"})],
     level="intermediate",
     extra_q=[{"q": "Direction of water movement: i_cell = 1, C_cell = 0.2 M, P_cell = 0.2 MPa, i_out = 1, C_out = 0.1 M, P_out = 0 MPa, T = 25 degC", "like": 0},
              {"q": "Direction of water movement: i_cell = 1, C_cell = -200 mM, P_cell = 0.2 MPa, i_out = 1, C_out = 100 mM, P_out = 0 MPa, T = 298.15 K",
               "outcome": "INVALID_INPUT"}],
     weak="the same relation evaluated with scipy.constants")


def _ln_q(products, reactants):
    _need(len(products) >= 1 and len(reactants) >= 1, "give at least one product and one reactant concentration")
    s = 0.0
    for c in products:
        _need(_isnum(c) and c > 0 and c <= 10000.0, "concentrations must be above 0 and at most 10 M (10000 mM)")
        s += math.log(c * 1e-3)
    for c in reactants:
        _need(_isnum(c) and c > 0 and c <= 10000.0, "concentrations must be above 0 and at most 10 M (10000 mM)")
        s -= math.log(c * 1e-3)
    return s


def reaction_free_energy(dG0, T, products, reactants):
    _fin(dG0)
    _pos("T", T)
    _need(abs(dG0) <= 500000.0, "the standard free-energy change must be within 500 kJ/mol")
    lnQ = _ln_q(products, reactants)
    RT = R_GAS * T
    dG = dG0 + RT * lnQ
    if abs(dG) <= 1e-9 * (abs(dG0) + RT):
        dG = 0.0
    if dG < 0:
        word = "forward"
    elif dG > 0:
        word = "reverse"
    else:
        word = "at equilibrium"
    return _out(Q=math.exp(lnQ), dG=dG, dG_kJ_mol=dG / 1000.0, K_eq=math.exp(-dG0 / RT), direction=word)


def _ref_reaction_free_energy(dG0, T, products, reactants):
    import numpy as np
    sc = _sc()
    Q = float(np.prod(np.array(products, dtype=float) / 1000.0) / np.prod(np.array(reactants, dtype=float) / 1000.0))
    RT = sc.R * T
    K = math.exp(-dG0 / RT)
    dG = RT * math.log(Q / K)
    if abs(dG) <= 1e-9 * (abs(dG0) + RT):
        dG = 0.0
    word = "at equilibrium" if dG == 0 else ("forward" if Q < K else "reverse")
    return {"Q": Q, "dG": dG, "dG_kJ_mol": dG / 1000.0, "K_eq": K, "direction": word}


def _wrong_reaction_free_energy(dG0, T, products, reactants):
    lnQ = _ln_q(products, reactants)
    RT = R_GAS * T
    dG = dG0 + RT * lnQ / math.log(10.0)                 # log10 Q used in place of ln Q
    word = "forward" if dG < 0 else ("reverse" if dG > 0 else "at equilibrium")
    return _out(Q=math.exp(lnQ), dG=dG, dG_kJ_mol=dG / 1000.0, K_eq=math.exp(-dG0 / RT), direction=word)


def _rand_rxn_dg(rng):
    return {"dG0": _sig(rng.uniform(-60000.0, 40000.0), 3), "T": _sig(rng.uniform(288.0, 315.0), 5),
            "products": [_sig(_lu(rng, 0.001, 20.0), 3) for _ in range(rng.randint(1, 3))],
            "reactants": [_sig(_lu(rng, 0.001, 20.0), 3) for _ in range(rng.randint(1, 3))]}


_add("cmb_reaction_free_energy",
     name="Free-energy change of a reaction at cellular concentrations", topic="metabolism / free energy",
     purpose="The actual free-energy change dG = dG0' + RT ln Q of a biochemical reaction at the concentrations in the cell (standard state 1 M), its equilibrium constant and the direction it proceeds.",
     inputs=[_phys("dG0", "molar_energy", "standard free-energy change dG0' (biochemical standard state, pH 7)"),
             _phys("T", "temperature", "absolute temperature (degC is converted)", show=_KELVIN),
             _vecin("products", "product concentrations in mM (list a species twice for a coefficient of 2; omit water and H+)", 1, 10),
             _vecin("reactants", "reactant concentrations in mM (list a species twice for a coefficient of 2; omit water and H+)", 1, 10)],
     outputs=[_o("Q", "number", "reaction quotient (concentrations relative to 1 M)"), _o("dG", "molar_energy", "actual free-energy change"),
              _o("dG_kJ_mol", "number", "actual free-energy change in kJ/mol"), _o("K_eq", "number", "equilibrium constant exp(-dG0'/RT)"),
              _o("direction", "number", "forward (dG < 0), reverse (dG > 0) or at equilibrium")],
     primary="dG", conditions=["T > 0", "|dG0'| <= 500 kJ/mol", "0 < each concentration <= 10 M", "ideal (activities = concentrations / 1 M)"],
     method="Q = prod(products / 1 M) / prod(reactants / 1 M); dG = dG0' + R T ln Q; K = exp(-dG0'/RT)",
     latex=r"\Delta G = \Delta G^{\circ\prime} + RT\ln Q,\quad Q = \frac{\prod [P]}{\prod [R]}",
     run=reaction_free_energy, reference=_ref_reaction_free_energy, wrong=_wrong_reaction_free_energy, rand=_rand_rxn_dg,
     phrases=("Free energy at cellular concentrations", "Actual free-energy change of a reaction", "Reaction free energy"),
     examples=[_ex({"dG0": -30500.0, "T": 310.15, "products": [0.25, 1.65], "reactants": [2.25]},
                   {"Q": 0.000183333333333, "dG": -52687.9254459, "dG_kJ_mol": -52.6879254459, "direction": "forward"}),
               _ex({"dG0": 5000.0, "T": 298.15, "products": [1.0], "reactants": [1000.0]},
                   {"Q": 0.001, "dG": -12124.0285076, "dG_kJ_mol": -12.1240285076, "direction": "forward"})],
     level="intermediate",
     extra_q=[{"q": "Reaction free energy: dG0 = -30.5 kJ/mol, T = 37 degC, products = [0.25, 1.65], reactants = [2.25]", "like": 0},
              {"q": "Reaction free energy: dG0 = -30.5 kJ/mol, T = 310.15 K, products = [0, 1.65], reactants = [2.25]", "outcome": "INVALID_INPUT"}],
     weak="numpy products of the concentrations and dG = RT ln(Q/K) with scipy.constants (the same relation in its Q/K form)")


def redox_free_energy(E_acceptor, E_donor, n):
    _fin(E_acceptor, E_donor)
    _need(isinstance(n, int) and not isinstance(n, bool) and n >= 1, "the number of electrons n must be a whole number of at least 1")
    _need(abs(E_acceptor) <= 3.0 and abs(E_donor) <= 3.0, "reduction potentials must be within 3 V")
    dE = E_acceptor - E_donor
    dG = -n * FARADAY * dE
    if dE > 0:
        word = "favourable as written"
    elif dE < 0:
        word = "unfavourable as written"
    else:
        word = "at equilibrium"
    return _out(dE=dE, dE_mV=dE * 1e3, dG=dG, dG_kJ_mol=dG / 1000.0, direction=word)


def _ref_redox_free_energy(E_acceptor, E_donor, n):
    sc = _sc()
    F = sc.physical_constants["Faraday constant"][0]
    dE = _brentq(lambda x: x - (E_acceptor - E_donor), -10.0, 10.0) if E_acceptor != E_donor else 0.0
    T = 298.15
    lnK = n * F * dE / (sc.R * T)                       # through the equilibrium constant: dG0' = -RT ln K
    dG = -sc.R * T * lnK
    word = "at equilibrium" if dE == 0 else ("favourable as written" if dG < 0 else "unfavourable as written")
    return {"dE": dE, "dE_mV": dE * 1e3, "dG": dG, "dG_kJ_mol": dG / 1000.0, "direction": word}


_add("cmb_redox_free_energy",
     name="Free-energy change of a redox reaction from reduction potentials", topic="metabolism / free energy",
     purpose="The standard free-energy change dG0' = -n F dE0' of electron transfer from a donor couple to an acceptor couple, from their standard reduction potentials.",
     inputs=[_phys("E_acceptor", "voltage", "standard reduction potential of the electron acceptor couple (E0')", show=_VOLT),
             _phys("E_donor", "voltage", "standard reduction potential of the electron donor couple (E0')", show=_VOLT),
             _intin("n", "number of electrons transferred", lo=1, hi=12)],
     outputs=[_o("dE", "voltage", "potential difference E0'(acceptor) - E0'(donor)"), _o("dE_mV", "number", "potential difference in mV"),
              _o("dG", "molar_energy", "standard free-energy change"), _o("dG_kJ_mol", "number", "standard free-energy change in kJ/mol"),
              _o("direction", "number", "favourable as written (dG < 0), unfavourable or at equilibrium")],
     primary="dG", conditions=["n a whole number >= 1", "|E0'| <= 3 V", "standard (pH 7) conditions"],
     method="dE0' = E0'(acceptor) - E0'(donor); dG0' = -n F dE0'",
     latex=r"\Delta G^{\circ\prime} = -nF\,\Delta E^{\circ\prime},\quad \Delta E^{\circ\prime} = E^{\circ\prime}_{acceptor} - E^{\circ\prime}_{donor}",
     run=redox_free_energy, reference=_ref_redox_free_energy,
     wrong=lambda E_acceptor, E_donor, n: (lambda dE: _out(dE=dE, dE_mV=dE * 1e3, dG=-n * FARADAY * dE, dG_kJ_mol=-n * FARADAY * dE / 1000.0,
                                                           direction="favourable as written" if dE > 0 else ("unfavourable as written" if dE < 0 else "at equilibrium")))(E_donor - E_acceptor),
     rand=lambda rng: {"E_acceptor": _sig(rng.uniform(-0.45, 0.85), 3), "E_donor": _sig(rng.uniform(-0.45, 0.85), 3), "n": rng.choice([1, 2, 2, 4])},
     phrases=("Redox free energy", "Free energy from reduction potentials", "Free energy of electron transfer"),
     examples=[_ex({"E_acceptor": 0.816, "E_donor": -0.32, "n": 2},
                   {"dE": 1.136, "dE_mV": 1136.0, "dG": -219214.674584, "dG_kJ_mol": -219.214674584, "direction": "favourable as written"}),
               _ex({"E_acceptor": 0.045, "E_donor": -0.219, "n": 2},
                   {"dE": 0.264, "dE_mV": 264.0, "dG": -50944.2553611, "dG_kJ_mol": -50.9442553611, "direction": "favourable as written"})],
     level="intermediate",
     extra_q=[{"q": "Redox free energy: E_acceptor = 816 mV, E_donor = -320 mV, n = 2", "like": 0},
              {"q": "Redox free energy: E_acceptor = 0.816 V, E_donor = -0.32 V, n = 0", "outcome": "INVALID_INPUT"}],
     weak="dG0' = -RT ln K with ln K = n F dE0' / RT and scipy.constants (the same relation through the equilibrium constant)")


def atp_yield(P_O_NADH, P_O_FADH2, shuttle, dG_ATP=None, dG_glucose=None):
    _pos("the P/O ratios", P_O_NADH, P_O_FADH2)
    _need(P_O_NADH <= 3.0 and P_O_FADH2 <= 3.0, "P/O ratios must be at most 3")
    _need(shuttle in ("malate-aspartate", "glycerol-phosphate"), "the shuttle must be malate-aspartate or glycerol-phosphate")
    substrate = 4.0                       # net 2 ATP from glycolysis + 2 GTP/ATP from the citric acid cycle
    if shuttle == "malate-aspartate":
        per_cyto = P_O_NADH
    else:
        per_cyto = P_O_FADH2
    oxidative = 8.0 * P_O_NADH + 2.0 * P_O_FADH2 + 2.0 * per_cyto
    total = substrate + oxidative
    ferm = 2.0
    out = {"ATP_aerobic": total, "ATP_fermentation": ferm, "ratio": total / ferm, "ATP_oxidative": oxidative,
           "ATP_substrate_level": substrate, "NADH_total": 10.0, "FADH2_total": 2.0}
    if dG_ATP is not None or dG_glucose is not None:
        _need(dG_ATP is not None and dG_glucose is not None, "give both dG_ATP and dG_glucose for the efficiencies")
        _pos("the energies dG_ATP and dG_glucose (as positive magnitudes)", dG_ATP, dG_glucose)
        eff = total * dG_ATP / dG_glucose
        _need(eff <= 1.0, "the ATP captured cannot hold more energy than the oxidation of glucose releases")
        out["efficiency_aerobic"] = eff
        out["efficiency_fermentation"] = ferm * dG_ATP / dG_glucose
    return _out(**out)


def _ref_atp_yield(P_O_NADH, P_O_FADH2, shuttle, dG_ATP=None, dG_glucose=None):
    # an explicit ledger of the steps per glucose: (step, ATP or GTP, cytosolic NADH, mitochondrial NADH, FADH2)
    ledger = [("hexokinase", -1, 0, 0, 0), ("phosphofructokinase", -1, 0, 0, 0),
              ("glyceraldehyde-3-phosphate dehydrogenase x2", 0, 2, 0, 0), ("phosphoglycerate kinase x2", 2, 0, 0, 0),
              ("pyruvate kinase x2", 2, 0, 0, 0), ("pyruvate dehydrogenase x2", 0, 0, 2, 0),
              ("isocitrate dehydrogenase x2", 0, 0, 2, 0), ("alpha-ketoglutarate dehydrogenase x2", 0, 0, 2, 0),
              ("succinyl-CoA synthetase x2", 2, 0, 0, 0), ("succinate dehydrogenase x2", 0, 0, 0, 2),
              ("malate dehydrogenase x2", 0, 0, 2, 0)]
    atp = sum(s[1] for s in ledger)
    cyto = sum(s[2] for s in ledger)
    mito = sum(s[3] for s in ledger)
    fadh2 = sum(s[4] for s in ledger)
    shuttle_po = P_O_NADH if shuttle == "malate-aspartate" else P_O_FADH2
    ox = mito * P_O_NADH + fadh2 * P_O_FADH2 + cyto * shuttle_po
    glycolysis_net = sum(s[1] for s in ledger[:5])          # what fermentation keeps (NADH re-oxidised to lactate/ethanol)
    total = atp + ox
    out = {"ATP_aerobic": float(total), "ATP_fermentation": float(glycolysis_net), "ratio": total / glycolysis_net,
           "ATP_oxidative": float(ox), "ATP_substrate_level": float(atp), "NADH_total": float(cyto + mito), "FADH2_total": float(fadh2)}
    if dG_ATP is not None:
        out["efficiency_aerobic"] = total * dG_ATP / dG_glucose
        out["efficiency_fermentation"] = glycolysis_net * dG_ATP / dG_glucose
    return out


def _wrong_atp_yield(P_O_NADH, P_O_FADH2, shuttle, dG_ATP=None, dG_glucose=None):
    r = dict(atp_yield(P_O_NADH, P_O_FADH2, shuttle))     # gross glycolysis: the 2 ATP invested are not subtracted
    r["ATP_aerobic"] += 2.0
    r["ATP_fermentation"] += 2.0
    r["ATP_substrate_level"] += 2.0
    r["ratio"] = r["ATP_aerobic"] / r["ATP_fermentation"]
    if dG_ATP is not None and dG_glucose is not None:
        r["efficiency_aerobic"] = r["ATP_aerobic"] * dG_ATP / dG_glucose
        r["efficiency_fermentation"] = r["ATP_fermentation"] * dG_ATP / dG_glucose
    return _out(**r)


def _rand_atp(rng):
    d = {"P_O_NADH": rng.choice([2.5, 3.0, 2.7, 2.0]), "P_O_FADH2": rng.choice([1.5, 2.0, 1.6]),
         "shuttle": rng.choice(["malate-aspartate", "glycerol-phosphate"])}
    if rng.random() < 0.5:
        d["dG_ATP"] = _sig(rng.uniform(30000.0, 55000.0), 3)
        d["dG_glucose"] = _sig(rng.uniform(2800000.0, 2900000.0), 4)
    return d


_add("cmb_atp_yield",
     name="ATP yield per glucose: aerobic respiration versus fermentation", topic="metabolism / cellular respiration",
     purpose="The ATP made per glucose by aerobic respiration (substrate-level plus oxidative phosphorylation from NADH and FADH2 at stated P/O ratios and the shuttle for cytosolic NADH) and by fermentation, and optionally the energy efficiency of each.",
     inputs=[_numin("P_O_NADH", "ATP made per mitochondrial NADH (P/O ratio, e.g. 2.5 or 3)"),
             _numin("P_O_FADH2", "ATP made per FADH2 (P/O ratio, e.g. 1.5 or 2)"),
             _choice("shuttle", ("malate-aspartate", "glycerol-phosphate"), "shuttle that carries the 2 cytosolic NADH into the mitochondrion"),
             _phys("dG_ATP", "molar_energy", "free energy stored per mol of ATP, a positive magnitude (optional, e.g. 30.5 kJ/mol)", required=False),
             _phys("dG_glucose", "molar_energy", "free energy released by complete oxidation of glucose, a positive magnitude (optional, e.g. 2870 kJ/mol)", required=False)],
     outputs=[_o("ATP_aerobic", "number", "ATP per glucose by aerobic respiration"), _o("ATP_fermentation", "number", "ATP per glucose by fermentation"),
              _o("ratio", "number", "aerobic yield / fermentation yield"), _o("ATP_oxidative", "number", "ATP from oxidative phosphorylation"),
              _o("ATP_substrate_level", "number", "ATP (and GTP) from substrate-level phosphorylation, net"),
              _o("NADH_total", "number", "NADH made per glucose"), _o("FADH2_total", "number", "FADH2 made per glucose"),
              _o("efficiency_aerobic", "number", "fraction of glucose's free energy stored in ATP, aerobic (when energies are given)"),
              _o("efficiency_fermentation", "number", "fraction of glucose's complete-oxidation free energy stored in ATP, fermentation (when energies are given)")],
     primary="ATP_aerobic", conditions=["0 < P/O <= 3", "both energies or neither", "efficiency <= 1"],
     method="ATP = 4 (substrate level, net) + 8 P/O(NADH) + 2 P/O(FADH2) + 2 P/O(shuttle); fermentation = 2; efficiency = ATP dG_ATP / dG_glucose",
     latex=r"N_{ATP} = 4 + 8\,(P/O)_{NADH} + 2\,(P/O)_{FADH_2} + 2\,(P/O)_{shuttle},\quad \eta = \frac{N_{ATP}\,\Delta G_{ATP}}{\Delta G_{glucose}}",
     run=atp_yield, reference=_ref_atp_yield, wrong=_wrong_atp_yield, rand=_rand_atp,
     phrases=("ATP yield per glucose", "ATP per glucose", "ATP yield of respiration"),
     examples=[_ex({"P_O_NADH": 2.5, "P_O_FADH2": 1.5, "shuttle": "malate-aspartate", "dG_ATP": 30500.0, "dG_glucose": 2870000.0},
                   {"ATP_aerobic": 32.0, "ATP_fermentation": 2.0, "ratio": 16.0, "ATP_oxidative": 28.0, "ATP_substrate_level": 4.0,
                    "efficiency_aerobic": 0.34006968641, "efficiency_fermentation": 0.0212543554007}),
               _ex({"P_O_NADH": 2.5, "P_O_FADH2": 1.5, "shuttle": "glycerol-phosphate"},
                   {"ATP_aerobic": 30.0, "ATP_fermentation": 2.0, "ratio": 15.0, "ATP_oxidative": 26.0}),
               _ex({"P_O_NADH": 3.0, "P_O_FADH2": 2.0, "shuttle": "malate-aspartate"},
                   {"ATP_aerobic": 38.0, "ratio": 19.0, "ATP_oxidative": 34.0})],
     level="intermediate",
     extra_q=[{"q": "ATP per glucose: P_O_NADH = 2.5, P_O_FADH2 = 1.5, shuttle = malate aspartate, dG_ATP = 30.5 kJ/mol, dG_glucose = 2870 kJ/mol", "like": 0},
              {"q": "ATP yield per glucose: P_O_NADH = 5, P_O_FADH2 = 1.5, shuttle = malate-aspartate", "outcome": "INVALID_INPUT"}],
     weak="an explicit step-by-step ledger of glycolysis, pyruvate oxidation and the citric acid cycle, summed")


def decimal_reduction_time(N0, N, t):
    _pos("the counts N0 and N and the time t", N0, N, t)
    _need(N < N0, "the surviving count N must be below the initial count N0")
    lr = math.log10(N0 / N)
    D = t / lr
    k = math.log(10.0) / D
    return _out(log_reduction=lr, D=D, D_min=D / MINUTE, k_per_min=k * MINUTE, survival_fraction=N / N0)


def _ref_decimal_reduction(N0, N, t):
    slope, icpt, _r2 = _linregress([0.0, t], [math.log10(N0), math.log10(N)])
    D = -1.0 / slope
    lr = icpt - math.log10(N)
    return {"log_reduction": lr, "D": D, "D_min": D / 60.0, "k_per_min": -slope * math.log(10.0) * 60.0, "survival_fraction": 10.0 ** (-lr)}


def _rand_drt(rng):
    N0 = _sig(_lu(rng, 1e3, 1e9), 3)
    return {"N0": N0, "N": _sig(N0 * 10.0 ** (-rng.uniform(0.5, 8.0)), 3), "t": _sig(_lu(rng, 10.0, 3600.0), 3)}


_add("cmb_decimal_reduction_time",
     name="D-value (decimal reduction time) from survivor counts", topic="microbial control / heat and chemical killing",
     purpose="The number of log10 reductions and the D-value (time for a 90 % kill) of a treatment from the viable counts before and after a stated time, assuming first-order (log-linear) death.",
     inputs=[_numin("N0", "initial viable count (CFU/mL or CFU)"),
             _numin("N", "surviving viable count after the treatment (same units as N0)"),
             _phys("t", "time", "treatment (exposure) time", show=_MINUTES)],
     outputs=[_o("log_reduction", "number", "number of log10 reductions log10(N0/N)"), _o("D", "time", "D-value (decimal reduction time)"),
              _o("D_min", "number", "D-value in minutes"), _o("k_per_min", "number", "first-order death rate constant in 1/min"),
              _o("survival_fraction", "number", "surviving fraction N/N0")],
     primary="D", conditions=["0 < N < N0", "t > 0", "first-order (log-linear) killing"],
     method="log reductions = log10(N0/N); D = t / log10(N0/N); k = ln 10 / D",
     latex=r"D = \frac{t}{\log_{10}(N_0/N)},\quad k = \frac{\ln 10}{D}",
     run=decimal_reduction_time, reference=_ref_decimal_reduction,
     wrong=lambda N0, N, t: (lambda lr: _out(log_reduction=lr, D=t / lr, D_min=t / lr / MINUTE, k_per_min=math.log(10.0) / (t / lr) * MINUTE,
                                             survival_fraction=N / N0))(math.log(N0 / N)),
     rand=_rand_drt,
     phrases=("D-value from survivor counts", "Decimal reduction time", "D-value"),
     examples=[_ex({"N0": 1e6, "N": 100.0, "t": 600.0},
                   {"log_reduction": 4.0, "D": 150.0, "D_min": 2.5, "k_per_min": 0.921034037198, "survival_fraction": 0.0001}),
               _ex({"N0": 5e7, "N": 5e4, "t": 270.0}, {"log_reduction": 3.0, "D": 90.0, "D_min": 1.5, "survival_fraction": 0.001})],
     level="beginner",
     extra_q=[{"q": "Decimal reduction time: N0 = 1e6, N = 100, t = 10 min", "like": 0},
              {"q": "D-value: N0 = 1000, N = 5000, t = 10 min", "outcome": "INVALID_INPUT"}])


def z_value(D1, T1, D2, T2):
    _pos("the D-values and temperatures", D1, D2, T1, T2)
    _need(T1 != T2 and D1 != D2, "the two temperatures and the two D-values must differ")
    _need((T2 - T1) * (D1 - D2) > 0, "the D-value must fall as the temperature rises (z > 0)")
    z = (T2 - T1) / math.log10(D1 / D2)
    return _out(z=z, fold_per_10_degrees=10.0 ** (10.0 / z))


def _ref_z_value(D1, T1, D2, T2):
    slope, _i, _r2 = _linregress([T1, T2], [math.log10(D1), math.log10(D2)])
    z = -1.0 / slope
    return {"z": z, "fold_per_10_degrees": 10.0 ** (-10.0 * slope)}


def _rand_z(rng):
    T1 = _sig(rng.uniform(333.15, 393.15), 5)
    dT = _sig(rng.uniform(3.0, 20.0), 3)
    z = rng.uniform(4.0, 25.0)
    D1 = _sig(_lu(rng, 30.0, 3000.0), 3)
    D2 = _sig(D1 * 10.0 ** (-dT / z), 3)
    if D2 >= D1:
        D2 = _sig(D1 * 0.5, 3)
    return {"D1": D1, "T1": T1, "D2": D2, "T2": _sig(T1 + dT, 5)}


_add("cmb_z_value",
     name="z-value from D-values at two temperatures", topic="microbial control / heat and chemical killing",
     purpose="The z-value (temperature rise that reduces the D-value ten-fold) from the D-values measured at two temperatures, and the fold change of D per 10 degrees.",
     inputs=[_phys("D1", "time", "D-value at the first temperature", show=_MINUTES),
             _phys("T1", "temperature", "first temperature (degC is converted)", show=_KELVIN),
             _phys("D2", "time", "D-value at the second temperature", show=_MINUTES),
             _phys("T2", "temperature", "second temperature (degC is converted)", show=_KELVIN)],
     outputs=[_o("z", "number", "z-value in kelvin (= degrees Celsius of difference)"),
              _o("fold_per_10_degrees", "number", "factor by which D falls for a 10-degree rise")],
     primary="z", conditions=["D-values and temperatures > 0", "T1 != T2", "D falls as T rises (z > 0)", "log-linear thermal death curve"],
     method="z = (T2 - T1) / log10(D1 / D2)", latex=r"z = \frac{T_2 - T_1}{\log_{10}(D_1/D_2)}",
     run=z_value, reference=_ref_z_value,
     wrong=lambda D1, T1, D2, T2: (lambda z: _out(z=z, fold_per_10_degrees=10.0 ** (10.0 / z)))((T2 - T1) / math.log(D1 / D2)),
     rand=_rand_z,
     phrases=("z-value", "z value from two D-values", "Thermal resistance z-value"),
     examples=[_ex({"D1": 600.0, "T1": 383.15, "D2": 60.0, "T2": 393.15}, {"z": 10.0, "fold_per_10_degrees": 10.0}),
               _ex({"D1": 240.0, "T1": 385.15, "D2": 30.0, "T2": 394.15}, {"z": 9.96578428466})],
     level="intermediate",
     extra_q=[{"q": "z-value: D1 = 10 min, T1 = 110 degC, D2 = 1 min, T2 = 120 degC", "like": 0},
              {"q": "z-value: D1 = 1 min, T1 = 110 degC, D2 = 10 min, T2 = 120 degC", "outcome": "INVALID_INPUT"}])


def thermal_death_time(D_ref, T_ref, z, T, log_reductions):
    _pos("D_ref, T_ref, z, T and the number of log reductions", D_ref, T_ref, z, T, log_reductions)
    _need(abs(T - T_ref) / z <= 50.0, "the temperature is too far from the reference temperature for this z-value")
    L = 10.0 ** ((T - T_ref) / z)
    D_T = D_ref / L
    t = log_reductions * D_T
    return _out(D_T=D_T, D_T_min=D_T / MINUTE, t=t, t_min=t / MINUTE, lethal_rate=L)


def _ref_thermal_death_time(D_ref, T_ref, z, T, log_reductions):
    import numpy as np
    logD = float(np.polyval([-1.0 / z, math.log10(D_ref) + T_ref / z], T))     # the thermal death line log10 D(T)
    D_T = 10.0 ** logD
    # time for n log reductions: the integral of 1/D over the hold reaches n
    t = _brentq(lambda tt: _quad(lambda s: 1.0 / D_T, 0.0, tt) - log_reductions, 0.0, 2.0 * log_reductions * D_T + 1e-300)
    return {"D_T": D_T, "D_T_min": D_T / 60.0, "t": t, "t_min": t / 60.0, "lethal_rate": D_ref / D_T}


_add("cmb_thermal_death_time",
     name="D-value at a new temperature and time to a target kill (z-value)", topic="microbial control / heat and chemical killing",
     purpose="The D-value at temperature T from a reference D-value and the z-value, and the holding time needed for a stated number of log10 reductions at T.",
     inputs=[_phys("D_ref", "time", "D-value at the reference temperature", show=_MINUTES),
             _phys("T_ref", "temperature", "reference temperature (degC is converted)", show=_KELVIN),
             _numin("z", "z-value in degrees (a temperature difference: degC = K)", words=("degC", "C", "K")),
             _phys("T", "temperature", "process temperature (degC is converted)", show=_KELVIN),
             _numin("log_reductions", "number of log10 reductions required (e.g. 12)")],
     outputs=[_o("D_T", "time", "D-value at T"), _o("D_T_min", "number", "D-value at T in minutes"),
              _o("t", "time", "holding time for the required log reductions at T"), _o("t_min", "number", "holding time in minutes"),
              _o("lethal_rate", "number", "lethal rate 10^((T - T_ref)/z) relative to the reference temperature")],
     primary="t", conditions=["D_ref, z, log reductions > 0", "|T - T_ref| / z <= 50", "log-linear thermal death"],
     method="D_T = D_ref x 10^((T_ref - T)/z); t = n D_T",
     latex=r"D_T = D_{ref}\,10^{(T_{ref}-T)/z},\quad t = n\,D_T",
     run=thermal_death_time, reference=_ref_thermal_death_time,
     wrong=lambda D_ref, T_ref, z, T, log_reductions: (lambda L: _out(D_T=D_ref * L, D_T_min=D_ref * L / MINUTE, t=log_reductions * D_ref * L,
                                                                         t_min=log_reductions * D_ref * L / MINUTE, lethal_rate=L))(10.0 ** ((T - T_ref) / z)),
     rand=lambda rng: (lambda Tr: {"D_ref": _sig(_lu(rng, 6.0, 600.0), 3), "T_ref": Tr, "z": _sig(rng.uniform(5.0, 15.0), 3),
                                   "T": _sig(Tr + rng.choice([-1.0, 1.0]) * rng.uniform(2.0, 15.0), 5),
                                   "log_reductions": rng.choice([1.0, 3.0, 5.0, 6.0, 12.0])})(rng.choice([394.25, 343.15, 353.15, 373.15])),
     phrases=("Thermal death time", "Time to a target kill", "D-value at a new temperature"),
     examples=[_ex({"D_ref": 12.0, "T_ref": 394.25, "z": 10.0, "T": 384.25, "log_reductions": 12.0},
                   {"D_T": 120.0, "D_T_min": 2.0, "t": 1440.0, "t_min": 24.0, "lethal_rate": 0.1}),
               _ex({"D_ref": 15.0, "T_ref": 394.25, "z": 10.0, "T": 399.25, "log_reductions": 5.0},
                   {"D_T": 4.74341649025, "D_T_min": 0.0790569415042, "t": 23.7170824513, "t_min": 0.395284707521, "lethal_rate": 3.16227766017})],
     level="intermediate",
     extra_q=[{"q": "Thermal death time: D_ref = 0.2 min, T_ref = 394.25 K, z = 10 degC, T = 384.25 K, log_reductions = 12", "like": 0},
              {"q": "Thermal death time: D_ref = 0.2 min, T_ref = 121.1 degC, z = 0 degC, T = 111.1 degC, log_reductions = 12", "outcome": "INVALID_INPUT"}])
