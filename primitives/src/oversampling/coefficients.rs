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

// https://www.earlevel.com/main/2010/12/05/building-a-windowed-sinc-filter/
// Factor: 0.045
// Length: 201
// Rejection: 70
pub const COEFFICIENTS_16: [f32; 201] = [
    0.000_023_679_097,
    0.000_029_368_508,
    0.000_034_930_443,
    0.000_039_975_657,
    0.000_044_056_076,
    0.000_046_675_348,
    0.000_047_303_198,
    0.000_045_393_42,
    0.000_040_405_343,
    0.000_031_828_265,
    0.000_019_208_326,
    0.000_002_176_991_5,
    -0.000_019_519_683,
    -0.000_045_992_11,
    -0.000_077_179_7,
    -0.000_112_826_034,
    -0.000_152_458_06,
    -0.000_195_370_45,
    -0.000_240_616_41,
    -0.000_287_006_03,
    -0.000_333_113_2,
    -0.000_377_291_76,
    -0.000_417_701_4,
    -0.000_452_343_9,
    -0.000_479_108_9,
    -0.000_495_829_56,
    -0.000_500_347_1,
    -0.000_490_582_4,
    -0.000_464_614_1,
    -0.000_420_761,
    -0.000_357_666_1,
    -0.000_274_380_2,
    -0.000_170_442_57,
    -0.000_045_955_29,
    0.000_098_351_15,
    0.000_261_064_4,
    0.000_440_051_26,
    0.000_632_436_3,
    0.000_834_600_1,
    0.001_042_199,
    0.001_250_208,
    0.001_452_988_1,
    0.001_644_377_6,
    0.001_817_807_4,
    0.001_966_439_4,
    0.002_083_326_2,
    0.002_161_588,
    0.002_194_607_2,
    0.002_176_231_2,
    0.002_100_984_6,
    0.001_964_281_5,
    0.001_762_634_4,
    0.001_493_853,
    0.001_157_229,
    0.000_753_697_3,
    0.000_285_972_4,
    -0.000_241_349_67,
    -0.000_821_725_47,
    -0.001_446_640_1,
    -0.002_105_628_4,
    -0.002_786_343_5,
    -0.003_474_673,
    -0.004_154_906,
    -0.004_809_947,
    -0.005_421_576_6,
    -0.005_970_751,
    -0.006_437_945,
    -0.006_803_523_7,
    -0.007_048_135_6,
    -0.007_153_130_6,
    -0.007_100_979,
    -0.006_875_694,
    -0.006_463_243_6,
    -0.005_851_940_3,
    -0.005_032_803_4,
    -0.003_999_881_4,
    -0.002_750_527_6,
    -0.001_285_617_6,
    0.000_390_291_83,
    0.002_268_875_3,
    0.004_338_022_3,
    0.006_581_898,
    0.008_981_075,
    0.011_512_753,
    0.014_151_033,
    0.016_867_284,
    0.019_630_551,
    0.022_408_042,
    0.025_165_638,
    0.027_868_463,
    0.030_481_474,
    0.032_970_056,
    0.035_300_635,
    0.037_441_27,
    0.039_362_226,
    0.041_036_53,
    0.042_440_448,
    0.043_553_92,
    0.044_360_947,
    0.044_849_884,
    0.045_013_655,
    0.044_849_884,
    0.044_360_947,
    0.043_553_92,
    0.042_440_448,
    0.041_036_53,
    0.039_362_226,
    0.037_441_27,
    0.035_300_635,
    0.032_970_056,
    0.030_481_474,
    0.027_868_463,
    0.025_165_638,
    0.022_408_042,
    0.019_630_551,
    0.016_867_284,
    0.014_151_033,
    0.011_512_753,
    0.008_981_075,
    0.006_581_898,
    0.004_338_022_3,
    0.002_268_875_3,
    0.000_390_291_83,
    -0.001_285_617_6,
    -0.002_750_527_6,
    -0.003_999_881_4,
    -0.005_032_803_4,
    -0.005_851_940_3,
    -0.006_463_243_6,
    -0.006_875_694,
    -0.007_100_979,
    -0.007_153_130_6,
    -0.007_048_135_6,
    -0.006_803_523_7,
    -0.006_437_945,
    -0.005_970_751,
    -0.005_421_576_6,
    -0.004_809_947,
    -0.004_154_906,
    -0.003_474_673,
    -0.002_786_343_5,
    -0.002_105_628_4,
    -0.001_446_640_1,
    -0.000_821_725_47,
    -0.000_241_349_67,
    0.000_285_972_4,
    0.000_753_697_3,
    0.001_157_229,
    0.001_493_853,
    0.001_762_634_4,
    0.001_964_281_5,
    0.002_100_984_6,
    0.002_176_231_2,
    0.002_194_607_2,
    0.002_161_588,
    0.002_083_326_2,
    0.001_966_439_4,
    0.001_817_807_4,
    0.001_644_377_6,
    0.001_452_988_1,
    0.001_250_208,
    0.001_042_199,
    0.000_834_600_1,
    0.000_632_436_3,
    0.000_440_051_26,
    0.000_261_064_4,
    0.000_098_351_15,
    -0.000_045_955_29,
    -0.000_170_442_57,
    -0.000_274_380_2,
    -0.000_357_666_1,
    -0.000_420_761,
    -0.000_464_614_1,
    -0.000_490_582_4,
    -0.000_500_347_1,
    -0.000_495_829_56,
    -0.000_479_108_9,
    -0.000_452_343_9,
    -0.000_417_701_4,
    -0.000_377_291_76,
    -0.000_333_113_2,
    -0.000_287_006_03,
    -0.000_240_616_41,
    -0.000_195_370_45,
    -0.000_152_458_06,
    -0.000_112_826_034,
    -0.000_077_179_7,
    -0.000_045_992_11,
    -0.000_019_519_683,
    0.000_002_176_991_5,
    0.000_019_208_326,
    0.000_031_828_265,
    0.000_040_405_343,
    0.000_045_393_42,
    0.000_047_303_198,
    0.000_046_675_348,
    0.000_044_056_076,
    0.000_039_975_657,
    0.000_034_930_443,
    0.000_029_368_508,
    0.000_023_679_097,
];