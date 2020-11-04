mod utils;

extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

use std::collections::HashMap;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init() {
    log("[Console Error Panic Hook] Initialised!");
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn calculate(n: i64, alpha: i64, beta: i64) -> i32 {
    let result = bsgs(n, alpha, beta);

    match result {
        Some(x) => {
            log("Found solution!");
            return x as i32
        },
        None => {
            log("No solution found.");
            return -1
        },
    }
}

fn mod_exp(base: i64, exp: i64, n: i64) -> i64 {
    if base == 0 {
        return 0;
    }

    let mut intermediate = 1;
    let mut b = base;
    let mut e = exp;

    while e != 0 {
        if e % 2 != 0 {
            intermediate = (intermediate * b) % n;
        }
        e /= 2;
        b = (b * b) % n;
    }

    return intermediate;
}

fn mod_inv(base: i64, n: i64) -> i64 {
    return mod_exp(base, n - 2, n);
}

/// Big-Step Small-Step
/// 
/// More efficient solver for discrete logarithm problem.
pub fn bsgs(n: i64, alpha: i64, beta: i64) -> Option<i64> {
    let m = (n as f64).sqrt().ceil() as i64;
    let mut precomp = HashMap::new();

    for j in 0..m {
        precomp.insert(mod_exp(alpha, j, n) as i64, j);
    }

    let invgenerator = mod_inv(mod_exp(alpha, m, n), n);
    let mut y: i64 = beta as i64;

    for i in 0..m {
        if precomp.contains_key(&y) {
            return Some((i * m) + precomp.get(&y).unwrap());
        }

        y = y * (invgenerator as i64) % (n as i64);
    }

    return None;
}