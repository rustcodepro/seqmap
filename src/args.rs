use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "rnaml",
    version = "1.0",
    about = "Machine learning and Target Prediction from RNAs
           ************************************************
           Author Gaurav Sablok,
           Email: codeprog@icloud.com
          ************************************************"
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// aligment view
    AlignmentView {
        /// alignment file
        alignmentfile: String,
        /// number of threads
        thread: String,
    },
    /// sam alignment view
    SamAlignment {
        /// sam file
        samfile: String,
        /// number of threads
        thread: String,
    },
}
