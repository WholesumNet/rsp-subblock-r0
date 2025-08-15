// use c_kzg::{KzgSettings};
// use std::env;
// use std::path::Path;

// trait Conversions {
//     pub fn to_bytes(&self) -> Vec<u8>;
// }

// impl Conversions for KzgSettings {
//     fn to_bytes(&self) -> Vec<u8> {
//         let size_max_length = core::mem::size_of::<u64>();
//         let size_roots_of_unity = core::mem::size_of::<fr_t>() * self.max_width as usize;
//         let size_g1_values = core::mem::size_of::<g1_t>() * FIELD_ELEMENTS_PER_BLOB;
//         let size_g2_values = core::mem::size_of::<g2_t>() * 65; // Magic: '#define TRUSTED_SETUP_NUM_G2_POINTS 65'

//         let length = size_max_length + size_roots_of_unity + size_g1_values + size_g2_values;
//         let mut data: Vec<u8> = Vec::new();
//         data.resize(length, 0);
//         let mut pos = 0;

//         data[..size_max_length].copy_from_slice(&self.max_width.to_be_bytes());
//         pos += size_max_length;

//         data[pos..pos + size_roots_of_unity].copy_from_slice(unsafe {
//             core::slice::from_raw_parts(self.roots_of_unity as *const u8, size_roots_of_unity)
//         });
//         pos += size_roots_of_unity;

//         data[pos..pos + size_g1_values].copy_from_slice(unsafe {
//             core::slice::from_raw_parts(self.g1_values as *const u8, size_g1_values)
//         });
//         pos += size_g1_values;

//         data[pos..pos + size_g2_values].copy_from_slice(unsafe {
//             core::slice::from_raw_parts(self.g2_values as *const u8, size_g2_values)
//         });

//         data
//     }
// }

fn main() {
    // let out_dir_env = env::var_os("OUT_DIR").unwrap();
    // let out_dir = Path::new(&out_dir_env);
    // let kzg_raw_path = out_dir.join("kzg_settings_raw.bin");
    // let kzg_trusted_setup = Path::new("./trusted_setup.txt");
    // let kzg_settings = KzgSettings::load_trusted_setup_file(kzg_trusted_setup).unwrap();
    // let kzg_setup_data = kzg_settings.to_bytes().to_vec();
    // std::fs::write(&kzg_raw_path, kzg_setup_data).unwrap();
    // env::set_var("KZG_FILE_PATH", kzg_raw_path.to_string_lossy().to_string());

    risc0_build::embed_methods();
}
