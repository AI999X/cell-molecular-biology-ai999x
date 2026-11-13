# cell-molecular-biology: the Cell and Molecular Biology program, user guide

**cell-molecular-biology-ai999x 1.6.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Cell and Molecular Biology covers membranes and diffusion, osmosis and membrane potentials, cell growth and the cell cycle, DNA and RNA melting temperatures and quantification, sequences, PCR and qPCR, gene expression, enzyme kinetics, inhibition and binding, buffers and dilutions. This guide is about `cell-molecular-biology`, the model as a self-contained program for Windows and Linux: 104 capabilities in 10 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/cell-molecular-biology-ai999x/); the Python package (`pip install cell-molecular-biology-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Cell and Molecular Biology | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cell-molecular-biology-1.5.0-windows-x86_64.zip |
| Desktop app: Cell and Molecular Biology | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cell-molecular-biology-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Cell and Molecular Biology in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-cell-molecular-biology-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-cell-molecular-biology.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cell-molecular-biology-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-cell-molecular-biology-1.5.0-linux-x86_64.tar.gz
./ai999x-cell-molecular-biology-1.5.0-linux-x86_64/ai999x-cell-molecular-biology
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `cell-molecular-biology` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
cell-molecular-biology --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
cell-molecular-biology --version
```

### In your browser

Open https://www.ai999x.com/models/cell-molecular-biology-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install cell-molecular-biology-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ cell-molecular-biology --version
cell-molecular-biology-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ cell-molecular-biology search "nernst equilibrium potential of an ion across a cell membrane"
cmb_nernst_potential                 Nernst equilibrium potential of an ion across a cell membrane  (membranes and diffusion / membrane potential)
cmb_cell_equilibration               Equilibration of a solute across a spherical cell's membrane  (membranes and diffusion / permeability)
cmb_water_potential                  Water potential of a plant cell or solution (solute plus pressure potential)  (membranes and diffusion / water potential)
cmb_channel_current                  Macroscopic current through ion channels  (membranes and diffusion / ion channels)
cmb_donnan_equilibrium               Donnan equilibrium with an impermeant anion  (membranes and diffusion / membrane potential)
cmb_water_movement                   Direction of water movement between a cell and its surroundings  (membranes and diffusion / water potential)
cmb_redox_free_energy                Free-energy change of a redox reaction from reduction potentials  (metabolism / free energy)
cmb_reaction_free_energy             Free-energy change of a reaction at cellular concentrations  (metabolism / free energy)
… 12 more
```

### 2.3 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ cell-molecular-biology list
cmb_absorbance_ratios                    A260/A280 and A260/A230 purity ratios
cmb_atp_yield                            ATP yield per glucose: aerobic respiration versus fermentation
cmb_binding_free_energy                  Standard binding free energy from Kd
cmb_buffer_after_addition                Buffer pH after adding strong acid or base
cmb_buffer_capacity                      Buffer capacity of a weak-acid buffer
cmb_buffer_preparation                   Recipe for a buffer of given pH and concentration
cmb_c1v1_dilution                        Dilution of a stock solution (C1 V1 = C2 V2)
cmb_cell_cycle_age_corrected             Cell-cycle phase durations corrected for the exponential age distribution
… 96 more
```

### 2.4 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
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

### 2.5 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ cell-molecular-biology solve cmb_nernst_potential z=1 "C_out=5 (mmol/L)" "C_in=140 (mmol/L)" "T=37 degC"
E = -0.08905869404 V
E_mV = -89.05869404
  by: Nernst equilibrium potential of an ion across a cell membrane  (E = (R T / z F) ln(C_out / C_in))
  C_out = 5 mol/m^3
  C_in = 140 mol/m^3
  T = 310.15 K
  assumes: z a non-zero whole number; concentrations > 0; T > 0
```

### 2.6 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ cell-molecular-biology solve cmb_nernst_potential z=1 "C_out=5 (mmol/L)" "C_in=140 (mmol/L)" "T=37 degC" --json
{"outcome":"SOLVED","message":"","value":-0.0890586940367,"unit":"V","solved_for":"E","si":[["C_out",5.0],["C_in",140.0],["T",310.15]],"values":[["E",-0.0890586940367,"V"],["E_mV",-89.0586940367,""]],"target":"E","phrase":"equilibrium potential (inside minus outside)","assumptions":["z a non-zero whole number","concentrations > 0","T > 0"],"name":"Nernst equilibrium potential of an ion across a cell membrane","equation":"E = (R T / z F) ln(C_out / C_in)"}
```

### 2.7 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ cell-molecular-biology solve cmb_nernst_potential z=1 "C_out=5 (mmol/L)" "C_in=140 (mmol/L)"
NEEDS_INPUT: cmb_nernst_potential needs T (absolute temperature (degC is converted)). Example: solve('cmb_nernst_potential', z=1, C_out='5 mol/m^3', C_in='140 mol/m^3', T='310.15 K').
```

### 2.8 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | cell-molecular-biology batch

`cases.jsonl`:

```json
{"key": "cmb_nernst_potential", "inputs": {"z": "1", "C_out": "5 (mmol/L)", "C_in": "140 (mmol/L)", "T": "37 degC"}}
{"key": "cmb_tm_nearest_neighbor", "inputs": {"seq": "CGTTCCAAAGATGTGGGCATGAGCTTAC", "Na": "50 (mmol/L)", "c1": "250 (nmol/L)", "c2": "250 (nmol/L)"}}
```

```shell
$ cell-molecular-biology batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":-0.0890586940367,"unit":"V","solved_for":"E","si":[["C_out",5.0],["C_in",140.0],["T",310.15]],"values":[["E",-0.0890586940367,"V"],["E_mV",-89.0586940367,""]],"target":"E","phrase":"equilibrium potential (inside minus outside)","assumptions":["z a non-zero whole number","concentrations > 0","T > 0"],"name":"Nernst equilibrium potential of an ion across a cell membrane","equation":"E = (R T / z F) ln(C_out / C_in)"}
{"outcome":"SOLVED","message":"","value":62.5816284138,"unit":"","solved_for":"Tm","si":[["Na",50.0],["c1",0.00025],["c2",0.00025]],"values":[["Tm",62.5816284138,""],["dH",-221.7,""],["dS",-628.76559587,""],["dG37",-26.6883504409,""]],"target":"Tm","phrase":"melting temperature in degC","assumptions":["DNA letters, length >= 2","Na+ > 0","c1 > c2/2 (self-complementary: k = c1)"],"name":"Nearest-neighbour melting temperature of a DNA duplex","equation":"sum nearest-neighbour dH and dS with initiation and terminal A/T terms (+ symmetry); dS += 0.368 (N-1) ln[Na+]; Tm = 1000 dH / (dS + R ln k) - 273.15, k = c1 - c2/2"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `cell-molecular-biology info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`cell-molecular-biology solve ... --json` prints one answer as one line of JSON. `cell-molecular-biology batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | cell-molecular-biology batch`. `cell-molecular-biology catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["cell-molecular-biology", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`cell-molecular-biology figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `cell_molecular_biology_ai999x.wasm` (https://www.ai999x.com/compute/rust/cell_molecular_biology_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("cell_molecular_biology_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `cell-molecular-biology info <id>` shows the same inputs and outputs in the terminal.

### Cell growth (14)

<a id="cmb_monod_batch_time"></a>

#### Batch culture time under Monod kinetics: `cmb_monod_batch_time`

The time for a batch culture (constant yield Y, Monod growth) to grow from biomass X0 to X, and the substrate left then.

$$
t = \frac{1}{\mu_{max}}\left[\left(1 + \frac{K_sY}{B}\right)\ln\frac{X}{X_0} - \frac{K_sY}{B}\ln\frac{B - X}{B - X_0}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `X0` | in | initial biomass concentration | kg/m^3 |
| `S0` | in | initial substrate concentration | kg/m^3 |
| `Y` | in | biomass yield (g biomass per g substrate) | - |
| `mu_max` | in | maximum specific growth rate | 1/s |
| `Ks` | in | half-saturation constant | kg/m^3 |
| `X` | in | target biomass concentration | kg/m^3 |
| `t` | out | time to reach X | s |
| `t_h` | out | time in hours | - |
| `S` | out | substrate remaining | kg/m^3 |
| `S_g_L` | out | substrate remaining in g/L | - |

**Applies when:** all inputs > 0; X0 < X < X0 + Y S0; no maintenance or death.

```shell
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

**What the worked example means:** This calculation determines the time required for a microbial population to grow from an initial biomass of 0.1 g/L to a final biomass of 4 g/L under Monod kinetics. It uses the maximum specific growth rate of 0.72 1/h, the half-saturation constant of 0.2 g/L, and the yield coefficient of 0.5. The result, 5.26514, represents the duration in hours needed for this growth phase. Simultaneously, it computes the remaining substrate concentration as 2.2, indicating how much nutrient is left after the specified biomass increase.

<a id="cmb_hemocytometer_count"></a>

#### Cell concentration from a hemocytometer count, `cmb_hemocytometer_count`

The cell concentration from counts in the large (1 mm x 1 mm x 0.1 mm) squares of a hemocytometer, the dilution factor and, optionally, the total cells in a suspension volume.

$$
c = \bar n \times d \times 10^4\ \mathrm{mL^{-1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `counts` | in | cells counted in each large square | - |
| `dilution` | in | dilution factor of the counted sample (1 for undiluted, 2 for 1:1 trypan blue) | - |
| `V` | in | volume of the cell suspension (optional) (optional) | m^3 |
| `mean_count` | out | mean count per large square | - |
| `cells_per_mL` | out | cells per mL | - |
| `total_cells` | out | total cells in V (when V is given) | - |

**Applies when:** counts >= 0; dilution >= 1; large squares of 0.1 uL.

```shell
$ cell-molecular-biology solve cmb_hemocytometer_count "counts=[52, 48, 55, 45]" dilution=2
mean_count = 50
cells_per_mL = 1000000
  by: Cell concentration from a hemocytometer count  (cells/mL = mean count x dilution x 10^4)
  dilution = 2
  assumes: counts >= 0; dilution >= 1; large squares of 0.1 uL
```

**What the worked example means:** This calculation determines the density of cells in a suspension using data from a hemocytometer. The process begins by averaging the number of cells counted in the large squares to obtain the mean count per square, which is 50. This value is then multiplied by the dilution factor, 2, to account for any reduction in sample concentration due to staining or mixing. Finally, the result is scaled by ten thousand because each large square holds one tenth of a microliter. The resulting concentration, 1e+06, represents the number of viable cells present in every milliliter of the original undiluted sample.

<a id="cmb_cell_cycle_age_corrected"></a>

#### Cell-cycle phase durations corrected for the exponential age distribution, `cmb_cell_cycle_age_corrected`

The durations of G1, S, G2 and M from phase fractions of an exponentially growing asynchronous population, using the age distribution n(a) = 2 ln2/T 2^(-a/T) (young cells are over-represented).

$$
F(a) = 2\left(1 - 2^{-a/T}\right),\quad a_k = -T\log_2\!\left(1 - \frac{F_k}{2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | total cell-cycle (doubling) time | s |
| `fractions` | in | fractions of cells in G1, S, G2 and M (in this order, summing to 1) | - |
| `G1_h` | out | G1 duration in hours | - |
| `S_h` | out | S duration in hours | - |
| `G2_h` | out | G2 duration in hours | - |
| `M_h` | out | M duration in hours | - |

**Applies when:** Tc > 0; four positive fractions summing to 1; balanced exponential growth.

```shell
$ cell-molecular-biology solve cmb_cell_cycle_age_corrected "Tc=24 h" "fractions=[0.45, 0.35, 0.15, 0.05]"
G1_h = 8.825562828
S_h = 8.861611432
G2_h = 4.623481871
M_h = 1.689343869
  by: Cell-cycle phase durations corrected for the exponential age distribution  (fraction younger than age a: F(a) = 2(1 - 2^(-a/T)); phase end ages a_k = -T log2(1 - F_k/2) from cumulative fractions)
  Tc = 86400 s
  assumes: Tc > 0; four positive fractions summing to 1; balanced exponential growth
```

**What the worked example means:** This calculation determines the actual durations of the cell cycle phases by accounting for the exponential age distribution inherent in asynchronously growing populations. Given a total doubling time of 24 h and specific proportions of cells in each phase, the model computes the corrected length of G1 as 8.82556. These values reflect the true temporal progression of the cell cycle rather than simple population snapshots.

<a id="cmb_cell_cycle_phase_durations"></a>

#### Cell-cycle phase durations from phase fractions, `cmb_cell_cycle_phase_durations`

The durations of G1, S, G2 and M from the fraction of an asynchronous population in each phase (simple proportion to the cycle time).

$$
t_{phase} = f_{phase}\,T_c
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | total cell-cycle time | s |
| `fractions` | in | fractions of cells in G1, S, G2 and M (in this order, summing to 1) | - |
| `G1_h` | out | G1 duration in hours | - |
| `S_h` | out | S duration in hours | - |
| `G2_h` | out | G2 duration in hours | - |
| `M_h` | out | M duration in hours | - |

**Applies when:** Tc > 0; four positive fractions summing to 1.

```shell
$ cell-molecular-biology solve cmb_cell_cycle_phase_durations "Tc=24 h" "fractions=[0.45, 0.35, 0.15, 0.05]"
G1_h = 10.8
S_h = 8.4
G2_h = 3.6
M_h = 1.2
  by: Cell-cycle phase durations from phase fractions  (duration of a phase = fraction x Tc)
  Tc = 86400 s
  assumes: Tc > 0; four positive fractions summing to 1
```

**What the worked example means:** This calculation determines the time spent in each phase of the cell cycle by multiplying the total cycle duration 24 h by the fraction of cells observed in that specific phase. The resulting values represent how long a typical cell remains in G1, S, G2 and M before progressing to the next stage. For example, the G1 phase lasts 10.8, while the synthesis phase lasts 8.4. These estimates assume an asynchronous population where the proportion of cells in a given phase reflects the relative duration of that phase within the entire cycle.

<a id="cmb_plate_count"></a>

#### Colony-forming units per mL from a plate count, `cmb_plate_count`

The viable count (CFU per mL) of the original culture from the colonies on a plate, the dilution factor of the plated sample and the plated volume.

$$
\mathrm{CFU/mL} = \frac{n \times DF}{V_{plated}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `colonies` | in | colonies counted on the plate | - |
| `dilution` | in | dilution factor of the plated sample (1e6 for a 10^-6 dilution) | - |
| `V` | in | volume plated | m^3 |
| `cfu_per_mL` | out | colony-forming units per mL of the original culture | - |
| `log10_cfu_per_mL` | out | log10 of CFU per mL (0 when no colonies) | - |

**Applies when:** colonies a whole number; dilution >= 1; V > 0.

```shell
$ cell-molecular-biology solve cmb_plate_count colonies=150 dilution=1000000 "V=0.1 mL"
cfu_per_mL = 1500000000
log10_cfu_per_mL = 9.176091259
  by: Colony-forming units per mL from a plate count  (CFU/mL = colonies x dilution factor / volume plated (mL))
  dilution = 1000000
  V = 1e-07 m^3
  assumes: colonies a whole number; dilution >= 1; V > 0
```

**What the worked example means:** This calculation determines the concentration of viable microorganisms in the original sample. It uses the number of colonies counted on the plate, which was 150, multiplied by the dilution factor of 1e+06, and divided by the volume plated of 0.1 mL. The result represents the estimated density of living cells capable of forming colonies in the undiluted culture, expressed as 1.5e+09 CFU per mL.

<a id="cmb_doubling_time_from_counts"></a>

#### Doubling time and number of generations from two counts, `cmb_doubling_time_from_counts`

The doubling (generation) time, the number of generations and the specific growth rate from the cell counts at the start and end of an interval of exponential growth.

$$
n = \log_2\frac{N}{N_0},\quad t_d = \frac{t}{n},\quad \mu = \frac{\ln(N/N_0)}{t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N0` | in | initial cell count (or concentration) | - |
| `N` | in | final cell count (same unit) | - |
| `t` | in | elapsed time | s |
| `td` | out | doubling time | s |
| `td_h` | out | doubling time in hours | - |
| `generations` | out | number of generations log2(N/N0) | - |
| `mu` | out | specific growth rate | 1/s |
| `mu_per_h` | out | specific growth rate per hour | - |

**Applies when:** N > N0 > 0; t > 0; exponential growth over the interval.

```shell
$ cell-molecular-biology solve cmb_doubling_time_from_counts N0=10000 N=160000 "t=8 h"
td = 7200 s
td_h = 2
generations = 4
mu = 9.627044174e-05 1/s
mu_per_h = 0.3465735903
  by: Doubling time and number of generations from two counts  (n = log2(N/N0) = 3.32 log10(N/N0); td = t/n; mu = ln 2 / td)
  N0 = 10000
  N = 160000
  t = 28800 s
  assumes: N > N0 > 0; t > 0; exponential growth over the interval
```

**What the worked example means:** This calculation determines the growth kinetics of a microbial population during exponential phase. By comparing the initial cell count of 10000 to the final count of 160000 over an elapsed time of 8 h, the population completed 4 generations. Consequently, the time required for the population to double in size, known as the doubling time, is 7200 s. The specific growth rate, which represents the instantaneous rate of increase per unit of biomass or cell number, is calculated as 9.62704e-05 1/s. These metrics allow researchers to quantify how rapidly the cells are proliferating under the given experimental conditions.

<a id="cmb_exponential_growth"></a>

#### Exponential growth of a cell population: `cmb_exponential_growth`

The number of cells after a time t of exponential growth from N0 with doubling time td, the number of generations and the specific growth rate.

$$
N = N_0\,2^{t/t_d} = N_0 e^{\mu t},\quad \mu = \frac{\ln 2}{t_d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N0` | in | initial number (or concentration) of cells | - |
| `td` | in | doubling (generation) time | s |
| `t` | in | elapsed time | s |
| `N` | out | number of cells at time t | - |
| `generations` | out | number of generations t/td | - |
| `fold` | out | fold increase N/N0 | - |
| `mu` | out | specific growth rate ln2/td | 1/s |
| `mu_per_h` | out | specific growth rate per hour | - |

**Applies when:** N0, td > 0; t >= 0; balanced exponential growth (no lag, no limitation).

```shell
$ cell-molecular-biology solve cmb_exponential_growth N0=1000 "td=0.3333333333 h" "t=2 h"
N = 64000.00003
generations = 6.000000001
fold = 64.00000003
mu = 0.0005776226505 1/s
mu_per_h = 2.079441542
  by: Exponential growth of a cell population  (N = N0 2^(t/td); mu = ln 2 / td)
  N0 = 1000
  td = 1200 s
  t = 7200 s
  assumes: N0, td > 0; t >= 0; balanced exponential growth (no lag, no limitation)
```

**What the worked example means:** This calculation determines the final cell count using exponential growth kinetics. Given an initial population of 1000 and a specific doubling time of 0.333333 h, the model computes that after an elapsed time of 2 h, the culture contains 64000 cells. This represents 6 generations, corresponding to a fold increase of 64. The specific growth rate is calculated as 0.000577623 1/s per second, which converts to 2.07944 per hour.

<a id="cmb_logistic_growth"></a>

#### Logistic growth of a population: `cmb_logistic_growth`

The population at time t under logistic growth with carrying capacity K and intrinsic rate r, the growth rate dN/dt there and the time of the inflection point (N = K/2).

$$
N(t) = \frac{K}{1 + \frac{K - N_0}{N_0}e^{-rt}},\quad \frac{dN}{dt} = rN\left(1 - \frac{N}{K}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N0` | in | initial population | - |
| `K` | in | carrying capacity (same unit) | - |
| `r` | in | intrinsic growth rate | 1/s |
| `t` | in | elapsed time | s |
| `N` | out | population at time t | - |
| `dNdt` | out | growth rate at time t in cells per second | - |
| `dNdt_per_h` | out | growth rate at time t in cells per hour | - |
| `t_inflection` | out | time when N = K/2 (negative if already past) | s |
| `t_inflection_h` | out | time of the inflection point in hours | - |

**Applies when:** 0 < N0 < K; r > 0; t >= 0.

```shell
$ cell-molecular-biology solve cmb_logistic_growth N0=10000 K=100000000 "r=0.72 1/h" "t=10 h"
N = 11813193.49
dNdt = 2083.535616
dNdt_per_h = 7500728.219
t_inflection = 46051.20183 s
t_inflection_h = 12.79200051
  by: Logistic growth of a population  (N = K / (1 + ((K - N0)/N0) e^(-r t)); dN/dt = r N (1 - N/K); t_infl = ln((K - N0)/N0)/r)
  N0 = 10000
  K = 100000000
  r = 0.0002 1/s
  t = 36000 s
  assumes: 0 < N0 < K; r > 0; t >= 0
```

**What the worked example means:** This calculation determines the state of a population undergoing logistic growth. Using the initial population 10000, carrying capacity 1e+08, intrinsic growth rate 0.72 1/h, and elapsed time 10 h, this gives the current population size 1.18132e+07. The instantaneous growth rate is found to be 2083.54 cells per second, which equals 7.50073e+06 cells per hour. Additionally, the time when the population reaches half the carrying capacity, known as the inflection point, is calculated as 46051.2 s seconds, or 12.792 hours.

<a id="cmb_monod_growth_rate"></a>

#### Monod specific growth rate: `cmb_monod_growth_rate`

The specific growth rate of a microbial culture limited by one substrate (Monod), its fraction of the maximum and the corresponding doubling time.

$$
\mu = \mu_{max}\frac{S}{K_s + S}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu_max` | in | maximum specific growth rate | 1/s |
| `S` | in | limiting substrate concentration | kg/m^3 |
| `Ks` | in | half-saturation constant | kg/m^3 |
| `mu` | out | specific growth rate | 1/s |
| `mu_per_h` | out | growth rate per hour | - |
| `fraction` | out | mu / mu_max | - |
| `td_h` | out | doubling time ln2/mu in hours (0 when mu = 0) | - |

**Applies when:** mu_max, Ks > 0; S >= 0; a single limiting substrate.

```shell
$ cell-molecular-biology solve cmb_monod_growth_rate "mu_max=0.72 1/h" "S=50 mg/L" "Ks=10 mg/L"
mu = 0.0001666666667 1/s
mu_per_h = 0.6
fraction = 0.8333333333
td_h = 1.155245301
  by: Monod specific growth rate  (mu = mu_max S / (Ks + S))
  mu_max = 0.0002 1/s
  S = 0.05 kg/m^3
  Ks = 0.01 kg/m^3
  assumes: mu_max, Ks > 0; S >= 0; a single limiting substrate
```

**What the worked example means:** This calculation determines the specific growth rate of a microbial culture using Monod kinetics. By combining the maximum specific growth rate 0.72 1/h with the limiting substrate concentration 50 mg/L and the half-saturation constant 10 mg/L, the model computes the actual growth rate as 0.000166667 1/s. This result indicates that the culture is growing at 0.833333 of its theoretical maximum capacity, corresponding to a doubling time of 1.15525 hours.

<a id="cmb_molecules_per_cell"></a>

#### Number of molecules in a cell from a concentration, `cmb_molecules_per_cell`

The number of molecules of a species at concentration c in a compartment of volume V, and the concentration that one molecule per compartment represents.

$$
N = c\,V\,N_A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | concentration | mol/m^3 |
| `V` | in | cell (compartment) volume | m^3 |
| `N` | out | number of molecules | - |
| `one_molecule` | out | concentration of one molecule in V | mol/m^3 |
| `one_molecule_nM` | out | one molecule per compartment in nM | - |

**Applies when:** c, V > 0.

```shell
$ cell-molecular-biology solve cmb_molecules_per_cell "c=1 (nmol/L)" "V=1 fL"
N = 0.602214076
one_molecule = 1.660539067e-06 mol/m^3
one_molecule_nM = 1.660539067
  by: Number of molecules in a cell from a concentration  (N = c V N_A)
  c = 1e-06 mol/m^3
  V = 1e-18 m^3
  assumes: c, V > 0
```

**What the worked example means:** This calculation determines the total count of specific molecules present within a cellular compartment based on its volume and the local concentration. By multiplying the concentration 1 (nmol/L) by the compartment volume 1 fL and Avogadro's constant, this gives the absolute number of molecules, which is 0.602214. Additionally, the model computes the concentration that would result from having exactly one single molecule in this space, yielding 1.66054e-06 mol/m^3 or approximately 1.66054 when converted to nanomolar units.

<a id="cmb_growth_rate_fit"></a>

#### Specific growth rate fitted to growth-curve data, `cmb_growth_rate_fit`

The specific growth rate, doubling time and back-extrapolated initial count from counts (or OD) measured during exponential growth, by a least-squares line through ln N against time.

$$
\ln N = \ln N_0 + \mu t,\quad t_d = \frac{\ln 2}{\mu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | sampling times in hours | - |
| `N` | in | cell counts or optical densities (positive) | - |
| `mu_per_h` | out | specific growth rate per hour | - |
| `td_h` | out | doubling time in hours | - |
| `N0_fit` | out | fitted count at t = 0 | - |
| `r2` | out | coefficient of determination of the log-linear fit | - |

**Applies when:** at least 3 points; counts > 0, increasing; exponential phase only.

```shell
$ cell-molecular-biology solve cmb_growth_rate_fit "t=[0, 1, 2, 3, 4]" "N=[1000, 1980, 4100, 7900, 16200]"
mu_per_h = 0.6953788399
td_h = 0.9967907287
N0_fit = 998.4351099
r2 = 0.9998073985
  by: Specific growth rate fitted to growth-curve data  (least-squares line ln N = ln N0 + mu t; td = ln 2 / mu)
  assumes: at least 3 points; counts > 0, increasing; exponential phase only
```

**What the worked example means:** This calculation determines the exponential growth kinetics by applying a least-squares linear regression to the natural logarithm of cell counts or optical densities plotted against sampling times. The slope of this line yields the specific growth rate, reported as 0.695379, while the intercept provides the back-extrapolated initial population size, 998.435. From the growth rate, the doubling time is derived as 0.996791 hours using the relationship between exponential increase and generation time. The coefficient of determination, 0.999807, indicates how well the log-linear model fits the observed data during the exponential phase.

<a id="cmb_logistic_time_to_reach"></a>

#### Time for a logistic population to reach a size, `cmb_logistic_time_to_reach`

The time for a population growing logistically from N0 (carrying capacity K, rate r) to reach N.

$$
t = \frac{1}{r}\ln\frac{N (K - N_0)}{N_0 (K - N)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N0` | in | initial population | - |
| `K` | in | carrying capacity | - |
| `r` | in | intrinsic growth rate | 1/s |
| `N` | in | target population | - |
| `t` | out | time to reach N | s |
| `t_h` | out | time in hours | - |

**Applies when:** 0 < N0 < N < K; r > 0.

```shell
$ cell-molecular-biology solve cmb_logistic_time_to_reach N0=10000 K=100000000 "r=0.72 1/h" N=50000000
t = 46051.20183 s
t_h = 12.79200051
  by: Time for a logistic population to reach a size  (t = (1/r) ln[ N (K - N0) / (N0 (K - N)) ])
  N0 = 10000
  K = 100000000
  r = 0.0002 1/s
  N = 50000000
  assumes: 0 < N0 < N < K; r > 0
```

**What the worked example means:** This calculation determines the duration required for a population starting at 10000 to grow to 5e+07 under logistic constraints defined by carrying capacity 1e+08 and intrinsic growth rate 0.72 1/h. The result, expressed as 46051.2 s, represents the specific elapsed time in seconds. This value indicates how long the biological system takes to approach the specified density while respecting environmental limits.

<a id="cmb_transformation_efficiency"></a>

#### Transformation efficiency: `cmb_transformation_efficiency`

Transformants per microgram of DNA from the colonies on a plate, the DNA mass used and the fraction of the transformation plated.

$$
\eta = \frac{n}{m_{DNA}\,(V_{plated}/V_{total})}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `colonies` | in | transformant colonies counted | - |
| `m_DNA` | in | DNA mass used in the transformation | kg |
| `V_plated` | in | volume plated | m^3 |
| `V_total` | in | total volume after recovery | m^3 |
| `efficiency` | out | transformants per microgram of DNA | - |
| `log10_efficiency` | out | log10 of the efficiency | - |
| `DNA_plated_ug` | out | micrograms of DNA plated | - |

**Applies when:** colonies a whole number; m_DNA, volumes > 0; V_plated <= V_total.

```shell
$ cell-molecular-biology solve cmb_transformation_efficiency colonies=200 "m_DNA=1 ng" "V_plated=100 uL" "V_total=1000 uL"
efficiency = 2000000
log10_efficiency = 6.301029996
DNA_plated_ug = 0.0001
  by: Transformation efficiency  (efficiency = colonies / (ug DNA x V_plated / V_total))
  m_DNA = 1e-12 kg
  V_plated = 1e-07 m^3
  V_total = 1e-06 m^3
  assumes: colonies a whole number; m_DNA, volumes > 0; V_plated <= V_total
```

**What the worked example means:** This calculation determines the number of successful transformants per microgram of DNA used. It takes the count of visible colonies, which was 200, and divides it by the mass of DNA actually plated. This plated mass is derived from the total DNA input of 1 ng scaled by the fraction of the recovery volume that was spread on the plate, calculated as 100 uL divided by 1000 uL. The resulting efficiency value of 2e+06 indicates how effectively the cells took up the foreign DNA under these specific conditions.

<a id="cmb_cell_geometry"></a>

#### Volume, surface area and surface-to-volume ratio of a spherical cell, `cmb_cell_geometry`

The volume (fL), surface area and surface-to-volume ratio of a spherical cell of a given diameter.

$$
V = \tfrac{4}{3}\pi r^3,\quad A = 4\pi r^2,\quad \frac{A}{V} = \frac{3}{r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `d` | in | cell diameter | m |
| `V` | out | cell volume | m^3 |
| `V_fL` | out | volume in femtolitres (um^3) | - |
| `A` | out | surface area | m^2 |
| `A_um2` | out | surface area in um^2 | - |
| `SA_V_per_um` | out | surface-to-volume ratio in 1/um | - |

**Applies when:** d > 0; a sphere.

```shell
$ cell-molecular-biology solve cmb_cell_geometry "d=10 um"
V = 5.235987756e-16 m^3
V_fL = 523.5987756
A = 3.141592654e-10 m^2
A_um2 = 314.1592654
SA_V_per_um = 0.6
  by: Volume, surface area and surface-to-volume ratio of a spherical cell  (V = (4/3) pi r^3, A = 4 pi r^2, A/V = 3/r)
  d = 1e-05 m
  assumes: d > 0; a sphere
```

**What the worked example means:** This calculation determines the geometric properties of a spherical cell based on its diameter. Using the input diameter 10 um, the model computes the total volume as 5.23599e-16 m^3 and the surface area as 3.14159e-10 m^2. These values are also provided in more biologically relevant units, showing a volume of 523.599 and a surface area of 314.159. The resulting surface-to-volume ratio is 0.6, which indicates how much membrane area is available per unit of internal volume.

### DNA and RNA (20)

<a id="cmb_absorbance_ratios"></a>

#### A260/A280 and A260/A230 purity ratios: `cmb_absorbance_ratios`

The A260/A280 and A260/A230 ratios of a nucleic acid sample after subtracting the A320 background (about 1.8 for pure DNA, 2.0 for pure RNA; lower values suggest protein or organic contaminants).

$$
\frac{A_{260} - A_{320}}{A_{280} - A_{320}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A260` | in | absorbance at 260 nm | - |
| `A280` | in | absorbance at 280 nm | - |
| `A230` | in | absorbance at 230 nm (optional) (optional) | - |
| `A320` | in | background absorbance at 320 nm (default 0) (optional) | - |
| `ratio_260_280` | out | A260/A280 after background subtraction | - |
| `ratio_260_230` | out | A260/A230 after background subtraction (when A230 is given) | - |

**Applies when:** absorbances > 0; A320 below the other readings.

```shell
$ cell-molecular-biology solve cmb_absorbance_ratios A260=0.9 A280=0.5 A230=0.42
ratio_260_280 = 1.8
ratio_260_230 = 2.142857143
  by: A260/A280 and A260/A230 purity ratios  (ratio = (A260 - A320)/(A280 - A320); (A260 - A320)/(A230 - A320))
  A260 = 0.9
  A280 = 0.5
  A230 = 0.42
  assumes: absorbances > 0; A320 below the other readings
```

**What the worked example means:** This calculation determines nucleic acid purity by computing the ratios of absorbance at 260 nm to those at 280 nm and 230 nm, after subtracting the background absorbance at 320 nm from all readings. The resulting A260 to A280 ratio of 1.8 reflects protein contamination levels, while the A260 to A230 ratio of 2.14286 indicates organic or salt contaminants. Pure DNA typically yields a value near 1.8 and pure RNA near 2.0 for the first ratio; deviations suggest impurities that may affect downstream applications.

<a id="cmb_tm_basic"></a>

#### Basic GC melting temperature of an oligonucleotide, `cmb_tm_basic`

The basic melting temperature of a DNA oligonucleotide of 14 nt or more, Tm = 64.9 + 41 (G + C - 16.4) / N (50 mM Na+ assumed, no salt term).

$$
T_m = 64.9 + 41\,\frac{n_G + n_C - 16.4}{N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA sequence (A, C, G, T), at least 14 nt | - |
| `Tm` | out | melting temperature in degC | - |
| `gc_percent` | out | GC content in per cent | - |
| `length` | out | length in nt | - |

**Applies when:** DNA letters only; N >= 14.

```shell
$ cell-molecular-biology solve cmb_tm_basic seq=AGCGGATAACAATTTCACACAGGA
Tm = 53.96666667
gc_percent = 41.66666667
length = 24
  by: Basic GC melting temperature of an oligonucleotide  (Tm = 64.9 + 41 (nG + nC - 16.4) / N)
  seq = AGCGGATAACAATTTCACACAGGA
  assumes: DNA letters only; N >= 14
```

**What the worked example means:** This calculation determines the basic melting temperature for a DNA oligonucleotide. It uses the sequence length, which is 24, and the number of guanine and cytosine bases to estimate stability. The resulting temperature, 53.9667, indicates where half of the double-stranded DNA dissociates into single strands under standard salt conditions.

<a id="cmb_dna_copy_number"></a>

#### Copy number and moles of a DNA or RNA molecule from its mass, `cmb_dna_copy_number`

The molecular weight, picomoles and number of copies in a mass of dsDNA, ssDNA or RNA of a given length (NEB average-mass formulas).

$$
N = \frac{m}{M}\,N_A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of nucleic acid | kg |
| `n` | in | length in base pairs (dsDNA) or bases (ssDNA, RNA) | - |
| `molecule` | in | dsDNA, ssDNA or RNA | - |
| `MW` | out | molecular weight | kg/mol |
| `MW_g_mol` | out | molecular weight in g/mol | - |
| `pmol` | out | picomoles | - |
| `copies` | out | number of molecules | - |

**Applies when:** m > 0; n a whole number >= 1.

```shell
$ cell-molecular-biology solve cmb_dna_copy_number "m=1 ng" n=3000 molecule=dsDNA
MW = 1853.91604 kg/mol
MW_g_mol = 1853916.04
pmol = 0.0005393987529
copies = 324833521.6
  by: Copy number and moles of a DNA or RNA molecule from its mass  (MW = 617.96 n + 36.04 (dsDNA), 308.97 n + 18.02 (ssDNA), 320.5 n + 159.0 (RNA) g/mol; copies = m / MW x N_A)
  m = 1e-12 kg
  molecule = dsDNA
  assumes: m > 0; n a whole number >= 1
```

**What the worked example means:** This calculation determines the molecular weight, amount in picomoles, and total number of copies for a specific mass of nucleic acid. The process begins by computing the average molecular weight based on the molecule type and length, yielding 1853.92 kg/mol. This value is then converted to grams per mole as 1.85392e+06. Using this weight, the input mass 1 ng is divided by the molecular weight and multiplied by Avogadro's constant to find the molar quantity, expressed here as 0.000539399. Finally, the absolute number of individual molecules, or copies, is derived from these moles, resulting in 3.24834e+08.

<a id="cmb_oligo_extinction_coefficient"></a>

#### Extinction coefficient of a DNA oligonucleotide at 260 nm (nearest neighbour), `cmb_oligo_extinction_coefficient`

The molar extinction coefficient at 260 nm of a single-stranded DNA oligonucleotide by the nearest-neighbour method, with micrograms and nanomoles per A260 unit (5'-hydroxyl oligo, 1 mL, 1 cm).

$$
\varepsilon_{260} = \sum_{i=1}^{N-1}\varepsilon_{i,i+1} - \sum_{i=2}^{N-1}\varepsilon_i
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA oligonucleotide 5'->3' (A, C, G, T) | - |
| `eps` | out | molar extinction coefficient at 260 nm | m^2/mol |
| `eps_L_mol_cm` | out | extinction coefficient in L/(mol cm) | - |
| `ug_per_A260` | out | micrograms per A260 unit (per mL at 1 cm) | - |
| `nmol_per_A260` | out | nanomoles per A260 unit (per mL at 1 cm) | - |

**Applies when:** DNA letters only; single strand.

```shell
$ cell-molecular-biology solve cmb_oligo_extinction_coefficient seq=ATGCATGCAT
eps = 9800 m^2/mol
eps_L_mol_cm = 98000
ug_per_A260 = 30.8878561
nmol_per_A260 = 10.20408163
  by: Extinction coefficient of a DNA oligonucleotide at 260 nm (nearest neighbour)  (eps = sum of nearest-neighbour pair values - sum of internal single-base values (Cantor-Warshaw-Shapiro table))
  seq = ATGCATGCAT
  assumes: DNA letters only; single strand
```

**What the worked example means:** This calculation determines the molar extinction coefficient at 260 nanometers for a single-stranded DNA oligonucleotide using the nearest neighbour method. The result, 9800 m^2/mol, represents the light absorption capacity of one mole of the sequence in square meters per mole. This gives practical conversion factors: 30.8879 indicates how many micrograms of the specific sequence correspond to an absorbance reading of one unit, while 10.2041 shows the equivalent amount in nanomoles. These values allow researchers to accurately quantify the concentration of their DNA samples from spectrophotometric readings.

<a id="cmb_gc_content"></a>

#### GC content of a DNA or RNA sequence: `cmb_gc_content`

The fraction and percentage of G + C in a DNA or RNA sequence, with the base counts and the length.

$$
\mathrm{GC} = \frac{n_G + n_C}{N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA or RNA sequence (A, C, G, T or U) | - |
| `gc_fraction` | out | fraction of G + C | - |
| `gc_percent` | out | GC content in per cent | - |
| `length` | out | length in bases | - |
| `G` | out | number of G | - |
| `C` | out | number of C | - |
| `A` | out | number of A | - |
| `T_or_U` | out | number of T (DNA) or U (RNA) | - |

**Applies when:** only A, C, G, T or U.

```shell
$ cell-molecular-biology solve cmb_gc_content seq=ATGCGCGATTACGC
gc_fraction = 0.5714285714
gc_percent = 57.14285714
length = 14
G = 4
C = 4
A = 3
T_or_U = 3
  by: GC content of a DNA or RNA sequence  (GC = (n_G + n_C) / N)
  seq = ATGCGCGATTACGC
  assumes: only A, C, G, T or U
```

**What the worked example means:** This calculation determines the proportion of guanine and cytosine bases in the provided nucleic acid sequence. By summing the count of guanine residues, which is 4, and cytosine residues, which is 4, and dividing by the total length of the sequence, 14, this gives the GC fraction of 0.571429. This value is then expressed as a percentage, resulting in a GC content of 57.1429. The remaining bases are accounted for by 3 adenines and 3 thymines or uracils, ensuring that all positions in the sequence are classified according to their specific nitrogenous base type.

<a id="cmb_ligation_insert_mass"></a>

#### Insert mass for a ligation at an insert:vector molar ratio, `cmb_ligation_insert_mass`

The mass of insert DNA needed for a ligation at a chosen insert:vector molar ratio, and the femtomoles of vector and insert (660 g/mol per bp).

$$
m_{ins} = r\,m_{vec}\,\frac{L_{ins}}{L_{vec}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m_vector` | in | mass of vector DNA | kg |
| `L_vector` | in | vector length in bp | - |
| `L_insert` | in | insert length in bp | - |
| `ratio` | in | insert:vector molar ratio (e.g. 3 for 3:1) | - |
| `m_insert` | out | mass of insert | kg |
| `m_insert_ng` | out | mass of insert in ng | - |
| `vector_fmol` | out | vector in fmol | - |
| `insert_fmol` | out | insert in fmol | - |

**Applies when:** masses, ratio > 0; lengths whole numbers >= 1.

```shell
$ cell-molecular-biology solve cmb_ligation_insert_mass "m_vector=50 ng" L_vector=3000 L_insert=1000 ratio=3
m_insert = 5e-11 kg
m_insert_ng = 50
vector_fmol = 25.25252525
insert_fmol = 75.75757576
  by: Insert mass for a ligation at an insert:vector molar ratio  (m_insert = ratio x m_vector x L_insert / L_vector)
  m_vector = 5e-11 kg
  ratio = 3
  assumes: masses, ratio > 0; lengths whole numbers >= 1
```

**What the worked example means:** This calculation determines the mass of insert DNA needed for a ligation reaction at a specific molar ratio relative to the vector. Using the input vector mass of 50 ng and lengths of 3000 base pairs for the vector and 1000 base pairs for the insert, the tool computes that you need 5e-11 kg of insert material. This result ensures the desired stoichiometry for efficient cloning.

<a id="cmb_orf_finder"></a>

#### Longest open reading frame on both strands, `cmb_orf_finder`

The longest open reading frame (ATG to the first in-frame stop) in all six frames of a DNA sequence with at least min_aa codons, its strand, coordinates on that strand, protein and the number of such ORFs.

$$
\mathrm{ORF} = \mathrm{ATG}\,(\mathrm{NNN})^{n}\,\{\mathrm{TAA,TAG,TGA}\}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA sequence 5'->3' | - |
| `min_aa` | in | minimum ORF length in codons (amino acids, stop excluded) | - |
| `strand` | out | + (given strand) or - (reverse complement) | - |
| `start` | out | 1-based start of the ATG on that strand (read 5'->3') | - |
| `end` | out | 1-based end of the stop codon on that strand | - |
| `length_nt` | out | ORF length in nt including the stop | - |
| `length_aa` | out | protein length in amino acids | - |
| `protein` | out | encoded protein (one-letter code) | - |
| `n_orfs` | out | number of ORFs of at least min_aa codons | - |

**Applies when:** DNA letters only; min_aa >= 1; at least one ORF.

```shell
$ cell-molecular-biology solve cmb_orf_finder seq=CCATGAAACGCATTAGCACCACCATTACCACCACCATCACCATTACCACAGGTAACGGTGCGGGCTGAGG min_aa=10
strand = +
start = 3
end = 68
length_nt = 66
length_aa = 21
protein = MKRISTTITTTITITTGNGAG
n_orfs = 1
  by: Longest open reading frame on both strands  (scan the three frames of each strand; an ORF starts at the first ATG after the previous stop and ends at the next stop; keep the longest (ties: + strand, then the lowest start))
  seq = CCATGAAACGCATTAGCACCACCATTACCACCACCATCACCATTACCACAGGTAACGGTGCGGGCTGAGG
  assumes: DNA letters only; min_aa >= 1; at least one ORF
```

**What the worked example means:** This calculation identifies the longest open reading frame within a DNA sequence by scanning all six possible frames. The result indicates that the most significant coding region lies on the + strand, beginning at position 3 and ending at position 68. This segment contains 21 amino acids and translates into the protein sequence MKRISTTITTTITITTGNGAG. A total of 1 distinct open reading frames meeting the minimum length criteria were detected in the input sequence.

<a id="cmb_oligo_molecular_weight"></a>

#### Molecular weight of a DNA or RNA oligonucleotide, `cmb_oligo_molecular_weight`

The average molecular weight of a single- or double-stranded DNA or RNA oligonucleotide, with a 5'-phosphate (sum of nucleotide monophosphates minus waters) and as the 5'-hydroxyl form of synthetic oligos.

$$
M = \sum_i m_i - (n - 1)\,M_{H_2O}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA or RNA sequence 5'->3' | - |
| `strands` | in | single strand or the duplex with its complement | - |
| `MW` | out | molecular weight (5'-phosphate) | kg/mol |
| `MW_g_mol` | out | molecular weight in g/mol (5'-phosphate) | - |
| `MW_hydroxyl_g_mol` | out | molecular weight in g/mol with 5'-hydroxyl ends | - |

**Applies when:** only A, C, G, T (DNA) or A, C, G, U (RNA).

```shell
$ cell-molecular-biology solve cmb_oligo_molecular_weight seq=ATGCATGCAT strands=single
MW = 3.1069898 kg/mol
MW_g_mol = 3106.9898
MW_hydroxyl_g_mol = 3027.009898
  by: Molecular weight of a DNA or RNA oligonucleotide  (MW = sum of nucleotide monophosphate weights - (n - 1) x 18.0153; 5'-OH form minus HPO3 (79.98) per strand)
  seq = ATGCATGCAT
  strands = single
  assumes: only A, C, G, T (DNA) or A, C, G, U (RNA)
```

**What the worked example means:** This calculation determines the average molecular weight of a DNA or RNA sequence by summing the weights of its individual nucleotide monophosphates and subtracting the mass of water molecules lost during phosphodiester bond formation. The result depends on whether the strand is single or double, as indicated by the input single. The primary output 3106.99 represents the weight in grams per mole assuming a 5'-phosphate group, while 3027.01 adjusts this value for synthetic oligos that typically have free hydroxyl ends at both termini.

<a id="cmb_mutation_rate_p0"></a>

#### Mutation rate by the fluctuation-test p0 method, `cmb_mutation_rate_p0`

The expected number of mutations per culture m = -ln p0 from the fraction of parallel cultures without mutants (Luria-Delbruck p0 method) and the mutation rate m / Nt.

$$
p_0 = e^{-m},\quad \mu = \frac{-\ln p_0}{N_t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n0` | in | number of cultures with no mutants | - |
| `n` | in | total number of cultures | - |
| `Nt` | in | final number of cells per culture | - |
| `p0` | out | fraction of cultures without mutants | - |
| `m` | out | mean mutations per culture | - |
| `mutation_rate` | out | mutations per cell per generation (m / Nt) | - |

**Applies when:** 1 <= n0 < n; Nt > 0; Poisson number of mutational events per culture.

```shell
$ cell-molecular-biology solve cmb_mutation_rate_p0 n0=11 n=20 Nt=200000000
p0 = 0.55
m = 0.5978370008
mutation_rate = 2.989185004e-09
  by: Mutation rate by the fluctuation-test p0 method  (p0 = e^-m, so m = -ln(n0/n); mu = m / Nt)
  Nt = 200000000
  assumes: 1 <= n0 < n; Nt > 0; Poisson number of mutational events per culture
```

**What the worked example means:** This calculation determines the mutation rate using the Luria-Delbruck p0 method. First, the fraction of cultures without mutants, denoted as 0.55, is calculated from the input counts of non-mutant cultures 11 and total cultures 20. The expected number of mutations per culture, 0.597837, is then derived as the negative natural logarithm of this fraction. Finally, the mutation rate 2.98919e-09 is obtained by dividing the mean mutations per culture by the final number of cells per culture 2e+08. This result represents the probability of a mutation occurring per cell division.

<a id="cmb_tm_nearest_neighbor"></a>

#### Nearest-neighbour melting temperature of a DNA duplex, `cmb_tm_nearest_neighbor`

The melting temperature of a perfectly matched DNA duplex by nearest-neighbour thermodynamics (SantaLucia-Hicks 2004 unified parameters) with the SantaLucia (1998) sodium correction, and the duplex dH, dS and dG at 37 degC.

$$
T_m = \frac{1000\,\Delta H^\circ}{\Delta S^\circ + 0.368(N-1)\ln[\mathrm{Na^+}] + R\ln k} - 273.15
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA sequence 5'->3' (A, C, G, T) | - |
| `Na` | in | monovalent cation (Na+) concentration | mol/m^3 |
| `c1` | in | concentration of the more concentrated strand | mol/m^3 |
| `c2` | in | concentration of the less concentrated strand (0 if in large excess) | mol/m^3 |
| `Tm` | out | melting temperature in degC | - |
| `dH` | out | duplex enthalpy in kcal/mol | - |
| `dS` | out | salt-corrected duplex entropy in cal/(mol K) | - |
| `dG37` | out | free energy at 37 degC in kcal/mol | - |

**Applies when:** DNA letters, length >= 2; Na+ > 0; c1 > c2/2 (self-complementary: k = c1).

```shell
$ cell-molecular-biology solve cmb_tm_nearest_neighbor seq=CGTTCCAAAGATGTGGGCATGAGCTTAC "Na=50 (mmol/L)" "c1=250 (nmol/L)" "c2=250 (nmol/L)"
Tm = 62.58162841
dH = -221.7
dS = -628.7655959
dG37 = -26.68835044
  by: Nearest-neighbour melting temperature of a DNA duplex  (sum nearest-neighbour dH and dS with initiation and terminal A/T terms (+ symmetry); dS += 0.368 (N-1) ln[Na+]; Tm = 1000 dH / (dS + R ln k) - 273.15, k = c1 - c2/2)
  Na = 50 mol/m^3
  c1 = 0.00025 mol/m^3
  c2 = 0.00025 mol/m^3
  seq = CGTTCCAAAGATGTGGGCATGAGCTTAC
  assumes: DNA letters, length >= 2; Na+ > 0; c1 > c2/2 (self-complementary: k = c1)
```

**What the worked example means:** This calculation determines the thermodynamic stability of a perfectly matched DNA duplex using nearest-neighbour parameters. It sums enthalpy and entropy contributions from adjacent base pairs, applies initiation and terminal corrections, and adjusts entropy for salt concentration and strand asymmetry. The resulting melting temperature, 62.5816 degrees Celsius, represents the point where half of the duplexes are dissociated under conditions defined by inputs CGTTCCAAAGATGTGGGCATGAGCTTAC, 50 (mmol/L), 250 (nmol/L), and 250 (nmol/L).

<a id="cmb_nucleic_acid_a260"></a>

#### Nucleic acid concentration from A260: `cmb_nucleic_acid_a260`

The concentration of dsDNA, ssDNA or RNA from its absorbance at 260 nm (50, 33 and 40 ug/mL per unit at 1 cm), with the dilution and path length, and the mass in a volume.

$$
c = A_{260}\times F \times DF / \ell
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A260` | in | absorbance at 260 nm (blank-corrected) | - |
| `molecule` | in | dsDNA, ssDNA or RNA | - |
| `dilution` | in | dilution factor of the measured sample (default 1) (optional) | - |
| `l` | in | path length (default 1 cm) (optional) | m |
| `V` | in | volume of the stock (optional, for the total mass) (optional) | m^3 |
| `c` | out | concentration of the stock | kg/m^3 |
| `c_ug_mL` | out | concentration in ug/mL (= ng/uL) | - |
| `mass_ug` | out | micrograms in V (when V is given) | - |

**Applies when:** A260 >= 0 (best within 0.1-1.0); dilution >= 1; l > 0.

```shell
$ cell-molecular-biology solve cmb_nucleic_acid_a260 A260=0.5 molecule=dsDNA dilution=10
c = 0.25 kg/m^3
c_ug_mL = 250
  by: Nucleic acid concentration from A260  (c (ug/mL) = A260 x factor x dilution / l(cm); factor 50 (dsDNA), 33 (ssDNA), 40 (RNA))
  A260 = 0.5
  dilution = 10
  molecule = dsDNA
  assumes: A260 >= 0 (best within 0.1-1.0); dilution >= 1; l > 0
```

**What the worked example means:** The stock concentration is 0.25 kg/m^3, which equals 250 in micrograms per milliliter.

<a id="cmb_library_size"></a>

#### Number of clones for a genomic library (Clarke-Carbon), `cmb_library_size`

The number of random clones needed so that any given sequence is present with probability P, for a genome of size G and inserts of size I.

$$
N = \frac{\ln(1 - P)}{\ln(1 - I/G)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `G` | in | genome size in bp | - |
| `I` | in | insert size in bp | - |
| `P` | in | desired probability (e.g. 99%) | - |
| `clones_exact` | out | number of clones (real value) | - |
| `clones` | out | number of clones rounded up | - |

**Applies when:** 0 < I < G; 0 < P < 1.

```shell
$ cell-molecular-biology solve cmb_library_size G=3000000000 I=40000 P=0.99
clones_exact = 345385.4614
clones = 345386
  by: Number of clones for a genomic library (Clarke-Carbon)  (N = ln(1 - P) / ln(1 - I/G))
  G = 3000000000
  I = 40000
  P = 0.99
  assumes: 0 < I < G; 0 < P < 1
```

**What the worked example means:** This model computes the number of clones needed for a genomic library using the Clarke-Carbon equation. It takes the genome size 3e+09, insert size 40000, and desired probability 0.99 as inputs. The result, 345385, represents the precise mathematical value, while 345386 provides the practical integer count by rounding up. This ensures that there is a high likelihood, specifically 0.99, that any specific DNA sequence from the genome is represented in the library.

<a id="cmb_tm_wallace"></a>

#### Primer melting temperature by the Wallace rule, `cmb_tm_wallace`

The approximate melting temperature (degC) of a short DNA primer (about 14-20 nt) by the Wallace rule, 2 degC per A/T and 4 degC per G/C.

$$
T_m = 2(n_A + n_T) + 4(n_G + n_C)\ ^\circ\mathrm{C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA primer sequence (A, C, G, T) | - |
| `Tm` | out | melting temperature in degC | - |
| `AT` | out | number of A and T | - |
| `GC` | out | number of G and C | - |

**Applies when:** DNA letters only; at least 2 bases (meant for 14-20 nt).

```shell
$ cell-molecular-biology solve cmb_tm_wallace seq=ACGTTGCAATGCCGTA
Tm = 48
AT = 8
GC = 8
  by: Primer melting temperature by the Wallace rule  (Tm = 2 (A + T) + 4 (G + C) degC)
  seq = ACGTTGCAATGCCGTA
  assumes: DNA letters only; at least 2 bases (meant for 14-20 nt)
```

**What the worked example means:** This calculation determines the approximate melting temperature of a short DNA primer using the Wallace rule. The method sums the contributions of each base type, assigning 2 degrees Celsius for every adenine or thymine and 4 degrees Celsius for every guanine or cytosine. For the input sequence, there are 8 instances of A or T and 8 instances of G or C, resulting in a final melting temperature of 48. This value represents the temperature at which half of the primer strands are expected to be dissociated from their complementary targets under standard conditions.

<a id="cmb_warburg_christian"></a>

#### Protein concentration by Warburg-Christian (A280 and A260), `cmb_warburg_christian`

The protein concentration (mg/mL) of a sample containing nucleic acid from its absorbances at 280 and 260 nm, protein = 1.55 A280 - 0.76 A260 (1 cm).

$$
c_{prot} = 1.55A_{280} - 0.76A_{260}\ \mathrm{mg/mL}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A280` | in | absorbance at 280 nm | - |
| `A260` | in | absorbance at 260 nm | - |
| `protein` | out | protein concentration | kg/m^3 |
| `protein_mg_mL` | out | protein in mg/mL | - |
| `ratio_280_260` | out | A280/A260 | - |

**Applies when:** absorbances >= 0, 1 cm path; a positive estimate.

```shell
$ cell-molecular-biology solve cmb_warburg_christian A280=0.8 A260=0.5
protein = 0.86 kg/m^3
protein_mg_mL = 0.86
ratio_280_260 = 1.6
  by: Protein concentration by Warburg-Christian (A280 and A260)  (protein (mg/mL) = 1.55 A280 - 0.76 A260)
  A280 = 0.8
  A260 = 0.5
  assumes: absorbances >= 0, 1 cm path; a positive estimate
```

**What the worked example means:** This calculation determines the protein concentration in mg/mL using the Warburg-Christian method. It takes the measured absorbance at 280 nm, which reflects both protein and nucleic acid content, and subtracts a correction factor based on the absorbance at 260 nm to account for nucleic acid interference. The result, 0.86, represents the estimated protein mass per volume after removing the contribution of nucleic acids. A higher ratio of 1.6 generally indicates purer protein samples, while lower ratios suggest significant nucleic acid contamination.

<a id="cmb_protein_concentration_a280"></a>

#### Protein concentration from A280 and the extinction coefficient, `cmb_protein_concentration_a280`

The molar (and, with the molar mass, mass) concentration of a protein from its absorbance at 280 nm and molar extinction coefficient (Beer-Lambert).

$$
c = \frac{A_{280}}{\varepsilon\,\ell}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A280` | in | absorbance at 280 nm | - |
| `eps` | in | molar extinction coefficient at 280 nm | m^2/mol |
| `MW` | in | molar mass of the protein (optional) (optional) | kg/mol |
| `l` | in | path length (default 1 cm) (optional) | m |
| `c` | out | protein concentration | mol/m^3 |
| `c_uM` | out | concentration in uM | - |
| `c_mg_mL` | out | concentration in mg/mL (when MW is given) | - |

**Applies when:** A280 >= 0; eps, l > 0.

```shell
$ cell-molecular-biology solve cmb_protein_concentration_a280 A280=0.75 "eps=43570 L/(mol*cm)" "MW=66430 g/mol"
c = 0.01721367914 mol/m^3
c_uM = 17.21367914
c_mg_mL = 1.143504705
  by: Protein concentration from A280 and the extinction coefficient  (c = A / (eps l); mass concentration = c x MW)
  A280 = 0.75
  eps = 4357 m^2/mol
  MW = 66.43 kg/mol
  assumes: A280 >= 0; eps, l > 0
```

**What the worked example means:** The molar concentration is 0.0172137 mol/m^3 and the mass concentration is 1.1435.

<a id="cmb_reverse_complement"></a>

#### Reverse complement of a DNA or RNA sequence, `cmb_reverse_complement`

The reverse complement (the other strand read 5'->3'), the complement and the reverse of a DNA or RNA sequence.

$$
\mathrm{rc}(s) = \overline{s_N}\cdots\overline{s_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA or RNA sequence 5'->3' | - |
| `reverse_complement` | out | reverse complement 5'->3' | - |
| `complement` | out | complement 3'->5' | - |
| `reverse` | out | the sequence reversed | - |

**Applies when:** only A, C, G, T or U.

```shell
$ cell-molecular-biology solve cmb_reverse_complement seq=ATGCGTACCTGA
reverse_complement = TCAGGTACGCAT
complement = TACGCATGGACT
reverse = AGTCCATGCGTA
  by: Reverse complement of a DNA or RNA sequence  (complement A-T (A-U), G-C, then reverse)
  seq = ATGCGTACCTGA
  assumes: only A, C, G, T or U
```

**What the worked example means:** This calculation determines the reverse complement of the input sequence ATGCGTACCTGA, which represents the complementary strand read in the five-prime to three-prime direction. The process involves first generating the complement of each nucleotide, resulting in TACGCATGGACT, and then reversing that string to produce TCAGGTACGCAT. This output is essential for understanding how DNA or RNA strands pair during replication and transcription.

<a id="cmb_tm_salt_adjusted"></a>

#### Salt-adjusted melting temperature of an oligonucleotide, `cmb_tm_salt_adjusted`

The melting temperature of a DNA oligonucleotide of 14 nt or more with a sodium correction: Tm = 81.5 + 16.6 log10[Na+] + 0.41 %GC - 600/N.

$$
T_m = 81.5 + 16.6\log_{10}[\mathrm{Na^+}] + 0.41\,\%GC - \frac{600}{N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA sequence (A, C, G, T), at least 14 nt | - |
| `Na` | in | monovalent cation (Na+) concentration | mol/m^3 |
| `Tm` | out | melting temperature in degC | - |
| `gc_percent` | out | GC content in per cent | - |
| `salt_term` | out | salt term 16.6 log10[Na+] in degC | - |

**Applies when:** DNA letters only; N >= 14; Na+ > 0.

```shell
$ cell-molecular-biology solve cmb_tm_salt_adjusted seq=AGCGGATAACAATTTCACACAGGA "Na=50 (mmol/L)"
Tm = 51.98623541
gc_percent = 41.66666667
salt_term = -21.59709793
  by: Salt-adjusted melting temperature of an oligonucleotide  (Tm = 81.5 + 16.6 log10([Na+]/M) + 0.41 %GC - 600/N)
  Na = 50 mol/m^3
  seq = AGCGGATAACAATTTCACACAGGA
  assumes: DNA letters only; N >= 14; Na+ > 0
```

**What the worked example means:** This calculation determines the thermal stability of a DNA oligonucleotide by estimating its melting temperature. The process begins by measuring the length and base composition of the sequence, specifically calculating the percentage of guanine and cytosine pairs as 41.6667. It then adjusts for ionic strength using the sodium concentration to produce a salt correction term of -21.5971. These values are combined with constant factors to yield the final melting temperature of 51.9862. This result indicates the temperature at which half of the DNA duplexes dissociate into single strands, serving as a critical parameter for designing primers and probes in molecular biology applications.

<a id="cmb_genome_coverage"></a>

#### Sequencing coverage and uncovered fraction (Lander-Waterman), `cmb_genome_coverage`

The mean sequencing depth C = L N / G, the expected fraction and number of bases not covered (e^-C) and the expected number of contigs (N e^-C).

$$
C = \frac{LN}{G},\quad P_0 = e^{-C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | read length in bp | - |
| `N` | in | number of reads | - |
| `G` | in | genome size in bp | - |
| `coverage` | out | mean coverage (depth) | - |
| `fraction_uncovered` | out | fraction of bases not covered e^-C | - |
| `bases_uncovered` | out | expected bases not covered | - |
| `contigs` | out | expected number of contigs N e^-C | - |

**Applies when:** L, N whole numbers >= 1; G > 0; reads placed uniformly at random.

```shell
$ cell-molecular-biology solve cmb_genome_coverage L=150 N=2000000 G=5000000
coverage = 60
fraction_uncovered = 8.756510763e-27
bases_uncovered = 4.378255381e-20
contigs = 1.751302153e-20
  by: Sequencing coverage and uncovered fraction (Lander-Waterman)  (C = L N / G; P(0 reads) = e^-C (Poisson))
  G = 5000000
  assumes: L, N whole numbers >= 1; G > 0; reads placed uniformly at random
```

**What the worked example means:** The model calculates the mean sequencing depth using the Lander-Waterman equation. Given a read length of 150 and a total number of reads of 2000000 mapped to a genome size of 5e+06, the resulting coverage is 60. This value determines the probability that any specific base is missed, which is calculated as e raised to the negative coverage, yielding an uncovered fraction of 8.75651e-27. Consequently, the expected number of bases not covered by any read is 4.37826e-20, and the expected number of contiguous sequences or contigs formed is 1.7513e-20.

<a id="cmb_transcribe"></a>

#### Transcription of a DNA strand into mRNA: `cmb_transcribe`

The mRNA (5'->3') transcribed from a DNA sequence given 5'->3' as either the coding (sense) strand or the template (antisense) strand.

$$
\mathrm{mRNA} = \mathrm{coding}(T\to U) = \mathrm{rc}(\mathrm{template})(T\to U)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | DNA sequence 5'->3' | - |
| `strand` | in | whether the given strand is the coding or the template strand | - |
| `mRNA` | out | mRNA sequence 5'->3' | - |
| `length` | out | length in nt | - |

**Applies when:** DNA letters only.

```shell
$ cell-molecular-biology solve cmb_transcribe seq=ATGGCCTTTAAG strand=coding
mRNA = AUGGCCUUUAAG
length = 12
  by: Transcription of a DNA strand into mRNA  (coding strand: T -> U; template strand: reverse complement, then T -> U)
  seq = ATGGCCTTTAAG
  strand = coding
  assumes: DNA letters only
```

In the worked example, seq = ATGGCCTTTAAG, strand = coding go through the equation above and give mRNA = AUGGCCUUUAAG, length = 12. It applies only when DNA letters only: outside that it says what is wrong instead of returning a number.

<a id="cmb_translate"></a>

#### Translation of an mRNA or DNA sequence into protein, `cmb_translate`

The protein encoded from the first base (frame 1) by the standard genetic code: up to the first stop codon, and the full translation with stops as *. A trailing incomplete codon is ignored.

$$
\text{codon} \mapsto \text{amino acid (table 1)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `seq` | in | mRNA or coding-strand DNA 5'->3' | - |
| `protein` | out | amino acids up to the first stop (one-letter code) | - |
| `full_translation` | out | full translation with * for stops | - |
| `length_aa` | out | length of the protein up to the first stop | - |
| `stops` | out | number of stop codons | - |

**Applies when:** only A, C, G, T or U; at least one codon.

```shell
$ cell-molecular-biology solve cmb_translate seq=AUGGCCAUUGUAAUGGGCCGCUGAAAGGGUGCCCGAUAG
protein = MAIVMGR
full_translation = MAIVMGR*KGAR*
length_aa = 7
stops = 2
  by: Translation of an mRNA or DNA sequence into protein  (read codons from base 1 with the standard code (NCBI table 1))
  seq = AUGGCCAUUGUAAUGGGCCGCUGAAAGGGUGCCCGAUAG
  assumes: only A, C, G, T or U; at least one codon
```

**What the worked example means:** This calculation determines the protein sequence encoded by the input nucleic acid string. It reads the sequence from the first base in triplets, mapping each codon to an amino acid using the standard genetic code. The primary output is the amino acid chain up to the first stop signal, which has a length of 7. The full translation including stops as asterisks is also provided, along with a count of 2 stop codons found in the entire frame.

### Enzyme kinetics (12)

<a id="cmb_competitive_inhibition"></a>

#### Competitive inhibition rate: `cmb_competitive_inhibition`

The rate of an enzyme with a competitive inhibitor (binding free enzyme only): Km rises by (1 + I/Ki), Vmax unchanged.

$$
v = \frac{V_{max}[S]}{K_m(1 + [I]/K_i) + [S]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vmax` | in | maximum rate without inhibitor | mol/(m^3*s) |
| `Km` | in | Michaelis constant | mol/m^3 |
| `S` | in | substrate concentration | mol/m^3 |
| `I` | in | inhibitor concentration | mol/m^3 |
| `Ki` | in | inhibition (dissociation) constant | mol/m^3 |
| `v` | out | rate with the inhibitor | mol/(m^3*s) |
| `Km_app` | out | apparent Km | mol/m^3 |
| `Vmax_app` | out | apparent Vmax | mol/(m^3*s) |
| `percent_inhibition` | out | per cent inhibition at this S | - |

**Applies when:** Vmax, Km, Ki > 0; S, I >= 0; rapid-equilibrium inhibitor binding.

```shell
$ cell-molecular-biology solve cmb_competitive_inhibition "Vmax=6 (umol/L)/min" "Km=50 (umol/L)" "S=100 (umol/L)" "I=20 (umol/L)" "Ki=10 (umol/L)"
v = 4e-05 mol/(m^3*s)
Km_app = 0.15 mol/m^3
Vmax_app = 0.0001 mol/(m^3*s)
percent_inhibition = 40
  by: Competitive inhibition rate  (v = Vmax S / (Km (1 + I/Ki) + S))
  Vmax = 0.0001 mol/(m^3*s)
  Km = 0.05 mol/m^3
  S = 0.1 mol/m^3
  I = 0.02 mol/m^3
  Ki = 0.01 mol/m^3
  assumes: Vmax, Km, Ki > 0; S, I >= 0; rapid-equilibrium inhibitor binding
```

**What the worked example means:** This calculation determines the enzyme reaction velocity under competitive inhibition using the provided substrate and inhibitor concentrations. The resulting rate is 4e-05 mol/(m^3*s), reflecting a shift in the apparent Michaelis constant to 0.15 mol/m^3 while the maximum velocity remains at 0.0001 mol/(m^3*s). This indicates that 40 of the potential activity is lost due to the inhibitor presence.

<a id="cmb_enzyme_assay_activity"></a>

#### Enzyme activity and specific activity from a spectrophotometric assay, `cmb_enzyme_assay_activity`

The enzyme activity (units = umol/min) in a cuvette from the absorbance change over a time (Beer-Lambert), the activity per mL of the enzyme sample and, with its protein concentration, the specific activity (U/mg).

$$
\text{U/mL} = \frac{(\Delta A/\Delta t)\,V_{assay}}{\varepsilon\,\ell\,V_{enz}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dA` | in | absorbance change over dt | - |
| `dt` | in | time of the absorbance change | s |
| `eps` | in | molar absorptivity of the measured product or substrate (NADH 6220 L/(mol cm)) | m^2/mol |
| `l` | in | path length | m |
| `V_assay` | in | total assay volume | m^3 |
| `V_enzyme` | in | volume of enzyme sample added | m^3 |
| `protein` | in | protein concentration of the enzyme sample (optional) (optional) | kg/m^3 |
| `rate` | out | rate of concentration change in the cuvette | mol/(m^3*s) |
| `units` | out | activity in the cuvette in umol/min (U) | - |
| `units_per_mL` | out | activity of the enzyme sample in U/mL | - |
| `specific_activity` | out | specific activity in U/mg (when protein is given) | - |

**Applies when:** dA != 0; dt, eps, l, volumes > 0; V_enzyme <= V_assay; linear initial rate.

```shell
$ cell-molecular-biology solve cmb_enzyme_assay_activity dA=0.12 "dt=1 min" "eps=6220 L/(mol*cm)" "l=1 cm" "V_assay=1 mL" "V_enzyme=20 uL" "protein=0.5 mg/mL"
rate = 0.0003215434084 mol/(m^3*s)
units = 0.0192926045
units_per_mL = 0.9646302251
specific_activity = 1.92926045
  by: Enzyme activity and specific activity from a spectrophotometric assay  (rate = |dA|/dt / (eps l); U = rate x V_assay (umol/min); U/mL = U / V_enzyme; U/mg = (U/mL) / (mg/mL))
  dA = 0.12
  dt = 60 s
  eps = 622 m^2/mol
  l = 0.01 m
  V_assay = 1e-06 m^3
  V_enzyme = 2e-08 m^3
  protein = 0.5 kg/m^3
  assumes: dA != 0; dt, eps, l, volumes > 0; V_enzyme <= V_assay; linear initial rate
```

**What the worked example means:** This calculation determines enzyme activity from a spectrophotometric assay using the Beer-Lambert law. The rate of concentration change in the cuvette was found to be 0.000321543 mol/(m^3*s), which translates to an absolute activity of 0.0192926. When normalized by the volume of enzyme added, this results in an activity concentration of 0.96463. If protein concentration data is available, the specific activity is calculated as 1.92926, providing a measure of purity and catalytic efficiency relative to the total protein mass present in the sample.

<a id="cmb_ic50_to_ki"></a>

#### Inhibition constant from IC50 (Cheng-Prusoff), `cmb_ic50_to_ki`

The inhibition constant Ki from a measured IC50 at substrate concentration S for a competitive, uncompetitive or noncompetitive inhibitor (Cheng-Prusoff relations).

$$
K_i = \frac{IC_{50}}{1 + [S]/K_m}\ \text{(competitive)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mode` | in | mechanism of inhibition | - |
| `IC50` | in | inhibitor concentration giving 50 % inhibition | mol/m^3 |
| `S` | in | substrate concentration of the assay | mol/m^3 |
| `Km` | in | Michaelis constant | mol/m^3 |
| `Ki` | out | inhibition constant | mol/m^3 |
| `Ki_uM` | out | Ki in uM | - |

**Applies when:** IC50, S, Km > 0; inhibitor not depleted (I >> E).

```shell
$ cell-molecular-biology solve cmb_ic50_to_ki mode=competitive "IC50=10 (umol/L)" "S=100 (umol/L)" "Km=50 (umol/L)"
Ki = 0.003333333333 mol/m^3
Ki_uM = 3.333333333
  by: Inhibition constant from IC50 (Cheng-Prusoff)  (competitive Ki = IC50/(1 + S/Km); uncompetitive Ki = IC50/(1 + Km/S); noncompetitive Ki = IC50)
  IC50 = 0.01 mol/m^3
  S = 0.1 mol/m^3
  Km = 0.05 mol/m^3
  mode = competitive
  assumes: IC50, S, Km > 0; inhibitor not depleted (I >> E)
```

**What the worked example means:** This calculation determines the true inhibition constant, denoted as Ki, which represents the concentration of inhibitor required to occupy half the enzyme sites at equilibrium. The value is derived from the observed half-maximal inhibitory concentration, IC50, using the Cheng-Prusoff equation appropriate for the specific mode of inhibition. For competitive inhibitors, the formula adjusts the IC50 by accounting for the substrate concentration and the Michaelis constant, yielding a Ki of 0.00333333 mol/m^3. This distinction is crucial because IC50 values depend on assay conditions such as substrate levels, whereas Ki is an intrinsic property of the inhibitor-enzyme interaction. The result provides a standardized metric for comparing inhibitor potency across different experimental setups.

<a id="cmb_michaelis_menten_nonlinear_fit"></a>

#### Km and Vmax by nonlinear least squares: `cmb_michaelis_menten_nonlinear_fit`

Km and Vmax fitted directly to initial-rate data by nonlinear least squares on v = Vmax S/(Km + S) (no linearizing transform), with the residual sum of squares and r^2.

$$
\min_{V,K}\sum_i\left(v_i - \frac{V S_i}{K + S_i}\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | substrate concentrations (any one unit) | - |
| `v` | in | initial rates (any one unit) | - |
| `Km` | out | Michaelis constant (unit of S) | - |
| `Vmax` | out | maximum rate (unit of v) | - |
| `rss` | out | residual sum of squares | - |
| `r2` | out | coefficient of determination | - |

**Applies when:** at least 3 points; S, v > 0; an interior least-squares minimum.

```shell
$ cell-molecular-biology solve cmb_michaelis_menten_nonlinear_fit "S=[1, 2, 5, 10, 20, 50]" "v=[16, 29.5, 49, 68, 79, 92]"
Km = 5.079276612
Vmax = 100.7136844
rss = 5.867526413
r2 = 0.9986415273
  by: Km and Vmax by nonlinear least squares  (Vmax profiled out (Vmax = sum v g / sum g^2, g = S/(Km + S)); the stationary point in Km found by bisection)
  assumes: at least 3 points; S, v > 0; an interior least-squares minimum
```

**What the worked example means:** This calculation determines the Michaelis constant and maximum reaction velocity by fitting the hyperbolic rate equation directly to your experimental data. The algorithm minimizes the squared differences between observed initial rates and predicted values, resulting in a Km of 5.07928 and a Vmax of 100.714. The goodness of fit is indicated by an r-squared value of 0.998642, where values closer to one suggest the model accurately describes the relationship between substrate concentration and reaction rate.

<a id="cmb_lineweaver_burk_fit"></a>

#### Km and Vmax from a Lineweaver-Burk plot: `cmb_lineweaver_burk_fit`

Km and Vmax from initial-rate data by the double-reciprocal (Lineweaver-Burk) least-squares line 1/v = (Km/Vmax)(1/S) + 1/Vmax.

$$
\frac{1}{v} = \frac{K_m}{V_{max}}\frac{1}{[S]} + \frac{1}{V_{max}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | substrate concentrations (any one unit, e.g. uM) | - |
| `v` | in | initial rates (any one unit) | - |
| `Km` | out | Michaelis constant (unit of S) | - |
| `Vmax` | out | maximum rate (unit of v) | - |
| `slope` | out | slope Km/Vmax | - |
| `intercept` | out | intercept 1/Vmax | - |
| `r2` | out | r^2 of the double-reciprocal line | - |

**Applies when:** at least 3 points; S, v > 0; positive slope and intercept.

```shell
$ cell-molecular-biology solve cmb_lineweaver_burk_fit "S=[1, 2, 5, 10, 20, 50]" "v=[16.7, 28.6, 50, 66.7, 80, 90.9]"
Km = 4.982268904
Vmax = 99.88884109
slope = 0.04987813304
intercept = 0.01001112826
r2 = 0.9999996643
  by: Km and Vmax from a Lineweaver-Burk plot  (least squares 1/v = a + b/S; Vmax = 1/a; Km = b/a)
  assumes: at least 3 points; S, v > 0; positive slope and intercept
```

**What the worked example means:** This calculation determines the kinetic parameters Km and Vmax by fitting initial rate data to a linear model. The reciprocal of the reaction velocity is plotted against the reciprocal of the substrate concentration, allowing for a least-squares regression. The resulting intercept provides the inverse of the maximum velocity, while the slope relates to the ratio of the Michaelis constant to the maximum velocity. For the dataset provided, the computed values are 4.98227 for the affinity constant and 99.8888 for the peak catalytic rate.

<a id="cmb_michaelis_menten_rate"></a>

#### Michaelis-Menten reaction rate: `cmb_michaelis_menten_rate`

The initial rate of an enzyme-catalysed reaction at substrate concentration S from Vmax and Km, and the fraction of Vmax.

$$
v = \frac{V_{max}[S]}{K_m + [S]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vmax` | in | maximum rate | mol/(m^3*s) |
| `Km` | in | Michaelis constant | mol/m^3 |
| `S` | in | substrate concentration | mol/m^3 |
| `v` | out | initial rate | mol/(m^3*s) |
| `v_uM_min` | out | rate in uM/min | - |
| `fraction` | out | v / Vmax | - |

**Applies when:** Vmax, Km > 0; S >= 0; initial-rate conditions, S >> E.

```shell
$ cell-molecular-biology solve cmb_michaelis_menten_rate "Vmax=100 (umol/L)/min" "Km=50 (umol/L)" "S=100 (umol/L)"
v = 0.001111111111 mol/(m^3*s)
v_uM_min = 66.66666667
fraction = 0.6666666667
  by: Michaelis-Menten reaction rate  (v = Vmax S / (Km + S))
  Vmax = 0.001666666667 mol/(m^3*s)
  Km = 0.05 mol/m^3
  S = 0.1 mol/m^3
  assumes: Vmax, Km > 0; S >= 0; initial-rate conditions, S >> E
```

**What the worked example means:** This calculation determines the initial velocity of an enzyme-catalyzed reaction using the Michaelis-Menten equation. Given a maximum velocity of 100 (umol/L)/min, a Michaelis constant of 50 (umol/L), and a substrate concentration of 100 (umol/L), the resulting initial rate is 0.00111111 mol/(m^3*s). The fraction of the maximum velocity achieved under these conditions is 0.666667, indicating how close the enzyme is to saturation.

<a id="cmb_mixed_inhibition"></a>

#### Mixed inhibition rate: `cmb_mixed_inhibition`

The rate, apparent Km and apparent Vmax of an enzyme with a mixed inhibitor that binds free enzyme (Ki) and the enzyme-substrate complex (Kii) with different affinities.

$$
v = \frac{V_{max}[S]}{K_m\left(1 + \frac{[I]}{K_i}\right) + [S]\left(1 + \frac{[I]}{K_i'}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vmax` | in | maximum rate without inhibitor | mol/(m^3*s) |
| `Km` | in | Michaelis constant | mol/m^3 |
| `S` | in | substrate concentration | mol/m^3 |
| `I` | in | inhibitor concentration | mol/m^3 |
| `Ki` | in | dissociation constant of EI | mol/m^3 |
| `Kii` | in | dissociation constant of ESI | mol/m^3 |
| `v` | out | rate with the inhibitor | mol/(m^3*s) |
| `Km_app` | out | apparent Km | mol/m^3 |
| `Vmax_app` | out | apparent Vmax | mol/(m^3*s) |
| `alpha` | out | Kii / Ki | - |

**Applies when:** Vmax, Km, Ki, Kii > 0; S, I >= 0; rapid equilibrium.

```shell
$ cell-molecular-biology solve cmb_mixed_inhibition "Vmax=6 (umol/L)/min" "Km=50 (umol/L)" "S=100 (umol/L)" "I=20 (umol/L)" "Ki=10 (umol/L)" "Kii=40 (umol/L)"
v = 3.333333333e-05 mol/(m^3*s)
Km_app = 0.1 mol/m^3
Vmax_app = 6.666666667e-05 mol/(m^3*s)
alpha = 4
  by: Mixed inhibition rate  (v = Vmax S / (Km (1 + I/Ki) + S (1 + I/Kii)); Km_app = Km (1 + I/Ki)/(1 + I/Kii); Vmax_app = Vmax/(1 + I/Kii))
  Vmax = 0.0001 mol/(m^3*s)
  Km = 0.05 mol/m^3
  S = 0.1 mol/m^3
  I = 0.02 mol/m^3
  Ki = 0.01 mol/m^3
  Kii = 0.04 mol/m^3
  assumes: Vmax, Km, Ki, Kii > 0; S, I >= 0; rapid equilibrium
```

**What the worked example means:** This calculation determines the enzyme kinetics under mixed inhibition. Using the maximum rate without inhibitor, Michaelis constant, substrate concentration, inhibitor concentration, and dissociation constants for the free enzyme and enzyme-substrate complex, this gives the current rate, apparent Km, and apparent Vmax. The resulting rate is 3.33333e-05 mol/(m^3*s), indicating how fast the product forms given the inhibitor's presence. The apparent Km is 0.1 mol/m^3, reflecting changes in substrate affinity, while the apparent Vmax is 6.66667e-05 mol/(m^3*s), showing the altered maximum velocity.

<a id="cmb_noncompetitive_inhibition"></a>

#### Noncompetitive inhibition rate: `cmb_noncompetitive_inhibition`

The rate of an enzyme with a pure noncompetitive inhibitor (equal affinity for E and ES): Vmax falls by (1 + I/Ki), Km unchanged.

$$
v = \frac{V_{max}[S]}{(K_m + [S])(1 + [I]/K_i)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vmax` | in | maximum rate without inhibitor | mol/(m^3*s) |
| `Km` | in | Michaelis constant | mol/m^3 |
| `S` | in | substrate concentration | mol/m^3 |
| `I` | in | inhibitor concentration | mol/m^3 |
| `Ki` | in | inhibition (dissociation) constant | mol/m^3 |
| `v` | out | rate with the inhibitor | mol/(m^3*s) |
| `Km_app` | out | apparent Km | mol/m^3 |
| `Vmax_app` | out | apparent Vmax | mol/(m^3*s) |
| `percent_inhibition` | out | per cent inhibition at this S | - |

**Applies when:** Vmax, Km, Ki > 0; S, I >= 0; rapid-equilibrium inhibitor binding.

```shell
$ cell-molecular-biology solve cmb_noncompetitive_inhibition "Vmax=6 (umol/L)/min" "Km=50 (umol/L)" "S=100 (umol/L)" "I=20 (umol/L)" "Ki=10 (umol/L)"
v = 2.222222222e-05 mol/(m^3*s)
Km_app = 0.05 mol/m^3
Vmax_app = 3.333333333e-05 mol/(m^3*s)
percent_inhibition = 66.66666667
  by: Noncompetitive inhibition rate  (v = Vmax S / ((Km + S)(1 + I/Ki)))
  Vmax = 0.0001 mol/(m^3*s)
  Km = 0.05 mol/m^3
  S = 0.1 mol/m^3
  I = 0.02 mol/m^3
  Ki = 0.01 mol/m^3
  assumes: Vmax, Km, Ki > 0; S, I >= 0; rapid-equilibrium inhibitor binding
```

In the worked example, Vmax = 6 (umol/L)/min, Km = 50 (umol/L), S = 100 (umol/L), I = 20 (umol/L), Ki = 10 (umol/L) go through the equation above and give v = 2.222222222e-05 mol/(m^3*s), Km_app = 0.05 mol/m^3, Vmax_app = 3.333333333e-05 mol/(m^3*s), percent_inhibition = 66.66666667. It applies only when Vmax, Km, Ki > 0; S, I >= 0; rapid-equilibrium inhibitor binding: outside that it says what is wrong instead of returning a number.

<a id="cmb_q10_temperature"></a>

#### Rate at another temperature from Q10: `cmb_q10_temperature`

A biological rate at temperature T2 from its value at T1 and the temperature coefficient Q10, and the equivalent Arrhenius activation energy over that interval.

$$
k_2 = k_1 Q_{10}^{(T_2 - T_1)/10},\quad E_a = \frac{R\ln(k_2/k_1)}{1/T_1 - 1/T_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k1` | in | rate at T1 (any unit) | - |
| `T1` | in | first temperature (degC is converted) | K |
| `T2` | in | second temperature (degC is converted) | K |
| `Q10` | in | temperature coefficient Q10 | - |
| `k2` | out | rate at T2 (unit of k1) | - |
| `ratio` | out | k2 / k1 | - |
| `Ea` | out | equivalent activation energy | J/mol |
| `Ea_kJ_mol` | out | activation energy in kJ/mol | - |

**Applies when:** k1, Q10 > 0; T1 != T2, both > 0 K.

```shell
$ cell-molecular-biology solve cmb_q10_temperature k1=10 "T1=298.15 K" "T2=308.15 K" Q10=2
k2 = 20
ratio = 2
Ea = 52948.86217 J/mol
Ea_kJ_mol = 52.94886217
  by: Rate at another temperature from Q10  (k2 = k1 Q10^((T2 - T1)/10); Ea = R ln(k2/k1) / (1/T1 - 1/T2))
  k1 = 10
  T1 = 298.15 K
  T2 = 308.15 K
  Q10 = 2
  assumes: k1, Q10 > 0; T1 != T2, both > 0 K
```

**What the worked example means:** This calculation determines the biological reaction rate at temperature 308.15 K based on its known value 10 at temperature 298.15 K and the temperature coefficient 2. The resulting rate is 20, which represents a change factor of 2 compared to the original rate. Additionally, the equivalent Arrhenius activation energy for this temperature interval is computed as 52.9489, providing insight into the thermal sensitivity of the process.

<a id="cmb_substrate_for_fraction_of_vmax"></a>

#### Substrate concentration for a fraction of Vmax, `cmb_substrate_for_fraction_of_vmax`

The substrate concentration at which a Michaelis-Menten enzyme runs at a fraction f of Vmax (e.g. 90 % needs 9 Km).

$$
[S] = K_m\frac{f}{1 - f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Km` | in | Michaelis constant | mol/m^3 |
| `f` | in | fraction of Vmax (0 to 1) | - |
| `S` | out | required substrate concentration | mol/m^3 |
| `S_uM` | out | S in uM | - |
| `S_over_Km` | out | S as a multiple of Km | - |

**Applies when:** Km > 0; 0 < f < 1.

```shell
$ cell-molecular-biology solve cmb_substrate_for_fraction_of_vmax "Km=50 (umol/L)" f=0.9
S = 0.45 mol/m^3
S_uM = 450
S_over_Km = 9
  by: Substrate concentration for a fraction of Vmax  (S = Km f / (1 - f))
  Km = 0.05 mol/m^3
  f = 0.9
  assumes: Km > 0; 0 < f < 1
```

**What the worked example means:** This calculation determines the substrate concentration needed for an enzyme to operate at a specific fraction of its maximum velocity. Using the input values of 50 (umol/L) and 0.9, the result shows that the necessary substrate level is 0.45 mol/m^3. Expressed in micromolar units, this concentration is 450. The outcome indicates that achieving this rate requires a substrate amount equal to 9 times the Michaelis constant, highlighting how sensitive the reaction rate is to substrate availability near saturation.

<a id="cmb_kcat_catalytic_efficiency"></a>

#### Turnover number kcat and catalytic efficiency kcat/Km, `cmb_kcat_catalytic_efficiency`

The turnover number kcat = Vmax/[E]t, the catalytic efficiency kcat/Km (compare with the diffusion limit of about 1e8-1e9 per M per s) and the time per turnover.

$$
k_{cat} = \frac{V_{max}}{[E]_t},\quad \eta = \frac{k_{cat}}{K_m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vmax` | in | maximum rate | mol/(m^3*s) |
| `Et` | in | total enzyme concentration | mol/m^3 |
| `Km` | in | Michaelis constant | mol/m^3 |
| `kcat` | out | turnover number | 1/s |
| `kcat_per_s` | out | kcat in 1/s | - |
| `efficiency` | out | catalytic efficiency kcat/Km | m^3/(mol*s) |
| `efficiency_per_M_s` | out | kcat/Km in 1/(M s) | - |
| `turnover_time` | out | time per catalytic event 1/kcat | s |

**Applies when:** Vmax, Et, Km > 0.

```shell
$ cell-molecular-biology solve cmb_kcat_catalytic_efficiency "Vmax=60 (umol/L)/min" "Et=10 (nmol/L)" "Km=25 (umol/L)"
kcat = 100 1/s
kcat_per_s = 100
efficiency = 4000 m^3/(mol*s)
efficiency_per_M_s = 4000000
turnover_time = 0.01 s
  by: Turnover number kcat and catalytic efficiency kcat/Km  (kcat = Vmax / [E]t; efficiency = kcat / Km)
  Vmax = 0.001 mol/(m^3*s)
  Et = 1e-05 mol/m^3
  Km = 0.025 mol/m^3
  assumes: Vmax, Et, Km > 0
```

**What the worked example means:** This calculation determines the enzyme's performance by first finding the turnover number, which represents how many substrate molecules each active site converts per second. The result for the turnover number is 100 1/s. Next, the catalytic efficiency is calculated by dividing this rate by the Michaelis constant to assess how effectively the enzyme works at low substrate concentrations. The computed efficiency value is 4000 m^3/(mol*s), indicating the enzyme's ability to compete with other reactions or approach diffusion limits.

<a id="cmb_uncompetitive_inhibition"></a>

#### Uncompetitive inhibition rate: `cmb_uncompetitive_inhibition`

The rate of an enzyme with an uncompetitive inhibitor (binding the enzyme-substrate complex only): Km and Vmax both fall by (1 + I/Ki).

$$
v = \frac{V_{max}[S]}{K_m + [S](1 + [I]/K_i)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Vmax` | in | maximum rate without inhibitor | mol/(m^3*s) |
| `Km` | in | Michaelis constant | mol/m^3 |
| `S` | in | substrate concentration | mol/m^3 |
| `I` | in | inhibitor concentration | mol/m^3 |
| `Ki` | in | inhibition (dissociation) constant | mol/m^3 |
| `v` | out | rate with the inhibitor | mol/(m^3*s) |
| `Km_app` | out | apparent Km | mol/m^3 |
| `Vmax_app` | out | apparent Vmax | mol/(m^3*s) |
| `percent_inhibition` | out | per cent inhibition at this S | - |

**Applies when:** Vmax, Km, Ki > 0; S, I >= 0; rapid-equilibrium inhibitor binding.

```shell
$ cell-molecular-biology solve cmb_uncompetitive_inhibition "Vmax=6 (umol/L)/min" "Km=50 (umol/L)" "S=100 (umol/L)" "I=20 (umol/L)" "Ki=10 (umol/L)"
v = 2.857142857e-05 mol/(m^3*s)
Km_app = 0.01666666667 mol/m^3
Vmax_app = 3.333333333e-05 mol/(m^3*s)
percent_inhibition = 57.14285714
  by: Uncompetitive inhibition rate  (v = Vmax S / (Km + S (1 + I/Ki)))
  Vmax = 0.0001 mol/(m^3*s)
  Km = 0.05 mol/m^3
  S = 0.1 mol/m^3
  I = 0.02 mol/m^3
  Ki = 0.01 mol/m^3
  assumes: Vmax, Km, Ki > 0; S, I >= 0; rapid-equilibrium inhibitor binding
```

In the worked example, Vmax = 6 (umol/L)/min, Km = 50 (umol/L), S = 100 (umol/L), I = 20 (umol/L), Ki = 10 (umol/L) go through the equation above and give v = 2.857142857e-05 mol/(m^3*s), Km_app = 0.01666666667 mol/m^3, Vmax_app = 3.333333333e-05 mol/(m^3*s), percent_inhibition = 57.14285714. It applies only when Vmax, Km, Ki > 0; S, I >= 0; rapid-equilibrium inhibitor binding: outside that it says what is wrong instead of returning a number.

### Enzyme kinetics and binding (10)

<a id="cmb_ligand_depletion_binding"></a>

#### Complex concentration with ligand depletion (quadratic binding), `cmb_ligand_depletion_binding`

The concentration of receptor-ligand complex when neither component is in excess (exact quadratic solution), with the fractions bound and the free ligand.

$$
[RL] = \frac{(R_t + L_t + K_d) - \sqrt{(R_t + L_t + K_d)^2 - 4R_tL_t}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rt` | in | total receptor concentration | mol/m^3 |
| `Lt` | in | total ligand concentration | mol/m^3 |
| `Kd` | in | dissociation constant | mol/m^3 |
| `RL` | out | complex concentration | mol/m^3 |
| `fraction_receptor_bound` | out | RL / Rt | - |
| `free_ligand` | out | free ligand Lt - RL | mol/m^3 |
| `fraction_ligand_bound` | out | RL / Lt | - |

**Applies when:** Rt, Lt, Kd > 0; single site, 1:1 binding at equilibrium.

```shell
$ cell-molecular-biology solve cmb_ligand_depletion_binding "Rt=10 (nmol/L)" "Lt=20 (nmol/L)" "Kd=10 (nmol/L)"
RL = 5.857864376e-06 mol/m^3
fraction_receptor_bound = 0.5857864376
free_ligand = 1.414213562e-05 mol/m^3
fraction_ligand_bound = 0.2928932188
  by: Complex concentration with ligand depletion (quadratic binding)  (RL = [(Rt + Lt + Kd) - sqrt((Rt + Lt + Kd)^2 - 4 Rt Lt)] / 2)
  Rt = 1e-05 mol/m^3
  Lt = 2e-05 mol/m^3
  Kd = 1e-05 mol/m^3
  assumes: Rt, Lt, Kd > 0; single site, 1:1 binding at equilibrium
```

**What the worked example means:** This calculation determines the equilibrium concentration of receptor-ligand complexes using the exact quadratic solution for single-site binding. Given total receptor 10 (nmol/L), total ligand 20 (nmol/L), and dissociation constant 10 (nmol/L), the resulting complex concentration is 5.85786e-06 mol/m^3. This corresponds to a fraction of receptors bound of 0.585786 and free ligand remaining at 1.41421e-05 mol/m^3.

<a id="cmb_kd_from_rates"></a>

#### Dissociation constant from association and dissociation rates, `cmb_kd_from_rates`

The equilibrium dissociation constant Kd = koff/kon, the association constant, the complex half-life and the residence time.

$$
K_d = \frac{k_{off}}{k_{on}},\quad t_{1/2} = \frac{\ln 2}{k_{off}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `kon` | in | association rate constant | m^3/(mol*s) |
| `koff` | in | dissociation rate constant | 1/s |
| `Kd` | out | dissociation constant | mol/m^3 |
| `Kd_nM` | out | Kd in nM | - |
| `Ka_per_M` | out | association constant in 1/M | - |
| `half_life` | out | complex half-life ln2/koff | s |
| `residence_time` | out | mean residence time 1/koff | s |

**Applies when:** kon, koff > 0; simple 1:1 binding.

```shell
$ cell-molecular-biology solve cmb_kd_from_rates "kon=1000000 1/((mol/L)*s)" "koff=0.001 1/s"
Kd = 1e-06 mol/m^3
Kd_nM = 1
Ka_per_M = 1000000000
half_life = 693.1471806 s
residence_time = 1000 s
  by: Dissociation constant from association and dissociation rates  (Kd = koff/kon; t1/2 = ln 2/koff; tau = 1/koff)
  kon = 1000 m^3/(mol*s)
  koff = 0.001 1/s
  assumes: kon, koff > 0; simple 1:1 binding
```

**What the worked example means:** This calculation determines the equilibrium dissociation constant Kd from the association rate kon and dissociation rate koff. The result, 1e-06 mol/m^3, represents the concentration at which half the binding sites are occupied. A lower value indicates tighter binding. Additionally, the complex half-life 693.147 s and mean residence time 1000 s describe how long the bound state persists.

<a id="cmb_fraction_bound"></a>

#### Fraction of receptor bound at a free ligand concentration, `cmb_fraction_bound`

The fractional occupancy L/(Kd + L) of a single-site receptor at free ligand concentration L, and the bound concentration for a receptor total Rt (ligand not depleted).

$$
\theta = \frac{[L]}{K_d + [L]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | free ligand concentration | mol/m^3 |
| `Kd` | in | dissociation constant | mol/m^3 |
| `Rt` | in | total receptor concentration (optional) (optional) | mol/m^3 |
| `theta` | out | fraction bound | - |
| `theta_percent` | out | per cent bound | - |
| `bound` | out | bound complex concentration (when Rt is given) | mol/m^3 |

**Applies when:** L, Kd > 0; single independent site; ligand in excess (free = total).

```shell
$ cell-molecular-biology solve cmb_fraction_bound "L=10 (nmol/L)" "Kd=5 (nmol/L)"
theta = 0.6666666667
theta_percent = 66.66666667
  by: Fraction of receptor bound at a free ligand concentration  (theta = L / (Kd + L))
  L = 1e-05 mol/m^3
  Kd = 5e-06 mol/m^3
  assumes: L, Kd > 0; single independent site; ligand in excess (free = total)
```

In the worked example, L = 10 (nmol/L), Kd = 5 (nmol/L) go through the equation above and give theta = 0.6666666667, theta_percent = 66.66666667. It applies only when L, Kd > 0; single independent site; ligand in excess (free = total): outside that it says what is wrong instead of returning a number.

<a id="cmb_hill_equation"></a>

#### Fractional saturation by the Hill equation, `cmb_hill_equation`

The fractional saturation (or response) of a cooperative binding protein at ligand concentration L, with half-saturation K and Hill coefficient n.

$$
\theta = \frac{[L]^n}{K_{0.5}^n + [L]^n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | free ligand concentration | mol/m^3 |
| `K` | in | ligand concentration for half saturation (K0.5) | mol/m^3 |
| `n` | in | Hill coefficient | - |
| `theta` | out | fractional saturation | - |
| `theta_percent` | out | saturation in per cent | - |

**Applies when:** L, K, n > 0.

```shell
$ cell-molecular-biology solve cmb_hill_equation "L=26 (umol/L)" "K=26 (umol/L)" n=2.8
theta = 0.5
theta_percent = 50
  by: Fractional saturation by the Hill equation  (theta = L^n / (K^n + L^n))
  L = 0.026 mol/m^3
  K = 0.026 mol/m^3
  n = 2.8
  assumes: L, K, n > 0
```

**What the worked example means:** This calculation determines the fractional saturation of a cooperative binding protein using the Hill equation. Given a free ligand concentration of 26 (umol/L), a half-saturation constant of 26 (umol/L), and a Hill coefficient of 2.8, the model computes the fraction of binding sites occupied by the ligand. The resulting fractional saturation is 0.5, indicating that 50 of the available sites are bound at this specific ligand concentration.

<a id="cmb_mwc_saturation"></a>

#### Fractional saturation in the MWC (concerted) model, `cmb_mwc_saturation`

The fractional saturation and the fraction of molecules in the R state of an allosteric protein with n sites in the Monod-Wyman-Changeux model (L0 = [T]/[R] without ligand, c = KR/KT).

$$
Y = \frac{\alpha(1+\alpha)^{n-1} + L_0 c\alpha(1+c\alpha)^{n-1}}{(1+\alpha)^n + L_0(1+c\alpha)^n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `S` | in | ligand concentration | mol/m^3 |
| `KR` | in | dissociation constant of a site in the R state | mol/m^3 |
| `c` | in | ratio KR/KT (0 < c < 1) | - |
| `L0` | in | allosteric constant [T0]/[R0] | - |
| `n` | in | number of binding sites | - |
| `Y` | out | fractional saturation | - |
| `R_fraction` | out | fraction of molecules in the R state | - |

**Applies when:** S, KR, c, L0 > 0; 1 <= n <= 12; concerted R/T transition.

```shell
$ cell-molecular-biology solve cmb_mwc_saturation "S=10 (umol/L)" "KR=2 (umol/L)" c=0.01 L0=9000 n=4
Y = 0.1308425393
R_fraction = 0.1059208076
  by: Fractional saturation in the MWC (concerted) model  (alpha = S/KR; Y = [alpha (1 + alpha)^(n-1) + L0 c alpha (1 + c alpha)^(n-1)] / [(1 + alpha)^n + L0 (1 + c alpha)^n])
  S = 0.01 mol/m^3
  KR = 0.002 mol/m^3
  c = 0.01
  L0 = 9000
  assumes: S, KR, c, L0 > 0; 1 <= n <= 12; concerted R/T transition
```

**What the worked example means:** This calculation determines the fractional saturation Y and the fraction of molecules in the R state for an allosteric protein using the Monod-Wyman-Changeux model. The result depends on the ligand concentration S, the dissociation constant KR, the ratio c, the allosteric constant L0, and the number of binding sites n. With the current inputs, the protein exhibits a fractional saturation of 0.130843, indicating that this proportion of available sites are occupied by the ligand. Simultaneously, the fraction of molecules in the high-affinity R state is 0.105921, reflecting how the ligand shifts the equilibrium between the tense and relaxed conformations.

<a id="cmb_hill_coefficient_from_ec"></a>

#### Hill coefficient from EC10 and EC90: `cmb_hill_coefficient_from_ec`

The Hill coefficient n = ln 81 / ln(EC90/EC10) and the EC50 (their geometric mean) of a Hill dose-response curve (81-fold range for n = 1).

$$
n_H = \frac{\ln 81}{\ln(EC_{90}/EC_{10})}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `EC10` | in | concentration giving 10 % of the maximum | mol/m^3 |
| `EC90` | in | concentration giving 90 % of the maximum | mol/m^3 |
| `n` | out | Hill coefficient | - |
| `EC50` | out | half-maximal concentration | mol/m^3 |
| `ratio` | out | EC90 / EC10 | - |

**Applies when:** 0 < EC10 < EC90; a Hill curve.

```shell
$ cell-molecular-biology solve cmb_hill_coefficient_from_ec "EC10=1 (umol/L)" "EC90=81 (umol/L)"
n = 1
EC50 = 0.009 mol/m^3
ratio = 81
  by: Hill coefficient from EC10 and EC90  (n = ln 81 / ln(EC90/EC10); EC50 = sqrt(EC10 EC90))
  EC10 = 0.001 mol/m^3
  EC90 = 0.081 mol/m^3
  assumes: 0 < EC10 < EC90; a Hill curve
```

**What the worked example means:** This calculation determines the steepness of a dose-response curve using the Hill equation. By taking the natural logarithm of eighty-one divided by the natural logarithm of the ratio between the concentration at ninety percent response (81 (umol/L)) and ten percent response (1 (umol/L)), this gives the Hill coefficient (1). The half-maximal effective concentration (0.009 mol/m^3) is calculated as the geometric mean of these two input values. A higher coefficient indicates a steeper transition between low and high response states, while the EC50 represents the potency of the stimulus.

<a id="cmb_scatchard_fit"></a>

#### Kd and Bmax from a Scatchard plot: `cmb_scatchard_fit`

The dissociation constant and the total binding sites from saturation-binding data by the Scatchard least-squares line B/F = Bmax/Kd - B/Kd.

$$
\frac{B}{F} = \frac{B_{max}}{K_d} - \frac{B}{K_d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | bound ligand concentrations (any one unit, e.g. nM or fmol/mg) | - |
| `F` | in | free ligand concentrations (e.g. nM) | - |
| `Kd` | out | dissociation constant (unit of F) | - |
| `Bmax` | out | total binding sites (unit of B) | - |
| `slope` | out | slope -1/Kd | - |
| `intercept` | out | intercept Bmax/Kd | - |
| `r2` | out | r^2 of the Scatchard line | - |

**Applies when:** at least 3 points; B, F > 0; negative slope (one class of sites).

```shell
$ cell-molecular-biology solve cmb_scatchard_fit "B=[16.7, 28.6, 50, 66.7, 80]" "F=[1, 2, 5, 10, 20]"
Kd = 4.986488927
Bmax = 99.93037658
slope = -0.2005419073
intercept = 20.04022831
r2 = 0.9999971119
  by: Kd and Bmax from a Scatchard plot  (least squares B/F = a + m B; Kd = -1/m; Bmax = -a/m)
  assumes: at least 3 points; B, F > 0; negative slope (one class of sites)
```

**What the worked example means:** This calculation determines the affinity and capacity of a receptor for its ligand using linear regression on Scatchard coordinates. By plotting the ratio of bound to free ligand against the bound concentration, this gives a straight line where the negative reciprocal of the slope yields the dissociation constant 4.98649, and the negative ratio of the intercept to the slope gives the maximum number of binding sites 99.9304. The resulting fit quality is indicated by an r-squared value of 0.999997, confirming how well the data adheres to the single-site binding model.

<a id="cmb_hill_ligand_for_fraction"></a>

#### Ligand concentration for a given saturation (Hill), `cmb_hill_ligand_for_fraction`

The ligand concentration that gives a fractional saturation theta for a cooperative protein with half-saturation K and Hill coefficient n.

$$
[L] = K_{0.5}\left(\frac{\theta}{1 - \theta}\right)^{1/n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | half-saturation concentration K0.5 | mol/m^3 |
| `n` | in | Hill coefficient | - |
| `theta` | in | fractional saturation (0 to 1) | - |
| `L` | out | required ligand concentration | mol/m^3 |
| `L_uM` | out | L in uM | - |

**Applies when:** K, n > 0; 0 < theta < 1.

```shell
$ cell-molecular-biology solve cmb_hill_ligand_for_fraction "K=26 (umol/L)" n=2.8 theta=0.9
L = 0.05698679654 mol/m^3
L_uM = 56.98679654
  by: Ligand concentration for a given saturation (Hill)  (L = K (theta/(1 - theta))^(1/n))
  K = 0.026 mol/m^3
  n = 2.8
  theta = 0.9
  assumes: K, n > 0; 0 < theta < 1
```

**What the worked example means:** This calculation determines the ligand concentration needed for a cooperative protein to reach a specific fractional saturation. Using the Hill equation, the result shows that with a half-saturation constant of 26 (umol/L) and a Hill coefficient of 2.8, a concentration of 0.0569868 mol/m^3 is required to bind enough ligand to satisfy the target saturation level of 0.9.

<a id="cmb_competitive_occupancy"></a>

#### Receptor occupancy with a competing ligand, `cmb_competitive_occupancy`

The fractions of a receptor occupied by a ligand and by a competitor binding the same site, the free fraction and the apparent Kd of the ligand in the competitor's presence (no depletion).

$$
\theta_L = \frac{[L]/K_d}{1 + [L]/K_d + [I]/K_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `L` | in | free ligand concentration | mol/m^3 |
| `Kd` | in | ligand dissociation constant | mol/m^3 |
| `I` | in | free competitor concentration | mol/m^3 |
| `Ki` | in | competitor dissociation constant | mol/m^3 |
| `occupancy_L` | out | fraction occupied by the ligand | - |
| `occupancy_I` | out | fraction occupied by the competitor | - |
| `free_fraction` | out | fraction of free receptor | - |
| `Kd_app` | out | apparent Kd of the ligand Kd (1 + I/Ki) | mol/m^3 |

**Applies when:** Kd, Ki > 0; L, I >= 0; one site, mutually exclusive binding, no depletion.

```shell
$ cell-molecular-biology solve cmb_competitive_occupancy "L=10 (nmol/L)" "Kd=5 (nmol/L)" "I=20 (nmol/L)" "Ki=10 (nmol/L)"
occupancy_L = 0.4
occupancy_I = 0.4
free_fraction = 0.2
Kd_app = 1.5e-05 mol/m^3
  by: Receptor occupancy with a competing ligand  (occupancy_L = (L/Kd) / (1 + L/Kd + I/Ki))
  L = 1e-05 mol/m^3
  Kd = 5e-06 mol/m^3
  I = 2e-05 mol/m^3
  Ki = 1e-05 mol/m^3
  assumes: Kd, Ki > 0; L, I >= 0; one site, mutually exclusive binding, no depletion
```

**What the worked example means:** This calculation determines how a single receptor site is distributed among three states: bound to the primary ligand, bound to a competing inhibitor, or unbound. Using the provided concentrations and dissociation constants, it computes that the fraction occupied by the ligand is 0.4 and the fraction occupied by the competitor is 0.4. The remaining portion, 0.2, represents receptors not currently bound by either molecule. Additionally, the apparent dissociation constant for the ligand in the presence of this competitor is calculated as 1.5e-05 mol/m^3, reflecting how the competitor reduces the effective affinity of the ligand for the receptor.

<a id="cmb_binding_free_energy"></a>

#### Standard binding free energy from Kd: `cmb_binding_free_energy`

The standard free energy of binding dG = RT ln(Kd / 1 M) (negative for favourable binding) at temperature T.

$$
\Delta G^\circ = RT\ln\frac{K_d}{1\ \mathrm{M}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Kd` | in | dissociation constant | mol/m^3 |
| `T` | in | absolute temperature (degC is converted) | K |
| `dG` | out | standard binding free energy | J/mol |
| `dG_kJ_mol` | out | dG in kJ/mol | - |
| `dG_kcal_mol` | out | dG in kcal/mol | - |

**Applies when:** Kd, T > 0; standard state 1 M.

```shell
$ cell-molecular-biology solve cmb_binding_free_energy "Kd=10 (nmol/L)" "T=298.15 K"
dG = -45664.07602 J/mol
dG_kJ_mol = -45.66407602
dG_kcal_mol = -10.9139761
  by: Standard binding free energy from Kd  (dG = R T ln(Kd/1 M) = -R T ln Ka)
  Kd = 1e-05 mol/m^3
  T = 298.15 K
  assumes: Kd, T > 0; standard state 1 M
```

**What the worked example means:** This calculation determines the standard Gibbs free energy change for a binding interaction using the relationship between the dissociation constant and thermodynamic stability. By inputting the measured dissociation constant 10 (nmol/L) and the absolute temperature 298.15 K, the model computes the energy required to separate the complex into its components under standard conditions of one molar concentration. The resulting value, expressed as -45664.1 J/mol in joules per mole or equivalently -45.6641 in kilojoules per mole, indicates the spontaneity of the association; a negative value signifies favorable binding where the complex is more stable than the separated parts.

### Membranes and diffusion (16)

<a id="cmb_cell_volume_osmotic"></a>

#### Cell volume after an osmotic change (Boyle-van 't Hoff), `cmb_cell_volume_osmotic`

The equilibrium volume of a cell moved from osmolarity osm0 to osm1, with an osmotically inactive volume fraction b (an ideal osmometer).

$$
V = V_b + (V_0 - V_b)\,\frac{\mathrm{osm}_0}{\mathrm{osm}_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V0` | in | initial cell volume | m^3 |
| `b` | in | osmotically inactive fraction of the initial volume (0 to 1) | - |
| `osm0` | in | initial (isotonic) osmolarity | mol/m^3 |
| `osm1` | in | new outside osmolarity | mol/m^3 |
| `V` | out | new equilibrium cell volume | m^3 |
| `V_fL` | out | new volume in femtolitres | - |
| `ratio` | out | new volume relative to the initial volume | - |

**Applies when:** V0, osm0, osm1 > 0; 0 <= b < 1; impermeant solutes, water in equilibrium.

```shell
$ cell-molecular-biology solve cmb_cell_volume_osmotic "V0=90 fL" b=0.4 "osm0=300 mmol/L" "osm1=600 mmol/L"
V = 6.3e-17 m^3
V_fL = 63
ratio = 0.7
  by: Cell volume after an osmotic change (Boyle-van 't Hoff)  (V = Vb + (V0 - Vb) osm0 / osm1 with Vb = b V0)
  V0 = 9e-17 m^3
  b = 0.4
  osm0 = 300 mol/m^3
  osm1 = 600 mol/m^3
  assumes: V0, osm0, osm1 > 0; 0 <= b < 1; impermeant solutes, water in equilibrium
```

**What the worked example means:** This calculation determines the final volume of a cell after it moves from an initial osmolarity to a new external osmolarity, assuming the cell acts as an ideal osmometer with impermeant solutes. Using the Boyle-van 't Hoff relationship, the result shows that the new equilibrium volume is 6.3e-17 m^3. This corresponds to 63 in femtolitres. The volume has changed by a factor of 0.7 compared to the starting size, reflecting how water shifts across the membrane to balance osmotic pressure while accounting for the osmotically inactive volume fraction.

<a id="cmb_frap_diffusion"></a>

#### Diffusion coefficient from a FRAP half-time (Soumpasis), `cmb_frap_diffusion`

The two-dimensional diffusion coefficient from the half-time of fluorescence recovery after photobleaching a uniform circular spot of radius w (Soumpasis' exact solution, D = 0.224 w^2 / t_half).

$$
f(t) = e^{-2\tau_D/t}\left[I_0\!\left(\tfrac{2\tau_D}{t}\right) + I_1\!\left(\tfrac{2\tau_D}{t}\right)\right],\quad D = \frac{w^2}{4\tau_D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `w` | in | radius of the bleached spot | m |
| `t_half` | in | half-time of recovery | s |
| `D` | out | diffusion coefficient | m^2/s |
| `D_um2_s` | out | D in um^2/s | - |
| `tauD` | out | characteristic diffusion time w^2/(4D) | s |
| `gamma` | out | tauD / t_half (about 1.12) | - |

**Applies when:** w, t_half > 0; 2-D free diffusion, uniform circular bleach, full mobile fraction.

```shell
$ cell-molecular-biology solve cmb_frap_diffusion "w=1 um" "t_half=0.5 s"
D = 4.472955755e-13 m^2/s
D_um2_s = 0.4472955755
tauD = 0.5589145382 s
gamma = 1.117829076
  by: Diffusion coefficient from a FRAP half-time (Soumpasis)  (f(t) = exp(-2 tauD/t)[I0(2 tauD/t) + I1(2 tauD/t)]; solve f(t_half) = 1/2 for tauD; D = w^2/(4 tauD))
  w = 1e-06 m
  t_half = 0.5 s
  assumes: w, t_half > 0; 2-D free diffusion, uniform circular bleach, full mobile fraction
```

**What the worked example means:** This calculation determines the two-dimensional diffusion coefficient using Soumpasis' exact solution for fluorescence recovery after photobleaching. By inputting the bleached spot radius 1 um and the measured half-time of recovery 0.5 s, the model solves for the characteristic diffusion time, yielding a diffusion coefficient of 4.47296e-13 m^2/s. This value quantifies how rapidly molecules move within the plane of the membrane or cytoplasm under the assumption of free, uniform diffusion.

<a id="cmb_diffusion_limited_rate"></a>

#### Diffusion-limited association rate constant (Smoluchowski), `cmb_diffusion_limited_rate`

The maximum (diffusion-limited) bimolecular association rate constant of two spheres that react on contact.

$$
k_D = 4\pi (D_A + D_B)(r_A + r_B) N_A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `DA` | in | diffusion coefficient of molecule A | m^2/s |
| `DB` | in | diffusion coefficient of molecule B | m^2/s |
| `rA` | in | radius of molecule A | m |
| `rB` | in | radius of molecule B | m |
| `k` | out | diffusion-limited rate constant | m^3/(mol*s) |
| `k_per_M_s` | out | rate constant in 1/(M s) | - |

**Applies when:** all inputs > 0; reaction on every contact, no interaction forces.

```shell
$ cell-molecular-biology solve cmb_diffusion_limited_rate "DA=100 um^2/s" "DB=1000 um^2/s" "rA=2 nm" "rB=0.5 nm"
k = 20811024.49 m^3/(mol*s)
k_per_M_s = 2.081102449e+10
  by: Diffusion-limited association rate constant (Smoluchowski)  (k = 4 pi (DA + DB)(rA + rB) N_A)
  DA = 1e-10 m^2/s
  DB = 1e-09 m^2/s
  rA = 2e-09 m
  rB = 5e-10 m
  assumes: all inputs > 0; reaction on every contact, no interaction forces
```

In the worked example, DA = 100 um^2/s, DB = 1000 um^2/s, rA = 2 nm, rB = 0.5 nm go through the equation above and give k = 20811024.49 m^3/(mol*s), k_per_M_s = 2.081102449e+10. It applies only when all inputs > 0; reaction on every contact, no interaction forces: outside that it says what is wrong instead of returning a number.

<a id="cmb_water_movement"></a>

#### Direction of water movement between a cell and its surroundings, `cmb_water_movement`

The water potentials of a cell and of the surrounding solution and the direction of net water movement (from higher to lower water potential).

$$
\Psi = -iCRT + \Psi_p,\quad \text{water moves toward lower } \Psi
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `i_cell` | in | ionization constant of the cell's solute | - |
| `C_cell` | in | solute concentration in the cell | mol/m^3 |
| `P_cell` | in | pressure potential of the cell (turgor) | Pa |
| `i_out` | in | ionization constant of the outside solute | - |
| `C_out` | in | solute concentration outside | mol/m^3 |
| `P_out` | in | pressure potential outside (0 for an open solution) | Pa |
| `T` | in | absolute temperature (degC is converted) | K |
| `psi_cell` | out | water potential of the cell | Pa |
| `psi_out` | out | water potential outside | Pa |
| `dpsi` | out | outside minus cell water potential | Pa |
| `dpsi_MPa` | out | outside minus cell water potential in MPa | - |
| `direction` | out | net water movement: into the cell, out of the cell or no net movement | - |

**Applies when:** i > 0, T > 0; concentrations >= 0; equal within 1e-9 = no net movement.

```shell
$ cell-molecular-biology solve cmb_water_movement i_cell=1 "C_cell=200 (mmol/L)" "P_cell=0.2 MPa" i_out=1 "C_out=100 (mmol/L)" "P_out=0 MPa" "T=298.15 K"
psi_cell = -295791.4059 Pa
psi_out = -247895.703 Pa
dpsi = 47895.70296 Pa
dpsi_MPa = 0.04789570296
direction = into the cell
  by: Direction of water movement between a cell and its surroundings  (psi = -i C R T + psi_p for each side; water moves from the higher to the lower water potential)
  i_cell = 1
  C_cell = 200 mol/m^3
  P_cell = 200000 Pa
  i_out = 1
  C_out = 100 mol/m^3
  P_out = 0 Pa
  T = 298.15 K
  assumes: i > 0, T > 0; concentrations >= 0; equal within 1e-9 = no net movement
```

In the worked example, i_cell = 1, C_cell = 200 (mmol/L), P_cell = 0.2 MPa, i_out = 1, C_out = 100 (mmol/L), P_out = 0 MPa, T = 298.15 K go through the equation above and give psi_cell = -295791.4059 Pa, psi_out = -247895.703 Pa, dpsi = 47895.70296 Pa, dpsi_MPa = 0.04789570296. It applies only when i > 0, T > 0; concentrations >= 0; equal within 1e-9 = no net movement: outside that it says what is wrong instead of returning a number.

<a id="cmb_donnan_equilibrium"></a>

#### Donnan equilibrium with an impermeant anion, `cmb_donnan_equilibrium`

The inside cation and anion concentrations, the Donnan ratio, the Donnan potential and the osmotic excess when a cell holds impermeant anions (charge -zA) bathed in a monovalent salt (KCl) of concentration c.

$$
[\mathrm{Cl}]_i = \frac{-z_A A + \sqrt{z_A^2A^2 + 4c^2}}{2},\quad E = \frac{RT}{F}\ln\frac{c}{[\mathrm{K}]_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `A` | in | concentration of impermeant anions inside | mol/m^3 |
| `zA` | in | magnitude of the impermeant anion's charge (1, 2 or 3) | - |
| `c` | in | outside concentration of the monovalent salt | mol/m^3 |
| `T` | in | absolute temperature (degC is converted) | K |
| `K_in` | out | inside cation concentration | mol/m^3 |
| `Cl_in` | out | inside permeant anion concentration | mol/m^3 |
| `ratio` | out | Donnan ratio K_in / K_out = Cl_out / Cl_in | - |
| `E` | out | Donnan potential (inside minus outside) | V |
| `E_mV` | out | Donnan potential in mV | - |
| `excess_mOsm` | out | inside minus outside osmolarity in mOsm/L | - |

**Applies when:** A, c, T > 0; zA = 1, 2 or 3; ideal solutions, electroneutral compartments.

```shell
$ cell-molecular-biology solve cmb_donnan_equilibrium "A=100 (mmol/L)" zA=1 "c=100 (mmol/L)" "T=298.15 K"
K_in = 161.8033989 mol/m^3
Cl_in = 61.80339887 mol/m^3
ratio = 1.618033989
E = -0.01236357289 V
E_mV = -12.36357289
excess_mOsm = 123.6067977
  by: Donnan equilibrium with an impermeant anion  (K_in Cl_in = c^2 and K_in = Cl_in + zA A: Cl_in = (-zA A + sqrt(zA^2 A^2 + 4c^2))/2; E = (RT/F) ln(c/K_in))
  A = 100 mol/m^3
  c = 100 mol/m^3
  T = 298.15 K
  assumes: A, c, T > 0; zA = 1, 2 or 3; ideal solutions, electroneutral compartments
```

**What the worked example means:** This calculation determines the equilibrium state of a cell containing impermeant anions bathed in a monovalent salt solution. Given the internal impermeant anion concentration of 100 (mmol/L), its charge magnitude of 1, the external salt concentration of 100 (mmol/L), and the temperature of 298.15 K, the model computes the resulting ion distributions. The inside potassium concentration reaches 161.803 mol/m^3, while the inside chloride concentration is 61.8034 mol/m^3. This imbalance creates a Donnan ratio of 1.61803 and generates a membrane potential of -12.3636 millivolts, accompanied by an osmotic excess of 123.607 milliosmoles per liter.

<a id="cmb_cell_equilibration"></a>

#### Equilibration of a solute across a spherical cell's membrane, `cmb_cell_equilibration`

The internal concentration of a permeant solute at time t for a spherical cell (A/V = 3/r) bathed at a constant outside concentration, with the time constant and half-time.

$$
C(t) = C_{out} + (C_0 - C_{out})\,e^{-3Pt/r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | cell radius | m |
| `P` | in | membrane permeability coefficient | m/s |
| `C_out` | in | constant outside concentration | mol/m^3 |
| `C0` | in | initial inside concentration | mol/m^3 |
| `t` | in | elapsed time | s |
| `C` | out | inside concentration at time t | mol/m^3 |
| `tau` | out | time constant r/(3P) | s |
| `t_half` | out | half-time of equilibration | s |

**Applies when:** r, P > 0; C_out, C0, t >= 0; well-mixed cell of constant volume, outside unchanged.

```shell
$ cell-molecular-biology solve cmb_cell_equilibration "r=10 um" "P=1e-05 cm/s" "C_out=10 (mmol/L)" "C0=0 (mmol/L)" "t=30 s"
C = 5.934303403 mol/m^3
tau = 33.33333333 s
t_half = 23.10490602 s
  by: Equilibration of a solute across a spherical cell's membrane  (dC/dt = P (A/V) (C_out - C), A/V = 3/r: C = C_out + (C0 - C_out) exp(-3Pt/r))
  r = 1e-05 m
  P = 1e-07 m/s
  C_out = 10 mol/m^3
  C0 = 0 mol/m^3
  t = 30 s
  assumes: r, P > 0; C_out, C0, t >= 0; well-mixed cell of constant volume, outside unchanged
```

**What the worked example means:** This calculation determines the solute concentration inside a spherical cell at a specific moment. It uses the cell radius, membrane permeability, constant outside concentration, initial inside concentration, and elapsed time. The result, 5.9343 mol/m^3, represents the current internal level as it approaches equilibrium with the surrounding medium.

<a id="cmb_fick_flux"></a>

#### Fick's first law: steady diffusion flux, `cmb_fick_flux`

The steady one-dimensional diffusive flux between two concentrations a distance apart (positive from side 1 to side 2).

$$
J = -D\frac{dC}{dx} = D\,\frac{C_1 - C_2}{\Delta x}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient of the solute | m^2/s |
| `C1` | in | concentration on side 1 | mol/m^3 |
| `C2` | in | concentration on side 2 | mol/m^3 |
| `dx` | in | distance between the two sides | m |
| `J` | out | molar flux from side 1 to side 2 | mol/(m^2*s) |
| `J_nmol_cm2_s` | out | flux in nmol per cm^2 per second | - |

**Applies when:** D > 0, dx > 0; C1, C2 >= 0; steady state, linear profile.

```shell
$ cell-molecular-biology solve cmb_fick_flux "D=1000 um^2/s" "C1=10 (mmol/L)" "C2=2 (mmol/L)" "dx=10 um"
J = 0.0008 mol/(m^2*s)
J_nmol_cm2_s = 80
  by: Fick's first law: steady diffusion flux  (J = D (C1 - C2) / dx (Fick's first law with a linear steady profile))
  D = 1e-09 m^2/s
  C1 = 10 mol/m^3
  C2 = 2 mol/m^3
  dx = 1e-05 m
  assumes: D > 0, dx > 0; C1, C2 >= 0; steady state, linear profile
```

**What the worked example means:** This calculation determines the molar flux of a solute moving through a medium based on Fick's first law. Using a diffusion coefficient of 1000 um^2/s and a concentration difference between side one (10 (mmol/L)) and side two (2 (mmol/L)) across a distance of 10 um, the resulting flux from side one to side two is 0.0008 mol/(m^2*s). This value represents the rate at which particles move per unit area due to the concentration gradient.

<a id="cmb_channel_current"></a>

#### Macroscopic current through ion channels: `cmb_channel_current`

The whole-cell current I = N Po gamma (V - E) through N channels of single-channel conductance gamma and open probability Po, and the single-channel current.

$$
I = N P_o \gamma\,(V - E_{rev})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N` | in | number of channels | - |
| `Po` | in | open probability (0 to 1) | - |
| `gamma` | in | single-channel conductance | S |
| `V` | in | membrane potential | V |
| `E` | in | reversal potential of the channel | V |
| `I` | out | macroscopic current (positive outward) | A |
| `I_pA` | out | current in pA | - |
| `G` | out | total open conductance N Po gamma | S |
| `i_single` | out | single open-channel current gamma (V - E) | A |

**Applies when:** N >= 1; 0 <= Po <= 1; gamma > 0; ohmic open channels.

```shell
$ cell-molecular-biology solve cmb_channel_current N=1000 Po=0.3 "gamma=20 pS" "V=-40 mV" "E=-90 mV"
I = 3e-10 A
I_pA = 300
G = 6e-09 S
i_single = 1e-12 A
  by: Macroscopic current through ion channels  (I = N Po gamma (V - E))
  Po = 0.3
  gamma = 2e-11 S
  V = -0.04 V
  E = -0.09 V
  assumes: N >= 1; 0 <= Po <= 1; gamma > 0; ohmic open channels
```

**What the worked example means:** This calculation determines the total ionic current flowing through a population of ion channels. It multiplies the number of functional channels by their open probability and single-channel conductance to find the total conductance, then applies the electrochemical driving force defined by the difference between the membrane potential and the reversal potential. The result, 3e-10 A, represents the net flow of charge across the membrane in amperes, while 6e-09 S shows the aggregate ease with which ions can pass through the open channels.

<a id="cmb_membrane_permeability"></a>

#### Membrane permeability coefficient and flux, `cmb_membrane_permeability`

The permeability coefficient P = K D / L of a lipid membrane for a solute with partition coefficient K, and the inward flux P (C_out - C_in).

$$
P = \frac{K D}{L},\quad J = P\,(C_{out} - C_{in})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient of the solute inside the membrane | m^2/s |
| `K` | in | membrane/water partition coefficient | - |
| `L` | in | membrane thickness | m |
| `C_out` | in | concentration outside the cell | mol/m^3 |
| `C_in` | in | concentration inside the cell | mol/m^3 |
| `P` | out | permeability coefficient | m/s |
| `P_cm_s` | out | permeability coefficient in cm/s | - |
| `J` | out | inward flux (positive into the cell) | mol/(m^2*s) |

**Applies when:** D, K, L > 0; concentrations >= 0; steady state, linear profile in the membrane.

```shell
$ cell-molecular-biology solve cmb_membrane_permeability "D=100 um^2/s" K=0.01 "L=5 nm" "C_out=10 (mmol/L)" "C_in=1 (mmol/L)"
P = 0.0002 m/s
P_cm_s = 0.02
J = 0.0018 mol/(m^2*s)
  by: Membrane permeability coefficient and flux  (P = K D / L; J = P (C_out - C_in))
  D = 1e-10 m^2/s
  K = 0.01
  L = 5e-09 m
  C_out = 10 mol/m^3
  C_in = 1 mol/m^3
  assumes: D, K, L > 0; concentrations >= 0; steady state, linear profile in the membrane
```

**What the worked example means:** This calculation determines the rate at which a solute crosses a lipid bilayer. The permeability coefficient, reported as 0.0002 m/s, combines the solute's diffusion speed within the membrane with its tendency to partition into the lipid phase relative to water, divided by the barrier thickness. Using this coefficient and the difference between external and internal concentrations, the model computes the net inward flux, given here as 0.0018 mol/(m^2*s). A positive value indicates movement into the cell, while a negative value signifies efflux.

<a id="cmb_nernst_potential"></a>

#### Nernst equilibrium potential of an ion across a cell membrane, `cmb_nernst_potential`

The equilibrium (reversal) potential inside relative to outside for an ion of charge z from its outside and inside concentrations.

$$
E = \frac{RT}{zF}\ln\frac{C_{out}}{C_{in}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `z` | in | charge number of the ion (e.g. 1 for K+, 2 for Ca2+, -1 for Cl-) | - |
| `C_out` | in | outside concentration | mol/m^3 |
| `C_in` | in | inside concentration | mol/m^3 |
| `T` | in | absolute temperature (degC is converted) | K |
| `E` | out | equilibrium potential (inside minus outside) | V |
| `E_mV` | out | equilibrium potential in mV | - |

**Applies when:** z a non-zero whole number; concentrations > 0; T > 0.

```shell
$ cell-molecular-biology solve cmb_nernst_potential z=1 "C_out=5 (mmol/L)" "C_in=140 (mmol/L)" "T=310.15 K"
E = -0.08905869404 V
E_mV = -89.05869404
  by: Nernst equilibrium potential of an ion across a cell membrane  (E = (R T / z F) ln(C_out / C_in))
  C_out = 5 mol/m^3
  C_in = 140 mol/m^3
  T = 310.15 K
  assumes: z a non-zero whole number; concentrations > 0; T > 0
```

**What the worked example means:** This calculation determines the electrical potential difference across a cell membrane that exactly balances the concentration gradient of a specific ion. Using the Nernst equation, the result depends on the ion's charge, the absolute temperature, and the ratio of external to internal concentrations. For the given inputs, the computed equilibrium potential is -0.0890587 V volts, which corresponds to -89.0587 millivolts. This value represents the voltage at which there is no net movement of the ion across the membrane.

<a id="cmb_osmolarity_tonicity"></a>

#### Osmolarity of a solution and its tonicity for a cell, `cmb_osmolarity_tonicity`

The total osmolarity of a solution of several solutes (sum of i c) and whether it is hypertonic, hypotonic or isotonic (within 1 %) relative to the cell's osmolarity, assuming impermeant solutes.

$$
\mathrm{osm} = \sum_k i_k c_k
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | solute concentrations in mM (one per solute) | - |
| `i` | in | van 't Hoff factor of each solute | - |
| `osm_cell` | in | osmolarity of the cell interior | mol/m^3 |
| `osmolarity_mOsm` | out | osmolarity of the solution in mOsm/L | - |
| `difference_mOsm` | out | solution minus cell osmolarity in mOsm/L | - |
| `tonicity` | out | hypertonic, hypotonic or isotonic (within 1 %) | - |

**Applies when:** one factor per concentration; solutes impermeant (tonicity = osmolarity).

```shell
$ cell-molecular-biology solve cmb_osmolarity_tonicity "c=[150]" "i=[2]" "osm_cell=300 mmol/L"
osmolarity_mOsm = 300
difference_mOsm = 0
tonicity = isotonic
  by: Osmolarity of a solution and its tonicity for a cell  (osmolarity = sum i_k c_k; compared with the cell's osmolarity (isotonic within 1 %))
  osm_cell = 300 mol/m^3
  assumes: one factor per concentration; solutes impermeant (tonicity = osmolarity)
```

**What the worked example means:** The total osmolarity is the sum of each solute's concentration times its van 't Hoff factor. The result, 300, represents the total particle concentration in milliosmoles per liter. By comparing this value to the internal osmolarity of the cell, this gives the difference, reported as 0. Based on whether this difference falls within one percent of the cell's value, the solution is classified as isotonic, indicating how water will move across the membrane.

<a id="cmb_osmotic_pressure"></a>

#### Osmotic pressure (van 't Hoff): `cmb_osmotic_pressure`

The osmotic pressure of a dilute solution from the solute concentration, the van 't Hoff factor and the temperature.

$$
\Pi = i\,c\,R\,T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `i` | in | van 't Hoff factor (particles per formula unit, e.g. 2 for NaCl) | - |
| `c` | in | solute concentration | mol/m^3 |
| `T` | in | absolute temperature (degC is converted) | K |
| `pi` | out | osmotic pressure | Pa |
| `pi_atm` | out | osmotic pressure in atm | - |
| `pi_kPa` | out | osmotic pressure in kPa | - |

**Applies when:** i, c, T > 0; ideal dilute solution.

```shell
$ cell-molecular-biology solve cmb_osmotic_pressure i=2 "c=150 (mmol/L)" "T=310.15 K"
pi = 773619.1743 Pa
pi_atm = 7.635027627
pi_kPa = 773.6191743
  by: Osmotic pressure (van 't Hoff)  (Pi = i c R T)
  i = 2
  c = 150 mol/m^3
  T = 310.15 K
  assumes: i, c, T > 0; ideal dilute solution
```

**What the worked example means:** This calculation determines the osmotic pressure of a dilute solution using the van 't Hoff equation. It combines the solute concentration, the absolute temperature, and the van 't Hoff factor to estimate the pressure required to prevent water from flowing into the solution through a semipermeable membrane. The resulting osmotic pressure is 773619 Pa, which corresponds to 773.619 in kilopascals.

<a id="cmb_rms_displacement"></a>

#### Root-mean-square displacement of a diffusing particle, `cmb_rms_displacement`

The mean-square and root-mean-square displacement of a particle diffusing freely for a time t in 1, 2 or 3 dimensions.

$$
\langle r^2\rangle = 2dDt,\quad r_{\mathrm{rms}} = \sqrt{2dDt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | diffusion coefficient | m^2/s |
| `t` | in | elapsed time | s |
| `dims` | in | number of dimensions (1, 2 or 3) | - |
| `rms` | out | root-mean-square displacement | m |
| `rms_um` | out | root-mean-square displacement in micrometres | - |
| `msd` | out | mean-square displacement | m^2 |

**Applies when:** D > 0, t > 0; dims = 1, 2 or 3.

```shell
$ cell-molecular-biology solve cmb_rms_displacement "D=1000 um^2/s" "t=1 s" dims=3
rms = 7.745966692e-05 m
rms_um = 77.45966692
msd = 6e-09 m^2
  by: Root-mean-square displacement of a diffusing particle  (<r^2> = 2 d D t; rms = sqrt(2 d D t))
  D = 1e-09 m^2/s
  t = 1 s
  assumes: D > 0, t > 0; dims = 1, 2 or 3
```

**What the worked example means:** This calculation determines the mean-square and root-mean-square displacement for a particle undergoing free diffusion. Using a diffusion coefficient of 1000 um^2/s and an elapsed time of 1 s, the model computes the result based on the dimensionality specified by 3. The final root-mean-square displacement is 7.74597e-05 m meters, providing a characteristic distance scale for the random walk in this context.

<a id="cmb_stokes_einstein"></a>

#### Stokes-Einstein diffusion coefficient: `cmb_stokes_einstein`

The diffusion coefficient of a sphere of hydrodynamic radius r in a fluid of viscosity eta at temperature T.

$$
D = \frac{k_B T}{6\pi\eta r}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | absolute temperature (degC is converted) | K |
| `eta` | in | viscosity of the medium | Pa*s |
| `r` | in | hydrodynamic radius | m |
| `D` | out | diffusion coefficient | m^2/s |
| `D_um2_s` | out | diffusion coefficient in um^2/s | - |
| `friction` | out | friction coefficient 6 pi eta r in kg/s | - |

**Applies when:** T, eta, r > 0; a sphere at low Reynolds number.

```shell
$ cell-molecular-biology solve cmb_stokes_einstein "T=298.15 K" "eta=0.89 mPa*s" "r=2.5 nm"
D = 9.814924382e-11 m^2/s
D_um2_s = 98.14924382
friction = 4.194026193e-11
  by: Stokes-Einstein diffusion coefficient  (D = k T / (6 pi eta r))
  T = 298.15 K
  eta = 0.00089 Pa*s
  r = 2.5e-09 m
  assumes: T, eta, r > 0; a sphere at low Reynolds number
```

**What the worked example means:** This calculation determines the diffusion coefficient for a spherical particle using the Stokes-Einstein equation. It relies on the absolute temperature, the viscosity of the surrounding medium, and the hydrodynamic radius of the particle. The resulting diffusion coefficient, reported as 9.81492e-11 m^2/s, indicates how rapidly the particle moves through the fluid due to thermal energy. Additionally, the friction coefficient is calculated as 4.19403e-11, representing the resistance encountered by the sphere during its motion.

<a id="cmb_diffusion_time"></a>

#### Time to diffuse a distance: `cmb_diffusion_time`

The time for the root-mean-square displacement of a diffusing molecule to reach a distance x in 1, 2 or 3 dimensions.

$$
t = \frac{x^2}{2dD}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | distance (root-mean-square displacement to reach) | m |
| `D` | in | diffusion coefficient | m^2/s |
| `dims` | in | number of dimensions (1, 2 or 3) | - |
| `t` | out | diffusion time | s |
| `t_min` | out | diffusion time in minutes | - |

**Applies when:** x > 0, D > 0; dims = 1, 2 or 3.

```shell
$ cell-molecular-biology solve cmb_diffusion_time "x=10 um" "D=1000 um^2/s" dims=3
t = 0.01666666667 s
t_min = 0.0002777777778
  by: Time to diffuse a distance  (t = x^2 / (2 d D))
  x = 1e-05 m
  D = 1e-09 m^2/s
  assumes: x > 0, D > 0; dims = 1, 2 or 3
```

**What the worked example means:** This calculation determines the time required for a molecule to achieve a root-mean-square displacement of 10 um through diffusion. Using the diffusion coefficient of 1000 um^2/s and considering 3 dimensions, the resulting time is 0.0166667 s. This duration represents how long it takes for random thermal motion to move the substance across that specific distance.

<a id="cmb_water_potential"></a>

#### Water potential of a plant cell or solution (solute plus pressure potential), `cmb_water_potential`

The solute potential psi_s = -i C R T and the water potential psi = psi_s + psi_p of a solution or cell (pure water at atmospheric pressure = 0).

$$
\Psi_s = -iCRT,\quad \Psi = \Psi_s + \Psi_p
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `i` | in | ionization (van 't Hoff) constant: 1 for sucrose, about 2 for NaCl | - |
| `C` | in | molar concentration of the solute | mol/m^3 |
| `T` | in | absolute temperature (degC is converted) | K |
| `psi_p` | in | pressure potential (turgor; 0 for an open solution) | Pa |
| `psi_s` | out | solute (osmotic) potential | Pa |
| `psi_s_MPa` | out | solute potential in MPa | - |
| `psi` | out | water potential | Pa |
| `psi_MPa` | out | water potential in MPa | - |

**Applies when:** i > 0, T > 0; C >= 0; ideal dilute solution.

```shell
$ cell-molecular-biology solve cmb_water_potential i=1 "C=150 (mmol/L)" "T=293.15 K" "psi_p=0 MPa"
psi_s = -365607.7075 Pa
psi_s_MPa = -0.3656077075
psi = -365607.7075 Pa
psi_MPa = -0.3656077075
  by: Water potential of a plant cell or solution (solute plus pressure potential)  (psi_s = -i C R T; psi = psi_s + psi_p)
  i = 1
  C = 150 mol/m^3
  T = 293.15 K
  psi_p = 0 Pa
  assumes: i > 0, T > 0; C >= 0; ideal dilute solution
```

In the worked example, i = 1, C = 150 (mmol/L), T = 293.15 K, psi_p = 0 MPa go through the equation above and give psi_s = -365607.7075 Pa, psi_s_MPa = -0.3656077075, psi = -365607.7075 Pa, psi_MPa = -0.3656077075. It applies only when i > 0, T > 0; C >= 0; ideal dilute solution: outside that it says what is wrong instead of returning a number.

### Metabolism (3)

<a id="cmb_atp_yield"></a>

#### ATP yield per glucose: aerobic respiration versus fermentation, `cmb_atp_yield`

The ATP made per glucose by aerobic respiration (substrate-level plus oxidative phosphorylation from NADH and FADH2 at stated P/O ratios and the shuttle for cytosolic NADH) and by fermentation, and optionally the energy efficiency of each.

$$
N_{ATP} = 4 + 8\,(P/O)_{NADH} + 2\,(P/O)_{FADH_2} + 2\,(P/O)_{shuttle},\quad \eta = \frac{N_{ATP}\,\Delta G_{ATP}}{\Delta G_{glucose}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P_O_NADH` | in | ATP made per mitochondrial NADH (P/O ratio, e.g. 2.5 or 3) | - |
| `P_O_FADH2` | in | ATP made per FADH2 (P/O ratio, e.g. 1.5 or 2) | - |
| `shuttle` | in | shuttle that carries the 2 cytosolic NADH into the mitochondrion | - |
| `dG_ATP` | in | free energy stored per mol of ATP, a positive magnitude (optional, e.g. 30.5 kJ/mol) (optional) | J/mol |
| `dG_glucose` | in | free energy released by complete oxidation of glucose, a positive magnitude (optional, e.g. 2870 kJ/mol) (optional) | J/mol |
| `ATP_aerobic` | out | ATP per glucose by aerobic respiration | - |
| `ATP_fermentation` | out | ATP per glucose by fermentation | - |
| `ratio` | out | aerobic yield / fermentation yield | - |
| `ATP_oxidative` | out | ATP from oxidative phosphorylation | - |
| `ATP_substrate_level` | out | ATP (and GTP) from substrate-level phosphorylation, net | - |
| `NADH_total` | out | NADH made per glucose | - |
| `FADH2_total` | out | FADH2 made per glucose | - |
| `efficiency_aerobic` | out | fraction of glucose's free energy stored in ATP, aerobic (when energies are given) | - |
| `efficiency_fermentation` | out | fraction of glucose's complete-oxidation free energy stored in ATP, fermentation (when energies are given) | - |

**Applies when:** 0 < P/O <= 3; both energies or neither; efficiency <= 1.

```shell
$ cell-molecular-biology solve cmb_atp_yield P_O_NADH=2.5 P_O_FADH2=1.5 shuttle=malate-aspartate "dG_ATP=30.5 kJ/mol" "dG_glucose=2870 kJ/mol"
ATP_aerobic = 32
ATP_fermentation = 2
ratio = 16
ATP_oxidative = 28
ATP_substrate_level = 4
NADH_total = 10
FADH2_total = 2
efficiency_aerobic = 0.3400696864
efficiency_fermentation = 0.0212543554
  by: ATP yield per glucose: aerobic respiration versus fermentation  (ATP = 4 (substrate level, net) + 8 P/O(NADH) + 2 P/O(FADH2) + 2 P/O(shuttle); fermentation = 2; efficiency = ATP dG_ATP / dG_glucose)
  P_O_NADH = 2.5
  P_O_FADH2 = 1.5
  dG_ATP = 30500 J/mol
  dG_glucose = 2870000 J/mol
  shuttle = malate-aspartate
  assumes: 0 < P/O <= 3; both energies or neither; efficiency <= 1
```

In the worked example, P_O_NADH = 2.5, P_O_FADH2 = 1.5, shuttle = malate-aspartate, dG_ATP = 30.5 kJ/mol, dG_glucose = 2870 kJ/mol go through the equation above and give ATP_aerobic = 32, ATP_fermentation = 2, ratio = 16, ATP_oxidative = 28. It applies only when 0 < P/O <= 3; both energies or neither; efficiency <= 1: outside that it says what is wrong instead of returning a number.

<a id="cmb_reaction_free_energy"></a>

#### Free-energy change of a reaction at cellular concentrations, `cmb_reaction_free_energy`

The actual free-energy change dG = dG0' + RT ln Q of a biochemical reaction at the concentrations in the cell (standard state 1 M), its equilibrium constant and the direction it proceeds.

$$
\Delta G = \Delta G^{\circ\prime} + RT\ln Q,\quad Q = \frac{\prod [P]}{\prod [R]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dG0` | in | standard free-energy change dG0' (biochemical standard state, pH 7) | J/mol |
| `T` | in | absolute temperature (degC is converted) | K |
| `products` | in | product concentrations in mM (list a species twice for a coefficient of 2; omit water and H+) | - |
| `reactants` | in | reactant concentrations in mM (list a species twice for a coefficient of 2; omit water and H+) | - |
| `Q` | out | reaction quotient (concentrations relative to 1 M) | - |
| `dG` | out | actual free-energy change | J/mol |
| `dG_kJ_mol` | out | actual free-energy change in kJ/mol | - |
| `K_eq` | out | equilibrium constant exp(-dG0'/RT) | - |
| `direction` | out | forward (dG < 0), reverse (dG > 0) or at equilibrium | - |

**Applies when:** T > 0; |dG0'| <= 500 kJ/mol; 0 < each concentration <= 10 M; ideal (activities = concentrations / 1 M).

```shell
$ cell-molecular-biology solve cmb_reaction_free_energy "dG0=-30.5 kJ/mol" "T=310.15 K" "products=[0.25, 1.65]" "reactants=[2.25]"
Q = 0.0001833333333
dG = -52687.92545 J/mol
dG_kJ_mol = -52.68792545
K_eq = 136971.0459
direction = forward
  by: Free-energy change of a reaction at cellular concentrations  (Q = prod(products / 1 M) / prod(reactants / 1 M); dG = dG0' + R T ln Q; K = exp(-dG0'/RT))
  dG0 = -30500 J/mol
  T = 310.15 K
  assumes: T > 0; |dG0'| <= 500 kJ/mol; 0 < each concentration <= 10 M; ideal (activities = concentrations / 1 M)
```

In the worked example, dG0 = -30.5 kJ/mol, T = 310.15 K, products = [0.25, 1.65], reactants = [2.25] go through the equation above and give Q = 0.0001833333333, dG = -52687.92545 J/mol, dG_kJ_mol = -52.68792545, K_eq = 136971.0459. It applies only when T > 0; |dG0'| <= 500 kJ/mol; 0 < each concentration <= 10 M; ideal (activities = concentrations / 1 M): outside that it says what is wrong instead of returning a number.

<a id="cmb_redox_free_energy"></a>

#### Free-energy change of a redox reaction from reduction potentials, `cmb_redox_free_energy`

The standard free-energy change dG0' = -n F dE0' of electron transfer from a donor couple to an acceptor couple, from their standard reduction potentials.

$$
\Delta G^{\circ\prime} = -nF\,\Delta E^{\circ\prime},\quad \Delta E^{\circ\prime} = E^{\circ\prime}_{acceptor} - E^{\circ\prime}_{donor}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E_acceptor` | in | standard reduction potential of the electron acceptor couple (E0') | V |
| `E_donor` | in | standard reduction potential of the electron donor couple (E0') | V |
| `n` | in | number of electrons transferred | - |
| `dE` | out | potential difference E0'(acceptor) - E0'(donor) | V |
| `dE_mV` | out | potential difference in mV | - |
| `dG` | out | standard free-energy change | J/mol |
| `dG_kJ_mol` | out | standard free-energy change in kJ/mol | - |
| `direction` | out | favourable as written (dG < 0), unfavourable or at equilibrium | - |

**Applies when:** n a whole number >= 1; |E0'| <= 3 V; standard (pH 7) conditions.

```shell
$ cell-molecular-biology solve cmb_redox_free_energy "E_acceptor=0.816 V" "E_donor=-0.32 V" n=2
dE = 1.136 V
dE_mV = 1136
dG = -219214.6746 J/mol
dG_kJ_mol = -219.2146746
direction = favourable as written
  by: Free-energy change of a redox reaction from reduction potentials  (dE0' = E0'(acceptor) - E0'(donor); dG0' = -n F dE0')
  E_acceptor = 0.816 V
  E_donor = -0.32 V
  assumes: n a whole number >= 1; |E0'| <= 3 V; standard (pH 7) conditions
```

In the worked example, E_acceptor = 0.816 V, E_donor = -0.32 V, n = 2 go through the equation above and give dE = 1.136 V, dE_mV = 1136, dG = -219214.6746 J/mol, dG_kJ_mol = -219.2146746. It applies only when n a whole number >= 1; |E0'| <= 3 V; standard (pH 7) conditions: outside that it says what is wrong instead of returning a number.

### Microbial control (3)

<a id="cmb_decimal_reduction_time"></a>

#### D-value (decimal reduction time) from survivor counts, `cmb_decimal_reduction_time`

The number of log10 reductions and the D-value (time for a 90 % kill) of a treatment from the viable counts before and after a stated time, assuming first-order (log-linear) death.

$$
D = \frac{t}{\log_{10}(N_0/N)},\quad k = \frac{\ln 10}{D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N0` | in | initial viable count (CFU/mL or CFU) | - |
| `N` | in | surviving viable count after the treatment (same units as N0) | - |
| `t` | in | treatment (exposure) time | s |
| `log_reduction` | out | number of log10 reductions log10(N0/N) | - |
| `D` | out | D-value (decimal reduction time) | s |
| `D_min` | out | D-value in minutes | - |
| `k_per_min` | out | first-order death rate constant in 1/min | - |
| `survival_fraction` | out | surviving fraction N/N0 | - |

**Applies when:** 0 < N < N0; t > 0; first-order (log-linear) killing.

```shell
$ cell-molecular-biology solve cmb_decimal_reduction_time N0=1000000 N=100 "t=10 min"
log_reduction = 4
D = 150 s
D_min = 2.5
k_per_min = 0.9210340372
survival_fraction = 0.0001
  by: D-value (decimal reduction time) from survivor counts  (log reductions = log10(N0/N); D = t / log10(N0/N); k = ln 10 / D)
  N0 = 1000000
  N = 100
  t = 600 s
  assumes: 0 < N < N0; t > 0; first-order (log-linear) killing
```

In the worked example, N0 = 1000000, N = 100, t = 10 min go through the equation above and give log_reduction = 4, D = 150 s, D_min = 2.5, k_per_min = 0.9210340372. It applies only when 0 < N < N0; t > 0; first-order (log-linear) killing: outside that it says what is wrong instead of returning a number.

<a id="cmb_thermal_death_time"></a>

#### D-value at a new temperature and time to a target kill (z-value), `cmb_thermal_death_time`

The D-value at temperature T from a reference D-value and the z-value, and the holding time needed for a stated number of log10 reductions at T.

$$
D_T = D_{ref}\,10^{(T_{ref}-T)/z},\quad t = n\,D_T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D_ref` | in | D-value at the reference temperature | s |
| `T_ref` | in | reference temperature (degC is converted) | K |
| `z` | in | z-value in degrees (a temperature difference: degC = K) | - |
| `T` | in | process temperature (degC is converted) | K |
| `log_reductions` | in | number of log10 reductions required (e.g. 12) | - |
| `D_T` | out | D-value at T | s |
| `D_T_min` | out | D-value at T in minutes | - |
| `t` | out | holding time for the required log reductions at T | s |
| `t_min` | out | holding time in minutes | - |
| `lethal_rate` | out | lethal rate 10^((T - T_ref)/z) relative to the reference temperature | - |

**Applies when:** D_ref, z, log reductions > 0; |T - T_ref| / z <= 50; log-linear thermal death.

```shell
$ cell-molecular-biology solve cmb_thermal_death_time "D_ref=0.2 min" "T_ref=394.25 K" z=10 "T=384.25 K" log_reductions=12
D_T = 120 s
D_T_min = 2
t = 1440 s
t_min = 24
lethal_rate = 0.1
  by: D-value at a new temperature and time to a target kill (z-value)  (D_T = D_ref x 10^((T_ref - T)/z); t = n D_T)
  D_ref = 12 s
  T_ref = 394.25 K
  z = 10
  T = 384.25 K
  log_reductions = 12
  assumes: D_ref, z, log reductions > 0; |T - T_ref| / z <= 50; log-linear thermal death
```

In the worked example, D_ref = 0.2 min, T_ref = 394.25 K, z = 10, T = 384.25 K, log_reductions = 12 go through the equation above and give D_T = 120 s, D_T_min = 2, t = 1440 s, t_min = 24. It applies only when D_ref, z, log reductions > 0; |T - T_ref| / z <= 50; log-linear thermal death: outside that it says what is wrong instead of returning a number.

<a id="cmb_z_value"></a>

#### z-value from D-values at two temperatures, `cmb_z_value`

The z-value (temperature rise that reduces the D-value ten-fold) from the D-values measured at two temperatures, and the fold change of D per 10 degrees.

$$
z = \frac{T_2 - T_1}{\log_{10}(D_1/D_2)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D1` | in | D-value at the first temperature | s |
| `T1` | in | first temperature (degC is converted) | K |
| `D2` | in | D-value at the second temperature | s |
| `T2` | in | second temperature (degC is converted) | K |
| `z` | out | z-value in kelvin (= degrees Celsius of difference) | - |
| `fold_per_10_degrees` | out | factor by which D falls for a 10-degree rise | - |

**Applies when:** D-values and temperatures > 0; T1 != T2; D falls as T rises (z > 0); log-linear thermal death curve.

```shell
$ cell-molecular-biology solve cmb_z_value "D1=10 min" "T1=383.15 K" "D2=1 min" "T2=393.15 K"
z = 10
fold_per_10_degrees = 10
  by: z-value from D-values at two temperatures  (z = (T2 - T1) / log10(D1 / D2))
  D1 = 600 s
  T1 = 383.15 K
  D2 = 60 s
  T2 = 393.15 K
  assumes: D-values and temperatures > 0; T1 != T2; D falls as T rises (z > 0); log-linear thermal death curve
```

In the worked example, D1 = 10 min, T1 = 383.15 K, D2 = 1 min, T2 = 393.15 K go through the equation above and give z = 10, fold_per_10_degrees = 10. It applies only when D-values and temperatures > 0; T1 != T2; D falls as T rises (z > 0); log-linear thermal death curve: outside that it says what is wrong instead of returning a number.

### Microscopy (2)

<a id="cmb_specimen_size"></a>

#### Actual size of a specimen from a micrograph, `cmb_specimen_size`

The actual size of an object from its size measured on an image (drawing or micrograph), by the magnification or by the image's scale bar; also the magnification of the image.

$$
\text{actual size} = \frac{\text{image size}}{M},\quad M = \frac{\text{bar on image}}{\text{bar label}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `image_size` | in | size of the object measured on the image | m |
| `M` | in | magnification of the image (give this or the scale bar) (optional) | - |
| `bar_image` | in | length of the scale bar measured on the image (optional, with bar_label) (optional) | m |
| `bar_label` | in | length the scale bar stands for (optional, with bar_image) (optional) | m |
| `size` | out | actual size of the object | m |
| `size_um` | out | actual size in micrometres | - |
| `M_image` | out | magnification of the image | - |

**Applies when:** all lengths > 0; either the magnification or both scale-bar lengths, not both.

```shell
$ cell-molecular-biology solve cmb_specimen_size "image_size=12 mm" M=400
size = 3e-05 m
size_um = 30
M_image = 400
  by: Actual size of a specimen from a micrograph  (actual size = image size / M; with a scale bar M = bar length on the image / length it represents)
  image_size = 0.012 m
  M = 400
  assumes: all lengths > 0; either the magnification or both scale-bar lengths, not both
```

In the worked example, image_size = 12 mm, M = 400 go through the equation above and give size = 3e-05 m, size_um = 30, M_image = 400. It applies only when all lengths > 0; either the magnification or both scale-bar lengths, not both: outside that it says what is wrong instead of returning a number.

<a id="cmb_microscope_resolution"></a>

#### Light microscope: total magnification and resolution limit, `cmb_microscope_resolution`

The total magnification of a compound light microscope (objective x eyepiece) and its resolution limit d = lambda / (2 NA) (Abbe).

$$
M = M_{obj}\,M_{oc},\quad d = \frac{\lambda}{2\,\mathrm{NA}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M_obj` | in | magnification of the objective lens (e.g. 40 for 40x) | - |
| `M_oc` | in | magnification of the eyepiece (ocular) lens (e.g. 10 for 10x) | - |
| `wavelength` | in | wavelength of the light | m |
| `NA` | in | numerical aperture of the objective | - |
| `M` | out | total magnification | - |
| `d` | out | resolution limit (smallest resolvable separation) | m |
| `d_nm` | out | resolution limit in nanometres | - |
| `d_um` | out | resolution limit in micrometres | - |

**Applies when:** magnifications > 0; wavelength > 0; 0 < NA <= 1.6.

```shell
$ cell-molecular-biology solve cmb_microscope_resolution M_obj=40 M_oc=10 "wavelength=550 nm" NA=0.65
M = 400
d = 4.230769231e-07 m
d_nm = 423.0769231
d_um = 0.4230769231
  by: Light microscope: total magnification and resolution limit  (M = M_objective x M_eyepiece; d = lambda / (2 NA) (Abbe))
  M_obj = 40
  M_oc = 10
  wavelength = 5.5e-07 m
  NA = 0.65
  assumes: magnifications > 0; wavelength > 0; 0 < NA <= 1.6
```

In the worked example, M_obj = 40, M_oc = 10, wavelength = 550 nm, NA = 0.65 go through the equation above and give M = 400, d = 4.230769231e-07 m, d_nm = 423.0769231, d_um = 0.4230769231. It applies only when magnifications > 0; wavelength > 0; 0 < NA <= 1.6: outside that it says what is wrong instead of returning a number.

### PCR and gene expression (11)

<a id="cmb_induction_dynamics"></a>

#### mRNA and protein after switching a gene on, `cmb_induction_dynamics`

The mRNA and protein numbers a time t after a gene is switched on from zero (constant transcription and translation, first-order decay), and the protein's fraction of its steady state.

$$
p(t) = \frac{k_{tl}k_{tx}}{\delta_m}\left[\frac{1 - e^{-\delta_p t}}{\delta_p} - \frac{e^{-\delta_m t} - e^{-\delta_p t}}{\delta_p - \delta_m}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k_tx` | in | transcription rate | Hz |
| `delta_m` | in | mRNA decay rate constant | 1/s |
| `k_tl` | in | translation rate per mRNA | Hz |
| `delta_p` | in | protein decay (plus dilution) rate constant | 1/s |
| `t` | in | time since induction | s |
| `m` | out | mRNA per cell at time t | - |
| `p` | out | protein per cell at time t | - |
| `fraction_of_steady_state` | out | p / p_ss | - |

**Applies when:** all rates > 0; t >= 0; m(0) = p(0) = 0.

```shell
$ cell-molecular-biology solve cmb_induction_dynamics "k_tx=36 1/h" "delta_m=7.2 1/h" "k_tl=180 1/h" "delta_p=0.36 1/h" "t=2 h"
m = 4.999997213
p = 1219.073084
fraction_of_steady_state = 0.4876292336
  by: mRNA and protein after switching a gene on  (m = (k_tx/delta_m)(1 - e^(-delta_m t)); p = (k_tl k_tx/delta_m)[(1 - e^(-delta_p t))/delta_p - (e^(-delta_m t) - e^(-delta_p t))/(delta_p - delta_m)] (t e^(-delta t) when equal))
  k_tx = 0.01 Hz
  delta_m = 0.002 1/s
  k_tl = 0.05 Hz
  delta_p = 0.0001 1/s
  t = 7200 s
  assumes: all rates > 0; t >= 0; m(0) = p(0) = 0
```

**What the worked example means:** This calculation determines the abundance of messenger RNA and protein in a single cell at a specific moment after gene induction. Starting from zero, it applies constant transcription and translation rates alongside first-order decay for both species. The result shows that the system contains 5 mRNA copies and 1219.07 protein copies, with the protein level representing 0.487629 of its eventual equilibrium value.

<a id="cmb_mrna_half_life_fit"></a>

#### mRNA half-life fitted to a decay time course, `cmb_mrna_half_life_fit`

The decay rate constant and half-life of an mRNA (or protein) from levels measured after transcription is stopped, by a least-squares line through ln(level) against time.

$$
\ln L = \ln L_0 - kt,\quad t_{1/2} = \frac{\ln 2}{k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | times after the shut-off in minutes | - |
| `level` | in | measured levels (relative units, positive) | - |
| `k_per_min` | out | decay rate constant per minute | - |
| `half_life_min` | out | half-life in minutes | - |
| `initial_fit` | out | fitted level at t = 0 | - |
| `r2` | out | coefficient of determination of the log-linear fit | - |

**Applies when:** at least 3 points; levels > 0 and decaying; first-order decay.

```shell
$ cell-molecular-biology solve cmb_mrna_half_life_fit "t=[0, 30, 60, 90, 120]" "level=[100, 70, 50.5, 35.2, 24.8]"
k_per_min = 0.01158700742
half_life_min = 59.82106989
initial_fit = 99.95560576
r2 = 0.9997993322
  by: mRNA half-life fitted to a decay time course  (least squares ln(level) = ln(L0) - k t; t1/2 = ln 2 / k)
  assumes: at least 3 points; levels > 0 and decaying; first-order decay
```

**What the worked example means:** This calculation determines the stability of mRNA by fitting a first-order decay model to measured levels over time. Using least squares regression on the natural logarithm of the level against time, the algorithm extracts the decay rate constant 0.011587. The resulting half-life, calculated as the natural log of two divided by this rate constant, is 59.8211 minutes. This value represents the time required for the mRNA population to reduce to fifty percent of its initial amount under the assumption of exponential decay.

<a id="cmb_pcr_amplification"></a>

#### PCR amplification after n cycles: `cmb_pcr_amplification`

The number of copies after n PCR cycles from N0 template copies at an amplification efficiency E (1 = perfect doubling).

$$
N = N_0(1 + E)^n
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N0` | in | initial template copies | - |
| `E` | in | amplification efficiency per cycle (0 to 1, e.g. 95%) | - |
| `n` | in | number of cycles | - |
| `N` | out | copies after n cycles | - |
| `fold` | out | fold amplification (1 + E)^n | - |
| `log10_fold` | out | log10 of the fold amplification | - |

**Applies when:** N0 > 0; 0 < E <= 1; 0 <= n <= 60; exponential phase (no plateau).

```shell
$ cell-molecular-biology solve cmb_pcr_amplification N0=100 E=1 n=30
N = 1.073741824e+11
fold = 1073741824
log10_fold = 9.03089987
  by: PCR amplification after n cycles  (N = N0 (1 + E)^n)
  N0 = 100
  E = 1
  assumes: N0 > 0; 0 < E <= 1; 0 <= n <= 60; exponential phase (no plateau)
```

In the worked example, N0 = 100, E = 1, n = 30 go through the equation above and give N = 1.073741824e+11, fold = 1073741824, log10_fold = 9.03089987. It applies only when N0 > 0; 0 < E <= 1; 0 <= n <= 60; exponential phase (no plateau): outside that it says what is wrong instead of returning a number.

<a id="cmb_pcr_cycles_needed"></a>

#### PCR cycles needed to reach a number of copies, `cmb_pcr_cycles_needed`

The number of PCR cycles needed to amplify N0 copies to N at efficiency E (exact value and whole cycles, rounded up).

$$
n = \frac{\ln(N/N_0)}{\ln(1 + E)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N0` | in | initial template copies | - |
| `N` | in | target number of copies | - |
| `E` | in | amplification efficiency per cycle (0 to 1) | - |
| `cycles_exact` | out | cycles needed (real value) | - |
| `cycles` | out | whole cycles needed | - |

**Applies when:** N > N0 > 0; 0 < E <= 1.

```shell
$ cell-molecular-biology solve cmb_pcr_cycles_needed N0=100 N=1e+10 E=0.95
cycles_exact = 27.58291489
cycles = 28
  by: PCR cycles needed to reach a number of copies  (n = ln(N/N0) / ln(1 + E))
  N0 = 100
  N = 1e+10
  E = 0.95
  assumes: N > N0 > 0; 0 < E <= 1
```

In the worked example, N0 = 100, N = 1e+10, E = 0.95 go through the equation above and give cycles_exact = 27.58291489, cycles = 28. It applies only when N > N0 > 0; 0 < E <= 1: outside that it says what is wrong instead of returning a number.

<a id="cmb_qpcr_delta_delta_ct"></a>

#### qPCR relative expression by the delta-delta Ct method, `cmb_qpcr_delta_delta_ct`

The fold change in a target gene's expression between treated and control samples, normalized to a reference gene, by 2^-ddCt (100 % efficiency assumed).

$$
\Delta\Delta C_t = \Delta C_{t,treated} - \Delta C_{t,control},\quad \text{fold} = 2^{-\Delta\Delta C_t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ct_target_treated` | in | Ct of the target gene in the treated sample | - |
| `Ct_ref_treated` | in | Ct of the reference gene in the treated sample | - |
| `Ct_target_control` | in | Ct of the target gene in the control sample | - |
| `Ct_ref_control` | in | Ct of the reference gene in the control sample | - |
| `dCt_treated` | out | Ct target - Ct reference, treated | - |
| `dCt_control` | out | Ct target - Ct reference, control | - |
| `ddCt` | out | dCt treated - dCt control | - |
| `fold_change` | out | relative expression 2^-ddCt | - |
| `log2_fold_change` | out | log2 fold change (-ddCt) | - |

**Applies when:** 0 < Ct <= 60; efficiencies of target and reference near 100 %.

```shell
$ cell-molecular-biology solve cmb_qpcr_delta_delta_ct Ct_target_treated=24.5 Ct_ref_treated=18 Ct_target_control=27 Ct_ref_control=18.2
dCt_treated = 6.5
dCt_control = 8.8
ddCt = -2.3
fold_change = 4.924577653
log2_fold_change = 2.3
  by: qPCR relative expression by the delta-delta Ct method  (ddCt = (Ct_t - Ct_r)_treated - (Ct_t - Ct_r)_control; fold = 2^-ddCt)
  Ct_target_treated = 24.5
  Ct_ref_treated = 18
  Ct_target_control = 27
  Ct_ref_control = 18.2
  assumes: 0 < Ct <= 60; efficiencies of target and reference near 100 %
```

In the worked example, Ct_target_treated = 24.5, Ct_ref_treated = 18, Ct_target_control = 27, Ct_ref_control = 18.2 go through the equation above and give dCt_treated = 6.5, dCt_control = 8.8, ddCt = -2.3, fold_change = 4.924577653. It applies only when 0 < Ct <= 60; efficiencies of target and reference near 100 %: outside that it says what is wrong instead of returning a number.

<a id="cmb_qpcr_pfaffl_ratio"></a>

#### qPCR relative expression with efficiency correction (Pfaffl), `cmb_qpcr_pfaffl_ratio`

The efficiency-corrected expression ratio of a target gene in a sample versus a control, normalized to a reference gene (Pfaffl 2001).

$$
\text{ratio} = \frac{(1 + E_t)^{\Delta C_{t,target}}}{(1 + E_r)^{\Delta C_{t,ref}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `E_target` | in | amplification efficiency of the target assay (0 to 1) | - |
| `E_ref` | in | amplification efficiency of the reference assay (0 to 1) | - |
| `Ct_target_control` | in | target Ct in the control | - |
| `Ct_target_sample` | in | target Ct in the sample | - |
| `Ct_ref_control` | in | reference Ct in the control | - |
| `Ct_ref_sample` | in | reference Ct in the sample | - |
| `ratio` | out | expression ratio sample/control | - |
| `log2_ratio` | out | log2 of the ratio | - |
| `dCt_target` | out | Ct control - Ct sample for the target | - |
| `dCt_ref` | out | Ct control - Ct sample for the reference | - |

**Applies when:** 0 < E <= 1; 0 < Ct <= 60.

```shell
$ cell-molecular-biology solve cmb_qpcr_pfaffl_ratio E_target=0.95 E_ref=0.9 Ct_target_control=26 Ct_target_sample=23.5 Ct_ref_control=18 Ct_ref_sample=18.3
ratio = 6.437431321
log2_ratio = 2.686485135
dCt_target = 2.5
dCt_ref = -0.3
  by: qPCR relative expression with efficiency correction (Pfaffl)  (ratio = (1 + E_t)^(Ct_t,control - Ct_t,sample) / (1 + E_r)^(Ct_r,control - Ct_r,sample))
  E_target = 0.95
  E_ref = 0.9
  Ct_target_control = 26
  Ct_target_sample = 23.5
  Ct_ref_control = 18
  Ct_ref_sample = 18.3
  assumes: 0 < E <= 1; 0 < Ct <= 60
```

**What the worked example means:** This calculation determines the relative expression of a target gene in an experimental sample compared to a control, normalized against a reference gene. It uses the Pfaffl method, which accounts for differences in amplification efficiency between the target and reference assays. The target Ct difference is 2.5 and the reference Ct difference is -0.3. Based on these values and the respective efficiencies, the final expression ratio is 6.43743, meaning the gene is expressed at this level relative to the control after normalization.

<a id="cmb_qpcr_standard_curve"></a>

#### qPCR standard curve: slope, intercept and efficiency, `cmb_qpcr_standard_curve`

The least-squares standard curve Ct = m log10(quantity) + b from a dilution series, its r^2 and the amplification efficiency 10^(-1/m) - 1.

$$
C_t = m\log_{10}q + b,\quad E = 10^{-1/m} - 1
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `quantity` | in | starting quantities of the standards (copies or ng) | - |
| `Ct` | in | measured Ct of each standard | - |
| `slope` | out | slope m (about -3.32 at 100 %) | - |
| `intercept` | out | intercept b (Ct of one unit) | - |
| `r2` | out | coefficient of determination | - |
| `efficiency` | out | efficiency 10^(-1/m) - 1 | - |
| `amplification_factor` | out | amplification per cycle 10^(-1/m) | - |

**Applies when:** at least 3 standards; quantities > 0; negative slope.

```shell
$ cell-molecular-biology solve cmb_qpcr_standard_curve "quantity=[1000000.0, 100000, 10000, 1000, 100]" "Ct=[15.1, 18.5, 21.8, 25.2, 28.5]"
slope = -3.35
intercept = 35.22
r2 = 0.9999732687
efficiency = 0.988416978
amplification_factor = 1.988416978
  by: qPCR standard curve: slope, intercept and efficiency  (least squares Ct = m log10 q + b; E = 10^(-1/m) - 1)
  assumes: at least 3 standards; quantities > 0; negative slope
```

In the worked example, quantity = [1e+06, 100000, 10000, 1000, 100], Ct = [15.1, 18.5, 21.8, 25.2, 28.5] go through the equation above and give slope = -3.35, intercept = 35.22, r2 = 0.9999732687, efficiency = 0.988416978. It applies only when at least 3 standards; quantities > 0; negative slope: outside that it says what is wrong instead of returning a number.

<a id="cmb_qpcr_quantify"></a>

#### Quantity from a Ct and a standard curve: `cmb_qpcr_quantify`

The starting quantity of an unknown from its Ct and the standard curve Ct = m log10 q + b.

$$
q = 10^{(C_t - b)/m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `slope` | in | standard-curve slope m (negative) | - |
| `intercept` | in | standard-curve intercept b | - |
| `Ct` | in | Ct of the unknown | - |
| `quantity` | out | starting quantity (unit of the standards) | - |
| `log10_quantity` | out | log10 of the quantity | - |

**Applies when:** slope < 0; Ct inside the range of the standards.

```shell
$ cell-molecular-biology solve cmb_qpcr_quantify slope=-3.32 intercept=38 Ct=24.7
quantity = 10139.67643
log10_quantity = 4.006024096
  by: Quantity from a Ct and a standard curve  (q = 10^((Ct - b)/m))
  slope = -3.32
  intercept = 38
  Ct = 24.7
  assumes: slope < 0; Ct inside the range of the standards
```

**What the worked example means:** This calculation determines the initial amount of target material in an unknown sample using its cycle threshold value and a standard curve. By substituting the observed Ct of 24.7 into the rearranged equation, the log base ten of the quantity is 4.00602. This corresponds to an absolute starting quantity of 10139.7, which is expressed in the same units as the calibration standards used to generate the curve.

<a id="cmb_expression_steady_state"></a>

#### Steady-state mRNA and protein numbers: `cmb_expression_steady_state`

The steady-state numbers of mRNA (k_tx/delta_m) and protein (k_tl m/delta_p) per cell for constant transcription, translation and first-order decay (dilution included), with the half-lives.

$$
m^* = \frac{k_{tx}}{\delta_m},\quad p^* = \frac{k_{tl}\,k_{tx}}{\delta_m\,\delta_p}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k_tx` | in | transcription rate (mRNA made per cell per unit time) | Hz |
| `delta_m` | in | mRNA decay rate constant | 1/s |
| `k_tl` | in | translation rate (proteins per mRNA per unit time) | Hz |
| `delta_p` | in | protein decay (plus dilution) rate constant | 1/s |
| `m_ss` | out | steady-state mRNA per cell | - |
| `p_ss` | out | steady-state protein per cell | - |
| `mRNA_half_life` | out | mRNA half-life | s |
| `protein_half_life` | out | protein half-life | s |
| `mRNA_half_life_min` | out | mRNA half-life in minutes | - |
| `protein_half_life_h` | out | protein half-life in hours | - |

**Applies when:** all rates > 0; constant rates, first-order decay.

```shell
$ cell-molecular-biology solve cmb_expression_steady_state "k_tx=36 1/h" "delta_m=7.2 1/h" "k_tl=180 1/h" "delta_p=0.36 1/h"
m_ss = 5
p_ss = 2500
mRNA_half_life = 346.5735903 s
protein_half_life = 6931.471806 s
mRNA_half_life_min = 5.776226505
protein_half_life_h = 1.925408835
  by: Steady-state mRNA and protein numbers  (dm/dt = k_tx - delta_m m, dp/dt = k_tl m - delta_p p; at steady state m = k_tx/delta_m, p = k_tl k_tx/(delta_m delta_p))
  k_tx = 0.01 Hz
  delta_m = 0.002 1/s
  k_tl = 0.05 Hz
  delta_p = 0.0001 1/s
  assumes: all rates > 0; constant rates, first-order decay
```

**What the worked example means:** This calculation determines the equilibrium number of mRNA molecules per cell, which is 5, and the corresponding number of protein molecules per cell, which is 2500. These values are derived from the transcription rate 36 1/h, the translation rate 180 1/h, and the decay constants 7.2 1/h for mRNA and 0.36 1/h for protein. The result indicates that under constant production and first-order degradation, the cellular concentrations stabilize when synthesis balances removal. The stability of these pools is characterized by their half-lives: 5.77623 minutes for the transcript and 1.92541 hours for the protein product.

<a id="cmb_transcription_time"></a>

#### Time to transcribe a gene: `cmb_transcription_time`

The time for RNA polymerase to transcribe a gene of n nucleotides at an elongation rate, and, with an initiation rate, the number of polymerases on the gene and their spacing.

$$
t = \frac{n}{v},\quad N_{pol} = k_{init}\,t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | gene (transcript) length in nucleotides | - |
| `rate` | in | elongation rate in nucleotides per second | - |
| `k_init` | in | initiation rate (optional) (optional) | Hz |
| `t` | out | transcription time | s |
| `t_min` | out | time in minutes | - |
| `polymerases` | out | polymerases on the gene at steady state (when k_init is given) | - |
| `spacing_nt` | out | mean spacing between polymerases in nt (when k_init is given) | - |

**Applies when:** n >= 1; rate > 0; constant elongation rate.

```shell
$ cell-molecular-biology solve cmb_transcription_time n=20000 rate=40
t = 500 s
t_min = 8.333333333
  by: Time to transcribe a gene  (t = n / rate; polymerases = k_init t; spacing = rate / k_init)
  rate = 40
  assumes: n >= 1; rate > 0; constant elongation rate
```

In the worked example, n = 20000, rate = 40 go through the equation above and give t = 500 s, t_min = 8.333333333. It applies only when n >= 1; rate > 0; constant elongation rate: outside that it says what is wrong instead of returning a number.

<a id="cmb_translation_time"></a>

#### Time to translate a protein: `cmb_translation_time`

The time for a ribosome to synthesise a protein of n_aa amino acids at an elongation rate, the coding length with the stop codon, and, with an initiation rate, the ribosomes per mRNA and their spacing.

$$
t = \frac{n_{aa}}{v},\quad N_{rib} = k_{init}\,t
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n_aa` | in | protein length in amino acids | - |
| `rate` | in | elongation rate in amino acids per second | - |
| `k_init` | in | initiation rate per mRNA (optional) (optional) | Hz |
| `t` | out | translation time | s |
| `t_min` | out | time in minutes | - |
| `coding_nt` | out | coding sequence length in nt including the stop codon | - |
| `ribosomes` | out | ribosomes per mRNA at steady state (when k_init is given) | - |
| `spacing_codons` | out | mean spacing between ribosomes in codons (when k_init is given) | - |

**Applies when:** n_aa >= 1; rate > 0.

```shell
$ cell-molecular-biology solve cmb_translation_time n_aa=300 rate=6
t = 50 s
t_min = 0.8333333333
coding_nt = 903
  by: Time to translate a protein  (t = n_aa / rate; coding length = 3 (n_aa + 1); ribosomes = k_init t)
  rate = 6
  assumes: n_aa >= 1; rate > 0
```

**What the worked example means:** Translation of a protein with 300 amino acids at an elongation rate of 6 takes 50 s seconds, corresponding to a coding length of 903 nucleotides.

### Solution biochemistry (13)

<a id="cmb_buffer_capacity"></a>

#### Buffer capacity of a weak-acid buffer: `cmb_buffer_capacity`

The buffer capacity beta = dCb/dpH (strong base per unit pH change) of a monoprotic buffer of total concentration C at a pH, including water, and the buffer's own contribution (25 degC, Kw = 1e-14).

$$
\beta = 2.303\left([\mathrm{H^+}] + \frac{K_w}{[\mathrm{H^+}]} + \frac{C K_a[\mathrm{H^+}]}{(K_a + [\mathrm{H^+}])^2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | total buffer concentration | mol/m^3 |
| `pKa` | in | pKa of the buffer acid | - |
| `pH` | in | pH | - |
| `beta` | out | buffer capacity per pH unit | mol/m^3 |
| `beta_mM` | out | buffer capacity in mM per pH unit | - |
| `beta_buffer_mM` | out | contribution of the buffer alone in mM per pH unit | - |

**Applies when:** C > 0; monoprotic acid; Kw = 1e-14 (25 degC), ideal solution.

```shell
$ cell-molecular-biology solve cmb_buffer_capacity "C=100 (mmol/L)" pKa=7.21 pH=7.21
beta = 57.56514274 mol/m^3
beta_mM = 57.56514274
beta_buffer_mM = 57.56462732
  by: Buffer capacity of a weak-acid buffer  (beta = 2.303 ([H+] + Kw/[H+] + C Ka [H+] / (Ka + [H+])^2))
  C = 100 mol/m^3
  pKa = 7.21
  pH = 7.21
  assumes: C > 0; monoprotic acid; Kw = 1e-14 (25 degC), ideal solution
```

**What the worked example means:** This calculation determines the buffer capacity, which measures how much strong base is needed to change the pH by one unit. It combines the effects of free hydrogen ions, hydroxide ions from water, and the weak acid equilibrium. The total capacity computed here is 57.5651 mol/m^3, while the portion arising solely from the buffer species is 57.5646. A higher value indicates a solution that resists pH changes more effectively when acids or bases are added.

<a id="cmb_buffer_after_addition"></a>

#### Buffer pH after adding strong acid or base, `cmb_buffer_after_addition`

The pH of a buffer (total concentration C, volume V, initial pH0) after adding an amount of strong acid or strong base, by Henderson-Hasselbalch on the new amounts.

$$
\mathrm{pH} = \mathrm{p}K_a + \log_{10}\frac{n_{A^-} \pm n}{n_{HA} \mp n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | total buffer concentration | mol/m^3 |
| `V` | in | buffer volume | m^3 |
| `pH0` | in | initial pH | - |
| `pKa` | in | pKa of the buffer acid | - |
| `n_add` | in | amount of strong acid or base added | mol |
| `added` | in | strong acid or strong base | - |
| `pH` | out | new pH | - |
| `delta_pH` | out | change in pH | - |
| `n_base` | out | base form after the addition | mol |
| `n_acid` | out | acid form after the addition | mol |

**Applies when:** C, V, n_add > 0; the addition smaller than the form it consumes; Henderson-Hasselbalch valid.

```shell
$ cell-molecular-biology solve cmb_buffer_after_addition "C=100 (mmol/L)" "V=100 mL" pH0=7.21 pKa=7.21 "n_add=1 mmol" added=base
pH = 7.386091259
delta_pH = 0.1760912591
n_base = 0.006 mol
n_acid = 0.004 mol
  by: Buffer pH after adding strong acid or base  (split C V by the initial ratio 10^(pH0 - pKa); base converts HA to A- (acid the reverse); pH = pKa + log10(n_A/n_HA))
  C = 100 mol/m^3
  V = 0.0001 m^3
  pH0 = 7.21
  pKa = 7.21
  n_add = 0.001 mol
  added = base
  assumes: C, V, n_add > 0; the addition smaller than the form it consumes; Henderson-Hasselbalch valid
```

In the worked example, C = 100 (mmol/L), V = 100 mL, pH0 = 7.21, pKa = 7.21, n_add = 1 mmol, added = base go through the equation above and give pH = 7.386091259, delta_pH = 0.1760912591, n_base = 0.006 mol, n_acid = 0.004 mol. It applies only when C, V, n_add > 0; the addition smaller than the form it consumes; Henderson-Hasselbalch valid: outside that it says what is wrong instead of returning a number.

<a id="cmb_henderson_hasselbalch"></a>

#### Buffer pH by the Henderson-Hasselbalch equation, `cmb_henderson_hasselbalch`

The pH of a buffer from the pKa and the concentrations of the conjugate base and the weak acid.

$$
\mathrm{pH} = \mathrm{p}K_a + \log_{10}\frac{[\mathrm{A^-}]}{[\mathrm{HA}]}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pKa` | in | pKa of the weak acid | - |
| `A` | in | conjugate base concentration | mol/m^3 |
| `HA` | in | weak acid concentration | mol/m^3 |
| `pH` | out | buffer pH | - |
| `ratio` | out | [A-]/[HA] | - |
| `fraction_base` | out | fraction in the base form | - |

**Applies when:** A, HA > 0; buffer concentrations well above [H+] and [OH-].

```shell
$ cell-molecular-biology solve cmb_henderson_hasselbalch pKa=7.21 "A=60 (mmol/L)" "HA=40 (mmol/L)"
pH = 7.386091259
ratio = 1.5
fraction_base = 0.6
  by: Buffer pH by the Henderson-Hasselbalch equation  (pH = pKa + log10([A-]/[HA]))
  pKa = 7.21
  A = 60 mol/m^3
  HA = 40 mol/m^3
  assumes: A, HA > 0; buffer concentrations well above [H+] and [OH-]
```

**What the worked example means:** This calculation determines the pH of a buffer solution using the Henderson-Hasselbalch equation. It takes the pKa of the weak acid, which is 7.21, and the concentrations of the conjugate base at 60 (mmol/L) and the weak acid at 40 (mmol/L). The resulting pH is 7.38609, indicating the acidity level maintained by the buffer system.

<a id="cmb_c1v1_dilution"></a>

#### Dilution of a stock solution (C1 V1 = C2 V2), `cmb_c1v1_dilution`

The volume of stock and of diluent needed to make a volume V2 at concentration C2 from a stock at C1.

$$
C_1V_1 = C_2V_2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C1` | in | stock concentration | mol/m^3 |
| `C2` | in | final concentration | mol/m^3 |
| `V2` | in | final volume | m^3 |
| `V1` | out | volume of stock | m^3 |
| `V1_uL` | out | volume of stock in uL | - |
| `V_diluent` | out | volume of diluent | m^3 |
| `dilution_factor` | out | dilution factor C1/C2 | - |

**Applies when:** C1, C2, V2 > 0; C2 <= C1; volumes additive.

```shell
$ cell-molecular-biology solve cmb_c1v1_dilution "C1=1000 (mmol/L)" "C2=50 (mmol/L)" "V2=10 mL"
V1 = 5e-07 m^3
V1_uL = 500
V_diluent = 9.5e-06 m^3
dilution_factor = 20
  by: Dilution of a stock solution (C1 V1 = C2 V2)  (V1 = C2 V2 / C1; diluent = V2 - V1)
  C1 = 1000 mol/m^3
  C2 = 50 mol/m^3
  V2 = 1e-05 m^3
  assumes: C1, C2, V2 > 0; C2 <= C1; volumes additive
```

**What the worked example means:** This calculation determines the specific volumes required to prepare a diluted solution using the conservation of mass principle, expressed as C1 V1 equals C2 V2. Given a stock concentration of 1000 (mmol/L) and a desired final concentration of 50 (mmol/L) for a target volume of 10 mL, the algorithm computes that 5e-07 m^3 of stock is needed. Consequently, 9.5e-06 m^3 of diluent must be added to reach the final volume. The process assumes that volumes are additive and that the final concentration does not exceed the stock concentration.

<a id="cmb_ionic_strength"></a>

#### Ionic strength of a solution: `cmb_ionic_strength`

The ionic strength I = (1/2) sum c z^2 of a solution from the concentration and charge of each ion.

$$
I = \tfrac{1}{2}\sum_i c_i z_i^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `c` | in | ion concentrations in mM (one per ion) | - |
| `z` | in | charge of each ion (e.g. 1, -1, 2) | - |
| `I` | out | ionic strength | mol/m^3 |
| `I_mM` | out | ionic strength in mM | - |
| `I_M` | out | ionic strength in M | - |

**Applies when:** one charge per concentration; charges non-zero whole numbers.

```shell
$ cell-molecular-biology solve cmb_ionic_strength "c=[150, 150]" "z=[1, -1]"
I = 150 mol/m^3
I_mM = 150
I_M = 0.15
  by: Ionic strength of a solution  (I = 1/2 sum c_i z_i^2)
  assumes: one charge per concentration; charges non-zero whole numbers
```

**What the worked example means:** This calculation determines the ionic strength, which quantifies the concentration of electrically charged particles in a solution. It uses the formula where each ion's contribution is its concentration multiplied by the square of its charge, summed together and then halved. The result, 150, reflects how strongly the ions influence electrostatic interactions within the mixture, affecting properties like protein stability and reaction rates.

<a id="cmb_mass_for_molarity"></a>

#### Mass of solute for a solution of given molarity, `cmb_mass_for_molarity`

The mass of a compound to weigh out for a volume V of solution at molar concentration C.

$$
m = C\,V\,M
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | desired concentration | mol/m^3 |
| `V` | in | volume of solution | m^3 |
| `MW` | in | molar mass of the compound | kg/mol |
| `m` | out | mass to weigh | kg |
| `m_g` | out | mass in grams | - |
| `m_mg` | out | mass in milligrams | - |
| `n` | out | amount of substance | mol |

**Applies when:** C, V, MW > 0.

```shell
$ cell-molecular-biology solve cmb_mass_for_molarity "C=150 (mmol/L)" "V=500 mL" "MW=58.44 g/mol"
m = 0.004383 kg
m_g = 4.383
m_mg = 4383
n = 0.075 mol
  by: Mass of solute for a solution of given molarity  (m = C V MW)
  C = 150 mol/m^3
  V = 0.0005 m^3
  MW = 0.05844 kg/mol
  assumes: C, V, MW > 0
```

**What the worked example means:** This capability determines the precise mass of a compound required to create a solution with a specific molarity. It multiplies the target concentration 150 (mmol/L) by the final volume 500 mL to find the total amount of substance needed, then converts this mole count into mass using the compound's molar mass 58.44 g/mol. The resulting weight to measure on a balance is 0.004383 kg, which equals 4.383 grams or 4383 milligrams depending on the scale of the experiment.

<a id="cmb_molarity_from_mass"></a>

#### Molar concentration from mass and volume: `cmb_molarity_from_mass`

The molar concentration of a solution made by dissolving a mass of compound to a final volume V.

$$
C = \frac{m}{M\,V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of solute | kg |
| `MW` | in | molar mass | kg/mol |
| `V` | in | final volume | m^3 |
| `C` | out | molar concentration | mol/m^3 |
| `C_mM` | out | concentration in mM | - |
| `C_M` | out | concentration in M | - |
| `n` | out | amount of substance | mol |

**Applies when:** m, MW, V > 0.

```shell
$ cell-molecular-biology solve cmb_molarity_from_mass "m=2920 mg" "MW=58.44 g/mol" "V=500 mL"
C = 99.93155373 mol/m^3
C_mM = 99.93155373
C_M = 0.09993155373
n = 0.04996577687 mol
  by: Molar concentration from mass and volume  (C = m / (MW V))
  m = 0.00292 kg
  MW = 0.05844 kg/mol
  V = 0.0005 m^3
  assumes: m, MW, V > 0
```

**What the worked example means:** This calculation determines the molar concentration of a solution by dividing the mass of the solute by the product of its molar mass and the final volume. Using an input mass of 2920 mg, a molar mass of 58.44 g/mol, and a volume of 500 mL, the resulting concentration is 99.9316 mol/m^3 in moles per cubic meter. This value corresponds to 0.0999316 in molarity and 99.9316 in millimolar. The amount of substance present is calculated as 0.0499658 mol moles.

<a id="cmb_percent_wv_to_molarity"></a>

#### Molarity of a per cent (w/v) solution: `cmb_percent_wv_to_molarity`

The molar concentration and grams per litre of a solution given in per cent weight per volume (g per 100 mL), e.g. 0.9 % NaCl.

$$
C = \frac{10\times(\%\,w/v)}{M}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `p` | in | concentration in per cent w/v (written with %, e.g. 0.9%) | - |
| `MW` | in | molar mass of the solute | kg/mol |
| `C` | out | molar concentration | mol/m^3 |
| `C_mM` | out | concentration in mM | - |
| `C_M` | out | concentration in M | - |
| `mass_concentration` | out | mass concentration | kg/m^3 |
| `g_per_L` | out | grams per litre | - |

**Applies when:** 0 < p < 100 %; MW > 0.

```shell
$ cell-molecular-biology solve cmb_percent_wv_to_molarity p=0.009 "MW=58.44 g/mol"
C = 154.0041068 mol/m^3
C_mM = 154.0041068
C_M = 0.1540041068
mass_concentration = 9 kg/m^3
g_per_L = 9
  by: Molarity of a per cent (w/v) solution  (g/L = 10 x (% w/v); C = (g/L) / MW)
  p = 0.009
  MW = 0.05844 kg/mol
  assumes: 0 < p < 100 %; MW > 0
```

**What the worked example means:** This calculation determines the molarity and grams per litre of a solution defined by its weight-per-volume percentage. By converting the input percentage 0.009 into grams per litre, the solution contains 9. Dividing this mass concentration by the solute's molar mass 58.44 g/mol gives the final molar concentration of 0.154004, or equivalently 154.004 millimolar.

<a id="cmb_buffer_preparation"></a>

#### Recipe for a buffer of given pH and concentration, `cmb_buffer_preparation`

The concentrations, amounts and (with molar masses) masses of the weak acid and its conjugate base needed for a volume V of buffer at total concentration C and a target pH.

$$
\frac{[\mathrm{A^-}]}{[\mathrm{HA}]} = 10^{\mathrm{pH} - \mathrm{p}K_a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C` | in | total buffer concentration | mol/m^3 |
| `V` | in | volume of buffer | m^3 |
| `pH` | in | target pH | - |
| `pKa` | in | pKa of the buffer acid | - |
| `MW_acid` | in | molar mass of the acid form (optional) (optional) | kg/mol |
| `MW_base` | in | molar mass of the base form (optional) (optional) | kg/mol |
| `C_base` | out | base-form concentration | mol/m^3 |
| `C_acid` | out | acid-form concentration | mol/m^3 |
| `n_base` | out | amount of base form | mol |
| `n_acid` | out | amount of acid form | mol |
| `ratio` | out | [A-]/[HA] | - |
| `m_acid_g` | out | grams of the acid form (when MW_acid is given) | - |
| `m_base_g` | out | grams of the base form (when MW_base is given) | - |

**Applies when:** C, V > 0; Henderson-Hasselbalch valid (pH within about 1 of pKa).

```shell
$ cell-molecular-biology solve cmb_buffer_preparation "C=100 (mmol/L)" "V=1000 mL" pH=7.4 pKa=7.21 "MW_acid=137.99 g/mol" "MW_base=141.96 g/mol"
C_base = 60.76610641 mol/m^3
C_acid = 39.23389359 mol/m^3
n_base = 0.06076610641 mol
n_acid = 0.03923389359 mol
ratio = 1.548816619
m_acid_g = 5.413884976
m_base_g = 8.626356466
  by: Recipe for a buffer of given pH and concentration  (r = 10^(pH - pKa); C_base = C r/(1 + r); C_acid = C - C_base; n = C V; m = n M)
  C = 100 mol/m^3
  V = 0.001 m^3
  pH = 7.4
  pKa = 7.21
  MW_acid = 0.13799 kg/mol
  MW_base = 0.14196 kg/mol
  assumes: C, V > 0; Henderson-Hasselbalch valid (pH within about 1 of pKa)
```

**What the worked example means:** This calculation determines the precise amounts of weak acid and conjugate base required to prepare a buffer solution. Given a target pH, the ratio of base to acid is found using the Henderson-Hasselbalch equation. This ratio, combined with the total desired concentration 100 (mmol/L) and volume 1000 mL, yields the individual concentrations 39.2339 mol/m^3 and 60.7661 mol/m^3. Multiplying these by the volume gives the molar amounts needed for weighing or dispensing.

<a id="cmb_centrifuge_rcf"></a>

#### Relative centrifugal force from rotor speed, `cmb_centrifuge_rcf`

The relative centrifugal force (x g) at radius r for a rotor speed in rpm.

$$
\mathrm{RCF} = \frac{\omega^2 r}{g} = 1.118\times10^{-5}\,r_{cm}\,N_{rpm}^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rpm` | in | rotor speed | rad/s |
| `r` | in | radius from the rotor axis | m |
| `RCF` | out | relative centrifugal force in multiples of g | - |
| `rpm_value` | out | rotor speed in rpm | - |

**Applies when:** rpm, r > 0.

```shell
$ cell-molecular-biology solve cmb_centrifuge_rcf "rpm=3000 rpm" "r=10 cm"
RCF = 1006.419562
rpm_value = 3000
  by: Relative centrifugal force from rotor speed  (RCF = omega^2 r / g = 1.118e-5 r(cm) rpm^2)
  rpm = 314.1592654 rad/s
  r = 0.1 m
  assumes: rpm, r > 0
```

**What the worked example means:** This calculation determines the relative centrifugal force expressed as multiples of Earth's gravity. It uses the rotor speed of 3000 rpm and the radius from the axis of 10 cm. The resulting force is 1006.42, which indicates how strongly particles are pushed outward during centrifugation.

<a id="cmb_centrifuge_rpm"></a>

#### Rotor speed for a required centrifugal force, `cmb_centrifuge_rpm`

The rotor speed (rpm) that gives a required relative centrifugal force (x g) at radius r.

$$
N_{rpm} = \frac{60}{2\pi}\sqrt{\frac{\mathrm{RCF}\,g}{r}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `RCF` | in | required relative centrifugal force in multiples of g | - |
| `r` | in | radius from the rotor axis | m |
| `omega` | out | angular speed | rad/s |
| `rpm_value` | out | rotor speed in rpm | - |

**Applies when:** RCF, r > 0.

```shell
$ cell-molecular-biology solve cmb_centrifuge_rpm RCF=1000 "r=10 cm"
omega = 313.1557121 rad/s
rpm_value = 2990.416772
  by: Rotor speed for a required centrifugal force  (rpm = sqrt(RCF g / r) x 60 / (2 pi))
  RCF = 1000
  r = 0.1 m
  assumes: RCF, r > 0
```

**What the worked example means:** This calculation determines the angular velocity and rotational speed needed for a centrifuge rotor. Given a required relative centrifugal force of 1000 at a radius of 10 cm, the model computes an angular speed of 313.156 rad/s radians per second, which corresponds to a rotor speed of 2990.42. This ensures that samples placed at the specified distance from the axis experience the desired gravitational equivalent force.

<a id="cmb_serial_dilution"></a>

#### Serial dilution series: `cmb_serial_dilution`

The concentrations after each step of a serial dilution with a constant factor, the final concentration and total dilution, and (with the tube volume) the transfer and diluent volumes per step.

$$
C_k = \frac{C_0}{f^k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | starting concentration | mol/m^3 |
| `f` | in | dilution factor per step (e.g. 10) | - |
| `n` | in | number of dilution steps | - |
| `V` | in | final volume in each tube (optional) (optional) | m^3 |
| `C_final` | out | concentration after n steps | mol/m^3 |
| `total_dilution` | out | total dilution factor f^n | - |
| `concentrations_mM` | out | concentration after each step in mM | - |
| `transfer` | out | volume carried to the next tube (when V is given) | m^3 |
| `diluent` | out | diluent per tube (when V is given) | m^3 |

**Applies when:** C0 > 0; f > 1; 1 <= n <= 30.

```shell
$ cell-molecular-biology solve cmb_serial_dilution "C0=10 (mmol/L)" f=10 n=4
C_final = 0.001 mol/m^3
total_dilution = 10000
concentrations_mM = [1.0,0.1,0.01,0.001]
  by: Serial dilution series  (C_k = C0 / f^k; transfer = V/f, diluent = V - V/f)
  C0 = 10 mol/m^3
  f = 10
  assumes: C0 > 0; f > 1; 1 <= n <= 30
```

In the worked example, C0 = 10 (mmol/L), f = 10, n = 4 go through the equation above and give C_final = 0.001 mol/m^3, total_dilution = 10000, concentrations_mM = [1, 0.1, 0.01, 0.001]. It applies only when C0 > 0; f > 1; 1 <= n <= 30: outside that it says what is wrong instead of returning a number.

<a id="cmb_sedimentation_time"></a>

#### Time to pellet a particle in a centrifuge (k-factor), `cmb_sedimentation_time`

The time to sediment a particle of sedimentation coefficient s from r_min to r_max at a rotor speed, and the rotor's k-factor (t in hours = k / s).

$$
t = \frac{\ln(r_{max}/r_{min})}{\omega^2 s},\quad k = \frac{\ln(r_{max}/r_{min})}{\omega^2}\cdot\frac{10^{13}}{3600}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `s` | in | sedimentation coefficient in svedbergs | - |
| `rpm` | in | rotor speed | rad/s |
| `r_min` | in | starting (minimum) radius | m |
| `r_max` | in | radius of the pellet (maximum radius) | m |
| `t` | out | sedimentation time | s |
| `t_h` | out | time in hours | - |
| `k_factor` | out | k-factor (t_h = k / s) | - |

**Applies when:** s, rpm > 0; 0 < r_min < r_max; constant s (no diffusion).

```shell
$ cell-molecular-biology solve cmb_sedimentation_time s=70 "rpm=40000 rpm" "r_min=5 cm" "r_max=10 cm"
t = 5643.521741 s
t_h = 1.567644928
k_factor = 109.735145
  by: Time to pellet a particle in a centrifuge (k-factor)  (dr/dt = s omega^2 r: t = ln(r_max/r_min) / (omega^2 s); k = ln(r_max/r_min) / omega^2 x 1e13 / 3600)
  s = 70
  rpm = 4188.790205 rad/s
  r_min = 0.05 m
  r_max = 0.1 m
  assumes: s, rpm > 0; 0 < r_min < r_max; constant s (no diffusion)
```

**What the worked example means:** This calculation determines how long it takes for a particle with a sedimentation coefficient of 70 to move from a starting radius of 5 cm to a maximum radius of 10 cm while spinning at 40000 rpm. The result shows that the process requires 5643.52 s seconds, or 1.56764 hours. Additionally, the rotor's efficiency is characterized by a k-factor of 109.735, which allows you to estimate the time in hours by dividing this value by the particle's sedimentation coefficient.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
