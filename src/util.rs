#[rustfmt::skip]
const FLOOR1_INVERSE_DB_TABLE: [f64; 256] = [
    1.0649863e-07, 1.1341951e-07, 1.2079015e-07, 1.2863978e-07,
    1.3699951e-07, 1.4590251e-07, 1.5538408e-07, 1.6548181e-07,
    1.7623575e-07, 1.8768855e-07, 1.9988561e-07, 2.1287530e-07,
    2.2670913e-07, 2.4144197e-07, 2.5713223e-07, 2.7384213e-07,
    2.9163793e-07, 3.1059021e-07, 3.3077411e-07, 3.5226968e-07,
    3.7516214e-07, 3.9954229e-07, 4.2550680e-07, 4.5315863e-07,
    4.8260743e-07, 5.1396998e-07, 5.4737065e-07, 5.8294187e-07,
    6.2082472e-07, 6.6116941e-07, 7.0413592e-07, 7.4989464e-07,
    7.9862701e-07, 8.5052630e-07, 9.0579828e-07, 9.6466216e-07,
    1.0273513e-06, 1.0941144e-06, 1.1652161e-06, 1.2409384e-06,
    1.3215816e-06, 1.4074654e-06, 1.4989305e-06, 1.5963394e-06,
    1.7000785e-06, 1.8105592e-06, 1.9282195e-06, 2.0535261e-06,
    2.1869758e-06, 2.3290978e-06, 2.4804557e-06, 2.6416497e-06,
    2.8133190e-06, 2.9961443e-06, 3.1908506e-06, 3.3982101e-06,
    3.6190449e-06, 3.8542308e-06, 4.1047004e-06, 4.3714470e-06,
    4.6555282e-06, 4.9580707e-06, 5.2802740e-06, 5.6234160e-06,
    5.9888572e-06, 6.3780469e-06, 6.7925283e-06, 7.2339451e-06,
    7.7040476e-06, 8.2047000e-06, 8.7378876e-06, 9.3057248e-06,
    9.9104632e-06, 1.0554501e-05, 1.1240392e-05, 1.1970856e-05,
    1.2748789e-05, 1.3577278e-05, 1.4459606e-05, 1.5399272e-05,
    1.6400004e-05, 1.7465768e-05, 1.8600792e-05, 1.9809576e-05,
    2.1096914e-05, 2.2467911e-05, 2.3928002e-05, 2.5482978e-05,
    2.7139006e-05, 2.8902651e-05, 3.0780908e-05, 3.2781225e-05,
    3.4911534e-05, 3.7180282e-05, 3.9596466e-05, 4.2169667e-05,
    4.4910090e-05, 4.7828601e-05, 5.0936773e-05, 5.4246931e-05,
    5.7772202e-05, 6.1526565e-05, 6.5524908e-05, 6.9783085e-05,
    7.4317983e-05, 7.9147585e-05, 8.4291040e-05, 8.9768747e-05,
    9.5602426e-05, 0.00010181521, 0.00010843174, 0.00011547824,
    0.00012298267, 0.00013097477, 0.00013948625, 0.00014855085,
    0.00015820453, 0.00016848555, 0.00017943469, 0.00019109536,
    0.00020351382, 0.00021673929, 0.00023082423, 0.00024582449,
    0.00026179955, 0.00027881276, 0.00029693158, 0.00031622787,
    0.00033677814, 0.00035866388, 0.00038197188, 0.00040679456,
    0.00043323036, 0.00046138411, 0.00049136745, 0.00052329927,
    0.00055730621, 0.00059352311, 0.00063209358, 0.00067317058,
    0.00071691700, 0.00076350630, 0.00081312324, 0.00086596457,
    0.00092223983, 0.00098217216, 0.0010459992,  0.0011139742,
    0.0011863665,  0.0012634633,  0.0013455702,  0.0014330129,
    0.0015261382,  0.0016253153,  0.0017309374,  0.0018434235,
    0.0019632195,  0.0020908006,  0.0022266726,  0.0023713743,
    0.0025254795,  0.0026895994,  0.0028643847,  0.0030505286,
    0.0032487691,  0.0034598925,  0.0036847358,  0.0039241906,
    0.0041792066,  0.0044507950,  0.0047400328,  0.0050480668,
    0.0053761186,  0.0057254891,  0.0060975636,  0.0064938176,
    0.0069158225,  0.0073652516,  0.0078438871,  0.0083536271,
    0.0088964928,  0.009474637,   0.010090352,   0.010746080,
    0.011444421,   0.012188144,   0.012980198,   0.013823725,
    0.014722068,   0.015678791,   0.016697687,   0.017782797,
    0.018938423,   0.020169149,   0.021479854,   0.022875735,
    0.024362330,   0.025945531,   0.027631618,   0.029427276,
    0.031339626,   0.033376252,   0.035545228,   0.037855157,
    0.040315199,   0.042935108,   0.045725273,   0.048696758,
    0.051861348,   0.055231591,   0.058820850,   0.062643361,
    0.066714279,   0.071049749,   0.075666962,   0.080584227,
    0.085821044,   0.091398179,   0.097337747,   0.10366330,
    0.11039993,    0.11757434,    0.12521498,    0.13335215,
    0.14201813,    0.15124727,    0.16107617,    0.17154380,
    0.18269168,    0.19456402,    0.20720788,    0.22067342,
    0.23501402,    0.25028656,    0.26655159,    0.28387361,
    0.30232132,    0.32196786,    0.34289114,    0.36517414,
    0.38890521,    0.41417847,    0.44109412,    0.46975890,
    0.50028648,    0.53279791,    0.56742212,    0.60429640,
    0.64356699,    0.68538959,    0.72993007,    0.77736504,
    0.82788260,    0.88168307,    0.9389798,     1.
];

pub fn ilog(x: u32) -> u32 {
    const BITS: usize = std::mem::size_of::<u32>() * 8;
    BITS as u32 - x.leading_zeros()
}

pub fn float32_unpack(x: u32) -> f32 {
    let mut mantissa: i32 = (x & 0x001FFFFF) as i32;
    let sign: bool = (x & 0x80000000) != 0;
    let exponent: i32 = ((x & 0x7FE00000) >> 21) as i32;
    if sign == true {
        mantissa *= -1;
    }
    let pow: f32 = 2.0_f32.powi(exponent - 788);
    let f = mantissa as f32 * pow;
    assert!(f.is_finite());
    f
}

pub fn lookup1_values(entries: u32, dimensions: u32) -> u32 {
    let mut retval: u32 = 0;
    while (retval + 1).pow(dimensions) <= entries {
        retval += 1;
    }
    retval
}

pub fn low_neighbor(v: &[i32], x: usize) -> usize {
    assert!(x < v.len());
    let range = &v[..x];
    range
        .iter()
        .enumerate()
        .filter(|(_n, i)| **i < v[x])
        .max_by_key(|(_n, i)| **i)
        .unwrap_or_else(|| panic!("No values less than v[{}]={}", x, v[x]))
        .0
}

pub fn high_neighbor(v: &[i32], x: usize) -> usize {
    assert!(x < v.len());
    let range = &v[..x];
    range
        .iter()
        .enumerate()
        .filter(|(_n, i)| **i > v[x])
        .min_by_key(|(_n, i)| **i)
        .unwrap_or_else(|| panic!("No values greater than v[{}]={}", x, v[x]))
        .0
}

pub fn render_point(x0: i32, y0: i32, x1: i32, y1: i32, x: i32) -> i32 {
    let dy = y1 - y0;
    let adx = x1 - x0;
    let ady = dy.abs();
    let err = ady * (x - x0);
    let off = err / adx;
    if dy < 0 {
        y0 - off
    } else {
        y0 + off
    }
}

pub fn render_line(x0: i32, y0: i32, x1: i32, y1: i32, v: &mut Vec<i32>) {
    assert!(x0 <= x1);
    let range = 0..=v.len();
    assert!(range.contains(&(x0 as usize)));
    assert!(range.contains(&(x1 as usize)));

    let dy = y1 - y0;
    let adx = x1 - x0;
    let base = dy / adx;

    let mut y = y0;
    let mut err = 0;
    let sy = if dy < 0 { base - 1 } else { base + 1 };
    let ady = dy.abs() - base.abs() * adx;
    v[x0 as usize] = y;

    for x in x0 + 1..x1 {
        err += ady;
        if err >= adx {
            err -= adx;
            y += sy;
        } else {
            y += base;
        }
        v[x as usize] = y;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ilog() {
        assert_eq!(ilog(0), 0);
        assert_eq!(ilog(1), 1);
        assert_eq!(ilog(2), 2);
        assert_eq!(ilog(3), 2);
        assert_eq!(ilog(4), 3);
        assert_eq!(ilog(7), 3);
    }

    #[test]
    fn test_float32_unpack() {
        // Zero
        assert_eq!(float32_unpack(0x00000000), 0.0);
        assert_eq!(float32_unpack(0x80000000), 0.0);
        assert_eq!(float32_unpack(0x66600000), 0.0);

        // 1 in different ways (mantissa and exponent cancel each other out)
        assert_eq!(float32_unpack(0x62800001), 1.0);
        assert_eq!(float32_unpack(0x62000010), 1.0);
        assert_eq!(float32_unpack(0x61800100), 1.0);
        assert_eq!(float32_unpack(0x61001000), 1.0);
        assert_eq!(float32_unpack(0x60810000), 1.0);
        assert_eq!(float32_unpack(0x60100000), 1.0);

        // -1 in different ways (mantissa and exponent cancel each other out)
        assert_eq!(float32_unpack(0xE2800001), -1.0);
        assert_eq!(float32_unpack(0xE2000010), -1.0);
        assert_eq!(float32_unpack(0xE1800100), -1.0);
        assert_eq!(float32_unpack(0xE1001000), -1.0);
        assert_eq!(float32_unpack(0xE0810000), -1.0);
        assert_eq!(float32_unpack(0xE0100000), -1.0);

        // A few other numbers
        assert_eq!(float32_unpack(0x62800004), 4.0);
        assert_eq!(float32_unpack(0x62800009), 9.0);
        assert_eq!(float32_unpack(0x62800010), 16.0);
        assert_eq!(float32_unpack(0x62800019), 25.0);
        assert_eq!(float32_unpack(0x628F4240), 1_000_000.0);
        assert_eq!(float32_unpack(0xE28F4240), -1_000_000.0);

        // Fractions
        assert_eq!(float32_unpack(0x62600001), 0.5);
        assert_eq!(float32_unpack(0xE2600001), -0.5);
        assert_eq!(float32_unpack(0x62400001), 0.25);
        assert_eq!(float32_unpack(0xE2400001), -0.25);
        assert_eq!(float32_unpack(0x62600001), 0.5);
        assert_eq!(float32_unpack(0xE2600001), -0.5);
        assert_eq!(float32_unpack(0x61800001), 0.00390625);
        assert_eq!(float32_unpack(0xE1800001), -0.00390625);
    }

    #[test]
    fn test_lookup1_values() {
        assert_eq!(lookup1_values(0, 0), 0); // 0 to the 0th power is undefined
        assert_eq!(lookup1_values(0, 1), 0);
        assert_eq!(lookup1_values(0, 2), 0);
        assert_eq!(lookup1_values(0, 10), 0);
        assert_eq!(lookup1_values(81, 4), 3);
        assert_eq!(lookup1_values(271, 4), 4);
        assert_eq!(lookup1_values(625, 4), 5);
        assert_eq!(lookup1_values(81, 2), 9);
        assert_eq!(lookup1_values(168, 2), 12);
        assert_eq!(lookup1_values(169, 2), 13);
        assert_eq!(lookup1_values(224, 2), 14);
        assert_eq!(lookup1_values(225, 2), 15);
        assert_eq!(lookup1_values(288, 2), 16);
        assert_eq!(lookup1_values(289, 2), 17);
    }

    #[test]
    fn test_low_neighbor() {
        assert_eq!(low_neighbor(&[0, 1, 2], 2), 1);
        assert_eq!(
            low_neighbor(
                &[0, 128, 12, 46, 4, 8, 16, 23, 33, 70, 2, 6, 10, 14, 19, 28, 39, 58, 90],
                2
            ),
            0
        );
        assert_eq!(
            low_neighbor(
                &[0, 128, 12, 46, 4, 8, 16, 23, 33, 70, 2, 6, 10, 14, 19, 28, 39, 58, 90],
                18
            ),
            9
        );
    }

    #[test]
    #[should_panic]
    fn test_low_neighbor_invalid1() {
        low_neighbor(&[], 0);
    }

    #[test]
    #[should_panic]
    fn test_low_neighbor_invalid2() {
        low_neighbor(&[0], 0);
    }

    #[test]
    #[should_panic]
    fn test_low_neighbor_invalid3() {
        low_neighbor(&[0, 1, 2], 5);
    }

    #[test]
    #[should_panic]
    fn test_low_neighbor_invalid4() {
        low_neighbor(&[2, 1, 0], 2);
    }

    fn test_high_neighbor() {
        assert_eq!(high_neighbor(&[2, 1, 0], 2), 1);
        assert_eq!(
            high_neighbor(
                &[0, 128, 12, 46, 4, 8, 16, 23, 33, 70, 2, 6, 10, 14, 19, 28, 39, 58, 90],
                2
            ),
            1
        );
        assert_eq!(
            high_neighbor(
                &[0, 128, 12, 46, 4, 8, 16, 23, 33, 70, 2, 6, 10, 14, 19, 28, 39, 58, 90],
                18
            ),
            1
        );
    }

    #[test]
    #[should_panic]
    fn test_high_neighbor_invalid1() {
        high_neighbor(&[], 0);
    }

    #[test]
    #[should_panic]
    fn test_high_neighbor_invalid2() {
        high_neighbor(&[0], 0);
    }

    #[test]
    #[should_panic]
    fn test_high_neighbor_invalid3() {
        high_neighbor(&[0, 1, 2], 5);
    }

    #[test]
    #[should_panic]
    fn test_high_neighbor_invalid4() {
        high_neighbor(&[0, 1, 2], 2);
    }

    #[test]
    fn test_render_point() {
        assert_eq!(render_point(0, 0, 1, 0, 0), 0);
        assert_eq!(render_point(0, 83, 128, 72, 12), 82);
        assert_eq!(render_point(12, 86, 128, 72, 46), 82);
    }

    #[test]
    fn test_render_line() {
        // Flat line
        let mut v = vec![0; 5];
        render_line(0, 0, 5, 0, &mut v);
        assert_eq!(v, vec![0, 0, 0, 0, 0]);

        // Simple line
        let mut v = vec![0; 5];
        render_line(0, 0, 5, 5, &mut v);
        assert_eq!(v, vec![0, 1, 2, 3, 4]);

        // From an example
        let mut v = vec![0; 12];
        render_line(0, 166, 12, 172, &mut v);
        assert_eq!(
            v,
            vec![166, 166, 167, 167, 168, 168, 169, 169, 170, 170, 171, 171]
        );

        // Another example
        let mut v = vec![0; 16];
        render_line(12, 172, 16, 162, &mut v);
        assert_eq!(
            v,
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 172, 170, 167, 165]
        );
    }

    #[test]
    #[should_panic]
    fn test_render_line_invalid1() {
        let mut v = Vec::new();
        render_line(0, 0, 100, 100, &mut v);
    }

    #[test]
    #[should_panic]
    fn test_render_line_invalid2() {
        let mut v = vec![0; 12];
        render_line(0, 0, -12, 0, &mut v);
    }
}
