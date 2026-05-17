# Experiment 1.2: Understanding how it works

## Screenshot
![Experiment 1.2](images/experiment-1-2.png)

## Explanation
Pada eksperimen ini, saya menambahkan satu println! tepat setelah spawner.spawn(...). Hasilnya, teks "Kamali's Komputer: hey hey" tampil di output sebelum teks dari task async. Hal ini terjadi karena spawner.spawn(...) hanya memasukkan task ke dalam queue executor, tetapi task tersebut belum langsung dijalankan sampai executor.run() dipanggil. Sementara itu, println! setelah spawn berada di alur utama program, sehingga dieksekusi terlebih dahulu. Setelah drop(spawner) dipanggil, executor mulai menjalankan task yang ada di queue. Task baru mencetak "Kamali's Komputer: howdy!", lalu menunggu TimerFuture selama dua detik, kemudian melanjutkan eksekusi dan mencetak "Kamali's Komputer: done!".

# Experiment 1.3: Multiple Spawn and removing drop

## Screenshot

Multiple spawn with drop(spawner);:
![Experiment 1.3 Multiple Spawn](images/experiment-1-3-multiple-spawn.png)

Without drop(spawner);:
![Experiment 1.3 Without Drop](images/experiment-1-3-without-drop.png)

## Explanation

Pada eksperimen ini, saya mereplikasi spawner.spawn(...) menjadi tiga task async. Setiap task mencetak pesan awal, menunggu TimerFuture selama dua detik, lalu mencetak pesan selesai. Efek dari spawning adalah task dimasukkan ke dalam queue executor untuk dijalankan. Karena ada tiga task, executor akan menjalankan task-task tersebut dan setiap task dapat berhenti sementara ketika menunggu timer.

Spawner berfungsi untuk memasukkan task baru ke dalam task queue. 
Executor berfungsi untuk mengambil task dari queue, melakukan polling terhadap future, dan menjalankan future sampai selesai. 

Ketika future belum selesai, task akan disimpan kembali agar bisa dilanjutkan ketika sudah siap. drop(spawner) berfungsi untuk menutup sisi pengirim task sehingga executor mengetahui bahwa tidak akan ada task baru lagi.

Ketika drop(spawner) dikomen/dihapus, program tidak langsung selesai walaupun task sudah mencetak output. Hal ini terjadi karena channel pengirim masih hidup, sehingga executor masih menunggu kemungkinan ada task baru yang masuk. Ketika drop(spawner) dikembalikan, executor dapat berhenti setelah semua task di queue selesai dijalankan.

# Experiment 2.1: Original code, and how it run

## Screenshot

![Experiment 2.1 Original Chat](images/experiment-2-1-original-chat.png)

## How to Run

To run the broadcast chat application, first enter the `chat-async` directory.

```bash
cd chat-async
```

Kemudian jalankan server dengan command berikut:
```bash
cargo run --bin server
```

Server akan berjalan pada alamat 127.0.0.1:2000.

Setelah server berjalan, buka tiga terminal baru untuk menjalankan tiga client. Pada masing-masing terminal client, masuk ke folder project chat:
```bash
cd chat-async
```

Lalu jalankan command berikut:
```bash
cargo run --bin client
```

Setelah semua client berhasil terhubung ke server, ketik pesan dari salah satu client. Pesan tersebut akan dikirim ke server, lalu server akan membroadcast pesan tersebut ke client-client yang sedang terhubung.

### Penjelasan

Pada eksperimen ini, saya menjalankan satu server dan tiga client sesuai instruksi tutorial. Server berperan sebagai pusat koneksi yang menerima pesan dari client melalui websocket. Setiap client terhubung ke server menggunakan alamat ws://127.0.0.1:2000.

Ketika salah satu client mengetik pesan, pesan tersebut dikirim ke server. Server kemudian menerima pesan tersebut dan mengirimkannya kembali melalui broadcast channel ke client-client yang terhubung. Dengan demikian, pesan yang diketik dari satu client dapat terlihat di client lainnya.

Eksperimen ini menunjukkan penggunaan asynchronous programming pada aplikasi chat. Server perlu menangani beberapa client secara bersamaan, menerima pesan dari client, dan mengirimkan pesan ke client lain tanpa harus menunggu satu proses selesai terlebih dahulu. Karena itu, websocket dan asynchronous programming cocok digunakan untuk kasus broadcast chat seperti ini.

# Experiment 2.2: Modifying port

## Screenshot

![Experiment 2.2 Modifying Port](images/experiment-2-2-modifying-port.png)

## Penjelasan

Pada eksperimen ini, saya mengubah port websocket dari 2000 menjadi 8080. Perubahan dilakukan pada dua sisi, yaitu server dan client. Pada sisi server, port diubah di file src/bin/server.rs, tepatnya pada bagian TcpListener::bind("127.0.0.1:8080"). Pada sisi client, port diubah di file src/bin/client.rs, tepatnya pada bagian URI websocket menjadi ws://127.0.0.1:8080.

Perubahan perlu dilakukan pada kedua sisi karena websocket adalah koneksi antara client dan server. Jika hanya server yang diubah tetapi client masih mengarah ke port lama, client tidak akan bisa terhubung. Sebaliknya, jika hanya client yang diubah tetapi server masih berjalan di port lama, koneksi juga gagal.

Protocol yang digunakan tetap sama, yaitu ws. Protocol ini didefinisikan pada URI websocket di file src/bin/client.rs, yaitu pada bagian ws://127.0.0.1:8080. Setelah port diubah pada server dan client, aplikasi tetap dapat berjalan dengan baik, dan pesan dari satu client masih dapat diterima oleh client lain.