use std::sync::Arc;

use std::convert::AsRef;
use std::path::Path;
use std::fmt::Display;

pub fn main<P: AsRef<Path> + Display>(path: P) {

    let context = Arc::new(rune::default_context().unwrap());
    let options = rune::Options::default();

    let mut warnings = rune::Warnings::new();

    let unit = match rune::load_path(&*context, &options, &path.as_ref(), &mut warnings) {
        Ok(unit) => Arc::new(unit),
        Err(_) => return println!("There's no script at {} for me to run!", path)
    };

    let vm = runestick::Vm::new(context.clone(), unit.clone());

    let mut execution: runestick::VmExecution = vm.call(&["main"], ()).unwrap();
    let last = std::time::Instant::now();

    let result = execution.complete();

    let duration = std::time::Instant::now().duration_since(last);

    match result {
        Ok(result) => {
            println!("returned {:?} in ({:?})", result, duration);
            None
        },
        Err(error) => {
            println!("aborted! ({}) at ({:?})", error, duration);
            Some(error)
        }
    };

}
