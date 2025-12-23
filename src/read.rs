use crate::view::FastaFile;
use anyhow::Result;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl FastaFile {
    pub fn readalignment(self) -> Result<String, Box<dyn Error>> {
        let fileopen = File::open(self.pathname.clone()).expect("file not present");
        let fileread = BufReader::new(fileopen);
        let mut records: Vec<String> = Vec::new();
        for i in fileread.lines() {
            let line = i.expect("line not present");
            if !line.starts_with(">") {
                records.push(line.clone());
            }
        }
        let len = records[0].len();
        let n_seq = records.len() as f64;
        let bases = ['A', 'C', 'G', 'T'];
        let mut freqs: Vec<[f64; 4]> = vec![[0.0; 4]; len];
        for record in &records {
            for (i, base) in record.chars().enumerate() {
                match base as char {
                    'A' => freqs[i][0] += 1.0,
                    'C' => freqs[i][1] += 1.0,
                    'G' => freqs[i][2] += 1.0,
                    'T' => freqs[i][3] += 1.0,
                    _ => {}
                }
            }
        }
        for col in &mut freqs {
            let sum: f64 = col.iter().sum();
            if sum > 0.0 {
                for f in col.iter_mut() {
                    *f /= sum;
                }
            }
        }
        let mut ic: Vec<f64> = vec![];
        for col in &freqs {
            let mut h = 0.0;
            for &p in col {
                if p > 0.0 {
                    h -= p * p.log2();
                }
            }
            ic.push(2.0 - h);
        }
        let root = SVGBackend::new("seqlogo.svg", (1200, 400)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..len, 0.0..2.5)?;

        chart
            .configure_mesh()
            .x_desc("Position")
            .y_desc("Information content (bits)")
            .draw()?;
        for (pos, (&height, freq)) in ic.iter().zip(freqs.iter()).enumerate() {
            let mut y_bottom = 0.0;
            let mut sorted_idx: Vec<usize> = (0..4).collect();
            sorted_idx.sort_by(|&a, &b| freq[b].partial_cmp(&freq[a]).unwrap());
            for &idx in &sorted_idx {
                let f = freq[idx];
                let letter = bases[idx];
                if f < 0.01 {
                    continue;
                }
                let h = height * f;
                let color = match letter {
                    'A' => RED,
                    'C' => BLUE,
                    'G' => GREEN,
                    'T' => CYAN,
                    _ => BLACK,
                };
                chart.draw_series(std::iter::once(
                    Rectangle::new(
                        [(pos as f32 - 0.4) as f64, y_bottom as f64],
                        [(pos as f32 + 0.4) as f64, y_bottom + h as f64],
                    )
                    .set_style(ShapeStyle::from(color).filled()),
                ))?;
                chart.draw_series(std::iter::once(Text::new(
                    letter.to_string(),
                    (pos as f64, (y_bottom + h / 2.0) as f64),
                    ("Arial", 24).into_font().color(&WHITE),
                )))?;
                y_bottom += h;
            }
        }
        root.present()?;
        println!("Sequence logo saved to seqlogo.svg");
        Ok("The plot has been written".to_string())
    }
}
