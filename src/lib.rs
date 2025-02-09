//! pocsuite-rs is a vulnerability testing framework.

pub mod core;
pub mod http;
pub mod pocs;
pub mod utils;
pub mod ui;
pub mod discovery;

pub use crate::core::{Poc, PocConfig, PocResult, PocError};
pub use crate::http::HttpClient;
pub use crate::pocs::{ExamplePoc, PocManager};
pub use crate::ui::table::ResultTable;