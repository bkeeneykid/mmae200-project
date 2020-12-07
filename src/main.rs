use plotters::prelude::*;

fn main() {
    let root = BitMapBackend::new("3d-surface.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("Torque on beam", ("sans-serif", 40))
        .build_cartesian_3d(0.0..5.0, 0.0..100.0, 50.0..100.0)
        .unwrap();

    chart.configure_axes()
        .draw()
        .unwrap();

    let mut data = vec![];

    for x in (1..26).map(|v| v as f64 / 5.0) {
        let mut row = vec![];
        for z in (0..25).map(|v| (v as f64 * 2.0) + 50.0) {
            // x = length
            // z = force
            let torque = x * z;
            let reaction_force = torque/5.0;
            let point = (x, reaction_force, z);
            println!("{:?} {:?} {:?}", torque, reaction_force,  point);
            row.push(point);
        }
        data.push(row);
    }

    chart.draw_series(
        (0..24)
            .map(|x| std::iter::repeat(x).zip(0..24))
            .flatten()
            .map(|(x,z)| {
                Polygon::new(vec![
                    data[x][z],
                    data[x+1][z],
                    data[x+1][z+1],
                    data[x][z+1],
                ], &BLUE.mix(0.3))
            })
    ).unwrap();

}
