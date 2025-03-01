#![feature(coverage_attribute)]

pub mod com;
pub mod commands;
pub mod message_receiver;
pub mod parser;
pub mod data;
pub mod plate_setup;
#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "python")]
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[cfg(feature = "python")]
#[pymodule]
mod qslib_rs {

    #[pymodule_export]
    use crate::python::PyQSConnection;

    #[pymodule_export]
    use crate::python::PyMessageResponse;

    #[pymodule_export]
    use crate::python::PyLogReceiver;

    #[pymodule_export]
    use crate::parser::OkResponse;

    #[pymodule_export]
    use crate::python::UnexpectedMessageResponse;

    #[pymodule_export]
    use crate::python::DisconnectedBeforeResponse;

    #[pymodule_export]
    use crate::python::CommandError;

    #[pymodule_export]
    use crate::python::QslibException;

    #[pymodule_export]
    use crate::python::CommandResponseError;

    
    
}
