use std::sync::{Arc,Mutex};

// FnMut vs Fn — mutable state inside a closure
//static mut counts: HashMap<String, usize> = HashMap::new(Mutex::new());
/* above didn't work before */


// 1. Wrap your map in an Arc and Mutex
let counts = Arc::new(Mutex::new(HashMap::new()));

// 2. Clone the reference for the closure
let counts_clone = Arc::clone(&counts);

registry.register(move |line| {
	//3. Lock the mutex inside the closure
	let mut map = match counts_clone.lock(){                       
        	Ok(guard) => guard,
        	Err(poisoned) => {
                	eprintln!("mutex poisoned, recovering...");
                	poisoned.into_inner() //recovering
        	}
	}
	*map.entry(line.level.to_string()).or_insert(0) +=1;
});

/* Commenting out this
registry.register(|line| {
    *counts.entry(line.level.to_string()).or_insert(0) += 1;
});
*/
