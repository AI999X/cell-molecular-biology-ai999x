# Cell and Molecular Biology: the desktop app, user guide

**Cell and Molecular Biology**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Cell and Molecular Biology covers membranes and diffusion, osmosis and membrane potentials, cell growth and the cell cycle, DNA and RNA melting temperatures and quantification, sequences, PCR and qPCR, gene expression, enzyme kinetics, inhibition and binding, buffers and dilutions. Its desktop app holds all 104 calculations in 10 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-cell-molecular-biology-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-cell-molecular-biology-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-cell-molecular-biology-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-cell-molecular-biology.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-cell-molecular-biology-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-cell-molecular-biology-1.6.5-linux-x86_64.tar.gz
./ai999x-cell-molecular-biology-1.6.5-linux-x86_64/ai999x-cell-molecular-biology
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Cell and Molecular Biology app, opened on Nernst equilibrium potential of an ion across a cell membrane with its worked example.](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/hero.webp)

*The Cell and Molecular Biology app, opened on Nernst equilibrium potential of an ion across a cell membrane with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Membranes and diffusion › Nernst equilibrium potential of an ion across a cell membrane (or type part of its name in the search box). It opens with a worked example filled in: “Nernst potential: z = 1, C_out = 5 mM, C_in = 140 mM, T = 310.15 K” The answer: E = -0.08905869404 V, E_mV = -89.05869404.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Nernst equilibrium potential of an ion across a cell membrane: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/scene-1.webp)

*Nernst equilibrium potential of an ion across a cell membrane: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it: the app reads the quantities and their units, picks the calculation and fills in its form.

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Nearest-neighbour melting temperature of a DNA duplex](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/scene-2.webp)

*Nearest-neighbour melting temperature of a DNA duplex*

![qPCR relative expression by the delta-delta Ct method](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/scene-3.webp)

*qPCR relative expression by the delta-delta Ct method*

![Fractional saturation by the Hill equation](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/scene-4.webp)

*Fractional saturation by the Hill equation*

![Recipe for a buffer of given pH and concentration](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/scene-5.webp)

*Recipe for a buffer of given pH and concentration*

![Cell-cycle phase durations corrected for the exponential age distribution](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/scene-6.webp)

*Cell-cycle phase durations corrected for the exponential age distribution*

![Competitive inhibition rate](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/scene-7.webp)

*Competitive inhibition rate*

![ATP yield per glucose: aerobic respiration versus fermentation](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/scene-8.webp)

*ATP yield per glucose: aerobic respiration versus fermentation*

![Nernst equilibrium potential of an ion across a cell membrane: a chart, enlarged.](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/chart-1.webp)

*Nernst equilibrium potential of an ion across a cell membrane: a chart, enlarged.*

![Nearest-neighbour melting temperature of a DNA duplex: a chart, enlarged.](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/chart-2.webp)

*Nearest-neighbour melting temperature of a DNA duplex: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Cell and Molecular Biology app in the dark theme.](https://www.ai999x.com/?ax_shot=cell-molecular-biology-ai999x/hero-dark.webp)

*The Cell and Molecular Biology app in the dark theme.*

## 7. Every calculation

104 calculations in 10 topics; each has its own entry on the model's page (https://www.ai999x.com/models/cell-molecular-biology-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Cell growth | Batch culture time under Monod kinetics, Cell concentration from a hemocytometer count, Cell-cycle phase durations corrected for the exponential age distribution, Cell-cycle phase durations from phase fractions, Colony-forming units per mL from a plate count, Doubling time and number of generations from two counts, Exponential growth of a cell population, Logistic growth of a population, Monod specific growth rate, Number of molecules in a cell from a concentration, Specific growth rate fitted to growth-curve data, Time for a logistic population to reach a size, Transformation efficiency, Volume, surface area and surface-to-volume ratio of a spherical cell |
| DNA and RNA | A260/A280 and A260/A230 purity ratios, Basic GC melting temperature of an oligonucleotide, Copy number and moles of a DNA or RNA molecule from its mass, Extinction coefficient of a DNA oligonucleotide at 260 nm (nearest neighbour), GC content of a DNA or RNA sequence, Insert mass for a ligation at an insert:vector molar ratio, Longest open reading frame on both strands, Molecular weight of a DNA or RNA oligonucleotide, Mutation rate by the fluctuation-test p0 method, Nearest-neighbour melting temperature of a DNA duplex, Nucleic acid concentration from A260, Number of clones for a genomic library (Clarke-Carbon), Primer melting temperature by the Wallace rule, Protein concentration by Warburg-Christian (A280 and A260), Protein concentration from A280 and the extinction coefficient, Reverse complement of a DNA or RNA sequence, Salt-adjusted melting temperature of an oligonucleotide, Sequencing coverage and uncovered fraction (Lander-Waterman), Transcription of a DNA strand into mRNA, Translation of an mRNA or DNA sequence into protein |
| Enzyme kinetics | Competitive inhibition rate, Enzyme activity and specific activity from a spectrophotometric assay, Inhibition constant from IC50 (Cheng-Prusoff), Km and Vmax by nonlinear least squares, Km and Vmax from a Lineweaver-Burk plot, Michaelis-Menten reaction rate, Mixed inhibition rate, Noncompetitive inhibition rate, Rate at another temperature from Q10, Substrate concentration for a fraction of Vmax, Turnover number kcat and catalytic efficiency kcat/Km, Uncompetitive inhibition rate |
| Enzyme kinetics and binding | Complex concentration with ligand depletion (quadratic binding), Dissociation constant from association and dissociation rates, Fraction of receptor bound at a free ligand concentration, Fractional saturation by the Hill equation, Fractional saturation in the MWC (concerted) model, Hill coefficient from EC10 and EC90, Kd and Bmax from a Scatchard plot, Ligand concentration for a given saturation (Hill), Receptor occupancy with a competing ligand, Standard binding free energy from Kd |
| Membranes and diffusion | Cell volume after an osmotic change (Boyle-van 't Hoff), Diffusion coefficient from a FRAP half-time (Soumpasis), Diffusion-limited association rate constant (Smoluchowski), Direction of water movement between a cell and its surroundings, Donnan equilibrium with an impermeant anion, Equilibration of a solute across a spherical cell's membrane, Fick's first law: steady diffusion flux, Macroscopic current through ion channels, Membrane permeability coefficient and flux, Nernst equilibrium potential of an ion across a cell membrane, Osmolarity of a solution and its tonicity for a cell, Osmotic pressure (van 't Hoff), Root-mean-square displacement of a diffusing particle, Stokes-Einstein diffusion coefficient, Time to diffuse a distance, Water potential of a plant cell or solution (solute plus pressure potential) |
| Metabolism | ATP yield per glucose: aerobic respiration versus fermentation, Free-energy change of a reaction at cellular concentrations, Free-energy change of a redox reaction from reduction potentials |
| Microbial control | D-value (decimal reduction time) from survivor counts, D-value at a new temperature and time to a target kill (z-value), z-value from D-values at two temperatures |
| Microscopy | Actual size of a specimen from a micrograph, Light microscope: total magnification and resolution limit |
| PCR and gene expression | mRNA and protein after switching a gene on, mRNA half-life fitted to a decay time course, PCR amplification after n cycles, PCR cycles needed to reach a number of copies, qPCR relative expression by the delta-delta Ct method, qPCR relative expression with efficiency correction (Pfaffl), qPCR standard curve: slope, intercept and efficiency, Quantity from a Ct and a standard curve, Steady-state mRNA and protein numbers, Time to transcribe a gene, Time to translate a protein |
| Solution biochemistry | Buffer capacity of a weak-acid buffer, Buffer pH after adding strong acid or base, Buffer pH by the Henderson-Hasselbalch equation, Dilution of a stock solution (C1 V1 = C2 V2), Ionic strength of a solution, Mass of solute for a solution of given molarity, Molar concentration from mass and volume, Molarity of a per cent (w/v) solution, Recipe for a buffer of given pH and concentration, Relative centrifugal force from rotor speed, Rotor speed for a required centrifugal force, Serial dilution series, Time to pellet a particle in a centrifuge (k-factor) |

## 8. Other ways to use it

The same engine runs on the command line (`cell-molecular-biology`, its own guide), in Python (`pip install cell-molecular-biology-ai999x`, `import cell_molecular_biology_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/cell-molecular-biology-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/cell-molecular-biology-ai999x/
