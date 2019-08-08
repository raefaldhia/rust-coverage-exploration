use std::io::{self, Read};

fn b_selection() -> io::Result<()> {
	// mencari bilangan terbesar dari 3 buah bilangan
	let bil1: i32 = 50;
	let bil2: i32 = 40;
	let bil3: i32 = 30;

    println!("bilangan terbesar adalah {}", cariMax3Bil_05(bil1, bil2, bil3));
		
	// menghitung nilai mutu
	let uts: f64 = 65.0;
	let uas: f64 = 40.0;
	let tugas: f64 = 30.0;
	let jumHadir: i32 = 10;
	println!("nilai Mutu{}", hitNilaiMutu_06(uts, uas, tugas, jumHadir));

	// menghitung jumlah hari dalam suatu bulan pada tahun tertentu
	
    let mut month: String = String::new();
	io::stdin().read_line(&mut month)?;
    let mut year: String = String::new();
	io::stdin().read_line(&mut year)?;
	let numDays: i32 = getNumberDayMonth_18(month.trim().parse::<i32>().unwrap_or(0), year.trim().parse::<i32>().unwrap_or(0));
	if (numDays>0) {
		println!("{}", numDays);
    } else {
		println!("Invalid month.");
	}
	Ok(())
}

fn cariMax3Bil_05(A: i32, B: i32, C: i32) -> i32 {		
	let mut max: i32 = A;
	if (max < B) {
		max = B;
	}
	if (max < C) {
		max = C;
	}
	return max;
}

fn hitNilaiMutu_06(uts: f64, uas: f64, tugas: f64, hadir: i32) -> char {
	let mut nilaiMutu: char = ' ';
	
	let nilaiHadir: f32 = hadir as f32 / 14.0 * 100.0;
	let nilai: f32 = (0.3 * uts as f32) + (0.4 * uas as f32) + (0.2 * tugas as f32) + (0.1 * nilaiHadir as f32);
		
	if(nilai >= 85.0) {
		nilaiMutu = 'A';
	}else if (nilai >= 70.0) {
		nilaiMutu = 'B';
	} else if (nilai >= 55.0) {
		nilaiMutu = 'C';
	}else if (nilai >= 40.0) {
		nilaiMutu = 'D';
	} else {
		nilaiMutu = 'E';
	}
	return nilaiMutu;
}
	
fn hitNilaiMutu_062(uts: f64, uas: f64, tugas: f64, hadir: i32) -> char{
	let mut nilaiMutu: char = ' ';
	
	let nilaiHadir: f64 = hadir as f64 / 14.0 * 100.0;
	let nilai: f64 = (0.3 * uts) + (0.4 * uas) + (0.2 * tugas) + (0.1 * nilaiHadir);
		
	if(nilai >= 85.0) {
		nilaiMutu = 'A';
	}else { 
		if (nilai >= 70.0) {
			nilaiMutu = 'B';
		} else {
			if (nilai >= 55.0) {
				nilaiMutu = 'C';
			}else {
				if (nilai >= 40.0) {
					nilaiMutu = 'D';
				} else {
					nilaiMutu = 'E';
				}
			}
		}
	}
	return nilaiMutu;
}

fn getUpah_063(jamMasuk: i32, jamKeluar: i32) -> i32 {
	let mut lama: i32;
    let mut biaya: i32;
	
	if (jamKeluar > jamMasuk) {
		lama = jamKeluar - jamMasuk;
	}else if (jamMasuk > jamKeluar) {
		lama = 12 - jamMasuk + jamKeluar;
	}else{
		lama = 0;
	} 
	if (lama <= 2) {
		biaya = 2000;
	}else {
		biaya = 2000 + ((lama-2) * 500);
	}
	return biaya;
}

fn menentukanBilGanjil_064(bil: i32) {
	if (bil % 2 > 0) {
		print!("Bilangan Ganjil");
	}else{
		print!("Bilangan Genap");
	}
}
	
fn getDayFromNumber_07(idxDay: i32) -> &'static str {
    let mut day: &'static str = "";

	match idxDay {
		1 => day = "Senin",
		2 => day = "Selasa",
		3 => day = "Rabu",
		4 => day = "Kamis",
		5 => day = "Jumat",
		6 => day = "Sabtu",
		7 => day = "Minggu",
		_ => (),
	}
	return day;
}

fn tukarNominalUang_08(nominalUang: i32, totalBelanja: i32) {
	let mut kembalian: i32;
	let mut pec50K: i32 = 0;
	let mut pec10K: i32 = 0; 
	let mut pec5K: i32 = 0; 
	let mut pec1K: i32 = 0;
	if (nominalUang > totalBelanja) { 
		kembalian = totalBelanja - nominalUang; 
		
		if(kembalian > 0){
			pec50K = kembalian / 50000; 
			pec10K = (kembalian % 50000) / 10000;
			pec5K = ((kembalian % 50000) % 10000) / 5000;
			pec1K = (((kembalian % 50000) % 10000) % 5000) / 1000;
		}
		println!("Pecahan 50 ribu :{}lembar", pec50K);
		println!("Pecahan 10 ribu :{}lembar", pec10K);
		println!("Pecahan 5 ribu :{}lembar", pec5K);
		println!("Pecahan seribu :{}lembar", pec1K);
	}
}

fn getKuadran_09(x: i32, y: i32) -> i32 {
	let mut kuadran: i32 = 0;
	let mut isOnSB: bool = false;
	
	if (x==0 && y==0) {
		isOnSB = true;
	}
	if (!isOnSB) {
		if (x>0 && y>0) {
			kuadran = 1;
	   }else if(x<0 && y>0) {
		   kuadran = 2;
	   }else if(x<0 && y<0) {
		   kuadran = 3;
	   }else if(x>0 && y<0){
		   kuadran = 4;
	   }
	}
	return kuadran;
}
	
fn tampilSuhu_10(suhu: i32) {
	if (suhu <0) {
        println!("Cair");
	}else {
		if ( suhu <= 100 ) {
			println!("Padat");
        }else{
          	println!("Gas");
        }
    }
}

fn getNumberDayMonth2_10(idxMonth: i32, year: i32) -> i32{
	let mut numDays: i32 = 0;
	
	if ((idxMonth == 1) || (idxMonth == 3) || (idxMonth == 5) || (idxMonth == 1) || 
			(idxMonth == 8) || (idxMonth == 10) || (idxMonth == 12)) { 
		numDays = 31; 
	}else if ((idxMonth == 4) || (idxMonth == 6) || (idxMonth == 9) || (idxMonth == 11)){ 
		numDays = 30; 
	}else if ((idxMonth == 2)){ 
		if (((year % 4 == 0) && (year % 100 > 0)) || (year % 400 == 0)) {
			numDays = 29;
		}else {
			numDays = 28;
		} 
	}
	return numDays;
}

fn getSizeKaos_12(T: i32, BB: i32) -> char {
	let mut size: char = ' ';

	if (T > 170) {
		if ( (BB > 60) && (BB <= 80) ) {
			size = 'X';
		}
	}else { 
	    if (T > 150) {
	        if (BB <= 80) {
	    	    size = 'L';
	    	}
	    }else{ 
	        size = 'M';
	    }
	}
	return size;
}

fn calCulateGajih_17(gol: char, durasiJamKerja: i32) -> i32 {	
	let mut upah: i32 = 0;
	match gol {
		'A' => { 
			upah = durasiJamKerja * 10000;
			if (durasiJamKerja > 40) {
			    upah = upah + (durasiJamKerja * 5000);
			}
        },
		'B' => {
			upah = durasiJamKerja * 7500;
			if (durasiJamKerja > 40) {
			    upah = upah + (durasiJamKerja * 4000);
			}
		},
		'C' => {
			upah = durasiJamKerja * 5000;
		    if (durasiJamKerja > 40) {
			    upah = upah + (durasiJamKerja * 3000);
			}
		},
		'D' => { 
			upah = durasiJamKerja * 2500;
			if (durasiJamKerja > 40) {
			    upah = upah + (durasiJamKerja * 2000);
			}
        },
		_ => print!("Golongan tidak ada"),
	}
	return upah;
}
	
fn getNumberDayMonth_18(idxMonth: i32, year: i32) -> i32 {
	let mut numDays: i32 = 0;
	
	match idxMonth {
       	1  |
       	3  |
       	5  |
       	7  |
       	8  |
       	10 |
       	12 => numDays = 31,
       	4  | 
       	6  |
       	9  | 
       	11 => numDays = 30,
       	2 => {
       		if (((year % 4 == 0) && !(year % 100 == 0)) || (year % 400 == 0)) {
       			numDays = 29;

			} else {
       			numDays = 28;
			}
       	},
       	_ => numDays = 0,
    }
	return numDays;
}
	
fn OutputOperasiPilihan_19(pil: char, bil: i32) {
	// melakukan operasi sesuai dengan pilihan
	// Pil A, --> cek bilangan apakah ganjil, genap
	// Pil B, --> cek bilangan apakah positif, negatif atau bilangan nol
	// Pil C, --> Pangkat dua, jika bilangan positif
	// Pil D, --> Absolutkan, jika bilangan negatif
	// Pil E, --> menampilkan bilangan saja
	// lain-lain --> inputan salah
	// tampilkan hasil operasi jika sudah melakukan pengoperasian
	
	let mut hasil: i32;
	match pil {
		'A' =>  if (bil % 2 > 0) {
				    println!("bil ({}) adalah bilangan Ganjil", bil);
				}else{
				    println!("bil ({}) adalah bilangan Genap", bil);  
				}
        ,
		'B' =>  if (bil > 0) {
				    println!("bil ({}) adalah bilangan Positif", bil);
		  		}else if(bil < 0) {
		  		    println!("bil ({}) adalah bilangan Negatif", bil);  
		  		}else{
		  		    println!("bil ({}) adalah bilangan nol", bil);  
		  		}
        ,
		'C' =>  if (bil > 0) {
				    hasil = bil * bil;
				    println!("{}^ 2 = {}", bil, hasil);
  		  		}
        ,
		'D' =>  if (bil < 0) {
					hasil = bil * (-1);
	  		  	}
        ,
		'E' => println!("{}", bil),
		_ => println!("Pilihan Salah"),
	}
}
	
fn OutputSwitchSwitch_20(abjad: char, bil: i32) {	
	// print abjad dari A s.d E dan angka dari 1 s.d 5 sesuai dengan pasangannya
	// A1 --> cetak Huruf Perama - satu
	// selain itu masuk pada kategori lain-lain
	
	match abjad {
		'A' =>	match bil {
				    1 =>  print!("Huruf Pertama - satu"),
                    2 =>  print!("Huruf Pertama - dua"),
					3 =>  print!("Huruf Pertama - tiga"),
					4 =>  print!("Huruf Pertama - empat"),
					5 =>  print!("Huruf Pertama - lima"),
					_ => print!("Pasangan A tidak ada"),
                }
        ,
		'B' =>	match bil {
					1 =>  print!("Huruf Kedua - satu"),
					2 =>  print!("Huruf Kedua - dua"),
					3 =>  print!("Huruf Kedua - tiga"),
					4 =>  print!("Huruf Kedua - empat"),
					5 =>  print!("Huruf Kedua - lima"),
					_ => print!("Pasangan B tidak ada"),
				   }
        ,
		'C' =>	match bil {
					1 =>  print!("Huruf Ketiga - satu"),
					2 =>  print!("Huruf Ketiga - dua"),
					3 =>  print!("Huruf Ketiga - tiga"),
					4 =>  print!("Huruf Ketiga - empat"),
					5 =>  print!("Huruf Ketiga - lima"),
					_ => print!("Pasangan C tidak ada"),
				}
        ,
		'D' =>	match bil {
					1 =>  print!("Huruf Keempat - satu"),
					2 =>  print!("Huruf Keempat - dua"),
					3 =>  print!("Huruf Keempat - tiga"),
					4 =>  print!("Huruf Keempat - empat"),
					5 =>  print!("Huruf Keempat - lima"),
				    _ => print!("Pasangan D tidak ada"),
                }
        ,
		'E' =>	match bil {
					1 =>  print!("Huruf Kelima - satu"),
					2 =>  print!("Huruf Kelima - dua"),
					3 =>  print!("Huruf Kelima - tiga"),
					4 =>  print!("Huruf Kelima - empat"),
					5 =>  print!("Huruf Kelima - lima"),
					_ => print!("Pasangan E tidak ada"),
                }
        ,
		_ => print!("Pasangan tidak terbentuk"),
	}
}

fn OutputSwitchSwitch_20_1(abjad: char, bil: i32) {	
	// print abjad dari A s.d E dan angka dari 1 s.d 5 sesuai dengan pasangannya
	// A1 --> cetak Huruf Perama - satu
	// selain itu masuk pada kategori lain-lain
		
	match abjad {
		'A' =>	match bil {
					1 => print!("Huruf Pertama - satu"),
					2 => print!("Huruf Pertama - dua"),
					3 => print!("Huruf Pertama - tiga"), 
					_ => print!("Pasangan A tidak ada"),
				}
		,
		'B' =>	match bil {
					1 => print!("Huruf Kedua - satu"),
					2 => print!("Huruf Kedua - dua"),
					_ => print!("Pasangan B tidak ada"),
				}
		,
		'C' => print!("Huruf Ketiga - satu"),
		'D' =>  print!("Huruf Keempat"),
		_ => print!("Pasangan tidak terbentuk"),
	}
}

// Selection Cukup Rumit
fn nilaiTerbilang(bil: i32) -> String {
	// mengkonversikan sebuah bilangan dengan range -9999 s.d 9999
	let mut terbilang: String = String::new();
	let ribu: usize;
	let ratus: usize;
	let puluh: usize;
	let satuan: usize;
	let terbilangSatuan: &[&'static str] = &["", "satu ", "dua ", "tiga ", "empat ", "lima ",  
									   		 "enam ", "tujuh ", "delapan ", "sembilan "];
	
	if(bil<0){
		terbilang = String::from("(negatif) ");
	}else if(bil==0){
		terbilang = String::from("nol");
	}
	
	ribu     = bil as usize / 1000;
	ratus    = (bil as usize % 1000) / 100;
	puluh    = ((bil as usize % 1000) % 100) / 10;
	satuan   = ((bil as usize % 1000) % 100) % 10;
	
	if(ribu == 1) {
		terbilang.push_str("seribu");
	}else if(ribu >= 2){
		terbilang.push_str(terbilangSatuan[ribu]);
		terbilang.push_str("ribu");
	}
	
	if(ratus == 1) {
		terbilang.push_str("seratus");
	}else if(ribu >= 2){
		terbilang.push_str(terbilangSatuan[ratus]);
		terbilang.push_str("ratus");
	}
	
	if(puluh == 0){
		terbilang.push_str(terbilangSatuan[satuan]);
	}else if(puluh == 1) {
		if(satuan == 0) {
			terbilang.push_str("sepuluh");
		}else if(satuan == 1) {
			terbilang.push_str("sebelas");
		}else if(satuan >= 2){
			terbilang.push_str(terbilangSatuan[satuan]);
			terbilang.push_str("belas");
		}
	}else if(puluh >= 2){
		terbilang.push_str(terbilangSatuan[puluh]);
		terbilang.push_str("puluh");
		terbilang.push_str(terbilangSatuan[satuan]);
	}
	
	return terbilang;
}