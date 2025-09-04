use rasn_compiler::{
    prelude::*,
    OutputMode
};
use std::path::Path;

fn main() {
    let spec_src_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("i41/E1ap.asn");
    let spec_tgt_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec/e1ap.rs");
    match Compiler::<RasnBackend, _>::new()
        .add_asn_sources_by_path(vec![spec_src_path].iter())
        // set an output path for the generated rust code
        .set_output_mode(OutputMode::SingleFile(spec_tgt_path))
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
