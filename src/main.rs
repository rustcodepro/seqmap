use crate::args::CommandParse;
use crate::args::Commands;
use clap::Parser;
mod args;
mod read;
mod sam;
mod view;

/*
Gaurav Sablok,
codeprog@icloud.com
*/

fn main() {
    //let fontgenerate = FIGfont::standard().unwrap();
    //let repgenerate = fontgenerate.convert("rustRET");
    //println!("{}", repgenerate.unwrap());
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::AlignmentView {
            alignmentfile,
            thread,
        } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command = mirrnaup().unwrap();
                println!("The machine learning model has finished:{}", command);
            });
        }
        Commands::SamAlignment { samfile, thread } => {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(thread.parse::<usize>().unwrap())
                .build()
                .unwrap();
            pool.install(|| {
                let command =
                    mirrna(annotationfile, psrnatarget, *expectationinput, predictfasta).unwrap();
                println!("The machine learning model has been completed: {}", command);
            })
        }
    }
}
