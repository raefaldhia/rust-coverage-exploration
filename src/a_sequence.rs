use std::io::{self, Read};

#[cfg(test)]
mod tests {
    use super::*;
/*	
	#[test]
	fn test_a_sequence() {
		a_sequence();
	}
*/

	#[test]
	fn test() {
		luasLimas(2f32, 6f32);
		sisaTukarNominalPec1Kdgn50K10K5K(50000);
		isSuhuCair(50);
		isSuhuUap(50);
		isSuhuPadat(50);
		isSuhuPadat(-1);
		isYearKabisat(6, 2000);
		isPointOrigin(0, 0);
		isPointKuadran1(1, 1);
		isPointKuadran2(1, 1);
		isPointKuadran3(1, 1);
		isPointKuadran4(1, 1);
		isPointNotOrigin(1, 1);
	}
}

pub fn a_sequence() -> io::Result<()> {
	// menghitung luas limas
	let alasLimas:   f32 = 12.95;
    let tinggiLimas: f32 = 10.5;
		
    println!("Luas Limas = {}", luasLimas(alasLimas, tinggiLimas));
	
	// persamaan linear y = 3x - 4
	let mut X = String::new();
	print!("(x) : ");
	io::stdin().read_line(&mut X)?;
	println!("Persamaan Linear");
	println!("y = 3x + 4");
	println!("  = 3*{} + 4", X.trim().parse::<i32>().unwrap_or(0));
	println!("  = {}", persamaanLinear(X.trim().parse::<i32>().unwrap_or(0)));

	// menambahkan detik dengan jam yang sudah ditentukan
	let detik:    i32 = 500;
	let tbhJam:   i32 = 1;
	let tbhMenit: i32 = 10;
	let tbhDetik: i32 = 50;
	println!("Penambahan detik : 500, dengan 1 jam 10 menit 50 detik");
	println!("  = {}", addDetikWithJam(detik, tbhJam, tbhMenit, tbhDetik));

	Ok(())
}

fn luasLimas(sisiAlas: f32, tinggiMiring: f32) -> f32 {
	let luas: f32 = (sisiAlas * sisiAlas) + (4.0 * (sisiAlas * tinggiMiring / 2.0));
	return luas;	
}

fn persamaanLinear(x: i32) -> i32 {
	let y: i32 = 3 * x - 4;
	return y;	
}

fn addDetikWithJam(mut detik: i32, hour: i32, minute: i32, second: i32) -> i32 {
	let s: i32 = (hour * 3600) + (minute * 60) + second;
	detik = detik + s;
	return detik;
}

fn sisaTukarNominalPec1Kdgn50K10K5K(nominalUang: i32) -> i32 {
	let pec50K: i32 = nominalUang / 50000;
	let pec10K: i32 = (nominalUang % 50000) / 10000;
	let pec5K: i32 = ((nominalUang % 50000) % 10000) / 5000;
	let pec1K: i32 = (((nominalUang % 50000) % 10000) % 5000) / 1000;
	return pec1K;	
}

fn isYearKabisat(idxMonth: i32, year: i32) -> bool {
	let isKabisat: bool = ((year % 4 == 0) && (year % 100 > 0)) || (year % 400 == 0); 
	return isKabisat;
}

fn isPointOrigin(x: i32, y: i32) -> bool {
	let isOrigin: bool = (x == 0)  && (y == 0); 
	return isOrigin;
}

fn isPointKuadran1(x: i32, y: i32) -> bool {
	let isKuadran1: bool = (x > 0)  && (y > 0); 
	return isKuadran1;
}

fn isPointKuadran2(x: i32, y: i32) -> bool {
	let isKuadran2: bool = (x < 0)  && (y > 0); 
	return isKuadran2;
}

fn isPointKuadran3(x: i32, y: i32) -> bool {
	let isKuadran3: bool = (x < 0)  && (y < 0); 
	return isKuadran3;
}

fn isPointKuadran4(x: i32, y: i32) -> bool {
	let isPointKuadran4: bool = (x > 0)  && !(y >= 0); 
	return isPointKuadran4;
}

fn isPointNotOrigin(x: i32, y: i32) -> bool {
	let isNotOrigin: bool = (x != 0) || (y != 0); 
	return isNotOrigin;
}

fn isSuhuPadat(suhu: i32) -> bool {
	let isPadat: bool = suhu >=0 && suhu<= 100; 
	return isPadat;
}

fn isSuhuCair(suhu: i32) -> bool {
	let isCair: bool = suhu<= 0; 
	return isCair;
}

fn isSuhuUap(suhu: i32) -> bool {
	let isUap: bool = (suhu >= 100); 
	return isUap;
}