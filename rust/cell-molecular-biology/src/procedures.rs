//! cell_molecular_biology: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/cell_molecular_biology/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "cmb_absorbance_ratios",
    "cmb_binding_free_energy",
    "cmb_buffer_after_addition",
    "cmb_buffer_capacity",
    "cmb_buffer_preparation",
    "cmb_c1v1_dilution",
    "cmb_cell_cycle_age_corrected",
    "cmb_cell_cycle_phase_durations",
    "cmb_cell_equilibration",
    "cmb_cell_geometry",
    "cmb_cell_volume_osmotic",
    "cmb_centrifuge_rcf",
    "cmb_centrifuge_rpm",
    "cmb_channel_current",
    "cmb_competitive_inhibition",
    "cmb_competitive_occupancy",
    "cmb_diffusion_limited_rate",
    "cmb_diffusion_time",
    "cmb_dna_copy_number",
    "cmb_donnan_equilibrium",
    "cmb_doubling_time_from_counts",
    "cmb_enzyme_assay_activity",
    "cmb_exponential_growth",
    "cmb_expression_steady_state",
    "cmb_fick_flux",
    "cmb_fraction_bound",
    "cmb_frap_diffusion",
    "cmb_gc_content",
    "cmb_genome_coverage",
    "cmb_growth_rate_fit",
    "cmb_hemocytometer_count",
    "cmb_henderson_hasselbalch",
    "cmb_hill_coefficient_from_ec",
    "cmb_hill_equation",
    "cmb_hill_ligand_for_fraction",
    "cmb_ic50_to_ki",
    "cmb_induction_dynamics",
    "cmb_ionic_strength",
    "cmb_kcat_catalytic_efficiency",
    "cmb_kd_from_rates",
    "cmb_library_size",
    "cmb_ligand_depletion_binding",
    "cmb_ligation_insert_mass",
    "cmb_lineweaver_burk_fit",
    "cmb_logistic_growth",
    "cmb_logistic_time_to_reach",
    "cmb_mass_for_molarity",
    "cmb_membrane_permeability",
    "cmb_michaelis_menten_nonlinear_fit",
    "cmb_michaelis_menten_rate",
    "cmb_mixed_inhibition",
    "cmb_molarity_from_mass",
    "cmb_molecules_per_cell",
    "cmb_monod_batch_time",
    "cmb_monod_growth_rate",
    "cmb_mrna_half_life_fit",
    "cmb_mutation_rate_p0",
    "cmb_mwc_saturation",
    "cmb_nernst_potential",
    "cmb_noncompetitive_inhibition",
    "cmb_nucleic_acid_a260",
    "cmb_oligo_extinction_coefficient",
    "cmb_oligo_molecular_weight",
    "cmb_orf_finder",
    "cmb_osmolarity_tonicity",
    "cmb_osmotic_pressure",
    "cmb_pcr_amplification",
    "cmb_pcr_cycles_needed",
    "cmb_percent_wv_to_molarity",
    "cmb_plate_count",
    "cmb_protein_concentration_a280",
    "cmb_q10_temperature",
    "cmb_qpcr_delta_delta_ct",
    "cmb_qpcr_pfaffl_ratio",
    "cmb_qpcr_quantify",
    "cmb_qpcr_standard_curve",
    "cmb_reverse_complement",
    "cmb_rms_displacement",
    "cmb_scatchard_fit",
    "cmb_sedimentation_time",
    "cmb_serial_dilution",
    "cmb_stokes_einstein",
    "cmb_substrate_for_fraction_of_vmax",
    "cmb_tm_basic",
    "cmb_tm_nearest_neighbor",
    "cmb_tm_salt_adjusted",
    "cmb_tm_wallace",
    "cmb_transcribe",
    "cmb_transcription_time",
    "cmb_transformation_efficiency",
    "cmb_translate",
    "cmb_translation_time",
    "cmb_uncompetitive_inhibition",
    "cmb_warburg_christian",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "cmb_absorbance_ratios" => {
            let r = f_absorbance_ratios(match v.get("A260") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A280") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A230") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("A320") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_34(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_binding_free_energy" => {
            let r = f_binding_free_energy(match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_78(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_buffer_after_addition" => {
            let r = f_buffer_after_addition(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pH0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pKa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_add") { Some(x) => py::from_val(x), None => { return None } }, match v.get("added") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_133(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_buffer_capacity" => {
            let r = f_buffer_capacity(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pKa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_178(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_buffer_preparation" => {
            let r = f_buffer_preparation(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pKa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW_acid") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("MW_base") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_228(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_c1v1_dilution" => {
            let r = f_c1v1_dilution(match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_274(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_cycle_age_corrected" => {
            let r = f_cell_cycle_age_corrected(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fractions") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_330(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_cycle_phase_durations" => {
            let r = f_cell_cycle_simple(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fractions") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_383(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_equilibration" => {
            let r = f_cell_equilibration(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_431(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_geometry" => {
            let r = f_cell_geometry(match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_475(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_volume_osmotic" => {
            let r = f_cell_volume_osmotic(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("osm0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("osm1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_528(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_centrifuge_rcf" => {
            let r = f_centrifuge_rcf(match v.get("rpm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_572(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_centrifuge_rpm" => {
            let r = f_centrifuge_rpm(match v.get("RCF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_616(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_channel_current" => {
            let r = f_channel_current(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Po") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_673(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_competitive_inhibition" => {
            let r = s_699(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_731(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_competitive_occupancy" => {
            let r = f_competitive_occupancy(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_779(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_diffusion_limited_rate" => {
            let r = f_smoluchowski_rate(match v.get("DA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("DB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rB") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_823(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_diffusion_time" => {
            let r = f_diffusion_time(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dims") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_869(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_dna_copy_number" => {
            let r = f_dna_copy_number(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("molecule") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_919(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_donnan_equilibrium" => {
            let r = f_donnan_equilibrium(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_967(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_doubling_time_from_counts" => {
            let r = f_doubling_from_counts(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1013(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_enzyme_assay_activity" => {
            let r = f_enzyme_assay_activity(match v.get("dA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_assay") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_enzyme") { Some(x) => py::from_val(x), None => { return None } }, match v.get("protein") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1065(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_exponential_growth" => {
            let r = f_exponential_growth(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("td") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1113(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_expression_steady_state" => {
            let r = f_expression_steady_state(match v.get("k_tx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_tl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta_p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1157(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_fick_flux" => {
            let r = f_fick_flux(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dx") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1205(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_fraction_bound" => {
            let r = f_fraction_bound(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rt") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1252(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_frap_diffusion" => {
            let r = f_frap_diffusion(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_half") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1331(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_gc_content" => {
            let r = f_gc_content(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1386(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_genome_coverage" => {
            let r = f_genome_coverage(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1437(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_growth_rate_fit" => {
            let r = f_growth_rate_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1501(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_hemocytometer_count" => {
            let r = f_hemocytometer(match v.get("counts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dilution") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1558(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_henderson_hasselbalch" => {
            let r = f_henderson_hasselbalch(match v.get("pKa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HA") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1603(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_hill_coefficient_from_ec" => {
            let r = f_hill_from_ec(match v.get("EC10") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EC90") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1649(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_hill_equation" => {
            let r = f_hill_equation(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1693(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_hill_ligand_for_fraction" => {
            let r = f_hill_ligand_for_fraction(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1746(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_ic50_to_ki" => {
            let r = f_ic50_to_ki(match v.get("mode") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IC50") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1796(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_induction_dynamics" => {
            let r = f_induction_dynamics(match v.get("k_tx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_tl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1846(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_ionic_strength" => {
            let r = f_ionic_strength(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1903(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_kcat_catalytic_efficiency" => {
            let r = f_kcat_efficiency(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Et") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1947(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_kd_from_rates" => {
            let r = f_kd_from_rates(match v.get("kon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("koff") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1991(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_library_size" => {
            let r = f_library_size(match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2046(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_ligand_depletion_binding" => {
            let r = f_ligand_depletion_binding(match v.get("Rt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2090(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_ligation_insert_mass" => {
            let r = f_ligation_insert_mass(match v.get("m_vector") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L_vector") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L_insert") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2141(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_lineweaver_burk_fit" => {
            let r = f_lineweaver_burk(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2211(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_logistic_growth" => {
            let r = f_logistic_growth(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2261(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_logistic_time_to_reach" => {
            let r = f_logistic_time_to_reach(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2308(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mass_for_molarity" => {
            let r = f_mass_for_molarity(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2352(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_membrane_permeability" => {
            let r = f_membrane_permeability(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2400(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_michaelis_menten_nonlinear_fit" => {
            let r = f_mm_nonlinear_fit(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2504(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_michaelis_menten_rate" => {
            let r = f_michaelis_menten(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2552(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mixed_inhibition" => {
            let r = f_mixed_inhibition(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kii") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2601(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_molarity_from_mass" => {
            let r = f_molarity_from_mass(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2645(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_molecules_per_cell" => {
            let r = f_molecules_per_cell(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2689(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_monod_batch_time" => {
            let r = f_monod_batch_time(match v.get("X0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2736(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_monod_growth_rate" => {
            let r = f_monod_growth(match v.get("mu_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ks") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2787(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mrna_half_life_fit" => {
            let r = f_mrna_half_life_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2849(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mutation_rate_p0" => {
            let r = f_mutation_rate_p0(match v.get("n0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2901(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mwc_saturation" => {
            let r = f_mwc_saturation(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("KR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2949(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_nernst_potential" => {
            let r = f_nernst_potential(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2997(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_noncompetitive_inhibition" => {
            let r = s_3023(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3055(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_nucleic_acid_a260" => {
            let r = f_nucleic_acid_a260(match v.get("A260") { Some(x) => py::from_val(x), None => { return None } }, match v.get("molecule") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dilution") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("l") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.01)) })().ok()? } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3111(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_oligo_extinction_coefficient" => {
            let r = f_oligo_extinction(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3170(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_oligo_molecular_weight" => {
            let r = f_oligo_molecular_weight(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strands") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3230(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_orf_finder" => {
            let r = f_orf_finder(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("min_aa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3320(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_osmolarity_tonicity" => {
            let r = f_osmolarity_tonicity(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("osm_cell") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3382(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_osmotic_pressure" => {
            let r = f_osmotic_pressure(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3426(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_pcr_amplification" => {
            let r = f_pcr_amplification(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3483(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_pcr_cycles_needed" => {
            let r = f_pcr_cycles_needed(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3540(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_percent_wv_to_molarity" => {
            let r = f_percent_to_molarity(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_plate_count" => {
            let r = f_cfu_count(match v.get("colonies") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dilution") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3645(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_protein_concentration_a280" => {
            let r = f_protein_a280(match v.get("A280") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("l") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.01)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3696(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_q10_temperature" => {
            let r = f_q10_rate(match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q10") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3742(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_qpcr_delta_delta_ct" => {
            let r = f_qpcr_ddct(match v.get("Ct_target_treated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_ref_treated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_target_control") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_ref_control") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3787(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_qpcr_pfaffl_ratio" => {
            let r = f_qpcr_pfaffl(match v.get("E_target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_target_control") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_target_sample") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_ref_control") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_ref_sample") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3841(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_qpcr_quantify" => {
            let r = f_qpcr_quantify(match v.get("slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3885(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_qpcr_standard_curve" => {
            let r = f_qpcr_standard_curve(match v.get("quantity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3947(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_reverse_complement" => {
            let r = f_reverse_complement(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4004(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_rms_displacement" => {
            let r = f_rms_displacement(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dims") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4050(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_scatchard_fit" => {
            let r = f_scatchard_fit(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4116(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_sedimentation_time" => {
            let r = f_sedimentation_time(match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rpm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r_min") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r_max") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4162(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_serial_dilution" => {
            let r = f_serial_dilution(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4217(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_stokes_einstein" => {
            let r = f_stokes_einstein(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4261(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_substrate_for_fraction_of_vmax" => {
            let r = f_substrate_for_fraction(match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4314(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_tm_basic" => {
            let r = f_tm_basic(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4370(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_tm_nearest_neighbor" => {
            let r = f_tm_nearest_neighbor(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4447(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_tm_salt_adjusted" => {
            let r = f_tm_salt_adjusted(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4509(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_tm_wallace" => {
            let r = f_tm_wallace(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4566(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_transcribe" => {
            let r = f_transcribe(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strand") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4628(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_transcription_time" => {
            let r = f_transcription_time(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_init") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4678(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_transformation_efficiency" => {
            let r = f_transformation_efficiency(match v.get("colonies") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_DNA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_plated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_total") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4729(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_translate" => {
            let r = f_translate(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4788(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_translation_time" => {
            let r = f_translation_time(match v.get("n_aa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_init") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4838(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_uncompetitive_inhibition" => {
            let r = s_4864(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4896(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_warburg_christian" => {
            let r = f_warburg_christian(match v.get("A280") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A260") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4944(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_FARADAY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(96485.33212331001)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_G0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(9.80665)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_HOUR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3600.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_K_B() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.380649e-23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LN2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.6931471805599453)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MINUTE() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(60.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_N_A() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(6.02214076e+23)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_RPM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.10471975511965977)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_GAS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.31446261815324)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__A260_FACTOR() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("dsDNA"), Py::Float(50.0)), (Py::str("ssDNA"), Py::Float(33.0)), (Py::str("RNA"), Py::Float(40.0))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__CODONS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("TTT"), Py::str("F")), (Py::str("TTC"), Py::str("F")), (Py::str("TTA"), Py::str("L")), (Py::str("TTG"), Py::str("L")), (Py::str("TCT"), Py::str("S")), (Py::str("TCC"), Py::str("S")), (Py::str("TCA"), Py::str("S")), (Py::str("TCG"), Py::str("S")), (Py::str("TAT"), Py::str("Y")), (Py::str("TAC"), Py::str("Y")), (Py::str("TAA"), Py::str("*")), (Py::str("TAG"), Py::str("*")), (Py::str("TGT"), Py::str("C")), (Py::str("TGC"), Py::str("C")), (Py::str("TGA"), Py::str("*")), (Py::str("TGG"), Py::str("W")), (Py::str("CTT"), Py::str("L")), (Py::str("CTC"), Py::str("L")), (Py::str("CTA"), Py::str("L")), (Py::str("CTG"), Py::str("L")), (Py::str("CCT"), Py::str("P")), (Py::str("CCC"), Py::str("P")), (Py::str("CCA"), Py::str("P")), (Py::str("CCG"), Py::str("P")), (Py::str("CAT"), Py::str("H")), (Py::str("CAC"), Py::str("H")), (Py::str("CAA"), Py::str("Q")), (Py::str("CAG"), Py::str("Q")), (Py::str("CGT"), Py::str("R")), (Py::str("CGC"), Py::str("R")), (Py::str("CGA"), Py::str("R")), (Py::str("CGG"), Py::str("R")), (Py::str("ATT"), Py::str("I")), (Py::str("ATC"), Py::str("I")), (Py::str("ATA"), Py::str("I")), (Py::str("ATG"), Py::str("M")), (Py::str("ACT"), Py::str("T")), (Py::str("ACC"), Py::str("T")), (Py::str("ACA"), Py::str("T")), (Py::str("ACG"), Py::str("T")), (Py::str("AAT"), Py::str("N")), (Py::str("AAC"), Py::str("N")), (Py::str("AAA"), Py::str("K")), (Py::str("AAG"), Py::str("K")), (Py::str("AGT"), Py::str("S")), (Py::str("AGC"), Py::str("S")), (Py::str("AGA"), Py::str("R")), (Py::str("AGG"), Py::str("R")), (Py::str("GTT"), Py::str("V")), (Py::str("GTC"), Py::str("V")), (Py::str("GTA"), Py::str("V")), (Py::str("GTG"), Py::str("V")), (Py::str("GCT"), Py::str("A")), (Py::str("GCC"), Py::str("A")), (Py::str("GCA"), Py::str("A")), (Py::str("GCG"), Py::str("A")), (Py::str("GAT"), Py::str("D")), (Py::str("GAC"), Py::str("D")), (Py::str("GAA"), Py::str("E")), (Py::str("GAG"), Py::str("E")), (Py::str("GGT"), Py::str("G")), (Py::str("GGC"), Py::str("G")), (Py::str("GGA"), Py::str("G")), (Py::str("GGG"), Py::str("G"))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__COPY_MW() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("dsDNA"), Py::tuple(vec![Py::Float(617.96), Py::Float(36.04)])), (Py::str("ssDNA"), Py::tuple(vec![Py::Float(308.97), Py::Float(18.02)])), (Py::str("RNA"), Py::tuple(vec![Py::Float(320.5), Py::Float(159.0)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__DNA_COMP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("A"), Py::str("T")), (Py::str("T"), Py::str("A")), (Py::str("G"), Py::str("C")), (Py::str("C"), Py::str("G"))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__DNA_MONO() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("A"), Py::Float(331.2218)), (Py::str("C"), Py::Float(307.1971)), (Py::str("G"), Py::Float(347.2212)), (Py::str("T"), Py::Float(322.2085))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__EPS1() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("A"), Py::Float(15400.0)), (Py::str("C"), Py::Float(7400.0)), (Py::str("G"), Py::Float(11500.0)), (Py::str("T"), Py::Float(8700.0))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__EPS2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("AA"), Py::Float(27400.0)), (Py::str("AC"), Py::Float(21200.0)), (Py::str("AG"), Py::Float(25000.0)), (Py::str("AT"), Py::Float(22800.0)), (Py::str("CA"), Py::Float(21200.0)), (Py::str("CC"), Py::Float(14600.0)), (Py::str("CG"), Py::Float(18000.0)), (Py::str("CT"), Py::Float(15200.0)), (Py::str("GA"), Py::Float(25200.0)), (Py::str("GC"), Py::Float(17600.0)), (Py::str("GG"), Py::Float(21600.0)), (Py::str("GT"), Py::Float(20000.0)), (Py::str("TA"), Py::Float(23400.0)), (Py::str("TC"), Py::Float(16200.0)), (Py::str("TG"), Py::Float(19000.0)), (Py::str("TT"), Py::Float(16800.0))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__HPO3() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(79.979902)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__INHIB() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("competitive"), Py::tuple(vec![Py::Bool(true), Py::Bool(false)])), (Py::str("uncompetitive"), Py::tuple(vec![Py::Bool(false), Py::Bool(true)])), (Py::str("noncompetitive"), Py::tuple(vec![Py::Bool(true), Py::Bool(true)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__NN4() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("AA/TT"), Py::tuple(vec![Py::Float(-7.6), Py::Float(-21.3)])), (Py::str("AT/TA"), Py::tuple(vec![Py::Float(-7.2), Py::Float(-20.4)])), (Py::str("TA/AT"), Py::tuple(vec![Py::Float(-7.2), Py::Float(-21.3)])), (Py::str("CA/GT"), Py::tuple(vec![Py::Float(-8.5), Py::Float(-22.7)])), (Py::str("GT/CA"), Py::tuple(vec![Py::Float(-8.4), Py::Float(-22.4)])), (Py::str("CT/GA"), Py::tuple(vec![Py::Float(-7.8), Py::Float(-21.0)])), (Py::str("GA/CT"), Py::tuple(vec![Py::Float(-8.2), Py::Float(-22.2)])), (Py::str("CG/GC"), Py::tuple(vec![Py::Float(-10.6), Py::Float(-27.2)])), (Py::str("GC/CG"), Py::tuple(vec![Py::Float(-9.8), Py::Float(-24.4)])), (Py::str("GG/CC"), Py::tuple(vec![Py::Float(-8.0), Py::Float(-19.9)]))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__NN4_INIT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::Float(0.2), Py::Float(-5.7)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__NN4_INIT_AT() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::Float(2.2), Py::Float(6.9)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__NN4_SYM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::Float(0.0), Py::Float(-1.4)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__RNA_COMP() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("A"), Py::str("U")), (Py::str("U"), Py::str("A")), (Py::str("G"), Py::str("C")), (Py::str("C"), Py::str("G"))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__RNA_MONO() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("A"), Py::Float(347.2212)), (Py::str("C"), Py::Float(323.1965)), (Py::str("G"), Py::Float(363.2206)), (Py::str("U"), Py::Float(324.1813))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__WATER() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(18.0153)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t2451 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t2451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t2451[0].clone();
    v_fb = t2451[1].clone();
    if ({ let t2452 = v_fa.clone(); Py::Bool({ let t2453 = Py::Int(0i64); if py::eq(&t2452, &t2453) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t2454 = v_fb.clone(); Py::Bool({ let t2455 = Py::Int(0i64); if py::eq(&t2454, &t2455) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t2456 = { let t2458 = v_fa.clone(); Py::Bool({ let t2459 = Py::Int(0i64); if py::gt(&t2458, &t2459)? { true } else { false } }) }; Py::Bool({ let t2457 = { let t2460 = v_fb.clone(); Py::Bool({ let t2461 = Py::Int(0i64); if py::gt(&t2460, &t2461)? { true } else { false } }) }; if py::ne(&t2456, &t2457) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t2462 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t2462;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t2467 = { let t2463 = v_m.clone(); Py::Bool({ let t2464 = v_a.clone(); if py::eq(&t2463, &t2464) { true } else { false } }) }; if t2467.truthy() { t2467 } else { { let t2465 = v_m.clone(); Py::Bool({ let t2466 = v_b.clone(); if py::eq(&t2465, &t2466) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t2468 = v_fm.clone(); Py::Bool({ let t2469 = Py::Int(0i64); if py::eq(&t2468, &t2469) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t2470 = { let t2472 = v_fm.clone(); Py::Bool({ let t2473 = Py::Int(0i64); if py::gt(&t2472, &t2473)? { true } else { false } }) }; Py::Bool({ let t2471 = { let t2474 = v_fa.clone(); Py::Bool({ let t2475 = Py::Int(0i64); if py::gt(&t2474, &t2475)? { true } else { false } }) }; if py::eq(&t2470, &t2471) { true } else { false } }) }).truthy() {
            let t2476 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t2476.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2476[0].clone();
            v_fa = t2476[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__clean_seq(mut v_text: Py, mut v_rna_ok: Py, mut v_dna_ok: Py) -> R {
    let mut v_ch: Py = Py::None;
    let mut v_has_t: Py = Py::None;
    let mut v_has_u: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__need(Py::Bool(matches!(v_text.clone(), Py::Str(_))), Py::str("give the sequence as letters"))?;
    v_s = Py::str("");
    for t4755 in py::iter(&(py::method(&v_text.clone(), "upper", &[])?))? {
        v_ch = t4755;
        if ({ let t4756 = v_ch.clone(); Py::Bool({ let t4757 = Py::str(" "); if py::eq(&t4756, &t4757) { true } else { false } }) }).truthy() {
            continue;
        }
        let _ = f__need({ let t4758 = v_ch.clone(); Py::Bool({ let t4759 = Py::str("ACGTU"); if py::contains(&t4759, &t4758)? { true } else { false } }) }, Py::str("the sequence may contain only A, C, G, T or U"))?;
        v_s = py::iadd(&v_s, &v_ch.clone())?;
    }
    let _ = f__need({ let t4760 = py::len(&v_s.clone())?; Py::Bool({ let t4761 = Py::Int(1i64); if py::ge(&t4760, &t4761)? { true } else { false } }) }, Py::str("the sequence is empty"))?;
    let t4766 = (Py::tuple(vec![{ let t4762 = Py::str("T"); Py::Bool({ let t4763 = v_s.clone(); if py::contains(&t4763, &t4762)? { true } else { false } }) }, { let t4764 = Py::str("U"); Py::Bool({ let t4765 = v_s.clone(); if py::contains(&t4765, &t4764)? { true } else { false } }) }])).items()?;
    if t4766.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_has_t = t4766[0].clone();
    v_has_u = t4766[1].clone();
    let _ = f__need(py::not(&{ let t4767 = v_has_t.clone(); if !t4767.truthy() { t4767 } else { v_has_u.clone() } }), Py::str("a sequence cannot contain both T and U"))?;
    let _ = f__need({ let t4768 = v_rna_ok.clone(); if t4768.truthy() { t4768 } else { py::not(&v_has_u.clone()) } }, Py::str("give a DNA sequence (A, C, G, T)"))?;
    let _ = f__need({ let t4769 = v_dna_ok.clone(); if t4769.truthy() { t4769 } else { py::not(&v_has_t.clone()) } }, Py::str("give an RNA sequence (A, C, G, U)"))?;
    return Ok(Py::tuple(vec![v_s.clone(), v_has_u.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t3872 in py::iter(&(v_vals.clone()))? {
        v_v = t3872;
        let _ = f__need(f__isnum(v_v.clone())?, Py::str("every input must be a finite number"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_name: Py, mut v_v: Py, mut v_lo_open: Py, mut v_hi_open: Py) -> R {
    let _ = f__need(f__isnum(v_v.clone())?, Py::str(&[py::format(&v_name.clone(), "")?, " must be a number".to_string()].concat()))?;
    let _ = f__need({ let t4301 = (if (v_lo_open.clone()).truthy() { { let t4293 = v_v.clone(); Py::Bool({ let t4294 = Py::Int(0i64); if py::gt(&t4293, &t4294)? { true } else { false } }) } } else { { let t4295 = v_v.clone(); Py::Bool({ let t4296 = Py::Int(0i64); if py::ge(&t4295, &t4296)? { true } else { false } }) } }); if !t4301.truthy() { t4301 } else { (if (v_hi_open.clone()).truthy() { { let t4297 = v_v.clone(); Py::Bool({ let t4298 = Py::Int(1i64); if py::lt(&t4297, &t4298)? { true } else { false } }) } } else { { let t4299 = v_v.clone(); Py::Bool({ let t4300 = Py::Int(1i64); if py::le(&t4299, &t4300)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__inhibited(mut v_Vmax: Py, mut v_Km: Py, mut v_S: Py, mut v_I: Py, mut v_Ki: Py, mut v_mode: Py) -> R {
    let mut v_Km_app: Py = Py::None;
    let mut v_V_app: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_v0: Py = Py::None;
    let _ = f__pos(Py::str("Vmax, Km and Ki"), Py::tuple(vec![v_Vmax.clone(), v_Km.clone(), v_Ki.clone()]))?;
    let _ = f__nonneg(Py::str("S and I"), Py::tuple(vec![v_S.clone(), v_I.clone()]))?;
    v_a = py::add(&Py::Float(1.0), &py::truediv(&v_I.clone(), &v_Ki.clone())?)?;
    if ({ let t4875 = v_mode.clone(); Py::Bool({ let t4876 = Py::str("competitive"); if py::eq(&t4875, &t4876) { true } else { false } }) }).truthy() {
        let t4877 = (Py::tuple(vec![py::mul(&v_Km.clone(), &v_a.clone())?, v_Vmax.clone()])).items()?;
        if t4877.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Km_app = t4877[0].clone();
        v_V_app = t4877[1].clone();
    } else {
        if ({ let t4878 = v_mode.clone(); Py::Bool({ let t4879 = Py::str("uncompetitive"); if py::eq(&t4878, &t4879) { true } else { false } }) }).truthy() {
            let t4880 = (Py::tuple(vec![py::truediv(&v_Km.clone(), &v_a.clone())?, py::truediv(&v_Vmax.clone(), &v_a.clone())?])).items()?;
            if t4880.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_Km_app = t4880[0].clone();
            v_V_app = t4880[1].clone();
        } else {
            let t4881 = (Py::tuple(vec![v_Km.clone(), py::truediv(&v_Vmax.clone(), &v_a.clone())?])).items()?;
            if t4881.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_Km_app = t4881[0].clone();
            v_V_app = t4881[1].clone();
        }
    }
    v_v = py::truediv(&py::mul(&v_V_app.clone(), &v_S.clone())?, &py::add(&v_Km_app.clone(), &v_S.clone())?)?;
    v_v0 = py::truediv(&py::mul(&v_Vmax.clone(), &v_S.clone())?, &py::add(&v_Km.clone(), &v_S.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("Km_app"), v_Km_app.clone()), (Py::str("Vmax_app"), v_V_app.clone()), (Py::str("percent_inhibition"), (if ({ let t4894 = v_v0.clone(); Py::Bool({ let t4895 = Py::Int(0i64); if py::gt(&t4894, &t4895)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(100.0), &py::sub(&Py::Float(1.0), &py::truediv(&v_v.clone(), &v_v0.clone())?)?)? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__isnum(mut v_v: Py) -> R {
    return Ok({ let t4924 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t4924.truthy() { t4924 } else { { let t4923 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t4923.truthy() { t4923 } else { py::math1("isfinite", &v_v.clone())? } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__linfit(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_dy: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    v_n = py::len(&v_x.clone())?;
    let t4093 = (Py::tuple(vec![f__mean(v_x.clone())?, f__mean(v_y.clone())?])).items()?;
    if t4093.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t4093[0].clone();
    v_my = t4093[1].clone();
    let t4094 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t4094.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_sxx = t4094[0].clone();
    v_sxy = t4094[1].clone();
    v_syy = t4094[2].clone();
    for t4095 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t4095;
        let t4096 = (Py::tuple(vec![py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &v_mx.clone())?, py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &v_my.clone())?])).items()?;
        if t4096.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_dx = t4096[0].clone();
        v_dy = t4096[1].clone();
        v_sxx = py::iadd(&v_sxx, &py::mul(&v_dx.clone(), &v_dx.clone())?)?;
        v_sxy = py::iadd(&v_sxy, &py::mul(&v_dx.clone(), &v_dy.clone())?)?;
        v_syy = py::iadd(&v_syy, &py::mul(&v_dy.clone(), &v_dy.clone())?)?;
    }
    let _ = f__need({ let t4097 = v_sxx.clone(); Py::Bool({ let t4098 = Py::Int(0i64); if py::gt(&t4097, &t4098)? { true } else { false } }) }, Py::str("the x values must not all be equal"))?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    v_r2 = (if ({ let t4099 = v_syy.clone(); Py::Bool({ let t4100 = Py::Int(0i64); if py::eq(&t4099, &t4100) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)? });
    return Ok(Py::tuple(vec![v_b.clone(), v_a.clone(), v_r2.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mean(mut v_xs: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_s = Py::Float(0.0);
    for t4092 in py::iter(&(v_xs.clone()))? {
        v_x = t4092;
        v_s = py::iadd(&v_s, &v_x.clone())?;
    }
    return Ok(py::truediv(&v_s.clone(), &py::len(&v_xs.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mm_dprofile(mut v_S: Py, mut v_v: Py, mut v_Km: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_da: Py = Py::None;
    let mut v_db: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gp: Py = Py::None;
    let mut v_k: Py = Py::None;
    let t2445 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t2445.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2445[0].clone();
    v_b = t2445[1].clone();
    v_da = t2445[2].clone();
    v_db = t2445[3].clone();
    for t2446 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) {
        v_k = t2446;
        v_g = py::truediv(&py::getitem(&v_S.clone(), &v_k.clone())?, &py::add(&v_Km.clone(), &py::getitem(&v_S.clone(), &v_k.clone())?)?)?;
        v_gp = py::truediv(&py::neg(&py::getitem(&v_S.clone(), &v_k.clone())?)?, &py::pow(&py::add(&v_Km.clone(), &py::getitem(&v_S.clone(), &v_k.clone())?)?, &Py::Int(2i64))?)?;
        v_a = py::iadd(&v_a, &py::mul(&py::getitem(&v_v.clone(), &v_k.clone())?, &v_g.clone())?)?;
        v_b = py::iadd(&v_b, &py::mul(&v_g.clone(), &v_g.clone())?)?;
        v_da = py::iadd(&v_da, &py::mul(&py::getitem(&v_v.clone(), &v_k.clone())?, &v_gp.clone())?)?;
        v_db = py::iadd(&v_db, &py::mul(&py::mul(&Py::Float(2.0), &v_g.clone())?, &v_gp.clone())?)?;
    }
    return Ok(py::mul(&py::neg(&v_a.clone())?, &py::sub(&py::mul(&py::mul(&Py::Float(2.0), &v_da.clone())?, &v_b.clone())?, &py::mul(&v_a.clone(), &v_db.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mm_profile(mut v_S: Py, mut v_v: Py, mut v_Km: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_k: Py = Py::None;
    let t2477 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t2477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2477[0].clone();
    v_b = t2477[1].clone();
    for t2478 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) {
        v_k = t2478;
        v_g = py::truediv(&py::getitem(&v_S.clone(), &v_k.clone())?, &py::add(&v_Km.clone(), &py::getitem(&v_S.clone(), &v_k.clone())?)?)?;
        v_a = py::iadd(&v_a, &py::mul(&py::getitem(&v_v.clone(), &v_k.clone())?, &v_g.clone())?)?;
        v_b = py::iadd(&v_b, &py::mul(&v_g.clone(), &v_g.clone())?)?;
    }
    return Ok(Py::tuple(vec![v_a.clone(), v_b.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nn_sums(mut v_s: Py) -> R {
    let mut v_at: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_ch: Py = Py::None;
    let mut v_dh: Py = Py::None;
    let mut v_ds: Py = Py::None;
    let mut v_ends: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_rv: Py = Py::None;
    let mut v_selfcomp: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_c = Py::str("");
    for t4424 in py::iter(&(v_s.clone()))? {
        v_ch = t4424;
        v_c = py::iadd(&v_c, &py::getitem(&c__DNA_COMP()?, &v_ch.clone())?)?;
    }
    let t4425 = (Py::tuple(vec![py::getitem(&c__NN4_INIT()?, &Py::Int(0i64))?, py::getitem(&c__NN4_INIT()?, &Py::Int(1i64))?])).items()?;
    if t4425.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dh = t4425[0].clone();
    v_ds = t4425[1].clone();
    v_ends = py::add(&py::getitem(&v_s.clone(), &Py::Int(0i64))?, &py::getitem(&v_s.clone(), &py::neg(&Py::Int(1i64))?)?)?;
    v_at = py::add(&py::method(&v_ends.clone(), "count", &[Py::str("A")])?, &py::method(&v_ends.clone(), "count", &[Py::str("T")])?)?;
    v_dh = py::iadd(&v_dh, &py::mul(&py::getitem(&c__NN4_INIT_AT()?, &Py::Int(0i64))?, &v_at.clone())?)?;
    v_ds = py::iadd(&v_ds, &py::mul(&py::getitem(&c__NN4_INIT_AT()?, &Py::Int(1i64))?, &v_at.clone())?)?;
    for t4426 in py::range_iter(&[(py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t4426;
        v_nb = py::add(&py::add(&py::slice(&v_s.clone(), Some((v_i.clone()).i()?), Some((py::add(&v_i.clone(), &Py::Int(2i64))?).i()?), None)?, &Py::str("/"))?, &py::slice(&v_c.clone(), Some((v_i.clone()).i()?), Some((py::add(&v_i.clone(), &Py::Int(2i64))?).i()?), None)?)?;
        v_rv = py::slice(&v_nb.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?;
        v_v = (if ({ let t4427 = v_nb.clone(); Py::Bool({ let t4428 = c__NN4()?; if py::contains(&t4428, &t4427)? { true } else { false } }) }).truthy() { py::getitem(&c__NN4()?, &v_nb.clone())? } else { py::getitem(&c__NN4()?, &v_rv.clone())? });
        v_dh = py::iadd(&v_dh, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?;
        v_ds = py::iadd(&v_ds, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?;
    }
    v_selfcomp = { let t4429 = v_s.clone(); Py::Bool({ let t4430 = f__revcomp(v_s.clone(), Py::Bool(false))?; if py::eq(&t4429, &t4430) { true } else { false } }) };
    if (v_selfcomp.clone()).truthy() {
        v_dh = py::iadd(&v_dh, &py::getitem(&c__NN4_SYM()?, &Py::Int(0i64))?)?;
        v_ds = py::iadd(&v_ds, &py::getitem(&c__NN4_SYM()?, &Py::Int(1i64))?)?;
    }
    return Ok(Py::tuple(vec![v_dh.clone(), v_ds.clone(), v_selfcomp.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4922 in py::iter(&(v_vals.clone()))? {
        v_v = t4922;
        let _ = f__need({ let t4927 = f__isnum(v_v.clone())?; if !t4927.truthy() { t4927 } else { { let t4925 = v_v.clone(); Py::Bool({ let t4926 = Py::Int(0i64); if py::ge(&t4925, &t4926)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t4930 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t4931 = (t4930).items()?;
if t4931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4931[0].clone();
v_v = t4931[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__phase_check(mut v_fractions: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__need({ let t362 = py::len(&v_fractions.clone())?; Py::Bool({ let t363 = Py::Int(4i64); if py::eq(&t362, &t363) { true } else { false } }) }, Py::str("give four fractions: G1, S, G2, M"))?;
    v_s = Py::Float(0.0);
    for t364 in py::iter(&(v_fractions.clone()))? {
        v_f = t364;
        let _ = f__need({ let t367 = f__isnum(v_f.clone())?; if !t367.truthy() { t367 } else { { let t365 = v_f.clone(); Py::Bool({ let t366 = Py::Int(0i64); if py::gt(&t365, &t366)? { true } else { false } }) } } }, Py::str("every phase fraction must be positive"))?;
        v_s = py::iadd(&v_s, &v_f.clone())?;
    }
    let _ = f__need({ let t368 = py::abs(&py::sub(&v_s.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t369 = Py::Float(1e-06); if py::le(&t368, &t369)? { true } else { false } }) }, Py::str("the four fractions must add up to 1"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t4865 in py::iter(&(v_vals.clone()))? {
        v_v = t4865;
        let _ = f__need({ let t4870 = f__isnum(v_v.clone())?; if !t4870.truthy() { t4870 } else { { let t4868 = v_v.clone(); Py::Bool({ let t4869 = Py::Int(0i64); if py::gt(&t4868, &t4869)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t4935 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t4935.truthy() { t4935 } else { { let t4934 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t4934.truthy() { t4934 } else { { let t4932 = v_x.clone(); Py::Bool({ let t4933 = Py::None; if py::is(&t4932, &t4933) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t4936 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t4936;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t4939 = { let t4937 = v_x.clone(); Py::Bool({ let t4938 = Py::Int(0i64); if py::eq(&t4937, &t4938) { true } else { false } }) }; if t4939.truthy() { t4939 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t4940 = v_x.clone(); Py::Bool({ let t4941 = Py::Int(0i64); if py::eq(&t4940, &t4941) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t4950 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t4950.truthy() { t4950 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t4954 = py::not(&{ let t4951 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t4951.truthy() { t4951 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t4954.truthy() { t4954 } else { { let t4952 = py::len(&v_a.clone())?; Py::Bool({ let t4953 = py::len(&v_b.clone())?; if py::ne(&t4952, &t4953) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t4955 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t4956 = (t4955).items()?;
if t4956.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t4956[0].clone();
v_y = t4956[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t4957 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t4957.truthy() { t4957 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t4958 = v_a.clone(); Py::Bool({ let t4959 = v_b.clone(); if py::eq(&t4958, &t4959) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t4961: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t4960 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t4960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4960[0].clone();
            v_b = t4960[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4961 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t4962 = v_d.clone(); Py::Bool({ let t4963 = Py::Int(0i64); if py::eq(&t4962, &t4963) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t4968 = { let t4964 = py::abs(&v_a.clone())?; Py::Bool({ let t4965 = Py::Float(1e-15); if py::lt(&t4964, &t4965)? { true } else { false } }) }; if !t4968.truthy() { t4968 } else { { let t4966 = py::abs(&v_b.clone())?; Py::Bool({ let t4967 = Py::Float(1e-15); if py::lt(&t4966, &t4967)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__revcomp(mut v_s: Py, mut v_rna: Py) -> R {
    let mut v_comp: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_comp = (if (v_rna.clone()).truthy() { c__RNA_COMP()? } else { c__DNA_COMP()? });
    v_out = Py::str("");
    for t4612 in py::range_iter(&[(py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_k = t4612;
        v_out = py::iadd(&v_out, &py::getitem(&v_comp.clone(), &py::getitem(&v_s.clone(), &v_k.clone())?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__soumpasis(mut v_x: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s0: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_term0: Py = Py::None;
    let mut v_term1: Py = Py::None;
    let t1310 = (Py::tuple(vec![Py::Float(1.0), py::truediv(&v_x.clone(), &Py::Float(2.0))?])).items()?;
    if t1310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term0 = t1310[0].clone();
    v_term1 = t1310[1].clone();
    let t1311 = (Py::tuple(vec![v_term0.clone(), v_term1.clone()])).items()?;
    if t1311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s0 = t1311[0].clone();
    v_s1 = t1311[1].clone();
    v_k = Py::Int(0i64);
    while (Py::Bool(true)).truthy() {
        v_k = py::iadd(&v_k, &Py::Int(1i64))?;
        v_term0 = py::mul(&v_term0, &py::truediv(&py::pow(&py::truediv(&v_x.clone(), &Py::Float(2.0))?, &Py::Int(2i64))?, &py::mul(&v_k.clone(), &v_k.clone())?)?)?;
        v_term1 = py::mul(&v_term1, &py::truediv(&py::pow(&py::truediv(&v_x.clone(), &Py::Float(2.0))?, &Py::Int(2i64))?, &py::mul(&v_k.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?)?;
        v_s0 = py::iadd(&v_s0, &v_term0.clone())?;
        v_s1 = py::iadd(&v_s1, &v_term1.clone())?;
        if ({ let t1316 = { let t1312 = v_term0.clone(); Py::Bool({ let t1313 = py::mul(&Py::Float(1e-18), &v_s0.clone())?; if py::lt(&t1312, &t1313)? { true } else { false } }) }; if !t1316.truthy() { t1316 } else { { let t1314 = v_term1.clone(); Py::Bool({ let t1315 = py::mul(&Py::Float(1e-18), &v_s1.clone())?; if py::lt(&t1314, &t1315)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t1317 = v_k.clone(); Py::Bool({ let t1318 = Py::Int(500i64); if py::gt(&t1317, &t1318)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(py::mul(&py::math1("exp", &py::neg(&v_x.clone())?)?, &py::add(&v_s0.clone(), &v_s1.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__soumpasis_root() -> R {
    return Ok(f__bisect({ py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__soumpasis(v_x.clone())?, &Py::Float(0.5))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.1), Py::Float(10.0), Py::Int(400i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__strand_mw(mut v_s: Py, mut v_rna: Py) -> R {
    let mut v_ch: Py = Py::None;
    let mut v_tab: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_tab = (if (v_rna.clone()).truthy() { c__RNA_MONO()? } else { c__DNA_MONO()? });
    v_w = Py::Float(0.0);
    for t3214 in py::iter(&(v_s.clone()))? {
        v_ch = t3214;
        v_w = py::iadd(&v_w, &py::getitem(&v_tab.clone(), &v_ch.clone())?)?;
    }
    return Ok(py::sub(&v_w.clone(), &py::mul(&py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?, &c__WATER()?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_absorbance_ratios(mut v_A260: Py, mut v_A280: Py, mut v_A230: Py, mut v_A320: Py) -> R {
    let mut v_a230: Py = Py::None;
    let mut v_a260: Py = Py::None;
    let mut v_a280: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("A260 and A280"), Py::tuple(vec![v_A260.clone(), v_A280.clone()]))?;
    let _ = f__nonneg(Py::str("A320"), Py::tuple(vec![v_A320.clone()]))?;
    let t11 = (Py::tuple(vec![py::sub(&v_A260.clone(), &v_A320.clone())?, py::sub(&v_A280.clone(), &v_A320.clone())?])).items()?;
    if t11.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a260 = t11[0].clone();
    v_a280 = t11[1].clone();
    let _ = f__need({ let t16 = { let t12 = v_a260.clone(); Py::Bool({ let t13 = Py::Int(0i64); if py::gt(&t12, &t13)? { true } else { false } }) }; if !t16.truthy() { t16 } else { { let t14 = v_a280.clone(); Py::Bool({ let t15 = Py::Int(0i64); if py::gt(&t14, &t15)? { true } else { false } }) } } }, Py::str("the background A320 must be below A260 and A280"))?;
    v_out = Py::dict(vec![(Py::str("ratio_260_280"), py::truediv(&v_a260.clone(), &v_a280.clone())?)]);
    if ({ let t17 = v_A230.clone(); Py::Bool({ let t18 = Py::None; if !py::is(&t17, &t18) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("A230"), Py::tuple(vec![v_A230.clone()]))?;
        v_a230 = py::sub(&v_A230.clone(), &v_A320.clone())?;
        let _ = f__need({ let t19 = v_a230.clone(); Py::Bool({ let t20 = Py::Int(0i64); if py::gt(&t19, &t20)? { true } else { false } }) }, Py::str("the background A320 must be below A230"))?;
        py::setitem(&v_out.clone(), &Py::str("ratio_260_230"), py::truediv(&v_a260.clone(), &v_a230.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_binding_free_energy(mut v_Kd: Py, mut v_T: Py) -> R {
    let mut v_dG: Py = Py::None;
    let _ = f__pos(Py::str("Kd and T"), Py::tuple(vec![v_Kd.clone(), v_T.clone()]))?;
    v_dG = py::mul(&py::mul(&c_R_GAS()?, &v_T.clone())?, &py::math1("log", &py::mul(&v_Kd.clone(), &Py::Float(0.001))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("dG"), v_dG.clone()), (Py::str("dG_kJ_mol"), py::truediv(&v_dG.clone(), &Py::Float(1000.0))?), (Py::str("dG_kcal_mol"), py::truediv(&v_dG.clone(), &Py::Float(4184.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buffer_after_addition(mut v_C: Py, mut v_V: Py, mut v_pH0: Py, mut v_pKa: Py, mut v_n_add: Py, mut v_added: Py) -> R {
    let mut v_na: Py = Py::None;
    let mut v_na2: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_nb2: Py = Py::None;
    let mut v_pH: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("C and V"), Py::tuple(vec![v_C.clone(), v_V.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_pH0.clone(), v_pKa.clone()]))?;
    let _ = f__pos(Py::str("the amount added"), Py::tuple(vec![v_n_add.clone()]))?;
    let _ = f__need({ let t111 = v_added.clone(); Py::Bool({ let t112 = Py::tuple(vec![Py::str("acid"), Py::str("base")]); if py::contains(&t112, &t111)? { true } else { false } }) }, Py::str("added: acid or base (strong)"))?;
    v_r = py::pow(&Py::Float(10.0), &py::sub(&v_pH0.clone(), &v_pKa.clone())?)?;
    v_nb = py::truediv(&py::mul(&py::mul(&v_C.clone(), &v_V.clone())?, &v_r.clone())?, &py::add(&Py::Float(1.0), &v_r.clone())?)?;
    v_na = py::sub(&py::mul(&v_C.clone(), &v_V.clone())?, &v_nb.clone())?;
    if ({ let t113 = v_added.clone(); Py::Bool({ let t114 = Py::str("base"); if py::eq(&t113, &t114) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t115 = v_n_add.clone(); Py::Bool({ let t116 = v_na.clone(); if py::lt(&t115, &t116)? { true } else { false } }) }, Py::str("the base added exceeds the acid form: the buffer is exhausted"))?;
        let t117 = (Py::tuple(vec![py::add(&v_nb.clone(), &v_n_add.clone())?, py::sub(&v_na.clone(), &v_n_add.clone())?])).items()?;
        if t117.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nb2 = t117[0].clone();
        v_na2 = t117[1].clone();
    } else {
        let _ = f__need({ let t118 = v_n_add.clone(); Py::Bool({ let t119 = v_nb.clone(); if py::lt(&t118, &t119)? { true } else { false } }) }, Py::str("the acid added exceeds the base form: the buffer is exhausted"))?;
        let t120 = (Py::tuple(vec![py::sub(&v_nb.clone(), &v_n_add.clone())?, py::add(&v_na.clone(), &v_n_add.clone())?])).items()?;
        if t120.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nb2 = t120[0].clone();
        v_na2 = t120[1].clone();
    }
    v_pH = py::add(&v_pKa.clone(), &py::math1("log10", &py::truediv(&v_nb2.clone(), &v_na2.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("pH"), v_pH.clone()), (Py::str("delta_pH"), py::sub(&v_pH.clone(), &v_pH0.clone())?), (Py::str("n_base"), v_nb2.clone()), (Py::str("n_acid"), v_na2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buffer_capacity(mut v_C: Py, mut v_pKa: Py, mut v_pH: Py) -> R {
    let mut v_Cm: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_Ka: Py = Py::None;
    let mut v_Kw: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_beta_buffer: Py = Py::None;
    let _ = f__pos(Py::str("C"), Py::tuple(vec![v_C.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_pKa.clone(), v_pH.clone()]))?;
    v_H = py::pow(&Py::Float(10.0), &py::neg(&v_pH.clone())?)?;
    v_Ka = py::pow(&Py::Float(10.0), &py::neg(&v_pKa.clone())?)?;
    v_Kw = Py::Float(1e-14);
    v_Cm = py::mul(&v_C.clone(), &Py::Float(0.001))?;
    v_beta = py::mul(&py::math1("log", &Py::Float(10.0))?, &py::add(&py::add(&v_H.clone(), &py::truediv(&v_Kw.clone(), &v_H.clone())?)?, &py::truediv(&py::mul(&py::mul(&v_Cm.clone(), &v_Ka.clone())?, &v_H.clone())?, &py::pow(&py::add(&v_Ka.clone(), &v_H.clone())?, &Py::Int(2i64))?)?)?)?;
    v_beta_buffer = py::truediv(&py::mul(&py::mul(&py::mul(&py::math1("log", &Py::Float(10.0))?, &v_Cm.clone())?, &v_Ka.clone())?, &v_H.clone())?, &py::pow(&py::add(&v_Ka.clone(), &v_H.clone())?, &Py::Int(2i64))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("beta"), py::mul(&v_beta.clone(), &Py::Float(1000.0))?), (Py::str("beta_mM"), py::mul(&v_beta.clone(), &Py::Float(1000.0))?), (Py::str("beta_buffer_mM"), py::mul(&v_beta_buffer.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_buffer_preparation(mut v_C: Py, mut v_V: Py, mut v_pH: Py, mut v_pKa: Py, mut v_MW_acid: Py, mut v_MW_base: Py) -> R {
    let mut v_ca: Py = Py::None;
    let mut v_cb: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("C and V"), Py::tuple(vec![v_C.clone(), v_V.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_pH.clone(), v_pKa.clone()]))?;
    v_r = py::pow(&Py::Float(10.0), &py::sub(&v_pH.clone(), &v_pKa.clone())?)?;
    v_cb = py::truediv(&py::mul(&v_C.clone(), &v_r.clone())?, &py::add(&Py::Float(1.0), &v_r.clone())?)?;
    v_ca = py::sub(&v_C.clone(), &v_cb.clone())?;
    v_out = Py::dict(vec![(Py::str("C_base"), v_cb.clone()), (Py::str("C_acid"), v_ca.clone()), (Py::str("n_base"), py::mul(&v_cb.clone(), &v_V.clone())?), (Py::str("n_acid"), py::mul(&v_ca.clone(), &v_V.clone())?), (Py::str("ratio"), v_r.clone())]);
    if ({ let t211 = v_MW_acid.clone(); Py::Bool({ let t212 = Py::None; if !py::is(&t211, &t212) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the acid molar mass"), Py::tuple(vec![v_MW_acid.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("m_acid_g"), py::mul(&py::mul(&py::mul(&v_ca.clone(), &v_V.clone())?, &v_MW_acid.clone())?, &Py::Float(1000.0))?)?;
    }
    if ({ let t213 = v_MW_base.clone(); Py::Bool({ let t214 = Py::None; if !py::is(&t213, &t214) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the base molar mass"), Py::tuple(vec![v_MW_base.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("m_base_g"), py::mul(&py::mul(&py::mul(&v_cb.clone(), &v_V.clone())?, &v_MW_base.clone())?, &Py::Float(1000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_c1v1_dilution(mut v_C1: Py, mut v_C2: Py, mut v_V2: Py) -> R {
    let mut v_V1: Py = Py::None;
    let _ = f__pos(Py::str("C1, C2 and V2"), Py::tuple(vec![v_C1.clone(), v_C2.clone(), v_V2.clone()]))?;
    let _ = f__need({ let t260 = v_C2.clone(); Py::Bool({ let t261 = v_C1.clone(); if py::le(&t260, &t261)? { true } else { false } }) }, Py::str("the final concentration cannot exceed the stock concentration"))?;
    v_V1 = py::truediv(&py::mul(&v_C2.clone(), &v_V2.clone())?, &v_C1.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V1"), v_V1.clone()), (Py::str("V1_uL"), py::mul(&v_V1.clone(), &Py::Float(1000000000.0))?), (Py::str("V_diluent"), py::sub(&v_V2.clone(), &v_V1.clone())?), (Py::str("dilution_factor"), py::truediv(&v_C1.clone(), &v_C2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cell_cycle_age_corrected(mut v_Tc: Py, mut v_fractions: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ends: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("the cycle time Tc"), Py::tuple(vec![v_Tc.clone()]))?;
    let _ = f__phase_check(v_fractions.clone())?;
    let t314 = (Py::tuple(vec![Py::list(vec![]), Py::Float(0.0)])).items()?;
    if t314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ends = t314[0].clone();
    v_F = t314[1].clone();
    for t315 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
        v_k = t315;
        v_F = py::iadd(&v_F, &py::getitem(&v_fractions.clone(), &v_k.clone())?)?;
        if ({ let t316 = v_k.clone(); Py::Bool({ let t317 = Py::Int(3i64); if py::eq(&t316, &t317) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_ends.clone(), "append", &[v_Tc.clone()])?;
        } else {
            let _ = py::method(&v_ends.clone(), "append", &[py::mul(&py::neg(&v_Tc.clone())?, &py::math1("log2", &py::sub(&Py::Float(1.0), &py::truediv(&v_F.clone(), &Py::Float(2.0))?)?)?)?])?;
        }
    }
    v_d = Py::list(vec![py::getitem(&v_ends.clone(), &Py::Int(0i64))?, py::sub(&py::getitem(&v_ends.clone(), &Py::Int(1i64))?, &py::getitem(&v_ends.clone(), &Py::Int(0i64))?)?, py::sub(&py::getitem(&v_ends.clone(), &Py::Int(2i64))?, &py::getitem(&v_ends.clone(), &Py::Int(1i64))?)?, py::sub(&py::getitem(&v_ends.clone(), &Py::Int(3i64))?, &py::getitem(&v_ends.clone(), &Py::Int(2i64))?)?]);
    return Ok(f__out(Py::dict(vec![(Py::str("G1_h"), py::truediv(&py::getitem(&v_d.clone(), &Py::Int(0i64))?, &c_HOUR()?)?), (Py::str("S_h"), py::truediv(&py::getitem(&v_d.clone(), &Py::Int(1i64))?, &c_HOUR()?)?), (Py::str("G2_h"), py::truediv(&py::getitem(&v_d.clone(), &Py::Int(2i64))?, &c_HOUR()?)?), (Py::str("M_h"), py::truediv(&py::getitem(&v_d.clone(), &Py::Int(3i64))?, &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cell_cycle_simple(mut v_Tc: Py, mut v_fractions: Py) -> R {
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("the cycle time Tc"), Py::tuple(vec![v_Tc.clone()]))?;
    let _ = f__phase_check(v_fractions.clone())?;
    v_d = { let mut out = vec![]; for t370 in py::iter(&(v_fractions.clone()))? { let mut v_f: Py; 
v_f = t370;
out.push(py::mul(&v_f.clone(), &v_Tc.clone())?); } Py::list(out) };
    return Ok(f__out(Py::dict(vec![(Py::str("G1_h"), py::truediv(&py::getitem(&v_d.clone(), &Py::Int(0i64))?, &c_HOUR()?)?), (Py::str("S_h"), py::truediv(&py::getitem(&v_d.clone(), &Py::Int(1i64))?, &c_HOUR()?)?), (Py::str("G2_h"), py::truediv(&py::getitem(&v_d.clone(), &Py::Int(2i64))?, &c_HOUR()?)?), (Py::str("M_h"), py::truediv(&py::getitem(&v_d.clone(), &Py::Int(3i64))?, &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cell_equilibration(mut v_r: Py, mut v_P: Py, mut v_C_out: Py, mut v_C0: Py, mut v_t: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("r and P"), Py::tuple(vec![v_r.clone(), v_P.clone()]))?;
    let _ = f__nonneg(Py::str("the concentrations and t"), Py::tuple(vec![v_C_out.clone(), v_C0.clone(), v_t.clone()]))?;
    v_k = py::truediv(&py::mul(&Py::Float(3.0), &v_P.clone())?, &v_r.clone())?;
    v_C = py::add(&v_C_out.clone(), &py::mul(&py::sub(&v_C0.clone(), &v_C_out.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("tau"), py::truediv(&Py::Float(1.0), &v_k.clone())?), (Py::str("t_half"), py::truediv(&c_LN2()?, &v_k.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cell_geometry(mut v_d: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the diameter d"), Py::tuple(vec![v_d.clone()]))?;
    v_r = py::truediv(&v_d.clone(), &Py::Float(2.0))?;
    v_V = py::mul(&py::mul(&py::truediv(&Py::Float(4.0), &Py::Float(3.0))?, &Py::Float(std::f64::consts::PI))?, &py::pow(&v_r.clone(), &Py::Int(3i64))?)?;
    v_A = py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &v_r.clone())?, &v_r.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("V_fL"), py::mul(&v_V.clone(), &Py::Float(1e+18))?), (Py::str("A"), v_A.clone()), (Py::str("A_um2"), py::mul(&v_A.clone(), &Py::Float(1000000000000.0))?), (Py::str("SA_V_per_um"), py::mul(&py::truediv(&v_A.clone(), &v_V.clone())?, &Py::Float(1e-06))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cell_volume_osmotic(mut v_V0: Py, mut v_b: Py, mut v_osm0: Py, mut v_osm1: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_Vb: Py = Py::None;
    let _ = f__pos(Py::str("V0, osm0 and osm1"), Py::tuple(vec![v_V0.clone(), v_osm0.clone(), v_osm1.clone()]))?;
    let _ = f__frac(Py::str("the osmotically inactive fraction b"), v_b.clone(), Py::Bool(false), Py::Bool(true))?;
    v_Vb = py::mul(&v_b.clone(), &v_V0.clone())?;
    v_V = py::add(&v_Vb.clone(), &py::truediv(&py::mul(&py::sub(&v_V0.clone(), &v_Vb.clone())?, &v_osm0.clone())?, &v_osm1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("V_fL"), py::mul(&v_V.clone(), &Py::Float(1e+18))?), (Py::str("ratio"), py::truediv(&v_V.clone(), &v_V0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_centrifuge_rcf(mut v_rpm: Py, mut v_r: Py) -> R {
    let mut v_rcf: Py = Py::None;
    let _ = f__pos(Py::str("the rotor speed and the radius"), Py::tuple(vec![v_rpm.clone(), v_r.clone()]))?;
    v_rcf = py::truediv(&py::mul(&py::mul(&v_rpm.clone(), &v_rpm.clone())?, &v_r.clone())?, &c_G0()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("RCF"), v_rcf.clone()), (Py::str("rpm_value"), py::truediv(&v_rpm.clone(), &c_RPM()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_centrifuge_rpm(mut v_RCF: Py, mut v_r: Py) -> R {
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("the RCF and the radius"), Py::tuple(vec![v_RCF.clone(), v_r.clone()]))?;
    v_w = py::math1("sqrt", &py::truediv(&py::mul(&v_RCF.clone(), &c_G0()?)?, &v_r.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("omega"), v_w.clone()), (Py::str("rpm_value"), py::truediv(&v_w.clone(), &c_RPM()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cfu_count(mut v_colonies: Py, mut v_dilution: Py, mut v_V: Py) -> R {
    let mut v_c: Py = Py::None;
    let _ = f__need({ let t3621 = Py::Bool(matches!(v_colonies.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3621.truthy() { t3621 } else { { let t3619 = v_colonies.clone(); Py::Bool({ let t3620 = Py::Int(0i64); if py::ge(&t3619, &t3620)? { true } else { false } }) } } }, Py::str("the colony count must be a whole number"))?;
    let _ = f__need({ let t3626 = f__isnum(v_dilution.clone())?; if !t3626.truthy() { t3626 } else { { let t3624 = v_dilution.clone(); Py::Bool({ let t3625 = Py::Int(1i64); if py::ge(&t3624, &t3625)? { true } else { false } }) } } }, Py::str("the dilution factor must be at least 1 (e.g. 1e6 for a 10^-6 dilution)"))?;
    let _ = f__pos(Py::str("the plated volume V"), Py::tuple(vec![v_V.clone()]))?;
    v_c = py::truediv(&py::mul(&v_colonies.clone(), &v_dilution.clone())?, &py::mul(&v_V.clone(), &Py::Float(1000000.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("cfu_per_mL"), v_c.clone()), (Py::str("log10_cfu_per_mL"), (if ({ let t3643 = v_c.clone(); Py::Bool({ let t3644 = Py::Int(0i64); if py::gt(&t3643, &t3644)? { true } else { false } }) }).truthy() { py::math1("log10", &v_c.clone())? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_channel_current(mut v_N: Py, mut v_Po: Py, mut v_gamma: Py, mut v_V: Py, mut v_E: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_I: Py = Py::None;
    let _ = f__need({ let t644 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t644.truthy() { t644 } else { { let t642 = v_N.clone(); Py::Bool({ let t643 = Py::Int(1i64); if py::ge(&t642, &t643)? { true } else { false } }) } } }, Py::str("N must be a whole number of channels, at least 1"))?;
    let _ = f__frac(Py::str("the open probability Po"), v_Po.clone(), Py::Bool(false), Py::Bool(false))?;
    let _ = f__pos(Py::str("the single-channel conductance gamma"), Py::tuple(vec![v_gamma.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_V.clone(), v_E.clone()]))?;
    v_G = py::mul(&py::mul(&v_N.clone(), &v_Po.clone())?, &v_gamma.clone())?;
    v_I = py::mul(&v_G.clone(), &py::sub(&v_V.clone(), &v_E.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("I"), v_I.clone()), (Py::str("I_pA"), py::mul(&v_I.clone(), &Py::Float(1000000000000.0))?), (Py::str("G"), v_G.clone()), (Py::str("i_single"), py::mul(&v_gamma.clone(), &py::sub(&v_V.clone(), &v_E.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_competitive_occupancy(mut v_L: Py, mut v_Kd: Py, mut v_I: Py, mut v_Ki: Py) -> R {
    let mut v_den: Py = Py::None;
    let _ = f__pos(Py::str("Kd and Ki"), Py::tuple(vec![v_Kd.clone(), v_Ki.clone()]))?;
    let _ = f__nonneg(Py::str("L and I"), Py::tuple(vec![v_L.clone(), v_I.clone()]))?;
    v_den = py::add(&py::add(&Py::Float(1.0), &py::truediv(&v_L.clone(), &v_Kd.clone())?)?, &py::truediv(&v_I.clone(), &v_Ki.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("occupancy_L"), py::truediv(&py::truediv(&v_L.clone(), &v_Kd.clone())?, &v_den.clone())?), (Py::str("occupancy_I"), py::truediv(&py::truediv(&v_I.clone(), &v_Ki.clone())?, &v_den.clone())?), (Py::str("free_fraction"), py::truediv(&Py::Float(1.0), &v_den.clone())?), (Py::str("Kd_app"), py::mul(&v_Kd.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_I.clone(), &v_Ki.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diffusion_time(mut v_x: Py, mut v_D: Py, mut v_dims: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("x and D"), Py::tuple(vec![v_x.clone(), v_D.clone()]))?;
    let _ = f__need({ let t855 = v_dims.clone(); Py::Bool({ let t856 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)]); if py::contains(&t856, &t855)? { true } else { false } }) }, Py::str("dims must be 1, 2 or 3"))?;
    v_t = py::truediv(&py::mul(&v_x.clone(), &v_x.clone())?, &py::mul(&py::mul(&Py::Float(2.0), &v_dims.clone())?, &v_D.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_min"), py::truediv(&v_t.clone(), &c_MINUTE()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dna_copy_number(mut v_m: Py, mut v_n: Py, mut v_molecule: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_mol: Py = Py::None;
    let mut v_mw: Py = Py::None;
    let _ = f__pos(Py::str("the mass m"), Py::tuple(vec![v_m.clone()]))?;
    let _ = f__need({ let t903 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t903.truthy() { t903 } else { { let t901 = v_n.clone(); Py::Bool({ let t902 = Py::Int(1i64); if py::ge(&t901, &t902)? { true } else { false } }) } } }, Py::str("the length n must be a whole number of bases or base pairs"))?;
    let _ = f__need({ let t904 = v_molecule.clone(); Py::Bool({ let t905 = c__COPY_MW()?; if py::contains(&t905, &t904)? { true } else { false } }) }, Py::str("molecule: dsDNA, ssDNA or RNA"))?;
    let t906 = (py::getitem(&c__COPY_MW()?, &v_molecule.clone())?).items()?;
    if t906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t906[0].clone();
    v_b = t906[1].clone();
    v_mw = py::add(&py::mul(&v_a.clone(), &v_n.clone())?, &v_b.clone())?;
    v_mol = py::truediv(&py::mul(&v_m.clone(), &Py::Float(1000.0))?, &v_mw.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("MW"), py::truediv(&v_mw.clone(), &Py::Float(1000.0))?), (Py::str("MW_g_mol"), v_mw.clone()), (Py::str("pmol"), py::mul(&v_mol.clone(), &Py::Float(1000000000000.0))?), (Py::str("copies"), py::mul(&v_mol.clone(), &c_N_A()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_donnan_equilibrium(mut v_A: Py, mut v_zA: Py, mut v_c: Py, mut v_T: Py) -> R {
    let mut v_Cl_in: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_K_in: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::str("the impermeant anion concentration A, the salt concentration c and T"), Py::tuple(vec![v_A.clone(), v_c.clone(), v_T.clone()]))?;
    let _ = f__need({ let t954 = Py::Bool(matches!(v_zA.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t954.truthy() { t954 } else { { let t953 = py::not(&Py::Bool(matches!(v_zA.clone(), Py::Bool(_)))); if !t953.truthy() { t953 } else { { let t951 = v_zA.clone(); Py::Bool({ let t952 = Py::Int(1i64); if py::ge(&t951, &t952)? { true } else { false } }) } } } } }, Py::str("zA is the magnitude of the anion's charge (1, 2, 3)"))?;
    v_q = py::mul(&v_zA.clone(), &v_A.clone())?;
    v_Cl_in = py::truediv(&py::add(&py::neg(&v_q.clone())?, &py::math1("sqrt", &py::add(&py::mul(&v_q.clone(), &v_q.clone())?, &py::mul(&py::mul(&Py::Float(4.0), &v_c.clone())?, &v_c.clone())?)?)?)?, &Py::Float(2.0))?;
    v_K_in = py::add(&v_Cl_in.clone(), &v_q.clone())?;
    v_E = py::mul(&py::truediv(&py::mul(&c_R_GAS()?, &v_T.clone())?, &c_FARADAY()?)?, &py::math1("log", &py::truediv(&v_c.clone(), &v_K_in.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("K_in"), v_K_in.clone()), (Py::str("Cl_in"), v_Cl_in.clone()), (Py::str("ratio"), py::truediv(&v_K_in.clone(), &v_c.clone())?), (Py::str("E"), v_E.clone()), (Py::str("E_mV"), py::mul(&v_E.clone(), &Py::Float(1000.0))?), (Py::str("excess_mOsm"), py::sub(&py::add(&py::add(&v_K_in.clone(), &v_Cl_in.clone())?, &v_A.clone())?, &py::mul(&Py::Float(2.0), &v_c.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_doubling_from_counts(mut v_N0: Py, mut v_N: Py, mut v_t: Py) -> R {
    let mut v_mu: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_td: Py = Py::None;
    let _ = f__pos(Py::str("N0, N and t"), Py::tuple(vec![v_N0.clone(), v_N.clone(), v_t.clone()]))?;
    let _ = f__need({ let t999 = v_N.clone(); Py::Bool({ let t1000 = v_N0.clone(); if py::gt(&t999, &t1000)? { true } else { false } }) }, Py::str("the final count must exceed the initial count"))?;
    v_n = py::math1("log2", &py::truediv(&v_N.clone(), &v_N0.clone())?)?;
    v_td = py::truediv(&v_t.clone(), &v_n.clone())?;
    v_mu = py::truediv(&c_LN2()?, &v_td.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("td"), v_td.clone()), (Py::str("td_h"), py::truediv(&v_td.clone(), &c_HOUR()?)?), (Py::str("generations"), v_n.clone()), (Py::str("mu"), v_mu.clone()), (Py::str("mu_per_h"), py::mul(&v_mu.clone(), &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_enzyme_assay_activity(mut v_dA: Py, mut v_dt: Py, mut v_eps: Py, mut v_l: Py, mut v_V_assay: Py, mut v_V_enzyme: Py, mut v_protein: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_rate: Py = Py::None;
    let mut v_u_per_ml: Py = Py::None;
    let mut v_umol_min: Py = Py::None;
    let _ = f__pos(Py::str("dt, eps, l and the volumes"), Py::tuple(vec![v_dt.clone(), v_eps.clone(), v_l.clone(), v_V_assay.clone(), v_V_enzyme.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_dA.clone()]))?;
    let _ = f__need({ let t1046 = v_dA.clone(); Py::Bool({ let t1047 = Py::Int(0i64); if py::ne(&t1046, &t1047) { true } else { false } }) }, Py::str("the absorbance change must not be zero"))?;
    let _ = f__need({ let t1048 = v_V_enzyme.clone(); Py::Bool({ let t1049 = v_V_assay.clone(); if py::le(&t1048, &t1049)? { true } else { false } }) }, Py::str("the enzyme volume cannot exceed the assay volume"))?;
    v_rate = py::truediv(&py::truediv(&py::abs(&v_dA.clone())?, &v_dt.clone())?, &py::mul(&v_eps.clone(), &v_l.clone())?)?;
    v_umol_min = py::mul(&py::mul(&py::mul(&v_rate.clone(), &v_V_assay.clone())?, &Py::Float(1000000.0))?, &c_MINUTE()?)?;
    v_u_per_ml = py::truediv(&v_umol_min.clone(), &py::mul(&v_V_enzyme.clone(), &Py::Float(1000000.0))?)?;
    v_out = Py::dict(vec![(Py::str("rate"), v_rate.clone()), (Py::str("units"), v_umol_min.clone()), (Py::str("units_per_mL"), v_u_per_ml.clone())]);
    if ({ let t1050 = v_protein.clone(); Py::Bool({ let t1051 = Py::None; if !py::is(&t1050, &t1051) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the protein concentration"), Py::tuple(vec![v_protein.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("specific_activity"), py::truediv(&v_u_per_ml.clone(), &v_protein.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exponential_growth(mut v_N0: Py, mut v_td: Py, mut v_t: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("N0 and the doubling time td"), Py::tuple(vec![v_N0.clone(), v_td.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_n = py::truediv(&v_t.clone(), &v_td.clone())?;
    v_N = py::mul(&v_N0.clone(), &py::pow(&Py::Float(2.0), &v_n.clone())?)?;
    v_mu = py::truediv(&c_LN2()?, &v_td.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("generations"), v_n.clone()), (Py::str("fold"), py::truediv(&v_N.clone(), &v_N0.clone())?), (Py::str("mu"), v_mu.clone()), (Py::str("mu_per_h"), py::mul(&v_mu.clone(), &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_expression_steady_state(mut v_k_tx: Py, mut v_delta_m: Py, mut v_k_tl: Py, mut v_delta_p: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let _ = f__pos(Py::str("all rates"), Py::tuple(vec![v_k_tx.clone(), v_delta_m.clone(), v_k_tl.clone(), v_delta_p.clone()]))?;
    v_m = py::truediv(&v_k_tx.clone(), &v_delta_m.clone())?;
    v_p = py::truediv(&py::mul(&v_k_tl.clone(), &v_m.clone())?, &v_delta_p.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("m_ss"), v_m.clone()), (Py::str("p_ss"), v_p.clone()), (Py::str("mRNA_half_life"), py::truediv(&c_LN2()?, &v_delta_m.clone())?), (Py::str("protein_half_life"), py::truediv(&c_LN2()?, &v_delta_p.clone())?), (Py::str("mRNA_half_life_min"), py::truediv(&py::truediv(&c_LN2()?, &v_delta_m.clone())?, &c_MINUTE()?)?), (Py::str("protein_half_life_h"), py::truediv(&py::truediv(&c_LN2()?, &v_delta_p.clone())?, &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fick_flux(mut v_D: Py, mut v_C1: Py, mut v_C2: Py, mut v_dx: Py) -> R {
    let mut v_J: Py = Py::None;
    let _ = f__pos(Py::str("the diffusion coefficient D and the distance dx"), Py::tuple(vec![v_D.clone(), v_dx.clone()]))?;
    let _ = f__nonneg(Py::str("the concentrations C1 and C2"), Py::tuple(vec![v_C1.clone(), v_C2.clone()]))?;
    v_J = py::truediv(&py::mul(&v_D.clone(), &py::sub(&v_C1.clone(), &v_C2.clone())?)?, &v_dx.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("J"), v_J.clone()), (Py::str("J_nmol_cm2_s"), py::mul(&v_J.clone(), &Py::Float(100000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fraction_bound(mut v_L: Py, mut v_Kd: Py, mut v_Rt: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__pos(Py::str("L and Kd"), Py::tuple(vec![v_L.clone(), v_Kd.clone()]))?;
    v_th = py::truediv(&v_L.clone(), &py::add(&v_Kd.clone(), &v_L.clone())?)?;
    v_out = Py::dict(vec![(Py::str("theta"), v_th.clone()), (Py::str("theta_percent"), py::mul(&Py::Float(100.0), &v_th.clone())?)]);
    if ({ let t1237 = v_Rt.clone(); Py::Bool({ let t1238 = Py::None; if !py::is(&t1237, &t1238) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the receptor concentration Rt"), Py::tuple(vec![v_Rt.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("bound"), py::mul(&v_th.clone(), &v_Rt.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_frap_diffusion(mut v_w: Py, mut v_t_half: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_tauD: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let _ = f__pos(Py::str("the bleach radius w and the half-time t_half"), Py::tuple(vec![v_w.clone(), v_t_half.clone()]))?;
    v_xs = f__soumpasis_root()?;
    v_tauD = py::truediv(&py::mul(&v_xs.clone(), &v_t_half.clone())?, &Py::Float(2.0))?;
    v_D = py::truediv(&py::mul(&v_w.clone(), &v_w.clone())?, &py::mul(&Py::Float(4.0), &v_tauD.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("D_um2_s"), py::mul(&v_D.clone(), &Py::Float(1000000000000.0))?), (Py::str("tauD"), v_tauD.clone()), (Py::str("gamma"), py::truediv(&v_tauD.clone(), &v_t_half.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gc_content(mut v_seq: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rna: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t1372 = (f__clean_seq(v_seq.clone(), Py::Bool(true), Py::Bool(true))?).items()?;
    if t1372.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1372[0].clone();
    v_rna = t1372[1].clone();
    let t1373 = (Py::tuple(vec![py::method(&v_s.clone(), "count", &[Py::str("G")])?, py::method(&v_s.clone(), "count", &[Py::str("C")])?])).items()?;
    if t1373.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1373[0].clone();
    v_c = t1373[1].clone();
    v_n = py::len(&v_s.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("gc_fraction"), py::truediv(&py::add(&v_g.clone(), &v_c.clone())?, &v_n.clone())?), (Py::str("gc_percent"), py::truediv(&py::mul(&Py::Float(100.0), &py::add(&v_g.clone(), &v_c.clone())?)?, &v_n.clone())?), (Py::str("length"), v_n.clone()), (Py::str("G"), v_g.clone()), (Py::str("C"), v_c.clone()), (Py::str("A"), py::method(&v_s.clone(), "count", &[Py::str("A")])?), (Py::str("T_or_U"), (if (v_rna.clone()).truthy() { py::method(&v_s.clone(), "count", &[Py::str("U")])? } else { py::method(&v_s.clone(), "count", &[Py::str("T")])? }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_genome_coverage(mut v_L: Py, mut v_N: Py, mut v_G: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let _ = f__need({ let t1418 = Py::Bool(matches!(v_L.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1418.truthy() { t1418 } else { { let t1417 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1417.truthy() { t1417 } else { { let t1416 = { let t1412 = v_L.clone(); Py::Bool({ let t1413 = Py::Int(1i64); if py::ge(&t1412, &t1413)? { true } else { false } }) }; if !t1416.truthy() { t1416 } else { { let t1414 = v_N.clone(); Py::Bool({ let t1415 = Py::Int(1i64); if py::ge(&t1414, &t1415)? { true } else { false } }) } } } } } } }, Py::str("read length and read count are whole numbers"))?;
    let _ = f__pos(Py::str("the genome size G"), Py::tuple(vec![v_G.clone()]))?;
    v_c = py::truediv(&py::mul(&v_L.clone(), &v_N.clone())?, &v_G.clone())?;
    v_p0 = py::math1("exp", &py::neg(&v_c.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("coverage"), v_c.clone()), (Py::str("fraction_uncovered"), v_p0.clone()), (Py::str("bases_uncovered"), py::mul(&v_G.clone(), &v_p0.clone())?), (Py::str("contigs"), py::mul(&v_N.clone(), &v_p0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_growth_rate_fit(mut v_t: Py, mut v_N: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__need({ let t1467 = { let t1463 = py::len(&v_t.clone())?; Py::Bool({ let t1464 = py::len(&v_N.clone())?; if py::eq(&t1463, &t1464) { true } else { false } }) }; if !t1467.truthy() { t1467 } else { { let t1465 = py::len(&v_t.clone())?; Py::Bool({ let t1466 = Py::Int(3i64); if py::ge(&t1465, &t1466)? { true } else { false } }) } } }, Py::str("give at least 3 times and one count per time"))?;
    for t1468 in py::iter(&(v_N.clone()))? {
        v_v = t1468;
        let _ = f__need({ let t1473 = f__isnum(v_v.clone())?; if !t1473.truthy() { t1473 } else { { let t1471 = v_v.clone(); Py::Bool({ let t1472 = Py::Int(0i64); if py::gt(&t1471, &t1472)? { true } else { false } }) } } }, Py::str("counts must be positive"))?;
    }
    for t1474 in py::iter(&(v_t.clone()))? {
        v_v = t1474;
        let _ = f__fin(Py::tuple(vec![v_v.clone()]))?;
    }
    v_y = { let mut out = vec![]; for t1476 in py::iter(&(v_N.clone()))? { let mut v_v: Py; 
v_v = t1476;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) };
    let t1486 = (f__linfit(v_t.clone(), v_y.clone())?).items()?;
    if t1486.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1486[0].clone();
    v_a = t1486[1].clone();
    v_r2 = t1486[2].clone();
    let _ = f__need({ let t1487 = v_b.clone(); Py::Bool({ let t1488 = Py::Int(0i64); if py::gt(&t1487, &t1488)? { true } else { false } }) }, Py::str("the counts do not increase: no growth rate can be fitted"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("mu_per_h"), v_b.clone()), (Py::str("td_h"), py::truediv(&c_LN2()?, &v_b.clone())?), (Py::str("N0_fit"), py::math1("exp", &v_a.clone())?), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hemocytometer(mut v_counts: Py, mut v_dilution: Py, mut v_V: Py) -> R {
    let mut v_conc: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t1527 = py::len(&v_counts.clone())?; Py::Bool({ let t1528 = Py::Int(1i64); if py::ge(&t1527, &t1528)? { true } else { false } }) }, Py::str("give the counts of at least one square"))?;
    for t1529 in py::iter(&(v_counts.clone()))? {
        v_v = t1529;
        let _ = f__need({ let t1534 = f__isnum(v_v.clone())?; if !t1534.truthy() { t1534 } else { { let t1532 = v_v.clone(); Py::Bool({ let t1533 = Py::Int(0i64); if py::ge(&t1532, &t1533)? { true } else { false } }) } } }, Py::str("counts must be zero or positive"))?;
    }
    let _ = f__need({ let t1537 = f__isnum(v_dilution.clone())?; if !t1537.truthy() { t1537 } else { { let t1535 = v_dilution.clone(); Py::Bool({ let t1536 = Py::Int(1i64); if py::ge(&t1535, &t1536)? { true } else { false } }) } } }, Py::str("the dilution factor must be at least 1"))?;
    v_m = f__mean(v_counts.clone())?;
    v_conc = py::mul(&py::mul(&v_m.clone(), &v_dilution.clone())?, &Py::Float(10000.0))?;
    v_out = Py::dict(vec![(Py::str("mean_count"), v_m.clone()), (Py::str("cells_per_mL"), v_conc.clone())]);
    if ({ let t1539 = v_V.clone(); Py::Bool({ let t1540 = Py::None; if !py::is(&t1539, &t1540) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the suspension volume V"), Py::tuple(vec![v_V.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("total_cells"), py::mul(&py::mul(&v_conc.clone(), &v_V.clone())?, &Py::Float(1000000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_henderson_hasselbalch(mut v_pKa: Py, mut v_A: Py, mut v_HA: Py) -> R {
    let mut v_pH: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_pKa.clone()]))?;
    let _ = f__pos(Py::str("the base and acid concentrations"), Py::tuple(vec![v_A.clone(), v_HA.clone()]))?;
    v_pH = py::add(&v_pKa.clone(), &py::math1("log10", &py::truediv(&v_A.clone(), &v_HA.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("pH"), v_pH.clone()), (Py::str("ratio"), py::truediv(&v_A.clone(), &v_HA.clone())?), (Py::str("fraction_base"), py::truediv(&v_A.clone(), &py::add(&v_A.clone(), &v_HA.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_equation(mut v_L: Py, mut v_K: Py, mut v_n: Py) -> R {
    let mut v_theta: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__pos(Py::str("L, K and n"), Py::tuple(vec![v_L.clone(), v_K.clone(), v_n.clone()]))?;
    v_x = py::pow(&py::truediv(&v_L.clone(), &v_K.clone())?, &v_n.clone())?;
    v_theta = py::truediv(&v_x.clone(), &py::add(&Py::Float(1.0), &v_x.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("theta"), v_theta.clone()), (Py::str("theta_percent"), py::mul(&Py::Float(100.0), &v_theta.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_from_ec(mut v_EC10: Py, mut v_EC90: Py) -> R {
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("EC10 and EC90"), Py::tuple(vec![v_EC10.clone(), v_EC90.clone()]))?;
    let _ = f__need({ let t1635 = v_EC90.clone(); Py::Bool({ let t1636 = v_EC10.clone(); if py::gt(&t1635, &t1636)? { true } else { false } }) }, Py::str("EC90 must exceed EC10"))?;
    v_n = py::truediv(&py::math1("log", &Py::Float(81.0))?, &py::math1("log", &py::truediv(&v_EC90.clone(), &v_EC10.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("n"), v_n.clone()), (Py::str("EC50"), py::math1("sqrt", &py::mul(&v_EC10.clone(), &v_EC90.clone())?)?), (Py::str("ratio"), py::truediv(&v_EC90.clone(), &v_EC10.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hill_ligand_for_fraction(mut v_K: Py, mut v_n: Py, mut v_theta: Py) -> R {
    let mut v_L: Py = Py::None;
    let _ = f__pos(Py::str("K and n"), Py::tuple(vec![v_K.clone(), v_n.clone()]))?;
    let _ = f__frac(Py::str("the fractional saturation theta"), v_theta.clone(), Py::Bool(true), Py::Bool(true))?;
    v_L = py::mul(&v_K.clone(), &py::pow(&py::truediv(&v_theta.clone(), &py::sub(&Py::Float(1.0), &v_theta.clone())?)?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("L"), v_L.clone()), (Py::str("L_uM"), py::mul(&v_L.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ic50_to_ki(mut v_mode: Py, mut v_IC50: Py, mut v_S: Py, mut v_Km: Py) -> R {
    let mut v_Ki: Py = Py::None;
    let _ = f__need({ let t1772 = v_mode.clone(); Py::Bool({ let t1773 = c__INHIB()?; if py::contains(&t1773, &t1772)? { true } else { false } }) }, Py::str("mode: competitive, uncompetitive or noncompetitive"))?;
    let _ = f__pos(Py::str("IC50, S and Km"), Py::tuple(vec![v_IC50.clone(), v_S.clone(), v_Km.clone()]))?;
    if ({ let t1780 = v_mode.clone(); Py::Bool({ let t1781 = Py::str("competitive"); if py::eq(&t1780, &t1781) { true } else { false } }) }).truthy() {
        v_Ki = py::truediv(&v_IC50.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_S.clone(), &v_Km.clone())?)?)?;
    } else {
        if ({ let t1782 = v_mode.clone(); Py::Bool({ let t1783 = Py::str("uncompetitive"); if py::eq(&t1782, &t1783) { true } else { false } }) }).truthy() {
            v_Ki = py::truediv(&v_IC50.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_Km.clone(), &v_S.clone())?)?)?;
        } else {
            v_Ki = v_IC50.clone();
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Ki"), v_Ki.clone()), (Py::str("Ki_uM"), py::mul(&v_Ki.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_induction_dynamics(mut v_k_tx: Py, mut v_delta_m: Py, mut v_k_tl: Py, mut v_delta_p: Py, mut v_t: Py) -> R {
    let mut v_cross: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_p_ss: Py = Py::None;
    let _ = f__pos(Py::str("all rates"), Py::tuple(vec![v_k_tx.clone(), v_delta_m.clone(), v_k_tl.clone(), v_delta_p.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_m = py::mul(&py::truediv(&v_k_tx.clone(), &v_delta_m.clone())?, &py::sub(&Py::Float(1.0), &py::math1("exp", &py::mul(&py::neg(&v_delta_m.clone())?, &v_t.clone())?)?)?)?;
    v_d = py::sub(&v_delta_p.clone(), &v_delta_m.clone())?;
    if ({ let t1832 = py::abs(&v_d.clone())?; Py::Bool({ let t1833 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_delta_p.clone(), v_delta_m.clone()], true)?)?; if py::le(&t1832, &t1833)? { true } else { false } }) }).truthy() {
        v_cross = py::mul(&v_t.clone(), &py::math1("exp", &py::mul(&py::neg(&v_delta_m.clone())?, &v_t.clone())?)?)?;
    } else {
        v_cross = py::truediv(&py::sub(&py::math1("exp", &py::mul(&py::neg(&v_delta_m.clone())?, &v_t.clone())?)?, &py::math1("exp", &py::mul(&py::neg(&v_delta_p.clone())?, &v_t.clone())?)?)?, &v_d.clone())?;
    }
    v_p = py::mul(&py::truediv(&py::mul(&v_k_tl.clone(), &v_k_tx.clone())?, &v_delta_m.clone())?, &py::sub(&py::truediv(&py::sub(&Py::Float(1.0), &py::math1("exp", &py::mul(&py::neg(&v_delta_p.clone())?, &v_t.clone())?)?)?, &v_delta_p.clone())?, &v_cross.clone())?)?;
    v_p_ss = py::truediv(&py::mul(&v_k_tl.clone(), &v_k_tx.clone())?, &py::mul(&v_delta_m.clone(), &v_delta_p.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("p"), v_p.clone()), (Py::str("fraction_of_steady_state"), py::truediv(&v_p.clone(), &v_p_ss.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ionic_strength(mut v_c: Py, mut v_z: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t1876 = { let t1872 = py::len(&v_c.clone())?; Py::Bool({ let t1873 = py::len(&v_z.clone())?; if py::eq(&t1872, &t1873) { true } else { false } }) }; if !t1876.truthy() { t1876 } else { { let t1874 = py::len(&v_c.clone())?; Py::Bool({ let t1875 = Py::Int(1i64); if py::ge(&t1874, &t1875)? { true } else { false } }) } } }, Py::str("give one charge per concentration"))?;
    for t1877 in py::iter(&(v_c.clone()))? {
        v_v = t1877;
        let _ = f__need({ let t1882 = f__isnum(v_v.clone())?; if !t1882.truthy() { t1882 } else { { let t1880 = v_v.clone(); Py::Bool({ let t1881 = Py::Int(0i64); if py::ge(&t1880, &t1881)? { true } else { false } }) } } }, Py::str("concentrations must be zero or positive"))?;
    }
    for t1883 in py::iter(&(v_z.clone()))? {
        v_v = t1883;
        let _ = f__need({ let t1889 = f__isnum(v_v.clone())?; if !t1889.truthy() { t1889 } else { { let t1888 = { let t1884 = v_v.clone(); Py::Bool({ let t1885 = py::int(&v_v.clone())?; if py::eq(&t1884, &t1885) { true } else { false } }) }; if !t1888.truthy() { t1888 } else { { let t1886 = v_v.clone(); Py::Bool({ let t1887 = Py::Int(0i64); if py::ne(&t1886, &t1887) { true } else { false } }) } } } } }, Py::str("charges must be non-zero whole numbers"))?;
    }
    v_s = Py::Float(0.0);
    for t1890 in py::range_iter(&[(py::len(&v_c.clone())?).i()?]) {
        v_k = t1890;
        v_s = py::iadd(&v_s, &py::mul(&py::mul(&py::getitem(&v_c.clone(), &v_k.clone())?, &py::getitem(&v_z.clone(), &v_k.clone())?)?, &py::getitem(&v_z.clone(), &v_k.clone())?)?)?;
    }
    v_I = py::mul(&Py::Float(0.5), &v_s.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("I"), v_I.clone()), (Py::str("I_mM"), v_I.clone()), (Py::str("I_M"), py::truediv(&v_I.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kcat_efficiency(mut v_Vmax: Py, mut v_Et: Py, mut v_Km: Py) -> R {
    let mut v_eff: Py = Py::None;
    let mut v_kcat: Py = Py::None;
    let _ = f__pos(Py::str("Vmax, Et and Km"), Py::tuple(vec![v_Vmax.clone(), v_Et.clone(), v_Km.clone()]))?;
    v_kcat = py::truediv(&v_Vmax.clone(), &v_Et.clone())?;
    v_eff = py::truediv(&v_kcat.clone(), &v_Km.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("kcat"), v_kcat.clone()), (Py::str("kcat_per_s"), v_kcat.clone()), (Py::str("efficiency"), v_eff.clone()), (Py::str("efficiency_per_M_s"), py::mul(&v_eff.clone(), &Py::Float(1000.0))?), (Py::str("turnover_time"), py::truediv(&Py::Float(1.0), &v_kcat.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kd_from_rates(mut v_kon: Py, mut v_koff: Py) -> R {
    let mut v_Kd: Py = Py::None;
    let _ = f__pos(Py::str("kon and koff"), Py::tuple(vec![v_kon.clone(), v_koff.clone()]))?;
    v_Kd = py::truediv(&v_koff.clone(), &v_kon.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Kd"), v_Kd.clone()), (Py::str("Kd_nM"), py::mul(&v_Kd.clone(), &Py::Float(1000000.0))?), (Py::str("Ka_per_M"), py::truediv(&py::truediv(&v_kon.clone(), &v_koff.clone())?, &Py::Float(1000.0))?), (Py::str("half_life"), py::truediv(&c_LN2()?, &v_koff.clone())?), (Py::str("residence_time"), py::truediv(&Py::Float(1.0), &v_koff.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_library_size(mut v_G: Py, mut v_I: Py, mut v_P: Py) -> R {
    let mut v_clones: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("the genome size and the insert size"), Py::tuple(vec![v_G.clone(), v_I.clone()]))?;
    let _ = f__need({ let t2023 = v_I.clone(); Py::Bool({ let t2024 = v_G.clone(); if py::lt(&t2023, &t2024)? { true } else { false } }) }, Py::str("the insert must be smaller than the genome"))?;
    let _ = f__frac(Py::str("the probability P"), v_P.clone(), Py::Bool(true), Py::Bool(true))?;
    v_n = py::truediv(&py::math1("log", &py::sub(&Py::Float(1.0), &v_P.clone())?)?, &py::math1("log", &py::sub(&Py::Float(1.0), &py::truediv(&v_I.clone(), &v_G.clone())?)?)?)?;
    v_clones = py::int(&py::math1("ceil", &py::sub(&v_n.clone(), &Py::Float(1e-09))?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("clones_exact"), v_n.clone()), (Py::str("clones"), v_clones.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ligand_depletion_binding(mut v_Rt: Py, mut v_Lt: Py, mut v_Kd: Py) -> R {
    let mut v_RL: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("Rt, Lt and Kd"), Py::tuple(vec![v_Rt.clone(), v_Lt.clone(), v_Kd.clone()]))?;
    v_s = py::add(&py::add(&v_Rt.clone(), &v_Lt.clone())?, &v_Kd.clone())?;
    v_RL = py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &v_Rt.clone())?, &v_Lt.clone())?, &py::add(&v_s.clone(), &py::math1("sqrt", &py::sub(&py::mul(&v_s.clone(), &v_s.clone())?, &py::mul(&py::mul(&Py::Float(4.0), &v_Rt.clone())?, &v_Lt.clone())?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("RL"), v_RL.clone()), (Py::str("fraction_receptor_bound"), py::truediv(&v_RL.clone(), &v_Rt.clone())?), (Py::str("free_ligand"), py::sub(&v_Lt.clone(), &v_RL.clone())?), (Py::str("fraction_ligand_bound"), py::truediv(&v_RL.clone(), &v_Lt.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ligation_insert_mass(mut v_m_vector: Py, mut v_L_vector: Py, mut v_L_insert: Py, mut v_ratio: Py) -> R {
    let mut v_m_ins: Py = Py::None;
    let mut v_mol_vec: Py = Py::None;
    let _ = f__pos(Py::str("the vector mass, the lengths and the ratio"), Py::tuple(vec![v_m_vector.clone(), v_ratio.clone()]))?;
    let _ = f__need({ let t2128 = Py::Bool(matches!(v_L_vector.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2128.truthy() { t2128 } else { { let t2127 = Py::Bool(matches!(v_L_insert.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2127.truthy() { t2127 } else { { let t2126 = { let t2122 = v_L_vector.clone(); Py::Bool({ let t2123 = Py::Int(1i64); if py::ge(&t2122, &t2123)? { true } else { false } }) }; if !t2126.truthy() { t2126 } else { { let t2124 = v_L_insert.clone(); Py::Bool({ let t2125 = Py::Int(1i64); if py::ge(&t2124, &t2125)? { true } else { false } }) } } } } } } }, Py::str("lengths are whole numbers of bp"))?;
    v_m_ins = py::truediv(&py::mul(&py::mul(&v_ratio.clone(), &v_m_vector.clone())?, &v_L_insert.clone())?, &v_L_vector.clone())?;
    v_mol_vec = py::truediv(&py::mul(&v_m_vector.clone(), &Py::Float(1000.0))?, &py::mul(&Py::Float(660.0), &v_L_vector.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("m_insert"), v_m_ins.clone()), (Py::str("m_insert_ng"), py::mul(&v_m_ins.clone(), &Py::Float(1000000000000.0))?), (Py::str("vector_fmol"), py::mul(&v_mol_vec.clone(), &Py::Float(1000000000000000.0))?), (Py::str("insert_fmol"), py::mul(&py::mul(&v_ratio.clone(), &v_mol_vec.clone())?, &Py::Float(1000000000000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lineweaver_burk(mut v_S: Py, mut v_v: Py) -> R {
    let mut v_Vmax: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__need({ let t2171 = { let t2167 = py::len(&v_S.clone())?; Py::Bool({ let t2168 = py::len(&v_v.clone())?; if py::eq(&t2167, &t2168) { true } else { false } }) }; if !t2171.truthy() { t2171 } else { { let t2169 = py::len(&v_S.clone())?; Py::Bool({ let t2170 = Py::Int(3i64); if py::ge(&t2169, &t2170)? { true } else { false } }) } } }, Py::str("give at least 3 substrate concentrations with one rate each"))?;
    for t2172 in py::iter(&(v_S.clone()))? {
        v_a = t2172;
        let _ = f__need({ let t2177 = f__isnum(v_a.clone())?; if !t2177.truthy() { t2177 } else { { let t2175 = v_a.clone(); Py::Bool({ let t2176 = Py::Int(0i64); if py::gt(&t2175, &t2176)? { true } else { false } }) } } }, Py::str("substrate concentrations must be positive"))?;
    }
    for t2178 in py::iter(&(v_v.clone()))? {
        v_a = t2178;
        let _ = f__need({ let t2181 = f__isnum(v_a.clone())?; if !t2181.truthy() { t2181 } else { { let t2179 = v_a.clone(); Py::Bool({ let t2180 = Py::Int(0i64); if py::gt(&t2179, &t2180)? { true } else { false } }) } } }, Py::str("rates must be positive"))?;
    }
    v_x = { let mut out = vec![]; for t2182 in py::iter(&(v_S.clone()))? { let mut v_a: Py; 
v_a = t2182;
out.push(py::truediv(&Py::Float(1.0), &v_a.clone())?); } Py::list(out) };
    v_y = { let mut out = vec![]; for t2183 in py::iter(&(v_v.clone()))? { let mut v_a: Py; 
v_a = t2183;
out.push(py::truediv(&Py::Float(1.0), &v_a.clone())?); } Py::list(out) };
    let t2193 = (f__linfit(v_x.clone(), v_y.clone())?).items()?;
    if t2193.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2193[0].clone();
    v_b = t2193[1].clone();
    v_r2 = t2193[2].clone();
    let _ = f__need({ let t2198 = { let t2194 = v_b.clone(); Py::Bool({ let t2195 = Py::Int(0i64); if py::gt(&t2194, &t2195)? { true } else { false } }) }; if !t2198.truthy() { t2198 } else { { let t2196 = v_m.clone(); Py::Bool({ let t2197 = Py::Int(0i64); if py::gt(&t2196, &t2197)? { true } else { false } }) } } }, Py::str("the double-reciprocal line must have a positive slope and intercept"))?;
    v_Vmax = py::truediv(&Py::Float(1.0), &v_b.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Km"), py::truediv(&v_m.clone(), &v_b.clone())?), (Py::str("Vmax"), v_Vmax.clone()), (Py::str("slope"), v_m.clone()), (Py::str("intercept"), v_b.clone()), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logistic_growth(mut v_N0: Py, mut v_K: Py, mut v_r: Py, mut v_t: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_ae: Py = Py::None;
    let mut v_gap: Py = Py::None;
    let _ = f__pos(Py::str("N0, K and r"), Py::tuple(vec![v_N0.clone(), v_K.clone(), v_r.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    let _ = f__need({ let t2247 = v_N0.clone(); Py::Bool({ let t2248 = v_K.clone(); if py::lt(&t2247, &t2248)? { true } else { false } }) }, Py::str("the initial population must be below the carrying capacity"))?;
    v_a = py::truediv(&py::sub(&v_K.clone(), &v_N0.clone())?, &v_N0.clone())?;
    v_ae = py::mul(&v_a.clone(), &py::math1("exp", &py::mul(&py::neg(&v_r.clone())?, &v_t.clone())?)?)?;
    v_N = py::truediv(&v_K.clone(), &py::add(&Py::Float(1.0), &v_ae.clone())?)?;
    v_gap = py::truediv(&v_ae.clone(), &py::add(&Py::Float(1.0), &v_ae.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("dNdt"), py::mul(&py::mul(&v_r.clone(), &v_N.clone())?, &v_gap.clone())?), (Py::str("dNdt_per_h"), py::mul(&py::mul(&py::mul(&v_r.clone(), &v_N.clone())?, &v_gap.clone())?, &c_HOUR()?)?), (Py::str("t_inflection"), py::truediv(&py::math1("log", &v_a.clone())?, &v_r.clone())?), (Py::str("t_inflection_h"), py::truediv(&py::truediv(&py::math1("log", &v_a.clone())?, &v_r.clone())?, &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logistic_time_to_reach(mut v_N0: Py, mut v_K: Py, mut v_r: Py, mut v_N: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("N0, K, r and N"), Py::tuple(vec![v_N0.clone(), v_K.clone(), v_r.clone(), v_N.clone()]))?;
    let _ = f__need({ let t2293 = v_N0.clone(); Py::Bool({ let t2294 = v_N.clone(); if py::lt(&t2293, &t2294)? { { let t2295 = v_K.clone(); if py::lt(&t2294, &t2295)? { true } else { false } } } else { false } }) }, Py::str("the target must lie between the initial population and the carrying capacity"))?;
    v_t = py::truediv(&py::math1("log", &py::truediv(&py::mul(&v_N.clone(), &py::sub(&v_K.clone(), &v_N0.clone())?)?, &py::mul(&v_N0.clone(), &py::sub(&v_K.clone(), &v_N.clone())?)?)?)?, &v_r.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_h"), py::truediv(&v_t.clone(), &c_HOUR()?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mass_for_molarity(mut v_C: Py, mut v_V: Py, mut v_MW: Py) -> R {
    let mut v_m: Py = Py::None;
    let _ = f__pos(Py::str("C, V and MW"), Py::tuple(vec![v_C.clone(), v_V.clone(), v_MW.clone()]))?;
    v_m = py::mul(&py::mul(&v_C.clone(), &v_V.clone())?, &v_MW.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("m"), v_m.clone()), (Py::str("m_g"), py::mul(&v_m.clone(), &Py::Float(1000.0))?), (Py::str("m_mg"), py::mul(&v_m.clone(), &Py::Float(1000000.0))?), (Py::str("n"), py::mul(&v_C.clone(), &v_V.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_membrane_permeability(mut v_D: Py, mut v_K: Py, mut v_L: Py, mut v_C_out: Py, mut v_C_in: Py) -> R {
    let mut v_J: Py = Py::None;
    let mut v_P: Py = Py::None;
    let _ = f__pos(Py::str("D, K and L"), Py::tuple(vec![v_D.clone(), v_K.clone(), v_L.clone()]))?;
    let _ = f__nonneg(Py::str("the concentrations"), Py::tuple(vec![v_C_out.clone(), v_C_in.clone()]))?;
    v_P = py::truediv(&py::mul(&v_K.clone(), &v_D.clone())?, &v_L.clone())?;
    v_J = py::mul(&v_P.clone(), &py::sub(&v_C_out.clone(), &v_C_in.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("P_cm_s"), py::mul(&v_P.clone(), &Py::Float(100.0))?), (Py::str("J"), v_J.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_michaelis_menten(mut v_Vmax: Py, mut v_Km: Py, mut v_S: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("Vmax and Km"), Py::tuple(vec![v_Vmax.clone(), v_Km.clone()]))?;
    let _ = f__nonneg(Py::str("the substrate concentration S"), Py::tuple(vec![v_S.clone()]))?;
    v_v = py::truediv(&py::mul(&v_Vmax.clone(), &v_S.clone())?, &py::add(&v_Km.clone(), &v_S.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("v_uM_min"), py::mul(&py::mul(&v_v.clone(), &Py::Float(1000.0))?, &c_MINUTE()?)?), (Py::str("fraction"), py::truediv(&v_S.clone(), &py::add(&v_Km.clone(), &v_S.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mixed_inhibition(mut v_Vmax: Py, mut v_Km: Py, mut v_S: Py, mut v_I: Py, mut v_Ki: Py, mut v_Kii: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_ap: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("Vmax, Km, Ki and Kii"), Py::tuple(vec![v_Vmax.clone(), v_Km.clone(), v_Ki.clone(), v_Kii.clone()]))?;
    let _ = f__nonneg(Py::str("S and I"), Py::tuple(vec![v_S.clone(), v_I.clone()]))?;
    let t2588 = (Py::tuple(vec![py::add(&Py::Float(1.0), &py::truediv(&v_I.clone(), &v_Ki.clone())?)?, py::add(&Py::Float(1.0), &py::truediv(&v_I.clone(), &v_Kii.clone())?)?])).items()?;
    if t2588.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2588[0].clone();
    v_ap = t2588[1].clone();
    v_v = py::truediv(&py::mul(&v_Vmax.clone(), &v_S.clone())?, &py::add(&py::mul(&v_Km.clone(), &v_a.clone())?, &py::mul(&v_S.clone(), &v_ap.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("Km_app"), py::truediv(&py::mul(&v_Km.clone(), &v_a.clone())?, &v_ap.clone())?), (Py::str("Vmax_app"), py::truediv(&v_Vmax.clone(), &v_ap.clone())?), (Py::str("alpha"), py::truediv(&v_Kii.clone(), &v_Ki.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mm_nonlinear_fit(mut v_S: Py, mut v_v: Py) -> R {
    let mut v_Km: Py = Py::None;
    let mut v_Vmax: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f1: Py = Py::None;
    let mut v_f2: Py = Py::None;
    let mut v_fbest: Py = Py::None;
    let mut v_grid: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mv: Py = Py::None;
    let mut v_obj: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rss: Py = Py::None;
    let mut v_sst: Py = Py::None;
    let _ = f__need({ let t2430 = { let t2426 = py::len(&v_S.clone())?; Py::Bool({ let t2427 = py::len(&v_v.clone())?; if py::eq(&t2426, &t2427) { true } else { false } }) }; if !t2430.truthy() { t2430 } else { { let t2428 = py::len(&v_S.clone())?; Py::Bool({ let t2429 = Py::Int(3i64); if py::ge(&t2428, &t2429)? { true } else { false } }) } } }, Py::str("give at least 3 substrate concentrations with one rate each"))?;
    for t2431 in py::iter(&(v_S.clone()))? {
        v_a = t2431;
        let _ = f__need({ let t2436 = f__isnum(v_a.clone())?; if !t2436.truthy() { t2436 } else { { let t2434 = v_a.clone(); Py::Bool({ let t2435 = Py::Int(0i64); if py::gt(&t2434, &t2435)? { true } else { false } }) } } }, Py::str("substrate concentrations must be positive"))?;
    }
    for t2437 in py::iter(&(v_v.clone()))? {
        v_a = t2437;
        let _ = f__need({ let t2440 = f__isnum(v_a.clone())?; if !t2440.truthy() { t2440 } else { { let t2438 = v_a.clone(); Py::Bool({ let t2439 = Py::Int(0i64); if py::gt(&t2438, &t2439)? { true } else { false } }) } } }, Py::str("rates must be positive"))?;
    }
    let t2441 = (Py::tuple(vec![py::mul(&py::minmax(&[v_S.clone()], false)?, &Py::Float(0.0001))?, py::mul(&py::minmax(&[v_S.clone()], true)?, &Py::Float(10000.0))?])).items()?;
    if t2441.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2441[0].clone();
    v_hi = t2441[1].clone();
    v_grid = { let mut out = vec![]; for t2442 in py::range_iter(&[(Py::Int(401i64)).i()?]) { let mut v_k: Py; 
v_k = t2442;
out.push(py::mul(&v_lo.clone(), &py::pow(&py::truediv(&v_hi.clone(), &v_lo.clone())?, &py::truediv(&v_k.clone(), &Py::Float(400.0))?)?)?); } Py::list(out) };
    let t2443 = (Py::tuple(vec![Py::None, Py::None])).items()?;
    if t2443.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_best = t2443[0].clone();
    v_fbest = t2443[1].clone();
    for t2444 in py::range_iter(&[(Py::Int(400i64)).i()?]) {
        v_k = t2444;
        let t2447 = (Py::tuple(vec![f__mm_dprofile(v_S.clone(), v_v.clone(), py::getitem(&v_grid.clone(), &v_k.clone())?)?, f__mm_dprofile(v_S.clone(), v_v.clone(), py::getitem(&v_grid.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?])).items()?;
        if t2447.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f1 = t2447[0].clone();
        v_f2 = t2447[1].clone();
        if ({ let t2448 = v_f1.clone(); Py::Bool({ let t2449 = Py::Int(0i64); if py::lt(&t2448, &t2449)? { { let t2450 = v_f2.clone(); if py::le(&t2449, &t2450)? { true } else { false } } } else { false } }) }).truthy() {
            v_r = f__bisect({ let v_S = v_S.clone(); let v_a = v_a.clone(); let v_best = v_best.clone(); let v_f1 = v_f1.clone(); let v_f2 = v_f2.clone(); let v_fbest = v_fbest.clone(); let v_grid = v_grid.clone(); let v_hi = v_hi.clone(); let v_k = v_k.clone(); let v_lo = v_lo.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__mm_dprofile(v_S.clone(), v_v.clone(), v_x.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::getitem(&v_grid.clone(), &v_k.clone())?, py::getitem(&v_grid.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, Py::Int(400i64))?;
            let t2479 = (f__mm_profile(v_S.clone(), v_v.clone(), v_r.clone())?).items()?;
            if t2479.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2479[0].clone();
            v_b = t2479[1].clone();
            v_obj = py::truediv(&py::mul(&py::neg(&v_a.clone())?, &v_a.clone())?, &v_b.clone())?;
            if ({ let t2484 = { let t2480 = v_best.clone(); Py::Bool({ let t2481 = Py::None; if py::is(&t2480, &t2481) { true } else { false } }) }; if t2484.truthy() { t2484 } else { { let t2482 = v_obj.clone(); Py::Bool({ let t2483 = v_fbest.clone(); if py::lt(&t2482, &t2483)? { true } else { false } }) } } }).truthy() {
                let t2485 = (Py::tuple(vec![v_r.clone(), v_obj.clone()])).items()?;
                if t2485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_best = t2485[0].clone();
                v_fbest = t2485[1].clone();
            }
        }
    }
    let _ = f__need({ let t2486 = v_best.clone(); Py::Bool({ let t2487 = Py::None; if !py::is(&t2486, &t2487) { true } else { false } }) }, Py::str("the data do not define a Michaelis-Menten curve (no interior least-squares minimum)"))?;
    v_Km = v_best.clone();
    let t2488 = (f__mm_profile(v_S.clone(), v_v.clone(), v_Km.clone())?).items()?;
    if t2488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2488[0].clone();
    v_b = t2488[1].clone();
    v_Vmax = py::truediv(&v_a.clone(), &v_b.clone())?;
    let t2490 = (Py::tuple(vec![Py::Float(0.0), f__mean(v_v.clone())?])).items()?;
    if t2490.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rss = t2490[0].clone();
    v_mv = t2490[1].clone();
    v_sst = Py::Float(0.0);
    for t2491 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) {
        v_k = t2491;
        v_e = py::sub(&py::getitem(&v_v.clone(), &v_k.clone())?, &py::truediv(&py::mul(&v_Vmax.clone(), &py::getitem(&v_S.clone(), &v_k.clone())?)?, &py::add(&v_Km.clone(), &py::getitem(&v_S.clone(), &v_k.clone())?)?)?)?;
        v_rss = py::iadd(&v_rss, &py::mul(&v_e.clone(), &v_e.clone())?)?;
        v_sst = py::iadd(&v_sst, &py::pow(&py::sub(&py::getitem(&v_v.clone(), &v_k.clone())?, &v_mv.clone())?, &Py::Int(2i64))?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Km"), v_Km.clone()), (Py::str("Vmax"), v_Vmax.clone()), (Py::str("rss"), v_rss.clone()), (Py::str("r2"), py::sub(&Py::Float(1.0), &py::truediv(&v_rss.clone(), &v_sst.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molarity_from_mass(mut v_m: Py, mut v_MW: Py, mut v_V: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__pos(Py::str("m, MW and V"), Py::tuple(vec![v_m.clone(), v_MW.clone(), v_V.clone()]))?;
    v_C = py::truediv(&py::truediv(&v_m.clone(), &v_MW.clone())?, &v_V.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_mM"), v_C.clone()), (Py::str("C_M"), py::truediv(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("n"), py::truediv(&v_m.clone(), &v_MW.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_molecules_per_cell(mut v_c: Py, mut v_V: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_one: Py = Py::None;
    let _ = f__pos(Py::str("the concentration c and the volume V"), Py::tuple(vec![v_c.clone(), v_V.clone()]))?;
    v_N = py::mul(&py::mul(&v_c.clone(), &v_V.clone())?, &c_N_A()?)?;
    v_one = py::truediv(&Py::Float(1.0), &py::mul(&v_V.clone(), &c_N_A()?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), v_N.clone()), (Py::str("one_molecule"), v_one.clone()), (Py::str("one_molecule_nM"), py::mul(&v_one.clone(), &Py::Float(1000000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_monod_batch_time(mut v_X0: Py, mut v_S0: Py, mut v_Y: Py, mut v_mu_max: Py, mut v_Ks: Py, mut v_X: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("X0, S0, Y, mu_max and Ks"), Py::tuple(vec![v_X0.clone(), v_S0.clone(), v_Y.clone(), v_mu_max.clone(), v_Ks.clone()]))?;
    v_B = py::add(&v_X0.clone(), &py::mul(&v_Y.clone(), &v_S0.clone())?)?;
    let _ = f__need({ let t2721 = v_X0.clone(); Py::Bool({ let t2722 = v_X.clone(); if py::lt(&t2721, &t2722)? { { let t2723 = v_B.clone(); if py::lt(&t2722, &t2723)? { true } else { false } } } else { false } }) }, Py::str("the target biomass must lie between X0 and X0 + Y S0"))?;
    v_S = py::truediv(&py::sub(&v_B.clone(), &v_X.clone())?, &v_Y.clone())?;
    v_t = py::truediv(&py::add(&py::math1("log", &py::truediv(&v_X.clone(), &v_X0.clone())?)?, &py::mul(&py::truediv(&py::mul(&v_Ks.clone(), &v_Y.clone())?, &v_B.clone())?, &py::sub(&py::math1("log", &py::truediv(&v_X.clone(), &v_X0.clone())?)?, &py::math1("log", &py::truediv(&py::sub(&v_B.clone(), &v_X.clone())?, &py::sub(&v_B.clone(), &v_X0.clone())?)?)?)?)?)?, &v_mu_max.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_h"), py::truediv(&v_t.clone(), &c_HOUR()?)?), (Py::str("S"), v_S.clone()), (Py::str("S_g_L"), v_S.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_monod_growth(mut v_mu_max: Py, mut v_S: Py, mut v_Ks: Py) -> R {
    let mut v_mu: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("mu_max and Ks"), Py::tuple(vec![v_mu_max.clone(), v_Ks.clone()]))?;
    let _ = f__nonneg(Py::str("the substrate concentration S"), Py::tuple(vec![v_S.clone()]))?;
    v_mu = py::truediv(&py::mul(&v_mu_max.clone(), &v_S.clone())?, &py::add(&v_Ks.clone(), &v_S.clone())?)?;
    v_out = Py::dict(vec![(Py::str("mu"), v_mu.clone()), (Py::str("mu_per_h"), py::mul(&v_mu.clone(), &c_HOUR()?)?), (Py::str("fraction"), py::truediv(&v_S.clone(), &py::add(&v_Ks.clone(), &v_S.clone())?)?)]);
    py::setitem(&v_out.clone(), &Py::str("td_h"), (if ({ let t2772 = v_mu.clone(); Py::Bool({ let t2773 = Py::Int(0i64); if py::gt(&t2772, &t2773)? { true } else { false } }) }).truthy() { py::truediv(&py::truediv(&c_LN2()?, &v_mu.clone())?, &c_HOUR()?)? } else { Py::Float(0.0) }))?;
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mrna_half_life_fit(mut v_t: Py, mut v_level: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t2817 = { let t2813 = py::len(&v_t.clone())?; Py::Bool({ let t2814 = py::len(&v_level.clone())?; if py::eq(&t2813, &t2814) { true } else { false } }) }; if !t2817.truthy() { t2817 } else { { let t2815 = py::len(&v_t.clone())?; Py::Bool({ let t2816 = Py::Int(3i64); if py::ge(&t2815, &t2816)? { true } else { false } }) } } }, Py::str("give at least 3 times and one level per time"))?;
    for t2818 in py::iter(&(v_level.clone()))? {
        v_v = t2818;
        let _ = f__need({ let t2823 = f__isnum(v_v.clone())?; if !t2823.truthy() { t2823 } else { { let t2821 = v_v.clone(); Py::Bool({ let t2822 = Py::Int(0i64); if py::gt(&t2821, &t2822)? { true } else { false } }) } } }, Py::str("levels must be positive"))?;
    }
    let t2834 = (f__linfit(v_t.clone(), { let mut out = vec![]; for t2833 in py::iter(&(v_level.clone()))? { let mut v_v: Py; 
v_v = t2833;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) })?).items()?;
    if t2834.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t2834[0].clone();
    v_a = t2834[1].clone();
    v_r2 = t2834[2].clone();
    let _ = f__need({ let t2835 = v_b.clone(); Py::Bool({ let t2836 = Py::Int(0i64); if py::lt(&t2835, &t2836)? { true } else { false } }) }, Py::str("the levels do not decay: no half-life can be fitted"))?;
    v_k = py::neg(&v_b.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("k_per_min"), v_k.clone()), (Py::str("half_life_min"), py::truediv(&c_LN2()?, &v_k.clone())?), (Py::str("initial_fit"), py::math1("exp", &v_a.clone())?), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mutation_rate_p0(mut v_n0: Py, mut v_n: Py, mut v_Nt: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let _ = f__need({ let t2882 = Py::Bool(matches!(v_n0.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2882.truthy() { t2882 } else { { let t2881 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2881.truthy() { t2881 } else { { let t2880 = { let t2875 = v_n.clone(); Py::Bool({ let t2876 = Py::Int(1i64); if py::ge(&t2875, &t2876)? { true } else { false } }) }; if !t2880.truthy() { t2880 } else { { let t2877 = Py::Int(1i64); Py::Bool({ let t2878 = v_n0.clone(); if py::le(&t2877, &t2878)? { { let t2879 = v_n.clone(); if py::lt(&t2878, &t2879)? { true } else { false } } } else { false } }) } } } } } } }, Py::str("n0 must be a whole number between 1 and n - 1"))?;
    let _ = f__pos(Py::str("the cells per culture Nt"), Py::tuple(vec![v_Nt.clone()]))?;
    v_p0 = py::truediv(&v_n0.clone(), &v_n.clone())?;
    v_m = py::neg(&py::math1("log", &v_p0.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("p0"), v_p0.clone()), (Py::str("m"), v_m.clone()), (Py::str("mutation_rate"), py::truediv(&v_m.clone(), &v_Nt.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mwc_saturation(mut v_S: Py, mut v_KR: Py, mut v_c: Py, mut v_L0: Py, mut v_n: Py) -> R {
    let mut v_Z: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_num: Py = Py::None;
    let _ = f__pos(Py::str("S, KR, c and L0"), Py::tuple(vec![v_S.clone(), v_KR.clone(), v_c.clone(), v_L0.clone()]))?;
    let _ = f__need({ let t2936 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2936.truthy() { t2936 } else { { let t2933 = Py::Int(1i64); Py::Bool({ let t2934 = v_n.clone(); if py::le(&t2933, &t2934)? { { let t2935 = Py::Int(12i64); if py::le(&t2934, &t2935)? { true } else { false } } } else { false } }) } } }, Py::str("the number of sites n must be a whole number from 1 to 12"))?;
    v_a = py::truediv(&v_S.clone(), &v_KR.clone())?;
    v_num = py::add(&py::mul(&v_a.clone(), &py::pow(&py::add(&Py::Float(1.0), &v_a.clone())?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::mul(&py::mul(&v_L0.clone(), &v_c.clone())?, &v_a.clone())?, &py::pow(&py::add(&Py::Float(1.0), &py::mul(&v_c.clone(), &v_a.clone())?)?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?;
    v_Z = py::add(&py::pow(&py::add(&Py::Float(1.0), &v_a.clone())?, &v_n.clone())?, &py::mul(&v_L0.clone(), &py::pow(&py::add(&Py::Float(1.0), &py::mul(&v_c.clone(), &v_a.clone())?)?, &v_n.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Y"), py::truediv(&v_num.clone(), &v_Z.clone())?), (Py::str("R_fraction"), py::truediv(&py::pow(&py::add(&Py::Float(1.0), &v_a.clone())?, &v_n.clone())?, &v_Z.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nernst_potential(mut v_z: Py, mut v_C_out: Py, mut v_C_in: Py, mut v_T: Py) -> R {
    let mut v_E: Py = Py::None;
    let _ = f__need({ let t2978 = Py::Bool(matches!(v_z.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2978.truthy() { t2978 } else { { let t2977 = py::not(&Py::Bool(matches!(v_z.clone(), Py::Bool(_)))); if !t2977.truthy() { t2977 } else { { let t2975 = v_z.clone(); Py::Bool({ let t2976 = Py::Int(0i64); if py::ne(&t2975, &t2976) { true } else { false } }) } } } } }, Py::str("the charge number z must be a non-zero whole number"))?;
    let _ = f__pos(Py::str("the concentrations and T"), Py::tuple(vec![v_C_out.clone(), v_C_in.clone(), v_T.clone()]))?;
    v_E = py::mul(&py::truediv(&py::mul(&c_R_GAS()?, &v_T.clone())?, &py::mul(&v_z.clone(), &c_FARADAY()?)?)?, &py::math1("log", &py::truediv(&v_C_out.clone(), &v_C_in.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("E"), v_E.clone()), (Py::str("E_mV"), py::mul(&v_E.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nucleic_acid_a260(mut v_A260: Py, mut v_molecule: Py, mut v_dilution: Py, mut v_l: Py, mut v_V: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_ug_ml: Py = Py::None;
    let _ = f__nonneg(Py::str("A260"), Py::tuple(vec![v_A260.clone()]))?;
    let _ = f__need({ let t3087 = v_molecule.clone(); Py::Bool({ let t3088 = c__A260_FACTOR()?; if py::contains(&t3088, &t3087)? { true } else { false } }) }, Py::str("molecule: dsDNA, ssDNA or RNA"))?;
    let _ = f__need({ let t3091 = f__isnum(v_dilution.clone())?; if !t3091.truthy() { t3091 } else { { let t3089 = v_dilution.clone(); Py::Bool({ let t3090 = Py::Int(1i64); if py::ge(&t3089, &t3090)? { true } else { false } }) } } }, Py::str("the dilution factor must be at least 1"))?;
    let _ = f__pos(Py::str("the path length l"), Py::tuple(vec![v_l.clone()]))?;
    v_ug_ml = py::truediv(&py::mul(&py::mul(&v_A260.clone(), &py::getitem(&c__A260_FACTOR()?, &v_molecule.clone())?)?, &v_dilution.clone())?, &py::mul(&v_l.clone(), &Py::Float(100.0))?)?;
    v_out = Py::dict(vec![(Py::str("c"), py::mul(&v_ug_ml.clone(), &Py::Float(0.001))?), (Py::str("c_ug_mL"), v_ug_ml.clone())]);
    if ({ let t3096 = v_V.clone(); Py::Bool({ let t3097 = Py::None; if !py::is(&t3096, &t3097) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the volume V"), Py::tuple(vec![v_V.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("mass_ug"), py::mul(&py::mul(&v_ug_ml.clone(), &v_V.clone())?, &Py::Float(1000000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oligo_extinction(mut v_seq: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_mw: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3152 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t3152.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3152[0].clone();
    v__ = t3152[1].clone();
    v_n = py::len(&v_s.clone())?;
    if ({ let t3153 = v_n.clone(); Py::Bool({ let t3154 = Py::Int(1i64); if py::eq(&t3153, &t3154) { true } else { false } }) }).truthy() {
        v_eps = py::getitem(&c__EPS1()?, &v_s.clone())?;
    } else {
        v_eps = Py::Float(0.0);
        for t3155 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t3155;
            v_eps = py::iadd(&v_eps, &py::getitem(&c__EPS2()?, &py::slice(&v_s.clone(), Some((v_i.clone()).i()?), Some((py::add(&v_i.clone(), &Py::Int(2i64))?).i()?), None)?)?)?;
        }
        for t3156 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t3156;
            v_eps = py::sub(&v_eps, &py::getitem(&c__EPS1()?, &py::getitem(&v_s.clone(), &v_i.clone())?)?)?;
        }
    }
    v_mw = py::sub(&f__strand_mw(v_s.clone(), Py::Bool(false))?, &c__HPO3()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("eps"), py::truediv(&v_eps.clone(), &Py::Float(10.0))?), (Py::str("eps_L_mol_cm"), v_eps.clone()), (Py::str("ug_per_A260"), py::mul(&py::truediv(&v_mw.clone(), &v_eps.clone())?, &Py::Float(1000.0))?), (Py::str("nmol_per_A260"), py::truediv(&Py::Float(1000000.0), &v_eps.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oligo_molecular_weight(mut v_seq: Py, mut v_strands: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_ch: Py = Py::None;
    let mut v_comp: Py = Py::None;
    let mut v_mw: Py = Py::None;
    let mut v_n_str: Py = Py::None;
    let mut v_rna: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3211 = (f__clean_seq(v_seq.clone(), Py::Bool(true), Py::Bool(true))?).items()?;
    if t3211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3211[0].clone();
    v_rna = t3211[1].clone();
    let _ = f__need({ let t3212 = v_strands.clone(); Py::Bool({ let t3213 = Py::tuple(vec![Py::str("single"), Py::str("double")]); if py::contains(&t3213, &t3212)? { true } else { false } }) }, Py::str("strands: single or double"))?;
    v_mw = f__strand_mw(v_s.clone(), v_rna.clone())?;
    v_n_str = Py::Int(1i64);
    if ({ let t3215 = v_strands.clone(); Py::Bool({ let t3216 = Py::str("double"); if py::eq(&t3215, &t3216) { true } else { false } }) }).truthy() {
        v_comp = (if (v_rna.clone()).truthy() { c__RNA_COMP()? } else { c__DNA_COMP()? });
        v_c = Py::str("");
        for t3217 in py::iter(&(v_s.clone()))? {
            v_ch = t3217;
            v_c = py::iadd(&v_c, &py::getitem(&v_comp.clone(), &v_ch.clone())?)?;
        }
        v_mw = py::iadd(&v_mw, &f__strand_mw(v_c.clone(), v_rna.clone())?)?;
        v_n_str = Py::Int(2i64);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("MW"), py::truediv(&v_mw.clone(), &Py::Float(1000.0))?), (Py::str("MW_g_mol"), v_mw.clone()), (Py::str("MW_hydroxyl_g_mol"), py::sub(&v_mw.clone(), &py::mul(&v_n_str.clone(), &c__HPO3()?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_orf_finder(mut v_seq: Py, mut v_min_aa: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_aa: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_cand: Py = Py::None;
    let mut v_cod: Py = Py::None;
    let mut v_count: Py = Py::None;
    let mut v_en: Py = Py::None;
    let mut v_frame: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n_aa: Py = Py::None;
    let mut v_prot: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_st: Py = Py::None;
    let mut v_start: Py = Py::None;
    let mut v_strand: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t3271 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t3271.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3271[0].clone();
    v__ = t3271[1].clone();
    let _ = f__need({ let t3274 = Py::Bool(matches!(v_min_aa.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3274.truthy() { t3274 } else { { let t3272 = v_min_aa.clone(); Py::Bool({ let t3273 = Py::Int(1i64); if py::ge(&t3272, &t3273)? { true } else { false } }) } } }, Py::str("min_aa must be a whole number of codons, at least 1"))?;
    v_L = py::len(&v_s.clone())?;
    v_best = Py::None;
    v_count = Py::Int(0i64);
    for t3275 in py::iter(&(Py::tuple(vec![Py::str("+"), Py::str("-")])))? {
        v_strand = t3275;
        v_t = (if ({ let t3276 = v_strand.clone(); Py::Bool({ let t3277 = Py::str("+"); if py::eq(&t3276, &t3277) { true } else { false } }) }).truthy() { v_s.clone() } else { f__revcomp(v_s.clone(), Py::Bool(false))? });
        for t3279 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v_frame = t3279;
            v_start = py::neg(&Py::Int(1i64))?;
            v_k = v_frame.clone();
            while ({ let t3303 = py::add(&v_k.clone(), &Py::Int(3i64))?; Py::Bool({ let t3304 = v_L.clone(); if py::le(&t3303, &t3304)? { true } else { false } }) }).truthy() {
                v_cod = py::slice(&v_t.clone(), Some((v_k.clone()).i()?), Some((py::add(&v_k.clone(), &Py::Int(3i64))?).i()?), None)?;
                v_aa = py::getitem(&c__CODONS()?, &v_cod.clone())?;
                if ({ let t3280 = v_aa.clone(); Py::Bool({ let t3281 = Py::str("*"); if py::eq(&t3280, &t3281) { true } else { false } }) }).truthy() {
                    if ({ let t3282 = v_start.clone(); Py::Bool({ let t3283 = Py::Int(0i64); if py::ge(&t3282, &t3283)? { true } else { false } }) }).truthy() {
                        v_n_aa = py::floordiv(&py::sub(&v_k.clone(), &v_start.clone())?, &Py::Int(3i64))?;
                        if ({ let t3284 = v_n_aa.clone(); Py::Bool({ let t3285 = v_min_aa.clone(); if py::ge(&t3284, &t3285)? { true } else { false } }) }).truthy() {
                            v_count = py::iadd(&v_count, &Py::Int(1i64))?;
                            v_prot = Py::str("");
                            for t3286 in py::range_iter(&[(v_start.clone()).i()?, (v_k.clone()).i()?, (Py::Int(3i64)).i()?]) {
                                v_q = t3286;
                                v_prot = py::iadd(&v_prot, &py::getitem(&c__CODONS()?, &py::slice(&v_t.clone(), Some((v_q.clone()).i()?), Some((py::add(&v_q.clone(), &Py::Int(3i64))?).i()?), None)?)?)?;
                            }
                            v_cand = Py::tuple(vec![v_n_aa.clone(), v_strand.clone(), py::add(&v_start.clone(), &Py::Int(1i64))?, py::add(&v_k.clone(), &Py::Int(3i64))?, v_prot.clone()]);
                            if ({ let t3297 = { let t3287 = v_best.clone(); Py::Bool({ let t3288 = Py::None; if py::is(&t3287, &t3288) { true } else { false } }) }; if t3297.truthy() { t3297 } else { { let t3296 = { let t3289 = v_n_aa.clone(); Py::Bool({ let t3290 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::gt(&t3289, &t3290)? { true } else { false } }) }; if t3296.truthy() { t3296 } else { { let t3295 = { let t3291 = v_n_aa.clone(); Py::Bool({ let t3292 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::eq(&t3291, &t3292) { true } else { false } }) }; if !t3295.truthy() { t3295 } else { { let t3293 = Py::tuple(vec![v_strand.clone(), py::add(&v_start.clone(), &Py::Int(1i64))?]); Py::Bool({ let t3294 = Py::tuple(vec![py::getitem(&v_best.clone(), &Py::Int(1i64))?, py::getitem(&v_best.clone(), &Py::Int(2i64))?]); if py::lt(&t3293, &t3294)? { true } else { false } }) } } } } } } }).truthy() {
                                v_best = v_cand.clone();
                            }
                        }
                    }
                    v_start = py::neg(&Py::Int(1i64))?;
                } else {
                    if ({ let t3302 = { let t3298 = v_cod.clone(); Py::Bool({ let t3299 = Py::str("ATG"); if py::eq(&t3298, &t3299) { true } else { false } }) }; if !t3302.truthy() { t3302 } else { { let t3300 = v_start.clone(); Py::Bool({ let t3301 = Py::Int(0i64); if py::lt(&t3300, &t3301)? { true } else { false } }) } } }).truthy() {
                        v_start = v_k.clone();
                    }
                }
                v_k = py::iadd(&v_k, &Py::Int(3i64))?;
            }
        }
    }
    let _ = f__need({ let t3305 = v_best.clone(); Py::Bool({ let t3306 = Py::None; if !py::is(&t3305, &t3306) { true } else { false } }) }, Py::str("no open reading frame of at least min_aa codons"))?;
    let t3307 = (v_best.clone()).items()?;
    if t3307.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_n_aa = t3307[0].clone();
    v_strand = t3307[1].clone();
    v_st = t3307[2].clone();
    v_en = t3307[3].clone();
    v_prot = t3307[4].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("strand"), v_strand.clone()), (Py::str("start"), v_st.clone()), (Py::str("end"), v_en.clone()), (Py::str("length_nt"), py::add(&py::sub(&v_en.clone(), &v_st.clone())?, &Py::Int(1i64))?), (Py::str("length_aa"), v_n_aa.clone()), (Py::str("protein"), v_prot.clone()), (Py::str("n_orfs"), v_count.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_osmolarity_tonicity(mut v_c: Py, mut v_i: Py, mut v_osm_cell: Py) -> R {
    let mut v_cell: Py = Py::None;
    let mut v_diff: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_osm: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_word: Py = Py::None;
    let _ = f__need({ let t3350 = { let t3346 = py::len(&v_c.clone())?; Py::Bool({ let t3347 = py::len(&v_i.clone())?; if py::eq(&t3346, &t3347) { true } else { false } }) }; if !t3350.truthy() { t3350 } else { { let t3348 = py::len(&v_c.clone())?; Py::Bool({ let t3349 = Py::Int(1i64); if py::ge(&t3348, &t3349)? { true } else { false } }) } } }, Py::str("give one van 't Hoff factor per concentration"))?;
    for t3351 in py::iter(&(v_c.clone()))? {
        v_v = t3351;
        let _ = f__need({ let t3356 = f__isnum(v_v.clone())?; if !t3356.truthy() { t3356 } else { { let t3354 = v_v.clone(); Py::Bool({ let t3355 = Py::Int(0i64); if py::ge(&t3354, &t3355)? { true } else { false } }) } } }, Py::str("concentrations must be zero or positive"))?;
    }
    for t3357 in py::iter(&(v_i.clone()))? {
        v_v = t3357;
        let _ = f__need({ let t3360 = f__isnum(v_v.clone())?; if !t3360.truthy() { t3360 } else { { let t3358 = v_v.clone(); Py::Bool({ let t3359 = Py::Int(0i64); if py::gt(&t3358, &t3359)? { true } else { false } }) } } }, Py::str("van 't Hoff factors must be positive"))?;
    }
    let _ = f__pos(Py::str("the cell's osmolarity"), Py::tuple(vec![v_osm_cell.clone()]))?;
    v_osm = Py::Float(0.0);
    for t3365 in py::range_iter(&[(py::len(&v_c.clone())?).i()?]) {
        v_k = t3365;
        v_osm = py::iadd(&v_osm, &py::mul(&py::getitem(&v_c.clone(), &v_k.clone())?, &py::getitem(&v_i.clone(), &v_k.clone())?)?)?;
    }
    v_cell = v_osm_cell.clone();
    v_diff = py::sub(&v_osm.clone(), &v_cell.clone())?;
    if ({ let t3366 = py::abs(&v_diff.clone())?; Py::Bool({ let t3367 = py::mul(&Py::Float(0.01), &v_cell.clone())?; if py::le(&t3366, &t3367)? { true } else { false } }) }).truthy() {
        v_word = Py::str("isotonic");
    } else {
        if ({ let t3368 = v_diff.clone(); Py::Bool({ let t3369 = Py::Int(0i64); if py::gt(&t3368, &t3369)? { true } else { false } }) }).truthy() {
            v_word = Py::str("hypertonic");
        } else {
            v_word = Py::str("hypotonic");
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("osmolarity_mOsm"), v_osm.clone()), (Py::str("difference_mOsm"), v_diff.clone()), (Py::str("tonicity"), v_word.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_osmotic_pressure(mut v_i: Py, mut v_c: Py, mut v_T: Py) -> R {
    let mut v_pi: Py = Py::None;
    let _ = f__pos(Py::str("the van 't Hoff factor i, the concentration c and T"), Py::tuple(vec![v_i.clone(), v_c.clone(), v_T.clone()]))?;
    v_pi = py::mul(&py::mul(&py::mul(&v_i.clone(), &v_c.clone())?, &c_R_GAS()?)?, &v_T.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("pi"), v_pi.clone()), (Py::str("pi_atm"), py::truediv(&v_pi.clone(), &Py::Float(101325.0))?), (Py::str("pi_kPa"), py::truediv(&v_pi.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pcr_amplification(mut v_N0: Py, mut v_E: Py, mut v_n: Py) -> R {
    let mut v_fold: Py = Py::None;
    let _ = f__pos(Py::str("the initial copies N0"), Py::tuple(vec![v_N0.clone()]))?;
    let _ = f__frac(Py::str("the efficiency E"), v_E.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__need({ let t3470 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3470.truthy() { t3470 } else { { let t3467 = Py::Int(0i64); Py::Bool({ let t3468 = v_n.clone(); if py::le(&t3467, &t3468)? { { let t3469 = Py::Int(60i64); if py::le(&t3468, &t3469)? { true } else { false } } } else { false } }) } } }, Py::str("the number of cycles must be a whole number from 0 to 60"))?;
    v_fold = py::pow(&py::add(&Py::Float(1.0), &v_E.clone())?, &v_n.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("N"), py::mul(&v_N0.clone(), &v_fold.clone())?), (Py::str("fold"), v_fold.clone()), (Py::str("log10_fold"), py::mul(&v_n.clone(), &py::math1("log10", &py::add(&Py::Float(1.0), &v_E.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pcr_cycles_needed(mut v_N0: Py, mut v_N: Py, mut v_E: Py) -> R {
    let mut v_cycles: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__pos(Py::str("N0 and N"), Py::tuple(vec![v_N0.clone(), v_N.clone()]))?;
    let _ = f__need({ let t3515 = v_N.clone(); Py::Bool({ let t3516 = v_N0.clone(); if py::gt(&t3515, &t3516)? { true } else { false } }) }, Py::str("the target must exceed the starting copies"))?;
    let _ = f__frac(Py::str("the efficiency E"), v_E.clone(), Py::Bool(true), Py::Bool(false))?;
    v_n = py::truediv(&py::math1("log", &py::truediv(&v_N.clone(), &v_N0.clone())?)?, &py::math1("log", &py::add(&Py::Float(1.0), &v_E.clone())?)?)?;
    v_w = py::round(&v_n.clone(), None)?;
    v_cycles = (if ({ let t3526 = py::abs(&py::sub(&v_n.clone(), &v_w.clone())?)?; Py::Bool({ let t3527 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), v_n.clone()], true)?)?; if py::le(&t3526, &t3527)? { true } else { false } }) }).truthy() { py::int(&v_w.clone())? } else { py::int(&py::math1("ceil", &v_n.clone())?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("cycles_exact"), v_n.clone()), (Py::str("cycles"), v_cycles.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_percent_to_molarity(mut v_p: Py, mut v_MW: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let _ = f__frac(Py::str("the per cent w/v as a fraction"), v_p.clone(), Py::Bool(true), Py::Bool(true))?;
    let _ = f__pos(Py::str("MW"), Py::tuple(vec![v_MW.clone()]))?;
    v_rho = py::mul(&v_p.clone(), &Py::Float(1000.0))?;
    v_C = py::truediv(&v_rho.clone(), &v_MW.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_mM"), v_C.clone()), (Py::str("C_M"), py::truediv(&v_C.clone(), &Py::Float(1000.0))?), (Py::str("mass_concentration"), v_rho.clone()), (Py::str("g_per_L"), v_rho.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_protein_a280(mut v_A280: Py, mut v_eps: Py, mut v_MW: Py, mut v_l: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__nonneg(Py::str("A280"), Py::tuple(vec![v_A280.clone()]))?;
    let _ = f__pos(Py::str("the extinction coefficient and the path length"), Py::tuple(vec![v_eps.clone(), v_l.clone()]))?;
    v_c = py::truediv(&v_A280.clone(), &py::mul(&v_eps.clone(), &v_l.clone())?)?;
    v_out = Py::dict(vec![(Py::str("c"), v_c.clone()), (Py::str("c_uM"), py::mul(&v_c.clone(), &Py::Float(1000.0))?)]);
    if ({ let t3681 = v_MW.clone(); Py::Bool({ let t3682 = Py::None; if !py::is(&t3681, &t3682) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the molar mass"), Py::tuple(vec![v_MW.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("c_mg_mL"), py::mul(&v_c.clone(), &v_MW.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_q10_rate(mut v_k1: Py, mut v_T1: Py, mut v_T2: Py, mut v_Q10: Py) -> R {
    let mut v_Ea: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let _ = f__pos(Py::str("k1, T1, T2 and Q10"), Py::tuple(vec![v_k1.clone(), v_T1.clone(), v_T2.clone(), v_Q10.clone()]))?;
    let _ = f__need({ let t3728 = v_T1.clone(); Py::Bool({ let t3729 = v_T2.clone(); if py::ne(&t3728, &t3729) { true } else { false } }) }, Py::str("the two temperatures must differ"))?;
    v_k2 = py::mul(&v_k1.clone(), &py::pow(&v_Q10.clone(), &py::truediv(&py::sub(&v_T2.clone(), &v_T1.clone())?, &Py::Float(10.0))?)?)?;
    v_Ea = py::truediv(&py::mul(&c_R_GAS()?, &py::math1("log", &py::truediv(&v_k2.clone(), &v_k1.clone())?)?)?, &py::sub(&py::truediv(&Py::Float(1.0), &v_T1.clone())?, &py::truediv(&Py::Float(1.0), &v_T2.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("k2"), v_k2.clone()), (Py::str("ratio"), py::truediv(&v_k2.clone(), &v_k1.clone())?), (Py::str("Ea"), v_Ea.clone()), (Py::str("Ea_kJ_mol"), py::truediv(&v_Ea.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qpcr_ddct(mut v_Ct_target_treated: Py, mut v_Ct_ref_treated: Py, mut v_Ct_target_control: Py, mut v_Ct_ref_control: Py) -> R {
    let mut v_dc: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_Ct_target_treated.clone(), v_Ct_ref_treated.clone(), v_Ct_target_control.clone(), v_Ct_ref_control.clone()]))?;
    for t3771 in py::iter(&(Py::tuple(vec![v_Ct_target_treated.clone(), v_Ct_ref_treated.clone(), v_Ct_target_control.clone(), v_Ct_ref_control.clone()])))? {
        v_v = t3771;
        let _ = f__need({ let t3772 = Py::Int(0i64); Py::Bool({ let t3773 = v_v.clone(); if py::lt(&t3772, &t3773)? { { let t3774 = Py::Int(60i64); if py::le(&t3773, &t3774)? { true } else { false } } } else { false } }) }, Py::str("Ct values lie between 0 and 60 cycles"))?;
    }
    v_dt = py::sub(&v_Ct_target_treated.clone(), &v_Ct_ref_treated.clone())?;
    v_dc = py::sub(&v_Ct_target_control.clone(), &v_Ct_ref_control.clone())?;
    v_dd = py::sub(&v_dt.clone(), &v_dc.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("dCt_treated"), v_dt.clone()), (Py::str("dCt_control"), v_dc.clone()), (Py::str("ddCt"), v_dd.clone()), (Py::str("fold_change"), py::pow(&Py::Float(2.0), &py::neg(&v_dd.clone())?)?), (Py::str("log2_fold_change"), py::neg(&v_dd.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qpcr_pfaffl(mut v_E_target: Py, mut v_E_ref: Py, mut v_Ct_target_control: Py, mut v_Ct_target_sample: Py, mut v_Ct_ref_control: Py, mut v_Ct_ref_sample: Py) -> R {
    let mut v_dr: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__frac(Py::str("E_target"), v_E_target.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("E_ref"), v_E_ref.clone(), Py::Bool(true), Py::Bool(false))?;
    for t3824 in py::iter(&(Py::tuple(vec![v_Ct_target_control.clone(), v_Ct_target_sample.clone(), v_Ct_ref_control.clone(), v_Ct_ref_sample.clone()])))? {
        v_v = t3824;
        let _ = f__need({ let t3828 = f__isnum(v_v.clone())?; if !t3828.truthy() { t3828 } else { { let t3825 = Py::Int(0i64); Py::Bool({ let t3826 = v_v.clone(); if py::lt(&t3825, &t3826)? { { let t3827 = Py::Int(60i64); if py::le(&t3826, &t3827)? { true } else { false } } } else { false } }) } } }, Py::str("Ct values lie between 0 and 60 cycles"))?;
    }
    v_dt = py::sub(&v_Ct_target_control.clone(), &v_Ct_target_sample.clone())?;
    v_dr = py::sub(&v_Ct_ref_control.clone(), &v_Ct_ref_sample.clone())?;
    v_ratio = py::truediv(&py::pow(&py::add(&Py::Float(1.0), &v_E_target.clone())?, &v_dt.clone())?, &py::pow(&py::add(&Py::Float(1.0), &v_E_ref.clone())?, &v_dr.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("ratio"), v_ratio.clone()), (Py::str("log2_ratio"), py::math1("log2", &v_ratio.clone())?), (Py::str("dCt_target"), v_dt.clone()), (Py::str("dCt_ref"), v_dr.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qpcr_quantify(mut v_slope: Py, mut v_intercept: Py, mut v_Ct: Py) -> R {
    let mut v_q: Py = Py::None;
    let _ = f__need({ let t3871 = f__isnum(v_slope.clone())?; if !t3871.truthy() { t3871 } else { { let t3869 = v_slope.clone(); Py::Bool({ let t3870 = Py::Int(0i64); if py::lt(&t3869, &t3870)? { true } else { false } }) } } }, Py::str("the slope of a standard curve is negative"))?;
    let _ = f__fin(Py::tuple(vec![v_intercept.clone(), v_Ct.clone()]))?;
    v_q = py::pow(&Py::Float(10.0), &py::truediv(&py::sub(&v_Ct.clone(), &v_intercept.clone())?, &v_slope.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("quantity"), v_q.clone()), (Py::str("log10_quantity"), py::truediv(&py::sub(&v_Ct.clone(), &v_intercept.clone())?, &v_slope.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qpcr_standard_curve(mut v_quantity: Py, mut v_Ct: Py) -> R {
    let mut v_amp: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__need({ let t3915 = { let t3911 = py::len(&v_quantity.clone())?; Py::Bool({ let t3912 = py::len(&v_Ct.clone())?; if py::eq(&t3911, &t3912) { true } else { false } }) }; if !t3915.truthy() { t3915 } else { { let t3913 = py::len(&v_Ct.clone())?; Py::Bool({ let t3914 = Py::Int(3i64); if py::ge(&t3913, &t3914)? { true } else { false } }) } } }, Py::str("give at least 3 standards with one Ct each"))?;
    for t3916 in py::iter(&(v_quantity.clone()))? {
        v_v = t3916;
        let _ = f__need({ let t3921 = f__isnum(v_v.clone())?; if !t3921.truthy() { t3921 } else { { let t3919 = v_v.clone(); Py::Bool({ let t3920 = Py::Int(0i64); if py::gt(&t3919, &t3920)? { true } else { false } }) } } }, Py::str("standard quantities must be positive"))?;
    }
    v_x = { let mut out = vec![]; for t3922 in py::iter(&(v_quantity.clone()))? { let mut v_v: Py; 
v_v = t3922;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) };
    let t3932 = (f__linfit(v_x.clone(), v_Ct.clone())?).items()?;
    if t3932.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t3932[0].clone();
    v_b = t3932[1].clone();
    v_r2 = t3932[2].clone();
    let _ = f__need({ let t3933 = v_m.clone(); Py::Bool({ let t3934 = Py::Int(0i64); if py::lt(&t3933, &t3934)? { true } else { false } }) }, Py::str("Ct must fall as the quantity rises (a negative slope)"))?;
    v_amp = py::pow(&Py::Float(10.0), &py::truediv(&py::neg(&Py::Float(1.0))?, &v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("slope"), v_m.clone()), (Py::str("intercept"), v_b.clone()), (Py::str("r2"), v_r2.clone()), (Py::str("efficiency"), py::sub(&v_amp.clone(), &Py::Float(1.0))?), (Py::str("amplification_factor"), v_amp.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reverse_complement(mut v_seq: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_ch: Py = Py::None;
    let mut v_comp: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_rev: Py = Py::None;
    let mut v_rna: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3988 = (f__clean_seq(v_seq.clone(), Py::Bool(true), Py::Bool(true))?).items()?;
    if t3988.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3988[0].clone();
    v_rna = t3988[1].clone();
    v_comp = (if (v_rna.clone()).truthy() { c__RNA_COMP()? } else { c__DNA_COMP()? });
    v_c = Py::str("");
    for t3989 in py::iter(&(v_s.clone()))? {
        v_ch = t3989;
        v_c = py::iadd(&v_c, &py::getitem(&v_comp.clone(), &v_ch.clone())?)?;
    }
    v_rev = Py::str("");
    for t3990 in py::range_iter(&[(py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_k = t3990;
        v_rev = py::iadd(&v_rev, &py::getitem(&v_s.clone(), &v_k.clone())?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("reverse_complement"), f__revcomp(v_s.clone(), v_rna.clone())?), (Py::str("complement"), v_c.clone()), (Py::str("reverse"), v_rev.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rms_displacement(mut v_D: Py, mut v_t: Py, mut v_dims: Py) -> R {
    let mut v_msd: Py = Py::None;
    let mut v_rms: Py = Py::None;
    let _ = f__pos(Py::str("D and t"), Py::tuple(vec![v_D.clone(), v_t.clone()]))?;
    let _ = f__need({ let t4036 = v_dims.clone(); Py::Bool({ let t4037 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)]); if py::contains(&t4037, &t4036)? { true } else { false } }) }, Py::str("dims must be 1, 2 or 3"))?;
    v_msd = py::mul(&py::mul(&py::mul(&Py::Float(2.0), &v_dims.clone())?, &v_D.clone())?, &v_t.clone())?;
    v_rms = py::math1("sqrt", &v_msd.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("rms"), v_rms.clone()), (Py::str("rms_um"), py::mul(&v_rms.clone(), &Py::Float(1000000.0))?), (Py::str("msd"), v_msd.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_scatchard_fit(mut v_B: Py, mut v_F: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__need({ let t4080 = { let t4076 = py::len(&v_B.clone())?; Py::Bool({ let t4077 = py::len(&v_F.clone())?; if py::eq(&t4076, &t4077) { true } else { false } }) }; if !t4080.truthy() { t4080 } else { { let t4078 = py::len(&v_B.clone())?; Py::Bool({ let t4079 = Py::Int(3i64); if py::ge(&t4078, &t4079)? { true } else { false } }) } } }, Py::str("give at least 3 bound and free concentrations"))?;
    for t4081 in py::iter(&(v_B.clone()))? {
        v_a = t4081;
        let _ = f__need({ let t4086 = f__isnum(v_a.clone())?; if !t4086.truthy() { t4086 } else { { let t4084 = v_a.clone(); Py::Bool({ let t4085 = Py::Int(0i64); if py::gt(&t4084, &t4085)? { true } else { false } }) } } }, Py::str("bound concentrations must be positive"))?;
    }
    for t4087 in py::iter(&(v_F.clone()))? {
        v_a = t4087;
        let _ = f__need({ let t4090 = f__isnum(v_a.clone())?; if !t4090.truthy() { t4090 } else { { let t4088 = v_a.clone(); Py::Bool({ let t4089 = Py::Int(0i64); if py::gt(&t4088, &t4089)? { true } else { false } }) } } }, Py::str("free concentrations must be positive"))?;
    }
    v_y = { let mut out = vec![]; for t4091 in py::range_iter(&[(py::len(&v_B.clone())?).i()?]) { let mut v_k: Py; 
v_k = t4091;
out.push(py::truediv(&py::getitem(&v_B.clone(), &v_k.clone())?, &py::getitem(&v_F.clone(), &v_k.clone())?)?); } Py::list(out) };
    let t4101 = (f__linfit(v_B.clone(), v_y.clone())?).items()?;
    if t4101.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4101[0].clone();
    v_b = t4101[1].clone();
    v_r2 = t4101[2].clone();
    let _ = f__need({ let t4102 = v_m.clone(); Py::Bool({ let t4103 = Py::Int(0i64); if py::lt(&t4102, &t4103)? { true } else { false } }) }, Py::str("the Scatchard line must have a negative slope"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("Kd"), py::truediv(&py::neg(&Py::Float(1.0))?, &v_m.clone())?), (Py::str("Bmax"), py::truediv(&py::neg(&v_b.clone())?, &v_m.clone())?), (Py::str("slope"), v_m.clone()), (Py::str("intercept"), v_b.clone()), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sedimentation_time(mut v_s: Py, mut v_rpm: Py, mut v_r_min: Py, mut v_r_max: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_sec: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("s, the rotor speed and the radii"), Py::tuple(vec![v_s.clone(), v_rpm.clone(), v_r_min.clone(), v_r_max.clone()]))?;
    let _ = f__need({ let t4148 = v_r_max.clone(); Py::Bool({ let t4149 = v_r_min.clone(); if py::gt(&t4148, &t4149)? { true } else { false } }) }, Py::str("r_max must exceed r_min"))?;
    v_sec = py::mul(&v_s.clone(), &Py::Float(1e-13))?;
    v_t = py::truediv(&py::math1("log", &py::truediv(&v_r_max.clone(), &v_r_min.clone())?)?, &py::mul(&py::mul(&v_rpm.clone(), &v_rpm.clone())?, &v_sec.clone())?)?;
    v_k = py::mul(&py::truediv(&py::truediv(&py::math1("log", &py::truediv(&v_r_max.clone(), &v_r_min.clone())?)?, &py::mul(&v_rpm.clone(), &v_rpm.clone())?)?, &c_HOUR()?)?, &Py::Float(10000000000000.0))?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_h"), py::truediv(&v_t.clone(), &c_HOUR()?)?), (Py::str("k_factor"), v_k.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_serial_dilution(mut v_C0: Py, mut v_f: Py, mut v_n: Py, mut v_V: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_concs: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("C0"), Py::tuple(vec![v_C0.clone()]))?;
    let _ = f__need({ let t4196 = f__isnum(v_f.clone())?; if !t4196.truthy() { t4196 } else { { let t4194 = v_f.clone(); Py::Bool({ let t4195 = Py::Int(1i64); if py::gt(&t4194, &t4195)? { true } else { false } }) } } }, Py::str("the dilution factor per step must exceed 1"))?;
    let _ = f__need({ let t4200 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4200.truthy() { t4200 } else { { let t4197 = Py::Int(1i64); Py::Bool({ let t4198 = v_n.clone(); if py::le(&t4197, &t4198)? { { let t4199 = Py::Int(30i64); if py::le(&t4198, &t4199)? { true } else { false } } } else { false } }) } } }, Py::str("the number of steps must be a whole number from 1 to 30"))?;
    v_concs = Py::list(vec![]);
    v_c = v_C0.clone();
    for t4201 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t4201;
        v_c = py::truediv(&v_c.clone(), &v_f.clone())?;
        let _ = py::method(&v_concs.clone(), "append", &[v_c.clone()])?;
    }
    v_out = Py::dict(vec![(Py::str("C_final"), py::getitem(&v_concs.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("total_dilution"), py::pow(&v_f.clone(), &v_n.clone())?), (Py::str("concentrations_mM"), v_concs.clone())]);
    if ({ let t4202 = v_V.clone(); Py::Bool({ let t4203 = Py::None; if !py::is(&t4202, &t4203) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the tube volume V"), Py::tuple(vec![v_V.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("transfer"), py::truediv(&v_V.clone(), &v_f.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("diluent"), py::sub(&v_V.clone(), &py::truediv(&v_V.clone(), &v_f.clone())?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_smoluchowski_rate(mut v_DA: Py, mut v_DB: Py, mut v_rA: Py, mut v_rB: Py) -> R {
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("the diffusion coefficients and radii"), Py::tuple(vec![v_DA.clone(), v_DB.clone(), v_rA.clone(), v_rB.clone()]))?;
    v_k = py::mul(&py::mul(&py::mul(&py::mul(&Py::Float(4.0), &Py::Float(std::f64::consts::PI))?, &py::add(&v_DA.clone(), &v_DB.clone())?)?, &py::add(&v_rA.clone(), &v_rB.clone())?)?, &c_N_A()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("k_per_M_s"), py::mul(&v_k.clone(), &Py::Float(1000.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stokes_einstein(mut v_T: Py, mut v_eta: Py, mut v_r: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_f: Py = Py::None;
    let _ = f__pos(Py::str("T, eta and r"), Py::tuple(vec![v_T.clone(), v_eta.clone(), v_r.clone()]))?;
    v_f = py::mul(&py::mul(&py::mul(&Py::Float(6.0), &Py::Float(std::f64::consts::PI))?, &v_eta.clone())?, &v_r.clone())?;
    v_D = py::truediv(&py::mul(&c_K_B()?, &v_T.clone())?, &v_f.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D"), v_D.clone()), (Py::str("D_um2_s"), py::mul(&v_D.clone(), &Py::Float(1000000000000.0))?), (Py::str("friction"), v_f.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_substrate_for_fraction(mut v_Km: Py, mut v_f: Py) -> R {
    let mut v_S: Py = Py::None;
    let _ = f__pos(Py::str("Km"), Py::tuple(vec![v_Km.clone()]))?;
    let _ = f__frac(Py::str("the fraction of Vmax f"), v_f.clone(), Py::Bool(true), Py::Bool(true))?;
    v_S = py::truediv(&py::mul(&v_Km.clone(), &v_f.clone())?, &py::sub(&Py::Float(1.0), &v_f.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("S"), v_S.clone()), (Py::str("S_uM"), py::mul(&v_S.clone(), &Py::Float(1000.0))?), (Py::str("S_over_Km"), py::truediv(&v_f.clone(), &py::sub(&Py::Float(1.0), &v_f.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tm_basic(mut v_seq: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_gc: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t4355 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4355.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4355[0].clone();
    v__ = t4355[1].clone();
    let _ = f__need({ let t4356 = py::len(&v_s.clone())?; Py::Bool({ let t4357 = Py::Int(14i64); if py::ge(&t4356, &t4357)? { true } else { false } }) }, Py::str("the basic formula is for 14 nt or more (use the Wallace rule for shorter primers)"))?;
    v_n = py::len(&v_s.clone())?;
    v_gc = py::add(&py::method(&v_s.clone(), "count", &[Py::str("G")])?, &py::method(&v_s.clone(), "count", &[Py::str("C")])?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Tm"), py::add(&Py::Float(64.9), &py::truediv(&py::mul(&Py::Float(41.0), &py::sub(&v_gc.clone(), &Py::Float(16.4))?)?, &v_n.clone())?)?), (Py::str("gc_percent"), py::truediv(&py::mul(&Py::Float(100.0), &v_gc.clone())?, &v_n.clone())?), (Py::str("length"), v_n.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tm_nearest_neighbor(mut v_seq: Py, mut v_Na: Py, mut v_c1: Py, mut v_c2: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_dg37: Py = Py::None;
    let mut v_dh: Py = Py::None;
    let mut v_ds: Py = Py::None;
    let mut v_ds_corr: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_k_molar: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_selfcomp: Py = Py::None;
    let mut v_tm: Py = Py::None;
    let t4411 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4411.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4411[0].clone();
    v__ = t4411[1].clone();
    let _ = f__need({ let t4412 = py::len(&v_s.clone())?; Py::Bool({ let t4413 = Py::Int(2i64); if py::ge(&t4412, &t4413)? { true } else { false } }) }, Py::str("the sequence needs at least 2 bases"))?;
    let _ = f__pos(Py::str("Na and c1"), Py::tuple(vec![v_Na.clone(), v_c1.clone()]))?;
    let _ = f__nonneg(Py::str("c2"), Py::tuple(vec![v_c2.clone()]))?;
    let t4432 = (f__nn_sums(v_s.clone())?).items()?;
    if t4432.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dh = t4432[0].clone();
    v_ds = t4432[1].clone();
    v_selfcomp = t4432[2].clone();
    if (v_selfcomp.clone()).truthy() {
        v_k = v_c1.clone();
    } else {
        let _ = f__need({ let t4433 = v_c1.clone(); Py::Bool({ let t4434 = py::truediv(&v_c2.clone(), &Py::Float(2.0))?; if py::gt(&t4433, &t4434)? { true } else { false } }) }, Py::str("c1 must exceed c2/2 (k = c1 - c2/2 must be positive)"))?;
        v_k = py::sub(&v_c1.clone(), &py::truediv(&v_c2.clone(), &Py::Float(2.0))?)?;
    }
    v_k_molar = py::mul(&v_k.clone(), &Py::Float(0.001))?;
    v_ds_corr = py::add(&v_ds.clone(), &py::mul(&py::mul(&Py::Float(0.368), &py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?)?, &py::math1("log", &py::mul(&v_Na.clone(), &Py::Float(0.001))?)?)?)?;
    v_tm = py::sub(&py::truediv(&py::mul(&Py::Float(1000.0), &v_dh.clone())?, &py::add(&v_ds_corr.clone(), &py::mul(&Py::Float(1.987), &py::math1("log", &v_k_molar.clone())?)?)?)?, &Py::Float(273.15))?;
    v_dg37 = py::sub(&v_dh.clone(), &py::truediv(&py::mul(&Py::Float(310.15), &v_ds_corr.clone())?, &Py::Float(1000.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Tm"), v_tm.clone()), (Py::str("dH"), v_dh.clone()), (Py::str("dS"), v_ds_corr.clone()), (Py::str("dG37"), v_dg37.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tm_salt_adjusted(mut v_seq: Py, mut v_Na: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pgc: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t4488 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4488[0].clone();
    v__ = t4488[1].clone();
    let _ = f__need({ let t4489 = py::len(&v_s.clone())?; Py::Bool({ let t4490 = Py::Int(14i64); if py::ge(&t4489, &t4490)? { true } else { false } }) }, Py::str("the salt-adjusted formula is for 14 nt or more"))?;
    let _ = f__pos(Py::str("the sodium concentration"), Py::tuple(vec![v_Na.clone()]))?;
    v_n = py::len(&v_s.clone())?;
    v_pgc = py::truediv(&py::mul(&Py::Float(100.0), &py::add(&py::method(&v_s.clone(), "count", &[Py::str("G")])?, &py::method(&v_s.clone(), "count", &[Py::str("C")])?)?)?, &v_n.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Tm"), py::add(&py::sub(&py::add(&Py::Float(81.5), &py::mul(&Py::Float(0.41), &v_pgc.clone())?)?, &py::truediv(&Py::Float(600.0), &v_n.clone())?)?, &py::mul(&Py::Float(16.6), &py::math1("log10", &py::mul(&v_Na.clone(), &Py::Float(0.001))?)?)?)?), (Py::str("gc_percent"), v_pgc.clone()), (Py::str("salt_term"), py::mul(&Py::Float(16.6), &py::math1("log10", &py::mul(&v_Na.clone(), &Py::Float(0.001))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tm_wallace(mut v_seq: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_at: Py = Py::None;
    let mut v_gc: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t4550 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4550.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4550[0].clone();
    v__ = t4550[1].clone();
    let _ = f__need({ let t4551 = py::len(&v_s.clone())?; Py::Bool({ let t4552 = Py::Int(2i64); if py::ge(&t4551, &t4552)? { true } else { false } }) }, Py::str("the primer needs at least 2 bases"))?;
    let t4553 = (Py::tuple(vec![py::add(&py::method(&v_s.clone(), "count", &[Py::str("A")])?, &py::method(&v_s.clone(), "count", &[Py::str("T")])?)?, py::add(&py::method(&v_s.clone(), "count", &[Py::str("G")])?, &py::method(&v_s.clone(), "count", &[Py::str("C")])?)?])).items()?;
    if t4553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_at = t4553[0].clone();
    v_gc = t4553[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("Tm"), py::add(&py::mul(&Py::Float(2.0), &v_at.clone())?, &py::mul(&Py::Float(4.0), &v_gc.clone())?)?), (Py::str("AT"), v_at.clone()), (Py::str("GC"), v_gc.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transcribe(mut v_seq: Py, mut v_strand: Py) -> R {
    let mut v_ch: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_rna: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_src: Py = Py::None;
    let t4607 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4607.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4607[0].clone();
    v_rna = t4607[1].clone();
    let _ = f__need({ let t4608 = v_strand.clone(); Py::Bool({ let t4609 = Py::tuple(vec![Py::str("coding"), Py::str("template")]); if py::contains(&t4609, &t4608)? { true } else { false } }) }, Py::str("strand: coding or template"))?;
    v_src = (if ({ let t4610 = v_strand.clone(); Py::Bool({ let t4611 = Py::str("coding"); if py::eq(&t4610, &t4611) { true } else { false } }) }).truthy() { v_s.clone() } else { f__revcomp(v_s.clone(), Py::Bool(false))? });
    v_m = Py::str("");
    for t4613 in py::iter(&(v_src.clone()))? {
        v_ch = t4613;
        v_m = py::iadd(&v_m, &(if ({ let t4614 = v_ch.clone(); Py::Bool({ let t4615 = Py::str("T"); if py::eq(&t4614, &t4615) { true } else { false } }) }).truthy() { Py::str("U") } else { v_ch.clone() }))?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("mRNA"), v_m.clone()), (Py::str("length"), py::len(&v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transcription_time(mut v_n: Py, mut v_rate: Py, mut v_k_init: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__need({ let t4656 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4656.truthy() { t4656 } else { { let t4654 = v_n.clone(); Py::Bool({ let t4655 = Py::Int(1i64); if py::ge(&t4654, &t4655)? { true } else { false } }) } } }, Py::str("the gene length must be a whole number of nucleotides"))?;
    let _ = f__pos(Py::str("the elongation rate"), Py::tuple(vec![v_rate.clone()]))?;
    v_t = py::truediv(&v_n.clone(), &v_rate.clone())?;
    v_out = Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_min"), py::truediv(&v_t.clone(), &c_MINUTE()?)?)]);
    if ({ let t4663 = v_k_init.clone(); Py::Bool({ let t4664 = Py::None; if !py::is(&t4663, &t4664) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the initiation rate"), Py::tuple(vec![v_k_init.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("polymerases"), py::mul(&v_k_init.clone(), &v_t.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("spacing_nt"), py::truediv(&v_rate.clone(), &v_k_init.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transformation_efficiency(mut v_colonies: Py, mut v_m_DNA: Py, mut v_V_plated: Py, mut v_V_total: Py) -> R {
    let mut v_eff: Py = Py::None;
    let mut v_ug_plated: Py = Py::None;
    let _ = f__need({ let t4706 = Py::Bool(matches!(v_colonies.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4706.truthy() { t4706 } else { { let t4704 = v_colonies.clone(); Py::Bool({ let t4705 = Py::Int(0i64); if py::ge(&t4704, &t4705)? { true } else { false } }) } } }, Py::str("the colony count must be a whole number"))?;
    let _ = f__pos(Py::str("the DNA mass and the volumes"), Py::tuple(vec![v_m_DNA.clone(), v_V_plated.clone(), v_V_total.clone()]))?;
    let _ = f__need({ let t4713 = v_V_plated.clone(); Py::Bool({ let t4714 = v_V_total.clone(); if py::le(&t4713, &t4714)? { true } else { false } }) }, Py::str("the plated volume cannot exceed the total volume"))?;
    v_ug_plated = py::truediv(&py::mul(&py::mul(&v_m_DNA.clone(), &Py::Float(1000000000.0))?, &v_V_plated.clone())?, &v_V_total.clone())?;
    v_eff = py::truediv(&v_colonies.clone(), &v_ug_plated.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("efficiency"), v_eff.clone()), (Py::str("log10_efficiency"), (if ({ let t4727 = v_eff.clone(); Py::Bool({ let t4728 = Py::Int(0i64); if py::gt(&t4727, &t4728)? { true } else { false } }) }).truthy() { py::math1("log10", &v_eff.clone())? } else { Py::Float(0.0) })), (Py::str("DNA_plated_ug"), v_ug_plated.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_translate(mut v_seq: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_full: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_protein: Py = Py::None;
    let mut v_rna: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_stop: Py = Py::None;
    let t4770 = (f__clean_seq(v_seq.clone(), Py::Bool(true), Py::Bool(true))?).items()?;
    if t4770.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4770[0].clone();
    v_rna = t4770[1].clone();
    v_d = py::method(&v_s.clone(), "replace", &[Py::str("U"), Py::str("T")])?;
    v_n = py::floordiv(&py::len(&v_d.clone())?, &Py::Int(3i64))?;
    let _ = f__need({ let t4771 = v_n.clone(); Py::Bool({ let t4772 = Py::Int(1i64); if py::ge(&t4771, &t4772)? { true } else { false } }) }, Py::str("the sequence needs at least one codon"))?;
    v_full = Py::str("");
    for t4773 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t4773;
        v_full = py::iadd(&v_full, &py::getitem(&c__CODONS()?, &py::slice(&v_d.clone(), Some((py::mul(&Py::Int(3i64), &v_k.clone())?).i()?), Some((py::add(&py::mul(&Py::Int(3i64), &v_k.clone())?, &Py::Int(3i64))?).i()?), None)?)?)?;
    }
    v_stop = py::method(&v_full.clone(), "find", &[Py::str("*")])?;
    v_protein = (if ({ let t4774 = v_stop.clone(); Py::Bool({ let t4775 = Py::Int(0i64); if py::lt(&t4774, &t4775)? { true } else { false } }) }).truthy() { v_full.clone() } else { py::slice(&v_full.clone(), None, Some((v_stop.clone()).i()?), None)? });
    return Ok(f__out(Py::dict(vec![(Py::str("protein"), v_protein.clone()), (Py::str("full_translation"), v_full.clone()), (Py::str("length_aa"), py::len(&v_protein.clone())?), (Py::str("stops"), py::method(&v_full.clone(), "count", &[Py::str("*")])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_translation_time(mut v_n_aa: Py, mut v_rate: Py, mut v_k_init: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__need({ let t4816 = Py::Bool(matches!(v_n_aa.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4816.truthy() { t4816 } else { { let t4814 = v_n_aa.clone(); Py::Bool({ let t4815 = Py::Int(1i64); if py::ge(&t4814, &t4815)? { true } else { false } }) } } }, Py::str("the protein length must be a whole number of amino acids"))?;
    let _ = f__pos(Py::str("the elongation rate"), Py::tuple(vec![v_rate.clone()]))?;
    v_t = py::truediv(&v_n_aa.clone(), &v_rate.clone())?;
    v_out = Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_min"), py::truediv(&v_t.clone(), &c_MINUTE()?)?), (Py::str("coding_nt"), py::mul(&Py::Int(3i64), &py::add(&v_n_aa.clone(), &Py::Int(1i64))?)?)]);
    if ({ let t4823 = v_k_init.clone(); Py::Bool({ let t4824 = Py::None; if !py::is(&t4823, &t4824) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the initiation rate"), Py::tuple(vec![v_k_init.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("ribosomes"), py::mul(&v_k_init.clone(), &v_t.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("spacing_codons"), py::truediv(&v_rate.clone(), &v_k_init.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_warburg_christian(mut v_A280: Py, mut v_A260: Py) -> R {
    let mut v_p: Py = Py::None;
    let _ = f__nonneg(Py::str("the absorbances"), Py::tuple(vec![v_A280.clone(), v_A260.clone()]))?;
    v_p = py::sub(&py::mul(&Py::Float(1.55), &v_A280.clone())?, &py::mul(&Py::Float(0.76), &v_A260.clone())?)?;
    let _ = f__need({ let t4928 = v_p.clone(); Py::Bool({ let t4929 = Py::Int(0i64); if py::gt(&t4928, &t4929)? { true } else { false } }) }, Py::str("the absorbances give no positive protein estimate (nucleic acid dominates)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("protein"), v_p.clone()), (Py::str("protein_mg_mL"), v_p.clone()), (Py::str("ratio_280_260"), (if ({ let t4942 = v_A260.clone(); Py::Bool({ let t4943 = Py::Int(0i64); if py::gt(&t4942, &t4943)? { true } else { false } }) }).truthy() { py::truediv(&v_A280.clone(), &v_A260.clone())? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1013(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1015: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1014 = v_inputs.clone(); f_doubling_from_counts(py::getitem(&t1014, &Py::str("N0"))?, py::getitem(&t1014, &Py::str("N"))?, py::getitem(&t1014, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1015 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1016 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1038 = (t1016).items()?;
        if t1038.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1038[0].clone();
        v_v = t1038[1].clone();
        if ({ let t1017 = v_k.clone(); Py::Bool({ let t1018 = v_outputs.clone(); if !py::contains(&t1018, &t1017)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1065(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1067: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1066 = v_inputs.clone(); f_enzyme_assay_activity(py::getitem(&t1066, &Py::str("dA"))?, py::getitem(&t1066, &Py::str("dt"))?, py::getitem(&t1066, &Py::str("eps"))?, py::getitem(&t1066, &Py::str("l"))?, py::getitem(&t1066, &Py::str("V_assay"))?, py::getitem(&t1066, &Py::str("V_enzyme"))?, py::method(&t1066, "get", &[Py::str("protein"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1067 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1068 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1090 = (t1068).items()?;
        if t1090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1090[0].clone();
        v_v = t1090[1].clone();
        if ({ let t1069 = v_k.clone(); Py::Bool({ let t1070 = v_outputs.clone(); if !py::contains(&t1070, &t1069)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1113(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1115: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1114 = v_inputs.clone(); f_exponential_growth(py::getitem(&t1114, &Py::str("N0"))?, py::getitem(&t1114, &Py::str("td"))?, py::getitem(&t1114, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1115 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1116 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1138 = (t1116).items()?;
        if t1138.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1138[0].clone();
        v_v = t1138[1].clone();
        if ({ let t1117 = v_k.clone(); Py::Bool({ let t1118 = v_outputs.clone(); if !py::contains(&t1118, &t1117)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1157(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1159: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1158 = v_inputs.clone(); f_expression_steady_state(py::getitem(&t1158, &Py::str("k_tx"))?, py::getitem(&t1158, &Py::str("delta_m"))?, py::getitem(&t1158, &Py::str("k_tl"))?, py::getitem(&t1158, &Py::str("delta_p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1159 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1160 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1182 = (t1160).items()?;
        if t1182.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1182[0].clone();
        v_v = t1182[1].clone();
        if ({ let t1161 = v_k.clone(); Py::Bool({ let t1162 = v_outputs.clone(); if !py::contains(&t1162, &t1161)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1205(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1207: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1206 = v_inputs.clone(); f_fick_flux(py::getitem(&t1206, &Py::str("D"))?, py::getitem(&t1206, &Py::str("C1"))?, py::getitem(&t1206, &Py::str("C2"))?, py::getitem(&t1206, &Py::str("dx"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1207 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1208 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1230 = (t1208).items()?;
        if t1230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1230[0].clone();
        v_v = t1230[1].clone();
        if ({ let t1209 = v_k.clone(); Py::Bool({ let t1210 = v_outputs.clone(); if !py::contains(&t1210, &t1209)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1252(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1254: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1253 = v_inputs.clone(); f_fraction_bound(py::getitem(&t1253, &Py::str("L"))?, py::getitem(&t1253, &Py::str("Kd"))?, py::method(&t1253, "get", &[Py::str("Rt"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1254 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1255 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1277 = (t1255).items()?;
        if t1277.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1277[0].clone();
        v_v = t1277[1].clone();
        if ({ let t1256 = v_k.clone(); Py::Bool({ let t1257 = v_outputs.clone(); if !py::contains(&t1257, &t1256)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_133(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t135: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t134 = v_inputs.clone(); f_buffer_after_addition(py::getitem(&t134, &Py::str("C"))?, py::getitem(&t134, &Py::str("V"))?, py::getitem(&t134, &Py::str("pH0"))?, py::getitem(&t134, &Py::str("pKa"))?, py::getitem(&t134, &Py::str("n_add"))?, py::getitem(&t134, &Py::str("added"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t135 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t136 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t158 = (t136).items()?;
        if t158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t158[0].clone();
        v_v = t158[1].clone();
        if ({ let t137 = v_k.clone(); Py::Bool({ let t138 = v_outputs.clone(); if !py::contains(&t138, &t137)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1331(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1333: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1332 = v_inputs.clone(); f_frap_diffusion(py::getitem(&t1332, &Py::str("w"))?, py::getitem(&t1332, &Py::str("t_half"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1333 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1334 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1356 = (t1334).items()?;
        if t1356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1356[0].clone();
        v_v = t1356[1].clone();
        if ({ let t1335 = v_k.clone(); Py::Bool({ let t1336 = v_outputs.clone(); if !py::contains(&t1336, &t1335)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1386(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1388: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1387 = v_inputs.clone(); f_gc_content(py::getitem(&t1387, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1388 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1389 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1411 = (t1389).items()?;
        if t1411.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1411[0].clone();
        v_v = t1411[1].clone();
        if ({ let t1390 = v_k.clone(); Py::Bool({ let t1391 = v_outputs.clone(); if !py::contains(&t1391, &t1390)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1437(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1439: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1438 = v_inputs.clone(); f_genome_coverage(py::getitem(&t1438, &Py::str("L"))?, py::getitem(&t1438, &Py::str("N"))?, py::getitem(&t1438, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1439 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1440 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1462 = (t1440).items()?;
        if t1462.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1462[0].clone();
        v_v = t1462[1].clone();
        if ({ let t1441 = v_k.clone(); Py::Bool({ let t1442 = v_outputs.clone(); if !py::contains(&t1442, &t1441)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1501(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1503: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1502 = v_inputs.clone(); f_growth_rate_fit(py::getitem(&t1502, &Py::str("t"))?, py::getitem(&t1502, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1503 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1504 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1526 = (t1504).items()?;
        if t1526.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1526[0].clone();
        v_v = t1526[1].clone();
        if ({ let t1505 = v_k.clone(); Py::Bool({ let t1506 = v_outputs.clone(); if !py::contains(&t1506, &t1505)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1558(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1560: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1559 = v_inputs.clone(); f_hemocytometer(py::getitem(&t1559, &Py::str("counts"))?, py::getitem(&t1559, &Py::str("dilution"))?, py::method(&t1559, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1560 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1561 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1583 = (t1561).items()?;
        if t1583.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1583[0].clone();
        v_v = t1583[1].clone();
        if ({ let t1562 = v_k.clone(); Py::Bool({ let t1563 = v_outputs.clone(); if !py::contains(&t1563, &t1562)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1603(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1605: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1604 = v_inputs.clone(); f_henderson_hasselbalch(py::getitem(&t1604, &Py::str("pKa"))?, py::getitem(&t1604, &Py::str("A"))?, py::getitem(&t1604, &Py::str("HA"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1605 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1606 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1628 = (t1606).items()?;
        if t1628.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1628[0].clone();
        v_v = t1628[1].clone();
        if ({ let t1607 = v_k.clone(); Py::Bool({ let t1608 = v_outputs.clone(); if !py::contains(&t1608, &t1607)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1649(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1651: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1650 = v_inputs.clone(); f_hill_from_ec(py::getitem(&t1650, &Py::str("EC10"))?, py::getitem(&t1650, &Py::str("EC90"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1651 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1652 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1674 = (t1652).items()?;
        if t1674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1674[0].clone();
        v_v = t1674[1].clone();
        if ({ let t1653 = v_k.clone(); Py::Bool({ let t1654 = v_outputs.clone(); if !py::contains(&t1654, &t1653)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1693(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1695: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1694 = v_inputs.clone(); f_hill_equation(py::getitem(&t1694, &Py::str("L"))?, py::getitem(&t1694, &Py::str("K"))?, py::getitem(&t1694, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1695 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1696 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1718 = (t1696).items()?;
        if t1718.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1718[0].clone();
        v_v = t1718[1].clone();
        if ({ let t1697 = v_k.clone(); Py::Bool({ let t1698 = v_outputs.clone(); if !py::contains(&t1698, &t1697)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1746(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1748: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1747 = v_inputs.clone(); f_hill_ligand_for_fraction(py::getitem(&t1747, &Py::str("K"))?, py::getitem(&t1747, &Py::str("n"))?, py::getitem(&t1747, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1748 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1749 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1771 = (t1749).items()?;
        if t1771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1771[0].clone();
        v_v = t1771[1].clone();
        if ({ let t1750 = v_k.clone(); Py::Bool({ let t1751 = v_outputs.clone(); if !py::contains(&t1751, &t1750)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_178(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t180: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t179 = v_inputs.clone(); f_buffer_capacity(py::getitem(&t179, &Py::str("C"))?, py::getitem(&t179, &Py::str("pKa"))?, py::getitem(&t179, &Py::str("pH"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t180 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t181 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t203 = (t181).items()?;
        if t203.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t203[0].clone();
        v_v = t203[1].clone();
        if ({ let t182 = v_k.clone(); Py::Bool({ let t183 = v_outputs.clone(); if !py::contains(&t183, &t182)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1796(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1798: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1797 = v_inputs.clone(); f_ic50_to_ki(py::getitem(&t1797, &Py::str("mode"))?, py::getitem(&t1797, &Py::str("IC50"))?, py::getitem(&t1797, &Py::str("S"))?, py::getitem(&t1797, &Py::str("Km"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1798 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1799 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1821 = (t1799).items()?;
        if t1821.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1821[0].clone();
        v_v = t1821[1].clone();
        if ({ let t1800 = v_k.clone(); Py::Bool({ let t1801 = v_outputs.clone(); if !py::contains(&t1801, &t1800)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1846(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1848: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1847 = v_inputs.clone(); f_induction_dynamics(py::getitem(&t1847, &Py::str("k_tx"))?, py::getitem(&t1847, &Py::str("delta_m"))?, py::getitem(&t1847, &Py::str("k_tl"))?, py::getitem(&t1847, &Py::str("delta_p"))?, py::getitem(&t1847, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1848 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1849 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1871 = (t1849).items()?;
        if t1871.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1871[0].clone();
        v_v = t1871[1].clone();
        if ({ let t1850 = v_k.clone(); Py::Bool({ let t1851 = v_outputs.clone(); if !py::contains(&t1851, &t1850)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1903(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1905: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1904 = v_inputs.clone(); f_ionic_strength(py::getitem(&t1904, &Py::str("c"))?, py::getitem(&t1904, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1905 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1906 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1928 = (t1906).items()?;
        if t1928.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1928[0].clone();
        v_v = t1928[1].clone();
        if ({ let t1907 = v_k.clone(); Py::Bool({ let t1908 = v_outputs.clone(); if !py::contains(&t1908, &t1907)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1947(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1949: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1948 = v_inputs.clone(); f_kcat_efficiency(py::getitem(&t1948, &Py::str("Vmax"))?, py::getitem(&t1948, &Py::str("Et"))?, py::getitem(&t1948, &Py::str("Km"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1949 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1950 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1972 = (t1950).items()?;
        if t1972.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1972[0].clone();
        v_v = t1972[1].clone();
        if ({ let t1951 = v_k.clone(); Py::Bool({ let t1952 = v_outputs.clone(); if !py::contains(&t1952, &t1951)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1991(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1993: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1992 = v_inputs.clone(); f_kd_from_rates(py::getitem(&t1992, &Py::str("kon"))?, py::getitem(&t1992, &Py::str("koff"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1993 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1994 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2016 = (t1994).items()?;
        if t2016.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2016[0].clone();
        v_v = t2016[1].clone();
        if ({ let t1995 = v_k.clone(); Py::Bool({ let t1996 = v_outputs.clone(); if !py::contains(&t1996, &t1995)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2046(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2048: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2047 = v_inputs.clone(); f_library_size(py::getitem(&t2047, &Py::str("G"))?, py::getitem(&t2047, &Py::str("I"))?, py::getitem(&t2047, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2048 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2049 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2071 = (t2049).items()?;
        if t2071.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2071[0].clone();
        v_v = t2071[1].clone();
        if ({ let t2050 = v_k.clone(); Py::Bool({ let t2051 = v_outputs.clone(); if !py::contains(&t2051, &t2050)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2090(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2092: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2091 = v_inputs.clone(); f_ligand_depletion_binding(py::getitem(&t2091, &Py::str("Rt"))?, py::getitem(&t2091, &Py::str("Lt"))?, py::getitem(&t2091, &Py::str("Kd"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2092 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2093 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2115 = (t2093).items()?;
        if t2115.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2115[0].clone();
        v_v = t2115[1].clone();
        if ({ let t2094 = v_k.clone(); Py::Bool({ let t2095 = v_outputs.clone(); if !py::contains(&t2095, &t2094)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2141(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2143: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2142 = v_inputs.clone(); f_ligation_insert_mass(py::getitem(&t2142, &Py::str("m_vector"))?, py::getitem(&t2142, &Py::str("L_vector"))?, py::getitem(&t2142, &Py::str("L_insert"))?, py::getitem(&t2142, &Py::str("ratio"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2143 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2144 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2166 = (t2144).items()?;
        if t2166.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2166[0].clone();
        v_v = t2166[1].clone();
        if ({ let t2145 = v_k.clone(); Py::Bool({ let t2146 = v_outputs.clone(); if !py::contains(&t2146, &t2145)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2211(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2213: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2212 = v_inputs.clone(); f_lineweaver_burk(py::getitem(&t2212, &Py::str("S"))?, py::getitem(&t2212, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2213 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2214 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2236 = (t2214).items()?;
        if t2236.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2236[0].clone();
        v_v = t2236[1].clone();
        if ({ let t2215 = v_k.clone(); Py::Bool({ let t2216 = v_outputs.clone(); if !py::contains(&t2216, &t2215)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2261(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2263: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2262 = v_inputs.clone(); f_logistic_growth(py::getitem(&t2262, &Py::str("N0"))?, py::getitem(&t2262, &Py::str("K"))?, py::getitem(&t2262, &Py::str("r"))?, py::getitem(&t2262, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2263 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2264 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2286 = (t2264).items()?;
        if t2286.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2286[0].clone();
        v_v = t2286[1].clone();
        if ({ let t2265 = v_k.clone(); Py::Bool({ let t2266 = v_outputs.clone(); if !py::contains(&t2266, &t2265)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_228(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t230: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t229 = v_inputs.clone(); f_buffer_preparation(py::getitem(&t229, &Py::str("C"))?, py::getitem(&t229, &Py::str("V"))?, py::getitem(&t229, &Py::str("pH"))?, py::getitem(&t229, &Py::str("pKa"))?, py::method(&t229, "get", &[Py::str("MW_acid"), Py::None])?, py::method(&t229, "get", &[Py::str("MW_base"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t230 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t231 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t253 = (t231).items()?;
        if t253.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t253[0].clone();
        v_v = t253[1].clone();
        if ({ let t232 = v_k.clone(); Py::Bool({ let t233 = v_outputs.clone(); if !py::contains(&t233, &t232)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2308(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2310: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2309 = v_inputs.clone(); f_logistic_time_to_reach(py::getitem(&t2309, &Py::str("N0"))?, py::getitem(&t2309, &Py::str("K"))?, py::getitem(&t2309, &Py::str("r"))?, py::getitem(&t2309, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2310 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2311 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2333 = (t2311).items()?;
        if t2333.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2333[0].clone();
        v_v = t2333[1].clone();
        if ({ let t2312 = v_k.clone(); Py::Bool({ let t2313 = v_outputs.clone(); if !py::contains(&t2313, &t2312)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2352(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2354: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2353 = v_inputs.clone(); f_mass_for_molarity(py::getitem(&t2353, &Py::str("C"))?, py::getitem(&t2353, &Py::str("V"))?, py::getitem(&t2353, &Py::str("MW"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2354 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2355 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2377 = (t2355).items()?;
        if t2377.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2377[0].clone();
        v_v = t2377[1].clone();
        if ({ let t2356 = v_k.clone(); Py::Bool({ let t2357 = v_outputs.clone(); if !py::contains(&t2357, &t2356)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2400(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2402: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2401 = v_inputs.clone(); f_membrane_permeability(py::getitem(&t2401, &Py::str("D"))?, py::getitem(&t2401, &Py::str("K"))?, py::getitem(&t2401, &Py::str("L"))?, py::getitem(&t2401, &Py::str("C_out"))?, py::getitem(&t2401, &Py::str("C_in"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2402 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2403 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2425 = (t2403).items()?;
        if t2425.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2425[0].clone();
        v_v = t2425[1].clone();
        if ({ let t2404 = v_k.clone(); Py::Bool({ let t2405 = v_outputs.clone(); if !py::contains(&t2405, &t2404)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2504(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2506: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2505 = v_inputs.clone(); f_mm_nonlinear_fit(py::getitem(&t2505, &Py::str("S"))?, py::getitem(&t2505, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2506 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2507 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2529 = (t2507).items()?;
        if t2529.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2529[0].clone();
        v_v = t2529[1].clone();
        if ({ let t2508 = v_k.clone(); Py::Bool({ let t2509 = v_outputs.clone(); if !py::contains(&t2509, &t2508)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2552(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2554: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2553 = v_inputs.clone(); f_michaelis_menten(py::getitem(&t2553, &Py::str("Vmax"))?, py::getitem(&t2553, &Py::str("Km"))?, py::getitem(&t2553, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2554 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2555 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2577 = (t2555).items()?;
        if t2577.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2577[0].clone();
        v_v = t2577[1].clone();
        if ({ let t2556 = v_k.clone(); Py::Bool({ let t2557 = v_outputs.clone(); if !py::contains(&t2557, &t2556)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2601(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2603: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2602 = v_inputs.clone(); f_mixed_inhibition(py::getitem(&t2602, &Py::str("Vmax"))?, py::getitem(&t2602, &Py::str("Km"))?, py::getitem(&t2602, &Py::str("S"))?, py::getitem(&t2602, &Py::str("I"))?, py::getitem(&t2602, &Py::str("Ki"))?, py::getitem(&t2602, &Py::str("Kii"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2603 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2604 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2626 = (t2604).items()?;
        if t2626.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2626[0].clone();
        v_v = t2626[1].clone();
        if ({ let t2605 = v_k.clone(); Py::Bool({ let t2606 = v_outputs.clone(); if !py::contains(&t2606, &t2605)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2645(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2647: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2646 = v_inputs.clone(); f_molarity_from_mass(py::getitem(&t2646, &Py::str("m"))?, py::getitem(&t2646, &Py::str("MW"))?, py::getitem(&t2646, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2647 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2648 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2670 = (t2648).items()?;
        if t2670.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2670[0].clone();
        v_v = t2670[1].clone();
        if ({ let t2649 = v_k.clone(); Py::Bool({ let t2650 = v_outputs.clone(); if !py::contains(&t2650, &t2649)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2689(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2691: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2690 = v_inputs.clone(); f_molecules_per_cell(py::getitem(&t2690, &Py::str("c"))?, py::getitem(&t2690, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2691 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2692 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2714 = (t2692).items()?;
        if t2714.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2714[0].clone();
        v_v = t2714[1].clone();
        if ({ let t2693 = v_k.clone(); Py::Bool({ let t2694 = v_outputs.clone(); if !py::contains(&t2694, &t2693)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2736(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2738: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2737 = v_inputs.clone(); f_monod_batch_time(py::getitem(&t2737, &Py::str("X0"))?, py::getitem(&t2737, &Py::str("S0"))?, py::getitem(&t2737, &Py::str("Y"))?, py::getitem(&t2737, &Py::str("mu_max"))?, py::getitem(&t2737, &Py::str("Ks"))?, py::getitem(&t2737, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2738 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2739 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2761 = (t2739).items()?;
        if t2761.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2761[0].clone();
        v_v = t2761[1].clone();
        if ({ let t2740 = v_k.clone(); Py::Bool({ let t2741 = v_outputs.clone(); if !py::contains(&t2741, &t2740)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_274(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t276: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t275 = v_inputs.clone(); f_c1v1_dilution(py::getitem(&t275, &Py::str("C1"))?, py::getitem(&t275, &Py::str("C2"))?, py::getitem(&t275, &Py::str("V2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t276 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t277 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t299 = (t277).items()?;
        if t299.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t299[0].clone();
        v_v = t299[1].clone();
        if ({ let t278 = v_k.clone(); Py::Bool({ let t279 = v_outputs.clone(); if !py::contains(&t279, &t278)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2787(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2789: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2788 = v_inputs.clone(); f_monod_growth(py::getitem(&t2788, &Py::str("mu_max"))?, py::getitem(&t2788, &Py::str("S"))?, py::getitem(&t2788, &Py::str("Ks"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2789 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2790 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2812 = (t2790).items()?;
        if t2812.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2812[0].clone();
        v_v = t2812[1].clone();
        if ({ let t2791 = v_k.clone(); Py::Bool({ let t2792 = v_outputs.clone(); if !py::contains(&t2792, &t2791)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2849(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2851: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2850 = v_inputs.clone(); f_mrna_half_life_fit(py::getitem(&t2850, &Py::str("t"))?, py::getitem(&t2850, &Py::str("level"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2851 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2852 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2874 = (t2852).items()?;
        if t2874.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2874[0].clone();
        v_v = t2874[1].clone();
        if ({ let t2853 = v_k.clone(); Py::Bool({ let t2854 = v_outputs.clone(); if !py::contains(&t2854, &t2853)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2901(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2903: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2902 = v_inputs.clone(); f_mutation_rate_p0(py::getitem(&t2902, &Py::str("n0"))?, py::getitem(&t2902, &Py::str("n"))?, py::getitem(&t2902, &Py::str("Nt"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2903 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2904 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2926 = (t2904).items()?;
        if t2926.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2926[0].clone();
        v_v = t2926[1].clone();
        if ({ let t2905 = v_k.clone(); Py::Bool({ let t2906 = v_outputs.clone(); if !py::contains(&t2906, &t2905)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2949(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2951: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2950 = v_inputs.clone(); f_mwc_saturation(py::getitem(&t2950, &Py::str("S"))?, py::getitem(&t2950, &Py::str("KR"))?, py::getitem(&t2950, &Py::str("c"))?, py::getitem(&t2950, &Py::str("L0"))?, py::getitem(&t2950, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2951 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2952 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2974 = (t2952).items()?;
        if t2974.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2974[0].clone();
        v_v = t2974[1].clone();
        if ({ let t2953 = v_k.clone(); Py::Bool({ let t2954 = v_outputs.clone(); if !py::contains(&t2954, &t2953)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2997(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2999: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2998 = v_inputs.clone(); f_nernst_potential(py::getitem(&t2998, &Py::str("z"))?, py::getitem(&t2998, &Py::str("C_out"))?, py::getitem(&t2998, &Py::str("C_in"))?, py::getitem(&t2998, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2999 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3000 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3022 = (t3000).items()?;
        if t3022.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3022[0].clone();
        v_v = t3022[1].clone();
        if ({ let t3001 = v_k.clone(); Py::Bool({ let t3002 = v_outputs.clone(); if !py::contains(&t3002, &t3001)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3023(mut v_Vmax: Py, mut v_Km: Py, mut v_S: Py, mut v_I: Py, mut v_Ki: Py) -> R {
    return Ok(f__inhibited(v_Vmax.clone(), v_Km.clone(), v_S.clone(), v_I.clone(), v_Ki.clone(), Py::str("noncompetitive"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3055(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3057: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3056 = v_inputs.clone(); s_3023(py::getitem(&t3056, &Py::str("Vmax"))?, py::getitem(&t3056, &Py::str("Km"))?, py::getitem(&t3056, &Py::str("S"))?, py::getitem(&t3056, &Py::str("I"))?, py::getitem(&t3056, &Py::str("Ki"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3057 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3058 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3080 = (t3058).items()?;
        if t3080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3080[0].clone();
        v_v = t3080[1].clone();
        if ({ let t3059 = v_k.clone(); Py::Bool({ let t3060 = v_outputs.clone(); if !py::contains(&t3060, &t3059)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3111(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3113: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3112 = v_inputs.clone(); f_nucleic_acid_a260(py::getitem(&t3112, &Py::str("A260"))?, py::getitem(&t3112, &Py::str("molecule"))?, py::method(&t3112, "get", &[Py::str("dilution"), Py::Float(1.0)])?, py::method(&t3112, "get", &[Py::str("l"), Py::Float(0.01)])?, py::method(&t3112, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3113 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3114 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3136 = (t3114).items()?;
        if t3136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3136[0].clone();
        v_v = t3136[1].clone();
        if ({ let t3115 = v_k.clone(); Py::Bool({ let t3116 = v_outputs.clone(); if !py::contains(&t3116, &t3115)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3170(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3172: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3171 = v_inputs.clone(); f_oligo_extinction(py::getitem(&t3171, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3172 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3173 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3195 = (t3173).items()?;
        if t3195.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3195[0].clone();
        v_v = t3195[1].clone();
        if ({ let t3174 = v_k.clone(); Py::Bool({ let t3175 = v_outputs.clone(); if !py::contains(&t3175, &t3174)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3230(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3232: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3231 = v_inputs.clone(); f_oligo_molecular_weight(py::getitem(&t3231, &Py::str("seq"))?, py::getitem(&t3231, &Py::str("strands"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3232 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3233 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3255 = (t3233).items()?;
        if t3255.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3255[0].clone();
        v_v = t3255[1].clone();
        if ({ let t3234 = v_k.clone(); Py::Bool({ let t3235 = v_outputs.clone(); if !py::contains(&t3235, &t3234)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_330(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t332: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t331 = v_inputs.clone(); f_cell_cycle_age_corrected(py::getitem(&t331, &Py::str("Tc"))?, py::getitem(&t331, &Py::str("fractions"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t332 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t333 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t355 = (t333).items()?;
        if t355.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t355[0].clone();
        v_v = t355[1].clone();
        if ({ let t334 = v_k.clone(); Py::Bool({ let t335 = v_outputs.clone(); if !py::contains(&t335, &t334)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3320(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3322: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3321 = v_inputs.clone(); f_orf_finder(py::getitem(&t3321, &Py::str("seq"))?, py::getitem(&t3321, &Py::str("min_aa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3322 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3323 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3345 = (t3323).items()?;
        if t3345.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3345[0].clone();
        v_v = t3345[1].clone();
        if ({ let t3324 = v_k.clone(); Py::Bool({ let t3325 = v_outputs.clone(); if !py::contains(&t3325, &t3324)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3382(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3384: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3383 = v_inputs.clone(); f_osmolarity_tonicity(py::getitem(&t3383, &Py::str("c"))?, py::getitem(&t3383, &Py::str("i"))?, py::getitem(&t3383, &Py::str("osm_cell"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3384 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3385 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3407 = (t3385).items()?;
        if t3407.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3407[0].clone();
        v_v = t3407[1].clone();
        if ({ let t3386 = v_k.clone(); Py::Bool({ let t3387 = v_outputs.clone(); if !py::contains(&t3387, &t3386)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_34(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t36: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t35 = v_inputs.clone(); f_absorbance_ratios(py::getitem(&t35, &Py::str("A260"))?, py::getitem(&t35, &Py::str("A280"))?, py::method(&t35, "get", &[Py::str("A230"), Py::None])?, py::method(&t35, "get", &[Py::str("A320"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t36 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t37 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t59 = (t37).items()?;
        if t59.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t59[0].clone();
        v_v = t59[1].clone();
        if ({ let t38 = v_k.clone(); Py::Bool({ let t39 = v_outputs.clone(); if !py::contains(&t39, &t38)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3426(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3428: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3427 = v_inputs.clone(); f_osmotic_pressure(py::getitem(&t3427, &Py::str("i"))?, py::getitem(&t3427, &Py::str("c"))?, py::getitem(&t3427, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3428 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3429 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3451 = (t3429).items()?;
        if t3451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3451[0].clone();
        v_v = t3451[1].clone();
        if ({ let t3430 = v_k.clone(); Py::Bool({ let t3431 = v_outputs.clone(); if !py::contains(&t3431, &t3430)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3483(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3485: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3484 = v_inputs.clone(); f_pcr_amplification(py::getitem(&t3484, &Py::str("N0"))?, py::getitem(&t3484, &Py::str("E"))?, py::getitem(&t3484, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3485 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3486 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3508 = (t3486).items()?;
        if t3508.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3508[0].clone();
        v_v = t3508[1].clone();
        if ({ let t3487 = v_k.clone(); Py::Bool({ let t3488 = v_outputs.clone(); if !py::contains(&t3488, &t3487)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3540(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3542: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3541 = v_inputs.clone(); f_pcr_cycles_needed(py::getitem(&t3541, &Py::str("N0"))?, py::getitem(&t3541, &Py::str("N"))?, py::getitem(&t3541, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3542 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3543 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3565 = (t3543).items()?;
        if t3565.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3565[0].clone();
        v_v = t3565[1].clone();
        if ({ let t3544 = v_k.clone(); Py::Bool({ let t3545 = v_outputs.clone(); if !py::contains(&t3545, &t3544)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3593(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3595: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3594 = v_inputs.clone(); f_percent_to_molarity(py::getitem(&t3594, &Py::str("p"))?, py::getitem(&t3594, &Py::str("MW"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3595 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3596 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3618 = (t3596).items()?;
        if t3618.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3618[0].clone();
        v_v = t3618[1].clone();
        if ({ let t3597 = v_k.clone(); Py::Bool({ let t3598 = v_outputs.clone(); if !py::contains(&t3598, &t3597)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3645(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3647: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3646 = v_inputs.clone(); f_cfu_count(py::getitem(&t3646, &Py::str("colonies"))?, py::getitem(&t3646, &Py::str("dilution"))?, py::getitem(&t3646, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3647 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3648 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3670 = (t3648).items()?;
        if t3670.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3670[0].clone();
        v_v = t3670[1].clone();
        if ({ let t3649 = v_k.clone(); Py::Bool({ let t3650 = v_outputs.clone(); if !py::contains(&t3650, &t3649)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3696(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3698: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3697 = v_inputs.clone(); f_protein_a280(py::getitem(&t3697, &Py::str("A280"))?, py::getitem(&t3697, &Py::str("eps"))?, py::method(&t3697, "get", &[Py::str("MW"), Py::None])?, py::method(&t3697, "get", &[Py::str("l"), Py::Float(0.01)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3698 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3699 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3721 = (t3699).items()?;
        if t3721.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3721[0].clone();
        v_v = t3721[1].clone();
        if ({ let t3700 = v_k.clone(); Py::Bool({ let t3701 = v_outputs.clone(); if !py::contains(&t3701, &t3700)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3742(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3744: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3743 = v_inputs.clone(); f_q10_rate(py::getitem(&t3743, &Py::str("k1"))?, py::getitem(&t3743, &Py::str("T1"))?, py::getitem(&t3743, &Py::str("T2"))?, py::getitem(&t3743, &Py::str("Q10"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3744 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3745 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3767 = (t3745).items()?;
        if t3767.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3767[0].clone();
        v_v = t3767[1].clone();
        if ({ let t3746 = v_k.clone(); Py::Bool({ let t3747 = v_outputs.clone(); if !py::contains(&t3747, &t3746)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3787(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3789: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3788 = v_inputs.clone(); f_qpcr_ddct(py::getitem(&t3788, &Py::str("Ct_target_treated"))?, py::getitem(&t3788, &Py::str("Ct_ref_treated"))?, py::getitem(&t3788, &Py::str("Ct_target_control"))?, py::getitem(&t3788, &Py::str("Ct_ref_control"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3789 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3790 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3812 = (t3790).items()?;
        if t3812.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3812[0].clone();
        v_v = t3812[1].clone();
        if ({ let t3791 = v_k.clone(); Py::Bool({ let t3792 = v_outputs.clone(); if !py::contains(&t3792, &t3791)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_383(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t385: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t384 = v_inputs.clone(); f_cell_cycle_simple(py::getitem(&t384, &Py::str("Tc"))?, py::getitem(&t384, &Py::str("fractions"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t385 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t386 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t408 = (t386).items()?;
        if t408.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t408[0].clone();
        v_v = t408[1].clone();
        if ({ let t387 = v_k.clone(); Py::Bool({ let t388 = v_outputs.clone(); if !py::contains(&t388, &t387)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3841(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3843: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3842 = v_inputs.clone(); f_qpcr_pfaffl(py::getitem(&t3842, &Py::str("E_target"))?, py::getitem(&t3842, &Py::str("E_ref"))?, py::getitem(&t3842, &Py::str("Ct_target_control"))?, py::getitem(&t3842, &Py::str("Ct_target_sample"))?, py::getitem(&t3842, &Py::str("Ct_ref_control"))?, py::getitem(&t3842, &Py::str("Ct_ref_sample"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3843 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3844 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3866 = (t3844).items()?;
        if t3866.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3866[0].clone();
        v_v = t3866[1].clone();
        if ({ let t3845 = v_k.clone(); Py::Bool({ let t3846 = v_outputs.clone(); if !py::contains(&t3846, &t3845)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3885(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3887: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3886 = v_inputs.clone(); f_qpcr_quantify(py::getitem(&t3886, &Py::str("slope"))?, py::getitem(&t3886, &Py::str("intercept"))?, py::getitem(&t3886, &Py::str("Ct"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3887 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3888 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3910 = (t3888).items()?;
        if t3910.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3910[0].clone();
        v_v = t3910[1].clone();
        if ({ let t3889 = v_k.clone(); Py::Bool({ let t3890 = v_outputs.clone(); if !py::contains(&t3890, &t3889)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3947(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3949: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3948 = v_inputs.clone(); f_qpcr_standard_curve(py::getitem(&t3948, &Py::str("quantity"))?, py::getitem(&t3948, &Py::str("Ct"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3949 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3950 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3972 = (t3950).items()?;
        if t3972.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3972[0].clone();
        v_v = t3972[1].clone();
        if ({ let t3951 = v_k.clone(); Py::Bool({ let t3952 = v_outputs.clone(); if !py::contains(&t3952, &t3951)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4004(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4006: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4005 = v_inputs.clone(); f_reverse_complement(py::getitem(&t4005, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4006 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4007 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4029 = (t4007).items()?;
        if t4029.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4029[0].clone();
        v_v = t4029[1].clone();
        if ({ let t4008 = v_k.clone(); Py::Bool({ let t4009 = v_outputs.clone(); if !py::contains(&t4009, &t4008)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4050(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4052: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4051 = v_inputs.clone(); f_rms_displacement(py::getitem(&t4051, &Py::str("D"))?, py::getitem(&t4051, &Py::str("t"))?, py::getitem(&t4051, &Py::str("dims"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4052 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4053 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4075 = (t4053).items()?;
        if t4075.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4075[0].clone();
        v_v = t4075[1].clone();
        if ({ let t4054 = v_k.clone(); Py::Bool({ let t4055 = v_outputs.clone(); if !py::contains(&t4055, &t4054)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4116(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4118: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4117 = v_inputs.clone(); f_scatchard_fit(py::getitem(&t4117, &Py::str("B"))?, py::getitem(&t4117, &Py::str("F"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4118 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4119 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4141 = (t4119).items()?;
        if t4141.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4141[0].clone();
        v_v = t4141[1].clone();
        if ({ let t4120 = v_k.clone(); Py::Bool({ let t4121 = v_outputs.clone(); if !py::contains(&t4121, &t4120)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4162(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4164: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4163 = v_inputs.clone(); f_sedimentation_time(py::getitem(&t4163, &Py::str("s"))?, py::getitem(&t4163, &Py::str("rpm"))?, py::getitem(&t4163, &Py::str("r_min"))?, py::getitem(&t4163, &Py::str("r_max"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4164 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4165 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4187 = (t4165).items()?;
        if t4187.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4187[0].clone();
        v_v = t4187[1].clone();
        if ({ let t4166 = v_k.clone(); Py::Bool({ let t4167 = v_outputs.clone(); if !py::contains(&t4167, &t4166)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4217(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4219: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4218 = v_inputs.clone(); f_serial_dilution(py::getitem(&t4218, &Py::str("C0"))?, py::getitem(&t4218, &Py::str("f"))?, py::getitem(&t4218, &Py::str("n"))?, py::method(&t4218, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4219 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4220 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4242 = (t4220).items()?;
        if t4242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4242[0].clone();
        v_v = t4242[1].clone();
        if ({ let t4221 = v_k.clone(); Py::Bool({ let t4222 = v_outputs.clone(); if !py::contains(&t4222, &t4221)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4261(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4263: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4262 = v_inputs.clone(); f_stokes_einstein(py::getitem(&t4262, &Py::str("T"))?, py::getitem(&t4262, &Py::str("eta"))?, py::getitem(&t4262, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4263 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4264 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4286 = (t4264).items()?;
        if t4286.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4286[0].clone();
        v_v = t4286[1].clone();
        if ({ let t4265 = v_k.clone(); Py::Bool({ let t4266 = v_outputs.clone(); if !py::contains(&t4266, &t4265)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_431(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t433: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t432 = v_inputs.clone(); f_cell_equilibration(py::getitem(&t432, &Py::str("r"))?, py::getitem(&t432, &Py::str("P"))?, py::getitem(&t432, &Py::str("C_out"))?, py::getitem(&t432, &Py::str("C0"))?, py::getitem(&t432, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t433 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t434 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t456 = (t434).items()?;
        if t456.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t456[0].clone();
        v_v = t456[1].clone();
        if ({ let t435 = v_k.clone(); Py::Bool({ let t436 = v_outputs.clone(); if !py::contains(&t436, &t435)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4314(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4316: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4315 = v_inputs.clone(); f_substrate_for_fraction(py::getitem(&t4315, &Py::str("Km"))?, py::getitem(&t4315, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4316 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4317 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4339 = (t4317).items()?;
        if t4339.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4339[0].clone();
        v_v = t4339[1].clone();
        if ({ let t4318 = v_k.clone(); Py::Bool({ let t4319 = v_outputs.clone(); if !py::contains(&t4319, &t4318)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4370(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4372: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4371 = v_inputs.clone(); f_tm_basic(py::getitem(&t4371, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4372 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4373 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4395 = (t4373).items()?;
        if t4395.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4395[0].clone();
        v_v = t4395[1].clone();
        if ({ let t4374 = v_k.clone(); Py::Bool({ let t4375 = v_outputs.clone(); if !py::contains(&t4375, &t4374)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4447(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4449: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4448 = v_inputs.clone(); f_tm_nearest_neighbor(py::getitem(&t4448, &Py::str("seq"))?, py::getitem(&t4448, &Py::str("Na"))?, py::getitem(&t4448, &Py::str("c1"))?, py::getitem(&t4448, &Py::str("c2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4449 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4450 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4472 = (t4450).items()?;
        if t4472.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4472[0].clone();
        v_v = t4472[1].clone();
        if ({ let t4451 = v_k.clone(); Py::Bool({ let t4452 = v_outputs.clone(); if !py::contains(&t4452, &t4451)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4509(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4511: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4510 = v_inputs.clone(); f_tm_salt_adjusted(py::getitem(&t4510, &Py::str("seq"))?, py::getitem(&t4510, &Py::str("Na"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4511 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4512 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4534 = (t4512).items()?;
        if t4534.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4534[0].clone();
        v_v = t4534[1].clone();
        if ({ let t4513 = v_k.clone(); Py::Bool({ let t4514 = v_outputs.clone(); if !py::contains(&t4514, &t4513)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4566(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4568: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4567 = v_inputs.clone(); f_tm_wallace(py::getitem(&t4567, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4568 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4569 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4591 = (t4569).items()?;
        if t4591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4591[0].clone();
        v_v = t4591[1].clone();
        if ({ let t4570 = v_k.clone(); Py::Bool({ let t4571 = v_outputs.clone(); if !py::contains(&t4571, &t4570)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4628(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4630: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4629 = v_inputs.clone(); f_transcribe(py::getitem(&t4629, &Py::str("seq"))?, py::getitem(&t4629, &Py::str("strand"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4630 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4631 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4653 = (t4631).items()?;
        if t4653.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4653[0].clone();
        v_v = t4653[1].clone();
        if ({ let t4632 = v_k.clone(); Py::Bool({ let t4633 = v_outputs.clone(); if !py::contains(&t4633, &t4632)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4678(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4680: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4679 = v_inputs.clone(); f_transcription_time(py::getitem(&t4679, &Py::str("n"))?, py::getitem(&t4679, &Py::str("rate"))?, py::method(&t4679, "get", &[Py::str("k_init"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4680 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4681 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4703 = (t4681).items()?;
        if t4703.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4703[0].clone();
        v_v = t4703[1].clone();
        if ({ let t4682 = v_k.clone(); Py::Bool({ let t4683 = v_outputs.clone(); if !py::contains(&t4683, &t4682)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4729(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4731: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4730 = v_inputs.clone(); f_transformation_efficiency(py::getitem(&t4730, &Py::str("colonies"))?, py::getitem(&t4730, &Py::str("m_DNA"))?, py::getitem(&t4730, &Py::str("V_plated"))?, py::getitem(&t4730, &Py::str("V_total"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4731 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4732 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4754 = (t4732).items()?;
        if t4754.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4754[0].clone();
        v_v = t4754[1].clone();
        if ({ let t4733 = v_k.clone(); Py::Bool({ let t4734 = v_outputs.clone(); if !py::contains(&t4734, &t4733)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_475(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t477: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t476 = v_inputs.clone(); f_cell_geometry(py::getitem(&t476, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t477 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t478 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t500 = (t478).items()?;
        if t500.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t500[0].clone();
        v_v = t500[1].clone();
        if ({ let t479 = v_k.clone(); Py::Bool({ let t480 = v_outputs.clone(); if !py::contains(&t480, &t479)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4788(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4790: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4789 = v_inputs.clone(); f_translate(py::getitem(&t4789, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4790 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4791 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4813 = (t4791).items()?;
        if t4813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4813[0].clone();
        v_v = t4813[1].clone();
        if ({ let t4792 = v_k.clone(); Py::Bool({ let t4793 = v_outputs.clone(); if !py::contains(&t4793, &t4792)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4838(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4840: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4839 = v_inputs.clone(); f_translation_time(py::getitem(&t4839, &Py::str("n_aa"))?, py::getitem(&t4839, &Py::str("rate"))?, py::method(&t4839, "get", &[Py::str("k_init"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4840 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4841 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4863 = (t4841).items()?;
        if t4863.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4863[0].clone();
        v_v = t4863[1].clone();
        if ({ let t4842 = v_k.clone(); Py::Bool({ let t4843 = v_outputs.clone(); if !py::contains(&t4843, &t4842)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4864(mut v_Vmax: Py, mut v_Km: Py, mut v_S: Py, mut v_I: Py, mut v_Ki: Py) -> R {
    return Ok(f__inhibited(v_Vmax.clone(), v_Km.clone(), v_S.clone(), v_I.clone(), v_Ki.clone(), Py::str("uncompetitive"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4896(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4898: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4897 = v_inputs.clone(); s_4864(py::getitem(&t4897, &Py::str("Vmax"))?, py::getitem(&t4897, &Py::str("Km"))?, py::getitem(&t4897, &Py::str("S"))?, py::getitem(&t4897, &Py::str("I"))?, py::getitem(&t4897, &Py::str("Ki"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4898 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4899 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4921 = (t4899).items()?;
        if t4921.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4921[0].clone();
        v_v = t4921[1].clone();
        if ({ let t4900 = v_k.clone(); Py::Bool({ let t4901 = v_outputs.clone(); if !py::contains(&t4901, &t4900)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4944(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4946: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4945 = v_inputs.clone(); f_warburg_christian(py::getitem(&t4945, &Py::str("A280"))?, py::getitem(&t4945, &Py::str("A260"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4946 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4947 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4969 = (t4947).items()?;
        if t4969.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4969[0].clone();
        v_v = t4969[1].clone();
        if ({ let t4948 = v_k.clone(); Py::Bool({ let t4949 = v_outputs.clone(); if !py::contains(&t4949, &t4948)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_528(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t530: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t529 = v_inputs.clone(); f_cell_volume_osmotic(py::getitem(&t529, &Py::str("V0"))?, py::getitem(&t529, &Py::str("b"))?, py::getitem(&t529, &Py::str("osm0"))?, py::getitem(&t529, &Py::str("osm1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t530 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t531 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t553 = (t531).items()?;
        if t553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t553[0].clone();
        v_v = t553[1].clone();
        if ({ let t532 = v_k.clone(); Py::Bool({ let t533 = v_outputs.clone(); if !py::contains(&t533, &t532)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_572(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t574: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t573 = v_inputs.clone(); f_centrifuge_rcf(py::getitem(&t573, &Py::str("rpm"))?, py::getitem(&t573, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t574 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t575 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t597 = (t575).items()?;
        if t597.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t597[0].clone();
        v_v = t597[1].clone();
        if ({ let t576 = v_k.clone(); Py::Bool({ let t577 = v_outputs.clone(); if !py::contains(&t577, &t576)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_616(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t618: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t617 = v_inputs.clone(); f_centrifuge_rpm(py::getitem(&t617, &Py::str("RCF"))?, py::getitem(&t617, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t618 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t619 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t641 = (t619).items()?;
        if t641.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t641[0].clone();
        v_v = t641[1].clone();
        if ({ let t620 = v_k.clone(); Py::Bool({ let t621 = v_outputs.clone(); if !py::contains(&t621, &t620)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_673(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t675: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t674 = v_inputs.clone(); f_channel_current(py::getitem(&t674, &Py::str("N"))?, py::getitem(&t674, &Py::str("Po"))?, py::getitem(&t674, &Py::str("gamma"))?, py::getitem(&t674, &Py::str("V"))?, py::getitem(&t674, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t675 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t676 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t698 = (t676).items()?;
        if t698.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t698[0].clone();
        v_v = t698[1].clone();
        if ({ let t677 = v_k.clone(); Py::Bool({ let t678 = v_outputs.clone(); if !py::contains(&t678, &t677)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_699(mut v_Vmax: Py, mut v_Km: Py, mut v_S: Py, mut v_I: Py, mut v_Ki: Py) -> R {
    return Ok(f__inhibited(v_Vmax.clone(), v_Km.clone(), v_S.clone(), v_I.clone(), v_Ki.clone(), Py::str("competitive"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_731(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t733: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t732 = v_inputs.clone(); s_699(py::getitem(&t732, &Py::str("Vmax"))?, py::getitem(&t732, &Py::str("Km"))?, py::getitem(&t732, &Py::str("S"))?, py::getitem(&t732, &Py::str("I"))?, py::getitem(&t732, &Py::str("Ki"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t733 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t734 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t756 = (t734).items()?;
        if t756.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t756[0].clone();
        v_v = t756[1].clone();
        if ({ let t735 = v_k.clone(); Py::Bool({ let t736 = v_outputs.clone(); if !py::contains(&t736, &t735)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_779(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t781: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t780 = v_inputs.clone(); f_competitive_occupancy(py::getitem(&t780, &Py::str("L"))?, py::getitem(&t780, &Py::str("Kd"))?, py::getitem(&t780, &Py::str("I"))?, py::getitem(&t780, &Py::str("Ki"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t781 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t782 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t804 = (t782).items()?;
        if t804.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t804[0].clone();
        v_v = t804[1].clone();
        if ({ let t783 = v_k.clone(); Py::Bool({ let t784 = v_outputs.clone(); if !py::contains(&t784, &t783)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_78(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t80: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t79 = v_inputs.clone(); f_binding_free_energy(py::getitem(&t79, &Py::str("Kd"))?, py::getitem(&t79, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t80 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t81 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t103 = (t81).items()?;
        if t103.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t103[0].clone();
        v_v = t103[1].clone();
        if ({ let t82 = v_k.clone(); Py::Bool({ let t83 = v_outputs.clone(); if !py::contains(&t83, &t82)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_823(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t825: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t824 = v_inputs.clone(); f_smoluchowski_rate(py::getitem(&t824, &Py::str("DA"))?, py::getitem(&t824, &Py::str("DB"))?, py::getitem(&t824, &Py::str("rA"))?, py::getitem(&t824, &Py::str("rB"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t825 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t826 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t848 = (t826).items()?;
        if t848.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t848[0].clone();
        v_v = t848[1].clone();
        if ({ let t827 = v_k.clone(); Py::Bool({ let t828 = v_outputs.clone(); if !py::contains(&t828, &t827)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_869(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t871: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t870 = v_inputs.clone(); f_diffusion_time(py::getitem(&t870, &Py::str("x"))?, py::getitem(&t870, &Py::str("D"))?, py::getitem(&t870, &Py::str("dims"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t871 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t872 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t894 = (t872).items()?;
        if t894.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t894[0].clone();
        v_v = t894[1].clone();
        if ({ let t873 = v_k.clone(); Py::Bool({ let t874 = v_outputs.clone(); if !py::contains(&t874, &t873)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_919(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t921: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t920 = v_inputs.clone(); f_dna_copy_number(py::getitem(&t920, &Py::str("m"))?, py::getitem(&t920, &Py::str("n"))?, py::getitem(&t920, &Py::str("molecule"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t921 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t922 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t944 = (t922).items()?;
        if t944.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t944[0].clone();
        v_v = t944[1].clone();
        if ({ let t923 = v_k.clone(); Py::Bool({ let t924 = v_outputs.clone(); if !py::contains(&t924, &t923)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_967(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t969: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t968 = v_inputs.clone(); f_donnan_equilibrium(py::getitem(&t968, &Py::str("A"))?, py::getitem(&t968, &Py::str("zA"))?, py::getitem(&t968, &Py::str("c"))?, py::getitem(&t968, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t969 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t970 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t992 = (t970).items()?;
        if t992.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t992[0].clone();
        v_v = t992[1].clone();
        if ({ let t971 = v_k.clone(); Py::Bool({ let t972 = v_outputs.clone(); if !py::contains(&t972, &t971)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
