mod reservoir;

use gnuplot::{AxesCommon, Color, Figure, Fix};
use ndarray::arr2;
use ndarray::{s, stack, Array1, Array2, Axis};
use reservoir::Reservoir;
use std::f64::consts::PI;

fn main() {
    let p = 0.05;
    let input_size = 1;
    let output_size = 1;
    let reservoir_size = 1000;
    let la_ridge = 0.0000001; // 1e-7
    let spr = 0.9;

    let init_length: usize = 1000;
    let train_length: usize = 4000;
    let test_length: usize = 2000;
    let tmax: usize = train_length + test_length;

    let mut reservoir = Reservoir::new(input_size, output_size, reservoir_size, p, spr);

    let data: Array1<f64> = Array1::linspace(0.0, 250.0 * PI, tmax).mapv(f64::sin);

    let mut target: Array1<f64> = Array1::zeros(train_length - init_length);
    target.assign(&(data.slice(s![(init_length + 1)..(train_length + 1)])));
    let target: Array2<f64> = target.insert_axis(Axis(1));

    let mut train_data: Array1<f64> = Array1::zeros(train_length);
    train_data.assign(&(data.slice(s![..train_length])));

    let state = train(
        &input_size,
        &reservoir_size,
        &init_length,
        &la_ridge,
        &mut reservoir,
        &train_data,
        &target,
    );

    let val = train_result(
        &target,
        &mut reservoir,
        &input_size,
        &output_size,
        &reservoir_size,
        &init_length,
        &train_length,
    );

    predict(
        &train_length,
        &test_length,
        &tmax,
        &output_size,
        state,
        val,
        data,
        &mut reservoir,
    );
}

fn train(
    input_size: &usize,
    reservoir_size: &usize,
    init_length: &usize,
    la_ridge: &f64,
    reservoir: &mut Reservoir,
    train_data: &Array1<f64>,
    target: &Array2<f64>,
) -> Array2<f64> {
    let mut X: Array2<f64> = Array2::zeros((0, *reservoir_size));
    let mut state: Array2<f64> = Array2::zeros((*reservoir_size, 1));

    for (i, val) in train_data.iter().enumerate() {
        state = reservoir.next_state(&arr2(&[[*val]]), &state);

        if i >= *init_length {
            X = stack![Axis(0), X, state.t()];
        }
    }
    reservoir.update_w(&X, target, la_ridge);

    state
}

fn train_result(
    target: &Array2<f64>,
    reservoir: &mut Reservoir,
    input_size: &usize,
    output_size: &usize,
    reservoir_size: &usize,
    init_length: &usize,
    train_length: &usize,
) -> f64 {
    let mut outputs: Array2<f64> = Array2::zeros((0, *output_size));
    let mut state: Array2<f64> = Array2::zeros((*reservoir_size, 1));
    let mut val: f64 = 0.0;

    for v in target {
        val = *v;
        state = reservoir.next_state(&arr2(&[[val]]), &state);

        outputs = stack![Axis(0), outputs, reservoir.output(&state)];
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(
            *init_length..*train_length,
            outputs.iter(),
            &[Color("blue")],
        )
        .lines(*init_length..*train_length, target.iter(), &[Color("red")])
        .set_y_range(Fix(-1.1), Fix(1.1));
    fg.save_to_png("training.png", 1024, 768).unwrap();

    val
}

fn predict(
    train_length: &usize,
    test_length: &usize,
    tmax: &usize,
    output_size: &usize,
    state: Array2<f64>,
    val: f64,
    data: Array1<f64>,
    reservoir: &mut Reservoir,
) {
    let mut state: Array2<f64> = state;
    let mut outputs: Array2<f64> = Array2::zeros((0, *output_size));

    let mut y: Array2<f64> = Array2::zeros((*output_size, 1));
    y.fill(val);

    for _ in 0..*test_length {
        state = reservoir.next_state(&y, &state);
        y = reservoir.output(&state);

        outputs = stack![Axis(0), outputs, y];
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(*train_length..*tmax, outputs.iter(), &[Color("blue")])
        .lines(
            *train_length..*tmax,
            data.slice(s![*train_length..]).to_owned().iter(),
            &[Color("red")],
        )
        .set_y_range(Fix(-1.1), Fix(1.1));
    fg.save_to_png("prediction.png", 1024, 768).unwrap();
}
