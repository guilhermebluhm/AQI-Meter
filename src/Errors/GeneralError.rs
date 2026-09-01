use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum GeneralError{
    
    FalhaDeserializacao(String),
    FalhaProcessarRequisicaoHTTP(String),
    FalhaAoMontarTipo(String),
    
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self { 
            GeneralError::FalhaDeserializacao(e) => {write!(f, "Falha deserializacao: {}", e)},
            GeneralError::FalhaProcessarRequisicaoHTTP(e) => {write!(f, "Falha processar requisicao: {}", e)},
            GeneralError::FalhaAoMontarTipo(arg) => {write!(f, "{}", arg)}
        }
    }
}