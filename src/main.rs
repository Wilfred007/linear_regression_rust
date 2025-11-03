use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::linear_regression::LinearRegression;
use csv::ReaderBuilder;
use std::error::Error;

fn read_csv(path: &str) -> Result<(Vec<f64>, Vec<f64>), Box<dyn Error>> {
    // Create a CSV reader that automatically reads the header row
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut gdp = Vec::new();
    let mut life_satisfaction = Vec::new();

    // Iterate over each record (row)
    for result in rdr.records() {
        let record = result?;

        // record[0] = Country name (we skip it)
        let gdp_value: f64 = record[1].parse()?; // GDP per capita
        let life_value: f64 = record[2].parse()?; // Life satisfaction

        gdp.push(gdp_value);
        life_satisfaction.push(life_value);
    }

    Ok((gdp, life_satisfaction))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Path to your CSV file
    let path = "country_stats.csv";

    // Read the data
    let (gdp, life_satisfaction) = read_csv(path)?;

    // Convert GDP to column vector [[x1], [x2], ...]
    let x: Vec<Vec<f64>> = gdp.iter().map(|&v| vec![v]).collect();

    // Create feature matrix and target vector
    let x_matrix = DenseMatrix::from_2d_vec(&x);
    let y = life_satisfaction.clone();

    // Train a linear regression model
    let lr = LinearRegression::fit(&x_matrix, &y, Default::default())?;

    // Predict for Cyprus (GDP per capita = 22587)
    let x_new = DenseMatrix::from_2d_vec(&vec![vec![22587.0]]);
    let prediction = lr.predict(&x_new)?;

    println!("Predicted life satisfaction for Cyprus: {:.2}", prediction[0]);

    Ok(())
}
