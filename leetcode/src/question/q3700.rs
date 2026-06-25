const MOD: u64 = 1000000007;

pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
    let m = (r-l+1) as usize;
    let d = m*2;
    let mut seed = vec![1u64; d];
    let mut mat = vec![vec![0u64; d]; d];
    for i in 0..m{
        for j in 0..m{
            if i < j {
                mat[i][m+j] = 1;
            }else if i> j{
                mat[m+i][j] = 1;
            }
        }
    }
    let mut e = (n-1) as usize;
    while e>0{
        if e & 1 == 1{
            seed = vec_mul(&seed, &mat);
        }
        mat = mat_mul(&mat, &mat);
        e >>= 1;
    }
    let mut ans = 0u64;
    for i in 0..d{
        ans = (ans + seed[i]) % MOD;
    }
    ans as i32
}

fn mat_mul(a: &[Vec<u64>], b: &[Vec<u64>]) -> Vec<Vec<u64>>{
    let len = a.len();
    let mut res = vec![vec![0u64;len];len];
    for i in 0..len{
        let sub = &a[i];
        for j in 0..len{
            res[i][j] = {
                let mut sum = 0u64;
                for k in 0..len{
                    if sub[k] == 0 || b[k][j] == 0 {
                        continue;
                    }
                    sum = (sum + sub[k]*b[k][j])%MOD;
                }
                sum
            }
        }
    }
    res
}

fn vec_mul(a: &[u64], mat: &[Vec<u64>]) -> Vec<u64>{
    let len = mat.len();
    let mut res = vec![0u64; len];
    for i in 0..len{
        res[i] = {
            let mut sum = 0u64;
            for j in 0..len{
                if mat[j][i] == 0 || a[j] == 0 {
                    continue;
                }
                sum = (sum + a[j]*mat[j][i])%MOD;
            }
            sum
        }
    }
    res
}