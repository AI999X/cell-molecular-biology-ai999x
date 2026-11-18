//! Cell and Molecular Biology: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p cell-molecular-biology-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&cell_molecular_biology_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "cmb_monod_batch_time", "inputs": {"X0": "0.1 g/L", "S0": "10 g/L", "Y": "0.5", "mu_max": "0.72 1/h", "Ks": "0.2 g/L", "X": "4 g/L"}}));
    println!();
    show(json!({"key": "cmb_absorbance_ratios", "inputs": {"A260": "0.9", "A280": "0.5", "A230": "0.42"}}));
    println!();
    show(json!({"key": "cmb_competitive_inhibition", "inputs": {"Vmax": "6 (umol/L)/min", "Km": "50 (umol/L)", "S": "100 (umol/L)", "I": "20 (umol/L)", "Ki": "10 (umol/L)"}}));
    println!();
    show(json!({"key": "cmb_ligand_depletion_binding", "inputs": {"Rt": "10 (nmol/L)", "Lt": "20 (nmol/L)", "Kd": "10 (nmol/L)"}}));
    println!();
    show(json!({"key": "cmb_cell_volume_osmotic", "inputs": {"V0": "90 fL", "b": "0.4", "osm0": "300 mmol/L", "osm1": "600 mmol/L"}}));
    println!();
    show(json!({"key": "cmb_atp_yield", "inputs": {"P_O_NADH": "2.5", "P_O_FADH2": "1.5", "shuttle": "malate-aspartate", "dG_ATP": "30.5 kJ/mol", "dG_glucose": "2870 kJ/mol"}}));
    println!();
    show(json!({"key": "cmb_decimal_reduction_time", "inputs": {"N0": "1000000", "N": "100", "t": "10 min"}}));
    println!();
    show(json!({"key": "cmb_specimen_size", "inputs": {"image_size": "12 mm", "M": "400"}}));
    println!();
    show(json!({"key": "cmb_induction_dynamics", "inputs": {"k_tx": "36 1/h", "delta_m": "7.2 1/h", "k_tl": "180 1/h", "delta_p": "0.36 1/h", "t": "2 h"}}));
    println!();
    show(json!({"key": "cmb_buffer_capacity", "inputs": {"C": "100 (mmol/L)", "pKa": "7.21", "pH": "7.21"}}));
    println!();
    show(json!({"key": "cmb_hemocytometer_count", "inputs": {"counts": [52, 48, 55, 45], "dilution": "2"}}));
    println!();
    show(json!({"key": "cmb_tm_basic", "inputs": {"seq": "AGCGGATAACAATTTCACACAGGA"}}));
    println!();
    show(json!({"key": "cmb_enzyme_assay_activity", "inputs": {"dA": "0.12", "dt": "1 min", "eps": "6220 L/(mol*cm)", "l": "1 cm", "V_assay": "1 mL", "V_enzyme": "20 uL", "protein": "0.5 mg/mL"}}));
    println!();
    show(json!({"key": "cmb_kd_from_rates", "inputs": {"kon": "1000000 1/((mol/L)*s)", "koff": "0.001 1/s"}}));
    println!();
    show(json!({"key": "cmb_frap_diffusion", "inputs": {"w": "1 um", "t_half": "0.5 s"}}));
}
