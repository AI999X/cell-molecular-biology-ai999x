//! Cell Molecular Biology's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: value maps over two inputs with iso-lines, one output against one input, a list value moved, bars over
//! the options of a choice, a verdict along an input, a dilution series, measured data with the fit the model returns,
//! and sequences with what the model returns for them (docs/FIGURE_PARITY.md).

use crate::figs_common::{self as c, Chart, FieldSpec, Strip};
use ax_engine::figkit::{self as k, Solved};
use ax_engine::figures::Figure;
use ax_engine::{solve, Input};

/// The field maps: capability, output shown, input across, input up, logarithmic (across, up), title.
const FIELDS: &[(&str, &str, &str, &str, (bool, bool), &str)] = &[
    ("cmb_binding_free_energy", "dG", "T", "Kd", (false, false),
     "Standard binding free energy over temperature and dissociation constant"),
    ("cmb_centrifuge_rcf", "RCF", "rpm", "r", (false, false), "Relative centrifugal force over rotor speed and radius"),
    ("cmb_centrifuge_rpm", "omega", "RCF", "r", (false, false),
     "Rotor speed for a required centrifugal force over the force and the radius"),
    ("cmb_dna_copy_number", "copies", "n", "m", (true, false),
     "Copy number of a nucleic acid over its length and its mass"),
    ("cmb_fraction_bound", "theta", "L", "Kd", (true, true),
     "Fraction of receptor bound over free ligand and dissociation constant"),
    ("cmb_frap_diffusion", "D", "w", "t_half", (false, false),
     "Diffusion coefficient from FRAP over spot radius and recovery half-time"),
    ("cmb_kd_from_rates", "Kd", "kon", "koff", (false, false),
     "Dissociation constant over association and dissociation rate constants"),
    ("cmb_molecules_per_cell", "N", "c", "V", (false, false),
     "Number of molecules in a cell over concentration and cell volume"),
    ("cmb_nucleic_acid_a260", "c", "A260", "dilution", (false, false),
     "Nucleic acid concentration over A260 and dilution factor"),
    ("cmb_percent_wv_to_molarity", "C", "p", "MW", (false, false),
     "Molarity of a per cent (w/v) solution over the per cent and the molar mass"),
    ("cmb_substrate_for_fraction_of_vmax", "S", "f", "Km", (false, false),
     "Substrate concentration for a fraction of Vmax over the fraction and Km"),
    ("cmb_transcription_time", "t", "rate", "n", (false, true),
     "Time to transcribe a gene over elongation rate and gene length"),
    ("cmb_translation_time", "t", "rate", "n_aa", (false, true),
     "Time to translate a protein over elongation rate and protein length"),
    ("cmb_warburg_christian", "protein", "A280", "A260", (false, false),
     "Protein concentration (Warburg-Christian) over A280 and A260"),
];

/// Inputs a field needs that may be left to their default: capability, input, the default as the model reads it.
const DEFAULTS: &[(&str, &str, f64)] = &[("cmb_nucleic_acid_a260", "dilution", 1.0)];

/// The sweeps: capability, kind, output, input, logarithmic (across, up), title.
const SWEEPS: &[(&str, &str, &str, &str, (bool, bool), &str)] = &[
    ("cmb_doubling_time_from_counts", "td-vs-n0", "td", "N0", (false, false),
     "Doubling time against the initial count"),
    ("cmb_hill_equation", "theta-vs-l", "theta", "L", (true, false),
     "Fractional saturation (Hill equation) against free ligand"),
    ("cmb_nernst_potential", "e-vs-c-out", "E", "C_out", (true, false),
     "Nernst potential against the outside concentration"),
    ("cmb_qpcr_delta_delta_ct", "fold-change-vs-ct-target-treated", "fold_change", "Ct_target_treated", (false, true),
     "Fold change (delta-delta Ct) against the target Ct of the treated sample"),
];

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let mut v: Vec<Option<Figure>> = vec![];
    if let Some(f) = FIELDS.iter().find(|f| f.0 == s.key) {
        let spec = FieldSpec { z: f.1, x: f.2, y: f.3, log: f.4, title: f.5, zunit: "" };
        match DEFAULTS.iter().find(|d| d.0 == s.key && c::getv(s, d.1).is_none()) {
            Some(d) => v.push(field_with_default(s, &spec, d.1, d.2)),
            None => v.push(c::field_map(s, &spec)),
        }
    }
    if let Some(w) = SWEEPS.iter().find(|w| w.0 == s.key) {
        v.push(sweep_xy(s, w.1, w.5, w.2, w.3, w.4));
    }
    v.push(match s.key {
        "cmb_ionic_strength" => list_sweep(s, "Ionic strength as one ion concentration moves", "I", "c"),
        "cmb_qpcr_standard_curve" => list_sweep(s, "qPCR efficiency as the quantity of one standard moves", "efficiency",
                                                "quantity"),
        "cmb_oligo_molecular_weight" => c::option_bars(s, "bars", "Molecular weight of an oligonucleotide, single strand \
                                                                   and duplex", "MW", "strands"),
        "cmb_osmolarity_tonicity" => tonicity(s),
        "cmb_serial_dilution" => dilution_series(s),
        "cmb_growth_rate_fit" => growth_fit(s),
        "cmb_mrna_half_life_fit" => decay_fit(s),
        "cmb_lineweaver_burk_fit" => lineweaver(s),
        "cmb_michaelis_menten_nonlinear_fit" => mm_fit(s),
        "cmb_scatchard_fit" => scatchard(s),
        "cmb_gc_content" => io_gc(s),
        "cmb_oligo_extinction_coefficient" => io_extinction(s),
        "cmb_orf_finder" => io_orf(s),
        "cmb_reverse_complement" => io_revcomp(s),
        "cmb_tm_basic" => io_tm(s, "Tm", "Basic melting temperature of an oligonucleotide"),
        "cmb_tm_wallace" => io_tm(s, "Tm", "Primer melting temperature by the Wallace rule"),
        "cmb_transcribe" => io_transcribe(s),
        "cmb_translate" => io_translate(s),
        _ => None,
    });
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ fields
/// A field map when an input it explores was left to its default: the model is solved again with that default stated
/// (the same answer), and the map drawn round it.
fn field_with_default(s: &Solved, spec: &FieldSpec, sym: &str, def: f64) -> Option<Figure> {
    let mut given: Vec<(String, Input)> = s.given.to_vec();
    given.push((sym.to_string(), Input::Float(def)));
    let a = solve(s.p, s.key, &given, s.target);
    if !a.solved() || k::num_of(&a, spec.z) != s.out(spec.z) {
        return None;
    }
    let s2 = Solved::new(s.p, s.key, &given, s.target, &a);
    c::field_map(&s2, spec)
}

// ------------------------------------------------------------------------------------------------ sweeps
/// One output against one input round the visitor's value (a tenth to ten times on a logarithmic axis, else half to
/// one and a half times), 31 runs of the model, the visitor's point marked.
fn sweep_xy(s: &Solved, kind: &str, title: &str, y: &str, x: &str, log: (bool, bool)) -> Option<Figure> {
    let (x0, y0) = (c::getv(s, x)?, c::getv(s, y)?);
    let (lo, hi) = c::around(x0, log.0)?;
    let xs = c::grid(lo, hi, 31, log.0, c::is_int(s, x));
    let pts: Vec<(f64, f64)> = c::curve(s, x, &xs, y).into_iter().filter(|p| !log.1 || p.1 > 0.0).collect();
    if pts.len() < 2 || (log.1 && y0 <= 0.0) {
        return None;
    }
    let yr = Chart::range(&pts.iter().map(|p| p.1).chain([y0]).collect::<Vec<_>>());
    let xr = Chart::range(&pts.iter().map(|p| p.0).chain([x0]).collect::<Vec<_>>());
    let mut ch = Chart::new(xr, yr, log, (false, true), &c::axis_label(s, x), &c::axis_label(s, y),
                            &format!("{y} = {}", c::tick(y0)));
    ch.line(&pts, "axp-c", "", false);
    ch.mark((x0, y0), &format!("{x} = {}", c::tick(x0)));
    let body = ch.body();
    let caption = format!("{y} ({}) against {x} ({}) from {} to {}{}, the other inputs as given: {} points, each a run of \
                           the model. Marked: {x} = {}, {y} = {}.", c::meaning(s, y), c::meaning(s, x), c::tick(pts[0].0),
                          c::tick(pts[pts.len() - 1].0),
                          match log { (true, true) => " (logarithmic axes)", (true, false) => " (logarithmic axis across)",
                                      (false, true) => " (logarithmic axis up)", _ => "" },
                          pts.len(), c::tick(x0), c::tick(y0));
    Some(s.chart(kind, title, &caption, &body))
}

/// An output as the last value of a list input moves from half to one and a half times its value, the other values as
/// given (kind "sensitivity").
fn list_sweep(s: &Solved, title: &str, out: &str, list: &str) -> Option<Figure> {
    let v = s.vec(list)?;
    let y0 = s.out(out)?;
    let i = v.len().checked_sub(1)?;
    let x0 = v[i];
    let (lo, hi) = c::around(x0, false)?;
    let mut pts = vec![];
    for xv in k::span(lo, hi, 31, false) {
        let mut w = v.clone();
        w[i] = xv;
        let Some(a) = s.with_input(&[(list, Input::Json(serde_json::json!(w)))]) else { continue };
        if let Some(y) = k::num_of(&a, out) {
            pts.push((xv, y));
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let lab = format!("{list}[{}]", i + 1);
    let yr = Chart::range(&pts.iter().map(|p| p.1).chain([y0]).collect::<Vec<_>>());
    let mut ch = Chart::new((lo, hi), yr, (false, false), (false, true), &format!("value {} of {list}: {}", i + 1,
                            c::meaning(s, list)), &c::axis_label(s, out), &format!("{out} = {}", c::tick(y0)));
    ch.line(&pts, "axp-c", "", false);
    ch.mark((x0, y0), &format!("{lab} = {}", c::tick(x0)));
    let body = ch.body();
    let caption = format!("{out} ({}) as value {} of {list} moves from {} to {}, the other values as given ({}); {} runs of \
                           the model. Marked: {lab} = {}, {out} = {}.", c::meaning(s, out), i + 1, c::tick(lo), c::tick(hi),
                          v.iter().map(|x| c::tick(*x)).collect::<Vec<_>>().join(", "), pts.len(), c::tick(x0),
                          c::tick(y0));
    Some(s.chart("sensitivity", title, &caption, &body))
}

// ------------------------------------------------------------------------------------------------ tonicity
/// The tonicity verdict along the cell's osmolarity (a tenth to ten times the value given, 81 runs): each band
/// coloured by the verdict returned, the line the difference the model returns.
fn tonicity(s: &Solved) -> Option<Figure> {
    let x0 = s.get("osm_cell")?;
    let d0 = s.out("difference_mOsm")?;
    let mine = s.out_text("tonicity")?;
    let (lo, hi) = c::around(x0, true)?;
    let xs = k::span(lo, hi, 81, true);
    let mut runs: Vec<(f64, String, f64)> = vec![];
    for xv in &xs {
        let Some(a) = s.with(&[("osm_cell", *xv)]) else { continue };
        let Some(t) = a.values.iter().find(|v| v.0 == "tonicity").and_then(|v| v.1.as_str().map(String::from)) else {
            continue;
        };
        let Some(d) = k::num_of(&a, "difference_mOsm") else { continue };
        runs.push((*xv, t, d));
    }
    if runs.len() < 4 {
        return None;
    }
    let mut names: Vec<String> = vec![];
    for r in &runs {
        if !names.contains(&r.1) {
            names.push(r.1.clone());
        }
    }
    let ds: Vec<f64> = runs.iter().map(|r| r.2).chain([d0]).collect();
    let mut ch = Chart::new((lo, hi), Chart::range(&ds), (true, false), (false, true),
                            &c::axis_label(s, "osm_cell"), "solution minus cell (mOsm/L)", "");
    const BANDS: [&str; 3] = ["axp-s", "axp-s2", "axp-s3"];
    // each run's band reaches half-way (geometrically) to its neighbours
    let mut changes = vec![];
    for (i, r) in runs.iter().enumerate() {
        let l = if i == 0 { r.0 } else { (runs[i - 1].0 * r.0).sqrt() };
        let rr = if i + 1 == runs.len() { r.0 } else { (runs[i + 1].0 * r.0).sqrt() };
        let ci = names.iter().position(|n| *n == r.1).unwrap_or(0) % 3;
        ch.xband(l, rr, BANDS[ci]);
        if i > 0 && runs[i - 1].1 != r.1 {
            changes.push(l);
        }
    }
    ch.hguide(0.0, "");
    let pts: Vec<(f64, f64)> = runs.iter().map(|r| (r.0, r.2)).collect();
    ch.line(&pts, "axp-c", "", false);
    ch.mark((x0, d0), &mine);
    let mut body = ch.body();
    // the verdict key above the plot
    let mut x = c::CX0;
    for (i, n) in names.iter().enumerate() {
        body.push_str(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"14\" width=\"16\" height=\"12\"/><text class=\"axp-lt\" \
                                x=\"{:.1}\" y=\"25\">{}</text>", BANDS[i % 3], x + 20.0, c::esc(n)));
        x += 20.0 + n.chars().count() as f64 * 7.6 + 18.0;
    }
    let caption = format!("Tonicity (hypertonic, hypotonic or isotonic within 1 %) along osm_cell, the osmolarity of the cell \
                           interior, from {} to {} mol/m^3 (logarithmic axis): {} runs of the model, each band coloured by \
                           the verdict returned{}; the line is the difference, solution minus cell, that the model returns \
                           (the solution's osmolarity is {} mOsm/L). Marked: osm_cell = {} mol/m^3 gives {mine}.",
                          c::tick(lo), c::tick(hi), runs.len(),
                          if changes.is_empty() { String::new() } else {
                              format!("; it changes near osm_cell = {}", changes.iter().map(|v| c::tick(*v))
                                  .collect::<Vec<_>>().join(", "))
                          }, c::tick(s.out("osmolarity_mOsm")?), c::tick(x0));
    Some(s.chart("decision", "Tonicity of a solution for a cell along the cell's osmolarity", &caption, &body))
}

// ------------------------------------------------------------------------------------------------ dilution series
/// The concentration after each step of a serial dilution, as returned (logarithmic axis when it spans more than a
/// factor 100), each step a stem with its value.
fn dilution_series(s: &Solved) -> Option<Figure> {
    let v = s.out_list("concentrations_mM")?;
    if v.is_empty() || v.len() > 40 {
        return None;
    }
    let c0 = s.get("C0")?;
    let pos = v.iter().all(|x| *x > 0.0);
    let all: Vec<f64> = v.iter().copied().chain([c0]).collect();
    let (lo, hi) = Chart::range(&all);
    let log = pos && hi / lo > 100.0;
    let yr = if log { (lo, hi) } else { (0.0f64.min(lo), hi) };
    let n = v.len();
    let mut ch = Chart::new((-0.5, n as f64 + 0.5), yr, (false, log), (false, true), "step (0: the stock)",
                            "concentration (mM)", &format!("C_final = {} mM", c::tick(v[n - 1])));
    let base = if log { ch.ya.lo } else { 0.0 };
    let pts: Vec<(f64, f64)> = std::iter::once((0.0, c0)).chain(v.iter().enumerate().map(|(i, x)| ((i + 1) as f64, *x)))
        .collect();
    for (i, p) in pts.iter().enumerate() {
        ch.line(&[(p.0, base), *p], if i == 0 { "axp-c4" } else { "axp-c" }, "", false);
    }
    ch.dots(&pts[..1], "axp-d4", 5.0);
    ch.dots(&pts[1..], "axp-d", 5.0);
    if n <= 12 {
        for p in &pts {
            let (x, y) = (ch.xa.px(p.0), ch.ya.px(p.1));
            ch.text(x, y - 9.0, "middle", "axp-vt", &c::tick(p.1));
        }
    }
    let body = ch.body();
    let f = c::getv(s, "f")?;
    let caption = format!("Serial dilution of the stock C0 = {} mM by a factor {} per step: the concentration after each of \
                           the {n} steps as the model returns it ({}){}. Total dilution {}.", c::tick(c0), c::tick(f),
                          v.iter().map(|x| c::tick(*x)).collect::<Vec<_>>().join(", "),
                          if log { ", on a logarithmic axis" } else { "" }, c::tick(s.out("total_dilution")?));
    Some(s.chart("series", "Serial dilution: the concentration after each step", &caption, &body))
}

// ------------------------------------------------------------------------------------------------ fits
fn pairs(s: &Solved, x: &str, y: &str) -> Option<Vec<(f64, f64)>> {
    let (xs, ys) = (s.vec(x)?, s.vec(y)?);
    (xs.len() == ys.len() && xs.len() >= 2).then(|| xs.into_iter().zip(ys).collect())
}

/// Growth data on a logarithmic count axis with the exponential growth the model computes from its fitted N0 and
/// doubling time (the model's exponential-growth capability at 61 times).
fn growth_fit(s: &Solved) -> Option<Figure> {
    let d = pairs(s, "t", "N")?;
    let (n0, td_h, mu, r2) = (s.out("N0_fit")?, s.out("td_h")?, s.out("mu_per_h")?, s.out("r2")?);
    if d.iter().any(|p| p.1 <= 0.0) {
        return None;
    }
    let (t0, t1) = Chart::range(&d.iter().map(|p| p.0).collect::<Vec<_>>());
    let mut curve = vec![];
    for t in k::span(t0.min(0.0), t1, 61, false) {
        let a = s.solve_other("cmb_exponential_growth", &[("N0", Input::Float(n0)), ("td", Input::Float(td_h * 3600.0)),
                                                          ("t", Input::Float(t * 3600.0))]);
        if let Some(n) = a.as_ref().and_then(|a| k::num_of(a, "N")) {
            curve.push((t, n));
        }
    }
    if curve.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = d.iter().map(|p| p.1).chain(curve.iter().map(|p| p.1)).collect();
    let mut ch = Chart::new((t0.min(0.0), t1), Chart::range_pos(&ys), (false, true), (true, true), "time t (h)",
                            "count N (logarithmic)", &format!("mu = {} 1/h, td = {} h", c::tick(mu), c::tick(td_h)));
    ch.line(&curve, "axp-c", "fitted growth", false);
    ch.dots(&d, "axp-d2", 5.0);
    ch.legend.push(("axp-c2".into(), "data as given".into()));
    ch.mark((0.0, n0), &format!("N0 = {}", c::tick(n0)));
    let body = ch.body();
    let caption = format!("Growth data N against t as given ({} points, logarithmic count axis) and the exponential growth \
                           from the fitted N0 = {} and doubling time {} h, each point computed by the model's exponential \
                           growth capability. Returned: mu = {} per hour, r2 = {}.", d.len(), c::tick(n0), c::tick(td_h),
                          c::tick(mu), c::tick(r2));
    Some(s.chart("xy", "Specific growth rate fitted to growth-curve data", &caption, &body))
}

/// mRNA decay data on a logarithmic axis with the decay drawn from the fitted level at t = 0 and rate constant the
/// model returns (a straight line on this axis).
fn decay_fit(s: &Solved) -> Option<Figure> {
    let d = pairs(s, "t", "level")?;
    let (l0, kk, half, r2) = (s.out("initial_fit")?, s.out("k_per_min")?, s.out("half_life_min")?, s.out("r2")?);
    if d.iter().any(|p| p.1 <= 0.0) || l0 <= 0.0 {
        return None;
    }
    let (t0, t1) = Chart::range(&d.iter().map(|p| p.0).collect::<Vec<_>>());
    let curve: Vec<(f64, f64)> = k::span(t0.min(0.0), t1, 41, false).into_iter().map(|t| (t, l0 * (-kk * t).exp()))
        .collect();
    let ys: Vec<f64> = d.iter().map(|p| p.1).chain(curve.iter().map(|p| p.1)).collect();
    let mut ch = Chart::new((t0.min(0.0), t1), Chart::range_pos(&ys), (false, true), (true, true),
                            "time after the shut-off t (min)", "level (logarithmic)",
                            &format!("half-life = {} min", c::tick(half)));
    ch.line(&curve, "axp-c", "fitted decay", false);
    ch.dots(&d, "axp-d2", 5.0);
    ch.legend.push(("axp-c2".into(), "data as given".into()));
    if half > t0.min(0.0) && half <= t1 {
        ch.vguide(half, &format!("t = {} min", c::tick(half)));
        ch.mark((half, l0 / 2.0), "half the start");
    }
    let body = ch.body();
    let caption = format!("Levels against time as given ({} points, logarithmic axis) and the decay line drawn with the \
                           returned fit: level at t = 0 = {}, k = {} per minute. Returned half-life {} min (marked, \
                           where the line reaches half its start), r2 = {}.", d.len(), c::tick(l0), c::tick(kk),
                          c::tick(half), c::tick(r2));
    Some(s.chart("xy", "mRNA half-life fitted to a decay time course", &caption, &body))
}

/// The double-reciprocal plot: points at (1/S, 1/v) from the data given and the line of the returned slope and
/// intercept, extended to its crossing of the 1/S axis at -1/Km.
fn lineweaver(s: &Solved) -> Option<Figure> {
    let d = pairs(s, "S", "v")?;
    if d.iter().any(|p| p.0 == 0.0 || p.1 == 0.0) {
        return None;
    }
    let (km, vmax, slope, icpt, r2) = (s.out("Km")?, s.out("Vmax")?, s.out("slope")?, s.out("intercept")?, s.out("r2")?);
    let rec: Vec<(f64, f64)> = d.iter().map(|p| (1.0 / p.0, 1.0 / p.1)).collect();
    let xmax = Chart::range(&rec.iter().map(|p| p.0).collect::<Vec<_>>()).1;
    let x_left = if km > 0.0 { -1.0 / km } else { 0.0 };
    let xr = (x_left.min(0.0) * 1.15, xmax * 1.05);
    let line: Vec<(f64, f64)> = k::span(xr.0, xr.1, 2, false).into_iter().map(|x| (x, icpt + slope * x)).collect();
    let ys: Vec<f64> = rec.iter().map(|p| p.1).chain(line.iter().map(|p| p.1)).chain([0.0]).collect();
    let mut ch = Chart::new(xr, Chart::range(&ys), (false, false), (false, true), "1/S", "1/v",
                            &format!("Km = {}, Vmax = {}", c::tick(km), c::tick(vmax)));
    ch.hguide(0.0, "");
    ch.vguide(0.0, "");
    ch.line(&line, "axp-c", "returned line", false);
    ch.dots(&rec, "axp-d2", 5.0);
    ch.legend.push(("axp-c2".into(), "data (1/S, 1/v)".into()));
    ch.mark((0.0, icpt), &format!("1/Vmax = {}", c::tick(icpt)));
    if km > 0.0 {
        ch.dots(&[(-1.0 / km, 0.0)], "axp-d", 5.0);
        let (px, py) = (ch.xa.px(-1.0 / km), ch.ya.px(0.0));
        ch.text(px, py + 18.0, "middle", "axp-vt", &format!("-1/Km = {}", c::tick(-1.0 / km)));
    }
    let body = ch.body();
    let caption = format!("Lineweaver-Burk plot: the {} measurements as points at (1/S, 1/v) and the line of the returned \
                           slope {} and intercept {} (r2 = {}). Marked: 1/Vmax on the 1/v axis and -1/Km where the line \
                           crosses the 1/S axis, from the returned Km = {} and Vmax = {}.", d.len(), c::tick(slope),
                          c::tick(icpt), c::tick(r2), c::tick(km), c::tick(vmax));
    Some(s.chart("xy", "Km and Vmax from a Lineweaver-Burk plot", &caption, &body))
}

/// Rates against substrate as given, with the Michaelis-Menten curve of the fitted Km and Vmax computed by the model's
/// own rate capability at 61 concentrations.
fn mm_fit(s: &Solved) -> Option<Figure> {
    let d = pairs(s, "S", "v")?;
    let (km, vmax, r2) = (s.out("Km")?, s.out("Vmax")?, s.out("r2")?);
    let smax = Chart::range(&d.iter().map(|p| p.0).collect::<Vec<_>>()).1;
    let mut curve = vec![];
    for x in k::span(0.0, smax * 1.1, 61, false) {
        let a = s.solve_other("cmb_michaelis_menten_rate", &[("Vmax", Input::Float(vmax)), ("Km", Input::Float(km)),
                                                             ("S", Input::Float(x))]);
        if let Some(v) = a.as_ref().and_then(|a| k::num_of(a, "v")) {
            curve.push((x, v));
        }
    }
    if curve.len() < 2 {
        return None;
    }
    let ys: Vec<f64> = d.iter().map(|p| p.1).chain(curve.iter().map(|p| p.1)).chain([0.0, vmax]).collect();
    let mut ch = Chart::new((0.0, smax * 1.1), Chart::range(&ys), (false, false), (false, true), "substrate S",
                            "rate v", &format!("Km = {}, Vmax = {}", c::tick(km), c::tick(vmax)));
    ch.hguide(vmax, &format!("Vmax = {}", c::tick(vmax)));
    ch.line(&curve, "axp-c", "fitted curve", false);
    ch.dots(&d, "axp-d2", 5.0);
    ch.legend.push(("axp-c2".into(), "data as given".into()));
    ch.mark((km, vmax / 2.0), &format!("S = Km = {}", c::tick(km)));
    let body = ch.body();
    let caption = format!("The {} measured rates against substrate as given and the Michaelis-Menten curve of the fitted \
                           Km = {} and Vmax = {}, each point computed by the model's rate capability (r2 = {}, residual sum \
                           of squares {}). Marked: S = Km, where the rate is half of Vmax.", d.len(), c::tick(km),
                          c::tick(vmax), c::tick(r2), c::tick(s.out("rss")?));
    Some(s.chart("xy", "Km and Vmax by nonlinear least squares", &caption, &body))
}

/// The Scatchard plot: bound/free against bound from the data given and the line of the returned slope and intercept,
/// reaching the bound axis at Bmax.
fn scatchard(s: &Solved) -> Option<Figure> {
    let (b, f) = (s.vec("B")?, s.vec("F")?);
    if b.len() != f.len() || b.len() < 2 || f.iter().any(|x| *x == 0.0) {
        return None;
    }
    let (kd, bmax, slope, icpt, r2) = (s.out("Kd")?, s.out("Bmax")?, s.out("slope")?, s.out("intercept")?, s.out("r2")?);
    let d: Vec<(f64, f64)> = b.iter().zip(&f).map(|(b, f)| (*b, b / f)).collect();
    let xr = (0.0, Chart::range(&d.iter().map(|p| p.0).chain([bmax]).collect::<Vec<_>>()).1 * 1.05);
    let line: Vec<(f64, f64)> = k::span(xr.0, xr.1, 2, false).into_iter().map(|x| (x, icpt + slope * x)).collect();
    let ys: Vec<f64> = d.iter().map(|p| p.1).chain([0.0, icpt]).collect();
    let mut ch = Chart::new(xr, Chart::range(&ys), (false, false), (false, true), "bound B", "bound / free (B/F)",
                            &format!("Kd = {}, Bmax = {}", c::tick(kd), c::tick(bmax)));
    ch.hguide(0.0, "");
    ch.line(&line, "axp-c", "returned line", false);
    ch.dots(&d, "axp-d2", 5.0);
    ch.legend.push(("axp-c2".into(), "data (B, B/F)".into()));
    ch.mark((bmax, 0.0), &format!("Bmax = {}", c::tick(bmax)));
    let body = ch.body();
    let caption = format!("Scatchard plot: the {} measurements as points at (B, B/F) and the line of the returned slope {} \
                           (= -1/Kd) and intercept {} (= Bmax/Kd), r2 = {}. Returned Kd = {}; marked: Bmax = {}, where the \
                           line meets the bound axis.", b.len(), c::tick(slope), c::tick(icpt), c::tick(r2), c::tick(kd),
                          c::tick(bmax));
    Some(s.chart("xy", "Kd and Bmax from a Scatchard plot", &caption, &body))
}

// ------------------------------------------------------------------------------------------------ sequences
/// Parts of an input/result drawing, top to bottom.
enum Part {
    Head(String),
    Rows(Vec<Strip>, usize),
    Notes(Vec<(String, String)>),
    Codons(Vec<(String, char)>),
}

/// An input-and-result drawing: what was given, an arrow, what the model returned (kind "io").
fn io_fig(s: &Solved, title: &str, caption: &str, parts: Vec<Part>) -> Option<Figure> {
    let mut out = String::new();
    let mut y = 10.0;
    for p in parts {
        match p {
            Part::Head(t) => {
                y += 14.0;
                out.push_str(&format!("<text class=\"axp-bt\" x=\"16\" y=\"{y:.1}\">{}</text>", c::esc(&t)));
                y += 10.0;
            }
            Part::Rows(rows, per) => {
                let (b, y2) = c::strips(&rows, y, per);
                out.push_str(&b);
                y = y2;
            }
            Part::Notes(lines) => {
                let (b, y2) = c::notes(&lines, y + 12.0);
                out.push_str(&b);
                y = y2 - 6.0;
            }
            Part::Codons(cs) => {
                let per = 12;
                let w = ((520.0 - 32.0) / per as f64).min(40.0);
                for (i, (codon, aa)) in cs.iter().enumerate() {
                    let (col, row) = (i % per, i / per);
                    let x = 16.0 + w * col as f64;
                    let yy = y + row as f64 * 50.0;
                    let class = if *aa == '*' { "axp-b4" } else if i % 2 == 0 { "axp-b" } else { "axp-b3" };
                    out.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{yy:.1}\" width=\"{:.1}\" height=\"20\"/>\
                                           <text class=\"axp-t\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11\">{}</text>\
                                           <text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                          w - 3.0, x + (w - 3.0) / 2.0, yy + 14.0, c::esc(codon), x + (w - 3.0) / 2.0,
                                          yy + 38.0, aa));
                }
                y += cs.len().div_ceil(per) as f64 * 50.0 + 4.0;
            }
        }
    }
    Some(s.figure("io", title, caption, &out, (y + 10.0).max(90.0)))
}

fn strip(label: &str, seq: &[char], class: impl Fn(usize, char) -> &'static str) -> Strip {
    Strip { label: label.to_string(),
            cells: seq.iter().enumerate().map(|(i, ch)| (ch.to_string(), class(i, *ch).to_string())).collect() }
}

fn is_gc(ch: char) -> bool {
    matches!(ch.to_ascii_uppercase(), 'G' | 'C')
}

/// Letters per row for a sequence of n letters (rows of up to 20, evened out).
fn per_row(n: usize) -> usize {
    let rows = n.div_ceil(20).max(1);
    n.div_ceil(rows).max(1)
}

fn vals(s: &Solved, syms: &[&str]) -> Vec<(String, String)> {
    syms.iter().filter_map(|y| s.out_text(y).map(|v| (format!("{y} ="), v))).collect()
}

fn io_gc(s: &Solved) -> Option<Figure> {
    let seq = c::letters(&s.txt("seq")?, 120)?;
    let (g, cc) = (s.out("G")?, s.out("C")?);
    let shaded = seq.iter().filter(|c| is_gc(**c)).count() as f64;
    if shaded != g + cc {
        return None;
    }
    let caption = format!("The sequence as given, base by base, G and C shaded ({} of {} bases); the model returns \
                           gc_fraction = {}, gc_percent = {} and the counts G = {}, C = {}, A = {}, T or U = {}.",
                          shaded, seq.len(), s.out_text("gc_fraction")?, s.out_text("gc_percent")?, g, cc,
                          s.out_text("A")?, s.out_text("T_or_U")?);
    io_fig(s, "GC content of a DNA or RNA sequence", &caption, vec![
        Part::Head("Given: the sequence (G and C shaded)".into()),
        Part::Rows(vec![strip("seq", &seq, |_, ch| if is_gc(ch) { "axp-b" } else { "" })], per_row(seq.len())),
        Part::Head("Returned".into()),
        Part::Notes(vals(s, &["gc_fraction", "gc_percent", "length", "G", "C", "A", "T_or_U"])),
    ])
}

fn io_extinction(s: &Solved) -> Option<Figure> {
    let seq = c::letters(&s.txt("seq")?, 120)?;
    let caption = format!("The oligonucleotide as given ({} bases, each nearest-neighbour pair counts toward the extinction \
                           coefficient); the model returns eps = {} m^2/mol ({} L/(mol cm)), {} ug and {} nmol per A260 unit.",
                          seq.len(), s.out_text("eps")?, s.out_text("eps_L_mol_cm")?, s.out_text("ug_per_A260")?,
                          s.out_text("nmol_per_A260")?);
    io_fig(s, "Extinction coefficient of a DNA oligonucleotide at 260 nm", &caption, vec![
        Part::Head("Given: the oligonucleotide 5' to 3'".into()),
        Part::Rows(vec![strip("seq", &seq, |_, _| "")], per_row(seq.len())),
        Part::Head("Returned".into()),
        Part::Notes(vals(s, &["eps", "eps_L_mol_cm", "ug_per_A260", "nmol_per_A260"])),
    ])
}

fn io_orf(s: &Solved) -> Option<Figure> {
    let seq_t = s.txt("seq")?;
    let seq = c::letters(&seq_t, 150)?;
    let strand = s.out_text("strand")?;
    let (st, en, nt) = (s.out("start")?, s.out("end")?, s.out("length_nt")?);
    if en - st + 1.0 != nt || st < 1.0 || en as usize > seq.len() {
        return None;
    }
    // the strand the frame lies on: the given one, or its reverse complement as the model writes it
    let (label, shown) = if strand == "+" { ("+ strand", seq.clone()) } else {
        let a = s.solve_other("cmb_reverse_complement", &[("seq", Input::Text(seq_t.clone()))])?;
        let rc = a.values.iter().find(|v| v.0 == "reverse_complement")?.1.as_str()?.to_string();
        ("- strand", c::letters(&rc, 150)?)
    };
    if shown.len() != seq.len() {
        return None;
    }
    let (a, b) = (st as usize - 1, en as usize - 1);
    let prot = c::letters(&s.out_text("protein")?, 60)?;
    let caption = format!("The sequence as given ({} bases){}: the longest open reading frame the model finds is shaded, \
                           from base {} to base {} on the {} strand ({} nt with the stop codon, start and stop in the \
                           second colour), and its protein below ({} amino acids). Open reading frames of at least the \
                           minimum length: {}.", seq.len(), if strand == "+" { "" } else { ", shown as its reverse complement" },
                          s.out_text("start")?, s.out_text("end")?, strand, s.out_text("length_nt")?,
                          s.out_text("length_aa")?, s.out_text("n_orfs")?);
    let per = per_row(shown.len());
    io_fig(s, "Longest open reading frame on both strands", &caption, vec![
        Part::Head(format!("Given: the sequence ({label} shown 5' to 3'), the frame shaded")),
        Part::Rows(vec![strip(label, &shown, |i, _| {
            if i < a || i > b { "" } else if i < a + 3 || i + 3 > b { "axp-b2" } else { "axp-b" }
        })], per),
        Part::Head("Returned: the protein".into()),
        Part::Rows(vec![strip("protein", &prot, |_, _| "axp-b3")], per_row(prot.len())),
        Part::Notes(vals(s, &["strand", "start", "end", "length_nt", "length_aa", "n_orfs"])),
    ])
}

fn pair_class(_: usize, ch: char) -> &'static str {
    match ch.to_ascii_uppercase() {
        'G' | 'C' => "axp-b2",
        _ => "axp-b",
    }
}

fn io_revcomp(s: &Solved) -> Option<Figure> {
    let seq = c::letters(&s.txt("seq")?, 90)?;
    let comp = c::letters(&s.out_text("complement")?, 90)?;
    let rc = c::letters(&s.out_text("reverse_complement")?, 90)?;
    if comp.len() != seq.len() || rc.len() != seq.len() {
        return None;
    }
    let per = per_row(seq.len());
    let caption = format!("The strand as given (5' to 3') with the complement the model returns under it (3' to 5'), A-T \
                           (or A-U) pairs and G-C pairs in two colours; read 5' to 3', the partner strand is the reverse \
                           complement {}.", rc.iter().collect::<String>());
    io_fig(s, "Reverse complement: the partner strand of a DNA or RNA sequence", &caption, vec![
        Part::Head("Given strand 5' to 3', and the complement returned (3' to 5')".into()),
        Part::Rows(vec![strip("given 5'", &seq, pair_class), strip("compl. 3'", &comp, pair_class)], per),
        Part::Head("Returned: the reverse complement, 5' to 3'".into()),
        Part::Rows(vec![strip("revcomp 5'", &rc, pair_class)], per),
    ])
}

fn io_tm(s: &Solved, tm: &str, title: &str) -> Option<Figure> {
    let seq = c::letters(&s.txt("seq")?, 120)?;
    let shaded = seq.iter().filter(|c| is_gc(**c)).count() as f64;
    // the shading must agree with the model's own count
    let ok = match (s.out("GC"), s.out("gc_percent"), s.out("length")) {
        (Some(gc), _, _) => gc == shaded,
        (None, Some(p), Some(n)) => (p * n / 100.0 - shaded).abs() < 1e-6,
        _ => false,
    };
    if !ok {
        return None;
    }
    let syms: Vec<&str> = s.a.values.iter().map(|v| v.0.as_str()).collect();
    let caption = format!("The oligonucleotide as given, G and C shaded ({} of {} bases); the model returns {}.",
                          shaded, seq.len(),
                          s.a.values.iter().filter_map(|v| s.out_text(&v.0).map(|t| format!("{} = {t}", v.0)))
                              .collect::<Vec<_>>().join(", "));
    io_fig(s, title, &caption, vec![
        Part::Head("Given: the oligonucleotide (G and C shaded)".into()),
        Part::Rows(vec![strip("seq", &seq, |_, ch| if is_gc(ch) { "axp-b" } else { "" })], per_row(seq.len())),
        Part::Head(format!("Returned: {tm} = {} degC", s.out_text(tm)?)),
        Part::Notes(vals(s, &syms)),
    ])
}

fn io_transcribe(s: &Solved) -> Option<Figure> {
    let seq = c::letters(&s.txt("seq")?, 90)?;
    let strand = s.txt("strand")?;
    let m = c::letters(&s.out_text("mRNA")?, 90)?;
    if m.len() != seq.len() {
        return None;
    }
    let per = per_row(seq.len());
    let caption = format!("The DNA as given, read as the {strand} strand, and the mRNA the model returns (5' to 3'), T in the \
                           DNA and U in the mRNA shaded: mRNA = {}, length {}.", m.iter().collect::<String>(),
                          s.out_text("length")?);
    io_fig(s, "Transcription of a DNA strand into mRNA", &caption, vec![
        Part::Head(format!("Given: the DNA, the {strand} strand")),
        Part::Rows(vec![strip("DNA", &seq, |_, ch| if ch.to_ascii_uppercase() == 'T' { "axp-b4" } else { "" })], per),
        Part::Head("Returned: the mRNA 5' to 3'".into()),
        Part::Rows(vec![strip("mRNA", &m, |_, ch| if ch.to_ascii_uppercase() == 'U' { "axp-b" } else { "" })], per),
        Part::Notes(vals(s, &["length"])),
    ])
}

fn io_translate(s: &Solved) -> Option<Figure> {
    let seq = c::letters(&s.txt("seq")?, 180)?;
    let full: Vec<char> = s.out_text("full_translation")?.chars().collect();
    let n = seq.len() / 3;
    if full.len() != n || n == 0 || n > 48 {
        return None;
    }
    let codons: Vec<(String, char)> = (0..n).map(|i| (seq[3 * i..3 * i + 3].iter().collect::<String>(), full[i])).collect();
    let caption = format!("The sequence as given read in its {n} codons, each with the amino acid the model assigns under it \
                           (* a stop codon): full translation {}, protein up to the first stop {} ({} amino acids), {} stop \
                           codons.", full.iter().collect::<String>(), s.out_text("protein")?, s.out_text("length_aa")?,
                          s.out_text("stops")?);
    io_fig(s, "Translation of an mRNA or DNA sequence into protein", &caption, vec![
        Part::Head("Given: the sequence in codons; returned: the amino acid of each".into()),
        Part::Codons(codons),
        Part::Notes(vals(s, &["protein", "full_translation", "length_aa", "stops"])),
    ])
}
