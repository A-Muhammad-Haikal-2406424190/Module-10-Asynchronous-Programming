# Module 9 - Asynchronous Programming

## 1.1 Original timer from the book

Perubahan yang dilakukan:
- Mengadopsi kode executor + timer dari Async Book.
- Mengganti teks output dari `Ade's Komputer` menjadi `Muhammad Haikal's Komputer`.

Hasil `cargo run`:

```text
Muhammad Haikal's Komputer: howdy!
...delay 2 detik...
Muhammad Haikal's Komputer: done!
```

## 1.2 Understanding how it works

Perubahan yang dilakukan:
- Menambahkan satu `println!` tepat setelah `spawner.spawn(...)`:
  `Muhammad Haikal's Komputer: hey hey!`

Hasil `cargo run`:

```text
Muhammad Haikal's Komputer: hey hey!
Muhammad Haikal's Komputer: howdy!
...delay 2 detik...
Muhammad Haikal's Komputer: done!
```

Penjelasan:
- `hey hey!` dieksekusi langsung oleh thread `main`, karena berada di luar future async.
- Future di dalam `spawn` baru mulai diproses saat `executor.run()` berjalan.
- Karena itu `hey hey!` selalu muncul lebih dulu, kemudian `howdy!`, lalu setelah timer selesai muncul `done!`.

## 1.3 Multiple spawn and removing drop

Perubahan yang dilakukan:
- Menambahkan 3 task async lewat `spawner.spawn(...)`.
- Masing-masing task mencetak `howdy1/2/3` lalu `done1/2/3` setelah timer 2 detik.

Hasil `cargo run` (dengan `drop(spawner)`):

```text
Muhammad Haikal's Komputer: hey hey!
Muhammad Haikal's Komputer: howdy1!
Muhammad Haikal's Komputer: howdy2!
Muhammad Haikal's Komputer: howdy3!
Muhammad Haikal's Komputer: done1!
Muhammad Haikal's Komputer: done2!
Muhammad Haikal's Komputer: done3!
```

Hasil saat `drop(spawner)` dihapus (dijalankan dengan `timeout 8 cargo run`):

```text
Muhammad Haikal's Komputer: hey hey!
Muhammad Haikal's Komputer: howdy1!
Muhammad Haikal's Komputer: howdy2!
Muhammad Haikal's Komputer: howdy3!
Muhammad Haikal's Komputer: done1!
Muhammad Haikal's Komputer: done3!
Muhammad Haikal's Komputer: done2!
```

Lalu proses tidak selesai (timeout), karena receiver executor masih menunggu channel ditutup.

Penjelasan efek spawn/spawner/executor/drop:
- `spawn` mendaftarkan future sebagai task ke queue.
- `spawner` adalah pengirim task ke queue tersebut.
- `executor` mengambil task dari queue dan mem-poll sampai selesai.
- `drop(spawner)` menutup jalur pengiriman task baru; setelah queue habis, `executor.run()` bisa berhenti.
- Jika `drop(spawner)` tidak dipanggil, queue receiver terus menunggu task baru sehingga program terlihat "hang".
