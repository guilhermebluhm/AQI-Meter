use std::fmt;

#[derive(Debug)]
pub enum AppError{
    FalhaMontagemDadosAplicacao(String),
}

impl fmt::Display for AppError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self { 
            AppError::FalhaMontagemDadosAplicacao(e) => {write!(f, "{}", *e)}
        }
    }
}