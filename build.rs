extern crate winapi_tlb_bindgen;
fn main() {
    let gbdaaut_rs = {
        let gbdaaut_rs = std::env::var_os("OUT_DIR").unwrap();
        let mut gbdaaut_rs: std::path::PathBuf = gbdaaut_rs.into();
        gbdaaut_rs.push("gbdaaut.rs");
        let gbdaaut_rs = std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(gbdaaut_rs).unwrap();
        std::io::BufWriter::new(gbdaaut_rs)
    };


    let _ =
        winapi_tlb_bindgen::build(
            std::path::Path::new(r"./src/gbda_aut.tlb"),
            false,
            gbdaaut_rs,
        ).unwrap();
}