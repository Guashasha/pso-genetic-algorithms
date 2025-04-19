use std::{cmp, fs};

use plotters::{
    prelude::*,
    style::full_palette::{AMBER_A700, GREEN_A700, INDIGO, LIME_A700, PURPLE},
};

use crate::EvolutionStats;

const FILE_NAME: &str = "alg_comparison.png";

pub fn plot_comparison(evol1: EvolutionStats, evol2: EvolutionStats) {
    if let Err(e) = fs::create_dir("plots") {
        if let std::io::ErrorKind::PermissionDenied = e.kind() {
            panic!("No se tienen permisos para crear la carpeta de las gráficas")
        }
    }

    let path = &(String::from("plots/") + FILE_NAME);
    let root_drawing_area = BitMapBackend::new(path, (600, 400)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let max_y = cmp::max(evol1.best.len(), evol2.best.len()) * 100;
    let max_x = cmp::max(
        evol1
            .worst
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap()
            .ceil() as i64,
        evol2
            .worst
            .iter()
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .unwrap()
            .ceil() as i64,
    );

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .caption("Algorithms comparison", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..max_y, 0..max_x)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // dibujar datos de la primera evolución (pso)
    ctx.draw_series(LineSeries::new(
        evol1
            .best
            .iter()
            .enumerate()
            .map(|(i, x)| (i, (*x * 100f64).floor() as i64)),
        &GREEN_A700,
    ))
    .unwrap()
    .label("pso mejor")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN_A700));

    ctx.draw_series(LineSeries::new(
        evol1
            .middle
            .iter()
            .enumerate()
            .map(|(i, x)| (i, (*x * 100f64).floor() as i64)),
        &LIME_A700,
    ))
    .unwrap()
    .label("pso mediana")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], LIME_A700));

    ctx.draw_series(LineSeries::new(
        evol1
            .worst
            .iter()
            .enumerate()
            .map(|(i, x)| (i, (*x * 100f64).floor() as i64)),
        &AMBER_A700,
    ))
    .unwrap()
    .label("pso peor")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], AMBER_A700));

    // dibujar datos de la segunda evolución (genetica)
    ctx.draw_series(LineSeries::new(
        evol2
            .best
            .iter()
            .enumerate()
            .map(|(i, x)| (i, (*x * 100f64).floor() as i64)),
        &BLUE,
    ))
    .unwrap()
    .label("genetica mejor")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    ctx.draw_series(LineSeries::new(
        evol2
            .middle
            .iter()
            .enumerate()
            .map(|(i, x)| (i, (*x * 100f64).floor() as i64)),
        &INDIGO,
    ))
    .unwrap()
    .label("genetica mediana")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], INDIGO));

    ctx.draw_series(LineSeries::new(
        evol2
            .worst
            .iter()
            .enumerate()
            .map(|(i, x)| (i, (*x * 100f64).floor() as i64)),
        &PURPLE,
    ))
    .unwrap()
    .label("genetica peor")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], PURPLE));

    ctx.configure_series_labels()
        .border_style(BLACK)
        .draw()
        .unwrap();
}
