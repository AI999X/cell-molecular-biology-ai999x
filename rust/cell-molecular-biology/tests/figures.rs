//! Every Cell Molecular Biology capability's own live figures (src/figures.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text outside
//! its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use cell_molecular_biology_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/cell-molecular-biology-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("cmb_binding_free_energy", &["field"]),
    ("cmb_centrifuge_rcf", &["field"]),
    ("cmb_centrifuge_rpm", &["field"]),
    ("cmb_dna_copy_number", &["field"]),
    ("cmb_doubling_time_from_counts", &["td-vs-n0"]),
    ("cmb_fraction_bound", &["field"]),
    ("cmb_frap_diffusion", &["field"]),
    ("cmb_gc_content", &["io"]),
    ("cmb_growth_rate_fit", &["xy"]),
    ("cmb_hill_equation", &["theta-vs-l"]),
    ("cmb_ionic_strength", &["sensitivity"]),
    ("cmb_kd_from_rates", &["field"]),
    ("cmb_lineweaver_burk_fit", &["xy"]),
    ("cmb_michaelis_menten_nonlinear_fit", &["xy"]),
    ("cmb_molecules_per_cell", &["field"]),
    ("cmb_mrna_half_life_fit", &["xy"]),
    ("cmb_nernst_potential", &["e-vs-c-out"]),
    ("cmb_nucleic_acid_a260", &["field"]),
    ("cmb_oligo_extinction_coefficient", &["io"]),
    ("cmb_oligo_molecular_weight", &["bars"]),
    ("cmb_orf_finder", &["io"]),
    ("cmb_osmolarity_tonicity", &["decision"]),
    ("cmb_percent_wv_to_molarity", &["field"]),
    ("cmb_qpcr_delta_delta_ct", &["fold-change-vs-ct-target-treated"]),
    ("cmb_qpcr_standard_curve", &["sensitivity"]),
    ("cmb_reverse_complement", &["io"]),
    ("cmb_scatchard_fit", &["xy"]),
    ("cmb_serial_dilution", &["series"]),
    ("cmb_substrate_for_fraction_of_vmax", &["field"]),
    ("cmb_tm_basic", &["io"]),
    ("cmb_tm_wallace", &["io"]),
    ("cmb_transcribe", &["io"]),
    ("cmb_transcription_time", &["field"]),
    ("cmb_translate", &["io"]),
    ("cmb_translation_time", &["field"]),
    ("cmb_warburg_christian", &["field"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("cmb_gc_content", "seq", "ATGCATTTGCGAAT"),
    ("cmb_oligo_extinction_coefficient", "seq", "GGCATGCATT"),
    ("cmb_orf_finder", "seq", "CCTCAGCCCGCACCGTTACCTGTGGTAATGGTGATGGTGGTGGTAATGGTGGTGCTAATGCGTTTCATGG"),
    ("cmb_reverse_complement", "seq", "ATGCGTACCAAA"),
    ("cmb_tm_basic", "seq", "AGCGGATAACAATTTCACACAGGAGG"),
    ("cmb_tm_wallace", "seq", "ACGTTGCAATGCCGTAGG"),
    ("cmb_transcribe", "strand", "template"),
    ("cmb_translate", "seq", "AUGGCCAUUGUAAUGGGCCGCUGA"),
    ("cmb_oligo_molecular_weight", "seq", "ATGCATGCATGG"),
    ("cmb_ionic_strength", "c", "[100, 100]"),
    ("cmb_qpcr_standard_curve", "Ct", "[15.0, 18.4, 21.9, 25.1, 28.6]"),
    ("cmb_osmolarity_tonicity", "c", "[120]"),
    ("cmb_growth_rate_fit", "N", "[1000, 1900, 3900, 7600, 15100]"),
    ("cmb_mrna_half_life_fit", "level", "[100, 75, 55, 41, 30]"),
    ("cmb_lineweaver_burk_fit", "v", "[15, 27, 48, 66, 79, 90]"),
    ("cmb_michaelis_menten_nonlinear_fit", "v", "[15, 27, 48, 66, 79, 90]"),
    ("cmb_scatchard_fit", "B", "[15, 27, 48, 66, 79]"),
];

/// A capability refused by the model for these inputs gives no figure (capability, inputs).
const REFUSED: (&str, &[(&str, &str)]) = ("cmb_gc_content", &[("seq", "")]);


/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else if cs[i] == '[' {
            let s0 = i;
            let mut depth = 0;
            loop {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                i += 1;
            }
            i += 1;
            value = cs[s0..i].iter().collect();
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        assert!(ax_engine::solve(p, key, &given, None).solved(), "{key}: the worked example is not solved");
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// The numbers of a text multiplied by f ("12 V" -> "16.44 V"; "[3, 6]" -> "[4.11, 8.22]"; "8e-05 m^3/s" ->
/// "0.0001096 m^3/s"): a number that follows a letter or "^" (a unit's power) is kept.
fn scale_text(t: &str, f: f64) -> Option<String> {
    let cs: Vec<char> = t.chars().collect();
    let mut out = String::new();
    let mut changed = false;
    let mut i = 0;
    while i < cs.len() {
        let prev = if i == 0 { ' ' } else { cs[i - 1] };
        let starts = cs[i].is_ascii_digit() || (cs[i] == '-' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit());
        if starts && !(prev.is_alphanumeric() || prev == '^' || prev == '.' || prev == '_') {
            let s0 = i;
            i += 1;
            while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
                i += 1;
            }
            if i + 1 < cs.len() && (cs[i] == 'e' || cs[i] == 'E')
                && (cs[i + 1].is_ascii_digit() || ((cs[i + 1] == '-' || cs[i + 1] == '+') && i + 2 < cs.len())) {
                i += 2;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let tok: String = cs[s0..i].iter().collect();
            match tok.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    changed = true;
                }
                _ => out.push_str(&tok),
            }
        } else {
            out.push(cs[i]);
            i += 1;
        }
    }
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        for (_, sym, val) in OTHER.iter().filter(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73, 2.0] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    let p = pack();
    let (key, inputs) = REFUSED;
    let given: Vec<(String, Input)> = inputs.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, key, &given, None);
    assert!(!a.solved(), "{key}: expected a refusal");
    assert!(figures(p, key, &given, None, None).is_empty());
}

/// Writes every worked example's own figures (and those of the OTHER inputs, prefixed "x-") as SVG files into
/// $FIG_DUMP, for a contact sheet: cargo test --test figures -- --ignored dump
#[test]
#[ignore]
fn dump() {
    let Ok(dir) = std::env::var("FIG_DUMP") else { return };
    let p = pack();
    for (key, _) in KINDS {
        let given = example(key);
        for f in figures(p, key, &given, None, None).iter().filter(|f| !f.slug.contains("-live-")) {
            std::fs::write(format!("{dir}/{}.svg", f.slug), &f.svg).unwrap();
        }
        for (_, sym, val) in OTHER.iter().filter(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            for f in figures(p, key, &g, None, None).iter().filter(|f| !f.slug.contains("-live-")) {
                std::fs::write(format!("{dir}/x-{}.svg", f.slug), &f.svg).unwrap();
            }
        }
    }
}
