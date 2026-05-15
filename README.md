# 💧 Sistem Kontrol Pompa Air — Belajar Rust dari Proyek Nyata

> Proyek ini dirancang khusus untuk pemula yang ingin belajar bahasa pemrograman **Rust** melalui studi kasus nyata: sistem kontrol pompa air industri sederhana.

---

## 📋 Daftar Isi

- [Tentang Proyek](#-tentang-proyek)
- [Konsep Rust yang Dipelajari](#-konsep-rust-yang-dipelajari)
- [Cara Kerja Program](#-cara-kerja-program)
- [Struktur Kode](#-struktur-kode)
- [Prasyarat](#-prasyarat)
- [Instalasi & Menjalankan](#-instalasi--menjalankan)
- [Contoh Output](#-contoh-output)
- [Penjelasan Kode](#-penjelasan-kode)

---

## 🔍 Tentang Proyek

Program ini mensimulasikan sistem kontrol pompa air pada tangki industri. Pengguna memasukkan level air saat ini (dalam liter), lalu program akan:

- **Memvalidasi input** — menolak angka negatif, bukan angka, atau melebihi kapasitas
- **Menentukan status pompa** — MATI, NYALA, atau STANDBY
- **Menghitung estimasi waktu** pengisian tangki hingga penuh (jika pompa nyala)

Meski sederhana, proyek ini mencakup pola-pola penting yang sering ditemui di program Rust sungguhan.

---

## 🧠 Konsep Rust yang Dipelajari

| No | Konsep | Di mana dalam kode |
|----|--------|--------------------|
| 1 | **Struct** — mendefinisikan tipe data sendiri | `struct SistemPompa { ... }` |
| 2 | **impl** — menambahkan method ke struct | `impl SistemPompa { ... }` |
| 3 | **Loop & validasi input** | Fungsi `minta_input_angka()` |
| 4 | **Pattern matching** dengan `match` | Parsing input `Ok` / `Err` |
| 5 | **Ownership & borrowing** dengan `&self` | Method `hitung_estimasi_waktu` |
| 6 | **Tipe data numerik** `f64` | Semua perhitungan volume & waktu |
| 7 | **String vs &str** | `String::from(...)` vs literal string |
| 8 | **Kondisional** `if / else if / else` | Logika kontrol di `main()` |
| 9 | **Komputasi numerik** | Rumus `sisa_volume / debit_pompa` |
| 10 | **Standard library** `std::io` | Membaca input dari terminal |

---

## ⚙️ Cara Kerja Program

Program ini bekerja dengan alur logika yang sangat disiplin untuk memastikan keamanan data sistem kontrol:

1. **Inisialisasi Sistem**: Menetapkan parameter tetap seperti kapasitas (1000L), debit (5L/detik), dan batas level rendah (200L).
2. **Input & Validasi Berulang (Loop)**: 
   - Program masuk ke dalam perulangan tak terbatas (`loop`) untuk meminta input user.
   - **Pengecekan Tipe**: Harus berupa angka (f64).
   - **Pengecekan Rentang**: Angka tidak boleh negatif dan tidak boleh melebihi kapasitas tangki.
   - Program hanya akan lanjut jika input sudah memenuhi semua kriteria.
3. **Logika Kontrol (Decision Making)**:
   - **TANGKI PENUH**: Jika level $\geq$ kapasitas, pompa dimatikan.
   - **LEVEL RENDAH**: Jika level $<$ batas bawah, pompa menyala dan **Komputasi Numerik** dijalankan untuk menghitung estimasi waktu pengisian.
   - **STANDBY**: Jika di antara keduanya, pompa dalam posisi siaga.

Algoritma kontrol ini adalah bentuk sederhana dari **Finite State Machine (FSM)** — konsep yang sangat umum di sistem embedded dan IoT.

---

## 📁 Struktur Kode

```
pompa-air/
├── src/
│   └── main.rs        # Seluruh kode program
└── Cargo.toml         # Konfigurasi proyek Rust
```

### Ringkasan `main.rs`

```
main.rs
 ├── fn minta_input_angka(kapasitas)  → f64
 │       Validasi input berulang sampai sah
 │
 ├── struct SistemPompa
 │       nama_pompa, batas_level_rendah,
 │       kapasitas_tangki, debit_pompa
 │
 ├── impl SistemPompa
 │   └── fn hitung_estimasi_waktu(&self, level) → f64
 │           Komputasi numerik laju pengisian
 │
 └── fn main()
         Inisialisasi, ambil input, logika kontrol
```

---

## 🛠️ Prasyarat

Sebelum menjalankan proyek ini, pastikan kamu sudah menginstal **Rust** di komputer.

### Install Rust (semua platform)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> **Windows?** Unduh installer dari [https://rustup.rs](https://rustup.rs) lalu ikuti petunjuknya.

Verifikasi instalasi:

```bash
rustc --version
cargo --version
```

Jika muncul nomor versi, berarti Rust sudah siap digunakan! 

---

## 🚀 Instalasi & Menjalankan

### 1. Clone repositori ini

```bash
git clone https://github.com/username/pompa-air-rust.git
cd pompa-air-rust
```

### 2. Jalankan program

```bash
cargo run
```

Cargo akan otomatis mengkompilasi kode lalu menjalankannya. Tidak perlu langkah kompilasi terpisah.

### 3. Build saja (tanpa langsung jalan)

```bash
cargo build
```

File executable akan tersimpan di `target/debug/pompa-air`.

---

## 💻 Contoh Output

**Skenario 1 — Level Rendah (Pompa Nyala):**
```
--- Selamat Datang di Kontrol Pompa Utama A1 ---
Masukkan level air saat ini (liter):
150
Status: LEVEL RENDAH. Pompa NYALA.
Estimasi tangki penuh dalam: 170.0 detik
```

**Skenario 2 — Level Aman (Standby):**
```
--- Selamat Datang di Kontrol Pompa Utama A1 ---
Masukkan level air saat ini (liter):
500
Status: LEVEL AMAN. Pompa STANDBY.
```

**Skenario 3 — Tangki Penuh:**
```
--- Selamat Datang di Kontrol Pompa Utama A1 ---
Masukkan level air saat ini (liter):
1000
Status: TANGKI PENUH. Pompa MATI.
```

**Skenario 4 — Input Tidak Valid (diulang otomatis):**
```
Masukkan level air saat ini (liter):
abc
Input tidak valid, coba lagi :
-50
Level tidak boleh negatif!
2000
Level tidak boleh melebihi kapasitas tangki (1000 liter)!
300
Status: LEVEL AMAN. Pompa STANDBY.
```

---

## 📖 Penjelasan Kode

### 1. Struct — Membuat Tipe Data Sendiri

```rust
struct SistemPompa {
    nama_pompa: String,
    batas_level_rendah: f64,
    kapasitas_tangki: f64,
    debit_pompa: f64,
}
```

`struct` di Rust mirip seperti "blueprint" atau cetakan. Kita mendefinisikan sekumpulan data yang saling berkaitan menjadi satu kesatuan.

---

###  2. impl — Menambahkan Kemampuan ke Struct

```rust
impl SistemPompa {
    fn hitung_estimasi_waktu(&self, level_sekarang: f64) -> f64 {
        let sisa_volume = self.kapasitas_tangki - level_sekarang;
        // ...
    }
}
```

`impl` digunakan untuk menambahkan *method* (fungsi yang terikat pada struct). `&self` artinya method ini meminjam data struct tanpa mengambil kepemilikannya — ini adalah konsep **borrowing** di Rust.

---

### 3. Pattern Matching dengan `match`

```rust
match input_teks.trim().parse::<f64>() {
    Ok(num)  => { /* berhasil diparse */ },
    Err(_)   => println!("Input tidak valid, coba lagi :"),
}
```

`match` adalah cara Rust menangani kemungkinan hasil yang berbeda secara eksplisit. `parse()` mengembalikan tipe `Result<f64, Error>` yang harus ditangani — tidak bisa diabaikan begitu saja. Ini membuat kode Rust jauh lebih aman dari crash tak terduga.

---

### 4. Loop Tak Terbatas dengan Validasi

```rust
fn minta_input_angka(kapasitas: f64) -> f64 {
    loop {
        // ... baca dan validasi input
        // hanya `return` jika valid
    }
}
```

`loop` di Rust berjalan selamanya sampai ada `return` atau `break`. Pola ini sangat umum untuk membuat program yang terus meminta input sampai pengguna memberikan data yang benar.

---

### 5. Ownership — Konsep Unik Rust

```rust
let pompa_industri = SistemPompa { ... };

// &self dalam method = meminjam, tidak mengambil alih
pompa_industri.hitung_estimasi_waktu(level_input);

// pompa_industri masih bisa dipakai setelahnya!
println!("{}", pompa_industri.nama_pompa);
```

Ownership adalah fitur paling khas Rust. Setiap nilai punya satu pemilik. Jika kita hanya ingin "meminjam" data tanpa memindahkan kepemilikan, kita pakai `&` (reference/borrowing).

---

## Hasil Program

Berikut adalah ScreenShoot Hasil Program :

![Hasil Terminal](Screenshot%202026-05-15%20082954.png)

![Hasil Terminal](Screenshot%202026-05-15%20083252.png)
---
