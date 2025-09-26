use std::collections::BTreeMap;

const TRIAL_DIVISION_PRIMES: [u64; 46] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199];
const MILLER_RABIN_BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

// a * b % m をオーバーフローせずに計算する
fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

// base^exp % m をオーバーフローせずに計算する (繰り返し二乗法)
fn mod_pow(base: u64, exp: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut base = base % m;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            res = mod_mul(res, base, m);
        }
        base = mod_mul(base, base, m);
        exp /= 2;
    }
    res
}

// ミラー・ラビン素数判定法
fn is_prime_miller_rabin(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut d = n - 1;
    while d % 2 == 0 {
        d /= 2;
    }
    for &a in &MILLER_RABIN_BASES {
        if n == a { return true; }
        let mut x = mod_pow(a, d, n);
        let mut t = d;
        while t != n - 1 && x != 1 && x != n - 1 {
            x = mod_mul(x, x, n);
            t *= 2;
        }
        if x != n - 1 && t % 2 == 0 {
            return false;
        }
    }
    true
}

// 最大公約数を求める (ユークリッドの互除法)
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

// ポラード・ロー素因数分解法
fn pollards_rho(n: u64) -> u64 {
    if n % 2 == 0 { return 2; }
    if is_prime_miller_rabin(n) { return n; }

    let mut x: u64 = 2;
    let mut y: u64 = 2;
    let mut d: u64 = 1;
    let mut c: u64 = 1;

    while d == 1 {
        x = mod_mul(x, x, n).wrapping_add(c) % n;
        y = mod_mul(y, y, n).wrapping_add(c) % n;
        y = mod_mul(y, y, n).wrapping_add(c) % n;
        let diff = if x > y { x - y } else { y - x };
        d = gcd(diff, n);
        if d == n {
            c += 1;
            x = 2;
            y = 2;
            d = 1;
        }
    }
    d
}

// 素因数分解のメイン関数
pub fn prime_factorize(mut n: u64) -> BTreeMap<u64, i64> {
    let mut factors = BTreeMap::new();
    if n < 2 { return factors; }
    for &p in &TRIAL_DIVISION_PRIMES {
        while n % p == 0 {
            *factors.entry(p).or_insert(0) += 1;
            n /= p;
        }
    }
    if n == 1 { return factors; }

    let mut current_n = n;
    while current_n > 1 {
        if is_prime_miller_rabin(current_n) {
            *factors.entry(current_n).or_insert(0) += 1;
            break;
        }
        let factor = pollards_rho(current_n);
        // factorも分解して追加する
        let sub_factors = prime_factorize(factor);
        for (p, count) in sub_factors {
            *factors.entry(p).or_insert(0) += count;
        }
        current_n /= factor;
    }
    factors
}

