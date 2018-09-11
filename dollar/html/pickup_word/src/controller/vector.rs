pub fn length(v: &(f64, f64))->f64{
    (v.0 * v.0 + v.1*v.1).sqrt()
}

pub fn normalize(v : &mut (f64, f64)){
    let len = length(&v);
    v.0 = v.0 / len;
    v.1 = v.1 / len;
}

pub fn dot(v1: &(f64, f64), v2: &(f64, f64))->f64{
    v1.0*v2.0 + v1.1*v2.1
}

pub fn sign(v1: &(f64, f64), v2: &(f64, f64))->f64{
    if v1.1*v2.0 > v1.0*v2.1 {
        return 1.0;
    }else{
        return -1.0;
    }
}