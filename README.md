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