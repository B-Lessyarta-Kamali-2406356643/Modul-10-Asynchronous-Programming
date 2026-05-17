# Experiment 1.2: Understanding how it works

## Screenshot
![Experiment 1.2](images/experiment-1-2.png)

## Explanation
Pada eksperimen ini, saya menambahkan satu println! tepat setelah spawner.spawn(...). Hasilnya, teks "Kamali's Komputer: hey hey" tampil di output sebelum teks dari task async. Hal ini terjadi karena spawner.spawn(...) hanya memasukkan task ke dalam queue executor, tetapi task tersebut belum langsung dijalankan sampai executor.run() dipanggil. Sementara itu, println! setelah spawn berada di alur utama program, sehingga dieksekusi terlebih dahulu. Setelah drop(spawner) dipanggil, executor mulai menjalankan task yang ada di queue. Task baru mencetak "Kamali's Komputer: howdy!", lalu menunggu TimerFuture selama dua detik, kemudian melanjutkan eksekusi dan mencetak "Kamali's Komputer: done!".