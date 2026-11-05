# cell-molecular-biology-ai999x — offline, verified cell and molecular biology solver with step-by-step answers

[![tests](https://github.com/AI999X/cell-molecular-biology-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/cell-molecular-biology-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/cell-molecular-biology-ai999x)](https://pypi.org/project/cell-molecular-biology-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/cell-molecular-biology-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/cell-molecular-biology-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/cell-molecular-biology-ai999x/)** · **[Guides](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.1.0

Field: **Life Sciences** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

cell-molecular-biology-ai999x is an offline, deterministic Python solver that answers cell and molecular biology questions with verified results from 94 certified capabilities, gives 619 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**cell-molecular-biology-ai999x** is a compact, deterministic cell and molecular biology engine for Python. It reads a question in plain English (or a typed call), picks a certified cell and molecular biology capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** cell-molecular-biology-ai999x comes as the AI999X desktop app and as the command-line program `cell-molecular-biology` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Cell and Molecular Biology desktop app** 1.4.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cell-molecular-biology-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cell-molecular-biology-1.4.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.4.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-1.4.0-linux-x86_64.tar.gz) |
| **Command line** (`cell-molecular-biology`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/cell-molecular-biology-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install cell-molecular-biology-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Cell and Molecular Biology** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Cell and Molecular Biology among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cell-molecular-biology-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-cell-molecular-biology-1.4.0-linux-x86_64.tar.gz
./ai999x-cell-molecular-biology-1.4.0-linux-x86_64/ai999x-cell-molecular-biology
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.4.0/ai999x-cli-1.4.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.4.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.4.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
cell-molecular-biology --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.4.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.4.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.4.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
cell-molecular-biology --version
```

Every command below was run as shown; the text under it is what the program printed.

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ cell-molecular-biology info cmb_nernst_potential
Nernst equilibrium potential of an ion across a cell membrane
  E = (R T / z F) ln(C_out / C_in)
  in   z        charge number of the ion (e.g. 1 for K+, 2 for Ca2+, -1 for Cl-) (integer) []
  in   C_out    outside concentration [mol/m^3]
  in   C_in     inside concentration [mol/m^3]
  in   T        absolute temperature (degC is converted) [K]
  out  E        equilibrium potential (inside minus outside) [V]
  out  E_mV     equilibrium potential in mV []
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ cell-molecular-biology solve cmb_nernst_potential z=1 "C_out=5 (mmol/L)" "C_in=140 (mmol/L)" "T=37 degC"
E = -0.08905869404 V
E_mV = -89.05869404
  by: Nernst equilibrium potential of an ion across a cell membrane  (E = (R T / z F) ln(C_out / C_in))
  C_out = 5 mol/m^3
  C_in = 140 mol/m^3
  T = 310.15 K
  assumes: z a non-zero whole number; concentrations > 0; T > 0
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | cell-molecular-biology batch

`cases.jsonl`:

```json
{"key": "cmb_nernst_potential", "inputs": {"z": "1", "C_out": "5 (mmol/L)", "C_in": "140 (mmol/L)", "T": "37 degC"}}
{"key": "cmb_tm_nearest_neighbor", "inputs": {"seq": "CGTTCCAAAGATGTGGGCATGAGCTTAC", "Na": "50 (mmol/L)", "c1": "250 (nmol/L)", "c2": "250 (nmol/L)"}}
```

```console
$ cell-molecular-biology batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":-0.0890586940367,"unit":"V","solved_for":"E","si":[["C_out",5.0],["C_in",140.0],["T",310.15]],"values":[["E",-0.0890586940367,"V"],["E_mV",-89.0586940367,""]],"target":"E","phrase":"equilibrium potential (inside minus outside)","assumptions":["z a non-zero whole number","concentrations > 0","T > 0"],"name":"Nernst equilibrium potential of an ion across a cell membrane","equation":"E = (R T / z F) ln(C_out / C_in)"}
{"outcome":"SOLVED","message":"","value":62.5816284138,"unit":"","solved_for":"Tm","si":[["Na",50.0],["c1",0.00025],["c2",0.00025]],"values":[["Tm",62.5816284138,""],["dH",-221.7,""],["dS",-628.76559587,""],["dG37",-26.6883504409,""]],"target":"Tm","phrase":"melting temperature in degC","assumptions":["DNA letters, length >= 2","Na+ > 0","c1 > c2/2 (self-complementary: k = c1)"],"name":"Nearest-neighbour melting temperature of a DNA duplex","equation":"sum nearest-neighbour dH and dS with initiation and terminal A/T terms (+ symmetry); dS += 0.368 (N-1) ln[Na+]; Tm = 1000 dH / (dS + R ln k) - 273.15, k = c1 - c2/2"}
```

### In your browser

Open [Cell and Molecular Biology on ai999x.com](https://www.ai999x.com/models/cell-molecular-biology-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `cell_molecular_biology_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install cell-molecular-biology-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Batch culture time under Monod kinetics

The time for a batch culture (constant yield Y, Monod growth) to grow from biomass X0 to X, and the substrate left then.

Command line:

```console
$ cell-molecular-biology solve cmb_monod_batch_time "X0=0.1 g/L" "S0=10 g/L" Y=0.5 "mu_max=0.72 1/h" "Ks=0.2 g/L" "X=4 g/L"
t = 18954.49601 s
t_h = 5.265137782
S = 2.2 kg/m^3
S_g_L = 2.2
  by: Batch culture time under Monod kinetics  (S = S0 - (X - X0)/Y; integrate dX/dt = mu_max S X/(Ks + S): t = [ln(X/X0) + (Ks Y/B)(ln(X/X0) - ln((B - X)/(B - X0)))]/mu_max, B = X0 + Y S0)
  X0 = 0.1 kg/m^3
  S0 = 10 kg/m^3
  Y = 0.5
  mu_max = 0.0002 1/s
  Ks = 0.2 kg/m^3
  X = 4 kg/m^3
  assumes: all inputs > 0; X0 < X < X0 + Y S0; no maintenance or death
```

Python:

```python
import cell_molecular_biology_ai999x as m

a = m.solve("cmb_monod_batch_time", X0="0.1 g/L", S0="10 g/L", Y="0.5", mu_max="0.72 1/h", Ks="0.2 g/L", X="4 g/L")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/cell-molecular-biology-ai999x/#cap-cmb_monod_batch_time) and press **Compute in your browser**.

### Cell concentration from a hemocytometer count

The cell concentration from counts in the large (1 mm x 1 mm x 0.1 mm) squares of a hemocytometer, the dilution factor and, optionally, the total cells in a suspension volume.

Command line:

```console
$ cell-molecular-biology solve cmb_hemocytometer_count "counts=[52, 48, 55, 45]" dilution=2
mean_count = 50
cells_per_mL = 1000000
  by: Cell concentration from a hemocytometer count  (cells/mL = mean count x dilution x 10^4)
  dilution = 2
  assumes: counts >= 0; dilution >= 1; large squares of 0.1 uL
```

Python:

```python
import cell_molecular_biology_ai999x as m

a = m.solve("cmb_hemocytometer_count", counts=[52, 48, 55, 45], dilution="2")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/cell-molecular-biology-ai999x/#cap-cmb_hemocytometer_count) and press **Compute in your browser**.

### Cell-cycle phase durations corrected for the exponential age distribution

The durations of G1, S, G2 and M from phase fractions of an exponentially growing asynchronous population, using the age distribution n(a) = 2 ln2/T 2^(-a/T) (young cells are over-represented).

Command line:

```console
$ cell-molecular-biology solve cmb_cell_cycle_age_corrected "Tc=24 h" "fractions=[0.45, 0.35, 0.15, 0.05]"
G1_h = 8.825562828
S_h = 8.861611432
G2_h = 4.623481871
M_h = 1.689343869
  by: Cell-cycle phase durations corrected for the exponential age distribution  (fraction younger than age a: F(a) = 2(1 - 2^(-a/T)); phase end ages a_k = -T log2(1 - F_k/2) from cumulative fractions)
  Tc = 86400 s
  assumes: Tc > 0; four positive fractions summing to 1; balanced exponential growth
```

Python:

```python
import cell_molecular_biology_ai999x as m

a = m.solve("cmb_cell_cycle_age_corrected", Tc="24 h", fractions=[0.45, 0.35, 0.15, 0.05])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/cell-molecular-biology-ai999x/#cap-cmb_cell_cycle_age_corrected) and press **Compute in your browser**.

### Cell-cycle phase durations from phase fractions

The durations of G1, S, G2 and M from the fraction of an asynchronous population in each phase (simple proportion to the cycle time).

Command line:

```console
$ cell-molecular-biology solve cmb_cell_cycle_phase_durations "Tc=24 h" "fractions=[0.45, 0.35, 0.15, 0.05]"
G1_h = 10.8
S_h = 8.4
G2_h = 3.6
M_h = 1.2
  by: Cell-cycle phase durations from phase fractions  (duration of a phase = fraction x Tc)
  Tc = 86400 s
  assumes: Tc > 0; four positive fractions summing to 1
```

Python:

```python
import cell_molecular_biology_ai999x as m

a = m.solve("cmb_cell_cycle_phase_durations", Tc="24 h", fractions=[0.45, 0.35, 0.15, 0.05])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/cell-molecular-biology-ai999x/#cap-cmb_cell_cycle_phase_durations) and press **Compute in your browser**.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/blob/main/docs/cell_molecular_biology_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/blob/main/docs/cell_molecular_biology_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/blob/main/docs/cell_molecular_biology_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/blob/main/docs/cell_molecular_biology_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/cell-molecular-biology-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified cell and molecular biology capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 619 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `cell-molecular-biology` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** membranes and diffusion, cell growth, DNA and RNA, PCR and gene expression, enzyme kinetics and binding, solution biochemistry.

## Model overview

| Property | Value |
|---|---|
| Package | `cell-molecular-biology-ai999x` 1.1.0 (import `cell_molecular_biology_ai999x`) |
| Type | Deterministic neuro-symbolic cell and molecular biology engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 94 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 619 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `cell-molecular-biology-ai999x "<question>"`; for the native program use `cell-molecular-biology` (above). The outputs above were produced by running this exact code on cell-molecular-biology-ai999x 1.1.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `cell-molecular-biology-ai999x`; the answer is the one it returned, with the steps it took.

**Nernst equilibrium potential of an ion across a cell membrane** · _Membranes and diffusion_

> Nernst potential: z = 1, C_out = 5 mM, C_in = 140 mM, T = 37 degC

→ **E = -0.08905869404 V, E_mV = -89.05869404**

<details><summary>Steps</summary>

- Nernst equilibrium potential of an ion across a cell membrane: z = 1, C_out = 5 (mmol/L), C_in = 140 (mmol/L), T = 37 degC → E = -0.08905869404 V, E_mV = -89.05869404 V

</details>

**Nearest-neighbour melting temperature of a DNA duplex** · _DNA and RNA_

> Nearest-neighbour melting temperature: seq = CGTTCCAAAGATGTGGGCATGAGCTTAC, Na = 50 mM, c1 = 250 nM, c2 = 250 nM

→ **Tm = 62.58162841, dH = -221.7, dS = -628.7655959, dG37 = -26.68835044**

<details><summary>Steps</summary>

- Nearest-neighbour melting temperature of a DNA duplex: seq = CGTTCCAAAGATGTGGGCATGAGCTTAC, Na = 50 (mmol/L), c1 = 250 (nmol/L), c2 = 250 (nmol/L) → Tm = 62.58162841, dH = -221.7, dS = -628.7655959, dG37 = -26.68835044

</details>

**qPCR relative expression by the delta-delta Ct method** · _PCR and gene expression_

> qPCR delta-delta Ct: Ct_target_treated = 24.5, Ct_ref_treated = 18, Ct_target_control = 27, Ct_ref_control = 18.2

→ **dCt_treated = 6.5, dCt_control = 8.8, ddCt = -2.3, fold_change = 4.924577653, log2_fold_change = 2.3**

<details><summary>Steps</summary>

- qPCR relative expression by the delta-delta Ct method: Ct_target_treated = 24.5, Ct_ref_treated = 18, Ct_target_control = 27, Ct_ref_control = 18.2 → dCt_treated = 6.5, dCt_control = 8.8, ddCt = -2.3, fold_change = 4.924577653, log2_fold_change = 2.3

</details>

**Fractional saturation by the Hill equation** · _Enzyme kinetics and binding_

`m.solve("cmb_hill_equation", L=0.026, K=0.026, n=2.8)`

→ **theta = 0.5, theta_percent = 50**

<details><summary>Steps</summary>

- Fractional saturation by the Hill equation: L = 0.026 mol/m^3, K = 0.026 mol/m^3, n = 2.8 → theta = 0.5, theta_percent = 50

</details>

**Recipe for a buffer of given pH and concentration** · _Solution biochemistry_

> Buffer recipe: C = 0.1 M, V = 1 L, pH = 7.4, pKa = 7.21

→ **C_base = 60.76610641 mol/m^3, C_acid = 39.23389359 mol/m^3, n_base = 0.06076610641 mol, n_acid = 0.03923389359 mol, ratio = 1.548816619**

<details><summary>Steps</summary>

- Recipe for a buffer of given pH and concentration: C = 0.1 (mol/L), V = 1 L, pH = 7.4, pKa = 7.21 → C_base = 60.76610641 mol/m^3, C_acid = 39.23389359 mol/m^3, n_base = 0.06076610641 mol, n_acid = 0.03923389359 mol, ratio = 1.548816619 mol/m^3

</details>

**Cell-cycle phase durations corrected for the exponential age distribution** · _Cell growth_

> Age-corrected cell-cycle phase durations: Tc = 24 h, fractions = [0.45, 0.35, 0.15, 0.05]

→ **G1_h = 8.825562828, S_h = 8.861611432, G2_h = 4.623481871, M_h = 1.689343869**

<details><summary>Steps</summary>

- Cell-cycle phase durations corrected for the exponential age distribution: Tc = 24 h, fractions = [0.45, 0.35, 0.15, 0.05] → G1_h = 8.825562828, S_h = 8.861611432, G2_h = 4.623481871, M_h = 1.689343869

</details>

## Figures

Every point was computed by `cell-molecular-biology-ai999x`.

**Fractional saturation by the Hill equation**

![Fractional saturation by the Hill equation — computed by cell-molecular-biology-ai999x](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/resolve/main/figures/cmb-hill-equation-theta-vs-l.svg)

_theta (fractional saturation) against L (free ligand concentration), with K = 0.026 mol/m^3, n = 2.8. 31 points computed by cell-molecular-biology-ai999x; marked: L = 0.026, theta = 0.5._

**Nernst equilibrium potential of an ion across a cell membrane**

![Nernst equilibrium potential of an ion across a cell membrane — computed by cell-molecular-biology-ai999x](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/resolve/main/figures/cmb-nernst-potential-e-vs-c-out.svg)

_E (equilibrium potential (inside minus outside)) against C_out (outside concentration), with C_in = 140 mol/m^3, T = 310.15 K, z = 1. 31 points computed by cell-molecular-biology-ai999x; marked: C_out = 5, E = -0.0890587 V._

**qPCR relative expression by the delta-delta Ct method**

![qPCR relative expression by the delta-delta Ct method — computed by cell-molecular-biology-ai999x](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/resolve/main/figures/cmb-qpcr-delta-delta-ct-fold-change-vs-ct-target-treated.svg)

_fold_change (relative expression 2^-ddCt) against Ct_target_treated (Ct of the target gene in the treated sample), with Ct_ref_control = 18.2, Ct_ref_treated = 18, Ct_target_control = 27. 31 points computed by cell-molecular-biology-ai999x; marked: Ct_target_treated = 24.5, fold_change = 4.92458._

**Doubling time and number of generations from two counts**

![Doubling time and number of generations from two counts — computed by cell-molecular-biology-ai999x](https://huggingface.co/AI999X/cell-molecular-biology-ai999x/resolve/main/figures/cmb-doubling-time-from-counts-td-vs-n0.svg)

_td (doubling time) against N0 (initial cell count (or concentration)), with N = 1.60e5, t = 28800 s. 31 points computed by cell-molecular-biology-ai999x; marked: N0 = 10000, td = 7200 s._

## Capabilities

94 certified procedures. A few of them:

- **Procedures:** A260/A280 and A260/A230 purity ratios; Basic GC melting temperature of an oligonucleotide; Batch culture time under Monod kinetics; Buffer capacity of a weak-acid buffer; Buffer pH after adding strong acid or base; Buffer pH by the Henderson-Hasselbalch equation; Cell concentration from a hemocytometer count; Cell volume after an osmotic change (Boyle-van 't Hoff); Cell-cycle phase durations corrected for the exponential age distribution; Cell-cycle phase durations from phase fractions; Colony-forming units per mL from a plate count; Competitive inhibition rate; Complex concentration with ligand depletion (quadratic binding); Copy number and moles of a DNA or RNA molecule from its mass; Diffusion coefficient from a FRAP half-time (Soumpasis); Diffusion-limited association rate constant (Smoluchowski); Dilution of a stock solution (C1 V1 = C2 V2); Dissociation constant from association and dissociation rates; Donnan equilibrium with an impermeant anion; Doubling time and number of generations from two counts; Enzyme activity and specific activity from a spectrophotometric assay; Equilibration of a solute across a spherical cell's membrane; Exponential growth of a cell population; Extinction coefficient of a DNA oligonucleotide at 260 nm (nearest neighbour) …

`cell_molecular_biology_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified cell and molecular biology capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.
- **Not medical advice.** It computes textbook quantities from the numbers you give it; it does not diagnose, treat or recommend doses, and its results are no substitute for a qualified clinician.

## Limitations

- It answers what its certified capabilities cover (membranes and diffusion, cell growth, DNA and RNA, PCR and gene expression, enzyme kinetics and binding, solution biochemistry); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{cell_molecular_biology_ai999x_1_1_0,
  title  = {cell-molecular-biology-ai999x: an offline, verified cell and molecular biology solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.1.0},
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
