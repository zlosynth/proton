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
// Factor: 0.09
// Length: 121
// Rejection: 70
pub const COEFFICIENTS_8: [f32; 121] = [
    -0.000_037_513_328,
    -0.000_046_917_38,
    -0.000_049_415_186,
    -0.000_040_607_705,
    -0.000_016_461_57,
    0.000_025_854_459,
    0.000_087_087_49,
    0.000_165_100_27,
    0.000_254_296_78,
    0.000_345_416,
    0.000_425_814_13,
    0.000_480_311_84,
    0.000_492_618_6,
    0.000_447_270_78,
    0.000_331_935_04,
    0.000_139_850_38,
    -0.000_127_881_45,
    -0.000_460_485_78,
    -0.000_836_514_64,
    -0.001_223_947_2,
    -0.001_581_598_7,
    -0.001_861_924_9,
    -0.002_015_154_8,
    -0.001_994_515_8,
    -0.001_762_146_8,
    -0.001_295_147_9,
    -0.000_591_107_4,
    0.000_327_597_26,
    0.001_411_406_8,
    0.002_583_669_7,
    0.003_743_609_2,
    0.004_772_313_4,
    0.005_541_596,
    0.005_925_264,
    0.005_812_008,
    0.005_118_879_5,
    0.003_804_128,
    0.001_878_075_1,
    -0.000_589_242_6,
    -0.003_464_777_7,
    -0.006_554_338_6,
    -0.009_610_038,
    -0.012_343_264,
    -0.014_442_606,
    -0.015_595_665,
    -0.015_513_239,
    -0.013_954_036,
    -0.010_747_89,
    -0.005_815_428,
    0.000_817_686_4,
    0.009_013_462,
    0.018_522_272,
    0.028_991_928,
    0.039_984_886,
    0.051_002_57,
    0.061_515_287,
    0.070_995_57,
    0.078_952_51,
    0.084_964_484,
    0.088_707_77,
    0.089_978_69,
    0.088_707_77,
    0.084_964_484,
    0.078_952_51,
    0.070_995_57,
    0.061_515_287,
    0.051_002_57,
    0.039_984_886,
    0.028_991_928,
    0.018_522_272,
    0.009_013_462,
    0.000_817_686_4,
    -0.005_815_428,
    -0.010_747_89,
    -0.013_954_036,
    -0.015_513_239,
    -0.015_595_665,
    -0.014_442_606,
    -0.012_343_264,
    -0.009_610_038,
    -0.006_554_338_6,
    -0.003_464_777_7,
    -0.000_589_242_6,
    0.001_878_075_1,
    0.003_804_128,
    0.005_118_879_5,
    0.005_812_008,
    0.005_925_264,
    0.005_541_596,
    0.004_772_313_4,
    0.003_743_609_2,
    0.002_583_669_7,
    0.001_411_406_8,
    0.000_327_597_26,
    -0.000_591_107_4,
    -0.001_295_147_9,
    -0.001_762_146_8,
    -0.001_994_515_8,
    -0.002_015_154_8,
    -0.001_861_924_9,
    -0.001_581_598_7,
    -0.001_223_947_2,
    -0.000_836_514_64,
    -0.000_460_485_78,
    -0.000_127_881_45,
    0.000_139_850_38,
    0.000_331_935_04,
    0.000_447_270_78,
    0.000_492_618_6,
    0.000_480_311_84,
    0.000_425_814_13,
    0.000_345_416,
    0.000_254_296_78,
    0.000_165_100_27,
    0.000_087_087_49,
    0.000_025_854_459,
    -0.000_016_461_57,
    -0.000_040_607_705,
    -0.000_049_415_186,
    -0.000_046_917_38,
    -0.000_037_513_328,
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
