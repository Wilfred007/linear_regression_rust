📊 GDP vs. Life Satisfaction Prediction (Rust + SmartCore)

This project demonstrates how to use Rust and the SmartCore machine learning library to perform a simple linear regression — predicting life satisfaction based on a country's GDP per capita.

It’s inspired by classical data analysis examples from OECD datasets (like the “GDP vs Life Satisfaction” example in Python’s scikit-learn tutorials), but fully implemented in Rust for performance and safety.

🚀 Overview

This program:

Reads a CSV file (country_stats.csv) containing data about countries, GDP per capita, and life satisfaction.

Uses SmartCore’s LinearRegression model (similar to sklearn.linear_model.LinearRegression() in Python) to train a model.

Predicts the life satisfaction of a specific country (in this example, Cyprus) based on its GDP.

🧠 How It Works
1️⃣ Data Input

The program expects a CSV file with this format:

Country	GDP per capita	Life satisfaction
France	38429	6.5
Germany	47603	7.0
Japan	41289	5.9
Cyprus	22587	?

The CSV must include headers, as the reader is configured with .has_headers(true).

2️⃣ Data Processing

The function read_csv():

Reads the CSV file.

Parses each line.

Extracts GDP and life satisfaction as floating-point numbers.

Stores them in two Vec<f64> collections for training.

let (gdp, life_satisfaction) = read_csv("country_stats.csv")?;

3️⃣ Model Training

Using SmartCore’s LinearRegression, we train a model:

let lr = LinearRegression::fit(&x_matrix, &y, Default::default())?;


Here:

x_matrix → GDP per capita values (features)

y → life satisfaction scores (targets)

The model learns a linear relationship between GDP and life satisfaction.

Mathematically:

LifeSatisfaction
=
𝑎
+
𝑏
×
GDP
LifeSatisfaction=a+b×GDP
4️⃣ Prediction

After training, the model predicts life satisfaction for Cyprus (GDP per capita = 22587):

let x_new = DenseMatrix::from_2d_vec(&vec![vec![22587.0]]);
let prediction = lr.predict(&x_new)?;
println!("Predicted life satisfaction for Cyprus: {:.2}", prediction[0]);


If your data is realistic, you’ll get a value around 5.5 to 6.0 — indicating moderate life satisfaction for that GDP level.

🧩 Code Explanation
Section	Purpose
read_csv()	Reads and parses CSV into two vectors
DenseMatrix::from_2d_vec()	Converts a 2D vector into a matrix SmartCore can use
LinearRegression::fit()	Trains a linear regression model
lr.predict()	Makes predictions for new data points
📦 Dependencies

Add these to your Cargo.toml:

[dependencies]
smartcore = "0.3.0"
csv = "1.3.0"


(You may update versions as needed.)

🧰 Project Structure
gdp_life_regression/
├── Cargo.toml
├── src/
│   └── main.rs
└── country_stats.csv

🧪 Example Run
$ cargo run
Predicted life satisfaction for Cyprus: 5.73


(The exact number depends on your dataset.)

📈 Visualization (Conceptual)

If you were to visualize this data:

Life Satisfaction ↑
                   • Germany
                • France
            • Japan
        • Cyprus
──────────────────────────────→ GDP per Capita


The regression model fits a straight line through these data points — showing how life satisfaction tends to increase with GDP.

🧵 Notes

This project showcases SmartCore’s API design, which mirrors scikit-learn, but in Rust.

It’s an excellent example of how to use Rust for machine learning, data analysis, and predictive modeling with full type safety and zero-cost abstractions.