# TripleFiveSynth

A simple idea of using a 555 timer per note in a synth.

## `/src/main.rs`


This will generate appropriate capacitor and trim values for each note.

See the output in [tune_triple_fives.txt](https://raw.githubusercontent.com/alexanderbrevig/TripleFiveSynth/tune_triple_fives.txt) for details.

## 555 Configuration

The plan is to have one of these voices per note:
![555 voice](https://raw.githubusercontent.com/AlexanderBrevig/TripleFiveSynth/main/img/triplefive.png)

## Example tuning for 440Hz

Per the line in `tune_triple_fives.txt` we get:

> 0440.00Hz => 150.00nF	Trim => 1431.9Ω -> 71.60%

This means, our `C3` should be `150nF`, and `R3` at approx `71.6%`.
