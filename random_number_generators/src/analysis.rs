use plotters::prelude::*;
use plotters_svg::SVGBackend;
use crate::rng::base::*;

const SCATTER_SAMPLE : usize = 40000;
const HISTO_SAMPLE : usize = 80;

pub fn analyse(data : &Vec<u32>,name : &str) -> Result<(), Box<dyn std::error::Error>> {

    scatter_plot(&data, name);

    qq_plot(&data, name);

    histogram_plot(&data, name);
    
    return Ok(())
}


fn scatter_plot(data : &Vec<u32> ,name : &str) -> Result<(), Box<dyn std::error::Error>>{

    let scatter_data : Vec<(f64,f64)> = data[0..SCATTER_SAMPLE].iter().enumerate().map(|(x)|return ((x.0 as f64) / SCATTER_SAMPLE as f64 , *x.1 as f64 / u32::MAX as f64 ) ).collect();

    let path = format!("plots/scatter_{}.png",name);
    let title = format!("Scatter {}, {} points",name,SCATTER_SAMPLE);

    let root = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(30)
        .caption(&title, ("sans-serif", 100.0))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("sans-serif", 25.0))
        .draw()?;

    scatter_ctx.draw_series(
        scatter_data.iter()
            .map(|(x ,y)| Circle::new((*x,*y ), 1, BLUE.filled())),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");


    return Ok(())

}

//TODO add x = x line
fn qq_plot(data : &Vec<u32>, name: &str)-> Result<(), Box<dyn std::error::Error>> {
    let sample_size = data.len() as u32;
    let mut qq_data = data.clone();
    qq_data.sort();
    let qq_data : Vec<(f64,f64)> = qq_data.iter().enumerate().map(|(x)|return (x.0 as f64 /  sample_size as f64 , *x.1 as f64 / u32::MAX as f64) ).collect();

    let path = format!("plots/qqplot_{}.png",name);
    let title = format!("QQ Plot {}",name);

    let root = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut scatter_ctx = ChartBuilder::on(&root)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(30)
        .caption(&title, ("sans-serif", 100.0))
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("sans-serif", 25.0))
        .draw()?;

    scatter_ctx.draw_series(
        qq_data.iter()
            .map(|(x ,y)| Circle::new((*x,*y ), 1, BLUE.filled())),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    return Ok(());
}

fn histogram_plot(data : &Vec<u32>, name: &str)-> Result<(), Box<dyn std::error::Error>> {

    let sample_size = data.len() as u32;
    let histo_data : Vec<f64> = data.iter().map(|(x)|*x as f64 /  u32::MAX as f64).collect();

    let path = format!("plots/histogramme_{}.png",name);
    let title = format!("Histogramme {}",name);

    let root = BitMapBackend::new(&path, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut histogram_ctx = ChartBuilder::on(&root)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(30)
        .caption(&title, ("sans-serif", 100.0))
        .build_cartesian_2d((0f64..1f64).step(1f64/HISTO_SAMPLE as f64).use_floor(), 0f64..1.1f64)?;
        histogram_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .label_style(("sans-serif", 25.0))
        .draw()?;

        histogram_ctx.draw_series(
            Histogram::vertical(&histogram_ctx)
            .style(BLUE.stroke_width(3))
            .margin(0)
            .data(histo_data.iter().map(|x| (*x, 1f64/sample_size as f64 * HISTO_SAMPLE as f64))),
        )?;

        // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    return Ok(());
}

/*
Khi2
Moyenne
corrélation pearson

QQ plot DONE
histogramme DONE
scatter plot DONE





*/

