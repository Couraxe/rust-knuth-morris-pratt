fn next_arr(pat: &String) -> Vec::<i32>
{
	let m = pat.len() as i32;
	let mut narr = vec![0; m as usize];

	let mut i: i32 = 0;
	let mut j: i32 = -1;

	narr[0] = -1;

	while i < m - 1
	{
		if j == -1
		|| pat.chars().nth(i as usize).unwrap()
			== pat.chars().nth(j as usize).unwrap()
		{
			i += 1;
			j += 1;

			if pat.chars().nth(i as usize).unwrap()
				== pat.chars().nth(j as usize).unwrap()
			{
				narr[i as usize] = narr[j as usize];
			} else
			{
				narr[i as usize] = j;
			}
		} else
		{
			j = narr[j as usize];
		}
	}

	narr
}

fn kmp(pat: &String, txt: &String, narr: &Vec::<i32>) -> i32
{
	let n = txt.len() as i32;
	let m = pat.len() as i32;

	if n >= m
	{
		let mut i: i32 = 0;
		let mut j: i32 = 0;

		while i < n && j < m
		{
			println!("T[{}] = {:?}, P[{}] = {:?}", i, txt.chars().nth(i as usize), j, pat.chars().nth(j as usize));
			if j == -1
			|| txt.chars().nth(i as usize).unwrap() == pat.chars().nth(j as usize).unwrap()
			{
				i += 1;
				j += 1;
			} else
			{
				j = narr[j as usize];
			}
		}
		if j == m
		{
			return i - m + 1;
		}
	}

	n
}

fn main()
{
	let txt = String::from("Hello world");
	let pat = String::from("world");

	let narr = next_arr(&pat);
	let occ  = kmp(&pat, &txt, &narr);

	println!("Found at: {}", occ);
}
