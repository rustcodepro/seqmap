#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct FastaFile {
    pub pathname: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct SamFile {
    pub samfile: String,
}
