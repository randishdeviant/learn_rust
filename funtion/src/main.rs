fn main() {
    println!("total harga: {}", hitung_total(200, 2));
    println!("{}", cek_angka_genap(5));
    println!("{}", cek_angka_positif(2));
}

fn hitung_total(harga: u32, jumlah: u32) -> u32 {
    harga * jumlah
}

fn cek_angka_genap(angka: i8) -> bool {
    angka % 2 == 0
}

fn cek_angka_positif(angka: i8) -> bool {
    angka > 0
}


