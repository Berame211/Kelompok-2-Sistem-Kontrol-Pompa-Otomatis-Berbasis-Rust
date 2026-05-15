use std::io; // Library untuk input/output

fn minta_input_angka(kapasitas: f64) -> f64 {
    loop {
        let mut input_teks = String::new();
        io::stdin().read_line(&mut input_teks).expect("Gagal membaca input");
        match input_teks.trim().parse::<f64>() {
            Ok(num) => {
                if num < 0.0 {
                    println!("Level tidak boleh negatif!");
                } else if num > kapasitas {
                    println!("Level tidak boleh melebihi kapasitas tangki ({:.0} liter)!", kapasitas);
                } else {
                    return num;
                }
            },
            Err(_) => println!("Input tidak valid, coba lagi :"),
        }
    }
}

// Poin 3: Pemrograman Berbasis Objek (Struct)
struct SistemPompa {
    nama_pompa: String,
    batas_level_rendah: f64,
    kapasitas_tangki: f64, // Liter
    debit_pompa: f64, 
}

impl SistemPompa {
    // Poin 4: Komputasi Numerik
    // Fungsi untuk menghitung sisa waktu sampai penuh
    fn hitung_estimasi_waktu(&self, level_sekarang: f64) -> f64 {
        let sisa_volume = self.kapasitas_tangki - level_sekarang;
        if sisa_volume <= 0.0 {
            0.0
        } else {
            sisa_volume / self.debit_pompa
        }
        
    }
}

fn main() {
    // Inisialisasi objek
    let pompa_industri = SistemPompa {
        nama_pompa: String::from("Pompa Utama A1"),
        kapasitas_tangki: 1000.0,
        debit_pompa: 5.0,
        batas_level_rendah: 200.0,
    };

    println!("--- Selamat Datang di Kontrol {} ---", pompa_industri.nama_pompa);
    
    // Meminta input level air
    println!("Masukkan level air saat ini (liter): ");
    let level_input = minta_input_angka(pompa_industri.kapasitas_tangki);

    // Logika Kontrol (Poin 2: Implementasi Algoritma)
    if level_input >= pompa_industri.kapasitas_tangki {
        println!("Status: TANGKI PENUH. Pompa MATI.");
    } else if level_input < pompa_industri.batas_level_rendah {
        println!("Status: LEVEL RENDAH. Pompa NYALA.");
        let waktu = pompa_industri.hitung_estimasi_waktu(level_input);
        println!("Estimasi tangki penuh dalam: {:.1} detik", waktu);
    } else {
        println!("Status: LEVEL AMAN. Pompa STANDBY.");
    }
}
