fn OutputBintang_42(N: i32) {
	// Output bintang dilakukan jika N bernilai positif
	if N > 0 {
		for i in 1..N {
			print!("*");
		}
	} else {
		print!("N invalid, harus bernilai positif");
	}
}
	
fn isBilPrima_43(bil: i32) -> bool {
	let mut isPrima: bool = false;
	let mut temp: i32;
	
	if bil > 1 {
		temp = bil - 1;
		while temp > 1 && isPrima == false {
			if bil % temp == 0 {
			    isPrima = true;
			}
			temp = temp - 1;
		}
	}
	return isPrima;
}
	
fn pengurutanBilangan_44(bil: &mut[i32], N: usize) -> &[i32] {
    let mut k: usize;
    let mut temp: i32;
	if N > 0  {
		k = N - 1;
        loop {
            for i in 0..k {
				if bil[i] > bil[i+1] {
					temp     = bil[i];
					bil[i]   = bil[i+1];
					bil[i+1] = temp;
				}
			}
			k = k - 1;
            if k > 0 {
                continue;
            }
            break;
		}
	}
	return bil;
}
	
fn isAdaBil_45(bil: &[i32], N: usize, cari: i32) -> &'static str {
	// pre kondisi N adalah panjang array bilangan
	let mut status: &'static str = "tidak ketemu";
		
	if N > 0 {
		status = "Deretan Bilangan Kosong";
	} else {
		for i in 0..(N-1) {
		    if (bil[i] == cari){
				status = "bilangan ketemu";
			}
		}
	}
	return status;
}

fn calPangkatDua_46_47(bil: i32, mut pangkat: i32) -> f32 {
	let mut result: f32 = 1f32;
	let mut temp: i32;
    let mut i: i32;
	
	if bil == 0 && pangkat > 0 {
		result = 0f32;
	} else if pangkat > 0 {
		i = 0;
		loop {
			result = result * bil as f32;
			i = i + 1;
            if i < pangkat {
                continue;
            }
            break;
		}
	} else if pangkat < 0 {
		if bil != 0 {
			i = 0;
			while i < pangkat {
				result = result * 1f32/bil as f32;
				i = i+1;
			}
		} else {
			result = -99999f32;
		}
	} else { // pangkat = 0
		if bil == 0 {
			result = -99999f32;
		}
	}
	return result;
}
	
fn desimalToBiner_48(bil: i32) -> [i32; 10] {
	// panjang array yang dikembalikan adalah 10
	// Ilustrasi :
	// desimal = 11 --> biner = 0000001011
	// desimal = 0  --> biner = 0000000000
	// desimal < 0  --> biner = tidak terdefinisi (diisi dengan -1-1-1-1-1-1-1-1-1-1
	
	let mut biner: [i32; 10] = [0; 10];
	let mut calTemp: i32;
    let mut idx: usize;
	
	if bil >= 0 {
		if bil == 0 {
			idx = 10;
		} else {
			calTemp = bil;
			idx = 9;
			while(calTemp != 1) {
				biner[idx] = calTemp % 2;
				calTemp = calTemp/2;
				idx = idx - 1;
			}
			biner[idx] = calTemp;
		}
        for idx in (idx-1)..0 {
			biner[idx] = 0;
        }
	} else {
        for idx in 1..9 {
			biner[idx] = -1;
        }
	}
	return biner;
}
	
fn GetIndexByElemen_49(bil: &[i32], N: usize, cari: i32) -> i32 {
	// cari index by elemen
	// jika elemen bernilai negatif --> return -1
	// jika elemen bernilai nol --> return 0
	// jika elemen tidak ketemu --> return -2
	// jika elemen ketemu --> return indexnya
	
    let mut i: usize;
    let mut idx: i32;
	let mut ketemu: bool = false;
	
	idx = 0;
	if N > 0 {
		if cari <= 0 {
			if cari < 0 {
				idx = -1;
			}
		} else {
			idx = -2;
			i   = 0;
			while i<N || ketemu == true {
				if bil[i] == cari {
					ketemu = true;
					idx = i as i32;
				}
				i = i+1;
			}
		}
	}
	return idx;
}
	
fn GetElemenTerbesar_50(bil: &[i32], N: usize) -> i32 {
	// nilai N adalah jumlah bil
	// prekondisi jumlah deretam pada array bil sama dengan nilai N
	// cari elemen terbesar dari deretan bilangan
	// jika jumlah deret bilangan bernilai negatif --> return -1
	// jika jumlah deret bilangan bernilai nol --> return 0
	// jika jumlah deret bilangan lebih besar dari nol --> return max bilnya
	
    let mut i: usize;
    let mut idx: i32;
    let mut max: i32;
	let mut ketemu: bool = false;
	
	max = 0;
	if N <= 0 {
		if N<0 {
			max = -1;
		}
	} else {
		idx = -2;
	    i   = 0;
	    loop {
			if bil[i] > max {
				max = bil[i];
			}
			i = i+1;
            if (i < N) {
                continue;
            }
            break;
	    }
	}
	return (max);
}