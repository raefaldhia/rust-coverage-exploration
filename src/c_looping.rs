fn OutputBintang_21(N: i32) {
	for i in 0..N {
        print!("*");
    }
}
	
fn jumBil_22(bil: &[i32], N: usize) -> i32 {
    let mut sum: i32 = 0;
	let mut i: usize = 0;
    while i < N {
        sum = sum + bil[i];
        i = i + 1;
    }
	return sum;
}
	
fn jumBil_23(N: i32) -> i32 {
	let mut sum: i32 = 0;
	let mut i: i32 = 0;
    loop {
        sum = sum + 1;
        i = i + 1;
        if i <= N {
            continue;
        }
        break;
    }
	return sum;
}
	
fn outputFaktorlBilangan_24(nilai: i32) {
	println!("Faktor-faktornya:");
    for i in nilai..1 {
        if (nilai % i == 0) {
            println!("{} ", i /* tampilkan bilangan ketika nilainya
                               * merupakan modulus dari nilai yang dicari
                               */
                            );
        }
    }
}
	
fn cariBil_25(bil: &[i32], N: usize, cari: i32) -> bool {
	let mut i: usize = 0;
	let mut ketemu: bool = false;
	
	while i < N || ketemu == true {
		if bil[i] == cari {
			ketemu = true;
		}
		i = i + 1;
	}
	return ketemu;
}

fn OutputDeretGanjilGenap_27(N: usize){
	let mut deretGanjil: Vec<usize> = Vec::with_capacity(N);
	let mut deretGenap: Vec<usize> = Vec::with_capacity(N);
	
	let mut nGanjil: usize = 0;
	let nGenap: usize = 0;
    for i in 1..N {
		if i % 2 > 0 {
			deretGanjil[nGanjil] = i;
			nGanjil = nGanjil + 1;
		} else { 
			deretGenap[nGenap] = i;
			nGanjil = nGenap + 1;
		}
    }
	println!("Deret Ganjil");
    for i in 0..(nGanjil-1) {
		print!("{} ", deretGanjil[i]);
    }
	println!("Deret Genap");
    for i in 0..(nGenap-1) {
		print!("{} ", deretGenap[i]);
    }
}

fn guessNumber_28(guessBil: &[i32], N: usize, secret: i32) -> bool {
	let mut ketemu: bool = false;
	let mut i: usize = 0;
	while i < N || ketemu == true {
		if guessBil[i] < secret {
			println!("Sorry, your guess ({}) is too low.", guessBil[i]);
			println!( " Try again.\n> ");
		} else if guessBil[i] > secret {
			println!( "Sorry, your guess ({}) is too high.", guessBil[i]);
			println!( " Try again.\n> " );
		} else {
			println!( "Yeaa, your guess ({}) is right.",  guessBil[i]);
			println!( "secret = {}", secret);
			ketemu = true;	
		}
		i = i + 1;
	}
	return ketemu;
}
	
fn OutputPolaXYZ29(N: i32, X: i32) {
	// prekondisi N dan X tidak bernilai 0 
	// menampilkan pola bilangan dari 1 s.d. N
	// dimana bilangan yang berkelipatan value dari X diganti dengan bintang
	// contoh : 15, 3 --> output : 1 2 * 4 5 * 7 8 * 10 11 * 13 14 * 
	
	let i: i32 = 1;
	loop {
		if i % X == 0 {
			print!("* ");
		} else {
			print!("{} ", i);
		}
        
        if i<=N {
            continue;
        }
        break;
    }
}
	
fn OutputCountBilPencacah_30(bil: &[i32], N: usize, cari: i32) {
	// menghitung nilai pencacah bilangan dari kumpulan bilangan
	// nilai pencacah bilangan antara 1 s.d 10, lebih dari itu tidak dihitung
	// contoh : isi bil 1, 1, 2, 15
	// nilai 1 ada 2, nilai 2 ada 1 dan tidak terdefinisi ada 1
	
	let mut count1: i32 = 0;
	let mut count2: i32 = 0;
	let mut count3: i32 = 0;
	let mut count4: i32 = 0;
	let mut count5: i32 = 0;
	let mut count6: i32 = 0;
	let mut count7: i32 = 0;
	let mut count8: i32 = 0;
	let mut count9: i32 = 0;
	let mut count10: i32 = 0;
	let mut countUnd: i32 = 0;
	for i in 0..(N-1) {
		match bil[i] {
			1 => count1 = count1 + 1,
			2 => count2 = count2 + 1,
			3 => count3 = count3 + 1,
			4 => count4 = count4 + 1,
			5 => count5 = count5 + 1,
			6 => count6 = count6 + 1,
			7 => count7 = count7 + 1,
			8 => count8 = count8 + 1,
			9 => count9 = count9 + 1,
			10 => count10 = count10 + 1,
			_ => countUnd = countUnd + 1,
		}
	}
	
	println!("pencacah 1 = {}" , count1);
	println!("pencacah 2 = {}" , count2);
	println!("pencacah 3 = {}" , count3);
	println!("pencacah 4 = {}" , count4);
	println!("pencacah 5 = {}" , count5);
	println!("pencacah 6 = {}" , count6);
	println!("pencacah 7 = {}" , count7);
	println!("pencacah 8 = {}" , count8);
	println!("pencacah 9 = {}" , count9);
	println!("pencacah 10 = {}" , count10);
	println!("pencacah diluar 1.d. 10 = {}" , countUnd);
}
	
fn OutputCalculateDeretBilBaseOnOp_31(bil: &[i32], N: usize, op: char) -> i32 {
	// mengirimlkan hasil kalkulasi dari deret bilangan berdasarkan operatornya (+, -, *)
	// Contoh : Bil = [1, 5, 7, 10] dan op = +
	// Output = 23
	
	let mut result: i32 = 0;
	let mut i: usize = 0;
	
	while i < N {
		match op {
		    '+' => result = result + bil[i], 
		    '-' => result = result - bil[i], 
		    '*' => result = result * bil[i], 
            _ => (),
		}
		i = i + 1;
	}
	
	return result;
}
	
fn konversiDesToBiner_32(mut bil: i32) -> String {
	let mut sisa: i32;
	let mut biner: String = String::new();		
	
    biner.push(' ');
	loop {
		sisa = bil % 2;
		bil = bil / 2;
		match sisa {
			0 => biner.insert(0, '0'),
			1 => biner.insert(0, '1'),
            _ => (),
		}

        if bil > 0 {
            continue;
        }
        break;
    }
	return biner;
}
	
fn OutputBintangSegiempat_33(N: i32) {
	for i in 1..N {
        for j in 1..i {
            print!("*");
        }
		println!();
	}
}
	
fn OutputBintangSegitiga_34(N: i32){
	let mut i: i32 = 1;
	while i <= N {
        for j in 1..i {
            print!("*");
        }
		println!();
		i = i + 1;
	}
}
	
fn geserBilKiri_36(str_: &mut [char], N: usize, nGeser: i32) -> &[char] {
	// Geser 3 kali
	// str --> 1, 2, 5, 7
	// 1 : 7, 1, 2, 5
	// 2 : 5, 7, 1, 2
	// 3 : 2, 5, 7, 1
	
	let mut j: usize;
	let mut temp: char;
	
	for i in 1..nGeser {
		temp = str_[N-1];
		j = N - 1; 
		while j > 0 {
			str_[j] = str_[j-1]; 
			j = j - 1;
		}
		str_[j] = temp;
	}
	return str_;
}
	
fn OutputBintangSegitigaTerbalik_37(N: i32) {
	let mut i: i32 = 1;
    let mut j: i32;
	while i <= N {
		j = 1;
		while j < i {
			print!(" ");
			j = j + 1;
		}
		while j <= N {
			print!("*");
			j = j + 1;
		}
        println!();
		i = i + 1;
	}
}