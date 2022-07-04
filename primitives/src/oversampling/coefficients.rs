// https://www.earlevel.com/main/2010/12/05/building-a-windowed-sinc-filter/
// Factor: 0.43
// Length: 91
// Rejection: 90
pub const COEFFICIENTS_2: [f32; 91] = [
    -0.000_005_988_801_6,
    0.000_003_584_150_6,
    0.000_025_796_378,
    0.000_007_861_443,
    -0.000_058_795_435,
    -0.000_054_984_45,
    0.000_087_295_455,
    0.000_158_839_89,
    -0.000_067_896_32,
    -0.000_319_964_46,
    -0.000_064_997_42,
    0.000_493_564_6,
    0.000_376_430_28,
    -0.000_571_887_4,
    -0.000_887_972_4,
    0.000_390_068_37,
    0.001_524_861_2,
    0.000_231_162_85,
    -0.002_075_111,
    -0.001_404_115_3,
    0.002_188_675_6,
    0.003_070_332,
    -0.001_440_047_6,
    -0.004_908_716,
    -0.000_539_721_46,
    0.006_299_618_6,
    0.003_881_939,
    -0.006_382_805,
    -0.008_296_353,
    0.004_223_07,
    0.012_943_688,
    0.000_939_581_7,
    -0.016_412_662,
    -0.009_415_971,
    0.016_805_384,
    0.020_854_158,
    -0.011_844_58,
    -0.034_167_67,
    -0.001_288_871_3,
    0.047_654_36,
    0.027_429_905,
    -0.059_300_08,
    -0.082_277_77,
    0.067_201_64,
    0.309_995_86,
    0.429_998_55,
    0.309_995_86,
    0.067_201_64,
    -0.082_277_77,
    -0.059_300_08,
    0.027_429_905,
    0.047_654_36,
    -0.001_288_871_3,
    -0.034_167_67,
    -0.011_844_58,
    0.020_854_158,
    0.016_805_384,
    -0.009_415_971,
    -0.016_412_662,
    0.000_939_581_7,
    0.012_943_688,
    0.004_223_07,
    -0.008_296_353,
    -0.006_382_805,
    0.003_881_939,
    0.006_299_618_6,
    -0.000_539_721_46,
    -0.004_908_716,
    -0.001_440_047_6,
    0.003_070_332,
    0.002_188_675_6,
    -0.001_404_115_3,
    -0.002_075_111,
    0.000_231_162_85,
    0.001_524_861_2,
    0.000_390_068_37,
    -0.000_887_972_4,
    -0.000_571_887_4,
    0.000_376_430_28,
    0.000_493_564_6,
    -0.000_064_997_42,
    -0.000_319_964_46,
    -0.000_067_896_32,
    0.000_158_839_89,
    0.000_087_295_455,
    -0.000_054_984_45,
    -0.000_058_795_435,
    0.000_007_861_443,
    0.000_025_796_378,
    0.000_003_584_150_6,
    -0.000_005_988_801_6,
];
