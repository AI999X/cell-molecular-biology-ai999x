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
    "cmb_atp_yield",
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
    "cmb_decimal_reduction_time",
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
    "cmb_microscope_resolution",
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
    "cmb_reaction_free_energy",
    "cmb_redox_free_energy",
    "cmb_reverse_complement",
    "cmb_rms_displacement",
    "cmb_scatchard_fit",
    "cmb_sedimentation_time",
    "cmb_serial_dilution",
    "cmb_specimen_size",
    "cmb_stokes_einstein",
    "cmb_substrate_for_fraction_of_vmax",
    "cmb_thermal_death_time",
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
    "cmb_water_movement",
    "cmb_water_potential",
    "cmb_z_value",
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
        "cmb_atp_yield" => {
            let r = f_atp_yield(match v.get("P_O_NADH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_O_FADH2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("shuttle") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dG_ATP") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("dG_glucose") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_100(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_binding_free_energy" => {
            let r = f_binding_free_energy(match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_144(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_buffer_after_addition" => {
            let r = f_buffer_after_addition(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pH0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pKa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_add") { Some(x) => py::from_val(x), None => { return None } }, match v.get("added") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_199(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_buffer_capacity" => {
            let r = f_buffer_capacity(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pKa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_244(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_buffer_preparation" => {
            let r = f_buffer_preparation(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("pKa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW_acid") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("MW_base") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_294(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_c1v1_dilution" => {
            let r = f_c1v1_dilution(match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_340(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_cycle_age_corrected" => {
            let r = f_cell_cycle_age_corrected(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fractions") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_396(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_cycle_phase_durations" => {
            let r = f_cell_cycle_simple(match v.get("Tc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fractions") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_449(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_equilibration" => {
            let r = f_cell_equilibration(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_497(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_geometry" => {
            let r = f_cell_geometry(match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_541(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_cell_volume_osmotic" => {
            let r = f_cell_volume_osmotic(match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("osm0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("osm1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_594(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_centrifuge_rcf" => {
            let r = f_centrifuge_rcf(match v.get("rpm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_638(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_centrifuge_rpm" => {
            let r = f_centrifuge_rpm(match v.get("RCF") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_682(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_channel_current" => {
            let r = f_channel_current(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Po") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_739(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_competitive_inhibition" => {
            let r = s_765(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_797(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_competitive_occupancy" => {
            let r = f_competitive_occupancy(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_845(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_decimal_reduction_time" => {
            let r = f_decimal_reduction_time(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_891(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_diffusion_limited_rate" => {
            let r = f_smoluchowski_rate(match v.get("DA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("DB") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rB") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_935(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_diffusion_time" => {
            let r = f_diffusion_time(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dims") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_981(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_dna_copy_number" => {
            let r = f_dna_copy_number(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("molecule") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1031(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_donnan_equilibrium" => {
            let r = f_donnan_equilibrium(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1079(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_doubling_time_from_counts" => {
            let r = f_doubling_from_counts(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1125(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_enzyme_assay_activity" => {
            let r = f_enzyme_assay_activity(match v.get("dA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("l") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_assay") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_enzyme") { Some(x) => py::from_val(x), None => { return None } }, match v.get("protein") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1177(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_exponential_growth" => {
            let r = f_exponential_growth(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("td") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1225(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_expression_steady_state" => {
            let r = f_expression_steady_state(match v.get("k_tx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_tl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta_p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1269(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_fick_flux" => {
            let r = f_fick_flux(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dx") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1317(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_fraction_bound" => {
            let r = f_fraction_bound(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rt") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1364(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_frap_diffusion" => {
            let r = f_frap_diffusion(match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_half") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1443(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_gc_content" => {
            let r = f_gc_content(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1498(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_genome_coverage" => {
            let r = f_genome_coverage(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1549(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_growth_rate_fit" => {
            let r = f_growth_rate_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1613(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_hemocytometer_count" => {
            let r = f_hemocytometer(match v.get("counts") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dilution") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1670(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_henderson_hasselbalch" => {
            let r = f_henderson_hasselbalch(match v.get("pKa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("HA") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1715(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_hill_coefficient_from_ec" => {
            let r = f_hill_from_ec(match v.get("EC10") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EC90") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1761(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_hill_equation" => {
            let r = f_hill_equation(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1805(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_hill_ligand_for_fraction" => {
            let r = f_hill_ligand_for_fraction(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1858(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_ic50_to_ki" => {
            let r = f_ic50_to_ki(match v.get("mode") { Some(x) => py::from_val(x), None => { return None } }, match v.get("IC50") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1908(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_induction_dynamics" => {
            let r = f_induction_dynamics(match v.get("k_tx") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_tl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("delta_p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1958(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_ionic_strength" => {
            let r = f_ionic_strength(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2015(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_kcat_catalytic_efficiency" => {
            let r = f_kcat_efficiency(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Et") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2059(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_kd_from_rates" => {
            let r = f_kd_from_rates(match v.get("kon") { Some(x) => py::from_val(x), None => { return None } }, match v.get("koff") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2103(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_library_size" => {
            let r = f_library_size(match v.get("G") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2158(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_ligand_depletion_binding" => {
            let r = f_ligand_depletion_binding(match v.get("Rt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Lt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2202(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_ligation_insert_mass" => {
            let r = f_ligation_insert_mass(match v.get("m_vector") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L_vector") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L_insert") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_lineweaver_burk_fit" => {
            let r = f_lineweaver_burk(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2323(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_logistic_growth" => {
            let r = f_logistic_growth(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2373(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_logistic_time_to_reach" => {
            let r = f_logistic_time_to_reach(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2420(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mass_for_molarity" => {
            let r = f_mass_for_molarity(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2464(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_membrane_permeability" => {
            let r = f_membrane_permeability(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2512(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_michaelis_menten_nonlinear_fit" => {
            let r = f_mm_nonlinear_fit(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2616(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_michaelis_menten_rate" => {
            let r = f_michaelis_menten(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2664(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_microscope_resolution" => {
            let r = f_microscope_resolution(match v.get("M_obj") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M_oc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wavelength") { Some(x) => py::from_val(x), None => { return None } }, match v.get("NA") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2710(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mixed_inhibition" => {
            let r = f_mixed_inhibition(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kii") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2759(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_molarity_from_mass" => {
            let r = f_molarity_from_mass(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2803(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_molecules_per_cell" => {
            let r = f_molecules_per_cell(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2847(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_monod_batch_time" => {
            let r = f_monod_batch_time(match v.get("X0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ks") { Some(x) => py::from_val(x), None => { return None } }, match v.get("X") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2894(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_monod_growth_rate" => {
            let r = f_monod_growth(match v.get("mu_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ks") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2945(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mrna_half_life_fit" => {
            let r = f_mrna_half_life_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3007(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mutation_rate_p0" => {
            let r = f_mutation_rate_p0(match v.get("n0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Nt") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3059(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_mwc_saturation" => {
            let r = f_mwc_saturation(match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("KR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3107(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_nernst_potential" => {
            let r = f_nernst_potential(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3155(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_noncompetitive_inhibition" => {
            let r = s_3181(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3213(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_nucleic_acid_a260" => {
            let r = f_nucleic_acid_a260(match v.get("A260") { Some(x) => py::from_val(x), None => { return None } }, match v.get("molecule") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dilution") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } }, match v.get("l") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.01)) })().ok()? } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3269(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_oligo_extinction_coefficient" => {
            let r = f_oligo_extinction(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3328(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_oligo_molecular_weight" => {
            let r = f_oligo_molecular_weight(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strands") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3388(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_orf_finder" => {
            let r = f_orf_finder(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("min_aa") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3478(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_osmolarity_tonicity" => {
            let r = f_osmolarity_tonicity(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("osm_cell") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3540(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_osmotic_pressure" => {
            let r = f_osmotic_pressure(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3584(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_pcr_amplification" => {
            let r = f_pcr_amplification(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3641(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_pcr_cycles_needed" => {
            let r = f_pcr_cycles_needed(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3698(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_percent_wv_to_molarity" => {
            let r = f_percent_to_molarity(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3751(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_plate_count" => {
            let r = f_cfu_count(match v.get("colonies") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dilution") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3803(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_protein_concentration_a280" => {
            let r = f_protein_a280(match v.get("A280") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("MW") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("l") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.01)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3854(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_q10_temperature" => {
            let r = f_q10_rate(match v.get("k1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q10") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3900(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_qpcr_delta_delta_ct" => {
            let r = f_qpcr_ddct(match v.get("Ct_target_treated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_ref_treated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_target_control") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_ref_control") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3945(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_qpcr_pfaffl_ratio" => {
            let r = f_qpcr_pfaffl(match v.get("E_target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_target_control") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_target_sample") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_ref_control") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct_ref_sample") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3999(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_qpcr_quantify" => {
            let r = f_qpcr_quantify(match v.get("slope") { Some(x) => py::from_val(x), None => { return None } }, match v.get("intercept") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4043(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_qpcr_standard_curve" => {
            let r = f_qpcr_standard_curve(match v.get("quantity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ct") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4105(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_reaction_free_energy" => {
            let r = f_reaction_free_energy(match v.get("dG0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("products") { Some(x) => py::from_val(x), None => { return None } }, match v.get("reactants") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4177(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_redox_free_energy" => {
            let r = f_redox_free_energy(match v.get("E_acceptor") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_donor") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4231(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_reverse_complement" => {
            let r = f_reverse_complement(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4288(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_rms_displacement" => {
            let r = f_rms_displacement(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dims") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4334(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_scatchard_fit" => {
            let r = f_scatchard_fit(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4400(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_sedimentation_time" => {
            let r = f_sedimentation_time(match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rpm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r_min") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r_max") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4446(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_serial_dilution" => {
            let r = f_serial_dilution(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4501(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_specimen_size" => {
            let r = f_specimen_size(match v.get("image_size") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("bar_image") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("bar_label") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4560(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_stokes_einstein" => {
            let r = f_stokes_einstein(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4604(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_substrate_for_fraction_of_vmax" => {
            let r = f_substrate_for_fraction(match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4657(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_thermal_death_time" => {
            let r = f_thermal_death_time(match v.get("D_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_ref") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("log_reductions") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4703(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_tm_basic" => {
            let r = f_tm_basic(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4759(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_tm_nearest_neighbor" => {
            let r = f_tm_nearest_neighbor(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4836(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_tm_salt_adjusted" => {
            let r = f_tm_salt_adjusted(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4898(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_tm_wallace" => {
            let r = f_tm_wallace(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4955(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_transcribe" => {
            let r = f_transcribe(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } }, match v.get("strand") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5017(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_transcription_time" => {
            let r = f_transcription_time(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_init") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5067(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_transformation_efficiency" => {
            let r = f_transformation_efficiency(match v.get("colonies") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m_DNA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_plated") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_total") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5118(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_translate" => {
            let r = f_translate(match v.get("seq") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5177(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_translation_time" => {
            let r = f_translation_time(match v.get("n_aa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rate") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k_init") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5227(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_uncompetitive_inhibition" => {
            let r = s_5253(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5285(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_warburg_christian" => {
            let r = f_warburg_christian(match v.get("A280") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A260") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5333(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_water_movement" => {
            let r = f_water_movement(match v.get("i_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_cell") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5388(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_water_potential" => {
            let r = f_water_potential(match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("psi_p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5437(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cmb_z_value" => {
            let r = f_z_value(match v.get("D1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5488(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    let t2563 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t2563.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t2563[0].clone();
    v_fb = t2563[1].clone();
    if ({ let t2564 = v_fa.clone(); Py::Bool({ let t2565 = Py::Int(0i64); if py::eq(&t2564, &t2565) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t2566 = v_fb.clone(); Py::Bool({ let t2567 = Py::Int(0i64); if py::eq(&t2566, &t2567) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t2568 = { let t2570 = v_fa.clone(); Py::Bool({ let t2571 = Py::Int(0i64); if py::gt(&t2570, &t2571)? { true } else { false } }) }; Py::Bool({ let t2569 = { let t2572 = v_fb.clone(); Py::Bool({ let t2573 = Py::Int(0i64); if py::gt(&t2572, &t2573)? { true } else { false } }) }; if py::ne(&t2568, &t2569) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t2574 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t2574;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t2579 = { let t2575 = v_m.clone(); Py::Bool({ let t2576 = v_a.clone(); if py::eq(&t2575, &t2576) { true } else { false } }) }; if t2579.truthy() { t2579 } else { { let t2577 = v_m.clone(); Py::Bool({ let t2578 = v_b.clone(); if py::eq(&t2577, &t2578) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t2580 = v_fm.clone(); Py::Bool({ let t2581 = Py::Int(0i64); if py::eq(&t2580, &t2581) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t2582 = { let t2584 = v_fm.clone(); Py::Bool({ let t2585 = Py::Int(0i64); if py::gt(&t2584, &t2585)? { true } else { false } }) }; Py::Bool({ let t2583 = { let t2586 = v_fa.clone(); Py::Bool({ let t2587 = Py::Int(0i64); if py::gt(&t2586, &t2587)? { true } else { false } }) }; if py::eq(&t2582, &t2583) { true } else { false } }) }).truthy() {
            let t2588 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t2588.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2588[0].clone();
            v_fa = t2588[1].clone();
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
    for t5144 in py::iter(&(py::method(&v_text.clone(), "upper", &[])?))? {
        v_ch = t5144;
        if ({ let t5145 = v_ch.clone(); Py::Bool({ let t5146 = Py::str(" "); if py::eq(&t5145, &t5146) { true } else { false } }) }).truthy() {
            continue;
        }
        let _ = f__need({ let t5147 = v_ch.clone(); Py::Bool({ let t5148 = Py::str("ACGTU"); if py::contains(&t5148, &t5147)? { true } else { false } }) }, Py::str("the sequence may contain only A, C, G, T or U"))?;
        v_s = py::iadd(&v_s, &v_ch.clone())?;
    }
    let _ = f__need({ let t5149 = py::len(&v_s.clone())?; Py::Bool({ let t5150 = Py::Int(1i64); if py::ge(&t5149, &t5150)? { true } else { false } }) }, Py::str("the sequence is empty"))?;
    let t5155 = (Py::tuple(vec![{ let t5151 = Py::str("T"); Py::Bool({ let t5152 = v_s.clone(); if py::contains(&t5152, &t5151)? { true } else { false } }) }, { let t5153 = Py::str("U"); Py::Bool({ let t5154 = v_s.clone(); if py::contains(&t5154, &t5153)? { true } else { false } }) }])).items()?;
    if t5155.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_has_t = t5155[0].clone();
    v_has_u = t5155[1].clone();
    let _ = f__need(py::not(&{ let t5156 = v_has_t.clone(); if !t5156.truthy() { t5156 } else { v_has_u.clone() } }), Py::str("a sequence cannot contain both T and U"))?;
    let _ = f__need({ let t5157 = v_rna_ok.clone(); if t5157.truthy() { t5157 } else { py::not(&v_has_u.clone()) } }, Py::str("give a DNA sequence (A, C, G, T)"))?;
    let _ = f__need({ let t5158 = v_dna_ok.clone(); if t5158.truthy() { t5158 } else { py::not(&v_has_t.clone()) } }, Py::str("give an RNA sequence (A, C, G, U)"))?;
    return Ok(Py::tuple(vec![v_s.clone(), v_has_u.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5424 in py::iter(&(v_vals.clone()))? {
        v_v = t5424;
        let _ = f__need(f__isnum(v_v.clone())?, Py::str("every input must be a finite number"))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_name: Py, mut v_v: Py, mut v_lo_open: Py, mut v_hi_open: Py) -> R {
    let _ = f__need(f__isnum(v_v.clone())?, Py::str(&[py::format(&v_name.clone(), "")?, " must be a number".to_string()].concat()))?;
    let _ = f__need({ let t4644 = (if (v_lo_open.clone()).truthy() { { let t4636 = v_v.clone(); Py::Bool({ let t4637 = Py::Int(0i64); if py::gt(&t4636, &t4637)? { true } else { false } }) } } else { { let t4638 = v_v.clone(); Py::Bool({ let t4639 = Py::Int(0i64); if py::ge(&t4638, &t4639)? { true } else { false } }) } }); if !t4644.truthy() { t4644 } else { (if (v_hi_open.clone()).truthy() { { let t4640 = v_v.clone(); Py::Bool({ let t4641 = Py::Int(1i64); if py::lt(&t4640, &t4641)? { true } else { false } }) } } else { { let t4642 = v_v.clone(); Py::Bool({ let t4643 = Py::Int(1i64); if py::le(&t4642, &t4643)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
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
    if ({ let t5264 = v_mode.clone(); Py::Bool({ let t5265 = Py::str("competitive"); if py::eq(&t5264, &t5265) { true } else { false } }) }).truthy() {
        let t5266 = (Py::tuple(vec![py::mul(&v_Km.clone(), &v_a.clone())?, v_Vmax.clone()])).items()?;
        if t5266.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Km_app = t5266[0].clone();
        v_V_app = t5266[1].clone();
    } else {
        if ({ let t5267 = v_mode.clone(); Py::Bool({ let t5268 = Py::str("uncompetitive"); if py::eq(&t5267, &t5268) { true } else { false } }) }).truthy() {
            let t5269 = (Py::tuple(vec![py::truediv(&v_Km.clone(), &v_a.clone())?, py::truediv(&v_Vmax.clone(), &v_a.clone())?])).items()?;
            if t5269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_Km_app = t5269[0].clone();
            v_V_app = t5269[1].clone();
        } else {
            let t5270 = (Py::tuple(vec![v_Km.clone(), py::truediv(&v_Vmax.clone(), &v_a.clone())?])).items()?;
            if t5270.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_Km_app = t5270[0].clone();
            v_V_app = t5270[1].clone();
        }
    }
    v_v = py::truediv(&py::mul(&v_V_app.clone(), &v_S.clone())?, &py::add(&v_Km_app.clone(), &v_S.clone())?)?;
    v_v0 = py::truediv(&py::mul(&v_Vmax.clone(), &v_S.clone())?, &py::add(&v_Km.clone(), &v_S.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v"), v_v.clone()), (Py::str("Km_app"), v_Km_app.clone()), (Py::str("Vmax_app"), v_V_app.clone()), (Py::str("percent_inhibition"), (if ({ let t5283 = v_v0.clone(); Py::Bool({ let t5284 = Py::Int(0i64); if py::gt(&t5283, &t5284)? { true } else { false } }) }).truthy() { py::mul(&Py::Float(100.0), &py::sub(&Py::Float(1.0), &py::truediv(&v_v.clone(), &v_v0.clone())?)?)? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__isnum(mut v_v: Py) -> R {
    return Ok({ let t5465 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5465.truthy() { t5465 } else { { let t5464 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5464.truthy() { t5464 } else { py::math1("isfinite", &v_v.clone())? } } } });
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
    let t4377 = (Py::tuple(vec![f__mean(v_x.clone())?, f__mean(v_y.clone())?])).items()?;
    if t4377.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t4377[0].clone();
    v_my = t4377[1].clone();
    let t4378 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t4378.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_sxx = t4378[0].clone();
    v_sxy = t4378[1].clone();
    v_syy = t4378[2].clone();
    for t4379 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t4379;
        let t4380 = (Py::tuple(vec![py::sub(&py::getitem(&v_x.clone(), &v_i.clone())?, &v_mx.clone())?, py::sub(&py::getitem(&v_y.clone(), &v_i.clone())?, &v_my.clone())?])).items()?;
        if t4380.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_dx = t4380[0].clone();
        v_dy = t4380[1].clone();
        v_sxx = py::iadd(&v_sxx, &py::mul(&v_dx.clone(), &v_dx.clone())?)?;
        v_sxy = py::iadd(&v_sxy, &py::mul(&v_dx.clone(), &v_dy.clone())?)?;
        v_syy = py::iadd(&v_syy, &py::mul(&v_dy.clone(), &v_dy.clone())?)?;
    }
    let _ = f__need({ let t4381 = v_sxx.clone(); Py::Bool({ let t4382 = Py::Int(0i64); if py::gt(&t4381, &t4382)? { true } else { false } }) }, Py::str("the x values must not all be equal"))?;
    v_b = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    v_a = py::sub(&v_my.clone(), &py::mul(&v_b.clone(), &v_mx.clone())?)?;
    v_r2 = (if ({ let t4383 = v_syy.clone(); Py::Bool({ let t4384 = Py::Int(0i64); if py::eq(&t4383, &t4384) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)? });
    return Ok(Py::tuple(vec![v_b.clone(), v_a.clone(), v_r2.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ln_q(mut v_products: Py, mut v_reactants: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__need({ let t4144 = { let t4140 = py::len(&v_products.clone())?; Py::Bool({ let t4141 = Py::Int(1i64); if py::ge(&t4140, &t4141)? { true } else { false } }) }; if !t4144.truthy() { t4144 } else { { let t4142 = py::len(&v_reactants.clone())?; Py::Bool({ let t4143 = Py::Int(1i64); if py::ge(&t4142, &t4143)? { true } else { false } }) } } }, Py::str("give at least one product and one reactant concentration"))?;
    v_s = Py::Float(0.0);
    for t4145 in py::iter(&(v_products.clone()))? {
        v_c = t4145;
        let _ = f__need({ let t4151 = f__isnum(v_c.clone())?; if !t4151.truthy() { t4151 } else { { let t4150 = { let t4146 = v_c.clone(); Py::Bool({ let t4147 = Py::Int(0i64); if py::gt(&t4146, &t4147)? { true } else { false } }) }; if !t4150.truthy() { t4150 } else { { let t4148 = v_c.clone(); Py::Bool({ let t4149 = Py::Float(10000.0); if py::le(&t4148, &t4149)? { true } else { false } }) } } } } }, Py::str("concentrations must be above 0 and at most 10 M (10000 mM)"))?;
        v_s = py::iadd(&v_s, &py::math1("log", &py::mul(&v_c.clone(), &Py::Float(0.001))?)?)?;
    }
    for t4152 in py::iter(&(v_reactants.clone()))? {
        v_c = t4152;
        let _ = f__need({ let t4158 = f__isnum(v_c.clone())?; if !t4158.truthy() { t4158 } else { { let t4157 = { let t4153 = v_c.clone(); Py::Bool({ let t4154 = Py::Int(0i64); if py::gt(&t4153, &t4154)? { true } else { false } }) }; if !t4157.truthy() { t4157 } else { { let t4155 = v_c.clone(); Py::Bool({ let t4156 = Py::Float(10000.0); if py::le(&t4155, &t4156)? { true } else { false } }) } } } } }, Py::str("concentrations must be above 0 and at most 10 M (10000 mM)"))?;
        v_s = py::sub(&v_s, &py::math1("log", &py::mul(&v_c.clone(), &Py::Float(0.001))?)?)?;
    }
    return Ok(v_s.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mean(mut v_xs: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_s = Py::Float(0.0);
    for t4376 in py::iter(&(v_xs.clone()))? {
        v_x = t4376;
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
    let t2557 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0), Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t2557.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2557[0].clone();
    v_b = t2557[1].clone();
    v_da = t2557[2].clone();
    v_db = t2557[3].clone();
    for t2558 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) {
        v_k = t2558;
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
    let t2589 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
    if t2589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2589[0].clone();
    v_b = t2589[1].clone();
    for t2590 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) {
        v_k = t2590;
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
    for t4813 in py::iter(&(v_s.clone()))? {
        v_ch = t4813;
        v_c = py::iadd(&v_c, &py::getitem(&c__DNA_COMP()?, &v_ch.clone())?)?;
    }
    let t4814 = (Py::tuple(vec![py::getitem(&c__NN4_INIT()?, &Py::Int(0i64))?, py::getitem(&c__NN4_INIT()?, &Py::Int(1i64))?])).items()?;
    if t4814.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_dh = t4814[0].clone();
    v_ds = t4814[1].clone();
    v_ends = py::add(&py::getitem(&v_s.clone(), &Py::Int(0i64))?, &py::getitem(&v_s.clone(), &py::neg(&Py::Int(1i64))?)?)?;
    v_at = py::add(&py::method(&v_ends.clone(), "count", &[Py::str("A")])?, &py::method(&v_ends.clone(), "count", &[Py::str("T")])?)?;
    v_dh = py::iadd(&v_dh, &py::mul(&py::getitem(&c__NN4_INIT_AT()?, &Py::Int(0i64))?, &v_at.clone())?)?;
    v_ds = py::iadd(&v_ds, &py::mul(&py::getitem(&c__NN4_INIT_AT()?, &Py::Int(1i64))?, &v_at.clone())?)?;
    for t4815 in py::range_iter(&[(py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t4815;
        v_nb = py::add(&py::add(&py::slice(&v_s.clone(), Some((v_i.clone()).i()?), Some((py::add(&v_i.clone(), &Py::Int(2i64))?).i()?), None)?, &Py::str("/"))?, &py::slice(&v_c.clone(), Some((v_i.clone()).i()?), Some((py::add(&v_i.clone(), &Py::Int(2i64))?).i()?), None)?)?;
        v_rv = py::slice(&v_nb.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?;
        v_v = (if ({ let t4816 = v_nb.clone(); Py::Bool({ let t4817 = c__NN4()?; if py::contains(&t4817, &t4816)? { true } else { false } }) }).truthy() { py::getitem(&c__NN4()?, &v_nb.clone())? } else { py::getitem(&c__NN4()?, &v_rv.clone())? });
        v_dh = py::iadd(&v_dh, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?;
        v_ds = py::iadd(&v_ds, &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?;
    }
    v_selfcomp = { let t4818 = v_s.clone(); Py::Bool({ let t4819 = f__revcomp(v_s.clone(), Py::Bool(false))?; if py::eq(&t4818, &t4819) { true } else { false } }) };
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
    for t5420 in py::iter(&(v_vals.clone()))? {
        v_v = t5420;
        let _ = f__need({ let t5423 = f__isnum(v_v.clone())?; if !t5423.truthy() { t5423 } else { { let t5421 = v_v.clone(); Py::Bool({ let t5422 = Py::Int(0i64); if py::ge(&t5421, &t5422)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t5476 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t5477 = (t5476).items()?;
if t5477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5477[0].clone();
v_v = t5477[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__phase_check(mut v_fractions: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__need({ let t428 = py::len(&v_fractions.clone())?; Py::Bool({ let t429 = Py::Int(4i64); if py::eq(&t428, &t429) { true } else { false } }) }, Py::str("give four fractions: G1, S, G2, M"))?;
    v_s = Py::Float(0.0);
    for t430 in py::iter(&(v_fractions.clone()))? {
        v_f = t430;
        let _ = f__need({ let t433 = f__isnum(v_f.clone())?; if !t433.truthy() { t433 } else { { let t431 = v_f.clone(); Py::Bool({ let t432 = Py::Int(0i64); if py::gt(&t431, &t432)? { true } else { false } }) } } }, Py::str("every phase fraction must be positive"))?;
        v_s = py::iadd(&v_s, &v_f.clone())?;
    }
    let _ = f__need({ let t434 = py::abs(&py::sub(&v_s.clone(), &Py::Float(1.0))?)?; Py::Bool({ let t435 = Py::Float(1e-06); if py::le(&t434, &t435)? { true } else { false } }) }, Py::str("the four fractions must add up to 1"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5463 in py::iter(&(v_vals.clone()))? {
        v_v = t5463;
        let _ = f__need({ let t5468 = f__isnum(v_v.clone())?; if !t5468.truthy() { t5468 } else { { let t5466 = v_v.clone(); Py::Bool({ let t5467 = Py::Int(0i64); if py::gt(&t5466, &t5467)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t5481 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t5481.truthy() { t5481 } else { { let t5480 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t5480.truthy() { t5480 } else { { let t5478 = v_x.clone(); Py::Bool({ let t5479 = Py::None; if py::is(&t5478, &t5479) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t5482 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t5482;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t5485 = { let t5483 = v_x.clone(); Py::Bool({ let t5484 = Py::Int(0i64); if py::eq(&t5483, &t5484) { true } else { false } }) }; if t5485.truthy() { t5485 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t5486 = v_x.clone(); Py::Bool({ let t5487 = Py::Int(0i64); if py::eq(&t5486, &t5487) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t5494 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t5494.truthy() { t5494 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t5498 = py::not(&{ let t5495 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t5495.truthy() { t5495 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t5498.truthy() { t5498 } else { { let t5496 = py::len(&v_a.clone())?; Py::Bool({ let t5497 = py::len(&v_b.clone())?; if py::ne(&t5496, &t5497) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t5499 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5500 = (t5499).items()?;
if t5500.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5500[0].clone();
v_y = t5500[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t5501 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t5501.truthy() { t5501 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t5502 = v_a.clone(); Py::Bool({ let t5503 = v_b.clone(); if py::eq(&t5502, &t5503) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t5505: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t5504 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t5504.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5504[0].clone();
            v_b = t5504[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5505 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t5506 = v_d.clone(); Py::Bool({ let t5507 = Py::Int(0i64); if py::eq(&t5506, &t5507) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t5512 = { let t5508 = py::abs(&v_a.clone())?; Py::Bool({ let t5509 = Py::Float(1e-15); if py::lt(&t5508, &t5509)? { true } else { false } }) }; if !t5512.truthy() { t5512 } else { { let t5510 = py::abs(&v_b.clone())?; Py::Bool({ let t5511 = Py::Float(1e-15); if py::lt(&t5510, &t5511)? { true } else { false } }) } } }).truthy() {
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
    for t5001 in py::range_iter(&[(py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_k = t5001;
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
    let t1422 = (Py::tuple(vec![Py::Float(1.0), py::truediv(&v_x.clone(), &Py::Float(2.0))?])).items()?;
    if t1422.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_term0 = t1422[0].clone();
    v_term1 = t1422[1].clone();
    let t1423 = (Py::tuple(vec![v_term0.clone(), v_term1.clone()])).items()?;
    if t1423.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s0 = t1423[0].clone();
    v_s1 = t1423[1].clone();
    v_k = Py::Int(0i64);
    while (Py::Bool(true)).truthy() {
        v_k = py::iadd(&v_k, &Py::Int(1i64))?;
        v_term0 = py::mul(&v_term0, &py::truediv(&py::pow(&py::truediv(&v_x.clone(), &Py::Float(2.0))?, &Py::Int(2i64))?, &py::mul(&v_k.clone(), &v_k.clone())?)?)?;
        v_term1 = py::mul(&v_term1, &py::truediv(&py::pow(&py::truediv(&v_x.clone(), &Py::Float(2.0))?, &Py::Int(2i64))?, &py::mul(&v_k.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?)?;
        v_s0 = py::iadd(&v_s0, &v_term0.clone())?;
        v_s1 = py::iadd(&v_s1, &v_term1.clone())?;
        if ({ let t1428 = { let t1424 = v_term0.clone(); Py::Bool({ let t1425 = py::mul(&Py::Float(1e-18), &v_s0.clone())?; if py::lt(&t1424, &t1425)? { true } else { false } }) }; if !t1428.truthy() { t1428 } else { { let t1426 = v_term1.clone(); Py::Bool({ let t1427 = py::mul(&Py::Float(1e-18), &v_s1.clone())?; if py::lt(&t1426, &t1427)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t1429 = v_k.clone(); Py::Bool({ let t1430 = Py::Int(500i64); if py::gt(&t1429, &t1430)? { true } else { false } }) }).truthy() {
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
    for t3372 in py::iter(&(v_s.clone()))? {
        v_ch = t3372;
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
fn f_atp_yield(mut v_P_O_NADH: Py, mut v_P_O_FADH2: Py, mut v_shuttle: Py, mut v_dG_ATP: Py, mut v_dG_glucose: Py) -> R {
    let mut v_eff: Py = Py::None;
    let mut v_ferm: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_oxidative: Py = Py::None;
    let mut v_per_cyto: Py = Py::None;
    let mut v_substrate: Py = Py::None;
    let mut v_total: Py = Py::None;
    let _ = f__pos(Py::str("the P/O ratios"), Py::tuple(vec![v_P_O_NADH.clone(), v_P_O_FADH2.clone()]))?;
    let _ = f__need({ let t70 = { let t66 = v_P_O_NADH.clone(); Py::Bool({ let t67 = Py::Float(3.0); if py::le(&t66, &t67)? { true } else { false } }) }; if !t70.truthy() { t70 } else { { let t68 = v_P_O_FADH2.clone(); Py::Bool({ let t69 = Py::Float(3.0); if py::le(&t68, &t69)? { true } else { false } }) } } }, Py::str("P/O ratios must be at most 3"))?;
    let _ = f__need({ let t71 = v_shuttle.clone(); Py::Bool({ let t72 = Py::tuple(vec![Py::str("malate-aspartate"), Py::str("glycerol-phosphate")]); if py::contains(&t72, &t71)? { true } else { false } }) }, Py::str("the shuttle must be malate-aspartate or glycerol-phosphate"))?;
    v_substrate = Py::Float(4.0);
    if ({ let t73 = v_shuttle.clone(); Py::Bool({ let t74 = Py::str("malate-aspartate"); if py::eq(&t73, &t74) { true } else { false } }) }).truthy() {
        v_per_cyto = v_P_O_NADH.clone();
    } else {
        v_per_cyto = v_P_O_FADH2.clone();
    }
    v_oxidative = py::add(&py::add(&py::mul(&Py::Float(8.0), &v_P_O_NADH.clone())?, &py::mul(&Py::Float(2.0), &v_P_O_FADH2.clone())?)?, &py::mul(&Py::Float(2.0), &v_per_cyto.clone())?)?;
    v_total = py::add(&v_substrate.clone(), &v_oxidative.clone())?;
    v_ferm = Py::Float(2.0);
    v_out = Py::dict(vec![(Py::str("ATP_aerobic"), v_total.clone()), (Py::str("ATP_fermentation"), v_ferm.clone()), (Py::str("ratio"), py::truediv(&v_total.clone(), &v_ferm.clone())?), (Py::str("ATP_oxidative"), v_oxidative.clone()), (Py::str("ATP_substrate_level"), v_substrate.clone()), (Py::str("NADH_total"), Py::Float(10.0)), (Py::str("FADH2_total"), Py::Float(2.0))]);
    if ({ let t79 = { let t75 = v_dG_ATP.clone(); Py::Bool({ let t76 = Py::None; if !py::is(&t75, &t76) { true } else { false } }) }; if t79.truthy() { t79 } else { { let t77 = v_dG_glucose.clone(); Py::Bool({ let t78 = Py::None; if !py::is(&t77, &t78) { true } else { false } }) } } }).truthy() {
        let _ = f__need({ let t84 = { let t80 = v_dG_ATP.clone(); Py::Bool({ let t81 = Py::None; if !py::is(&t80, &t81) { true } else { false } }) }; if !t84.truthy() { t84 } else { { let t82 = v_dG_glucose.clone(); Py::Bool({ let t83 = Py::None; if !py::is(&t82, &t83) { true } else { false } }) } } }, Py::str("give both dG_ATP and dG_glucose for the efficiencies"))?;
        let _ = f__pos(Py::str("the energies dG_ATP and dG_glucose (as positive magnitudes)"), Py::tuple(vec![v_dG_ATP.clone(), v_dG_glucose.clone()]))?;
        v_eff = py::truediv(&py::mul(&v_total.clone(), &v_dG_ATP.clone())?, &v_dG_glucose.clone())?;
        let _ = f__need({ let t85 = v_eff.clone(); Py::Bool({ let t86 = Py::Float(1.0); if py::le(&t85, &t86)? { true } else { false } }) }, Py::str("the ATP captured cannot hold more energy than the oxidation of glucose releases"))?;
        py::setitem(&v_out.clone(), &Py::str("efficiency_aerobic"), v_eff.clone())?;
        py::setitem(&v_out.clone(), &Py::str("efficiency_fermentation"), py::truediv(&py::mul(&v_ferm.clone(), &v_dG_ATP.clone())?, &v_dG_glucose.clone())?)?;
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
    let _ = f__need({ let t177 = v_added.clone(); Py::Bool({ let t178 = Py::tuple(vec![Py::str("acid"), Py::str("base")]); if py::contains(&t178, &t177)? { true } else { false } }) }, Py::str("added: acid or base (strong)"))?;
    v_r = py::pow(&Py::Float(10.0), &py::sub(&v_pH0.clone(), &v_pKa.clone())?)?;
    v_nb = py::truediv(&py::mul(&py::mul(&v_C.clone(), &v_V.clone())?, &v_r.clone())?, &py::add(&Py::Float(1.0), &v_r.clone())?)?;
    v_na = py::sub(&py::mul(&v_C.clone(), &v_V.clone())?, &v_nb.clone())?;
    if ({ let t179 = v_added.clone(); Py::Bool({ let t180 = Py::str("base"); if py::eq(&t179, &t180) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t181 = v_n_add.clone(); Py::Bool({ let t182 = v_na.clone(); if py::lt(&t181, &t182)? { true } else { false } }) }, Py::str("the base added exceeds the acid form: the buffer is exhausted"))?;
        let t183 = (Py::tuple(vec![py::add(&v_nb.clone(), &v_n_add.clone())?, py::sub(&v_na.clone(), &v_n_add.clone())?])).items()?;
        if t183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nb2 = t183[0].clone();
        v_na2 = t183[1].clone();
    } else {
        let _ = f__need({ let t184 = v_n_add.clone(); Py::Bool({ let t185 = v_nb.clone(); if py::lt(&t184, &t185)? { true } else { false } }) }, Py::str("the acid added exceeds the base form: the buffer is exhausted"))?;
        let t186 = (Py::tuple(vec![py::sub(&v_nb.clone(), &v_n_add.clone())?, py::add(&v_na.clone(), &v_n_add.clone())?])).items()?;
        if t186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nb2 = t186[0].clone();
        v_na2 = t186[1].clone();
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
    if ({ let t277 = v_MW_acid.clone(); Py::Bool({ let t278 = Py::None; if !py::is(&t277, &t278) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the acid molar mass"), Py::tuple(vec![v_MW_acid.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("m_acid_g"), py::mul(&py::mul(&py::mul(&v_ca.clone(), &v_V.clone())?, &v_MW_acid.clone())?, &Py::Float(1000.0))?)?;
    }
    if ({ let t279 = v_MW_base.clone(); Py::Bool({ let t280 = Py::None; if !py::is(&t279, &t280) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t326 = v_C2.clone(); Py::Bool({ let t327 = v_C1.clone(); if py::le(&t326, &t327)? { true } else { false } }) }, Py::str("the final concentration cannot exceed the stock concentration"))?;
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
    let t380 = (Py::tuple(vec![Py::list(vec![]), Py::Float(0.0)])).items()?;
    if t380.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ends = t380[0].clone();
    v_F = t380[1].clone();
    for t381 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
        v_k = t381;
        v_F = py::iadd(&v_F, &py::getitem(&v_fractions.clone(), &v_k.clone())?)?;
        if ({ let t382 = v_k.clone(); Py::Bool({ let t383 = Py::Int(3i64); if py::eq(&t382, &t383) { true } else { false } }) }).truthy() {
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
    v_d = { let mut out = vec![]; for t436 in py::iter(&(v_fractions.clone()))? { let mut v_f: Py; 
v_f = t436;
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
    let _ = f__need({ let t3779 = Py::Bool(matches!(v_colonies.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3779.truthy() { t3779 } else { { let t3777 = v_colonies.clone(); Py::Bool({ let t3778 = Py::Int(0i64); if py::ge(&t3777, &t3778)? { true } else { false } }) } } }, Py::str("the colony count must be a whole number"))?;
    let _ = f__need({ let t3784 = f__isnum(v_dilution.clone())?; if !t3784.truthy() { t3784 } else { { let t3782 = v_dilution.clone(); Py::Bool({ let t3783 = Py::Int(1i64); if py::ge(&t3782, &t3783)? { true } else { false } }) } } }, Py::str("the dilution factor must be at least 1 (e.g. 1e6 for a 10^-6 dilution)"))?;
    let _ = f__pos(Py::str("the plated volume V"), Py::tuple(vec![v_V.clone()]))?;
    v_c = py::truediv(&py::mul(&v_colonies.clone(), &v_dilution.clone())?, &py::mul(&v_V.clone(), &Py::Float(1000000.0))?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("cfu_per_mL"), v_c.clone()), (Py::str("log10_cfu_per_mL"), (if ({ let t3801 = v_c.clone(); Py::Bool({ let t3802 = Py::Int(0i64); if py::gt(&t3801, &t3802)? { true } else { false } }) }).truthy() { py::math1("log10", &v_c.clone())? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_channel_current(mut v_N: Py, mut v_Po: Py, mut v_gamma: Py, mut v_V: Py, mut v_E: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_I: Py = Py::None;
    let _ = f__need({ let t710 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t710.truthy() { t710 } else { { let t708 = v_N.clone(); Py::Bool({ let t709 = Py::Int(1i64); if py::ge(&t708, &t709)? { true } else { false } }) } } }, Py::str("N must be a whole number of channels, at least 1"))?;
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
fn f_decimal_reduction_time(mut v_N0: Py, mut v_N: Py, mut v_t: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lr: Py = Py::None;
    let _ = f__pos(Py::str("the counts N0 and N and the time t"), Py::tuple(vec![v_N0.clone(), v_N.clone(), v_t.clone()]))?;
    let _ = f__need({ let t877 = v_N.clone(); Py::Bool({ let t878 = v_N0.clone(); if py::lt(&t877, &t878)? { true } else { false } }) }, Py::str("the surviving count N must be below the initial count N0"))?;
    v_lr = py::math1("log10", &py::truediv(&v_N0.clone(), &v_N.clone())?)?;
    v_D = py::truediv(&v_t.clone(), &v_lr.clone())?;
    v_k = py::truediv(&py::math1("log", &Py::Float(10.0))?, &v_D.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("log_reduction"), v_lr.clone()), (Py::str("D"), v_D.clone()), (Py::str("D_min"), py::truediv(&v_D.clone(), &c_MINUTE()?)?), (Py::str("k_per_min"), py::mul(&v_k.clone(), &c_MINUTE()?)?), (Py::str("survival_fraction"), py::truediv(&v_N.clone(), &v_N0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_diffusion_time(mut v_x: Py, mut v_D: Py, mut v_dims: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("x and D"), Py::tuple(vec![v_x.clone(), v_D.clone()]))?;
    let _ = f__need({ let t967 = v_dims.clone(); Py::Bool({ let t968 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)]); if py::contains(&t968, &t967)? { true } else { false } }) }, Py::str("dims must be 1, 2 or 3"))?;
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
    let _ = f__need({ let t1015 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1015.truthy() { t1015 } else { { let t1013 = v_n.clone(); Py::Bool({ let t1014 = Py::Int(1i64); if py::ge(&t1013, &t1014)? { true } else { false } }) } } }, Py::str("the length n must be a whole number of bases or base pairs"))?;
    let _ = f__need({ let t1016 = v_molecule.clone(); Py::Bool({ let t1017 = c__COPY_MW()?; if py::contains(&t1017, &t1016)? { true } else { false } }) }, Py::str("molecule: dsDNA, ssDNA or RNA"))?;
    let t1018 = (py::getitem(&c__COPY_MW()?, &v_molecule.clone())?).items()?;
    if t1018.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1018[0].clone();
    v_b = t1018[1].clone();
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
    let _ = f__need({ let t1066 = Py::Bool(matches!(v_zA.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1066.truthy() { t1066 } else { { let t1065 = py::not(&Py::Bool(matches!(v_zA.clone(), Py::Bool(_)))); if !t1065.truthy() { t1065 } else { { let t1063 = v_zA.clone(); Py::Bool({ let t1064 = Py::Int(1i64); if py::ge(&t1063, &t1064)? { true } else { false } }) } } } } }, Py::str("zA is the magnitude of the anion's charge (1, 2, 3)"))?;
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
    let _ = f__need({ let t1111 = v_N.clone(); Py::Bool({ let t1112 = v_N0.clone(); if py::gt(&t1111, &t1112)? { true } else { false } }) }, Py::str("the final count must exceed the initial count"))?;
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
    let _ = f__need({ let t1158 = v_dA.clone(); Py::Bool({ let t1159 = Py::Int(0i64); if py::ne(&t1158, &t1159) { true } else { false } }) }, Py::str("the absorbance change must not be zero"))?;
    let _ = f__need({ let t1160 = v_V_enzyme.clone(); Py::Bool({ let t1161 = v_V_assay.clone(); if py::le(&t1160, &t1161)? { true } else { false } }) }, Py::str("the enzyme volume cannot exceed the assay volume"))?;
    v_rate = py::truediv(&py::truediv(&py::abs(&v_dA.clone())?, &v_dt.clone())?, &py::mul(&v_eps.clone(), &v_l.clone())?)?;
    v_umol_min = py::mul(&py::mul(&py::mul(&v_rate.clone(), &v_V_assay.clone())?, &Py::Float(1000000.0))?, &c_MINUTE()?)?;
    v_u_per_ml = py::truediv(&v_umol_min.clone(), &py::mul(&v_V_enzyme.clone(), &Py::Float(1000000.0))?)?;
    v_out = Py::dict(vec![(Py::str("rate"), v_rate.clone()), (Py::str("units"), v_umol_min.clone()), (Py::str("units_per_mL"), v_u_per_ml.clone())]);
    if ({ let t1162 = v_protein.clone(); Py::Bool({ let t1163 = Py::None; if !py::is(&t1162, &t1163) { true } else { false } }) }).truthy() {
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
    if ({ let t1349 = v_Rt.clone(); Py::Bool({ let t1350 = Py::None; if !py::is(&t1349, &t1350) { true } else { false } }) }).truthy() {
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
    let t1484 = (f__clean_seq(v_seq.clone(), Py::Bool(true), Py::Bool(true))?).items()?;
    if t1484.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1484[0].clone();
    v_rna = t1484[1].clone();
    let t1485 = (Py::tuple(vec![py::method(&v_s.clone(), "count", &[Py::str("G")])?, py::method(&v_s.clone(), "count", &[Py::str("C")])?])).items()?;
    if t1485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1485[0].clone();
    v_c = t1485[1].clone();
    v_n = py::len(&v_s.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("gc_fraction"), py::truediv(&py::add(&v_g.clone(), &v_c.clone())?, &v_n.clone())?), (Py::str("gc_percent"), py::truediv(&py::mul(&Py::Float(100.0), &py::add(&v_g.clone(), &v_c.clone())?)?, &v_n.clone())?), (Py::str("length"), v_n.clone()), (Py::str("G"), v_g.clone()), (Py::str("C"), v_c.clone()), (Py::str("A"), py::method(&v_s.clone(), "count", &[Py::str("A")])?), (Py::str("T_or_U"), (if (v_rna.clone()).truthy() { py::method(&v_s.clone(), "count", &[Py::str("U")])? } else { py::method(&v_s.clone(), "count", &[Py::str("T")])? }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_genome_coverage(mut v_L: Py, mut v_N: Py, mut v_G: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let _ = f__need({ let t1530 = Py::Bool(matches!(v_L.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1530.truthy() { t1530 } else { { let t1529 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1529.truthy() { t1529 } else { { let t1528 = { let t1524 = v_L.clone(); Py::Bool({ let t1525 = Py::Int(1i64); if py::ge(&t1524, &t1525)? { true } else { false } }) }; if !t1528.truthy() { t1528 } else { { let t1526 = v_N.clone(); Py::Bool({ let t1527 = Py::Int(1i64); if py::ge(&t1526, &t1527)? { true } else { false } }) } } } } } } }, Py::str("read length and read count are whole numbers"))?;
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
    let _ = f__need({ let t1579 = { let t1575 = py::len(&v_t.clone())?; Py::Bool({ let t1576 = py::len(&v_N.clone())?; if py::eq(&t1575, &t1576) { true } else { false } }) }; if !t1579.truthy() { t1579 } else { { let t1577 = py::len(&v_t.clone())?; Py::Bool({ let t1578 = Py::Int(3i64); if py::ge(&t1577, &t1578)? { true } else { false } }) } } }, Py::str("give at least 3 times and one count per time"))?;
    for t1580 in py::iter(&(v_N.clone()))? {
        v_v = t1580;
        let _ = f__need({ let t1585 = f__isnum(v_v.clone())?; if !t1585.truthy() { t1585 } else { { let t1583 = v_v.clone(); Py::Bool({ let t1584 = Py::Int(0i64); if py::gt(&t1583, &t1584)? { true } else { false } }) } } }, Py::str("counts must be positive"))?;
    }
    for t1586 in py::iter(&(v_t.clone()))? {
        v_v = t1586;
        let _ = f__fin(Py::tuple(vec![v_v.clone()]))?;
    }
    v_y = { let mut out = vec![]; for t1588 in py::iter(&(v_N.clone()))? { let mut v_v: Py; 
v_v = t1588;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) };
    let t1598 = (f__linfit(v_t.clone(), v_y.clone())?).items()?;
    if t1598.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1598[0].clone();
    v_a = t1598[1].clone();
    v_r2 = t1598[2].clone();
    let _ = f__need({ let t1599 = v_b.clone(); Py::Bool({ let t1600 = Py::Int(0i64); if py::gt(&t1599, &t1600)? { true } else { false } }) }, Py::str("the counts do not increase: no growth rate can be fitted"))?;
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
    let _ = f__need({ let t1639 = py::len(&v_counts.clone())?; Py::Bool({ let t1640 = Py::Int(1i64); if py::ge(&t1639, &t1640)? { true } else { false } }) }, Py::str("give the counts of at least one square"))?;
    for t1641 in py::iter(&(v_counts.clone()))? {
        v_v = t1641;
        let _ = f__need({ let t1646 = f__isnum(v_v.clone())?; if !t1646.truthy() { t1646 } else { { let t1644 = v_v.clone(); Py::Bool({ let t1645 = Py::Int(0i64); if py::ge(&t1644, &t1645)? { true } else { false } }) } } }, Py::str("counts must be zero or positive"))?;
    }
    let _ = f__need({ let t1649 = f__isnum(v_dilution.clone())?; if !t1649.truthy() { t1649 } else { { let t1647 = v_dilution.clone(); Py::Bool({ let t1648 = Py::Int(1i64); if py::ge(&t1647, &t1648)? { true } else { false } }) } } }, Py::str("the dilution factor must be at least 1"))?;
    v_m = f__mean(v_counts.clone())?;
    v_conc = py::mul(&py::mul(&v_m.clone(), &v_dilution.clone())?, &Py::Float(10000.0))?;
    v_out = Py::dict(vec![(Py::str("mean_count"), v_m.clone()), (Py::str("cells_per_mL"), v_conc.clone())]);
    if ({ let t1651 = v_V.clone(); Py::Bool({ let t1652 = Py::None; if !py::is(&t1651, &t1652) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t1747 = v_EC90.clone(); Py::Bool({ let t1748 = v_EC10.clone(); if py::gt(&t1747, &t1748)? { true } else { false } }) }, Py::str("EC90 must exceed EC10"))?;
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
    let _ = f__need({ let t1884 = v_mode.clone(); Py::Bool({ let t1885 = c__INHIB()?; if py::contains(&t1885, &t1884)? { true } else { false } }) }, Py::str("mode: competitive, uncompetitive or noncompetitive"))?;
    let _ = f__pos(Py::str("IC50, S and Km"), Py::tuple(vec![v_IC50.clone(), v_S.clone(), v_Km.clone()]))?;
    if ({ let t1892 = v_mode.clone(); Py::Bool({ let t1893 = Py::str("competitive"); if py::eq(&t1892, &t1893) { true } else { false } }) }).truthy() {
        v_Ki = py::truediv(&v_IC50.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_S.clone(), &v_Km.clone())?)?)?;
    } else {
        if ({ let t1894 = v_mode.clone(); Py::Bool({ let t1895 = Py::str("uncompetitive"); if py::eq(&t1894, &t1895) { true } else { false } }) }).truthy() {
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
    if ({ let t1944 = py::abs(&v_d.clone())?; Py::Bool({ let t1945 = py::mul(&Py::Float(1e-09), &py::minmax(&[v_delta_p.clone(), v_delta_m.clone()], true)?)?; if py::le(&t1944, &t1945)? { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t1988 = { let t1984 = py::len(&v_c.clone())?; Py::Bool({ let t1985 = py::len(&v_z.clone())?; if py::eq(&t1984, &t1985) { true } else { false } }) }; if !t1988.truthy() { t1988 } else { { let t1986 = py::len(&v_c.clone())?; Py::Bool({ let t1987 = Py::Int(1i64); if py::ge(&t1986, &t1987)? { true } else { false } }) } } }, Py::str("give one charge per concentration"))?;
    for t1989 in py::iter(&(v_c.clone()))? {
        v_v = t1989;
        let _ = f__need({ let t1994 = f__isnum(v_v.clone())?; if !t1994.truthy() { t1994 } else { { let t1992 = v_v.clone(); Py::Bool({ let t1993 = Py::Int(0i64); if py::ge(&t1992, &t1993)? { true } else { false } }) } } }, Py::str("concentrations must be zero or positive"))?;
    }
    for t1995 in py::iter(&(v_z.clone()))? {
        v_v = t1995;
        let _ = f__need({ let t2001 = f__isnum(v_v.clone())?; if !t2001.truthy() { t2001 } else { { let t2000 = { let t1996 = v_v.clone(); Py::Bool({ let t1997 = py::int(&v_v.clone())?; if py::eq(&t1996, &t1997) { true } else { false } }) }; if !t2000.truthy() { t2000 } else { { let t1998 = v_v.clone(); Py::Bool({ let t1999 = Py::Int(0i64); if py::ne(&t1998, &t1999) { true } else { false } }) } } } } }, Py::str("charges must be non-zero whole numbers"))?;
    }
    v_s = Py::Float(0.0);
    for t2002 in py::range_iter(&[(py::len(&v_c.clone())?).i()?]) {
        v_k = t2002;
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
    return Ok(f__out(Py::dict(vec![(Py::str("Kd"), v_Kd.clone()), (Py::str("Kd_nM"), py::mul(&v_Kd.clone(), &Py::Float(1000000.0))?), (Py::str("Ka_per_M"), py::mul(&py::truediv(&v_kon.clone(), &v_koff.clone())?, &Py::Float(1000.0))?), (Py::str("half_life"), py::truediv(&c_LN2()?, &v_koff.clone())?), (Py::str("residence_time"), py::truediv(&Py::Float(1.0), &v_koff.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_library_size(mut v_G: Py, mut v_I: Py, mut v_P: Py) -> R {
    let mut v_clones: Py = Py::None;
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("the genome size and the insert size"), Py::tuple(vec![v_G.clone(), v_I.clone()]))?;
    let _ = f__need({ let t2135 = v_I.clone(); Py::Bool({ let t2136 = v_G.clone(); if py::lt(&t2135, &t2136)? { true } else { false } }) }, Py::str("the insert must be smaller than the genome"))?;
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
    let _ = f__need({ let t2240 = Py::Bool(matches!(v_L_vector.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2240.truthy() { t2240 } else { { let t2239 = Py::Bool(matches!(v_L_insert.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2239.truthy() { t2239 } else { { let t2238 = { let t2234 = v_L_vector.clone(); Py::Bool({ let t2235 = Py::Int(1i64); if py::ge(&t2234, &t2235)? { true } else { false } }) }; if !t2238.truthy() { t2238 } else { { let t2236 = v_L_insert.clone(); Py::Bool({ let t2237 = Py::Int(1i64); if py::ge(&t2236, &t2237)? { true } else { false } }) } } } } } } }, Py::str("lengths are whole numbers of bp"))?;
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
    let _ = f__need({ let t2283 = { let t2279 = py::len(&v_S.clone())?; Py::Bool({ let t2280 = py::len(&v_v.clone())?; if py::eq(&t2279, &t2280) { true } else { false } }) }; if !t2283.truthy() { t2283 } else { { let t2281 = py::len(&v_S.clone())?; Py::Bool({ let t2282 = Py::Int(3i64); if py::ge(&t2281, &t2282)? { true } else { false } }) } } }, Py::str("give at least 3 substrate concentrations with one rate each"))?;
    for t2284 in py::iter(&(v_S.clone()))? {
        v_a = t2284;
        let _ = f__need({ let t2289 = f__isnum(v_a.clone())?; if !t2289.truthy() { t2289 } else { { let t2287 = v_a.clone(); Py::Bool({ let t2288 = Py::Int(0i64); if py::gt(&t2287, &t2288)? { true } else { false } }) } } }, Py::str("substrate concentrations must be positive"))?;
    }
    for t2290 in py::iter(&(v_v.clone()))? {
        v_a = t2290;
        let _ = f__need({ let t2293 = f__isnum(v_a.clone())?; if !t2293.truthy() { t2293 } else { { let t2291 = v_a.clone(); Py::Bool({ let t2292 = Py::Int(0i64); if py::gt(&t2291, &t2292)? { true } else { false } }) } } }, Py::str("rates must be positive"))?;
    }
    v_x = { let mut out = vec![]; for t2294 in py::iter(&(v_S.clone()))? { let mut v_a: Py; 
v_a = t2294;
out.push(py::truediv(&Py::Float(1.0), &v_a.clone())?); } Py::list(out) };
    v_y = { let mut out = vec![]; for t2295 in py::iter(&(v_v.clone()))? { let mut v_a: Py; 
v_a = t2295;
out.push(py::truediv(&Py::Float(1.0), &v_a.clone())?); } Py::list(out) };
    let t2305 = (f__linfit(v_x.clone(), v_y.clone())?).items()?;
    if t2305.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2305[0].clone();
    v_b = t2305[1].clone();
    v_r2 = t2305[2].clone();
    let _ = f__need({ let t2310 = { let t2306 = v_b.clone(); Py::Bool({ let t2307 = Py::Int(0i64); if py::gt(&t2306, &t2307)? { true } else { false } }) }; if !t2310.truthy() { t2310 } else { { let t2308 = v_m.clone(); Py::Bool({ let t2309 = Py::Int(0i64); if py::gt(&t2308, &t2309)? { true } else { false } }) } } }, Py::str("the double-reciprocal line must have a positive slope and intercept"))?;
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
    let _ = f__need({ let t2359 = v_N0.clone(); Py::Bool({ let t2360 = v_K.clone(); if py::lt(&t2359, &t2360)? { true } else { false } }) }, Py::str("the initial population must be below the carrying capacity"))?;
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
    let _ = f__need({ let t2405 = v_N0.clone(); Py::Bool({ let t2406 = v_N.clone(); if py::lt(&t2405, &t2406)? { { let t2407 = v_K.clone(); if py::lt(&t2406, &t2407)? { true } else { false } } } else { false } }) }, Py::str("the target must lie between the initial population and the carrying capacity"))?;
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
fn f_microscope_resolution(mut v_M_obj: Py, mut v_M_oc: Py, mut v_wavelength: Py, mut v_NA: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_d: Py = Py::None;
    let _ = f__pos(Py::str("the magnifications, the wavelength and the numerical aperture"), Py::tuple(vec![v_M_obj.clone(), v_M_oc.clone(), v_wavelength.clone(), v_NA.clone()]))?;
    let _ = f__need({ let t2696 = v_NA.clone(); Py::Bool({ let t2697 = Py::Float(1.6); if py::le(&t2696, &t2697)? { true } else { false } }) }, Py::str("the numerical aperture of a light microscope cannot exceed about 1.6 (oil immersion is about 1.4)"))?;
    v_M = py::mul(&v_M_obj.clone(), &v_M_oc.clone())?;
    v_d = py::truediv(&v_wavelength.clone(), &py::mul(&Py::Float(2.0), &v_NA.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("M"), v_M.clone()), (Py::str("d"), v_d.clone()), (Py::str("d_nm"), py::mul(&v_d.clone(), &Py::Float(1000000000.0))?), (Py::str("d_um"), py::mul(&v_d.clone(), &Py::Float(1000000.0))?)]))?);
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
    let t2746 = (Py::tuple(vec![py::add(&Py::Float(1.0), &py::truediv(&v_I.clone(), &v_Ki.clone())?)?, py::add(&Py::Float(1.0), &py::truediv(&v_I.clone(), &v_Kii.clone())?)?])).items()?;
    if t2746.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2746[0].clone();
    v_ap = t2746[1].clone();
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
    let _ = f__need({ let t2542 = { let t2538 = py::len(&v_S.clone())?; Py::Bool({ let t2539 = py::len(&v_v.clone())?; if py::eq(&t2538, &t2539) { true } else { false } }) }; if !t2542.truthy() { t2542 } else { { let t2540 = py::len(&v_S.clone())?; Py::Bool({ let t2541 = Py::Int(3i64); if py::ge(&t2540, &t2541)? { true } else { false } }) } } }, Py::str("give at least 3 substrate concentrations with one rate each"))?;
    for t2543 in py::iter(&(v_S.clone()))? {
        v_a = t2543;
        let _ = f__need({ let t2548 = f__isnum(v_a.clone())?; if !t2548.truthy() { t2548 } else { { let t2546 = v_a.clone(); Py::Bool({ let t2547 = Py::Int(0i64); if py::gt(&t2546, &t2547)? { true } else { false } }) } } }, Py::str("substrate concentrations must be positive"))?;
    }
    for t2549 in py::iter(&(v_v.clone()))? {
        v_a = t2549;
        let _ = f__need({ let t2552 = f__isnum(v_a.clone())?; if !t2552.truthy() { t2552 } else { { let t2550 = v_a.clone(); Py::Bool({ let t2551 = Py::Int(0i64); if py::gt(&t2550, &t2551)? { true } else { false } }) } } }, Py::str("rates must be positive"))?;
    }
    let t2553 = (Py::tuple(vec![py::mul(&py::minmax(&[v_S.clone()], false)?, &Py::Float(0.0001))?, py::mul(&py::minmax(&[v_S.clone()], true)?, &Py::Float(10000.0))?])).items()?;
    if t2553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t2553[0].clone();
    v_hi = t2553[1].clone();
    v_grid = { let mut out = vec![]; for t2554 in py::range_iter(&[(Py::Int(401i64)).i()?]) { let mut v_k: Py; 
v_k = t2554;
out.push(py::mul(&v_lo.clone(), &py::pow(&py::truediv(&v_hi.clone(), &v_lo.clone())?, &py::truediv(&v_k.clone(), &Py::Float(400.0))?)?)?); } Py::list(out) };
    let t2555 = (Py::tuple(vec![Py::None, Py::None])).items()?;
    if t2555.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_best = t2555[0].clone();
    v_fbest = t2555[1].clone();
    for t2556 in py::range_iter(&[(Py::Int(400i64)).i()?]) {
        v_k = t2556;
        let t2559 = (Py::tuple(vec![f__mm_dprofile(v_S.clone(), v_v.clone(), py::getitem(&v_grid.clone(), &v_k.clone())?)?, f__mm_dprofile(v_S.clone(), v_v.clone(), py::getitem(&v_grid.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?])).items()?;
        if t2559.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_f1 = t2559[0].clone();
        v_f2 = t2559[1].clone();
        if ({ let t2560 = v_f1.clone(); Py::Bool({ let t2561 = Py::Int(0i64); if py::lt(&t2560, &t2561)? { { let t2562 = v_f2.clone(); if py::le(&t2561, &t2562)? { true } else { false } } } else { false } }) }).truthy() {
            v_r = f__bisect({ let v_S = v_S.clone(); let v_a = v_a.clone(); let v_best = v_best.clone(); let v_f1 = v_f1.clone(); let v_f2 = v_f2.clone(); let v_fbest = v_fbest.clone(); let v_grid = v_grid.clone(); let v_hi = v_hi.clone(); let v_k = v_k.clone(); let v_lo = v_lo.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__mm_dprofile(v_S.clone(), v_v.clone(), v_x.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::getitem(&v_grid.clone(), &v_k.clone())?, py::getitem(&v_grid.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, Py::Int(400i64))?;
            let t2591 = (f__mm_profile(v_S.clone(), v_v.clone(), v_r.clone())?).items()?;
            if t2591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t2591[0].clone();
            v_b = t2591[1].clone();
            v_obj = py::truediv(&py::mul(&py::neg(&v_a.clone())?, &v_a.clone())?, &v_b.clone())?;
            if ({ let t2596 = { let t2592 = v_best.clone(); Py::Bool({ let t2593 = Py::None; if py::is(&t2592, &t2593) { true } else { false } }) }; if t2596.truthy() { t2596 } else { { let t2594 = v_obj.clone(); Py::Bool({ let t2595 = v_fbest.clone(); if py::lt(&t2594, &t2595)? { true } else { false } }) } } }).truthy() {
                let t2597 = (Py::tuple(vec![v_r.clone(), v_obj.clone()])).items()?;
                if t2597.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_best = t2597[0].clone();
                v_fbest = t2597[1].clone();
            }
        }
    }
    let _ = f__need({ let t2598 = v_best.clone(); Py::Bool({ let t2599 = Py::None; if !py::is(&t2598, &t2599) { true } else { false } }) }, Py::str("the data do not define a Michaelis-Menten curve (no interior least-squares minimum)"))?;
    v_Km = v_best.clone();
    let t2600 = (f__mm_profile(v_S.clone(), v_v.clone(), v_Km.clone())?).items()?;
    if t2600.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2600[0].clone();
    v_b = t2600[1].clone();
    v_Vmax = py::truediv(&v_a.clone(), &v_b.clone())?;
    let t2602 = (Py::tuple(vec![Py::Float(0.0), f__mean(v_v.clone())?])).items()?;
    if t2602.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rss = t2602[0].clone();
    v_mv = t2602[1].clone();
    v_sst = Py::Float(0.0);
    for t2603 in py::range_iter(&[(py::len(&v_S.clone())?).i()?]) {
        v_k = t2603;
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
    let _ = f__need({ let t2879 = v_X0.clone(); Py::Bool({ let t2880 = v_X.clone(); if py::lt(&t2879, &t2880)? { { let t2881 = v_B.clone(); if py::lt(&t2880, &t2881)? { true } else { false } } } else { false } }) }, Py::str("the target biomass must lie between X0 and X0 + Y S0"))?;
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
    py::setitem(&v_out.clone(), &Py::str("td_h"), (if ({ let t2930 = v_mu.clone(); Py::Bool({ let t2931 = Py::Int(0i64); if py::gt(&t2930, &t2931)? { true } else { false } }) }).truthy() { py::truediv(&py::truediv(&c_LN2()?, &v_mu.clone())?, &c_HOUR()?)? } else { Py::Float(0.0) }))?;
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
    let _ = f__need({ let t2975 = { let t2971 = py::len(&v_t.clone())?; Py::Bool({ let t2972 = py::len(&v_level.clone())?; if py::eq(&t2971, &t2972) { true } else { false } }) }; if !t2975.truthy() { t2975 } else { { let t2973 = py::len(&v_t.clone())?; Py::Bool({ let t2974 = Py::Int(3i64); if py::ge(&t2973, &t2974)? { true } else { false } }) } } }, Py::str("give at least 3 times and one level per time"))?;
    for t2976 in py::iter(&(v_level.clone()))? {
        v_v = t2976;
        let _ = f__need({ let t2981 = f__isnum(v_v.clone())?; if !t2981.truthy() { t2981 } else { { let t2979 = v_v.clone(); Py::Bool({ let t2980 = Py::Int(0i64); if py::gt(&t2979, &t2980)? { true } else { false } }) } } }, Py::str("levels must be positive"))?;
    }
    let t2992 = (f__linfit(v_t.clone(), { let mut out = vec![]; for t2991 in py::iter(&(v_level.clone()))? { let mut v_v: Py; 
v_v = t2991;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) })?).items()?;
    if t2992.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t2992[0].clone();
    v_a = t2992[1].clone();
    v_r2 = t2992[2].clone();
    let _ = f__need({ let t2993 = v_b.clone(); Py::Bool({ let t2994 = Py::Int(0i64); if py::lt(&t2993, &t2994)? { true } else { false } }) }, Py::str("the levels do not decay: no half-life can be fitted"))?;
    v_k = py::neg(&v_b.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("k_per_min"), v_k.clone()), (Py::str("half_life_min"), py::truediv(&c_LN2()?, &v_k.clone())?), (Py::str("initial_fit"), py::math1("exp", &v_a.clone())?), (Py::str("r2"), v_r2.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mutation_rate_p0(mut v_n0: Py, mut v_n: Py, mut v_Nt: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let _ = f__need({ let t3040 = Py::Bool(matches!(v_n0.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3040.truthy() { t3040 } else { { let t3039 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3039.truthy() { t3039 } else { { let t3038 = { let t3033 = v_n.clone(); Py::Bool({ let t3034 = Py::Int(1i64); if py::ge(&t3033, &t3034)? { true } else { false } }) }; if !t3038.truthy() { t3038 } else { { let t3035 = Py::Int(1i64); Py::Bool({ let t3036 = v_n0.clone(); if py::le(&t3035, &t3036)? { { let t3037 = v_n.clone(); if py::lt(&t3036, &t3037)? { true } else { false } } } else { false } }) } } } } } } }, Py::str("n0 must be a whole number between 1 and n - 1"))?;
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
    let _ = f__need({ let t3094 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3094.truthy() { t3094 } else { { let t3091 = Py::Int(1i64); Py::Bool({ let t3092 = v_n.clone(); if py::le(&t3091, &t3092)? { { let t3093 = Py::Int(12i64); if py::le(&t3092, &t3093)? { true } else { false } } } else { false } }) } } }, Py::str("the number of sites n must be a whole number from 1 to 12"))?;
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
    let _ = f__need({ let t3136 = Py::Bool(matches!(v_z.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3136.truthy() { t3136 } else { { let t3135 = py::not(&Py::Bool(matches!(v_z.clone(), Py::Bool(_)))); if !t3135.truthy() { t3135 } else { { let t3133 = v_z.clone(); Py::Bool({ let t3134 = Py::Int(0i64); if py::ne(&t3133, &t3134) { true } else { false } }) } } } } }, Py::str("the charge number z must be a non-zero whole number"))?;
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
    let _ = f__need({ let t3245 = v_molecule.clone(); Py::Bool({ let t3246 = c__A260_FACTOR()?; if py::contains(&t3246, &t3245)? { true } else { false } }) }, Py::str("molecule: dsDNA, ssDNA or RNA"))?;
    let _ = f__need({ let t3249 = f__isnum(v_dilution.clone())?; if !t3249.truthy() { t3249 } else { { let t3247 = v_dilution.clone(); Py::Bool({ let t3248 = Py::Int(1i64); if py::ge(&t3247, &t3248)? { true } else { false } }) } } }, Py::str("the dilution factor must be at least 1"))?;
    let _ = f__pos(Py::str("the path length l"), Py::tuple(vec![v_l.clone()]))?;
    v_ug_ml = py::truediv(&py::mul(&py::mul(&v_A260.clone(), &py::getitem(&c__A260_FACTOR()?, &v_molecule.clone())?)?, &v_dilution.clone())?, &py::mul(&v_l.clone(), &Py::Float(100.0))?)?;
    v_out = Py::dict(vec![(Py::str("c"), py::mul(&v_ug_ml.clone(), &Py::Float(0.001))?), (Py::str("c_ug_mL"), v_ug_ml.clone())]);
    if ({ let t3254 = v_V.clone(); Py::Bool({ let t3255 = Py::None; if !py::is(&t3254, &t3255) { true } else { false } }) }).truthy() {
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
    let t3310 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t3310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3310[0].clone();
    v__ = t3310[1].clone();
    v_n = py::len(&v_s.clone())?;
    if ({ let t3311 = v_n.clone(); Py::Bool({ let t3312 = Py::Int(1i64); if py::eq(&t3311, &t3312) { true } else { false } }) }).truthy() {
        v_eps = py::getitem(&c__EPS1()?, &v_s.clone())?;
    } else {
        v_eps = Py::Float(0.0);
        for t3313 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t3313;
            v_eps = py::iadd(&v_eps, &py::getitem(&c__EPS2()?, &py::slice(&v_s.clone(), Some((v_i.clone()).i()?), Some((py::add(&v_i.clone(), &Py::Int(2i64))?).i()?), None)?)?)?;
        }
        for t3314 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t3314;
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
    let t3369 = (f__clean_seq(v_seq.clone(), Py::Bool(true), Py::Bool(true))?).items()?;
    if t3369.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3369[0].clone();
    v_rna = t3369[1].clone();
    let _ = f__need({ let t3370 = v_strands.clone(); Py::Bool({ let t3371 = Py::tuple(vec![Py::str("single"), Py::str("double")]); if py::contains(&t3371, &t3370)? { true } else { false } }) }, Py::str("strands: single or double"))?;
    v_mw = f__strand_mw(v_s.clone(), v_rna.clone())?;
    v_n_str = Py::Int(1i64);
    if ({ let t3373 = v_strands.clone(); Py::Bool({ let t3374 = Py::str("double"); if py::eq(&t3373, &t3374) { true } else { false } }) }).truthy() {
        v_comp = (if (v_rna.clone()).truthy() { c__RNA_COMP()? } else { c__DNA_COMP()? });
        v_c = Py::str("");
        for t3375 in py::iter(&(v_s.clone()))? {
            v_ch = t3375;
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
    let t3429 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t3429.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3429[0].clone();
    v__ = t3429[1].clone();
    let _ = f__need({ let t3432 = Py::Bool(matches!(v_min_aa.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3432.truthy() { t3432 } else { { let t3430 = v_min_aa.clone(); Py::Bool({ let t3431 = Py::Int(1i64); if py::ge(&t3430, &t3431)? { true } else { false } }) } } }, Py::str("min_aa must be a whole number of codons, at least 1"))?;
    v_L = py::len(&v_s.clone())?;
    v_best = Py::None;
    v_count = Py::Int(0i64);
    for t3433 in py::iter(&(Py::tuple(vec![Py::str("+"), Py::str("-")])))? {
        v_strand = t3433;
        v_t = (if ({ let t3434 = v_strand.clone(); Py::Bool({ let t3435 = Py::str("+"); if py::eq(&t3434, &t3435) { true } else { false } }) }).truthy() { v_s.clone() } else { f__revcomp(v_s.clone(), Py::Bool(false))? });
        for t3437 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v_frame = t3437;
            v_start = py::neg(&Py::Int(1i64))?;
            v_k = v_frame.clone();
            while ({ let t3461 = py::add(&v_k.clone(), &Py::Int(3i64))?; Py::Bool({ let t3462 = v_L.clone(); if py::le(&t3461, &t3462)? { true } else { false } }) }).truthy() {
                v_cod = py::slice(&v_t.clone(), Some((v_k.clone()).i()?), Some((py::add(&v_k.clone(), &Py::Int(3i64))?).i()?), None)?;
                v_aa = py::getitem(&c__CODONS()?, &v_cod.clone())?;
                if ({ let t3438 = v_aa.clone(); Py::Bool({ let t3439 = Py::str("*"); if py::eq(&t3438, &t3439) { true } else { false } }) }).truthy() {
                    if ({ let t3440 = v_start.clone(); Py::Bool({ let t3441 = Py::Int(0i64); if py::ge(&t3440, &t3441)? { true } else { false } }) }).truthy() {
                        v_n_aa = py::floordiv(&py::sub(&v_k.clone(), &v_start.clone())?, &Py::Int(3i64))?;
                        if ({ let t3442 = v_n_aa.clone(); Py::Bool({ let t3443 = v_min_aa.clone(); if py::ge(&t3442, &t3443)? { true } else { false } }) }).truthy() {
                            v_count = py::iadd(&v_count, &Py::Int(1i64))?;
                            v_prot = Py::str("");
                            for t3444 in py::range_iter(&[(v_start.clone()).i()?, (v_k.clone()).i()?, (Py::Int(3i64)).i()?]) {
                                v_q = t3444;
                                v_prot = py::iadd(&v_prot, &py::getitem(&c__CODONS()?, &py::slice(&v_t.clone(), Some((v_q.clone()).i()?), Some((py::add(&v_q.clone(), &Py::Int(3i64))?).i()?), None)?)?)?;
                            }
                            v_cand = Py::tuple(vec![v_n_aa.clone(), v_strand.clone(), py::add(&v_start.clone(), &Py::Int(1i64))?, py::add(&v_k.clone(), &Py::Int(3i64))?, v_prot.clone()]);
                            if ({ let t3455 = { let t3445 = v_best.clone(); Py::Bool({ let t3446 = Py::None; if py::is(&t3445, &t3446) { true } else { false } }) }; if t3455.truthy() { t3455 } else { { let t3454 = { let t3447 = v_n_aa.clone(); Py::Bool({ let t3448 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::gt(&t3447, &t3448)? { true } else { false } }) }; if t3454.truthy() { t3454 } else { { let t3453 = { let t3449 = v_n_aa.clone(); Py::Bool({ let t3450 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::eq(&t3449, &t3450) { true } else { false } }) }; if !t3453.truthy() { t3453 } else { { let t3451 = Py::tuple(vec![v_strand.clone(), py::add(&v_start.clone(), &Py::Int(1i64))?]); Py::Bool({ let t3452 = Py::tuple(vec![py::getitem(&v_best.clone(), &Py::Int(1i64))?, py::getitem(&v_best.clone(), &Py::Int(2i64))?]); if py::lt(&t3451, &t3452)? { true } else { false } }) } } } } } } }).truthy() {
                                v_best = v_cand.clone();
                            }
                        }
                    }
                    v_start = py::neg(&Py::Int(1i64))?;
                } else {
                    if ({ let t3460 = { let t3456 = v_cod.clone(); Py::Bool({ let t3457 = Py::str("ATG"); if py::eq(&t3456, &t3457) { true } else { false } }) }; if !t3460.truthy() { t3460 } else { { let t3458 = v_start.clone(); Py::Bool({ let t3459 = Py::Int(0i64); if py::lt(&t3458, &t3459)? { true } else { false } }) } } }).truthy() {
                        v_start = v_k.clone();
                    }
                }
                v_k = py::iadd(&v_k, &Py::Int(3i64))?;
            }
        }
    }
    let _ = f__need({ let t3463 = v_best.clone(); Py::Bool({ let t3464 = Py::None; if !py::is(&t3463, &t3464) { true } else { false } }) }, Py::str("no open reading frame of at least min_aa codons"))?;
    let t3465 = (v_best.clone()).items()?;
    if t3465.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_n_aa = t3465[0].clone();
    v_strand = t3465[1].clone();
    v_st = t3465[2].clone();
    v_en = t3465[3].clone();
    v_prot = t3465[4].clone();
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
    let _ = f__need({ let t3508 = { let t3504 = py::len(&v_c.clone())?; Py::Bool({ let t3505 = py::len(&v_i.clone())?; if py::eq(&t3504, &t3505) { true } else { false } }) }; if !t3508.truthy() { t3508 } else { { let t3506 = py::len(&v_c.clone())?; Py::Bool({ let t3507 = Py::Int(1i64); if py::ge(&t3506, &t3507)? { true } else { false } }) } } }, Py::str("give one van 't Hoff factor per concentration"))?;
    for t3509 in py::iter(&(v_c.clone()))? {
        v_v = t3509;
        let _ = f__need({ let t3514 = f__isnum(v_v.clone())?; if !t3514.truthy() { t3514 } else { { let t3512 = v_v.clone(); Py::Bool({ let t3513 = Py::Int(0i64); if py::ge(&t3512, &t3513)? { true } else { false } }) } } }, Py::str("concentrations must be zero or positive"))?;
    }
    for t3515 in py::iter(&(v_i.clone()))? {
        v_v = t3515;
        let _ = f__need({ let t3518 = f__isnum(v_v.clone())?; if !t3518.truthy() { t3518 } else { { let t3516 = v_v.clone(); Py::Bool({ let t3517 = Py::Int(0i64); if py::gt(&t3516, &t3517)? { true } else { false } }) } } }, Py::str("van 't Hoff factors must be positive"))?;
    }
    let _ = f__pos(Py::str("the cell's osmolarity"), Py::tuple(vec![v_osm_cell.clone()]))?;
    v_osm = Py::Float(0.0);
    for t3523 in py::range_iter(&[(py::len(&v_c.clone())?).i()?]) {
        v_k = t3523;
        v_osm = py::iadd(&v_osm, &py::mul(&py::getitem(&v_c.clone(), &v_k.clone())?, &py::getitem(&v_i.clone(), &v_k.clone())?)?)?;
    }
    v_cell = v_osm_cell.clone();
    v_diff = py::sub(&v_osm.clone(), &v_cell.clone())?;
    if ({ let t3524 = py::abs(&v_diff.clone())?; Py::Bool({ let t3525 = py::mul(&Py::Float(0.01), &v_cell.clone())?; if py::le(&t3524, &t3525)? { true } else { false } }) }).truthy() {
        v_word = Py::str("isotonic");
    } else {
        if ({ let t3526 = v_diff.clone(); Py::Bool({ let t3527 = Py::Int(0i64); if py::gt(&t3526, &t3527)? { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t3628 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t3628.truthy() { t3628 } else { { let t3625 = Py::Int(0i64); Py::Bool({ let t3626 = v_n.clone(); if py::le(&t3625, &t3626)? { { let t3627 = Py::Int(60i64); if py::le(&t3626, &t3627)? { true } else { false } } } else { false } }) } } }, Py::str("the number of cycles must be a whole number from 0 to 60"))?;
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
    let _ = f__need({ let t3673 = v_N.clone(); Py::Bool({ let t3674 = v_N0.clone(); if py::gt(&t3673, &t3674)? { true } else { false } }) }, Py::str("the target must exceed the starting copies"))?;
    let _ = f__frac(Py::str("the efficiency E"), v_E.clone(), Py::Bool(true), Py::Bool(false))?;
    v_n = py::truediv(&py::math1("log", &py::truediv(&v_N.clone(), &v_N0.clone())?)?, &py::math1("log", &py::add(&Py::Float(1.0), &v_E.clone())?)?)?;
    v_w = py::round(&v_n.clone(), None)?;
    v_cycles = (if ({ let t3684 = py::abs(&py::sub(&v_n.clone(), &v_w.clone())?)?; Py::Bool({ let t3685 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), v_n.clone()], true)?)?; if py::le(&t3684, &t3685)? { true } else { false } }) }).truthy() { py::int(&v_w.clone())? } else { py::int(&py::math1("ceil", &v_n.clone())?)? });
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
    if ({ let t3839 = v_MW.clone(); Py::Bool({ let t3840 = Py::None; if !py::is(&t3839, &t3840) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t3886 = v_T1.clone(); Py::Bool({ let t3887 = v_T2.clone(); if py::ne(&t3886, &t3887) { true } else { false } }) }, Py::str("the two temperatures must differ"))?;
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
    for t3929 in py::iter(&(Py::tuple(vec![v_Ct_target_treated.clone(), v_Ct_ref_treated.clone(), v_Ct_target_control.clone(), v_Ct_ref_control.clone()])))? {
        v_v = t3929;
        let _ = f__need({ let t3930 = Py::Int(0i64); Py::Bool({ let t3931 = v_v.clone(); if py::lt(&t3930, &t3931)? { { let t3932 = Py::Int(60i64); if py::le(&t3931, &t3932)? { true } else { false } } } else { false } }) }, Py::str("Ct values lie between 0 and 60 cycles"))?;
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
    for t3982 in py::iter(&(Py::tuple(vec![v_Ct_target_control.clone(), v_Ct_target_sample.clone(), v_Ct_ref_control.clone(), v_Ct_ref_sample.clone()])))? {
        v_v = t3982;
        let _ = f__need({ let t3986 = f__isnum(v_v.clone())?; if !t3986.truthy() { t3986 } else { { let t3983 = Py::Int(0i64); Py::Bool({ let t3984 = v_v.clone(); if py::lt(&t3983, &t3984)? { { let t3985 = Py::Int(60i64); if py::le(&t3984, &t3985)? { true } else { false } } } else { false } }) } } }, Py::str("Ct values lie between 0 and 60 cycles"))?;
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
    let _ = f__need({ let t4029 = f__isnum(v_slope.clone())?; if !t4029.truthy() { t4029 } else { { let t4027 = v_slope.clone(); Py::Bool({ let t4028 = Py::Int(0i64); if py::lt(&t4027, &t4028)? { true } else { false } }) } } }, Py::str("the slope of a standard curve is negative"))?;
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
    let _ = f__need({ let t4073 = { let t4069 = py::len(&v_quantity.clone())?; Py::Bool({ let t4070 = py::len(&v_Ct.clone())?; if py::eq(&t4069, &t4070) { true } else { false } }) }; if !t4073.truthy() { t4073 } else { { let t4071 = py::len(&v_Ct.clone())?; Py::Bool({ let t4072 = Py::Int(3i64); if py::ge(&t4071, &t4072)? { true } else { false } }) } } }, Py::str("give at least 3 standards with one Ct each"))?;
    for t4074 in py::iter(&(v_quantity.clone()))? {
        v_v = t4074;
        let _ = f__need({ let t4079 = f__isnum(v_v.clone())?; if !t4079.truthy() { t4079 } else { { let t4077 = v_v.clone(); Py::Bool({ let t4078 = Py::Int(0i64); if py::gt(&t4077, &t4078)? { true } else { false } }) } } }, Py::str("standard quantities must be positive"))?;
    }
    v_x = { let mut out = vec![]; for t4080 in py::iter(&(v_quantity.clone()))? { let mut v_v: Py; 
v_v = t4080;
out.push(py::math1("log10", &v_v.clone())?); } Py::list(out) };
    let t4090 = (f__linfit(v_x.clone(), v_Ct.clone())?).items()?;
    if t4090.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4090[0].clone();
    v_b = t4090[1].clone();
    v_r2 = t4090[2].clone();
    let _ = f__need({ let t4091 = v_m.clone(); Py::Bool({ let t4092 = Py::Int(0i64); if py::lt(&t4091, &t4092)? { true } else { false } }) }, Py::str("Ct must fall as the quantity rises (a negative slope)"))?;
    v_amp = py::pow(&Py::Float(10.0), &py::truediv(&py::neg(&Py::Float(1.0))?, &v_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("slope"), v_m.clone()), (Py::str("intercept"), v_b.clone()), (Py::str("r2"), v_r2.clone()), (Py::str("efficiency"), py::sub(&v_amp.clone(), &Py::Float(1.0))?), (Py::str("amplification_factor"), v_amp.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reaction_free_energy(mut v_dG0: Py, mut v_T: Py, mut v_products: Py, mut v_reactants: Py) -> R {
    let mut v_RT: Py = Py::None;
    let mut v_dG: Py = Py::None;
    let mut v_lnQ: Py = Py::None;
    let mut v_word: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_dG0.clone()]))?;
    let _ = f__pos(Py::str("T"), Py::tuple(vec![v_T.clone()]))?;
    let _ = f__need({ let t4138 = py::abs(&v_dG0.clone())?; Py::Bool({ let t4139 = Py::Float(500000.0); if py::le(&t4138, &t4139)? { true } else { false } }) }, Py::str("the standard free-energy change must be within 500 kJ/mol"))?;
    v_lnQ = f__ln_q(v_products.clone(), v_reactants.clone())?;
    v_RT = py::mul(&c_R_GAS()?, &v_T.clone())?;
    v_dG = py::add(&v_dG0.clone(), &py::mul(&v_RT.clone(), &v_lnQ.clone())?)?;
    if ({ let t4159 = py::abs(&v_dG.clone())?; Py::Bool({ let t4160 = py::mul(&Py::Float(1e-09), &py::add(&py::abs(&v_dG0.clone())?, &v_RT.clone())?)?; if py::le(&t4159, &t4160)? { true } else { false } }) }).truthy() {
        v_dG = Py::Float(0.0);
    }
    if ({ let t4161 = v_dG.clone(); Py::Bool({ let t4162 = Py::Int(0i64); if py::lt(&t4161, &t4162)? { true } else { false } }) }).truthy() {
        v_word = Py::str("forward");
    } else {
        if ({ let t4163 = v_dG.clone(); Py::Bool({ let t4164 = Py::Int(0i64); if py::gt(&t4163, &t4164)? { true } else { false } }) }).truthy() {
            v_word = Py::str("reverse");
        } else {
            v_word = Py::str("at equilibrium");
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("Q"), py::math1("exp", &v_lnQ.clone())?), (Py::str("dG"), v_dG.clone()), (Py::str("dG_kJ_mol"), py::truediv(&v_dG.clone(), &Py::Float(1000.0))?), (Py::str("K_eq"), py::math1("exp", &py::truediv(&py::neg(&v_dG0.clone())?, &v_RT.clone())?)?), (Py::str("direction"), v_word.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_redox_free_energy(mut v_E_acceptor: Py, mut v_E_donor: Py, mut v_n: Py) -> R {
    let mut v_dE: Py = Py::None;
    let mut v_dG: Py = Py::None;
    let mut v_word: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_E_acceptor.clone(), v_E_donor.clone()]))?;
    let _ = f__need({ let t4209 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4209.truthy() { t4209 } else { { let t4208 = py::not(&Py::Bool(matches!(v_n.clone(), Py::Bool(_)))); if !t4208.truthy() { t4208 } else { { let t4206 = v_n.clone(); Py::Bool({ let t4207 = Py::Int(1i64); if py::ge(&t4206, &t4207)? { true } else { false } }) } } } } }, Py::str("the number of electrons n must be a whole number of at least 1"))?;
    let _ = f__need({ let t4214 = { let t4210 = py::abs(&v_E_acceptor.clone())?; Py::Bool({ let t4211 = Py::Float(3.0); if py::le(&t4210, &t4211)? { true } else { false } }) }; if !t4214.truthy() { t4214 } else { { let t4212 = py::abs(&v_E_donor.clone())?; Py::Bool({ let t4213 = Py::Float(3.0); if py::le(&t4212, &t4213)? { true } else { false } }) } } }, Py::str("reduction potentials must be within 3 V"))?;
    v_dE = py::sub(&v_E_acceptor.clone(), &v_E_donor.clone())?;
    v_dG = py::mul(&py::mul(&py::neg(&v_n.clone())?, &c_FARADAY()?)?, &v_dE.clone())?;
    if ({ let t4215 = v_dE.clone(); Py::Bool({ let t4216 = Py::Int(0i64); if py::gt(&t4215, &t4216)? { true } else { false } }) }).truthy() {
        v_word = Py::str("favourable as written");
    } else {
        if ({ let t4217 = v_dE.clone(); Py::Bool({ let t4218 = Py::Int(0i64); if py::lt(&t4217, &t4218)? { true } else { false } }) }).truthy() {
            v_word = Py::str("unfavourable as written");
        } else {
            v_word = Py::str("at equilibrium");
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("dE"), v_dE.clone()), (Py::str("dE_mV"), py::mul(&v_dE.clone(), &Py::Float(1000.0))?), (Py::str("dG"), v_dG.clone()), (Py::str("dG_kJ_mol"), py::truediv(&v_dG.clone(), &Py::Float(1000.0))?), (Py::str("direction"), v_word.clone())]))?);
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
    let t4272 = (f__clean_seq(v_seq.clone(), Py::Bool(true), Py::Bool(true))?).items()?;
    if t4272.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4272[0].clone();
    v_rna = t4272[1].clone();
    v_comp = (if (v_rna.clone()).truthy() { c__RNA_COMP()? } else { c__DNA_COMP()? });
    v_c = Py::str("");
    for t4273 in py::iter(&(v_s.clone()))? {
        v_ch = t4273;
        v_c = py::iadd(&v_c, &py::getitem(&v_comp.clone(), &v_ch.clone())?)?;
    }
    v_rev = Py::str("");
    for t4274 in py::range_iter(&[(py::sub(&py::len(&v_s.clone())?, &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_k = t4274;
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
    let _ = f__need({ let t4320 = v_dims.clone(); Py::Bool({ let t4321 = Py::tuple(vec![Py::Int(1i64), Py::Int(2i64), Py::Int(3i64)]); if py::contains(&t4321, &t4320)? { true } else { false } }) }, Py::str("dims must be 1, 2 or 3"))?;
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
    let _ = f__need({ let t4364 = { let t4360 = py::len(&v_B.clone())?; Py::Bool({ let t4361 = py::len(&v_F.clone())?; if py::eq(&t4360, &t4361) { true } else { false } }) }; if !t4364.truthy() { t4364 } else { { let t4362 = py::len(&v_B.clone())?; Py::Bool({ let t4363 = Py::Int(3i64); if py::ge(&t4362, &t4363)? { true } else { false } }) } } }, Py::str("give at least 3 bound and free concentrations"))?;
    for t4365 in py::iter(&(v_B.clone()))? {
        v_a = t4365;
        let _ = f__need({ let t4370 = f__isnum(v_a.clone())?; if !t4370.truthy() { t4370 } else { { let t4368 = v_a.clone(); Py::Bool({ let t4369 = Py::Int(0i64); if py::gt(&t4368, &t4369)? { true } else { false } }) } } }, Py::str("bound concentrations must be positive"))?;
    }
    for t4371 in py::iter(&(v_F.clone()))? {
        v_a = t4371;
        let _ = f__need({ let t4374 = f__isnum(v_a.clone())?; if !t4374.truthy() { t4374 } else { { let t4372 = v_a.clone(); Py::Bool({ let t4373 = Py::Int(0i64); if py::gt(&t4372, &t4373)? { true } else { false } }) } } }, Py::str("free concentrations must be positive"))?;
    }
    v_y = { let mut out = vec![]; for t4375 in py::range_iter(&[(py::len(&v_B.clone())?).i()?]) { let mut v_k: Py; 
v_k = t4375;
out.push(py::truediv(&py::getitem(&v_B.clone(), &v_k.clone())?, &py::getitem(&v_F.clone(), &v_k.clone())?)?); } Py::list(out) };
    let t4385 = (f__linfit(v_B.clone(), v_y.clone())?).items()?;
    if t4385.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t4385[0].clone();
    v_b = t4385[1].clone();
    v_r2 = t4385[2].clone();
    let _ = f__need({ let t4386 = v_m.clone(); Py::Bool({ let t4387 = Py::Int(0i64); if py::lt(&t4386, &t4387)? { true } else { false } }) }, Py::str("the Scatchard line must have a negative slope"))?;
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
    let _ = f__need({ let t4432 = v_r_max.clone(); Py::Bool({ let t4433 = v_r_min.clone(); if py::gt(&t4432, &t4433)? { true } else { false } }) }, Py::str("r_max must exceed r_min"))?;
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
    let _ = f__need({ let t4480 = f__isnum(v_f.clone())?; if !t4480.truthy() { t4480 } else { { let t4478 = v_f.clone(); Py::Bool({ let t4479 = Py::Int(1i64); if py::gt(&t4478, &t4479)? { true } else { false } }) } } }, Py::str("the dilution factor per step must exceed 1"))?;
    let _ = f__need({ let t4484 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4484.truthy() { t4484 } else { { let t4481 = Py::Int(1i64); Py::Bool({ let t4482 = v_n.clone(); if py::le(&t4481, &t4482)? { { let t4483 = Py::Int(30i64); if py::le(&t4482, &t4483)? { true } else { false } } } else { false } }) } } }, Py::str("the number of steps must be a whole number from 1 to 30"))?;
    v_concs = Py::list(vec![]);
    v_c = v_C0.clone();
    for t4485 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t4485;
        v_c = py::truediv(&v_c.clone(), &v_f.clone())?;
        let _ = py::method(&v_concs.clone(), "append", &[v_c.clone()])?;
    }
    v_out = Py::dict(vec![(Py::str("C_final"), py::getitem(&v_concs.clone(), &py::neg(&Py::Int(1i64))?)?), (Py::str("total_dilution"), py::pow(&v_f.clone(), &v_n.clone())?), (Py::str("concentrations_mM"), v_concs.clone())]);
    if ({ let t4486 = v_V.clone(); Py::Bool({ let t4487 = Py::None; if !py::is(&t4486, &t4487) { true } else { false } }) }).truthy() {
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
fn f_specimen_size(mut v_image_size: Py, mut v_M: Py, mut v_bar_image: Py, mut v_bar_label: Py) -> R {
    let mut v_Meff: Py = Py::None;
    let mut v_bars: Py = Py::None;
    let mut v_size: Py = Py::None;
    let _ = f__pos(Py::str("the measured image size"), Py::tuple(vec![v_image_size.clone()]))?;
    v_bars = { let t4537 = { let t4533 = v_bar_image.clone(); Py::Bool({ let t4534 = Py::None; if !py::is(&t4533, &t4534) { true } else { false } }) }; if t4537.truthy() { t4537 } else { { let t4535 = v_bar_label.clone(); Py::Bool({ let t4536 = Py::None; if !py::is(&t4535, &t4536) { true } else { false } }) } } };
    let _ = f__need(py::not(&{ let t4540 = { let t4538 = v_M.clone(); Py::Bool({ let t4539 = Py::None; if !py::is(&t4538, &t4539) { true } else { false } }) }; if !t4540.truthy() { t4540 } else { v_bars.clone() } }), Py::str("give either the magnification M or the scale bar (bar_image and bar_label), not both"))?;
    if ({ let t4541 = v_M.clone(); Py::Bool({ let t4542 = Py::None; if !py::is(&t4541, &t4542) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the magnification M"), Py::tuple(vec![v_M.clone()]))?;
        v_Meff = v_M.clone();
        v_size = py::truediv(&v_image_size.clone(), &v_M.clone())?;
    } else {
        let _ = f__need({ let t4547 = { let t4543 = v_bar_image.clone(); Py::Bool({ let t4544 = Py::None; if !py::is(&t4543, &t4544) { true } else { false } }) }; if !t4547.truthy() { t4547 } else { { let t4545 = v_bar_label.clone(); Py::Bool({ let t4546 = Py::None; if !py::is(&t4545, &t4546) { true } else { false } }) } } }, Py::str("give the magnification M, or both the scale bar's length on the image (bar_image) and the length it stands for (bar_label)"))?;
        let _ = f__pos(Py::str("the scale-bar lengths"), Py::tuple(vec![v_bar_image.clone(), v_bar_label.clone()]))?;
        v_Meff = py::truediv(&v_bar_image.clone(), &v_bar_label.clone())?;
        v_size = py::truediv(&py::mul(&v_image_size.clone(), &v_bar_label.clone())?, &v_bar_image.clone())?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("size"), v_size.clone()), (Py::str("size_um"), py::mul(&v_size.clone(), &Py::Float(1000000.0))?), (Py::str("M_image"), v_Meff.clone())]))?);
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
fn f_thermal_death_time(mut v_D_ref: Py, mut v_T_ref: Py, mut v_z: Py, mut v_T: Py, mut v_log_reductions: Py) -> R {
    let mut v_D_T: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__pos(Py::str("D_ref, T_ref, z, T and the number of log reductions"), Py::tuple(vec![v_D_ref.clone(), v_T_ref.clone(), v_z.clone(), v_T.clone(), v_log_reductions.clone()]))?;
    let _ = f__need({ let t4689 = py::truediv(&py::abs(&py::sub(&v_T.clone(), &v_T_ref.clone())?)?, &v_z.clone())?; Py::Bool({ let t4690 = Py::Float(50.0); if py::le(&t4689, &t4690)? { true } else { false } }) }, Py::str("the temperature is too far from the reference temperature for this z-value"))?;
    v_L = py::pow(&Py::Float(10.0), &py::truediv(&py::sub(&v_T.clone(), &v_T_ref.clone())?, &v_z.clone())?)?;
    v_D_T = py::truediv(&v_D_ref.clone(), &v_L.clone())?;
    v_t = py::mul(&v_log_reductions.clone(), &v_D_T.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("D_T"), v_D_T.clone()), (Py::str("D_T_min"), py::truediv(&v_D_T.clone(), &c_MINUTE()?)?), (Py::str("t"), v_t.clone()), (Py::str("t_min"), py::truediv(&v_t.clone(), &c_MINUTE()?)?), (Py::str("lethal_rate"), v_L.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tm_basic(mut v_seq: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_gc: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t4744 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4744.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4744[0].clone();
    v__ = t4744[1].clone();
    let _ = f__need({ let t4745 = py::len(&v_s.clone())?; Py::Bool({ let t4746 = Py::Int(14i64); if py::ge(&t4745, &t4746)? { true } else { false } }) }, Py::str("the basic formula is for 14 nt or more (use the Wallace rule for shorter primers)"))?;
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
    let t4800 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4800[0].clone();
    v__ = t4800[1].clone();
    let _ = f__need({ let t4801 = py::len(&v_s.clone())?; Py::Bool({ let t4802 = Py::Int(2i64); if py::ge(&t4801, &t4802)? { true } else { false } }) }, Py::str("the sequence needs at least 2 bases"))?;
    let _ = f__pos(Py::str("Na and c1"), Py::tuple(vec![v_Na.clone(), v_c1.clone()]))?;
    let _ = f__nonneg(Py::str("c2"), Py::tuple(vec![v_c2.clone()]))?;
    let t4821 = (f__nn_sums(v_s.clone())?).items()?;
    if t4821.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_dh = t4821[0].clone();
    v_ds = t4821[1].clone();
    v_selfcomp = t4821[2].clone();
    if (v_selfcomp.clone()).truthy() {
        v_k = v_c1.clone();
    } else {
        let _ = f__need({ let t4822 = v_c1.clone(); Py::Bool({ let t4823 = py::truediv(&v_c2.clone(), &Py::Float(2.0))?; if py::gt(&t4822, &t4823)? { true } else { false } }) }, Py::str("c1 must exceed c2/2 (k = c1 - c2/2 must be positive)"))?;
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
    let t4877 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4877.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4877[0].clone();
    v__ = t4877[1].clone();
    let _ = f__need({ let t4878 = py::len(&v_s.clone())?; Py::Bool({ let t4879 = Py::Int(14i64); if py::ge(&t4878, &t4879)? { true } else { false } }) }, Py::str("the salt-adjusted formula is for 14 nt or more"))?;
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
    let t4939 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4939.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4939[0].clone();
    v__ = t4939[1].clone();
    let _ = f__need({ let t4940 = py::len(&v_s.clone())?; Py::Bool({ let t4941 = Py::Int(2i64); if py::ge(&t4940, &t4941)? { true } else { false } }) }, Py::str("the primer needs at least 2 bases"))?;
    let t4942 = (Py::tuple(vec![py::add(&py::method(&v_s.clone(), "count", &[Py::str("A")])?, &py::method(&v_s.clone(), "count", &[Py::str("T")])?)?, py::add(&py::method(&v_s.clone(), "count", &[Py::str("G")])?, &py::method(&v_s.clone(), "count", &[Py::str("C")])?)?])).items()?;
    if t4942.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_at = t4942[0].clone();
    v_gc = t4942[1].clone();
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
    let t4996 = (f__clean_seq(v_seq.clone(), Py::Bool(false), Py::Bool(true))?).items()?;
    if t4996.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t4996[0].clone();
    v_rna = t4996[1].clone();
    let _ = f__need({ let t4997 = v_strand.clone(); Py::Bool({ let t4998 = Py::tuple(vec![Py::str("coding"), Py::str("template")]); if py::contains(&t4998, &t4997)? { true } else { false } }) }, Py::str("strand: coding or template"))?;
    v_src = (if ({ let t4999 = v_strand.clone(); Py::Bool({ let t5000 = Py::str("coding"); if py::eq(&t4999, &t5000) { true } else { false } }) }).truthy() { v_s.clone() } else { f__revcomp(v_s.clone(), Py::Bool(false))? });
    v_m = Py::str("");
    for t5002 in py::iter(&(v_src.clone()))? {
        v_ch = t5002;
        v_m = py::iadd(&v_m, &(if ({ let t5003 = v_ch.clone(); Py::Bool({ let t5004 = Py::str("T"); if py::eq(&t5003, &t5004) { true } else { false } }) }).truthy() { Py::str("U") } else { v_ch.clone() }))?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("mRNA"), v_m.clone()), (Py::str("length"), py::len(&v_m.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_transcription_time(mut v_n: Py, mut v_rate: Py, mut v_k_init: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__need({ let t5045 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5045.truthy() { t5045 } else { { let t5043 = v_n.clone(); Py::Bool({ let t5044 = Py::Int(1i64); if py::ge(&t5043, &t5044)? { true } else { false } }) } } }, Py::str("the gene length must be a whole number of nucleotides"))?;
    let _ = f__pos(Py::str("the elongation rate"), Py::tuple(vec![v_rate.clone()]))?;
    v_t = py::truediv(&v_n.clone(), &v_rate.clone())?;
    v_out = Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_min"), py::truediv(&v_t.clone(), &c_MINUTE()?)?)]);
    if ({ let t5052 = v_k_init.clone(); Py::Bool({ let t5053 = Py::None; if !py::is(&t5052, &t5053) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t5095 = Py::Bool(matches!(v_colonies.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5095.truthy() { t5095 } else { { let t5093 = v_colonies.clone(); Py::Bool({ let t5094 = Py::Int(0i64); if py::ge(&t5093, &t5094)? { true } else { false } }) } } }, Py::str("the colony count must be a whole number"))?;
    let _ = f__pos(Py::str("the DNA mass and the volumes"), Py::tuple(vec![v_m_DNA.clone(), v_V_plated.clone(), v_V_total.clone()]))?;
    let _ = f__need({ let t5102 = v_V_plated.clone(); Py::Bool({ let t5103 = v_V_total.clone(); if py::le(&t5102, &t5103)? { true } else { false } }) }, Py::str("the plated volume cannot exceed the total volume"))?;
    v_ug_plated = py::truediv(&py::mul(&py::mul(&v_m_DNA.clone(), &Py::Float(1000000000.0))?, &v_V_plated.clone())?, &v_V_total.clone())?;
    v_eff = py::truediv(&v_colonies.clone(), &v_ug_plated.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("efficiency"), v_eff.clone()), (Py::str("log10_efficiency"), (if ({ let t5116 = v_eff.clone(); Py::Bool({ let t5117 = Py::Int(0i64); if py::gt(&t5116, &t5117)? { true } else { false } }) }).truthy() { py::math1("log10", &v_eff.clone())? } else { Py::Float(0.0) })), (Py::str("DNA_plated_ug"), v_ug_plated.clone())]))?);
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
    let t5159 = (f__clean_seq(v_seq.clone(), Py::Bool(true), Py::Bool(true))?).items()?;
    if t5159.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t5159[0].clone();
    v_rna = t5159[1].clone();
    v_d = py::method(&v_s.clone(), "replace", &[Py::str("U"), Py::str("T")])?;
    v_n = py::floordiv(&py::len(&v_d.clone())?, &Py::Int(3i64))?;
    let _ = f__need({ let t5160 = v_n.clone(); Py::Bool({ let t5161 = Py::Int(1i64); if py::ge(&t5160, &t5161)? { true } else { false } }) }, Py::str("the sequence needs at least one codon"))?;
    v_full = Py::str("");
    for t5162 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t5162;
        v_full = py::iadd(&v_full, &py::getitem(&c__CODONS()?, &py::slice(&v_d.clone(), Some((py::mul(&Py::Int(3i64), &v_k.clone())?).i()?), Some((py::add(&py::mul(&Py::Int(3i64), &v_k.clone())?, &Py::Int(3i64))?).i()?), None)?)?)?;
    }
    v_stop = py::method(&v_full.clone(), "find", &[Py::str("*")])?;
    v_protein = (if ({ let t5163 = v_stop.clone(); Py::Bool({ let t5164 = Py::Int(0i64); if py::lt(&t5163, &t5164)? { true } else { false } }) }).truthy() { v_full.clone() } else { py::slice(&v_full.clone(), None, Some((v_stop.clone()).i()?), None)? });
    return Ok(f__out(Py::dict(vec![(Py::str("protein"), v_protein.clone()), (Py::str("full_translation"), v_full.clone()), (Py::str("length_aa"), py::len(&v_protein.clone())?), (Py::str("stops"), py::method(&v_full.clone(), "count", &[Py::str("*")])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_translation_time(mut v_n_aa: Py, mut v_rate: Py, mut v_k_init: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    let _ = f__need({ let t5205 = Py::Bool(matches!(v_n_aa.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5205.truthy() { t5205 } else { { let t5203 = v_n_aa.clone(); Py::Bool({ let t5204 = Py::Int(1i64); if py::ge(&t5203, &t5204)? { true } else { false } }) } } }, Py::str("the protein length must be a whole number of amino acids"))?;
    let _ = f__pos(Py::str("the elongation rate"), Py::tuple(vec![v_rate.clone()]))?;
    v_t = py::truediv(&v_n_aa.clone(), &v_rate.clone())?;
    v_out = Py::dict(vec![(Py::str("t"), v_t.clone()), (Py::str("t_min"), py::truediv(&v_t.clone(), &c_MINUTE()?)?), (Py::str("coding_nt"), py::mul(&Py::Int(3i64), &py::add(&v_n_aa.clone(), &Py::Int(1i64))?)?)]);
    if ({ let t5212 = v_k_init.clone(); Py::Bool({ let t5213 = Py::None; if !py::is(&t5212, &t5213) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t5317 = v_p.clone(); Py::Bool({ let t5318 = Py::Int(0i64); if py::gt(&t5317, &t5318)? { true } else { false } }) }, Py::str("the absorbances give no positive protein estimate (nucleic acid dominates)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("protein"), v_p.clone()), (Py::str("protein_mg_mL"), v_p.clone()), (Py::str("ratio_280_260"), (if ({ let t5331 = v_A260.clone(); Py::Bool({ let t5332 = Py::Int(0i64); if py::gt(&t5331, &t5332)? { true } else { false } }) }).truthy() { py::truediv(&v_A280.clone(), &v_A260.clone())? } else { Py::Float(0.0) }))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_water_movement(mut v_i_cell: Py, mut v_C_cell: Py, mut v_P_cell: Py, mut v_i_out: Py, mut v_C_out: Py, mut v_P_out: Py, mut v_T: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_psi_cell: Py = Py::None;
    let mut v_psi_out: Py = Py::None;
    let mut v_word: Py = Py::None;
    let _ = f__pos(Py::str("the van 't Hoff factors and T"), Py::tuple(vec![v_i_cell.clone(), v_i_out.clone(), v_T.clone()]))?;
    let _ = f__nonneg(Py::str("the concentrations"), Py::tuple(vec![v_C_cell.clone(), v_C_out.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_P_cell.clone(), v_P_out.clone()]))?;
    v_psi_cell = py::add(&py::mul(&py::mul(&py::mul(&py::neg(&v_i_cell.clone())?, &v_C_cell.clone())?, &c_R_GAS()?)?, &v_T.clone())?, &v_P_cell.clone())?;
    v_psi_out = py::add(&py::mul(&py::mul(&py::mul(&py::neg(&v_i_out.clone())?, &v_C_out.clone())?, &c_R_GAS()?)?, &v_T.clone())?, &v_P_out.clone())?;
    v_d = py::sub(&v_psi_out.clone(), &v_psi_cell.clone())?;
    if ({ let t5370 = py::abs(&v_d.clone())?; Py::Bool({ let t5371 = py::mul(&Py::Float(1e-09), &py::add(&py::abs(&v_psi_cell.clone())?, &py::abs(&v_psi_out.clone())?)?)?; if py::le(&t5370, &t5371)? { true } else { false } }) }).truthy() {
        v_d = Py::Float(0.0);
    }
    if ({ let t5372 = v_d.clone(); Py::Bool({ let t5373 = Py::Int(0i64); if py::gt(&t5372, &t5373)? { true } else { false } }) }).truthy() {
        v_word = Py::str("into the cell");
    } else {
        if ({ let t5374 = v_d.clone(); Py::Bool({ let t5375 = Py::Int(0i64); if py::lt(&t5374, &t5375)? { true } else { false } }) }).truthy() {
            v_word = Py::str("out of the cell");
        } else {
            v_word = Py::str("no net movement");
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("psi_cell"), v_psi_cell.clone()), (Py::str("psi_out"), v_psi_out.clone()), (Py::str("dpsi"), v_d.clone()), (Py::str("dpsi_MPa"), py::mul(&v_d.clone(), &Py::Float(1e-06))?), (Py::str("direction"), v_word.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_water_potential(mut v_i: Py, mut v_C: Py, mut v_T: Py, mut v_psi_p: Py) -> R {
    let mut v_psi: Py = Py::None;
    let mut v_psi_s: Py = Py::None;
    let _ = f__pos(Py::str("the van 't Hoff factor i and T"), Py::tuple(vec![v_i.clone(), v_T.clone()]))?;
    let _ = f__nonneg(Py::str("the solute concentration C"), Py::tuple(vec![v_C.clone()]))?;
    let _ = f__fin(Py::tuple(vec![v_psi_p.clone()]))?;
    v_psi_s = py::mul(&py::mul(&py::mul(&py::neg(&v_i.clone())?, &v_C.clone())?, &c_R_GAS()?)?, &v_T.clone())?;
    v_psi = py::add(&v_psi_s.clone(), &v_psi_p.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("psi_s"), v_psi_s.clone()), (Py::str("psi_s_MPa"), py::mul(&v_psi_s.clone(), &Py::Float(1e-06))?), (Py::str("psi"), v_psi.clone()), (Py::str("psi_MPa"), py::mul(&v_psi.clone(), &Py::Float(1e-06))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_z_value(mut v_D1: Py, mut v_T1: Py, mut v_D2: Py, mut v_T2: Py) -> R {
    let mut v_z: Py = Py::None;
    let _ = f__pos(Py::str("the D-values and temperatures"), Py::tuple(vec![v_D1.clone(), v_D2.clone(), v_T1.clone(), v_T2.clone()]))?;
    let _ = f__need({ let t5473 = { let t5469 = v_T1.clone(); Py::Bool({ let t5470 = v_T2.clone(); if py::ne(&t5469, &t5470) { true } else { false } }) }; if !t5473.truthy() { t5473 } else { { let t5471 = v_D1.clone(); Py::Bool({ let t5472 = v_D2.clone(); if py::ne(&t5471, &t5472) { true } else { false } }) } } }, Py::str("the two temperatures and the two D-values must differ"))?;
    let _ = f__need({ let t5474 = py::mul(&py::sub(&v_T2.clone(), &v_T1.clone())?, &py::sub(&v_D1.clone(), &v_D2.clone())?)?; Py::Bool({ let t5475 = Py::Int(0i64); if py::gt(&t5474, &t5475)? { true } else { false } }) }, Py::str("the D-value must fall as the temperature rises (z > 0)"))?;
    v_z = py::truediv(&py::sub(&v_T2.clone(), &v_T1.clone())?, &py::math1("log10", &py::truediv(&v_D1.clone(), &v_D2.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("z"), v_z.clone()), (Py::str("fold_per_10_degrees"), py::pow(&Py::Float(10.0), &py::truediv(&Py::Float(10.0), &v_z.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_100(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t102: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t101 = v_inputs.clone(); f_atp_yield(py::getitem(&t101, &Py::str("P_O_NADH"))?, py::getitem(&t101, &Py::str("P_O_FADH2"))?, py::getitem(&t101, &Py::str("shuttle"))?, py::method(&t101, "get", &[Py::str("dG_ATP"), Py::None])?, py::method(&t101, "get", &[Py::str("dG_glucose"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t102 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t103 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t125 = (t103).items()?;
        if t125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t125[0].clone();
        v_v = t125[1].clone();
        if ({ let t104 = v_k.clone(); Py::Bool({ let t105 = v_outputs.clone(); if !py::contains(&t105, &t104)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1031(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1033: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1032 = v_inputs.clone(); f_dna_copy_number(py::getitem(&t1032, &Py::str("m"))?, py::getitem(&t1032, &Py::str("n"))?, py::getitem(&t1032, &Py::str("molecule"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1033 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1034 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1056 = (t1034).items()?;
        if t1056.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1056[0].clone();
        v_v = t1056[1].clone();
        if ({ let t1035 = v_k.clone(); Py::Bool({ let t1036 = v_outputs.clone(); if !py::contains(&t1036, &t1035)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1079(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1081: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1080 = v_inputs.clone(); f_donnan_equilibrium(py::getitem(&t1080, &Py::str("A"))?, py::getitem(&t1080, &Py::str("zA"))?, py::getitem(&t1080, &Py::str("c"))?, py::getitem(&t1080, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1081 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1082 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1104 = (t1082).items()?;
        if t1104.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1104[0].clone();
        v_v = t1104[1].clone();
        if ({ let t1083 = v_k.clone(); Py::Bool({ let t1084 = v_outputs.clone(); if !py::contains(&t1084, &t1083)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1125(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1127: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1126 = v_inputs.clone(); f_doubling_from_counts(py::getitem(&t1126, &Py::str("N0"))?, py::getitem(&t1126, &Py::str("N"))?, py::getitem(&t1126, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1127 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1128 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1150 = (t1128).items()?;
        if t1150.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1150[0].clone();
        v_v = t1150[1].clone();
        if ({ let t1129 = v_k.clone(); Py::Bool({ let t1130 = v_outputs.clone(); if !py::contains(&t1130, &t1129)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1177(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1179: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1178 = v_inputs.clone(); f_enzyme_assay_activity(py::getitem(&t1178, &Py::str("dA"))?, py::getitem(&t1178, &Py::str("dt"))?, py::getitem(&t1178, &Py::str("eps"))?, py::getitem(&t1178, &Py::str("l"))?, py::getitem(&t1178, &Py::str("V_assay"))?, py::getitem(&t1178, &Py::str("V_enzyme"))?, py::method(&t1178, "get", &[Py::str("protein"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1179 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1180 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1202 = (t1180).items()?;
        if t1202.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1202[0].clone();
        v_v = t1202[1].clone();
        if ({ let t1181 = v_k.clone(); Py::Bool({ let t1182 = v_outputs.clone(); if !py::contains(&t1182, &t1181)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1225(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1227: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1226 = v_inputs.clone(); f_exponential_growth(py::getitem(&t1226, &Py::str("N0"))?, py::getitem(&t1226, &Py::str("td"))?, py::getitem(&t1226, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1227 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1228 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1250 = (t1228).items()?;
        if t1250.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1250[0].clone();
        v_v = t1250[1].clone();
        if ({ let t1229 = v_k.clone(); Py::Bool({ let t1230 = v_outputs.clone(); if !py::contains(&t1230, &t1229)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1269(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1271: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1270 = v_inputs.clone(); f_expression_steady_state(py::getitem(&t1270, &Py::str("k_tx"))?, py::getitem(&t1270, &Py::str("delta_m"))?, py::getitem(&t1270, &Py::str("k_tl"))?, py::getitem(&t1270, &Py::str("delta_p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1271 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1272 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1294 = (t1272).items()?;
        if t1294.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1294[0].clone();
        v_v = t1294[1].clone();
        if ({ let t1273 = v_k.clone(); Py::Bool({ let t1274 = v_outputs.clone(); if !py::contains(&t1274, &t1273)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1317(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1319: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1318 = v_inputs.clone(); f_fick_flux(py::getitem(&t1318, &Py::str("D"))?, py::getitem(&t1318, &Py::str("C1"))?, py::getitem(&t1318, &Py::str("C2"))?, py::getitem(&t1318, &Py::str("dx"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1319 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1320 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1342 = (t1320).items()?;
        if t1342.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1342[0].clone();
        v_v = t1342[1].clone();
        if ({ let t1321 = v_k.clone(); Py::Bool({ let t1322 = v_outputs.clone(); if !py::contains(&t1322, &t1321)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1364(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1366: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1365 = v_inputs.clone(); f_fraction_bound(py::getitem(&t1365, &Py::str("L"))?, py::getitem(&t1365, &Py::str("Kd"))?, py::method(&t1365, "get", &[Py::str("Rt"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1366 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1367 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1389 = (t1367).items()?;
        if t1389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1389[0].clone();
        v_v = t1389[1].clone();
        if ({ let t1368 = v_k.clone(); Py::Bool({ let t1369 = v_outputs.clone(); if !py::contains(&t1369, &t1368)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_144(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t146: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t145 = v_inputs.clone(); f_binding_free_energy(py::getitem(&t145, &Py::str("Kd"))?, py::getitem(&t145, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t146 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t147 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t169 = (t147).items()?;
        if t169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t169[0].clone();
        v_v = t169[1].clone();
        if ({ let t148 = v_k.clone(); Py::Bool({ let t149 = v_outputs.clone(); if !py::contains(&t149, &t148)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1443(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1445: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1444 = v_inputs.clone(); f_frap_diffusion(py::getitem(&t1444, &Py::str("w"))?, py::getitem(&t1444, &Py::str("t_half"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1445 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1446 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1468 = (t1446).items()?;
        if t1468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1468[0].clone();
        v_v = t1468[1].clone();
        if ({ let t1447 = v_k.clone(); Py::Bool({ let t1448 = v_outputs.clone(); if !py::contains(&t1448, &t1447)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1498(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1500: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1499 = v_inputs.clone(); f_gc_content(py::getitem(&t1499, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1500 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1501 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1523 = (t1501).items()?;
        if t1523.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1523[0].clone();
        v_v = t1523[1].clone();
        if ({ let t1502 = v_k.clone(); Py::Bool({ let t1503 = v_outputs.clone(); if !py::contains(&t1503, &t1502)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1549(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1551: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1550 = v_inputs.clone(); f_genome_coverage(py::getitem(&t1550, &Py::str("L"))?, py::getitem(&t1550, &Py::str("N"))?, py::getitem(&t1550, &Py::str("G"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1551 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1552 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1574 = (t1552).items()?;
        if t1574.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1574[0].clone();
        v_v = t1574[1].clone();
        if ({ let t1553 = v_k.clone(); Py::Bool({ let t1554 = v_outputs.clone(); if !py::contains(&t1554, &t1553)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1613(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1615: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1614 = v_inputs.clone(); f_growth_rate_fit(py::getitem(&t1614, &Py::str("t"))?, py::getitem(&t1614, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1615 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1616 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1638 = (t1616).items()?;
        if t1638.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1638[0].clone();
        v_v = t1638[1].clone();
        if ({ let t1617 = v_k.clone(); Py::Bool({ let t1618 = v_outputs.clone(); if !py::contains(&t1618, &t1617)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1670(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1672: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1671 = v_inputs.clone(); f_hemocytometer(py::getitem(&t1671, &Py::str("counts"))?, py::getitem(&t1671, &Py::str("dilution"))?, py::method(&t1671, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1672 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1673 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1695 = (t1673).items()?;
        if t1695.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1695[0].clone();
        v_v = t1695[1].clone();
        if ({ let t1674 = v_k.clone(); Py::Bool({ let t1675 = v_outputs.clone(); if !py::contains(&t1675, &t1674)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1715(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1717: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1716 = v_inputs.clone(); f_henderson_hasselbalch(py::getitem(&t1716, &Py::str("pKa"))?, py::getitem(&t1716, &Py::str("A"))?, py::getitem(&t1716, &Py::str("HA"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1717 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1718 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1740 = (t1718).items()?;
        if t1740.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1740[0].clone();
        v_v = t1740[1].clone();
        if ({ let t1719 = v_k.clone(); Py::Bool({ let t1720 = v_outputs.clone(); if !py::contains(&t1720, &t1719)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1761(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1763: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1762 = v_inputs.clone(); f_hill_from_ec(py::getitem(&t1762, &Py::str("EC10"))?, py::getitem(&t1762, &Py::str("EC90"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1763 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1764 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1786 = (t1764).items()?;
        if t1786.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1786[0].clone();
        v_v = t1786[1].clone();
        if ({ let t1765 = v_k.clone(); Py::Bool({ let t1766 = v_outputs.clone(); if !py::contains(&t1766, &t1765)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1805(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1807: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1806 = v_inputs.clone(); f_hill_equation(py::getitem(&t1806, &Py::str("L"))?, py::getitem(&t1806, &Py::str("K"))?, py::getitem(&t1806, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1807 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1808 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1830 = (t1808).items()?;
        if t1830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1830[0].clone();
        v_v = t1830[1].clone();
        if ({ let t1809 = v_k.clone(); Py::Bool({ let t1810 = v_outputs.clone(); if !py::contains(&t1810, &t1809)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1858(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1860: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1859 = v_inputs.clone(); f_hill_ligand_for_fraction(py::getitem(&t1859, &Py::str("K"))?, py::getitem(&t1859, &Py::str("n"))?, py::getitem(&t1859, &Py::str("theta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1860 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1861 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1883 = (t1861).items()?;
        if t1883.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1883[0].clone();
        v_v = t1883[1].clone();
        if ({ let t1862 = v_k.clone(); Py::Bool({ let t1863 = v_outputs.clone(); if !py::contains(&t1863, &t1862)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1908(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1910: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1909 = v_inputs.clone(); f_ic50_to_ki(py::getitem(&t1909, &Py::str("mode"))?, py::getitem(&t1909, &Py::str("IC50"))?, py::getitem(&t1909, &Py::str("S"))?, py::getitem(&t1909, &Py::str("Km"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1910 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1911 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1933 = (t1911).items()?;
        if t1933.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1933[0].clone();
        v_v = t1933[1].clone();
        if ({ let t1912 = v_k.clone(); Py::Bool({ let t1913 = v_outputs.clone(); if !py::contains(&t1913, &t1912)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1958(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1960: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1959 = v_inputs.clone(); f_induction_dynamics(py::getitem(&t1959, &Py::str("k_tx"))?, py::getitem(&t1959, &Py::str("delta_m"))?, py::getitem(&t1959, &Py::str("k_tl"))?, py::getitem(&t1959, &Py::str("delta_p"))?, py::getitem(&t1959, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1960 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1961 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1983 = (t1961).items()?;
        if t1983.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1983[0].clone();
        v_v = t1983[1].clone();
        if ({ let t1962 = v_k.clone(); Py::Bool({ let t1963 = v_outputs.clone(); if !py::contains(&t1963, &t1962)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_199(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t201: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t200 = v_inputs.clone(); f_buffer_after_addition(py::getitem(&t200, &Py::str("C"))?, py::getitem(&t200, &Py::str("V"))?, py::getitem(&t200, &Py::str("pH0"))?, py::getitem(&t200, &Py::str("pKa"))?, py::getitem(&t200, &Py::str("n_add"))?, py::getitem(&t200, &Py::str("added"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t201 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t202 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t224 = (t202).items()?;
        if t224.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t224[0].clone();
        v_v = t224[1].clone();
        if ({ let t203 = v_k.clone(); Py::Bool({ let t204 = v_outputs.clone(); if !py::contains(&t204, &t203)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2015(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2017: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2016 = v_inputs.clone(); f_ionic_strength(py::getitem(&t2016, &Py::str("c"))?, py::getitem(&t2016, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2017 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2018 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2040 = (t2018).items()?;
        if t2040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2040[0].clone();
        v_v = t2040[1].clone();
        if ({ let t2019 = v_k.clone(); Py::Bool({ let t2020 = v_outputs.clone(); if !py::contains(&t2020, &t2019)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2059(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2061: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2060 = v_inputs.clone(); f_kcat_efficiency(py::getitem(&t2060, &Py::str("Vmax"))?, py::getitem(&t2060, &Py::str("Et"))?, py::getitem(&t2060, &Py::str("Km"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2061 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2062 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2084 = (t2062).items()?;
        if t2084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2084[0].clone();
        v_v = t2084[1].clone();
        if ({ let t2063 = v_k.clone(); Py::Bool({ let t2064 = v_outputs.clone(); if !py::contains(&t2064, &t2063)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2103(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2105: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2104 = v_inputs.clone(); f_kd_from_rates(py::getitem(&t2104, &Py::str("kon"))?, py::getitem(&t2104, &Py::str("koff"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2105 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2106 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2128 = (t2106).items()?;
        if t2128.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2128[0].clone();
        v_v = t2128[1].clone();
        if ({ let t2107 = v_k.clone(); Py::Bool({ let t2108 = v_outputs.clone(); if !py::contains(&t2108, &t2107)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2158(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2160: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2159 = v_inputs.clone(); f_library_size(py::getitem(&t2159, &Py::str("G"))?, py::getitem(&t2159, &Py::str("I"))?, py::getitem(&t2159, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2160 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2161 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2183 = (t2161).items()?;
        if t2183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2183[0].clone();
        v_v = t2183[1].clone();
        if ({ let t2162 = v_k.clone(); Py::Bool({ let t2163 = v_outputs.clone(); if !py::contains(&t2163, &t2162)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2202(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2204: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2203 = v_inputs.clone(); f_ligand_depletion_binding(py::getitem(&t2203, &Py::str("Rt"))?, py::getitem(&t2203, &Py::str("Lt"))?, py::getitem(&t2203, &Py::str("Kd"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2204 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2205 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2227 = (t2205).items()?;
        if t2227.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2227[0].clone();
        v_v = t2227[1].clone();
        if ({ let t2206 = v_k.clone(); Py::Bool({ let t2207 = v_outputs.clone(); if !py::contains(&t2207, &t2206)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2253(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2255: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2254 = v_inputs.clone(); f_ligation_insert_mass(py::getitem(&t2254, &Py::str("m_vector"))?, py::getitem(&t2254, &Py::str("L_vector"))?, py::getitem(&t2254, &Py::str("L_insert"))?, py::getitem(&t2254, &Py::str("ratio"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2255 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2256 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2278 = (t2256).items()?;
        if t2278.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2278[0].clone();
        v_v = t2278[1].clone();
        if ({ let t2257 = v_k.clone(); Py::Bool({ let t2258 = v_outputs.clone(); if !py::contains(&t2258, &t2257)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2323(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2325: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2324 = v_inputs.clone(); f_lineweaver_burk(py::getitem(&t2324, &Py::str("S"))?, py::getitem(&t2324, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2325 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2326 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2348 = (t2326).items()?;
        if t2348.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2348[0].clone();
        v_v = t2348[1].clone();
        if ({ let t2327 = v_k.clone(); Py::Bool({ let t2328 = v_outputs.clone(); if !py::contains(&t2328, &t2327)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2373(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2375: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2374 = v_inputs.clone(); f_logistic_growth(py::getitem(&t2374, &Py::str("N0"))?, py::getitem(&t2374, &Py::str("K"))?, py::getitem(&t2374, &Py::str("r"))?, py::getitem(&t2374, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2375 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2376 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2398 = (t2376).items()?;
        if t2398.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2398[0].clone();
        v_v = t2398[1].clone();
        if ({ let t2377 = v_k.clone(); Py::Bool({ let t2378 = v_outputs.clone(); if !py::contains(&t2378, &t2377)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2420(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2422: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2421 = v_inputs.clone(); f_logistic_time_to_reach(py::getitem(&t2421, &Py::str("N0"))?, py::getitem(&t2421, &Py::str("K"))?, py::getitem(&t2421, &Py::str("r"))?, py::getitem(&t2421, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2422 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2423 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2445 = (t2423).items()?;
        if t2445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2445[0].clone();
        v_v = t2445[1].clone();
        if ({ let t2424 = v_k.clone(); Py::Bool({ let t2425 = v_outputs.clone(); if !py::contains(&t2425, &t2424)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_244(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t246: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t245 = v_inputs.clone(); f_buffer_capacity(py::getitem(&t245, &Py::str("C"))?, py::getitem(&t245, &Py::str("pKa"))?, py::getitem(&t245, &Py::str("pH"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t246 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t247 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t269 = (t247).items()?;
        if t269.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t269[0].clone();
        v_v = t269[1].clone();
        if ({ let t248 = v_k.clone(); Py::Bool({ let t249 = v_outputs.clone(); if !py::contains(&t249, &t248)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2464(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2466: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2465 = v_inputs.clone(); f_mass_for_molarity(py::getitem(&t2465, &Py::str("C"))?, py::getitem(&t2465, &Py::str("V"))?, py::getitem(&t2465, &Py::str("MW"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2466 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2467 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2489 = (t2467).items()?;
        if t2489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2489[0].clone();
        v_v = t2489[1].clone();
        if ({ let t2468 = v_k.clone(); Py::Bool({ let t2469 = v_outputs.clone(); if !py::contains(&t2469, &t2468)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2512(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2514: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2513 = v_inputs.clone(); f_membrane_permeability(py::getitem(&t2513, &Py::str("D"))?, py::getitem(&t2513, &Py::str("K"))?, py::getitem(&t2513, &Py::str("L"))?, py::getitem(&t2513, &Py::str("C_out"))?, py::getitem(&t2513, &Py::str("C_in"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2514 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2515 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2537 = (t2515).items()?;
        if t2537.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2537[0].clone();
        v_v = t2537[1].clone();
        if ({ let t2516 = v_k.clone(); Py::Bool({ let t2517 = v_outputs.clone(); if !py::contains(&t2517, &t2516)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2616(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2618: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2617 = v_inputs.clone(); f_mm_nonlinear_fit(py::getitem(&t2617, &Py::str("S"))?, py::getitem(&t2617, &Py::str("v"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2618 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2619 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2641 = (t2619).items()?;
        if t2641.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2641[0].clone();
        v_v = t2641[1].clone();
        if ({ let t2620 = v_k.clone(); Py::Bool({ let t2621 = v_outputs.clone(); if !py::contains(&t2621, &t2620)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2664(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2666: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2665 = v_inputs.clone(); f_michaelis_menten(py::getitem(&t2665, &Py::str("Vmax"))?, py::getitem(&t2665, &Py::str("Km"))?, py::getitem(&t2665, &Py::str("S"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2666 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2667 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2689 = (t2667).items()?;
        if t2689.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2689[0].clone();
        v_v = t2689[1].clone();
        if ({ let t2668 = v_k.clone(); Py::Bool({ let t2669 = v_outputs.clone(); if !py::contains(&t2669, &t2668)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2710(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2712: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2711 = v_inputs.clone(); f_microscope_resolution(py::getitem(&t2711, &Py::str("M_obj"))?, py::getitem(&t2711, &Py::str("M_oc"))?, py::getitem(&t2711, &Py::str("wavelength"))?, py::getitem(&t2711, &Py::str("NA"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2712 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2713 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2735 = (t2713).items()?;
        if t2735.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2735[0].clone();
        v_v = t2735[1].clone();
        if ({ let t2714 = v_k.clone(); Py::Bool({ let t2715 = v_outputs.clone(); if !py::contains(&t2715, &t2714)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2759(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2761: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2760 = v_inputs.clone(); f_mixed_inhibition(py::getitem(&t2760, &Py::str("Vmax"))?, py::getitem(&t2760, &Py::str("Km"))?, py::getitem(&t2760, &Py::str("S"))?, py::getitem(&t2760, &Py::str("I"))?, py::getitem(&t2760, &Py::str("Ki"))?, py::getitem(&t2760, &Py::str("Kii"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2761 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2762 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2784 = (t2762).items()?;
        if t2784.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2784[0].clone();
        v_v = t2784[1].clone();
        if ({ let t2763 = v_k.clone(); Py::Bool({ let t2764 = v_outputs.clone(); if !py::contains(&t2764, &t2763)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2803(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2805: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2804 = v_inputs.clone(); f_molarity_from_mass(py::getitem(&t2804, &Py::str("m"))?, py::getitem(&t2804, &Py::str("MW"))?, py::getitem(&t2804, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2805 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2806 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2828 = (t2806).items()?;
        if t2828.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2828[0].clone();
        v_v = t2828[1].clone();
        if ({ let t2807 = v_k.clone(); Py::Bool({ let t2808 = v_outputs.clone(); if !py::contains(&t2808, &t2807)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2847(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2849: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2848 = v_inputs.clone(); f_molecules_per_cell(py::getitem(&t2848, &Py::str("c"))?, py::getitem(&t2848, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2849 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2850 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2872 = (t2850).items()?;
        if t2872.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2872[0].clone();
        v_v = t2872[1].clone();
        if ({ let t2851 = v_k.clone(); Py::Bool({ let t2852 = v_outputs.clone(); if !py::contains(&t2852, &t2851)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2894(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2896: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2895 = v_inputs.clone(); f_monod_batch_time(py::getitem(&t2895, &Py::str("X0"))?, py::getitem(&t2895, &Py::str("S0"))?, py::getitem(&t2895, &Py::str("Y"))?, py::getitem(&t2895, &Py::str("mu_max"))?, py::getitem(&t2895, &Py::str("Ks"))?, py::getitem(&t2895, &Py::str("X"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2896 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2897 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2919 = (t2897).items()?;
        if t2919.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2919[0].clone();
        v_v = t2919[1].clone();
        if ({ let t2898 = v_k.clone(); Py::Bool({ let t2899 = v_outputs.clone(); if !py::contains(&t2899, &t2898)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_294(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t296: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t295 = v_inputs.clone(); f_buffer_preparation(py::getitem(&t295, &Py::str("C"))?, py::getitem(&t295, &Py::str("V"))?, py::getitem(&t295, &Py::str("pH"))?, py::getitem(&t295, &Py::str("pKa"))?, py::method(&t295, "get", &[Py::str("MW_acid"), Py::None])?, py::method(&t295, "get", &[Py::str("MW_base"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t296 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t297 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t319 = (t297).items()?;
        if t319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t319[0].clone();
        v_v = t319[1].clone();
        if ({ let t298 = v_k.clone(); Py::Bool({ let t299 = v_outputs.clone(); if !py::contains(&t299, &t298)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2945(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2947: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2946 = v_inputs.clone(); f_monod_growth(py::getitem(&t2946, &Py::str("mu_max"))?, py::getitem(&t2946, &Py::str("S"))?, py::getitem(&t2946, &Py::str("Ks"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2947 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2948 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2970 = (t2948).items()?;
        if t2970.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2970[0].clone();
        v_v = t2970[1].clone();
        if ({ let t2949 = v_k.clone(); Py::Bool({ let t2950 = v_outputs.clone(); if !py::contains(&t2950, &t2949)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3007(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3009: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3008 = v_inputs.clone(); f_mrna_half_life_fit(py::getitem(&t3008, &Py::str("t"))?, py::getitem(&t3008, &Py::str("level"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3009 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3010 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3032 = (t3010).items()?;
        if t3032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3032[0].clone();
        v_v = t3032[1].clone();
        if ({ let t3011 = v_k.clone(); Py::Bool({ let t3012 = v_outputs.clone(); if !py::contains(&t3012, &t3011)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3059(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3061: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3060 = v_inputs.clone(); f_mutation_rate_p0(py::getitem(&t3060, &Py::str("n0"))?, py::getitem(&t3060, &Py::str("n"))?, py::getitem(&t3060, &Py::str("Nt"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3061 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3062 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3084 = (t3062).items()?;
        if t3084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3084[0].clone();
        v_v = t3084[1].clone();
        if ({ let t3063 = v_k.clone(); Py::Bool({ let t3064 = v_outputs.clone(); if !py::contains(&t3064, &t3063)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3107(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3109: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3108 = v_inputs.clone(); f_mwc_saturation(py::getitem(&t3108, &Py::str("S"))?, py::getitem(&t3108, &Py::str("KR"))?, py::getitem(&t3108, &Py::str("c"))?, py::getitem(&t3108, &Py::str("L0"))?, py::getitem(&t3108, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3109 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3110 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3132 = (t3110).items()?;
        if t3132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3132[0].clone();
        v_v = t3132[1].clone();
        if ({ let t3111 = v_k.clone(); Py::Bool({ let t3112 = v_outputs.clone(); if !py::contains(&t3112, &t3111)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3155(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3157: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3156 = v_inputs.clone(); f_nernst_potential(py::getitem(&t3156, &Py::str("z"))?, py::getitem(&t3156, &Py::str("C_out"))?, py::getitem(&t3156, &Py::str("C_in"))?, py::getitem(&t3156, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3157 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3158 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3180 = (t3158).items()?;
        if t3180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3180[0].clone();
        v_v = t3180[1].clone();
        if ({ let t3159 = v_k.clone(); Py::Bool({ let t3160 = v_outputs.clone(); if !py::contains(&t3160, &t3159)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3181(mut v_Vmax: Py, mut v_Km: Py, mut v_S: Py, mut v_I: Py, mut v_Ki: Py) -> R {
    return Ok(f__inhibited(v_Vmax.clone(), v_Km.clone(), v_S.clone(), v_I.clone(), v_Ki.clone(), Py::str("noncompetitive"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3213(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3215: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3214 = v_inputs.clone(); s_3181(py::getitem(&t3214, &Py::str("Vmax"))?, py::getitem(&t3214, &Py::str("Km"))?, py::getitem(&t3214, &Py::str("S"))?, py::getitem(&t3214, &Py::str("I"))?, py::getitem(&t3214, &Py::str("Ki"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3215 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3216 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3238 = (t3216).items()?;
        if t3238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3238[0].clone();
        v_v = t3238[1].clone();
        if ({ let t3217 = v_k.clone(); Py::Bool({ let t3218 = v_outputs.clone(); if !py::contains(&t3218, &t3217)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3269(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3271: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3270 = v_inputs.clone(); f_nucleic_acid_a260(py::getitem(&t3270, &Py::str("A260"))?, py::getitem(&t3270, &Py::str("molecule"))?, py::method(&t3270, "get", &[Py::str("dilution"), Py::Float(1.0)])?, py::method(&t3270, "get", &[Py::str("l"), Py::Float(0.01)])?, py::method(&t3270, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3271 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3272 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3294 = (t3272).items()?;
        if t3294.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3294[0].clone();
        v_v = t3294[1].clone();
        if ({ let t3273 = v_k.clone(); Py::Bool({ let t3274 = v_outputs.clone(); if !py::contains(&t3274, &t3273)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3328(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3330: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3329 = v_inputs.clone(); f_oligo_extinction(py::getitem(&t3329, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3330 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3331 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3353 = (t3331).items()?;
        if t3353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3353[0].clone();
        v_v = t3353[1].clone();
        if ({ let t3332 = v_k.clone(); Py::Bool({ let t3333 = v_outputs.clone(); if !py::contains(&t3333, &t3332)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3388(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3390: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3389 = v_inputs.clone(); f_oligo_molecular_weight(py::getitem(&t3389, &Py::str("seq"))?, py::getitem(&t3389, &Py::str("strands"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3390 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3391 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3413 = (t3391).items()?;
        if t3413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3413[0].clone();
        v_v = t3413[1].clone();
        if ({ let t3392 = v_k.clone(); Py::Bool({ let t3393 = v_outputs.clone(); if !py::contains(&t3393, &t3392)? { true } else { false } }) }).truthy() {
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
fn s_340(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t342: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t341 = v_inputs.clone(); f_c1v1_dilution(py::getitem(&t341, &Py::str("C1"))?, py::getitem(&t341, &Py::str("C2"))?, py::getitem(&t341, &Py::str("V2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t342 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t343 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t365 = (t343).items()?;
        if t365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t365[0].clone();
        v_v = t365[1].clone();
        if ({ let t344 = v_k.clone(); Py::Bool({ let t345 = v_outputs.clone(); if !py::contains(&t345, &t344)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3478(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3480: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3479 = v_inputs.clone(); f_orf_finder(py::getitem(&t3479, &Py::str("seq"))?, py::getitem(&t3479, &Py::str("min_aa"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3480 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3481 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3503 = (t3481).items()?;
        if t3503.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3503[0].clone();
        v_v = t3503[1].clone();
        if ({ let t3482 = v_k.clone(); Py::Bool({ let t3483 = v_outputs.clone(); if !py::contains(&t3483, &t3482)? { true } else { false } }) }).truthy() {
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
            v_want = { let t3541 = v_inputs.clone(); f_osmolarity_tonicity(py::getitem(&t3541, &Py::str("c"))?, py::getitem(&t3541, &Py::str("i"))?, py::getitem(&t3541, &Py::str("osm_cell"))?)? };
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
fn s_3584(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3586: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3585 = v_inputs.clone(); f_osmotic_pressure(py::getitem(&t3585, &Py::str("i"))?, py::getitem(&t3585, &Py::str("c"))?, py::getitem(&t3585, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3586 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3587 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3609 = (t3587).items()?;
        if t3609.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3609[0].clone();
        v_v = t3609[1].clone();
        if ({ let t3588 = v_k.clone(); Py::Bool({ let t3589 = v_outputs.clone(); if !py::contains(&t3589, &t3588)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3641(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3643: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3642 = v_inputs.clone(); f_pcr_amplification(py::getitem(&t3642, &Py::str("N0"))?, py::getitem(&t3642, &Py::str("E"))?, py::getitem(&t3642, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3643 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3644 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3666 = (t3644).items()?;
        if t3666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3666[0].clone();
        v_v = t3666[1].clone();
        if ({ let t3645 = v_k.clone(); Py::Bool({ let t3646 = v_outputs.clone(); if !py::contains(&t3646, &t3645)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3698(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3700: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3699 = v_inputs.clone(); f_pcr_cycles_needed(py::getitem(&t3699, &Py::str("N0"))?, py::getitem(&t3699, &Py::str("N"))?, py::getitem(&t3699, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3700 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3701 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3723 = (t3701).items()?;
        if t3723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3723[0].clone();
        v_v = t3723[1].clone();
        if ({ let t3702 = v_k.clone(); Py::Bool({ let t3703 = v_outputs.clone(); if !py::contains(&t3703, &t3702)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3751(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3753: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3752 = v_inputs.clone(); f_percent_to_molarity(py::getitem(&t3752, &Py::str("p"))?, py::getitem(&t3752, &Py::str("MW"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3753 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3754 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3776 = (t3754).items()?;
        if t3776.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3776[0].clone();
        v_v = t3776[1].clone();
        if ({ let t3755 = v_k.clone(); Py::Bool({ let t3756 = v_outputs.clone(); if !py::contains(&t3756, &t3755)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3803(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3805: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3804 = v_inputs.clone(); f_cfu_count(py::getitem(&t3804, &Py::str("colonies"))?, py::getitem(&t3804, &Py::str("dilution"))?, py::getitem(&t3804, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3805 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3806 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3828 = (t3806).items()?;
        if t3828.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3828[0].clone();
        v_v = t3828[1].clone();
        if ({ let t3807 = v_k.clone(); Py::Bool({ let t3808 = v_outputs.clone(); if !py::contains(&t3808, &t3807)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3854(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3856: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3855 = v_inputs.clone(); f_protein_a280(py::getitem(&t3855, &Py::str("A280"))?, py::getitem(&t3855, &Py::str("eps"))?, py::method(&t3855, "get", &[Py::str("MW"), Py::None])?, py::method(&t3855, "get", &[Py::str("l"), Py::Float(0.01)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3856 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3857 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3879 = (t3857).items()?;
        if t3879.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3879[0].clone();
        v_v = t3879[1].clone();
        if ({ let t3858 = v_k.clone(); Py::Bool({ let t3859 = v_outputs.clone(); if !py::contains(&t3859, &t3858)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3900(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3902: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3901 = v_inputs.clone(); f_q10_rate(py::getitem(&t3901, &Py::str("k1"))?, py::getitem(&t3901, &Py::str("T1"))?, py::getitem(&t3901, &Py::str("T2"))?, py::getitem(&t3901, &Py::str("Q10"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3902 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3903 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3925 = (t3903).items()?;
        if t3925.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3925[0].clone();
        v_v = t3925[1].clone();
        if ({ let t3904 = v_k.clone(); Py::Bool({ let t3905 = v_outputs.clone(); if !py::contains(&t3905, &t3904)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3945(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3947: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3946 = v_inputs.clone(); f_qpcr_ddct(py::getitem(&t3946, &Py::str("Ct_target_treated"))?, py::getitem(&t3946, &Py::str("Ct_ref_treated"))?, py::getitem(&t3946, &Py::str("Ct_target_control"))?, py::getitem(&t3946, &Py::str("Ct_ref_control"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3947 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3948 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3970 = (t3948).items()?;
        if t3970.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3970[0].clone();
        v_v = t3970[1].clone();
        if ({ let t3949 = v_k.clone(); Py::Bool({ let t3950 = v_outputs.clone(); if !py::contains(&t3950, &t3949)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_396(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t398: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t397 = v_inputs.clone(); f_cell_cycle_age_corrected(py::getitem(&t397, &Py::str("Tc"))?, py::getitem(&t397, &Py::str("fractions"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t398 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t399 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t421 = (t399).items()?;
        if t421.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t421[0].clone();
        v_v = t421[1].clone();
        if ({ let t400 = v_k.clone(); Py::Bool({ let t401 = v_outputs.clone(); if !py::contains(&t401, &t400)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3999(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4001: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4000 = v_inputs.clone(); f_qpcr_pfaffl(py::getitem(&t4000, &Py::str("E_target"))?, py::getitem(&t4000, &Py::str("E_ref"))?, py::getitem(&t4000, &Py::str("Ct_target_control"))?, py::getitem(&t4000, &Py::str("Ct_target_sample"))?, py::getitem(&t4000, &Py::str("Ct_ref_control"))?, py::getitem(&t4000, &Py::str("Ct_ref_sample"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4001 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4002 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4024 = (t4002).items()?;
        if t4024.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4024[0].clone();
        v_v = t4024[1].clone();
        if ({ let t4003 = v_k.clone(); Py::Bool({ let t4004 = v_outputs.clone(); if !py::contains(&t4004, &t4003)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4043(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4045: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4044 = v_inputs.clone(); f_qpcr_quantify(py::getitem(&t4044, &Py::str("slope"))?, py::getitem(&t4044, &Py::str("intercept"))?, py::getitem(&t4044, &Py::str("Ct"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4045 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4046 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4068 = (t4046).items()?;
        if t4068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4068[0].clone();
        v_v = t4068[1].clone();
        if ({ let t4047 = v_k.clone(); Py::Bool({ let t4048 = v_outputs.clone(); if !py::contains(&t4048, &t4047)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4105(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4107: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4106 = v_inputs.clone(); f_qpcr_standard_curve(py::getitem(&t4106, &Py::str("quantity"))?, py::getitem(&t4106, &Py::str("Ct"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4107 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4108 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4130 = (t4108).items()?;
        if t4130.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4130[0].clone();
        v_v = t4130[1].clone();
        if ({ let t4109 = v_k.clone(); Py::Bool({ let t4110 = v_outputs.clone(); if !py::contains(&t4110, &t4109)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4177(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4179: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4178 = v_inputs.clone(); f_reaction_free_energy(py::getitem(&t4178, &Py::str("dG0"))?, py::getitem(&t4178, &Py::str("T"))?, py::getitem(&t4178, &Py::str("products"))?, py::getitem(&t4178, &Py::str("reactants"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4179 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4180 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4202 = (t4180).items()?;
        if t4202.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4202[0].clone();
        v_v = t4202[1].clone();
        if ({ let t4181 = v_k.clone(); Py::Bool({ let t4182 = v_outputs.clone(); if !py::contains(&t4182, &t4181)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4231(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4233: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4232 = v_inputs.clone(); f_redox_free_energy(py::getitem(&t4232, &Py::str("E_acceptor"))?, py::getitem(&t4232, &Py::str("E_donor"))?, py::getitem(&t4232, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4233 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4234 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4256 = (t4234).items()?;
        if t4256.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4256[0].clone();
        v_v = t4256[1].clone();
        if ({ let t4235 = v_k.clone(); Py::Bool({ let t4236 = v_outputs.clone(); if !py::contains(&t4236, &t4235)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4288(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4290: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4289 = v_inputs.clone(); f_reverse_complement(py::getitem(&t4289, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4290 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4291 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4313 = (t4291).items()?;
        if t4313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4313[0].clone();
        v_v = t4313[1].clone();
        if ({ let t4292 = v_k.clone(); Py::Bool({ let t4293 = v_outputs.clone(); if !py::contains(&t4293, &t4292)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4334(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4336: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4335 = v_inputs.clone(); f_rms_displacement(py::getitem(&t4335, &Py::str("D"))?, py::getitem(&t4335, &Py::str("t"))?, py::getitem(&t4335, &Py::str("dims"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4336 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4337 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4359 = (t4337).items()?;
        if t4359.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4359[0].clone();
        v_v = t4359[1].clone();
        if ({ let t4338 = v_k.clone(); Py::Bool({ let t4339 = v_outputs.clone(); if !py::contains(&t4339, &t4338)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4400(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4402: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4401 = v_inputs.clone(); f_scatchard_fit(py::getitem(&t4401, &Py::str("B"))?, py::getitem(&t4401, &Py::str("F"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4402 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4403 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4425 = (t4403).items()?;
        if t4425.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4425[0].clone();
        v_v = t4425[1].clone();
        if ({ let t4404 = v_k.clone(); Py::Bool({ let t4405 = v_outputs.clone(); if !py::contains(&t4405, &t4404)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4446(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4448: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4447 = v_inputs.clone(); f_sedimentation_time(py::getitem(&t4447, &Py::str("s"))?, py::getitem(&t4447, &Py::str("rpm"))?, py::getitem(&t4447, &Py::str("r_min"))?, py::getitem(&t4447, &Py::str("r_max"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4448 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4449 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4471 = (t4449).items()?;
        if t4471.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4471[0].clone();
        v_v = t4471[1].clone();
        if ({ let t4450 = v_k.clone(); Py::Bool({ let t4451 = v_outputs.clone(); if !py::contains(&t4451, &t4450)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_449(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t451: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t450 = v_inputs.clone(); f_cell_cycle_simple(py::getitem(&t450, &Py::str("Tc"))?, py::getitem(&t450, &Py::str("fractions"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t451 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t452 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t474 = (t452).items()?;
        if t474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t474[0].clone();
        v_v = t474[1].clone();
        if ({ let t453 = v_k.clone(); Py::Bool({ let t454 = v_outputs.clone(); if !py::contains(&t454, &t453)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4501(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4503: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4502 = v_inputs.clone(); f_serial_dilution(py::getitem(&t4502, &Py::str("C0"))?, py::getitem(&t4502, &Py::str("f"))?, py::getitem(&t4502, &Py::str("n"))?, py::method(&t4502, "get", &[Py::str("V"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4503 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4504 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4526 = (t4504).items()?;
        if t4526.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4526[0].clone();
        v_v = t4526[1].clone();
        if ({ let t4505 = v_k.clone(); Py::Bool({ let t4506 = v_outputs.clone(); if !py::contains(&t4506, &t4505)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4560(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4562: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4561 = v_inputs.clone(); f_specimen_size(py::getitem(&t4561, &Py::str("image_size"))?, py::method(&t4561, "get", &[Py::str("M"), Py::None])?, py::method(&t4561, "get", &[Py::str("bar_image"), Py::None])?, py::method(&t4561, "get", &[Py::str("bar_label"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4562 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4563 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4585 = (t4563).items()?;
        if t4585.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4585[0].clone();
        v_v = t4585[1].clone();
        if ({ let t4564 = v_k.clone(); Py::Bool({ let t4565 = v_outputs.clone(); if !py::contains(&t4565, &t4564)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4604(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4606: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4605 = v_inputs.clone(); f_stokes_einstein(py::getitem(&t4605, &Py::str("T"))?, py::getitem(&t4605, &Py::str("eta"))?, py::getitem(&t4605, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4606 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4607 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4629 = (t4607).items()?;
        if t4629.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4629[0].clone();
        v_v = t4629[1].clone();
        if ({ let t4608 = v_k.clone(); Py::Bool({ let t4609 = v_outputs.clone(); if !py::contains(&t4609, &t4608)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4657(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4659: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4658 = v_inputs.clone(); f_substrate_for_fraction(py::getitem(&t4658, &Py::str("Km"))?, py::getitem(&t4658, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4659 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4660 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4682 = (t4660).items()?;
        if t4682.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4682[0].clone();
        v_v = t4682[1].clone();
        if ({ let t4661 = v_k.clone(); Py::Bool({ let t4662 = v_outputs.clone(); if !py::contains(&t4662, &t4661)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4703(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4705: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4704 = v_inputs.clone(); f_thermal_death_time(py::getitem(&t4704, &Py::str("D_ref"))?, py::getitem(&t4704, &Py::str("T_ref"))?, py::getitem(&t4704, &Py::str("z"))?, py::getitem(&t4704, &Py::str("T"))?, py::getitem(&t4704, &Py::str("log_reductions"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4705 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4706 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4728 = (t4706).items()?;
        if t4728.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4728[0].clone();
        v_v = t4728[1].clone();
        if ({ let t4707 = v_k.clone(); Py::Bool({ let t4708 = v_outputs.clone(); if !py::contains(&t4708, &t4707)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4759(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4761: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4760 = v_inputs.clone(); f_tm_basic(py::getitem(&t4760, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4761 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4762 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4784 = (t4762).items()?;
        if t4784.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4784[0].clone();
        v_v = t4784[1].clone();
        if ({ let t4763 = v_k.clone(); Py::Bool({ let t4764 = v_outputs.clone(); if !py::contains(&t4764, &t4763)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4836(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4838: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4837 = v_inputs.clone(); f_tm_nearest_neighbor(py::getitem(&t4837, &Py::str("seq"))?, py::getitem(&t4837, &Py::str("Na"))?, py::getitem(&t4837, &Py::str("c1"))?, py::getitem(&t4837, &Py::str("c2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4838 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4839 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4861 = (t4839).items()?;
        if t4861.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4861[0].clone();
        v_v = t4861[1].clone();
        if ({ let t4840 = v_k.clone(); Py::Bool({ let t4841 = v_outputs.clone(); if !py::contains(&t4841, &t4840)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4898(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4900: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4899 = v_inputs.clone(); f_tm_salt_adjusted(py::getitem(&t4899, &Py::str("seq"))?, py::getitem(&t4899, &Py::str("Na"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4900 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4901 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4923 = (t4901).items()?;
        if t4923.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4923[0].clone();
        v_v = t4923[1].clone();
        if ({ let t4902 = v_k.clone(); Py::Bool({ let t4903 = v_outputs.clone(); if !py::contains(&t4903, &t4902)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4955(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4957: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4956 = v_inputs.clone(); f_tm_wallace(py::getitem(&t4956, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4957 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4958 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4980 = (t4958).items()?;
        if t4980.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4980[0].clone();
        v_v = t4980[1].clone();
        if ({ let t4959 = v_k.clone(); Py::Bool({ let t4960 = v_outputs.clone(); if !py::contains(&t4960, &t4959)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_497(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t499: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t498 = v_inputs.clone(); f_cell_equilibration(py::getitem(&t498, &Py::str("r"))?, py::getitem(&t498, &Py::str("P"))?, py::getitem(&t498, &Py::str("C_out"))?, py::getitem(&t498, &Py::str("C0"))?, py::getitem(&t498, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t499 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t500 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t522 = (t500).items()?;
        if t522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t522[0].clone();
        v_v = t522[1].clone();
        if ({ let t501 = v_k.clone(); Py::Bool({ let t502 = v_outputs.clone(); if !py::contains(&t502, &t501)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5017(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5019: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5018 = v_inputs.clone(); f_transcribe(py::getitem(&t5018, &Py::str("seq"))?, py::getitem(&t5018, &Py::str("strand"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5019 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5020 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5042 = (t5020).items()?;
        if t5042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5042[0].clone();
        v_v = t5042[1].clone();
        if ({ let t5021 = v_k.clone(); Py::Bool({ let t5022 = v_outputs.clone(); if !py::contains(&t5022, &t5021)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5067(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5069: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5068 = v_inputs.clone(); f_transcription_time(py::getitem(&t5068, &Py::str("n"))?, py::getitem(&t5068, &Py::str("rate"))?, py::method(&t5068, "get", &[Py::str("k_init"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5069 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5070 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5092 = (t5070).items()?;
        if t5092.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5092[0].clone();
        v_v = t5092[1].clone();
        if ({ let t5071 = v_k.clone(); Py::Bool({ let t5072 = v_outputs.clone(); if !py::contains(&t5072, &t5071)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5118(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5120: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5119 = v_inputs.clone(); f_transformation_efficiency(py::getitem(&t5119, &Py::str("colonies"))?, py::getitem(&t5119, &Py::str("m_DNA"))?, py::getitem(&t5119, &Py::str("V_plated"))?, py::getitem(&t5119, &Py::str("V_total"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5120 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5121 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5143 = (t5121).items()?;
        if t5143.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5143[0].clone();
        v_v = t5143[1].clone();
        if ({ let t5122 = v_k.clone(); Py::Bool({ let t5123 = v_outputs.clone(); if !py::contains(&t5123, &t5122)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5177(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5179: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5178 = v_inputs.clone(); f_translate(py::getitem(&t5178, &Py::str("seq"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5179 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5180 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5202 = (t5180).items()?;
        if t5202.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5202[0].clone();
        v_v = t5202[1].clone();
        if ({ let t5181 = v_k.clone(); Py::Bool({ let t5182 = v_outputs.clone(); if !py::contains(&t5182, &t5181)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5227(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5229: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5228 = v_inputs.clone(); f_translation_time(py::getitem(&t5228, &Py::str("n_aa"))?, py::getitem(&t5228, &Py::str("rate"))?, py::method(&t5228, "get", &[Py::str("k_init"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5229 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5230 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5252 = (t5230).items()?;
        if t5252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5252[0].clone();
        v_v = t5252[1].clone();
        if ({ let t5231 = v_k.clone(); Py::Bool({ let t5232 = v_outputs.clone(); if !py::contains(&t5232, &t5231)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5253(mut v_Vmax: Py, mut v_Km: Py, mut v_S: Py, mut v_I: Py, mut v_Ki: Py) -> R {
    return Ok(f__inhibited(v_Vmax.clone(), v_Km.clone(), v_S.clone(), v_I.clone(), v_Ki.clone(), Py::str("uncompetitive"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5285(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5287: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5286 = v_inputs.clone(); s_5253(py::getitem(&t5286, &Py::str("Vmax"))?, py::getitem(&t5286, &Py::str("Km"))?, py::getitem(&t5286, &Py::str("S"))?, py::getitem(&t5286, &Py::str("I"))?, py::getitem(&t5286, &Py::str("Ki"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5287 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5288 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5310 = (t5288).items()?;
        if t5310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5310[0].clone();
        v_v = t5310[1].clone();
        if ({ let t5289 = v_k.clone(); Py::Bool({ let t5290 = v_outputs.clone(); if !py::contains(&t5290, &t5289)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5333(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5335: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5334 = v_inputs.clone(); f_warburg_christian(py::getitem(&t5334, &Py::str("A280"))?, py::getitem(&t5334, &Py::str("A260"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5335 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5336 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5358 = (t5336).items()?;
        if t5358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5358[0].clone();
        v_v = t5358[1].clone();
        if ({ let t5337 = v_k.clone(); Py::Bool({ let t5338 = v_outputs.clone(); if !py::contains(&t5338, &t5337)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5388(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5390: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5389 = v_inputs.clone(); f_water_movement(py::getitem(&t5389, &Py::str("i_cell"))?, py::getitem(&t5389, &Py::str("C_cell"))?, py::getitem(&t5389, &Py::str("P_cell"))?, py::getitem(&t5389, &Py::str("i_out"))?, py::getitem(&t5389, &Py::str("C_out"))?, py::getitem(&t5389, &Py::str("P_out"))?, py::getitem(&t5389, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5390 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5391 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5413 = (t5391).items()?;
        if t5413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5413[0].clone();
        v_v = t5413[1].clone();
        if ({ let t5392 = v_k.clone(); Py::Bool({ let t5393 = v_outputs.clone(); if !py::contains(&t5393, &t5392)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_541(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t543: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t542 = v_inputs.clone(); f_cell_geometry(py::getitem(&t542, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t543 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t544 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t566 = (t544).items()?;
        if t566.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t566[0].clone();
        v_v = t566[1].clone();
        if ({ let t545 = v_k.clone(); Py::Bool({ let t546 = v_outputs.clone(); if !py::contains(&t546, &t545)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5437(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5439: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5438 = v_inputs.clone(); f_water_potential(py::getitem(&t5438, &Py::str("i"))?, py::getitem(&t5438, &Py::str("C"))?, py::getitem(&t5438, &Py::str("T"))?, py::getitem(&t5438, &Py::str("psi_p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5439 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5440 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5462 = (t5440).items()?;
        if t5462.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5462[0].clone();
        v_v = t5462[1].clone();
        if ({ let t5441 = v_k.clone(); Py::Bool({ let t5442 = v_outputs.clone(); if !py::contains(&t5442, &t5441)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5488(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5490: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5489 = v_inputs.clone(); f_z_value(py::getitem(&t5489, &Py::str("D1"))?, py::getitem(&t5489, &Py::str("T1"))?, py::getitem(&t5489, &Py::str("D2"))?, py::getitem(&t5489, &Py::str("T2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5490 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5491 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5513 = (t5491).items()?;
        if t5513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5513[0].clone();
        v_v = t5513[1].clone();
        if ({ let t5492 = v_k.clone(); Py::Bool({ let t5493 = v_outputs.clone(); if !py::contains(&t5493, &t5492)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_594(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t596: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t595 = v_inputs.clone(); f_cell_volume_osmotic(py::getitem(&t595, &Py::str("V0"))?, py::getitem(&t595, &Py::str("b"))?, py::getitem(&t595, &Py::str("osm0"))?, py::getitem(&t595, &Py::str("osm1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t596 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t597 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t619 = (t597).items()?;
        if t619.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t619[0].clone();
        v_v = t619[1].clone();
        if ({ let t598 = v_k.clone(); Py::Bool({ let t599 = v_outputs.clone(); if !py::contains(&t599, &t598)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_638(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t640: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t639 = v_inputs.clone(); f_centrifuge_rcf(py::getitem(&t639, &Py::str("rpm"))?, py::getitem(&t639, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t640 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t641 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t663 = (t641).items()?;
        if t663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t663[0].clone();
        v_v = t663[1].clone();
        if ({ let t642 = v_k.clone(); Py::Bool({ let t643 = v_outputs.clone(); if !py::contains(&t643, &t642)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_682(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t684: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t683 = v_inputs.clone(); f_centrifuge_rpm(py::getitem(&t683, &Py::str("RCF"))?, py::getitem(&t683, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t684 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t685 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t707 = (t685).items()?;
        if t707.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t707[0].clone();
        v_v = t707[1].clone();
        if ({ let t686 = v_k.clone(); Py::Bool({ let t687 = v_outputs.clone(); if !py::contains(&t687, &t686)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_739(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t741: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t740 = v_inputs.clone(); f_channel_current(py::getitem(&t740, &Py::str("N"))?, py::getitem(&t740, &Py::str("Po"))?, py::getitem(&t740, &Py::str("gamma"))?, py::getitem(&t740, &Py::str("V"))?, py::getitem(&t740, &Py::str("E"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t741 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t742 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t764 = (t742).items()?;
        if t764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t764[0].clone();
        v_v = t764[1].clone();
        if ({ let t743 = v_k.clone(); Py::Bool({ let t744 = v_outputs.clone(); if !py::contains(&t744, &t743)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_765(mut v_Vmax: Py, mut v_Km: Py, mut v_S: Py, mut v_I: Py, mut v_Ki: Py) -> R {
    return Ok(f__inhibited(v_Vmax.clone(), v_Km.clone(), v_S.clone(), v_I.clone(), v_Ki.clone(), Py::str("competitive"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_797(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t799: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t798 = v_inputs.clone(); s_765(py::getitem(&t798, &Py::str("Vmax"))?, py::getitem(&t798, &Py::str("Km"))?, py::getitem(&t798, &Py::str("S"))?, py::getitem(&t798, &Py::str("I"))?, py::getitem(&t798, &Py::str("Ki"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t799 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t800 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t822 = (t800).items()?;
        if t822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t822[0].clone();
        v_v = t822[1].clone();
        if ({ let t801 = v_k.clone(); Py::Bool({ let t802 = v_outputs.clone(); if !py::contains(&t802, &t801)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_845(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t847: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t846 = v_inputs.clone(); f_competitive_occupancy(py::getitem(&t846, &Py::str("L"))?, py::getitem(&t846, &Py::str("Kd"))?, py::getitem(&t846, &Py::str("I"))?, py::getitem(&t846, &Py::str("Ki"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t847 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t848 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t870 = (t848).items()?;
        if t870.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t870[0].clone();
        v_v = t870[1].clone();
        if ({ let t849 = v_k.clone(); Py::Bool({ let t850 = v_outputs.clone(); if !py::contains(&t850, &t849)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_891(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t893: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t892 = v_inputs.clone(); f_decimal_reduction_time(py::getitem(&t892, &Py::str("N0"))?, py::getitem(&t892, &Py::str("N"))?, py::getitem(&t892, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t893 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t894 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t916 = (t894).items()?;
        if t916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t916[0].clone();
        v_v = t916[1].clone();
        if ({ let t895 = v_k.clone(); Py::Bool({ let t896 = v_outputs.clone(); if !py::contains(&t896, &t895)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_935(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t937: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t936 = v_inputs.clone(); f_smoluchowski_rate(py::getitem(&t936, &Py::str("DA"))?, py::getitem(&t936, &Py::str("DB"))?, py::getitem(&t936, &Py::str("rA"))?, py::getitem(&t936, &Py::str("rB"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t937 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t938 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t960 = (t938).items()?;
        if t960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t960[0].clone();
        v_v = t960[1].clone();
        if ({ let t939 = v_k.clone(); Py::Bool({ let t940 = v_outputs.clone(); if !py::contains(&t940, &t939)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_981(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t983: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t982 = v_inputs.clone(); f_diffusion_time(py::getitem(&t982, &Py::str("x"))?, py::getitem(&t982, &Py::str("D"))?, py::getitem(&t982, &Py::str("dims"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t983 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t984 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1006 = (t984).items()?;
        if t1006.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1006[0].clone();
        v_v = t1006[1].clone();
        if ({ let t985 = v_k.clone(); Py::Bool({ let t986 = v_outputs.clone(); if !py::contains(&t986, &t985)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
