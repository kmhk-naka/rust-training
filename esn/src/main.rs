mod reservoir;

use gnuplot::{AxesCommon, Color, Figure, Fix};
use ndarray::arr2;
use ndarray::{s, Array1, Array2, Axis};
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
        &reservoir_size,
        &init_length,
        &train_length,
        &la_ridge,
        &mut reservoir,
        &train_data,
        &target,
    );

    let val = train_result(
        &target,
        &mut reservoir,
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
    reservoir_size: &usize,
    init_length: &usize,
    train_length: &usize,
    la_ridge: &f64,
    reservoir: &mut Reservoir,
    train_data: &Array1<f64>,
    target: &Array2<f64>,
) -> Array2<f64> {
    let mut X: Array2<f64> = Array2::zeros((*train_length - *init_length, *reservoir_size));
    let mut state: Array2<f64> = Array2::zeros((*reservoir_size, 1));

    for (i, val) in train_data.iter().enumerate() {
        state = reservoir.next_state(&arr2(&[[*val]]), &state);

        if i >= *init_length {
            let mut av = X.row_mut(i - *init_length);
            av.assign(&state.column(0));
        }
    }
    reservoir.update_w(&X, target, la_ridge);

    state
}

fn train_result(
    target: &Array2<f64>,
    reservoir: &mut Reservoir,
    output_size: &usize,
    reservoir_size: &usize,
    init_length: &usize,
    train_length: &usize,
) -> f64 {
    let mut outputs: Array2<f64> = Array2::zeros((*train_length - *init_length, *output_size));
    let mut state: Array2<f64> = Array2::zeros((*reservoir_size, 1));
    let mut val: f64 = 0.0;

    for (i, v) in target.iter().enumerate() {
        val = *v;
        state = reservoir.next_state(&arr2(&[[val]]), &state);

        let mut av = outputs.row_mut(i);
        av.assign(&reservoir.output(&state).column(0));
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
    let mut outputs: Array2<f64> = Array2::zeros((*test_length, *output_size));

    let mut test_data: Array1<f64> = Array1::zeros(*test_length);
    test_data.assign(&(data.slice(s![*train_length..])));

    let mut y: Array2<f64> = Array2::zeros((*output_size, 1));
    y.fill(val);

    for (i, val) in test_data.iter().enumerate() {
        state = reservoir.next_state(&arr2(&[[*val]]), &state);
        y = reservoir.output(&state);

        let mut av = outputs.row_mut(i);
        av.assign(&y.column(0));
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .lines(*train_length..*tmax, outputs.iter(), &[Color("blue")])
        .lines(
            *train_length..*tmax,
            test_data.iter(),
            &[Color("red")],
        )
        .set_y_range(Fix(-1.1), Fix(1.1));
    fg.save_to_png("prediction.png", 1024, 768).unwrap();
}
