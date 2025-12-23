use crate::view::SamFile;
use anyhow::Result;
use plotters::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::process::Command;

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl SamFile {
    pub fn viewsame(&self, idref: &str) -> Result<String, Box<dyn Error>> {
        let samopen = File::open(self.samfile.clone()).expect("file not present");
        let samread = BufReader::new(samopen);
        let mut filestruct: Vec<(String, String)> = Vec::new();
        for i in samread.lines() {
            let line = i.expect("file not present");
            let linevec = line.split("\t").map(|x| x.to_string()).collect::<Vec<_>>();
            let fileinsert: (String, String) = (linevec[0].clone(), linevec[9].clone());
            filestruct.push(fileinsert);
        }
        let mut filtered: Vec<(String, String)> = Vec::new();
        for i in filestruct.iter() {
            if i.0 == idref.to_string() {
                filtered.push(i.clone());
            }
        }

        let mut filewrite = File::create("extracted-reads.fasta").expect("file not present");
        for i in filtered.iter() {
            writeln!(filewrite, ">{}\n{}\n", i.0, i.1).expect("file not present");
        }

        let _ = Command::new("mafft")
            .arg("extracted-reads.fasta")
            .arg(">")
            .arg("aligned.fasta")
            .output()
            .expect("command failed");

        let alignmentread = File::open("aligned.fasta").expect("file not present");
        let alignment_read = BufReader::new(alignmentread);
        let mut alignmentseq: Vec<String> = Vec::new();
        for i in alignment_read.lines() {
            let line = i.expect("line not present");
            if line.starts_with(">") {
                continue;
            }
            if !line.starts_with(">") {
                alignmentseq.push(line.clone())
            }
        }
        let len = alignmentseq[0].len();
        let n_seq = alignmentseq.len() as f64;
        let bases = ['A', 'C', 'G', 'T'];
        let mut freqs: Vec<[f64; 4]> = vec![[0.0; 4]; len];
        for record in &alignmentseq {
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
                        [(pos as f32 - 0.4) as usize, y_bottom as usize],
                        [(pos as f32 + 0.4) as usize, (y_bottom + h) as usize],
                    )
                    .set_style(ShapeStyle::from(color).filled()),
                ))?;
                chart.draw_series(std::iter::once(Text::new(
                    letter.to_string(),
                    (pos as f64, y_bottom + h / 2.0),
                    ("Arial", 24).into_font().color(&WHITE),
                )))?;
                y_bottom += h;
            }
        }
        root.present()?;
        println!("Sequence logo saved to seqlogo.svg");

        Ok("The method has been implemented and the image has been saved".to_string())
    }
}
