
#[derive(Debug)]
pub enum AstrometricPrior {
    NoPrior = 0,
    GalaxyBayesian = 1,
    GalaxyBayesian10 = 2,
    Hipparcos = 3,
    HipparcosProperMotion = 4,
    Tycho2 = 5,
    Quasar = 6,
}

impl AstrometricPrior {
    pub fn from_str(input: Option<&str>) -> Option<AstrometricPrior> {
        match input {
            None => None,
            Some("0") => Some(AstrometricPrior::NoPrior),
            Some("1") => Some(AstrometricPrior::GalaxyBayesian),
            Some("2") => Some(AstrometricPrior::GalaxyBayesian10),
            Some("3") => Some(AstrometricPrior::Hipparcos),
            Some("4") => Some(AstrometricPrior::HipparcosProperMotion),
            Some("5") => Some(AstrometricPrior::Tycho2),
            Some("6") => Some(AstrometricPrior::Quasar),
            _ => {
                println!("Unknown AstrometricPrior input: {:?}", input);
                None
            }
        }
    }
}

// For more info, see https://gaia.esac.esa.int/documentation/GDR1/datamodel/Ch1/tgas_source.html
#[derive(Debug)]
pub struct GaiaEntry {
    pub hip: Option<i32>,
    pub tycho2_id: Option<String>,
    pub solution_id: u64,
    pub source_id: u64,
    pub random_index: u64,
    pub ref_epoch: f64,
    pub ra: f64,
    pub ra_error: f64,
    pub dec: f64,
    pub dec_error: f64,
    pub parallax: f64,
    pub parallax_error: f64,
    pub pmra: f64,
    pub pmra_error: f64,
    pub pmdec: f64,
    pub pmdec_error: f64,
    pub ra_dec_corr: f32,
    pub ra_parallax_corr: f32,
    pub ra_pmra_corr: f32,
    pub ra_pmdec_corr: f32,
    pub dec_parallax_corr: f32,
    pub dec_pmra_corr: f32,
    pub dec_pmdec_corr: f32,
    pub parallax_pmra_corr: f32,
    pub parallax_pmdec_corr: f32,
    pub pmra_pmdec_corr: f32,
    pub astrometric_n_obs_al: u32,
    pub astrometric_n_obs_ac: u32,
    pub astrometric_n_good_obs_al: u32,
    pub astrometric_n_good_obs_ac: u32,
    pub astrometric_n_bad_obs_al: u32,
    pub astrometric_n_bad_obs_ac: u32,
    pub astrometric_delta_q: Option<f32>,
    pub astrometric_excess_noise: f64,
    pub astrometric_excess_noise_sig: f64,
    pub astrometric_primary_flag: bool,
    pub astrometric_relegation_factor: f32,
    pub astrometric_weight_al: f32,
    pub astrometric_weight_ac: Option<f32>,
    pub astrometric_priors_used: AstrometricPrior,
    pub matched_observations: u16,
    pub duplicated_source: bool,
    pub scan_direction_strength_k1: f32,
    pub scan_direction_strength_k2: f32,
    pub scan_direction_strength_k3: f32,
    pub scan_direction_strength_k4: f32,
    pub scan_direction_mean_k1: f32,
    pub scan_direction_mean_k2: f32,
    pub scan_direction_mean_k3: f32,
    pub scan_direction_mean_k4: f32,
    pub phot_g_n_obs: u32,
    pub phot_g_mean_flux: f64,
    pub phot_g_mean_flux_error: f64,
    pub phot_g_mean_mag: f64,
    pub phot_variable_flag: String,
    pub l: f64,
    pub b: f64,
    pub ecl_lon: f64,
    pub ecl_lat: f64,
}

impl GaiaEntry {
    pub fn from_line(line: String) -> Option<GaiaEntry> {
        let mut split = line.split(',');

        Some(GaiaEntry {
            hip: get_optional_i32!(split.next(), "hip"),
            tycho2_id: get_optional_string!(split.next(), "tycho2_id").map(String::from),
            solution_id: try_get_u64!(split.next(), "solution_id"),
            source_id: try_get_u64!(split.next(), "source_id"),
            random_index: try_get_u64!(split.next(), "random_index"),
            ref_epoch: try_get_f64!(split.next(), "ref_epoch"),
            ra: try_get_f64!(split.next(), "ra"),
            ra_error: try_get_f64!(split.next(), "ra_error"),
            dec: try_get_f64!(split.next(), "dec"),
            dec_error: try_get_f64!(split.next(), "dec_error"),
            parallax: try_get_f64!(split.next(), "parallax"),
            parallax_error: try_get_f64!(split.next(), "parallax_error"),
            pmra: try_get_f64!(split.next(), "pmra"),
            pmra_error: try_get_f64!(split.next(), "pmra_error"),
            pmdec: try_get_f64!(split.next(), "pmdec"),
            pmdec_error: try_get_f64!(split.next(), "pmdec_error"),
            ra_dec_corr: try_get_f32!(split.next(), "ra_dec_corr"),
            ra_parallax_corr: try_get_f32!(split.next(), "ra_parallax_corr"),
            ra_pmra_corr: try_get_f32!(split.next(), "ra_pmra_corr"),
            ra_pmdec_corr: try_get_f32!(split.next(), "ra_pmdec_corr"),
            dec_parallax_corr: try_get_f32!(split.next(), "dec_parallax_corr"),
            dec_pmra_corr: try_get_f32!(split.next(), "dec_pmra_corr"),
            dec_pmdec_corr: try_get_f32!(split.next(), "dec_pmdec_corr"),
            parallax_pmra_corr: try_get_f32!(split.next(), "parallax_pmra_corr"),
            parallax_pmdec_corr: try_get_f32!(split.next(), "parallax_pmdec_corr"),
            pmra_pmdec_corr: try_get_f32!(split.next(), "pmra_pmdec_corr"),
            astrometric_n_obs_al: try_get_u32!(split.next(), "astrometric_n_obs_al"),
            astrometric_n_obs_ac: try_get_u32!(split.next(), "astrometric_n_obs_ac"),
            astrometric_n_good_obs_al: try_get_u32!(split.next(), "astrometric_n_good_obs_al"),
            astrometric_n_good_obs_ac: try_get_u32!(split.next(), "astrometric_n_good_obs_ac"),
            astrometric_n_bad_obs_al: try_get_u32!(split.next(), "astrometric_n_bad_obs_al"),
            astrometric_n_bad_obs_ac: try_get_u32!(split.next(), "astrometric_n_bad_obs_ac"),
            astrometric_delta_q: get_optional_f32!(split.next(), "astrometric_delta_q"),
            astrometric_excess_noise: try_get_f64!(split.next(), "astrometric_excess_noise"),
            astrometric_excess_noise_sig: try_get_f64!(split.next(), "astrometric_excess_noise_sig"),
            astrometric_primary_flag: try_get_bool!(split.next(), "astrometric_primary_flag"),
            astrometric_relegation_factor: try_get_f32!(split.next(), "astrometric_relegation_factor"),
            astrometric_weight_al: try_get_f32!(split.next(), "astrometric_weight_al"),
            astrometric_weight_ac: get_optional_f32!(split.next(), "astrometric_weight_ac"),
            astrometric_priors_used: try_get!(AstrometricPrior::from_str(split.next()), "astrometric_priors_used"),
            matched_observations: try_get_u16!(split.next(), "matched_observations"),
            duplicated_source: try_get_bool!(split.next(), "duplicated_source"),
            scan_direction_strength_k1: try_get_f32!(split.next(), "scan_direction_strength_k1"),
            scan_direction_strength_k2: try_get_f32!(split.next(), "scan_direction_strength_k2"),
            scan_direction_strength_k3: try_get_f32!(split.next(), "scan_direction_strength_k3"),
            scan_direction_strength_k4: try_get_f32!(split.next(), "scan_direction_strength_k4"),
            scan_direction_mean_k1: try_get_f32!(split.next(), "scan_direction_mean_k1"),
            scan_direction_mean_k2: try_get_f32!(split.next(), "scan_direction_mean_k2"),
            scan_direction_mean_k3: try_get_f32!(split.next(), "scan_direction_mean_k3"),
            scan_direction_mean_k4: try_get_f32!(split.next(), "scan_direction_mean_k4"),
            phot_g_n_obs: try_get_u32!(split.next(), "phot_g_n_obs"),
            phot_g_mean_flux: try_get_f64!(split.next(), "phot_g_mean_flux"),
            phot_g_mean_flux_error: try_get_f64!(split.next(), "phot_g_mean_flux_error"),
            phot_g_mean_mag: try_get_f64!(split.next(), "phot_g_mean_mag"),
            phot_variable_flag: String::from(try_get_nonempty_string!(split.next(), "phot_variable_flag")),
            l: try_get_f64!(split.next(), "l"),
            b: try_get_f64!(split.next(), "b"),
            ecl_lon: try_get_f64!(split.next(), "ecl_lon"),
            ecl_lat: try_get_f64!(split.next(), "ecl_lat"),
        })
    }

    pub fn validate_headers(line: &str) {
        let mut split = line.split(',');
        debug_assert_eq!(split.next(), Some("hip"));
        debug_assert_eq!(split.next(), Some("tycho2_id"));
        debug_assert_eq!(split.next(), Some("solution_id"));
        debug_assert_eq!(split.next(), Some("source_id"));
        debug_assert_eq!(split.next(), Some("random_index"));
        debug_assert_eq!(split.next(), Some("ref_epoch"));
        debug_assert_eq!(split.next(), Some("ra"));
        debug_assert_eq!(split.next(), Some("ra_error"));
        debug_assert_eq!(split.next(), Some("dec"));
        debug_assert_eq!(split.next(), Some("dec_error"));
        debug_assert_eq!(split.next(), Some("parallax"));
        debug_assert_eq!(split.next(), Some("parallax_error"));
        debug_assert_eq!(split.next(), Some("pmra"));
        debug_assert_eq!(split.next(), Some("pmra_error"));
        debug_assert_eq!(split.next(), Some("pmdec"));
        debug_assert_eq!(split.next(), Some("pmdec_error"));
        debug_assert_eq!(split.next(), Some("ra_dec_corr"));
        debug_assert_eq!(split.next(), Some("ra_parallax_corr"));
        debug_assert_eq!(split.next(), Some("ra_pmra_corr"));
        debug_assert_eq!(split.next(), Some("ra_pmdec_corr"));
        debug_assert_eq!(split.next(), Some("dec_parallax_corr"));
        debug_assert_eq!(split.next(), Some("dec_pmra_corr"));
        debug_assert_eq!(split.next(), Some("dec_pmdec_corr"));
        debug_assert_eq!(split.next(), Some("parallax_pmra_corr"));
        debug_assert_eq!(split.next(), Some("parallax_pmdec_corr"));
        debug_assert_eq!(split.next(), Some("pmra_pmdec_corr"));
        debug_assert_eq!(split.next(), Some("astrometric_n_obs_al"));
        debug_assert_eq!(split.next(), Some("astrometric_n_obs_ac"));
        debug_assert_eq!(split.next(), Some("astrometric_n_good_obs_al"));
        debug_assert_eq!(split.next(), Some("astrometric_n_good_obs_ac"));
        debug_assert_eq!(split.next(), Some("astrometric_n_bad_obs_al"));
        debug_assert_eq!(split.next(), Some("astrometric_n_bad_obs_ac"));
        debug_assert_eq!(split.next(), Some("astrometric_delta_q"));
        debug_assert_eq!(split.next(), Some("astrometric_excess_noise"));
        debug_assert_eq!(split.next(), Some("astrometric_excess_noise_sig"));
        debug_assert_eq!(split.next(), Some("astrometric_primary_flag"));
        debug_assert_eq!(split.next(), Some("astrometric_relegation_factor"));
        debug_assert_eq!(split.next(), Some("astrometric_weight_al"));
        debug_assert_eq!(split.next(), Some("astrometric_weight_ac"));
        debug_assert_eq!(split.next(), Some("astrometric_priors_used"));
        debug_assert_eq!(split.next(), Some("matched_observations"));
        debug_assert_eq!(split.next(), Some("duplicated_source"));
        debug_assert_eq!(split.next(), Some("scan_direction_strength_k1"));
        debug_assert_eq!(split.next(), Some("scan_direction_strength_k2"));
        debug_assert_eq!(split.next(), Some("scan_direction_strength_k3"));
        debug_assert_eq!(split.next(), Some("scan_direction_strength_k4"));
        debug_assert_eq!(split.next(), Some("scan_direction_mean_k1"));
        debug_assert_eq!(split.next(), Some("scan_direction_mean_k2"));
        debug_assert_eq!(split.next(), Some("scan_direction_mean_k3"));
        debug_assert_eq!(split.next(), Some("scan_direction_mean_k4"));
        debug_assert_eq!(split.next(), Some("phot_g_n_obs"));
        debug_assert_eq!(split.next(), Some("phot_g_mean_flux"));
        debug_assert_eq!(split.next(), Some("phot_g_mean_flux_error"));
        debug_assert_eq!(split.next(), Some("phot_g_mean_mag"));
        debug_assert_eq!(split.next(), Some("phot_variable_flag"));
        debug_assert_eq!(split.next(), Some("l"));
        debug_assert_eq!(split.next(), Some("b"));
        debug_assert_eq!(split.next(), Some("ecl_lon"));
        debug_assert_eq!(split.next(), Some("ecl_lat"));
        debug_assert_eq!(split.next(), None);
    }
}