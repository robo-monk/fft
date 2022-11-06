use std::ops::{Range, RangeInclusive};
use plotters::prelude::*;

fn draw(x_range: Range<f32>, y_range: Range<f32>, resolution: u32) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("test.png", (640, 480)).into_drawing_area();
    
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        // .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;
        .build_cartesian_2d(x_range.clone(), y_range.clone())?;

    chart.configure_mesh().draw()?;

    // chart
    //     .draw_series(LineSeries::new(
    //         (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
    //         &RED,
    //     ))?
    //     .label("y = x^2")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // chart
    //     .draw_series(LineSeries::new(
    //         (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, (2.0 + x) * x)),
    //         &BLUE,
    //     ))?
    //     .label("y = x^2")
    //     .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    let samples: RangeInclusive<i32> = ((x_range.start as i32) * resolution as i32)..=((x_range.end as i32) * resolution as i32);
    println!("samples -> {:?}", samples);
    chart
        .draw_series(LineSeries::new(
            samples.map(|x| x as f32 / resolution as f32).map(|x| (x, x.sin())),
            &BLUE,
        ))?
        .label("y = sin(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // draw(-25.0..25.0, -1.0..1.0, 100)
    draw(-25.0..25.0, -1.0..1.0, 100)
}
