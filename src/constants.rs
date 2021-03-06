pub mod constants {
    pub const MAX_ITERATIONS: u64 = 600;
    pub const INFINITY: f64 = 128.0;
    pub const LN_16: f64 = 2.772588722239781; // 16.ln()
    pub const LN_2_3: f64 = 0.832909122935104; // 2.3.ln()
    pub const LN_1E_NEGATIVE_10: f64 = -23.025850929940457; // 1e-10.ln()
    pub const COLORS: [(u8, u8, u8); 21] = [
        (255, 255, 255),
        (238, 244, 244),
        (220, 233, 233),
        (203, 221, 221),
        (185, 210, 210),
        (167, 198, 198),
        (156, 184, 188),
        (145, 169, 177),
        (133, 153, 165),
        (122, 137, 154),
        (111, 122, 142),
        (100, 107, 132),
        (89, 92, 121),
        (77, 77, 108),
        (66, 66, 92),
        (55, 55, 76),
        (44, 44, 62),
        (33, 33, 46),
        (21, 21, 30),
        (10, 10, 14),
        (0, 0, 0),
    ];
}
