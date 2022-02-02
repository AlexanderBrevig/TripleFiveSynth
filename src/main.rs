use std::collections::HashSet;
use std::hash::Hash;
use std::io::Write;
use std::fs::File;

const PICO_VALS: [f64; 24] = [1.0, 1.1, 1.2, 1.3, 1.5, 1.6, 1.8, 2.0, 2.2, 2.4, 2.7, 3.0, 3.3, 3.6, 3.9, 4.3, 4.7, 5.1, 5.6, 6.2, 6.8, 7.5, 8.2, 9.1];
const HZ: [f64; 108] = [16.35, 17.32, 18.35, 19.45, 20.60, 21.83, 23.12, 24.50, 25.96, 27.50, 29.14, 30.87, 32.70, 34.65, 36.71, 38.89, 41.20, 43.65, 46.25, 49.00, 51.91, 55.00, 58.27, 61.74, 65.41, 69.30, 73.42, 77.78, 82.41, 87.31, 92.50, 98.00, 103.83, 110.00, 116.54, 123.47, 130.81, 138.59, 146.83, 155.56, 164.81, 174.61, 185.00, 196.00, 207.65, 220.00, 233.08, 246.94, 261.63, 277.18, 293.66, 311.13, 329.63, 349.23, 369.99, 392.00, 415.30, 440.00, 466.16, 493.88, 523.25, 554.37, 587.33, 622.25, 659.25, 698.46, 739.99, 783.99, 830.61, 880.00, 932.33, 987.77, 1046.50, 1108.73, 1174.66, 1244.51, 1318.51, 1396.91, 1479.98, 1567.98, 1661.22, 1760.00, 1864.66, 1975.53, 2093.00, 2217.46, 2349.32, 2489.02, 2637.02, 2793.83, 2959.96, 3135.96, 3322.44, 3520.00, 3729.31, 3951.07, 4186.01, 4434.92, 4698.63, 4978.03, 5274.04, 5587.65, 5919.91, 6271.93, 6644.88, 7040.00, 7458.62, 7902.13];

const DELTA:f64 = 0.0000000000001;
const PICO: f64 = 0.000000000001;
const NANO: f64 = 0.000000001;
const MICRO: f64 = 0.000001;
const TRIM_MAX:f64 = -1000.0;
const TRIM_MIN:f64 = 1000.0;

fn find_min_max_trim(freq:f64, cap:usize, base_unit:f64) -> (f64,f64,f64,f64) {
    let c1: f64 =   PICO_VALS[cap] * base_unit;

    let min:f64 = find_width_trim(c1, TRIM_MIN);
    let max:f64 = find_width_trim(c1, TRIM_MAX);
    let (best_trim, best_freq) = find_trim_and_freq(c1, freq);

    return (min,max,best_trim,best_freq);
}

fn find_width_trim(c1:f64, trim:f64) -> f64 {
    let r1: f64 = 1000.0;
    let r2: f64 = 10000.0+trim;

    let t1: f64 = 0.693 * (r1+r2) * c1; 
    let t2: f64 = 0.693 * r2 * c1;
    //let f:f64 = (1.0/(t1 + t2)*100.0).round() / 100.0;
    let f:f64 = 1.0/(t1 + t2);
    return f;
}
// No python here my dude 
fn find_trim_and_freq(c1:f64, f:f64) -> (f64,f64) {
    let mut trim:f64 = 1000.0;
    let delta:f64 = 0.1;
    let mut best_delta:f64 = 100000.0;
    let mut best_trim:f64 = 0.0;
    let mut best_freq:f64 = 0.0;
    while trim>=-1000.0 {
        let f_found = find_width_trim(c1, trim);

        let freq_delta = (f_found - f).abs();
        if freq_delta < best_delta {
            best_delta = freq_delta;
            best_trim = trim;
            best_freq = f_found;
        }
        if freq_delta <= 0.005{
            break;
        }
        trim -= delta;
    }
    best_trim = (best_trim*100.0).round()/100.0;
    best_freq = (best_freq*100.0).round()/100.0;
    return (best_trim, best_freq);
}

fn find(freq:f64, note:usize, cap:usize, base_unit:f64) -> Result<(f64, f64,f64,f64), &'static str>{
    let (min,max,trim,freq_result) = find_min_max_trim(freq, cap, base_unit);
    if min <= HZ[note] && max >= HZ[note] {
        return Ok((min, max, trim, freq_result));
    } else {
        return Err("Could not find match for note");
    }
}

#[derive(Hash, Debug)]
struct Cap {
    value: u32,
    unit: String,
}
impl PartialEq for Cap {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.unit == other.unit
    }
}
impl Eq for Cap {}


fn main() -> std::io::Result<()> {
    println!("CAPACITOR & TRIM CALCULATOR FOR 555 SYNTH:\n\n");
    let file_name = "tune_triple_fives.txt";
    println!("Writing to file `{}`", file_name);
    let mut file = File::create(file_name)?;
    let mut found = 0;
    let mut capvals = HashSet::new();
    let mut min_trim = TRIM_MIN; // Seems backwards, but it is correct
    let mut max_trim = TRIM_MAX;
    for (i,freq) in HZ.iter().enumerate() {
        let mut found_match = false;
        for (_, unit) in [MICRO, NANO, PICO].iter().enumerate() {
            for (_, multiplier) in [1.0,10.0,100.0].iter().enumerate() {
                for (v, value) in PICO_VALS.iter().enumerate() {
                    //println!("freq #{}:{}, value #{}:{}, unit #{}:{}", i,freq,v,value,u,unit);
                    match find(*freq, i, v, *unit * multiplier) {
                        Ok((_min,_max,trim,_best_freq)) => {
                            let unitstr:&str = match unit {
                                x if x >= &(&MICRO-DELTA) && x <= &(&MICRO+DELTA) => "u",
                                x if x >= &(&NANO-DELTA) && x <= &(&NANO+DELTA) => "n",
                                x if x >= &(&PICO-DELTA) && x <= &(&PICO+DELTA) => "p",
                                _ => "",
                            };
                            // Keep inventory of used capacitor values
                            capvals.insert( Cap{ value: (*value*multiplier).round() as u32,unit: unitstr.to_string()});
                            // Print information for layout
                            file.write_fmt(format_args!("{:07.2}Hz => {:06.2}{}F\tTrim => {:06.1}Ω -> {:05.2}%\n", freq, (value*multiplier*100.0).round()/100.0, unitstr, ((1000.0+trim)*100.0).round()/100.0, (((1000.0+trim)/2000.0)*10000.0).round()/100.0))?;

                            found_match = true;
                            found += 1;
                            if trim < min_trim {
                                min_trim = trim;
                            } else if trim > max_trim {
                                max_trim = trim;
                            }
                            break;
                        },
                        Err(_)=> {},
                    }
                }
            }
        }
        if !found_match {
            println!("ERROR: No match for #{}:{}", i, freq)
        }
    }
    println!("Found {}", found);
    println!("Capacitor values #{}", capvals.len());
    println!("Trim from {} to {}", min_trim, max_trim);
    /*for capval in &capvals {
        println!("{:?}", capval)
    }*/
    Ok(())
}

#[test]
fn test_known_good_c0() {
    let res = find(16.35, 0, 14, MICRO).unwrap();
    assert_eq!(16.35, res.3);
    assert_eq!(815.0, res.2);
}

#[test]
fn test_known_good_a4() {
    let res = find(440.0, 57, 4, NANO * 100.0).unwrap();
    assert_eq!(440.0, res.3);
    assert_eq!(431.8, res.2);
}

#[test]
fn test_known_good_b8() {
    let res = find(7902.13, 107, 22, NANO).unwrap();
    assert_eq!(7902.13, res.3);
    assert_eq!(634.7, res.2);
}
