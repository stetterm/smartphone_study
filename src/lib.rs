pub mod stats {

    fn average(data: &Vec<f64>) -> f64 {
        let mut avg: f64 = 0.;
        for value in data.iter() {
            avg += value;
        }
        avg / data.len() as f64
    }

    pub fn sample_stdev(data: &Vec<f64>) -> f64 {
        let avg = average(data);
        let mut stdev: f64 = 0.;
        for value in data.iter() {
            stdev += (value - avg).powi(2);
        }
        (stdev / (data.len() - 1) as f64).powf(0.5)
    }

    pub fn sample_correlation(data1: &Vec<f64>, data2: &Vec<f64>) -> f64 {
        assert!(data1.len() == data2.len());
        let (avg1, avg2) = (average(&data1), average(&data2));
        let mut covariance: f64 = 0.;
        for i in 0..data1.len() {
            covariance += (data1[i] - avg1) * (data2[i] - avg2);
        }
        let covariance = covariance / (data1.len() - 1) as f64;
        let (stdev1, stdev2) = (sample_stdev(&data1), sample_stdev(&data2));
        covariance / (stdev1 * stdev2)
    }
}