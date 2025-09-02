use rasn_compiler::prelude::*;
use rasn_compiler::OutputMode;
use std::path::PathBuf;

fn main() {
    match Compiler::<RasnBackend, _>::new()
        .add_asn_sources_by_path(vec![PathBuf::from("pdcp-sim/i41/E1ap.asn")].iter())
        // set an output path for the generated rust code
        .set_output_mode(OutputMode::SingleFile(PathBuf::from("pdcp-sim/asn/generated.rs")))
        .compile()
    {
        Ok(warnings /* Vec<Box<dyn Error>> */) => {
            if warnings.is_empty() {
                println!("Compiled successfully with no warnings.");
            } else {
                println!("Compiled successfully with {} warnings:", warnings.len());
                for warning in warnings {
                    println!(" - {:?}", warning);
                }
            }
            /* handle compilation warnings */
        }
        Err(error /* Box<dyn Error> */) => {
            eprintln!("Failed to compile: {:?}", error);
            std::process::exit(1);
            /* handle compilation error */
        }
    }
}
