use chrono::prelude::*;
use csv::Reader;
use plotters::prelude::*;
use std::io;
use std::env::args;

const DAY_MS: i64 = 8640; // to avoid overflow of f32
const FONT: &'static str = "DejaVu Sans";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("out.png", (640 * 2, 480 * 2)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    let mut reader = Reader::from_reader(io::stdin());
    let records = reader
        .records()
        .map(|x| {
            let parsed = x.unwrap().get(1).unwrap().parse::<DateTime<Utc>>().unwrap();
            (
                (parsed.timestamp() / DAY_MS) as f32,
                parsed.hour() as f32 + (parsed.minute() as f32) / 59.0,
            )
        })
        .collect::<Vec<(f32, f32)>>();
    let min = records.iter().map(|x| x.0 as i32).min().unwrap() as f32;
    let max = records.iter().map(|x| x.0 as i32).max().unwrap() as f32;
	
	let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("When does {} chat?", args().nth(1).expect("please supply your username as an argument")),
            (FONT, 48).into_font(),
        )
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(min..max, 0f32..23f32)?;

    chart
        .configure_mesh()
        .x_labels(5)
        .y_labels(5)
        .y_label_style((FONT, 24).into_font())
        .x_label_style((FONT, 24).into_font())
        .x_label_formatter(&|x| {
            format!(
                "{}",
                Utc.timestamp_millis((*x as i64) * DAY_MS * 1000).year()
            )
        })
        .y_label_formatter(&|x| format!("{:02}:00", x))
        .draw()?;

    chart.draw_series(PointSeries::of_element(records, 1, &RED, &|c, s, st| {
        return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
    }))?;
    Ok(())
}
