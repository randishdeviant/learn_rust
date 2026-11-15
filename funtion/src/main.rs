fn main() {
   println!("total harga: {}", hitung_total(200, 2));
}

fn hitung_total(harga: u32, jumlah: u32) -> u32 {
  harga * jumlah
}