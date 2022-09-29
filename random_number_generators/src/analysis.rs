use plotters::prelude::*;
use plotters_svg::SVGBackend;
use crate::rng::base::*;

const SCATTER_SAMPLE : usize = 40000;

pub fn analyse(mut data : Vec<u32>,name : &str) -> Result<(), Box<dyn std::error::Error>> {

    let sample = data.len() as u32;

    let scatter_data : Vec<(f64,f64)> = data[0..SCATTER_SAMPLE].iter().enumerate().map(|(x)|return ((x.0 as f64) / SCATTER_SAMPLE as f64 , *x.1 as f64 / u32::MAX as f64 ) ).collect();
    scatter_plot(scatter_data, name);


    let mut qq_data = data.clone();
    qq_data.sort();
    let qq_data : Vec<(f64,f64)> = qq_data.iter().enumerate().map(|(x)|return (x.0 as f64 /  sample as f64 , *x.1 as f64 / u32::MAX as f64) ).collect();
    qq_plot(qq_data, name);
    



    return Ok(())
}


fn scatter_plot(data : Vec<(f64,f64)> ,name : &str) -> Result<(), Box<dyn std::error::Error>>{

    let path = format!("scatter_{}.png",name);
    let title = format!("Scatter {}",name);

    let root = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption(&title, ("sans-serif", 100.0))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    scatter_ctx.draw_series(
        data.iter()
            .map(|(x ,y)| Circle::new((*x,*y ), 1, BLUE.filled())),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");


    return Ok(())

}


//TODO add x = x line
fn qq_plot(data : Vec<(f64,f64)>, name: &str)-> Result<(), Box<dyn std::error::Error>> {

    let path = format!("qqplot_{}.png",name);
    let title = format!("QQ Plot {}",name);

    let root = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption(&title, ("sans-serif", 100.0))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    scatter_ctx.draw_series(
        data.iter()
            .map(|(x ,y)| Circle::new((*x,*y ), 1, BLUE.filled())),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    return Ok(());
}
/*
Khi2


QQ plot
histogramme
densité de fonction
scatter plot





*/

