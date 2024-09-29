use chrono::{DateTime, FixedOffset, Utc};

pub const JULIAN_CALENDER_START: f64 = 2451545.0;
pub const FRACTIONAL_JULIAN_DAY_FOR_LEAP_YEAR: f64 = 0.0008;
pub const ADDED_SECS: f64 = 69.184 / 86400.0;

/// Helper function to convert a timestamp to a human-readable string
fn ts2human(ts: f64, tz: FixedOffset) -> String {
    let utc_dt = DateTime::from_timestamp(ts as i64, 0).unwrap();
    let dt: DateTime<FixedOffset> = utc_dt.with_timezone(&tz);
    dt.to_string()
}

/// Helper function to format degrees into human-readable string
fn deg2human(deg: f64) -> String {
    let rad = deg.to_radians();
    let seconds = (deg * 3600.0).floor() as i32;
    let d = seconds / 3600;
    let m = (seconds / 60) % 60;
    let s = seconds % 60;
    format!("∠{:.3}rad = ∠{}°{}′{}″ = ∠{:.3}°", rad, d, m, s, deg)
}

/// Converts Julian date to Unix timestamp
fn j2ts(j: f64) -> f64 {
    (j - 2440587.5) * 86400.0
}

/// Converts Unix timestamp to Julian date
fn ts2j(ts: f64) -> f64 {
    ts / 86400.0 + 2440587.5
}

/// Calculates sunrise and sunset times based on latitude, longitude, and elevation
fn calculate_sunrise_sunset(
    current_timestamp: f64,
    latitude: f64,
    longitude: f64,
    elevation: f64,
    tz: FixedOffset,
) -> Result<(f64, f64), String> {
    println!("Latitude               f       = {}", deg2human(latitude));
    println!("Longitude              l_w     = {}", deg2human(longitude));
    println!(
        "Now                    ts      = {}",
        ts2human(current_timestamp, tz)
    );

    let j_date = ts2j(current_timestamp);
    println!("Julian date            j_date  = {:.3} days", j_date);

    let n = (j_date - (JULIAN_CALENDER_START + FRACTIONAL_JULIAN_DAY_FOR_LEAP_YEAR) + ADDED_SECS)
        .ceil();
    println!("Julian day             n       = {:.3} days", n);

    let solar_mean_time = n - longitude / 360.0;
    println!(
        "Mean solar time        J_      = {:.9} days",
        solar_mean_time
    );

    let solar_mean_anomaly = (357.5291 + 0.98560028 * solar_mean_time) % 360.0;
    println!(
        "Solar mean anomaly     M       = {}",
        deg2human(solar_mean_anomaly)
    );
    let solar_mean_anomaly_radians = solar_mean_anomaly.to_radians();

    let equation_of_center = (1.9148 * solar_mean_anomaly_radians.sin())
        + (0.02 * (2.0 * solar_mean_anomaly_radians).sin())
        + (0.0003 * (3.0 * solar_mean_anomaly_radians).sin());
    println!(
        "Equation of the center C       = {}",
        deg2human(equation_of_center)
    );

    let eliptic_longitude = (solar_mean_anomaly + equation_of_center + 180.0 + 102.9372) % 360.0;
    println!(
        "Ecliptic longitude     L       = {}",
        deg2human(eliptic_longitude)
    );

    let eliptic_longitude_radians = eliptic_longitude.to_radians();
    let solar_transit =
        JULIAN_CALENDER_START + solar_mean_time + 0.0053 * solar_mean_anomaly_radians.sin()
            - (0.0069 * (2.0 * eliptic_longitude).sin());
    println!(
        "Solar transit time     J_trans = {:.3}",
        j2ts(solar_transit)
    );

    // declination of the sun
    let sin_d = eliptic_longitude_radians.sin() * (23.4397_f64).sin();
    let cos_d = sin_d.asin().cos();

    // hour angle
    let some_cos = ((-0.833 - 2.076 * elevation.sqrt() / 60.0)
        .to_radians()
        .sin()
        - (latitude.to_radians().sin() * sin_d))
        / (latitude.to_radians().cos() * cos_d);
    let w0_radians = some_cos.acos();
    let w0_degrees = w0_radians.to_degrees();

    let j_rise = solar_transit - w0_degrees / 360.0;
    let j_set = solar_transit + w0_degrees / 360.0;

    println!("Sunrise                j_rise  = {:.3}", j2ts(j_rise));
    println!("Sunset                 j_set   = {:.3}", j2ts(j_set));

    Ok((j2ts(j_rise), j2ts(j_set)))
}

fn main() {
    let latitude = 27.6706;
    let longitude = 84.4385;
    let elevation = 0.0;

    let tz = FixedOffset::east_opt(5 /* hours */ * 3600 + 45 /* mins */ * 60).unwrap();
    let current_timestamp = Utc::now().timestamp() as f64;

    match calculate_sunrise_sunset(current_timestamp, latitude, longitude, elevation, tz) {
        Ok((sunrise, sunset)) => {
            println!("Sunrise: {}", ts2human(sunrise, tz));
            println!("Sunset: {}", ts2human(sunset, tz));
        }
        Err(e) => eprintln!("Error calculating sunrise/sunset: {}", e),
    }
}
