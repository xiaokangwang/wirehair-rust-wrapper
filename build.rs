extern crate cc;

fn main() {
    let mut wirehair_build = cc::Build::new();

    wirehair_build.cpp(true)
        .file("src/wirehair/wirehair.cpp")
        .file("src/wirehair/gf256.cpp")
        .file("src/wirehair/WirehairCodec.cpp")
        .file("src/wirehair/WirehairTools.cpp")
        .include("src/wirehair")
        .flag("-msse4.1")
        .shared_flag(true);

    match std::env::var("TARGET").unwrap().as_str() {
        "x86_64-unknown-linux-gnu" => {
            println!("sse4.1 not disabled")
        }
        _ => {
            println!("sse4.1 disabled");

            wirehair_build.define("GF256_TARGET_MOBILE", None);
        }
    }

    wirehair_build.compile("wirehair");
}
